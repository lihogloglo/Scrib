# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6065 frames over 65.6 s · **median 10.58 ms (95 fps)** · mean 10.82 · p95 13.30 · p99 16.51 (1% low 61 fps) · worst 42.67 ms · 78 spikes over 1.5× median · 59.4 stutters/min (work 50.3/min)
- warm-up: 324 frames over 8.9 s, worst 2468.4 ms
- hitches (> 26.4 ms): **32** measured, 50 in warm-up
- timed GPU passes: 6.53 ms of a 10.82 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.58 ms, limit 11 ms
- **FAIL** frame_ms: 65 stutters (> 1.5x moving median of 60) in 1.09 min = 59.41/min, limit 1/min
- **FAIL** worst frame 42.67 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.848 | 2.039 | 2.068 | 0.91 | 0.175 |
| main_opaque_pass_3d | 1.480 | 1.633 | 2.043 | 0.91 | 0.059 |
| early prepass | 0.443 | 0.489 | 0.716 | 0.91 | 0.088 |
| bloom | 0.286 | 0.315 | 0.321 | 0.91 | 0.064 |
| atmosphere_luts | 0.275 | 0.303 | 0.366 | 0.91 | 0.003 |
| water_velocity | 0.254 | 0.280 | 0.347 | 0.91 | 0.002 |
| water_sspr | 0.183 | 0.202 | 0.203 | 0.91 | 0.002 |
| ssao | 0.172 | 0.189 | 0.193 | 0.91 | 0.007 |
| atmos_froxel_inject | 0.132 | 0.146 | 0.147 | 0.91 | 0.001 |
| auxiliary_views | 0.128 | 0.141 | 0.510 | 0.91 | 0.140 |
| clouds_trace | 0.126 | 0.139 | 0.230 | 0.91 | 0.001 |
| water_fft | 0.118 | 0.130 | 0.137 | 0.91 | 0.003 |
| uniform_allocation | 0.117 | 0.129 | 0.180 | 0.91 | 0.023 |
| upscaling | 0.106 | 0.117 | 0.125 | 0.91 | 0.004 |
| clouds_composite | 0.096 | 0.106 | 0.109 | 0.91 | 0.002 |
| bin_unpacking | 0.094 | 0.104 | 0.142 | 0.91 | 0.017 |
| tonemapping | 0.077 | 0.085 | 0.088 | 0.91 | 0.003 |
| main_transmissive_pass_3d | 0.077 | 0.085 | 0.177 | 0.91 | 0.008 |
| clustering | 0.058 | 0.064 | 0.067 | 0.91 | 0.018 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.91 | 0.002 |
| clouds_resolve | 0.044 | 0.049 | 0.049 | 0.91 | 0.001 |
| lightprobe_irradiance_map | 0.043 | 0.417 | 0.417 | 0.10 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.91 | 0.001 |
| surface_trail | 0.036 | 0.056 | 0.057 | 0.64 | 0.002 |
| surface_exposure_far | 0.034 | 0.043 | 0.045 | 0.80 | 0.002 |
| water_snapshot | 0.029 | 0.032 | 0.036 | 0.91 | 0.002 |
| atmos_froxel_integrate | 0.028 | 0.031 | 0.032 | 0.91 | 0.001 |
| clouds_shadow | 0.028 | 0.030 | 0.033 | 0.91 | 0.003 |
| early_mesh_preprocessing | 0.027 | 0.030 | 0.046 | 0.91 | 0.004 |
| lightprobe_radiance_map | 0.023 | 0.219 | 0.229 | 0.10 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.91 | 0.002 |
| night_sky | 0.012 | 0.014 | 0.033 | 0.91 | 0.001 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.026 | 0.91 | 0.125 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.91 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.91 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.91 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.90 | 0.003 |
| lightprobe_copy | 0.002 | 0.014 | 0.015 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.304 | 11.552 | 36.93 | 56430.5 |
| main/busy/ms | 8.986 | 10.650 | 29.26 | 54497.5 |
| render/busy/ms | 8.735 | 11.021 | 36.40 | 52980.7 |
| stage/main/post_update | 5.595 | 6.918 | 10.16 | 33936.1 |
| stage/render/render | 5.435 | 6.979 | 15.38 | 32962.6 |
| render/graph/ms | 4.676 | 6.221 | 14.32 | 28360.1 |
| encode//sum | 3.952 | 5.418 | 22.43 | 23966.7 |
| stage/render/graph_submit | 3.512 | 4.756 | 11.63 | 21298.9 |
| encode/submit | 2.259 | 3.244 | 6.47 | 13698.4 |
| stage/main/update | 2.130 | 2.541 | 22.65 | 12916.0 |
| encode/finish_wall | 1.340 | 1.963 | 6.15 | 8124.6 |
| stage/render/camera_driver | 1.111 | 1.533 | 4.32 | 6739.5 |
| encode/pass:shadow_directional_light/sum | 0.835 | 1.536 | 5.44 | 5064.5 |
| encode/pass:shadow_point_light/sum | 0.789 | 3.015 | 15.74 | 4786.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.749 | 1.071 | 1.99 | 4544.0 |
| stage/render/prepare_write_phase_buffers | 0.748 | 1.064 | 2.17 | 4537.1 |
| encode/pass:early prepass/sum | 0.725 | 1.050 | 1.94 | 4396.9 |
| stage/render/prepare_assets | 0.563 | 0.726 | 12.26 | 3411.7 |
| bigspace/total/ms | 0.538 | 1.323 | 2.81 | 3260.5 |
| render/present/ms | 0.473 | 1.226 | 2.39 | 2868.4 |
| stage/render/prepare_resources | 0.453 | 0.542 | 1.93 | 2749.4 |
| stage/render/prepare_meshes | 0.439 | 0.553 | 6.84 | 2663.4 |
| stage/main/first | 0.399 | 0.521 | 5.53 | 2422.6 |
| stage/main/pre_update | 0.355 | 0.528 | 1.38 | 2153.9 |
| stage/render/specialize | 0.320 | 0.418 | 13.05 | 1942.1 |
| terrain/update/ms | 0.312 | 0.400 | 2.46 | 1892.7 |
| stage/render/queue | 0.284 | 0.408 | 7.37 | 1719.8 |
| terrain/select/ms | 0.279 | 0.359 | 0.90 | 1690.6 |
| stage/render/prepare_bind_groups | 0.273 | 0.485 | 2.12 | 1652.9 |
| controller/feed/ms | 0.263 | 0.329 | 0.73 | 1597.1 |
| stream/drive/ms | 0.247 | 0.330 | 20.34 | 1499.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.234 | 0.341 | 1.00 | 1420.6 |
| bigspace/low_precision/ms | 0.223 | 0.749 | 2.13 | 1354.8 |
| stage/main/fixed_main_loop | 0.212 | 0.367 | 1.16 | 1286.0 |
| far/select/ms | 0.199 | 0.265 | 0.56 | 1208.8 |
| stage/render/create_views | 0.196 | 0.251 | 1.14 | 1187.8 |
| stream/tick/ms | 0.191 | 0.263 | 0.68 | 1157.3 |
| stage/render/prepare_batch_phases | 0.189 | 0.269 | 0.55 | 1148.0 |
| stage/render/extract_commands | 0.170 | 0.201 | 0.45 | 1031.4 |
| anim/evaluate/ms | 0.162 | 0.283 | 1.63 | 984.0 |
| stage/render/prepare_flush | 0.158 | 0.286 | 1.06 | 955.5 |
| bigspace/high_precision/ms | 0.153 | 0.661 | 1.67 | 925.7 |
| anim/evaluate_cpu/ms | 0.152 | 0.188 | 0.71 | 922.8 |
| stage/render/prepare_views | 0.140 | 0.175 | 0.50 | 846.5 |
| controller/move/ms | 0.128 | 0.267 | 0.51 | 773.9 |
| stage/main/last | 0.116 | 0.160 | 2.77 | 704.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.098 | 0.162 | 0.60 | 593.6 |
| render/acquire/ms | 0.096 | 0.123 | 0.38 | 581.4 |
| stage/render/prepare_collect_phase_buffers | 0.063 | 0.132 | 0.46 | 384.6 |
| pipelines/compile_ms | 0.047 | 0.000 | 122.71 | 285.7 |
| physics/step/ms | 0.046 | 0.083 | 0.57 | 276.1 |
| pipelines/front_ms | 0.045 | 0.000 | 119.09 | 274.0 |
| stage/render/phase_sort | 0.034 | 0.061 | 0.23 | 206.3 |
| physics/insert/ms | 0.032 | 0.360 | 0.93 | 193.0 |
| stage/render/cleanup | 0.032 | 0.096 | 0.68 | 191.3 |
| encode/resolve_diagnostics/sum | 0.029 | 0.040 | 0.53 | 177.9 |
| weather/map/ms | 0.025 | 0.033 | 0.47 | 152.4 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.022 | 0.04 | 86.9 |
| anim/apply/ms | 0.011 | 0.021 | 0.24 | 68.6 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.12 | 64.4 |
| controller/avoid/ms | 0.011 | 0.014 | 0.20 | 64.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.10 | 62.7 |
| script/update/ms | 0.007 | 0.009 | 0.17 | 41.3 |
| script/input/ms | 0.005 | 0.007 | 0.19 | 32.4 |
| script/ui/ms | 0.005 | 0.006 | 0.53 | 31.9 |
| stream/despawn/ms | 0.005 | 0.000 | 1.18 | 30.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 28.9 |
| controller/write/ms | 0.003 | 0.004 | 0.12 | 20.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.59 | 13.4 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.74 | 11.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 6.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.40 | 4.8 |
| stage/render/prepare | 0.001 | 0.002 | 0.13 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.12 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.10 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 30 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 594 |
| crowd/c0/meshes/count | count | 353 | 646 | 150 |
| crowd/c1/count | count | 145 | 156 | 148 |
| crowd/c1/joints/count | count | 5531 | 5996 | 5602 |
| crowd/c1/meshes/count | count | 1177 | 1310 | 1191 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4610 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4593 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.41 | 1.45 | 0.44 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.04 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.12 | 0.01 |
| encode/finish_critical | ms | 0.94 | 5.44 | 1.17 |
| encode/pass:early prepass/max | ms | 0.74 | 1.94 | 0.96 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 1.99 | 1.17 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.60 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.00 | 0.30 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 5.44 | 0.90 |
| encode/pass:shadow_point_light/max | ms | 0.76 | 1.54 | 0.72 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.53 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.18 | 36.40 | 9.05 |
| gpu/timed/ms | ms | 7.94 | 9.28 | 8.84 |
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
| phases/opaque3d/bins | count | 1292 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3403 | 5142 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1292 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3403 | 5142 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 16 | 192 | 4 |
| phases/shadow/bins | count | 5035 | 8971 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6376 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 200 | 246 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 181 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 541 | 6 |
| preprocess/bind_groups_created | count | 13 | 541 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 212 | 123 |
| process/cpu_usage | % | 26.54 | 39.71 | 17.31 |
| process/mem_usage | GiB | 5.48 | 5.65 | 5.65 |
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
| skin/blocks_changed/count | count | 14 | 129 | 10 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 365.6 k | 812.3 k | 486.7 k |
| skin/joints_written/count | count | 5712 | 12.7 k | 7604 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4915 | 5009 | 5009 |
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
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.17 | 41.96 | 36.41 |
| system/mem_usage | % | 56.86 | 57.37 | 57.36 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 65 | 239 | 54 |
| views/all/bins | count | 7645 | 11.9 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 245 | 293 | 75 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.7 k | 35.6 k | 8119 |
| views/cascade/batch_sets | count | 9 | 51 | 4 |
| views/cascade/bins | count | 2800 | 6199 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8897 | 23.5 k | 6376 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 410 | 23.5 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 186 | 0 |
| views/lamp_face/bins | count | 2172 | 2841 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2431 | 3135 | 0 |
| views/lamp_face/pipelines | count | 164 | 203 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2883 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2878 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8426 | 13.1 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8585 | 13.3 k | 7918 |
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
| anim/bones_written/count | count | 59.07 | 303 | 358.3 k |
| anim/events/count | count | 0.09 | 3 | 566 |
| assets/Image/added | count | 0.10 | 43 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.57 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1309 | 4675 |
| encode//count | count | 33.33 | 220 | 202.2 k |
| encode/bind_walk/init_textures | count | 12.4 k | 114.7 k | 75.47 M |
| encode/bind_walk/init_textures_skipped | count | 60.8 k | 122.6 k | 368.75 M |
| encode/bind_walk/inits | count | 456 | 1100 | 2.77 M |
| encode/bind_walk/inits_skipped | count | 105 | 158 | 636.8 k |
| encode/bind_walk/merge_textures | count | 12.4 k | 114.6 k | 75.03 M |
| encode/bind_walk/merge_textures_skipped | count | 5366 | 6012 | 32.55 M |
| encode/bind_walk/merges | count | 98.19 | 329 | 595.5 k |
| encode/bind_walk/merges_skipped | count | 82.65 | 131 | 501.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6065 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6065 |
| encode/pass:early prepass/count | count | 1 | 1 | 6065 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6065 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6065 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6065 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6065 |
| encode/pass:shadow_point_light/count | count | 1.35 | 48 | 8162 |
| encode/queue/command_buffers | count | 44.69 | 278 | 271.1 k |
| encode/queue/submits | count | 2.02 | 4 | 12.2 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.88 M | 11.41 G |
| encode/queue/write_buffer/calls | count | 256 | 807 | 1.55 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.5 k | 3.52 M | 1.08 G |
| encode/queue/write_buffer_with/calls | count | 10.99 | 465 | 66.7 k |
| encode/queue/write_texture/bytes | bytes | 11.7 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6065 |
| physics/inserts | count | 0.37 | 107 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2335 |
| physics/steps | count | 0.65 | 2 | 3939 |
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
| stream/spawn/items | count | 0.06 | 21 | 378 |
| stream/upload/bytes | bytes | 5582 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1224 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.4 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2468.4 | main/busy/ms 2439, stage/main/post_update 2326, stage/main/update 104, surface/advance_cpu_ms 74.26, stage/main/pre_update 4.69, stage/main/last 0.97, surface/cell_upload_ms 0.71, stage/main/first 0.48, stream/drive/ms 0.47, stage/main/fixed_main_loop 0.40, weather/map/ms 0.17, bigspace/total/ms 0.11 |
| warm-up | 3.23 | 764.8 | main/busy/ms 2587, stage/main/post_update 2350, render/thread/ms 727, render/busy/ms 600, stage/render/prepare_assets 549, stage/main/update 223, stage/render/prepare_views 123, render/acquire/ms 123, surface/advance_cpu_ms 74.26, stage/render/create_views 29.06, stage/render/render 14.33, render/graph/ms 9.26 |
| warm-up | 3.35 | 112.8 | render/thread/ms 822, render/busy/ms 643, stage/render/prepare_assets 573, main/busy/ms 170, stage/render/prepare_views 124, stage/main/update 123, render/acquire/ms 123, pipelines/compile_ms 109, stage/render/render 74.31, pipelines/front_ms 69.02, render/present/ms 56.37, pipelines/driver_ms 40.04 |
| warm-up | 3.39 | 48.4 | pipelines/compile_ms 490, pipelines/front_ms 423, pipelines/wait_ms 189, render/thread/ms 141, stage/render/render 98.26, render/busy/ms 72.52, main/busy/ms 68.30, render/present/ms 67.89, pipelines/driver_ms 67.48, stage/main/post_update 50.85, render/graph/ms 28.95, stage/render/prepare_assets 26.06 |
| warm-up | 3.52 | 127.6 | pipelines/wait_ms 3889, pipelines/compile_ms 1065, pipelines/front_ms 1001, render/thread/ms 171, render/busy/ms 154, main/busy/ms 149, pipelines/driver_ms 64.62, stage/main/update 54.50, stage/main/post_update 51.33, stage/render/render 48.12, stage/render/prepare_assets 33.36, stage/main/first 32.11 |
| warm-up | 3.59 | 66.1 | pipelines/wait_ms 3875, pipelines/compile_ms 1761, pipelines/front_ms 1637, render/thread/ms 188, render/busy/ms 186, main/busy/ms 139, pipelines/driver_ms 123, stage/render/prepare_assets 83.60, stage/main/update 55.83, stage/main/first 32.00, stage/main/post_update 29.96, stage/render/prepare_bind_groups 28.85 |
| warm-up | 3.63 | 38.8 | pipelines/compile_ms 1273, pipelines/front_ms 1177, pipelines/wait_ms 291, render/thread/ms 99.37, render/busy/ms 97.58, pipelines/driver_ms 96.18, stage/render/prepare_assets 75.11, main/busy/ms 72.17, stage/main/pre_update 26.01, stage/main/post_update 23.85, terrain/update/ms 15.84, stage/render/render 10.92 |
| warm-up | 3.68 | 32.0 | pipelines/compile_ms 1449, pipelines/front_ms 1386, pipelines/wait_ms 259, pipelines/driver_ms 63.34, render/thread/ms 49.80, render/busy/ms 48.19, main/busy/ms 27.73, stage/render/prepare_assets 19.31, stage/main/post_update 17.72, encode//sum 14.14, stage/render/render 10.86, stage/render/prepare_resources 9.20 |
| warm-up | 3.75 | 36.9 | pipelines/compile_ms 447, pipelines/front_ms 426, main/busy/ms 65.11, render/thread/ms 52.25, stage/main/update 49.77, stream/drive/ms 46.53, render/busy/ms 45.87, stage/render/prepare_assets 27.92, pipelines/driver_ms 20.80, stage/render/render 14.47, stage/main/post_update 10.46, pipelines/wait_ms 9.75 |
| warm-up | 3.82 | 72.7 | pipelines/compile_ms 1047, pipelines/front_ms 997, render/thread/ms 77.90, render/busy/ms 71.51, main/busy/ms 66.46, stage/main/update 51.85, stage/render/prepare_assets 51.46, pipelines/driver_ms 49.73, stream/drive/ms 47.68, pipelines/wait_ms 36.23, stage/render/render 16.22, render/graph/ms 10.22 |
| warm-up | 3.90 | 31.1 | pipelines/compile_ms 277, pipelines/front_ms 261, render/thread/ms 41.95, main/busy/ms 41.48, render/busy/ms 40.25, stage/main/update 29.83, stream/drive/ms 25.73, stage/render/prepare_assets 20.86, pipelines/driver_ms 15.22, stage/render/render 12.18, render/graph/ms 9.88, stage/main/post_update 7.21 |
| warm-up | 3.96 | 62.9 | pipelines/compile_ms 493, pipelines/front_ms 470, render/thread/ms 71.70, render/busy/ms 69.94, stage/render/prepare_assets 47.44, main/busy/ms 44.51, stage/main/update 31.79, stream/drive/ms 27.58, pipelines/driver_ms 23.35, stage/render/render 14.61, render/graph/ms 12.02, stage/render/graph_submit 8.07 |
| warm-up | 4.08 | 92.9 | pipelines/compile_ms 860, pipelines/front_ms 833, render/thread/ms 77.11, render/busy/ms 75.57, main/busy/ms 64.43, stage/render/prepare_assets 59.31, stage/main/update 42.26, pipelines/driver_ms 27.38, stage/main/post_update 13.35, stage/render/render 8.65, render/graph/ms 6.58, encode//sum 5.46 |
| warm-up | 4.18 | 107.4 | pipelines/compile_ms 1596, pipelines/front_ms 1527, render/thread/ms 149, render/busy/ms 134, stage/render/prepare_assets 100, pipelines/driver_ms 69.28, main/busy/ms 65.20, stage/main/update 43.10, stage/render/render 38.12, render/graph/ms 22.83, stage/render/graph_submit 20.70, encode/submit 17.32 |
| warm-up | 4.23 | 42.4 | pipelines/compile_ms 916, pipelines/front_ms 866, pipelines/wait_ms 264, render/thread/ms 124, render/busy/ms 109, stage/render/prepare_assets 58.28, pipelines/driver_ms 49.58, stage/render/render 40.67, main/busy/ms 39.60, render/graph/ms 25.27, stage/render/graph_submit 21.94, stage/main/update 20.96 |
| warm-up | 4.31 | 89.2 | pipelines/compile_ms 821, pipelines/front_ms 791, pipelines/wait_ms 421, render/thread/ms 110, render/busy/ms 106, main/busy/ms 83.35, stage/main/update 46.40, stage/render/render 35.87, render/graph/ms 33.05, pipelines/driver_ms 29.89, encode//sum 29.08, stage/main/post_update 25.40 |
| warm-up | 4.38 | 60.5 | pipelines/compile_ms 2004, pipelines/front_ms 1939, pipelines/wait_ms 315, render/thread/ms 131, render/busy/ms 127, encode//sum 96.24, main/busy/ms 91.57, pipelines/driver_ms 64.93, stage/render/render 46.98, render/graph/ms 43.98, stage/main/update 41.90, stage/main/post_update 35.67 |
| warm-up | 4.44 | 65.0 | pipelines/compile_ms 1837, pipelines/front_ms 1771, pipelines/wait_ms 607, encode//sum 126, render/thread/ms 111, render/busy/ms 109, pipelines/driver_ms 65.92, main/busy/ms 53.22, stage/render/render 38.86, render/graph/ms 36.01, stage/render/graph_submit 28.37, stage/main/post_update 21.53 |
| warm-up | 4.48 | 36.0 | pipelines/compile_ms 957, pipelines/front_ms 917, pipelines/wait_ms 641, main/busy/ms 88.87, render/thread/ms 83.97, render/busy/ms 82.02, stage/main/update 62.67, encode//sum 57.95, stream/drive/ms 53.94, pipelines/driver_ms 39.95, stage/render/render 26.01, render/graph/ms 23.33 |
| warm-up | 4.56 | 82.3 | pipelines/compile_ms 1458, pipelines/front_ms 1407, pipelines/wait_ms 246, main/busy/ms 103, stage/main/update 74.11, render/thread/ms 65.90, render/busy/ms 64.24, stream/drive/ms 55.14, pipelines/driver_ms 50.85, stage/render/prepare_assets 21.87, stage/main/post_update 17.59, stage/render/render 14.78 |
| warm-up | 4.62 | 62.8 | pipelines/compile_ms 2243, pipelines/front_ms 2169, pipelines/wait_ms 497, main/busy/ms 118, stage/main/update 87.07, render/thread/ms 85.29, render/busy/ms 83.46, pipelines/driver_ms 73.99, stream/drive/ms 49.89, stage/render/prepare_assets 31.69, stage/render/render 16.81, stage/main/post_update 16.35 |
| warm-up | 4.78 | 158.7 | pipelines/compile_ms 2491, pipelines/front_ms 2296, pipelines/wait_ms 452, pipelines/driver_ms 195, render/thread/ms 144, render/busy/ms 142, main/busy/ms 111, stage/render/prepare_assets 106, stage/main/update 83.61, stream/drive/ms 49.59, stage/render/render 15.26, stage/main/post_update 14.99 |
| warm-up | 4.82 | 44.2 | pipelines/compile_ms 1333, pipelines/front_ms 1172, pipelines/driver_ms 161, render/thread/ms 134, render/busy/ms 132, stage/render/prepare_assets 92.54, main/busy/ms 89.76, stage/main/update 62.92, stream/drive/ms 40.89, stage/render/render 25.30, render/graph/ms 23.28, stage/render/graph_submit 20.07 |
| warm-up | 4.90 | 72.6 | main/busy/ms 81.34, pipelines/compile_ms 71.62, render/thread/ms 69.75, render/busy/ms 68.08, pipelines/front_ms 67.46, stage/main/update 63.01, stream/drive/ms 40.27, stage/render/prepare_assets 30.15, stage/render/render 22.15, render/graph/ms 20.09, stage/render/graph_submit 17.22, encode/submit 14.98 |
| warm-up | 4.93 | 29.6 | main/busy/ms 68.25, render/thread/ms 56.99, render/busy/ms 55.70, stage/main/update 51.20, stream/drive/ms 35.19, stage/render/prepare_assets 26.59, pipelines/compile_ms 13.18, stage/render/render 12.57, pipelines/front_ms 12.24, render/graph/ms 10.74, stage/main/post_update 9.33, encode//sum 8.29 |
| warm-up | 4.96 | 37.8 | pipelines/compile_ms 1449, pipelines/front_ms 1407, main/busy/ms 56.87, pipelines/driver_ms 41.76, stage/main/update 40.73, render/thread/ms 37.87, render/busy/ms 36.48, stream/drive/ms 34.90, stage/render/render 12.74, render/graph/ms 10.98, stage/main/post_update 9.70, stage/render/prepare_assets 9.10 |
| warm-up | 5.09 | 30.7 | render/thread/ms 44.71, render/busy/ms 43.03, pipelines/compile_ms 41.18, pipelines/front_ms 38.17, main/busy/ms 30.97, stage/render/render 14.64, stage/main/update 12.92, render/graph/ms 12.28, stage/render/prepare_assets 12.02, stage/main/post_update 11.95, stage/render/graph_submit 8.92, encode//sum 8.88 |
| warm-up | 5.13 | 45.6 | pipelines/compile_ms 339, pipelines/front_ms 328, render/thread/ms 62.40, render/busy/ms 60.77, main/busy/ms 35.86, stage/render/prepare_assets 28.97, stage/render/render 18.28, stage/main/update 18.22, render/graph/ms 15.94, stage/render/graph_submit 12.48, stage/main/post_update 11.76, pipelines/driver_ms 10.99 |
| warm-up | 5.17 | 35.1 | pipelines/compile_ms 890, pipelines/front_ms 865, render/thread/ms 64.50, render/busy/ms 62.95, stage/render/prepare_assets 36.12, main/busy/ms 36.08, pipelines/driver_ms 25.06, stage/main/update 21.51, stage/render/render 16.70, render/graph/ms 14.54, stage/render/graph_submit 11.51, stage/main/post_update 8.78 |
| warm-up | 5.21 | 35.5 | pipelines/compile_ms 596, pipelines/front_ms 579, render/thread/ms 59.97, render/busy/ms 58.71, main/busy/ms 48.71, stage/main/post_update 27.98, stage/render/prepare_assets 21.10, pipelines/driver_ms 16.75, stage/main/update 15.03, stage/render/render 13.91, render/graph/ms 12.00, stage/render/prepare_resources 10.83 |
| warm-up | 5.25 | 39.5 | render/thread/ms 70.91, render/busy/ms 69.59, main/busy/ms 67.89, pipelines/compile_ms 59.37, pipelines/front_ms 56.46, stage/main/post_update 40.20, pipelines/wait_ms 30.52, stage/render/render 26.14, render/graph/ms 24.01, stage/render/graph_submit 19.39, stage/main/update 14.12, stage/render/prepare_assets 11.90 |
| warm-up | 5.28 | 33.1 | render/thread/ms 69.23, render/busy/ms 67.90, main/busy/ms 62.19, pipelines/wait_ms 30.04, pipelines/compile_ms 29.83, pipelines/front_ms 28.46, stage/main/post_update 26.93, stage/render/render 23.07, render/graph/ms 20.76, stage/render/graph_submit 16.00, stage/main/pre_update 15.12, stage/render/prepare_assets 14.61 |
| warm-up | 5.33 | 49.2 | pipelines/compile_ms 1025, pipelines/front_ms 999, render/thread/ms 75.94, render/busy/ms 74.70, main/busy/ms 63.25, stage/render/prepare_assets 36.09, stage/main/post_update 27.96, pipelines/driver_ms 25.71, stage/main/update 22.15, stream/drive/ms 14.93, stage/render/prepare_write_phase_buffers 14.15, stage/render/render 10.98 |
| warm-up | 5.49 | 29.4 | pipelines/compile_ms 562, pipelines/front_ms 535, pipelines/wait_ms 56.36, render/thread/ms 46.95, render/busy/ms 45.40, main/busy/ms 30.91, pipelines/driver_ms 27.25, stage/render/prepare_assets 24.04, stage/main/post_update 14.94, stage/render/render 14.67, render/graph/ms 12.58, stage/main/update 10.87 |
| warm-up | 5.54 | 29.6 | render/thread/ms 39.51, render/busy/ms 38.35, pipelines/compile_ms 28.63, pipelines/front_ms 26.80, main/busy/ms 24.79, stage/render/prepare_assets 21.89, stage/main/post_update 12.34, stage/render/render 10.71, render/graph/ms 8.89, stage/main/update 8.03, encode//sum 7.88, stage/render/graph_submit 6.29 |
| warm-up | 6.03 | 31.8 | render/thread/ms 39.86, render/busy/ms 38.43, main/busy/ms 28.98, stage/render/prepare_assets 16.74, stage/main/update 12.03, stage/main/post_update 10.21, stage/render/render 8.83, render/graph/ms 7.02, encode//sum 6.83, stage/render/graph_submit 4.90, stream/drive/ms 4.13, stage/render/specialize 3.41 |
| warm-up | 6.22 | 29.6 | main/busy/ms 39.59, render/thread/ms 35.41, render/busy/ms 34.12, stage/main/update 20.64, stage/main/post_update 12.07, stage/render/render 9.75, render/graph/ms 8.09, encode//sum 6.97, stage/render/prepare_assets 5.73, stream/drive/ms 5.37, stage/render/graph_submit 5.32, stage/render/specialize 4.42 |
| warm-up | 6.25 | 31.6 | render/thread/ms 46.90, render/busy/ms 45.47, main/busy/ms 42.65, stage/main/update 19.89, stage/main/post_update 14.94, stage/render/prepare_assets 13.32, stage/render/render 11.60, encode//sum 9.62, render/graph/ms 9.60, stage/render/graph_submit 6.15, stream/drive/ms 5.63, stage/render/specialize 4.43 |
| warm-up | 6.56 | 45.9 | render/thread/ms 50.86, render/busy/ms 49.56, main/busy/ms 36.73, stage/render/prepare_assets 30.84, stage/main/update 23.49, stream/drive/ms 16.68, stage/render/render 10.95, stage/main/post_update 9.91, render/graph/ms 9.06, encode//sum 8.23, stage/render/graph_submit 6.84, encode/submit 4.33 |
| warm-up | 6.78 | 36.7 | pipelines/compile_ms 133, pipelines/front_ms 130, main/busy/ms 42.06, render/thread/ms 27.79, render/busy/ms 26.79, stage/main/update 24.02, stream/drive/ms 13.38, stage/render/prepare_assets 11.12, stage/main/post_update 10.99, stage/render/render 9.44, render/graph/ms 7.99, encode//sum 7.07 |
| warm-up | 6.84 | 33.3 | render/thread/ms 45.08, render/busy/ms 43.87, main/busy/ms 21.43, stage/main/post_update 10.81, stage/render/specialize 10.72, stage/render/render 10.33, render/graph/ms 8.56, stage/render/queue 7.28, stage/main/update 7.13, stage/render/prepare_assets 7.07, encode//sum 6.65, stage/render/graph_submit 6.37 |
| warm-up | 8.09 | 31.5 | pipelines/compile_ms 135, pipelines/front_ms 131, render/thread/ms 36.46, render/busy/ms 35.37, main/busy/ms 17.47, stage/render/render 10.92, stage/main/post_update 10.43, stage/render/specialize 9.60, render/graph/ms 9.41, encode//sum 7.47, stage/render/graph_submit 7.10, stage/render/queue 6.14 |
| measure | 0.41 | 31.4 | render/thread/ms 48.17, render/busy/ms 47.13, stage/render/render 18.86, main/busy/ms 18.42, encode//sum 17.66, render/graph/ms 17.04, stage/render/graph_submit 12.52, stage/main/post_update 11.22, encode/pass:shadow_point_light/sum 10.97, stage/render/specialize 9.54, encode/submit 7.93, stage/render/queue 6.84 |
| measure | 1.69 | 31.2 | render/thread/ms 34.68, render/busy/ms 33.60, main/busy/ms 17.80, stage/main/post_update 10.76, stage/render/render 10.10, stage/render/specialize 9.28, render/graph/ms 8.59, encode//sum 6.83, stage/render/graph_submit 6.30, stage/render/queue 5.41, stage/main/update 4.82, stage/render/prepare_meshes 4.16 |
| measure | 3.02 | 29.7 | render/thread/ms 34.54, render/busy/ms 33.53, main/busy/ms 18.69, stage/main/post_update 12.13, stage/render/render 10.64, stage/render/specialize 9.32, render/graph/ms 9.19, encode//sum 7.83, stage/render/graph_submit 7.10, stage/render/queue 6.39, stage/main/update 4.26, encode/submit 4.12 |
| measure | 5.73 | 29.4 | render/thread/ms 35.34, render/busy/ms 34.35, main/busy/ms 18.18, stage/main/post_update 11.84, stage/render/render 10.17, stage/render/specialize 9.86, render/graph/ms 8.78, encode//sum 6.84, stage/render/queue 6.63, stage/render/graph_submit 6.46, stage/main/update 3.90, encode/submit 3.57 |
| measure | 8.30 | 30.7 | render/thread/ms 33.18, render/busy/ms 32.13, main/busy/ms 18.67, stage/main/post_update 12.20, stage/render/render 9.75, stage/render/specialize 9.14, encode//sum 8.39, render/graph/ms 8.27, stage/render/graph_submit 6.31, stage/render/queue 6.31, stage/main/update 4.15, encode/submit 3.34 |
| measure | 10.86 | 32.5 | render/thread/ms 39.15, render/busy/ms 37.99, main/busy/ms 20.13, stage/main/post_update 12.72, stage/render/specialize 11.76, stage/render/render 11.20, render/graph/ms 9.49, encode//sum 8.80, stage/render/graph_submit 7.29, stage/render/queue 6.36, encode/submit 4.29, stage/main/update 4.16 |
| measure | 15.73 | 31.2 | render/thread/ms 36.83, render/busy/ms 35.71, main/busy/ms 18.54, stage/main/post_update 11.47, stage/render/render 10.80, stage/render/specialize 10.16, render/graph/ms 9.25, encode//sum 7.10, stage/render/graph_submit 7.10, stage/render/queue 6.98, encode/submit 4.11, stage/main/update 4.09 |
| measure | 18.17 | 33.7 | main/busy/ms 38.65, stage/main/update 24.81, stream/drive/ms 20.57, render/thread/ms 18.27, render/busy/ms 17.02, stage/main/post_update 11.30, stage/render/render 10.53, render/graph/ms 8.86, encode//sum 8.00, stage/render/graph_submit 6.37, encode/submit 4.16, encode/pass:shadow_point_light/sum 2.57 |
| measure | 19.47 | 34.4 | render/thread/ms 44.70, render/busy/ms 43.64, main/busy/ms 21.54, stage/render/render 14.58, stage/main/post_update 13.48, render/graph/ms 13.08, encode//sum 12.92, stage/render/specialize 10.33, stage/render/graph_submit 9.41, encode/pass:shadow_point_light/sum 8.39, stage/render/queue 6.97, encode/submit 5.91 |
| measure | 23.04 | 38.1 | render/thread/ms 47.31, render/busy/ms 46.14, main/busy/ms 29.69, stage/render/render 15.06, encode//sum 14.92, stage/main/post_update 13.56, render/graph/ms 13.43, stage/render/specialize 13.40, stage/render/graph_submit 10.34, stage/render/queue 7.83, stage/main/update 6.67, encode/submit 6.16 |
| measure | 24.64 | 31.6 | render/thread/ms 41.03, render/busy/ms 40.00, main/busy/ms 19.49, stage/render/render 14.02, encode//sum 12.67, render/graph/ms 12.58, stage/main/post_update 11.91, stage/render/specialize 10.74, stage/render/graph_submit 9.41, encode/pass:shadow_point_light/sum 6.44, stage/render/queue 6.41, encode/submit 5.56 |
| measure | 27.29 | 36.7 | render/thread/ms 43.53, render/busy/ms 42.41, main/busy/ms 25.61, stage/render/render 14.31, render/graph/ms 12.72, stage/render/specialize 11.77, encode//sum 11.64, stage/main/post_update 11.62, stage/render/graph_submit 10.03, stage/render/queue 7.72, stage/main/update 7.36, encode/pass:shadow_point_light/sum 7.10 |
| measure | 28.75 | 30.1 | render/thread/ms 40.30, render/busy/ms 39.15, main/busy/ms 19.63, stage/render/render 14.04, stage/main/post_update 12.61, render/graph/ms 12.40, encode//sum 12.27, stage/render/specialize 9.77, stage/render/graph_submit 9.51, stage/render/queue 6.67, encode/pass:shadow_point_light/sum 5.89, encode/submit 5.82 |
| measure | 30.46 | 32.0 | render/thread/ms 39.21, render/busy/ms 38.10, main/busy/ms 21.90, stage/main/post_update 12.44, stage/render/specialize 11.14, stage/render/render 10.84, render/graph/ms 9.40, encode//sum 7.49, stage/render/graph_submit 7.41, stage/render/queue 6.68, stage/main/update 5.96, stage/render/prepare_meshes 3.89 |
| measure | 35.36 | 31.3 | render/thread/ms 38.70, render/busy/ms 37.62, main/busy/ms 21.70, stage/main/post_update 13.10, stage/render/render 10.45, stage/render/specialize 10.27, render/graph/ms 8.85, encode//sum 8.18, stage/render/graph_submit 6.79, stage/render/queue 6.67, stage/main/update 4.70, encode/finish_wall 3.72 |
| measure | 36.69 | 29.5 | render/thread/ms 35.72, render/busy/ms 34.56, main/busy/ms 19.55, stage/main/post_update 11.81, stage/render/render 10.52, stage/render/specialize 9.76, render/graph/ms 8.89, encode//sum 8.47, stage/render/graph_submit 6.92, stage/render/queue 6.26, stage/main/update 4.26, encode/submit 3.82 |
| measure | 40.80 | 31.8 | render/thread/ms 39.17, render/busy/ms 38.13, main/busy/ms 18.42, stage/render/render 11.15, stage/main/post_update 10.36, stage/render/specialize 10.23, render/graph/ms 9.72, encode//sum 7.87, stage/render/graph_submit 7.85, stage/render/queue 6.66, stage/main/update 4.45, encode/finish_wall 4.01 |
| measure | 43.21 | 30.1 | render/thread/ms 35.62, render/busy/ms 34.54, main/busy/ms 16.91, stage/render/specialize 9.99, stage/main/post_update 9.93, stage/render/render 9.73, render/graph/ms 8.28, encode//sum 7.47, stage/render/graph_submit 6.63, stage/render/queue 6.26, stage/main/update 3.87, encode/submit 3.58 |
| measure | 45.65 | 35.8 | render/thread/ms 39.84, render/busy/ms 38.79, main/busy/ms 19.28, stage/main/post_update 11.95, stage/render/specialize 10.93, stage/render/render 10.73, render/graph/ms 9.21, encode//sum 8.01, stage/render/graph_submit 7.35, stage/render/queue 6.41, stage/render/prepare_meshes 4.66, stage/main/update 4.39 |
| measure | 59.04 | 42.7 | render/thread/ms 57.04, render/busy/ms 55.67, main/busy/ms 30.27, stage/render/render 21.06, render/graph/ms 19.23, stage/main/post_update 16.09, stage/render/graph_submit 15.67, encode//sum 11.13, stage/render/specialize 10.38, stage/main/update 9.25, encode/finish_wall 8.07, encode/submit 7.65 |
| measure | 59.07 | 33.5 | render/thread/ms 58.37, render/busy/ms 57.29, main/busy/ms 24.74, stage/render/prepare_assets 16.59, stage/main/post_update 11.66, stage/render/specialize 10.52, stage/render/render 10.44, render/graph/ms 9.05, stage/render/prepare_meshes 8.05, encode//sum 7.67, stage/main/update 7.49, stage/render/graph_submit 7.00 |
| measure | 62.57 | 30.0 | render/thread/ms 37.80, render/busy/ms 36.80, main/busy/ms 18.48, stage/render/render 10.74, stage/render/specialize 10.45, stage/main/post_update 9.97, render/graph/ms 9.44, encode//sum 8.21, stage/render/graph_submit 7.31, stage/render/queue 6.41, stage/main/update 5.88, encode/submit 3.72 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_nomat\balmora_walk@2560x1440_arm=base-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 11%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **79%** (min 40%) · 2624 MHz (min 2610) · memory 8101 MHz · 80.0 W · 75 °C (max 80) · VRAM 6750 MiB · throttled 100% of samples

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
