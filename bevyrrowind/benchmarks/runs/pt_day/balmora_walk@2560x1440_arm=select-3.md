# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); select.gate=on (command line: --set); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6249 frames over 65.6 s · **median 10.27 ms (97 fps)** · mean 10.51 · p95 12.89 · p99 16.19 (1% low 62 fps) · worst 44.97 ms · 77 spikes over 1.5× median · 64.9 stutters/min (work 60.3/min)
- warm-up: 324 frames over 9.2 s, worst 2584.3 ms
- hitches (> 25.7 ms): **37** measured, 55 in warm-up
- timed GPU passes: 6.29 ms of a 10.51 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.27 ms, limit 11 ms
- **FAIL** frame_ms: 71 stutters (> 1.5x moving median of 60) in 1.09 min = 64.89/min, limit 1/min
- **FAIL** worst frame 44.97 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.800 | 2.034 | 2.067 | 0.89 | 0.180 |
| main_opaque_pass_3d | 1.429 | 1.614 | 2.031 | 0.89 | 0.065 |
| early prepass | 0.430 | 0.486 | 0.715 | 0.89 | 0.088 |
| bloom | 0.279 | 0.315 | 0.322 | 0.89 | 0.063 |
| atmosphere_luts | 0.268 | 0.303 | 0.365 | 0.89 | 0.003 |
| water_velocity | 0.248 | 0.280 | 0.347 | 0.89 | 0.002 |
| water_sspr | 0.178 | 0.202 | 0.203 | 0.89 | 0.002 |
| ssao | 0.167 | 0.188 | 0.193 | 0.89 | 0.007 |
| atmos_froxel_inject | 0.129 | 0.146 | 0.147 | 0.89 | 0.001 |
| clouds_trace | 0.124 | 0.140 | 0.232 | 0.89 | 0.001 |
| water_fft | 0.118 | 0.133 | 0.139 | 0.89 | 0.003 |
| uniform_allocation | 0.113 | 0.127 | 0.181 | 0.89 | 0.025 |
| upscaling | 0.105 | 0.119 | 0.126 | 0.89 | 0.004 |
| clouds_composite | 0.094 | 0.106 | 0.109 | 0.89 | 0.002 |
| bin_unpacking | 0.092 | 0.104 | 0.142 | 0.89 | 0.018 |
| auxiliary_views | 0.092 | 0.104 | 0.460 | 0.89 | 0.107 |
| main_transmissive_pass_3d | 0.075 | 0.085 | 0.178 | 0.89 | 0.009 |
| tonemapping | 0.075 | 0.085 | 0.088 | 0.89 | 0.004 |
| clustering | 0.056 | 0.064 | 0.067 | 0.89 | 0.018 |
| render_sky | 0.046 | 0.052 | 0.053 | 0.89 | 0.002 |
| clouds_resolve | 0.043 | 0.049 | 0.049 | 0.89 | 0.001 |
| surface_exposure | 0.038 | 0.044 | 0.045 | 0.88 | 0.001 |
| surface_trail | 0.034 | 0.056 | 0.057 | 0.60 | 0.002 |
| surface_exposure_far | 0.033 | 0.042 | 0.045 | 0.79 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.89 | 0.003 |
| lightprobe_irradiance_map | 0.027 | 0.412 | 0.416 | 0.07 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.030 | 0.046 | 0.89 | 0.005 |
| atmos_froxel_integrate | 0.026 | 0.029 | 0.032 | 0.89 | 0.001 |
| water_snapshot | 0.022 | 0.025 | 0.028 | 0.89 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.89 | 0.002 |
| lightprobe_radiance_map | 0.014 | 0.219 | 0.230 | 0.07 | 0.004 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.89 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.89 | 0.123 |
| clouds_fog_light | 0.010 | 0.012 | 0.012 | 0.89 | 0.002 |
| per_view_shadow_pass | 0.009 | 0.011 | 0.022 | 0.89 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.89 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.88 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.07 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.07 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.07 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.075 | 11.152 | 40.09 | 56709.6 |
| main/busy/ms | 8.531 | 10.121 | 28.19 | 53310.9 |
| render/busy/ms | 8.519 | 10.650 | 39.53 | 53234.6 |
| stage/main/post_update | 5.354 | 6.626 | 10.03 | 33455.8 |
| stage/render/render | 5.293 | 6.719 | 35.07 | 33073.8 |
| render/graph/ms | 4.545 | 5.973 | 34.33 | 28400.0 |
| encode//sum | 3.650 | 4.728 | 32.73 | 22806.1 |
| stage/render/graph_submit | 3.388 | 4.554 | 11.72 | 21170.0 |
| encode/submit | 2.162 | 3.137 | 6.58 | 13510.5 |
| stage/main/update | 1.925 | 2.324 | 21.90 | 12026.6 |
| encode/finish_wall | 1.308 | 1.921 | 5.56 | 8176.1 |
| stage/render/camera_driver | 1.103 | 1.504 | 30.38 | 6894.3 |
| encode/pass:shadow_directional_light/sum | 0.846 | 1.525 | 2.79 | 5289.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.757 | 1.051 | 1.93 | 4732.0 |
| encode/pass:early prepass/sum | 0.739 | 1.038 | 1.82 | 4620.9 |
| stage/render/prepare_write_phase_buffers | 0.721 | 1.046 | 2.95 | 4507.8 |
| encode/pass:shadow_point_light/sum | 0.578 | 2.726 | 16.68 | 3613.4 |
| stage/render/prepare_assets | 0.550 | 0.713 | 10.44 | 3434.2 |
| bigspace/total/ms | 0.508 | 1.220 | 2.00 | 3177.0 |
| render/present/ms | 0.464 | 1.214 | 2.69 | 2896.8 |
| stage/render/prepare_resources | 0.439 | 0.525 | 1.94 | 2740.9 |
| stage/render/prepare_meshes | 0.437 | 0.550 | 6.52 | 2731.6 |
| stage/main/first | 0.400 | 0.524 | 5.25 | 2497.2 |
| stage/main/pre_update | 0.353 | 0.514 | 1.70 | 2207.4 |
| stage/render/specialize | 0.307 | 0.396 | 11.64 | 1917.0 |
| stage/render/queue | 0.276 | 0.404 | 7.56 | 1726.4 |
| stage/render/prepare_bind_groups | 0.274 | 0.481 | 2.65 | 1711.2 |
| controller/feed/ms | 0.261 | 0.328 | 0.72 | 1629.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.235 | 0.350 | 0.99 | 1466.9 |
| bigspace/low_precision/ms | 0.210 | 0.647 | 1.68 | 1310.4 |
| stage/main/fixed_main_loop | 0.207 | 0.366 | 1.10 | 1294.5 |
| stream/drive/ms | 0.203 | 0.280 | 20.15 | 1267.9 |
| stage/render/create_views | 0.197 | 0.256 | 0.87 | 1229.1 |
| stage/render/prepare_batch_phases | 0.185 | 0.264 | 0.61 | 1155.2 |
| stage/render/extract_commands | 0.170 | 0.201 | 0.49 | 1065.4 |
| stage/render/prepare_flush | 0.160 | 0.301 | 1.51 | 1001.2 |
| anim/evaluate/ms | 0.156 | 0.253 | 2.21 | 971.8 |
| anim/evaluate_cpu/ms | 0.151 | 0.184 | 0.70 | 943.1 |
| stream/tick/ms | 0.147 | 0.214 | 0.54 | 920.7 |
| bigspace/high_precision/ms | 0.139 | 0.533 | 1.63 | 869.0 |
| stage/render/prepare_views | 0.136 | 0.172 | 0.55 | 850.5 |
| controller/move/ms | 0.131 | 0.298 | 0.64 | 819.7 |
| stage/main/last | 0.116 | 0.162 | 3.45 | 727.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.098 | 0.160 | 0.67 | 612.4 |
| render/acquire/ms | 0.093 | 0.120 | 0.52 | 578.2 |
| stage/render/prepare_collect_phase_buffers | 0.061 | 0.129 | 0.62 | 380.5 |
| terrain/update/ms | 0.057 | 0.284 | 1.45 | 353.4 |
| pipelines/compile_ms | 0.048 | 0.000 | 127.88 | 301.2 |
| pipelines/front_ms | 0.046 | 0.000 | 123.71 | 289.1 |
| physics/step/ms | 0.045 | 0.086 | 0.59 | 280.1 |
| stage/render/cleanup | 0.035 | 0.113 | 0.62 | 217.1 |
| physics/insert/ms | 0.030 | 0.342 | 0.84 | 187.9 |
| encode/resolve_diagnostics/sum | 0.030 | 0.040 | 0.52 | 187.6 |
| stage/render/phase_sort | 0.030 | 0.051 | 0.37 | 186.1 |
| weather/map/ms | 0.025 | 0.033 | 0.22 | 155.9 |
| terrain/select/ms | 0.023 | 0.253 | 0.66 | 140.9 |
| far/select/ms | 0.015 | 0.164 | 0.49 | 95.1 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.61 | 94.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.29 | 70.8 |
| anim/apply/ms | 0.011 | 0.020 | 0.44 | 70.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.21 | 65.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.14 | 65.0 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 43.6 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 32.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.06 | 30.7 |
| stream/spawn/ms | 0.004 | 0.000 | 1.08 | 27.6 |
| script/ui/ms | 0.004 | 0.006 | 0.13 | 26.5 |
| controller/write/ms | 0.004 | 0.005 | 0.14 | 22.1 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 13.3 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.17 | 12.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 6.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.53 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.16 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 561 | 579 | 578 |
| anim/characters/count | count | 562 | 579 | 578 |
| anim/evaluated/count | count | 20 | 33 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 15 |
| crowd/c0/joints/count | count | 1306 | 2262 | 707 |
| crowd/c0/meshes/count | count | 355 | 646 | 189 |
| crowd/c1/count | count | 145 | 156 | 146 |
| crowd/c1/joints/count | count | 5531 | 6023 | 5489 |
| crowd/c1/meshes/count | count | 1173 | 1297 | 1152 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4596 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4572 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.40 | 1.50 | 0.70 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.61 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.29 | 0.02 |
| encode/finish_critical | ms | 0.93 | 2.79 | 1.13 |
| encode/pass:early prepass/max | ms | 0.76 | 1.82 | 0.91 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.77 | 1.93 | 1.13 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.67 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 0.99 | 0.24 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.79 | 0.76 |
| encode/pass:shadow_point_light/max | ms | 0.75 | 1.35 | 0.82 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.52 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 8.84 | 39.53 | 8.30 |
| gpu/timed/ms | ms | 7.83 | 11.27 | 8.30 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 823 | 1416 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 823 | 1416 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1275 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3425 | 5137 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/bins | count | 1275 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3425 | 5137 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 13 | 231 | 4 |
| phases/shadow/bins | count | 5031 | 8793 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.1 k | 6386 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 203 | 247 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3182 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 182 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 11 | 621 | 9 |
| preprocess/bind_groups_created | count | 11 | 621 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 232 | 120 |
| process/cpu_usage | % | 26.44 | 39.62 | 21.15 |
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
| skin/blocks_changed/count | count | 13 | 131 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 369.4 k | 812.0 k | 500.7 k |
| skin/joints_written/count | count | 5772 | 12.7 k | 7824 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4916 | 5010 | 5010 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 4 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 3 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.46 | 52.16 | 30.29 |
| system/mem_usage | % | 54.67 | 55.16 | 55.12 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 61 | 278 | 54 |
| views/all/bins | count | 7611 | 11.7 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.1 k | 14.0 k |
| views/all/pipelines | count | 248 | 294 | 75 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 10.6 k | 33.3 k | 8127 |
| views/cascade/batch_sets | count | 9 | 53 | 4 |
| views/cascade/bins | count | 2808 | 6184 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8916 | 23.2 k | 6386 |
| views/cascade/pipelines | count | 36 | 53 | 29 |
| views/cascade/work_items | count | 409 | 22.2 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2147 | 2751 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2402 | 3048 | 0 |
| views/lamp_face/pipelines | count | 168 | 209 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2932 | 0 |
| views/main/batch_sets | count | 50 | 52 | 50 |
| views/main/bins | count | 2844 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8496 | 13.0 k | 7618 |
| views/main/pipelines | count | 46 | 48 | 46 |
| views/main/work_items | count | 8656 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 50.81 | 220 | 317.5 k |
| anim/events/count | count | 0.09 | 3 | 536 |
| assets/Image/added | count | 0.09 | 36 | 592 |
| assets/Image/modified | count | 0.00 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.65 | 201 | 4061 |
| assets/Mesh/removed | count | 0.77 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.47 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.75 | 1299 | 4675 |
| encode//count | count | 31.77 | 220 | 198.5 k |
| encode/bind_walk/init_textures | count | 11.5 k | 119.2 k | 71.98 M |
| encode/bind_walk/init_textures_skipped | count | 63.8 k | 141.2 k | 398.99 M |
| encode/bind_walk/inits | count | 449 | 1209 | 2.81 M |
| encode/bind_walk/inits_skipped | count | 106 | 161 | 664.2 k |
| encode/bind_walk/merge_textures | count | 11.4 k | 119.1 k | 71.53 M |
| encode/bind_walk/merge_textures_skipped | count | 5360 | 6012 | 33.50 M |
| encode/bind_walk/merges | count | 96.05 | 352 | 600.2 k |
| encode/bind_walk/merges_skipped | count | 82.71 | 132 | 516.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6249 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6249 |
| encode/pass:early prepass/count | count | 1 | 1 | 6249 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6249 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6249 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6249 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6249 |
| encode/pass:shadow_point_light/count | count | 0.96 | 48 | 6004 |
| encode/queue/command_buffers | count | 42.75 | 278 | 267.1 k |
| encode/queue/submits | count | 2.02 | 4 | 12.6 k |
| encode/queue/write_buffer/bytes | bytes | 1.87 M | 4.89 M | 11.71 G |
| encode/queue/write_buffer/calls | count | 250 | 927 | 1.56 M |
| encode/queue/write_buffer_with/bytes | bytes | 178.8 k | 3.52 M | 1.12 G |
| encode/queue/write_buffer_with/calls | count | 10.94 | 612 | 68.3 k |
| encode/queue/write_texture/bytes | bytes | 11.5 k | 8.39 M | 71.91 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6249 |
| physics/inserts | count | 0.36 | 102 | 2247 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.37 | 175 | 2335 |
| physics/steps | count | 0.63 | 3 | 3939 |
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
| stream/deactivate/items | count | 0.04 | 29 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5417 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 53 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1188 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.7 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2584.3 | main/busy/ms 2553, stage/main/post_update 2449, stage/main/update 95.58, surface/advance_cpu_ms 66.19, stage/main/pre_update 4.57, stage/main/last 1.39, stage/main/first 0.80, surface/cell_upload_ms 0.75, stream/drive/ms 0.61, stage/main/fixed_main_loop 0.35, stream/tick/ms 0.16, weather/map/ms 0.14 |
| warm-up | 3.34 | 756.9 | main/busy/ms 2699, stage/main/post_update 2477, render/thread/ms 724, render/busy/ms 554, stage/render/prepare_assets 502, stage/main/update 207, stage/render/prepare_views 165, render/acquire/ms 165, surface/advance_cpu_ms 66.19, stage/render/create_views 28.53, stage/render/render 15.62, render/graph/ms 8.96 |
| warm-up | 3.52 | 182.9 | render/thread/ms 890, render/busy/ms 622, stage/render/prepare_assets 556, main/busy/ms 175, stage/render/prepare_views 166, render/acquire/ms 165, pipelines/compile_ms 126, stage/main/update 118, stage/render/render 117, render/present/ms 102, pipelines/driver_ms 85.59, stage/main/post_update 41.64 |
| warm-up | 3.57 | 47.5 | pipelines/compile_ms 634, pipelines/front_ms 398, pipelines/wait_ms 251, pipelines/driver_ms 235, render/thread/ms 207, stage/render/render 137, render/present/ms 120, render/busy/ms 85.98, main/busy/ms 76.68, stage/render/prepare_assets 54.69, stage/main/post_update 53.55, terrain/update/ms 29.46 |
| warm-up | 3.72 | 147.4 | pipelines/wait_ms 4224, pipelines/compile_ms 1971, pipelines/front_ms 1708, pipelines/driver_ms 263, render/thread/ms 167, main/busy/ms 164, render/busy/ms 142, stage/render/prepare_assets 79.67, stage/main/post_update 68.48, stage/main/update 61.01, stage/render/render 42.66, stage/render/prepare_resources 33.00 |
| warm-up | 3.85 | 127.5 | pipelines/wait_ms 4342, pipelines/compile_ms 2421, pipelines/front_ms 2241, render/thread/ms 249, render/busy/ms 247, pipelines/driver_ms 180, stage/render/prepare_assets 180, main/busy/ms 149, stage/main/update 64.88, stage/main/post_update 47.32, stage/render/prepare_resources 32.68, terrain/update/ms 30.40 |
| warm-up | 3.88 | 37.3 | pipelines/compile_ms 1295, pipelines/front_ms 1199, pipelines/wait_ms 391, render/thread/ms 159, render/busy/ms 157, stage/render/prepare_assets 120, pipelines/driver_ms 96.14, main/busy/ms 56.35, stage/render/render 28.76, render/graph/ms 26.29, stage/main/post_update 25.43, stage/main/update 17.94 |
| warm-up | 3.96 | 29.9 | pipelines/compile_ms 813, pipelines/front_ms 790, pipelines/wait_ms 129, render/thread/ms 53.19, main/busy/ms 40.81, render/busy/ms 37.29, encode//sum 25.98, pipelines/driver_ms 22.97, stage/main/post_update 21.13, stage/render/prepare_views 15.49, stage/render/render 14.62, stage/main/update 14.19 |
| warm-up | 3.99 | 32.0 | pipelines/compile_ms 966, pipelines/front_ms 939, render/thread/ms 59.13, render/busy/ms 44.11, main/busy/ms 31.51, pipelines/driver_ms 27.30, stage/render/prepare_assets 23.11, stage/main/post_update 16.99, stage/render/prepare_views 13.58, render/acquire/ms 13.41, stage/render/render 12.58, render/graph/ms 10.25 |
| warm-up | 4.03 | 36.9 | pipelines/compile_ms 497, pipelines/front_ms 479, render/thread/ms 65.97, main/busy/ms 65.70, stage/main/update 48.13, render/busy/ms 45.17, stream/drive/ms 43.02, stage/render/prepare_assets 26.08, stage/render/prepare_views 19.52, render/acquire/ms 19.39, pipelines/driver_ms 18.57, stage/main/post_update 12.11 |
| warm-up | 4.09 | 60.3 | pipelines/compile_ms 1287, pipelines/front_ms 1240, render/thread/ms 68.52, main/busy/ms 63.77, render/busy/ms 55.54, stage/main/update 49.20, pipelines/driver_ms 46.93, stream/drive/ms 42.30, stage/render/prepare_assets 39.94, pipelines/wait_ms 12.88, stage/render/prepare_views 11.96, render/acquire/ms 11.86 |
| warm-up | 4.12 | 30.0 | pipelines/compile_ms 1320, pipelines/front_ms 1269, render/thread/ms 59.23, render/busy/ms 57.39, pipelines/driver_ms 51.28, main/busy/ms 48.43, stage/render/prepare_assets 33.00, stage/main/update 28.77, stage/render/render 19.10, render/graph/ms 16.78, pipelines/wait_ms 14.44, stage/main/post_update 12.97 |
| warm-up | 4.16 | 34.8 | pipelines/compile_ms 552, pipelines/front_ms 532, main/busy/ms 46.12, render/thread/ms 45.48, render/busy/ms 43.60, stage/main/update 26.06, pipelines/driver_ms 19.90, stage/render/prepare_assets 19.52, stage/render/render 18.72, render/graph/ms 16.42, stage/render/graph_submit 12.43, stage/main/post_update 12.27 |
| warm-up | 4.25 | 65.4 | pipelines/compile_ms 81.58, pipelines/front_ms 77.85, render/thread/ms 64.87, render/busy/ms 62.83, main/busy/ms 57.16, stage/render/prepare_assets 44.08, stage/main/update 35.17, stream/drive/ms 30.27, stage/main/post_update 12.86, stage/render/render 11.96, render/graph/ms 9.18, stage/render/graph_submit 6.55 |
| warm-up | 4.28 | 33.3 | pipelines/compile_ms 290, pipelines/front_ms 279, render/thread/ms 67.59, render/busy/ms 65.48, stage/render/prepare_assets 43.61, main/busy/ms 38.85, stage/main/update 15.04, stage/main/post_update 14.22, stage/render/render 13.44, render/graph/ms 10.58, pipelines/driver_ms 10.27, stage/render/graph_submit 7.98 |
| warm-up | 4.38 | 98.6 | pipelines/compile_ms 1616, pipelines/front_ms 1570, render/thread/ms 88.17, render/busy/ms 86.27, main/busy/ms 68.47, stage/render/prepare_assets 66.47, stage/main/update 47.94, pipelines/driver_ms 46.78, stage/render/render 12.56, stage/main/post_update 12.47, render/graph/ms 10.31, stage/render/graph_submit 8.20 |
| warm-up | 4.48 | 101.8 | pipelines/compile_ms 2350, pipelines/front_ms 2269, render/thread/ms 147, render/busy/ms 144, stage/render/prepare_assets 109, pipelines/driver_ms 81.26, main/busy/ms 68.67, stage/main/update 46.53, stage/render/render 27.63, render/graph/ms 24.50, stage/render/graph_submit 21.56, encode/submit 17.00 |
| warm-up | 4.53 | 44.8 | pipelines/compile_ms 1088, pipelines/front_ms 1039, render/thread/ms 121, render/busy/ms 117, stage/render/prepare_assets 73.80, pipelines/wait_ms 65.71, pipelines/driver_ms 48.42, main/busy/ms 42.39, stage/render/render 27.69, render/graph/ms 24.60, stage/render/graph_submit 20.44, encode/submit 17.88 |
| warm-up | 4.63 | 100.0 | pipelines/compile_ms 1013, pipelines/front_ms 978, pipelines/wait_ms 492, render/thread/ms 117, render/busy/ms 114, main/busy/ms 64.84, stage/render/prepare_assets 41.94, pipelines/driver_ms 34.72, stage/main/update 30.09, stage/main/post_update 26.78, stage/render/prepare_resources 26.53, stage/render/render 16.04 |
| warm-up | 4.68 | 52.1 | pipelines/compile_ms 1776, pipelines/front_ms 1719, pipelines/wait_ms 575, render/thread/ms 130, render/busy/ms 127, main/busy/ms 67.61, pipelines/driver_ms 57.48, encode//sum 51.68, stage/main/update 32.71, stage/render/prepare_assets 27.86, stage/main/post_update 27.70, stage/render/prepare_resources 26.85 |
| warm-up | 4.74 | 64.9 | pipelines/compile_ms 2072, pipelines/front_ms 2008, pipelines/wait_ms 1094, render/thread/ms 102, render/busy/ms 99.82, pipelines/driver_ms 63.59, encode//sum 52.62, main/busy/ms 45.28, stage/render/render 27.78, render/graph/ms 25.42, stage/render/prepare_assets 22.27, stage/main/update 19.93 |
| warm-up | 4.78 | 39.3 | pipelines/compile_ms 1651, pipelines/front_ms 1605, pipelines/wait_ms 1108, main/busy/ms 88.97, render/thread/ms 85.19, render/busy/ms 83.80, stage/main/update 66.61, stream/drive/ms 58.40, pipelines/driver_ms 46.13, stage/render/prepare_assets 29.57, stage/render/render 15.50, stage/render/prepare_resources 13.92 |
| warm-up | 4.86 | 72.2 | pipelines/compile_ms 1066, pipelines/front_ms 1034, pipelines/wait_ms 395, main/busy/ms 105, stage/main/update 79.96, render/thread/ms 61.41, render/busy/ms 58.52, stream/drive/ms 57.78, pipelines/driver_ms 32.48, stage/render/prepare_assets 16.77, stage/render/render 15.89, render/graph/ms 12.98 |
| warm-up | 4.92 | 59.3 | pipelines/compile_ms 1774, pipelines/front_ms 1714, pipelines/wait_ms 785, main/busy/ms 116, stage/main/update 91.93, render/thread/ms 67.21, render/busy/ms 64.29, pipelines/driver_ms 59.50, stream/drive/ms 51.52, stage/render/prepare_assets 22.39, stage/render/render 17.36, render/graph/ms 14.48 |
| warm-up | 5.02 | 106.9 | pipelines/compile_ms 1812, pipelines/front_ms 1754, pipelines/wait_ms 781, main/busy/ms 99.32, stage/main/update 78.47, render/thread/ms 77.57, render/busy/ms 76.11, pipelines/driver_ms 57.63, stream/drive/ms 53.12, stage/render/prepare_assets 40.14, stage/render/render 13.41, stage/main/post_update 12.65 |
| warm-up | 5.07 | 44.2 | pipelines/compile_ms 1193, pipelines/front_ms 1157, pipelines/wait_ms 235, main/busy/ms 83.67, render/thread/ms 70.01, render/busy/ms 68.59, stage/main/update 54.82, stream/drive/ms 42.61, stage/render/prepare_assets 42.16, pipelines/driver_ms 35.65, stage/main/post_update 19.53, stage/render/render 12.57 |
| warm-up | 5.15 | 82.9 | pipelines/compile_ms 1236, pipelines/front_ms 1202, main/busy/ms 92.31, stage/main/update 61.48, render/thread/ms 61.17, render/busy/ms 59.83, stream/drive/ms 41.54, stage/render/prepare_assets 37.97, pipelines/driver_ms 33.42, stage/main/post_update 22.06, stage/render/render 10.19, render/graph/ms 8.04 |
| warm-up | 5.19 | 36.5 | pipelines/compile_ms 731, pipelines/front_ms 713, main/busy/ms 71.71, render/thread/ms 56.50, render/busy/ms 55.14, stage/main/update 52.67, stream/drive/ms 32.65, stage/render/prepare_assets 29.67, pipelines/driver_ms 18.46, stage/main/post_update 12.41, stage/render/render 12.00, render/graph/ms 9.96 |
| warm-up | 5.23 | 44.2 | pipelines/compile_ms 791, pipelines/front_ms 771, main/busy/ms 64.15, stage/main/update 50.18, render/thread/ms 46.73, render/busy/ms 45.28, stream/drive/ms 32.46, pipelines/driver_ms 20.38, stage/render/prepare_assets 15.51, stage/render/render 11.78, render/graph/ms 9.82, encode//sum 8.06 |
| warm-up | 5.36 | 30.6 | pipelines/compile_ms 175, pipelines/front_ms 169, render/thread/ms 46.60, render/busy/ms 44.92, main/busy/ms 35.70, stage/main/post_update 15.12, stage/render/render 14.29, stage/main/update 14.10, render/graph/ms 11.70, stage/render/prepare_assets 11.51, encode//sum 9.05, stage/render/graph_submit 7.95 |
| warm-up | 5.40 | 41.5 | pipelines/compile_ms 114, pipelines/front_ms 110, render/thread/ms 59.95, render/busy/ms 58.47, main/busy/ms 40.76, stage/render/prepare_assets 28.97, stage/main/update 20.82, stage/render/render 15.45, render/graph/ms 13.18, stage/main/post_update 13.16, stage/render/graph_submit 9.50, encode//sum 8.32 |
| warm-up | 5.43 | 36.4 | pipelines/compile_ms 344, pipelines/front_ms 334, render/thread/ms 62.24, render/busy/ms 60.82, main/busy/ms 41.69, stage/render/prepare_assets 33.59, stage/main/update 25.75, stage/render/render 13.79, render/graph/ms 11.69, pipelines/driver_ms 10.92, stage/main/post_update 9.68, encode//sum 9.05 |
| warm-up | 5.49 | 59.4 | pipelines/compile_ms 852, pipelines/front_ms 827, render/thread/ms 84.38, render/busy/ms 83.05, main/busy/ms 55.37, stage/render/prepare_assets 35.87, pipelines/driver_ms 24.63, pipelines/wait_ms 21.51, stage/main/post_update 20.51, stage/main/update 18.85, stage/render/render 11.05, bigspace/total/ms 10.50 |
| warm-up | 5.55 | 34.3 | pipelines/compile_ms 596, pipelines/front_ms 582, render/thread/ms 45.02, render/busy/ms 43.57, main/busy/ms 31.23, stage/render/prepare_assets 18.55, stage/main/update 15.12, pipelines/driver_ms 14.35, stage/render/render 13.09, render/graph/ms 10.83, encode//sum 10.17, stage/main/post_update 10.01 |
| warm-up | 5.60 | 33.4 | pipelines/compile_ms 1157, pipelines/front_ms 1129, render/thread/ms 40.41, render/busy/ms 38.88, main/busy/ms 38.07, pipelines/driver_ms 27.51, stage/main/update 19.73, stage/render/prepare_assets 19.45, stream/drive/ms 13.64, stage/render/render 13.24, stage/main/post_update 12.81, render/graph/ms 10.93 |
| warm-up | 5.85 | 30.3 | pipelines/compile_ms 179, pipelines/front_ms 173, render/thread/ms 40.18, render/busy/ms 38.74, main/busy/ms 35.47, stage/main/post_update 16.05, stage/main/update 15.39, stage/render/render 13.38, render/graph/ms 11.27, stage/render/prepare_assets 10.80, encode//sum 9.41, stream/drive/ms 8.61 |
| warm-up | 5.93 | 32.3 | pipelines/compile_ms 938, pipelines/front_ms 917, main/busy/ms 39.28, render/thread/ms 24.97, stage/main/update 23.92, render/busy/ms 23.78, pipelines/driver_ms 20.44, stream/drive/ms 17.91, stage/main/post_update 9.91, stage/render/render 9.65, encode//sum 8.30, render/graph/ms 7.85 |
| warm-up | 6.27 | 31.6 | render/thread/ms 34.67, render/busy/ms 33.75, main/busy/ms 25.61, stage/render/prepare_assets 15.85, stage/main/update 11.22, stage/main/post_update 9.59, stage/render/render 8.65, render/graph/ms 7.31, encode//sum 6.74, stage/render/graph_submit 4.96, stream/drive/ms 3.61, encode/submit 2.90 |
| warm-up | 6.46 | 30.6 | main/busy/ms 44.19, render/thread/ms 39.37, render/busy/ms 37.75, stage/main/update 22.45, stage/render/render 14.07, stage/main/post_update 13.82, render/graph/ms 11.52, encode//sum 10.43, stage/render/graph_submit 8.04, stream/drive/ms 6.16, stage/render/prepare_assets 5.50, encode/submit 4.72 |
| warm-up | 6.49 | 30.7 | render/thread/ms 43.68, main/busy/ms 42.40, render/busy/ms 42.14, stage/main/update 19.40, stage/main/post_update 13.39, stage/render/prepare_assets 13.02, stage/render/render 12.34, render/graph/ms 10.09, encode//sum 9.85, stage/render/graph_submit 6.87, stream/drive/ms 6.56, stage/render/specialize 4.68 |
| warm-up | 6.52 | 29.9 | pipelines/compile_ms 86.81, pipelines/front_ms 80.94, render/thread/ms 41.51, render/busy/ms 40.48, main/busy/ms 30.48, stage/render/prepare_assets 21.40, stage/main/update 13.94, stage/main/post_update 9.87, stage/render/render 7.54, encode//sum 6.81, render/graph/ms 6.07, pipelines/driver_ms 5.87 |
| warm-up | 6.76 | 32.9 | pipelines/compile_ms 146, pipelines/front_ms 141, render/thread/ms 47.80, render/busy/ms 46.71, main/busy/ms 37.80, stage/render/prepare_assets 28.81, stage/main/update 24.89, stream/drive/ms 19.03, stage/render/render 11.02, stage/main/post_update 9.57, render/graph/ms 9.37, encode//sum 9.19 |
| warm-up | 7.10 | 34.4 | pipelines/compile_ms 139, pipelines/front_ms 136, render/thread/ms 40.85, render/busy/ms 39.61, main/busy/ms 16.89, stage/render/specialize 12.55, stage/render/render 11.57, render/graph/ms 9.85, stage/main/post_update 9.60, encode//sum 8.53, stage/render/graph_submit 7.69, stage/render/queue 7.04 |
| warm-up | 8.34 | 32.6 | render/thread/ms 39.07, render/busy/ms 37.90, main/busy/ms 19.35, stage/render/render 11.45, stage/main/post_update 11.28, render/graph/ms 9.87, stage/render/specialize 9.85, stage/render/graph_submit 7.60, encode//sum 7.51, stage/render/queue 6.65, stage/main/update 5.31, encode/submit 4.48 |
| measure | 0.04 | 33.8 | render/thread/ms 39.40, render/busy/ms 38.18, stage/render/render 33.56, render/graph/ms 32.13, stage/render/camera_driver 25.99, main/busy/ms 16.41, stage/main/post_update 9.47, encode//sum 7.51, stage/render/graph_submit 6.02, stage/main/update 4.21, encode/submit 3.46, encode/finish_wall 2.56 |
| measure | 1.65 | 32.6 | render/thread/ms 38.41, render/busy/ms 37.23, main/busy/ms 24.61, stage/main/post_update 14.51, stage/render/render 10.90, stage/render/specialize 10.32, render/graph/ms 9.39, encode//sum 7.43, stage/render/graph_submit 7.28, stage/render/queue 6.90, stage/main/update 6.37, encode/submit 4.42 |
| measure | 3.04 | 32.7 | render/thread/ms 36.70, render/busy/ms 35.63, main/busy/ms 17.35, stage/main/post_update 11.28, stage/render/render 10.94, render/graph/ms 9.46, stage/render/specialize 8.91, encode//sum 8.00, stage/render/graph_submit 6.80, stage/render/queue 5.94, stage/render/prepare_meshes 4.78, encode/submit 4.37 |
| measure | 3.76 | 41.1 | render/thread/ms 49.81, render/busy/ms 48.80, stage/render/render 41.31, render/graph/ms 39.94, stage/render/camera_driver 31.70, main/busy/ms 18.66, stage/main/post_update 12.29, encode//sum 10.26, stage/render/graph_submit 8.13, encode/pass:shadow_point_light/sum 5.53, encode/submit 5.14, stage/main/update 3.71 |
| measure | 4.47 | 30.7 | render/thread/ms 34.45, render/busy/ms 33.44, main/busy/ms 18.20, stage/main/post_update 12.22, stage/render/render 10.83, render/graph/ms 9.44, encode//sum 9.11, stage/render/specialize 9.02, stage/render/graph_submit 6.79, stage/render/queue 6.16, encode/submit 4.25, stage/main/update 3.49 |
| measure | 7.01 | 30.0 | render/thread/ms 34.75, render/busy/ms 33.35, main/busy/ms 18.02, stage/main/post_update 11.04, stage/render/render 10.72, stage/render/specialize 9.27, render/graph/ms 9.00, encode//sum 8.18, stage/render/graph_submit 6.59, stage/render/queue 6.46, encode/pass:shadow_point_light/sum 4.19, encode/submit 4.11 |
| measure | 15.82 | 32.3 | render/thread/ms 37.26, render/busy/ms 36.26, main/busy/ms 17.71, stage/render/render 11.13, stage/main/post_update 11.04, stage/render/specialize 9.90, render/graph/ms 9.71, encode//sum 7.94, stage/render/graph_submit 7.37, stage/render/queue 6.48, stage/main/update 4.07, encode/submit 4.01 |
| measure | 17.13 | 30.3 | render/thread/ms 37.67, render/busy/ms 36.49, main/busy/ms 18.30, stage/main/post_update 10.98, stage/render/render 10.70, stage/render/specialize 9.83, render/graph/ms 9.27, encode//sum 9.00, stage/render/queue 7.33, stage/render/graph_submit 6.90, stage/main/update 4.36, encode/submit 4.00 |
| measure | 23.01 | 32.7 | render/thread/ms 44.55, render/busy/ms 43.51, main/busy/ms 24.19, stage/render/render 12.69, stage/render/specialize 11.95, stage/main/post_update 11.82, render/graph/ms 11.11, encode//sum 9.67, stage/render/graph_submit 8.49, stage/render/queue 7.87, stage/main/update 6.35, encode/submit 5.00 |
| measure | 27.28 | 38.2 | render/thread/ms 45.60, render/busy/ms 44.58, main/busy/ms 24.60, stage/render/render 15.87, render/graph/ms 14.32, encode//sum 12.85, stage/main/post_update 12.65, stage/render/specialize 11.45, encode/pass:shadow_point_light/sum 10.96, stage/render/graph_submit 10.81, stage/render/queue 7.56, encode/submit 6.72 |
| measure | 30.60 | 32.3 | render/thread/ms 39.88, render/busy/ms 38.86, main/busy/ms 20.91, stage/main/post_update 12.27, stage/render/render 12.16, stage/render/specialize 10.93, render/graph/ms 10.65, encode//sum 8.40, stage/render/graph_submit 8.09, stage/render/queue 7.00, stage/main/update 5.19, encode/pass:shadow_point_light/sum 5.03 |
| measure | 35.96 | 30.4 | render/thread/ms 36.23, render/busy/ms 35.09, main/busy/ms 20.41, stage/main/post_update 11.09, stage/render/render 10.62, stage/render/specialize 9.61, render/graph/ms 9.16, stage/render/graph_submit 7.04, stage/render/queue 7.03, encode//sum 7.03, stage/main/update 5.05, encode/submit 4.23 |
| measure | 37.27 | 32.2 | render/thread/ms 40.52, render/busy/ms 39.44, main/busy/ms 22.45, stage/main/post_update 12.30, stage/render/specialize 11.89, stage/render/render 10.19, render/graph/ms 8.72, stage/render/queue 7.46, encode//sum 7.03, stage/render/graph_submit 6.81, stage/main/update 6.40, encode/submit 3.70 |
| measure | 37.48 | 31.2 | render/thread/ms 35.16, render/busy/ms 34.22, main/busy/ms 18.66, stage/main/post_update 9.98, stage/render/specialize 9.71, stage/render/render 9.69, render/graph/ms 8.35, encode//sum 7.22, stage/render/graph_submit 6.48, stage/render/queue 6.33, stage/main/update 5.96, encode/submit 3.88 |
| measure | 41.17 | 32.9 | render/thread/ms 38.21, render/busy/ms 37.08, main/busy/ms 19.95, stage/main/post_update 11.52, stage/render/render 10.07, stage/render/specialize 9.95, render/graph/ms 8.58, stage/render/graph_submit 6.83, stage/render/queue 6.63, encode//sum 6.63, stage/main/update 5.12, encode/submit 3.95 |
| measure | 46.11 | 32.5 | render/thread/ms 39.07, render/busy/ms 37.97, main/busy/ms 18.39, stage/render/render 11.07, stage/main/post_update 10.75, stage/render/specialize 10.38, render/graph/ms 9.57, encode//sum 7.97, stage/render/graph_submit 7.60, stage/render/queue 6.70, stage/main/update 4.60, stage/render/prepare_meshes 4.33 |
| measure | 56.43 | 30.3 | render/thread/ms 34.07, render/busy/ms 33.01, main/busy/ms 14.83, stage/render/render 9.50, stage/render/specialize 9.19, stage/main/post_update 8.86, render/graph/ms 8.23, encode//sum 6.96, stage/render/queue 6.26, stage/render/graph_submit 6.10, stage/main/update 3.72, stage/render/prepare_meshes 3.61 |
| measure | 59.20 | 45.0 | render/thread/ms 54.11, render/busy/ms 52.52, main/busy/ms 29.72, stage/main/post_update 17.33, stage/render/render 17.11, render/graph/ms 15.10, stage/render/graph_submit 11.91, stage/render/specialize 11.49, encode//sum 9.49, encode/submit 8.00, stage/main/update 7.83, stage/render/prepare_meshes 6.96 |
| measure | 59.23 | 31.6 | render/thread/ms 58.41, render/busy/ms 57.32, main/busy/ms 24.98, stage/render/prepare_assets 16.28, stage/main/post_update 12.65, stage/render/specialize 11.63, stage/render/render 10.33, render/graph/ms 9.01, stage/render/prepare_meshes 7.65, encode//sum 7.28, stage/render/graph_submit 7.00, stage/main/update 6.52 |
| measure | 63.17 | 32.1 | render/thread/ms 38.89, render/busy/ms 37.82, main/busy/ms 18.69, stage/render/render 11.92, stage/render/specialize 10.66, render/graph/ms 10.62, stage/main/post_update 10.05, encode//sum 9.14, stage/render/graph_submit 8.43, stage/render/queue 6.44, stage/main/update 5.95, encode/submit 4.67 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set select.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=select-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 5%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **79%** (min 35%) · 2621 MHz (min 2610) · memory 8101 MHz · 82.4 W · 78 °C (max 83) · VRAM 7056 MiB · throttled 100% of samples

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
- `select.gate=on (command line: --set)`
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
