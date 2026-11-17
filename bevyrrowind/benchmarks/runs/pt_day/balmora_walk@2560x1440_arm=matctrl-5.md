# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); mw.matctrl.gate=on (command line: --set); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6081 frames over 65.6 s · **median 10.60 ms (94 fps)** · mean 10.80 · p95 13.45 · p99 16.52 (1% low 61 fps) · worst 44.01 ms · 77 spikes over 1.5× median · 57.6 stutters/min (work 56.7/min)
- warm-up: 323 frames over 9.5 s, worst 2654.0 ms
- hitches (> 26.5 ms): **33** measured, 46 in warm-up
- timed GPU passes: 6.31 ms of a 10.80 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.60 ms, limit 11 ms
- **FAIL** frame_ms: 63 stutters (> 1.5x moving median of 60) in 1.09 min = 57.58/min, limit 1/min
- **FAIL** worst frame 44.01 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.788 | 2.032 | 2.066 | 0.88 | 0.174 |
| main_opaque_pass_3d | 1.427 | 1.622 | 2.039 | 0.88 | 0.061 |
| early prepass | 0.430 | 0.489 | 0.718 | 0.88 | 0.088 |
| bloom | 0.276 | 0.314 | 0.319 | 0.88 | 0.062 |
| atmosphere_luts | 0.266 | 0.303 | 0.365 | 0.88 | 0.003 |
| water_velocity | 0.246 | 0.279 | 0.347 | 0.88 | 0.002 |
| water_sspr | 0.177 | 0.201 | 0.203 | 0.88 | 0.002 |
| ssao | 0.165 | 0.188 | 0.191 | 0.88 | 0.007 |
| auxiliary_views | 0.149 | 0.169 | 0.596 | 0.88 | 0.163 |
| atmos_froxel_inject | 0.128 | 0.145 | 0.147 | 0.88 | 0.001 |
| clouds_trace | 0.124 | 0.141 | 0.234 | 0.88 | 0.001 |
| water_fft | 0.116 | 0.131 | 0.138 | 0.88 | 0.003 |
| uniform_allocation | 0.112 | 0.128 | 0.182 | 0.88 | 0.024 |
| upscaling | 0.105 | 0.119 | 0.126 | 0.88 | 0.004 |
| clouds_composite | 0.093 | 0.105 | 0.109 | 0.88 | 0.002 |
| bin_unpacking | 0.092 | 0.104 | 0.142 | 0.88 | 0.018 |
| main_transmissive_pass_3d | 0.075 | 0.085 | 0.178 | 0.88 | 0.008 |
| tonemapping | 0.075 | 0.085 | 0.088 | 0.88 | 0.004 |
| clustering | 0.056 | 0.064 | 0.067 | 0.88 | 0.018 |
| render_sky | 0.045 | 0.051 | 0.052 | 0.88 | 0.002 |
| clouds_resolve | 0.043 | 0.048 | 0.049 | 0.88 | 0.001 |
| surface_exposure | 0.039 | 0.044 | 0.045 | 0.88 | 0.001 |
| surface_trail | 0.034 | 0.056 | 0.057 | 0.61 | 0.002 |
| surface_exposure_far | 0.033 | 0.042 | 0.045 | 0.78 | 0.002 |
| clouds_shadow | 0.027 | 0.031 | 0.033 | 0.88 | 0.003 |
| early_mesh_preprocessing | 0.026 | 0.030 | 0.047 | 0.88 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.029 | 0.032 | 0.88 | 0.001 |
| lightprobe_irradiance_map | 0.024 | 0.413 | 0.416 | 0.06 | 0.001 |
| water_snapshot | 0.022 | 0.025 | 0.028 | 0.88 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.88 | 0.002 |
| lightprobe_radiance_map | 0.013 | 0.219 | 0.230 | 0.06 | 0.004 |
| night_sky | 0.012 | 0.014 | 0.034 | 0.88 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.88 | 0.124 |
| clouds_fog_light | 0.010 | 0.012 | 0.012 | 0.88 | 0.002 |
| per_view_shadow_pass | 0.009 | 0.011 | 0.022 | 0.88 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.88 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.88 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.06 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.06 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.06 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.165 | 11.469 | 37.32 | 55730.8 |
| main/busy/ms | 8.987 | 10.743 | 29.35 | 54648.6 |
| render/busy/ms | 8.592 | 10.953 | 36.61 | 52250.1 |
| stage/main/post_update | 5.584 | 6.966 | 10.22 | 33956.8 |
| stage/render/render | 5.498 | 7.214 | 20.53 | 33430.9 |
| render/graph/ms | 4.734 | 6.437 | 19.86 | 28789.2 |
| encode//sum | 3.988 | 5.525 | 26.39 | 24250.0 |
| stage/render/graph_submit | 3.548 | 4.924 | 18.60 | 21577.3 |
| encode/submit | 2.285 | 3.330 | 17.54 | 13894.1 |
| stage/main/update | 2.147 | 2.573 | 22.52 | 13054.3 |
| encode/finish_wall | 1.352 | 1.945 | 4.49 | 8220.3 |
| stage/render/camera_driver | 1.133 | 1.578 | 3.89 | 6890.6 |
| encode/pass:shadow_point_light/sum | 0.962 | 3.399 | 16.14 | 5848.4 |
| encode/pass:shadow_directional_light/sum | 0.836 | 1.541 | 2.56 | 5082.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.754 | 1.076 | 1.86 | 4587.0 |
| stage/render/prepare_write_phase_buffers | 0.744 | 1.103 | 2.16 | 4526.6 |
| encode/pass:early prepass/sum | 0.727 | 1.070 | 1.56 | 4423.4 |
| bigspace/total/ms | 0.586 | 1.416 | 2.60 | 3562.4 |
| stage/render/prepare_assets | 0.516 | 0.671 | 11.41 | 3135.1 |
| render/present/ms | 0.477 | 1.221 | 2.41 | 2902.4 |
| stage/render/prepare_resources | 0.449 | 0.543 | 1.06 | 2729.1 |
| stage/render/prepare_meshes | 0.426 | 0.541 | 5.69 | 2587.6 |
| stage/main/first | 0.396 | 0.525 | 5.48 | 2408.8 |
| stage/main/pre_update | 0.355 | 0.532 | 2.67 | 2161.6 |
| terrain/update/ms | 0.308 | 0.397 | 1.57 | 1870.6 |
| stage/render/prepare_bind_groups | 0.275 | 0.476 | 2.45 | 1671.4 |
| terrain/select/ms | 0.275 | 0.356 | 0.90 | 1669.4 |
| stream/drive/ms | 0.262 | 0.350 | 20.24 | 1592.5 |
| controller/feed/ms | 0.257 | 0.321 | 0.86 | 1560.8 |
| stage/render/specialize | 0.251 | 0.321 | 11.98 | 1528.9 |
| bigspace/low_precision/ms | 0.241 | 0.980 | 1.97 | 1463.1 |
| encode/pass:main_transparent_pass_3d/sum | 0.234 | 0.346 | 0.90 | 1423.8 |
| stage/render/queue | 0.218 | 0.304 | 7.34 | 1325.8 |
| stage/main/fixed_main_loop | 0.214 | 0.370 | 1.22 | 1300.4 |
| stream/tick/ms | 0.203 | 0.278 | 0.56 | 1235.8 |
| stage/render/create_views | 0.196 | 0.253 | 1.13 | 1192.7 |
| far/select/ms | 0.195 | 0.258 | 0.68 | 1185.6 |
| stage/render/prepare_batch_phases | 0.189 | 0.267 | 0.60 | 1148.4 |
| anim/evaluate/ms | 0.179 | 0.461 | 1.71 | 1089.3 |
| bigspace/high_precision/ms | 0.178 | 0.910 | 2.17 | 1081.2 |
| stage/render/extract_commands | 0.170 | 0.199 | 0.41 | 1030.9 |
| stage/render/prepare_flush | 0.161 | 0.307 | 0.92 | 978.3 |
| anim/evaluate_cpu/ms | 0.152 | 0.190 | 0.76 | 926.1 |
| stage/render/prepare_views | 0.139 | 0.176 | 0.44 | 844.9 |
| controller/move/ms | 0.126 | 0.279 | 0.55 | 767.0 |
| stage/main/last | 0.114 | 0.160 | 1.59 | 695.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.097 | 0.162 | 0.60 | 589.1 |
| render/acquire/ms | 0.095 | 0.125 | 0.39 | 578.2 |
| stage/render/prepare_collect_phase_buffers | 0.061 | 0.128 | 0.88 | 370.2 |
| pipelines/compile_ms | 0.048 | 0.000 | 123.98 | 289.7 |
| physics/step/ms | 0.046 | 0.087 | 0.49 | 281.6 |
| pipelines/front_ms | 0.046 | 0.000 | 120.04 | 277.9 |
| stage/render/cleanup | 0.033 | 0.102 | 0.60 | 200.4 |
| stage/render/phase_sort | 0.033 | 0.058 | 0.26 | 199.6 |
| physics/insert/ms | 0.031 | 0.345 | 0.96 | 187.5 |
| encode/resolve_diagnostics/sum | 0.029 | 0.039 | 0.59 | 179.2 |
| weather/map/ms | 0.025 | 0.034 | 0.20 | 153.8 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.45 | 87.3 |
| anim/apply/ms | 0.012 | 0.021 | 0.42 | 71.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.30 | 65.5 |
| weather/eval/ms | 0.010 | 0.014 | 0.11 | 63.1 |
| controller/avoid/ms | 0.010 | 0.013 | 0.21 | 61.9 |
| script/update/ms | 0.007 | 0.010 | 0.13 | 44.6 |
| stream/despawn/ms | 0.006 | 0.000 | 1.13 | 35.9 |
| stream/spawn/ms | 0.005 | 0.000 | 1.09 | 31.0 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 30.9 |
| script/ui/ms | 0.005 | 0.006 | 0.46 | 29.1 |
| controller/write/ms | 0.003 | 0.004 | 0.17 | 20.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 13.3 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.94 | 11.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.47 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.15 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 26 | 42 | 13 |
| crowd/c0/joints/count | count | 1277 | 2218 | 594 |
| crowd/c0/meshes/count | count | 352 | 633 | 150 |
| crowd/c1/count | count | 146 | 157 | 148 |
| crowd/c1/joints/count | count | 5531 | 6037 | 5602 |
| crowd/c1/meshes/count | count | 1194 | 1311 | 1191 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4612 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4591 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.41 | 1.59 | 0.30 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.45 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.30 | 0.02 |
| encode/finish_critical | ms | 0.96 | 2.56 | 0.88 |
| encode/pass:early prepass/max | ms | 0.74 | 1.56 | 0.82 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 1.86 | 0.88 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.60 | 0.14 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 0.90 | 0.22 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.56 | 0.77 |
| encode/pass:shadow_point_light/max | ms | 0.81 | 1.55 | 0.93 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.59 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.19 | 36.61 | 8.13 |
| gpu/timed/ms | ms | 7.93 | 9.03 | 8.13 |
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
| phases/alpha_mask3d/items | count | 811 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 811 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1293 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3407 | 5130 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1293 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3407 | 5130 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 21 | 201 | 4 |
| phases/shadow/bins | count | 5022 | 8817 | 2399 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.3 k | 6346 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 201 | 252 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 188 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 12 | 563 | 6 |
| preprocess/bind_groups_created | count | 12 | 563 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 240 | 123 |
| process/cpu_usage | % | 26.57 | 39.81 | 21.63 |
| process/mem_usage | GiB | 5.49 | 5.66 | 5.66 |
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
| skin/blocks_changed/count | count | 14 | 125 | 10 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 369.9 k | 811.3 k | 505.6 k |
| skin/joints_written/count | count | 5780 | 12.7 k | 7900 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4916 | 5010 | 5010 |
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
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.41 | 52.11 | 32.61 |
| system/mem_usage | % | 55.38 | 55.91 | 55.87 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2900 | 3016 | 3016 |
| textures/images | count | 2680 | 2740 | 2740 |
| views/all/batch_sets | count | 72 | 250 | 54 |
| views/all/bins | count | 7583 | 11.7 k | 5897 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 248 | 299 | 74 |
| views/all/pipelines_distinct | count | 70 | 74 | 61 |
| views/all/work_items | count | 10.7 k | 35.8 k | 8119 |
| views/cascade/batch_sets | count | 9 | 53 | 4 |
| views/cascade/bins | count | 2796 | 6154 | 2399 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8911 | 23.4 k | 6346 |
| views/cascade/pipelines | count | 36 | 53 | 28 |
| views/cascade/work_items | count | 429 | 23.4 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 192 | 0 |
| views/lamp_face/bins | count | 2146 | 2881 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2406 | 3187 | 0 |
| views/lamp_face/pipelines | count | 166 | 201 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2872 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2886 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8426 | 13.0 k | 7610 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8588 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 58.40 | 257 | 355.1 k |
| anim/events/count | count | 0.08 | 2 | 496 |
| assets/Image/added | count | 0.10 | 34 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.24 | 11 | 7533 |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1218 | 4676 |
| encode//count | count | 34.16 | 220 | 207.7 k |
| encode/bind_walk/init_textures | count | 13.0 k | 114.7 k | 79.25 M |
| encode/bind_walk/init_textures_skipped | count | 65.0 k | 149.2 k | 395.36 M |
| encode/bind_walk/inits | count | 463 | 1127 | 2.81 M |
| encode/bind_walk/inits_skipped | count | 108 | 174 | 657.6 k |
| encode/bind_walk/merge_textures | count | 13.0 k | 114.6 k | 78.81 M |
| encode/bind_walk/merge_textures_skipped | count | 5366 | 6012 | 32.63 M |
| encode/bind_walk/merges | count | 99.63 | 334 | 605.8 k |
| encode/bind_walk/merges_skipped | count | 84.16 | 133 | 511.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6081 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6081 |
| encode/pass:early prepass/count | count | 1 | 1 | 6081 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6081 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6081 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6081 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6081 |
| encode/pass:shadow_point_light/count | count | 1.56 | 48 | 9461 |
| encode/queue/command_buffers | count | 45.73 | 278 | 278.1 k |
| encode/queue/submits | count | 2.02 | 4 | 12.3 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.89 M | 11.43 G |
| encode/queue/write_buffer/calls | count | 263 | 840 | 1.60 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.6 k | 3.52 M | 1.08 G |
| encode/queue/write_buffer_with/calls | count | 10.99 | 465 | 66.8 k |
| encode/queue/write_texture/bytes | bytes | 11.8 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6081 |
| physics/inserts | count | 0.37 | 111 | 2269 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 161 | 2335 |
| physics/steps | count | 0.65 | 3 | 3938 |
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
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 22 | 378 |
| stream/upload/bytes | bytes | 5567 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1221 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.5 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.65 | 2654.0 | main/busy/ms 2602, stage/main/post_update 2495, stage/main/update 98.80, surface/advance_cpu_ms 68.87, stage/main/pre_update 4.53, stage/main/last 1.39, surface/cell_upload_ms 0.84, stream/drive/ms 0.59, stage/main/first 0.53, stage/main/fixed_main_loop 0.37, weather/map/ms 0.25, stream/tick/ms 0.13 |
| warm-up | 3.39 | 736.1 | main/busy/ms 2761, stage/main/post_update 2527, render/thread/ms 714, render/busy/ms 548, stage/render/prepare_assets 500, stage/main/update 216, stage/render/prepare_views 162, render/acquire/ms 162, surface/advance_cpu_ms 68.87, stage/render/create_views 27.69, stage/render/render 13.22, render/graph/ms 8.54 |
| warm-up | 3.45 | 63.9 | render/thread/ms 759, render/busy/ms 592, stage/render/prepare_assets 525, main/busy/ms 180, stage/render/prepare_views 163, render/acquire/ms 162, stage/main/update 122, pipelines/compile_ms 67.84, pipelines/front_ms 63.31, stage/main/post_update 44.14, stage/render/create_views 28.39, terrain/update/ms 25.13 |
| warm-up | 3.53 | 79.1 | pipelines/compile_ms 146, pipelines/front_ms 135, render/thread/ms 124, stage/render/render 81.57, render/present/ms 61.52, render/busy/ms 61.49, pipelines/wait_ms 50.44, main/busy/ms 39.76, terrain/update/ms 32.12, stage/render/prepare_assets 25.61, stage/main/post_update 22.86, render/graph/ms 18.97 |
| warm-up | 3.67 | 138.2 | pipelines/wait_ms 6947, pipelines/compile_ms 1541, pipelines/front_ms 1204, pipelines/driver_ms 336, render/thread/ms 215, render/busy/ms 154, stage/render/render 80.75, render/present/ms 60.77, main/busy/ms 52.41, stage/render/prepare_resources 50.44, stage/render/prepare_write_phase_buffers 40.97, stage/main/post_update 36.96 |
| warm-up | 3.71 | 37.1 | pipelines/wait_ms 6898, pipelines/compile_ms 2031, pipelines/front_ms 1656, pipelines/driver_ms 375, render/thread/ms 171, render/busy/ms 169, main/busy/ms 114, stage/main/update 65.46, stage/render/prepare_resources 50.99, stage/render/prepare_write_phase_buffers 41.17, stage/main/post_update 37.00, stage/render/prepare_flush 25.86 |
| warm-up | 3.82 | 116.2 | pipelines/compile_ms 2218, pipelines/front_ms 2090, pipelines/wait_ms 517, pipelines/driver_ms 128, render/thread/ms 100, render/busy/ms 98.10, main/busy/ms 97.99, stage/main/update 67.46, stage/render/prepare_assets 66.27, stage/render/render 21.59, stage/main/post_update 20.73, render/graph/ms 18.86 |
| warm-up | 3.90 | 75.9 | pipelines/compile_ms 1906, pipelines/front_ms 1679, pipelines/wait_ms 520, pipelines/driver_ms 226, render/thread/ms 140, render/busy/ms 136, stage/render/prepare_assets 103, main/busy/ms 32.90, stage/render/render 28.92, render/graph/ms 25.25, stage/render/graph_submit 21.85, stage/main/post_update 17.17 |
| warm-up | 3.97 | 30.6 | pipelines/compile_ms 71.74, pipelines/front_ms 67.76, render/thread/ms 41.69, render/busy/ms 39.54, main/busy/ms 27.99, stage/main/post_update 16.25, stage/render/prepare_assets 15.30, stage/render/render 11.63, terrain/update/ms 9.90, render/graph/ms 8.98, encode//sum 8.34, stage/render/graph_submit 6.52 |
| warm-up | 4.07 | 79.7 | pipelines/compile_ms 1512, pipelines/front_ms 1460, render/thread/ms 60.13, render/busy/ms 58.30, main/busy/ms 55.05, pipelines/driver_ms 52.01, stage/main/update 42.23, stage/render/prepare_assets 37.28, stream/drive/ms 36.13, stage/render/render 14.26, render/graph/ms 11.96, encode//sum 6.95 |
| warm-up | 4.17 | 81.4 | pipelines/compile_ms 278, pipelines/front_ms 264, render/thread/ms 59.91, render/busy/ms 55.78, main/busy/ms 52.58, stage/main/update 40.83, stream/drive/ms 36.12, stage/render/render 29.72, render/graph/ms 25.04, stage/render/prepare_assets 22.94, stage/render/graph_submit 15.74, pipelines/driver_ms 14.12 |
| warm-up | 4.35 | 160.4 | pipelines/compile_ms 499, pipelines/front_ms 484, render/thread/ms 92.97, render/busy/ms 90.11, main/busy/ms 74.76, stage/render/prepare_assets 58.38, stage/main/update 41.51, stage/render/render 22.23, render/graph/ms 19.10, pipelines/wait_ms 15.79, pipelines/lock_ms 15.41, pipelines/driver_ms 15.21 |
| warm-up | 4.43 | 84.8 | pipelines/compile_ms 1618, pipelines/front_ms 1568, render/thread/ms 127, render/busy/ms 124, stage/render/prepare_assets 98.94, main/busy/ms 56.53, pipelines/driver_ms 49.84, stage/main/update 36.12, stage/render/render 17.01, stage/render/cleanup 15.38, render/graph/ms 13.88, stage/render/graph_submit 11.52 |
| warm-up | 4.47 | 38.6 | pipelines/compile_ms 1263, pipelines/front_ms 1221, render/thread/ms 83.38, render/busy/ms 81.78, main/busy/ms 53.91, stage/render/prepare_assets 53.09, pipelines/wait_ms 42.64, pipelines/driver_ms 41.91, stage/main/update 30.32, pipelines/lock_ms 15.51, stage/main/post_update 10.32, stage/render/render 9.72 |
| warm-up | 4.54 | 69.6 | pipelines/compile_ms 958, pipelines/front_ms 916, render/thread/ms 89.23, render/busy/ms 87.13, main/busy/ms 79.39, stage/render/prepare_assets 43.48, pipelines/driver_ms 41.55, pipelines/wait_ms 40.67, stage/main/post_update 37.45, stage/main/update 30.60, bigspace/total/ms 15.95, pipelines/lock_ms 13.63 |
| warm-up | 4.61 | 70.4 | pipelines/compile_ms 1313, pipelines/front_ms 1260, pipelines/wait_ms 159, render/thread/ms 115, render/busy/ms 113, main/busy/ms 64.08, pipelines/driver_ms 52.18, stage/render/prepare_assets 45.08, stage/main/post_update 38.29, stage/render/render 21.13, stage/render/prepare_write_phase_buffers 19.58, render/graph/ms 18.56 |
| warm-up | 4.69 | 77.7 | pipelines/compile_ms 1634, pipelines/front_ms 1522, pipelines/wait_ms 293, render/thread/ms 129, render/busy/ms 126, pipelines/driver_ms 113, stage/render/prepare_assets 39.81, main/busy/ms 37.01, stage/render/prepare_write_phase_buffers 33.34, stage/render/render 26.60, render/graph/ms 22.61, encode//sum 18.33 |
| warm-up | 4.78 | 89.6 | pipelines/compile_ms 2187, pipelines/front_ms 2050, pipelines/wait_ms 169, render/thread/ms 157, render/busy/ms 154, pipelines/driver_ms 137, main/busy/ms 73.55, stage/render/render 42.28, stage/render/prepare_assets 38.86, render/graph/ms 37.85, stage/main/post_update 37.40, encode//sum 31.76 |
| warm-up | 4.82 | 46.0 | pipelines/compile_ms 1506, pipelines/front_ms 1452, main/busy/ms 152, render/thread/ms 126, render/busy/ms 121, stage/main/update 97.09, pipelines/wait_ms 93.35, stream/drive/ms 56.85, pipelines/driver_ms 53.39, stage/render/render 47.64, render/graph/ms 41.15, encode//sum 36.93 |
| warm-up | 4.94 | 112.8 | pipelines/compile_ms 2103, pipelines/front_ms 2054, pipelines/wait_ms 131, main/busy/ms 121, stage/main/update 97.49, render/thread/ms 79.51, render/busy/ms 73.77, stream/drive/ms 57.02, pipelines/driver_ms 48.45, stage/render/prepare_assets 35.20, stage/render/render 25.70, encode//sum 21.20 |
| warm-up | 4.98 | 40.9 | pipelines/compile_ms 2249, pipelines/front_ms 2197, main/busy/ms 102, pipelines/wait_ms 76.53, render/thread/ms 70.31, stage/main/update 68.39, render/busy/ms 68.11, stream/drive/ms 54.99, pipelines/driver_ms 52.38, stage/render/prepare_assets 32.73, stage/main/post_update 16.88, stage/render/render 13.24 |
| warm-up | 5.09 | 110.3 | pipelines/compile_ms 935, pipelines/front_ms 909, pipelines/wait_ms 115, main/busy/ms 109, render/thread/ms 69.93, stage/main/update 69.86, render/busy/ms 68.20, stream/drive/ms 56.07, stage/render/prepare_assets 28.13, pipelines/driver_ms 26.08, stage/main/post_update 17.99, stage/render/render 15.04 |
| warm-up | 5.15 | 58.9 | pipelines/compile_ms 1488, pipelines/front_ms 1441, pipelines/wait_ms 110, main/busy/ms 93.67, render/thread/ms 77.97, render/busy/ms 76.30, stage/main/update 64.44, stage/render/prepare_assets 50.08, stream/drive/ms 49.75, pipelines/driver_ms 47.19, stage/main/post_update 14.42, stage/render/render 14.28 |
| warm-up | 5.21 | 64.4 | pipelines/compile_ms 2519, pipelines/front_ms 2455, main/busy/ms 91.21, stage/main/update 68.73, render/thread/ms 66.84, render/busy/ms 65.48, pipelines/driver_ms 64.36, stream/drive/ms 48.94, stage/render/prepare_assets 45.12, pipelines/wait_ms 15.32, stage/main/post_update 15.10, stage/render/render 11.12 |
| warm-up | 5.25 | 42.2 | pipelines/compile_ms 1406, pipelines/front_ms 1375, main/busy/ms 79.84, render/thread/ms 58.66, stage/main/update 58.62, render/busy/ms 57.16, stream/drive/ms 33.05, pipelines/driver_ms 30.90, stage/render/prepare_assets 27.94, stage/render/render 19.31, render/graph/ms 17.26, stage/main/post_update 14.87 |
| warm-up | 5.33 | 77.8 | pipelines/compile_ms 120, pipelines/front_ms 114, main/busy/ms 87.23, render/thread/ms 76.33, render/busy/ms 73.90, stage/main/update 58.50, stream/drive/ms 33.16, stage/render/render 25.66, render/graph/ms 22.38, stage/main/post_update 21.93, stage/render/prepare_assets 21.17, stage/render/graph_submit 18.68 |
| warm-up | 5.46 | 125.7 | pipelines/compile_ms 526, pipelines/front_ms 416, render/thread/ms 162, pipelines/driver_ms 110, render/busy/ms 93.74, stage/render/render 82.29, main/busy/ms 78.00, render/present/ms 67.51, stage/main/post_update 41.23, stage/main/update 23.67, stage/render/prepare_assets 18.91, bigspace/total/ms 16.98 |
| warm-up | 5.51 | 31.5 | pipelines/compile_ms 2764, pipelines/front_ms 2378, pipelines/driver_ms 386, pipelines/wait_ms 81.97, render/thread/ms 41.85, render/busy/ms 40.46, main/busy/ms 38.76, stage/main/update 20.74, stage/render/render 11.87, stage/main/post_update 11.60, pipelines/lock_ms 11.00, render/graph/ms 9.88 |
| warm-up | 5.59 | 31.3 | pipelines/compile_ms 142, pipelines/front_ms 137, pipelines/wait_ms 72.05, render/thread/ms 40.95, render/busy/ms 39.59, main/busy/ms 28.82, stage/render/prepare_assets 14.96, stage/main/update 14.36, stage/render/render 13.69, render/graph/ms 11.97, stage/main/post_update 9.51, stage/render/graph_submit 9.26 |
| warm-up | 5.62 | 30.6 | pipelines/compile_ms 239, pipelines/front_ms 229, pipelines/wait_ms 73.90, render/thread/ms 50.78, render/busy/ms 49.25, main/busy/ms 33.16, stage/render/prepare_assets 19.09, stage/render/render 15.28, stage/main/update 14.40, render/graph/ms 13.41, stage/main/post_update 12.86, stage/render/graph_submit 10.60 |
| warm-up | 5.76 | 34.6 | pipelines/compile_ms 191, pipelines/front_ms 184, render/thread/ms 41.85, main/busy/ms 40.27, render/busy/ms 40.05, stage/main/update 23.90, stage/render/prepare_assets 20.34, stream/drive/ms 17.70, stage/render/render 13.12, render/graph/ms 10.70, stage/main/post_update 10.62, stage/render/graph_submit 8.10 |
| warm-up | 5.95 | 35.1 | render/thread/ms 46.72, render/busy/ms 45.74, stage/render/prepare_assets 29.48, main/busy/ms 27.13, stage/main/update 10.72, stage/main/post_update 9.79, stage/render/render 8.39, encode//sum 7.73, render/graph/ms 6.92, stage/render/graph_submit 4.70, stream/drive/ms 3.69, encode/submit 2.81 |
| warm-up | 6.47 | 32.0 | pipelines/compile_ms 148, pipelines/front_ms 143, render/thread/ms 44.57, render/busy/ms 43.41, main/busy/ms 32.39, stage/render/prepare_assets 18.30, stage/main/update 12.83, stage/main/post_update 10.47, stage/render/render 10.00, render/graph/ms 8.41, encode//sum 8.20, stage/render/graph_submit 6.08 |
| warm-up | 6.68 | 31.6 | pipelines/compile_ms 149, pipelines/front_ms 144, render/thread/ms 41.72, main/busy/ms 41.49, render/busy/ms 40.22, stage/main/update 22.01, stage/render/render 14.79, stage/main/post_update 13.87, render/graph/ms 12.65, encode//sum 8.94, stage/render/graph_submit 8.75, stage/render/prepare_assets 7.52 |
| warm-up | 6.71 | 31.4 | render/thread/ms 51.45, render/busy/ms 49.60, main/busy/ms 48.81, stage/main/update 21.31, stage/main/post_update 18.52, stage/render/render 14.94, stage/render/prepare_assets 12.47, render/graph/ms 12.27, encode//sum 9.49, stage/render/graph_submit 8.35, stream/drive/ms 7.71, stage/render/prepare_write_phase_buffers 4.77 |
| warm-up | 6.74 | 31.1 | pipelines/compile_ms 97.05, pipelines/front_ms 91.19, render/thread/ms 49.41, render/busy/ms 47.27, main/busy/ms 47.11, stage/main/update 20.65, stage/main/post_update 16.86, stage/render/prepare_assets 15.58, stage/render/render 13.87, render/graph/ms 10.90, encode//sum 9.24, stage/render/graph_submit 7.13 |
| warm-up | 6.82 | 33.9 | pipelines/compile_ms 180, pipelines/front_ms 173, main/busy/ms 46.36, render/thread/ms 32.91, render/busy/ms 31.24, stage/main/update 25.98, stream/drive/ms 15.32, stage/render/render 14.78, stage/main/post_update 14.18, render/graph/ms 12.16, encode//sum 11.80, stage/render/graph_submit 8.72 |
| warm-up | 6.88 | 32.5 | main/busy/ms 45.43, render/thread/ms 43.24, render/busy/ms 41.48, stage/main/update 23.54, pipelines/compile_ms 21.73, pipelines/front_ms 20.67, stage/main/post_update 14.60, stage/render/render 13.86, stage/render/prepare_assets 12.45, render/graph/ms 11.46, encode//sum 9.00, stream/drive/ms 8.14 |
| warm-up | 6.91 | 29.9 | render/thread/ms 42.28, render/busy/ms 40.94, pipelines/compile_ms 36.66, main/busy/ms 35.34, pipelines/front_ms 35.19, stage/main/update 18.37, stage/render/prepare_assets 18.23, stage/main/post_update 11.33, stage/render/render 10.76, render/graph/ms 9.02, encode//sum 7.76, stage/render/graph_submit 6.65 |
| warm-up | 7.32 | 48.3 | render/thread/ms 53.89, render/busy/ms 52.35, main/busy/ms 34.51, pipelines/compile_ms 28.18, pipelines/front_ms 26.89, stage/render/render 14.40, stage/main/update 13.77, stage/render/prepare_assets 12.34, render/graph/ms 12.13, stage/main/post_update 11.77, stage/render/specialize 10.19, encode//sum 10.06 |
| warm-up | 8.69 | 32.5 | render/thread/ms 37.12, render/busy/ms 35.95, pipelines/compile_ms 23.43, pipelines/front_ms 22.46, main/busy/ms 18.75, stage/main/post_update 11.26, stage/render/render 10.70, stage/render/specialize 9.81, render/graph/ms 9.16, encode//sum 7.03, stage/render/graph_submit 7.02, stage/render/queue 6.68 |
| measure | 0.43 | 36.0 | render/thread/ms 39.41, render/busy/ms 37.83, encode//sum 21.78, main/busy/ms 18.51, stage/render/render 14.25, render/graph/ms 12.20, stage/main/post_update 11.43, stage/render/graph_submit 9.67, stage/render/specialize 9.57, stage/render/queue 5.96, encode/submit 5.55, stage/main/update 4.43 |
| measure | 1.42 | 32.0 | render/thread/ms 34.54, render/busy/ms 33.39, stage/render/render 24.12, render/graph/ms 22.62, stage/render/graph_submit 20.26, encode/submit 18.32, main/busy/ms 18.09, stage/main/post_update 11.71, encode//sum 7.40, stage/main/update 4.08, encode/pass:shadow_point_light/sum 3.10, stage/render/camera_driver 2.24 |
| measure | 1.74 | 30.6 | render/thread/ms 34.88, render/busy/ms 33.85, main/busy/ms 18.95, stage/main/post_update 10.58, stage/render/render 10.34, stage/render/specialize 9.10, render/graph/ms 8.71, encode//sum 7.01, stage/render/graph_submit 6.64, stage/render/queue 5.72, stage/main/update 5.00, stage/render/prepare_meshes 4.21 |
| measure | 3.09 | 30.2 | render/thread/ms 34.78, render/busy/ms 33.65, main/busy/ms 18.65, stage/main/post_update 12.43, stage/render/render 10.67, stage/render/specialize 9.33, render/graph/ms 9.04, encode//sum 7.20, stage/render/graph_submit 6.82, stage/render/queue 5.93, stage/main/update 3.92, encode/submit 3.84 |
| measure | 4.44 | 30.4 | render/thread/ms 36.13, render/busy/ms 35.21, main/busy/ms 20.38, stage/main/post_update 13.45, stage/render/render 11.57, render/graph/ms 10.12, stage/render/specialize 9.27, encode//sum 9.09, stage/render/graph_submit 7.61, stage/render/queue 5.97, encode/submit 4.47, encode/pass:shadow_point_light/sum 3.96 |
| measure | 5.77 | 30.9 | render/thread/ms 35.85, render/busy/ms 34.56, main/busy/ms 21.09, stage/main/post_update 13.10, stage/render/render 10.99, stage/render/specialize 9.91, render/graph/ms 9.29, encode//sum 7.97, stage/render/graph_submit 7.18, stage/render/queue 6.13, stage/main/update 4.51, encode/submit 4.02 |
| measure | 15.74 | 32.2 | render/thread/ms 37.07, render/busy/ms 36.01, main/busy/ms 18.34, stage/render/render 11.85, stage/main/post_update 11.54, render/graph/ms 10.23, stage/render/specialize 10.19, encode//sum 8.65, stage/render/graph_submit 7.84, stage/render/queue 6.11, encode/submit 4.46, stage/main/update 3.99 |
| measure | 17.10 | 31.8 | render/thread/ms 39.48, render/busy/ms 38.36, main/busy/ms 21.49, stage/render/render 13.88, render/graph/ms 12.36, stage/main/post_update 11.85, encode//sum 11.74, stage/render/specialize 9.65, stage/render/graph_submit 9.42, stage/render/queue 6.40, encode/pass:shadow_point_light/sum 6.15, encode/submit 5.57 |
| measure | 18.19 | 32.2 | main/busy/ms 39.12, stage/main/update 24.75, render/thread/ms 21.39, stream/drive/ms 20.54, render/busy/ms 20.33, stage/render/render 12.99, stage/main/post_update 11.42, render/graph/ms 11.41, encode//sum 9.67, stage/render/graph_submit 8.63, encode/submit 5.65, encode/pass:shadow_point_light/sum 5.14 |
| measure | 20.83 | 33.4 | render/thread/ms 35.28, render/busy/ms 34.33, main/busy/ms 19.88, stage/main/post_update 11.26, stage/render/render 10.93, render/graph/ms 9.60, encode//sum 9.48, stage/render/specialize 9.43, stage/render/graph_submit 7.22, stage/render/queue 6.39, stage/main/update 4.55, encode/submit 4.44 |
| measure | 23.01 | 33.7 | render/thread/ms 41.31, render/busy/ms 40.24, main/busy/ms 24.85, stage/main/post_update 12.89, stage/render/render 12.37, stage/render/specialize 11.76, encode//sum 11.00, render/graph/ms 10.89, stage/render/graph_submit 8.51, stage/render/queue 7.38, stage/main/update 6.53, encode/submit 5.06 |
| measure | 24.52 | 32.3 | render/thread/ms 37.91, render/busy/ms 36.70, main/busy/ms 21.39, stage/main/post_update 13.22, stage/render/render 12.30, render/graph/ms 10.80, encode//sum 9.68, stage/render/specialize 9.56, stage/render/graph_submit 8.38, stage/render/queue 6.84, encode/submit 5.11, stage/main/update 4.54 |
| measure | 27.28 | 36.8 | render/thread/ms 46.15, render/busy/ms 45.05, main/busy/ms 27.19, stage/render/render 17.28, render/graph/ms 15.65, encode//sum 15.10, stage/main/post_update 14.23, stage/render/graph_submit 12.41, stage/render/specialize 11.34, encode/pass:shadow_point_light/sum 9.27, encode/submit 7.61, stage/render/queue 7.06 |
| measure | 28.74 | 31.9 | render/thread/ms 41.03, render/busy/ms 40.04, main/busy/ms 20.52, stage/render/render 14.21, render/graph/ms 12.67, stage/main/post_update 12.09, encode//sum 11.07, stage/render/specialize 9.63, stage/render/graph_submit 9.28, encode/pass:shadow_point_light/sum 6.81, stage/render/queue 6.34, encode/submit 5.89 |
| measure | 30.49 | 34.9 | render/thread/ms 41.64, render/busy/ms 40.42, main/busy/ms 20.39, stage/render/render 13.64, stage/main/post_update 12.39, render/graph/ms 11.93, stage/render/specialize 10.40, encode//sum 9.72, stage/render/graph_submit 9.48, stage/render/queue 7.54, encode/submit 5.32, stage/main/update 5.18 |
| measure | 31.94 | 30.9 | render/thread/ms 39.29, render/busy/ms 38.28, main/busy/ms 18.65, stage/render/render 13.79, render/graph/ms 12.26, stage/main/post_update 11.62, encode//sum 11.57, stage/render/specialize 9.74, stage/render/graph_submit 9.47, stage/render/queue 6.29, encode/submit 5.61, encode/pass:shadow_point_light/sum 5.30 |
| measure | 35.36 | 33.3 | render/thread/ms 39.61, render/busy/ms 38.44, main/busy/ms 19.74, stage/render/render 12.65, stage/main/post_update 12.56, render/graph/ms 11.02, stage/render/specialize 10.48, encode//sum 8.89, stage/render/graph_submit 8.49, stage/render/queue 6.82, encode/submit 4.82, stage/main/update 4.44 |
| measure | 37.48 | 30.4 | render/thread/ms 36.43, render/busy/ms 35.35, main/busy/ms 20.02, stage/main/post_update 13.21, stage/render/render 12.15, render/graph/ms 10.57, encode//sum 9.85, stage/render/specialize 9.36, stage/render/graph_submit 8.15, stage/render/queue 5.97, encode/submit 4.75, stage/main/update 4.11 |
| measure | 43.19 | 30.6 | render/thread/ms 37.03, render/busy/ms 35.95, main/busy/ms 17.33, stage/render/render 10.66, stage/main/post_update 10.62, stage/render/specialize 10.07, render/graph/ms 9.19, encode//sum 7.47, stage/render/graph_submit 7.34, stage/render/queue 6.68, encode/submit 4.21, stage/main/update 3.92 |
| measure | 45.65 | 33.4 | render/thread/ms 38.72, render/busy/ms 37.74, main/busy/ms 17.07, stage/render/render 10.81, stage/render/specialize 10.23, stage/main/post_update 9.91, render/graph/ms 9.54, stage/render/graph_submit 7.55, encode//sum 7.22, stage/render/queue 6.31, stage/main/update 4.65, stage/render/prepare_meshes 4.32 |
| measure | 59.02 | 44.0 | render/thread/ms 54.88, render/busy/ms 53.28, main/busy/ms 29.87, stage/main/post_update 17.45, stage/render/render 17.35, render/graph/ms 15.29, stage/render/specialize 12.26, stage/render/graph_submit 11.75, encode//sum 10.55, stage/main/update 8.18, stage/render/queue 6.88, encode/submit 6.87 |
| measure | 59.06 | 34.4 | render/thread/ms 57.53, render/busy/ms 56.37, main/busy/ms 24.00, stage/render/prepare_assets 15.75, stage/main/post_update 12.62, stage/render/specialize 12.21, stage/render/render 9.86, render/graph/ms 8.49, encode//sum 7.67, stage/render/queue 6.85, stage/render/prepare_meshes 6.78, stage/render/graph_submit 6.38 |
| measure | 62.57 | 30.7 | render/thread/ms 38.58, render/busy/ms 37.59, main/busy/ms 18.45, stage/render/render 11.49, stage/render/specialize 10.47, render/graph/ms 10.22, stage/main/post_update 9.85, stage/render/graph_submit 8.07, encode//sum 7.62, stage/render/queue 6.51, stage/main/update 5.92, encode/submit 4.38 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set mw.matctrl.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=matctrl-5 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 10%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (250 samples): GPU load **78%** (min 41%) · 2624 MHz (min 2610) · memory 8101 MHz · 80.8 W · 76 °C (max 82) · VRAM 7087 MiB · throttled 100% of samples

## Tunables

- `far.handoff_scale=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.bias=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_distance_m=24 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_px=2 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `mw.matctrl.gate=on (command line: --set)`
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
