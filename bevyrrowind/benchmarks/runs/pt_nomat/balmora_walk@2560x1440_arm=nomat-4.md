# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.depth_only.skip_material=on (command line: --set); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6071 frames over 65.7 s · **median 10.63 ms (94 fps)** · mean 10.81 · p95 13.41 · p99 16.21 (1% low 62 fps) · worst 43.42 ms · 65 spikes over 1.5× median · 53.0 stutters/min (work 50.3/min)
- warm-up: 323 frames over 8.9 s, worst 2446.5 ms
- hitches (> 26.6 ms): **29** measured, 49 in warm-up
- timed GPU passes: 6.43 ms of a 10.81 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.63 ms, limit 11 ms
- **FAIL** frame_ms: 58 stutters (> 1.5x moving median of 60) in 1.09 min = 53.01/min, limit 1/min
- **FAIL** worst frame 43.42 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.829 | 2.035 | 2.068 | 0.90 | 0.175 |
| main_opaque_pass_3d | 1.448 | 1.610 | 2.019 | 0.90 | 0.060 |
| early prepass | 0.437 | 0.486 | 0.712 | 0.90 | 0.089 |
| bloom | 0.283 | 0.315 | 0.321 | 0.90 | 0.063 |
| atmosphere_luts | 0.273 | 0.304 | 0.365 | 0.90 | 0.003 |
| water_velocity | 0.252 | 0.280 | 0.347 | 0.90 | 0.002 |
| water_sspr | 0.181 | 0.201 | 0.202 | 0.90 | 0.002 |
| ssao | 0.169 | 0.188 | 0.191 | 0.90 | 0.007 |
| auxiliary_views | 0.134 | 0.149 | 0.520 | 0.90 | 0.145 |
| atmos_froxel_inject | 0.131 | 0.145 | 0.147 | 0.90 | 0.001 |
| clouds_trace | 0.126 | 0.140 | 0.231 | 0.90 | 0.001 |
| water_fft | 0.119 | 0.132 | 0.138 | 0.90 | 0.003 |
| uniform_allocation | 0.116 | 0.128 | 0.183 | 0.90 | 0.024 |
| upscaling | 0.106 | 0.118 | 0.125 | 0.90 | 0.004 |
| clouds_composite | 0.095 | 0.105 | 0.108 | 0.90 | 0.002 |
| bin_unpacking | 0.093 | 0.104 | 0.141 | 0.90 | 0.018 |
| main_transmissive_pass_3d | 0.076 | 0.085 | 0.177 | 0.90 | 0.009 |
| tonemapping | 0.076 | 0.085 | 0.087 | 0.90 | 0.004 |
| clustering | 0.058 | 0.064 | 0.067 | 0.90 | 0.018 |
| render_sky | 0.046 | 0.051 | 0.052 | 0.90 | 0.002 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.90 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.90 | 0.001 |
| surface_trail | 0.035 | 0.056 | 0.057 | 0.63 | 0.002 |
| surface_exposure_far | 0.034 | 0.043 | 0.045 | 0.80 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.90 | 0.003 |
| lightprobe_irradiance_map | 0.027 | 0.411 | 0.415 | 0.07 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.030 | 0.047 | 0.90 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.029 | 0.031 | 0.90 | 0.001 |
| water_snapshot | 0.022 | 0.025 | 0.028 | 0.90 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.024 | 0.90 | 0.002 |
| lightprobe_radiance_map | 0.015 | 0.218 | 0.229 | 0.07 | 0.004 |
| night_sky | 0.012 | 0.014 | 0.034 | 0.90 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.90 | 0.125 |
| clouds_fog_light | 0.010 | 0.012 | 0.012 | 0.90 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.90 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.90 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.90 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.07 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.07 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.07 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.260 | 11.452 | 37.15 | 56214.8 |
| main/busy/ms | 8.968 | 10.678 | 29.38 | 54445.6 |
| render/busy/ms | 8.698 | 10.915 | 36.60 | 52808.3 |
| stage/main/post_update | 5.540 | 6.899 | 10.68 | 33633.4 |
| stage/render/render | 5.404 | 6.856 | 13.23 | 32805.3 |
| render/graph/ms | 4.648 | 6.072 | 12.29 | 28220.4 |
| encode//sum | 3.726 | 4.943 | 21.92 | 22621.7 |
| stage/render/graph_submit | 3.473 | 4.634 | 9.57 | 21087.3 |
| encode/submit | 2.232 | 3.163 | 6.37 | 13550.1 |
| stage/main/update | 2.153 | 2.575 | 22.80 | 13072.9 |
| encode/finish_wall | 1.319 | 1.901 | 4.11 | 8008.2 |
| stage/render/camera_driver | 1.122 | 1.532 | 4.29 | 6811.7 |
| encode/pass:shadow_directional_light/sum | 0.843 | 1.512 | 3.65 | 5118.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.811 | 1.092 | 3.42 | 4925.6 |
| encode/pass:early prepass/sum | 0.794 | 1.070 | 2.01 | 4817.6 |
| stage/render/prepare_write_phase_buffers | 0.754 | 1.094 | 2.30 | 4574.5 |
| stage/render/prepare_assets | 0.564 | 0.733 | 11.00 | 3425.5 |
| bigspace/total/ms | 0.540 | 1.280 | 2.55 | 3280.6 |
| render/present/ms | 0.466 | 1.190 | 2.21 | 2831.6 |
| stage/render/prepare_resources | 0.447 | 0.531 | 0.97 | 2714.0 |
| stage/render/prepare_meshes | 0.430 | 0.537 | 7.79 | 2612.3 |
| stage/main/first | 0.406 | 0.528 | 4.95 | 2461.8 |
| stage/main/pre_update | 0.359 | 0.535 | 2.59 | 2179.8 |
| stage/render/specialize | 0.311 | 0.409 | 10.87 | 1887.6 |
| encode/pass:shadow_point_light/sum | 0.310 | 2.016 | 7.55 | 1882.7 |
| terrain/update/ms | 0.305 | 0.391 | 1.76 | 1853.1 |
| stage/render/prepare_bind_groups | 0.281 | 0.501 | 2.54 | 1703.1 |
| stage/render/queue | 0.277 | 0.398 | 7.54 | 1682.5 |
| terrain/select/ms | 0.271 | 0.348 | 0.89 | 1648.2 |
| controller/feed/ms | 0.260 | 0.328 | 0.71 | 1578.8 |
| stream/drive/ms | 0.249 | 0.328 | 20.95 | 1512.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.234 | 0.344 | 1.03 | 1419.5 |
| bigspace/low_precision/ms | 0.218 | 0.665 | 2.25 | 1322.5 |
| stage/main/fixed_main_loop | 0.215 | 0.372 | 1.34 | 1307.3 |
| far/select/ms | 0.202 | 0.271 | 0.58 | 1224.5 |
| stage/render/create_views | 0.197 | 0.255 | 0.95 | 1193.6 |
| stream/tick/ms | 0.193 | 0.262 | 0.50 | 1169.0 |
| stage/render/prepare_batch_phases | 0.192 | 0.277 | 0.49 | 1164.7 |
| stage/render/extract_commands | 0.169 | 0.199 | 0.74 | 1024.3 |
| stage/render/prepare_flush | 0.168 | 0.328 | 1.19 | 1019.0 |
| pipelines/compile_ms | 0.163 | 0.000 | 138.52 | 989.0 |
| anim/evaluate/ms | 0.162 | 0.312 | 1.54 | 982.4 |
| pipelines/front_ms | 0.158 | 0.000 | 135.53 | 960.4 |
| bigspace/high_precision/ms | 0.158 | 0.645 | 1.73 | 957.7 |
| anim/evaluate_cpu/ms | 0.148 | 0.190 | 0.73 | 901.4 |
| stage/render/prepare_views | 0.138 | 0.173 | 0.49 | 839.0 |
| controller/move/ms | 0.131 | 0.226 | 0.41 | 795.3 |
| stage/main/last | 0.117 | 0.160 | 2.04 | 709.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.095 | 0.151 | 0.68 | 575.6 |
| render/acquire/ms | 0.095 | 0.123 | 0.43 | 574.9 |
| stage/render/prepare_collect_phase_buffers | 0.060 | 0.129 | 0.50 | 365.0 |
| physics/step/ms | 0.047 | 0.086 | 0.67 | 283.3 |
| stage/render/cleanup | 0.034 | 0.108 | 0.73 | 206.3 |
| physics/insert/ms | 0.032 | 0.354 | 0.94 | 191.6 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.80 | 189.2 |
| stage/render/phase_sort | 0.030 | 0.053 | 0.22 | 181.1 |
| weather/map/ms | 0.025 | 0.034 | 0.29 | 154.6 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.60 | 89.3 |
| anim/apply/ms | 0.011 | 0.020 | 0.42 | 67.7 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.21 | 67.2 |
| controller/avoid/ms | 0.010 | 0.014 | 0.14 | 62.8 |
| weather/eval/ms | 0.010 | 0.014 | 0.10 | 61.9 |
| script/update/ms | 0.007 | 0.009 | 0.18 | 43.3 |
| script/input/ms | 0.005 | 0.007 | 0.10 | 32.1 |
| stream/despawn/ms | 0.005 | 0.000 | 1.21 | 31.2 |
| script/ui/ms | 0.005 | 0.007 | 0.20 | 30.7 |
| pipelines/driver_ms | 0.005 | 0.000 | 3.70 | 28.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 28.6 |
| controller/write/ms | 0.003 | 0.004 | 0.03 | 20.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 13.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.06 | 5.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.40 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.09 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.9 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.3 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 30 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 27 | 42 | 12 |
| crowd/c0/joints/count | count | 1301 | 2204 | 535 |
| crowd/c0/meshes/count | count | 353 | 634 | 144 |
| crowd/c1/count | count | 146 | 154 | 149 |
| crowd/c1/joints/count | count | 5531 | 5894 | 5661 |
| crowd/c1/meshes/count | count | 1167 | 1254 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4614 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.40 | 1.60 | 0.37 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.60 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.21 | 0.01 |
| encode/finish_critical | ms | 0.97 | 3.65 | 1.71 |
| encode/pass:early prepass/max | ms | 0.80 | 2.01 | 0.74 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 3.42 | 0.58 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.68 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 1.03 | 0.21 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 3.65 | 1.71 |
| encode/pass:shadow_point_light/max | ms | 0.10 | 1.35 | 0.71 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.80 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.22 | 36.60 | 8.76 |
| gpu/timed/ms | ms | 7.89 | 8.84 | 8.14 |
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
| phases/alpha_mask3d/items | count | 812 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1283 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3402 | 5138 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1283 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3402 | 5138 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 16 | 201 | 29 |
| phases/shadow/bins | count | 5003 | 8984 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6384 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 200 | 257 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 187 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 14 | 559 | 57 |
| preprocess/bind_groups_created | count | 14 | 559 | 57 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 202 | 124 |
| process/cpu_usage | % | 25.96 | 38.46 | 26.44 |
| process/mem_usage | GiB | 5.51 | 5.68 | 5.68 |
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
| skin/blocks_changed/count | count | 14 | 131 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 370.7 k | 809.2 k | 491.8 k |
| skin/joints_written/count | count | 5792 | 12.6 k | 7684 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4917 | 5011 | 5011 |
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
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.86 | 57.28 | 33.06 |
| system/mem_usage | % | 57.12 | 57.80 | 57.78 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2900 | 3016 | 3016 |
| textures/images | count | 2680 | 2740 | 2740 |
| views/all/batch_sets | count | 67 | 250 | 79 |
| views/all/bins | count | 7624 | 11.9 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.4 k | 14.0 k |
| views/all/pipelines | count | 247 | 300 | 75 |
| views/all/pipelines_distinct | count | 71 | 74 | 61 |
| views/all/work_items | count | 10.7 k | 36.0 k | 14.3 k |
| views/cascade/batch_sets | count | 9 | 54 | 29 |
| views/cascade/bins | count | 2796 | 6218 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8912 | 23.5 k | 6384 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 416 | 23.5 k | 6384 |
| views/lamp_face/batch_sets | count | 0 | 187 | 0 |
| views/lamp_face/bins | count | 2131 | 2906 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2388 | 3200 | 0 |
| views/lamp_face/pipelines | count | 164 | 210 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2849 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2864 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8426 | 13.0 k | 7616 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8587 | 13.2 k | 7924 |
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
| anim/bones_written/count | count | 51.33 | 293 | 311.6 k |
| anim/events/count | count | 0.10 | 3 | 579 |
| assets/Image/added | count | 0.10 | 41 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.57 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1386 | 4675 |
| encode//count | count | 33.50 | 219 | 203.4 k |
| encode/bind_walk/init_textures | count | 9973 | 44.8 k | 60.55 M |
| encode/bind_walk/init_textures_skipped | count | 45.5 k | 70.9 k | 276.25 M |
| encode/bind_walk/inits | count | 458 | 1085 | 2.78 M |
| encode/bind_walk/inits_skipped | count | 98.57 | 149 | 598.4 k |
| encode/bind_walk/merge_textures | count | 9901 | 44.7 k | 60.11 M |
| encode/bind_walk/merge_textures_skipped | count | 10.1 k | 26.6 k | 61.26 M |
| encode/bind_walk/merges | count | 97.48 | 297 | 591.8 k |
| encode/bind_walk/merges_skipped | count | 89.87 | 144 | 545.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6071 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6071 |
| encode/pass:early prepass/count | count | 1 | 1 | 6071 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6071 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6071 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6071 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6071 |
| encode/pass:shadow_point_light/count | count | 1.39 | 48 | 8429 |
| encode/queue/command_buffers | count | 44.91 | 277 | 272.6 k |
| encode/queue/submits | count | 2.02 | 4 | 12.2 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.88 M | 11.42 G |
| encode/queue/write_buffer/calls | count | 259 | 837 | 1.57 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.7 k | 3.52 M | 1.08 G |
| encode/queue/write_buffer_with/calls | count | 10.99 | 465 | 66.7 k |
| encode/queue/write_texture/bytes | bytes | 11.7 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6071 |
| physics/inserts | count | 0.37 | 95 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2335 |
| physics/steps | count | 0.65 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 8 |
| pipelines/created | count | 0.00 | 2 | 12 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5576 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1223 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.6 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.45 | 2446.5 | main/busy/ms 2417, stage/main/post_update 2314, stage/main/update 95.21, surface/advance_cpu_ms 66.23, stage/main/pre_update 4.52, stage/main/last 1.02, surface/cell_upload_ms 0.72, stage/main/first 0.63, stream/drive/ms 0.40, stage/main/fixed_main_loop 0.40, weather/map/ms 0.12, bigspace/total/ms 0.10 |
| warm-up | 3.18 | 730.7 | main/busy/ms 2544, stage/main/post_update 2336, render/thread/ms 699, render/busy/ms 533, stage/render/prepare_assets 486, stage/main/update 194, stage/render/prepare_views 163, render/acquire/ms 163, surface/advance_cpu_ms 66.23, stage/render/create_views 27.58, stage/render/render 11.45, render/graph/ms 7.08 |
| warm-up | 3.29 | 108.1 | render/thread/ms 791, render/busy/ms 575, stage/render/prepare_assets 512, stage/render/prepare_views 164, render/acquire/ms 164, main/busy/ms 147, pipelines/compile_ms 114, stage/main/update 103, pipelines/front_ms 76.50, stage/render/render 66.75, render/present/ms 51.97, pipelines/driver_ms 37.37 |
| warm-up | 3.32 | 32.6 | pipelines/compile_ms 348, pipelines/front_ms 278, render/thread/ms 118, stage/render/render 74.98, pipelines/driver_ms 69.53, main/busy/ms 68.55, render/busy/ms 65.42, stage/main/post_update 53.10, render/present/ms 51.30, stage/render/prepare_assets 26.36, pipelines/wait_ms 25.49, terrain/update/ms 23.34 |
| warm-up | 3.47 | 152.9 | pipelines/wait_ms 4583, pipelines/compile_ms 1638, pipelines/front_ms 1529, render/thread/ms 156, render/busy/ms 145, pipelines/driver_ms 110, main/busy/ms 90.10, stage/render/prepare_resources 56.30, stage/main/post_update 52.25, stage/render/render 48.54, render/graph/ms 36.14, encode//sum 30.44 |
| warm-up | 3.50 | 31.1 | pipelines/wait_ms 4560, pipelines/compile_ms 1432, pipelines/front_ms 1352, render/thread/ms 159, render/busy/ms 149, main/busy/ms 114, pipelines/driver_ms 79.69, stage/render/prepare_resources 64.65, stage/main/update 60.62, stage/render/render 33.51, encode//sum 33.16, stage/main/fixed_main_loop 28.98 |
| warm-up | 3.62 | 121.8 | pipelines/compile_ms 1582, pipelines/front_ms 1499, pipelines/wait_ms 185, render/thread/ms 102, render/busy/ms 99.68, main/busy/ms 89.68, pipelines/driver_ms 83.25, stage/main/update 63.83, stage/render/prepare_assets 62.98, stage/render/render 20.67, stage/main/post_update 19.52, render/graph/ms 18.37 |
| warm-up | 3.68 | 56.0 | pipelines/compile_ms 2362, pipelines/front_ms 2243, pipelines/wait_ms 373, render/thread/ms 127, render/busy/ms 124, pipelines/driver_ms 119, stage/render/prepare_assets 86.70, stage/render/render 33.38, main/busy/ms 31.80, render/graph/ms 30.20, stage/render/graph_submit 27.84, encode//sum 17.35 |
| warm-up | 3.82 | 57.4 | pipelines/compile_ms 780, pipelines/front_ms 732, main/busy/ms 54.36, render/thread/ms 50.75, render/busy/ms 49.43, pipelines/driver_ms 48.30, stage/main/update 40.13, stage/render/prepare_assets 35.19, stream/drive/ms 33.86, pipelines/wait_ms 13.16, stage/render/render 9.50, render/graph/ms 7.74 |
| warm-up | 3.90 | 60.9 | pipelines/compile_ms 658, pipelines/front_ms 632, render/thread/ms 55.77, render/busy/ms 53.38, main/busy/ms 52.85, stage/main/update 40.10, stream/drive/ms 35.62, stage/render/prepare_assets 30.63, pipelines/driver_ms 26.54, stage/render/render 16.70, render/graph/ms 13.67, stage/render/graph_submit 10.23 |
| warm-up | 3.93 | 33.4 | pipelines/compile_ms 896, pipelines/front_ms 861, render/thread/ms 62.40, render/busy/ms 56.44, main/busy/ms 47.39, pipelines/driver_ms 35.89, stage/render/prepare_assets 31.51, stage/main/update 21.36, stage/render/render 17.44, stream/drive/ms 13.58, stage/main/post_update 11.99, render/graph/ms 11.05 |
| warm-up | 4.02 | 89.8 | pipelines/compile_ms 489, pipelines/front_ms 473, render/thread/ms 75.84, render/busy/ms 70.29, main/busy/ms 55.92, stage/render/prepare_assets 48.92, stage/main/update 25.91, pipelines/driver_ms 16.38, stage/render/render 14.13, stage/main/post_update 13.33, stream/drive/ms 12.41, render/graph/ms 8.24 |
| warm-up | 4.08 | 58.2 | pipelines/compile_ms 1143, pipelines/front_ms 1115, render/thread/ms 82.84, render/busy/ms 81.38, stage/render/prepare_assets 63.83, main/busy/ms 45.06, pipelines/driver_ms 27.62, stage/main/update 27.49, stage/render/render 11.04, render/graph/ms 9.13, stage/main/post_update 8.86, stage/render/graph_submit 7.00 |
| warm-up | 4.11 | 32.4 | pipelines/compile_ms 1394, pipelines/front_ms 1355, render/thread/ms 58.17, render/busy/ms 56.68, main/busy/ms 46.49, stage/render/prepare_assets 39.62, pipelines/driver_ms 39.07, stage/main/update 29.34, stage/render/render 9.77, stage/main/post_update 9.58, render/graph/ms 7.77, pipelines/wait_ms 5.59 |
| warm-up | 4.16 | 47.5 | pipelines/compile_ms 571, pipelines/front_ms 550, main/busy/ms 71.89, render/thread/ms 66.31, render/busy/ms 63.66, stage/main/update 36.43, pipelines/driver_ms 20.99, stage/render/prepare_assets 17.89, stage/render/render 17.87, render/graph/ms 14.69, stage/main/post_update 14.25, stage/main/pre_update 13.19 |
| warm-up | 4.24 | 82.7 | pipelines/compile_ms 268, pipelines/front_ms 259, render/thread/ms 94.21, render/busy/ms 91.47, main/busy/ms 73.95, stage/main/update 29.35, stage/render/prepare_assets 22.29, stage/main/pre_update 21.56, stage/render/render 18.61, stage/render/prepare_resources 16.91, render/graph/ms 15.37, stage/main/post_update 12.93 |
| warm-up | 4.31 | 65.8 | pipelines/compile_ms 2082, pipelines/front_ms 2024, pipelines/wait_ms 218, render/thread/ms 102, render/busy/ms 100, pipelines/driver_ms 58.43, main/busy/ms 58.07, stage/render/prepare_assets 35.40, stage/main/post_update 24.33, stage/render/prepare_write_phase_buffers 11.98, stage/main/pre_update 11.92, stage/render/render 11.69 |
| warm-up | 4.35 | 37.2 | pipelines/compile_ms 2393, pipelines/front_ms 2324, pipelines/wait_ms 371, render/thread/ms 86.19, render/busy/ms 84.07, pipelines/driver_ms 68.46, main/busy/ms 58.94, encode//sum 36.65, stage/main/post_update 33.44, stage/render/prepare_assets 20.79, stage/render/render 19.78, render/graph/ms 17.61 |
| warm-up | 4.41 | 58.2 | pipelines/compile_ms 1056, pipelines/front_ms 1023, pipelines/wait_ms 415, render/thread/ms 86.51, render/busy/ms 84.88, main/busy/ms 43.75, encode//sum 38.35, pipelines/driver_ms 33.31, stage/render/render 26.12, stage/main/post_update 24.95, stage/render/prepare_assets 24.33, render/graph/ms 23.89 |
| warm-up | 4.48 | 76.4 | pipelines/compile_ms 1322, pipelines/front_ms 1282, pipelines/wait_ms 385, render/thread/ms 127, render/busy/ms 124, main/busy/ms 118, stage/main/update 94.56, encode//sum 55.58, stream/drive/ms 53.34, stage/render/render 46.36, render/graph/ms 43.37, pipelines/driver_ms 40.13 |
| warm-up | 4.57 | 85.0 | pipelines/compile_ms 2511, pipelines/front_ms 2441, pipelines/wait_ms 790, main/busy/ms 121, render/thread/ms 118, render/busy/ms 115, stage/main/update 97.05, pipelines/driver_ms 70.35, stream/drive/ms 54.10, encode//sum 53.87, stage/render/prepare_assets 44.65, stage/render/render 41.46 |
| warm-up | 4.60 | 36.1 | pipelines/compile_ms 2223, pipelines/front_ms 2163, pipelines/wait_ms 813, main/busy/ms 101, render/thread/ms 76.64, render/busy/ms 74.79, stage/main/update 73.63, stream/drive/ms 64.93, pipelines/driver_ms 60.13, stage/render/prepare_assets 33.34, stage/render/render 22.24, render/graph/ms 19.41 |
| warm-up | 4.71 | 109.4 | pipelines/compile_ms 752, pipelines/front_ms 731, pipelines/wait_ms 346, main/busy/ms 103, stage/main/update 81.30, render/thread/ms 75.73, render/busy/ms 74.17, stream/drive/ms 66.00, stage/render/prepare_assets 33.11, pipelines/driver_ms 21.99, stage/render/render 20.17, render/graph/ms 18.02 |
| warm-up | 4.77 | 62.4 | pipelines/compile_ms 771, pipelines/front_ms 747, pipelines/wait_ms 203, main/busy/ms 92.96, render/thread/ms 82.32, render/busy/ms 80.81, stage/main/update 70.78, stage/render/prepare_assets 52.98, stream/drive/ms 51.03, pipelines/driver_ms 23.79, stage/main/post_update 12.62, stage/render/render 11.92 |
| warm-up | 4.84 | 69.3 | pipelines/compile_ms 1554, pipelines/front_ms 1514, main/busy/ms 93.90, stage/main/update 71.09, render/thread/ms 69.81, render/busy/ms 68.22, stream/drive/ms 49.55, stage/render/prepare_assets 44.16, pipelines/driver_ms 39.46, stage/main/post_update 14.53, stage/render/render 11.54, render/graph/ms 9.43 |
| warm-up | 4.87 | 29.8 | pipelines/compile_ms 1125, pipelines/front_ms 1096, main/busy/ms 72.76, stage/main/update 53.33, render/thread/ms 52.27, render/busy/ms 50.95, stream/drive/ms 32.13, stage/render/prepare_assets 28.90, pipelines/driver_ms 28.82, stage/main/post_update 12.39, stage/render/render 11.44, render/graph/ms 9.68 |
| warm-up | 4.93 | 57.1 | pipelines/compile_ms 298, pipelines/front_ms 288, main/busy/ms 66.44, stage/main/update 49.52, render/thread/ms 47.22, render/busy/ms 44.45, stream/drive/ms 32.29, stage/render/prepare_assets 20.37, stage/render/render 12.09, pipelines/driver_ms 10.61, stage/main/post_update 9.86, render/graph/ms 8.77 |
| warm-up | 4.97 | 36.5 | pipelines/compile_ms 417, pipelines/front_ms 402, render/thread/ms 56.08, render/busy/ms 53.03, main/busy/ms 46.90, stage/render/render 21.17, stage/main/update 20.03, stage/main/post_update 19.61, render/graph/ms 17.55, stage/render/prepare_assets 15.84, pipelines/driver_ms 14.98, stage/render/graph_submit 14.08 |
| warm-up | 5.00 | 29.2 | pipelines/compile_ms 327, pipelines/front_ms 312, render/thread/ms 54.87, render/busy/ms 53.35, main/busy/ms 45.50, stage/main/post_update 21.81, stage/render/render 20.99, render/graph/ms 18.69, stage/main/update 15.79, stage/render/graph_submit 14.89, pipelines/driver_ms 14.65, stage/render/prepare_assets 11.56 |
| warm-up | 5.07 | 40.8 | pipelines/compile_ms 552, pipelines/front_ms 538, render/thread/ms 62.13, render/busy/ms 60.61, main/busy/ms 38.11, stage/render/prepare_assets 26.86, stage/main/update 19.17, pipelines/driver_ms 14.78, stage/main/post_update 12.52, stage/render/render 12.17, render/graph/ms 10.11, encode//sum 7.85 |
| warm-up | 5.10 | 34.6 | pipelines/compile_ms 672, pipelines/front_ms 653, render/thread/ms 66.28, render/busy/ms 64.60, stage/render/prepare_assets 40.32, main/busy/ms 36.06, pipelines/driver_ms 18.94, stage/main/update 17.66, stage/main/post_update 11.66, stage/render/render 11.18, render/graph/ms 8.90, encode//sum 7.23 |
| warm-up | 5.15 | 53.0 | pipelines/compile_ms 1140, pipelines/front_ms 1106, render/thread/ms 74.85, render/busy/ms 73.22, main/busy/ms 59.23, pipelines/driver_ms 33.44, stage/render/prepare_assets 27.76, pipelines/wait_ms 25.77, stage/main/update 25.17, stage/render/prepare_write_phase_buffers 18.22, stage/main/pre_update 15.96, stage/main/post_update 12.34 |
| warm-up | 5.19 | 36.3 | pipelines/compile_ms 1126, pipelines/front_ms 1092, pipelines/wait_ms 93.73, render/thread/ms 77.76, render/busy/ms 76.42, main/busy/ms 64.87, pipelines/driver_ms 34.04, stage/main/update 26.16, stage/render/prepare_write_phase_buffers 24.49, stage/render/prepare_assets 20.27, stage/main/post_update 17.04, stage/main/pre_update 15.90 |
| warm-up | 5.24 | 35.4 | pipelines/compile_ms 203, pipelines/front_ms 198, render/thread/ms 42.94, render/busy/ms 41.36, main/busy/ms 29.55, stage/render/prepare_assets 16.84, stage/render/render 13.55, stage/main/update 12.30, stage/main/post_update 11.60, render/graph/ms 11.20, encode//sum 7.83, stage/render/graph_submit 7.49 |
| warm-up | 5.27 | 32.0 | pipelines/compile_ms 783, pipelines/front_ms 762, render/thread/ms 53.78, render/busy/ms 52.45, stage/render/prepare_assets 32.04, main/busy/ms 30.92, pipelines/driver_ms 21.12, stage/main/update 13.63, stage/main/post_update 11.14, stage/render/render 10.60, render/graph/ms 8.32, encode//sum 7.96 |
| warm-up | 5.34 | 30.4 | render/thread/ms 34.32, main/busy/ms 32.54, render/busy/ms 32.40, stage/main/update 18.42, stage/render/prepare_assets 17.00, stage/render/render 11.28, stream/drive/ms 9.80, render/graph/ms 8.61, stage/main/post_update 8.58, encode//sum 7.12, stage/render/graph_submit 6.15, encode/submit 4.57 |
| warm-up | 5.40 | 37.9 | render/thread/ms 48.52, render/busy/ms 47.31, stage/render/prepare_assets 25.15, main/busy/ms 25.05, stage/main/post_update 10.45, stage/render/render 9.90, stage/main/update 8.67, render/graph/ms 8.19, encode//sum 7.96, stage/render/graph_submit 5.98, encode/submit 3.84, stage/render/prepare_write_phase_buffers 2.73 |
| warm-up | 5.45 | 33.7 | pipelines/compile_ms 180, pipelines/front_ms 172, render/thread/ms 41.13, render/busy/ms 40.08, stage/render/prepare_assets 24.88, main/busy/ms 21.40, stage/main/post_update 9.49, stage/render/render 9.00, pipelines/driver_ms 8.34, render/graph/ms 7.51, stage/main/update 7.43, encode//sum 6.96 |
| warm-up | 5.93 | 30.6 | render/thread/ms 38.13, render/busy/ms 37.06, main/busy/ms 28.18, stage/render/prepare_assets 17.18, stage/main/update 11.83, stage/main/post_update 9.81, stage/render/render 8.14, encode//sum 7.51, render/graph/ms 6.60, stage/render/graph_submit 4.51, stage/render/specialize 3.40, stream/drive/ms 3.19 |
| warm-up | 6.27 | 32.8 | render/thread/ms 43.33, render/busy/ms 42.00, main/busy/ms 41.85, stage/main/update 19.25, stage/main/post_update 15.44, stage/render/prepare_assets 15.16, stage/render/render 12.61, render/graph/ms 10.65, encode//sum 9.58, stage/render/graph_submit 6.94, stream/drive/ms 4.13, stage/render/camera_driver 3.59 |
| warm-up | 6.66 | 35.5 | pipelines/compile_ms 148, pipelines/front_ms 144, main/busy/ms 40.72, render/thread/ms 23.10, render/busy/ms 21.86, stage/main/update 21.43, stream/drive/ms 14.34, stage/main/post_update 13.98, stream/spawn/ms 13.09, stage/render/render 10.24, render/graph/ms 8.59, encode//sum 7.43 |
| warm-up | 6.76 | 30.4 | render/thread/ms 39.59, render/busy/ms 38.45, main/busy/ms 20.52, stage/render/render 11.82, stage/main/post_update 11.58, render/graph/ms 10.21, stage/render/specialize 9.88, stage/render/graph_submit 7.92, encode//sum 7.67, stage/render/queue 7.05, stage/main/update 5.93, encode/submit 4.65 |
| warm-up | 8.06 | 31.3 | render/thread/ms 39.25, render/busy/ms 37.99, main/busy/ms 19.03, stage/render/render 11.45, stage/main/post_update 10.89, render/graph/ms 9.78, stage/render/specialize 9.65, stage/render/graph_submit 7.48, encode//sum 7.02, stage/render/queue 7.01, stage/main/update 5.15, encode/submit 4.38 |
| measure | 0.43 | 28.8 | render/thread/ms 37.89, render/busy/ms 36.81, main/busy/ms 17.71, stage/render/render 13.34, render/graph/ms 11.79, stage/main/post_update 10.49, stage/render/graph_submit 9.64, stage/render/specialize 8.87, encode//sum 7.75, encode/submit 6.76, stage/render/queue 6.28, stage/main/update 4.35 |
| measure | 1.76 | 29.1 | render/thread/ms 34.96, render/busy/ms 33.93, main/busy/ms 17.80, stage/main/post_update 10.57, stage/render/render 10.45, render/graph/ms 9.00, stage/render/specialize 8.12, encode//sum 7.00, stage/render/graph_submit 6.43, stage/render/queue 5.60, stage/render/prepare_meshes 4.73, stage/main/update 4.34 |
| measure | 5.86 | 29.5 | render/thread/ms 34.50, render/busy/ms 33.42, main/busy/ms 18.75, stage/main/post_update 12.16, stage/render/render 9.75, stage/render/specialize 8.76, render/graph/ms 8.18, encode//sum 6.87, stage/render/queue 6.81, stage/render/graph_submit 6.28, stage/main/update 4.05, encode/submit 3.78 |
| measure | 9.67 | 29.3 | render/thread/ms 35.16, render/busy/ms 34.10, main/busy/ms 19.80, stage/main/post_update 11.67, stage/render/render 10.24, stage/render/specialize 9.60, render/graph/ms 8.63, encode//sum 7.67, stage/render/graph_submit 6.50, stage/render/queue 5.89, stage/main/update 5.40, encode/submit 4.04 |
| measure | 18.17 | 30.7 | main/busy/ms 38.98, stage/main/update 24.59, stream/drive/ms 21.19, render/thread/ms 20.06, render/busy/ms 19.05, stage/render/render 11.54, stage/main/post_update 11.40, render/graph/ms 10.13, encode//sum 9.14, stage/render/graph_submit 7.14, encode/submit 4.69, stage/render/camera_driver 2.87 |
| measure | 19.41 | 29.6 | render/thread/ms 32.93, render/busy/ms 31.82, main/busy/ms 18.61, stage/main/post_update 11.80, stage/render/render 10.12, stage/render/specialize 8.71, render/graph/ms 8.57, encode//sum 8.12, stage/render/graph_submit 6.22, stage/render/queue 5.97, stage/main/update 4.07, encode/submit 3.96 |
| measure | 20.80 | 30.4 | render/thread/ms 35.26, render/busy/ms 34.16, main/busy/ms 18.03, stage/render/render 10.91, stage/main/post_update 10.78, render/graph/ms 9.39, stage/render/specialize 9.15, encode//sum 8.68, stage/render/graph_submit 6.93, stage/render/queue 6.45, stage/main/update 4.57, encode/submit 4.57 |
| measure | 23.00 | 32.6 | render/thread/ms 38.57, render/busy/ms 37.48, main/busy/ms 23.97, stage/main/post_update 12.19, stage/render/specialize 10.69, stage/render/render 9.95, render/graph/ms 8.53, stage/render/queue 7.79, encode//sum 7.05, stage/render/graph_submit 6.33, stage/main/update 6.16, encode/submit 3.92 |
| measure | 27.28 | 37.5 | render/thread/ms 43.17, render/busy/ms 41.88, main/busy/ms 25.81, stage/render/render 14.67, render/graph/ms 12.90, encode//sum 12.48, stage/main/post_update 12.41, stage/render/specialize 11.12, stage/render/graph_submit 9.76, stage/render/queue 7.56, stage/main/update 6.76, encode/submit 6.62 |
| measure | 28.74 | 29.1 | render/thread/ms 37.83, render/busy/ms 36.81, main/busy/ms 18.95, stage/render/render 11.58, stage/main/post_update 11.43, render/graph/ms 10.12, stage/render/specialize 9.17, encode//sum 8.99, stage/render/graph_submit 7.38, stage/render/queue 7.06, encode/submit 4.80, stage/main/update 4.29 |
| measure | 30.47 | 33.4 | render/thread/ms 38.15, render/busy/ms 37.15, main/busy/ms 21.97, stage/main/post_update 12.88, stage/render/render 11.21, render/graph/ms 9.63, stage/render/specialize 9.61, encode//sum 8.20, stage/render/graph_submit 7.44, stage/render/queue 7.05, stage/main/update 5.79, encode/submit 3.95 |
| measure | 31.87 | 28.8 | render/thread/ms 35.61, render/busy/ms 34.59, main/busy/ms 19.92, stage/main/post_update 13.21, stage/render/render 10.96, render/graph/ms 9.50, stage/render/specialize 9.14, stage/render/graph_submit 7.25, encode//sum 7.20, stage/render/queue 6.21, encode/submit 4.32, stage/main/update 4.27 |
| measure | 35.36 | 31.0 | render/thread/ms 39.55, render/busy/ms 38.20, main/busy/ms 19.47, stage/render/render 12.85, stage/main/post_update 12.00, render/graph/ms 11.23, stage/render/specialize 9.70, stage/render/graph_submit 8.66, encode//sum 8.33, stage/render/queue 6.58, encode/submit 5.35, encode/pass:shadow_point_light/sum 4.72 |
| measure | 36.78 | 29.4 | render/thread/ms 36.78, render/busy/ms 35.59, main/busy/ms 18.91, stage/render/render 12.10, stage/main/post_update 12.07, render/graph/ms 10.47, encode//sum 9.21, stage/render/specialize 8.69, stage/render/graph_submit 7.98, stage/render/queue 6.20, encode/pass:shadow_point_light/sum 5.11, encode/submit 4.71 |
| measure | 40.79 | 33.2 | render/thread/ms 39.19, render/busy/ms 38.00, main/busy/ms 20.44, stage/main/post_update 12.15, stage/render/render 10.86, stage/render/specialize 9.64, render/graph/ms 9.30, stage/render/graph_submit 7.35, encode//sum 6.93, stage/render/queue 6.44, stage/main/update 4.66, encode/submit 4.03 |
| measure | 43.21 | 29.2 | render/thread/ms 35.46, render/busy/ms 34.43, main/busy/ms 18.26, stage/main/post_update 10.78, stage/render/render 10.01, stage/render/specialize 9.15, render/graph/ms 8.56, encode//sum 6.94, stage/render/graph_submit 6.81, stage/render/queue 6.11, stage/main/update 3.97, encode/submit 3.65 |
| measure | 45.68 | 33.3 | render/thread/ms 35.28, render/busy/ms 34.32, main/busy/ms 18.58, stage/main/post_update 11.10, stage/render/specialize 9.31, stage/render/render 9.27, render/graph/ms 7.99, encode//sum 6.65, stage/render/graph_submit 6.12, stage/render/queue 6.02, stage/render/prepare_meshes 4.42, stage/main/update 4.36 |
| measure | 56.44 | 28.6 | render/thread/ms 35.74, render/busy/ms 34.70, main/busy/ms 15.49, stage/render/render 10.40, render/graph/ms 9.07, stage/main/post_update 8.85, stage/render/specialize 8.76, encode//sum 7.71, stage/render/graph_submit 7.00, stage/render/queue 6.31, stage/main/update 4.11, encode/submit 3.79 |
| measure | 59.01 | 43.4 | render/thread/ms 54.06, render/busy/ms 52.46, main/busy/ms 32.66, stage/main/post_update 18.32, stage/render/render 16.34, render/graph/ms 14.24, encode//sum 11.90, stage/render/graph_submit 11.34, stage/render/specialize 10.86, stage/main/update 9.21, stage/render/prepare_meshes 8.25, encode/submit 5.96 |
| measure | 59.04 | 31.0 | render/thread/ms 57.41, render/busy/ms 56.33, main/busy/ms 27.44, stage/render/prepare_assets 15.54, stage/main/post_update 13.79, stage/render/specialize 10.91, stage/render/render 9.67, stage/render/prepare_meshes 9.01, render/graph/ms 8.34, encode//sum 8.02, stage/main/update 7.18, stage/render/graph_submit 6.31 |
| measure | 62.56 | 31.3 | render/thread/ms 37.30, render/busy/ms 35.40, main/busy/ms 18.77, stage/render/render 10.88, stage/main/post_update 10.17, stage/render/specialize 9.36, render/graph/ms 8.72, encode//sum 7.00, stage/render/graph_submit 6.54, stage/render/queue 6.03, stage/main/update 5.99, stage/render/prepare_meshes 4.87 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set render.depth_only.skip_material=on --resolution 2560x1440 --bench-out target/perf\pt_nomat\balmora_walk@2560x1440_arm=nomat-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 11%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **77%** (min 49%) · 2625 MHz (min 2625) · memory 8101 MHz · 80.1 W · 75 °C (max 80) · VRAM 6709 MiB · throttled 100% of samples

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
- `render.depth_only.skip_material=on (command line: --set)`
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
