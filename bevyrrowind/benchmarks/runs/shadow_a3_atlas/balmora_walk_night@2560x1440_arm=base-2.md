# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6034 frames over 65.6 s · **median 10.63 ms (94 fps)** · mean 10.88 · p95 13.36 · p99 15.66 (1% low 64 fps) · worst 36.42 ms · 53 spikes over 1.5× median · 36.6 stutters/min (work 29.2/min)
- warm-up: 352 frames over 9.5 s, worst 2547.1 ms
- hitches (> 26.6 ms): **2** measured, 47 in warm-up
- timed GPU passes: 6.83 ms of a 10.88 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.63 ms, limit 11 ms
- **FAIL** frame_ms: 40 stutters (> 1.5x moving median of 60) in 1.09 min = 36.56/min, limit 1/min
- **FAIL** worst frame 36.42 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.035 | 2.167 | 2.505 | 0.94 | 0.179 |
| main_opaque_pass_3d | 1.276 | 1.358 | 1.863 | 0.94 | 0.071 |
| early prepass | 0.500 | 0.532 | 0.763 | 0.94 | 0.093 |
| bloom | 0.323 | 0.344 | 0.672 | 0.94 | 0.066 |
| atmosphere_luts | 0.308 | 0.328 | 0.468 | 0.94 | 0.003 |
| auxiliary_views | 0.295 | 0.314 | 1.115 | 0.94 | 0.268 |
| water_velocity | 0.225 | 0.239 | 0.248 | 0.94 | 0.002 |
| water_sspr | 0.203 | 0.216 | 0.346 | 0.94 | 0.002 |
| ssao | 0.186 | 0.198 | 0.204 | 0.94 | 0.008 |
| atmos_froxel_inject | 0.128 | 0.136 | 0.130 | 0.94 | 0.002 |
| upscaling | 0.126 | 0.134 | 0.152 | 0.94 | 0.004 |
| water_fft | 0.120 | 0.128 | 0.127 | 0.94 | 0.003 |
| uniform_allocation | 0.115 | 0.123 | 0.144 | 0.94 | 0.024 |
| clouds_composite | 0.100 | 0.107 | 0.109 | 0.94 | 0.002 |
| bin_unpacking | 0.090 | 0.096 | 0.116 | 0.94 | 0.018 |
| tonemapping | 0.086 | 0.092 | 0.095 | 0.94 | 0.004 |
| gpu_instance_materialization | 0.086 | 0.091 | 0.095 | 0.94 | 0.004 |
| main_transmissive_pass_3d | 0.073 | 0.077 | 0.150 | 0.94 | 0.009 |
| clustering | 0.063 | 0.067 | 0.068 | 0.94 | 0.020 |
| early_mesh_preprocessing | 0.053 | 0.056 | 0.072 | 0.94 | 0.004 |
| clouds_trace | 0.052 | 0.056 | 0.078 | 0.94 | 0.001 |
| render_sky | 0.051 | 0.055 | 0.054 | 0.94 | 0.002 |
| clouds_resolve | 0.047 | 0.050 | 0.051 | 0.94 | 0.001 |
| lightprobe_irradiance_map | 0.044 | 0.469 | 0.761 | 0.09 | 0.001 |
| surface_trail | 0.042 | 0.062 | 0.057 | 0.67 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.94 | 0.001 |
| water_snapshot | 0.025 | 0.026 | 0.035 | 0.94 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.94 | 0.002 |
| range_unpacking | 0.021 | 0.023 | 0.029 | 0.94 | 0.003 |
| lightprobe_radiance_map | 0.020 | 0.220 | 0.230 | 0.09 | 0.005 |
| per_view_shadow_pass | 0.018 | 0.019 | 0.027 | 0.94 | 0.005 |
| main_transparent_pass_3d | 0.017 | 0.018 | 0.032 | 0.94 | 0.197 |
| night_sky | 0.016 | 0.017 | 0.043 | 0.94 | 0.002 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.94 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.94 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.94 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.94 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.667 | 12.063 | 22.06 | 58329.8 |
| render/busy/ms | 9.134 | 11.547 | 20.60 | 55113.0 |
| main/busy/ms | 8.643 | 10.259 | 31.92 | 52152.8 |
| stage/render/render | 5.857 | 7.762 | 17.41 | 35344.0 |
| render/graph/ms | 5.140 | 7.015 | 12.47 | 31017.7 |
| stage/main/post_update | 5.025 | 6.153 | 9.56 | 30318.3 |
| encode//sum | 4.456 | 6.528 | 25.50 | 26888.1 |
| stage/render/graph_submit | 3.662 | 5.006 | 9.72 | 22097.3 |
| encode/submit | 2.390 | 3.412 | 6.66 | 14418.5 |
| stage/main/update | 1.875 | 2.408 | 24.90 | 11316.7 |
| stage/render/camera_driver | 1.413 | 1.997 | 4.24 | 8525.7 |
| encode/finish_wall | 1.350 | 1.782 | 4.06 | 8144.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.830 | 1.121 | 1.86 | 5009.6 |
| encode/pass:shadow_directional_light/sum | 0.828 | 1.268 | 1.85 | 4997.2 |
| encode/pass:early prepass/sum | 0.820 | 1.111 | 1.57 | 4948.2 |
| stage/render/prepare_write_phase_buffers | 0.679 | 1.023 | 1.86 | 4094.6 |
| stage/render/prepare_assets | 0.534 | 0.664 | 12.27 | 3223.7 |
| stage/main/fixed_main_loop | 0.532 | 0.899 | 3.39 | 3213.1 |
| bigspace/total/ms | 0.514 | 1.233 | 2.65 | 3103.6 |
| stage/render/prepare_meshes | 0.450 | 0.577 | 4.37 | 2716.6 |
| stage/render/prepare_resources | 0.450 | 0.552 | 3.92 | 2714.0 |
| render/present/ms | 0.442 | 1.213 | 11.44 | 2667.0 |
| encode/pass:shadow_point_light/sum | 0.402 | 2.071 | 4.77 | 2425.5 |
| stage/main/first | 0.395 | 0.514 | 5.70 | 2382.0 |
| stage/main/pre_update | 0.382 | 0.532 | 3.49 | 2308.0 |
| controller/feed/ms | 0.325 | 0.421 | 0.83 | 1963.1 |
| stage/render/prepare_bind_groups | 0.321 | 0.538 | 2.32 | 1934.7 |
| stage/render/prepare_flush | 0.292 | 0.506 | 4.48 | 1761.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.284 | 0.449 | 0.84 | 1713.9 |
| stage/render/prepare_batch_phases | 0.281 | 0.422 | 0.92 | 1697.1 |
| stage/main/last | 0.251 | 0.302 | 1.19 | 1514.0 |
| bigspace/low_precision/ms | 0.225 | 0.692 | 2.14 | 1356.8 |
| stream/drive/ms | 0.210 | 0.302 | 11.01 | 1264.3 |
| stage/render/create_views | 0.188 | 0.243 | 0.91 | 1131.9 |
| stage/render/extract_commands | 0.186 | 0.223 | 0.56 | 1124.7 |
| stage/render/specialize | 0.184 | 0.272 | 2.46 | 1108.8 |
| anim/evaluate/ms | 0.160 | 0.255 | 1.48 | 963.9 |
| anim/evaluate_cpu/ms | 0.155 | 0.199 | 0.54 | 938.1 |
| stream/tick/ms | 0.154 | 0.226 | 0.52 | 929.7 |
| bigspace/high_precision/ms | 0.141 | 0.572 | 1.65 | 848.2 |
| stage/render/prepare_views | 0.134 | 0.178 | 0.50 | 809.0 |
| stage/render/queue | 0.115 | 0.182 | 2.20 | 696.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.100 | 0.165 | 0.62 | 602.6 |
| render/acquire/ms | 0.091 | 0.124 | 0.39 | 549.8 |
| stage/render/phase_sort | 0.091 | 0.128 | 0.42 | 549.7 |
| controller/write/ms | 0.074 | 0.100 | 0.38 | 444.4 |
| controller/move/ms | 0.074 | 0.173 | 0.45 | 443.9 |
| terrain/update/ms | 0.061 | 0.294 | 1.59 | 370.9 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.074 | 0.43 | 291.8 |
| physics/step/ms | 0.046 | 0.100 | 2.67 | 279.7 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 0.53 | 189.9 |
| physics/insert/ms | 0.030 | 0.338 | 0.88 | 180.0 |
| pipelines/compile_ms | 0.029 | 0.000 | 83.12 | 175.9 |
| pipelines/front_ms | 0.028 | 0.000 | 80.08 | 169.6 |
| terrain/select/ms | 0.024 | 0.260 | 0.54 | 144.8 |
| stage/render/cleanup | 0.024 | 0.035 | 0.80 | 142.6 |
| weather/map/ms | 0.018 | 0.024 | 0.24 | 106.3 |
| far/select/ms | 0.016 | 0.170 | 0.49 | 96.5 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.022 | 0.25 | 81.9 |
| anim/apply/ms | 0.012 | 0.022 | 0.22 | 70.3 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.23 | 66.1 |
| weather/eval/ms | 0.011 | 0.014 | 0.23 | 65.0 |
| script/update/ms | 0.008 | 0.012 | 0.20 | 49.7 |
| controller/avoid/ms | 0.007 | 0.012 | 0.15 | 39.9 |
| stream/despawn/ms | 0.006 | 0.000 | 1.02 | 36.8 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 31.3 |
| stream/spawn/ms | 0.004 | 0.000 | 1.21 | 22.2 |
| stream/upload/ms | 0.003 | 0.000 | 1.33 | 19.8 |
| script/ui/ms | 0.003 | 0.004 | 0.04 | 19.7 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.66 | 13.6 |
| pipelines/driver_ms | 0.001 | 0.000 | 3.04 | 6.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 6.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.47 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.7 |
| stage/render/prepare | 0.001 | 0.003 | 0.12 | 4.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 32 | 17 |
| controller/movers/count | count | 19 | 22 | 17 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1281 | 2262 | 601 |
| crowd/c0/meshes/count | count | 353 | 646 | 159 |
| crowd/c1/count | count | 145 | 157 | 148 |
| crowd/c1/joints/count | count | 5573 | 5999 | 5595 |
| crowd/c1/meshes/count | count | 1171 | 1297 | 1182 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 162 | 216 | 205 |
| crowd/joints_extracted/count | count | 682 | 7017 | 437 |
| crowd/joints_uploaded/count | count | 3617 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.45 | 1.39 | 0.51 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.25 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.23 | 0.01 |
| encode/finish_critical | ms | 0.98 | 1.86 | 1.18 |
| encode/pass:early prepass/max | ms | 0.82 | 1.57 | 1.10 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.84 | 1.86 | 0.74 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.62 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.27 | 0.84 | 0.33 |
| encode/pass:shadow_directional_light/max | ms | 0.82 | 1.85 | 1.18 |
| encode/pass:shadow_point_light/max | ms | 0.11 | 1.36 | 0.84 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.53 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 9.13 | 31.92 | 9.40 |
| gpu/timed/ms | ms | 7.94 | 9.67 | 7.66 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 158 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 348 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 158 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 348 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 762 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2476 | 3554 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 762 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2476 | 3554 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 26 | 143 | 27 |
| phases/shadow/bins | count | 1195 | 1635 | 671 |
| phases/shadow/instance_batches | count | 4 | 8 | 8 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 98.3 k |
| phases/shadow/items | count | 2839 | 3533 | 2027 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 154 | 199 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2643 | 3182 | 2346 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 183 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 334 | 335 | 335 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 17 | 443 | 43 |
| preprocess/bind_groups_created | count | 12 | 438 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 6 |
| preprocess/bind_groups_reused | count | 131 | 233 | 123 |
| process/cpu_usage | % | 25.96 | 38.39 | 33.49 |
| process/mem_usage | GiB | 5.56 | 5.74 | 5.74 |
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
| skin/blocks_changed/count | count | 14 | 121 | 9 |
| skin/blocks_skipped/count | count | 147 | 208 | 196 |
| skin/blocks_walked/count | count | 13.50 | 29 | 9 |
| skin/joint_bytes_written/bytes | bytes | 48.6 k | 807.7 k | 37.1 k |
| skin/joints_written/count | count | 760 | 12.6 k | 580 |
| skin/upload_writes/count | count | 8 | 27 | 10 |
| started_load_count |  | 6441 | 6565 | 6565 |
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
| stream/latency/s | s | 0.04 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.65 | 40.50 | 33.18 |
| system/mem_usage | % | 56.80 | 57.39 | 57.32 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 83 | 202 | 85 |
| views/all/bins | count | 3146 | 3834 | 2961 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 12 | 16 | 16 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 27.1 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 196.6 k |
| views/all/items | count | 8514 | 10.8 k | 7969 |
| views/all/pipelines | count | 202 | 246 | 66 |
| views/all/pipelines_distinct | count | 70 | 74 | 61 |
| views/all/work_items | count | 154.1 k | 206.9 k | 205.0 k |
| views/cascade/batch_sets | count | 13 | 34 | 27 |
| views/cascade/bins | count | 718 | 850 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 8 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 98.3 k |
| views/cascade/items | count | 2121 | 2424 | 2027 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 100.3 k |
| views/lamp_face/batch_sets | count | 0 | 132 | 0 |
| views/lamp_face/bins | count | 459 | 864 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 22 | 48 | 0 |
| views/lamp_face/drawn | count | 0 | 44 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 706 | 1190 | 0 |
| views/lamp_face/pipelines | count | 129 | 171 | 0 |
| views/lamp_face/revealed | count | 0 | 5 | 0 |
| views/lamp_face/work_items | count | 0 | 689 | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2012 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5912 | 7528 | 5942 |
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
| anim/bones_written/count | count | 46.03 | 253 | 277.7 k |
| anim/events/count | count | 0.08 | 3 | 482 |
| assets/Image/added | count | 0.10 | 31 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.48 | 262 | 2874 |
| assets/Mesh/removed | count | 0.81 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.24 | 10 | 7484 |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.25 | 1218 | 1490 |
| encode//count | count | 39.67 | 204 | 239.4 k |
| encode/bind_walk/init_textures | count | 9413 | 23.9 k | 56.80 M |
| encode/bind_walk/init_textures_skipped | count | 57.2 k | 83.4 k | 344.99 M |
| encode/bind_walk/inits | count | 576 | 972 | 3.47 M |
| encode/bind_walk/inits_skipped | count | 142 | 235 | 857.7 k |
| encode/bind_walk/merge_textures | count | 9350 | 23.8 k | 56.42 M |
| encode/bind_walk/merge_textures_skipped | count | 11.3 k | 23.7 k | 68.05 M |
| encode/bind_walk/merges | count | 106 | 273 | 639.8 k |
| encode/bind_walk/merges_skipped | count | 131 | 230 | 790.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6034 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6034 |
| encode/pass:early prepass/count | count | 1 | 1 | 6034 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6034 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6034 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6034 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6034 |
| encode/pass:shadow_point_light/count | count | 2.93 | 44 | 17.7 k |
| encode/queue/command_buffers | count | 52.61 | 258 | 317.5 k |
| encode/queue/submits | count | 2.01 | 4 | 12.1 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.60 M | 14.01 G |
| encode/queue/write_buffer/calls | count | 278 | 677 | 1.68 M |
| encode/queue/write_buffer_with/bytes | bytes | 161.4 k | 1.28 M | 973.73 M |
| encode/queue/write_buffer_with/calls | count | 10.41 | 795 | 62.8 k |
| encode/queue/write_texture/bytes | bytes | 2111 | 589.8 k | 12.74 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6034 |
| physics/inserts | count | 0.37 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 175 | 2335 |
| physics/steps | count | 0.65 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 1 | 3 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 35 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5610 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1230 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.6 ms; slowest 49, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2547.1 | main/busy/ms 2515, stage/main/post_update 2472, stage/main/update 32.10, stage/main/pre_update 6.66, stage/main/last 1.43, surface/advance_cpu_ms 0.72, stage/main/first 0.60, stage/main/fixed_main_loop 0.53, stream/drive/ms 0.46, surface/cell_upload_ms 0.32, stream/tick/ms 0.10, bigspace/total/ms 0.09 |
| warm-up | 3.09 | 538.9 | main/busy/ms 2652, stage/main/post_update 2502, render/thread/ms 500, render/busy/ms 405, stage/render/prepare_assets 367, stage/main/update 132, stage/render/prepare_views 91.29, render/acquire/ms 91.04, stage/render/create_views 16.43, stage/render/render 13.36, render/graph/ms 9.26, stage/main/pre_update 8.49 |
| warm-up | 3.16 | 72.0 | render/thread/ms 553, render/busy/ms 458, stage/render/prepare_assets 403, main/busy/ms 163, stage/main/update 108, stage/render/prepare_views 91.85, render/acquire/ms 91.46, pipelines/compile_ms 47.05, pipelines/front_ms 42.08, stage/main/post_update 41.83, terrain/update/ms 22.21, stage/render/render 21.18 |
| warm-up | 3.20 | 41.8 | pipelines/wait_ms 214, pipelines/compile_ms 178, pipelines/front_ms 164, render/thread/ms 94.07, render/busy/ms 86.53, main/busy/ms 71.83, stage/main/post_update 49.76, stage/render/render 40.36, stage/render/prepare_assets 36.93, render/graph/ms 32.39, terrain/update/ms 28.07, encode//sum 14.36 |
| warm-up | 3.35 | 149.2 | pipelines/wait_ms 6129, pipelines/compile_ms 1428, pipelines/front_ms 1340, render/thread/ms 182, render/busy/ms 175, main/busy/ms 128, stage/main/post_update 98.37, pipelines/driver_ms 88.09, stage/render/prepare_resources 38.72, stage/render/render 38.17, stage/render/prepare_assets 36.31, terrain/update/ms 35.63 |
| warm-up | 3.41 | 61.1 | pipelines/wait_ms 6343, pipelines/compile_ms 1566, pipelines/front_ms 1475, render/thread/ms 195, render/busy/ms 193, main/busy/ms 156, pipelines/driver_ms 90.82, stage/main/post_update 70.73, stage/main/update 62.89, stage/render/prepare_assets 53.77, stage/render/prepare_resources 43.05, terrain/update/ms 36.32 |
| warm-up | 3.51 | 101.7 | pipelines/compile_ms 2071, pipelines/front_ms 1955, pipelines/wait_ms 542, render/thread/ms 129, render/busy/ms 127, pipelines/driver_ms 115, main/busy/ms 92.37, stage/render/prepare_assets 84.99, stage/main/update 65.43, stage/main/post_update 17.74, stage/render/prepare_write_phase_buffers 14.98, terrain/update/ms 13.96 |
| warm-up | 3.55 | 36.5 | pipelines/compile_ms 1909, pipelines/front_ms 1799, pipelines/wait_ms 125, render/thread/ms 110, pipelines/driver_ms 109, render/busy/ms 109, stage/render/prepare_assets 76.68, main/busy/ms 59.52, stage/main/post_update 37.57, stage/render/render 24.17, render/graph/ms 22.13, encode//sum 14.50 |
| warm-up | 3.59 | 38.3 | pipelines/compile_ms 397, pipelines/front_ms 372, render/thread/ms 64.69, render/busy/ms 62.98, main/busy/ms 61.88, pipelines/wait_ms 40.83, stage/main/post_update 39.06, stage/render/prepare_assets 31.54, pipelines/driver_ms 24.76, stage/render/render 23.89, render/graph/ms 21.95, encode//sum 14.90 |
| warm-up | 3.62 | 34.5 | pipelines/compile_ms 754, pipelines/front_ms 714, pipelines/wait_ms 93.00, render/thread/ms 63.65, render/busy/ms 62.00, main/busy/ms 56.27, pipelines/driver_ms 39.48, stage/render/prepare_assets 29.14, stage/main/post_update 18.97, stage/main/update 18.35, stage/main/pre_update 12.39, stage/render/render 10.46 |
| warm-up | 3.66 | 42.8 | pipelines/compile_ms 690, pipelines/front_ms 663, main/busy/ms 112, render/thread/ms 73.61, stage/main/update 72.46, render/busy/ms 69.54, pipelines/wait_ms 63.00, pipelines/driver_ms 26.30, stage/main/post_update 24.92, stage/render/render 15.17, stage/render/prepare_assets 14.88, stage/render/prepare_resources 14.01 |
| warm-up | 3.76 | 98.0 | pipelines/compile_ms 1856, pipelines/front_ms 1775, main/busy/ms 89.51, pipelines/driver_ms 80.40, render/thread/ms 72.65, render/busy/ms 68.61, stage/main/update 66.28, stage/render/prepare_assets 28.28, stage/main/post_update 17.86, stage/render/render 16.14, render/graph/ms 11.82, stage/render/prepare_resources 10.25 |
| warm-up | 3.81 | 45.8 | pipelines/compile_ms 1995, pipelines/front_ms 1896, pipelines/driver_ms 99.02, render/thread/ms 66.41, render/busy/ms 64.84, stage/render/prepare_assets 46.39, main/busy/ms 31.38, pipelines/wait_ms 17.68, stage/main/update 15.65, stage/render/render 12.47, render/graph/ms 10.53, pipelines/lock_ms 9.51 |
| warm-up | 3.84 | 30.5 | pipelines/compile_ms 437, pipelines/front_ms 407, render/thread/ms 59.65, render/busy/ms 58.05, stage/render/prepare_assets 42.65, main/busy/ms 33.32, pipelines/driver_ms 29.74, stage/main/update 17.42, pipelines/wait_ms 15.22, stage/render/render 10.13, render/graph/ms 8.20, stage/main/post_update 7.27 |
| warm-up | 3.90 | 37.1 | pipelines/compile_ms 286, pipelines/front_ms 277, render/thread/ms 54.85, render/busy/ms 46.31, stage/render/render 31.33, main/busy/ms 28.91, render/graph/ms 28.23, encode//sum 23.92, stage/render/graph_submit 21.31, stage/main/update 15.67, encode/submit 11.34, pipelines/driver_ms 9.37 |
| warm-up | 3.93 | 30.0 | pipelines/compile_ms 298, pipelines/front_ms 288, render/thread/ms 62.26, render/busy/ms 53.90, main/busy/ms 46.35, stage/render/render 29.85, stage/main/update 28.03, render/graph/ms 26.84, stage/render/graph_submit 18.82, encode//sum 13.68, stage/render/prepare_assets 12.39, stage/main/post_update 10.93 |
| warm-up | 4.02 | 88.1 | pipelines/compile_ms 987, pipelines/front_ms 948, render/thread/ms 80.05, render/busy/ms 78.24, main/busy/ms 66.38, stage/render/prepare_assets 53.80, pipelines/driver_ms 38.98, stage/main/update 29.12, stage/main/fixed_main_loop 17.57, stage/render/render 16.14, render/graph/ms 13.69, stream/drive/ms 11.86 |
| warm-up | 4.11 | 89.8 | pipelines/compile_ms 2356, pipelines/front_ms 2282, render/thread/ms 116, render/busy/ms 114, stage/render/prepare_assets 83.37, main/busy/ms 74.43, pipelines/driver_ms 74.33, stage/main/update 34.26, stage/render/render 25.38, render/graph/ms 22.44, stream/drive/ms 21.10, stage/render/graph_submit 19.16 |
| warm-up | 4.16 | 55.7 | pipelines/compile_ms 1520, pipelines/front_ms 1477, render/thread/ms 112, render/busy/ms 108, main/busy/ms 79.25, stage/render/prepare_assets 49.56, pipelines/driver_ms 43.45, stage/render/render 41.48, stage/main/update 40.54, render/graph/ms 38.91, stage/render/graph_submit 35.58, stage/main/post_update 28.96 |
| warm-up | 4.26 | 94.8 | main/busy/ms 213, stage/main/update 177, pipelines/compile_ms 166, pipelines/front_ms 157, render/thread/ms 137, render/busy/ms 133, pipelines/wait_ms 78.60, stage/render/prepare_assets 37.81, stage/render/render 28.29, stage/main/post_update 26.52, render/graph/ms 26.23, stage/render/graph_submit 22.98 |
| warm-up | 4.42 | 164.3 | pipelines/compile_ms 3113, pipelines/front_ms 3028, pipelines/wait_ms 353, main/busy/ms 181, stage/main/update 164, render/thread/ms 146, render/busy/ms 143, pipelines/driver_ms 84.87, stage/render/prepare_assets 54.92, stage/render/render 34.99, render/graph/ms 31.41, pipelines/lock_ms 26.80 |
| warm-up | 4.46 | 33.2 | pipelines/compile_ms 3100, pipelines/front_ms 3015, pipelines/wait_ms 278, render/thread/ms 91.13, render/busy/ms 88.18, pipelines/driver_ms 84.50, main/busy/ms 42.13, stage/render/render 35.02, render/graph/ms 31.57, stage/render/prepare_assets 27.26, pipelines/lock_ms 26.80, stage/render/prepare_flush 18.26 |
| warm-up | 4.52 | 61.0 | pipelines/compile_ms 688, pipelines/front_ms 663, pipelines/wait_ms 151, render/thread/ms 84.49, main/busy/ms 84.20, render/busy/ms 80.68, stage/main/update 49.62, pipelines/driver_ms 25.67, stage/main/post_update 20.83, stage/render/prepare_flush 18.54, stage/render/prepare_write_phase_buffers 18.16, stage/render/render 14.76 |
| warm-up | 4.59 | 77.3 | pipelines/compile_ms 889, pipelines/front_ms 857, pipelines/wait_ms 151, render/thread/ms 114, render/busy/ms 108, main/busy/ms 91.05, stage/render/prepare_assets 47.93, stage/main/update 46.27, stage/main/post_update 36.17, pipelines/driver_ms 31.60, stage/render/prepare_write_phase_buffers 23.11, stage/render/render 18.60 |
| warm-up | 4.65 | 52.2 | pipelines/compile_ms 1908, pipelines/front_ms 1851, render/thread/ms 111, render/busy/ms 107, pipelines/wait_ms 104, main/busy/ms 65.29, stage/render/prepare_assets 60.75, pipelines/driver_ms 57.47, stage/main/update 29.58, stage/main/post_update 28.98, encode//sum 28.26, stage/render/render 22.89 |
| warm-up | 4.71 | 60.3 | pipelines/compile_ms 2085, pipelines/front_ms 2027, pipelines/wait_ms 114, render/thread/ms 103, render/busy/ms 101, main/busy/ms 64.22, pipelines/driver_ms 58.84, stage/render/prepare_assets 43.02, stage/main/update 38.17, encode//sum 32.93, stage/render/render 27.46, render/graph/ms 24.68 |
| warm-up | 4.77 | 62.1 | pipelines/compile_ms 716, pipelines/front_ms 697, main/busy/ms 111, render/thread/ms 103, render/busy/ms 99.93, stage/main/update 80.98, stage/render/render 32.02, encode//sum 28.58, stage/render/prepare_assets 28.49, render/graph/ms 28.17, pipelines/driver_ms 19.38, stage/render/graph_submit 19.09 |
| warm-up | 4.94 | 174.8 | pipelines/compile_ms 2150, pipelines/front_ms 2081, render/thread/ms 162, render/busy/ms 159, main/busy/ms 147, stage/main/update 121, stage/render/prepare_assets 78.71, pipelines/driver_ms 69.39, stream/drive/ms 62.20, pipelines/wait_ms 49.08, stage/render/render 45.08, render/graph/ms 41.07 |
| warm-up | 4.99 | 46.9 | pipelines/compile_ms 1856, pipelines/front_ms 1794, render/thread/ms 153, render/busy/ms 151, main/busy/ms 97.16, stage/render/prepare_assets 74.06, stage/main/update 70.38, pipelines/driver_ms 62.14, stream/drive/ms 55.28, pipelines/wait_ms 46.80, stage/render/render 43.61, encode//sum 41.36 |
| warm-up | 5.03 | 41.7 | render/thread/ms 81.21, render/busy/ms 79.41, main/busy/ms 62.59, pipelines/compile_ms 41.06, pipelines/front_ms 36.24, stage/render/render 33.59, stage/main/update 33.40, encode//sum 33.36, render/graph/ms 31.40, stage/render/graph_submit 25.81, stage/main/post_update 18.74, encode/finish_wall 13.45 |
| warm-up | 5.08 | 47.4 | render/thread/ms 75.45, render/busy/ms 73.96, main/busy/ms 55.01, pipelines/compile_ms 54.30, pipelines/front_ms 51.51, stage/main/update 28.98, stage/render/render 23.23, render/graph/ms 21.23, stage/main/post_update 17.81, stage/render/prepare_assets 16.42, stage/render/graph_submit 16.04, encode//sum 14.94 |
| warm-up | 5.11 | 27.9 | pipelines/compile_ms 764, pipelines/front_ms 741, render/thread/ms 61.57, render/busy/ms 59.28, main/busy/ms 31.71, pipelines/driver_ms 22.84, stage/render/prepare_assets 17.87, stage/main/update 14.98, stage/render/render 13.45, render/graph/ms 11.39, encode//sum 9.94, stage/main/post_update 8.79 |
| warm-up | 5.25 | 34.6 | pipelines/compile_ms 290, pipelines/front_ms 279, render/thread/ms 48.94, render/busy/ms 47.37, main/busy/ms 44.54, stage/render/render 30.72, render/graph/ms 28.58, stage/main/update 20.57, stage/render/camera_driver 18.84, stage/main/post_update 18.34, stream/drive/ms 12.01, pipelines/wait_ms 11.84 |
| warm-up | 5.29 | 35.0 | pipelines/compile_ms 89.92, pipelines/front_ms 85.17, render/thread/ms 57.28, render/busy/ms 55.72, main/busy/ms 46.30, stage/render/render 31.52, render/graph/ms 29.37, stage/main/update 22.07, stage/render/camera_driver 18.78, stage/main/post_update 18.14, stage/render/prepare_assets 14.78, pipelines/wait_ms 11.64 |
| warm-up | 5.32 | 26.6 | pipelines/compile_ms 478, pipelines/front_ms 464, render/thread/ms 49.08, render/busy/ms 47.67, main/busy/ms 32.62, stage/main/update 16.42, stage/render/render 16.25, stage/render/prepare_assets 15.59, pipelines/driver_ms 14.34, render/graph/ms 14.08, encode//sum 10.24, stage/render/graph_submit 9.82 |
| warm-up | 5.84 | 29.8 | pipelines/compile_ms 159, pipelines/front_ms 156, main/busy/ms 34.52, render/thread/ms 23.48, stage/main/update 22.68, render/busy/ms 22.40, stream/drive/ms 14.58, stage/render/render 10.99, render/graph/ms 9.51, encode//sum 9.22, stage/main/post_update 6.89, stage/render/graph_submit 5.96 |
| warm-up | 5.97 | 29.3 | render/thread/ms 43.26, render/busy/ms 41.67, main/busy/ms 32.90, stage/render/render 21.43, render/graph/ms 19.39, stage/main/update 15.35, stage/render/graph_submit 13.38, stage/main/post_update 13.00, encode//sum 12.87, encode/submit 9.98, stage/render/prepare_assets 8.08, stage/render/camera_driver 5.78 |
| warm-up | 6.02 | 29.9 | render/thread/ms 37.59, render/busy/ms 36.65, main/busy/ms 24.75, stage/render/prepare_assets 19.76, stage/main/update 10.37, stage/render/render 9.42, encode//sum 8.97, render/graph/ms 7.98, stage/main/post_update 7.72, stage/render/graph_submit 5.20, stage/main/fixed_main_loop 2.93, encode/submit 2.89 |
| warm-up | 6.13 | 28.7 | render/thread/ms 46.46, render/busy/ms 45.31, main/busy/ms 36.58, stage/main/update 21.99, stage/render/render 13.23, stage/render/prepare_assets 12.16, render/graph/ms 11.66, encode//sum 10.69, stage/main/post_update 9.17, stage/render/graph_submit 6.96, stage/render/prepare_meshes 6.50, stage/render/camera_driver 4.53 |
| warm-up | 6.24 | 29.4 | render/thread/ms 45.16, render/busy/ms 43.62, main/busy/ms 43.50, stage/main/update 22.91, stage/render/render 20.57, render/graph/ms 18.51, stage/render/graph_submit 13.84, stage/main/post_update 13.58, encode//sum 11.70, encode/submit 10.96, stage/render/prepare_assets 8.71, stage/render/camera_driver 4.43 |
| warm-up | 6.27 | 30.9 | pipelines/compile_ms 153, pipelines/front_ms 147, render/thread/ms 49.56, render/busy/ms 47.91, main/busy/ms 42.78, stage/main/update 22.98, stage/render/render 20.67, render/graph/ms 18.44, stage/render/graph_submit 13.09, stage/main/post_update 12.53, encode//sum 12.07, stage/render/prepare_assets 10.57 |
| warm-up | 6.30 | 34.0 | pipelines/compile_ms 161, pipelines/front_ms 155, render/thread/ms 49.47, render/busy/ms 48.30, main/busy/ms 41.50, stage/main/update 21.38, stage/render/prepare_assets 15.48, stage/render/render 13.08, stage/main/post_update 11.38, render/graph/ms 11.36, encode//sum 11.08, stage/render/graph_submit 6.98 |
| warm-up | 6.34 | 39.6 | render/thread/ms 61.82, render/busy/ms 60.52, main/busy/ms 50.68, stage/main/update 24.25, pipelines/compile_ms 22.72, pipelines/front_ms 21.85, stage/render/prepare_assets 18.14, stage/main/post_update 17.12, stage/render/render 16.58, render/graph/ms 14.79, encode//sum 13.43, stage/render/graph_submit 8.96 |
| warm-up | 6.40 | 32.7 | render/thread/ms 39.43, render/busy/ms 38.42, main/busy/ms 34.56, stage/main/update 19.04, stage/render/prepare_assets 15.18, stage/render/render 12.30, render/graph/ms 10.80, stage/main/post_update 10.19, encode//sum 9.97, stage/render/graph_submit 7.08, stream/drive/ms 4.65, encode/submit 4.42 |
| warm-up | 6.54 | 37.3 | main/busy/ms 48.54, render/thread/ms 36.88, render/busy/ms 35.65, stage/main/update 31.01, stage/render/render 14.50, stream/drive/ms 13.62, render/graph/ms 12.74, encode//sum 12.70, stage/main/post_update 12.36, stage/render/graph_submit 7.84, stage/render/prepare_assets 6.91, encode/submit 4.76 |
| warm-up | 6.58 | 36.4 | main/busy/ms 55.08, render/thread/ms 54.36, render/busy/ms 52.60, stage/main/update 33.73, stage/render/render 20.59, render/graph/ms 17.94, encode//sum 15.29, stage/main/post_update 15.13, stage/render/prepare_assets 13.26, stage/render/graph_submit 11.49, stage/render/prepare_meshes 6.62, encode/submit 6.31 |
| warm-up | 6.61 | 33.5 | render/thread/ms 56.05, render/busy/ms 54.44, main/busy/ms 48.96, stage/main/update 28.64, stage/render/prepare_assets 20.92, stage/render/render 17.78, render/graph/ms 15.31, stage/main/post_update 13.87, encode//sum 12.34, stage/render/graph_submit 10.32, stage/render/prepare_meshes 6.57, stream/drive/ms 6.29 |
| measure | 18.34 | 35.6 | main/busy/ms 41.52, render/thread/ms 29.18, render/busy/ms 28.15, stage/main/update 26.88, stage/render/render 16.91, encode//sum 16.69, render/graph/ms 15.35, stage/render/graph_submit 10.91, stage/main/post_update 9.14, encode/submit 7.35, stage/render/camera_driver 4.31, stage/render/prepare_assets 4.13 |
| measure | 59.17 | 36.4 | render/thread/ms 42.53, render/busy/ms 41.05, main/busy/ms 25.26, stage/render/prepare_assets 17.20, stage/render/render 13.66, stage/main/post_update 12.66, render/graph/ms 11.57, encode//sum 11.32, stage/render/graph_submit 7.93, stage/main/update 4.95, encode/submit 4.37, stage/main/fixed_main_loop 3.74 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\balmora_walk_night@2560x1440_arm=base-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 8%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1512 pmon samples)
- focus: clean
- nvidia-smi over the window (254 samples): GPU load **81%** (min 62%) · 2625 MHz (min 2625) · memory 8101 MHz · 77.2 W · 73 °C (max 78) · VRAM 6531 MiB · throttled 100% of samples

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
