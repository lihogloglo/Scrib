# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); groundcover.density_scale=0 (command line: --set); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1194 frames over 15.0 s · **median 12.65 ms (79 fps)** · mean 12.56 · p95 14.12 · p99 14.51 (1% low 69 fps) · worst 16.82 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 312 frames over 9.2 s, worst 2542.8 ms
- hitches (> 31.6 ms): **0** measured, 25 in warm-up
- timed GPU passes: 11.06 ms of a 12.56 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.65 ms, limit 11 ms
- pass worst frame 16.82 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.264 | 3.264 | 3.336 | 1.00 | 0.101 |
| dlss_super_resolution | 2.023 | 2.023 | 2.046 | 1.00 | 0.181 |
| sun_visibility | 0.781 | 0.781 | 0.808 | 1.00 | 0.007 |
| gpu_instance_materialization | 0.661 | 0.661 | 0.898 | 1.00 | 0.009 |
| early prepass | 0.627 | 0.627 | 0.633 | 1.00 | 0.128 |
| main_transmissive_pass_3d | 0.397 | 0.397 | 0.408 | 1.00 | 0.008 |
| atmosphere_luts | 0.371 | 0.371 | 0.380 | 1.00 | 0.003 |
| bloom | 0.314 | 0.314 | 0.330 | 1.00 | 0.059 |
| atmos_froxel_inject | 0.310 | 0.310 | 0.308 | 1.00 | 0.001 |
| water_velocity | 0.306 | 0.306 | 0.310 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.229 | 0.229 | 0.262 | 1.00 | 0.004 |
| water_sspr | 0.217 | 0.217 | 0.210 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.196 | 1.00 | 0.007 |
| clouds_trace | 0.151 | 0.151 | 0.169 | 1.00 | 0.001 |
| water_fft | 0.135 | 0.135 | 0.139 | 1.00 | 0.002 |
| uniform_allocation | 0.121 | 0.121 | 0.154 | 1.00 | 0.022 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| upscaling | 0.114 | 0.114 | 0.119 | 1.00 | 0.004 |
| bin_unpacking | 0.098 | 0.098 | 0.123 | 1.00 | 0.015 |
| tonemapping | 0.086 | 0.086 | 0.089 | 1.00 | 0.003 |
| range_unpacking | 0.082 | 0.082 | 0.094 | 1.00 | 0.003 |
| clustering | 0.063 | 0.063 | 0.074 | 1.00 | 0.021 |
| render_sky | 0.062 | 0.062 | 0.063 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.411 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.220 | 0.230 | 0.12 | 0.004 |
| per_view_shadow_pass | 0.027 | 0.027 | 0.053 | 1.00 | 0.007 |
| water_snapshot | 0.023 | 0.023 | 0.025 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.023 | 1.00 | 0.195 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.002 |
| surface_exposure_far | 0.011 | 0.036 | 0.039 | 0.31 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| surface_exposure | 0.010 | 0.036 | 0.036 | 0.28 | 0.001 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.010 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.577 | 13.171 | 15.69 | 13823.0 |
| render/busy/ms | 7.718 | 8.812 | 15.19 | 9215.1 |
| main/busy/ms | 6.672 | 7.849 | 9.73 | 7966.3 |
| encode//sum | 5.326 | 6.840 | 10.24 | 6359.6 |
| stage/render/render | 5.129 | 5.914 | 6.83 | 6124.0 |
| render/graph/ms | 4.536 | 5.304 | 6.20 | 5416.5 |
| stage/main/post_update | 3.731 | 4.747 | 6.79 | 4455.4 |
| stage/render/prepare_views | 3.549 | 5.282 | 6.06 | 4237.4 |
| render/acquire/ms | 3.500 | 5.230 | 6.01 | 4179.0 |
| stage/render/graph_submit | 3.277 | 3.982 | 4.87 | 3912.8 |
| encode/submit | 1.976 | 2.335 | 3.01 | 2359.2 |
| pipelines/compile_ms | 1.629 | 0.000 | 118.70 | 1945.2 |
| pipelines/front_ms | 1.582 | 0.000 | 115.95 | 1888.8 |
| encode/finish_wall | 1.328 | 1.788 | 2.40 | 1585.8 |
| stage/main/update | 1.321 | 1.528 | 2.22 | 1577.5 |
| stage/render/camera_driver | 1.176 | 1.357 | 1.66 | 1404.1 |
| encode/pass:early prepass/sum | 0.865 | 1.176 | 1.61 | 1032.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.831 | 1.144 | 1.50 | 992.1 |
| encode/pass:shadow_directional_light/sum | 0.817 | 1.418 | 2.05 | 975.9 |
| stage/render/prepare_write_phase_buffers | 0.516 | 0.735 | 0.94 | 615.7 |
| stage/main/fixed_main_loop | 0.506 | 0.767 | 1.37 | 604.4 |
| stage/render/prepare_resources | 0.427 | 0.504 | 0.61 | 510.0 |
| stage/render/prepare_assets | 0.402 | 0.490 | 1.56 | 480.3 |
| stage/main/pre_update | 0.391 | 0.466 | 1.15 | 467.0 |
| render/present/ms | 0.359 | 0.425 | 0.60 | 428.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.351 | 0.504 | 0.74 | 419.3 |
| bigspace/total/ms | 0.328 | 0.578 | 1.78 | 392.0 |
| stage/main/first | 0.311 | 0.374 | 1.70 | 371.6 |
| stage/render/prepare_flush | 0.273 | 0.376 | 0.68 | 325.7 |
| stage/render/prepare_bind_groups | 0.272 | 0.391 | 0.60 | 324.6 |
| stage/render/prepare_batch_phases | 0.268 | 0.366 | 0.54 | 319.7 |
| stage/main/last | 0.235 | 0.326 | 0.75 | 280.3 |
| stage/render/prepare_meshes | 0.226 | 0.278 | 1.11 | 269.8 |
| stage/render/extract_commands | 0.199 | 0.233 | 0.32 | 237.1 |
| controller/feed/ms | 0.181 | 0.224 | 0.58 | 215.8 |
| stage/render/create_views | 0.160 | 0.199 | 0.41 | 191.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.137 | 0.207 | 0.34 | 164.2 |
| stream/drive/ms | 0.136 | 0.160 | 0.28 | 162.4 |
| controller/move/ms | 0.124 | 0.273 | 0.43 | 147.9 |
| bigspace/high_precision/ms | 0.110 | 0.236 | 1.06 | 131.4 |
| stream/tick/ms | 0.109 | 0.127 | 0.25 | 129.8 |
| stage/render/queue | 0.102 | 0.147 | 2.35 | 121.8 |
| bigspace/low_precision/ms | 0.090 | 0.218 | 1.12 | 107.2 |
| stage/render/specialize | 0.080 | 0.112 | 3.25 | 95.6 |
| anim/evaluate/ms | 0.068 | 0.090 | 0.96 | 81.3 |
| anim/evaluate_cpu/ms | 0.060 | 0.074 | 0.48 | 72.0 |
| encode/resolve_diagnostics/sum | 0.060 | 0.088 | 0.26 | 71.9 |
| controller/write/ms | 0.052 | 0.067 | 0.18 | 61.8 |
| pipelines/driver_ms | 0.047 | 0.000 | 4.17 | 56.4 |
| stage/render/prepare_collect_phase_buffers | 0.045 | 0.078 | 0.26 | 53.2 |
| terrain/update/ms | 0.035 | 0.047 | 0.08 | 41.6 |
| physics/step/ms | 0.034 | 0.052 | 0.11 | 40.4 |
| stage/render/phase_sort | 0.028 | 0.048 | 0.10 | 33.1 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.042 | 0.16 | 32.1 |
| stage/render/cleanup | 0.025 | 0.032 | 0.27 | 29.4 |
| weather/map/ms | 0.024 | 0.033 | 0.10 | 29.1 |
| encode/camera_driver_span_end/sum | 0.023 | 0.036 | 0.12 | 27.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.02 | 12.8 |
| script/update/ms | 0.007 | 0.008 | 0.02 | 7.9 |
| script/input/ms | 0.005 | 0.006 | 0.07 | 6.2 |
| anim/apply/ms | 0.005 | 0.006 | 0.38 | 5.8 |
| controller/avoid/ms | 0.004 | 0.006 | 0.02 | 5.0 |
| far/select/ms | 0.004 | 0.004 | 0.02 | 4.3 |
| script/ui/ms | 0.003 | 0.004 | 0.04 | 3.5 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.09 | 2.3 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.02 | 1.5 |
| stream/promote/ms | 0.001 | 0.001 | 0.12 | 1.4 |
| character/finish/ms | 0.001 | 0.002 | 0.02 | 1.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.01 | 0.8 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 0.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| terrain/select/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
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
| crowd/joints_extracted/count | count | 149 | 181 | 149 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 38.0 k | 38.0 k | 38.0 k |
| encode//max | ms | 0.62 | 1.82 | 0.97 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.16 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.12 | 0.01 |
| encode/finish_critical | ms | 1.00 | 2.05 | 1.17 |
| encode/pass:early prepass/max | ms | 0.85 | 1.61 | 0.79 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.83 | 1.50 | 1.17 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.13 | 0.34 | 0.13 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.34 | 0.74 | 0.38 |
| encode/pass:shadow_directional_light/max | ms | 0.75 | 2.05 | 1.01 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.26 | 0.07 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 11.63 | 15.19 | 11.49 |
| gpu/timed/ms | ms | 11.63 | 12.81 | 11.49 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 793 | 793 | 793 |
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
| precache/throttled | count | 0 | 14 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 53 |
| preprocess/bind_groups_created | count | 0 | 62 | 46 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 7 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 26.01 | 39.23 | 31.70 |
| process/mem_usage | GiB | 3.03 | 3.10 | 3.10 |
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
| skin/joint_bytes_written/bytes | bytes | 10.5 k | 12.5 k | 10.5 k |
| skin/joints_written/count | count | 164 | 196 | 164 |
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
| system/cpu_usage | % | 29.67 | 38.09 | 31.39 |
| system/mem_usage | % | 55.79 | 56.02 | 56.02 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2548 | 2548 | 2548 |
| textures/images | count | 2000 | 2000 | 2000 |
| views/all/batch_sets | count | 105 | 152 | 145 |
| views/all/bins | count | 3193 | 3806 | 3193 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 53 | 71 | 71 |
| views/all/instance_draws | count | 17.6 k | 29.4 k | 29.4 k |
| views/all/instance_slots | count | 1.50 M | 1.66 M | 1.66 M |
| views/all/items | count | 9459 | 14.4 k | 9459 |
| views/all/pipelines | count | 89 | 96 | 89 |
| views/all/pipelines_distinct | count | 73 | 73 | 73 |
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
| anim/events/count | count | 0.12 | 3 | 149 |
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
| encode//count | count | 28.56 | 31 | 34.1 k |
| encode/bind_walk/init_textures | count | 7457 | 12.4 k | 8.90 M |
| encode/bind_walk/init_textures_skipped | count | 53.2 k | 60.4 k | 63.52 M |
| encode/bind_walk/inits | count | 652 | 779 | 779.0 k |
| encode/bind_walk/inits_skipped | count | 135 | 145 | 161.5 k |
| encode/bind_walk/merge_textures | count | 7393 | 12.3 k | 8.83 M |
| encode/bind_walk/merge_textures_skipped | count | 14.6 k | 21.9 k | 17.48 M |
| encode/bind_walk/merges | count | 135 | 162 | 160.9 k |
| encode/bind_walk/merges_skipped | count | 118 | 135 | 141.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1194 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1194 |
| encode/pass:early prepass/count | count | 1 | 1 | 1194 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1194 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1194 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1194 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1194 |
| encode/queue/command_buffers | count | 38.56 | 41 | 46.0 k |
| encode/queue/submits | count | 2 | 2 | 2388 |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 3.38 M | 2.26 G |
| encode/queue/write_buffer/calls | count | 252 | 324 | 301.3 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 38.97 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 10.7 k |
| encode/queue/write_texture/bytes | bytes | 22.1 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1194 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.75 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.03 | 1 | 30 |
| pipelines/created | count | 0.03 | 1 | 30 |
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

## Hitches (> 31.6 ms; slowest 25, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2542.8 | main/busy/ms 2509, stage/main/post_update 2390, stage/main/update 105, surface/advance_cpu_ms 71.87, stage/main/pre_update 9.62, stage/main/last 1.07, surface/cell_upload_ms 0.80, stage/main/first 0.54, stage/main/fixed_main_loop 0.47, stream/drive/ms 0.32, weather/map/ms 0.19, bigspace/total/ms 0.09 |
| warm-up | 3.37 | 824.6 | main/busy/ms 2555, stage/main/post_update 2424, render/thread/ms 765, render/busy/ms 607, stage/render/prepare_assets 570, stage/render/prepare_views 155, render/acquire/ms 155, stage/main/update 112, surface/advance_cpu_ms 71.87, stage/render/render 22.87, render/graph/ms 18.43, stage/render/camera_driver 16.14 |
| warm-up | 3.51 | 146.0 | render/thread/ms 909, render/busy/ms 667, stage/render/prepare_assets 578, pipelines/compile_ms 157, stage/render/prepare_views 156, render/acquire/ms 155, stage/render/render 146, main/busy/ms 121, pipelines/driver_ms 90.50, render/present/ms 86.22, stage/main/post_update 84.45, pipelines/front_ms 66.01 |
| warm-up | 3.62 | 106.1 | pipelines/compile_ms 2116, pipelines/front_ms 1125, pipelines/driver_ms 991, pipelines/wait_ms 440, render/thread/ms 249, stage/render/render 181, render/busy/ms 164, main/busy/ms 106, pipelines/lock_ms 95.59, render/graph/ms 95.02, render/present/ms 83.92, stage/main/post_update 63.41 |
| warm-up | 3.76 | 140.6 | pipelines/wait_ms 7573, pipelines/compile_ms 3985, pipelines/front_ms 2932, pipelines/driver_ms 1053, render/thread/ms 243, render/busy/ms 241, main/busy/ms 125, pipelines/lock_ms 114, stage/main/update 81.88, stage/render/render 78.98, render/graph/ms 76.25, stage/render/camera_driver 61.94 |
| warm-up | 3.91 | 149.3 | pipelines/wait_ms 10.4 k, pipelines/compile_ms 3376, pipelines/front_ms 3133, render/thread/ms 284, render/busy/ms 282, pipelines/driver_ms 243, main/busy/ms 141, stage/render/prepare_assets 101, stage/main/update 77.10, stage/render/prepare_meshes 49.55, stage/main/post_update 45.78, pipelines/lock_ms 41.51 |
| warm-up | 3.95 | 40.4 | pipelines/wait_ms 3337, pipelines/compile_ms 1824, pipelines/front_ms 1710, render/thread/ms 183, render/busy/ms 181, pipelines/driver_ms 114, stage/render/prepare_assets 81.56, main/busy/ms 74.47, stage/render/prepare_meshes 49.63, stage/main/post_update 43.37, stage/render/render 36.12, render/graph/ms 33.42 |
| warm-up | 4.08 | 50.7 | pipelines/compile_ms 320, pipelines/front_ms 298, pipelines/wait_ms 218, render/thread/ms 78.77, render/busy/ms 76.10, main/busy/ms 50.48, stage/render/prepare_assets 36.97, pipelines/driver_ms 22.12, stage/main/post_update 20.86, stage/main/update 20.12, stage/render/render 19.45, render/graph/ms 16.16 |
| warm-up | 4.14 | 58.9 | pipelines/compile_ms 1368, pipelines/front_ms 1282, pipelines/wait_ms 269, render/thread/ms 104, render/busy/ms 102, pipelines/driver_ms 86.94, stage/render/prepare_assets 59.81, main/busy/ms 44.98, stage/render/render 28.61, render/graph/ms 25.38, stage/main/update 22.07, stage/render/graph_submit 21.28 |
| warm-up | 4.18 | 42.2 | pipelines/compile_ms 1193, pipelines/front_ms 1121, render/thread/ms 96.21, render/busy/ms 93.87, pipelines/driver_ms 71.77, stage/render/prepare_assets 55.87, pipelines/wait_ms 53.44, main/busy/ms 40.97, stage/render/render 27.32, render/graph/ms 24.15, stage/render/graph_submit 19.63, stage/main/post_update 15.94 |
| warm-up | 4.27 | 81.1 | pipelines/compile_ms 646, pipelines/front_ms 628, render/thread/ms 117, render/busy/ms 115, stage/render/prepare_assets 92.94, main/busy/ms 40.57, pipelines/driver_ms 18.23, stage/main/post_update 15.09, stage/render/render 15.02, render/graph/ms 12.60, stage/main/update 10.84, encode//sum 8.87 |
| warm-up | 4.48 | 64.9 | pipelines/compile_ms 138, pipelines/front_ms 132, main/busy/ms 94.04, render/thread/ms 91.76, render/busy/ms 90.01, stage/main/update 64.50, stage/render/render 39.71, render/graph/ms 36.92, stage/render/prepare_assets 34.70, stage/render/camera_driver 22.99, pipelines/wait_ms 16.79, pipelines/lock_ms 15.94 |
| warm-up | 4.53 | 55.7 | pipelines/compile_ms 167, pipelines/front_ms 159, render/thread/ms 113, render/busy/ms 111, main/busy/ms 93.96, stage/main/update 60.42, stage/render/prepare_assets 51.03, stage/render/render 39.13, render/graph/ms 36.51, pipelines/wait_ms 35.15, stage/render/camera_driver 22.37, stage/main/post_update 19.78 |
| warm-up | 4.57 | 39.2 | pipelines/compile_ms 137, pipelines/front_ms 130, render/thread/ms 87.92, render/busy/ms 86.46, stage/render/prepare_assets 55.58, main/busy/ms 51.98, stage/main/post_update 26.08, stage/main/update 19.95, pipelines/wait_ms 19.28, stage/render/render 12.54, render/graph/ms 10.46, encode//sum 8.11 |
| warm-up | 4.65 | 79.3 | pipelines/compile_ms 1537, pipelines/front_ms 1492, pipelines/wait_ms 200, render/thread/ms 98.78, render/busy/ms 97.43, main/busy/ms 45.62, pipelines/driver_ms 44.81, stage/render/prepare_resources 29.71, stage/render/prepare_assets 24.67, stage/main/post_update 22.93, pipelines/lock_ms 18.91, stage/render/prepare_flush 18.26 |
| warm-up | 4.70 | 34.5 | pipelines/compile_ms 490, pipelines/front_ms 473, pipelines/wait_ms 394, render/thread/ms 51.58, render/busy/ms 49.81, main/busy/ms 48.13, stage/main/post_update 20.48, stage/main/update 19.75, pipelines/driver_ms 17.39, stage/render/render 16.92, render/graph/ms 13.37, stage/render/prepare_assets 12.24 |
| warm-up | 4.74 | 39.7 | pipelines/compile_ms 658, pipelines/front_ms 634, pipelines/wait_ms 105, main/busy/ms 96.89, render/thread/ms 68.97, render/busy/ms 66.91, stage/main/update 60.83, stage/main/post_update 24.70, pipelines/driver_ms 24.48, stage/render/prepare_assets 20.96, stage/render/render 19.65, stage/render/prepare_flush 15.15 |
| warm-up | 4.83 | 88.9 | pipelines/compile_ms 1067, pipelines/front_ms 1032, pipelines/wait_ms 99.62, main/busy/ms 97.50, render/thread/ms 94.54, render/busy/ms 92.79, stage/main/update 66.33, stage/render/prepare_assets 56.90, pipelines/driver_ms 35.05, stage/render/render 16.39, stage/main/post_update 15.40, render/graph/ms 12.23 |
| warm-up | 4.88 | 45.1 | pipelines/compile_ms 897, pipelines/front_ms 872, pipelines/wait_ms 155, render/thread/ms 102, render/busy/ms 101, stage/render/prepare_assets 71.03, main/busy/ms 66.23, stage/main/update 35.83, pipelines/driver_ms 25.11, stage/main/post_update 16.69, stage/render/render 14.26, render/graph/ms 12.16 |
| warm-up | 4.96 | 78.4 | pipelines/compile_ms 1094, pipelines/front_ms 1065, render/thread/ms 120, render/busy/ms 119, pipelines/wait_ms 83.80, stage/render/prepare_assets 52.04, main/busy/ms 51.98, stage/render/prepare_resources 33.31, stage/main/update 29.52, pipelines/driver_ms 29.37, stage/render/render 16.56, render/graph/ms 14.68 |
| warm-up | 4.99 | 32.8 | pipelines/compile_ms 898, pipelines/front_ms 872, render/thread/ms 107, render/busy/ms 106, main/busy/ms 45.77, stage/render/prepare_resources 35.01, stage/render/prepare_assets 31.47, pipelines/driver_ms 25.66, stage/main/update 22.87, stage/render/render 19.88, render/graph/ms 17.92, stage/render/graph_submit 14.73 |
| warm-up | 5.08 | 33.7 | pipelines/compile_ms 604, pipelines/front_ms 585, render/thread/ms 58.47, render/busy/ms 57.06, main/busy/ms 51.41, stage/main/update 29.52, pipelines/driver_ms 18.45, stage/render/render 14.68, stage/render/prepare_flush 13.19, stage/main/post_update 13.18, render/graph/ms 12.68, stream/drive/ms 10.34 |
| warm-up | 5.35 | 37.9 | pipelines/compile_ms 598, pipelines/front_ms 577, main/busy/ms 48.75, render/thread/ms 48.28, render/busy/ms 46.19, stage/main/update 23.05, stage/render/render 21.50, pipelines/driver_ms 21.13, render/graph/ms 19.08, stage/main/post_update 18.97, stage/render/graph_submit 14.44, encode//sum 10.06 |
| warm-up | 5.62 | 42.7 | render/thread/ms 55.30, render/busy/ms 52.84, main/busy/ms 36.20, stage/render/prepare_assets 24.39, stage/render/render 17.31, stage/main/update 15.34, render/graph/ms 14.04, stage/main/post_update 13.45, encode//sum 11.20, stage/render/graph_submit 9.93, encode/submit 7.11, stage/render/camera_driver 3.82 |
| warm-up | 6.55 | 76.9 | pipelines/compile_ms 220, pipelines/front_ms 178, render/thread/ms 94.26, render/busy/ms 92.76, stage/render/render 69.42, render/graph/ms 67.43, encode//sum 64.14, stage/render/graph_submit 63.41, encode/finish_wall 56.50, encode/pass:early prepass/sum 55.12, pipelines/driver_ms 42.03, main/busy/ms 31.30 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --set groundcover.density_scale=0 --resolution 2560x1440 --bench-out target/perf\gc_p0\balmora@2560x1440_arm=empty-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 11%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 90.9 W · 78 °C (max 81) · VRAM 6916 MiB · throttled 100% of samples

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
