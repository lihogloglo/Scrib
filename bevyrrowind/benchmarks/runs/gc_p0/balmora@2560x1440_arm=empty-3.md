# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); groundcover.density_scale=0 (command line: --set); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1200 frames over 15.0 s · **median 12.58 ms (79 fps)** · mean 12.50 · p95 14.06 · p99 14.56 (1% low 69 fps) · worst 16.92 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 311 frames over 9.1 s, worst 2547.9 ms
- hitches (> 31.5 ms): **0** measured, 21 in warm-up
- timed GPU passes: 11.01 ms of a 12.50 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.58 ms, limit 11 ms
- pass worst frame 16.92 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.194 | 3.194 | 3.294 | 1.00 | 0.082 |
| dlss_super_resolution | 2.022 | 2.022 | 2.047 | 1.00 | 0.182 |
| sun_visibility | 0.781 | 0.781 | 0.808 | 1.00 | 0.006 |
| gpu_instance_materialization | 0.661 | 0.661 | 0.897 | 1.00 | 0.008 |
| early prepass | 0.629 | 0.629 | 0.638 | 1.00 | 0.123 |
| main_transmissive_pass_3d | 0.396 | 0.396 | 0.407 | 1.00 | 0.007 |
| atmosphere_luts | 0.366 | 0.366 | 0.438 | 1.00 | 0.003 |
| bloom | 0.314 | 0.314 | 0.331 | 1.00 | 0.058 |
| atmos_froxel_inject | 0.310 | 0.310 | 0.308 | 1.00 | 0.001 |
| water_velocity | 0.306 | 0.306 | 0.311 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.249 | 0.249 | 0.280 | 1.00 | 0.003 |
| water_sspr | 0.218 | 0.218 | 0.213 | 1.00 | 0.002 |
| ssao | 0.188 | 0.188 | 0.196 | 1.00 | 0.007 |
| clouds_trace | 0.152 | 0.152 | 0.170 | 1.00 | 0.001 |
| water_fft | 0.134 | 0.134 | 0.138 | 1.00 | 0.002 |
| uniform_allocation | 0.120 | 0.120 | 0.153 | 1.00 | 0.018 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| upscaling | 0.115 | 0.115 | 0.120 | 1.00 | 0.004 |
| bin_unpacking | 0.098 | 0.098 | 0.123 | 1.00 | 0.013 |
| range_unpacking | 0.086 | 0.086 | 0.096 | 1.00 | 0.003 |
| tonemapping | 0.085 | 0.085 | 0.089 | 1.00 | 0.003 |
| clustering | 0.063 | 0.063 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.062 | 0.062 | 0.062 | 1.00 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.412 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.220 | 0.232 | 0.12 | 0.003 |
| per_view_shadow_pass | 0.026 | 0.026 | 0.052 | 1.00 | 0.006 |
| water_snapshot | 0.023 | 0.023 | 0.024 | 1.00 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.024 | 1.00 | 0.180 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.001 |
| surface_exposure_far | 0.011 | 0.035 | 0.040 | 0.31 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| surface_exposure | 0.010 | 0.035 | 0.036 | 0.28 | 0.001 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.014 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.479 | 13.033 | 15.81 | 13774.3 |
| render/busy/ms | 7.392 | 8.514 | 15.27 | 8870.4 |
| main/busy/ms | 6.462 | 7.799 | 9.74 | 7754.5 |
| encode//sum | 4.893 | 6.295 | 12.57 | 5871.3 |
| stage/render/render | 4.871 | 5.692 | 8.26 | 5844.7 |
| pipelines/compile_ms | 4.383 | 13.167 | 122.93 | 5259.4 |
| render/graph/ms | 4.277 | 5.048 | 7.71 | 5132.4 |
| pipelines/front_ms | 4.263 | 12.801 | 120.39 | 5115.1 |
| stage/render/prepare_views | 3.782 | 5.491 | 6.88 | 4538.4 |
| render/acquire/ms | 3.730 | 5.435 | 6.82 | 4476.4 |
| stage/main/post_update | 3.575 | 4.592 | 6.50 | 4289.5 |
| stage/render/graph_submit | 3.079 | 3.805 | 6.56 | 3694.6 |
| encode/submit | 1.876 | 2.277 | 3.01 | 2251.2 |
| stage/main/update | 1.320 | 1.547 | 2.38 | 1584.1 |
| encode/finish_wall | 1.217 | 1.650 | 4.36 | 1460.5 |
| stage/render/camera_driver | 1.111 | 1.297 | 1.68 | 1333.6 |
| encode/pass:early prepass/sum | 0.775 | 1.057 | 3.56 | 930.0 |
| encode/pass:shadow_directional_light/sum | 0.756 | 1.345 | 2.00 | 907.0 |
| encode/pass:main_opaque_pass_3d/sum | 0.738 | 1.034 | 1.30 | 885.7 |
| stage/render/prepare_write_phase_buffers | 0.486 | 0.712 | 0.87 | 583.6 |
| stage/main/fixed_main_loop | 0.448 | 0.646 | 1.27 | 538.2 |
| stage/render/prepare_resources | 0.426 | 0.502 | 0.65 | 510.8 |
| stage/render/prepare_assets | 0.392 | 0.473 | 1.83 | 470.8 |
| stage/main/pre_update | 0.380 | 0.453 | 1.53 | 456.1 |
| render/present/ms | 0.356 | 0.423 | 0.72 | 427.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.333 | 0.468 | 0.62 | 399.1 |
| stage/main/first | 0.320 | 0.382 | 1.73 | 384.4 |
| bigspace/total/ms | 0.303 | 0.501 | 1.50 | 363.1 |
| stage/render/prepare_flush | 0.275 | 0.403 | 0.88 | 329.8 |
| stage/render/prepare_batch_phases | 0.262 | 0.356 | 0.56 | 314.3 |
| stage/render/prepare_bind_groups | 0.258 | 0.380 | 0.59 | 309.2 |
| stage/main/last | 0.242 | 0.331 | 0.79 | 290.4 |
| stage/render/prepare_meshes | 0.208 | 0.260 | 1.24 | 249.8 |
| stage/render/extract_commands | 0.191 | 0.223 | 0.33 | 229.3 |
| controller/feed/ms | 0.173 | 0.218 | 0.57 | 207.5 |
| stage/render/create_views | 0.159 | 0.198 | 0.33 | 191.2 |
| stream/drive/ms | 0.135 | 0.157 | 0.33 | 161.8 |
| pipelines/driver_ms | 0.120 | 0.652 | 4.69 | 144.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.118 | 0.172 | 0.27 | 142.1 |
| stream/tick/ms | 0.109 | 0.126 | 0.30 | 130.2 |
| stage/render/queue | 0.105 | 0.141 | 2.11 | 126.1 |
| bigspace/high_precision/ms | 0.102 | 0.185 | 0.93 | 121.9 |
| stage/render/specialize | 0.100 | 0.176 | 3.27 | 119.5 |
| controller/move/ms | 0.080 | 0.155 | 0.23 | 96.1 |
| bigspace/low_precision/ms | 0.078 | 0.107 | 1.06 | 93.7 |
| anim/evaluate/ms | 0.067 | 0.083 | 0.65 | 80.7 |
| anim/evaluate_cpu/ms | 0.059 | 0.070 | 0.12 | 70.9 |
| encode/resolve_diagnostics/sum | 0.057 | 0.085 | 0.36 | 68.9 |
| controller/write/ms | 0.053 | 0.067 | 0.13 | 63.1 |
| stage/render/prepare_collect_phase_buffers | 0.043 | 0.072 | 0.22 | 52.2 |
| terrain/update/ms | 0.035 | 0.046 | 0.35 | 42.0 |
| physics/step/ms | 0.033 | 0.051 | 0.18 | 39.4 |
| stage/render/cleanup | 0.025 | 0.033 | 0.21 | 30.3 |
| stage/render/phase_sort | 0.025 | 0.049 | 0.17 | 29.8 |
| encode/camera_driver_span_begin/sum | 0.024 | 0.038 | 0.12 | 29.1 |
| weather/map/ms | 0.023 | 0.032 | 0.11 | 28.2 |
| encode/camera_driver_span_end/sum | 0.021 | 0.035 | 0.11 | 25.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.04 | 13.0 |
| script/update/ms | 0.006 | 0.008 | 0.08 | 7.5 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 5.7 |
| controller/avoid/ms | 0.004 | 0.007 | 0.02 | 5.4 |
| anim/apply/ms | 0.004 | 0.006 | 0.04 | 5.1 |
| far/select/ms | 0.004 | 0.004 | 0.03 | 4.4 |
| script/ui/ms | 0.003 | 0.004 | 0.02 | 3.3 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.05 | 2.2 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| character/finish/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| stream/promote/ms | 0.001 | 0.001 | 0.02 | 1.2 |
| pipelines/wait_ms | 0.001 | 0.014 | 0.03 | 1.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.7 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.02 | 0.7 |
| terrain/select/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| stage/render/prepare | 0.000 | 0.003 | 0.03 | 0.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.4 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| assets/cutout_decode/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 115 | 181 | 102 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 38.0 k | 38.0 k | 38.0 k |
| encode//max | ms | 0.59 | 4.31 | 1.02 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.12 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.11 | 0.02 |
| encode/finish_critical | ms | 0.91 | 4.31 | 1.03 |
| encode/pass:early prepass/max | ms | 0.78 | 3.56 | 0.91 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.73 | 1.30 | 1.03 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.27 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.34 | 0.62 | 0.39 |
| encode/pass:shadow_directional_light/max | ms | 0.70 | 2.00 | 0.43 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.36 | 0.05 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 11.56 | 15.27 | 11.58 |
| gpu/timed/ms | ms | 11.56 | 13.14 | 11.58 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 818 | 818 | 818 |
| groundcover/slots/capacity | count | 658.0 k | 658.0 k | 658.0 k |
| groundcover/slots/capacity_bytes | bytes | 42.11 M | 42.11 M | 42.11 M |
| groundcover/slots/live | count | 0 | 0 | 0 |
| groundcover/tiles/capped | count | 0 | 0 | 0 |
| groundcover/tiles/casting | count | 6 | 6 | 6 |
| groundcover/tiles/resident | count | 63 | 63 | 63 |
| groundcover/tiles/scattered | count | 63 | 63 | 63 |
| groundcover/tiles/unplaced | count | 0 | 0 | 0 |
| groundcover/upload/bytes | bytes | 0 | 0 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1931 | 1931 | 1931 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 217.5 k | 217.5 k | 217.5 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 11.3 k | 11.3 k | 11.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 321.65 M | 321.65 M | 321.65 M |
| phases/alpha_mask3d/batch_sets | count | 19 | 19 | 19 |
| phases/alpha_mask3d/bins | count | 258 | 258 | 258 |
| phases/alpha_mask3d/instance_batches | count | 12 | 12 | 12 |
| phases/alpha_mask3d/instance_draws | count | 573 | 573 | 573 |
| phases/alpha_mask3d/instance_slots | count | 117.5 k | 117.5 k | 117.5 k |
| phases/alpha_mask3d/items | count | 329 | 329 | 329 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 8 | 8 | 8 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/alpha_mask3d_prepass/bins | count | 258 | 258 | 258 |
| phases/alpha_mask3d_prepass/instance_batches | count | 12 | 12 | 12 |
| phases/alpha_mask3d_prepass/instance_draws | count | 573 | 573 | 573 |
| phases/alpha_mask3d_prepass/instance_slots | count | 117.5 k | 117.5 k | 117.5 k |
| phases/alpha_mask3d_prepass/items | count | 329 | 329 | 329 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 31 | 31 | 31 |
| phases/opaque3d/bins | count | 883 | 883 | 883 |
| phases/opaque3d/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d/instance_draws | count | 5306 | 5306 | 5306 |
| phases/opaque3d/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d/items | count | 2792 | 2792 | 2792 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 22 | 22 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 27 | 27 | 27 |
| phases/opaque3d_prepass/bins | count | 883 | 883 | 883 |
| phases/opaque3d_prepass/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/instance_draws | count | 5306 | 5306 | 5306 |
| phases/opaque3d_prepass/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d_prepass/items | count | 2792 | 2792 | 2792 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 18 | 18 | 18 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 9 | 56 | 9 |
| phases/shadow/bins | count | 911 | 1524 | 911 |
| phases/shadow/instance_batches | count | 9 | 27 | 9 |
| phases/shadow/instance_draws | count | 5866 | 17.6 k | 5866 |
| phases/shadow/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| phases/shadow/items | count | 3217 | 8176 | 3217 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 33 | 40 | 33 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 346 | 346 | 346 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 33 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 24.52 | 40.38 | 16.83 |
| process/mem_usage | GiB | 3.01 | 3.06 | 3.06 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2386 | 2386 | 2386 |
| scene/batch/masked/live | count | 2386 | 2386 | 2386 |
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
| scene/batch/opaque/high_water | count | 16.8 k | 16.8 k | 16.8 k |
| scene/batch/opaque/live | count | 16.8 k | 16.8 k | 16.8 k |
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
| scene/cells/resident | count | 101 | 101 | 101 |
| scene/chains/high_water | count | 1639 | 1639 | 1639 |
| scene/chains/live | count | 1639 | 1639 | 1639 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 0 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 0 | 0 | 0 |
| shadow_atlas/live_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/live_tiles | count | 0 | 0 | 0 |
| shadow_atlas/mover_draws | count | 0 | 0 | 0 |
| shadow_atlas/movers | count | 0 | 0 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 0 | 0 | 0 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/live_direct | count | 0 | 0 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 0 | 0 | 0 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/static_tiles | count | 0 | 0 | 0 |
| shadow_atlas/views | count | 0 | 0 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 5 | 7 | 5 |
| skin/blocks_skipped/count | count | 176 | 178 | 176 |
| skin/blocks_walked/count | count | 5 | 7 | 5 |
| skin/joint_bytes_written/bytes | bytes | 9984 | 12.5 k | 7168 |
| skin/joints_written/count | count | 156 | 196 | 112 |
| skin/upload_writes/count | count | 3 | 4 | 3 |
| started_load_count |  | 5306 | 5308 | 5308 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 174 | 174 | 174 |
| statics/materials_bound | count | 174 | 174 | 174 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.05 | 0.05 | 0.05 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 187 | 187 | 187 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.35 | 40.48 | 24.62 |
| system/mem_usage | % | 56.23 | 56.47 | 56.45 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2548 | 2548 | 2548 |
| textures/images | count | 2000 | 2000 | 2000 |
| views/all/batch_sets | count | 105 | 152 | 105 |
| views/all/bins | count | 3193 | 3806 | 3193 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 53 | 71 | 53 |
| views/all/instance_draws | count | 17.6 k | 29.4 k | 17.6 k |
| views/all/instance_slots | count | 1.50 M | 1.66 M | 1.50 M |
| views/all/items | count | 9459 | 14.4 k | 9459 |
| views/all/pipelines | count | 89 | 96 | 89 |
| views/all/pipelines_distinct | count | 73 | 73 | 73 |
| views/all/work_items | count | 1.50 M | 1.68 M | 1.50 M |
| views/cascade/batch_sets | count | 9 | 56 | 9 |
| views/cascade/bins | count | 911 | 1524 | 911 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 9 | 27 | 9 |
| views/cascade/instance_draws | count | 5866 | 17.6 k | 5866 |
| views/cascade/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| views/cascade/items | count | 3217 | 8176 | 3217 |
| views/cascade/pipelines | count | 33 | 40 | 33 |
| views/cascade/work_items | count | 82.9 k | 256.7 k | 82.9 k |
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
| views/main/batch_sets | count | 96 | 96 | 96 |
| views/main/bins | count | 2282 | 2282 | 2282 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 44 | 44 | 44 |
| views/main/instance_draws | count | 11.8 k | 11.8 k | 11.8 k |
| views/main/instance_slots | count | 1.41 M | 1.41 M | 1.41 M |
| views/main/items | count | 6242 | 6242 | 6242 |
| views/main/pipelines | count | 56 | 56 | 56 |
| views/main/work_items | count | 1.42 M | 1.42 M | 1.42 M |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 15.5 k | 15.5 k | 15.5 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.12 | 3 | 143 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 0.00 | 4 | 4 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| encode//count | count | 28.55 | 31 | 34.3 k |
| encode/bind_walk/init_textures | count | 7456 | 12.4 k | 8.95 M |
| encode/bind_walk/init_textures_skipped | count | 44.5 k | 54.6 k | 53.42 M |
| encode/bind_walk/inits | count | 652 | 779 | 782.8 k |
| encode/bind_walk/inits_skipped | count | 127 | 139 | 152.4 k |
| encode/bind_walk/merge_textures | count | 7391 | 12.3 k | 8.87 M |
| encode/bind_walk/merge_textures_skipped | count | 17.5 k | 27.6 k | 21.03 M |
| encode/bind_walk/merges | count | 135 | 162 | 161.7 k |
| encode/bind_walk/merges_skipped | count | 116 | 135 | 139.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1200 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1200 |
| encode/pass:early prepass/count | count | 1 | 1 | 1200 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1200 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1200 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1200 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1200 |
| encode/queue/command_buffers | count | 38.55 | 41 | 46.3 k |
| encode/queue/submits | count | 2 | 2 | 2400 |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 3.38 M | 2.26 G |
| encode/queue/write_buffer/calls | count | 250 | 322 | 299.9 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 39.17 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 10.8 k |
| encode/queue/write_texture/bytes | bytes | 22.0 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1200 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.75 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.06 | 1 | 71 |
| pipelines/created | count | 0.07 | 1 | 79 |
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
| surface/shelter_far_redraws | count | 0.00 | 1 | 1 |
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.5 ms; slowest 21, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2547.9 | main/busy/ms 2516, stage/main/post_update 2400, stage/main/update 102, surface/advance_cpu_ms 71.27, stage/main/pre_update 9.18, stage/main/last 1.08, surface/cell_upload_ms 0.73, stage/main/first 0.59, stream/drive/ms 0.52, stage/main/fixed_main_loop 0.43, weather/map/ms 0.28, stream/tick/ms 0.13 |
| warm-up | 3.36 | 809.9 | main/busy/ms 2565, stage/main/post_update 2435, render/thread/ms 748, render/busy/ms 592, stage/render/prepare_assets 557, stage/render/prepare_views 153, render/acquire/ms 153, stage/main/update 109, surface/advance_cpu_ms 71.27, stage/render/render 19.40, render/graph/ms 15.47, stage/render/camera_driver 13.42 |
| warm-up | 3.51 | 156.1 | render/thread/ms 901, render/busy/ms 655, stage/render/prepare_assets 568, pipelines/compile_ms 236, stage/render/prepare_views 153, render/acquire/ms 153, stage/render/render 146, pipelines/front_ms 141, pipelines/driver_ms 95.40, render/present/ms 93.29, main/busy/ms 83.11, render/graph/ms 50.96 |
| warm-up | 3.65 | 140.9 | pipelines/compile_ms 1757, pipelines/front_ms 966, pipelines/wait_ms 800, pipelines/driver_ms 791, render/thread/ms 292, stage/render/render 216, render/busy/ms 200, render/graph/ms 122, render/present/ms 92.20, stage/render/camera_driver 72.19, main/busy/ms 63.65, stage/render/prepare_assets 53.14 |
| warm-up | 3.75 | 93.1 | pipelines/wait_ms 7023, pipelines/compile_ms 3554, pipelines/front_ms 2732, pipelines/driver_ms 821, render/thread/ms 227, render/busy/ms 224, main/busy/ms 146, stage/render/render 95.28, render/graph/ms 91.91, stage/main/update 79.47, stage/render/camera_driver 71.92, stage/render/prepare_assets 68.06 |
| warm-up | 3.93 | 179.7 | pipelines/wait_ms 10.3 k, pipelines/compile_ms 3658, pipelines/front_ms 3400, pipelines/driver_ms 257, render/thread/ms 232, render/busy/ms 230, main/busy/ms 138, stage/render/prepare_assets 117, stage/main/update 76.72, stage/render/prepare_flush 52.55, stage/main/post_update 36.50, stage/render/prepare_resources 23.89 |
| warm-up | 3.97 | 45.4 | pipelines/wait_ms 4344, pipelines/compile_ms 2548, pipelines/front_ms 2340, pipelines/driver_ms 208, render/thread/ms 188, render/busy/ms 185, encode//sum 120, stage/render/prepare_assets 94.06, main/busy/ms 61.22, stage/render/prepare_flush 43.01, stage/main/post_update 39.39, stage/render/render 34.70 |
| warm-up | 4.10 | 51.4 | render/thread/ms 70.53, render/busy/ms 68.23, pipelines/compile_ms 49.70, pipelines/front_ms 47.07, main/busy/ms 45.44, stage/render/prepare_assets 33.18, stage/render/render 19.50, stage/main/post_update 18.02, render/graph/ms 16.44, stage/main/update 15.07, encode//sum 13.25, stage/render/graph_submit 11.48 |
| warm-up | 4.15 | 44.5 | pipelines/compile_ms 854, pipelines/front_ms 808, render/thread/ms 89.73, render/busy/ms 87.26, stage/render/prepare_assets 50.44, pipelines/driver_ms 46.33, main/busy/ms 42.08, stage/render/render 19.41, stage/main/update 16.71, render/graph/ms 16.23, stage/main/post_update 16.00, stage/render/graph_submit 12.10 |
| warm-up | 4.22 | 72.1 | pipelines/compile_ms 1382, pipelines/front_ms 1319, render/thread/ms 109, render/busy/ms 104, stage/render/prepare_assets 64.57, pipelines/driver_ms 62.43, main/busy/ms 36.29, stage/render/render 31.18, render/graph/ms 25.29, stage/render/graph_submit 20.17, pipelines/wait_ms 13.58, stage/main/post_update 12.66 |
| warm-up | 4.30 | 83.3 | pipelines/compile_ms 1115, pipelines/front_ms 1082, render/thread/ms 149, render/busy/ms 145, stage/render/prepare_assets 111, pipelines/driver_ms 32.67, stage/render/render 29.01, main/busy/ms 25.49, render/graph/ms 23.83, stage/render/graph_submit 19.08, pipelines/wait_ms 11.21, stage/main/update 8.67 |
| warm-up | 4.33 | 31.7 | pipelines/compile_ms 767, pipelines/front_ms 748, render/thread/ms 112, render/busy/ms 111, stage/render/prepare_assets 92.90, main/busy/ms 20.09, pipelines/driver_ms 19.21, stage/render/render 13.13, render/graph/ms 11.46, stage/main/update 8.49, stage/render/graph_submit 8.45, encode//sum 7.91 |
| warm-up | 4.57 | 38.9 | render/thread/ms 61.14, render/busy/ms 59.16, main/busy/ms 55.05, stage/render/prepare_assets 36.35, stage/main/update 30.99, stage/render/render 15.03, stage/main/post_update 12.97, render/graph/ms 12.15, encode//sum 9.22, stage/render/graph_submit 8.42, stream/drive/ms 7.59, encode/submit 6.49 |
| warm-up | 4.63 | 57.4 | render/thread/ms 91.29, main/busy/ms 89.46, render/busy/ms 89.21, stage/main/update 64.84, stage/render/prepare_assets 52.37, stage/render/render 19.90, render/graph/ms 17.12, character/finish/ms 15.45, stage/main/post_update 11.66, stage/render/graph_submit 11.48, encode//sum 9.89, stream/drive/ms 9.16 |
| warm-up | 4.70 | 65.7 | pipelines/compile_ms 269, pipelines/front_ms 259, render/thread/ms 117, main/busy/ms 117, render/busy/ms 115, stage/main/update 96.36, stage/render/prepare_assets 68.11, pipelines/wait_ms 51.68, character/finish/ms 28.42, stage/render/render 17.98, render/graph/ms 15.34, stage/main/post_update 10.89 |
| warm-up | 4.77 | 73.3 | pipelines/compile_ms 354, pipelines/front_ms 339, pipelines/wait_ms 306, render/thread/ms 134, render/busy/ms 133, main/busy/ms 90.63, stage/render/prepare_assets 75.58, stage/main/update 67.91, stage/render/prepare_flush 24.13, pipelines/driver_ms 15.47, character/finish/ms 15.14, stage/main/post_update 14.27 |
| warm-up | 5.27 | 37.3 | pipelines/wait_ms 2168, pipelines/compile_ms 410, pipelines/front_ms 397, render/thread/ms 49.19, render/busy/ms 47.01, main/busy/ms 43.69, stage/main/update 24.91, stage/render/render 17.51, stage/render/prepare_assets 14.62, render/graph/ms 14.44, stage/main/post_update 13.51, pipelines/driver_ms 13.01 |
| warm-up | 5.35 | 53.3 | pipelines/wait_ms 525, render/thread/ms 73.01, render/busy/ms 71.27, main/busy/ms 47.13, stage/render/prepare_assets 35.43, stage/main/update 24.82, stage/render/render 19.67, render/graph/ms 16.95, encode//sum 13.23, stage/main/post_update 12.89, pipelines/compile_ms 12.36, pipelines/front_ms 11.86 |
| warm-up | 5.40 | 32.1 | pipelines/compile_ms 185, pipelines/front_ms 181, pipelines/wait_ms 103, render/thread/ms 51.88, render/busy/ms 50.58, main/busy/ms 39.11, stage/render/prepare_assets 27.24, stage/main/update 21.14, stage/main/post_update 12.85, stage/render/render 11.99, render/graph/ms 10.41, encode//sum 8.64 |
| warm-up | 5.73 | 36.1 | pipelines/wait_ms 861, render/thread/ms 45.61, render/busy/ms 43.95, main/busy/ms 42.86, stage/main/update 24.16, pipelines/compile_ms 21.75, pipelines/front_ms 21.14, stage/render/prepare_assets 16.67, stage/render/render 14.08, render/graph/ms 11.84, stage/main/post_update 10.84, encode//sum 9.08 |
| warm-up | 6.24 | 34.1 | render/thread/ms 46.90, render/busy/ms 44.94, pipelines/compile_ms 35.00, pipelines/front_ms 33.37, main/busy/ms 33.08, stage/render/render 21.24, render/graph/ms 18.55, stage/main/post_update 14.95, stage/render/graph_submit 13.40, encode//sum 13.03, stage/main/update 12.80, encode/submit 8.30 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --set groundcover.density_scale=0 --resolution 2560x1440 --bench-out target/perf\gc_p0\balmora@2560x1440_arm=empty-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 91.7 W · 78 °C (max 80) · VRAM 6916 MiB · throttled 100% of samples

## Tunables

- `far.handoff_scale=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `groundcover.density_scale=0 (command line: --set)`
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
