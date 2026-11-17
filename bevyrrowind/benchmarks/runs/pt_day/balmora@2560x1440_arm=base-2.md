# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1529 frames over 15.0 s · **median 9.75 ms (103 fps)** · mean 9.82 · p95 11.23 · p99 12.37 (1% low 81 fps) · worst 27.26 ms · 7 spikes over 1.5× median · 32.0 stutters/min (work 12.0/min)
- warm-up: 279 frames over 8.6 s, worst 2690.7 ms
- hitches (> 25.0 ms): **1** measured, 56 in warm-up
- timed GPU passes: 8.45 ms of a 9.82 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.75 ms, limit 11 ms
- pass worst frame 27.26 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.859 | 2.899 | 2.918 | 0.99 | 0.080 |
| dlss_super_resolution | 1.993 | 2.021 | 2.044 | 0.99 | 0.186 |
| early prepass | 0.607 | 0.616 | 0.625 | 0.99 | 0.099 |
| atmosphere_luts | 0.333 | 0.337 | 0.394 | 0.99 | 0.004 |
| bloom | 0.313 | 0.318 | 0.333 | 0.99 | 0.063 |
| main_transmissive_pass_3d | 0.307 | 0.311 | 0.332 | 0.99 | 0.009 |
| water_velocity | 0.235 | 0.239 | 0.263 | 0.99 | 0.002 |
| water_sspr | 0.207 | 0.210 | 0.215 | 0.99 | 0.002 |
| ssao | 0.188 | 0.190 | 0.198 | 0.99 | 0.008 |
| clouds_trace | 0.144 | 0.146 | 0.165 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.144 | 0.146 | 0.151 | 0.99 | 0.001 |
| water_fft | 0.129 | 0.131 | 0.135 | 0.99 | 0.003 |
| uniform_allocation | 0.117 | 0.119 | 0.157 | 0.99 | 0.025 |
| clouds_composite | 0.110 | 0.112 | 0.113 | 0.99 | 0.002 |
| upscaling | 0.094 | 0.096 | 0.117 | 0.99 | 0.004 |
| bin_unpacking | 0.090 | 0.091 | 0.122 | 0.99 | 0.018 |
| tonemapping | 0.085 | 0.086 | 0.090 | 0.99 | 0.004 |
| clustering | 0.064 | 0.065 | 0.074 | 0.99 | 0.021 |
| lightprobe_irradiance_map | 0.051 | 0.415 | 0.415 | 0.12 | 0.001 |
| render_sky | 0.051 | 0.052 | 0.052 | 0.99 | 0.002 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 0.99 | 0.001 |
| water_snapshot | 0.033 | 0.033 | 0.035 | 0.99 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.041 | 0.99 | 0.006 |
| lightprobe_radiance_map | 0.027 | 0.217 | 0.227 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.026 | 0.029 | 0.99 | 0.087 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.046 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.013 | 0.029 | 0.99 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.99 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.006 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.015 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.823 | 10.256 | 26.13 | 13489.6 |
| main/busy/ms | 7.565 | 8.400 | 9.94 | 11566.4 |
| render/busy/ms | 7.535 | 8.768 | 25.55 | 11521.2 |
| stage/render/render | 5.085 | 6.102 | 10.84 | 7775.5 |
| render/graph/ms | 4.482 | 5.415 | 10.21 | 6852.4 |
| stage/main/post_update | 4.459 | 5.178 | 6.53 | 6818.4 |
| encode//sum | 4.450 | 5.806 | 9.91 | 6803.8 |
| stage/render/graph_submit | 3.430 | 4.348 | 6.68 | 5245.1 |
| encode/submit | 2.157 | 2.726 | 4.09 | 3297.8 |
| stage/main/update | 1.949 | 2.363 | 3.53 | 2979.4 |
| encode/finish_wall | 1.307 | 1.879 | 2.77 | 1997.9 |
| stage/render/camera_driver | 0.996 | 1.143 | 1.95 | 1522.3 |
| stage/render/prepare_views | 0.956 | 2.454 | 3.47 | 1462.3 |
| render/acquire/ms | 0.911 | 2.405 | 3.40 | 1392.9 |
| encode/pass:shadow_directional_light/sum | 0.848 | 1.479 | 2.39 | 1296.5 |
| encode/pass:early prepass/sum | 0.786 | 1.103 | 1.47 | 1201.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.766 | 1.089 | 1.42 | 1171.6 |
| stage/render/prepare_write_phase_buffers | 0.581 | 0.810 | 1.24 | 888.0 |
| stage/render/prepare_assets | 0.484 | 0.635 | 1.60 | 739.4 |
| bigspace/total/ms | 0.421 | 1.090 | 2.33 | 643.3 |
| stage/render/prepare_resources | 0.404 | 0.469 | 0.80 | 617.6 |
| render/present/ms | 0.376 | 0.453 | 1.90 | 575.6 |
| stage/main/pre_update | 0.362 | 0.501 | 0.91 | 553.2 |
| stage/main/first | 0.302 | 0.380 | 1.29 | 462.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.271 | 0.399 | 0.65 | 414.8 |
| terrain/update/ms | 0.239 | 0.314 | 0.76 | 365.0 |
| stage/render/prepare_bind_groups | 0.222 | 0.313 | 0.58 | 339.1 |
| stage/render/prepare_meshes | 0.216 | 0.282 | 2.62 | 330.2 |
| terrain/select/ms | 0.208 | 0.274 | 0.43 | 317.3 |
| stream/drive/ms | 0.206 | 0.293 | 0.66 | 314.7 |
| stage/main/fixed_main_loop | 0.194 | 0.375 | 0.91 | 296.3 |
| stage/render/extract_commands | 0.182 | 0.217 | 0.52 | 278.0 |
| stage/render/prepare_batch_phases | 0.181 | 0.244 | 0.37 | 277.1 |
| stage/render/create_views | 0.180 | 0.234 | 0.65 | 274.6 |
| far/select/ms | 0.178 | 0.258 | 0.73 | 272.5 |
| stream/tick/ms | 0.174 | 0.251 | 0.58 | 266.2 |
| bigspace/high_precision/ms | 0.166 | 0.760 | 2.10 | 253.5 |
| controller/move/ms | 0.166 | 0.313 | 0.62 | 253.3 |
| controller/feed/ms | 0.165 | 0.211 | 0.63 | 252.7 |
| stage/render/prepare_flush | 0.156 | 0.263 | 0.64 | 238.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.135 | 0.206 | 0.40 | 207.0 |
| stage/main/last | 0.118 | 0.160 | 0.67 | 180.0 |
| stage/render/queue | 0.110 | 0.140 | 5.59 | 168.4 |
| bigspace/low_precision/ms | 0.101 | 0.194 | 1.35 | 154.0 |
| anim/evaluate/ms | 0.094 | 0.229 | 1.23 | 143.3 |
| stage/render/specialize | 0.092 | 0.112 | 8.80 | 141.0 |
| anim/evaluate_cpu/ms | 0.063 | 0.079 | 0.51 | 96.9 |
| stage/render/prepare_collect_phase_buffers | 0.062 | 0.118 | 0.18 | 95.3 |
| encode/resolve_diagnostics/sum | 0.062 | 0.090 | 0.27 | 94.2 |
| encode/camera_driver_span_begin/sum | 0.029 | 0.043 | 0.11 | 45.1 |
| physics/step/ms | 0.028 | 0.058 | 0.49 | 43.5 |
| weather/map/ms | 0.026 | 0.035 | 0.42 | 40.4 |
| encode/camera_driver_span_end/sum | 0.026 | 0.041 | 0.19 | 40.1 |
| stage/render/cleanup | 0.023 | 0.033 | 0.33 | 35.4 |
| stage/render/phase_sort | 0.017 | 0.031 | 0.06 | 25.7 |
| weather/eval/ms | 0.012 | 0.016 | 0.04 | 18.0 |
| script/update/ms | 0.008 | 0.010 | 0.03 | 11.8 |
| script/input/ms | 0.006 | 0.007 | 0.39 | 9.1 |
| controller/avoid/ms | 0.006 | 0.007 | 0.02 | 8.7 |
| anim/apply/ms | 0.005 | 0.006 | 0.03 | 6.9 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.6 |
| controller/write/ms | 0.002 | 0.002 | 0.05 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 1.0 |
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
| encode//max | ms | 0.65 | 1.87 | 0.91 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.11 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.19 | 0.02 |
| encode/finish_critical | ms | 0.94 | 2.39 | 1.17 |
| encode/pass:early prepass/max | ms | 0.77 | 1.47 | 1.17 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 1.42 | 0.82 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.13 | 0.40 | 0.22 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.27 | 0.65 | 0.39 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 2.39 | 1.14 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.27 | 0.08 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.21 | 25.55 | 9.21 |
| gpu/timed/ms | ms | 9.21 | 10.14 | 9.21 |
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
| process/cpu_usage | % | 33.33 | 42.78 | 28.85 |
| process/mem_usage | GiB | 4.77 | 4.85 | 4.84 |
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
| skin/blocks_changed/count | count | 5 | 6 | 4 |
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
| system/cpu_usage | % | 37.67 | 46.37 | 40.32 |
| system/mem_usage | % | 59.23 | 59.41 | 59.29 |
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
| anim/events/count | count | 0.09 | 3 | 144 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.24 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 52 | 40.6 k |
| encode/bind_walk/init_textures | count | 7165 | 16.3 k | 10.96 M |
| encode/bind_walk/init_textures_skipped | count | 35.6 k | 79.9 k | 54.44 M |
| encode/bind_walk/inits | count | 345 | 721 | 528.1 k |
| encode/bind_walk/inits_skipped | count | 67.00 | 142 | 102.4 k |
| encode/bind_walk/merge_textures | count | 7101 | 16.2 k | 10.86 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 8162 | 6.24 M |
| encode/bind_walk/merges | count | 82.26 | 173 | 125.8 k |
| encode/bind_walk/merges_skipped | count | 50.75 | 105 | 77.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1529 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1529 |
| encode/pass:early prepass/count | count | 1 | 2 | 1529 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1529 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 1529 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 1529 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 1529 |
| encode/queue/command_buffers | count | 36.57 | 72 | 55.9 k |
| encode/queue/submits | count | 2 | 4 | 3058 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.86 M | 2.20 G |
| encode/queue/write_buffer/calls | count | 217 | 468 | 332.3 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 65.3 k | 49.91 M |
| encode/queue/write_buffer_with/calls | count | 9 | 18 | 13.8 k |
| encode/queue/write_texture/bytes | bytes | 11.7 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1529 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.59 | 2 | 901 |
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
| warm-up | 2.69 | 2690.7 | main/busy/ms 2654, stage/main/post_update 2548, stage/main/update 97.04, surface/advance_cpu_ms 68.75, stage/main/pre_update 5.40, stage/main/last 1.14, surface/cell_upload_ms 0.76, stage/main/first 0.52, stream/drive/ms 0.43, stage/main/fixed_main_loop 0.36, weather/map/ms 0.12, stream/tick/ms 0.09 |
| warm-up | 3.47 | 774.3 | main/busy/ms 2812, stage/main/post_update 2580, render/thread/ms 744, render/busy/ms 560, stage/render/prepare_assets 511, stage/main/update 217, stage/render/prepare_views 179, render/acquire/ms 178, surface/advance_cpu_ms 68.75, stage/render/create_views 28.01, stage/render/render 15.20, render/graph/ms 8.37 |
| warm-up | 3.64 | 176.0 | render/thread/ms 901, render/busy/ms 628, stage/render/prepare_assets 562, main/busy/ms 189, stage/render/prepare_views 179, render/acquire/ms 178, stage/main/update 126, stage/render/render 110, render/present/ms 94.58, pipelines/compile_ms 49.32, stage/main/post_update 46.53, pipelines/front_ms 39.50 |
| warm-up | 3.68 | 40.7 | pipelines/compile_ms 381, pipelines/front_ms 238, render/thread/ms 179, pipelines/driver_ms 142, stage/render/render 110, main/busy/ms 104, render/present/ms 91.27, render/busy/ms 87.33, stage/main/update 54.86, stage/render/prepare_assets 51.78, pipelines/wait_ms 45.66, stage/main/post_update 33.63 |
| warm-up | 3.83 | 150.1 | pipelines/wait_ms 4644, pipelines/compile_ms 1999, pipelines/front_ms 1752, pipelines/driver_ms 248, render/thread/ms 134, render/busy/ms 130, main/busy/ms 101, stage/render/prepare_assets 97.55, stage/main/update 52.06, stage/main/post_update 40.52, pipelines/lock_ms 24.43, stage/render/render 23.97 |
| warm-up | 3.86 | 33.0 | pipelines/wait_ms 4733, pipelines/compile_ms 2129, pipelines/front_ms 1995, render/thread/ms 142, render/busy/ms 140, pipelines/driver_ms 134, stage/render/prepare_assets 107, main/busy/ms 65.10, stage/main/post_update 39.97, stage/render/render 19.86, stage/main/update 18.73, render/graph/ms 17.29 |
| warm-up | 3.91 | 48.0 | pipelines/compile_ms 1183, pipelines/front_ms 1138, pipelines/wait_ms 474, render/thread/ms 69.72, render/busy/ms 68.05, main/busy/ms 61.61, pipelines/driver_ms 44.67, stage/render/prepare_assets 40.09, stage/main/post_update 32.98, stage/main/update 22.01, stage/render/render 16.25, bigspace/total/ms 14.85 |
| warm-up | 3.99 | 33.0 | pipelines/compile_ms 393, pipelines/front_ms 376, render/thread/ms 56.00, render/busy/ms 53.97, main/busy/ms 33.89, stage/render/prepare_resources 28.27, stage/main/post_update 23.48, pipelines/driver_ms 17.08, stage/render/render 13.73, render/graph/ms 11.35, encode//sum 9.34, pipelines/wait_ms 9.04 |
| warm-up | 4.02 | 26.6 | pipelines/compile_ms 717, pipelines/front_ms 681, render/thread/ms 57.13, render/busy/ms 54.98, main/busy/ms 41.12, pipelines/driver_ms 35.76, stage/main/post_update 23.14, stage/render/prepare_resources 18.94, stage/render/render 14.74, render/graph/ms 12.33, stage/render/prepare_assets 11.54, stage/render/graph_submit 10.13 |
| warm-up | 4.06 | 34.7 | pipelines/compile_ms 1495, pipelines/front_ms 1430, pipelines/wait_ms 111, main/busy/ms 68.34, pipelines/driver_ms 65.88, render/thread/ms 57.69, render/busy/ms 55.76, stage/main/update 51.52, stream/drive/ms 43.80, stage/render/prepare_assets 29.08, stage/render/render 16.58, render/graph/ms 14.47 |
| warm-up | 4.12 | 65.9 | pipelines/compile_ms 1609, pipelines/front_ms 1553, pipelines/wait_ms 120, render/thread/ms 72.23, render/busy/ms 70.53, main/busy/ms 60.60, pipelines/driver_ms 55.95, stage/render/prepare_assets 50.75, stage/main/update 48.29, stream/drive/ms 44.18, stage/render/render 14.15, render/graph/ms 12.14 |
| warm-up | 4.20 | 68.8 | pipelines/compile_ms 386, pipelines/front_ms 371, render/thread/ms 52.80, render/busy/ms 49.74, main/busy/ms 38.13, stage/render/prepare_assets 28.77, stage/main/update 25.52, stream/drive/ms 20.85, stage/render/render 17.12, pipelines/driver_ms 14.60, render/graph/ms 13.63, stage/render/graph_submit 9.00 |
| warm-up | 4.24 | 37.4 | pipelines/compile_ms 508, pipelines/front_ms 489, render/thread/ms 73.85, main/busy/ms 70.45, render/busy/ms 70.41, stage/main/update 54.13, stream/drive/ms 48.98, stage/render/prepare_assets 46.37, pipelines/driver_ms 18.85, stage/render/render 18.74, render/graph/ms 14.90, stage/render/graph_submit 10.41 |
| warm-up | 4.36 | 124.1 | pipelines/compile_ms 786, pipelines/front_ms 760, main/busy/ms 95.61, render/thread/ms 88.74, render/busy/ms 86.94, stage/main/update 74.72, stage/render/prepare_assets 64.84, stream/drive/ms 48.60, pipelines/driver_ms 26.07, stage/render/render 13.43, render/graph/ms 11.16, stage/main/post_update 10.19 |
| warm-up | 4.43 | 67.2 | pipelines/compile_ms 1633, pipelines/front_ms 1576, main/busy/ms 106, render/thread/ms 98.73, render/busy/ms 97.40, stage/main/update 83.94, stage/render/prepare_assets 76.15, pipelines/driver_ms 56.71, stream/drive/ms 46.69, stage/render/render 11.97, stage/main/post_update 11.31, render/graph/ms 9.73 |
| warm-up | 4.50 | 67.3 | pipelines/compile_ms 1394, pipelines/front_ms 1350, main/busy/ms 82.86, stage/main/update 63.95, render/thread/ms 63.15, render/busy/ms 61.43, stream/drive/ms 45.90, pipelines/driver_ms 43.91, stage/render/prepare_assets 42.42, stage/render/render 10.29, stage/main/post_update 8.63, render/graph/ms 7.75 |
| warm-up | 4.56 | 63.9 | pipelines/compile_ms 1165, pipelines/front_ms 1131, pipelines/wait_ms 109, render/thread/ms 73.72, render/busy/ms 71.89, main/busy/ms 53.09, pipelines/driver_ms 34.90, stage/render/prepare_assets 30.30, stage/render/prepare_resources 18.84, stage/main/first 15.49, stage/main/update 12.12, stage/render/render 10.05 |
| warm-up | 4.59 | 35.1 | pipelines/compile_ms 1009, pipelines/front_ms 974, pipelines/wait_ms 217, render/thread/ms 80.98, render/busy/ms 79.70, main/busy/ms 59.32, pipelines/driver_ms 34.88, stage/render/prepare_resources 26.86, stage/render/prepare_assets 22.79, stage/main/first 15.41, stage/main/post_update 11.13, stage/main/update 10.78 |
| warm-up | 4.65 | 59.4 | pipelines/compile_ms 298, pipelines/front_ms 288, pipelines/wait_ms 129, render/thread/ms 87.27, render/busy/ms 86.05, main/busy/ms 68.10, stage/main/post_update 33.48, stage/render/prepare_resources 28.94, stage/main/update 22.85, stage/render/render 16.11, render/graph/ms 14.48, stage/render/prepare_bind_groups 12.11 |
| warm-up | 4.71 | 54.5 | pipelines/compile_ms 1023, pipelines/front_ms 990, render/thread/ms 99.57, render/busy/ms 98.25, main/busy/ms 84.39, pipelines/wait_ms 45.06, stage/main/post_update 42.58, stage/render/prepare_resources 35.51, pipelines/driver_ms 32.84, stage/main/update 28.79, stage/render/prepare_assets 19.95, stage/render/render 15.87 |
| warm-up | 4.77 | 59.6 | pipelines/compile_ms 2183, pipelines/front_ms 2117, pipelines/wait_ms 912, render/thread/ms 94.23, render/busy/ms 92.59, main/busy/ms 81.09, pipelines/driver_ms 65.95, stage/render/prepare_resources 40.14, stage/main/update 29.41, stage/main/post_update 29.25, stage/render/prepare_assets 28.25, stage/main/pre_update 16.44 |
| warm-up | 4.82 | 55.0 | pipelines/compile_ms 2109, pipelines/front_ms 2051, pipelines/wait_ms 1139, render/thread/ms 88.09, render/busy/ms 86.34, main/busy/ms 69.05, pipelines/driver_ms 58.28, stage/render/prepare_assets 33.68, stage/render/prepare_resources 25.88, stage/main/post_update 24.97, stage/main/update 24.48, stage/main/pre_update 12.95 |
| warm-up | 4.88 | 51.9 | pipelines/compile_ms 921, pipelines/front_ms 897, pipelines/wait_ms 252, main/busy/ms 81.22, render/thread/ms 70.94, render/busy/ms 69.26, stage/main/update 55.41, stream/drive/ms 47.99, stage/render/prepare_assets 43.87, pipelines/driver_ms 23.74, stage/main/post_update 16.23, stage/render/render 11.67 |
| warm-up | 4.92 | 47.1 | pipelines/compile_ms 1011, pipelines/front_ms 981, pipelines/wait_ms 688, main/busy/ms 114, stage/main/update 98.18, stream/drive/ms 87.26, render/thread/ms 52.54, render/busy/ms 50.85, stage/render/prepare_assets 29.83, pipelines/driver_ms 29.81, stage/render/render 12.10, render/graph/ms 9.67 |
| warm-up | 4.99 | 66.7 | pipelines/compile_ms 1283, pipelines/front_ms 1241, pipelines/wait_ms 690, main/busy/ms 71.43, stage/main/update 53.86, stream/drive/ms 43.16, pipelines/driver_ms 41.77, render/thread/ms 38.96, render/busy/ms 37.31, stage/render/prepare_assets 16.02, stage/render/render 12.35, render/graph/ms 10.15 |
| warm-up | 5.02 | 29.9 | pipelines/compile_ms 1093, pipelines/front_ms 1054, main/busy/ms 47.93, render/thread/ms 45.08, render/busy/ms 42.92, pipelines/driver_ms 39.76, stage/main/update 27.05, stage/render/render 19.15, render/graph/ms 16.70, stage/render/prepare_assets 12.40, stage/render/graph_submit 11.65, stage/main/post_update 9.85 |
| warm-up | 5.08 | 58.0 | pipelines/compile_ms 855, pipelines/front_ms 825, render/thread/ms 61.64, render/busy/ms 59.62, main/busy/ms 59.00, stage/main/update 41.12, pipelines/driver_ms 30.26, stage/render/prepare_assets 25.92, stage/render/render 19.32, render/graph/ms 16.79, stage/render/graph_submit 11.82, encode//sum 9.87 |
| warm-up | 5.11 | 37.0 | pipelines/compile_ms 843, pipelines/front_ms 815, render/thread/ms 65.82, render/busy/ms 64.43, main/busy/ms 49.37, stage/render/prepare_assets 36.66, stage/main/update 35.27, pipelines/driver_ms 27.98, stream/drive/ms 12.48, stage/render/render 10.96, render/graph/ms 8.95, encode//sum 7.79 |
| warm-up | 5.19 | 28.5 | pipelines/compile_ms 60.12, pipelines/front_ms 57.18, render/thread/ms 46.06, render/busy/ms 44.34, main/busy/ms 36.41, stage/main/update 21.14, stage/render/render 13.57, render/graph/ms 11.62, stage/main/post_update 9.36, stage/render/graph_submit 9.13, encode//sum 8.26, stage/render/prepare_resources 7.15 |
| warm-up | 5.22 | 31.6 | pipelines/compile_ms 297, pipelines/front_ms 288, render/thread/ms 50.18, render/busy/ms 48.48, main/busy/ms 42.10, stage/main/update 27.56, stream/drive/ms 16.16, stage/render/render 14.15, render/graph/ms 11.67, stage/render/prepare_assets 11.14, stage/render/graph_submit 9.02, encode//sum 8.93 |
| warm-up | 5.26 | 36.9 | pipelines/compile_ms 297, pipelines/front_ms 288, render/thread/ms 60.30, render/busy/ms 58.67, main/busy/ms 49.43, stage/main/update 24.28, stream/drive/ms 16.00, stage/render/render 13.31, stage/render/prepare_resources 13.00, render/graph/ms 11.02, stage/render/prepare_assets 10.50, stage/main/post_update 10.06 |
| warm-up | 5.29 | 30.8 | pipelines/compile_ms 214, pipelines/front_ms 208, render/thread/ms 62.07, render/busy/ms 60.80, main/busy/ms 46.05, stage/render/prepare_resources 20.46, stage/main/last 14.87, stage/main/update 12.84, stage/main/post_update 11.95, stage/render/render 11.22, render/graph/ms 9.59, stage/render/prepare_write_phase_buffers 7.06 |
| warm-up | 5.34 | 47.1 | pipelines/compile_ms 800, pipelines/front_ms 779, render/thread/ms 63.73, render/busy/ms 60.97, main/busy/ms 46.09, pipelines/driver_ms 21.32, stage/render/prepare_assets 20.45, stage/main/update 18.67, stage/main/post_update 14.54, stage/render/render 11.90, stage/render/prepare_resources 11.30, render/graph/ms 10.02 |
| warm-up | 5.38 | 40.4 | pipelines/compile_ms 1731, pipelines/front_ms 1686, render/thread/ms 67.89, render/busy/ms 64.57, pipelines/driver_ms 45.32, main/busy/ms 41.94, stage/render/prepare_assets 28.96, stage/main/update 19.77, stage/render/render 17.88, render/graph/ms 15.34, stage/main/post_update 14.57, stage/render/graph_submit 11.36 |
| warm-up | 5.42 | 43.9 | pipelines/compile_ms 1695, pipelines/front_ms 1650, render/thread/ms 71.10, render/busy/ms 69.44, main/busy/ms 54.08, pipelines/driver_ms 45.14, stage/main/update 32.90, stage/render/prepare_assets 18.40, stage/render/render 17.90, render/graph/ms 15.73, stage/render/prepare_collect_phase_buffers 12.01, stage/main/post_update 11.74 |
| warm-up | 5.47 | 51.0 | pipelines/compile_ms 551, pipelines/front_ms 536, render/thread/ms 71.50, render/busy/ms 70.22, main/busy/ms 53.37, stage/main/update 33.27, stage/render/prepare_assets 29.45, pipelines/driver_ms 15.51, stage/render/prepare_collect_phase_buffers 12.06, stage/render/render 11.93, stage/main/post_update 10.80, render/graph/ms 10.21 |
| warm-up | 5.58 | 34.3 | pipelines/compile_ms 1590, pipelines/front_ms 1549, pipelines/driver_ms 40.84, main/busy/ms 40.57, render/thread/ms 40.10, render/busy/ms 38.68, stage/main/update 18.65, stage/render/prepare_assets 14.88, stage/render/render 12.29, stream/drive/ms 11.64, render/graph/ms 10.38, stage/main/post_update 10.36 |
| warm-up | 5.69 | 34.5 | render/thread/ms 50.04, render/busy/ms 48.27, main/busy/ms 34.40, stage/render/prepare_assets 24.49, stage/main/update 17.69, stage/render/render 14.58, render/graph/ms 12.01, stage/main/post_update 10.00, stage/render/graph_submit 9.49, encode//sum 8.41, encode/submit 6.24, stream/drive/ms 4.38 |
| warm-up | 5.72 | 32.2 | pipelines/compile_ms 120, pipelines/front_ms 113, render/thread/ms 58.76, render/busy/ms 57.35, stage/render/prepare_assets 34.01, main/busy/ms 33.64, stage/main/update 16.51, stage/render/render 11.10, stage/main/post_update 10.54, render/graph/ms 9.29, encode//sum 6.92, pipelines/wait_ms 6.91 |
| warm-up | 5.78 | 37.4 | render/thread/ms 53.98, render/busy/ms 52.24, main/busy/ms 35.69, stage/render/prepare_assets 22.22, stage/render/render 15.96, stage/main/update 14.15, render/graph/ms 13.63, stage/main/post_update 13.07, stage/render/graph_submit 9.88, encode//sum 9.33, encode/submit 6.92, stream/drive/ms 5.60 |
| warm-up | 5.82 | 36.4 | render/thread/ms 57.84, render/busy/ms 56.53, main/busy/ms 36.30, stage/render/prepare_assets 34.28, stage/main/update 15.37, pipelines/compile_ms 14.04, pipelines/front_ms 13.04, stage/main/post_update 12.21, stage/render/render 8.65, encode//sum 7.23, render/graph/ms 6.80, stream/drive/ms 4.94 |
| warm-up | 5.86 | 34.5 | render/thread/ms 56.93, render/busy/ms 55.69, stage/render/prepare_assets 35.67, main/busy/ms 33.54, stage/main/update 15.46, pipelines/compile_ms 14.04, pipelines/front_ms 13.04, stage/main/post_update 11.63, stage/render/render 8.28, encode//sum 7.37, render/graph/ms 6.61, stream/drive/ms 5.28 |
| warm-up | 5.91 | 36.9 | pipelines/compile_ms 902, pipelines/front_ms 883, render/thread/ms 50.79, render/busy/ms 49.54, main/busy/ms 28.84, stage/render/prepare_assets 26.96, pipelines/driver_ms 18.95, stage/render/render 14.85, render/graph/ms 13.13, stage/main/post_update 12.42, stage/main/update 10.86, stage/render/graph_submit 10.78 |
| warm-up | 5.95 | 32.5 | pipelines/compile_ms 160, pipelines/front_ms 150, render/thread/ms 62.56, render/busy/ms 60.81, main/busy/ms 47.87, stage/main/update 27.46, stage/render/render 22.22, stage/render/prepare_assets 21.70, render/graph/ms 19.80, stage/render/graph_submit 16.09, stage/main/post_update 13.55, encode//sum 10.53 |
| warm-up | 5.99 | 48.7 | pipelines/compile_ms 152, pipelines/front_ms 143, render/thread/ms 64.96, render/busy/ms 63.42, main/busy/ms 52.96, stage/render/prepare_assets 29.90, stage/main/update 28.09, stage/render/render 16.25, stage/main/post_update 15.85, render/graph/ms 14.06, encode//sum 10.66, stage/render/graph_submit 10.26 |
| warm-up | 6.06 | 44.2 | render/thread/ms 48.07, main/busy/ms 47.26, render/busy/ms 46.53, stage/main/update 23.78, stage/render/render 16.04, stage/main/post_update 15.80, stage/render/prepare_assets 15.35, stream/drive/ms 14.10, render/graph/ms 13.82, stage/render/graph_submit 10.06, encode//sum 8.95, stage/render/prepare_bind_groups 6.19 |
| warm-up | 6.10 | 36.9 | pipelines/compile_ms 206, pipelines/front_ms 199, render/thread/ms 56.46, render/busy/ms 55.02, main/busy/ms 52.87, stage/main/update 28.98, stage/render/prepare_assets 22.40, stage/main/post_update 15.84, stage/render/render 15.51, stream/drive/ms 14.49, render/graph/ms 13.39, stage/render/graph_submit 9.25 |
| warm-up | 6.12 | 26.8 | pipelines/compile_ms 1201, pipelines/front_ms 1173, render/thread/ms 48.70, main/busy/ms 47.65, render/busy/ms 47.13, pipelines/driver_ms 27.44, stage/main/update 20.73, stage/main/post_update 19.22, stage/render/render 18.51, stage/render/prepare_assets 16.68, render/graph/ms 16.10, stage/render/graph_submit 11.16 |
| warm-up | 6.17 | 28.5 | pipelines/compile_ms 60.58, pipelines/front_ms 57.62, render/thread/ms 40.08, render/busy/ms 38.78, main/busy/ms 30.31, stage/render/prepare_assets 14.80, stage/main/update 13.64, stage/render/render 13.26, render/graph/ms 11.49, stage/main/post_update 11.15, stage/render/graph_submit 8.05, encode//sum 7.96 |
| warm-up | 6.20 | 25.4 | render/thread/ms 46.24, pipelines/compile_ms 44.95, render/busy/ms 44.83, pipelines/front_ms 43.04, main/busy/ms 38.98, stage/main/update 17.84, stage/render/prepare_assets 15.01, stage/render/render 14.51, render/graph/ms 12.61, stage/main/post_update 11.02, stage/render/graph_submit 8.89, encode//sum 8.51 |
| warm-up | 6.32 | 35.0 | pipelines/compile_ms 1135, pipelines/front_ms 1105, main/busy/ms 47.16, render/thread/ms 30.73, stage/main/update 30.39, pipelines/driver_ms 30.06, render/busy/ms 29.14, stream/drive/ms 21.22, stage/render/render 12.06, stage/main/post_update 11.17, render/graph/ms 10.08, encode//sum 8.37 |
| warm-up | 6.40 | 32.9 | pipelines/compile_ms 364, pipelines/front_ms 355, main/busy/ms 37.18, render/thread/ms 28.61, render/busy/ms 27.40, stage/main/update 19.30, stage/main/post_update 13.01, stage/render/render 11.75, stage/render/prepare_assets 10.32, render/graph/ms 10.04, pipelines/driver_ms 9.56, encode//sum 8.34 |
| warm-up | 6.64 | 40.6 | main/busy/ms 42.36, render/thread/ms 35.32, render/busy/ms 33.39, stage/main/update 21.04, stage/render/render 16.63, stage/main/post_update 16.24, render/graph/ms 13.58, stream/drive/ms 11.03, encode//sum 10.29, stage/render/graph_submit 9.75, stage/render/prepare_assets 9.19, encode/submit 6.52 |
| warm-up | 6.68 | 37.0 | render/thread/ms 46.61, render/busy/ms 45.15, main/busy/ms 34.78, stage/render/prepare_assets 25.49, stage/main/post_update 16.33, stage/main/update 12.59, stage/render/render 11.70, render/graph/ms 9.53, encode//sum 9.27, stage/render/graph_submit 6.20, stream/drive/ms 5.27, encode/submit 3.26 |
| warm-up | 7.02 | 31.4 | pipelines/compile_ms 134, pipelines/front_ms 130, render/thread/ms 38.38, render/busy/ms 37.35, main/busy/ms 16.00, stage/render/render 10.53, stage/render/specialize 9.63, render/graph/ms 9.20, stage/main/post_update 8.43, stage/render/graph_submit 7.29, encode//sum 7.23, stage/render/queue 6.77 |
| warm-up | 8.27 | 32.7 | render/thread/ms 37.02, render/busy/ms 35.81, main/busy/ms 15.84, stage/render/render 10.54, render/graph/ms 9.02, stage/main/post_update 8.58, stage/render/specialize 8.47, encode//sum 7.15, stage/render/graph_submit 7.09, stage/render/queue 6.16, stage/render/prepare_meshes 4.39, stage/main/update 4.24 |
| measure | 0.84 | 27.3 | render/thread/ms 34.45, render/busy/ms 32.09, main/busy/ms 14.41, stage/render/render 10.30, encode//sum 9.09, render/graph/ms 8.98, stage/render/specialize 8.86, stage/main/post_update 8.86, stage/render/graph_submit 7.14, stage/render/queue 5.69, encode/submit 4.36, stage/main/update 3.40 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=base-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 5%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 80%) · 2625 MHz (min 2625) · memory 8101 MHz · 97.1 W · 77 °C (max 80) · VRAM 6426 MiB · throttled 100% of samples

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
