# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: texvariety · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5837 frames over 65.6 s · **median 10.97 ms (91 fps)** · mean 11.25 · p95 13.88 · p99 19.46 (1% low 51 fps) · worst 52.23 ms · 107 spikes over 1.5× median · 50.3 stutters/min (work 51.2/min)
- warm-up: 322 frames over 14.4 s, worst 2677.2 ms
- hitches (> 27.4 ms): **34** measured, 129 in warm-up
- timed GPU passes: 6.53 ms of a 11.25 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.97 ms, limit 11 ms
- **FAIL** frame_ms: 55 stutters (> 1.5x moving median of 60) in 1.09 min = 50.27/min, limit 1/min
- **FAIL** worst frame 52.23 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.897 | 2.017 | 2.049 | 0.94 | 0.185 |
| main_opaque_pass_3d | 1.363 | 1.449 | 1.907 | 0.94 | 0.081 |
| early prepass | 0.464 | 0.494 | 0.729 | 0.94 | 0.094 |
| bloom | 0.290 | 0.308 | 0.315 | 0.94 | 0.068 |
| atmosphere_luts | 0.286 | 0.304 | 0.366 | 0.94 | 0.004 |
| water_velocity | 0.253 | 0.269 | 0.347 | 0.94 | 0.002 |
| water_sspr | 0.190 | 0.202 | 0.203 | 0.94 | 0.003 |
| ssao | 0.179 | 0.190 | 0.194 | 0.94 | 0.008 |
| atmos_froxel_inject | 0.138 | 0.146 | 0.148 | 0.94 | 0.002 |
| clouds_trace | 0.129 | 0.137 | 0.232 | 0.94 | 0.001 |
| water_fft | 0.122 | 0.130 | 0.138 | 0.94 | 0.003 |
| uniform_allocation | 0.121 | 0.129 | 0.180 | 0.94 | 0.029 |
| auxiliary_views | 0.115 | 0.123 | 0.492 | 0.94 | 0.143 |
| upscaling | 0.106 | 0.113 | 0.119 | 0.94 | 0.005 |
| clouds_composite | 0.099 | 0.105 | 0.108 | 0.94 | 0.002 |
| bin_unpacking | 0.098 | 0.104 | 0.142 | 0.94 | 0.021 |
| tonemapping | 0.080 | 0.085 | 0.086 | 0.94 | 0.004 |
| main_transmissive_pass_3d | 0.076 | 0.081 | 0.162 | 0.94 | 0.010 |
| clustering | 0.060 | 0.064 | 0.068 | 0.94 | 0.020 |
| render_sky | 0.048 | 0.051 | 0.052 | 0.94 | 0.002 |
| clouds_resolve | 0.045 | 0.048 | 0.049 | 0.94 | 0.001 |
| surface_exposure | 0.042 | 0.044 | 0.045 | 0.94 | 0.001 |
| surface_trail | 0.038 | 0.056 | 0.057 | 0.68 | 0.003 |
| lightprobe_irradiance_map | 0.038 | 0.413 | 0.416 | 0.09 | 0.001 |
| surface_exposure_far | 0.035 | 0.043 | 0.045 | 0.83 | 0.002 |
| clouds_shadow | 0.029 | 0.030 | 0.033 | 0.94 | 0.004 |
| early_mesh_preprocessing | 0.029 | 0.030 | 0.047 | 0.94 | 0.006 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.031 | 0.94 | 0.001 |
| water_snapshot | 0.027 | 0.029 | 0.033 | 0.94 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.94 | 0.003 |
| lightprobe_radiance_map | 0.020 | 0.218 | 0.227 | 0.09 | 0.005 |
| night_sky | 0.012 | 0.013 | 0.034 | 0.94 | 0.002 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 0.94 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.94 | 0.006 |
| main_transparent_pass_3d | 0.011 | 0.011 | 0.026 | 0.94 | 0.128 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.94 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.94 | 0.003 |
| lightprobe_copy | 0.001 | 0.014 | 0.015 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.007 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/compile_ms | 18.122 | 0.000 | 4694.75 | 105779.8 |
| pipelines/driver_ms | 16.086 | 0.000 | 4480.91 | 93895.0 |
| pipelines/wait_ms | 13.644 | 0.000 | 8962.45 | 79642.2 |
| render/thread/ms | 9.691 | 12.050 | 49.39 | 56563.6 |
| main/busy/ms | 9.438 | 11.335 | 32.46 | 55090.3 |
| render/busy/ms | 9.187 | 11.562 | 48.69 | 53624.6 |
| encode//sum | 6.170 | 8.197 | 36.14 | 36013.6 |
| stage/main/post_update | 5.776 | 7.196 | 22.85 | 33712.4 |
| stage/render/render | 5.304 | 6.759 | 32.49 | 30961.4 |
| render/graph/ms | 4.599 | 6.036 | 31.45 | 26844.9 |
| stage/render/graph_submit | 3.318 | 4.431 | 28.49 | 19365.1 |
| stage/main/update | 2.296 | 2.853 | 23.53 | 13399.3 |
| encode/submit | 2.130 | 2.988 | 25.82 | 12430.5 |
| pipelines/front_ms | 2.036 | 0.000 | 220.31 | 11884.8 |
| encode/finish_wall | 1.233 | 1.813 | 4.54 | 7195.5 |
| stage/render/camera_driver | 1.224 | 1.717 | 5.53 | 7145.2 |
| stage/render/prepare_write_phase_buffers | 0.824 | 1.222 | 3.33 | 4810.8 |
| stage/render/prepare_assets | 0.619 | 0.827 | 10.13 | 3611.2 |
| bigspace/total/ms | 0.509 | 1.045 | 2.50 | 2970.8 |
| stage/render/prepare_meshes | 0.491 | 0.625 | 8.03 | 2863.9 |
| stage/render/specialize | 0.478 | 0.645 | 15.67 | 2790.7 |
| stage/render/prepare_resources | 0.476 | 0.590 | 2.33 | 2776.1 |
| stage/main/first | 0.426 | 0.576 | 5.98 | 2488.4 |
| render/present/ms | 0.402 | 0.580 | 2.54 | 2346.5 |
| stage/main/pre_update | 0.387 | 0.578 | 3.19 | 2261.1 |
| stage/render/queue | 0.325 | 0.458 | 10.91 | 1899.5 |
| terrain/update/ms | 0.312 | 0.406 | 2.01 | 1819.3 |
| stage/render/prepare_bind_groups | 0.308 | 0.555 | 3.21 | 1795.7 |
| controller/feed/ms | 0.291 | 0.380 | 0.99 | 1697.4 |
| terrain/select/ms | 0.276 | 0.358 | 1.53 | 1611.7 |
| stream/drive/ms | 0.261 | 0.378 | 21.11 | 1521.3 |
| stage/main/fixed_main_loop | 0.235 | 0.407 | 1.52 | 1369.4 |
| stage/render/create_views | 0.216 | 0.291 | 1.60 | 1261.2 |
| far/select/ms | 0.215 | 0.316 | 0.65 | 1257.3 |
| bigspace/low_precision/ms | 0.207 | 0.557 | 1.55 | 1207.6 |
| stage/render/prepare_batch_phases | 0.206 | 0.300 | 1.23 | 1204.5 |
| stream/tick/ms | 0.201 | 0.285 | 0.86 | 1172.7 |
| stage/render/extract_commands | 0.187 | 0.229 | 0.51 | 1088.9 |
| stage/render/prepare_flush | 0.174 | 0.367 | 1.37 | 1014.5 |
| anim/evaluate/ms | 0.158 | 0.271 | 1.20 | 921.1 |
| anim/evaluate_cpu/ms | 0.155 | 0.202 | 0.64 | 904.9 |
| stage/render/prepare_views | 0.147 | 0.191 | 0.87 | 856.4 |
| bigspace/high_precision/ms | 0.137 | 0.482 | 1.52 | 801.8 |
| controller/move/ms | 0.136 | 0.263 | 0.56 | 794.8 |
| stage/main/last | 0.127 | 0.195 | 1.57 | 743.2 |
| render/acquire/ms | 0.102 | 0.132 | 0.78 | 592.6 |
| stage/render/prepare_collect_phase_buffers | 0.076 | 0.154 | 0.70 | 441.1 |
| physics/step/ms | 0.051 | 0.095 | 0.62 | 298.8 |
| encode/resolve_diagnostics/sum | 0.035 | 0.048 | 0.61 | 206.1 |
| physics/insert/ms | 0.034 | 0.368 | 1.33 | 195.6 |
| stage/render/cleanup | 0.033 | 0.099 | 0.67 | 194.8 |
| stage/render/phase_sort | 0.028 | 0.048 | 0.30 | 163.3 |
| weather/map/ms | 0.026 | 0.035 | 0.22 | 153.8 |
| encode/camera_driver_span_begin/sum | 0.017 | 0.025 | 0.53 | 100.6 |
| encode/camera_driver_span_end/sum | 0.013 | 0.022 | 0.59 | 75.5 |
| anim/apply/ms | 0.012 | 0.022 | 0.41 | 71.6 |
| weather/eval/ms | 0.012 | 0.015 | 0.18 | 67.7 |
| controller/avoid/ms | 0.012 | 0.015 | 0.14 | 67.2 |
| script/update/ms | 0.008 | 0.010 | 0.13 | 46.6 |
| stream/despawn/ms | 0.005 | 0.000 | 1.14 | 31.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 30.9 |
| script/input/ms | 0.005 | 0.007 | 0.24 | 30.0 |
| script/ui/ms | 0.005 | 0.007 | 0.08 | 29.4 |
| controller/write/ms | 0.004 | 0.005 | 0.17 | 21.8 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.63 | 14.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.05 | 6.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.58 | 5.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.37 | 4.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.10 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1272 | 2262 | 583 |
| crowd/c0/meshes/count | count | 348 | 646 | 162 |
| crowd/c1/count | count | 146 | 157 | 148 |
| crowd/c1/joints/count | count | 5531 | 6037 | 5613 |
| crowd/c1/meshes/count | count | 1167 | 1311 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4621 | 10.3 k | 6528 |
| crowd/joints_uploaded/count | count | 4608 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.85 | 3.45 | 0.63 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.53 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.59 | 0.01 |
| encode/finish_critical | ms | 0.85 | 3.45 | 0.63 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.61 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.63 | 48.69 | 8.02 |
| gpu/timed/ms | ms | 7.63 | 11.18 | 8.02 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 750 | 763 | 755 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 803 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 803 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1285 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3379 | 5112 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1285 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3379 | 5112 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 14 | 200 | 4 |
| phases/shadow/bins | count | 5096 | 8733 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.5 k | 26.4 k | 6376 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 207 | 259 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 249 | 329 | 329 |
| pipelines/waiting | count | 1 | 11 | 0 |
| precache/draw_waiting | count | 0 | 10 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 1 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 36 | 76 | 0 |
| preprocess/bind_groups | count | 13 | 557 | 6 |
| preprocess/bind_groups_created | count | 13 | 557 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 127 | 214 | 123 |
| process/cpu_usage | % | 32.21 | 87.98 | 33.65 |
| process/mem_usage | GiB | 4.22 | 4.47 | 4.44 |
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
| skin/blocks_changed/count | count | 13 | 133 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 201 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 368.9 k | 811.5 k | 497.2 k |
| skin/joints_written/count | count | 5764 | 12.7 k | 7768 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4991 | 5084 | 5084 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 40.02 | 96.34 | 36.24 |
| system/mem_usage | % | 69.21 | 70.54 | 70.22 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 1447 | 1452 | 1451 |
| textures/images | count | 820 | 830 | 825 |
| views/all/batch_sets | count | 63 | 247 | 54 |
| views/all/bins | count | 7654 | 11.6 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 252 | 306 | 75 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 10.8 k | 35.6 k | 8119 |
| views/cascade/batch_sets | count | 9 | 54 | 4 |
| views/cascade/bins | count | 2790 | 6153 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8936 | 23.5 k | 6376 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 410 | 23.5 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 192 | 0 |
| views/lamp_face/bins | count | 2228 | 2732 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2475 | 3027 | 0 |
| views/lamp_face/pipelines | count | 171 | 208 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2903 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2876 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8366 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8534 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 48.50 | 199 | 283.1 k |
| anim/events/count | count | 0.10 | 3 | 565 |
| assets/Image/added | count | 0.11 | 42 | 613 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.15 | 86 | 852 |
| assets/Mesh/added | count | 0.70 | 152 | 4061 |
| assets/Mesh/removed | count | 0.83 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.70 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 51.95 | 3684 | 303.2 k |
| ecs/entities_delta | count | 0.80 | 1272 | 4674 |
| encode//count | count | 38.77 | 273 | 226.3 k |
| encode/bind_walk/init_textures | count | 23.2 k | 55.3 k | 135.68 M |
| encode/bind_walk/init_textures_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/inits | count | 564 | 1232 | 3.29 M |
| encode/bind_walk/inits_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/merge_textures | count | 5483 | 34.0 k | 32.01 M |
| encode/bind_walk/merge_textures_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/merges | count | 182 | 411 | 1.06 M |
| encode/bind_walk/merges_skipped | count | 0 | 0 | 0 |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5837 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5837 |
| encode/queue/command_buffers | count | 43.78 | 278 | 255.6 k |
| encode/queue/submits | count | 2.02 | 4 | 11.8 k |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 4.89 M | 11.02 G |
| encode/queue/write_buffer/calls | count | 253 | 831 | 1.48 M |
| encode/queue/write_buffer_with/bytes | bytes | 178.6 k | 3.52 M | 1.04 G |
| encode/queue/write_buffer_with/calls | count | 11.07 | 465 | 64.6 k |
| encode/queue/write_texture/bytes | bytes | 10.7 k | 8.39 M | 62.48 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 562 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5837 |
| physics/inserts | count | 0.39 | 88 | 2286 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.67 | 3 | 3938 |
| physics/steps_dropped | count | 0.00 | 1 | 1 |
| pipelines/after_reveal | count | 0.00 | 1 | 11 |
| pipelines/created | count | 0.02 | 2 | 123 |
| precache/hit | count | 0.00 | 2 | 3 |
| precache/missed | count | 0.00 | 2 | 7 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5800 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 28 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1272 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.4 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.68 | 2677.2 | main/busy/ms 2647, stage/main/post_update 2536, stage/main/update 101, surface/advance_cpu_ms 70.84, stage/main/pre_update 5.51, stage/main/last 1.50, surface/cell_upload_ms 0.78, stage/main/first 0.69, stream/drive/ms 0.44, stage/main/fixed_main_loop 0.38, weather/map/ms 0.11, stream/tick/ms 0.11 |
| warm-up | 3.43 | 750.3 | main/busy/ms 2792, stage/main/post_update 2567, render/thread/ms 715, render/busy/ms 548, stage/render/prepare_assets 501, stage/main/update 210, stage/render/prepare_views 163, render/acquire/ms 163, surface/advance_cpu_ms 70.84, stage/render/create_views 26.73, stage/render/render 14.41, render/graph/ms 8.43 |
| warm-up | 3.62 | 190.3 | render/thread/ms 884, render/busy/ms 620, stage/render/prepare_assets 557, pipelines/compile_ms 390, pipelines/driver_ms 301, main/busy/ms 183, stage/render/prepare_views 163, render/acquire/ms 163, stage/main/update 116, stage/render/render 115, render/present/ms 101, pipelines/front_ms 89.09 |
| warm-up | 3.78 | 132.2 | pipelines/compile_ms 1340, pipelines/driver_ms 1126, pipelines/wait_ms 257, pipelines/front_ms 213, main/busy/ms 135, render/thread/ms 133, render/busy/ms 129, stage/main/update 84.63, stage/render/prepare_assets 47.55, stage/render/prepare_resources 45.40, stage/main/post_update 43.09, stage/render/render 19.00 |
| warm-up | 3.90 | 116.7 | pipelines/compile_ms 2098, pipelines/driver_ms 1750, pipelines/wait_ms 1649, pipelines/front_ms 348, render/thread/ms 218, render/busy/ms 216, main/busy/ms 129, stage/render/prepare_assets 115, stage/main/update 84.66, stage/render/prepare_resources 55.50, stage/main/post_update 34.44, stage/render/prepare_bind_groups 16.61 |
| warm-up | 4.01 | 48.3 | pipelines/wait_ms 758, pipelines/compile_ms 466, pipelines/driver_ms 386, render/thread/ms 80.68, pipelines/front_ms 79.87, render/busy/ms 78.81, main/busy/ms 35.76, stage/render/prepare_resources 30.72, stage/main/post_update 22.03, stage/render/prepare_assets 17.44, stage/render/prepare_bind_groups 11.50, terrain/update/ms 8.92 |
| warm-up | 4.10 | 66.6 | pipelines/wait_ms 812, pipelines/compile_ms 229, pipelines/driver_ms 135, pipelines/front_ms 94.00, render/thread/ms 84.79, render/busy/ms 83.09, main/busy/ms 54.41, stage/render/prepare_assets 27.66, stage/render/prepare_bind_groups 22.41, stage/main/update 21.15, stage/main/post_update 19.13, stage/render/prepare_resources 13.31 |
| warm-up | 4.18 | 73.9 | pipelines/wait_ms 592, pipelines/compile_ms 208, render/thread/ms 136, render/busy/ms 134, pipelines/driver_ms 118, main/busy/ms 102, pipelines/front_ms 89.78, stage/main/update 81.11, stream/drive/ms 52.25, stage/render/prepare_resources 35.97, stage/render/prepare_assets 28.00, stage/render/prepare_flush 27.10 |
| warm-up | 4.28 | 107.5 | pipelines/wait_ms 198, render/thread/ms 165, render/busy/ms 163, main/busy/ms 142, pipelines/compile_ms 136, pipelines/driver_ms 113, stage/render/prepare_assets 69.54, stage/main/update 68.02, stage/main/post_update 56.80, stream/drive/ms 52.09, stage/render/prepare_resources 28.69, stage/render/prepare_flush 26.35 |
| warm-up | 4.34 | 58.9 | pipelines/wait_ms 198, render/thread/ms 148, render/busy/ms 146, pipelines/compile_ms 136, pipelines/driver_ms 113, main/busy/ms 97.49, stage/render/prepare_assets 79.37, stage/main/post_update 56.28, stage/render/prepare_resources 26.00, pipelines/front_ms 22.67, stage/main/update 15.51, stage/main/pre_update 15.13 |
| warm-up | 4.42 | 74.0 | render/thread/ms 125, render/busy/ms 118, main/busy/ms 68.85, stage/render/prepare_assets 50.01, stage/render/prepare_resources 23.07, stage/main/post_update 20.50, stage/main/pre_update 20.49, stage/main/update 18.04, stage/render/prepare_flush 13.24, stage/render/prepare_batch_phases 12.61, stream/drive/ms 10.16, stage/render/render 8.63 |
| warm-up | 4.50 | 85.4 | render/thread/ms 153, render/busy/ms 146, main/busy/ms 132, stage/main/update 57.48, stage/main/post_update 52.09, stage/render/prepare_assets 47.67, stage/render/prepare_batch_phases 42.29, stream/drive/ms 27.55, stage/render/prepare_flush 26.23, bigspace/total/ms 20.91, stage/main/pre_update 17.91, stage/render/render 14.19 |
| warm-up | 4.67 | 169.8 | render/thread/ms 223, render/busy/ms 222, main/busy/ms 135, stage/render/prepare_assets 62.04, stage/render/prepare_write_phase_buffers 57.23, stage/main/update 54.31, stage/main/post_update 39.72, stage/render/prepare_batch_phases 29.93, stage/render/prepare_flush 29.18, stream/drive/ms 28.32, stage/main/last 26.12, stage/render/prepare_resources 21.58 |
| warm-up | 4.77 | 102.7 | render/thread/ms 224, render/busy/ms 222, main/busy/ms 109, stage/render/prepare_assets 96.78, stage/render/prepare_write_phase_buffers 61.41, stage/main/update 37.37, stage/render/prepare_resources 31.61, stage/main/last 26.11, stage/main/post_update 25.64, stage/render/prepare_flush 14.25, stage/main/fixed_main_loop 10.84, stage/render/render 9.09 |
| warm-up | 4.86 | 81.9 | render/thread/ms 146, render/busy/ms 144, main/busy/ms 91.26, stage/render/prepare_assets 72.61, stage/main/update 42.27, stage/main/post_update 34.50, stage/render/prepare_resources 26.17, stage/render/prepare_batch_phases 9.69, stage/render/prepare_flush 8.80, stage/render/render 8.34, stage/render/prepare_write_phase_buffers 7.29, bigspace/total/ms 6.09 |
| warm-up | 4.93 | 69.8 | pipelines/wait_ms 1684, pipelines/compile_ms 847, pipelines/driver_ms 764, render/thread/ms 124, render/busy/ms 122, pipelines/front_ms 83.47, main/busy/ms 82.80, stage/main/post_update 44.76, stage/render/prepare_assets 39.76, stage/render/prepare_resources 24.20, stage/main/update 23.47, stage/render/prepare_write_phase_buffers 13.68 |
| warm-up | 4.99 | 64.1 | pipelines/wait_ms 2962, pipelines/compile_ms 887, pipelines/driver_ms 804, render/thread/ms 110, render/busy/ms 108, pipelines/front_ms 83.47, main/busy/ms 73.55, stage/main/post_update 39.45, stage/render/prepare_assets 34.61, stage/main/update 20.80, stage/render/prepare_resources 19.43, encode//sum 15.85 |
| warm-up | 5.09 | 95.2 | pipelines/compile_ms 1604, pipelines/driver_ms 1546, pipelines/wait_ms 1278, render/thread/ms 141, render/busy/ms 138, main/busy/ms 77.66, encode//sum 58.28, pipelines/front_ms 58.11, stage/main/post_update 53.95, stage/render/prepare_assets 27.01, stage/render/render 23.44, stage/render/create_views 23.41 |
| warm-up | 5.19 | 101.8 | pipelines/compile_ms 1650, pipelines/driver_ms 1554, pipelines/wait_ms 1121, render/thread/ms 189, render/busy/ms 187, main/busy/ms 102, pipelines/front_ms 95.63, encode//sum 71.77, stage/main/post_update 65.94, stage/render/prepare_resources 36.42, stage/render/render 29.67, render/graph/ms 26.13 |
| warm-up | 5.24 | 53.1 | pipelines/wait_ms 3993, pipelines/compile_ms 363, pipelines/driver_ms 290, render/thread/ms 140, render/busy/ms 139, pipelines/front_ms 72.56, main/busy/ms 66.57, stage/render/prepare_assets 42.06, stage/main/post_update 29.51, encode//sum 27.55, stage/render/prepare_resources 26.26, stage/render/prepare_flush 19.87 |
| warm-up | 5.35 | 105.8 | pipelines/wait_ms 4467, pipelines/compile_ms 317, pipelines/driver_ms 269, render/thread/ms 146, render/busy/ms 145, main/busy/ms 111, stage/main/update 67.02, stream/drive/ms 55.50, stage/render/prepare_assets 52.09, pipelines/front_ms 48.02, stage/main/post_update 30.57, stage/render/prepare_write_phase_buffers 21.78 |
| warm-up | 5.48 | 130.3 | pipelines/wait_ms 6782, render/thread/ms 230, render/busy/ms 228, pipelines/compile_ms 219, pipelines/driver_ms 164, main/busy/ms 151, stage/main/update 107, stage/render/prepare_resources 70.54, stream/drive/ms 58.59, pipelines/front_ms 54.89, stage/render/prepare_write_phase_buffers 46.50, stage/render/prepare_assets 38.14 |
| warm-up | 5.62 | 140.7 | pipelines/wait_ms 5187, render/thread/ms 248, render/busy/ms 246, pipelines/compile_ms 180, main/busy/ms 148, pipelines/driver_ms 138, stage/main/update 113, stage/render/prepare_resources 81.24, stream/drive/ms 66.47, stage/render/prepare_assets 58.68, pipelines/front_ms 41.91, stage/render/prepare_flush 30.83 |
| warm-up | 5.74 | 124.0 | render/thread/ms 221, render/busy/ms 219, main/busy/ms 147, stage/main/update 95.14, stage/render/prepare_assets 89.72, stream/drive/ms 64.17, stage/render/prepare_resources 39.32, stage/main/post_update 32.01, stage/render/prepare_bind_groups 27.44, stage/render/prepare_flush 20.28, stage/render/render 13.70, stage/render/specialize 11.32 |
| warm-up | 5.88 | 140.5 | render/thread/ms 231, render/busy/ms 228, main/busy/ms 183, stage/main/update 112, stage/render/prepare_assets 56.06, stage/main/post_update 53.17, stream/drive/ms 50.33, stage/render/prepare_write_phase_buffers 47.10, stage/render/prepare_flush 43.92, stage/render/prepare_resources 37.66, stage/render/render 16.64, render/graph/ms 13.27 |
| warm-up | 5.96 | 83.0 | render/thread/ms 191, render/busy/ms 190, main/busy/ms 163, stage/main/update 102, stage/render/prepare_write_phase_buffers 51.18, stream/drive/ms 49.68, stage/render/prepare_assets 45.74, stage/main/post_update 34.77, stage/render/prepare_flush 32.74, stage/render/prepare_resources 29.10, stage/render/render 14.61, stage/main/fixed_main_loop 13.31 |
| warm-up | 6.03 | 62.6 | render/thread/ms 116, render/busy/ms 114, main/busy/ms 105, stage/main/update 64.10, stage/render/prepare_assets 42.56, stream/drive/ms 37.99, stage/render/prepare_batch_phases 15.53, stage/main/pre_update 14.02, stage/render/render 13.20, stage/main/fixed_main_loop 12.58, stage/render/prepare_resources 12.16, stage/render/prepare_write_phase_buffers 11.24 |
| warm-up | 6.11 | 87.4 | render/thread/ms 117, render/busy/ms 115, main/busy/ms 96.78, stage/main/update 60.29, stream/drive/ms 38.13, stage/render/prepare_assets 35.57, stage/main/pre_update 19.73, stage/render/prepare_batch_phases 16.51, stage/render/prepare_flush 13.88, stage/render/prepare_resources 13.41, stage/render/render 12.80, stage/render/prepare_bind_groups 11.84 |
| warm-up | 6.35 | 106.4 | render/thread/ms 134, render/busy/ms 132, main/busy/ms 75.09, stage/main/post_update 50.85, stage/render/prepare_resources 38.28, bigspace/total/ms 27.57, stage/render/prepare_flush 19.43, stage/render/prepare_write_phase_buffers 19.21, stage/render/prepare_meshes 17.09, bigspace/low_precision/ms 16.45, stage/main/update 15.96, stage/render/prepare_assets 12.84 |
| warm-up | 6.49 | 52.2 | pipelines/wait_ms 7662, pipelines/compile_ms 3677, pipelines/driver_ms 3510, pipelines/front_ms 167, render/thread/ms 87.40, render/busy/ms 85.69, main/busy/ms 54.52, stage/render/prepare_assets 35.64, stage/main/post_update 20.84, stage/render/prepare_resources 19.38, stage/main/update 13.77, stage/render/render 11.88 |
| warm-up | 6.60 | 56.6 | pipelines/wait_ms 2941, pipelines/compile_ms 2677, pipelines/driver_ms 2397, pipelines/front_ms 279, render/thread/ms 97.25, render/busy/ms 95.78, main/busy/ms 71.39, stage/main/post_update 49.58, bigspace/total/ms 32.54, stage/render/prepare_assets 30.70, stage/render/prepare_resources 23.20, bigspace/high_precision/ms 19.45 |
| warm-up | 6.67 | 49.5 | pipelines/wait_ms 5942, render/thread/ms 73.26, render/busy/ms 71.70, main/busy/ms 62.67, stage/main/update 38.51, stage/render/prepare_assets 33.19, pipelines/compile_ms 25.07, pipelines/driver_ms 22.28, stage/main/post_update 15.35, stage/render/render 11.40, render/graph/ms 8.91, stream/drive/ms 8.49 |
| warm-up | 6.78 | 64.0 | pipelines/compile_ms 3399, pipelines/driver_ms 3202, pipelines/wait_ms 1768, pipelines/front_ms 197, render/thread/ms 101, render/busy/ms 99.44, main/busy/ms 81.37, stage/main/post_update 46.29, stage/render/prepare_assets 30.79, stage/main/update 24.32, stage/render/prepare_resources 17.26, stream/drive/ms 16.49 |
| warm-up | 6.88 | 51.9 | pipelines/wait_ms 3591, pipelines/compile_ms 224, pipelines/driver_ms 165, render/thread/ms 87.78, render/busy/ms 85.93, pipelines/front_ms 59.07, main/busy/ms 57.95, stage/render/prepare_assets 34.39, stage/main/post_update 33.19, stage/main/update 15.43, stage/render/prepare_resources 13.93, bigspace/total/ms 11.78 |
| warm-up | 6.93 | 54.6 | pipelines/wait_ms 1861, pipelines/compile_ms 120, render/thread/ms 95.49, render/busy/ms 93.89, pipelines/driver_ms 91.74, main/busy/ms 43.70, stage/render/prepare_assets 40.82, pipelines/front_ms 27.93, stage/main/post_update 24.62, stage/render/prepare_flush 15.84, bigspace/total/ms 13.51, bigspace/high_precision/ms 11.63 |
| warm-up | 7.00 | 64.9 | pipelines/wait_ms 1980, pipelines/compile_ms 125, render/thread/ms 110, render/busy/ms 108, pipelines/driver_ms 91.85, stage/render/prepare_assets 58.54, main/busy/ms 55.11, stage/main/post_update 38.46, pipelines/front_ms 32.69, stage/render/prepare_resources 12.06, stage/render/render 11.65, stage/render/prepare_flush 11.65 |
| warm-up | 7.86 | 54.1 | pipelines/wait_ms 15.9 k, pipelines/compile_ms 127, pipelines/driver_ms 102, render/thread/ms 83.81, render/busy/ms 82.37, main/busy/ms 40.30, stage/render/prepare_resources 30.50, pipelines/front_ms 25.63, stage/main/update 21.54, stage/render/prepare_assets 19.14, stage/render/prepare_write_phase_buffers 13.58, stage/main/post_update 12.58 |
| warm-up | 7.91 | 51.4 | pipelines/wait_ms 16.0 k, pipelines/compile_ms 381, pipelines/driver_ms 308, render/thread/ms 97.94, render/busy/ms 96.40, pipelines/front_ms 73.01, main/busy/ms 58.62, stage/main/update 36.29, stage/render/prepare_assets 24.64, stage/render/specialize 19.75, stage/render/prepare_resources 16.21, stage/main/post_update 14.54 |
| warm-up | 8.03 | 71.1 | pipelines/wait_ms 25.4 k, pipelines/compile_ms 331, pipelines/driver_ms 251, render/thread/ms 112, render/busy/ms 111, main/busy/ms 92.05, pipelines/front_ms 79.83, stage/main/post_update 59.88, stage/render/prepare_resources 31.90, stage/main/update 19.49, stage/render/prepare_assets 19.06, stage/render/prepare_write_phase_buffers 15.74 |
| warm-up | 8.16 | 51.7 | pipelines/compile_ms 4557, pipelines/wait_ms 4376, pipelines/driver_ms 4185, pipelines/front_ms 372, render/thread/ms 87.98, render/busy/ms 86.32, main/busy/ms 47.96, stage/render/prepare_assets 26.49, stage/main/post_update 22.78, stage/render/render 15.89, stage/main/update 15.42, render/graph/ms 13.40 |
| warm-up | 8.28 | 48.6 | pipelines/wait_ms 5148, pipelines/compile_ms 866, pipelines/driver_ms 742, pipelines/front_ms 124, render/thread/ms 75.48, render/busy/ms 73.77, main/busy/ms 50.23, stage/main/post_update 21.16, stage/render/prepare_resources 19.60, stage/main/update 17.06, stage/render/render 14.85, render/graph/ms 12.43 |
| warm-up | 8.35 | 61.7 | pipelines/wait_ms 9787, pipelines/compile_ms 908, pipelines/driver_ms 772, pipelines/front_ms 135, render/thread/ms 103, render/busy/ms 101, main/busy/ms 70.90, stage/main/post_update 27.78, stage/render/prepare_flush 27.59, stage/render/prepare_resources 25.34, stage/main/update 23.68, stage/render/render 15.41 |
| warm-up | 8.44 | 51.3 | pipelines/wait_ms 4681, render/thread/ms 87.96, render/busy/ms 86.55, main/busy/ms 56.73, pipelines/compile_ms 47.62, pipelines/driver_ms 38.79, stage/main/post_update 27.87, stage/render/prepare_assets 27.62, stage/main/update 17.16, stage/render/render 13.28, stage/render/prepare_write_phase_buffers 12.80, stage/render/prepare_resources 12.70 |
| warm-up | 8.51 | 73.2 | pipelines/wait_ms 9579, pipelines/compile_ms 141, pipelines/driver_ms 124, render/thread/ms 118, render/busy/ms 117, main/busy/ms 74.90, stage/render/prepare_assets 36.59, stage/main/post_update 35.48, stage/main/update 31.92, stage/render/prepare_write_phase_buffers 25.57, pipelines/front_ms 17.47, stage/render/prepare_resources 15.51 |
| warm-up | 8.57 | 59.8 | pipelines/wait_ms 19.3 k, pipelines/compile_ms 156, pipelines/driver_ms 125, render/thread/ms 115, render/busy/ms 114, main/busy/ms 77.19, stage/main/post_update 38.58, stage/render/prepare_assets 37.82, pipelines/front_ms 31.32, stage/main/update 30.16, stage/render/prepare_write_phase_buffers 22.35, stage/render/prepare_resources 14.63 |
| warm-up | 8.67 | 50.9 | render/thread/ms 88.55, render/busy/ms 87.14, main/busy/ms 51.44, stage/main/update 27.24, stage/render/prepare_assets 25.17, stage/main/post_update 15.61, stage/render/prepare_bind_groups 12.97, stage/render/render 12.86, stage/render/prepare_meshes 11.10, encode//sum 10.76, render/graph/ms 10.47, stage/render/prepare_resources 7.26 |
| warm-up | 8.72 | 54.4 | render/thread/ms 95.03, render/busy/ms 93.59, main/busy/ms 58.69, stage/render/prepare_assets 28.29, stage/main/update 25.91, stage/main/post_update 20.06, stage/render/render 12.29, encode//sum 11.22, stage/render/prepare_meshes 11.21, stage/render/prepare_flush 10.64, render/graph/ms 9.69, stage/render/prepare_write_phase_buffers 8.55 |
| warm-up | 8.81 | 82.1 | render/thread/ms 125, render/busy/ms 123, main/busy/ms 66.97, stage/render/prepare_resources 38.45, stage/main/update 30.09, stage/render/prepare_flush 27.30, stage/main/post_update 22.72, stream/drive/ms 19.87, stage/render/prepare_assets 15.99, stage/render/render 13.53, encode//sum 11.42, render/graph/ms 11.05 |
| warm-up | 8.89 | 85.2 | render/thread/ms 160, render/busy/ms 158, main/busy/ms 118, stage/render/prepare_resources 81.84, stage/main/post_update 65.38, stage/main/update 34.70, anim/evaluate/ms 20.96, stream/drive/ms 19.72, stage/render/prepare_flush 19.17, stage/render/render 15.11, stage/render/prepare_write_phase_buffers 12.88, render/graph/ms 12.78 |
| warm-up | 8.95 | 57.8 | pipelines/wait_ms 1240, pipelines/compile_ms 780, pipelines/driver_ms 681, render/thread/ms 135, render/busy/ms 133, main/busy/ms 109, pipelines/front_ms 98.62, stage/main/post_update 67.95, stage/render/prepare_resources 55.23, stage/render/prepare_assets 26.89, anim/evaluate/ms 21.16, stage/main/update 20.13 |
| warm-up | 9.08 | 58.5 | pipelines/wait_ms 9736, pipelines/compile_ms 1493, pipelines/driver_ms 1356, pipelines/front_ms 137, render/thread/ms 85.36, render/busy/ms 79.95, main/busy/ms 48.57, stage/main/update 25.50, stage/render/prepare_write_phase_buffers 15.09, stage/main/post_update 14.41, stage/render/render 14.39, render/graph/ms 11.92 |
| warm-up | 9.21 | 86.6 | render/thread/ms 123, render/busy/ms 121, main/busy/ms 65.57, stage/main/post_update 36.52, stage/render/prepare_assets 32.88, stage/render/prepare_resources 28.63, stage/render/prepare_write_phase_buffers 27.67, stage/main/update 13.57, stage/render/render 12.99, encode//sum 11.21, render/graph/ms 10.91, bigspace/total/ms 8.53 |
| warm-up | 9.35 | 49.0 | main/busy/ms 77.49, render/thread/ms 70.94, render/busy/ms 69.25, stage/main/post_update 39.11, stage/main/update 26.07, stage/render/prepare_assets 21.32, stage/render/prepare_flush 18.19, stage/render/cleanup 15.51, stage/render/render 12.07, encode//sum 9.98, render/graph/ms 9.30, stage/render/graph_submit 5.95 |
| warm-up | 9.56 | 73.3 | render/thread/ms 91.24, render/busy/ms 89.60, main/busy/ms 49.35, stage/main/post_update 25.75, stage/render/prepare_resources 19.66, stage/main/update 15.34, stage/render/prepare_flush 14.07, stage/render/prepare_bind_groups 13.85, stage/render/render 13.72, stage/render/prepare_write_phase_buffers 12.52, render/graph/ms 11.45, encode//sum 11.08 |
| warm-up | 9.62 | 56.7 | render/thread/ms 124, render/busy/ms 122, main/busy/ms 73.45, stage/render/prepare_flush 31.63, stage/render/prepare_resources 31.55, stage/main/post_update 21.94, stage/render/render 16.39, stage/main/last 15.87, stage/render/prepare_write_phase_buffers 14.93, encode//sum 14.58, stage/render/prepare_bind_groups 14.25, render/graph/ms 13.94 |
| warm-up | 9.71 | 94.9 | pipelines/wait_ms 3146, pipelines/compile_ms 2631, pipelines/driver_ms 2444, pipelines/front_ms 187, render/thread/ms 148, render/busy/ms 146, main/busy/ms 86.98, stage/main/post_update 32.14, stage/render/render 29.88, stage/render/prepare_resources 25.70, stage/render/prepare_write_phase_buffers 23.52, stage/render/prepare_flush 22.83 |
| warm-up | 9.81 | 56.6 | pipelines/wait_ms 5777, render/thread/ms 86.40, render/busy/ms 84.69, main/busy/ms 56.01, stage/render/prepare_assets 48.66, pipelines/compile_ms 44.39, pipelines/driver_ms 33.93, stage/main/update 30.91, stage/render/render 17.07, stage/main/post_update 16.68, render/graph/ms 14.11, encode//sum 11.79 |
| warm-up | 10.12 | 53.9 | pipelines/wait_ms 9200, pipelines/compile_ms 1158, pipelines/driver_ms 1109, render/thread/ms 81.51, render/busy/ms 79.92, pipelines/front_ms 49.45, main/busy/ms 44.24, stage/main/post_update 19.06, stage/render/specialize 18.93, stage/main/update 17.87, stage/render/queue 15.29, stage/render/render 13.53 |
| warm-up | 10.18 | 68.6 | pipelines/wait_ms 9200, pipelines/compile_ms 1158, pipelines/driver_ms 1109, render/thread/ms 115, render/busy/ms 114, pipelines/front_ms 49.45, main/busy/ms 47.84, stage/render/prepare_resources 22.36, stage/main/post_update 19.89, stage/render/specialize 19.07, stage/main/update 18.07, stage/render/queue 15.99 |
| warm-up | 10.91 | 51.4 | pipelines/compile_ms 5690, pipelines/driver_ms 5278, pipelines/wait_ms 572, pipelines/front_ms 412, render/thread/ms 78.97, render/busy/ms 77.28, main/busy/ms 55.94, stage/main/post_update 39.84, stage/render/prepare_flush 20.62, anim/evaluate/ms 17.72, stage/render/prepare_batch_phases 14.02, stage/render/render 13.39 |
| warm-up | 11.12 | 50.2 | pipelines/wait_ms 5425, pipelines/compile_ms 223, pipelines/driver_ms 151, render/thread/ms 75.46, render/busy/ms 73.50, pipelines/front_ms 71.60, main/busy/ms 53.64, stage/main/post_update 34.64, stage/main/update 13.34, stage/render/render 12.86, stage/render/prepare_assets 12.84, encode//sum 11.83 |
| warm-up | 12.83 | 54.4 | pipelines/wait_ms 875, pipelines/compile_ms 83.49, render/thread/ms 64.87, render/busy/ms 63.23, pipelines/driver_ms 48.10, pipelines/front_ms 35.39, main/busy/ms 29.87, stage/main/post_update 16.46, stage/render/specialize 15.84, encode//sum 15.45, stage/render/queue 14.60, stage/render/render 14.44 |
| warm-up | 13.94 | 57.4 | pipelines/wait_ms 26.7 k, pipelines/compile_ms 88.52, render/thread/ms 74.34, render/busy/ms 73.12, main/busy/ms 68.85, pipelines/front_ms 46.07, pipelines/driver_ms 42.45, stage/main/update 28.99, stage/main/post_update 21.82, stage/render/prepare_batch_phases 16.55, encode//sum 15.28, stage/render/render 13.59 |
| measure | 0.76 | 52.2 | render/thread/ms 67.33, render/busy/ms 65.95, main/busy/ms 30.52, encode//sum 19.29, stage/render/render 17.56, stage/main/post_update 17.20, render/graph/ms 15.37, stage/render/specialize 11.98, stage/render/queue 11.36, stage/render/graph_submit 10.37, stage/render/prepare_assets 8.71, stage/main/update 8.61 |

## Run

- `../target/profiling/walkonce.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\wgpu_walk_once\balmora_walk@2560x1440_arm=onetex-1 --bench-baseline none --off texvariety
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 13%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1364 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **74%** (min 32%) · 2623 MHz (min 2610) · memory 8101 MHz · 77.2 W · 76 °C (max 83) · VRAM 5833 MiB · throttled 100% of samples

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
