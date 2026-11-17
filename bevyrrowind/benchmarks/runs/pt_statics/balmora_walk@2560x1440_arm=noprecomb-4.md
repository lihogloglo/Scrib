# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: precombine · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5462 frames over 65.6 s · **median 11.62 ms (86 fps)** · mean 12.02 · p95 15.05 · p99 19.02 (1% low 53 fps) · worst 70.83 ms · 84 spikes over 1.5× median · 57.6 stutters/min (work 47.5/min)
- warm-up: 326 frames over 8.9 s, worst 2535.7 ms
- hitches (> 29.1 ms): **30** measured, 32 in warm-up
- timed GPU passes: 6.98 ms of a 12.02 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.62 ms, limit 11 ms
- **FAIL** frame_ms: 63 stutters (> 1.5x moving median of 60) in 1.09 min = 57.58/min, limit 1/min
- **FAIL** worst frame 70.83 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.985 | 2.033 | 2.064 | 0.98 | 0.174 |
| main_opaque_pass_3d | 1.568 | 1.607 | 2.016 | 0.98 | 0.066 |
| early prepass | 0.456 | 0.467 | 0.680 | 0.98 | 0.087 |
| bloom | 0.307 | 0.314 | 0.321 | 0.98 | 0.064 |
| atmosphere_luts | 0.297 | 0.304 | 0.365 | 0.98 | 0.003 |
| water_velocity | 0.269 | 0.276 | 0.347 | 0.98 | 0.002 |
| water_sspr | 0.197 | 0.201 | 0.203 | 0.98 | 0.002 |
| ssao | 0.184 | 0.188 | 0.193 | 0.98 | 0.007 |
| atmos_froxel_inject | 0.142 | 0.146 | 0.147 | 0.98 | 0.001 |
| clouds_trace | 0.138 | 0.141 | 0.231 | 0.98 | 0.001 |
| auxiliary_views | 0.129 | 0.132 | 0.546 | 0.98 | 0.141 |
| uniform_allocation | 0.127 | 0.131 | 0.178 | 0.98 | 0.027 |
| water_fft | 0.127 | 0.131 | 0.138 | 0.98 | 0.003 |
| upscaling | 0.114 | 0.117 | 0.124 | 0.98 | 0.004 |
| bin_unpacking | 0.104 | 0.107 | 0.142 | 0.98 | 0.019 |
| clouds_composite | 0.103 | 0.106 | 0.108 | 0.98 | 0.002 |
| main_transmissive_pass_3d | 0.083 | 0.085 | 0.178 | 0.98 | 0.009 |
| tonemapping | 0.083 | 0.085 | 0.088 | 0.98 | 0.003 |
| clustering | 0.062 | 0.064 | 0.067 | 0.98 | 0.018 |
| render_sky | 0.050 | 0.052 | 0.052 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.050 | 0.411 | 0.415 | 0.12 | 0.001 |
| clouds_resolve | 0.047 | 0.048 | 0.049 | 0.98 | 0.001 |
| surface_exposure | 0.042 | 0.043 | 0.044 | 0.98 | 0.001 |
| surface_trail | 0.042 | 0.056 | 0.057 | 0.74 | 0.002 |
| early_mesh_preprocessing | 0.039 | 0.039 | 0.062 | 0.98 | 0.005 |
| surface_exposure_far | 0.035 | 0.042 | 0.044 | 0.85 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.98 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.230 | 0.12 | 0.004 |
| water_snapshot | 0.024 | 0.024 | 0.028 | 0.98 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.98 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.032 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.013 | 0.023 | 0.98 | 0.122 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.98 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.98 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.97 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| main/busy/ms | 10.452 | 12.804 | 31.69 | 57089.2 |
| render/thread/ms | 9.802 | 12.190 | 61.54 | 53538.6 |
| render/busy/ms | 9.266 | 11.668 | 60.94 | 50608.2 |
| stage/main/post_update | 6.937 | 8.895 | 13.78 | 37890.8 |
| stage/render/render | 5.598 | 7.164 | 14.92 | 30573.9 |
| render/graph/ms | 4.875 | 6.427 | 13.58 | 26627.2 |
| stage/render/graph_submit | 3.687 | 4.903 | 10.77 | 20136.9 |
| encode//sum | 3.610 | 4.702 | 16.90 | 19719.6 |
| encode/submit | 2.350 | 3.259 | 6.88 | 12835.5 |
| stage/main/update | 2.204 | 2.699 | 22.70 | 12037.7 |
| encode/finish_wall | 1.372 | 2.043 | 4.94 | 7495.4 |
| stage/render/camera_driver | 1.130 | 1.549 | 4.37 | 6174.3 |
| encode/pass:shadow_point_light/sum | 0.881 | 3.427 | 16.23 | 4811.4 |
| encode/pass:shadow_directional_light/sum | 0.880 | 1.598 | 2.57 | 4804.0 |
| encode/pass:main_opaque_pass_3d/sum | 0.802 | 1.114 | 2.01 | 4382.7 |
| stage/render/prepare_write_phase_buffers | 0.788 | 1.167 | 2.68 | 4301.8 |
| encode/pass:early prepass/sum | 0.775 | 1.087 | 2.08 | 4231.1 |
| bigspace/total/ms | 0.594 | 1.378 | 2.20 | 3243.3 |
| stage/render/prepare_assets | 0.587 | 0.743 | 20.63 | 3203.5 |
| stage/render/prepare_meshes | 0.531 | 0.660 | 8.59 | 2898.1 |
| stage/render/prepare_resources | 0.452 | 0.572 | 2.49 | 2471.5 |
| render/present/ms | 0.440 | 0.584 | 2.13 | 2405.5 |
| stage/main/first | 0.404 | 0.526 | 4.96 | 2204.8 |
| stage/render/specialize | 0.385 | 0.438 | 20.19 | 2102.0 |
| stage/main/pre_update | 0.370 | 0.564 | 5.60 | 2019.9 |
| stage/render/queue | 0.348 | 0.496 | 13.28 | 1902.5 |
| terrain/update/ms | 0.304 | 0.390 | 1.64 | 1662.8 |
| stage/render/prepare_bind_groups | 0.289 | 0.511 | 2.47 | 1576.3 |
| terrain/select/ms | 0.270 | 0.348 | 1.03 | 1476.9 |
| controller/feed/ms | 0.269 | 0.340 | 0.87 | 1467.8 |
| stream/drive/ms | 0.252 | 0.336 | 20.75 | 1378.7 |
| stage/main/fixed_main_loop | 0.234 | 0.381 | 1.48 | 1280.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.232 | 0.343 | 1.00 | 1265.4 |
| bigspace/low_precision/ms | 0.229 | 0.703 | 1.82 | 1249.8 |
| stage/render/create_views | 0.204 | 0.262 | 1.11 | 1115.3 |
| far/select/ms | 0.198 | 0.270 | 0.66 | 1083.5 |
| stage/render/prepare_batch_phases | 0.193 | 0.281 | 0.75 | 1055.8 |
| stream/tick/ms | 0.190 | 0.258 | 0.72 | 1038.4 |
| stage/render/extract_commands | 0.174 | 0.205 | 0.43 | 949.9 |
| stage/render/prepare_flush | 0.173 | 0.382 | 2.93 | 942.5 |
| anim/evaluate/ms | 0.166 | 0.277 | 1.59 | 907.4 |
| bigspace/high_precision/ms | 0.161 | 0.652 | 1.85 | 877.7 |
| anim/evaluate_cpu/ms | 0.157 | 0.197 | 0.78 | 859.4 |
| stage/render/prepare_views | 0.141 | 0.175 | 0.55 | 767.9 |
| controller/move/ms | 0.128 | 0.245 | 0.54 | 699.9 |
| stage/main/last | 0.124 | 0.166 | 0.56 | 678.8 |
| render/acquire/ms | 0.096 | 0.122 | 0.38 | 524.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.094 | 0.149 | 0.81 | 514.3 |
| stage/render/prepare_collect_phase_buffers | 0.064 | 0.133 | 0.47 | 348.9 |
| stage/render/cleanup | 0.057 | 0.144 | 0.63 | 312.8 |
| pipelines/compile_ms | 0.057 | 0.000 | 132.76 | 312.2 |
| pipelines/front_ms | 0.055 | 0.000 | 128.91 | 299.5 |
| physics/step/ms | 0.049 | 0.084 | 0.57 | 268.1 |
| physics/insert/ms | 0.037 | 0.392 | 1.01 | 204.2 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.82 | 170.9 |
| stage/render/phase_sort | 0.029 | 0.050 | 0.57 | 158.3 |
| weather/map/ms | 0.024 | 0.033 | 0.13 | 133.5 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.55 | 80.9 |
| anim/apply/ms | 0.012 | 0.022 | 0.20 | 65.9 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.51 | 61.5 |
| controller/avoid/ms | 0.011 | 0.015 | 0.24 | 59.1 |
| weather/eval/ms | 0.010 | 0.014 | 0.11 | 56.0 |
| script/update/ms | 0.007 | 0.009 | 0.19 | 39.1 |
| stream/despawn/ms | 0.006 | 0.000 | 1.03 | 30.2 |
| script/input/ms | 0.005 | 0.007 | 0.14 | 29.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.14 | 29.2 |
| script/ui/ms | 0.005 | 0.006 | 0.03 | 24.6 |
| controller/write/ms | 0.004 | 0.005 | 0.46 | 19.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.71 | 14.2 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.66 | 12.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.5 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.12 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 0 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1291 | 2262 | 641 |
| crowd/c0/meshes/count | count | 355 | 646 | 174 |
| crowd/c1/count | count | 146 | 155 | 147 |
| crowd/c1/joints/count | count | 5531 | 5947 | 5555 |
| crowd/c1/meshes/count | count | 1167 | 1270 | 1167 |
| crowd/c2/count | count | 394 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4619 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4597 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 83.3 k | 91.8 k | 84.3 k |
| encode//max | ms | 0.39 | 1.79 | 0.51 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.55 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.51 | 0.02 |
| encode/finish_critical | ms | 0.99 | 2.57 | 0.90 |
| encode/pass:early prepass/max | ms | 0.79 | 2.08 | 0.85 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 2.01 | 0.67 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.81 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 1.00 | 0.18 |
| encode/pass:shadow_directional_light/max | ms | 0.82 | 2.57 | 0.90 |
| encode/pass:shadow_point_light/max | ms | 0.82 | 1.45 | 0.83 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.82 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 10.53 | 60.94 | 8.88 |
| gpu/timed/ms | ms | 7.85 | 9.87 | 8.18 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 11.6 k | 12.8 k | 11.5 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 800 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 800 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1368 | 1534 | 1424 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 5281 | 7381 | 5167 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1368 | 1534 | 1424 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 5281 | 7381 | 5167 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 17 | 205 | 4 |
| phases/shadow/bins | count | 4446 | 8124 | 2604 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 16.9 k | 44.8 k | 12.0 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 202 | 245 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 324 | 325 | 325 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 15 | 567 | 6 |
| preprocess/bind_groups_created | count | 15 | 567 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 216 | 123 |
| process/cpu_usage | % | 27.29 | 42.58 | 29.33 |
| process/mem_usage | GiB | 5.47 | 5.66 | 5.64 |
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
| skin/blocks_changed/count | count | 13 | 126 | 12 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 366.3 k | 813.8 k | 489.7 k |
| skin/joints_written/count | count | 5724 | 12.7 k | 7652 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 5219 | 5307 | 5307 |
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
| stream/latency/s | s | 0.04 | 0.14 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 742 | 851 | 683 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.29 | 55.05 | 33.59 |
| system/mem_usage | % | 62.25 | 63.16 | 60.32 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 67 | 254 | 54 |
| views/all/bins | count | 7262 | 11.0 k | 6246 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 29.1 k | 59.3 k | 23.6 k |
| views/all/pipelines | count | 249 | 292 | 74 |
| views/all/pipelines_distinct | count | 70 | 74 | 61 |
| views/all/work_items | count | 14.9 k | 57.2 k | 12.1 k |
| views/cascade/batch_sets | count | 9 | 47 | 4 |
| views/cascade/bins | count | 2935 | 6036 | 2604 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 15.1 k | 42.0 k | 12.0 k |
| views/cascade/pipelines | count | 36 | 53 | 28 |
| views/cascade/work_items | count | 698 | 39.5 k | 182 |
| views/lamp_face/batch_sets | count | 0 | 188 | 0 |
| views/lamp_face/bins | count | 1419 | 2199 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1849 | 2879 | 0 |
| views/lamp_face/pipelines | count | 166 | 199 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2139 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 3169 | 3712 | 3642 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 12.0 k | 17.5 k | 11.6 k |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 12.2 k | 17.7 k | 11.9 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 55.0 k | 61.4 k | 55.9 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 67.73 | 291 | 369.9 k |
| anim/events/count | count | 0.10 | 3 | 546 |
| assets/Image/added | count | 0.11 | 43 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.52 | 189 | 2822 |
| assets/Mesh/removed | count | 0.87 | 1146 | 4740 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.95 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.71 | 1244 | 9344 |
| encode//count | count | 33.52 | 220 | 183.1 k |
| encode/bind_walk/init_textures | count | 12.6 k | 114.7 k | 68.86 M |
| encode/bind_walk/init_textures_skipped | count | 67.8 k | 149.1 k | 370.10 M |
| encode/bind_walk/inits | count | 462 | 1124 | 2.52 M |
| encode/bind_walk/inits_skipped | count | 112 | 165 | 609.9 k |
| encode/bind_walk/merge_textures | count | 12.5 k | 114.6 k | 68.46 M |
| encode/bind_walk/merge_textures_skipped | count | 5374 | 6012 | 29.35 M |
| encode/bind_walk/merges | count | 98.85 | 333 | 539.9 k |
| encode/bind_walk/merges_skipped | count | 86.46 | 132 | 472.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5462 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5462 |
| encode/pass:early prepass/count | count | 1 | 1 | 5462 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5462 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5462 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5462 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5462 |
| encode/pass:shadow_point_light/count | count | 1.38 | 48 | 7536 |
| encode/queue/command_buffers | count | 44.92 | 278 | 245.3 k |
| encode/queue/submits | count | 2.01 | 4 | 11.0 k |
| encode/queue/write_buffer/bytes | bytes | 1.95 M | 6.63 M | 10.62 G |
| encode/queue/write_buffer/calls | count | 261 | 848 | 1.42 M |
| encode/queue/write_buffer_with/bytes | bytes | 172.6 k | 1.30 M | 942.63 M |
| encode/queue/write_buffer_with/calls | count | 10.53 | 576 | 57.5 k |
| encode/queue/write_texture/bytes | bytes | 13.0 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5462 |
| physics/inserts | count | 0.43 | 105 | 2327 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.43 | 161 | 2335 |
| physics/steps | count | 0.72 | 3 | 3938 |
| physics/steps_dropped | count | 0.00 | 1 | 2 |
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
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 16 | 351 |
| stream/upload/bytes | bytes | 6198 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.01 | 1 | 28 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1359 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.1 ms; slowest 62, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2535.7 | main/busy/ms 2504, stage/main/post_update 2399, stage/main/update 96.30, surface/advance_cpu_ms 67.46, stage/main/pre_update 4.80, stage/main/last 1.04, surface/cell_upload_ms 0.79, stage/main/first 0.67, stream/drive/ms 0.47, stage/main/fixed_main_loop 0.34, weather/map/ms 0.15, stream/tick/ms 0.11 |
| warm-up | 3.28 | 743.4 | main/busy/ms 2644, stage/main/post_update 2422, render/thread/ms 707, render/busy/ms 542, stage/render/prepare_assets 486, stage/main/update 207, stage/render/prepare_views 162, render/acquire/ms 162, surface/advance_cpu_ms 67.46, stage/render/create_views 28.17, stage/render/render 11.69, stage/render/prepare_flush 8.06 |
| warm-up | 3.39 | 114.5 | render/thread/ms 804, render/busy/ms 584, stage/render/prepare_assets 514, stage/render/prepare_views 163, render/acquire/ms 162, main/busy/ms 161, stage/main/update 115, stage/render/render 71.66, pipelines/compile_ms 61.08, render/present/ms 57.18, pipelines/driver_ms 39.91, stage/main/post_update 34.56 |
| warm-up | 3.49 | 78.4 | pipelines/wait_ms 704, pipelines/compile_ms 608, pipelines/front_ms 546, main/busy/ms 88.83, pipelines/driver_ms 62.17, stage/main/post_update 57.73, render/thread/ms 57.23, render/busy/ms 50.03, stage/render/prepare_assets 26.48, terrain/update/ms 15.52, stage/render/render 14.52, stage/main/update 13.09 |
| warm-up | 3.60 | 109.6 | pipelines/wait_ms 5997, pipelines/compile_ms 1440, pipelines/front_ms 1367, main/busy/ms 152, render/thread/ms 136, render/busy/ms 134, stage/main/update 80.27, pipelines/driver_ms 72.93, stage/render/prepare_resources 40.67, stage/main/post_update 37.75, stage/render/prepare_assets 36.92, stage/render/prepare_write_phase_buffers 33.81 |
| warm-up | 3.68 | 78.4 | pipelines/wait_ms 5579, pipelines/compile_ms 2109, pipelines/front_ms 2001, render/thread/ms 153, render/busy/ms 150, main/busy/ms 136, pipelines/driver_ms 108, stage/main/update 87.61, stage/render/prepare_assets 58.20, stage/render/prepare_resources 40.03, stage/render/prepare_write_phase_buffers 33.89, stage/main/fixed_main_loop 23.62 |
| warm-up | 3.71 | 30.7 | pipelines/compile_ms 1492, pipelines/front_ms 1423, pipelines/wait_ms 415, render/thread/ms 85.38, render/busy/ms 83.47, pipelines/driver_ms 68.45, stage/render/prepare_assets 65.00, main/busy/ms 39.78, stage/main/post_update 17.90, stage/main/update 14.43, stage/render/render 12.59, terrain/update/ms 11.30 |
| warm-up | 3.87 | 46.0 | pipelines/compile_ms 424, pipelines/front_ms 411, main/busy/ms 52.66, stage/main/update 44.05, stream/drive/ms 40.61, render/thread/ms 22.84, render/busy/ms 21.36, pipelines/driver_ms 12.91, stage/render/prepare_assets 9.58, stage/render/render 8.58, render/graph/ms 6.86, encode//sum 5.40 |
| warm-up | 4.10 | 44.2 | pipelines/compile_ms 603, pipelines/front_ms 586, main/busy/ms 56.40, render/thread/ms 45.78, render/busy/ms 43.91, stage/main/update 35.15, stage/render/render 19.07, pipelines/driver_ms 17.63, stage/main/post_update 15.59, render/graph/ms 14.36, stage/render/graph_submit 11.00, stage/render/prepare_assets 9.19 |
| warm-up | 4.15 | 43.8 | pipelines/compile_ms 1803, pipelines/front_ms 1744, main/busy/ms 63.02, pipelines/driver_ms 58.87, render/thread/ms 56.52, render/busy/ms 54.62, pipelines/wait_ms 52.00, stage/main/update 39.84, stage/main/post_update 18.25, stage/render/prepare_assets 16.22, stage/render/render 15.00, render/graph/ms 12.63 |
| warm-up | 4.19 | 49.0 | pipelines/compile_ms 1315, pipelines/front_ms 1269, render/thread/ms 66.23, render/busy/ms 64.22, pipelines/wait_ms 53.87, pipelines/driver_ms 45.81, main/busy/ms 39.01, stage/render/render 25.09, stage/main/update 22.85, render/graph/ms 22.39, stage/render/prepare_assets 20.18, stage/render/graph_submit 16.84 |
| warm-up | 4.24 | 44.2 | pipelines/compile_ms 195, pipelines/front_ms 186, render/thread/ms 73.47, render/busy/ms 71.43, stage/render/render 32.36, render/graph/ms 29.59, encode//sum 27.71, main/busy/ms 23.32, stage/render/graph_submit 19.76, stage/render/prepare_assets 19.75, pipelines/wait_ms 16.21, encode/pass:shadow_point_light/sum 15.54 |
| warm-up | 4.33 | 87.7 | pipelines/compile_ms 1205, pipelines/front_ms 1161, pipelines/wait_ms 160, render/thread/ms 110, render/busy/ms 108, encode//sum 88.17, pipelines/driver_ms 44.68, stage/render/render 42.43, render/graph/ms 39.48, main/busy/ms 22.16, stage/render/graph_submit 21.94, stage/render/camera_driver 17.37 |
| warm-up | 4.36 | 31.1 | pipelines/compile_ms 1126, pipelines/front_ms 1085, pipelines/wait_ms 147, render/thread/ms 101, render/busy/ms 98.93, encode//sum 73.39, pipelines/driver_ms 40.28, stage/render/render 33.98, render/graph/ms 31.28, main/busy/ms 22.84, stage/render/graph_submit 19.40, stage/render/prepare_resources 15.74 |
| warm-up | 4.68 | 34.9 | pipelines/compile_ms 192, pipelines/front_ms 185, render/thread/ms 54.24, render/busy/ms 52.97, main/busy/ms 36.69, stage/render/prepare_assets 18.77, stage/main/update 17.56, stream/drive/ms 12.63, stage/main/post_update 12.58, stage/render/prepare_resources 12.32, stage/render/render 12.29, render/graph/ms 10.44 |
| warm-up | 4.71 | 34.4 | pipelines/compile_ms 590, pipelines/front_ms 566, render/thread/ms 62.65, render/busy/ms 61.25, main/busy/ms 38.58, pipelines/driver_ms 24.04, stage/main/post_update 20.38, stage/render/prepare_assets 19.53, stage/render/phase_sort 17.35, stage/render/render 12.85, render/graph/ms 10.86, stage/main/update 10.69 |
| warm-up | 4.75 | 42.7 | pipelines/compile_ms 398, pipelines/front_ms 381, render/thread/ms 71.34, render/busy/ms 70.00, main/busy/ms 54.90, stage/main/post_update 24.05, stage/main/update 22.77, stage/render/phase_sort 17.38, pipelines/driver_ms 17.38, stage/render/prepare_resources 16.39, stage/render/render 14.27, render/graph/ms 12.39 |
| warm-up | 4.80 | 48.0 | pipelines/compile_ms 1918, pipelines/front_ms 1870, render/thread/ms 70.02, render/busy/ms 68.80, main/busy/ms 59.93, pipelines/driver_ms 48.10, stage/main/update 36.48, stage/render/prepare_assets 25.84, stage/render/prepare_resources 16.08, stage/main/post_update 14.44, stage/render/render 13.20, render/graph/ms 11.38 |
| warm-up | 4.84 | 40.2 | pipelines/compile_ms 2131, pipelines/front_ms 2080, render/thread/ms 64.58, render/busy/ms 63.32, pipelines/driver_ms 50.90, main/busy/ms 39.10, stage/render/prepare_assets 32.70, stage/main/update 23.30, stage/render/render 11.88, pipelines/wait_ms 11.54, render/graph/ms 9.96, encode//sum 9.27 |
| warm-up | 4.98 | 29.7 | pipelines/compile_ms 199, pipelines/front_ms 189, main/busy/ms 52.78, render/thread/ms 39.26, render/busy/ms 37.44, stage/main/post_update 27.46, stage/main/update 19.20, stage/render/render 15.19, render/graph/ms 12.51, pipelines/driver_ms 10.93, stream/drive/ms 10.56, encode//sum 10.50 |
| warm-up | 5.03 | 47.5 | pipelines/compile_ms 364, pipelines/front_ms 343, main/busy/ms 55.15, render/thread/ms 47.96, render/busy/ms 46.25, stage/main/post_update 27.43, pipelines/driver_ms 21.44, stage/main/update 20.28, stage/render/prepare_assets 15.65, stage/render/render 15.06, render/graph/ms 12.21, encode//sum 11.23 |
| warm-up | 5.08 | 49.7 | pipelines/compile_ms 601, pipelines/front_ms 576, render/thread/ms 55.13, render/busy/ms 53.81, main/busy/ms 32.90, stage/render/prepare_assets 32.88, pipelines/driver_ms 25.46, stage/main/post_update 13.73, stage/main/update 11.80, stage/render/render 10.41, render/graph/ms 8.32, encode//sum 7.85 |
| warm-up | 5.18 | 59.4 | pipelines/compile_ms 474, pipelines/front_ms 460, render/thread/ms 76.82, render/busy/ms 75.10, main/busy/ms 40.87, stage/render/prepare_assets 20.97, stage/main/post_update 17.32, stage/main/update 17.02, pipelines/driver_ms 14.00, stage/render/render 13.40, stage/render/prepare_resources 12.55, render/graph/ms 11.25 |
| warm-up | 5.25 | 43.9 | pipelines/compile_ms 623, pipelines/front_ms 603, render/thread/ms 43.89, render/busy/ms 42.59, main/busy/ms 27.93, stage/render/prepare_assets 22.57, pipelines/driver_ms 19.85, stage/main/post_update 11.24, stage/render/render 10.57, stage/main/update 10.02, render/graph/ms 8.71, encode//sum 8.08 |
| warm-up | 5.44 | 29.7 | pipelines/compile_ms 184, pipelines/front_ms 180, render/thread/ms 33.55, render/busy/ms 31.63, main/busy/ms 31.34, stage/main/update 14.41, stage/render/render 12.44, stage/main/post_update 12.09, render/graph/ms 10.02, encode//sum 8.92, stage/render/prepare_assets 8.86, stage/render/graph_submit 7.04 |
| warm-up | 5.52 | 30.1 | pipelines/compile_ms 527, pipelines/front_ms 516, main/busy/ms 39.49, stage/main/update 29.50, stream/drive/ms 26.10, render/thread/ms 17.46, render/busy/ms 16.39, pipelines/driver_ms 11.28, stage/render/render 6.65, stage/main/post_update 6.30, encode//sum 6.10, render/graph/ms 5.15 |
| warm-up | 5.59 | 43.8 | pipelines/compile_ms 286, pipelines/front_ms 279, main/busy/ms 45.15, stage/main/update 34.88, stream/drive/ms 30.88, render/thread/ms 22.94, render/busy/ms 21.97, pipelines/driver_ms 7.91, stage/render/render 7.82, encode//sum 7.66, stage/main/post_update 7.07, stage/render/prepare_assets 7.02 |
| warm-up | 5.66 | 29.1 | pipelines/compile_ms 135, pipelines/front_ms 132, main/busy/ms 36.62, render/thread/ms 25.85, stage/main/update 25.36, render/busy/ms 24.59, stream/drive/ms 21.63, stage/render/render 10.64, stage/render/prepare_assets 10.08, render/graph/ms 8.63, stage/main/post_update 7.72, encode//sum 7.72 |
| warm-up | 5.72 | 37.3 | pipelines/compile_ms 134, pipelines/front_ms 131, main/busy/ms 37.38, render/thread/ms 31.96, render/busy/ms 30.75, stage/main/update 27.13, stream/drive/ms 19.64, stage/render/render 12.55, render/graph/ms 10.63, encode//sum 9.72, stage/render/graph_submit 8.17, stage/render/prepare_assets 7.32 |
| warm-up | 5.77 | 35.4 | pipelines/compile_ms 75.11, pipelines/front_ms 71.16, main/busy/ms 41.06, stage/main/update 28.60, render/thread/ms 23.71, render/busy/ms 22.44, stream/drive/ms 19.24, stage/render/render 10.49, render/graph/ms 8.74, stage/main/post_update 8.08, encode//sum 7.19, stage/render/graph_submit 6.34 |
| warm-up | 5.98 | 35.1 | pipelines/compile_ms 1054, pipelines/front_ms 1021, render/thread/ms 47.25, main/busy/ms 46.96, render/busy/ms 45.55, pipelines/driver_ms 32.56, stage/main/update 21.90, pipelines/wait_ms 21.37, stage/main/post_update 18.14, stage/render/render 13.20, render/graph/ms 10.78, stage/render/prepare_assets 10.35 |
| warm-up | 7.84 | 44.3 | render/thread/ms 43.80, render/busy/ms 42.72, main/busy/ms 21.36, stage/render/specialize 13.69, stage/main/post_update 10.23, stage/render/render 9.25, stage/render/queue 8.79, render/graph/ms 7.77, stage/main/update 7.44, encode//sum 6.98, stage/render/graph_submit 5.88, stage/render/prepare_meshes 5.07 |
| measure | 0.42 | 44.8 | render/thread/ms 60.51, render/busy/ms 59.33, stage/render/render 18.67, main/busy/ms 18.39, encode//sum 17.41, stage/render/specialize 16.83, render/graph/ms 16.64, stage/render/graph_submit 12.67, stage/main/post_update 11.47, encode/pass:shadow_point_light/sum 10.80, stage/render/queue 10.15, encode/submit 7.67 |
| measure | 1.78 | 44.8 | render/thread/ms 49.07, render/busy/ms 47.88, main/busy/ms 19.81, stage/render/specialize 16.44, stage/main/post_update 13.08, stage/render/render 11.29, stage/render/queue 10.30, render/graph/ms 9.65, stage/render/graph_submit 7.63, encode//sum 6.90, stage/render/prepare_meshes 4.83, encode/submit 4.43 |
| measure | 3.26 | 49.6 | render/thread/ms 60.57, render/busy/ms 59.38, main/busy/ms 22.47, stage/render/specialize 18.09, stage/render/render 15.41, stage/main/post_update 14.98, render/graph/ms 13.80, stage/render/queue 12.06, encode//sum 10.24, stage/render/graph_submit 10.05, encode/pass:shadow_point_light/sum 7.72, encode/submit 6.42 |
| measure | 4.77 | 46.9 | render/thread/ms 54.99, render/busy/ms 53.90, main/busy/ms 22.18, stage/render/specialize 17.37, stage/main/post_update 14.92, stage/render/render 12.89, render/graph/ms 11.41, stage/render/queue 11.31, stage/render/graph_submit 8.75, encode//sum 8.51, stage/render/prepare_meshes 5.70, encode/submit 5.52 |
| measure | 6.21 | 47.4 | render/thread/ms 51.19, render/busy/ms 50.10, main/busy/ms 21.48, stage/render/specialize 16.95, stage/main/post_update 13.60, stage/render/render 10.64, stage/render/queue 10.60, render/graph/ms 9.07, encode//sum 7.24, stage/render/graph_submit 6.77, stage/render/prepare_meshes 6.12, stage/main/update 5.18 |
| measure | 7.64 | 47.9 | render/thread/ms 53.16, render/busy/ms 52.17, main/busy/ms 21.60, stage/render/specialize 17.68, stage/main/post_update 14.09, stage/render/queue 11.63, stage/render/render 10.02, render/graph/ms 8.58, encode//sum 7.40, stage/render/prepare_meshes 7.14, stage/render/graph_submit 6.54, stage/main/update 4.37 |
| measure | 9.11 | 45.8 | render/thread/ms 52.32, render/busy/ms 51.19, main/busy/ms 21.74, stage/render/specialize 17.96, stage/main/post_update 14.60, stage/render/queue 11.36, stage/render/render 10.59, render/graph/ms 8.94, stage/render/graph_submit 6.99, encode//sum 6.64, stage/render/prepare_meshes 5.92, stage/main/update 4.20 |
| measure | 10.62 | 46.2 | render/thread/ms 53.33, render/busy/ms 52.34, main/busy/ms 21.88, stage/render/specialize 17.70, stage/main/post_update 14.73, stage/render/queue 11.59, stage/render/render 10.72, render/graph/ms 9.26, encode//sum 7.52, stage/render/graph_submit 7.05, stage/render/prepare_meshes 6.49, stage/main/update 4.46 |
| measure | 15.75 | 51.3 | render/thread/ms 57.13, render/busy/ms 56.13, main/busy/ms 22.41, stage/render/specialize 19.47, stage/main/post_update 15.28, stage/render/queue 12.42, stage/render/render 11.17, render/graph/ms 9.59, encode//sum 7.91, stage/render/graph_submit 7.40, stage/render/prepare_meshes 6.95, stage/main/update 4.38 |
| measure | 17.26 | 46.1 | render/thread/ms 53.21, render/busy/ms 52.22, main/busy/ms 21.74, stage/render/specialize 17.96, stage/main/post_update 15.04, stage/render/queue 11.45, stage/render/render 11.04, render/graph/ms 9.60, stage/render/graph_submit 7.58, encode//sum 7.52, stage/render/prepare_meshes 6.54, encode/submit 4.25 |
| measure | 18.03 | 50.0 | render/thread/ms 54.97, render/busy/ms 53.81, main/busy/ms 24.56, stage/render/specialize 18.38, stage/main/post_update 14.97, stage/render/render 12.04, stage/render/queue 11.59, render/graph/ms 10.44, stage/render/graph_submit 8.11, encode//sum 7.21, stage/main/update 6.30, stage/render/prepare_meshes 5.73 |
| measure | 18.17 | 33.9 | main/busy/ms 44.27, stage/main/update 24.72, render/thread/ms 22.92, render/busy/ms 21.90, stream/drive/ms 21.00, stage/main/post_update 16.16, stage/render/render 10.73, render/graph/ms 9.30, encode//sum 7.43, stage/render/graph_submit 6.87, encode/submit 4.31, stage/render/prepare_assets 3.02 |
| measure | 19.66 | 52.5 | render/thread/ms 54.25, render/busy/ms 53.07, main/busy/ms 30.00, stage/render/specialize 18.20, stage/main/post_update 16.06, stage/render/queue 12.67, stage/render/render 10.77, render/graph/ms 9.18, encode//sum 7.42, stage/render/graph_submit 7.01, stage/main/pre_update 6.06, stage/render/prepare_meshes 5.85 |
| measure | 21.26 | 51.1 | render/thread/ms 56.75, render/busy/ms 55.61, main/busy/ms 23.95, stage/render/specialize 17.38, stage/main/post_update 15.72, stage/render/render 12.42, stage/render/queue 12.05, render/graph/ms 10.83, stage/render/graph_submit 8.31, encode//sum 7.81, stage/render/prepare_meshes 6.87, encode/submit 5.23 |
| measure | 23.08 | 54.5 | render/thread/ms 62.64, render/busy/ms 61.64, main/busy/ms 29.83, stage/render/specialize 20.57, stage/main/post_update 17.04, stage/render/render 13.24, stage/render/queue 13.17, render/graph/ms 11.69, encode//sum 9.11, stage/render/graph_submit 8.94, stage/main/update 7.22, stage/render/prepare_meshes 7.22 |
| measure | 23.13 | 29.7 | render/thread/ms 42.96, render/busy/ms 41.76, main/busy/ms 34.50, stage/main/post_update 16.77, stage/render/render 14.20, render/graph/ms 12.28, stage/main/update 11.67, encode//sum 10.85, stage/render/prepare_assets 9.01, stage/render/graph_submit 8.49, encode/pass:shadow_point_light/sum 8.21, encode/submit 5.36 |
| measure | 24.83 | 50.6 | render/thread/ms 56.72, render/busy/ms 55.66, main/busy/ms 23.78, stage/render/specialize 17.71, stage/main/post_update 15.80, stage/render/render 12.94, stage/render/queue 11.72, render/graph/ms 11.29, stage/render/graph_submit 8.82, encode//sum 8.70, stage/render/prepare_meshes 7.93, encode/submit 5.80 |
| measure | 27.32 | 57.1 | render/thread/ms 61.22, render/busy/ms 60.05, main/busy/ms 27.25, stage/render/specialize 20.11, stage/main/post_update 16.59, stage/render/render 14.25, render/graph/ms 12.63, stage/render/queue 12.59, stage/render/graph_submit 10.06, encode//sum 8.36, encode/pass:shadow_point_light/sum 7.55, stage/render/prepare_meshes 7.28 |
| measure | 29.02 | 50.5 | render/thread/ms 57.23, render/busy/ms 56.17, main/busy/ms 24.79, stage/render/specialize 19.53, stage/main/post_update 17.12, stage/render/render 12.54, stage/render/queue 11.46, render/graph/ms 11.00, encode//sum 8.25, stage/render/graph_submit 8.19, stage/render/prepare_meshes 6.85, encode/submit 5.53 |
| measure | 30.73 | 52.7 | render/thread/ms 58.31, render/busy/ms 57.13, main/busy/ms 25.50, stage/render/specialize 18.46, stage/main/post_update 17.41, stage/render/render 14.15, render/graph/ms 12.50, stage/render/queue 12.00, stage/render/graph_submit 9.63, encode//sum 8.12, encode/pass:shadow_point_light/sum 7.87, stage/render/prepare_meshes 6.97 |
| measure | 32.42 | 47.4 | render/thread/ms 56.57, render/busy/ms 55.31, main/busy/ms 23.64, stage/render/specialize 17.38, stage/main/post_update 16.22, stage/render/render 12.90, stage/render/queue 11.76, render/graph/ms 11.20, stage/render/graph_submit 8.63, encode//sum 8.04, stage/render/prepare_meshes 6.96, encode/submit 5.58 |
| measure | 35.38 | 55.7 | render/thread/ms 61.68, render/busy/ms 60.52, main/busy/ms 25.55, stage/render/specialize 19.93, stage/main/post_update 17.22, stage/render/queue 13.56, stage/render/render 12.93, render/graph/ms 11.34, stage/render/graph_submit 8.67, encode//sum 8.30, stage/render/prepare_meshes 7.74, stage/main/update 4.94 |
| measure | 37.02 | 51.1 | render/thread/ms 58.49, render/busy/ms 57.42, main/busy/ms 24.14, stage/render/specialize 17.39, stage/main/post_update 16.96, stage/render/render 12.54, stage/render/queue 12.42, render/graph/ms 11.10, stage/render/prepare_meshes 9.12, stage/render/graph_submit 8.53, encode//sum 8.50, encode/pass:shadow_point_light/sum 5.72 |
| measure | 37.49 | 47.4 | render/thread/ms 52.38, render/busy/ms 51.36, main/busy/ms 23.18, stage/render/specialize 17.59, stage/main/post_update 16.09, stage/render/render 12.98, render/graph/ms 11.59, stage/render/queue 10.61, stage/render/graph_submit 8.87, encode//sum 8.18, encode/submit 6.12, encode/pass:shadow_point_light/sum 6.01 |
| measure | 40.81 | 47.1 | render/thread/ms 50.90, render/busy/ms 49.81, main/busy/ms 22.75, stage/render/specialize 17.11, stage/main/post_update 14.19, stage/render/queue 10.93, stage/render/render 9.51, render/graph/ms 7.98, encode//sum 6.25, stage/render/graph_submit 6.14, stage/render/prepare_meshes 5.05, stage/main/update 4.71 |
| measure | 43.22 | 46.6 | render/thread/ms 52.82, render/busy/ms 51.70, main/busy/ms 20.90, stage/render/specialize 17.92, stage/main/post_update 13.14, stage/render/queue 11.47, stage/render/render 10.97, render/graph/ms 9.51, stage/render/graph_submit 7.74, encode//sum 6.94, stage/render/prepare_meshes 5.33, stage/main/update 4.14 |
| measure | 45.66 | 46.7 | render/thread/ms 53.67, render/busy/ms 52.30, main/busy/ms 20.94, stage/render/specialize 17.85, stage/main/post_update 12.93, stage/render/render 11.60, stage/render/queue 10.98, render/graph/ms 9.98, stage/render/graph_submit 7.71, encode//sum 6.44, stage/render/prepare_meshes 5.86, encode/submit 4.87 |
| measure | 56.45 | 44.6 | render/thread/ms 47.53, render/busy/ms 46.47, main/busy/ms 17.49, stage/render/specialize 16.46, stage/main/post_update 10.84, stage/render/queue 10.60, stage/render/render 9.55, render/graph/ms 8.23, encode//sum 6.63, stage/render/graph_submit 6.22, stage/render/prepare_meshes 5.21, stage/main/update 3.65 |
| measure | 59.07 | 70.8 | render/thread/ms 83.70, render/busy/ms 82.32, main/busy/ms 31.59, stage/render/prepare_assets 26.18, stage/main/post_update 19.02, stage/render/specialize 17.33, stage/render/render 14.37, render/graph/ms 12.54, encode//sum 11.69, stage/render/queue 11.25, stage/render/graph_submit 9.21, stage/main/update 6.71 |
| measure | 62.60 | 48.8 | render/thread/ms 53.06, render/busy/ms 51.94, main/busy/ms 22.48, stage/render/specialize 17.69, stage/main/post_update 12.96, stage/render/render 11.54, stage/render/queue 11.15, render/graph/ms 10.12, encode//sum 8.50, stage/render/graph_submit 7.91, stage/render/prepare_meshes 6.65, stage/main/update 6.32 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=noprecomb-4 --bench-baseline none --off precombine
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 8%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **69%** (min 34%) · 2625 MHz (min 2625) · memory 8101 MHz · 74.6 W · 77 °C (max 83) · VRAM 6757 MiB · throttled 100% of samples

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
