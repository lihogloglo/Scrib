# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.skin.skip_unchanged=on (command line: --set); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6102 frames over 65.6 s · **median 10.50 ms (95 fps)** · mean 10.76 · p95 13.29 · p99 16.30 (1% low 61 fps) · worst 45.51 ms · 77 spikes over 1.5× median · 57.6 stutters/min (work 50.3/min)
- warm-up: 322 frames over 9.1 s, worst 2554.9 ms
- hitches (> 26.2 ms): **33** measured, 45 in warm-up
- timed GPU passes: 6.57 ms of a 10.76 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.50 ms, limit 11 ms
- **FAIL** frame_ms: 63 stutters (> 1.5x moving median of 60) in 1.09 min = 57.58/min, limit 1/min
- **FAIL** worst frame 45.51 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.855 | 2.036 | 2.072 | 0.91 | 0.176 |
| main_opaque_pass_3d | 1.481 | 1.625 | 2.043 | 0.91 | 0.066 |
| early prepass | 0.447 | 0.490 | 0.718 | 0.91 | 0.086 |
| bloom | 0.286 | 0.314 | 0.321 | 0.91 | 0.062 |
| atmosphere_luts | 0.277 | 0.304 | 0.366 | 0.91 | 0.003 |
| water_velocity | 0.254 | 0.279 | 0.347 | 0.91 | 0.002 |
| water_sspr | 0.191 | 0.210 | 0.211 | 0.91 | 0.002 |
| ssao | 0.171 | 0.188 | 0.193 | 0.91 | 0.007 |
| auxiliary_views | 0.143 | 0.157 | 0.532 | 0.91 | 0.156 |
| atmos_froxel_inject | 0.133 | 0.146 | 0.148 | 0.91 | 0.001 |
| clouds_trace | 0.128 | 0.140 | 0.230 | 0.91 | 0.001 |
| water_fft | 0.119 | 0.130 | 0.137 | 0.91 | 0.003 |
| uniform_allocation | 0.118 | 0.129 | 0.182 | 0.91 | 0.025 |
| upscaling | 0.106 | 0.117 | 0.124 | 0.91 | 0.004 |
| clouds_composite | 0.096 | 0.106 | 0.109 | 0.91 | 0.002 |
| bin_unpacking | 0.095 | 0.104 | 0.142 | 0.91 | 0.018 |
| main_transmissive_pass_3d | 0.078 | 0.086 | 0.179 | 0.91 | 0.008 |
| tonemapping | 0.078 | 0.085 | 0.089 | 0.91 | 0.003 |
| clustering | 0.058 | 0.064 | 0.067 | 0.91 | 0.018 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.91 | 0.002 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.91 | 0.001 |
| lightprobe_irradiance_map | 0.042 | 0.412 | 0.417 | 0.10 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.91 | 0.001 |
| surface_trail | 0.036 | 0.056 | 0.057 | 0.63 | 0.002 |
| surface_exposure_far | 0.034 | 0.042 | 0.045 | 0.81 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.91 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.031 | 0.047 | 0.91 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.031 | 0.91 | 0.001 |
| water_snapshot | 0.022 | 0.025 | 0.028 | 0.91 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.220 | 0.230 | 0.10 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.91 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.91 | 0.001 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.026 | 0.91 | 0.121 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.91 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.91 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.91 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.91 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.275 | 11.559 | 38.67 | 56598.9 |
| main/busy/ms | 8.932 | 10.648 | 28.81 | 54501.4 |
| render/busy/ms | 8.723 | 11.039 | 38.18 | 53230.0 |
| stage/main/post_update | 5.551 | 6.931 | 10.73 | 33873.9 |
| stage/render/render | 5.408 | 6.905 | 16.21 | 32996.6 |
| render/graph/ms | 4.667 | 6.169 | 15.12 | 28480.2 |
| encode//sum | 3.687 | 4.830 | 27.53 | 22497.1 |
| stage/render/graph_submit | 3.482 | 4.653 | 11.20 | 21247.2 |
| encode/submit | 2.263 | 3.212 | 6.79 | 13809.2 |
| stage/main/update | 2.123 | 2.528 | 22.09 | 12956.4 |
| encode/finish_wall | 1.296 | 1.879 | 4.66 | 7905.7 |
| stage/render/camera_driver | 1.132 | 1.583 | 3.88 | 6910.2 |
| encode/pass:shadow_point_light/sum | 0.880 | 3.130 | 16.57 | 5370.8 |
| encode/pass:shadow_directional_light/sum | 0.811 | 1.467 | 2.35 | 4948.5 |
| stage/render/prepare_write_phase_buffers | 0.764 | 1.095 | 2.50 | 4662.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.736 | 1.036 | 1.66 | 4491.7 |
| encode/pass:early prepass/sum | 0.708 | 1.016 | 1.60 | 4319.5 |
| stage/render/prepare_assets | 0.563 | 0.729 | 10.24 | 3435.2 |
| bigspace/total/ms | 0.543 | 1.284 | 2.79 | 3314.3 |
| render/present/ms | 0.457 | 1.186 | 2.13 | 2788.9 |
| stage/render/prepare_resources | 0.444 | 0.537 | 1.17 | 2709.3 |
| stage/render/prepare_meshes | 0.429 | 0.537 | 7.35 | 2617.4 |
| stage/main/first | 0.393 | 0.516 | 5.31 | 2395.9 |
| stage/main/pre_update | 0.359 | 0.530 | 3.67 | 2190.5 |
| stage/render/specialize | 0.321 | 0.414 | 11.82 | 1958.6 |
| terrain/update/ms | 0.303 | 0.387 | 2.15 | 1851.2 |
| stage/render/queue | 0.287 | 0.412 | 7.42 | 1752.4 |
| stage/render/prepare_bind_groups | 0.277 | 0.487 | 2.52 | 1687.4 |
| terrain/select/ms | 0.270 | 0.345 | 0.96 | 1646.6 |
| controller/feed/ms | 0.262 | 0.327 | 0.80 | 1595.7 |
| stream/drive/ms | 0.241 | 0.319 | 20.02 | 1473.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.230 | 0.337 | 0.96 | 1405.6 |
| bigspace/low_precision/ms | 0.230 | 0.765 | 1.77 | 1405.0 |
| stage/main/fixed_main_loop | 0.212 | 0.369 | 1.09 | 1295.9 |
| far/select/ms | 0.195 | 0.259 | 0.80 | 1192.8 |
| stage/render/create_views | 0.195 | 0.253 | 1.01 | 1190.9 |
| stage/render/prepare_batch_phases | 0.188 | 0.267 | 0.69 | 1144.7 |
| stream/tick/ms | 0.186 | 0.250 | 0.50 | 1132.0 |
| stage/render/extract_commands | 0.170 | 0.199 | 0.52 | 1037.3 |
| stage/render/prepare_flush | 0.160 | 0.311 | 1.06 | 978.9 |
| anim/evaluate/ms | 0.159 | 0.266 | 1.65 | 973.2 |
| bigspace/high_precision/ms | 0.152 | 0.609 | 1.93 | 926.2 |
| anim/evaluate_cpu/ms | 0.151 | 0.187 | 0.62 | 919.8 |
| controller/move/ms | 0.146 | 0.352 | 0.60 | 890.5 |
| stage/render/prepare_views | 0.138 | 0.173 | 0.54 | 843.3 |
| stage/main/last | 0.117 | 0.160 | 3.05 | 716.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.096 | 0.154 | 0.79 | 584.3 |
| render/acquire/ms | 0.095 | 0.122 | 0.51 | 580.0 |
| stage/render/prepare_collect_phase_buffers | 0.061 | 0.127 | 0.56 | 369.2 |
| pipelines/compile_ms | 0.048 | 0.000 | 121.40 | 292.0 |
| physics/step/ms | 0.047 | 0.087 | 0.61 | 285.5 |
| pipelines/front_ms | 0.046 | 0.000 | 117.47 | 280.1 |
| stage/render/cleanup | 0.033 | 0.099 | 0.80 | 199.6 |
| encode/resolve_diagnostics/sum | 0.031 | 0.041 | 0.59 | 187.1 |
| physics/insert/ms | 0.031 | 0.338 | 0.87 | 187.0 |
| stage/render/phase_sort | 0.029 | 0.050 | 0.49 | 175.8 |
| weather/map/ms | 0.025 | 0.033 | 0.23 | 153.2 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.46 | 90.4 |
| anim/apply/ms | 0.011 | 0.019 | 0.12 | 69.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.29 | 67.9 |
| controller/avoid/ms | 0.010 | 0.014 | 0.22 | 63.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.06 | 62.3 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 42.9 |
| script/input/ms | 0.005 | 0.006 | 0.20 | 33.0 |
| stream/despawn/ms | 0.005 | 0.000 | 1.01 | 30.2 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 28.4 |
| script/ui/ms | 0.005 | 0.006 | 0.19 | 28.1 |
| controller/write/ms | 0.004 | 0.004 | 0.05 | 21.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 14.1 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.96 | 11.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 6.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.25 | 3.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.13 | 0.3 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 593 |
| crowd/c0/meshes/count | count | 353 | 646 | 156 |
| crowd/c1/count | count | 146 | 155 | 147 |
| crowd/c1/joints/count | count | 5531 | 5892 | 5571 |
| crowd/c1/meshes/count | count | 1181 | 1255 | 1177 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 636 | 6760 | 667 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.39 | 1.65 | 0.56 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.46 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.29 | 0.01 |
| encode/finish_critical | ms | 0.93 | 2.35 | 0.92 |
| encode/pass:early prepass/max | ms | 0.73 | 1.60 | 0.92 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.75 | 1.66 | 0.83 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.79 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 0.96 | 0.21 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 2.35 | 0.79 |
| encode/pass:shadow_point_light/max | ms | 0.78 | 1.42 | 0.90 |
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
| frame/work/ms | ms | 9.11 | 38.18 | 9.05 |
| gpu/timed/ms | ms | 7.92 | 11.26 | 8.27 |
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
| phases/alpha_mask3d/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1290 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3404 | 5117 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1290 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3404 | 5117 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 21 | 232 | 4 |
| phases/shadow/bins | count | 5024 | 8855 | 2405 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6352 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 203 | 248 | 29 |
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
| preprocess/bind_groups | count | 12 | 623 | 6 |
| preprocess/bind_groups_created | count | 12 | 623 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 228 | 123 |
| process/cpu_usage | % | 26.44 | 40.28 | 22.33 |
| process/mem_usage | GiB | 5.50 | 5.66 | 5.66 |
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
| skin/blocks_changed/count | count | 14 | 122 | 15 |
| skin/blocks_skipped/count | count | 185 | 237 | 223 |
| skin/blocks_walked/count | count | 13 | 18 | 15 |
| skin/joint_bytes_written/bytes | bytes | 42.8 k | 810.2 k | 44.0 k |
| skin/joints_written/count | count | 668 | 12.7 k | 688 |
| skin/upload_writes/count | count | 7 | 35 | 6 |
| started_load_count |  | 4917 | 5010 | 5010 |
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
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.85 | 39.86 | 29.24 |
| system/mem_usage | % | 54.56 | 55.16 | 55.09 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 69 | 279 | 54 |
| views/all/bins | count | 7592 | 11.7 k | 5903 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.4 k | 14.0 k |
| views/all/pipelines | count | 248 | 294 | 75 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.7 k | 34.2 k | 8119 |
| views/cascade/batch_sets | count | 9 | 50 | 4 |
| views/cascade/bins | count | 2804 | 6187 | 2405 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8899 | 23.5 k | 6352 |
| views/cascade/pipelines | count | 36 | 53 | 29 |
| views/cascade/work_items | count | 403 | 21.9 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 195 | 0 |
| views/lamp_face/bins | count | 2144 | 2826 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2402 | 3112 | 0 |
| views/lamp_face/pipelines | count | 167 | 201 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2949 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2874 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8425 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8586 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 55.63 | 232 | 339.5 k |
| anim/events/count | count | 0.09 | 3 | 527 |
| assets/Image/added | count | 0.10 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.79 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.55 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1260 | 4675 |
| encode//count | count | 33.82 | 220 | 206.4 k |
| encode/bind_walk/init_textures | count | 12.8 k | 119.2 k | 78.14 M |
| encode/bind_walk/init_textures_skipped | count | 68.5 k | 154.2 k | 418.27 M |
| encode/bind_walk/inits | count | 459 | 1215 | 2.80 M |
| encode/bind_walk/inits_skipped | count | 110 | 163 | 670.8 k |
| encode/bind_walk/merge_textures | count | 12.7 k | 119.1 k | 77.70 M |
| encode/bind_walk/merge_textures_skipped | count | 5366 | 6012 | 32.74 M |
| encode/bind_walk/merges | count | 99.08 | 356 | 604.6 k |
| encode/bind_walk/merges_skipped | count | 84.50 | 132 | 515.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6102 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6102 |
| encode/pass:early prepass/count | count | 1 | 1 | 6102 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6102 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6102 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6102 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6102 |
| encode/pass:shadow_point_light/count | count | 1.47 | 48 | 8973 |
| encode/queue/command_buffers | count | 45.31 | 278 | 276.5 k |
| encode/queue/submits | count | 2.02 | 4 | 12.3 k |
| encode/queue/write_buffer/bytes | bytes | 1.54 M | 4.44 M | 9.41 G |
| encode/queue/write_buffer/calls | count | 265 | 933 | 1.61 M |
| encode/queue/write_buffer_with/bytes | bytes | 179.0 k | 3.52 M | 1.09 G |
| encode/queue/write_buffer_with/calls | count | 10.98 | 465 | 67.0 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6102 |
| physics/inserts | count | 0.37 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2335 |
| physics/steps | count | 0.65 | 3 | 3939 |
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
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5548 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1216 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.2 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2554.9 | main/busy/ms 2524, stage/main/post_update 2422, stage/main/update 93.28, surface/advance_cpu_ms 64.78, stage/main/pre_update 4.36, stage/main/last 1.26, surface/cell_upload_ms 0.78, stage/main/first 0.47, stream/drive/ms 0.43, stage/main/fixed_main_loop 0.37, weather/map/ms 0.12, stream/tick/ms 0.11 |
| warm-up | 3.33 | 770.3 | main/busy/ms 2670, stage/main/post_update 2448, render/thread/ms 730, render/busy/ms 542, stage/render/prepare_assets 491, stage/main/update 207, stage/render/prepare_views 182, render/acquire/ms 182, surface/advance_cpu_ms 64.78, stage/render/create_views 29.03, stage/render/render 14.76, render/graph/ms 7.73 |
| warm-up | 3.50 | 174.6 | render/thread/ms 888, render/busy/ms 609, stage/render/prepare_assets 544, stage/render/prepare_views 183, render/acquire/ms 182, main/busy/ms 176, pipelines/compile_ms 142, stage/main/update 122, stage/render/render 111, render/present/ms 96.68, pipelines/driver_ms 80.04, pipelines/front_ms 61.59 |
| warm-up | 3.68 | 163.5 | pipelines/wait_ms 4218, pipelines/compile_ms 2185, pipelines/front_ms 1922, pipelines/driver_ms 262, main/busy/ms 190, render/thread/ms 135, render/busy/ms 132, stage/main/update 74.58, stage/render/prepare_assets 59.05, stage/main/post_update 57.96, stage/main/fixed_main_loop 33.44, stage/render/render 30.67 |
| warm-up | 3.77 | 87.1 | pipelines/wait_ms 4478, pipelines/compile_ms 2258, pipelines/front_ms 2107, render/thread/ms 187, render/busy/ms 185, main/busy/ms 164, pipelines/driver_ms 151, stage/render/prepare_assets 115, stage/main/update 75.68, stage/main/fixed_main_loop 33.09, stage/main/post_update 31.22, stage/render/prepare_resources 29.18 |
| warm-up | 3.85 | 34.6 | pipelines/compile_ms 853, pipelines/front_ms 816, pipelines/wait_ms 239, render/thread/ms 58.67, render/busy/ms 56.27, pipelines/driver_ms 37.79, main/busy/ms 29.09, encode//sum 18.10, stage/render/prepare_resources 17.70, stage/render/render 17.23, stage/main/post_update 17.19, render/graph/ms 13.97 |
| warm-up | 3.91 | 35.9 | pipelines/compile_ms 905, pipelines/front_ms 868, encode//sum 83.80, render/thread/ms 52.76, render/busy/ms 50.95, pipelines/driver_ms 37.02, main/busy/ms 27.45, stage/render/render 26.48, render/graph/ms 24.27, stage/render/graph_submit 21.76, stage/render/prepare_assets 17.66, stage/main/post_update 16.87 |
| warm-up | 4.00 | 73.9 | pipelines/compile_ms 1148, pipelines/front_ms 1094, main/busy/ms 61.60, pipelines/driver_ms 53.68, render/thread/ms 51.20, render/busy/ms 49.69, stage/main/update 46.33, stream/drive/ms 40.68, stage/render/prepare_assets 36.66, pipelines/wait_ms 21.76, stage/main/post_update 9.41, stage/render/render 8.27 |
| warm-up | 4.03 | 31.4 | pipelines/compile_ms 1183, pipelines/front_ms 1124, render/thread/ms 60.92, render/busy/ms 58.91, pipelines/driver_ms 58.29, main/busy/ms 46.12, stage/render/prepare_assets 31.84, stage/main/update 28.91, pipelines/wait_ms 27.82, stage/render/render 22.63, render/graph/ms 20.31, stage/render/graph_submit 14.73 |
| warm-up | 4.07 | 39.6 | pipelines/compile_ms 374, pipelines/front_ms 357, render/thread/ms 53.85, render/busy/ms 51.85, main/busy/ms 44.61, stage/main/update 28.76, stage/render/render 24.26, stage/render/prepare_assets 22.56, render/graph/ms 21.88, pipelines/driver_ms 16.54, stage/render/graph_submit 16.14, pipelines/wait_ms 13.00 |
| warm-up | 4.12 | 51.6 | pipelines/compile_ms 756, pipelines/front_ms 730, render/thread/ms 74.77, render/busy/ms 73.12, main/busy/ms 47.77, stage/render/prepare_assets 43.19, stage/main/update 31.86, stream/drive/ms 26.19, pipelines/driver_ms 25.34, stage/render/render 22.53, render/graph/ms 20.31, stage/render/camera_driver 12.26 |
| warm-up | 4.18 | 55.6 | pipelines/compile_ms 487, pipelines/front_ms 469, render/thread/ms 90.85, render/busy/ms 89.03, main/busy/ms 61.97, stage/render/prepare_assets 47.82, stage/main/update 32.65, stream/drive/ms 27.39, stage/render/render 24.57, render/graph/ms 22.20, pipelines/driver_ms 17.33, stage/main/post_update 14.62 |
| warm-up | 4.25 | 71.1 | pipelines/compile_ms 947, pipelines/front_ms 913, render/thread/ms 90.01, render/busy/ms 87.77, main/busy/ms 74.17, stage/render/prepare_assets 56.24, stage/main/update 39.88, pipelines/driver_ms 33.95, stage/main/post_update 16.74, stage/render/render 14.80, pipelines/wait_ms 12.96, pipelines/lock_ms 12.24 |
| warm-up | 4.33 | 75.6 | pipelines/compile_ms 1891, pipelines/front_ms 1830, render/thread/ms 93.88, render/busy/ms 91.70, main/busy/ms 71.91, stage/render/prepare_assets 65.15, pipelines/driver_ms 61.29, stage/main/update 43.32, stage/main/post_update 18.68, pipelines/wait_ms 17.56, pipelines/lock_ms 16.62, stage/render/render 15.75 |
| warm-up | 4.38 | 57.9 | pipelines/compile_ms 1322, pipelines/front_ms 1279, render/thread/ms 95.71, render/busy/ms 90.22, main/busy/ms 63.36, pipelines/wait_ms 52.18, pipelines/driver_ms 42.78, stage/render/prepare_assets 40.41, stage/render/render 37.66, render/graph/ms 31.74, stage/main/post_update 21.15, encode//sum 17.12 |
| warm-up | 4.43 | 46.2 | pipelines/compile_ms 378, pipelines/front_ms 363, render/thread/ms 87.43, pipelines/wait_ms 81.94, render/busy/ms 81.79, main/busy/ms 62.33, stage/render/render 43.00, encode//sum 40.77, render/graph/ms 36.92, stage/main/post_update 19.25, stage/render/camera_driver 18.93, stage/render/prepare_assets 18.84 |
| warm-up | 4.52 | 92.5 | pipelines/compile_ms 844, pipelines/front_ms 814, pipelines/wait_ms 185, render/thread/ms 119, render/busy/ms 117, main/busy/ms 70.34, encode//sum 53.92, stage/render/render 50.29, render/graph/ms 46.62, stage/main/post_update 38.92, pipelines/driver_ms 29.87, stage/render/camera_driver 28.80 |
| warm-up | 4.58 | 54.1 | pipelines/compile_ms 2493, pipelines/front_ms 2413, pipelines/wait_ms 1324, render/thread/ms 132, render/busy/ms 130, main/busy/ms 81.74, pipelines/driver_ms 79.97, encode//sum 53.34, stage/render/render 52.62, stage/main/post_update 52.47, render/graph/ms 48.87, stage/render/camera_driver 28.74 |
| warm-up | 4.62 | 44.2 | pipelines/compile_ms 2416, pipelines/front_ms 2344, pipelines/wait_ms 1860, render/thread/ms 86.23, render/busy/ms 83.58, pipelines/driver_ms 71.97, main/busy/ms 58.38, stage/main/post_update 31.77, encode//sum 28.99, stage/render/render 23.01, render/graph/ms 20.11, stage/render/prepare_assets 17.93 |
| warm-up | 4.66 | 41.5 | pipelines/compile_ms 982, pipelines/front_ms 954, pipelines/wait_ms 714, main/busy/ms 97.51, render/thread/ms 68.99, render/busy/ms 66.63, stage/main/update 62.97, stream/drive/ms 54.86, pipelines/driver_ms 28.14, stage/render/prepare_assets 22.64, stage/main/post_update 21.83, stage/render/prepare_write_phase_buffers 14.62 |
| warm-up | 4.76 | 99.1 | pipelines/compile_ms 1272, pipelines/front_ms 1230, pipelines/wait_ms 711, main/busy/ms 127, render/thread/ms 90.66, render/busy/ms 88.79, stage/main/update 88.03, stream/drive/ms 56.29, pipelines/driver_ms 42.61, stage/render/prepare_write_phase_buffers 30.59, stage/main/post_update 26.91, stage/render/render 18.04 |
| warm-up | 4.83 | 64.5 | pipelines/compile_ms 2269, pipelines/front_ms 2185, pipelines/wait_ms 1625, main/busy/ms 128, render/thread/ms 96.13, stage/main/update 93.92, render/busy/ms 93.91, pipelines/driver_ms 84.20, stream/drive/ms 44.02, stage/render/prepare_assets 25.67, stage/main/post_update 20.36, stage/render/render 20.19 |
| warm-up | 4.94 | 115.8 | pipelines/compile_ms 1495, pipelines/front_ms 1440, pipelines/wait_ms 971, main/busy/ms 101, render/thread/ms 97.38, render/busy/ms 95.57, stage/main/update 77.12, stage/render/prepare_assets 62.85, pipelines/driver_ms 54.71, stream/drive/ms 43.37, stage/render/render 15.44, render/graph/ms 13.08 |
| warm-up | 4.99 | 51.1 | pipelines/compile_ms 947, pipelines/front_ms 920, render/thread/ms 101, render/busy/ms 99.59, main/busy/ms 92.99, stage/render/prepare_assets 75.29, stage/main/update 65.36, pipelines/wait_ms 48.11, stream/drive/ms 44.53, pipelines/driver_ms 27.47, stage/main/post_update 17.38, stage/render/render 12.37 |
| warm-up | 5.07 | 74.9 | pipelines/compile_ms 1290, pipelines/front_ms 1253, main/busy/ms 87.18, render/thread/ms 82.58, render/busy/ms 81.29, stage/main/update 65.55, stage/render/prepare_assets 49.74, stream/drive/ms 43.83, pipelines/driver_ms 36.54, stage/render/render 20.64, pipelines/wait_ms 20.58, render/graph/ms 18.64 |
| warm-up | 5.13 | 33.0 | pipelines/compile_ms 405, pipelines/front_ms 393, main/busy/ms 47.25, render/thread/ms 37.72, render/busy/ms 36.47, stage/main/update 34.44, stream/drive/ms 28.15, pipelines/driver_ms 12.62, stage/render/prepare_assets 12.22, stage/render/render 9.39, encode//sum 7.96, render/graph/ms 7.67 |
| warm-up | 5.24 | 48.4 | pipelines/compile_ms 200, pipelines/front_ms 195, render/thread/ms 67.25, render/busy/ms 64.88, main/busy/ms 27.21, stage/render/prepare_resources 21.60, stage/render/render 15.54, render/graph/ms 13.11, stage/main/update 12.08, stage/render/prepare_assets 10.85, stage/render/graph_submit 9.73, stage/main/post_update 9.17 |
| warm-up | 5.34 | 66.0 | pipelines/compile_ms 128, pipelines/front_ms 124, render/thread/ms 72.26, render/busy/ms 70.96, main/busy/ms 38.84, stage/render/prepare_assets 38.08, stage/main/update 20.26, stage/render/render 12.73, stage/main/post_update 11.74, render/graph/ms 10.74, encode//sum 8.68, stage/render/graph_submit 8.01 |
| warm-up | 5.39 | 49.2 | pipelines/compile_ms 1502, pipelines/front_ms 1462, render/thread/ms 94.23, render/busy/ms 92.86, main/busy/ms 64.89, stage/render/prepare_assets 60.65, pipelines/driver_ms 40.00, stage/main/update 26.55, stage/main/pre_update 21.56, stream/drive/ms 15.55, stage/main/post_update 12.23, stage/render/render 12.08 |
| warm-up | 5.42 | 38.2 | pipelines/compile_ms 1689, pipelines/front_ms 1642, render/thread/ms 74.16, render/busy/ms 72.74, main/busy/ms 67.98, pipelines/wait_ms 56.26, pipelines/driver_ms 47.71, stage/render/prepare_assets 43.06, stage/main/update 21.70, stage/main/pre_update 21.58, stage/main/post_update 19.27, stream/drive/ms 14.09 |
| warm-up | 5.55 | 29.6 | pipelines/compile_ms 940, pipelines/front_ms 916, render/thread/ms 41.35, render/busy/ms 40.04, pipelines/wait_ms 37.89, main/busy/ms 24.70, pipelines/driver_ms 23.96, stage/render/prepare_assets 20.67, stage/render/render 11.82, stage/main/post_update 10.63, render/graph/ms 10.00, stage/main/update 9.11 |
| warm-up | 5.60 | 29.1 | render/thread/ms 40.13, render/busy/ms 38.82, main/busy/ms 26.67, stage/render/prepare_assets 22.12, stage/main/post_update 12.60, stage/render/render 11.48, stage/main/update 9.73, render/graph/ms 9.67, encode//sum 7.51, stage/render/graph_submit 6.96, encode/submit 5.16, stream/drive/ms 3.90 |
| warm-up | 6.02 | 29.9 | pipelines/compile_ms 337, pipelines/front_ms 328, main/busy/ms 33.84, stage/main/update 20.56, render/thread/ms 19.34, render/busy/ms 18.21, stream/drive/ms 14.43, pipelines/driver_ms 9.23, stage/main/post_update 9.03, stage/render/render 8.56, encode//sum 7.40, render/graph/ms 7.05 |
| warm-up | 6.16 | 32.0 | pipelines/compile_ms 292, pipelines/front_ms 283, render/thread/ms 36.74, render/busy/ms 35.77, main/busy/ms 28.55, stage/render/prepare_assets 15.93, stage/main/update 11.94, stage/main/post_update 10.57, pipelines/driver_ms 9.37, stage/render/render 7.97, encode//sum 6.63, render/graph/ms 6.61 |
| warm-up | 6.34 | 29.2 | main/busy/ms 39.53, render/thread/ms 36.53, render/busy/ms 35.34, stage/main/update 20.59, stage/main/post_update 14.19, stage/render/render 11.21, render/graph/ms 9.42, stage/render/prepare_assets 9.34, encode//sum 8.63, stage/render/graph_submit 5.86, stream/drive/ms 5.27, stage/render/camera_driver 3.43 |
| warm-up | 6.37 | 30.4 | render/thread/ms 48.00, render/busy/ms 46.70, main/busy/ms 40.80, stage/main/post_update 18.04, stage/main/update 16.44, stage/render/prepare_assets 14.67, stage/render/render 13.33, render/graph/ms 11.54, encode//sum 9.16, stage/render/graph_submit 8.09, stream/drive/ms 6.45, encode/submit 5.06 |
| warm-up | 6.66 | 38.8 | render/thread/ms 57.71, render/busy/ms 56.47, main/busy/ms 40.70, stage/render/prepare_assets 33.51, stage/main/update 22.58, stream/drive/ms 16.36, stage/main/post_update 13.72, stage/render/render 12.24, render/graph/ms 10.31, encode//sum 9.63, stage/render/graph_submit 7.24, encode/submit 3.98 |
| warm-up | 7.00 | 31.1 | render/thread/ms 38.23, render/busy/ms 37.19, main/busy/ms 17.90, stage/render/render 11.12, stage/render/specialize 10.08, stage/main/post_update 9.96, render/graph/ms 9.64, stage/render/graph_submit 7.39, stage/render/queue 6.61, encode//sum 6.54, stage/main/update 5.11, encode/submit 4.19 |
| warm-up | 8.28 | 31.6 | pipelines/compile_ms 136, pipelines/front_ms 134, render/thread/ms 36.94, render/busy/ms 35.86, main/busy/ms 18.17, stage/render/render 10.43, stage/main/post_update 10.31, stage/render/specialize 9.86, render/graph/ms 8.93, encode//sum 7.03, stage/render/graph_submit 6.80, stage/render/queue 6.64 |
| measure | 1.69 | 29.8 | render/thread/ms 35.92, render/busy/ms 34.96, main/busy/ms 18.85, stage/main/post_update 11.87, stage/render/render 11.40, render/graph/ms 10.00, encode//sum 9.04, stage/render/specialize 8.97, stage/render/graph_submit 7.07, stage/render/queue 5.36, encode/submit 4.24, stage/render/prepare_meshes 4.20 |
| measure | 4.48 | 29.7 | render/thread/ms 36.83, render/busy/ms 35.82, main/busy/ms 19.21, stage/main/post_update 12.56, stage/render/render 10.97, stage/render/specialize 10.05, render/graph/ms 9.51, encode//sum 8.07, stage/render/graph_submit 7.04, stage/render/queue 6.66, encode/submit 4.32, stage/main/update 3.93 |
| measure | 5.81 | 30.7 | render/thread/ms 36.42, render/busy/ms 35.36, main/busy/ms 18.64, stage/main/post_update 12.04, stage/render/specialize 10.58, stage/render/render 10.39, render/graph/ms 8.70, encode//sum 7.07, stage/render/graph_submit 6.89, stage/render/queue 6.33, stage/main/update 3.96, encode/submit 3.70 |
| measure | 7.14 | 33.3 | render/thread/ms 40.87, render/busy/ms 39.77, main/busy/ms 19.72, stage/render/render 13.16, render/graph/ms 11.61, stage/main/post_update 11.47, stage/render/specialize 10.57, encode//sum 10.16, stage/render/graph_submit 8.66, stage/render/queue 7.22, stage/main/update 5.42, encode/submit 5.24 |
| measure | 8.43 | 29.1 | render/thread/ms 35.73, render/busy/ms 34.66, main/busy/ms 17.47, stage/main/post_update 10.92, stage/render/render 10.60, stage/render/specialize 9.31, render/graph/ms 9.04, encode//sum 6.80, stage/render/graph_submit 6.75, stage/render/queue 6.22, stage/main/update 4.09, encode/submit 3.98 |
| measure | 9.73 | 35.1 | render/thread/ms 43.94, render/busy/ms 42.88, main/busy/ms 21.33, stage/main/post_update 13.69, stage/render/render 12.96, render/graph/ms 11.46, stage/render/specialize 9.95, encode//sum 9.24, stage/render/graph_submit 8.54, stage/render/queue 6.32, stage/render/prepare_assets 5.86, encode/submit 5.02 |
| measure | 15.72 | 29.9 | render/thread/ms 35.18, render/busy/ms 34.13, main/busy/ms 20.43, stage/main/post_update 11.57, stage/render/specialize 10.44, stage/render/render 10.30, render/graph/ms 8.65, stage/render/graph_submit 6.78, encode//sum 6.73, stage/render/queue 6.17, stage/main/update 4.18, encode/submit 3.97 |
| measure | 17.03 | 29.4 | render/thread/ms 34.44, render/busy/ms 33.43, main/busy/ms 20.43, stage/main/post_update 10.99, stage/render/render 9.84, stage/render/specialize 9.77, render/graph/ms 8.31, encode//sum 7.67, stage/render/queue 6.35, stage/render/graph_submit 5.98, stage/main/update 4.22, encode/submit 3.56 |
| measure | 18.17 | 30.2 | main/busy/ms 38.57, stage/main/update 24.10, stream/drive/ms 20.25, render/thread/ms 20.02, render/busy/ms 18.99, stage/main/post_update 11.71, stage/render/render 11.06, render/graph/ms 9.59, encode//sum 7.58, stage/render/graph_submit 7.12, encode/submit 4.62, encode/pass:shadow_point_light/sum 3.75 |
| measure | 20.82 | 29.4 | render/thread/ms 37.21, render/busy/ms 36.15, main/busy/ms 22.06, stage/main/post_update 12.33, stage/render/render 11.51, render/graph/ms 9.98, stage/render/specialize 9.57, encode//sum 8.19, stage/render/graph_submit 7.39, stage/render/queue 7.19, encode/submit 4.98, stage/main/update 4.93 |
| measure | 23.03 | 33.5 | render/thread/ms 40.75, render/busy/ms 39.70, main/busy/ms 24.32, stage/main/post_update 12.31, stage/render/specialize 12.10, stage/render/render 11.83, render/graph/ms 10.37, encode//sum 8.29, stage/render/graph_submit 7.81, stage/render/queue 7.23, stage/main/update 6.58, encode/submit 4.82 |
| measure | 24.56 | 30.3 | render/thread/ms 36.68, render/busy/ms 35.61, main/busy/ms 19.85, stage/main/post_update 11.80, stage/render/render 11.08, stage/render/specialize 10.09, render/graph/ms 9.63, encode//sum 8.29, stage/render/graph_submit 7.37, stage/render/queue 6.30, stage/main/update 5.52, encode/submit 4.66 |
| measure | 27.27 | 35.8 | render/thread/ms 48.38, render/busy/ms 47.33, main/busy/ms 28.04, stage/render/render 17.16, render/graph/ms 15.59, encode//sum 13.92, stage/main/post_update 13.54, stage/render/specialize 12.28, stage/render/graph_submit 11.92, encode/pass:shadow_point_light/sum 11.13, encode/submit 7.86, stage/render/queue 7.76 |
| measure | 28.74 | 29.5 | render/thread/ms 38.95, render/busy/ms 37.94, main/busy/ms 19.41, stage/main/post_update 12.67, stage/render/render 12.38, render/graph/ms 10.93, stage/render/specialize 9.89, encode//sum 9.07, stage/render/graph_submit 8.28, stage/render/queue 6.79, encode/pass:shadow_point_light/sum 5.26, encode/submit 5.06 |
| measure | 30.47 | 33.2 | render/thread/ms 41.65, render/busy/ms 40.58, main/busy/ms 21.90, stage/render/render 12.66, stage/main/post_update 12.64, render/graph/ms 11.14, stage/render/specialize 10.69, stage/render/graph_submit 8.64, encode//sum 8.59, stage/render/queue 7.22, stage/main/update 5.70, encode/submit 5.20 |
| measure | 31.92 | 30.0 | render/thread/ms 40.09, render/busy/ms 39.03, main/busy/ms 19.77, stage/render/render 12.96, stage/main/post_update 12.54, render/graph/ms 11.19, stage/render/specialize 9.63, encode//sum 8.54, stage/render/graph_submit 8.27, encode/pass:shadow_point_light/sum 6.86, stage/render/queue 6.77, encode/submit 5.09 |
| measure | 35.35 | 30.9 | render/thread/ms 39.98, render/busy/ms 38.86, main/busy/ms 19.89, stage/render/render 12.76, stage/main/post_update 12.38, render/graph/ms 11.28, stage/render/specialize 10.31, stage/render/graph_submit 8.76, encode//sum 8.48, stage/render/queue 6.97, encode/submit 5.27, stage/main/update 4.66 |
| measure | 36.72 | 29.6 | render/thread/ms 39.17, render/busy/ms 38.06, main/busy/ms 19.25, stage/main/post_update 12.40, stage/render/render 12.25, render/graph/ms 10.73, stage/render/specialize 10.09, encode//sum 9.09, stage/render/graph_submit 8.08, stage/render/queue 7.02, encode/submit 5.04, encode/pass:shadow_point_light/sum 4.80 |
| measure | 40.80 | 34.1 | render/thread/ms 40.13, render/busy/ms 38.99, main/busy/ms 19.06, stage/render/render 11.42, stage/main/post_update 10.86, stage/render/specialize 10.13, render/graph/ms 9.86, stage/render/graph_submit 7.83, encode//sum 7.41, stage/render/queue 6.58, stage/main/update 4.51, encode/submit 4.31 |
| measure | 43.21 | 34.3 | render/thread/ms 37.45, render/busy/ms 36.51, main/busy/ms 17.91, stage/main/post_update 10.44, stage/render/specialize 10.02, stage/render/render 10.01, render/graph/ms 8.69, encode//sum 6.94, stage/render/graph_submit 6.88, stage/render/queue 6.80, stage/main/update 4.75, stage/render/prepare_meshes 3.59 |
| measure | 45.66 | 32.7 | render/thread/ms 38.06, render/busy/ms 36.87, main/busy/ms 16.81, stage/render/render 10.79, stage/main/post_update 10.00, stage/render/specialize 9.82, render/graph/ms 9.29, stage/render/graph_submit 7.33, encode//sum 7.08, stage/render/queue 6.37, stage/main/update 4.36, stage/render/prepare_meshes 4.23 |
| measure | 56.44 | 30.9 | render/thread/ms 37.99, render/busy/ms 36.92, main/busy/ms 14.34, stage/render/render 10.84, stage/render/specialize 10.11, render/graph/ms 9.57, stage/main/post_update 8.02, encode//sum 7.46, stage/render/graph_submit 7.39, stage/render/queue 6.75, encode/submit 4.28, stage/main/update 3.91 |
| measure | 59.04 | 45.5 | render/thread/ms 55.92, render/busy/ms 54.47, main/busy/ms 32.30, stage/main/post_update 18.55, stage/render/render 17.68, render/graph/ms 15.69, stage/render/graph_submit 12.62, stage/render/specialize 12.08, encode//sum 10.98, stage/main/update 9.32, encode/submit 8.65, stage/render/prepare_meshes 7.77 |
| measure | 59.07 | 34.5 | render/thread/ms 58.83, render/busy/ms 57.86, main/busy/ms 23.91, stage/render/prepare_assets 14.85, stage/main/post_update 12.43, stage/render/specialize 12.03, stage/render/render 10.47, render/graph/ms 9.21, stage/render/prepare_meshes 8.43, encode//sum 7.31, stage/render/graph_submit 7.15, stage/render/queue 6.40 |
| measure | 62.57 | 30.9 | render/thread/ms 38.70, render/busy/ms 37.66, main/busy/ms 18.52, stage/render/render 11.27, stage/render/specialize 10.51, render/graph/ms 10.03, stage/main/post_update 9.79, encode//sum 8.08, stage/render/graph_submit 7.81, stage/render/queue 6.38, stage/main/update 5.77, stage/render/prepare_meshes 4.22 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set render.skin.skip_unchanged=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=skin-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 13%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **79%** (min 55%) · 2622 MHz (min 2610) · memory 8101 MHz · 80.8 W · 76 °C (max 82) · VRAM 7058 MiB · throttled 100% of samples

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
