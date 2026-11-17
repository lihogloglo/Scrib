# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); mw.matctrl.gate=on (command line: --set); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1565 frames over 15.0 s · **median 9.59 ms (104 fps)** · mean 9.59 · p95 10.95 · p99 11.47 (1% low 87 fps) · worst 26.36 ms · 4 spikes over 1.5× median · 12.0 stutters/min (work 4.0/min)
- warm-up: 279 frames over 8.4 s, worst 2554.4 ms
- hitches (> 25.0 ms): **1** measured, 53 in warm-up
- timed GPU passes: 8.42 ms of a 9.59 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.59 ms, limit 11 ms
- pass worst frame 26.36 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.810 | 2.822 | 2.913 | 1.00 | 0.063 |
| dlss_super_resolution | 2.012 | 2.021 | 2.043 | 1.00 | 0.181 |
| early prepass | 0.606 | 0.609 | 0.623 | 1.00 | 0.091 |
| atmosphere_luts | 0.319 | 0.320 | 0.341 | 1.00 | 0.003 |
| bloom | 0.314 | 0.315 | 0.331 | 1.00 | 0.060 |
| main_transmissive_pass_3d | 0.289 | 0.290 | 0.329 | 1.00 | 0.008 |
| water_velocity | 0.239 | 0.240 | 0.267 | 1.00 | 0.002 |
| water_sspr | 0.202 | 0.203 | 0.210 | 1.00 | 0.002 |
| ssao | 0.190 | 0.191 | 0.198 | 1.00 | 0.007 |
| clouds_trace | 0.152 | 0.153 | 0.191 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.144 | 0.145 | 0.152 | 1.00 | 0.001 |
| water_fft | 0.130 | 0.131 | 0.135 | 1.00 | 0.003 |
| uniform_allocation | 0.119 | 0.120 | 0.162 | 1.00 | 0.021 |
| clouds_composite | 0.112 | 0.112 | 0.114 | 1.00 | 0.002 |
| upscaling | 0.112 | 0.112 | 0.118 | 1.00 | 0.004 |
| bin_unpacking | 0.091 | 0.091 | 0.122 | 1.00 | 0.016 |
| tonemapping | 0.088 | 0.089 | 0.095 | 1.00 | 0.003 |
| clustering | 0.066 | 0.067 | 0.078 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.053 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.413 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.052 | 1.00 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.036 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.032 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.030 | 0.030 | 0.041 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.231 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.028 | 1.00 | 0.080 |
| water_snapshot | 0.025 | 0.025 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.047 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 1.00 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.014 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.657 | 10.072 | 25.28 | 13548.7 |
| main/busy/ms | 7.140 | 8.003 | 10.09 | 11173.9 |
| render/busy/ms | 6.958 | 8.122 | 24.70 | 10889.9 |
| stage/render/render | 4.751 | 5.758 | 9.02 | 7435.5 |
| stage/main/post_update | 4.184 | 4.895 | 6.34 | 6547.6 |
| render/graph/ms | 4.169 | 5.080 | 6.95 | 6524.6 |
| encode//sum | 3.860 | 5.308 | 11.41 | 6041.5 |
| stage/render/graph_submit | 3.196 | 4.078 | 5.92 | 5002.3 |
| encode/submit | 2.046 | 2.606 | 5.19 | 3202.8 |
| stage/main/update | 1.838 | 2.241 | 3.11 | 2875.7 |
| stage/render/prepare_views | 1.378 | 3.016 | 4.85 | 2157.0 |
| render/acquire/ms | 1.333 | 2.966 | 4.71 | 2086.8 |
| encode/finish_wall | 1.172 | 1.640 | 2.34 | 1834.6 |
| stage/render/camera_driver | 0.919 | 1.051 | 1.27 | 1438.2 |
| encode/pass:shadow_directional_light/sum | 0.737 | 1.294 | 1.95 | 1153.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.728 | 1.004 | 1.54 | 1139.6 |
| encode/pass:early prepass/sum | 0.715 | 0.999 | 1.43 | 1118.6 |
| stage/render/prepare_write_phase_buffers | 0.544 | 0.744 | 0.96 | 851.3 |
| stage/render/prepare_assets | 0.391 | 0.475 | 1.53 | 612.3 |
| stage/render/prepare_resources | 0.385 | 0.453 | 1.05 | 602.9 |
| bigspace/total/ms | 0.376 | 0.875 | 1.54 | 588.3 |
| render/present/ms | 0.366 | 0.440 | 2.86 | 572.0 |
| stage/main/pre_update | 0.348 | 0.465 | 0.89 | 544.3 |
| stage/main/first | 0.291 | 0.352 | 2.34 | 455.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.267 | 0.411 | 0.81 | 417.3 |
| terrain/update/ms | 0.228 | 0.292 | 0.65 | 356.6 |
| stage/render/prepare_bind_groups | 0.205 | 0.275 | 0.41 | 320.6 |
| stage/render/prepare_meshes | 0.205 | 0.256 | 2.28 | 320.5 |
| terrain/select/ms | 0.198 | 0.254 | 0.63 | 310.6 |
| stream/drive/ms | 0.188 | 0.255 | 0.59 | 294.7 |
| stage/main/fixed_main_loop | 0.185 | 0.370 | 1.00 | 289.6 |
| stage/render/prepare_batch_phases | 0.173 | 0.229 | 0.38 | 270.6 |
| stage/render/extract_commands | 0.169 | 0.203 | 0.56 | 264.4 |
| stage/render/create_views | 0.169 | 0.218 | 0.68 | 264.2 |
| far/select/ms | 0.164 | 0.226 | 0.51 | 256.5 |
| stream/tick/ms | 0.160 | 0.224 | 0.56 | 250.6 |
| controller/move/ms | 0.158 | 0.250 | 0.65 | 246.8 |
| stage/render/prepare_flush | 0.147 | 0.256 | 0.78 | 230.6 |
| controller/feed/ms | 0.142 | 0.184 | 0.56 | 221.6 |
| bigspace/high_precision/ms | 0.136 | 0.422 | 1.36 | 212.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.120 | 0.189 | 0.44 | 187.9 |
| stage/main/last | 0.116 | 0.154 | 0.49 | 181.3 |
| stage/render/queue | 0.097 | 0.123 | 6.24 | 151.4 |
| anim/evaluate/ms | 0.095 | 0.234 | 1.32 | 147.9 |
| bigspace/low_precision/ms | 0.091 | 0.199 | 1.16 | 142.4 |
| stage/render/specialize | 0.068 | 0.082 | 8.45 | 105.8 |
| anim/evaluate_cpu/ms | 0.061 | 0.078 | 0.44 | 96.0 |
| stage/render/prepare_collect_phase_buffers | 0.059 | 0.115 | 0.19 | 92.7 |
| encode/resolve_diagnostics/sum | 0.055 | 0.092 | 0.52 | 85.8 |
| physics/step/ms | 0.029 | 0.060 | 0.44 | 46.1 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.043 | 0.47 | 41.5 |
| weather/map/ms | 0.025 | 0.034 | 0.32 | 39.0 |
| encode/camera_driver_span_end/sum | 0.023 | 0.043 | 0.10 | 36.3 |
| stage/render/cleanup | 0.022 | 0.031 | 0.30 | 35.0 |
| stage/render/phase_sort | 0.018 | 0.033 | 0.07 | 28.7 |
| weather/eval/ms | 0.011 | 0.014 | 0.03 | 16.5 |
| script/update/ms | 0.007 | 0.009 | 0.36 | 10.4 |
| controller/avoid/ms | 0.006 | 0.008 | 0.03 | 9.2 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 8.0 |
| anim/apply/ms | 0.004 | 0.006 | 0.04 | 6.7 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 5.7 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 0.9 |
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
| crowd/c1/count | count | 122 | 123 | 122 |
| crowd/c1/joints/count | count | 4848 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1088 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.53 | 2.14 | 0.79 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.47 | 0.06 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.10 | 0.03 |
| encode/finish_critical | ms | 0.86 | 2.14 | 0.91 |
| encode/pass:early prepass/max | ms | 0.71 | 1.43 | 0.59 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.73 | 1.54 | 0.91 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.44 | 0.16 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 0.81 | 0.37 |
| encode/pass:shadow_directional_light/max | ms | 0.67 | 1.95 | 0.74 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.52 | 0.08 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.11 | 24.70 | 9.13 |
| gpu/timed/ms | ms | 9.11 | 10.39 | 9.13 |
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
| process/cpu_usage | % | 26.09 | 38.39 | 26.13 |
| process/mem_usage | GiB | 4.68 | 4.77 | 4.77 |
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
| skin/blocks_changed/count | count | 6 | 7 | 4 |
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
| system/cpu_usage | % | 31.10 | 40.46 | 35.07 |
| system/mem_usage | % | 58.09 | 58.33 | 58.33 |
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
| anim/events/count | count | 0.09 | 3 | 134 |
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
| encode//count | count | 26.56 | 29 | 41.6 k |
| encode/bind_walk/init_textures | count | 7165 | 14.0 k | 11.21 M |
| encode/bind_walk/init_textures_skipped | count | 39.5 k | 52.6 k | 61.79 M |
| encode/bind_walk/inits | count | 345 | 437 | 540.5 k |
| encode/bind_walk/inits_skipped | count | 71.00 | 83 | 111.1 k |
| encode/bind_walk/merge_textures | count | 7101 | 13.9 k | 11.11 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.39 M |
| encode/bind_walk/merges | count | 82.26 | 103 | 128.7 k |
| encode/bind_walk/merges_skipped | count | 52.75 | 58 | 82.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1565 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1565 |
| encode/pass:early prepass/count | count | 1 | 1 | 1565 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1565 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1565 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1565 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1565 |
| encode/queue/command_buffers | count | 36.56 | 39 | 57.2 k |
| encode/queue/submits | count | 2 | 2 | 3130 |
| encode/queue/write_buffer/bytes | bytes | 1.38 M | 3.65 M | 2.16 G |
| encode/queue/write_buffer/calls | count | 217 | 287 | 339.6 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 51.08 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 14.1 k |
| encode/queue/write_texture/bytes | bytes | 11.5 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1565 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 1 | 900 |
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
| warm-up | 2.55 | 2554.4 | main/busy/ms 2521, stage/main/post_update 2418, stage/main/update 95.66, surface/advance_cpu_ms 66.49, stage/main/pre_update 4.43, stage/main/last 1.16, surface/cell_upload_ms 0.77, stream/drive/ms 0.63, stage/main/first 0.54, stage/main/fixed_main_loop 0.38, stream/tick/ms 0.16, weather/map/ms 0.12 |
| warm-up | 3.31 | 750.8 | main/busy/ms 2667, stage/main/post_update 2441, render/thread/ms 717, render/busy/ms 547, stage/render/prepare_assets 496, stage/main/update 212, stage/render/prepare_views 165, render/acquire/ms 165, surface/advance_cpu_ms 66.49, stage/render/create_views 27.80, stage/render/render 15.77, render/graph/ms 9.11 |
| warm-up | 3.48 | 176.1 | render/thread/ms 875, render/busy/ms 613, stage/render/prepare_assets 547, main/busy/ms 172, stage/render/prepare_views 166, render/acquire/ms 165, stage/main/update 122, stage/render/render 112, render/present/ms 96.94, stage/main/post_update 37.18, stage/render/create_views 28.14, pipelines/compile_ms 26.47 |
| warm-up | 3.53 | 45.4 | pipelines/compile_ms 464, pipelines/front_ms 335, pipelines/wait_ms 234, render/thread/ms 195, main/busy/ms 128, pipelines/driver_ms 128, stage/render/render 127, render/busy/ms 101, render/present/ms 93.81, stage/main/update 84.81, stage/render/prepare_assets 51.82, pipelines/lock_ms 40.56 |
| warm-up | 3.72 | 193.3 | pipelines/wait_ms 4775, pipelines/compile_ms 2726, pipelines/front_ms 2467, pipelines/driver_ms 259, render/thread/ms 154, render/busy/ms 150, main/busy/ms 125, stage/render/prepare_assets 97.80, stage/main/update 86.29, pipelines/lock_ms 66.92, stage/render/render 44.06, render/graph/ms 40.03 |
| warm-up | 3.79 | 65.2 | pipelines/wait_ms 4591, pipelines/compile_ms 2639, pipelines/front_ms 2490, render/thread/ms 175, render/busy/ms 173, pipelines/driver_ms 148, stage/render/prepare_assets 105, main/busy/ms 75.00, stage/main/post_update 57.23, stage/render/prepare_resources 38.26, pipelines/lock_ms 28.54, encode//sum 23.64 |
| warm-up | 3.83 | 49.4 | pipelines/compile_ms 740, pipelines/front_ms 683, render/thread/ms 105, pipelines/wait_ms 103, render/busy/ms 103, main/busy/ms 66.64, pipelines/driver_ms 57.31, stage/main/post_update 49.42, stage/render/prepare_resources 37.71, stage/render/prepare_assets 33.66, stage/render/render 23.33, render/graph/ms 20.91 |
| warm-up | 3.89 | 32.2 | pipelines/compile_ms 547, pipelines/front_ms 518, render/thread/ms 48.41, main/busy/ms 41.72, render/busy/ms 41.71, pipelines/driver_ms 28.89, stage/main/post_update 25.76, stage/render/render 17.45, stage/render/prepare_resources 12.47, render/graph/ms 10.72, encode//sum 9.91, terrain/update/ms 8.64 |
| warm-up | 3.95 | 41.9 | pipelines/compile_ms 598, pipelines/front_ms 567, main/busy/ms 66.75, render/thread/ms 57.78, render/busy/ms 56.34, stage/main/update 47.52, stream/drive/ms 44.01, stage/render/prepare_assets 39.56, pipelines/driver_ms 31.22, stage/main/post_update 15.15, stage/render/render 11.24, terrain/update/ms 9.49 |
| warm-up | 3.99 | 45.5 | pipelines/compile_ms 960, pipelines/front_ms 931, render/thread/ms 67.90, render/busy/ms 66.75, main/busy/ms 60.14, stage/render/prepare_assets 52.75, stage/main/update 48.37, stream/drive/ms 43.96, pipelines/driver_ms 29.09, terrain/update/ms 9.49, stage/render/render 8.84, stage/main/post_update 7.88 |
| warm-up | 4.04 | 38.9 | pipelines/compile_ms 163, pipelines/front_ms 160, render/thread/ms 35.79, render/busy/ms 34.35, main/busy/ms 29.34, stage/render/prepare_assets 19.59, stage/main/update 18.56, stream/drive/ms 15.07, stage/render/render 10.56, render/graph/ms 8.61, encode//sum 8.12, stage/main/post_update 6.68 |
| warm-up | 4.07 | 29.3 | pipelines/compile_ms 163, pipelines/front_ms 160, render/thread/ms 48.60, render/busy/ms 47.31, stage/render/prepare_assets 33.40, main/busy/ms 25.01, stage/main/update 9.80, stage/main/post_update 8.71, stage/render/render 7.99, encode//sum 6.48, render/graph/ms 6.22, stage/render/graph_submit 4.21 |
| warm-up | 4.15 | 78.4 | pipelines/compile_ms 195, pipelines/front_ms 184, render/thread/ms 89.83, render/busy/ms 87.29, stage/render/prepare_assets 54.59, main/busy/ms 48.36, stage/main/update 30.96, stage/render/render 25.28, render/graph/ms 22.39, stage/render/graph_submit 20.13, stream/drive/ms 19.68, pipelines/driver_ms 10.31 |
| warm-up | 4.20 | 47.0 | pipelines/compile_ms 324, pipelines/front_ms 308, render/thread/ms 99.57, render/busy/ms 96.43, main/busy/ms 74.74, stage/main/update 56.39, stage/render/prepare_assets 56.24, stage/render/render 30.90, render/graph/ms 26.75, stage/render/graph_submit 23.42, stream/drive/ms 19.73, pipelines/driver_ms 16.61 |
| warm-up | 4.33 | 134.4 | pipelines/compile_ms 1037, pipelines/front_ms 999, main/busy/ms 136, render/thread/ms 124, render/busy/ms 122, stage/main/update 111, stream/drive/ms 58.62, stage/render/prepare_assets 56.61, pipelines/driver_ms 38.82, stage/render/prepare_resources 33.58, stage/render/render 17.58, pipelines/wait_ms 17.47 |
| warm-up | 4.41 | 79.5 | pipelines/compile_ms 1144, pipelines/front_ms 1103, render/thread/ms 142, render/busy/ms 140, main/busy/ms 122, pipelines/wait_ms 91.30, stage/main/update 89.55, stage/render/prepare_assets 74.88, stream/drive/ms 59.96, pipelines/driver_ms 41.48, pipelines/lock_ms 40.63, stage/render/prepare_resources 33.17 |
| warm-up | 4.49 | 74.2 | pipelines/compile_ms 278, pipelines/front_ms 267, render/thread/ms 112, render/busy/ms 110, pipelines/wait_ms 94.92, stage/render/prepare_assets 82.09, main/busy/ms 75.87, stage/main/update 41.04, pipelines/lock_ms 27.68, stage/main/post_update 21.64, stream/drive/ms 12.37, stage/render/render 12.14 |
| warm-up | 4.54 | 54.4 | pipelines/compile_ms 1225, pipelines/front_ms 1190, pipelines/wait_ms 263, render/thread/ms 102, render/busy/ms 100, stage/render/prepare_assets 74.35, main/busy/ms 61.96, stage/main/update 35.99, pipelines/driver_ms 35.80, stage/main/post_update 15.41, stream/drive/ms 11.07, stage/render/render 9.75 |
| warm-up | 4.59 | 25.2 | pipelines/compile_ms 685, pipelines/front_ms 665, pipelines/wait_ms 74.21, render/thread/ms 39.77, render/busy/ms 38.40, main/busy/ms 27.52, pipelines/driver_ms 19.62, stage/main/update 15.17, stage/render/render 10.74, stage/render/prepare_assets 10.05, render/graph/ms 8.98, stage/main/post_update 7.43 |
| warm-up | 4.66 | 35.8 | pipelines/compile_ms 675, pipelines/front_ms 653, main/busy/ms 78.55, stage/main/update 65.02, stream/drive/ms 58.85, pipelines/wait_ms 45.23, render/thread/ms 42.81, render/busy/ms 39.27, stage/render/render 24.41, pipelines/driver_ms 21.58, render/graph/ms 20.35, stage/render/graph_submit 16.84 |
| warm-up | 4.72 | 64.4 | pipelines/compile_ms 488, pipelines/front_ms 473, main/busy/ms 121, stage/main/update 105, stream/drive/ms 99.25, render/thread/ms 47.90, render/busy/ms 44.65, pipelines/wait_ms 26.74, stage/render/render 22.75, render/graph/ms 18.99, stage/render/prepare_assets 17.10, stage/render/graph_submit 16.27 |
| warm-up | 4.79 | 69.0 | pipelines/compile_ms 808, pipelines/front_ms 781, main/busy/ms 73.51, stage/main/update 55.49, stream/drive/ms 46.06, render/thread/ms 45.35, render/busy/ms 43.83, stage/render/prepare_assets 27.04, pipelines/driver_ms 26.44, pipelines/wait_ms 25.36, stage/main/post_update 12.37, encode//sum 10.92 |
| warm-up | 4.83 | 36.6 | pipelines/compile_ms 1269, pipelines/front_ms 1218, render/thread/ms 57.77, render/busy/ms 56.36, pipelines/driver_ms 50.85, main/busy/ms 45.36, stage/render/prepare_assets 23.33, stage/main/post_update 18.62, stage/main/update 14.57, stage/render/render 10.09, bigspace/total/ms 9.47, stage/render/prepare_resources 8.70 |
| warm-up | 4.92 | 73.1 | pipelines/compile_ms 582, pipelines/front_ms 566, main/busy/ms 67.32, render/thread/ms 55.42, render/busy/ms 53.51, stage/main/update 48.64, stage/render/prepare_assets 22.81, pipelines/driver_ms 15.70, stream/drive/ms 13.25, stage/render/render 12.66, render/graph/ms 10.26, stage/main/post_update 9.87 |
| warm-up | 4.96 | 38.9 | pipelines/compile_ms 1639, pipelines/front_ms 1600, render/thread/ms 65.42, render/busy/ms 64.05, main/busy/ms 44.31, pipelines/wait_ms 39.81, pipelines/driver_ms 38.76, stage/render/prepare_assets 34.00, stage/main/update 28.27, stage/render/render 9.89, stage/main/post_update 9.31, render/graph/ms 8.02 |
| warm-up | 4.99 | 32.2 | pipelines/compile_ms 1310, pipelines/front_ms 1279, render/thread/ms 58.19, render/busy/ms 56.50, main/busy/ms 44.41, pipelines/wait_ms 40.06, pipelines/driver_ms 31.13, stage/main/update 19.35, stage/main/post_update 18.20, stage/render/render 18.17, stage/render/prepare_assets 16.60, render/graph/ms 16.36 |
| warm-up | 5.03 | 31.9 | pipelines/compile_ms 723, pipelines/front_ms 697, render/thread/ms 56.36, main/busy/ms 55.62, render/busy/ms 54.58, stage/main/update 32.63, pipelines/driver_ms 26.26, stage/render/render 19.62, render/graph/ms 17.35, stage/main/post_update 17.19, stream/drive/ms 16.93, stage/render/prepare_assets 14.63 |
| warm-up | 5.09 | 35.3 | pipelines/compile_ms 329, pipelines/front_ms 315, render/thread/ms 50.38, render/busy/ms 46.59, main/busy/ms 41.48, stage/main/update 25.38, pipelines/driver_ms 14.51, stage/render/render 12.60, render/graph/ms 10.84, stage/main/post_update 9.30, stage/render/prepare_assets 9.19, stage/render/graph_submit 8.03 |
| warm-up | 5.12 | 37.6 | pipelines/compile_ms 190, pipelines/front_ms 183, main/busy/ms 63.33, render/thread/ms 63.11, render/busy/ms 59.82, stage/main/update 37.25, stage/main/post_update 19.52, stage/render/render 12.31, stage/render/prepare_assets 10.96, render/graph/ms 10.08, stage/render/prepare_resources 7.93, encode//sum 7.72 |
| warm-up | 5.17 | 50.4 | pipelines/compile_ms 775, pipelines/front_ms 751, render/thread/ms 66.23, render/busy/ms 64.14, main/busy/ms 57.58, stage/main/update 26.10, pipelines/driver_ms 23.88, stage/main/post_update 22.92, stage/render/prepare_assets 21.37, stage/render/render 11.50, render/graph/ms 8.78, encode//sum 7.68 |
| warm-up | 5.21 | 36.4 | pipelines/compile_ms 1512, pipelines/front_ms 1469, render/thread/ms 63.44, render/busy/ms 61.53, pipelines/driver_ms 43.56, main/busy/ms 37.68, stage/render/prepare_assets 28.67, stage/main/post_update 14.82, stage/main/update 14.13, stage/render/render 13.46, render/graph/ms 11.17, stage/render/graph_submit 7.93 |
| warm-up | 5.24 | 35.3 | pipelines/compile_ms 821, pipelines/front_ms 797, render/thread/ms 54.41, render/busy/ms 52.90, main/busy/ms 35.57, stage/render/prepare_assets 29.51, pipelines/driver_ms 23.74, stage/main/update 18.67, stage/render/render 12.53, render/graph/ms 10.63, stage/main/post_update 10.16, stage/render/graph_submit 7.88 |
| warm-up | 5.28 | 36.9 | pipelines/compile_ms 317, pipelines/front_ms 308, render/thread/ms 54.84, render/busy/ms 53.51, stage/render/prepare_assets 33.93, main/busy/ms 31.78, stage/main/update 16.91, stage/render/render 9.52, stage/main/post_update 9.17, pipelines/driver_ms 8.89, render/graph/ms 7.71, encode//sum 7.14 |
| warm-up | 5.38 | 32.7 | pipelines/compile_ms 202, pipelines/front_ms 197, render/thread/ms 50.13, render/busy/ms 48.53, main/busy/ms 43.34, stage/main/update 20.83, stage/main/post_update 15.80, stage/render/prepare_assets 15.32, stream/drive/ms 11.77, stage/render/render 11.39, render/graph/ms 9.43, stage/render/prepare_resources 8.06 |
| warm-up | 5.42 | 26.0 | pipelines/compile_ms 1359, pipelines/front_ms 1330, render/thread/ms 42.48, render/busy/ms 41.23, main/busy/ms 29.10, pipelines/driver_ms 28.57, stage/render/prepare_assets 20.29, stage/main/post_update 11.71, stage/main/update 11.00, stage/render/render 10.40, render/graph/ms 8.67, encode//sum 7.35 |
| warm-up | 5.49 | 26.4 | pipelines/compile_ms 172, pipelines/front_ms 168, render/thread/ms 46.31, render/busy/ms 45.01, main/busy/ms 31.31, stage/render/prepare_assets 27.40, stage/main/update 15.40, stage/render/render 10.73, stage/main/post_update 9.49, render/graph/ms 8.91, encode//sum 8.82, stage/render/graph_submit 6.62 |
| warm-up | 5.53 | 36.1 | pipelines/compile_ms 84.75, pipelines/front_ms 79.62, render/thread/ms 54.06, render/busy/ms 52.83, stage/render/prepare_assets 31.25, main/busy/ms 26.89, stage/main/update 13.01, stage/render/render 9.95, render/graph/ms 8.28, stage/main/post_update 8.20, encode//sum 7.56, stage/render/graph_submit 6.03 |
| warm-up | 5.58 | 40.7 | render/thread/ms 48.58, render/busy/ms 46.62, main/busy/ms 33.07, stage/render/prepare_assets 22.69, stage/render/render 14.35, stage/main/post_update 13.34, stage/main/update 12.15, render/graph/ms 11.73, stage/render/graph_submit 8.26, encode//sum 8.09, encode/submit 5.62, stream/drive/ms 5.00 |
| warm-up | 5.61 | 33.6 | render/thread/ms 58.07, render/busy/ms 56.84, stage/render/prepare_assets 36.36, main/busy/ms 35.48, stage/main/update 13.97, stage/main/post_update 12.80, pipelines/compile_ms 12.26, pipelines/front_ms 11.60, stage/render/render 9.67, render/graph/ms 7.86, encode//sum 7.51, stage/render/graph_submit 5.33 |
| warm-up | 5.65 | 35.5 | pipelines/compile_ms 175, pipelines/front_ms 169, render/thread/ms 53.46, render/busy/ms 52.46, stage/render/prepare_assets 34.82, main/busy/ms 31.67, stage/main/update 13.80, stage/main/post_update 11.05, stage/render/render 8.16, encode//sum 7.22, render/graph/ms 6.72, pipelines/driver_ms 6.24 |
| warm-up | 5.70 | 32.3 | pipelines/compile_ms 705, pipelines/front_ms 690, render/thread/ms 46.39, render/busy/ms 44.74, main/busy/ms 29.30, stage/render/prepare_assets 26.17, pipelines/driver_ms 14.67, stage/render/render 12.70, stage/main/update 11.95, stage/main/post_update 10.95, render/graph/ms 10.50, stage/render/graph_submit 8.45 |
| warm-up | 5.74 | 34.0 | pipelines/compile_ms 182, pipelines/front_ms 174, render/thread/ms 58.58, render/busy/ms 56.28, main/busy/ms 48.31, stage/render/render 28.18, stage/main/update 25.23, render/graph/ms 23.21, stage/render/prepare_assets 19.43, stage/render/graph_submit 17.97, stage/main/post_update 13.45, encode/finish_wall 10.35 |
| warm-up | 5.78 | 38.3 | pipelines/compile_ms 175, pipelines/front_ms 167, render/thread/ms 57.21, render/busy/ms 55.44, main/busy/ms 53.12, stage/render/render 26.15, stage/main/update 25.30, render/graph/ms 21.64, stage/main/post_update 16.55, stage/render/graph_submit 14.76, stage/render/prepare_assets 14.63, encode//sum 9.35 |
| warm-up | 5.81 | 36.1 | render/thread/ms 55.47, render/busy/ms 54.05, main/busy/ms 45.79, stage/main/post_update 24.32, stage/render/prepare_assets 19.39, stage/render/render 19.08, render/graph/ms 16.94, stage/main/update 13.01, stage/render/graph_submit 11.76, encode//sum 10.39, encode/submit 7.13, stage/render/camera_driver 4.96 |
| warm-up | 5.85 | 35.0 | pipelines/compile_ms 171, pipelines/front_ms 167, main/busy/ms 52.74, render/thread/ms 52.58, render/busy/ms 50.91, stage/main/update 24.34, stage/main/post_update 21.55, stage/render/prepare_assets 19.69, stage/render/render 19.22, render/graph/ms 16.86, stream/drive/ms 14.69, stage/render/graph_submit 12.00 |
| warm-up | 5.89 | 41.4 | pipelines/compile_ms 570, pipelines/front_ms 557, render/thread/ms 52.62, main/busy/ms 52.54, render/busy/ms 50.92, stage/main/update 29.11, stage/render/prepare_assets 22.59, stage/main/post_update 16.62, stage/render/render 15.37, stream/drive/ms 14.69, render/graph/ms 13.02, pipelines/driver_ms 12.88 |
| warm-up | 5.92 | 26.6 | pipelines/compile_ms 1190, pipelines/front_ms 1164, render/thread/ms 50.24, render/busy/ms 48.15, main/busy/ms 46.00, pipelines/driver_ms 25.43, stage/main/update 20.95, stage/main/post_update 17.97, stage/render/render 17.69, stage/render/prepare_assets 17.29, render/graph/ms 15.12, pipelines/wait_ms 13.00 |
| warm-up | 6.10 | 34.5 | pipelines/compile_ms 951, pipelines/front_ms 920, main/busy/ms 48.15, render/thread/ms 34.69, render/busy/ms 33.47, pipelines/driver_ms 31.36, stage/main/update 30.02, stream/drive/ms 21.45, stage/render/render 11.50, stage/main/post_update 11.05, stage/render/prepare_assets 9.78, render/graph/ms 9.69 |
| warm-up | 6.19 | 29.5 | main/busy/ms 35.88, render/thread/ms 26.98, render/busy/ms 25.93, stage/main/update 17.30, stage/main/post_update 13.13, stage/render/render 11.13, stage/render/prepare_assets 9.85, render/graph/ms 9.67, encode//sum 8.16, stage/render/graph_submit 7.01, encode/submit 4.18, stream/drive/ms 3.31 |
| warm-up | 6.26 | 26.3 | render/thread/ms 36.21, render/busy/ms 34.99, main/busy/ms 21.90, stage/render/prepare_assets 17.36, stage/render/render 10.36, stage/main/post_update 9.98, render/graph/ms 8.75, stage/main/update 7.78, encode//sum 7.35, stage/render/graph_submit 6.28, encode/submit 4.19, stream/drive/ms 2.76 |
| warm-up | 6.46 | 45.5 | render/thread/ms 46.26, render/busy/ms 44.75, main/busy/ms 42.22, stage/render/prepare_assets 26.19, stage/main/update 19.96, stage/main/post_update 15.66, stage/render/render 12.22, stream/drive/ms 10.51, render/graph/ms 9.92, encode//sum 8.47, stage/render/graph_submit 6.62, encode/finish_wall 3.41 |
| warm-up | 6.80 | 31.2 | render/thread/ms 37.72, render/busy/ms 36.60, main/busy/ms 15.24, stage/render/render 10.81, stage/render/specialize 9.39, render/graph/ms 9.20, stage/main/post_update 8.65, encode//sum 7.51, stage/render/graph_submit 7.26, stage/render/queue 6.72, encode/submit 4.35, stage/main/update 4.03 |
| warm-up | 8.02 | 29.4 | render/thread/ms 36.16, render/busy/ms 33.17, main/busy/ms 15.09, stage/render/render 9.76, stage/render/specialize 8.48, render/graph/ms 8.43, stage/main/post_update 7.97, stage/render/graph_submit 6.53, stage/render/queue 6.07, encode//sum 5.40, stage/main/update 4.77, encode/submit 3.87 |
| measure | 0.83 | 26.4 | render/thread/ms 33.87, render/busy/ms 30.73, main/busy/ms 13.73, stage/render/render 9.00, stage/render/specialize 8.51, render/graph/ms 7.81, stage/main/post_update 7.42, stage/render/queue 6.33, stage/render/graph_submit 5.91, encode//sum 5.66, encode/submit 3.54, stage/main/update 3.47 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set mw.matctrl.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=matctrl-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 7%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 98.1 W · 77 °C (max 79) · VRAM 6426 MiB · throttled 100% of samples

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
