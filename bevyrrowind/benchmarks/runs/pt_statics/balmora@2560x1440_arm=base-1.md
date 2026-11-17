# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1546 frames over 15.0 s · **median 9.68 ms (103 fps)** · mean 9.71 · p95 11.10 · p99 11.83 (1% low 85 fps) · worst 43.31 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 8.0/min)
- warm-up: 279 frames over 8.3 s, worst 2462.2 ms
- hitches (> 25.0 ms): **2** measured, 58 in warm-up
- timed GPU passes: 8.51 ms of a 9.71 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.68 ms, limit 11 ms
- **FAIL** worst frame 43.31 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.874 | 2.894 | 2.958 | 0.99 | 0.064 |
| dlss_super_resolution | 2.006 | 2.021 | 2.044 | 0.99 | 0.186 |
| early prepass | 0.613 | 0.618 | 0.633 | 0.99 | 0.097 |
| atmosphere_luts | 0.332 | 0.334 | 0.395 | 0.99 | 0.003 |
| bloom | 0.312 | 0.314 | 0.330 | 0.99 | 0.064 |
| main_transmissive_pass_3d | 0.309 | 0.311 | 0.334 | 0.99 | 0.009 |
| water_velocity | 0.238 | 0.240 | 0.265 | 0.99 | 0.002 |
| water_sspr | 0.215 | 0.217 | 0.214 | 0.99 | 0.002 |
| ssao | 0.188 | 0.189 | 0.197 | 0.99 | 0.007 |
| clouds_trace | 0.147 | 0.149 | 0.169 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.144 | 0.145 | 0.152 | 0.99 | 0.001 |
| water_fft | 0.132 | 0.133 | 0.136 | 0.99 | 0.003 |
| uniform_allocation | 0.118 | 0.119 | 0.162 | 0.99 | 0.022 |
| upscaling | 0.113 | 0.114 | 0.120 | 0.99 | 0.004 |
| clouds_composite | 0.110 | 0.111 | 0.113 | 0.99 | 0.002 |
| bin_unpacking | 0.090 | 0.090 | 0.123 | 0.99 | 0.016 |
| tonemapping | 0.086 | 0.086 | 0.091 | 0.99 | 0.004 |
| clustering | 0.064 | 0.064 | 0.075 | 0.99 | 0.021 |
| render_sky | 0.052 | 0.052 | 0.053 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.412 | 0.414 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.99 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 0.99 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.228 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 0.99 | 0.086 |
| water_snapshot | 0.025 | 0.025 | 0.026 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| surface_exposure_far | 0.014 | 0.043 | 0.046 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 0.99 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.740 | 10.136 | 42.24 | 13512.4 |
| main/busy/ms | 7.413 | 8.287 | 39.70 | 11460.4 |
| render/busy/ms | 7.203 | 8.375 | 41.04 | 11136.6 |
| stage/render/render | 4.818 | 5.811 | 39.09 | 7447.9 |
| stage/main/post_update | 4.367 | 5.102 | 36.61 | 6750.8 |
| render/graph/ms | 4.224 | 5.170 | 37.74 | 6530.1 |
| encode//sum | 4.016 | 5.199 | 261.56 | 6209.4 |
| stage/render/graph_submit | 3.204 | 4.118 | 36.73 | 4952.9 |
| encode/submit | 2.031 | 2.604 | 5.32 | 3139.3 |
| stage/main/update | 1.896 | 2.359 | 3.85 | 2931.7 |
| stage/render/prepare_views | 1.217 | 2.794 | 4.72 | 1881.4 |
| encode/finish_wall | 1.199 | 1.695 | 33.76 | 1853.9 |
| render/acquire/ms | 1.169 | 2.731 | 4.67 | 1807.4 |
| stage/render/camera_driver | 0.964 | 1.117 | 1.80 | 1489.6 |
| encode/pass:shadow_directional_light/sum | 0.750 | 1.325 | 2.25 | 1159.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.731 | 1.008 | 33.46 | 1130.3 |
| encode/pass:early prepass/sum | 0.704 | 1.010 | 1.60 | 1089.0 |
| stage/render/prepare_write_phase_buffers | 0.558 | 0.775 | 1.01 | 862.0 |
| stage/render/prepare_assets | 0.468 | 0.604 | 1.54 | 724.1 |
| stage/render/prepare_resources | 0.399 | 0.463 | 0.66 | 616.8 |
| bigspace/total/ms | 0.397 | 0.947 | 1.88 | 613.9 |
| render/present/ms | 0.368 | 0.447 | 1.90 | 568.4 |
| stage/main/pre_update | 0.357 | 0.491 | 0.95 | 551.9 |
| stage/main/first | 0.302 | 0.377 | 1.50 | 466.1 |
| encode/pass:main_transparent_pass_3d/sum | 0.243 | 0.361 | 0.69 | 375.6 |
| terrain/update/ms | 0.238 | 0.305 | 0.86 | 367.3 |
| stage/render/prepare_bind_groups | 0.214 | 0.303 | 0.49 | 330.4 |
| terrain/select/ms | 0.207 | 0.266 | 0.79 | 319.3 |
| stage/render/prepare_meshes | 0.202 | 0.262 | 2.05 | 312.8 |
| stream/drive/ms | 0.197 | 0.280 | 0.65 | 303.9 |
| stage/main/fixed_main_loop | 0.190 | 0.384 | 0.88 | 293.2 |
| stage/render/prepare_batch_phases | 0.184 | 0.249 | 0.37 | 284.0 |
| far/select/ms | 0.180 | 0.252 | 0.75 | 277.8 |
| stage/render/create_views | 0.178 | 0.238 | 0.74 | 275.7 |
| stage/render/extract_commands | 0.173 | 0.205 | 0.66 | 268.2 |
| stream/tick/ms | 0.166 | 0.242 | 0.62 | 257.1 |
| controller/feed/ms | 0.164 | 0.215 | 0.58 | 253.5 |
| stage/render/prepare_flush | 0.155 | 0.271 | 0.57 | 240.2 |
| bigspace/high_precision/ms | 0.152 | 0.612 | 1.63 | 234.6 |
| controller/move/ms | 0.142 | 0.318 | 0.66 | 219.0 |
| stage/main/last | 0.117 | 0.154 | 0.77 | 180.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.116 | 0.179 | 0.27 | 179.2 |
| stage/render/queue | 0.112 | 0.144 | 5.77 | 173.3 |
| bigspace/low_precision/ms | 0.092 | 0.206 | 1.27 | 142.3 |
| stage/render/specialize | 0.088 | 0.111 | 8.16 | 135.9 |
| anim/evaluate/ms | 0.086 | 0.198 | 1.26 | 133.6 |
| anim/evaluate_cpu/ms | 0.061 | 0.077 | 0.45 | 93.7 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.111 | 0.19 | 89.4 |
| encode/resolve_diagnostics/sum | 0.054 | 0.082 | 0.46 | 83.6 |
| physics/step/ms | 0.028 | 0.057 | 0.41 | 43.9 |
| weather/map/ms | 0.026 | 0.034 | 0.46 | 40.6 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.041 | 0.23 | 40.6 |
| stage/render/cleanup | 0.023 | 0.032 | 0.31 | 35.6 |
| encode/camera_driver_span_end/sum | 0.023 | 0.038 | 0.32 | 35.3 |
| stage/render/phase_sort | 0.019 | 0.035 | 0.12 | 29.8 |
| weather/eval/ms | 0.012 | 0.016 | 0.04 | 18.0 |
| script/update/ms | 0.007 | 0.011 | 0.02 | 11.1 |
| script/input/ms | 0.006 | 0.007 | 0.11 | 8.6 |
| controller/avoid/ms | 0.006 | 0.007 | 0.02 | 8.5 |
| script/ui/ms | 0.005 | 0.007 | 0.02 | 7.6 |
| anim/apply/ms | 0.005 | 0.006 | 0.02 | 7.2 |
| controller/write/ms | 0.002 | 0.003 | 0.02 | 3.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 0.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.6 |
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
| encode//max | ms | 0.54 | 32.72 | 0.47 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.23 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.32 | 0.03 |
| encode/finish_critical | ms | 0.85 | 33.46 | 0.89 |
| encode/pass:early prepass/max | ms | 0.70 | 1.60 | 0.89 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.71 | 33.46 | 0.63 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.27 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.69 | 0.38 |
| encode/pass:shadow_directional_light/max | ms | 0.69 | 2.25 | 0.66 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.46 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.21 | 41.04 | 9.29 |
| gpu/timed/ms | ms | 9.20 | 10.80 | 9.29 |
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
| process/cpu_usage | % | 33.33 | 48.08 | 38.94 |
| process/mem_usage | GiB | 4.75 | 4.81 | 4.79 |
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
| skin/blocks_changed/count | count | 4 | 7 | 4 |
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
| system/cpu_usage | % | 38.99 | 52.52 | 40.35 |
| system/mem_usage | % | 54.58 | 54.74 | 54.63 |
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
| anim/events/count | count | 0.10 | 3 | 155 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.20 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 29 | 41.1 k |
| encode/bind_walk/init_textures | count | 7164 | 14.0 k | 11.08 M |
| encode/bind_walk/init_textures_skipped | count | 35.6 k | 48.7 k | 55.04 M |
| encode/bind_walk/inits | count | 345 | 437 | 533.9 k |
| encode/bind_walk/inits_skipped | count | 66.99 | 79 | 103.6 k |
| encode/bind_walk/merge_textures | count | 7101 | 13.9 k | 10.98 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.31 M |
| encode/bind_walk/merges | count | 82.25 | 103 | 127.2 k |
| encode/bind_walk/merges_skipped | count | 50.75 | 56 | 78.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1546 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1546 |
| encode/pass:early prepass/count | count | 1 | 1 | 1546 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1546 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1546 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1546 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1546 |
| encode/queue/command_buffers | count | 36.57 | 39 | 56.5 k |
| encode/queue/submits | count | 2 | 2 | 3092 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.86 M | 2.22 G |
| encode/queue/write_buffer/calls | count | 217 | 288 | 335.9 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.46 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.9 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1546 |
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

## Hitches (> 25.0 ms; slowest 60, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.46 | 2462.2 | main/busy/ms 2433, stage/main/post_update 2328, stage/main/update 96.33, surface/advance_cpu_ms 67.03, stage/main/pre_update 4.39, stage/main/last 1.02, surface/cell_upload_ms 0.79, stage/main/first 0.58, stage/main/fixed_main_loop 0.38, stream/drive/ms 0.33, stream/tick/ms 0.10, weather/map/ms 0.09 |
| warm-up | 3.19 | 728.4 | main/busy/ms 2566, stage/main/post_update 2350, render/thread/ms 691, render/busy/ms 526, stage/render/prepare_assets 477, stage/main/update 202, stage/render/prepare_views 162, render/acquire/ms 162, surface/advance_cpu_ms 67.03, stage/render/create_views 27.89, stage/render/render 12.71, render/graph/ms 8.35 |
| warm-up | 3.31 | 115.3 | render/thread/ms 790, render/busy/ms 566, stage/render/prepare_assets 503, stage/render/prepare_views 163, render/acquire/ms 162, main/busy/ms 153, stage/main/update 109, stage/render/render 76.95, render/present/ms 61.05, stage/main/post_update 33.34, stage/render/create_views 28.60, pipelines/compile_ms 22.95 |
| warm-up | 3.35 | 48.5 | pipelines/compile_ms 365, pipelines/front_ms 285, pipelines/wait_ms 197, render/thread/ms 146, stage/render/render 106, render/busy/ms 86.71, pipelines/driver_ms 80.38, main/busy/ms 66.68, render/present/ms 58.85, stage/main/post_update 52.08, render/graph/ms 45.88, stage/render/graph_submit 39.32 |
| warm-up | 3.46 | 107.9 | pipelines/wait_ms 3835, pipelines/compile_ms 1179, pipelines/front_ms 1054, render/thread/ms 153, render/busy/ms 150, pipelines/driver_ms 125, main/busy/ms 115, stage/render/render 83.17, render/graph/ms 75.14, stage/render/graph_submit 68.46, stage/main/post_update 54.79, encode/finish_wall 54.70 |
| warm-up | 3.52 | 60.3 | pipelines/wait_ms 5585, pipelines/compile_ms 1767, pipelines/front_ms 1666, render/thread/ms 164, render/busy/ms 162, pipelines/driver_ms 100, main/busy/ms 87.98, stage/render/prepare_assets 55.75, stage/main/update 53.40, stage/render/render 51.61, encode//sum 50.19, render/graph/ms 43.89 |
| warm-up | 3.57 | 45.6 | pipelines/wait_ms 2042, pipelines/compile_ms 1860, pipelines/front_ms 1764, render/thread/ms 102, render/busy/ms 99.99, pipelines/driver_ms 96.78, stage/render/prepare_assets 62.47, main/busy/ms 49.42, stage/main/post_update 23.92, stage/main/update 19.06, terrain/update/ms 15.02, stage/render/render 14.65 |
| warm-up | 3.60 | 33.3 | pipelines/compile_ms 1073, pipelines/front_ms 1024, pipelines/wait_ms 228, render/thread/ms 73.63, render/busy/ms 71.94, pipelines/driver_ms 48.83, main/busy/ms 45.22, stage/render/prepare_assets 44.31, stage/main/post_update 21.82, stage/main/update 18.73, terrain/update/ms 10.17, stage/render/prepare_resources 9.92 |
| warm-up | 3.64 | 40.8 | pipelines/compile_ms 203, pipelines/front_ms 194, pipelines/wait_ms 134, render/thread/ms 69.94, render/busy/ms 67.82, stage/render/prepare_assets 31.26, main/busy/ms 29.28, stage/main/post_update 16.77, stage/render/render 16.29, encode//sum 15.10, render/graph/ms 14.00, stage/render/graph_submit 10.70 |
| warm-up | 3.67 | 28.2 | pipelines/compile_ms 324, pipelines/front_ms 312, render/thread/ms 66.16, render/busy/ms 64.06, main/busy/ms 36.95, stage/render/render 19.67, stage/main/post_update 18.98, encode//sum 18.27, render/graph/ms 17.19, pipelines/wait_ms 16.07, stage/render/graph_submit 13.63, stage/render/prepare_assets 12.94 |
| warm-up | 3.71 | 43.3 | pipelines/compile_ms 872, pipelines/front_ms 833, pipelines/wait_ms 136, render/thread/ms 68.04, render/busy/ms 66.35, pipelines/driver_ms 39.39, main/busy/ms 38.77, stage/render/prepare_assets 34.50, stage/main/post_update 20.11, stage/render/render 14.15, render/graph/ms 11.76, stage/main/update 10.10 |
| warm-up | 3.75 | 34.1 | pipelines/compile_ms 1701, pipelines/front_ms 1616, pipelines/wait_ms 171, pipelines/driver_ms 85.22, render/thread/ms 73.10, render/busy/ms 70.57, main/busy/ms 62.00, stage/main/update 46.28, stage/render/prepare_assets 44.61, stream/drive/ms 39.57, stage/render/render 19.90, render/graph/ms 16.83 |
| warm-up | 3.80 | 57.1 | pipelines/compile_ms 1526, pipelines/front_ms 1462, render/thread/ms 67.40, render/busy/ms 64.96, pipelines/driver_ms 64.61, main/busy/ms 55.46, pipelines/wait_ms 50.46, stage/main/update 44.32, stream/drive/ms 40.07, stage/render/prepare_assets 39.19, stage/render/render 19.74, render/graph/ms 16.88 |
| warm-up | 3.87 | 43.2 | pipelines/compile_ms 530, pipelines/front_ms 513, render/thread/ms 50.55, render/busy/ms 48.64, main/busy/ms 37.52, stage/main/update 23.23, stage/render/prepare_assets 21.69, stage/render/render 21.68, stream/drive/ms 19.21, render/graph/ms 19.17, pipelines/driver_ms 17.40, stage/render/graph_submit 15.87 |
| warm-up | 3.91 | 37.6 | pipelines/compile_ms 216, pipelines/front_ms 209, render/thread/ms 58.78, render/busy/ms 56.74, main/busy/ms 39.17, stage/render/prepare_assets 36.11, stage/main/update 23.38, stream/drive/ms 17.78, stage/render/render 14.84, render/graph/ms 12.35, encode//sum 8.70, stage/render/graph_submit 8.61 |
| warm-up | 4.00 | 97.0 | pipelines/compile_ms 406, pipelines/front_ms 392, render/thread/ms 87.56, render/busy/ms 84.44, stage/render/prepare_assets 57.46, main/busy/ms 50.26, stage/main/update 31.66, stream/drive/ms 19.60, stage/render/render 19.60, render/graph/ms 15.96, pipelines/driver_ms 14.16, stage/render/graph_submit 12.13 |
| warm-up | 4.07 | 64.6 | pipelines/compile_ms 1202, pipelines/front_ms 1166, render/thread/ms 98.25, render/busy/ms 95.49, main/busy/ms 94.91, stage/main/update 77.71, stage/render/prepare_assets 68.58, stream/drive/ms 51.13, pipelines/driver_ms 35.94, stage/render/render 17.74, render/graph/ms 14.28, stage/render/graph_submit 11.32 |
| warm-up | 4.22 | 149.5 | pipelines/compile_ms 1320, pipelines/front_ms 1284, render/thread/ms 119, render/busy/ms 117, stage/render/prepare_assets 101, main/busy/ms 99.81, stage/main/update 81.67, stream/drive/ms 50.58, pipelines/driver_ms 36.69, pipelines/wait_ms 10.39, pipelines/lock_ms 10.01, stage/render/render 9.65 |
| warm-up | 4.26 | 47.0 | pipelines/compile_ms 583, pipelines/front_ms 552, render/thread/ms 121, render/busy/ms 119, stage/render/prepare_assets 86.76, main/busy/ms 58.35, pipelines/driver_ms 31.46, stage/main/update 28.20, stage/render/render 22.40, render/graph/ms 20.21, stage/main/post_update 19.16, stage/render/graph_submit 18.12 |
| warm-up | 4.31 | 43.5 | pipelines/compile_ms 161, pipelines/front_ms 139, render/thread/ms 78.97, render/busy/ms 76.95, main/busy/ms 61.28, pipelines/wait_ms 46.45, stage/main/update 27.10, stage/render/render 23.43, pipelines/driver_ms 21.95, render/graph/ms 21.24, stage/main/post_update 19.44, stage/render/prepare_assets 19.05 |
| warm-up | 4.36 | 48.5 | pipelines/wait_ms 272, pipelines/compile_ms 163, pipelines/front_ms 155, render/thread/ms 78.28, render/busy/ms 76.80, main/busy/ms 61.81, stage/main/update 31.13, stage/render/prepare_assets 30.53, stage/main/post_update 18.35, stage/render/prepare_resources 17.34, stage/render/render 9.55, encode//sum 8.73 |
| warm-up | 4.39 | 35.0 | pipelines/wait_ms 229, render/thread/ms 65.16, render/busy/ms 63.87, pipelines/compile_ms 61.74, pipelines/front_ms 59.20, main/busy/ms 48.00, stage/render/prepare_assets 38.22, stage/main/post_update 22.18, stage/main/update 20.56, stage/render/render 9.29, encode//sum 7.72, render/graph/ms 7.51 |
| warm-up | 4.45 | 55.1 | pipelines/compile_ms 1422, pipelines/front_ms 1378, pipelines/wait_ms 324, render/thread/ms 79.62, render/busy/ms 78.31, pipelines/driver_ms 43.32, main/busy/ms 43.22, stage/render/prepare_write_phase_buffers 33.47, stage/main/post_update 22.43, stage/render/prepare_assets 16.03, stage/main/update 14.55, stage/render/prepare_bind_groups 11.25 |
| warm-up | 4.48 | 30.9 | pipelines/compile_ms 2397, pipelines/front_ms 2324, pipelines/wait_ms 1107, render/thread/ms 78.62, render/busy/ms 77.21, pipelines/driver_ms 72.38, main/busy/ms 46.18, stage/render/prepare_write_phase_buffers 33.65, stage/main/post_update 24.86, stage/main/update 13.63, stage/render/prepare_bind_groups 11.03, stage/render/render 10.79 |
| warm-up | 4.51 | 28.2 | pipelines/compile_ms 1034, pipelines/front_ms 1002, pipelines/wait_ms 929, main/busy/ms 72.28, stage/main/update 49.98, render/thread/ms 45.02, stream/drive/ms 44.07, render/busy/ms 43.37, pipelines/driver_ms 31.54, stage/main/post_update 15.36, stage/render/prepare_assets 14.43, stage/render/render 11.71 |
| warm-up | 4.55 | 49.2 | pipelines/compile_ms 527, pipelines/front_ms 507, pipelines/wait_ms 222, main/busy/ms 113, stage/main/update 101, stream/drive/ms 94.13, render/thread/ms 33.38, render/busy/ms 31.90, pipelines/driver_ms 19.51, stage/render/prepare_assets 13.40, stage/render/render 10.47, stream/spawn/ms 10.23 |
| warm-up | 4.64 | 85.1 | pipelines/compile_ms 1245, pipelines/front_ms 1203, pipelines/wait_ms 372, main/busy/ms 77.38, stage/main/update 60.45, stream/drive/ms 54.22, pipelines/driver_ms 42.27, render/thread/ms 40.60, render/busy/ms 39.25, stage/render/prepare_assets 22.44, stream/spawn/ms 10.75, stage/render/render 9.42 |
| warm-up | 4.72 | 64.0 | pipelines/compile_ms 1322, pipelines/front_ms 1277, pipelines/wait_ms 201, main/busy/ms 63.13, pipelines/driver_ms 44.75, stage/main/update 43.92, render/thread/ms 42.57, render/busy/ms 40.70, stage/render/prepare_assets 18.12, stage/render/render 12.36, stream/drive/ms 12.09, stage/main/post_update 10.63 |
| warm-up | 4.76 | 46.0 | pipelines/compile_ms 1365, pipelines/front_ms 1318, render/thread/ms 64.46, render/busy/ms 62.94, main/busy/ms 49.28, pipelines/driver_ms 46.74, stage/render/prepare_assets 35.47, stage/main/update 33.40, stream/drive/ms 12.55, stage/render/render 11.10, stage/main/post_update 9.39, render/graph/ms 9.15 |
| warm-up | 4.90 | 27.7 | main/busy/ms 42.76, render/thread/ms 41.59, render/busy/ms 40.00, pipelines/compile_ms 30.07, pipelines/front_ms 28.76, stage/main/update 23.59, stage/render/render 19.13, render/graph/ms 16.84, stage/main/post_update 13.64, stage/render/graph_submit 13.35, encode//sum 8.26, encode/submit 6.24 |
| warm-up | 4.94 | 40.1 | pipelines/compile_ms 1233, pipelines/front_ms 1189, main/busy/ms 48.20, render/thread/ms 46.49, render/busy/ms 44.93, pipelines/driver_ms 44.24, stage/main/update 25.76, stage/render/render 17.27, stage/main/post_update 16.72, stage/render/prepare_assets 15.07, render/graph/ms 15.03, stage/render/graph_submit 11.89 |
| warm-up | 4.98 | 38.0 | pipelines/compile_ms 1474, pipelines/front_ms 1422, pipelines/driver_ms 52.25, render/thread/ms 51.93, render/busy/ms 50.55, main/busy/ms 37.92, stage/render/prepare_assets 23.57, stage/main/update 16.04, stage/main/post_update 15.55, stage/render/render 11.80, render/graph/ms 9.92, encode//sum 7.74 |
| warm-up | 5.01 | 31.6 | pipelines/compile_ms 514, pipelines/front_ms 499, render/thread/ms 57.17, render/busy/ms 55.46, main/busy/ms 44.99, stage/render/render 23.19, stage/main/post_update 22.47, render/graph/ms 21.06, stage/render/prepare_assets 18.10, stage/render/graph_submit 16.79, stage/main/update 15.33, pipelines/driver_ms 15.31 |
| warm-up | 5.05 | 41.0 | pipelines/compile_ms 610, pipelines/front_ms 593, render/thread/ms 59.10, render/busy/ms 57.41, main/busy/ms 45.94, stage/render/prepare_assets 26.19, stage/main/post_update 24.34, pipelines/wait_ms 23.81, stage/render/render 21.01, render/graph/ms 18.93, pipelines/driver_ms 16.92, stage/render/graph_submit 15.26 |
| warm-up | 5.10 | 45.7 | pipelines/compile_ms 592, pipelines/front_ms 575, render/thread/ms 68.17, render/busy/ms 66.75, main/busy/ms 36.42, stage/render/prepare_assets 35.23, pipelines/wait_ms 23.34, stage/main/post_update 18.02, pipelines/driver_ms 17.38, stage/main/update 11.29, stage/render/render 11.21, render/graph/ms 9.10 |
| warm-up | 5.14 | 42.3 | pipelines/compile_ms 419, pipelines/front_ms 408, render/thread/ms 78.07, render/busy/ms 76.38, main/busy/ms 37.58, stage/main/post_update 18.63, stage/render/prepare_assets 17.62, stage/main/update 13.66, stage/render/prepare_resources 13.32, stage/render/render 12.61, stage/render/prepare_write_phase_buffers 10.69, pipelines/driver_ms 10.68 |
| warm-up | 5.20 | 57.4 | pipelines/compile_ms 1336, pipelines/front_ms 1302, render/thread/ms 96.58, render/busy/ms 94.90, main/busy/ms 45.96, pipelines/driver_ms 33.36, stage/render/prepare_resources 25.74, stage/main/post_update 25.14, stage/render/prepare_assets 22.84, stage/main/update 12.80, stage/render/render 11.38, render/graph/ms 9.32 |
| warm-up | 5.22 | 25.8 | pipelines/compile_ms 1383, pipelines/front_ms 1345, render/thread/ms 80.25, render/busy/ms 78.63, pipelines/wait_ms 57.21, main/busy/ms 56.75, pipelines/driver_ms 37.95, stage/main/post_update 24.25, stage/main/update 23.89, stage/render/prepare_assets 22.03, stage/render/render 19.97, render/graph/ms 17.86 |
| warm-up | 5.26 | 33.4 | pipelines/compile_ms 688, pipelines/front_ms 667, pipelines/wait_ms 61.77, render/thread/ms 50.67, render/busy/ms 48.98, main/busy/ms 45.89, stage/main/update 25.82, pipelines/driver_ms 21.57, stage/render/render 20.82, render/graph/ms 18.62, stage/render/prepare_assets 16.70, stream/drive/ms 13.17 |
| warm-up | 5.33 | 27.0 | pipelines/compile_ms 592, pipelines/front_ms 574, pipelines/wait_ms 67.01, render/thread/ms 46.28, render/busy/ms 44.86, main/busy/ms 30.07, stage/render/prepare_assets 19.79, pipelines/driver_ms 17.65, stage/main/update 11.99, stage/main/post_update 11.53, stage/render/render 11.46, render/graph/ms 9.65 |
| warm-up | 5.39 | 40.3 | pipelines/compile_ms 392, pipelines/front_ms 385, render/thread/ms 53.02, render/busy/ms 51.31, pipelines/wait_ms 41.49, main/busy/ms 36.64, stage/render/prepare_assets 27.55, stage/main/update 18.01, stage/render/render 15.14, render/graph/ms 12.80, stage/main/post_update 11.93, stage/render/graph_submit 9.76 |
| warm-up | 5.42 | 25.0 | pipelines/compile_ms 698, pipelines/front_ms 680, render/thread/ms 55.72, render/busy/ms 54.02, pipelines/wait_ms 50.47, main/busy/ms 36.79, stage/render/prepare_assets 33.50, pipelines/driver_ms 17.70, stage/main/update 15.02, stage/main/post_update 13.08, stage/render/render 11.39, render/graph/ms 9.40 |
| warm-up | 5.47 | 28.3 | render/thread/ms 44.91, render/busy/ms 43.81, main/busy/ms 27.34, stage/render/prepare_assets 22.08, stage/main/update 12.26, stage/main/post_update 9.59, stage/render/render 8.96, encode//sum 7.52, render/graph/ms 7.37, stage/render/graph_submit 4.97, stream/drive/ms 4.06, stage/render/specialize 3.58 |
| warm-up | 5.52 | 31.2 | pipelines/compile_ms 174, pipelines/front_ms 168, render/thread/ms 50.86, render/busy/ms 49.42, main/busy/ms 32.66, stage/render/prepare_assets 20.25, stage/render/render 13.91, stage/main/update 12.89, stage/main/post_update 12.61, render/graph/ms 11.79, encode//sum 8.68, stage/render/graph_submit 7.85 |
| warm-up | 5.56 | 42.7 | render/thread/ms 61.44, render/busy/ms 59.93, main/busy/ms 36.34, stage/render/prepare_assets 36.00, stage/main/update 15.05, stage/main/post_update 13.07, stage/render/render 12.09, render/graph/ms 9.90, encode//sum 7.57, stage/render/graph_submit 6.38, stream/drive/ms 5.78, encode/submit 3.57 |
| warm-up | 5.60 | 39.7 | pipelines/compile_ms 749, pipelines/front_ms 736, render/thread/ms 61.42, render/busy/ms 60.26, stage/render/prepare_assets 40.99, main/busy/ms 33.28, pipelines/driver_ms 13.28, stage/main/update 13.20, stage/main/post_update 12.92, stage/render/render 9.70, render/graph/ms 7.94, encode//sum 7.40 |
| warm-up | 5.65 | 25.0 | pipelines/compile_ms 175, pipelines/front_ms 172, render/thread/ms 41.74, render/busy/ms 40.15, main/busy/ms 33.50, stage/render/prepare_assets 17.07, stage/main/update 14.41, stage/main/post_update 13.89, stage/render/render 13.46, render/graph/ms 11.47, stage/render/graph_submit 8.66, encode//sum 6.81 |
| warm-up | 5.68 | 32.2 | pipelines/compile_ms 310, pipelines/front_ms 300, render/thread/ms 48.07, render/busy/ms 46.65, main/busy/ms 40.54, stage/render/prepare_assets 24.93, stage/main/update 18.25, stage/main/post_update 14.62, stage/render/render 12.90, render/graph/ms 11.08, pipelines/driver_ms 9.55, stage/render/graph_submit 8.42 |
| warm-up | 5.75 | 47.4 | main/busy/ms 49.69, render/thread/ms 35.57, render/busy/ms 34.11, stage/main/update 30.09, stream/drive/ms 17.29, stage/main/post_update 13.41, stage/render/render 12.80, render/graph/ms 10.39, stage/render/prepare_assets 9.52, encode//sum 8.16, stage/render/graph_submit 7.22, encode/submit 3.73 |
| warm-up | 5.81 | 41.6 | render/thread/ms 41.45, main/busy/ms 41.27, render/busy/ms 39.95, stage/main/update 19.16, stage/render/prepare_assets 17.71, stage/main/post_update 13.13, stage/render/render 11.98, render/graph/ms 9.99, stream/drive/ms 9.69, encode//sum 7.37, stage/render/graph_submit 7.12, encode/submit 5.09 |
| warm-up | 5.85 | 41.4 | pipelines/compile_ms 1182, pipelines/front_ms 1141, render/thread/ms 53.05, render/busy/ms 51.82, main/busy/ms 44.94, pipelines/driver_ms 41.69, stage/render/prepare_assets 31.87, stage/main/update 22.36, stage/main/post_update 14.07, stage/render/render 9.40, encode//sum 9.04, render/graph/ms 7.52 |
| warm-up | 5.90 | 50.5 | pipelines/compile_ms 1182, pipelines/front_ms 1141, render/thread/ms 73.70, render/busy/ms 72.43, stage/render/prepare_assets 52.87, pipelines/driver_ms 41.69, main/busy/ms 39.50, stage/main/update 19.61, stage/main/post_update 12.33, stage/render/render 10.85, render/graph/ms 8.97, encode//sum 8.73 |
| warm-up | 6.04 | 37.6 | pipelines/compile_ms 444, pipelines/front_ms 428, main/busy/ms 50.49, render/thread/ms 34.38, render/busy/ms 32.76, stage/main/update 31.90, stream/drive/ms 23.46, pipelines/driver_ms 16.82, stage/render/render 12.08, stage/main/post_update 11.14, stage/render/prepare_assets 9.96, render/graph/ms 9.94 |
| warm-up | 6.16 | 30.4 | pipelines/compile_ms 368, pipelines/front_ms 359, main/busy/ms 37.89, render/thread/ms 29.00, render/busy/ms 27.54, stage/main/update 17.78, stage/main/post_update 13.88, stage/render/render 12.84, render/graph/ms 10.92, stage/render/prepare_assets 9.55, pipelines/driver_ms 9.46, encode//sum 8.05 |
| warm-up | 6.40 | 37.7 | main/busy/ms 44.60, render/thread/ms 36.97, render/busy/ms 35.32, stage/main/update 20.18, stage/main/post_update 17.68, stage/render/prepare_assets 14.28, stage/render/render 14.00, render/graph/ms 11.73, stream/drive/ms 9.35, stage/render/graph_submit 8.79, encode//sum 8.78, encode/submit 5.53 |
| warm-up | 6.44 | 33.8 | render/thread/ms 45.65, render/busy/ms 44.54, main/busy/ms 40.51, stage/render/prepare_assets 26.67, stage/main/post_update 18.67, stage/main/update 15.09, stage/render/render 10.42, render/graph/ms 8.78, encode//sum 7.77, stage/render/graph_submit 5.96, stream/drive/ms 4.04, bigspace/total/ms 3.49 |
| warm-up | 6.74 | 29.8 | pipelines/compile_ms 128, pipelines/front_ms 125, render/thread/ms 36.79, render/busy/ms 34.83, main/busy/ms 15.64, stage/render/render 10.39, render/graph/ms 9.03, stage/render/specialize 8.90, stage/main/post_update 8.27, stage/render/graph_submit 7.15, encode//sum 6.69, stage/render/queue 6.19 |
| warm-up | 7.95 | 30.0 | render/thread/ms 37.11, render/busy/ms 34.14, main/busy/ms 15.33, stage/render/render 9.96, stage/render/specialize 9.13, stage/main/post_update 9.00, render/graph/ms 8.71, stage/render/graph_submit 6.86, stage/render/queue 5.56, encode//sum 5.36, stage/render/prepare_meshes 4.26, encode/submit 3.98 |
| measure | 0.83 | 25.5 | render/thread/ms 32.86, render/busy/ms 29.74, main/busy/ms 14.15, stage/render/render 9.09, stage/render/specialize 8.24, stage/main/post_update 8.05, render/graph/ms 7.79, encode//sum 6.28, stage/render/graph_submit 5.97, stage/render/queue 5.89, stage/main/update 3.78, encode/submit 3.64 |
| measure | 0.90 | 43.3 | encode//sum 266, render/thread/ms 52.08, render/busy/ms 48.88, main/busy/ms 47.59, stage/render/render 44.52, render/graph/ms 42.49, stage/main/post_update 41.79, stage/render/graph_submit 40.20, encode/finish_wall 34.97, encode/pass:main_opaque_pass_3d/sum 34.04, encode/submit 5.34, stage/main/update 3.41 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=base-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 3%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **97%** (min 68%) · 2625 MHz (min 2625) · memory 8101 MHz · 98.7 W · 80 °C (max 84) · VRAM 6447 MiB · throttled 100% of samples

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
