# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.day_gate=on (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1547 frames over 15.0 s · **median 9.73 ms (103 fps)** · mean 9.70 · p95 11.07 · p99 11.74 (1% low 85 fps) · worst 27.05 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 278 frames over 8.5 s, worst 2596.1 ms
- hitches (> 25.0 ms): **1** measured, 49 in warm-up
- timed GPU passes: 8.59 ms of a 9.70 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.73 ms, limit 11 ms
- pass worst frame 27.05 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.935 | 2.938 | 2.978 | 1.00 | 0.062 |
| dlss_super_resolution | 2.018 | 2.021 | 2.043 | 1.00 | 0.181 |
| early prepass | 0.620 | 0.621 | 0.632 | 1.00 | 0.090 |
| atmosphere_luts | 0.325 | 0.326 | 0.341 | 1.00 | 0.003 |
| bloom | 0.314 | 0.314 | 0.331 | 1.00 | 0.061 |
| main_transmissive_pass_3d | 0.312 | 0.312 | 0.334 | 1.00 | 0.008 |
| water_velocity | 0.239 | 0.239 | 0.264 | 1.00 | 0.002 |
| water_sspr | 0.208 | 0.208 | 0.210 | 1.00 | 0.002 |
| ssao | 0.192 | 0.192 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.149 | 0.149 | 0.170 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.146 | 0.152 | 1.00 | 0.001 |
| water_fft | 0.131 | 0.131 | 0.135 | 1.00 | 0.003 |
| uniform_allocation | 0.117 | 0.117 | 0.158 | 1.00 | 0.020 |
| upscaling | 0.113 | 0.113 | 0.118 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.087 | 0.087 | 0.119 | 1.00 | 0.016 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.075 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.412 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.028 | 0.221 | 0.232 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 1.00 | 0.083 |
| water_snapshot | 0.024 | 0.024 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.045 | 0.047 | 0.32 | 0.003 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 1.00 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.014 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.773 | 10.162 | 25.74 | 13571.1 |
| main/busy/ms | 7.069 | 7.952 | 10.55 | 10936.0 |
| render/busy/ms | 6.843 | 7.791 | 25.06 | 10585.5 |
| stage/render/render | 4.571 | 5.368 | 8.52 | 7072.1 |
| stage/main/post_update | 4.218 | 4.953 | 6.53 | 6524.9 |
| render/graph/ms | 4.002 | 4.745 | 7.76 | 6191.4 |
| encode//sum | 3.746 | 4.880 | 9.07 | 5794.8 |
| stage/render/graph_submit | 3.021 | 3.714 | 6.56 | 4673.1 |
| encode/submit | 1.892 | 2.312 | 5.30 | 2926.7 |
| stage/main/update | 1.751 | 2.167 | 3.55 | 2709.0 |
| stage/render/prepare_views | 1.623 | 3.213 | 4.68 | 2510.4 |
| render/acquire/ms | 1.578 | 3.175 | 4.63 | 2441.4 |
| encode/finish_wall | 1.148 | 1.599 | 2.35 | 1776.3 |
| stage/render/camera_driver | 0.925 | 1.060 | 1.27 | 1431.6 |
| encode/pass:shadow_directional_light/sum | 0.748 | 1.306 | 2.03 | 1156.7 |
| encode/pass:early prepass/sum | 0.705 | 0.961 | 1.31 | 1090.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.697 | 0.965 | 1.26 | 1078.2 |
| stage/render/prepare_write_phase_buffers | 0.530 | 0.740 | 0.98 | 819.9 |
| stage/render/prepare_assets | 0.445 | 0.566 | 1.61 | 689.2 |
| stage/render/prepare_resources | 0.381 | 0.441 | 0.71 | 589.9 |
| bigspace/total/ms | 0.353 | 0.723 | 1.72 | 546.7 |
| render/present/ms | 0.352 | 0.417 | 1.27 | 544.1 |
| stage/main/pre_update | 0.337 | 0.428 | 0.91 | 521.2 |
| stage/main/first | 0.290 | 0.348 | 1.54 | 447.9 |
| terrain/update/ms | 0.232 | 0.298 | 0.65 | 358.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.231 | 0.326 | 0.61 | 357.2 |
| stage/render/prepare_bind_groups | 0.205 | 0.282 | 0.53 | 317.2 |
| terrain/select/ms | 0.202 | 0.259 | 0.62 | 312.4 |
| stage/render/prepare_meshes | 0.193 | 0.238 | 2.67 | 297.9 |
| stream/drive/ms | 0.183 | 0.240 | 0.58 | 282.9 |
| stage/main/fixed_main_loop | 0.176 | 0.322 | 0.90 | 272.5 |
| stage/render/prepare_batch_phases | 0.175 | 0.235 | 0.40 | 271.0 |
| stage/render/extract_commands | 0.168 | 0.203 | 0.46 | 260.3 |
| stage/render/create_views | 0.165 | 0.208 | 0.69 | 255.8 |
| far/select/ms | 0.160 | 0.197 | 0.56 | 247.0 |
| controller/feed/ms | 0.157 | 0.203 | 0.57 | 243.5 |
| stream/tick/ms | 0.155 | 0.207 | 0.55 | 240.2 |
| stage/render/prepare_flush | 0.153 | 0.280 | 0.63 | 236.0 |
| bigspace/high_precision/ms | 0.132 | 0.343 | 1.55 | 204.6 |
| controller/move/ms | 0.121 | 0.187 | 0.58 | 187.6 |
| stage/main/last | 0.119 | 0.157 | 2.59 | 184.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.115 | 0.180 | 0.46 | 178.6 |
| stage/render/queue | 0.107 | 0.135 | 5.61 | 165.2 |
| anim/evaluate/ms | 0.085 | 0.207 | 1.18 | 131.5 |
| stage/render/specialize | 0.081 | 0.101 | 8.31 | 125.5 |
| bigspace/low_precision/ms | 0.076 | 0.180 | 1.00 | 118.2 |
| anim/evaluate_cpu/ms | 0.061 | 0.075 | 0.49 | 94.1 |
| stage/render/prepare_collect_phase_buffers | 0.056 | 0.113 | 0.20 | 86.6 |
| encode/resolve_diagnostics/sum | 0.054 | 0.082 | 0.16 | 83.4 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.039 | 0.09 | 40.5 |
| physics/step/ms | 0.026 | 0.050 | 0.46 | 39.5 |
| weather/map/ms | 0.025 | 0.033 | 0.38 | 38.1 |
| encode/camera_driver_span_end/sum | 0.023 | 0.036 | 0.14 | 35.7 |
| stage/render/cleanup | 0.023 | 0.032 | 0.41 | 35.1 |
| stage/render/phase_sort | 0.020 | 0.036 | 0.11 | 30.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.03 | 16.3 |
| script/update/ms | 0.007 | 0.008 | 0.43 | 10.4 |
| controller/avoid/ms | 0.005 | 0.007 | 0.08 | 8.5 |
| script/input/ms | 0.005 | 0.006 | 0.16 | 7.9 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 6.8 |
| script/ui/ms | 0.004 | 0.005 | 0.38 | 6.1 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
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
| crowd/c1/count | count | 122 | 123 | 123 |
| crowd/c1/joints/count | count | 4848 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1080 | 1088 | 1088 |
| crowd/c2/count | count | 284 | 284 | 283 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.52 | 1.90 | 0.64 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.09 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.14 | 0.02 |
| encode/finish_critical | ms | 0.84 | 2.03 | 0.64 |
| encode/pass:early prepass/max | ms | 0.71 | 1.31 | 0.61 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.70 | 1.26 | 0.63 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.46 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.61 | 0.27 |
| encode/pass:shadow_directional_light/max | ms | 0.69 | 2.03 | 0.63 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.16 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.23 | 25.06 | 9.33 |
| gpu/timed/ms | ms | 9.23 | 10.81 | 9.33 |
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
| phases/opaque3d/batch_sets | count | 20 | 20 | 20 |
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 19 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 15 | 15 |
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
| preprocess/bind_groups_reused | count | 123 | 123 | 123 |
| process/cpu_usage | % | 26.54 | 37.44 | 24.52 |
| process/mem_usage | GiB | 4.67 | 4.71 | 4.71 |
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
| skin/blocks_changed/count | count | 4 | 6 | 4 |
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
| system/cpu_usage | % | 30.74 | 39.07 | 26.92 |
| system/mem_usage | % | 58.67 | 58.83 | 58.82 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 51 | 80 | 51 |
| views/all/bins | count | 5322 | 8059 | 5322 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 15.3 k | 27.0 k | 15.3 k |
| views/all/pipelines | count | 69 | 76 | 69 |
| views/all/pipelines_distinct | count | 61 | 61 | 61 |
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
| views/main/batch_sets | count | 50 | 50 | 50 |
| views/main/bins | count | 3170 | 3170 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 46 | 46 | 46 |
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
| anim/events/count | count | 0.10 | 3 | 151 |
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
| encode//count | count | 26.57 | 29 | 41.1 k |
| encode/bind_walk/init_textures | count | 7164 | 14.0 k | 11.08 M |
| encode/bind_walk/init_textures_skipped | count | 31.7 k | 44.8 k | 49.07 M |
| encode/bind_walk/inits | count | 341 | 433 | 528.1 k |
| encode/bind_walk/inits_skipped | count | 62.99 | 75 | 97.4 k |
| encode/bind_walk/merge_textures | count | 7100 | 13.9 k | 10.98 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.31 M |
| encode/bind_walk/merges | count | 82.25 | 103 | 127.2 k |
| encode/bind_walk/merges_skipped | count | 48.75 | 54 | 75.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1547 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1547 |
| encode/pass:early prepass/count | count | 1 | 1 | 1547 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1547 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1547 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1547 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1547 |
| encode/queue/command_buffers | count | 36.57 | 39 | 56.6 k |
| encode/queue/submits | count | 2 | 2 | 3094 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.65 M | 2.22 G |
| encode/queue/write_buffer/calls | count | 211 | 281 | 326.8 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.49 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.9 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1547 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 1 | 901 |
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

## Hitches (> 25.0 ms; slowest 50, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.60 | 2596.1 | main/busy/ms 2566, stage/main/post_update 2460, stage/main/update 97.37, surface/advance_cpu_ms 66.86, stage/main/pre_update 4.61, stage/main/last 1.03, surface/cell_upload_ms 0.88, stage/main/first 0.59, stage/main/fixed_main_loop 0.54, stream/drive/ms 0.47, stream/tick/ms 0.16, weather/map/ms 0.13 |
| warm-up | 3.35 | 753.9 | main/busy/ms 2709, stage/main/post_update 2484, render/thread/ms 724, render/busy/ms 550, stage/render/prepare_assets 500, stage/main/update 211, stage/render/prepare_views 168, render/acquire/ms 168, surface/advance_cpu_ms 66.86, stage/render/create_views 28.65, stage/render/render 16.59, render/graph/ms 9.77 |
| warm-up | 3.52 | 173.4 | render/thread/ms 878, render/busy/ms 616, stage/render/prepare_assets 551, pipelines/compile_ms 343, pipelines/front_ms 218, main/busy/ms 175, stage/render/prepare_views 168, render/acquire/ms 168, pipelines/driver_ms 125, stage/main/update 121, stage/render/render 110, render/present/ms 93.84 |
| warm-up | 3.79 | 248.1 | pipelines/wait_ms 4830, pipelines/compile_ms 2379, pipelines/front_ms 2221, pipelines/driver_ms 158, main/busy/ms 134, render/thread/ms 131, render/busy/ms 128, stage/render/prepare_assets 96.15, stage/main/update 61.70, stage/main/post_update 40.05, pipelines/lock_ms 23.06, stage/render/render 22.40 |
| warm-up | 3.89 | 101.8 | pipelines/wait_ms 4931, pipelines/compile_ms 3203, pipelines/front_ms 2903, pipelines/driver_ms 300, render/thread/ms 209, render/busy/ms 208, stage/render/prepare_assets 179, main/busy/ms 42.20, pipelines/lock_ms 25.60, stage/main/post_update 20.77, stage/render/render 20.61, render/graph/ms 18.51 |
| warm-up | 3.95 | 27.7 | pipelines/compile_ms 251, pipelines/front_ms 233, main/busy/ms 34.83, render/thread/ms 29.49, render/busy/ms 27.71, stage/main/post_update 19.26, pipelines/driver_ms 18.44, stage/render/render 11.32, stage/main/update 11.23, render/graph/ms 9.42, terrain/update/ms 9.22, encode//sum 8.91 |
| warm-up | 4.01 | 30.8 | pipelines/compile_ms 1149, pipelines/front_ms 1100, main/busy/ms 73.63, stage/main/update 53.39, render/thread/ms 51.01, pipelines/driver_ms 49.00, render/busy/ms 48.98, pipelines/wait_ms 48.03, stream/drive/ms 47.54, stage/render/prepare_assets 29.15, stage/main/post_update 16.14, stage/render/render 13.36 |
| warm-up | 4.08 | 74.9 | pipelines/compile_ms 684, pipelines/front_ms 665, render/thread/ms 69.79, render/busy/ms 68.30, main/busy/ms 64.00, stage/render/prepare_assets 53.39, stage/main/update 52.33, stream/drive/ms 48.18, pipelines/wait_ms 20.55, pipelines/driver_ms 19.18, stage/render/render 9.79, terrain/update/ms 8.03 |
| warm-up | 4.15 | 57.0 | pipelines/compile_ms 174, pipelines/front_ms 167, render/thread/ms 42.12, render/busy/ms 40.65, main/busy/ms 39.73, stage/main/update 25.56, stage/render/prepare_assets 24.95, stream/drive/ms 19.85, stage/render/render 11.11, render/graph/ms 8.94, stage/main/post_update 8.01, encode//sum 7.47 |
| warm-up | 4.18 | 34.3 | pipelines/compile_ms 362, pipelines/front_ms 349, render/thread/ms 58.26, render/busy/ms 56.37, main/busy/ms 41.92, stage/render/prepare_assets 40.02, stage/main/update 25.90, stream/drive/ms 20.21, pipelines/driver_ms 13.66, stage/render/render 12.06, render/graph/ms 9.51, stage/main/post_update 8.62 |
| warm-up | 4.26 | 81.8 | pipelines/compile_ms 756, pipelines/front_ms 732, render/thread/ms 86.26, render/busy/ms 84.30, stage/render/prepare_assets 63.70, main/busy/ms 50.79, stage/main/update 29.40, pipelines/driver_ms 23.77, stream/drive/ms 19.04, stage/render/render 15.33, render/graph/ms 13.12, stage/main/post_update 12.96 |
| warm-up | 4.30 | 34.6 | pipelines/compile_ms 694, pipelines/front_ms 670, main/busy/ms 100, render/thread/ms 86.72, render/busy/ms 84.90, stage/main/update 74.89, stage/render/prepare_assets 59.39, stream/drive/ms 38.54, pipelines/driver_ms 23.56, stage/render/render 16.65, stage/main/post_update 14.43, render/graph/ms 14.21 |
| warm-up | 4.48 | 177.2 | pipelines/compile_ms 1025, pipelines/front_ms 990, render/thread/ms 118, render/busy/ms 116, main/busy/ms 112, stage/render/prepare_assets 93.16, stage/main/update 86.60, stream/drive/ms 51.80, pipelines/driver_ms 35.60, stage/main/post_update 13.32, stage/render/render 12.43, render/graph/ms 9.81 |
| warm-up | 4.53 | 52.8 | pipelines/compile_ms 1148, pipelines/front_ms 1100, render/thread/ms 130, render/busy/ms 128, pipelines/wait_ms 102, stage/render/prepare_assets 102, main/busy/ms 87.61, pipelines/driver_ms 47.39, stage/main/update 39.48, stream/drive/ms 27.00, stage/main/post_update 15.29, stage/main/pre_update 14.39 |
| warm-up | 4.61 | 84.1 | pipelines/compile_ms 342, pipelines/front_ms 319, pipelines/wait_ms 157, main/busy/ms 94.95, render/thread/ms 86.97, render/busy/ms 85.06, stage/render/prepare_assets 48.08, stage/main/update 41.30, pipelines/driver_ms 23.15, stage/main/post_update 21.15, stream/drive/ms 14.46, stage/main/pre_update 14.21 |
| warm-up | 4.66 | 50.4 | pipelines/compile_ms 308, pipelines/front_ms 296, render/thread/ms 85.05, render/busy/ms 83.55, main/busy/ms 71.20, pipelines/wait_ms 58.64, stage/render/prepare_assets 46.70, stage/main/update 29.73, stage/main/post_update 17.42, pipelines/driver_ms 12.33, stage/main/pre_update 11.81, stage/render/render 9.53 |
| warm-up | 4.70 | 37.4 | pipelines/compile_ms 1494, pipelines/front_ms 1453, render/thread/ms 66.35, render/busy/ms 64.80, pipelines/wait_ms 51.90, pipelines/driver_ms 41.26, main/busy/ms 41.17, stage/render/prepare_assets 37.85, stage/main/pre_update 11.48, stage/main/update 11.46, stage/render/render 10.26, stage/main/fixed_main_loop 8.31 |
| warm-up | 4.79 | 39.8 | pipelines/compile_ms 699, pipelines/front_ms 678, main/busy/ms 82.59, pipelines/wait_ms 68.67, stage/main/update 65.00, stream/drive/ms 59.10, render/thread/ms 50.65, render/busy/ms 48.39, stage/render/render 28.29, render/graph/ms 25.48, pipelines/driver_ms 20.57, stage/render/graph_submit 14.44 |
| warm-up | 4.88 | 90.2 | pipelines/compile_ms 663, pipelines/front_ms 643, main/busy/ms 144, stage/main/update 121, stream/drive/ms 115, render/thread/ms 79.00, render/busy/ms 77.38, pipelines/wait_ms 71.49, stage/render/render 27.36, render/graph/ms 24.80, stage/render/prepare_assets 21.05, pipelines/driver_ms 20.37 |
| warm-up | 4.94 | 61.8 | pipelines/compile_ms 1240, pipelines/front_ms 1202, main/busy/ms 94.84, render/thread/ms 70.86, render/busy/ms 69.53, stage/main/update 66.86, stream/drive/ms 60.29, pipelines/wait_ms 49.51, pipelines/driver_ms 37.90, stage/render/prepare_assets 22.93, stage/render/prepare_resources 15.08, stage/main/pre_update 10.38 |
| warm-up | 4.97 | 29.6 | pipelines/compile_ms 1597, pipelines/front_ms 1550, render/thread/ms 52.07, render/busy/ms 50.53, main/busy/ms 49.06, pipelines/driver_ms 46.58, pipelines/wait_ms 24.04, stage/render/render 21.67, stage/main/post_update 21.09, render/graph/ms 19.56, encode//sum 18.50, stage/render/graph_submit 16.63 |
| warm-up | 5.06 | 69.9 | pipelines/compile_ms 1107, pipelines/front_ms 1079, pipelines/wait_ms 105, main/busy/ms 67.91, stage/main/update 49.56, render/thread/ms 48.67, render/busy/ms 46.89, pipelines/driver_ms 27.51, stage/render/prepare_assets 21.62, stream/drive/ms 13.23, stage/render/render 12.33, render/graph/ms 10.13 |
| warm-up | 5.10 | 38.4 | pipelines/compile_ms 834, pipelines/front_ms 814, pipelines/wait_ms 115, render/thread/ms 60.82, render/busy/ms 59.50, main/busy/ms 46.09, stage/render/prepare_assets 32.93, stage/main/update 31.80, pipelines/driver_ms 20.14, stage/render/render 9.99, render/graph/ms 8.16, stage/main/post_update 7.33 |
| warm-up | 5.17 | 27.6 | pipelines/compile_ms 685, pipelines/front_ms 663, main/busy/ms 37.69, render/thread/ms 33.09, render/busy/ms 31.65, stage/main/update 25.71, pipelines/driver_ms 22.23, stream/drive/ms 16.32, stage/render/render 12.23, render/graph/ms 10.12, stage/render/prepare_assets 8.38, stage/render/graph_submit 7.50 |
| warm-up | 5.20 | 27.4 | pipelines/compile_ms 722, pipelines/front_ms 697, render/thread/ms 41.89, render/busy/ms 40.54, main/busy/ms 35.44, pipelines/driver_ms 24.88, stage/main/update 15.11, stage/main/post_update 14.87, stage/render/render 10.60, stage/render/prepare_resources 10.54, render/graph/ms 8.67, encode//sum 6.80 |
| warm-up | 5.23 | 28.2 | pipelines/compile_ms 303, pipelines/front_ms 292, render/thread/ms 50.33, main/busy/ms 48.11, render/busy/ms 45.64, stage/main/post_update 23.18, stage/main/update 18.33, stage/render/render 15.59, pipelines/driver_ms 11.25, stage/render/prepare_resources 10.90, render/graph/ms 10.50, stage/render/graph_submit 7.44 |
| warm-up | 5.27 | 39.0 | pipelines/compile_ms 198, pipelines/front_ms 193, render/thread/ms 52.19, main/busy/ms 48.29, render/busy/ms 47.54, stage/main/update 20.98, stage/render/prepare_assets 19.13, stage/main/post_update 18.00, stage/render/render 14.52, stream/drive/ms 9.92, render/graph/ms 9.39, encode//sum 6.69 |
| warm-up | 5.32 | 54.9 | pipelines/compile_ms 816, pipelines/front_ms 791, render/thread/ms 69.70, render/busy/ms 68.34, main/busy/ms 45.95, stage/render/prepare_assets 34.83, pipelines/driver_ms 24.87, stage/main/post_update 20.41, stage/main/update 17.28, stream/drive/ms 11.43, stage/render/prepare_resources 10.76, stage/render/render 9.33 |
| warm-up | 5.35 | 32.3 | pipelines/compile_ms 831, pipelines/front_ms 807, render/thread/ms 72.45, render/busy/ms 70.78, main/busy/ms 42.27, stage/render/prepare_assets 24.87, pipelines/driver_ms 23.80, stage/main/post_update 20.65, stage/main/update 15.76, stage/render/prepare_bind_groups 11.95, stage/render/render 11.22, stage/render/prepare_resources 9.48 |
| warm-up | 5.40 | 48.8 | pipelines/compile_ms 1347, pipelines/front_ms 1306, render/thread/ms 65.46, render/busy/ms 63.97, pipelines/driver_ms 40.37, main/busy/ms 37.73, stage/render/prepare_assets 28.98, stage/main/update 17.91, stage/main/post_update 13.64, stage/render/render 11.49, stage/render/prepare_bind_groups 11.07, render/graph/ms 9.47 |
| warm-up | 5.43 | 31.4 | pipelines/compile_ms 1626, pipelines/front_ms 1576, render/thread/ms 59.41, render/busy/ms 58.13, pipelines/driver_ms 50.60, main/busy/ms 41.41, stage/render/prepare_assets 37.20, pipelines/wait_ms 35.05, stage/main/update 23.49, stream/drive/ms 12.93, stage/main/post_update 11.70, stage/render/render 10.22 |
| warm-up | 5.50 | 31.9 | render/thread/ms 36.52, render/busy/ms 35.36, main/busy/ms 28.23, stage/render/prepare_assets 18.62, pipelines/compile_ms 14.10, pipelines/front_ms 13.00, stage/main/update 12.76, stage/main/post_update 10.54, stage/render/render 8.01, encode//sum 6.90, render/graph/ms 6.49, stage/render/graph_submit 4.28 |
| warm-up | 5.53 | 27.1 | render/thread/ms 51.04, render/busy/ms 46.94, main/busy/ms 32.50, stage/render/prepare_assets 26.25, pipelines/compile_ms 14.10, stage/main/update 13.81, stage/main/post_update 13.01, pipelines/front_ms 13.00, stage/render/render 12.13, render/graph/ms 7.67, encode//sum 7.00, stage/render/graph_submit 5.56 |
| warm-up | 5.58 | 35.5 | pipelines/compile_ms 1292, pipelines/front_ms 1265, pipelines/wait_ms 65.57, render/thread/ms 40.04, render/busy/ms 38.16, main/busy/ms 32.55, pipelines/driver_ms 27.26, stage/render/prepare_assets 16.79, stage/main/update 15.80, stage/render/render 13.66, render/graph/ms 11.42, stage/main/post_update 10.12 |
| warm-up | 5.65 | 49.8 | render/thread/ms 63.42, render/busy/ms 62.21, stage/render/prepare_assets 32.87, main/busy/ms 26.74, stage/main/update 12.40, stage/render/render 10.26, render/graph/ms 8.60, stage/main/post_update 8.26, encode//sum 7.97, stage/render/graph_submit 5.57, stage/render/specialize 5.22, stage/render/prepare_meshes 3.95 |
| warm-up | 5.71 | 43.3 | pipelines/compile_ms 149, pipelines/front_ms 144, render/thread/ms 47.90, render/busy/ms 46.74, main/busy/ms 26.16, stage/render/prepare_assets 25.58, stage/main/update 11.49, stage/main/post_update 9.22, stage/render/render 8.82, render/graph/ms 7.27, encode//sum 7.10, stage/render/graph_submit 5.37 |
| warm-up | 5.76 | 29.7 | pipelines/compile_ms 155, pipelines/front_ms 150, render/thread/ms 44.18, render/busy/ms 43.07, main/busy/ms 27.73, stage/render/prepare_assets 22.97, stage/main/update 11.62, stage/main/post_update 10.61, stage/render/render 10.44, render/graph/ms 8.82, encode//sum 7.58, stage/render/graph_submit 5.95 |
| warm-up | 5.79 | 25.5 | render/thread/ms 46.75, render/busy/ms 45.43, stage/render/prepare_assets 28.97, main/busy/ms 27.88, stage/main/update 12.68, stage/render/render 9.87, stage/main/post_update 9.81, render/graph/ms 8.14, encode//sum 6.65, stage/render/graph_submit 6.09, stream/drive/ms 3.83, encode/submit 3.07 |
| warm-up | 5.82 | 35.4 | pipelines/compile_ms 148, pipelines/front_ms 139, render/thread/ms 52.30, render/busy/ms 50.20, main/busy/ms 47.23, stage/main/update 26.04, stage/render/prepare_assets 20.78, stage/render/render 13.65, stage/main/post_update 13.19, render/graph/ms 11.27, pipelines/driver_ms 8.42, encode//sum 8.33 |
| warm-up | 5.86 | 37.7 | pipelines/compile_ms 148, pipelines/front_ms 139, render/thread/ms 57.58, render/busy/ms 55.65, main/busy/ms 51.33, stage/main/update 25.85, stage/render/prepare_assets 21.58, stage/main/post_update 15.63, stage/render/render 13.68, render/graph/ms 11.36, encode//sum 8.99, pipelines/driver_ms 8.42 |
| warm-up | 5.89 | 28.9 | render/thread/ms 51.92, render/busy/ms 50.20, main/busy/ms 41.97, stage/render/prepare_assets 19.79, stage/main/post_update 18.51, stage/render/render 15.66, stage/main/update 15.08, render/graph/ms 13.36, encode//sum 10.86, stage/render/graph_submit 9.07, encode/submit 6.20, stream/drive/ms 5.24 |
| warm-up | 5.93 | 34.6 | main/busy/ms 54.60, render/thread/ms 46.05, render/busy/ms 44.08, stage/main/update 30.58, stream/drive/ms 19.81, stage/render/prepare_assets 16.95, stage/main/post_update 16.82, stage/render/render 16.57, render/graph/ms 13.93, encode//sum 11.29, stage/render/graph_submit 10.30, encode/submit 7.15 |
| warm-up | 5.98 | 49.8 | pipelines/compile_ms 575, pipelines/front_ms 555, main/busy/ms 59.94, render/thread/ms 50.67, render/busy/ms 48.74, stage/main/update 37.51, pipelines/driver_ms 20.57, stage/render/prepare_assets 20.34, stream/drive/ms 19.20, stage/render/render 15.82, stage/main/post_update 14.91, render/graph/ms 13.17 |
| warm-up | 6.00 | 25.3 | pipelines/compile_ms 990, pipelines/front_ms 960, main/busy/ms 48.34, render/thread/ms 48.06, render/busy/ms 46.33, pipelines/driver_ms 30.67, stage/main/update 26.67, stage/render/prepare_assets 16.76, stage/render/render 14.60, stage/main/post_update 13.68, render/graph/ms 12.23, stream/drive/ms 9.06 |
| warm-up | 6.19 | 39.9 | pipelines/compile_ms 353, pipelines/front_ms 344, main/busy/ms 46.18, render/thread/ms 33.02, render/busy/ms 31.50, stage/main/update 28.95, stream/drive/ms 20.95, stage/render/render 12.77, stage/main/post_update 11.32, render/graph/ms 10.63, stage/render/prepare_assets 9.68, pipelines/driver_ms 8.71 |
| warm-up | 6.28 | 31.9 | pipelines/compile_ms 369, pipelines/front_ms 359, main/busy/ms 37.56, render/thread/ms 32.54, render/busy/ms 31.04, stage/main/update 18.30, stage/render/prepare_assets 13.51, stage/main/post_update 12.92, stage/render/render 11.53, pipelines/driver_ms 9.98, render/graph/ms 9.52, encode//sum 7.97 |
| warm-up | 6.59 | 48.1 | main/busy/ms 49.87, render/thread/ms 46.23, render/busy/ms 44.48, pipelines/compile_ms 24.65, stage/render/prepare_assets 23.83, pipelines/front_ms 23.81, stage/main/update 22.80, stage/main/post_update 15.59, stage/render/render 13.55, stream/drive/ms 12.06, render/graph/ms 11.27, encode//sum 9.37 |
| warm-up | 6.94 | 33.3 | pipelines/compile_ms 140, pipelines/front_ms 137, render/thread/ms 39.80, render/busy/ms 37.96, main/busy/ms 15.56, stage/render/render 10.85, render/graph/ms 9.42, stage/render/specialize 9.30, stage/main/post_update 8.28, encode//sum 7.50, stage/render/graph_submit 7.41, stage/render/queue 6.61 |
| warm-up | 8.16 | 28.5 | render/thread/ms 35.31, render/busy/ms 32.07, main/busy/ms 14.70, stage/render/render 9.64, stage/main/post_update 8.71, render/graph/ms 8.35, stage/render/specialize 8.17, stage/render/graph_submit 6.56, encode//sum 5.81, stage/render/queue 5.75, stage/render/prepare_meshes 3.79, encode/submit 3.70 |
| measure | 0.84 | 27.0 | render/thread/ms 34.24, render/busy/ms 32.03, main/busy/ms 14.30, stage/render/render 10.63, render/graph/ms 9.26, stage/render/specialize 8.39, encode//sum 8.19, stage/main/post_update 7.77, stage/render/graph_submit 7.41, stage/render/queue 5.70, encode/submit 4.43, stage/main/update 3.65 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set shadow.lamp.day_gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=daygate-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 4%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 98.4 W · 77 °C (max 80) · VRAM 6425 MiB · throttled 100% of samples

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
- `shadow.lamp.day_gate=on (command line: --set)`
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
