# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1169 frames over 15.0 s · **median 12.84 ms (78 fps)** · mean 12.84 · p95 14.54 · p99 15.02 (1% low 67 fps) · worst 22.80 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 0.0/min)
- warm-up: 311 frames over 9.0 s, worst 2575.3 ms
- hitches (> 32.1 ms): **0** measured, 21 in warm-up
- timed GPU passes: 11.17 ms of a 12.84 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.84 ms, limit 11 ms
- pass worst frame 22.80 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.234 | 3.240 | 3.330 | 1.00 | 0.084 |
| dlss_super_resolution | 2.018 | 2.021 | 2.046 | 1.00 | 0.179 |
| sun_visibility | 0.795 | 0.796 | 0.814 | 1.00 | 0.006 |
| gpu_instance_materialization | 0.787 | 0.789 | 0.903 | 1.00 | 0.008 |
| early prepass | 0.629 | 0.630 | 0.641 | 1.00 | 0.122 |
| main_transmissive_pass_3d | 0.397 | 0.398 | 0.409 | 1.00 | 0.007 |
| atmosphere_luts | 0.369 | 0.370 | 0.378 | 1.00 | 0.003 |
| bloom | 0.314 | 0.315 | 0.331 | 1.00 | 0.059 |
| atmos_froxel_inject | 0.310 | 0.310 | 0.309 | 1.00 | 0.001 |
| water_velocity | 0.306 | 0.307 | 0.311 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.254 | 0.255 | 0.280 | 1.00 | 0.003 |
| water_sspr | 0.209 | 0.209 | 0.210 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.198 | 1.00 | 0.007 |
| clouds_trace | 0.152 | 0.153 | 0.174 | 1.00 | 0.001 |
| water_fft | 0.134 | 0.134 | 0.138 | 1.00 | 0.002 |
| clouds_composite | 0.118 | 0.118 | 0.120 | 1.00 | 0.002 |
| uniform_allocation | 0.117 | 0.117 | 0.150 | 1.00 | 0.018 |
| upscaling | 0.114 | 0.114 | 0.120 | 1.00 | 0.004 |
| bin_unpacking | 0.096 | 0.096 | 0.121 | 1.00 | 0.013 |
| tonemapping | 0.085 | 0.085 | 0.089 | 1.00 | 0.003 |
| range_unpacking | 0.084 | 0.084 | 0.097 | 1.00 | 0.002 |
| clustering | 0.065 | 0.065 | 0.077 | 1.00 | 0.020 |
| render_sky | 0.063 | 0.063 | 0.065 | 1.00 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.413 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.027 | 0.220 | 0.230 | 0.12 | 0.003 |
| per_view_shadow_pass | 0.026 | 0.027 | 0.053 | 1.00 | 0.006 |
| atmos_froxel_integrate | 0.026 | 0.026 | 0.027 | 1.00 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 1.00 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.017 | 0.017 | 0.025 | 1.00 | 0.180 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 1.00 | 0.002 |
| surface_exposure_far | 0.011 | 0.035 | 0.036 | 0.31 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.014 | 1.00 | 0.003 |
| surface_exposure | 0.010 | 0.035 | 0.036 | 0.28 | 0.001 |
| night_sky | 0.006 | 0.006 | 0.008 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.832 | 13.548 | 21.87 | 13831.2 |
| render/busy/ms | 7.512 | 8.952 | 17.73 | 8781.0 |
| main/busy/ms | 6.455 | 7.880 | 9.62 | 7545.6 |
| encode//sum | 5.015 | 6.820 | 62.02 | 5863.0 |
| stage/render/render | 4.988 | 6.138 | 15.44 | 5831.3 |
| pipelines/compile_ms | 4.568 | 14.207 | 129.82 | 5340.4 |
| pipelines/front_ms | 4.443 | 13.528 | 126.16 | 5193.4 |
| render/graph/ms | 4.396 | 5.499 | 14.70 | 5138.6 |
| stage/render/prepare_views | 4.012 | 5.918 | 7.61 | 4689.8 |
| render/acquire/ms | 3.960 | 5.868 | 7.53 | 4628.8 |
| stage/main/post_update | 3.559 | 4.641 | 6.26 | 4160.2 |
| stage/render/graph_submit | 3.198 | 4.176 | 13.45 | 3738.9 |
| encode/submit | 1.958 | 2.574 | 4.97 | 2289.3 |
| stage/main/update | 1.304 | 1.525 | 2.22 | 1524.0 |
| encode/finish_wall | 1.257 | 1.734 | 9.49 | 1469.3 |
| stage/render/camera_driver | 1.110 | 1.319 | 1.56 | 1297.8 |
| encode/pass:early prepass/sum | 0.822 | 1.143 | 8.19 | 961.2 |
| encode/pass:shadow_directional_light/sum | 0.795 | 1.399 | 8.41 | 929.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.760 | 1.089 | 1.37 | 888.7 |
| stage/main/fixed_main_loop | 0.489 | 0.883 | 1.33 | 571.9 |
| stage/render/prepare_write_phase_buffers | 0.466 | 0.676 | 0.92 | 545.0 |
| stage/render/prepare_resources | 0.433 | 0.518 | 1.26 | 506.1 |
| stage/render/prepare_assets | 0.395 | 0.488 | 1.56 | 462.2 |
| stage/main/pre_update | 0.380 | 0.451 | 0.79 | 444.6 |
| render/present/ms | 0.360 | 0.430 | 0.79 | 421.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.342 | 0.499 | 8.15 | 400.2 |
| stage/main/first | 0.317 | 0.376 | 1.41 | 371.1 |
| bigspace/total/ms | 0.305 | 0.501 | 1.78 | 356.9 |
| stage/render/prepare_flush | 0.274 | 0.411 | 1.85 | 320.7 |
| stage/render/prepare_batch_phases | 0.267 | 0.377 | 0.54 | 311.9 |
| stage/render/prepare_bind_groups | 0.253 | 0.376 | 0.59 | 295.4 |
| stage/main/last | 0.235 | 0.324 | 0.71 | 275.3 |
| stage/render/prepare_meshes | 0.218 | 0.273 | 1.11 | 255.1 |
| stage/render/extract_commands | 0.191 | 0.223 | 0.34 | 223.3 |
| controller/feed/ms | 0.175 | 0.228 | 0.38 | 204.1 |
| stage/render/create_views | 0.158 | 0.195 | 0.50 | 184.9 |
| stream/drive/ms | 0.133 | 0.153 | 0.34 | 155.0 |
| pipelines/driver_ms | 0.126 | 0.733 | 4.13 | 147.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.123 | 0.188 | 0.32 | 143.2 |
| controller/move/ms | 0.117 | 0.414 | 0.74 | 137.3 |
| bigspace/high_precision/ms | 0.108 | 0.189 | 1.55 | 125.7 |
| stage/render/queue | 0.107 | 0.142 | 2.95 | 125.2 |
| stream/tick/ms | 0.106 | 0.124 | 0.25 | 124.5 |
| stage/render/specialize | 0.100 | 0.181 | 3.65 | 116.9 |
| bigspace/low_precision/ms | 0.078 | 0.112 | 0.73 | 91.4 |
| anim/evaluate_cpu/ms | 0.067 | 0.076 | 0.45 | 78.0 |
| encode/resolve_diagnostics/sum | 0.056 | 0.087 | 0.19 | 65.6 |
| controller/write/ms | 0.053 | 0.068 | 0.11 | 62.4 |
| anim/evaluate/ms | 0.049 | 0.064 | 0.48 | 57.7 |
| stage/render/prepare_collect_phase_buffers | 0.042 | 0.071 | 0.33 | 48.9 |
| terrain/update/ms | 0.035 | 0.047 | 0.48 | 41.4 |
| physics/step/ms | 0.032 | 0.049 | 0.09 | 37.6 |
| stage/render/cleanup | 0.027 | 0.040 | 0.37 | 31.6 |
| stage/render/phase_sort | 0.026 | 0.049 | 0.11 | 30.3 |
| weather/map/ms | 0.024 | 0.033 | 0.05 | 28.2 |
| encode/camera_driver_span_begin/sum | 0.024 | 0.037 | 0.08 | 27.9 |
| encode/camera_driver_span_end/sum | 0.020 | 0.033 | 0.10 | 23.6 |
| weather/eval/ms | 0.011 | 0.014 | 0.04 | 12.8 |
| script/update/ms | 0.006 | 0.008 | 0.03 | 7.4 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 5.6 |
| controller/avoid/ms | 0.004 | 0.007 | 0.02 | 5.2 |
| anim/apply/ms | 0.004 | 0.005 | 0.38 | 5.2 |
| far/select/ms | 0.004 | 0.004 | 0.04 | 4.3 |
| script/ui/ms | 0.003 | 0.004 | 0.04 | 3.6 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.08 | 2.3 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.01 | 1.5 |
| character/finish/ms | 0.001 | 0.002 | 0.01 | 1.4 |
| pipelines/wait_ms | 0.001 | 0.014 | 0.03 | 1.3 |
| stream/promote/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.0 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| terrain/select/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| stage/render/prepare | 0.000 | 0.002 | 0.06 | 0.4 |
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
| crowd/c1/count | count | 122 | 123 | 123 |
| crowd/c1/joints/count | count | 4848 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1080 | 1088 | 1088 |
| crowd/c2/count | count | 284 | 284 | 283 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 136 | 181 | 181 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 38.0 k | 38.0 k | 38.0 k |
| encode//max | ms | 0.59 | 9.09 | 0.46 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.08 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.10 | 0.02 |
| encode/finish_critical | ms | 0.94 | 9.09 | 0.86 |
| encode/pass:early prepass/max | ms | 0.80 | 8.19 | 0.86 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.75 | 1.37 | 0.51 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.32 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.33 | 8.15 | 0.35 |
| encode/pass:shadow_directional_light/max | ms | 0.73 | 8.41 | 0.40 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.19 | 0.05 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 11.87 | 17.73 | 11.49 |
| gpu/timed/ms | ms | 11.87 | 12.90 | 11.49 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1324 | 1324 | 1324 |
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
| precache/throttled | count | 0 | 39 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 121 | 121 | 121 |
| process/cpu_usage | % | 26.44 | 48.08 | 12.26 |
| process/mem_usage | GiB | 3.06 | 3.14 | 3.11 |
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
| skin/blocks_skipped/count | count | 176 | 178 | 174 |
| skin/blocks_walked/count | count | 5 | 7 | 7 |
| skin/joint_bytes_written/bytes | bytes | 9984 | 12.5 k | 12.5 k |
| skin/joints_written/count | count | 156 | 196 | 196 |
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
| system/cpu_usage | % | 30.21 | 45.44 | 23.47 |
| system/mem_usage | % | 56.06 | 56.27 | 56.18 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2548 | 2548 | 2548 |
| textures/images | count | 2000 | 2000 | 2000 |
| views/all/batch_sets | count | 103 | 150 | 103 |
| views/all/bins | count | 3193 | 3806 | 3193 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 53 | 71 | 53 |
| views/all/instance_draws | count | 17.6 k | 29.4 k | 17.6 k |
| views/all/instance_slots | count | 1.50 M | 1.66 M | 1.50 M |
| views/all/items | count | 9459 | 14.4 k | 9459 |
| views/all/pipelines | count | 87 | 94 | 87 |
| views/all/pipelines_distinct | count | 71 | 71 | 71 |
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
| anim/events/count | count | 0.11 | 2 | 133 |
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
| encode//count | count | 28.56 | 31 | 33.4 k |
| encode/bind_walk/init_textures | count | 7455 | 12.4 k | 8.71 M |
| encode/bind_walk/init_textures_skipped | count | 44.0 k | 52.7 k | 51.48 M |
| encode/bind_walk/inits | count | 648 | 775 | 757.9 k |
| encode/bind_walk/inits_skipped | count | 126 | 137 | 147.9 k |
| encode/bind_walk/merge_textures | count | 7390 | 12.3 k | 8.64 M |
| encode/bind_walk/merge_textures_skipped | count | 17.0 k | 25.7 k | 19.92 M |
| encode/bind_walk/merges | count | 135 | 162 | 157.5 k |
| encode/bind_walk/merges_skipped | count | 115 | 132 | 134.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1169 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1169 |
| encode/pass:early prepass/count | count | 1 | 1 | 1169 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1169 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1169 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1169 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1169 |
| encode/queue/command_buffers | count | 38.56 | 41 | 45.1 k |
| encode/queue/submits | count | 2 | 2 | 2338 |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 3.38 M | 2.21 G |
| encode/queue/write_buffer/calls | count | 246 | 319 | 287.6 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 38.16 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 10.5 k |
| encode/queue/write_texture/bytes | bytes | 22.5 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1169 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.77 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.06 | 1 | 69 |
| pipelines/created | count | 0.07 | 1 | 81 |
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

## Hitches (> 32.1 ms; slowest 21, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2575.3 | main/busy/ms 2542, stage/main/post_update 2426, stage/main/update 103, surface/advance_cpu_ms 70.40, stage/main/pre_update 9.46, stage/main/last 1.17, surface/cell_upload_ms 0.76, stage/main/first 0.51, stage/main/fixed_main_loop 0.38, stream/drive/ms 0.36, weather/map/ms 0.27, stream/tick/ms 0.11 |
| warm-up | 3.41 | 834.9 | main/busy/ms 2588, stage/main/post_update 2457, render/thread/ms 773, render/busy/ms 672, stage/render/prepare_assets 644, stage/main/update 110, stage/render/prepare_views 97.70, render/acquire/ms 97.47, surface/advance_cpu_ms 70.40, stage/render/render 13.57, stage/main/pre_update 11.53, stage/render/prepare_resources 9.32 |
| warm-up | 3.51 | 100.9 | render/thread/ms 871, render/busy/ms 707, stage/render/prepare_assets 649, stage/render/render 98.80, stage/render/prepare_views 98.41, render/acquire/ms 98.07, main/busy/ms 79.73, render/present/ms 65.68, stage/main/post_update 53.54, render/graph/ms 31.21, pipelines/compile_ms 28.69, pipelines/front_ms 25.63 |
| warm-up | 3.62 | 111.0 | pipelines/compile_ms 1152, pipelines/front_ms 881, pipelines/wait_ms 352, pipelines/driver_ms 270, render/thread/ms 208, render/busy/ms 141, stage/render/render 119, pipelines/lock_ms 108, main/busy/ms 68.38, render/present/ms 64.13, render/graph/ms 53.68, stage/main/post_update 39.61 |
| warm-up | 3.74 | 121.8 | pipelines/wait_ms 3607, pipelines/compile_ms 2656, pipelines/front_ms 2288, pipelines/driver_ms 368, render/thread/ms 229, render/busy/ms 224, pipelines/lock_ms 124, main/busy/ms 98.13, stage/render/prepare_assets 62.20, stage/main/update 61.29, stage/render/prepare_resources 48.41, stage/render/prepare_flush 46.68 |
| warm-up | 3.85 | 109.3 | pipelines/wait_ms 8552, pipelines/compile_ms 3051, pipelines/front_ms 2887, render/thread/ms 225, render/busy/ms 223, pipelines/driver_ms 164, main/busy/ms 129, stage/render/prepare_assets 102, pipelines/lock_ms 66.39, stage/main/update 53.41, stage/render/prepare_resources 45.91, stage/main/post_update 43.56 |
| warm-up | 3.94 | 84.5 | pipelines/wait_ms 6897, pipelines/compile_ms 2123, pipelines/front_ms 2026, render/thread/ms 176, render/busy/ms 174, main/busy/ms 126, pipelines/driver_ms 97.13, stage/render/prepare_assets 95.31, stage/main/fixed_main_loop 51.11, pipelines/lock_ms 50.04, stage/main/post_update 40.13, stage/main/update 19.29 |
| warm-up | 4.03 | 39.5 | pipelines/compile_ms 1268, pipelines/front_ms 1191, pipelines/wait_ms 322, pipelines/driver_ms 76.70, render/thread/ms 67.48, render/busy/ms 64.97, main/busy/ms 53.65, stage/main/post_update 26.94, stage/render/render 22.66, render/graph/ms 19.48, stage/render/graph_submit 15.72, encode//sum 13.43 |
| warm-up | 4.10 | 65.7 | pipelines/compile_ms 1188, pipelines/front_ms 1117, pipelines/wait_ms 273, render/thread/ms 98.47, render/busy/ms 95.41, pipelines/driver_ms 71.62, main/busy/ms 51.80, stage/render/prepare_assets 48.52, stage/render/render 29.14, render/graph/ms 25.31, stage/render/graph_submit 20.72, stage/main/post_update 20.69 |
| warm-up | 4.15 | 49.9 | pipelines/compile_ms 700, pipelines/front_ms 657, render/thread/ms 108, render/busy/ms 105, stage/render/prepare_assets 69.07, pipelines/wait_ms 62.10, pipelines/driver_ms 43.09, main/busy/ms 32.52, stage/render/render 25.58, render/graph/ms 21.70, stage/render/graph_submit 15.41, stage/main/update 14.21 |
| warm-up | 4.22 | 74.7 | pipelines/compile_ms 800, pipelines/front_ms 778, render/thread/ms 120, render/busy/ms 117, stage/render/prepare_assets 91.99, pipelines/wait_ms 39.42, main/busy/ms 36.31, pipelines/lock_ms 30.46, pipelines/driver_ms 21.81, stage/render/render 18.77, render/graph/ms 15.73, stage/main/pre_update 14.03 |
| warm-up | 4.26 | 36.4 | pipelines/compile_ms 995, pipelines/front_ms 968, render/thread/ms 106, render/busy/ms 105, stage/render/prepare_assets 87.75, main/busy/ms 40.04, pipelines/wait_ms 38.95, pipelines/lock_ms 30.03, pipelines/driver_ms 26.90, stage/main/pre_update 13.73, stage/render/render 11.78, stage/main/update 11.65 |
| warm-up | 4.43 | 41.7 | render/thread/ms 63.23, render/busy/ms 61.76, main/busy/ms 41.18, stage/render/prepare_assets 33.87, stage/main/update 22.03, stage/render/render 12.97, stage/main/post_update 11.01, render/graph/ms 10.97, encode//sum 7.97, stage/render/graph_submit 6.71, encode/submit 4.62, stage/render/camera_driver 3.19 |
| warm-up | 4.57 | 36.9 | render/thread/ms 52.11, render/busy/ms 50.12, main/busy/ms 44.33, stage/main/update 25.64, stage/render/prepare_assets 19.30, stage/render/render 15.71, render/graph/ms 13.26, stage/main/post_update 11.17, encode//sum 9.64, scene/load/chains/ms 8.74, stage/render/graph_submit 8.71, encode/submit 6.51 |
| warm-up | 4.64 | 36.2 | pipelines/compile_ms 750, pipelines/front_ms 733, render/thread/ms 62.00, pipelines/wait_ms 61.86, render/busy/ms 60.11, main/busy/ms 34.29, stage/render/prepare_assets 29.61, pipelines/driver_ms 16.72, stage/main/update 15.21, stage/render/render 14.41, render/graph/ms 12.06, stage/main/post_update 9.84 |
| warm-up | 4.97 | 32.1 | render/thread/ms 38.28, render/busy/ms 36.64, main/busy/ms 31.84, stage/render/render 16.34, stage/main/update 15.62, render/graph/ms 14.12, stage/main/post_update 11.55, stage/render/graph_submit 9.84, encode//sum 9.46, encode/submit 6.91, stage/render/prepare_assets 5.73, stream/drive/ms 3.92 |
| warm-up | 5.08 | 52.9 | pipelines/wait_ms 644, pipelines/compile_ms 368, pipelines/front_ms 344, render/thread/ms 63.38, render/busy/ms 62.18, main/busy/ms 58.47, stage/main/post_update 42.67, stage/render/prepare_flush 35.74, pipelines/driver_ms 24.03, stage/render/render 11.31, stage/main/update 11.12, render/graph/ms 9.70 |
| warm-up | 5.20 | 34.4 | pipelines/wait_ms 2197, pipelines/compile_ms 378, pipelines/front_ms 365, render/thread/ms 53.66, render/busy/ms 52.21, main/busy/ms 48.95, stage/main/update 30.80, stage/render/render 16.06, render/graph/ms 14.11, pipelines/driver_ms 12.68, stage/main/post_update 11.29, encode//sum 10.58 |
| warm-up | 5.27 | 37.6 | pipelines/wait_ms 1380, render/thread/ms 66.24, render/busy/ms 64.53, main/busy/ms 45.50, pipelines/compile_ms 42.04, pipelines/front_ms 40.45, stage/main/update 24.57, stage/render/prepare_assets 24.33, stage/render/render 14.40, stage/main/post_update 13.68, render/graph/ms 12.16, stage/render/prepare_meshes 10.44 |
| warm-up | 5.30 | 35.5 | pipelines/wait_ms 1453, render/thread/ms 70.18, render/busy/ms 68.63, main/busy/ms 56.18, pipelines/compile_ms 41.09, pipelines/front_ms 39.36, stage/main/update 36.33, stage/render/render 30.99, render/graph/ms 28.67, stage/render/graph_submit 24.56, encode/finish_wall 18.85, stage/render/prepare_assets 17.64 |
| warm-up | 5.37 | 66.0 | pipelines/wait_ms 2132, pipelines/compile_ms 424, pipelines/front_ms 408, render/thread/ms 94.97, render/busy/ms 93.44, main/busy/ms 59.07, stage/main/update 38.53, stage/render/prepare_assets 37.84, stage/render/render 33.08, render/graph/ms 30.81, stage/render/graph_submit 26.27, encode/finish_wall 19.05 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\gc_p0\balmora@2560x1440_arm=on-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 90.5 W · 78 °C (max 80) · VRAM 7625 MiB · throttled 100% of samples

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
