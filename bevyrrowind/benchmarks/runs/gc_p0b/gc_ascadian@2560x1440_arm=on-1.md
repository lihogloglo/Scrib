# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1300 frames over 15.0 s · **median 11.59 ms (86 fps)** · mean 11.55 · p95 12.95 · p99 13.45 (1% low 74 fps) · worst 20.51 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 308 frames over 9.2 s, worst 2496.0 ms
- hitches (> 29.0 ms): **0** measured, 40 in warm-up
- timed GPU passes: 9.70 ms of a 11.55 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.59 ms, limit 11 ms
- pass worst frame 20.51 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.292 | 2.300 | 2.333 | 1.00 | 0.072 |
| dlss_super_resolution | 2.061 | 2.069 | 2.092 | 1.00 | 0.182 |
| early prepass | 1.330 | 1.335 | 1.350 | 1.00 | 0.111 |
| sun_visibility | 0.683 | 0.685 | 0.705 | 1.00 | 0.006 |
| atmosphere_luts | 0.354 | 0.355 | 0.362 | 1.00 | 0.003 |
| water_velocity | 0.328 | 0.329 | 0.333 | 1.00 | 0.002 |
| bloom | 0.322 | 0.324 | 0.339 | 1.00 | 0.062 |
| atmos_froxel_inject | 0.308 | 0.310 | 0.310 | 1.00 | 0.001 |
| water_sspr | 0.200 | 0.201 | 0.202 | 1.00 | 0.002 |
| gpu_instance_materialization | 0.195 | 0.196 | 0.247 | 1.00 | 0.007 |
| ssao | 0.188 | 0.189 | 0.197 | 1.00 | 0.007 |
| uniform_allocation | 0.141 | 0.141 | 0.185 | 1.00 | 0.022 |
| water_fft | 0.136 | 0.136 | 0.139 | 1.00 | 0.002 |
| upscaling | 0.131 | 0.131 | 0.135 | 1.00 | 0.004 |
| clouds_composite | 0.108 | 0.109 | 0.111 | 1.00 | 0.002 |
| bin_unpacking | 0.102 | 0.102 | 0.133 | 1.00 | 0.015 |
| early_mesh_preprocessing | 0.090 | 0.090 | 0.127 | 1.00 | 0.004 |
| tonemapping | 0.087 | 0.087 | 0.092 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.081 | 0.081 | 0.086 | 1.00 | 0.008 |
| clustering | 0.068 | 0.069 | 0.080 | 1.00 | 0.019 |
| render_sky | 0.063 | 0.063 | 0.065 | 1.00 | 0.002 |
| clouds_trace | 0.057 | 0.057 | 0.061 | 1.00 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.412 | 0.414 | 0.12 | 0.001 |
| clouds_resolve | 0.048 | 0.048 | 0.049 | 1.00 | 0.001 |
| point_shadow_atlas | 0.036 | 0.073 | 0.080 | 0.50 | 0.127 |
| range_unpacking | 0.036 | 0.036 | 0.044 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.030 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.221 | 0.232 | 0.12 | 0.004 |
| per_view_shadow_pass | 0.026 | 0.027 | 0.050 | 1.00 | 0.006 |
| water_snapshot | 0.024 | 0.024 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 1.00 | 0.002 |
| clouds_shadow | 0.021 | 0.021 | 0.023 | 1.00 | 0.003 |
| main_transparent_pass_3d | 0.014 | 0.014 | 0.027 | 1.00 | 0.232 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.014 | 1.00 | 0.003 |
| point_shadow_atlas/early_mesh_preprocessing | 0.007 | 0.015 | 0.016 | 0.50 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.008 | 1.00 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.007 | 0.007 | 0.50 | 0.001 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.007 | 0.007 | 0.50 | 0.001 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.007 | 0.007 | 0.50 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.003 | 0.00 | 0.009 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.380 | 11.780 | 19.38 | 13494.1 |
| render/busy/ms | 8.571 | 9.821 | 17.47 | 11142.4 |
| main/busy/ms | 8.068 | 9.257 | 10.25 | 10488.1 |
| stage/render/render | 5.385 | 6.354 | 13.88 | 7000.4 |
| render/graph/ms | 4.786 | 5.729 | 13.18 | 6221.4 |
| encode//sum | 4.708 | 6.258 | 13.24 | 6120.0 |
| stage/main/post_update | 4.684 | 5.637 | 7.12 | 6089.0 |
| stage/render/graph_submit | 3.459 | 4.335 | 11.79 | 4496.8 |
| encode/submit | 2.113 | 2.637 | 5.55 | 2746.7 |
| stage/main/update | 1.687 | 2.137 | 2.56 | 2193.3 |
| stage/render/prepare_views | 1.504 | 3.156 | 4.51 | 1955.4 |
| render/acquire/ms | 1.459 | 3.114 | 4.46 | 1896.5 |
| encode/finish_wall | 1.369 | 1.888 | 8.13 | 1779.5 |
| stage/render/camera_driver | 1.249 | 1.433 | 1.84 | 1623.6 |
| encode/pass:early prepass/sum | 0.879 | 1.183 | 7.26 | 1143.3 |
| encode/pass:shadow_directional_light/sum | 0.871 | 1.506 | 2.18 | 1132.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.828 | 1.176 | 1.67 | 1076.0 |
| stage/render/prepare_assets | 0.564 | 0.679 | 1.68 | 733.0 |
| bigspace/total/ms | 0.536 | 1.078 | 1.73 | 696.9 |
| stage/render/prepare_write_phase_buffers | 0.519 | 0.764 | 0.93 | 675.1 |
| stage/main/fixed_main_loop | 0.492 | 0.768 | 1.53 | 639.7 |
| stage/render/prepare_meshes | 0.477 | 0.642 | 0.99 | 619.9 |
| stage/render/prepare_resources | 0.423 | 0.490 | 0.98 | 549.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.421 | 0.601 | 1.40 | 547.8 |
| stage/main/pre_update | 0.379 | 0.448 | 0.93 | 492.2 |
| stage/main/first | 0.368 | 0.440 | 1.93 | 478.8 |
| render/present/ms | 0.350 | 0.422 | 1.60 | 455.2 |
| controller/feed/ms | 0.316 | 0.407 | 0.80 | 410.7 |
| stage/render/prepare_flush | 0.307 | 0.453 | 1.06 | 399.4 |
| stage/render/prepare_batch_phases | 0.305 | 0.418 | 0.62 | 396.8 |
| stage/main/last | 0.273 | 0.355 | 0.91 | 355.1 |
| stage/render/prepare_bind_groups | 0.267 | 0.419 | 0.65 | 347.5 |
| bigspace/low_precision/ms | 0.236 | 0.441 | 1.53 | 306.3 |
| stage/render/extract_commands | 0.189 | 0.219 | 0.32 | 245.2 |
| stage/render/create_views | 0.182 | 0.220 | 0.64 | 236.8 |
| anim/evaluate_cpu/ms | 0.172 | 0.216 | 0.63 | 224.2 |
| stream/drive/ms | 0.170 | 0.230 | 0.63 | 221.0 |
| anim/evaluate/ms | 0.167 | 0.334 | 1.86 | 217.1 |
| stage/render/specialize | 0.153 | 0.208 | 0.56 | 199.0 |
| stream/tick/ms | 0.142 | 0.194 | 0.58 | 184.1 |
| stage/render/queue | 0.134 | 0.177 | 0.34 | 174.1 |
| bigspace/high_precision/ms | 0.133 | 0.350 | 1.29 | 173.1 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.127 | 0.329 | 0.94 | 165.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.112 | 0.176 | 0.32 | 145.9 |
| controller/write/ms | 0.061 | 0.078 | 0.49 | 79.5 |
| controller/move/ms | 0.054 | 0.097 | 0.47 | 70.3 |
| encode/resolve_diagnostics/sum | 0.052 | 0.080 | 0.37 | 67.8 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.074 | 0.21 | 59.4 |
| physics/step/ms | 0.042 | 0.071 | 0.49 | 54.9 |
| stage/render/phase_sort | 0.040 | 0.059 | 0.13 | 51.8 |
| terrain/update/ms | 0.035 | 0.047 | 0.09 | 45.0 |
| stage/render/cleanup | 0.024 | 0.033 | 0.23 | 31.4 |
| encode/camera_driver_span_begin/sum | 0.021 | 0.036 | 0.08 | 27.5 |
| encode/camera_driver_span_end/sum | 0.019 | 0.033 | 0.18 | 25.0 |
| weather/map/ms | 0.015 | 0.020 | 0.03 | 19.5 |
| anim/apply/ms | 0.011 | 0.014 | 0.03 | 13.9 |
| controller/avoid/ms | 0.010 | 0.018 | 0.03 | 12.7 |
| weather/eval/ms | 0.009 | 0.011 | 0.50 | 11.6 |
| script/update/ms | 0.008 | 0.010 | 0.48 | 9.8 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 6.3 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 5.2 |
| far/select/ms | 0.004 | 0.004 | 0.04 | 4.7 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.01 | 2.4 |
| character/finish/ms | 0.002 | 0.002 | 0.38 | 2.0 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.02 | 1.7 |
| stream/promote/ms | 0.001 | 0.001 | 0.03 | 1.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.07 | 1.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 0.9 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.01 | 0.8 |
| terrain/select/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
| assets/cutout_decode/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.000 | 0.000 | 0.00 | 0.0 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| scene/load/chains/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| scene/load/plan/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| scene/load/wanted/ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| anim/a2/count | count | 3 | 3 | 3 |
| anim/a3/count | count | 447 | 447 | 447 |
| anim/characters/count | count | 450 | 450 | 450 |
| anim/evaluated/count | count | 26.50 | 27 | 26 |
| controller/movers/count | count | 28 | 28 | 28 |
| crowd/c0/count | count | 30 | 30 | 30 |
| crowd/c0/joints/count | count | 1249 | 1249 | 1249 |
| crowd/c0/meshes/count | count | 398 | 398 | 398 |
| crowd/c1/count | count | 116 | 116 | 116 |
| crowd/c1/joints/count | count | 5172 | 5172 | 5172 |
| crowd/c1/meshes/count | count | 1293 | 1293 | 1293 |
| crowd/c2/count | count | 304 | 304 | 304 |
| crowd/c2/joints/count | count | 5254 | 5254 | 5254 |
| crowd/c2/meshes/count | count | 1802 | 1802 | 1802 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 210 | 210 | 210 |
| crowd/joints_extracted/count | count | 764 | 1210 | 547 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.8 k | 43.8 k | 43.8 k |
| encode//max | ms | 0.61 | 8.08 | 0.53 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.08 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.18 | 0.01 |
| encode/finish_critical | ms | 1.02 | 8.08 | 1.14 |
| encode/pass:early prepass/max | ms | 0.88 | 7.26 | 1.03 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.84 | 1.67 | 1.14 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.32 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.40 | 1.40 | 0.37 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.24 | 0.94 | 0.18 |
| encode/pass:shadow_directional_light/max | ms | 0.82 | 2.18 | 0.79 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.37 | 0.04 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 10.38 | 17.47 | 10.28 |
| gpu/timed/ms | ms | 10.38 | 11.63 | 10.28 |
| groundcover/batches/count | count | 10 | 10 | 10 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1362 | 1362 | 1362 |
| groundcover/slots/capacity | count | 94.7 k | 94.7 k | 94.7 k |
| groundcover/slots/capacity_bytes | bytes | 6.06 M | 6.06 M | 6.06 M |
| groundcover/slots/live | count | 42.2 k | 42.2 k | 42.2 k |
| groundcover/tiles/capped | count | 0 | 0 | 0 |
| groundcover/tiles/casting | count | 5 | 5 | 5 |
| groundcover/tiles/resident | count | 63 | 63 | 63 |
| groundcover/tiles/scattered | count | 63 | 63 | 63 |
| groundcover/tiles/unplaced | count | 0 | 0 | 0 |
| groundcover/upload/bytes | bytes | 0 | 0 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2433 | 2433 | 2433 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 303.9 k | 303.9 k | 303.9 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.6 k | 12.6 k | 12.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 343.66 M | 343.66 M | 343.66 M |
| phases/alpha_mask3d/batch_sets | count | 15 | 15 | 15 |
| phases/alpha_mask3d/bins | count | 360 | 360 | 360 |
| phases/alpha_mask3d/instance_batches | count | 8 | 8 | 8 |
| phases/alpha_mask3d/instance_draws | count | 565 | 565 | 565 |
| phases/alpha_mask3d/instance_slots | count | 16.8 k | 16.8 k | 16.8 k |
| phases/alpha_mask3d/items | count | 565 | 565 | 565 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 15 | 15 | 15 |
| phases/alpha_mask3d_prepass/bins | count | 360 | 360 | 360 |
| phases/alpha_mask3d_prepass/instance_batches | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/instance_draws | count | 565 | 565 | 565 |
| phases/alpha_mask3d_prepass/instance_slots | count | 16.8 k | 16.8 k | 16.8 k |
| phases/alpha_mask3d_prepass/items | count | 565 | 565 | 565 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 27 | 27 |
| phases/opaque3d/bins | count | 1365 | 1365 | 1365 |
| phases/opaque3d/instance_batches | count | 6 | 6 | 6 |
| phases/opaque3d/instance_draws | count | 6757 | 6757 | 6757 |
| phases/opaque3d/instance_slots | count | 127.0 k | 127.0 k | 127.0 k |
| phases/opaque3d/items | count | 2847 | 2847 | 2847 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 22 | 22 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 24 | 24 | 24 |
| phases/opaque3d_prepass/bins | count | 1365 | 1365 | 1365 |
| phases/opaque3d_prepass/instance_batches | count | 6 | 6 | 6 |
| phases/opaque3d_prepass/instance_draws | count | 6757 | 6757 | 6757 |
| phases/opaque3d_prepass/instance_slots | count | 127.0 k | 127.0 k | 127.0 k |
| phases/opaque3d_prepass/items | count | 2847 | 2847 | 2847 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 18 | 18 | 18 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 50 | 115 | 12 |
| phases/shadow/bins | count | 2189 | 2189 | 2171 |
| phases/shadow/instance_batches | count | 7 | 21 | 7 |
| phases/shadow/instance_draws | count | 7315 | 21.9 k | 7315 |
| phases/shadow/instance_slots | count | 50.1 k | 150.3 k | 50.1 k |
| phases/shadow/items | count | 5361 | 5368 | 5350 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 51 | 51 | 49 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2374 | 2374 | 2374 |
| physics/items | count | 79 | 79 | 79 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 56 | 56 | 56 |
| pipelines/total | count | 348 | 348 | 348 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 7 | 149 | 5 |
| preprocess/bind_groups_created | count | 2 | 142 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 137 | 137 | 137 |
| process/cpu_usage | % | 25 | 41.04 | 19.71 |
| process/mem_usage | GiB | 3.15 | 3.20 | 3.20 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 1720 | 1720 | 1720 |
| scene/batch/masked/live | count | 1720 | 1720 | 1720 |
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
| scene/batch/opaque/high_water | count | 19.7 k | 19.7 k | 19.7 k |
| scene/batch/opaque/live | count | 19.7 k | 19.7 k | 19.7 k |
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
| scene/cells/resident | count | 118 | 118 | 118 |
| scene/chains/high_water | count | 1663 | 1663 | 1663 |
| scene/chains/live | count | 1663 | 1663 | 1663 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/scene_bump | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/scene_live | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 0 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 4 | 4 | 4 |
| shadow_atlas/live_occ_pct | % | 0.59 | 0.59 | 0.59 |
| shadow_atlas/live_tiles | count | 6 | 6 | 6 |
| shadow_atlas/mover_draws | count | 1 | 4 | 0 |
| shadow_atlas/movers | count | 100 | 100 | 85 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 1 | 4 | 0 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 4 | 4 | 4 |
| shadow_atlas/render/live_direct | count | 1 | 4 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 6 | 6 | 6 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 5.86 | 5.86 | 5.86 |
| shadow_atlas/static_tiles | count | 24 | 24 | 24 |
| shadow_atlas/views | count | 6 | 6 | 6 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 18 | 26 | 13 |
| skin/blocks_skipped/count | count | 192 | 200 | 197 |
| skin/blocks_walked/count | count | 18 | 27 | 13 |
| skin/joint_bytes_written/bytes | bytes | 58.9 k | 79.9 k | 63.0 k |
| skin/joints_written/count | count | 920 | 1248 | 984 |
| skin/upload_writes/count | count | 8 | 12 | 8 |
| started_load_count |  | 5837 | 5838 | 5838 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 247 | 247 | 247 |
| statics/materials_bound | count | 247 | 247 | 247 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.05 | 0.05 | 0.05 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 221 | 221 | 221 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 17.40 M | 17.40 M | 17.40 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 899 | 899 | 899 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.10 | 40.64 | 28.18 |
| system/mem_usage | % | 51.39 | 51.55 | 51.55 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2953 | 2953 | 2953 |
| textures/images | count | 2502 | 2502 | 2502 |
| views/all/batch_sets | count | 131 | 196 | 93 |
| views/all/bins | count | 5624 | 5639 | 5621 |
| views/all/count | count | 10 | 10 | 10 |
| views/all/instance_batches | count | 35 | 49 | 35 |
| views/all/instance_draws | count | 22.0 k | 36.6 k | 22.0 k |
| views/all/instance_slots | count | 337.7 k | 438.0 k | 337.7 k |
| views/all/items | count | 12.2 k | 12.2 k | 12.2 k |
| views/all/pipelines | count | 105 | 105 | 103 |
| views/all/pipelines_distinct | count | 74 | 74 | 74 |
| views/all/work_items | count | 345.1 k | 450.4 k | 345.1 k |
| views/cascade/batch_sets | count | 12 | 54 | 12 |
| views/cascade/bins | count | 2037 | 2037 | 2037 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 7 | 21 | 7 |
| views/cascade/instance_draws | count | 7315 | 21.9 k | 7315 |
| views/cascade/instance_slots | count | 50.1 k | 150.3 k | 50.1 k |
| views/cascade/items | count | 5209 | 5216 | 5216 |
| views/cascade/pipelines | count | 40 | 40 | 40 |
| views/cascade/work_items | count | 50.3 k | 155.5 k | 50.3 k |
| views/lamp_face/batch_sets | count | 29 | 91 | 0 |
| views/lamp_face/bins | count | 152 | 152 | 134 |
| views/lamp_face/count | count | 6 | 6 | 6 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 152 | 152 | 134 |
| views/lamp_face/pipelines | count | 11 | 11 | 9 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 29 | 91 | 0 |
| views/main/batch_sets | count | 81 | 81 | 81 |
| views/main/bins | count | 3450 | 3450 | 3450 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 28 | 28 | 28 |
| views/main/instance_draws | count | 14.6 k | 14.6 k | 14.6 k |
| views/main/instance_slots | count | 287.6 k | 287.6 k | 287.6 k |
| views/main/items | count | 6824 | 6824 | 6824 |
| views/main/pipelines | count | 54 | 54 | 54 |
| views/main/work_items | count | 294.8 k | 294.8 k | 294.8 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 18.7 k | 18.7 k | 18.7 k |
| world/point_lights | count | 355 | 355 | 355 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 35.94 | 106 | 46.7 k |
| anim/events/count | count | 0.04 | 2 | 58 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.77 | 8 | 3600 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| encode//count | count | 28.25 | 30 | 36.7 k |
| encode/bind_walk/init_textures | count | 9236 | 13.0 k | 12.01 M |
| encode/bind_walk/init_textures_skipped | count | 62.8 k | 73.7 k | 81.65 M |
| encode/bind_walk/inits | count | 737 | 839 | 957.6 k |
| encode/bind_walk/inits_skipped | count | 173 | 187 | 225.4 k |
| encode/bind_walk/merge_textures | count | 9174 | 12.9 k | 11.93 M |
| encode/bind_walk/merge_textures_skipped | count | 19.1 k | 30.0 k | 24.78 M |
| encode/bind_walk/merges | count | 140 | 161 | 181.5 k |
| encode/bind_walk/merges_skipped | count | 159 | 181 | 207.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1300 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1300 |
| encode/pass:early prepass/count | count | 1 | 1 | 1300 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1300 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1300 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1300 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 650 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1300 |
| encode/queue/command_buffers | count | 38.75 | 41 | 50.4 k |
| encode/queue/submits | count | 2 | 2 | 2600 |
| encode/queue/write_buffer/bytes | bytes | 2.75 M | 3.49 M | 3.57 G |
| encode/queue/write_buffer/calls | count | 272 | 344 | 353.8 k |
| encode/queue/write_buffer_with/bytes | bytes | 43.3 k | 43.3 k | 56.24 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 11.7 k |
| encode/queue/write_texture/bytes | bytes | 20.3 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1300 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.69 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| precache/hit | count | 0 | 0 | 0 |
| precache/missed | count | 0 | 0 | 0 |
| precache/requested | count | 0 | 0 | 0 |
| scene/load/deferred | count | 0 | 0 | 0 |
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
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.0 ms; slowest 40, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2496.0 | main/busy/ms 2465, stage/main/post_update 2414, stage/main/update 36.75, stage/main/pre_update 9.72, stage/main/last 1.18, surface/advance_cpu_ms 0.89, stream/drive/ms 0.56, stage/main/first 0.52, stage/main/fixed_main_loop 0.43, surface/cell_upload_ms 0.30, stream/tick/ms 0.22, bigspace/total/ms 0.08 |
| warm-up | 3.26 | 767.5 | main/busy/ms 2510, stage/main/post_update 2446, render/thread/ms 708, render/busy/ms 544, stage/render/prepare_assets 506, stage/render/prepare_views 160, render/acquire/ms 160, stage/main/update 43.45, stage/render/render 24.72, render/graph/ms 19.65, stage/render/camera_driver 17.99, stage/main/pre_update 11.79 |
| warm-up | 3.37 | 104.0 | render/thread/ms 810, render/busy/ms 577, stage/render/prepare_assets 516, stage/render/prepare_views 161, render/acquire/ms 160, stage/render/render 105, main/busy/ms 84.74, pipelines/compile_ms 81.43, render/present/ms 73.12, pipelines/driver_ms 54.48, stage/main/post_update 51.48, render/graph/ms 29.79 |
| warm-up | 3.43 | 66.9 | pipelines/compile_ms 474, pipelines/front_ms 330, render/thread/ms 167, pipelines/wait_ms 162, pipelines/driver_ms 143, stage/render/render 105, render/busy/ms 92.74, render/present/ms 73.48, main/busy/ms 60.80, stage/render/prepare_assets 34.76, stage/main/post_update 32.88, terrain/update/ms 31.53 |
| warm-up | 3.53 | 98.0 | pipelines/wait_ms 2120, pipelines/compile_ms 1156, pipelines/front_ms 1017, render/thread/ms 159, render/busy/ms 153, pipelines/driver_ms 138, main/busy/ms 95.58, stage/main/update 52.25, stage/render/prepare_assets 40.62, stage/render/render 33.18, render/graph/ms 26.19, stage/main/post_update 25.45 |
| warm-up | 3.65 | 119.2 | pipelines/wait_ms 5635, pipelines/compile_ms 2794, pipelines/front_ms 2649, render/thread/ms 184, render/busy/ms 182, pipelines/driver_ms 145, main/busy/ms 110, stage/render/prepare_assets 86.97, stage/main/update 58.23, stage/main/post_update 32.21, stage/render/prepare_flush 23.33, stage/render/render 21.89 |
| warm-up | 3.69 | 34.1 | pipelines/wait_ms 3760, pipelines/compile_ms 2204, pipelines/front_ms 2097, render/thread/ms 121, render/busy/ms 119, pipelines/driver_ms 107, stage/render/prepare_assets 91.46, main/busy/ms 66.80, stage/main/post_update 25.98, stage/main/first 20.65, stage/render/render 19.56, render/graph/ms 17.23 |
| warm-up | 3.72 | 31.9 | pipelines/compile_ms 211, pipelines/front_ms 196, pipelines/wait_ms 86.61, render/thread/ms 60.92, render/busy/ms 59.38, main/busy/ms 47.78, stage/render/prepare_assets 40.09, stage/main/first 18.09, stage/main/post_update 15.90, pipelines/driver_ms 14.38, stage/render/render 10.54, terrain/update/ms 9.93 |
| warm-up | 3.76 | 46.4 | pipelines/compile_ms 304, pipelines/front_ms 293, render/thread/ms 74.63, render/busy/ms 72.71, main/busy/ms 54.29, pipelines/wait_ms 41.09, stage/main/update 27.67, stage/render/prepare_resources 24.37, stage/main/post_update 22.46, stage/render/prepare_assets 21.64, encode//sum 19.82, stage/render/render 17.34 |
| warm-up | 3.86 | 93.7 | pipelines/compile_ms 1418, pipelines/front_ms 1329, pipelines/wait_ms 261, render/thread/ms 137, render/busy/ms 134, main/busy/ms 119, pipelines/driver_ms 89.12, stage/main/post_update 74.94, stage/render/prepare_resources 44.22, stage/main/update 39.83, encode//sum 29.06, stage/render/render 24.79 |
| warm-up | 3.90 | 40.7 | pipelines/compile_ms 1909, pipelines/front_ms 1798, pipelines/wait_ms 264, render/thread/ms 130, render/busy/ms 128, pipelines/driver_ms 111, main/busy/ms 102, stage/main/post_update 70.22, stage/render/prepare_assets 36.97, stage/render/prepare_resources 23.69, stage/render/render 23.21, stage/main/update 20.96 |
| warm-up | 4.01 | 113.2 | pipelines/compile_ms 2072, pipelines/front_ms 1967, pipelines/wait_ms 156, render/thread/ms 149, render/busy/ms 147, pipelines/driver_ms 105, stage/render/prepare_assets 100, main/busy/ms 33.93, stage/render/render 31.01, render/graph/ms 28.25, stage/render/graph_submit 15.56, stage/main/post_update 12.05 |
| warm-up | 4.07 | 30.7 | pipelines/compile_ms 359, pipelines/front_ms 345, render/thread/ms 57.60, render/busy/ms 56.22, stage/render/prepare_assets 35.22, main/busy/ms 30.18, stage/main/update 14.14, pipelines/driver_ms 14.10, stage/render/render 9.34, stage/main/post_update 8.11, render/graph/ms 7.45, encode//sum 6.18 |
| warm-up | 4.20 | 36.4 | pipelines/compile_ms 1032, pipelines/front_ms 999, render/thread/ms 59.10, render/busy/ms 56.35, main/busy/ms 49.43, pipelines/driver_ms 33.33, stage/render/prepare_assets 22.49, stage/main/update 19.26, stage/render/render 17.96, stage/main/post_update 16.92, render/graph/ms 15.91, stage/render/graph_submit 11.68 |
| warm-up | 4.31 | 60.2 | pipelines/compile_ms 869, pipelines/front_ms 842, render/thread/ms 81.37, render/busy/ms 79.61, main/busy/ms 60.74, stage/main/post_update 27.82, pipelines/driver_ms 27.71, stage/main/update 21.78, stage/render/prepare_assets 18.72, stage/render/prepare_resources 15.50, stage/render/render 14.29, render/graph/ms 12.07 |
| warm-up | 4.40 | 88.8 | pipelines/compile_ms 725, pipelines/front_ms 704, render/thread/ms 142, render/busy/ms 138, main/busy/ms 115, stage/render/prepare_assets 52.05, stage/main/update 49.27, stage/main/post_update 47.08, stage/render/prepare_resources 33.72, pipelines/driver_ms 21.03, stage/render/prepare_write_phase_buffers 13.27, stage/render/render 12.98 |
| warm-up | 4.49 | 93.6 | pipelines/compile_ms 1829, pipelines/front_ms 1777, main/busy/ms 177, render/thread/ms 177, render/busy/ms 172, stage/main/update 102, stage/render/prepare_assets 81.54, pipelines/driver_ms 52.31, stage/main/post_update 39.76, stage/render/prepare_resources 26.98, character/finish/ms 23.75, stage/render/render 22.32 |
| warm-up | 4.65 | 159.8 | pipelines/compile_ms 3402, pipelines/front_ms 3310, pipelines/wait_ms 303, render/thread/ms 242, render/busy/ms 239, stage/render/prepare_assets 168, main/busy/ms 120, pipelines/driver_ms 92.28, stage/main/update 70.22, stage/render/render 25.86, render/graph/ms 22.23, stage/main/post_update 21.10 |
| warm-up | 4.70 | 44.8 | pipelines/compile_ms 3132, pipelines/front_ms 3040, pipelines/wait_ms 394, render/thread/ms 194, render/busy/ms 192, stage/render/prepare_assets 149, pipelines/driver_ms 91.79, main/busy/ms 58.98, stage/main/update 25.98, stage/render/render 16.19, stage/main/post_update 14.36, render/graph/ms 13.24 |
| warm-up | 4.74 | 43.2 | pipelines/compile_ms 974, pipelines/front_ms 939, pipelines/wait_ms 116, render/thread/ms 84.66, render/busy/ms 82.83, main/busy/ms 64.84, stage/render/prepare_assets 49.70, stage/main/update 36.64, pipelines/driver_ms 35.77, stage/render/render 14.61, render/graph/ms 12.24, stage/main/post_update 12.12 |
| warm-up | 4.78 | 36.2 | pipelines/compile_ms 135, pipelines/front_ms 131, render/thread/ms 76.29, render/busy/ms 74.63, main/busy/ms 55.04, stage/render/prepare_assets 40.45, stage/main/update 30.34, pipelines/wait_ms 23.60, stage/render/render 15.18, render/graph/ms 12.94, stage/main/post_update 11.78, stage/render/graph_submit 8.94 |
| warm-up | 4.81 | 36.0 | pipelines/compile_ms 287, pipelines/front_ms 278, pipelines/wait_ms 108, render/thread/ms 69.30, render/busy/ms 67.98, main/busy/ms 59.78, stage/main/update 31.54, stage/render/prepare_assets 21.19, stage/main/post_update 14.05, stage/render/render 13.87, render/graph/ms 11.98, pipelines/driver_ms 9.11 |
| warm-up | 4.85 | 37.4 | pipelines/compile_ms 854, pipelines/front_ms 831, pipelines/wait_ms 118, render/thread/ms 67.65, render/busy/ms 66.16, main/busy/ms 58.91, stage/main/update 30.21, pipelines/driver_ms 23.09, stage/main/post_update 16.41, stage/render/prepare_assets 16.29, stage/render/render 16.28, render/graph/ms 14.26 |
| warm-up | 4.91 | 31.2 | pipelines/compile_ms 978, pipelines/front_ms 947, main/busy/ms 58.21, render/thread/ms 51.29, render/busy/ms 49.76, stage/main/update 35.22, pipelines/wait_ms 30.93, pipelines/driver_ms 30.42, stage/render/render 16.89, render/graph/ms 14.48, stage/main/post_update 14.23, character/finish/ms 10.02 |
| warm-up | 5.16 | 40.7 | pipelines/compile_ms 542, pipelines/front_ms 528, main/busy/ms 55.64, render/thread/ms 43.97, render/busy/ms 42.62, stage/main/post_update 27.09, stage/main/update 22.14, stage/render/render 14.61, pipelines/driver_ms 14.51, render/graph/ms 12.60, stage/render/prepare_resources 10.59, stage/render/graph_submit 9.29 |
| warm-up | 5.22 | 60.0 | pipelines/compile_ms 1312, pipelines/front_ms 1274, render/thread/ms 84.60, render/busy/ms 82.89, main/busy/ms 79.19, stage/main/update 38.17, pipelines/driver_ms 37.69, stage/main/post_update 32.09, stage/render/render 28.82, render/graph/ms 26.46, stage/render/prepare_assets 18.10, stage/render/camera_driver 17.80 |
| warm-up | 5.27 | 53.4 | pipelines/compile_ms 1066, pipelines/front_ms 1036, render/thread/ms 101, render/busy/ms 98.98, main/busy/ms 80.09, stage/render/prepare_assets 41.36, stage/main/update 36.34, stage/main/post_update 32.27, pipelines/driver_ms 30.06, stage/render/render 29.25, render/graph/ms 27.00, stage/render/camera_driver 18.01 |
| warm-up | 5.34 | 68.4 | pipelines/compile_ms 1455, pipelines/front_ms 1416, render/thread/ms 102, render/busy/ms 100, main/busy/ms 76.99, stage/main/update 45.33, pipelines/driver_ms 38.68, stage/render/prepare_assets 38.13, stage/render/prepare_flush 25.66, stage/main/post_update 18.78, pipelines/wait_ms 15.45, stage/render/render 14.25 |
| warm-up | 5.38 | 45.9 | pipelines/compile_ms 1417, pipelines/front_ms 1380, render/thread/ms 103, render/busy/ms 102, main/busy/ms 87.12, stage/main/update 47.57, pipelines/driver_ms 37.06, stage/render/prepare_flush 26.16, stage/render/prepare_assets 21.53, stage/main/post_update 20.93, stage/render/render 18.74, render/graph/ms 16.19 |
| warm-up | 5.44 | 52.4 | pipelines/compile_ms 1382, pipelines/front_ms 1347, render/thread/ms 94.02, render/busy/ms 92.31, main/busy/ms 61.21, stage/render/prepare_assets 43.85, pipelines/driver_ms 34.99, stage/main/update 22.55, stage/main/post_update 19.41, stage/render/render 17.79, render/graph/ms 15.12, pipelines/wait_ms 10.99 |
| warm-up | 5.47 | 39.4 | pipelines/compile_ms 1324, pipelines/front_ms 1292, render/thread/ms 88.17, render/busy/ms 86.92, stage/render/prepare_assets 60.67, main/busy/ms 41.84, pipelines/driver_ms 32.48, stage/main/update 16.33, stage/render/render 13.90, stage/main/post_update 12.29, render/graph/ms 11.73, pipelines/wait_ms 10.66 |
| warm-up | 5.53 | 31.7 | pipelines/compile_ms 527, pipelines/front_ms 513, pipelines/wait_ms 57.12, render/thread/ms 51.24, render/busy/ms 39.88, main/busy/ms 37.95, stage/main/update 16.89, stage/render/render 16.27, pipelines/driver_ms 14.09, render/graph/ms 13.58, stage/render/prepare_assets 13.08, stage/main/post_update 12.17 |
| warm-up | 5.74 | 33.5 | pipelines/compile_ms 176, pipelines/front_ms 171, render/thread/ms 54.63, render/busy/ms 52.78, main/busy/ms 37.54, stage/render/prepare_assets 20.80, stage/render/render 20.18, render/graph/ms 17.43, stage/main/update 14.82, stage/main/post_update 13.57, stage/render/graph_submit 12.67, encode//sum 11.06 |
| warm-up | 5.77 | 29.6 | pipelines/compile_ms 354, pipelines/front_ms 347, render/thread/ms 58.62, render/busy/ms 57.38, main/busy/ms 39.53, stage/render/prepare_assets 32.96, stage/main/update 17.54, stage/render/render 13.51, stage/main/post_update 13.36, render/graph/ms 11.66, encode//sum 9.51, stage/render/graph_submit 8.27 |
| warm-up | 5.85 | 31.2 | render/thread/ms 46.51, render/busy/ms 44.51, main/busy/ms 44.10, pipelines/compile_ms 22.22, pipelines/front_ms 20.64, stage/main/update 20.00, stage/render/render 19.77, render/graph/ms 17.02, stage/main/post_update 16.80, stage/render/graph_submit 12.50, encode//sum 11.12, encode/submit 7.76 |
| warm-up | 5.88 | 33.6 | pipelines/compile_ms 61.49, render/thread/ms 59.27, pipelines/front_ms 58.37, render/busy/ms 57.20, main/busy/ms 54.41, stage/main/update 26.67, stage/render/prepare_assets 19.59, stage/render/render 16.83, stage/main/post_update 15.93, render/graph/ms 14.31, encode//sum 10.93, stage/render/graph_submit 10.25 |
| warm-up | 5.92 | 37.7 | pipelines/compile_ms 73.81, pipelines/front_ms 70.01, render/thread/ms 66.76, render/busy/ms 65.58, main/busy/ms 53.86, stage/render/prepare_assets 36.31, stage/main/update 32.44, stage/render/render 10.55, stage/main/post_update 9.82, render/graph/ms 8.99, encode//sum 8.28, stage/render/prepare_meshes 7.10 |
| warm-up | 6.07 | 35.4 | pipelines/compile_ms 181, pipelines/front_ms 174, pipelines/wait_ms 55.28, render/thread/ms 51.30, render/busy/ms 49.77, main/busy/ms 45.05, stage/render/render 20.55, stage/main/update 19.39, render/graph/ms 17.98, stage/main/post_update 17.76, encode//sum 14.75, stage/render/prepare_assets 13.31 |
| warm-up | 6.14 | 41.4 | pipelines/compile_ms 450, pipelines/front_ms 433, render/thread/ms 60.95, render/busy/ms 59.79, main/busy/ms 53.41, stage/render/prepare_assets 31.64, stage/main/update 27.92, pipelines/driver_ms 17.20, stage/main/post_update 14.37, stage/render/render 11.52, render/graph/ms 9.84, encode//sum 8.24 |
| warm-up | 6.58 | 32.3 | render/thread/ms 46.91, render/busy/ms 45.55, main/busy/ms 42.68, pipelines/compile_ms 23.03, pipelines/front_ms 22.05, stage/main/update 18.85, stage/main/post_update 17.94, stage/render/prepare_assets 14.55, stage/render/render 12.79, render/graph/ms 11.15, encode//sum 10.76, stage/render/graph_submit 7.45 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --resolution 2560x1440 --bench-out target/perf\gc_p0b\gc_ascadian@2560x1440_arm=on-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (285 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 97%) · 2625 MHz (min 2625) · memory 8101 MHz · 89.4 W · 72 °C (max 75) · VRAM 6150 MiB · throttled 100% of samples

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
