# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1293 frames over 15.0 s · **median 11.69 ms (86 fps)** · mean 11.61 · p95 13.01 · p99 13.53 (1% low 74 fps) · worst 18.19 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 313 frames over 8.8 s, worst 2517.1 ms
- hitches (> 29.2 ms): **0** measured, 34 in warm-up
- timed GPU passes: 10.19 ms of a 11.61 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.69 ms, limit 11 ms
- pass worst frame 18.19 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.223 | 3.226 | 3.325 | 1.00 | 0.085 |
| dlss_super_resolution | 2.023 | 2.025 | 2.045 | 1.00 | 0.180 |
| sun_visibility | 0.782 | 0.783 | 0.808 | 1.00 | 0.007 |
| early prepass | 0.650 | 0.651 | 0.658 | 1.00 | 0.098 |
| main_transmissive_pass_3d | 0.397 | 0.397 | 0.409 | 1.00 | 0.008 |
| atmosphere_luts | 0.373 | 0.373 | 0.380 | 1.00 | 0.003 |
| bloom | 0.314 | 0.315 | 0.331 | 1.00 | 0.059 |
| atmos_froxel_inject | 0.308 | 0.309 | 0.309 | 1.00 | 0.002 |
| water_velocity | 0.306 | 0.306 | 0.311 | 1.00 | 0.002 |
| water_sspr | 0.213 | 0.213 | 0.210 | 1.00 | 0.002 |
| ssao | 0.188 | 0.188 | 0.196 | 1.00 | 0.008 |
| clouds_trace | 0.152 | 0.153 | 0.172 | 1.00 | 0.001 |
| water_fft | 0.135 | 0.135 | 0.136 | 1.00 | 0.002 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| uniform_allocation | 0.116 | 0.116 | 0.147 | 1.00 | 0.023 |
| upscaling | 0.113 | 0.113 | 0.119 | 1.00 | 0.004 |
| bin_unpacking | 0.090 | 0.090 | 0.114 | 1.00 | 0.017 |
| tonemapping | 0.085 | 0.085 | 0.088 | 1.00 | 0.003 |
| gpu_instance_materialization | 0.068 | 0.068 | 0.091 | 1.00 | 0.004 |
| clustering | 0.063 | 0.063 | 0.074 | 1.00 | 0.021 |
| render_sky | 0.062 | 0.062 | 0.062 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.058 | 0.058 | 0.081 | 1.00 | 0.005 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.412 | 0.13 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.223 | 0.233 | 0.13 | 0.004 |
| water_snapshot | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| range_unpacking | 0.022 | 0.022 | 0.030 | 1.00 | 0.004 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.018 | 0.018 | 0.035 | 1.00 | 0.006 |
| main_transparent_pass_3d | 0.017 | 0.017 | 0.026 | 1.00 | 0.195 |
| surface_exposure_far | 0.012 | 0.039 | 0.043 | 0.30 | 0.002 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.002 |
| surface_exposure | 0.011 | 0.040 | 0.041 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.007 | 0.007 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.13 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.13 | 0.005 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.622 | 12.023 | 17.06 | 13734.0 |
| render/busy/ms | 7.599 | 8.615 | 16.50 | 9826.0 |
| main/busy/ms | 6.770 | 8.028 | 9.56 | 8753.5 |
| encode//sum | 5.107 | 6.510 | 9.53 | 6603.1 |
| stage/render/render | 5.030 | 5.772 | 9.10 | 6503.3 |
| render/graph/ms | 4.441 | 5.133 | 8.43 | 5741.7 |
| stage/main/post_update | 3.887 | 4.857 | 6.39 | 5026.1 |
| stage/render/graph_submit | 3.232 | 3.917 | 7.31 | 4178.4 |
| stage/render/prepare_views | 2.710 | 4.301 | 5.32 | 3503.9 |
| render/acquire/ms | 2.663 | 4.244 | 5.26 | 3443.8 |
| pipelines/compile_ms | 2.229 | 0.000 | 131.14 | 2881.6 |
| pipelines/front_ms | 2.168 | 0.000 | 126.82 | 2803.0 |
| encode/submit | 1.975 | 2.318 | 4.80 | 2554.2 |
| stage/main/update | 1.368 | 1.824 | 2.15 | 1769.4 |
| encode/finish_wall | 1.287 | 1.739 | 2.57 | 1664.2 |
| stage/render/camera_driver | 1.128 | 1.287 | 1.52 | 1458.6 |
| encode/pass:shadow_directional_light/sum | 0.829 | 1.406 | 2.21 | 1071.4 |
| encode/pass:early prepass/sum | 0.798 | 1.094 | 2.07 | 1032.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.761 | 1.067 | 1.51 | 983.4 |
| stage/render/prepare_write_phase_buffers | 0.531 | 0.769 | 1.02 | 687.0 |
| stage/render/prepare_resources | 0.422 | 0.489 | 0.64 | 545.9 |
| stage/main/fixed_main_loop | 0.410 | 0.634 | 1.49 | 529.7 |
| stage/render/prepare_assets | 0.400 | 0.479 | 1.53 | 516.7 |
| stage/main/pre_update | 0.385 | 0.452 | 0.76 | 497.3 |
| render/present/ms | 0.359 | 0.429 | 1.20 | 464.2 |
| bigspace/total/ms | 0.357 | 0.653 | 1.46 | 461.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.350 | 0.492 | 0.64 | 452.1 |
| stage/main/first | 0.307 | 0.363 | 2.33 | 397.2 |
| stage/render/prepare_flush | 0.270 | 0.386 | 0.71 | 349.5 |
| stage/render/prepare_batch_phases | 0.264 | 0.367 | 0.54 | 341.7 |
| stage/render/prepare_bind_groups | 0.262 | 0.377 | 0.52 | 338.2 |
| stage/main/last | 0.239 | 0.330 | 1.23 | 308.5 |
| stage/render/prepare_meshes | 0.215 | 0.261 | 1.19 | 277.5 |
| stage/render/extract_commands | 0.199 | 0.230 | 0.30 | 256.7 |
| controller/feed/ms | 0.188 | 0.239 | 0.74 | 243.0 |
| stage/render/create_views | 0.158 | 0.190 | 0.31 | 204.1 |
| stream/drive/ms | 0.140 | 0.165 | 0.59 | 181.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.138 | 0.205 | 0.30 | 177.9 |
| bigspace/high_precision/ms | 0.129 | 0.310 | 1.27 | 166.4 |
| stream/tick/ms | 0.113 | 0.134 | 0.56 | 145.8 |
| stage/render/queue | 0.096 | 0.126 | 2.39 | 123.8 |
| bigspace/low_precision/ms | 0.094 | 0.228 | 0.97 | 121.6 |
| stage/render/specialize | 0.085 | 0.138 | 3.52 | 109.7 |
| anim/evaluate/ms | 0.076 | 0.148 | 0.80 | 97.9 |
| controller/move/ms | 0.064 | 0.134 | 0.27 | 82.3 |
| anim/evaluate_cpu/ms | 0.061 | 0.074 | 0.46 | 78.7 |
| pipelines/driver_ms | 0.061 | 0.000 | 4.32 | 78.6 |
| encode/resolve_diagnostics/sum | 0.060 | 0.090 | 0.20 | 77.5 |
| controller/write/ms | 0.051 | 0.066 | 0.17 | 66.5 |
| stage/render/prepare_collect_phase_buffers | 0.049 | 0.085 | 0.17 | 62.7 |
| terrain/update/ms | 0.036 | 0.049 | 0.50 | 46.4 |
| stage/render/phase_sort | 0.029 | 0.046 | 0.09 | 37.2 |
| physics/step/ms | 0.029 | 0.051 | 0.18 | 37.0 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.040 | 0.12 | 34.3 |
| weather/map/ms | 0.025 | 0.034 | 0.09 | 31.7 |
| stage/render/cleanup | 0.024 | 0.032 | 0.49 | 31.5 |
| encode/camera_driver_span_end/sum | 0.024 | 0.037 | 0.12 | 30.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.07 | 13.8 |
| script/update/ms | 0.007 | 0.008 | 0.08 | 8.9 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 6.7 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 5.6 |
| controller/avoid/ms | 0.004 | 0.007 | 0.02 | 5.6 |
| far/select/ms | 0.004 | 0.004 | 0.03 | 4.7 |
| script/ui/ms | 0.003 | 0.004 | 0.02 | 4.1 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.02 | 2.5 |
| character/finish/ms | 0.001 | 0.001 | 0.36 | 1.7 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.05 | 1.6 |
| stream/promote/ms | 0.001 | 0.002 | 0.02 | 1.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 1.2 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.8 |
| terrain/select/ms | 0.001 | 0.001 | 0.06 | 0.7 |
| pipelines/wait_ms | 0.001 | 0.000 | 0.03 | 0.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 0.4 |
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
| crowd/c2/count | count | 283 | 283 | 283 |
| crowd/c2/joints/count | count | 4916 | 4916 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1373 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 147 | 181 | 136 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.9 k | 37.9 k | 37.9 k |
| encode//max | ms | 0.62 | 1.84 | 0.50 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.12 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.12 | 0.02 |
| encode/finish_critical | ms | 0.94 | 2.21 | 1.32 |
| encode/pass:early prepass/max | ms | 0.79 | 2.07 | 0.99 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.75 | 1.51 | 0.75 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.13 | 0.30 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.34 | 0.64 | 0.22 |
| encode/pass:shadow_directional_light/max | ms | 0.78 | 2.21 | 1.32 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.20 | 0.06 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 10.80 | 16.50 | 10.81 |
| gpu/timed/ms | ms | 10.80 | 12.11 | 10.81 |
| groundcover/batches/count | count | 0 | 0 | 0 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1333 | 1333 | 1333 |
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
| phases/shadow/batch_sets | count | 4 | 41 | 34 |
| phases/shadow/bins | count | 896 | 1509 | 896 |
| phases/shadow/instance_batches | count | 4 | 12 | 12 |
| phases/shadow/instance_draws | count | 5861 | 17.6 k | 17.6 k |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 147.5 k |
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
| precache/throttled | count | 0 | 27 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 53 |
| preprocess/bind_groups_created | count | 0 | 62 | 46 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 7 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 24.66 | 42.79 | 31.70 |
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
| skin/blocks_changed/count | count | 6 | 7 | 6 |
| skin/blocks_skipped/count | count | 175 | 180 | 175 |
| skin/blocks_walked/count | count | 6 | 7 | 6 |
| skin/joint_bytes_written/bytes | bytes | 12.3 k | 12.5 k | 9472 |
| skin/joints_written/count | count | 192 | 196 | 148 |
| skin/upload_writes/count | count | 4 | 5 | 4 |
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
| system/cpu_usage | % | 29.61 | 38.99 | 33.77 |
| system/mem_usage | % | 50.86 | 51.03 | 51.03 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2548 | 2548 | 2548 |
| textures/images | count | 2000 | 2000 | 2000 |
| views/all/batch_sets | count | 64 | 101 | 94 |
| views/all/bins | count | 3142 | 3755 | 3142 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 12 | 20 | 20 |
| views/all/instance_draws | count | 17.6 k | 29.3 k | 29.3 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 245.8 k |
| views/all/items | count | 9408 | 14.4 k | 9408 |
| views/all/pipelines | count | 75 | 82 | 75 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 153.9 k | 260.4 k | 255.4 k |
| views/cascade/batch_sets | count | 4 | 41 | 34 |
| views/cascade/bins | count | 896 | 1509 | 896 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 12 |
| views/cascade/instance_draws | count | 5861 | 17.6 k | 17.6 k |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 3202 | 8161 | 3202 |
| views/cascade/pipelines | count | 27 | 34 | 27 |
| views/cascade/work_items | count | 49.2 k | 155.6 k | 150.6 k |
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
| anim/events/count | count | 0.11 | 3 | 141 |
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
| encode//count | count | 28.55 | 31 | 36.9 k |
| encode/bind_walk/init_textures | count | 7197 | 12.1 k | 9.31 M |
| encode/bind_walk/init_textures_skipped | count | 44.0 k | 52.7 k | 56.95 M |
| encode/bind_walk/inits | count | 504 | 608 | 651.6 k |
| encode/bind_walk/inits_skipped | count | 129 | 139 | 166.2 k |
| encode/bind_walk/merge_textures | count | 7132 | 12.0 k | 9.22 M |
| encode/bind_walk/merge_textures_skipped | count | 8857 | 16.1 k | 11.45 M |
| encode/bind_walk/merges | count | 91.27 | 111 | 118.0 k |
| encode/bind_walk/merges_skipped | count | 113 | 129 | 146.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1293 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1293 |
| encode/pass:early prepass/count | count | 1 | 1 | 1293 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1293 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1293 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1293 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1293 |
| encode/queue/command_buffers | count | 38.55 | 41 | 49.8 k |
| encode/queue/submits | count | 2 | 2 | 2586 |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 3.37 M | 2.43 G |
| encode/queue/write_buffer/calls | count | 233 | 307 | 301.6 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 42.20 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 11.6 k |
| encode/queue/write_texture/bytes | bytes | 20.4 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1293 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.70 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.03 | 1 | 43 |
| pipelines/created | count | 0.03 | 1 | 43 |
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

## Hitches (> 29.2 ms; slowest 34, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2517.1 | main/busy/ms 2486, stage/main/post_update 2368, stage/main/update 104, surface/advance_cpu_ms 73.12, stage/main/pre_update 10.65, stage/main/last 1.12, surface/cell_upload_ms 1.05, stage/main/first 0.60, stream/drive/ms 0.46, stage/main/fixed_main_loop 0.46, weather/map/ms 0.19, stream/tick/ms 0.11 |
| warm-up | 3.28 | 764.6 | main/busy/ms 2534, stage/main/post_update 2401, render/thread/ms 702, render/busy/ms 546, stage/render/prepare_assets 510, stage/render/prepare_views 154, render/acquire/ms 153, stage/main/update 111, surface/advance_cpu_ms 73.12, stage/render/render 21.40, render/graph/ms 17.86, stage/render/camera_driver 15.62 |
| warm-up | 3.39 | 103.8 | render/thread/ms 803, render/busy/ms 589, stage/render/prepare_assets 520, stage/render/prepare_views 154, render/acquire/ms 154, stage/render/render 99.69, main/busy/ms 89.30, stage/main/post_update 64.07, render/present/ms 60.53, encode//sum 49.69, render/graph/ms 37.44, terrain/update/ms 25.26 |
| warm-up | 3.44 | 49.8 | pipelines/compile_ms 403, pipelines/front_ms 300, render/thread/ms 149, pipelines/driver_ms 103, stage/render/render 101, main/busy/ms 89.61, pipelines/wait_ms 89.01, render/busy/ms 85.11, stage/main/post_update 69.06, render/present/ms 63.95, encode//sum 47.14, render/graph/ms 35.14 |
| warm-up | 3.51 | 74.5 | pipelines/wait_ms 1481, pipelines/compile_ms 934, pipelines/front_ms 803, main/busy/ms 205, pipelines/driver_ms 131, stage/main/update 118, render/thread/ms 108, render/busy/ms 101, stage/main/post_update 76.86, stage/render/prepare_assets 34.83, stage/render/render 29.06, render/graph/ms 20.98 |
| warm-up | 3.70 | 191.4 | pipelines/wait_ms 7126, pipelines/compile_ms 2911, pipelines/front_ms 2698, pipelines/driver_ms 213, main/busy/ms 179, render/thread/ms 144, render/busy/ms 142, stage/main/update 121, stage/render/prepare_assets 86.14, stage/main/post_update 49.28, stage/render/prepare_resources 17.89, terrain/update/ms 13.90 |
| warm-up | 3.73 | 30.3 | pipelines/wait_ms 5862, pipelines/compile_ms 2909, pipelines/front_ms 2706, pipelines/driver_ms 203, render/thread/ms 113, render/busy/ms 110, stage/render/prepare_assets 84.60, main/busy/ms 40.95, stage/main/post_update 21.12, stage/render/render 15.85, encode//sum 15.56, terrain/update/ms 14.70 |
| warm-up | 3.82 | 34.3 | pipelines/compile_ms 273, pipelines/front_ms 255, pipelines/wait_ms 64.89, render/thread/ms 52.99, render/busy/ms 50.95, main/busy/ms 47.70, stage/main/post_update 20.51, stage/main/update 18.64, pipelines/driver_ms 18.54, stage/render/render 12.56, stage/render/prepare_resources 12.33, encode//sum 11.25 |
| warm-up | 3.87 | 53.1 | pipelines/compile_ms 367, pipelines/front_ms 348, pipelines/wait_ms 94.13, render/thread/ms 83.62, render/busy/ms 81.55, main/busy/ms 66.63, stage/render/prepare_assets 36.34, stage/main/update 24.66, stage/main/post_update 19.90, pipelines/driver_ms 18.45, stage/render/render 18.19, render/graph/ms 15.31 |
| warm-up | 3.90 | 32.3 | pipelines/compile_ms 934, pipelines/front_ms 888, pipelines/wait_ms 96.38, render/thread/ms 80.23, render/busy/ms 77.76, main/busy/ms 56.92, pipelines/driver_ms 46.20, stage/render/prepare_assets 44.85, stage/main/update 19.49, stage/render/render 19.48, render/graph/ms 16.35, stage/main/pre_update 12.71 |
| warm-up | 3.95 | 50.3 | pipelines/compile_ms 1240, pipelines/front_ms 1179, render/thread/ms 77.99, render/busy/ms 75.30, pipelines/driver_ms 60.53, stage/render/prepare_assets 51.04, main/busy/ms 39.00, pipelines/wait_ms 31.53, stage/render/render 14.68, stage/main/update 14.02, render/graph/ms 11.87, stage/main/post_update 9.97 |
| warm-up | 4.00 | 50.5 | pipelines/compile_ms 1010, pipelines/front_ms 949, render/thread/ms 92.65, render/busy/ms 90.38, stage/render/prepare_assets 64.54, pipelines/driver_ms 60.99, main/busy/ms 52.39, pipelines/wait_ms 45.60, stage/main/first 15.30, stage/render/render 15.05, pipelines/lock_ms 14.40, render/graph/ms 12.73 |
| warm-up | 4.04 | 41.0 | pipelines/compile_ms 617, pipelines/front_ms 582, render/thread/ms 83.78, render/busy/ms 82.31, stage/render/prepare_assets 60.96, main/busy/ms 48.35, pipelines/driver_ms 34.62, pipelines/wait_ms 16.60, stage/main/first 15.31, pipelines/lock_ms 13.01, stage/render/render 12.79, stage/main/update 11.00 |
| warm-up | 4.15 | 35.1 | pipelines/compile_ms 208, pipelines/front_ms 203, main/busy/ms 47.99, render/thread/ms 44.16, render/busy/ms 41.95, stage/main/update 21.28, stage/main/post_update 20.29, stage/render/render 17.26, render/graph/ms 14.67, stage/render/prepare_assets 14.37, stage/render/graph_submit 9.96, encode//sum 7.23 |
| warm-up | 4.19 | 39.7 | pipelines/compile_ms 950, pipelines/front_ms 902, main/busy/ms 80.58, render/thread/ms 64.82, render/busy/ms 61.92, pipelines/driver_ms 47.90, stage/main/update 46.31, stage/render/prepare_assets 25.96, stage/main/post_update 21.63, stage/render/render 21.54, render/graph/ms 17.77, pipelines/wait_ms 14.98 |
| warm-up | 4.25 | 55.3 | pipelines/compile_ms 742, pipelines/front_ms 699, render/thread/ms 73.30, render/busy/ms 71.36, main/busy/ms 65.65, stage/render/prepare_assets 46.84, pipelines/driver_ms 42.64, stage/main/update 41.85, stage/render/render 13.55, pipelines/wait_ms 12.54, pipelines/lock_ms 12.05, stage/main/post_update 11.33 |
| warm-up | 4.29 | 45.5 | render/thread/ms 84.95, render/busy/ms 83.34, stage/render/prepare_assets 58.17, pipelines/compile_ms 51.85, pipelines/front_ms 49.63, main/busy/ms 44.32, stage/main/update 23.29, stage/render/render 17.79, render/graph/ms 15.71, stage/main/post_update 14.20, stage/render/graph_submit 9.20, encode//sum 6.16 |
| warm-up | 4.34 | 49.4 | pipelines/compile_ms 134, pipelines/front_ms 117, render/thread/ms 92.51, render/busy/ms 90.78, stage/render/prepare_assets 60.73, main/busy/ms 48.39, stage/render/render 23.70, stage/main/update 21.07, render/graph/ms 20.63, pipelines/driver_ms 17.13, stage/main/post_update 15.05, stage/render/graph_submit 14.17 |
| warm-up | 4.37 | 30.9 | pipelines/compile_ms 726, pipelines/front_ms 690, main/busy/ms 90.00, render/thread/ms 77.78, render/busy/ms 75.58, stage/render/prepare_assets 39.98, pipelines/driver_ms 35.32, stage/main/update 35.03, stage/main/post_update 29.84, stage/render/render 21.07, render/graph/ms 16.93, stage/render/graph_submit 13.45 |
| warm-up | 4.45 | 76.5 | pipelines/compile_ms 1210, pipelines/front_ms 1172, main/busy/ms 82.07, render/thread/ms 67.73, render/busy/ms 65.26, pipelines/wait_ms 54.45, pipelines/driver_ms 38.10, stage/main/update 32.44, stage/main/post_update 30.83, stage/render/prepare_assets 25.53, stage/render/render 19.83, render/graph/ms 16.47 |
| warm-up | 4.49 | 39.4 | pipelines/compile_ms 1447, pipelines/front_ms 1404, render/thread/ms 75.62, pipelines/wait_ms 75.34, render/busy/ms 73.76, main/busy/ms 48.05, pipelines/driver_ms 43.43, stage/render/prepare_assets 29.38, stage/main/post_update 25.01, stage/render/render 14.65, pipelines/lock_ms 14.11, stage/main/update 13.44 |
| warm-up | 4.53 | 40.0 | pipelines/compile_ms 920, pipelines/front_ms 893, render/thread/ms 76.16, render/busy/ms 74.76, main/busy/ms 51.70, pipelines/wait_ms 44.58, stage/render/prepare_assets 29.99, stage/main/post_update 28.93, pipelines/driver_ms 26.82, stage/main/update 12.79, stage/render/prepare_flush 10.29, stage/render/prepare_resources 9.43 |
| warm-up | 4.62 | 94.9 | pipelines/compile_ms 1225, pipelines/front_ms 1175, render/thread/ms 119, render/busy/ms 116, pipelines/wait_ms 105, pipelines/driver_ms 49.95, main/busy/ms 48.18, stage/render/prepare_assets 42.24, stage/render/prepare_write_phase_buffers 37.31, stage/main/post_update 23.62, stage/main/update 14.04, stage/render/prepare_flush 13.09 |
| warm-up | 4.68 | 57.5 | pipelines/compile_ms 2122, pipelines/front_ms 2043, pipelines/wait_ms 138, render/thread/ms 137, render/busy/ms 133, main/busy/ms 106, pipelines/driver_ms 78.79, stage/main/update 67.26, stage/render/prepare_write_phase_buffers 45.34, stage/render/prepare_assets 31.59, stage/render/render 19.98, stage/main/post_update 19.74 |
| warm-up | 4.79 | 109.3 | pipelines/compile_ms 2127, pipelines/front_ms 2065, pipelines/wait_ms 197, render/thread/ms 141, render/busy/ms 139, main/busy/ms 107, stage/main/update 72.27, pipelines/driver_ms 62.26, stage/render/prepare_assets 55.72, stage/render/render 44.13, render/graph/ms 41.55, stage/render/graph_submit 36.02 |
| warm-up | 4.82 | 32.8 | pipelines/compile_ms 1565, pipelines/front_ms 1524, pipelines/wait_ms 144, render/thread/ms 116, render/busy/ms 114, stage/render/prepare_assets 56.52, main/busy/ms 56.38, stage/render/render 44.69, render/graph/ms 42.80, pipelines/driver_ms 41.08, stage/render/graph_submit 38.78, encode//sum 35.81 |
| warm-up | 4.85 | 31.6 | pipelines/compile_ms 374, pipelines/front_ms 365, render/thread/ms 61.43, render/busy/ms 59.58, main/busy/ms 44.65, stage/main/update 27.33, stage/render/render 20.47, render/graph/ms 18.10, stage/render/prepare_assets 18.00, stage/render/graph_submit 13.60, encode/submit 10.58, encode//sum 9.56 |
| warm-up | 5.21 | 30.0 | render/thread/ms 54.20, render/busy/ms 52.71, main/busy/ms 44.45, stage/render/prepare_assets 23.63, stage/main/update 22.05, stage/main/post_update 13.39, stage/render/render 12.33, render/graph/ms 10.21, encode//sum 7.39, stage/render/graph_submit 6.71, stage/render/prepare_meshes 5.61, stream/drive/ms 5.23 |
| warm-up | 5.41 | 32.4 | pipelines/compile_ms 161, pipelines/front_ms 157, render/thread/ms 43.12, render/busy/ms 41.23, main/busy/ms 33.10, stage/render/render 15.90, stage/main/update 15.27, stage/render/prepare_assets 14.54, render/graph/ms 13.36, stage/main/post_update 11.60, encode//sum 10.72, stage/render/graph_submit 9.51 |
| warm-up | 5.44 | 38.1 | render/thread/ms 63.48, render/busy/ms 61.63, main/busy/ms 37.69, stage/render/prepare_assets 26.68, stage/render/render 16.96, stage/main/update 16.70, render/graph/ms 13.92, stage/main/post_update 13.45, encode//sum 11.67, stage/render/graph_submit 9.10, stage/render/prepare_meshes 7.39, encode/submit 5.78 |
| warm-up | 5.47 | 29.7 | render/thread/ms 62.63, render/busy/ms 61.21, main/busy/ms 39.85, stage/render/prepare_assets 27.67, stage/main/update 20.14, stage/render/render 15.39, render/graph/ms 13.02, stage/main/post_update 12.87, encode//sum 9.74, stage/render/graph_submit 9.01, stage/render/prepare_meshes 6.63, encode/submit 5.48 |
| warm-up | 5.78 | 30.3 | pipelines/compile_ms 142, pipelines/front_ms 135, render/thread/ms 38.73, render/busy/ms 37.45, main/busy/ms 32.41, stage/main/update 16.08, stage/render/prepare_assets 14.66, stage/render/render 13.07, render/graph/ms 11.30, stage/main/post_update 10.36, encode//sum 8.92, stage/render/graph_submit 8.15 |
| warm-up | 6.23 | 29.5 | render/thread/ms 52.22, render/busy/ms 50.30, main/busy/ms 47.15, stage/main/update 22.02, stage/render/render 21.14, render/graph/ms 18.36, stage/main/post_update 17.24, encode//sum 13.41, stage/render/graph_submit 13.10, stage/render/prepare_assets 10.85, encode/submit 8.26, stream/drive/ms 7.03 |
| warm-up | 6.26 | 30.1 | render/thread/ms 55.52, render/busy/ms 54.27, main/busy/ms 41.18, stage/render/prepare_assets 23.55, stage/main/update 19.21, stage/render/render 15.17, render/graph/ms 13.32, stage/main/post_update 13.02, encode//sum 10.86, stage/render/graph_submit 9.50, stream/drive/ms 6.46, encode/submit 5.49 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\gc_p0b\balmora@2560x1440_arm=on-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (285 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 92.0 W · 75 °C (max 77) · VRAM 5346 MiB · throttled 100% of samples

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
