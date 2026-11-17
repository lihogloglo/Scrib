# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5923 frames over 65.6 s · **median 10.47 ms (95 fps)** · mean 11.08 · p95 14.85 · p99 16.65 (1% low 60 fps) · worst 30.09 ms · 137 spikes over 1.5× median · 22.8 stutters/min (work 22.8/min)
- warm-up: 323 frames over 9.5 s, worst 2467.2 ms
- hitches (> 26.2 ms): **7** measured, 54 in warm-up
- timed GPU passes: 6.62 ms of a 11.08 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.47 ms, limit 11 ms
- **FAIL** frame_ms: 25 stutters (> 1.5x moving median of 60) in 1.09 min = 22.85/min, limit 1/min
- pass worst frame 30.09 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.978 | 2.033 | 2.065 | 0.97 | 0.174 |
| main_opaque_pass_3d | 1.270 | 1.305 | 1.666 | 0.97 | 0.062 |
| early prepass | 0.484 | 0.497 | 0.720 | 0.97 | 0.086 |
| bloom | 0.308 | 0.316 | 0.323 | 0.97 | 0.064 |
| atmosphere_luts | 0.302 | 0.310 | 0.372 | 0.97 | 0.003 |
| auxiliary_views | 0.258 | 0.265 | 1.111 | 0.97 | 0.244 |
| water_velocity | 0.255 | 0.263 | 0.350 | 0.97 | 0.002 |
| water_sspr | 0.196 | 0.202 | 0.203 | 0.97 | 0.002 |
| ssao | 0.183 | 0.188 | 0.193 | 0.97 | 0.007 |
| uniform_allocation | 0.141 | 0.145 | 0.175 | 0.97 | 0.025 |
| upscaling | 0.126 | 0.130 | 0.137 | 0.97 | 0.004 |
| water_fft | 0.125 | 0.129 | 0.137 | 0.97 | 0.003 |
| atmos_froxel_inject | 0.124 | 0.128 | 0.129 | 0.97 | 0.001 |
| clouds_composite | 0.102 | 0.105 | 0.109 | 0.97 | 0.002 |
| bin_unpacking | 0.099 | 0.102 | 0.125 | 0.97 | 0.018 |
| tonemapping | 0.085 | 0.087 | 0.091 | 0.97 | 0.003 |
| main_transmissive_pass_3d | 0.078 | 0.080 | 0.156 | 0.97 | 0.008 |
| clustering | 0.062 | 0.064 | 0.067 | 0.97 | 0.019 |
| clouds_trace | 0.054 | 0.056 | 0.073 | 0.97 | 0.001 |
| render_sky | 0.050 | 0.052 | 0.053 | 0.97 | 0.002 |
| clouds_resolve | 0.047 | 0.049 | 0.049 | 0.97 | 0.001 |
| lightprobe_irradiance_map | 0.042 | 0.413 | 0.416 | 0.10 | 0.001 |
| surface_trail | 0.039 | 0.056 | 0.057 | 0.69 | 0.002 |
| early_mesh_preprocessing | 0.036 | 0.037 | 0.046 | 0.97 | 0.004 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.97 | 0.001 |
| water_snapshot | 0.024 | 0.025 | 0.028 | 0.97 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.219 | 0.229 | 0.10 | 0.005 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.97 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.036 | 0.97 | 0.002 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.028 | 0.97 | 0.174 |
| clouds_fog_light | 0.012 | 0.012 | 0.013 | 0.97 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| per_view_shadow_pass | 0.009 | 0.010 | 0.015 | 0.97 | 0.004 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.97 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.97 | 0.003 |
| lightprobe_copy | 0.001 | 0.012 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.655 | 13.322 | 25.72 | 57189.3 |
| render/busy/ms | 9.141 | 12.781 | 25.04 | 54139.8 |
| main/busy/ms | 8.781 | 10.758 | 29.00 | 52008.3 |
| stage/render/render | 5.701 | 8.444 | 16.68 | 33765.9 |
| stage/main/post_update | 5.362 | 6.992 | 10.08 | 31761.6 |
| render/graph/ms | 4.971 | 7.637 | 15.55 | 29442.0 |
| encode//sum | 4.119 | 6.557 | 31.49 | 24397.1 |
| stage/render/graph_submit | 3.640 | 5.508 | 11.95 | 21560.9 |
| encode/submit | 2.326 | 3.701 | 6.28 | 13774.2 |
| stage/main/update | 2.156 | 2.588 | 22.33 | 12767.1 |
| encode/finish_wall | 1.356 | 1.906 | 5.69 | 8030.4 |
| encode/pass:shadow_point_light/sum | 1.314 | 5.269 | 16.89 | 7783.9 |
| stage/render/camera_driver | 1.276 | 2.109 | 4.01 | 7558.3 |
| encode/pass:shadow_directional_light/sum | 0.764 | 1.261 | 1.88 | 4523.9 |
| stage/render/prepare_write_phase_buffers | 0.736 | 1.123 | 2.41 | 4358.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.732 | 1.055 | 1.87 | 4334.7 |
| encode/pass:early prepass/sum | 0.727 | 1.055 | 1.56 | 4308.4 |
| stage/render/prepare_assets | 0.576 | 0.728 | 9.21 | 3411.0 |
| bigspace/total/ms | 0.514 | 1.182 | 3.38 | 3042.9 |
| stage/render/prepare_meshes | 0.481 | 0.631 | 1.89 | 2847.6 |
| stage/render/prepare_resources | 0.447 | 0.529 | 1.48 | 2647.5 |
| render/present/ms | 0.420 | 0.516 | 4.81 | 2488.4 |
| stage/main/first | 0.393 | 0.516 | 4.92 | 2329.8 |
| stage/main/pre_update | 0.354 | 0.495 | 1.54 | 2095.1 |
| terrain/update/ms | 0.308 | 0.392 | 2.00 | 1822.6 |
| stage/render/specialize | 0.295 | 0.457 | 4.05 | 1745.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.292 | 0.435 | 0.98 | 1732.2 |
| terrain/select/ms | 0.274 | 0.350 | 1.13 | 1625.8 |
| stage/render/prepare_bind_groups | 0.268 | 0.479 | 2.42 | 1588.5 |
| stage/render/queue | 0.268 | 0.432 | 3.66 | 1588.5 |
| stage/render/prepare_batch_phases | 0.268 | 0.379 | 0.84 | 1586.3 |
| controller/feed/ms | 0.256 | 0.325 | 0.92 | 1516.6 |
| stream/drive/ms | 0.243 | 0.321 | 20.41 | 1437.2 |
| stage/main/fixed_main_loop | 0.219 | 0.368 | 0.90 | 1295.1 |
| bigspace/low_precision/ms | 0.211 | 0.584 | 1.78 | 1249.6 |
| far/select/ms | 0.199 | 0.265 | 0.53 | 1180.6 |
| stage/render/create_views | 0.195 | 0.252 | 0.83 | 1153.0 |
| stream/tick/ms | 0.187 | 0.246 | 0.72 | 1106.0 |
| stage/render/prepare_flush | 0.172 | 0.321 | 0.90 | 1017.0 |
| stage/render/extract_commands | 0.168 | 0.196 | 0.65 | 995.8 |
| anim/evaluate/ms | 0.162 | 0.289 | 2.05 | 960.3 |
| anim/evaluate_cpu/ms | 0.154 | 0.190 | 0.83 | 912.6 |
| bigspace/high_precision/ms | 0.139 | 0.541 | 2.86 | 824.0 |
| stage/render/prepare_views | 0.138 | 0.170 | 0.36 | 814.7 |
| controller/move/ms | 0.124 | 0.207 | 0.64 | 736.0 |
| stage/main/last | 0.118 | 0.166 | 1.76 | 700.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.096 | 0.154 | 0.88 | 567.3 |
| render/acquire/ms | 0.095 | 0.119 | 0.33 | 561.2 |
| stage/render/cleanup | 0.093 | 0.228 | 0.86 | 550.4 |
| stage/render/phase_sort | 0.064 | 0.090 | 0.59 | 377.4 |
| stage/render/prepare_collect_phase_buffers | 0.060 | 0.130 | 0.39 | 357.3 |
| physics/step/ms | 0.047 | 0.086 | 0.58 | 279.3 |
| physics/insert/ms | 0.034 | 0.374 | 0.87 | 203.2 |
| encode/resolve_diagnostics/sum | 0.033 | 0.055 | 0.53 | 198.4 |
| weather/map/ms | 0.016 | 0.021 | 0.13 | 97.3 |
| pipelines/compile_ms | 0.016 | 0.000 | 83.35 | 96.2 |
| pipelines/front_ms | 0.016 | 0.000 | 80.37 | 93.0 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.022 | 0.45 | 84.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.32 | 64.2 |
| anim/apply/ms | 0.011 | 0.021 | 0.38 | 64.0 |
| controller/avoid/ms | 0.011 | 0.014 | 0.27 | 63.2 |
| weather/eval/ms | 0.010 | 0.012 | 0.18 | 57.1 |
| script/update/ms | 0.007 | 0.008 | 0.20 | 39.3 |
| stream/despawn/ms | 0.005 | 0.000 | 1.44 | 30.3 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 29.9 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 27.8 |
| script/ui/ms | 0.005 | 0.006 | 0.08 | 27.5 |
| controller/write/ms | 0.004 | 0.005 | 0.04 | 21.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.57 | 14.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.13 | 3.7 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.97 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.01 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 31 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1277 | 2262 | 652 |
| crowd/c0/meshes/count | count | 353 | 646 | 162 |
| crowd/c1/count | count | 146 | 157 | 147 |
| crowd/c1/joints/count | count | 5539 | 5999 | 5544 |
| crowd/c1/meshes/count | count | 1181 | 1297 | 1179 |
| crowd/c2/count | count | 399 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 165 | 216 | 205 |
| crowd/joints_extracted/count | count | 3712 | 7863 | 5763 |
| crowd/joints_uploaded/count | count | 3657 | 6220 | 5763 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.41 | 1.49 | 0.67 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.45 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.32 | 0.01 |
| encode/finish_critical | ms | 0.93 | 1.88 | 1.14 |
| encode/pass:early prepass/max | ms | 0.76 | 1.56 | 1.00 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.75 | 1.87 | 0.67 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.88 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.28 | 0.98 | 0.23 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 1.88 | 1.14 |
| encode/pass:shadow_point_light/max | ms | 0.78 | 1.27 | 0.81 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.53 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 8.95 | 29.00 | 8.22 |
| gpu/timed/ms | ms | 7.51 | 10.02 | 7.85 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 255 | 479 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3948 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 255 | 479 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3948 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 4114 | 4335 | 4335 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 4114 | 4335 | 4335 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 17 | 213 | 20 |
| phases/shadow/bins | count | 10.4 k | 11.9 k | 7980 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 27.1 k | 29.2 k | 25.7 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 205 | 228 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 16 | 592 | 37 |
| preprocess/bind_groups_created | count | 16 | 592 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 135 | 284 | 129 |
| process/cpu_usage | % | 25.48 | 42.38 | 24.04 |
| process/mem_usage | GiB | 5.44 | 5.60 | 5.60 |
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
| skin/blocks_changed/count | count | 13 | 95 | 10 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 165 | 216 | 205 |
| skin/joint_bytes_written/bytes | bytes | 281.1 k | 812.7 k | 445.7 k |
| skin/joints_written/count | count | 4392 | 12.7 k | 6964 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4919 | 5010 | 5010 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 431 | 528 | 396 |
| statics/materials_bound | count | 431 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.45 | 40.57 | 28.60 |
| system/mem_usage | % | 54.27 | 54.87 | 54.83 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 69 | 264 | 72 |
| views/all/bins | count | 19.0 k | 20.5 k | 17.6 k |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 55.1 k | 57.6 k | 55.2 k |
| views/all/pipelines | count | 251 | 277 | 68 |
| views/all/pipelines_distinct | count | 71 | 76 | 63 |
| views/all/work_items | count | 41.5 k | 56.1 k | 56.0 k |
| views/cascade/batch_sets | count | 10 | 28 | 20 |
| views/cascade/bins | count | 7663 | 7994 | 7980 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 24.5 k | 25.7 k | 25.7 k |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 11.7 k | 25.7 k | 25.7 k |
| views/lamp_face/batch_sets | count | 0 | 189 | 0 |
| views/lamp_face/bins | count | 3714 | 4537 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3994 | 4858 | 0 |
| views/lamp_face/pipelines | count | 180 | 201 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 3996 | 0 |
| views/main/batch_sets | count | 51 | 55 | 52 |
| views/main/bins | count | 8722 | 9612 | 9612 |
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
| anim/bones_written/count | count | 42.19 | 295 | 249.9 k |
| anim/events/count | count | 0.09 | 3 | 512 |
| assets/Image/added | count | 0.10 | 33 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.69 | 152 | 4061 |
| assets/Mesh/removed | count | 0.82 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.66 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.79 | 1309 | 4676 |
| encode//count | count | 37.94 | 221 | 224.7 k |
| encode/bind_walk/init_textures | count | 15.0 k | 115.2 k | 88.80 M |
| encode/bind_walk/init_textures_skipped | count | 56.3 k | 123.4 k | 333.68 M |
| encode/bind_walk/inits | count | 531 | 1258 | 3.15 M |
| encode/bind_walk/inits_skipped | count | 126 | 193 | 745.2 k |
| encode/bind_walk/merge_textures | count | 14.9 k | 115.1 k | 88.42 M |
| encode/bind_walk/merge_textures_skipped | count | 5531 | 6026 | 32.76 M |
| encode/bind_walk/merges | count | 107 | 349 | 634.1 k |
| encode/bind_walk/merges_skipped | count | 105 | 151 | 623.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5923 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5923 |
| encode/pass:early prepass/count | count | 1 | 1 | 5923 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5923 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5923 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5923 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5923 |
| encode/pass:shadow_point_light/count | count | 2.74 | 48 | 16.3 k |
| encode/queue/command_buffers | count | 50.70 | 279 | 300.3 k |
| encode/queue/submits | count | 2.02 | 4 | 11.9 k |
| encode/queue/write_buffer/bytes | bytes | 2.25 M | 4.35 M | 13.33 G |
| encode/queue/write_buffer/calls | count | 270 | 884 | 1.60 M |
| encode/queue/write_buffer_with/bytes | bytes | 170.7 k | 3.53 M | 1.01 G |
| encode/queue/write_buffer_with/calls | count | 11.04 | 465 | 65.4 k |
| encode/queue/write_texture/bytes | bytes | 10.6 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5923 |
| physics/inserts | count | 0.38 | 111 | 2267 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
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
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5715 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1253 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.2 ms; slowest 61, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2467.2 | main/busy/ms 2432, stage/main/post_update 2392, stage/main/update 31.29, stage/main/pre_update 4.60, stage/main/last 1.47, surface/advance_cpu_ms 0.73, stage/main/first 0.50, stream/drive/ms 0.46, surface/cell_upload_ms 0.41, stage/main/fixed_main_loop 0.38, stream/tick/ms 0.11, bigspace/total/ms 0.07 |
| warm-up | 3.23 | 758.6 | main/busy/ms 2569, stage/main/post_update 2417, render/thread/ms 725, render/busy/ms 556, stage/render/prepare_assets 505, stage/render/prepare_views 163, render/acquire/ms 163, stage/main/update 137, stage/render/create_views 29.80, stage/render/render 16.14, render/graph/ms 9.24, stage/render/camera_driver 7.16 |
| warm-up | 3.43 | 199.4 | render/thread/ms 907, render/busy/ms 628, stage/render/prepare_assets 560, main/busy/ms 170, stage/render/prepare_views 163, render/acquire/ms 163, pipelines/compile_ms 140, stage/render/render 132, render/present/ms 116, stage/main/update 114, pipelines/driver_ms 97.85, pipelines/front_ms 42.32 |
| warm-up | 3.65 | 209.5 | pipelines/wait_ms 4203, pipelines/compile_ms 2647, pipelines/front_ms 2147, pipelines/driver_ms 500, encode//sum 359, render/thread/ms 156, render/busy/ms 152, main/busy/ms 94.04, stage/render/prepare_assets 85.53, stage/main/update 61.52, stage/render/render 57.83, render/graph/ms 54.04 |
| warm-up | 3.69 | 38.6 | pipelines/wait_ms 4317, pipelines/compile_ms 2544, pipelines/front_ms 2195, encode//sum 373, pipelines/driver_ms 349, render/thread/ms 179, render/busy/ms 175, stage/render/prepare_assets 92.83, stage/render/render 72.97, render/graph/ms 68.68, stage/render/graph_submit 65.64, encode/finish_wall 49.06 |
| warm-up | 3.73 | 45.1 | pipelines/compile_ms 938, pipelines/front_ms 886, pipelines/wait_ms 242, render/thread/ms 78.77, render/busy/ms 75.44, pipelines/driver_ms 51.85, stage/render/prepare_assets 38.89, main/busy/ms 34.42, stage/render/render 29.69, render/graph/ms 26.00, stage/render/graph_submit 23.43, encode//sum 17.47 |
| warm-up | 3.83 | 27.7 | pipelines/compile_ms 782, pipelines/front_ms 744, render/thread/ms 46.32, render/busy/ms 44.41, pipelines/driver_ms 37.97, main/busy/ms 32.35, stage/render/prepare_assets 17.12, stage/main/post_update 15.32, stage/render/render 11.91, encode//sum 11.36, terrain/update/ms 9.57, stage/main/update 9.17 |
| warm-up | 3.91 | 59.5 | pipelines/compile_ms 1135, pipelines/front_ms 1100, pipelines/wait_ms 105, main/busy/ms 46.30, render/thread/ms 44.78, render/busy/ms 42.54, pipelines/driver_ms 35.26, stage/main/update 34.70, stream/drive/ms 30.38, stage/render/prepare_assets 23.95, stage/render/render 13.50, render/graph/ms 10.07 |
| warm-up | 4.00 | 67.6 | pipelines/compile_ms 488, pipelines/front_ms 471, render/thread/ms 61.49, render/busy/ms 59.75, main/busy/ms 54.07, stage/render/prepare_assets 41.06, stage/main/update 40.03, stream/drive/ms 35.02, pipelines/driver_ms 17.08, stage/render/render 12.37, render/graph/ms 10.01, stage/main/post_update 8.16 |
| warm-up | 4.08 | 77.8 | pipelines/compile_ms 59.57, render/thread/ms 57.58, pipelines/front_ms 56.34, render/busy/ms 55.82, stage/render/prepare_assets 33.06, main/busy/ms 32.34, stage/render/render 15.73, stage/main/update 14.89, render/graph/ms 13.42, stage/render/graph_submit 11.21, stage/main/post_update 11.10, stream/drive/ms 10.17 |
| warm-up | 4.13 | 48.2 | pipelines/compile_ms 966, pipelines/front_ms 938, render/thread/ms 77.83, render/busy/ms 76.00, stage/render/prepare_assets 45.93, pipelines/driver_ms 28.65, main/busy/ms 25.73, stage/render/render 21.71, render/graph/ms 19.28, stage/render/graph_submit 16.79, encode/finish_wall 9.91, stage/main/post_update 9.36 |
| warm-up | 4.24 | 107.2 | pipelines/compile_ms 1252, pipelines/front_ms 1184, render/thread/ms 110, render/busy/ms 108, pipelines/driver_ms 68.38, stage/render/prepare_assets 51.26, main/busy/ms 50.89, stage/render/render 49.48, render/graph/ms 47.51, stage/render/camera_driver 37.10, stage/main/update 34.80, stage/render/graph_submit 10.18 |
| warm-up | 4.29 | 51.0 | pipelines/compile_ms 954, pipelines/front_ms 875, render/thread/ms 126, render/busy/ms 124, pipelines/driver_ms 78.47, main/busy/ms 70.63, stage/render/prepare_assets 69.58, stage/main/update 50.09, stage/render/render 45.90, render/graph/ms 43.64, stage/render/camera_driver 38.34, stage/main/post_update 11.75 |
| warm-up | 4.33 | 38.5 | pipelines/compile_ms 1086, pipelines/front_ms 1032, render/thread/ms 70.58, render/busy/ms 68.37, pipelines/driver_ms 54.45, main/busy/ms 49.35, stage/render/prepare_assets 43.39, stage/main/update 25.78, stage/render/render 12.90, stage/main/post_update 12.75, render/graph/ms 10.33, stage/render/graph_submit 6.74 |
| warm-up | 4.38 | 50.7 | pipelines/compile_ms 630, pipelines/front_ms 602, pipelines/wait_ms 70.51, render/thread/ms 64.29, render/busy/ms 62.23, main/busy/ms 42.76, pipelines/driver_ms 27.47, stage/render/prepare_assets 26.34, stage/main/post_update 17.88, stage/render/render 15.43, render/graph/ms 12.99, stage/main/update 12.33 |
| warm-up | 4.43 | 52.5 | pipelines/compile_ms 1051, pipelines/front_ms 1005, pipelines/wait_ms 179, render/thread/ms 88.15, render/busy/ms 85.95, main/busy/ms 49.00, pipelines/driver_ms 45.41, encode//sum 45.22, stage/render/prepare_assets 33.06, stage/main/post_update 26.90, stage/render/render 25.76, render/graph/ms 23.13 |
| warm-up | 4.55 | 114.2 | pipelines/compile_ms 2310, pipelines/front_ms 2229, pipelines/wait_ms 477, render/thread/ms 159, render/busy/ms 157, pipelines/driver_ms 81.01, encode//sum 72.56, main/busy/ms 47.11, stage/render/render 45.48, render/graph/ms 42.73, stage/render/prepare_write_phase_buffers 33.47, stage/render/prepare_resources 26.05 |
| warm-up | 4.59 | 43.2 | pipelines/compile_ms 2128, pipelines/front_ms 2052, pipelines/wait_ms 829, render/thread/ms 147, render/busy/ms 146, main/busy/ms 125, stage/main/update 99.80, pipelines/driver_ms 76.43, stream/drive/ms 57.09, encode//sum 42.16, stage/render/render 40.47, render/graph/ms 38.12 |
| warm-up | 4.75 | 159.0 | pipelines/compile_ms 2248, pipelines/front_ms 2166, pipelines/wait_ms 806, main/busy/ms 124, render/thread/ms 114, render/busy/ms 113, stage/main/update 100, pipelines/driver_ms 81.97, stream/drive/ms 58.23, stage/render/prepare_assets 42.71, stage/render/render 30.62, render/graph/ms 28.30 |
| warm-up | 4.79 | 38.1 | pipelines/compile_ms 2028, pipelines/front_ms 1962, pipelines/wait_ms 344, render/thread/ms 108, render/busy/ms 106, main/busy/ms 85.10, pipelines/driver_ms 66.32, stage/main/update 63.17, stream/drive/ms 49.59, stage/render/prepare_assets 44.49, stage/render/render 32.55, render/graph/ms 29.95 |
| warm-up | 4.90 | 112.4 | pipelines/compile_ms 791, pipelines/front_ms 761, main/busy/ms 88.27, render/thread/ms 80.15, render/busy/ms 78.34, stage/main/update 62.91, stream/drive/ms 48.60, stage/render/prepare_assets 47.86, pipelines/driver_ms 29.66, stage/render/render 18.63, render/graph/ms 16.23, stage/main/post_update 15.70 |
| warm-up | 4.93 | 27.8 | pipelines/compile_ms 863, pipelines/front_ms 835, main/busy/ms 93.12, render/thread/ms 73.14, render/busy/ms 71.68, stage/main/update 64.84, stage/render/prepare_assets 49.31, stream/drive/ms 44.45, pipelines/driver_ms 28.42, stage/main/post_update 19.41, stage/render/render 11.70, render/graph/ms 9.69 |
| warm-up | 5.02 | 96.0 | pipelines/compile_ms 1519, pipelines/front_ms 1473, main/busy/ms 85.93, stage/main/update 65.64, render/thread/ms 61.01, render/busy/ms 59.52, pipelines/driver_ms 45.42, stream/drive/ms 43.95, stage/render/prepare_assets 33.88, stage/render/render 14.90, stage/main/post_update 13.07, render/graph/ms 12.92 |
| warm-up | 5.06 | 32.7 | pipelines/compile_ms 1063, pipelines/front_ms 1028, main/busy/ms 65.13, render/thread/ms 63.48, render/busy/ms 62.12, stage/main/update 49.97, pipelines/driver_ms 35.08, stream/drive/ms 32.17, stage/render/prepare_assets 30.48, stage/render/render 16.20, render/graph/ms 14.31, encode//sum 10.20 |
| warm-up | 5.11 | 49.5 | pipelines/compile_ms 632, pipelines/front_ms 613, main/busy/ms 59.60, render/thread/ms 50.22, render/busy/ms 48.96, stage/main/update 44.89, stream/drive/ms 32.98, pipelines/driver_ms 18.60, stage/render/render 15.02, render/graph/ms 13.19, stage/render/prepare_assets 13.15, encode//sum 11.25 |
| warm-up | 5.13 | 28.7 | pipelines/compile_ms 542, pipelines/front_ms 528, render/thread/ms 51.26, render/busy/ms 49.90, main/busy/ms 29.32, stage/render/render 18.45, render/graph/ms 16.55, pipelines/driver_ms 14.60, encode//sum 13.08, stage/main/update 12.41, stage/render/graph_submit 11.79, stage/main/post_update 11.35 |
| warm-up | 5.19 | 30.8 | pipelines/compile_ms 312, pipelines/front_ms 302, render/thread/ms 42.31, render/busy/ms 41.03, main/busy/ms 29.61, stage/main/update 13.54, stage/render/render 13.48, stage/render/prepare_assets 12.17, render/graph/ms 11.50, stage/main/post_update 10.76, encode//sum 10.47, pipelines/driver_ms 10.20 |
| warm-up | 5.24 | 31.8 | pipelines/compile_ms 763, pipelines/front_ms 745, render/thread/ms 49.79, render/busy/ms 47.91, main/busy/ms 36.78, pipelines/driver_ms 18.20, stage/render/render 17.08, stage/main/update 15.84, render/graph/ms 14.45, stage/render/prepare_assets 13.13, stage/main/post_update 12.54, encode//sum 11.23 |
| warm-up | 5.29 | 45.2 | pipelines/compile_ms 173, pipelines/front_ms 167, render/thread/ms 61.86, render/busy/ms 60.34, main/busy/ms 36.86, stage/render/prepare_assets 31.14, stage/main/update 15.99, stage/render/render 13.21, stage/main/post_update 12.62, render/graph/ms 10.89, encode//sum 10.76, stage/render/graph_submit 6.78 |
| warm-up | 5.35 | 34.5 | render/thread/ms 54.96, render/busy/ms 53.49, main/busy/ms 39.65, stage/main/post_update 19.29, stage/render/render 18.72, render/graph/ms 16.82, stage/main/update 14.30, encode//sum 11.41, stage/render/graph_submit 10.89, stage/render/prepare_assets 9.33, encode/submit 7.10, stage/render/prepare_resources 7.05 |
| warm-up | 5.38 | 33.9 | pipelines/compile_ms 200, pipelines/front_ms 194, render/thread/ms 63.75, render/busy/ms 62.20, main/busy/ms 41.67, stage/main/post_update 22.90, stage/render/render 18.08, render/graph/ms 15.93, stage/render/prepare_resources 12.88, stage/main/update 12.34, stage/render/prepare_assets 11.27, encode//sum 10.93 |
| warm-up | 5.44 | 55.0 | pipelines/compile_ms 1662, pipelines/front_ms 1614, render/thread/ms 75.91, render/busy/ms 74.29, pipelines/driver_ms 47.36, main/busy/ms 34.82, stage/render/prepare_assets 25.48, stage/render/render 24.27, render/graph/ms 21.88, stage/render/graph_submit 17.75, stage/main/post_update 15.93, stage/main/update 11.88 |
| warm-up | 5.47 | 35.8 | pipelines/compile_ms 1783, pipelines/front_ms 1727, render/thread/ms 76.05, render/busy/ms 74.34, pipelines/driver_ms 56.07, main/busy/ms 36.24, stage/render/prepare_assets 33.19, stage/render/render 24.69, render/graph/ms 22.27, stage/render/graph_submit 18.08, stage/main/update 16.26, pipelines/wait_ms 12.90 |
| warm-up | 5.54 | 37.3 | render/thread/ms 56.42, render/busy/ms 55.10, main/busy/ms 48.91, pipelines/compile_ms 43.03, pipelines/front_ms 40.07, stage/render/prepare_assets 24.88, stage/main/update 21.15, stage/main/post_update 17.73, stage/render/render 16.51, render/graph/ms 14.35, stream/drive/ms 13.51, encode//sum 10.97 |
| warm-up | 5.56 | 27.7 | render/thread/ms 56.92, render/busy/ms 55.72, main/busy/ms 45.44, stage/render/prepare_assets 29.17, stage/main/update 21.91, stage/main/post_update 16.74, stage/render/render 14.48, stream/drive/ms 13.82, render/graph/ms 12.48, encode//sum 10.61, stage/render/graph_submit 7.41, encode/pass:shadow_point_light/sum 5.00 |
| warm-up | 5.60 | 32.9 | render/thread/ms 52.94, render/busy/ms 51.60, main/busy/ms 33.81, stage/render/prepare_assets 21.44, stage/render/render 15.79, stage/main/post_update 13.88, render/graph/ms 13.70, stage/main/update 13.46, encode//sum 10.87, stage/render/graph_submit 9.12, encode/submit 5.92, encode/pass:shadow_point_light/sum 4.49 |
| warm-up | 5.63 | 35.4 | pipelines/compile_ms 1203, pipelines/front_ms 1180, render/thread/ms 61.82, render/busy/ms 60.26, main/busy/ms 32.26, stage/render/prepare_assets 26.21, pipelines/driver_ms 22.74, stage/render/render 17.86, render/graph/ms 15.58, stage/main/post_update 13.96, stage/main/update 12.51, encode//sum 11.12 |
| warm-up | 5.67 | 42.0 | pipelines/compile_ms 1241, pipelines/front_ms 1215, render/thread/ms 68.23, render/busy/ms 66.29, stage/render/prepare_assets 35.20, main/busy/ms 26.11, pipelines/driver_ms 25.60, stage/render/render 22.61, render/graph/ms 19.95, stage/render/graph_submit 13.65, stage/main/post_update 11.89, encode//sum 10.04 |
| warm-up | 5.71 | 31.2 | pipelines/compile_ms 214, pipelines/front_ms 201, render/thread/ms 64.01, render/busy/ms 62.11, stage/render/prepare_assets 30.73, main/busy/ms 25.78, stage/render/render 23.13, render/graph/ms 20.33, pipelines/driver_ms 13.91, stage/render/graph_submit 13.80, stage/main/post_update 11.91, encode//sum 10.52 |
| warm-up | 6.14 | 28.2 | pipelines/compile_ms 130, pipelines/front_ms 125, main/busy/ms 33.55, render/thread/ms 21.74, render/busy/ms 20.71, stage/main/update 20.58, stream/drive/ms 14.67, stage/render/render 10.38, stage/main/post_update 9.10, render/graph/ms 8.77, encode//sum 7.70, stage/render/graph_submit 5.84 |
| warm-up | 6.26 | 28.1 | render/thread/ms 42.15, render/busy/ms 40.54, main/busy/ms 30.18, stage/render/render 17.90, render/graph/ms 15.54, stage/main/post_update 12.19, stage/render/graph_submit 11.14, encode//sum 10.92, stage/main/update 10.58, encode/submit 8.44, stage/render/prepare_assets 7.84, encode/pass:shadow_point_light/sum 6.30 |
| warm-up | 6.29 | 32.8 | render/thread/ms 46.25, render/busy/ms 45.29, main/busy/ms 24.48, stage/render/prepare_assets 15.60, stage/render/render 10.08, encode//sum 9.52, stage/main/update 9.07, render/graph/ms 8.72, stage/main/post_update 7.92, stage/render/specialize 6.82, stage/render/graph_submit 5.88, encode/pass:shadow_point_light/sum 4.34 |
| warm-up | 6.41 | 35.1 | render/thread/ms 52.14, render/busy/ms 50.86, main/busy/ms 27.66, stage/render/prepare_assets 17.92, stage/main/update 12.20, stage/render/render 11.40, stage/main/post_update 11.16, render/graph/ms 9.78, encode//sum 8.17, stage/render/specialize 7.35, stage/render/graph_submit 6.32, encode/pass:shadow_point_light/sum 4.57 |
| warm-up | 6.49 | 27.7 | render/thread/ms 41.85, render/busy/ms 40.74, main/busy/ms 30.74, stage/main/update 14.14, stage/render/render 12.92, stage/main/post_update 11.59, render/graph/ms 11.40, encode//sum 8.91, stage/render/graph_submit 8.05, stage/render/specialize 7.56, stage/render/prepare_assets 6.35, encode/submit 5.66 |
| warm-up | 6.53 | 32.2 | pipelines/compile_ms 134, pipelines/front_ms 129, render/thread/ms 54.48, render/busy/ms 52.55, main/busy/ms 36.25, stage/render/render 19.40, render/graph/ms 16.67, stage/main/update 14.64, stage/main/post_update 14.40, encode//sum 12.49, stage/render/specialize 12.04, stage/render/graph_submit 11.75 |
| warm-up | 6.59 | 34.1 | render/thread/ms 44.16, render/busy/ms 43.12, main/busy/ms 30.15, stage/render/prepare_assets 18.65, pipelines/compile_ms 16.20, pipelines/front_ms 15.45, stage/main/update 14.21, stage/render/render 10.51, stage/main/post_update 10.34, render/graph/ms 8.85, encode//sum 8.46, stage/render/graph_submit 5.86 |
| warm-up | 6.62 | 30.4 | render/thread/ms 52.20, render/busy/ms 50.59, main/busy/ms 38.47, stage/render/render 17.08, stage/main/update 16.88, stage/main/post_update 15.70, stage/render/prepare_assets 15.20, render/graph/ms 15.14, encode//sum 12.42, stage/render/graph_submit 10.61, encode/pass:shadow_point_light/sum 8.10, encode/submit 7.15 |
| warm-up | 6.64 | 28.0 | render/thread/ms 51.19, render/busy/ms 49.36, main/busy/ms 49.32, stage/main/update 28.43, stage/render/render 21.63, render/graph/ms 19.32, encode//sum 16.06, stage/main/post_update 15.66, stream/drive/ms 14.88, stage/render/graph_submit 13.33, encode/pass:shadow_point_light/sum 9.93, encode/submit 8.90 |
| warm-up | 6.68 | 35.3 | render/thread/ms 48.52, render/busy/ms 46.99, main/busy/ms 45.12, stage/main/update 28.22, stage/render/render 17.28, render/graph/ms 14.96, stage/render/prepare_assets 14.33, stream/drive/ms 14.27, encode//sum 13.16, stage/main/post_update 11.82, stage/render/graph_submit 10.27, encode/pass:shadow_point_light/sum 7.92 |
| warm-up | 6.73 | 27.0 | pipelines/compile_ms 149, pipelines/front_ms 143, render/thread/ms 40.77, render/busy/ms 39.68, main/busy/ms 33.44, stage/main/update 15.12, stage/main/post_update 11.96, stage/render/render 11.57, stage/render/prepare_assets 10.99, render/graph/ms 10.02, encode//sum 8.87, stage/render/graph_submit 6.90 |
| warm-up | 6.88 | 36.4 | pipelines/compile_ms 144, pipelines/front_ms 141, render/thread/ms 52.70, render/busy/ms 51.56, main/busy/ms 33.67, stage/render/prepare_assets 26.90, stage/main/update 15.65, stage/render/render 13.09, stage/main/post_update 12.55, render/graph/ms 11.52, encode//sum 9.43, stage/render/graph_submit 8.32 |
| warm-up | 6.94 | 34.0 | render/thread/ms 52.30, render/busy/ms 51.19, pipelines/compile_ms 25.75, pipelines/front_ms 24.76, main/busy/ms 23.21, stage/render/prepare_assets 22.21, stage/render/render 11.85, stage/main/post_update 10.54, render/graph/ms 10.01, encode//sum 9.18, stage/main/update 8.21, stage/render/graph_submit 6.87 |
| warm-up | 7.11 | 27.1 | pipelines/compile_ms 144, pipelines/front_ms 140, render/thread/ms 34.50, render/busy/ms 33.31, main/busy/ms 32.93, stage/main/post_update 12.81, stage/render/prepare_assets 12.49, stage/main/update 12.05, stage/render/render 11.65, render/graph/ms 9.78, encode//sum 9.12, stage/render/graph_submit 6.47 |
| warm-up | 7.15 | 30.4 | main/busy/ms 40.08, render/thread/ms 33.18, render/busy/ms 32.00, stage/main/update 22.58, stream/drive/ms 12.52, stage/main/post_update 12.43, stage/render/render 11.44, render/graph/ms 9.80, encode//sum 8.75, stage/render/graph_submit 6.80, encode/pass:shadow_point_light/sum 5.48, stage/render/specialize 5.14 |
| measure | 0.76 | 27.8 | main/busy/ms 32.15, render/thread/ms 28.30, render/busy/ms 27.20, stage/main/update 18.04, stage/render/render 14.67, stream/drive/ms 13.45, render/graph/ms 12.86, encode//sum 11.98, stage/main/post_update 10.91, stage/render/graph_submit 9.23, encode/pass:shadow_point_light/sum 8.30, encode/submit 5.88 |
| measure | 2.18 | 27.2 | encode//sum 37.12, render/thread/ms 37.03, render/busy/ms 35.78, stage/render/render 24.24, render/graph/ms 22.37, encode/pass:shadow_point_light/sum 21.50, main/busy/ms 18.19, stage/render/graph_submit 16.69, pipelines/compile_ms 12.86, pipelines/front_ms 12.64, stage/main/post_update 10.71, encode/submit 9.54 |
| measure | 2.72 | 26.2 | render/thread/ms 36.65, render/busy/ms 35.35, encode//sum 27.84, stage/render/render 22.91, render/graph/ms 21.00, main/busy/ms 20.73, encode/pass:shadow_point_light/sum 19.93, stage/render/graph_submit 15.17, stage/main/post_update 12.21, encode/submit 9.29, encode/finish_wall 5.91, stage/render/camera_driver 5.71 |
| measure | 18.18 | 30.1 | main/busy/ms 39.33, stage/main/update 24.68, render/thread/ms 23.06, render/busy/ms 21.98, stream/drive/ms 20.69, stage/render/render 12.15, stage/main/post_update 11.57, render/graph/ms 10.50, encode//sum 9.96, stage/render/graph_submit 7.45, encode/submit 4.84, encode/pass:shadow_point_light/sum 4.31 |
| measure | 23.08 | 28.5 | render/thread/ms 40.98, render/busy/ms 39.83, main/busy/ms 30.35, stage/main/post_update 14.73, stage/render/render 13.99, encode//sum 12.45, render/graph/ms 12.21, stage/main/update 11.65, stage/render/prepare_assets 9.02, stage/render/graph_submit 8.04, encode/pass:shadow_point_light/sum 6.51, stage/render/queue 5.21 |
| measure | 59.00 | 27.4 | render/thread/ms 36.23, render/busy/ms 34.50, main/busy/ms 33.08, stage/main/post_update 19.12, stage/render/render 16.98, render/graph/ms 14.49, encode//sum 10.84, stage/render/graph_submit 10.33, stage/main/update 8.99, stage/render/prepare_assets 6.82, encode/submit 6.67, bigspace/total/ms 4.08 |
| measure | 59.03 | 27.2 | render/thread/ms 37.75, render/busy/ms 36.60, main/busy/ms 27.45, stage/render/prepare_assets 15.26, stage/main/post_update 14.71, stage/render/render 10.22, render/graph/ms 8.58, stage/main/update 7.15, encode//sum 7.04, stage/render/graph_submit 6.07, bigspace/total/ms 3.93, encode/submit 3.68 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=nocull-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 10%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **72%** (min 54%) · 2620 MHz (min 2610) · memory 8101 MHz · 75.4 W · 76 °C (max 81) · VRAM 7016 MiB · throttled 100% of samples

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
