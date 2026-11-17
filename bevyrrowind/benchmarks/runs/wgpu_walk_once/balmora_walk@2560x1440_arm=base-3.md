# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5192 frames over 65.6 s · **median 12.15 ms (82 fps)** · mean 12.64 · p95 15.89 · p99 20.50 (1% low 49 fps) · worst 43.47 ms · 80 spikes over 1.5× median · 53.0 stutters/min (work 51.2/min)
- warm-up: 322 frames over 15.7 s, worst 2472.1 ms
- hitches (> 30.4 ms): **29** measured, 139 in warm-up
- timed GPU passes: 7.19 ms of a 12.64 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.15 ms, limit 11 ms
- **FAIL** frame_ms: 58 stutters (> 1.5x moving median of 60) in 1.09 min = 53.01/min, limit 1/min
- **FAIL** worst frame 43.47 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.033 | 2.037 | 2.071 | 1.00 | 0.177 |
| main_opaque_pass_3d | 1.644 | 1.646 | 2.050 | 1.00 | 0.072 |
| early prepass | 0.500 | 0.501 | 0.714 | 1.00 | 0.093 |
| bloom | 0.315 | 0.315 | 0.323 | 1.00 | 0.066 |
| atmosphere_luts | 0.305 | 0.306 | 0.367 | 1.00 | 0.004 |
| water_velocity | 0.263 | 0.264 | 0.347 | 1.00 | 0.002 |
| water_sspr | 0.201 | 0.202 | 0.203 | 1.00 | 0.003 |
| ssao | 0.188 | 0.188 | 0.191 | 1.00 | 0.007 |
| atmos_froxel_inject | 0.145 | 0.146 | 0.147 | 1.00 | 0.002 |
| clouds_trace | 0.141 | 0.142 | 0.232 | 1.00 | 0.001 |
| uniform_allocation | 0.133 | 0.133 | 0.182 | 1.00 | 0.027 |
| water_fft | 0.129 | 0.129 | 0.137 | 1.00 | 0.003 |
| auxiliary_views | 0.123 | 0.123 | 0.493 | 1.00 | 0.133 |
| upscaling | 0.117 | 0.117 | 0.127 | 1.00 | 0.004 |
| bin_unpacking | 0.107 | 0.107 | 0.143 | 1.00 | 0.019 |
| clouds_composite | 0.105 | 0.106 | 0.108 | 1.00 | 0.002 |
| tonemapping | 0.085 | 0.085 | 0.089 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.080 | 0.080 | 0.160 | 1.00 | 0.009 |
| clustering | 0.064 | 0.064 | 0.067 | 1.00 | 0.019 |
| render_sky | 0.051 | 0.051 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.412 | 0.416 | 0.12 | 0.001 |
| clouds_resolve | 0.048 | 0.048 | 0.049 | 1.00 | 0.001 |
| surface_exposure | 0.045 | 0.045 | 0.045 | 1.00 | 0.001 |
| surface_trail | 0.045 | 0.056 | 0.057 | 0.79 | 0.002 |
| surface_exposure_far | 0.039 | 0.044 | 0.046 | 0.88 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| early_mesh_preprocessing | 0.031 | 0.031 | 0.047 | 1.00 | 0.005 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.230 | 0.12 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.028 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| night_sky | 0.014 | 0.014 | 0.033 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.013 | 0.013 | 0.023 | 1.00 | 0.124 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.022 | 1.00 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.012 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.012 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/compile_ms | 15.425 | 0.000 | 8166.27 | 80084.0 |
| pipelines/driver_ms | 13.490 | 0.000 | 7753.89 | 70038.1 |
| render/thread/ms | 11.316 | 14.377 | 41.27 | 58752.7 |
| pipelines/wait_ms | 11.233 | 0.000 | 16821.08 | 58320.8 |
| encode//sum | 10.888 | 15.883 | 41.04 | 56533.0 |
| render/busy/ms | 10.849 | 13.884 | 40.62 | 56329.7 |
| main/busy/ms | 9.718 | 12.405 | 39.02 | 50457.1 |
| stage/render/render | 7.065 | 9.169 | 21.43 | 36681.2 |
| render/graph/ms | 6.402 | 8.423 | 20.61 | 33240.3 |
| stage/main/post_update | 6.119 | 8.412 | 33.45 | 31767.9 |
| stage/render/graph_submit | 5.179 | 6.870 | 18.48 | 26891.2 |
| encode/finish_wall | 2.869 | 4.173 | 5.88 | 14898.2 |
| encode/submit | 2.332 | 3.221 | 16.27 | 12108.2 |
| stage/main/update | 2.252 | 2.745 | 32.00 | 11691.2 |
| pipelines/front_ms | 1.935 | 0.000 | 412.38 | 10046.0 |
| stage/render/camera_driver | 1.164 | 1.643 | 5.05 | 6041.9 |
| stage/render/prepare_write_phase_buffers | 0.790 | 1.147 | 2.87 | 4099.3 |
| stage/render/prepare_assets | 0.609 | 0.788 | 11.72 | 3159.9 |
| bigspace/total/ms | 0.567 | 1.485 | 3.83 | 2943.2 |
| stage/render/prepare_meshes | 0.472 | 0.600 | 9.65 | 2449.8 |
| stage/render/specialize | 0.461 | 0.637 | 13.97 | 2391.9 |
| stage/render/prepare_resources | 0.460 | 0.561 | 1.29 | 2389.4 |
| stage/main/first | 0.425 | 0.556 | 6.02 | 2207.8 |
| stage/main/pre_update | 0.370 | 0.555 | 2.83 | 1918.9 |
| render/present/ms | 0.368 | 0.453 | 1.90 | 1912.4 |
| stage/render/queue | 0.332 | 0.476 | 9.35 | 1723.5 |
| terrain/update/ms | 0.308 | 0.393 | 1.86 | 1598.6 |
| stage/render/prepare_bind_groups | 0.299 | 0.552 | 2.99 | 1552.7 |
| controller/feed/ms | 0.287 | 0.375 | 0.84 | 1492.6 |
| terrain/select/ms | 0.273 | 0.349 | 0.99 | 1416.5 |
| stream/drive/ms | 0.263 | 0.367 | 29.66 | 1366.1 |
| stage/main/fixed_main_loop | 0.247 | 0.389 | 1.24 | 1281.4 |
| bigspace/low_precision/ms | 0.241 | 0.642 | 3.39 | 1252.0 |
| far/select/ms | 0.212 | 0.305 | 0.64 | 1100.4 |
| stage/render/create_views | 0.207 | 0.277 | 1.12 | 1073.9 |
| stage/render/prepare_batch_phases | 0.200 | 0.289 | 1.44 | 1040.0 |
| stream/tick/ms | 0.199 | 0.280 | 0.93 | 1033.6 |
| stage/render/extract_commands | 0.179 | 0.214 | 0.52 | 932.0 |
| stage/render/prepare_flush | 0.170 | 0.313 | 0.85 | 883.3 |
| bigspace/high_precision/ms | 0.168 | 0.580 | 3.28 | 870.5 |
| anim/evaluate/ms | 0.156 | 0.255 | 2.89 | 811.7 |
| anim/evaluate_cpu/ms | 0.148 | 0.190 | 0.63 | 766.0 |
| stage/render/prepare_views | 0.143 | 0.182 | 0.48 | 742.2 |
| controller/move/ms | 0.138 | 0.326 | 0.79 | 716.5 |
| stage/main/last | 0.124 | 0.197 | 1.63 | 643.8 |
| render/acquire/ms | 0.098 | 0.125 | 0.45 | 510.6 |
| stage/render/prepare_collect_phase_buffers | 0.064 | 0.137 | 0.61 | 333.0 |
| physics/step/ms | 0.055 | 0.091 | 0.52 | 288.2 |
| physics/insert/ms | 0.040 | 0.393 | 0.93 | 205.2 |
| encode/resolve_diagnostics/sum | 0.036 | 0.049 | 0.63 | 185.3 |
| stage/render/cleanup | 0.035 | 0.117 | 0.81 | 179.5 |
| stage/render/phase_sort | 0.027 | 0.047 | 0.22 | 141.3 |
| weather/map/ms | 0.026 | 0.035 | 0.18 | 134.9 |
| encode/camera_driver_span_begin/sum | 0.017 | 0.024 | 0.60 | 87.8 |
| encode/camera_driver_span_end/sum | 0.012 | 0.021 | 0.26 | 61.3 |
| anim/apply/ms | 0.012 | 0.021 | 0.40 | 60.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.06 | 57.7 |
| controller/avoid/ms | 0.011 | 0.015 | 0.14 | 57.2 |
| script/update/ms | 0.008 | 0.010 | 0.21 | 39.1 |
| stream/despawn/ms | 0.006 | 0.000 | 1.02 | 30.7 |
| stream/spawn/ms | 0.006 | 0.000 | 1.12 | 29.7 |
| script/ui/ms | 0.005 | 0.007 | 0.15 | 27.9 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 24.7 |
| controller/write/ms | 0.003 | 0.005 | 0.04 | 17.7 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.62 | 14.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 5.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.3 |
| stage/render/prepare | 0.001 | 0.002 | 0.06 | 2.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.7 |
| stream/upload/ms | 0.000 | 0.000 | 0.10 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1258 | 2262 | 583 |
| crowd/c0/meshes/count | count | 343 | 646 | 162 |
| crowd/c1/count | count | 147 | 156 | 148 |
| crowd/c1/joints/count | count | 5535 | 5982 | 5613 |
| crowd/c1/meshes/count | count | 1167 | 1284 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4621 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4601 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.34 | 5.59 | 2.18 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.60 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.26 | 0.01 |
| encode/finish_critical | ms | 2.34 | 5.59 | 2.18 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.63 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 10.61 | 40.62 | 8.87 |
| gpu/timed/ms | ms | 7.91 | 9.12 | 8.28 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 801 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 801 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1286 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3368 | 5112 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1286 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3368 | 5112 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 15 | 206 | 4 |
| phases/shadow/bins | count | 5073 | 8769 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6376 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 206 | 259 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 217 | 476 | 19 |
| physics/pending | count | 0 | 213 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 279 | 329 | 329 |
| pipelines/waiting | count | 1 | 8 | 0 |
| precache/draw_waiting | count | 0 | 7 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 1 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 29 | 68 | 0 |
| preprocess/bind_groups | count | 14 | 569 | 6 |
| preprocess/bind_groups_created | count | 14 | 569 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 218 | 123 |
| process/cpu_usage | % | 30.82 | 70.19 | 25.56 |
| process/mem_usage | GiB | 5.67 | 5.95 | 5.95 |
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
| skin/blocks_changed/count | count | 13 | 127 | 13 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 370.4 k | 807.2 k | 493.6 k |
| skin/joints_written/count | count | 5788 | 12.6 k | 7712 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4917 | 5010 | 5010 |
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
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.21 | 85.63 | 28.94 |
| system/mem_usage | % | 65.53 | 66.24 | 66.19 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 65 | 255 | 54 |
| views/all/bins | count | 7616 | 11.7 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.8 k | 14.0 k |
| views/all/pipelines | count | 253 | 308 | 75 |
| views/all/pipelines_distinct | count | 71 | 75 | 61 |
| views/all/work_items | count | 10.8 k | 35.6 k | 8119 |
| views/cascade/batch_sets | count | 9 | 53 | 4 |
| views/cascade/bins | count | 2790 | 6120 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8920 | 23.5 k | 6376 |
| views/cascade/pipelines | count | 36 | 53 | 29 |
| views/cascade/work_items | count | 410 | 23.5 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 188 | 0 |
| views/lamp_face/bins | count | 2224 | 2746 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2472 | 3040 | 0 |
| views/lamp_face/pipelines | count | 170 | 208 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2857 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2883 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8348 | 13.0 k | 7610 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8520 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 49.03 | 231 | 254.6 k |
| anim/events/count | count | 0.11 | 3 | 565 |
| assets/Image/added | count | 0.11 | 36 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.78 | 152 | 4061 |
| assets/Mesh/removed | count | 0.93 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 464 |
| assets/StandardMaterial/modified | count | 4.17 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 41.31 | 3482 | 214.5 k |
| ecs/entities_delta | count | 0.90 | 1186 | 4674 |
| encode//count | count | 39.30 | 273 | 204.0 k |
| encode/bind_walk/init_textures | count | 81.0 k | 216.5 k | 420.42 M |
| encode/bind_walk/init_textures_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/inits | count | 568 | 1259 | 2.95 M |
| encode/bind_walk/inits_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/merge_textures | count | 17.6 k | 119.4 k | 91.25 M |
| encode/bind_walk/merge_textures_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/merges | count | 183 | 410 | 952.3 k |
| encode/bind_walk/merges_skipped | count | 0 | 0 | 0 |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5192 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5192 |
| encode/queue/command_buffers | count | 44.32 | 278 | 230.1 k |
| encode/queue/submits | count | 2.02 | 4 | 10.5 k |
| encode/queue/write_buffer/bytes | bytes | 1.90 M | 4.88 M | 9.89 G |
| encode/queue/write_buffer/calls | count | 257 | 851 | 1.33 M |
| encode/queue/write_buffer_with/bytes | bytes | 179.2 k | 3.52 M | 930.27 M |
| encode/queue/write_buffer_with/calls | count | 11.33 | 465 | 58.8 k |
| encode/queue/write_texture/bytes | bytes | 12.0 k | 8.39 M | 62.48 M |
| encode/queue/write_texture/calls | count | 0.11 | 42 | 562 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5192 |
| physics/inserts | count | 0.44 | 92 | 2287 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.45 | 145 | 2335 |
| physics/steps | count | 0.76 | 3 | 3940 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 6 |
| pipelines/created | count | 0.02 | 2 | 112 |
| precache/hit | count | 0.00 | 2 | 4 |
| precache/missed | count | 0.00 | 2 | 6 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 6520 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.01 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1429 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.4 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2472.1 | main/busy/ms 2445, stage/main/post_update 2338, stage/main/update 97.73, surface/advance_cpu_ms 67.74, stage/main/pre_update 4.67, stage/main/last 0.99, surface/cell_upload_ms 0.71, stream/drive/ms 0.62, stage/main/first 0.56, stage/main/fixed_main_loop 0.48, weather/map/ms 0.27, stream/tick/ms 0.16 |
| warm-up | 3.30 | 823.5 | main/busy/ms 2617, stage/main/post_update 2361, render/thread/ms 793, render/busy/ms 655, stage/render/prepare_assets 607, stage/main/update 243, stage/render/prepare_views 135, render/acquire/ms 135, surface/advance_cpu_ms 67.74, stage/render/create_views 27.51, stage/render/render 13.95, render/graph/ms 9.97 |
| warm-up | 3.42 | 121.2 | render/thread/ms 892, render/busy/ms 701, stage/render/prepare_assets 633, pipelines/compile_ms 314, pipelines/driver_ms 240, main/busy/ms 200, stage/main/update 150, stage/render/prepare_views 135, render/acquire/ms 135, pipelines/front_ms 73.46, stage/render/render 71.13, render/present/ms 55.43 |
| warm-up | 3.61 | 138.0 | pipelines/compile_ms 1057, pipelines/driver_ms 893, pipelines/wait_ms 400, render/thread/ms 186, render/busy/ms 183, pipelines/front_ms 164, main/busy/ms 84.40, stage/render/prepare_resources 55.11, stage/render/prepare_flush 34.88, stage/main/post_update 31.26, stage/main/update 30.56, stage/render/prepare_bind_groups 29.64 |
| warm-up | 3.69 | 77.7 | pipelines/compile_ms 1445, pipelines/driver_ms 1279, pipelines/wait_ms 399, render/thread/ms 205, render/busy/ms 203, pipelines/front_ms 166, main/busy/ms 141, stage/main/update 74.29, stage/render/prepare_resources 64.58, stage/render/prepare_assets 36.62, stage/main/post_update 36.57, stage/render/prepare_flush 32.44 |
| warm-up | 3.77 | 89.0 | pipelines/wait_ms 1633, pipelines/compile_ms 1352, pipelines/driver_ms 1180, pipelines/front_ms 171, render/thread/ms 150, render/busy/ms 148, main/busy/ms 95.32, stage/render/prepare_assets 82.38, stage/main/update 55.27, stage/render/prepare_resources 39.17, stage/main/post_update 22.07, terrain/update/ms 14.22 |
| warm-up | 4.00 | 55.6 | pipelines/wait_ms 2127, pipelines/compile_ms 455, pipelines/driver_ms 318, pipelines/front_ms 137, main/busy/ms 123, render/thread/ms 100, render/busy/ms 98.58, stage/main/update 65.29, stream/drive/ms 60.52, stage/main/post_update 51.94, stage/render/prepare_resources 31.22, stage/render/prepare_assets 27.08 |
| warm-up | 4.15 | 147.9 | pipelines/wait_ms 2449, pipelines/compile_ms 514, pipelines/driver_ms 372, render/thread/ms 159, render/busy/ms 158, pipelines/front_ms 142, main/busy/ms 120, stage/render/prepare_assets 68.33, stage/main/update 64.23, stream/drive/ms 60.07, stage/main/post_update 47.63, stage/render/prepare_flush 29.88 |
| warm-up | 4.29 | 95.3 | pipelines/wait_ms 3975, pipelines/compile_ms 294, pipelines/driver_ms 197, render/thread/ms 109, render/busy/ms 107, pipelines/front_ms 97.60, stage/render/prepare_assets 56.14, main/busy/ms 49.88, stage/main/update 20.64, stage/main/post_update 17.23, stage/render/prepare_flush 16.62, stage/render/prepare_write_phase_buffers 13.80 |
| warm-up | 4.42 | 132.0 | pipelines/wait_ms 4865, pipelines/compile_ms 1143, pipelines/driver_ms 922, pipelines/front_ms 222, render/thread/ms 203, render/busy/ms 201, main/busy/ms 103, stage/render/prepare_assets 89.60, stage/main/post_update 44.96, stage/main/update 44.22, stage/render/prepare_write_phase_buffers 30.71, stream/drive/ms 28.56 |
| warm-up | 4.53 | 104.2 | pipelines/wait_ms 890, pipelines/compile_ms 849, pipelines/driver_ms 725, render/thread/ms 213, render/busy/ms 212, main/busy/ms 171, pipelines/front_ms 124, stage/main/post_update 72.16, stage/render/prepare_assets 66.56, stage/render/prepare_resources 50.28, stage/render/prepare_batch_phases 49.85, stage/main/update 46.25 |
| warm-up | 4.72 | 191.1 | pipelines/compile_ms 1324, pipelines/driver_ms 1246, pipelines/wait_ms 513, render/thread/ms 244, render/busy/ms 243, main/busy/ms 144, stage/render/prepare_assets 106, pipelines/front_ms 78.22, stage/main/post_update 41.58, stage/render/prepare_flush 38.53, stage/main/update 36.57, stage/main/last 34.49 |
| warm-up | 4.84 | 126.9 | pipelines/wait_ms 1425, pipelines/compile_ms 1414, pipelines/driver_ms 1313, render/thread/ms 255, render/busy/ms 253, stage/render/prepare_assets 138, pipelines/front_ms 102, main/busy/ms 94.31, stage/render/prepare_flush 47.31, stage/main/update 41.52, stage/render/prepare_write_phase_buffers 29.57, stage/main/fixed_main_loop 18.24 |
| warm-up | 4.95 | 106.2 | pipelines/wait_ms 2420, pipelines/compile_ms 1381, pipelines/driver_ms 1250, render/thread/ms 185, render/busy/ms 183, pipelines/front_ms 131, stage/render/prepare_assets 78.46, main/busy/ms 64.44, stage/render/prepare_write_phase_buffers 26.18, stage/render/prepare_resources 20.64, stage/main/post_update 19.86, stage/render/prepare_flush 18.65 |
| warm-up | 5.08 | 135.0 | pipelines/wait_ms 2915, pipelines/compile_ms 1406, pipelines/driver_ms 1262, render/thread/ms 211, render/busy/ms 209, pipelines/front_ms 144, main/busy/ms 81.64, stage/render/prepare_write_phase_buffers 68.71, stage/render/prepare_assets 45.74, stage/render/prepare_resources 45.63, stage/main/post_update 37.87, encode//sum 32.59 |
| warm-up | 5.26 | 171.5 | pipelines/wait_ms 2930, render/thread/ms 274, render/busy/ms 254, pipelines/compile_ms 225, pipelines/driver_ms 167, main/busy/ms 114, stage/render/prepare_write_phase_buffers 85.16, stage/main/post_update 76.89, pipelines/front_ms 57.67, stage/render/prepare_assets 54.80, stage/render/prepare_resources 45.84, encode//sum 42.02 |
| warm-up | 5.33 | 78.2 | pipelines/wait_ms 8606, pipelines/compile_ms 326, pipelines/driver_ms 226, render/thread/ms 220, render/busy/ms 200, main/busy/ms 100, pipelines/front_ms 99.43, encode//sum 74.83, stage/main/post_update 70.62, stage/render/render 48.05, render/graph/ms 45.03, stage/render/prepare_assets 44.84 |
| warm-up | 5.42 | 90.8 | pipelines/wait_ms 7083, pipelines/compile_ms 216, render/thread/ms 152, render/busy/ms 150, pipelines/driver_ms 138, pipelines/front_ms 78.93, main/busy/ms 70.36, encode//sum 61.14, stage/render/prepare_assets 59.09, stage/render/render 40.91, render/graph/ms 38.16, stage/main/post_update 38.05 |
| warm-up | 5.53 | 101.8 | render/thread/ms 172, render/busy/ms 165, main/busy/ms 111, stage/main/update 75.52, stage/render/prepare_assets 70.85, stream/drive/ms 62.96, stage/main/post_update 26.70, stage/render/prepare_bind_groups 24.64, stage/render/prepare_write_phase_buffers 18.99, stage/render/prepare_flush 17.46, stage/render/prepare_resources 17.30, stage/render/render 15.64 |
| warm-up | 5.62 | 94.0 | render/thread/ms 173, render/busy/ms 166, main/busy/ms 133, stage/main/update 102, stage/render/prepare_assets 67.79, stream/drive/ms 62.23, stage/render/prepare_resources 26.83, stage/render/prepare_bind_groups 20.13, stage/render/prepare_write_phase_buffers 19.81, stage/render/render 15.93, stage/render/prepare_flush 15.92, stage/main/post_update 12.15 |
| warm-up | 5.79 | 166.9 | render/thread/ms 226, render/busy/ms 225, main/busy/ms 165, stage/main/update 115, stage/render/prepare_assets 92.80, stream/drive/ms 68.08, stage/render/prepare_resources 37.66, stage/render/prepare_flush 32.12, stage/main/pre_update 28.75, stage/render/prepare_batch_phases 21.95, stage/render/prepare_bind_groups 19.11, stage/main/post_update 12.87 |
| warm-up | 5.89 | 99.9 | render/thread/ms 231, render/busy/ms 230, main/busy/ms 164, stage/main/update 85.12, stream/drive/ms 68.77, stage/render/prepare_assets 66.64, stage/render/prepare_flush 35.34, stage/render/prepare_resources 32.44, stage/main/post_update 30.10, stage/render/prepare_write_phase_buffers 29.03, stage/main/fixed_main_loop 25.60, stage/render/prepare_batch_phases 22.37 |
| warm-up | 6.01 | 126.4 | render/thread/ms 187, render/busy/ms 186, main/busy/ms 152, stage/main/update 78.66, stage/render/prepare_assets 76.37, stream/drive/ms 63.90, stage/main/post_update 42.46, stage/render/prepare_write_phase_buffers 33.10, stage/main/fixed_main_loop 24.86, bigspace/total/ms 19.32, stage/render/prepare_resources 17.98, stage/render/prepare_flush 16.89 |
| warm-up | 6.15 | 137.1 | render/thread/ms 227, render/busy/ms 206, main/busy/ms 178, stage/render/prepare_assets 111, stage/main/update 103, stream/drive/ms 63.80, stage/main/post_update 54.78, stage/render/prepare_flush 23.19, stage/render/prepare_resources 21.78, stage/render/prepare_views 19.36, render/acquire/ms 19.16, stage/render/prepare_write_phase_buffers 16.80 |
| warm-up | 6.26 | 106.4 | render/thread/ms 208, render/busy/ms 188, main/busy/ms 181, stage/render/prepare_assets 82.33, stage/main/update 79.13, stage/main/post_update 68.33, stream/drive/ms 40.62, stage/render/prepare_write_phase_buffers 29.32, stage/render/prepare_resources 29.29, stage/main/fixed_main_loop 25.92, stage/render/prepare_flush 21.82, stage/render/prepare_views 19.33 |
| warm-up | 6.33 | 76.6 | render/thread/ms 146, render/busy/ms 144, main/busy/ms 124, stage/main/update 60.99, stage/render/prepare_assets 46.60, stage/main/post_update 40.71, stream/drive/ms 40.71, stage/render/prepare_resources 31.22, stage/render/prepare_write_phase_buffers 23.36, stage/render/prepare_flush 19.72, stage/main/fixed_main_loop 17.59, stage/render/render 14.29 |
| warm-up | 6.45 | 66.2 | render/thread/ms 110, render/busy/ms 108, main/busy/ms 53.92, stage/render/prepare_assets 29.95, stage/main/update 27.46, stage/render/prepare_bind_groups 19.03, stage/main/post_update 18.28, stage/render/render 16.63, render/graph/ms 13.91, stage/render/prepare_resources 12.01, encode//sum 10.39, stage/render/graph_submit 9.29 |
| warm-up | 6.52 | 64.9 | render/thread/ms 118, render/busy/ms 116, main/busy/ms 67.39, stage/render/prepare_resources 33.61, stage/main/post_update 29.75, stage/render/prepare_assets 25.09, stage/main/update 18.91, stage/render/prepare_bind_groups 17.59, stage/render/render 16.62, render/graph/ms 14.07, stage/main/pre_update 11.72, encode//sum 10.03 |
| warm-up | 6.66 | 54.2 | pipelines/compile_ms 7387, pipelines/driver_ms 6939, pipelines/wait_ms 2015, pipelines/front_ms 447, render/thread/ms 79.43, render/busy/ms 77.87, main/busy/ms 49.83, stage/render/prepare_assets 27.99, stage/main/update 20.50, stage/main/pre_update 12.40, stage/main/post_update 11.75, stage/render/render 10.79 |
| warm-up | 6.88 | 69.6 | pipelines/compile_ms 8588, pipelines/driver_ms 7989, pipelines/wait_ms 4676, pipelines/front_ms 599, render/thread/ms 106, render/busy/ms 105, main/busy/ms 87.15, stage/main/post_update 36.92, stage/render/prepare_write_phase_buffers 31.10, stage/main/update 26.04, stage/render/render 19.78, stage/render/prepare_batch_phases 13.79 |
| warm-up | 6.94 | 59.5 | pipelines/compile_ms 11.4 k, pipelines/driver_ms 10.5 k, pipelines/wait_ms 7894, pipelines/front_ms 828, render/thread/ms 121, render/busy/ms 120, main/busy/ms 99.95, stage/main/post_update 42.22, stage/render/prepare_assets 34.01, stage/main/update 32.97, stage/render/prepare_resources 27.19, stream/drive/ms 24.68 |
| warm-up | 7.00 | 59.1 | pipelines/compile_ms 6410, pipelines/driver_ms 5878, pipelines/wait_ms 3218, pipelines/front_ms 532, render/thread/ms 103, render/busy/ms 101, main/busy/ms 78.43, stage/render/prepare_assets 56.63, stage/main/update 33.84, stage/main/post_update 26.64, stream/drive/ms 23.32, stage/render/prepare_resources 16.82 |
| warm-up | 7.21 | 67.1 | pipelines/compile_ms 3559, pipelines/driver_ms 3347, pipelines/front_ms 212, pipelines/wait_ms 142, render/thread/ms 102, render/busy/ms 100, main/busy/ms 72.30, stage/main/post_update 54.89, stage/render/prepare_assets 31.39, bigspace/total/ms 21.82, stage/render/prepare_flush 15.41, stage/render/prepare_resources 13.95 |
| warm-up | 7.30 | 88.5 | pipelines/wait_ms 3843, pipelines/compile_ms 3625, pipelines/driver_ms 3396, pipelines/front_ms 229, render/thread/ms 150, render/busy/ms 148, main/busy/ms 58.38, stage/render/prepare_flush 48.41, stage/main/post_update 41.23, stage/render/prepare_assets 32.67, bigspace/total/ms 19.51, stage/render/prepare_resources 18.81 |
| warm-up | 7.37 | 73.7 | pipelines/wait_ms 3701, render/thread/ms 155, render/busy/ms 152, pipelines/compile_ms 66.17, stage/render/prepare_flush 49.75, pipelines/driver_ms 48.58, main/busy/ms 45.19, stage/render/prepare_assets 34.07, stage/main/post_update 27.67, stage/render/prepare_write_phase_buffers 19.53, pipelines/front_ms 17.59, stage/render/render 16.89 |
| warm-up | 7.44 | 65.2 | pipelines/wait_ms 4383, pipelines/compile_ms 3135, pipelines/driver_ms 2842, pipelines/front_ms 293, render/thread/ms 131, render/busy/ms 128, main/busy/ms 59.39, stage/render/prepare_assets 33.14, stage/main/update 27.07, stage/render/prepare_write_phase_buffers 25.34, stage/main/post_update 22.95, stage/render/prepare_flush 17.66 |
| warm-up | 7.51 | 76.5 | pipelines/wait_ms 7989, pipelines/compile_ms 3211, pipelines/driver_ms 2889, pipelines/front_ms 322, render/thread/ms 133, render/busy/ms 131, main/busy/ms 78.45, stage/render/prepare_assets 54.76, stage/render/prepare_resources 36.58, stage/main/update 28.70, stage/main/post_update 25.93, stage/main/last 15.16 |
| warm-up | 7.89 | 72.0 | pipelines/compile_ms 3775, pipelines/driver_ms 3577, pipelines/wait_ms 272, pipelines/front_ms 198, render/thread/ms 98.39, render/busy/ms 94.60, main/busy/ms 75.97, stage/main/post_update 30.51, stage/main/update 26.72, stream/drive/ms 19.16, stage/render/prepare_write_phase_buffers 17.95, stage/render/prepare_resources 16.96 |
| warm-up | 8.47 | 89.5 | pipelines/wait_ms 24.4 k, pipelines/compile_ms 126, render/thread/ms 119, render/busy/ms 117, pipelines/driver_ms 86.57, main/busy/ms 81.88, stage/main/update 48.61, pipelines/front_ms 39.83, stream/drive/ms 32.61, stage/main/post_update 25.43, stage/render/render 23.40, stage/render/prepare_batch_phases 21.11 |
| warm-up | 8.68 | 55.0 | pipelines/wait_ms 10.1 k, pipelines/compile_ms 237, pipelines/driver_ms 197, render/thread/ms 79.07, render/busy/ms 77.22, main/busy/ms 42.35, pipelines/front_ms 39.50, stage/main/post_update 21.80, stage/render/render 15.16, stage/main/update 14.54, render/graph/ms 12.64, encode//sum 11.86 |
| warm-up | 8.79 | 57.1 | pipelines/wait_ms 5184, pipelines/compile_ms 137, pipelines/driver_ms 137, render/thread/ms 103, render/busy/ms 101, main/busy/ms 57.13, stage/render/prepare_resources 36.25, stage/main/post_update 34.66, stage/render/render 16.74, render/graph/ms 14.04, stage/main/update 12.89, encode//sum 12.52 |
| warm-up | 8.90 | 60.0 | pipelines/wait_ms 21.4 k, pipelines/compile_ms 105, render/thread/ms 94.98, render/busy/ms 93.45, pipelines/driver_ms 89.76, main/busy/ms 44.24, stage/render/prepare_assets 36.25, stage/render/prepare_resources 20.37, stage/main/post_update 16.19, pipelines/front_ms 14.93, stage/main/update 13.96, stage/render/render 13.79 |
| warm-up | 9.00 | 100.3 | pipelines/wait_ms 5375, render/thread/ms 142, render/busy/ms 140, pipelines/compile_ms 50.75, main/busy/ms 45.85, pipelines/driver_ms 42.00, stage/render/prepare_write_phase_buffers 41.06, stage/render/prepare_assets 38.42, stage/render/prepare_resources 26.67, stage/main/post_update 17.33, stage/main/update 15.18, stage/render/render 14.38 |
| warm-up | 9.15 | 57.2 | render/thread/ms 91.98, render/busy/ms 90.24, main/busy/ms 75.18, stage/main/post_update 39.76, stage/render/prepare_resources 24.23, stage/main/update 23.79, stage/render/render 15.04, render/graph/ms 12.52, encode//sum 12.30, stage/render/prepare_assets 10.75, stage/render/prepare_flush 8.63, stage/render/graph_submit 8.54 |
| warm-up | 9.25 | 57.6 | render/thread/ms 96.43, render/busy/ms 94.81, main/busy/ms 72.27, stage/render/prepare_flush 30.10, stage/main/post_update 28.09, stage/main/update 23.30, stage/render/prepare_assets 21.55, stage/render/render 15.25, render/graph/ms 12.79, stage/main/fixed_main_loop 12.30, stream/drive/ms 11.93, encode//sum 11.90 |
| warm-up | 9.64 | 137.1 | pipelines/compile_ms 4745, pipelines/driver_ms 4375, pipelines/wait_ms 1794, pipelines/front_ms 370, render/thread/ms 180, render/busy/ms 178, main/busy/ms 77.71, stage/render/prepare_resources 58.71, stage/render/prepare_write_phase_buffers 40.50, stage/main/update 32.10, stage/render/prepare_assets 23.34, stage/render/render 22.99 |
| warm-up | 9.72 | 82.9 | pipelines/wait_ms 6488, pipelines/compile_ms 4770, pipelines/driver_ms 4390, pipelines/front_ms 380, render/thread/ms 210, render/busy/ms 209, main/busy/ms 84.75, stage/render/prepare_resources 60.74, stage/render/prepare_write_phase_buffers 41.96, stage/main/update 30.78, stage/render/prepare_assets 30.01, stage/render/prepare_flush 25.74 |
| warm-up | 9.80 | 77.0 | pipelines/wait_ms 4695, render/thread/ms 144, render/busy/ms 142, main/busy/ms 60.33, stage/render/prepare_resources 34.62, stage/render/prepare_assets 29.82, pipelines/compile_ms 25.56, stage/render/prepare_write_phase_buffers 21.96, stage/main/update 20.79, stage/main/post_update 19.84, stage/render/prepare_flush 18.32, pipelines/driver_ms 15.41 |
| warm-up | 9.86 | 61.9 | pipelines/wait_ms 4720, pipelines/compile_ms 144, render/thread/ms 120, render/busy/ms 118, pipelines/driver_ms 101, main/busy/ms 56.20, pipelines/front_ms 43.47, stage/render/prepare_assets 34.22, stage/render/prepare_resources 27.89, stage/main/update 22.64, stage/main/post_update 21.13, stage/render/prepare_write_phase_buffers 16.06 |
| warm-up | 10.02 | 76.8 | render/thread/ms 104, render/busy/ms 102, main/busy/ms 77.22, stage/main/post_update 49.36, stage/render/prepare_assets 28.33, stage/render/prepare_resources 20.87, stage/main/update 19.20, stage/render/prepare_write_phase_buffers 18.66, stage/render/render 15.66, encode//sum 13.49, render/graph/ms 13.33, bigspace/total/ms 12.24 |
| warm-up | 10.08 | 59.4 | render/thread/ms 126, render/busy/ms 124, main/busy/ms 109, stage/main/post_update 72.08, stage/render/prepare_resources 35.62, stage/render/prepare_assets 33.11, stage/main/update 24.53, stage/render/prepare_write_phase_buffers 18.14, stage/render/render 11.12, encode//sum 10.95, render/graph/ms 8.92, stage/render/prepare_flush 8.06 |
| warm-up | 10.18 | 101.7 | render/thread/ms 121, render/busy/ms 120, main/busy/ms 99.41, stage/main/post_update 66.01, stage/render/prepare_assets 46.35, stage/render/prepare_resources 33.21, stage/main/update 20.05, bigspace/total/ms 14.11, bigspace/high_precision/ms 13.47, encode//sum 12.38, stage/render/render 11.49, render/graph/ms 9.05 |
| warm-up | 10.24 | 60.0 | render/thread/ms 124, render/busy/ms 123, main/busy/ms 56.21, stage/render/prepare_assets 39.51, stage/main/post_update 31.17, stage/render/prepare_write_phase_buffers 28.41, stage/render/prepare_resources 22.77, stage/main/update 15.83, encode//sum 13.31, stage/render/render 12.88, bigspace/total/ms 12.00, render/graph/ms 10.55 |
| warm-up | 10.62 | 67.6 | pipelines/wait_ms 25.8 k, pipelines/compile_ms 800, pipelines/front_ms 419, pipelines/driver_ms 382, render/thread/ms 93.20, render/busy/ms 91.23, main/busy/ms 83.52, stage/main/post_update 62.38, stage/render/prepare_resources 21.92, stage/render/prepare_assets 19.06, anim/evaluate/ms 17.22, stage/main/update 14.97 |
| warm-up | 10.69 | 73.0 | pipelines/wait_ms 28.8 k, pipelines/compile_ms 875, pipelines/driver_ms 461, pipelines/front_ms 414, render/thread/ms 133, render/busy/ms 132, main/busy/ms 79.72, stage/main/post_update 60.90, stage/render/prepare_assets 39.64, stage/render/prepare_resources 31.86, anim/evaluate/ms 17.33, stage/render/render 16.06 |
| warm-up | 10.76 | 68.8 | pipelines/wait_ms 22.9 k, pipelines/compile_ms 377, pipelines/driver_ms 250, render/thread/ms 138, render/busy/ms 136, pipelines/front_ms 127, main/busy/ms 53.61, stage/render/prepare_assets 31.31, stage/render/prepare_flush 27.61, stage/render/prepare_resources 26.26, stage/main/post_update 23.86, stage/main/update 22.88 |
| warm-up | 10.83 | 65.7 | pipelines/wait_ms 14.3 k, pipelines/compile_ms 226, pipelines/driver_ms 151, render/thread/ms 130, render/busy/ms 128, main/busy/ms 79.28, pipelines/front_ms 74.34, stage/main/post_update 37.07, stage/main/update 33.66, stage/render/prepare_write_phase_buffers 28.09, stage/render/prepare_assets 26.30, stage/render/prepare_resources 20.77 |
| warm-up | 11.00 | 142.1 | pipelines/wait_ms 27.1 k, pipelines/compile_ms 7825, pipelines/driver_ms 7377, pipelines/front_ms 448, render/thread/ms 147, render/busy/ms 145, main/busy/ms 67.47, stage/main/update 38.26, stage/render/prepare_assets 21.19, stage/main/post_update 20.43, stage/render/specialize 19.39, encode//sum 17.79 |
| warm-up | 11.07 | 66.3 | pipelines/wait_ms 33.0 k, pipelines/compile_ms 7918, pipelines/driver_ms 7444, pipelines/front_ms 474, render/thread/ms 184, render/busy/ms 183, main/busy/ms 84.54, stage/main/post_update 36.78, stage/render/prepare_bind_groups 30.23, stage/render/prepare_resources 29.35, stage/render/prepare_assets 24.46, stage/main/update 24.14 |
| warm-up | 11.15 | 79.6 | pipelines/wait_ms 19.7 k, pipelines/compile_ms 498, pipelines/front_ms 260, pipelines/driver_ms 238, render/thread/ms 137, render/busy/ms 135, main/busy/ms 87.27, stage/render/prepare_assets 41.85, stage/main/post_update 37.85, stage/render/prepare_resources 22.30, stage/main/update 21.00, stage/render/prepare_collect_phase_buffers 18.38 |
| warm-up | 11.25 | 61.1 | pipelines/wait_ms 8319, pipelines/compile_ms 5293, pipelines/driver_ms 4904, pipelines/front_ms 389, render/thread/ms 96.28, render/busy/ms 93.92, main/busy/ms 63.64, stage/render/prepare_write_phase_buffers 37.21, stage/main/post_update 29.65, stage/main/update 24.77, encode//sum 22.03, stage/render/render 21.09 |
| warm-up | 11.31 | 61.8 | pipelines/wait_ms 15.1 k, pipelines/compile_ms 999, pipelines/driver_ms 835, pipelines/front_ms 164, render/thread/ms 112, render/busy/ms 110, main/busy/ms 91.37, stage/render/prepare_write_phase_buffers 39.58, stage/main/post_update 31.63, stage/main/pre_update 27.39, stage/main/update 25.43, encode//sum 24.67 |
| warm-up | 11.38 | 66.6 | pipelines/wait_ms 20.0 k, pipelines/compile_ms 198, pipelines/driver_ms 139, render/thread/ms 115, render/busy/ms 114, main/busy/ms 79.25, pipelines/front_ms 58.68, stage/render/prepare_assets 35.55, stage/main/pre_update 29.26, stage/render/prepare_resources 27.08, encode//sum 23.28, stage/main/update 21.13 |
| warm-up | 14.00 | 63.2 | render/thread/ms 80.56, render/busy/ms 79.03, main/busy/ms 33.47, encode//sum 26.46, stage/render/render 22.95, stage/main/post_update 21.04, render/graph/ms 20.64, stage/render/graph_submit 16.44, stage/render/queue 16.31, stage/render/specialize 16.19, encode/finish_wall 9.04, stage/main/update 8.71 |

## Run

- `../target/profiling/walkonce.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\wgpu_walk_once\balmora_walk@2560x1440_arm=base-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1386 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **66%** (min 36%) · 2623 MHz (min 2460) · memory 8101 MHz · 72.2 W · 74 °C (max 80) · VRAM 6663 MiB · throttled 99% of samples

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
