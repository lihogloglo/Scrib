# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.skin.skip_unchanged=on (command line: --set); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1525 frames over 15.0 s · **median 9.91 ms (101 fps)** · mean 9.84 · p95 11.18 · p99 11.71 (1% low 85 fps) · worst 27.37 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 279 frames over 9.0 s, worst 2701.7 ms
- hitches (> 25.0 ms): **1** measured, 59 in warm-up
- timed GPU passes: 8.60 ms of a 9.84 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.91 ms, limit 11 ms
- pass worst frame 27.37 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.943 | 2.947 | 2.991 | 1.00 | 0.061 |
| dlss_super_resolution | 2.017 | 2.020 | 2.044 | 1.00 | 0.184 |
| early prepass | 0.617 | 0.618 | 0.629 | 1.00 | 0.094 |
| bloom | 0.314 | 0.314 | 0.331 | 1.00 | 0.060 |
| atmosphere_luts | 0.313 | 0.313 | 0.387 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.312 | 0.312 | 0.334 | 1.00 | 0.009 |
| water_velocity | 0.239 | 0.240 | 0.264 | 1.00 | 0.002 |
| water_sspr | 0.216 | 0.217 | 0.215 | 1.00 | 0.002 |
| ssao | 0.192 | 0.192 | 0.198 | 1.00 | 0.007 |
| clouds_trace | 0.148 | 0.148 | 0.168 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.146 | 0.152 | 1.00 | 0.001 |
| water_fft | 0.131 | 0.131 | 0.135 | 1.00 | 0.003 |
| uniform_allocation | 0.120 | 0.120 | 0.159 | 1.00 | 0.021 |
| upscaling | 0.112 | 0.112 | 0.118 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.091 | 0.091 | 0.122 | 1.00 | 0.015 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.413 | 0.416 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| water_snapshot | 0.033 | 0.033 | 0.035 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.028 | 0.222 | 0.232 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 1.00 | 0.085 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.045 | 0.047 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 1.00 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.016 | 0.016 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.907 | 10.275 | 26.04 | 13582.4 |
| main/busy/ms | 7.147 | 7.975 | 9.30 | 10898.6 |
| render/busy/ms | 6.846 | 7.795 | 25.44 | 10440.0 |
| stage/render/render | 4.564 | 5.384 | 8.46 | 6960.2 |
| stage/main/post_update | 4.257 | 4.989 | 6.50 | 6491.3 |
| render/graph/ms | 3.988 | 4.754 | 6.76 | 6082.4 |
| encode//sum | 3.769 | 4.966 | 14.05 | 5747.5 |
| stage/render/graph_submit | 3.001 | 3.757 | 5.81 | 4576.2 |
| encode/submit | 1.875 | 2.288 | 5.23 | 2860.1 |
| stage/main/update | 1.780 | 2.233 | 3.17 | 2715.1 |
| stage/render/prepare_views | 1.750 | 3.382 | 5.00 | 2668.6 |
| render/acquire/ms | 1.705 | 3.336 | 4.95 | 2599.9 |
| encode/finish_wall | 1.145 | 1.598 | 3.48 | 1745.5 |
| stage/render/camera_driver | 0.932 | 1.068 | 1.32 | 1422.0 |
| encode/pass:shadow_directional_light/sum | 0.737 | 1.274 | 2.76 | 1124.1 |
| encode/pass:early prepass/sum | 0.702 | 0.975 | 2.62 | 1070.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.689 | 0.969 | 1.21 | 1050.4 |
| stage/render/prepare_write_phase_buffers | 0.540 | 0.749 | 0.96 | 824.1 |
| stage/render/prepare_assets | 0.445 | 0.564 | 1.98 | 679.2 |
| stage/render/prepare_resources | 0.373 | 0.431 | 0.75 | 569.0 |
| bigspace/total/ms | 0.365 | 0.703 | 1.71 | 557.0 |
| render/present/ms | 0.356 | 0.423 | 1.83 | 542.6 |
| stage/main/pre_update | 0.336 | 0.416 | 0.84 | 512.6 |
| stage/main/first | 0.292 | 0.358 | 1.28 | 445.8 |
| terrain/update/ms | 0.234 | 0.295 | 0.72 | 357.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.232 | 0.333 | 0.56 | 354.5 |
| stage/render/prepare_bind_groups | 0.206 | 0.278 | 0.43 | 314.5 |
| terrain/select/ms | 0.205 | 0.258 | 0.68 | 312.4 |
| stage/render/prepare_meshes | 0.205 | 0.248 | 2.42 | 312.1 |
| stage/main/fixed_main_loop | 0.181 | 0.328 | 0.92 | 275.8 |
| stream/drive/ms | 0.180 | 0.230 | 0.69 | 274.1 |
| stage/render/prepare_batch_phases | 0.179 | 0.242 | 0.39 | 273.0 |
| stage/render/extract_commands | 0.169 | 0.199 | 0.30 | 257.9 |
| stage/render/create_views | 0.165 | 0.211 | 0.58 | 251.0 |
| far/select/ms | 0.159 | 0.195 | 0.66 | 242.5 |
| stream/tick/ms | 0.153 | 0.197 | 0.67 | 232.7 |
| controller/feed/ms | 0.152 | 0.195 | 0.55 | 232.4 |
| controller/move/ms | 0.150 | 0.285 | 0.67 | 228.7 |
| stage/render/prepare_flush | 0.148 | 0.250 | 0.66 | 226.0 |
| bigspace/high_precision/ms | 0.129 | 0.317 | 1.35 | 196.5 |
| stage/main/last | 0.120 | 0.164 | 0.70 | 182.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.114 | 0.176 | 0.24 | 173.7 |
| stage/render/queue | 0.108 | 0.137 | 5.59 | 164.5 |
| bigspace/low_precision/ms | 0.092 | 0.221 | 1.03 | 139.9 |
| stage/render/specialize | 0.085 | 0.104 | 8.43 | 129.3 |
| anim/evaluate/ms | 0.084 | 0.188 | 1.23 | 128.4 |
| anim/evaluate_cpu/ms | 0.063 | 0.078 | 0.57 | 96.1 |
| encode/resolve_diagnostics/sum | 0.058 | 0.091 | 0.35 | 88.1 |
| stage/render/prepare_collect_phase_buffers | 0.056 | 0.110 | 0.22 | 84.7 |
| physics/step/ms | 0.028 | 0.056 | 0.18 | 42.3 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.041 | 0.11 | 40.6 |
| weather/map/ms | 0.025 | 0.032 | 0.41 | 37.4 |
| encode/camera_driver_span_end/sum | 0.023 | 0.038 | 0.33 | 35.8 |
| stage/render/cleanup | 0.023 | 0.033 | 0.20 | 34.7 |
| stage/render/phase_sort | 0.017 | 0.030 | 0.07 | 26.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.04 | 16.8 |
| script/update/ms | 0.007 | 0.008 | 0.33 | 10.0 |
| controller/avoid/ms | 0.006 | 0.008 | 0.04 | 9.1 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.9 |
| anim/apply/ms | 0.004 | 0.006 | 0.04 | 6.6 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 5.8 |
| controller/write/ms | 0.002 | 0.002 | 0.03 | 2.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.06 | 1.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
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
| crowd/joints_extracted/count | count | 115 | 181 | 147 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.52 | 3.34 | 0.77 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.11 | 0.04 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.33 | 0.04 |
| encode/finish_critical | ms | 0.83 | 3.34 | 0.77 |
| encode/pass:early prepass/max | ms | 0.70 | 2.62 | 0.72 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.70 | 1.21 | 0.73 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.24 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.56 | 0.24 |
| encode/pass:shadow_directional_light/max | ms | 0.68 | 2.76 | 0.67 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.35 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.25 | 25.44 | 9.24 |
| gpu/timed/ms | ms | 9.25 | 10.89 | 9.24 |
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
| process/cpu_usage | % | 25.96 | 37.50 | 20.19 |
| process/mem_usage | GiB | 4.63 | 4.69 | 4.69 |
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
| skin/blocks_changed/count | count | 5 | 7 | 6 |
| skin/blocks_skipped/count | count | 176 | 177 | 175 |
| skin/blocks_walked/count | count | 5 | 7 | 6 |
| skin/joint_bytes_written/bytes | bytes | 9216 | 12.5 k | 10.2 k |
| skin/joints_written/count | count | 144 | 196 | 160 |
| skin/upload_writes/count | count | 5 | 5 | 5 |
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
| system/cpu_usage | % | 31.85 | 36.36 | 28.82 |
| system/mem_usage | % | 60.21 | 60.37 | 60.37 |
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
| anim/events/count | count | 0.11 | 2 | 164 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.25 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| encode//count | count | 26.57 | 29 | 40.5 k |
| encode/bind_walk/init_textures | count | 7165 | 14.0 k | 10.93 M |
| encode/bind_walk/init_textures_skipped | count | 35.6 k | 48.7 k | 54.29 M |
| encode/bind_walk/inits | count | 345 | 437 | 526.7 k |
| encode/bind_walk/inits_skipped | count | 67.00 | 79 | 102.2 k |
| encode/bind_walk/merge_textures | count | 7101 | 13.9 k | 10.83 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.22 M |
| encode/bind_walk/merges | count | 82.26 | 103 | 125.4 k |
| encode/bind_walk/merges_skipped | count | 50.75 | 56 | 77.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1525 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1525 |
| encode/pass:early prepass/count | count | 1 | 1 | 1525 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1525 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1525 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1525 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1525 |
| encode/queue/command_buffers | count | 36.57 | 39 | 55.8 k |
| encode/queue/submits | count | 2 | 2 | 3050 |
| encode/queue/write_buffer/bytes | bytes | 1.14 M | 3.35 M | 1.74 G |
| encode/queue/write_buffer/calls | count | 221 | 291 | 336.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 49.78 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.7 k |
| encode/queue/write_texture/bytes | bytes | 11.8 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1525 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.59 | 2 | 900 |
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
| warm-up | 2.70 | 2701.7 | main/busy/ms 2671, stage/main/post_update 2563, stage/main/update 98.14, surface/advance_cpu_ms 68.61, stage/main/pre_update 4.79, stage/main/last 1.18, surface/cell_upload_ms 0.78, stage/main/first 0.76, stream/drive/ms 0.55, stage/main/fixed_main_loop 0.40, weather/map/ms 0.16, stream/tick/ms 0.13 |
| warm-up | 3.65 | 943.8 | main/busy/ms 2858, stage/main/post_update 2590, render/thread/ms 912, render/busy/ms 727, stage/render/prepare_assets 678, stage/main/update 252, stage/render/prepare_views 181, render/acquire/ms 181, surface/advance_cpu_ms 68.61, stage/render/create_views 28.61, stage/render/render 12.89, render/graph/ms 8.19 |
| warm-up | 3.75 | 107.0 | render/thread/ms 1001, render/busy/ms 758, stage/render/prepare_assets 697, main/busy/ms 230, stage/render/prepare_views 181, render/acquire/ms 181, stage/main/update 158, stage/render/render 77.14, render/present/ms 61.29, pipelines/compile_ms 48.22, pipelines/front_ms 45.08, stage/main/post_update 41.48 |
| warm-up | 3.84 | 83.1 | pipelines/wait_ms 1151, pipelines/compile_ms 660, pipelines/front_ms 613, render/thread/ms 170, stage/render/render 139, main/busy/ms 117, render/present/ms 113, stage/main/post_update 81.87, render/busy/ms 56.77, pipelines/driver_ms 47.23, terrain/update/ms 26.79, render/graph/ms 24.93 |
| warm-up | 3.90 | 66.7 | pipelines/wait_ms 3156, pipelines/compile_ms 890, pipelines/front_ms 831, main/busy/ms 150, render/thread/ms 141, stage/main/post_update 84.03, render/busy/ms 83.62, stage/render/render 80.62, pipelines/driver_ms 59.78, render/present/ms 56.13, stage/main/update 49.23, render/graph/ms 23.33 |
| warm-up | 4.01 | 103.0 | pipelines/wait_ms 4242, pipelines/compile_ms 1979, pipelines/front_ms 1870, render/thread/ms 141, render/busy/ms 139, main/busy/ms 121, pipelines/driver_ms 109, stage/main/update 75.11, stage/render/prepare_assets 73.34, stage/main/post_update 29.92, stage/render/prepare_bind_groups 22.20, terrain/update/ms 17.52 |
| warm-up | 4.05 | 40.5 | pipelines/wait_ms 2363, pipelines/compile_ms 2196, pipelines/front_ms 2080, pipelines/driver_ms 116, render/thread/ms 106, render/busy/ms 104, stage/render/prepare_assets 75.10, main/busy/ms 61.31, stage/main/update 36.77, stage/main/post_update 18.48, terrain/update/ms 15.26, stage/render/render 14.47 |
| warm-up | 4.09 | 41.2 | pipelines/compile_ms 846, pipelines/front_ms 797, pipelines/wait_ms 208, render/thread/ms 62.85, render/busy/ms 57.55, pipelines/driver_ms 48.61, stage/render/prepare_assets 34.80, main/busy/ms 33.94, stage/main/post_update 19.10, stage/render/render 13.33, terrain/update/ms 10.11, stage/main/update 9.83 |
| warm-up | 4.13 | 40.6 | pipelines/compile_ms 622, pipelines/front_ms 586, pipelines/wait_ms 95.32, render/thread/ms 77.03, render/busy/ms 70.79, stage/render/prepare_assets 35.91, pipelines/driver_ms 35.42, main/busy/ms 29.20, stage/render/render 19.26, stage/main/post_update 19.18, render/graph/ms 12.65, stage/render/prepare_bind_groups 12.63 |
| warm-up | 4.16 | 30.1 | pipelines/compile_ms 491, pipelines/front_ms 476, render/thread/ms 65.51, render/busy/ms 62.89, pipelines/wait_ms 50.50, main/busy/ms 39.18, stage/render/prepare_assets 16.26, pipelines/driver_ms 15.73, stage/main/update 15.59, stage/render/render 15.16, stage/main/post_update 14.94, stage/render/prepare_bind_groups 12.52 |
| warm-up | 4.21 | 49.1 | pipelines/compile_ms 1274, pipelines/front_ms 1225, pipelines/wait_ms 77.84, render/thread/ms 71.95, render/busy/ms 70.08, pipelines/driver_ms 48.97, main/busy/ms 42.74, stage/render/prepare_assets 38.72, stage/main/update 17.51, stage/main/post_update 15.99, stage/render/render 11.69, terrain/update/ms 9.18 |
| warm-up | 4.23 | 25.0 | pipelines/compile_ms 1278, pipelines/front_ms 1226, render/thread/ms 68.33, render/busy/ms 66.17, main/busy/ms 63.03, pipelines/driver_ms 51.30, stage/main/update 48.50, pipelines/wait_ms 42.53, stream/drive/ms 42.40, stage/render/prepare_assets 37.97, encode//sum 36.80, stage/render/render 18.63 |
| warm-up | 4.30 | 65.7 | pipelines/compile_ms 1068, pipelines/front_ms 1032, main/busy/ms 56.86, render/thread/ms 56.15, render/busy/ms 54.34, stage/main/update 46.57, stream/drive/ms 42.28, encode//sum 36.35, pipelines/driver_ms 35.78, stage/render/prepare_assets 29.67, stage/render/render 16.17, render/graph/ms 14.17 |
| warm-up | 4.35 | 42.2 | render/thread/ms 36.04, render/busy/ms 34.35, main/busy/ms 31.66, stage/main/update 20.11, stage/render/prepare_assets 18.01, stream/drive/ms 16.23, stage/render/render 12.27, render/graph/ms 10.10, encode//sum 8.88, stage/render/graph_submit 7.43, stage/main/post_update 6.88, encode/submit 4.83 |
| warm-up | 4.39 | 38.8 | render/thread/ms 58.62, render/busy/ms 56.60, main/busy/ms 43.44, stage/render/prepare_assets 38.10, stage/main/update 27.88, stream/drive/ms 21.80, stage/render/render 13.79, render/graph/ms 11.05, stage/main/post_update 8.70, encode//sum 8.04, stage/render/graph_submit 7.65, encode/submit 4.94 |
| warm-up | 4.49 | 104.7 | pipelines/compile_ms 416, pipelines/front_ms 398, render/thread/ms 106, render/busy/ms 102, stage/render/prepare_assets 60.36, main/busy/ms 54.77, stage/main/update 35.37, stage/render/render 35.21, render/graph/ms 31.19, stage/render/graph_submit 27.39, stream/drive/ms 23.35, encode//sum 20.14 |
| warm-up | 4.56 | 67.2 | pipelines/compile_ms 564, pipelines/front_ms 540, render/thread/ms 116, render/busy/ms 113, main/busy/ms 108, stage/main/update 88.74, stage/render/prepare_assets 67.00, stream/drive/ms 58.11, stage/render/render 34.91, render/graph/ms 30.82, pipelines/wait_ms 29.17, pipelines/lock_ms 28.21 |
| warm-up | 4.73 | 171.0 | pipelines/compile_ms 1034, pipelines/front_ms 1002, render/thread/ms 134, render/busy/ms 133, main/busy/ms 111, stage/render/prepare_assets 110, stage/main/update 93.45, stream/drive/ms 57.79, pipelines/wait_ms 41.78, pipelines/lock_ms 40.61, pipelines/driver_ms 31.60, stage/render/render 12.90 |
| warm-up | 4.77 | 37.3 | pipelines/compile_ms 964, pipelines/front_ms 935, render/thread/ms 120, render/busy/ms 118, stage/render/prepare_assets 94.63, main/busy/ms 54.50, stage/main/update 30.14, pipelines/driver_ms 28.85, pipelines/wait_ms 16.96, pipelines/lock_ms 16.34, stage/main/post_update 14.06, stage/render/render 12.00 |
| warm-up | 4.84 | 65.9 | pipelines/compile_ms 234, pipelines/front_ms 215, pipelines/wait_ms 100, render/thread/ms 90.44, render/busy/ms 88.28, main/busy/ms 85.99, stage/main/update 53.97, stage/render/prepare_assets 29.56, stage/render/prepare_flush 22.18, pipelines/driver_ms 18.42, stage/main/post_update 15.31, pipelines/lock_ms 11.16 |
| warm-up | 4.90 | 60.4 | pipelines/compile_ms 211, pipelines/wait_ms 201, pipelines/front_ms 194, main/busy/ms 119, render/thread/ms 111, render/busy/ms 110, stage/main/update 52.51, stage/render/prepare_flush 41.31, stage/main/post_update 36.87, stage/render/prepare_assets 31.13, pipelines/driver_ms 17.44, stage/main/pre_update 14.79 |
| warm-up | 4.96 | 59.2 | pipelines/compile_ms 1383, pipelines/front_ms 1341, pipelines/wait_ms 463, render/thread/ms 88.27, render/busy/ms 86.49, main/busy/ms 75.80, pipelines/driver_ms 42.65, stage/main/post_update 38.64, stage/render/prepare_assets 30.28, stage/render/prepare_flush 19.51, stage/main/update 15.88, stage/main/pre_update 14.54 |
| warm-up | 4.99 | 31.7 | pipelines/compile_ms 1840, pipelines/front_ms 1779, pipelines/wait_ms 647, render/thread/ms 69.16, render/busy/ms 67.41, pipelines/driver_ms 60.99, main/busy/ms 36.77, stage/render/prepare_assets 18.93, stage/main/update 15.99, stage/main/post_update 15.70, stage/render/prepare_resources 13.97, stage/render/render 12.33 |
| warm-up | 5.07 | 60.6 | pipelines/compile_ms 1071, pipelines/front_ms 1042, pipelines/wait_ms 144, render/thread/ms 72.97, main/busy/ms 72.86, stage/main/update 59.60, stream/drive/ms 53.77, stage/render/render 52.71, render/present/ms 42.88, render/busy/ms 29.86, pipelines/driver_ms 28.70, stage/render/prepare_assets 11.45 |
| warm-up | 5.12 | 47.3 | pipelines/compile_ms 440, pipelines/front_ms 430, main/busy/ms 141, stage/main/update 121, stream/drive/ms 108, pipelines/wait_ms 90.93, render/thread/ms 79.53, stage/render/render 58.42, render/present/ms 43.84, render/busy/ms 35.43, render/graph/ms 13.72, stage/main/post_update 13.50 |
| warm-up | 5.19 | 73.1 | pipelines/compile_ms 1154, pipelines/front_ms 1119, main/busy/ms 92.44, stage/main/update 70.91, stream/drive/ms 57.81, pipelines/wait_ms 53.96, render/thread/ms 46.72, render/busy/ms 44.39, pipelines/driver_ms 34.81, stage/render/prepare_assets 23.37, stage/main/post_update 15.41, stage/render/render 14.73 |
| warm-up | 5.23 | 39.1 | pipelines/compile_ms 1329, pipelines/front_ms 1289, render/thread/ms 59.80, render/busy/ms 58.51, pipelines/wait_ms 53.97, pipelines/driver_ms 40.22, main/busy/ms 39.70, stage/render/prepare_assets 21.07, stage/main/post_update 16.30, stage/render/prepare_write_phase_buffers 15.62, stage/main/update 10.37, stage/render/render 9.64 |
| warm-up | 5.30 | 70.6 | pipelines/compile_ms 822, pipelines/front_ms 794, render/thread/ms 102, render/busy/ms 101, main/busy/ms 79.61, stage/render/prepare_resources 46.51, stage/main/update 40.41, pipelines/driver_ms 27.89, stage/main/post_update 17.10, stage/render/prepare_write_phase_buffers 15.67, stage/main/pre_update 13.16, stage/render/prepare_assets 12.27 |
| warm-up | 5.38 | 72.5 | pipelines/compile_ms 1799, pipelines/front_ms 1743, pipelines/wait_ms 169, render/thread/ms 115, render/busy/ms 114, main/busy/ms 80.05, pipelines/driver_ms 55.96, stage/main/update 53.57, stage/render/prepare_resources 44.02, stage/render/prepare_assets 38.38, stage/render/render 11.92, stream/drive/ms 11.58 |
| warm-up | 5.41 | 30.6 | pipelines/compile_ms 2069, pipelines/front_ms 2009, pipelines/wait_ms 228, render/thread/ms 75.10, render/busy/ms 73.41, pipelines/driver_ms 59.96, main/busy/ms 45.54, stage/render/prepare_assets 37.42, stage/main/update 26.41, stage/render/render 12.97, stream/drive/ms 12.60, render/graph/ms 10.82 |
| warm-up | 5.45 | 26.6 | pipelines/compile_ms 309, pipelines/front_ms 298, render/thread/ms 40.67, render/busy/ms 38.82, main/busy/ms 37.36, stage/main/update 19.95, stage/render/render 12.53, pipelines/wait_ms 12.04, stage/main/post_update 11.27, pipelines/driver_ms 10.71, render/graph/ms 10.35, stage/render/graph_submit 7.73 |
| warm-up | 5.50 | 28.9 | pipelines/compile_ms 304, pipelines/front_ms 294, render/thread/ms 40.73, render/busy/ms 38.95, pipelines/wait_ms 36.78, main/busy/ms 35.24, stage/main/update 23.27, stream/drive/ms 15.31, stage/render/render 12.26, stage/render/prepare_assets 11.60, render/graph/ms 9.99, pipelines/driver_ms 9.39 |
| warm-up | 5.53 | 26.9 | pipelines/compile_ms 579, pipelines/front_ms 553, render/thread/ms 47.85, render/busy/ms 46.26, main/busy/ms 39.45, pipelines/wait_ms 37.74, pipelines/driver_ms 26.39, stage/main/update 21.89, stage/main/post_update 12.11, stage/render/render 11.58, stage/render/prepare_resources 9.91, render/graph/ms 9.50 |
| warm-up | 5.59 | 60.8 | pipelines/compile_ms 513, pipelines/front_ms 487, render/thread/ms 76.37, render/busy/ms 74.79, main/busy/ms 52.56, stage/render/prepare_assets 27.80, stage/main/update 26.26, pipelines/driver_ms 25.21, stage/main/post_update 19.51, stage/render/render 12.43, stage/render/prepare_resources 12.14, render/graph/ms 10.33 |
| warm-up | 5.62 | 29.5 | pipelines/compile_ms 852, pipelines/front_ms 829, render/thread/ms 75.45, render/busy/ms 74.02, main/busy/ms 57.98, stage/render/prepare_assets 34.35, stage/main/update 25.03, stage/main/post_update 24.01, pipelines/driver_ms 22.86, stream/drive/ms 11.75, stage/render/render 11.59, render/graph/ms 9.56 |
| warm-up | 5.70 | 78.2 | pipelines/compile_ms 1067, pipelines/front_ms 1037, main/busy/ms 100, render/thread/ms 83.39, render/busy/ms 81.89, stage/main/post_update 51.61, stage/render/prepare_assets 30.79, pipelines/driver_ms 29.90, pipelines/wait_ms 26.49, stage/render/prepare_resources 24.89, stage/main/update 20.45, stage/main/fixed_main_loop 14.85 |
| warm-up | 5.74 | 38.3 | pipelines/compile_ms 1775, pipelines/front_ms 1725, main/busy/ms 89.91, render/thread/ms 85.97, render/busy/ms 84.49, stage/main/post_update 50.02, pipelines/driver_ms 49.46, stage/render/prepare_resources 33.04, pipelines/wait_ms 32.63, stage/render/prepare_assets 28.63, stage/main/fixed_main_loop 12.69, stage/main/update 11.82 |
| warm-up | 5.78 | 43.4 | pipelines/compile_ms 1620, pipelines/front_ms 1574, render/thread/ms 58.66, render/busy/ms 57.13, pipelines/driver_ms 46.46, main/busy/ms 38.61, stage/render/prepare_assets 25.13, stage/main/post_update 16.52, stage/main/update 13.55, pipelines/wait_ms 12.48, stage/render/render 12.21, render/graph/ms 9.74 |
| warm-up | 5.82 | 39.1 | pipelines/compile_ms 530, pipelines/front_ms 510, render/thread/ms 61.05, render/busy/ms 59.52, main/busy/ms 41.52, stage/render/prepare_assets 34.37, stage/main/update 23.61, pipelines/driver_ms 20.68, stage/render/render 13.24, stream/drive/ms 12.70, render/graph/ms 10.70, stage/main/post_update 10.50 |
| warm-up | 5.89 | 31.0 | pipelines/compile_ms 415, pipelines/front_ms 404, pipelines/wait_ms 87.01, render/thread/ms 41.07, render/busy/ms 39.70, main/busy/ms 29.16, stage/render/prepare_assets 17.28, stage/main/update 12.25, stage/main/post_update 11.44, stage/render/render 11.34, pipelines/driver_ms 10.90, render/graph/ms 9.44 |
| warm-up | 5.98 | 46.9 | pipelines/compile_ms 215, pipelines/front_ms 211, render/thread/ms 63.77, render/busy/ms 57.68, main/busy/ms 37.62, stage/render/render 32.20, render/graph/ms 25.42, stage/render/graph_submit 20.23, stage/main/update 17.67, stage/render/prepare_assets 17.15, pipelines/wait_ms 17.10, stage/main/post_update 12.83 |
| warm-up | 6.03 | 48.0 | pipelines/compile_ms 185, pipelines/front_ms 177, render/thread/ms 86.93, render/busy/ms 81.01, pipelines/wait_ms 59.44, main/busy/ms 43.47, stage/render/render 31.45, render/graph/ms 24.60, stage/render/prepare_assets 23.33, stage/render/graph_submit 19.50, stage/main/update 16.54, stage/main/post_update 11.11 |
| warm-up | 6.08 | 51.2 | pipelines/compile_ms 368, pipelines/front_ms 353, render/thread/ms 87.79, render/busy/ms 86.39, pipelines/wait_ms 58.30, main/busy/ms 49.96, stage/render/prepare_assets 43.91, stage/main/update 22.75, pipelines/driver_ms 14.82, stage/render/render 12.63, render/graph/ms 10.49, stage/main/fixed_main_loop 10.32 |
| warm-up | 6.13 | 51.2 | pipelines/compile_ms 576, pipelines/front_ms 558, render/thread/ms 93.95, render/busy/ms 89.90, stage/render/prepare_assets 44.37, main/busy/ms 42.88, stage/main/update 21.92, stage/render/prepare_resources 21.42, pipelines/driver_ms 17.62, stage/render/render 14.69, stage/main/post_update 13.08, render/graph/ms 10.03 |
| warm-up | 6.20 | 39.9 | pipelines/compile_ms 1581, pipelines/front_ms 1549, render/thread/ms 49.44, render/busy/ms 48.40, pipelines/wait_ms 46.87, main/busy/ms 32.60, pipelines/driver_ms 31.79, stage/render/prepare_assets 29.83, stage/main/update 14.00, stage/main/post_update 12.38, stage/render/render 9.34, render/graph/ms 7.73 |
| warm-up | 6.29 | 35.1 | pipelines/compile_ms 98.93, pipelines/front_ms 94.83, main/busy/ms 45.83, render/thread/ms 41.30, render/busy/ms 39.44, stage/main/update 20.30, stage/main/post_update 20.20, stage/render/render 15.15, render/graph/ms 12.70, pipelines/wait_ms 12.55, pipelines/lock_ms 12.23, stage/render/prepare_assets 11.36 |
| warm-up | 6.32 | 30.2 | pipelines/compile_ms 98.93, pipelines/front_ms 94.83, render/thread/ms 46.58, render/busy/ms 44.72, main/busy/ms 44.27, stage/main/update 19.18, stage/main/post_update 17.13, stage/render/render 15.21, stage/render/prepare_assets 14.24, render/graph/ms 12.85, pipelines/wait_ms 12.55, pipelines/lock_ms 12.23 |
| warm-up | 6.40 | 57.0 | main/busy/ms 50.90, render/thread/ms 46.14, render/busy/ms 44.67, stage/main/update 27.94, stage/render/prepare_assets 21.57, stream/drive/ms 18.35, stage/main/post_update 15.58, stage/render/render 10.87, render/graph/ms 8.56, pipelines/compile_ms 8.24, pipelines/front_ms 7.77, encode//sum 7.10 |
| warm-up | 6.45 | 45.1 | render/thread/ms 66.07, render/busy/ms 64.30, main/busy/ms 41.76, stage/render/prepare_assets 24.46, pipelines/compile_ms 21.26, stage/render/render 21.00, pipelines/front_ms 19.74, render/graph/ms 18.41, stage/main/update 17.23, stage/main/post_update 16.50, stage/render/graph_submit 13.80, encode//sum 8.55 |
| warm-up | 6.50 | 31.6 | pipelines/compile_ms 1466, pipelines/front_ms 1422, pipelines/driver_ms 44.13, render/thread/ms 38.53, render/busy/ms 37.25, main/busy/ms 27.61, stage/render/prepare_assets 19.37, pipelines/wait_ms 18.82, pipelines/lock_ms 18.24, stage/main/update 11.73, stage/render/render 9.46, stage/main/post_update 9.20 |
| warm-up | 6.62 | 30.1 | pipelines/compile_ms 62.58, pipelines/front_ms 60.09, main/busy/ms 35.83, render/thread/ms 33.89, render/busy/ms 32.59, stage/main/update 19.68, stage/render/render 11.16, stage/main/post_update 10.26, render/graph/ms 9.50, stage/render/prepare_batch_phases 7.77, encode//sum 6.64, stage/render/graph_submit 6.55 |
| warm-up | 6.71 | 26.2 | pipelines/compile_ms 371, pipelines/front_ms 361, render/thread/ms 46.15, render/busy/ms 44.64, main/busy/ms 33.53, stage/render/prepare_assets 25.44, stage/main/update 15.15, stage/main/post_update 12.47, stage/render/render 10.20, pipelines/driver_ms 10.08, render/graph/ms 8.17, stream/drive/ms 8.13 |
| warm-up | 6.75 | 28.1 | pipelines/compile_ms 560, pipelines/front_ms 548, render/thread/ms 41.49, render/busy/ms 40.16, main/busy/ms 32.46, stage/render/prepare_assets 22.92, stage/main/update 14.05, stage/main/post_update 12.72, pipelines/driver_ms 11.62, stage/render/render 10.66, render/graph/ms 8.96, encode//sum 7.22 |
| warm-up | 6.84 | 34.1 | pipelines/compile_ms 161, pipelines/front_ms 157, render/thread/ms 37.84, render/busy/ms 36.44, main/busy/ms 30.55, stage/main/update 19.37, stage/render/prepare_assets 18.93, stream/drive/ms 13.84, stage/render/render 10.48, render/graph/ms 8.60, stage/main/post_update 7.54, encode//sum 7.03 |
| warm-up | 6.91 | 27.2 | main/busy/ms 29.09, render/thread/ms 22.93, render/busy/ms 21.64, stage/main/update 16.77, stage/render/render 9.87, stage/main/post_update 8.36, render/graph/ms 8.18, stage/render/prepare_assets 7.71, encode//sum 6.60, stage/render/graph_submit 5.94, encode/submit 3.63, encode/finish_wall 2.38 |
| warm-up | 7.01 | 25.1 | render/thread/ms 29.44, main/busy/ms 29.23, render/busy/ms 28.12, stage/render/render 15.00, stage/main/post_update 13.42, render/graph/ms 13.02, stage/main/update 11.74, encode//sum 9.98, stage/render/graph_submit 9.17, stage/render/prepare_assets 5.56, encode/submit 5.26, encode/finish_wall 3.91 |
| warm-up | 7.09 | 49.5 | render/thread/ms 50.19, render/busy/ms 48.41, main/busy/ms 43.42, stage/render/prepare_assets 26.50, stage/main/update 22.13, stage/main/post_update 14.58, stage/render/render 14.25, stream/drive/ms 12.55, render/graph/ms 11.97, stage/render/graph_submit 8.87, encode//sum 8.18, encode/submit 5.95 |
| warm-up | 7.41 | 29.5 | render/thread/ms 36.75, render/busy/ms 34.63, main/busy/ms 14.62, stage/render/render 10.70, render/graph/ms 9.39, stage/render/specialize 8.91, stage/main/post_update 7.98, stage/render/graph_submit 7.20, encode//sum 7.10, stage/render/queue 6.31, encode/submit 4.19, stage/main/update 4.15 |
| warm-up | 8.63 | 28.6 | render/thread/ms 35.23, render/busy/ms 32.53, main/busy/ms 14.87, stage/render/render 9.71, render/graph/ms 8.47, stage/render/specialize 8.21, stage/main/post_update 8.06, stage/render/graph_submit 6.54, encode//sum 6.03, stage/render/queue 5.77, stage/render/prepare_meshes 3.95, stage/main/update 3.93 |
| measure | 0.84 | 27.4 | render/thread/ms 34.99, render/busy/ms 32.29, main/busy/ms 14.01, stage/render/render 11.04, render/graph/ms 9.69, stage/render/specialize 8.55, encode//sum 7.90, stage/render/graph_submit 7.79, stage/main/post_update 7.47, stage/render/queue 5.71, encode/submit 4.36, stage/main/update 3.93 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set render.skin.skip_unchanged=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=skin-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 6%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 85%) · 2625 MHz (min 2625) · memory 8101 MHz · 96.5 W · 77 °C (max 79) · VRAM 6426 MiB · throttled 100% of samples

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
