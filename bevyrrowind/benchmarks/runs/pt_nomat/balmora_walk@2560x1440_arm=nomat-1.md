# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.depth_only.skip_material=on (command line: --set); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6027 frames over 65.6 s · **median 10.61 ms (94 fps)** · mean 10.89 · p95 13.40 · p99 17.48 (1% low 57 fps) · worst 55.65 ms · 87 spikes over 1.5× median · 69.5 stutters/min (work 58.5/min)
- warm-up: 323 frames over 9.5 s, worst 2618.4 ms
- hitches (> 26.5 ms): **33** measured, 59 in warm-up
- timed GPU passes: 6.70 ms of a 10.89 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.61 ms, limit 11 ms
- **FAIL** frame_ms: 76 stutters (> 1.5x moving median of 60) in 1.09 min = 69.46/min, limit 1/min
- **FAIL** worst frame 55.65 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.894 | 2.035 | 2.067 | 0.93 | 0.175 |
| main_opaque_pass_3d | 1.525 | 1.639 | 2.050 | 0.93 | 0.069 |
| early prepass | 0.466 | 0.500 | 0.735 | 0.93 | 0.089 |
| bloom | 0.292 | 0.314 | 0.321 | 0.93 | 0.062 |
| atmosphere_luts | 0.284 | 0.305 | 0.366 | 0.93 | 0.003 |
| water_velocity | 0.259 | 0.278 | 0.347 | 0.93 | 0.002 |
| water_sspr | 0.187 | 0.201 | 0.202 | 0.93 | 0.002 |
| ssao | 0.176 | 0.189 | 0.191 | 0.93 | 0.007 |
| atmos_froxel_inject | 0.136 | 0.146 | 0.147 | 0.93 | 0.002 |
| clouds_trace | 0.130 | 0.140 | 0.232 | 0.93 | 0.001 |
| water_fft | 0.121 | 0.130 | 0.137 | 0.93 | 0.003 |
| uniform_allocation | 0.121 | 0.130 | 0.181 | 0.93 | 0.026 |
| auxiliary_views | 0.118 | 0.127 | 0.500 | 0.93 | 0.127 |
| upscaling | 0.109 | 0.117 | 0.125 | 0.93 | 0.004 |
| clouds_composite | 0.098 | 0.106 | 0.109 | 0.93 | 0.002 |
| bin_unpacking | 0.098 | 0.105 | 0.142 | 0.93 | 0.019 |
| main_transmissive_pass_3d | 0.080 | 0.086 | 0.179 | 0.93 | 0.009 |
| tonemapping | 0.079 | 0.085 | 0.087 | 0.93 | 0.003 |
| clustering | 0.059 | 0.064 | 0.067 | 0.93 | 0.019 |
| render_sky | 0.048 | 0.052 | 0.053 | 0.93 | 0.002 |
| clouds_resolve | 0.045 | 0.048 | 0.049 | 0.93 | 0.001 |
| lightprobe_irradiance_map | 0.042 | 0.414 | 0.416 | 0.10 | 0.001 |
| surface_exposure | 0.041 | 0.044 | 0.045 | 0.93 | 0.001 |
| surface_trail | 0.037 | 0.056 | 0.057 | 0.65 | 0.002 |
| surface_exposure_far | 0.035 | 0.043 | 0.045 | 0.82 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.93 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.046 | 0.93 | 0.005 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.031 | 0.93 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.93 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.219 | 0.231 | 0.10 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.93 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.013 | 0.027 | 0.93 | 0.122 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.93 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.93 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.93 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.93 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.334 | 11.361 | 53.89 | 56256.9 |
| main/busy/ms | 8.993 | 10.668 | 28.69 | 54198.9 |
| render/busy/ms | 8.792 | 10.857 | 53.31 | 52991.7 |
| stage/main/post_update | 5.567 | 6.900 | 9.85 | 33551.9 |
| stage/render/render | 5.428 | 6.778 | 50.37 | 32716.4 |
| render/graph/ms | 4.695 | 6.025 | 49.64 | 28294.9 |
| encode//sum | 3.743 | 4.833 | 47.32 | 22556.9 |
| stage/render/graph_submit | 3.523 | 4.600 | 48.22 | 21230.6 |
| encode/submit | 2.243 | 3.124 | 6.55 | 13518.0 |
| stage/main/update | 2.148 | 2.578 | 22.02 | 12945.8 |
| encode/finish_wall | 1.346 | 1.927 | 44.12 | 8109.4 |
| stage/render/camera_driver | 1.120 | 1.534 | 7.94 | 6748.5 |
| encode/pass:shadow_directional_light/sum | 0.842 | 1.515 | 2.67 | 5075.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.819 | 1.114 | 2.14 | 4935.1 |
| encode/pass:early prepass/sum | 0.810 | 1.109 | 43.56 | 4879.0 |
| stage/render/prepare_write_phase_buffers | 0.750 | 1.092 | 2.41 | 4517.8 |
| stage/render/prepare_assets | 0.580 | 0.751 | 16.91 | 3493.5 |
| bigspace/total/ms | 0.545 | 1.310 | 2.18 | 3286.4 |
| stage/render/prepare_resources | 0.449 | 0.547 | 1.00 | 2703.8 |
| render/present/ms | 0.448 | 0.739 | 2.56 | 2702.2 |
| stage/render/prepare_meshes | 0.441 | 0.564 | 5.70 | 2657.7 |
| stage/main/first | 0.401 | 0.527 | 5.33 | 2418.1 |
| encode/pass:shadow_point_light/sum | 0.400 | 2.214 | 7.27 | 2409.4 |
| stage/main/pre_update | 0.361 | 0.522 | 3.55 | 2172.8 |
| stage/render/specialize | 0.323 | 0.425 | 11.38 | 1948.2 |
| terrain/update/ms | 0.305 | 0.392 | 1.51 | 1840.9 |
| stage/render/queue | 0.292 | 0.421 | 9.83 | 1757.3 |
| stage/render/prepare_bind_groups | 0.280 | 0.499 | 2.35 | 1685.4 |
| terrain/select/ms | 0.272 | 0.349 | 1.08 | 1637.3 |
| controller/feed/ms | 0.263 | 0.335 | 0.71 | 1586.0 |
| stream/drive/ms | 0.254 | 0.337 | 19.83 | 1528.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.237 | 0.350 | 1.09 | 1426.6 |
| bigspace/low_precision/ms | 0.225 | 0.699 | 1.76 | 1357.0 |
| stage/main/fixed_main_loop | 0.218 | 0.375 | 1.14 | 1316.7 |
| far/select/ms | 0.202 | 0.277 | 0.76 | 1216.7 |
| stage/render/create_views | 0.198 | 0.257 | 1.56 | 1194.2 |
| stream/tick/ms | 0.196 | 0.268 | 0.63 | 1183.9 |
| stage/render/prepare_batch_phases | 0.191 | 0.278 | 0.59 | 1151.3 |
| stage/render/extract_commands | 0.172 | 0.203 | 0.40 | 1036.9 |
| stage/render/prepare_flush | 0.166 | 0.315 | 1.88 | 998.9 |
| anim/evaluate/ms | 0.160 | 0.274 | 1.45 | 964.4 |
| bigspace/high_precision/ms | 0.156 | 0.644 | 1.71 | 939.0 |
| anim/evaluate_cpu/ms | 0.151 | 0.188 | 0.61 | 907.6 |
| stage/render/prepare_views | 0.137 | 0.171 | 0.55 | 824.1 |
| controller/move/ms | 0.129 | 0.280 | 0.48 | 780.4 |
| stage/main/last | 0.119 | 0.161 | 4.93 | 718.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.097 | 0.155 | 0.65 | 585.2 |
| render/acquire/ms | 0.093 | 0.119 | 0.48 | 563.0 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.126 | 0.51 | 351.3 |
| pipelines/compile_ms | 0.050 | 0.000 | 126.31 | 300.9 |
| pipelines/front_ms | 0.048 | 0.000 | 121.95 | 289.1 |
| physics/step/ms | 0.048 | 0.088 | 0.58 | 288.7 |
| stage/render/cleanup | 0.032 | 0.096 | 0.86 | 193.3 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.66 | 189.5 |
| physics/insert/ms | 0.031 | 0.356 | 0.94 | 188.4 |
| stage/render/phase_sort | 0.029 | 0.051 | 0.28 | 177.3 |
| weather/map/ms | 0.024 | 0.032 | 0.24 | 146.9 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.54 | 89.0 |
| encode/camera_driver_span_end/sum | 0.012 | 0.019 | 0.66 | 70.3 |
| anim/apply/ms | 0.011 | 0.020 | 0.45 | 68.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.10 | 62.1 |
| controller/avoid/ms | 0.010 | 0.014 | 0.05 | 61.8 |
| script/update/ms | 0.007 | 0.009 | 0.20 | 42.6 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 33.0 |
| stream/despawn/ms | 0.005 | 0.000 | 1.17 | 30.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 28.7 |
| script/ui/ms | 0.004 | 0.006 | 0.22 | 26.9 |
| controller/write/ms | 0.004 | 0.004 | 0.23 | 21.1 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.81 | 14.9 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.36 | 11.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.30 | 3.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.3 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1272 | 2262 | 641 |
| crowd/c0/meshes/count | count | 349 | 646 | 174 |
| crowd/c1/count | count | 146 | 156 | 147 |
| crowd/c1/joints/count | count | 5539 | 5991 | 5555 |
| crowd/c1/meshes/count | count | 1179 | 1295 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4612 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4589 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.41 | 43.94 | 0.40 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.54 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.66 | 0.02 |
| encode/finish_critical | ms | 0.98 | 43.94 | 1.14 |
| encode/pass:early prepass/max | ms | 0.80 | 43.56 | 1.14 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 2.14 | 0.72 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.65 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 1.09 | 0.17 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 2.67 | 1.02 |
| encode/pass:shadow_point_light/max | ms | 0.58 | 1.49 | 0.76 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.66 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.18 | 53.31 | 8.30 |
| gpu/timed/ms | ms | 7.90 | 11.14 | 8.30 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 821 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 821 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1288 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3423 | 5114 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1288 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3423 | 5114 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 17 | 203 | 4 |
| phases/shadow/bins | count | 5023 | 8788 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6386 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 204 | 254 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 181 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 323 | 324 | 324 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 12 | 563 | 9 |
| preprocess/bind_groups_created | count | 12 | 563 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 211 | 120 |
| process/cpu_usage | % | 26.70 | 41.35 | 27.40 |
| process/mem_usage | GiB | 5.49 | 5.68 | 5.66 |
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
| skin/blocks_changed/count | count | 13 | 122 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 370.2 k | 810.5 k | 492.5 k |
| skin/joints_written/count | count | 5784 | 12.7 k | 7696 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4917 | 5011 | 5011 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.78 | 48.94 | 33.76 |
| system/mem_usage | % | 62.49 | 63.37 | 63.21 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 66 | 252 | 54 |
| views/all/bins | count | 7638 | 11.7 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 251 | 301 | 75 |
| views/all/pipelines_distinct | count | 70 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 34.8 k | 8127 |
| views/cascade/batch_sets | count | 9 | 48 | 4 |
| views/cascade/bins | count | 2805 | 6179 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8907 | 23.5 k | 6386 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 403 | 22.8 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2147 | 2821 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2404 | 3123 | 0 |
| views/lamp_face/pipelines | count | 168 | 207 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2933 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2874 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8484 | 13.0 k | 7618 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8643 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 53.09 | 251 | 320.0 k |
| anim/events/count | count | 0.09 | 4 | 518 |
| assets/Image/added | count | 0.10 | 37 | 592 |
| assets/Image/modified | count | 0.00 | 3 | 19 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.59 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.78 | 1309 | 4676 |
| encode//count | count | 32.77 | 219 | 197.5 k |
| encode/bind_walk/init_textures | count | 10.3 k | 44.8 k | 62.08 M |
| encode/bind_walk/init_textures_skipped | count | 48.4 k | 113.8 k | 291.43 M |
| encode/bind_walk/inits | count | 455 | 1095 | 2.74 M |
| encode/bind_walk/inits_skipped | count | 102 | 284 | 614.9 k |
| encode/bind_walk/merge_textures | count | 10.2 k | 44.7 k | 61.64 M |
| encode/bind_walk/merge_textures_skipped | count | 7871 | 19.3 k | 47.44 M |
| encode/bind_walk/merges | count | 96.74 | 303 | 583.1 k |
| encode/bind_walk/merges_skipped | count | 90.48 | 260 | 545.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 6027 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 6027 |
| encode/pass:early prepass/count | count | 1 | 2 | 6027 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 6027 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 6027 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 6027 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 6027 |
| encode/pass:shadow_point_light/count | count | 1.21 | 48 | 7272 |
| encode/queue/command_buffers | count | 44.00 | 277 | 265.2 k |
| encode/queue/submits | count | 2.02 | 4 | 12.2 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.70 M | 11.35 G |
| encode/queue/write_buffer/calls | count | 257 | 843 | 1.55 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.7 k | 3.52 M | 1.07 G |
| encode/queue/write_buffer_with/calls | count | 11.01 | 465 | 66.3 k |
| encode/queue/write_texture/bytes | bytes | 12.0 k | 8.39 M | 72.50 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 565 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 6027 |
| physics/inserts | count | 0.37 | 91 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.65 | 3 | 3939 |
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
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5617 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1231 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.5 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.62 | 2618.4 | main/busy/ms 2588, stage/main/post_update 2483, stage/main/update 96.00, surface/advance_cpu_ms 66.76, stage/main/pre_update 5.09, stage/main/last 1.14, surface/cell_upload_ms 0.77, stream/drive/ms 0.52, stage/main/first 0.52, stage/main/fixed_main_loop 0.35, weather/map/ms 0.24, stream/tick/ms 0.17 |
| warm-up | 3.46 | 840.3 | main/busy/ms 2770, stage/main/post_update 2514, render/thread/ms 806, render/busy/ms 625, stage/render/prepare_assets 574, stage/main/update 241, stage/render/prepare_views 178, render/acquire/ms 177, surface/advance_cpu_ms 66.76, stage/render/create_views 28.69, stage/render/render 13.90, render/graph/ms 9.31 |
| warm-up | 3.58 | 121.6 | render/thread/ms 910, render/busy/ms 668, stage/render/prepare_assets 602, main/busy/ms 203, stage/render/prepare_views 178, render/acquire/ms 178, stage/main/update 150, pipelines/compile_ms 113, stage/render/render 81.40, render/present/ms 65.02, pipelines/front_ms 63.54, pipelines/driver_ms 49.09 |
| warm-up | 3.63 | 49.0 | pipelines/compile_ms 537, pipelines/front_ms 413, pipelines/wait_ms 344, render/thread/ms 151, pipelines/driver_ms 125, stage/render/render 108, render/busy/ms 84.33, main/busy/ms 68.45, render/present/ms 66.53, stage/main/post_update 51.22, render/graph/ms 40.65, stage/render/graph_submit 37.77 |
| warm-up | 3.76 | 127.6 | pipelines/wait_ms 4812, pipelines/compile_ms 1723, pipelines/front_ms 1586, render/thread/ms 171, render/busy/ms 165, pipelines/driver_ms 137, main/busy/ms 75.48, stage/main/post_update 55.02, stage/render/render 54.94, render/graph/ms 48.01, stage/render/graph_submit 45.35, stage/render/prepare_resources 42.66 |
| warm-up | 3.80 | 42.4 | pipelines/wait_ms 4533, pipelines/compile_ms 1728, pipelines/front_ms 1640, render/thread/ms 162, render/busy/ms 160, main/busy/ms 110, pipelines/driver_ms 87.84, stage/main/update 60.83, stage/render/prepare_resources 47.22, stage/render/prepare_batch_phases 42.42, stage/render/prepare_assets 27.60, stage/main/post_update 24.77 |
| warm-up | 3.91 | 115.7 | pipelines/compile_ms 1238, pipelines/front_ms 1169, pipelines/wait_ms 171, render/thread/ms 105, render/busy/ms 103, main/busy/ms 97.28, stage/render/prepare_assets 73.53, pipelines/driver_ms 69.31, stage/main/update 63.15, stage/main/post_update 17.91, terrain/update/ms 14.44, stage/render/render 11.96 |
| warm-up | 3.96 | 47.3 | pipelines/compile_ms 2009, pipelines/front_ms 1759, pipelines/driver_ms 250, pipelines/wait_ms 243, render/thread/ms 112, render/busy/ms 110, stage/render/prepare_assets 93.22, main/busy/ms 30.43, encode//sum 15.26, stage/main/post_update 14.94, stage/render/render 11.64, terrain/update/ms 10.70 |
| warm-up | 4.00 | 38.5 | pipelines/compile_ms 1708, pipelines/front_ms 1457, pipelines/driver_ms 251, pipelines/wait_ms 138, render/thread/ms 83.27, render/busy/ms 73.04, stage/render/prepare_assets 36.33, encode//sum 34.65, main/busy/ms 31.24, stage/render/render 28.07, render/graph/ms 25.24, stage/render/graph_submit 22.51 |
| warm-up | 4.14 | 82.7 | pipelines/compile_ms 782, pipelines/front_ms 739, render/thread/ms 59.82, main/busy/ms 59.64, render/busy/ms 58.23, stage/main/update 46.96, pipelines/driver_ms 43.26, stream/drive/ms 41.96, stage/render/prepare_assets 34.47, stage/render/render 10.90, render/graph/ms 9.06, stage/render/prepare_write_phase_buffers 8.87 |
| warm-up | 4.20 | 43.5 | pipelines/compile_ms 81.17, pipelines/front_ms 74.47, render/thread/ms 38.90, main/busy/ms 37.51, render/busy/ms 37.32, stage/render/prepare_assets 21.16, stage/main/update 17.37, stage/main/post_update 13.56, stage/render/render 10.73, stream/drive/ms 9.89, render/graph/ms 8.74, pipelines/driver_ms 6.70 |
| warm-up | 4.24 | 32.6 | pipelines/compile_ms 717, pipelines/front_ms 684, render/thread/ms 53.41, render/busy/ms 51.80, main/busy/ms 47.29, stage/render/prepare_assets 33.47, pipelines/driver_ms 32.88, stage/main/update 31.53, stream/drive/ms 26.23, stage/render/render 10.11, stage/main/post_update 9.93, render/graph/ms 7.97 |
| warm-up | 4.30 | 60.5 | pipelines/compile_ms 713, pipelines/front_ms 683, render/thread/ms 66.96, render/busy/ms 65.03, main/busy/ms 58.51, stage/render/prepare_assets 44.29, stage/main/update 31.78, pipelines/driver_ms 30.07, stream/drive/ms 27.70, stage/main/pre_update 12.07, stage/render/render 11.79, stage/main/post_update 10.15 |
| warm-up | 4.33 | 37.6 | pipelines/compile_ms 107, pipelines/front_ms 102, render/thread/ms 64.54, render/busy/ms 62.52, stage/render/prepare_assets 42.29, main/busy/ms 37.39, stage/render/render 13.58, stage/main/pre_update 11.73, render/graph/ms 11.08, stage/main/update 9.77, stage/main/post_update 9.63, stage/render/graph_submit 8.45 |
| warm-up | 4.41 | 77.6 | pipelines/compile_ms 1159, pipelines/front_ms 1123, render/thread/ms 72.57, render/busy/ms 70.22, main/busy/ms 66.91, stage/render/prepare_assets 49.99, stage/main/update 45.17, pipelines/driver_ms 36.20, pipelines/wait_ms 17.31, stage/main/post_update 13.62, stage/render/render 13.52, render/graph/ms 10.59 |
| warm-up | 4.56 | 153.2 | pipelines/compile_ms 2712, pipelines/front_ms 2635, render/thread/ms 174, render/busy/ms 172, stage/render/prepare_assets 148, pipelines/driver_ms 76.60, main/busy/ms 72.56, stage/main/update 48.99, pipelines/wait_ms 19.43, stage/main/post_update 14.33, stage/render/render 13.06, render/graph/ms 10.11 |
| warm-up | 4.62 | 53.9 | pipelines/compile_ms 2766, pipelines/front_ms 2694, render/thread/ms 177, render/busy/ms 175, stage/render/prepare_assets 124, pipelines/wait_ms 90.54, pipelines/driver_ms 71.87, main/busy/ms 57.06, stage/main/pre_update 19.36, stage/main/update 17.23, stage/render/prepare_bind_groups 14.67, stage/render/render 12.59 |
| warm-up | 4.74 | 119.8 | pipelines/compile_ms 1546, pipelines/front_ms 1507, pipelines/wait_ms 549, render/thread/ms 157, render/busy/ms 156, main/busy/ms 57.42, stage/render/prepare_assets 51.47, pipelines/driver_ms 39.32, stage/main/pre_update 25.96, stage/render/render 22.88, stage/render/prepare_bind_groups 22.56, render/graph/ms 20.69 |
| warm-up | 4.82 | 83.3 | pipelines/compile_ms 2477, pipelines/front_ms 2391, pipelines/wait_ms 1266, render/thread/ms 181, render/busy/ms 179, pipelines/driver_ms 86.05, main/busy/ms 61.11, encode//sum 55.71, stage/render/prepare_assets 48.09, stage/render/render 39.69, render/graph/ms 37.27, stage/main/post_update 32.65 |
| warm-up | 4.89 | 64.3 | pipelines/compile_ms 2817, pipelines/front_ms 2709, pipelines/wait_ms 1242, render/thread/ms 123, render/busy/ms 121, pipelines/driver_ms 108, main/busy/ms 58.13, encode//sum 56.42, stage/render/render 38.74, render/graph/ms 36.09, stage/main/post_update 33.58, stage/render/prepare_assets 30.70 |
| warm-up | 4.92 | 35.1 | pipelines/compile_ms 774, pipelines/front_ms 739, pipelines/wait_ms 440, main/busy/ms 83.87, render/thread/ms 79.95, render/busy/ms 77.71, stage/main/update 61.19, stream/drive/ms 52.06, pipelines/driver_ms 34.96, stage/render/prepare_assets 26.76, stage/render/render 24.96, render/graph/ms 22.14 |
| warm-up | 5.00 | 77.8 | pipelines/compile_ms 888, pipelines/front_ms 861, pipelines/wait_ms 436, main/busy/ms 106, stage/main/update 82.57, render/thread/ms 67.36, render/busy/ms 63.57, stream/drive/ms 52.24, pipelines/driver_ms 27.47, stage/render/render 21.93, render/graph/ms 16.90, stage/render/prepare_assets 15.72 |
| warm-up | 5.06 | 57.4 | pipelines/compile_ms 1736, pipelines/front_ms 1687, pipelines/wait_ms 732, main/busy/ms 131, stage/main/update 105, render/thread/ms 70.63, render/busy/ms 66.92, stream/drive/ms 53.55, pipelines/driver_ms 49.50, stage/render/prepare_assets 30.60, stage/render/render 19.77, render/graph/ms 14.73 |
| warm-up | 5.17 | 118.0 | pipelines/compile_ms 1618, pipelines/front_ms 1568, pipelines/wait_ms 302, main/busy/ms 106, stage/main/update 82.86, render/thread/ms 75.17, render/busy/ms 73.24, stream/drive/ms 54.37, pipelines/driver_ms 49.56, stage/render/prepare_assets 41.25, stage/render/render 13.12, stage/main/post_update 12.73 |
| warm-up | 5.23 | 52.5 | pipelines/compile_ms 1030, pipelines/front_ms 993, main/busy/ms 85.85, render/thread/ms 79.05, render/busy/ms 77.34, stage/main/update 62.71, stream/drive/ms 49.12, stage/render/prepare_assets 42.54, pipelines/driver_ms 37.58, stage/render/render 14.02, stage/main/post_update 13.78, render/graph/ms 11.56 |
| warm-up | 5.31 | 80.1 | pipelines/compile_ms 721, pipelines/front_ms 699, main/busy/ms 93.03, render/thread/ms 74.33, render/busy/ms 72.56, stage/main/update 67.80, stream/drive/ms 48.11, stage/render/prepare_assets 36.75, pipelines/driver_ms 21.40, stage/render/render 17.04, stage/main/post_update 15.25, render/graph/ms 14.28 |
| warm-up | 5.34 | 30.6 | pipelines/compile_ms 690, pipelines/front_ms 672, main/busy/ms 77.40, render/thread/ms 60.08, render/busy/ms 58.28, stage/main/update 57.01, stream/drive/ms 38.55, stage/render/prepare_assets 22.86, stage/render/render 18.18, pipelines/driver_ms 17.97, render/graph/ms 15.48, stage/main/post_update 12.09 |
| warm-up | 5.41 | 76.4 | pipelines/compile_ms 708, pipelines/front_ms 685, main/busy/ms 74.09, stage/main/update 56.69, render/thread/ms 56.08, render/busy/ms 53.54, stream/drive/ms 38.99, stage/render/prepare_assets 22.54, pipelines/driver_ms 22.47, stage/render/render 16.28, render/graph/ms 13.11, stage/render/graph_submit 9.96 |
| warm-up | 5.45 | 34.7 | pipelines/compile_ms 1268, pipelines/front_ms 1234, render/thread/ms 63.14, render/busy/ms 60.77, main/busy/ms 40.93, pipelines/driver_ms 34.39, stage/main/update 19.81, stage/render/prepare_assets 19.54, stage/render/render 14.21, stage/main/post_update 13.49, render/graph/ms 11.34, stage/render/graph_submit 8.22 |
| warm-up | 5.51 | 38.9 | pipelines/compile_ms 684, pipelines/front_ms 665, render/thread/ms 54.90, render/busy/ms 53.45, main/busy/ms 46.95, stage/main/update 26.11, stage/render/prepare_assets 22.78, pipelines/driver_ms 19.15, stage/main/post_update 14.11, stage/render/render 11.50, render/graph/ms 9.47, encode//sum 8.36 |
| warm-up | 5.55 | 34.4 | pipelines/compile_ms 1039, pipelines/front_ms 1009, render/thread/ms 60.74, render/busy/ms 58.63, main/busy/ms 44.03, pipelines/driver_ms 30.02, stage/render/prepare_assets 25.16, stage/main/update 21.94, stage/render/render 15.53, stage/main/post_update 14.69, render/graph/ms 13.02, stage/render/graph_submit 9.48 |
| warm-up | 5.61 | 38.3 | pipelines/compile_ms 367, pipelines/front_ms 358, render/thread/ms 50.04, render/busy/ms 48.52, main/busy/ms 33.43, stage/render/prepare_assets 18.53, stage/main/update 16.20, stage/render/render 14.00, render/graph/ms 11.98, stage/main/post_update 10.90, pipelines/driver_ms 9.60, stage/render/graph_submit 8.92 |
| warm-up | 5.83 | 50.4 | pipelines/compile_ms 601, pipelines/front_ms 585, render/thread/ms 67.26, render/busy/ms 65.82, main/busy/ms 44.08, stage/render/prepare_assets 29.18, stage/main/update 20.70, pipelines/driver_ms 16.43, stage/main/post_update 15.88, stage/render/render 12.47, render/graph/ms 10.54, encode//sum 7.69 |
| warm-up | 5.91 | 31.4 | pipelines/compile_ms 186, pipelines/front_ms 177, render/thread/ms 53.67, render/busy/ms 52.16, main/busy/ms 47.14, stage/render/prepare_assets 27.58, stage/main/update 25.28, stage/main/post_update 15.46, stage/render/render 13.63, stream/drive/ms 12.34, render/graph/ms 11.24, encode//sum 9.62 |
| warm-up | 5.97 | 34.0 | pipelines/compile_ms 572, pipelines/front_ms 558, render/thread/ms 50.80, render/busy/ms 49.39, main/busy/ms 27.10, stage/render/prepare_assets 26.27, pipelines/driver_ms 13.61, stage/main/post_update 12.68, stage/render/render 10.56, encode//sum 9.30, stage/main/update 8.58, render/graph/ms 8.41 |
| warm-up | 6.00 | 30.7 | pipelines/compile_ms 219, pipelines/front_ms 214, render/thread/ms 53.59, render/busy/ms 52.20, stage/render/prepare_assets 33.17, main/busy/ms 25.12, stage/render/render 11.49, stage/main/post_update 11.49, render/graph/ms 9.35, encode//sum 8.48, stage/main/update 8.32, stage/render/graph_submit 6.55 |
| warm-up | 6.06 | 33.1 | pipelines/compile_ms 388, pipelines/front_ms 363, render/thread/ms 50.26, render/busy/ms 48.79, main/busy/ms 30.28, stage/render/prepare_assets 29.44, pipelines/driver_ms 25.45, stage/main/post_update 12.54, stage/main/update 11.99, stage/render/render 11.29, render/graph/ms 9.07, encode//sum 7.67 |
| warm-up | 6.11 | 29.1 | render/thread/ms 45.84, render/busy/ms 44.39, stage/render/prepare_assets 27.09, main/busy/ms 26.79, stage/main/post_update 12.31, stage/render/render 11.02, render/graph/ms 8.88, stage/main/update 8.53, encode//sum 8.52, stage/render/graph_submit 5.99, encode/submit 3.73, stream/drive/ms 2.87 |
| warm-up | 6.62 | 33.7 | render/thread/ms 44.93, render/busy/ms 43.71, main/busy/ms 30.70, stage/render/prepare_assets 18.41, stage/main/update 11.99, stage/main/post_update 10.24, stage/render/render 9.32, encode//sum 7.80, render/graph/ms 7.68, stage/render/specialize 5.48, stage/render/graph_submit 5.30, stage/main/pre_update 4.67 |
| warm-up | 6.82 | 30.9 | main/busy/ms 38.25, render/thread/ms 34.84, render/busy/ms 33.65, stage/main/update 20.25, stage/main/post_update 13.58, stage/render/render 12.34, render/graph/ms 10.54, encode//sum 8.99, stage/render/graph_submit 6.85, stage/render/prepare_assets 5.29, stream/drive/ms 5.03, stage/render/camera_driver 3.52 |
| warm-up | 6.86 | 32.4 | render/thread/ms 49.27, render/busy/ms 47.28, main/busy/ms 46.37, stage/main/post_update 20.01, stage/main/update 17.88, stage/render/render 17.27, render/graph/ms 14.63, stage/render/graph_submit 10.78, stage/render/prepare_assets 10.22, encode//sum 9.27, encode/submit 7.38, stream/drive/ms 6.01 |
| warm-up | 6.96 | 35.7 | main/busy/ms 46.57, render/thread/ms 35.53, render/busy/ms 34.12, stage/main/update 26.19, stream/drive/ms 15.25, pipelines/compile_ms 14.97, stage/main/post_update 14.71, pipelines/front_ms 14.64, stage/render/render 14.34, render/graph/ms 11.94, encode//sum 9.37, stage/render/graph_submit 8.46 |
| warm-up | 7.02 | 29.8 | main/busy/ms 42.58, render/thread/ms 39.55, render/busy/ms 38.25, stage/main/update 23.79, stage/render/prepare_assets 16.06, stage/render/render 13.90, stage/main/post_update 13.39, render/graph/ms 11.96, encode//sum 8.85, stage/render/graph_submit 8.67, stream/drive/ms 6.88, encode/submit 5.56 |
| warm-up | 7.38 | 36.9 | main/busy/ms 44.69, render/thread/ms 29.53, render/busy/ms 27.95, stage/main/update 26.05, stream/drive/ms 14.08, stage/render/prepare_assets 12.77, stage/main/post_update 11.22, stage/render/render 9.18, render/graph/ms 7.15, encode//sum 6.57, stage/render/graph_submit 4.99, encode/submit 3.08 |
| warm-up | 7.44 | 41.4 | render/thread/ms 48.16, render/busy/ms 47.11, main/busy/ms 28.91, stage/main/update 12.70, stage/render/specialize 12.52, stage/main/post_update 11.73, stage/render/render 9.94, stage/render/queue 8.66, render/graph/ms 8.36, encode//sum 7.34, stage/render/graph_submit 6.26, stage/render/prepare_assets 5.27 |
| warm-up | 8.69 | 29.3 | render/thread/ms 35.21, render/busy/ms 34.11, main/busy/ms 17.41, stage/render/render 10.45, stage/main/post_update 9.95, stage/render/specialize 8.96, render/graph/ms 8.93, stage/render/graph_submit 6.92, encode//sum 6.64, stage/render/queue 5.99, stage/main/update 4.84, encode/submit 4.23 |
| measure | 0.42 | 33.3 | render/thread/ms 36.06, render/busy/ms 34.38, main/busy/ms 17.74, stage/render/render 11.89, stage/main/post_update 11.34, render/graph/ms 9.75, stage/render/specialize 8.51, stage/render/graph_submit 7.81, encode//sum 7.04, stage/render/queue 5.76, encode/submit 4.97, stage/main/update 4.01 |
| measure | 1.70 | 32.9 | render/thread/ms 38.25, render/busy/ms 37.30, main/busy/ms 17.22, stage/render/render 11.93, stage/main/post_update 11.04, render/graph/ms 10.46, encode//sum 9.78, stage/render/specialize 8.96, stage/render/graph_submit 7.64, stage/render/queue 6.10, encode/submit 4.67, stage/render/prepare_meshes 4.62 |
| measure | 1.85 | 55.6 | render/thread/ms 61.09, render/busy/ms 60.06, stage/render/render 54.63, render/graph/ms 53.24, encode//sum 50.60, stage/render/graph_submit 50.54, encode/finish_wall 45.19, encode/pass:early prepass/sum 43.91, main/busy/ms 16.89, stage/main/post_update 10.90, encode/submit 5.38, encode/pass:shadow_point_light/sum 4.00 |
| measure | 15.73 | 30.0 | render/thread/ms 36.73, render/busy/ms 35.74, main/busy/ms 18.20, stage/main/post_update 11.31, stage/render/render 10.97, stage/render/specialize 9.72, render/graph/ms 9.56, encode//sum 7.82, stage/render/graph_submit 7.20, stage/render/queue 6.42, encode/submit 4.36, stage/main/update 4.31 |
| measure | 18.16 | 31.1 | main/busy/ms 38.64, stage/main/update 24.47, stream/drive/ms 20.10, render/thread/ms 19.56, render/busy/ms 18.59, stage/main/post_update 11.34, stage/render/render 11.15, render/graph/ms 9.75, encode//sum 7.96, stage/render/graph_submit 7.16, encode/submit 4.84, stage/render/camera_driver 2.47 |
| measure | 19.46 | 32.1 | render/thread/ms 41.35, render/busy/ms 40.30, main/busy/ms 20.68, stage/render/render 12.87, stage/main/post_update 12.16, render/graph/ms 11.31, encode//sum 10.01, stage/render/specialize 9.21, stage/render/graph_submit 8.18, stage/render/queue 6.77, encode/submit 5.68, stage/main/update 5.50 |
| measure | 20.79 | 31.1 | render/thread/ms 34.48, render/busy/ms 33.45, main/busy/ms 18.89, stage/main/post_update 11.42, stage/render/specialize 9.55, stage/render/render 9.43, render/graph/ms 7.95, encode//sum 7.17, stage/render/queue 6.78, stage/render/graph_submit 5.92, stage/main/update 4.71, encode/submit 3.49 |
| measure | 23.00 | 32.6 | render/thread/ms 38.48, render/busy/ms 37.41, main/busy/ms 23.73, stage/main/post_update 11.87, stage/render/specialize 11.59, stage/render/render 9.82, render/graph/ms 8.28, stage/render/queue 7.56, encode//sum 6.96, stage/main/update 6.54, stage/render/graph_submit 6.28, encode/submit 3.68 |
| measure | 24.47 | 30.5 | render/thread/ms 36.36, render/busy/ms 35.25, main/busy/ms 19.62, stage/main/post_update 11.95, stage/render/render 10.59, stage/render/specialize 9.65, render/graph/ms 9.09, encode//sum 8.71, stage/render/graph_submit 6.86, stage/render/queue 6.57, stage/main/update 4.58, encode/submit 4.07 |
| measure | 27.28 | 35.6 | render/thread/ms 42.54, render/busy/ms 41.31, main/busy/ms 26.88, stage/render/render 13.81, stage/main/post_update 13.48, render/graph/ms 12.18, stage/render/specialize 11.68, encode//sum 10.25, stage/render/graph_submit 9.21, stage/render/queue 7.38, encode/submit 6.42, stage/main/update 6.02 |
| measure | 28.69 | 29.5 | render/thread/ms 36.66, render/busy/ms 35.65, main/busy/ms 21.34, stage/main/post_update 11.48, stage/render/render 11.01, render/graph/ms 9.55, stage/render/specialize 9.36, encode//sum 7.64, stage/render/graph_submit 7.53, stage/render/queue 6.77, stage/main/update 4.61, encode/submit 4.08 |
| measure | 30.48 | 32.5 | render/thread/ms 40.00, render/busy/ms 38.78, main/busy/ms 21.24, stage/main/post_update 12.69, stage/render/render 12.16, render/graph/ms 10.50, stage/render/specialize 10.06, stage/render/graph_submit 7.93, encode//sum 7.70, stage/render/queue 7.19, stage/main/update 5.54, encode/submit 5.05 |
| measure | 35.35 | 30.4 | render/thread/ms 39.56, render/busy/ms 38.51, main/busy/ms 20.77, stage/main/post_update 12.79, stage/render/render 12.32, render/graph/ms 10.87, stage/render/specialize 10.08, encode//sum 8.29, stage/render/graph_submit 8.13, stage/render/queue 7.32, encode/submit 4.65, stage/main/update 4.46 |
| measure | 40.80 | 32.1 | render/thread/ms 38.56, render/busy/ms 37.51, main/busy/ms 19.54, stage/main/post_update 11.38, stage/render/render 10.13, stage/render/specialize 9.38, render/graph/ms 8.86, stage/render/graph_submit 6.97, stage/render/queue 6.96, encode//sum 6.59, stage/main/update 4.95, encode/submit 4.03 |
| measure | 45.64 | 30.9 | render/thread/ms 37.91, render/busy/ms 36.79, main/busy/ms 18.09, stage/main/post_update 10.29, stage/render/render 10.19, stage/render/specialize 9.86, render/graph/ms 8.68, encode//sum 6.84, stage/render/graph_submit 6.60, stage/render/queue 6.44, stage/main/update 4.64, stage/render/prepare_meshes 4.49 |
| measure | 59.03 | 43.3 | render/thread/ms 53.77, render/busy/ms 52.47, main/busy/ms 26.13, stage/render/render 16.27, render/graph/ms 14.54, stage/main/post_update 14.04, encode//sum 12.81, stage/render/specialize 11.51, stage/render/graph_submit 11.20, stage/render/queue 10.04, stage/main/update 7.79, stage/render/prepare_meshes 6.10 |
| measure | 59.10 | 42.0 | render/thread/ms 39.46, render/busy/ms 38.34, stage/render/prepare_assets 21.63, main/busy/ms 17.84, stage/main/post_update 9.84, stage/render/render 9.35, render/graph/ms 7.97, encode//sum 7.50, stage/render/graph_submit 5.92, stage/main/update 4.23, encode/submit 3.84, encode/finish_wall 2.15 |
| measure | 62.45 | 33.3 | render/thread/ms 35.80, render/busy/ms 34.90, main/busy/ms 21.64, stage/render/specialize 10.48, stage/main/post_update 9.98, stage/render/render 9.54, render/graph/ms 8.41, encode//sum 7.78, stage/render/queue 6.79, stage/render/graph_submit 6.31, stage/main/update 5.81, stage/main/pre_update 3.90 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set render.depth_only.skip_material=on --resolution 2560x1440 --bench-out target/perf\pt_nomat\balmora_walk@2560x1440_arm=nomat-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 11%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **77%** (min 44%) · 2625 MHz (min 2625) · memory 8101 MHz · 80.1 W · 75 °C (max 81) · VRAM 7002 MiB · throttled 100% of samples

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
- `render.depth_only.skip_material=on (command line: --set)`
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
