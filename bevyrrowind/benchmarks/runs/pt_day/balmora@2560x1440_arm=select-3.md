# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); select.gate=on (command line: --set); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1541 frames over 15.0 s · **median 9.69 ms (103 fps)** · mean 9.74 · p95 11.06 · p99 11.96 (1% low 84 fps) · worst 68.41 ms · 6 spikes over 1.5× median · 28.0 stutters/min (work 20.0/min)
- warm-up: 278 frames over 8.4 s, worst 2594.0 ms
- hitches (> 25.0 ms): **2** measured, 57 in warm-up
- timed GPU passes: 8.51 ms of a 9.74 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.69 ms, limit 11 ms
- **FAIL** worst frame 68.41 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.869 | 2.890 | 2.958 | 0.99 | 0.078 |
| dlss_super_resolution | 2.006 | 2.021 | 2.044 | 0.99 | 0.184 |
| early prepass | 0.615 | 0.620 | 0.630 | 0.99 | 0.095 |
| atmosphere_luts | 0.336 | 0.338 | 0.396 | 0.99 | 0.003 |
| bloom | 0.314 | 0.316 | 0.332 | 0.99 | 0.061 |
| main_transmissive_pass_3d | 0.309 | 0.311 | 0.332 | 0.99 | 0.009 |
| water_velocity | 0.237 | 0.239 | 0.263 | 0.99 | 0.002 |
| water_sspr | 0.211 | 0.212 | 0.213 | 0.99 | 0.002 |
| ssao | 0.191 | 0.192 | 0.197 | 0.99 | 0.007 |
| clouds_trace | 0.148 | 0.149 | 0.169 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.145 | 0.146 | 0.153 | 0.99 | 0.001 |
| water_fft | 0.130 | 0.131 | 0.134 | 0.99 | 0.003 |
| uniform_allocation | 0.121 | 0.122 | 0.162 | 0.99 | 0.024 |
| clouds_composite | 0.111 | 0.112 | 0.113 | 0.99 | 0.002 |
| upscaling | 0.110 | 0.111 | 0.117 | 0.99 | 0.004 |
| bin_unpacking | 0.090 | 0.091 | 0.122 | 0.99 | 0.018 |
| tonemapping | 0.083 | 0.083 | 0.089 | 0.99 | 0.003 |
| clustering | 0.064 | 0.065 | 0.075 | 0.99 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.412 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 0.99 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 0.99 | 0.006 |
| lightprobe_radiance_map | 0.027 | 0.220 | 0.231 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.030 | 0.99 | 0.086 |
| water_snapshot | 0.024 | 0.024 | 0.026 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.047 | 0.32 | 0.003 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.024 | 0.99 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.797 | 10.151 | 67.52 | 13556.1 |
| render/busy/ms | 7.236 | 8.234 | 66.41 | 11150.3 |
| main/busy/ms | 6.930 | 7.846 | 9.07 | 10678.6 |
| stage/render/render | 4.862 | 5.686 | 63.85 | 7492.3 |
| encode//sum | 4.290 | 5.629 | 63.70 | 6611.5 |
| render/graph/ms | 4.277 | 5.040 | 63.21 | 6591.0 |
| stage/main/post_update | 4.153 | 4.909 | 6.07 | 6399.5 |
| stage/render/graph_submit | 3.249 | 3.973 | 62.20 | 5006.2 |
| encode/submit | 1.969 | 2.391 | 5.62 | 3033.5 |
| stage/main/update | 1.643 | 2.056 | 2.98 | 2531.9 |
| encode/finish_wall | 1.309 | 1.774 | 58.85 | 2016.9 |
| stage/render/prepare_views | 1.240 | 2.858 | 4.08 | 1910.4 |
| pipelines/compile_ms | 1.237 | 0.000 | 136.30 | 1906.5 |
| pipelines/front_ms | 1.205 | 0.000 | 133.24 | 1856.8 |
| render/acquire/ms | 1.197 | 2.816 | 4.04 | 1844.6 |
| stage/render/camera_driver | 0.972 | 1.117 | 1.44 | 1498.2 |
| encode/pass:early prepass/sum | 0.822 | 1.070 | 57.91 | 1267.3 |
| encode/pass:shadow_directional_light/sum | 0.821 | 1.366 | 2.24 | 1265.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.757 | 1.031 | 1.45 | 1166.8 |
| stage/render/prepare_write_phase_buffers | 0.564 | 0.768 | 0.98 | 869.0 |
| stage/render/prepare_assets | 0.464 | 0.597 | 1.49 | 714.4 |
| stage/render/prepare_resources | 0.387 | 0.447 | 0.67 | 595.7 |
| bigspace/total/ms | 0.367 | 0.827 | 1.58 | 566.2 |
| render/present/ms | 0.364 | 0.433 | 1.96 | 561.1 |
| stage/main/pre_update | 0.353 | 0.463 | 0.96 | 544.3 |
| stage/main/first | 0.297 | 0.358 | 1.61 | 458.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.260 | 0.375 | 0.57 | 400.1 |
| stage/render/prepare_bind_groups | 0.216 | 0.302 | 1.33 | 333.1 |
| stage/render/prepare_meshes | 0.211 | 0.262 | 2.36 | 325.2 |
| stage/main/fixed_main_loop | 0.184 | 0.349 | 0.96 | 283.5 |
| stage/render/extract_commands | 0.179 | 0.213 | 0.42 | 275.2 |
| stage/render/prepare_batch_phases | 0.177 | 0.232 | 0.42 | 272.6 |
| stage/render/create_views | 0.169 | 0.217 | 0.81 | 259.8 |
| controller/move/ms | 0.155 | 0.289 | 0.84 | 238.1 |
| stage/render/prepare_flush | 0.153 | 0.265 | 0.70 | 235.8 |
| stream/drive/ms | 0.153 | 0.217 | 0.59 | 235.3 |
| controller/feed/ms | 0.149 | 0.196 | 0.63 | 229.9 |
| bigspace/high_precision/ms | 0.135 | 0.396 | 1.33 | 207.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.133 | 0.203 | 0.31 | 204.8 |
| stream/tick/ms | 0.123 | 0.180 | 0.57 | 189.1 |
| stage/main/last | 0.120 | 0.159 | 0.86 | 184.6 |
| stage/render/queue | 0.107 | 0.136 | 5.94 | 165.2 |
| stage/render/specialize | 0.094 | 0.129 | 8.57 | 145.3 |
| bigspace/low_precision/ms | 0.089 | 0.200 | 1.33 | 136.7 |
| anim/evaluate/ms | 0.083 | 0.199 | 1.27 | 127.6 |
| anim/evaluate_cpu/ms | 0.061 | 0.076 | 0.57 | 94.6 |
| stage/render/prepare_collect_phase_buffers | 0.061 | 0.121 | 0.26 | 94.5 |
| encode/resolve_diagnostics/sum | 0.059 | 0.095 | 0.26 | 90.2 |
| pipelines/driver_ms | 0.032 | 0.000 | 3.96 | 49.7 |
| terrain/update/ms | 0.031 | 0.041 | 0.13 | 47.2 |
| encode/camera_driver_span_begin/sum | 0.028 | 0.042 | 0.17 | 43.2 |
| physics/step/ms | 0.028 | 0.057 | 0.12 | 42.6 |
| weather/map/ms | 0.025 | 0.033 | 0.11 | 38.3 |
| encode/camera_driver_span_end/sum | 0.025 | 0.039 | 0.20 | 37.8 |
| stage/render/cleanup | 0.023 | 0.032 | 0.33 | 35.8 |
| stage/render/phase_sort | 0.017 | 0.032 | 0.09 | 25.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.41 | 17.5 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 11.2 |
| controller/avoid/ms | 0.006 | 0.007 | 0.45 | 8.9 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 8.4 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 6.7 |
| far/select/ms | 0.004 | 0.004 | 0.02 | 5.7 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 5.4 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 3.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.09 | 1.0 |
| terrain/select/ms | 0.000 | 0.001 | 0.01 | 0.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 0.6 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.03 | 0.5 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| crowd/c1/count | count | 123 | 123 | 122 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1080 |
| crowd/c2/count | count | 283 | 284 | 284 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.59 | 58.80 | 0.62 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.17 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.20 | 0.02 |
| encode/finish_critical | ms | 0.92 | 58.80 | 0.81 |
| encode/pass:early prepass/max | ms | 0.76 | 57.91 | 0.67 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 1.45 | 0.81 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.31 | 0.12 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 0.57 | 0.43 |
| encode/pass:shadow_directional_light/max | ms | 0.75 | 2.24 | 0.60 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.26 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.20 | 66.41 | 9.18 |
| gpu/timed/ms | ms | 9.20 | 10.86 | 9.18 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1943 | 1943 | 1943 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 207.2 k | 207.2 k | 207.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 13.3 k | 13.3 k | 13.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 485.99 M | 485.99 M | 485.99 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 21 | 21 | 21 |
| phases/opaque3d/bins | count | 1306 | 1306 | 1306 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/bins | count | 1306 | 1306 | 1306 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 1 | 30 | 1 |
| phases/shadow/bins | count | 2152 | 4889 | 2152 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6053 | 17.7 k | 6053 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 23 | 30 | 23 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 320 | 320 | 320 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 9 | 0 |
| preprocess/bind_groups | count | 0 | 62 | 0 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 127 | 127 | 127 |
| process/cpu_usage | % | 28.37 | 38.94 | 24.29 |
| process/mem_usage | GiB | 4.64 | 4.73 | 4.73 |
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
| skin/blocks_changed/count | count | 5 | 7 | 5 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 181 | 181 | 181 |
| skin/joint_bytes_written/bytes | bytes | 309.5 k | 309.5 k | 309.5 k |
| skin/joints_written/count | count | 4836 | 4836 | 4836 |
| skin/upload_writes/count | count | 1 | 1 | 1 |
| started_load_count |  | 3704 | 3706 | 3706 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 386 | 386 | 386 |
| statics/materials_bound | count | 386 | 386 | 386 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.97 | 45.49 | 28.53 |
| system/mem_usage | % | 57.95 | 58.26 | 58.26 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 53 | 82 | 53 |
| views/all/bins | count | 5324 | 8061 | 5324 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 15.3 k | 27.0 k | 15.3 k |
| views/all/pipelines | count | 71 | 78 | 71 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 9496 | 27.2 k | 9496 |
| views/cascade/batch_sets | count | 1 | 30 | 1 |
| views/cascade/bins | count | 2152 | 4889 | 2152 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 6053 | 17.7 k | 6053 |
| views/cascade/pipelines | count | 23 | 30 | 23 |
| views/cascade/work_items | count | 34 | 17.7 k | 34 |
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
| views/main/batch_sets | count | 52 | 52 | 52 |
| views/main/bins | count | 3172 | 3172 | 3172 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 48 | 48 | 48 |
| views/main/work_items | count | 9462 | 9462 | 9462 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 3 | 155 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.21 | 11 | 4939 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 29 | 40.9 k |
| encode/bind_walk/init_textures | count | 7165 | 14.0 k | 11.04 M |
| encode/bind_walk/init_textures_skipped | count | 40.0 k | 54.5 k | 61.59 M |
| encode/bind_walk/inits | count | 345 | 437 | 532.2 k |
| encode/bind_walk/inits_skipped | count | 71.50 | 85 | 110.2 k |
| encode/bind_walk/merge_textures | count | 7101 | 13.9 k | 10.94 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.29 M |
| encode/bind_walk/merges | count | 82.26 | 103 | 126.8 k |
| encode/bind_walk/merges_skipped | count | 53.00 | 59 | 81.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1541 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1541 |
| encode/pass:early prepass/count | count | 1 | 1 | 1541 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1541 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1541 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1541 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1541 |
| encode/queue/command_buffers | count | 36.57 | 39 | 56.4 k |
| encode/queue/submits | count | 2 | 2 | 3082 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.86 M | 2.22 G |
| encode/queue/write_buffer/calls | count | 217 | 288 | 334.9 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.30 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.9 k |
| encode/queue/write_texture/bytes | bytes | 11.7 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1541 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 3 | 900 |
| physics/steps_dropped | count | 0.00 | 1 | 1 |
| pipelines/after_reveal | count | 0.02 | 1 | 27 |
| pipelines/created | count | 0.02 | 1 | 27 |
| precache/hit | count | 0 | 0 | 0 |
| precache/missed | count | 0 | 0 | 0 |
| precache/requested | count | 0 | 0 | 0 |
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

## Hitches (> 25.0 ms; slowest 59, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.59 | 2594.0 | main/busy/ms 2560, stage/main/post_update 2453, stage/main/update 97.90, surface/advance_cpu_ms 67.05, stage/main/pre_update 5.10, stage/main/last 1.07, surface/cell_upload_ms 0.77, stage/main/first 0.55, stream/drive/ms 0.51, stage/main/fixed_main_loop 0.36, weather/map/ms 0.15, stream/tick/ms 0.10 |
| warm-up | 3.35 | 760.9 | main/busy/ms 2701, stage/main/post_update 2481, render/thread/ms 722, render/busy/ms 543, stage/render/prepare_assets 493, stage/main/update 206, stage/render/prepare_views 173, render/acquire/ms 173, surface/advance_cpu_ms 67.05, stage/render/create_views 28.38, stage/render/render 15.57, render/graph/ms 8.86 |
| warm-up | 3.53 | 173.5 | render/thread/ms 877, render/busy/ms 611, stage/render/prepare_assets 545, stage/render/prepare_views 174, render/acquire/ms 173, main/busy/ms 172, stage/main/update 114, stage/render/render 108, render/present/ms 92.45, pipelines/compile_ms 73.11, pipelines/front_ms 61.25, stage/main/post_update 44.62 |
| warm-up | 3.73 | 177.5 | pipelines/wait_ms 4084, pipelines/compile_ms 2014, pipelines/front_ms 1786, pipelines/driver_ms 228, render/thread/ms 121, render/busy/ms 118, main/busy/ms 111, stage/render/prepare_assets 74.71, stage/main/update 59.60, stage/main/post_update 35.49, stage/render/render 30.68, render/graph/ms 27.42 |
| warm-up | 3.77 | 36.7 | pipelines/wait_ms 4057, pipelines/compile_ms 2021, pipelines/front_ms 1893, render/thread/ms 134, render/busy/ms 131, pipelines/driver_ms 129, stage/render/prepare_assets 91.19, main/busy/ms 53.37, stage/render/render 20.03, stage/main/post_update 19.82, render/graph/ms 17.79, stage/main/update 15.24 |
| warm-up | 3.82 | 53.0 | pipelines/compile_ms 1137, pipelines/front_ms 1096, pipelines/wait_ms 165, render/thread/ms 72.93, render/busy/ms 71.14, stage/render/prepare_assets 48.43, pipelines/driver_ms 41.57, main/busy/ms 37.64, stage/main/post_update 14.84, terrain/update/ms 14.42, stage/main/update 13.07, stage/render/prepare_resources 8.50 |
| warm-up | 3.84 | 26.3 | pipelines/compile_ms 830, pipelines/front_ms 799, pipelines/wait_ms 144, render/thread/ms 58.17, render/busy/ms 56.62, stage/render/prepare_assets 37.66, pipelines/driver_ms 30.66, main/busy/ms 23.92, stage/main/post_update 14.07, terrain/update/ms 9.46, stage/render/render 8.82, encode//sum 7.20 |
| warm-up | 3.88 | 34.2 | pipelines/compile_ms 425, pipelines/front_ms 398, render/thread/ms 51.87, render/busy/ms 50.03, main/busy/ms 36.30, pipelines/driver_ms 26.73, stage/main/post_update 24.45, encode//sum 15.07, stage/render/render 14.84, render/graph/ms 12.75, stage/render/prepare_resources 12.38, pipelines/wait_ms 11.21 |
| warm-up | 3.92 | 37.4 | pipelines/compile_ms 1097, pipelines/front_ms 1035, render/thread/ms 67.47, render/busy/ms 65.36, pipelines/driver_ms 61.82, pipelines/wait_ms 50.22, main/busy/ms 48.43, stage/main/post_update 26.43, stage/render/prepare_resources 22.85, stage/render/render 16.16, stage/render/prepare_write_phase_buffers 15.44, encode//sum 14.78 |
| warm-up | 3.95 | 32.0 | pipelines/compile_ms 1149, pipelines/front_ms 1087, pipelines/wait_ms 64.62, render/thread/ms 64.33, render/busy/ms 62.17, pipelines/driver_ms 61.72, main/busy/ms 37.90, stage/render/prepare_assets 21.68, stage/main/post_update 16.07, stage/render/prepare_resources 15.34, stage/render/render 11.58, stage/render/prepare_write_phase_buffers 10.24 |
| warm-up | 3.98 | 33.8 | pipelines/compile_ms 838, pipelines/front_ms 795, main/busy/ms 62.45, render/thread/ms 61.59, render/busy/ms 59.20, stage/main/update 48.77, stream/drive/ms 44.52, pipelines/driver_ms 42.57, stage/render/prepare_assets 41.81, pipelines/wait_ms 28.81, stage/render/render 11.29, terrain/update/ms 9.17 |
| warm-up | 4.05 | 66.7 | pipelines/compile_ms 1067, pipelines/front_ms 1030, render/thread/ms 70.76, render/busy/ms 68.92, main/busy/ms 60.39, stage/render/prepare_assets 54.29, stage/main/update 48.71, stream/drive/ms 44.51, pipelines/wait_ms 37.53, pipelines/driver_ms 37.07, stage/render/render 9.33, render/graph/ms 7.12 |
| warm-up | 4.11 | 52.0 | pipelines/compile_ms 183, pipelines/front_ms 179, render/thread/ms 43.71, render/busy/ms 41.80, main/busy/ms 32.06, stage/render/prepare_assets 25.86, stage/main/update 19.87, stream/drive/ms 15.99, stage/render/render 10.97, render/graph/ms 8.69, encode//sum 8.50, stage/main/post_update 6.24 |
| warm-up | 4.14 | 26.5 | main/busy/ms 74.45, stage/main/update 58.95, render/thread/ms 53.95, render/busy/ms 52.26, stream/drive/ms 36.80, stage/render/prepare_assets 36.41, stage/render/render 10.56, render/graph/ms 8.16, stage/main/post_update 8.14, encode//sum 7.31, stage/render/graph_submit 5.08, encode/submit 3.72 |
| warm-up | 4.32 | 183.4 | pipelines/compile_ms 230, pipelines/front_ms 225, render/thread/ms 123, render/busy/ms 110, main/busy/ms 77.63, stage/main/update 61.54, stage/render/prepare_assets 55.42, stream/drive/ms 36.84, stage/render/prepare_bind_groups 33.53, stage/render/render 27.77, render/graph/ms 14.31, encode/submit 13.64 |
| warm-up | 4.36 | 44.5 | pipelines/compile_ms 372, pipelines/front_ms 352, render/thread/ms 131, render/busy/ms 119, main/busy/ms 70.93, stage/render/prepare_assets 62.11, stage/main/update 51.88, stream/drive/ms 35.50, stage/render/prepare_bind_groups 34.36, stage/render/render 28.30, pipelines/driver_ms 19.77, render/graph/ms 14.69 |
| warm-up | 4.43 | 68.3 | pipelines/compile_ms 141, pipelines/front_ms 127, main/busy/ms 68.57, render/thread/ms 51.31, stage/main/update 49.74, render/busy/ms 49.56, stream/drive/ms 34.55, stage/render/prepare_assets 30.00, pipelines/driver_ms 13.88, stage/render/render 11.94, render/graph/ms 9.60, stage/main/post_update 8.37 |
| warm-up | 4.49 | 52.8 | pipelines/compile_ms 1005, pipelines/front_ms 960, pipelines/wait_ms 77.89, render/thread/ms 63.14, render/busy/ms 61.30, main/busy/ms 45.73, pipelines/driver_ms 44.99, stage/render/prepare_assets 26.15, stage/main/update 14.23, stage/main/first 12.83, stage/main/post_update 12.61, stage/render/render 11.73 |
| warm-up | 4.55 | 66.5 | pipelines/compile_ms 1221, pipelines/front_ms 1168, pipelines/wait_ms 439, render/thread/ms 102, render/busy/ms 100, pipelines/driver_ms 53.43, main/busy/ms 52.57, stage/render/prepare_resources 28.75, stage/render/prepare_assets 22.37, stage/main/update 18.44, stage/render/prepare_write_phase_buffers 17.99, stage/main/post_update 16.47 |
| warm-up | 4.59 | 42.0 | pipelines/wait_ms 421, pipelines/compile_ms 233, pipelines/front_ms 225, render/thread/ms 99.02, render/busy/ms 97.66, main/busy/ms 45.39, stage/render/prepare_assets 30.18, stage/render/prepare_resources 25.08, stage/main/update 20.88, stage/main/post_update 17.62, stage/render/prepare_write_phase_buffers 15.87, stage/render/render 10.46 |
| warm-up | 4.67 | 79.3 | pipelines/compile_ms 1550, pipelines/front_ms 1508, pipelines/wait_ms 218, render/thread/ms 90.41, render/busy/ms 89.12, main/busy/ms 73.79, stage/render/prepare_assets 57.07, pipelines/driver_ms 42.06, stage/main/update 41.41, stage/main/post_update 24.00, stage/render/render 9.95, render/graph/ms 8.12 |
| warm-up | 4.71 | 39.2 | pipelines/compile_ms 2653, pipelines/front_ms 2568, pipelines/wait_ms 275, render/thread/ms 89.92, render/busy/ms 87.75, pipelines/driver_ms 85.34, main/busy/ms 78.49, stage/main/update 47.13, stage/render/prepare_assets 38.40, stage/main/post_update 22.52, stage/render/render 16.61, render/graph/ms 14.02 |
| warm-up | 4.75 | 35.3 | pipelines/compile_ms 1137, pipelines/front_ms 1092, pipelines/wait_ms 117, render/thread/ms 62.55, render/busy/ms 60.30, main/busy/ms 50.35, pipelines/driver_ms 44.88, stage/main/update 26.33, stage/render/prepare_assets 20.20, stage/render/render 16.17, render/graph/ms 13.41, stage/main/post_update 12.49 |
| warm-up | 4.79 | 39.3 | pipelines/compile_ms 266, pipelines/front_ms 256, main/busy/ms 75.11, stage/main/update 58.39, render/thread/ms 53.39, pipelines/wait_ms 52.55, render/busy/ms 51.97, stream/drive/ms 49.25, stage/render/prepare_assets 32.20, stage/render/render 10.62, pipelines/driver_ms 9.28, render/graph/ms 8.27 |
| warm-up | 4.83 | 47.1 | pipelines/compile_ms 1046, pipelines/front_ms 1017, pipelines/wait_ms 215, main/busy/ms 102, stage/main/update 87.58, stream/drive/ms 81.48, render/thread/ms 40.84, render/busy/ms 39.35, pipelines/driver_ms 28.94, stage/render/prepare_assets 21.32, stage/render/render 10.83, stage/main/post_update 8.66 |
| warm-up | 4.89 | 57.2 | pipelines/compile_ms 1175, pipelines/front_ms 1143, pipelines/wait_ms 164, main/busy/ms 64.23, stage/main/update 43.33, stream/drive/ms 36.69, render/thread/ms 32.07, pipelines/driver_ms 32.01, render/busy/ms 29.83, stage/render/render 12.76, stage/main/post_update 11.88, stage/render/prepare_assets 11.82 |
| warm-up | 4.98 | 85.6 | pipelines/compile_ms 626, pipelines/front_ms 606, render/thread/ms 95.81, render/busy/ms 93.62, main/busy/ms 81.58, stage/main/post_update 30.68, pipelines/wait_ms 29.87, stage/render/prepare_resources 28.90, stage/render/prepare_assets 26.14, stage/main/update 24.84, pipelines/driver_ms 20.00, stage/render/prepare_flush 18.04 |
| warm-up | 5.04 | 67.7 | pipelines/compile_ms 1317, pipelines/front_ms 1278, render/thread/ms 130, render/busy/ms 129, main/busy/ms 108, stage/render/prepare_assets 50.51, stage/main/update 42.62, pipelines/wait_ms 42.25, pipelines/driver_ms 39.19, stage/main/post_update 30.78, stage/render/prepare_resources 30.00, stage/render/prepare_flush 17.56 |
| warm-up | 5.09 | 49.9 | pipelines/compile_ms 2134, pipelines/front_ms 2072, render/thread/ms 93.18, render/busy/ms 91.66, main/busy/ms 71.54, pipelines/driver_ms 62.56, stage/render/prepare_assets 52.67, stage/main/update 37.70, pipelines/wait_ms 21.61, stage/main/post_update 15.51, stage/main/fixed_main_loop 14.05, stream/drive/ms 12.27 |
| warm-up | 5.16 | 27.9 | pipelines/compile_ms 299, pipelines/front_ms 289, pipelines/wait_ms 58.79, render/thread/ms 37.71, render/busy/ms 36.14, main/busy/ms 35.56, stage/main/update 21.57, stage/render/render 13.10, render/graph/ms 10.97, pipelines/driver_ms 10.35, encode//sum 8.64, stage/main/post_update 7.97 |
| warm-up | 5.22 | 28.1 | pipelines/compile_ms 501, pipelines/front_ms 487, pipelines/wait_ms 111, main/busy/ms 42.42, render/thread/ms 41.74, render/busy/ms 39.93, stage/main/update 23.74, stage/render/render 17.84, render/graph/ms 15.65, stream/drive/ms 15.43, pipelines/driver_ms 13.26, stage/main/post_update 12.53 |
| warm-up | 5.24 | 28.0 | pipelines/compile_ms 252, pipelines/front_ms 246, pipelines/wait_ms 111, render/thread/ms 45.37, render/busy/ms 43.56, main/busy/ms 37.22, stage/main/post_update 17.03, stage/render/render 14.62, stage/main/update 12.42, render/graph/ms 12.38, encode//sum 9.79, stage/render/graph_submit 8.58 |
| warm-up | 5.29 | 41.7 | pipelines/compile_ms 261, pipelines/front_ms 255, render/thread/ms 55.52, render/busy/ms 53.93, main/busy/ms 49.54, stage/main/update 29.12, stage/render/prepare_assets 18.04, stage/render/render 13.65, stage/main/post_update 12.84, render/graph/ms 11.65, stage/render/graph_submit 8.11, encode//sum 7.65 |
| warm-up | 5.33 | 43.4 | pipelines/compile_ms 861, pipelines/front_ms 841, render/thread/ms 67.97, render/busy/ms 66.26, main/busy/ms 50.47, stage/render/prepare_assets 34.58, stage/main/update 32.79, pipelines/driver_ms 19.77, stage/render/render 14.59, render/graph/ms 12.46, stage/main/post_update 11.00, stage/render/graph_submit 9.40 |
| warm-up | 5.37 | 40.4 | pipelines/compile_ms 922, pipelines/front_ms 897, render/thread/ms 69.42, render/busy/ms 67.82, main/busy/ms 46.49, stage/render/prepare_assets 29.76, pipelines/driver_ms 25.47, stage/main/update 24.47, stage/main/post_update 15.51, stage/render/render 12.36, render/graph/ms 10.42, stage/render/prepare_resources 8.35 |
| warm-up | 5.41 | 44.9 | pipelines/compile_ms 727, pipelines/front_ms 703, render/thread/ms 63.94, render/busy/ms 62.53, main/busy/ms 44.04, stage/render/prepare_assets 28.85, pipelines/driver_ms 23.95, stage/main/update 23.40, stage/main/post_update 14.16, stage/render/render 10.51, render/graph/ms 8.73, stage/render/prepare_resources 8.38 |
| warm-up | 5.45 | 32.3 | pipelines/compile_ms 696, pipelines/front_ms 676, render/thread/ms 58.41, render/busy/ms 56.79, main/busy/ms 32.49, stage/render/prepare_assets 31.95, pipelines/driver_ms 19.40, stage/main/update 13.58, stage/main/post_update 12.24, stage/render/render 10.45, render/graph/ms 8.42, encode//sum 7.28 |
| warm-up | 5.52 | 28.1 | pipelines/compile_ms 378, pipelines/front_ms 367, render/thread/ms 43.41, render/busy/ms 41.86, main/busy/ms 33.08, stage/main/post_update 19.80, stage/render/prepare_resources 14.07, stage/render/render 12.34, render/graph/ms 10.44, pipelines/driver_ms 10.30, anim/evaluate/ms 9.66, stage/main/update 8.87 |
| warm-up | 5.59 | 25.1 | pipelines/compile_ms 560, pipelines/front_ms 549, render/thread/ms 35.31, render/busy/ms 33.99, main/busy/ms 24.16, stage/render/prepare_assets 13.74, pipelines/driver_ms 11.43, stage/render/render 10.30, stage/main/update 10.22, stage/main/post_update 9.15, render/graph/ms 8.63, encode//sum 7.46 |
| warm-up | 5.65 | 37.6 | pipelines/compile_ms 185, pipelines/front_ms 182, render/thread/ms 50.52, render/busy/ms 49.47, main/busy/ms 35.63, stage/render/prepare_assets 28.67, stage/main/update 18.28, stage/main/post_update 11.21, stage/render/render 10.85, render/graph/ms 9.19, encode//sum 8.99, stage/render/graph_submit 6.76 |
| warm-up | 5.73 | 31.6 | render/thread/ms 49.23, render/busy/ms 47.98, main/busy/ms 31.66, stage/render/prepare_assets 23.76, stage/main/update 12.40, stage/render/render 11.96, render/graph/ms 10.17, stage/main/post_update 10.12, encode//sum 9.20, stage/render/graph_submit 7.08, stream/drive/ms 5.06, encode/submit 4.78 |
| warm-up | 5.77 | 35.6 | render/thread/ms 53.15, render/busy/ms 52.09, main/busy/ms 32.19, stage/render/prepare_assets 31.37, stage/main/update 13.30, stage/main/post_update 9.53, stage/render/render 8.61, encode//sum 7.49, render/graph/ms 7.06, stream/drive/ms 4.88, stage/render/graph_submit 4.84, stage/main/fixed_main_loop 3.64 |
| warm-up | 5.80 | 28.8 | render/thread/ms 51.62, render/busy/ms 50.50, stage/render/prepare_assets 33.56, main/busy/ms 30.27, pipelines/compile_ms 24.51, pipelines/front_ms 23.35, stage/main/update 13.55, stage/main/post_update 10.58, stage/render/render 8.61, render/graph/ms 7.08, encode//sum 6.70, stage/render/graph_submit 4.91 |
| warm-up | 5.82 | 25.5 | render/thread/ms 47.57, render/busy/ms 46.59, stage/render/prepare_assets 32.45, main/busy/ms 27.47, pipelines/compile_ms 24.51, pipelines/front_ms 23.35, stage/main/update 11.84, stage/main/post_update 9.77, stage/render/render 7.13, render/graph/ms 5.82, encode//sum 5.78, stream/drive/ms 4.01 |
| warm-up | 5.85 | 28.9 | render/thread/ms 46.90, render/busy/ms 45.65, main/busy/ms 31.51, stage/render/prepare_assets 26.30, stage/main/update 14.69, stage/main/post_update 10.86, stage/render/render 10.47, render/graph/ms 8.41, encode//sum 8.30, stage/render/graph_submit 5.86, stream/drive/ms 3.90, encode/submit 3.68 |
| warm-up | 5.88 | 28.1 | render/thread/ms 49.53, render/busy/ms 47.80, main/busy/ms 39.03, stage/render/prepare_assets 20.68, stage/main/update 16.14, stage/render/render 14.95, stage/main/post_update 14.55, render/graph/ms 12.12, encode//sum 10.56, stage/render/graph_submit 7.88, stream/drive/ms 5.39, encode/submit 4.90 |
| warm-up | 5.92 | 45.1 | render/thread/ms 55.32, render/busy/ms 53.82, main/busy/ms 37.18, stage/render/prepare_assets 31.75, stage/main/post_update 15.34, stage/main/update 13.31, stage/render/render 11.74, render/graph/ms 9.64, encode//sum 7.97, stage/render/graph_submit 6.15, stream/drive/ms 5.86, encode/submit 4.03 |
| warm-up | 5.96 | 36.6 | pipelines/compile_ms 144, pipelines/front_ms 141, render/thread/ms 54.73, render/busy/ms 53.61, stage/render/prepare_assets 39.15, main/busy/ms 30.36, stage/main/update 12.21, stage/main/post_update 11.94, stage/render/render 7.64, render/graph/ms 6.20, encode//sum 6.14, stage/render/graph_submit 4.21 |
| warm-up | 5.99 | 25.3 | pipelines/compile_ms 161, pipelines/front_ms 156, render/thread/ms 42.57, render/busy/ms 41.16, main/busy/ms 41.01, stage/main/update 25.35, stage/render/prepare_assets 22.59, stream/drive/ms 13.17, stage/render/render 11.79, render/graph/ms 9.96, stage/main/post_update 9.76, stage/render/graph_submit 7.85 |
| warm-up | 6.02 | 30.4 | main/busy/ms 43.97, render/thread/ms 36.64, render/busy/ms 35.18, stage/main/update 26.40, pipelines/compile_ms 25.18, pipelines/front_ms 23.09, stream/drive/ms 13.66, stage/render/prepare_assets 13.17, stage/render/render 12.65, stage/main/post_update 11.53, render/graph/ms 10.79, stage/render/graph_submit 8.54 |
| warm-up | 6.04 | 28.2 | render/thread/ms 37.47, render/busy/ms 36.29, main/busy/ms 35.91, stage/main/update 17.03, stage/render/prepare_assets 14.26, pipelines/compile_ms 14.10, pipelines/front_ms 13.12, stage/main/post_update 11.85, stage/render/render 9.51, render/graph/ms 7.82, stream/drive/ms 6.74, encode//sum 6.71 |
| warm-up | 6.19 | 27.4 | pipelines/compile_ms 143, pipelines/front_ms 140, main/busy/ms 29.71, render/thread/ms 24.07, render/busy/ms 23.06, stage/main/update 18.89, stream/drive/ms 14.13, stage/render/render 9.73, render/graph/ms 8.45, stage/main/post_update 7.27, stage/render/prepare_assets 7.13, encode//sum 6.74 |
| warm-up | 6.26 | 27.7 | main/busy/ms 31.20, render/thread/ms 25.45, render/busy/ms 24.42, stage/main/update 17.89, stage/render/render 10.73, stage/main/post_update 9.51, render/graph/ms 9.41, encode//sum 8.44, stage/render/prepare_assets 8.04, stage/render/graph_submit 7.07, encode/submit 4.39, encode/finish_wall 2.70 |
| warm-up | 6.49 | 28.4 | main/busy/ms 42.32, render/thread/ms 41.54, render/busy/ms 39.49, stage/main/update 21.34, stage/render/render 20.29, render/graph/ms 17.36, stage/main/post_update 14.20, stage/render/graph_submit 12.68, encode//sum 12.34, stream/drive/ms 12.14, stage/render/prepare_assets 11.19, encode/submit 7.69 |
| warm-up | 6.54 | 45.7 | pipelines/compile_ms 146, pipelines/front_ms 144, render/thread/ms 52.33, render/busy/ms 50.92, main/busy/ms 45.58, stage/render/prepare_assets 30.01, stage/main/update 22.87, stage/main/post_update 15.06, stage/render/render 12.83, stream/drive/ms 12.33, render/graph/ms 10.72, encode//sum 9.79 |
| warm-up | 6.85 | 30.9 | render/thread/ms 37.55, render/busy/ms 36.56, main/busy/ms 14.41, stage/render/render 10.43, stage/render/specialize 9.38, render/graph/ms 9.09, stage/main/post_update 7.41, stage/render/graph_submit 7.18, encode//sum 6.45, stage/render/queue 6.04, encode/submit 4.18, stage/main/update 3.90 |
| warm-up | 8.06 | 31.7 | pipelines/compile_ms 124, pipelines/front_ms 119, render/thread/ms 38.98, render/busy/ms 36.08, main/busy/ms 14.83, stage/render/render 11.22, render/graph/ms 9.84, stage/render/specialize 8.62, stage/main/post_update 8.37, stage/render/graph_submit 7.51, encode//sum 7.36, stage/render/queue 6.02 |
| measure | 0.85 | 27.7 | render/thread/ms 34.59, render/busy/ms 33.21, main/busy/ms 13.95, stage/render/render 11.29, render/graph/ms 9.89, encode//sum 9.03, stage/render/specialize 8.68, stage/render/graph_submit 7.96, stage/main/post_update 7.50, stage/render/queue 6.06, encode/submit 4.80, stage/main/update 3.62 |
| measure | 13.95 | 68.4 | render/thread/ms 76.33, render/busy/ms 73.67, encode//sum 69.56, stage/render/render 68.95, render/graph/ms 67.79, stage/render/graph_submit 65.78, encode/finish_wall 60.43, encode/pass:early prepass/sum 58.70, main/busy/ms 14.81, stage/main/post_update 9.38, encode/submit 5.39, stage/main/update 3.48 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set select.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=select-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 14%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **96%** (min 41%) · 2625 MHz (min 2625) · memory 8101 MHz · 96.9 W · 77 °C (max 80) · VRAM 6426 MiB · throttled 100% of samples

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
- `select.gate=on (command line: --set)`
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
