# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: precombine · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1564 frames over 15.0 s · **median 9.59 ms (104 fps)** · mean 9.60 · p95 11.00 · p99 11.97 (1% low 84 fps) · worst 41.39 ms · 3 spikes over 1.5× median · 12.0 stutters/min (work 4.0/min)
- warm-up: 316 frames over 8.2 s, worst 2450.8 ms
- hitches (> 25.0 ms): **1** measured, 37 in warm-up
- timed GPU passes: 8.38 ms of a 9.60 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.59 ms, limit 11 ms
- **FAIL** worst frame 41.39 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.797 | 2.828 | 2.945 | 0.99 | 0.062 |
| dlss_super_resolution | 2.004 | 2.026 | 2.049 | 0.99 | 0.179 |
| early prepass | 0.583 | 0.589 | 0.604 | 0.99 | 0.095 |
| atmosphere_luts | 0.336 | 0.339 | 0.395 | 0.99 | 0.003 |
| bloom | 0.311 | 0.314 | 0.330 | 0.99 | 0.062 |
| main_transmissive_pass_3d | 0.294 | 0.297 | 0.331 | 0.99 | 0.008 |
| water_velocity | 0.237 | 0.240 | 0.266 | 0.99 | 0.002 |
| water_sspr | 0.208 | 0.210 | 0.213 | 0.99 | 0.002 |
| ssao | 0.187 | 0.189 | 0.197 | 0.99 | 0.007 |
| clouds_trace | 0.148 | 0.150 | 0.170 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.144 | 0.146 | 0.151 | 0.99 | 0.001 |
| water_fft | 0.129 | 0.131 | 0.134 | 0.99 | 0.003 |
| uniform_allocation | 0.119 | 0.120 | 0.161 | 0.99 | 0.021 |
| upscaling | 0.113 | 0.114 | 0.120 | 0.99 | 0.004 |
| clouds_composite | 0.111 | 0.113 | 0.115 | 0.99 | 0.002 |
| bin_unpacking | 0.091 | 0.092 | 0.122 | 0.99 | 0.015 |
| tonemapping | 0.085 | 0.086 | 0.092 | 0.99 | 0.003 |
| clustering | 0.065 | 0.065 | 0.077 | 0.99 | 0.020 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.414 | 0.414 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.051 | 0.052 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.038 | 0.039 | 0.052 | 0.99 | 0.005 |
| clouds_shadow | 0.031 | 0.032 | 0.035 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.221 | 0.232 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.026 | 0.029 | 0.99 | 0.083 |
| water_snapshot | 0.024 | 0.025 | 0.026 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| surface_exposure_far | 0.012 | 0.043 | 0.045 | 0.29 | 0.002 |
| surface_exposure | 0.011 | 0.042 | 0.043 | 0.27 | 0.001 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.007 | 0.007 | 0.020 | 0.99 | 0.005 |
| night_sky | 0.006 | 0.006 | 0.007 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.606 | 9.958 | 39.73 | 13459.2 |
| main/busy/ms | 7.658 | 8.614 | 10.32 | 11976.5 |
| render/busy/ms | 6.830 | 7.968 | 39.15 | 10681.6 |
| stage/main/post_update | 4.725 | 5.516 | 6.89 | 7390.4 |
| stage/render/render | 4.534 | 5.598 | 8.98 | 7091.0 |
| render/graph/ms | 3.958 | 4.920 | 7.92 | 6190.0 |
| encode//sum | 3.181 | 4.771 | 19.72 | 4974.3 |
| stage/render/graph_submit | 2.974 | 3.941 | 6.83 | 4651.7 |
| encode/submit | 1.913 | 2.619 | 4.92 | 2991.5 |
| stage/main/update | 1.832 | 2.279 | 4.42 | 2865.7 |
| pipelines/compile_ms | 1.620 | 0.000 | 136.19 | 2533.8 |
| pipelines/front_ms | 1.574 | 0.000 | 133.18 | 2461.4 |
| stage/render/prepare_views | 1.463 | 3.026 | 3.95 | 2288.1 |
| render/acquire/ms | 1.418 | 2.978 | 3.91 | 2218.0 |
| encode/finish_wall | 1.084 | 1.474 | 3.23 | 1695.0 |
| stage/render/camera_driver | 0.930 | 1.081 | 1.54 | 1453.8 |
| encode/pass:early prepass/sum | 0.735 | 1.004 | 2.13 | 1150.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.716 | 0.968 | 2.23 | 1120.2 |
| stage/render/prepare_write_phase_buffers | 0.527 | 0.740 | 0.94 | 824.7 |
| stage/render/prepare_assets | 0.440 | 0.567 | 1.57 | 688.3 |
| bigspace/total/ms | 0.412 | 0.815 | 1.89 | 645.1 |
| stage/render/prepare_resources | 0.381 | 0.437 | 0.56 | 595.2 |
| render/present/ms | 0.358 | 0.446 | 2.19 | 559.6 |
| stage/main/pre_update | 0.336 | 0.412 | 0.90 | 525.8 |
| stage/main/first | 0.289 | 0.352 | 1.65 | 452.8 |
| encode/pass:shadow_directional_light/sum | 0.288 | 1.063 | 1.92 | 451.1 |
| terrain/update/ms | 0.229 | 0.290 | 0.59 | 357.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.226 | 0.343 | 0.68 | 353.2 |
| stage/render/prepare_bind_groups | 0.206 | 0.278 | 0.40 | 322.7 |
| stage/render/prepare_meshes | 0.204 | 0.244 | 4.91 | 318.6 |
| terrain/select/ms | 0.199 | 0.252 | 0.56 | 310.9 |
| stream/drive/ms | 0.185 | 0.249 | 0.64 | 289.5 |
| stage/render/prepare_batch_phases | 0.178 | 0.238 | 0.36 | 278.3 |
| stage/main/fixed_main_loop | 0.174 | 0.322 | 0.87 | 271.6 |
| stage/render/create_views | 0.168 | 0.218 | 0.75 | 263.4 |
| stage/render/extract_commands | 0.168 | 0.199 | 0.65 | 263.2 |
| far/select/ms | 0.167 | 0.227 | 0.66 | 260.5 |
| stream/tick/ms | 0.155 | 0.210 | 0.61 | 242.1 |
| controller/move/ms | 0.155 | 0.308 | 0.71 | 241.7 |
| stage/render/prepare_flush | 0.148 | 0.274 | 0.87 | 232.0 |
| controller/feed/ms | 0.138 | 0.185 | 0.61 | 215.6 |
| bigspace/high_precision/ms | 0.137 | 0.413 | 1.53 | 213.8 |
| stage/main/last | 0.123 | 0.161 | 0.67 | 191.7 |
| stage/render/queue | 0.119 | 0.151 | 10.00 | 185.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.102 | 0.160 | 0.30 | 159.2 |
| stage/render/specialize | 0.098 | 0.140 | 16.15 | 152.7 |
| bigspace/low_precision/ms | 0.095 | 0.228 | 1.27 | 148.2 |
| anim/evaluate/ms | 0.086 | 0.212 | 1.04 | 135.1 |
| anim/evaluate_cpu/ms | 0.060 | 0.076 | 0.19 | 94.1 |
| stage/render/prepare_collect_phase_buffers | 0.056 | 0.112 | 0.20 | 88.0 |
| pipelines/driver_ms | 0.046 | 0.000 | 4.26 | 72.4 |
| encode/resolve_diagnostics/sum | 0.042 | 0.075 | 0.42 | 65.6 |
| physics/step/ms | 0.025 | 0.052 | 0.08 | 39.8 |
| weather/map/ms | 0.024 | 0.032 | 0.19 | 37.5 |
| stage/render/cleanup | 0.023 | 0.035 | 0.26 | 35.9 |
| encode/camera_driver_span_begin/sum | 0.020 | 0.036 | 0.09 | 30.7 |
| stage/render/phase_sort | 0.017 | 0.031 | 0.16 | 26.5 |
| encode/camera_driver_span_end/sum | 0.017 | 0.032 | 0.34 | 26.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.03 | 16.8 |
| script/update/ms | 0.006 | 0.009 | 0.03 | 10.0 |
| controller/avoid/ms | 0.006 | 0.008 | 0.08 | 9.0 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 7.9 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 6.9 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.4 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.1 |
| stage/render/prepare | 0.001 | 0.002 | 0.12 | 1.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1080 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 68.4 k | 68.4 k | 68.4 k |
| encode//max | ms | 0.42 | 3.06 | 0.48 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.09 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.34 | 0.01 |
| encode/finish_critical | ms | 0.81 | 3.06 | 0.69 |
| encode/pass:early prepass/max | ms | 0.73 | 2.13 | 0.60 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.71 | 2.23 | 0.69 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.30 | 0.12 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.21 | 0.68 | 0.27 |
| encode/pass:shadow_directional_light/max | ms | 0.08 | 1.92 | 0.05 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.42 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.14 | 39.15 | 8.96 |
| gpu/timed/ms | ms | 9.14 | 9.97 | 8.96 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1943 | 1943 | 1943 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 207.2 k | 207.2 k | 207.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 10.5 k | 10.5 k | 10.5 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 321.65 M | 321.65 M | 321.65 M |
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
| phases/opaque3d/bins | count | 1408 | 1408 | 1408 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 6339 | 6339 | 6339 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/bins | count | 1408 | 1408 | 1408 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 6339 | 6339 | 6339 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 0 | 29 | 0 |
| phases/shadow/bins | count | 2028 | 4311 | 2028 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.6 k | 33.4 k | 10.6 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 22 | 29 | 22 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 320 | 320 | 320 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 22 | 0 |
| preprocess/bind_groups | count | 0 | 76 | 0 |
| preprocess/bind_groups_created | count | 0 | 76 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 27.01 | 43.75 | 25 |
| process/mem_usage | GiB | 4.70 | 4.75 | 4.73 |
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
| started_load_count |  | 4026 | 4028 | 4028 |
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
| stream/latency/s | s | 0.54 | 0.54 | 0.54 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 656 | 656 | 656 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.80 | 49.55 | 27.77 |
| system/mem_usage | % | 54.52 | 54.69 | 54.63 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 52 | 81 | 52 |
| views/all/bins | count | 5404 | 7687 | 5404 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 25.1 k | 47.8 k | 25.1 k |
| views/all/pipelines | count | 70 | 77 | 70 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 14.6 k | 48.0 k | 14.6 k |
| views/cascade/batch_sets | count | 0 | 29 | 0 |
| views/cascade/bins | count | 2028 | 4311 | 2028 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 10.6 k | 33.4 k | 10.6 k |
| views/cascade/pipelines | count | 22 | 29 | 22 |
| views/cascade/work_items | count | 0 | 33.4 k | 0 |
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
| views/main/bins | count | 3376 | 3376 | 3376 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 14.4 k | 14.4 k | 14.4 k |
| views/main/pipelines | count | 48 | 48 | 48 |
| views/main/work_items | count | 14.6 k | 14.6 k | 14.6 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 46.1 k | 46.1 k | 46.1 k |
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
| assets/StandardMaterial/modified | count | 3.16 | 11 | 4939 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.54 | 29 | 41.5 k |
| encode/bind_walk/init_textures | count | 5225 | 12.1 k | 8.17 M |
| encode/bind_walk/init_textures_skipped | count | 39.5 k | 52.6 k | 61.76 M |
| encode/bind_walk/inits | count | 336 | 429 | 525.3 k |
| encode/bind_walk/inits_skipped | count | 71 | 83 | 111.0 k |
| encode/bind_walk/merge_textures | count | 5162 | 12.0 k | 8.07 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.38 M |
| encode/bind_walk/merges | count | 78.26 | 99 | 122.4 k |
| encode/bind_walk/merges_skipped | count | 52.75 | 58 | 82.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1564 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1564 |
| encode/pass:early prepass/count | count | 1 | 1 | 1564 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1564 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1564 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1564 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1564 |
| encode/queue/command_buffers | count | 36.54 | 39 | 57.1 k |
| encode/queue/submits | count | 2 | 2 | 3128 |
| encode/queue/write_buffer/bytes | bytes | 1.49 M | 5.49 M | 2.33 G |
| encode/queue/write_buffer/calls | count | 212 | 283 | 331.3 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 51.05 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 14.1 k |
| encode/queue/write_texture/bytes | bytes | 11.5 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1564 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 3 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 36 |
| pipelines/created | count | 0.02 | 1 | 36 |
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

## Hitches (> 25.0 ms; slowest 38, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.45 | 2450.8 | main/busy/ms 2422, stage/main/post_update 2319, stage/main/update 94.20, surface/advance_cpu_ms 66.20, stage/main/pre_update 5.68, stage/main/last 1.12, surface/cell_upload_ms 0.75, stage/main/first 0.55, stream/drive/ms 0.41, stage/main/fixed_main_loop 0.34, weather/map/ms 0.17, bigspace/total/ms 0.10 |
| warm-up | 3.27 | 815.0 | main/busy/ms 2598, stage/main/post_update 2347, render/thread/ms 778, render/busy/ms 641, stage/render/prepare_assets 593, stage/main/update 236, stage/render/prepare_views 134, render/acquire/ms 134, surface/advance_cpu_ms 66.20, stage/render/create_views 27.91, stage/render/render 12.11, render/graph/ms 7.97 |
| warm-up | 3.38 | 112.8 | render/thread/ms 869, render/busy/ms 680, stage/render/prepare_assets 618, main/busy/ms 198, stage/main/update 146, stage/render/prepare_views 135, render/acquire/ms 134, stage/render/render 67.81, pipelines/compile_ms 65.48, pipelines/front_ms 61.69, render/present/ms 53.69, stage/main/post_update 41.03 |
| warm-up | 3.42 | 42.7 | pipelines/compile_ms 288, pipelines/front_ms 248, render/thread/ms 118, stage/render/render 76.95, main/busy/ms 65.50, render/present/ms 61.31, render/busy/ms 55.62, stage/main/post_update 48.11, pipelines/driver_ms 39.28, terrain/update/ms 30.19, stage/render/prepare_assets 26.35, pipelines/wait_ms 16.90 |
| warm-up | 3.52 | 99.5 | pipelines/wait_ms 3803, pipelines/compile_ms 1242, pipelines/front_ms 1140, render/thread/ms 122, render/busy/ms 110, pipelines/driver_ms 102, main/busy/ms 57.71, stage/main/post_update 45.21, stage/render/render 25.62, stage/render/prepare_resources 24.70, stage/render/prepare_bind_groups 22.73, stage/render/prepare_flush 15.29 |
| warm-up | 3.58 | 59.1 | pipelines/wait_ms 4547, pipelines/compile_ms 1674, pipelines/front_ms 1571, render/thread/ms 133, render/busy/ms 124, pipelines/driver_ms 104, main/busy/ms 82.13, stage/main/update 54.44, stage/render/prepare_resources 39.46, stage/render/prepare_bind_groups 23.46, stage/main/post_update 22.95, stage/render/prepare_assets 18.91 |
| warm-up | 3.67 | 90.8 | pipelines/compile_ms 1311, pipelines/front_ms 1251, pipelines/wait_ms 891, render/thread/ms 97.52, render/busy/ms 87.64, main/busy/ms 81.33, pipelines/driver_ms 60.66, stage/main/update 56.77, stage/render/prepare_assets 54.04, stage/main/post_update 18.45, stage/render/prepare_resources 16.90, terrain/update/ms 15.10 |
| warm-up | 3.70 | 29.9 | pipelines/compile_ms 1361, pipelines/front_ms 1296, pipelines/wait_ms 262, render/thread/ms 85.83, render/busy/ms 83.84, stage/render/prepare_assets 64.50, pipelines/driver_ms 64.21, main/busy/ms 44.37, stage/main/post_update 14.61, stage/render/render 11.72, stage/main/fixed_main_loop 11.62, terrain/update/ms 10.99 |
| warm-up | 3.73 | 26.8 | pipelines/compile_ms 948, pipelines/front_ms 895, pipelines/wait_ms 157, pipelines/driver_ms 53.70, render/thread/ms 49.32, render/busy/ms 47.69, main/busy/ms 41.58, stage/render/prepare_assets 25.29, encode//sum 17.43, stage/main/post_update 15.17, stage/render/render 13.69, render/graph/ms 11.63 |
| warm-up | 3.83 | 33.5 | pipelines/compile_ms 247, pipelines/front_ms 239, main/busy/ms 39.71, stage/main/update 30.76, stream/drive/ms 27.25, render/thread/ms 22.58, render/busy/ms 21.22, stage/render/prepare_assets 8.60, pipelines/driver_ms 8.15, stage/render/render 8.12, render/graph/ms 6.39, encode//sum 5.80 |
| warm-up | 3.95 | 33.5 | pipelines/compile_ms 188, pipelines/front_ms 184, main/busy/ms 29.24, render/thread/ms 27.46, render/busy/ms 26.02, stage/main/update 17.98, stage/render/prepare_assets 13.83, stream/drive/ms 12.75, stage/render/render 8.09, stage/main/post_update 7.35, render/graph/ms 6.24, encode//sum 6.10 |
| warm-up | 4.04 | 28.6 | pipelines/compile_ms 299, pipelines/front_ms 290, main/busy/ms 33.42, render/thread/ms 23.42, render/busy/ms 22.07, stage/main/update 22.00, stage/render/render 9.98, encode//sum 8.47, pipelines/driver_ms 8.26, render/graph/ms 7.69, stage/main/post_update 7.66, stage/render/graph_submit 5.23 |
| warm-up | 4.11 | 37.0 | pipelines/compile_ms 254, pipelines/front_ms 246, main/busy/ms 40.83, render/thread/ms 25.11, stage/main/update 24.99, render/busy/ms 23.65, stream/drive/ms 19.76, stage/main/post_update 9.41, stage/render/prepare_assets 9.24, stage/render/render 8.76, pipelines/driver_ms 8.09, render/graph/ms 7.03 |
| warm-up | 4.15 | 40.3 | pipelines/compile_ms 93.33, pipelines/front_ms 88.42, render/thread/ms 53.45, main/busy/ms 51.48, render/busy/ms 50.59, stage/main/post_update 25.00, stage/render/render 24.46, render/graph/ms 21.15, stage/main/update 19.01, stage/render/graph_submit 9.56, encode//sum 8.08, stage/render/prepare_assets 7.27 |
| warm-up | 4.19 | 48.4 | pipelines/compile_ms 753, pipelines/front_ms 727, render/thread/ms 77.13, render/busy/ms 74.27, pipelines/wait_ms 64.98, main/busy/ms 56.58, pipelines/driver_ms 26.28, stage/main/post_update 26.25, stage/render/render 24.36, stage/main/update 23.31, stage/render/prepare_assets 20.82, render/graph/ms 20.80 |
| warm-up | 4.23 | 35.4 | pipelines/compile_ms 1177, pipelines/front_ms 1138, pipelines/wait_ms 83.22, render/thread/ms 69.20, render/busy/ms 67.74, main/busy/ms 44.56, pipelines/driver_ms 39.61, stage/render/prepare_assets 27.42, stage/main/update 16.65, stage/main/post_update 14.93, stage/render/render 9.62, stage/render/prepare_resources 8.72 |
| warm-up | 4.33 | 28.5 | pipelines/compile_ms 593, pipelines/front_ms 575, render/thread/ms 43.43, render/busy/ms 41.97, main/busy/ms 25.02, pipelines/driver_ms 17.96, pipelines/wait_ms 17.67, stage/render/prepare_assets 16.00, stage/main/post_update 12.71, stage/render/render 9.95, render/graph/ms 8.31, stage/main/update 7.56 |
| warm-up | 4.41 | 67.9 | pipelines/compile_ms 1031, pipelines/front_ms 991, pipelines/wait_ms 246, render/thread/ms 81.42, render/busy/ms 80.02, main/busy/ms 76.81, stage/main/post_update 54.69, pipelines/driver_ms 40.06, stage/render/prepare_resources 36.13, stage/render/prepare_batch_phases 12.94, stage/render/render 11.87, render/graph/ms 9.97 |
| warm-up | 4.45 | 41.0 | pipelines/compile_ms 1899, pipelines/front_ms 1817, pipelines/wait_ms 320, render/thread/ms 107, render/busy/ms 105, main/busy/ms 101, pipelines/driver_ms 81.73, stage/main/post_update 73.20, stage/render/prepare_resources 62.39, bigspace/total/ms 22.32, stage/render/prepare_batch_phases 12.15, stage/render/prepare_assets 11.07 |
| warm-up | 4.52 | 70.4 | pipelines/compile_ms 1853, pipelines/front_ms 1785, pipelines/wait_ms 213, render/thread/ms 76.38, render/busy/ms 75.01, pipelines/driver_ms 68.26, main/busy/ms 51.85, stage/render/prepare_assets 32.51, stage/render/prepare_resources 27.50, stage/main/post_update 26.61, bigspace/total/ms 17.10, stage/main/update 12.20 |
| warm-up | 4.56 | 35.8 | pipelines/compile_ms 1600, pipelines/front_ms 1551, pipelines/wait_ms 364, render/thread/ms 70.95, render/busy/ms 69.38, main/busy/ms 49.32, pipelines/driver_ms 48.46, stage/render/prepare_assets 32.79, stage/main/update 24.37, stage/render/prepare_flush 14.51, stage/main/post_update 9.46, stage/render/render 9.20 |
| warm-up | 4.59 | 33.1 | pipelines/compile_ms 935, pipelines/front_ms 901, pipelines/wait_ms 369, render/thread/ms 59.49, render/busy/ms 57.31, main/busy/ms 46.13, pipelines/driver_ms 33.72, stage/main/update 23.00, stage/render/render 14.80, stage/render/prepare_flush 14.44, render/graph/ms 12.10, stage/render/prepare_assets 11.95 |
| warm-up | 4.63 | 36.6 | pipelines/compile_ms 546, pipelines/front_ms 515, pipelines/wait_ms 250, render/thread/ms 60.36, render/busy/ms 57.50, stage/render/render 40.81, render/graph/ms 37.48, stage/render/graph_submit 34.88, encode/submit 33.41, pipelines/driver_ms 30.92, main/busy/ms 28.93, stage/main/post_update 12.86 |
| warm-up | 4.69 | 46.8 | pipelines/compile_ms 201, pipelines/front_ms 196, main/busy/ms 55.26, stage/main/update 40.94, render/thread/ms 37.86, render/busy/ms 36.44, stream/drive/ms 27.59, stage/render/prepare_assets 18.02, stage/render/render 9.21, stage/main/post_update 8.31, encode//sum 8.08, render/graph/ms 7.11 |
| warm-up | 4.72 | 29.4 | pipelines/compile_ms 1341, pipelines/front_ms 1309, render/thread/ms 46.24, render/busy/ms 44.94, main/busy/ms 43.75, stage/main/update 32.66, pipelines/driver_ms 31.55, stream/drive/ms 27.68, stage/render/prepare_assets 27.13, stage/render/render 8.47, encode//sum 7.14, render/graph/ms 6.68 |
| warm-up | 4.75 | 30.5 | pipelines/compile_ms 219, pipelines/front_ms 212, main/busy/ms 27.34, render/thread/ms 23.06, render/busy/ms 21.81, stage/main/update 18.65, stream/drive/ms 14.24, stage/render/render 12.80, render/graph/ms 11.33, pipelines/wait_ms 9.81, stage/render/graph_submit 9.07, pipelines/driver_ms 6.60 |
| warm-up | 4.83 | 38.9 | main/busy/ms 43.45, stage/main/update 31.67, stream/drive/ms 26.86, render/thread/ms 25.06, render/busy/ms 23.68, stage/render/render 9.39, stage/render/prepare_assets 8.35, stage/main/post_update 7.90, render/graph/ms 7.72, encode//sum 5.98, stage/render/graph_submit 5.60, encode/submit 4.25 |
| warm-up | 4.86 | 25.3 | render/thread/ms 30.65, render/busy/ms 29.41, main/busy/ms 22.97, stage/render/prepare_assets 13.55, stage/main/update 9.26, stage/render/render 8.52, stage/main/post_update 8.32, render/graph/ms 7.00, encode//sum 6.07, stage/render/graph_submit 4.82, stream/drive/ms 4.45, encode/submit 3.41 |
| warm-up | 5.22 | 51.5 | main/busy/ms 57.80, render/thread/ms 43.75, render/busy/ms 42.04, stage/main/update 42.04, stage/render/render 13.84, stream/drive/ms 13.15, stage/render/prepare_assets 12.10, render/graph/ms 11.62, stage/main/post_update 9.56, stage/render/graph_submit 9.42, encode//sum 9.02, encode/submit 7.19 |
| warm-up | 5.24 | 25.1 | render/thread/ms 47.50, render/busy/ms 46.39, main/busy/ms 41.98, stage/main/update 30.57, stage/render/prepare_assets 19.44, stream/drive/ms 12.65, pipelines/compile_ms 9.01, pipelines/front_ms 8.47, stage/render/specialize 7.82, stage/render/render 7.65, render/graph/ms 6.17, encode//sum 6.08 |
| warm-up | 5.28 | 31.2 | main/busy/ms 64.20, stage/main/update 54.23, stream/drive/ms 49.39, pipelines/compile_ms 26.97, pipelines/front_ms 25.90, render/thread/ms 19.12, render/busy/ms 18.04, pipelines/wait_ms 9.31, stage/render/render 8.10, render/graph/ms 6.63, stage/main/post_update 6.17, encode//sum 6.00 |
| warm-up | 5.32 | 39.0 | pipelines/compile_ms 153, pipelines/front_ms 150, main/busy/ms 45.10, stage/main/update 33.51, stream/drive/ms 28.43, render/thread/ms 21.26, render/busy/ms 20.12, stage/render/prepare_assets 7.39, stage/render/render 7.28, stage/main/post_update 7.25, encode//sum 6.08, render/graph/ms 5.65 |
| warm-up | 6.07 | 27.6 | render/thread/ms 30.54, render/busy/ms 29.53, main/busy/ms 18.89, stage/render/prepare_assets 16.00, stage/main/post_update 8.83, stage/render/render 7.24, render/graph/ms 5.95, encode//sum 5.81, stage/main/update 5.59, stage/render/graph_submit 4.12, encode/submit 2.38, encode/finish_wall 1.76 |
| warm-up | 6.26 | 27.7 | render/thread/ms 28.32, main/busy/ms 27.73, render/busy/ms 27.09, stage/main/update 11.79, stage/main/post_update 11.12, stage/render/render 9.51, stage/render/prepare_assets 8.08, render/graph/ms 7.91, encode//sum 6.15, stage/render/graph_submit 5.88, encode/submit 4.02, stream/drive/ms 3.07 |
| warm-up | 6.36 | 25.8 | render/thread/ms 25.65, pipelines/compile_ms 24.62, render/busy/ms 24.42, pipelines/front_ms 23.21, main/busy/ms 19.81, stage/main/post_update 10.54, stage/render/render 9.56, stage/render/prepare_assets 9.34, render/graph/ms 7.79, encode//sum 6.88, stage/main/update 6.50, stage/render/graph_submit 4.66 |
| warm-up | 7.27 | 40.3 | render/thread/ms 46.63, render/busy/ms 45.41, main/busy/ms 16.14, stage/render/specialize 13.70, stage/render/render 10.31, stage/main/post_update 9.39, render/graph/ms 8.98, stage/render/queue 8.75, stage/render/graph_submit 7.12, stage/render/prepare_meshes 6.09, encode//sum 5.69, encode/submit 4.43 |
| warm-up | 7.51 | 31.7 | render/thread/ms 37.39, render/busy/ms 35.78, main/busy/ms 28.18, stage/render/prepare_assets 15.50, stage/main/post_update 14.56, stage/render/render 14.35, render/graph/ms 12.15, stage/render/graph_submit 9.03, stage/main/update 8.16, encode//sum 7.70, encode/submit 6.47, stage/render/camera_driver 2.75 |
| measure | 0.48 | 41.4 | render/thread/ms 48.64, render/busy/ms 45.71, stage/render/specialize 16.31, main/busy/ms 14.47, stage/render/queue 10.14, stage/render/render 9.27, stage/main/post_update 8.85, render/graph/ms 7.91, stage/render/graph_submit 5.92, encode//sum 5.87, stage/render/prepare_meshes 5.10, encode/submit 3.79 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=noprecomb-2 --bench-baseline none --off precombine
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 0%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 86%) · 2616 MHz (min 2610) · memory 8101 MHz · 99.1 W · 80 °C (max 82) · VRAM 6189 MiB · throttled 100% of samples

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
