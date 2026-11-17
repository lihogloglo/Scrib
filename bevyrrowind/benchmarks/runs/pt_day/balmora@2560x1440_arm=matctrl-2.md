# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); mw.matctrl.gate=on (command line: --set); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1552 frames over 15.0 s · **median 9.63 ms (104 fps)** · mean 9.67 · p95 11.10 · p99 11.85 (1% low 84 fps) · worst 27.32 ms · 5 spikes over 1.5× median · 20.0 stutters/min (work 8.0/min)
- warm-up: 278 frames over 8.5 s, worst 2538.1 ms
- hitches (> 25.0 ms): **1** measured, 53 in warm-up
- timed GPU passes: 8.46 ms of a 9.67 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.63 ms, limit 11 ms
- pass worst frame 27.32 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.886 | 2.916 | 2.959 | 0.99 | 0.076 |
| dlss_super_resolution | 1.999 | 2.020 | 2.044 | 0.99 | 0.186 |
| early prepass | 0.592 | 0.598 | 0.609 | 0.99 | 0.094 |
| bloom | 0.312 | 0.315 | 0.331 | 0.99 | 0.060 |
| atmosphere_luts | 0.311 | 0.315 | 0.341 | 0.99 | 0.003 |
| main_transmissive_pass_3d | 0.308 | 0.311 | 0.333 | 0.99 | 0.009 |
| water_velocity | 0.237 | 0.240 | 0.264 | 0.99 | 0.002 |
| water_sspr | 0.205 | 0.207 | 0.210 | 0.99 | 0.002 |
| ssao | 0.189 | 0.191 | 0.197 | 0.99 | 0.007 |
| clouds_trace | 0.147 | 0.149 | 0.169 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.144 | 0.146 | 0.152 | 0.99 | 0.001 |
| water_fft | 0.129 | 0.131 | 0.134 | 0.99 | 0.003 |
| uniform_allocation | 0.119 | 0.120 | 0.162 | 0.99 | 0.023 |
| upscaling | 0.112 | 0.113 | 0.119 | 0.99 | 0.004 |
| clouds_composite | 0.110 | 0.112 | 0.113 | 0.99 | 0.002 |
| bin_unpacking | 0.089 | 0.090 | 0.122 | 0.99 | 0.016 |
| tonemapping | 0.085 | 0.086 | 0.090 | 0.99 | 0.003 |
| clustering | 0.063 | 0.064 | 0.074 | 0.99 | 0.020 |
| render_sky | 0.051 | 0.052 | 0.052 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.412 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 0.99 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.031 | 0.033 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 0.99 | 0.005 |
| lightprobe_radiance_map | 0.028 | 0.222 | 0.234 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.025 | 0.028 | 0.99 | 0.084 |
| water_snapshot | 0.025 | 0.025 | 0.026 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| surface_exposure_far | 0.014 | 0.045 | 0.047 | 0.31 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.024 | 0.99 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.011 | 0.013 | 0.99 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.703 | 10.133 | 26.07 | 13506.9 |
| main/busy/ms | 7.253 | 8.106 | 19.94 | 11256.1 |
| render/busy/ms | 7.021 | 8.071 | 25.31 | 10895.9 |
| stage/render/render | 4.779 | 5.639 | 18.60 | 7417.5 |
| stage/main/post_update | 4.290 | 5.032 | 16.89 | 6658.3 |
| render/graph/ms | 4.193 | 5.002 | 17.85 | 6506.9 |
| encode//sum | 4.151 | 5.472 | 20.11 | 6442.4 |
| stage/render/graph_submit | 3.180 | 3.965 | 16.92 | 4936.1 |
| encode/submit | 1.957 | 2.387 | 5.31 | 3037.5 |
| stage/main/update | 1.820 | 2.236 | 3.41 | 2825.2 |
| stage/render/prepare_views | 1.361 | 2.940 | 3.66 | 2112.7 |
| render/acquire/ms | 1.316 | 2.890 | 4.31 | 2042.3 |
| encode/finish_wall | 1.252 | 1.744 | 13.27 | 1943.7 |
| stage/render/camera_driver | 0.957 | 1.104 | 1.85 | 1485.4 |
| encode/pass:shadow_directional_light/sum | 0.805 | 1.365 | 12.73 | 1249.8 |
| encode/pass:early prepass/sum | 0.755 | 1.043 | 12.48 | 1171.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.743 | 1.044 | 1.55 | 1153.2 |
| stage/render/prepare_write_phase_buffers | 0.553 | 0.756 | 1.12 | 858.0 |
| stage/render/prepare_assets | 0.396 | 0.482 | 1.49 | 615.4 |
| stage/render/prepare_resources | 0.390 | 0.452 | 0.78 | 604.9 |
| bigspace/total/ms | 0.371 | 0.868 | 2.16 | 575.2 |
| render/present/ms | 0.366 | 0.430 | 3.18 | 568.7 |
| stage/main/pre_update | 0.351 | 0.445 | 0.99 | 544.1 |
| stage/main/first | 0.300 | 0.358 | 1.71 | 464.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.252 | 0.375 | 0.81 | 390.6 |
| terrain/update/ms | 0.232 | 0.299 | 0.41 | 360.1 |
| stage/render/prepare_bind_groups | 0.212 | 0.294 | 0.77 | 328.3 |
| terrain/select/ms | 0.202 | 0.261 | 0.36 | 313.5 |
| stage/render/prepare_meshes | 0.200 | 0.252 | 2.20 | 310.8 |
| stage/main/fixed_main_loop | 0.191 | 0.394 | 0.96 | 297.1 |
| stream/drive/ms | 0.187 | 0.250 | 0.72 | 290.3 |
| stage/render/prepare_batch_phases | 0.176 | 0.235 | 0.39 | 273.2 |
| stage/render/extract_commands | 0.175 | 0.208 | 0.34 | 271.5 |
| stage/render/create_views | 0.171 | 0.222 | 0.67 | 266.1 |
| far/select/ms | 0.163 | 0.218 | 0.67 | 252.4 |
| controller/feed/ms | 0.161 | 0.205 | 0.33 | 249.2 |
| stream/tick/ms | 0.159 | 0.219 | 0.69 | 247.2 |
| stage/render/prepare_flush | 0.152 | 0.277 | 0.57 | 236.0 |
| bigspace/high_precision/ms | 0.136 | 0.348 | 1.94 | 210.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.129 | 0.200 | 0.43 | 200.8 |
| stage/main/last | 0.119 | 0.151 | 4.50 | 184.5 |
| controller/move/ms | 0.113 | 0.201 | 0.62 | 174.8 |
| stage/render/queue | 0.094 | 0.118 | 5.84 | 146.2 |
| anim/evaluate/ms | 0.094 | 0.234 | 3.74 | 145.7 |
| bigspace/low_precision/ms | 0.090 | 0.189 | 1.23 | 139.0 |
| stage/render/specialize | 0.072 | 0.089 | 8.77 | 111.6 |
| anim/evaluate_cpu/ms | 0.060 | 0.077 | 0.48 | 93.4 |
| encode/resolve_diagnostics/sum | 0.060 | 0.093 | 0.22 | 93.2 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.111 | 0.19 | 89.5 |
| encode/camera_driver_span_begin/sum | 0.028 | 0.043 | 0.17 | 43.8 |
| physics/step/ms | 0.027 | 0.056 | 0.48 | 42.3 |
| encode/camera_driver_span_end/sum | 0.025 | 0.042 | 0.13 | 39.4 |
| weather/map/ms | 0.024 | 0.033 | 0.06 | 37.5 |
| stage/render/cleanup | 0.023 | 0.033 | 0.32 | 35.4 |
| stage/render/phase_sort | 0.016 | 0.031 | 0.07 | 25.0 |
| weather/eval/ms | 0.011 | 0.015 | 0.05 | 16.8 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 11.2 |
| controller/avoid/ms | 0.006 | 0.007 | 0.05 | 8.9 |
| script/input/ms | 0.006 | 0.007 | 0.04 | 8.6 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 7.0 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 5.9 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 1.1 |
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
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.56 | 12.90 | 0.56 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.17 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.13 | 0.02 |
| encode/finish_critical | ms | 0.89 | 12.90 | 1.26 |
| encode/pass:early prepass/max | ms | 0.74 | 12.48 | 0.93 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.75 | 1.31 | 0.60 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.43 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.24 | 0.81 | 0.24 |
| encode/pass:shadow_directional_light/max | ms | 0.72 | 12.73 | 1.26 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.22 | 0.10 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.18 | 25.31 | 9.24 |
| gpu/timed/ms | ms | 9.18 | 10.76 | 9.24 |
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
| phases/opaque3d/bins | count | 1307 | 1307 | 1307 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/bins | count | 1307 | 1307 | 1307 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 1 | 30 | 23 |
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
| preprocess/bind_groups | count | 0 | 62 | 46 |
| preprocess/bind_groups_created | count | 0 | 62 | 46 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 127 | 127 | 127 |
| process/cpu_usage | % | 25.48 | 37.14 | 32.59 |
| process/mem_usage | GiB | 4.68 | 4.74 | 4.74 |
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
| skin/blocks_changed/count | count | 4 | 7 | 5 |
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
| system/cpu_usage | % | 30.37 | 44.76 | 35.69 |
| system/mem_usage | % | 58.70 | 58.90 | 58.90 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 53 | 82 | 75 |
| views/all/bins | count | 5326 | 8063 | 5326 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 15.3 k | 27.0 k | 15.3 k |
| views/all/pipelines | count | 71 | 78 | 71 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 9496 | 27.2 k | 15.5 k |
| views/cascade/batch_sets | count | 1 | 30 | 23 |
| views/cascade/bins | count | 2152 | 4889 | 2152 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 6053 | 17.7 k | 6053 |
| views/cascade/pipelines | count | 23 | 30 | 23 |
| views/cascade/work_items | count | 34 | 17.7 k | 6053 |
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
| views/main/bins | count | 3174 | 3174 | 3174 |
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
| anim/events/count | count | 0.10 | 2 | 148 |
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
| encode//count | count | 26.57 | 52 | 41.2 k |
| encode/bind_walk/init_textures | count | 7166 | 14.0 k | 11.12 M |
| encode/bind_walk/init_textures_skipped | count | 35.6 k | 62.5 k | 55.26 M |
| encode/bind_walk/inits | count | 345 | 652 | 536.0 k |
| encode/bind_walk/inits_skipped | count | 67 | 126 | 104.0 k |
| encode/bind_walk/merge_textures | count | 7102 | 13.9 k | 11.02 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 8162 | 6.33 M |
| encode/bind_walk/merges | count | 82.26 | 156 | 127.7 k |
| encode/bind_walk/merges_skipped | count | 50.75 | 98 | 78.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1552 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1552 |
| encode/pass:early prepass/count | count | 1 | 2 | 1552 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1552 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 1552 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 1552 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 1552 |
| encode/queue/command_buffers | count | 36.57 | 72 | 56.8 k |
| encode/queue/submits | count | 2 | 4 | 3104 |
| encode/queue/write_buffer/bytes | bytes | 1.38 M | 3.65 M | 2.14 G |
| encode/queue/write_buffer/calls | count | 217 | 400 | 336.8 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 65.3 k | 50.66 M |
| encode/queue/write_buffer_with/calls | count | 9 | 18 | 14.0 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1552 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 2 | 901 |
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

## Hitches (> 25.0 ms; slowest 54, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2538.1 | main/busy/ms 2507, stage/main/post_update 2400, stage/main/update 98.04, surface/advance_cpu_ms 68.74, stage/main/pre_update 4.40, stage/main/last 1.09, surface/cell_upload_ms 0.82, stage/main/first 0.52, stage/main/fixed_main_loop 0.50, stream/drive/ms 0.44, weather/map/ms 0.11, stream/tick/ms 0.11 |
| warm-up | 3.30 | 757.4 | main/busy/ms 2648, stage/main/post_update 2427, render/thread/ms 720, render/busy/ms 557, stage/render/prepare_assets 507, stage/main/update 206, stage/render/prepare_views 160, render/acquire/ms 160, surface/advance_cpu_ms 68.74, stage/render/create_views 27.89, stage/render/render 13.04, render/graph/ms 8.56 |
| warm-up | 3.40 | 108.6 | render/thread/ms 811, render/busy/ms 597, stage/render/prepare_assets 533, main/busy/ms 161, stage/render/prepare_views 161, render/acquire/ms 160, stage/main/update 113, stage/render/render 69.67, render/present/ms 54.29, pipelines/compile_ms 44.37, pipelines/front_ms 38.75, stage/main/post_update 36.80 |
| warm-up | 3.45 | 41.8 | pipelines/compile_ms 316, pipelines/front_ms 271, render/thread/ms 119, pipelines/wait_ms 93.85, stage/render/render 77.88, render/busy/ms 64.94, main/busy/ms 62.19, render/present/ms 53.19, stage/main/post_update 45.74, pipelines/driver_ms 45.49, stage/render/prepare_assets 26.29, terrain/update/ms 25.49 |
| warm-up | 3.60 | 149.5 | pipelines/wait_ms 4972, pipelines/compile_ms 1793, pipelines/front_ms 1649, render/thread/ms 172, render/busy/ms 169, pipelines/driver_ms 144, main/busy/ms 106, stage/main/post_update 53.45, stage/render/render 42.73, stage/render/prepare_resources 39.78, render/graph/ms 38.68, stage/render/prepare_assets 31.59 |
| warm-up | 3.65 | 57.9 | pipelines/wait_ms 5158, pipelines/compile_ms 1970, pipelines/front_ms 1851, render/thread/ms 200, render/busy/ms 198, main/busy/ms 148, pipelines/driver_ms 119, stage/main/update 89.94, stage/render/prepare_resources 52.29, stage/render/prepare_assets 48.54, stage/render/render 39.59, render/graph/ms 37.15 |
| warm-up | 3.75 | 99.6 | pipelines/compile_ms 1797, pipelines/front_ms 1716, pipelines/wait_ms 456, render/thread/ms 120, render/busy/ms 118, main/busy/ms 99.68, pipelines/driver_ms 80.63, stage/render/prepare_assets 73.96, stage/main/update 71.20, stage/render/render 23.15, render/graph/ms 20.88, stage/main/post_update 18.63 |
| warm-up | 3.82 | 65.1 | pipelines/compile_ms 2177, pipelines/front_ms 1812, pipelines/driver_ms 365, pipelines/wait_ms 245, render/thread/ms 125, render/busy/ms 123, stage/render/prepare_assets 104, main/busy/ms 34.80, encode//sum 18.42, stage/main/post_update 17.58, stage/render/render 13.49, render/graph/ms 11.30 |
| warm-up | 3.84 | 25.0 | pipelines/compile_ms 913, pipelines/front_ms 590, pipelines/driver_ms 323, render/thread/ms 83.79, render/busy/ms 81.04, pipelines/wait_ms 69.08, stage/render/prepare_assets 49.91, main/busy/ms 32.36, stage/render/render 25.73, render/graph/ms 22.75, stage/render/graph_submit 20.18, encode//sum 19.33 |
| warm-up | 3.93 | 31.5 | pipelines/compile_ms 259, pipelines/front_ms 247, main/busy/ms 57.98, render/thread/ms 50.36, render/busy/ms 48.41, stage/main/update 46.23, stream/drive/ms 42.43, stage/render/prepare_assets 23.02, stage/render/render 12.98, pipelines/driver_ms 11.81, render/graph/ms 10.63, stage/render/graph_submit 8.30 |
| warm-up | 4.00 | 69.4 | pipelines/compile_ms 1136, pipelines/front_ms 1089, render/thread/ms 73.44, render/busy/ms 71.57, main/busy/ms 55.42, pipelines/wait_ms 53.90, stage/render/prepare_assets 47.53, pipelines/driver_ms 46.95, stage/main/update 45.39, stream/drive/ms 41.49, stage/render/render 12.37, render/graph/ms 10.13 |
| warm-up | 4.07 | 57.9 | pipelines/compile_ms 317, pipelines/front_ms 305, render/thread/ms 52.55, main/busy/ms 48.51, render/busy/ms 47.28, stage/main/update 27.48, stage/render/prepare_assets 26.55, stream/drive/ms 23.40, stage/render/render 17.91, render/graph/ms 12.22, pipelines/driver_ms 12.12, stage/render/graph_submit 9.44 |
| warm-up | 4.11 | 36.6 | pipelines/compile_ms 628, pipelines/front_ms 604, main/busy/ms 78.28, render/thread/ms 60.64, render/busy/ms 58.93, stage/main/update 57.16, stream/drive/ms 51.83, stage/render/prepare_assets 40.16, pipelines/driver_ms 23.95, stage/render/render 13.47, render/graph/ms 11.08, stage/main/post_update 9.35 |
| warm-up | 4.24 | 129.1 | pipelines/compile_ms 1241, pipelines/front_ms 1206, main/busy/ms 81.04, render/thread/ms 76.32, render/busy/ms 74.84, stage/main/update 62.70, stage/render/prepare_assets 55.80, stream/drive/ms 50.88, pipelines/driver_ms 35.46, stage/render/render 13.45, render/graph/ms 11.36, stage/main/post_update 9.67 |
| warm-up | 4.29 | 56.6 | pipelines/compile_ms 935, pipelines/front_ms 910, main/busy/ms 92.70, render/thread/ms 85.57, render/busy/ms 84.07, stage/main/update 73.14, stage/render/prepare_assets 63.55, stream/drive/ms 37.32, pipelines/driver_ms 24.94, stage/render/render 13.25, render/graph/ms 11.17, stage/main/post_update 9.06 |
| warm-up | 4.38 | 82.2 | pipelines/compile_ms 116, pipelines/front_ms 111, main/busy/ms 91.04, stage/main/update 70.27, render/thread/ms 63.85, render/busy/ms 62.28, stage/render/prepare_assets 43.41, stream/drive/ms 37.71, stage/render/render 10.01, stage/main/post_update 10.00, render/graph/ms 8.02, encode//sum 6.09 |
| warm-up | 4.45 | 72.4 | pipelines/compile_ms 1349, pipelines/front_ms 1294, pipelines/wait_ms 281, render/thread/ms 98.96, render/busy/ms 97.42, main/busy/ms 57.42, pipelines/driver_ms 54.63, stage/render/prepare_assets 47.44, stage/main/update 24.95, stage/render/prepare_write_phase_buffers 20.16, stage/main/post_update 16.87, stage/render/render 10.60 |
| warm-up | 4.50 | 52.8 | pipelines/compile_ms 1642, pipelines/front_ms 1571, pipelines/wait_ms 367, render/thread/ms 110, render/busy/ms 109, main/busy/ms 78.01, pipelines/driver_ms 70.46, stage/render/prepare_assets 40.11, stage/main/post_update 36.02, stage/main/update 26.17, stage/render/prepare_write_phase_buffers 20.72, stage/render/prepare_resources 14.27 |
| warm-up | 4.54 | 37.5 | pipelines/compile_ms 325, pipelines/front_ms 307, pipelines/wait_ms 86.47, render/thread/ms 69.31, render/busy/ms 67.44, main/busy/ms 59.72, stage/main/post_update 30.39, stage/render/prepare_assets 26.64, stage/main/update 20.75, pipelines/driver_ms 17.55, stage/render/render 13.89, stage/render/prepare_resources 10.59 |
| warm-up | 4.60 | 61.1 | pipelines/compile_ms 854, pipelines/front_ms 826, render/thread/ms 60.83, render/busy/ms 59.42, stage/render/prepare_assets 39.94, main/busy/ms 39.16, pipelines/driver_ms 27.59, stage/main/update 17.19, stage/main/post_update 14.41, stage/render/render 9.47, render/graph/ms 7.66, encode//sum 5.73 |
| warm-up | 4.64 | 37.7 | pipelines/compile_ms 1520, pipelines/front_ms 1469, pipelines/wait_ms 107, render/thread/ms 68.79, render/busy/ms 66.97, main/busy/ms 53.24, pipelines/driver_ms 51.17, stage/render/prepare_assets 31.20, stage/main/update 25.56, stage/main/post_update 19.80, stage/render/render 14.96, render/graph/ms 12.75 |
| warm-up | 4.68 | 42.8 | pipelines/compile_ms 1335, pipelines/front_ms 1293, pipelines/wait_ms 157, render/thread/ms 64.80, render/busy/ms 62.72, main/busy/ms 48.05, pipelines/driver_ms 41.49, stage/main/update 26.09, stage/render/prepare_assets 21.31, stage/render/render 17.34, stage/main/post_update 15.21, render/graph/ms 14.70 |
| warm-up | 4.79 | 113.3 | pipelines/compile_ms 1227, pipelines/front_ms 1195, render/thread/ms 135, render/busy/ms 133, main/busy/ms 103, stage/main/update 85.71, stream/drive/ms 79.41, pipelines/wait_ms 53.39, stage/render/prepare_assets 42.83, pipelines/driver_ms 32.31, stage/render/prepare_resources 23.37, stage/render/prepare_write_phase_buffers 22.26 |
| warm-up | 4.83 | 38.7 | pipelines/compile_ms 1391, pipelines/front_ms 1352, pipelines/wait_ms 232, main/busy/ms 144, render/thread/ms 126, stage/main/update 126, render/busy/ms 124, stream/drive/ms 120, pipelines/driver_ms 38.76, stage/render/prepare_assets 36.21, stage/render/prepare_resources 23.20, stage/render/prepare_write_phase_buffers 22.71 |
| warm-up | 4.89 | 54.8 | pipelines/compile_ms 2183, pipelines/front_ms 2112, pipelines/wait_ms 257, main/busy/ms 77.16, pipelines/driver_ms 70.71, stage/main/update 51.03, stream/drive/ms 44.46, render/thread/ms 35.68, render/busy/ms 33.95, stage/main/post_update 16.05, stage/render/prepare_assets 14.75, stage/render/render 12.18 |
| warm-up | 4.93 | 45.7 | pipelines/compile_ms 1635, pipelines/front_ms 1578, pipelines/wait_ms 87.27, main/busy/ms 62.32, pipelines/driver_ms 56.98, render/thread/ms 45.95, render/busy/ms 38.15, stage/main/update 33.61, stage/render/render 23.16, stage/main/post_update 16.32, pipelines/lock_ms 15.58, render/graph/ms 14.79 |
| warm-up | 5.04 | 101.6 | pipelines/compile_ms 912, pipelines/front_ms 887, pipelines/wait_ms 313, render/thread/ms 108, render/busy/ms 100, main/busy/ms 80.18, stage/render/prepare_assets 63.33, stage/main/update 62.15, pipelines/driver_ms 25.17, stage/render/render 23.66, render/graph/ms 15.36, stream/drive/ms 14.88 |
| warm-up | 5.10 | 61.3 | pipelines/compile_ms 1065, pipelines/front_ms 1040, pipelines/wait_ms 252, render/thread/ms 132, render/busy/ms 130, main/busy/ms 87.55, stage/render/prepare_assets 73.20, stage/main/update 50.76, pipelines/driver_ms 25.21, stream/drive/ms 24.71, stage/main/post_update 20.25, stage/render/prepare_resources 15.78 |
| warm-up | 5.13 | 37.8 | pipelines/compile_ms 2056, pipelines/front_ms 2003, render/thread/ms 89.70, render/busy/ms 88.25, main/busy/ms 72.29, pipelines/wait_ms 69.02, pipelines/driver_ms 52.19, stage/render/prepare_assets 37.69, stage/main/post_update 33.40, stage/main/update 22.45, stage/render/prepare_resources 14.23, stream/drive/ms 14.11 |
| warm-up | 5.16 | 26.8 | pipelines/compile_ms 1856, pipelines/front_ms 1805, pipelines/wait_ms 82.29, render/thread/ms 59.66, render/busy/ms 57.87, pipelines/driver_ms 50.93, main/busy/ms 47.78, stage/main/post_update 23.63, stage/render/prepare_assets 21.45, stage/main/update 17.65, stage/render/render 15.22, pipelines/lock_ms 14.06 |
| warm-up | 5.19 | 30.1 | pipelines/compile_ms 417, pipelines/front_ms 401, render/thread/ms 50.31, render/busy/ms 48.61, main/busy/ms 35.76, stage/main/update 18.22, pipelines/driver_ms 15.62, pipelines/wait_ms 14.56, stage/render/render 14.04, render/graph/ms 11.87, stage/main/post_update 10.46, stage/render/graph_submit 9.08 |
| warm-up | 5.26 | 48.6 | pipelines/compile_ms 220, pipelines/front_ms 212, render/thread/ms 47.10, render/busy/ms 45.56, main/busy/ms 43.51, stage/main/update 28.79, stream/drive/ms 18.64, stage/render/render 12.71, render/graph/ms 10.70, stage/render/prepare_resources 10.05, stage/render/prepare_assets 9.75, stage/main/post_update 9.28 |
| warm-up | 5.32 | 38.1 | pipelines/compile_ms 1057, pipelines/front_ms 1024, pipelines/wait_ms 77.19, render/thread/ms 44.81, render/busy/ms 43.06, main/busy/ms 39.77, pipelines/driver_ms 33.01, stage/main/update 20.39, stage/render/render 16.82, stage/render/prepare_assets 16.38, render/graph/ms 14.39, stream/drive/ms 13.74 |
| warm-up | 5.41 | 69.7 | pipelines/compile_ms 1284, pipelines/front_ms 1245, main/busy/ms 61.62, render/thread/ms 51.52, render/busy/ms 49.84, pipelines/driver_ms 39.12, stage/main/update 34.17, stage/render/prepare_assets 28.36, pipelines/wait_ms 27.18, stage/main/post_update 21.12, stream/drive/ms 19.92, stage/render/render 12.66 |
| warm-up | 5.44 | 29.8 | pipelines/compile_ms 729, pipelines/front_ms 709, render/thread/ms 55.32, render/busy/ms 54.13, stage/render/prepare_assets 33.99, main/busy/ms 33.16, pipelines/wait_ms 24.90, pipelines/driver_ms 20.04, stage/main/update 16.79, stage/render/render 10.70, stage/main/post_update 10.42, render/graph/ms 8.85 |
| warm-up | 5.68 | 35.1 | pipelines/compile_ms 99.31, pipelines/front_ms 94.76, render/thread/ms 50.77, render/busy/ms 49.06, main/busy/ms 30.11, stage/render/render 15.39, stage/render/prepare_assets 15.10, render/graph/ms 13.04, stage/main/update 11.41, stage/main/post_update 11.36, encode//sum 10.55, stage/render/graph_submit 9.51 |
| warm-up | 5.73 | 43.7 | pipelines/compile_ms 99.31, pipelines/front_ms 94.76, render/thread/ms 66.17, render/busy/ms 64.61, stage/render/prepare_assets 35.97, main/busy/ms 32.87, stage/main/post_update 12.49, stage/render/render 12.24, stage/main/update 11.10, render/graph/ms 10.00, encode//sum 9.70, stage/render/graph_submit 7.01 |
| warm-up | 5.76 | 34.9 | render/thread/ms 61.40, render/busy/ms 60.20, stage/render/prepare_assets 41.33, main/busy/ms 34.37, stage/main/update 13.83, stage/main/post_update 12.26, stage/render/render 8.11, render/graph/ms 6.52, encode//sum 6.51, stream/drive/ms 5.17, stage/render/graph_submit 4.46, stage/main/pre_update 3.22 |
| warm-up | 5.80 | 36.4 | render/thread/ms 57.17, render/busy/ms 55.84, stage/render/prepare_assets 34.50, main/busy/ms 32.80, stage/main/update 14.69, stage/main/post_update 11.65, stage/render/render 8.99, render/graph/ms 7.18, encode//sum 6.87, pipelines/compile_ms 5.56, pipelines/front_ms 5.23, stream/drive/ms 4.85 |
| warm-up | 5.86 | 33.9 | pipelines/compile_ms 881, pipelines/front_ms 857, render/thread/ms 50.26, render/busy/ms 48.97, main/busy/ms 32.49, stage/render/prepare_assets 25.99, pipelines/driver_ms 24.36, stage/main/update 14.35, stage/render/render 13.06, stage/main/post_update 12.22, render/graph/ms 11.12, encode//sum 8.94 |
| warm-up | 5.88 | 28.9 | pipelines/compile_ms 655, pipelines/front_ms 638, render/thread/ms 55.28, render/busy/ms 53.86, main/busy/ms 39.21, stage/render/prepare_assets 32.81, stage/main/update 21.33, pipelines/driver_ms 16.97, stage/main/post_update 12.79, stage/render/render 10.24, render/graph/ms 8.40, encode//sum 6.77 |
| warm-up | 5.92 | 39.8 | pipelines/compile_ms 110, pipelines/front_ms 106, render/thread/ms 56.29, render/busy/ms 54.97, main/busy/ms 39.73, stage/render/prepare_assets 31.29, stage/main/update 18.51, stage/main/post_update 13.86, stage/render/render 9.93, render/graph/ms 8.08, encode//sum 7.13, stage/render/graph_submit 5.31 |
| warm-up | 6.00 | 56.4 | render/thread/ms 52.99, render/busy/ms 51.02, main/busy/ms 43.85, stage/render/prepare_assets 26.52, pipelines/compile_ms 22.65, stage/main/update 22.54, pipelines/front_ms 20.50, stream/drive/ms 16.00, stage/render/render 15.64, render/graph/ms 13.20, stage/main/post_update 12.18, stage/render/graph_submit 9.70 |
| warm-up | 6.04 | 41.6 | pipelines/compile_ms 747, pipelines/front_ms 722, render/thread/ms 64.24, render/busy/ms 62.25, stage/render/prepare_assets 40.20, main/busy/ms 37.65, pipelines/driver_ms 25.20, stage/main/update 17.27, stage/main/post_update 12.60, stage/render/render 12.59, render/graph/ms 10.03, stream/drive/ms 9.75 |
| warm-up | 6.07 | 36.6 | pipelines/compile_ms 1118, pipelines/front_ms 1085, render/thread/ms 51.59, render/busy/ms 50.39, main/busy/ms 44.73, stage/render/prepare_assets 33.46, pipelines/driver_ms 32.35, stage/main/update 24.31, stage/main/post_update 12.49, stream/drive/ms 8.91, stage/render/render 8.59, render/graph/ms 6.83 |
| warm-up | 6.10 | 27.2 | pipelines/compile_ms 393, pipelines/front_ms 384, render/thread/ms 47.02, render/busy/ms 45.96, main/busy/ms 35.23, stage/render/prepare_assets 29.50, stage/main/update 18.80, stage/main/post_update 10.63, pipelines/driver_ms 9.30, stage/render/render 9.22, render/graph/ms 7.61, encode//sum 6.51 |
| warm-up | 6.17 | 26.2 | pipelines/compile_ms 58.36, pipelines/front_ms 55.65, render/thread/ms 39.65, render/busy/ms 38.00, main/busy/ms 37.72, stage/main/post_update 18.31, stage/render/render 13.37, stage/main/update 13.21, render/graph/ms 11.48, stage/render/graph_submit 8.30, stage/render/prepare_assets 7.70, encode//sum 7.27 |
| warm-up | 6.26 | 35.7 | pipelines/compile_ms 348, pipelines/front_ms 340, main/busy/ms 43.19, render/thread/ms 28.38, stage/main/update 27.40, render/busy/ms 27.11, stream/drive/ms 19.04, stage/render/render 11.77, render/graph/ms 10.03, stage/main/post_update 9.68, stage/render/prepare_assets 8.63, encode//sum 7.82 |
| warm-up | 6.35 | 26.8 | pipelines/compile_ms 175, pipelines/front_ms 172, main/busy/ms 34.86, render/thread/ms 25.93, render/busy/ms 24.72, stage/main/update 17.21, stage/main/post_update 12.98, stage/render/render 11.10, render/graph/ms 9.34, stage/render/prepare_assets 8.66, encode//sum 7.91, stage/render/graph_submit 6.17 |
| warm-up | 6.59 | 30.5 | main/busy/ms 43.00, render/thread/ms 40.13, render/busy/ms 38.39, stage/main/update 22.97, stage/render/render 16.09, stage/main/post_update 13.96, render/graph/ms 13.69, encode//sum 13.03, stage/render/prepare_assets 12.41, stream/drive/ms 11.06, stage/render/graph_submit 9.85, encode/submit 5.12 |
| warm-up | 6.63 | 43.3 | render/thread/ms 49.73, render/busy/ms 48.62, main/busy/ms 39.41, stage/render/prepare_assets 29.62, stage/main/update 21.11, stage/main/post_update 12.50, stream/drive/ms 11.47, stage/render/render 10.35, encode//sum 9.19, render/graph/ms 8.70, stage/render/graph_submit 5.83, encode/finish_wall 3.42 |
| warm-up | 6.93 | 32.7 | render/thread/ms 39.85, render/busy/ms 38.14, main/busy/ms 16.64, stage/render/render 11.26, render/graph/ms 9.45, stage/render/specialize 9.18, stage/main/post_update 8.35, encode//sum 8.16, stage/render/graph_submit 7.46, stage/render/queue 6.92, stage/main/update 5.68, encode/submit 4.89 |
| warm-up | 8.14 | 30.0 | render/thread/ms 36.81, render/busy/ms 34.03, main/busy/ms 15.29, stage/render/render 9.69, stage/main/post_update 8.68, stage/render/specialize 8.62, render/graph/ms 8.37, stage/render/graph_submit 6.54, stage/render/queue 6.11, encode//sum 6.02, stage/render/prepare_meshes 4.50, stage/main/update 4.41 |
| measure | 0.84 | 27.3 | render/thread/ms 34.96, render/busy/ms 32.52, main/busy/ms 13.32, stage/render/render 11.35, encode//sum 9.75, render/graph/ms 9.73, stage/render/specialize 8.83, stage/render/graph_submit 7.80, stage/main/post_update 7.57, stage/render/queue 5.93, encode/submit 4.79, stage/main/update 3.48 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set mw.matctrl.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=matctrl-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 11%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 89%) · 2625 MHz (min 2625) · memory 8101 MHz · 97.7 W · 77 °C (max 80) · VRAM 6427 MiB · throttled 100% of samples

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
