# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6117 frames over 65.7 s · **median 10.47 ms (95 fps)** · mean 10.73 · p95 13.37 · p99 16.85 (1% low 59 fps) · worst 40.80 ms · 79 spikes over 1.5× median · 60.3 stutters/min (work 53.0/min)
- warm-up: 325 frames over 9.2 s, worst 2567.0 ms
- hitches (> 26.2 ms): **32** measured, 46 in warm-up
- timed GPU passes: 6.48 ms of a 10.73 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.47 ms, limit 11 ms
- **FAIL** frame_ms: 66 stutters (> 1.5x moving median of 60) in 1.09 min = 60.32/min, limit 1/min
- **FAIL** worst frame 40.80 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.836 | 2.036 | 2.066 | 0.90 | 0.174 |
| main_opaque_pass_3d | 1.469 | 1.629 | 2.052 | 0.90 | 0.059 |
| early prepass | 0.449 | 0.498 | 0.737 | 0.90 | 0.088 |
| bloom | 0.283 | 0.314 | 0.321 | 0.90 | 0.063 |
| atmosphere_luts | 0.274 | 0.304 | 0.366 | 0.90 | 0.003 |
| water_velocity | 0.252 | 0.279 | 0.347 | 0.90 | 0.002 |
| water_sspr | 0.182 | 0.202 | 0.203 | 0.90 | 0.002 |
| ssao | 0.170 | 0.188 | 0.193 | 0.90 | 0.007 |
| atmos_froxel_inject | 0.131 | 0.146 | 0.147 | 0.90 | 0.001 |
| clouds_trace | 0.127 | 0.140 | 0.230 | 0.90 | 0.001 |
| water_fft | 0.119 | 0.132 | 0.138 | 0.90 | 0.003 |
| auxiliary_views | 0.117 | 0.130 | 0.531 | 0.90 | 0.129 |
| uniform_allocation | 0.115 | 0.128 | 0.181 | 0.90 | 0.023 |
| upscaling | 0.107 | 0.119 | 0.126 | 0.90 | 0.004 |
| clouds_composite | 0.095 | 0.106 | 0.109 | 0.90 | 0.002 |
| bin_unpacking | 0.094 | 0.104 | 0.143 | 0.90 | 0.017 |
| main_transmissive_pass_3d | 0.077 | 0.086 | 0.178 | 0.90 | 0.009 |
| tonemapping | 0.076 | 0.085 | 0.088 | 0.90 | 0.003 |
| clustering | 0.058 | 0.064 | 0.068 | 0.90 | 0.018 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.90 | 0.002 |
| clouds_resolve | 0.043 | 0.048 | 0.049 | 0.90 | 0.001 |
| lightprobe_irradiance_map | 0.042 | 0.414 | 0.416 | 0.10 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.90 | 0.001 |
| surface_trail | 0.035 | 0.056 | 0.057 | 0.63 | 0.002 |
| surface_exposure_far | 0.034 | 0.042 | 0.045 | 0.80 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.90 | 0.003 |
| early_mesh_preprocessing | 0.027 | 0.030 | 0.047 | 0.90 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.029 | 0.031 | 0.90 | 0.001 |
| water_snapshot | 0.022 | 0.025 | 0.028 | 0.90 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.219 | 0.230 | 0.10 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.024 | 0.90 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.90 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.90 | 0.123 |
| clouds_fog_light | 0.010 | 0.012 | 0.012 | 0.90 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.90 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.90 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.90 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.171 | 11.508 | 36.94 | 56098.5 |
| main/busy/ms | 8.950 | 10.641 | 24.72 | 54747.9 |
| render/busy/ms | 8.616 | 11.006 | 36.45 | 52703.2 |
| stage/main/post_update | 5.541 | 6.934 | 10.70 | 33894.9 |
| stage/render/render | 5.352 | 6.954 | 15.03 | 32740.4 |
| render/graph/ms | 4.609 | 6.184 | 14.06 | 28191.0 |
| encode//sum | 3.631 | 4.820 | 23.62 | 22208.7 |
| stage/render/graph_submit | 3.456 | 4.678 | 10.40 | 21138.8 |
| encode/submit | 2.203 | 3.198 | 6.87 | 13475.2 |
| stage/main/update | 2.150 | 2.545 | 18.40 | 13151.3 |
| encode/finish_wall | 1.328 | 1.920 | 4.47 | 8121.5 |
| stage/render/camera_driver | 1.099 | 1.543 | 3.59 | 6721.2 |
| encode/pass:shadow_directional_light/sum | 0.855 | 1.529 | 3.56 | 5230.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.775 | 1.077 | 1.69 | 4743.6 |
| encode/pass:shadow_point_light/sum | 0.758 | 3.319 | 17.00 | 4634.5 |
| encode/pass:early prepass/sum | 0.756 | 1.066 | 1.68 | 4626.8 |
| stage/render/prepare_write_phase_buffers | 0.728 | 1.068 | 2.16 | 4452.3 |
| bigspace/total/ms | 0.556 | 1.334 | 2.87 | 3401.1 |
| stage/render/prepare_assets | 0.554 | 0.713 | 11.46 | 3390.4 |
| render/present/ms | 0.459 | 1.171 | 2.74 | 2809.1 |
| stage/render/prepare_resources | 0.443 | 0.523 | 0.91 | 2709.9 |
| stage/render/prepare_meshes | 0.430 | 0.538 | 6.35 | 2632.7 |
| stage/main/first | 0.399 | 0.522 | 5.38 | 2437.7 |
| stage/main/pre_update | 0.359 | 0.526 | 4.21 | 2197.4 |
| stage/render/specialize | 0.313 | 0.400 | 13.77 | 1913.2 |
| terrain/update/ms | 0.302 | 0.387 | 1.89 | 1849.9 |
| stage/render/queue | 0.281 | 0.407 | 7.66 | 1720.9 |
| stage/render/prepare_bind_groups | 0.275 | 0.484 | 2.27 | 1681.3 |
| terrain/select/ms | 0.269 | 0.346 | 0.91 | 1646.4 |
| controller/feed/ms | 0.265 | 0.329 | 0.85 | 1618.6 |
| stream/drive/ms | 0.247 | 0.326 | 16.39 | 1510.1 |
| bigspace/low_precision/ms | 0.229 | 0.802 | 2.36 | 1399.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.228 | 0.334 | 1.02 | 1397.2 |
| stage/main/fixed_main_loop | 0.210 | 0.365 | 1.36 | 1286.4 |
| far/select/ms | 0.201 | 0.267 | 0.58 | 1232.5 |
| stage/render/create_views | 0.197 | 0.254 | 0.93 | 1207.5 |
| stream/tick/ms | 0.192 | 0.259 | 0.72 | 1172.1 |
| stage/render/prepare_batch_phases | 0.190 | 0.271 | 0.62 | 1162.8 |
| stage/render/extract_commands | 0.170 | 0.199 | 0.44 | 1037.2 |
| anim/evaluate/ms | 0.168 | 0.321 | 1.35 | 1028.1 |
| stage/render/prepare_flush | 0.165 | 0.328 | 1.37 | 1009.6 |
| bigspace/high_precision/ms | 0.163 | 0.678 | 2.35 | 998.6 |
| anim/evaluate_cpu/ms | 0.152 | 0.199 | 0.75 | 931.7 |
| stage/render/prepare_views | 0.139 | 0.173 | 0.63 | 848.7 |
| controller/move/ms | 0.137 | 0.336 | 0.61 | 835.3 |
| stage/main/last | 0.114 | 0.157 | 1.60 | 697.5 |
| render/acquire/ms | 0.096 | 0.124 | 0.56 | 586.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.094 | 0.151 | 0.69 | 576.5 |
| stage/render/prepare_collect_phase_buffers | 0.061 | 0.131 | 0.51 | 370.4 |
| pipelines/compile_ms | 0.047 | 0.000 | 123.15 | 285.7 |
| physics/step/ms | 0.045 | 0.085 | 0.71 | 277.0 |
| pipelines/front_ms | 0.045 | 0.000 | 119.18 | 274.3 |
| stage/render/cleanup | 0.032 | 0.093 | 0.63 | 196.7 |
| physics/insert/ms | 0.032 | 0.350 | 0.93 | 193.7 |
| stage/render/phase_sort | 0.031 | 0.054 | 0.24 | 190.6 |
| encode/resolve_diagnostics/sum | 0.031 | 0.041 | 1.39 | 186.7 |
| weather/map/ms | 0.025 | 0.032 | 0.22 | 151.4 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.44 | 88.9 |
| anim/apply/ms | 0.012 | 0.021 | 0.49 | 73.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.24 | 68.3 |
| weather/eval/ms | 0.011 | 0.014 | 0.10 | 64.6 |
| controller/avoid/ms | 0.010 | 0.014 | 0.25 | 64.1 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 42.4 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 31.7 |
| stream/despawn/ms | 0.005 | 0.000 | 0.98 | 31.6 |
| script/ui/ms | 0.005 | 0.006 | 0.03 | 28.3 |
| stream/spawn/ms | 0.004 | 0.000 | 1.11 | 27.5 |
| controller/write/ms | 0.003 | 0.004 | 0.05 | 21.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.66 | 14.0 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.97 | 11.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.0 |
| stage/render/prepare | 0.001 | 0.002 | 0.13 | 3.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 5 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 42 | 13 |
| crowd/c0/joints/count | count | 1272 | 2218 | 595 |
| crowd/c0/meshes/count | count | 353 | 633 | 158 |
| crowd/c1/count | count | 145 | 156 | 147 |
| crowd/c1/joints/count | count | 5531 | 5957 | 5569 |
| crowd/c1/meshes/count | count | 1176 | 1283 | 1175 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4612 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4589 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.39 | 1.57 | 0.40 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.44 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.24 | 0.01 |
| encode/finish_critical | ms | 0.96 | 3.56 | 1.13 |
| encode/pass:early prepass/max | ms | 0.77 | 1.68 | 0.80 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.79 | 1.69 | 1.13 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.69 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 1.02 | 0.18 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 3.56 | 0.65 |
| encode/pass:shadow_point_light/max | ms | 0.80 | 1.40 | 0.88 |
| encode/resolve_diagnostics/max | ms | 0.03 | 1.39 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.10 | 36.45 | 11.10 |
| gpu/timed/ms | ms | 7.89 | 11.39 | 8.27 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1293 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3411 | 5110 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1293 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3411 | 5110 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 14 | 199 | 4 |
| phases/shadow/bins | count | 4994 | 8802 | 2405 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.3 k | 26.4 k | 6352 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 203 | 253 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 184 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 555 | 24 |
| preprocess/bind_groups_created | count | 13 | 555 | 24 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 128 | 237 | 105 |
| process/cpu_usage | % | 26.92 | 41.74 | 30.05 |
| process/mem_usage | GiB | 5.52 | 5.70 | 5.70 |
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
| skin/blocks_changed/count | count | 13 | 122 | 13 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 368.9 k | 810.2 k | 494.3 k |
| skin/joints_written/count | count | 5764 | 12.7 k | 7724 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4918 | 5011 | 5011 |
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
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.31 | 44.54 | 32.43 |
| system/mem_usage | % | 54.82 | 55.51 | 54.81 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 63 | 246 | 54 |
| views/all/bins | count | 7592 | 11.7 k | 5903 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 249 | 300 | 75 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 10.7 k | 35.4 k | 8123 |
| views/cascade/batch_sets | count | 9 | 54 | 4 |
| views/cascade/bins | count | 2796 | 6216 | 2405 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8904 | 23.5 k | 6352 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 409 | 22.9 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 191 | 0 |
| views/lamp_face/bins | count | 2152 | 2751 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2402 | 3049 | 0 |
| views/lamp_face/pipelines | count | 168 | 204 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2917 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2886 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8432 | 13.0 k | 7614 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8599 | 13.2 k | 7922 |
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
| anim/bones_written/count | count | 57.56 | 228 | 352.1 k |
| anim/events/count | count | 0.09 | 3 | 528 |
| assets/Image/added | count | 0.10 | 43 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.66 | 152 | 4061 |
| assets/Mesh/removed | count | 0.79 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.54 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.76 | 1328 | 4675 |
| encode//count | count | 32.78 | 220 | 200.5 k |
| encode/bind_walk/init_textures | count | 12.2 k | 114.7 k | 74.51 M |
| encode/bind_walk/init_textures_skipped | count | 65.3 k | 156.8 k | 399.43 M |
| encode/bind_walk/inits | count | 455 | 1119 | 2.78 M |
| encode/bind_walk/inits_skipped | count | 109 | 167 | 664.3 k |
| encode/bind_walk/merge_textures | count | 12.1 k | 114.6 k | 74.06 M |
| encode/bind_walk/merge_textures_skipped | count | 5364 | 6012 | 32.81 M |
| encode/bind_walk/merges | count | 97.54 | 334 | 596.7 k |
| encode/bind_walk/merges_skipped | count | 84.50 | 132 | 516.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6117 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6117 |
| encode/pass:early prepass/count | count | 1 | 1 | 6117 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6117 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6117 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6117 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6117 |
| encode/pass:shadow_point_light/count | count | 1.21 | 48 | 7397 |
| encode/queue/command_buffers | count | 44.00 | 278 | 269.2 k |
| encode/queue/submits | count | 2.02 | 4 | 12.3 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.88 M | 11.51 G |
| encode/queue/write_buffer/calls | count | 255 | 828 | 1.56 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.6 k | 3.53 M | 1.09 G |
| encode/queue/write_buffer_with/calls | count | 10.98 | 465 | 67.1 k |
| encode/queue/write_texture/bytes | bytes | 11.7 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6117 |
| physics/inserts | count | 0.37 | 111 | 2261 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 129 | 2335 |
| physics/steps | count | 0.64 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 19 | 378 |
| stream/upload/bytes | bytes | 5534 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1213 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.2 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2567.0 | main/busy/ms 2534, stage/main/post_update 2430, stage/main/update 96.12, surface/advance_cpu_ms 65.62, stage/main/pre_update 4.60, stage/main/last 1.17, stream/drive/ms 0.89, surface/cell_upload_ms 0.83, stage/main/first 0.55, stage/main/fixed_main_loop 0.44, stream/tick/ms 0.22, weather/map/ms 0.14 |
| warm-up | 3.43 | 861.9 | main/busy/ms 2676, stage/main/post_update 2457, render/thread/ms 823, render/busy/ms 640, stage/render/prepare_assets 590, stage/main/update 205, stage/render/prepare_views 177, render/acquire/ms 177, surface/advance_cpu_ms 65.62, stage/render/create_views 29.06, stage/render/render 15.76, render/graph/ms 8.46 |
| warm-up | 3.54 | 113.5 | render/thread/ms 920, render/busy/ms 684, stage/render/prepare_assets 620, stage/render/prepare_views 178, render/acquire/ms 178, main/busy/ms 162, stage/main/update 113, pipelines/compile_ms 104, stage/render/render 72.56, pipelines/front_ms 61.88, render/present/ms 57.48, pipelines/driver_ms 42.14 |
| warm-up | 3.60 | 60.0 | pipelines/compile_ms 593, pipelines/wait_ms 534, pipelines/front_ms 511, render/thread/ms 154, stage/render/render 107, render/busy/ms 101, pipelines/driver_ms 82.06, main/busy/ms 63.21, render/graph/ms 53.39, render/present/ms 52.30, stage/main/post_update 44.17, stage/render/graph_submit 41.97 |
| warm-up | 3.70 | 99.4 | pipelines/wait_ms 3722, pipelines/compile_ms 1115, pipelines/front_ms 1033, render/thread/ms 154, render/busy/ms 151, main/busy/ms 148, pipelines/driver_ms 82.50, stage/render/render 71.25, stage/main/post_update 67.44, stage/main/update 54.75, render/graph/ms 54.16, stage/render/prepare_resources 45.13 |
| warm-up | 3.79 | 86.6 | pipelines/wait_ms 3606, pipelines/compile_ms 1695, pipelines/front_ms 1575, render/thread/ms 171, render/busy/ms 169, main/busy/ms 122, pipelines/driver_ms 120, stage/render/prepare_assets 77.35, stage/main/update 55.06, stage/render/prepare_resources 45.00, stage/main/post_update 43.56, stage/render/render 27.35 |
| warm-up | 3.83 | 38.7 | pipelines/compile_ms 1449, pipelines/front_ms 1342, pipelines/wait_ms 420, render/thread/ms 112, render/busy/ms 110, pipelines/driver_ms 107, stage/render/prepare_assets 76.67, main/busy/ms 29.93, stage/render/render 26.42, render/graph/ms 23.61, stage/render/graph_submit 20.47, stage/main/post_update 17.37 |
| warm-up | 3.86 | 31.9 | pipelines/compile_ms 868, pipelines/front_ms 821, pipelines/wait_ms 152, render/thread/ms 66.44, render/busy/ms 64.32, pipelines/driver_ms 46.51, stage/render/prepare_assets 32.26, main/busy/ms 28.86, stage/render/render 24.80, render/graph/ms 22.22, stage/render/graph_submit 19.06, stage/main/post_update 17.22 |
| warm-up | 3.90 | 37.6 | pipelines/compile_ms 1091, pipelines/front_ms 1032, pipelines/wait_ms 181, render/thread/ms 65.25, render/busy/ms 63.67, pipelines/driver_ms 59.86, stage/render/prepare_assets 35.53, main/busy/ms 27.63, stage/render/render 18.09, render/graph/ms 16.10, encode//sum 16.06, stage/main/post_update 13.98 |
| warm-up | 3.93 | 35.8 | pipelines/compile_ms 1259, pipelines/front_ms 1194, render/thread/ms 70.57, render/busy/ms 68.80, pipelines/driver_ms 64.56, main/busy/ms 33.42, pipelines/wait_ms 32.47, stage/render/render 25.65, render/graph/ms 23.40, stage/render/prepare_flush 17.37, stage/main/post_update 17.09, stage/render/graph_submit 16.30 |
| warm-up | 4.08 | 92.9 | pipelines/compile_ms 1166, pipelines/front_ms 1130, main/busy/ms 57.25, render/thread/ms 49.48, render/busy/ms 48.06, stage/main/update 44.72, stream/drive/ms 39.85, pipelines/driver_ms 36.35, stage/render/prepare_assets 34.39, pipelines/wait_ms 12.15, stage/render/render 9.46, stage/main/post_update 8.16 |
| warm-up | 4.13 | 36.0 | pipelines/compile_ms 320, pipelines/front_ms 305, render/thread/ms 41.17, render/busy/ms 39.26, main/busy/ms 38.10, stage/main/update 20.37, stage/render/prepare_assets 18.48, stage/render/render 15.76, pipelines/driver_ms 14.67, render/graph/ms 13.48, stage/render/graph_submit 11.09, stream/drive/ms 10.21 |
| warm-up | 4.24 | 82.2 | pipelines/compile_ms 266, pipelines/front_ms 256, render/thread/ms 73.57, render/busy/ms 71.10, main/busy/ms 58.20, stage/render/prepare_assets 39.48, stage/main/update 36.44, stream/drive/ms 28.76, stage/render/render 16.05, stage/main/post_update 13.76, render/graph/ms 12.81, pipelines/driver_ms 10.54 |
| warm-up | 4.29 | 55.3 | pipelines/compile_ms 662, pipelines/front_ms 634, render/thread/ms 95.25, render/busy/ms 92.09, main/busy/ms 66.92, stage/render/prepare_assets 45.27, stage/main/update 42.72, pipelines/driver_ms 28.66, stage/render/render 21.93, render/graph/ms 15.94, stage/main/post_update 13.86, stage/render/graph_submit 10.89 |
| warm-up | 4.43 | 136.4 | pipelines/compile_ms 1965, pipelines/front_ms 1902, render/thread/ms 130, render/busy/ms 128, main/busy/ms 66.58, pipelines/driver_ms 63.58, stage/render/prepare_assets 55.72, stage/main/update 43.19, stage/render/prepare_bind_groups 34.26, stage/render/render 27.19, render/graph/ms 22.43, stage/render/graph_submit 19.04 |
| warm-up | 4.51 | 78.3 | pipelines/compile_ms 2200, pipelines/front_ms 2090, pipelines/wait_ms 414, render/thread/ms 158, render/busy/ms 156, pipelines/driver_ms 110, main/busy/ms 73.53, stage/render/prepare_assets 60.84, stage/render/prepare_bind_groups 28.23, stage/main/post_update 24.26, stage/render/render 23.40, render/graph/ms 21.33 |
| warm-up | 4.59 | 78.6 | pipelines/compile_ms 898, pipelines/front_ms 822, pipelines/wait_ms 419, render/thread/ms 137, render/busy/ms 135, main/busy/ms 90.11, pipelines/driver_ms 75.48, stage/render/prepare_assets 47.68, encode//sum 34.92, stage/main/post_update 29.28, stage/main/pre_update 27.59, stage/render/prepare_resources 26.79 |
| warm-up | 4.69 | 103.9 | pipelines/compile_ms 2516, pipelines/front_ms 2440, pipelines/wait_ms 794, render/thread/ms 164, render/busy/ms 162, pipelines/driver_ms 75.87, main/busy/ms 75.07, stage/render/prepare_assets 56.42, encode//sum 47.84, stage/main/update 37.60, stage/render/render 25.88, stage/render/prepare_resources 24.81 |
| warm-up | 4.75 | 54.9 | pipelines/compile_ms 2946, pipelines/front_ms 2848, pipelines/wait_ms 995, render/thread/ms 147, render/busy/ms 146, pipelines/driver_ms 97.37, encode//sum 77.92, main/busy/ms 58.49, stage/main/update 36.08, stage/render/prepare_assets 34.59, stage/render/render 33.52, render/graph/ms 31.13 |
| warm-up | 4.78 | 32.1 | pipelines/compile_ms 526, pipelines/front_ms 500, pipelines/wait_ms 210, render/thread/ms 76.66, render/busy/ms 74.85, encode//sum 64.32, main/busy/ms 38.38, stage/render/render 28.97, render/graph/ms 26.66, pipelines/driver_ms 26.00, stage/render/graph_submit 19.01, stage/main/update 17.71 |
| warm-up | 4.81 | 34.0 | pipelines/compile_ms 267, pipelines/front_ms 258, main/busy/ms 85.59, stage/main/update 68.57, stream/drive/ms 57.08, render/thread/ms 49.83, render/busy/ms 48.47, stage/render/prepare_assets 15.97, pipelines/wait_ms 15.09, stage/render/render 14.26, render/graph/ms 12.14, encode//sum 10.18 |
| warm-up | 4.89 | 78.5 | pipelines/compile_ms 1804, pipelines/front_ms 1753, pipelines/wait_ms 565, main/busy/ms 114, stage/main/update 93.61, stream/drive/ms 56.21, render/thread/ms 55.28, render/busy/ms 52.07, pipelines/driver_ms 50.18, stage/render/render 26.32, render/graph/ms 22.11, stage/render/graph_submit 15.74 |
| warm-up | 4.99 | 96.6 | pipelines/compile_ms 2086, pipelines/front_ms 2030, pipelines/wait_ms 566, main/busy/ms 140, stage/main/update 116, render/thread/ms 92.97, render/busy/ms 84.75, stream/drive/ms 57.39, pipelines/driver_ms 55.71, stage/render/render 39.75, render/graph/ms 30.31, stage/render/prepare_assets 24.72 |
| warm-up | 5.09 | 105.1 | pipelines/compile_ms 1917, pipelines/front_ms 1865, main/busy/ms 112, render/thread/ms 112, render/busy/ms 106, stage/main/update 87.76, stream/drive/ms 58.94, pipelines/driver_ms 52.02, stage/render/prepare_assets 45.98, pipelines/wait_ms 34.99, stage/render/render 34.66, render/graph/ms 27.33 |
| warm-up | 5.14 | 47.0 | pipelines/compile_ms 1762, pipelines/front_ms 1709, main/busy/ms 91.65, render/thread/ms 89.77, render/busy/ms 88.11, stage/main/update 62.77, pipelines/driver_ms 52.64, stream/drive/ms 46.89, stage/render/prepare_assets 40.99, pipelines/wait_ms 27.42, stage/render/render 26.40, render/graph/ms 24.21 |
| warm-up | 5.22 | 77.6 | pipelines/compile_ms 567, pipelines/front_ms 546, main/busy/ms 93.53, render/thread/ms 67.28, stage/main/update 66.97, render/busy/ms 65.65, stream/drive/ms 45.55, stage/render/prepare_assets 32.79, pipelines/driver_ms 21.63, stage/render/render 16.54, stage/main/post_update 16.45, render/graph/ms 14.39 |
| warm-up | 5.25 | 34.3 | pipelines/compile_ms 266, pipelines/front_ms 256, main/busy/ms 76.83, stage/main/update 59.58, render/thread/ms 52.47, render/busy/ms 51.14, stream/drive/ms 32.50, stage/render/prepare_assets 27.58, stage/render/render 10.81, pipelines/driver_ms 10.21, stage/main/post_update 9.70, render/graph/ms 8.73 |
| warm-up | 5.31 | 58.2 | pipelines/compile_ms 982, pipelines/front_ms 953, main/busy/ms 69.11, stage/main/update 52.33, render/thread/ms 44.29, render/busy/ms 42.76, stream/drive/ms 32.57, pipelines/driver_ms 28.53, stage/render/prepare_assets 18.39, stage/render/render 11.31, stage/main/post_update 9.13, render/graph/ms 9.07 |
| warm-up | 5.34 | 31.6 | pipelines/compile_ms 1129, pipelines/front_ms 1096, render/thread/ms 50.08, render/busy/ms 47.63, main/busy/ms 40.95, pipelines/driver_ms 33.23, stage/main/post_update 17.76, stage/main/update 16.52, stage/render/render 16.01, render/graph/ms 13.66, stage/render/prepare_assets 10.37, encode//sum 9.16 |
| warm-up | 5.42 | 29.3 | pipelines/compile_ms 317, pipelines/front_ms 307, render/thread/ms 49.45, render/busy/ms 47.89, main/busy/ms 35.67, stage/render/render 16.09, stage/main/update 15.25, stage/main/post_update 14.58, render/graph/ms 14.11, stage/render/prepare_assets 11.65, encode//sum 11.00, stage/render/graph_submit 10.73 |
| warm-up | 5.45 | 30.2 | pipelines/compile_ms 603, pipelines/front_ms 588, render/thread/ms 52.45, render/busy/ms 50.85, main/busy/ms 37.99, stage/main/update 17.61, stage/render/render 17.48, pipelines/driver_ms 15.46, render/graph/ms 15.43, stage/render/prepare_assets 12.17, stage/render/graph_submit 12.03, stage/main/post_update 11.85 |
| warm-up | 5.50 | 49.6 | pipelines/compile_ms 717, pipelines/front_ms 700, render/thread/ms 65.41, render/busy/ms 63.76, main/busy/ms 40.54, stage/render/prepare_assets 30.35, stage/main/update 22.51, stage/render/render 18.52, pipelines/driver_ms 16.57, render/graph/ms 16.50, stage/render/graph_submit 13.51, stage/main/post_update 9.48 |
| warm-up | 5.63 | 29.6 | pipelines/compile_ms 193, pipelines/front_ms 187, render/thread/ms 46.88, render/busy/ms 45.63, stage/render/prepare_assets 29.68, main/busy/ms 28.33, stage/main/update 12.09, stage/main/post_update 9.87, stage/render/render 7.65, encode//sum 6.78, pipelines/driver_ms 5.95, render/graph/ms 5.90 |
| warm-up | 5.78 | 29.6 | render/thread/ms 50.22, render/busy/ms 49.00, stage/render/prepare_assets 29.82, main/busy/ms 25.44, stage/main/post_update 12.14, stage/render/render 11.31, render/graph/ms 9.37, stage/main/update 8.09, encode//sum 7.92, stage/render/graph_submit 6.91, encode/submit 4.20, encode/finish_wall 2.69 |
| warm-up | 6.32 | 32.4 | render/thread/ms 39.70, render/busy/ms 38.74, main/busy/ms 27.66, stage/render/prepare_assets 17.37, stage/main/update 11.27, stage/main/post_update 9.79, stage/render/render 8.71, render/graph/ms 7.22, encode//sum 7.17, stage/render/graph_submit 5.15, stream/drive/ms 4.16, stage/render/specialize 3.11 |
| warm-up | 6.53 | 33.0 | render/thread/ms 43.20, render/busy/ms 41.97, main/busy/ms 41.93, stage/main/update 18.41, stage/main/post_update 16.67, stage/render/render 14.65, render/graph/ms 12.59, encode//sum 11.43, stage/render/prepare_assets 9.20, stage/render/graph_submit 8.18, stream/drive/ms 6.10, encode/submit 4.49 |
| warm-up | 6.63 | 33.8 | main/busy/ms 42.84, render/thread/ms 29.64, render/busy/ms 28.12, stage/main/update 22.22, stage/main/post_update 15.92, stream/drive/ms 13.70, stage/render/render 12.84, render/graph/ms 10.68, encode//sum 8.91, stage/render/graph_submit 7.32, stage/render/prepare_assets 5.56, encode/submit 4.77 |
| warm-up | 7.05 | 34.5 | main/busy/ms 42.78, render/thread/ms 25.21, render/busy/ms 24.02, stage/main/update 22.20, stream/drive/ms 15.18, stream/spawn/ms 13.73, stage/main/post_update 13.19, stage/render/render 11.23, render/graph/ms 9.49, encode//sum 8.73, stage/render/graph_submit 6.80, stage/render/prepare_assets 4.93 |
| warm-up | 7.13 | 39.2 | pipelines/compile_ms 138, pipelines/front_ms 135, render/thread/ms 45.43, render/busy/ms 44.25, main/busy/ms 27.09, stage/main/update 12.49, stage/render/render 11.51, stage/render/specialize 11.41, stage/main/post_update 10.51, render/graph/ms 9.64, stage/render/queue 7.85, encode//sum 7.36 |
| warm-up | 8.40 | 29.6 | render/thread/ms 34.98, render/busy/ms 33.99, main/busy/ms 18.25, stage/render/render 10.54, stage/main/post_update 10.51, stage/render/specialize 9.34, render/graph/ms 9.13, encode//sum 7.17, stage/render/graph_submit 6.98, stage/render/queue 5.63, stage/main/update 4.98, encode/submit 3.97 |
| measure | 0.40 | 39.9 | render/thread/ms 43.38, render/busy/ms 42.42, encode//sum 20.27, main/busy/ms 17.82, stage/render/render 16.49, render/graph/ms 14.95, stage/main/post_update 11.64, stage/render/graph_submit 10.82, encode/pass:shadow_point_light/sum 10.73, stage/render/specialize 9.08, stage/render/queue 6.50, encode/submit 5.64 |
| measure | 1.68 | 30.8 | render/thread/ms 35.16, render/busy/ms 34.14, main/busy/ms 22.16, stage/main/post_update 11.63, stage/render/render 11.00, render/graph/ms 9.59, stage/render/specialize 8.92, encode//sum 7.77, stage/render/graph_submit 7.06, stage/render/queue 5.41, stage/main/pre_update 4.55, stage/main/update 4.37 |
| measure | 4.47 | 29.8 | render/thread/ms 36.07, render/busy/ms 35.16, main/busy/ms 18.80, stage/main/post_update 12.65, stage/render/render 10.84, stage/render/specialize 9.56, render/graph/ms 9.44, encode//sum 8.03, stage/render/graph_submit 7.00, stage/render/queue 6.13, encode/submit 4.10, stage/main/update 3.77 |
| measure | 7.08 | 30.0 | render/thread/ms 36.90, render/busy/ms 35.77, main/busy/ms 19.07, stage/render/render 12.57, stage/main/post_update 11.31, render/graph/ms 11.04, encode//sum 9.88, stage/render/specialize 9.21, stage/render/graph_submit 8.22, stage/render/queue 5.98, encode/submit 5.01, encode/pass:shadow_point_light/sum 4.98 |
| measure | 11.01 | 33.1 | render/thread/ms 40.57, render/busy/ms 39.41, main/busy/ms 18.17, stage/render/render 13.42, render/graph/ms 11.72, stage/main/post_update 11.62, encode//sum 11.14, stage/render/specialize 10.37, stage/render/graph_submit 8.35, stage/render/queue 6.70, encode/pass:shadow_point_light/sum 6.21, encode/submit 4.94 |
| measure | 15.72 | 29.6 | render/thread/ms 37.80, render/busy/ms 36.70, main/busy/ms 19.16, stage/main/post_update 12.35, stage/render/render 11.55, stage/render/specialize 10.30, render/graph/ms 10.06, stage/render/graph_submit 7.72, encode//sum 7.63, stage/render/queue 6.57, encode/submit 4.81, stage/main/update 4.17 |
| measure | 18.00 | 30.1 | render/thread/ms 35.95, render/busy/ms 34.80, main/busy/ms 17.73, stage/main/post_update 10.72, stage/render/render 10.41, stage/render/specialize 9.87, render/graph/ms 8.86, encode//sum 7.78, stage/render/graph_submit 6.71, stage/render/queue 6.28, stage/main/update 4.33, encode/submit 3.77 |
| measure | 19.37 | 30.0 | render/thread/ms 34.70, render/busy/ms 33.56, main/busy/ms 17.61, stage/main/post_update 11.25, stage/render/render 9.87, stage/render/specialize 9.31, render/graph/ms 8.35, encode//sum 7.72, stage/render/graph_submit 6.33, stage/render/queue 6.30, stage/main/update 4.05, encode/submit 3.48 |
| measure | 20.70 | 30.4 | render/thread/ms 35.56, render/busy/ms 34.42, main/busy/ms 18.25, stage/main/post_update 11.05, stage/render/render 10.41, stage/render/specialize 9.32, render/graph/ms 8.85, encode//sum 7.51, stage/render/graph_submit 6.91, stage/render/queue 6.59, stage/main/update 4.59, encode/submit 3.86 |
| measure | 23.00 | 33.3 | render/thread/ms 41.01, render/busy/ms 39.92, main/busy/ms 24.99, stage/main/post_update 12.52, stage/render/render 11.28, stage/render/specialize 11.15, render/graph/ms 9.71, encode//sum 8.28, stage/render/queue 8.01, stage/render/graph_submit 7.50, stage/main/update 6.83, encode/submit 4.14 |
| measure | 24.49 | 29.4 | render/thread/ms 37.99, render/busy/ms 36.94, main/busy/ms 20.77, stage/main/post_update 12.89, stage/render/render 11.75, stage/render/specialize 10.32, render/graph/ms 10.29, encode//sum 8.69, stage/render/graph_submit 7.76, stage/render/queue 6.32, stage/main/update 5.22, encode/submit 4.71 |
| measure | 27.28 | 33.0 | render/thread/ms 45.67, render/busy/ms 44.56, main/busy/ms 24.91, stage/render/render 16.75, render/graph/ms 15.16, stage/main/post_update 12.75, stage/render/graph_submit 11.83, stage/render/specialize 11.50, encode//sum 10.34, encode/pass:shadow_point_light/sum 9.33, encode/submit 7.54, stage/render/queue 7.38 |
| measure | 28.84 | 29.5 | render/thread/ms 40.11, render/busy/ms 38.98, main/busy/ms 20.28, stage/render/render 13.57, stage/main/post_update 13.07, render/graph/ms 12.09, stage/render/specialize 9.60, encode//sum 9.54, stage/render/graph_submit 9.18, encode/pass:shadow_point_light/sum 7.15, stage/render/queue 6.80, encode/submit 5.69 |
| measure | 30.48 | 34.0 | render/thread/ms 40.15, render/busy/ms 38.99, main/busy/ms 21.19, stage/render/render 12.44, stage/main/post_update 12.15, render/graph/ms 10.71, stage/render/specialize 10.46, encode//sum 8.43, stage/render/graph_submit 8.28, stage/render/queue 7.18, stage/main/update 5.75, encode/submit 4.64 |
| measure | 31.93 | 34.4 | render/thread/ms 40.92, render/busy/ms 39.85, main/busy/ms 20.39, stage/main/post_update 12.99, stage/render/render 12.52, stage/render/specialize 11.02, render/graph/ms 10.85, encode//sum 9.72, stage/render/graph_submit 8.14, stage/render/queue 7.06, encode/pass:shadow_point_light/sum 6.55, encode/submit 4.45 |
| measure | 35.35 | 31.3 | render/thread/ms 39.51, render/busy/ms 38.38, main/busy/ms 19.84, stage/render/render 12.37, stage/main/post_update 12.20, render/graph/ms 10.74, stage/render/specialize 10.45, encode//sum 8.51, stage/render/graph_submit 8.26, stage/render/queue 7.37, encode/submit 5.00, stage/main/update 4.29 |
| measure | 37.48 | 29.8 | render/thread/ms 39.18, render/busy/ms 38.22, main/busy/ms 19.48, stage/render/render 12.36, stage/main/post_update 11.97, render/graph/ms 10.97, stage/render/specialize 10.10, encode//sum 8.66, stage/render/graph_submit 8.46, stage/render/queue 6.96, encode/submit 4.98, encode/pass:shadow_point_light/sum 4.45 |
| measure | 40.79 | 31.0 | render/thread/ms 38.16, render/busy/ms 37.18, main/busy/ms 18.68, stage/main/post_update 10.97, stage/render/render 10.37, stage/render/specialize 10.02, render/graph/ms 9.02, stage/render/graph_submit 7.08, encode//sum 6.86, stage/render/queue 6.55, stage/main/update 4.49, encode/submit 3.95 |
| measure | 43.21 | 32.3 | render/thread/ms 37.24, render/busy/ms 36.25, main/busy/ms 17.08, stage/main/post_update 10.43, stage/render/specialize 10.41, stage/render/render 10.05, render/graph/ms 8.62, stage/render/graph_submit 6.81, stage/render/queue 6.78, encode//sum 6.27, stage/main/update 4.21, encode/submit 3.76 |
| measure | 45.64 | 35.0 | render/thread/ms 37.97, render/busy/ms 36.76, main/busy/ms 17.38, stage/render/specialize 10.57, stage/main/post_update 9.88, stage/render/render 9.84, render/graph/ms 8.37, encode//sum 7.19, stage/render/graph_submit 6.57, stage/render/queue 6.47, stage/main/update 4.67, stage/render/prepare_meshes 4.49 |
| measure | 56.44 | 30.2 | render/thread/ms 37.10, render/busy/ms 36.18, main/busy/ms 18.45, stage/render/render 10.02, stage/render/specialize 10.00, stage/main/post_update 9.07, render/graph/ms 8.83, encode//sum 7.26, stage/render/queue 7.08, stage/render/graph_submit 6.82, stage/main/update 4.12, stage/main/pre_update 3.76 |
| measure | 59.01 | 40.8 | render/thread/ms 54.22, render/busy/ms 52.76, main/busy/ms 31.56, stage/main/post_update 18.70, stage/render/render 17.01, render/graph/ms 15.01, stage/render/specialize 13.97, stage/render/graph_submit 11.74, encode//sum 11.08, stage/main/update 8.60, stage/render/prepare_meshes 6.74, encode/submit 6.34 |
| measure | 59.04 | 33.5 | render/thread/ms 57.86, render/busy/ms 56.80, main/busy/ms 25.82, stage/render/specialize 14.08, stage/render/prepare_assets 13.84, stage/main/post_update 13.68, stage/render/render 9.83, render/graph/ms 8.44, stage/render/prepare_meshes 7.58, encode//sum 7.21, stage/main/update 7.14, stage/render/graph_submit 6.36 |
| measure | 62.58 | 31.8 | render/thread/ms 38.29, render/busy/ms 37.29, main/busy/ms 19.77, stage/render/render 10.77, stage/render/specialize 10.51, stage/main/post_update 10.47, render/graph/ms 9.45, encode//sum 7.87, stage/render/graph_submit 7.29, stage/render/queue 6.61, stage/main/update 5.91, stage/render/prepare_meshes 4.35 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=base-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 0%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **78%** (min 58%) · 2622 MHz (min 2610) · memory 8101 MHz · 81.6 W · 78 °C (max 83) · VRAM 7062 MiB · throttled 100% of samples

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
