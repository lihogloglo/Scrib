# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.skin.skip_unchanged=on (command line: --set); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1549 frames over 15.0 s · **median 9.68 ms (103 fps)** · mean 9.69 · p95 11.11 · p99 11.60 (1% low 86 fps) · worst 27.36 ms · 4 spikes over 1.5× median · 16.0 stutters/min (work 8.0/min)
- warm-up: 280 frames over 8.5 s, worst 2599.9 ms
- hitches (> 25.0 ms): **2** measured, 60 in warm-up
- timed GPU passes: 8.54 ms of a 9.69 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.68 ms, limit 11 ms
- pass worst frame 27.36 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.897 | 2.909 | 2.988 | 1.00 | 0.072 |
| dlss_super_resolution | 2.014 | 2.022 | 2.043 | 1.00 | 0.182 |
| early prepass | 0.619 | 0.621 | 0.631 | 1.00 | 0.093 |
| atmosphere_luts | 0.325 | 0.326 | 0.342 | 1.00 | 0.003 |
| bloom | 0.313 | 0.315 | 0.331 | 1.00 | 0.060 |
| main_transmissive_pass_3d | 0.309 | 0.310 | 0.332 | 1.00 | 0.008 |
| water_velocity | 0.239 | 0.240 | 0.264 | 1.00 | 0.002 |
| water_sspr | 0.207 | 0.207 | 0.210 | 1.00 | 0.002 |
| ssao | 0.190 | 0.191 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.148 | 0.148 | 0.168 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.146 | 0.152 | 1.00 | 0.002 |
| water_fft | 0.128 | 0.129 | 0.134 | 1.00 | 0.003 |
| uniform_allocation | 0.119 | 0.120 | 0.162 | 1.00 | 0.023 |
| upscaling | 0.112 | 0.113 | 0.118 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.091 | 0.091 | 0.123 | 1.00 | 0.016 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.075 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.412 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.230 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 1.00 | 0.083 |
| water_snapshot | 0.025 | 0.025 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.045 | 0.047 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 1.00 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.751 | 10.182 | 26.38 | 13555.1 |
| main/busy/ms | 7.204 | 8.016 | 9.56 | 11158.3 |
| render/busy/ms | 7.040 | 8.007 | 25.21 | 10904.9 |
| stage/render/render | 4.744 | 5.584 | 22.87 | 7349.1 |
| stage/main/post_update | 4.274 | 5.005 | 5.98 | 6620.5 |
| render/graph/ms | 4.165 | 4.906 | 22.15 | 6452.2 |
| encode//sum | 4.033 | 5.222 | 22.32 | 6247.4 |
| stage/render/graph_submit | 3.166 | 3.900 | 21.20 | 4903.8 |
| encode/submit | 1.959 | 2.363 | 4.67 | 3033.9 |
| stage/main/update | 1.822 | 2.213 | 3.44 | 2821.9 |
| stage/render/prepare_views | 1.391 | 3.009 | 3.88 | 2154.7 |
| render/acquire/ms | 1.348 | 2.960 | 3.84 | 2088.6 |
| encode/finish_wall | 1.234 | 1.710 | 18.14 | 1911.2 |
| stage/render/camera_driver | 0.944 | 1.082 | 1.34 | 1462.2 |
| encode/pass:shadow_directional_light/sum | 0.789 | 1.355 | 2.10 | 1222.2 |
| encode/pass:early prepass/sum | 0.755 | 1.007 | 17.44 | 1169.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.741 | 1.022 | 1.31 | 1147.6 |
| stage/render/prepare_write_phase_buffers | 0.551 | 0.765 | 0.97 | 852.8 |
| stage/render/prepare_assets | 0.452 | 0.574 | 1.52 | 700.5 |
| stage/render/prepare_resources | 0.370 | 0.426 | 0.60 | 573.7 |
| bigspace/total/ms | 0.370 | 0.855 | 2.12 | 573.3 |
| render/present/ms | 0.363 | 0.426 | 1.84 | 561.6 |
| stage/main/pre_update | 0.342 | 0.428 | 1.03 | 529.4 |
| stage/main/first | 0.291 | 0.351 | 1.79 | 450.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.250 | 0.361 | 0.46 | 387.0 |
| terrain/update/ms | 0.234 | 0.301 | 0.82 | 362.6 |
| stage/render/prepare_bind_groups | 0.212 | 0.292 | 0.49 | 328.0 |
| terrain/select/ms | 0.204 | 0.262 | 0.78 | 315.4 |
| stage/render/prepare_meshes | 0.198 | 0.250 | 2.33 | 306.1 |
| stream/drive/ms | 0.190 | 0.249 | 0.66 | 295.0 |
| stage/main/fixed_main_loop | 0.179 | 0.335 | 0.80 | 276.6 |
| stage/render/prepare_batch_phases | 0.178 | 0.240 | 0.37 | 275.5 |
| stage/render/extract_commands | 0.171 | 0.202 | 0.29 | 264.9 |
| stage/render/create_views | 0.168 | 0.214 | 0.70 | 260.8 |
| far/select/ms | 0.164 | 0.225 | 0.71 | 254.7 |
| controller/move/ms | 0.163 | 0.361 | 0.77 | 252.7 |
| stream/tick/ms | 0.161 | 0.214 | 0.59 | 250.0 |
| controller/feed/ms | 0.155 | 0.199 | 0.59 | 240.1 |
| stage/render/prepare_flush | 0.149 | 0.253 | 0.78 | 230.7 |
| bigspace/high_precision/ms | 0.136 | 0.363 | 1.70 | 211.4 |
| encode/pass:main_transmissive_pass_3d/sum | 0.124 | 0.190 | 0.26 | 191.9 |
| stage/main/last | 0.117 | 0.154 | 0.79 | 181.2 |
| stage/render/queue | 0.107 | 0.134 | 5.77 | 166.5 |
| anim/evaluate/ms | 0.091 | 0.216 | 1.38 | 140.6 |
| stage/render/specialize | 0.085 | 0.108 | 8.58 | 132.3 |
| bigspace/low_precision/ms | 0.085 | 0.203 | 1.34 | 131.6 |
| anim/evaluate_cpu/ms | 0.062 | 0.076 | 0.47 | 95.8 |
| stage/render/prepare_collect_phase_buffers | 0.059 | 0.116 | 0.18 | 91.2 |
| encode/resolve_diagnostics/sum | 0.058 | 0.089 | 0.29 | 89.3 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.041 | 0.10 | 42.4 |
| physics/step/ms | 0.026 | 0.053 | 0.45 | 40.1 |
| encode/camera_driver_span_end/sum | 0.024 | 0.040 | 0.10 | 37.9 |
| weather/map/ms | 0.024 | 0.032 | 0.44 | 37.8 |
| stage/render/cleanup | 0.022 | 0.031 | 0.27 | 34.6 |
| stage/render/phase_sort | 0.016 | 0.030 | 0.17 | 25.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.03 | 16.9 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 10.7 |
| controller/avoid/ms | 0.006 | 0.007 | 0.03 | 8.7 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 8.3 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 6.7 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 5.7 |
| controller/write/ms | 0.002 | 0.002 | 0.03 | 2.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 1.0 |
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
| crowd/joints_extracted/count | count | 102 | 149 | 81 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.56 | 18.12 | 1.01 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.10 | 0.05 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.10 | 0.03 |
| encode/finish_critical | ms | 0.89 | 18.12 | 1.18 |
| encode/pass:early prepass/max | ms | 0.74 | 17.44 | 1.17 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.75 | 1.31 | 1.18 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.26 | 0.12 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.24 | 0.46 | 0.27 |
| encode/pass:shadow_directional_light/max | ms | 0.72 | 2.10 | 0.47 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.29 | 0.07 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.20 | 25.21 | 9.35 |
| gpu/timed/ms | ms | 9.20 | 10.89 | 9.35 |
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
| process/cpu_usage | % | 27.05 | 40.38 | 36.54 |
| process/mem_usage | GiB | 4.67 | 4.74 | 4.74 |
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
| skin/blocks_skipped/count | count | 177 | 180 | 177 |
| skin/blocks_walked/count | count | 4 | 6 | 4 |
| skin/joint_bytes_written/bytes | bytes | 7168 | 10.5 k | 5888 |
| skin/joints_written/count | count | 112 | 164 | 92 |
| skin/upload_writes/count | count | 3 | 4 | 2 |
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
| system/cpu_usage | % | 32.51 | 45.00 | 31.05 |
| system/mem_usage | % | 59.97 | 60.15 | 60.15 |
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
| anim/events/count | count | 0.09 | 2 | 145 |
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
| encode//count | count | 26.57 | 29 | 41.2 k |
| encode/bind_walk/init_textures | count | 7165 | 14.0 k | 11.10 M |
| encode/bind_walk/init_textures_skipped | count | 39.5 k | 52.6 k | 61.16 M |
| encode/bind_walk/inits | count | 345 | 437 | 535.0 k |
| encode/bind_walk/inits_skipped | count | 71.00 | 83 | 110.0 k |
| encode/bind_walk/merge_textures | count | 7101 | 13.9 k | 11.00 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.32 M |
| encode/bind_walk/merges | count | 82.26 | 103 | 127.4 k |
| encode/bind_walk/merges_skipped | count | 52.75 | 58 | 81.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1549 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1549 |
| encode/pass:early prepass/count | count | 1 | 1 | 1549 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1549 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1549 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1549 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1549 |
| encode/queue/command_buffers | count | 36.57 | 39 | 56.6 k |
| encode/queue/submits | count | 2 | 2 | 3098 |
| encode/queue/write_buffer/bytes | bytes | 1.13 M | 3.34 M | 1.76 G |
| encode/queue/write_buffer/calls | count | 219 | 287 | 339.0 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.56 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.9 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1549 |
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

## Hitches (> 25.0 ms; slowest 62, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.60 | 2599.9 | main/busy/ms 2569, stage/main/post_update 2463, stage/main/update 96.80, surface/advance_cpu_ms 67.01, stage/main/pre_update 5.21, stage/main/last 1.67, stream/drive/ms 1.00, surface/cell_upload_ms 0.82, stage/main/first 0.53, stage/main/fixed_main_loop 0.37, weather/map/ms 0.24, stream/tick/ms 0.20 |
| warm-up | 3.26 | 664.8 | main/busy/ms 2744, stage/main/post_update 2490, render/thread/ms 627, render/busy/ms 512, stage/render/prepare_assets 484, stage/main/update 238, stage/render/prepare_views 112, render/acquire/ms 112, surface/advance_cpu_ms 67.01, stage/render/render 11.73, stage/render/create_views 8.63, stage/main/pre_update 7.29 |
| warm-up | 3.37 | 109.8 | render/thread/ms 720, render/busy/ms 551, stage/render/prepare_assets 509, main/busy/ms 194, stage/main/update 147, stage/render/prepare_views 112, render/acquire/ms 112, pipelines/compile_ms 84.20, pipelines/front_ms 78.49, stage/render/render 70.36, render/present/ms 56.45, stage/main/post_update 37.33 |
| warm-up | 3.42 | 45.5 | pipelines/compile_ms 348, pipelines/front_ms 316, render/thread/ms 122, stage/render/render 81.20, main/busy/ms 66.91, render/present/ms 61.22, render/busy/ms 60.01, pipelines/wait_ms 55.22, stage/main/post_update 50.44, pipelines/driver_ms 32.71, stage/render/prepare_assets 26.03, terrain/update/ms 25.49 |
| warm-up | 3.45 | 33.5 | pipelines/wait_ms 766, pipelines/compile_ms 551, pipelines/front_ms 487, main/busy/ms 89.91, stage/main/post_update 78.35, pipelines/driver_ms 64.13, render/thread/ms 57.16, render/busy/ms 47.40, stage/render/render 27.16, pipelines/lock_ms 25.34, stage/render/prepare_assets 18.81, render/graph/ms 17.32 |
| warm-up | 3.55 | 99.7 | pipelines/wait_ms 4941, pipelines/compile_ms 1203, pipelines/front_ms 1103, main/busy/ms 113, render/thread/ms 111, render/busy/ms 108, pipelines/driver_ms 99.61, stage/main/update 51.75, stage/main/post_update 47.99, stage/render/prepare_assets 28.95, pipelines/lock_ms 27.09, stage/render/prepare_resources 23.46 |
| warm-up | 3.63 | 74.6 | pipelines/wait_ms 5364, pipelines/compile_ms 1435, pipelines/front_ms 1344, render/thread/ms 154, render/busy/ms 152, main/busy/ms 116, pipelines/driver_ms 91.33, stage/render/prepare_assets 65.83, stage/main/update 54.11, stage/main/pre_update 42.75, stage/render/prepare_resources 28.67, stage/render/prepare_bind_groups 22.10 |
| warm-up | 3.67 | 39.8 | pipelines/compile_ms 1655, pipelines/front_ms 1589, pipelines/wait_ms 1545, render/thread/ms 102, render/busy/ms 99.86, main/busy/ms 69.95, stage/render/prepare_assets 69.14, pipelines/driver_ms 65.94, stage/main/pre_update 34.00, stage/main/post_update 14.89, stage/render/prepare_resources 12.94, stage/main/update 10.78 |
| warm-up | 3.72 | 48.9 | pipelines/compile_ms 1635, pipelines/front_ms 1571, pipelines/wait_ms 461, render/thread/ms 75.68, render/busy/ms 73.77, pipelines/driver_ms 64.10, stage/render/prepare_assets 42.29, main/busy/ms 35.63, stage/render/render 18.71, render/graph/ms 16.45, stage/main/post_update 14.15, stage/render/graph_submit 13.64 |
| warm-up | 3.75 | 29.2 | pipelines/compile_ms 740, pipelines/front_ms 705, render/thread/ms 73.21, render/busy/ms 71.36, pipelines/wait_ms 56.19, stage/render/prepare_assets 36.54, pipelines/driver_ms 35.08, main/busy/ms 33.29, stage/main/post_update 20.79, stage/render/render 17.71, render/graph/ms 15.21, stage/render/graph_submit 12.76 |
| warm-up | 3.78 | 31.5 | pipelines/compile_ms 981, pipelines/front_ms 935, pipelines/wait_ms 79.35, render/thread/ms 57.72, render/busy/ms 54.97, pipelines/driver_ms 46.11, main/busy/ms 41.94, stage/main/post_update 29.21, encode//sum 17.02, stage/render/prepare_assets 14.92, stage/render/render 14.86, render/graph/ms 11.45 |
| warm-up | 3.81 | 28.3 | pipelines/compile_ms 765, pipelines/front_ms 725, pipelines/wait_ms 73.17, render/thread/ms 56.26, render/busy/ms 53.60, pipelines/driver_ms 39.48, main/busy/ms 30.01, stage/render/prepare_assets 22.04, stage/main/post_update 17.73, encode//sum 17.15, stage/render/render 16.67, render/graph/ms 13.60 |
| warm-up | 3.84 | 39.4 | pipelines/compile_ms 449, pipelines/front_ms 429, render/thread/ms 63.40, render/busy/ms 61.45, main/busy/ms 57.42, stage/main/update 44.64, stage/render/prepare_assets 41.50, stream/drive/ms 40.95, pipelines/driver_ms 19.59, stage/render/render 13.14, render/graph/ms 10.60, stage/render/graph_submit 8.18 |
| warm-up | 3.89 | 49.0 | pipelines/compile_ms 1217, pipelines/front_ms 1170, render/thread/ms 69.62, render/busy/ms 67.94, main/busy/ms 55.65, stage/render/prepare_assets 52.02, pipelines/driver_ms 46.73, stage/main/update 45.21, stream/drive/ms 40.93, stage/render/render 10.12, render/graph/ms 7.98, stage/main/post_update 6.31 |
| warm-up | 3.95 | 48.8 | pipelines/compile_ms 189, pipelines/front_ms 182, render/thread/ms 38.90, render/busy/ms 37.06, main/busy/ms 35.63, stage/main/update 23.30, stage/render/prepare_assets 20.44, stream/drive/ms 19.59, stage/render/render 12.98, render/graph/ms 10.61, encode//sum 8.57, stage/render/graph_submit 7.79 |
| warm-up | 3.99 | 34.3 | pipelines/compile_ms 189, pipelines/front_ms 182, main/busy/ms 68.34, render/thread/ms 56.24, render/busy/ms 54.28, stage/main/update 53.64, stream/drive/ms 49.54, stage/render/prepare_assets 35.01, stage/render/render 14.35, render/graph/ms 11.70, encode//sum 9.15, stage/render/graph_submit 8.06 |
| warm-up | 4.10 | 108.1 | pipelines/compile_ms 370, pipelines/front_ms 363, render/thread/ms 80.54, render/busy/ms 78.76, main/busy/ms 76.24, stage/main/update 60.11, stage/render/prepare_assets 54.89, stream/drive/ms 49.01, stage/render/render 17.96, render/graph/ms 15.54, stage/render/graph_submit 11.25, encode//sum 8.19 |
| warm-up | 4.16 | 62.2 | pipelines/compile_ms 526, pipelines/front_ms 513, render/thread/ms 111, render/busy/ms 109, main/busy/ms 77.45, stage/main/update 59.67, stage/render/prepare_assets 47.38, stage/render/create_views 33.24, stream/drive/ms 22.12, stage/render/render 18.24, render/graph/ms 15.47, pipelines/driver_ms 13.40 |
| warm-up | 4.22 | 64.7 | pipelines/compile_ms 169, pipelines/front_ms 162, render/thread/ms 107, render/busy/ms 105, main/busy/ms 71.71, stage/main/update 54.19, stage/render/prepare_assets 40.40, stage/render/create_views 33.36, stream/drive/ms 22.11, stage/render/render 16.00, render/graph/ms 13.18, stage/render/graph_submit 10.42 |
| warm-up | 4.25 | 25.8 | pipelines/compile_ms 147, pipelines/front_ms 141, render/thread/ms 72.79, main/busy/ms 71.31, render/busy/ms 70.74, stage/render/prepare_assets 36.14, stage/main/post_update 32.22, stage/render/render 16.78, stage/main/update 15.11, render/graph/ms 14.18, pipelines/wait_ms 14.13, stage/main/fixed_main_loop 11.84 |
| warm-up | 4.37 | 122.1 | pipelines/compile_ms 1355, pipelines/front_ms 1313, pipelines/wait_ms 324, main/busy/ms 103, render/thread/ms 82.47, render/busy/ms 80.75, stage/render/prepare_assets 42.65, pipelines/driver_ms 42.14, stage/main/post_update 40.79, stage/main/update 36.87, stage/render/render 19.59, render/graph/ms 17.24 |
| warm-up | 4.44 | 73.9 | pipelines/compile_ms 1855, pipelines/front_ms 1801, pipelines/wait_ms 312, render/thread/ms 103, render/busy/ms 101, main/busy/ms 73.44, stage/render/prepare_assets 72.23, pipelines/driver_ms 53.76, stage/main/update 35.13, stage/main/post_update 28.17, stage/render/render 15.60, render/graph/ms 13.51 |
| warm-up | 4.49 | 48.3 | pipelines/compile_ms 1755, pipelines/front_ms 1705, pipelines/wait_ms 173, render/thread/ms 72.43, render/busy/ms 70.91, main/busy/ms 52.15, stage/render/prepare_assets 50.20, pipelines/driver_ms 49.68, stage/main/post_update 24.96, stage/main/update 16.29, stage/render/render 9.78, render/graph/ms 7.86 |
| warm-up | 4.53 | 33.5 | pipelines/compile_ms 1613, pipelines/front_ms 1562, pipelines/wait_ms 325, render/thread/ms 59.51, render/busy/ms 57.69, main/busy/ms 52.58, pipelines/driver_ms 50.52, stage/main/update 25.37, stage/render/prepare_assets 21.16, stage/main/post_update 17.62, stage/render/render 14.28, render/graph/ms 12.09 |
| warm-up | 4.57 | 39.5 | pipelines/compile_ms 711, pipelines/front_ms 686, pipelines/wait_ms 154, render/thread/ms 58.13, render/busy/ms 56.07, main/busy/ms 47.73, pipelines/driver_ms 24.76, stage/main/update 22.55, stage/render/prepare_assets 19.20, stage/main/post_update 16.94, stage/render/render 15.65, render/graph/ms 12.97 |
| warm-up | 4.60 | 31.2 | pipelines/compile_ms 647, pipelines/front_ms 628, main/busy/ms 74.65, stage/main/update 55.43, render/thread/ms 50.38, render/busy/ms 48.48, stream/drive/ms 48.08, stage/render/prepare_assets 25.05, pipelines/driver_ms 18.92, stage/render/render 13.22, stage/main/post_update 12.33, render/graph/ms 10.70 |
| warm-up | 4.65 | 56.6 | pipelines/compile_ms 665, pipelines/front_ms 646, main/busy/ms 108, stage/main/update 93.84, stream/drive/ms 85.93, render/thread/ms 38.92, render/busy/ms 37.26, pipelines/driver_ms 18.75, stage/render/prepare_assets 18.53, stage/render/render 11.41, render/graph/ms 9.36, stage/main/post_update 8.23 |
| warm-up | 4.72 | 67.9 | pipelines/compile_ms 811, pipelines/front_ms 785, main/busy/ms 74.93, stage/main/update 49.16, stream/drive/ms 41.71, render/thread/ms 39.88, render/busy/ms 38.31, pipelines/driver_ms 25.52, stage/main/post_update 18.54, stage/render/prepare_assets 16.79, stage/render/render 13.58, render/graph/ms 11.60 |
| warm-up | 4.76 | 40.3 | pipelines/compile_ms 798, pipelines/front_ms 771, main/busy/ms 96.55, render/thread/ms 58.00, render/busy/ms 56.23, stage/main/update 52.74, pipelines/driver_ms 27.87, stage/main/post_update 26.59, stage/render/render 17.00, render/graph/ms 14.74, stage/render/prepare_assets 13.80, encode//sum 10.20 |
| warm-up | 4.88 | 117.5 | pipelines/compile_ms 1447, pipelines/front_ms 1403, main/busy/ms 110, render/thread/ms 91.05, render/busy/ms 89.41, stage/main/update 64.40, pipelines/wait_ms 50.09, pipelines/driver_ms 43.99, stage/render/prepare_assets 43.06, stage/main/post_update 17.95, stage/main/fixed_main_loop 15.00, stage/render/render 14.14 |
| warm-up | 4.92 | 43.6 | pipelines/compile_ms 1889, pipelines/front_ms 1827, pipelines/wait_ms 93.57, render/thread/ms 87.78, render/busy/ms 86.31, pipelines/driver_ms 61.80, main/busy/ms 60.70, stage/render/prepare_assets 49.54, stage/main/update 23.95, stream/drive/ms 14.19, stage/main/fixed_main_loop 12.85, stage/main/pre_update 11.72 |
| warm-up | 4.97 | 26.0 | pipelines/compile_ms 917, pipelines/front_ms 891, render/thread/ms 41.97, render/busy/ms 40.37, pipelines/wait_ms 38.82, main/busy/ms 34.28, pipelines/driver_ms 25.78, stage/main/update 18.51, stage/render/render 12.09, pipelines/lock_ms 10.89, render/graph/ms 10.10, stage/main/post_update 9.15 |
| warm-up | 5.02 | 26.1 | pipelines/compile_ms 732, pipelines/front_ms 711, pipelines/wait_ms 151, main/busy/ms 39.52, render/thread/ms 33.35, render/busy/ms 31.94, stage/main/update 26.36, pipelines/driver_ms 21.28, stream/drive/ms 17.86, stage/render/render 10.70, render/graph/ms 8.86, stage/render/prepare_assets 8.72 |
| warm-up | 5.07 | 27.6 | render/thread/ms 44.34, pipelines/compile_ms 42.61, render/busy/ms 42.44, pipelines/front_ms 38.98, main/busy/ms 37.53, stage/main/update 19.25, stage/render/render 14.17, render/graph/ms 11.96, stage/main/post_update 11.90, stage/render/prepare_assets 10.12, stage/render/graph_submit 8.57, encode//sum 7.41 |
| warm-up | 5.11 | 35.0 | pipelines/compile_ms 255, pipelines/front_ms 246, main/busy/ms 50.95, render/thread/ms 49.91, render/busy/ms 47.87, stage/main/update 18.44, stage/render/render 16.97, stage/main/post_update 14.27, render/graph/ms 14.24, stage/render/prepare_assets 13.93, stage/render/graph_submit 9.59, stage/main/fixed_main_loop 9.23 |
| warm-up | 5.16 | 49.5 | pipelines/compile_ms 669, pipelines/front_ms 649, render/thread/ms 53.98, render/busy/ms 52.23, main/busy/ms 45.30, stage/render/prepare_assets 25.25, pipelines/driver_ms 19.89, stage/main/update 13.79, stage/render/render 13.46, stage/main/post_update 13.33, render/graph/ms 11.09, stage/main/fixed_main_loop 9.29 |
| warm-up | 5.22 | 59.9 | pipelines/compile_ms 1157, pipelines/front_ms 1123, render/thread/ms 70.42, render/busy/ms 69.02, main/busy/ms 52.32, pipelines/driver_ms 33.84, stage/render/prepare_assets 33.71, stage/main/post_update 19.30, pipelines/wait_ms 18.17, stage/render/prepare_resources 16.03, stage/main/update 13.63, stage/render/render 9.85 |
| warm-up | 5.27 | 50.9 | pipelines/compile_ms 1216, pipelines/front_ms 1184, render/thread/ms 84.97, render/busy/ms 83.61, main/busy/ms 75.05, stage/main/post_update 42.28, stage/render/prepare_assets 34.75, pipelines/driver_ms 32.51, pipelines/wait_ms 19.23, stage/render/prepare_resources 16.99, stage/main/update 13.04, stage/render/render 10.63 |
| warm-up | 5.30 | 36.1 | pipelines/compile_ms 747, pipelines/front_ms 708, render/thread/ms 69.95, render/busy/ms 68.44, main/busy/ms 53.06, pipelines/driver_ms 38.71, stage/main/post_update 36.78, stage/render/prepare_assets 33.13, stage/render/render 11.11, stage/main/update 9.97, stage/render/specialize 9.38, render/graph/ms 8.95 |
| warm-up | 5.36 | 37.9 | pipelines/compile_ms 600, pipelines/front_ms 586, render/thread/ms 47.48, render/busy/ms 46.02, main/busy/ms 41.45, stage/main/update 20.46, stage/render/prepare_assets 16.91, pipelines/driver_ms 14.51, stage/render/render 14.00, stage/main/post_update 13.41, stream/drive/ms 12.70, render/graph/ms 11.97 |
| warm-up | 5.44 | 27.3 | pipelines/compile_ms 399, pipelines/front_ms 389, render/thread/ms 46.36, render/busy/ms 44.95, main/busy/ms 30.71, stage/render/prepare_assets 19.38, stage/main/update 12.54, stage/main/post_update 11.75, stage/render/render 10.94, pipelines/driver_ms 10.07, render/graph/ms 9.24, encode//sum 8.03 |
| warm-up | 5.50 | 40.0 | pipelines/compile_ms 609, pipelines/front_ms 595, render/thread/ms 51.39, render/busy/ms 49.98, main/busy/ms 34.66, stage/render/prepare_assets 28.14, pipelines/wait_ms 27.10, stage/main/update 17.60, stage/render/render 14.39, pipelines/driver_ms 13.86, render/graph/ms 12.34, stage/main/post_update 10.58 |
| warm-up | 5.53 | 29.9 | pipelines/compile_ms 505, pipelines/front_ms 489, render/thread/ms 60.59, render/busy/ms 58.90, main/busy/ms 33.85, stage/render/prepare_assets 31.47, pipelines/driver_ms 16.08, stage/render/render 14.57, stage/main/update 13.95, stage/main/post_update 13.50, render/graph/ms 12.48, stage/render/graph_submit 9.68 |
| warm-up | 5.59 | 40.1 | render/thread/ms 53.78, render/busy/ms 51.67, main/busy/ms 27.70, stage/render/prepare_assets 23.11, stage/render/render 14.40, stage/main/update 11.74, render/graph/ms 11.64, stage/main/post_update 9.59, encode//sum 9.44, stage/render/graph_submit 8.29, encode/submit 5.80, stage/render/specialize 3.99 |
| warm-up | 5.61 | 28.3 | render/thread/ms 62.78, render/busy/ms 60.58, main/busy/ms 33.84, stage/render/prepare_assets 29.44, pipelines/compile_ms 17.20, pipelines/front_ms 15.88, stage/render/render 14.26, stage/main/update 13.62, render/graph/ms 11.40, encode//sum 10.26, stage/main/post_update 10.22, stage/render/graph_submit 8.13 |
| warm-up | 5.66 | 50.6 | render/thread/ms 62.48, render/busy/ms 60.94, stage/render/prepare_assets 38.27, main/busy/ms 36.75, pipelines/compile_ms 17.20, pipelines/front_ms 15.88, stage/main/update 15.20, stage/main/post_update 11.29, stage/render/render 10.39, render/graph/ms 8.38, encode//sum 7.61, stage/render/graph_submit 5.89 |
| warm-up | 5.72 | 31.1 | pipelines/compile_ms 926, pipelines/front_ms 905, render/thread/ms 46.41, render/busy/ms 45.28, main/busy/ms 33.73, stage/render/prepare_assets 29.88, pipelines/driver_ms 20.53, stage/main/update 15.25, stage/main/post_update 12.39, stage/render/render 8.06, render/graph/ms 6.41, encode//sum 6.37 |
| warm-up | 5.75 | 32.9 | main/busy/ms 63.03, render/thread/ms 52.32, render/busy/ms 49.22, stage/main/post_update 34.06, stage/main/update 22.90, stage/render/prepare_assets 21.94, stage/render/render 17.92, render/graph/ms 14.02, stage/render/graph_submit 10.67, pipelines/compile_ms 7.80, pipelines/front_ms 7.41, encode//sum 7.36 |
| warm-up | 5.80 | 47.2 | pipelines/compile_ms 220, pipelines/front_ms 211, main/busy/ms 59.64, render/thread/ms 49.46, render/busy/ms 46.28, stage/main/post_update 33.76, stage/main/update 19.21, stage/render/render 18.42, stage/render/prepare_assets 17.25, pipelines/wait_ms 17.14, pipelines/lock_ms 16.56, render/graph/ms 14.50 |
| warm-up | 5.92 | 74.5 | pipelines/compile_ms 178, pipelines/front_ms 175, main/busy/ms 48.28, render/thread/ms 30.38, render/busy/ms 29.61, stage/main/update 27.90, stream/drive/ms 19.02, stage/main/post_update 14.12, stage/render/prepare_assets 13.19, stage/render/render 8.07, render/graph/ms 6.82, stage/render/graph_submit 4.81 |
| warm-up | 5.95 | 32.0 | pipelines/compile_ms 1198, pipelines/front_ms 1164, render/thread/ms 54.88, render/busy/ms 53.06, main/busy/ms 45.37, pipelines/driver_ms 34.18, stage/main/update 23.98, stage/render/prepare_assets 20.46, stage/render/render 17.03, render/graph/ms 14.65, stage/main/post_update 12.63, encode//sum 10.92 |
| warm-up | 5.99 | 38.3 | pipelines/compile_ms 1225, pipelines/front_ms 1191, render/thread/ms 49.35, render/busy/ms 47.72, main/busy/ms 43.24, pipelines/driver_ms 34.69, stage/render/prepare_assets 24.15, stage/main/update 24.14, pipelines/wait_ms 15.26, stage/render/render 13.54, render/graph/ms 11.62, stage/main/post_update 10.95 |
| warm-up | 6.12 | 27.3 | pipelines/compile_ms 71.14, pipelines/front_ms 68.04, main/busy/ms 51.02, render/thread/ms 38.35, render/busy/ms 35.64, stage/main/update 31.33, stream/drive/ms 21.75, stage/render/render 16.71, stage/main/post_update 13.91, render/graph/ms 13.60, stage/render/graph_submit 9.60, encode//sum 7.77 |
| warm-up | 6.16 | 37.4 | pipelines/compile_ms 260, pipelines/front_ms 251, main/busy/ms 53.76, render/thread/ms 42.68, render/busy/ms 40.36, stage/main/update 32.76, stream/drive/ms 22.08, stage/render/render 14.73, stage/main/post_update 14.07, render/graph/ms 11.90, stage/render/prepare_assets 10.58, pipelines/driver_ms 9.74 |
| warm-up | 6.26 | 27.8 | pipelines/compile_ms 379, pipelines/front_ms 369, main/busy/ms 36.70, render/thread/ms 32.15, render/busy/ms 30.90, stage/main/update 19.07, stage/render/render 12.18, stage/render/prepare_assets 12.06, stage/main/post_update 11.52, render/graph/ms 10.43, pipelines/driver_ms 10.19, encode//sum 8.34 |
| warm-up | 6.50 | 27.9 | main/busy/ms 34.21, render/thread/ms 33.65, render/busy/ms 32.17, stage/render/render 16.74, stage/main/post_update 15.59, render/graph/ms 14.38, stage/main/update 13.98, encode//sum 13.46, stage/render/graph_submit 9.82, stage/render/prepare_assets 7.72, encode/submit 5.17, encode/finish_wall 4.66 |
| warm-up | 6.53 | 30.6 | render/thread/ms 45.15, main/busy/ms 44.88, render/busy/ms 43.43, stage/main/update 20.97, stage/main/post_update 17.05, stage/render/render 16.41, stage/render/prepare_assets 15.92, render/graph/ms 13.91, stream/drive/ms 12.23, encode//sum 11.25, stage/render/graph_submit 9.35, encode/submit 4.97 |
| warm-up | 6.58 | 43.9 | render/thread/ms 52.00, render/busy/ms 50.68, main/busy/ms 46.43, stage/render/prepare_assets 29.96, stage/main/update 22.26, stage/main/post_update 16.89, stream/drive/ms 12.35, stage/render/render 11.67, render/graph/ms 9.74, encode//sum 8.25, stage/render/graph_submit 6.51, encode/submit 3.67 |
| warm-up | 6.92 | 31.2 | render/thread/ms 38.02, render/busy/ms 37.20, main/busy/ms 14.79, stage/render/specialize 9.21, stage/render/render 9.19, render/graph/ms 8.05, stage/main/post_update 8.03, encode//sum 7.24, stage/render/queue 6.90, stage/render/graph_submit 5.99, stage/main/update 4.47, stage/render/prepare_meshes 3.62 |
| warm-up | 8.13 | 29.7 | render/thread/ms 36.87, render/busy/ms 33.88, main/busy/ms 14.45, stage/render/render 10.36, render/graph/ms 9.08, stage/render/specialize 8.78, stage/main/post_update 8.17, stage/render/graph_submit 6.94, encode//sum 6.42, stage/render/queue 6.11, encode/submit 3.80, stage/render/prepare_meshes 3.80 |
| measure | 0.82 | 27.2 | render/thread/ms 34.16, render/busy/ms 31.89, main/busy/ms 14.10, stage/render/render 10.42, render/graph/ms 9.10, stage/render/specialize 8.67, encode//sum 8.24, stage/main/post_update 7.56, stage/render/graph_submit 7.28, stage/render/queue 5.88, stage/main/update 4.33, encode/submit 4.22 |
| measure | 5.87 | 27.4 | render/thread/ms 36.38, render/busy/ms 31.71, stage/render/render 27.70, encode//sum 27.36, render/graph/ms 26.38, stage/render/graph_submit 24.44, encode/finish_wall 19.39, encode/pass:early prepass/sum 18.16, main/busy/ms 13.23, stage/main/post_update 7.91, encode/submit 5.11, stage/render/prepare_views 3.85 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set render.skin.skip_unchanged=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=skin-5 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 2%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 85%) · 2625 MHz (min 2625) · memory 8101 MHz · 96.9 W · 77 °C (max 79) · VRAM 6426 MiB · throttled 100% of samples

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
- `render.skin.skip_unchanged=on (command line: --set)`
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
