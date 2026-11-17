# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: precombine · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5494 frames over 65.6 s · **median 11.61 ms (86 fps)** · mean 11.95 · p95 15.00 · p99 19.52 (1% low 51 fps) · worst 65.63 ms · 84 spikes over 1.5× median · 61.2 stutters/min (work 48.4/min)
- warm-up: 322 frames over 9.1 s, worst 2597.5 ms
- hitches (> 29.0 ms): **31** measured, 35 in warm-up
- timed GPU passes: 6.87 ms of a 11.95 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.61 ms, limit 11 ms
- **FAIL** frame_ms: 67 stutters (> 1.5x moving median of 60) in 1.09 min = 61.24/min, limit 1/min
- **FAIL** worst frame 65.63 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.952 | 2.036 | 2.068 | 0.96 | 0.174 |
| main_opaque_pass_3d | 1.539 | 1.606 | 2.015 | 0.96 | 0.060 |
| early prepass | 0.453 | 0.473 | 0.700 | 0.96 | 0.086 |
| bloom | 0.301 | 0.314 | 0.321 | 0.96 | 0.063 |
| atmosphere_luts | 0.292 | 0.305 | 0.367 | 0.96 | 0.003 |
| water_velocity | 0.265 | 0.277 | 0.347 | 0.96 | 0.002 |
| water_sspr | 0.193 | 0.201 | 0.203 | 0.96 | 0.002 |
| ssao | 0.181 | 0.188 | 0.193 | 0.96 | 0.007 |
| atmos_froxel_inject | 0.140 | 0.146 | 0.148 | 0.96 | 0.001 |
| auxiliary_views | 0.135 | 0.141 | 0.481 | 0.96 | 0.149 |
| clouds_trace | 0.134 | 0.140 | 0.229 | 0.96 | 0.001 |
| water_fft | 0.126 | 0.131 | 0.138 | 0.96 | 0.002 |
| uniform_allocation | 0.123 | 0.128 | 0.177 | 0.96 | 0.024 |
| upscaling | 0.112 | 0.117 | 0.124 | 0.96 | 0.004 |
| clouds_composite | 0.101 | 0.106 | 0.109 | 0.96 | 0.002 |
| bin_unpacking | 0.100 | 0.105 | 0.142 | 0.96 | 0.018 |
| main_transmissive_pass_3d | 0.082 | 0.086 | 0.179 | 0.96 | 0.008 |
| tonemapping | 0.082 | 0.085 | 0.088 | 0.96 | 0.003 |
| clustering | 0.061 | 0.064 | 0.067 | 0.96 | 0.018 |
| render_sky | 0.049 | 0.052 | 0.053 | 0.96 | 0.002 |
| lightprobe_irradiance_map | 0.049 | 0.413 | 0.416 | 0.12 | 0.001 |
| clouds_resolve | 0.046 | 0.048 | 0.049 | 0.96 | 0.001 |
| surface_exposure | 0.042 | 0.043 | 0.044 | 0.96 | 0.001 |
| surface_trail | 0.041 | 0.057 | 0.057 | 0.72 | 0.002 |
| early_mesh_preprocessing | 0.037 | 0.039 | 0.062 | 0.96 | 0.004 |
| surface_exposure_far | 0.035 | 0.042 | 0.045 | 0.84 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.96 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.032 | 0.96 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.219 | 0.229 | 0.12 | 0.004 |
| water_snapshot | 0.023 | 0.024 | 0.028 | 0.96 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.96 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.96 | 0.001 |
| main_transparent_pass_3d | 0.012 | 0.013 | 0.026 | 0.96 | 0.122 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.96 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.96 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.96 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.95 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| main/busy/ms | 10.409 | 12.795 | 27.63 | 57186.9 |
| render/thread/ms | 9.681 | 11.914 | 52.99 | 53187.9 |
| render/busy/ms | 9.134 | 11.391 | 52.46 | 50180.6 |
| stage/main/post_update | 6.916 | 8.894 | 14.44 | 37995.1 |
| stage/render/render | 5.550 | 7.016 | 14.75 | 30492.5 |
| render/graph/ms | 4.821 | 6.264 | 13.59 | 26484.8 |
| stage/render/graph_submit | 3.645 | 4.797 | 11.52 | 20023.1 |
| encode//sum | 3.638 | 4.755 | 20.23 | 19989.9 |
| encode/submit | 2.348 | 3.211 | 10.48 | 12899.5 |
| stage/main/update | 2.195 | 2.689 | 19.68 | 12056.8 |
| encode/finish_wall | 1.340 | 2.002 | 4.42 | 7360.1 |
| stage/render/camera_driver | 1.118 | 1.519 | 3.80 | 6142.3 |
| encode/pass:shadow_point_light/sum | 0.915 | 3.177 | 17.16 | 5029.3 |
| encode/pass:shadow_directional_light/sum | 0.855 | 1.571 | 3.33 | 4698.2 |
| stage/render/prepare_write_phase_buffers | 0.768 | 1.117 | 2.40 | 4217.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.760 | 1.066 | 1.93 | 4175.7 |
| encode/pass:early prepass/sum | 0.734 | 1.049 | 2.11 | 4030.3 |
| bigspace/total/ms | 0.598 | 1.367 | 2.58 | 3287.8 |
| stage/render/prepare_assets | 0.570 | 0.732 | 13.10 | 3133.8 |
| stage/render/prepare_meshes | 0.532 | 0.654 | 8.25 | 2921.3 |
| render/present/ms | 0.451 | 0.623 | 2.26 | 2476.5 |
| stage/render/prepare_resources | 0.444 | 0.547 | 1.48 | 2439.0 |
| stage/main/first | 0.405 | 0.527 | 5.42 | 2222.9 |
| stage/render/specialize | 0.378 | 0.427 | 20.20 | 2078.9 |
| stage/main/pre_update | 0.360 | 0.540 | 6.65 | 1980.0 |
| stage/render/queue | 0.340 | 0.486 | 12.63 | 1867.2 |
| terrain/update/ms | 0.306 | 0.390 | 1.77 | 1678.6 |
| stage/render/prepare_bind_groups | 0.280 | 0.487 | 2.56 | 1535.7 |
| terrain/select/ms | 0.272 | 0.348 | 1.02 | 1494.6 |
| controller/feed/ms | 0.262 | 0.328 | 0.95 | 1437.8 |
| stream/drive/ms | 0.253 | 0.352 | 17.48 | 1390.0 |
| bigspace/low_precision/ms | 0.233 | 0.762 | 1.78 | 1278.1 |
| stage/main/fixed_main_loop | 0.232 | 0.372 | 1.43 | 1274.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.227 | 0.332 | 0.82 | 1245.9 |
| stage/render/create_views | 0.202 | 0.259 | 7.70 | 1108.0 |
| far/select/ms | 0.197 | 0.263 | 0.76 | 1084.1 |
| stream/tick/ms | 0.192 | 0.262 | 0.79 | 1053.9 |
| stage/render/prepare_batch_phases | 0.191 | 0.273 | 0.66 | 1051.4 |
| stage/render/extract_commands | 0.172 | 0.201 | 0.48 | 944.5 |
| stage/render/prepare_flush | 0.171 | 0.371 | 1.24 | 941.3 |
| bigspace/high_precision/ms | 0.164 | 0.715 | 2.13 | 902.6 |
| anim/evaluate/ms | 0.164 | 0.279 | 2.46 | 900.3 |
| anim/evaluate_cpu/ms | 0.155 | 0.196 | 0.62 | 852.4 |
| stage/render/prepare_views | 0.140 | 0.178 | 0.54 | 770.6 |
| controller/move/ms | 0.134 | 0.309 | 0.65 | 738.8 |
| stage/main/last | 0.124 | 0.166 | 0.57 | 679.1 |
| render/acquire/ms | 0.097 | 0.122 | 0.41 | 530.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.093 | 0.146 | 0.66 | 508.5 |
| stage/render/prepare_collect_phase_buffers | 0.063 | 0.134 | 0.52 | 345.5 |
| stage/render/cleanup | 0.061 | 0.151 | 1.07 | 336.4 |
| pipelines/compile_ms | 0.054 | 0.000 | 122.60 | 294.3 |
| physics/step/ms | 0.051 | 0.087 | 0.63 | 281.6 |
| pipelines/front_ms | 0.051 | 0.000 | 118.81 | 281.4 |
| physics/insert/ms | 0.037 | 0.396 | 0.96 | 201.3 |
| stage/render/phase_sort | 0.031 | 0.052 | 0.24 | 169.2 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.47 | 169.1 |
| weather/map/ms | 0.024 | 0.033 | 0.24 | 134.1 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.57 | 78.4 |
| anim/apply/ms | 0.012 | 0.020 | 0.38 | 63.7 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.36 | 59.5 |
| controller/avoid/ms | 0.011 | 0.014 | 0.26 | 58.6 |
| weather/eval/ms | 0.010 | 0.014 | 0.09 | 55.6 |
| script/update/ms | 0.007 | 0.009 | 0.26 | 38.3 |
| stream/despawn/ms | 0.006 | 0.000 | 1.06 | 30.7 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 29.3 |
| script/input/ms | 0.005 | 0.006 | 0.14 | 27.9 |
| script/ui/ms | 0.005 | 0.006 | 0.03 | 25.9 |
| controller/write/ms | 0.003 | 0.004 | 0.30 | 18.9 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.71 | 14.8 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.79 | 12.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.53 | 5.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.11 | 5.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.7 |
| stage/render/prepare | 0.001 | 0.002 | 0.09 | 2.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.08 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.3 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 28 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1272 | 2262 | 583 |
| crowd/c0/meshes/count | count | 349 | 646 | 162 |
| crowd/c1/count | count | 147 | 157 | 148 |
| crowd/c1/joints/count | count | 5573 | 6038 | 5613 |
| crowd/c1/meshes/count | count | 1171 | 1324 | 1179 |
| crowd/c2/count | count | 397 | 417 | 417 |
| crowd/c2/joints/count | count | 6402 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4619 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4600 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 83.3 k | 91.8 k | 84.3 k |
| encode//max | ms | 0.39 | 2.35 | 0.47 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.57 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.36 | 0.02 |
| encode/finish_critical | ms | 0.96 | 3.33 | 1.12 |
| encode/pass:early prepass/max | ms | 0.75 | 2.11 | 0.86 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.78 | 1.93 | 1.12 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.66 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 0.82 | 0.17 |
| encode/pass:shadow_directional_light/max | ms | 0.81 | 3.33 | 0.94 |
| encode/pass:shadow_point_light/max | ms | 0.80 | 1.61 | 0.80 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.47 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 10.55 | 52.46 | 8.90 |
| gpu/timed/ms | ms | 7.88 | 11.24 | 8.17 |
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
| phases/alpha_mask3d/items | count | 799 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 799 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1368 | 1546 | 1424 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 5277 | 7410 | 5164 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1368 | 1546 | 1424 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 5277 | 7410 | 5164 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 21 | 195 | 4 |
| phases/shadow/bins | count | 4434 | 7994 | 2618 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 16.8 k | 44.7 k | 12.0 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 199 | 245 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 214 | 476 | 19 |
| physics/pending | count | 0 | 181 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 329 | 330 | 330 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 547 | 6 |
| preprocess/bind_groups_created | count | 13 | 547 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 128 | 227 | 123 |
| process/cpu_usage | % | 26.92 | 41.83 | 23.32 |
| process/mem_usage | GiB | 5.45 | 5.62 | 5.62 |
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
| skin/blocks_changed/count | count | 15 | 122 | 14 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 201 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 366.3 k | 807.8 k | 492.3 k |
| skin/joints_written/count | count | 5724 | 12.6 k | 7692 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 5218 | 5306 | 5306 |
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
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.14 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 742 | 851 | 683 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.96 | 48.72 | 35.23 |
| system/mem_usage | % | 66.81 | 67.35 | 67.34 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 68 | 242 | 54 |
| views/all/bins | count | 7199 | 10.8 k | 6260 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 28.9 k | 59.2 k | 23.6 k |
| views/all/pipelines | count | 243 | 292 | 75 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 14.8 k | 57.0 k | 12.1 k |
| views/cascade/batch_sets | count | 9 | 50 | 4 |
| views/cascade/bins | count | 2940 | 5995 | 2618 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 15.0 k | 42.0 k | 12.0 k |
| views/cascade/pipelines | count | 36 | 53 | 29 |
| views/cascade/work_items | count | 684 | 40.9 k | 182 |
| views/lamp_face/batch_sets | count | 0 | 187 | 0 |
| views/lamp_face/bins | count | 1408 | 2113 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1828 | 2805 | 0 |
| views/lamp_face/pipelines | count | 163 | 197 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2150 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 3148 | 3712 | 3642 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 12.0 k | 17.5 k | 11.6 k |
| views/main/pipelines | count | 46 | 49 | 46 |
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
| anim/bones_written/count | count | 60.35 | 265 | 331.5 k |
| anim/events/count | count | 0.10 | 3 | 569 |
| assets/Image/added | count | 0.11 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.51 | 189 | 2822 |
| assets/Mesh/removed | count | 0.86 | 1146 | 4740 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.92 | 11 | 21.5 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.70 | 1304 | 9344 |
| encode//count | count | 33.85 | 220 | 186.0 k |
| encode/bind_walk/init_textures | count | 12.7 k | 114.7 k | 69.95 M |
| encode/bind_walk/init_textures_skipped | count | 60.0 k | 154.2 k | 329.46 M |
| encode/bind_walk/inits | count | 461 | 1109 | 2.53 M |
| encode/bind_walk/inits_skipped | count | 103 | 150 | 568.5 k |
| encode/bind_walk/merge_textures | count | 12.7 k | 114.6 k | 69.55 M |
| encode/bind_walk/merge_textures_skipped | count | 5379 | 6012 | 29.55 M |
| encode/bind_walk/merges | count | 99.33 | 332 | 545.7 k |
| encode/bind_walk/merges_skipped | count | 81.42 | 126 | 447.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5494 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5494 |
| encode/pass:early prepass/count | count | 1 | 1 | 5494 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5494 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5494 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5494 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5494 |
| encode/pass:shadow_point_light/count | count | 1.46 | 48 | 8037 |
| encode/queue/command_buffers | count | 45.32 | 278 | 249.0 k |
| encode/queue/submits | count | 2.01 | 4 | 11.1 k |
| encode/queue/write_buffer/bytes | bytes | 1.95 M | 6.64 M | 10.72 G |
| encode/queue/write_buffer/calls | count | 259 | 816 | 1.42 M |
| encode/queue/write_buffer_with/bytes | bytes | 172.8 k | 1.30 M | 949.29 M |
| encode/queue/write_buffer_with/calls | count | 10.52 | 576 | 57.8 k |
| encode/queue/write_texture/bytes | bytes | 12.9 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5494 |
| physics/inserts | count | 0.42 | 111 | 2314 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.43 | 145 | 2335 |
| physics/steps | count | 0.72 | 3 | 3937 |
| physics/steps_dropped | count | 0.00 | 1 | 2 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 20 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 351 |
| stream/upload/bytes | bytes | 6162 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.01 | 1 | 28 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1351 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.0 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.60 | 2597.5 | main/busy/ms 2563, stage/main/post_update 2457, stage/main/update 97.92, surface/advance_cpu_ms 68.61, stage/main/pre_update 4.49, stage/main/last 1.04, surface/cell_upload_ms 0.80, stage/main/first 0.57, stream/drive/ms 0.47, stage/main/fixed_main_loop 0.41, stream/tick/ms 0.16, bigspace/total/ms 0.14 |
| warm-up | 3.27 | 675.7 | main/busy/ms 2745, stage/main/post_update 2483, render/thread/ms 643, render/busy/ms 527, stage/render/prepare_assets 500, stage/main/update 248, stage/render/prepare_views 113, render/acquire/ms 113, surface/advance_cpu_ms 68.61, stage/render/render 12.02, stage/render/create_views 9.15, render/graph/ms 7.64 |
| warm-up | 3.39 | 114.9 | render/thread/ms 739, render/busy/ms 569, stage/render/prepare_assets 526, main/busy/ms 203, stage/main/update 154, pipelines/compile_ms 127, stage/render/prepare_views 114, render/acquire/ms 113, pipelines/front_ms 81.78, stage/render/render 72.82, render/present/ms 57.39, pipelines/driver_ms 45.05 |
| warm-up | 3.43 | 41.9 | pipelines/compile_ms 469, pipelines/front_ms 383, pipelines/wait_ms 192, render/thread/ms 136, stage/render/render 94.73, pipelines/driver_ms 85.57, render/busy/ms 74.41, render/present/ms 61.43, main/busy/ms 61.29, stage/main/post_update 46.17, render/graph/ms 32.15, terrain/update/ms 27.76 |
| warm-up | 3.50 | 68.4 | pipelines/wait_ms 2452, pipelines/compile_ms 780, pipelines/front_ms 705, main/busy/ms 107, render/thread/ms 105, render/busy/ms 95.86, pipelines/driver_ms 75.04, stage/main/post_update 48.10, stage/render/render 42.75, render/graph/ms 32.54, stage/main/pre_update 29.23, stage/main/update 25.77 |
| warm-up | 3.60 | 102.0 | pipelines/wait_ms 4152, pipelines/compile_ms 1476, pipelines/front_ms 1403, render/thread/ms 163, main/busy/ms 162, render/busy/ms 159, stage/main/update 88.28, pipelines/driver_ms 72.60, stage/render/prepare_resources 42.99, stage/render/prepare_batch_phases 40.96, stage/main/post_update 35.49, stage/render/prepare_assets 30.98 |
| warm-up | 3.66 | 54.9 | pipelines/wait_ms 2060, pipelines/compile_ms 1683, pipelines/front_ms 1611, render/thread/ms 147, render/busy/ms 145, main/busy/ms 120, pipelines/driver_ms 72.63, stage/main/update 71.10, stage/render/prepare_assets 60.43, stage/main/post_update 34.30, stage/render/prepare_batch_phases 26.91, stage/render/prepare_resources 24.91 |
| warm-up | 3.69 | 36.0 | pipelines/compile_ms 1300, pipelines/front_ms 1228, pipelines/wait_ms 171, render/thread/ms 82.40, render/busy/ms 80.33, pipelines/driver_ms 71.63, stage/render/prepare_assets 55.63, main/busy/ms 44.37, stage/main/post_update 24.61, stage/render/render 18.59, render/graph/ms 16.42, stage/render/graph_submit 13.77 |
| warm-up | 3.85 | 44.0 | pipelines/compile_ms 658, pipelines/front_ms 639, main/busy/ms 52.68, stage/main/update 43.51, stream/drive/ms 39.66, render/thread/ms 25.04, render/busy/ms 23.48, pipelines/driver_ms 18.77, stage/render/prepare_assets 9.89, stage/render/render 8.50, render/graph/ms 6.54, encode//sum 5.40 |
| warm-up | 4.10 | 48.1 | pipelines/compile_ms 294, pipelines/front_ms 285, main/busy/ms 65.10, render/thread/ms 59.66, render/busy/ms 57.75, stage/main/update 27.43, stage/main/post_update 24.84, stage/render/render 12.63, render/graph/ms 10.25, pipelines/driver_ms 9.59, stage/render/graph_submit 7.53, stage/render/prepare_write_phase_buffers 7.25 |
| warm-up | 4.17 | 72.4 | pipelines/compile_ms 1914, pipelines/front_ms 1825, pipelines/wait_ms 111, render/thread/ms 106, render/busy/ms 105, pipelines/driver_ms 89.03, main/busy/ms 83.65, stage/main/update 56.90, stage/main/post_update 19.27, stage/render/prepare_bind_groups 18.95, stage/render/create_views 18.85, stage/render/prepare_assets 12.73 |
| warm-up | 4.23 | 58.0 | pipelines/compile_ms 2143, pipelines/front_ms 2047, pipelines/wait_ms 114, render/thread/ms 111, render/busy/ms 109, pipelines/driver_ms 96.03, main/busy/ms 52.00, stage/main/update 38.05, stage/render/render 28.63, stage/render/prepare_assets 27.75, render/graph/ms 25.98, stage/render/prepare_bind_groups 19.48 |
| warm-up | 4.28 | 54.1 | pipelines/compile_ms 1001, pipelines/front_ms 969, render/thread/ms 98.42, render/busy/ms 93.54, encode//sum 61.64, pipelines/wait_ms 58.77, stage/render/render 56.48, render/graph/ms 50.88, pipelines/driver_ms 32.53, stage/render/graph_submit 31.41, main/busy/ms 24.78, encode/submit 22.25 |
| warm-up | 4.32 | 41.1 | pipelines/compile_ms 968, pipelines/front_ms 936, pipelines/wait_ms 106, render/thread/ms 86.19, render/busy/ms 81.49, encode//sum 62.71, stage/render/render 45.83, render/graph/ms 40.49, pipelines/driver_ms 32.09, stage/render/graph_submit 30.76, main/busy/ms 27.86, encode/submit 23.11 |
| warm-up | 4.38 | 54.4 | pipelines/compile_ms 419, pipelines/front_ms 406, render/thread/ms 84.66, render/busy/ms 82.36, pipelines/wait_ms 50.21, main/busy/ms 32.42, stage/render/render 20.22, render/graph/ms 17.75, stage/render/prepare_assets 15.07, stage/main/update 14.80, encode//sum 13.93, pipelines/driver_ms 13.17 |
| warm-up | 4.42 | 43.0 | pipelines/compile_ms 998, pipelines/front_ms 967, render/thread/ms 85.74, render/busy/ms 83.71, pipelines/wait_ms 50.30, main/busy/ms 48.99, pipelines/driver_ms 31.52, stage/main/update 30.36, stage/render/prepare_flush 17.20, stage/render/prepare_resources 14.58, stage/render/render 14.47, stage/main/post_update 13.15 |
| warm-up | 4.61 | 32.5 | render/thread/ms 49.38, render/busy/ms 48.03, pipelines/compile_ms 27.51, pipelines/front_ms 26.45, main/busy/ms 25.26, stage/main/post_update 12.73, stage/render/render 12.02, stage/render/prepare_resources 11.08, render/graph/ms 10.23, encode//sum 7.26, stage/render/graph_submit 7.08, stage/render/queue 6.44 |
| warm-up | 4.66 | 44.6 | pipelines/compile_ms 442, pipelines/front_ms 432, render/thread/ms 74.72, render/busy/ms 73.30, main/busy/ms 48.42, pipelines/wait_ms 40.39, stage/main/update 28.72, stage/render/prepare_write_phase_buffers 27.49, stage/render/render 11.85, stage/main/post_update 11.09, stage/render/prepare_flush 10.73, pipelines/driver_ms 9.97 |
| warm-up | 4.69 | 31.2 | pipelines/compile_ms 1477, pipelines/front_ms 1442, render/thread/ms 62.65, render/busy/ms 61.08, main/busy/ms 51.12, pipelines/wait_ms 42.59, pipelines/driver_ms 34.55, stage/main/update 33.38, stage/render/prepare_write_phase_buffers 28.03, stage/render/render 11.70, stage/main/post_update 11.53, render/graph/ms 9.31 |
| warm-up | 4.85 | 64.7 | render/thread/ms 57.66, render/busy/ms 55.75, main/busy/ms 54.20, stage/main/update 35.02, stage/render/prepare_assets 16.97, stage/render/render 16.76, render/graph/ms 14.23, stage/main/post_update 11.28, encode//sum 10.68, stage/render/graph_submit 10.22, stage/render/specialize 8.11, encode/submit 7.52 |
| warm-up | 4.89 | 45.7 | pipelines/compile_ms 428, pipelines/front_ms 415, render/thread/ms 70.54, render/busy/ms 69.16, main/busy/ms 38.53, stage/render/prepare_assets 34.13, stage/main/update 22.91, pipelines/driver_ms 13.04, stage/render/render 11.98, encode//sum 10.14, render/graph/ms 10.05, stage/render/specialize 9.26 |
| warm-up | 5.05 | 30.5 | pipelines/compile_ms 187, pipelines/front_ms 182, main/busy/ms 45.39, render/thread/ms 41.50, render/busy/ms 39.83, stage/main/update 24.68, stage/render/render 17.76, render/graph/ms 15.56, stage/main/post_update 13.85, encode//sum 11.36, stream/drive/ms 11.23, stage/render/graph_submit 10.58 |
| warm-up | 5.11 | 56.7 | pipelines/compile_ms 427, pipelines/front_ms 413, render/thread/ms 65.37, render/busy/ms 63.62, main/busy/ms 60.52, stage/main/update 33.65, stage/render/prepare_assets 24.21, stage/render/render 19.21, stage/main/post_update 17.65, render/graph/ms 16.65, pipelines/driver_ms 13.42, encode//sum 11.98 |
| warm-up | 5.15 | 40.9 | pipelines/compile_ms 884, pipelines/front_ms 861, render/thread/ms 65.93, render/busy/ms 64.59, main/busy/ms 44.34, stage/render/prepare_assets 36.91, pipelines/driver_ms 22.51, stage/main/update 19.98, stage/main/post_update 15.69, stage/render/render 12.13, render/graph/ms 9.91, encode//sum 9.24 |
| warm-up | 5.24 | 42.9 | pipelines/compile_ms 563, pipelines/front_ms 546, render/thread/ms 58.56, render/busy/ms 56.78, main/busy/ms 34.28, pipelines/driver_ms 16.94, stage/main/update 15.81, stage/render/prepare_assets 15.35, stage/render/render 15.22, stage/main/post_update 12.90, render/graph/ms 12.35, stage/render/graph_submit 8.71 |
| warm-up | 5.31 | 40.7 | pipelines/compile_ms 255, pipelines/front_ms 248, render/thread/ms 47.06, render/busy/ms 45.67, main/busy/ms 27.90, stage/render/prepare_assets 24.92, stage/main/post_update 11.09, stage/main/update 10.15, stage/render/render 9.81, encode//sum 7.97, render/graph/ms 7.80, pipelines/driver_ms 7.43 |
| warm-up | 5.51 | 35.2 | pipelines/compile_ms 209, pipelines/front_ms 205, render/thread/ms 39.37, render/busy/ms 37.56, main/busy/ms 32.78, stage/main/post_update 15.05, stage/render/render 13.77, stage/main/update 12.01, stage/render/prepare_assets 11.87, render/graph/ms 11.31, encode//sum 9.35, stage/render/graph_submit 8.07 |
| warm-up | 5.61 | 37.7 | pipelines/compile_ms 1157, pipelines/front_ms 1131, main/busy/ms 48.96, stage/main/update 36.48, stream/drive/ms 31.27, render/thread/ms 25.25, pipelines/driver_ms 25.24, render/busy/ms 23.94, stage/render/render 11.97, encode//sum 10.93, render/graph/ms 9.83, stage/main/post_update 7.75 |
| warm-up | 5.68 | 44.4 | main/busy/ms 46.42, stage/main/update 36.08, stream/drive/ms 31.76, render/thread/ms 23.92, render/busy/ms 22.79, stage/render/prepare_assets 8.84, stage/render/render 8.30, stage/main/post_update 7.30, encode//sum 6.91, render/graph/ms 6.72, stage/render/graph_submit 4.68, encode/submit 3.01 |
| warm-up | 5.76 | 37.1 | pipelines/compile_ms 157, pipelines/front_ms 153, main/busy/ms 45.17, stage/main/update 32.62, render/thread/ms 32.15, render/busy/ms 31.02, stream/drive/ms 28.10, stage/render/render 12.97, encode//sum 11.13, render/graph/ms 10.90, stage/render/prepare_assets 9.99, stage/main/post_update 9.03 |
| warm-up | 5.81 | 37.6 | main/busy/ms 38.71, render/thread/ms 28.07, stage/main/update 27.74, render/busy/ms 26.72, stream/drive/ms 19.61, stage/render/render 10.53, render/graph/ms 8.70, stage/main/post_update 7.19, encode//sum 6.80, stage/render/prepare_assets 6.66, stage/render/graph_submit 6.52, encode/submit 5.04 |
| warm-up | 5.86 | 30.7 | main/busy/ms 41.08, stage/main/update 29.67, render/thread/ms 23.69, render/busy/ms 22.45, stream/drive/ms 19.53, stage/render/render 10.12, encode//sum 8.36, render/graph/ms 8.34, stage/main/post_update 7.87, stage/render/graph_submit 5.88, stage/render/prepare_assets 5.46, encode/submit 3.35 |
| warm-up | 6.79 | 37.7 | render/thread/ms 49.41, render/busy/ms 48.42, stage/render/render 36.60, render/graph/ms 35.29, stage/render/camera_driver 29.06, main/busy/ms 20.63, pipelines/compile_ms 20.45, pipelines/front_ms 19.30, stage/main/post_update 9.22, encode//sum 7.78, stage/main/update 7.01, stage/render/graph_submit 6.13 |
| warm-up | 8.06 | 43.6 | render/thread/ms 48.72, render/busy/ms 47.67, main/busy/ms 21.60, stage/render/specialize 15.36, stage/main/post_update 10.85, stage/render/queue 10.79, stage/render/render 10.61, render/graph/ms 9.18, stage/main/update 7.58, encode//sum 6.96, stage/render/graph_submit 6.83, stage/render/prepare_meshes 4.72 |
| measure | 0.48 | 44.7 | render/thread/ms 49.67, render/busy/ms 48.61, main/busy/ms 19.62, stage/render/specialize 16.34, stage/main/post_update 12.48, stage/render/render 11.58, stage/render/queue 10.13, render/graph/ms 10.11, encode//sum 8.02, stage/render/graph_submit 7.76, stage/render/prepare_meshes 4.99, encode/submit 4.85 |
| measure | 1.89 | 45.5 | render/thread/ms 51.40, render/busy/ms 50.26, main/busy/ms 19.18, stage/render/specialize 17.09, stage/main/post_update 12.03, stage/render/render 11.31, stage/render/queue 11.22, render/graph/ms 9.84, encode//sum 8.80, stage/render/graph_submit 7.40, stage/render/prepare_meshes 5.09, stage/main/update 4.46 |
| measure | 2.72 | 30.4 | render/thread/ms 35.40, render/busy/ms 34.26, main/busy/ms 21.40, stage/render/render 18.18, encode//sum 17.28, render/graph/ms 16.55, encode/pass:shadow_point_light/sum 15.81, stage/main/post_update 13.76, stage/render/graph_submit 11.27, stage/render/create_views 7.92, encode/submit 7.08, stage/render/camera_driver 5.16 |
| measure | 3.40 | 47.4 | render/thread/ms 52.10, render/busy/ms 50.96, main/busy/ms 21.10, stage/render/specialize 17.03, stage/main/post_update 13.70, stage/render/render 11.93, stage/render/queue 10.81, render/graph/ms 10.31, stage/render/graph_submit 7.94, encode//sum 7.41, stage/render/prepare_meshes 5.29, stage/main/update 5.01 |
| measure | 4.95 | 53.3 | render/thread/ms 54.15, render/busy/ms 53.13, main/busy/ms 25.00, stage/render/specialize 18.89, stage/main/post_update 15.89, stage/render/queue 11.97, stage/render/render 10.86, render/graph/ms 9.39, encode//sum 7.99, stage/render/graph_submit 7.10, stage/render/prepare_meshes 6.48, stage/main/update 5.28 |
| measure | 6.44 | 46.0 | render/thread/ms 52.18, render/busy/ms 51.13, main/busy/ms 21.30, stage/render/specialize 17.34, stage/main/post_update 13.98, stage/render/render 11.75, stage/render/queue 11.17, render/graph/ms 10.30, encode//sum 9.05, stage/render/graph_submit 7.44, stage/render/prepare_meshes 5.64, encode/pass:shadow_point_light/sum 4.78 |
| measure | 7.90 | 46.6 | render/thread/ms 50.48, render/busy/ms 49.31, main/busy/ms 21.91, stage/render/specialize 17.28, stage/main/post_update 14.79, stage/render/queue 10.65, stage/render/render 10.46, render/graph/ms 8.96, encode//sum 6.93, stage/render/graph_submit 6.53, stage/render/prepare_meshes 6.20, stage/main/update 4.39 |
| measure | 9.38 | 47.7 | render/thread/ms 48.61, render/busy/ms 47.55, main/busy/ms 21.96, stage/render/specialize 17.77, stage/main/post_update 14.47, stage/render/queue 10.96, stage/render/render 8.81, render/graph/ms 7.42, encode//sum 6.30, stage/render/prepare_meshes 5.64, stage/render/graph_submit 5.47, stage/main/update 4.42 |
| measure | 10.89 | 47.1 | render/thread/ms 52.98, render/busy/ms 51.74, main/busy/ms 23.12, stage/render/specialize 17.71, stage/main/post_update 15.32, stage/render/queue 11.46, stage/render/render 10.31, render/graph/ms 8.75, stage/render/prepare_meshes 7.27, encode//sum 6.72, stage/render/graph_submit 6.57, stage/main/update 4.57 |
| measure | 15.74 | 52.4 | render/thread/ms 55.62, render/busy/ms 54.57, main/busy/ms 23.60, stage/render/specialize 18.66, stage/main/post_update 16.13, stage/render/queue 12.14, stage/render/render 11.92, render/graph/ms 10.41, stage/render/graph_submit 7.88, encode//sum 7.40, stage/render/prepare_meshes 6.24, encode/submit 4.79 |
| measure | 17.24 | 48.3 | render/thread/ms 52.64, render/busy/ms 51.65, main/busy/ms 21.03, stage/render/specialize 17.41, stage/main/post_update 14.18, stage/render/queue 11.69, stage/render/render 10.97, render/graph/ms 9.38, encode//sum 8.00, stage/render/graph_submit 7.15, stage/render/prepare_meshes 6.31, encode/submit 4.14 |
| measure | 18.02 | 47.6 | render/thread/ms 52.98, render/busy/ms 51.68, main/busy/ms 21.36, stage/render/specialize 17.58, stage/main/post_update 14.41, stage/render/queue 11.51, stage/render/render 10.35, render/graph/ms 8.65, encode//sum 6.88, stage/render/graph_submit 6.68, stage/render/prepare_meshes 6.24, stage/main/update 4.23 |
| measure | 18.17 | 32.1 | main/busy/ms 39.97, stage/main/update 22.01, render/thread/ms 21.22, render/busy/ms 20.13, stream/drive/ms 17.74, stage/main/post_update 14.75, stage/render/render 10.48, render/graph/ms 8.90, encode//sum 7.85, stage/render/graph_submit 6.40, encode/submit 4.38, stage/render/prepare_assets 3.18 |
| measure | 19.60 | 50.7 | render/thread/ms 55.65, render/busy/ms 54.41, main/busy/ms 22.69, stage/render/specialize 17.80, stage/main/post_update 15.04, stage/render/queue 12.16, stage/render/render 11.69, render/graph/ms 10.11, encode//sum 8.06, stage/render/graph_submit 7.58, stage/render/prepare_meshes 6.91, stage/main/update 4.94 |
| measure | 21.18 | 50.4 | render/thread/ms 54.15, render/busy/ms 53.07, main/busy/ms 23.00, stage/render/specialize 17.64, stage/main/post_update 16.00, stage/render/render 11.40, stage/render/queue 11.15, render/graph/ms 9.88, encode//sum 7.51, stage/render/graph_submit 7.22, stage/render/prepare_meshes 7.07, stage/main/update 4.35 |
| measure | 23.06 | 54.4 | render/thread/ms 61.84, render/busy/ms 60.75, main/busy/ms 28.88, stage/render/specialize 20.24, stage/main/post_update 16.01, stage/render/queue 13.12, stage/render/render 13.01, render/graph/ms 11.39, stage/render/graph_submit 8.91, encode//sum 8.14, stage/render/prepare_meshes 7.64, stage/main/update 7.01 |
| measure | 24.77 | 48.5 | render/thread/ms 55.89, render/busy/ms 54.63, main/busy/ms 25.47, stage/render/specialize 18.22, stage/main/post_update 16.46, stage/render/render 12.39, stage/render/queue 11.36, render/graph/ms 10.69, encode//sum 8.96, stage/render/graph_submit 8.29, stage/render/prepare_meshes 6.73, stage/main/update 5.91 |
| measure | 27.30 | 55.8 | render/thread/ms 62.72, render/busy/ms 61.57, main/busy/ms 29.87, stage/render/specialize 20.49, stage/main/post_update 17.16, stage/render/render 14.67, render/graph/ms 13.09, stage/render/queue 12.77, encode//sum 11.16, stage/render/graph_submit 10.29, encode/pass:shadow_point_light/sum 8.18, stage/render/prepare_meshes 7.99 |
| measure | 28.96 | 51.1 | render/thread/ms 55.12, render/busy/ms 54.11, main/busy/ms 25.70, stage/render/specialize 17.70, stage/main/post_update 17.43, stage/render/queue 12.47, stage/render/render 11.41, render/graph/ms 9.99, encode//sum 8.70, stage/render/graph_submit 7.71, stage/render/prepare_meshes 6.68, encode/submit 4.45 |
| measure | 30.67 | 49.1 | render/thread/ms 55.67, render/busy/ms 54.58, main/busy/ms 30.75, stage/render/specialize 18.15, stage/main/post_update 16.88, stage/render/render 12.54, stage/render/queue 11.71, render/graph/ms 11.07, encode//sum 9.52, stage/render/graph_submit 8.34, stage/main/pre_update 6.96, stage/render/prepare_meshes 6.77 |
| measure | 32.41 | 52.4 | render/thread/ms 57.71, render/busy/ms 56.46, main/busy/ms 25.41, stage/render/specialize 17.83, stage/main/post_update 17.74, stage/render/render 13.77, render/graph/ms 12.09, stage/render/queue 11.81, encode//sum 10.94, stage/render/graph_submit 9.26, encode/pass:shadow_point_light/sum 8.15, stage/render/prepare_meshes 6.67 |
| measure | 35.36 | 55.1 | render/thread/ms 58.37, render/busy/ms 57.32, main/busy/ms 25.53, stage/render/specialize 18.41, stage/main/post_update 17.46, stage/render/render 12.25, stage/render/queue 11.89, render/graph/ms 10.77, stage/render/prepare_meshes 8.69, encode//sum 8.68, stage/render/graph_submit 8.45, encode/submit 4.80 |
| measure | 36.95 | 45.0 | render/thread/ms 52.18, render/busy/ms 51.17, main/busy/ms 25.20, stage/render/specialize 17.81, stage/main/post_update 17.64, stage/render/render 11.35, stage/render/queue 10.87, render/graph/ms 9.91, encode//sum 8.07, stage/render/graph_submit 7.58, stage/render/prepare_meshes 5.01, stage/main/update 4.54 |
| measure | 37.50 | 48.7 | render/thread/ms 50.64, render/busy/ms 49.49, main/busy/ms 24.60, stage/render/specialize 17.55, stage/main/post_update 17.20, stage/render/queue 10.98, stage/render/render 10.50, render/graph/ms 8.93, stage/render/graph_submit 7.09, encode//sum 6.54, stage/render/prepare_meshes 5.21, stage/main/update 4.37 |
| measure | 40.81 | 46.7 | render/thread/ms 53.19, render/busy/ms 52.21, main/busy/ms 21.85, stage/render/specialize 17.31, stage/main/post_update 13.94, stage/render/queue 11.35, stage/render/render 10.45, render/graph/ms 9.09, stage/render/graph_submit 7.17, encode//sum 6.77, stage/render/prepare_meshes 5.45, stage/main/update 4.63 |
| measure | 43.19 | 44.7 | render/thread/ms 51.40, render/busy/ms 50.30, main/busy/ms 21.02, stage/render/specialize 17.53, stage/main/post_update 13.81, stage/render/queue 10.81, stage/render/render 10.39, render/graph/ms 9.09, stage/render/graph_submit 7.39, encode//sum 6.58, stage/render/prepare_meshes 5.60, stage/main/update 4.04 |
| measure | 45.68 | 47.6 | render/thread/ms 52.69, render/busy/ms 51.43, main/busy/ms 21.97, stage/render/specialize 17.85, stage/main/post_update 13.45, stage/render/render 10.98, stage/render/queue 10.64, render/graph/ms 9.43, encode//sum 7.72, stage/render/graph_submit 7.46, stage/render/prepare_meshes 5.88, stage/main/update 5.09 |
| measure | 56.45 | 42.7 | render/thread/ms 48.62, render/busy/ms 47.64, main/busy/ms 18.77, stage/render/specialize 16.97, stage/main/post_update 12.39, stage/render/queue 9.91, stage/render/render 9.63, render/graph/ms 8.37, encode//sum 6.68, stage/render/graph_submit 6.30, stage/render/prepare_meshes 5.60, stage/main/update 4.08 |
| measure | 59.05 | 65.6 | render/thread/ms 72.05, render/busy/ms 70.90, main/busy/ms 30.49, stage/render/prepare_assets 19.60, stage/render/specialize 17.04, stage/main/post_update 16.71, stage/render/render 10.89, stage/render/queue 10.64, render/graph/ms 9.14, encode//sum 8.77, stage/main/update 7.88, stage/render/graph_submit 6.31 |
| measure | 62.59 | 46.7 | render/thread/ms 51.87, render/busy/ms 50.68, main/busy/ms 23.17, stage/render/specialize 18.01, stage/main/post_update 13.22, stage/render/queue 10.92, stage/render/render 10.64, render/graph/ms 9.21, encode//sum 7.84, stage/render/graph_submit 6.96, stage/main/update 6.89, stage/render/prepare_meshes 5.89 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=noprecomb-3 --bench-baseline none --off precombine
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 3%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **70%** (min 26%) · 2622 MHz (min 2610) · memory 8101 MHz · 75.6 W · 76 °C (max 83) · VRAM 6755 MiB · throttled 100% of samples

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
