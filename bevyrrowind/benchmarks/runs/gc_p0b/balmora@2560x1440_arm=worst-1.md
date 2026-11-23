# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); groundcover.render.reserve_worst_case=on (command line: --set); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1190 frames over 15.0 s · **median 12.68 ms (79 fps)** · mean 12.61 · p95 14.29 · p99 14.80 (1% low 68 fps) · worst 17.06 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 312 frames over 8.9 s, worst 2562.8 ms
- hitches (> 31.7 ms): **0** measured, 21 in warm-up
- timed GPU passes: 11.11 ms of a 12.61 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.68 ms, limit 11 ms
- pass worst frame 17.06 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.201 | 3.201 | 3.294 | 1.00 | 0.085 |
| dlss_super_resolution | 2.028 | 2.028 | 2.050 | 1.00 | 0.178 |
| sun_visibility | 0.781 | 0.781 | 0.808 | 1.00 | 0.006 |
| gpu_instance_materialization | 0.760 | 0.760 | 0.902 | 1.00 | 0.008 |
| early prepass | 0.627 | 0.627 | 0.634 | 1.00 | 0.119 |
| main_transmissive_pass_3d | 0.396 | 0.396 | 0.408 | 1.00 | 0.007 |
| atmosphere_luts | 0.369 | 0.369 | 0.377 | 1.00 | 0.003 |
| bloom | 0.315 | 0.315 | 0.331 | 1.00 | 0.058 |
| atmos_froxel_inject | 0.310 | 0.310 | 0.309 | 1.00 | 0.001 |
| water_velocity | 0.306 | 0.306 | 0.310 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.244 | 0.244 | 0.283 | 1.00 | 0.003 |
| water_sspr | 0.218 | 0.218 | 0.213 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.152 | 0.152 | 0.169 | 1.00 | 0.001 |
| water_fft | 0.135 | 0.135 | 0.139 | 1.00 | 0.002 |
| clouds_composite | 0.117 | 0.117 | 0.118 | 1.00 | 0.002 |
| uniform_allocation | 0.116 | 0.116 | 0.148 | 1.00 | 0.018 |
| upscaling | 0.114 | 0.114 | 0.120 | 1.00 | 0.004 |
| bin_unpacking | 0.095 | 0.095 | 0.120 | 1.00 | 0.013 |
| tonemapping | 0.086 | 0.086 | 0.089 | 1.00 | 0.003 |
| range_unpacking | 0.083 | 0.083 | 0.094 | 1.00 | 0.002 |
| clustering | 0.065 | 0.065 | 0.075 | 1.00 | 0.020 |
| render_sky | 0.062 | 0.062 | 0.062 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.411 | 0.13 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.230 | 0.13 | 0.003 |
| per_view_shadow_pass | 0.026 | 0.026 | 0.055 | 1.00 | 0.006 |
| water_snapshot | 0.023 | 0.023 | 0.024 | 1.00 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.024 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.020 | 1.00 | 0.179 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.001 |
| surface_exposure_far | 0.011 | 0.035 | 0.036 | 0.31 | 0.002 |
| surface_exposure | 0.010 | 0.037 | 0.037 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.007 | 0.007 | 0.007 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.13 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.596 | 13.266 | 15.82 | 13799.0 |
| render/busy/ms | 7.479 | 8.742 | 15.36 | 8900.3 |
| main/busy/ms | 6.432 | 7.675 | 13.54 | 7654.1 |
| encode//sum | 5.067 | 6.644 | 11.42 | 6029.7 |
| stage/render/render | 4.946 | 5.871 | 7.41 | 5886.0 |
| pipelines/compile_ms | 4.643 | 15.607 | 121.53 | 5524.7 |
| pipelines/front_ms | 4.514 | 15.034 | 118.79 | 5371.9 |
| render/graph/ms | 4.355 | 5.211 | 6.61 | 5183.0 |
| stage/render/prepare_views | 3.810 | 5.547 | 6.43 | 4533.9 |
| render/acquire/ms | 3.759 | 5.497 | 6.36 | 4472.9 |
| stage/main/post_update | 3.571 | 4.525 | 6.84 | 4249.8 |
| stage/render/graph_submit | 3.165 | 3.999 | 5.39 | 3765.8 |
| encode/submit | 1.927 | 2.406 | 3.14 | 2293.3 |
| stage/main/update | 1.314 | 1.559 | 2.13 | 1563.2 |
| encode/finish_wall | 1.255 | 1.718 | 2.44 | 1493.4 |
| stage/render/camera_driver | 1.105 | 1.265 | 1.67 | 1314.9 |
| encode/pass:early prepass/sum | 0.813 | 1.106 | 1.37 | 968.0 |
| encode/pass:shadow_directional_light/sum | 0.785 | 1.384 | 1.96 | 934.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.777 | 1.094 | 1.45 | 924.2 |
| stage/render/prepare_write_phase_buffers | 0.470 | 0.696 | 0.94 | 559.4 |
| stage/render/prepare_resources | 0.432 | 0.509 | 0.73 | 513.6 |
| stage/main/fixed_main_loop | 0.429 | 0.619 | 1.10 | 511.1 |
| stage/render/prepare_assets | 0.395 | 0.477 | 1.58 | 470.5 |
| stage/main/pre_update | 0.379 | 0.452 | 0.75 | 451.0 |
| render/present/ms | 0.358 | 0.421 | 0.74 | 425.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.334 | 0.484 | 0.69 | 397.0 |
| stage/main/first | 0.313 | 0.377 | 1.71 | 372.6 |
| bigspace/total/ms | 0.306 | 0.499 | 1.52 | 363.6 |
| stage/render/prepare_flush | 0.272 | 0.392 | 0.93 | 323.3 |
| stage/render/prepare_batch_phases | 0.269 | 0.375 | 0.58 | 320.0 |
| stage/render/prepare_bind_groups | 0.259 | 0.402 | 0.58 | 308.6 |
| stage/main/last | 0.255 | 0.352 | 6.35 | 303.0 |
| stage/render/prepare_meshes | 0.217 | 0.268 | 1.30 | 258.5 |
| stage/render/extract_commands | 0.194 | 0.226 | 0.29 | 231.3 |
| controller/feed/ms | 0.175 | 0.221 | 0.30 | 207.8 |
| stage/render/create_views | 0.156 | 0.189 | 0.33 | 186.0 |
| stream/drive/ms | 0.133 | 0.153 | 0.26 | 158.0 |
| pipelines/driver_ms | 0.128 | 0.653 | 3.93 | 152.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.125 | 0.185 | 0.32 | 149.3 |
| stream/tick/ms | 0.107 | 0.125 | 0.22 | 127.6 |
| stage/render/queue | 0.104 | 0.142 | 2.26 | 123.5 |
| stage/render/specialize | 0.104 | 0.185 | 3.30 | 123.3 |
| bigspace/high_precision/ms | 0.103 | 0.202 | 0.92 | 122.6 |
| bigspace/low_precision/ms | 0.078 | 0.122 | 0.65 | 93.1 |
| anim/evaluate_cpu/ms | 0.065 | 0.076 | 0.20 | 77.7 |
| controller/move/ms | 0.061 | 0.120 | 0.27 | 72.2 |
| encode/resolve_diagnostics/sum | 0.060 | 0.091 | 0.30 | 71.5 |
| controller/write/ms | 0.052 | 0.067 | 0.14 | 61.4 |
| anim/evaluate/ms | 0.050 | 0.061 | 1.05 | 59.3 |
| stage/render/prepare_collect_phase_buffers | 0.043 | 0.073 | 0.28 | 50.8 |
| terrain/update/ms | 0.035 | 0.046 | 0.46 | 41.5 |
| physics/step/ms | 0.031 | 0.051 | 0.08 | 36.8 |
| stage/render/cleanup | 0.025 | 0.033 | 0.25 | 30.3 |
| stage/render/phase_sort | 0.025 | 0.049 | 0.11 | 30.0 |
| encode/camera_driver_span_begin/sum | 0.025 | 0.039 | 0.15 | 29.7 |
| weather/map/ms | 0.023 | 0.032 | 0.06 | 27.8 |
| encode/camera_driver_span_end/sum | 0.022 | 0.035 | 0.19 | 25.9 |
| weather/eval/ms | 0.011 | 0.014 | 0.03 | 12.8 |
| script/update/ms | 0.006 | 0.008 | 0.03 | 7.3 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 6.0 |
| controller/avoid/ms | 0.004 | 0.007 | 0.03 | 5.1 |
| anim/apply/ms | 0.004 | 0.005 | 0.02 | 5.1 |
| far/select/ms | 0.004 | 0.004 | 0.04 | 4.3 |
| script/ui/ms | 0.003 | 0.004 | 0.01 | 3.2 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.01 | 2.1 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.02 | 1.5 |
| character/finish/ms | 0.001 | 0.002 | 0.00 | 1.5 |
| pipelines/wait_ms | 0.001 | 0.014 | 0.03 | 1.4 |
| stream/promote/ms | 0.001 | 0.001 | 0.00 | 1.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.01 | 0.7 |
| terrain/select/ms | 0.001 | 0.001 | 0.00 | 0.6 |
| stage/render/prepare | 0.000 | 0.002 | 0.07 | 0.5 |
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
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1080 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 147 | 181 | 181 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 38.0 k | 38.0 k | 38.0 k |
| encode//max | ms | 0.61 | 2.05 | 0.73 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.15 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.19 | 0.02 |
| encode/finish_critical | ms | 0.95 | 2.05 | 1.09 |
| encode/pass:early prepass/max | ms | 0.82 | 1.37 | 0.48 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.78 | 1.45 | 1.01 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.32 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.33 | 0.69 | 0.46 |
| encode/pass:shadow_directional_light/max | ms | 0.73 | 1.96 | 1.09 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.30 | 0.06 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 11.76 | 15.36 | 11.44 |
| gpu/timed/ms | ms | 11.76 | 13.10 | 11.44 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1265 | 1265 | 1265 |
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
| phases/opaque3d/batch_sets | count | 30 | 30 | 30 |
| phases/opaque3d/bins | count | 883 | 883 | 883 |
| phases/opaque3d/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d/instance_draws | count | 5306 | 5306 | 5306 |
| phases/opaque3d/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d/items | count | 2792 | 2792 | 2792 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 21 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 26 | 26 | 26 |
| phases/opaque3d_prepass/bins | count | 883 | 883 | 883 |
| phases/opaque3d_prepass/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/instance_draws | count | 5306 | 5306 | 5306 |
| phases/opaque3d_prepass/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d_prepass/items | count | 2792 | 2792 | 2792 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 9 | 56 | 49 |
| phases/shadow/bins | count | 911 | 1524 | 911 |
| phases/shadow/instance_batches | count | 9 | 27 | 27 |
| phases/shadow/instance_draws | count | 5866 | 17.6 k | 17.6 k |
| phases/shadow/instance_slots | count | 82.9 k | 248.6 k | 248.6 k |
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
| precache/throttled | count | 0 | 42 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 53 |
| preprocess/bind_groups_created | count | 0 | 62 | 46 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 7 |
| preprocess/bind_groups_reused | count | 121 | 121 | 121 |
| process/cpu_usage | % | 25.93 | 40.38 | 18.75 |
| process/mem_usage | GiB | 2.99 | 3.08 | 3.08 |
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
| skin/blocks_changed/count | count | 6 | 7 | 7 |
| skin/blocks_skipped/count | count | 175 | 181 | 174 |
| skin/blocks_walked/count | count | 6 | 7 | 7 |
| skin/joint_bytes_written/bytes | bytes | 10.2 k | 12.5 k | 12.5 k |
| skin/joints_written/count | count | 160 | 196 | 196 |
| skin/upload_writes/count | count | 6 | 6 | 6 |
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
| system/cpu_usage | % | 29.69 | 40.34 | 22.63 |
| system/mem_usage | % | 50.85 | 51.09 | 51.09 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2548 | 2548 | 2548 |
| textures/images | count | 2000 | 2000 | 2000 |
| views/all/batch_sets | count | 103 | 150 | 143 |
| views/all/bins | count | 3193 | 3806 | 3193 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 53 | 71 | 71 |
| views/all/instance_draws | count | 17.6 k | 29.4 k | 29.4 k |
| views/all/instance_slots | count | 1.50 M | 1.66 M | 1.66 M |
| views/all/items | count | 9459 | 14.4 k | 9459 |
| views/all/pipelines | count | 87 | 94 | 87 |
| views/all/pipelines_distinct | count | 71 | 71 | 71 |
| views/all/work_items | count | 1.50 M | 1.68 M | 1.67 M |
| views/cascade/batch_sets | count | 9 | 56 | 49 |
| views/cascade/bins | count | 911 | 1524 | 911 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 9 | 27 | 27 |
| views/cascade/instance_draws | count | 5866 | 17.6 k | 17.6 k |
| views/cascade/instance_slots | count | 82.9 k | 248.6 k | 248.6 k |
| views/cascade/items | count | 3217 | 8176 | 3217 |
| views/cascade/pipelines | count | 33 | 40 | 33 |
| views/cascade/work_items | count | 82.9 k | 256.7 k | 251.8 k |
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
| views/main/batch_sets | count | 94 | 94 | 94 |
| views/main/bins | count | 2282 | 2282 | 2282 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 44 | 44 | 44 |
| views/main/instance_draws | count | 11.8 k | 11.8 k | 11.8 k |
| views/main/instance_slots | count | 1.41 M | 1.41 M | 1.41 M |
| views/main/items | count | 6242 | 6242 | 6242 |
| views/main/pipelines | count | 54 | 54 | 54 |
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
| anim/events/count | count | 0.14 | 3 | 166 |
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
| encode//count | count | 28.56 | 31 | 34.0 k |
| encode/bind_walk/init_textures | count | 7457 | 12.4 k | 8.87 M |
| encode/bind_walk/init_textures_skipped | count | 48.4 k | 58.5 k | 57.57 M |
| encode/bind_walk/inits | count | 648 | 775 | 771.6 k |
| encode/bind_walk/inits_skipped | count | 131 | 142 | 155.6 k |
| encode/bind_walk/merge_textures | count | 7393 | 12.3 k | 8.80 M |
| encode/bind_walk/merge_textures_skipped | count | 17.5 k | 27.6 k | 20.86 M |
| encode/bind_walk/merges | count | 135 | 162 | 160.4 k |
| encode/bind_walk/merges_skipped | count | 118 | 136 | 140.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1190 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1190 |
| encode/pass:early prepass/count | count | 1 | 1 | 1190 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1190 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1190 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1190 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1190 |
| encode/queue/command_buffers | count | 38.56 | 41 | 45.9 k |
| encode/queue/submits | count | 2 | 2 | 2380 |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 3.38 M | 2.25 G |
| encode/queue/write_buffer/calls | count | 246 | 318 | 292.6 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 38.84 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 10.7 k |
| encode/queue/write_texture/bytes | bytes | 22.1 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1190 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.76 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.06 | 1 | 72 |
| pipelines/created | count | 0.07 | 1 | 85 |
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

## Hitches (> 31.7 ms; slowest 21, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2562.8 | main/busy/ms 2531, stage/main/post_update 2417, stage/main/update 102, surface/advance_cpu_ms 70.50, stage/main/pre_update 9.32, stage/main/last 1.10, surface/cell_upload_ms 0.83, stage/main/first 0.55, stream/drive/ms 0.46, stage/main/fixed_main_loop 0.41, weather/map/ms 0.27, stream/tick/ms 0.12 |
| warm-up | 3.37 | 808.9 | main/busy/ms 2580, stage/main/post_update 2451, render/thread/ms 748, render/busy/ms 586, stage/render/prepare_assets 551, stage/render/prepare_views 158, render/acquire/ms 157, stage/main/update 109, surface/advance_cpu_ms 70.50, stage/render/render 24.08, render/graph/ms 17.92, stage/render/camera_driver 15.34 |
| warm-up | 3.51 | 142.5 | render/thread/ms 889, render/busy/ms 642, stage/render/prepare_assets 561, stage/render/prepare_views 158, render/acquire/ms 158, stage/render/render 142, main/busy/ms 122, render/present/ms 89.27, stage/main/post_update 80.59, render/graph/ms 50.78, pipelines/compile_ms 29.30, pipelines/front_ms 27.70 |
| warm-up | 3.61 | 97.5 | pipelines/compile_ms 1651, pipelines/front_ms 833, pipelines/driver_ms 818, render/thread/ms 236, stage/render/render 173, pipelines/wait_ms 164, render/busy/ms 149, main/busy/ms 105, render/present/ms 86.44, render/graph/ms 85.26, stage/main/post_update 65.98, stage/render/camera_driver 54.16 |
| warm-up | 3.72 | 112.2 | pipelines/wait_ms 4806, pipelines/compile_ms 3220, pipelines/front_ms 2313, pipelines/driver_ms 907, render/thread/ms 193, render/busy/ms 190, main/busy/ms 131, stage/main/update 79.47, stage/render/prepare_assets 76.93, stage/render/render 63.89, render/graph/ms 59.97, stage/render/camera_driver 51.89 |
| warm-up | 3.84 | 112.6 | pipelines/wait_ms 10.3 k, pipelines/compile_ms 2961, pipelines/front_ms 2790, render/thread/ms 202, render/busy/ms 200, pipelines/driver_ms 171, main/busy/ms 142, stage/render/prepare_assets 138, stage/main/update 81.80, stage/main/post_update 50.39, encode//sum 17.92, stage/render/render 17.45 |
| warm-up | 3.91 | 70.2 | pipelines/wait_ms 6317, pipelines/compile_ms 2336, pipelines/front_ms 2177, render/thread/ms 172, render/busy/ms 171, pipelines/driver_ms 159, stage/render/prepare_assets 102, main/busy/ms 75.66, stage/main/post_update 34.79, stage/render/prepare_resources 28.10, stage/render/prepare_flush 16.68, stage/render/render 14.94 |
| warm-up | 3.98 | 46.6 | pipelines/compile_ms 796, pipelines/front_ms 759, pipelines/wait_ms 145, render/thread/ms 71.47, render/busy/ms 69.36, pipelines/driver_ms 36.89, main/busy/ms 36.51, stage/render/render 22.86, render/graph/ms 20.35, encode//sum 20.27, stage/main/post_update 17.57, stage/render/prepare_assets 15.48 |
| warm-up | 4.01 | 32.9 | pipelines/compile_ms 1008, pipelines/front_ms 963, pipelines/wait_ms 379, render/thread/ms 76.49, render/busy/ms 73.66, main/busy/ms 53.73, pipelines/driver_ms 44.69, stage/render/render 21.25, stage/render/prepare_assets 20.40, stage/main/post_update 19.86, stage/main/update 18.57, render/graph/ms 17.76 |
| warm-up | 4.09 | 76.4 | pipelines/compile_ms 1391, pipelines/front_ms 1313, pipelines/wait_ms 753, render/thread/ms 96.67, render/busy/ms 93.52, pipelines/driver_ms 77.13, main/busy/ms 58.93, stage/render/prepare_assets 51.96, stage/render/render 23.91, render/graph/ms 19.79, stage/main/post_update 19.11, stage/main/update 18.43 |
| warm-up | 4.14 | 47.7 | pipelines/compile_ms 1485, pipelines/front_ms 1411, pipelines/wait_ms 559, render/thread/ms 110, render/busy/ms 108, pipelines/driver_ms 73.95, stage/render/prepare_assets 69.77, main/busy/ms 42.39, stage/render/render 21.44, render/graph/ms 18.24, stage/main/update 18.03, stage/render/graph_submit 13.07 |
| warm-up | 4.22 | 82.7 | pipelines/compile_ms 977, pipelines/front_ms 953, render/thread/ms 126, render/busy/ms 124, stage/render/prepare_assets 94.17, pipelines/wait_ms 58.09, main/busy/ms 35.83, pipelines/driver_ms 23.66, stage/render/render 16.28, stage/main/update 16.00, render/graph/ms 14.02, stage/main/post_update 11.38 |
| warm-up | 4.37 | 36.0 | render/thread/ms 49.89, render/busy/ms 48.02, main/busy/ms 31.69, stage/render/prepare_assets 28.14, stage/main/update 14.19, stage/render/render 14.13, render/graph/ms 11.87, stage/main/post_update 10.34, stage/render/graph_submit 8.86, encode//sum 8.70, encode/submit 6.75, stream/drive/ms 3.25 |
| warm-up | 4.44 | 45.5 | render/thread/ms 63.67, main/busy/ms 63.40, render/busy/ms 61.69, stage/main/update 37.77, stage/render/prepare_assets 35.41, pipelines/compile_ms 32.63, pipelines/front_ms 31.25, stage/render/render 15.53, stage/main/post_update 13.30, render/graph/ms 12.92, encode//sum 9.18, pipelines/wait_ms 9.07 |
| warm-up | 4.51 | 69.1 | pipelines/compile_ms 233, pipelines/front_ms 136, render/thread/ms 107, render/busy/ms 106, pipelines/driver_ms 96.91, pipelines/wait_ms 62.53, main/busy/ms 53.84, stage/render/prepare_assets 50.20, stage/main/update 34.20, stage/render/prepare_flush 34.12, stage/render/render 12.01, stage/main/post_update 11.78 |
| warm-up | 4.64 | 42.0 | pipelines/compile_ms 161, pipelines/front_ms 153, render/thread/ms 68.61, render/busy/ms 66.70, main/busy/ms 52.13, stage/render/prepare_assets 27.20, stage/main/update 24.71, stage/render/render 18.28, stage/main/post_update 16.28, render/graph/ms 15.43, stage/render/graph_submit 10.60, encode//sum 10.30 |
| warm-up | 5.14 | 37.2 | pipelines/wait_ms 1089, pipelines/compile_ms 223, pipelines/front_ms 214, main/busy/ms 54.30, render/thread/ms 43.19, render/busy/ms 41.70, stage/main/update 40.62, stage/render/render 12.29, stage/render/prepare_assets 11.45, render/graph/ms 10.06, stage/main/post_update 9.15, encode//sum 8.81 |
| warm-up | 5.27 | 33.5 | pipelines/wait_ms 1419, pipelines/compile_ms 177, pipelines/front_ms 168, render/thread/ms 53.16, render/busy/ms 51.28, main/busy/ms 37.87, stage/render/render 19.84, stage/render/prepare_assets 18.59, stage/main/update 17.78, render/graph/ms 16.99, stage/main/post_update 12.96, stage/render/graph_submit 12.32 |
| warm-up | 5.31 | 41.2 | pipelines/wait_ms 2707, pipelines/compile_ms 261, pipelines/front_ms 253, render/thread/ms 69.22, render/busy/ms 67.57, main/busy/ms 45.59, stage/render/prepare_assets 37.14, stage/main/update 23.39, stage/render/render 15.59, stage/main/post_update 14.09, render/graph/ms 13.05, encode//sum 10.15 |
| warm-up | 5.39 | 33.8 | pipelines/wait_ms 1430, pipelines/compile_ms 250, pipelines/front_ms 237, render/thread/ms 54.52, render/busy/ms 53.30, main/busy/ms 50.51, stage/main/update 26.78, stage/main/post_update 18.96, pipelines/driver_ms 13.69, stage/render/render 12.93, render/graph/ms 11.31, stage/render/prepare_assets 11.12 |
| warm-up | 6.12 | 33.0 | render/thread/ms 39.62, render/busy/ms 37.40, main/busy/ms 27.21, stage/render/render 16.84, render/graph/ms 14.67, stage/main/post_update 12.75, encode//sum 11.28, stage/render/graph_submit 10.82, stage/main/update 10.07, stage/render/prepare_assets 8.24, encode/submit 7.62, stage/render/camera_driver 3.55 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --set groundcover.render.reserve_worst_case=true --resolution 2560x1440 --bench-out target/perf\gc_p0b\balmora@2560x1440_arm=worst-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (285 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 90.0 W · 73 °C (max 76) · VRAM 7577 MiB · throttled 100% of samples

## Tunables

- `far.handoff_scale=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `groundcover.render.reserve_worst_case=on (command line: --set)`
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
