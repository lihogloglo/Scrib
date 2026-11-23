# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); groundcover.density_scale=0 (command line: --set); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1187 frames over 15.0 s · **median 12.71 ms (79 fps)** · mean 12.64 · p95 14.27 · p99 14.84 (1% low 67 fps) · worst 18.63 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 312 frames over 9.4 s, worst 2578.3 ms
- hitches (> 31.8 ms): **0** measured, 31 in warm-up
- timed GPU passes: 11.14 ms of a 12.64 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.71 ms, limit 11 ms
- pass worst frame 18.63 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.216 | 3.216 | 3.312 | 1.00 | 0.080 |
| dlss_super_resolution | 2.023 | 2.023 | 2.045 | 1.00 | 0.180 |
| sun_visibility | 0.789 | 0.789 | 0.810 | 1.00 | 0.006 |
| gpu_instance_materialization | 0.758 | 0.758 | 0.902 | 1.00 | 0.008 |
| early prepass | 0.631 | 0.631 | 0.638 | 1.00 | 0.125 |
| main_transmissive_pass_3d | 0.399 | 0.399 | 0.411 | 1.00 | 0.007 |
| atmosphere_luts | 0.370 | 0.370 | 0.378 | 1.00 | 0.003 |
| bloom | 0.315 | 0.315 | 0.331 | 1.00 | 0.058 |
| atmos_froxel_inject | 0.309 | 0.309 | 0.308 | 1.00 | 0.001 |
| water_velocity | 0.306 | 0.306 | 0.310 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.249 | 0.249 | 0.284 | 1.00 | 0.003 |
| water_sspr | 0.221 | 0.221 | 0.213 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.151 | 0.151 | 0.169 | 1.00 | 0.001 |
| water_fft | 0.135 | 0.135 | 0.139 | 1.00 | 0.002 |
| uniform_allocation | 0.120 | 0.120 | 0.153 | 1.00 | 0.018 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| upscaling | 0.114 | 0.114 | 0.120 | 1.00 | 0.004 |
| bin_unpacking | 0.098 | 0.098 | 0.123 | 1.00 | 0.013 |
| tonemapping | 0.086 | 0.086 | 0.089 | 1.00 | 0.003 |
| range_unpacking | 0.085 | 0.085 | 0.095 | 1.00 | 0.002 |
| clustering | 0.064 | 0.064 | 0.075 | 1.00 | 0.020 |
| render_sky | 0.062 | 0.062 | 0.063 | 1.00 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.409 | 0.411 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.030 | 1.00 | 0.001 |
| per_view_shadow_pass | 0.027 | 0.027 | 0.053 | 1.00 | 0.006 |
| lightprobe_radiance_map | 0.027 | 0.218 | 0.226 | 0.12 | 0.003 |
| water_snapshot | 0.023 | 0.023 | 0.024 | 1.00 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.024 | 1.00 | 0.179 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.001 |
| atmosphere_environment | 0.011 | 0.011 | 0.014 | 1.00 | 0.003 |
| surface_exposure_far | 0.011 | 0.035 | 0.036 | 0.31 | 0.002 |
| surface_exposure | 0.010 | 0.036 | 0.036 | 0.28 | 0.001 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.014 | 0.014 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.641 | 13.255 | 17.44 | 13817.6 |
| render/busy/ms | 7.373 | 8.564 | 16.63 | 8751.3 |
| main/busy/ms | 6.362 | 7.605 | 11.55 | 7551.5 |
| stage/render/render | 4.876 | 5.723 | 8.10 | 5787.7 |
| encode//sum | 4.867 | 6.312 | 16.39 | 5777.5 |
| render/graph/ms | 4.292 | 5.085 | 7.40 | 5095.1 |
| stage/render/prepare_views | 3.966 | 5.623 | 6.78 | 4707.4 |
| render/acquire/ms | 3.915 | 5.571 | 6.73 | 4647.7 |
| stage/main/post_update | 3.494 | 4.380 | 6.27 | 4147.7 |
| stage/render/graph_submit | 3.103 | 3.870 | 6.01 | 3683.8 |
| encode/submit | 1.887 | 2.297 | 3.14 | 2240.0 |
| stage/main/update | 1.294 | 1.518 | 2.29 | 1535.6 |
| encode/finish_wall | 1.233 | 1.696 | 3.70 | 1463.8 |
| stage/render/camera_driver | 1.103 | 1.258 | 1.72 | 1308.8 |
| pipelines/compile_ms | 1.019 | 0.000 | 128.15 | 1210.0 |
| pipelines/front_ms | 0.993 | 0.000 | 125.14 | 1178.9 |
| encode/pass:early prepass/sum | 0.811 | 1.088 | 2.75 | 962.9 |
| encode/pass:shadow_directional_light/sum | 0.772 | 1.393 | 3.57 | 916.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.757 | 1.059 | 1.39 | 898.5 |
| stage/render/prepare_write_phase_buffers | 0.483 | 0.687 | 1.00 | 573.6 |
| stage/main/fixed_main_loop | 0.459 | 0.692 | 1.37 | 545.2 |
| stage/render/prepare_resources | 0.422 | 0.496 | 0.65 | 500.8 |
| stage/render/prepare_assets | 0.390 | 0.469 | 1.61 | 462.3 |
| stage/main/pre_update | 0.375 | 0.451 | 0.82 | 445.3 |
| render/present/ms | 0.353 | 0.409 | 0.87 | 418.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.328 | 0.453 | 0.62 | 389.5 |
| stage/main/first | 0.319 | 0.373 | 1.73 | 378.6 |
| bigspace/total/ms | 0.296 | 0.463 | 1.54 | 351.1 |
| stage/render/prepare_flush | 0.267 | 0.376 | 0.68 | 317.4 |
| stage/render/prepare_batch_phases | 0.262 | 0.366 | 0.50 | 310.9 |
| stage/render/prepare_bind_groups | 0.258 | 0.392 | 0.56 | 306.4 |
| stage/main/last | 0.247 | 0.350 | 5.35 | 293.2 |
| stage/render/prepare_meshes | 0.220 | 0.268 | 1.07 | 261.3 |
| stage/render/extract_commands | 0.190 | 0.219 | 0.28 | 225.1 |
| controller/feed/ms | 0.175 | 0.216 | 0.57 | 207.5 |
| stage/render/create_views | 0.157 | 0.195 | 0.32 | 186.5 |
| stream/drive/ms | 0.131 | 0.152 | 0.48 | 155.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.121 | 0.179 | 0.30 | 143.2 |
| stage/render/queue | 0.105 | 0.142 | 3.69 | 124.8 |
| stream/tick/ms | 0.105 | 0.122 | 0.45 | 124.6 |
| bigspace/high_precision/ms | 0.101 | 0.176 | 1.34 | 119.5 |
| controller/move/ms | 0.093 | 0.219 | 0.33 | 110.0 |
| bigspace/low_precision/ms | 0.076 | 0.102 | 0.64 | 89.8 |
| stage/render/specialize | 0.074 | 0.099 | 3.30 | 88.4 |
| anim/evaluate/ms | 0.071 | 0.082 | 1.55 | 84.0 |
| anim/evaluate_cpu/ms | 0.063 | 0.074 | 0.48 | 75.1 |
| encode/resolve_diagnostics/sum | 0.056 | 0.083 | 0.26 | 66.9 |
| controller/write/ms | 0.053 | 0.068 | 0.14 | 62.3 |
| stage/render/prepare_collect_phase_buffers | 0.042 | 0.074 | 0.20 | 50.4 |
| terrain/update/ms | 0.035 | 0.045 | 0.40 | 41.1 |
| physics/step/ms | 0.031 | 0.048 | 0.10 | 36.5 |
| pipelines/driver_ms | 0.026 | 0.000 | 3.73 | 31.1 |
| stage/render/phase_sort | 0.026 | 0.048 | 0.14 | 31.0 |
| stage/render/cleanup | 0.025 | 0.033 | 0.28 | 30.2 |
| weather/map/ms | 0.024 | 0.033 | 0.10 | 28.5 |
| encode/camera_driver_span_begin/sum | 0.024 | 0.036 | 0.12 | 28.3 |
| encode/camera_driver_span_end/sum | 0.021 | 0.033 | 0.14 | 24.6 |
| weather/eval/ms | 0.010 | 0.014 | 0.03 | 12.4 |
| script/update/ms | 0.006 | 0.008 | 0.02 | 7.4 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 5.7 |
| controller/avoid/ms | 0.004 | 0.006 | 0.07 | 5.0 |
| anim/apply/ms | 0.004 | 0.005 | 0.03 | 4.7 |
| far/select/ms | 0.004 | 0.004 | 0.18 | 4.3 |
| script/ui/ms | 0.003 | 0.004 | 0.39 | 3.8 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.02 | 2.2 |
| character/finish/ms | 0.001 | 0.002 | 0.02 | 1.5 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.02 | 1.5 |
| stream/promote/ms | 0.001 | 0.001 | 0.02 | 1.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 0.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.00 | 0.6 |
| terrain/select/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.4 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.3 |
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
| crowd/joints_extracted/count | count | 146 | 181 | 180 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 38.0 k | 38.0 k | 38.0 k |
| encode//max | ms | 0.58 | 3.48 | 0.72 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.12 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.14 | 0.02 |
| encode/finish_critical | ms | 0.94 | 3.57 | 0.78 |
| encode/pass:early prepass/max | ms | 0.82 | 2.75 | 0.75 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 1.39 | 0.78 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.30 | 0.18 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.33 | 0.62 | 0.31 |
| encode/pass:shadow_directional_light/max | ms | 0.72 | 3.57 | 0.63 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.26 | 0.05 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 11.81 | 16.63 | 11.49 |
| gpu/timed/ms | ms | 11.81 | 13.11 | 11.49 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 766 | 766 | 766 |
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
| phases/opaque3d/bins | count | 884 | 884 | 884 |
| phases/opaque3d/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d/instance_draws | count | 5306 | 5306 | 5306 |
| phases/opaque3d/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d/items | count | 2792 | 2792 | 2792 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 22 | 22 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 27 | 27 | 27 |
| phases/opaque3d_prepass/bins | count | 884 | 884 | 884 |
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
| precache/throttled | count | 0 | 6 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 24.52 | 38.94 | 25.24 |
| process/mem_usage | GiB | 3.10 | 3.12 | 3.11 |
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
| skin/blocks_changed/count | count | 5 | 7 | 6 |
| skin/blocks_skipped/count | count | 176 | 179 | 175 |
| skin/blocks_walked/count | count | 5 | 7 | 6 |
| skin/joint_bytes_written/bytes | bytes | 10.2 k | 12.5 k | 12.3 k |
| skin/joints_written/count | count | 160 | 196 | 192 |
| skin/upload_writes/count | count | 4 | 6 | 4 |
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
| system/cpu_usage | % | 29.15 | 43.57 | 25.68 |
| system/mem_usage | % | 55.30 | 55.44 | 55.37 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2548 | 2548 | 2548 |
| textures/images | count | 2000 | 2000 | 2000 |
| views/all/batch_sets | count | 105 | 152 | 105 |
| views/all/bins | count | 3195 | 3808 | 3195 |
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
| views/main/bins | count | 2284 | 2284 | 2284 |
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
| anim/events/count | count | 0.12 | 3 | 147 |
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
| encode//count | count | 28.55 | 31 | 33.9 k |
| encode/bind_walk/init_textures | count | 7457 | 12.4 k | 8.85 M |
| encode/bind_walk/init_textures_skipped | count | 48.4 k | 52.7 k | 57.42 M |
| encode/bind_walk/inits | count | 652 | 779 | 774.4 k |
| encode/bind_walk/inits_skipped | count | 131 | 140 | 155.5 k |
| encode/bind_walk/merge_textures | count | 7392 | 12.3 k | 8.77 M |
| encode/bind_walk/merge_textures_skipped | count | 13.7 k | 18.0 k | 16.23 M |
| encode/bind_walk/merges | count | 135 | 162 | 160.0 k |
| encode/bind_walk/merges_skipped | count | 116 | 130 | 137.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1187 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1187 |
| encode/pass:early prepass/count | count | 1 | 1 | 1187 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1187 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1187 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1187 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1187 |
| encode/queue/command_buffers | count | 38.55 | 41 | 45.8 k |
| encode/queue/submits | count | 2 | 2 | 2374 |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 3.38 M | 2.24 G |
| encode/queue/write_buffer/calls | count | 251 | 324 | 298.1 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 38.74 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 10.7 k |
| encode/queue/write_texture/bytes | bytes | 22.2 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1187 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.76 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 18 |
| pipelines/created | count | 0.01 | 1 | 17 |
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

## Hitches (> 31.8 ms; slowest 31, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2578.3 | main/busy/ms 2547, stage/main/post_update 2435, stage/main/update 99.40, surface/advance_cpu_ms 68.68, stage/main/pre_update 9.35, stage/main/last 1.03, surface/cell_upload_ms 0.74, stage/main/first 0.52, stream/drive/ms 0.42, stage/main/fixed_main_loop 0.38, weather/map/ms 0.19, stream/tick/ms 0.10 |
| warm-up | 3.41 | 828.3 | main/busy/ms 2594, stage/main/post_update 2469, render/thread/ms 764, render/busy/ms 608, stage/render/prepare_assets 573, stage/render/prepare_views 154, render/acquire/ms 153, stage/main/update 106, surface/advance_cpu_ms 68.68, stage/render/render 19.90, render/graph/ms 15.87, stage/render/camera_driver 13.62 |
| warm-up | 3.55 | 144.0 | render/thread/ms 905, render/busy/ms 660, stage/render/prepare_assets 579, stage/render/prepare_views 154, render/acquire/ms 154, stage/render/render 145, main/busy/ms 116, render/present/ms 91.17, pipelines/compile_ms 85.90, stage/main/post_update 81.76, pipelines/front_ms 81.53, render/graph/ms 51.90 |
| warm-up | 3.66 | 109.6 | pipelines/compile_ms 1756, pipelines/front_ms 974, pipelines/driver_ms 782, pipelines/wait_ms 571, render/thread/ms 249, stage/render/render 190, render/busy/ms 159, main/busy/ms 155, render/graph/ms 98.66, render/present/ms 89.66, stage/main/post_update 77.59, stage/render/camera_driver 62.36 |
| warm-up | 3.78 | 123.2 | pipelines/wait_ms 6023, pipelines/compile_ms 3682, pipelines/front_ms 2788, pipelines/driver_ms 894, render/thread/ms 227, render/busy/ms 225, main/busy/ms 127, stage/render/prepare_assets 124, stage/render/render 68.64, render/graph/ms 65.47, stage/main/update 62.20, stage/render/camera_driver 61.93 |
| warm-up | 3.82 | 40.7 | pipelines/wait_ms 7331, pipelines/compile_ms 2490, pipelines/front_ms 2349, render/thread/ms 158, render/busy/ms 156, pipelines/driver_ms 141, stage/render/prepare_assets 110, main/busy/ms 67.84, stage/main/post_update 27.56, stage/render/prepare_resources 21.54, terrain/update/ms 19.06, stage/main/update 16.44 |
| warm-up | 3.90 | 72.3 | pipelines/wait_ms 3540, pipelines/compile_ms 1103, pipelines/front_ms 1047, render/thread/ms 108, render/busy/ms 100, main/busy/ms 70.78, pipelines/driver_ms 55.85, stage/render/prepare_assets 46.42, stage/main/post_update 42.72, stage/render/prepare_resources 20.18, terrain/update/ms 19.06, stage/render/prepare_flush 9.73 |
| warm-up | 3.96 | 66.6 | pipelines/wait_ms 2226, pipelines/compile_ms 1079, pipelines/front_ms 996, render/thread/ms 135, encode//sum 134, render/busy/ms 127, main/busy/ms 109, stage/main/post_update 92.96, pipelines/driver_ms 83.37, stage/render/render 41.82, render/graph/ms 39.68, stage/render/graph_submit 30.61 |
| warm-up | 4.00 | 38.2 | pipelines/compile_ms 1482, pipelines/front_ms 1382, pipelines/wait_ms 1359, encode//sum 136, render/thread/ms 103, pipelines/driver_ms 101, render/busy/ms 100, main/busy/ms 98.83, stage/main/post_update 66.38, stage/render/render 44.30, render/graph/ms 42.00, stage/render/graph_submit 32.39 |
| warm-up | 4.07 | 45.3 | pipelines/compile_ms 962, pipelines/front_ms 904, pipelines/wait_ms 319, render/thread/ms 61.83, render/busy/ms 59.61, pipelines/driver_ms 58.58, main/busy/ms 57.36, stage/main/post_update 32.37, stage/render/prepare_assets 21.46, stage/render/render 18.81, stage/main/update 17.44, render/graph/ms 15.90 |
| warm-up | 4.14 | 66.7 | pipelines/compile_ms 1416, pipelines/front_ms 1324, pipelines/wait_ms 465, render/thread/ms 102, render/busy/ms 99.73, pipelines/driver_ms 92.02, stage/render/prepare_assets 51.19, main/busy/ms 47.92, stage/render/render 30.83, render/graph/ms 27.51, stage/main/update 17.36, stage/render/graph_submit 17.18 |
| warm-up | 4.18 | 46.3 | pipelines/compile_ms 913, pipelines/front_ms 848, pipelines/wait_ms 233, render/thread/ms 105, render/busy/ms 102, pipelines/driver_ms 65.19, stage/render/prepare_assets 57.83, main/busy/ms 41.01, stage/render/render 27.51, render/graph/ms 24.18, stage/main/first 14.51, stage/render/graph_submit 14.13 |
| warm-up | 4.27 | 87.3 | pipelines/compile_ms 587, pipelines/front_ms 567, render/thread/ms 122, render/busy/ms 120, stage/render/prepare_assets 93.27, main/busy/ms 47.93, pipelines/driver_ms 19.71, stage/main/post_update 15.58, stage/render/render 12.88, stage/main/update 11.78, stage/main/first 11.04, render/graph/ms 10.54 |
| warm-up | 4.30 | 32.0 | pipelines/compile_ms 739, pipelines/front_ms 720, render/thread/ms 110, render/busy/ms 108, stage/render/prepare_assets 90.32, main/busy/ms 45.93, stage/main/post_update 22.69, pipelines/driver_ms 18.32, stage/main/update 13.25, stage/render/render 12.34, render/graph/ms 10.51, stage/render/graph_submit 7.71 |
| warm-up | 4.43 | 36.2 | render/thread/ms 49.40, render/busy/ms 46.99, stage/render/prepare_assets 28.72, main/busy/ms 26.85, stage/render/render 13.26, stage/main/update 12.37, render/graph/ms 10.46, stage/main/post_update 8.86, encode//sum 7.69, stage/render/graph_submit 6.82, encode/submit 4.63, stage/render/camera_driver 3.28 |
| warm-up | 4.51 | 53.9 | pipelines/compile_ms 219, pipelines/front_ms 212, render/thread/ms 66.97, render/busy/ms 64.90, main/busy/ms 45.62, stage/render/prepare_assets 32.55, stage/main/update 24.52, stage/render/render 21.26, render/graph/ms 18.72, stage/main/post_update 12.44, stage/render/graph_submit 11.97, encode//sum 8.78 |
| warm-up | 4.55 | 39.3 | pipelines/compile_ms 543, pipelines/front_ms 525, render/thread/ms 85.67, render/busy/ms 83.62, main/busy/ms 76.19, stage/main/update 47.10, stage/render/prepare_assets 37.40, stage/render/render 23.28, render/graph/ms 21.02, pipelines/driver_ms 17.61, stage/render/graph_submit 12.39, stage/main/post_update 11.33 |
| warm-up | 4.60 | 53.7 | pipelines/compile_ms 815, pipelines/front_ms 790, render/thread/ms 73.67, render/busy/ms 71.97, main/busy/ms 64.83, stage/main/update 38.15, stage/render/prepare_assets 31.58, pipelines/driver_ms 25.22, stage/render/render 16.33, render/graph/ms 14.21, stage/main/last 10.93, stage/main/post_update 9.01 |
| warm-up | 4.67 | 69.6 | pipelines/compile_ms 796, pipelines/front_ms 772, render/thread/ms 105, render/busy/ms 104, main/busy/ms 63.14, stage/main/post_update 32.11, pipelines/wait_ms 26.50, stage/render/prepare_resources 24.84, stage/render/render 24.39, stage/render/prepare_assets 24.08, pipelines/driver_ms 24.07, render/graph/ms 22.28 |
| warm-up | 4.80 | 112.5 | pipelines/compile_ms 1669, pipelines/front_ms 1619, main/busy/ms 139, stage/main/update 102, render/thread/ms 98.42, render/busy/ms 96.12, pipelines/driver_ms 50.67, stage/render/render 40.65, render/graph/ms 37.71, stage/render/prepare_assets 33.50, stage/render/graph_submit 22.95, stage/main/post_update 22.34 |
| warm-up | 4.91 | 118.3 | pipelines/compile_ms 2491, pipelines/front_ms 2423, render/thread/ms 185, render/busy/ms 183, main/busy/ms 127, stage/main/update 89.33, stage/render/prepare_assets 81.36, pipelines/driver_ms 68.37, stage/render/render 44.48, render/graph/ms 41.45, stage/render/create_views 31.00, stage/render/graph_submit 25.99 |
| warm-up | 4.98 | 63.7 | pipelines/compile_ms 2176, pipelines/front_ms 2122, render/thread/ms 162, render/busy/ms 160, stage/render/prepare_assets 80.44, main/busy/ms 63.44, pipelines/driver_ms 54.37, stage/render/create_views 30.86, stage/main/update 29.42, pipelines/wait_ms 29.26, stage/render/render 22.93, render/graph/ms 20.45 |
| warm-up | 5.01 | 34.6 | pipelines/compile_ms 1276, pipelines/front_ms 1240, render/thread/ms 94.44, render/busy/ms 92.56, main/busy/ms 60.84, stage/render/prepare_assets 36.49, pipelines/driver_ms 35.93, pipelines/wait_ms 34.24, stage/main/update 29.88, stage/render/render 23.35, render/graph/ms 21.01, stage/render/graph_submit 17.31 |
| warm-up | 5.08 | 36.1 | pipelines/compile_ms 68.70, pipelines/front_ms 65.61, render/thread/ms 62.15, render/busy/ms 60.33, main/busy/ms 42.94, stage/main/update 23.36, stage/render/prepare_assets 16.97, stage/render/render 13.70, stage/main/post_update 13.06, render/graph/ms 11.69, encode//sum 8.22, stage/render/graph_submit 8.19 |
| warm-up | 5.11 | 32.2 | pipelines/compile_ms 448, pipelines/front_ms 430, render/thread/ms 64.72, render/busy/ms 62.76, main/busy/ms 48.05, stage/main/update 26.66, pipelines/driver_ms 18.06, stage/render/render 15.50, stage/main/post_update 14.25, stage/render/prepare_resources 13.60, render/graph/ms 13.21, stage/render/graph_submit 9.70 |
| warm-up | 5.23 | 36.2 | pipelines/compile_ms 443, pipelines/front_ms 430, main/busy/ms 51.00, render/thread/ms 46.38, render/busy/ms 44.38, stage/main/update 26.98, stage/render/render 20.40, render/graph/ms 17.73, stage/main/post_update 14.58, pipelines/driver_ms 13.27, stage/render/graph_submit 13.24, stage/render/prepare_assets 8.49 |
| warm-up | 5.29 | 35.4 | pipelines/compile_ms 293, pipelines/front_ms 279, render/thread/ms 54.72, render/busy/ms 52.81, main/busy/ms 32.81, stage/render/render 30.93, render/graph/ms 28.46, stage/render/graph_submit 23.81, encode/submit 14.78, pipelines/driver_ms 13.93, stage/main/update 13.77, stage/main/post_update 11.20 |
| warm-up | 5.32 | 35.3 | pipelines/compile_ms 245, pipelines/front_ms 237, render/thread/ms 67.93, render/busy/ms 65.94, main/busy/ms 34.01, stage/render/render 29.39, render/graph/ms 27.17, stage/render/graph_submit 23.25, stage/main/update 16.72, encode/submit 13.42, stage/render/prepare_assets 12.48, stage/main/post_update 10.38 |
| warm-up | 5.37 | 48.0 | pipelines/compile_ms 628, pipelines/front_ms 611, render/thread/ms 80.28, render/busy/ms 78.65, main/busy/ms 44.41, stage/render/prepare_resources 31.05, stage/main/update 20.49, stage/main/post_update 17.73, pipelines/driver_ms 17.33, stage/render/render 14.79, render/graph/ms 12.69, stage/render/prepare_assets 10.43 |
| warm-up | 5.44 | 41.2 | pipelines/compile_ms 411, pipelines/front_ms 396, render/thread/ms 67.31, render/busy/ms 65.22, main/busy/ms 41.79, stage/main/post_update 22.73, stage/render/prepare_flush 20.63, pipelines/wait_ms 15.64, pipelines/driver_ms 14.41, stage/render/render 12.61, stage/render/prepare_assets 12.01, stage/main/update 11.64 |
| warm-up | 5.78 | 47.2 | render/thread/ms 71.78, render/busy/ms 69.61, main/busy/ms 47.28, stage/render/prepare_assets 30.68, stage/main/update 22.45, stage/render/render 21.50, render/graph/ms 18.09, stage/main/post_update 15.57, encode//sum 14.44, stage/render/graph_submit 13.38, encode/submit 9.33, stream/drive/ms 6.32 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --set groundcover.density_scale=0 --resolution 2560x1440 --bench-out target/perf\gc_p0\balmora@2560x1440_arm=empty-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 89.4 W · 74 °C (max 77) · VRAM 7617 MiB · throttled 100% of samples

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
