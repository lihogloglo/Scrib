# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1198 frames over 15.0 s · **median 12.61 ms (79 fps)** · mean 12.53 · p95 14.11 · p99 14.58 (1% low 69 fps) · worst 17.15 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 310 frames over 9.2 s, worst 2544.0 ms
- hitches (> 31.5 ms): **0** measured, 37 in warm-up
- timed GPU passes: 11.02 ms of a 12.53 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.61 ms, limit 11 ms
- pass worst frame 17.15 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.191 | 3.191 | 3.267 | 1.00 | 0.084 |
| dlss_super_resolution | 2.023 | 2.023 | 2.048 | 1.00 | 0.178 |
| sun_visibility | 0.779 | 0.779 | 0.807 | 1.00 | 0.006 |
| gpu_instance_materialization | 0.666 | 0.666 | 0.905 | 1.00 | 0.008 |
| early prepass | 0.630 | 0.630 | 0.638 | 1.00 | 0.122 |
| main_transmissive_pass_3d | 0.400 | 0.400 | 0.411 | 1.00 | 0.007 |
| atmosphere_luts | 0.370 | 0.370 | 0.378 | 1.00 | 0.003 |
| bloom | 0.316 | 0.316 | 0.331 | 1.00 | 0.057 |
| atmos_froxel_inject | 0.310 | 0.310 | 0.308 | 1.00 | 0.001 |
| water_velocity | 0.306 | 0.306 | 0.310 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.242 | 0.242 | 0.273 | 1.00 | 0.003 |
| water_sspr | 0.224 | 0.224 | 0.214 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.151 | 0.151 | 0.169 | 1.00 | 0.001 |
| water_fft | 0.136 | 0.136 | 0.140 | 1.00 | 0.002 |
| uniform_allocation | 0.120 | 0.120 | 0.153 | 1.00 | 0.018 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| upscaling | 0.114 | 0.114 | 0.119 | 1.00 | 0.004 |
| bin_unpacking | 0.098 | 0.098 | 0.123 | 1.00 | 0.014 |
| tonemapping | 0.086 | 0.086 | 0.089 | 1.00 | 0.003 |
| range_unpacking | 0.085 | 0.085 | 0.095 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.075 | 1.00 | 0.020 |
| render_sky | 0.063 | 0.063 | 0.063 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.411 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.030 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.228 | 0.12 | 0.003 |
| per_view_shadow_pass | 0.027 | 0.027 | 0.053 | 1.00 | 0.006 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 1.00 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.023 | 1.00 | 0.183 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.001 |
| surface_exposure_far | 0.011 | 0.036 | 0.039 | 0.31 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| surface_exposure | 0.010 | 0.036 | 0.037 | 0.28 | 0.001 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.528 | 13.137 | 16.10 | 13810.7 |
| render/busy/ms | 7.399 | 8.481 | 15.63 | 8863.6 |
| main/busy/ms | 6.420 | 7.587 | 10.02 | 7690.7 |
| encode//sum | 4.961 | 6.413 | 10.56 | 5942.9 |
| stage/render/render | 4.889 | 5.728 | 6.70 | 5857.0 |
| render/graph/ms | 4.301 | 5.091 | 6.00 | 5152.4 |
| stage/render/prepare_views | 3.823 | 5.574 | 7.48 | 4580.0 |
| render/acquire/ms | 3.772 | 5.527 | 7.43 | 4518.3 |
| stage/main/post_update | 3.553 | 4.535 | 6.46 | 4256.3 |
| stage/render/graph_submit | 3.105 | 3.859 | 4.77 | 3719.6 |
| encode/submit | 1.893 | 2.305 | 2.81 | 2267.8 |
| stage/main/update | 1.295 | 1.484 | 2.14 | 1551.0 |
| encode/finish_wall | 1.232 | 1.643 | 2.26 | 1475.8 |
| stage/render/camera_driver | 1.110 | 1.274 | 1.53 | 1329.7 |
| pipelines/compile_ms | 0.876 | 0.000 | 120.41 | 1049.0 |
| pipelines/front_ms | 0.854 | 0.000 | 118.01 | 1023.3 |
| encode/pass:early prepass/sum | 0.799 | 1.107 | 1.57 | 957.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.769 | 1.078 | 1.48 | 921.8 |
| encode/pass:shadow_directional_light/sum | 0.767 | 1.358 | 1.93 | 918.5 |
| stage/render/prepare_write_phase_buffers | 0.486 | 0.706 | 1.38 | 582.8 |
| stage/main/fixed_main_loop | 0.455 | 0.702 | 1.35 | 545.0 |
| stage/render/prepare_resources | 0.429 | 0.496 | 0.70 | 514.0 |
| stage/render/prepare_assets | 0.392 | 0.474 | 1.46 | 469.3 |
| stage/main/pre_update | 0.384 | 0.460 | 0.99 | 460.1 |
| render/present/ms | 0.358 | 0.426 | 0.75 | 428.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.330 | 0.473 | 0.76 | 395.4 |
| stage/main/first | 0.316 | 0.377 | 1.96 | 378.3 |
| bigspace/total/ms | 0.306 | 0.519 | 1.42 | 366.6 |
| stage/render/prepare_flush | 0.270 | 0.353 | 0.73 | 323.3 |
| stage/render/prepare_batch_phases | 0.265 | 0.370 | 0.71 | 317.4 |
| stage/render/prepare_bind_groups | 0.258 | 0.367 | 0.60 | 309.4 |
| stage/main/last | 0.244 | 0.343 | 0.86 | 291.9 |
| stage/render/prepare_meshes | 0.212 | 0.260 | 1.00 | 253.4 |
| stage/render/extract_commands | 0.192 | 0.225 | 0.32 | 230.1 |
| controller/feed/ms | 0.171 | 0.208 | 0.66 | 204.6 |
| stage/render/create_views | 0.157 | 0.191 | 0.38 | 188.1 |
| stream/drive/ms | 0.133 | 0.152 | 0.30 | 159.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.122 | 0.180 | 0.32 | 146.3 |
| stream/tick/ms | 0.107 | 0.124 | 0.25 | 128.4 |
| stage/render/queue | 0.105 | 0.142 | 2.26 | 125.8 |
| bigspace/high_precision/ms | 0.105 | 0.217 | 1.17 | 125.5 |
| controller/move/ms | 0.088 | 0.226 | 0.37 | 105.8 |
| bigspace/low_precision/ms | 0.078 | 0.107 | 0.71 | 94.0 |
| stage/render/specialize | 0.076 | 0.097 | 3.88 | 90.5 |
| anim/evaluate/ms | 0.068 | 0.082 | 0.67 | 81.9 |
| anim/evaluate_cpu/ms | 0.062 | 0.073 | 0.65 | 74.5 |
| encode/resolve_diagnostics/sum | 0.058 | 0.085 | 0.15 | 69.4 |
| controller/write/ms | 0.053 | 0.069 | 0.14 | 63.7 |
| stage/render/prepare_collect_phase_buffers | 0.045 | 0.076 | 0.23 | 53.8 |
| terrain/update/ms | 0.034 | 0.044 | 0.44 | 41.0 |
| physics/step/ms | 0.032 | 0.050 | 0.16 | 37.8 |
| stage/render/phase_sort | 0.029 | 0.049 | 0.16 | 34.8 |
| stage/render/cleanup | 0.026 | 0.035 | 0.30 | 31.2 |
| encode/camera_driver_span_begin/sum | 0.025 | 0.038 | 0.24 | 29.9 |
| weather/map/ms | 0.024 | 0.032 | 0.07 | 28.6 |
| pipelines/driver_ms | 0.021 | 0.000 | 3.33 | 25.7 |
| encode/camera_driver_span_end/sum | 0.021 | 0.034 | 0.13 | 25.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.03 | 12.4 |
| script/update/ms | 0.006 | 0.007 | 0.04 | 7.1 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 5.6 |
| controller/avoid/ms | 0.004 | 0.006 | 0.02 | 5.0 |
| anim/apply/ms | 0.004 | 0.005 | 0.03 | 4.8 |
| far/select/ms | 0.004 | 0.004 | 0.02 | 4.4 |
| script/ui/ms | 0.003 | 0.004 | 0.02 | 3.4 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.01 | 2.1 |
| character/finish/ms | 0.001 | 0.002 | 0.04 | 1.6 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.00 | 1.6 |
| stream/promote/ms | 0.001 | 0.001 | 0.00 | 1.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.00 | 0.6 |
| terrain/select/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| stage/render/prepare | 0.000 | 0.002 | 0.04 | 0.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.03 | 0.3 |
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
| crowd/joints_extracted/count | count | 147 | 181 | 181 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 38.0 k | 38.0 k | 38.0 k |
| encode//max | ms | 0.59 | 1.73 | 0.57 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.24 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.13 | 0.01 |
| encode/finish_critical | ms | 0.94 | 1.93 | 0.84 |
| encode/pass:early prepass/max | ms | 0.80 | 1.57 | 0.84 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.77 | 1.48 | 0.61 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.32 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.32 | 0.76 | 0.29 |
| encode/pass:shadow_directional_light/max | ms | 0.72 | 1.93 | 0.81 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.15 | 0.06 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 11.54 | 15.63 | 11.42 |
| gpu/timed/ms | ms | 11.54 | 13.27 | 11.42 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1288 | 1288 | 1288 |
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
| precache/throttled | count | 0 | 5 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 22.54 | 35.58 | 24.32 |
| process/mem_usage | GiB | 3.03 | 3.11 | 3.11 |
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
| skin/blocks_changed/count | count | 5 | 7 | 7 |
| skin/blocks_skipped/count | count | 176 | 179 | 174 |
| skin/blocks_walked/count | count | 5 | 7 | 7 |
| skin/joint_bytes_written/bytes | bytes | 10.2 k | 12.5 k | 12.5 k |
| skin/joints_written/count | count | 160 | 196 | 196 |
| skin/upload_writes/count | count | 5 | 6 | 6 |
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
| system/cpu_usage | % | 28.13 | 37.20 | 29.04 |
| system/mem_usage | % | 55.67 | 55.93 | 55.93 |
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
| anim/events/count | count | 0.12 | 3 | 139 |
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
| encode//count | count | 28.55 | 31 | 34.2 k |
| encode/bind_walk/init_textures | count | 7454 | 12.4 k | 8.93 M |
| encode/bind_walk/init_textures_skipped | count | 52.7 k | 58.5 k | 63.15 M |
| encode/bind_walk/inits | count | 652 | 779 | 781.5 k |
| encode/bind_walk/inits_skipped | count | 135 | 144 | 161.7 k |
| encode/bind_walk/merge_textures | count | 7389 | 12.3 k | 8.85 M |
| encode/bind_walk/merge_textures_skipped | count | 14.1 k | 19.9 k | 16.95 M |
| encode/bind_walk/merges | count | 135 | 162 | 161.4 k |
| encode/bind_walk/merges_skipped | count | 118 | 133 | 141.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1198 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1198 |
| encode/pass:early prepass/count | count | 1 | 1 | 1198 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1198 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1198 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1198 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1198 |
| encode/queue/command_buffers | count | 38.55 | 41 | 46.2 k |
| encode/queue/submits | count | 2 | 2 | 2396 |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 3.38 M | 2.26 G |
| encode/queue/write_buffer/calls | count | 252 | 325 | 301.6 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 39.10 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 10.8 k |
| encode/queue/write_texture/bytes | bytes | 22.0 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1198 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.75 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.01 | 1 | 14 |
| pipelines/created | count | 0.01 | 1 | 14 |
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

## Hitches (> 31.5 ms; slowest 37, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2544.0 | main/busy/ms 2511, stage/main/post_update 2395, stage/main/update 103, surface/advance_cpu_ms 72.10, stage/main/pre_update 9.39, stage/main/last 1.10, surface/cell_upload_ms 0.99, stream/drive/ms 0.49, stage/main/first 0.47, stage/main/fixed_main_loop 0.45, weather/map/ms 0.18, stream/tick/ms 0.11 |
| warm-up | 3.23 | 687.8 | main/busy/ms 2556, stage/main/post_update 2426, render/thread/ms 627, render/busy/ms 533, stage/render/prepare_assets 508, stage/main/update 109, stage/render/prepare_views 92.04, render/acquire/ms 91.82, surface/advance_cpu_ms 72.10, stage/render/render 12.48, stage/main/pre_update 11.33, render/graph/ms 8.22 |
| warm-up | 3.34 | 105.7 | render/thread/ms 728, render/busy/ms 579, stage/render/prepare_assets 513, main/busy/ms 106, stage/render/render 99.02, stage/render/prepare_views 92.70, render/acquire/ms 92.34, pipelines/compile_ms 74.87, stage/main/post_update 71.47, pipelines/front_ms 70.99, render/present/ms 57.25, render/graph/ms 39.54 |
| warm-up | 3.44 | 99.8 | pipelines/compile_ms 1571, pipelines/front_ms 958, pipelines/driver_ms 613, pipelines/wait_ms 372, render/thread/ms 194, render/busy/ms 133, stage/render/render 131, main/busy/ms 101, pipelines/lock_ms 84.79, render/graph/ms 69.25, stage/main/post_update 68.04, render/present/ms 59.93 |
| warm-up | 3.57 | 134.1 | pipelines/wait_ms 5605, pipelines/compile_ms 2789, pipelines/front_ms 2066, pipelines/driver_ms 723, render/thread/ms 225, render/busy/ms 215, main/busy/ms 171, pipelines/lock_ms 86.83, stage/main/update 79.91, stage/render/render 64.55, stage/render/prepare_assets 60.86, render/graph/ms 54.12 |
| warm-up | 3.69 | 116.2 | pipelines/wait_ms 10.0 k, pipelines/compile_ms 2825, pipelines/front_ms 2629, render/thread/ms 235, render/busy/ms 214, pipelines/driver_ms 196, main/busy/ms 174, stage/main/update 92.03, stage/render/prepare_assets 87.75, stage/main/post_update 47.16, stage/render/prepare_batch_phases 40.59, stage/render/prepare_resources 34.26 |
| warm-up | 3.73 | 40.2 | pipelines/wait_ms 5697, pipelines/compile_ms 1981, pipelines/front_ms 1870, render/thread/ms 141, render/busy/ms 123, pipelines/driver_ms 111, stage/render/prepare_assets 79.20, main/busy/ms 69.16, stage/main/post_update 35.75, stage/main/update 20.70, stage/render/prepare_views 16.16, render/acquire/ms 15.99 |
| warm-up | 3.76 | 33.7 | pipelines/compile_ms 1374, pipelines/front_ms 1304, pipelines/wait_ms 1194, render/thread/ms 70.03, pipelines/driver_ms 69.95, render/busy/ms 68.34, main/busy/ms 46.47, stage/render/prepare_assets 35.64, stage/main/post_update 27.23, terrain/update/ms 10.35, stage/render/prepare_flush 10.20, stage/render/render 9.35 |
| warm-up | 3.81 | 46.8 | pipelines/compile_ms 1546, pipelines/front_ms 1483, pipelines/wait_ms 544, render/thread/ms 77.76, render/busy/ms 75.94, pipelines/driver_ms 63.23, main/busy/ms 50.82, stage/render/render 32.45, render/graph/ms 30.18, stage/main/post_update 28.50, stage/render/camera_driver 22.35, stage/render/prepare_assets 21.98 |
| warm-up | 3.86 | 52.0 | pipelines/compile_ms 1166, pipelines/front_ms 1117, pipelines/wait_ms 313, render/thread/ms 95.95, render/busy/ms 93.12, main/busy/ms 58.05, stage/render/render 51.28, pipelines/driver_ms 49.81, render/graph/ms 47.71, stage/main/update 24.83, stage/main/post_update 24.32, stage/render/graph_submit 24.09 |
| warm-up | 3.93 | 69.4 | pipelines/compile_ms 1057, pipelines/front_ms 979, render/thread/ms 110, render/busy/ms 106, pipelines/wait_ms 106, pipelines/driver_ms 78.22, stage/render/prepare_assets 48.99, main/busy/ms 43.52, stage/render/render 38.83, render/graph/ms 34.29, stage/render/graph_submit 27.30, stage/main/post_update 16.32 |
| warm-up | 4.00 | 67.6 | pipelines/compile_ms 1239, pipelines/front_ms 1161, render/thread/ms 124, render/busy/ms 122, stage/render/prepare_assets 79.43, pipelines/driver_ms 78.07, pipelines/wait_ms 47.23, main/busy/ms 33.28, stage/render/render 31.94, render/graph/ms 28.57, stage/render/graph_submit 22.38, stage/main/update 12.35 |
| warm-up | 4.06 | 60.4 | pipelines/compile_ms 1057, pipelines/front_ms 1019, render/thread/ms 124, render/busy/ms 122, stage/render/prepare_assets 92.46, pipelines/driver_ms 37.12, main/busy/ms 30.17, stage/render/render 23.09, render/graph/ms 20.96, pipelines/wait_ms 19.21, stage/render/graph_submit 17.97, stage/main/update 11.14 |
| warm-up | 4.09 | 34.8 | pipelines/compile_ms 622, pipelines/front_ms 604, render/thread/ms 91.19, render/busy/ms 89.62, stage/render/prepare_assets 66.84, main/busy/ms 28.00, pipelines/driver_ms 17.28, stage/render/render 16.98, render/graph/ms 14.99, stage/render/graph_submit 12.04, stage/main/update 10.51, stage/main/post_update 10.35 |
| warm-up | 4.22 | 39.9 | pipelines/compile_ms 744, pipelines/front_ms 716, render/thread/ms 66.01, render/busy/ms 63.94, main/busy/ms 44.99, stage/render/prepare_assets 29.92, pipelines/driver_ms 27.49, stage/main/update 20.42, stage/main/post_update 15.97, stage/render/render 15.58, render/graph/ms 13.03, encode//sum 10.60 |
| warm-up | 4.25 | 32.8 | render/thread/ms 69.67, render/busy/ms 67.73, main/busy/ms 47.61, stage/render/prepare_assets 35.75, stage/main/update 22.27, stage/main/post_update 15.64, stage/render/render 15.45, render/graph/ms 12.80, encode//sum 10.74, stage/render/graph_submit 8.87, encode/submit 6.38, stage/render/prepare_resources 5.36 |
| warm-up | 4.33 | 79.9 | pipelines/compile_ms 1240, pipelines/front_ms 1192, render/thread/ms 109, render/busy/ms 102, main/busy/ms 80.49, pipelines/wait_ms 59.42, pipelines/driver_ms 47.92, stage/render/render 47.79, stage/render/prepare_assets 43.28, stage/main/update 42.30, render/graph/ms 39.38, stage/main/post_update 29.90 |
| warm-up | 4.37 | 40.1 | pipelines/compile_ms 1391, pipelines/front_ms 1337, pipelines/wait_ms 171, render/thread/ms 117, render/busy/ms 109, main/busy/ms 79.04, pipelines/driver_ms 54.29, stage/render/render 49.22, stage/render/prepare_assets 43.61, stage/main/update 40.98, render/graph/ms 40.82, stage/main/post_update 31.43 |
| warm-up | 4.43 | 53.7 | pipelines/compile_ms 239, pipelines/front_ms 228, pipelines/wait_ms 143, render/thread/ms 91.02, render/busy/ms 89.43, main/busy/ms 59.85, stage/render/prepare_assets 44.27, stage/main/post_update 29.56, stage/main/update 22.78, stage/render/render 14.16, render/graph/ms 12.06, pipelines/driver_ms 10.59 |
| warm-up | 4.51 | 81.3 | pipelines/compile_ms 1266, pipelines/front_ms 1231, render/thread/ms 133, render/busy/ms 131, main/busy/ms 106, pipelines/wait_ms 94.79, stage/main/post_update 80.53, stage/render/prepare_assets 38.44, pipelines/driver_ms 35.60, stage/render/prepare_batch_phases 33.46, stage/main/update 19.20, stage/render/prepare_meshes 13.62 |
| warm-up | 4.55 | 37.1 | pipelines/compile_ms 1841, pipelines/front_ms 1783, pipelines/wait_ms 204, render/thread/ms 116, render/busy/ms 114, main/busy/ms 100, stage/main/post_update 74.69, pipelines/driver_ms 57.67, stage/render/prepare_batch_phases 34.27, stage/render/prepare_assets 19.04, stage/main/update 17.18, stage/render/render 14.94 |
| warm-up | 4.58 | 31.7 | pipelines/compile_ms 865, pipelines/front_ms 832, pipelines/wait_ms 141, main/busy/ms 109, stage/main/update 73.42, render/thread/ms 63.17, render/busy/ms 58.76, pipelines/driver_ms 32.28, stage/main/post_update 24.41, stage/render/render 20.69, render/graph/ms 15.58, stage/render/prepare_assets 13.18 |
| warm-up | 4.69 | 111.1 | pipelines/compile_ms 1251, pipelines/front_ms 1208, pipelines/wait_ms 124, main/busy/ms 110, render/thread/ms 92.75, render/busy/ms 88.26, stage/main/update 85.67, stage/render/prepare_assets 56.06, pipelines/driver_ms 42.76, stage/render/render 20.45, character/finish/ms 15.77, render/graph/ms 15.21 |
| warm-up | 4.75 | 61.0 | pipelines/compile_ms 1737, pipelines/front_ms 1678, pipelines/wait_ms 191, render/thread/ms 123, render/busy/ms 121, main/busy/ms 91.91, stage/render/prepare_assets 69.96, pipelines/driver_ms 59.18, stage/main/update 40.36, stage/main/pre_update 24.75, stage/main/post_update 17.39, stage/render/render 15.83 |
| warm-up | 4.80 | 46.7 | pipelines/compile_ms 1416, pipelines/front_ms 1364, render/thread/ms 96.94, render/busy/ms 95.07, main/busy/ms 94.93, pipelines/wait_ms 81.22, pipelines/driver_ms 51.74, stage/main/update 45.00, stage/render/prepare_assets 27.25, stage/main/pre_update 22.94, stage/main/post_update 19.43, stage/render/prepare_resources 15.61 |
| warm-up | 4.83 | 33.2 | pipelines/compile_ms 727, pipelines/front_ms 697, render/thread/ms 70.99, render/busy/ms 68.99, main/busy/ms 56.37, stage/main/update 37.79, pipelines/driver_ms 29.31, stage/render/prepare_assets 17.46, stage/render/render 15.07, stage/render/prepare_write_phase_buffers 14.97, pipelines/wait_ms 14.47, render/graph/ms 12.89 |
| warm-up | 4.86 | 32.4 | render/thread/ms 62.28, render/busy/ms 60.72, main/busy/ms 39.03, stage/render/prepare_assets 19.12, stage/main/update 16.32, stage/render/render 14.44, render/graph/ms 12.41, stage/main/post_update 11.70, stage/render/graph_submit 8.83, encode//sum 8.42, stage/render/prepare_resources 7.14, encode/submit 6.41 |
| warm-up | 4.90 | 33.5 | pipelines/compile_ms 1238, pipelines/front_ms 1202, render/thread/ms 58.39, render/busy/ms 56.85, main/busy/ms 42.78, pipelines/driver_ms 36.09, stage/render/prepare_assets 16.75, stage/main/post_update 16.18, stage/main/update 15.92, stage/render/render 13.83, render/graph/ms 11.92, encode//sum 8.97 |
| warm-up | 4.96 | 34.0 | pipelines/compile_ms 513, pipelines/front_ms 500, render/thread/ms 59.03, render/busy/ms 57.34, main/busy/ms 40.99, pipelines/wait_ms 30.64, stage/main/update 22.44, stage/render/prepare_assets 15.86, stage/render/render 15.70, pipelines/driver_ms 13.36, render/graph/ms 13.16, stage/main/post_update 9.73 |
| warm-up | 5.09 | 42.6 | pipelines/compile_ms 294, pipelines/front_ms 284, render/thread/ms 63.33, render/busy/ms 61.74, main/busy/ms 53.46, stage/render/render 39.19, render/graph/ms 37.07, stage/render/graph_submit 32.79, stage/main/post_update 32.49, encode//sum 16.80, encode/finish_wall 15.69, stage/main/update 13.12 |
| warm-up | 5.18 | 42.5 | pipelines/compile_ms 414, pipelines/front_ms 402, render/thread/ms 62.58, render/busy/ms 61.10, main/busy/ms 50.65, stage/main/post_update 21.83, stage/main/update 19.90, stage/render/prepare_resources 15.88, stage/render/render 14.49, render/graph/ms 12.43, pipelines/driver_ms 12.09, encode//sum 9.14 |
| warm-up | 5.27 | 83.5 | pipelines/compile_ms 1412, pipelines/front_ms 1376, render/thread/ms 124, render/busy/ms 122, main/busy/ms 56.31, pipelines/driver_ms 36.37, stage/render/prepare_resources 35.93, stage/render/prepare_assets 28.19, stage/render/render 24.88, render/graph/ms 22.65, encode//sum 20.56, stage/main/post_update 19.55 |
| warm-up | 5.32 | 36.3 | pipelines/compile_ms 1207, pipelines/front_ms 1168, render/thread/ms 43.41, main/busy/ms 43.30, render/busy/ms 41.35, pipelines/driver_ms 39.34, stage/main/update 22.75, stage/render/render 17.43, render/graph/ms 14.55, stage/main/post_update 11.59, stage/render/graph_submit 9.65, encode//sum 9.56 |
| warm-up | 5.56 | 34.9 | render/thread/ms 44.60, render/busy/ms 42.35, main/busy/ms 35.63, pipelines/compile_ms 30.23, pipelines/front_ms 28.71, stage/render/render 20.29, stage/main/update 18.80, render/graph/ms 17.04, encode//sum 13.09, stage/render/graph_submit 12.04, stage/main/post_update 11.41, stage/render/prepare_assets 9.23 |
| warm-up | 5.60 | 43.3 | render/thread/ms 70.47, render/busy/ms 67.95, main/busy/ms 38.64, pipelines/compile_ms 30.23, pipelines/front_ms 28.71, stage/render/prepare_assets 25.20, stage/render/render 23.82, render/graph/ms 20.05, stage/main/update 17.48, stage/render/graph_submit 14.38, encode//sum 13.13, stage/main/post_update 12.70 |
| warm-up | 5.71 | 43.0 | pipelines/compile_ms 161, pipelines/front_ms 157, render/thread/ms 54.00, main/busy/ms 29.30, stage/render/prepare_views 27.63, render/acquire/ms 27.51, render/busy/ms 25.53, stage/main/update 15.88, stage/render/render 14.19, render/graph/ms 12.55, encode//sum 10.04, stage/render/graph_submit 9.62 |
| warm-up | 5.79 | 36.6 | render/thread/ms 53.57, render/busy/ms 52.36, pipelines/compile_ms 41.08, stage/render/render 32.53, main/busy/ms 32.52, render/graph/ms 30.82, encode//sum 29.67, stage/render/graph_submit 27.09, pipelines/front_ms 26.46, encode/finish_wall 21.71, encode/pass:early prepass/sum 20.92, encode/pass:shadow_directional_light/sum 20.69 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\gc_p0\balmora@2560x1440_arm=on-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 91.1 W · 78 °C (max 80) · VRAM 7172 MiB · throttled 100% of samples

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
