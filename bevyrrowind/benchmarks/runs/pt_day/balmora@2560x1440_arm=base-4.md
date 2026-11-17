# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1541 frames over 15.0 s · **median 9.71 ms (103 fps)** · mean 9.74 · p95 11.20 · p99 12.58 (1% low 79 fps) · worst 26.66 ms · 8 spikes over 1.5× median · 32.0 stutters/min (work 20.0/min)
- warm-up: 279 frames over 8.7 s, worst 2655.5 ms
- hitches (> 25.0 ms): **1** measured, 56 in warm-up
- timed GPU passes: 8.47 ms of a 9.74 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.71 ms, limit 11 ms
- pass worst frame 26.66 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.903 | 2.951 | 2.996 | 0.98 | 0.073 |
| dlss_super_resolution | 1.989 | 2.022 | 2.044 | 0.98 | 0.187 |
| early prepass | 0.593 | 0.603 | 0.612 | 0.98 | 0.095 |
| atmosphere_luts | 0.321 | 0.326 | 0.342 | 0.98 | 0.003 |
| bloom | 0.311 | 0.316 | 0.332 | 0.98 | 0.064 |
| main_transmissive_pass_3d | 0.307 | 0.312 | 0.333 | 0.98 | 0.009 |
| water_velocity | 0.236 | 0.240 | 0.264 | 0.98 | 0.002 |
| water_sspr | 0.212 | 0.216 | 0.216 | 0.98 | 0.002 |
| ssao | 0.188 | 0.191 | 0.197 | 0.98 | 0.007 |
| clouds_trace | 0.147 | 0.149 | 0.169 | 0.98 | 0.001 |
| atmos_froxel_inject | 0.143 | 0.146 | 0.153 | 0.98 | 0.002 |
| water_fft | 0.130 | 0.132 | 0.135 | 0.98 | 0.003 |
| uniform_allocation | 0.115 | 0.117 | 0.159 | 0.98 | 0.022 |
| upscaling | 0.111 | 0.113 | 0.118 | 0.98 | 0.004 |
| clouds_composite | 0.110 | 0.112 | 0.113 | 0.98 | 0.002 |
| bin_unpacking | 0.086 | 0.087 | 0.118 | 0.98 | 0.016 |
| tonemapping | 0.084 | 0.086 | 0.089 | 0.98 | 0.003 |
| clustering | 0.063 | 0.064 | 0.075 | 0.98 | 0.021 |
| render_sky | 0.051 | 0.052 | 0.052 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.412 | 0.412 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 0.98 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 0.98 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.031 | 0.032 | 0.98 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 0.98 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.221 | 0.230 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.026 | 0.029 | 0.98 | 0.082 |
| water_snapshot | 0.025 | 0.025 | 0.026 | 0.98 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.98 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.046 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 0.98 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 0.98 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.98 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 0.98 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.98 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.736 | 10.165 | 25.11 | 13462.7 |
| main/busy/ms | 7.433 | 8.333 | 10.07 | 11453.6 |
| render/busy/ms | 7.286 | 8.478 | 24.47 | 11227.6 |
| stage/render/render | 4.928 | 5.900 | 17.33 | 7593.8 |
| stage/main/post_update | 4.420 | 5.131 | 7.08 | 6811.1 |
| render/graph/ms | 4.310 | 5.182 | 16.72 | 6642.3 |
| encode//sum | 4.182 | 5.480 | 16.94 | 6444.4 |
| stage/render/graph_submit | 3.282 | 4.127 | 15.63 | 5056.9 |
| encode/submit | 2.080 | 2.574 | 14.37 | 3205.8 |
| stage/main/update | 1.858 | 2.257 | 3.21 | 2863.9 |
| encode/finish_wall | 1.242 | 1.794 | 5.91 | 1914.4 |
| stage/render/prepare_views | 1.106 | 2.701 | 3.70 | 1704.5 |
| render/acquire/ms | 1.060 | 2.655 | 3.66 | 1633.2 |
| stage/render/camera_driver | 0.972 | 1.117 | 1.74 | 1498.1 |
| encode/pass:shadow_directional_light/sum | 0.789 | 1.373 | 2.51 | 1215.2 |
| encode/pass:early prepass/sum | 0.745 | 1.042 | 4.65 | 1148.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.734 | 1.033 | 1.88 | 1130.7 |
| stage/render/prepare_write_phase_buffers | 0.540 | 0.760 | 1.12 | 832.7 |
| stage/render/prepare_assets | 0.472 | 0.610 | 1.61 | 727.4 |
| bigspace/total/ms | 0.404 | 1.052 | 1.60 | 621.8 |
| stage/render/prepare_resources | 0.395 | 0.466 | 0.78 | 608.6 |
| render/present/ms | 0.391 | 0.466 | 2.64 | 601.8 |
| stage/main/pre_update | 0.359 | 0.499 | 0.98 | 553.7 |
| stage/main/first | 0.296 | 0.365 | 1.35 | 456.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.258 | 0.374 | 0.62 | 396.8 |
| terrain/update/ms | 0.234 | 0.302 | 0.77 | 361.4 |
| stage/render/prepare_bind_groups | 0.215 | 0.304 | 0.43 | 331.7 |
| stage/render/prepare_meshes | 0.212 | 0.273 | 2.09 | 326.0 |
| terrain/select/ms | 0.204 | 0.262 | 0.73 | 314.3 |
| stream/drive/ms | 0.196 | 0.264 | 0.79 | 302.2 |
| stage/main/fixed_main_loop | 0.191 | 0.372 | 1.10 | 294.0 |
| stage/render/prepare_batch_phases | 0.187 | 0.251 | 0.43 | 288.9 |
| stage/render/create_views | 0.176 | 0.225 | 0.69 | 270.6 |
| stage/render/extract_commands | 0.173 | 0.204 | 0.53 | 266.9 |
| far/select/ms | 0.173 | 0.242 | 0.73 | 265.8 |
| stream/tick/ms | 0.166 | 0.228 | 0.58 | 256.5 |
| controller/feed/ms | 0.164 | 0.216 | 0.57 | 252.3 |
| bigspace/high_precision/ms | 0.156 | 0.694 | 1.41 | 239.8 |
| stage/render/prepare_flush | 0.153 | 0.257 | 0.58 | 235.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.130 | 0.202 | 0.48 | 199.7 |
| stage/main/last | 0.120 | 0.164 | 0.67 | 185.2 |
| stage/render/queue | 0.112 | 0.146 | 5.56 | 172.3 |
| controller/move/ms | 0.109 | 0.164 | 0.60 | 167.2 |
| bigspace/low_precision/ms | 0.096 | 0.215 | 1.32 | 148.7 |
| stage/render/specialize | 0.091 | 0.115 | 8.15 | 140.1 |
| anim/evaluate/ms | 0.087 | 0.198 | 1.00 | 133.6 |
| stage/render/prepare_collect_phase_buffers | 0.063 | 0.122 | 0.25 | 96.5 |
| anim/evaluate_cpu/ms | 0.062 | 0.077 | 0.45 | 95.3 |
| encode/resolve_diagnostics/sum | 0.060 | 0.090 | 0.42 | 91.8 |
| physics/step/ms | 0.029 | 0.059 | 0.44 | 44.1 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.042 | 0.26 | 42.3 |
| weather/map/ms | 0.025 | 0.034 | 0.06 | 38.8 |
| encode/camera_driver_span_end/sum | 0.024 | 0.038 | 0.17 | 36.8 |
| stage/render/cleanup | 0.023 | 0.034 | 0.36 | 35.9 |
| stage/render/phase_sort | 0.019 | 0.035 | 0.12 | 29.1 |
| weather/eval/ms | 0.012 | 0.015 | 0.45 | 18.4 |
| script/update/ms | 0.008 | 0.011 | 0.50 | 12.5 |
| controller/avoid/ms | 0.006 | 0.008 | 0.02 | 9.2 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 8.5 |
| script/ui/ms | 0.005 | 0.007 | 0.07 | 7.6 |
| anim/apply/ms | 0.005 | 0.006 | 0.03 | 6.9 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.48 | 1.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.6 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 0.9 |
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
| encode//max | ms | 0.59 | 5.85 | 0.61 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.26 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.17 | 0.03 |
| encode/finish_critical | ms | 0.89 | 5.85 | 0.87 |
| encode/pass:early prepass/max | ms | 0.73 | 4.65 | 0.58 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.74 | 1.45 | 0.87 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.48 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 0.52 | 0.26 |
| encode/pass:shadow_directional_light/max | ms | 0.72 | 2.13 | 0.60 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.42 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.23 | 24.47 | 9.21 |
| gpu/timed/ms | ms | 9.23 | 10.79 | 9.21 |
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
| process/cpu_usage | % | 27.34 | 36.06 | 26.92 |
| process/mem_usage | GiB | 4.67 | 4.76 | 4.76 |
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
| system/cpu_usage | % | 38.11 | 47.81 | 35.10 |
| system/mem_usage | % | 59.46 | 60.07 | 60.07 |
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
| anim/events/count | count | 0.10 | 3 | 160 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.21 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 54 | 40.9 k |
| encode/bind_walk/init_textures | count | 7165 | 16.3 k | 11.04 M |
| encode/bind_walk/init_textures_skipped | count | 31.7 k | 72.2 k | 48.89 M |
| encode/bind_walk/inits | count | 341 | 725 | 526.1 k |
| encode/bind_walk/inits_skipped | count | 63.00 | 134 | 97.1 k |
| encode/bind_walk/merge_textures | count | 7101 | 16.2 k | 10.94 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 8162 | 6.29 M |
| encode/bind_walk/merges | count | 82.26 | 173 | 126.8 k |
| encode/bind_walk/merges_skipped | count | 48.75 | 101 | 75.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1541 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1541 |
| encode/pass:early prepass/count | count | 1 | 2 | 1541 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1541 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 1541 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 1541 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 1541 |
| encode/queue/command_buffers | count | 36.57 | 74 | 56.4 k |
| encode/queue/submits | count | 2 | 4 | 3082 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.65 M | 2.22 G |
| encode/queue/write_buffer/calls | count | 211 | 457 | 325.6 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 65.3 k | 50.30 M |
| encode/queue/write_buffer_with/calls | count | 9 | 18 | 13.9 k |
| encode/queue/write_texture/bytes | bytes | 11.7 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1541 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 2 | 900 |
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

## Hitches (> 25.0 ms; slowest 57, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.66 | 2655.5 | main/busy/ms 2625, stage/main/post_update 2520, stage/main/update 96.95, surface/advance_cpu_ms 67.04, stage/main/pre_update 4.40, stage/main/last 1.10, surface/cell_upload_ms 0.74, stream/drive/ms 0.61, stage/main/first 0.58, stage/main/fixed_main_loop 0.34, stream/tick/ms 0.13, weather/map/ms 0.09 |
| warm-up | 3.48 | 824.9 | main/busy/ms 2802, stage/main/post_update 2548, render/thread/ms 788, render/busy/ms 649, stage/render/prepare_assets 598, stage/main/update 240, stage/render/prepare_views 136, render/acquire/ms 136, surface/advance_cpu_ms 67.04, stage/render/create_views 28.66, stage/render/render 13.20, render/graph/ms 8.06 |
| warm-up | 3.59 | 110.6 | render/thread/ms 882, render/busy/ms 690, stage/render/prepare_assets 625, main/busy/ms 197, stage/main/update 147, stage/render/prepare_views 136, render/acquire/ms 136, stage/render/render 71.29, render/present/ms 55.92, stage/main/post_update 39.77, stage/render/create_views 29.19, terrain/update/ms 22.05 |
| warm-up | 3.65 | 55.2 | pipelines/compile_ms 415, pipelines/front_ms 373, pipelines/wait_ms 327, render/thread/ms 148, stage/render/render 104, render/busy/ms 80.75, main/busy/ms 80.50, render/present/ms 66.51, stage/main/post_update 63.52, pipelines/driver_ms 41.88, render/graph/ms 35.97, stage/render/prepare_assets 28.61 |
| warm-up | 3.78 | 134.1 | pipelines/wait_ms 4690, pipelines/compile_ms 1585, pipelines/front_ms 1466, render/thread/ms 175, render/busy/ms 159, main/busy/ms 156, pipelines/driver_ms 119, stage/main/post_update 76.74, stage/main/update 72.75, stage/render/render 68.34, render/graph/ms 52.65, stage/render/graph_submit 42.83 |
| warm-up | 3.85 | 66.2 | pipelines/wait_ms 4505, pipelines/compile_ms 1842, pipelines/front_ms 1739, render/thread/ms 184, render/busy/ms 182, main/busy/ms 145, pipelines/driver_ms 104, stage/main/update 79.14, stage/render/prepare_assets 62.24, stage/main/post_update 57.92, stage/render/prepare_resources 32.93, stage/render/prepare_flush 28.87 |
| warm-up | 3.88 | 37.9 | pipelines/compile_ms 1524, pipelines/front_ms 1462, pipelines/wait_ms 314, render/thread/ms 98.65, render/busy/ms 96.80, main/busy/ms 78.25, stage/render/prepare_assets 76.44, pipelines/driver_ms 62.02, stage/main/post_update 39.18, terrain/update/ms 17.54, stage/main/update 15.95, stage/main/pre_update 11.20 |
| warm-up | 3.96 | 48.1 | pipelines/compile_ms 73.00, render/thread/ms 70.23, render/busy/ms 68.16, pipelines/front_ms 68.12, main/busy/ms 55.61, stage/main/post_update 43.90, encode//sum 22.23, stage/render/render 21.46, render/graph/ms 19.06, bigspace/total/ms 18.30, bigspace/low_precision/ms 18.09, stage/render/graph_submit 15.90 |
| warm-up | 3.99 | 31.4 | pipelines/compile_ms 946, pipelines/front_ms 897, render/thread/ms 76.42, pipelines/wait_ms 75.77, main/busy/ms 73.62, render/busy/ms 73.57, pipelines/driver_ms 48.42, stage/main/post_update 46.05, stage/render/render 22.93, render/graph/ms 20.14, encode//sum 20.05, bigspace/total/ms 18.31 |
| warm-up | 4.04 | 53.4 | pipelines/compile_ms 1726, pipelines/front_ms 1642, pipelines/wait_ms 97.39, pipelines/driver_ms 83.48, render/thread/ms 78.58, render/busy/ms 76.10, stage/render/prepare_assets 49.66, main/busy/ms 45.82, stage/main/post_update 16.38, stage/main/update 13.95, stage/render/render 13.81, stage/main/pre_update 11.24 |
| warm-up | 4.13 | 63.2 | pipelines/compile_ms 875, pipelines/front_ms 848, render/thread/ms 53.52, main/busy/ms 52.80, render/busy/ms 51.43, stage/main/update 43.02, stream/drive/ms 39.42, pipelines/driver_ms 26.60, stage/render/prepare_assets 24.37, stage/render/render 22.96, pipelines/wait_ms 22.64, render/graph/ms 20.45 |
| warm-up | 4.20 | 59.0 | pipelines/compile_ms 305, pipelines/front_ms 294, render/thread/ms 43.79, render/busy/ms 41.75, main/busy/ms 37.84, stage/main/update 24.81, stage/render/prepare_assets 24.28, stream/drive/ms 21.13, stage/render/render 12.74, pipelines/driver_ms 11.12, render/graph/ms 10.42, encode//sum 7.59 |
| warm-up | 4.25 | 48.2 | pipelines/compile_ms 229, pipelines/front_ms 220, render/thread/ms 75.01, render/busy/ms 72.49, stage/render/prepare_assets 48.25, main/busy/ms 39.05, stage/main/update 23.83, stream/drive/ms 19.51, stage/render/render 15.16, render/graph/ms 12.35, stage/main/post_update 9.78, stage/render/graph_submit 8.96 |
| warm-up | 4.32 | 74.9 | pipelines/compile_ms 217, pipelines/front_ms 209, render/thread/ms 87.27, render/busy/ms 85.04, main/busy/ms 75.71, stage/main/update 58.30, stage/render/prepare_assets 46.51, stream/drive/ms 29.90, stage/render/render 26.14, render/graph/ms 23.50, stage/render/graph_submit 19.57, stage/main/post_update 11.19 |
| warm-up | 4.39 | 73.9 | pipelines/compile_ms 1548, pipelines/front_ms 1505, main/busy/ms 111, stage/main/update 91.39, render/thread/ms 87.02, render/busy/ms 85.48, stream/drive/ms 52.55, stage/render/prepare_assets 50.53, pipelines/driver_ms 43.31, stage/render/render 23.93, render/graph/ms 21.61, stage/render/graph_submit 18.14 |
| warm-up | 4.49 | 94.3 | pipelines/compile_ms 1341, pipelines/front_ms 1305, main/busy/ms 89.98, render/thread/ms 85.81, render/busy/ms 83.21, stage/main/update 64.66, stage/render/prepare_assets 54.82, stream/drive/ms 43.84, pipelines/driver_ms 36.30, stage/render/render 14.70, stage/main/post_update 13.02, render/graph/ms 11.66 |
| warm-up | 4.57 | 81.3 | pipelines/compile_ms 843, pipelines/front_ms 814, pipelines/wait_ms 122, render/thread/ms 99.26, render/busy/ms 96.06, main/busy/ms 66.91, stage/render/prepare_assets 64.22, stage/main/update 33.77, pipelines/driver_ms 28.66, stage/main/post_update 21.92, stage/render/render 15.64, stream/drive/ms 13.81 |
| warm-up | 4.65 | 79.0 | pipelines/compile_ms 909, pipelines/front_ms 877, pipelines/wait_ms 202, render/thread/ms 109, render/busy/ms 107, stage/render/prepare_assets 64.36, main/busy/ms 60.88, pipelines/driver_ms 32.10, stage/main/update 29.35, stage/main/post_update 19.18, stage/render/prepare_resources 14.25, stream/drive/ms 12.39 |
| warm-up | 4.69 | 44.7 | pipelines/compile_ms 1289, pipelines/front_ms 1233, pipelines/wait_ms 121, render/thread/ms 95.70, render/busy/ms 93.93, pipelines/driver_ms 56.43, main/busy/ms 50.25, stage/render/prepare_assets 36.00, stage/render/render 25.15, stage/main/update 23.33, render/graph/ms 22.91, stage/render/graph_submit 20.00 |
| warm-up | 4.72 | 26.1 | pipelines/compile_ms 1830, pipelines/front_ms 1761, pipelines/wait_ms 135, pipelines/driver_ms 69.34, render/thread/ms 61.68, render/busy/ms 60.03, main/busy/ms 49.42, stage/main/update 28.03, stage/render/render 24.92, render/graph/ms 22.87, stage/render/graph_submit 19.91, encode/submit 15.97 |
| warm-up | 4.77 | 48.9 | pipelines/compile_ms 1229, pipelines/front_ms 1191, pipelines/wait_ms 126, render/thread/ms 64.52, render/busy/ms 62.96, pipelines/driver_ms 37.71, main/busy/ms 37.25, stage/render/prepare_assets 21.95, stage/main/update 18.11, stage/render/prepare_resources 13.48, stage/main/post_update 13.04, stage/render/render 8.97 |
| warm-up | 4.81 | 42.7 | pipelines/compile_ms 820, pipelines/front_ms 795, render/thread/ms 80.71, render/busy/ms 79.24, main/busy/ms 54.32, pipelines/wait_ms 46.77, stage/render/prepare_assets 28.03, stage/main/post_update 25.99, pipelines/driver_ms 25.25, stage/main/update 22.18, stage/render/prepare_resources 14.70, stage/render/prepare_write_phase_buffers 10.16 |
| warm-up | 4.88 | 69.8 | pipelines/compile_ms 603, pipelines/front_ms 587, pipelines/wait_ms 176, main/busy/ms 143, stage/main/update 116, stream/drive/ms 99.18, render/thread/ms 98.79, render/busy/ms 97.39, stage/render/prepare_assets 26.94, stage/render/render 26.07, render/graph/ms 23.98, stage/render/prepare_write_phase_buffers 21.94 |
| warm-up | 4.96 | 75.7 | pipelines/compile_ms 2271, pipelines/front_ms 2201, pipelines/wait_ms 288, main/busy/ms 123, stage/main/update 106, stream/drive/ms 99.76, render/thread/ms 81.15, render/busy/ms 79.66, pipelines/driver_ms 69.25, stage/render/render 26.63, stage/render/prepare_assets 25.88, render/graph/ms 24.50 |
| warm-up | 5.02 | 40.8 | pipelines/compile_ms 682, pipelines/front_ms 659, main/busy/ms 64.70, render/thread/ms 49.09, render/busy/ms 46.95, stage/main/update 30.85, stage/render/render 26.09, pipelines/driver_ms 23.15, render/graph/ms 23.12, stage/main/post_update 19.20, stage/render/graph_submit 13.76, stage/render/prepare_assets 8.62 |
| warm-up | 5.09 | 76.4 | pipelines/compile_ms 914, pipelines/front_ms 887, main/busy/ms 82.09, render/thread/ms 79.58, render/busy/ms 77.48, pipelines/wait_ms 57.51, stage/main/update 55.30, stage/render/prepare_assets 33.03, pipelines/driver_ms 26.76, stage/render/render 22.96, render/graph/ms 19.71, stage/main/post_update 15.78 |
| warm-up | 5.14 | 42.4 | pipelines/compile_ms 923, pipelines/front_ms 898, pipelines/wait_ms 87.15, render/thread/ms 80.37, render/busy/ms 78.87, main/busy/ms 61.03, stage/render/prepare_assets 43.39, stage/main/update 35.75, pipelines/driver_ms 24.68, stream/drive/ms 12.66, stage/main/post_update 12.40, stage/render/render 11.04 |
| warm-up | 5.19 | 50.5 | pipelines/compile_ms 737, pipelines/front_ms 719, render/thread/ms 81.96, render/busy/ms 80.53, main/busy/ms 51.38, pipelines/wait_ms 46.19, stage/render/prepare_assets 36.54, stage/main/post_update 23.09, pipelines/driver_ms 18.09, stage/main/update 16.37, stage/render/prepare_write_phase_buffers 11.21, stage/render/render 9.87 |
| warm-up | 5.22 | 31.2 | pipelines/compile_ms 1102, pipelines/front_ms 1076, render/thread/ms 75.09, render/busy/ms 73.49, main/busy/ms 48.59, stage/render/prepare_assets 28.60, pipelines/driver_ms 26.94, stage/main/update 21.90, stage/main/post_update 20.25, pipelines/wait_ms 16.74, stage/render/prepare_write_phase_buffers 12.03, stage/render/render 10.61 |
| warm-up | 5.25 | 28.8 | pipelines/compile_ms 1126, pipelines/front_ms 1095, render/thread/ms 50.28, render/busy/ms 48.63, main/busy/ms 45.41, pipelines/driver_ms 30.80, stage/main/update 30.67, stage/render/render 17.99, stream/drive/ms 16.83, render/graph/ms 15.84, stage/render/graph_submit 13.11, stage/render/prepare_assets 12.64 |
| warm-up | 5.29 | 42.0 | pipelines/compile_ms 725, pipelines/front_ms 702, render/thread/ms 62.16, render/busy/ms 60.04, main/busy/ms 55.38, stage/main/update 27.06, stage/main/post_update 23.13, pipelines/driver_ms 22.15, stage/render/prepare_assets 20.23, stage/render/render 18.88, stream/drive/ms 16.74, render/graph/ms 16.17 |
| warm-up | 5.33 | 36.6 | pipelines/compile_ms 670, pipelines/front_ms 651, render/thread/ms 68.49, render/busy/ms 66.35, main/busy/ms 50.86, stage/main/post_update 27.49, stage/render/prepare_assets 22.69, pipelines/driver_ms 19.34, stage/main/update 18.33, stage/render/render 12.49, stage/render/prepare_resources 11.46, render/graph/ms 10.04 |
| warm-up | 5.36 | 32.4 | pipelines/compile_ms 764, pipelines/front_ms 740, render/thread/ms 52.79, main/busy/ms 52.08, render/busy/ms 51.16, stage/main/update 31.26, pipelines/driver_ms 23.58, stage/render/prepare_assets 18.50, stream/drive/ms 14.70, stage/main/post_update 13.84, stage/render/render 12.52, stream/spawn/ms 11.25 |
| warm-up | 5.41 | 53.1 | pipelines/compile_ms 904, pipelines/front_ms 873, render/thread/ms 56.85, render/busy/ms 55.16, main/busy/ms 49.68, pipelines/driver_ms 30.84, stage/render/prepare_assets 29.27, stage/main/update 28.31, stage/render/render 14.00, stream/drive/ms 13.65, stage/main/post_update 11.87, render/graph/ms 11.68 |
| warm-up | 5.46 | 46.3 | pipelines/compile_ms 1512, pipelines/front_ms 1469, render/thread/ms 67.86, render/busy/ms 66.25, pipelines/driver_ms 43.21, main/busy/ms 38.07, stage/render/prepare_assets 36.02, stage/main/update 16.01, stage/render/render 13.96, stage/main/post_update 12.49, render/graph/ms 11.75, stage/render/graph_submit 9.00 |
| warm-up | 5.58 | 34.5 | pipelines/compile_ms 378, pipelines/front_ms 369, main/busy/ms 33.09, render/thread/ms 32.51, render/busy/ms 31.03, stage/main/update 18.69, stage/render/prepare_assets 13.24, stream/drive/ms 11.45, stage/render/render 10.09, stage/main/post_update 9.05, pipelines/driver_ms 8.56, render/graph/ms 8.18 |
| warm-up | 5.63 | 28.1 | pipelines/compile_ms 177, pipelines/front_ms 174, render/thread/ms 42.24, render/busy/ms 40.94, main/busy/ms 27.66, stage/render/prepare_assets 20.48, stage/main/update 10.58, stage/main/post_update 10.18, stage/render/render 10.15, render/graph/ms 8.46, encode//sum 7.43, stage/render/graph_submit 5.45 |
| warm-up | 5.69 | 40.7 | pipelines/compile_ms 510, pipelines/front_ms 497, render/thread/ms 47.67, render/busy/ms 46.16, main/busy/ms 35.94, stage/render/prepare_assets 26.63, stage/main/update 19.01, pipelines/driver_ms 13.17, stage/render/render 12.62, stage/main/post_update 10.85, render/graph/ms 10.71, encode//sum 7.55 |
| warm-up | 5.76 | 50.6 | pipelines/compile_ms 190, pipelines/front_ms 183, render/thread/ms 67.35, render/busy/ms 65.31, main/busy/ms 30.55, stage/render/render 27.78, render/graph/ms 25.17, stage/render/graph_submit 19.73, stage/render/prepare_assets 16.94, stage/main/update 12.02, stage/main/post_update 11.58, encode/finish_wall 8.20 |
| warm-up | 5.79 | 27.4 | pipelines/compile_ms 307, pipelines/front_ms 295, render/thread/ms 71.43, render/busy/ms 69.47, main/busy/ms 31.39, stage/render/render 26.88, stage/render/prepare_assets 25.50, render/graph/ms 24.37, stage/render/graph_submit 18.51, pipelines/wait_ms 15.12, pipelines/lock_ms 14.04, pipelines/driver_ms 12.68 |
| warm-up | 5.84 | 52.2 | pipelines/compile_ms 301, pipelines/front_ms 290, render/thread/ms 62.41, render/busy/ms 60.96, main/busy/ms 33.69, stage/render/prepare_assets 32.89, stage/render/render 15.18, stage/main/update 14.14, render/graph/ms 13.06, pipelines/wait_ms 12.97, pipelines/lock_ms 12.08, pipelines/driver_ms 11.82 |
| warm-up | 5.88 | 35.8 | pipelines/compile_ms 236, pipelines/front_ms 228, render/thread/ms 72.21, render/busy/ms 70.69, main/busy/ms 48.03, stage/render/prepare_assets 32.41, stage/main/update 22.75, stage/main/post_update 18.12, stage/render/render 15.62, render/graph/ms 13.42, pipelines/driver_ms 8.00, stage/render/graph_submit 7.74 |
| warm-up | 5.93 | 52.7 | pipelines/compile_ms 408, pipelines/front_ms 397, render/thread/ms 78.84, render/busy/ms 77.56, stage/render/prepare_assets 45.85, main/busy/ms 45.06, stage/main/update 21.04, stage/main/post_update 17.41, stage/render/render 11.07, pipelines/driver_ms 10.97, render/graph/ms 9.27, stage/render/prepare_resources 7.11 |
| warm-up | 5.98 | 29.4 | pipelines/compile_ms 868, pipelines/front_ms 846, render/thread/ms 40.00, render/busy/ms 38.25, main/busy/ms 37.61, pipelines/driver_ms 22.15, stage/main/update 17.87, stage/main/post_update 14.23, stage/render/prepare_assets 14.01, stage/render/render 13.19, render/graph/ms 11.06, encode//sum 7.44 |
| warm-up | 6.00 | 26.0 | pipelines/compile_ms 294, pipelines/front_ms 286, render/thread/ms 45.47, render/busy/ms 43.86, main/busy/ms 41.62, stage/main/update 22.39, stage/render/prepare_assets 16.69, stage/main/post_update 13.46, stage/render/render 12.58, render/graph/ms 10.48, encode//sum 7.80, pipelines/driver_ms 7.68 |
| warm-up | 6.04 | 35.4 | pipelines/compile_ms 749, pipelines/front_ms 721, render/thread/ms 50.77, render/busy/ms 49.00, main/busy/ms 45.46, pipelines/driver_ms 27.77, stage/main/update 19.55, stage/render/render 18.20, stage/main/post_update 18.15, render/graph/ms 15.62, stage/render/prepare_assets 14.45, encode//sum 12.55 |
| warm-up | 6.10 | 46.0 | main/busy/ms 48.37, render/thread/ms 36.74, render/busy/ms 35.33, stage/main/update 24.00, stage/render/prepare_assets 17.83, pipelines/compile_ms 17.40, pipelines/front_ms 16.56, stage/main/post_update 15.61, stream/drive/ms 14.34, stage/render/render 9.11, encode//sum 7.45, render/graph/ms 7.25 |
| warm-up | 6.15 | 44.1 | pipelines/compile_ms 215, pipelines/front_ms 206, render/thread/ms 54.76, render/busy/ms 53.29, main/busy/ms 51.40, stage/main/update 24.02, stage/render/prepare_assets 23.48, stage/main/post_update 19.03, stage/render/render 15.57, render/graph/ms 13.55, encode//sum 9.98, stage/render/graph_submit 9.52 |
| warm-up | 6.20 | 27.1 | pipelines/compile_ms 444, pipelines/front_ms 431, render/thread/ms 38.11, render/busy/ms 36.81, main/busy/ms 32.98, stage/render/prepare_assets 15.25, stage/main/update 14.37, pipelines/driver_ms 13.02, stage/main/post_update 11.60, stage/render/render 11.13, render/graph/ms 9.36, encode//sum 6.90 |
| warm-up | 6.32 | 29.8 | pipelines/compile_ms 229, pipelines/front_ms 221, main/busy/ms 49.05, render/thread/ms 30.54, stage/main/update 29.42, render/busy/ms 29.06, stream/drive/ms 19.78, stage/render/render 12.72, stage/main/post_update 11.16, render/graph/ms 10.64, pipelines/driver_ms 8.23, encode//sum 7.78 |
| warm-up | 6.36 | 37.7 | pipelines/compile_ms 423, pipelines/front_ms 410, main/busy/ms 48.50, render/thread/ms 34.91, render/busy/ms 33.57, stage/main/update 29.46, stream/drive/ms 20.05, stage/render/prepare_assets 13.07, pipelines/driver_ms 13.06, stage/main/post_update 12.36, stage/render/render 10.63, render/graph/ms 8.63 |
| warm-up | 6.43 | 27.6 | pipelines/compile_ms 359, pipelines/front_ms 353, main/busy/ms 35.31, render/thread/ms 30.87, render/busy/ms 29.62, stage/main/update 18.07, stage/render/render 14.06, render/graph/ms 12.21, stage/main/post_update 11.16, stage/render/prepare_assets 9.98, encode//sum 9.44, stage/render/graph_submit 9.38 |
| warm-up | 6.64 | 28.2 | render/thread/ms 35.74, main/busy/ms 35.63, render/busy/ms 34.13, stage/render/render 16.54, stage/main/post_update 15.81, stage/main/update 15.62, render/graph/ms 14.17, encode//sum 11.41, stage/render/graph_submit 9.25, stage/render/prepare_assets 7.56, stage/render/camera_driver 4.69, encode/submit 4.69 |
| warm-up | 6.71 | 47.0 | render/thread/ms 46.54, main/busy/ms 46.33, render/busy/ms 44.71, stage/render/prepare_assets 25.08, stage/main/update 21.40, stage/main/post_update 17.69, stage/render/render 13.53, stream/drive/ms 11.68, render/graph/ms 11.29, encode//sum 9.19, stage/render/graph_submit 8.28, encode/submit 5.78 |
| warm-up | 7.09 | 31.9 | render/thread/ms 40.04, render/busy/ms 38.86, main/busy/ms 15.68, stage/render/render 11.40, render/graph/ms 9.87, stage/render/specialize 9.66, stage/main/post_update 8.66, encode//sum 8.33, stage/render/graph_submit 7.88, stage/render/queue 6.88, encode/submit 4.95, stage/main/update 4.67 |
| warm-up | 8.32 | 31.5 | render/thread/ms 34.78, render/busy/ms 33.90, main/busy/ms 15.53, stage/render/render 8.72, stage/main/post_update 8.57, stage/render/specialize 8.54, render/graph/ms 7.52, encode//sum 7.07, stage/render/queue 5.84, stage/render/graph_submit 5.47, stage/render/prepare_meshes 4.36, stage/main/update 4.15 |
| measure | 0.83 | 26.7 | render/thread/ms 33.95, render/busy/ms 31.58, main/busy/ms 14.73, stage/render/render 11.18, render/graph/ms 9.67, encode//sum 9.06, stage/main/post_update 8.30, stage/render/specialize 8.23, stage/render/graph_submit 7.77, stage/render/queue 5.67, encode/submit 4.99, stage/main/update 3.60 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=base-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 11%
- during the run: other processes 11% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 96%) · 2625 MHz (min 2625) · memory 8101 MHz · 98.1 W · 77 °C (max 80) · VRAM 6426 MiB · throttled 100% of samples

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
