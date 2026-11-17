# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); mw.matctrl.gate=on (command line: --set); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1526 frames over 15.0 s · **median 9.87 ms (101 fps)** · mean 9.83 · p95 11.19 · p99 11.58 (1% low 86 fps) · worst 27.45 ms · 2 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 278 frames over 8.6 s, worst 2551.6 ms
- hitches (> 25.0 ms): **1** measured, 61 in warm-up
- timed GPU passes: 8.59 ms of a 9.83 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.87 ms, limit 11 ms
- pass worst frame 27.45 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.928 | 2.934 | 2.971 | 1.00 | 0.062 |
| dlss_super_resolution | 2.017 | 2.021 | 2.044 | 1.00 | 0.183 |
| early prepass | 0.619 | 0.621 | 0.631 | 1.00 | 0.094 |
| atmosphere_luts | 0.324 | 0.324 | 0.341 | 1.00 | 0.003 |
| bloom | 0.314 | 0.315 | 0.331 | 1.00 | 0.060 |
| main_transmissive_pass_3d | 0.311 | 0.312 | 0.334 | 1.00 | 0.008 |
| water_velocity | 0.239 | 0.240 | 0.264 | 1.00 | 0.001 |
| water_sspr | 0.207 | 0.207 | 0.210 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.148 | 0.148 | 0.170 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.146 | 0.152 | 1.00 | 0.001 |
| water_fft | 0.133 | 0.133 | 0.137 | 1.00 | 0.003 |
| uniform_allocation | 0.120 | 0.120 | 0.163 | 1.00 | 0.021 |
| upscaling | 0.113 | 0.113 | 0.119 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.092 | 0.092 | 0.122 | 1.00 | 0.015 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.412 | 0.412 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| water_snapshot | 0.034 | 0.034 | 0.036 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 1.00 | 0.004 |
| lightprobe_radiance_map | 0.028 | 0.221 | 0.231 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 1.00 | 0.082 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.047 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 1.00 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.003 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.896 | 10.290 | 26.28 | 13575.1 |
| main/busy/ms | 7.081 | 7.984 | 9.22 | 10805.5 |
| render/busy/ms | 6.729 | 7.680 | 25.68 | 10269.2 |
| stage/render/render | 4.536 | 5.318 | 8.87 | 6922.5 |
| stage/main/post_update | 4.195 | 4.958 | 6.07 | 6401.0 |
| render/graph/ms | 3.963 | 4.702 | 6.23 | 6047.5 |
| encode//sum | 3.751 | 4.917 | 9.39 | 5724.3 |
| stage/render/graph_submit | 2.988 | 3.695 | 5.31 | 4559.4 |
| encode/submit | 1.860 | 2.277 | 4.93 | 2838.2 |
| stage/render/prepare_views | 1.857 | 3.500 | 4.50 | 2833.6 |
| render/acquire/ms | 1.812 | 3.454 | 4.46 | 2764.9 |
| stage/main/update | 1.775 | 2.215 | 3.22 | 2708.5 |
| encode/finish_wall | 1.147 | 1.605 | 2.42 | 1750.2 |
| stage/render/camera_driver | 0.920 | 1.058 | 1.37 | 1404.2 |
| encode/pass:shadow_directional_light/sum | 0.739 | 1.241 | 2.09 | 1127.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.707 | 0.986 | 1.30 | 1079.4 |
| encode/pass:early prepass/sum | 0.706 | 0.969 | 1.45 | 1077.6 |
| stage/render/prepare_write_phase_buffers | 0.538 | 0.736 | 0.97 | 820.9 |
| stage/render/prepare_assets | 0.383 | 0.456 | 1.46 | 584.2 |
| stage/render/prepare_resources | 0.382 | 0.443 | 0.65 | 583.4 |
| bigspace/total/ms | 0.366 | 0.740 | 1.56 | 557.9 |
| render/present/ms | 0.355 | 0.424 | 2.47 | 541.0 |
| stage/main/pre_update | 0.336 | 0.422 | 1.03 | 512.9 |
| stage/main/first | 0.300 | 0.362 | 1.59 | 457.4 |
| terrain/update/ms | 0.233 | 0.294 | 0.87 | 355.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.233 | 0.331 | 0.60 | 355.4 |
| stage/render/prepare_bind_groups | 0.205 | 0.278 | 0.39 | 313.5 |
| stage/render/prepare_meshes | 0.205 | 0.247 | 2.87 | 312.4 |
| terrain/select/ms | 0.203 | 0.255 | 0.68 | 310.2 |
| stage/main/fixed_main_loop | 0.179 | 0.328 | 0.89 | 272.9 |
| stream/drive/ms | 0.178 | 0.230 | 0.56 | 271.9 |
| stage/render/prepare_batch_phases | 0.175 | 0.233 | 0.39 | 266.3 |
| stage/render/extract_commands | 0.169 | 0.199 | 0.29 | 257.9 |
| stage/render/create_views | 0.162 | 0.210 | 0.60 | 246.7 |
| far/select/ms | 0.160 | 0.205 | 0.68 | 243.5 |
| stage/render/prepare_flush | 0.153 | 0.303 | 0.53 | 233.5 |
| stream/tick/ms | 0.151 | 0.195 | 0.52 | 230.9 |
| controller/feed/ms | 0.149 | 0.195 | 0.52 | 227.2 |
| controller/move/ms | 0.137 | 0.243 | 0.70 | 208.4 |
| bigspace/high_precision/ms | 0.126 | 0.290 | 1.35 | 191.7 |
| stage/main/last | 0.119 | 0.161 | 0.76 | 181.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.114 | 0.178 | 0.31 | 174.6 |
| bigspace/low_precision/ms | 0.096 | 0.212 | 1.14 | 146.3 |
| stage/render/queue | 0.095 | 0.123 | 5.86 | 145.5 |
| anim/evaluate/ms | 0.083 | 0.173 | 1.12 | 126.0 |
| stage/render/specialize | 0.067 | 0.083 | 8.41 | 102.7 |
| anim/evaluate_cpu/ms | 0.062 | 0.079 | 0.52 | 94.9 |
| encode/resolve_diagnostics/sum | 0.055 | 0.086 | 0.24 | 84.3 |
| stage/render/prepare_collect_phase_buffers | 0.054 | 0.110 | 0.20 | 82.3 |
| physics/step/ms | 0.028 | 0.057 | 0.20 | 43.3 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.040 | 0.08 | 39.8 |
| weather/map/ms | 0.024 | 0.032 | 0.43 | 36.5 |
| encode/camera_driver_span_end/sum | 0.023 | 0.039 | 0.20 | 35.8 |
| stage/render/cleanup | 0.023 | 0.032 | 0.31 | 35.2 |
| stage/render/phase_sort | 0.018 | 0.033 | 0.07 | 27.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.15 | 16.0 |
| script/update/ms | 0.007 | 0.007 | 0.41 | 10.6 |
| controller/avoid/ms | 0.006 | 0.007 | 0.03 | 8.6 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 7.7 |
| anim/apply/ms | 0.004 | 0.006 | 0.01 | 6.8 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 5.3 |
| controller/write/ms | 0.002 | 0.002 | 0.38 | 3.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.0 |
| stage/render/prepare | 0.000 | 0.002 | 0.04 | 0.7 |
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
| encode//max | ms | 0.52 | 2.12 | 0.88 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.08 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.20 | 0.05 |
| encode/finish_critical | ms | 0.84 | 2.12 | 0.96 |
| encode/pass:early prepass/max | ms | 0.71 | 1.45 | 0.96 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.72 | 1.30 | 0.49 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.31 | 0.13 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.60 | 0.31 |
| encode/pass:shadow_directional_light/max | ms | 0.69 | 2.09 | 0.87 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.24 | 0.10 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.24 | 25.68 | 9.31 |
| gpu/timed/ms | ms | 9.24 | 10.61 | 9.31 |
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
| process/cpu_usage | % | 24.52 | 36.19 | 24.88 |
| process/mem_usage | GiB | 4.64 | 4.70 | 4.70 |
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
| skin/blocks_changed/count | count | 5 | 7 | 4 |
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
| system/cpu_usage | % | 29.18 | 36.17 | 35.06 |
| system/mem_usage | % | 59.98 | 60.28 | 60.24 |
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
| anim/events/count | count | 0.09 | 3 | 135 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 0.00 | 4 | 4 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 29 | 40.5 k |
| encode/bind_walk/init_textures | count | 7164 | 14.0 k | 10.93 M |
| encode/bind_walk/init_textures_skipped | count | 39.5 k | 52.6 k | 60.25 M |
| encode/bind_walk/inits | count | 345 | 437 | 527.0 k |
| encode/bind_walk/inits_skipped | count | 70.99 | 83 | 108.3 k |
| encode/bind_walk/merge_textures | count | 7101 | 13.9 k | 10.84 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.23 M |
| encode/bind_walk/merges | count | 82.25 | 103 | 125.5 k |
| encode/bind_walk/merges_skipped | count | 52.75 | 58 | 80.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1526 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1526 |
| encode/pass:early prepass/count | count | 1 | 1 | 1526 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1526 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1526 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1526 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1526 |
| encode/queue/command_buffers | count | 36.57 | 39 | 55.8 k |
| encode/queue/submits | count | 2 | 2 | 3052 |
| encode/queue/write_buffer/bytes | bytes | 1.38 M | 3.65 M | 2.11 G |
| encode/queue/write_buffer/calls | count | 217 | 287 | 331.1 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 49.81 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.7 k |
| encode/queue/write_texture/bytes | bytes | 11.8 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1526 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.59 | 2 | 900 |
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

## Hitches (> 25.0 ms; slowest 62, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2551.6 | main/busy/ms 2519, stage/main/post_update 2407, stage/main/update 103, surface/advance_cpu_ms 71.75, stage/main/pre_update 5.16, stage/main/last 0.99, surface/cell_upload_ms 0.80, stream/drive/ms 0.57, stage/main/first 0.48, stage/main/fixed_main_loop 0.37, weather/map/ms 0.18, stream/tick/ms 0.13 |
| warm-up | 3.30 | 745.3 | main/busy/ms 2657, stage/main/post_update 2432, render/thread/ms 714, render/busy/ms 540, stage/render/prepare_assets 491, stage/main/update 211, stage/render/prepare_views 168, render/acquire/ms 168, surface/advance_cpu_ms 71.75, stage/render/create_views 27.64, stage/render/render 15.82, render/graph/ms 8.81 |
| warm-up | 3.48 | 179.9 | render/thread/ms 877, render/busy/ms 610, stage/render/prepare_assets 547, stage/render/prepare_views 168, render/acquire/ms 168, main/busy/ms 164, stage/render/render 114, stage/main/update 113, render/present/ms 99.50, stage/main/post_update 38.39, stage/render/create_views 27.93, terrain/update/ms 18.12 |
| warm-up | 3.69 | 191.2 | pipelines/wait_ms 3690, pipelines/compile_ms 1972, pipelines/front_ms 1753, pipelines/driver_ms 219, render/thread/ms 132, main/busy/ms 132, render/busy/ms 127, stage/render/prepare_assets 102, stage/main/update 78.10, stage/main/post_update 42.24, stage/render/render 16.93, terrain/update/ms 14.55 |
| warm-up | 3.78 | 97.3 | pipelines/wait_ms 3900, pipelines/compile_ms 3109, pipelines/front_ms 2906, render/thread/ms 212, render/busy/ms 208, pipelines/driver_ms 203, stage/render/prepare_assets 158, main/busy/ms 73.88, encode//sum 57.29, stage/render/render 42.08, render/graph/ms 38.30, stage/main/update 35.45 |
| warm-up | 3.82 | 38.4 | pipelines/compile_ms 1454, pipelines/front_ms 1354, pipelines/wait_ms 215, render/thread/ms 131, render/busy/ms 128, pipelines/driver_ms 100, stage/render/prepare_assets 83.42, encode//sum 57.48, stage/render/render 36.95, render/graph/ms 33.52, main/busy/ms 30.71, stage/render/graph_submit 30.61 |
| warm-up | 3.85 | 34.0 | pipelines/compile_ms 370, pipelines/front_ms 353, render/thread/ms 62.70, render/busy/ms 61.18, main/busy/ms 38.75, stage/render/prepare_assets 33.06, pipelines/wait_ms 26.43, pipelines/driver_ms 17.28, stage/main/post_update 16.36, stage/main/update 11.70, stage/render/render 10.35, stage/render/prepare_write_phase_buffers 10.00 |
| warm-up | 3.90 | 27.1 | pipelines/compile_ms 1195, pipelines/front_ms 1132, pipelines/driver_ms 63.38, pipelines/wait_ms 43.45, render/thread/ms 40.17, render/busy/ms 38.15, main/busy/ms 28.62, stage/render/render 20.80, render/graph/ms 18.55, stage/main/post_update 15.34, stage/render/graph_submit 12.41, terrain/update/ms 10.57 |
| warm-up | 3.94 | 40.0 | pipelines/compile_ms 887, pipelines/front_ms 854, render/thread/ms 63.36, main/busy/ms 63.13, render/busy/ms 61.07, stage/main/update 45.03, stream/drive/ms 40.99, pipelines/driver_ms 33.75, pipelines/wait_ms 32.76, stage/render/prepare_assets 31.15, stage/render/render 23.15, render/graph/ms 20.56 |
| warm-up | 4.00 | 57.6 | pipelines/compile_ms 538, pipelines/front_ms 522, render/thread/ms 75.83, render/busy/ms 73.89, main/busy/ms 59.76, stage/render/prepare_assets 53.00, stage/main/update 46.28, stream/drive/ms 41.13, pipelines/driver_ms 16.33, stage/render/render 15.15, render/graph/ms 12.88, stage/render/graph_submit 10.31 |
| warm-up | 4.06 | 57.7 | pipelines/compile_ms 365, pipelines/front_ms 357, render/thread/ms 44.18, render/busy/ms 42.61, main/busy/ms 36.78, stage/main/update 24.53, stage/render/prepare_assets 24.11, stream/drive/ms 20.37, stage/render/render 13.01, render/graph/ms 10.88, pipelines/driver_ms 8.60, encode//sum 8.19 |
| warm-up | 4.10 | 33.8 | pipelines/compile_ms 395, pipelines/front_ms 384, render/thread/ms 60.52, render/busy/ms 58.66, main/busy/ms 43.41, stage/render/prepare_assets 38.39, stage/main/update 26.60, stream/drive/ms 21.44, stage/render/render 14.17, render/graph/ms 11.57, pipelines/driver_ms 10.93, stage/main/post_update 8.83 |
| warm-up | 4.17 | 69.6 | pipelines/compile_ms 977, pipelines/front_ms 951, render/thread/ms 69.57, render/busy/ms 67.88, stage/render/prepare_assets 50.21, main/busy/ms 49.60, stage/main/update 30.92, pipelines/driver_ms 25.72, stream/drive/ms 20.46, stage/render/render 12.50, stage/main/post_update 10.76, render/graph/ms 10.28 |
| warm-up | 4.20 | 37.6 | pipelines/compile_ms 686, pipelines/front_ms 666, main/busy/ms 97.86, stage/main/update 82.65, render/thread/ms 73.76, render/busy/ms 70.89, stream/drive/ms 51.54, stage/render/prepare_assets 48.33, pipelines/driver_ms 19.81, stage/render/render 14.51, pipelines/wait_ms 11.39, pipelines/lock_ms 11.08 |
| warm-up | 4.32 | 116.5 | pipelines/compile_ms 114, pipelines/front_ms 109, main/busy/ms 107, stage/main/update 91.35, render/thread/ms 79.15, render/busy/ms 76.04, stream/drive/ms 53.56, stage/render/prepare_assets 44.07, stage/render/render 19.86, render/graph/ms 15.93, stage/render/graph_submit 12.17, stage/main/post_update 9.34 |
| warm-up | 4.38 | 54.8 | pipelines/compile_ms 979, pipelines/front_ms 945, render/thread/ms 95.52, render/busy/ms 92.51, main/busy/ms 90.05, stage/main/post_update 46.77, stage/render/prepare_assets 41.67, stage/render/render 35.13, pipelines/driver_ms 34.55, stage/main/update 33.93, render/graph/ms 31.48, stream/drive/ms 15.49 |
| warm-up | 4.46 | 84.0 | pipelines/compile_ms 1266, pipelines/front_ms 1220, pipelines/wait_ms 149, main/busy/ms 92.19, render/thread/ms 87.68, render/busy/ms 84.75, stage/main/post_update 50.33, pipelines/driver_ms 45.91, stage/render/prepare_assets 40.86, stage/main/update 32.13, stage/render/render 28.29, render/graph/ms 24.92 |
| warm-up | 4.56 | 97.6 | pipelines/compile_ms 1194, pipelines/front_ms 1154, pipelines/wait_ms 207, render/thread/ms 103, render/busy/ms 101, stage/render/prepare_assets 81.91, main/busy/ms 50.90, pipelines/driver_ms 39.96, stage/main/update 30.45, stage/main/post_update 13.36, pipelines/lock_ms 13.05, stage/render/render 9.39 |
| warm-up | 4.60 | 46.9 | pipelines/compile_ms 2369, pipelines/front_ms 2300, pipelines/wait_ms 217, render/thread/ms 101, render/busy/ms 99.73, stage/render/prepare_assets 81.21, pipelines/driver_ms 69.02, main/busy/ms 35.70, stage/main/update 19.33, pipelines/lock_ms 11.40, stage/render/render 9.27, stage/main/post_update 7.76 |
| warm-up | 4.64 | 36.4 | pipelines/compile_ms 1511, pipelines/front_ms 1468, pipelines/wait_ms 156, render/thread/ms 75.12, render/busy/ms 73.47, stage/render/prepare_assets 44.87, pipelines/driver_ms 42.79, main/busy/ms 38.22, stage/main/update 19.42, stage/render/render 12.50, stage/main/post_update 10.64, render/graph/ms 10.60 |
| warm-up | 4.68 | 36.5 | pipelines/compile_ms 264, pipelines/front_ms 255, render/thread/ms 61.99, render/busy/ms 60.20, main/busy/ms 39.46, stage/render/prepare_assets 25.43, stage/main/update 19.33, stage/main/post_update 13.78, stage/render/render 13.21, render/graph/ms 11.09, pipelines/driver_ms 8.43, stage/render/graph_submit 8.18 |
| warm-up | 4.72 | 44.2 | pipelines/compile_ms 226, pipelines/front_ms 219, main/busy/ms 82.27, stage/main/update 64.67, render/thread/ms 59.73, render/busy/ms 58.34, stream/drive/ms 57.82, stage/render/prepare_assets 20.06, stage/main/post_update 10.83, stage/render/render 9.54, render/graph/ms 7.60, encode//sum 7.01 |
| warm-up | 4.78 | 59.2 | pipelines/compile_ms 568, pipelines/front_ms 551, main/busy/ms 120, stage/main/update 102, stream/drive/ms 95.50, render/thread/ms 53.92, render/busy/ms 52.59, stage/render/prepare_assets 17.63, pipelines/driver_ms 17.03, stage/main/post_update 11.11, stage/render/render 9.56, stage/render/prepare_write_phase_buffers 8.14 |
| warm-up | 4.84 | 55.0 | pipelines/compile_ms 2177, pipelines/front_ms 2114, main/busy/ms 69.45, pipelines/driver_ms 62.91, stage/main/update 48.98, stream/drive/ms 42.19, render/thread/ms 37.02, render/busy/ms 35.56, stage/render/prepare_assets 13.66, stage/main/post_update 13.34, stage/render/render 10.33, render/graph/ms 8.24 |
| warm-up | 4.88 | 41.1 | pipelines/compile_ms 2036, pipelines/front_ms 1975, pipelines/driver_ms 61.44, main/busy/ms 52.66, render/thread/ms 50.55, render/busy/ms 48.89, stage/main/update 31.65, stage/render/render 20.98, render/graph/ms 18.91, stage/render/prepare_assets 14.46, stage/render/camera_driver 11.70, stage/main/post_update 11.56 |
| warm-up | 4.93 | 56.8 | pipelines/compile_ms 524, pipelines/front_ms 503, main/busy/ms 77.80, render/thread/ms 70.91, render/busy/ms 69.35, stage/main/update 58.65, stage/render/prepare_assets 33.23, stage/render/render 21.80, pipelines/driver_ms 20.36, render/graph/ms 19.65, pipelines/wait_ms 14.12, stage/render/camera_driver 11.87 |
| warm-up | 4.97 | 41.7 | pipelines/compile_ms 96.89, pipelines/front_ms 92.09, render/thread/ms 64.11, render/busy/ms 62.70, main/busy/ms 53.81, stage/main/update 38.83, stage/render/prepare_assets 35.11, pipelines/wait_ms 13.02, stream/drive/ms 11.88, stage/render/render 11.27, render/graph/ms 9.19, encode//sum 7.95 |
| warm-up | 5.05 | 25.8 | pipelines/compile_ms 802, pipelines/front_ms 780, main/busy/ms 48.61, render/thread/ms 42.51, render/busy/ms 40.75, stage/main/update 32.93, pipelines/driver_ms 22.28, stream/drive/ms 16.42, stage/render/render 14.74, render/graph/ms 12.59, stage/main/post_update 9.70, stage/render/graph_submit 8.80 |
| warm-up | 5.08 | 25.7 | pipelines/compile_ms 1229, pipelines/front_ms 1196, main/busy/ms 40.38, render/thread/ms 35.59, render/busy/ms 33.92, pipelines/driver_ms 33.34, stage/main/update 26.98, stream/drive/ms 15.92, stage/render/render 14.02, render/graph/ms 11.77, stage/render/prepare_assets 8.89, stage/render/graph_submit 8.19 |
| warm-up | 5.10 | 26.6 | pipelines/compile_ms 480, pipelines/front_ms 467, render/thread/ms 41.26, render/busy/ms 39.90, main/busy/ms 36.94, stage/main/update 21.77, pipelines/driver_ms 13.17, stage/render/render 11.77, stream/drive/ms 11.63, render/graph/ms 9.65, stage/main/post_update 9.27, stage/render/prepare_bind_groups 8.73 |
| warm-up | 5.13 | 32.4 | pipelines/compile_ms 332, pipelines/front_ms 322, render/thread/ms 48.81, render/busy/ms 47.42, main/busy/ms 43.10, stage/main/update 26.00, stage/render/render 12.44, stream/drive/ms 11.57, stage/main/post_update 10.96, stage/render/prepare_assets 10.87, render/graph/ms 10.26, pipelines/driver_ms 9.34 |
| warm-up | 5.16 | 29.6 | pipelines/compile_ms 321, pipelines/front_ms 312, render/thread/ms 46.63, main/busy/ms 45.48, render/busy/ms 45.12, stage/main/update 26.88, stage/render/prepare_assets 17.09, stage/render/render 14.90, render/graph/ms 12.75, stream/drive/ms 12.16, stage/main/post_update 11.68, stage/render/graph_submit 8.79 |
| warm-up | 5.22 | 60.3 | pipelines/compile_ms 243, pipelines/front_ms 236, render/thread/ms 69.21, render/busy/ms 67.69, main/busy/ms 55.73, stage/main/update 35.14, stage/render/prepare_assets 29.09, stream/drive/ms 19.98, stage/render/render 14.63, stage/main/post_update 14.30, render/graph/ms 12.41, stream/spawn/ms 9.49 |
| warm-up | 5.28 | 52.6 | pipelines/compile_ms 1025, pipelines/front_ms 998, render/thread/ms 91.46, render/busy/ms 90.06, main/busy/ms 65.67, stage/main/update 33.07, stage/render/prepare_resources 31.11, stage/render/prepare_assets 30.06, pipelines/driver_ms 27.44, stage/main/post_update 19.36, stream/drive/ms 12.60, stage/render/render 11.69 |
| warm-up | 5.35 | 69.2 | pipelines/compile_ms 1544, pipelines/front_ms 1502, render/thread/ms 99.89, render/busy/ms 98.41, main/busy/ms 51.15, pipelines/driver_ms 42.30, stage/render/render 36.67, render/graph/ms 34.72, stage/render/graph_submit 32.13, stage/render/prepare_assets 28.91, stage/render/prepare_resources 24.69, stage/main/update 21.08 |
| warm-up | 5.38 | 35.3 | pipelines/compile_ms 1167, pipelines/front_ms 1136, render/thread/ms 87.29, render/busy/ms 85.65, main/busy/ms 46.19, stage/render/render 38.92, render/graph/ms 36.92, stage/render/graph_submit 34.16, pipelines/driver_ms 31.40, stage/main/update 27.02, stage/render/prepare_assets 25.73, encode/finish_wall 21.65 |
| warm-up | 5.41 | 28.9 | pipelines/compile_ms 650, pipelines/front_ms 632, render/thread/ms 57.46, render/busy/ms 56.03, main/busy/ms 47.02, stage/main/update 26.22, stage/render/prepare_assets 18.16, pipelines/driver_ms 18.07, pipelines/wait_ms 16.96, stage/render/render 13.94, stream/drive/ms 13.52, stage/main/post_update 13.13 |
| warm-up | 5.48 | 25.6 | pipelines/compile_ms 799, pipelines/front_ms 777, pipelines/wait_ms 92.39, render/thread/ms 41.40, render/busy/ms 40.03, main/busy/ms 31.43, pipelines/driver_ms 21.94, stage/render/prepare_assets 20.15, stage/main/update 14.95, stage/main/post_update 10.96, stage/render/render 9.80, render/graph/ms 7.98 |
| warm-up | 5.52 | 29.3 | pipelines/compile_ms 395, pipelines/front_ms 387, render/thread/ms 35.16, render/busy/ms 34.10, main/busy/ms 31.00, stage/main/update 17.29, stage/render/prepare_assets 14.97, stage/render/render 10.63, render/graph/ms 9.07, encode//sum 8.83, stage/main/post_update 7.68, pipelines/driver_ms 7.62 |
| warm-up | 5.58 | 29.7 | render/thread/ms 35.19, render/busy/ms 33.56, main/busy/ms 28.65, stage/render/render 13.28, stage/main/post_update 12.27, stage/main/update 12.12, render/graph/ms 11.03, encode//sum 8.41, stage/render/graph_submit 7.81, encode/submit 5.05, stream/drive/ms 4.01, stage/render/specialize 3.80 |
| warm-up | 5.63 | 32.7 | pipelines/compile_ms 163, pipelines/front_ms 159, render/thread/ms 36.81, render/busy/ms 35.73, main/busy/ms 29.55, stage/render/prepare_assets 18.21, stage/main/update 12.49, stage/main/post_update 11.43, stage/render/render 8.42, render/graph/ms 6.98, encode//sum 6.67, stage/render/graph_submit 4.65 |
| warm-up | 5.72 | 26.2 | pipelines/compile_ms 120, pipelines/front_ms 113, main/busy/ms 40.65, render/thread/ms 36.21, render/busy/ms 34.03, stage/main/post_update 19.69, stage/render/render 16.53, stage/main/update 16.42, render/graph/ms 14.65, pipelines/wait_ms 13.89, pipelines/lock_ms 13.32, stage/render/graph_submit 11.12 |
| warm-up | 5.75 | 30.8 | pipelines/compile_ms 112, pipelines/front_ms 106, render/thread/ms 45.66, render/busy/ms 43.45, main/busy/ms 42.65, stage/main/update 19.35, stage/main/post_update 17.22, stage/render/render 14.24, pipelines/wait_ms 12.54, pipelines/lock_ms 12.10, render/graph/ms 11.96, stage/render/prepare_assets 10.64 |
| warm-up | 5.80 | 29.8 | render/thread/ms 37.26, render/busy/ms 36.11, main/busy/ms 28.58, stage/render/prepare_assets 14.60, stage/main/post_update 11.97, stage/render/render 11.64, stage/main/update 10.72, render/graph/ms 9.87, encode//sum 8.61, stage/render/graph_submit 6.90, encode/submit 4.12, stream/drive/ms 3.67 |
| warm-up | 5.87 | 39.6 | pipelines/compile_ms 777, pipelines/front_ms 760, render/thread/ms 54.24, render/busy/ms 52.79, main/busy/ms 32.18, stage/render/prepare_assets 30.07, pipelines/driver_ms 17.24, stage/main/post_update 15.93, stage/render/render 11.73, stage/main/update 10.32, render/graph/ms 9.85, pipelines/wait_ms 8.33 |
| warm-up | 5.94 | 41.4 | pipelines/compile_ms 492, pipelines/front_ms 478, render/thread/ms 54.25, render/busy/ms 51.91, main/busy/ms 34.76, stage/render/prepare_assets 22.65, stage/render/render 18.55, render/graph/ms 15.41, pipelines/driver_ms 14.64, stage/main/update 14.51, stage/main/post_update 14.48, stage/render/graph_submit 11.59 |
| warm-up | 5.97 | 34.7 | pipelines/compile_ms 125, pipelines/front_ms 119, render/thread/ms 55.80, render/busy/ms 53.72, main/busy/ms 48.64, stage/render/prepare_assets 31.88, stage/main/update 29.83, stream/drive/ms 21.47, stage/render/render 14.21, stage/main/post_update 11.41, render/graph/ms 11.38, stage/render/graph_submit 8.00 |
| warm-up | 6.04 | 63.5 | pipelines/compile_ms 245, pipelines/front_ms 235, render/thread/ms 65.00, render/busy/ms 63.31, main/busy/ms 55.23, stage/render/prepare_assets 39.70, stage/main/update 35.50, stream/drive/ms 22.11, stage/render/render 15.31, render/graph/ms 12.75, stage/main/post_update 11.46, pipelines/driver_ms 9.51 |
| warm-up | 6.06 | 27.1 | pipelines/compile_ms 278, pipelines/front_ms 267, render/thread/ms 64.52, render/busy/ms 62.96, main/busy/ms 47.42, stage/render/prepare_assets 36.49, stage/main/update 18.31, stage/render/render 15.30, render/graph/ms 12.88, pipelines/driver_ms 10.93, stage/main/post_update 10.13, stage/render/graph_submit 9.46 |
| warm-up | 6.11 | 46.2 | pipelines/compile_ms 738, pipelines/front_ms 716, render/thread/ms 58.15, main/busy/ms 56.71, render/busy/ms 56.53, stage/render/prepare_assets 27.41, stage/main/update 25.33, pipelines/driver_ms 22.19, stage/main/post_update 13.48, stage/render/render 12.30, render/graph/ms 10.05, stream/drive/ms 9.28 |
| warm-up | 6.14 | 25.6 | pipelines/compile_ms 676, pipelines/front_ms 657, render/thread/ms 57.17, render/busy/ms 55.68, main/busy/ms 49.63, stage/render/prepare_assets 27.88, stage/main/update 27.60, pipelines/driver_ms 19.80, stage/main/post_update 15.24, stage/render/render 11.50, render/graph/ms 9.41, stream/drive/ms 9.26 |
| warm-up | 6.17 | 34.0 | pipelines/compile_ms 402, pipelines/front_ms 390, render/thread/ms 52.44, render/busy/ms 51.29, main/busy/ms 35.55, stage/render/prepare_assets 29.91, stage/main/update 14.25, stage/main/post_update 13.50, pipelines/driver_ms 11.81, stage/render/render 9.91, render/graph/ms 8.17, encode//sum 7.20 |
| warm-up | 6.22 | 34.4 | pipelines/compile_ms 1038, pipelines/front_ms 1007, render/thread/ms 48.25, render/busy/ms 43.15, main/busy/ms 31.98, pipelines/driver_ms 30.49, stage/render/prepare_assets 22.13, stage/render/render 16.75, pipelines/wait_ms 14.65, stage/main/post_update 13.79, stage/main/update 12.58, render/graph/ms 11.17 |
| warm-up | 6.27 | 32.6 | pipelines/compile_ms 60.94, pipelines/front_ms 58.72, main/busy/ms 47.00, render/thread/ms 30.71, stage/main/update 29.92, render/busy/ms 29.28, stream/drive/ms 21.67, stage/render/render 13.18, render/graph/ms 11.19, stage/main/post_update 10.48, encode//sum 8.60, stage/render/graph_submit 8.07 |
| warm-up | 6.31 | 39.6 | main/busy/ms 50.93, render/thread/ms 36.01, render/busy/ms 34.61, stage/main/update 30.11, pipelines/compile_ms 28.67, pipelines/front_ms 27.32, stream/drive/ms 21.83, stage/main/post_update 13.45, stage/render/prepare_assets 12.80, stage/render/render 12.45, render/graph/ms 10.34, encode//sum 8.21 |
| warm-up | 6.43 | 34.4 | pipelines/compile_ms 1372, pipelines/front_ms 1341, main/busy/ms 38.75, pipelines/driver_ms 31.09, render/thread/ms 28.16, render/busy/ms 27.04, stage/main/update 17.96, stage/main/post_update 14.38, stage/render/render 11.77, render/graph/ms 10.13, stage/render/prepare_assets 9.40, encode//sum 8.22 |
| warm-up | 6.57 | 28.4 | render/thread/ms 41.62, render/busy/ms 39.78, main/busy/ms 34.93, stage/main/post_update 16.53, stage/render/render 16.02, stage/render/prepare_assets 14.95, render/graph/ms 13.06, encode//sum 12.27, stage/main/update 12.07, stage/render/graph_submit 9.47, encode/submit 5.14, encode/finish_wall 4.31 |
| warm-up | 6.60 | 32.3 | render/thread/ms 43.50, render/busy/ms 42.05, main/busy/ms 26.63, stage/render/prepare_assets 22.58, stage/render/render 12.03, stage/main/post_update 11.46, render/graph/ms 9.70, stage/main/update 9.16, encode//sum 8.93, stage/render/graph_submit 7.00, encode/submit 3.77, stream/drive/ms 3.58 |
| warm-up | 6.69 | 27.0 | main/busy/ms 32.96, render/thread/ms 28.23, render/busy/ms 27.00, pipelines/compile_ms 21.85, pipelines/front_ms 20.20, stage/main/update 16.68, stage/main/post_update 13.27, stage/render/render 12.79, render/graph/ms 11.11, stream/drive/ms 10.69, encode//sum 8.70, stage/render/graph_submit 7.33 |
| warm-up | 6.98 | 29.9 | render/thread/ms 36.59, render/busy/ms 34.81, main/busy/ms 16.74, stage/render/render 10.49, stage/main/post_update 9.86, render/graph/ms 9.13, stage/render/specialize 8.87, stage/render/graph_submit 7.30, encode//sum 6.37, stage/render/queue 6.29, stage/main/update 4.36, encode/submit 4.33 |
| warm-up | 8.20 | 28.0 | render/thread/ms 34.70, render/busy/ms 31.70, main/busy/ms 14.03, stage/render/render 9.30, stage/render/specialize 8.15, stage/main/post_update 8.13, render/graph/ms 8.09, stage/render/graph_submit 6.33, encode//sum 5.53, stage/render/queue 5.49, stage/render/prepare_meshes 3.96, encode/submit 3.48 |
| measure | 0.85 | 27.5 | render/thread/ms 34.42, render/busy/ms 31.99, main/busy/ms 13.51, stage/render/render 10.30, render/graph/ms 9.02, stage/render/specialize 8.48, stage/main/post_update 7.60, encode//sum 7.41, stage/render/graph_submit 7.28, stage/render/queue 5.96, encode/submit 4.36, stage/main/update 3.56 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set mw.matctrl.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=matctrl-5 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 7%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 96.9 W · 77 °C (max 79) · VRAM 6426 MiB · throttled 100% of samples

## Tunables

- `far.handoff_scale=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.bias=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_distance_m=24 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_px=2 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `mw.matctrl.gate=on (command line: --set)`
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
