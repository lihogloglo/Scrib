# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6051 frames over 65.6 s · **median 10.40 ms (96 fps)** · mean 10.85 · p95 14.38 · p99 16.25 (1% low 62 fps) · worst 28.09 ms · 113 spikes over 1.5× median · 22.8 stutters/min (work 24.7/min)
- warm-up: 324 frames over 9.3 s, worst 2469.7 ms
- hitches (> 26.0 ms): **6** measured, 45 in warm-up
- timed GPU passes: 6.51 ms of a 10.85 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.40 ms, limit 11 ms
- **FAIL** frame_ms: 25 stutters (> 1.5x moving median of 60) in 1.09 min = 22.85/min, limit 1/min
- pass worst frame 28.09 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.973 | 2.030 | 2.063 | 0.97 | 0.176 |
| main_opaque_pass_3d | 1.262 | 1.299 | 1.649 | 0.97 | 0.068 |
| early prepass | 0.472 | 0.486 | 0.699 | 0.97 | 0.085 |
| bloom | 0.307 | 0.316 | 0.322 | 0.97 | 0.062 |
| atmosphere_luts | 0.301 | 0.309 | 0.372 | 0.97 | 0.003 |
| water_velocity | 0.255 | 0.263 | 0.350 | 0.97 | 0.002 |
| auxiliary_views | 0.220 | 0.227 | 1.094 | 0.97 | 0.215 |
| water_sspr | 0.196 | 0.201 | 0.202 | 0.97 | 0.002 |
| ssao | 0.183 | 0.188 | 0.193 | 0.97 | 0.007 |
| upscaling | 0.126 | 0.130 | 0.137 | 0.97 | 0.004 |
| water_fft | 0.126 | 0.129 | 0.137 | 0.97 | 0.003 |
| atmos_froxel_inject | 0.124 | 0.127 | 0.128 | 0.97 | 0.001 |
| uniform_allocation | 0.121 | 0.124 | 0.154 | 0.97 | 0.026 |
| clouds_composite | 0.102 | 0.105 | 0.108 | 0.97 | 0.002 |
| bin_unpacking | 0.095 | 0.098 | 0.119 | 0.97 | 0.018 |
| tonemapping | 0.084 | 0.087 | 0.091 | 0.97 | 0.003 |
| main_transmissive_pass_3d | 0.076 | 0.078 | 0.154 | 0.97 | 0.009 |
| clustering | 0.062 | 0.063 | 0.067 | 0.97 | 0.019 |
| clouds_trace | 0.054 | 0.055 | 0.073 | 0.97 | 0.001 |
| render_sky | 0.050 | 0.051 | 0.052 | 0.97 | 0.002 |
| clouds_resolve | 0.048 | 0.050 | 0.051 | 0.97 | 0.001 |
| lightprobe_irradiance_map | 0.041 | 0.411 | 0.415 | 0.10 | 0.001 |
| surface_trail | 0.038 | 0.056 | 0.056 | 0.68 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.97 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.028 | 0.037 | 0.97 | 0.005 |
| water_snapshot | 0.024 | 0.025 | 0.028 | 0.97 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.218 | 0.229 | 0.10 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.97 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.036 | 0.97 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.024 | 0.97 | 0.124 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.97 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| per_view_shadow_pass | 0.009 | 0.010 | 0.015 | 0.97 | 0.005 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.97 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.97 | 0.003 |
| lightprobe_copy | 0.001 | 0.012 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.400 | 12.917 | 26.52 | 56877.0 |
| main/busy/ms | 9.040 | 11.013 | 25.55 | 54702.4 |
| render/busy/ms | 8.879 | 12.387 | 25.76 | 53726.5 |
| stage/render/render | 5.637 | 8.417 | 16.94 | 34108.2 |
| stage/main/post_update | 5.616 | 7.179 | 10.36 | 33981.7 |
| render/graph/ms | 4.930 | 7.645 | 15.70 | 29834.3 |
| encode//sum | 4.306 | 6.869 | 38.24 | 26057.2 |
| stage/render/graph_submit | 3.679 | 5.638 | 11.54 | 22262.6 |
| encode/submit | 2.349 | 3.776 | 6.35 | 14216.2 |
| stage/main/update | 2.160 | 2.616 | 18.92 | 13068.4 |
| encode/finish_wall | 1.372 | 1.943 | 6.17 | 8302.8 |
| stage/render/camera_driver | 1.198 | 2.009 | 4.31 | 7250.2 |
| encode/pass:shadow_point_light/sum | 1.159 | 5.154 | 16.74 | 7010.6 |
| encode/pass:shadow_directional_light/sum | 0.776 | 1.254 | 2.13 | 4697.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.762 | 1.074 | 1.94 | 4609.8 |
| encode/pass:early prepass/sum | 0.747 | 1.074 | 1.68 | 4518.0 |
| stage/render/prepare_write_phase_buffers | 0.740 | 1.081 | 2.92 | 4476.7 |
| stage/render/prepare_assets | 0.582 | 0.737 | 9.47 | 3522.7 |
| bigspace/total/ms | 0.575 | 1.404 | 2.82 | 3480.9 |
| stage/render/prepare_resources | 0.459 | 0.551 | 1.14 | 2774.5 |
| render/present/ms | 0.427 | 0.534 | 2.11 | 2582.2 |
| stage/render/prepare_meshes | 0.419 | 0.531 | 1.79 | 2535.6 |
| stage/main/first | 0.397 | 0.522 | 5.49 | 2401.4 |
| stage/main/pre_update | 0.360 | 0.516 | 1.39 | 2177.3 |
| terrain/update/ms | 0.315 | 0.416 | 2.94 | 1908.6 |
| terrain/select/ms | 0.281 | 0.371 | 1.00 | 1698.0 |
| controller/feed/ms | 0.263 | 0.331 | 0.85 | 1591.6 |
| stage/render/specialize | 0.259 | 0.395 | 2.52 | 1567.0 |
| stage/render/prepare_bind_groups | 0.259 | 0.440 | 2.33 | 1566.5 |
| stream/drive/ms | 0.251 | 0.335 | 16.80 | 1517.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.245 | 0.364 | 1.17 | 1484.4 |
| stage/render/queue | 0.244 | 0.386 | 3.10 | 1476.2 |
| bigspace/low_precision/ms | 0.237 | 0.754 | 2.59 | 1436.6 |
| stage/main/fixed_main_loop | 0.217 | 0.369 | 1.70 | 1310.9 |
| far/select/ms | 0.201 | 0.271 | 0.91 | 1218.3 |
| stream/tick/ms | 0.195 | 0.260 | 0.86 | 1178.5 |
| stage/render/create_views | 0.194 | 0.253 | 1.30 | 1173.0 |
| stage/render/prepare_batch_phases | 0.187 | 0.264 | 0.66 | 1130.2 |
| stage/render/extract_commands | 0.174 | 0.204 | 0.44 | 1054.1 |
| bigspace/high_precision/ms | 0.172 | 0.755 | 2.08 | 1042.7 |
| stage/render/prepare_flush | 0.170 | 0.354 | 0.93 | 1030.2 |
| anim/evaluate/ms | 0.166 | 0.290 | 1.64 | 1006.6 |
| anim/evaluate_cpu/ms | 0.154 | 0.190 | 0.79 | 929.1 |
| stage/render/prepare_views | 0.138 | 0.172 | 0.47 | 833.0 |
| pipelines/compile_ms | 0.133 | 0.000 | 135.41 | 806.7 |
| controller/move/ms | 0.132 | 0.268 | 0.57 | 801.1 |
| pipelines/front_ms | 0.130 | 0.000 | 132.58 | 787.4 |
| stage/main/last | 0.114 | 0.156 | 1.55 | 691.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.104 | 0.176 | 0.69 | 626.5 |
| render/acquire/ms | 0.094 | 0.121 | 0.43 | 568.3 |
| stage/render/prepare_collect_phase_buffers | 0.059 | 0.127 | 0.67 | 354.4 |
| physics/step/ms | 0.047 | 0.085 | 0.56 | 283.0 |
| physics/insert/ms | 0.034 | 0.373 | 1.00 | 208.3 |
| stage/render/phase_sort | 0.034 | 0.057 | 0.25 | 203.7 |
| stage/render/cleanup | 0.031 | 0.086 | 0.77 | 185.2 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.55 | 183.1 |
| weather/map/ms | 0.016 | 0.020 | 0.09 | 93.8 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.67 | 87.8 |
| anim/apply/ms | 0.011 | 0.021 | 0.47 | 68.5 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.28 | 66.6 |
| controller/avoid/ms | 0.010 | 0.014 | 0.15 | 62.9 |
| weather/eval/ms | 0.009 | 0.012 | 0.26 | 56.3 |
| script/update/ms | 0.007 | 0.009 | 0.33 | 43.4 |
| script/input/ms | 0.005 | 0.007 | 0.13 | 33.1 |
| stream/despawn/ms | 0.005 | 0.000 | 1.06 | 29.7 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 29.1 |
| script/ui/ms | 0.005 | 0.006 | 0.43 | 27.5 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 20.9 |
| pipelines/driver_ms | 0.003 | 0.000 | 3.09 | 19.3 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 14.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.11 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1277 | 2262 | 649 |
| crowd/c0/meshes/count | count | 353 | 646 | 177 |
| crowd/c1/count | count | 147 | 156 | 147 |
| crowd/c1/joints/count | count | 5539 | 5944 | 5547 |
| crowd/c1/meshes/count | count | 1167 | 1270 | 1164 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 165 | 216 | 204 |
| crowd/joints_extracted/count | count | 3722 | 9436 | 5731 |
| crowd/joints_uploaded/count | count | 3681 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.42 | 2.06 | 0.48 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.67 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.28 | 0.01 |
| encode/finish_critical | ms | 0.95 | 2.13 | 1.12 |
| encode/pass:early prepass/max | ms | 0.77 | 1.68 | 1.12 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.79 | 1.94 | 1.08 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.69 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.17 | 0.27 |
| encode/pass:shadow_directional_light/max | ms | 0.78 | 2.13 | 0.64 |
| encode/pass:shadow_point_light/max | ms | 0.78 | 1.53 | 0.94 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.55 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.11 | 25.76 | 8.23 |
| gpu/timed/ms | ms | 7.41 | 10.76 | 7.65 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 799 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 799 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1285 | 1398 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3356 | 5101 | 3193 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1285 | 1398 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3356 | 5101 | 3193 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 14 | 209 | 4 |
| phases/shadow/bins | count | 3655 | 4498 | 1481 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6135 | 7639 | 3025 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 201 | 227 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 211 | 476 | 19 |
| physics/pending | count | 0 | 235 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 323 | 324 | 324 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 12 | 580 | 9 |
| preprocess/bind_groups_created | count | 12 | 580 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 267 | 124 |
| process/cpu_usage | % | 26.92 | 39.42 | 33.33 |
| process/mem_usage | GiB | 5.41 | 5.57 | 5.57 |
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
| skin/blocks_changed/count | count | 13 | 130 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 164 | 216 | 204 |
| skin/joint_bytes_written/bytes | bytes | 279.8 k | 810.1 k | 439.8 k |
| skin/joints_written/count | count | 4372 | 12.7 k | 6872 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4918 | 5009 | 5009 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.54 | 40.84 | 33.81 |
| system/mem_usage | % | 54.40 | 54.95 | 54.86 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2900 | 3016 | 3016 |
| textures/images | count | 2680 | 2740 | 2740 |
| views/all/batch_sets | count | 65 | 258 | 56 |
| views/all/bins | count | 6199 | 7357 | 4989 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 20.4 k | 10.6 k |
| views/all/pipelines | count | 247 | 275 | 67 |
| views/all/pipelines_distinct | count | 71 | 74 | 63 |
| views/all/work_items | count | 10.6 k | 17.5 k | 8152 |
| views/cascade/batch_sets | count | 8 | 27 | 4 |
| views/cascade/bins | count | 1443 | 1521 | 1481 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3400 | 4452 | 3025 |
| views/cascade/pipelines | count | 22 | 27 | 19 |
| views/cascade/work_items | count | 324 | 4447 | 222 |
| views/lamp_face/batch_sets | count | 0 | 186 | 0 |
| views/lamp_face/bins | count | 2460 | 3084 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2747 | 3406 | 0 |
| views/lamp_face/pipelines | count | 178 | 201 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2786 | 0 |
| views/main/batch_sets | count | 50 | 53 | 52 |
| views/main/bins | count | 2882 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8308 | 13.0 k | 7622 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8474 | 13.2 k | 7930 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.9 k | 30.8 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 42.34 | 231 | 256.2 k |
| anim/events/count | count | 0.09 | 3 | 569 |
| assets/Image/added | count | 0.10 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.58 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1298 | 4675 |
| encode//count | count | 36.29 | 221 | 219.6 k |
| encode/bind_walk/init_textures | count | 14.0 k | 115.2 k | 84.91 M |
| encode/bind_walk/init_textures_skipped | count | 56.9 k | 119.1 k | 344.43 M |
| encode/bind_walk/inits | count | 461 | 1167 | 2.79 M |
| encode/bind_walk/inits_skipped | count | 102 | 143 | 619.8 k |
| encode/bind_walk/merge_textures | count | 14.0 k | 115.1 k | 84.53 M |
| encode/bind_walk/merge_textures_skipped | count | 5385 | 6012 | 32.58 M |
| encode/bind_walk/merges | count | 102 | 346 | 616.7 k |
| encode/bind_walk/merges_skipped | count | 81.56 | 125 | 493.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6051 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6051 |
| encode/pass:early prepass/count | count | 1 | 1 | 6051 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6051 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6051 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6051 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6051 |
| encode/pass:shadow_point_light/count | count | 2.33 | 48 | 14.1 k |
| encode/queue/command_buffers | count | 48.63 | 279 | 294.3 k |
| encode/queue/submits | count | 2.02 | 4 | 12.2 k |
| encode/queue/write_buffer/bytes | bytes | 1.81 M | 2.67 M | 10.95 G |
| encode/queue/write_buffer/calls | count | 256 | 866 | 1.55 M |
| encode/queue/write_buffer_with/bytes | bytes | 170.5 k | 3.53 M | 1.03 G |
| encode/queue/write_buffer_with/calls | count | 11.00 | 465 | 66.6 k |
| encode/queue/write_texture/bytes | bytes | 10.4 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6051 |
| physics/inserts | count | 0.37 | 110 | 2260 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.65 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 8 |
| pipelines/created | count | 0.00 | 1 | 9 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 20 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5594 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1227 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.0 ms; slowest 51, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2469.7 | main/busy/ms 2439, stage/main/post_update 2398, stage/main/update 32.28, stage/main/pre_update 4.45, stage/main/last 1.18, surface/advance_cpu_ms 0.74, stream/drive/ms 0.64, stage/main/first 0.58, stage/main/fixed_main_loop 0.34, surface/cell_upload_ms 0.30, stream/tick/ms 0.15, bigspace/total/ms 0.08 |
| warm-up | 3.25 | 783.9 | main/busy/ms 2580, stage/main/post_update 2422, render/thread/ms 742, render/busy/ms 558, stage/render/prepare_assets 508, stage/render/prepare_views 181, render/acquire/ms 181, stage/main/update 144, stage/render/create_views 29.67, stage/render/render 10.69, stage/render/prepare_resources 6.99, render/graph/ms 6.78 |
| warm-up | 3.36 | 107.8 | render/thread/ms 833, render/busy/ms 597, stage/render/prepare_assets 534, stage/render/prepare_views 182, render/acquire/ms 181, main/busy/ms 161, pipelines/compile_ms 143, stage/main/update 116, pipelines/front_ms 104, stage/render/render 66.98, render/present/ms 54.18, pipelines/driver_ms 38.90 |
| warm-up | 3.39 | 28.4 | pipelines/compile_ms 376, pipelines/front_ms 301, render/thread/ms 115, main/busy/ms 78.16, pipelines/driver_ms 75.20, stage/render/render 74.57, stage/main/post_update 62.68, render/busy/ms 61.39, render/present/ms 53.52, pipelines/wait_ms 30.58, stage/render/prepare_assets 27.12, terrain/update/ms 25.39 |
| warm-up | 3.56 | 166.5 | pipelines/wait_ms 4972, pipelines/compile_ms 1690, pipelines/front_ms 1585, render/thread/ms 140, render/busy/ms 137, pipelines/driver_ms 105, main/busy/ms 100, stage/main/post_update 68.99, encode//sum 43.96, stage/render/prepare_resources 43.24, stage/render/prepare_write_phase_buffers 33.99, stage/render/render 30.43 |
| warm-up | 3.59 | 34.1 | pipelines/wait_ms 5029, pipelines/compile_ms 1889, pipelines/front_ms 1796, render/thread/ms 147, render/busy/ms 145, main/busy/ms 119, pipelines/driver_ms 92.83, stage/main/update 75.48, encode//sum 48.11, stage/render/prepare_resources 44.43, stage/render/prepare_write_phase_buffers 34.11, stage/render/prepare_assets 30.78 |
| warm-up | 3.71 | 122.4 | pipelines/compile_ms 1697, pipelines/front_ms 1615, pipelines/wait_ms 336, render/thread/ms 105, render/busy/ms 103, main/busy/ms 91.66, pipelines/driver_ms 82.42, stage/render/prepare_assets 76.34, stage/main/update 67.06, stage/render/render 18.50, stage/main/post_update 16.76, render/graph/ms 16.06 |
| warm-up | 3.76 | 46.0 | pipelines/compile_ms 2222, pipelines/front_ms 2057, pipelines/wait_ms 251, pipelines/driver_ms 165, render/thread/ms 118, render/busy/ms 115, stage/render/prepare_assets 59.69, stage/render/render 29.83, main/busy/ms 28.91, render/graph/ms 26.27, stage/render/graph_submit 23.92, stage/render/create_views 22.01 |
| warm-up | 3.91 | 72.0 | pipelines/compile_ms 633, pipelines/front_ms 607, render/thread/ms 61.64, render/busy/ms 59.83, main/busy/ms 59.07, stage/main/update 48.20, stream/drive/ms 43.82, stage/render/prepare_assets 41.66, pipelines/driver_ms 25.98, stage/render/render 12.72, render/graph/ms 10.35, stage/render/graph_submit 7.81 |
| warm-up | 3.99 | 62.5 | pipelines/compile_ms 431, pipelines/front_ms 411, main/busy/ms 48.43, render/thread/ms 40.82, render/busy/ms 38.93, stage/main/update 33.71, stream/drive/ms 28.64, stage/render/prepare_assets 20.13, pipelines/driver_ms 20.01, stage/render/render 12.97, render/graph/ms 10.46, stage/main/post_update 9.38 |
| warm-up | 4.04 | 51.2 | pipelines/compile_ms 896, pipelines/front_ms 857, render/thread/ms 72.69, render/busy/ms 70.32, stage/render/prepare_assets 43.40, pipelines/driver_ms 38.47, main/busy/ms 32.82, stage/render/render 20.43, render/graph/ms 17.42, stage/render/graph_submit 14.49, stage/main/post_update 13.48, stage/main/update 11.96 |
| warm-up | 4.09 | 48.6 | pipelines/compile_ms 531, pipelines/front_ms 509, render/thread/ms 73.18, render/busy/ms 71.21, main/busy/ms 48.65, stage/render/prepare_assets 42.75, pipelines/driver_ms 22.40, stage/main/update 21.47, stage/render/render 18.63, stage/main/post_update 17.85, render/graph/ms 16.17, stage/render/graph_submit 13.44 |
| warm-up | 4.17 | 80.6 | pipelines/compile_ms 306, pipelines/front_ms 293, render/thread/ms 88.24, render/busy/ms 86.71, stage/render/prepare_assets 58.36, main/busy/ms 52.27, stage/main/update 28.75, stage/render/render 17.59, render/graph/ms 15.60, stage/main/post_update 15.15, pipelines/driver_ms 13.42, stream/drive/ms 12.59 |
| warm-up | 4.21 | 43.7 | pipelines/compile_ms 1008, pipelines/front_ms 974, render/thread/ms 95.88, render/busy/ms 93.97, main/busy/ms 64.42, stage/render/prepare_assets 57.88, stage/main/update 43.28, pipelines/wait_ms 41.24, pipelines/driver_ms 33.53, stage/render/render 19.82, render/graph/ms 17.10, stage/main/post_update 13.18 |
| warm-up | 4.25 | 43.2 | pipelines/compile_ms 1228, pipelines/front_ms 1192, render/thread/ms 64.04, render/busy/ms 62.21, main/busy/ms 57.56, pipelines/wait_ms 42.58, pipelines/driver_ms 36.42, stage/main/update 36.27, stage/render/prepare_assets 34.59, stage/render/render 12.32, stage/main/post_update 10.63, render/graph/ms 9.80 |
| warm-up | 4.33 | 72.9 | pipelines/compile_ms 768, pipelines/front_ms 744, render/thread/ms 97.63, render/busy/ms 91.57, main/busy/ms 89.61, stage/render/render 49.00, render/graph/ms 44.58, stage/main/post_update 42.82, stage/main/update 37.91, pipelines/wait_ms 26.07, pipelines/driver_ms 23.85, stage/render/graph_submit 19.80 |
| warm-up | 4.40 | 72.6 | pipelines/compile_ms 1374, pipelines/front_ms 1326, pipelines/wait_ms 223, render/thread/ms 129, render/busy/ms 123, main/busy/ms 87.83, stage/render/render 51.08, pipelines/driver_ms 47.88, render/graph/ms 46.38, stage/main/post_update 45.20, stage/main/update 36.10, stage/render/graph_submit 20.31 |
| warm-up | 4.47 | 75.0 | pipelines/compile_ms 1976, pipelines/front_ms 1912, pipelines/wait_ms 427, render/thread/ms 122, render/busy/ms 120, pipelines/driver_ms 63.76, stage/render/prepare_assets 43.16, main/busy/ms 33.40, stage/render/prepare_flush 24.45, stage/render/render 19.81, stage/main/post_update 17.54, render/graph/ms 17.36 |
| warm-up | 4.56 | 82.2 | pipelines/compile_ms 2206, pipelines/front_ms 2130, pipelines/wait_ms 868, render/thread/ms 144, render/busy/ms 142, pipelines/driver_ms 76.19, main/busy/ms 68.95, stage/render/prepare_assets 55.19, stage/main/post_update 49.89, stage/render/render 26.39, encode//sum 25.84, render/graph/ms 23.72 |
| warm-up | 4.63 | 72.1 | pipelines/compile_ms 2172, pipelines/front_ms 2095, pipelines/wait_ms 882, main/busy/ms 161, render/thread/ms 135, render/busy/ms 133, stage/main/update 101, pipelines/driver_ms 77.18, stream/drive/ms 61.71, stage/render/prepare_assets 49.78, stage/main/post_update 46.45, encode//sum 37.63 |
| warm-up | 4.78 | 154.3 | pipelines/compile_ms 2751, pipelines/front_ms 2664, pipelines/wait_ms 661, render/thread/ms 134, render/busy/ms 131, main/busy/ms 131, stage/main/update 105, pipelines/driver_ms 87.75, stream/drive/ms 63.13, stage/render/prepare_assets 59.66, stage/render/render 41.82, encode//sum 39.36 |
| warm-up | 4.82 | 32.7 | pipelines/compile_ms 2137, pipelines/front_ms 2071, pipelines/wait_ms 421, render/thread/ms 105, render/busy/ms 103, main/busy/ms 86.53, pipelines/driver_ms 66.39, stage/main/update 64.25, stream/drive/ms 55.01, stage/render/prepare_assets 46.88, stage/render/render 27.08, render/graph/ms 24.30 |
| warm-up | 4.93 | 109.3 | pipelines/compile_ms 542, pipelines/front_ms 524, main/busy/ms 99.79, stage/main/update 73.62, render/thread/ms 71.41, render/busy/ms 69.95, stream/drive/ms 54.61, stage/render/prepare_assets 45.53, pipelines/driver_ms 18.85, stage/main/post_update 16.43, stage/render/render 10.35, render/graph/ms 8.28 |
| warm-up | 4.97 | 44.3 | pipelines/compile_ms 1485, pipelines/front_ms 1436, main/busy/ms 91.21, render/thread/ms 78.63, render/busy/ms 77.24, stage/main/update 66.11, stage/render/prepare_assets 54.99, pipelines/driver_ms 48.76, stream/drive/ms 45.58, stage/main/post_update 15.93, stage/render/render 10.17, render/graph/ms 7.92 |
| warm-up | 5.04 | 66.5 | pipelines/compile_ms 1460, pipelines/front_ms 1413, main/busy/ms 82.18, render/thread/ms 68.85, render/busy/ms 67.24, stage/main/update 64.61, pipelines/driver_ms 46.38, stream/drive/ms 44.86, stage/render/prepare_assets 30.72, stage/render/render 19.05, render/graph/ms 16.75, stage/render/graph_submit 13.82 |
| warm-up | 5.06 | 28.0 | pipelines/compile_ms 274, pipelines/front_ms 264, main/busy/ms 73.90, stage/main/update 56.57, render/thread/ms 54.49, render/busy/ms 53.01, stream/drive/ms 32.96, stage/render/prepare_assets 18.97, stage/render/render 18.18, render/graph/ms 16.18, stage/render/graph_submit 13.65, encode/submit 11.59 |
| warm-up | 5.12 | 52.2 | pipelines/compile_ms 193, pipelines/front_ms 188, main/busy/ms 61.93, stage/main/update 45.20, render/thread/ms 36.95, render/busy/ms 35.71, stream/drive/ms 33.05, stage/render/prepare_assets 16.23, stage/main/post_update 10.16, stage/render/render 8.94, render/graph/ms 7.00, encode//sum 6.63 |
| warm-up | 5.14 | 26.5 | pipelines/compile_ms 869, pipelines/front_ms 831, render/thread/ms 40.23, render/busy/ms 38.87, pipelines/driver_ms 37.72, main/busy/ms 32.57, stage/main/update 14.25, stage/main/post_update 12.64, stage/render/render 11.41, stage/render/prepare_assets 11.04, render/graph/ms 9.49, encode//sum 8.29 |
| warm-up | 5.17 | 26.6 | pipelines/compile_ms 676, pipelines/front_ms 643, render/thread/ms 47.66, render/busy/ms 46.15, main/busy/ms 35.29, pipelines/driver_ms 32.85, stage/render/render 17.13, stage/main/update 16.09, render/graph/ms 15.11, stage/main/post_update 12.42, encode//sum 10.52, stage/render/graph_submit 9.99 |
| warm-up | 5.21 | 36.0 | render/thread/ms 54.83, render/busy/ms 53.31, main/busy/ms 43.01, stage/main/update 22.42, stage/render/render 18.33, render/graph/ms 16.39, stage/main/post_update 13.67, stage/render/graph_submit 11.86, stage/render/prepare_assets 10.64, encode//sum 10.45, encode/submit 7.44, stage/render/create_views 6.18 |
| warm-up | 5.27 | 59.8 | pipelines/compile_ms 115, pipelines/front_ms 110, render/thread/ms 86.95, render/busy/ms 85.53, main/busy/ms 69.99, stage/main/post_update 42.39, stage/render/prepare_write_phase_buffers 24.80, stage/main/update 21.07, stage/render/render 17.16, render/graph/ms 15.24, stage/render/prepare_resources 12.75, stage/render/prepare_assets 12.42 |
| warm-up | 5.36 | 73.7 | pipelines/compile_ms 1663, pipelines/front_ms 1612, render/thread/ms 82.46, render/busy/ms 80.57, main/busy/ms 54.59, pipelines/driver_ms 51.83, pipelines/wait_ms 49.15, stage/render/prepare_assets 45.70, stage/render/render 24.25, stage/main/update 22.83, stage/main/post_update 21.97, render/graph/ms 21.67 |
| warm-up | 5.46 | 32.3 | pipelines/compile_ms 367, pipelines/front_ms 359, render/thread/ms 45.27, render/busy/ms 43.55, main/busy/ms 31.49, stage/render/prepare_assets 18.12, stage/render/render 17.06, render/graph/ms 14.51, stage/main/post_update 13.34, encode//sum 12.46, stage/main/update 12.10, stage/render/graph_submit 10.20 |
| warm-up | 5.49 | 26.3 | pipelines/compile_ms 540, pipelines/front_ms 530, render/thread/ms 48.37, render/busy/ms 47.06, pipelines/wait_ms 31.02, stage/render/prepare_assets 27.60, main/busy/ms 23.00, stage/render/render 11.76, pipelines/driver_ms 10.26, encode//sum 10.23, render/graph/ms 9.75, stage/main/update 9.46 |
| warm-up | 5.58 | 35.3 | render/thread/ms 48.36, render/busy/ms 46.68, main/busy/ms 35.72, stage/render/prepare_assets 23.11, stage/main/update 20.38, stage/render/render 17.29, render/graph/ms 14.99, stream/drive/ms 13.60, encode//sum 12.76, stage/render/graph_submit 11.47, stage/main/post_update 9.78, encode/submit 8.66 |
| warm-up | 5.64 | 29.5 | render/thread/ms 48.59, render/busy/ms 47.02, main/busy/ms 30.59, stage/render/prepare_assets 17.35, stage/render/render 17.05, render/graph/ms 15.01, stage/main/post_update 13.80, stage/main/update 11.27, stage/render/graph_submit 10.17, encode//sum 10.08, encode/submit 6.96, encode/pass:shadow_point_light/sum 5.43 |
| warm-up | 5.66 | 26.2 | render/thread/ms 46.38, render/busy/ms 45.38, stage/render/prepare_assets 24.90, main/busy/ms 23.08, stage/render/render 11.67, encode//sum 10.59, render/graph/ms 10.05, stage/main/post_update 9.56, stage/main/update 8.05, stage/render/graph_submit 6.25, encode/pass:shadow_point_light/sum 4.50, stage/render/camera_driver 3.66 |
| warm-up | 5.69 | 26.9 | pipelines/compile_ms 172, pipelines/front_ms 163, pipelines/wait_ms 54.60, render/thread/ms 44.72, render/busy/ms 43.60, stage/render/prepare_assets 25.82, main/busy/ms 20.08, stage/render/render 11.22, encode//sum 11.11, render/graph/ms 9.49, pipelines/driver_ms 8.82, stage/main/post_update 8.28 |
| warm-up | 6.22 | 33.5 | render/thread/ms 38.88, render/busy/ms 37.74, main/busy/ms 24.65, stage/render/prepare_assets 15.47, stage/main/update 10.48, stage/render/render 10.47, stage/main/post_update 9.07, pipelines/compile_ms 8.92, encode//sum 8.91, render/graph/ms 8.78, pipelines/front_ms 8.23, stage/render/graph_submit 5.89 |
| warm-up | 6.44 | 27.8 | render/thread/ms 44.06, main/busy/ms 43.85, render/busy/ms 42.10, stage/main/post_update 18.49, stage/main/update 18.21, stage/render/render 17.56, render/graph/ms 15.30, encode//sum 12.24, stage/render/graph_submit 10.74, stage/render/prepare_assets 8.93, encode/submit 7.51, encode/pass:shadow_point_light/sum 7.46 |
| warm-up | 6.47 | 30.8 | render/thread/ms 46.83, render/busy/ms 45.18, main/busy/ms 42.11, stage/main/update 19.77, stage/render/render 16.94, stage/main/post_update 15.70, stage/render/prepare_assets 15.37, render/graph/ms 14.72, encode//sum 12.11, stage/render/graph_submit 10.75, encode/pass:shadow_point_light/sum 8.21, encode/submit 7.51 |
| warm-up | 6.57 | 29.5 | pipelines/compile_ms 160, pipelines/front_ms 155, render/thread/ms 42.20, render/busy/ms 41.07, main/busy/ms 40.16, stage/main/update 19.47, stage/main/post_update 14.71, stage/render/prepare_assets 13.29, stage/render/render 13.13, render/graph/ms 10.92, encode//sum 9.96, encode/pass:shadow_point_light/sum 7.39 |
| warm-up | 6.68 | 27.7 | render/thread/ms 42.90, render/busy/ms 41.98, main/busy/ms 24.24, stage/render/prepare_assets 23.10, stage/main/post_update 11.87, stage/render/render 10.15, encode//sum 9.82, render/graph/ms 8.66, stage/main/update 8.55, encode/pass:shadow_point_light/sum 6.18, stage/render/graph_submit 5.82, stream/drive/ms 4.01 |
| warm-up | 6.76 | 32.5 | pipelines/compile_ms 153, pipelines/front_ms 147, render/thread/ms 47.72, render/busy/ms 46.43, main/busy/ms 38.48, stage/main/update 24.33, stage/render/prepare_assets 23.03, stream/drive/ms 17.31, stage/render/render 13.69, encode//sum 11.91, render/graph/ms 11.70, stage/main/post_update 9.91 |
| warm-up | 7.02 | 36.2 | main/busy/ms 42.71, render/thread/ms 34.89, render/busy/ms 33.61, stage/main/update 24.73, pipelines/compile_ms 20.81, pipelines/front_ms 19.72, stream/drive/ms 14.48, stage/render/render 13.29, stage/render/prepare_assets 12.27, render/graph/ms 11.52, stage/main/post_update 11.16, encode//sum 9.87 |
| measure | 0.78 | 26.1 | main/busy/ms 34.44, render/thread/ms 26.44, render/busy/ms 25.32, stage/main/update 19.16, stage/render/render 13.50, stream/drive/ms 12.70, stage/main/post_update 12.48, render/graph/ms 11.93, encode//sum 9.19, stage/render/graph_submit 8.60, encode/pass:shadow_point_light/sum 6.88, stage/render/prepare_assets 5.71 |
| measure | 2.17 | 26.6 | encode//sum 38.99, render/thread/ms 37.55, render/busy/ms 35.99, stage/render/render 24.88, render/graph/ms 22.63, encode/pass:shadow_point_light/sum 21.20, main/busy/ms 19.92, stage/render/graph_submit 16.71, stage/main/post_update 12.45, pipelines/compile_ms 11.94, pipelines/front_ms 11.72, encode/submit 9.41 |
| measure | 2.72 | 27.6 | render/thread/ms 38.42, render/busy/ms 37.23, encode//sum 33.12, stage/render/render 24.73, render/graph/ms 22.85, encode/pass:shadow_point_light/sum 21.43, main/busy/ms 20.02, stage/render/graph_submit 16.44, stage/main/post_update 13.15, encode/submit 9.65, encode/finish_wall 6.83, stage/render/camera_driver 6.29 |
| measure | 18.17 | 28.1 | main/busy/ms 35.17, stage/main/update 21.34, render/thread/ms 18.71, render/busy/ms 17.74, stream/drive/ms 17.05, stage/main/post_update 11.39, stage/render/render 10.07, render/graph/ms 8.56, encode//sum 7.71, stage/render/graph_submit 6.13, encode/submit 3.99, stage/render/camera_driver 2.34 |
| measure | 23.07 | 26.9 | render/thread/ms 37.17, render/busy/ms 35.84, main/busy/ms 32.60, stage/main/post_update 14.50, stage/main/update 13.93, stage/render/render 13.66, render/graph/ms 11.70, encode//sum 11.32, stage/render/prepare_assets 8.98, stage/render/graph_submit 7.66, encode/pass:shadow_point_light/sum 5.62, encode/submit 4.81 |
| measure | 59.05 | 26.4 | render/thread/ms 37.20, render/busy/ms 36.02, main/busy/ms 26.42, stage/render/prepare_assets 14.52, stage/main/post_update 14.25, stage/render/render 11.87, encode//sum 10.68, render/graph/ms 10.33, stage/main/update 7.41, stage/render/graph_submit 7.11, encode/submit 4.15, stage/render/camera_driver 3.07 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=base-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 10%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **72%** (min 49%) · 2625 MHz (min 2625) · memory 8101 MHz · 76.0 W · 76 °C (max 82) · VRAM 6756 MiB · throttled 100% of samples

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
