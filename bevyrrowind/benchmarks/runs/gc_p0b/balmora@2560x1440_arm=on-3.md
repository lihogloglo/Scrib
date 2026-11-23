# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1294 frames over 15.0 s · **median 11.67 ms (86 fps)** · mean 11.60 · p95 12.94 · p99 13.37 (1% low 75 fps) · worst 19.11 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 309 frames over 8.9 s, worst 2586.4 ms
- hitches (> 29.2 ms): **0** measured, 30 in warm-up
- timed GPU passes: 10.19 ms of a 11.60 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.67 ms, limit 11 ms
- pass worst frame 19.11 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.222 | 3.224 | 3.310 | 1.00 | 0.069 |
| dlss_super_resolution | 2.022 | 2.023 | 2.046 | 1.00 | 0.183 |
| sun_visibility | 0.778 | 0.779 | 0.807 | 1.00 | 0.006 |
| early prepass | 0.649 | 0.650 | 0.658 | 1.00 | 0.097 |
| main_transmissive_pass_3d | 0.399 | 0.399 | 0.410 | 1.00 | 0.008 |
| atmosphere_luts | 0.365 | 0.366 | 0.431 | 1.00 | 0.003 |
| bloom | 0.314 | 0.314 | 0.331 | 1.00 | 0.060 |
| atmos_froxel_inject | 0.308 | 0.309 | 0.309 | 1.00 | 0.001 |
| water_velocity | 0.307 | 0.307 | 0.311 | 1.00 | 0.002 |
| water_sspr | 0.216 | 0.216 | 0.213 | 1.00 | 0.002 |
| ssao | 0.188 | 0.188 | 0.196 | 1.00 | 0.007 |
| clouds_trace | 0.152 | 0.152 | 0.171 | 1.00 | 0.001 |
| water_fft | 0.134 | 0.134 | 0.136 | 1.00 | 0.002 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| uniform_allocation | 0.116 | 0.116 | 0.147 | 1.00 | 0.019 |
| upscaling | 0.113 | 0.113 | 0.121 | 1.00 | 0.004 |
| bin_unpacking | 0.090 | 0.090 | 0.114 | 1.00 | 0.015 |
| tonemapping | 0.085 | 0.085 | 0.088 | 1.00 | 0.003 |
| gpu_instance_materialization | 0.081 | 0.081 | 0.097 | 1.00 | 0.004 |
| clustering | 0.063 | 0.063 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.062 | 0.062 | 0.062 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.059 | 0.059 | 0.085 | 1.00 | 0.004 |
| lightprobe_irradiance_map | 0.051 | 0.409 | 0.411 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.030 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.222 | 0.231 | 0.12 | 0.004 |
| water_snapshot | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 1.00 | 0.002 |
| range_unpacking | 0.022 | 0.022 | 0.028 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.018 | 0.018 | 0.034 | 1.00 | 0.005 |
| main_transparent_pass_3d | 0.017 | 0.017 | 0.026 | 1.00 | 0.190 |
| surface_exposure_far | 0.012 | 0.039 | 0.040 | 0.31 | 0.002 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.001 |
| surface_exposure | 0.011 | 0.039 | 0.040 | 0.29 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.014 | 0.014 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.578 | 11.930 | 17.97 | 13688.5 |
| render/busy/ms | 7.266 | 8.281 | 17.43 | 9401.8 |
| main/busy/ms | 6.762 | 7.974 | 9.54 | 8750.4 |
| stage/render/render | 4.765 | 5.559 | 7.95 | 6166.4 |
| encode//sum | 4.668 | 6.012 | 9.83 | 6040.5 |
| render/graph/ms | 4.182 | 4.934 | 6.14 | 5411.5 |
| stage/main/post_update | 3.821 | 4.762 | 6.54 | 4943.7 |
| stage/render/prepare_views | 3.010 | 4.626 | 5.39 | 3895.0 |
| stage/render/graph_submit | 3.008 | 3.691 | 4.96 | 3891.9 |
| render/acquire/ms | 2.960 | 4.580 | 5.34 | 3830.6 |
| encode/submit | 1.843 | 2.246 | 4.04 | 2385.4 |
| stage/main/update | 1.363 | 1.772 | 2.25 | 1763.7 |
| pipelines/compile_ms | 1.242 | 0.000 | 126.42 | 1607.2 |
| pipelines/front_ms | 1.207 | 0.000 | 123.72 | 1562.2 |
| encode/finish_wall | 1.183 | 1.588 | 2.25 | 1530.6 |
| stage/render/camera_driver | 1.091 | 1.252 | 1.52 | 1412.1 |
| encode/pass:shadow_directional_light/sum | 0.752 | 1.269 | 2.07 | 972.6 |
| encode/pass:early prepass/sum | 0.748 | 1.030 | 1.40 | 968.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.701 | 0.998 | 1.33 | 907.7 |
| stage/render/prepare_write_phase_buffers | 0.506 | 0.734 | 0.95 | 654.7 |
| stage/main/fixed_main_loop | 0.456 | 0.700 | 1.23 | 589.6 |
| stage/render/prepare_resources | 0.421 | 0.488 | 0.75 | 544.7 |
| stage/render/prepare_assets | 0.389 | 0.474 | 1.63 | 503.2 |
| stage/main/pre_update | 0.386 | 0.457 | 0.83 | 499.5 |
| render/present/ms | 0.352 | 0.416 | 1.75 | 456.0 |
| bigspace/total/ms | 0.350 | 0.638 | 1.39 | 453.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.323 | 0.460 | 0.74 | 417.9 |
| stage/main/first | 0.312 | 0.373 | 2.00 | 404.1 |
| stage/render/prepare_flush | 0.268 | 0.371 | 0.82 | 346.2 |
| stage/render/prepare_batch_phases | 0.258 | 0.359 | 0.55 | 333.8 |
| stage/render/prepare_bind_groups | 0.254 | 0.353 | 0.64 | 328.5 |
| stage/main/last | 0.246 | 0.337 | 0.86 | 318.5 |
| stage/render/prepare_meshes | 0.207 | 0.254 | 1.14 | 267.2 |
| stage/render/extract_commands | 0.190 | 0.223 | 0.34 | 246.3 |
| controller/feed/ms | 0.183 | 0.226 | 0.66 | 236.2 |
| stage/render/create_views | 0.158 | 0.190 | 0.36 | 203.8 |
| stream/drive/ms | 0.132 | 0.154 | 0.63 | 171.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.122 | 0.187 | 0.39 | 157.6 |
| bigspace/high_precision/ms | 0.121 | 0.296 | 1.21 | 156.4 |
| stream/tick/ms | 0.106 | 0.126 | 0.51 | 136.7 |
| controller/move/ms | 0.102 | 0.195 | 0.37 | 132.1 |
| stage/render/queue | 0.101 | 0.131 | 3.04 | 130.5 |
| bigspace/low_precision/ms | 0.094 | 0.234 | 1.09 | 121.7 |
| stage/render/specialize | 0.076 | 0.107 | 3.70 | 99.0 |
| anim/evaluate/ms | 0.076 | 0.156 | 0.92 | 98.8 |
| anim/evaluate_cpu/ms | 0.061 | 0.074 | 0.48 | 78.7 |
| encode/resolve_diagnostics/sum | 0.057 | 0.080 | 0.49 | 73.6 |
| controller/write/ms | 0.053 | 0.069 | 0.20 | 69.0 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.081 | 0.18 | 59.7 |
| pipelines/driver_ms | 0.035 | 0.000 | 3.96 | 45.0 |
| terrain/update/ms | 0.035 | 0.046 | 0.09 | 44.9 |
| physics/step/ms | 0.030 | 0.050 | 0.09 | 38.6 |
| stage/render/phase_sort | 0.028 | 0.046 | 0.11 | 36.8 |
| stage/render/cleanup | 0.025 | 0.033 | 0.26 | 32.1 |
| encode/camera_driver_span_begin/sum | 0.025 | 0.037 | 0.20 | 32.1 |
| weather/map/ms | 0.023 | 0.031 | 0.06 | 30.1 |
| encode/camera_driver_span_end/sum | 0.022 | 0.035 | 0.10 | 28.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.08 | 13.9 |
| script/update/ms | 0.006 | 0.008 | 0.02 | 8.1 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 6.6 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 5.6 |
| far/select/ms | 0.004 | 0.004 | 0.42 | 5.4 |
| controller/avoid/ms | 0.004 | 0.006 | 0.02 | 5.1 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 3.9 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.49 | 2.7 |
| stream/promote/ms | 0.001 | 0.001 | 0.38 | 1.7 |
| character/finish/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.01 | 1.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.06 | 0.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 0.8 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.01 | 0.8 |
| terrain/select/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.4 |
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
| crowd/joints_extracted/count | count | 136 | 181 | 149 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.9 k | 37.9 k | 37.9 k |
| encode//max | ms | 0.56 | 1.78 | 0.41 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.20 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.10 | 0.02 |
| encode/finish_critical | ms | 0.88 | 2.07 | 0.73 |
| encode/pass:early prepass/max | ms | 0.75 | 1.40 | 0.60 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.71 | 1.33 | 0.73 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.39 | 0.13 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.31 | 0.74 | 0.37 |
| encode/pass:shadow_directional_light/max | ms | 0.72 | 2.07 | 0.72 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.49 | 0.04 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 10.81 | 17.43 | 10.75 |
| gpu/timed/ms | ms | 10.81 | 12.26 | 10.75 |
| groundcover/batches/count | count | 0 | 0 | 0 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1274 | 1274 | 1274 |
| groundcover/slots/capacity | count | 0 | 0 | 0 |
| groundcover/slots/capacity_bytes | bytes | 0 | 0 | 0 |
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
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d/bins | count | 248 | 248 | 248 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 563 | 563 | 563 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 319 | 319 | 319 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 248 | 248 | 248 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 563 | 563 | 563 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 319 | 319 | 319 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 23 | 23 | 23 |
| phases/opaque3d/bins | count | 875 | 875 | 875 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2784 | 2784 | 2784 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/opaque3d_prepass/bins | count | 875 | 875 | 875 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2784 | 2784 | 2784 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 4 | 41 | 4 |
| phases/shadow/bins | count | 896 | 1509 | 896 |
| phases/shadow/instance_batches | count | 4 | 12 | 4 |
| phases/shadow/instance_draws | count | 5861 | 17.6 k | 5861 |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| phases/shadow/items | count | 3202 | 8161 | 3202 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 27 | 34 | 27 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 332 | 332 | 332 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 8 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 24.36 | 35.87 | 19.23 |
| process/mem_usage | GiB | 3.00 | 3.06 | 3.06 |
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
| skin/blocks_changed/count | count | 6 | 7 | 6 |
| skin/blocks_skipped/count | count | 175 | 178 | 175 |
| skin/blocks_walked/count | count | 6 | 7 | 6 |
| skin/joint_bytes_written/bytes | bytes | 9472 | 12.5 k | 10.5 k |
| skin/joints_written/count | count | 148 | 196 | 164 |
| skin/upload_writes/count | count | 5 | 6 | 5 |
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
| system/cpu_usage | % | 29.28 | 38.63 | 25.89 |
| system/mem_usage | % | 49.98 | 50.19 | 50.19 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2548 | 2548 | 2548 |
| textures/images | count | 2000 | 2000 | 2000 |
| views/all/batch_sets | count | 64 | 101 | 64 |
| views/all/bins | count | 3142 | 3755 | 3142 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 12 | 20 | 12 |
| views/all/instance_draws | count | 17.6 k | 29.3 k | 17.6 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 147.5 k |
| views/all/items | count | 9408 | 14.4 k | 9408 |
| views/all/pipelines | count | 75 | 82 | 75 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 153.9 k | 260.4 k | 153.9 k |
| views/cascade/batch_sets | count | 4 | 41 | 4 |
| views/cascade/bins | count | 896 | 1509 | 896 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 4 |
| views/cascade/instance_draws | count | 5861 | 17.6 k | 5861 |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| views/cascade/items | count | 3202 | 8161 | 3202 |
| views/cascade/pipelines | count | 27 | 34 | 27 |
| views/cascade/work_items | count | 49.2 k | 155.6 k | 49.2 k |
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
| views/main/batch_sets | count | 60 | 60 | 60 |
| views/main/bins | count | 2246 | 2246 | 2246 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 11.7 k | 11.7 k | 11.7 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6206 | 6206 | 6206 |
| views/main/pipelines | count | 48 | 48 | 48 |
| views/main/work_items | count | 104.8 k | 104.8 k | 104.8 k |
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
| anim/events/count | count | 0.13 | 4 | 165 |
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
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 28.56 | 31 | 37.0 k |
| encode/bind_walk/init_textures | count | 7196 | 12.1 k | 9.31 M |
| encode/bind_walk/init_textures_skipped | count | 41.1 k | 46.9 k | 53.25 M |
| encode/bind_walk/inits | count | 504 | 608 | 652.1 k |
| encode/bind_walk/inits_skipped | count | 127 | 134 | 163.7 k |
| encode/bind_walk/merge_textures | count | 7131 | 12.0 k | 9.23 M |
| encode/bind_walk/merge_textures_skipped | count | 6444 | 12.2 k | 8.34 M |
| encode/bind_walk/merges | count | 91.27 | 111 | 118.1 k |
| encode/bind_walk/merges_skipped | count | 112 | 126 | 144.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1294 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1294 |
| encode/pass:early prepass/count | count | 1 | 1 | 1294 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1294 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1294 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1294 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1294 |
| encode/queue/command_buffers | count | 38.56 | 41 | 49.9 k |
| encode/queue/submits | count | 2 | 2 | 2588 |
| encode/queue/write_buffer/bytes | bytes | 1.87 M | 3.36 M | 2.42 G |
| encode/queue/write_buffer/calls | count | 234 | 305 | 302.6 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 42.24 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 11.6 k |
| encode/queue/write_texture/bytes | bytes | 20.4 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1294 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.70 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 25 |
| pipelines/created | count | 0.02 | 1 | 25 |
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

## Hitches (> 29.2 ms; slowest 30, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.59 | 2586.4 | main/busy/ms 2553, stage/main/post_update 2437, stage/main/update 102, surface/advance_cpu_ms 70.34, stage/main/pre_update 9.54, stage/main/last 1.13, surface/cell_upload_ms 0.75, stage/main/first 0.66, stage/main/fixed_main_loop 0.37, stream/drive/ms 0.34, weather/map/ms 0.16, stream/tick/ms 0.10 |
| warm-up | 3.36 | 775.4 | main/busy/ms 2599, stage/main/post_update 2470, render/thread/ms 713, render/busy/ms 553, stage/render/prepare_assets 518, stage/render/prepare_views 155, render/acquire/ms 155, stage/main/update 109, surface/advance_cpu_ms 70.34, stage/render/render 21.02, render/graph/ms 15.19, stage/render/camera_driver 12.81 |
| warm-up | 3.46 | 102.5 | render/thread/ms 813, render/busy/ms 580, stage/render/prepare_assets 525, stage/render/prepare_views 156, render/acquire/ms 155, main/busy/ms 146, stage/render/render 103, render/present/ms 77.08, stage/main/post_update 60.34, stage/main/last 59.65, terrain/update/ms 25.17, render/graph/ms 23.97 |
| warm-up | 3.52 | 56.2 | pipelines/compile_ms 335, pipelines/front_ms 229, render/thread/ms 155, main/busy/ms 135, pipelines/driver_ms 105, stage/render/render 96.30, render/busy/ms 80.31, render/present/ms 74.00, stage/main/last 59.56, stage/main/post_update 51.59, stage/render/prepare_assets 38.35, terrain/update/ms 35.50 |
| warm-up | 3.69 | 172.5 | pipelines/wait_ms 5404, pipelines/compile_ms 2292, pipelines/front_ms 2044, pipelines/driver_ms 248, render/thread/ms 225, render/busy/ms 222, main/busy/ms 178, stage/main/update 76.51, stage/render/prepare_flush 72.92, stage/render/prepare_assets 49.75, stage/main/post_update 41.49, stage/render/prepare_write_phase_buffers 37.03 |
| warm-up | 3.78 | 83.5 | pipelines/wait_ms 5656, pipelines/compile_ms 2782, pipelines/front_ms 2602, render/thread/ms 251, render/busy/ms 249, pipelines/driver_ms 180, main/busy/ms 175, stage/render/prepare_assets 87.40, stage/main/update 76.85, stage/render/prepare_flush 72.76, stage/render/prepare_write_phase_buffers 37.23, stage/main/post_update 33.16 |
| warm-up | 3.82 | 39.0 | pipelines/compile_ms 1351, pipelines/front_ms 1295, pipelines/wait_ms 360, render/thread/ms 116, render/busy/ms 115, stage/render/prepare_assets 93.81, main/busy/ms 67.64, pipelines/driver_ms 56.02, stage/main/post_update 22.27, stage/main/pre_update 19.88, terrain/update/ms 15.10, stage/render/render 13.45 |
| warm-up | 3.88 | 44.3 | pipelines/compile_ms 149, pipelines/front_ms 134, render/thread/ms 64.83, render/busy/ms 62.84, pipelines/wait_ms 47.27, main/busy/ms 33.21, encode//sum 21.18, stage/render/render 19.16, render/graph/ms 17.06, pipelines/driver_ms 15.45, stage/render/prepare_assets 14.65, stage/main/post_update 14.60 |
| warm-up | 3.96 | 74.3 | pipelines/compile_ms 818, pipelines/front_ms 767, pipelines/wait_ms 126, render/thread/ms 116, render/busy/ms 113, pipelines/driver_ms 50.85, main/busy/ms 41.08, stage/render/prepare_resources 30.51, stage/render/render 25.45, encode//sum 23.17, render/graph/ms 21.99, stage/render/prepare_flush 16.52 |
| warm-up | 4.05 | 92.5 | pipelines/compile_ms 2634, pipelines/front_ms 2408, pipelines/wait_ms 283, pipelines/driver_ms 226, render/thread/ms 159, render/busy/ms 157, main/busy/ms 60.78, stage/render/prepare_assets 54.32, stage/render/render 42.92, render/graph/ms 39.13, stage/render/camera_driver 27.21, stage/render/prepare_resources 21.39 |
| warm-up | 4.15 | 96.8 | pipelines/compile_ms 2329, pipelines/front_ms 2136, pipelines/wait_ms 206, pipelines/driver_ms 192, render/thread/ms 182, render/busy/ms 180, stage/render/prepare_assets 130, main/busy/ms 42.83, stage/render/render 41.13, render/graph/ms 38.66, stage/render/camera_driver 26.88, stage/render/graph_submit 11.48 |
| warm-up | 4.22 | 69.9 | pipelines/compile_ms 991, pipelines/front_ms 963, render/thread/ms 163, render/busy/ms 162, stage/render/prepare_assets 142, pipelines/wait_ms 43.19, main/busy/ms 35.70, pipelines/driver_ms 27.76, stage/render/render 15.28, render/graph/ms 13.34, stage/main/fixed_main_loop 11.57, stage/main/update 11.04 |
| warm-up | 4.36 | 48.8 | pipelines/compile_ms 946, pipelines/front_ms 913, render/thread/ms 56.46, render/busy/ms 53.91, main/busy/ms 44.46, pipelines/driver_ms 33.04, stage/render/prepare_assets 32.04, stage/main/update 23.88, stage/render/render 16.10, render/graph/ms 12.61, stage/main/post_update 10.60, encode//sum 9.78 |
| warm-up | 4.43 | 63.2 | pipelines/compile_ms 1292, pipelines/front_ms 1247, render/thread/ms 99.18, main/busy/ms 96.88, render/busy/ms 96.29, pipelines/wait_ms 49.32, stage/main/update 46.11, stage/render/render 45.87, pipelines/driver_ms 44.20, render/graph/ms 42.21, stage/render/prepare_assets 38.63, stage/render/graph_submit 29.14 |
| warm-up | 4.54 | 110.4 | pipelines/compile_ms 404, pipelines/front_ms 390, render/thread/ms 161, render/busy/ms 158, stage/render/prepare_assets 101, pipelines/wait_ms 84.55, main/busy/ms 82.49, stage/render/render 43.63, render/graph/ms 40.90, stage/main/update 37.41, stage/render/graph_submit 29.01, stage/main/post_update 18.64 |
| warm-up | 4.57 | 37.4 | pipelines/compile_ms 1030, pipelines/front_ms 990, render/thread/ms 136, render/busy/ms 135, stage/render/prepare_assets 108, pipelines/wait_ms 53.86, main/busy/ms 41.81, pipelines/driver_ms 39.85, stage/main/post_update 19.92, stage/main/update 16.01, stage/render/render 12.23, render/graph/ms 10.10 |
| warm-up | 4.61 | 40.0 | pipelines/compile_ms 1648, pipelines/front_ms 1591, pipelines/wait_ms 110, pipelines/driver_ms 57.06, render/thread/ms 55.80, render/busy/ms 54.35, main/busy/ms 37.85, stage/render/prepare_assets 29.09, stage/main/post_update 19.03, pipelines/lock_ms 18.11, stage/main/update 13.10, stage/render/render 11.64 |
| warm-up | 4.65 | 39.0 | pipelines/compile_ms 736, pipelines/front_ms 714, pipelines/wait_ms 94.06, render/thread/ms 57.64, render/busy/ms 55.73, main/busy/ms 41.53, pipelines/driver_ms 21.71, stage/main/update 19.72, stage/main/post_update 14.91, stage/render/prepare_assets 13.57, stage/render/render 12.95, render/graph/ms 10.71 |
| warm-up | 4.72 | 66.6 | pipelines/compile_ms 240, pipelines/front_ms 233, render/thread/ms 101, render/busy/ms 98.76, main/busy/ms 86.44, stage/main/update 56.97, stage/main/post_update 22.22, stage/render/prepare_write_phase_buffers 20.68, character/finish/ms 20.60, stage/render/prepare_batch_phases 15.63, stage/render/render 12.57, stage/render/prepare_assets 12.25 |
| warm-up | 4.79 | 74.3 | pipelines/compile_ms 1414, pipelines/front_ms 1365, render/thread/ms 135, render/busy/ms 133, main/busy/ms 128, pipelines/wait_ms 100, stage/main/update 99.85, pipelines/driver_ms 49.05, stage/render/prepare_assets 48.18, stage/render/prepare_write_phase_buffers 25.54, character/finish/ms 24.53, stage/main/post_update 19.15 |
| warm-up | 4.88 | 87.5 | pipelines/compile_ms 2606, pipelines/front_ms 2531, render/thread/ms 157, render/busy/ms 155, pipelines/wait_ms 124, main/busy/ms 96.69, stage/render/prepare_assets 84.37, pipelines/driver_ms 75.13, stage/main/update 67.45, stage/render/prepare_resources 28.98, stream/drive/ms 18.63, stage/main/post_update 17.96 |
| warm-up | 4.92 | 38.3 | pipelines/compile_ms 1455, pipelines/front_ms 1418, render/thread/ms 122, render/busy/ms 120, main/busy/ms 57.97, stage/render/prepare_assets 48.35, pipelines/driver_ms 36.18, stage/main/update 32.82, stage/render/render 26.90, pipelines/wait_ms 26.22, stage/render/prepare_resources 26.19, render/graph/ms 23.61 |
| warm-up | 5.07 | 34.1 | render/thread/ms 60.21, render/busy/ms 58.54, main/busy/ms 39.67, pipelines/compile_ms 30.37, pipelines/front_ms 29.36, stage/main/update 23.76, stage/render/prepare_assets 18.35, stage/render/render 12.59, render/graph/ms 10.54, stage/render/prepare_resources 10.04, encode//sum 8.39, stage/main/post_update 8.34 |
| warm-up | 5.22 | 30.5 | pipelines/compile_ms 65.97, pipelines/front_ms 63.59, main/busy/ms 55.55, render/thread/ms 43.81, render/busy/ms 42.16, stage/main/update 31.37, stage/render/render 20.04, render/graph/ms 17.99, stage/main/last 11.61, encode//sum 8.29, stage/main/post_update 7.64, stage/render/graph_submit 6.89 |
| warm-up | 5.27 | 46.9 | pipelines/compile_ms 245, pipelines/front_ms 238, render/thread/ms 61.98, render/busy/ms 60.54, main/busy/ms 54.50, stage/main/update 17.93, stage/render/prepare_resources 13.82, stage/render/render 11.61, stage/main/last 11.54, stage/main/fixed_main_loop 10.64, stage/render/prepare_assets 10.53, render/graph/ms 9.81 |
| warm-up | 5.38 | 67.2 | pipelines/compile_ms 963, pipelines/front_ms 935, main/busy/ms 75.85, render/thread/ms 75.02, render/busy/ms 73.38, stage/main/update 53.26, stage/render/prepare_write_phase_buffers 32.79, pipelines/driver_ms 27.72, stage/render/render 16.06, stage/main/post_update 15.44, render/graph/ms 13.69, stage/render/graph_submit 9.62 |
| warm-up | 5.41 | 31.4 | pipelines/compile_ms 1810, pipelines/front_ms 1764, render/thread/ms 85.49, render/busy/ms 83.96, main/busy/ms 73.07, stage/main/update 53.77, pipelines/driver_ms 45.60, stage/render/prepare_write_phase_buffers 31.49, stage/render/prepare_assets 24.62, pipelines/wait_ms 18.53, stage/render/render 12.57, render/graph/ms 10.47 |
| warm-up | 5.62 | 39.8 | render/thread/ms 60.86, render/busy/ms 58.41, main/busy/ms 43.70, stage/render/prepare_assets 20.89, stage/render/render 20.14, stage/main/update 17.20, stage/main/post_update 16.93, render/graph/ms 16.91, stage/render/graph_submit 11.57, encode//sum 11.10, encode/submit 8.43, stage/render/prepare_meshes 5.65 |
| warm-up | 5.66 | 36.1 | render/thread/ms 70.12, render/busy/ms 68.30, main/busy/ms 48.84, stage/render/prepare_assets 35.54, stage/main/update 26.07, stage/render/render 15.48, render/graph/ms 12.95, stage/main/post_update 11.49, encode//sum 10.55, stage/render/graph_submit 9.27, encode/submit 6.26, stage/main/fixed_main_loop 5.85 |
| warm-up | 5.72 | 56.8 | pipelines/compile_ms 174, pipelines/front_ms 170, render/thread/ms 89.08, render/busy/ms 87.87, stage/render/prepare_assets 38.63, main/busy/ms 37.15, stage/render/create_views 25.18, stage/main/update 21.53, stage/render/render 11.04, render/graph/ms 9.48, encode//sum 8.21, stage/render/graph_submit 7.16 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\gc_p0b\balmora@2560x1440_arm=on-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (285 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 92.7 W · 75 °C (max 78) · VRAM 5347 MiB · throttled 100% of samples

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
