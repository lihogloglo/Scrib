# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1576 frames over 15.0 s · **median 9.50 ms (105 fps)** · mean 9.52 · p95 10.94 · p99 11.80 (1% low 85 fps) · worst 17.03 ms · 4 spikes over 1.5× median · 12.0 stutters/min (work 8.0/min)
- warm-up: 309 frames over 8.5 s, worst 2460.7 ms
- hitches (> 25.0 ms): **0** measured, 40 in warm-up
- timed GPU passes: 8.09 ms of a 9.52 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.50 ms, limit 11 ms
- pass worst frame 17.03 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.434 | 2.493 | 2.580 | 0.98 | 0.082 |
| dlss_super_resolution | 1.976 | 2.024 | 2.050 | 0.98 | 0.177 |
| early prepass | 0.637 | 0.652 | 0.667 | 0.98 | 0.099 |
| bloom | 0.307 | 0.314 | 0.332 | 0.98 | 0.061 |
| atmosphere_luts | 0.304 | 0.311 | 0.391 | 0.98 | 0.003 |
| main_transmissive_pass_3d | 0.295 | 0.302 | 0.331 | 0.98 | 0.009 |
| water_velocity | 0.234 | 0.239 | 0.265 | 0.98 | 0.002 |
| water_sspr | 0.205 | 0.210 | 0.213 | 0.98 | 0.002 |
| ssao | 0.185 | 0.189 | 0.198 | 0.98 | 0.007 |
| clouds_trace | 0.145 | 0.149 | 0.172 | 0.98 | 0.001 |
| atmos_froxel_inject | 0.142 | 0.145 | 0.152 | 0.98 | 0.001 |
| water_fft | 0.129 | 0.132 | 0.135 | 0.98 | 0.003 |
| uniform_allocation | 0.114 | 0.117 | 0.148 | 0.98 | 0.022 |
| upscaling | 0.111 | 0.114 | 0.119 | 0.98 | 0.004 |
| clouds_composite | 0.110 | 0.112 | 0.115 | 0.98 | 0.002 |
| bin_unpacking | 0.089 | 0.091 | 0.115 | 0.98 | 0.017 |
| tonemapping | 0.084 | 0.086 | 0.093 | 0.98 | 0.004 |
| gpu_instance_materialization | 0.077 | 0.079 | 0.091 | 0.98 | 0.004 |
| clustering | 0.065 | 0.066 | 0.078 | 0.98 | 0.020 |
| early_mesh_preprocessing | 0.061 | 0.062 | 0.089 | 0.98 | 0.004 |
| render_sky | 0.051 | 0.052 | 0.054 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.050 | 0.411 | 0.415 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.052 | 0.98 | 0.001 |
| clouds_shadow | 0.031 | 0.032 | 0.035 | 0.98 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.218 | 0.227 | 0.12 | 0.004 |
| water_snapshot | 0.023 | 0.024 | 0.025 | 0.98 | 0.002 |
| range_unpacking | 0.022 | 0.023 | 0.029 | 0.98 | 0.003 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.018 | 0.018 | 0.036 | 0.98 | 0.006 |
| surface_exposure_far | 0.012 | 0.039 | 0.041 | 0.30 | 0.002 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.98 | 0.002 |
| surface_exposure | 0.011 | 0.039 | 0.040 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.98 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.005 | 0.005 | 0.017 | 0.98 | 0.045 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.98 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.620 | 10.084 | 16.15 | 13584.9 |
| render/busy/ms | 7.227 | 8.559 | 15.39 | 11390.1 |
| main/busy/ms | 7.154 | 8.079 | 10.83 | 11275.0 |
| stage/render/render | 4.712 | 5.987 | 12.81 | 7425.9 |
| render/graph/ms | 4.121 | 5.292 | 12.26 | 6494.7 |
| stage/main/post_update | 4.045 | 4.816 | 7.33 | 6374.9 |
| encode//sum | 3.875 | 6.198 | 15.61 | 6107.3 |
| stage/render/graph_submit | 3.105 | 4.240 | 11.19 | 4892.9 |
| encode/submit | 1.954 | 2.699 | 7.77 | 3078.9 |
| stage/main/update | 1.845 | 2.278 | 2.81 | 2908.2 |
| encode/finish_wall | 1.197 | 1.764 | 7.77 | 1887.0 |
| stage/render/prepare_views | 1.066 | 2.696 | 3.79 | 1680.1 |
| render/acquire/ms | 1.019 | 2.651 | 3.75 | 1606.1 |
| stage/render/camera_driver | 0.954 | 1.084 | 1.56 | 1503.8 |
| encode/pass:shadow_directional_light/sum | 0.763 | 1.358 | 2.28 | 1202.2 |
| encode/pass:early prepass/sum | 0.740 | 1.045 | 7.24 | 1166.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.737 | 1.061 | 1.84 | 1161.4 |
| stage/render/prepare_write_phase_buffers | 0.538 | 0.754 | 0.95 | 847.6 |
| stage/render/prepare_assets | 0.448 | 0.591 | 1.78 | 706.7 |
| stage/render/prepare_resources | 0.397 | 0.456 | 1.08 | 625.2 |
| bigspace/total/ms | 0.388 | 1.018 | 2.43 | 610.8 |
| render/present/ms | 0.374 | 0.461 | 3.83 | 588.7 |
| stage/main/pre_update | 0.353 | 0.478 | 0.85 | 555.8 |
| stage/main/first | 0.294 | 0.361 | 1.67 | 464.0 |
| stage/render/prepare_flush | 0.268 | 0.397 | 0.71 | 422.0 |
| stage/render/prepare_bind_groups | 0.266 | 0.368 | 0.61 | 419.4 |
| stage/main/last | 0.250 | 0.305 | 0.78 | 393.2 |
| terrain/update/ms | 0.236 | 0.306 | 1.21 | 371.4 |
| stage/render/prepare_batch_phases | 0.213 | 0.310 | 0.43 | 335.2 |
| stage/render/prepare_meshes | 0.211 | 0.270 | 1.04 | 332.4 |
| terrain/select/ms | 0.205 | 0.266 | 1.06 | 323.0 |
| pipelines/compile_ms | 0.191 | 0.000 | 132.24 | 301.5 |
| stream/drive/ms | 0.190 | 0.269 | 0.62 | 298.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.188 | 0.307 | 1.30 | 295.6 |
| pipelines/front_ms | 0.187 | 0.000 | 129.44 | 294.2 |
| stage/main/fixed_main_loop | 0.184 | 0.357 | 1.03 | 289.9 |
| stage/render/extract_commands | 0.183 | 0.221 | 0.48 | 289.1 |
| far/select/ms | 0.170 | 0.231 | 0.72 | 268.2 |
| stage/render/create_views | 0.163 | 0.207 | 0.89 | 257.6 |
| stream/tick/ms | 0.161 | 0.237 | 0.56 | 254.4 |
| controller/feed/ms | 0.157 | 0.203 | 0.57 | 247.3 |
| bigspace/high_precision/ms | 0.154 | 0.608 | 1.53 | 243.0 |
| controller/move/ms | 0.143 | 0.251 | 0.57 | 225.4 |
| encode/pass:main_transmissive_pass_3d/sum | 0.120 | 0.195 | 1.13 | 189.0 |
| bigspace/low_precision/ms | 0.107 | 0.288 | 2.19 | 169.3 |
| stage/render/specialize | 0.094 | 0.121 | 3.65 | 148.4 |
| anim/evaluate/ms | 0.089 | 0.247 | 1.47 | 140.0 |
| stage/render/queue | 0.061 | 0.085 | 2.34 | 95.5 |
| anim/evaluate_cpu/ms | 0.060 | 0.075 | 0.24 | 94.5 |
| stage/render/phase_sort | 0.056 | 0.076 | 0.22 | 87.8 |
| encode/resolve_diagnostics/sum | 0.050 | 0.092 | 1.12 | 78.1 |
| stage/render/prepare_collect_phase_buffers | 0.044 | 0.073 | 0.19 | 69.9 |
| physics/step/ms | 0.029 | 0.059 | 0.47 | 45.3 |
| weather/map/ms | 0.025 | 0.034 | 0.13 | 39.5 |
| stage/render/cleanup | 0.022 | 0.031 | 0.31 | 35.0 |
| encode/camera_driver_span_begin/sum | 0.021 | 0.037 | 0.22 | 32.7 |
| encode/camera_driver_span_end/sum | 0.018 | 0.035 | 0.11 | 28.1 |
| weather/eval/ms | 0.011 | 0.014 | 0.05 | 17.2 |
| script/update/ms | 0.008 | 0.011 | 0.04 | 12.2 |
| controller/avoid/ms | 0.006 | 0.008 | 0.40 | 10.2 |
| script/input/ms | 0.006 | 0.007 | 0.03 | 8.8 |
| pipelines/driver_ms | 0.005 | 0.000 | 2.80 | 7.2 |
| anim/apply/ms | 0.004 | 0.006 | 0.12 | 7.0 |
| script/ui/ms | 0.004 | 0.005 | 0.35 | 6.5 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 1.2 |
| stage/render/prepare | 0.001 | 0.002 | 0.03 | 0.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 0.6 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.1 |
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
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.4 k | 37.4 k | 37.4 k |
| encode//max | ms | 0.49 | 7.33 | 0.37 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.22 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.11 | 0.01 |
| encode/finish_critical | ms | 0.85 | 7.33 | 0.75 |
| encode/pass:early prepass/max | ms | 0.71 | 7.24 | 0.59 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.72 | 1.84 | 0.75 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 1.13 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.17 | 1.30 | 0.21 |
| encode/pass:shadow_directional_light/max | ms | 0.67 | 2.28 | 0.44 |
| encode/resolve_diagnostics/max | ms | 0.04 | 1.12 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 8.94 | 15.39 | 8.89 |
| gpu/timed/ms | ms | 8.94 | 9.66 | 8.89 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1929 | 1929 | 1929 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 214.7 k | 214.7 k | 214.7 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 10.8 k | 10.8 k | 10.8 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 321.65 M | 321.65 M | 321.65 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d/bins | count | 249 | 249 | 249 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 563 | 563 | 563 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 320 | 320 | 320 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 249 | 249 | 249 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 563 | 563 | 563 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 320 | 320 | 320 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 23 | 23 | 23 |
| phases/opaque3d/bins | count | 877 | 877 | 877 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/opaque3d_prepass/bins | count | 877 | 877 | 877 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 4 | 41 | 4 |
| phases/shadow/bins | count | 896 | 1509 | 896 |
| phases/shadow/instance_batches | count | 4 | 12 | 4 |
| phases/shadow/instance_draws | count | 5861 | 17.6 k | 5861 |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| phases/shadow/items | count | 3202 | 8161 | 3202 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 27 | 34 | 27 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 319 | 319 | 319 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 1 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 25.96 | 39.42 | 28.44 |
| process/mem_usage | GiB | 4.67 | 4.75 | 4.75 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2386 | 2386 | 2386 |
| scene/batch/masked/live | count | 2386 | 2386 | 2386 |
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
| scene/batch/opaque/high_water | count | 16.8 k | 16.8 k | 16.8 k |
| scene/batch/opaque/live | count | 16.8 k | 16.8 k | 16.8 k |
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
| scene/cells/resident | count | 101 | 101 | 101 |
| scene/chains/high_water | count | 1639 | 1639 | 1639 |
| scene/chains/live | count | 1639 | 1639 | 1639 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 5 | 7 | 6 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 181 | 181 | 181 |
| skin/joint_bytes_written/bytes | bytes | 309.5 k | 309.5 k | 309.5 k |
| skin/joints_written/count | count | 4836 | 4836 | 4836 |
| skin/upload_writes/count | count | 1 | 1 | 1 |
| started_load_count |  | 5286 | 5288 | 5288 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 174 | 174 | 174 |
| statics/materials_bound | count | 174 | 174 | 174 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 187 | 187 | 187 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.88 | 40.13 | 30.62 |
| system/mem_usage | % | 54.30 | 54.58 | 54.57 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2552 | 2552 | 2552 |
| textures/images | count | 1998 | 1998 | 1998 |
| views/all/batch_sets | count | 64 | 101 | 64 |
| views/all/bins | count | 3148 | 3761 | 3148 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 12 | 20 | 12 |
| views/all/instance_draws | count | 17.6 k | 29.3 k | 17.6 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 147.5 k |
| views/all/items | count | 9462 | 14.4 k | 9462 |
| views/all/pipelines | count | 75 | 82 | 75 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 153.8 k | 260.3 k | 153.8 k |
| views/cascade/batch_sets | count | 4 | 41 | 4 |
| views/cascade/bins | count | 896 | 1509 | 896 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 4 |
| views/cascade/instance_draws | count | 5861 | 17.6 k | 5861 |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| views/cascade/items | count | 3202 | 8161 | 3202 |
| views/cascade/pipelines | count | 27 | 34 | 27 |
| views/cascade/work_items | count | 49.2 k | 155.6 k | 49.2 k |
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
| views/main/batch_sets | count | 60 | 60 | 60 |
| views/main/bins | count | 2252 | 2252 | 2252 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 11.7 k | 11.7 k | 11.7 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6260 | 6260 | 6260 |
| views/main/pipelines | count | 48 | 48 | 48 |
| views/main/work_items | count | 104.7 k | 104.7 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 15.1 k | 15.1 k | 15.1 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 2 | 151 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.14 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 27.55 | 30 | 43.4 k |
| encode/bind_walk/init_textures | count | 7095 | 13.9 k | 11.18 M |
| encode/bind_walk/init_textures_skipped | count | 35.2 k | 48.2 k | 55.51 M |
| encode/bind_walk/inits | count | 330 | 435 | 519.9 k |
| encode/bind_walk/inits_skipped | count | 47 | 59 | 74.1 k |
| encode/bind_walk/merge_textures | count | 7032 | 13.8 k | 11.08 M |
| encode/bind_walk/merge_textures_skipped | count | 3925 | 3925 | 6.19 M |
| encode/bind_walk/merges | count | 79.26 | 100 | 124.9 k |
| encode/bind_walk/merges_skipped | count | 30.75 | 36 | 48.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1576 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1576 |
| encode/pass:early prepass/count | count | 1 | 1 | 1576 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1576 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1576 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1576 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1576 |
| encode/queue/command_buffers | count | 37.55 | 40 | 59.2 k |
| encode/queue/submits | count | 2 | 2 | 3152 |
| encode/queue/write_buffer/bytes | bytes | 2.21 M | 3.83 M | 3.48 G |
| encode/queue/write_buffer/calls | count | 229 | 303 | 361.4 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 51.44 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 14.2 k |
| encode/queue/write_texture/bytes | bytes | 11.4 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1576 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.57 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 4 |
| pipelines/created | count | 0.00 | 1 | 3 |
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

## Hitches (> 25.0 ms; slowest 40, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.46 | 2460.7 | main/busy/ms 2432, stage/main/post_update 2323, stage/main/update 97.72, surface/advance_cpu_ms 65.85, stage/main/pre_update 6.62, stage/main/last 1.07, stage/main/fixed_main_loop 0.84, surface/cell_upload_ms 0.67, stage/main/first 0.59, stream/drive/ms 0.35, weather/map/ms 0.17, stream/tick/ms 0.11 |
| warm-up | 3.19 | 733.6 | main/busy/ms 2562, stage/main/post_update 2347, render/thread/ms 703, render/busy/ms 526, stage/render/prepare_assets 479, stage/main/update 198, stage/render/prepare_views 172, render/acquire/ms 172, surface/advance_cpu_ms 65.85, stage/render/create_views 27.42, stage/render/render 14.99, render/graph/ms 8.20 |
| warm-up | 3.37 | 173.0 | render/thread/ms 859, render/busy/ms 591, stage/render/prepare_assets 529, stage/render/prepare_views 172, render/acquire/ms 172, main/busy/ms 161, stage/render/render 111, stage/main/update 107, render/present/ms 96.25, stage/main/post_update 40.59, stage/render/create_views 27.81, pipelines/compile_ms 24.31 |
| warm-up | 3.41 | 45.6 | pipelines/compile_ms 464, pipelines/front_ms 322, pipelines/wait_ms 307, render/thread/ms 200, pipelines/driver_ms 142, stage/render/render 132, render/busy/ms 107, main/busy/ms 104, render/present/ms 91.82, stage/main/update 57.15, stage/render/prepare_assets 51.36, render/graph/ms 39.50 |
| warm-up | 3.56 | 148.7 | pipelines/wait_ms 4713, pipelines/compile_ms 2214, pipelines/front_ms 1950, pipelines/driver_ms 265, render/thread/ms 157, render/busy/ms 154, main/busy/ms 125, stage/render/prepare_assets 103, stage/main/update 53.87, stage/main/post_update 49.29, stage/render/render 42.29, render/graph/ms 39.14 |
| warm-up | 3.60 | 42.1 | pipelines/wait_ms 4457, pipelines/compile_ms 2126, pipelines/front_ms 1987, render/thread/ms 153, render/busy/ms 151, pipelines/driver_ms 140, stage/render/prepare_assets 122, main/busy/ms 81.46, stage/main/post_update 50.01, terrain/update/ms 15.48, stage/main/pre_update 15.08, stage/render/render 10.19 |
| warm-up | 3.65 | 45.4 | pipelines/compile_ms 758, pipelines/front_ms 721, pipelines/wait_ms 154, render/thread/ms 82.79, render/busy/ms 81.16, stage/render/prepare_assets 52.58, main/busy/ms 43.82, pipelines/driver_ms 37.08, stage/main/post_update 27.50, terrain/update/ms 13.95, stage/main/update 9.07, stage/render/render 8.79 |
| warm-up | 3.68 | 27.2 | pipelines/compile_ms 446, pipelines/front_ms 418, pipelines/wait_ms 104, render/thread/ms 68.83, render/busy/ms 67.12, stage/render/prepare_assets 38.22, main/busy/ms 33.37, pipelines/driver_ms 27.59, stage/main/post_update 17.29, encode//sum 17.12, stage/render/render 13.50, render/graph/ms 11.51 |
| warm-up | 3.75 | 39.9 | pipelines/compile_ms 979, pipelines/front_ms 936, main/busy/ms 66.13, render/thread/ms 53.96, render/busy/ms 51.48, pipelines/driver_ms 42.66, stage/main/update 39.60, stage/main/post_update 21.53, pipelines/wait_ms 19.77, stage/render/render 19.04, render/graph/ms 15.84, stage/render/graph_submit 13.03 |
| warm-up | 3.83 | 82.5 | pipelines/compile_ms 2033, pipelines/front_ms 1932, pipelines/wait_ms 116, main/busy/ms 109, pipelines/driver_ms 101, render/thread/ms 96.49, render/busy/ms 93.30, stage/main/update 87.31, stage/render/prepare_assets 52.82, stage/render/render 25.22, render/graph/ms 21.26, stage/render/graph_submit 17.51 |
| warm-up | 3.94 | 107.4 | pipelines/compile_ms 1631, pipelines/front_ms 1555, render/thread/ms 119, render/busy/ms 116, pipelines/wait_ms 109, stage/render/prepare_assets 91.25, main/busy/ms 80.02, pipelines/driver_ms 75.84, stage/main/update 62.57, stage/render/render 18.12, render/graph/ms 15.00, pipelines/lock_ms 12.08 |
| warm-up | 4.05 | 104.8 | pipelines/compile_ms 1266, pipelines/front_ms 1183, render/thread/ms 139, render/busy/ms 138, stage/render/prepare_assets 123, pipelines/driver_ms 82.92, main/busy/ms 21.65, pipelines/wait_ms 9.67, stage/render/render 9.42, pipelines/lock_ms 8.80, render/graph/ms 7.58, stage/main/update 6.73 |
| warm-up | 4.13 | 70.2 | pipelines/compile_ms 875, pipelines/front_ms 837, render/thread/ms 77.24, stage/render/prepare_views 40.08, render/acquire/ms 39.93, pipelines/driver_ms 37.70, render/busy/ms 35.59, stage/render/render 29.81, render/graph/ms 27.59, stage/render/graph_submit 25.41, encode/submit 22.26, main/busy/ms 18.72 |
| warm-up | 4.19 | 56.8 | main/busy/ms 51.24, stage/main/update 41.50, stream/drive/ms 36.32, render/thread/ms 21.08, render/busy/ms 19.95, stage/render/prepare_assets 9.74, stage/render/render 7.21, encode//sum 6.35, stage/main/post_update 5.73, render/graph/ms 5.57, stage/render/graph_submit 3.40, stage/render/camera_driver 2.01 |
| warm-up | 4.35 | 137.3 | pipelines/compile_ms 1070, pipelines/front_ms 1038, main/busy/ms 120, stage/main/update 109, render/thread/ms 51.42, render/busy/ms 49.22, pipelines/driver_ms 32.71, stage/render/prepare_assets 26.84, stream/drive/ms 25.84, stage/render/render 13.18, render/graph/ms 11.10, stage/render/graph_submit 8.04 |
| warm-up | 4.38 | 27.6 | pipelines/compile_ms 1140, pipelines/front_ms 1100, render/thread/ms 58.40, render/busy/ms 55.74, main/busy/ms 42.11, pipelines/driver_ms 39.07, stage/render/prepare_assets 28.28, stage/main/update 23.03, stage/render/render 12.91, stage/main/post_update 10.11, render/graph/ms 9.99, stage/render/graph_submit 7.34 |
| warm-up | 4.43 | 51.8 | pipelines/compile_ms 249, pipelines/front_ms 235, main/busy/ms 75.81, render/thread/ms 61.92, render/busy/ms 51.89, stage/main/update 50.10, stage/render/render 23.08, stage/render/prepare_assets 19.62, stage/main/post_update 15.51, pipelines/driver_ms 14.22, render/graph/ms 12.62, encode//sum 9.57 |
| warm-up | 4.53 | 101.3 | render/thread/ms 98.71, render/busy/ms 89.94, main/busy/ms 87.11, stage/render/prepare_assets 50.61, stage/main/update 42.02, stage/main/post_update 28.10, pipelines/compile_ms 26.79, pipelines/front_ms 25.35, stage/render/render 19.04, stage/main/pre_update 11.30, render/graph/ms 9.77, encode//sum 8.70 |
| warm-up | 4.58 | 53.5 | pipelines/compile_ms 883, pipelines/front_ms 855, render/thread/ms 104, render/busy/ms 102, stage/render/prepare_assets 62.66, main/busy/ms 59.90, pipelines/driver_ms 27.57, stage/main/post_update 22.04, stage/main/update 18.48, pipelines/wait_ms 14.30, stage/main/pre_update 11.09, stage/render/render 8.89 |
| warm-up | 4.66 | 74.5 | pipelines/compile_ms 3201, pipelines/front_ms 3078, pipelines/wait_ms 368, pipelines/driver_ms 123, render/thread/ms 109, render/busy/ms 107, stage/render/prepare_assets 71.23, main/busy/ms 62.12, stage/main/post_update 28.33, stage/main/update 22.40, stage/render/render 12.32, render/graph/ms 9.99 |
| warm-up | 4.72 | 59.8 | pipelines/compile_ms 2532, pipelines/front_ms 2425, pipelines/wait_ms 441, render/thread/ms 115, render/busy/ms 112, pipelines/driver_ms 107, main/busy/ms 83.99, stage/main/post_update 52.85, stage/render/prepare_assets 50.60, stage/render/prepare_flush 22.12, stage/main/update 19.13, stage/render/render 13.71 |
| warm-up | 4.77 | 54.9 | pipelines/compile_ms 436, pipelines/front_ms 410, pipelines/wait_ms 142, render/thread/ms 88.01, render/busy/ms 86.32, main/busy/ms 69.91, stage/main/post_update 41.04, stage/render/prepare_assets 30.54, pipelines/driver_ms 25.32, stage/render/prepare_flush 22.36, stage/main/update 18.00, stage/render/render 12.09 |
| warm-up | 4.83 | 61.1 | pipelines/compile_ms 883, pipelines/front_ms 845, pipelines/wait_ms 135, render/thread/ms 86.23, render/busy/ms 84.58, main/busy/ms 62.18, stage/render/prepare_assets 43.05, pipelines/driver_ms 37.51, stage/main/update 27.56, stage/main/post_update 25.10, stage/render/render 16.87, render/graph/ms 14.76 |
| warm-up | 4.88 | 49.7 | pipelines/compile_ms 1494, pipelines/front_ms 1447, pipelines/wait_ms 127, main/busy/ms 94.00, render/thread/ms 85.46, render/busy/ms 83.43, stage/main/update 62.09, pipelines/driver_ms 47.18, stage/render/prepare_assets 28.59, stage/render/render 24.47, stage/main/post_update 22.12, render/graph/ms 21.97 |
| warm-up | 4.96 | 76.1 | pipelines/compile_ms 2168, pipelines/front_ms 2104, pipelines/wait_ms 128, main/busy/ms 89.63, render/thread/ms 76.71, render/busy/ms 74.83, stage/main/update 65.03, pipelines/driver_ms 63.48, stage/render/prepare_assets 31.38, stage/render/render 21.38, render/graph/ms 18.54, stage/main/post_update 14.51 |
| warm-up | 5.01 | 51.2 | pipelines/compile_ms 1549, pipelines/front_ms 1504, pipelines/wait_ms 82.78, render/thread/ms 77.90, render/busy/ms 76.43, main/busy/ms 66.31, pipelines/driver_ms 45.28, stage/render/prepare_assets 34.44, stage/main/update 32.77, stage/main/post_update 20.15, stream/drive/ms 14.37, stage/render/render 12.57 |
| warm-up | 5.05 | 37.0 | pipelines/compile_ms 289, pipelines/front_ms 280, render/thread/ms 74.97, render/busy/ms 73.59, main/busy/ms 72.93, stage/main/update 38.96, stage/main/post_update 22.82, stage/render/prepare_assets 14.25, stage/render/render 11.26, stage/render/prepare_resources 10.54, stage/render/prepare_bind_groups 9.98, render/graph/ms 9.49 |
| warm-up | 5.10 | 55.1 | pipelines/compile_ms 290, pipelines/front_ms 281, render/thread/ms 82.28, render/busy/ms 80.93, main/busy/ms 72.69, stage/main/update 33.27, stage/main/post_update 26.97, stage/render/prepare_resources 24.34, stage/render/prepare_assets 11.87, stage/render/render 11.37, render/graph/ms 9.60, pipelines/driver_ms 9.24 |
| warm-up | 5.15 | 46.1 | pipelines/compile_ms 810, pipelines/front_ms 790, render/thread/ms 91.29, render/busy/ms 89.99, main/busy/ms 61.24, stage/main/update 26.21, stage/render/prepare_write_phase_buffers 22.31, stage/main/post_update 22.19, stage/render/prepare_resources 22.00, pipelines/driver_ms 20.69, stream/drive/ms 12.72, stage/render/render 12.28 |
| warm-up | 5.24 | 32.8 | pipelines/compile_ms 665, pipelines/front_ms 643, pipelines/wait_ms 76.14, main/busy/ms 37.40, render/thread/ms 37.12, render/busy/ms 35.38, stage/main/update 25.05, pipelines/driver_ms 21.67, stage/render/render 12.18, render/graph/ms 10.24, stage/render/prepare_assets 9.80, stage/main/post_update 7.32 |
| warm-up | 5.31 | 25.2 | pipelines/compile_ms 330, pipelines/front_ms 316, pipelines/wait_ms 44.67, render/thread/ms 38.17, main/busy/ms 37.11, render/busy/ms 36.10, stage/render/render 17.99, stage/main/post_update 17.60, render/graph/ms 16.06, stage/main/update 14.55, pipelines/driver_ms 14.24, stage/render/graph_submit 12.12 |
| warm-up | 5.38 | 29.5 | pipelines/compile_ms 530, pipelines/front_ms 513, render/thread/ms 43.77, main/busy/ms 43.09, render/busy/ms 41.95, stage/main/update 22.63, stage/render/render 19.75, pipelines/driver_ms 16.18, render/graph/ms 15.73, stage/main/post_update 15.32, stage/render/graph_submit 11.56, encode//sum 8.85 |
| warm-up | 5.42 | 36.9 | pipelines/compile_ms 357, pipelines/front_ms 344, render/thread/ms 51.72, render/busy/ms 49.80, main/busy/ms 49.79, stage/main/update 28.75, stage/render/render 20.75, render/graph/ms 16.62, stage/main/post_update 15.09, stage/render/graph_submit 12.68, pipelines/driver_ms 12.41, stage/render/prepare_assets 11.57 |
| warm-up | 5.45 | 32.3 | pipelines/compile_ms 379, pipelines/front_ms 368, render/thread/ms 45.69, render/busy/ms 44.36, main/busy/ms 34.96, stage/main/update 20.86, stage/render/prepare_assets 17.67, stage/render/render 11.60, pipelines/driver_ms 10.29, render/graph/ms 9.84, encode//sum 8.20, stage/main/post_update 7.91 |
| warm-up | 5.64 | 34.4 | render/thread/ms 49.69, render/busy/ms 47.59, main/busy/ms 34.08, pipelines/compile_ms 33.18, pipelines/front_ms 31.08, stage/render/render 18.33, stage/render/prepare_assets 16.29, render/graph/ms 15.35, stage/main/post_update 14.59, stage/main/update 13.79, encode//sum 12.99, stage/render/graph_submit 11.22 |
| warm-up | 5.68 | 39.9 | render/thread/ms 52.25, render/busy/ms 50.69, pipelines/compile_ms 33.18, main/busy/ms 32.50, pipelines/front_ms 31.08, stage/render/prepare_assets 26.06, stage/main/update 16.56, stage/render/render 13.19, render/graph/ms 10.84, stage/main/post_update 10.52, encode//sum 10.26, stage/render/graph_submit 7.93 |
| warm-up | 5.72 | 36.5 | render/thread/ms 45.82, render/busy/ms 44.86, stage/render/prepare_assets 27.99, main/busy/ms 24.10, stage/main/update 11.07, stage/main/post_update 7.61, stage/render/render 7.29, encode//sum 6.24, render/graph/ms 5.94, stage/render/graph_submit 3.91, stage/render/prepare_meshes 3.73, encode/submit 2.28 |
| warm-up | 6.40 | 26.3 | render/thread/ms 40.79, render/busy/ms 39.25, main/busy/ms 35.24, stage/main/update 15.86, stage/main/post_update 13.98, stage/render/render 13.09, render/graph/ms 10.96, stage/render/prepare_assets 10.76, encode//sum 8.34, stage/render/graph_submit 7.66, pipelines/compile_ms 5.47, pipelines/front_ms 5.20 |
| warm-up | 6.44 | 35.9 | render/thread/ms 43.04, render/busy/ms 42.04, main/busy/ms 28.12, stage/render/prepare_assets 20.75, stage/main/update 12.27, stage/main/post_update 9.98, stage/render/render 7.26, stage/render/prepare_meshes 7.14, encode//sum 6.08, render/graph/ms 5.92, stage/render/graph_submit 3.71, stream/drive/ms 3.51 |
| warm-up | 6.52 | 29.1 | pipelines/compile_ms 148, pipelines/front_ms 144, render/thread/ms 39.85, render/busy/ms 38.37, main/busy/ms 33.13, stage/main/update 19.84, stage/render/prepare_assets 14.48, stage/render/render 14.25, render/graph/ms 12.38, stage/main/post_update 9.75, stage/render/graph_submit 8.89, encode//sum 8.64 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=scene-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 11%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 92%) · 2622 MHz (min 2610) · memory 8101 MHz · 97.7 W · 79 °C (max 82) · VRAM 6439 MiB · throttled 100% of samples

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
