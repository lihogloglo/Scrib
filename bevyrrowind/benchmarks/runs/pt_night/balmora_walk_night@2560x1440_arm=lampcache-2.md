# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5290 frames over 65.6 s · **median 10.85 ms (92 fps)** · mean 12.41 · p95 18.22 · p99 21.39 (1% low 47 fps) · worst 32.05 ms · 618 spikes over 1.5× median · 15.5 stutters/min (work 13.7/min)
- warm-up: 325 frames over 9.2 s, worst 2396.7 ms
- hitches (> 27.1 ms): **5** measured, 50 in warm-up
- timed GPU passes: 7.14 ms of a 12.41 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.85 ms, limit 11 ms
- **FAIL** frame_ms: 17 stutters (> 1.5x moving median of 60) in 1.09 min = 15.54/min, limit 1/min
- pass worst frame 32.05 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.025 | 2.029 | 2.061 | 1.00 | 0.177 |
| main_opaque_pass_3d | 1.332 | 1.335 | 1.650 | 1.00 | 0.068 |
| auxiliary_views | 0.610 | 0.611 | 1.763 | 1.00 | 0.548 |
| early prepass | 0.502 | 0.503 | 0.698 | 1.00 | 0.089 |
| bloom | 0.315 | 0.315 | 0.321 | 1.00 | 0.064 |
| atmosphere_luts | 0.309 | 0.310 | 0.372 | 1.00 | 0.003 |
| water_velocity | 0.260 | 0.260 | 0.349 | 1.00 | 0.002 |
| water_sspr | 0.201 | 0.201 | 0.202 | 1.00 | 0.002 |
| ssao | 0.188 | 0.188 | 0.193 | 1.00 | 0.007 |
| water_fft | 0.129 | 0.130 | 0.138 | 1.00 | 0.003 |
| atmos_froxel_inject | 0.127 | 0.127 | 0.128 | 1.00 | 0.001 |
| uniform_allocation | 0.127 | 0.127 | 0.156 | 1.00 | 0.024 |
| upscaling | 0.125 | 0.126 | 0.134 | 1.00 | 0.004 |
| clouds_composite | 0.105 | 0.105 | 0.108 | 1.00 | 0.002 |
| bin_unpacking | 0.097 | 0.097 | 0.120 | 1.00 | 0.017 |
| tonemapping | 0.087 | 0.087 | 0.091 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.078 | 0.078 | 0.154 | 1.00 | 0.008 |
| clustering | 0.064 | 0.064 | 0.067 | 1.00 | 0.019 |
| clouds_trace | 0.057 | 0.057 | 0.074 | 1.00 | 0.001 |
| render_sky | 0.051 | 0.051 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.412 | 0.416 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.043 | 0.056 | 0.057 | 0.77 | 0.002 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.028 | 0.028 | 0.036 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.229 | 0.12 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.029 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| night_sky | 0.017 | 0.017 | 0.039 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.024 | 1.00 | 0.128 |
| clouds_fog_light | 0.012 | 0.012 | 0.013 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 1.00 | 0.004 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.204 | 16.957 | 26.06 | 59268.5 |
| encode//sum | 10.759 | 28.528 | 52.48 | 56914.0 |
| render/busy/ms | 10.666 | 16.349 | 25.57 | 56421.4 |
| main/busy/ms | 9.512 | 13.028 | 28.43 | 50316.1 |
| stage/render/render | 7.362 | 12.269 | 16.16 | 38947.2 |
| render/graph/ms | 6.672 | 11.441 | 15.34 | 35295.9 |
| stage/main/post_update | 6.111 | 9.226 | 12.02 | 32328.6 |
| stage/render/graph_submit | 5.058 | 8.980 | 12.43 | 26758.5 |
| encode/submit | 3.017 | 4.810 | 6.46 | 15962.2 |
| stage/main/update | 2.139 | 2.621 | 22.27 | 11316.8 |
| encode/finish_wall | 2.083 | 4.413 | 6.63 | 11020.5 |
| encode/pass:shadow_point_light/sum | 2.035 | 6.174 | 9.62 | 10765.5 |
| stage/render/camera_driver | 1.556 | 2.649 | 4.34 | 8233.6 |
| stage/render/prepare_write_phase_buffers | 0.791 | 1.228 | 2.99 | 4183.3 |
| encode/pass:shadow_directional_light/sum | 0.766 | 1.327 | 2.19 | 4051.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.689 | 1.085 | 1.84 | 3646.2 |
| encode/pass:early prepass/sum | 0.669 | 1.064 | 2.05 | 3540.6 |
| stage/render/prepare_assets | 0.627 | 0.786 | 9.61 | 3316.7 |
| bigspace/total/ms | 0.567 | 1.486 | 4.70 | 2999.3 |
| stage/render/prepare_resources | 0.458 | 0.558 | 1.42 | 2421.5 |
| render/present/ms | 0.444 | 0.537 | 2.14 | 2347.8 |
| stage/render/prepare_meshes | 0.417 | 0.542 | 2.15 | 2207.8 |
| stage/main/pre_update | 0.360 | 0.487 | 3.29 | 1905.9 |
| stage/main/first | 0.358 | 0.457 | 7.84 | 1894.6 |
| terrain/update/ms | 0.352 | 0.580 | 2.58 | 1863.0 |
| terrain/select/ms | 0.318 | 0.526 | 1.28 | 1680.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.287 | 0.468 | 1.09 | 1520.4 |
| stage/render/prepare_bind_groups | 0.286 | 0.533 | 2.06 | 1515.3 |
| controller/feed/ms | 0.271 | 0.341 | 0.80 | 1431.6 |
| stage/main/fixed_main_loop | 0.243 | 0.381 | 1.26 | 1286.7 |
| stream/drive/ms | 0.243 | 0.331 | 20.09 | 1286.3 |
| bigspace/low_precision/ms | 0.229 | 0.994 | 3.01 | 1209.9 |
| stage/render/specialize | 0.213 | 0.343 | 1.73 | 1127.8 |
| stage/render/queue | 0.199 | 0.349 | 1.56 | 1055.0 |
| far/select/ms | 0.199 | 0.264 | 0.57 | 1050.2 |
| stage/render/create_views | 0.196 | 0.264 | 0.82 | 1035.2 |
| stage/render/prepare_batch_phases | 0.193 | 0.274 | 0.54 | 1021.0 |
| stream/tick/ms | 0.183 | 0.246 | 0.67 | 966.5 |
| bigspace/high_precision/ms | 0.176 | 1.053 | 4.30 | 931.8 |
| stage/render/extract_commands | 0.175 | 0.205 | 0.43 | 924.1 |
| stage/render/prepare_flush | 0.167 | 0.336 | 0.83 | 883.7 |
| anim/evaluate/ms | 0.164 | 0.249 | 5.18 | 867.3 |
| anim/evaluate_cpu/ms | 0.154 | 0.200 | 0.66 | 815.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.140 | 0.259 | 0.86 | 741.8 |
| stage/render/prepare_views | 0.138 | 0.170 | 0.37 | 729.0 |
| controller/move/ms | 0.137 | 0.264 | 0.54 | 722.3 |
| stage/main/last | 0.121 | 0.170 | 1.80 | 639.1 |
| render/acquire/ms | 0.094 | 0.120 | 0.28 | 499.3 |
| stage/render/prepare_collect_phase_buffers | 0.062 | 0.131 | 0.45 | 328.0 |
| physics/step/ms | 0.054 | 0.090 | 0.55 | 287.3 |
| physics/insert/ms | 0.045 | 0.424 | 1.79 | 239.9 |
| encode/point_shadow_cache_copies/sum | 0.045 | 0.170 | 0.67 | 236.1 |
| stage/render/cleanup | 0.040 | 0.129 | 0.86 | 210.9 |
| stage/render/phase_sort | 0.038 | 0.068 | 0.30 | 201.1 |
| encode/resolve_diagnostics/sum | 0.028 | 0.039 | 0.58 | 149.9 |
| pipelines/compile_ms | 0.017 | 0.000 | 78.57 | 89.7 |
| pipelines/front_ms | 0.016 | 0.000 | 75.70 | 86.4 |
| weather/map/ms | 0.016 | 0.020 | 0.29 | 82.8 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.44 | 70.3 |
| anim/apply/ms | 0.012 | 0.023 | 0.38 | 61.2 |
| controller/avoid/ms | 0.010 | 0.014 | 0.18 | 54.4 |
| encode/camera_driver_span_end/sum | 0.010 | 0.019 | 0.05 | 53.9 |
| weather/eval/ms | 0.010 | 0.012 | 0.20 | 51.5 |
| script/update/ms | 0.007 | 0.009 | 0.09 | 38.6 |
| stream/despawn/ms | 0.006 | 0.000 | 1.13 | 31.7 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 28.4 |
| stream/spawn/ms | 0.005 | 0.000 | 1.19 | 28.2 |
| script/ui/ms | 0.005 | 0.006 | 0.23 | 25.3 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 18.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.68 | 14.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.4 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.87 | 3.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.10 | 2.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.9 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.01 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 0 | 8 | 0 |
| anim/a3/count | count | 569 | 579 | 578 |
| anim/characters/count | count | 571 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 16 |
| controller/movers/count | count | 20 | 22 | 16 |
| crowd/c0/count | count | 25 | 43 | 13 |
| crowd/c0/joints/count | count | 1173 | 2262 | 604 |
| crowd/c0/meshes/count | count | 335 | 646 | 144 |
| crowd/c1/count | count | 149 | 157 | 148 |
| crowd/c1/joints/count | count | 5640 | 6057 | 5592 |
| crowd/c1/meshes/count | count | 1203 | 1313 | 1197 |
| crowd/c2/count | count | 400 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1824 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 165 | 216 | 205 |
| crowd/joints_extracted/count | count | 3750 | 9436 | 5775 |
| crowd/joints_uploaded/count | count | 3721 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.65 | 1.72 | 1.07 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.44 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.05 | 0.03 |
| encode/finish_critical | ms | 0.96 | 2.19 | 1.61 |
| encode/pass:early prepass/max | ms | 0.63 | 2.05 | 0.79 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.66 | 1.84 | 0.75 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.86 | 0.21 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.27 | 1.09 | 0.40 |
| encode/pass:shadow_directional_light/max | ms | 0.75 | 2.19 | 1.61 |
| encode/pass:shadow_point_light/max | ms | 0.77 | 1.34 | 0.71 |
| encode/point_shadow_cache_copies/max | ms | 0.07 | 0.67 | 0.11 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.58 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.32 | 28.43 | 8.87 |
| gpu/timed/ms | ms | 7.63 | 9.53 | 7.66 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 9 | 27 | 0 |
| lamp_cache/copies | count | 0 | 15 | 0 |
| lamp_cache/deferred_faces | count | 0 | 14 | 0 |
| lamp_cache/lamps | count | 5 | 8 | 0 |
| lamp_cache/mover_faces | count | 0 | 12 | 0 |
| lamp_cache/movers | count | 92 | 418 | 0 |
| lamp_cache/queued | count | 0 | 40 | 0 |
| lamp_cache/ready | count | 5 | 8 | 0 |
| lamp_cache/render/copies | count | 8 | 15 | 0 |
| lamp_cache/render/drawn | count | 8 | 14 | 0 |
| lamp_cache/render/dropped_copies | count | 0 | 0 | 0 |
| lamp_cache/render/lamps | count | 6 | 8 | 0 |
| lamp_cache/render/loaded | count | 8 | 12 | 0 |
| lamp_cache/render/lost | count | 0 | 6 | 0 |
| lamp_cache/render/views | count | 12 | 35 | 0 |
| lamp_cache/stock_lamps | count | 0 | 2 | 0 |
| lamp_cache/store_starved | count | 0 | 8 | 0 |
| lamp_cache/store_used | count | 1 | 24 | 0 |
| lamp_cache/views | count | 0 | 35 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2621 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 240 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 752 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 240 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 752 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1298 | 1399 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3302 | 5148 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1298 | 1399 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3302 | 5148 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 22 | 100 | 19 |
| phases/shadow/bins | count | 1711 | 2650 | 1494 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3675 | 5888 | 3038 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 48 | 134 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2630 | 3268 | 2345 |
| physics/items | count | 137 | 476 | 19 |
| physics/pending | count | 0 | 218 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 22 | 338 | 37 |
| preprocess/bind_groups_created | count | 22 | 338 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 134 | 283 | 127 |
| process/cpu_usage | % | 23.66 | 38.46 | 27.40 |
| process/mem_usage | GiB | 5.42 | 5.60 | 5.60 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 0 | 0 | 0 |
| scene/batch/masked/count | count | 0 | 0 | 0 |
| scene/batch/masked/high_water | count | 0 | 0 | 0 |
| scene/batch/masked/live | count | 0 | 0 | 0 |
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
| scene/batch/opaque/capacity | count | 0 | 0 | 0 |
| scene/batch/opaque/count | count | 0 | 0 | 0 |
| scene/batch/opaque/high_water | count | 0 | 0 | 0 |
| scene/batch/opaque/live | count | 0 | 0 | 0 |
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
| scene/batches/count | count | 0 | 0 | 0 |
| scene/cells/resident | count | 0 | 0 | 0 |
| scene/chains/high_water | count | 0 | 0 | 0 |
| scene/chains/live | count | 0 | 0 | 0 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 0 | 0 | 0 |
| scene/slots/batch_capacity | count | 0 | 0 | 0 |
| scene/slots/batch_capacity_bytes | bytes | 0 | 0 | 0 |
| scene/slots/batch_live | count | 0 | 0 | 0 |
| scene/slots/scene_bump | count | 0 | 0 | 0 |
| scene/slots/scene_live | count | 0 | 0 | 0 |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 13 | 130 | 9 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 165 | 216 | 205 |
| skin/joint_bytes_written/bytes | bytes | 279.8 k | 811.6 k | 441.9 k |
| skin/joints_written/count | count | 4372 | 12.7 k | 6904 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4941 | 5011 | 5011 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 429 | 528 | 396 |
| statics/materials_bound | count | 429 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.13 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 323 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.41 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1126 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.04 | 42.94 | 32.09 |
| system/mem_usage | % | 54.11 | 54.83 | 54.82 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2910 | 3016 | 3016 |
| textures/images | count | 2691 | 2740 | 2740 |
| views/all/batch_sets | count | 74 | 151 | 71 |
| views/all/bins | count | 4927 | 5659 | 5002 |
| views/all/count | count | 9 | 41 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.0 k | 18.1 k | 10.7 k |
| views/all/pipelines | count | 95 | 177 | 67 |
| views/all/pipelines_distinct | count | 64 | 74 | 63 |
| views/all/work_items | count | 10.6 k | 18.2 k | 11.0 k |
| views/cascade/batch_sets | count | 8 | 27 | 19 |
| views/cascade/bins | count | 1449 | 1559 | 1494 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3302 | 4452 | 3038 |
| views/cascade/pipelines | count | 22 | 27 | 19 |
| views/cascade/work_items | count | 329 | 4448 | 3038 |
| views/lamp_face/batch_sets | count | 0 | 80 | 0 |
| views/lamp_face/bins | count | 313 | 1199 | 0 |
| views/lamp_face/count | count | 6 | 38 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 26 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 340 | 1451 | 0 |
| views/lamp_face/pipelines | count | 24 | 108 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 1150 | 0 |
| views/main/batch_sets | count | 51 | 53 | 52 |
| views/main/bins | count | 2902 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7924 | 13.1 k | 7614 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8224 | 13.3 k | 7922 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.0 k | 37.9 k | 30.8 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 58.67 | 271 | 310.4 k |
| anim/events/count | count | 0.10 | 3 | 539 |
| assets/Image/added | count | 0.11 | 33 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.77 | 152 | 4061 |
| assets/Mesh/removed | count | 0.92 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 457 |
| assets/StandardMaterial/modified | count | 4.10 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 60 | 394 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.88 | 1299 | 4675 |
| encode//count | count | 56.88 | 133 | 300.9 k |
| encode/bind_walk/init_textures | count | 19.5 k | 63.2 k | 103.08 M |
| encode/bind_walk/init_textures_skipped | count | 65.7 k | 141.3 k | 347.63 M |
| encode/bind_walk/inits | count | 517 | 791 | 2.74 M |
| encode/bind_walk/inits_skipped | count | 110 | 157 | 579.9 k |
| encode/bind_walk/merge_textures | count | 19.4 k | 63.2 k | 102.75 M |
| encode/bind_walk/merge_textures_skipped | count | 5417 | 6012 | 28.66 M |
| encode/bind_walk/merges | count | 127 | 233 | 669.6 k |
| encode/bind_walk/merges_skipped | count | 85.33 | 128 | 451.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5290 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5290 |
| encode/pass:early prepass/count | count | 1 | 1 | 5290 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5290 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5290 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5290 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5290 |
| encode/pass:shadow_point_light/count | count | 7.46 | 26 | 39.5 k |
| encode/point_shadow_cache_copies/count | count | 0.47 | 1 | 2460 |
| encode/queue/command_buffers | count | 74.82 | 170 | 395.8 k |
| encode/queue/submits | count | 2.02 | 4 | 10.7 k |
| encode/queue/write_buffer/bytes | bytes | 1.84 M | 2.97 M | 9.75 G |
| encode/queue/write_buffer/calls | count | 289 | 522 | 1.53 M |
| encode/queue/write_buffer_with/bytes | bytes | 130.3 k | 3.50 M | 689.05 M |
| encode/queue/write_buffer_with/calls | count | 11.29 | 465 | 59.7 k |
| encode/queue/write_texture/bytes | bytes | 11.9 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.11 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5290 |
| physics/inserts | count | 0.43 | 110 | 2269 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.44 | 145 | 2335 |
| physics/steps | count | 0.74 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 2 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6399 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1403 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.1 ms; slowest 55, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.40 | 2396.7 | main/busy/ms 2369, stage/main/post_update 2327, stage/main/update 32.93, stage/main/pre_update 4.74, stage/main/last 0.92, surface/advance_cpu_ms 0.73, stage/main/first 0.56, stream/drive/ms 0.53, stage/main/fixed_main_loop 0.36, surface/cell_upload_ms 0.28, stream/tick/ms 0.18, bigspace/total/ms 0.10 |
| warm-up | 3.12 | 722.3 | main/busy/ms 2507, stage/main/post_update 2350, render/thread/ms 692, render/busy/ms 525, stage/render/prepare_assets 473, stage/render/prepare_views 164, render/acquire/ms 164, stage/main/update 142, stage/render/create_views 30.17, stage/render/render 12.55, encode//sum 9.05, render/graph/ms 8.38 |
| warm-up | 3.23 | 111.7 | render/thread/ms 787, render/busy/ms 563, stage/render/prepare_assets 500, stage/render/prepare_views 165, render/acquire/ms 165, main/busy/ms 157, stage/main/update 113, pipelines/compile_ms 87.24, stage/render/render 72.97, render/present/ms 59.36, pipelines/driver_ms 46.82, pipelines/front_ms 40.42 |
| warm-up | 3.28 | 46.3 | pipelines/compile_ms 374, pipelines/front_ms 282, render/thread/ms 121, pipelines/driver_ms 92.46, stage/render/render 80.15, main/busy/ms 66.03, render/busy/ms 62.65, render/present/ms 57.69, stage/main/post_update 50.50, pipelines/wait_ms 39.42, stage/render/prepare_assets 28.40, terrain/update/ms 23.00 |
| warm-up | 3.36 | 85.0 | pipelines/wait_ms 3220, pipelines/compile_ms 1137, pipelines/front_ms 1042, render/thread/ms 104, render/busy/ms 101, pipelines/driver_ms 95.25, main/busy/ms 81.58, stage/main/post_update 53.19, stage/render/prepare_resources 26.07, stage/render/render 24.55, terrain/update/ms 22.42, render/graph/ms 21.62 |
| warm-up | 3.43 | 70.8 | pipelines/wait_ms 4290, pipelines/compile_ms 1445, pipelines/front_ms 1375, render/thread/ms 148, render/busy/ms 146, main/busy/ms 132, pipelines/driver_ms 69.82, stage/main/update 60.74, stage/render/prepare_resources 50.15, stage/main/post_update 24.38, terrain/update/ms 23.88, stage/render/prepare_assets 23.68 |
| warm-up | 3.53 | 96.5 | pipelines/compile_ms 1677, pipelines/front_ms 1596, pipelines/wait_ms 1431, render/thread/ms 135, render/busy/ms 132, main/busy/ms 112, pipelines/driver_ms 80.07, stage/render/prepare_assets 63.44, stage/main/update 61.65, stage/render/prepare_resources 26.46, stage/main/fixed_main_loop 23.34, stage/render/prepare_write_phase_buffers 19.34 |
| warm-up | 3.58 | 48.6 | pipelines/compile_ms 1724, pipelines/front_ms 1641, pipelines/wait_ms 330, render/thread/ms 112, render/busy/ms 110, stage/render/prepare_assets 88.50, pipelines/driver_ms 82.64, main/busy/ms 31.97, stage/main/post_update 15.62, stage/render/render 13.71, render/graph/ms 11.28, terrain/update/ms 10.49 |
| warm-up | 3.61 | 34.3 | pipelines/compile_ms 1302, pipelines/front_ms 1249, render/thread/ms 79.13, render/busy/ms 75.77, pipelines/driver_ms 52.92, stage/render/prepare_assets 43.47, main/busy/ms 30.08, stage/render/render 26.89, render/graph/ms 23.63, stage/render/graph_submit 21.24, encode/submit 16.80, stage/main/post_update 15.12 |
| warm-up | 3.77 | 79.8 | pipelines/compile_ms 1440, pipelines/front_ms 1387, render/thread/ms 56.15, main/busy/ms 55.75, render/busy/ms 54.59, pipelines/driver_ms 52.81, stage/main/update 44.81, stream/drive/ms 38.39, stage/render/prepare_assets 34.56, pipelines/wait_ms 12.41, stage/render/render 11.22, render/graph/ms 9.40 |
| warm-up | 3.81 | 30.8 | pipelines/compile_ms 84.77, pipelines/front_ms 80.18, render/thread/ms 31.62, render/busy/ms 29.90, main/busy/ms 27.92, stage/main/update 15.00, stage/render/render 13.92, render/graph/ms 11.63, stream/drive/ms 10.90, stage/render/prepare_assets 10.64, stage/main/post_update 8.90, stage/render/graph_submit 7.95 |
| warm-up | 3.89 | 58.7 | pipelines/compile_ms 460, pipelines/front_ms 445, main/busy/ms 66.80, render/thread/ms 52.23, render/busy/ms 47.12, stage/main/update 46.29, stream/drive/ms 27.32, stage/render/prepare_assets 27.09, stage/render/render 16.37, pipelines/driver_ms 15.64, stage/main/post_update 11.94, render/graph/ms 10.85 |
| warm-up | 3.96 | 69.0 | pipelines/compile_ms 308, pipelines/front_ms 298, render/thread/ms 82.47, render/busy/ms 80.16, main/busy/ms 64.89, stage/render/prepare_assets 63.25, stage/main/update 46.17, stream/drive/ms 28.68, stage/render/render 12.88, stage/main/post_update 10.24, render/graph/ms 10.23, pipelines/driver_ms 9.86 |
| warm-up | 4.03 | 62.4 | pipelines/compile_ms 763, pipelines/front_ms 725, render/thread/ms 92.01, render/busy/ms 89.58, main/busy/ms 75.05, stage/render/prepare_assets 61.63, stage/main/update 53.55, pipelines/driver_ms 37.68, stage/render/render 21.34, render/graph/ms 18.34, stage/render/graph_submit 15.51, stage/main/post_update 12.70 |
| warm-up | 4.10 | 77.4 | pipelines/compile_ms 1590, pipelines/front_ms 1526, render/thread/ms 88.18, render/busy/ms 86.02, main/busy/ms 70.67, pipelines/driver_ms 64.14, stage/render/prepare_assets 52.84, stage/main/update 46.41, stage/render/render 23.34, render/graph/ms 20.55, stage/render/graph_submit 17.53, stage/main/post_update 15.48 |
| warm-up | 4.16 | 53.6 | pipelines/compile_ms 984, pipelines/front_ms 949, render/thread/ms 94.66, render/busy/ms 90.25, main/busy/ms 70.31, stage/render/render 41.53, stage/render/prepare_assets 35.10, pipelines/driver_ms 34.98, render/graph/ms 31.30, stage/main/post_update 27.38, stage/main/update 20.13, pipelines/wait_ms 17.20 |
| warm-up | 4.21 | 55.1 | pipelines/compile_ms 1322, pipelines/front_ms 1283, render/thread/ms 93.29, render/busy/ms 88.98, pipelines/wait_ms 82.26, main/busy/ms 75.08, pipelines/driver_ms 39.14, stage/render/render 38.62, render/graph/ms 28.44, stage/main/post_update 23.36, stage/main/update 20.76, stage/main/pre_update 15.29 |
| warm-up | 4.33 | 114.1 | pipelines/compile_ms 3062, pipelines/front_ms 2961, pipelines/wait_ms 1257, render/thread/ms 141, render/busy/ms 139, pipelines/driver_ms 101, main/busy/ms 61.75, stage/render/prepare_assets 46.37, stage/render/prepare_write_phase_buffers 36.08, stage/render/prepare_resources 23.36, stage/main/update 22.86, stage/main/post_update 21.06 |
| warm-up | 4.41 | 83.2 | pipelines/compile_ms 2430, pipelines/front_ms 2341, pipelines/wait_ms 1722, render/thread/ms 179, render/busy/ms 177, pipelines/driver_ms 88.47, main/busy/ms 56.02, stage/render/prepare_write_phase_buffers 50.49, stage/render/prepare_resources 44.88, stage/render/prepare_assets 41.01, stage/main/post_update 22.12, stage/main/update 20.49 |
| warm-up | 4.45 | 35.6 | pipelines/compile_ms 819, pipelines/front_ms 791, pipelines/wait_ms 633, render/thread/ms 110, render/busy/ms 108, main/busy/ms 87.97, stage/main/update 66.61, stream/drive/ms 57.32, stage/render/prepare_write_phase_buffers 28.54, pipelines/driver_ms 28.36, stage/render/prepare_resources 28.31, stage/render/render 19.14 |
| warm-up | 4.55 | 102.2 | pipelines/compile_ms 2232, pipelines/front_ms 2173, pipelines/wait_ms 1554, main/busy/ms 86.87, stage/main/update 65.49, render/thread/ms 65.05, render/busy/ms 63.49, pipelines/driver_ms 58.38, stream/drive/ms 56.05, stage/render/prepare_assets 38.79, stage/render/render 16.41, render/graph/ms 14.31 |
| warm-up | 4.69 | 116.6 | pipelines/compile_ms 578, pipelines/front_ms 559, pipelines/wait_ms 284, main/busy/ms 98.00, stage/main/update 80.76, render/thread/ms 56.88, render/busy/ms 55.36, stream/drive/ms 42.73, stage/render/prepare_assets 30.81, pipelines/driver_ms 18.53, stage/render/render 16.71, render/graph/ms 14.62 |
| warm-up | 4.72 | 37.8 | pipelines/compile_ms 1017, pipelines/front_ms 985, main/busy/ms 79.54, render/thread/ms 72.37, render/busy/ms 66.82, stage/main/update 56.91, stream/drive/ms 47.82, pipelines/driver_ms 32.05, encode//sum 29.25, stage/render/prepare_assets 27.52, stage/render/render 21.14, render/graph/ms 18.79 |
| warm-up | 4.79 | 61.9 | pipelines/compile_ms 853, pipelines/front_ms 825, main/busy/ms 85.76, render/thread/ms 60.49, stage/main/update 58.43, render/busy/ms 54.87, stream/drive/ms 47.52, encode//sum 30.29, pipelines/driver_ms 27.56, stage/render/render 20.85, render/graph/ms 18.66, stage/main/post_update 18.39 |
| warm-up | 4.81 | 28.2 | pipelines/compile_ms 234, pipelines/front_ms 228, main/busy/ms 70.34, stage/main/update 48.75, render/thread/ms 48.37, render/busy/ms 46.93, stream/drive/ms 34.51, stage/render/prepare_assets 15.76, stage/main/post_update 15.29, stage/render/render 14.71, render/graph/ms 12.74, encode//sum 11.37 |
| warm-up | 4.87 | 59.2 | pipelines/compile_ms 999, pipelines/front_ms 964, main/busy/ms 70.72, stage/main/update 55.55, render/thread/ms 49.45, render/busy/ms 48.09, pipelines/driver_ms 34.65, stream/drive/ms 34.49, stage/render/prepare_assets 19.43, stage/render/render 12.44, encode//sum 10.92, render/graph/ms 10.38 |
| warm-up | 4.91 | 33.6 | pipelines/compile_ms 1194, pipelines/front_ms 1153, render/thread/ms 53.36, render/busy/ms 52.01, main/busy/ms 41.57, pipelines/driver_ms 40.93, stage/main/update 22.80, stage/render/prepare_assets 22.79, stage/main/post_update 11.91, stage/render/render 11.76, encode//sum 9.82, render/graph/ms 9.81 |
| warm-up | 4.96 | 31.7 | render/thread/ms 47.51, render/busy/ms 46.09, pipelines/compile_ms 33.68, pipelines/front_ms 32.24, main/busy/ms 29.18, stage/render/render 16.11, render/graph/ms 14.25, stage/render/prepare_assets 13.94, encode//sum 13.75, stage/main/update 12.37, stage/main/post_update 10.24, stage/render/graph_submit 9.45 |
| warm-up | 5.01 | 49.7 | render/thread/ms 73.07, render/busy/ms 71.70, main/busy/ms 41.23, stage/render/prepare_assets 22.11, stage/main/post_update 17.78, stage/main/update 17.57, stage/render/render 17, render/graph/ms 14.87, encode//sum 14.19, stage/render/prepare_flush 13.10, stage/render/graph_submit 9.88, stream/drive/ms 6.27 |
| warm-up | 5.05 | 35.3 | pipelines/compile_ms 424, pipelines/front_ms 407, render/thread/ms 75.41, render/busy/ms 72.33, main/busy/ms 48.62, stage/main/post_update 22.29, stage/main/update 19.31, stage/render/render 18.59, pipelines/driver_ms 17.51, render/graph/ms 16.32, stage/render/prepare_assets 15.75, encode//sum 13.77 |
| warm-up | 5.11 | 61.6 | pipelines/compile_ms 1595, pipelines/front_ms 1549, render/thread/ms 75.48, render/busy/ms 72.39, pipelines/driver_ms 45.70, main/busy/ms 35.64, stage/render/prepare_assets 33.30, stage/render/render 20.09, render/graph/ms 17.86, stage/main/post_update 15.08, stage/main/update 13.91, stage/render/graph_submit 12.53 |
| warm-up | 5.14 | 29.9 | pipelines/compile_ms 1460, pipelines/front_ms 1416, render/thread/ms 75.56, render/busy/ms 72.28, pipelines/driver_ms 44.31, main/busy/ms 43.46, stage/render/prepare_assets 33.44, stage/render/render 28.25, stage/main/update 26.39, render/graph/ms 25.58, encode//sum 21.82, stage/render/graph_submit 20.55 |
| warm-up | 5.18 | 38.4 | pipelines/compile_ms 745, pipelines/front_ms 716, render/thread/ms 60.02, render/busy/ms 56.74, main/busy/ms 42.81, pipelines/driver_ms 28.84, stage/main/update 25.35, stage/render/render 24.97, encode//sum 22.56, render/graph/ms 22.25, stage/render/prepare_assets 19.71, stage/render/graph_submit 17.25 |
| warm-up | 5.23 | 30.7 | render/thread/ms 46.04, pipelines/compile_ms 44.67, render/busy/ms 44.59, pipelines/front_ms 41.77, main/busy/ms 23.24, stage/render/prepare_assets 19.07, stage/render/render 16.43, render/graph/ms 14.45, encode//sum 10.76, stage/main/post_update 10.54, stage/render/graph_submit 10.12, stage/main/update 7.54 |
| warm-up | 5.28 | 29.9 | render/thread/ms 44.74, render/busy/ms 43.43, main/busy/ms 25.03, stage/render/render 19.16, render/graph/ms 17.26, stage/render/prepare_assets 15.49, encode//sum 13.86, stage/main/post_update 11.43, stage/render/graph_submit 11.15, pipelines/compile_ms 9.19, pipelines/front_ms 8.49, stage/main/update 8.38 |
| warm-up | 5.33 | 31.0 | pipelines/compile_ms 1109, pipelines/front_ms 1088, render/thread/ms 42.95, render/busy/ms 41.66, stage/render/prepare_assets 23.01, pipelines/driver_ms 21.44, main/busy/ms 20.16, stage/render/render 12.09, render/graph/ms 10.36, encode//sum 9.07, stage/main/post_update 8.24, stage/render/graph_submit 7.34 |
| warm-up | 5.38 | 37.4 | render/thread/ms 49.92, render/busy/ms 48.81, main/busy/ms 29.11, stage/render/prepare_assets 25.30, stage/render/render 18.13, render/graph/ms 16.49, stage/main/post_update 13.89, encode//sum 13.36, stage/render/graph_submit 11.41, stage/main/update 10.31, encode/submit 7.34, encode/pass:shadow_point_light/sum 6.69 |
| warm-up | 5.57 | 36.0 | pipelines/compile_ms 96.27, pipelines/front_ms 90.17, render/thread/ms 52.70, render/busy/ms 51.23, main/busy/ms 45.98, stage/main/update 33.83, stage/render/render 32.75, render/graph/ms 30.82, stream/drive/ms 28.50, stage/render/camera_driver 20.09, encode//sum 11.04, stage/render/graph_submit 10.60 |
| warm-up | 5.73 | 27.6 | pipelines/compile_ms 889, pipelines/front_ms 863, render/thread/ms 41.52, render/busy/ms 40.03, pipelines/driver_ms 25.97, main/busy/ms 23.72, stage/render/render 17.13, render/graph/ms 15.01, stage/render/prepare_assets 14.88, stage/main/post_update 11.62, stage/render/graph_submit 10.79, encode//sum 10.03 |
| warm-up | 5.77 | 28.4 | pipelines/compile_ms 302, pipelines/front_ms 293, render/thread/ms 41.17, render/busy/ms 39.90, main/busy/ms 30.93, stage/render/prepare_assets 15.91, stage/render/render 15.73, stage/main/post_update 14.98, render/graph/ms 14.02, encode//sum 9.78, pipelines/driver_ms 9.74, stage/render/graph_submit 9.74 |
| warm-up | 5.89 | 28.9 | pipelines/compile_ms 829, pipelines/front_ms 803, main/busy/ms 36.95, render/thread/ms 30.20, render/busy/ms 29.13, pipelines/driver_ms 26.37, stage/main/update 24.19, stream/drive/ms 17.09, stage/render/render 13.56, render/graph/ms 11.79, encode//sum 11.23, stage/main/post_update 9.28 |
| warm-up | 5.97 | 30.7 | pipelines/compile_ms 143, pipelines/front_ms 139, render/thread/ms 45.10, render/busy/ms 43.23, main/busy/ms 32.07, stage/render/render 20.32, render/graph/ms 17.86, stage/main/update 15.12, encode//sum 13.54, stage/main/post_update 13.19, stage/render/graph_submit 12.78, encode/submit 9.53 |
| warm-up | 6.02 | 31.5 | pipelines/compile_ms 160, pipelines/front_ms 157, render/thread/ms 38.12, render/busy/ms 37.05, main/busy/ms 24.73, stage/render/prepare_assets 13.70, stage/render/render 12.07, encode//sum 10.59, render/graph/ms 10.56, stage/main/post_update 10.33, stage/main/update 8.42, stage/render/graph_submit 6.98 |
| warm-up | 6.24 | 30.2 | pipelines/compile_ms 136, pipelines/front_ms 132, main/busy/ms 42.15, render/thread/ms 39.11, render/busy/ms 37.64, stage/main/update 19.16, stage/render/render 18.13, render/graph/ms 15.82, stage/main/post_update 15.24, encode//sum 11.52, stage/render/graph_submit 9.80, encode/pass:shadow_point_light/sum 7.42 |
| warm-up | 6.30 | 34.5 | render/thread/ms 42.01, render/busy/ms 40.92, main/busy/ms 31.69, pipelines/compile_ms 28.13, pipelines/front_ms 26.37, stage/render/prepare_assets 19.75, stage/main/update 15.08, stage/render/render 11.46, stage/main/post_update 10.58, render/graph/ms 9.94, encode//sum 9.17, encode/pass:shadow_point_light/sum 7.76 |
| warm-up | 6.38 | 30.9 | render/thread/ms 41.86, render/busy/ms 40.88, main/busy/ms 38.30, stage/main/update 19.30, stage/render/prepare_assets 16.41, stage/main/post_update 12.13, stage/render/render 11.75, render/graph/ms 10.31, encode//sum 8.34, encode/pass:shadow_point_light/sum 7.50, stage/render/graph_submit 6.17, stage/render/camera_driver 4.02 |
| warm-up | 6.41 | 30.7 | render/thread/ms 49.45, render/busy/ms 48.34, main/busy/ms 36.57, stage/render/prepare_assets 19.28, stage/main/update 17.67, encode//sum 16.67, stage/render/render 14.65, stage/main/post_update 13.96, render/graph/ms 12.87, encode/pass:shadow_point_light/sum 8.91, stage/render/graph_submit 8.19, stage/render/camera_driver 4.53 |
| warm-up | 6.45 | 30.2 | pipelines/compile_ms 135, pipelines/front_ms 133, render/thread/ms 44.13, render/busy/ms 42.95, main/busy/ms 22.95, stage/render/prepare_assets 22.31, stage/render/render 12.38, stage/main/post_update 10.98, render/graph/ms 10.74, encode//sum 9.16, stage/main/update 8.47, stage/render/graph_submit 7.39 |
| warm-up | 6.50 | 27.9 | render/thread/ms 48.04, render/busy/ms 46.86, main/busy/ms 37.77, stage/main/update 21.20, stage/render/prepare_assets 19.95, stage/render/render 16.91, render/graph/ms 15.10, encode//sum 13.92, stream/drive/ms 13.72, encode/pass:shadow_point_light/sum 12.93, stage/main/post_update 12.87, pipelines/compile_ms 10.48 |
| warm-up | 6.73 | 28.3 | render/thread/ms 33.94, main/busy/ms 33.58, render/busy/ms 32.37, stage/main/update 17.87, stage/render/render 15.19, render/graph/ms 13.27, encode//sum 10.91, stage/main/post_update 10.03, stage/render/prepare_assets 9.81, stage/render/graph_submit 9.68, encode/pass:shadow_point_light/sum 8.40, stream/drive/ms 7.57 |
| measure | 18.19 | 30.7 | main/busy/ms 39.20, render/thread/ms 28.12, render/busy/ms 26.91, stage/main/update 24.66, encode//sum 21.79, stream/drive/ms 20.34, stage/render/render 19.21, render/graph/ms 17.67, stage/render/graph_submit 12.56, stage/main/post_update 11.32, encode/submit 8.31, encode/pass:shadow_point_light/sum 8.02 |
| measure | 23.25 | 32.0 | encode//sum 68.30, render/thread/ms 50.95, render/busy/ms 49.83, main/busy/ms 33.46, stage/render/render 27.06, render/graph/ms 25.31, stage/render/graph_submit 18.38, stage/main/post_update 16.30, encode/pass:shadow_point_light/sum 15.12, stage/main/update 13.07, stage/render/prepare_assets 9.93, encode/finish_wall 9.69 |
| measure | 27.48 | 29.1 | encode//sum 83.70, render/thread/ms 47.95, render/busy/ms 46.74, main/busy/ms 30.55, stage/render/render 28.32, render/graph/ms 26.43, stage/render/graph_submit 20.52, stage/main/post_update 15.90, encode/pass:shadow_point_light/sum 15.55, encode/finish_wall 11.57, stage/main/update 10.91, encode/submit 9.06 |
| measure | 59.02 | 28.5 | main/busy/ms 36.78, render/thread/ms 36.48, render/busy/ms 34.99, stage/main/post_update 22.67, encode//sum 20.68, stage/render/render 19.73, render/graph/ms 17.72, stage/render/graph_submit 12.72, stage/main/update 9.37, encode/submit 7.92, stage/render/prepare_assets 5.33, encode/finish_wall 4.98 |
| measure | 59.05 | 27.2 | render/thread/ms 38.72, render/busy/ms 37.49, main/busy/ms 29.64, encode//sum 17.84, stage/main/post_update 16.85, stage/render/prepare_assets 14.34, stage/render/render 13.93, render/graph/ms 12.39, stage/render/graph_submit 8.77, stage/main/update 7.77, encode/submit 4.87, encode/finish_wall 3.99 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.cache.enabled=on --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=lampcache-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 2%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (250 samples): GPU load **67%** (min 38%) · 2625 MHz (min 2625) · memory 8101 MHz · 70.1 W · 74 °C (max 80) · VRAM 7016 MiB · throttled 100% of samples

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
- `sf.graphics_preset=custom (preset: derived from its members)`
- `sg.effects=high (preset: level requested)`
- `sg.lighting=high (preset: level requested)`
- `sg.post=custom (preset: derived from its members)`
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
- `shadow.lamp.cache.enabled=on (command line: --set)`
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
