# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6001 frames over 65.6 s · **median 10.50 ms (95 fps)** · mean 10.94 · p95 14.71 · p99 16.38 (1% low 61 fps) · worst 29.15 ms · 125 spikes over 1.5× median · 29.2 stutters/min (work 21.9/min)
- warm-up: 323 frames over 9.4 s, worst 2565.3 ms
- hitches (> 26.2 ms): **3** measured, 48 in warm-up
- timed GPU passes: 6.68 ms of a 10.94 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.50 ms, limit 11 ms
- **FAIL** frame_ms: 32 stutters (> 1.5x moving median of 60) in 1.09 min = 29.25/min, limit 1/min
- pass worst frame 29.15 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.009 | 2.030 | 2.064 | 0.99 | 0.175 |
| main_opaque_pass_3d | 1.291 | 1.304 | 1.640 | 0.99 | 0.067 |
| early prepass | 0.484 | 0.489 | 0.701 | 0.99 | 0.086 |
| bloom | 0.313 | 0.316 | 0.322 | 0.99 | 0.063 |
| atmosphere_luts | 0.306 | 0.309 | 0.371 | 0.99 | 0.003 |
| auxiliary_views | 0.260 | 0.263 | 1.119 | 0.99 | 0.243 |
| water_velocity | 0.259 | 0.262 | 0.349 | 0.99 | 0.002 |
| water_sspr | 0.199 | 0.201 | 0.203 | 0.99 | 0.002 |
| ssao | 0.186 | 0.188 | 0.191 | 0.99 | 0.007 |
| upscaling | 0.128 | 0.130 | 0.136 | 0.99 | 0.004 |
| water_fft | 0.128 | 0.129 | 0.137 | 0.99 | 0.003 |
| atmos_froxel_inject | 0.126 | 0.127 | 0.128 | 0.99 | 0.001 |
| uniform_allocation | 0.124 | 0.125 | 0.152 | 0.99 | 0.025 |
| clouds_composite | 0.104 | 0.105 | 0.108 | 0.99 | 0.002 |
| bin_unpacking | 0.096 | 0.097 | 0.118 | 0.99 | 0.019 |
| tonemapping | 0.086 | 0.087 | 0.091 | 0.99 | 0.003 |
| main_transmissive_pass_3d | 0.077 | 0.078 | 0.154 | 0.99 | 0.009 |
| clustering | 0.063 | 0.064 | 0.067 | 0.99 | 0.019 |
| clouds_trace | 0.055 | 0.056 | 0.073 | 0.99 | 0.001 |
| render_sky | 0.051 | 0.051 | 0.052 | 0.99 | 0.002 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 0.99 | 0.001 |
| lightprobe_irradiance_map | 0.048 | 0.411 | 0.415 | 0.12 | 0.001 |
| surface_trail | 0.039 | 0.056 | 0.057 | 0.70 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.028 | 0.028 | 0.036 | 0.99 | 0.005 |
| lightprobe_radiance_map | 0.026 | 0.218 | 0.228 | 0.12 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.029 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.035 | 0.99 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.013 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.024 | 0.99 | 0.124 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.99 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 0.99 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.550 | 13.214 | 24.26 | 57309.8 |
| main/busy/ms | 9.064 | 11.009 | 25.66 | 54392.4 |
| render/busy/ms | 9.038 | 12.662 | 23.63 | 54235.3 |
| stage/render/render | 5.758 | 8.628 | 18.77 | 34551.6 |
| stage/main/post_update | 5.639 | 7.226 | 10.75 | 33841.8 |
| render/graph/ms | 5.063 | 7.839 | 18.05 | 30383.4 |
| encode//sum | 4.216 | 6.412 | 29.24 | 25301.5 |
| stage/render/graph_submit | 3.779 | 5.769 | 16.23 | 22676.4 |
| encode/submit | 2.435 | 3.943 | 11.62 | 14612.5 |
| stage/main/update | 2.168 | 2.634 | 19.25 | 13011.6 |
| encode/finish_wall | 1.378 | 1.914 | 12.43 | 8271.9 |
| encode/pass:shadow_point_light/sum | 1.357 | 5.348 | 16.69 | 8144.1 |
| stage/render/camera_driver | 1.230 | 2.063 | 3.89 | 7383.6 |
| encode/pass:shadow_directional_light/sum | 0.768 | 1.251 | 2.19 | 4610.0 |
| encode/pass:main_opaque_pass_3d/sum | 0.762 | 1.095 | 1.91 | 4573.4 |
| stage/render/prepare_write_phase_buffers | 0.761 | 1.120 | 2.58 | 4568.4 |
| encode/pass:early prepass/sum | 0.744 | 1.087 | 11.93 | 4466.8 |
| stage/render/prepare_assets | 0.585 | 0.731 | 8.39 | 3508.7 |
| bigspace/total/ms | 0.561 | 1.374 | 2.19 | 3368.4 |
| stage/render/prepare_resources | 0.457 | 0.544 | 2.06 | 2745.0 |
| stage/render/prepare_meshes | 0.422 | 0.536 | 2.57 | 2531.2 |
| render/present/ms | 0.417 | 0.511 | 1.98 | 2504.3 |
| stage/main/first | 0.393 | 0.513 | 5.66 | 2356.1 |
| stage/main/pre_update | 0.356 | 0.500 | 1.11 | 2138.7 |
| terrain/update/ms | 0.313 | 0.407 | 2.28 | 1879.5 |
| terrain/select/ms | 0.280 | 0.366 | 1.04 | 1677.7 |
| controller/feed/ms | 0.266 | 0.333 | 0.84 | 1598.2 |
| stage/render/prepare_bind_groups | 0.261 | 0.445 | 2.27 | 1564.8 |
| stage/render/specialize | 0.257 | 0.385 | 2.48 | 1544.9 |
| stream/drive/ms | 0.249 | 0.333 | 17.19 | 1493.3 |
| stage/render/queue | 0.243 | 0.383 | 2.79 | 1458.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.240 | 0.358 | 1.13 | 1441.7 |
| bigspace/low_precision/ms | 0.229 | 0.734 | 1.79 | 1376.6 |
| stage/main/fixed_main_loop | 0.216 | 0.362 | 1.06 | 1294.1 |
| far/select/ms | 0.205 | 0.276 | 0.67 | 1227.7 |
| stage/render/create_views | 0.194 | 0.252 | 0.91 | 1162.2 |
| stream/tick/ms | 0.192 | 0.260 | 0.64 | 1149.4 |
| stage/render/prepare_batch_phases | 0.190 | 0.276 | 0.92 | 1137.4 |
| stage/render/extract_commands | 0.173 | 0.201 | 0.53 | 1036.4 |
| anim/evaluate/ms | 0.168 | 0.316 | 1.62 | 1005.8 |
| bigspace/high_precision/ms | 0.165 | 0.700 | 1.69 | 990.9 |
| stage/render/prepare_flush | 0.165 | 0.341 | 1.35 | 990.8 |
| anim/evaluate_cpu/ms | 0.154 | 0.192 | 0.83 | 924.1 |
| controller/move/ms | 0.138 | 0.255 | 0.80 | 828.0 |
| stage/render/prepare_views | 0.138 | 0.171 | 0.49 | 826.7 |
| stage/main/last | 0.115 | 0.153 | 1.77 | 687.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.101 | 0.167 | 0.71 | 605.4 |
| render/acquire/ms | 0.095 | 0.120 | 0.37 | 570.3 |
| stage/render/prepare_collect_phase_buffers | 0.064 | 0.135 | 0.50 | 385.9 |
| physics/step/ms | 0.048 | 0.086 | 0.47 | 285.3 |
| physics/insert/ms | 0.034 | 0.378 | 0.87 | 204.8 |
| stage/render/phase_sort | 0.034 | 0.058 | 0.25 | 204.1 |
| stage/render/cleanup | 0.034 | 0.112 | 0.74 | 201.4 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 0.59 | 188.6 |
| weather/map/ms | 0.016 | 0.020 | 0.15 | 94.4 |
| pipelines/compile_ms | 0.016 | 0.000 | 82.10 | 93.9 |
| pipelines/front_ms | 0.015 | 0.000 | 79.12 | 90.7 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.41 | 87.7 |
| anim/apply/ms | 0.011 | 0.019 | 0.23 | 65.5 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.20 | 65.5 |
| controller/avoid/ms | 0.011 | 0.014 | 0.32 | 64.2 |
| weather/eval/ms | 0.010 | 0.012 | 0.25 | 57.6 |
| script/update/ms | 0.007 | 0.009 | 0.24 | 43.3 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 31.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.10 | 30.1 |
| stream/spawn/ms | 0.005 | 0.000 | 1.13 | 28.8 |
| script/ui/ms | 0.005 | 0.006 | 0.22 | 27.3 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 20.6 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 14.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.42 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.13 | 3.6 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.97 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.01 | 0.0 |
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
| crowd/c1/meshes/count | count | 1153 | 1254 | 1153 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 163 | 216 | 204 |
| crowd/joints_extracted/count | count | 3669 | 8334 | 5731 |
| crowd/joints_uploaded/count | count | 3630 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.41 | 12.38 | 0.39 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.41 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.20 | 0.01 |
| encode/finish_critical | ms | 0.96 | 12.38 | 0.93 |
| encode/pass:early prepass/max | ms | 0.77 | 11.93 | 0.93 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.78 | 1.91 | 0.68 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.71 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.13 | 0.41 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 2.19 | 0.91 |
| encode/pass:shadow_point_light/max | ms | 0.80 | 1.41 | 0.90 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.59 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.23 | 25.66 | 8.82 |
| gpu/timed/ms | ms | 7.44 | 10.80 | 7.67 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 795 | 1416 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 795 | 1416 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1290 | 1399 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3346 | 5149 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1290 | 1399 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3346 | 5149 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 197 | 4 |
| phases/shadow/bins | count | 3688 | 4382 | 1481 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6102 | 7364 | 3025 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 204 | 234 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 207 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 323 | 324 | 324 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 11 | 555 | 6 |
| preprocess/bind_groups_created | count | 11 | 555 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 133 | 299 | 127 |
| process/cpu_usage | % | 26.92 | 39.09 | 21.63 |
| process/mem_usage | GiB | 5.46 | 5.63 | 5.62 |
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
| skin/blocks_changed/count | count | 13 | 107 | 12 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 163 | 216 | 204 |
| skin/joint_bytes_written/bytes | bytes | 280.8 k | 811.3 k | 456.7 k |
| skin/joints_written/count | count | 4388 | 12.7 k | 7136 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4919 | 5011 | 5011 |
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
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.83 | 44.11 | 30.22 |
| system/mem_usage | % | 54.09 | 54.69 | 54.64 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 69 | 246 | 56 |
| views/all/bins | count | 6171 | 7310 | 4989 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.5 k | 20.2 k | 10.6 k |
| views/all/pipelines | count | 247 | 281 | 67 |
| views/all/pipelines_distinct | count | 71 | 75 | 63 |
| views/all/work_items | count | 10.1 k | 18.1 k | 8144 |
| views/cascade/batch_sets | count | 8 | 27 | 4 |
| views/cascade/bins | count | 1443 | 1558 | 1481 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3387 | 4424 | 3025 |
| views/cascade/pipelines | count | 22 | 27 | 19 |
| views/cascade/work_items | count | 324 | 4424 | 222 |
| views/lamp_face/batch_sets | count | 0 | 189 | 0 |
| views/lamp_face/bins | count | 2436 | 2956 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2694 | 3272 | 0 |
| views/lamp_face/pipelines | count | 183 | 208 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2872 | 0 |
| views/main/batch_sets | count | 51 | 53 | 52 |
| views/main/bins | count | 2886 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8280 | 13.0 k | 7614 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8444 | 13.2 k | 7922 |
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
| anim/bones_written/count | count | 46.32 | 209 | 278.0 k |
| anim/events/count | count | 0.09 | 3 | 521 |
| assets/Image/added | count | 0.10 | 28 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.68 | 152 | 4061 |
| assets/Mesh/removed | count | 0.81 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 29 | 457 |
| assets/StandardMaterial/modified | count | 3.61 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.78 | 1386 | 4676 |
| encode//count | count | 37.78 | 219 | 226.7 k |
| encode/bind_walk/init_textures | count | 14.8 k | 112.9 k | 88.95 M |
| encode/bind_walk/init_textures_skipped | count | 67.6 k | 130.2 k | 405.59 M |
| encode/bind_walk/inits | count | 469 | 1119 | 2.81 M |
| encode/bind_walk/inits_skipped | count | 109 | 162 | 656.9 k |
| encode/bind_walk/merge_textures | count | 14.8 k | 112.9 k | 88.58 M |
| encode/bind_walk/merge_textures_skipped | count | 5393 | 6012 | 32.36 M |
| encode/bind_walk/merges | count | 105 | 340 | 627.7 k |
| encode/bind_walk/merges_skipped | count | 84.32 | 128 | 506.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6001 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6001 |
| encode/pass:early prepass/count | count | 1 | 1 | 6001 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6001 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6001 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6001 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6001 |
| encode/pass:shadow_point_light/count | count | 2.70 | 48 | 16.2 k |
| encode/queue/command_buffers | count | 50.50 | 277 | 303.0 k |
| encode/queue/submits | count | 2.02 | 4 | 12.1 k |
| encode/queue/write_buffer/bytes | bytes | 1.82 M | 2.71 M | 10.91 G |
| encode/queue/write_buffer/calls | count | 265 | 829 | 1.59 M |
| encode/queue/write_buffer_with/bytes | bytes | 169.8 k | 3.53 M | 1.02 G |
| encode/queue/write_buffer_with/calls | count | 11.01 | 465 | 66.1 k |
| encode/queue/write_texture/bytes | bytes | 10.5 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6001 |
| physics/inserts | count | 0.37 | 111 | 2248 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 2 | 3939 |
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
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 19 | 378 |
| stream/upload/bytes | bytes | 5641 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1237 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.2 ms; slowest 51, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2565.3 | main/busy/ms 2535, stage/main/post_update 2492, stage/main/update 34.23, stage/main/pre_update 5.30, stage/main/last 1.06, surface/advance_cpu_ms 0.73, stage/main/first 0.55, stream/drive/ms 0.45, stage/main/fixed_main_loop 0.35, surface/cell_upload_ms 0.30, stream/tick/ms 0.12, bigspace/total/ms 0.09 |
| warm-up | 3.30 | 738.1 | main/busy/ms 2684, stage/main/post_update 2523, render/thread/ms 703, render/busy/ms 534, stage/render/prepare_assets 487, stage/render/prepare_views 166, render/acquire/ms 166, stage/main/update 147, stage/render/create_views 28.36, stage/render/render 11.83, stage/main/pre_update 7.38, render/graph/ms 7.03 |
| warm-up | 3.42 | 121.3 | render/thread/ms 807, render/busy/ms 574, stage/render/prepare_assets 513, main/busy/ms 170, stage/render/prepare_views 167, render/acquire/ms 167, pipelines/compile_ms 133, stage/main/update 116, pipelines/front_ms 82.71, stage/render/render 79.67, render/present/ms 66.52, pipelines/driver_ms 50.03 |
| warm-up | 3.49 | 62.7 | pipelines/compile_ms 486, pipelines/front_ms 385, pipelines/wait_ms 236, render/thread/ms 148, stage/render/render 106, pipelines/driver_ms 101, render/busy/ms 74.36, render/present/ms 73.02, main/busy/ms 69.13, stage/main/post_update 53.48, pipelines/lock_ms 34.85, terrain/update/ms 32.63 |
| warm-up | 3.63 | 147.2 | pipelines/wait_ms 4737, pipelines/compile_ms 2145, pipelines/front_ms 2003, render/thread/ms 178, main/busy/ms 168, render/busy/ms 166, pipelines/driver_ms 142, stage/main/update 74.13, stage/render/prepare_resources 65.51, stage/main/post_update 57.38, stage/render/render 50.99, pipelines/lock_ms 41.38 |
| warm-up | 3.70 | 65.2 | pipelines/wait_ms 4503, pipelines/compile_ms 1847, pipelines/front_ms 1753, render/thread/ms 195, render/busy/ms 193, main/busy/ms 152, pipelines/driver_ms 94.09, stage/render/prepare_assets 82.93, stage/main/update 78.67, stage/render/prepare_resources 65.12, stage/main/fixed_main_loop 37.19, stage/main/post_update 28.49 |
| warm-up | 3.73 | 29.7 | pipelines/compile_ms 737, pipelines/front_ms 699, render/thread/ms 88.89, render/busy/ms 86.71, stage/render/prepare_assets 68.41, main/busy/ms 43.77, pipelines/driver_ms 38.43, terrain/update/ms 19.42, stage/main/post_update 17.58, stage/render/render 12.35, stage/main/update 11.36, render/graph/ms 9.78 |
| warm-up | 3.76 | 30.8 | pipelines/compile_ms 1311, pipelines/front_ms 1250, pipelines/wait_ms 86.62, pipelines/driver_ms 61.11, render/thread/ms 51.54, render/busy/ms 49.79, stage/render/prepare_assets 33.31, main/busy/ms 28.86, stage/main/post_update 18.51, terrain/update/ms 9.84, stage/render/render 9.54, render/graph/ms 7.39 |
| warm-up | 3.93 | 92.3 | pipelines/compile_ms 1230, pipelines/front_ms 1192, main/busy/ms 62.70, render/thread/ms 54.93, render/busy/ms 53.05, stage/main/update 49.33, stream/drive/ms 44.29, pipelines/driver_ms 38.15, stage/render/prepare_assets 34.97, stage/render/render 13.12, render/graph/ms 10.92, encode//sum 9.05 |
| warm-up | 3.97 | 37.5 | pipelines/compile_ms 1085, pipelines/front_ms 1048, render/thread/ms 75.08, render/busy/ms 67.41, main/busy/ms 52.41, stage/main/update 38.86, pipelines/driver_ms 37.03, stage/render/prepare_assets 36.83, stage/render/render 30.89, render/graph/ms 23.13, pipelines/wait_ms 22.93, pipelines/lock_ms 22.06 |
| warm-up | 4.02 | 50.0 | pipelines/compile_ms 548, pipelines/front_ms 527, render/thread/ms 67.77, render/busy/ms 60.15, main/busy/ms 51.61, stage/main/update 38.37, pipelines/wait_ms 33.39, stage/render/render 32.66, pipelines/lock_ms 32.54, stage/render/prepare_assets 27.27, render/graph/ms 24.84, pipelines/driver_ms 21.38 |
| warm-up | 4.13 | 100.4 | pipelines/compile_ms 77.87, pipelines/front_ms 74.38, render/thread/ms 73.59, render/busy/ms 71.81, main/busy/ms 70.04, stage/main/update 34.50, stream/drive/ms 30.35, stage/render/prepare_assets 23.00, stage/render/render 15.57, stage/render/prepare_resources 14.24, stage/main/fixed_main_loop 13.96, render/graph/ms 12.83 |
| warm-up | 4.19 | 61.5 | pipelines/compile_ms 1116, pipelines/front_ms 1080, render/thread/ms 105, render/busy/ms 101, main/busy/ms 88.99, stage/render/prepare_assets 44.43, stage/main/update 44.01, pipelines/wait_ms 35.56, pipelines/driver_ms 35.49, pipelines/lock_ms 35.11, stage/render/render 22.82, render/graph/ms 17.14 |
| warm-up | 4.33 | 135.6 | pipelines/compile_ms 2152, pipelines/front_ms 2087, render/thread/ms 127, render/busy/ms 123, pipelines/wait_ms 112, main/busy/ms 78.63, pipelines/lock_ms 68.11, pipelines/driver_ms 65.08, stage/render/prepare_assets 63.73, stage/render/render 50.44, stage/main/update 48.74, render/graph/ms 45.40 |
| warm-up | 4.40 | 68.1 | pipelines/compile_ms 1761, pipelines/front_ms 1696, pipelines/wait_ms 286, render/thread/ms 148, render/busy/ms 146, stage/render/prepare_assets 77.61, pipelines/driver_ms 64.34, main/busy/ms 51.35, stage/render/render 44.34, render/graph/ms 41.95, pipelines/lock_ms 41.03, stage/render/graph_submit 38.27 |
| warm-up | 4.47 | 74.0 | pipelines/compile_ms 931, pipelines/front_ms 891, pipelines/wait_ms 377, render/thread/ms 130, render/busy/ms 128, stage/render/prepare_assets 57.82, main/busy/ms 47.48, pipelines/driver_ms 40.49, stage/render/render 29.41, render/graph/ms 26.77, stage/main/post_update 22.99, encode//sum 18.94 |
| warm-up | 4.57 | 98.2 | pipelines/compile_ms 2218, pipelines/front_ms 2150, pipelines/wait_ms 832, render/thread/ms 150, render/busy/ms 148, main/busy/ms 71.07, pipelines/driver_ms 68.07, stage/render/render 48.59, stage/render/prepare_assets 47.01, render/graph/ms 45.75, stage/main/update 33.47, stage/main/post_update 30.81 |
| warm-up | 4.68 | 109.5 | pipelines/compile_ms 3127, pipelines/front_ms 3015, pipelines/wait_ms 1631, render/thread/ms 188, render/busy/ms 186, pipelines/driver_ms 112, main/busy/ms 96.82, encode//sum 80.01, stage/render/render 55.97, stage/main/update 53.24, render/graph/ms 53.17, stage/render/prepare_assets 45.65 |
| warm-up | 4.74 | 62.4 | pipelines/compile_ms 2152, pipelines/front_ms 2071, pipelines/wait_ms 1294, render/thread/ms 148, render/busy/ms 146, pipelines/driver_ms 81.48, encode//sum 74.70, main/busy/ms 61.90, stage/render/render 43.28, render/graph/ms 40.61, stage/render/prepare_assets 38.51, stage/main/update 36.61 |
| warm-up | 4.80 | 55.7 | pipelines/compile_ms 1696, pipelines/front_ms 1649, pipelines/wait_ms 606, main/busy/ms 79.91, render/thread/ms 75.76, render/busy/ms 74.15, stage/main/update 56.53, pipelines/driver_ms 47.12, stream/drive/ms 45.56, stage/render/prepare_assets 35.62, stage/render/render 20.17, render/graph/ms 17.72 |
| warm-up | 4.87 | 74.6 | pipelines/compile_ms 1314, pipelines/front_ms 1281, pipelines/wait_ms 775, main/busy/ms 119, render/thread/ms 69.98, render/busy/ms 67.62, stage/main/update 66.73, stream/drive/ms 46.66, stage/main/post_update 39.59, pipelines/driver_ms 33.57, stage/render/prepare_assets 25.08, stage/render/render 20.00 |
| warm-up | 4.95 | 75.4 | pipelines/compile_ms 1291, pipelines/front_ms 1256, pipelines/wait_ms 498, main/busy/ms 174, stage/main/update 119, render/thread/ms 82.70, render/busy/ms 80.26, stream/drive/ms 60.85, stage/main/post_update 42.13, pipelines/driver_ms 34.50, stage/render/prepare_assets 28.25, stage/render/render 23.00 |
| warm-up | 5.09 | 138.9 | pipelines/compile_ms 2506, pipelines/front_ms 2444, main/busy/ms 136, stage/main/update 112, render/thread/ms 92.42, render/busy/ms 90.91, pipelines/driver_ms 62.93, stream/drive/ms 60.25, stage/render/prepare_assets 48.64, stage/render/render 20.11, render/graph/ms 18.01, stage/main/post_update 14.20 |
| warm-up | 5.11 | 27.6 | pipelines/compile_ms 1875, pipelines/front_ms 1828, main/busy/ms 83.60, render/thread/ms 72.16, render/busy/ms 70.64, stage/main/update 65.19, pipelines/driver_ms 47.07, stream/drive/ms 43.76, stage/render/prepare_assets 31.94, stage/render/render 24.04, render/graph/ms 21.88, stage/render/graph_submit 17.43 |
| warm-up | 5.20 | 86.9 | pipelines/compile_ms 304, pipelines/front_ms 292, main/busy/ms 78.98, stage/main/update 61.65, render/thread/ms 57.15, render/busy/ms 55.66, stream/drive/ms 43.60, stage/render/prepare_assets 21.15, stage/render/render 20.77, render/graph/ms 18.64, stage/render/graph_submit 14.76, pipelines/driver_ms 11.75 |
| warm-up | 5.28 | 59.0 | pipelines/compile_ms 1015, pipelines/front_ms 982, main/busy/ms 62.47, stage/main/update 46.99, stream/drive/ms 37.14, render/thread/ms 35.45, render/busy/ms 34.07, pipelines/driver_ms 32.69, stage/render/render 14.63, render/graph/ms 12.68, encode//sum 9.97, stage/main/post_update 9.37 |
| warm-up | 5.35 | 30.2 | render/thread/ms 42.39, render/busy/ms 41.05, main/busy/ms 30.02, stage/render/render 13.53, stage/render/prepare_assets 13.06, stage/main/update 12.84, stage/main/post_update 11.64, render/graph/ms 11.56, encode//sum 10.52, stage/render/graph_submit 7.11, stream/drive/ms 5.25, encode/submit 4.77 |
| warm-up | 5.41 | 33.7 | pipelines/compile_ms 553, pipelines/front_ms 535, render/thread/ms 51.93, render/busy/ms 49.72, main/busy/ms 38.39, stage/main/update 18.71, pipelines/driver_ms 18.11, stage/render/render 17.14, render/graph/ms 14.48, stage/render/prepare_assets 14.39, stage/main/post_update 12.12, encode//sum 10.95 |
| warm-up | 5.46 | 46.0 | pipelines/compile_ms 543, pipelines/front_ms 525, render/thread/ms 64.30, render/busy/ms 62.49, main/busy/ms 40.41, stage/render/prepare_assets 33.45, stage/main/update 21.96, pipelines/driver_ms 18.04, stage/render/render 15.58, render/graph/ms 13.18, stream/drive/ms 10.86, stage/main/post_update 10.79 |
| warm-up | 5.48 | 26.4 | pipelines/compile_ms 633, pipelines/front_ms 610, render/thread/ms 59.84, render/busy/ms 56.93, main/busy/ms 38.41, stage/render/prepare_assets 25.55, pipelines/driver_ms 22.94, stage/main/update 21.51, stage/render/render 17.51, render/graph/ms 15.37, encode//sum 10.90, stage/render/graph_submit 10.80 |
| warm-up | 5.51 | 26.9 | pipelines/compile_ms 889, pipelines/front_ms 865, render/thread/ms 47.39, render/busy/ms 44.53, main/busy/ms 34.40, pipelines/driver_ms 23.42, stage/render/render 17.87, stage/main/update 16.25, render/graph/ms 15.66, encode//sum 13.09, stage/main/post_update 11.92, stage/render/prepare_assets 11.32 |
| warm-up | 5.57 | 35.7 | render/thread/ms 46.54, render/busy/ms 45.20, main/busy/ms 31.29, stage/render/prepare_assets 20.21, stage/render/render 14.57, stage/main/update 12.70, render/graph/ms 12.66, stage/main/post_update 11.66, encode//sum 11.59, stage/render/graph_submit 8.93, encode/submit 6.10, encode/pass:shadow_point_light/sum 4.64 |
| warm-up | 5.59 | 26.8 | pipelines/compile_ms 379, pipelines/front_ms 367, render/thread/ms 52.25, render/busy/ms 50.98, main/busy/ms 31.66, stage/render/prepare_assets 26.94, stage/main/update 14.44, stage/render/render 13.40, pipelines/driver_ms 11.96, render/graph/ms 11.69, stage/main/post_update 11.42, encode//sum 10.90 |
| warm-up | 5.68 | 38.3 | pipelines/compile_ms 339, pipelines/front_ms 333, render/thread/ms 50.05, render/busy/ms 48.40, main/busy/ms 34.72, stage/render/prepare_assets 21.22, stage/main/update 20.34, stage/render/render 20.08, render/graph/ms 17.72, stage/render/graph_submit 13.36, stream/drive/ms 12.84, encode//sum 12.42 |
| warm-up | 5.74 | 37.2 | render/thread/ms 55.29, render/busy/ms 53.67, main/busy/ms 30.68, stage/render/prepare_assets 23.02, stage/render/render 18.50, render/graph/ms 16.54, stage/main/post_update 14.25, stage/render/graph_submit 11.63, stage/main/update 11.60, encode//sum 11.27, encode/submit 7.78, stage/render/camera_driver 4.74 |
| warm-up | 5.77 | 28.1 | render/thread/ms 55.99, render/busy/ms 54.53, main/busy/ms 29.08, stage/render/prepare_assets 28.06, stage/render/render 14.85, stage/main/post_update 14.18, pipelines/compile_ms 12.98, render/graph/ms 12.80, pipelines/front_ms 11.79, encode//sum 10.55, stage/render/graph_submit 8.56, stage/main/update 8.43 |
| warm-up | 5.80 | 29.4 | pipelines/compile_ms 580, pipelines/front_ms 555, render/thread/ms 49.70, render/busy/ms 48.22, main/busy/ms 27.57, pipelines/driver_ms 25.26, stage/render/prepare_assets 21.86, stage/render/render 15.50, stage/main/post_update 13.32, render/graph/ms 13.27, encode//sum 11.20, stage/render/graph_submit 8.74 |
| warm-up | 6.02 | 34.8 | pipelines/compile_ms 384, pipelines/front_ms 375, main/busy/ms 42.14, render/thread/ms 29.57, render/busy/ms 28.31, stage/main/update 23.63, stream/drive/ms 18.67, stage/render/render 14.47, stage/main/post_update 12.79, render/graph/ms 12.77, encode//sum 11.11, pipelines/driver_ms 9.26 |
| warm-up | 6.31 | 29.7 | render/thread/ms 41.47, render/busy/ms 39.61, main/busy/ms 30.17, stage/render/render 17.54, render/graph/ms 15.34, stage/main/update 14.75, encode//sum 12.54, stage/main/post_update 11.79, stage/render/graph_submit 10.96, encode/submit 7.81, encode/pass:shadow_point_light/sum 7.67, stage/render/prepare_assets 5.57 |
| warm-up | 6.36 | 32.5 | render/thread/ms 35.34, render/busy/ms 34.33, main/busy/ms 24.57, stage/render/prepare_assets 12.34, stage/render/render 10.41, stage/main/post_update 10.14, stage/main/update 8.98, render/graph/ms 8.92, encode//sum 8.14, stage/render/graph_submit 6.11, encode/pass:shadow_point_light/sum 4.90, stream/drive/ms 4.19 |
| warm-up | 6.47 | 27.1 | pipelines/compile_ms 145, pipelines/front_ms 142, render/thread/ms 37.51, render/busy/ms 36.37, main/busy/ms 30.81, stage/main/update 17.17, stage/render/render 14.40, render/graph/ms 12.57, encode//sum 11.30, stage/render/prepare_assets 11.30, stage/main/post_update 9.90, stage/render/graph_submit 7.76 |
| warm-up | 6.58 | 33.3 | main/busy/ms 43.32, render/thread/ms 42.12, render/busy/ms 40.62, stage/main/update 19.47, stage/render/render 18.22, stage/main/post_update 16.24, render/graph/ms 15.46, encode//sum 11.77, stage/render/graph_submit 10.34, encode/pass:shadow_point_light/sum 6.83, encode/submit 6.65, stage/render/prepare_assets 5.93 |
| warm-up | 6.63 | 33.1 | render/thread/ms 39.95, render/busy/ms 38.86, main/busy/ms 32.31, stage/render/prepare_assets 18.89, stage/main/update 15.12, stage/main/post_update 10.61, stage/render/render 10.05, render/graph/ms 8.48, encode//sum 7.63, stage/render/graph_submit 5.71, encode/pass:shadow_point_light/sum 4.90, encode/submit 3.59 |
| warm-up | 6.66 | 27.1 | render/thread/ms 48.24, render/busy/ms 46.18, main/busy/ms 38.84, stage/render/render 19.96, render/graph/ms 17.32, stage/main/update 16.77, stage/main/post_update 15.66, stage/render/prepare_assets 15.32, stage/render/graph_submit 12.39, encode//sum 10.56, encode/submit 9.48, encode/pass:shadow_point_light/sum 7.47 |
| warm-up | 6.71 | 27.0 | pipelines/compile_ms 167, pipelines/front_ms 161, render/thread/ms 40.80, render/busy/ms 39.44, main/busy/ms 36.78, stage/main/update 16.34, stage/main/post_update 15.57, stage/render/render 15.44, render/graph/ms 13.64, stage/render/prepare_assets 13.43, stage/render/graph_submit 9.52, encode//sum 9.09 |
| warm-up | 6.82 | 26.3 | render/thread/ms 42.62, render/busy/ms 41.46, main/busy/ms 24.53, stage/render/prepare_assets 19.31, stage/render/render 13.80, stage/main/post_update 13.19, render/graph/ms 12.02, encode//sum 9.57, stage/render/graph_submit 8.29, stage/main/update 7.81, encode/pass:shadow_point_light/sum 7.19, encode/submit 5.49 |
| warm-up | 6.89 | 33.2 | render/thread/ms 48.86, render/busy/ms 47.72, main/busy/ms 39.19, stage/render/prepare_assets 24.94, stage/main/update 22.40, stream/drive/ms 16.01, stage/render/render 13.56, stage/main/post_update 12.87, render/graph/ms 11.86, encode//sum 9.82, stage/render/graph_submit 8.13, encode/pass:shadow_point_light/sum 6.91 |
| warm-up | 7.16 | 28.7 | main/busy/ms 38.00, render/thread/ms 26.49, render/busy/ms 25.39, stage/main/update 22.34, pipelines/compile_ms 14.90, pipelines/front_ms 14.39, stream/drive/ms 12.79, stage/render/render 11.49, stage/main/post_update 11.00, render/graph/ms 9.94, encode//sum 8.94, stage/render/graph_submit 6.58 |
| measure | 0.77 | 28.2 | main/busy/ms 33.60, render/thread/ms 26.97, render/busy/ms 25.93, stage/main/update 17.91, stage/render/render 14.23, stream/drive/ms 13.08, render/graph/ms 12.82, stage/main/post_update 12.55, encode//sum 11.81, stage/render/graph_submit 9.11, encode/pass:shadow_point_light/sum 6.96, encode/submit 6.02 |
| measure | 2.72 | 26.2 | render/thread/ms 34.12, render/busy/ms 32.83, encode//sum 25.12, stage/render/render 21.37, main/busy/ms 20.47, render/graph/ms 19.61, encode/pass:shadow_point_light/sum 19.35, stage/render/graph_submit 14.05, stage/main/post_update 13.22, encode/submit 8.56, encode/finish_wall 5.53, stage/render/camera_driver 5.44 |
| measure | 18.18 | 29.2 | main/busy/ms 37.00, render/thread/ms 24.62, render/busy/ms 23.58, stage/main/update 21.51, stream/drive/ms 17.45, stage/render/render 14.84, render/graph/ms 13.37, stage/main/post_update 12.57, encode//sum 12.12, stage/render/graph_submit 9.60, encode/pass:shadow_point_light/sum 9.02, encode/submit 6.38 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=base-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 0%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **72%** (min 50%) · 2625 MHz (min 2625) · memory 8101 MHz · 75.3 W · 76 °C (max 82) · VRAM 6756 MiB · throttled 100% of samples

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
