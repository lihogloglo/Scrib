# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.day_gate=on (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1552 frames over 15.0 s · **median 9.66 ms (104 fps)** · mean 9.67 · p95 11.10 · p99 12.05 (1% low 83 fps) · worst 27.57 ms · 6 spikes over 1.5× median · 24.0 stutters/min (work 12.0/min)
- warm-up: 277 frames over 8.8 s, worst 2562.2 ms
- hitches (> 25.0 ms): **1** measured, 58 in warm-up
- timed GPU passes: 8.41 ms of a 9.67 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.66 ms, limit 11 ms
- pass worst frame 27.57 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.841 | 2.882 | 2.969 | 0.99 | 0.063 |
| dlss_super_resolution | 1.992 | 2.021 | 2.044 | 0.99 | 0.188 |
| early prepass | 0.597 | 0.606 | 0.620 | 0.99 | 0.095 |
| atmosphere_luts | 0.323 | 0.327 | 0.378 | 0.99 | 0.003 |
| bloom | 0.310 | 0.315 | 0.331 | 0.99 | 0.064 |
| main_transmissive_pass_3d | 0.304 | 0.309 | 0.332 | 0.99 | 0.008 |
| water_velocity | 0.236 | 0.239 | 0.265 | 0.99 | 0.002 |
| water_sspr | 0.205 | 0.208 | 0.211 | 0.99 | 0.002 |
| ssao | 0.188 | 0.190 | 0.197 | 0.99 | 0.007 |
| clouds_trace | 0.147 | 0.149 | 0.172 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.143 | 0.146 | 0.153 | 0.99 | 0.001 |
| water_fft | 0.130 | 0.132 | 0.135 | 0.99 | 0.003 |
| uniform_allocation | 0.117 | 0.119 | 0.159 | 0.99 | 0.022 |
| upscaling | 0.111 | 0.113 | 0.119 | 0.99 | 0.004 |
| clouds_composite | 0.110 | 0.112 | 0.114 | 0.99 | 0.002 |
| bin_unpacking | 0.089 | 0.090 | 0.122 | 0.99 | 0.016 |
| tonemapping | 0.085 | 0.087 | 0.091 | 0.99 | 0.004 |
| clustering | 0.064 | 0.065 | 0.075 | 0.99 | 0.020 |
| render_sky | 0.051 | 0.052 | 0.053 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.050 | 0.409 | 0.412 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.99 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.031 | 0.032 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 0.99 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.221 | 0.233 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 0.99 | 0.084 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.047 | 0.31 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.012 | 0.022 | 0.99 | 0.005 |
| surface_exposure | 0.011 | 0.042 | 0.043 | 0.27 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.007 | 0.007 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.705 | 10.159 | 26.21 | 13510.5 |
| main/busy/ms | 7.271 | 8.228 | 10.44 | 11284.5 |
| render/busy/ms | 7.040 | 8.288 | 25.67 | 10925.9 |
| stage/render/render | 4.726 | 5.776 | 12.36 | 7334.0 |
| stage/main/post_update | 4.282 | 5.083 | 6.21 | 6646.3 |
| render/graph/ms | 4.130 | 5.107 | 11.53 | 6409.7 |
| encode//sum | 3.755 | 5.168 | 21.14 | 5827.2 |
| stage/render/graph_submit | 3.120 | 4.024 | 10.58 | 4842.1 |
| encode/submit | 1.974 | 2.567 | 7.66 | 3064.1 |
| stage/main/update | 1.858 | 2.275 | 3.69 | 2884.2 |
| stage/render/prepare_views | 1.340 | 3.018 | 3.94 | 2079.2 |
| render/acquire/ms | 1.294 | 2.965 | 3.89 | 2008.6 |
| encode/finish_wall | 1.177 | 1.689 | 7.75 | 1826.6 |
| stage/render/camera_driver | 0.954 | 1.119 | 2.05 | 1480.6 |
| encode/pass:shadow_directional_light/sum | 0.754 | 1.297 | 2.07 | 1170.2 |
| encode/pass:early prepass/sum | 0.725 | 1.008 | 6.61 | 1124.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.709 | 0.997 | 1.48 | 1099.6 |
| stage/render/prepare_write_phase_buffers | 0.543 | 0.751 | 1.06 | 842.2 |
| stage/render/prepare_assets | 0.456 | 0.592 | 1.56 | 707.5 |
| bigspace/total/ms | 0.387 | 0.896 | 1.68 | 600.2 |
| stage/render/prepare_resources | 0.386 | 0.456 | 0.96 | 599.1 |
| render/present/ms | 0.371 | 0.450 | 4.63 | 576.1 |
| stage/main/pre_update | 0.345 | 0.458 | 0.83 | 535.5 |
| stage/main/first | 0.300 | 0.369 | 1.58 | 465.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.236 | 0.346 | 0.95 | 366.3 |
| terrain/update/ms | 0.235 | 0.302 | 0.65 | 364.7 |
| stage/render/prepare_bind_groups | 0.209 | 0.298 | 0.51 | 324.0 |
| stage/render/prepare_meshes | 0.205 | 0.260 | 2.37 | 317.7 |
| terrain/select/ms | 0.204 | 0.260 | 0.57 | 316.7 |
| stream/drive/ms | 0.188 | 0.259 | 0.59 | 292.1 |
| stage/main/fixed_main_loop | 0.184 | 0.353 | 0.92 | 285.3 |
| stage/render/prepare_batch_phases | 0.180 | 0.244 | 0.39 | 279.6 |
| stage/render/create_views | 0.172 | 0.223 | 0.65 | 266.8 |
| stage/render/extract_commands | 0.171 | 0.206 | 0.52 | 266.1 |
| far/select/ms | 0.170 | 0.239 | 0.57 | 263.7 |
| controller/move/ms | 0.162 | 0.300 | 0.62 | 251.6 |
| stream/tick/ms | 0.160 | 0.225 | 0.56 | 249.1 |
| stage/render/prepare_flush | 0.152 | 0.268 | 0.55 | 235.4 |
| controller/feed/ms | 0.145 | 0.196 | 0.54 | 224.7 |
| bigspace/high_precision/ms | 0.139 | 0.422 | 1.36 | 215.4 |
| stage/main/last | 0.120 | 0.153 | 2.28 | 186.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.114 | 0.179 | 0.40 | 177.5 |
| stage/render/queue | 0.109 | 0.140 | 5.70 | 168.7 |
| bigspace/low_precision/ms | 0.101 | 0.249 | 1.28 | 157.2 |
| anim/evaluate/ms | 0.088 | 0.213 | 1.08 | 136.3 |
| stage/render/specialize | 0.086 | 0.108 | 8.56 | 133.1 |
| anim/evaluate_cpu/ms | 0.060 | 0.074 | 0.47 | 93.6 |
| stage/render/prepare_collect_phase_buffers | 0.060 | 0.117 | 0.24 | 93.6 |
| encode/resolve_diagnostics/sum | 0.053 | 0.082 | 0.17 | 81.7 |
| physics/step/ms | 0.028 | 0.056 | 0.44 | 43.0 |
| weather/map/ms | 0.025 | 0.034 | 0.38 | 39.0 |
| encode/camera_driver_span_begin/sum | 0.025 | 0.038 | 0.19 | 39.0 |
| stage/render/cleanup | 0.023 | 0.032 | 0.28 | 35.6 |
| encode/camera_driver_span_end/sum | 0.022 | 0.036 | 0.36 | 34.3 |
| stage/render/phase_sort | 0.018 | 0.035 | 0.09 | 28.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.06 | 17.3 |
| script/update/ms | 0.007 | 0.010 | 0.40 | 11.0 |
| controller/avoid/ms | 0.006 | 0.007 | 0.02 | 8.7 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 8.1 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 7.0 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.4 |
| controller/write/ms | 0.002 | 0.002 | 0.38 | 3.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 1.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.6 |
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
| crowd/c1/count | count | 123 | 123 | 122 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1080 |
| crowd/c2/count | count | 283 | 284 | 284 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.52 | 7.69 | 0.50 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.19 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.36 | 0.01 |
| encode/finish_critical | ms | 0.85 | 7.69 | 0.72 |
| encode/pass:early prepass/max | ms | 0.72 | 6.61 | 0.55 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.71 | 1.48 | 0.72 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.40 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.95 | 0.29 |
| encode/pass:shadow_directional_light/max | ms | 0.70 | 2.07 | 0.58 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.17 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.16 | 25.67 | 9.14 |
| gpu/timed/ms | ms | 9.16 | 10.80 | 9.14 |
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
| process/cpu_usage | % | 25.48 | 36.54 | 28.51 |
| process/mem_usage | GiB | 4.68 | 4.73 | 4.73 |
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
| system/cpu_usage | % | 33.73 | 59.71 | 33.17 |
| system/mem_usage | % | 58.90 | 59.03 | 59.02 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 53 | 82 | 53 |
| views/all/bins | count | 5326 | 8063 | 5326 |
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
| anim/events/count | count | 0.10 | 2 | 154 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.19 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 26.57 | 53 | 41.2 k |
| encode/bind_walk/init_textures | count | 7166 | 14.0 k | 11.12 M |
| encode/bind_walk/init_textures_skipped | count | 35.6 k | 62.5 k | 55.26 M |
| encode/bind_walk/inits | count | 345 | 653 | 536.0 k |
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
| encode/queue/command_buffers | count | 36.57 | 73 | 56.8 k |
| encode/queue/submits | count | 2 | 4 | 3104 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.65 M | 2.23 G |
| encode/queue/write_buffer/calls | count | 217 | 400 | 337.3 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 65.3 k | 50.66 M |
| encode/queue/write_buffer_with/calls | count | 9 | 18 | 14.0 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1552 |
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

## Hitches (> 25.0 ms; slowest 59, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2562.2 | main/busy/ms 2530, stage/main/post_update 2420, stage/main/update 101, surface/advance_cpu_ms 68.73, stage/main/pre_update 4.63, stage/main/last 1.42, surface/cell_upload_ms 0.75, stage/main/first 0.58, stream/drive/ms 0.43, stage/main/fixed_main_loop 0.36, weather/map/ms 0.16, bigspace/total/ms 0.08 |
| warm-up | 3.30 | 741.7 | main/busy/ms 2665, stage/main/post_update 2444, render/thread/ms 706, render/busy/ms 536, stage/render/prepare_assets 486, stage/main/update 206, stage/render/prepare_views 167, render/acquire/ms 166, surface/advance_cpu_ms 68.73, stage/render/create_views 28.48, stage/render/render 13.77, render/graph/ms 9.04 |
| warm-up | 3.49 | 187.0 | render/thread/ms 875, render/busy/ms 606, stage/render/prepare_assets 539, stage/render/prepare_views 167, render/acquire/ms 167, main/busy/ms 163, stage/render/render 118, stage/main/update 114, render/present/ms 102, stage/main/post_update 36.67, stage/render/create_views 28.98, pipelines/compile_ms 23.84 |
| warm-up | 3.53 | 39.5 | pipelines/compile_ms 450, pipelines/front_ms 310, render/thread/ms 206, main/busy/ms 176, pipelines/driver_ms 140, stage/render/render 134, pipelines/wait_ms 130, render/busy/ms 104, render/present/ms 101, stage/main/update 78.39, stage/main/post_update 56.65, stage/render/prepare_assets 54.94 |
| warm-up | 3.80 | 266.8 | pipelines/wait_ms 4360, pipelines/compile_ms 3055, pipelines/front_ms 2775, pipelines/driver_ms 280, main/busy/ms 182, render/thread/ms 180, render/busy/ms 175, stage/render/prepare_assets 119, stage/main/update 80.26, stage/main/post_update 59.57, stage/render/render 47.64, render/graph/ms 42.95 |
| warm-up | 3.91 | 114.6 | pipelines/wait_ms 4405, pipelines/compile_ms 3654, pipelines/front_ms 3242, pipelines/driver_ms 412, render/thread/ms 255, render/busy/ms 252, stage/render/prepare_assets 219, main/busy/ms 59.30, stage/render/render 25.06, stage/main/post_update 25.06, render/graph/ms 22.31, encode//sum 18.61 |
| warm-up | 3.95 | 34.8 | pipelines/compile_ms 1122, pipelines/front_ms 825, pipelines/driver_ms 297, pipelines/wait_ms 176, render/thread/ms 143, render/busy/ms 141, stage/render/prepare_assets 106, main/busy/ms 40.61, stage/render/render 25.91, render/graph/ms 23.59, stage/render/graph_submit 20.86, encode//sum 18.42 |
| warm-up | 4.00 | 49.9 | pipelines/compile_ms 953, pipelines/front_ms 860, pipelines/driver_ms 92.34, render/thread/ms 79.96, render/busy/ms 71.23, main/busy/ms 58.78, stage/render/render 51.38, render/graph/ms 48.51, stage/main/post_update 44.45, stage/render/camera_driver 24.21, stage/render/graph_submit 24.10, encode/submit 20.29 |
| warm-up | 4.07 | 56.4 | pipelines/compile_ms 746, pipelines/front_ms 727, render/thread/ms 59.14, render/busy/ms 56.97, stage/render/prepare_assets 36.37, main/busy/ms 32.80, pipelines/wait_ms 28.07, pipelines/driver_ms 19.45, stage/main/post_update 16.00, stage/render/render 14.68, render/graph/ms 12.26, stage/main/update 10.40 |
| warm-up | 4.09 | 25.9 | pipelines/compile_ms 598, pipelines/front_ms 572, render/thread/ms 72.01, render/busy/ms 69.62, main/busy/ms 67.45, stage/main/update 45.55, stream/drive/ms 41.41, stage/render/prepare_assets 36.62, pipelines/wait_ms 32.27, pipelines/driver_ms 26.39, stage/render/render 15.59, stage/main/post_update 14.66 |
| warm-up | 4.17 | 77.1 | pipelines/compile_ms 698, pipelines/front_ms 669, main/busy/ms 69.86, render/thread/ms 66.26, render/busy/ms 64.57, stage/main/update 44.92, stream/drive/ms 41.01, stage/render/prepare_assets 34.91, pipelines/driver_ms 28.52, stage/main/post_update 16.91, pipelines/wait_ms 12.59, stage/render/render 12.16 |
| warm-up | 4.25 | 72.2 | pipelines/compile_ms 918, pipelines/front_ms 894, render/thread/ms 53.88, render/busy/ms 52.46, stage/render/prepare_assets 41.17, main/busy/ms 38.42, stage/main/update 25.02, pipelines/driver_ms 24.42, stream/drive/ms 19.80, stage/render/render 8.42, stage/main/post_update 7.39, render/graph/ms 6.77 |
| warm-up | 4.28 | 26.7 | pipelines/compile_ms 749, pipelines/front_ms 729, render/thread/ms 69.02, render/busy/ms 67.71, stage/render/prepare_assets 54.75, main/busy/ms 36.26, stage/main/update 21.82, pipelines/driver_ms 19.36, stream/drive/ms 16.13, stage/render/render 9.31, stage/main/post_update 8.91, render/graph/ms 7.39 |
| warm-up | 4.34 | 64.8 | render/thread/ms 62.70, render/busy/ms 61.35, main/busy/ms 48.74, stage/render/prepare_assets 45.68, stage/main/update 28.96, stream/drive/ms 16.78, stage/render/render 10.69, stage/main/post_update 10.16, render/graph/ms 8.69, stage/render/graph_submit 6.06, encode//sum 5.76, encode/submit 4.33 |
| warm-up | 4.41 | 72.0 | main/busy/ms 119, pipelines/compile_ms 102, pipelines/front_ms 97.18, stage/main/update 96.31, render/thread/ms 88.65, render/busy/ms 87.32, stage/render/prepare_assets 67.93, stream/drive/ms 59.10, stage/main/post_update 11.80, stage/render/render 11.14, render/graph/ms 9.44, encode//sum 7.27 |
| warm-up | 4.54 | 123.2 | pipelines/compile_ms 867, pipelines/front_ms 836, main/busy/ms 119, render/thread/ms 104, render/busy/ms 103, stage/main/update 94.01, stage/render/prepare_assets 78.21, stream/drive/ms 60.79, pipelines/driver_ms 30.71, stage/main/post_update 13.27, stage/render/render 12.54, render/graph/ms 10.55 |
| warm-up | 4.60 | 62.8 | pipelines/compile_ms 1178, pipelines/front_ms 1136, render/thread/ms 105, render/busy/ms 103, main/busy/ms 74.82, stage/render/prepare_assets 59.85, pipelines/wait_ms 59.73, pipelines/driver_ms 41.31, stage/main/update 31.69, stage/main/post_update 18.82, stream/drive/ms 17.49, stage/render/prepare_flush 17.40 |
| warm-up | 4.67 | 74.5 | pipelines/compile_ms 517, pipelines/front_ms 496, pipelines/wait_ms 124, render/thread/ms 99.43, render/busy/ms 97.92, main/busy/ms 87.10, stage/render/prepare_assets 53.92, stage/main/update 42.21, stage/main/post_update 22.46, pipelines/driver_ms 20.05, stage/render/prepare_flush 17.49, stream/drive/ms 14.93 |
| warm-up | 4.74 | 65.1 | pipelines/compile_ms 937, pipelines/front_ms 907, pipelines/wait_ms 176, render/thread/ms 103, render/busy/ms 101, main/busy/ms 63.26, stage/render/prepare_assets 56.69, stage/main/update 34.14, pipelines/driver_ms 29.63, stage/main/post_update 16.12, stage/render/prepare_resources 15.39, stage/render/render 10.28 |
| warm-up | 4.77 | 29.4 | pipelines/compile_ms 1889, pipelines/front_ms 1836, pipelines/wait_ms 177, render/thread/ms 74.89, render/busy/ms 73.08, pipelines/driver_ms 53.50, main/busy/ms 37.02, stage/render/prepare_assets 29.51, stage/main/update 17.48, stage/render/prepare_resources 13.22, stage/render/render 12.11, render/graph/ms 9.92 |
| warm-up | 4.81 | 27.2 | pipelines/compile_ms 678, pipelines/front_ms 656, pipelines/wait_ms 50.14, render/thread/ms 37.78, render/busy/ms 35.82, main/busy/ms 31.79, pipelines/driver_ms 21.83, stage/main/update 15.15, stage/render/render 14.67, render/graph/ms 12.15, stage/main/post_update 10.31, stage/render/prepare_assets 9.50 |
| warm-up | 4.86 | 40.6 | pipelines/compile_ms 390, pipelines/front_ms 376, main/busy/ms 128, stage/main/update 104, stream/drive/ms 97.34, render/thread/ms 45.55, render/busy/ms 43.83, pipelines/wait_ms 42.79, stage/main/post_update 17.99, stage/render/render 16.01, stage/render/prepare_assets 15.99, pipelines/driver_ms 13.82 |
| warm-up | 5.00 | 143.8 | pipelines/compile_ms 2098, pipelines/front_ms 2030, pipelines/wait_ms 163, main/busy/ms 150, stage/main/update 126, stream/drive/ms 96.68, render/thread/ms 69.21, pipelines/driver_ms 68.35, render/busy/ms 67.79, stage/render/prepare_assets 41.79, stage/main/post_update 18.58, stage/render/render 14.24 |
| warm-up | 5.05 | 32.6 | pipelines/compile_ms 659, pipelines/front_ms 631, render/thread/ms 44.29, main/busy/ms 43.95, render/busy/ms 42.54, pipelines/driver_ms 27.54, stage/main/post_update 23.31, stage/render/render 23.16, render/graph/ms 21.03, stage/main/update 12.50, stage/render/graph_submit 10.46, stage/render/camera_driver 10.41 |
| warm-up | 5.08 | 32.3 | pipelines/compile_ms 523, pipelines/front_ms 499, main/busy/ms 75.23, render/thread/ms 53.07, render/busy/ms 49.83, stage/main/update 34.37, stage/render/render 33.35, stage/main/post_update 30.69, render/graph/ms 29.83, pipelines/driver_ms 24.48, stage/render/graph_submit 15.88, stage/render/camera_driver 13.71 |
| warm-up | 5.19 | 110.3 | pipelines/compile_ms 1151, pipelines/front_ms 1120, pipelines/wait_ms 524, render/thread/ms 84.66, main/busy/ms 84.33, render/busy/ms 81.82, stage/main/update 49.70, stage/render/prepare_assets 32.73, pipelines/driver_ms 30.50, stage/main/post_update 25.39, stage/render/render 21.14, render/graph/ms 17.81 |
| warm-up | 5.27 | 75.3 | pipelines/compile_ms 2500, pipelines/front_ms 2436, pipelines/wait_ms 634, render/thread/ms 128, render/busy/ms 127, main/busy/ms 74.25, pipelines/driver_ms 63.76, stage/render/prepare_assets 47.39, stage/main/post_update 39.45, stage/render/prepare_resources 36.06, stage/main/update 27.78, stage/render/specialize 19.32 |
| warm-up | 5.30 | 30.6 | pipelines/compile_ms 2078, pipelines/front_ms 2016, pipelines/wait_ms 142, render/thread/ms 100, render/busy/ms 98.62, pipelines/driver_ms 61.86, main/busy/ms 59.68, stage/main/post_update 33.30, stage/render/prepare_resources 32.61, stage/render/prepare_assets 20.93, pipelines/lock_ms 20.84, stage/main/update 18.36 |
| warm-up | 5.34 | 38.0 | pipelines/compile_ms 831, pipelines/front_ms 799, render/thread/ms 59.73, render/busy/ms 58.29, main/busy/ms 54.29, pipelines/driver_ms 32.41, stage/main/update 31.48, pipelines/wait_ms 25.31, stream/drive/ms 15.58, stage/render/prepare_assets 14.08, stage/main/post_update 14.05, stage/render/render 13.26 |
| warm-up | 5.38 | 43.2 | pipelines/compile_ms 384, pipelines/front_ms 368, render/thread/ms 66.60, render/busy/ms 65.09, main/busy/ms 63.39, stage/main/update 42.96, stream/drive/ms 34.30, stage/render/prepare_assets 24.64, pipelines/wait_ms 22.83, stream/spawn/ms 20.33, pipelines/driver_ms 16.50, stage/main/post_update 12.82 |
| warm-up | 5.42 | 38.9 | pipelines/compile_ms 989, pipelines/front_ms 958, pipelines/wait_ms 183, render/thread/ms 63.94, render/busy/ms 62.44, main/busy/ms 52.69, stage/main/update 35.73, pipelines/driver_ms 31.71, stage/render/prepare_assets 27.91, stream/drive/ms 23.97, stream/spawn/ms 20.71, stage/render/render 12.47 |
| warm-up | 5.49 | 45.2 | pipelines/compile_ms 470, pipelines/front_ms 454, render/thread/ms 50.06, main/busy/ms 48.55, render/busy/ms 48.43, stage/main/update 26.29, stage/render/prepare_assets 17.44, pipelines/driver_ms 16.78, stage/main/post_update 14.73, stage/render/render 12.59, render/graph/ms 10.37, stage/render/graph_submit 6.87 |
| warm-up | 5.52 | 34.9 | pipelines/compile_ms 1156, pipelines/front_ms 1118, render/thread/ms 59.59, render/busy/ms 57.83, main/busy/ms 40.49, pipelines/driver_ms 38.13, stage/render/prepare_assets 21.07, stage/render/render 17.81, stage/main/update 17.31, stage/main/post_update 15.64, render/graph/ms 15.30, stage/render/graph_submit 11.24 |
| warm-up | 5.56 | 31.7 | pipelines/compile_ms 1254, pipelines/front_ms 1216, render/thread/ms 51.42, render/busy/ms 49.87, pipelines/driver_ms 38.13, main/busy/ms 34.65, stage/render/prepare_assets 21.19, stage/main/update 16.91, stage/render/render 16.06, render/graph/ms 13.92, stage/main/post_update 10.92, stage/render/graph_submit 10.30 |
| warm-up | 5.59 | 34.2 | pipelines/compile_ms 630, pipelines/front_ms 611, render/thread/ms 50.47, render/busy/ms 48.40, main/busy/ms 36.99, stage/render/prepare_assets 24.18, pipelines/driver_ms 19.36, stage/main/update 18.52, stage/main/post_update 12.21, stage/render/render 12.19, render/graph/ms 9.73, stage/render/graph_submit 6.71 |
| warm-up | 5.64 | 32.9 | render/thread/ms 46.37, render/busy/ms 44.69, main/busy/ms 37.16, stage/main/update 20.52, stage/render/render 15.54, render/graph/ms 13.22, stage/main/post_update 11.19, stage/render/graph_submit 10.09, stage/render/prepare_write_phase_buffers 7.11, encode//sum 6.56, stage/render/prepare_assets 5.71, encode/submit 5.37 |
| warm-up | 5.70 | 40.2 | pipelines/compile_ms 372, pipelines/front_ms 362, render/thread/ms 53.85, render/busy/ms 52.42, main/busy/ms 46.23, stage/main/post_update 21.88, stage/main/update 18.82, stage/render/prepare_assets 14.05, stage/render/render 13.91, render/graph/ms 11.99, stream/drive/ms 11.20, bigspace/total/ms 10.35 |
| warm-up | 5.76 | 44.4 | pipelines/compile_ms 1036, pipelines/front_ms 1013, render/thread/ms 46.05, render/busy/ms 44.82, main/busy/ms 40.67, pipelines/driver_ms 22.62, stage/render/prepare_assets 20.32, stage/main/post_update 18.27, stage/main/update 15.15, stage/render/render 10.57, encode//sum 8.86, render/graph/ms 8.81 |
| warm-up | 5.87 | 53.0 | pipelines/compile_ms 304, pipelines/front_ms 295, render/thread/ms 56.43, render/busy/ms 54.69, main/busy/ms 52.09, stage/render/prepare_assets 28.88, stage/main/update 27.98, stage/render/render 18.69, pipelines/wait_ms 17.18, pipelines/lock_ms 16.79, render/graph/ms 16.44, stage/main/post_update 15.40 |
| warm-up | 5.91 | 32.9 | pipelines/compile_ms 95.69, pipelines/front_ms 91.55, render/thread/ms 66.70, render/busy/ms 65.47, stage/render/prepare_assets 42.54, main/busy/ms 38.98, pipelines/wait_ms 16.77, pipelines/lock_ms 16.44, stage/main/update 15.07, stage/main/post_update 14.39, stage/render/render 11.39, render/graph/ms 9.23 |
| warm-up | 5.93 | 26.0 | render/thread/ms 52.69, render/busy/ms 51.63, main/busy/ms 31.74, stage/render/prepare_assets 29.30, stage/main/update 12.77, stage/main/post_update 12.28, stage/render/render 9.76, render/graph/ms 7.97, encode//sum 7.27, stage/render/graph_submit 5.16, stream/drive/ms 3.68, encode/submit 2.87 |
| warm-up | 5.96 | 33.1 | render/thread/ms 53.62, render/busy/ms 52.00, main/busy/ms 33.08, stage/render/prepare_assets 20.34, stage/render/render 15.35, stage/main/post_update 14.17, render/graph/ms 13.24, stage/main/update 12.68, encode//sum 10.56, pipelines/compile_ms 9.48, stage/render/graph_submit 9.46, pipelines/front_ms 9.11 |
| warm-up | 6.01 | 41.0 | render/thread/ms 66.37, render/busy/ms 64.58, main/busy/ms 42.67, stage/render/prepare_assets 28.40, stage/main/post_update 18.22, stage/render/render 16.33, render/graph/ms 13.80, stage/main/update 11.91, encode//sum 11.00, stage/render/graph_submit 10.02, pipelines/compile_ms 9.48, pipelines/front_ms 9.11 |
| warm-up | 6.05 | 44.2 | pipelines/compile_ms 1396, pipelines/front_ms 1363, render/thread/ms 68.34, render/busy/ms 67.01, main/busy/ms 43.55, stage/render/prepare_assets 41.83, pipelines/driver_ms 33.04, pipelines/wait_ms 24.41, pipelines/lock_ms 23.91, stage/main/post_update 16.83, stage/main/update 13.58, stage/render/render 11.72 |
| warm-up | 6.08 | 30.3 | pipelines/compile_ms 1396, pipelines/front_ms 1363, render/thread/ms 59.64, render/busy/ms 58.51, stage/render/prepare_assets 39.54, pipelines/driver_ms 33.04, main/busy/ms 32.66, pipelines/wait_ms 24.41, pipelines/lock_ms 23.91, stage/main/update 13.42, stage/main/post_update 11.92, stage/render/render 9.44 |
| warm-up | 6.11 | 26.5 | render/thread/ms 48.43, render/busy/ms 47.42, main/busy/ms 31.80, stage/render/prepare_assets 28.21, stage/main/update 13.17, stage/main/post_update 12.19, stage/render/render 8.44, render/graph/ms 6.75, encode//sum 6.41, stage/render/graph_submit 4.54, stream/drive/ms 4.39, stage/main/pre_update 2.69 |
| warm-up | 6.13 | 27.6 | render/thread/ms 45.83, render/busy/ms 44.72, main/busy/ms 37.61, stage/render/prepare_assets 22.42, stage/main/post_update 16.49, stage/main/update 15.68, stage/render/render 10.38, render/graph/ms 8.56, encode//sum 7.04, stream/drive/ms 6.58, stage/render/graph_submit 6.08, encode/finish_wall 3.04 |
| warm-up | 6.16 | 29.5 | pipelines/compile_ms 137, pipelines/front_ms 132, render/thread/ms 48.83, render/busy/ms 47.47, main/busy/ms 40.84, stage/render/prepare_assets 26.49, stage/main/update 20.17, stage/main/post_update 15.60, stage/render/render 10.64, render/graph/ms 8.72, encode//sum 6.95, stage/render/graph_submit 6.31 |
| warm-up | 6.20 | 36.7 | pipelines/compile_ms 137, pipelines/front_ms 132, render/thread/ms 53.70, render/busy/ms 52.11, main/busy/ms 42.22, stage/render/prepare_assets 24.17, stage/main/update 19.81, stage/main/post_update 15.11, stage/render/render 14.01, render/graph/ms 11.97, encode//sum 8.07, stage/render/graph_submit 7.85 |
| warm-up | 6.28 | 56.9 | main/busy/ms 50.35, render/thread/ms 42.23, render/busy/ms 40.93, stage/main/update 27.37, stage/render/prepare_assets 19.14, stream/drive/ms 17.96, pipelines/compile_ms 15.33, stage/main/post_update 15.18, pipelines/front_ms 13.99, stage/render/render 11.45, render/graph/ms 9.63, encode//sum 8.19 |
| warm-up | 6.32 | 38.8 | render/thread/ms 55.77, render/busy/ms 54.52, main/busy/ms 48.76, pipelines/compile_ms 27.97, stage/render/prepare_assets 26.09, pipelines/front_ms 25.07, stage/main/update 24.27, stage/main/post_update 15.52, stage/render/render 13.24, render/graph/ms 11.28, stream/drive/ms 9.17, encode//sum 8.77 |
| warm-up | 6.35 | 30.9 | pipelines/compile_ms 773, pipelines/front_ms 752, render/thread/ms 51.15, render/busy/ms 49.76, main/busy/ms 48.43, stage/main/update 25.14, stage/render/prepare_assets 23.82, pipelines/driver_ms 21.68, stage/main/post_update 15.01, stage/render/render 13.19, render/graph/ms 11.32, stream/drive/ms 8.48 |
| warm-up | 6.51 | 37.6 | pipelines/compile_ms 169, pipelines/front_ms 165, main/busy/ms 43.55, render/thread/ms 30.14, render/busy/ms 29.10, stage/main/update 27.35, stream/drive/ms 18.69, stage/render/render 12.57, render/graph/ms 10.97, stage/main/post_update 10.41, stage/render/prepare_assets 8.39, encode//sum 7.83 |
| warm-up | 6.60 | 28.6 | main/busy/ms 35.14, render/thread/ms 29.73, render/busy/ms 28.04, stage/main/update 18.52, stage/render/render 13.05, stage/main/post_update 11.49, render/graph/ms 10.83, stage/render/prepare_assets 9.89, encode//sum 8.61, stage/render/graph_submit 7.93, encode/submit 4.46, encode/finish_wall 3.49 |
| warm-up | 6.84 | 28.4 | main/busy/ms 42.13, render/thread/ms 36.97, render/busy/ms 34.66, stage/main/update 19.25, stage/render/render 17.05, stage/main/post_update 15.74, render/graph/ms 13.88, encode//sum 10.60, stream/drive/ms 10.33, stage/render/prepare_assets 10.18, stage/render/graph_submit 9.91, encode/submit 5.92 |
| warm-up | 6.88 | 44.3 | render/thread/ms 49.17, render/busy/ms 47.74, main/busy/ms 43.36, stage/render/prepare_assets 28.05, stage/main/update 21.32, stage/main/post_update 14.93, stream/drive/ms 11.76, stage/render/render 11.60, render/graph/ms 9.55, encode//sum 8.00, stage/render/graph_submit 6.17, encode/submit 3.28 |
| warm-up | 7.23 | 31.5 | render/thread/ms 37.31, render/busy/ms 36.27, main/busy/ms 15.93, stage/render/render 9.39, stage/render/specialize 8.97, stage/main/post_update 8.90, render/graph/ms 8.00, encode//sum 6.26, stage/render/graph_submit 6.07, stage/render/queue 5.57, stage/main/update 4.61, stage/render/prepare_meshes 4.17 |
| warm-up | 8.45 | 29.0 | render/thread/ms 35.59, render/busy/ms 32.77, main/busy/ms 14.21, stage/render/render 9.72, stage/render/specialize 8.79, render/graph/ms 8.44, stage/main/post_update 8.25, stage/render/graph_submit 6.42, encode//sum 5.86, stage/render/queue 5.30, stage/render/prepare_meshes 4.24, stage/main/update 3.75 |
| measure | 0.85 | 27.6 | render/thread/ms 33.95, render/busy/ms 32.46, main/busy/ms 13.90, stage/render/render 10.87, render/graph/ms 9.59, encode//sum 9.15, stage/render/specialize 8.65, stage/main/post_update 7.74, stage/render/graph_submit 7.63, stage/render/queue 5.83, encode/submit 4.68, stage/main/update 3.89 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set shadow.lamp.day_gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=daygate-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 7%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 95%) · 2625 MHz (min 2625) · memory 8101 MHz · 97.3 W · 78 °C (max 80) · VRAM 6429 MiB · throttled 100% of samples

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
- `shadow.lamp.day_gate=on (command line: --set)`
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
