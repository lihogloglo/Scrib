# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5760 frames over 65.7 s · **median 10.95 ms (91 fps)** · mean 11.40 · p95 14.18 · p99 16.28 (1% low 61 fps) · worst 35.41 ms · 50 spikes over 1.5× median · 32.0 stutters/min (work 28.3/min)
- warm-up: 354 frames over 9.7 s, worst 2551.7 ms
- hitches (> 27.4 ms): **6** measured, 45 in warm-up
- timed GPU passes: 7.24 ms of a 11.40 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.95 ms, limit 11 ms
- **FAIL** frame_ms: 35 stutters (> 1.5x moving median of 60) in 1.09 min = 31.99/min, limit 1/min
- **FAIL** worst frame 35.41 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.153 | 2.186 | 2.654 | 0.99 | 0.178 |
| main_opaque_pass_3d | 1.348 | 1.369 | 1.887 | 0.99 | 0.084 |
| early prepass | 0.535 | 0.543 | 0.763 | 0.99 | 0.094 |
| auxiliary_views | 0.338 | 0.343 | 1.206 | 0.99 | 0.303 |
| bloom | 0.337 | 0.342 | 0.493 | 0.99 | 0.067 |
| atmosphere_luts | 0.324 | 0.329 | 0.461 | 0.99 | 0.003 |
| water_velocity | 0.234 | 0.238 | 0.247 | 0.99 | 0.002 |
| water_sspr | 0.209 | 0.212 | 0.203 | 0.99 | 0.003 |
| ssao | 0.195 | 0.198 | 0.205 | 0.99 | 0.008 |
| atmos_froxel_inject | 0.137 | 0.139 | 0.130 | 0.99 | 0.002 |
| upscaling | 0.136 | 0.138 | 0.161 | 0.99 | 0.005 |
| water_fft | 0.127 | 0.129 | 0.127 | 0.99 | 0.003 |
| uniform_allocation | 0.119 | 0.121 | 0.143 | 0.99 | 0.026 |
| clouds_composite | 0.105 | 0.107 | 0.109 | 0.99 | 0.002 |
| bin_unpacking | 0.095 | 0.096 | 0.115 | 0.99 | 0.019 |
| gpu_instance_materialization | 0.094 | 0.095 | 0.100 | 0.99 | 0.004 |
| tonemapping | 0.092 | 0.094 | 0.096 | 0.99 | 0.004 |
| main_transmissive_pass_3d | 0.078 | 0.079 | 0.145 | 0.99 | 0.009 |
| clustering | 0.065 | 0.066 | 0.068 | 0.99 | 0.020 |
| early_mesh_preprocessing | 0.056 | 0.056 | 0.071 | 0.99 | 0.005 |
| clouds_trace | 0.055 | 0.056 | 0.077 | 0.99 | 0.001 |
| render_sky | 0.054 | 0.055 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.053 | 0.468 | 0.766 | 0.11 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.99 | 0.001 |
| surface_trail | 0.043 | 0.059 | 0.057 | 0.72 | 0.002 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.032 | 0.99 | 0.001 |
| water_snapshot | 0.026 | 0.026 | 0.034 | 0.99 | 0.002 |
| lightprobe_radiance_map | 0.025 | 0.220 | 0.230 | 0.11 | 0.004 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.99 | 0.002 |
| range_unpacking | 0.022 | 0.023 | 0.028 | 0.99 | 0.004 |
| per_view_shadow_pass | 0.018 | 0.019 | 0.027 | 0.99 | 0.006 |
| night_sky | 0.017 | 0.017 | 0.039 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.016 | 0.017 | 0.032 | 0.99 | 0.207 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.98 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 0.99 | 0.003 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.033 | 12.726 | 28.71 | 57791.4 |
| render/busy/ms | 9.525 | 12.229 | 27.03 | 54864.7 |
| main/busy/ms | 8.952 | 10.749 | 31.38 | 51566.4 |
| stage/render/render | 6.127 | 8.274 | 24.16 | 35292.1 |
| render/graph/ms | 5.437 | 7.541 | 12.00 | 31315.5 |
| stage/main/post_update | 5.219 | 6.488 | 11.45 | 30059.7 |
| encode//sum | 4.871 | 7.115 | 19.17 | 28059.0 |
| stage/render/graph_submit | 3.893 | 5.389 | 8.60 | 22426.2 |
| encode/submit | 2.519 | 3.698 | 5.57 | 14511.7 |
| stage/main/update | 1.900 | 2.448 | 24.08 | 10944.4 |
| stage/render/camera_driver | 1.487 | 2.145 | 4.57 | 8562.4 |
| encode/finish_wall | 1.416 | 1.868 | 3.58 | 8155.5 |
| encode/pass:shadow_directional_light/sum | 0.823 | 1.282 | 2.97 | 4742.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.820 | 1.137 | 2.25 | 4723.2 |
| encode/pass:early prepass/sum | 0.813 | 1.132 | 1.96 | 4681.3 |
| stage/render/prepare_write_phase_buffers | 0.761 | 1.154 | 7.96 | 4385.9 |
| stage/main/fixed_main_loop | 0.586 | 0.972 | 3.06 | 3378.1 |
| stage/render/prepare_assets | 0.575 | 0.733 | 12.47 | 3313.8 |
| encode/pass:shadow_point_light/sum | 0.553 | 2.837 | 4.46 | 3184.2 |
| bigspace/total/ms | 0.516 | 1.192 | 2.10 | 2974.0 |
| stage/render/prepare_resources | 0.486 | 0.597 | 1.50 | 2799.0 |
| stage/render/prepare_meshes | 0.475 | 0.609 | 4.28 | 2733.2 |
| render/present/ms | 0.418 | 0.515 | 16.32 | 2409.3 |
| stage/main/pre_update | 0.403 | 0.567 | 2.87 | 2318.8 |
| stage/main/first | 0.400 | 0.525 | 5.75 | 2306.4 |
| controller/feed/ms | 0.354 | 0.453 | 0.79 | 2037.2 |
| stage/render/prepare_bind_groups | 0.334 | 0.568 | 2.08 | 1924.1 |
| stage/render/prepare_flush | 0.304 | 0.538 | 2.37 | 1752.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.300 | 0.466 | 1.46 | 1727.8 |
| stage/render/prepare_batch_phases | 0.295 | 0.457 | 0.86 | 1699.8 |
| stage/main/last | 0.256 | 0.317 | 0.85 | 1473.9 |
| bigspace/low_precision/ms | 0.220 | 0.550 | 1.64 | 1264.5 |
| stream/drive/ms | 0.214 | 0.312 | 10.25 | 1234.4 |
| stage/render/create_views | 0.204 | 0.264 | 7.90 | 1175.0 |
| stage/render/specialize | 0.186 | 0.281 | 2.47 | 1071.2 |
| stage/render/extract_commands | 0.176 | 0.209 | 0.56 | 1014.5 |
| stage/render/queue | 0.175 | 0.256 | 2.45 | 1005.5 |
| anim/evaluate/ms | 0.167 | 0.286 | 1.57 | 961.1 |
| anim/evaluate_cpu/ms | 0.160 | 0.205 | 0.48 | 919.2 |
| stream/tick/ms | 0.155 | 0.234 | 0.49 | 890.3 |
| bigspace/high_precision/ms | 0.138 | 0.456 | 1.65 | 793.9 |
| stage/render/prepare_views | 0.137 | 0.185 | 0.48 | 787.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.109 | 0.179 | 0.72 | 628.2 |
| render/acquire/ms | 0.090 | 0.121 | 0.44 | 517.5 |
| controller/move/ms | 0.090 | 0.219 | 0.76 | 517.4 |
| controller/write/ms | 0.074 | 0.098 | 0.55 | 426.1 |
| terrain/update/ms | 0.063 | 0.299 | 1.88 | 363.9 |
| stage/render/prepare_collect_phase_buffers | 0.049 | 0.076 | 0.31 | 280.7 |
| stage/render/phase_sort | 0.046 | 0.077 | 0.33 | 263.5 |
| physics/step/ms | 0.045 | 0.083 | 0.57 | 258.6 |
| encode/resolve_diagnostics/sum | 0.033 | 0.045 | 0.38 | 191.1 |
| physics/insert/ms | 0.032 | 0.362 | 0.93 | 184.3 |
| pipelines/compile_ms | 0.032 | 0.000 | 86.78 | 182.2 |
| pipelines/front_ms | 0.031 | 0.000 | 83.90 | 176.1 |
| stage/render/cleanup | 0.025 | 0.038 | 2.54 | 145.1 |
| terrain/select/ms | 0.025 | 0.264 | 0.48 | 144.8 |
| weather/map/ms | 0.017 | 0.023 | 0.25 | 98.0 |
| far/select/ms | 0.017 | 0.175 | 0.53 | 95.3 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.023 | 0.23 | 84.3 |
| anim/apply/ms | 0.013 | 0.025 | 0.22 | 73.6 |
| encode/camera_driver_span_end/sum | 0.011 | 0.021 | 0.21 | 66.0 |
| weather/eval/ms | 0.011 | 0.014 | 0.19 | 61.4 |
| script/update/ms | 0.009 | 0.013 | 0.14 | 50.6 |
| controller/avoid/ms | 0.007 | 0.013 | 0.13 | 40.5 |
| stream/despawn/ms | 0.006 | 0.000 | 1.14 | 36.2 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 31.0 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 27.7 |
| stream/spawn/ms | 0.004 | 0.000 | 1.10 | 22.5 |
| stream/upload/ms | 0.004 | 0.000 | 1.44 | 21.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.67 | 14.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.2 |
| pipelines/driver_ms | 0.001 | 0.000 | 3.00 | 6.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.47 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.06 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.14 | 2.4 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 34 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1291 | 2262 | 538 |
| crowd/c0/meshes/count | count | 355 | 646 | 129 |
| crowd/c1/count | count | 145 | 156 | 149 |
| crowd/c1/joints/count | count | 5531 | 5982 | 5658 |
| crowd/c1/meshes/count | count | 1163 | 1284 | 1212 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 162 | 216 | 205 |
| crowd/joints_extracted/count | count | 700 | 5614 | 719 |
| crowd/joints_uploaded/count | count | 3636 | 6220 | 5763 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.2 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.48 | 2.21 | 0.51 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.23 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.21 | 0.01 |
| encode/finish_critical | ms | 0.98 | 2.97 | 0.95 |
| encode/pass:early prepass/max | ms | 0.81 | 1.96 | 0.72 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 2.25 | 0.95 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.72 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.28 | 1.46 | 0.33 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 2.97 | 0.80 |
| encode/pass:shadow_point_light/max | ms | 0.13 | 1.93 | 0.97 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.38 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 9.31 | 31.38 | 8.96 |
| gpu/timed/ms | ms | 8.01 | 9.90 | 8.06 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 159 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 348 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 159 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 348 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 769 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2490 | 3537 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 769 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2490 | 3537 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 27 | 159 | 8 |
| phases/shadow/bins | count | 1250 | 1684 | 658 |
| phases/shadow/instance_batches | count | 4 | 8 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| phases/shadow/items | count | 2925 | 3684 | 2022 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 153 | 203 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3182 | 2349 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 188 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 332 | 333 | 333 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 19 | 470 | 11 |
| preprocess/bind_groups_created | count | 14 | 464 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 5 |
| preprocess/bind_groups_reused | count | 130 | 247 | 123 |
| process/cpu_usage | % | 25.48 | 39.42 | 32.88 |
| process/mem_usage | GiB | 5.51 | 5.70 | 5.69 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2976 | 3166 | 3166 |
| scene/batch/masked/live | count | 2976 | 3166 | 3166 |
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
| scene/batch/opaque/capacity | count | 41.0 k | 41.0 k | 41.0 k |
| scene/batch/opaque/count | count | 2 | 2 | 2 |
| scene/batch/opaque/high_water | count | 18.8 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 18.8 k | 19.9 k | 19.9 k |
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
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1591 | 1621 | 1621 |
| scene/chains/live | count | 1591 | 1621 | 1621 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 95 | 13 |
| skin/blocks_skipped/count | count | 146 | 206 | 191 |
| skin/blocks_walked/count | count | 14 | 29 | 14 |
| skin/joint_bytes_written/bytes | bytes | 49.7 k | 811.1 k | 45.8 k |
| skin/joints_written/count | count | 776 | 12.7 k | 716 |
| skin/upload_writes/count | count | 8 | 28 | 9 |
| started_load_count |  | 6441 | 6566 | 6566 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.14 | 51.30 | 30.72 |
| system/mem_usage | % | 56.97 | 57.66 | 57.66 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2666 | 2724 | 2724 |
| views/all/batch_sets | count | 83 | 218 | 66 |
| views/all/bins | count | 3224 | 3812 | 2948 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 12 | 16 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 147.5 k |
| views/all/items | count | 8602 | 10.7 k | 7964 |
| views/all/pipelines | count | 202 | 250 | 66 |
| views/all/pipelines_distinct | count | 70 | 74 | 61 |
| views/all/work_items | count | 154.1 k | 206.6 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 34 | 8 |
| views/cascade/bins | count | 722 | 880 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2122 | 2441 | 2022 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 4 | 130 | 0 |
| views/lamp_face/bins | count | 529 | 838 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 21 | 48 | 0 |
| views/lamp_face/drawn | count | 1 | 44 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 778 | 1247 | 0 |
| views/lamp_face/pipelines | count | 129 | 175 | 0 |
| views/lamp_face/revealed | count | 0 | 3 | 0 |
| views/lamp_face/work_items | count | 27 | 694 | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2028 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5912 | 7494 | 5942 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 104.6 k | 106.0 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 20.9 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 58.25 | 263 | 335.5 k |
| anim/events/count | count | 0.09 | 3 | 541 |
| assets/Image/added | count | 0.10 | 35 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.50 | 288 | 2874 |
| assets/Mesh/removed | count | 0.84 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.30 | 10 | 7481 |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.26 | 1133 | 1490 |
| encode//count | count | 41.20 | 204 | 237.3 k |
| encode/bind_walk/init_textures | count | 9985 | 29.1 k | 57.51 M |
| encode/bind_walk/init_textures_skipped | count | 61.0 k | 84.9 k | 351.58 M |
| encode/bind_walk/inits | count | 584 | 1021 | 3.36 M |
| encode/bind_walk/inits_skipped | count | 145 | 239 | 837.7 k |
| encode/bind_walk/merge_textures | count | 9922 | 29.0 k | 57.15 M |
| encode/bind_walk/merge_textures_skipped | count | 11.9 k | 26.6 k | 68.37 M |
| encode/bind_walk/merges | count | 109 | 284 | 626.5 k |
| encode/bind_walk/merges_skipped | count | 134 | 233 | 769.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5760 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5760 |
| encode/pass:early prepass/count | count | 1 | 1 | 5760 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5760 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5760 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5760 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5760 |
| encode/pass:shadow_point_light/count | count | 3.30 | 44 | 19.0 k |
| encode/queue/command_buffers | count | 54.52 | 258 | 314.0 k |
| encode/queue/submits | count | 2.01 | 4 | 11.6 k |
| encode/queue/write_buffer/bytes | bytes | 2.34 M | 3.53 M | 13.50 G |
| encode/queue/write_buffer/calls | count | 286 | 715 | 1.65 M |
| encode/queue/write_buffer_with/bytes | bytes | 161.7 k | 1.28 M | 931.26 M |
| encode/queue/write_buffer_with/calls | count | 10.48 | 873 | 60.4 k |
| encode/queue/write_texture/bytes | bytes | 2211 | 589.8 k | 12.74 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5760 |
| physics/inserts | count | 0.39 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 177 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 1 | 3 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 37 | 223 |
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5877 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1289 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.4 ms; slowest 51, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2551.7 | main/busy/ms 2516, stage/main/post_update 2471, stage/main/update 34.61, stage/main/pre_update 6.47, stage/main/last 1.24, surface/advance_cpu_ms 0.74, stage/main/first 0.57, stage/main/fixed_main_loop 0.41, surface/cell_upload_ms 0.37, stream/drive/ms 0.33, bigspace/total/ms 0.11, stream/tick/ms 0.10 |
| warm-up | 3.37 | 818.8 | main/busy/ms 2667, stage/main/post_update 2502, render/thread/ms 774, render/busy/ms 612, stage/render/prepare_assets 569, stage/render/prepare_views 159, render/acquire/ms 159, stage/main/update 148, stage/render/create_views 26.40, stage/render/render 10.85, stage/main/pre_update 8.67, render/graph/ms 7.67 |
| warm-up | 3.43 | 62.6 | render/thread/ms 820, render/busy/ms 657, stage/render/prepare_assets 599, main/busy/ms 175, stage/render/prepare_views 160, render/acquire/ms 160, stage/main/update 119, pipelines/compile_ms 75.42, pipelines/front_ms 71.32, stage/main/post_update 42.21, stage/render/create_views 26.96, terrain/update/ms 25.15 |
| warm-up | 3.50 | 67.4 | pipelines/wait_ms 374, pipelines/compile_ms 337, pipelines/front_ms 308, main/busy/ms 107, render/thread/ms 102, stage/main/post_update 85.19, render/busy/ms 78.23, stage/render/render 53.31, stage/render/prepare_assets 31.89, terrain/update/ms 31.51, render/graph/ms 29.33, pipelines/driver_ms 29.07 |
| warm-up | 3.66 | 156.1 | pipelines/wait_ms 6896, pipelines/compile_ms 1460, pipelines/front_ms 1375, render/thread/ms 192, render/busy/ms 169, main/busy/ms 141, stage/main/post_update 123, pipelines/driver_ms 84.83, stage/render/render 78.46, render/graph/ms 54.94, stage/render/graph_submit 36.15, stage/render/prepare_flush 34.37 |
| warm-up | 3.70 | 44.0 | pipelines/wait_ms 7054, pipelines/compile_ms 1928, pipelines/front_ms 1844, render/thread/ms 178, render/busy/ms 176, main/busy/ms 125, pipelines/driver_ms 84.37, stage/main/post_update 63.10, stage/main/update 55.85, stage/render/render 41.56, encode//sum 39.70, render/graph/ms 39.15 |
| warm-up | 3.80 | 98.4 | pipelines/compile_ms 2087, pipelines/front_ms 1990, pipelines/wait_ms 775, render/thread/ms 110, render/busy/ms 106, pipelines/driver_ms 96.81, main/busy/ms 89.09, stage/render/prepare_assets 70.74, stage/main/update 57.00, stage/main/post_update 23.71, stage/render/render 20.95, render/graph/ms 16.48 |
| warm-up | 3.83 | 32.6 | pipelines/compile_ms 1469, pipelines/front_ms 1389, pipelines/wait_ms 246, render/thread/ms 97.94, render/busy/ms 93.92, pipelines/driver_ms 79.55, stage/render/prepare_assets 74.07, main/busy/ms 42.83, stage/main/post_update 21.85, stage/render/render 16.18, stage/main/update 12.82, terrain/update/ms 12.74 |
| warm-up | 3.89 | 32.5 | pipelines/compile_ms 773, pipelines/front_ms 741, render/thread/ms 54.22, render/busy/ms 52.14, main/busy/ms 40.42, pipelines/wait_ms 40.35, pipelines/driver_ms 32.22, stage/main/post_update 24.36, encode//sum 20.55, stage/render/render 19.47, render/graph/ms 16.86, stage/render/prepare_resources 12.32 |
| warm-up | 4.07 | 130.8 | pipelines/compile_ms 1977, pipelines/front_ms 1894, pipelines/driver_ms 82.76, main/busy/ms 82.64, render/thread/ms 78.88, render/busy/ms 77.22, stage/main/update 64.64, stage/render/prepare_assets 45.68, pipelines/wait_ms 29.56, stage/render/render 19.67, render/graph/ms 17.25, stage/render/graph_submit 11.14 |
| warm-up | 4.11 | 43.3 | pipelines/compile_ms 1784, pipelines/front_ms 1711, render/thread/ms 81.09, render/busy/ms 79.52, pipelines/driver_ms 73.65, stage/render/prepare_assets 58.23, pipelines/wait_ms 31.28, main/busy/ms 23.78, stage/render/render 18.04, render/graph/ms 16.17, stage/main/update 11.18, stage/render/graph_submit 10.93 |
| warm-up | 4.22 | 47.2 | pipelines/compile_ms 84.87, pipelines/front_ms 81.22, render/thread/ms 57.35, render/busy/ms 55.68, main/busy/ms 55.23, stage/main/update 32.17, stage/render/render 21.99, stage/render/prepare_flush 16.06, stage/main/post_update 16.05, render/graph/ms 12.85, stream/drive/ms 10.31, stage/render/graph_submit 9.02 |
| warm-up | 4.27 | 54.7 | pipelines/compile_ms 841, pipelines/front_ms 800, render/thread/ms 88.52, render/busy/ms 86.54, main/busy/ms 61.07, pipelines/driver_ms 41.26, stage/main/update 35.51, stage/render/prepare_assets 32.03, stage/render/render 24.87, stage/main/post_update 20.37, render/graph/ms 15.16, stage/render/prepare_flush 15.13 |
| warm-up | 4.30 | 28.0 | pipelines/compile_ms 813, pipelines/front_ms 773, render/thread/ms 64.74, render/busy/ms 62.77, main/busy/ms 51.74, pipelines/driver_ms 40.11, stage/render/prepare_assets 35.90, stage/main/update 32.38, stream/drive/ms 19.95, stage/render/render 14.24, stage/main/post_update 12.27, render/graph/ms 11.39 |
| warm-up | 4.39 | 92.8 | pipelines/compile_ms 1417, pipelines/front_ms 1377, render/thread/ms 63.38, render/busy/ms 61.74, main/busy/ms 51.07, stage/render/prepare_assets 41.64, pipelines/driver_ms 39.79, stage/main/update 33.18, stream/drive/ms 21.28, stage/render/render 12.00, stage/main/post_update 10.30, render/graph/ms 9.65 |
| warm-up | 4.49 | 98.3 | pipelines/compile_ms 1455, pipelines/front_ms 1414, main/busy/ms 149, stage/main/update 130, render/thread/ms 119, render/busy/ms 117, stage/render/prepare_assets 60.29, pipelines/driver_ms 40.93, stage/render/render 25.67, stage/render/prepare_flush 24.54, render/graph/ms 23.73, stage/render/camera_driver 17.80 |
| warm-up | 4.63 | 138.3 | pipelines/compile_ms 1619, pipelines/front_ms 1563, main/busy/ms 163, stage/main/update 142, render/thread/ms 131, render/busy/ms 129, stage/render/prepare_assets 64.61, pipelines/driver_ms 55.85, stage/render/render 31.50, render/graph/ms 29.36, stage/render/prepare_flush 24.65, stage/render/camera_driver 17.81 |
| warm-up | 4.68 | 54.1 | pipelines/compile_ms 2027, pipelines/front_ms 1957, render/thread/ms 96.60, render/busy/ms 94.23, main/busy/ms 80.01, pipelines/driver_ms 69.54, stage/render/prepare_assets 49.35, stage/main/update 38.43, stage/render/render 30.12, render/graph/ms 27.12, stage/render/graph_submit 24.06, encode/finish_wall 18.61 |
| warm-up | 4.75 | 63.0 | pipelines/compile_ms 991, pipelines/front_ms 938, main/busy/ms 94.98, render/thread/ms 89.99, render/busy/ms 87.81, pipelines/wait_ms 68.18, pipelines/driver_ms 52.49, stage/main/update 38.65, stage/render/render 25.81, render/graph/ms 22.96, stage/render/prepare_assets 20.95, stage/render/graph_submit 20.00 |
| warm-up | 4.83 | 87.6 | pipelines/compile_ms 1406, pipelines/front_ms 1341, pipelines/wait_ms 139, render/thread/ms 124, render/busy/ms 122, main/busy/ms 88.07, pipelines/driver_ms 65.56, stage/main/update 43.23, stage/render/prepare_flush 40.38, stage/main/pre_update 24.89, stage/render/prepare_assets 24.06, stage/main/post_update 14.40 |
| warm-up | 4.88 | 51.9 | pipelines/compile_ms 1882, pipelines/front_ms 1828, pipelines/wait_ms 128, render/thread/ms 118, render/busy/ms 116, main/busy/ms 84.95, pipelines/driver_ms 54.08, stage/main/update 41.33, stage/render/prepare_flush 38.80, stage/render/render 30.37, render/graph/ms 27.83, stage/main/pre_update 20.58 |
| warm-up | 4.97 | 84.5 | pipelines/compile_ms 2103, pipelines/front_ms 2047, pipelines/wait_ms 167, render/thread/ms 104, render/busy/ms 101, main/busy/ms 91.74, pipelines/driver_ms 56.40, stage/main/update 37.77, stage/render/render 35.72, stage/render/prepare_assets 32.72, render/graph/ms 32.66, stage/main/post_update 27.71 |
| warm-up | 5.05 | 79.9 | pipelines/compile_ms 1388, pipelines/front_ms 1342, pipelines/wait_ms 151, render/thread/ms 128, render/busy/ms 124, main/busy/ms 116, stage/main/update 61.85, stage/render/prepare_assets 53.05, pipelines/driver_ms 46.03, stage/main/post_update 37.75, stage/render/render 29.43, render/graph/ms 26.02 |
| warm-up | 5.20 | 155.5 | pipelines/compile_ms 2598, pipelines/front_ms 2476, render/thread/ms 182, render/busy/ms 178, main/busy/ms 150, pipelines/driver_ms 122, pipelines/wait_ms 111, stage/main/update 110, stage/render/prepare_assets 63.39, stream/drive/ms 62.22, stage/render/render 46.90, stage/render/create_views 45.01 |
| warm-up | 5.26 | 59.8 | pipelines/compile_ms 2497, pipelines/front_ms 2385, render/thread/ms 168, render/busy/ms 166, pipelines/driver_ms 112, encode//sum 106, main/busy/ms 102, pipelines/wait_ms 84.43, stage/main/update 73.77, stream/drive/ms 61.98, stage/render/render 58.96, render/graph/ms 55.96 |
| warm-up | 5.30 | 33.8 | pipelines/compile_ms 236, pipelines/front_ms 229, encode//sum 89.14, render/thread/ms 84.95, render/busy/ms 82.91, main/busy/ms 55.25, stage/render/render 50.60, render/graph/ms 47.90, stage/render/graph_submit 38.81, stage/main/update 33.97, encode/finish_wall 20.29, encode/submit 18.63 |
| warm-up | 5.33 | 36.2 | pipelines/compile_ms 61.62, pipelines/front_ms 59.20, render/thread/ms 56.16, main/busy/ms 55.43, render/busy/ms 54.72, stage/main/update 38.39, stage/render/render 26.28, render/graph/ms 24.12, stage/render/graph_submit 19.33, encode//sum 14.25, stage/render/prepare_assets 11.95, stream/drive/ms 11.12 |
| warm-up | 5.42 | 29.5 | pipelines/compile_ms 1006, pipelines/front_ms 979, render/thread/ms 49.45, render/busy/ms 47.95, main/busy/ms 39.33, pipelines/driver_ms 26.71, stage/main/update 20.89, stage/render/render 19.43, render/graph/ms 17.41, encode//sum 14.24, stage/render/graph_submit 11.69, stage/main/post_update 11.19 |
| warm-up | 5.44 | 28.1 | pipelines/compile_ms 958, pipelines/front_ms 928, render/thread/ms 51.22, render/busy/ms 49.72, main/busy/ms 37.25, pipelines/driver_ms 29.29, stage/main/update 19.94, stage/render/render 18.25, render/graph/ms 16.27, encode//sum 11.74, stage/render/graph_submit 10.79, stage/main/post_update 10.46 |
| warm-up | 5.47 | 28.0 | pipelines/compile_ms 742, pipelines/front_ms 718, render/thread/ms 45.52, render/busy/ms 44.21, main/busy/ms 31.49, pipelines/driver_ms 23.82, stage/main/update 16.37, stage/render/render 15.81, render/graph/ms 13.98, stage/render/prepare_assets 12.83, encode//sum 10.78, stage/render/graph_submit 10.13 |
| warm-up | 5.57 | 31.1 | pipelines/compile_ms 82.37, pipelines/front_ms 78.75, render/thread/ms 49.54, render/busy/ms 48.12, main/busy/ms 39.98, stage/main/update 23.76, stage/render/render 15.89, render/graph/ms 13.84, stream/drive/ms 12.94, encode//sum 11.40, stage/main/post_update 9.84, stage/render/graph_submit 9.43 |
| warm-up | 5.94 | 45.8 | pipelines/compile_ms 70.79, pipelines/front_ms 65.80, render/thread/ms 63.68, render/busy/ms 45.58, stage/render/render 39.27, main/busy/ms 22.42, render/graph/ms 20.52, render/present/ms 17.87, stage/render/graph_submit 13.64, encode//sum 11.69, stage/render/prepare_assets 9.45, stage/main/update 9.22 |
| warm-up | 6.15 | 27.5 | pipelines/compile_ms 248, pipelines/front_ms 241, render/thread/ms 42.95, render/busy/ms 40.96, main/busy/ms 40.22, stage/render/render 24.15, render/graph/ms 21.64, stage/main/post_update 18.92, stage/main/update 14.30, stage/render/graph_submit 13.53, encode//sum 12.69, stage/render/camera_driver 7.94 |
| warm-up | 6.22 | 35.7 | pipelines/compile_ms 206, pipelines/front_ms 201, main/busy/ms 44.67, render/thread/ms 33.12, render/busy/ms 31.53, stage/main/update 28.47, stream/drive/ms 20.23, stage/render/render 14.95, render/graph/ms 12.54, encode//sum 12.07, stage/main/post_update 8.95, stage/render/graph_submit 7.93 |
| warm-up | 6.34 | 29.4 | pipelines/compile_ms 723, pipelines/front_ms 702, render/thread/ms 42.94, render/busy/ms 41.00, main/busy/ms 39.60, stage/render/render 21.41, pipelines/driver_ms 20.67, render/graph/ms 18.58, stage/main/post_update 17.30, stage/main/update 16.85, encode//sum 14.66, stage/render/graph_submit 12.33 |
| warm-up | 6.37 | 29.0 | pipelines/compile_ms 367, pipelines/front_ms 356, render/thread/ms 48.89, render/busy/ms 47.12, main/busy/ms 39.11, stage/render/prepare_assets 17.97, stage/render/render 17.41, stage/main/update 16.31, stage/main/post_update 15.10, render/graph/ms 14.51, encode//sum 14.13, pipelines/driver_ms 10.81 |
| warm-up | 6.41 | 35.5 | render/thread/ms 47.30, render/busy/ms 46.21, main/busy/ms 32.49, stage/render/prepare_assets 25.10, stage/main/update 13.88, stage/render/render 11.37, stage/main/post_update 10.95, encode//sum 10.60, render/graph/ms 9.33, stage/render/graph_submit 6.23, stream/drive/ms 3.73, stage/main/fixed_main_loop 3.54 |
| warm-up | 6.49 | 27.6 | pipelines/compile_ms 314, pipelines/front_ms 304, render/thread/ms 43.39, render/busy/ms 42.21, main/busy/ms 40.52, stage/main/update 24.94, stage/render/render 14.41, render/graph/ms 12.73, encode//sum 11.28, stage/main/post_update 10.93, pipelines/driver_ms 9.91, stage/render/prepare_assets 8.68 |
| warm-up | 6.52 | 28.4 | pipelines/compile_ms 160, pipelines/front_ms 156, render/thread/ms 45.66, render/busy/ms 44.56, main/busy/ms 38.61, stage/main/update 24.41, stage/render/prepare_assets 15.80, stage/render/render 12.99, render/graph/ms 11.51, encode//sum 9.87, stage/main/post_update 8.09, stage/render/graph_submit 7.47 |
| warm-up | 6.64 | 33.2 | render/thread/ms 51.67, render/busy/ms 50.48, main/busy/ms 43.92, stage/main/update 24.06, stage/render/render 16.85, encode//sum 15.24, render/graph/ms 15.15, stage/main/post_update 14.62, stage/render/prepare_assets 9.89, stage/render/graph_submit 9.73, stage/render/prepare_meshes 7.83, encode/submit 5.98 |
| warm-up | 6.70 | 38.4 | render/thread/ms 50.59, render/busy/ms 49.62, main/busy/ms 41.65, stage/main/update 21.82, stage/render/prepare_assets 18.53, stage/main/post_update 12.08, encode//sum 11.82, stage/render/render 10.94, render/graph/ms 9.46, stage/render/prepare_meshes 7.75, stage/render/graph_submit 5.84, stream/drive/ms 5.80 |
| warm-up | 6.74 | 38.0 | render/thread/ms 65.72, render/busy/ms 64.27, main/busy/ms 49.62, stage/main/update 25.75, stage/render/prepare_assets 20.97, stage/render/render 15.92, stage/main/post_update 15.81, encode//sum 14.35, render/graph/ms 13.62, stage/render/prepare_meshes 10.41, stage/render/graph_submit 8.40, stream/drive/ms 5.28 |
| warm-up | 6.79 | 29.3 | pipelines/compile_ms 182, pipelines/front_ms 175, render/thread/ms 37.57, render/busy/ms 36.43, main/busy/ms 32.81, stage/main/update 16.30, stage/render/prepare_assets 14.44, stage/render/render 11.87, render/graph/ms 10.37, encode//sum 9.50, stage/main/post_update 9.47, pipelines/driver_ms 7.13 |
| warm-up | 6.96 | 48.5 | pipelines/compile_ms 145, pipelines/front_ms 140, main/busy/ms 60.68, render/thread/ms 42.56, stage/main/update 41.54, render/busy/ms 41.05, stage/render/render 16.95, stream/drive/ms 16.50, render/graph/ms 14.85, encode//sum 12.95, stage/main/post_update 12.86, stage/render/graph_submit 10.41 |
| warm-up | 6.99 | 32.9 | pipelines/compile_ms 173, pipelines/front_ms 167, render/thread/ms 49.49, render/busy/ms 48.11, main/busy/ms 43.24, stage/main/update 20.99, stage/render/prepare_assets 19.14, stage/render/render 15.33, render/graph/ms 13.37, encode//sum 12.09, stage/main/post_update 11.50, stage/render/graph_submit 9.12 |
| measure | 1.70 | 28.4 | render/thread/ms 36.15, render/busy/ms 35.17, main/busy/ms 22.95, encode//sum 19.07, stage/render/render 16.59, render/graph/ms 14.84, stage/render/graph_submit 9.69, stage/main/update 9.67, stage/main/post_update 8.96, stage/render/prepare_assets 6.56, encode/submit 5.79, stage/render/camera_driver 5.05 |
| measure | 2.72 | 28.7 | render/thread/ms 37.97, render/busy/ms 36.94, encode//sum 24.12, stage/render/render 19.50, main/busy/ms 18.35, render/graph/ms 17.97, stage/render/graph_submit 12.08, stage/main/post_update 10.46, stage/render/create_views 8.11, encode/submit 7.82, stage/render/camera_driver 5.78, stage/main/update 4.50 |
| measure | 18.42 | 34.1 | main/busy/ms 41.18, stage/main/update 26.26, render/thread/ms 20.89, render/busy/ms 19.93, stage/main/post_update 10.95, stage/render/render 10.34, render/graph/ms 9.00, encode//sum 7.24, stage/render/graph_submit 6.30, encode/submit 4.18, stage/render/prepare_assets 3.89, stage/render/camera_driver 2.57 |
| measure | 31.18 | 30.3 | render/thread/ms 41.47, stage/render/render 32.33, render/busy/ms 24.52, main/busy/ms 22.47, render/present/ms 16.78, render/graph/ms 14.77, stage/main/post_update 14.12, encode//sum 13.62, stage/render/graph_submit 10.55, encode/submit 6.70, stage/main/update 4.22, stage/render/camera_driver 4.11 |
| measure | 59.21 | 35.4 | render/thread/ms 39.02, render/busy/ms 37.94, main/busy/ms 25.45, stage/render/prepare_assets 18.08, stage/main/post_update 12.57, stage/render/render 12.13, encode//sum 11.04, render/graph/ms 10.52, stage/render/graph_submit 6.59, stage/main/fixed_main_loop 4.40, stage/main/update 4.12, encode/submit 3.90 |
| measure | 61.20 | 28.0 | render/thread/ms 36.35, stage/render/render 27.73, render/busy/ms 19.83, main/busy/ms 18.87, render/present/ms 16.33, render/graph/ms 10.92, encode//sum 10.46, stage/main/post_update 9.75, stage/render/graph_submit 7.79, encode/submit 4.43, stage/main/update 4.31, encode/finish_wall 3.36 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\balmora_walk_night@2560x1440_arm=base-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1449 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **78%** (min 56%) · 2625 MHz (min 2625) · memory 8101 MHz · 76.1 W · 74 °C (max 79) · VRAM 6261 MiB · throttled 100% of samples

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
- `sf.graphics_preset=high (preset: level requested)`
- `sg.effects=high (preset: level requested)`
- `sg.lighting=high (preset: level requested)`
- `sg.post=high (preset: level requested)`
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
