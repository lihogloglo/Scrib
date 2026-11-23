# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); groundcover.render.reserve_worst_case=on (command line: --set); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1197 frames over 15.0 s · **median 12.59 ms (79 fps)** · mean 12.54 · p95 14.16 · p99 14.58 (1% low 69 fps) · worst 16.25 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 311 frames over 9.2 s, worst 2578.1 ms
- hitches (> 31.5 ms): **0** measured, 28 in warm-up
- timed GPU passes: 11.04 ms of a 12.54 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.59 ms, limit 11 ms
- pass worst frame 16.25 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.224 | 3.224 | 3.332 | 1.00 | 0.099 |
| dlss_super_resolution | 2.024 | 2.024 | 2.048 | 1.00 | 0.181 |
| sun_visibility | 0.781 | 0.781 | 0.809 | 1.00 | 0.006 |
| gpu_instance_materialization | 0.675 | 0.675 | 0.898 | 1.00 | 0.008 |
| early prepass | 0.629 | 0.629 | 0.639 | 1.00 | 0.126 |
| main_transmissive_pass_3d | 0.396 | 0.396 | 0.407 | 1.00 | 0.007 |
| atmosphere_luts | 0.372 | 0.372 | 0.380 | 1.00 | 0.003 |
| bloom | 0.315 | 0.315 | 0.331 | 1.00 | 0.058 |
| atmos_froxel_inject | 0.311 | 0.311 | 0.308 | 1.00 | 0.001 |
| water_velocity | 0.307 | 0.307 | 0.311 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.231 | 0.231 | 0.268 | 1.00 | 0.004 |
| water_sspr | 0.214 | 0.214 | 0.210 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.196 | 1.00 | 0.007 |
| clouds_trace | 0.151 | 0.151 | 0.170 | 1.00 | 0.001 |
| water_fft | 0.135 | 0.135 | 0.139 | 1.00 | 0.002 |
| uniform_allocation | 0.120 | 0.120 | 0.153 | 1.00 | 0.019 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| upscaling | 0.113 | 0.113 | 0.120 | 1.00 | 0.004 |
| bin_unpacking | 0.098 | 0.098 | 0.123 | 1.00 | 0.014 |
| tonemapping | 0.085 | 0.085 | 0.088 | 1.00 | 0.003 |
| range_unpacking | 0.085 | 0.085 | 0.094 | 1.00 | 0.003 |
| clustering | 0.063 | 0.063 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.062 | 0.062 | 0.063 | 1.00 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.413 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 1.00 | 0.001 |
| per_view_shadow_pass | 0.028 | 0.028 | 0.054 | 1.00 | 0.006 |
| lightprobe_radiance_map | 0.027 | 0.220 | 0.231 | 0.12 | 0.003 |
| water_snapshot | 0.023 | 0.023 | 0.024 | 1.00 | 0.001 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.024 | 1.00 | 0.185 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.014 | 1.00 | 0.003 |
| surface_exposure_far | 0.011 | 0.035 | 0.036 | 0.31 | 0.002 |
| surface_exposure | 0.010 | 0.036 | 0.036 | 0.28 | 0.001 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.550 | 13.175 | 15.07 | 13825.3 |
| render/busy/ms | 7.491 | 8.576 | 14.57 | 8967.2 |
| main/busy/ms | 6.504 | 7.782 | 9.69 | 7785.0 |
| encode//sum | 5.133 | 6.504 | 9.37 | 6143.8 |
| stage/render/render | 4.964 | 5.745 | 6.95 | 5941.6 |
| render/graph/ms | 4.377 | 5.088 | 6.27 | 5239.6 |
| stage/render/prepare_views | 3.757 | 5.493 | 7.07 | 4497.0 |
| render/acquire/ms | 3.706 | 5.440 | 7.02 | 4436.1 |
| stage/main/post_update | 3.637 | 4.643 | 6.28 | 4353.7 |
| stage/render/graph_submit | 3.152 | 3.834 | 4.98 | 3773.0 |
| encode/submit | 1.905 | 2.254 | 3.12 | 2280.6 |
| stage/main/update | 1.304 | 1.528 | 2.15 | 1560.9 |
| encode/finish_wall | 1.267 | 1.731 | 2.37 | 1517.0 |
| stage/render/camera_driver | 1.141 | 1.299 | 1.63 | 1365.6 |
| encode/pass:early prepass/sum | 0.817 | 1.128 | 1.50 | 978.3 |
| encode/pass:shadow_directional_light/sum | 0.795 | 1.416 | 1.98 | 951.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.776 | 1.074 | 1.37 | 928.6 |
| pipelines/compile_ms | 0.769 | 0.000 | 125.97 | 921.1 |
| pipelines/front_ms | 0.750 | 0.000 | 123.34 | 898.3 |
| stage/render/prepare_write_phase_buffers | 0.487 | 0.719 | 0.94 | 583.4 |
| stage/main/fixed_main_loop | 0.456 | 0.655 | 1.28 | 545.8 |
| stage/render/prepare_resources | 0.426 | 0.497 | 0.68 | 509.5 |
| stage/render/prepare_assets | 0.398 | 0.481 | 1.51 | 475.9 |
| stage/main/pre_update | 0.381 | 0.452 | 0.68 | 456.2 |
| render/present/ms | 0.353 | 0.415 | 0.75 | 422.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.341 | 0.479 | 0.68 | 408.2 |
| bigspace/total/ms | 0.312 | 0.511 | 1.60 | 373.0 |
| stage/main/first | 0.311 | 0.366 | 1.92 | 372.3 |
| stage/render/prepare_flush | 0.271 | 0.407 | 0.64 | 324.9 |
| stage/render/prepare_batch_phases | 0.271 | 0.379 | 0.56 | 324.0 |
| stage/render/prepare_bind_groups | 0.258 | 0.386 | 0.56 | 309.1 |
| stage/main/last | 0.245 | 0.358 | 0.68 | 293.0 |
| stage/render/prepare_meshes | 0.219 | 0.269 | 1.17 | 261.8 |
| stage/render/extract_commands | 0.196 | 0.224 | 0.34 | 234.1 |
| controller/feed/ms | 0.184 | 0.229 | 0.64 | 220.2 |
| stage/render/create_views | 0.155 | 0.193 | 0.36 | 185.6 |
| stream/drive/ms | 0.134 | 0.154 | 0.29 | 160.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.131 | 0.201 | 0.37 | 156.5 |
| bigspace/high_precision/ms | 0.109 | 0.229 | 1.43 | 130.2 |
| stream/tick/ms | 0.108 | 0.125 | 0.26 | 129.5 |
| stage/render/queue | 0.103 | 0.138 | 2.01 | 123.2 |
| controller/move/ms | 0.087 | 0.162 | 0.30 | 103.6 |
| bigspace/low_precision/ms | 0.079 | 0.114 | 0.67 | 95.1 |
| stage/render/specialize | 0.077 | 0.101 | 3.14 | 92.0 |
| anim/evaluate/ms | 0.066 | 0.080 | 0.75 | 79.2 |
| encode/resolve_diagnostics/sum | 0.060 | 0.098 | 0.21 | 72.3 |
| anim/evaluate_cpu/ms | 0.059 | 0.070 | 0.44 | 70.2 |
| controller/write/ms | 0.052 | 0.066 | 0.13 | 61.7 |
| stage/render/prepare_collect_phase_buffers | 0.045 | 0.079 | 0.22 | 54.2 |
| terrain/update/ms | 0.035 | 0.046 | 0.42 | 42.3 |
| physics/step/ms | 0.033 | 0.051 | 0.12 | 39.1 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.040 | 0.10 | 30.8 |
| stage/render/cleanup | 0.025 | 0.032 | 0.47 | 29.7 |
| weather/map/ms | 0.024 | 0.033 | 0.09 | 29.0 |
| stage/render/phase_sort | 0.024 | 0.044 | 0.11 | 28.5 |
| encode/camera_driver_span_end/sum | 0.022 | 0.036 | 0.15 | 26.8 |
| pipelines/driver_ms | 0.019 | 0.000 | 3.33 | 22.7 |
| weather/eval/ms | 0.010 | 0.014 | 0.03 | 12.5 |
| script/update/ms | 0.007 | 0.008 | 0.08 | 8.0 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 6.3 |
| anim/apply/ms | 0.004 | 0.005 | 0.36 | 5.4 |
| controller/avoid/ms | 0.004 | 0.007 | 0.06 | 5.4 |
| far/select/ms | 0.004 | 0.004 | 0.02 | 4.5 |
| script/ui/ms | 0.003 | 0.004 | 0.37 | 4.0 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.02 | 2.0 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.01 | 1.4 |
| character/finish/ms | 0.001 | 0.001 | 0.00 | 1.3 |
| stream/promote/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.0 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.01 | 0.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| terrain/select/ms | 0.001 | 0.001 | 0.01 | 0.7 |
| stage/render/prepare | 0.000 | 0.003 | 0.03 | 0.6 |
| weather/apply/ms | 0.000 | 0.000 | 0.01 | 0.3 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.2 |
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
| crowd/joints_extracted/count | count | 146 | 181 | 181 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 38.0 k | 38.0 k | 38.0 k |
| encode//max | ms | 0.61 | 1.84 | 1.01 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.10 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.15 | 0.02 |
| encode/finish_critical | ms | 0.95 | 1.98 | 1.17 |
| encode/pass:early prepass/max | ms | 0.82 | 1.50 | 1.17 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.79 | 1.37 | 0.76 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.37 | 0.17 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.34 | 0.68 | 0.37 |
| encode/pass:shadow_directional_light/max | ms | 0.74 | 1.98 | 0.70 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.21 | 0.11 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 11.60 | 14.57 | 11.49 |
| gpu/timed/ms | ms | 11.60 | 13.10 | 11.49 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1337 | 1337 | 1337 |
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
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 22.14 | 41.35 | 20.67 |
| process/mem_usage | GiB | 3.00 | 3.08 | 3.08 |
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
| skin/upload_writes/count | count | 3 | 5 | 4 |
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
| system/cpu_usage | % | 29.50 | 39.25 | 36.14 |
| system/mem_usage | % | 50.60 | 51.03 | 50.09 |
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
| anim/events/count | count | 0.13 | 4 | 151 |
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
| encode/bind_walk/init_textures | count | 7455 | 12.4 k | 8.92 M |
| encode/bind_walk/init_textures_skipped | count | 45.0 k | 50.8 k | 53.86 M |
| encode/bind_walk/inits | count | 652 | 779 | 780.9 k |
| encode/bind_walk/inits_skipped | count | 129 | 138 | 154.4 k |
| encode/bind_walk/merge_textures | count | 7390 | 12.3 k | 8.85 M |
| encode/bind_walk/merge_textures_skipped | count | 14.2 k | 19.9 k | 16.94 M |
| encode/bind_walk/merges | count | 135 | 162 | 161.3 k |
| encode/bind_walk/merges_skipped | count | 116 | 131 | 138.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1197 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1197 |
| encode/pass:early prepass/count | count | 1 | 1 | 1197 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1197 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1197 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1197 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1197 |
| encode/queue/command_buffers | count | 38.55 | 41 | 46.1 k |
| encode/queue/submits | count | 2 | 2 | 2394 |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 3.38 M | 2.26 G |
| encode/queue/write_buffer/calls | count | 250 | 323 | 299.6 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 39.07 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 10.8 k |
| encode/queue/write_texture/bytes | bytes | 22.0 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1197 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.75 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.01 | 1 | 12 |
| pipelines/created | count | 0.01 | 1 | 12 |
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

## Hitches (> 31.5 ms; slowest 28, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2578.1 | main/busy/ms 2548, stage/main/post_update 2426, stage/main/update 107, surface/advance_cpu_ms 74.27, stage/main/pre_update 10.59, stage/main/last 1.03, surface/cell_upload_ms 0.74, stage/main/first 0.58, stage/main/fixed_main_loop 0.49, stream/drive/ms 0.39, weather/map/ms 0.21, stream/tick/ms 0.10 |
| warm-up | 3.33 | 755.0 | main/busy/ms 2595, stage/main/post_update 2460, render/thread/ms 694, render/busy/ms 540, stage/render/prepare_assets 505, stage/render/prepare_views 152, render/acquire/ms 151, stage/main/update 113, surface/advance_cpu_ms 74.27, stage/render/render 21.19, render/graph/ms 17.32, stage/render/camera_driver 15.13 |
| warm-up | 3.45 | 118.4 | render/thread/ms 809, render/busy/ms 598, stage/render/prepare_assets 514, pipelines/compile_ms 512, pipelines/driver_ms 292, pipelines/front_ms 220, stage/render/prepare_views 152, render/acquire/ms 152, stage/render/render 116, main/busy/ms 102, stage/main/post_update 70.84, pipelines/wait_ms 68.02 |
| warm-up | 3.54 | 85.4 | pipelines/compile_ms 1166, pipelines/front_ms 811, pipelines/driver_ms 355, pipelines/wait_ms 207, render/thread/ms 199, render/busy/ms 140, main/busy/ms 133, stage/render/render 101, stage/main/post_update 96.01, pipelines/lock_ms 72.56, render/present/ms 58.20, stage/render/prepare_batch_phases 45.15 |
| warm-up | 3.69 | 150.5 | pipelines/wait_ms 5241, pipelines/compile_ms 2665, pipelines/front_ms 2276, pipelines/driver_ms 388, render/thread/ms 231, render/busy/ms 228, main/busy/ms 211, stage/main/post_update 97.40, stage/render/prepare_assets 96.28, stage/main/update 88.48, stage/render/prepare_resources 50.29, stage/render/prepare_batch_phases 44.65 |
| warm-up | 3.83 | 141.4 | pipelines/wait_ms 8326, pipelines/compile_ms 3664, pipelines/front_ms 3152, pipelines/driver_ms 512, render/thread/ms 285, render/busy/ms 283, main/busy/ms 169, stage/render/prepare_assets 157, stage/main/update 91.68, stage/main/post_update 54.09, stage/render/prepare_resources 47.97, stage/render/prepare_meshes 42.30 |
| warm-up | 3.89 | 57.0 | pipelines/wait_ms 3449, pipelines/compile_ms 2273, pipelines/front_ms 2062, pipelines/driver_ms 211, render/thread/ms 192, render/busy/ms 189, stage/render/prepare_assets 95.22, main/busy/ms 52.53, stage/render/prepare_meshes 42.02, stage/render/render 27.68, render/graph/ms 24.93, stage/main/post_update 24.35 |
| warm-up | 4.01 | 54.3 | pipelines/compile_ms 320, pipelines/front_ms 306, pipelines/wait_ms 144, render/thread/ms 62.60, render/busy/ms 60.02, main/busy/ms 45.70, stage/render/prepare_assets 30.73, stage/main/update 19.91, stage/main/post_update 18.71, stage/render/render 17.94, render/graph/ms 14.72, pipelines/driver_ms 13.59 |
| warm-up | 4.06 | 49.6 | pipelines/compile_ms 882, pipelines/front_ms 833, render/thread/ms 90.14, render/busy/ms 87.58, stage/render/prepare_assets 50.64, pipelines/driver_ms 49.03, main/busy/ms 34.87, stage/render/render 26.96, render/graph/ms 23.64, stage/render/graph_submit 17.67, stage/main/update 15.26, stage/main/post_update 12.23 |
| warm-up | 4.11 | 56.1 | pipelines/compile_ms 1609, pipelines/front_ms 1539, render/thread/ms 101, render/busy/ms 98.59, pipelines/driver_ms 70.55, stage/render/prepare_assets 64.30, main/busy/ms 34.81, stage/render/render 25.81, render/graph/ms 22.78, pipelines/wait_ms 19.49, stage/render/graph_submit 16.55, stage/main/post_update 16.41 |
| warm-up | 4.15 | 37.6 | pipelines/compile_ms 1085, pipelines/front_ms 1054, render/thread/ms 87.99, render/busy/ms 85.80, stage/render/prepare_assets 65.35, pipelines/driver_ms 31.46, main/busy/ms 31.04, stage/main/post_update 18.71, pipelines/wait_ms 16.26, stage/render/render 14.64, render/graph/ms 12.00, pipelines/lock_ms 11.33 |
| warm-up | 4.20 | 49.8 | pipelines/compile_ms 325, pipelines/front_ms 317, render/thread/ms 80.42, render/busy/ms 78.75, stage/render/prepare_assets 64.24, main/busy/ms 28.85, stage/main/update 11.60, stage/main/post_update 11.05, stage/render/render 10.05, pipelines/driver_ms 8.54, render/graph/ms 8.11, encode//sum 7.75 |
| warm-up | 4.31 | 47.2 | pipelines/compile_ms 815, pipelines/front_ms 786, render/thread/ms 56.34, render/busy/ms 53.85, main/busy/ms 42.82, pipelines/driver_ms 28.60, stage/main/update 23.84, stage/render/prepare_assets 21.12, stage/render/render 20.51, render/graph/ms 17.35, stage/main/post_update 12.38, stage/render/graph_submit 12.33 |
| warm-up | 4.36 | 47.0 | pipelines/compile_ms 920, pipelines/front_ms 884, render/thread/ms 86.01, render/busy/ms 83.98, main/busy/ms 66.26, stage/render/prepare_assets 46.04, stage/main/update 44.28, pipelines/driver_ms 35.85, stage/render/render 18.70, render/graph/ms 15.54, stage/main/post_update 14.01, pipelines/wait_ms 13.29 |
| warm-up | 4.41 | 49.0 | pipelines/compile_ms 522, pipelines/front_ms 505, render/thread/ms 91.72, render/busy/ms 90.16, stage/render/prepare_assets 62.90, main/busy/ms 62.02, stage/main/update 42.17, pipelines/driver_ms 16.87, stage/render/render 14.59, stage/main/post_update 12.95, render/graph/ms 12.34, pipelines/wait_ms 12.19 |
| warm-up | 4.50 | 59.0 | pipelines/compile_ms 97.81, pipelines/front_ms 92.72, render/thread/ms 86.51, render/busy/ms 84.25, main/busy/ms 80.28, stage/main/update 40.65, stage/render/prepare_assets 26.23, stage/main/post_update 20.43, stage/render/render 19.60, render/graph/ms 17.16, stage/render/graph_submit 12.11, stage/render/prepare_resources 11.65 |
| warm-up | 4.55 | 47.7 | pipelines/compile_ms 1267, pipelines/front_ms 1226, pipelines/wait_ms 142, render/thread/ms 102, render/busy/ms 100, main/busy/ms 93.53, pipelines/driver_ms 41.84, stage/render/prepare_assets 37.88, stage/main/update 33.07, stage/main/post_update 26.68, stage/render/render 24.51, render/graph/ms 20.72 |
| warm-up | 4.62 | 75.4 | pipelines/compile_ms 2144, pipelines/front_ms 2074, pipelines/wait_ms 270, render/thread/ms 120, render/busy/ms 118, main/busy/ms 72.28, pipelines/driver_ms 69.42, stage/render/prepare_assets 54.98, stage/render/render 23.27, stage/main/post_update 21.99, render/graph/ms 19.79, stage/main/update 17.56 |
| warm-up | 4.70 | 74.0 | pipelines/compile_ms 963, pipelines/front_ms 931, render/thread/ms 147, render/busy/ms 145, pipelines/wait_ms 130, main/busy/ms 106, stage/main/update 72.26, stage/render/render 62.48, render/graph/ms 58.71, stage/render/graph_submit 48.03, stage/render/prepare_assets 42.72, encode/finish_wall 35.79 |
| warm-up | 4.77 | 77.4 | pipelines/compile_ms 1841, pipelines/front_ms 1790, render/thread/ms 144, render/busy/ms 142, main/busy/ms 134, pipelines/wait_ms 98.00, stage/main/update 82.76, stage/render/prepare_assets 64.39, stage/render/render 56.73, render/graph/ms 53.06, pipelines/driver_ms 51.11, stage/render/graph_submit 42.25 |
| warm-up | 4.83 | 56.4 | pipelines/compile_ms 2443, pipelines/front_ms 2373, render/thread/ms 126, render/busy/ms 124, pipelines/wait_ms 120, main/busy/ms 98.86, pipelines/driver_ms 69.91, stage/render/prepare_assets 69.91, stage/main/update 38.18, stage/main/post_update 29.55, stage/render/render 20.90, stage/render/prepare_flush 20.57 |
| warm-up | 4.87 | 42.8 | pipelines/compile_ms 1274, pipelines/front_ms 1230, render/thread/ms 95.41, render/busy/ms 93.65, main/busy/ms 63.99, pipelines/wait_ms 46.58, pipelines/driver_ms 43.55, stage/main/update 30.77, stage/render/prepare_assets 26.47, stage/render/render 23.88, render/graph/ms 21.46, stage/render/prepare_flush 20.64 |
| warm-up | 4.92 | 47.3 | pipelines/compile_ms 1124, pipelines/front_ms 1088, pipelines/wait_ms 178, render/thread/ms 80.16, render/busy/ms 78.50, main/busy/ms 44.21, pipelines/driver_ms 35.57, stage/render/prepare_assets 34.84, stage/main/update 23.87, stage/render/render 17.16, render/graph/ms 14.74, stage/render/graph_submit 10.69 |
| warm-up | 5.21 | 71.1 | pipelines/compile_ms 615, pipelines/front_ms 599, render/thread/ms 84.39, render/busy/ms 83.00, main/busy/ms 77.28, stage/main/post_update 37.34, bigspace/total/ms 26.80, stage/main/update 19.02, bigspace/high_precision/ms 18.70, stage/render/render 17.52, stage/render/prepare_assets 16.67, pipelines/driver_ms 16.12 |
| warm-up | 5.30 | 59.8 | pipelines/compile_ms 831, pipelines/front_ms 809, render/thread/ms 84.47, render/busy/ms 63.40, main/busy/ms 50.99, stage/main/update 22.19, pipelines/driver_ms 22.11, stage/main/post_update 20.84, stage/render/prepare_views 19.97, stage/render/prepare_resources 19.91, render/acquire/ms 19.83, stage/render/render 13.95 |
| warm-up | 5.34 | 44.0 | pipelines/compile_ms 2338, pipelines/front_ms 2270, render/thread/ms 99.30, render/busy/ms 77.78, main/busy/ms 73.77, pipelines/driver_ms 67.99, stage/main/post_update 39.06, stage/render/render 27.79, stage/main/update 26.37, render/graph/ms 25.01, stage/render/prepare_views 20.05, render/acquire/ms 19.87 |
| warm-up | 5.58 | 37.9 | render/thread/ms 44.99, render/busy/ms 43.29, main/busy/ms 41.16, stage/main/update 19.74, stage/render/prepare_assets 15.94, stage/render/render 15.79, stage/main/post_update 14.37, render/graph/ms 13.42, encode//sum 10.92, stage/render/graph_submit 8.71, encode/submit 5.83, stream/drive/ms 4.38 |
| warm-up | 5.62 | 42.3 | render/thread/ms 71.91, render/busy/ms 70.32, main/busy/ms 45.94, stage/render/prepare_assets 33.68, stage/main/update 21.88, stage/render/render 16.57, stage/main/post_update 16.30, render/graph/ms 13.75, encode//sum 13.02, stage/render/graph_submit 8.21, stage/render/prepare_meshes 7.00, stage/render/camera_driver 4.99 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --set groundcover.render.reserve_worst_case=true --resolution 2560x1440 --bench-out target/perf\gc_p0b\balmora@2560x1440_arm=worst-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 1%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (285 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 89.9 W · 74 °C (max 77) · VRAM 7353 MiB · throttled 100% of samples

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
