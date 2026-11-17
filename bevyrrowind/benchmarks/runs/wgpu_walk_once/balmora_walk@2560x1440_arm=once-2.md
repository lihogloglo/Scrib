# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); render.wgpu.bind_group_walk_once=on (command line: --set); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5846 frames over 65.6 s · **median 10.94 ms (91 fps)** · mean 11.23 · p95 13.88 · p99 17.65 (1% low 57 fps) · worst 49.52 ms · 74 spikes over 1.5× median · 55.8 stutters/min (work 53.0/min)
- warm-up: 325 frames over 15.1 s, worst 2596.1 ms
- hitches (> 27.4 ms): **37** measured, 143 in warm-up
- timed GPU passes: 6.78 ms of a 11.23 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.94 ms, limit 11 ms
- **FAIL** frame_ms: 61 stutters (> 1.5x moving median of 60) in 1.09 min = 55.75/min, limit 1/min
- **FAIL** worst frame 49.52 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.926 | 2.035 | 2.066 | 0.95 | 0.181 |
| main_opaque_pass_3d | 1.549 | 1.636 | 2.034 | 0.95 | 0.076 |
| early prepass | 0.473 | 0.499 | 0.732 | 0.95 | 0.092 |
| bloom | 0.298 | 0.315 | 0.321 | 0.95 | 0.066 |
| atmosphere_luts | 0.288 | 0.304 | 0.367 | 0.95 | 0.004 |
| water_velocity | 0.253 | 0.267 | 0.347 | 0.95 | 0.002 |
| water_sspr | 0.191 | 0.202 | 0.203 | 0.95 | 0.002 |
| ssao | 0.179 | 0.189 | 0.193 | 0.95 | 0.008 |
| atmos_froxel_inject | 0.138 | 0.146 | 0.147 | 0.95 | 0.002 |
| clouds_trace | 0.133 | 0.141 | 0.231 | 0.95 | 0.001 |
| uniform_allocation | 0.124 | 0.131 | 0.180 | 0.95 | 0.028 |
| water_fft | 0.123 | 0.130 | 0.138 | 0.95 | 0.003 |
| upscaling | 0.111 | 0.118 | 0.124 | 0.95 | 0.004 |
| auxiliary_views | 0.108 | 0.114 | 0.434 | 0.95 | 0.124 |
| bin_unpacking | 0.100 | 0.106 | 0.142 | 0.95 | 0.020 |
| clouds_composite | 0.100 | 0.106 | 0.109 | 0.95 | 0.002 |
| tonemapping | 0.080 | 0.085 | 0.088 | 0.95 | 0.004 |
| main_transmissive_pass_3d | 0.077 | 0.081 | 0.160 | 0.95 | 0.009 |
| clustering | 0.061 | 0.064 | 0.068 | 0.95 | 0.019 |
| render_sky | 0.049 | 0.051 | 0.052 | 0.95 | 0.002 |
| clouds_resolve | 0.046 | 0.048 | 0.049 | 0.95 | 0.001 |
| lightprobe_irradiance_map | 0.044 | 0.413 | 0.416 | 0.11 | 0.001 |
| surface_exposure | 0.042 | 0.044 | 0.045 | 0.95 | 0.001 |
| surface_trail | 0.038 | 0.056 | 0.057 | 0.68 | 0.003 |
| surface_exposure_far | 0.036 | 0.043 | 0.045 | 0.84 | 0.002 |
| early_mesh_preprocessing | 0.030 | 0.031 | 0.047 | 0.95 | 0.005 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.95 | 0.004 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.031 | 0.95 | 0.001 |
| lightprobe_radiance_map | 0.023 | 0.219 | 0.229 | 0.11 | 0.005 |
| water_snapshot | 0.023 | 0.024 | 0.027 | 0.95 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.95 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.95 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.024 | 0.95 | 0.124 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.95 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.95 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.95 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.94 | 0.003 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/compile_ms | 14.449 | 0.000 | 5954.95 | 84471.6 |
| pipelines/driver_ms | 12.550 | 0.000 | 5521.29 | 73368.7 |
| render/thread/ms | 9.729 | 11.994 | 48.29 | 56875.3 |
| main/busy/ms | 9.310 | 11.099 | 33.11 | 54425.8 |
| render/busy/ms | 9.226 | 11.523 | 47.68 | 53934.9 |
| pipelines/wait_ms | 7.598 | 0.000 | 8687.75 | 44417.3 |
| encode//sum | 7.338 | 10.042 | 100.96 | 42899.1 |
| stage/main/post_update | 5.750 | 7.167 | 29.25 | 33612.6 |
| stage/render/render | 5.579 | 7.002 | 38.62 | 32615.1 |
| render/graph/ms | 4.880 | 6.277 | 37.61 | 28528.2 |
| stage/render/graph_submit | 3.662 | 4.783 | 15.80 | 21410.3 |
| encode/submit | 2.323 | 3.184 | 6.35 | 13579.4 |
| stage/main/update | 2.246 | 2.734 | 23.01 | 13127.6 |
| pipelines/front_ms | 1.899 | 0.000 | 433.66 | 11102.9 |
| encode/finish_wall | 1.395 | 2.070 | 10.67 | 8154.6 |
| stage/render/camera_driver | 1.162 | 1.600 | 21.73 | 6793.3 |
| stage/render/prepare_write_phase_buffers | 0.762 | 1.105 | 2.99 | 4451.9 |
| stage/render/prepare_assets | 0.595 | 0.770 | 11.02 | 3477.2 |
| bigspace/total/ms | 0.543 | 1.330 | 3.70 | 3171.5 |
| stage/render/prepare_meshes | 0.465 | 0.592 | 9.79 | 2718.9 |
| stage/render/prepare_resources | 0.456 | 0.554 | 1.62 | 2668.0 |
| stage/render/specialize | 0.444 | 0.596 | 15.60 | 2594.3 |
| stage/main/first | 0.418 | 0.549 | 5.85 | 2444.2 |
| render/present/ms | 0.405 | 0.555 | 2.42 | 2369.9 |
| stage/main/pre_update | 0.372 | 0.537 | 1.77 | 2173.9 |
| terrain/update/ms | 0.307 | 0.392 | 1.84 | 1793.1 |
| stage/render/queue | 0.305 | 0.432 | 7.70 | 1782.0 |
| stage/render/prepare_bind_groups | 0.293 | 0.533 | 3.09 | 1715.0 |
| controller/feed/ms | 0.273 | 0.360 | 0.84 | 1594.2 |
| terrain/select/ms | 0.272 | 0.347 | 0.97 | 1590.3 |
| stream/drive/ms | 0.259 | 0.365 | 20.85 | 1515.9 |
| stage/main/fixed_main_loop | 0.224 | 0.378 | 1.22 | 1308.0 |
| bigspace/low_precision/ms | 0.222 | 0.679 | 3.23 | 1297.4 |
| far/select/ms | 0.209 | 0.296 | 0.58 | 1223.3 |
| stage/render/create_views | 0.206 | 0.271 | 1.47 | 1202.3 |
| stream/tick/ms | 0.200 | 0.282 | 0.56 | 1168.8 |
| stage/render/prepare_batch_phases | 0.198 | 0.284 | 0.68 | 1157.3 |
| stage/render/extract_commands | 0.181 | 0.216 | 0.39 | 1059.7 |
| stage/render/prepare_flush | 0.170 | 0.343 | 1.06 | 993.5 |
| anim/evaluate/ms | 0.165 | 0.312 | 1.58 | 964.0 |
| bigspace/high_precision/ms | 0.161 | 0.641 | 1.64 | 942.8 |
| anim/evaluate_cpu/ms | 0.152 | 0.197 | 0.80 | 888.9 |
| stage/render/prepare_views | 0.142 | 0.180 | 0.41 | 829.7 |
| controller/move/ms | 0.139 | 0.278 | 0.67 | 810.3 |
| stage/main/last | 0.122 | 0.191 | 1.96 | 712.6 |
| render/acquire/ms | 0.098 | 0.125 | 0.31 | 570.5 |
| stage/render/prepare_collect_phase_buffers | 0.068 | 0.140 | 0.43 | 394.8 |
| physics/step/ms | 0.049 | 0.090 | 0.58 | 289.1 |
| stage/render/cleanup | 0.036 | 0.109 | 5.83 | 210.2 |
| encode/resolve_diagnostics/sum | 0.036 | 0.049 | 0.73 | 209.7 |
| physics/insert/ms | 0.034 | 0.376 | 1.01 | 198.8 |
| stage/render/phase_sort | 0.027 | 0.047 | 0.45 | 158.3 |
| weather/map/ms | 0.026 | 0.034 | 0.22 | 152.8 |
| encode/camera_driver_span_begin/sum | 0.017 | 0.024 | 0.57 | 97.5 |
| encode/camera_driver_span_end/sum | 0.012 | 0.021 | 0.56 | 71.5 |
| anim/apply/ms | 0.012 | 0.022 | 0.52 | 70.5 |
| controller/avoid/ms | 0.011 | 0.015 | 0.05 | 65.5 |
| weather/eval/ms | 0.011 | 0.015 | 0.08 | 64.1 |
| script/update/ms | 0.007 | 0.009 | 0.19 | 43.7 |
| stream/despawn/ms | 0.005 | 0.000 | 1.43 | 31.4 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 30.3 |
| script/ui/ms | 0.005 | 0.007 | 0.34 | 29.6 |
| script/input/ms | 0.005 | 0.007 | 0.05 | 28.0 |
| controller/write/ms | 0.004 | 0.005 | 0.21 | 20.5 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.68 | 14.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 4.8 |
| stage/render/prepare | 0.001 | 0.002 | 0.20 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.11 | 0.2 |
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
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 26 | 41 | 12 |
| crowd/c0/joints/count | count | 1252 | 2160 | 547 |
| crowd/c0/meshes/count | count | 350 | 621 | 140 |
| crowd/c1/count | count | 147 | 158 | 149 |
| crowd/c1/joints/count | count | 5531 | 6018 | 5649 |
| crowd/c1/meshes/count | count | 1185 | 1297 | 1201 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4619 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4597 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.01 | 7.13 | 0.87 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.57 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.56 | 0.02 |
| encode/finish_critical | ms | 1.01 | 7.13 | 0.87 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.73 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.62 | 47.68 | 8.39 |
| gpu/timed/ms | ms | 7.87 | 9.97 | 8.21 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 803 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 803 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1285 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3379 | 5130 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1285 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3379 | 5130 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 15 | 200 | 4 |
| phases/shadow/bins | count | 5014 | 8709 | 2405 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.3 k | 26.3 k | 6352 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 201 | 250 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 186 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 262 | 330 | 330 |
| pipelines/waiting | count | 1 | 7 | 0 |
| precache/draw_waiting | count | 0 | 5 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 1 | 2 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 33 | 77 | 0 |
| preprocess/bind_groups | count | 12 | 557 | 6 |
| preprocess/bind_groups_created | count | 12 | 557 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 128 | 210 | 123 |
| process/cpu_usage | % | 32.58 | 75 | 24.52 |
| process/mem_usage | GiB | 5.64 | 5.91 | 5.91 |
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
| skin/blocks_changed/count | count | 14 | 131 | 13 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 372.2 k | 812.3 k | 488.7 k |
| skin/joints_written/count | count | 5816 | 12.7 k | 7636 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4918 | 5012 | 5012 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 38.50 | 82.89 | 30.43 |
| system/mem_usage | % | 67.26 | 68.03 | 67.99 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 63 | 247 | 54 |
| views/all/bins | count | 7540 | 11.6 k | 5903 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.3 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 246 | 297 | 75 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.6 k | 35.5 k | 8119 |
| views/cascade/batch_sets | count | 9 | 49 | 4 |
| views/cascade/bins | count | 2797 | 6158 | 2405 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8894 | 23.5 k | 6352 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 399 | 22.5 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 191 | 0 |
| views/lamp_face/bins | count | 2139 | 2718 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2396 | 3014 | 0 |
| views/lamp_face/pipelines | count | 165 | 202 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2855 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2864 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8365 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8534 | 13.2 k | 7918 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 55.09 | 305 | 322.1 k |
| anim/events/count | count | 0.10 | 3 | 573 |
| assets/Image/added | count | 0.10 | 36 | 592 |
| assets/Image/modified | count | 0.00 | 3 | 17 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.69 | 152 | 4061 |
| assets/Mesh/removed | count | 0.83 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.70 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 11.19 | 1321 | 65.4 k |
| ecs/entities_delta | count | 0.80 | 1272 | 4674 |
| encode//count | count | 38.32 | 273 | 224.0 k |
| encode/bind_walk/init_textures | count | 11.8 k | 114.7 k | 69.19 M |
| encode/bind_walk/init_textures_skipped | count | 70.9 k | 143.8 k | 414.53 M |
| encode/bind_walk/inits | count | 453 | 1122 | 2.65 M |
| encode/bind_walk/inits_skipped | count | 113 | 161 | 658.0 k |
| encode/bind_walk/merge_textures | count | 11.8 k | 114.6 k | 68.77 M |
| encode/bind_walk/merge_textures_skipped | count | 5376 | 6012 | 31.43 M |
| encode/bind_walk/merges | count | 96.69 | 335 | 565.3 k |
| encode/bind_walk/merges_skipped | count | 86.11 | 131 | 503.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5846 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5846 |
| encode/queue/command_buffers | count | 43.34 | 278 | 253.3 k |
| encode/queue/submits | count | 2.02 | 4 | 11.8 k |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 4.88 M | 11.03 G |
| encode/queue/write_buffer/calls | count | 252 | 832 | 1.48 M |
| encode/queue/write_buffer_with/bytes | bytes | 178.3 k | 3.52 M | 1.04 G |
| encode/queue/write_buffer_with/calls | count | 11.07 | 465 | 64.7 k |
| encode/queue/write_texture/bytes | bytes | 10.9 k | 8.39 M | 63.53 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5846 |
| physics/inserts | count | 0.39 | 110 | 2284 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 129 | 2335 |
| physics/steps | count | 0.67 | 3 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 19 |
| pipelines/created | count | 0.02 | 2 | 128 |
| precache/hit | count | 0.00 | 2 | 3 |
| precache/missed | count | 0.00 | 2 | 7 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5791 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.01 | 1 | 30 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1270 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.4 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.60 | 2596.1 | main/busy/ms 2565, stage/main/post_update 2459, stage/main/update 97.01, surface/advance_cpu_ms 67.77, stage/main/pre_update 4.98, stage/main/last 1.34, surface/cell_upload_ms 0.79, stage/main/first 0.61, stage/main/fixed_main_loop 0.60, stream/drive/ms 0.37, stream/tick/ms 0.14, weather/map/ms 0.12 |
| warm-up | 3.32 | 723.3 | main/busy/ms 2708, stage/main/post_update 2484, render/thread/ms 684, render/busy/ms 538, stage/render/prepare_assets 485, stage/main/update 208, stage/render/prepare_views 143, render/acquire/ms 143, surface/advance_cpu_ms 67.77, stage/render/create_views 27.97, stage/render/render 13.96, render/graph/ms 9.43 |
| warm-up | 3.47 | 146.4 | render/thread/ms 806, render/busy/ms 604, stage/render/prepare_assets 515, pipelines/compile_ms 489, pipelines/driver_ms 376, main/busy/ms 182, stage/render/prepare_views 144, render/acquire/ms 143, stage/main/update 126, pipelines/front_ms 113, stage/render/render 90.68, render/present/ms 58.57 |
| warm-up | 3.58 | 103.2 | pipelines/compile_ms 528, pipelines/driver_ms 417, pipelines/front_ms 111, render/thread/ms 85.47, render/busy/ms 83.19, main/busy/ms 71.94, stage/main/post_update 40.15, stage/main/update 25.80, stage/render/prepare_assets 25.33, pipelines/wait_ms 23.41, stage/render/prepare_batch_phases 20.70, stage/render/render 14.55 |
| warm-up | 3.66 | 77.7 | pipelines/wait_ms 471, pipelines/compile_ms 456, pipelines/driver_ms 325, render/thread/ms 150, render/busy/ms 148, pipelines/front_ms 132, main/busy/ms 65.24, stage/render/prepare_resources 50.34, stage/main/post_update 34.52, stage/render/prepare_assets 29.39, stage/main/update 24.78, stage/render/prepare_batch_phases 23.86 |
| warm-up | 3.85 | 139.0 | pipelines/wait_ms 1637, pipelines/compile_ms 1471, pipelines/driver_ms 1305, pipelines/front_ms 166, render/thread/ms 142, render/busy/ms 139, main/busy/ms 137, stage/main/update 85.22, stage/render/prepare_assets 84.59, stage/render/prepare_resources 29.19, stage/main/post_update 27.14, stage/main/fixed_main_loop 18.23 |
| warm-up | 3.99 | 66.4 | pipelines/wait_ms 1088, pipelines/compile_ms 357, pipelines/driver_ms 301, render/thread/ms 93.45, render/busy/ms 91.52, pipelines/front_ms 56.11, stage/render/prepare_resources 26.70, main/busy/ms 26.59, stage/render/prepare_batch_phases 18.57, stage/render/prepare_assets 16.07, stage/main/post_update 15.42, stage/render/prepare_flush 13.92 |
| warm-up | 4.05 | 62.9 | pipelines/wait_ms 3547, pipelines/compile_ms 611, pipelines/driver_ms 487, render/thread/ms 127, render/busy/ms 125, pipelines/front_ms 124, main/busy/ms 79.01, stage/main/post_update 64.85, stage/render/prepare_resources 39.91, stage/render/prepare_batch_phases 28.24, stage/render/prepare_write_phase_buffers 16.88, stage/render/prepare_bind_groups 16.20 |
| warm-up | 4.16 | 110.1 | pipelines/wait_ms 3402, pipelines/compile_ms 457, pipelines/driver_ms 366, render/thread/ms 164, render/busy/ms 162, pipelines/front_ms 90.46, main/busy/ms 90.39, stage/main/post_update 62.93, stage/render/prepare_assets 48.84, stage/render/prepare_bind_groups 33.95, stage/render/prepare_resources 32.80, stage/main/update 19.92 |
| warm-up | 4.32 | 110.6 | pipelines/wait_ms 2725, pipelines/compile_ms 1012, pipelines/driver_ms 826, pipelines/front_ms 186, render/thread/ms 111, render/busy/ms 110, main/busy/ms 93.72, stage/main/update 50.52, stream/drive/ms 46.27, stage/render/prepare_assets 44.94, stage/main/post_update 35.15, stage/render/prepare_resources 27.29 |
| warm-up | 4.51 | 90.0 | pipelines/wait_ms 4754, pipelines/compile_ms 1361, pipelines/driver_ms 1155, pipelines/front_ms 206, render/thread/ms 133, render/busy/ms 132, stage/render/prepare_assets 65.77, main/busy/ms 42.57, stage/render/prepare_batch_phases 30.77, stage/main/update 24.65, stage/main/post_update 13.07, stage/render/render 12.91 |
| warm-up | 4.59 | 82.0 | pipelines/wait_ms 4927, pipelines/compile_ms 518, pipelines/driver_ms 395, render/thread/ms 154, render/busy/ms 153, pipelines/front_ms 122, stage/render/prepare_assets 64.07, main/busy/ms 59.16, stage/render/prepare_batch_phases 31.13, stage/main/update 26.45, stage/main/post_update 23.45, stage/render/render 21.20 |
| warm-up | 4.73 | 112.1 | pipelines/compile_ms 1413, pipelines/driver_ms 1293, pipelines/wait_ms 1105, pipelines/front_ms 120, render/thread/ms 87.09, main/busy/ms 85.75, render/busy/ms 85.38, stage/main/update 50.23, stage/render/prepare_assets 39.65, stage/main/post_update 20.90, stage/render/prepare_resources 18.87, stage/main/fixed_main_loop 10.36 |
| warm-up | 4.84 | 103.0 | pipelines/wait_ms 1853, pipelines/compile_ms 1492, pipelines/driver_ms 1341, render/thread/ms 151, pipelines/front_ms 151, render/busy/ms 150, main/busy/ms 70.96, stage/render/prepare_resources 51.06, stage/render/prepare_assets 41.70, stage/main/post_update 36.24, stage/main/update 21.56, stage/render/prepare_batch_phases 21.20 |
| warm-up | 4.89 | 58.4 | pipelines/wait_ms 1989, pipelines/compile_ms 225, render/thread/ms 151, render/busy/ms 149, pipelines/driver_ms 116, pipelines/front_ms 109, main/busy/ms 87.03, stage/main/post_update 55.66, stage/render/prepare_assets 51.54, stage/render/prepare_resources 34.78, stage/main/update 21.99, stage/render/prepare_batch_phases 20.94 |
| warm-up | 5.01 | 110.8 | pipelines/wait_ms 1242, render/thread/ms 157, render/busy/ms 154, pipelines/compile_ms 146, stage/render/prepare_assets 85.59, pipelines/front_ms 77.27, main/busy/ms 69.46, pipelines/driver_ms 68.88, stage/render/render 29.51, stage/main/post_update 28.77, stage/main/update 26.58, render/graph/ms 26.47 |
| warm-up | 5.10 | 95.3 | render/thread/ms 182, render/busy/ms 179, stage/render/prepare_assets 86.56, main/busy/ms 81.35, stage/main/post_update 35.03, stage/main/update 34.73, stage/render/prepare_resources 33.37, stage/render/render 31.10, encode//sum 30.38, stream/drive/ms 29.36, render/graph/ms 28.11, stage/render/camera_driver 17.69 |
| warm-up | 5.21 | 107.0 | render/thread/ms 173, render/busy/ms 171, main/busy/ms 107, stage/main/post_update 61.41, stage/render/prepare_assets 52.92, stage/render/prepare_resources 45.91, stage/main/update 35.79, stream/drive/ms 29.40, stage/render/prepare_write_phase_buffers 24.72, encode//sum 17.48, stage/render/render 14.69, bigspace/total/ms 14.03 |
| warm-up | 5.34 | 127.8 | render/thread/ms 197, render/busy/ms 196, main/busy/ms 186, stage/main/update 118, stage/main/post_update 54.36, stream/drive/ms 53.53, stage/render/prepare_assets 48.09, stage/render/prepare_write_phase_buffers 45.00, stage/render/prepare_resources 37.10, stage/render/prepare_flush 31.03, stage/render/prepare_batch_phases 13.45, stage/render/prepare_bind_groups 9.92 |
| warm-up | 5.50 | 165.3 | pipelines/compile_ms 1270, pipelines/driver_ms 1196, pipelines/wait_ms 726, render/thread/ms 228, render/busy/ms 227, main/busy/ms 179, stage/main/update 113, stage/render/prepare_assets 103, pipelines/front_ms 74.67, stream/drive/ms 45.76, stage/render/prepare_write_phase_buffers 38.69, stage/render/prepare_flush 33.48 |
| warm-up | 5.59 | 86.6 | pipelines/wait_ms 6837, pipelines/compile_ms 1354, pipelines/driver_ms 1258, render/thread/ms 206, render/busy/ms 204, main/busy/ms 117, pipelines/front_ms 95.72, stage/render/prepare_assets 83.90, stage/main/update 69.13, stream/drive/ms 55.74, stage/render/prepare_write_phase_buffers 51.45, stage/render/prepare_flush 15.72 |
| warm-up | 5.66 | 74.0 | pipelines/wait_ms 6111, main/busy/ms 159, render/thread/ms 145, render/busy/ms 143, stage/main/update 130, stream/drive/ms 107, pipelines/compile_ms 83.43, pipelines/driver_ms 62.39, stage/render/prepare_write_phase_buffers 40.91, stage/render/prepare_assets 31.09, stage/render/prepare_flush 28.52, pipelines/front_ms 21.05 |
| warm-up | 5.81 | 143.6 | pipelines/wait_ms 4278, pipelines/compile_ms 200, main/busy/ms 172, render/thread/ms 153, render/busy/ms 152, pipelines/driver_ms 148, stage/main/update 98.21, stream/drive/ms 66.80, pipelines/front_ms 52.36, stage/render/prepare_assets 49.85, stage/main/post_update 39.64, stage/render/prepare_resources 32.12 |
| warm-up | 5.94 | 136.4 | pipelines/wait_ms 12.4 k, pipelines/compile_ms 1631, pipelines/driver_ms 1517, render/thread/ms 216, render/busy/ms 214, main/busy/ms 188, pipelines/front_ms 114, stage/main/update 112, stage/render/prepare_resources 65.45, stage/render/prepare_assets 46.69, stream/drive/ms 46.15, stage/render/prepare_batch_phases 43.47 |
| warm-up | 6.05 | 107.4 | pipelines/wait_ms 8147, pipelines/compile_ms 1431, pipelines/driver_ms 1370, render/thread/ms 202, render/busy/ms 200, main/busy/ms 196, stage/main/update 122, pipelines/front_ms 61.15, stage/render/prepare_assets 50.33, stream/drive/ms 44.67, stage/render/prepare_batch_phases 42.58, stage/render/prepare_resources 41.88 |
| warm-up | 6.18 | 127.0 | pipelines/wait_ms 6161, pipelines/compile_ms 1663, pipelines/driver_ms 1565, render/thread/ms 173, render/busy/ms 171, main/busy/ms 158, pipelines/front_ms 98.83, stage/render/prepare_assets 69.08, stage/main/post_update 67.85, stage/main/update 50.02, stage/render/prepare_write_phase_buffers 24.06, stage/render/prepare_resources 22.01 |
| warm-up | 6.27 | 91.0 | pipelines/wait_ms 18.8 k, pipelines/compile_ms 1946, pipelines/driver_ms 1792, render/thread/ms 169, render/busy/ms 168, pipelines/front_ms 154, main/busy/ms 107, stage/render/prepare_assets 71.77, stage/main/post_update 67.35, stage/main/update 30.76, stage/render/prepare_flush 28.03, stage/render/render 20.82 |
| warm-up | 6.37 | 101.5 | pipelines/wait_ms 15.3 k, pipelines/compile_ms 342, pipelines/driver_ms 279, render/thread/ms 170, render/busy/ms 168, pipelines/front_ms 63.17, stage/render/prepare_assets 57.80, main/busy/ms 54.55, stage/render/prepare_write_phase_buffers 33.03, stage/render/prepare_resources 31.10, stage/main/post_update 24.87, stage/main/update 20.64 |
| warm-up | 6.54 | 70.7 | pipelines/wait_ms 711, pipelines/compile_ms 134, render/thread/ms 110, render/busy/ms 108, pipelines/driver_ms 99.54, main/busy/ms 57.77, pipelines/front_ms 34.36, stage/main/post_update 33.75, stage/render/prepare_resources 27.91, stage/render/prepare_assets 23.91, stage/render/prepare_write_phase_buffers 20.24, stage/main/update 15.69 |
| warm-up | 6.61 | 78.4 | render/thread/ms 124, render/busy/ms 123, main/busy/ms 52.47, stage/render/prepare_resources 36.62, stage/main/post_update 26.16, stage/render/prepare_assets 22.32, stage/render/prepare_write_phase_buffers 20.90, stage/main/update 18.41, stage/render/prepare_flush 16.89, stage/render/prepare_bind_groups 9.75, stage/render/render 9.32, render/graph/ms 7.12 |
| warm-up | 6.71 | 100.1 | pipelines/compile_ms 3083, pipelines/driver_ms 2950, render/thread/ms 145, render/busy/ms 143, pipelines/front_ms 133, pipelines/wait_ms 129, main/busy/ms 57.59, stage/render/prepare_assets 41.58, stage/render/prepare_resources 30.27, stage/main/update 24.09, stage/render/prepare_flush 15.93, stage/render/prepare_batch_phases 15.64 |
| warm-up | 6.78 | 67.4 | pipelines/wait_ms 6590, pipelines/compile_ms 3168, pipelines/driver_ms 3024, render/thread/ms 154, render/busy/ms 152, pipelines/front_ms 144, main/busy/ms 55.14, stage/render/prepare_assets 40.93, stage/render/prepare_resources 39.58, stage/main/update 24.16, stage/render/render 17.68, stage/render/prepare_batch_phases 16.45 |
| warm-up | 6.84 | 58.2 | pipelines/wait_ms 9758, pipelines/compile_ms 142, render/thread/ms 119, render/busy/ms 117, pipelines/driver_ms 117, stage/render/prepare_assets 48.70, main/busy/ms 41.39, stage/render/prepare_resources 28.73, pipelines/front_ms 25.05, stage/main/post_update 19.65, stage/render/prepare_write_phase_buffers 16.16, stage/main/update 14.24 |
| warm-up | 6.95 | 77.7 | pipelines/compile_ms 3555, pipelines/driver_ms 3257, pipelines/front_ms 297, render/thread/ms 104, render/busy/ms 103, main/busy/ms 71.62, stage/main/post_update 34.90, stage/render/prepare_write_phase_buffers 27.05, stage/render/prepare_resources 22.08, stage/main/update 17.44, stage/render/render 16.71, render/graph/ms 14.71 |
| warm-up | 7.05 | 66.7 | pipelines/compile_ms 2621, pipelines/driver_ms 2388, pipelines/wait_ms 499, pipelines/front_ms 234, render/thread/ms 82.78, render/busy/ms 81.34, main/busy/ms 66.85, stage/main/post_update 44.97, stage/render/prepare_assets 39.03, bigspace/total/ms 24.82, stage/render/prepare_resources 14.96, stage/main/update 13.91 |
| warm-up | 7.15 | 56.3 | pipelines/wait_ms 20.9 k, pipelines/compile_ms 2821, pipelines/driver_ms 2562, pipelines/front_ms 259, render/thread/ms 95.30, render/busy/ms 93.69, main/busy/ms 61.75, stage/render/prepare_assets 32.71, stage/main/post_update 31.42, stage/render/prepare_resources 27.29, stage/main/update 19.87, stage/render/render 11.48 |
| warm-up | 7.26 | 65.0 | pipelines/wait_ms 9727, pipelines/compile_ms 3934, pipelines/driver_ms 3604, pipelines/front_ms 330, render/thread/ms 102, render/busy/ms 99.86, main/busy/ms 74.41, stage/main/post_update 35.13, stage/render/prepare_resources 34.92, stage/main/update 24.77, stage/render/prepare_assets 19.65, stage/render/render 13.87 |
| warm-up | 7.31 | 56.7 | pipelines/wait_ms 6038, pipelines/compile_ms 150, render/thread/ms 116, pipelines/driver_ms 115, render/busy/ms 114, main/busy/ms 75.73, stage/render/prepare_resources 46.12, stage/main/post_update 37.21, pipelines/front_ms 35.57, stage/main/update 27.07, stage/render/prepare_assets 23.53, stage/render/render 12.52 |
| warm-up | 7.41 | 91.9 | pipelines/wait_ms 7952, pipelines/compile_ms 3815, pipelines/driver_ms 3591, pipelines/front_ms 224, render/thread/ms 133, render/busy/ms 131, main/busy/ms 82.99, stage/render/prepare_assets 53.70, stage/main/update 41.09, stage/render/prepare_resources 34.88, stage/main/post_update 27.52, stage/render/render 12.74 |
| warm-up | 7.55 | 93.2 | pipelines/wait_ms 20.0 k, pipelines/compile_ms 270, pipelines/driver_ms 227, render/thread/ms 136, render/busy/ms 134, main/busy/ms 102, stage/main/post_update 55.52, stage/render/prepare_assets 47.32, pipelines/front_ms 43.05, stage/main/fixed_main_loop 21.48, stage/render/prepare_meshes 18.58, stage/render/prepare_resources 17.99 |
| warm-up | 8.11 | 66.8 | pipelines/wait_ms 358, render/thread/ms 95.53, render/busy/ms 93.79, pipelines/compile_ms 85.53, pipelines/driver_ms 53.87, main/busy/ms 45.20, pipelines/front_ms 31.66, stage/main/post_update 25.80, stage/render/prepare_resources 22.75, stage/render/prepare_flush 22.62, stage/render/prepare_write_phase_buffers 17.73, stage/render/render 14.63 |
| warm-up | 8.19 | 77.5 | render/thread/ms 133, render/busy/ms 124, main/busy/ms 83.93, stage/main/post_update 43.46, stage/render/prepare_assets 34.24, stage/render/prepare_resources 31.87, stage/main/pre_update 24.17, stage/render/prepare_write_phase_buffers 17.02, stage/render/render 15.39, anim/evaluate/ms 13.71, render/graph/ms 12.91, stage/main/update 11.77 |
| warm-up | 8.32 | 77.9 | render/thread/ms 94.33, main/busy/ms 92.86, render/busy/ms 92.79, stage/main/post_update 73.95, anim/evaluate/ms 53.20, stage/render/prepare_collect_phase_buffers 35.93, stage/render/render 14.58, render/graph/ms 12.25, stage/main/update 11.47, encode//sum 11.38, stage/render/prepare_batch_phases 9.65, stage/render/graph_submit 9.37 |
| warm-up | 8.48 | 83.6 | pipelines/compile_ms 1755, pipelines/driver_ms 1528, pipelines/wait_ms 448, pipelines/front_ms 227, render/thread/ms 100.00, render/busy/ms 98.57, main/busy/ms 58.21, stage/render/prepare_resources 43.45, stage/main/post_update 24.23, stage/main/update 14.28, stage/render/cleanup 14.23, stage/main/pre_update 14.02 |
| warm-up | 8.59 | 60.7 | render/thread/ms 105, render/busy/ms 103, main/busy/ms 69.79, stage/main/update 40.46, stage/render/prepare_assets 29.83, stage/render/render 29.30, encode//sum 28.59, stage/render/prepare_resources 27.64, render/graph/ms 27.03, stage/render/graph_submit 24.09, stage/main/post_update 21.37, stream/drive/ms 20.27 |
| warm-up | 8.90 | 60.1 | pipelines/compile_ms 3981, pipelines/driver_ms 3771, pipelines/wait_ms 583, pipelines/front_ms 210, render/thread/ms 106, render/busy/ms 104, main/busy/ms 80.34, stage/main/post_update 47.88, stage/render/prepare_resources 27.92, stage/render/prepare_write_phase_buffers 25.21, stage/render/render 21.22, encode//sum 19.02 |
| warm-up | 8.98 | 79.8 | pipelines/compile_ms 3981, pipelines/driver_ms 3771, pipelines/wait_ms 583, pipelines/front_ms 210, render/thread/ms 121, render/busy/ms 119, main/busy/ms 87.99, stage/main/post_update 39.83, stage/render/prepare_assets 34.97, stage/render/prepare_write_phase_buffers 21.41, stage/main/update 21.14, stage/render/render 21.11 |
| warm-up | 9.24 | 80.7 | render/thread/ms 116, render/busy/ms 114, main/busy/ms 47.73, stage/render/prepare_write_phase_buffers 39.48, stage/main/update 26.94, stage/render/prepare_resources 26.25, stage/render/prepare_assets 19.35, stage/main/post_update 15.07, encode//sum 13.38, stage/render/render 12.77, stream/drive/ms 11.51, render/graph/ms 10.31 |
| warm-up | 9.54 | 84.9 | pipelines/wait_ms 12.4 k, pipelines/compile_ms 2591, pipelines/driver_ms 2357, pipelines/front_ms 234, render/thread/ms 113, render/busy/ms 111, main/busy/ms 98.49, stage/main/post_update 56.82, stage/render/render 34.53, stage/main/update 32.48, render/graph/ms 31.71, stage/render/graph_submit 28.93 |
| warm-up | 9.62 | 77.3 | pipelines/wait_ms 9326, render/thread/ms 148, render/busy/ms 146, main/busy/ms 138, pipelines/compile_ms 135, pipelines/driver_ms 94.65, stage/main/post_update 75.01, stage/render/render 53.12, stage/main/update 44.67, pipelines/front_ms 40.14, render/graph/ms 39.96, stage/render/graph_submit 37.10 |
| warm-up | 9.73 | 115.9 | render/thread/ms 181, render/busy/ms 179, main/busy/ms 155, stage/main/post_update 99.42, stage/render/render 45.62, stage/render/prepare_assets 45.20, stage/main/update 38.70, stage/render/prepare_write_phase_buffers 35.93, stage/render/prepare_resources 35.44, render/graph/ms 32.70, bigspace/total/ms 31.45, stage/render/graph_submit 29.86 |
| warm-up | 9.85 | 61.9 | render/thread/ms 108, render/busy/ms 106, main/busy/ms 94.30, stage/main/update 43.93, stage/render/prepare_resources 33.97, stage/main/post_update 33.40, stage/render/render 16.34, encode//sum 15.38, stage/render/prepare_bind_groups 15.20, stage/render/prepare_write_phase_buffers 14.61, render/graph/ms 13.90, stage/render/prepare_assets 13.34 |
| warm-up | 9.91 | 61.6 | render/thread/ms 116, render/busy/ms 114, main/busy/ms 88.26, stage/main/update 43.36, stage/render/prepare_assets 35.25, stage/main/post_update 34.09, stage/render/prepare_resources 30.30, stage/render/prepare_write_phase_buffers 14.05, stage/render/render 13.78, encode//sum 13.56, render/graph/ms 11.19, stream/drive/ms 9.20 |
| warm-up | 10.10 | 66.6 | render/thread/ms 98.58, render/busy/ms 97.10, main/busy/ms 67.59, stage/main/post_update 36.53, stage/render/prepare_assets 29.88, stage/render/render 17.95, stage/render/prepare_resources 17.33, bigspace/total/ms 15.68, render/graph/ms 15.51, encode//sum 14.35, stage/main/update 13.72, bigspace/low_precision/ms 13.36 |
| warm-up | 10.16 | 58.3 | render/thread/ms 120, render/busy/ms 119, main/busy/ms 82.21, stage/main/post_update 53.03, stage/render/prepare_resources 29.04, stage/render/prepare_assets 27.88, stage/render/prepare_flush 24.94, stage/main/update 15.49, bigspace/total/ms 14.04, bigspace/low_precision/ms 13.44, encode//sum 13.35, stage/render/render 12.26 |
| warm-up | 10.22 | 55.8 | render/thread/ms 111, render/busy/ms 109, main/busy/ms 100, stage/main/post_update 72.57, stage/render/prepare_resources 23.72, stage/render/prepare_flush 21.42, stage/render/queue 16.83, stage/main/update 14.28, stage/render/render 13.48, stage/render/prepare_assets 13.20, encode//sum 12.91, render/graph/ms 11.44 |
| warm-up | 10.31 | 91.8 | render/thread/ms 133, render/busy/ms 126, main/busy/ms 91.63, stage/main/post_update 62.69, stage/render/prepare_assets 35.73, stage/render/prepare_resources 23.38, stage/render/queue 19.58, stage/main/update 18.73, stage/render/prepare_batch_phases 14.90, stage/render/render 12.95, encode//sum 12.59, render/graph/ms 10.81 |
| warm-up | 10.37 | 61.0 | pipelines/compile_ms 1964, pipelines/driver_ms 1835, pipelines/wait_ms 1067, render/thread/ms 137, render/busy/ms 130, pipelines/front_ms 129, main/busy/ms 87.08, stage/main/update 45.01, stage/main/post_update 29.33, stage/render/prepare_assets 27.76, stage/render/prepare_resources 27.16, stage/render/prepare_write_phase_buffers 22.63 |
| warm-up | 10.43 | 65.6 | pipelines/wait_ms 7385, pipelines/compile_ms 3728, pipelines/driver_ms 3441, pipelines/front_ms 287, render/thread/ms 115, render/busy/ms 113, main/busy/ms 63.36, stage/main/update 38.75, stage/render/prepare_resources 30.15, stage/render/prepare_assets 24.98, stage/render/prepare_write_phase_buffers 24.11, encode//sum 14.61 |
| warm-up | 10.63 | 57.8 | pipelines/compile_ms 4347, pipelines/driver_ms 4107, pipelines/wait_ms 746, pipelines/front_ms 240, render/thread/ms 83.66, render/busy/ms 80.12, main/busy/ms 69.30, stage/main/update 36.67, stage/render/prepare_resources 26.94, stage/main/post_update 23.51, stage/render/prepare_write_phase_buffers 21.67, encode//sum 14.82 |
| warm-up | 10.78 | 65.7 | pipelines/wait_ms 26.8 k, pipelines/compile_ms 3098, pipelines/driver_ms 2658, pipelines/front_ms 440, render/thread/ms 84.45, render/busy/ms 81.97, main/busy/ms 50.19, encode//sum 32.42, stage/render/prepare_resources 32.23, stage/main/update 27.14, stage/render/render 25.83, render/graph/ms 23.42 |
| warm-up | 10.86 | 77.9 | pipelines/wait_ms 32.9 k, pipelines/compile_ms 3279, pipelines/driver_ms 2778, pipelines/front_ms 501, render/thread/ms 138, render/busy/ms 136, main/busy/ms 63.66, encode//sum 45.55, stage/render/prepare_resources 39.20, stage/render/render 33.43, render/graph/ms 30.91, stage/render/graph_submit 27.69 |
| warm-up | 10.95 | 87.4 | pipelines/wait_ms 19.6 k, pipelines/compile_ms 289, pipelines/driver_ms 196, render/thread/ms 159, render/busy/ms 158, main/busy/ms 98.69, pipelines/front_ms 93.65, stage/main/post_update 55.69, stage/render/prepare_resources 39.72, stage/render/prepare_assets 32.28, encode//sum 30.40, stage/main/update 27.78 |
| warm-up | 11.04 | 58.7 | render/thread/ms 84.38, render/busy/ms 82.84, main/busy/ms 50.79, encode//sum 42.58, stage/render/prepare_assets 32.89, stage/main/post_update 29.40, stage/render/render 27.31, render/graph/ms 25.02, stage/render/graph_submit 21.94, encode/finish_wall 15.49, stage/main/update 12.78, bigspace/total/ms 9.68 |

## Run

- `../target/profiling/walkonce.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set render.wgpu.bind_group_walk_once=on --resolution 2560x1440 --bench-out target/perf\wgpu_walk_once\balmora_walk@2560x1440_arm=once-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 16%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1386 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **75%** (min 40%) · 2625 MHz (min 2610) · memory 8101 MHz · 78.4 W · 75 °C (max 82) · VRAM 6859 MiB · throttled 100% of samples

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
- `render.wgpu.bind_group_walk_once=on (command line: --set)`
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
