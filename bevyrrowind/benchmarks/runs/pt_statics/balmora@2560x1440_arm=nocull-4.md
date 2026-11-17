# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1512 frames over 15.0 s · **median 9.92 ms (101 fps)** · mean 9.92 · p95 11.28 · p99 11.70 (1% low 85 fps) · worst 26.87 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 279 frames over 8.5 s, worst 2459.3 ms
- hitches (> 25.0 ms): **1** measured, 57 in warm-up
- timed GPU passes: 8.62 ms of a 9.92 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.92 ms, limit 11 ms
- pass worst frame 26.87 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.926 | 2.932 | 3.000 | 1.00 | 0.068 |
| dlss_super_resolution | 2.020 | 2.024 | 2.049 | 1.00 | 0.185 |
| early prepass | 0.603 | 0.604 | 0.614 | 1.00 | 0.089 |
| atmosphere_luts | 0.326 | 0.327 | 0.342 | 1.00 | 0.003 |
| bloom | 0.314 | 0.315 | 0.331 | 1.00 | 0.062 |
| main_transmissive_pass_3d | 0.311 | 0.312 | 0.333 | 1.00 | 0.008 |
| water_velocity | 0.240 | 0.241 | 0.265 | 1.00 | 0.002 |
| water_sspr | 0.207 | 0.208 | 0.210 | 1.00 | 0.002 |
| ssao | 0.190 | 0.191 | 0.198 | 1.00 | 0.007 |
| clouds_trace | 0.149 | 0.149 | 0.169 | 1.00 | 0.001 |
| uniform_allocation | 0.148 | 0.149 | 0.197 | 1.00 | 0.023 |
| atmos_froxel_inject | 0.147 | 0.147 | 0.153 | 1.00 | 0.001 |
| water_fft | 0.131 | 0.131 | 0.135 | 1.00 | 0.003 |
| upscaling | 0.114 | 0.114 | 0.119 | 1.00 | 0.004 |
| clouds_composite | 0.112 | 0.112 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.103 | 0.103 | 0.134 | 1.00 | 0.017 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.413 | 0.414 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.033 | 0.033 | 0.050 | 1.00 | 0.005 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| main_transparent_pass_3d | 0.028 | 0.028 | 0.031 | 1.00 | 0.134 |
| lightprobe_radiance_map | 0.028 | 0.221 | 0.230 | 0.12 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.043 | 0.044 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 1.00 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.003 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.887 | 10.248 | 25.72 | 13437.8 |
| render/busy/ms | 7.134 | 8.146 | 25.11 | 10787.3 |
| main/busy/ms | 7.006 | 7.890 | 9.09 | 10593.5 |
| stage/render/render | 4.735 | 5.536 | 11.00 | 7159.7 |
| render/graph/ms | 4.134 | 4.905 | 7.66 | 6250.7 |
| encode//sum | 4.085 | 5.330 | 10.15 | 6176.2 |
| stage/main/post_update | 4.079 | 4.870 | 6.05 | 6167.4 |
| stage/render/graph_submit | 3.085 | 3.812 | 5.97 | 4664.8 |
| encode/submit | 1.919 | 2.363 | 7.30 | 2901.3 |
| stage/main/update | 1.810 | 2.253 | 3.55 | 2736.6 |
| stage/render/prepare_views | 1.445 | 3.040 | 4.10 | 2185.4 |
| render/acquire/ms | 1.400 | 2.989 | 4.04 | 2117.4 |
| pipelines/compile_ms | 1.241 | 0.000 | 136.86 | 1875.8 |
| pipelines/front_ms | 1.207 | 0.000 | 131.66 | 1824.4 |
| encode/finish_wall | 1.190 | 1.673 | 2.69 | 1798.7 |
| stage/render/camera_driver | 0.993 | 1.131 | 1.64 | 1500.7 |
| encode/pass:shadow_directional_light/sum | 0.771 | 1.348 | 1.92 | 1165.4 |
| encode/pass:early prepass/sum | 0.737 | 0.997 | 1.41 | 1114.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.701 | 0.973 | 1.23 | 1060.4 |
| stage/render/prepare_write_phase_buffers | 0.583 | 0.815 | 0.99 | 881.6 |
| stage/render/prepare_assets | 0.451 | 0.577 | 2.16 | 681.6 |
| stage/render/prepare_resources | 0.385 | 0.446 | 0.72 | 581.5 |
| bigspace/total/ms | 0.361 | 0.753 | 2.25 | 545.3 |
| render/present/ms | 0.353 | 0.419 | 3.12 | 533.1 |
| stage/main/pre_update | 0.344 | 0.449 | 0.88 | 520.1 |
| encode/pass:main_transparent_pass_3d/sum | 0.314 | 0.453 | 0.67 | 474.4 |
| stage/main/first | 0.288 | 0.350 | 1.35 | 435.9 |
| terrain/update/ms | 0.231 | 0.295 | 0.63 | 349.1 |
| stage/render/prepare_batch_phases | 0.229 | 0.300 | 0.47 | 346.6 |
| stage/render/prepare_bind_groups | 0.212 | 0.298 | 0.51 | 320.4 |
| stage/render/prepare_meshes | 0.206 | 0.253 | 1.22 | 311.4 |
| terrain/select/ms | 0.201 | 0.258 | 0.60 | 304.1 |
| stream/drive/ms | 0.186 | 0.245 | 0.69 | 281.8 |
| stage/main/fixed_main_loop | 0.185 | 0.344 | 0.92 | 280.0 |
| far/select/ms | 0.169 | 0.233 | 0.68 | 255.7 |
| stage/render/create_views | 0.169 | 0.218 | 0.68 | 255.7 |
| stage/render/extract_commands | 0.169 | 0.197 | 0.33 | 255.1 |
| stream/tick/ms | 0.159 | 0.214 | 0.66 | 240.2 |
| stage/render/prepare_flush | 0.154 | 0.276 | 0.54 | 232.1 |
| controller/feed/ms | 0.147 | 0.193 | 0.60 | 222.1 |
| controller/move/ms | 0.135 | 0.282 | 0.82 | 204.4 |
| bigspace/high_precision/ms | 0.130 | 0.340 | 1.31 | 196.5 |
| stage/main/last | 0.119 | 0.177 | 0.75 | 180.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.118 | 0.180 | 0.43 | 178.1 |
| stage/render/queue | 0.109 | 0.141 | 5.91 | 165.4 |
| stage/render/cleanup | 0.100 | 0.154 | 0.33 | 151.2 |
| stage/render/specialize | 0.093 | 0.125 | 8.41 | 141.0 |
| anim/evaluate/ms | 0.087 | 0.224 | 0.99 | 130.9 |
| bigspace/low_precision/ms | 0.083 | 0.178 | 1.46 | 125.6 |
| anim/evaluate_cpu/ms | 0.060 | 0.075 | 0.50 | 90.1 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.116 | 0.19 | 87.1 |
| encode/resolve_diagnostics/sum | 0.056 | 0.085 | 0.23 | 84.4 |
| pipelines/driver_ms | 0.034 | 0.000 | 5.45 | 51.4 |
| physics/step/ms | 0.030 | 0.058 | 0.50 | 45.3 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.038 | 0.12 | 39.6 |
| weather/map/ms | 0.024 | 0.033 | 0.49 | 37.0 |
| stage/render/phase_sort | 0.023 | 0.039 | 0.19 | 35.3 |
| encode/camera_driver_span_end/sum | 0.023 | 0.036 | 0.10 | 34.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.04 | 16.0 |
| script/update/ms | 0.007 | 0.009 | 0.06 | 10.3 |
| controller/avoid/ms | 0.006 | 0.007 | 0.37 | 8.9 |
| script/input/ms | 0.006 | 0.007 | 0.03 | 8.3 |
| anim/apply/ms | 0.004 | 0.006 | 0.04 | 6.7 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 5.6 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.09 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.03 | 0.4 |
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
| encode//max | ms | 0.55 | 2.46 | 0.47 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.12 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.10 | 0.02 |
| encode/finish_critical | ms | 0.87 | 2.46 | 0.77 |
| encode/pass:early prepass/max | ms | 0.74 | 1.41 | 0.77 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.71 | 1.23 | 0.73 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.43 | 0.14 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.30 | 0.67 | 0.24 |
| encode/pass:shadow_directional_light/max | ms | 0.72 | 1.92 | 0.55 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.23 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.27 | 25.11 | 9.31 |
| gpu/timed/ms | ms | 9.27 | 10.47 | 9.31 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1943 | 1943 | 1943 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 207.2 k | 207.2 k | 207.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 13.3 k | 13.3 k | 13.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 485.99 M | 485.99 M | 485.99 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d/bins | count | 330 | 330 | 330 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3189 | 3189 | 3189 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 330 | 330 | 330 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3189 | 3189 | 3189 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 21 | 21 | 21 |
| phases/opaque3d/bins | count | 3895 | 3895 | 3895 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 9283 | 9283 | 9283 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/bins | count | 3895 | 3895 | 3895 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 9283 | 9283 | 9283 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 4 | 34 | 4 |
| phases/shadow/bins | count | 10.2 k | 12.9 k | 10.2 k |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 31.2 k | 42.9 k | 31.2 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 27 | 34 | 27 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 321 | 321 | 321 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 8 | 0 |
| preprocess/bind_groups | count | 0 | 64 | 0 |
| preprocess/bind_groups_created | count | 0 | 64 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 137 | 137 | 137 |
| process/cpu_usage | % | 26.09 | 43.54 | 25 |
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
| system/cpu_usage | % | 31.73 | 38.81 | 30.62 |
| system/mem_usage | % | 54.41 | 54.58 | 54.57 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 58 | 88 | 58 |
| views/all/bins | count | 18.6 k | 21.4 k | 18.6 k |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 56.2 k | 67.8 k | 56.2 k |
| views/all/pipelines | count | 77 | 84 | 77 |
| views/all/pipelines_distinct | count | 65 | 65 | 65 |
| views/all/work_items | count | 34.7 k | 68.3 k | 34.7 k |
| views/cascade/batch_sets | count | 4 | 34 | 4 |
| views/cascade/bins | count | 10.2 k | 12.9 k | 10.2 k |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 31.2 k | 42.9 k | 31.2 k |
| views/cascade/pipelines | count | 27 | 34 | 27 |
| views/cascade/work_items | count | 9346 | 42.9 k | 9346 |
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
| views/main/batch_sets | count | 54 | 54 | 54 |
| views/main/bins | count | 8450 | 8450 | 8450 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 24.9 k | 24.9 k | 24.9 k |
| views/main/pipelines | count | 50 | 50 | 50 |
| views/main/work_items | count | 25.4 k | 25.4 k | 25.4 k |
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
| anim/events/count | count | 0.11 | 2 | 162 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.28 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 29 | 40.2 k |
| encode/bind_walk/init_textures | count | 7180 | 14.0 k | 10.86 M |
| encode/bind_walk/init_textures_skipped | count | 40.1 k | 54.7 k | 60.65 M |
| encode/bind_walk/inits | count | 422 | 515 | 637.9 k |
| encode/bind_walk/inits_skipped | count | 102 | 115 | 153.5 k |
| encode/bind_walk/merge_textures | count | 7116 | 13.9 k | 10.76 M |
| encode/bind_walk/merge_textures_skipped | count | 4225 | 4225 | 6.39 M |
| encode/bind_walk/merges | count | 84.26 | 105 | 127.4 k |
| encode/bind_walk/merges_skipped | count | 83 | 89 | 125.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1512 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1512 |
| encode/pass:early prepass/count | count | 1 | 1 | 1512 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1512 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1512 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1512 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1512 |
| encode/queue/command_buffers | count | 36.57 | 39 | 55.3 k |
| encode/queue/submits | count | 2 | 2 | 3024 |
| encode/queue/write_buffer/bytes | bytes | 1.84 M | 4.56 M | 2.79 G |
| encode/queue/write_buffer/calls | count | 233 | 305 | 352.4 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 49.35 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.6 k |
| encode/queue/write_texture/bytes | bytes | 11.9 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1512 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.60 | 2 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 27 |
| pipelines/created | count | 0.02 | 1 | 26 |
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

## Hitches (> 25.0 ms; slowest 58, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.46 | 2459.3 | main/busy/ms 2432, stage/main/post_update 2325, stage/main/update 96.61, surface/advance_cpu_ms 66.23, stage/main/pre_update 5.71, stage/main/last 1.11, stage/main/first 0.79, surface/cell_upload_ms 0.74, stream/drive/ms 0.64, stage/main/fixed_main_loop 0.38, stream/tick/ms 0.17, weather/map/ms 0.17 |
| warm-up | 3.17 | 713.8 | main/busy/ms 2575, stage/main/post_update 2357, render/thread/ms 681, render/busy/ms 519, stage/render/prepare_assets 469, stage/main/update 203, stage/render/prepare_views 156, render/acquire/ms 156, surface/advance_cpu_ms 66.23, stage/render/create_views 31.10, stage/render/render 13.94, render/graph/ms 7.32 |
| warm-up | 3.35 | 174.9 | render/thread/ms 838, render/busy/ms 585, stage/render/prepare_assets 520, main/busy/ms 174, stage/render/prepare_views 157, render/acquire/ms 156, stage/main/update 113, stage/render/render 111, render/present/ms 96.73, stage/main/post_update 47.34, pipelines/compile_ms 44.36, pipelines/front_ms 39.80 |
| warm-up | 3.41 | 60.1 | pipelines/compile_ms 790, pipelines/front_ms 608, pipelines/wait_ms 381, render/thread/ms 216, pipelines/driver_ms 182, stage/render/render 149, render/busy/ms 118, main/busy/ms 108, render/present/ms 97.17, stage/main/update 62.92, stage/render/prepare_assets 51.40, render/graph/ms 50.56 |
| warm-up | 3.56 | 152.5 | pipelines/wait_ms 4794, pipelines/compile_ms 2336, pipelines/front_ms 2020, pipelines/driver_ms 315, render/thread/ms 185, render/busy/ms 178, main/busy/ms 121, stage/render/prepare_assets 109, stage/render/render 64.45, stage/main/update 59.70, render/graph/ms 56.29, stage/render/graph_submit 50.26 |
| warm-up | 3.65 | 85.5 | pipelines/wait_ms 4570, pipelines/compile_ms 2510, pipelines/front_ms 2159, pipelines/driver_ms 351, render/thread/ms 210, render/busy/ms 208, stage/render/prepare_assets 183, main/busy/ms 64.97, stage/main/post_update 43.58, stage/render/render 16.65, render/graph/ms 14.34, terrain/update/ms 13.93 |
| warm-up | 3.68 | 30.7 | pipelines/compile_ms 1009, pipelines/front_ms 791, pipelines/driver_ms 219, pipelines/wait_ms 167, render/thread/ms 111, render/busy/ms 110, stage/render/prepare_assets 94.59, main/busy/ms 46.27, stage/main/post_update 14.21, stage/main/first 14.15, terrain/update/ms 13.60, stage/main/update 11.06 |
| warm-up | 3.73 | 26.0 | pipelines/compile_ms 853, pipelines/front_ms 814, pipelines/wait_ms 65.95, render/thread/ms 45.37, render/busy/ms 43.10, pipelines/driver_ms 38.78, main/busy/ms 38.52, stage/main/post_update 24.42, stage/render/render 17.02, encode//sum 14.92, render/graph/ms 14.57, stage/render/prepare_bind_groups 11.54 |
| warm-up | 3.87 | 79.3 | pipelines/compile_ms 1154, pipelines/front_ms 1118, render/thread/ms 62.13, render/busy/ms 60.55, main/busy/ms 56.47, stage/render/prepare_assets 45.87, stage/main/update 43.29, stream/drive/ms 38.57, pipelines/driver_ms 36.07, pipelines/wait_ms 12.13, stage/render/render 10.71, render/graph/ms 8.94 |
| warm-up | 3.93 | 53.7 | render/thread/ms 41.11, render/busy/ms 39.90, main/busy/ms 34.43, stage/render/prepare_assets 25.10, stage/main/update 22.28, stream/drive/ms 18.37, stage/render/render 9.99, render/graph/ms 8.29, stage/main/post_update 7.09, encode//sum 7.01, stage/render/graph_submit 5.29, encode/submit 3.61 |
| warm-up | 3.96 | 29.2 | pipelines/compile_ms 176, pipelines/front_ms 168, main/busy/ms 64.99, render/thread/ms 53.01, stage/main/update 51.91, render/busy/ms 51.30, stream/drive/ms 47.73, stage/render/prepare_assets 36.01, stage/render/render 10.43, pipelines/driver_ms 8.46, render/graph/ms 8.04, stage/main/post_update 6.93 |
| warm-up | 4.05 | 96.3 | pipelines/compile_ms 176, pipelines/front_ms 168, main/busy/ms 70.90, render/thread/ms 62.90, render/busy/ms 61.31, stage/main/update 56.94, stage/render/prepare_assets 47.84, stream/drive/ms 46.50, stage/render/render 8.60, pipelines/driver_ms 8.46, stage/main/post_update 7.55, render/graph/ms 6.42 |
| warm-up | 4.09 | 30.7 | pipelines/compile_ms 169, pipelines/front_ms 161, main/busy/ms 79.64, render/thread/ms 68.20, render/busy/ms 66.86, stage/main/update 62.19, stage/render/prepare_assets 48.42, stream/drive/ms 26.54, stage/render/render 10.02, render/graph/ms 8.12, stage/main/post_update 7.92, pipelines/driver_ms 7.50 |
| warm-up | 4.20 | 118.1 | pipelines/compile_ms 306, pipelines/front_ms 293, render/thread/ms 93.07, render/busy/ms 91.14, main/busy/ms 77.48, stage/main/update 56.77, stage/render/render 39.79, render/graph/ms 37.14, stage/render/prepare_assets 35.22, stage/render/graph_submit 33.11, stream/drive/ms 25.99, pipelines/wait_ms 20.27 |
| warm-up | 4.24 | 38.6 | pipelines/compile_ms 435, pipelines/front_ms 419, pipelines/wait_ms 137, render/thread/ms 104, render/busy/ms 102, stage/render/prepare_assets 40.54, stage/render/render 38.51, render/graph/ms 36.01, main/busy/ms 34.81, stage/render/graph_submit 32.62, stage/main/post_update 16.68, pipelines/driver_ms 16.14 |
| warm-up | 4.36 | 120.8 | pipelines/compile_ms 1392, pipelines/front_ms 1346, pipelines/wait_ms 200, render/thread/ms 125, render/busy/ms 124, main/busy/ms 73.40, stage/render/prepare_assets 57.02, pipelines/driver_ms 45.58, stage/main/update 43.16, stage/render/prepare_write_phase_buffers 33.74, stage/main/post_update 14.15, stage/render/render 9.97 |
| warm-up | 4.44 | 79.5 | pipelines/compile_ms 2531, pipelines/front_ms 2433, pipelines/wait_ms 222, render/thread/ms 146, render/busy/ms 145, pipelines/driver_ms 98.62, stage/render/prepare_assets 78.82, main/busy/ms 78.03, stage/main/update 49.73, stage/render/prepare_write_phase_buffers 32.96, stage/render/render 10.50, stage/main/post_update 10.49 |
| warm-up | 4.47 | 33.0 | pipelines/compile_ms 1932, pipelines/front_ms 1852, pipelines/wait_ms 167, render/thread/ms 82.46, render/busy/ms 80.78, pipelines/driver_ms 80.15, stage/render/prepare_assets 51.08, main/busy/ms 46.43, stage/main/update 21.82, stage/main/post_update 14.95, stage/render/render 11.43, render/graph/ms 9.52 |
| warm-up | 4.52 | 42.4 | pipelines/compile_ms 523, pipelines/front_ms 505, render/thread/ms 65.19, render/busy/ms 63.55, main/busy/ms 50.18, stage/main/update 23.55, pipelines/wait_ms 21.64, stage/main/post_update 18.67, pipelines/driver_ms 17.87, stage/render/prepare_assets 16.57, stage/render/render 15.56, render/graph/ms 13.70 |
| warm-up | 4.55 | 35.7 | render/thread/ms 65.18, render/busy/ms 63.60, pipelines/compile_ms 44.03, pipelines/front_ms 41.42, main/busy/ms 41.21, stage/main/update 21.82, stage/render/prepare_assets 20.65, stage/render/render 14.05, stage/main/post_update 12.98, render/graph/ms 12.11, stage/render/graph_submit 9.66, stage/render/prepare_resources 7.40 |
| warm-up | 4.59 | 33.3 | pipelines/compile_ms 427, pipelines/front_ms 415, main/busy/ms 61.31, render/thread/ms 49.42, stage/main/update 48.79, render/busy/ms 47.88, stream/drive/ms 39.73, stage/render/prepare_assets 25.88, stage/render/render 12.52, pipelines/driver_ms 12.25, render/graph/ms 10.30, encode//sum 9.24 |
| warm-up | 4.63 | 48.2 | pipelines/compile_ms 1335, pipelines/front_ms 1299, main/busy/ms 94.95, stage/main/update 81.33, stream/drive/ms 72.47, render/thread/ms 42.59, render/busy/ms 40.98, pipelines/driver_ms 35.72, stage/render/prepare_assets 18.08, stage/render/render 13.13, render/graph/ms 10.97, encode//sum 9.54 |
| warm-up | 4.69 | 57.2 | pipelines/compile_ms 1138, pipelines/front_ms 1106, main/busy/ms 71.91, stage/main/update 45.55, render/thread/ms 41.33, render/busy/ms 39.76, stream/drive/ms 37.00, pipelines/driver_ms 31.70, stage/main/post_update 19.24, stage/render/render 14.58, stage/render/prepare_assets 14.20, render/graph/ms 12.61 |
| warm-up | 4.75 | 62.4 | pipelines/compile_ms 270, pipelines/front_ms 261, main/busy/ms 108, render/thread/ms 75.06, render/busy/ms 73.11, stage/main/update 56.38, stage/main/post_update 31.86, stage/render/prepare_flush 27.54, stage/render/render 16.06, stage/render/prepare_assets 15.37, render/graph/ms 14.10, pipelines/wait_ms 9.58 |
| warm-up | 4.85 | 97.4 | pipelines/compile_ms 1257, pipelines/front_ms 1213, main/busy/ms 123, render/thread/ms 104, render/busy/ms 102, stage/main/update 67.43, pipelines/driver_ms 44.49, stage/render/prepare_assets 39.37, stage/render/prepare_flush 27.83, stage/main/post_update 25.44, stage/main/pre_update 16.34, stage/render/render 12.96 |
| warm-up | 4.89 | 36.2 | pipelines/compile_ms 1201, pipelines/front_ms 1159, render/thread/ms 80.64, render/busy/ms 79.05, main/busy/ms 55.00, pipelines/driver_ms 42.13, stage/render/prepare_assets 39.03, stage/main/update 22.18, stage/render/render 13.86, stage/main/post_update 13.63, stream/drive/ms 12.53, stage/main/pre_update 12.23 |
| warm-up | 4.92 | 29.0 | pipelines/compile_ms 1714, pipelines/front_ms 1664, pipelines/wait_ms 86.40, render/thread/ms 53.99, render/busy/ms 52.44, pipelines/driver_ms 49.74, main/busy/ms 37.30, stage/main/update 16.29, stage/render/render 13.86, stage/render/prepare_assets 12.98, stage/main/post_update 12.20, render/graph/ms 11.95 |
| warm-up | 4.97 | 25.5 | pipelines/compile_ms 51.71, pipelines/front_ms 48.86, main/busy/ms 45.26, render/thread/ms 41.02, render/busy/ms 39.04, stage/main/update 28.67, stream/drive/ms 16.02, stage/render/render 13.89, render/graph/ms 11.47, stage/main/post_update 10.42, stage/render/graph_submit 8.72, pipelines/wait_ms 8.41 |
| warm-up | 4.99 | 29.2 | pipelines/compile_ms 83.76, pipelines/front_ms 77.21, render/thread/ms 42.43, render/busy/ms 40.34, main/busy/ms 40.25, stage/main/update 27.45, stream/drive/ms 16.78, stage/render/render 14.10, render/graph/ms 11.67, stage/render/prepare_assets 9.59, stage/render/graph_submit 8.79, encode//sum 7.43 |
| warm-up | 5.02 | 26.9 | pipelines/compile_ms 396, pipelines/front_ms 379, pipelines/wait_ms 64.47, render/thread/ms 44.53, render/busy/ms 42.78, main/busy/ms 33.04, stage/main/update 18.32, pipelines/driver_ms 17.43, stage/render/render 12.63, render/graph/ms 10.70, stage/render/prepare_assets 10.04, stage/main/post_update 8.99 |
| warm-up | 5.06 | 42.5 | pipelines/compile_ms 937, pipelines/front_ms 905, pipelines/wait_ms 226, render/thread/ms 59.56, render/busy/ms 57.73, main/busy/ms 38.54, pipelines/driver_ms 31.41, stage/render/prepare_assets 23.82, stage/main/update 21.65, stage/render/render 14.57, render/graph/ms 12.21, stage/main/post_update 10.57 |
| warm-up | 5.10 | 36.5 | pipelines/compile_ms 615, pipelines/front_ms 595, pipelines/wait_ms 162, render/thread/ms 67.70, render/busy/ms 65.82, main/busy/ms 43.05, stage/render/prepare_assets 26.61, pipelines/driver_ms 19.77, stage/main/update 19.39, stage/main/post_update 16.00, stage/render/render 15.96, render/graph/ms 13.39 |
| warm-up | 5.14 | 43.2 | pipelines/compile_ms 647, pipelines/front_ms 631, render/thread/ms 62.82, render/busy/ms 61.28, main/busy/ms 36.58, stage/render/prepare_assets 29.14, pipelines/driver_ms 16.04, stage/main/post_update 15.90, pipelines/wait_ms 15.18, stage/main/update 13.30, stage/render/render 12.47, render/graph/ms 10.34 |
| warm-up | 5.19 | 41.3 | pipelines/compile_ms 1057, pipelines/front_ms 1027, render/thread/ms 65.59, render/busy/ms 64.07, stage/render/prepare_assets 40.07, main/busy/ms 32.25, pipelines/driver_ms 29.97, pipelines/wait_ms 15.37, stage/main/update 13.78, stage/main/post_update 11.87, stage/render/render 9.44, render/graph/ms 7.50 |
| warm-up | 5.25 | 39.9 | pipelines/compile_ms 785, pipelines/front_ms 762, render/thread/ms 51.43, render/busy/ms 49.73, main/busy/ms 29.22, pipelines/driver_ms 22.79, stage/render/render 18.42, stage/render/prepare_assets 18.01, render/graph/ms 16.18, stage/render/graph_submit 12.76, stage/main/post_update 12.12, stage/main/update 11.05 |
| warm-up | 5.28 | 30.2 | pipelines/compile_ms 667, pipelines/front_ms 643, render/thread/ms 60.79, render/busy/ms 58.80, main/busy/ms 42.12, stage/render/render 25.69, pipelines/driver_ms 23.42, render/graph/ms 22.54, stage/main/update 21.20, stage/render/prepare_assets 15.66, stage/render/graph_submit 15.42, stage/main/post_update 14.14 |
| warm-up | 5.33 | 48.0 | pipelines/compile_ms 86.69, pipelines/front_ms 81.91, render/thread/ms 73.35, render/busy/ms 71.38, main/busy/ms 66.27, stage/main/update 29.51, stage/main/post_update 27.94, stage/render/render 19.20, render/graph/ms 16.30, stage/render/prepare_assets 14.51, stream/drive/ms 12.70, stage/render/prepare_resources 12.65 |
| warm-up | 5.35 | 28.9 | pipelines/compile_ms 198, pipelines/front_ms 193, render/thread/ms 70.45, render/busy/ms 68.94, main/busy/ms 52.50, stage/main/post_update 23.87, stage/render/prepare_assets 22.32, stage/main/update 19.87, stage/render/prepare_resources 10.99, stage/render/render 10.33, render/graph/ms 8.38, stage/render/prepare_batch_phases 7.03 |
| warm-up | 5.39 | 40.0 | pipelines/compile_ms 774, pipelines/front_ms 754, render/thread/ms 63.72, render/busy/ms 62.30, main/busy/ms 43.27, stage/render/prepare_assets 24.12, stage/main/post_update 21.19, pipelines/driver_ms 19.31, stage/main/update 11.42, stage/render/render 11.34, render/graph/ms 9.28, encode//sum 7.74 |
| warm-up | 5.46 | 40.6 | pipelines/compile_ms 1472, pipelines/front_ms 1438, render/thread/ms 53.67, render/busy/ms 52.22, main/busy/ms 38.77, pipelines/driver_ms 33.87, stage/render/prepare_assets 26.48, stage/main/update 20.90, stage/render/render 12.69, render/graph/ms 10.81, stage/main/post_update 10.74, encode//sum 8.55 |
| warm-up | 5.51 | 33.9 | pipelines/compile_ms 207, pipelines/front_ms 200, render/thread/ms 47.98, render/busy/ms 46.81, main/busy/ms 27.66, stage/render/prepare_assets 22.00, stage/main/update 12.61, stage/render/render 9.57, stage/main/post_update 9.42, render/graph/ms 7.96, encode//sum 7.21, pipelines/driver_ms 6.55 |
| warm-up | 5.55 | 36.0 | render/thread/ms 64.29, render/busy/ms 63.03, main/busy/ms 31.67, stage/render/prepare_assets 18.12, stage/render/render 14.41, stage/main/update 14.21, stage/render/specialize 13.31, render/graph/ms 12.53, stage/main/post_update 11.96, encode//sum 10.62, stage/render/graph_submit 8.50, encode/submit 5.36 |
| warm-up | 5.59 | 36.6 | render/thread/ms 67.97, render/busy/ms 66.66, main/busy/ms 30.31, stage/render/prepare_assets 21.87, stage/render/render 16.01, render/graph/ms 14.10, stage/main/update 13.48, encode//sum 11.91, stage/render/specialize 11.50, stage/main/post_update 11.31, stage/render/graph_submit 9.17, encode/submit 5.81 |
| warm-up | 5.62 | 36.2 | render/thread/ms 59.53, render/busy/ms 58.28, main/busy/ms 30.84, stage/render/prepare_assets 28.87, stage/main/update 14.84, stage/render/render 10.24, stage/main/post_update 9.38, render/graph/ms 8.63, encode//sum 8.24, stage/render/specialize 7.27, stage/render/graph_submit 5.86, stream/drive/ms 4.62 |
| warm-up | 5.66 | 38.9 | render/thread/ms 60.83, render/busy/ms 59.69, stage/render/prepare_assets 34.00, main/busy/ms 29.94, pipelines/compile_ms 22.78, pipelines/front_ms 21.87, stage/main/update 13.83, stage/main/post_update 9.20, stage/render/render 8.96, render/graph/ms 7.48, encode//sum 6.66, stage/render/specialize 6.05 |
| warm-up | 5.71 | 27.6 | render/thread/ms 41.60, render/busy/ms 40.39, main/busy/ms 31.69, stage/main/update 16.04, stage/render/prepare_assets 11.11, stage/main/post_update 10.18, stage/render/render 9.17, render/graph/ms 7.81, stage/render/specialize 7.16, encode//sum 5.89, stage/render/graph_submit 5.32, stream/drive/ms 5.30 |
| warm-up | 5.77 | 42.3 | render/thread/ms 54.60, render/busy/ms 53.31, main/busy/ms 38.93, stage/main/update 20.15, stage/render/prepare_assets 15.61, stage/main/post_update 12.15, stage/render/render 11.29, render/graph/ms 9.75, stage/render/specialize 8.91, encode//sum 8.36, stage/render/graph_submit 7.30, stage/render/prepare_meshes 5.11 |
| warm-up | 5.84 | 41.0 | pipelines/compile_ms 151, pipelines/front_ms 145, render/thread/ms 45.21, main/busy/ms 44.87, render/busy/ms 43.55, stage/main/update 23.24, stage/render/prepare_assets 16.33, stream/drive/ms 13.89, stage/main/post_update 13.23, stage/render/render 12.47, render/graph/ms 10.20, encode//sum 8.83 |
| warm-up | 5.87 | 32.2 | pipelines/compile_ms 171, pipelines/front_ms 163, render/thread/ms 50.68, render/busy/ms 48.69, main/busy/ms 45.34, stage/main/update 22.03, stage/render/prepare_assets 19.05, stage/render/render 15.67, stage/main/post_update 15.65, render/graph/ms 13.01, encode//sum 9.83, stage/render/graph_submit 8.82 |
| warm-up | 5.90 | 34.6 | render/thread/ms 49.72, render/busy/ms 47.84, main/busy/ms 38.61, pipelines/compile_ms 29.99, pipelines/front_ms 27.63, stage/main/update 20.41, stage/render/prepare_assets 20.21, stage/render/render 15.36, render/graph/ms 12.99, stage/main/post_update 11.86, stage/render/graph_submit 9.69, encode//sum 8.78 |
| warm-up | 6.07 | 28.7 | pipelines/compile_ms 143, pipelines/front_ms 141, render/thread/ms 34.96, main/busy/ms 34.52, render/busy/ms 33.64, stage/main/update 22.53, stream/drive/ms 14.85, stage/render/render 10.38, render/graph/ms 8.70, stage/render/prepare_assets 8.54, stage/main/post_update 7.63, encode//sum 6.93 |
| warm-up | 6.12 | 26.6 | main/busy/ms 35.59, render/thread/ms 35.46, render/busy/ms 34.13, stage/main/update 16.69, stage/main/post_update 12.96, pipelines/compile_ms 11.53, stage/render/render 10.67, pipelines/front_ms 10.31, render/graph/ms 9.18, encode//sum 8.26, stage/render/specialize 7.03, stage/render/prepare_assets 6.48 |
| warm-up | 6.15 | 27.1 | render/thread/ms 44.41, render/busy/ms 43.20, main/busy/ms 31.01, stage/main/update 17.50, stage/render/prepare_assets 14.73, stage/render/render 9.38, stage/main/post_update 9.00, stage/render/specialize 8.12, encode//sum 8.03, render/graph/ms 7.89, stage/render/graph_submit 5.59, encode/submit 3.52 |
| warm-up | 6.39 | 37.0 | render/thread/ms 48.93, render/busy/ms 47.28, main/busy/ms 33.99, stage/render/render 15.89, stage/main/update 15.52, stage/main/post_update 13.93, render/graph/ms 13.77, encode//sum 11.35, stage/render/graph_submit 9.51, stage/render/specialize 7.23, stage/render/prepare_assets 6.84, encode/submit 6.33 |
| warm-up | 6.46 | 52.2 | pipelines/compile_ms 144, pipelines/front_ms 141, render/thread/ms 52.18, render/busy/ms 51.08, main/busy/ms 40.24, stage/render/prepare_assets 30.88, stage/main/update 20.05, stage/main/post_update 13.33, stream/drive/ms 9.89, stage/render/render 8.33, encode//sum 7.59, render/graph/ms 6.69 |
| warm-up | 6.86 | 29.4 | pipelines/compile_ms 145, pipelines/front_ms 142, render/thread/ms 37.08, render/busy/ms 36.00, main/busy/ms 17.78, stage/main/post_update 10.40, stage/render/render 10.33, stage/render/specialize 9.10, render/graph/ms 8.85, encode//sum 7.29, stage/render/graph_submit 6.80, stage/render/queue 6.14 |
| warm-up | 8.11 | 31.2 | render/thread/ms 38.58, render/busy/ms 36.19, main/busy/ms 13.52, stage/render/specialize 12.89, stage/render/render 10.41, render/graph/ms 9.07, stage/main/post_update 7.65, stage/render/graph_submit 7.01, encode//sum 6.73, stage/render/queue 6.68, encode/submit 4.25, stage/main/update 3.65 |
| measure | 0.84 | 26.9 | render/thread/ms 33.98, render/busy/ms 31.95, main/busy/ms 12.92, stage/render/render 11.30, render/graph/ms 9.86, encode//sum 9.30, stage/render/specialize 8.55, pipelines/compile_ms 8.51, pipelines/front_ms 8.26, stage/render/graph_submit 7.72, stage/main/post_update 7.26, stage/render/queue 6.01 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=nocull-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 0%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 82%) · 2617 MHz (min 2610) · memory 8101 MHz · 96.8 W · 79 °C (max 81) · VRAM 6447 MiB · throttled 100% of samples

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
