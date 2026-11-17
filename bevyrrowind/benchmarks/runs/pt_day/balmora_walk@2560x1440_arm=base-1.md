# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5776 frames over 65.6 s · **median 10.99 ms (91 fps)** · mean 11.37 · p95 14.06 · p99 17.16 (1% low 58 fps) · worst 50.16 ms · 76 spikes over 1.5× median · 56.7 stutters/min (work 52.1/min)
- warm-up: 324 frames over 9.3 s, worst 2614.8 ms
- hitches (> 27.5 ms): **31** measured, 48 in warm-up
- timed GPU passes: 6.96 ms of a 11.37 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.99 ms, limit 11 ms
- **FAIL** frame_ms: 62 stutters (> 1.5x moving median of 60) in 1.09 min = 56.67/min, limit 1/min
- **FAIL** worst frame 50.16 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.954 | 2.035 | 2.068 | 0.96 | 0.175 |
| main_opaque_pass_3d | 1.574 | 1.640 | 2.051 | 0.96 | 0.078 |
| early prepass | 0.482 | 0.502 | 0.737 | 0.96 | 0.088 |
| bloom | 0.302 | 0.314 | 0.321 | 0.96 | 0.062 |
| atmosphere_luts | 0.293 | 0.305 | 0.367 | 0.96 | 0.003 |
| water_velocity | 0.267 | 0.278 | 0.347 | 0.96 | 0.002 |
| water_sspr | 0.194 | 0.202 | 0.203 | 0.96 | 0.002 |
| ssao | 0.181 | 0.189 | 0.193 | 0.96 | 0.007 |
| auxiliary_views | 0.160 | 0.167 | 0.586 | 0.96 | 0.176 |
| atmos_froxel_inject | 0.140 | 0.146 | 0.148 | 0.96 | 0.001 |
| clouds_trace | 0.135 | 0.140 | 0.230 | 0.96 | 0.001 |
| uniform_allocation | 0.128 | 0.133 | 0.185 | 0.96 | 0.029 |
| water_fft | 0.127 | 0.133 | 0.140 | 0.96 | 0.003 |
| upscaling | 0.112 | 0.117 | 0.124 | 0.96 | 0.004 |
| clouds_composite | 0.101 | 0.106 | 0.108 | 0.96 | 0.002 |
| bin_unpacking | 0.101 | 0.105 | 0.141 | 0.96 | 0.021 |
| main_transmissive_pass_3d | 0.083 | 0.086 | 0.179 | 0.96 | 0.009 |
| tonemapping | 0.082 | 0.085 | 0.088 | 0.96 | 0.004 |
| clustering | 0.061 | 0.064 | 0.067 | 0.96 | 0.018 |
| render_sky | 0.050 | 0.052 | 0.053 | 0.96 | 0.002 |
| clouds_resolve | 0.046 | 0.048 | 0.049 | 0.96 | 0.001 |
| lightprobe_irradiance_map | 0.044 | 0.414 | 0.417 | 0.11 | 0.001 |
| surface_exposure | 0.042 | 0.044 | 0.045 | 0.96 | 0.001 |
| surface_trail | 0.039 | 0.056 | 0.057 | 0.70 | 0.002 |
| surface_exposure_far | 0.036 | 0.042 | 0.045 | 0.86 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.96 | 0.003 |
| early_mesh_preprocessing | 0.029 | 0.031 | 0.047 | 0.96 | 0.006 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.031 | 0.96 | 0.001 |
| water_snapshot | 0.024 | 0.025 | 0.028 | 0.96 | 0.002 |
| lightprobe_radiance_map | 0.023 | 0.221 | 0.233 | 0.11 | 0.005 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.96 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.96 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.024 | 0.96 | 0.126 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.96 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.96 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.96 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.96 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.954 | 12.387 | 39.65 | 57492.2 |
| render/busy/ms | 9.419 | 11.862 | 39.14 | 54402.1 |
| main/busy/ms | 9.322 | 11.082 | 25.70 | 53846.5 |
| stage/main/post_update | 5.836 | 7.275 | 11.54 | 33711.2 |
| stage/render/render | 5.808 | 7.335 | 14.96 | 33547.1 |
| render/graph/ms | 5.077 | 6.569 | 13.91 | 29322.6 |
| encode//sum | 4.668 | 6.329 | 27.11 | 26964.1 |
| stage/render/graph_submit | 3.823 | 4.962 | 11.17 | 22080.8 |
| encode/submit | 2.399 | 3.278 | 6.85 | 13858.0 |
| stage/main/update | 2.186 | 2.640 | 18.67 | 12628.5 |
| encode/finish_wall | 1.479 | 2.132 | 4.44 | 8543.4 |
| stage/render/camera_driver | 1.200 | 1.699 | 3.67 | 6928.3 |
| encode/pass:shadow_point_light/sum | 1.003 | 3.411 | 16.27 | 5790.5 |
| encode/pass:shadow_directional_light/sum | 0.894 | 1.637 | 2.56 | 5165.1 |
| stage/render/prepare_write_phase_buffers | 0.829 | 1.184 | 2.54 | 4785.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.791 | 1.142 | 1.94 | 4571.1 |
| encode/pass:early prepass/sum | 0.766 | 1.119 | 1.85 | 4424.6 |
| stage/render/prepare_assets | 0.597 | 0.768 | 10.92 | 3448.4 |
| bigspace/total/ms | 0.519 | 1.261 | 2.86 | 2996.7 |
| stage/render/prepare_resources | 0.473 | 0.567 | 1.12 | 2734.0 |
| stage/render/prepare_meshes | 0.463 | 0.586 | 7.89 | 2674.1 |
| render/present/ms | 0.442 | 0.562 | 2.21 | 2552.2 |
| stage/main/first | 0.406 | 0.530 | 6.70 | 2342.7 |
| stage/main/pre_update | 0.369 | 0.519 | 1.34 | 2130.4 |
| stage/render/specialize | 0.352 | 0.449 | 11.77 | 2031.0 |
| stage/render/queue | 0.320 | 0.454 | 8.18 | 1848.6 |
| terrain/update/ms | 0.319 | 0.424 | 2.19 | 1841.8 |
| stage/render/prepare_bind_groups | 0.298 | 0.544 | 2.69 | 1721.3 |
| terrain/select/ms | 0.285 | 0.384 | 1.04 | 1645.3 |
| controller/feed/ms | 0.270 | 0.335 | 1.51 | 1561.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.261 | 0.384 | 1.00 | 1505.0 |
| stream/drive/ms | 0.253 | 0.342 | 16.82 | 1462.3 |
| stage/main/fixed_main_loop | 0.230 | 0.382 | 1.11 | 1325.9 |
| bigspace/low_precision/ms | 0.218 | 0.701 | 2.29 | 1258.1 |
| stage/render/create_views | 0.203 | 0.264 | 1.35 | 1172.7 |
| far/select/ms | 0.199 | 0.276 | 0.62 | 1148.8 |
| stream/tick/ms | 0.195 | 0.269 | 0.67 | 1126.8 |
| stage/render/prepare_batch_phases | 0.193 | 0.275 | 0.55 | 1115.4 |
| stage/render/extract_commands | 0.180 | 0.210 | 0.41 | 1038.5 |
| stage/render/prepare_flush | 0.166 | 0.321 | 0.85 | 957.1 |
| anim/evaluate/ms | 0.157 | 0.245 | 1.78 | 908.0 |
| anim/evaluate_cpu/ms | 0.155 | 0.200 | 0.84 | 894.1 |
| stage/render/prepare_views | 0.137 | 0.171 | 0.47 | 792.9 |
| bigspace/high_precision/ms | 0.137 | 0.499 | 1.49 | 788.9 |
| controller/move/ms | 0.133 | 0.303 | 0.73 | 767.4 |
| stage/main/last | 0.116 | 0.161 | 1.59 | 667.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.114 | 0.189 | 0.65 | 657.2 |
| render/acquire/ms | 0.093 | 0.120 | 0.42 | 537.9 |
| stage/render/prepare_collect_phase_buffers | 0.064 | 0.136 | 0.53 | 371.8 |
| pipelines/compile_ms | 0.051 | 0.000 | 123.51 | 295.8 |
| physics/step/ms | 0.051 | 0.091 | 0.59 | 292.1 |
| pipelines/front_ms | 0.049 | 0.000 | 119.03 | 283.2 |
| physics/insert/ms | 0.036 | 0.390 | 1.07 | 210.5 |
| stage/render/cleanup | 0.032 | 0.097 | 0.74 | 183.3 |
| encode/resolve_diagnostics/sum | 0.032 | 0.042 | 0.52 | 182.7 |
| stage/render/phase_sort | 0.031 | 0.053 | 0.32 | 178.9 |
| weather/map/ms | 0.025 | 0.034 | 0.09 | 146.0 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.58 | 89.3 |
| anim/apply/ms | 0.012 | 0.022 | 0.37 | 68.9 |
| encode/camera_driver_span_end/sum | 0.012 | 0.020 | 0.48 | 67.3 |
| controller/avoid/ms | 0.010 | 0.014 | 0.06 | 60.5 |
| weather/eval/ms | 0.010 | 0.014 | 0.25 | 60.1 |
| script/update/ms | 0.008 | 0.009 | 0.22 | 45.2 |
| script/input/ms | 0.006 | 0.007 | 0.23 | 33.6 |
| stream/despawn/ms | 0.005 | 0.000 | 1.00 | 30.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 29.1 |
| script/ui/ms | 0.004 | 0.005 | 0.04 | 23.7 |
| controller/write/ms | 0.003 | 0.005 | 0.16 | 19.8 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.67 | 14.6 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.48 | 12.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.16 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 26 | 42 | 12 |
| crowd/c0/joints/count | count | 1301 | 2218 | 535 |
| crowd/c0/meshes/count | count | 353 | 633 | 144 |
| crowd/c1/count | count | 147 | 157 | 147 |
| crowd/c1/joints/count | count | 5573 | 6037 | 5597 |
| crowd/c1/meshes/count | count | 1195 | 1311 | 1181 |
| crowd/c2/count | count | 386 | 419 | 419 |
| crowd/c2/joints/count | count | 6398 | 6968 | 6968 |
| crowd/c2/meshes/count | count | 1820 | 1996 | 1996 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4614 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.46 | 1.37 | 0.58 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.58 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.48 | 0.02 |
| encode/finish_critical | ms | 1.01 | 2.56 | 2.06 |
| encode/pass:early prepass/max | ms | 0.80 | 1.85 | 0.80 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.81 | 1.94 | 0.86 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.65 | 0.15 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 1.00 | 0.21 |
| encode/pass:shadow_directional_light/max | ms | 0.84 | 2.56 | 2.06 |
| encode/pass:shadow_point_light/max | ms | 0.81 | 1.46 | 1.13 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.52 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.58 | 39.14 | 9.48 |
| gpu/timed/ms | ms | 7.96 | 8.96 | 8.51 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1292 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3406 | 5128 | 3189 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1292 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3406 | 5128 | 3189 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 21 | 205 | 29 |
| phases/shadow/bins | count | 5045 | 8841 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6376 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 205 | 257 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 188 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 567 | 58 |
| preprocess/bind_groups_created | count | 13 | 567 | 58 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 217 | 123 |
| process/cpu_usage | % | 27.01 | 42.38 | 23.56 |
| process/mem_usage | GiB | 5.49 | 5.66 | 5.66 |
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
| skin/blocks_changed/count | count | 13 | 126 | 9 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 370.2 k | 809.2 k | 487.7 k |
| skin/joints_written/count | count | 5784 | 12.6 k | 7620 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4921 | 5014 | 5014 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.87 | 48.56 | 29.20 |
| system/mem_usage | % | 54.41 | 55.05 | 54.98 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 72 | 254 | 79 |
| views/all/bins | count | 7612 | 11.7 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 251 | 304 | 75 |
| views/all/pipelines_distinct | count | 71 | 74 | 61 |
| views/all/work_items | count | 10.7 k | 34.5 k | 14.3 k |
| views/cascade/batch_sets | count | 9 | 49 | 29 |
| views/cascade/bins | count | 2788 | 6161 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8907 | 23.5 k | 6376 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 414 | 21.9 k | 6376 |
| views/lamp_face/batch_sets | count | 0 | 189 | 0 |
| views/lamp_face/bins | count | 2199 | 2837 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2459 | 3136 | 0 |
| views/lamp_face/pipelines | count | 169 | 206 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2887 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2884 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8429 | 13.0 k | 7612 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8592 | 13.2 k | 7920 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 56.14 | 252 | 324.3 k |
| anim/events/count | count | 0.09 | 3 | 526 |
| assets/Image/added | count | 0.10 | 34 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.70 | 152 | 4061 |
| assets/Mesh/removed | count | 0.84 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.75 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.81 | 1309 | 4675 |
| encode//count | count | 34.41 | 219 | 198.8 k |
| encode/bind_walk/init_textures | count | 13.1 k | 114.7 k | 75.57 M |
| encode/bind_walk/init_textures_skipped | count | 59.3 k | 146.3 k | 342.42 M |
| encode/bind_walk/inits | count | 464 | 1124 | 2.68 M |
| encode/bind_walk/inits_skipped | count | 103 | 157 | 597.5 k |
| encode/bind_walk/merge_textures | count | 13.0 k | 114.6 k | 75.15 M |
| encode/bind_walk/merge_textures_skipped | count | 5366 | 6012 | 30.99 M |
| encode/bind_walk/merges | count | 100 | 333 | 578.5 k |
| encode/bind_walk/merges_skipped | count | 81.79 | 130 | 472.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5776 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5776 |
| encode/pass:early prepass/count | count | 1 | 1 | 5776 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5776 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5776 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5776 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5776 |
| encode/pass:shadow_point_light/count | count | 1.61 | 48 | 9293 |
| encode/queue/command_buffers | count | 46.04 | 277 | 265.9 k |
| encode/queue/submits | count | 2.02 | 4 | 11.7 k |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 4.88 M | 10.90 G |
| encode/queue/write_buffer/calls | count | 264 | 848 | 1.53 M |
| encode/queue/write_buffer_with/bytes | bytes | 178.1 k | 3.52 M | 1.03 G |
| encode/queue/write_buffer_with/calls | count | 11.09 | 465 | 64.1 k |
| encode/queue/write_texture/bytes | bytes | 12.3 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5776 |
| physics/inserts | count | 0.39 | 106 | 2265 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.68 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 24 | 378 |
| stream/upload/bytes | bytes | 5861 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.01 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 55 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1285 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.5 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.61 | 2614.8 | main/busy/ms 2582, stage/main/post_update 2476, stage/main/update 95.64, surface/advance_cpu_ms 65.59, stage/main/pre_update 6.53, stage/main/last 1.18, surface/cell_upload_ms 0.76, stage/main/first 0.64, stream/drive/ms 0.44, stage/main/fixed_main_loop 0.35, weather/map/ms 0.17, stream/tick/ms 0.09 |
| warm-up | 3.49 | 872.8 | main/busy/ms 2760, stage/main/post_update 2501, render/thread/ms 836, render/busy/ms 643, stage/render/prepare_assets 592, stage/main/update 242, stage/render/prepare_views 188, render/acquire/ms 188, surface/advance_cpu_ms 65.59, stage/render/create_views 29.05, stage/render/render 15.49, stage/main/pre_update 8.88 |
| warm-up | 3.60 | 110.4 | render/thread/ms 930, render/busy/ms 686, stage/render/prepare_assets 622, main/busy/ms 195, stage/render/prepare_views 188, render/acquire/ms 188, stage/main/update 150, pipelines/compile_ms 80.52, stage/render/render 71.58, render/present/ms 56.35, pipelines/driver_ms 43.17, pipelines/front_ms 37.35 |
| warm-up | 3.67 | 49.0 | pipelines/compile_ms 391, pipelines/front_ms 333, pipelines/wait_ms 254, pipelines/driver_ms 58.42, render/thread/ms 48.90, main/busy/ms 47.07, render/busy/ms 46.05, stage/main/post_update 23.85, stage/render/prepare_assets 19.50, stage/render/render 17.81, render/graph/ms 14.74, terrain/update/ms 11.84 |
| warm-up | 3.73 | 62.3 | pipelines/wait_ms 2969, pipelines/compile_ms 715, pipelines/front_ms 683, render/thread/ms 78.31, render/busy/ms 62.77, main/busy/ms 53.47, pipelines/driver_ms 32.15, stage/render/prepare_resources 26.05, stage/render/prepare_assets 22.07, stage/main/post_update 21.53, terrain/update/ms 15.20, stage/render/prepare_views 13.87 |
| warm-up | 3.82 | 83.7 | pipelines/wait_ms 5730, pipelines/compile_ms 1662, pipelines/front_ms 1573, render/thread/ms 116, main/busy/ms 107, render/busy/ms 101, pipelines/driver_ms 89.38, stage/main/update 61.40, stage/render/prepare_resources 43.28, stage/render/prepare_write_phase_buffers 20.81, stage/render/prepare_assets 20.22, stage/main/post_update 18.79 |
| warm-up | 3.88 | 65.2 | pipelines/wait_ms 3037, pipelines/compile_ms 1636, pipelines/front_ms 1556, render/thread/ms 121, render/busy/ms 119, main/busy/ms 104, pipelines/driver_ms 80.11, stage/main/update 64.00, stage/render/prepare_assets 58.31, stage/main/post_update 25.59, stage/render/prepare_write_phase_buffers 20.91, stage/render/prepare_resources 19.65 |
| warm-up | 3.92 | 36.8 | pipelines/compile_ms 1483, pipelines/front_ms 1424, render/thread/ms 88.72, render/busy/ms 87.05, pipelines/wait_ms 68.79, stage/render/prepare_assets 67.58, pipelines/driver_ms 59.06, main/busy/ms 43.41, stage/main/post_update 22.05, stage/render/render 12.55, terrain/update/ms 10.50, render/graph/ms 10.35 |
| warm-up | 3.95 | 30.3 | pipelines/compile_ms 1037, pipelines/front_ms 987, render/thread/ms 63.35, render/busy/ms 61.51, pipelines/driver_ms 50.53, main/busy/ms 42.47, stage/render/prepare_assets 40.29, pipelines/wait_ms 21.44, stage/main/post_update 20.21, stage/main/update 14.65, stage/render/render 10.62, terrain/update/ms 9.23 |
| warm-up | 4.10 | 87.7 | pipelines/compile_ms 609, pipelines/front_ms 571, main/busy/ms 61.31, render/thread/ms 56.80, render/busy/ms 52.78, stage/main/update 48.24, stream/drive/ms 43.91, pipelines/driver_ms 37.89, stage/render/prepare_assets 32.35, stage/render/render 15.89, render/graph/ms 11.49, stage/main/post_update 8.12 |
| warm-up | 4.19 | 72.4 | pipelines/compile_ms 1104, pipelines/front_ms 1064, render/thread/ms 56.14, render/busy/ms 54.58, stage/render/prepare_assets 40.84, pipelines/driver_ms 39.42, main/busy/ms 33.15, stage/main/update 16.98, stream/drive/ms 11.20, stage/main/post_update 9.66, stage/render/render 8.84, render/graph/ms 6.85 |
| warm-up | 4.28 | 64.2 | pipelines/compile_ms 292, pipelines/front_ms 281, main/busy/ms 60.18, render/thread/ms 55.52, render/busy/ms 53.41, stage/main/update 33.92, stage/render/prepare_assets 33.57, stream/drive/ms 18.99, stage/main/post_update 17.75, stage/render/render 12.63, pipelines/driver_ms 10.62, render/graph/ms 9.95 |
| warm-up | 4.34 | 60.1 | pipelines/compile_ms 484, pipelines/front_ms 468, render/thread/ms 81.92, render/busy/ms 79.82, main/busy/ms 73.61, stage/render/prepare_assets 50.49, stage/main/update 40.97, stage/main/post_update 18.42, pipelines/driver_ms 16.67, stage/render/render 14.76, render/graph/ms 12.07, stage/render/graph_submit 8.15 |
| warm-up | 4.43 | 91.3 | pipelines/compile_ms 1041, pipelines/front_ms 1008, render/thread/ms 107, render/busy/ms 105, pipelines/wait_ms 84.21, stage/render/prepare_assets 69.79, main/busy/ms 67.04, stage/main/update 33.11, pipelines/driver_ms 33.08, stage/main/post_update 19.05, stage/render/render 16.54, render/graph/ms 14.17 |
| warm-up | 4.50 | 68.9 | pipelines/compile_ms 1323, pipelines/front_ms 1285, pipelines/wait_ms 249, render/thread/ms 125, render/busy/ms 123, stage/render/prepare_assets 50.72, main/busy/ms 45.66, encode//sum 43.09, pipelines/driver_ms 38.11, stage/render/render 23.59, stage/main/update 21.83, render/graph/ms 20.93 |
| warm-up | 4.55 | 52.8 | pipelines/compile_ms 1907, pipelines/front_ms 1851, pipelines/wait_ms 573, render/thread/ms 108, render/busy/ms 106, pipelines/driver_ms 56.19, main/busy/ms 52.23, encode//sum 45.72, stage/main/update 29.73, stage/render/prepare_assets 27.03, stage/render/render 24.00, render/graph/ms 21.31 |
| warm-up | 4.63 | 73.2 | pipelines/compile_ms 1476, pipelines/front_ms 1431, pipelines/wait_ms 609, render/thread/ms 111, render/busy/ms 108, encode//sum 56.39, pipelines/driver_ms 45.78, stage/render/render 39.71, render/graph/ms 35.70, main/busy/ms 35.37, stage/render/prepare_assets 24.67, stage/render/graph_submit 19.49 |
| warm-up | 4.71 | 79.7 | pipelines/compile_ms 1459, pipelines/front_ms 1414, pipelines/wait_ms 340, render/thread/ms 146, render/busy/ms 142, encode//sum 51.74, pipelines/driver_ms 45.43, stage/render/prepare_resources 44.26, stage/render/render 37.19, main/busy/ms 34.10, render/graph/ms 33.33, stage/render/graph_submit 19.05 |
| warm-up | 4.74 | 33.4 | pipelines/compile_ms 1627, pipelines/front_ms 1572, pipelines/wait_ms 185, render/thread/ms 102, render/busy/ms 101, pipelines/driver_ms 54.75, main/busy/ms 43.45, stage/render/prepare_resources 35.18, stage/render/prepare_assets 23.60, stage/main/update 22.22, stage/main/post_update 13.23, stage/render/prepare_collect_phase_buffers 12.54 |
| warm-up | 4.81 | 73.0 | pipelines/compile_ms 1812, pipelines/front_ms 1763, pipelines/wait_ms 258, main/busy/ms 78.87, render/thread/ms 71.32, render/busy/ms 69.58, stage/main/update 58.86, pipelines/driver_ms 48.41, stage/render/prepare_assets 46.97, stream/drive/ms 45.33, stage/render/render 13.82, encode//sum 11.54 |
| warm-up | 4.85 | 38.6 | pipelines/compile_ms 1777, pipelines/front_ms 1733, pipelines/wait_ms 277, main/busy/ms 100, stage/main/update 76.52, render/thread/ms 70.04, render/busy/ms 68.08, stream/drive/ms 44.54, pipelines/driver_ms 44.17, stage/render/prepare_assets 37.12, stage/render/render 22.12, render/graph/ms 19.21 |
| warm-up | 4.93 | 82.8 | pipelines/compile_ms 678, pipelines/front_ms 657, main/busy/ms 127, stage/main/update 106, render/thread/ms 70.07, pipelines/wait_ms 68.59, render/busy/ms 68.12, stream/drive/ms 53.38, stage/render/render 30.16, stage/render/prepare_assets 28.19, render/graph/ms 27.47, stage/render/graph_submit 21.58 |
| warm-up | 5.03 | 93.1 | pipelines/compile_ms 1124, pipelines/front_ms 1093, main/busy/ms 113, render/thread/ms 91.35, render/busy/ms 89.69, stage/main/update 86.70, stream/drive/ms 53.50, stage/render/prepare_assets 46.01, pipelines/driver_ms 31.48, stage/render/render 22.94, render/graph/ms 20.74, stage/render/graph_submit 17.72 |
| warm-up | 5.13 | 71.7 | pipelines/compile_ms 1717, pipelines/front_ms 1671, main/busy/ms 76.03, stage/main/update 52.63, render/thread/ms 49.51, render/busy/ms 47.89, pipelines/driver_ms 46.36, stream/drive/ms 39.07, stage/render/prepare_assets 18.12, pipelines/wait_ms 15.40, stage/render/render 15.20, render/graph/ms 13.13 |
| warm-up | 5.16 | 37.9 | pipelines/compile_ms 924, pipelines/front_ms 896, main/busy/ms 65.87, render/thread/ms 51.73, render/busy/ms 50.12, stage/main/update 48.77, stream/drive/ms 32.55, pipelines/driver_ms 27.62, stage/render/prepare_assets 25.20, stage/render/render 11.81, render/graph/ms 9.84, stage/main/post_update 8.38 |
| warm-up | 5.21 | 49.7 | pipelines/compile_ms 556, pipelines/front_ms 540, main/busy/ms 64.79, render/thread/ms 49.35, stage/main/update 48.70, render/busy/ms 47.79, stream/drive/ms 32.52, stage/render/prepare_assets 20.64, pipelines/driver_ms 16.69, stage/render/render 12.70, render/graph/ms 10.62, stage/main/post_update 9.15 |
| warm-up | 5.31 | 31.6 | pipelines/compile_ms 350, pipelines/front_ms 335, render/thread/ms 47.22, render/busy/ms 45.67, main/busy/ms 39.41, stage/main/post_update 18.41, stage/render/render 16.88, stage/main/update 15.71, pipelines/driver_ms 15.11, render/graph/ms 14.92, stage/render/prepare_assets 11.00, stage/render/graph_submit 9.99 |
| warm-up | 5.38 | 42.0 | pipelines/compile_ms 677, pipelines/front_ms 658, render/thread/ms 51.41, render/busy/ms 49.65, main/busy/ms 49.22, stage/main/update 25.23, pipelines/driver_ms 19.43, stage/render/prepare_assets 18.84, stage/main/post_update 18.19, stage/render/render 15.95, render/graph/ms 13.85, stream/drive/ms 11.06 |
| warm-up | 5.42 | 40.9 | pipelines/compile_ms 388, pipelines/front_ms 369, render/thread/ms 62.67, render/busy/ms 60.87, main/busy/ms 60.04, stage/main/update 29.61, stage/render/prepare_assets 23.41, stage/main/post_update 19.89, pipelines/driver_ms 18.58, stage/render/render 14.47, render/graph/ms 12.31, stream/drive/ms 11.04 |
| warm-up | 5.46 | 34.1 | pipelines/compile_ms 94.61, pipelines/front_ms 84.55, render/thread/ms 58.92, render/busy/ms 57.52, main/busy/ms 46.37, stage/render/prepare_assets 22.51, stage/main/update 18.83, stage/main/post_update 12.02, stage/render/render 10.22, pipelines/driver_ms 10.05, stage/render/prepare_resources 9.44, render/graph/ms 8.29 |
| warm-up | 5.49 | 30.2 | pipelines/compile_ms 204, pipelines/front_ms 197, render/thread/ms 56.09, render/busy/ms 54.77, main/busy/ms 34.57, stage/render/prepare_assets 13.82, stage/main/update 13.31, stage/render/render 12.00, stage/render/prepare_resources 11.53, stage/main/post_update 10.31, render/graph/ms 9.95, encode//sum 9.28 |
| warm-up | 5.52 | 30.0 | pipelines/compile_ms 396, pipelines/front_ms 384, render/thread/ms 56.46, render/busy/ms 55.06, main/busy/ms 35.53, stage/render/prepare_resources 19.55, stage/main/post_update 17.28, stage/render/render 13.49, stage/main/update 12.49, pipelines/driver_ms 11.88, render/graph/ms 11.50, encode//sum 9.74 |
| warm-up | 5.60 | 37.4 | pipelines/compile_ms 1004, pipelines/front_ms 978, render/thread/ms 52.29, render/busy/ms 50.94, main/busy/ms 43.57, pipelines/wait_ms 32.47, pipelines/driver_ms 25.84, stage/main/update 23.86, stage/render/prepare_assets 20.25, stage/main/post_update 13.84, stream/drive/ms 12.98, stage/render/render 12.43 |
| warm-up | 6.26 | 31.2 | render/thread/ms 38.16, render/busy/ms 37.25, main/busy/ms 28.74, stage/render/prepare_assets 15.07, stage/main/update 11.28, stage/main/post_update 10.75, stage/render/render 7.95, encode//sum 6.78, render/graph/ms 6.66, stage/render/graph_submit 4.60, stage/render/specialize 4.37, stream/drive/ms 3.47 |
| warm-up | 6.47 | 29.9 | main/busy/ms 41.13, render/thread/ms 38.52, render/busy/ms 37.26, stage/main/update 24.59, stage/main/post_update 11.72, stage/render/render 11.04, stage/render/prepare_assets 9.70, encode//sum 9.60, render/graph/ms 9.42, stage/render/graph_submit 6.08, stream/drive/ms 5.90, encode/submit 3.42 |
| warm-up | 6.50 | 32.9 | pipelines/compile_ms 149, pipelines/front_ms 144, render/thread/ms 50.06, render/busy/ms 48.54, main/busy/ms 42.55, stage/main/update 19.72, stage/render/prepare_assets 17.50, stage/main/post_update 16.11, stage/render/render 12.59, encode//sum 10.81, render/graph/ms 10.21, stage/render/graph_submit 6.66 |
| warm-up | 6.71 | 37.7 | pipelines/compile_ms 157, pipelines/front_ms 152, render/thread/ms 48.52, render/busy/ms 46.74, main/busy/ms 44.05, stage/main/update 20.47, stage/render/prepare_assets 19.06, stage/main/post_update 17.68, stage/render/render 15.69, render/graph/ms 13.20, encode//sum 11.33, stage/render/graph_submit 9.18 |
| warm-up | 6.75 | 38.8 | render/thread/ms 59.37, render/busy/ms 57.68, main/busy/ms 56.99, stage/main/update 32.90, stage/render/prepare_assets 32.20, pipelines/compile_ms 21.90, pipelines/front_ms 20.52, stage/main/post_update 17.26, stream/drive/ms 15.24, stage/render/render 14.86, render/graph/ms 12.47, encode//sum 10.02 |
| warm-up | 6.79 | 42.7 | render/thread/ms 63.29, render/busy/ms 62.13, main/busy/ms 49.06, stage/render/prepare_assets 40.70, stage/main/update 29.21, pipelines/compile_ms 21.90, pipelines/front_ms 20.52, stream/drive/ms 18.38, stage/main/post_update 12.83, stage/render/render 12.01, render/graph/ms 10.33, encode//sum 7.93 |
| warm-up | 7.13 | 32.7 | render/thread/ms 43.33, render/busy/ms 42.27, main/busy/ms 21.45, pipelines/compile_ms 15.90, pipelines/front_ms 14.77, stage/render/render 12.30, stage/main/post_update 11.63, render/graph/ms 10.76, stage/render/specialize 10.55, encode//sum 8.78, stage/render/graph_submit 8.27, stage/main/update 7.01 |
| warm-up | 8.44 | 31.8 | render/thread/ms 36.43, render/busy/ms 35.43, main/busy/ms 18.89, stage/main/post_update 11.13, stage/render/render 10.09, stage/render/specialize 9.53, render/graph/ms 8.64, encode//sum 7.20, stage/render/queue 6.81, stage/render/graph_submit 6.61, stage/main/update 5.38, stage/render/prepare_meshes 3.61 |
| measure | 0.42 | 32.6 | render/thread/ms 38.46, render/busy/ms 36.68, main/busy/ms 18.09, stage/render/render 12.31, stage/main/post_update 10.60, render/graph/ms 9.97, stage/render/specialize 9.33, stage/render/graph_submit 7.78, encode//sum 7.42, stage/render/queue 6.91, encode/submit 5.21, stage/main/update 4.09 |
| measure | 1.77 | 31.1 | render/thread/ms 37.85, render/busy/ms 36.78, main/busy/ms 17.31, stage/render/render 11.74, stage/main/post_update 10.96, encode//sum 10.82, render/graph/ms 10.26, stage/render/specialize 8.98, stage/render/graph_submit 7.64, stage/render/queue 6.00, stage/render/prepare_meshes 4.66, encode/submit 4.32 |
| measure | 3.24 | 30.9 | render/thread/ms 42.08, render/busy/ms 41.07, main/busy/ms 20.73, stage/render/render 14.67, stage/main/post_update 13.59, render/graph/ms 13.10, encode//sum 10.60, stage/render/specialize 9.93, stage/render/graph_submit 9.79, encode/submit 6.42, stage/render/queue 6.34, encode/pass:shadow_point_light/sum 5.30 |
| measure | 4.73 | 30.8 | render/thread/ms 38.22, render/busy/ms 37.16, main/busy/ms 19.84, stage/main/post_update 13.07, stage/render/render 12.18, render/graph/ms 10.64, stage/render/specialize 9.50, encode//sum 8.69, stage/render/graph_submit 8.12, stage/render/queue 6.72, encode/submit 4.99, stage/main/update 4.12 |
| measure | 6.13 | 33.1 | render/thread/ms 41.18, render/busy/ms 40.08, main/busy/ms 20.47, stage/render/render 12.16, stage/main/post_update 11.26, render/graph/ms 10.60, stage/render/specialize 10.26, encode//sum 9.04, stage/render/graph_submit 8.05, stage/render/queue 7.65, stage/main/update 4.77, encode/submit 4.73 |
| measure | 10.30 | 30.5 | render/thread/ms 38.96, render/busy/ms 37.87, main/busy/ms 18.89, stage/render/render 12.24, stage/main/post_update 11.57, render/graph/ms 10.65, stage/render/specialize 9.81, encode//sum 9.64, stage/render/graph_submit 7.95, stage/render/queue 6.92, encode/submit 4.83, encode/pass:shadow_point_light/sum 4.37 |
| measure | 11.69 | 31.0 | render/thread/ms 39.11, render/busy/ms 38.03, main/busy/ms 18.26, stage/render/render 13.21, render/graph/ms 11.65, encode//sum 11.56, stage/main/post_update 11.49, stage/render/specialize 9.43, stage/render/graph_submit 8.67, stage/render/queue 6.95, encode/pass:shadow_point_light/sum 5.68, encode/submit 5.17 |
| measure | 15.73 | 31.0 | render/thread/ms 38.86, render/busy/ms 37.83, main/busy/ms 19.30, stage/main/post_update 12.52, stage/render/render 11.03, stage/render/specialize 10.38, render/graph/ms 9.55, encode//sum 8.26, stage/render/queue 7.64, stage/render/graph_submit 7.30, encode/submit 4.28, stage/main/update 4.15 |
| measure | 17.12 | 30.3 | render/thread/ms 39.96, render/busy/ms 39.00, main/busy/ms 20.05, stage/render/render 12.74, stage/main/post_update 11.50, render/graph/ms 11.35, stage/render/specialize 10.21, encode//sum 9.68, stage/render/graph_submit 8.44, stage/render/queue 7.01, stage/main/update 5.30, encode/submit 4.80 |
| measure | 19.48 | 30.3 | render/thread/ms 40.87, render/busy/ms 39.91, main/busy/ms 23.40, stage/main/post_update 13.18, stage/render/render 12.77, render/graph/ms 11.36, encode//sum 10.93, stage/render/specialize 10.52, stage/render/graph_submit 8.53, stage/render/queue 7.00, stage/main/update 6.08, encode/submit 5.51 |
| measure | 20.88 | 30.3 | render/thread/ms 37.67, render/busy/ms 36.64, main/busy/ms 18.61, stage/main/post_update 11.36, stage/render/render 11.18, encode//sum 10.06, stage/render/specialize 9.92, render/graph/ms 9.74, stage/render/graph_submit 7.37, stage/render/queue 7.30, encode/submit 4.63, stage/main/update 4.28 |
| measure | 23.03 | 35.2 | render/thread/ms 42.31, render/busy/ms 41.24, main/busy/ms 25.77, stage/main/post_update 13.25, stage/render/render 12.66, stage/render/specialize 11.56, render/graph/ms 11.10, encode//sum 10.83, stage/render/graph_submit 8.63, stage/render/queue 7.92, stage/main/update 6.58, encode/submit 5.22 |
| measure | 27.30 | 38.4 | render/thread/ms 48.29, render/busy/ms 47.23, main/busy/ms 27.78, encode//sum 19.96, stage/render/render 16.46, render/graph/ms 14.96, stage/main/post_update 14.84, stage/render/specialize 12.09, stage/render/graph_submit 11.80, stage/render/queue 8.56, encode/pass:shadow_point_light/sum 7.99, encode/submit 7.11 |
| measure | 28.78 | 30.6 | render/thread/ms 39.04, render/busy/ms 37.97, main/busy/ms 19.21, stage/main/post_update 12.14, stage/render/render 11.84, render/graph/ms 10.28, stage/render/specialize 10.28, encode//sum 9.65, stage/render/graph_submit 7.79, stage/render/queue 6.84, encode/submit 4.86, stage/main/update 4.59 |
| measure | 30.48 | 34.8 | render/thread/ms 42.55, render/busy/ms 41.51, main/busy/ms 21.88, stage/main/post_update 13.33, stage/render/render 12.65, stage/render/specialize 11.67, render/graph/ms 11.22, encode//sum 10.46, stage/render/graph_submit 8.50, stage/render/queue 7.24, stage/main/update 5.75, encode/submit 5.07 |
| measure | 32.08 | 31.2 | render/thread/ms 40.68, render/busy/ms 39.59, main/busy/ms 19.93, stage/render/render 13.41, stage/main/post_update 13.21, render/graph/ms 11.88, encode//sum 11.52, stage/render/specialize 9.80, stage/render/graph_submit 9.23, stage/render/queue 7.64, encode/submit 6.09, stage/main/update 4.21 |
| measure | 35.35 | 32.1 | render/thread/ms 41.21, render/busy/ms 40.26, main/busy/ms 20.45, stage/render/render 13.18, stage/main/post_update 12.57, render/graph/ms 11.69, encode//sum 11.07, stage/render/specialize 10.37, stage/render/graph_submit 8.91, stage/render/queue 7.54, encode/pass:shadow_point_light/sum 5.88, encode/submit 5.41 |
| measure | 40.82 | 35.9 | render/thread/ms 39.93, render/busy/ms 38.90, main/busy/ms 19.26, stage/main/post_update 11.23, stage/render/render 10.88, stage/render/specialize 10.13, render/graph/ms 9.46, encode//sum 9.29, stage/render/graph_submit 7.52, stage/render/queue 7.23, stage/main/update 4.47, encode/submit 3.81 |
| measure | 43.21 | 32.0 | render/thread/ms 38.19, render/busy/ms 37.16, main/busy/ms 17.34, stage/render/render 11.39, stage/main/post_update 10.59, render/graph/ms 10.08, stage/render/specialize 10.02, encode//sum 9.50, stage/render/graph_submit 8.19, stage/render/queue 6.59, stage/main/update 4.24, encode/submit 4.20 |
| measure | 45.65 | 34.9 | render/thread/ms 40.28, render/busy/ms 39.08, main/busy/ms 18.48, stage/render/render 11.43, stage/main/post_update 10.76, stage/render/specialize 9.96, render/graph/ms 9.93, encode//sum 9.50, stage/render/graph_submit 7.90, stage/render/queue 7.00, stage/render/prepare_meshes 4.91, stage/main/update 4.42 |
| measure | 59.04 | 50.2 | render/thread/ms 54.91, render/busy/ms 53.16, main/busy/ms 33.64, stage/main/post_update 19.26, stage/render/render 15.80, render/graph/ms 13.53, encode//sum 13.50, stage/render/graph_submit 10.58, stage/render/specialize 10.18, stage/main/update 9.83, stage/render/prepare_meshes 8.29, stage/render/prepare_assets 7.31 |
| measure | 59.07 | 33.0 | render/thread/ms 60.30, render/busy/ms 59.28, main/busy/ms 26.62, stage/render/prepare_assets 17.74, stage/main/post_update 13.24, stage/render/render 10.47, stage/render/specialize 10.28, encode//sum 9.84, render/graph/ms 9.09, stage/render/prepare_meshes 9.08, stage/main/update 8.05, stage/render/graph_submit 7.04 |
| measure | 62.58 | 32.6 | render/thread/ms 40.24, render/busy/ms 39.11, main/busy/ms 20.08, stage/render/render 11.80, encode//sum 11.65, stage/main/post_update 11.24, stage/render/specialize 10.92, render/graph/ms 10.42, stage/render/graph_submit 8.15, stage/render/queue 6.81, stage/main/update 6.35, encode/submit 4.32 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=base-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 20%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **74%** (min 56%) · 2621 MHz (min 2610) · memory 8101 MHz · 77.6 W · 77 °C (max 82) · VRAM 6800 MiB · throttled 100% of samples

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
