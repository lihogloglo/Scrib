# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=custom (preset: derived from its members); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=0 (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6860 frames over 65.6 s · **median 9.46 ms (106 fps)** · mean 9.57 · p95 11.06 · p99 12.82 (1% low 78 fps) · worst 31.18 ms · 33 spikes over 1.5× median · 30.2 stutters/min (work 24.7/min)
- warm-up: 354 frames over 8.8 s, worst 2335.8 ms
- hitches (> 25.0 ms): **3** measured, 41 in warm-up
- timed GPU passes: 6.26 ms of a 9.57 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.46 ms, limit 11 ms
- **FAIL** frame_ms: 33 stutters (> 1.5x moving median of 60) in 1.09 min = 30.16/min, limit 1/min
- pass worst frame 31.18 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.869 | 2.035 | 2.067 | 0.92 | 0.172 |
| main_opaque_pass_3d | 1.404 | 1.528 | 2.362 | 0.92 | 0.081 |
| early prepass | 0.470 | 0.511 | 0.754 | 0.92 | 0.092 |
| bloom | 0.291 | 0.317 | 0.324 | 0.92 | 0.061 |
| atmosphere_luts | 0.288 | 0.313 | 0.379 | 0.92 | 0.004 |
| water_velocity | 0.207 | 0.225 | 0.247 | 0.92 | 0.002 |
| water_sspr | 0.185 | 0.202 | 0.203 | 0.92 | 0.003 |
| ssao | 0.173 | 0.188 | 0.191 | 0.92 | 0.008 |
| upscaling | 0.118 | 0.128 | 0.135 | 0.92 | 0.004 |
| atmos_froxel_inject | 0.117 | 0.128 | 0.130 | 0.92 | 0.002 |
| water_fft | 0.112 | 0.122 | 0.126 | 0.92 | 0.003 |
| uniform_allocation | 0.107 | 0.117 | 0.141 | 0.92 | 0.024 |
| clouds_composite | 0.096 | 0.105 | 0.109 | 0.92 | 0.002 |
| gpu_instance_materialization | 0.087 | 0.094 | 0.098 | 0.92 | 0.004 |
| bin_unpacking | 0.085 | 0.093 | 0.113 | 0.92 | 0.018 |
| tonemapping | 0.080 | 0.087 | 0.091 | 0.92 | 0.004 |
| main_transmissive_pass_3d | 0.064 | 0.069 | 0.130 | 0.92 | 0.009 |
| clustering | 0.059 | 0.064 | 0.068 | 0.92 | 0.021 |
| clouds_trace | 0.051 | 0.056 | 0.079 | 0.92 | 0.001 |
| early_mesh_preprocessing | 0.051 | 0.055 | 0.071 | 0.92 | 0.004 |
| render_sky | 0.048 | 0.052 | 0.053 | 0.92 | 0.002 |
| clouds_resolve | 0.046 | 0.050 | 0.051 | 0.92 | 0.001 |
| lightprobe_irradiance_map | 0.034 | 0.413 | 0.417 | 0.08 | 0.001 |
| surface_trail | 0.033 | 0.056 | 0.057 | 0.58 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.031 | 0.92 | 0.001 |
| water_snapshot | 0.022 | 0.024 | 0.027 | 0.92 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.024 | 0.92 | 0.002 |
| range_unpacking | 0.020 | 0.022 | 0.027 | 0.92 | 0.003 |
| lightprobe_radiance_map | 0.018 | 0.220 | 0.231 | 0.08 | 0.004 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.92 | 0.006 |
| night_sky | 0.016 | 0.017 | 0.045 | 0.92 | 0.002 |
| main_transparent_pass_3d | 0.013 | 0.014 | 0.030 | 0.92 | 0.182 |
| atmosphere_environment | 0.010 | 0.011 | 0.012 | 0.92 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.92 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.92 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.92 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.92 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.08 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.08 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.08 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.376 | 9.536 | 19.82 | 57456.7 |
| main/busy/ms | 7.881 | 8.891 | 29.44 | 54063.6 |
| render/busy/ms | 7.861 | 8.926 | 19.23 | 53927.6 |
| stage/render/render | 4.844 | 5.744 | 12.23 | 33231.6 |
| stage/main/post_update | 4.417 | 5.139 | 10.10 | 30298.4 |
| render/graph/ms | 4.202 | 4.932 | 11.07 | 28823.5 |
| encode//sum | 3.426 | 4.143 | 13.71 | 23502.3 |
| stage/render/graph_submit | 3.015 | 3.668 | 8.27 | 20682.0 |
| encode/submit | 1.880 | 2.660 | 5.04 | 12894.9 |
| stage/main/update | 1.838 | 2.297 | 23.68 | 12611.9 |
| encode/finish_wall | 1.217 | 1.575 | 3.29 | 8346.6 |
| stage/render/camera_driver | 1.126 | 1.352 | 2.71 | 7724.8 |
| encode/pass:shadow_directional_light/sum | 0.846 | 1.245 | 2.66 | 5800.9 |
| encode/pass:early prepass/sum | 0.822 | 1.062 | 2.20 | 5641.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.810 | 1.064 | 1.91 | 5558.8 |
| bigspace/total/ms | 0.578 | 1.324 | 2.46 | 3964.2 |
| stage/render/prepare_write_phase_buffers | 0.550 | 0.757 | 1.59 | 3772.5 |
| stage/render/prepare_assets | 0.534 | 0.665 | 10.05 | 3665.4 |
| stage/main/fixed_main_loop | 0.498 | 0.960 | 2.33 | 3417.7 |
| stage/render/prepare_meshes | 0.448 | 0.558 | 3.86 | 3075.9 |
| render/present/ms | 0.424 | 1.166 | 1.87 | 2909.5 |
| stage/render/prepare_resources | 0.419 | 0.493 | 0.96 | 2876.8 |
| stage/main/pre_update | 0.388 | 0.493 | 2.24 | 2665.0 |
| controller/feed/ms | 0.340 | 0.426 | 0.84 | 2334.6 |
| stage/main/first | 0.305 | 0.375 | 5.83 | 2093.2 |
| stage/render/prepare_bind_groups | 0.288 | 0.443 | 2.05 | 1977.0 |
| stage/render/prepare_flush | 0.282 | 0.502 | 1.14 | 1935.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.276 | 0.444 | 1.10 | 1894.9 |
| stage/render/prepare_batch_phases | 0.271 | 0.419 | 0.81 | 1861.0 |
| stage/main/last | 0.255 | 0.302 | 0.81 | 1748.3 |
| bigspace/low_precision/ms | 0.244 | 0.733 | 2.17 | 1673.0 |
| stream/drive/ms | 0.209 | 0.288 | 10.76 | 1435.2 |
| stage/render/extract_commands | 0.195 | 0.225 | 0.56 | 1335.4 |
| anim/evaluate/ms | 0.189 | 0.492 | 1.79 | 1296.2 |
| bigspace/high_precision/ms | 0.176 | 0.729 | 1.98 | 1210.4 |
| stage/render/create_views | 0.165 | 0.205 | 0.67 | 1132.0 |
| anim/evaluate_cpu/ms | 0.162 | 0.206 | 0.83 | 1109.4 |
| stream/tick/ms | 0.155 | 0.223 | 0.51 | 1066.5 |
| stage/render/prepare_views | 0.132 | 0.171 | 1.12 | 908.7 |
| stage/render/specialize | 0.130 | 0.238 | 1.82 | 891.5 |
| pipelines/compile_ms | 0.105 | 0.000 | 129.87 | 722.4 |
| pipelines/front_ms | 0.103 | 0.000 | 126.69 | 703.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.098 | 0.151 | 0.70 | 675.6 |
| render/acquire/ms | 0.090 | 0.120 | 1.09 | 619.6 |
| stage/render/queue | 0.089 | 0.158 | 1.65 | 610.9 |
| stage/render/phase_sort | 0.078 | 0.107 | 0.42 | 533.4 |
| controller/write/ms | 0.073 | 0.100 | 0.58 | 499.5 |
| controller/move/ms | 0.069 | 0.200 | 0.54 | 474.8 |
| terrain/update/ms | 0.058 | 0.291 | 1.98 | 399.3 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.073 | 0.24 | 318.6 |
| physics/step/ms | 0.038 | 0.081 | 0.75 | 260.2 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.53 | 221.9 |
| physics/insert/ms | 0.025 | 0.268 | 0.98 | 168.1 |
| stage/render/cleanup | 0.023 | 0.032 | 0.43 | 158.7 |
| terrain/select/ms | 0.022 | 0.258 | 0.82 | 147.7 |
| weather/map/ms | 0.018 | 0.024 | 0.19 | 120.5 |
| far/select/ms | 0.014 | 0.167 | 0.39 | 98.5 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.022 | 0.57 | 98.2 |
| anim/apply/ms | 0.013 | 0.024 | 0.45 | 91.5 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.18 | 75.1 |
| weather/eval/ms | 0.011 | 0.014 | 0.18 | 73.5 |
| script/update/ms | 0.008 | 0.011 | 0.05 | 54.5 |
| controller/avoid/ms | 0.006 | 0.012 | 0.05 | 40.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.14 | 36.6 |
| script/input/ms | 0.005 | 0.006 | 0.21 | 36.5 |
| stream/spawn/ms | 0.003 | 0.000 | 1.07 | 22.3 |
| script/ui/ms | 0.003 | 0.004 | 0.02 | 21.3 |
| stream/upload/ms | 0.003 | 0.000 | 1.37 | 21.0 |
| pipelines/driver_ms | 0.003 | 0.000 | 3.18 | 19.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 14.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 7.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.6 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 559 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1306 | 2262 | 601 |
| crowd/c0/meshes/count | count | 356 | 646 | 159 |
| crowd/c1/count | count | 144 | 155 | 148 |
| crowd/c1/joints/count | count | 5474 | 5902 | 5595 |
| crowd/c1/meshes/count | count | 1163 | 1256 | 1182 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6362 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1819 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 215 | 206 |
| crowd/joints_extracted/count | count | 717 | 5392 | 693 |
| crowd/joints_uploaded/count | count | 3580 | 6186 | 5807 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.41 | 1.62 | 0.44 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.57 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.18 | 0.02 |
| encode/finish_critical | ms | 0.94 | 2.66 | 0.97 |
| encode/pass:early prepass/max | ms | 0.81 | 2.20 | 0.84 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.81 | 1.91 | 0.97 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.70 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.26 | 1.10 | 0.24 |
| encode/pass:shadow_directional_light/max | ms | 0.81 | 2.66 | 0.76 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.53 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.22 | 29.44 | 8.59 |
| gpu/timed/ms | ms | 7.28 | 9.17 | 8.42 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2595 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.5 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 148 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 315 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 148 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 315 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 767 | 973 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2485 | 3526 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 767 | 973 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2485 | 3526 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 13 | 34 | 8 |
| phases/shadow/bins | count | 726 | 875 | 671 |
| phases/shadow/instance_batches | count | 4 | 8 | 4 |
| phases/shadow/instance_draws | count | 6557 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| phases/shadow/items | count | 2128 | 2427 | 2035 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 25 | 28 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2708 | 3183 | 2349 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 332 | 333 | 333 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 13 | 167 | 11 |
| preprocess/bind_groups_created | count | 8 | 161 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 5 |
| preprocess/bind_groups_reused | count | 125 | 145 | 123 |
| process/cpu_usage | % | 25.70 | 43.27 | 23.33 |
| process/mem_usage | GiB | 4.96 | 5.22 | 5.22 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2976 | 3166 | 3166 |
| scene/batch/masked/live | count | 2976 | 3166 | 3166 |
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
| scene/batch/opaque/capacity | count | 41.0 k | 41.0 k | 41.0 k |
| scene/batch/opaque/count | count | 2 | 2 | 2 |
| scene/batch/opaque/high_water | count | 18.8 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 18.8 k | 19.9 k | 19.9 k |
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
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1591 | 1621 | 1621 |
| scene/chains/live | count | 1591 | 1621 | 1621 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 0 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 0 | 0 | 0 |
| shadow_atlas/live_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/live_tiles | count | 0 | 0 | 0 |
| shadow_atlas/mover_draws | count | 0 | 0 | 0 |
| shadow_atlas/movers | count | 0 | 0 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 0 | 0 | 0 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/live_direct | count | 0 | 0 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 0 | 0 | 0 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/static_tiles | count | 0 | 0 | 0 |
| shadow_atlas/views | count | 0 | 0 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 92 | 15 |
| skin/blocks_skipped/count | count | 145 | 203 | 191 |
| skin/blocks_walked/count | count | 14 | 28 | 15 |
| skin/joint_bytes_written/bytes | bytes | 50.4 k | 812.7 k | 54.0 k |
| skin/joints_written/count | count | 788 | 12.7 k | 844 |
| skin/upload_writes/count | count | 6 | 24 | 11 |
| started_load_count |  | 6439 | 6563 | 6563 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 237 | 333 | 180 |
| statics/materials_bound | count | 237 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.05 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.24 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.74 | 38.96 | 29.29 |
| system/mem_usage | % | 45.98 | 46.69 | 46.69 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 71 | 93 | 66 |
| views/all/bins | count | 2764 | 3049 | 2961 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 12 | 16 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 147.5 k |
| views/all/items | count | 7971 | 9769 | 7977 |
| views/all/pipelines | count | 72 | 75 | 66 |
| views/all/pipelines_distinct | count | 62 | 64 | 61 |
| views/all/work_items | count | 154.0 k | 206.6 k | 154.0 k |
| views/cascade/batch_sets | count | 13 | 34 | 8 |
| views/cascade/bins | count | 726 | 875 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6557 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2128 | 2427 | 2035 |
| views/cascade/pipelines | count | 25 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2016 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5912 | 7490 | 5942 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 104.6 k | 106.0 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 21.6 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 56.70 | 234 | 389.0 k |
| anim/events/count | count | 0.07 | 3 | 500 |
| assets/Image/added | count | 0.09 | 34 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 11 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.42 | 275 | 2874 |
| assets/Mesh/removed | count | 0.71 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.09 | 10 | 7476 |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 9 |
| ecs/entities_delta | count | 0.22 | 1133 | 1490 |
| encode//count | count | 27.89 | 30 | 191.3 k |
| encode/bind_walk/init_textures | count | 8473 | 11.1 k | 58.12 M |
| encode/bind_walk/init_textures_skipped | count | 33.7 k | 52.3 k | 231.26 M |
| encode/bind_walk/inits | count | 523 | 768 | 3.59 M |
| encode/bind_walk/inits_skipped | count | 117 | 217 | 805.9 k |
| encode/bind_walk/merge_textures | count | 8410 | 11.0 k | 57.69 M |
| encode/bind_walk/merge_textures_skipped | count | 9813 | 22.0 k | 67.32 M |
| encode/bind_walk/merges | count | 91.03 | 103 | 624.5 k |
| encode/bind_walk/merges_skipped | count | 112 | 217 | 768.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6860 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6860 |
| encode/pass:early prepass/count | count | 1 | 1 | 6860 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6860 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6860 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6860 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6860 |
| encode/queue/command_buffers | count | 37.90 | 41 | 260.0 k |
| encode/queue/submits | count | 2.01 | 4 | 13.8 k |
| encode/queue/write_buffer/bytes | bytes | 2.31 M | 3.33 M | 15.86 G |
| encode/queue/write_buffer/calls | count | 244 | 306 | 1.67 M |
| encode/queue/write_buffer_with/bytes | bytes | 19.1 k | 1.14 M | 131.00 M |
| encode/queue/write_buffer_with/calls | count | 10.24 | 834 | 70.3 k |
| encode/queue/write_texture/bytes | bytes | 1923 | 1.05 M | 13.19 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6860 |
| physics/inserts | count | 0.33 | 93 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.34 | 175 | 2335 |
| physics/steps | count | 0.57 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 8 |
| pipelines/created | count | 0.00 | 1 | 8 |
| precache/hit | count | 0.00 | 1 | 1 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 29 | 223 |
| stream/despawn/items | count | 0.03 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 4935 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1082 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 44, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.34 | 2335.8 | main/busy/ms 2308, stage/main/post_update 2261, stage/main/update 36.32, stage/main/pre_update 7.22, stage/main/last 0.96, surface/advance_cpu_ms 0.83, stage/main/first 0.51, stream/drive/ms 0.41, stage/main/fixed_main_loop 0.37, surface/cell_upload_ms 0.28, stream/tick/ms 0.09, bigspace/total/ms 0.09 |
| warm-up | 3.07 | 730.4 | main/busy/ms 2446, stage/main/post_update 2283, render/thread/ms 698, render/busy/ms 568, stage/render/prepare_assets 513, stage/main/update 146, stage/render/prepare_views 127, render/acquire/ms 127, stage/render/create_views 35.88, stage/render/render 11.32, stage/main/pre_update 8.95, render/graph/ms 7.32 |
| warm-up | 3.17 | 108.4 | render/thread/ms 787, render/busy/ms 603, stage/render/prepare_assets 534, main/busy/ms 158, stage/render/prepare_views 128, render/acquire/ms 128, pipelines/compile_ms 124, stage/main/update 114, pipelines/front_ms 81.51, stage/render/render 70.59, render/present/ms 56.43, pipelines/driver_ms 42.88 |
| warm-up | 3.21 | 35.7 | pipelines/compile_ms 474, pipelines/front_ms 391, pipelines/wait_ms 201, render/thread/ms 124, main/busy/ms 97.83, stage/render/render 88.03, pipelines/driver_ms 83.61, render/busy/ms 67.92, render/present/ms 55.31, stage/main/post_update 49.73, stage/main/update 37.07, render/graph/ms 31.65 |
| warm-up | 3.37 | 161.0 | pipelines/wait_ms 4576, pipelines/compile_ms 2004, pipelines/front_ms 1870, main/busy/ms 188, render/thread/ms 150, render/busy/ms 147, pipelines/driver_ms 134, stage/main/update 92.45, stage/main/post_update 73.81, stage/render/render 64.78, render/graph/ms 55.78, encode//sum 49.82 |
| warm-up | 3.45 | 75.3 | pipelines/wait_ms 4569, pipelines/compile_ms 2202, pipelines/front_ms 2083, render/thread/ms 185, render/busy/ms 182, main/busy/ms 146, pipelines/driver_ms 118, stage/main/update 69.67, stage/render/prepare_assets 66.44, encode//sum 52.50, stage/render/render 47.72, stage/main/post_update 45.23 |
| warm-up | 3.48 | 32.0 | pipelines/compile_ms 602, pipelines/front_ms 574, pipelines/wait_ms 193, render/thread/ms 98.56, render/busy/ms 96.16, stage/render/prepare_assets 72.46, main/busy/ms 49.95, pipelines/driver_ms 28.00, stage/main/post_update 16.83, stage/render/render 16.39, terrain/update/ms 15.81, stage/main/update 14.97 |
| warm-up | 3.52 | 41.2 | pipelines/compile_ms 847, pipelines/front_ms 808, pipelines/wait_ms 247, render/thread/ms 68.45, render/busy/ms 66.49, stage/render/prepare_assets 49.28, pipelines/driver_ms 38.53, main/busy/ms 30.84, stage/main/post_update 16.60, terrain/update/ms 10.67, stage/render/render 9.31, stage/main/update 9.20 |
| warm-up | 3.59 | 28.5 | pipelines/compile_ms 262, pipelines/front_ms 252, render/thread/ms 36.90, main/busy/ms 36.68, render/busy/ms 35.37, stage/main/post_update 21.76, stage/render/render 12.64, render/graph/ms 10.57, stage/main/update 9.88, pipelines/driver_ms 9.31, encode//sum 8.51, terrain/update/ms 8.27 |
| warm-up | 3.62 | 36.2 | pipelines/compile_ms 574, pipelines/front_ms 543, main/busy/ms 74.94, render/thread/ms 56.10, stage/main/update 55.26, render/busy/ms 54.59, pipelines/driver_ms 30.19, stage/render/prepare_assets 25.52, stage/main/post_update 13.61, stage/render/render 13.08, render/graph/ms 11.01, terrain/update/ms 8.79 |
| warm-up | 3.71 | 87.8 | pipelines/compile_ms 1346, pipelines/front_ms 1282, main/busy/ms 69.50, render/thread/ms 67.07, render/busy/ms 65.45, pipelines/driver_ms 64.44, stage/main/update 56.24, stage/render/prepare_assets 42.57, stage/render/render 14.42, pipelines/wait_ms 12.98, render/graph/ms 12.36, stage/render/graph_submit 9.49 |
| warm-up | 3.75 | 45.6 | pipelines/compile_ms 1338, pipelines/front_ms 1280, render/thread/ms 61.32, render/busy/ms 59.95, pipelines/driver_ms 57.70, stage/render/prepare_assets 41.30, pipelines/wait_ms 21.38, main/busy/ms 20.30, stage/render/render 14.28, render/graph/ms 12.60, stage/render/graph_submit 10.37, stage/main/update 8.58 |
| warm-up | 3.91 | 84.4 | pipelines/compile_ms 597, pipelines/front_ms 575, render/thread/ms 89.98, render/busy/ms 86.47, main/busy/ms 63.21, stage/render/prepare_flush 29.48, stage/main/update 28.63, stage/main/post_update 27.49, stage/render/prepare_assets 25.30, pipelines/driver_ms 22.15, stream/drive/ms 18.63, stage/render/render 17.41 |
| warm-up | 3.96 | 46.7 | pipelines/compile_ms 825, pipelines/front_ms 794, render/thread/ms 100, render/busy/ms 95.68, main/busy/ms 79.66, stage/render/prepare_assets 34.47, stage/main/update 34.25, pipelines/driver_ms 31.20, stage/render/prepare_flush 29.52, stage/main/post_update 28.97, stage/render/render 17.70, stream/drive/ms 14.31 |
| warm-up | 4.09 | 138.1 | pipelines/compile_ms 1171, pipelines/front_ms 1129, main/busy/ms 183, stage/main/update 155, render/thread/ms 124, render/busy/ms 120, stage/render/prepare_assets 86.34, pipelines/driver_ms 41.68, stage/render/render 23.63, render/graph/ms 19.26, pipelines/wait_ms 12.37, stage/render/graph_submit 11.37 |
| warm-up | 4.25 | 158.9 | pipelines/compile_ms 2520, pipelines/front_ms 2393, render/thread/ms 174, render/busy/ms 171, main/busy/ms 162, stage/main/update 145, stage/render/prepare_assets 138, pipelines/driver_ms 127, pipelines/wait_ms 118, pipelines/lock_ms 39.85, stage/render/render 21.96, render/graph/ms 18.51 |
| warm-up | 4.29 | 33.0 | pipelines/compile_ms 1855, pipelines/front_ms 1750, pipelines/wait_ms 146, render/thread/ms 108, pipelines/driver_ms 106, render/busy/ms 104, stage/render/prepare_assets 69.27, main/busy/ms 55.72, pipelines/lock_ms 30.68, stage/main/update 24.14, stage/render/render 22.75, render/graph/ms 17.91 |
| warm-up | 4.37 | 81.9 | pipelines/compile_ms 314, pipelines/front_ms 300, render/thread/ms 95.39, render/busy/ms 91.66, main/busy/ms 90.34, pipelines/wait_ms 47.55, stage/main/post_update 30.23, stage/render/prepare_assets 29.88, stage/render/prepare_write_phase_buffers 23.23, stage/main/last 22.83, stage/render/render 22.19, stage/main/update 21.96 |
| warm-up | 4.40 | 35.9 | render/thread/ms 98.96, render/busy/ms 97.52, main/busy/ms 78.49, pipelines/compile_ms 36.46, pipelines/front_ms 34.27, stage/render/prepare_assets 27.95, stage/main/post_update 27.36, stage/main/update 23.20, stage/main/last 22.83, stage/render/prepare_resources 21.97, stage/render/prepare_write_phase_buffers 21.46, stream/drive/ms 12.24 |
| warm-up | 4.48 | 73.8 | pipelines/compile_ms 2462, pipelines/front_ms 2395, pipelines/wait_ms 233, render/thread/ms 99.31, render/busy/ms 97.68, main/busy/ms 83.94, pipelines/driver_ms 67.00, stage/main/post_update 40.69, stage/main/update 35.32, stage/render/prepare_resources 31.18, stage/render/prepare_assets 22.55, stage/render/prepare_write_phase_buffers 21.57 |
| warm-up | 4.51 | 27.8 | pipelines/compile_ms 2673, pipelines/front_ms 2602, pipelines/wait_ms 234, main/busy/ms 101, render/thread/ms 87.56, render/busy/ms 85.81, pipelines/driver_ms 70.50, stage/main/update 52.80, stage/main/post_update 38.50, stage/render/prepare_assets 26.80, stage/render/prepare_write_phase_buffers 23.50, stage/render/render 15.53 |
| warm-up | 4.58 | 77.9 | pipelines/compile_ms 694, pipelines/front_ms 663, pipelines/wait_ms 219, main/busy/ms 75.16, render/thread/ms 68.04, render/busy/ms 65.78, stage/main/update 52.60, stage/render/prepare_assets 32.90, pipelines/driver_ms 30.46, stage/render/render 19.13, render/graph/ms 16.14, stage/main/post_update 11.84 |
| warm-up | 4.63 | 51.3 | pipelines/compile_ms 807, pipelines/front_ms 767, pipelines/wait_ms 411, main/busy/ms 102, render/thread/ms 84.84, render/busy/ms 82.76, stage/main/update 71.18, stream/drive/ms 45.44, stage/render/prepare_assets 44.47, pipelines/driver_ms 39.69, pipelines/lock_ms 18.32, stage/main/post_update 15.24 |
| warm-up | 4.73 | 99.1 | pipelines/compile_ms 1201, pipelines/front_ms 1166, pipelines/wait_ms 655, main/busy/ms 113, stage/main/update 83.96, render/thread/ms 81.41, render/busy/ms 79.90, stream/drive/ms 60.80, stage/render/prepare_assets 41.32, pipelines/driver_ms 35.37, pipelines/lock_ms 17.61, stage/main/post_update 14.53 |
| warm-up | 4.76 | 30.0 | pipelines/compile_ms 1631, pipelines/front_ms 1587, pipelines/wait_ms 470, main/busy/ms 73.52, render/thread/ms 65.71, render/busy/ms 64.07, stage/main/update 52.92, pipelines/driver_ms 43.80, stage/render/prepare_assets 33.50, stream/drive/ms 20.71, stage/render/render 11.21, stage/main/post_update 10.11 |
| warm-up | 4.79 | 30.8 | pipelines/compile_ms 1319, pipelines/front_ms 1282, pipelines/wait_ms 68.16, main/busy/ms 45.82, render/thread/ms 41.76, render/busy/ms 40.13, pipelines/driver_ms 36.64, stage/main/update 28.70, stage/render/prepare_assets 15.02, stage/render/render 10.65, render/graph/ms 8.65, pipelines/lock_ms 8.55 |
| warm-up | 4.86 | 29.5 | render/thread/ms 43.80, render/busy/ms 42.51, main/busy/ms 38.31, pipelines/compile_ms 34.37, pipelines/front_ms 32.52, stage/main/update 22.88, stage/render/render 12.38, render/graph/ms 10.53, stage/main/post_update 10.29, stage/render/prepare_assets 7.76, stage/render/prepare_write_phase_buffers 7.62, stage/render/graph_submit 7.20 |
| warm-up | 4.98 | 28.8 | pipelines/compile_ms 733, pipelines/front_ms 710, render/thread/ms 42.66, render/busy/ms 41.16, main/busy/ms 34.60, pipelines/driver_ms 23.40, stage/main/update 14.74, stage/render/render 13.74, stage/main/post_update 13.29, stage/render/prepare_resources 12.51, render/graph/ms 11.86, stage/render/graph_submit 8.07 |
| warm-up | 5.05 | 50.2 | pipelines/compile_ms 266, pipelines/front_ms 254, render/thread/ms 65.62, render/busy/ms 64.29, main/busy/ms 55.53, stage/main/update 40.50, stage/render/prepare_batch_phases 19.47, stream/drive/ms 13.69, stage/render/render 12.06, pipelines/driver_ms 11.19, render/graph/ms 10.26, pipelines/wait_ms 9.16 |
| warm-up | 5.07 | 25.3 | pipelines/compile_ms 459, pipelines/front_ms 442, render/thread/ms 66.55, render/busy/ms 65.22, main/busy/ms 55.24, stage/main/update 37.64, stage/render/prepare_batch_phases 19.76, pipelines/driver_ms 17.08, stream/drive/ms 11.90, stage/render/prepare_assets 11.46, stage/render/render 11.21, pipelines/wait_ms 9.42 |
| warm-up | 5.29 | 25.0 | main/busy/ms 29.37, pipelines/compile_ms 23.27, render/thread/ms 22.57, render/busy/ms 21.46, pipelines/front_ms 21.30, stage/main/update 19.07, stream/drive/ms 14.03, stage/render/render 8.92, render/graph/ms 7.40, encode//sum 7.17, stage/render/prepare_assets 6.92, stage/main/post_update 6.22 |
| warm-up | 5.52 | 27.2 | pipelines/compile_ms 140, pipelines/front_ms 136, main/busy/ms 30.05, stage/main/update 20.64, render/thread/ms 19.45, render/busy/ms 18.41, stream/drive/ms 12.34, stage/render/render 8.11, render/graph/ms 6.68, encode//sum 6.64, stage/main/post_update 6.22, stage/render/prepare_assets 5.42 |
| warm-up | 5.62 | 25.1 | render/thread/ms 40.23, render/busy/ms 38.56, main/busy/ms 34.24, stage/main/update 15.20, stage/render/render 14.70, stage/main/post_update 13.77, render/graph/ms 12.44, stage/render/prepare_assets 11.14, encode//sum 10.29, stage/render/graph_submit 8.65, encode/submit 5.90, stage/render/camera_driver 3.62 |
| warm-up | 5.68 | 55.1 | pipelines/compile_ms 162, pipelines/front_ms 141, render/thread/ms 66.89, render/busy/ms 65.76, stage/render/render 34.43, encode//sum 34.25, render/graph/ms 32.70, main/busy/ms 31.26, stage/render/graph_submit 29.89, encode/finish_wall 27.33, encode/pass:early prepass/sum 26.47, pipelines/driver_ms 21.21 |
| warm-up | 5.79 | 27.4 | main/busy/ms 37.54, render/thread/ms 31.28, render/busy/ms 30.20, stage/main/update 24.54, stage/render/render 8.92, stage/render/prepare_assets 8.23, stage/main/post_update 8.15, render/graph/ms 7.53, encode//sum 6.34, stage/render/graph_submit 5.09, stream/drive/ms 4.65, stage/render/prepare_meshes 3.89 |
| warm-up | 5.89 | 29.0 | render/thread/ms 41.07, render/busy/ms 39.34, main/busy/ms 38.92, stage/main/update 20.36, stage/render/render 19.76, render/graph/ms 17.51, stage/render/graph_submit 13.47, stage/main/post_update 13.02, encode//sum 11.14, pipelines/compile_ms 10.38, pipelines/front_ms 9.51, encode/submit 8.68 |
| warm-up | 5.95 | 33.6 | render/thread/ms 43.68, render/busy/ms 42.66, main/busy/ms 39.15, stage/main/update 21.64, stage/render/prepare_assets 16.72, stage/main/post_update 10.14, stage/render/render 9.01, render/graph/ms 7.54, stage/render/prepare_meshes 7.51, encode//sum 6.70, stage/render/graph_submit 4.64, stream/drive/ms 4.49 |
| warm-up | 5.98 | 35.1 | render/thread/ms 56.65, render/busy/ms 55.27, main/busy/ms 45.56, stage/main/update 22.25, stage/render/prepare_assets 18.93, stage/main/post_update 14.62, stage/render/render 13.49, render/graph/ms 11.14, encode//sum 10.45, stage/render/prepare_meshes 9.47, stage/render/graph_submit 7.28, encode/submit 4.26 |
| warm-up | 6.04 | 32.1 | pipelines/compile_ms 141, pipelines/front_ms 137, render/thread/ms 36.79, render/busy/ms 35.89, main/busy/ms 29.58, stage/main/update 16.61, stage/render/prepare_assets 15.03, stage/render/render 7.70, stage/main/post_update 7.16, encode//sum 6.43, render/graph/ms 6.42, stage/render/graph_submit 4.09 |
| warm-up | 6.18 | 50.1 | main/busy/ms 58.12, render/thread/ms 40.71, stage/main/update 40.15, render/busy/ms 39.35, stream/drive/ms 14.16, stage/render/prepare_assets 14.06, stage/main/post_update 12.12, stage/render/render 11.64, render/graph/ms 9.83, encode//sum 8.07, stage/render/graph_submit 6.38, stage/render/prepare_meshes 5.86 |
| warm-up | 6.21 | 31.0 | render/thread/ms 48.52, render/busy/ms 47.25, main/busy/ms 46.95, stage/main/update 26.64, stage/render/prepare_assets 21.59, stage/main/post_update 12.20, stage/render/render 11.13, render/graph/ms 9.48, encode//sum 8.06, stage/render/prepare_meshes 7.20, stream/drive/ms 6.85, stage/render/graph_submit 6.20 |
| measure | 18.29 | 31.2 | main/busy/ms 37.33, stage/main/update 25.99, render/thread/ms 19.88, render/busy/ms 18.94, stage/render/render 9.44, render/graph/ms 8.20, stage/main/post_update 7.65, encode//sum 6.37, stage/render/graph_submit 6.08, encode/submit 3.76, stage/render/prepare_assets 3.68, encode/finish_wall 2.34 |
| measure | 59.11 | 26.7 | render/thread/ms 38.05, render/busy/ms 36.63, main/busy/ms 34.96, stage/main/post_update 18.43, stage/render/render 16.61, render/graph/ms 14.48, encode//sum 11.82, stage/render/graph_submit 10.30, stage/main/update 9.46, stage/render/prepare_assets 8.68, encode/submit 6.12, encode/finish_wall 4.28 |
| measure | 59.14 | 27.0 | render/thread/ms 37.50, render/busy/ms 36.44, main/busy/ms 25.96, stage/render/prepare_assets 17.24, stage/main/post_update 13.23, stage/render/render 8.91, encode//sum 7.85, render/graph/ms 7.35, stage/main/update 5.27, stage/render/graph_submit 4.73, stage/main/fixed_main_loop 3.15, encode/submit 2.87 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.count=0 --resolution 2560x1440 --bench-out target/perf\shadow_review2\balmora_walk_night@2560x1440_arm=nolamps-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **85%** (min 63%) · 2621 MHz (min 2610) · memory 8101 MHz · 86.2 W · 80 °C (max 84) · VRAM 5551 MiB · throttled 100% of samples

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
- `sg.post=high (preset: level requested)`
- `sg.shadows=custom (preset: derived from its members)`
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
- `shadow.lamp.count=0 (command line: --set)`
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
