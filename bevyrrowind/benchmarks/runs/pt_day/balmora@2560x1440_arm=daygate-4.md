# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.day_gate=on (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1562 frames over 15.0 s · **median 9.60 ms (104 fps)** · mean 9.61 · p95 11.04 · p99 12.47 (1% low 80 fps) · worst 25.86 ms · 7 spikes over 1.5× median · 28.0 stutters/min (work 12.0/min)
- warm-up: 277 frames over 8.7 s, worst 2575.6 ms
- hitches (> 25.0 ms): **1** measured, 58 in warm-up
- timed GPU passes: 8.33 ms of a 9.61 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.60 ms, limit 11 ms
- pass worst frame 25.86 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.780 | 2.825 | 2.958 | 0.98 | 0.074 |
| dlss_super_resolution | 1.988 | 2.020 | 2.044 | 0.98 | 0.181 |
| early prepass | 0.603 | 0.612 | 0.628 | 0.98 | 0.095 |
| atmosphere_luts | 0.320 | 0.325 | 0.393 | 0.98 | 0.004 |
| bloom | 0.310 | 0.315 | 0.331 | 0.98 | 0.060 |
| main_transmissive_pass_3d | 0.287 | 0.292 | 0.330 | 0.98 | 0.008 |
| water_velocity | 0.235 | 0.239 | 0.266 | 0.98 | 0.002 |
| water_sspr | 0.202 | 0.205 | 0.211 | 0.98 | 0.002 |
| ssao | 0.188 | 0.191 | 0.197 | 0.98 | 0.007 |
| clouds_trace | 0.147 | 0.150 | 0.171 | 0.98 | 0.001 |
| atmos_froxel_inject | 0.142 | 0.144 | 0.152 | 0.98 | 0.001 |
| water_fft | 0.129 | 0.131 | 0.135 | 0.98 | 0.003 |
| uniform_allocation | 0.116 | 0.118 | 0.155 | 0.98 | 0.022 |
| clouds_composite | 0.111 | 0.112 | 0.114 | 0.98 | 0.002 |
| upscaling | 0.110 | 0.112 | 0.118 | 0.98 | 0.004 |
| bin_unpacking | 0.089 | 0.091 | 0.123 | 0.98 | 0.016 |
| tonemapping | 0.088 | 0.089 | 0.095 | 0.98 | 0.004 |
| clustering | 0.066 | 0.067 | 0.079 | 0.98 | 0.020 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.050 | 0.411 | 0.414 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.051 | 0.052 | 0.98 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.036 | 0.98 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.031 | 0.032 | 0.98 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.030 | 0.040 | 0.98 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.220 | 0.230 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.026 | 0.029 | 0.98 | 0.083 |
| water_snapshot | 0.024 | 0.024 | 0.026 | 0.98 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.98 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.046 | 0.31 | 0.002 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| per_view_shadow_pass | 0.011 | 0.012 | 0.023 | 0.98 | 0.005 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 0.98 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.014 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.008 | 0.98 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.98 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.646 | 10.103 | 24.62 | 13505.0 |
| main/busy/ms | 7.229 | 8.091 | 9.46 | 11292.2 |
| render/busy/ms | 7.148 | 8.319 | 24.12 | 11165.0 |
| stage/render/render | 4.842 | 5.926 | 11.32 | 7563.1 |
| stage/main/post_update | 4.269 | 4.971 | 6.43 | 6667.7 |
| render/graph/ms | 4.239 | 5.235 | 8.70 | 6620.9 |
| encode//sum | 4.022 | 5.497 | 8.74 | 6282.9 |
| stage/render/graph_submit | 3.243 | 4.214 | 6.67 | 5065.0 |
| encode/submit | 2.086 | 2.673 | 8.51 | 3257.9 |
| stage/main/update | 1.839 | 2.211 | 3.51 | 2872.9 |
| encode/finish_wall | 1.196 | 1.692 | 3.25 | 1867.7 |
| stage/render/prepare_views | 1.159 | 2.883 | 4.45 | 1810.3 |
| render/acquire/ms | 1.115 | 2.827 | 4.34 | 1740.9 |
| stage/render/camera_driver | 0.942 | 1.076 | 1.92 | 1472.1 |
| encode/pass:shadow_directional_light/sum | 0.754 | 1.330 | 3.03 | 1177.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.716 | 1.007 | 1.69 | 1118.2 |
| encode/pass:early prepass/sum | 0.707 | 1.011 | 1.62 | 1104.1 |
| stage/render/prepare_write_phase_buffers | 0.542 | 0.751 | 1.10 | 846.0 |
| stage/render/prepare_assets | 0.459 | 0.592 | 1.60 | 716.8 |
| bigspace/total/ms | 0.400 | 1.044 | 1.73 | 625.0 |
| stage/render/prepare_resources | 0.386 | 0.449 | 0.78 | 603.5 |
| render/present/ms | 0.384 | 0.465 | 5.01 | 599.1 |
| stage/main/pre_update | 0.348 | 0.470 | 0.87 | 544.0 |
| stage/main/first | 0.293 | 0.361 | 1.62 | 458.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.276 | 0.440 | 0.77 | 431.2 |
| terrain/update/ms | 0.232 | 0.303 | 0.91 | 361.9 |
| stage/render/prepare_bind_groups | 0.210 | 0.285 | 0.47 | 327.3 |
| stage/render/prepare_meshes | 0.210 | 0.269 | 2.33 | 327.2 |
| terrain/select/ms | 0.201 | 0.262 | 0.86 | 314.4 |
| stream/drive/ms | 0.191 | 0.258 | 0.72 | 299.1 |
| stage/main/fixed_main_loop | 0.183 | 0.363 | 0.85 | 286.2 |
| stage/render/prepare_batch_phases | 0.177 | 0.233 | 0.36 | 276.4 |
| stage/render/extract_commands | 0.173 | 0.208 | 0.63 | 270.7 |
| stage/render/create_views | 0.171 | 0.226 | 0.76 | 267.1 |
| far/select/ms | 0.168 | 0.243 | 0.56 | 263.1 |
| controller/move/ms | 0.164 | 0.281 | 0.60 | 256.0 |
| stream/tick/ms | 0.162 | 0.224 | 0.62 | 253.3 |
| bigspace/high_precision/ms | 0.158 | 0.697 | 1.53 | 247.1 |
| controller/feed/ms | 0.156 | 0.204 | 0.55 | 243.8 |
| stage/render/prepare_flush | 0.151 | 0.289 | 0.65 | 236.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.126 | 0.200 | 0.40 | 196.4 |
| stage/main/last | 0.118 | 0.156 | 0.58 | 183.6 |
| stage/render/queue | 0.110 | 0.140 | 5.87 | 171.2 |
| bigspace/low_precision/ms | 0.097 | 0.211 | 1.45 | 152.3 |
| anim/evaluate/ms | 0.095 | 0.248 | 1.23 | 149.1 |
| stage/render/specialize | 0.088 | 0.111 | 7.92 | 137.2 |
| anim/evaluate_cpu/ms | 0.062 | 0.078 | 0.50 | 96.8 |
| stage/render/prepare_collect_phase_buffers | 0.059 | 0.116 | 0.24 | 91.9 |
| encode/resolve_diagnostics/sum | 0.055 | 0.085 | 0.19 | 85.9 |
| physics/step/ms | 0.028 | 0.056 | 0.10 | 43.5 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.041 | 0.10 | 41.5 |
| weather/map/ms | 0.025 | 0.033 | 0.40 | 39.7 |
| encode/camera_driver_span_end/sum | 0.023 | 0.038 | 0.13 | 36.6 |
| stage/render/cleanup | 0.022 | 0.031 | 0.29 | 35.0 |
| stage/render/phase_sort | 0.015 | 0.031 | 0.07 | 24.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.11 | 17.6 |
| script/update/ms | 0.007 | 0.010 | 0.03 | 11.3 |
| controller/avoid/ms | 0.006 | 0.008 | 0.09 | 10.0 |
| script/input/ms | 0.006 | 0.007 | 0.22 | 8.7 |
| anim/apply/ms | 0.005 | 0.006 | 0.04 | 7.2 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 5.8 |
| controller/write/ms | 0.002 | 0.002 | 0.04 | 2.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 1.0 |
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
| encode//max | ms | 0.54 | 1.80 | 0.30 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.10 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.13 | 0.01 |
| encode/finish_critical | ms | 0.86 | 3.03 | 0.74 |
| encode/pass:early prepass/max | ms | 0.69 | 1.30 | 0.49 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.72 | 1.29 | 0.74 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.40 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.26 | 0.76 | 0.14 |
| encode/pass:shadow_directional_light/max | ms | 0.68 | 3.03 | 0.48 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.19 | 0.02 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.11 | 24.12 | 9.17 |
| gpu/timed/ms | ms | 9.11 | 10.54 | 9.17 |
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
| process/cpu_usage | % | 26.58 | 39.42 | 21.63 |
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
| skin/blocks_changed/count | count | 5 | 6 | 5 |
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
| system/cpu_usage | % | 31.84 | 48.76 | 26.13 |
| system/mem_usage | % | 58.05 | 58.27 | 58.27 |
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
| anim/events/count | count | 0.10 | 2 | 160 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.17 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 26.56 | 52 | 41.5 k |
| encode/bind_walk/init_textures | count | 7167 | 16.3 k | 11.19 M |
| encode/bind_walk/init_textures_skipped | count | 35.6 k | 79.9 k | 55.63 M |
| encode/bind_walk/inits | count | 345 | 721 | 539.5 k |
| encode/bind_walk/inits_skipped | count | 67.01 | 142 | 104.7 k |
| encode/bind_walk/merge_textures | count | 7103 | 16.2 k | 11.09 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 8162 | 6.37 M |
| encode/bind_walk/merges | count | 82.27 | 173 | 128.5 k |
| encode/bind_walk/merges_skipped | count | 50.75 | 105 | 79.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1562 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1562 |
| encode/pass:early prepass/count | count | 1 | 2 | 1562 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1562 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 1562 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 1562 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 1562 |
| encode/queue/command_buffers | count | 36.56 | 72 | 57.1 k |
| encode/queue/submits | count | 2 | 4 | 3124 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.86 M | 2.25 G |
| encode/queue/write_buffer/calls | count | 217 | 468 | 339.5 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 65.3 k | 50.98 M |
| encode/queue/write_buffer_with/calls | count | 9 | 18 | 14.1 k |
| encode/queue/write_texture/bytes | bytes | 11.5 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1562 |
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

## Hitches (> 25.0 ms; slowest 59, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2575.6 | main/busy/ms 2542, stage/main/post_update 2439, stage/main/update 94.09, surface/advance_cpu_ms 65.33, stage/main/pre_update 4.83, stage/main/last 1.15, surface/cell_upload_ms 0.81, stage/main/first 0.58, stream/drive/ms 0.37, stage/main/fixed_main_loop 0.36, weather/map/ms 0.24, bigspace/total/ms 0.09 |
| warm-up | 3.33 | 755.6 | main/busy/ms 2693, stage/main/post_update 2468, render/thread/ms 721, render/busy/ms 546, stage/render/prepare_assets 495, stage/main/update 209, stage/render/prepare_views 170, render/acquire/ms 169, surface/advance_cpu_ms 65.33, stage/render/create_views 28.18, stage/render/render 16.27, render/graph/ms 8.97 |
| warm-up | 3.56 | 233.1 | render/thread/ms 936, render/busy/ms 668, stage/render/prepare_assets 546, encode//sum 399, pipelines/compile_ms 281, main/busy/ms 179, pipelines/front_ms 175, stage/render/render 171, stage/render/prepare_views 170, render/acquire/ms 170, stage/main/update 122, pipelines/driver_ms 107 |
| warm-up | 3.60 | 35.0 | pipelines/compile_ms 573, pipelines/front_ms 452, encode//sum 396, render/thread/ms 237, stage/render/render 169, render/busy/ms 139, main/busy/ms 139, pipelines/driver_ms 121, render/present/ms 96.95, stage/main/update 84.87, render/graph/ms 70.89, stage/render/graph_submit 65.10 |
| warm-up | 3.83 | 231.8 | pipelines/wait_ms 3936, pipelines/compile_ms 2082, pipelines/front_ms 1934, render/thread/ms 170, render/busy/ms 164, pipelines/driver_ms 147, main/busy/ms 139, stage/render/prepare_assets 131, stage/main/update 84.35, stage/main/post_update 43.80, pipelines/lock_ms 27.28, stage/render/render 26.50 |
| warm-up | 3.90 | 72.4 | pipelines/wait_ms 4077, pipelines/compile_ms 2483, pipelines/front_ms 2327, render/thread/ms 216, render/busy/ms 214, stage/render/prepare_assets 185, pipelines/driver_ms 156, main/busy/ms 61.73, stage/main/post_update 27.38, pipelines/lock_ms 26.55, encode//sum 20.68, stage/render/render 19.71 |
| warm-up | 3.95 | 41.8 | pipelines/compile_ms 1103, pipelines/front_ms 1066, pipelines/wait_ms 160, render/thread/ms 109, render/busy/ms 82.48, stage/render/prepare_assets 58.72, main/busy/ms 44.49, pipelines/driver_ms 37.08, encode//sum 28.08, stage/render/prepare_views 24.72, render/acquire/ms 24.53, terrain/update/ms 15.81 |
| warm-up | 4.00 | 55.3 | pipelines/compile_ms 764, pipelines/front_ms 728, render/thread/ms 94.75, pipelines/wait_ms 73.41, render/busy/ms 47.23, stage/render/prepare_views 44.61, render/acquire/ms 44.43, main/busy/ms 36.25, pipelines/driver_ms 36.16, stage/render/render 27.05, render/graph/ms 23.35, stage/render/graph_submit 20.43 |
| warm-up | 4.06 | 44.2 | pipelines/compile_ms 1230, pipelines/front_ms 1175, render/thread/ms 55.96, pipelines/driver_ms 55.78, render/busy/ms 54.19, stage/render/prepare_assets 32.65, main/busy/ms 30.60, stage/main/post_update 15.90, stage/render/render 13.38, render/graph/ms 11.05, terrain/update/ms 9.51, stage/render/graph_submit 8.72 |
| warm-up | 4.10 | 38.3 | pipelines/compile_ms 960, pipelines/front_ms 927, render/thread/ms 75.28, render/busy/ms 73.51, main/busy/ms 68.13, stage/render/prepare_assets 52.61, stage/main/update 47.86, stream/drive/ms 44.15, pipelines/driver_ms 33.06, stage/main/post_update 13.99, stage/render/render 12.82, pipelines/wait_ms 10.86 |
| warm-up | 4.16 | 58.4 | pipelines/compile_ms 801, pipelines/front_ms 777, render/thread/ms 68.62, render/busy/ms 66.94, main/busy/ms 65.48, stage/render/prepare_assets 48.93, stage/main/update 48.17, stream/drive/ms 44.06, pipelines/driver_ms 24.54, pipelines/wait_ms 13.00, stage/render/render 10.71, stage/main/post_update 9.97 |
| warm-up | 4.23 | 59.4 | pipelines/compile_ms 199, pipelines/front_ms 192, render/thread/ms 44.35, render/busy/ms 42.87, main/busy/ms 36.77, stage/render/prepare_assets 24.51, stage/main/update 24.04, stream/drive/ms 19.65, stage/render/render 13.30, render/graph/ms 11.17, encode//sum 11.09, stage/render/graph_submit 8.00 |
| warm-up | 4.27 | 41.4 | pipelines/compile_ms 610, pipelines/front_ms 590, render/thread/ms 69.65, render/busy/ms 67.82, stage/render/prepare_assets 43.81, main/busy/ms 40.86, stage/main/update 25.73, stream/drive/ms 20.45, pipelines/driver_ms 19.79, stage/render/render 15.45, render/graph/ms 12.89, encode//sum 10.99 |
| warm-up | 4.35 | 79.2 | pipelines/compile_ms 1053, pipelines/front_ms 1022, render/thread/ms 81.10, render/busy/ms 79.39, stage/render/prepare_assets 56.58, main/busy/ms 47.49, pipelines/driver_ms 31.03, stage/main/update 31.00, stream/drive/ms 20.56, stage/render/render 14.45, render/graph/ms 12.22, stage/main/post_update 9.28 |
| warm-up | 4.42 | 71.3 | pipelines/compile_ms 749, pipelines/front_ms 726, main/busy/ms 103, render/thread/ms 94.41, render/busy/ms 92.57, stage/main/update 87.17, stage/render/prepare_assets 69.34, stream/drive/ms 62.13, pipelines/driver_ms 23.42, stage/render/render 15.08, render/graph/ms 12.60, stage/render/graph_submit 9.86 |
| warm-up | 4.51 | 89.3 | pipelines/compile_ms 124, pipelines/front_ms 118, main/busy/ms 112, stage/main/update 92.54, render/thread/ms 89.39, render/busy/ms 87.54, stream/drive/ms 62.01, stage/render/prepare_assets 53.32, stage/render/render 22.40, render/graph/ms 19.44, stage/render/graph_submit 14.47, encode//sum 11.46 |
| warm-up | 4.56 | 44.1 | pipelines/compile_ms 650, pipelines/front_ms 623, render/thread/ms 79.53, render/busy/ms 77.08, main/busy/ms 75.47, stage/main/update 37.83, stage/render/prepare_assets 30.71, pipelines/driver_ms 27.40, stage/main/post_update 26.85, stage/render/render 25.82, render/graph/ms 22.17, stage/render/graph_submit 14.45 |
| warm-up | 4.63 | 74.5 | pipelines/compile_ms 1315, pipelines/front_ms 1262, pipelines/wait_ms 186, render/thread/ms 78.30, render/busy/ms 75.65, main/busy/ms 71.61, pipelines/driver_ms 53.50, stage/render/prepare_assets 36.98, stage/main/update 33.52, stage/main/post_update 28.26, stage/render/render 16.69, stream/drive/ms 14.12 |
| warm-up | 4.71 | 81.7 | pipelines/compile_ms 784, pipelines/front_ms 752, pipelines/wait_ms 267, render/thread/ms 108, render/busy/ms 106, stage/render/prepare_assets 73.54, main/busy/ms 70.98, stage/main/update 37.00, pipelines/driver_ms 31.78, stage/main/post_update 26.72, stage/render/render 10.45, stage/render/prepare_bind_groups 8.06 |
| warm-up | 4.76 | 50.3 | pipelines/compile_ms 1261, pipelines/front_ms 1225, render/thread/ms 102, render/busy/ms 99.64, pipelines/wait_ms 86.39, stage/render/prepare_assets 70.70, main/busy/ms 63.10, pipelines/driver_ms 36.05, stage/main/update 32.65, stage/main/post_update 22.79, stage/render/render 10.07, stage/render/prepare_bind_groups 7.93 |
| warm-up | 4.81 | 42.3 | pipelines/compile_ms 2072, pipelines/front_ms 2014, render/thread/ms 67.65, render/busy/ms 66.29, pipelines/wait_ms 62.37, pipelines/driver_ms 57.66, stage/render/prepare_assets 44, main/busy/ms 36.64, stage/main/update 14.87, stage/main/post_update 14.32, stage/render/render 9.68, render/graph/ms 7.83 |
| warm-up | 4.85 | 45.0 | pipelines/compile_ms 1405, pipelines/front_ms 1367, pipelines/wait_ms 141, render/thread/ms 65.73, render/busy/ms 64.42, main/busy/ms 43.00, stage/render/prepare_assets 38.61, pipelines/driver_ms 37.73, stage/main/post_update 25.74, stage/main/update 11.57, stage/render/render 10.38, render/graph/ms 8.53 |
| warm-up | 5.00 | 121.6 | pipelines/compile_ms 1980, pipelines/front_ms 1923, pipelines/wait_ms 137, main/busy/ms 130, stage/main/update 116, stream/drive/ms 93.87, render/thread/ms 87.10, render/busy/ms 85.02, pipelines/driver_ms 56.66, stage/render/prepare_write_phase_buffers 28.78, stage/render/prepare_resources 21.29, stage/render/prepare_assets 19.26 |
| warm-up | 5.04 | 40.6 | pipelines/compile_ms 1783, pipelines/front_ms 1730, pipelines/wait_ms 103, render/thread/ms 98.56, main/busy/ms 97.57, render/busy/ms 96.87, stage/main/update 64.51, pipelines/driver_ms 53.05, stream/drive/ms 42.14, stage/render/prepare_write_phase_buffers 29.27, stage/render/prepare_assets 24.41, stage/render/prepare_resources 21.56 |
| warm-up | 5.07 | 30.4 | pipelines/compile_ms 705, pipelines/front_ms 683, main/busy/ms 65.48, render/thread/ms 55.14, render/busy/ms 53.25, stage/main/update 28.46, pipelines/driver_ms 22.98, stage/render/render 20.94, render/graph/ms 18.50, stage/render/prepare_assets 14.65, stage/render/graph_submit 13.16, stage/main/last 13.01 |
| warm-up | 5.13 | 58.3 | pipelines/compile_ms 1103, pipelines/front_ms 1066, main/busy/ms 69.61, render/thread/ms 59.81, render/busy/ms 58.08, stage/main/update 51.17, pipelines/driver_ms 37.17, stage/render/prepare_assets 20.03, stage/render/render 16.55, render/graph/ms 14.33, stream/drive/ms 11.97, pipelines/wait_ms 11.63 |
| warm-up | 5.16 | 37.0 | pipelines/compile_ms 1269, pipelines/front_ms 1234, pipelines/wait_ms 203, render/thread/ms 60.67, render/busy/ms 59.39, main/busy/ms 50.07, stage/main/update 35.00, pipelines/driver_ms 34.86, stage/render/prepare_assets 30.95, stream/drive/ms 12.17, stage/render/render 10.11, stage/main/post_update 8.95 |
| warm-up | 5.21 | 25.1 | pipelines/compile_ms 256, pipelines/front_ms 248, render/thread/ms 38.09, render/busy/ms 36.46, main/busy/ms 36.13, stage/main/update 20.06, stage/render/render 12.37, render/graph/ms 10.53, stage/main/post_update 9.82, pipelines/driver_ms 7.89, stage/render/graph_submit 7.19, encode//sum 6.56 |
| warm-up | 5.24 | 28.9 | pipelines/compile_ms 513, pipelines/front_ms 495, render/thread/ms 44.70, render/busy/ms 43.22, main/busy/ms 40.86, stage/main/update 24.99, pipelines/driver_ms 18.20, stream/drive/ms 15.90, stage/render/prepare_assets 12.42, stage/render/render 11.34, stage/main/post_update 9.92, render/graph/ms 9.62 |
| warm-up | 5.29 | 35.4 | pipelines/compile_ms 280, pipelines/front_ms 270, render/thread/ms 50.91, render/busy/ms 47.75, main/busy/ms 45.93, stage/main/update 30.74, stage/render/render 13.05, stage/render/prepare_write_phase_buffers 12.62, render/graph/ms 11.03, pipelines/driver_ms 10.16, stage/main/post_update 9.61, encode//sum 8.22 |
| warm-up | 5.34 | 50.2 | pipelines/compile_ms 348, pipelines/front_ms 337, render/thread/ms 72.79, render/busy/ms 69.53, main/busy/ms 49.98, stage/main/update 29.81, stage/render/prepare_assets 25.95, stage/main/post_update 14.01, stage/render/render 12.65, stage/render/prepare_write_phase_buffers 11.79, pipelines/driver_ms 11.26, render/graph/ms 10.52 |
| warm-up | 5.41 | 54.6 | pipelines/compile_ms 603, pipelines/front_ms 588, render/thread/ms 52.04, render/busy/ms 50.26, main/busy/ms 38.63, stage/main/update 20.63, stage/render/prepare_assets 17.86, pipelines/driver_ms 15.33, stream/drive/ms 14.43, stage/render/render 14.33, render/graph/ms 12.01, stage/main/post_update 11.04 |
| warm-up | 5.47 | 56.7 | pipelines/compile_ms 1655, pipelines/front_ms 1611, render/thread/ms 91.47, render/busy/ms 89.67, main/busy/ms 69.96, stage/main/post_update 46.86, pipelines/driver_ms 44.13, stage/render/render 30.78, render/graph/ms 28.67, stage/render/graph_submit 19.68, stage/render/prepare_flush 18.39, stage/render/prepare_assets 18.25 |
| warm-up | 5.52 | 47.0 | pipelines/compile_ms 2148, pipelines/front_ms 2088, render/thread/ms 88.83, render/busy/ms 87.04, pipelines/wait_ms 74.51, main/busy/ms 73.21, pipelines/driver_ms 59.72, stage/main/post_update 46.36, stage/render/render 32.47, render/graph/ms 30.57, stage/render/graph_submit 22.86, stage/render/prepare_assets 22.71 |
| warm-up | 5.55 | 32.1 | pipelines/compile_ms 964, pipelines/front_ms 937, pipelines/wait_ms 62.57, render/thread/ms 57.22, render/busy/ms 55.74, main/busy/ms 39.90, stage/render/prepare_assets 30.65, pipelines/driver_ms 27.62, stage/main/update 22.03, stage/render/render 15.18, render/graph/ms 13.31, stream/drive/ms 11.96 |
| warm-up | 5.58 | 27.6 | pipelines/compile_ms 130, pipelines/front_ms 122, render/thread/ms 49.52, render/busy/ms 48.11, main/busy/ms 42.15, stage/main/update 22.03, stage/render/prepare_assets 15.71, stage/main/post_update 13.41, stage/render/render 13.34, stream/drive/ms 11.62, render/graph/ms 11.21, stage/render/graph_submit 8.43 |
| warm-up | 5.62 | 25.2 | pipelines/compile_ms 961, pipelines/front_ms 938, render/thread/ms 39.54, render/busy/ms 38.36, main/busy/ms 27.27, pipelines/driver_ms 22.15, stage/render/prepare_assets 17.64, stage/main/update 12.40, pipelines/wait_ms 12.26, stage/render/render 9.87, stage/main/post_update 9.47, render/graph/ms 8.35 |
| warm-up | 5.67 | 37.3 | pipelines/compile_ms 388, pipelines/front_ms 380, pipelines/wait_ms 65.80, render/thread/ms 38.96, render/busy/ms 37.34, main/busy/ms 36.61, stage/main/update 19.33, stage/render/prepare_assets 14.30, stage/render/render 14.24, render/graph/ms 11.91, stage/main/post_update 10.68, encode//sum 8.68 |
| warm-up | 5.75 | 48.9 | pipelines/compile_ms 201, pipelines/front_ms 196, render/thread/ms 63.89, render/busy/ms 62.33, main/busy/ms 32.97, stage/render/prepare_assets 25.90, stage/render/render 14.64, stage/main/update 13.93, render/graph/ms 12.60, stage/main/post_update 12.09, stage/render/graph_submit 9.87, stage/render/specialize 8.05 |
| warm-up | 5.78 | 34.4 | pipelines/compile_ms 303, pipelines/front_ms 294, render/thread/ms 74.58, render/busy/ms 73.09, main/busy/ms 38.91, stage/render/prepare_assets 35.33, stage/main/update 17.42, stage/render/render 14.59, stage/main/post_update 14.28, render/graph/ms 12.47, pipelines/wait_ms 10.26, stage/render/graph_submit 9.52 |
| warm-up | 5.82 | 42.4 | pipelines/compile_ms 328, pipelines/front_ms 315, render/thread/ms 60.35, render/busy/ms 59.16, pipelines/wait_ms 53.76, main/busy/ms 44.79, stage/render/prepare_assets 27.61, stage/main/update 20.74, stage/main/post_update 16.28, pipelines/driver_ms 13.30, stage/render/render 11.71, render/graph/ms 9.93 |
| warm-up | 5.85 | 29.0 | pipelines/compile_ms 226, pipelines/front_ms 217, render/thread/ms 55.36, render/busy/ms 53.71, pipelines/wait_ms 44.28, main/busy/ms 43.40, stage/render/prepare_assets 21.44, stage/main/update 19.40, stage/main/post_update 16.90, stage/render/render 13.06, render/graph/ms 11.28, encode//sum 8.98 |
| warm-up | 5.92 | 29.6 | main/busy/ms 44.07, render/thread/ms 38.58, render/busy/ms 36.59, stage/main/update 25.63, stage/render/render 18.05, render/graph/ms 15.56, stage/main/post_update 12.77, stage/render/graph_submit 10.88, encode//sum 10.56, stage/render/prepare_assets 8.79, stream/drive/ms 6.38, encode/submit 6.34 |
| warm-up | 5.94 | 28.8 | pipelines/compile_ms 1165, pipelines/front_ms 1135, main/busy/ms 42.69, render/thread/ms 42.15, render/busy/ms 40.54, pipelines/driver_ms 30.71, stage/main/update 23.08, stage/render/prepare_assets 17.83, stage/render/render 12.71, stage/main/post_update 12.52, render/graph/ms 10.67, encode//sum 8.09 |
| warm-up | 6.00 | 35.2 | main/busy/ms 52.51, render/thread/ms 44.68, render/busy/ms 37.65, stage/main/post_update 33.63, stage/render/render 26.68, render/graph/ms 18.98, pipelines/compile_ms 16.11, pipelines/front_ms 14.84, stage/main/update 12.95, stage/render/graph_submit 12.51, encode//sum 10.22, encode/submit 7.18 |
| warm-up | 6.06 | 60.8 | pipelines/compile_ms 116, pipelines/front_ms 111, main/busy/ms 65.90, render/thread/ms 62.99, render/busy/ms 56.21, stage/main/post_update 34.45, pipelines/wait_ms 28.12, stage/render/prepare_assets 28.07, pipelines/lock_ms 27.62, stage/render/render 23.67, stage/main/update 22.03, render/graph/ms 16.17 |
| warm-up | 6.09 | 29.9 | pipelines/compile_ms 109, pipelines/front_ms 105, render/thread/ms 56.37, render/busy/ms 55.09, main/busy/ms 42.79, stage/render/prepare_assets 32.93, pipelines/wait_ms 28.10, pipelines/lock_ms 27.62, stage/main/update 19.59, stage/main/post_update 13.43, stage/render/render 11.75, render/graph/ms 10.02 |
| warm-up | 6.20 | 27.8 | pipelines/compile_ms 1465, pipelines/front_ms 1416, pipelines/driver_ms 49.02, render/thread/ms 44.44, render/busy/ms 43.07, main/busy/ms 27.85, stage/render/prepare_assets 23.98, stage/main/update 12.36, pipelines/wait_ms 11.95, pipelines/lock_ms 11.47, stage/main/post_update 10.99, stage/render/render 9.97 |
| warm-up | 6.22 | 25.1 | render/thread/ms 49.10, render/busy/ms 47.61, main/busy/ms 46.34, pipelines/compile_ms 45.75, pipelines/front_ms 43.32, stage/main/update 29.32, stage/render/prepare_assets 28.33, stream/drive/ms 18.94, stage/main/post_update 12.32, stage/render/render 11.91, render/graph/ms 9.88, encode//sum 7.74 |
| warm-up | 6.27 | 43.0 | pipelines/compile_ms 293, pipelines/front_ms 281, render/thread/ms 52.45, render/busy/ms 50.91, main/busy/ms 44.14, stage/main/update 26.99, stage/render/prepare_assets 26.68, stream/drive/ms 19.24, stage/render/render 13.79, stage/main/post_update 11.86, pipelines/driver_ms 11.78, render/graph/ms 11.59 |
| warm-up | 6.29 | 28.9 | pipelines/compile_ms 317, pipelines/front_ms 305, render/thread/ms 55.00, render/busy/ms 53.70, main/busy/ms 30.26, stage/render/prepare_assets 26.25, stage/main/post_update 12.64, stage/render/render 12.19, pipelines/driver_ms 11.96, stage/main/update 11.01, render/graph/ms 10.26, encode//sum 7.43 |
| warm-up | 6.32 | 25.4 | pipelines/compile_ms 69.82, pipelines/front_ms 67.22, render/thread/ms 48.95, render/busy/ms 47.75, main/busy/ms 39.45, stage/render/prepare_assets 21.33, stage/main/update 17.81, stage/main/post_update 15.03, stage/render/render 11.94, encode//sum 10.21, render/graph/ms 10.11, stream/drive/ms 9.59 |
| warm-up | 6.37 | 32.8 | pipelines/compile_ms 195, pipelines/front_ms 188, render/thread/ms 47.47, render/busy/ms 46.15, main/busy/ms 33.82, stage/render/prepare_assets 24.54, stage/render/render 13.48, stage/main/post_update 13.40, stage/main/update 13.13, render/graph/ms 11.65, encode//sum 8.59, stage/render/graph_submit 8.37 |
| warm-up | 6.48 | 28.5 | pipelines/compile_ms 961, pipelines/front_ms 940, main/busy/ms 38.93, render/thread/ms 29.27, render/busy/ms 28.16, stage/main/update 26.16, pipelines/driver_ms 20.72, stream/drive/ms 19.29, stage/render/render 11.98, render/graph/ms 10.54, stage/main/post_update 8.10, stage/render/prepare_assets 7.86 |
| warm-up | 6.71 | 31.0 | render/thread/ms 40.90, render/busy/ms 38.12, main/busy/ms 34.53, stage/render/render 16.52, stage/main/post_update 15.92, stage/main/update 13.22, render/graph/ms 12.70, stage/render/prepare_assets 12.05, encode//sum 10.60, stage/render/graph_submit 8.91, encode/submit 5.51, stream/drive/ms 4.76 |
| warm-up | 6.74 | 29.7 | render/thread/ms 42.59, render/busy/ms 40.16, main/busy/ms 29.79, stage/render/prepare_assets 19.97, stage/main/post_update 13.92, stage/render/render 11.70, stage/main/update 10.38, render/graph/ms 8.32, encode//sum 8.19, stage/render/graph_submit 5.48, stream/drive/ms 4.87, encode/finish_wall 2.85 |
| warm-up | 7.11 | 31.9 | render/thread/ms 36.40, render/busy/ms 35.31, main/busy/ms 15.16, stage/render/render 9.38, stage/render/specialize 9.36, stage/main/post_update 8.18, render/graph/ms 8.10, stage/render/graph_submit 6.18, encode//sum 6.13, stage/render/queue 5.84, stage/main/update 4.03, stage/render/prepare_meshes 3.49 |
| warm-up | 8.32 | 29.1 | render/thread/ms 35.73, render/busy/ms 33.01, main/busy/ms 14.10, stage/render/render 9.88, render/graph/ms 8.64, stage/render/specialize 8.57, stage/main/post_update 8.02, stage/render/graph_submit 6.66, stage/render/queue 5.85, encode//sum 5.81, encode/submit 4.06, stage/render/prepare_meshes 4.05 |
| measure | 0.85 | 25.9 | render/thread/ms 33.20, render/busy/ms 30.21, main/busy/ms 12.90, stage/render/render 9.53, render/graph/ms 8.30, stage/render/specialize 8.01, stage/main/post_update 7.24, stage/render/graph_submit 6.35, stage/render/queue 5.97, encode//sum 5.67, encode/submit 3.62, stage/main/update 3.56 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set shadow.lamp.day_gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=daygate-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 7%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 98.7 W · 77 °C (max 79) · VRAM 6426 MiB · throttled 100% of samples

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
