# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.batched_pass=on (command line: --set); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6674 frames over 65.6 s · **median 9.64 ms (104 fps)** · mean 9.84 · p95 11.64 · p99 13.20 (1% low 76 fps) · worst 31.53 ms · 32 spikes over 1.5× median · 26.5 stutters/min (work 25.6/min)
- warm-up: 354 frames over 9.5 s, worst 2553.1 ms
- hitches (> 25.0 ms): **3** measured, 44 in warm-up
- timed GPU passes: 6.39 ms of a 9.84 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.64 ms, limit 11 ms
- **FAIL** frame_ms: 29 stutters (> 1.5x moving median of 60) in 1.09 min = 26.51/min, limit 1/min
- pass worst frame 31.53 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.923 | 2.029 | 2.062 | 0.95 | 0.170 |
| main_opaque_pass_3d | 1.193 | 1.259 | 1.561 | 0.95 | 0.064 |
| early prepass | 0.481 | 0.507 | 0.729 | 0.95 | 0.090 |
| bloom | 0.300 | 0.317 | 0.325 | 0.95 | 0.064 |
| atmosphere_luts | 0.295 | 0.312 | 0.376 | 0.95 | 0.003 |
| water_velocity | 0.213 | 0.225 | 0.246 | 0.95 | 0.002 |
| water_sspr | 0.191 | 0.202 | 0.203 | 0.95 | 0.002 |
| batched_point_shadows | 0.189 | 0.342 | 0.490 | 0.55 | 0.127 |
| ssao | 0.177 | 0.187 | 0.191 | 0.95 | 0.007 |
| upscaling | 0.122 | 0.129 | 0.141 | 0.95 | 0.004 |
| atmos_froxel_inject | 0.121 | 0.128 | 0.129 | 0.95 | 0.001 |
| water_fft | 0.116 | 0.122 | 0.124 | 0.95 | 0.003 |
| uniform_allocation | 0.113 | 0.119 | 0.146 | 0.95 | 0.021 |
| clouds_composite | 0.099 | 0.105 | 0.108 | 0.95 | 0.002 |
| bin_unpacking | 0.091 | 0.096 | 0.121 | 0.95 | 0.016 |
| gpu_instance_materialization | 0.090 | 0.095 | 0.099 | 0.95 | 0.004 |
| tonemapping | 0.082 | 0.086 | 0.090 | 0.95 | 0.004 |
| main_transmissive_pass_3d | 0.067 | 0.071 | 0.135 | 0.95 | 0.008 |
| clustering | 0.061 | 0.065 | 0.068 | 0.95 | 0.018 |
| early_mesh_preprocessing | 0.054 | 0.057 | 0.076 | 0.95 | 0.003 |
| clouds_trace | 0.053 | 0.056 | 0.079 | 0.95 | 0.001 |
| render_sky | 0.050 | 0.053 | 0.053 | 0.95 | 0.002 |
| clouds_resolve | 0.047 | 0.050 | 0.051 | 0.95 | 0.001 |
| lightprobe_irradiance_map | 0.036 | 0.413 | 0.416 | 0.09 | 0.001 |
| surface_trail | 0.035 | 0.057 | 0.057 | 0.62 | 0.002 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.032 | 0.95 | 0.001 |
| batched_point_shadows/uniform_allocation | 0.027 | 0.048 | 0.082 | 0.55 | 0.010 |
| batched_point_shadows/early_mesh_preprocessing | 0.026 | 0.046 | 0.065 | 0.55 | 0.006 |
| water_snapshot | 0.023 | 0.024 | 0.028 | 0.95 | 0.002 |
| range_unpacking | 0.023 | 0.024 | 0.031 | 0.95 | 0.003 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.95 | 0.002 |
| lightprobe_radiance_map | 0.019 | 0.220 | 0.231 | 0.09 | 0.004 |
| night_sky | 0.016 | 0.017 | 0.044 | 0.95 | 0.002 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.026 | 0.95 | 0.005 |
| batched_point_shadows/bin_unpacking | 0.015 | 0.027 | 0.047 | 0.55 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.030 | 0.95 | 0.185 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.95 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 0.95 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.95 | 0.003 |
| batched_point_shadows/range_unpacking | 0.004 | 0.007 | 0.007 | 0.55 | 0.002 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.95 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.95 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.560 | 10.220 | 18.37 | 57127.5 |
| render/busy/ms | 8.063 | 9.758 | 17.79 | 53814.7 |
| main/busy/ms | 7.897 | 9.189 | 29.55 | 52702.8 |
| stage/render/render | 5.148 | 6.318 | 12.34 | 34357.7 |
| render/graph/ms | 4.501 | 5.656 | 11.72 | 30036.7 |
| stage/main/post_update | 4.452 | 5.304 | 9.37 | 29714.8 |
| stage/render/graph_submit | 3.280 | 4.395 | 10.37 | 21887.8 |
| encode//sum | 3.175 | 3.834 | 7.98 | 21191.4 |
| encode/submit | 2.026 | 2.691 | 5.64 | 13523.1 |
| stage/main/update | 1.804 | 2.298 | 23.68 | 12042.2 |
| encode/finish_wall | 1.312 | 2.156 | 7.78 | 8755.2 |
| stage/render/camera_driver | 1.168 | 1.332 | 2.73 | 7791.9 |
| encode/pass:early prepass/sum | 0.788 | 1.017 | 2.10 | 5257.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.785 | 1.025 | 2.22 | 5238.5 |
| encode/pass:shadow_directional_light/sum | 0.779 | 1.179 | 1.94 | 5198.2 |
| encode/pass:shadow_point_light/sum | 0.647 | 1.973 | 7.75 | 4318.4 |
| stage/render/prepare_write_phase_buffers | 0.627 | 0.967 | 1.77 | 4185.2 |
| bigspace/total/ms | 0.547 | 1.287 | 2.70 | 3648.8 |
| stage/render/prepare_assets | 0.511 | 0.639 | 9.53 | 3411.4 |
| stage/main/fixed_main_loop | 0.484 | 0.909 | 2.49 | 3229.2 |
| render/present/ms | 0.411 | 0.540 | 2.03 | 2740.5 |
| stage/render/prepare_resources | 0.410 | 0.491 | 3.87 | 2738.1 |
| stage/render/prepare_meshes | 0.403 | 0.504 | 3.79 | 2691.8 |
| stage/main/pre_update | 0.375 | 0.505 | 4.36 | 2501.5 |
| stage/main/first | 0.355 | 0.458 | 5.94 | 2369.2 |
| stage/render/prepare_bind_groups | 0.322 | 0.547 | 1.64 | 2146.5 |
| controller/feed/ms | 0.311 | 0.389 | 0.66 | 2078.2 |
| stage/render/prepare_flush | 0.289 | 0.505 | 1.97 | 1926.4 |
| stage/render/prepare_batch_phases | 0.270 | 0.407 | 2.42 | 1803.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.258 | 0.414 | 0.80 | 1722.5 |
| stage/main/last | 0.248 | 0.294 | 1.04 | 1653.4 |
| bigspace/low_precision/ms | 0.231 | 0.757 | 2.49 | 1542.3 |
| stream/drive/ms | 0.200 | 0.277 | 10.83 | 1336.0 |
| stage/render/create_views | 0.194 | 0.254 | 0.93 | 1293.4 |
| bigspace/high_precision/ms | 0.168 | 0.712 | 1.92 | 1121.9 |
| stage/render/extract_commands | 0.165 | 0.195 | 0.40 | 1102.0 |
| anim/evaluate/ms | 0.154 | 0.268 | 1.49 | 1027.7 |
| anim/evaluate_cpu/ms | 0.148 | 0.184 | 0.40 | 988.1 |
| stream/tick/ms | 0.147 | 0.212 | 0.57 | 981.4 |
| stage/render/queue | 0.138 | 0.214 | 1.49 | 920.5 |
| stage/render/specialize | 0.131 | 0.215 | 1.49 | 871.1 |
| stage/render/prepare_views | 0.130 | 0.166 | 0.47 | 864.9 |
| pipelines/compile_ms | 0.109 | 0.000 | 132.28 | 725.3 |
| pipelines/front_ms | 0.106 | 0.000 | 128.89 | 706.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.090 | 0.143 | 0.33 | 598.8 |
| render/acquire/ms | 0.086 | 0.110 | 0.34 | 572.4 |
| controller/move/ms | 0.075 | 0.223 | 0.99 | 497.5 |
| controller/write/ms | 0.074 | 0.102 | 0.32 | 491.4 |
| terrain/update/ms | 0.057 | 0.281 | 1.55 | 383.7 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.072 | 0.22 | 308.2 |
| physics/step/ms | 0.036 | 0.079 | 0.78 | 242.1 |
| stage/render/phase_sort | 0.034 | 0.058 | 0.17 | 224.0 |
| encode/point_shadow_cache_copies/sum | 0.033 | 0.078 | 0.29 | 220.1 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.24 | 206.1 |
| physics/insert/ms | 0.024 | 0.270 | 0.81 | 158.8 |
| stage/render/cleanup | 0.022 | 0.031 | 0.66 | 147.5 |
| terrain/select/ms | 0.021 | 0.248 | 0.51 | 138.9 |
| weather/map/ms | 0.016 | 0.023 | 0.08 | 109.7 |
| far/select/ms | 0.015 | 0.166 | 0.45 | 96.9 |
| encode/camera_driver_span_begin/sum | 0.012 | 0.021 | 0.09 | 81.8 |
| anim/apply/ms | 0.011 | 0.019 | 0.10 | 73.4 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.31 | 70.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.26 | 68.2 |
| script/update/ms | 0.008 | 0.011 | 0.12 | 50.2 |
| controller/avoid/ms | 0.006 | 0.012 | 0.03 | 38.3 |
| stream/despawn/ms | 0.005 | 0.000 | 1.15 | 36.4 |
| script/input/ms | 0.005 | 0.006 | 0.05 | 31.7 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 29.1 |
| stream/spawn/ms | 0.003 | 0.000 | 1.10 | 21.7 |
| pipelines/driver_ms | 0.003 | 0.000 | 3.39 | 19.2 |
| stream/upload/ms | 0.003 | 0.000 | 1.07 | 17.9 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 13.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.05 | 7.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 3.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.6 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 32 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 535 |
| crowd/c0/meshes/count | count | 353 | 646 | 144 |
| crowd/c1/count | count | 145 | 154 | 149 |
| crowd/c1/joints/count | count | 5531 | 5894 | 5661 |
| crowd/c1/meshes/count | count | 1167 | 1254 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 161 | 216 | 205 |
| crowd/joints_extracted/count | count | 646 | 5417 | 315 |
| crowd/joints_uploaded/count | count | 3598 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.38 | 1.40 | 0.39 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.09 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.31 | 0.01 |
| encode/finish_critical | ms | 0.97 | 7.75 | 0.82 |
| encode/pass:early prepass/max | ms | 0.79 | 2.10 | 0.78 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.80 | 2.22 | 0.82 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.33 | 0.12 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 0.80 | 0.30 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 1.94 | 0.80 |
| encode/pass:shadow_point_light/max | ms | 0.98 | 7.75 | 0.19 |
| encode/point_shadow_cache_copies/max | ms | 0.06 | 0.29 | 0.05 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.24 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.11 | 29.55 | 7.82 |
| gpu/timed/ms | ms | 7.56 | 8.53 | 7.82 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 10 | 26 | 0 |
| lamp_cache/copies | count | 7 | 15 | 0 |
| lamp_cache/deferred_faces | count | 0 | 15 | 0 |
| lamp_cache/lamps | count | 7 | 8 | 0 |
| lamp_cache/mover_faces | count | 7 | 12 | 0 |
| lamp_cache/movers | count | 133 | 250 | 0 |
| lamp_cache/queued | count | 0 | 40 | 0 |
| lamp_cache/ready | count | 7 | 8 | 0 |
| lamp_cache/render/copies | count | 10 | 15 | 0 |
| lamp_cache/render/drawn | count | 10 | 14 | 0 |
| lamp_cache/render/dropped_copies | count | 0 | 0 | 0 |
| lamp_cache/render/lamps | count | 7 | 8 | 0 |
| lamp_cache/render/loaded | count | 10 | 12 | 0 |
| lamp_cache/render/lost | count | 0 | 6 | 0 |
| lamp_cache/render/views | count | 13 | 32 | 0 |
| lamp_cache/stock_lamps | count | 1 | 2 | 0 |
| lamp_cache/store_starved | count | 0 | 8 | 0 |
| lamp_cache/store_used | count | 10 | 24 | 0 |
| lamp_cache/views | count | 10 | 32 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 157 | 386 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 335 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 157 | 386 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 335 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 766 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2477 | 3521 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 766 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2477 | 3521 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 27 | 85 | 8 |
| phases/shadow/bins | count | 856 | 1260 | 671 |
| phases/shadow/instance_batches | count | 4 | 10 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 106.5 k | 49.2 k |
| phases/shadow/items | count | 2336 | 3024 | 2027 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 48 | 114 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3182 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 327 | 328 | 328 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 28 | 291 | 11 |
| preprocess/bind_groups_created | count | 23 | 285 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 8 | 5 |
| preprocess/bind_groups_reused | count | 138 | 255 | 123 |
| process/cpu_usage | % | 25 | 38.39 | 21.63 |
| process/mem_usage | GiB | 5.23 | 5.42 | 5.42 |
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
| skin/blocks_changed/count | count | 13 | 89 | 6 |
| skin/blocks_skipped/count | count | 147 | 205 | 198 |
| skin/blocks_walked/count | count | 13 | 28 | 7 |
| skin/joint_bytes_written/bytes | bytes | 46.6 k | 809.6 k | 28.2 k |
| skin/joints_written/count | count | 728 | 12.7 k | 440 |
| skin/upload_writes/count | count | 7 | 25 | 7 |
| started_load_count |  | 6438 | 6563 | 6563 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.05 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.70 | 38.96 | 31.59 |
| system/mem_usage | % | 56.30 | 56.81 | 56.77 |
| terrain/patches | count | 511 | 520 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 85 | 144 | 66 |
| views/all/bins | count | 2913 | 3288 | 2961 |
| views/all/count | count | 18 | 41 | 3 |
| views/all/instance_batches | count | 12 | 18 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 204.8 k | 147.5 k |
| views/all/items | count | 8030 | 10.0 k | 7969 |
| views/all/pipelines | count | 96 | 161 | 66 |
| views/all/pipelines_distinct | count | 68 | 74 | 61 |
| views/all/work_items | count | 154.1 k | 214.5 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 34 | 8 |
| views/cascade/bins | count | 719 | 858 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2118 | 2439 | 2027 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 12 | 55 | 0 |
| views/lamp_face/bins | count | 141 | 476 | 0 |
| views/lamp_face/count | count | 15 | 38 | 0 |
| views/lamp_face/culled | count | 0 | 6 | 0 |
| views/lamp_face/drawn | count | 8 | 22 | 0 |
| views/lamp_face/instance_batches | count | 0 | 2 | 0 |
| views/lamp_face/instance_draws | count | 0 | 24 | 0 |
| views/lamp_face/instance_slots | count | 0 | 8192 | 0 |
| views/lamp_face/items | count | 186 | 603 | 0 |
| views/lamp_face/pipelines | count | 23 | 87 | 0 |
| views/lamp_face/revealed | count | 0 | 4 | 0 |
| views/lamp_face/work_items | count | 81 | 8526 | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2018 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5910 | 7462 | 5942 |
| views/main/pipelines | count | 47 | 49 | 46 |
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
| world/meshes | count | 21.1 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 40.87 | 224 | 272.8 k |
| anim/events/count | count | 0.08 | 3 | 535 |
| assets/Image/added | count | 0.09 | 34 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.43 | 295 | 2874 |
| assets/Mesh/removed | count | 0.73 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.12 | 10 | 7475 |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.22 | 1245 | 1490 |
| encode//count | count | 27.90 | 30 | 186.2 k |
| encode/bind_walk/init_textures | count | 10.1 k | 31.7 k | 67.39 M |
| encode/bind_walk/init_textures_skipped | count | 53.9 k | 78.2 k | 359.60 M |
| encode/bind_walk/inits | count | 599 | 770 | 4.00 M |
| encode/bind_walk/inits_skipped | count | 139 | 235 | 926.6 k |
| encode/bind_walk/merge_textures | count | 10.0 k | 31.6 k | 66.97 M |
| encode/bind_walk/merge_textures_skipped | count | 8799 | 16.8 k | 58.73 M |
| encode/bind_walk/merges | count | 122 | 195 | 814.2 k |
| encode/bind_walk/merges_skipped | count | 127 | 227 | 848.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6674 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6674 |
| encode/pass:early prepass/count | count | 1 | 1 | 6674 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6674 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6674 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6674 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6674 |
| encode/pass:shadow_point_light/count | count | 0.57 | 1 | 3793 |
| encode/point_shadow_cache_copies/count | count | 0.55 | 1 | 3691 |
| encode/queue/command_buffers | count | 39.03 | 43 | 260.5 k |
| encode/queue/submits | count | 2.01 | 4 | 13.4 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.61 M | 15.49 G |
| encode/queue/write_buffer/calls | count | 288 | 468 | 1.92 M |
| encode/queue/write_buffer_with/bytes | bytes | 121.5 k | 1.24 M | 811.03 M |
| encode/queue/write_buffer_with/calls | count | 10.28 | 894 | 68.6 k |
| encode/queue/write_texture/bytes | bytes | 1908 | 589.8 k | 12.74 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6674 |
| physics/inserts | count | 0.34 | 88 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 175 | 2335 |
| physics/steps | count | 0.59 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 8 |
| pipelines/created | count | 0.00 | 1 | 9 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 35 | 223 |
| stream/despawn/items | count | 0.03 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5072 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1112 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 47, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2553.1 | main/busy/ms 2522, stage/main/post_update 2482, stage/main/update 30.57, stage/main/pre_update 6.20, stage/main/last 1.23, surface/advance_cpu_ms 0.75, stage/main/first 0.50, stage/main/fixed_main_loop 0.37, stream/drive/ms 0.32, surface/cell_upload_ms 0.29, stream/tick/ms 0.11, script/update/ms 0.10 |
| warm-up | 3.55 | 1001.5 | main/busy/ms 2702, stage/main/post_update 2506, render/thread/ms 961, render/busy/ms 814, stage/render/prepare_assets 765, stage/main/update 180, stage/render/prepare_views 145, render/acquire/ms 145, stage/render/create_views 29.79, stage/render/render 12.44, render/graph/ms 9.11, stage/main/pre_update 8.05 |
| warm-up | 3.64 | 80.5 | render/thread/ms 1024, render/busy/ms 876, stage/render/prepare_assets 815, main/busy/ms 209, stage/main/update 158, stage/render/prepare_views 146, render/acquire/ms 145, pipelines/compile_ms 41.02, pipelines/front_ms 37.87, stage/main/post_update 37.27, stage/render/create_views 30.14, terrain/update/ms 18.81 |
| warm-up | 3.71 | 71.2 | pipelines/wait_ms 1005, pipelines/compile_ms 616, pipelines/front_ms 558, render/thread/ms 133, render/busy/ms 127, main/busy/ms 74.09, stage/render/render 66.83, render/graph/ms 61.00, pipelines/driver_ms 57.87, stage/render/prepare_assets 51.19, stage/main/post_update 49.92, stage/render/graph_submit 28.51 |
| warm-up | 3.73 | 28.4 | pipelines/wait_ms 1482, pipelines/compile_ms 725, pipelines/front_ms 649, render/thread/ms 91.91, render/busy/ms 86.04, pipelines/driver_ms 75.86, stage/render/render 73.17, main/busy/ms 70.26, render/graph/ms 67.09, stage/main/post_update 52.42, stage/render/graph_submit 31.00, encode/finish_wall 25.99 |
| warm-up | 3.86 | 128.5 | pipelines/wait_ms 3721, pipelines/compile_ms 1331, pipelines/front_ms 1257, render/thread/ms 148, render/busy/ms 145, pipelines/driver_ms 73.70, main/busy/ms 41.41, stage/render/prepare_resources 39.13, stage/render/prepare_flush 30.85, stage/render/prepare_assets 26.66, stage/main/post_update 25.77, stage/render/prepare_write_phase_buffers 21.00 |
| warm-up | 3.91 | 43.1 | pipelines/wait_ms 3394, pipelines/compile_ms 1308, pipelines/front_ms 1250, render/thread/ms 166, render/busy/ms 164, main/busy/ms 87.04, stage/main/update 61.43, pipelines/driver_ms 58.86, stage/render/prepare_resources 50.74, stage/render/prepare_flush 30.75, stage/render/render 24.79, stage/render/prepare_assets 23.15 |
| warm-up | 4.05 | 143.9 | pipelines/compile_ms 2322, pipelines/front_ms 2059, pipelines/wait_ms 364, pipelines/driver_ms 263, render/thread/ms 151, render/busy/ms 149, main/busy/ms 91.91, stage/render/render 68.14, stage/main/update 66.03, render/graph/ms 65.99, stage/render/prepare_assets 60.00, stage/render/camera_driver 58.03 |
| warm-up | 4.08 | 29.7 | pipelines/compile_ms 2640, pipelines/front_ms 2361, pipelines/wait_ms 425, pipelines/driver_ms 279, render/thread/ms 138, render/busy/ms 136, stage/render/prepare_assets 74.38, stage/render/render 57.02, render/graph/ms 55.12, stage/render/camera_driver 47.02, main/busy/ms 38.29, stage/main/post_update 17.71 |
| warm-up | 4.15 | 26.3 | pipelines/compile_ms 202, pipelines/front_ms 186, main/busy/ms 64.06, render/thread/ms 43.60, stage/main/update 43.58, render/busy/ms 36.76, stage/render/render 19.87, pipelines/driver_ms 15.82, stage/main/post_update 14.61, render/graph/ms 12.78, stage/render/prepare_assets 9.88, stage/render/graph_submit 9.48 |
| warm-up | 4.24 | 91.4 | pipelines/compile_ms 812, pipelines/front_ms 751, render/thread/ms 76.16, main/busy/ms 74.65, render/busy/ms 69.50, pipelines/driver_ms 61.14, stage/main/update 59.99, stage/render/prepare_assets 44.32, stage/render/render 21.37, render/graph/ms 14.32, stage/render/graph_submit 10.53, stage/main/post_update 7.96 |
| warm-up | 4.28 | 44.6 | pipelines/compile_ms 1059, pipelines/front_ms 994, render/thread/ms 80.79, render/busy/ms 74.67, pipelines/driver_ms 64.65, stage/render/prepare_assets 56.63, main/busy/ms 42.22, stage/main/update 29.23, stage/render/render 16.81, pipelines/wait_ms 12.89, render/graph/ms 10.44, pipelines/lock_ms 9.02 |
| warm-up | 4.32 | 34.4 | pipelines/compile_ms 757, pipelines/front_ms 736, render/thread/ms 50.11, render/busy/ms 48.76, stage/render/prepare_assets 35.54, pipelines/driver_ms 21.42, main/busy/ms 19.83, pipelines/wait_ms 9.63, stage/render/render 9.31, stage/main/update 9.03, render/graph/ms 7.82, pipelines/lock_ms 5.99 |
| warm-up | 4.37 | 46.6 | pipelines/compile_ms 236, pipelines/front_ms 229, render/thread/ms 48.88, stage/render/prepare_views 30.08, render/acquire/ms 29.91, render/busy/ms 17.60, main/busy/ms 16.97, encode//sum 15.35, stage/render/render 13.71, render/graph/ms 11.81, stage/main/update 9.51, stage/render/graph_submit 9.47 |
| warm-up | 4.42 | 30.5 | pipelines/compile_ms 641, pipelines/front_ms 615, main/busy/ms 38.36, render/thread/ms 32.08, render/busy/ms 30.73, pipelines/driver_ms 26.16, stage/main/update 25.14, pipelines/wait_ms 17.09, stream/drive/ms 15.61, stage/render/prepare_assets 12.28, stage/render/render 10.91, render/graph/ms 9.07 |
| warm-up | 4.46 | 44.6 | pipelines/compile_ms 726, pipelines/front_ms 700, main/busy/ms 59.10, render/thread/ms 53.95, render/busy/ms 52.18, pipelines/driver_ms 26.71, stage/render/prepare_assets 26.64, stage/main/update 24.91, stage/render/render 16.69, stage/main/last 15.04, render/graph/ms 14.40, stage/main/post_update 13.50 |
| warm-up | 4.52 | 55.5 | pipelines/compile_ms 493, pipelines/front_ms 475, main/busy/ms 191, stage/main/update 155, render/thread/ms 77.12, render/busy/ms 75.07, stage/render/prepare_assets 34.30, stage/render/render 19.17, pipelines/driver_ms 18.08, render/graph/ms 16.46, stage/main/last 15.01, stage/main/post_update 13.83 |
| warm-up | 4.76 | 242.1 | pipelines/compile_ms 2980, pipelines/front_ms 2902, main/busy/ms 162, stage/main/update 146, render/thread/ms 123, render/busy/ms 121, stage/render/prepare_assets 90.17, pipelines/wait_ms 86.86, pipelines/driver_ms 77.96, pipelines/lock_ms 14.44, stage/render/render 11.98, render/graph/ms 9.87 |
| warm-up | 4.83 | 43.2 | pipelines/compile_ms 64.98, render/thread/ms 60.73, render/busy/ms 57.89, pipelines/front_ms 54.78, main/busy/ms 41.34, stage/render/render 20.82, stage/main/update 19.33, render/graph/ms 17.42, stage/render/graph_submit 14.43, stage/main/post_update 13.19, stage/render/prepare_write_phase_buffers 13.18, encode/submit 10.47 |
| warm-up | 4.90 | 73.6 | pipelines/compile_ms 129, pipelines/front_ms 125, render/thread/ms 110, render/busy/ms 107, main/busy/ms 72.05, pipelines/wait_ms 47.19, stage/main/post_update 37.16, stage/render/prepare_assets 34.13, stage/main/update 23.28, stage/render/render 17.76, stage/render/prepare_write_phase_buffers 17.67, stage/render/prepare_resources 16.23 |
| warm-up | 4.95 | 49.1 | pipelines/compile_ms 129, pipelines/front_ms 125, render/thread/ms 107, render/busy/ms 105, main/busy/ms 73.94, stage/render/prepare_assets 51.44, pipelines/wait_ms 47.19, stage/main/post_update 33.37, stage/main/update 27.76, stage/render/render 21.59, render/graph/ms 19.30, stage/render/graph_submit 16.14 |
| warm-up | 5.07 | 118.8 | pipelines/compile_ms 2427, pipelines/front_ms 2332, render/thread/ms 123, render/busy/ms 121, pipelines/driver_ms 94.82, stage/render/prepare_assets 61.71, main/busy/ms 47.29, stage/render/create_views 36.30, pipelines/wait_ms 26.69, stage/main/update 21.50, stage/render/render 15.74, stage/main/post_update 14.33 |
| warm-up | 5.12 | 49.9 | pipelines/compile_ms 2542, pipelines/front_ms 2441, render/thread/ms 127, render/busy/ms 126, pipelines/driver_ms 101, main/busy/ms 77.58, stage/render/prepare_assets 63.27, stage/main/update 52.13, stage/render/create_views 36.80, pipelines/wait_ms 28.66, stage/render/render 14.13, stage/main/post_update 13.06 |
| warm-up | 5.24 | 126.4 | pipelines/compile_ms 1468, pipelines/front_ms 1340, render/thread/ms 133, render/busy/ms 132, pipelines/driver_ms 128, main/busy/ms 76.18, stage/render/prepare_assets 60.90, stage/main/update 53.24, stage/render/create_views 45.00, pipelines/wait_ms 23.15, stage/render/render 14.44, render/graph/ms 12.30 |
| warm-up | 5.28 | 33.1 | pipelines/compile_ms 1725, pipelines/front_ms 1589, pipelines/driver_ms 136, render/thread/ms 122, render/busy/ms 121, main/busy/ms 91.42, stage/main/update 64.83, stream/drive/ms 50.11, stage/render/create_views 43.84, stage/render/prepare_assets 40.44, pipelines/wait_ms 29.38, stage/render/render 25.31 |
| warm-up | 5.34 | 62.3 | pipelines/compile_ms 469, pipelines/front_ms 450, main/busy/ms 98.86, stage/main/update 79.78, stream/drive/ms 53.89, render/thread/ms 51.49, render/busy/ms 49.45, stage/render/render 28.70, render/graph/ms 26.13, stage/render/graph_submit 19.46, pipelines/driver_ms 19.02, stage/main/post_update 10.74 |
| warm-up | 5.39 | 45.9 | pipelines/compile_ms 139, pipelines/front_ms 132, main/busy/ms 65.61, stage/main/update 48.48, render/thread/ms 48.28, render/busy/ms 46.36, stage/render/render 18.05, render/graph/ms 15.27, stage/render/prepare_assets 14.64, stage/render/graph_submit 10.87, stream/drive/ms 10.05, stage/main/post_update 9.87 |
| warm-up | 5.43 | 43.1 | pipelines/compile_ms 610, pipelines/front_ms 590, render/thread/ms 54.96, render/busy/ms 53.49, main/busy/ms 50.79, stage/main/update 32.67, stage/render/prepare_assets 21.60, pipelines/driver_ms 20.14, stage/render/render 15.14, render/graph/ms 12.82, stage/main/post_update 10.28, stage/render/graph_submit 9.00 |
| warm-up | 5.46 | 26.3 | pipelines/compile_ms 945, pipelines/front_ms 916, render/thread/ms 49.37, render/busy/ms 48.14, main/busy/ms 38.12, pipelines/driver_ms 28.64, stage/main/update 20.73, stage/render/prepare_assets 16.52, stage/render/render 12.01, render/graph/ms 9.95, stage/main/post_update 9.57, encode//sum 7.26 |
| warm-up | 5.49 | 29.7 | pipelines/compile_ms 376, pipelines/front_ms 365, render/thread/ms 50.62, render/busy/ms 49.30, main/busy/ms 40.18, stage/main/update 18.90, stage/render/render 14.70, stage/main/post_update 14.17, render/graph/ms 12.64, pipelines/driver_ms 10.77, stage/render/graph_submit 9.11, stage/render/prepare_write_phase_buffers 7.69 |
| warm-up | 5.51 | 25.1 | render/thread/ms 48.81, render/busy/ms 47.44, main/busy/ms 37.67, stage/main/update 18.14, stage/render/render 14.10, stage/main/post_update 13.69, render/graph/ms 12.26, stage/render/prepare_assets 8.79, stage/render/graph_submit 8.73, stage/render/prepare_resources 8.10, encode//sum 6.72, encode/submit 6.03 |
| warm-up | 5.54 | 25.5 | pipelines/compile_ms 183, pipelines/front_ms 180, render/thread/ms 38.10, render/busy/ms 36.92, main/busy/ms 32.39, stage/main/update 16.47, stage/render/prepare_assets 13.22, stage/render/render 10.99, stage/main/post_update 9.40, render/graph/ms 9.32, encode//sum 6.96, stage/render/graph_submit 6.38 |
| warm-up | 5.63 | 30.1 | pipelines/compile_ms 455, pipelines/front_ms 441, render/thread/ms 43.14, render/busy/ms 41.71, main/busy/ms 36.67, stage/main/post_update 17.03, stage/main/update 14.79, pipelines/driver_ms 14.23, stage/render/render 13.35, render/graph/ms 11.34, stage/render/graph_submit 7.76, encode//sum 7.05 |
| warm-up | 6.14 | 29.7 | main/busy/ms 32.94, stage/main/update 20.81, render/thread/ms 20.36, render/busy/ms 19.30, stream/drive/ms 12.52, stage/render/render 8.68, render/graph/ms 7.24, stage/main/post_update 7.03, encode//sum 6.31, stage/render/graph_submit 4.94, stage/render/prepare_assets 4.35, encode/submit 2.89 |
| warm-up | 6.26 | 25.4 | pipelines/compile_ms 138, pipelines/front_ms 133, render/thread/ms 41.81, render/busy/ms 40.29, main/busy/ms 31.77, stage/render/prepare_assets 16.40, stage/render/render 14.93, stage/main/update 14.88, render/graph/ms 12.68, stage/main/post_update 10.76, stage/render/graph_submit 8.81, encode//sum 8.31 |
| warm-up | 6.29 | 31.5 | pipelines/compile_ms 147, pipelines/front_ms 141, render/thread/ms 41.22, render/busy/ms 40.20, main/busy/ms 26.72, stage/render/prepare_assets 24.30, stage/main/update 13.41, stage/render/render 8.65, stage/main/post_update 7.11, render/graph/ms 7.10, encode//sum 6.76, pipelines/driver_ms 5.71 |
| warm-up | 6.39 | 25.1 | main/busy/ms 31.73, render/thread/ms 30.92, render/busy/ms 29.83, stage/main/update 19.60, stage/render/render 9.55, stage/render/prepare_assets 8.84, render/graph/ms 8.09, stage/main/post_update 7.74, encode//sum 6.26, stage/render/graph_submit 5.37, stream/drive/ms 4.83, stage/render/prepare_meshes 3.66 |
| warm-up | 6.50 | 35.7 | main/busy/ms 42.54, render/thread/ms 42.12, render/busy/ms 40.95, stage/main/update 22.94, stage/render/render 15.78, render/graph/ms 13.97, stage/main/post_update 13.74, encode//sum 10.83, stage/render/prepare_assets 10.11, stage/render/graph_submit 9.29, encode/submit 5.25, stage/render/prepare_meshes 4.80 |
| warm-up | 6.56 | 33.8 | render/thread/ms 45.67, render/busy/ms 44.57, main/busy/ms 36.86, stage/main/update 18.18, stage/render/prepare_assets 13.56, stage/render/render 12.12, render/graph/ms 10.67, stage/main/post_update 9.84, stage/render/graph_submit 7.83, encode//sum 6.61, stage/render/prepare_meshes 5.86, encode/finish_wall 4.40 |
| warm-up | 6.60 | 37.8 | pipelines/compile_ms 143, pipelines/front_ms 137, render/thread/ms 60.45, render/busy/ms 58.77, main/busy/ms 42.73, stage/main/update 20.36, stage/render/render 19.82, render/graph/ms 17.37, stage/render/prepare_assets 16.33, stage/main/post_update 13.25, stage/render/graph_submit 12.77, encode//sum 8.29 |
| warm-up | 6.66 | 34.4 | render/thread/ms 46.01, render/busy/ms 44.88, main/busy/ms 31.13, stage/main/update 15.82, stage/render/prepare_assets 15.29, stage/render/render 14.09, render/graph/ms 12.35, stage/render/graph_submit 9.73, stage/main/post_update 9.43, pipelines/compile_ms 8.48, pipelines/front_ms 7.81, encode//sum 7.80 |
| warm-up | 6.82 | 44.2 | pipelines/compile_ms 141, pipelines/front_ms 135, main/busy/ms 56.60, render/thread/ms 45.34, render/busy/ms 43.81, stage/main/update 40.01, stage/render/render 20.49, render/graph/ms 18.05, stage/render/graph_submit 14.55, stream/drive/ms 14.27, stage/render/prepare_assets 10.87, stage/main/post_update 10.28 |
| warm-up | 6.85 | 35.4 | pipelines/compile_ms 170, pipelines/front_ms 162, render/thread/ms 53.30, render/busy/ms 52.18, main/busy/ms 46.39, stage/main/update 27.40, stage/render/prepare_assets 20.55, stage/render/render 18.57, render/graph/ms 16.43, stage/render/graph_submit 12.77, stage/main/post_update 10.85, encode//sum 8.71 |
| warm-up | 6.88 | 30.3 | render/thread/ms 46.75, render/busy/ms 45.76, main/busy/ms 34.46, pipelines/compile_ms 29.59, pipelines/front_ms 27.86, stage/render/prepare_assets 20.07, stage/main/update 18.25, stage/render/render 14.18, render/graph/ms 12.53, stage/render/graph_submit 9.93, stage/main/post_update 9.78, stream/drive/ms 7.49 |
| measure | 18.35 | 31.5 | main/busy/ms 37.06, stage/main/update 25.60, render/thread/ms 19.89, render/busy/ms 18.88, stage/render/render 9.80, render/graph/ms 8.50, stage/main/post_update 7.86, stage/render/graph_submit 6.13, encode//sum 5.51, encode/submit 4.12, stage/render/prepare_assets 3.70, stage/render/camera_driver 2.25 |
| measure | 59.03 | 25.6 | render/thread/ms 35.30, render/busy/ms 33.72, main/busy/ms 32.31, stage/render/render 17.94, stage/main/post_update 16.66, render/graph/ms 15.36, encode//sum 13.90, stage/render/graph_submit 9.99, stage/main/update 8.18, stage/render/prepare_assets 7.71, encode/submit 5.98, stage/render/camera_driver 5.14 |
| measure | 59.06 | 26.7 | render/thread/ms 35.28, render/busy/ms 34.21, main/busy/ms 25.47, stage/render/prepare_assets 15.92, stage/main/post_update 11.96, stage/render/render 10.52, encode//sum 9.85, render/graph/ms 8.87, stage/main/update 5.79, stage/render/graph_submit 5.09, stage/main/fixed_main_loop 3.84, stage/render/camera_driver 3.58 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.cache.enabled=on --set shadow.lamp.batched_pass=on --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\balmora_walk_night@2560x1440_arm=batched-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1386 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **82%** (min 56%) · 2624 MHz (min 2610) · memory 8101 MHz · 81.2 W · 75 °C (max 80) · VRAM 6132 MiB · throttled 100% of samples

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
