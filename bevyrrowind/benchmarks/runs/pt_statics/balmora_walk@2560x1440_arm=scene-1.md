# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6272 frames over 65.6 s · **median 10.34 ms (97 fps)** · mean 10.47 · p95 12.92 · p99 16.20 (1% low 62 fps) · worst 35.40 ms · 77 spikes over 1.5× median · 54.8 stutters/min (work 53.0/min)
- warm-up: 356 frames over 9.5 s, worst 2656.0 ms
- hitches (> 25.8 ms): **6** measured, 40 in warm-up
- timed GPU passes: 6.41 ms of a 10.47 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.34 ms, limit 11 ms
- **FAIL** frame_ms: 60 stutters (> 1.5x moving median of 60) in 1.09 min = 54.84/min, limit 1/min
- **FAIL** worst frame 35.40 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.779 | 2.038 | 2.072 | 0.87 | 0.171 |
| main_opaque_pass_3d | 1.385 | 1.588 | 1.949 | 0.87 | 0.063 |
| early prepass | 0.455 | 0.521 | 0.748 | 0.87 | 0.087 |
| bloom | 0.275 | 0.315 | 0.321 | 0.87 | 0.064 |
| atmosphere_luts | 0.266 | 0.304 | 0.367 | 0.87 | 0.003 |
| water_velocity | 0.244 | 0.280 | 0.347 | 0.87 | 0.002 |
| water_sspr | 0.183 | 0.210 | 0.211 | 0.87 | 0.002 |
| ssao | 0.164 | 0.188 | 0.193 | 0.87 | 0.007 |
| auxiliary_views | 0.151 | 0.174 | 0.605 | 0.87 | 0.166 |
| atmos_froxel_inject | 0.127 | 0.146 | 0.148 | 0.87 | 0.001 |
| clouds_trace | 0.122 | 0.140 | 0.233 | 0.87 | 0.001 |
| water_fft | 0.116 | 0.133 | 0.140 | 0.87 | 0.003 |
| upscaling | 0.103 | 0.118 | 0.127 | 0.87 | 0.004 |
| uniform_allocation | 0.103 | 0.118 | 0.168 | 0.87 | 0.021 |
| clouds_composite | 0.092 | 0.106 | 0.109 | 0.87 | 0.002 |
| gpu_instance_materialization | 0.083 | 0.095 | 0.100 | 0.87 | 0.004 |
| bin_unpacking | 0.082 | 0.093 | 0.132 | 0.87 | 0.017 |
| tonemapping | 0.074 | 0.085 | 0.088 | 0.87 | 0.003 |
| main_transmissive_pass_3d | 0.074 | 0.085 | 0.178 | 0.87 | 0.008 |
| clustering | 0.056 | 0.064 | 0.067 | 0.87 | 0.018 |
| early_mesh_preprocessing | 0.049 | 0.056 | 0.088 | 0.87 | 0.003 |
| render_sky | 0.045 | 0.052 | 0.053 | 0.87 | 0.002 |
| clouds_resolve | 0.042 | 0.049 | 0.049 | 0.87 | 0.001 |
| lightprobe_irradiance_map | 0.040 | 0.415 | 0.417 | 0.10 | 0.001 |
| surface_exposure | 0.035 | 0.040 | 0.041 | 0.87 | 0.001 |
| surface_trail | 0.034 | 0.057 | 0.057 | 0.60 | 0.002 |
| surface_exposure_far | 0.032 | 0.040 | 0.042 | 0.78 | 0.002 |
| clouds_shadow | 0.027 | 0.031 | 0.034 | 0.87 | 0.003 |
| atmos_froxel_integrate | 0.026 | 0.030 | 0.032 | 0.87 | 0.001 |
| water_snapshot | 0.021 | 0.024 | 0.028 | 0.87 | 0.002 |
| lightprobe_radiance_map | 0.021 | 0.220 | 0.230 | 0.10 | 0.004 |
| range_unpacking | 0.020 | 0.023 | 0.032 | 0.87 | 0.003 |
| water_mips | 0.019 | 0.022 | 0.024 | 0.87 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.017 | 0.037 | 0.87 | 0.006 |
| night_sky | 0.012 | 0.014 | 0.034 | 0.87 | 0.001 |
| clouds_fog_light | 0.010 | 0.012 | 0.012 | 0.87 | 0.002 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.87 | 0.003 |
| main_transparent_pass_3d | 0.008 | 0.009 | 0.020 | 0.87 | 0.085 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.87 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.164 | 11.495 | 29.35 | 57475.3 |
| render/busy/ms | 8.599 | 10.991 | 28.80 | 53931.1 |
| main/busy/ms | 8.546 | 10.143 | 30.10 | 53602.5 |
| stage/render/render | 5.411 | 6.981 | 25.95 | 33937.1 |
| stage/main/post_update | 5.015 | 6.277 | 8.64 | 31454.5 |
| render/graph/ms | 4.671 | 6.246 | 25.23 | 29297.8 |
| encode//sum | 3.954 | 5.272 | 23.71 | 24799.9 |
| stage/render/graph_submit | 3.492 | 4.774 | 23.96 | 21901.9 |
| encode/submit | 2.259 | 3.256 | 22.86 | 14166.9 |
| stage/main/update | 2.157 | 2.597 | 24.27 | 13527.6 |
| encode/finish_wall | 1.335 | 1.948 | 4.69 | 8376.0 |
| stage/render/camera_driver | 1.119 | 1.522 | 3.80 | 7018.0 |
| encode/pass:shadow_point_light/sum | 0.935 | 3.261 | 16.12 | 5866.1 |
| encode/pass:shadow_directional_light/sum | 0.830 | 1.521 | 2.76 | 5207.0 |
| encode/pass:main_opaque_pass_3d/sum | 0.757 | 1.065 | 1.79 | 4750.1 |
| encode/pass:early prepass/sum | 0.721 | 1.042 | 1.81 | 4522.7 |
| stage/render/prepare_write_phase_buffers | 0.640 | 0.963 | 2.28 | 4011.4 |
| bigspace/total/ms | 0.527 | 1.310 | 3.43 | 3307.6 |
| stage/render/prepare_assets | 0.526 | 0.679 | 12.38 | 3302.1 |
| render/present/ms | 0.471 | 1.258 | 2.36 | 2952.3 |
| stage/render/prepare_resources | 0.431 | 0.515 | 0.85 | 2705.4 |
| stage/render/prepare_meshes | 0.417 | 0.524 | 4.32 | 2616.9 |
| stage/main/first | 0.396 | 0.517 | 5.39 | 2483.0 |
| stage/main/pre_update | 0.347 | 0.500 | 2.90 | 2175.1 |
| stage/render/prepare_bind_groups | 0.319 | 0.551 | 2.08 | 2003.8 |
| terrain/update/ms | 0.310 | 0.401 | 2.22 | 1942.5 |
| stage/render/prepare_flush | 0.286 | 0.501 | 1.37 | 1791.9 |
| terrain/select/ms | 0.277 | 0.360 | 0.98 | 1734.6 |
| controller/feed/ms | 0.261 | 0.336 | 0.71 | 1635.2 |
| stage/render/specialize | 0.253 | 0.354 | 5.79 | 1588.6 |
| stream/drive/ms | 0.251 | 0.332 | 10.91 | 1577.3 |
| stage/main/last | 0.250 | 0.297 | 1.51 | 1565.8 |
| bigspace/low_precision/ms | 0.231 | 0.877 | 1.76 | 1451.3 |
| stage/render/prepare_batch_phases | 0.223 | 0.331 | 0.58 | 1397.9 |
| stage/main/fixed_main_loop | 0.205 | 0.360 | 1.27 | 1285.5 |
| stream/tick/ms | 0.198 | 0.270 | 0.58 | 1240.8 |
| far/select/ms | 0.195 | 0.261 | 0.50 | 1223.1 |
| encode/pass:main_transparent_pass_3d/sum | 0.192 | 0.292 | 1.08 | 1203.2 |
| stage/render/create_views | 0.183 | 0.234 | 8.18 | 1146.6 |
| stage/render/queue | 0.179 | 0.285 | 3.77 | 1122.0 |
| stage/render/extract_commands | 0.176 | 0.208 | 0.59 | 1102.7 |
| pipelines/compile_ms | 0.165 | 0.000 | 129.13 | 1036.1 |
| pipelines/front_ms | 0.160 | 0.000 | 126.39 | 1006.2 |
| anim/evaluate/ms | 0.159 | 0.268 | 2.41 | 994.2 |
| bigspace/high_precision/ms | 0.156 | 0.694 | 3.13 | 979.1 |
| anim/evaluate_cpu/ms | 0.149 | 0.189 | 0.78 | 931.4 |
| stage/render/prepare_views | 0.140 | 0.174 | 0.40 | 879.0 |
| controller/move/ms | 0.127 | 0.288 | 0.67 | 798.4 |
| encode/pass:main_transmissive_pass_3d/sum | 0.096 | 0.158 | 0.76 | 601.5 |
| render/acquire/ms | 0.094 | 0.121 | 0.35 | 591.9 |
| stage/render/phase_sort | 0.079 | 0.108 | 0.31 | 495.3 |
| physics/step/ms | 0.046 | 0.087 | 0.56 | 289.9 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.072 | 0.28 | 286.4 |
| physics/insert/ms | 0.031 | 0.332 | 0.92 | 195.2 |
| encode/resolve_diagnostics/sum | 0.031 | 0.041 | 0.57 | 192.2 |
| weather/map/ms | 0.025 | 0.033 | 0.23 | 156.4 |
| stage/render/cleanup | 0.024 | 0.035 | 0.89 | 150.1 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.56 | 79.5 |
| anim/apply/ms | 0.011 | 0.020 | 0.21 | 70.6 |
| weather/eval/ms | 0.010 | 0.014 | 0.24 | 65.1 |
| controller/avoid/ms | 0.010 | 0.014 | 0.10 | 63.8 |
| encode/camera_driver_span_end/sum | 0.010 | 0.018 | 0.33 | 62.5 |
| script/update/ms | 0.007 | 0.009 | 0.19 | 44.3 |
| script/input/ms | 0.005 | 0.006 | 0.06 | 32.8 |
| script/ui/ms | 0.005 | 0.007 | 0.07 | 31.4 |
| stream/despawn/ms | 0.005 | 0.000 | 1.04 | 30.2 |
| pipelines/driver_ms | 0.005 | 0.000 | 3.77 | 29.9 |
| controller/write/ms | 0.003 | 0.004 | 0.06 | 21.7 |
| stream/upload/ms | 0.003 | 0.000 | 1.40 | 21.4 |
| stream/spawn/ms | 0.003 | 0.000 | 1.09 | 20.3 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.62 | 13.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 6.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.17 | 3.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 15 |
| controller/movers/count | count | 20 | 22 | 15 |
| crowd/c0/count | count | 27 | 42 | 14 |
| crowd/c0/joints/count | count | 1281 | 2218 | 641 |
| crowd/c0/meshes/count | count | 355 | 634 | 174 |
| crowd/c1/count | count | 145 | 154 | 147 |
| crowd/c1/joints/count | count | 5531 | 5913 | 5555 |
| crowd/c1/meshes/count | count | 1166 | 1243 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4601 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4584 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 48.7 k | 56.8 k | 47.0 k |
| encode//max | ms | 0.40 | 1.64 | 0.50 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.56 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.33 | 0.01 |
| encode/finish_critical | ms | 0.95 | 2.76 | 0.84 |
| encode/pass:early prepass/max | ms | 0.73 | 1.81 | 0.65 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.77 | 1.79 | 0.84 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.76 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.18 | 1.08 | 0.20 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 2.76 | 0.72 |
| encode/pass:shadow_point_light/max | ms | 0.77 | 1.67 | 0.87 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.57 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 8.81 | 30.10 | 8.35 |
| gpu/timed/ms | ms | 8.05 | 9.23 | 8.35 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 299.7 k | 299.7 k | 299.7 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 11.9 k | 13.1 k | 11.8 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 158 | 388 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 336 | 588 | 516 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 158 | 388 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 336 | 588 | 516 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 21 | 23 | 22 |
| phases/opaque3d/bins | count | 764 | 971 | 761 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2528 | 3561 | 2454 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 18 |
| phases/opaque3d_prepass/bins | count | 764 | 971 | 761 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2528 | 3561 | 2454 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 23 | 194 | 8 |
| phases/shadow/bins | count | 1908 | 3166 | 1094 |
| phases/shadow/instance_batches | count | 4 | 12 | 4 |
| phases/shadow/instance_draws | count | 6580 | 20.3 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| phases/shadow/items | count | 5910 | 12.5 k | 4394 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 161 | 221 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2344 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 198 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 327 | 328 | 328 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 17 | 530 | 11 |
| preprocess/bind_groups_created | count | 12 | 523 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 214 | 119 |
| process/cpu_usage | % | 26.44 | 39.23 | 29.33 |
| process/mem_usage | GiB | 5.56 | 5.80 | 5.80 |
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
| skin/blocks_changed/count | count | 14 | 129 | 9 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 369.4 k | 810.8 k | 486.1 k |
| skin/joints_written/count | count | 5772 | 12.7 k | 7596 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 6449 | 6573 | 6573 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.18 | 41.69 | 39.25 |
| system/mem_usage | % | 67.45 | 68.26 | 68.16 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2895 | 3010 | 3010 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 78 | 249 | 64 |
| views/all/bins | count | 3886 | 5145 | 3376 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 12 | 20 | 12 |
| views/all/instance_draws | count | 19.7 k | 33.8 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 147.5 k |
| views/all/items | count | 11.7 k | 19.1 k | 10.3 k |
| views/all/pipelines | count | 206 | 268 | 75 |
| views/all/pipelines_distinct | count | 68 | 73 | 59 |
| views/all/work_items | count | 154.0 k | 264.4 k | 153.8 k |
| views/cascade/batch_sets | count | 12 | 62 | 8 |
| views/cascade/bins | count | 1516 | 2510 | 1094 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 4 |
| views/cascade/instance_draws | count | 6580 | 20.3 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| views/cascade/items | count | 5235 | 11.5 k | 4394 |
| views/cascade/pipelines | count | 38 | 55 | 31 |
| views/cascade/work_items | count | 49.4 k | 159.0 k | 49.3 k |
| views/lamp_face/batch_sets | count | 0 | 146 | 0 |
| views/lamp_face/bins | count | 399 | 800 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 627 | 1113 | 0 |
| views/lamp_face/pipelines | count | 123 | 174 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 730 | 0 |
| views/main/batch_sets | count | 56 | 61 | 56 |
| views/main/bins | count | 2016 | 2320 | 2282 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5960 | 7542 | 5940 |
| views/main/pipelines | count | 45 | 49 | 44 |
| views/main/work_items | count | 104.5 k | 106.0 k | 104.5 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 20.6 k | 26.4 k | 18.5 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 56.31 | 238 | 353.2 k |
| anim/events/count | count | 0.08 | 4 | 533 |
| assets/Image/added | count | 0.09 | 35 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 19 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.46 | 243 | 2874 |
| assets/Mesh/removed | count | 0.77 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 471 |
| assets/StandardMaterial/modified | count | 3.46 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 9 |
| ecs/entities_delta | count | 0.24 | 1330 | 1490 |
| encode//count | count | 35.27 | 223 | 221.2 k |
| encode/bind_walk/init_textures | count | 12.9 k | 118.4 k | 81.21 M |
| encode/bind_walk/init_textures_skipped | count | 73.7 k | 166.1 k | 462.55 M |
| encode/bind_walk/inits | count | 452 | 1098 | 2.84 M |
| encode/bind_walk/inits_skipped | count | 99.12 | 164 | 621.7 k |
| encode/bind_walk/merge_textures | count | 12.9 k | 118.3 k | 80.75 M |
| encode/bind_walk/merge_textures_skipped | count | 5236 | 5983 | 32.84 M |
| encode/bind_walk/merges | count | 97.53 | 348 | 611.7 k |
| encode/bind_walk/merges_skipped | count | 71.24 | 134 | 446.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6272 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6272 |
| encode/pass:early prepass/count | count | 1 | 1 | 6272 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6272 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6272 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6272 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6272 |
| encode/pass:shadow_point_light/count | count | 1.58 | 48 | 9932 |
| encode/queue/command_buffers | count | 46.86 | 281 | 293.9 k |
| encode/queue/submits | count | 2.01 | 4 | 12.6 k |
| encode/queue/write_buffer/bytes | bytes | 2.70 M | 5.09 M | 16.96 G |
| encode/queue/write_buffer/calls | count | 267 | 797 | 1.67 M |
| encode/queue/write_buffer_with/bytes | bytes | 173.8 k | 1.30 M | 1.09 G |
| encode/queue/write_buffer_with/calls | count | 10.36 | 738 | 65.0 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 72.50 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 565 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6272 |
| physics/inserts | count | 0.36 | 109 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.37 | 145 | 2335 |
| physics/steps | count | 0.63 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 10 |
| pipelines/created | count | 0.00 | 2 | 13 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 2 | 6 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5397 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 30 |
| surface/shelter_redraws | count | 0.01 | 1 | 55 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1183 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.8 ms; slowest 46, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.66 | 2656.0 | main/busy/ms 2625, stage/main/post_update 2515, stage/main/update 102, surface/advance_cpu_ms 68.24, stage/main/pre_update 4.65, stage/main/last 1.02, stream/drive/ms 1.01, surface/cell_upload_ms 0.77, stage/main/first 0.52, stage/main/fixed_main_loop 0.50, stream/tick/ms 0.22, weather/map/ms 0.16 |
| warm-up | 3.49 | 834.6 | main/busy/ms 2807, stage/main/post_update 2539, render/thread/ms 801, render/busy/ms 631, stage/render/prepare_assets 582, stage/main/update 253, stage/render/prepare_views 166, render/acquire/ms 166, surface/advance_cpu_ms 68.24, stage/render/create_views 28.21, stage/render/render 13.80, render/graph/ms 9.40 |
| warm-up | 3.61 | 118.9 | render/thread/ms 899, render/busy/ms 674, stage/render/prepare_assets 609, main/busy/ms 204, stage/render/prepare_views 167, render/acquire/ms 167, stage/main/update 155, pipelines/compile_ms 126, pipelines/front_ms 81.66, stage/render/render 74.20, render/present/ms 57.37, pipelines/driver_ms 43.85 |
| warm-up | 3.65 | 39.6 | pipelines/compile_ms 454, pipelines/front_ms 372, render/thread/ms 136, pipelines/wait_ms 114, stage/render/render 90.57, pipelines/driver_ms 82.20, render/busy/ms 76.21, main/busy/ms 60.93, render/present/ms 58.27, stage/main/post_update 45.15, render/graph/ms 30.90, stage/render/prepare_assets 27.92 |
| warm-up | 3.78 | 127.6 | pipelines/wait_ms 4771, pipelines/compile_ms 1696, pipelines/front_ms 1550, render/thread/ms 162, render/busy/ms 156, pipelines/driver_ms 146, main/busy/ms 73.82, stage/main/post_update 59.44, stage/render/prepare_resources 53.04, stage/render/prepare_flush 38.07, stage/render/render 36.04, render/graph/ms 29.76 |
| warm-up | 3.82 | 43.8 | pipelines/wait_ms 5260, pipelines/compile_ms 1871, pipelines/front_ms 1725, render/thread/ms 165, render/busy/ms 162, pipelines/driver_ms 146, main/busy/ms 102, encode//sum 64.47, stage/render/prepare_resources 64.12, stage/main/update 55.68, stage/main/post_update 38.30, stage/render/prepare_flush 37.89 |
| warm-up | 3.92 | 101.1 | pipelines/compile_ms 930, pipelines/front_ms 870, pipelines/wait_ms 604, render/thread/ms 110, render/busy/ms 108, main/busy/ms 83.98, encode//sum 65.58, pipelines/driver_ms 59.55, stage/main/update 58.58, stage/render/prepare_assets 52.82, stage/render/render 31.89, render/graph/ms 29.29 |
| warm-up | 3.97 | 27.9 | pipelines/compile_ms 589, pipelines/front_ms 569, pipelines/wait_ms 186, render/thread/ms 49.19, render/busy/ms 47.94, main/busy/ms 27.78, stage/render/prepare_assets 25.32, pipelines/driver_ms 20.15, stage/main/post_update 14.77, terrain/update/ms 9.42, stage/render/render 8.42, stage/main/update 7.54 |
| warm-up | 4.03 | 33.5 | pipelines/compile_ms 1120, pipelines/front_ms 1075, pipelines/wait_ms 176, pipelines/driver_ms 44.73, render/thread/ms 43.65, render/busy/ms 41.81, main/busy/ms 38.43, stage/main/post_update 24.59, stage/render/render 13.45, render/graph/ms 11.00, stage/render/prepare_write_phase_buffers 9.21, terrain/update/ms 8.62 |
| warm-up | 4.05 | 28.3 | pipelines/compile_ms 911, pipelines/front_ms 870, pipelines/wait_ms 165, main/busy/ms 80.47, stage/main/update 57.25, render/thread/ms 53.99, render/busy/ms 51.97, pipelines/driver_ms 40.14, stage/main/post_update 17.65, stage/render/render 16.22, render/graph/ms 13.74, stage/render/prepare_bind_groups 12.96 |
| warm-up | 4.17 | 119.1 | pipelines/compile_ms 1306, pipelines/front_ms 1247, render/thread/ms 80.78, main/busy/ms 78.89, render/busy/ms 78.72, stage/main/update 60.45, pipelines/driver_ms 59.07, stage/render/prepare_assets 43.50, stage/render/render 19.22, render/graph/ms 16.85, stage/render/graph_submit 14.01, stage/main/post_update 11.49 |
| warm-up | 4.22 | 45.9 | pipelines/compile_ms 1826, pipelines/front_ms 1756, render/thread/ms 78.73, render/busy/ms 76.91, pipelines/driver_ms 70.01, stage/render/prepare_assets 56.22, main/busy/ms 24.26, pipelines/wait_ms 15.63, stage/render/render 15.17, pipelines/lock_ms 13.84, render/graph/ms 13.14, stage/render/graph_submit 10.72 |
| warm-up | 4.32 | 30.2 | main/busy/ms 40.65, render/thread/ms 37.66, render/busy/ms 36.05, pipelines/compile_ms 30.76, pipelines/front_ms 29.35, stage/main/update 25.61, stage/render/render 14.37, render/graph/ms 12.40, stream/drive/ms 12.27, stage/render/graph_submit 9.33, stage/render/prepare_assets 8.62, stage/main/post_update 7.81 |
| warm-up | 4.37 | 48.0 | pipelines/compile_ms 581, pipelines/front_ms 562, render/thread/ms 60.83, render/busy/ms 58.91, main/busy/ms 56.99, stage/main/update 37.90, stage/render/prepare_assets 23.06, stream/drive/ms 22.90, pipelines/driver_ms 18.88, stage/render/render 17.57, render/graph/ms 15.17, stage/main/post_update 10.22 |
| warm-up | 4.41 | 36.6 | pipelines/compile_ms 887, pipelines/front_ms 854, render/thread/ms 67.11, render/busy/ms 64.99, main/busy/ms 64.91, stage/main/update 37.35, pipelines/driver_ms 33.21, stream/drive/ms 25.61, stage/render/prepare_assets 25.25, stage/render/render 21.55, render/graph/ms 18.79, stage/main/post_update 15.50 |
| warm-up | 4.48 | 71.3 | pipelines/compile_ms 1470, pipelines/front_ms 1417, render/thread/ms 77.00, render/busy/ms 74.86, main/busy/ms 56.70, pipelines/driver_ms 53.50, stage/render/prepare_assets 37.75, stage/main/update 26.53, stage/render/render 18.89, render/graph/ms 15.88, stage/main/post_update 14.57, pipelines/wait_ms 14.42 |
| warm-up | 4.55 | 68.7 | pipelines/compile_ms 1232, pipelines/front_ms 1190, main/busy/ms 175, stage/main/update 154, render/thread/ms 92.94, render/busy/ms 90.96, stage/render/prepare_assets 63.00, pipelines/driver_ms 41.64, stage/render/render 15.11, render/graph/ms 12.43, stage/main/post_update 9.53, stage/render/graph_submit 8.75 |
| warm-up | 4.78 | 228.6 | pipelines/compile_ms 2569, pipelines/front_ms 2381, pipelines/driver_ms 188, main/busy/ms 167, stage/main/update 148, render/thread/ms 134, render/busy/ms 132, stage/render/prepare_assets 115, stage/render/render 10.87, pipelines/wait_ms 9.73, render/graph/ms 9.06, stage/main/post_update 8.90 |
| warm-up | 4.83 | 54.0 | pipelines/compile_ms 2647, pipelines/front_ms 2443, pipelines/driver_ms 204, render/thread/ms 142, render/busy/ms 140, stage/render/prepare_assets 83.73, main/busy/ms 81.57, stage/main/update 43.31, stage/render/render 33.92, render/graph/ms 29.36, stage/main/post_update 27.22, stage/render/graph_submit 23.83 |
| warm-up | 4.90 | 68.7 | pipelines/compile_ms 190, pipelines/front_ms 170, render/thread/ms 95.52, render/busy/ms 92.72, main/busy/ms 87.41, stage/main/update 50.88, stage/render/render 34.86, pipelines/wait_ms 31.07, render/graph/ms 29.99, stage/main/post_update 26.59, stage/render/graph_submit 24.10, stage/render/prepare_assets 23.38 |
| warm-up | 4.99 | 95.8 | pipelines/compile_ms 902, pipelines/front_ms 873, render/thread/ms 129, render/busy/ms 127, main/busy/ms 56.50, stage/render/prepare_resources 43.13, stage/render/prepare_assets 41.64, stage/main/update 36.94, pipelines/wait_ms 30.29, pipelines/driver_ms 28.97, stage/render/render 13.33, stage/main/post_update 11.52 |
| warm-up | 5.05 | 52.1 | pipelines/compile_ms 2786, pipelines/front_ms 2709, render/thread/ms 134, render/busy/ms 131, pipelines/wait_ms 103, pipelines/driver_ms 76.44, main/busy/ms 66.50, stage/render/prepare_resources 48.24, stage/main/update 41.94, stage/render/render 23.15, stage/render/prepare_assets 22.45, render/graph/ms 20.72 |
| warm-up | 5.12 | 77.9 | pipelines/compile_ms 1983, pipelines/front_ms 1932, render/thread/ms 120, render/busy/ms 117, pipelines/wait_ms 104, main/busy/ms 72.69, stage/render/prepare_assets 54.25, pipelines/driver_ms 51.50, stage/main/update 39.92, stage/render/render 24.48, render/graph/ms 22.02, encode//sum 21.59 |
| warm-up | 5.20 | 76.0 | pipelines/compile_ms 842, pipelines/front_ms 821, render/thread/ms 130, render/busy/ms 128, main/busy/ms 91.69, stage/render/prepare_assets 71.38, stage/main/update 54.29, pipelines/wait_ms 35.78, stage/render/render 28.45, render/graph/ms 25.84, stage/main/post_update 22.39, pipelines/driver_ms 21.44 |
| warm-up | 5.32 | 120.4 | pipelines/compile_ms 2352, pipelines/front_ms 2290, render/thread/ms 147, render/busy/ms 145, main/busy/ms 141, stage/main/update 97.25, pipelines/wait_ms 81.64, pipelines/driver_ms 61.99, stream/drive/ms 57.82, stage/render/prepare_assets 52.88, stage/render/render 43.12, render/graph/ms 40.28 |
| warm-up | 5.37 | 46.5 | pipelines/compile_ms 1804, pipelines/front_ms 1752, render/thread/ms 130, render/busy/ms 128, main/busy/ms 123, stage/main/update 73.96, stream/drive/ms 57.90, pipelines/wait_ms 53.71, pipelines/driver_ms 52.67, stage/render/render 40.73, stage/render/prepare_assets 39.65, stage/main/post_update 38.75 |
| warm-up | 5.40 | 30.5 | pipelines/compile_ms 283, pipelines/front_ms 270, render/thread/ms 64.44, render/busy/ms 62.44, main/busy/ms 56.07, stage/main/update 24.06, stage/main/post_update 23.95, stage/render/render 22.59, encode//sum 20.64, render/graph/ms 20.29, stage/render/graph_submit 16.22, stage/render/prepare_assets 14.05 |
| warm-up | 5.43 | 35.0 | pipelines/compile_ms 125, pipelines/front_ms 119, render/thread/ms 54.78, render/busy/ms 53.31, main/busy/ms 38.74, stage/main/update 22.11, stage/render/render 12.99, stage/render/prepare_assets 12.57, render/graph/ms 10.99, stage/main/post_update 10.10, encode//sum 8.48, stage/render/graph_submit 8.25 |
| warm-up | 5.51 | 27.2 | pipelines/compile_ms 755, pipelines/front_ms 730, render/thread/ms 46.72, render/busy/ms 45.14, main/busy/ms 41.01, pipelines/driver_ms 25.31, stage/main/post_update 19.31, stage/main/update 16.55, stage/render/render 13.68, render/graph/ms 11.53, encode//sum 8.43, stage/render/prepare_assets 7.94 |
| warm-up | 5.53 | 28.0 | pipelines/compile_ms 719, pipelines/front_ms 698, render/thread/ms 47.00, render/busy/ms 45.26, main/busy/ms 40.75, pipelines/driver_ms 21.66, stage/main/post_update 20.17, stage/render/render 15.44, stage/main/update 15.29, render/graph/ms 13.12, stage/render/prepare_assets 11.18, stage/render/graph_submit 9.01 |
| warm-up | 5.66 | 34.5 | pipelines/compile_ms 922, pipelines/front_ms 886, render/thread/ms 46.01, render/busy/ms 44.31, main/busy/ms 43.23, pipelines/driver_ms 36.14, stage/main/update 21.94, stage/render/prepare_assets 17.91, stage/render/render 15.59, stage/main/post_update 14.31, stream/drive/ms 13.73, render/graph/ms 13.43 |
| warm-up | 6.26 | 28.2 | render/thread/ms 40.77, render/busy/ms 39.06, main/busy/ms 30.01, pipelines/compile_ms 22.02, pipelines/front_ms 20.13, stage/render/render 15.81, stage/main/update 13.30, render/graph/ms 13.11, stage/main/post_update 13.09, encode//sum 11.60, stage/render/prepare_assets 11.38, stage/render/graph_submit 9.19 |
| warm-up | 6.31 | 30.1 | render/thread/ms 35.57, render/busy/ms 34.55, main/busy/ms 21.92, stage/render/prepare_assets 19.81, stage/main/update 8.69, pipelines/compile_ms 8.22, stage/main/post_update 8.10, stage/render/render 7.82, pipelines/front_ms 7.64, encode//sum 7.47, render/graph/ms 6.28, stage/render/graph_submit 4.36 |
| warm-up | 6.40 | 26.1 | main/busy/ms 40.18, render/thread/ms 39.01, render/busy/ms 37.71, stage/main/update 25.20, stage/render/render 13.14, render/graph/ms 11.46, stage/render/prepare_assets 10.90, stage/main/post_update 10.44, stage/render/graph_submit 8.89, encode//sum 8.54, encode/submit 5.91, stage/render/prepare_meshes 5.42 |
| warm-up | 6.53 | 30.4 | render/thread/ms 38.80, main/busy/ms 37.86, render/busy/ms 37.12, stage/main/update 19.22, stage/render/render 14.64, stage/main/post_update 13.71, render/graph/ms 12.28, encode//sum 9.62, stage/render/prepare_assets 8.74, stage/render/graph_submit 8.58, encode/submit 5.59, stage/render/prepare_meshes 4.81 |
| warm-up | 6.56 | 32.7 | render/thread/ms 44.51, render/busy/ms 43.55, main/busy/ms 32.89, stage/render/prepare_assets 18.67, stage/main/update 18.14, stage/main/post_update 8.83, stage/render/render 8.19, encode//sum 7.54, render/graph/ms 6.67, stage/render/prepare_meshes 6.55, stage/render/graph_submit 4.28, stream/drive/ms 3.78 |
| warm-up | 6.62 | 33.8 | render/thread/ms 49.29, render/busy/ms 47.76, main/busy/ms 36.17, stage/render/render 17.30, stage/main/update 15.61, stage/main/post_update 15.44, render/graph/ms 14.98, encode//sum 10.86, stage/render/graph_submit 10.81, stage/render/prepare_assets 9.90, encode/submit 6.77, stage/render/prepare_meshes 6.76 |
| warm-up | 6.67 | 30.8 | pipelines/compile_ms 228, pipelines/front_ms 221, render/thread/ms 35.56, render/busy/ms 34.63, main/busy/ms 29.88, stage/render/prepare_assets 17.41, stage/main/update 16.88, stage/main/post_update 8.69, stage/render/render 7.71, pipelines/driver_ms 6.88, encode//sum 6.67, render/graph/ms 6.33 |
| warm-up | 6.78 | 37.2 | main/busy/ms 43.83, render/thread/ms 29.69, render/busy/ms 28.52, stage/main/update 27.61, stream/drive/ms 13.38, stage/render/render 12.96, stage/main/post_update 11.89, render/graph/ms 10.95, encode//sum 9.30, stage/render/graph_submit 8.08, stage/render/prepare_assets 6.08, encode/submit 4.89 |
| warm-up | 6.83 | 30.2 | pipelines/compile_ms 144, pipelines/front_ms 140, main/busy/ms 39.09, render/thread/ms 38.92, render/busy/ms 37.70, stage/main/update 23.93, stage/render/prepare_assets 15.63, stage/render/render 10.92, stage/main/post_update 10.02, render/graph/ms 8.98, encode//sum 8.11, stage/render/graph_submit 6.31 |
| measure | 0.19 | 30.2 | render/thread/ms 36.69, render/busy/ms 35.58, stage/render/render 30.16, render/graph/ms 28.86, stage/render/graph_submit 26.38, encode/submit 24.06, main/busy/ms 15.74, stage/main/post_update 9.17, encode//sum 6.69, stage/main/update 4.13, encode/pass:shadow_point_light/sum 2.89, encode/finish_wall 2.36 |
| measure | 8.13 | 30.2 | render/thread/ms 37.59, render/busy/ms 36.58, encode//sum 23.37, stage/render/render 19.26, render/graph/ms 17.57, main/busy/ms 16.27, encode/pass:shadow_point_light/sum 16.12, stage/render/graph_submit 12.62, stage/main/post_update 9.68, stage/render/create_views 8.36, encode/submit 7.52, encode/finish_wall 5.12 |
| measure | 18.16 | 33.1 | main/busy/ms 39.77, stage/main/update 25.89, render/thread/ms 20.50, render/busy/ms 19.49, stage/main/post_update 10.92, stage/render/render 10.26, render/graph/ms 8.91, encode//sum 7.13, stage/render/graph_submit 6.49, encode/submit 4.44, stage/render/prepare_assets 3.94, encode/pass:shadow_point_light/sum 2.87 |
| measure | 23.08 | 27.4 | render/thread/ms 38.69, render/busy/ms 37.66, main/busy/ms 30.04, encode//sum 18.17, stage/render/render 15.66, render/graph/ms 14.06, stage/main/post_update 13.88, stage/main/update 12.02, encode/pass:shadow_point_light/sum 10.31, stage/render/graph_submit 9.92, stage/render/prepare_assets 8.64, encode/submit 5.21 |
| measure | 59.00 | 32.3 | render/thread/ms 31.12, render/busy/ms 29.73, main/busy/ms 23.10, stage/render/render 13.46, stage/main/post_update 12.98, render/graph/ms 11.72, encode//sum 11.43, stage/render/prepare_assets 9.23, stage/render/graph_submit 8.61, encode/submit 5.37, stage/main/update 5.30, encode/finish_wall 3.36 |
| measure | 59.04 | 35.4 | render/thread/ms 35.60, render/busy/ms 34.55, stage/render/prepare_assets 20.20, main/busy/ms 18.91, stage/main/post_update 9.79, stage/render/render 8.98, encode//sum 8.62, render/graph/ms 7.65, stage/render/graph_submit 5.55, stage/main/update 4.47, encode/submit 3.61, encode/finish_wall 1.98 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=scene-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 4%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **82%** (min 61%) · 2619 MHz (min 2610) · memory 8101 MHz · 82.7 W · 78 °C (max 83) · VRAM 7019 MiB · throttled 100% of samples

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
