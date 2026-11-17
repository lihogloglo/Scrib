# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5906 frames over 65.6 s · **median 10.65 ms (94 fps)** · mean 11.12 · p95 14.79 · p99 17.69 (1% low 57 fps) · worst 30.26 ms · 145 spikes over 1.5× median · 21.9 stutters/min (work 22.9/min)
- warm-up: 322 frames over 9.5 s, worst 2504.1 ms
- hitches (> 26.6 ms): **3** measured, 47 in warm-up
- timed GPU passes: 6.64 ms of a 11.12 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.65 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.94/min, limit 1/min
- pass worst frame 30.26 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.971 | 2.031 | 2.063 | 0.97 | 0.176 |
| main_opaque_pass_3d | 1.276 | 1.315 | 1.658 | 0.97 | 0.061 |
| early prepass | 0.479 | 0.494 | 0.719 | 0.97 | 0.084 |
| bloom | 0.306 | 0.316 | 0.322 | 0.97 | 0.064 |
| atmosphere_luts | 0.300 | 0.309 | 0.373 | 0.97 | 0.003 |
| auxiliary_views | 0.289 | 0.298 | 1.086 | 0.97 | 0.269 |
| water_velocity | 0.254 | 0.262 | 0.349 | 0.97 | 0.002 |
| water_sspr | 0.203 | 0.209 | 0.210 | 0.97 | 0.002 |
| ssao | 0.183 | 0.188 | 0.193 | 0.97 | 0.007 |
| uniform_allocation | 0.142 | 0.147 | 0.177 | 0.97 | 0.025 |
| water_fft | 0.126 | 0.129 | 0.137 | 0.97 | 0.003 |
| upscaling | 0.125 | 0.129 | 0.137 | 0.97 | 0.004 |
| atmos_froxel_inject | 0.124 | 0.127 | 0.129 | 0.97 | 0.001 |
| clouds_composite | 0.101 | 0.104 | 0.108 | 0.97 | 0.002 |
| bin_unpacking | 0.098 | 0.101 | 0.125 | 0.97 | 0.018 |
| tonemapping | 0.084 | 0.086 | 0.091 | 0.97 | 0.003 |
| main_transmissive_pass_3d | 0.077 | 0.079 | 0.156 | 0.97 | 0.008 |
| clustering | 0.062 | 0.064 | 0.067 | 0.97 | 0.019 |
| clouds_trace | 0.055 | 0.056 | 0.075 | 0.97 | 0.001 |
| render_sky | 0.050 | 0.051 | 0.052 | 0.97 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.97 | 0.001 |
| lightprobe_irradiance_map | 0.041 | 0.411 | 0.416 | 0.10 | 0.001 |
| surface_trail | 0.039 | 0.056 | 0.057 | 0.70 | 0.002 |
| early_mesh_preprocessing | 0.035 | 0.036 | 0.046 | 0.97 | 0.004 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.97 | 0.001 |
| water_snapshot | 0.024 | 0.025 | 0.029 | 0.97 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.219 | 0.229 | 0.10 | 0.004 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.97 | 0.002 |
| main_transparent_pass_3d | 0.016 | 0.017 | 0.031 | 0.97 | 0.172 |
| night_sky | 0.016 | 0.016 | 0.038 | 0.97 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.97 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| per_view_shadow_pass | 0.009 | 0.010 | 0.015 | 0.97 | 0.004 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.97 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.97 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.721 | 13.266 | 23.09 | 57411.8 |
| render/busy/ms | 9.204 | 12.738 | 22.55 | 54356.0 |
| main/busy/ms | 8.755 | 10.730 | 27.99 | 51709.7 |
| stage/render/render | 5.807 | 8.438 | 14.43 | 34295.5 |
| stage/main/post_update | 5.331 | 6.930 | 10.72 | 31485.4 |
| render/graph/ms | 5.075 | 7.631 | 13.50 | 29975.5 |
| encode//sum | 4.500 | 7.556 | 23.49 | 26577.2 |
| stage/render/graph_submit | 3.725 | 5.512 | 10.08 | 21997.2 |
| encode/submit | 2.385 | 3.659 | 5.94 | 14083.2 |
| stage/main/update | 2.162 | 2.579 | 21.96 | 12770.7 |
| encode/pass:shadow_point_light/sum | 1.475 | 4.980 | 15.88 | 8709.0 |
| encode/finish_wall | 1.385 | 1.967 | 5.46 | 8177.6 |
| stage/render/camera_driver | 1.296 | 2.089 | 3.51 | 7656.8 |
| stage/render/prepare_write_phase_buffers | 0.764 | 1.127 | 2.84 | 4511.5 |
| encode/pass:shadow_directional_light/sum | 0.749 | 1.264 | 2.20 | 4421.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.719 | 1.062 | 1.78 | 4246.7 |
| encode/pass:early prepass/sum | 0.701 | 1.043 | 4.38 | 4141.2 |
| stage/render/prepare_assets | 0.576 | 0.727 | 10.81 | 3401.1 |
| bigspace/total/ms | 0.522 | 1.256 | 2.16 | 3085.2 |
| stage/render/prepare_meshes | 0.455 | 0.611 | 2.20 | 2690.1 |
| stage/render/prepare_resources | 0.448 | 0.533 | 1.15 | 2644.9 |
| render/present/ms | 0.425 | 0.531 | 3.86 | 2512.9 |
| stage/main/first | 0.401 | 0.526 | 5.09 | 2365.4 |
| stage/main/pre_update | 0.348 | 0.497 | 1.55 | 2055.3 |
| terrain/update/ms | 0.311 | 0.399 | 2.00 | 1838.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.298 | 0.439 | 1.18 | 1758.8 |
| stage/render/specialize | 0.278 | 0.425 | 6.07 | 1640.3 |
| terrain/select/ms | 0.277 | 0.357 | 0.99 | 1638.8 |
| stage/render/prepare_bind_groups | 0.267 | 0.493 | 2.24 | 1575.0 |
| stage/render/prepare_batch_phases | 0.264 | 0.372 | 0.76 | 1560.0 |
| stage/render/queue | 0.259 | 0.409 | 3.00 | 1527.5 |
| controller/feed/ms | 0.257 | 0.321 | 1.06 | 1517.7 |
| stream/drive/ms | 0.238 | 0.315 | 19.92 | 1406.5 |
| stage/main/fixed_main_loop | 0.219 | 0.367 | 1.14 | 1291.7 |
| bigspace/low_precision/ms | 0.214 | 0.607 | 1.76 | 1263.3 |
| far/select/ms | 0.201 | 0.269 | 0.65 | 1188.6 |
| stage/render/create_views | 0.191 | 0.247 | 0.86 | 1127.2 |
| stream/tick/ms | 0.183 | 0.245 | 0.67 | 1080.6 |
| stage/render/extract_commands | 0.167 | 0.195 | 0.46 | 986.2 |
| stage/render/prepare_flush | 0.165 | 0.328 | 0.93 | 975.1 |
| anim/evaluate/ms | 0.164 | 0.262 | 1.60 | 970.1 |
| anim/evaluate_cpu/ms | 0.159 | 0.203 | 0.78 | 937.8 |
| controller/move/ms | 0.153 | 0.325 | 0.64 | 902.9 |
| bigspace/high_precision/ms | 0.145 | 0.594 | 1.71 | 857.4 |
| stage/render/prepare_views | 0.134 | 0.167 | 0.53 | 794.0 |
| stage/main/last | 0.118 | 0.165 | 1.87 | 695.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.099 | 0.165 | 0.62 | 585.8 |
| render/acquire/ms | 0.092 | 0.118 | 0.45 | 542.8 |
| stage/render/cleanup | 0.079 | 0.199 | 0.77 | 467.5 |
| stage/render/prepare_collect_phase_buffers | 0.064 | 0.135 | 0.54 | 376.3 |
| stage/render/phase_sort | 0.062 | 0.090 | 0.32 | 367.6 |
| physics/step/ms | 0.048 | 0.087 | 0.53 | 283.2 |
| physics/insert/ms | 0.035 | 0.382 | 0.92 | 204.5 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.46 | 178.1 |
| weather/map/ms | 0.016 | 0.021 | 0.19 | 94.6 |
| pipelines/compile_ms | 0.016 | 0.000 | 80.94 | 92.2 |
| pipelines/front_ms | 0.015 | 0.000 | 77.90 | 89.0 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.17 | 83.0 |
| anim/apply/ms | 0.012 | 0.024 | 0.53 | 68.3 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.45 | 62.7 |
| controller/avoid/ms | 0.011 | 0.014 | 0.23 | 62.5 |
| weather/eval/ms | 0.010 | 0.013 | 0.16 | 57.1 |
| script/update/ms | 0.007 | 0.009 | 0.29 | 40.2 |
| stream/despawn/ms | 0.005 | 0.000 | 1.07 | 30.3 |
| script/input/ms | 0.005 | 0.006 | 0.11 | 29.9 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 28.9 |
| script/ui/ms | 0.005 | 0.006 | 0.49 | 27.2 |
| controller/write/ms | 0.003 | 0.004 | 0.09 | 19.6 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.67 | 13.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.7 |
| pipelines/driver_ms | 0.001 | 0.000 | 3.04 | 3.3 |
| stage/render/prepare | 0.001 | 0.002 | 0.17 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
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
| controller/movers/count | count | 19 | 22 | 17 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1291 | 2262 | 537 |
| crowd/c0/meshes/count | count | 353 | 646 | 146 |
| crowd/c1/count | count | 147 | 156 | 149 |
| crowd/c1/joints/count | count | 5573 | 5995 | 5659 |
| crowd/c1/meshes/count | count | 1180 | 1297 | 1195 |
| crowd/c2/count | count | 399 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 163 | 216 | 204 |
| crowd/joints_extracted/count | count | 3677 | 8590 | 5731 |
| crowd/joints_uploaded/count | count | 3642 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.42 | 5.43 | 0.40 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.17 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.45 | 0.01 |
| encode/finish_critical | ms | 0.93 | 5.43 | 0.90 |
| encode/pass:early prepass/max | ms | 0.72 | 4.38 | 0.77 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.73 | 1.78 | 0.90 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.62 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.28 | 1.18 | 0.29 |
| encode/pass:shadow_directional_light/max | ms | 0.75 | 2.20 | 0.81 |
| encode/pass:shadow_point_light/max | ms | 0.79 | 1.37 | 0.81 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.46 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.04 | 27.99 | 8.68 |
| gpu/timed/ms | ms | 7.54 | 10.80 | 7.70 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 254 | 479 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3955 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 254 | 479 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3955 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 4120 | 4335 | 4335 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 4120 | 4335 | 4335 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 23 | 193 | 5 |
| phases/shadow/bins | count | 10.4 k | 11.7 k | 7980 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 27.0 k | 29.1 k | 25.7 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 198 | 232 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2346 |
| physics/items | count | 194 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 16 | 551 | 6 |
| preprocess/bind_groups_created | count | 16 | 551 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 136 | 303 | 129 |
| process/cpu_usage | % | 25 | 35.41 | 22.12 |
| process/mem_usage | GiB | 5.45 | 5.61 | 5.61 |
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
| skin/blocks_changed/count | count | 13 | 106 | 10 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 163 | 216 | 204 |
| skin/joint_bytes_written/bytes | bytes | 280.1 k | 808.1 k | 443.9 k |
| skin/joints_written/count | count | 4376 | 12.6 k | 6936 |
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
| stream/despawn/queue | count | 0 | 1 | 0 |
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
| system/cpu_usage | % | 30.21 | 45.07 | 29.78 |
| system/mem_usage | % | 54.69 | 55.23 | 55.14 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 72 | 244 | 57 |
| views/all/bins | count | 19.0 k | 20.4 k | 17.6 k |
| views/all/count | count | 45 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 55.0 k | 57.5 k | 55.2 k |
| views/all/pipelines | count | 245 | 281 | 68 |
| views/all/pipelines_distinct | count | 71 | 76 | 63 |
| views/all/work_items | count | 41.5 k | 56.1 k | 42.2 k |
| views/cascade/batch_sets | count | 11 | 28 | 5 |
| views/cascade/bins | count | 7670 | 7994 | 7980 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 24.5 k | 25.7 k | 25.7 k |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 11.7 k | 25.7 k | 11.9 k |
| views/lamp_face/batch_sets | count | 0 | 184 | 0 |
| views/lamp_face/bins | count | 3449 | 4503 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3661 | 4819 | 0 |
| views/lamp_face/pipelines | count | 174 | 204 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 3994 | 0 |
| views/main/batch_sets | count | 51 | 55 | 52 |
| views/main/bins | count | 8730 | 9612 | 9612 |
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
| world/meshes | count | 31.8 k | 37.9 k | 30.8 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 63.59 | 365 | 375.5 k |
| anim/events/count | count | 0.09 | 2 | 510 |
| assets/Image/added | count | 0.10 | 34 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.69 | 152 | 4061 |
| assets/Mesh/removed | count | 0.82 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.67 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.79 | 1296 | 4676 |
| encode//count | count | 39.03 | 219 | 230.5 k |
| encode/bind_walk/init_textures | count | 15.6 k | 112.9 k | 92.29 M |
| encode/bind_walk/init_textures_skipped | count | 64.4 k | 152.0 k | 380.09 M |
| encode/bind_walk/inits | count | 538 | 1188 | 3.18 M |
| encode/bind_walk/inits_skipped | count | 132 | 215 | 778.9 k |
| encode/bind_walk/merge_textures | count | 15.6 k | 112.9 k | 91.92 M |
| encode/bind_walk/merge_textures_skipped | count | 5533 | 6026 | 32.68 M |
| encode/bind_walk/merges | count | 109 | 337 | 644.7 k |
| encode/bind_walk/merges_skipped | count | 108 | 163 | 638.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5906 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5906 |
| encode/pass:early prepass/count | count | 1 | 1 | 5906 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5906 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5906 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5906 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5906 |
| encode/pass:shadow_point_light/count | count | 3.02 | 48 | 17.8 k |
| encode/queue/command_buffers | count | 52.07 | 277 | 307.5 k |
| encode/queue/submits | count | 2.02 | 4 | 11.9 k |
| encode/queue/write_buffer/bytes | bytes | 2.25 M | 4.33 M | 13.32 G |
| encode/queue/write_buffer/calls | count | 276 | 823 | 1.63 M |
| encode/queue/write_buffer_with/bytes | bytes | 169.9 k | 3.54 M | 1.00 G |
| encode/queue/write_buffer_with/calls | count | 11.05 | 465 | 65.2 k |
| encode/queue/write_texture/bytes | bytes | 10.7 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5906 |
| physics/inserts | count | 0.38 | 111 | 2266 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.67 | 2 | 3939 |
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
| stream/upload/bytes | bytes | 5732 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1257 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.6 ms; slowest 50, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2504.1 | main/busy/ms 2472, stage/main/post_update 2431, stage/main/update 32.67, stage/main/pre_update 4.26, stage/main/last 1.08, surface/advance_cpu_ms 0.73, stage/main/first 0.55, stage/main/fixed_main_loop 0.48, stream/drive/ms 0.43, surface/cell_upload_ms 0.35, stream/tick/ms 0.12, bigspace/total/ms 0.10 |
| warm-up | 3.33 | 827.8 | main/busy/ms 2657, stage/main/post_update 2461, render/thread/ms 793, render/busy/ms 628, stage/render/prepare_assets 577, stage/main/update 181, stage/render/prepare_views 160, render/acquire/ms 160, stage/render/create_views 29.60, stage/render/render 14.46, render/graph/ms 7.90, stage/render/prepare_resources 7.25 |
| warm-up | 3.47 | 133.5 | render/thread/ms 907, render/busy/ms 683, stage/render/prepare_assets 621, main/busy/ms 212, stage/render/prepare_views 161, render/acquire/ms 160, stage/main/update 155, pipelines/compile_ms 85.30, stage/render/render 76.32, render/present/ms 63.64, pipelines/driver_ms 47.36, stage/main/post_update 41.62 |
| warm-up | 3.66 | 166.8 | pipelines/wait_ms 4247, pipelines/compile_ms 1890, pipelines/front_ms 1702, pipelines/driver_ms 188, main/busy/ms 153, render/thread/ms 134, render/busy/ms 130, stage/main/update 70.80, stage/render/render 63.86, render/graph/ms 58.91, stage/render/graph_submit 46.20, encode//sum 34.52 |
| warm-up | 3.73 | 71.2 | pipelines/wait_ms 4439, pipelines/compile_ms 2153, pipelines/front_ms 1988, render/thread/ms 177, render/busy/ms 174, pipelines/driver_ms 165, main/busy/ms 128, stage/render/prepare_assets 79.15, stage/main/update 57.87, stage/render/render 50.58, render/graph/ms 46.85, stage/render/graph_submit 44.48 |
| warm-up | 3.76 | 32.0 | pipelines/compile_ms 877, pipelines/front_ms 822, pipelines/wait_ms 215, render/thread/ms 96.89, render/busy/ms 94.89, stage/render/prepare_assets 77.68, pipelines/driver_ms 54.83, main/busy/ms 45.38, stage/main/post_update 16.04, terrain/update/ms 15.26, stage/main/update 12.51, stage/render/render 10.48 |
| warm-up | 3.80 | 39.4 | pipelines/compile_ms 853, pipelines/front_ms 806, pipelines/wait_ms 191, render/thread/ms 66.74, render/busy/ms 64.57, pipelines/driver_ms 46.81, main/busy/ms 39.70, stage/render/prepare_assets 36.60, stage/main/post_update 18.63, terrain/update/ms 10.27, stage/render/render 9.86, stage/render/prepare_resources 8.93 |
| warm-up | 3.85 | 31.6 | pipelines/compile_ms 844, pipelines/front_ms 812, render/thread/ms 50.46, render/busy/ms 48.49, pipelines/driver_ms 31.60, main/busy/ms 27.18, encode//sum 22.25, stage/render/prepare_assets 18.80, stage/main/post_update 15.86, stage/render/render 14.92, render/graph/ms 12.74, pipelines/wait_ms 10.04 |
| warm-up | 3.97 | 96.7 | pipelines/compile_ms 1337, pipelines/front_ms 1285, main/busy/ms 62.51, render/thread/ms 60.67, render/busy/ms 58.56, pipelines/driver_ms 52.12, stage/main/update 45.74, stream/drive/ms 40.22, stage/render/prepare_assets 39.52, pipelines/wait_ms 14.57, stage/render/render 13.57, render/graph/ms 11.22 |
| warm-up | 4.01 | 27.8 | pipelines/compile_ms 115, pipelines/front_ms 110, render/thread/ms 34.48, render/busy/ms 32.70, main/busy/ms 25.99, stage/render/prepare_assets 15.26, stage/main/update 14.66, stage/render/render 13.32, render/graph/ms 11.44, stream/drive/ms 10.03, stage/render/graph_submit 8.91, stage/main/post_update 6.42 |
| warm-up | 4.09 | 69.1 | pipelines/compile_ms 220, pipelines/front_ms 209, render/thread/ms 60.08, render/busy/ms 56.73, main/busy/ms 48.57, stage/render/prepare_assets 37.20, stage/main/update 31.69, stream/drive/ms 22.68, stage/render/render 15.85, render/graph/ms 12.19, pipelines/driver_ms 11.06, stage/main/post_update 10.22 |
| warm-up | 4.13 | 31.7 | pipelines/compile_ms 236, pipelines/front_ms 228, render/thread/ms 68.05, render/busy/ms 65.63, main/busy/ms 44.97, stage/render/prepare_assets 40.60, stage/main/update 23.40, stage/render/render 15.68, stage/main/post_update 13.61, render/graph/ms 12.74, stage/render/graph_submit 8.91, pipelines/driver_ms 8.14 |
| warm-up | 4.25 | 125.4 | pipelines/compile_ms 1202, pipelines/front_ms 1165, render/thread/ms 110, render/busy/ms 108, main/busy/ms 61.81, stage/render/prepare_assets 51.37, stage/render/render 46.72, render/graph/ms 44.11, stage/main/update 43.47, stage/render/graph_submit 39.20, pipelines/driver_ms 36.65, encode//sum 31.08 |
| warm-up | 4.32 | 69.5 | pipelines/compile_ms 1668, pipelines/front_ms 1613, render/thread/ms 137, render/busy/ms 136, pipelines/wait_ms 93.19, stage/render/prepare_assets 76.88, main/busy/ms 66.61, pipelines/driver_ms 55.58, stage/render/render 46.99, stage/main/update 46.98, render/graph/ms 44.84, stage/render/graph_submit 39.87 |
| warm-up | 4.38 | 61.5 | pipelines/compile_ms 651, pipelines/front_ms 625, pipelines/wait_ms 202, render/thread/ms 98.53, render/busy/ms 96.61, main/busy/ms 64.23, stage/render/prepare_assets 59.99, pipelines/driver_ms 25.97, stage/main/update 23.82, stage/main/post_update 21.04, stage/render/render 17.22, encode//sum 16.91 |
| warm-up | 4.46 | 81.7 | pipelines/compile_ms 1949, pipelines/front_ms 1882, pipelines/wait_ms 510, render/thread/ms 128, render/busy/ms 126, pipelines/driver_ms 67.13, main/busy/ms 66.10, encode//sum 62.11, stage/render/prepare_assets 36.40, stage/main/update 29.37, stage/render/render 27.86, render/graph/ms 25.16 |
| warm-up | 4.55 | 85.4 | pipelines/compile_ms 2296, pipelines/front_ms 2219, pipelines/wait_ms 390, render/thread/ms 157, render/busy/ms 155, main/busy/ms 77.92, pipelines/driver_ms 76.99, encode//sum 60.02, stage/main/post_update 42.39, stage/render/prepare_resources 41.40, stage/render/prepare_assets 34.48, stage/render/render 31.03 |
| warm-up | 4.61 | 59.0 | pipelines/compile_ms 1917, pipelines/front_ms 1845, pipelines/wait_ms 185, render/thread/ms 130, render/busy/ms 128, pipelines/driver_ms 72.20, main/busy/ms 64.71, encode//sum 55.78, stage/main/post_update 40.15, stage/render/prepare_assets 35.72, stage/render/render 31.88, stage/render/prepare_resources 29.87 |
| warm-up | 4.64 | 29.7 | pipelines/compile_ms 1985, pipelines/front_ms 1915, pipelines/wait_ms 365, main/busy/ms 76.08, render/thread/ms 75.09, render/busy/ms 73.34, pipelines/driver_ms 70.20, stage/main/update 56.88, encode//sum 50.66, stream/drive/ms 45.37, stage/render/render 29.26, render/graph/ms 26.78 |
| warm-up | 4.72 | 84.4 | pipelines/compile_ms 1018, pipelines/front_ms 992, pipelines/wait_ms 286, main/busy/ms 109, stage/main/update 80.26, render/thread/ms 69.08, render/busy/ms 67.22, stream/drive/ms 46.58, stage/render/render 39.01, render/graph/ms 36.29, pipelines/driver_ms 25.81, stage/render/camera_driver 21.62 |
| warm-up | 4.81 | 89.3 | pipelines/compile_ms 1014, pipelines/front_ms 984, main/busy/ms 133, render/thread/ms 109, stage/main/update 104, render/busy/ms 100, pipelines/wait_ms 94.90, stream/drive/ms 53.55, stage/render/render 53.23, render/graph/ms 43.13, pipelines/driver_ms 29.75, stage/render/prepare_assets 29.15 |
| warm-up | 4.93 | 115.2 | pipelines/compile_ms 2428, pipelines/front_ms 2365, render/thread/ms 128, render/busy/ms 120, main/busy/ms 111, stage/main/update 85.32, stage/render/prepare_assets 72.04, pipelines/driver_ms 63.34, stream/drive/ms 52.85, stage/render/render 32.81, pipelines/wait_ms 32.20, pipelines/lock_ms 31.66 |
| warm-up | 4.98 | 56.8 | pipelines/compile_ms 1953, pipelines/front_ms 1876, render/thread/ms 116, render/busy/ms 110, main/busy/ms 91.65, stage/render/prepare_assets 82.02, pipelines/driver_ms 76.47, stage/main/update 65.58, stream/drive/ms 46.54, pipelines/wait_ms 34.67, pipelines/lock_ms 34.03, stage/render/render 19.38 |
| warm-up | 5.05 | 67.7 | pipelines/compile_ms 128, pipelines/front_ms 95.33, render/thread/ms 91.27, render/busy/ms 85.39, main/busy/ms 85.35, stage/main/update 65.26, stage/render/prepare_assets 59.44, stream/drive/ms 46.15, pipelines/driver_ms 32.36, stage/render/render 17.70, stage/main/post_update 12.01, render/graph/ms 11.11 |
| warm-up | 5.08 | 30.9 | main/busy/ms 73.92, render/thread/ms 65.90, render/busy/ms 64.57, stage/main/update 55.42, pipelines/compile_ms 53.60, pipelines/front_ms 51.09, stream/drive/ms 35.61, stage/render/prepare_assets 31.98, stage/render/render 15.42, render/graph/ms 13.43, stage/main/post_update 10.52, encode//sum 10.40 |
| warm-up | 5.14 | 53.3 | pipelines/compile_ms 1115, pipelines/front_ms 1087, main/busy/ms 62.15, render/thread/ms 47.45, stage/main/update 47.32, render/busy/ms 46.01, stream/drive/ms 35.68, pipelines/driver_ms 28.60, stage/render/render 15.30, render/graph/ms 13.27, stage/render/prepare_assets 11.82, encode//sum 10.66 |
| warm-up | 5.16 | 27.0 | pipelines/compile_ms 1403, pipelines/front_ms 1365, render/thread/ms 45.82, render/busy/ms 44.35, pipelines/driver_ms 38.11, main/busy/ms 27.78, stage/render/render 13.54, stage/main/update 12.21, render/graph/ms 11.63, stage/render/prepare_assets 11.19, encode//sum 10.39, stage/main/post_update 10.25 |
| warm-up | 5.21 | 27.7 | pipelines/compile_ms 58.29, pipelines/front_ms 55.04, render/thread/ms 43.23, render/busy/ms 41.92, main/busy/ms 27.82, stage/render/render 15.83, render/graph/ms 13.90, stage/main/update 11.44, stage/main/post_update 10.44, encode//sum 10.05, stage/render/graph_submit 9.84, stage/render/prepare_assets 8.89 |
| warm-up | 5.27 | 27.2 | render/thread/ms 47.05, render/busy/ms 44.87, main/busy/ms 44.13, stage/main/update 22.98, stage/render/render 21.76, render/graph/ms 19.09, stage/main/post_update 14.21, stage/render/graph_submit 12.84, encode//sum 12.76, encode/submit 9.20, stream/drive/ms 8.86, stage/render/prepare_assets 7.88 |
| warm-up | 5.32 | 56.7 | pipelines/compile_ms 482, pipelines/front_ms 466, render/thread/ms 64.35, render/busy/ms 62.34, main/busy/ms 42.14, stage/render/prepare_assets 30.20, stage/main/update 20.14, stage/render/render 19.56, render/graph/ms 17.03, pipelines/driver_ms 15.93, stage/main/post_update 14.85, encode//sum 12.44 |
| warm-up | 5.36 | 41.1 | pipelines/compile_ms 678, pipelines/front_ms 654, render/thread/ms 76.67, render/busy/ms 75.34, stage/render/prepare_assets 46.83, main/busy/ms 42.19, stage/main/update 24.20, pipelines/driver_ms 23.86, stream/drive/ms 15.31, stage/render/render 12.73, stage/main/post_update 12.15, render/graph/ms 10.73 |
| warm-up | 5.39 | 30.0 | pipelines/compile_ms 773, pipelines/front_ms 753, render/thread/ms 61.40, render/busy/ms 60.17, main/busy/ms 41.32, stage/render/prepare_assets 35.19, stage/main/update 23.02, pipelines/driver_ms 20.23, stream/drive/ms 14.10, stage/render/render 12.71, stage/main/post_update 11.97, render/graph/ms 10.82 |
| warm-up | 5.45 | 29.6 | pipelines/compile_ms 754, pipelines/front_ms 732, render/thread/ms 50.87, render/busy/ms 49.35, main/busy/ms 27.08, stage/render/prepare_assets 22.10, pipelines/driver_ms 21.92, stage/render/render 18.18, render/graph/ms 15.94, stage/main/post_update 12.49, stage/render/graph_submit 12.10, encode//sum 11.74 |
| warm-up | 5.48 | 28.5 | pipelines/compile_ms 232, pipelines/front_ms 220, render/thread/ms 51.91, render/busy/ms 50.47, main/busy/ms 28.21, stage/render/prepare_assets 21.76, stage/render/render 18.98, render/graph/ms 16.78, stage/main/post_update 14.64, stage/render/graph_submit 12.90, encode//sum 11.93, pipelines/driver_ms 11.32 |
| warm-up | 5.55 | 27.9 | pipelines/compile_ms 376, pipelines/front_ms 367, render/thread/ms 46.22, render/busy/ms 44.97, stage/render/prepare_assets 27.29, main/busy/ms 26.60, stage/main/post_update 12.83, stage/render/render 10.96, render/graph/ms 9.02, encode//sum 8.98, pipelines/driver_ms 8.48, stage/main/update 8.45 |
| warm-up | 5.72 | 30.1 | render/thread/ms 41.61, render/busy/ms 40.32, main/busy/ms 32.27, stage/main/update 16.73, stage/render/prepare_assets 14.99, stage/render/render 12.78, render/graph/ms 11.07, stage/main/post_update 10.86, encode//sum 9.00, stream/drive/ms 8.91, stage/render/graph_submit 7.52, encode/submit 5.32 |
| warm-up | 6.09 | 28.7 | pipelines/compile_ms 181, pipelines/front_ms 176, main/busy/ms 34.94, render/thread/ms 27.37, render/busy/ms 26.19, stage/main/update 21.79, stream/drive/ms 14.31, stage/render/render 12.81, encode//sum 11.67, render/graph/ms 11.07, stage/main/post_update 8.80, stage/render/graph_submit 7.65 |
| warm-up | 6.24 | 38.1 | pipelines/compile_ms 299, pipelines/front_ms 292, render/thread/ms 48.95, render/busy/ms 47.86, main/busy/ms 23.79, stage/render/prepare_assets 19.84, stage/render/render 10.74, encode//sum 9.35, render/graph/ms 9.15, stage/main/update 8.77, stage/main/post_update 8.19, pipelines/driver_ms 7.52 |
| warm-up | 6.48 | 30.0 | render/thread/ms 47.27, render/busy/ms 46.04, main/busy/ms 40.52, stage/main/post_update 16.87, stage/main/update 16.35, stage/render/render 15.38, render/graph/ms 13.83, encode//sum 12.62, stage/render/graph_submit 9.02, encode/pass:shadow_point_light/sum 7.80, stage/render/specialize 7.17, encode/submit 5.56 |
| warm-up | 6.51 | 29.3 | pipelines/compile_ms 150, pipelines/front_ms 147, render/thread/ms 50.04, render/busy/ms 48.90, main/busy/ms 40.21, stage/main/update 17.28, stage/main/post_update 15.52, stage/render/render 15.10, render/graph/ms 13.28, encode//sum 13.17, stage/render/prepare_assets 12.96, stage/render/graph_submit 8.57 |
| warm-up | 6.54 | 31.2 | pipelines/compile_ms 168, pipelines/front_ms 163, render/thread/ms 46.43, render/busy/ms 45.33, main/busy/ms 32.26, stage/render/prepare_assets 18.86, stage/main/update 14.70, stage/render/render 12.28, stage/main/post_update 11.20, render/graph/ms 10.56, encode//sum 9.82, stage/render/graph_submit 7.62 |
| warm-up | 6.57 | 30.5 | render/thread/ms 51.71, render/busy/ms 50.49, main/busy/ms 36.00, pipelines/compile_ms 17.78, pipelines/front_ms 16.70, stage/main/update 15.40, stage/render/render 14.47, stage/main/post_update 14.38, stage/render/prepare_assets 13.57, render/graph/ms 12.71, encode//sum 10.70, stage/render/graph_submit 8.59 |
| warm-up | 6.63 | 34.4 | render/thread/ms 47.42, render/busy/ms 46.44, main/busy/ms 35.64, stage/main/update 18.08, stage/render/prepare_assets 17.65, stage/main/post_update 12.61, stage/render/render 10.80, encode//sum 9.47, render/graph/ms 9.28, stage/render/graph_submit 5.89, encode/pass:shadow_point_light/sum 5.46, stage/render/specialize 4.56 |
| warm-up | 6.78 | 29.7 | render/thread/ms 48.68, render/busy/ms 47.50, main/busy/ms 41.58, stage/main/update 25.03, stream/drive/ms 16.98, stage/render/prepare_assets 16.84, stage/render/render 12.38, stage/main/post_update 12.31, pipelines/compile_ms 11.42, pipelines/front_ms 10.84, render/graph/ms 10.80, encode//sum 9.77 |
| warm-up | 6.81 | 37.7 | render/thread/ms 61.45, render/busy/ms 60.42, main/busy/ms 40.96, stage/render/prepare_assets 30.46, stage/main/update 24.87, stream/drive/ms 17.13, stage/render/render 11.48, stage/main/post_update 11.48, render/graph/ms 9.99, encode//sum 9.03, stage/render/graph_submit 6.66, stage/render/specialize 5.76 |
| warm-up | 7.00 | 30.0 | render/thread/ms 37.49, render/busy/ms 36.07, main/busy/ms 33.10, stage/main/update 18.64, stage/render/render 14.93, render/graph/ms 13.09, encode//sum 10.77, stage/main/post_update 9.93, stage/render/graph_submit 9.76, stage/render/prepare_assets 7.78, stream/drive/ms 7.66, encode/submit 6.72 |
| warm-up | 7.07 | 33.5 | pipelines/compile_ms 137, pipelines/front_ms 133, main/busy/ms 39.32, render/thread/ms 34.13, render/busy/ms 33.04, stage/main/post_update 19.71, stage/main/update 12.86, stage/render/prepare_assets 11.50, stage/render/render 11.21, render/graph/ms 9.65, encode//sum 9.41, stage/render/graph_submit 6.57 |
| measure | 18.18 | 29.8 | main/busy/ms 37.76, render/thread/ms 24.10, stage/main/update 23.94, render/busy/ms 23.00, stream/drive/ms 20.16, stage/render/render 14.77, render/graph/ms 13.19, encode//sum 12.33, stage/main/post_update 11.33, stage/render/graph_submit 9.44, encode/pass:shadow_point_light/sum 8.48, encode/submit 6.14 |
| measure | 23.15 | 27.6 | render/thread/ms 42.13, render/busy/ms 40.83, main/busy/ms 30.66, encode//sum 15.11, stage/render/render 14.85, stage/main/post_update 13.85, render/graph/ms 12.94, stage/main/update 12.59, stage/render/prepare_assets 9.61, stage/render/graph_submit 8.88, encode/pass:shadow_point_light/sum 8.74, encode/submit 5.37 |
| measure | 59.05 | 30.3 | render/thread/ms 39.79, render/busy/ms 38.59, main/busy/ms 28.69, stage/render/prepare_assets 15.26, stage/main/post_update 15.13, encode//sum 11.68, stage/render/render 11.52, render/graph/ms 10.04, stage/main/update 7.82, stage/render/graph_submit 6.69, encode/submit 3.87, stage/render/camera_driver 3.19 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=nocull-5 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 1%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **73%** (min 40%) · 2625 MHz (min 2625) · memory 8101 MHz · 75.3 W · 75 °C (max 82) · VRAM 7052 MiB · throttled 100% of samples

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
