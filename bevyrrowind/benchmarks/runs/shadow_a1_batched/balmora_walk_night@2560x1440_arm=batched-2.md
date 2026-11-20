# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.batched_pass=on (command line: --set); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6576 frames over 65.6 s · **median 9.83 ms (102 fps)** · mean 9.98 · p95 11.46 · p99 13.47 (1% low 74 fps) · worst 32.44 ms · 27 spikes over 1.5× median · 21.9 stutters/min (work 25.6/min)
- warm-up: 353 frames over 9.1 s, worst 2453.3 ms
- hitches (> 25.0 ms): **2** measured, 51 in warm-up
- timed GPU passes: 6.67 ms of a 9.98 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.83 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.94/min, limit 1/min
- pass worst frame 32.44 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.991 | 2.030 | 2.063 | 0.98 | 0.171 |
| main_opaque_pass_3d | 1.250 | 1.275 | 1.570 | 0.98 | 0.072 |
| early prepass | 0.503 | 0.513 | 0.729 | 0.98 | 0.091 |
| bloom | 0.311 | 0.317 | 0.325 | 0.98 | 0.063 |
| atmosphere_luts | 0.306 | 0.312 | 0.376 | 0.98 | 0.003 |
| water_velocity | 0.221 | 0.225 | 0.246 | 0.98 | 0.002 |
| water_sspr | 0.206 | 0.210 | 0.211 | 0.98 | 0.002 |
| batched_point_shadows | 0.199 | 0.356 | 0.528 | 0.56 | 0.135 |
| ssao | 0.184 | 0.188 | 0.191 | 0.98 | 0.008 |
| upscaling | 0.127 | 0.129 | 0.141 | 0.98 | 0.004 |
| atmos_froxel_inject | 0.125 | 0.127 | 0.129 | 0.98 | 0.001 |
| water_fft | 0.121 | 0.123 | 0.125 | 0.98 | 0.003 |
| uniform_allocation | 0.116 | 0.119 | 0.145 | 0.98 | 0.023 |
| clouds_composite | 0.103 | 0.105 | 0.108 | 0.98 | 0.002 |
| bin_unpacking | 0.094 | 0.096 | 0.120 | 0.98 | 0.017 |
| gpu_instance_materialization | 0.093 | 0.095 | 0.099 | 0.98 | 0.004 |
| tonemapping | 0.085 | 0.087 | 0.091 | 0.98 | 0.004 |
| main_transmissive_pass_3d | 0.071 | 0.072 | 0.147 | 0.98 | 0.008 |
| clustering | 0.063 | 0.064 | 0.067 | 0.98 | 0.018 |
| early_mesh_preprocessing | 0.055 | 0.056 | 0.074 | 0.98 | 0.004 |
| clouds_trace | 0.055 | 0.056 | 0.078 | 0.98 | 0.001 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.98 | 0.002 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.98 | 0.001 |
| lightprobe_irradiance_map | 0.048 | 0.415 | 0.416 | 0.12 | 0.001 |
| surface_trail | 0.037 | 0.056 | 0.057 | 0.65 | 0.002 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 0.98 | 0.001 |
| batched_point_shadows/uniform_allocation | 0.028 | 0.051 | 0.080 | 0.56 | 0.012 |
| batched_point_shadows/early_mesh_preprocessing | 0.027 | 0.048 | 0.070 | 0.56 | 0.006 |
| lightprobe_radiance_map | 0.025 | 0.220 | 0.231 | 0.12 | 0.004 |
| water_snapshot | 0.024 | 0.024 | 0.028 | 0.98 | 0.002 |
| range_unpacking | 0.024 | 0.024 | 0.031 | 0.98 | 0.003 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.017 | 0.017 | 0.026 | 0.98 | 0.005 |
| night_sky | 0.016 | 0.017 | 0.042 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.016 | 0.017 | 0.031 | 0.98 | 0.188 |
| batched_point_shadows/bin_unpacking | 0.016 | 0.028 | 0.044 | 0.56 | 0.008 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 0.98 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.98 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.98 | 0.003 |
| batched_point_shadows/range_unpacking | 0.004 | 0.007 | 0.007 | 0.56 | 0.002 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.98 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.98 | 0.003 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.707 | 10.079 | 18.40 | 57256.1 |
| render/busy/ms | 8.231 | 9.606 | 17.98 | 54125.6 |
| main/busy/ms | 8.075 | 9.379 | 30.31 | 53103.4 |
| stage/render/render | 5.212 | 6.082 | 12.09 | 34271.2 |
| render/graph/ms | 4.584 | 5.417 | 10.68 | 30146.8 |
| stage/main/post_update | 4.573 | 5.465 | 9.61 | 30072.4 |
| stage/render/graph_submit | 3.338 | 4.090 | 8.23 | 21950.3 |
| encode//sum | 3.269 | 3.953 | 6.37 | 21496.3 |
| encode/submit | 2.074 | 2.506 | 5.16 | 13636.8 |
| stage/main/update | 1.827 | 2.309 | 23.98 | 12017.1 |
| encode/finish_wall | 1.302 | 1.786 | 4.03 | 8564.4 |
| stage/render/camera_driver | 1.193 | 1.368 | 3.33 | 7846.9 |
| encode/pass:early prepass/sum | 0.829 | 1.070 | 1.98 | 5448.9 |
| encode/pass:shadow_directional_light/sum | 0.828 | 1.218 | 2.36 | 5444.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.826 | 1.085 | 1.53 | 5428.6 |
| stage/render/prepare_write_phase_buffers | 0.646 | 1.000 | 2.21 | 4246.7 |
| encode/pass:shadow_point_light/sum | 0.601 | 1.574 | 3.29 | 3954.9 |
| bigspace/total/ms | 0.560 | 1.329 | 2.37 | 3685.2 |
| stage/render/prepare_assets | 0.525 | 0.645 | 11.07 | 3455.1 |
| stage/main/fixed_main_loop | 0.521 | 0.990 | 4.13 | 3424.9 |
| stage/render/prepare_meshes | 0.426 | 0.541 | 3.89 | 2802.5 |
| stage/render/prepare_resources | 0.414 | 0.503 | 1.07 | 2721.5 |
| render/present/ms | 0.390 | 0.456 | 1.87 | 2562.6 |
| stage/main/pre_update | 0.375 | 0.476 | 2.05 | 2467.1 |
| stage/main/first | 0.351 | 0.450 | 5.50 | 2307.5 |
| stage/render/prepare_bind_groups | 0.329 | 0.554 | 3.76 | 2163.9 |
| controller/feed/ms | 0.322 | 0.406 | 0.81 | 2115.3 |
| stage/render/prepare_flush | 0.288 | 0.482 | 1.56 | 1893.2 |
| stage/render/prepare_batch_phases | 0.276 | 0.418 | 0.74 | 1815.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.266 | 0.419 | 0.66 | 1751.5 |
| stage/main/last | 0.250 | 0.300 | 1.82 | 1645.9 |
| bigspace/low_precision/ms | 0.241 | 0.750 | 1.99 | 1584.3 |
| stream/drive/ms | 0.205 | 0.293 | 10.08 | 1345.9 |
| stage/render/create_views | 0.194 | 0.257 | 0.84 | 1274.7 |
| stage/render/extract_commands | 0.170 | 0.200 | 0.45 | 1120.9 |
| bigspace/high_precision/ms | 0.168 | 0.693 | 1.84 | 1106.2 |
| anim/evaluate/ms | 0.159 | 0.268 | 2.14 | 1047.7 |
| anim/evaluate_cpu/ms | 0.152 | 0.194 | 0.50 | 999.1 |
| stream/tick/ms | 0.151 | 0.223 | 0.49 | 994.8 |
| stage/render/queue | 0.142 | 0.216 | 1.33 | 934.7 |
| stage/render/specialize | 0.138 | 0.230 | 1.83 | 907.0 |
| stage/render/prepare_views | 0.131 | 0.166 | 0.33 | 862.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.094 | 0.148 | 0.35 | 615.0 |
| render/acquire/ms | 0.086 | 0.111 | 0.29 | 568.0 |
| controller/move/ms | 0.081 | 0.276 | 0.89 | 531.0 |
| controller/write/ms | 0.073 | 0.100 | 0.32 | 481.6 |
| terrain/update/ms | 0.058 | 0.283 | 1.83 | 379.4 |
| stage/render/prepare_collect_phase_buffers | 0.045 | 0.072 | 0.23 | 297.8 |
| physics/step/ms | 0.044 | 0.117 | 0.55 | 290.9 |
| encode/point_shadow_cache_copies/sum | 0.034 | 0.079 | 0.42 | 220.3 |
| stage/render/phase_sort | 0.033 | 0.056 | 0.19 | 213.9 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 0.38 | 207.0 |
| pipelines/compile_ms | 0.026 | 0.000 | 81.77 | 173.0 |
| pipelines/front_ms | 0.025 | 0.000 | 79.18 | 167.5 |
| physics/insert/ms | 0.024 | 0.271 | 1.03 | 159.7 |
| stage/render/cleanup | 0.023 | 0.032 | 0.72 | 149.0 |
| terrain/select/ms | 0.021 | 0.250 | 0.48 | 138.4 |
| weather/map/ms | 0.017 | 0.025 | 0.18 | 114.3 |
| far/select/ms | 0.015 | 0.166 | 0.53 | 97.4 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.20 | 83.6 |
| anim/apply/ms | 0.012 | 0.021 | 0.15 | 76.7 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.20 | 69.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.09 | 69.6 |
| script/update/ms | 0.008 | 0.011 | 0.17 | 50.8 |
| controller/avoid/ms | 0.006 | 0.012 | 0.03 | 38.9 |
| stream/despawn/ms | 0.006 | 0.000 | 1.02 | 36.3 |
| script/input/ms | 0.005 | 0.006 | 0.12 | 32.0 |
| script/ui/ms | 0.004 | 0.006 | 0.12 | 28.1 |
| stream/spawn/ms | 0.003 | 0.000 | 1.12 | 21.7 |
| stream/upload/ms | 0.003 | 0.000 | 1.25 | 18.7 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.66 | 13.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 7.1 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.70 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.53 | 5.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 4.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 561 | 579 | 578 |
| anim/characters/count | count | 562 | 579 | 578 |
| anim/evaluated/count | count | 18 | 31 | 17 |
| controller/movers/count | count | 19 | 21 | 17 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1291 | 2262 | 583 |
| crowd/c0/meshes/count | count | 355 | 646 | 162 |
| crowd/c1/count | count | 147 | 155 | 148 |
| crowd/c1/joints/count | count | 5542 | 5942 | 5613 |
| crowd/c1/meshes/count | count | 1187 | 1272 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 161 | 215 | 204 |
| crowd/joints_extracted/count | count | 666 | 5577 | 505 |
| crowd/joints_uploaded/count | count | 3616 | 6186 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.40 | 1.13 | 0.36 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.20 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.20 | 0.01 |
| encode/finish_critical | ms | 1.02 | 3.29 | 0.86 |
| encode/pass:early prepass/max | ms | 0.83 | 1.98 | 0.77 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.83 | 1.53 | 0.62 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.35 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 0.66 | 0.32 |
| encode/pass:shadow_directional_light/max | ms | 0.81 | 2.36 | 0.86 |
| encode/pass:shadow_point_light/max | ms | 1.03 | 3.29 | 0.26 |
| encode/point_shadow_cache_copies/max | ms | 0.06 | 0.42 | 0.05 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.38 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.31 | 30.31 | 8.65 |
| gpu/timed/ms | ms | 7.61 | 8.56 | 7.84 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 9 | 24 | 0 |
| lamp_cache/copies | count | 8 | 16 | 0 |
| lamp_cache/deferred_faces | count | 0 | 16 | 0 |
| lamp_cache/lamps | count | 6 | 8 | 0 |
| lamp_cache/mover_faces | count | 8 | 12 | 0 |
| lamp_cache/movers | count | 141 | 298 | 0 |
| lamp_cache/queued | count | 0 | 40 | 0 |
| lamp_cache/ready | count | 6 | 8 | 0 |
| lamp_cache/render/copies | count | 9 | 16 | 0 |
| lamp_cache/render/drawn | count | 9 | 14 | 0 |
| lamp_cache/render/dropped_copies | count | 0 | 0 | 0 |
| lamp_cache/render/lamps | count | 7 | 8 | 0 |
| lamp_cache/render/loaded | count | 9 | 12 | 0 |
| lamp_cache/render/lost | count | 0 | 6 | 0 |
| lamp_cache/render/views | count | 14 | 33 | 0 |
| lamp_cache/stock_lamps | count | 0 | 2 | 0 |
| lamp_cache/store_starved | count | 0 | 8 | 0 |
| lamp_cache/store_used | count | 11 | 24 | 0 |
| lamp_cache/views | count | 10 | 33 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 157 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 335 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 157 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 335 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 765 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2478 | 3554 | 2455 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 765 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2478 | 3554 | 2455 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 27 | 80 | 8 |
| phases/shadow/bins | count | 868 | 1282 | 658 |
| phases/shadow/instance_batches | count | 4 | 10 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 106.5 k | 49.2 k |
| phases/shadow/items | count | 2336 | 2973 | 2014 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 50 | 110 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2650 | 3183 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 333 | 334 | 334 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 27 | 282 | 11 |
| preprocess/bind_groups_created | count | 22 | 274 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 8 | 5 |
| preprocess/bind_groups_reused | count | 132 | 242 | 123 |
| process/cpu_usage | % | 25 | 36.22 | 32.86 |
| process/mem_usage | GiB | 5.24 | 5.43 | 5.43 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2976 | 3166 | 3166 |
| scene/batch/masked/live | count | 2976 | 3166 | 3166 |
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
| scene/batch/opaque/high_water | count | 18.8 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 18.8 k | 19.9 k | 19.9 k |
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
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1591 | 1621 | 1621 |
| scene/chains/live | count | 1591 | 1621 | 1621 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 95 | 11 |
| skin/blocks_skipped/count | count | 146 | 204 | 193 |
| skin/blocks_walked/count | count | 14 | 25 | 11 |
| skin/joint_bytes_written/bytes | bytes | 47.1 k | 812.2 k | 33.5 k |
| skin/joints_written/count | count | 736 | 12.7 k | 524 |
| skin/upload_writes/count | count | 8 | 27 | 10 |
| started_load_count |  | 6438 | 6562 | 6562 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.05 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.95 | 46.69 | 30.73 |
| system/mem_usage | % | 56.30 | 57.18 | 57.05 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2666 | 2724 | 2724 |
| views/all/batch_sets | count | 85 | 139 | 66 |
| views/all/bins | count | 2929 | 3441 | 2948 |
| views/all/count | count | 18 | 42 | 3 |
| views/all/instance_batches | count | 12 | 18 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 204.8 k | 147.5 k |
| views/all/items | count | 8018 | 10.2 k | 7954 |
| views/all/pipelines | count | 98 | 151 | 66 |
| views/all/pipelines_distinct | count | 67 | 74 | 61 |
| views/all/work_items | count | 154.1 k | 214.5 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 33 | 8 |
| views/cascade/bins | count | 712 | 860 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2118 | 2427 | 2014 |
| views/cascade/pipelines | count | 24 | 27 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 12 | 55 | 0 |
| views/lamp_face/bins | count | 136 | 488 | 0 |
| views/lamp_face/count | count | 15 | 39 | 0 |
| views/lamp_face/culled | count | 0 | 12 | 0 |
| views/lamp_face/drawn | count | 9 | 23 | 0 |
| views/lamp_face/instance_batches | count | 0 | 2 | 0 |
| views/lamp_face/instance_draws | count | 0 | 24 | 0 |
| views/lamp_face/instance_slots | count | 0 | 8192 | 0 |
| views/lamp_face/items | count | 184 | 618 | 0 |
| views/lamp_face/pipelines | count | 24 | 84 | 0 |
| views/lamp_face/revealed | count | 0 | 3 | 0 |
| views/lamp_face/work_items | count | 64 | 8711 | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2014 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5908 | 7528 | 5940 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 104.6 k | 106.0 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 21.3 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 47.29 | 221 | 311.0 k |
| anim/events/count | count | 0.08 | 3 | 533 |
| assets/Image/added | count | 0.09 | 34 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 12 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.44 | 237 | 2874 |
| assets/Mesh/removed | count | 0.74 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.14 | 10 | 7481 |
| assets/StandardMaterial/removed | count | 0.06 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.23 | 1260 | 1489 |
| encode//count | count | 27.91 | 30 | 183.6 k |
| encode/bind_walk/init_textures | count | 9104 | 21.3 k | 59.87 M |
| encode/bind_walk/init_textures_skipped | count | 48.9 k | 65.5 k | 321.37 M |
| encode/bind_walk/inits | count | 601 | 770 | 3.95 M |
| encode/bind_walk/inits_skipped | count | 134 | 232 | 878.4 k |
| encode/bind_walk/merge_textures | count | 9041 | 21.3 k | 59.45 M |
| encode/bind_walk/merge_textures_skipped | count | 9124 | 19.1 k | 60.00 M |
| encode/bind_walk/merges | count | 123 | 205 | 807.6 k |
| encode/bind_walk/merges_skipped | count | 124 | 225 | 817.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6576 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6576 |
| encode/pass:early prepass/count | count | 1 | 1 | 6576 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6576 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6576 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6576 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6576 |
| encode/pass:shadow_point_light/count | count | 0.57 | 1 | 3728 |
| encode/point_shadow_cache_copies/count | count | 0.55 | 1 | 3623 |
| encode/queue/command_buffers | count | 39.04 | 43 | 256.8 k |
| encode/queue/submits | count | 2.01 | 4 | 13.2 k |
| encode/queue/write_buffer/bytes | bytes | 2.33 M | 3.41 M | 15.30 G |
| encode/queue/write_buffer/calls | count | 289 | 461 | 1.90 M |
| encode/queue/write_buffer_with/bytes | bytes | 124.2 k | 1.25 M | 816.68 M |
| encode/queue/write_buffer_with/calls | count | 10.30 | 720 | 67.7 k |
| encode/queue/write_texture/bytes | bytes | 2006 | 1.05 M | 13.19 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6576 |
| physics/inserts | count | 0.34 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 175 | 2335 |
| physics/steps | count | 0.60 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 1 | 3 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 31 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5148 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1129 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 53, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.45 | 2453.3 | main/busy/ms 2421, stage/main/post_update 2379, stage/main/update 31.20, stage/main/pre_update 6.77, stage/main/last 1.14, surface/advance_cpu_ms 0.77, stage/main/first 0.51, stage/main/fixed_main_loop 0.39, stream/drive/ms 0.35, surface/cell_upload_ms 0.30, stream/tick/ms 0.10, bigspace/total/ms 0.07 |
| warm-up | 3.20 | 744.0 | main/busy/ms 2560, stage/main/post_update 2406, render/thread/ms 705, render/busy/ms 567, stage/render/prepare_assets 522, stage/main/update 136, stage/render/prepare_views 136, render/acquire/ms 136, stage/render/create_views 29.15, stage/render/render 9.86, stage/main/pre_update 8.72, render/graph/ms 6.79 |
| warm-up | 3.25 | 54.7 | render/thread/ms 742, render/busy/ms 603, stage/render/prepare_assets 546, main/busy/ms 161, stage/render/prepare_views 136, render/acquire/ms 136, stage/main/update 110, pipelines/compile_ms 54.46, pipelines/front_ms 51.18, stage/main/post_update 39.77, stage/render/create_views 29.56, terrain/update/ms 21.28 |
| warm-up | 3.29 | 36.3 | pipelines/wait_ms 297, pipelines/compile_ms 206, pipelines/front_ms 191, main/busy/ms 112, render/thread/ms 71.53, render/busy/ms 65.56, stage/main/post_update 65.19, stage/main/update 39.32, stage/render/render 34.57, terrain/update/ms 29.01, render/graph/ms 27.77, stage/render/prepare_assets 25.52 |
| warm-up | 3.43 | 141.0 | pipelines/wait_ms 4506, pipelines/compile_ms 1210, pipelines/front_ms 1126, render/thread/ms 116, render/busy/ms 108, main/busy/ms 107, pipelines/driver_ms 83.95, stage/main/post_update 63.02, stage/render/prepare_resources 56.28, stage/main/update 38.43, stage/render/render 38.24, render/graph/ms 29.64 |
| warm-up | 3.49 | 59.8 | pipelines/wait_ms 4542, pipelines/compile_ms 1762, pipelines/front_ms 1659, render/thread/ms 139, render/busy/ms 135, main/busy/ms 128, pipelines/driver_ms 103, stage/main/update 77.92, stage/render/prepare_resources 67.26, stage/main/post_update 23.05, stage/render/render 20.63, stage/render/prepare_assets 18.16 |
| warm-up | 3.62 | 131.7 | pipelines/compile_ms 2262, pipelines/front_ms 2175, pipelines/wait_ms 456, render/thread/ms 131, main/busy/ms 130, render/busy/ms 129, pipelines/driver_ms 86.84, stage/main/update 81.52, stage/render/prepare_assets 74.23, stage/main/post_update 21.00, stage/render/render 19.04, render/graph/ms 16.67 |
| warm-up | 3.66 | 40.4 | pipelines/compile_ms 2108, pipelines/front_ms 2022, pipelines/wait_ms 130, render/thread/ms 112, render/busy/ms 110, encode//sum 89.44, pipelines/driver_ms 85.74, stage/render/prepare_assets 71.86, main/busy/ms 57.19, stage/main/post_update 37.47, stage/render/render 32.53, render/graph/ms 30.25 |
| warm-up | 3.69 | 29.7 | pipelines/compile_ms 866, pipelines/front_ms 808, pipelines/wait_ms 134, encode//sum 90.11, render/thread/ms 66.13, render/busy/ms 64.21, pipelines/driver_ms 58.01, main/busy/ms 49.34, stage/main/post_update 34.72, stage/render/prepare_assets 29.49, stage/render/render 29.31, render/graph/ms 27.01 |
| warm-up | 3.72 | 27.9 | pipelines/compile_ms 360, pipelines/front_ms 330, pipelines/wait_ms 131, render/thread/ms 53.88, render/busy/ms 52.09, main/busy/ms 40.96, pipelines/driver_ms 30.11, stage/render/prepare_assets 20.90, stage/main/post_update 20.44, stage/main/update 15.41, stage/render/prepare_write_phase_buffers 10.54, stage/render/render 10.14 |
| warm-up | 3.76 | 37.7 | main/busy/ms 99.13, stage/main/update 69.14, render/thread/ms 58.39, render/busy/ms 56.17, pipelines/compile_ms 43.82, pipelines/front_ms 41.48, stage/main/post_update 23.39, stage/render/prepare_assets 16.60, stage/render/render 12.93, stage/render/prepare_write_phase_buffers 10.83, render/graph/ms 10.43, terrain/update/ms 8.15 |
| warm-up | 3.86 | 106.7 | pipelines/compile_ms 1808, pipelines/front_ms 1723, pipelines/driver_ms 85.52, main/busy/ms 81.27, render/thread/ms 70.91, render/busy/ms 68.93, stage/main/update 61.96, stage/render/prepare_assets 44.81, pipelines/wait_ms 25.60, stage/render/render 13.97, render/graph/ms 11.79, stage/main/post_update 11.62 |
| warm-up | 3.91 | 43.1 | pipelines/compile_ms 2136, pipelines/front_ms 2038, pipelines/driver_ms 97.77, render/thread/ms 62.86, render/busy/ms 61.45, stage/render/prepare_assets 48.05, pipelines/wait_ms 40.38, main/busy/ms 19.88, stage/render/render 10.20, render/graph/ms 8.54, stage/main/update 8.27, stage/render/graph_submit 6.64 |
| warm-up | 3.98 | 36.4 | pipelines/compile_ms 55.45, pipelines/front_ms 53.41, main/busy/ms 42.30, render/thread/ms 34.18, render/busy/ms 32.67, stage/main/update 22.46, stage/main/post_update 17.11, stage/render/render 14.59, render/graph/ms 12.53, stage/render/graph_submit 8.32, encode//sum 6.80, stage/render/prepare_flush 5.54 |
| warm-up | 4.05 | 76.8 | pipelines/compile_ms 648, pipelines/front_ms 626, main/busy/ms 125, stage/main/update 107, render/thread/ms 86.86, render/busy/ms 85.41, stage/render/prepare_resources 34.21, pipelines/driver_ms 22.01, stage/render/prepare_assets 14.67, stage/render/render 14.37, render/graph/ms 12.24, stage/main/post_update 11.94 |
| warm-up | 4.19 | 133.5 | pipelines/compile_ms 1985, pipelines/front_ms 1924, render/thread/ms 123, render/busy/ms 122, main/busy/ms 113, stage/main/update 97.53, pipelines/driver_ms 61.12, stage/render/prepare_resources 56.72, stage/render/prepare_assets 45.30, stage/render/render 8.30, pipelines/wait_ms 8.23, stage/main/post_update 7.83 |
| warm-up | 4.22 | 33.7 | pipelines/compile_ms 1404, pipelines/front_ms 1360, render/thread/ms 89.41, render/busy/ms 87.05, pipelines/driver_ms 44.88, stage/render/prepare_assets 42.48, main/busy/ms 25.44, stage/render/prepare_resources 23.94, stage/render/render 17.60, render/graph/ms 15.10, stage/render/graph_submit 13.13, stage/main/update 11.82 |
| warm-up | 4.27 | 50.2 | pipelines/compile_ms 101, pipelines/front_ms 94.40, main/busy/ms 79.87, render/thread/ms 75.88, stage/main/update 67.07, render/busy/ms 62.55, stage/render/render 33.88, render/graph/ms 20.28, stage/render/graph_submit 16.66, render/present/ms 13.03, encode/submit 12.80, stage/render/prepare_assets 12.15 |
| warm-up | 4.37 | 101.2 | pipelines/compile_ms 749, pipelines/front_ms 724, pipelines/wait_ms 128, main/busy/ms 107, render/thread/ms 105, render/busy/ms 93.15, stage/main/update 65.48, stage/render/render 30.60, stage/render/prepare_assets 29.01, pipelines/driver_ms 24.68, render/graph/ms 17.70, stage/render/graph_submit 13.70 |
| warm-up | 4.41 | 42.1 | pipelines/compile_ms 1487, pipelines/front_ms 1438, pipelines/wait_ms 142, render/thread/ms 97.09, render/busy/ms 95.43, main/busy/ms 70.52, pipelines/driver_ms 48.77, stage/render/prepare_resources 29.22, stage/render/prepare_assets 27.95, stage/render/render 21.58, stage/main/update 20.89, stage/main/post_update 20.50 |
| warm-up | 4.46 | 48.2 | pipelines/compile_ms 1810, pipelines/front_ms 1735, pipelines/wait_ms 242, render/thread/ms 83.53, render/busy/ms 82.05, pipelines/driver_ms 75.05, main/busy/ms 59.15, stage/render/prepare_assets 35.44, stage/main/update 24.17, stage/render/prepare_resources 18.95, stage/main/post_update 18.39, stage/render/render 15.12 |
| warm-up | 4.53 | 71.1 | pipelines/compile_ms 1436, pipelines/front_ms 1367, pipelines/wait_ms 326, render/thread/ms 108, render/busy/ms 107, main/busy/ms 73.00, pipelines/driver_ms 68.85, stage/render/prepare_assets 49.93, stage/main/update 36.05, stage/render/prepare_resources 19.29, stream/drive/ms 18.02, stage/main/post_update 17.85 |
| warm-up | 4.60 | 68.6 | pipelines/compile_ms 929, pipelines/front_ms 887, pipelines/wait_ms 156, render/thread/ms 121, render/busy/ms 120, main/busy/ms 98.93, stage/main/update 66.37, pipelines/driver_ms 42.17, stage/render/prepare_resources 35.97, stage/render/prepare_assets 34.39, stream/drive/ms 18.34, stage/main/post_update 18.11 |
| warm-up | 4.71 | 111.7 | pipelines/compile_ms 2300, pipelines/front_ms 2235, pipelines/wait_ms 373, render/thread/ms 135, main/busy/ms 135, render/busy/ms 133, stage/main/update 77.03, pipelines/driver_ms 65.72, stage/render/prepare_assets 59.04, stage/main/post_update 42.70, stage/render/prepare_bind_groups 27.28, bigspace/total/ms 26.11 |
| warm-up | 4.77 | 53.5 | pipelines/compile_ms 2322, pipelines/front_ms 2264, pipelines/wait_ms 320, render/thread/ms 114, render/busy/ms 112, main/busy/ms 109, stage/render/prepare_assets 68.23, pipelines/driver_ms 57.48, stage/main/update 54.38, stage/main/post_update 41.33, bigspace/total/ms 26.19, stage/render/prepare_bind_groups 25.73 |
| warm-up | 4.81 | 44.7 | pipelines/compile_ms 748, pipelines/front_ms 727, main/busy/ms 71.62, render/thread/ms 71.34, render/busy/ms 70.02, stage/main/update 43.56, stage/render/prepare_assets 35.24, pipelines/driver_ms 21.06, stage/main/post_update 14.81, stream/drive/ms 14.74, stage/render/render 11.94, render/graph/ms 10.17 |
| warm-up | 4.86 | 47.0 | pipelines/compile_ms 321, pipelines/front_ms 309, main/busy/ms 84.18, render/thread/ms 67.29, render/busy/ms 65.87, stage/main/update 59.98, stream/drive/ms 38.59, stage/render/prepare_assets 20.57, stage/main/post_update 15.53, stage/render/render 13.10, pipelines/driver_ms 12.16, render/graph/ms 11.06 |
| warm-up | 4.90 | 38.1 | pipelines/compile_ms 288, pipelines/front_ms 276, main/busy/ms 104, stage/main/update 86.12, stream/drive/ms 61.55, render/thread/ms 55.55, render/busy/ms 53.98, stage/render/prepare_assets 16.13, stage/render/render 14.74, render/graph/ms 12.42, pipelines/driver_ms 11.74, stage/main/post_update 11.49 |
| warm-up | 4.97 | 73.6 | pipelines/compile_ms 1617, pipelines/front_ms 1576, main/busy/ms 78.64, stage/main/update 62.37, render/thread/ms 48.10, render/busy/ms 46.58, pipelines/driver_ms 41.50, stream/drive/ms 38.23, stage/render/prepare_assets 17.60, stage/render/render 13.59, render/graph/ms 11.49, encode//sum 9.37 |
| warm-up | 5.00 | 25.8 | pipelines/compile_ms 1811, pipelines/front_ms 1763, pipelines/driver_ms 47.78, render/thread/ms 45.01, render/busy/ms 43.66, main/busy/ms 32.81, stage/render/prepare_assets 18.12, stage/main/update 15.88, stage/render/render 11.91, render/graph/ms 10.20, stage/main/post_update 8.18, stage/render/graph_submit 7.03 |
| warm-up | 5.03 | 34.7 | pipelines/compile_ms 455, pipelines/front_ms 439, render/thread/ms 54.24, render/busy/ms 52.87, main/busy/ms 45.50, stage/main/update 25.37, pipelines/driver_ms 16.23, stage/render/render 13.00, stage/main/post_update 12.28, stage/render/prepare_resources 11.04, render/graph/ms 11.03, stage/render/prepare_assets 8.96 |
| warm-up | 5.06 | 28.9 | pipelines/compile_ms 225, pipelines/front_ms 219, render/thread/ms 54.21, render/busy/ms 52.95, main/busy/ms 44.10, stage/main/update 25.20, stage/render/prepare_assets 13.90, stage/main/post_update 12.67, stage/render/render 11.79, stage/render/prepare_resources 10.91, render/graph/ms 9.77, encode//sum 7.05 |
| warm-up | 5.16 | 27.4 | pipelines/compile_ms 704, pipelines/front_ms 683, render/thread/ms 36.19, render/busy/ms 34.75, main/busy/ms 33.42, pipelines/driver_ms 21.55, stage/main/update 15.17, stage/render/render 12.61, stage/main/post_update 12.47, render/graph/ms 10.82, stage/render/prepare_assets 9.67, stage/render/graph_submit 7.59 |
| warm-up | 5.19 | 29.2 | pipelines/compile_ms 794, pipelines/front_ms 770, render/thread/ms 48.44, render/busy/ms 47.06, main/busy/ms 38.67, pipelines/driver_ms 23.88, stage/main/update 17.83, stage/main/post_update 13.88, stage/render/render 12.84, render/graph/ms 10.99, stage/render/prepare_assets 10.72, stage/render/prepare_resources 9.48 |
| warm-up | 5.26 | 32.9 | pipelines/compile_ms 667, pipelines/front_ms 640, main/busy/ms 41.42, render/thread/ms 36.70, render/busy/ms 35.14, pipelines/driver_ms 26.95, stage/main/update 21.70, stream/drive/ms 14.29, stage/render/render 13.75, stage/main/post_update 12.42, render/graph/ms 11.60, encode//sum 8.10 |
| warm-up | 5.31 | 27.2 | render/thread/ms 42.21, render/busy/ms 40.77, main/busy/ms 32.04, pipelines/compile_ms 29.92, pipelines/front_ms 27.18, stage/render/render 14.83, stage/main/update 14.50, render/graph/ms 12.89, stage/render/prepare_assets 11.78, stage/main/post_update 10.92, stage/render/graph_submit 8.83, encode//sum 8.54 |
| warm-up | 5.51 | 32.0 | pipelines/compile_ms 363, pipelines/front_ms 352, main/busy/ms 42.47, render/thread/ms 27.92, render/busy/ms 26.55, stage/main/update 23.77, stream/drive/ms 15.79, stage/render/render 12.42, pipelines/driver_ms 11.12, stage/main/post_update 10.80, render/graph/ms 10.55, encode//sum 7.24 |
| warm-up | 5.77 | 25.3 | main/busy/ms 31.10, render/thread/ms 20.65, stage/main/update 19.90, render/busy/ms 19.67, stream/drive/ms 12.25, stage/render/render 8.62, stage/main/post_update 7.74, render/graph/ms 7.28, encode//sum 6.20, stage/render/prepare_assets 5.11, stage/render/graph_submit 4.72, encode/submit 2.80 |
| warm-up | 5.84 | 30.9 | render/thread/ms 38.04, render/busy/ms 36.43, main/busy/ms 31.93, stage/main/update 16.02, stage/render/render 15.76, render/graph/ms 13.62, stage/main/post_update 12.08, stage/render/prepare_assets 11.02, stage/render/graph_submit 9.53, encode//sum 9.49, encode/submit 5.96, stage/render/camera_driver 3.90 |
| warm-up | 5.90 | 32.1 | pipelines/compile_ms 293, pipelines/front_ms 280, render/thread/ms 38.31, render/busy/ms 37.35, main/busy/ms 27.00, stage/render/prepare_assets 20.74, pipelines/driver_ms 13.61, stage/main/update 12.74, stage/render/render 7.70, stage/main/post_update 7.05, render/graph/ms 6.30, encode//sum 6.01 |
| warm-up | 6.07 | 31.4 | main/busy/ms 32.44, render/thread/ms 31.91, render/busy/ms 30.79, stage/main/update 21.00, stage/render/render 11.39, render/graph/ms 9.81, stage/render/prepare_assets 9.30, encode//sum 8.85, stage/render/graph_submit 7.22, stage/main/post_update 7.05, encode/submit 4.57, stream/drive/ms 4.20 |
| warm-up | 6.13 | 25.3 | render/thread/ms 41.15, main/busy/ms 39.94, render/busy/ms 39.71, pipelines/compile_ms 28.44, pipelines/front_ms 26.61, stage/main/update 21.55, stage/render/render 16.41, render/graph/ms 14.21, stage/main/post_update 12.91, encode//sum 10.28, stage/render/graph_submit 10.20, encode/submit 6.88 |
| warm-up | 6.17 | 39.7 | render/thread/ms 50.65, render/busy/ms 49.51, main/busy/ms 40.79, pipelines/compile_ms 28.44, pipelines/front_ms 26.61, stage/main/update 25.20, stage/render/prepare_assets 18.55, stage/render/render 13.19, render/graph/ms 11.65, stage/render/graph_submit 9.09, stage/main/post_update 8.86, encode//sum 7.66 |
| warm-up | 6.21 | 39.7 | render/thread/ms 65.71, render/busy/ms 64.15, main/busy/ms 44.95, stage/main/update 25.05, stage/render/render 22.48, stage/render/prepare_assets 21.78, render/graph/ms 20.04, stage/render/graph_submit 15.88, stage/main/post_update 11.37, encode//sum 9.66, encode/finish_wall 9.32, encode/pass:shadow_point_light/sum 8.78 |
| warm-up | 6.24 | 25.6 | render/thread/ms 55.30, render/busy/ms 53.88, main/busy/ms 37.61, stage/render/render 22.57, render/graph/ms 20.26, stage/main/update 17.75, stage/render/graph_submit 15.95, stage/render/prepare_assets 12.82, stage/main/post_update 11.90, encode/finish_wall 10.31, encode/pass:shadow_point_light/sum 9.85, encode//sum 9.23 |
| warm-up | 6.27 | 31.4 | render/thread/ms 43.46, render/busy/ms 42.44, main/busy/ms 31.33, stage/main/update 15.69, stage/render/render 15.19, stage/render/prepare_assets 14.93, render/graph/ms 13.65, stage/render/graph_submit 10.78, stage/main/post_update 9.38, encode/finish_wall 7.74, encode/pass:shadow_point_light/sum 7.31, encode//sum 6.84 |
| warm-up | 6.43 | 39.4 | main/busy/ms 57.95, render/thread/ms 44.02, render/busy/ms 42.29, stage/main/update 41.84, stage/render/render 23.58, render/graph/ms 21.23, stage/render/graph_submit 17.34, stream/drive/ms 14.36, encode/finish_wall 11.03, stage/main/post_update 10.84, encode/pass:shadow_point_light/sum 8.87, encode//sum 7.82 |
| warm-up | 6.46 | 37.6 | main/busy/ms 52.47, render/thread/ms 52.21, render/busy/ms 50.94, stage/main/update 33.17, stage/render/render 20.45, render/graph/ms 18.58, stage/render/prepare_assets 16.33, stage/render/graph_submit 14.58, stage/main/post_update 11.94, encode/finish_wall 10.11, encode/pass:shadow_point_light/sum 7.95, encode//sum 7.91 |
| warm-up | 6.49 | 28.5 | render/thread/ms 45.37, render/busy/ms 44.32, main/busy/ms 35.79, stage/render/prepare_assets 22.13, stage/main/update 18.71, stage/render/render 13.10, render/graph/ms 11.59, stage/main/post_update 10.12, stage/render/graph_submit 9.04, stream/drive/ms 6.84, encode//sum 6.01, encode/finish_wall 5.96 |
| warm-up | 6.59 | 26.0 | render/thread/ms 37.63, render/busy/ms 36.55, main/busy/ms 33.75, stage/main/update 20.25, stage/render/render 17.06, render/graph/ms 15.50, stage/render/graph_submit 11.74, encode//sum 9.52, stage/main/post_update 8.68, encode/finish_wall 7.72, stage/render/prepare_assets 7.50, pipelines/compile_ms 7.01 |
| warm-up | 6.78 | 74.4 | pipelines/compile_ms 183, pipelines/front_ms 122, render/thread/ms 84.60, render/busy/ms 83.68, stage/render/render 75.78, render/graph/ms 74.51, stage/render/graph_submit 71.71, encode/submit 65.45, pipelines/driver_ms 60.51, main/busy/ms 19.10, stage/main/post_update 7.85, stage/main/update 7.66 |
| measure | 18.30 | 32.4 | main/busy/ms 37.54, stage/main/update 25.90, render/thread/ms 22.51, render/busy/ms 21.52, stage/render/render 10.76, render/graph/ms 9.40, stage/main/post_update 7.97, stage/render/graph_submit 7.00, encode//sum 6.07, encode/submit 4.11, stage/render/prepare_assets 3.88, encode/finish_wall 2.91 |
| measure | 59.07 | 29.8 | render/thread/ms 35.04, render/busy/ms 34.09, main/busy/ms 26.86, stage/render/prepare_assets 16.70, stage/main/post_update 11.84, stage/render/render 9.88, render/graph/ms 8.56, encode//sum 8.47, stage/main/fixed_main_loop 5.74, stage/render/graph_submit 5.38, stage/main/update 5.21, stage/render/camera_driver 3.07 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.cache.enabled=on --set shadow.lamp.batched_pass=on --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\balmora_walk_night@2560x1440_arm=batched-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 14%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1408 pmon samples)
- focus: clean
- nvidia-smi over the window (254 samples): GPU load **82%** (min 49%) · 2625 MHz (min 2625) · memory 8101 MHz · 81.1 W · 75 °C (max 80) · VRAM 6138 MiB · throttled 100% of samples

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
- `shadow.lamp.batched_pass=on (command line: --set)`
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
