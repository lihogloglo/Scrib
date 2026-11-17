# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: precombine · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1571 frames over 15.0 s · **median 9.47 ms (106 fps)** · mean 9.55 · p95 10.90 · p99 11.48 (1% low 87 fps) · worst 42.05 ms · 4 spikes over 1.5× median · 16.0 stutters/min (work 12.0/min)
- warm-up: 316 frames over 8.1 s, worst 2453.0 ms
- hitches (> 25.0 ms): **1** measured, 41 in warm-up
- timed GPU passes: 8.34 ms of a 9.55 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.47 ms, limit 11 ms
- **FAIL** worst frame 42.05 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.764 | 2.794 | 2.921 | 0.99 | 0.062 |
| dlss_super_resolution | 1.998 | 2.020 | 2.042 | 0.99 | 0.184 |
| early prepass | 0.591 | 0.598 | 0.611 | 0.99 | 0.096 |
| atmosphere_luts | 0.321 | 0.324 | 0.371 | 0.99 | 0.003 |
| bloom | 0.310 | 0.314 | 0.330 | 0.99 | 0.061 |
| main_transmissive_pass_3d | 0.289 | 0.293 | 0.331 | 0.99 | 0.009 |
| water_velocity | 0.236 | 0.239 | 0.265 | 0.99 | 0.002 |
| water_sspr | 0.214 | 0.217 | 0.215 | 0.99 | 0.002 |
| ssao | 0.189 | 0.191 | 0.198 | 0.99 | 0.007 |
| clouds_trace | 0.148 | 0.150 | 0.172 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.143 | 0.145 | 0.152 | 0.99 | 0.001 |
| water_fft | 0.129 | 0.131 | 0.134 | 0.99 | 0.003 |
| uniform_allocation | 0.119 | 0.120 | 0.162 | 0.99 | 0.021 |
| upscaling | 0.113 | 0.114 | 0.120 | 0.99 | 0.004 |
| clouds_composite | 0.111 | 0.112 | 0.114 | 0.99 | 0.002 |
| bin_unpacking | 0.090 | 0.091 | 0.123 | 0.99 | 0.015 |
| tonemapping | 0.085 | 0.086 | 0.092 | 0.99 | 0.003 |
| clustering | 0.065 | 0.066 | 0.078 | 0.99 | 0.020 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.414 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.051 | 0.052 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.039 | 0.039 | 0.052 | 0.99 | 0.005 |
| clouds_shadow | 0.031 | 0.032 | 0.035 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.220 | 0.229 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 0.99 | 0.081 |
| water_snapshot | 0.024 | 0.025 | 0.026 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| surface_exposure_far | 0.012 | 0.043 | 0.045 | 0.29 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| surface_exposure | 0.011 | 0.042 | 0.043 | 0.27 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.014 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.007 | 0.007 | 0.020 | 0.99 | 0.004 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.545 | 9.883 | 40.07 | 13424.6 |
| main/busy/ms | 7.694 | 8.529 | 17.73 | 12087.9 |
| render/busy/ms | 7.151 | 8.223 | 39.47 | 11235.0 |
| stage/render/render | 4.859 | 5.786 | 16.79 | 7633.9 |
| stage/main/post_update | 4.745 | 5.444 | 15.08 | 7455.1 |
| render/graph/ms | 4.268 | 5.153 | 16.11 | 6704.6 |
| encode//sum | 3.732 | 5.062 | 56.44 | 5863.6 |
| stage/render/graph_submit | 3.289 | 4.171 | 15.17 | 5167.6 |
| encode/submit | 2.145 | 2.726 | 4.66 | 3369.8 |
| stage/main/update | 1.831 | 2.191 | 4.67 | 2876.6 |
| pipelines/compile_ms | 1.235 | 0.000 | 132.43 | 1939.8 |
| pipelines/front_ms | 1.201 | 0.000 | 129.39 | 1886.4 |
| encode/finish_wall | 1.175 | 1.540 | 11.70 | 1845.3 |
| stage/render/prepare_views | 1.065 | 2.547 | 4.08 | 1673.2 |
| render/acquire/ms | 1.020 | 2.497 | 4.03 | 1603.0 |
| stage/render/camera_driver | 0.925 | 1.063 | 1.46 | 1453.5 |
| encode/pass:early prepass/sum | 0.794 | 1.053 | 9.71 | 1248.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.759 | 1.025 | 1.69 | 1193.0 |
| stage/render/prepare_write_phase_buffers | 0.523 | 0.739 | 1.07 | 821.7 |
| stage/render/prepare_assets | 0.453 | 0.581 | 1.80 | 711.0 |
| bigspace/total/ms | 0.432 | 1.012 | 1.73 | 678.1 |
| stage/render/prepare_resources | 0.379 | 0.433 | 0.77 | 595.7 |
| render/present/ms | 0.373 | 0.461 | 2.21 | 586.7 |
| stage/main/pre_update | 0.347 | 0.471 | 0.89 | 545.6 |
| encode/pass:shadow_directional_light/sum | 0.327 | 1.127 | 9.96 | 513.3 |
| stage/main/first | 0.287 | 0.352 | 1.41 | 451.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.258 | 0.400 | 0.55 | 406.0 |
| terrain/update/ms | 0.232 | 0.297 | 0.39 | 364.5 |
| stage/render/prepare_meshes | 0.210 | 0.264 | 4.66 | 329.7 |
| stage/render/prepare_bind_groups | 0.207 | 0.283 | 0.37 | 324.8 |
| terrain/select/ms | 0.202 | 0.259 | 0.32 | 317.4 |
| stream/drive/ms | 0.196 | 0.268 | 0.70 | 307.2 |
| stage/main/fixed_main_loop | 0.181 | 0.364 | 0.97 | 285.0 |
| stage/render/prepare_batch_phases | 0.178 | 0.236 | 0.43 | 278.9 |
| stage/render/create_views | 0.172 | 0.227 | 0.81 | 270.2 |
| far/select/ms | 0.169 | 0.231 | 0.68 | 266.3 |
| stage/render/extract_commands | 0.169 | 0.200 | 0.57 | 265.5 |
| stream/tick/ms | 0.164 | 0.232 | 0.63 | 257.1 |
| bigspace/high_precision/ms | 0.155 | 0.645 | 1.49 | 243.0 |
| stage/render/prepare_flush | 0.148 | 0.308 | 0.68 | 232.4 |
| controller/feed/ms | 0.137 | 0.179 | 0.36 | 215.8 |
| controller/move/ms | 0.128 | 0.209 | 0.56 | 201.0 |
| stage/main/last | 0.122 | 0.153 | 0.52 | 191.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.116 | 0.175 | 0.40 | 181.7 |
| stage/render/queue | 0.115 | 0.140 | 10.80 | 180.5 |
| stage/render/specialize | 0.097 | 0.129 | 15.82 | 152.8 |
| bigspace/low_precision/ms | 0.096 | 0.204 | 1.33 | 151.0 |
| anim/evaluate/ms | 0.087 | 0.211 | 1.28 | 137.0 |
| anim/evaluate_cpu/ms | 0.060 | 0.074 | 0.19 | 95.0 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.111 | 0.27 | 90.8 |
| encode/resolve_diagnostics/sum | 0.051 | 0.075 | 0.28 | 80.2 |
| pipelines/driver_ms | 0.034 | 0.000 | 4.32 | 53.5 |
| physics/step/ms | 0.027 | 0.055 | 0.40 | 42.1 |
| weather/map/ms | 0.026 | 0.034 | 0.46 | 40.1 |
| encode/camera_driver_span_begin/sum | 0.025 | 0.039 | 0.24 | 39.5 |
| stage/render/cleanup | 0.024 | 0.036 | 0.42 | 37.8 |
| encode/camera_driver_span_end/sum | 0.022 | 0.035 | 0.08 | 33.9 |
| stage/render/phase_sort | 0.017 | 0.032 | 0.08 | 26.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.07 | 17.4 |
| script/update/ms | 0.007 | 0.010 | 0.15 | 11.1 |
| controller/avoid/ms | 0.006 | 0.007 | 0.48 | 9.9 |
| script/input/ms | 0.005 | 0.007 | 0.09 | 8.6 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 7.0 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 6.8 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.07 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.09 | 1.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.13 | 0.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.12 | 0.1 |
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
| encode//max | ms | 0.50 | 11.48 | 0.43 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.24 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.08 | 0.02 |
| encode/finish_critical | ms | 0.87 | 11.48 | 0.67 |
| encode/pass:early prepass/max | ms | 0.79 | 9.71 | 0.67 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 1.69 | 0.59 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.40 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 0.55 | 0.18 |
| encode/pass:shadow_directional_light/max | ms | 0.10 | 9.96 | 0.06 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.28 | 0.03 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.08 | 39.47 | 9.16 |
| gpu/timed/ms | ms | 9.08 | 10.58 | 9.16 |
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
| precache/throttled | count | 0 | 14 | 0 |
| preprocess/bind_groups | count | 0 | 76 | 0 |
| preprocess/bind_groups_created | count | 0 | 76 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 28.37 | 41.35 | 25.48 |
| process/mem_usage | GiB | 4.69 | 4.76 | 4.76 |
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
| skin/blocks_changed/count | count | 5 | 6 | 3 |
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
| stream/latency/s | s | 0.53 | 0.53 | 0.53 |
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
| system/cpu_usage | % | 33.85 | 42.57 | 41.67 |
| system/mem_usage | % | 54.24 | 54.44 | 54.44 |
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
| anim/events/count | count | 0.10 | 3 | 155 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.14 | 11 | 4939 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 26.54 | 29 | 41.7 k |
| encode/bind_walk/init_textures | count | 5226 | 12.1 k | 8.21 M |
| encode/bind_walk/init_textures_skipped | count | 38.5 k | 48.7 k | 60.51 M |
| encode/bind_walk/inits | count | 336 | 429 | 527.6 k |
| encode/bind_walk/inits_skipped | count | 70.00 | 79 | 110.0 k |
| encode/bind_walk/merge_textures | count | 5162 | 12.0 k | 8.11 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.41 M |
| encode/bind_walk/merges | count | 78.26 | 99 | 123.0 k |
| encode/bind_walk/merges_skipped | count | 52.25 | 56 | 82.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1571 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1571 |
| encode/pass:early prepass/count | count | 1 | 1 | 1571 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1571 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1571 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1571 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1571 |
| encode/queue/command_buffers | count | 36.54 | 39 | 57.4 k |
| encode/queue/submits | count | 2 | 2 | 3142 |
| encode/queue/write_buffer/bytes | bytes | 1.49 M | 5.49 M | 2.34 G |
| encode/queue/write_buffer/calls | count | 212 | 283 | 332.8 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 51.28 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 14.1 k |
| encode/queue/write_texture/bytes | bytes | 11.4 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1571 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.57 | 3 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 28 |
| pipelines/created | count | 0.02 | 1 | 28 |
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

## Hitches (> 25.0 ms; slowest 42, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.45 | 2453.0 | main/busy/ms 2425, stage/main/post_update 2318, stage/main/update 97.37, surface/advance_cpu_ms 65.61, stage/main/pre_update 5.47, stage/main/last 1.12, surface/cell_upload_ms 0.72, stream/drive/ms 0.60, stage/main/first 0.59, stage/main/fixed_main_loop 0.35, weather/map/ms 0.20, stream/tick/ms 0.13 |
| warm-up | 3.14 | 686.3 | main/busy/ms 2559, stage/main/post_update 2342, render/thread/ms 656, render/busy/ms 524, stage/render/prepare_assets 479, stage/main/update 203, stage/render/prepare_views 128, render/acquire/ms 128, surface/advance_cpu_ms 65.61, stage/render/create_views 27.32, stage/render/render 11.24, stage/main/pre_update 7.25 |
| warm-up | 3.24 | 100.2 | render/thread/ms 739, render/busy/ms 563, stage/render/prepare_assets 505, main/busy/ms 153, stage/render/prepare_views 129, render/acquire/ms 129, stage/main/update 110, stage/render/render 60.08, render/present/ms 47.38, pipelines/compile_ms 44.81, pipelines/front_ms 41.31, stage/main/post_update 33.55 |
| warm-up | 3.28 | 43.7 | pipelines/compile_ms 400, pipelines/front_ms 369, pipelines/wait_ms 270, render/thread/ms 126, stage/render/render 84.98, render/busy/ms 78.56, main/busy/ms 60.41, render/present/ms 46.60, stage/main/post_update 46.39, render/graph/ms 37.34, pipelines/driver_ms 31.08, stage/render/prepare_assets 26.33 |
| warm-up | 3.39 | 109.7 | pipelines/wait_ms 3334, pipelines/compile_ms 1243, pipelines/front_ms 1134, render/thread/ms 141, render/busy/ms 138, pipelines/driver_ms 108, main/busy/ms 80.14, stage/render/render 53.02, render/graph/ms 49.02, stage/main/post_update 46.60, stage/render/graph_submit 23.52, stage/render/prepare_bind_groups 20.51 |
| warm-up | 3.46 | 71.0 | pipelines/wait_ms 5029, pipelines/compile_ms 1408, pipelines/front_ms 1305, render/thread/ms 169, render/busy/ms 166, main/busy/ms 115, pipelines/driver_ms 103, stage/main/update 61.87, stage/render/prepare_resources 31.98, stage/main/post_update 29.30, stage/render/render 28.37, stage/render/prepare_bind_groups 27.37 |
| warm-up | 3.55 | 82.3 | pipelines/wait_ms 2109, pipelines/compile_ms 1747, pipelines/front_ms 1676, render/thread/ms 139, render/busy/ms 136, main/busy/ms 91.61, pipelines/driver_ms 71.51, stage/render/prepare_assets 62.12, stage/main/update 60.01, stage/render/phase_sort 26.22, stage/main/post_update 25.39, stage/render/render 17.42 |
| warm-up | 3.58 | 30.2 | pipelines/compile_ms 1357, pipelines/front_ms 1300, pipelines/wait_ms 170, render/thread/ms 96.94, render/busy/ms 95.31, stage/render/prepare_assets 72.44, pipelines/driver_ms 57.07, main/busy/ms 44.16, stage/main/post_update 15.66, stage/main/pre_update 14.24, stage/render/render 11.95, terrain/update/ms 10.44 |
| warm-up | 3.60 | 27.5 | pipelines/compile_ms 1134, pipelines/front_ms 1082, pipelines/wait_ms 138, pipelines/driver_ms 51.58, render/thread/ms 50.89, render/busy/ms 49.43, main/busy/ms 40.55, stage/render/prepare_assets 28.50, stage/main/post_update 14.68, stage/main/pre_update 13.77, stage/render/render 10.58, terrain/update/ms 9.58 |
| warm-up | 3.71 | 32.0 | pipelines/compile_ms 213, pipelines/front_ms 199, main/busy/ms 39.40, stage/main/update 31.22, stream/drive/ms 28.33, render/thread/ms 23.44, render/busy/ms 22.10, pipelines/driver_ms 13.72, stage/render/render 9.79, render/graph/ms 8.24, stage/render/prepare_assets 7.95, stage/render/graph_submit 5.95 |
| warm-up | 3.77 | 25.3 | pipelines/compile_ms 573, pipelines/front_ms 550, pipelines/wait_ms 37.65, main/busy/ms 26.48, pipelines/driver_ms 22.63, render/thread/ms 20.67, render/busy/ms 18.81, stage/main/update 18.65, stream/drive/ms 14.83, stage/render/render 8.55, stage/render/prepare_assets 7.58, encode//sum 6.47 |
| warm-up | 3.81 | 30.7 | pipelines/compile_ms 175, pipelines/front_ms 169, main/busy/ms 30.98, render/thread/ms 26.66, render/busy/ms 24.61, stage/main/update 18.98, stream/drive/ms 14.72, stage/render/render 12.70, render/graph/ms 10.29, stage/render/prepare_assets 8.54, stage/main/post_update 8.02, stage/render/graph_submit 7.54 |
| warm-up | 3.84 | 26.5 | pipelines/compile_ms 174, pipelines/front_ms 166, render/thread/ms 34.77, render/busy/ms 33.37, main/busy/ms 22.99, stage/render/prepare_assets 15.91, stage/render/render 11.59, render/graph/ms 9.95, stage/main/post_update 9.37, stage/main/update 8.30, pipelines/driver_ms 8.12, encode//sum 7.12 |
| warm-up | 3.88 | 41.7 | pipelines/compile_ms 213, pipelines/front_ms 203, render/thread/ms 54.19, main/busy/ms 49.20, render/busy/ms 43.23, stage/main/post_update 28.87, stage/render/render 27.70, render/graph/ms 16.43, stage/render/prepare_assets 15.43, stage/main/update 12.26, render/present/ms 10.42, pipelines/driver_ms 9.95 |
| warm-up | 3.90 | 25.1 | pipelines/compile_ms 335, pipelines/front_ms 324, main/busy/ms 60.00, render/thread/ms 51.40, render/busy/ms 40.81, stage/main/post_update 27.13, stage/render/render 26.08, stage/main/update 24.98, render/graph/ms 14.97, stage/render/prepare_assets 14.83, pipelines/driver_ms 11.20, render/present/ms 10.23 |
| warm-up | 3.95 | 50.9 | pipelines/compile_ms 496, pipelines/front_ms 483, render/thread/ms 44.64, render/busy/ms 43.50, main/busy/ms 34.43, stage/render/prepare_assets 29.87, stage/main/update 20.93, pipelines/driver_ms 13.04, stage/render/render 8.26, stage/main/post_update 6.78, render/graph/ms 6.68, encode//sum 4.80 |
| warm-up | 4.01 | 37.9 | pipelines/compile_ms 167, pipelines/front_ms 160, main/busy/ms 37.00, render/thread/ms 35.35, render/busy/ms 33.89, pipelines/wait_ms 23.57, stage/main/update 16.59, stage/main/post_update 15.97, stage/render/render 11.01, render/graph/ms 8.93, pipelines/driver_ms 7.43, stage/render/prepare_assets 7.33 |
| warm-up | 4.07 | 41.9 | pipelines/compile_ms 838, pipelines/front_ms 810, pipelines/wait_ms 83.21, render/thread/ms 51.25, render/busy/ms 49.95, pipelines/driver_ms 28.25, main/busy/ms 26.09, stage/render/prepare_write_phase_buffers 16.79, stage/main/update 12.36, stage/render/prepare_resources 10.65, stage/render/render 9.34, stage/main/post_update 8.41 |
| warm-up | 4.12 | 32.3 | render/thread/ms 45.21, render/busy/ms 43.50, main/busy/ms 31.13, stage/render/render 17.21, render/graph/ms 14.59, pipelines/compile_ms 14.39, pipelines/front_ms 13.61, stage/main/update 13.42, stage/main/post_update 13.02, stage/render/prepare_resources 11.91, stage/render/graph_submit 10.97, stream/drive/ms 9.79 |
| warm-up | 4.14 | 26.7 | pipelines/compile_ms 402, pipelines/front_ms 390, render/thread/ms 45.21, render/busy/ms 43.24, main/busy/ms 31.54, stage/main/update 14.72, stage/render/prepare_assets 14.67, stage/main/post_update 12.70, pipelines/driver_ms 12.22, stage/render/render 11.14, stream/drive/ms 9.91, stage/render/prepare_resources 9.71 |
| warm-up | 4.20 | 28.1 | pipelines/compile_ms 957, pipelines/front_ms 926, render/thread/ms 39.52, render/busy/ms 37.43, main/busy/ms 36.44, pipelines/driver_ms 31.21, pipelines/wait_ms 20.61, stage/main/update 17.82, stage/main/post_update 13.86, stage/render/render 12.21, render/graph/ms 9.83, stage/render/graph_submit 7.39 |
| warm-up | 4.24 | 34.6 | pipelines/compile_ms 469, pipelines/front_ms 453, render/thread/ms 49.46, render/busy/ms 47.51, main/busy/ms 35.12, stage/main/update 19.23, pipelines/driver_ms 16.28, stage/render/prepare_assets 14.69, stage/main/post_update 11.73, stage/render/render 10.40, render/graph/ms 8.23, stage/render/specialize 7.29 |
| warm-up | 4.27 | 38.3 | pipelines/compile_ms 772, pipelines/front_ms 743, pipelines/wait_ms 89.78, render/thread/ms 59.34, render/busy/ms 51.54, main/busy/ms 42.82, pipelines/driver_ms 28.57, stage/main/update 18.16, stage/render/prepare_assets 16.78, stage/main/post_update 16.06, stage/render/prepare_resources 12.14, stage/render/render 9.73 |
| warm-up | 4.33 | 34.6 | render/thread/ms 48.26, render/busy/ms 46.57, main/busy/ms 46.37, stage/main/update 26.64, stage/render/prepare_flush 19.99, stage/main/post_update 12.67, stage/render/render 10.27, render/graph/ms 8.41, encode//sum 6.88, stage/render/graph_submit 6.01, stage/render/prepare_assets 5.81, encode/submit 4.28 |
| warm-up | 4.36 | 32.9 | pipelines/compile_ms 603, pipelines/front_ms 583, render/thread/ms 53.19, render/busy/ms 51.56, main/busy/ms 50.55, stage/main/update 27.41, pipelines/driver_ms 20.09, stage/render/prepare_flush 20.02, stage/main/post_update 16.99, stage/render/prepare_assets 12.81, stage/render/render 10.13, render/graph/ms 8.49 |
| warm-up | 4.39 | 27.7 | pipelines/compile_ms 803, pipelines/front_ms 777, render/thread/ms 46.38, render/busy/ms 44.99, main/busy/ms 38.14, pipelines/driver_ms 25.95, stage/main/post_update 25.39, stage/render/prepare_assets 15.16, stage/render/render 10.65, render/graph/ms 9.07, stage/main/update 7.46, stage/render/prepare_resources 6.80 |
| warm-up | 4.53 | 50.9 | pipelines/compile_ms 959, pipelines/front_ms 932, main/busy/ms 67.71, pipelines/wait_ms 63.99, stage/main/update 47.86, render/thread/ms 44.32, render/busy/ms 42.38, stream/drive/ms 29.88, pipelines/driver_ms 26.77, stage/render/render 15.76, stage/render/prepare_assets 13.90, render/graph/ms 13.51 |
| warm-up | 4.56 | 35.8 | pipelines/compile_ms 983, pipelines/front_ms 953, pipelines/wait_ms 52.46, main/busy/ms 49.22, render/thread/ms 45.32, render/busy/ms 43.87, stage/main/update 36.08, stream/drive/ms 29.54, pipelines/driver_ms 29.32, stage/render/prepare_assets 22.38, stage/render/render 8.84, stage/main/post_update 7.52 |
| warm-up | 4.61 | 29.8 | pipelines/compile_ms 864, pipelines/front_ms 841, main/busy/ms 37.68, render/thread/ms 29.57, render/busy/ms 27.78, pipelines/driver_ms 22.69, stage/main/update 20.32, stream/drive/ms 16.09, stage/render/render 14.00, render/graph/ms 11.83, stage/render/graph_submit 9.51, stage/main/post_update 5.88 |
| warm-up | 4.67 | 38.1 | pipelines/compile_ms 777, pipelines/front_ms 754, main/busy/ms 52.94, render/thread/ms 36.82, render/busy/ms 35.32, stage/main/update 27.93, pipelines/driver_ms 23.44, stream/drive/ms 19.42, stage/main/post_update 15.34, stage/render/render 11.68, render/graph/ms 9.88, bigspace/total/ms 7.97 |
| warm-up | 4.72 | 25.3 | render/thread/ms 34.58, render/busy/ms 32.88, main/busy/ms 29.77, stage/main/update 16.57, stage/render/render 13.28, stream/drive/ms 11.51, render/graph/ms 11.04, stage/render/prepare_assets 10.80, stage/main/post_update 8.37, stage/render/graph_submit 7.66, encode//sum 7.49, encode/submit 5.75 |
| warm-up | 4.76 | 41.6 | pipelines/compile_ms 192, pipelines/front_ms 186, render/thread/ms 52.31, render/busy/ms 50.93, main/busy/ms 36.88, stage/main/update 14.96, stage/render/render 13.29, stage/render/prepare_resources 13.25, stage/render/prepare_assets 12.40, render/graph/ms 11.51, stage/main/pre_update 11.03, stage/render/graph_submit 8.63 |
| warm-up | 5.14 | 57.4 | pipelines/compile_ms 150, pipelines/front_ms 148, main/busy/ms 60.71, stage/main/update 45.44, render/thread/ms 42.58, render/busy/ms 40.94, stage/render/render 12.56, stream/drive/ms 12.20, stage/render/prepare_assets 11.13, render/graph/ms 10.56, stage/main/post_update 8.77, stage/render/graph_submit 8.14 |
| warm-up | 5.17 | 28.7 | pipelines/compile_ms 202, pipelines/front_ms 196, render/thread/ms 52.36, render/busy/ms 51.19, main/busy/ms 46.96, stage/main/update 34.46, stage/render/prepare_assets 20.30, stream/drive/ms 12.00, stage/render/render 10.17, render/graph/ms 8.57, stage/render/specialize 8.31, stage/main/post_update 7.80 |
| warm-up | 5.21 | 31.4 | main/busy/ms 67.01, stage/main/update 56.14, stream/drive/ms 51.54, pipelines/compile_ms 32.36, pipelines/front_ms 30.68, pipelines/wait_ms 29.63, render/thread/ms 21.55, render/busy/ms 20.33, stage/render/render 9.79, render/graph/ms 8.14, encode//sum 7.88, stage/main/post_update 6.74 |
| warm-up | 5.25 | 41.1 | pipelines/compile_ms 115, pipelines/front_ms 110, main/busy/ms 45.52, stage/main/update 34.12, stream/drive/ms 28.77, render/thread/ms 22.44, render/busy/ms 21.30, stage/render/prepare_assets 8.76, stage/render/render 7.69, stage/main/post_update 7.25, render/graph/ms 5.96, encode//sum 5.93 |
| warm-up | 6.02 | 28.2 | render/thread/ms 29.53, render/busy/ms 28.48, main/busy/ms 18.36, stage/render/prepare_assets 15.55, pipelines/compile_ms 13.08, pipelines/front_ms 11.93, stage/main/post_update 9.65, stage/render/render 7.14, render/graph/ms 5.76, encode//sum 5.64, stage/main/update 4.66, stage/render/graph_submit 3.93 |
| warm-up | 6.16 | 25.3 | render/thread/ms 28.61, render/busy/ms 27.34, main/busy/ms 27.01, pipelines/compile_ms 12.09, stage/main/update 11.75, pipelines/front_ms 11.43, stage/render/render 10.53, stage/main/post_update 10.47, render/graph/ms 8.94, stage/render/prepare_assets 7.16, stage/render/graph_submit 7.00, encode//sum 6.86 |
| warm-up | 7.03 | 25.3 | pipelines/compile_ms 135, pipelines/front_ms 132, main/busy/ms 27.77, render/thread/ms 21.79, render/busy/ms 19.31, stage/main/update 16.69, stream/drive/ms 12.94, stage/main/post_update 8.59, stage/render/render 8.13, render/graph/ms 6.87, stage/render/prepare_assets 6.17, encode//sum 5.33 |
| warm-up | 7.17 | 39.9 | render/thread/ms 47.03, render/busy/ms 44.10, main/busy/ms 18.40, stage/render/specialize 13.98, stage/render/render 10.46, render/graph/ms 9.21, stage/main/post_update 9.05, stage/render/queue 8.75, stage/render/graph_submit 7.27, encode//sum 6.68, stage/render/prepare_meshes 5.96, stage/main/update 5.94 |
| warm-up | 7.45 | 32.3 | render/thread/ms 34.16, render/busy/ms 32.65, main/busy/ms 28.68, stage/main/post_update 14.07, stage/render/render 13.71, stage/render/prepare_assets 13.46, render/graph/ms 11.83, stage/main/update 9.80, stage/render/graph_submit 8.68, encode//sum 6.86, encode/submit 6.08, stream/drive/ms 4.66 |
| measure | 0.47 | 42.0 | render/thread/ms 48.20, render/busy/ms 45.61, stage/render/specialize 15.94, main/busy/ms 15.65, stage/render/queue 10.92, stage/render/render 9.35, stage/main/post_update 8.81, render/graph/ms 8.02, stage/render/graph_submit 6.09, encode//sum 5.68, stage/render/prepare_meshes 4.83, stage/main/update 4.51 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=noprecomb-3 --bench-baseline none --off precombine
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 11%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 81%) · 2625 MHz (min 2625) · memory 8101 MHz · 99.1 W · 80 °C (max 82) · VRAM 6189 MiB · throttled 100% of samples

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
