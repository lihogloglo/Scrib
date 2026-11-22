# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.max_lamps=16 (command line: --set); shadow.lamp.atlas.soft_pcf_radius=0 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6563 frames over 65.6 s · **median 9.90 ms (101 fps)** · mean 10.00 · p95 11.64 · p99 13.48 (1% low 74 fps) · worst 31.21 ms · 33 spikes over 1.5× median · 27.4 stutters/min (work 22.9/min)
- warm-up: 354 frames over 8.9 s, worst 2394.7 ms
- hitches (> 25.0 ms): **4** measured, 42 in warm-up
- timed GPU passes: 6.45 ms of a 10.00 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.90 ms, limit 11 ms
- **FAIL** frame_ms: 30 stutters (> 1.5x moving median of 60) in 1.09 min = 27.42/min, limit 1/min
- pass worst frame 31.21 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.856 | 2.032 | 2.066 | 0.91 | 0.175 |
| main_opaque_pass_3d | 1.488 | 1.630 | 2.347 | 0.91 | 0.071 |
| early prepass | 0.468 | 0.512 | 0.752 | 0.91 | 0.088 |
| bloom | 0.290 | 0.317 | 0.325 | 0.91 | 0.063 |
| atmosphere_luts | 0.286 | 0.313 | 0.379 | 0.91 | 0.003 |
| water_velocity | 0.205 | 0.225 | 0.246 | 0.91 | 0.002 |
| water_sspr | 0.184 | 0.202 | 0.203 | 0.91 | 0.002 |
| ssao | 0.171 | 0.188 | 0.191 | 0.91 | 0.008 |
| point_shadow_atlas | 0.135 | 0.245 | 0.472 | 0.55 | 0.234 |
| upscaling | 0.117 | 0.128 | 0.135 | 0.91 | 0.004 |
| atmos_froxel_inject | 0.117 | 0.128 | 0.129 | 0.91 | 0.001 |
| water_fft | 0.112 | 0.123 | 0.126 | 0.91 | 0.003 |
| uniform_allocation | 0.106 | 0.116 | 0.141 | 0.91 | 0.023 |
| clouds_composite | 0.096 | 0.105 | 0.109 | 0.91 | 0.002 |
| gpu_instance_materialization | 0.088 | 0.096 | 0.100 | 0.91 | 0.004 |
| bin_unpacking | 0.084 | 0.093 | 0.113 | 0.91 | 0.017 |
| tonemapping | 0.079 | 0.087 | 0.091 | 0.91 | 0.004 |
| main_transmissive_pass_3d | 0.064 | 0.070 | 0.140 | 0.91 | 0.008 |
| clustering | 0.059 | 0.064 | 0.068 | 0.91 | 0.019 |
| clouds_trace | 0.051 | 0.055 | 0.078 | 0.91 | 0.001 |
| early_mesh_preprocessing | 0.049 | 0.054 | 0.070 | 0.91 | 0.004 |
| render_sky | 0.048 | 0.052 | 0.053 | 0.91 | 0.002 |
| clouds_resolve | 0.046 | 0.050 | 0.051 | 0.91 | 0.001 |
| lightprobe_irradiance_map | 0.035 | 0.414 | 0.417 | 0.08 | 0.001 |
| surface_trail | 0.034 | 0.057 | 0.057 | 0.61 | 0.002 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.031 | 0.91 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.026 | 0.047 | 0.066 | 0.55 | 0.011 |
| water_snapshot | 0.022 | 0.024 | 0.027 | 0.91 | 0.002 |
| range_unpacking | 0.021 | 0.023 | 0.028 | 0.91 | 0.003 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.91 | 0.002 |
| lightprobe_radiance_map | 0.018 | 0.220 | 0.231 | 0.08 | 0.004 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.91 | 0.005 |
| night_sky | 0.015 | 0.017 | 0.044 | 0.91 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.031 | 0.91 | 0.181 |
| point_shadow_atlas/uniform_allocation | 0.014 | 0.025 | 0.059 | 0.55 | 0.007 |
| point_shadow_atlas/bin_unpacking | 0.010 | 0.018 | 0.038 | 0.55 | 0.005 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.91 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.91 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.91 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.55 | 0.002 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.91 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.08 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.08 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.91 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.08 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.793 | 10.113 | 20.65 | 57709.2 |
| render/busy/ms | 8.271 | 9.595 | 20.04 | 54285.9 |
| main/busy/ms | 8.176 | 9.383 | 29.51 | 53659.9 |
| stage/render/render | 5.123 | 6.085 | 10.11 | 33620.4 |
| stage/main/post_update | 4.623 | 5.469 | 8.33 | 30340.9 |
| render/graph/ms | 4.447 | 5.282 | 8.95 | 29184.8 |
| encode//sum | 3.332 | 4.080 | 7.80 | 21865.7 |
| stage/render/graph_submit | 3.124 | 3.901 | 6.53 | 20503.4 |
| encode/submit | 1.957 | 2.802 | 4.60 | 12840.7 |
| stage/main/update | 1.847 | 2.314 | 24.22 | 12123.4 |
| stage/render/camera_driver | 1.260 | 1.523 | 3.04 | 8271.5 |
| encode/finish_wall | 1.248 | 1.617 | 3.06 | 8191.2 |
| encode/pass:early prepass/sum | 0.835 | 1.074 | 2.10 | 5480.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.834 | 1.088 | 1.85 | 5473.3 |
| encode/pass:shadow_directional_light/sum | 0.832 | 1.233 | 1.81 | 5460.6 |
| stage/render/prepare_write_phase_buffers | 0.596 | 0.896 | 1.92 | 3909.6 |
| bigspace/total/ms | 0.556 | 1.329 | 3.34 | 3647.9 |
| stage/render/prepare_assets | 0.516 | 0.654 | 9.87 | 3386.8 |
| stage/main/fixed_main_loop | 0.512 | 0.946 | 2.40 | 3363.3 |
| render/present/ms | 0.430 | 1.201 | 1.97 | 2821.3 |
| stage/render/prepare_meshes | 0.430 | 0.540 | 3.95 | 2820.5 |
| stage/render/prepare_resources | 0.422 | 0.498 | 0.93 | 2767.4 |
| stage/main/pre_update | 0.393 | 0.546 | 2.06 | 2577.4 |
| stage/main/first | 0.365 | 0.481 | 5.76 | 2392.3 |
| stage/render/prepare_flush | 0.323 | 0.529 | 1.39 | 2122.4 |
| controller/feed/ms | 0.322 | 0.401 | 0.90 | 2112.8 |
| stage/render/prepare_bind_groups | 0.313 | 0.503 | 2.08 | 2055.4 |
| stage/render/prepare_batch_phases | 0.275 | 0.419 | 0.83 | 1807.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.273 | 0.436 | 0.92 | 1788.5 |
| stage/main/last | 0.254 | 0.301 | 2.65 | 1667.6 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.239 | 1.090 | 2.21 | 1568.9 |
| bigspace/low_precision/ms | 0.234 | 0.741 | 2.84 | 1535.4 |
| stream/drive/ms | 0.206 | 0.278 | 10.08 | 1348.8 |
| stage/render/create_views | 0.199 | 0.269 | 0.62 | 1308.6 |
| stage/render/extract_commands | 0.194 | 0.228 | 0.59 | 1274.9 |
| bigspace/high_precision/ms | 0.174 | 0.738 | 1.53 | 1140.5 |
| anim/evaluate/ms | 0.161 | 0.293 | 1.58 | 1056.5 |
| stage/render/specialize | 0.158 | 0.254 | 1.84 | 1039.4 |
| anim/evaluate_cpu/ms | 0.154 | 0.197 | 0.61 | 1008.3 |
| stream/tick/ms | 0.152 | 0.217 | 0.49 | 997.8 |
| stage/render/prepare_views | 0.134 | 0.175 | 0.59 | 881.3 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.123 | 0.896 | 1.44 | 806.9 |
| stage/render/queue | 0.100 | 0.173 | 1.41 | 653.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.095 | 0.147 | 0.78 | 622.4 |
| render/acquire/ms | 0.092 | 0.121 | 0.55 | 602.0 |
| stage/render/phase_sort | 0.080 | 0.109 | 0.30 | 523.8 |
| controller/write/ms | 0.074 | 0.103 | 0.32 | 488.8 |
| controller/move/ms | 0.072 | 0.195 | 0.62 | 470.4 |
| terrain/update/ms | 0.059 | 0.289 | 1.77 | 388.5 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.053 | 0.000 | 3.04 | 350.5 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.072 | 0.46 | 302.4 |
| physics/step/ms | 0.043 | 0.104 | 0.57 | 285.5 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.66 | 211.8 |
| pipelines/compile_ms | 0.025 | 0.000 | 77.84 | 161.7 |
| physics/insert/ms | 0.024 | 0.291 | 0.84 | 157.7 |
| pipelines/front_ms | 0.024 | 0.000 | 75.11 | 156.1 |
| stage/render/cleanup | 0.024 | 0.033 | 0.40 | 155.1 |
| terrain/select/ms | 0.022 | 0.256 | 0.80 | 144.4 |
| weather/map/ms | 0.018 | 0.025 | 0.20 | 120.1 |
| far/select/ms | 0.014 | 0.165 | 0.45 | 92.5 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.49 | 88.2 |
| anim/apply/ms | 0.012 | 0.021 | 0.19 | 76.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.55 | 72.8 |
| weather/eval/ms | 0.011 | 0.014 | 0.09 | 70.4 |
| script/update/ms | 0.008 | 0.011 | 0.10 | 50.6 |
| controller/avoid/ms | 0.006 | 0.012 | 0.19 | 40.2 |
| stream/despawn/ms | 0.005 | 0.000 | 1.08 | 35.6 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 33.2 |
| stream/spawn/ms | 0.003 | 0.000 | 1.16 | 22.2 |
| script/ui/ms | 0.003 | 0.004 | 0.04 | 20.0 |
| stream/upload/ms | 0.003 | 0.000 | 1.05 | 18.6 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.61 | 13.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.6 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.73 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.53 | 5.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.5 |
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
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 18 | 30 | 15 |
| controller/movers/count | count | 19 | 22 | 15 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 583 |
| crowd/c0/meshes/count | count | 357 | 646 | 162 |
| crowd/c1/count | count | 146 | 156 | 147 |
| crowd/c1/joints/count | count | 5539 | 5942 | 5581 |
| crowd/c1/meshes/count | count | 1181 | 1272 | 1171 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 216 | 204 |
| crowd/joints_extracted/count | count | 654 | 6961 | 644 |
| crowd/joints_uploaded/count | count | 3607 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.4 k | 47.7 k |
| encode//max | ms | 0.41 | 2.29 | 0.55 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.49 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.55 | 0.01 |
| encode/finish_critical | ms | 0.98 | 3.04 | 0.87 |
| encode/pass:early prepass/max | ms | 0.83 | 2.10 | 0.81 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.84 | 1.85 | 0.87 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.78 | 0.21 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.26 | 0.92 | 0.27 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.83 | 2.21 | 0.97 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.33 | 1.44 | 0.22 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.11 | 3.04 | 0.94 |
| encode/pass:shadow_directional_light/max | ms | 0.82 | 1.81 | 0.85 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.66 | 0.04 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.59 | 29.51 | 8.75 |
| gpu/timed/ms | ms | 7.86 | 9.30 | 8.75 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.4 k | 302.4 k | 302.4 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
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
| phases/opaque3d/items | count | 2480 | 3552 | 2457 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 766 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2480 | 3552 | 2457 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 74 | 420 | 8 |
| phases/shadow/bins | count | 837 | 1558 | 658 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2309 | 3301 | 2014 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 50 | 151 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2679 | 3183 | 2344 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 335 | 336 | 336 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 15 | 275 | 11 |
| preprocess/bind_groups_created | count | 10 | 263 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 136 | 227 | 123 |
| process/cpu_usage | % | 25.48 | 40.27 | 15.87 |
| process/mem_usage | GiB | 5.09 | 5.38 | 5.38 |
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
| shadow_atlas/live_occ_pct | % | 4.79 | 29.49 | 0 |
| shadow_atlas/live_tiles | count | 17 | 59 | 0 |
| shadow_atlas/mover_draws | count | 10 | 37 | 0 |
| shadow_atlas/movers | count | 161 | 462 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 2 | 0 |
| shadow_atlas/render/blits | count | 10 | 37 | 0 |
| shadow_atlas/render/clears | count | 0 | 18 | 0 |
| shadow_atlas/render/lamps | count | 8 | 17 | 0 |
| shadow_atlas/render/live_direct | count | 10 | 37 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 12 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 17 | 56 | 0 |
| shadow_atlas/resizes | count | 32 | 41 | 41 |
| shadow_atlas/static_occ_pct | % | 16.41 | 36.33 | 0 |
| shadow_atlas/static_tiles | count | 42 | 105 | 0 |
| shadow_atlas/views | count | 17 | 56 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 13 | 121 | 12 |
| skin/blocks_skipped/count | count | 146 | 205 | 191 |
| skin/blocks_walked/count | count | 13 | 30 | 13 |
| skin/joint_bytes_written/bytes | bytes | 46.3 k | 811.1 k | 38.9 k |
| skin/joints_written/count | count | 724 | 12.7 k | 608 |
| skin/upload_writes/count | count | 7 | 24 | 9 |
| started_load_count |  | 6441 | 6565 | 6565 |
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
| stream/latency/s | s | 0.04 | 0.07 | 0.06 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.54 | 46.78 | 28.32 |
| system/mem_usage | % | 46.89 | 47.85 | 47.85 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2887 | 3003 | 3003 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 133 | 475 | 66 |
| views/all/bins | count | 2933 | 3556 | 2948 |
| views/all/count | count | 20 | 59 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 8009 | 10.5 k | 7958 |
| views/all/pipelines | count | 97 | 198 | 66 |
| views/all/pipelines_distinct | count | 66 | 75 | 61 |
| views/all/work_items | count | 154.1 k | 230.9 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 33 | 8 |
| views/cascade/bins | count | 714 | 860 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2122 | 2427 | 2014 |
| views/cascade/pipelines | count | 24 | 27 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 61 | 388 | 0 |
| views/lamp_face/bins | count | 98 | 720 | 0 |
| views/lamp_face/count | count | 17 | 56 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 139 | 881 | 0 |
| views/lamp_face/pipelines | count | 25 | 125 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 110 | 25.2 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2010 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5908 | 7524 | 5944 |
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
| anim/bones_written/count | count | 48.33 | 217 | 317.2 k |
| anim/events/count | count | 0.08 | 3 | 532 |
| assets/Image/added | count | 0.09 | 34 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 11 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.44 | 289 | 2874 |
| assets/Mesh/removed | count | 0.74 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.14 | 10 | 7476 |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.23 | 1154 | 1489 |
| encode//count | count | 27.92 | 30 | 183.2 k |
| encode/bind_walk/init_textures | count | 9046 | 23.1 k | 59.37 M |
| encode/bind_walk/init_textures_skipped | count | 52.2 k | 73.3 k | 342.80 M |
| encode/bind_walk/inits | count | 562 | 768 | 3.69 M |
| encode/bind_walk/inits_skipped | count | 171 | 249 | 1.12 M |
| encode/bind_walk/merge_textures | count | 8983 | 23.0 k | 58.96 M |
| encode/bind_walk/merge_textures_skipped | count | 8752 | 18.6 k | 57.44 M |
| encode/bind_walk/merges | count | 104 | 181 | 681.5 k |
| encode/bind_walk/merges_skipped | count | 150 | 224 | 986.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6563 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6563 |
| encode/pass:early prepass/count | count | 1 | 1 | 6563 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6563 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6563 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6563 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.27 | 1 | 1800 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.25 | 1 | 1640 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.05 | 1 | 304 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6563 |
| encode/queue/command_buffers | count | 38.50 | 42 | 252.7 k |
| encode/queue/submits | count | 2.01 | 4 | 13.2 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.44 M | 15.23 G |
| encode/queue/write_buffer/calls | count | 271 | 460 | 1.78 M |
| encode/queue/write_buffer_with/bytes | bytes | 161.3 k | 1.29 M | 1.06 G |
| encode/queue/write_buffer_with/calls | count | 10.30 | 876 | 67.6 k |
| encode/queue/write_texture/bytes | bytes | 2010 | 1.05 M | 13.19 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6563 |
| physics/inserts | count | 0.34 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 181 | 2335 |
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
| stream/deactivate/items | count | 0.03 | 39 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5158 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1131 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 46, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.39 | 2394.7 | main/busy/ms 2365, stage/main/post_update 2322, stage/main/update 32.54, stage/main/pre_update 7.38, stage/main/last 1.02, surface/advance_cpu_ms 0.78, stream/drive/ms 0.61, stage/main/first 0.51, stage/main/fixed_main_loop 0.37, surface/cell_upload_ms 0.28, stream/tick/ms 0.13, bigspace/total/ms 0.12 |
| warm-up | 3.24 | 841.5 | main/busy/ms 2505, stage/main/post_update 2353, render/thread/ms 800, render/busy/ms 662, stage/render/prepare_assets 604, stage/render/prepare_views 135, render/acquire/ms 135, stage/main/update 135, stage/render/create_views 29.99, stage/render/render 18.46, render/graph/ms 14.92, stage/render/camera_driver 13.24 |
| warm-up | 3.34 | 107.2 | render/thread/ms 890, render/busy/ms 698, stage/render/prepare_assets 626, main/busy/ms 160, stage/render/prepare_views 136, render/acquire/ms 135, stage/main/update 108, stage/render/render 79.02, pipelines/compile_ms 61.43, render/present/ms 56.94, stage/main/post_update 41.10, pipelines/driver_ms 37.89 |
| warm-up | 3.50 | 139.2 | pipelines/wait_ms 3851, pipelines/compile_ms 1252, pipelines/front_ms 1164, main/busy/ms 141, pipelines/driver_ms 87.41, render/thread/ms 87.15, render/busy/ms 83.40, stage/main/update 71.94, stage/main/post_update 51.31, encode//sum 45.87, stage/render/prepare_resources 27.58, stage/render/render 16.92 |
| warm-up | 3.58 | 81.7 | pipelines/wait_ms 4144, pipelines/compile_ms 2005, pipelines/front_ms 1898, render/thread/ms 133, render/busy/ms 130, main/busy/ms 118, pipelines/driver_ms 108, stage/main/update 69.82, stage/render/prepare_assets 61.45, encode//sum 48.79, stage/main/post_update 28.96, stage/render/prepare_resources 27.05 |
| warm-up | 3.64 | 38.8 | pipelines/compile_ms 405, pipelines/front_ms 386, render/thread/ms 48.37, render/busy/ms 47.00, main/busy/ms 38.38, stage/render/prepare_assets 30.48, stage/main/post_update 19.55, pipelines/driver_ms 18.74, stage/main/update 10.38, terrain/update/ms 9.79, pipelines/wait_ms 8.69, stage/render/render 8.68 |
| warm-up | 3.69 | 29.6 | pipelines/compile_ms 812, pipelines/front_ms 759, pipelines/wait_ms 308, pipelines/driver_ms 52.93, render/thread/ms 50.70, render/busy/ms 48.90, main/busy/ms 30.49, encode//sum 17.19, stage/render/prepare_resources 17.16, stage/main/post_update 13.89, stage/render/render 12.17, stage/render/prepare_assets 12.13 |
| warm-up | 3.83 | 127.8 | pipelines/compile_ms 1774, pipelines/front_ms 1698, main/busy/ms 87.88, render/thread/ms 76.55, pipelines/driver_ms 76.00, render/busy/ms 73.88, pipelines/wait_ms 67.08, stage/main/update 67.05, stage/render/prepare_assets 47.73, stage/render/render 18.04, render/graph/ms 15.43, stage/main/post_update 14.72 |
| warm-up | 3.86 | 26.1 | pipelines/compile_ms 1998, pipelines/front_ms 1911, pipelines/wait_ms 101, pipelines/driver_ms 86.51, render/thread/ms 79.37, render/busy/ms 77.71, stage/render/prepare_assets 55.87, main/busy/ms 35.03, stage/main/update 21.88, stage/render/render 14.12, render/graph/ms 11.94, stage/render/graph_submit 9.03 |
| warm-up | 3.90 | 40.8 | pipelines/compile_ms 871, pipelines/front_ms 843, pipelines/wait_ms 47.14, render/thread/ms 42.23, render/busy/ms 41.02, pipelines/driver_ms 28.04, stage/render/prepare_assets 27.84, main/busy/ms 19.01, stage/main/update 8.63, stage/render/render 8.50, render/graph/ms 6.96, encode//sum 5.71 |
| warm-up | 3.96 | 31.8 | pipelines/compile_ms 115, pipelines/front_ms 108, render/thread/ms 45.00, render/busy/ms 42.49, main/busy/ms 34.68, stage/main/update 25.46, stage/render/render 20.32, render/graph/ms 17.95, encode//sum 15.93, stage/render/graph_submit 14.56, stage/render/prepare_resources 11.20, encode/submit 7.69 |
| warm-up | 4.00 | 36.8 | render/thread/ms 61.47, render/busy/ms 59.20, main/busy/ms 45.07, pipelines/compile_ms 31.62, pipelines/front_ms 28.52, stage/main/update 22.33, stage/main/post_update 18.08, stage/render/render 16.65, render/graph/ms 14.06, stage/render/prepare_assets 12.50, stage/render/prepare_resources 12.16, stage/render/graph_submit 10.10 |
| warm-up | 4.04 | 41.4 | pipelines/compile_ms 184, pipelines/front_ms 171, render/thread/ms 62.75, render/busy/ms 60.80, main/busy/ms 56.39, stage/render/prepare_assets 25.51, stage/main/post_update 22.73, stage/render/render 14.97, stage/main/pre_update 14.91, stage/main/update 14.10, pipelines/driver_ms 12.84, render/graph/ms 12.36 |
| warm-up | 4.10 | 67.0 | pipelines/compile_ms 1059, pipelines/front_ms 1023, main/busy/ms 140, stage/main/update 107, render/thread/ms 73.09, render/busy/ms 71.41, stage/render/prepare_assets 49.18, pipelines/driver_ms 36.13, stage/main/pre_update 15.74, stage/render/render 12.42, stage/main/post_update 11.24, render/graph/ms 10.31 |
| warm-up | 4.23 | 127.3 | pipelines/compile_ms 1703, pipelines/front_ms 1659, main/busy/ms 143, stage/main/update 127, render/thread/ms 82.02, render/busy/ms 80.39, stage/render/prepare_assets 62.56, pipelines/driver_ms 44.16, stage/render/render 11.56, render/graph/ms 9.50, stage/main/post_update 7.43, stage/render/graph_submit 6.82 |
| warm-up | 4.28 | 43.9 | pipelines/compile_ms 1295, pipelines/front_ms 1256, render/thread/ms 73.37, render/busy/ms 71.78, pipelines/wait_ms 63.68, main/busy/ms 61.42, stage/render/prepare_assets 46.52, pipelines/driver_ms 39.47, stage/main/update 36.37, stage/render/render 11.92, render/graph/ms 10.04, stage/main/post_update 9.15 |
| warm-up | 4.30 | 26.0 | pipelines/compile_ms 504, pipelines/front_ms 484, main/busy/ms 67.10, pipelines/wait_ms 62.49, render/thread/ms 55.76, render/busy/ms 54.03, stage/main/update 26.96, stage/main/post_update 25.23, stage/render/prepare_assets 21.54, pipelines/driver_ms 20.10, stage/render/render 14.65, render/graph/ms 12.67 |
| warm-up | 4.38 | 77.2 | pipelines/compile_ms 143, pipelines/front_ms 137, pipelines/wait_ms 100, main/busy/ms 80.86, render/thread/ms 72.69, render/busy/ms 70.86, stage/main/post_update 36.84, stage/main/update 22.65, stage/render/prepare_assets 20.25, stage/render/prepare_write_phase_buffers 15.73, stage/render/render 14.68, render/graph/ms 12.66 |
| warm-up | 4.45 | 73.4 | pipelines/compile_ms 1316, pipelines/front_ms 1275, pipelines/wait_ms 120, render/thread/ms 117, render/busy/ms 116, main/busy/ms 98.86, stage/render/prepare_write_phase_buffers 61.95, stage/main/update 53.16, pipelines/driver_ms 41.21, stage/render/prepare_assets 23.87, stage/main/post_update 22.95, stage/render/render 12.21 |
| warm-up | 4.54 | 84.8 | pipelines/compile_ms 2768, pipelines/front_ms 2682, pipelines/wait_ms 731, render/thread/ms 128, render/busy/ms 126, main/busy/ms 114, pipelines/driver_ms 86.28, stage/main/update 72.68, stage/render/prepare_write_phase_buffers 50.76, stage/render/prepare_assets 36.94, stage/render/render 25.15, render/graph/ms 22.43 |
| warm-up | 4.58 | 40.3 | pipelines/compile_ms 2102, pipelines/front_ms 2030, pipelines/wait_ms 889, render/thread/ms 92.50, render/busy/ms 90.56, main/busy/ms 89.99, pipelines/driver_ms 71.98, stage/render/prepare_assets 42.72, stage/main/update 42.63, stage/render/render 28.39, render/graph/ms 25.42, stage/main/post_update 17.39 |
| warm-up | 4.65 | 74.4 | pipelines/compile_ms 762, pipelines/front_ms 735, pipelines/wait_ms 293, render/thread/ms 77.31, main/busy/ms 75.97, render/busy/ms 75.72, stage/main/update 49.29, stage/render/prepare_assets 46.23, pipelines/driver_ms 27.54, stage/render/render 16.25, render/graph/ms 14.15, stage/main/post_update 10.24 |
| warm-up | 4.69 | 42.2 | pipelines/compile_ms 596, pipelines/front_ms 584, pipelines/wait_ms 308, render/thread/ms 80.47, render/busy/ms 78.97, main/busy/ms 73.61, stage/main/update 50.28, stage/render/prepare_assets 43.48, stage/render/render 14.92, render/graph/ms 12.94, pipelines/driver_ms 12.56, stage/main/post_update 10.65 |
| warm-up | 4.73 | 39.2 | pipelines/compile_ms 856, pipelines/front_ms 835, pipelines/wait_ms 207, main/busy/ms 82.88, render/thread/ms 66.23, render/busy/ms 64.52, stage/main/update 56.32, stream/drive/ms 33.31, stage/render/prepare_assets 23.04, pipelines/driver_ms 20.42, stage/render/render 17.50, render/graph/ms 15.06 |
| warm-up | 4.78 | 42.5 | pipelines/compile_ms 1016, pipelines/front_ms 983, main/busy/ms 104, stage/main/update 82.59, stream/drive/ms 64.41, render/thread/ms 48.94, render/busy/ms 47.42, pipelines/driver_ms 33.41, stage/render/prepare_assets 17.13, pipelines/wait_ms 14.66, stage/render/render 14.42, stage/main/post_update 12.70 |
| warm-up | 4.86 | 80.3 | pipelines/compile_ms 539, pipelines/front_ms 518, main/busy/ms 83.41, stage/main/update 66.55, render/thread/ms 55.58, render/busy/ms 54.30, stream/drive/ms 36.96, stage/render/prepare_assets 27.17, pipelines/driver_ms 20.44, stage/render/render 10.96, stage/main/post_update 9.54, render/graph/ms 9.03 |
| warm-up | 4.90 | 41.3 | pipelines/compile_ms 755, pipelines/front_ms 734, render/thread/ms 64.63, render/busy/ms 63.34, main/busy/ms 45.75, stage/render/prepare_assets 36.01, stage/main/update 28.08, pipelines/driver_ms 21.43, stage/render/render 10.82, stage/main/post_update 9.15, render/graph/ms 8.75, encode//sum 6.58 |
| warm-up | 4.94 | 25.3 | pipelines/compile_ms 746, pipelines/front_ms 724, render/thread/ms 42.97, render/busy/ms 41.44, main/busy/ms 35.20, pipelines/driver_ms 21.91, stage/main/update 18.39, stage/render/render 16.78, render/graph/ms 13.99, stage/main/post_update 11.19, stage/render/graph_submit 9.51, encode//sum 8.21 |
| warm-up | 5.11 | 26.0 | pipelines/compile_ms 173, pipelines/front_ms 169, render/thread/ms 33.69, render/busy/ms 32.42, main/busy/ms 26.50, stage/render/render 12.23, render/graph/ms 10.46, stage/main/post_update 10.23, stage/main/update 9.78, stage/render/prepare_assets 7.63, stage/render/graph_submit 7.44, encode//sum 7.22 |
| warm-up | 5.40 | 28.6 | pipelines/compile_ms 104, pipelines/front_ms 98.56, main/busy/ms 43.41, render/thread/ms 32.77, render/busy/ms 30.91, stage/main/update 26.23, stream/drive/ms 17.91, stage/render/render 16.15, render/graph/ms 13.75, stage/render/graph_submit 10.17, stage/main/post_update 10.17, encode//sum 7.06 |
| warm-up | 5.67 | 32.4 | main/busy/ms 38.49, render/thread/ms 27.38, stage/main/update 26.29, render/busy/ms 26.23, stream/drive/ms 18.15, stage/render/render 10.83, render/graph/ms 9.04, stage/main/post_update 7.76, encode//sum 7.74, stage/render/prepare_assets 6.54, stage/render/graph_submit 5.52, stage/render/camera_driver 3.32 |
| warm-up | 5.78 | 25.8 | render/thread/ms 36.48, render/busy/ms 35.20, main/busy/ms 29.14, stage/main/update 13.63, stage/render/render 12.75, stage/render/prepare_assets 11.16, render/graph/ms 10.75, stage/main/post_update 10.63, encode//sum 9.42, stage/render/graph_submit 7.20, encode/submit 4.31, stage/render/prepare_bind_groups 3.43 |
| warm-up | 5.83 | 29.9 | render/thread/ms 35.24, render/busy/ms 34.35, main/busy/ms 23.02, stage/render/prepare_assets 17.12, stage/main/update 11.18, stage/render/render 7.51, stage/main/post_update 6.57, encode//sum 6.51, render/graph/ms 6.27, stage/render/graph_submit 3.92, stream/drive/ms 3.14, stage/render/prepare_meshes 2.77 |
| warm-up | 5.93 | 27.2 | render/thread/ms 34.06, render/busy/ms 32.87, main/busy/ms 32.01, stage/main/update 20.32, stage/render/prepare_assets 10.42, stage/render/render 8.97, encode//sum 7.97, render/graph/ms 7.42, stage/main/post_update 6.10, stage/render/graph_submit 4.87, stage/render/prepare_meshes 4.40, stream/drive/ms 3.44 |
| warm-up | 6.06 | 30.6 | main/busy/ms 41.99, render/thread/ms 41.24, render/busy/ms 39.79, stage/main/update 23.51, stage/render/render 14.71, render/graph/ms 12.59, stage/main/post_update 11.59, stage/render/prepare_assets 10.30, encode//sum 9.58, stage/render/graph_submit 8.96, encode/submit 5.92, stage/render/prepare_meshes 4.97 |
| warm-up | 6.09 | 35.2 | render/thread/ms 48.93, render/busy/ms 47.90, main/busy/ms 38.54, stage/main/update 18.78, stage/render/prepare_assets 15.83, stage/render/render 13.23, stage/main/post_update 11.92, render/graph/ms 11.36, stage/render/graph_submit 8.17, encode//sum 7.62, stage/render/prepare_meshes 5.83, encode/finish_wall 5.17 |
| warm-up | 6.13 | 35.4 | render/thread/ms 62.58, render/busy/ms 61.09, main/busy/ms 42.15, stage/render/render 24.64, render/graph/ms 22.20, stage/main/update 19.39, stage/render/graph_submit 17.44, stage/main/post_update 14.47, stage/render/prepare_assets 13.05, encode/finish_wall 10.13, encode//sum 9.81, encode/pass:point_shadow_atlas_live_indirect/sum 9.55 |
| warm-up | 6.16 | 32.4 | pipelines/compile_ms 144, pipelines/front_ms 141, render/thread/ms 57.96, render/busy/ms 56.57, main/busy/ms 39.19, stage/render/render 22.82, render/graph/ms 20.73, stage/main/update 19.29, stage/render/graph_submit 16.27, stage/main/post_update 13.52, stage/render/prepare_assets 12.57, encode//sum 9.61 |
| warm-up | 6.20 | 34.6 | pipelines/compile_ms 144, pipelines/front_ms 141, render/thread/ms 49.64, render/busy/ms 48.61, main/busy/ms 36.02, stage/render/prepare_assets 19.31, stage/main/update 18.95, stage/render/render 14.65, render/graph/ms 12.97, stage/main/post_update 10.98, stage/render/graph_submit 9.88, encode//sum 7.34 |
| warm-up | 6.36 | 45.5 | main/busy/ms 58.32, render/thread/ms 51.23, render/busy/ms 49.69, stage/main/update 41.68, stage/render/render 22.51, render/graph/ms 20.35, stage/render/graph_submit 16.37, stream/drive/ms 14.35, stage/main/post_update 10.85, stage/render/prepare_assets 9.85, encode/finish_wall 9.20, encode//sum 8.66 |
| warm-up | 6.39 | 34.2 | pipelines/compile_ms 144, pipelines/front_ms 139, render/thread/ms 55.01, render/busy/ms 53.83, main/busy/ms 44.99, stage/main/update 27.46, stage/render/prepare_assets 19.11, stage/render/render 17.20, render/graph/ms 15.25, stage/render/graph_submit 11.19, stage/main/post_update 10.33, encode//sum 8.59 |
| warm-up | 6.42 | 25.2 | pipelines/compile_ms 144, pipelines/front_ms 139, render/thread/ms 41.96, render/busy/ms 41.07, main/busy/ms 36.97, stage/main/update 19.93, stage/render/prepare_assets 19.19, stage/render/render 12.21, render/graph/ms 10.84, stage/main/post_update 9.10, stream/drive/ms 8.07, stage/render/graph_submit 7.96 |
| measure | 18.35 | 31.2 | main/busy/ms 39.48, stage/main/update 26.15, render/thread/ms 21.65, render/busy/ms 20.69, stage/render/render 10.23, stage/main/post_update 9.26, render/graph/ms 8.93, stage/render/graph_submit 5.94, encode//sum 5.73, encode/submit 3.82, stage/render/prepare_assets 3.77, stage/render/camera_driver 2.86 |
| measure | 23.02 | 25.3 | render/thread/ms 29.63, render/busy/ms 28.30, main/busy/ms 27.01, stage/render/render 14.34, render/graph/ms 12.42, stage/main/update 12.33, stage/main/post_update 10.22, encode//sum 9.13, stage/render/graph_submit 8.28, stage/render/prepare_assets 8.19, encode/submit 5.08, stage/render/camera_driver 3.95 |
| measure | 59.07 | 25.6 | render/thread/ms 35.93, render/busy/ms 34.43, main/busy/ms 31.05, stage/render/render 17.65, render/graph/ms 15.22, stage/main/post_update 15.22, encode//sum 14.21, stage/render/graph_submit 9.69, stage/main/update 8.57, stage/render/prepare_assets 6.74, stage/render/camera_driver 5.38, encode/submit 5.33 |
| measure | 59.10 | 27.0 | render/thread/ms 37.80, render/busy/ms 36.77, main/busy/ms 27.44, stage/render/prepare_assets 15.28, stage/main/post_update 12.23, stage/render/render 12.00, render/graph/ms 10.19, encode//sum 9.92, stage/render/graph_submit 5.86, stage/main/update 5.66, stage/render/camera_driver 4.19, stage/main/fixed_main_loop 3.93 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.soft_pcf_radius=0 --set shadow.lamp.atlas.max_lamps=16 --resolution 2560x1440 --bench-out target/perf\shadow_review\balmora_walk_night@2560x1440_arm=pcf0lamps16-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 8%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **84%** (min 55%) · 2623 MHz (min 2610) · memory 8101 MHz · 84.2 W · 79 °C (max 84) · VRAM 5517 MiB · throttled 100% of samples

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
