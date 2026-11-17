# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.depth_only.skip_material=on (command line: --set); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6003 frames over 65.6 s · **median 10.69 ms (94 fps)** · mean 10.94 · p95 13.50 · p99 16.79 (1% low 60 fps) · worst 59.00 ms · 81 spikes over 1.5× median · 63.1 stutters/min (work 53.9/min)
- warm-up: 325 frames over 9.5 s, worst 2573.4 ms
- hitches (> 26.7 ms): **36** measured, 46 in warm-up
- timed GPU passes: 6.73 ms of a 10.94 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.69 ms, limit 11 ms
- **FAIL** frame_ms: 69 stutters (> 1.5x moving median of 60) in 1.09 min = 63.07/min, limit 1/min
- **FAIL** worst frame 59.00 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.886 | 2.035 | 2.067 | 0.93 | 0.176 |
| main_opaque_pass_3d | 1.517 | 1.637 | 2.054 | 0.93 | 0.067 |
| early prepass | 0.454 | 0.490 | 0.717 | 0.93 | 0.087 |
| bloom | 0.291 | 0.315 | 0.321 | 0.93 | 0.062 |
| atmosphere_luts | 0.281 | 0.303 | 0.365 | 0.93 | 0.003 |
| water_velocity | 0.258 | 0.278 | 0.347 | 0.93 | 0.002 |
| auxiliary_views | 0.192 | 0.207 | 0.679 | 0.93 | 0.198 |
| water_sspr | 0.187 | 0.201 | 0.203 | 0.93 | 0.002 |
| ssao | 0.174 | 0.188 | 0.191 | 0.93 | 0.007 |
| atmos_froxel_inject | 0.135 | 0.145 | 0.147 | 0.93 | 0.001 |
| clouds_trace | 0.130 | 0.140 | 0.231 | 0.93 | 0.001 |
| uniform_allocation | 0.121 | 0.131 | 0.182 | 0.93 | 0.025 |
| water_fft | 0.120 | 0.129 | 0.137 | 0.93 | 0.003 |
| upscaling | 0.108 | 0.117 | 0.125 | 0.93 | 0.004 |
| clouds_composite | 0.098 | 0.106 | 0.109 | 0.93 | 0.002 |
| bin_unpacking | 0.097 | 0.105 | 0.141 | 0.93 | 0.018 |
| tonemapping | 0.079 | 0.085 | 0.089 | 0.93 | 0.004 |
| main_transmissive_pass_3d | 0.078 | 0.085 | 0.178 | 0.93 | 0.008 |
| clustering | 0.059 | 0.064 | 0.067 | 0.93 | 0.018 |
| render_sky | 0.048 | 0.051 | 0.052 | 0.93 | 0.002 |
| lightprobe_irradiance_map | 0.046 | 0.413 | 0.416 | 0.11 | 0.001 |
| clouds_resolve | 0.045 | 0.049 | 0.049 | 0.93 | 0.001 |
| surface_exposure | 0.041 | 0.044 | 0.045 | 0.93 | 0.001 |
| surface_trail | 0.037 | 0.056 | 0.057 | 0.65 | 0.002 |
| surface_exposure_far | 0.035 | 0.043 | 0.045 | 0.82 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.93 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.047 | 0.93 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.032 | 0.93 | 0.001 |
| lightprobe_radiance_map | 0.024 | 0.219 | 0.229 | 0.11 | 0.004 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.93 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.93 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.026 | 0.93 | 0.122 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.93 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.93 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.93 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.92 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.522 | 11.870 | 56.96 | 57158.6 |
| render/busy/ms | 8.972 | 11.354 | 56.39 | 53860.1 |
| main/busy/ms | 8.944 | 10.635 | 27.92 | 53687.9 |
| stage/render/render | 5.610 | 7.123 | 53.77 | 33679.8 |
| stage/main/post_update | 5.561 | 6.884 | 9.81 | 33385.3 |
| render/graph/ms | 4.869 | 6.367 | 52.96 | 29226.1 |
| encode//sum | 4.024 | 5.454 | 149.00 | 24158.3 |
| stage/render/graph_submit | 3.645 | 4.793 | 51.77 | 21879.2 |
| encode/submit | 2.338 | 3.218 | 21.94 | 14034.0 |
| stage/main/update | 2.119 | 2.540 | 21.82 | 12719.8 |
| encode/finish_wall | 1.381 | 1.970 | 49.19 | 8289.0 |
| stage/render/camera_driver | 1.171 | 1.660 | 4.09 | 7027.7 |
| encode/pass:shadow_directional_light/sum | 0.848 | 1.530 | 3.18 | 5090.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.818 | 1.102 | 2.23 | 4911.1 |
| encode/pass:early prepass/sum | 0.794 | 1.083 | 48.52 | 4768.5 |
| stage/render/prepare_write_phase_buffers | 0.789 | 1.132 | 2.42 | 4735.6 |
| stage/render/prepare_assets | 0.567 | 0.727 | 9.96 | 3402.7 |
| bigspace/total/ms | 0.527 | 1.307 | 2.36 | 3161.9 |
| encode/pass:shadow_point_light/sum | 0.468 | 2.293 | 9.04 | 2809.9 |
| render/present/ms | 0.457 | 1.110 | 2.49 | 2741.2 |
| stage/render/prepare_resources | 0.456 | 0.555 | 1.58 | 2738.5 |
| stage/render/prepare_meshes | 0.428 | 0.540 | 9.94 | 2571.0 |
| stage/main/first | 0.398 | 0.522 | 5.42 | 2391.4 |
| stage/main/pre_update | 0.358 | 0.517 | 3.74 | 2148.5 |
| stage/render/specialize | 0.316 | 0.416 | 10.77 | 1898.2 |
| terrain/update/ms | 0.306 | 0.393 | 2.18 | 1835.5 |
| stage/render/queue | 0.289 | 0.424 | 7.71 | 1734.9 |
| stage/render/prepare_bind_groups | 0.287 | 0.520 | 2.75 | 1721.0 |
| terrain/select/ms | 0.272 | 0.349 | 1.06 | 1635.0 |
| controller/feed/ms | 0.264 | 0.328 | 0.77 | 1584.3 |
| stream/drive/ms | 0.252 | 0.336 | 19.70 | 1511.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.237 | 0.347 | 1.00 | 1423.2 |
| bigspace/low_precision/ms | 0.219 | 0.700 | 1.81 | 1315.7 |
| stage/main/fixed_main_loop | 0.216 | 0.367 | 1.33 | 1297.7 |
| far/select/ms | 0.196 | 0.263 | 0.55 | 1178.5 |
| stream/tick/ms | 0.195 | 0.266 | 0.72 | 1172.6 |
| stage/render/create_views | 0.193 | 0.249 | 1.19 | 1161.5 |
| stage/render/prepare_batch_phases | 0.187 | 0.264 | 0.56 | 1120.5 |
| stage/render/extract_commands | 0.172 | 0.202 | 0.41 | 1031.7 |
| stage/render/prepare_flush | 0.164 | 0.299 | 1.42 | 982.4 |
| anim/evaluate/ms | 0.154 | 0.246 | 1.49 | 927.1 |
| anim/evaluate_cpu/ms | 0.150 | 0.191 | 0.71 | 902.7 |
| bigspace/high_precision/ms | 0.147 | 0.591 | 1.66 | 883.3 |
| stage/render/prepare_views | 0.136 | 0.171 | 0.46 | 816.9 |
| controller/move/ms | 0.132 | 0.214 | 0.51 | 789.5 |
| stage/main/last | 0.117 | 0.162 | 4.80 | 701.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.098 | 0.157 | 0.74 | 587.2 |
| render/acquire/ms | 0.093 | 0.120 | 0.42 | 557.3 |
| stage/render/prepare_collect_phase_buffers | 0.056 | 0.121 | 0.49 | 338.5 |
| pipelines/compile_ms | 0.050 | 0.000 | 121.37 | 297.3 |
| pipelines/front_ms | 0.047 | 0.000 | 117.19 | 285.1 |
| physics/step/ms | 0.047 | 0.086 | 0.58 | 284.8 |
| physics/insert/ms | 0.031 | 0.352 | 0.76 | 189.0 |
| stage/render/cleanup | 0.031 | 0.088 | 0.86 | 186.0 |
| stage/render/phase_sort | 0.031 | 0.052 | 0.32 | 183.3 |
| encode/resolve_diagnostics/sum | 0.030 | 0.040 | 0.48 | 181.2 |
| weather/map/ms | 0.025 | 0.032 | 0.08 | 147.2 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.021 | 0.64 | 88.2 |
| anim/apply/ms | 0.011 | 0.021 | 0.45 | 68.7 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.22 | 65.3 |
| controller/avoid/ms | 0.010 | 0.014 | 0.05 | 62.8 |
| weather/eval/ms | 0.010 | 0.014 | 0.09 | 62.4 |
| script/update/ms | 0.007 | 0.009 | 0.22 | 43.4 |
| script/input/ms | 0.005 | 0.006 | 0.21 | 31.7 |
| stream/despawn/ms | 0.005 | 0.000 | 1.12 | 30.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 28.7 |
| script/ui/ms | 0.004 | 0.006 | 0.21 | 26.9 |
| controller/write/ms | 0.003 | 0.004 | 0.24 | 20.9 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 13.8 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.18 | 12.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.23 | 6.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.13 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.3 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 30 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1277 | 2262 | 595 |
| crowd/c0/meshes/count | count | 352 | 646 | 158 |
| crowd/c1/count | count | 146 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5940 | 5601 |
| crowd/c1/meshes/count | count | 1179 | 1280 | 1183 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4612 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.41 | 48.92 | 0.30 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.64 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.22 | 0.01 |
| encode/finish_critical | ms | 0.98 | 48.92 | 0.89 |
| encode/pass:early prepass/max | ms | 0.79 | 48.52 | 0.86 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.83 | 2.23 | 0.66 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.74 | 0.14 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.00 | 0.29 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 3.18 | 0.89 |
| encode/pass:shadow_point_light/max | ms | 0.17 | 1.47 | 0.84 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.48 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.22 | 56.39 | 8.89 |
| gpu/timed/ms | ms | 7.97 | 9.34 | 8.47 |
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
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1283 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3388 | 5148 | 3187 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1283 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3388 | 5148 | 3187 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 25 | 202 | 4 |
| phases/shadow/bins | count | 5039 | 9058 | 2405 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.5 k | 6352 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 206 | 253 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2349 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 186 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 323 | 324 | 324 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 565 | 6 |
| preprocess/bind_groups_created | count | 13 | 565 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 229 | 123 |
| process/cpu_usage | % | 26.68 | 36.61 | 30.58 |
| process/mem_usage | GiB | 5.48 | 5.64 | 5.64 |
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
| skin/blocks_changed/count | count | 14 | 125 | 14 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 369.9 k | 807.2 k | 492.5 k |
| skin/joints_written/count | count | 5780 | 12.6 k | 7696 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4917 | 5011 | 5011 |
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
| system/cpu_usage | % | 31.70 | 43.18 | 29.85 |
| system/mem_usage | % | 56.70 | 57.25 | 57.20 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 74 | 251 | 54 |
| views/all/bins | count | 7614 | 11.9 k | 5903 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.4 k | 14.0 k |
| views/all/pipelines | count | 253 | 300 | 75 |
| views/all/pipelines_distinct | count | 71 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 33.3 k | 8117 |
| views/cascade/batch_sets | count | 9 | 54 | 4 |
| views/cascade/bins | count | 2802 | 6195 | 2405 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8903 | 23.5 k | 6352 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 424 | 23.0 k | 201 |
| views/lamp_face/batch_sets | count | 5 | 194 | 0 |
| views/lamp_face/bins | count | 2168 | 2951 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 1 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2408 | 3248 | 0 |
| views/lamp_face/pipelines | count | 170 | 208 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 80 | 2892 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2862 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8378 | 13.1 k | 7608 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8549 | 13.3 k | 7916 |
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
| anim/bones_written/count | count | 53.25 | 272 | 319.7 k |
| anim/events/count | count | 0.09 | 3 | 561 |
| assets/Image/added | count | 0.10 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.68 | 152 | 4061 |
| assets/Mesh/removed | count | 0.81 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.60 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.78 | 1274 | 4676 |
| encode//count | count | 35.73 | 220 | 214.5 k |
| encode/bind_walk/init_textures | count | 10.4 k | 41.0 k | 62.43 M |
| encode/bind_walk/init_textures_skipped | count | 51.6 k | 81.1 k | 309.60 M |
| encode/bind_walk/inits | count | 468 | 1098 | 2.81 M |
| encode/bind_walk/inits_skipped | count | 105 | 155 | 629.4 k |
| encode/bind_walk/merge_textures | count | 10.3 k | 40.9 k | 62.00 M |
| encode/bind_walk/merge_textures_skipped | count | 10.3 k | 26.6 k | 61.77 M |
| encode/bind_walk/merges | count | 100 | 303 | 603.1 k |
| encode/bind_walk/merges_skipped | count | 94.36 | 148 | 566.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6003 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6003 |
| encode/pass:early prepass/count | count | 1 | 1 | 6003 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6003 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6003 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6003 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6003 |
| encode/pass:shadow_point_light/count | count | 1.94 | 48 | 11.7 k |
| encode/queue/command_buffers | count | 47.69 | 278 | 286.3 k |
| encode/queue/submits | count | 2.02 | 4 | 12.1 k |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 4.88 M | 11.33 G |
| encode/queue/write_buffer/calls | count | 269 | 843 | 1.62 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.5 k | 3.52 M | 1.07 G |
| encode/queue/write_buffer_with/calls | count | 11.01 | 465 | 66.1 k |
| encode/queue/write_texture/bytes | bytes | 11.9 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6003 |
| physics/inserts | count | 0.38 | 111 | 2255 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 3 | 3938 |
| physics/steps_dropped | count | 0.00 | 1 | 1 |
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
| stream/upload/bytes | bytes | 5639 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1236 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.7 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2573.4 | main/busy/ms 2543, stage/main/post_update 2440, stage/main/update 94.87, surface/advance_cpu_ms 66.55, stage/main/pre_update 4.69, stage/main/last 1.18, surface/cell_upload_ms 0.78, weather/map/ms 0.60, stage/main/first 0.50, stream/drive/ms 0.48, stage/main/fixed_main_loop 0.34, stream/tick/ms 0.17 |
| warm-up | 3.33 | 754.2 | main/busy/ms 2685, stage/main/post_update 2466, render/thread/ms 716, render/busy/ms 549, stage/render/prepare_assets 497, stage/main/update 205, stage/render/prepare_views 162, render/acquire/ms 162, surface/advance_cpu_ms 66.55, stage/render/create_views 29.73, stage/render/render 15.89, render/graph/ms 9.41 |
| warm-up | 3.51 | 186.1 | render/thread/ms 885, render/busy/ms 616, stage/render/prepare_assets 547, main/busy/ms 174, stage/render/prepare_views 162, render/acquire/ms 162, pipelines/compile_ms 131, stage/render/render 123, stage/main/update 118, render/present/ms 107, pipelines/driver_ms 90.16, pipelines/front_ms 40.67 |
| warm-up | 3.56 | 43.3 | pipelines/compile_ms 839, pipelines/front_ms 597, pipelines/driver_ms 242, render/thread/ms 210, pipelines/wait_ms 180, stage/render/render 141, main/busy/ms 113, render/busy/ms 107, render/present/ms 103, stage/main/update 62.30, stage/render/prepare_assets 51.45, render/graph/ms 37.39 |
| warm-up | 3.71 | 148.4 | pipelines/wait_ms 3825, pipelines/compile_ms 2303, pipelines/front_ms 2066, pipelines/driver_ms 237, main/busy/ms 149, render/thread/ms 145, render/busy/ms 142, stage/render/prepare_assets 82.64, stage/main/update 59.92, stage/render/render 51.28, render/graph/ms 47.78, stage/main/post_update 41.82 |
| warm-up | 3.74 | 29.8 | pipelines/wait_ms 3755, pipelines/compile_ms 1702, pipelines/front_ms 1612, render/thread/ms 131, render/busy/ms 129, stage/render/prepare_assets 94.73, pipelines/driver_ms 89.39, main/busy/ms 87.91, stage/main/post_update 31.81, stage/main/last 29.28, stage/render/render 26.30, render/graph/ms 23.37 |
| warm-up | 3.78 | 44.3 | pipelines/compile_ms 338, pipelines/front_ms 322, pipelines/wait_ms 222, render/thread/ms 69.20, render/busy/ms 67.35, stage/render/prepare_assets 45.26, main/busy/ms 36.74, stage/main/post_update 20.62, pipelines/driver_ms 15.84, terrain/update/ms 14.91, stage/render/render 14.50, render/graph/ms 11.97 |
| warm-up | 3.87 | 30.1 | pipelines/compile_ms 569, pipelines/front_ms 544, pipelines/wait_ms 99.43, render/thread/ms 48.02, render/busy/ms 46.10, main/busy/ms 34.40, pipelines/driver_ms 25.40, stage/render/prepare_assets 21.66, stage/main/post_update 15.73, stage/render/render 12.65, render/graph/ms 10.28, terrain/update/ms 9.02 |
| warm-up | 3.96 | 71.8 | pipelines/compile_ms 1581, pipelines/front_ms 1526, render/thread/ms 57.28, render/busy/ms 55.77, pipelines/driver_ms 54.93, main/busy/ms 52.27, stage/main/update 40.70, stream/drive/ms 35.69, stage/render/prepare_assets 33.47, pipelines/wait_ms 19.92, stage/render/render 11.89, render/graph/ms 10.13 |
| warm-up | 4.07 | 91.3 | pipelines/compile_ms 467, pipelines/front_ms 446, render/thread/ms 64.82, render/busy/ms 62.98, main/busy/ms 56.56, stage/main/update 41.90, stage/render/prepare_assets 36.92, stream/drive/ms 36.91, pipelines/driver_ms 21.07, stage/render/render 19.15, render/graph/ms 16.85, stage/render/graph_submit 13.72 |
| warm-up | 4.11 | 37.5 | pipelines/compile_ms 537, pipelines/front_ms 518, render/thread/ms 84.84, render/busy/ms 83.11, stage/render/prepare_assets 54.62, main/busy/ms 38.96, stage/main/update 21.56, pipelines/driver_ms 19.02, stage/render/render 18.54, pipelines/wait_ms 18.05, pipelines/lock_ms 17.36, render/graph/ms 16.35 |
| warm-up | 4.20 | 91.0 | pipelines/compile_ms 597, pipelines/front_ms 578, render/thread/ms 85.49, render/busy/ms 83.87, stage/render/prepare_assets 63.71, main/busy/ms 50.41, stage/main/update 27.07, pipelines/driver_ms 18.49, pipelines/wait_ms 17.23, pipelines/lock_ms 16.53, stage/main/post_update 14.94, stream/drive/ms 14.64 |
| warm-up | 4.27 | 75.3 | pipelines/compile_ms 543, pipelines/front_ms 527, render/thread/ms 102, render/busy/ms 99.88, stage/render/prepare_assets 75.95, main/busy/ms 60.46, stage/main/update 36.82, pipelines/driver_ms 16.24, stage/render/render 15.53, stage/main/post_update 13.41, render/graph/ms 13.15, stage/render/graph_submit 10.68 |
| warm-up | 4.32 | 42.5 | pipelines/compile_ms 1255, pipelines/front_ms 1209, render/thread/ms 82.67, render/busy/ms 80.99, main/busy/ms 61.58, stage/render/prepare_assets 54.27, pipelines/driver_ms 46.05, stage/main/update 40.62, stage/render/render 17.24, render/graph/ms 14.93, stage/render/graph_submit 12.54, stage/main/post_update 11.50 |
| warm-up | 4.40 | 85.4 | pipelines/compile_ms 1365, pipelines/front_ms 1308, render/thread/ms 109, render/busy/ms 106, pipelines/driver_ms 56.69, pipelines/wait_ms 52.40, stage/render/render 50.23, render/graph/ms 47.04, main/busy/ms 46.50, stage/render/graph_submit 44.66, stage/render/prepare_assets 43.71, encode/submit 37.28 |
| warm-up | 4.50 | 95.9 | pipelines/compile_ms 1545, pipelines/front_ms 1492, pipelines/wait_ms 460, render/thread/ms 142, render/busy/ms 139, stage/render/render 65.92, render/graph/ms 62.38, main/busy/ms 61.93, pipelines/driver_ms 53.27, stage/render/graph_submit 43.47, encode/submit 38.19, stage/render/prepare_assets 33.51 |
| warm-up | 4.56 | 62.3 | pipelines/compile_ms 2197, pipelines/front_ms 2132, pipelines/wait_ms 498, render/thread/ms 119, render/busy/ms 117, pipelines/driver_ms 65.45, main/busy/ms 64.56, stage/render/render 39.72, render/graph/ms 36.91, stage/render/prepare_assets 35.58, stage/main/fixed_main_loop 25.45, stage/render/camera_driver 25.09 |
| warm-up | 4.62 | 59.8 | pipelines/compile_ms 1697, pipelines/front_ms 1628, pipelines/wait_ms 199, render/thread/ms 108, render/busy/ms 106, pipelines/driver_ms 69.33, main/busy/ms 47.66, stage/render/prepare_assets 44.61, stage/render/render 32.73, render/graph/ms 29.79, encode//sum 28.24, stage/main/update 20.29 |
| warm-up | 4.67 | 49.6 | pipelines/compile_ms 1277, pipelines/front_ms 1222, pipelines/wait_ms 203, main/busy/ms 126, stage/main/update 103, render/thread/ms 90.90, render/busy/ms 88.50, stream/drive/ms 60.29, pipelines/driver_ms 55.92, encode//sum 36.05, stage/render/render 32.45, render/graph/ms 29.14 |
| warm-up | 4.81 | 143.8 | pipelines/compile_ms 2352, pipelines/front_ms 2286, pipelines/wait_ms 147, main/busy/ms 124, stage/main/update 102, render/thread/ms 95.84, render/busy/ms 93.07, pipelines/driver_ms 66.55, stream/drive/ms 62.23, stage/render/prepare_assets 38.40, stage/render/render 36.92, render/graph/ms 33.25 |
| warm-up | 4.85 | 41.3 | pipelines/compile_ms 2223, pipelines/front_ms 2163, main/busy/ms 112, render/thread/ms 96.73, render/busy/ms 94.72, stage/main/update 84.44, stream/drive/ms 71.30, pipelines/wait_ms 64.01, pipelines/driver_ms 60.42, stage/render/prepare_assets 38.96, stage/render/render 27.03, render/graph/ms 23.86 |
| warm-up | 5.06 | 201.7 | pipelines/compile_ms 3065, pipelines/front_ms 2663, pipelines/driver_ms 402, render/thread/ms 144, render/busy/ms 142, main/busy/ms 123, stage/render/prepare_assets 101, stage/main/update 89.20, stream/drive/ms 70.48, pipelines/wait_ms 42.45, stage/main/post_update 19.53, stage/render/render 14.19 |
| warm-up | 5.10 | 44.5 | pipelines/compile_ms 2938, pipelines/front_ms 2533, pipelines/driver_ms 405, render/thread/ms 147, render/busy/ms 143, main/busy/ms 103, stage/render/prepare_assets 100, stage/main/update 68.56, stream/drive/ms 46.98, pipelines/wait_ms 42.66, stage/render/render 31.29, render/graph/ms 26.80 |
| warm-up | 5.18 | 84.2 | pipelines/compile_ms 140, pipelines/front_ms 132, main/busy/ms 92.06, render/thread/ms 73.45, render/busy/ms 69.43, stage/main/update 67.26, stream/drive/ms 46.11, stage/render/render 29.22, stage/render/prepare_assets 27.82, render/graph/ms 24.65, stage/render/graph_submit 19.72, encode/submit 18.75 |
| warm-up | 5.27 | 53.5 | pipelines/compile_ms 842, pipelines/front_ms 815, main/busy/ms 61.40, stage/main/update 45.56, render/thread/ms 42.12, render/busy/ms 40.29, stream/drive/ms 34.23, pipelines/driver_ms 26.47, stage/render/prepare_assets 13.91, stage/render/render 13.00, render/graph/ms 10.69, stage/main/post_update 8.42 |
| warm-up | 5.34 | 34.6 | pipelines/compile_ms 185, pipelines/front_ms 175, render/thread/ms 47.97, render/busy/ms 46.52, main/busy/ms 39.68, stage/main/update 20.83, stage/render/prepare_assets 15.88, stage/render/render 14.38, stage/main/post_update 13.48, render/graph/ms 12.35, pipelines/driver_ms 10.05, stage/render/graph_submit 9.20 |
| warm-up | 5.39 | 51.3 | pipelines/compile_ms 286, pipelines/front_ms 274, render/thread/ms 76.52, render/busy/ms 74.97, main/busy/ms 62.82, stage/main/post_update 36.46, stage/render/prepare_assets 31.79, stage/main/update 20.32, stage/render/render 13.44, pipelines/driver_ms 12.00, render/graph/ms 11.09, stage/render/prepare_write_phase_buffers 10.88 |
| warm-up | 5.47 | 52.8 | pipelines/compile_ms 165, pipelines/front_ms 159, render/thread/ms 59.03, render/busy/ms 57.60, main/busy/ms 39.59, stage/render/prepare_assets 28.32, stage/main/update 22.76, stage/render/render 12.67, render/graph/ms 10.69, stage/main/post_update 10.23, stage/render/graph_submit 7.66, encode//sum 6.92 |
| warm-up | 5.57 | 31.3 | pipelines/compile_ms 309, pipelines/front_ms 300, render/thread/ms 43.96, render/busy/ms 42.64, main/busy/ms 28.92, stage/render/prepare_assets 22.28, stage/main/post_update 11.92, stage/render/render 10.21, stage/main/update 10.15, pipelines/driver_ms 9.72, render/graph/ms 8.23, encode//sum 7.26 |
| warm-up | 5.68 | 31.2 | render/thread/ms 44.41, render/busy/ms 42.70, main/busy/ms 41.90, stage/main/update 23.49, stage/render/prepare_assets 21.26, stage/render/render 15.24, stage/main/post_update 12.50, render/graph/ms 12.42, stream/drive/ms 12.24, encode//sum 9.83, stage/render/graph_submit 8.61, encode/submit 5.40 |
| warm-up | 5.71 | 31.2 | pipelines/compile_ms 606, pipelines/front_ms 587, render/thread/ms 52.10, render/busy/ms 49.84, main/busy/ms 36.41, stage/render/prepare_assets 23.28, pipelines/driver_ms 18.56, stage/main/post_update 16.16, stage/render/render 14.24, stage/main/update 13.07, render/graph/ms 11.11, encode//sum 8.32 |
| warm-up | 5.75 | 37.7 | pipelines/compile_ms 1329, pipelines/front_ms 1294, render/thread/ms 55.93, render/busy/ms 54.07, pipelines/driver_ms 34.60, main/busy/ms 30.26, stage/render/prepare_assets 28.50, stage/main/post_update 14.74, stage/render/render 11.87, render/graph/ms 9.29, stage/main/update 8.40, encode//sum 7.72 |
| warm-up | 6.39 | 35.6 | pipelines/compile_ms 167, pipelines/front_ms 164, render/thread/ms 47.76, render/busy/ms 46.65, main/busy/ms 32.97, stage/render/prepare_assets 18.55, stage/main/update 13.49, stage/main/post_update 12.12, stage/render/render 9.44, render/graph/ms 7.81, encode//sum 7.51, stage/render/specialize 6.45 |
| warm-up | 6.48 | 29.4 | pipelines/compile_ms 83.94, pipelines/front_ms 81.04, render/thread/ms 40.03, render/busy/ms 38.41, main/busy/ms 37.91, stage/main/update 19.61, stage/render/render 14.42, stage/main/post_update 13.48, render/graph/ms 11.34, stage/render/prepare_assets 8.33, encode//sum 7.42, stage/render/graph_submit 6.94 |
| warm-up | 6.63 | 33.1 | pipelines/compile_ms 960, pipelines/front_ms 929, main/busy/ms 45.44, render/thread/ms 41.73, render/busy/ms 39.84, pipelines/driver_ms 30.51, stage/main/update 24.15, stage/main/post_update 16.12, stage/render/render 14.21, render/graph/ms 11.52, encode//sum 9.00, stage/render/graph_submit 7.26 |
| warm-up | 6.73 | 33.1 | pipelines/compile_ms 94.47, pipelines/front_ms 85.93, render/thread/ms 38.79, render/busy/ms 37.58, main/busy/ms 30.53, stage/render/prepare_assets 17.29, stage/main/update 12.60, stage/main/post_update 11.22, stage/render/render 8.99, pipelines/driver_ms 8.55, encode//sum 7.51, render/graph/ms 7.23 |
| warm-up | 6.83 | 34.9 | render/thread/ms 44.28, render/busy/ms 43.20, main/busy/ms 39.82, stage/main/update 21.64, stage/render/prepare_assets 21.59, stage/main/post_update 11.34, stage/render/render 9.28, encode//sum 7.79, render/graph/ms 7.54, stage/render/graph_submit 5.19, stream/drive/ms 4.02, stage/render/prepare_write_phase_buffers 2.87 |
| warm-up | 7.30 | 37.9 | render/thread/ms 42.62, render/busy/ms 41.64, main/busy/ms 24.81, stage/main/post_update 11.04, stage/render/render 10.68, stage/render/specialize 10.68, stage/main/update 9.90, render/graph/ms 9.09, encode//sum 8.58, stage/render/queue 7.57, stage/render/graph_submit 6.41, stage/render/prepare_assets 3.85 |
| warm-up | 8.61 | 31.5 | render/thread/ms 36.95, render/busy/ms 35.88, main/busy/ms 19.65, stage/render/render 10.77, stage/main/post_update 10.59, render/graph/ms 9.23, stage/render/specialize 9.15, stage/render/graph_submit 7.07, stage/render/queue 6.77, encode//sum 6.66, stage/main/update 6.40, encode/submit 4.12 |
| measure | 0.41 | 30.5 | render/thread/ms 48.23, render/busy/ms 46.90, encode//sum 34.73, stage/render/render 20.24, render/graph/ms 18.31, main/busy/ms 16.69, stage/render/graph_submit 13.93, stage/main/post_update 10.67, encode/pass:shadow_point_light/sum 8.70, stage/render/specialize 8.67, encode/submit 7.62, stage/render/queue 6.46 |
| measure | 1.69 | 31.0 | render/thread/ms 36.01, render/busy/ms 34.89, main/busy/ms 19.99, stage/render/render 10.60, stage/main/post_update 10.22, encode//sum 9.38, render/graph/ms 9.02, stage/render/specialize 8.76, stage/render/graph_submit 6.47, stage/render/queue 5.85, stage/render/prepare_meshes 4.54, stage/main/pre_update 4.05 |
| measure | 2.51 | 32.0 | render/thread/ms 35.57, render/busy/ms 34.71, stage/render/render 28.92, render/graph/ms 27.65, stage/render/graph_submit 25.12, encode/submit 22.87, main/busy/ms 19.42, stage/main/post_update 12.24, encode//sum 7.48, stage/main/update 4.78, encode/pass:shadow_point_light/sum 2.83, stage/render/camera_driver 2.43 |
| measure | 4.40 | 29.3 | render/thread/ms 35.03, render/busy/ms 33.95, main/busy/ms 18.74, stage/main/post_update 12.42, stage/render/render 10.03, stage/render/specialize 9.33, render/graph/ms 8.46, encode//sum 6.81, stage/render/queue 6.55, stage/render/graph_submit 6.26, stage/main/update 3.69, encode/submit 3.65 |
| measure | 5.71 | 29.5 | render/thread/ms 33.47, render/busy/ms 32.43, main/busy/ms 18.14, stage/main/post_update 12.00, stage/render/render 9.64, stage/render/specialize 8.54, render/graph/ms 8.14, encode//sum 7.41, stage/render/graph_submit 6.19, stage/render/queue 5.76, stage/main/update 3.71, encode/submit 3.45 |
| measure | 8.29 | 29.7 | render/thread/ms 36.87, render/busy/ms 34.29, main/busy/ms 17.58, stage/render/render 12.73, stage/main/post_update 10.82, render/graph/ms 9.66, stage/render/specialize 8.48, encode//sum 7.52, stage/render/graph_submit 7.32, stage/render/queue 6.44, stage/main/update 4.25, encode/submit 4.09 |
| measure | 8.54 | 59.0 | encode//sum 153, render/thread/ms 65.10, render/busy/ms 64.01, stage/render/render 58.82, render/graph/ms 57.33, stage/render/graph_submit 54.91, encode/finish_wall 50.33, encode/pass:early prepass/sum 49.19, main/busy/ms 17.93, stage/main/post_update 11.51, encode/submit 4.65, stage/main/update 4.16 |
| measure | 10.97 | 28.3 | render/thread/ms 36.16, render/busy/ms 35.19, main/busy/ms 18.20, stage/main/post_update 11.59, stage/render/render 11.43, render/graph/ms 10.02, stage/render/specialize 8.98, encode//sum 8.57, stage/render/graph_submit 7.41, stage/render/queue 6.66, encode/submit 4.41, stage/main/update 4.12 |
| measure | 15.74 | 29.0 | render/thread/ms 37.66, render/busy/ms 36.68, main/busy/ms 18.32, stage/render/render 11.80, stage/main/post_update 11.51, encode//sum 11.51, render/graph/ms 10.36, stage/render/specialize 9.94, stage/render/graph_submit 7.64, stage/render/queue 6.82, encode/submit 4.49, stage/main/update 4.22 |
| measure | 18.15 | 29.4 | main/busy/ms 37.96, stage/main/update 23.78, render/thread/ms 20.38, stream/drive/ms 19.94, render/busy/ms 19.36, stage/render/render 11.60, stage/main/post_update 11.55, render/graph/ms 10.16, encode//sum 8.65, stage/render/graph_submit 7.54, encode/submit 4.89, encode/finish_wall 2.71 |
| measure | 19.44 | 29.4 | render/thread/ms 37.73, render/busy/ms 36.68, main/busy/ms 18.62, stage/main/post_update 11.59, stage/render/render 11.17, stage/render/specialize 9.85, render/graph/ms 9.71, encode//sum 9.03, stage/render/graph_submit 7.13, stage/render/queue 6.80, encode/submit 4.63, stage/main/update 4.45 |
| measure | 23.03 | 33.8 | render/thread/ms 40.82, render/busy/ms 39.63, main/busy/ms 23.93, stage/render/render 12.76, stage/main/post_update 12.54, encode//sum 11.53, render/graph/ms 11.14, stage/render/specialize 11.01, stage/render/graph_submit 8.36, stage/render/queue 7.44, stage/main/update 6.06, encode/submit 5.20 |
| measure | 24.65 | 34.5 | render/thread/ms 45.33, render/busy/ms 44.26, main/busy/ms 22.05, encode//sum 16.84, stage/render/render 16.62, render/graph/ms 15.07, stage/main/post_update 14.00, stage/render/graph_submit 11.07, stage/render/specialize 9.84, encode/submit 7.19, stage/render/queue 6.89, stage/main/update 4.82 |
| measure | 27.28 | 34.2 | render/thread/ms 46.19, render/busy/ms 45.06, main/busy/ms 24.82, stage/render/render 15.14, render/graph/ms 13.54, encode//sum 13.02, stage/main/post_update 12.98, stage/render/specialize 11.09, stage/render/graph_submit 10.20, stage/render/queue 8.12, encode/submit 6.40, stage/main/update 6.02 |
| measure | 28.81 | 29.1 | render/thread/ms 39.53, render/busy/ms 38.50, main/busy/ms 20.06, stage/render/render 13.85, stage/main/post_update 12.62, render/graph/ms 12.37, encode//sum 10.78, stage/render/specialize 9.61, stage/render/graph_submit 9.15, stage/render/queue 6.18, encode/submit 6.18, stage/main/update 4.64 |
| measure | 30.50 | 33.7 | render/thread/ms 40.90, render/busy/ms 39.73, main/busy/ms 20.80, stage/render/render 12.91, stage/main/post_update 12.37, render/graph/ms 11.26, encode//sum 11.04, stage/render/specialize 9.91, stage/render/graph_submit 8.82, stage/render/queue 6.95, stage/main/update 5.57, encode/submit 5.45 |
| measure | 31.97 | 29.6 | render/thread/ms 38.59, render/busy/ms 37.61, main/busy/ms 20.32, stage/render/render 13.15, stage/main/post_update 12.94, render/graph/ms 11.62, encode//sum 10.46, stage/render/specialize 9.08, stage/render/graph_submit 9.08, stage/render/queue 6.36, encode/pass:shadow_point_light/sum 5.61, encode/submit 5.48 |
| measure | 35.35 | 31.6 | render/thread/ms 38.21, render/busy/ms 37.16, main/busy/ms 20.90, stage/main/post_update 12.32, stage/render/render 11.41, stage/render/specialize 10.05, render/graph/ms 10.00, encode//sum 8.52, stage/render/graph_submit 7.59, stage/render/queue 7.15, stage/main/update 4.72, encode/submit 4.66 |
| measure | 40.81 | 34.0 | render/thread/ms 37.34, render/busy/ms 36.34, main/busy/ms 19.10, stage/main/post_update 11.27, stage/render/render 9.92, stage/render/specialize 9.84, render/graph/ms 8.46, encode//sum 7.90, stage/render/queue 6.96, stage/render/graph_submit 6.52, stage/main/update 4.26, encode/submit 3.72 |
| measure | 43.22 | 30.0 | render/thread/ms 35.04, render/busy/ms 34.00, main/busy/ms 16.61, stage/main/post_update 9.83, stage/render/specialize 9.65, stage/render/render 9.36, render/graph/ms 8.01, encode//sum 7.16, stage/render/queue 6.33, stage/render/graph_submit 6.24, stage/main/update 4.29, encode/submit 3.55 |
| measure | 45.67 | 31.5 | render/thread/ms 38.45, render/busy/ms 37.37, main/busy/ms 17.10, stage/render/render 10.72, stage/render/specialize 10.00, stage/main/post_update 9.87, render/graph/ms 9.27, encode//sum 8.43, stage/render/graph_submit 7.28, stage/render/queue 6.50, stage/main/update 4.21, stage/render/prepare_meshes 4.21 |
| measure | 56.43 | 28.5 | render/thread/ms 35.44, render/busy/ms 34.47, main/busy/ms 15.11, stage/render/render 10.08, stage/main/post_update 9.12, render/graph/ms 8.78, stage/render/specialize 8.76, encode//sum 7.55, stage/render/graph_submit 6.75, stage/render/queue 6.64, stage/render/prepare_meshes 3.89, stage/main/update 3.73 |
| measure | 59.04 | 45.7 | render/thread/ms 57.48, render/busy/ms 56.05, main/busy/ms 32.05, stage/main/post_update 18.24, stage/render/render 17.26, render/graph/ms 15.32, encode//sum 13.22, stage/render/graph_submit 12.22, stage/render/prepare_meshes 10.28, stage/render/specialize 10.14, stage/main/update 9.08, encode/submit 6.43 |
| measure | 59.07 | 28.6 | render/thread/ms 58.56, render/busy/ms 57.52, main/busy/ms 27.12, stage/render/prepare_assets 14.95, stage/main/post_update 13.95, stage/render/prepare_meshes 10.91, stage/render/specialize 10.23, stage/render/render 9.80, encode//sum 8.54, render/graph/ms 8.51, stage/main/update 7.39, stage/render/graph_submit 6.53 |
| measure | 62.58 | 30.0 | render/thread/ms 38.64, render/busy/ms 37.67, main/busy/ms 19.38, stage/render/render 11.48, stage/main/post_update 10.46, render/graph/ms 10.17, stage/render/specialize 9.88, encode//sum 8.55, stage/render/graph_submit 8.05, stage/render/queue 6.70, stage/main/update 6.11, encode/submit 4.51 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set render.depth_only.skip_material=on --resolution 2560x1440 --bench-out target/perf\pt_nomat\balmora_walk@2560x1440_arm=nomat-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 7%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **77%** (min 50%) · 2624 MHz (min 2610) · memory 8101 MHz · 79.5 W · 76 °C (max 81) · VRAM 6740 MiB · throttled 100% of samples

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
- `render.depth_only.skip_material=on (command line: --set)`
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
