# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5925 frames over 65.6 s · **median 10.84 ms (92 fps)** · mean 11.08 · p95 13.58 · p99 16.82 (1% low 59 fps) · worst 45.29 ms · 71 spikes over 1.5× median · 61.2 stutters/min (work 52.1/min)
- warm-up: 322 frames over 9.5 s, worst 2549.4 ms
- hitches (> 27.1 ms): **33** measured, 55 in warm-up
- timed GPU passes: 6.57 ms of a 11.08 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.84 ms, limit 11 ms
- **FAIL** frame_ms: 67 stutters (> 1.5x moving median of 60) in 1.09 min = 61.24/min, limit 1/min
- **FAIL** worst frame 45.29 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.867 | 2.034 | 2.066 | 0.92 | 0.174 |
| main_opaque_pass_3d | 1.488 | 1.622 | 2.040 | 0.92 | 0.067 |
| early prepass | 0.455 | 0.496 | 0.734 | 0.92 | 0.087 |
| bloom | 0.288 | 0.314 | 0.319 | 0.92 | 0.063 |
| atmosphere_luts | 0.279 | 0.304 | 0.365 | 0.92 | 0.003 |
| water_velocity | 0.257 | 0.280 | 0.347 | 0.92 | 0.002 |
| water_sspr | 0.185 | 0.202 | 0.203 | 0.92 | 0.002 |
| ssao | 0.173 | 0.188 | 0.191 | 0.92 | 0.007 |
| atmos_froxel_inject | 0.134 | 0.146 | 0.148 | 0.92 | 0.001 |
| clouds_trace | 0.129 | 0.140 | 0.232 | 0.92 | 0.001 |
| auxiliary_views | 0.126 | 0.137 | 0.508 | 0.92 | 0.138 |
| water_fft | 0.122 | 0.132 | 0.139 | 0.92 | 0.003 |
| uniform_allocation | 0.119 | 0.129 | 0.183 | 0.92 | 0.026 |
| upscaling | 0.109 | 0.119 | 0.126 | 0.92 | 0.004 |
| clouds_composite | 0.097 | 0.106 | 0.109 | 0.92 | 0.002 |
| bin_unpacking | 0.095 | 0.103 | 0.141 | 0.92 | 0.019 |
| main_transmissive_pass_3d | 0.079 | 0.086 | 0.180 | 0.92 | 0.008 |
| tonemapping | 0.078 | 0.085 | 0.088 | 0.92 | 0.003 |
| clustering | 0.058 | 0.064 | 0.067 | 0.92 | 0.019 |
| render_sky | 0.047 | 0.051 | 0.052 | 0.92 | 0.002 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.92 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.92 | 0.001 |
| surface_trail | 0.037 | 0.056 | 0.057 | 0.66 | 0.003 |
| surface_exposure_far | 0.035 | 0.042 | 0.045 | 0.82 | 0.002 |
| lightprobe_irradiance_map | 0.029 | 0.412 | 0.416 | 0.07 | 0.001 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.92 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.047 | 0.92 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.031 | 0.92 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.92 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.92 | 0.002 |
| lightprobe_radiance_map | 0.015 | 0.219 | 0.230 | 0.07 | 0.004 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.92 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.026 | 0.92 | 0.127 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.92 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.92 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.92 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.91 | 0.003 |
| lightprobe_copy | 0.001 | 0.012 | 0.013 | 0.07 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.07 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.07 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.549 | 11.630 | 39.14 | 56577.1 |
| main/busy/ms | 9.222 | 10.909 | 25.75 | 54637.5 |
| render/busy/ms | 8.989 | 11.134 | 38.58 | 53262.7 |
| stage/main/post_update | 5.745 | 7.124 | 9.68 | 34040.4 |
| stage/render/render | 5.577 | 6.996 | 17.42 | 33041.7 |
| render/graph/ms | 4.826 | 6.243 | 16.16 | 28593.1 |
| encode//sum | 4.463 | 5.937 | 28.35 | 26441.0 |
| stage/render/graph_submit | 3.642 | 4.795 | 11.88 | 21577.0 |
| encode/submit | 2.301 | 3.188 | 7.56 | 13634.8 |
| stage/main/update | 2.196 | 2.630 | 19.08 | 13013.1 |
| encode/finish_wall | 1.417 | 2.071 | 8.26 | 8396.7 |
| stage/render/camera_driver | 1.131 | 1.544 | 4.52 | 6699.4 |
| encode/pass:shadow_directional_light/sum | 0.874 | 1.596 | 3.80 | 5179.2 |
| encode/pass:shadow_point_light/sum | 0.782 | 3.022 | 17.87 | 4635.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.779 | 1.113 | 1.86 | 4617.4 |
| stage/render/prepare_write_phase_buffers | 0.774 | 1.107 | 2.61 | 4583.4 |
| encode/pass:early prepass/sum | 0.753 | 1.086 | 1.79 | 4460.7 |
| stage/render/prepare_assets | 0.577 | 0.739 | 11.57 | 3416.1 |
| bigspace/total/ms | 0.564 | 1.378 | 2.38 | 3342.0 |
| render/present/ms | 0.464 | 1.084 | 2.73 | 2750.7 |
| stage/render/prepare_resources | 0.464 | 0.550 | 1.17 | 2747.5 |
| stage/render/prepare_meshes | 0.439 | 0.554 | 5.24 | 2599.3 |
| stage/main/first | 0.399 | 0.521 | 5.93 | 2365.8 |
| stage/main/pre_update | 0.363 | 0.532 | 1.54 | 2148.8 |
| stage/render/specialize | 0.331 | 0.425 | 13.04 | 1960.8 |
| terrain/update/ms | 0.320 | 0.424 | 2.43 | 1895.4 |
| stage/render/queue | 0.300 | 0.428 | 7.84 | 1775.9 |
| terrain/select/ms | 0.286 | 0.383 | 0.99 | 1693.8 |
| stage/render/prepare_bind_groups | 0.282 | 0.490 | 3.39 | 1668.1 |
| controller/feed/ms | 0.269 | 0.334 | 0.89 | 1594.7 |
| stream/drive/ms | 0.254 | 0.338 | 16.88 | 1507.1 |
| encode/pass:main_transparent_pass_3d/sum | 0.253 | 0.373 | 0.91 | 1498.5 |
| bigspace/low_precision/ms | 0.230 | 0.856 | 1.96 | 1361.1 |
| stage/main/fixed_main_loop | 0.221 | 0.376 | 1.07 | 1308.4 |
| far/select/ms | 0.204 | 0.280 | 0.59 | 1210.5 |
| stage/render/create_views | 0.199 | 0.256 | 1.00 | 1177.3 |
| stream/tick/ms | 0.197 | 0.270 | 0.55 | 1164.5 |
| stage/render/prepare_batch_phases | 0.193 | 0.276 | 0.65 | 1140.7 |
| stage/render/extract_commands | 0.171 | 0.201 | 0.43 | 1015.3 |
| bigspace/high_precision/ms | 0.168 | 0.779 | 1.88 | 994.8 |
| anim/evaluate/ms | 0.166 | 0.277 | 1.59 | 986.1 |
| stage/render/prepare_flush | 0.164 | 0.342 | 1.46 | 974.7 |
| anim/evaluate_cpu/ms | 0.154 | 0.193 | 0.78 | 914.7 |
| controller/move/ms | 0.141 | 0.291 | 0.75 | 837.1 |
| stage/render/prepare_views | 0.139 | 0.173 | 0.59 | 824.3 |
| stage/main/last | 0.119 | 0.158 | 3.18 | 705.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.109 | 0.186 | 0.70 | 642.9 |
| render/acquire/ms | 0.095 | 0.121 | 0.55 | 563.8 |
| stage/render/prepare_collect_phase_buffers | 0.067 | 0.137 | 0.55 | 396.2 |
| pipelines/compile_ms | 0.050 | 0.000 | 122.13 | 298.3 |
| pipelines/front_ms | 0.048 | 0.000 | 118.01 | 285.6 |
| physics/step/ms | 0.048 | 0.088 | 0.59 | 283.0 |
| physics/insert/ms | 0.036 | 0.377 | 1.02 | 210.5 |
| stage/render/phase_sort | 0.035 | 0.061 | 0.42 | 206.7 |
| stage/render/cleanup | 0.033 | 0.104 | 0.79 | 195.1 |
| encode/resolve_diagnostics/sum | 0.031 | 0.041 | 0.78 | 183.5 |
| weather/map/ms | 0.025 | 0.033 | 0.32 | 147.7 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.52 | 87.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.53 | 67.3 |
| anim/apply/ms | 0.011 | 0.020 | 0.04 | 67.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.32 | 62.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.21 | 61.5 |
| script/update/ms | 0.007 | 0.009 | 0.54 | 41.9 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 31.4 |
| stream/despawn/ms | 0.005 | 0.000 | 1.07 | 31.1 |
| stream/spawn/ms | 0.005 | 0.000 | 1.14 | 29.5 |
| script/ui/ms | 0.004 | 0.006 | 0.39 | 26.4 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 19.9 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.70 | 14.3 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.16 | 12.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.07 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.07 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1296 | 2262 | 547 |
| crowd/c0/meshes/count | count | 353 | 646 | 140 |
| crowd/c1/count | count | 147 | 158 | 148 |
| crowd/c1/joints/count | count | 5531 | 6045 | 5617 |
| crowd/c1/meshes/count | count | 1193 | 1313 | 1193 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4614 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4596 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.44 | 7.35 | 0.39 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.52 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.53 | 0.01 |
| encode/finish_critical | ms | 0.99 | 7.35 | 1.01 |
| encode/pass:early prepass/max | ms | 0.77 | 1.79 | 0.55 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.79 | 1.86 | 1.01 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.70 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.24 | 0.91 | 0.28 |
| encode/pass:shadow_directional_light/max | ms | 0.82 | 3.80 | 0.49 |
| encode/pass:shadow_point_light/max | ms | 0.79 | 1.47 | 1.08 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.78 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.48 | 38.58 | 8.43 |
| gpu/timed/ms | ms | 7.89 | 11.61 | 8.27 |
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
| phases/alpha_mask3d/items | count | 814 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 814 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1284 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3412 | 5092 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1284 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3412 | 5092 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 17 | 226 | 4 |
| phases/shadow/bins | count | 5024 | 8709 | 2413 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.2 k | 6360 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 204 | 242 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 186 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 329 | 330 | 330 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 12 | 615 | 6 |
| preprocess/bind_groups_created | count | 12 | 615 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 223 | 123 |
| process/cpu_usage | % | 26.92 | 38.94 | 26.57 |
| process/mem_usage | GiB | 5.58 | 5.75 | 5.75 |
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
| skin/blocks_changed/count | count | 13 | 123 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 368.6 k | 811.0 k | 489.6 k |
| skin/joints_written/count | count | 5760 | 12.7 k | 7650 |
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
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.38 | 50.72 | 31.76 |
| system/mem_usage | % | 55.31 | 55.99 | 55.98 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 67 | 275 | 54 |
| views/all/bins | count | 7562 | 11.6 k | 5911 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.2 k | 14.0 k |
| views/all/pipelines | count | 251 | 289 | 75 |
| views/all/pipelines_distinct | count | 70 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 33.8 k | 8119 |
| views/cascade/batch_sets | count | 9 | 51 | 4 |
| views/cascade/bins | count | 2796 | 6150 | 2413 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8911 | 23.5 k | 6360 |
| views/cascade/pipelines | count | 36 | 52 | 29 |
| views/cascade/work_items | count | 409 | 22.1 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 188 | 0 |
| views/lamp_face/bins | count | 2179 | 2703 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2404 | 3002 | 0 |
| views/lamp_face/pipelines | count | 169 | 203 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2861 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2864 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8434 | 12.9 k | 7610 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8601 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 54.99 | 302 | 325.8 k |
| anim/events/count | count | 0.09 | 3 | 545 |
| assets/Image/added | count | 0.10 | 34 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.69 | 152 | 4061 |
| assets/Mesh/removed | count | 0.82 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.66 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.79 | 1386 | 4676 |
| encode//count | count | 33.05 | 222 | 195.8 k |
| encode/bind_walk/init_textures | count | 12.3 k | 119.2 k | 72.66 M |
| encode/bind_walk/init_textures_skipped | count | 63.5 k | 138.7 k | 376.25 M |
| encode/bind_walk/inits | count | 456 | 1213 | 2.70 M |
| encode/bind_walk/inits_skipped | count | 107 | 163 | 636.8 k |
| encode/bind_walk/merge_textures | count | 12.2 k | 119.1 k | 72.23 M |
| encode/bind_walk/merge_textures_skipped | count | 5368 | 6012 | 31.81 M |
| encode/bind_walk/merges | count | 98.00 | 350 | 580.6 k |
| encode/bind_walk/merges_skipped | count | 83.99 | 129 | 497.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5925 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5925 |
| encode/pass:early prepass/count | count | 1 | 1 | 5925 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5925 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5925 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5925 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5925 |
| encode/pass:shadow_point_light/count | count | 1.27 | 48 | 7543 |
| encode/queue/command_buffers | count | 44.34 | 280 | 262.7 k |
| encode/queue/submits | count | 2.02 | 4 | 11.9 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.88 M | 11.16 G |
| encode/queue/write_buffer/calls | count | 257 | 918 | 1.52 M |
| encode/queue/write_buffer_with/bytes | bytes | 179.5 k | 3.52 M | 1.06 G |
| encode/queue/write_buffer_with/calls | count | 11.04 | 465 | 65.4 k |
| encode/queue/write_texture/bytes | bytes | 12.1 k | 8.59 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5925 |
| physics/inserts | count | 0.38 | 110 | 2249 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 2 | 3939 |
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
| stream/upload/bytes | bytes | 5713 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1253 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.1 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2549.4 | main/busy/ms 2519, stage/main/post_update 2414, stage/main/update 96.54, surface/advance_cpu_ms 67.30, stage/main/pre_update 4.65, stage/main/last 1.06, surface/cell_upload_ms 0.76, stream/drive/ms 0.60, stage/main/first 0.53, stage/main/fixed_main_loop 0.35, weather/map/ms 0.16, stream/tick/ms 0.12 |
| warm-up | 3.31 | 757.7 | main/busy/ms 2663, stage/main/post_update 2441, render/thread/ms 725, render/busy/ms 548, stage/render/prepare_assets 499, stage/main/update 207, stage/render/prepare_views 172, render/acquire/ms 172, surface/advance_cpu_ms 67.30, stage/render/create_views 28.84, stage/render/render 14.82, render/graph/ms 8.42 |
| warm-up | 3.48 | 171.7 | render/thread/ms 881, render/busy/ms 613, stage/render/prepare_assets 550, stage/render/prepare_views 173, render/acquire/ms 172, main/busy/ms 172, pipelines/compile_ms 118, stage/main/update 118, stage/render/render 110, render/present/ms 95.22, pipelines/driver_ms 74.88, pipelines/front_ms 43.35 |
| warm-up | 3.72 | 212.9 | pipelines/wait_ms 4004, pipelines/compile_ms 2342, pipelines/front_ms 2104, pipelines/driver_ms 238, main/busy/ms 161, render/thread/ms 146, render/busy/ms 143, stage/render/prepare_assets 120, stage/main/post_update 87.38, stage/main/update 53.83, terrain/update/ms 18.54, stage/render/render 14.18 |
| warm-up | 3.82 | 97.5 | pipelines/wait_ms 4203, pipelines/compile_ms 3294, pipelines/front_ms 2934, encode//sum 375, pipelines/driver_ms 360, render/thread/ms 224, render/busy/ms 222, main/busy/ms 150, stage/render/prepare_assets 138, stage/main/post_update 113, stage/render/render 72.75, render/graph/ms 70.14 |
| warm-up | 3.89 | 77.9 | pipelines/compile_ms 1666, pipelines/front_ms 1401, encode//sum 374, pipelines/driver_ms 265, pipelines/wait_ms 219, render/thread/ms 171, render/busy/ms 168, main/busy/ms 115, stage/main/post_update 88.85, stage/render/prepare_assets 85.68, stage/render/render 71.71, render/graph/ms 69.39 |
| warm-up | 3.94 | 46.3 | pipelines/compile_ms 732, pipelines/front_ms 696, render/thread/ms 121, render/busy/ms 91.57, stage/render/prepare_assets 67.83, main/busy/ms 50.55, pipelines/driver_ms 36.15, stage/main/post_update 25.01, pipelines/wait_ms 22.71, stage/render/render 20.87, stage/main/update 20.55, stage/render/prepare_views 17.51 |
| warm-up | 3.97 | 30.8 | pipelines/compile_ms 1378, pipelines/front_ms 1337, render/thread/ms 74.99, pipelines/driver_ms 41.55, main/busy/ms 39.54, render/busy/ms 32.28, stage/render/prepare_views 31.35, render/acquire/ms 31.15, pipelines/wait_ms 24.80, stage/render/render 21.72, stage/main/update 19.76, encode//sum 17.96 |
| warm-up | 4.01 | 40.7 | pipelines/compile_ms 1114, pipelines/front_ms 1082, render/thread/ms 68.72, render/busy/ms 53.58, stage/render/prepare_assets 35.21, main/busy/ms 35.16, pipelines/driver_ms 32.91, stage/main/post_update 20.93, stage/render/prepare_views 14.25, render/acquire/ms 14.14, stage/render/render 10.71, render/graph/ms 9.19 |
| warm-up | 4.13 | 88.6 | pipelines/compile_ms 449, pipelines/front_ms 422, render/thread/ms 72.32, main/busy/ms 68.65, render/busy/ms 65.83, stage/main/update 48.39, stream/drive/ms 40.70, stage/render/prepare_assets 38.10, pipelines/driver_ms 27.01, stage/render/render 22.22, render/graph/ms 19.38, stage/render/graph_submit 14.00 |
| warm-up | 4.19 | 33.4 | pipelines/compile_ms 461, pipelines/front_ms 442, render/thread/ms 45.79, render/busy/ms 43.77, main/busy/ms 32.60, stage/render/prepare_assets 20.77, pipelines/driver_ms 18.73, stage/render/render 16.11, stage/main/update 15.00, render/graph/ms 13.88, stage/render/graph_submit 10.00, pipelines/wait_ms 8.61 |
| warm-up | 4.25 | 36.6 | pipelines/compile_ms 647, pipelines/front_ms 623, render/thread/ms 56.98, render/busy/ms 55.27, main/busy/ms 38.93, stage/render/prepare_assets 28.73, pipelines/driver_ms 23.30, stage/main/update 16.20, stage/main/post_update 15.87, stage/render/render 11.58, render/graph/ms 9.10, stage/render/create_views 6.52 |
| warm-up | 4.31 | 57.9 | pipelines/compile_ms 103, pipelines/front_ms 98.16, render/thread/ms 73.35, render/busy/ms 70.74, main/busy/ms 57.78, stage/render/prepare_assets 38.61, stage/main/update 37.74, stage/main/post_update 13.39, stage/render/render 11.84, render/graph/ms 9.04, stage/render/prepare_resources 8.23, stage/render/graph_submit 6.25 |
| warm-up | 4.37 | 64.4 | pipelines/compile_ms 919, pipelines/front_ms 889, render/thread/ms 84.45, render/busy/ms 81.75, main/busy/ms 59.08, stage/render/prepare_assets 43.29, stage/main/update 37.57, pipelines/driver_ms 29.86, stage/main/post_update 13.93, stage/render/render 12.51, render/graph/ms 9.49, stage/render/prepare_resources 8.26 |
| warm-up | 4.40 | 30.5 | pipelines/compile_ms 1353, pipelines/front_ms 1311, render/thread/ms 70.49, render/busy/ms 68.10, main/busy/ms 67.25, pipelines/driver_ms 41.81, stage/render/prepare_assets 28.60, stage/main/post_update 28.20, stage/main/update 27.46, stage/render/render 20.79, pipelines/wait_ms 18.66, render/graph/ms 17.90 |
| warm-up | 4.50 | 97.6 | pipelines/compile_ms 1359, pipelines/front_ms 1308, pipelines/wait_ms 430, render/thread/ms 97.75, render/busy/ms 95.52, main/busy/ms 69.43, pipelines/driver_ms 51.18, stage/render/prepare_assets 34.25, stage/main/post_update 33.27, stage/main/update 26.57, stage/render/render 22.71, render/graph/ms 19.95 |
| warm-up | 4.58 | 79.8 | pipelines/compile_ms 1738, pipelines/front_ms 1679, pipelines/wait_ms 441, render/thread/ms 143, render/busy/ms 141, main/busy/ms 69.95, pipelines/driver_ms 58.43, encode//sum 50.49, stage/render/prepare_assets 43.11, stage/main/update 33.63, stage/render/render 25.23, stage/render/prepare_resources 22.56 |
| warm-up | 4.66 | 81.2 | pipelines/compile_ms 2080, pipelines/front_ms 2019, pipelines/wait_ms 466, render/thread/ms 139, render/busy/ms 136, main/busy/ms 105, pipelines/driver_ms 61.16, stage/main/update 55.78, encode//sum 50.97, stage/render/render 37.59, stage/render/prepare_assets 36.87, stream/drive/ms 36.80 |
| warm-up | 4.72 | 56.6 | pipelines/compile_ms 2130, pipelines/front_ms 2065, pipelines/wait_ms 862, render/thread/ms 112, render/busy/ms 111, main/busy/ms 98.53, pipelines/driver_ms 64.82, stage/main/update 52.77, stage/render/prepare_assets 39.62, stage/main/post_update 30.15, stage/render/render 29.02, render/graph/ms 23.96 |
| warm-up | 4.81 | 88.5 | pipelines/compile_ms 1621, pipelines/front_ms 1575, pipelines/wait_ms 558, main/busy/ms 120, render/thread/ms 104, render/busy/ms 102, stage/main/update 81.16, stage/render/prepare_assets 49.90, pipelines/driver_ms 46.39, stage/render/render 22.09, render/graph/ms 19.53, stream/drive/ms 19.47 |
| warm-up | 4.90 | 87.6 | pipelines/compile_ms 2316, pipelines/front_ms 2238, pipelines/wait_ms 400, main/busy/ms 136, render/thread/ms 116, render/busy/ms 114, stage/main/update 109, pipelines/driver_ms 77.40, stage/render/prepare_assets 64.23, stream/drive/ms 53.57, stage/render/render 31.93, render/graph/ms 29.01 |
| warm-up | 4.94 | 46.7 | pipelines/compile_ms 1657, pipelines/front_ms 1598, pipelines/wait_ms 266, main/busy/ms 139, stage/main/update 115, render/thread/ms 92.28, render/busy/ms 90.68, stream/drive/ms 86.86, pipelines/driver_ms 58.82, stage/render/prepare_assets 39.00, stage/render/render 26.86, render/graph/ms 24.17 |
| warm-up | 5.04 | 102.1 | pipelines/compile_ms 1815, pipelines/front_ms 1758, main/busy/ms 101, render/thread/ms 80.85, render/busy/ms 79.03, stage/main/update 74.26, pipelines/driver_ms 57.49, stream/drive/ms 54.76, stage/render/prepare_assets 32.59, stage/render/render 21.16, pipelines/wait_ms 19.92, render/graph/ms 18.09 |
| warm-up | 5.08 | 39.7 | pipelines/compile_ms 1873, pipelines/front_ms 1812, main/busy/ms 89.53, render/thread/ms 77.42, render/busy/ms 75.27, stage/main/update 68.49, pipelines/driver_ms 60.25, stream/drive/ms 53.77, stage/render/prepare_assets 33.06, pipelines/wait_ms 21.90, stage/render/render 20.16, render/graph/ms 17.17 |
| warm-up | 5.20 | 119.3 | pipelines/compile_ms 763, pipelines/front_ms 739, main/busy/ms 89.08, render/thread/ms 81.16, render/busy/ms 79.31, stage/main/update 67.67, stage/render/prepare_assets 52.19, stream/drive/ms 52.18, pipelines/driver_ms 23.65, stage/render/render 12.69, stage/main/post_update 11.96, render/graph/ms 10.37 |
| warm-up | 5.27 | 71.5 | pipelines/compile_ms 1601, pipelines/front_ms 1559, render/thread/ms 116, render/busy/ms 113, main/busy/ms 92.12, stage/render/prepare_assets 71.95, stage/main/update 47.23, pipelines/driver_ms 41.90, stage/main/post_update 35.89, stream/drive/ms 28.09, stage/render/render 28.01, render/graph/ms 24.17 |
| warm-up | 5.32 | 41.4 | pipelines/compile_ms 991, pipelines/front_ms 964, render/thread/ms 92.74, main/busy/ms 89.80, render/busy/ms 89.46, stage/main/update 46.24, stage/render/prepare_assets 45.44, stage/main/post_update 35.12, stream/drive/ms 28.38, stage/render/render 27.66, pipelines/driver_ms 26.44, render/graph/ms 24.08 |
| warm-up | 5.37 | 52.8 | pipelines/compile_ms 123, pipelines/front_ms 116, render/thread/ms 76.17, render/busy/ms 74.65, main/busy/ms 49.97, stage/main/update 25.48, stage/render/prepare_assets 21.47, stage/main/post_update 17.18, stage/render/prepare_resources 13.55, stage/render/render 11.52, encode//sum 9.68, render/graph/ms 9.61 |
| warm-up | 5.44 | 76.2 | render/thread/ms 115, render/busy/ms 114, pipelines/compile_ms 59.89, pipelines/front_ms 56.54, main/busy/ms 46.01, stage/render/prepare_assets 26.71, stage/render/prepare_write_phase_buffers 26.41, stage/main/update 23.40, stage/render/prepare_resources 16.79, stage/main/post_update 14.68, stage/render/prepare_flush 12.70, stage/render/render 11.47 |
| warm-up | 5.49 | 42.7 | pipelines/compile_ms 2204, pipelines/front_ms 2147, render/thread/ms 95.17, render/busy/ms 93.78, pipelines/driver_ms 56.96, main/busy/ms 39.63, stage/render/prepare_assets 27.95, stage/render/prepare_write_phase_buffers 25.34, stage/main/update 18.37, pipelines/wait_ms 15.35, pipelines/lock_ms 14.80, stage/render/render 12.59 |
| warm-up | 5.60 | 50.0 | pipelines/compile_ms 605, pipelines/front_ms 585, render/thread/ms 62.62, render/busy/ms 61.04, pipelines/wait_ms 43.95, main/busy/ms 36.88, stage/render/prepare_assets 29.44, pipelines/driver_ms 19.73, stage/main/update 18.66, stage/render/render 18.62, render/graph/ms 16.53, stage/render/graph_submit 13.82 |
| warm-up | 5.68 | 34.0 | pipelines/compile_ms 403, pipelines/front_ms 390, pipelines/wait_ms 86.63, render/thread/ms 51.98, render/busy/ms 50.67, main/busy/ms 38.42, stage/main/update 18.69, stage/render/prepare_assets 15.53, stage/main/post_update 14.62, stage/render/prepare_resources 13.04, pipelines/driver_ms 12.61, stage/render/render 11.92 |
| warm-up | 5.73 | 30.1 | pipelines/compile_ms 196, pipelines/front_ms 191, render/thread/ms 50.30, render/busy/ms 48.65, main/busy/ms 34.88, stage/main/post_update 18.72, stage/render/render 13.75, stage/render/prepare_assets 12.16, render/graph/ms 11.64, stage/main/update 10.20, encode//sum 8.50, stage/render/graph_submit 7.78 |
| warm-up | 5.84 | 35.5 | pipelines/compile_ms 539, pipelines/front_ms 527, render/thread/ms 40.04, render/busy/ms 38.40, main/busy/ms 37.32, stage/main/update 19.03, stage/render/prepare_assets 18.17, stage/main/post_update 12.52, pipelines/driver_ms 12.10, stage/render/render 11.47, stream/drive/ms 9.73, render/graph/ms 9.35 |
| warm-up | 5.90 | 34.2 | render/thread/ms 54.48, render/busy/ms 52.36, main/busy/ms 26.18, stage/render/render 21.35, stage/render/prepare_assets 19.77, render/graph/ms 18.82, stage/render/graph_submit 15.34, stage/main/post_update 11.26, stage/main/update 8.71, encode//sum 7.93, encode/finish_wall 7.33, encode/submit 6.28 |
| warm-up | 5.95 | 45.6 | render/thread/ms 64.42, render/busy/ms 62.40, pipelines/compile_ms 60.94, pipelines/front_ms 58.44, stage/render/prepare_assets 35.80, main/busy/ms 35.77, stage/render/render 17.83, render/graph/ms 15.38, stage/main/post_update 12.59, stage/render/graph_submit 12.29, stage/main/first 8.61, stage/main/update 8.47 |
| warm-up | 6.01 | 31.7 | pipelines/compile_ms 566, pipelines/front_ms 542, render/thread/ms 54.58, render/busy/ms 53.17, main/busy/ms 33.48, stage/render/prepare_assets 29.11, pipelines/driver_ms 23.88, stage/main/update 16.60, stage/render/render 16.23, render/graph/ms 13.88, stage/main/post_update 11.22, stage/render/graph_submit 10.35 |
| warm-up | 6.52 | 30.8 | render/thread/ms 37.41, render/busy/ms 36.42, main/busy/ms 28.01, stage/render/prepare_assets 16.50, stage/main/update 11.25, stage/main/post_update 10.38, stage/render/render 7.84, render/graph/ms 6.47, encode//sum 6.26, stage/render/graph_submit 4.26, stream/drive/ms 3.75, stage/render/specialize 3.39 |
| warm-up | 6.85 | 36.4 | main/busy/ms 43.26, render/thread/ms 33.74, render/busy/ms 32.51, stage/main/update 23.22, stage/main/post_update 15.11, stream/drive/ms 14.52, stage/render/render 13.54, render/graph/ms 11.58, encode//sum 9.20, stage/render/prepare_assets 7.27, stage/render/graph_submit 6.92, encode/pass:shadow_point_light/sum 6.18 |
| warm-up | 7.26 | 36.8 | main/busy/ms 43.60, render/thread/ms 29.58, render/busy/ms 28.48, stage/main/update 24.70, stream/drive/ms 12.99, stage/main/post_update 12.07, stage/render/prepare_assets 11.67, stage/render/render 10.71, render/graph/ms 9.10, encode//sum 8.99, stage/render/graph_submit 6.76, encode/submit 4.17 |
| warm-up | 7.35 | 31.2 | render/thread/ms 39.38, render/busy/ms 38.43, main/busy/ms 18.27, stage/render/render 12.06, render/graph/ms 10.68, stage/main/post_update 10.28, stage/render/specialize 9.53, encode//sum 8.59, stage/render/graph_submit 8.13, stage/render/queue 6.27, stage/main/update 5.29, encode/submit 4.66 |
| warm-up | 8.64 | 33.7 | render/thread/ms 42.18, render/busy/ms 41.13, main/busy/ms 19.35, stage/render/render 14.20, render/graph/ms 12.66, stage/main/post_update 11.38, stage/render/specialize 10.01, stage/render/graph_submit 9.61, encode//sum 8.10, stage/render/queue 7.52, encode/submit 5.60, encode/pass:shadow_point_light/sum 5.30 |
| measure | 0.48 | 30.1 | render/thread/ms 38.88, render/busy/ms 37.93, main/busy/ms 21.31, stage/main/post_update 14.46, stage/render/render 13.08, render/graph/ms 11.60, encode//sum 10.10, stage/render/specialize 9.94, stage/render/graph_submit 8.46, encode/pass:shadow_point_light/sum 6.55, stage/render/queue 6.52, encode/submit 5.15 |
| measure | 1.90 | 32.1 | render/thread/ms 39.55, render/busy/ms 38.49, main/busy/ms 18.77, stage/render/render 12.49, stage/main/post_update 12.11, render/graph/ms 10.88, stage/render/specialize 9.27, encode//sum 9.02, stage/render/graph_submit 8.20, stage/render/queue 6.87, encode/submit 5.03, stage/render/prepare_meshes 4.69 |
| measure | 4.92 | 33.4 | render/thread/ms 38.22, render/busy/ms 37.15, main/busy/ms 22.34, stage/main/post_update 13.29, stage/render/render 11.74, stage/render/specialize 10.35, render/graph/ms 10.26, encode//sum 8.76, stage/render/graph_submit 7.63, stage/render/queue 6.87, stage/main/update 5.70, encode/submit 4.16 |
| measure | 6.32 | 29.5 | render/thread/ms 35.51, render/busy/ms 34.37, main/busy/ms 18.77, stage/main/post_update 11.09, stage/render/render 10.98, stage/render/specialize 9.52, render/graph/ms 9.38, encode//sum 9.26, stage/render/graph_submit 7.10, stage/render/queue 6.16, stage/main/update 4.53, encode/submit 4.27 |
| measure | 7.67 | 29.7 | render/thread/ms 37.14, render/busy/ms 36.08, main/busy/ms 17.93, stage/render/render 12.48, stage/main/post_update 10.98, render/graph/ms 10.93, stage/render/specialize 9.28, encode//sum 9.25, stage/render/graph_submit 8.13, stage/render/queue 6.25, encode/pass:shadow_point_light/sum 5.42, encode/submit 5.13 |
| measure | 11.69 | 30.8 | render/thread/ms 37.20, render/busy/ms 36.14, main/busy/ms 19.03, stage/main/post_update 12.29, stage/render/render 12.14, encode//sum 11.34, render/graph/ms 10.55, stage/render/specialize 9.67, stage/render/graph_submit 7.99, stage/render/queue 6.20, encode/submit 4.68, stage/main/update 4.11 |
| measure | 15.75 | 29.5 | render/thread/ms 36.70, render/busy/ms 35.69, main/busy/ms 18.52, stage/main/post_update 11.39, stage/render/render 10.76, stage/render/specialize 9.92, render/graph/ms 9.38, encode//sum 7.85, stage/render/graph_submit 6.98, stage/render/queue 6.74, stage/main/update 4.44, encode/submit 4.04 |
| measure | 17.13 | 32.3 | render/thread/ms 41.11, render/busy/ms 40.15, main/busy/ms 19.47, stage/render/render 12.89, stage/main/post_update 12.01, render/graph/ms 11.46, encode//sum 10.68, stage/render/specialize 10.00, stage/render/graph_submit 8.38, stage/render/queue 6.95, encode/submit 5.23, encode/pass:shadow_point_light/sum 4.56 |
| measure | 18.00 | 30.4 | render/thread/ms 37.46, render/busy/ms 36.35, main/busy/ms 19.07, stage/main/post_update 11.20, stage/render/render 11.01, stage/render/specialize 9.85, render/graph/ms 9.52, encode//sum 9.47, stage/render/graph_submit 7.12, stage/render/queue 7.01, stage/main/update 5.40, encode/submit 4.45 |
| measure | 19.43 | 29.7 | render/thread/ms 36.29, render/busy/ms 35.09, main/busy/ms 19.06, stage/main/post_update 11.82, stage/render/render 10.57, stage/render/specialize 10.09, render/graph/ms 8.92, encode//sum 8.06, stage/render/graph_submit 7.07, stage/render/queue 7.05, encode/submit 4.59, stage/main/update 4.50 |
| measure | 23.01 | 33.1 | render/thread/ms 41.04, render/busy/ms 39.90, main/busy/ms 25.05, stage/main/post_update 13.37, stage/render/render 11.74, stage/render/specialize 11.54, encode//sum 10.83, render/graph/ms 10.13, stage/render/graph_submit 7.99, stage/render/queue 7.55, stage/main/update 6.28, encode/submit 4.22 |
| measure | 27.28 | 36.7 | render/thread/ms 46.66, render/busy/ms 45.53, main/busy/ms 26.44, stage/render/render 16.49, render/graph/ms 14.89, stage/main/post_update 13.47, encode//sum 12.95, stage/render/graph_submit 11.82, stage/render/specialize 11.35, stage/render/queue 8.15, encode/pass:shadow_point_light/sum 7.69, encode/submit 7.46 |
| measure | 30.48 | 32.5 | render/thread/ms 40.65, render/busy/ms 39.62, main/busy/ms 22.05, stage/main/post_update 12.43, stage/render/render 11.69, stage/render/specialize 10.82, render/graph/ms 10.25, encode//sum 8.78, stage/render/graph_submit 8.14, stage/render/queue 7.59, stage/main/update 6.30, encode/submit 4.61 |
| measure | 31.95 | 29.6 | render/thread/ms 39.49, render/busy/ms 38.46, main/busy/ms 19.87, stage/main/post_update 12.52, stage/render/render 12.15, render/graph/ms 10.73, stage/render/specialize 10.28, encode//sum 8.99, stage/render/graph_submit 8.20, stage/render/queue 7.11, encode/submit 4.68, stage/main/update 4.52 |
| measure | 35.36 | 32.4 | render/thread/ms 40.85, render/busy/ms 39.84, main/busy/ms 22.72, stage/main/post_update 13.90, stage/render/render 12.54, render/graph/ms 11.10, encode//sum 11.07, stage/render/specialize 10.36, stage/render/graph_submit 8.24, stage/render/queue 7.27, stage/main/update 5.16, encode/submit 4.91 |
| measure | 37.48 | 29.6 | render/thread/ms 38.25, render/busy/ms 37.15, main/busy/ms 18.59, stage/render/render 12.18, stage/main/post_update 11.80, encode//sum 10.75, render/graph/ms 10.70, stage/render/specialize 9.69, stage/render/graph_submit 8.17, stage/render/queue 7.05, encode/submit 5.22, encode/pass:shadow_point_light/sum 4.72 |
| measure | 40.79 | 31.7 | render/thread/ms 37.86, render/busy/ms 36.78, main/busy/ms 18.81, stage/main/post_update 10.63, stage/render/render 10.06, stage/render/specialize 9.98, render/graph/ms 8.63, encode//sum 8.51, stage/render/graph_submit 6.72, stage/render/queue 6.48, stage/main/update 4.81, encode/submit 3.85 |
| measure | 43.22 | 30.8 | render/thread/ms 36.71, render/busy/ms 35.62, main/busy/ms 18.70, stage/main/post_update 11.08, stage/render/specialize 10.08, stage/render/render 9.93, encode//sum 8.64, render/graph/ms 8.51, stage/render/graph_submit 6.77, stage/render/queue 6.63, stage/main/update 4.73, stage/render/prepare_meshes 3.82 |
| measure | 45.67 | 34.7 | render/thread/ms 40.78, render/busy/ms 39.74, main/busy/ms 18.64, stage/render/render 12.03, stage/main/post_update 10.74, render/graph/ms 10.59, stage/render/specialize 10.31, encode//sum 8.73, stage/render/graph_submit 8.63, stage/render/queue 6.99, stage/main/update 4.77, encode/submit 4.34 |
| measure | 59.04 | 45.3 | render/thread/ms 54.05, render/busy/ms 52.62, main/busy/ms 26.80, stage/render/render 16.27, stage/main/post_update 15.03, render/graph/ms 14.57, stage/render/specialize 13.24, encode//sum 12.05, stage/render/graph_submit 11.82, stage/main/update 8.04, stage/render/queue 7.53, encode/finish_wall 6.33 |
| measure | 59.10 | 32.3 | render/thread/ms 36.21, render/busy/ms 35.16, main/busy/ms 20.35, stage/render/prepare_assets 18.77, stage/main/post_update 11.19, stage/render/render 9.07, encode//sum 8.69, render/graph/ms 7.78, stage/render/graph_submit 5.88, stage/main/update 5.26, encode/submit 3.77, encode/finish_wall 2.18 |
| measure | 62.56 | 31.0 | render/thread/ms 39.11, render/busy/ms 38.07, main/busy/ms 19.38, stage/render/render 12.35, render/graph/ms 11.00, stage/main/post_update 10.54, stage/render/specialize 10.15, encode//sum 9.53, stage/render/graph_submit 8.71, stage/render/queue 6.26, stage/main/update 5.79, encode/finish_wall 4.73 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=base-5 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 2%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (250 samples): GPU load **76%** (min 44%) · 2623 MHz (min 2610) · memory 8101 MHz · 79.1 W · 76 °C (max 81) · VRAM 6808 MiB · throttled 100% of samples

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
