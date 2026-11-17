# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5900 frames over 65.6 s · **median 10.83 ms (92 fps)** · mean 11.13 · p95 14.05 · p99 17.22 (1% low 58 fps) · worst 47.72 ms · 80 spikes over 1.5× median · 59.4 stutters/min (work 52.1/min)
- warm-up: 323 frames over 8.9 s, worst 2470.4 ms
- hitches (> 27.1 ms): **33** measured, 41 in warm-up
- timed GPU passes: 6.68 ms of a 11.13 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.83 ms, limit 11 ms
- **FAIL** frame_ms: 65 stutters (> 1.5x moving median of 60) in 1.09 min = 59.41/min, limit 1/min
- **FAIL** worst frame 47.72 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.869 | 2.036 | 2.068 | 0.92 | 0.174 |
| main_opaque_pass_3d | 1.498 | 1.632 | 2.047 | 0.92 | 0.067 |
| early prepass | 0.458 | 0.499 | 0.737 | 0.92 | 0.086 |
| bloom | 0.289 | 0.315 | 0.322 | 0.92 | 0.062 |
| atmosphere_luts | 0.279 | 0.303 | 0.365 | 0.92 | 0.003 |
| water_velocity | 0.256 | 0.279 | 0.347 | 0.92 | 0.002 |
| auxiliary_views | 0.194 | 0.212 | 0.677 | 0.92 | 0.205 |
| water_sspr | 0.185 | 0.202 | 0.203 | 0.92 | 0.002 |
| ssao | 0.173 | 0.189 | 0.193 | 0.92 | 0.007 |
| atmos_froxel_inject | 0.134 | 0.146 | 0.148 | 0.92 | 0.001 |
| clouds_trace | 0.128 | 0.140 | 0.232 | 0.92 | 0.001 |
| water_fft | 0.120 | 0.131 | 0.138 | 0.92 | 0.003 |
| uniform_allocation | 0.117 | 0.128 | 0.177 | 0.92 | 0.025 |
| upscaling | 0.108 | 0.118 | 0.125 | 0.92 | 0.004 |
| clouds_composite | 0.097 | 0.106 | 0.109 | 0.92 | 0.002 |
| bin_unpacking | 0.096 | 0.104 | 0.141 | 0.92 | 0.018 |
| main_transmissive_pass_3d | 0.079 | 0.086 | 0.178 | 0.92 | 0.009 |
| tonemapping | 0.078 | 0.085 | 0.088 | 0.92 | 0.004 |
| clustering | 0.059 | 0.064 | 0.067 | 0.92 | 0.018 |
| render_sky | 0.047 | 0.051 | 0.053 | 0.92 | 0.002 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.92 | 0.001 |
| surface_exposure | 0.041 | 0.044 | 0.045 | 0.92 | 0.001 |
| lightprobe_irradiance_map | 0.039 | 0.412 | 0.416 | 0.09 | 0.001 |
| surface_trail | 0.037 | 0.056 | 0.057 | 0.66 | 0.002 |
| surface_exposure_far | 0.035 | 0.043 | 0.045 | 0.81 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.033 | 0.92 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.047 | 0.92 | 0.005 |
| water_snapshot | 0.028 | 0.030 | 0.035 | 0.92 | 0.002 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.031 | 0.92 | 0.001 |
| lightprobe_radiance_map | 0.020 | 0.219 | 0.229 | 0.09 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.92 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.92 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.92 | 0.123 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.92 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.020 | 0.92 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.92 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.91 | 0.003 |
| lightprobe_copy | 0.001 | 0.014 | 0.015 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.694 | 12.338 | 40.17 | 57194.0 |
| render/busy/ms | 9.142 | 11.785 | 39.55 | 53939.2 |
| main/busy/ms | 9.102 | 10.929 | 29.51 | 53702.7 |
| stage/render/render | 5.718 | 7.500 | 14.09 | 33734.2 |
| stage/main/post_update | 5.694 | 7.116 | 10.75 | 33595.4 |
| render/graph/ms | 4.975 | 6.721 | 13.05 | 29355.2 |
| encode//sum | 4.264 | 6.018 | 18.19 | 25158.2 |
| stage/render/graph_submit | 3.741 | 5.098 | 9.44 | 22069.3 |
| encode/submit | 2.400 | 3.404 | 6.61 | 14162.9 |
| stage/main/update | 2.147 | 2.569 | 23.05 | 12665.4 |
| encode/finish_wall | 1.414 | 2.060 | 3.94 | 8341.8 |
| encode/pass:shadow_point_light/sum | 1.213 | 3.741 | 16.92 | 7157.1 |
| stage/render/camera_driver | 1.182 | 1.673 | 4.19 | 6971.9 |
| encode/pass:shadow_directional_light/sum | 0.839 | 1.561 | 2.64 | 4949.6 |
| stage/render/prepare_write_phase_buffers | 0.787 | 1.131 | 2.72 | 4642.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.757 | 1.102 | 1.83 | 4467.6 |
| encode/pass:early prepass/sum | 0.724 | 1.078 | 1.71 | 4268.7 |
| stage/render/prepare_assets | 0.573 | 0.732 | 9.93 | 3380.6 |
| bigspace/total/ms | 0.527 | 1.308 | 2.48 | 3106.8 |
| stage/render/prepare_resources | 0.459 | 0.551 | 1.07 | 2708.9 |
| render/present/ms | 0.458 | 1.018 | 2.29 | 2701.9 |
| stage/render/prepare_meshes | 0.454 | 0.587 | 6.10 | 2676.9 |
| stage/main/first | 0.400 | 0.524 | 5.74 | 2358.9 |
| stage/main/pre_update | 0.353 | 0.510 | 2.03 | 2082.7 |
| stage/render/specialize | 0.329 | 0.422 | 11.41 | 1939.3 |
| terrain/update/ms | 0.311 | 0.403 | 1.81 | 1833.1 |
| stage/render/queue | 0.294 | 0.427 | 7.49 | 1734.5 |
| stage/render/prepare_bind_groups | 0.287 | 0.527 | 2.36 | 1694.0 |
| terrain/select/ms | 0.278 | 0.363 | 0.89 | 1638.3 |
| controller/feed/ms | 0.260 | 0.328 | 0.85 | 1533.8 |
| stream/drive/ms | 0.250 | 0.325 | 20.72 | 1474.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.240 | 0.356 | 0.99 | 1417.7 |
| bigspace/low_precision/ms | 0.221 | 0.716 | 2.26 | 1301.0 |
| stage/main/fixed_main_loop | 0.220 | 0.368 | 1.02 | 1295.7 |
| far/select/ms | 0.197 | 0.273 | 0.74 | 1160.6 |
| stage/render/create_views | 0.196 | 0.252 | 1.22 | 1156.0 |
| stream/tick/ms | 0.193 | 0.265 | 0.75 | 1136.0 |
| stage/render/prepare_batch_phases | 0.189 | 0.271 | 0.59 | 1115.4 |
| stage/render/extract_commands | 0.173 | 0.204 | 0.44 | 1020.1 |
| stage/render/prepare_flush | 0.161 | 0.306 | 0.73 | 950.3 |
| anim/evaluate/ms | 0.157 | 0.243 | 1.92 | 924.9 |
| anim/evaluate_cpu/ms | 0.153 | 0.192 | 0.71 | 900.8 |
| bigspace/high_precision/ms | 0.148 | 0.648 | 1.64 | 874.5 |
| stage/render/prepare_views | 0.137 | 0.171 | 0.68 | 808.2 |
| controller/move/ms | 0.132 | 0.240 | 0.61 | 779.4 |
| stage/main/last | 0.115 | 0.161 | 1.56 | 676.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.102 | 0.172 | 0.81 | 599.1 |
| render/acquire/ms | 0.094 | 0.120 | 0.56 | 552.9 |
| stage/render/prepare_collect_phase_buffers | 0.063 | 0.134 | 0.51 | 373.2 |
| pipelines/compile_ms | 0.052 | 0.000 | 129.84 | 305.7 |
| pipelines/front_ms | 0.050 | 0.000 | 125.78 | 293.6 |
| physics/step/ms | 0.048 | 0.087 | 0.56 | 283.1 |
| physics/insert/ms | 0.034 | 0.365 | 0.95 | 199.7 |
| stage/render/phase_sort | 0.032 | 0.056 | 0.24 | 189.6 |
| stage/render/cleanup | 0.032 | 0.094 | 0.65 | 188.7 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.57 | 183.8 |
| weather/map/ms | 0.024 | 0.032 | 0.15 | 143.7 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.45 | 86.6 |
| anim/apply/ms | 0.012 | 0.022 | 0.37 | 68.7 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.55 | 64.5 |
| controller/avoid/ms | 0.011 | 0.014 | 0.12 | 62.6 |
| weather/eval/ms | 0.010 | 0.014 | 0.19 | 61.4 |
| script/update/ms | 0.007 | 0.009 | 0.19 | 42.5 |
| script/input/ms | 0.005 | 0.007 | 0.17 | 32.3 |
| stream/despawn/ms | 0.005 | 0.000 | 1.30 | 31.1 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 28.6 |
| script/ui/ms | 0.005 | 0.006 | 0.21 | 26.6 |
| controller/write/ms | 0.003 | 0.004 | 0.08 | 20.3 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 14.2 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.10 | 12.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.42 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.8 |
| stage/render/prepare | 0.001 | 0.002 | 0.16 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.06 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 29 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1277 | 2262 | 595 |
| crowd/c0/meshes/count | count | 352 | 646 | 158 |
| crowd/c1/count | count | 146 | 156 | 148 |
| crowd/c1/joints/count | count | 5531 | 5995 | 5601 |
| crowd/c1/meshes/count | count | 1177 | 1297 | 1183 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4612 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4589 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.42 | 1.34 | 0.42 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.45 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.55 | 0.01 |
| encode/finish_critical | ms | 0.98 | 2.64 | 0.98 |
| encode/pass:early prepass/max | ms | 0.74 | 1.71 | 0.98 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 1.83 | 0.68 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.81 | 0.06 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.99 | 0.21 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 2.64 | 0.78 |
| encode/pass:shadow_point_light/max | ms | 0.81 | 1.62 | 0.84 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.57 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.38 | 39.55 | 8.17 |
| gpu/timed/ms | ms | 7.98 | 8.95 | 8.17 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 804 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 804 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1283 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3393 | 5144 | 3189 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1283 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3393 | 5144 | 3189 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 25 | 201 | 4 |
| phases/shadow/bins | count | 5040 | 9063 | 2413 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.5 k | 6360 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 205 | 254 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2349 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 188 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 323 | 324 | 324 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 14 | 555 | 4 |
| preprocess/bind_groups_created | count | 14 | 555 | 4 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 223 | 125 |
| process/cpu_usage | % | 25.96 | 37.32 | 33.33 |
| process/mem_usage | GiB | 5.47 | 5.64 | 5.64 |
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
| skin/blocks_changed/count | count | 14 | 130 | 16 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 366.1 k | 811.3 k | 489.0 k |
| skin/joints_written/count | count | 5720 | 12.7 k | 7640 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4917 | 5010 | 5010 |
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
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.98 | 42.57 | 31.90 |
| system/mem_usage | % | 56.77 | 57.39 | 57.35 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 72 | 248 | 54 |
| views/all/bins | count | 7606 | 11.9 k | 5911 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.4 k | 14.0 k |
| views/all/pipelines | count | 250 | 301 | 75 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 34.9 k | 8121 |
| views/cascade/batch_sets | count | 9 | 50 | 4 |
| views/cascade/bins | count | 2802 | 6138 | 2413 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8901 | 23.6 k | 6360 |
| views/cascade/pipelines | count | 36 | 53 | 29 |
| views/cascade/work_items | count | 417 | 22.8 k | 201 |
| views/lamp_face/batch_sets | count | 5 | 190 | 0 |
| views/lamp_face/bins | count | 2172 | 2952 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 1 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2410 | 3248 | 0 |
| views/lamp_face/pipelines | count | 168 | 208 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 96 | 2880 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2862 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8381 | 13.1 k | 7612 |
| views/main/pipelines | count | 45 | 49 | 46 |
| views/main/work_items | count | 8552 | 13.3 k | 7920 |
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
| anim/bones_written/count | count | 59.06 | 251 | 348.5 k |
| anim/events/count | count | 0.10 | 3 | 562 |
| assets/Image/added | count | 0.10 | 31 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.69 | 152 | 4061 |
| assets/Mesh/removed | count | 0.82 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.67 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 60 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.79 | 1309 | 4675 |
| encode//count | count | 35.91 | 220 | 211.9 k |
| encode/bind_walk/init_textures | count | 14.0 k | 114.7 k | 82.75 M |
| encode/bind_walk/init_textures_skipped | count | 65.6 k | 151.8 k | 387.02 M |
| encode/bind_walk/inits | count | 469 | 1117 | 2.77 M |
| encode/bind_walk/inits_skipped | count | 108 | 173 | 639.8 k |
| encode/bind_walk/merge_textures | count | 14.0 k | 114.6 k | 82.32 M |
| encode/bind_walk/merge_textures_skipped | count | 5366 | 6012 | 31.66 M |
| encode/bind_walk/merges | count | 102 | 334 | 602.9 k |
| encode/bind_walk/merges_skipped | count | 84.11 | 129 | 496.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5900 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5900 |
| encode/pass:early prepass/count | count | 1 | 1 | 5900 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5900 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5900 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5900 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5900 |
| encode/pass:shadow_point_light/count | count | 1.99 | 48 | 11.7 k |
| encode/queue/command_buffers | count | 47.92 | 278 | 282.7 k |
| encode/queue/submits | count | 2.02 | 4 | 11.9 k |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 4.99 M | 11.14 G |
| encode/queue/write_buffer/calls | count | 268 | 830 | 1.58 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.7 k | 3.52 M | 1.05 G |
| encode/queue/write_buffer_with/calls | count | 11.05 | 465 | 65.2 k |
| encode/queue/write_texture/bytes | bytes | 12.0 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5900 |
| physics/inserts | count | 0.38 | 110 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.67 | 3 | 3938 |
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
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5738 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1258 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.1 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2470.4 | main/busy/ms 2443, stage/main/post_update 2338, stage/main/update 96.01, surface/advance_cpu_ms 66.91, stage/main/pre_update 4.65, stage/main/last 1.15, surface/cell_upload_ms 0.78, stream/drive/ms 0.58, stage/main/first 0.55, stage/main/fixed_main_loop 0.43, stream/tick/ms 0.16, weather/map/ms 0.10 |
| warm-up | 3.17 | 701.0 | main/busy/ms 2570, stage/main/post_update 2359, render/thread/ms 670, render/busy/ms 526, stage/render/prepare_assets 479, stage/main/update 197, stage/render/prepare_views 141, render/acquire/ms 141, surface/advance_cpu_ms 66.91, stage/render/create_views 27.04, stage/render/render 11.57, render/graph/ms 7.58 |
| warm-up | 3.29 | 114.6 | render/thread/ms 769, render/busy/ms 567, stage/render/prepare_assets 503, main/busy/ms 147, stage/render/prepare_views 143, render/acquire/ms 142, stage/main/update 105, pipelines/compile_ms 101, stage/render/render 73.86, render/present/ms 59.47, pipelines/front_ms 57.51, pipelines/driver_ms 43.02 |
| warm-up | 3.44 | 133.2 | pipelines/wait_ms 3584, pipelines/compile_ms 1331, pipelines/front_ms 1219, main/busy/ms 164, render/thread/ms 116, pipelines/driver_ms 112, render/busy/ms 110, stage/main/post_update 75.13, stage/main/update 56.61, stage/render/prepare_resources 46.92, stage/main/pre_update 26.02, terrain/update/ms 20.73 |
| warm-up | 3.53 | 95.8 | pipelines/wait_ms 3957, pipelines/compile_ms 1864, pipelines/front_ms 1770, render/thread/ms 173, render/busy/ms 171, main/busy/ms 165, pipelines/driver_ms 94.37, stage/render/prepare_assets 83.10, stage/main/update 82.37, stage/main/post_update 49.23, stage/render/prepare_resources 46.72, stage/main/pre_update 26.85 |
| warm-up | 3.57 | 37.7 | pipelines/compile_ms 1378, pipelines/front_ms 1312, pipelines/wait_ms 382, render/thread/ms 107, render/busy/ms 105, stage/render/prepare_assets 86.07, pipelines/driver_ms 65.38, main/busy/ms 57.73, stage/main/update 33.53, stage/main/post_update 16.61, terrain/update/ms 14.71, stage/render/render 11.89 |
| warm-up | 3.73 | 58.4 | pipelines/compile_ms 826, pipelines/front_ms 801, render/thread/ms 50.15, render/busy/ms 48.45, main/busy/ms 47.74, stage/main/update 36.52, stage/render/prepare_assets 34.26, stream/drive/ms 32.34, pipelines/driver_ms 24.90, stage/render/render 9.16, stage/main/post_update 7.32, render/graph/ms 7.02 |
| warm-up | 3.82 | 67.5 | pipelines/compile_ms 1020, pipelines/front_ms 976, render/thread/ms 67.08, render/busy/ms 65.34, main/busy/ms 52.40, pipelines/driver_ms 43.94, stage/main/update 39.82, stage/render/prepare_assets 38.18, stream/drive/ms 34.42, stage/render/render 20.95, render/graph/ms 18.64, stage/render/graph_submit 15.97 |
| warm-up | 3.94 | 90.5 | pipelines/compile_ms 363, pipelines/front_ms 351, render/thread/ms 74.14, render/busy/ms 72.37, main/busy/ms 56.44, stage/render/prepare_assets 47.06, stage/main/update 30.20, stage/main/post_update 17.68, stream/drive/ms 15.93, stage/render/render 13.80, pipelines/driver_ms 11.71, render/graph/ms 11.56 |
| warm-up | 4.00 | 61.9 | pipelines/compile_ms 709, pipelines/front_ms 693, render/thread/ms 90.79, render/busy/ms 89.43, stage/render/prepare_assets 69.68, main/busy/ms 59.35, stage/main/update 35.96, pipelines/driver_ms 16.34, stage/main/post_update 14.53, stage/render/render 10.81, render/graph/ms 8.91, stage/render/graph_submit 6.81 |
| warm-up | 4.05 | 47.6 | pipelines/compile_ms 1366, pipelines/front_ms 1331, render/thread/ms 71.55, render/busy/ms 70.12, main/busy/ms 54.58, stage/render/prepare_assets 51.89, stage/main/update 34.63, pipelines/driver_ms 34.47, stage/main/post_update 10.45, stage/render/render 9.26, render/graph/ms 7.28, stage/render/graph_submit 5.15 |
| warm-up | 4.08 | 34.8 | pipelines/compile_ms 995, pipelines/front_ms 964, render/thread/ms 62.18, render/busy/ms 60.24, main/busy/ms 53.48, stage/render/prepare_assets 31.00, pipelines/driver_ms 30.48, stage/main/update 25.11, stage/main/post_update 20.19, stage/render/render 14.76, render/graph/ms 12.31, stage/render/graph_submit 6.94 |
| warm-up | 4.12 | 40.0 | pipelines/compile_ms 125, pipelines/front_ms 118, render/thread/ms 59.36, render/busy/ms 56.99, main/busy/ms 55.32, stage/main/post_update 23.76, stage/main/update 22.85, stage/render/prepare_assets 19.64, stage/render/render 15.74, render/graph/ms 13.32, stage/render/graph_submit 7.86, pipelines/driver_ms 7.20 |
| warm-up | 4.19 | 67.1 | pipelines/compile_ms 774, pipelines/front_ms 749, pipelines/wait_ms 128, render/thread/ms 73.81, render/busy/ms 71.90, main/busy/ms 38.15, stage/render/prepare_assets 36.80, pipelines/driver_ms 25.33, stage/main/post_update 14.93, stage/main/update 14.45, stage/render/render 12.61, render/graph/ms 10.63 |
| warm-up | 4.25 | 63.6 | pipelines/compile_ms 1709, pipelines/front_ms 1640, pipelines/wait_ms 260, render/thread/ms 103, render/busy/ms 101, encode//sum 94.28, pipelines/driver_ms 68.88, stage/render/prepare_assets 45.82, main/busy/ms 42.53, stage/render/render 31.72, encode/pass:shadow_point_light/sum 30.59, render/graph/ms 29.48 |
| warm-up | 4.36 | 102.2 | pipelines/compile_ms 2451, pipelines/front_ms 2361, pipelines/wait_ms 599, render/thread/ms 154, render/busy/ms 152, encode//sum 137, pipelines/driver_ms 89.97, stage/render/render 51.89, main/busy/ms 51.67, render/graph/ms 49.18, encode/pass:shadow_point_light/sum 34.51, stage/render/graph_submit 31.31 |
| warm-up | 4.39 | 37.8 | pipelines/compile_ms 1704, pipelines/front_ms 1652, pipelines/wait_ms 525, render/thread/ms 133, render/busy/ms 130, main/busy/ms 124, encode//sum 92.56, stage/main/update 90.79, stream/drive/ms 53.22, pipelines/driver_ms 51.74, stage/render/render 49.71, render/graph/ms 46.87 |
| warm-up | 4.55 | 152.4 | pipelines/compile_ms 2316, pipelines/front_ms 2248, pipelines/wait_ms 544, main/busy/ms 119, render/thread/ms 104, render/busy/ms 101, stage/main/update 95.00, pipelines/driver_ms 68.16, stream/drive/ms 54.33, encode//sum 49.06, stage/render/prepare_assets 38.80, stage/render/render 36.33 |
| warm-up | 4.58 | 36.1 | pipelines/compile_ms 2535, pipelines/front_ms 2462, pipelines/wait_ms 486, main/busy/ms 99.10, render/thread/ms 99.03, render/busy/ms 96.85, pipelines/driver_ms 72.83, stage/main/update 71.28, stream/drive/ms 55.06, stage/render/prepare_assets 46.59, stage/render/render 20.09, render/graph/ms 17.28 |
| warm-up | 4.71 | 125.3 | pipelines/compile_ms 759, pipelines/front_ms 737, main/busy/ms 109, render/thread/ms 78.14, render/busy/ms 76.83, stage/main/update 75.39, stream/drive/ms 54.73, stage/render/prepare_assets 47.24, pipelines/driver_ms 21.49, stage/main/post_update 17.43, stage/render/render 11.15, render/graph/ms 9.08 |
| warm-up | 4.74 | 36.8 | pipelines/compile_ms 1876, pipelines/front_ms 1815, main/busy/ms 91.27, render/thread/ms 76.67, render/busy/ms 75.40, stage/main/update 62.96, pipelines/driver_ms 61.29, stage/render/prepare_assets 56.18, stream/drive/ms 41.74, pipelines/wait_ms 24.33, stage/main/post_update 18.93, stage/render/render 9.34 |
| warm-up | 4.83 | 81.3 | pipelines/compile_ms 2009, pipelines/front_ms 1947, main/busy/ms 77.78, render/thread/ms 64.70, render/busy/ms 63.22, pipelines/driver_ms 62.52, stage/main/update 58.81, stage/render/prepare_assets 44.36, stream/drive/ms 41.02, pipelines/wait_ms 23.51, stage/main/post_update 11.50, stage/render/render 10.47 |
| warm-up | 4.89 | 44.3 | pipelines/compile_ms 235, pipelines/front_ms 228, main/busy/ms 48.29, stage/main/update 35.00, render/thread/ms 34.37, render/busy/ms 32.99, stream/drive/ms 24.97, stage/render/render 10.33, stage/render/prepare_assets 9.71, render/graph/ms 8.45, encode//sum 8.22, stage/main/post_update 6.83 |
| warm-up | 4.97 | 32.2 | pipelines/compile_ms 147, pipelines/front_ms 141, render/thread/ms 46.60, render/busy/ms 45.22, main/busy/ms 38.09, stage/main/update 17.21, stage/main/post_update 15.31, stage/render/prepare_assets 14.77, stage/render/render 13.07, render/graph/ms 11.00, encode//sum 7.77, stage/render/graph_submit 7.61 |
| warm-up | 5.01 | 37.7 | pipelines/compile_ms 303, pipelines/front_ms 292, render/thread/ms 59.77, render/busy/ms 57.59, main/busy/ms 42.97, stage/main/post_update 19.14, stage/render/render 18.36, stage/main/update 18.00, stage/render/prepare_assets 17.02, render/graph/ms 15.26, stage/render/graph_submit 11.19, pipelines/driver_ms 10.75 |
| warm-up | 5.08 | 46.0 | pipelines/compile_ms 529, pipelines/front_ms 510, render/thread/ms 57.48, render/busy/ms 55.79, main/busy/ms 38.38, stage/render/prepare_assets 30.40, pipelines/driver_ms 19.04, stage/main/update 18.52, stage/main/post_update 11.97, stage/render/render 11.90, render/graph/ms 9.68, encode//sum 7.79 |
| warm-up | 5.11 | 29.5 | pipelines/compile_ms 970, pipelines/front_ms 942, render/thread/ms 61.69, render/busy/ms 59.89, main/busy/ms 44.00, stage/render/prepare_assets 30.09, pipelines/driver_ms 27.51, stage/main/update 17.17, stage/render/render 13.96, stage/main/post_update 13.03, render/graph/ms 11.65, stage/main/pre_update 8.61 |
| warm-up | 5.14 | 29.6 | pipelines/compile_ms 841, pipelines/front_ms 818, render/thread/ms 53.22, render/busy/ms 51.35, main/busy/ms 38.54, pipelines/driver_ms 22.80, stage/render/render 16.37, render/graph/ms 13.94, stage/main/post_update 13.10, stage/main/update 12.37, stage/render/graph_submit 10.36, stage/render/prepare_resources 10.22 |
| warm-up | 5.17 | 30.0 | pipelines/compile_ms 415, pipelines/front_ms 395, render/thread/ms 56.76, render/busy/ms 55.07, main/busy/ms 32.51, pipelines/driver_ms 19.43, stage/render/render 16.76, stage/main/update 14.67, render/graph/ms 14.50, stage/render/prepare_resources 13.49, stage/main/post_update 12.60, stage/render/graph_submit 10.77 |
| warm-up | 5.21 | 40.2 | pipelines/compile_ms 222, pipelines/front_ms 208, render/thread/ms 58.14, render/busy/ms 56.91, main/busy/ms 36.56, stage/render/prepare_assets 24.17, stage/main/update 17.35, pipelines/driver_ms 14.05, stage/main/post_update 13.26, stage/render/render 13.20, render/graph/ms 11.18, encode//sum 9.06 |
| warm-up | 5.32 | 39.1 | pipelines/compile_ms 1541, pipelines/front_ms 1509, render/thread/ms 47.13, render/busy/ms 45.68, main/busy/ms 40.47, pipelines/driver_ms 31.74, stage/render/prepare_assets 24.21, stage/main/update 23.11, stream/drive/ms 12.96, stage/render/render 12.40, stage/main/post_update 12.04, render/graph/ms 10.05 |
| warm-up | 5.45 | 33.2 | pipelines/compile_ms 343, pipelines/front_ms 323, render/thread/ms 52.65, render/busy/ms 51.35, stage/render/prepare_assets 33.18, main/busy/ms 23.49, pipelines/driver_ms 19.95, stage/render/render 13.07, render/graph/ms 11.09, stage/main/post_update 9.94, stage/main/update 8.46, stage/render/graph_submit 8.46 |
| warm-up | 5.95 | 32.2 | render/thread/ms 40.14, render/busy/ms 39.07, main/busy/ms 28.44, stage/render/prepare_assets 17.94, stage/main/update 11.22, stage/main/post_update 11.10, stage/render/render 8.78, render/graph/ms 7.25, encode//sum 6.85, stage/render/graph_submit 5.17, stream/drive/ms 3.91, stage/render/specialize 3.40 |
| warm-up | 6.17 | 30.1 | pipelines/compile_ms 143, pipelines/front_ms 137, render/thread/ms 46.20, render/busy/ms 44.60, main/busy/ms 40.87, stage/main/update 17.00, stage/main/post_update 16.33, stage/render/render 14.10, render/graph/ms 12.04, stage/render/prepare_assets 10.10, encode//sum 9.57, stage/render/graph_submit 8.15 |
| warm-up | 6.28 | 37.8 | main/busy/ms 44.68, render/thread/ms 30.48, render/busy/ms 29.20, stage/main/update 24.40, stage/main/post_update 15.22, stream/drive/ms 14.25, stage/render/render 10.93, render/graph/ms 9.12, encode//sum 7.98, stage/render/prepare_assets 7.52, stage/render/graph_submit 6.35, encode/submit 3.90 |
| warm-up | 6.31 | 29.7 | render/thread/ms 39.68, main/busy/ms 39.32, render/busy/ms 38.41, stage/main/update 18.97, stage/main/post_update 14.67, stage/render/prepare_assets 14.27, stage/render/render 11.45, render/graph/ms 9.45, encode//sum 8.87, stage/render/graph_submit 5.79, stream/drive/ms 4.48, stage/render/camera_driver 3.49 |
| warm-up | 6.68 | 32.4 | main/busy/ms 40.82, render/thread/ms 24.15, render/busy/ms 23.10, stage/main/update 21.93, stream/drive/ms 15.15, stage/main/post_update 13.89, stream/spawn/ms 13.66, stage/render/render 11.23, render/graph/ms 9.75, encode//sum 7.83, stage/render/graph_submit 7.36, stage/render/prepare_assets 5.24 |
| warm-up | 6.78 | 31.4 | pipelines/compile_ms 165, pipelines/front_ms 161, render/thread/ms 42.60, render/busy/ms 41.44, main/busy/ms 19.39, stage/render/render 11.67, stage/main/post_update 10.45, stage/render/specialize 10.17, render/graph/ms 10.05, stage/render/graph_submit 7.66, stage/render/queue 7.16, encode//sum 7.14 |
| warm-up | 8.04 | 30.5 | pipelines/compile_ms 132, pipelines/front_ms 129, render/thread/ms 36.39, render/busy/ms 35.39, main/busy/ms 18.68, stage/render/render 10.72, stage/main/post_update 10.26, stage/render/specialize 9.58, render/graph/ms 9.24, stage/render/graph_submit 7.10, encode//sum 6.87, stage/render/queue 6.17 |
| measure | 0.41 | 31.2 | render/thread/ms 35.89, render/busy/ms 34.64, main/busy/ms 16.35, stage/render/render 11.70, render/graph/ms 10.11, stage/main/post_update 9.95, stage/render/specialize 9.02, stage/render/graph_submit 7.88, encode//sum 6.31, stage/render/queue 6.06, encode/submit 5.05, stage/main/update 3.60 |
| measure | 1.70 | 30.8 | render/thread/ms 36.79, render/busy/ms 35.64, main/busy/ms 17.65, stage/render/render 12.09, render/graph/ms 10.46, stage/main/post_update 10.42, encode//sum 9.69, stage/render/specialize 9.16, stage/render/graph_submit 7.66, stage/render/queue 5.33, encode/pass:shadow_point_light/sum 4.44, stage/render/prepare_meshes 4.29 |
| measure | 3.05 | 31.0 | render/thread/ms 36.37, render/busy/ms 35.20, main/busy/ms 18.84, stage/main/post_update 12.29, stage/render/render 11.40, stage/render/specialize 9.80, render/graph/ms 9.77, encode//sum 7.79, stage/render/graph_submit 7.49, stage/render/queue 6.20, encode/submit 4.25, stage/main/update 3.91 |
| measure | 7.05 | 29.4 | render/thread/ms 35.82, render/busy/ms 34.64, main/busy/ms 19.20, stage/render/render 11.45, stage/main/post_update 10.99, render/graph/ms 9.83, stage/render/specialize 9.36, encode//sum 8.50, stage/render/graph_submit 7.37, stage/render/queue 5.97, stage/main/update 4.62, encode/submit 4.47 |
| measure | 8.39 | 29.7 | render/thread/ms 36.05, render/busy/ms 34.97, main/busy/ms 16.63, stage/render/render 11.63, stage/main/post_update 10.55, render/graph/ms 10.10, stage/render/specialize 9.32, stage/render/graph_submit 7.61, encode//sum 7.52, stage/render/queue 6.06, encode/submit 4.48, stage/main/update 3.66 |
| measure | 9.72 | 34.4 | render/thread/ms 47.81, render/busy/ms 46.65, main/busy/ms 23.59, stage/render/render 17.18, render/graph/ms 15.49, stage/main/post_update 15.36, encode//sum 13.23, stage/render/graph_submit 11.20, stage/render/specialize 10.20, encode/pass:shadow_point_light/sum 9.07, encode/submit 6.57, stage/render/queue 6.55 |
| measure | 15.71 | 33.0 | render/thread/ms 42.36, render/busy/ms 41.26, main/busy/ms 18.83, stage/render/render 14.40, render/graph/ms 12.91, stage/main/post_update 12.18, encode//sum 11.41, stage/render/specialize 10.82, stage/render/graph_submit 9.92, encode/pass:shadow_point_light/sum 8.03, stage/render/queue 6.96, encode/submit 6.12 |
| measure | 17.12 | 33.5 | render/thread/ms 42.48, render/busy/ms 41.43, main/busy/ms 20.38, stage/render/render 13.55, stage/main/post_update 12.25, render/graph/ms 12.13, stage/render/specialize 9.91, encode//sum 9.67, stage/render/graph_submit 9.33, stage/render/queue 6.85, encode/submit 6.11, encode/pass:shadow_point_light/sum 5.27 |
| measure | 18.16 | 32.7 | main/busy/ms 39.86, stage/main/update 25.41, render/thread/ms 22.47, render/busy/ms 21.38, stream/drive/ms 20.95, stage/render/render 13.20, stage/main/post_update 11.81, render/graph/ms 11.56, encode//sum 10.14, stage/render/graph_submit 8.81, encode/submit 5.68, encode/pass:shadow_point_light/sum 4.27 |
| measure | 19.48 | 30.1 | render/thread/ms 42.82, render/busy/ms 41.67, main/busy/ms 20.23, stage/render/render 13.33, encode//sum 11.88, render/graph/ms 11.74, stage/main/post_update 11.19, stage/render/specialize 10.58, stage/render/graph_submit 8.67, stage/render/queue 7.07, encode/pass:shadow_point_light/sum 6.80, encode/submit 5.75 |
| measure | 20.90 | 29.5 | render/thread/ms 38.32, render/busy/ms 37.31, main/busy/ms 18.86, stage/render/render 12.28, stage/main/post_update 11.85, render/graph/ms 10.87, encode//sum 10.86, stage/render/specialize 9.70, stage/render/graph_submit 8.26, stage/render/queue 6.99, encode/submit 4.99, encode/pass:shadow_point_light/sum 4.72 |
| measure | 23.04 | 32.6 | render/thread/ms 43.58, render/busy/ms 42.55, main/busy/ms 24.02, stage/render/render 14.53, render/graph/ms 12.98, stage/main/post_update 12.53, encode//sum 12.27, stage/render/specialize 11.41, stage/render/graph_submit 9.97, stage/render/queue 7.26, encode/pass:shadow_point_light/sum 6.89, stage/main/update 6.14 |
| measure | 24.70 | 30.0 | render/thread/ms 39.95, render/busy/ms 38.88, main/busy/ms 19.30, stage/render/render 13.99, encode//sum 12.65, render/graph/ms 12.55, stage/main/post_update 12.19, stage/render/specialize 9.96, stage/render/graph_submit 9.77, encode/pass:shadow_point_light/sum 7.25, stage/render/queue 6.74, encode/submit 6.33 |
| measure | 27.29 | 36.4 | render/thread/ms 46.21, render/busy/ms 44.69, main/busy/ms 28.02, stage/render/render 15.82, stage/main/post_update 13.94, render/graph/ms 13.87, encode//sum 12.72, stage/render/specialize 11.75, stage/render/graph_submit 10.89, encode/pass:shadow_point_light/sum 8.01, stage/render/queue 7.79, encode/submit 6.90 |
| measure | 28.92 | 32.6 | render/thread/ms 42.78, render/busy/ms 41.72, main/busy/ms 20.47, stage/render/render 14.58, render/graph/ms 13.09, stage/main/post_update 12.66, encode//sum 11.75, stage/render/specialize 10.36, stage/render/graph_submit 9.73, stage/render/queue 7.38, encode/pass:shadow_point_light/sum 7.13, encode/submit 5.82 |
| measure | 30.57 | 32.8 | render/thread/ms 42.42, render/busy/ms 41.42, main/busy/ms 19.74, stage/render/render 14.34, render/graph/ms 12.85, stage/main/post_update 12.22, encode//sum 10.80, stage/render/specialize 9.97, stage/render/graph_submit 9.80, encode/pass:shadow_point_light/sum 7.92, stage/render/queue 7.27, encode/submit 6.26 |
| measure | 32.06 | 29.9 | render/thread/ms 39.37, render/busy/ms 38.35, main/busy/ms 19.62, stage/render/render 13.23, stage/main/post_update 12.16, render/graph/ms 11.74, stage/render/specialize 9.81, encode//sum 9.62, stage/render/graph_submit 8.93, stage/render/queue 6.62, encode/pass:shadow_point_light/sum 6.59, encode/submit 5.88 |
| measure | 35.34 | 31.4 | render/thread/ms 38.06, render/busy/ms 36.78, main/busy/ms 20.07, stage/main/post_update 12.64, stage/render/render 11.57, stage/render/specialize 10.51, render/graph/ms 9.96, encode//sum 9.31, stage/render/graph_submit 7.62, stage/render/queue 6.77, encode/submit 4.44, stage/main/update 4.39 |
| measure | 37.48 | 29.5 | render/thread/ms 36.83, render/busy/ms 35.81, main/busy/ms 18.64, stage/main/post_update 11.43, stage/render/render 11.02, stage/render/specialize 9.90, render/graph/ms 9.56, encode//sum 9.41, stage/render/graph_submit 7.46, stage/render/queue 6.85, encode/submit 4.22, stage/main/update 4.19 |
| measure | 40.80 | 35.1 | render/thread/ms 39.20, render/busy/ms 38.10, main/busy/ms 19.03, stage/main/post_update 11.26, stage/render/render 10.69, stage/render/specialize 10.67, render/graph/ms 9.14, encode//sum 8.48, stage/render/graph_submit 7.24, stage/render/queue 6.81, stage/main/update 4.50, encode/submit 3.89 |
| measure | 43.21 | 32.1 | render/thread/ms 39.11, render/busy/ms 37.86, main/busy/ms 17.77, stage/render/render 12.13, stage/main/post_update 10.87, render/graph/ms 10.60, encode//sum 10.23, stage/render/specialize 9.67, stage/render/graph_submit 8.51, stage/render/queue 6.61, encode/finish_wall 4.41, stage/main/update 4.38 |
| measure | 45.65 | 34.4 | render/thread/ms 37.05, render/busy/ms 35.99, main/busy/ms 18.24, stage/main/post_update 10.59, stage/render/specialize 10.32, stage/render/render 9.87, encode//sum 9.07, render/graph/ms 8.44, stage/render/queue 6.88, stage/render/graph_submit 6.42, stage/main/update 4.55, stage/render/prepare_meshes 4.20 |
| measure | 56.43 | 29.7 | render/thread/ms 35.36, render/busy/ms 34.38, main/busy/ms 15.40, stage/render/render 9.73, stage/render/specialize 9.65, encode//sum 8.95, stage/main/post_update 8.94, render/graph/ms 8.39, stage/render/queue 6.35, stage/render/graph_submit 6.30, stage/main/update 3.89, stage/render/prepare_meshes 3.76 |
| measure | 58.95 | 47.7 | render/thread/ms 55.03, render/busy/ms 53.37, main/busy/ms 32.27, stage/main/post_update 18.26, stage/render/render 16.88, render/graph/ms 14.73, encode//sum 12.71, stage/render/specialize 11.49, stage/render/graph_submit 11.20, stage/main/update 9.25, stage/render/prepare_assets 7.13, stage/render/prepare_meshes 6.50 |
| measure | 62.57 | 30.7 | render/thread/ms 38.38, render/busy/ms 37.38, main/busy/ms 19.89, stage/main/post_update 10.59, stage/render/specialize 10.56, stage/render/render 10.44, render/graph/ms 9.17, encode//sum 8.55, stage/render/graph_submit 7.05, stage/render/queue 6.64, stage/main/update 6.25, stage/render/prepare_meshes 4.15 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_nomat\balmora_walk@2560x1440_arm=base-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 1%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **77%** (min 45%) · 2624 MHz (min 2610) · memory 8101 MHz · 78.6 W · 75 °C (max 80) · VRAM 6708 MiB · throttled 100% of samples

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
