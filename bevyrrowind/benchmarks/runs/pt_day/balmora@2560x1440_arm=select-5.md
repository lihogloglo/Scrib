# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); select.gate=on (command line: --set); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1545 frames over 15.0 s · **median 9.74 ms (103 fps)** · mean 9.71 · p95 11.07 · p99 11.58 (1% low 86 fps) · worst 26.37 ms · 3 spikes over 1.5× median · 12.0 stutters/min (work 4.0/min)
- warm-up: 279 frames over 8.6 s, worst 2593.7 ms
- hitches (> 25.0 ms): **1** measured, 59 in warm-up
- timed GPU passes: 8.58 ms of a 9.71 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.74 ms, limit 11 ms
- pass worst frame 26.37 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.937 | 2.942 | 2.983 | 1.00 | 0.065 |
| dlss_super_resolution | 2.017 | 2.021 | 2.044 | 1.00 | 0.188 |
| early prepass | 0.605 | 0.606 | 0.616 | 1.00 | 0.093 |
| atmosphere_luts | 0.326 | 0.326 | 0.342 | 1.00 | 0.003 |
| bloom | 0.314 | 0.314 | 0.331 | 1.00 | 0.062 |
| main_transmissive_pass_3d | 0.310 | 0.311 | 0.332 | 1.00 | 0.008 |
| water_velocity | 0.240 | 0.240 | 0.264 | 1.00 | 0.002 |
| water_sspr | 0.208 | 0.208 | 0.210 | 1.00 | 0.002 |
| ssao | 0.191 | 0.191 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.149 | 0.149 | 0.170 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.146 | 0.153 | 1.00 | 0.001 |
| water_fft | 0.132 | 0.133 | 0.136 | 1.00 | 0.003 |
| uniform_allocation | 0.120 | 0.120 | 0.162 | 1.00 | 0.021 |
| upscaling | 0.113 | 0.113 | 0.118 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.089 | 0.089 | 0.122 | 1.00 | 0.016 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.412 | 0.413 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.032 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.028 | 0.223 | 0.237 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 1.00 | 0.082 |
| water_snapshot | 0.025 | 0.025 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.047 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 1.00 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.011 | 0.011 | 0.011 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.004 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.754 | 10.156 | 25.01 | 13525.0 |
| render/busy/ms | 6.821 | 7.850 | 24.50 | 10538.5 |
| main/busy/ms | 6.687 | 7.621 | 9.31 | 10331.7 |
| stage/render/render | 4.529 | 5.348 | 11.64 | 6997.5 |
| stage/main/post_update | 4.006 | 4.835 | 6.50 | 6189.6 |
| render/graph/ms | 3.958 | 4.741 | 7.29 | 6114.4 |
| encode//sum | 3.831 | 5.042 | 7.60 | 5918.7 |
| stage/render/graph_submit | 2.964 | 3.717 | 6.04 | 4579.7 |
| encode/submit | 1.837 | 2.262 | 7.70 | 2837.9 |
| stage/render/prepare_views | 1.626 | 3.319 | 4.68 | 2511.8 |
| render/acquire/ms | 1.580 | 3.270 | 4.63 | 2441.5 |
| stage/main/update | 1.575 | 2.042 | 2.89 | 2432.6 |
| encode/finish_wall | 1.146 | 1.607 | 2.39 | 1771.2 |
| stage/render/camera_driver | 0.937 | 1.076 | 1.77 | 1447.8 |
| encode/pass:shadow_directional_light/sum | 0.728 | 1.251 | 2.09 | 1124.5 |
| encode/pass:early prepass/sum | 0.701 | 0.946 | 1.66 | 1083.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.697 | 0.962 | 1.53 | 1077.4 |
| stage/render/prepare_write_phase_buffers | 0.543 | 0.749 | 0.93 | 839.6 |
| stage/render/prepare_assets | 0.448 | 0.562 | 1.60 | 691.8 |
| stage/render/prepare_resources | 0.380 | 0.436 | 0.81 | 587.7 |
| render/present/ms | 0.353 | 0.416 | 3.99 | 545.0 |
| bigspace/total/ms | 0.343 | 0.630 | 1.60 | 530.6 |
| stage/main/pre_update | 0.338 | 0.427 | 0.93 | 521.9 |
| stage/main/first | 0.293 | 0.352 | 1.67 | 453.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.236 | 0.334 | 0.59 | 364.3 |
| stage/render/prepare_bind_groups | 0.204 | 0.274 | 0.41 | 314.7 |
| stage/render/prepare_meshes | 0.200 | 0.247 | 2.32 | 308.4 |
| stage/main/fixed_main_loop | 0.177 | 0.321 | 1.16 | 273.0 |
| stage/render/prepare_batch_phases | 0.177 | 0.240 | 0.38 | 272.9 |
| stage/render/extract_commands | 0.172 | 0.202 | 0.45 | 266.2 |
| stage/render/create_views | 0.165 | 0.213 | 0.76 | 255.0 |
| stage/render/prepare_flush | 0.148 | 0.255 | 0.55 | 228.8 |
| controller/feed/ms | 0.147 | 0.194 | 0.59 | 227.6 |
| controller/move/ms | 0.143 | 0.283 | 0.79 | 220.3 |
| stream/drive/ms | 0.139 | 0.181 | 0.62 | 215.4 |
| stage/main/last | 0.118 | 0.168 | 0.78 | 181.9 |
| bigspace/high_precision/ms | 0.116 | 0.253 | 1.08 | 178.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.115 | 0.176 | 0.28 | 177.7 |
| stream/tick/ms | 0.112 | 0.151 | 0.59 | 173.8 |
| stage/render/queue | 0.106 | 0.136 | 5.34 | 163.7 |
| bigspace/low_precision/ms | 0.085 | 0.194 | 1.26 | 132.0 |
| anim/evaluate/ms | 0.084 | 0.217 | 0.83 | 130.2 |
| stage/render/specialize | 0.082 | 0.101 | 8.62 | 127.0 |
| anim/evaluate_cpu/ms | 0.062 | 0.078 | 0.51 | 95.7 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.114 | 0.22 | 89.5 |
| encode/resolve_diagnostics/sum | 0.056 | 0.090 | 0.20 | 87.1 |
| terrain/update/ms | 0.031 | 0.042 | 0.07 | 47.2 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.042 | 0.10 | 42.0 |
| physics/step/ms | 0.026 | 0.052 | 0.44 | 40.9 |
| weather/map/ms | 0.025 | 0.032 | 0.45 | 38.1 |
| encode/camera_driver_span_end/sum | 0.024 | 0.040 | 0.11 | 36.3 |
| stage/render/cleanup | 0.023 | 0.032 | 0.28 | 36.0 |
| stage/render/phase_sort | 0.018 | 0.032 | 0.12 | 27.4 |
| weather/eval/ms | 0.011 | 0.014 | 0.03 | 16.4 |
| script/update/ms | 0.006 | 0.008 | 0.03 | 9.9 |
| controller/avoid/ms | 0.006 | 0.007 | 0.04 | 9.1 |
| script/input/ms | 0.006 | 0.007 | 0.39 | 8.7 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 6.7 |
| script/ui/ms | 0.004 | 0.005 | 0.42 | 5.8 |
| far/select/ms | 0.004 | 0.004 | 0.02 | 5.6 |
| controller/write/ms | 0.002 | 0.002 | 0.04 | 3.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 0.6 |
| terrain/select/ms | 0.000 | 0.001 | 0.01 | 0.6 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.53 | 2.32 | 0.80 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.10 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.11 | 0.02 |
| encode/finish_critical | ms | 0.83 | 2.32 | 0.80 |
| encode/pass:early prepass/max | ms | 0.70 | 1.26 | 0.77 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.71 | 1.28 | 0.63 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.28 | 0.15 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.59 | 0.37 |
| encode/pass:shadow_directional_light/max | ms | 0.67 | 2.09 | 0.73 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.20 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.23 | 24.50 | 9.26 |
| gpu/timed/ms | ms | 9.22 | 10.83 | 9.26 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1943 | 1943 | 1943 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 207.2 k | 207.2 k | 207.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 13.3 k | 13.3 k | 13.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 485.99 M | 485.99 M | 485.99 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 21 | 21 | 21 |
| phases/opaque3d/bins | count | 1306 | 1306 | 1306 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/bins | count | 1306 | 1306 | 1306 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 1 | 30 | 1 |
| phases/shadow/bins | count | 2152 | 4889 | 2152 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6053 | 17.7 k | 6053 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 23 | 30 | 23 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 320 | 320 | 320 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 0 | 62 | 0 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 127 | 127 | 127 |
| process/cpu_usage | % | 24.04 | 39.02 | 22.60 |
| process/mem_usage | GiB | 4.67 | 4.72 | 4.72 |
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
| skin/blocks_changed/count | count | 5 | 7 | 3 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 181 | 181 | 181 |
| skin/joint_bytes_written/bytes | bytes | 309.5 k | 309.5 k | 309.5 k |
| skin/joints_written/count | count | 4836 | 4836 | 4836 |
| skin/upload_writes/count | count | 1 | 1 | 1 |
| started_load_count |  | 3704 | 3706 | 3706 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 386 | 386 | 386 |
| statics/materials_bound | count | 386 | 386 | 386 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.67 | 36.24 | 26.86 |
| system/mem_usage | % | 60.24 | 60.43 | 60.43 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 53 | 82 | 53 |
| views/all/bins | count | 5324 | 8061 | 5324 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 15.3 k | 27.0 k | 15.3 k |
| views/all/pipelines | count | 71 | 78 | 71 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 9496 | 27.2 k | 9496 |
| views/cascade/batch_sets | count | 1 | 30 | 1 |
| views/cascade/bins | count | 2152 | 4889 | 2152 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 6053 | 17.7 k | 6053 |
| views/cascade/pipelines | count | 23 | 30 | 23 |
| views/cascade/work_items | count | 34 | 17.7 k | 34 |
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
| views/main/batch_sets | count | 52 | 52 | 52 |
| views/main/bins | count | 3172 | 3172 | 3172 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 48 | 48 | 48 |
| views/main/work_items | count | 9462 | 9462 | 9462 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.09 | 2 | 135 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.20 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 52 | 41.0 k |
| encode/bind_walk/init_textures | count | 7165 | 14.0 k | 11.07 M |
| encode/bind_walk/init_textures_skipped | count | 43.8 k | 78.0 k | 67.74 M |
| encode/bind_walk/inits | count | 345 | 652 | 533.6 k |
| encode/bind_walk/inits_skipped | count | 75.50 | 142 | 116.6 k |
| encode/bind_walk/merge_textures | count | 7101 | 13.9 k | 10.97 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 8162 | 6.31 M |
| encode/bind_walk/merges | count | 82.26 | 156 | 127.1 k |
| encode/bind_walk/merges_skipped | count | 55.00 | 106 | 85.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1545 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1545 |
| encode/pass:early prepass/count | count | 1 | 2 | 1545 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1545 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 1545 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 1545 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 1545 |
| encode/queue/command_buffers | count | 36.57 | 72 | 56.5 k |
| encode/queue/submits | count | 2 | 4 | 3090 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.65 M | 2.22 G |
| encode/queue/write_buffer/calls | count | 217 | 400 | 335.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 65.3 k | 50.43 M |
| encode/queue/write_buffer_with/calls | count | 9 | 18 | 13.9 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1545 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| precache/hit | count | 0 | 0 | 0 |
| precache/missed | count | 0 | 0 | 0 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0 | 0 | 0 |
| stream/deactivate/items | count | 0 | 0 | 0 |
| stream/despawn/items | count | 0 | 0 | 0 |
| stream/fetch/items | count | 0 | 0 | 0 |
| stream/spawn/items | count | 0 | 0 | 0 |
| stream/upload/bytes | bytes | 0 | 0 | 0 |
| stream/upload/items | count | 0 | 0 | 0 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 1 |
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 60, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.59 | 2593.7 | main/busy/ms 2564, stage/main/post_update 2459, stage/main/update 96.64, surface/advance_cpu_ms 66.92, stage/main/pre_update 4.54, stage/main/last 1.00, surface/cell_upload_ms 0.79, stage/main/first 0.56, stage/main/fixed_main_loop 0.45, stream/drive/ms 0.37, weather/map/ms 0.12, stream/tick/ms 0.07 |
| warm-up | 3.37 | 772.6 | main/busy/ms 2707, stage/main/post_update 2487, render/thread/ms 734, render/busy/ms 601, stage/render/prepare_assets 553, stage/main/update 207, stage/render/prepare_views 130, render/acquire/ms 130, surface/advance_cpu_ms 66.92, stage/render/create_views 28.54, stage/render/render 12.38, render/graph/ms 8.05 |
| warm-up | 3.47 | 107.6 | render/thread/ms 824, render/busy/ms 641, stage/render/prepare_assets 578, main/busy/ms 165, stage/render/prepare_views 130, render/acquire/ms 130, stage/main/update 114, stage/render/render 69.18, render/present/ms 53.39, pipelines/compile_ms 44.81, pipelines/front_ms 41.54, stage/main/post_update 39.68 |
| warm-up | 3.52 | 45.3 | pipelines/compile_ms 304, pipelines/front_ms 270, render/thread/ms 127, stage/render/render 87.85, main/busy/ms 65.43, render/busy/ms 63.28, render/present/ms 63.03, stage/main/post_update 49.85, pipelines/wait_ms 37.83, pipelines/driver_ms 33.45, terrain/update/ms 27.91, stage/render/prepare_assets 25.75 |
| warm-up | 3.60 | 82.4 | pipelines/wait_ms 2829, pipelines/compile_ms 950, pipelines/front_ms 881, render/thread/ms 117, render/busy/ms 102, pipelines/driver_ms 68.87, main/busy/ms 60.32, stage/main/post_update 49.04, stage/render/render 41.90, stage/render/prepare_resources 34.12, render/graph/ms 27.41, pipelines/lock_ms 24.20 |
| warm-up | 3.70 | 95.7 | pipelines/wait_ms 5479, pipelines/compile_ms 1752, pipelines/front_ms 1668, render/thread/ms 174, render/busy/ms 171, main/busy/ms 143, pipelines/driver_ms 84.07, stage/main/update 59.78, stage/render/prepare_batch_phases 44.67, stage/main/fixed_main_loop 38.94, stage/render/prepare_resources 34.44, stage/render/prepare_assets 26.63 |
| warm-up | 3.81 | 111.1 | pipelines/wait_ms 2830, pipelines/compile_ms 2247, pipelines/front_ms 2151, render/thread/ms 162, render/busy/ms 159, main/busy/ms 146, pipelines/driver_ms 96.55, stage/render/prepare_assets 77.34, stage/main/update 62.42, stage/main/fixed_main_loop 39.94, stage/render/prepare_batch_phases 34.61, stage/main/post_update 20.61 |
| warm-up | 3.85 | 46.2 | pipelines/compile_ms 1925, pipelines/front_ms 1834, pipelines/wait_ms 252, render/thread/ms 110, render/busy/ms 107, pipelines/driver_ms 91.52, stage/render/prepare_assets 87.74, main/busy/ms 64.04, stage/main/pre_update 31.31, stage/main/post_update 19.48, encode//sum 15.52, stage/render/render 14.02 |
| warm-up | 3.89 | 32.1 | pipelines/compile_ms 1004, pipelines/front_ms 927, pipelines/wait_ms 111, pipelines/driver_ms 77.12, render/thread/ms 70.48, render/busy/ms 67.01, main/busy/ms 58.54, stage/render/prepare_assets 34.86, stage/main/pre_update 31.04, stage/render/render 24.44, render/graph/ms 20.67, stage/render/graph_submit 18.05 |
| warm-up | 3.96 | 31.2 | pipelines/compile_ms 70.61, pipelines/front_ms 67.07, main/busy/ms 59.39, render/thread/ms 52.26, render/busy/ms 50.24, stage/main/update 45.15, stream/drive/ms 41.31, stage/render/prepare_assets 31.08, stage/render/render 11.90, terrain/update/ms 10.43, render/graph/ms 9.56, stage/main/post_update 8.23 |
| warm-up | 4.03 | 68.9 | pipelines/compile_ms 1164, pipelines/front_ms 1127, render/thread/ms 69.65, render/busy/ms 67.80, main/busy/ms 57.36, stage/main/update 45.87, stage/render/prepare_assets 44.64, stream/drive/ms 41.36, pipelines/driver_ms 36.84, pipelines/wait_ms 22.04, stage/render/render 16.72, render/graph/ms 14.72 |
| warm-up | 4.10 | 47.6 | pipelines/compile_ms 318, pipelines/front_ms 303, render/thread/ms 51.97, render/busy/ms 42.24, main/busy/ms 41.63, stage/main/update 25.26, stage/render/render 23.37, stream/drive/ms 20.25, stage/render/prepare_assets 19.46, pipelines/driver_ms 14.62, render/graph/ms 13.27, render/present/ms 9.31 |
| warm-up | 4.13 | 32.4 | pipelines/compile_ms 397, pipelines/front_ms 380, main/busy/ms 98.10, stage/main/update 84.83, stream/drive/ms 62.23, render/thread/ms 53.39, render/busy/ms 51.55, stage/render/prepare_assets 33.51, pipelines/driver_ms 16.81, stage/render/render 12.68, render/graph/ms 10.28, stage/main/post_update 7.12 |
| warm-up | 4.28 | 146.2 | pipelines/compile_ms 1383, pipelines/front_ms 1345, main/busy/ms 103, stage/main/update 87.81, render/thread/ms 79.74, render/busy/ms 72.74, stream/drive/ms 62.09, stage/render/prepare_assets 58.09, pipelines/driver_ms 38.59, stage/render/render 14.44, stage/main/post_update 8.64, render/graph/ms 6.97 |
| warm-up | 4.31 | 33.5 | pipelines/compile_ms 1350, pipelines/front_ms 1313, render/thread/ms 86.07, render/busy/ms 79.02, main/busy/ms 73.23, stage/render/prepare_assets 48.16, stage/main/update 41.90, pipelines/driver_ms 37.28, stream/drive/ms 23.30, stage/render/render 17.49, pipelines/wait_ms 12.89, stage/main/post_update 11.13 |
| warm-up | 4.42 | 108.0 | pipelines/compile_ms 208, pipelines/front_ms 198, main/busy/ms 90.24, render/thread/ms 89.67, render/busy/ms 87.88, stage/main/update 41.02, stage/render/prepare_assets 34.51, stage/render/render 26.85, render/graph/ms 24.26, stream/drive/ms 23.55, stage/render/graph_submit 18.02, stage/main/pre_update 17.44 |
| warm-up | 4.49 | 71.0 | pipelines/compile_ms 1434, pipelines/front_ms 1384, pipelines/wait_ms 273, render/thread/ms 98.32, render/busy/ms 96.31, main/busy/ms 59.26, stage/render/prepare_assets 56.15, pipelines/driver_ms 49.83, stage/render/render 24.09, render/graph/ms 21.59, stage/main/pre_update 21.10, stage/main/post_update 16.10 |
| warm-up | 4.57 | 80.8 | pipelines/compile_ms 2107, pipelines/front_ms 2039, pipelines/wait_ms 386, render/thread/ms 96.72, render/busy/ms 95.07, pipelines/driver_ms 67.56, main/busy/ms 59.40, stage/render/prepare_assets 56.58, stage/main/post_update 20.50, stage/main/update 19.66, stage/render/prepare_write_phase_buffers 12.90, stage/render/render 9.17 |
| warm-up | 4.61 | 39.5 | pipelines/compile_ms 960, pipelines/front_ms 932, pipelines/wait_ms 115, render/thread/ms 90.88, render/busy/ms 89.36, main/busy/ms 59.47, stage/render/prepare_assets 38.39, pipelines/driver_ms 28.37, stage/main/update 24.75, stage/main/post_update 20.24, stage/render/prepare_write_phase_buffers 14.90, stage/render/render 10.50 |
| warm-up | 4.70 | 88.4 | pipelines/compile_ms 1512, pipelines/front_ms 1469, pipelines/wait_ms 313, render/thread/ms 93.81, render/busy/ms 92.31, stage/render/prepare_assets 56.38, main/busy/ms 52.65, pipelines/driver_ms 42.09, stage/main/post_update 23.20, stage/main/update 17.55, stage/render/render 11.72, render/graph/ms 9.81 |
| warm-up | 4.79 | 62.8 | pipelines/compile_ms 1091, pipelines/front_ms 1061, pipelines/wait_ms 82.90, main/busy/ms 63.42, render/thread/ms 57.06, render/busy/ms 55.27, stage/main/update 31.60, pipelines/driver_ms 29.71, stage/render/prepare_assets 23.22, stage/main/post_update 16.01, stage/render/render 12.15, stage/main/fixed_main_loop 10.61 |
| warm-up | 4.85 | 64.7 | pipelines/compile_ms 1424, pipelines/front_ms 1386, main/busy/ms 85.55, render/thread/ms 83.79, render/busy/ms 81.88, stage/main/update 63.33, stream/drive/ms 46.71, pipelines/wait_ms 44.76, pipelines/driver_ms 37.67, stage/render/prepare_assets 36.83, stage/render/render 30.18, render/graph/ms 27.33 |
| warm-up | 4.88 | 26.6 | pipelines/compile_ms 1111, pipelines/front_ms 1078, main/busy/ms 108, stage/main/update 92.80, stream/drive/ms 84.20, render/thread/ms 64.48, render/busy/ms 62.56, pipelines/wait_ms 47.31, pipelines/driver_ms 33.02, stage/render/render 32.93, render/graph/ms 30.05, stage/render/prepare_assets 21.00 |
| warm-up | 4.96 | 85.8 | pipelines/compile_ms 1161, pipelines/front_ms 1125, main/busy/ms 67.79, stage/main/update 51.15, render/thread/ms 43.00, stream/drive/ms 42.44, render/busy/ms 41.38, pipelines/driver_ms 35.25, stage/render/render 19.95, render/graph/ms 17.81, stage/render/prepare_assets 13.76, stage/render/graph_submit 11.13 |
| warm-up | 5.02 | 53.1 | pipelines/compile_ms 1132, pipelines/front_ms 1100, render/thread/ms 70.44, render/busy/ms 68.20, main/busy/ms 64.47, stage/main/update 33.21, pipelines/driver_ms 32.78, stage/render/render 27.58, render/graph/ms 24.94, stage/main/post_update 20.93, stage/render/prepare_resources 18.14, stage/render/prepare_assets 15.44 |
| warm-up | 5.08 | 64.3 | pipelines/compile_ms 882, pipelines/front_ms 856, main/busy/ms 91.28, render/thread/ms 82.69, render/busy/ms 80.66, stage/main/update 58.85, stage/render/prepare_assets 34.10, pipelines/driver_ms 26.23, pipelines/wait_ms 21.57, stage/render/render 21.48, stage/main/post_update 20.69, render/graph/ms 19.00 |
| warm-up | 5.13 | 50.1 | pipelines/compile_ms 808, pipelines/front_ms 785, render/thread/ms 76.43, render/busy/ms 75.03, main/busy/ms 60.97, stage/render/prepare_assets 44.24, stage/main/update 38.10, pipelines/driver_ms 23.17, stage/main/post_update 14.31, stream/drive/ms 11.94, pipelines/wait_ms 11.71, stage/render/render 11.57 |
| warm-up | 5.16 | 29.9 | pipelines/compile_ms 1048, pipelines/front_ms 1019, render/thread/ms 66.00, render/busy/ms 64.37, main/busy/ms 43.77, pipelines/driver_ms 29.17, stage/render/prepare_assets 25.45, stage/main/post_update 19.88, pipelines/wait_ms 17.95, stage/main/update 17.78, stage/render/render 12.41, render/graph/ms 10.31 |
| warm-up | 5.19 | 25.3 | pipelines/compile_ms 1263, pipelines/front_ms 1230, render/thread/ms 49.70, render/busy/ms 47.77, main/busy/ms 41.59, pipelines/driver_ms 33.59, stage/main/update 20.74, pipelines/wait_ms 18.38, stage/main/post_update 13.92, stage/render/render 12.93, stage/render/prepare_assets 12.58, render/graph/ms 11.00 |
| warm-up | 5.22 | 34.7 | pipelines/compile_ms 878, pipelines/front_ms 858, main/busy/ms 54.87, render/thread/ms 51.54, render/busy/ms 49.89, stage/main/update 27.77, pipelines/driver_ms 20.19, stage/render/prepare_assets 18.69, stream/drive/ms 17.73, stage/main/pre_update 14.83, stage/render/render 12.24, render/graph/ms 10.18 |
| warm-up | 5.26 | 41.1 | pipelines/compile_ms 886, pipelines/front_ms 862, main/busy/ms 68.24, render/thread/ms 61.40, render/busy/ms 59.65, stage/main/update 37.59, pipelines/driver_ms 23.89, stage/render/prepare_assets 19.13, stream/drive/ms 18.54, stage/render/render 14.76, stage/main/pre_update 14.17, stage/render/prepare_write_phase_buffers 13.64 |
| warm-up | 5.30 | 35.3 | pipelines/compile_ms 922, pipelines/front_ms 893, render/thread/ms 53.62, render/busy/ms 51.86, main/busy/ms 47.73, pipelines/wait_ms 30.74, pipelines/driver_ms 28.46, stage/main/update 26.95, stage/render/prepare_assets 15.22, stage/main/post_update 13.92, stage/render/prepare_write_phase_buffers 12.55, stage/render/render 12.19 |
| warm-up | 5.36 | 42.0 | pipelines/compile_ms 450, pipelines/front_ms 435, render/thread/ms 47.77, render/busy/ms 46.04, main/busy/ms 43.38, stage/main/post_update 19.33, stage/main/update 17.06, stage/render/prepare_assets 15.38, pipelines/driver_ms 15.19, stage/render/render 14.50, render/graph/ms 12.13, encode//sum 8.27 |
| warm-up | 5.40 | 34.4 | pipelines/compile_ms 521, pipelines/front_ms 502, render/thread/ms 56.75, render/busy/ms 55.14, main/busy/ms 38.27, stage/render/prepare_assets 18.76, pipelines/driver_ms 18.26, stage/render/render 15.03, stage/main/post_update 14.68, stage/main/update 14.60, render/graph/ms 12.80, stage/render/graph_submit 8.77 |
| warm-up | 5.44 | 40.6 | pipelines/compile_ms 650, pipelines/front_ms 629, render/thread/ms 54.50, render/busy/ms 52.98, main/busy/ms 37.70, stage/render/prepare_assets 25.83, pipelines/driver_ms 21.48, stage/main/update 16.88, stage/render/render 12.85, stage/main/post_update 12.33, render/graph/ms 10.74, encode//sum 7.48 |
| warm-up | 5.47 | 34.7 | pipelines/compile_ms 815, pipelines/front_ms 794, render/thread/ms 56.13, render/busy/ms 54.55, main/busy/ms 40.22, stage/render/prepare_assets 28.16, pipelines/driver_ms 21.45, stage/main/update 14.33, stage/render/render 10.70, stage/main/post_update 9.54, stage/main/pre_update 9.22, render/graph/ms 8.46 |
| warm-up | 5.53 | 59.5 | pipelines/compile_ms 796, pipelines/front_ms 776, render/thread/ms 86.87, render/busy/ms 85.41, main/busy/ms 37.88, pipelines/driver_ms 20.13, pipelines/wait_ms 18.95, stage/render/prepare_write_phase_buffers 16.19, stage/render/prepare_resources 15.15, stage/render/prepare_assets 14.43, stage/render/render 11.54, stage/main/update 10.83 |
| warm-up | 5.58 | 46.6 | pipelines/compile_ms 960, pipelines/front_ms 938, render/thread/ms 104, render/busy/ms 102, main/busy/ms 53.10, stage/main/post_update 31.76, stage/render/prepare_write_phase_buffers 31.20, stage/render/prepare_resources 27.09, pipelines/driver_ms 22.63, pipelines/wait_ms 19.78, stage/main/update 14.83, stage/render/render 12.23 |
| warm-up | 5.62 | 41.6 | pipelines/compile_ms 1124, pipelines/front_ms 1098, pipelines/wait_ms 120, render/thread/ms 85.65, render/busy/ms 83.99, main/busy/ms 68.97, stage/render/prepare_resources 37.27, stage/main/post_update 36.76, pipelines/driver_ms 25.94, stage/main/update 22.74, stage/render/prepare_write_phase_buffers 19.05, stage/render/render 12.94 |
| warm-up | 5.65 | 29.0 | pipelines/compile_ms 1289, pipelines/front_ms 1252, pipelines/wait_ms 120, render/thread/ms 67.03, render/busy/ms 65.55, main/busy/ms 46.32, pipelines/driver_ms 37.09, stage/render/prepare_resources 23.21, stage/render/prepare_assets 20.06, stage/main/update 18.92, stage/main/post_update 17.71, stream/drive/ms 12.75 |
| warm-up | 5.69 | 26.4 | pipelines/compile_ms 1127, pipelines/front_ms 1097, pipelines/wait_ms 68.71, render/thread/ms 42.70, render/busy/ms 41.18, main/busy/ms 33.21, pipelines/driver_ms 30.41, stage/render/prepare_assets 17.38, stage/main/post_update 13.83, stage/main/update 12.91, stage/render/render 11.00, render/graph/ms 9.13 |
| warm-up | 5.78 | 39.5 | pipelines/compile_ms 203, pipelines/front_ms 199, render/thread/ms 54.81, render/busy/ms 53.15, pipelines/wait_ms 46.04, main/busy/ms 39.74, stage/render/prepare_assets 30.22, stage/main/update 18.16, stage/render/render 14.02, stage/main/post_update 13.56, render/graph/ms 11.74, stage/render/graph_submit 8.39 |
| warm-up | 5.82 | 36.1 | pipelines/compile_ms 509, pipelines/front_ms 494, pipelines/wait_ms 76.42, render/thread/ms 65.05, render/busy/ms 63.51, stage/render/prepare_assets 38.25, main/busy/ms 31.94, stage/main/update 15.59, pipelines/driver_ms 14.74, stage/render/render 12.55, render/graph/ms 10.51, stage/main/post_update 9.58 |
| warm-up | 5.87 | 42.8 | pipelines/compile_ms 349, pipelines/front_ms 338, render/thread/ms 48.79, render/busy/ms 45.85, main/busy/ms 25.87, stage/render/prepare_assets 21.27, stage/render/render 13.33, stage/main/update 11.89, pipelines/driver_ms 10.66, render/graph/ms 9.94, stage/main/post_update 8.78, encode//sum 8.19 |
| warm-up | 5.90 | 27.8 | pipelines/compile_ms 383, pipelines/front_ms 369, render/thread/ms 62.86, render/busy/ms 60.22, stage/render/prepare_assets 31.71, main/busy/ms 29.84, stage/render/render 14.19, pipelines/driver_ms 13.31, stage/main/update 12.79, render/graph/ms 11.04, stage/main/post_update 9.66, encode//sum 8.31 |
| warm-up | 5.95 | 46.4 | render/thread/ms 59.65, render/busy/ms 58.38, stage/render/prepare_assets 35.26, pipelines/compile_ms 33.93, main/busy/ms 32.81, pipelines/front_ms 31.29, stage/main/update 14.32, stage/main/post_update 9.90, stage/render/render 9.43, render/graph/ms 7.81, encode//sum 7.09, stage/render/graph_submit 5.50 |
| warm-up | 6.00 | 31.8 | pipelines/compile_ms 366, pipelines/front_ms 357, render/thread/ms 44.75, render/busy/ms 43.57, stage/render/prepare_assets 28.03, main/busy/ms 27.16, stage/main/update 12.70, stage/main/post_update 8.89, pipelines/driver_ms 8.67, stage/render/render 7.83, encode//sum 6.38, render/graph/ms 6.30 |
| warm-up | 6.03 | 26.9 | pipelines/compile_ms 162, pipelines/front_ms 158, render/thread/ms 49.54, render/busy/ms 47.82, main/busy/ms 39.11, stage/render/prepare_assets 22.30, stage/main/update 16.12, stage/main/post_update 15.01, stage/render/render 14.40, render/graph/ms 12.04, stage/render/graph_submit 8.99, encode//sum 6.73 |
| warm-up | 6.05 | 28.6 | pipelines/compile_ms 295, pipelines/front_ms 284, main/busy/ms 47.36, render/thread/ms 47.20, render/busy/ms 45.41, stage/main/update 23.81, stage/render/render 18.30, render/graph/ms 15.75, stage/main/post_update 14.79, stage/render/graph_submit 11.45, pipelines/driver_ms 11.09, stage/render/prepare_assets 9.33 |
| warm-up | 6.09 | 38.3 | pipelines/compile_ms 132, pipelines/front_ms 126, render/thread/ms 52.72, render/busy/ms 51.37, main/busy/ms 46.88, stage/main/update 23.09, stage/render/render 15.19, stage/main/post_update 14.18, stage/render/prepare_assets 13.55, render/graph/ms 13.06, encode//sum 9.23, stage/render/graph_submit 8.13 |
| warm-up | 6.17 | 54.6 | pipelines/compile_ms 181, pipelines/front_ms 175, main/busy/ms 49.55, render/thread/ms 47.61, render/busy/ms 46.10, stage/main/update 24.92, stage/main/post_update 18.01, stream/drive/ms 16.37, stage/render/render 16.30, stage/render/prepare_assets 14.76, render/graph/ms 13.92, stage/render/graph_submit 10.70 |
| warm-up | 6.20 | 35.5 | pipelines/compile_ms 195, pipelines/front_ms 188, render/thread/ms 59.75, render/busy/ms 58.20, main/busy/ms 50.84, stage/main/update 24.96, stage/render/render 21.33, render/graph/ms 18.85, stage/main/post_update 17.68, stage/render/prepare_assets 17.42, stage/render/graph_submit 13.37, encode//sum 10.03 |
| warm-up | 6.24 | 35.7 | pipelines/compile_ms 1387, pipelines/front_ms 1362, render/thread/ms 50.89, render/busy/ms 49.67, main/busy/ms 44.59, stage/main/update 25.48, pipelines/driver_ms 25.33, stage/render/prepare_assets 23.08, stage/render/render 14.87, pipelines/wait_ms 13.98, render/graph/ms 13.26, stage/main/post_update 10.87 |
| warm-up | 6.38 | 29.2 | main/busy/ms 36.97, render/thread/ms 25.90, render/busy/ms 24.68, stage/main/update 24.27, stream/drive/ms 16.99, stage/render/render 10.40, render/graph/ms 8.85, stage/main/post_update 8.09, stage/render/prepare_assets 7.48, encode//sum 7.31, stage/render/graph_submit 5.96, encode/submit 3.75 |
| warm-up | 6.44 | 25.2 | main/busy/ms 31.63, render/thread/ms 26.89, render/busy/ms 25.51, stage/main/update 16.22, stage/render/render 10.99, stage/main/post_update 10.39, stage/render/prepare_assets 10.19, render/graph/ms 9.19, encode//sum 7.27, stage/render/graph_submit 6.85, encode/submit 4.78, stream/drive/ms 3.29 |
| warm-up | 6.68 | 35.7 | main/busy/ms 38.17, render/thread/ms 37.93, render/busy/ms 36.30, stage/main/update 19.21, stage/render/render 15.67, stage/main/post_update 13.56, stage/render/prepare_assets 13.53, render/graph/ms 13.25, encode//sum 11.05, stream/drive/ms 9.42, stage/render/graph_submit 9.40, encode/submit 5.31 |
| warm-up | 6.71 | 35.1 | render/thread/ms 48.28, render/busy/ms 47.01, main/busy/ms 38.95, stage/render/prepare_assets 29.27, stage/main/update 17.69, stage/main/post_update 15.86, stage/render/render 11.13, stream/drive/ms 10.47, render/graph/ms 9.16, encode//sum 8.80, stage/render/graph_submit 6.40, encode/finish_wall 3.34 |
| warm-up | 7.05 | 31.2 | render/thread/ms 37.92, render/busy/ms 36.33, main/busy/ms 16.30, stage/render/render 11.15, render/graph/ms 9.89, stage/main/post_update 9.88, stage/render/specialize 9.12, stage/render/graph_submit 7.96, encode//sum 6.40, stage/render/queue 6.11, encode/submit 4.66, stage/render/prepare_meshes 4.05 |
| warm-up | 8.26 | 28.9 | render/thread/ms 35.39, render/busy/ms 32.74, main/busy/ms 14.54, stage/render/render 9.73, stage/render/specialize 8.61, render/graph/ms 8.40, stage/main/post_update 8.19, stage/render/graph_submit 6.47, stage/render/queue 5.69, encode//sum 5.47, stage/main/update 4.19, stage/render/prepare_meshes 3.85 |
| measure | 0.83 | 26.4 | render/thread/ms 33.76, render/busy/ms 31.10, main/busy/ms 12.94, stage/render/render 10.07, render/graph/ms 8.76, stage/render/specialize 8.70, stage/main/post_update 7.82, encode//sum 7.64, stage/render/graph_submit 6.96, stage/render/queue 5.45, encode/submit 4.17, stage/main/update 2.83 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set select.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=select-5 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 7%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 96%) · 2625 MHz (min 2625) · memory 8101 MHz · 97.6 W · 77 °C (max 79) · VRAM 6426 MiB · throttled 100% of samples

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
