# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1541 frames over 15.0 s · **median 9.76 ms (102 fps)** · mean 9.74 · p95 11.11 · p99 11.65 (1% low 86 fps) · worst 27.01 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 279 frames over 8.6 s, worst 2465.4 ms
- hitches (> 25.0 ms): **1** measured, 60 in warm-up
- timed GPU passes: 8.61 ms of a 9.74 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.76 ms, limit 11 ms
- pass worst frame 27.01 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.960 | 2.966 | 3.009 | 1.00 | 0.075 |
| dlss_super_resolution | 2.021 | 2.025 | 2.050 | 1.00 | 0.182 |
| early prepass | 0.609 | 0.611 | 0.623 | 1.00 | 0.096 |
| atmosphere_luts | 0.323 | 0.323 | 0.343 | 1.00 | 0.003 |
| bloom | 0.314 | 0.315 | 0.332 | 1.00 | 0.061 |
| main_transmissive_pass_3d | 0.311 | 0.312 | 0.334 | 1.00 | 0.009 |
| water_velocity | 0.240 | 0.240 | 0.265 | 1.00 | 0.002 |
| water_sspr | 0.208 | 0.208 | 0.210 | 1.00 | 0.002 |
| ssao | 0.191 | 0.192 | 0.198 | 1.00 | 0.007 |
| clouds_trace | 0.149 | 0.150 | 0.170 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.147 | 0.153 | 1.00 | 0.001 |
| water_fft | 0.131 | 0.131 | 0.135 | 1.00 | 0.003 |
| uniform_allocation | 0.119 | 0.119 | 0.160 | 1.00 | 0.023 |
| upscaling | 0.114 | 0.114 | 0.120 | 1.00 | 0.004 |
| clouds_composite | 0.112 | 0.112 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.090 | 0.090 | 0.122 | 1.00 | 0.016 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.414 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.028 | 0.224 | 0.230 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.025 | 0.027 | 1.00 | 0.083 |
| water_snapshot | 0.024 | 0.025 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.043 | 0.046 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.024 | 1.00 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.804 | 10.177 | 25.75 | 13566.6 |
| main/busy/ms | 7.205 | 8.027 | 10.48 | 11102.7 |
| render/busy/ms | 7.048 | 8.061 | 25.16 | 10860.4 |
| stage/render/render | 4.734 | 5.573 | 12.01 | 7295.2 |
| stage/main/post_update | 4.307 | 5.033 | 6.68 | 6636.6 |
| render/graph/ms | 4.151 | 4.948 | 7.00 | 6396.4 |
| encode//sum | 4.047 | 5.370 | 8.04 | 6235.8 |
| stage/render/graph_submit | 3.147 | 3.918 | 5.95 | 4849.7 |
| encode/submit | 1.956 | 2.385 | 7.54 | 3014.0 |
| stage/main/update | 1.786 | 2.209 | 3.69 | 2752.3 |
| stage/render/prepare_views | 1.439 | 2.965 | 4.31 | 2217.6 |
| render/acquire/ms | 1.395 | 2.912 | 4.26 | 2149.3 |
| encode/finish_wall | 1.220 | 1.694 | 2.46 | 1879.3 |
| stage/render/camera_driver | 0.949 | 1.072 | 1.37 | 1462.5 |
| encode/pass:shadow_directional_light/sum | 0.782 | 1.309 | 2.32 | 1205.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.731 | 1.009 | 1.33 | 1126.1 |
| encode/pass:early prepass/sum | 0.726 | 0.985 | 1.33 | 1119.1 |
| stage/render/prepare_write_phase_buffers | 0.550 | 0.763 | 1.11 | 847.0 |
| stage/render/prepare_assets | 0.449 | 0.566 | 1.56 | 692.6 |
| stage/render/prepare_resources | 0.390 | 0.455 | 0.62 | 600.7 |
| render/present/ms | 0.361 | 0.429 | 4.72 | 556.8 |
| bigspace/total/ms | 0.358 | 0.822 | 1.67 | 551.6 |
| stage/main/pre_update | 0.344 | 0.457 | 0.79 | 530.5 |
| stage/main/first | 0.293 | 0.355 | 1.34 | 451.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.244 | 0.345 | 0.51 | 375.8 |
| terrain/update/ms | 0.233 | 0.298 | 0.61 | 359.4 |
| stage/render/prepare_bind_groups | 0.210 | 0.292 | 0.42 | 324.3 |
| terrain/select/ms | 0.203 | 0.258 | 0.58 | 313.0 |
| stage/render/prepare_meshes | 0.199 | 0.244 | 2.19 | 306.1 |
| stream/drive/ms | 0.188 | 0.251 | 0.68 | 289.3 |
| stage/main/fixed_main_loop | 0.180 | 0.334 | 0.93 | 277.8 |
| stage/render/prepare_batch_phases | 0.176 | 0.235 | 0.38 | 271.1 |
| stage/render/extract_commands | 0.174 | 0.207 | 0.51 | 268.4 |
| stage/render/create_views | 0.169 | 0.212 | 0.71 | 260.0 |
| far/select/ms | 0.164 | 0.212 | 0.62 | 252.5 |
| stream/tick/ms | 0.159 | 0.215 | 0.63 | 244.8 |
| controller/feed/ms | 0.153 | 0.197 | 0.61 | 235.5 |
| stage/render/prepare_flush | 0.153 | 0.271 | 0.53 | 235.2 |
| bigspace/high_precision/ms | 0.136 | 0.385 | 1.40 | 209.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.124 | 0.192 | 0.43 | 191.4 |
| stage/main/last | 0.117 | 0.157 | 0.61 | 180.0 |
| controller/move/ms | 0.115 | 0.184 | 0.65 | 177.2 |
| stage/render/queue | 0.105 | 0.133 | 5.57 | 161.9 |
| anim/evaluate/ms | 0.086 | 0.218 | 1.14 | 133.1 |
| stage/render/specialize | 0.085 | 0.106 | 8.24 | 131.7 |
| bigspace/low_precision/ms | 0.080 | 0.170 | 1.41 | 122.6 |
| anim/evaluate_cpu/ms | 0.061 | 0.076 | 0.50 | 94.5 |
| encode/resolve_diagnostics/sum | 0.059 | 0.091 | 0.28 | 90.5 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.116 | 0.25 | 89.6 |
| encode/camera_driver_span_begin/sum | 0.028 | 0.040 | 0.11 | 43.0 |
| physics/step/ms | 0.025 | 0.051 | 0.49 | 39.0 |
| weather/map/ms | 0.024 | 0.033 | 0.09 | 37.4 |
| encode/camera_driver_span_end/sum | 0.024 | 0.038 | 0.10 | 37.3 |
| stage/render/cleanup | 0.022 | 0.032 | 0.18 | 34.6 |
| stage/render/phase_sort | 0.017 | 0.031 | 0.16 | 26.7 |
| weather/eval/ms | 0.011 | 0.014 | 0.04 | 16.4 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 11.0 |
| controller/avoid/ms | 0.006 | 0.007 | 0.40 | 9.4 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 8.3 |
| anim/apply/ms | 0.005 | 0.006 | 0.08 | 7.1 |
| script/ui/ms | 0.004 | 0.005 | 0.13 | 6.0 |
| controller/write/ms | 0.002 | 0.002 | 0.03 | 3.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.02 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 0.5 |
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
| encode//max | ms | 0.55 | 1.75 | 0.67 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.11 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.10 | 0.02 |
| encode/finish_critical | ms | 0.87 | 2.32 | 0.91 |
| encode/pass:early prepass/max | ms | 0.72 | 1.33 | 0.80 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.74 | 1.33 | 0.91 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.43 | 0.26 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.24 | 0.51 | 0.49 |
| encode/pass:shadow_directional_light/max | ms | 0.72 | 2.32 | 0.64 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.28 | 0.07 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.26 | 25.16 | 9.27 |
| gpu/timed/ms | ms | 9.26 | 10.81 | 9.27 |
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
| process/cpu_usage | % | 26.44 | 36.06 | 31.90 |
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
| skin/blocks_changed/count | count | 4 | 5 | 3 |
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
| system/cpu_usage | % | 30.55 | 37.53 | 34.68 |
| system/mem_usage | % | 54.14 | 54.35 | 54.35 |
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
| anim/events/count | count | 0.08 | 3 | 128 |
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
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 26.57 | 29 | 40.9 k |
| encode/bind_walk/init_textures | count | 7165 | 14.0 k | 11.04 M |
| encode/bind_walk/init_textures_skipped | count | 35.6 k | 48.7 k | 54.86 M |
| encode/bind_walk/inits | count | 345 | 437 | 532.2 k |
| encode/bind_walk/inits_skipped | count | 67.00 | 79 | 103.2 k |
| encode/bind_walk/merge_textures | count | 7101 | 13.9 k | 10.94 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.29 M |
| encode/bind_walk/merges | count | 82.26 | 103 | 126.8 k |
| encode/bind_walk/merges_skipped | count | 50.75 | 56 | 78.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1541 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1541 |
| encode/pass:early prepass/count | count | 1 | 1 | 1541 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1541 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1541 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1541 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1541 |
| encode/queue/command_buffers | count | 36.57 | 39 | 56.4 k |
| encode/queue/submits | count | 2 | 2 | 3082 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.65 M | 2.22 G |
| encode/queue/write_buffer/calls | count | 217 | 287 | 334.9 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.30 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.9 k |
| encode/queue/write_texture/bytes | bytes | 11.7 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1541 |
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

## Hitches (> 25.0 ms; slowest 61, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2465.4 | main/busy/ms 2435, stage/main/post_update 2330, stage/main/update 96.52, surface/advance_cpu_ms 66.70, stage/main/pre_update 4.55, stage/main/last 0.97, surface/cell_upload_ms 0.82, stream/drive/ms 0.55, stage/main/first 0.46, stage/main/fixed_main_loop 0.34, stream/tick/ms 0.12, bigspace/total/ms 0.11 |
| warm-up | 3.33 | 867.4 | main/busy/ms 2616, stage/main/post_update 2354, render/thread/ms 837, render/busy/ms 701, stage/render/prepare_assets 654, stage/main/update 249, stage/render/prepare_views 133, render/acquire/ms 132, surface/advance_cpu_ms 66.70, stage/render/create_views 27.59, stage/render/render 13.79, render/graph/ms 9.25 |
| warm-up | 3.44 | 108.1 | render/thread/ms 929, render/busy/ms 740, stage/render/prepare_assets 680, main/busy/ms 201, stage/main/update 156, stage/render/prepare_views 133, render/acquire/ms 133, stage/render/render 72.64, render/present/ms 57.01, pipelines/compile_ms 42.81, pipelines/front_ms 37.73, stage/main/post_update 34.55 |
| warm-up | 3.48 | 39.8 | pipelines/compile_ms 307, pipelines/front_ms 258, render/thread/ms 118, stage/render/render 78.11, render/present/ms 62.62, main/busy/ms 59.70, render/busy/ms 54.50, pipelines/driver_ms 48.88, stage/main/post_update 45.38, stage/render/prepare_assets 26.69, terrain/update/ms 23.69, pipelines/wait_ms 17.51 |
| warm-up | 3.61 | 133.0 | pipelines/wait_ms 4945, pipelines/compile_ms 1497, pipelines/front_ms 1372, render/thread/ms 154, render/busy/ms 144, pipelines/driver_ms 125, main/busy/ms 115, stage/main/post_update 101, stage/render/prepare_resources 65.62, stage/render/render 35.67, render/graph/ms 24.88, stage/render/prepare_batch_phases 20.38 |
| warm-up | 3.67 | 55.0 | pipelines/wait_ms 5192, pipelines/compile_ms 2141, pipelines/front_ms 2009, render/thread/ms 182, render/busy/ms 180, main/busy/ms 159, pipelines/driver_ms 132, stage/main/post_update 79.09, stage/render/prepare_resources 71.42, stage/main/update 58.28, stage/render/prepare_assets 40.44, stage/render/render 26.27 |
| warm-up | 3.77 | 104.5 | pipelines/compile_ms 1886, pipelines/front_ms 1803, pipelines/wait_ms 455, render/thread/ms 121, render/busy/ms 119, main/busy/ms 105, stage/render/prepare_assets 85.75, pipelines/driver_ms 83.02, stage/main/update 61.28, stage/main/post_update 21.57, stage/render/render 16.21, terrain/update/ms 15.42 |
| warm-up | 3.84 | 63.1 | pipelines/compile_ms 1856, pipelines/front_ms 1625, pipelines/wait_ms 275, pipelines/driver_ms 231, render/thread/ms 129, render/busy/ms 126, stage/render/prepare_assets 98.39, main/busy/ms 38.30, stage/render/render 23.60, render/graph/ms 20.50, stage/render/graph_submit 18.16, encode//sum 16.68 |
| warm-up | 4.00 | 67.3 | pipelines/compile_ms 1095, pipelines/front_ms 1041, render/thread/ms 60.16, render/busy/ms 58.29, pipelines/driver_ms 54.60, main/busy/ms 53.88, stage/main/update 43.32, stream/drive/ms 39.62, stage/render/prepare_assets 39.18, stage/render/render 14.10, render/graph/ms 11.71, stage/render/graph_submit 9.51 |
| warm-up | 4.03 | 29.0 | pipelines/compile_ms 1234, pipelines/front_ms 1170, render/thread/ms 64.66, pipelines/driver_ms 64.29, render/busy/ms 63.06, main/busy/ms 39.49, stage/render/prepare_assets 31.49, stage/main/update 26.78, stream/drive/ms 21.80, stage/render/prepare_resources 13.14, stage/render/render 11.93, render/graph/ms 9.97 |
| warm-up | 4.08 | 46.7 | pipelines/compile_ms 493, pipelines/front_ms 469, render/thread/ms 63.71, render/busy/ms 61.99, main/busy/ms 45.21, stage/main/update 30.05, stage/render/prepare_assets 26.75, stream/drive/ms 24.51, pipelines/driver_ms 24.04, stage/render/render 13.80, stage/render/prepare_resources 13.67, render/graph/ms 11.60 |
| warm-up | 4.11 | 34.4 | pipelines/compile_ms 184, pipelines/front_ms 174, main/busy/ms 67.86, render/thread/ms 60.65, render/busy/ms 58.70, stage/main/update 52.44, stream/drive/ms 46.91, stage/render/prepare_assets 38.59, stage/render/render 12.83, render/graph/ms 10.43, pipelines/driver_ms 10.13, stage/main/post_update 8.95 |
| warm-up | 4.24 | 123.0 | pipelines/compile_ms 1235, pipelines/front_ms 1205, main/busy/ms 87.56, render/thread/ms 78.95, render/busy/ms 77.21, stage/main/update 71.00, stage/render/prepare_assets 59.64, stream/drive/ms 45.16, pipelines/driver_ms 29.52, stage/render/render 10.63, stage/main/post_update 9.75, render/graph/ms 8.44 |
| warm-up | 4.33 | 95.5 | pipelines/compile_ms 1349, pipelines/front_ms 1307, render/thread/ms 128, render/busy/ms 125, main/busy/ms 93.44, stage/render/prepare_assets 83.21, stage/main/update 75.01, pipelines/driver_ms 41.92, stream/drive/ms 35.45, stage/render/render 30.94, render/graph/ms 27.17, stage/render/graph_submit 24.06 |
| warm-up | 4.38 | 42.9 | pipelines/compile_ms 115, pipelines/front_ms 102, render/thread/ms 101, render/busy/ms 97.96, main/busy/ms 75.06, stage/main/update 55.70, stage/render/prepare_assets 53.47, stream/drive/ms 34.95, stage/render/render 31.89, render/graph/ms 28.03, stage/render/graph_submit 25.03, encode/submit 22.29 |
| warm-up | 4.48 | 108.0 | pipelines/compile_ms 1186, pipelines/front_ms 1150, pipelines/wait_ms 547, render/thread/ms 126, render/busy/ms 124, main/busy/ms 68.68, stage/render/prepare_assets 56.50, stage/render/prepare_write_phase_buffers 37.15, stage/main/post_update 36.56, pipelines/driver_ms 36.24, stage/main/update 19.58, bigspace/total/ms 18.78 |
| warm-up | 4.56 | 78.8 | pipelines/compile_ms 2141, pipelines/front_ms 2054, pipelines/wait_ms 783, render/thread/ms 171, render/busy/ms 169, pipelines/driver_ms 87.34, main/busy/ms 80.06, stage/render/prepare_assets 77.21, stage/render/prepare_write_phase_buffers 44.50, stage/main/post_update 39.66, stage/main/update 29.10, bigspace/total/ms 18.93 |
| warm-up | 4.59 | 26.3 | pipelines/compile_ms 1593, pipelines/front_ms 1523, pipelines/wait_ms 238, render/thread/ms 92.75, render/busy/ms 91.12, pipelines/driver_ms 69.86, stage/render/prepare_assets 44.96, main/busy/ms 41.60, stage/main/update 22.26, stage/render/prepare_batch_phases 15.84, stage/main/post_update 11.44, stage/render/render 9.54 |
| warm-up | 4.65 | 58.2 | pipelines/compile_ms 1470, pipelines/front_ms 1422, pipelines/wait_ms 200, render/thread/ms 52.82, render/busy/ms 51.43, pipelines/driver_ms 48.81, stage/render/prepare_assets 34.14, main/busy/ms 32.70, stage/main/post_update 14.16, stage/main/update 11.38, stage/render/render 9.17, render/graph/ms 7.53 |
| warm-up | 4.68 | 32.0 | pipelines/compile_ms 1062, pipelines/front_ms 1021, pipelines/wait_ms 198, render/thread/ms 62.97, render/busy/ms 60.62, main/busy/ms 51.57, pipelines/driver_ms 41.07, stage/render/prepare_assets 29.49, stage/main/update 24.68, stage/main/post_update 20.61, stage/render/render 16.16, render/graph/ms 13.52 |
| warm-up | 4.73 | 49.5 | pipelines/compile_ms 230, pipelines/front_ms 219, render/thread/ms 59.71, render/busy/ms 57.25, main/busy/ms 51.63, stage/main/update 25.52, stage/render/prepare_assets 21.29, stage/main/post_update 20.03, stage/render/render 17.08, render/graph/ms 13.98, pipelines/driver_ms 11.02, stage/render/graph_submit 10.66 |
| warm-up | 4.76 | 28.6 | pipelines/compile_ms 199, pipelines/front_ms 194, main/busy/ms 124, stage/main/update 104, stream/drive/ms 98.05, pipelines/wait_ms 56.90, render/thread/ms 51.77, render/busy/ms 48.01, stage/render/prepare_assets 25.32, stage/render/render 14.27, stage/main/post_update 13.16, render/graph/ms 9.89 |
| warm-up | 4.88 | 120.6 | pipelines/compile_ms 1583, pipelines/front_ms 1540, main/busy/ms 118, stage/main/update 103, stream/drive/ms 98.75, pipelines/wait_ms 59.02, render/thread/ms 47.17, render/busy/ms 43.40, pipelines/driver_ms 42.87, stage/render/prepare_assets 23.74, stage/render/render 13.79, render/graph/ms 9.62 |
| warm-up | 4.91 | 36.9 | pipelines/compile_ms 1463, pipelines/front_ms 1420, render/thread/ms 59.77, render/busy/ms 58.32, main/busy/ms 45.17, pipelines/driver_ms 42.14, stage/main/update 26.03, stage/render/prepare_assets 24.80, stage/render/render 11.56, stage/main/post_update 11.44, render/graph/ms 9.65, stage/render/queue 8.58 |
| warm-up | 4.95 | 40.2 | pipelines/compile_ms 674, pipelines/front_ms 654, main/busy/ms 65.71, render/thread/ms 65.64, render/busy/ms 64.02, stage/main/update 39.02, pipelines/driver_ms 20.59, stage/render/prepare_assets 20.47, stage/main/post_update 15.69, stage/render/render 13.67, render/graph/ms 11.36, encode//sum 10.17 |
| warm-up | 5.00 | 44.0 | pipelines/compile_ms 1634, pipelines/front_ms 1591, main/busy/ms 67.93, render/thread/ms 62.68, render/busy/ms 60.94, pipelines/wait_ms 49.57, stage/main/update 45.82, pipelines/driver_ms 42.61, stage/render/prepare_assets 27.77, stage/main/post_update 13.25, stage/render/render 13.17, stream/drive/ms 10.84 |
| warm-up | 5.04 | 37.4 | pipelines/compile_ms 1070, pipelines/front_ms 1042, render/thread/ms 55.44, render/busy/ms 53.97, pipelines/wait_ms 49.68, main/busy/ms 47.96, stage/main/update 34.08, pipelines/driver_ms 28.43, stage/render/prepare_assets 27.90, stream/drive/ms 11.43, stage/render/render 10.14, render/graph/ms 8.22 |
| warm-up | 5.08 | 27.1 | pipelines/compile_ms 468, pipelines/front_ms 454, render/thread/ms 40.68, render/busy/ms 39.08, main/busy/ms 33.41, stage/main/update 19.17, stage/render/render 14.70, pipelines/driver_ms 13.40, render/graph/ms 12.59, stage/render/graph_submit 8.93, stage/main/post_update 8.77, encode//sum 6.79 |
| warm-up | 5.14 | 34.6 | pipelines/compile_ms 356, pipelines/front_ms 338, render/thread/ms 47.74, render/busy/ms 46.24, main/busy/ms 42.68, stage/main/update 25.95, pipelines/driver_ms 18.08, stage/render/render 17.48, stream/drive/ms 17.02, render/graph/ms 15.66, stage/main/post_update 12.23, stage/render/graph_submit 11.87 |
| warm-up | 5.18 | 33.5 | pipelines/compile_ms 313, pipelines/front_ms 298, render/thread/ms 56.72, render/busy/ms 55.19, main/busy/ms 33.13, stage/render/prepare_assets 22.38, pipelines/driver_ms 15.19, stage/main/update 14.34, stage/main/post_update 14.00, stage/render/render 12.79, render/graph/ms 10.89, encode//sum 8.27 |
| warm-up | 5.21 | 36.7 | pipelines/compile_ms 402, pipelines/front_ms 389, render/thread/ms 57.19, render/busy/ms 55.90, main/busy/ms 35.51, stage/render/prepare_assets 30.43, stage/main/update 18.98, pipelines/driver_ms 13.21, stage/main/post_update 11.24, stage/render/render 9.87, render/graph/ms 7.88, encode//sum 7.29 |
| warm-up | 5.26 | 42.9 | pipelines/compile_ms 817, pipelines/front_ms 793, render/thread/ms 61.37, render/busy/ms 59.93, main/busy/ms 39.39, stage/render/prepare_assets 30.69, pipelines/driver_ms 24.03, stage/main/update 22.52, stage/main/post_update 11.26, stage/render/render 10.92, render/graph/ms 8.85, encode//sum 6.94 |
| warm-up | 5.29 | 39.7 | pipelines/compile_ms 814, pipelines/front_ms 789, render/thread/ms 67.57, render/busy/ms 65.95, main/busy/ms 33.52, stage/render/prepare_assets 29.36, pipelines/driver_ms 24.82, stage/render/render 15.51, stage/main/update 15.42, render/graph/ms 13.40, stage/main/post_update 11.91, stage/render/graph_submit 10.37 |
| warm-up | 5.34 | 45.8 | pipelines/compile_ms 1206, pipelines/front_ms 1168, render/thread/ms 71.52, render/busy/ms 69.93, main/busy/ms 43.88, pipelines/driver_ms 38.01, stage/render/prepare_assets 37.16, stage/main/post_update 15.69, pipelines/wait_ms 15.60, stage/render/render 14.92, stage/main/update 14.18, render/graph/ms 12.79 |
| warm-up | 5.38 | 35.2 | pipelines/compile_ms 1232, pipelines/front_ms 1196, render/thread/ms 63.58, render/busy/ms 62.01, main/busy/ms 42.27, stage/render/prepare_assets 39.58, pipelines/driver_ms 35.58, pipelines/wait_ms 15.58, stage/main/post_update 14.60, stage/main/update 14.22, stage/render/render 10.94, stage/main/pre_update 10.19 |
| warm-up | 5.45 | 27.8 | render/thread/ms 37.20, render/busy/ms 35.73, main/busy/ms 32.12, stage/main/update 17.06, stage/render/render 13.36, stage/render/prepare_assets 13.14, stream/drive/ms 11.71, render/graph/ms 11.33, stage/main/post_update 10.83, stage/render/graph_submit 8.40, encode//sum 7.16, encode/submit 5.31 |
| warm-up | 5.50 | 34.6 | pipelines/compile_ms 1205, pipelines/front_ms 1176, render/thread/ms 40.07, render/busy/ms 38.36, pipelines/driver_ms 28.89, main/busy/ms 28.11, stage/render/prepare_assets 19.75, stage/main/update 12.26, stage/main/post_update 10.62, stage/render/render 9.65, render/graph/ms 7.50, encode//sum 6.94 |
| warm-up | 5.54 | 25.2 | pipelines/compile_ms 390, pipelines/front_ms 383, render/thread/ms 38.09, render/busy/ms 36.62, main/busy/ms 31.81, pipelines/wait_ms 28.43, stage/render/prepare_assets 13.91, stage/main/update 13.84, stage/main/post_update 12.44, stage/render/render 11.57, render/graph/ms 9.43, encode//sum 8.07 |
| warm-up | 5.58 | 41.0 | pipelines/compile_ms 226, pipelines/front_ms 220, render/thread/ms 60.58, render/busy/ms 59.08, main/busy/ms 33.88, stage/render/prepare_assets 26.05, stage/render/render 24.60, render/graph/ms 22.34, stage/render/graph_submit 19.63, stage/main/update 16.54, stage/main/post_update 11.86, encode//sum 7.62 |
| warm-up | 5.62 | 32.9 | pipelines/compile_ms 115, pipelines/front_ms 108, render/thread/ms 66.97, render/busy/ms 65.65, main/busy/ms 38.89, stage/render/prepare_assets 31.34, stage/render/render 22.75, render/graph/ms 20.86, stage/render/graph_submit 18.30, stage/main/update 16.52, stage/main/post_update 16.34, pipelines/driver_ms 6.57 |
| warm-up | 5.65 | 31.2 | pipelines/compile_ms 77.40, pipelines/front_ms 73.10, render/thread/ms 55.69, render/busy/ms 54.43, main/busy/ms 36.67, stage/render/prepare_assets 32.01, stage/main/post_update 15.36, stage/main/update 14.34, stage/render/render 9.24, render/graph/ms 7.25, encode//sum 6.52, stage/render/graph_submit 4.77 |
| warm-up | 5.68 | 33.3 | pipelines/compile_ms 333, pipelines/front_ms 321, render/thread/ms 58.47, render/busy/ms 56.99, stage/render/prepare_assets 31.35, main/busy/ms 26.94, stage/render/render 12.78, pipelines/driver_ms 12.21, stage/main/update 11.97, render/graph/ms 10.33, stage/main/post_update 9.14, encode//sum 8.23 |
| warm-up | 5.71 | 26.8 | pipelines/compile_ms 514, pipelines/front_ms 494, render/thread/ms 55.17, render/busy/ms 53.24, main/busy/ms 33.43, pipelines/driver_ms 19.58, stage/render/prepare_assets 18.70, stage/render/render 17.26, render/graph/ms 14.46, stage/main/update 13.47, stage/main/post_update 13.40, stage/render/graph_submit 10.66 |
| warm-up | 5.76 | 50.6 | pipelines/compile_ms 364, pipelines/front_ms 351, render/thread/ms 62.79, render/busy/ms 60.96, main/busy/ms 36.97, stage/render/prepare_assets 32.08, stage/main/post_update 15.37, stage/main/update 14.21, stage/render/render 14.01, pipelines/driver_ms 13.00, render/graph/ms 11.67, encode//sum 8.59 |
| warm-up | 5.79 | 30.2 | pipelines/compile_ms 979, pipelines/front_ms 959, render/thread/ms 63.24, render/busy/ms 62.04, stage/render/prepare_assets 42.61, main/busy/ms 33.59, pipelines/driver_ms 20.27, stage/main/update 13.13, stage/main/post_update 11.24, stage/render/render 8.77, render/graph/ms 7.14, encode//sum 7.10 |
| warm-up | 5.82 | 34.0 | pipelines/compile_ms 796, pipelines/front_ms 781, render/thread/ms 52.94, render/busy/ms 51.85, stage/render/prepare_assets 33.67, main/busy/ms 33.60, pipelines/driver_ms 14.64, stage/main/update 13.38, stage/main/post_update 10.92, stage/render/render 9.03, render/graph/ms 7.37, encode//sum 7.00 |
| warm-up | 5.88 | 29.0 | pipelines/compile_ms 132, pipelines/front_ms 126, render/thread/ms 44.43, render/busy/ms 42.89, main/busy/ms 42.81, stage/main/update 21.97, stage/main/post_update 15.48, stage/render/prepare_assets 15.17, stage/render/render 13.38, render/graph/ms 11.19, encode//sum 8.58, stage/render/graph_submit 7.73 |
| warm-up | 5.91 | 30.9 | pipelines/compile_ms 132, pipelines/front_ms 126, render/thread/ms 48.59, render/busy/ms 47.00, main/busy/ms 42.33, stage/main/update 20.48, stage/render/prepare_assets 18.97, stage/main/post_update 14.62, stage/render/render 13.46, render/graph/ms 11.35, encode//sum 10.42, stage/render/graph_submit 7.71 |
| warm-up | 5.98 | 48.7 | main/busy/ms 49.74, render/thread/ms 42.06, render/busy/ms 40.50, stage/main/update 27.37, stage/render/prepare_assets 17.61, stream/drive/ms 16.70, stage/main/post_update 13.88, stage/render/render 12.60, render/graph/ms 10.40, pipelines/compile_ms 8.79, encode//sum 8.54, pipelines/front_ms 8.21 |
| warm-up | 6.02 | 37.2 | render/thread/ms 50.88, render/busy/ms 49.32, main/busy/ms 48.42, stage/main/update 24.96, stage/render/prepare_assets 24.86, pipelines/compile_ms 22.55, pipelines/front_ms 20.73, stage/main/post_update 14.16, stage/render/render 13.08, render/graph/ms 10.84, encode//sum 8.78, stream/drive/ms 8.68 |
| warm-up | 6.05 | 30.1 | pipelines/compile_ms 992, pipelines/front_ms 960, render/thread/ms 46.43, main/busy/ms 45.42, render/busy/ms 44.71, pipelines/driver_ms 31.62, stage/main/update 23.06, stage/render/prepare_assets 20.02, stage/main/post_update 13.61, stage/render/render 13.38, render/graph/ms 10.99, stream/drive/ms 8.51 |
| warm-up | 6.08 | 30.4 | pipelines/compile_ms 1015, pipelines/front_ms 984, render/thread/ms 43.33, render/busy/ms 41.78, main/busy/ms 36.67, pipelines/driver_ms 31.67, stage/main/update 15.82, stage/render/prepare_assets 15.77, stage/render/render 12.70, stage/main/post_update 11.88, render/graph/ms 10.58, stage/render/graph_submit 7.47 |
| warm-up | 6.21 | 39.1 | pipelines/compile_ms 285, pipelines/front_ms 275, main/busy/ms 74.08, render/thread/ms 49.05, render/busy/ms 46.98, stage/main/post_update 37.00, stage/main/update 30.47, stream/drive/ms 21.38, stage/render/render 20.33, render/graph/ms 17.79, stage/render/graph_submit 13.22, pipelines/driver_ms 10.15 |
| warm-up | 6.26 | 43.0 | pipelines/compile_ms 1071, pipelines/front_ms 1038, main/busy/ms 61.48, render/thread/ms 48.72, render/busy/ms 47.05, pipelines/driver_ms 33.22, stage/main/update 29.87, stage/main/post_update 24.99, stream/drive/ms 21.47, pipelines/wait_ms 20.03, stage/render/render 14.40, render/graph/ms 12.32 |
| warm-up | 6.36 | 31.3 | pipelines/compile_ms 790, pipelines/front_ms 771, main/busy/ms 45.05, render/thread/ms 39.38, render/busy/ms 38.06, stage/main/update 22.57, pipelines/driver_ms 18.87, stage/main/post_update 14.37, stage/render/render 13.33, stage/render/prepare_assets 11.55, render/graph/ms 11.49, encode//sum 9.39 |
| warm-up | 6.58 | 26.8 | render/thread/ms 33.79, render/busy/ms 32.22, main/busy/ms 30.83, stage/render/render 15.28, stage/main/post_update 13.13, render/graph/ms 12.64, stage/main/update 12.49, encode//sum 11.57, stage/render/graph_submit 8.47, stage/render/prepare_assets 8.27, encode/submit 4.60, stage/render/camera_driver 3.98 |
| warm-up | 6.61 | 25.4 | render/thread/ms 41.35, main/busy/ms 41.00, render/busy/ms 39.46, stage/main/update 20.78, stage/render/render 17.46, render/graph/ms 14.50, stage/main/post_update 13.30, stream/drive/ms 11.86, stage/render/prepare_assets 11.69, encode//sum 11.24, stage/render/graph_submit 10.21, encode/submit 6.48 |
| warm-up | 6.65 | 48.3 | render/thread/ms 50.35, render/busy/ms 49.02, main/busy/ms 40.41, stage/render/prepare_assets 27.33, stage/main/update 20.62, stage/main/post_update 13.07, stage/render/render 12.98, stream/drive/ms 11.97, render/graph/ms 11.13, encode//sum 8.05, stage/render/graph_submit 7.94, encode/submit 5.25 |
| warm-up | 6.99 | 31.2 | render/thread/ms 38.49, render/busy/ms 36.96, main/busy/ms 14.60, stage/render/render 11.27, render/graph/ms 9.90, stage/render/specialize 9.25, stage/main/post_update 7.90, stage/render/graph_submit 7.90, encode//sum 7.29, stage/render/queue 6.41, encode/submit 4.73, stage/main/update 4.16 |
| warm-up | 8.20 | 30.8 | render/thread/ms 36.53, render/busy/ms 34.04, main/busy/ms 14.83, stage/render/render 10.40, render/graph/ms 9.11, stage/render/specialize 8.60, stage/main/post_update 8.13, stage/render/graph_submit 7.23, stage/render/queue 6.10, encode//sum 5.86, stage/render/prepare_meshes 4.30, encode/submit 4.20 |
| measure | 0.83 | 27.0 | render/thread/ms 34.27, render/busy/ms 31.71, main/busy/ms 14.30, stage/render/render 10.76, render/graph/ms 9.40, encode//sum 9.31, stage/render/specialize 8.31, stage/main/post_update 8.08, stage/render/graph_submit 7.52, stage/render/queue 5.68, encode/submit 4.64, stage/main/update 3.88 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=base-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 17%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **97%** (min 86%) · 2614 MHz (min 2610) · memory 8101 MHz · 98.4 W · 80 °C (max 82) · VRAM 6447 MiB · throttled 100% of samples

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
