# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.max_lamps=16 (command line: --set); shadow.lamp.atlas.soft_pcf_radius=0 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6648 frames over 65.6 s · **median 9.78 ms (102 fps)** · mean 9.87 · p95 11.51 · p99 13.30 (1% low 75 fps) · worst 32.16 ms · 33 spikes over 1.5× median · 31.1 stutters/min (work 21.0/min)
- warm-up: 355 frames over 8.9 s, worst 2385.1 ms
- hitches (> 25.0 ms): **2** measured, 41 in warm-up
- timed GPU passes: 6.39 ms of a 9.87 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.78 ms, limit 11 ms
- **FAIL** frame_ms: 34 stutters (> 1.5x moving median of 60) in 1.09 min = 31.08/min, limit 1/min
- pass worst frame 32.16 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.835 | 2.034 | 2.066 | 0.90 | 0.175 |
| main_opaque_pass_3d | 1.479 | 1.639 | 2.362 | 0.90 | 0.061 |
| early prepass | 0.464 | 0.515 | 0.754 | 0.90 | 0.088 |
| bloom | 0.286 | 0.317 | 0.326 | 0.90 | 0.063 |
| atmosphere_luts | 0.283 | 0.313 | 0.379 | 0.90 | 0.003 |
| water_velocity | 0.203 | 0.225 | 0.247 | 0.90 | 0.002 |
| water_sspr | 0.182 | 0.202 | 0.203 | 0.90 | 0.002 |
| ssao | 0.169 | 0.188 | 0.191 | 0.90 | 0.007 |
| point_shadow_atlas | 0.146 | 0.270 | 0.470 | 0.54 | 0.240 |
| upscaling | 0.116 | 0.129 | 0.138 | 0.90 | 0.004 |
| atmos_froxel_inject | 0.115 | 0.128 | 0.130 | 0.90 | 0.002 |
| water_fft | 0.110 | 0.122 | 0.125 | 0.90 | 0.003 |
| uniform_allocation | 0.101 | 0.112 | 0.140 | 0.90 | 0.021 |
| clouds_composite | 0.095 | 0.105 | 0.109 | 0.90 | 0.002 |
| gpu_instance_materialization | 0.086 | 0.096 | 0.100 | 0.90 | 0.004 |
| bin_unpacking | 0.083 | 0.092 | 0.113 | 0.90 | 0.016 |
| tonemapping | 0.078 | 0.087 | 0.092 | 0.90 | 0.004 |
| main_transmissive_pass_3d | 0.064 | 0.070 | 0.145 | 0.90 | 0.008 |
| clustering | 0.058 | 0.064 | 0.068 | 0.90 | 0.019 |
| clouds_trace | 0.050 | 0.056 | 0.079 | 0.90 | 0.001 |
| early_mesh_preprocessing | 0.048 | 0.053 | 0.069 | 0.90 | 0.003 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.90 | 0.002 |
| clouds_resolve | 0.045 | 0.050 | 0.051 | 0.90 | 0.001 |
| surface_trail | 0.034 | 0.057 | 0.057 | 0.59 | 0.002 |
| lightprobe_irradiance_map | 0.033 | 0.414 | 0.417 | 0.08 | 0.001 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.031 | 0.90 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.026 | 0.049 | 0.069 | 0.54 | 0.009 |
| water_snapshot | 0.022 | 0.024 | 0.027 | 0.90 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.90 | 0.002 |
| range_unpacking | 0.020 | 0.022 | 0.027 | 0.90 | 0.003 |
| lightprobe_radiance_map | 0.018 | 0.220 | 0.230 | 0.08 | 0.004 |
| main_transparent_pass_3d | 0.015 | 0.017 | 0.030 | 0.90 | 0.181 |
| per_view_shadow_pass | 0.015 | 0.017 | 0.027 | 0.90 | 0.005 |
| night_sky | 0.015 | 0.017 | 0.044 | 0.90 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.015 | 0.027 | 0.056 | 0.54 | 0.008 |
| point_shadow_atlas/bin_unpacking | 0.011 | 0.019 | 0.035 | 0.54 | 0.006 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.90 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.90 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.90 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.54 | 0.002 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.90 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.08 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.08 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.90 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.08 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.659 | 10.011 | 19.35 | 57566.7 |
| render/busy/ms | 8.131 | 9.437 | 18.72 | 54057.2 |
| main/busy/ms | 8.045 | 9.189 | 29.99 | 53486.4 |
| stage/render/render | 5.037 | 6.040 | 11.34 | 33486.4 |
| stage/main/post_update | 4.534 | 5.326 | 10.17 | 30143.9 |
| render/graph/ms | 4.356 | 5.209 | 9.89 | 28959.5 |
| encode//sum | 3.189 | 3.979 | 8.34 | 21200.7 |
| stage/render/graph_submit | 3.058 | 3.859 | 7.85 | 20328.4 |
| encode/submit | 1.941 | 2.820 | 6.70 | 12902.6 |
| stage/main/update | 1.830 | 2.303 | 23.64 | 12165.6 |
| stage/render/camera_driver | 1.237 | 1.490 | 2.93 | 8220.7 |
| encode/finish_wall | 1.205 | 1.581 | 2.96 | 8010.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.806 | 1.071 | 1.81 | 5360.9 |
| encode/pass:early prepass/sum | 0.805 | 1.053 | 1.82 | 5350.4 |
| encode/pass:shadow_directional_light/sum | 0.803 | 1.202 | 2.27 | 5335.1 |
| stage/render/prepare_write_phase_buffers | 0.607 | 0.898 | 1.63 | 4034.4 |
| bigspace/total/ms | 0.553 | 1.295 | 3.08 | 3673.8 |
| stage/render/prepare_assets | 0.505 | 0.652 | 10.45 | 3358.9 |
| stage/main/fixed_main_loop | 0.502 | 0.927 | 1.99 | 3337.1 |
| render/present/ms | 0.433 | 1.213 | 1.96 | 2881.3 |
| stage/render/prepare_resources | 0.425 | 0.503 | 0.86 | 2823.0 |
| stage/render/prepare_meshes | 0.421 | 0.518 | 3.54 | 2797.0 |
| stage/main/pre_update | 0.383 | 0.541 | 3.09 | 2545.9 |
| stage/main/first | 0.364 | 0.478 | 5.76 | 2417.0 |
| controller/feed/ms | 0.322 | 0.403 | 0.79 | 2141.1 |
| stage/render/prepare_flush | 0.316 | 0.499 | 1.16 | 2101.5 |
| stage/render/prepare_bind_groups | 0.301 | 0.494 | 1.65 | 2004.1 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.283 | 1.097 | 2.17 | 1879.3 |
| stage/render/prepare_batch_phases | 0.271 | 0.409 | 1.08 | 1804.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.261 | 0.428 | 2.17 | 1737.0 |
| stage/main/last | 0.253 | 0.300 | 0.81 | 1683.2 |
| bigspace/low_precision/ms | 0.239 | 0.735 | 1.87 | 1587.8 |
| stream/drive/ms | 0.199 | 0.270 | 10.69 | 1319.8 |
| stage/render/create_views | 0.196 | 0.263 | 0.51 | 1300.6 |
| stage/render/extract_commands | 0.187 | 0.219 | 0.57 | 1244.2 |
| bigspace/high_precision/ms | 0.168 | 0.707 | 2.52 | 1119.2 |
| anim/evaluate/ms | 0.160 | 0.280 | 1.62 | 1065.9 |
| anim/evaluate_cpu/ms | 0.153 | 0.190 | 0.56 | 1018.6 |
| stage/render/specialize | 0.152 | 0.241 | 1.58 | 1010.9 |
| stream/tick/ms | 0.146 | 0.208 | 0.46 | 972.0 |
| stage/render/prepare_views | 0.135 | 0.174 | 1.47 | 900.1 |
| pipelines/compile_ms | 0.108 | 0.000 | 127.55 | 717.8 |
| pipelines/front_ms | 0.105 | 0.000 | 124.62 | 698.1 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.101 | 0.856 | 1.39 | 670.2 |
| stage/render/queue | 0.096 | 0.168 | 1.60 | 641.0 |
| render/acquire/ms | 0.094 | 0.123 | 1.43 | 628.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.089 | 0.140 | 0.73 | 590.1 |
| stage/render/phase_sort | 0.080 | 0.109 | 0.38 | 528.6 |
| controller/write/ms | 0.073 | 0.099 | 0.51 | 487.3 |
| controller/move/ms | 0.068 | 0.186 | 0.56 | 449.2 |
| terrain/update/ms | 0.059 | 0.289 | 1.78 | 390.2 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.050 | 0.000 | 2.82 | 330.6 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.074 | 0.46 | 313.7 |
| physics/step/ms | 0.044 | 0.105 | 0.59 | 290.5 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 0.48 | 204.8 |
| stage/render/cleanup | 0.024 | 0.034 | 0.43 | 160.6 |
| physics/insert/ms | 0.023 | 0.288 | 0.85 | 156.0 |
| terrain/select/ms | 0.022 | 0.256 | 0.73 | 144.4 |
| weather/map/ms | 0.018 | 0.024 | 0.18 | 118.4 |
| far/select/ms | 0.015 | 0.165 | 0.46 | 97.6 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.52 | 84.9 |
| anim/apply/ms | 0.011 | 0.021 | 0.18 | 76.2 |
| encode/camera_driver_span_end/sum | 0.010 | 0.020 | 0.57 | 69.5 |
| weather/eval/ms | 0.010 | 0.014 | 0.08 | 69.0 |
| script/update/ms | 0.008 | 0.011 | 0.30 | 51.4 |
| controller/avoid/ms | 0.006 | 0.012 | 0.04 | 39.3 |
| stream/despawn/ms | 0.005 | 0.000 | 0.99 | 34.8 |
| script/input/ms | 0.005 | 0.006 | 0.17 | 31.3 |
| stream/spawn/ms | 0.003 | 0.000 | 1.14 | 21.9 |
| script/ui/ms | 0.003 | 0.005 | 0.04 | 21.5 |
| pipelines/driver_ms | 0.003 | 0.000 | 3.02 | 19.7 |
| stream/upload/ms | 0.003 | 0.000 | 1.03 | 18.1 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 13.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 7.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.47 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.5 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 33 | 17 |
| controller/movers/count | count | 19 | 22 | 17 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1313 | 2262 | 641 |
| crowd/c0/meshes/count | count | 357 | 646 | 174 |
| crowd/c1/count | count | 146 | 156 | 147 |
| crowd/c1/joints/count | count | 5531 | 5997 | 5555 |
| crowd/c1/meshes/count | count | 1181 | 1299 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 215 | 205 |
| crowd/joints_extracted/count | count | 695 | 6935 | 451 |
| crowd/joints_uploaded/count | count | 3616 | 6186 | 5763 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.4 k | 47.7 k |
| encode//max | ms | 0.39 | 1.17 | 0.45 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.52 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.57 | 0.01 |
| encode/finish_critical | ms | 0.95 | 2.82 | 1.11 |
| encode/pass:early prepass/max | ms | 0.81 | 1.82 | 0.79 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 1.81 | 1.11 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.73 | 0.06 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 2.17 | 0.38 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.78 | 2.17 | 0.82 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.39 | 1.39 | 0.14 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.12 | 2.82 | 1.26 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.27 | 0.57 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.48 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.56 | 29.99 | 8.81 |
| gpu/timed/ms | ms | 7.91 | 9.41 | 8.81 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.5 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 318 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 318 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 766 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2482 | 3551 | 2458 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 766 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2482 | 3551 | 2458 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 70 | 462 | 8 |
| phases/shadow/bins | count | 878 | 1546 | 658 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2346 | 3323 | 2022 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 54 | 151 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2679 | 3182 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 330 | 331 | 331 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 15 | 288 | 9 |
| preprocess/bind_groups_created | count | 10 | 282 | 4 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 136 | 224 | 125 |
| process/cpu_usage | % | 25.48 | 41.83 | 31.73 |
| process/mem_usage | GiB | 5.11 | 5.43 | 5.43 |
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
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 6 | 0 |
| shadow_atlas/empty_faces | count | 0 | 2 | 0 |
| shadow_atlas/lamps | count | 8 | 17 | 0 |
| shadow_atlas/live_occ_pct | % | 11.33 | 29.49 | 0 |
| shadow_atlas/live_tiles | count | 18 | 62 | 0 |
| shadow_atlas/mover_draws | count | 10 | 37 | 0 |
| shadow_atlas/movers | count | 144 | 475 | 0 |
| shadow_atlas/moving_lamps | count | 1 | 2 | 0 |
| shadow_atlas/render/blits | count | 10 | 37 | 0 |
| shadow_atlas/render/clears | count | 0 | 18 | 0 |
| shadow_atlas/render/lamps | count | 8 | 17 | 0 |
| shadow_atlas/render/live_direct | count | 10 | 37 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 12 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 17 | 55 | 0 |
| shadow_atlas/resizes | count | 32 | 41 | 41 |
| shadow_atlas/static_occ_pct | % | 16.41 | 36.33 | 0 |
| shadow_atlas/static_tiles | count | 42 | 95 | 0 |
| shadow_atlas/views | count | 17 | 55 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 121 | 11 |
| skin/blocks_skipped/count | count | 145 | 205 | 194 |
| skin/blocks_walked/count | count | 14 | 28 | 11 |
| skin/joint_bytes_written/bytes | bytes | 48.9 k | 810.2 k | 33.8 k |
| skin/joints_written/count | count | 764 | 12.7 k | 528 |
| skin/upload_writes/count | count | 8 | 29 | 12 |
| started_load_count |  | 6439 | 6563 | 6563 |
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
| stream/latency/s | s | 0.04 | 0.07 | 0.06 |
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
| system/cpu_usage | % | 29.41 | 41.78 | 34.72 |
| system/mem_usage | % | 46.24 | 47.17 | 47.17 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 130 | 517 | 66 |
| views/all/bins | count | 2935 | 3578 | 2948 |
| views/all/count | count | 20 | 58 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 8058 | 10.6 k | 7968 |
| views/all/pipelines | count | 101 | 198 | 66 |
| views/all/pipelines_distinct | count | 66 | 75 | 61 |
| views/all/work_items | count | 154.1 k | 231.1 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 33 | 8 |
| views/cascade/bins | count | 716 | 853 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2122 | 2454 | 2022 |
| views/cascade/pipelines | count | 24 | 27 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 59 | 434 | 0 |
| views/lamp_face/bins | count | 126 | 714 | 0 |
| views/lamp_face/count | count | 17 | 55 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 179 | 877 | 0 |
| views/lamp_face/pipelines | count | 29 | 125 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 128 | 25.2 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2010 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5912 | 7522 | 5946 |
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
| anim/bones_written/count | count | 49.80 | 223 | 331.1 k |
| anim/events/count | count | 0.08 | 3 | 544 |
| assets/Image/added | count | 0.09 | 35 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 11 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.43 | 305 | 2874 |
| assets/Mesh/removed | count | 0.73 | 1242 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.13 | 10 | 7480 |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.22 | 1245 | 1490 |
| encode//count | count | 27.91 | 30 | 185.5 k |
| encode/bind_walk/init_textures | count | 9054 | 22.8 k | 60.19 M |
| encode/bind_walk/init_textures_skipped | count | 48.0 k | 75.8 k | 318.95 M |
| encode/bind_walk/inits | count | 567 | 785 | 3.77 M |
| encode/bind_walk/inits_skipped | count | 167 | 247 | 1.11 M |
| encode/bind_walk/merge_textures | count | 8992 | 22.7 k | 59.78 M |
| encode/bind_walk/merge_textures_skipped | count | 10.5 k | 29.0 k | 69.59 M |
| encode/bind_walk/merges | count | 106 | 180 | 703.3 k |
| encode/bind_walk/merges_skipped | count | 150 | 221 | 999.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6648 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6648 |
| encode/pass:early prepass/count | count | 1 | 1 | 6648 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6648 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6648 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6648 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.34 | 1 | 2288 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.18 | 1 | 1218 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.04 | 1 | 279 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6648 |
| encode/queue/command_buffers | count | 38.49 | 42 | 255.9 k |
| encode/queue/submits | count | 2.01 | 4 | 13.4 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.31 M | 15.45 G |
| encode/queue/write_buffer/calls | count | 275 | 455 | 1.83 M |
| encode/queue/write_buffer_with/bytes | bytes | 160.1 k | 1.29 M | 1.06 G |
| encode/queue/write_buffer_with/calls | count | 10.28 | 924 | 68.4 k |
| encode/queue/write_texture/bytes | bytes | 1985 | 1.05 M | 13.19 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6648 |
| physics/inserts | count | 0.34 | 87 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 175 | 2335 |
| physics/steps | count | 0.59 | 2 | 3939 |
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
| stream/deactivate/items | count | 0.03 | 31 | 223 |
| stream/despawn/items | count | 0.03 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5092 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1116 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 43, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.39 | 2385.1 | main/busy/ms 2355, stage/main/post_update 2313, stage/main/update 31.35, stage/main/pre_update 7.70, stage/main/last 1.00, surface/advance_cpu_ms 0.79, stage/main/first 0.60, stage/main/fixed_main_loop 0.37, surface/cell_upload_ms 0.33, stream/drive/ms 0.26, bigspace/total/ms 0.08, stream/tick/ms 0.07 |
| warm-up | 3.12 | 738.7 | main/busy/ms 2487, stage/main/post_update 2338, render/thread/ms 704, render/busy/ms 565, stage/render/prepare_assets 502, stage/render/prepare_views 137, render/acquire/ms 136, stage/main/update 131, stage/render/create_views 29.85, stage/render/render 23.16, render/graph/ms 18.85, stage/render/camera_driver 16.93 |
| warm-up | 3.25 | 125.1 | render/thread/ms 812, render/busy/ms 607, stage/render/prepare_assets 532, main/busy/ms 158, stage/render/prepare_views 137, render/acquire/ms 137, stage/main/update 107, stage/render/render 92.75, pipelines/compile_ms 70.56, render/present/ms 67.79, pipelines/driver_ms 50.07, stage/main/post_update 36.84 |
| warm-up | 3.28 | 28.1 | pipelines/compile_ms 382, pipelines/front_ms 262, render/thread/ms 135, pipelines/driver_ms 120, stage/render/render 89.96, main/busy/ms 83.07, render/present/ms 77.80, stage/main/post_update 60.09, render/busy/ms 56.01, stage/render/prepare_assets 30.54, terrain/update/ms 27.23, pipelines/wait_ms 19.78 |
| warm-up | 3.41 | 131.1 | pipelines/wait_ms 4622, pipelines/compile_ms 1627, pipelines/front_ms 1439, pipelines/driver_ms 187, render/thread/ms 126, main/busy/ms 112, render/busy/ms 111, stage/main/post_update 93.79, encode//sum 46.64, stage/render/render 37.15, pipelines/lock_ms 27.13, stage/render/prepare_write_phase_buffers 24.51 |
| warm-up | 3.45 | 39.4 | pipelines/wait_ms 5186, pipelines/compile_ms 1632, pipelines/front_ms 1503, render/thread/ms 136, render/busy/ms 133, pipelines/driver_ms 129, main/busy/ms 127, stage/main/post_update 57.10, stage/main/update 55.58, encode//sum 49.58, stage/render/prepare_assets 33.02, stage/render/prepare_write_phase_buffers 24.21 |
| warm-up | 3.57 | 120.5 | pipelines/compile_ms 1535, pipelines/front_ms 1465, pipelines/wait_ms 947, render/thread/ms 116, render/busy/ms 113, main/busy/ms 95.33, stage/render/prepare_assets 86.00, pipelines/driver_ms 69.76, stage/main/update 62.97, stage/main/post_update 20.99, terrain/update/ms 14.99, stage/render/prepare_batch_phases 10.45 |
| warm-up | 3.61 | 44.8 | pipelines/compile_ms 1859, pipelines/front_ms 1770, pipelines/wait_ms 428, render/thread/ms 122, render/busy/ms 120, stage/render/prepare_assets 101, pipelines/driver_ms 89.00, main/busy/ms 52.68, stage/main/post_update 19.58, stage/main/update 18.46, encode//sum 15.35, stage/render/render 13.19 |
| warm-up | 3.72 | 35.4 | pipelines/compile_ms 185, pipelines/front_ms 175, main/busy/ms 76.92, stage/main/update 60.58, render/thread/ms 48.91, render/busy/ms 46.49, stage/render/prepare_assets 21.88, stage/render/render 16.43, render/graph/ms 13.70, pipelines/driver_ms 10.67, stage/render/graph_submit 10.27, stage/main/post_update 9.85 |
| warm-up | 3.81 | 97.3 | pipelines/compile_ms 1129, pipelines/front_ms 1080, main/busy/ms 71.38, render/thread/ms 64.55, render/busy/ms 62.75, stage/main/update 58.97, pipelines/driver_ms 48.99, stage/render/prepare_assets 41.57, stage/render/render 14.35, render/graph/ms 12.02, stage/render/graph_submit 9.41, encode//sum 7.82 |
| warm-up | 3.85 | 40.2 | pipelines/compile_ms 1262, pipelines/front_ms 1206, pipelines/driver_ms 56.44, render/thread/ms 55.40, render/busy/ms 54.12, stage/render/prepare_assets 40.99, main/busy/ms 18.38, stage/render/render 9.40, stage/main/update 8.15, render/graph/ms 7.81, pipelines/wait_ms 6.60, stage/render/graph_submit 5.86 |
| warm-up | 3.96 | 34.1 | main/busy/ms 106, stage/main/update 94.38, render/thread/ms 43.59, render/busy/ms 41.38, stage/render/prepare_assets 18.75, pipelines/compile_ms 16.64, pipelines/front_ms 15.48, stage/render/render 14.45, render/graph/ms 11.50, encode//sum 9.28, stage/render/graph_submit 7.97, stage/main/post_update 5.04 |
| warm-up | 4.13 | 171.2 | pipelines/compile_ms 1275, pipelines/front_ms 1237, main/busy/ms 108, render/thread/ms 100, render/busy/ms 98.35, stage/main/update 95.96, stage/render/prepare_assets 77.35, pipelines/driver_ms 38.27, stage/render/render 13.85, render/graph/ms 11.44, stage/render/graph_submit 8.52, encode//sum 7.85 |
| warm-up | 4.17 | 40.9 | pipelines/compile_ms 1398, pipelines/front_ms 1354, render/thread/ms 111, render/busy/ms 110, stage/render/prepare_assets 67.48, main/busy/ms 51.61, pipelines/driver_ms 44.07, stage/main/update 35.72, stage/render/render 23.23, render/graph/ms 21.27, stage/render/graph_submit 12.91, encode/submit 8.91 |
| warm-up | 4.23 | 60.1 | pipelines/compile_ms 247, pipelines/front_ms 237, main/busy/ms 84.98, render/thread/ms 84.30, render/busy/ms 82.31, pipelines/wait_ms 68.30, stage/main/update 65.88, stage/render/render 28.27, render/graph/ms 25.87, stage/render/prepare_assets 18.95, stage/render/graph_submit 16.31, pipelines/lock_ms 14.52 |
| warm-up | 4.31 | 81.4 | pipelines/compile_ms 134, pipelines/front_ms 129, render/thread/ms 112, render/busy/ms 110, main/busy/ms 98.56, pipelines/wait_ms 64.77, stage/main/update 50.80, stage/main/post_update 42.02, stage/render/prepare_assets 35.99, stage/render/render 16.94, render/graph/ms 14.63, stage/render/prepare_resources 14.47 |
| warm-up | 4.36 | 47.3 | pipelines/compile_ms 1648, pipelines/front_ms 1590, render/thread/ms 103, render/busy/ms 101, pipelines/wait_ms 69.94, main/busy/ms 68.23, pipelines/driver_ms 58.28, stage/main/post_update 38.22, stage/render/prepare_assets 36.78, stage/main/update 23.48, stage/render/prepare_resources 15.52, stage/render/render 12.33 |
| warm-up | 4.43 | 72.2 | pipelines/compile_ms 2663, pipelines/front_ms 2574, pipelines/wait_ms 282, render/thread/ms 103, render/busy/ms 101, pipelines/driver_ms 89.16, main/busy/ms 76.50, stage/main/update 47.30, stage/render/prepare_assets 26.80, stage/main/post_update 21.94, stage/render/prepare_flush 20.19, stream/drive/ms 19.13 |
| warm-up | 4.51 | 83.4 | pipelines/compile_ms 1258, pipelines/front_ms 1221, pipelines/wait_ms 213, render/thread/ms 134, render/busy/ms 132, main/busy/ms 119, stage/main/update 75.13, stage/render/prepare_assets 44.61, pipelines/driver_ms 37.01, stage/main/post_update 32.48, stage/render/prepare_flush 31.66, stream/drive/ms 20.72 |
| warm-up | 4.59 | 82.4 | pipelines/compile_ms 2239, pipelines/front_ms 2173, pipelines/wait_ms 212, render/thread/ms 113, render/busy/ms 112, main/busy/ms 86.42, stage/render/prepare_assets 70.32, pipelines/driver_ms 65.57, stage/main/update 52.12, stage/main/post_update 20.94, stage/render/render 13.62, stage/render/prepare_flush 12.54 |
| warm-up | 4.64 | 41.7 | pipelines/compile_ms 2480, pipelines/front_ms 2410, pipelines/wait_ms 263, render/thread/ms 74.05, render/busy/ms 72.79, pipelines/driver_ms 69.88, stage/render/prepare_assets 52.63, main/busy/ms 45.24, stage/main/update 24.99, stage/render/render 13.29, render/graph/ms 11.56, stage/main/post_update 8.83 |
| warm-up | 4.68 | 41.5 | pipelines/compile_ms 565, pipelines/front_ms 547, main/busy/ms 74.78, render/thread/ms 65.82, render/busy/ms 63.68, pipelines/wait_ms 53.86, stage/main/update 41.48, stream/drive/ms 27.25, stage/main/post_update 22.80, stage/render/prepare_assets 20.88, stage/render/render 20.07, render/graph/ms 18.12 |
| warm-up | 4.72 | 44.7 | pipelines/compile_ms 327, pipelines/front_ms 313, main/busy/ms 118, stage/main/update 83.44, render/thread/ms 58.06, render/busy/ms 55.93, stream/drive/ms 49.86, pipelines/wait_ms 32.78, stage/main/post_update 24.35, stage/render/render 18.76, render/graph/ms 16.68, pipelines/driver_ms 14.27 |
| warm-up | 4.81 | 89.9 | pipelines/compile_ms 824, pipelines/front_ms 801, main/busy/ms 124, stage/main/update 106, stream/drive/ms 62.73, render/thread/ms 56.73, render/busy/ms 55.08, pipelines/wait_ms 31.81, pipelines/driver_ms 23.22, stage/render/prepare_assets 20.29, stage/render/render 14.35, render/graph/ms 11.90 |
| warm-up | 4.87 | 61.1 | pipelines/compile_ms 2210, pipelines/front_ms 2154, main/busy/ms 77.53, stage/main/update 62.51, render/thread/ms 60.51, render/busy/ms 58.96, pipelines/driver_ms 56.50, stream/drive/ms 39.55, stage/render/prepare_assets 26.47, pipelines/wait_ms 16.26, stage/render/render 13.55, render/graph/ms 11.17 |
| warm-up | 4.93 | 25.4 | pipelines/compile_ms 228, pipelines/front_ms 221, render/thread/ms 38.72, main/busy/ms 36.47, render/busy/ms 36.15, stage/render/render 21.05, stage/main/update 20.78, render/graph/ms 17.97, stage/render/graph_submit 12.72, stage/main/post_update 11.63, pipelines/driver_ms 7.27, encode//sum 7.03 |
| warm-up | 4.96 | 30.1 | pipelines/compile_ms 245, pipelines/front_ms 237, render/thread/ms 45.61, render/busy/ms 44.42, main/busy/ms 40.83, stage/main/update 25.12, stage/render/render 18.06, render/graph/ms 16.31, stage/render/prepare_assets 13.34, stage/render/graph_submit 12.26, stage/main/post_update 10.44, encode//sum 7.61 |
| warm-up | 5.05 | 32.7 | pipelines/compile_ms 662, pipelines/front_ms 641, render/thread/ms 43.91, render/busy/ms 42.31, main/busy/ms 38.16, stage/main/update 24.75, pipelines/driver_ms 21.11, stream/drive/ms 17.63, stage/render/render 13.22, stage/render/prepare_flush 11.95, render/graph/ms 11.18, stage/main/post_update 9.50 |
| warm-up | 5.20 | 27.3 | pipelines/compile_ms 1127, pipelines/front_ms 1087, pipelines/driver_ms 39.54, render/thread/ms 37.06, render/busy/ms 35.99, main/busy/ms 31.40, stage/main/update 16.77, stage/render/prepare_assets 12.04, stage/render/render 9.77, stage/main/post_update 8.66, render/graph/ms 8.19, stream/drive/ms 7.73 |
| warm-up | 5.57 | 25.0 | pipelines/compile_ms 140, pipelines/front_ms 136, pipelines/wait_ms 54.56, main/busy/ms 30.92, render/thread/ms 20.33, stage/main/update 20.16, render/busy/ms 19.27, stream/drive/ms 11.93, stage/render/render 8.00, render/graph/ms 6.66, stage/main/post_update 6.46, encode//sum 5.70 |
| warm-up | 5.64 | 26.9 | pipelines/compile_ms 133, pipelines/front_ms 129, render/thread/ms 34.38, render/busy/ms 32.87, main/busy/ms 28.25, stage/render/render 14.01, stage/main/update 13.33, render/graph/ms 11.94, stage/main/post_update 10.12, stage/render/prepare_assets 9.43, encode//sum 9.42, stage/render/graph_submit 8.15 |
| warm-up | 5.70 | 34.0 | render/thread/ms 41.46, render/busy/ms 40.37, main/busy/ms 28.87, pipelines/compile_ms 23.56, pipelines/front_ms 21.98, stage/render/prepare_assets 21.88, stage/main/update 13.54, stage/render/render 9.09, stage/main/post_update 8.22, render/graph/ms 7.51, encode//sum 7.26, stage/render/graph_submit 4.55 |
| warm-up | 5.91 | 28.0 | render/thread/ms 42.77, render/busy/ms 41.17, main/busy/ms 40.18, stage/render/render 20.14, stage/main/update 19.15, render/graph/ms 17.73, stage/main/post_update 13.97, stage/render/graph_submit 12.47, pipelines/compile_ms 10.30, pipelines/front_ms 9.57, encode/submit 9.16, encode//sum 8.95 |
| warm-up | 5.94 | 28.4 | render/thread/ms 48.93, render/busy/ms 47.46, main/busy/ms 42.73, stage/main/update 21.37, stage/render/render 21.02, render/graph/ms 18.89, stage/main/post_update 14.33, stage/render/graph_submit 14.29, encode/submit 8.27, encode//sum 8.25, stage/render/prepare_assets 8.09, encode/finish_wall 6.14 |
| warm-up | 5.98 | 41.8 | render/thread/ms 56.24, render/busy/ms 55.14, main/busy/ms 41.30, stage/main/update 23.02, stage/render/prepare_assets 19.21, stage/render/render 17.41, render/graph/ms 15.77, stage/render/graph_submit 12.85, stage/main/post_update 10.27, encode/finish_wall 8.59, encode/pass:point_shadow_atlas_live_indirect/sum 8.31, encode//sum 6.80 |
| warm-up | 6.01 | 29.4 | render/thread/ms 57.93, render/busy/ms 56.88, main/busy/ms 45.19, stage/main/update 24.42, stage/render/prepare_assets 19.82, stage/render/render 18.59, render/graph/ms 16.90, stage/render/graph_submit 12.96, stage/main/post_update 10.87, encode/finish_wall 8.67, encode/pass:point_shadow_atlas_live_indirect/sum 8.30, encode//sum 7.47 |
| warm-up | 6.04 | 37.6 | pipelines/compile_ms 142, pipelines/front_ms 138, render/thread/ms 59.96, render/busy/ms 58.89, main/busy/ms 39.40, stage/render/render 20.55, stage/main/update 18.97, render/graph/ms 18.76, stage/render/prepare_assets 14.32, stage/render/graph_submit 13.54, stage/main/post_update 13.03, encode/finish_wall 9.48 |
| warm-up | 6.10 | 35.2 | render/thread/ms 42.04, render/busy/ms 40.97, main/busy/ms 30.01, stage/main/update 17.06, stage/render/render 16.60, render/graph/ms 14.99, stage/render/prepare_assets 14.65, pipelines/compile_ms 12.17, stage/render/graph_submit 12.10, pipelines/front_ms 11.41, encode/finish_wall 8.37, encode/pass:point_shadow_atlas_live_indirect/sum 8.11 |
| warm-up | 6.23 | 37.9 | main/busy/ms 54.76, render/thread/ms 46.37, render/busy/ms 44.79, stage/main/update 38.83, stage/render/render 23.56, render/graph/ms 21.21, stage/render/graph_submit 17.62, stream/drive/ms 14.20, stage/main/post_update 10.68, pipelines/compile_ms 10.61, encode/finish_wall 10.32, encode/pass:point_shadow_atlas_live_indirect/sum 9.97 |
| warm-up | 6.26 | 33.5 | render/thread/ms 53.36, render/busy/ms 51.94, main/busy/ms 48.47, stage/main/update 31.12, stage/render/prepare_assets 19.17, stage/render/render 18.44, render/graph/ms 16.32, stage/render/graph_submit 12.82, stage/main/post_update 9.99, encode/finish_wall 9.30, encode/pass:point_shadow_atlas_live_indirect/sum 9.03, encode//sum 8.01 |
| warm-up | 6.30 | 33.0 | render/thread/ms 50.01, render/busy/ms 49.04, main/busy/ms 35.94, stage/render/prepare_assets 24.95, stage/main/update 20.97, stage/render/render 13.00, render/graph/ms 11.50, stage/render/graph_submit 8.96, stage/main/post_update 7.95, stream/drive/ms 7.83, encode//sum 6.43, encode/finish_wall 6.05 |
| measure | 18.37 | 32.2 | main/busy/ms 38.10, stage/main/update 25.41, render/thread/ms 20.63, render/busy/ms 19.65, stage/render/render 9.83, stage/main/post_update 8.67, render/graph/ms 8.47, encode//sum 5.69, stage/render/graph_submit 5.47, stage/render/prepare_assets 4.24, encode/submit 3.56, stage/render/camera_driver 2.87 |
| measure | 59.17 | 28.1 | render/thread/ms 37.55, render/busy/ms 36.48, main/busy/ms 26.90, stage/render/prepare_assets 15.59, stage/main/post_update 14.12, stage/render/render 11.72, render/graph/ms 10.17, encode//sum 7.56, stage/render/graph_submit 6.49, stage/main/update 4.92, encode/submit 3.90, stage/render/camera_driver 3.51 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.soft_pcf_radius=0 --set shadow.lamp.atlas.max_lamps=16 --resolution 2560x1440 --bench-out target/perf\shadow_review\balmora_walk_night@2560x1440_arm=pcf0lamps16-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **85%** (min 49%) · 2620 MHz (min 2610) · memory 8101 MHz · 84.8 W · 79 °C (max 85) · VRAM 5516 MiB · throttled 100% of samples

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
- `shadow.lamp.atlas.max_lamps=16 (command line: --set)`
- `shadow.lamp.atlas.soft_pcf_radius=0 (command line: --set)`
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
