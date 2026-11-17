# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); select.gate=on (command line: --set); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1548 frames over 15.0 s · **median 9.70 ms (103 fps)** · mean 9.69 · p95 11.07 · p99 11.75 (1% low 85 fps) · worst 27.31 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 277 frames over 8.6 s, worst 2576.1 ms
- hitches (> 25.0 ms): **1** measured, 52 in warm-up
- timed GPU passes: 8.52 ms of a 9.69 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.70 ms, limit 11 ms
- pass worst frame 27.31 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.886 | 2.899 | 2.949 | 1.00 | 0.063 |
| dlss_super_resolution | 2.012 | 2.021 | 2.044 | 1.00 | 0.181 |
| early prepass | 0.603 | 0.606 | 0.616 | 1.00 | 0.094 |
| atmosphere_luts | 0.326 | 0.328 | 0.393 | 1.00 | 0.003 |
| bloom | 0.314 | 0.315 | 0.331 | 1.00 | 0.061 |
| main_transmissive_pass_3d | 0.308 | 0.309 | 0.331 | 1.00 | 0.009 |
| water_velocity | 0.238 | 0.239 | 0.263 | 1.00 | 0.002 |
| water_sspr | 0.214 | 0.215 | 0.213 | 1.00 | 0.002 |
| ssao | 0.190 | 0.191 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.148 | 0.149 | 0.169 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.145 | 0.146 | 0.152 | 1.00 | 0.001 |
| water_fft | 0.131 | 0.131 | 0.135 | 1.00 | 0.003 |
| uniform_allocation | 0.120 | 0.121 | 0.158 | 1.00 | 0.022 |
| upscaling | 0.113 | 0.113 | 0.119 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.091 | 0.091 | 0.123 | 1.00 | 0.016 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.065 | 0.065 | 0.075 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.412 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.032 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.032 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.221 | 0.228 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.026 | 0.028 | 1.00 | 0.081 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.047 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.026 | 1.00 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.011 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.742 | 10.199 | 26.03 | 13532.0 |
| render/busy/ms | 6.835 | 7.844 | 25.53 | 10580.0 |
| main/busy/ms | 6.748 | 7.668 | 9.07 | 10445.2 |
| stage/render/render | 4.542 | 5.389 | 12.14 | 7030.7 |
| stage/main/post_update | 4.041 | 4.826 | 6.19 | 6255.2 |
| render/graph/ms | 3.963 | 4.739 | 7.55 | 6134.9 |
| encode//sum | 3.802 | 5.071 | 12.70 | 5886.2 |
| stage/render/graph_submit | 2.977 | 3.741 | 6.62 | 4608.3 |
| encode/submit | 1.856 | 2.319 | 9.66 | 2873.0 |
| stage/main/update | 1.601 | 2.045 | 3.22 | 2477.7 |
| stage/render/prepare_views | 1.592 | 3.230 | 4.13 | 2464.4 |
| render/acquire/ms | 1.547 | 3.180 | 4.09 | 2395.5 |
| encode/finish_wall | 1.142 | 1.595 | 3.23 | 1768.4 |
| stage/render/camera_driver | 0.929 | 1.058 | 1.38 | 1438.5 |
| encode/pass:shadow_directional_light/sum | 0.738 | 1.271 | 3.16 | 1142.8 |
| encode/pass:early prepass/sum | 0.701 | 0.975 | 2.61 | 1084.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.697 | 0.993 | 1.33 | 1079.0 |
| stage/render/prepare_write_phase_buffers | 0.540 | 0.758 | 1.05 | 836.6 |
| stage/render/prepare_assets | 0.448 | 0.569 | 1.52 | 692.8 |
| stage/render/prepare_resources | 0.382 | 0.448 | 0.59 | 591.4 |
| bigspace/total/ms | 0.360 | 0.743 | 1.39 | 558.0 |
| render/present/ms | 0.360 | 0.430 | 4.38 | 556.5 |
| stage/main/pre_update | 0.340 | 0.427 | 0.82 | 525.6 |
| stage/main/first | 0.291 | 0.350 | 1.65 | 450.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.232 | 0.333 | 0.64 | 359.4 |
| stage/render/prepare_bind_groups | 0.207 | 0.281 | 0.48 | 320.5 |
| stage/render/prepare_meshes | 0.201 | 0.248 | 2.98 | 311.7 |
| stage/main/fixed_main_loop | 0.176 | 0.320 | 0.89 | 272.1 |
| stage/render/prepare_batch_phases | 0.174 | 0.236 | 0.37 | 269.6 |
| stage/render/extract_commands | 0.171 | 0.201 | 0.35 | 265.4 |
| stage/render/create_views | 0.166 | 0.212 | 0.80 | 257.7 |
| controller/move/ms | 0.159 | 0.289 | 0.63 | 246.0 |
| stage/render/prepare_flush | 0.153 | 0.281 | 0.58 | 236.2 |
| controller/feed/ms | 0.150 | 0.197 | 0.62 | 232.4 |
| stream/drive/ms | 0.143 | 0.196 | 0.62 | 221.5 |
| bigspace/high_precision/ms | 0.128 | 0.301 | 1.22 | 197.5 |
| stage/main/last | 0.118 | 0.161 | 0.68 | 182.4 |
| encode/pass:main_transmissive_pass_3d/sum | 0.116 | 0.179 | 0.37 | 179.9 |
| stream/tick/ms | 0.115 | 0.161 | 0.54 | 177.9 |
| stage/render/queue | 0.108 | 0.141 | 5.66 | 167.6 |
| bigspace/low_precision/ms | 0.091 | 0.245 | 1.14 | 141.6 |
| anim/evaluate/ms | 0.087 | 0.209 | 1.24 | 134.7 |
| stage/render/specialize | 0.082 | 0.102 | 8.77 | 126.6 |
| anim/evaluate_cpu/ms | 0.063 | 0.080 | 0.45 | 97.1 |
| stage/render/prepare_collect_phase_buffers | 0.059 | 0.117 | 0.19 | 91.8 |
| encode/resolve_diagnostics/sum | 0.054 | 0.078 | 0.18 | 84.1 |
| terrain/update/ms | 0.030 | 0.040 | 0.14 | 46.9 |
| physics/step/ms | 0.026 | 0.053 | 0.42 | 40.6 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.038 | 0.16 | 40.6 |
| weather/map/ms | 0.025 | 0.032 | 0.48 | 39.0 |
| encode/camera_driver_span_end/sum | 0.023 | 0.038 | 0.12 | 35.8 |
| stage/render/cleanup | 0.023 | 0.032 | 0.22 | 35.5 |
| stage/render/phase_sort | 0.018 | 0.033 | 0.18 | 27.8 |
| weather/eval/ms | 0.011 | 0.014 | 0.03 | 16.4 |
| script/update/ms | 0.007 | 0.008 | 0.38 | 10.3 |
| controller/avoid/ms | 0.006 | 0.007 | 0.38 | 8.7 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 8.2 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 6.8 |
| far/select/ms | 0.004 | 0.004 | 0.37 | 6.1 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 5.9 |
| controller/write/ms | 0.002 | 0.002 | 0.03 | 2.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 1.0 |
| terrain/select/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 0.6 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| encode//max | ms | 0.52 | 3.09 | 0.60 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.16 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.12 | 0.02 |
| encode/finish_critical | ms | 0.83 | 3.16 | 0.75 |
| encode/pass:early prepass/max | ms | 0.70 | 2.61 | 0.55 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.71 | 1.33 | 0.51 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.37 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 0.64 | 0.30 |
| encode/pass:shadow_directional_light/max | ms | 0.68 | 3.16 | 0.75 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.18 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.19 | 25.53 | 9.25 |
| gpu/timed/ms | ms | 9.19 | 10.96 | 9.25 |
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
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 0 | 62 | 0 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 127 | 127 | 127 |
| process/cpu_usage | % | 25.12 | 36.54 | 23.08 |
| process/mem_usage | GiB | 4.64 | 4.74 | 4.74 |
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
| skin/blocks_changed/count | count | 5 | 7 | 4 |
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
| system/cpu_usage | % | 29.87 | 36.54 | 29.22 |
| system/mem_usage | % | 60.12 | 60.45 | 60.45 |
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
| anim/events/count | count | 0.10 | 3 | 150 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.20 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 29 | 41.1 k |
| encode/bind_walk/init_textures | count | 7166 | 14.0 k | 11.09 M |
| encode/bind_walk/init_textures_skipped | count | 35.6 k | 48.7 k | 55.12 M |
| encode/bind_walk/inits | count | 345 | 437 | 534.6 k |
| encode/bind_walk/inits_skipped | count | 67 | 79 | 103.7 k |
| encode/bind_walk/merge_textures | count | 7102 | 13.9 k | 10.99 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.32 M |
| encode/bind_walk/merges | count | 82.26 | 103 | 127.3 k |
| encode/bind_walk/merges_skipped | count | 50.75 | 56 | 78.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1548 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1548 |
| encode/pass:early prepass/count | count | 1 | 1 | 1548 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1548 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1548 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1548 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1548 |
| encode/queue/command_buffers | count | 36.57 | 39 | 56.6 k |
| encode/queue/submits | count | 2 | 2 | 3096 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.65 M | 2.23 G |
| encode/queue/write_buffer/calls | count | 217 | 287 | 336.4 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.53 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.9 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1548 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
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

## Hitches (> 25.0 ms; slowest 53, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2576.1 | main/busy/ms 2546, stage/main/post_update 2438, stage/main/update 99.00, surface/advance_cpu_ms 66.77, stage/main/pre_update 4.69, stage/main/last 0.98, surface/cell_upload_ms 0.76, stage/main/first 0.48, stage/main/fixed_main_loop 0.46, stream/drive/ms 0.36, weather/map/ms 0.12, bigspace/total/ms 0.08 |
| warm-up | 3.32 | 739.7 | main/busy/ms 2685, stage/main/post_update 2462, render/thread/ms 708, render/busy/ms 537, stage/render/prepare_assets 489, stage/main/update 209, stage/render/prepare_views 166, render/acquire/ms 166, surface/advance_cpu_ms 66.77, stage/render/create_views 28.17, stage/render/render 14.56, render/graph/ms 8.03 |
| warm-up | 3.49 | 177.6 | render/thread/ms 867, render/busy/ms 604, stage/render/prepare_assets 540, stage/render/prepare_views 167, main/busy/ms 167, render/acquire/ms 166, stage/main/update 116, stage/render/render 112, render/present/ms 97.41, pipelines/compile_ms 46.63, pipelines/front_ms 42.72, stage/main/post_update 37.91 |
| warm-up | 3.53 | 40.5 | pipelines/compile_ms 495, pipelines/front_ms 346, pipelines/wait_ms 201, render/thread/ms 198, pipelines/driver_ms 149, main/busy/ms 140, stage/render/render 130, render/present/ms 111, render/busy/ms 87.05, stage/main/update 77.36, stage/render/prepare_assets 52.62, stage/main/post_update 49.67 |
| warm-up | 3.70 | 162.6 | pipelines/wait_ms 4311, pipelines/compile_ms 1991, pipelines/front_ms 1772, pipelines/driver_ms 219, main/busy/ms 156, render/thread/ms 122, render/busy/ms 102, stage/main/update 99.17, stage/render/prepare_assets 72.75, stage/main/post_update 48.84, stage/render/render 38.08, render/present/ms 19.49 |
| warm-up | 3.80 | 108.3 | pipelines/wait_ms 4599, pipelines/compile_ms 2856, pipelines/front_ms 2464, pipelines/driver_ms 392, render/thread/ms 188, render/busy/ms 185, stage/render/prepare_assets 165, main/busy/ms 62.73, stage/main/update 32.32, stage/main/post_update 24.11, encode//sum 16.95, terrain/update/ms 16.80 |
| warm-up | 3.83 | 26.7 | pipelines/compile_ms 1360, pipelines/front_ms 1039, pipelines/wait_ms 490, pipelines/driver_ms 321, render/thread/ms 130, render/busy/ms 127, stage/render/prepare_assets 108, main/busy/ms 31.18, encode//sum 17.34, stage/main/post_update 16.62, terrain/update/ms 16.43, stage/render/render 12.72 |
| warm-up | 3.89 | 62.6 | pipelines/compile_ms 1476, pipelines/front_ms 1362, pipelines/driver_ms 114, pipelines/wait_ms 92.51, render/thread/ms 81.45, render/busy/ms 79.90, main/busy/ms 41.87, stage/main/post_update 27.13, stage/render/prepare_resources 27.07, stage/render/render 24.87, render/graph/ms 23.03, stage/render/camera_driver 17.54 |
| warm-up | 3.96 | 58.0 | pipelines/compile_ms 455, pipelines/front_ms 437, pipelines/wait_ms 133, render/thread/ms 62.27, render/busy/ms 60.20, main/busy/ms 40.67, stage/render/prepare_assets 39.23, stage/main/post_update 20.26, pipelines/driver_ms 18.76, stage/render/render 15.31, stage/main/update 14.89, render/graph/ms 12.75 |
| warm-up | 3.99 | 25.6 | pipelines/compile_ms 943, pipelines/front_ms 909, pipelines/wait_ms 140, render/thread/ms 75.85, main/busy/ms 75.02, render/busy/ms 73.52, stage/main/update 50.65, stage/render/prepare_assets 42.76, stream/drive/ms 36.23, pipelines/driver_ms 34.41, stage/render/render 19.60, render/graph/ms 16.38 |
| warm-up | 4.06 | 70.1 | pipelines/compile_ms 876, pipelines/front_ms 852, render/thread/ms 63.26, render/busy/ms 61.29, main/busy/ms 58.21, stage/main/update 41.50, stream/drive/ms 36.08, stage/render/prepare_assets 35.45, pipelines/driver_ms 24.19, stage/render/render 15.13, render/graph/ms 12.41, terrain/update/ms 8.78 |
| warm-up | 4.14 | 68.1 | pipelines/compile_ms 580, pipelines/front_ms 564, render/thread/ms 53.80, render/busy/ms 52.59, stage/render/prepare_assets 41.41, main/busy/ms 33.33, stage/main/update 19.72, pipelines/driver_ms 16.25, stream/drive/ms 15.31, stage/main/post_update 8.03, stage/render/render 8.01, encode//sum 7.08 |
| warm-up | 4.23 | 69.0 | pipelines/compile_ms 362, pipelines/front_ms 352, render/thread/ms 54.92, render/busy/ms 52.71, main/busy/ms 46.14, stage/render/prepare_assets 32.34, stage/main/update 29.08, stream/drive/ms 19.11, stage/render/render 15.58, render/graph/ms 12.89, stage/main/post_update 9.67, pipelines/driver_ms 9.44 |
| warm-up | 4.29 | 59.0 | main/busy/ms 113, pipelines/compile_ms 103, pipelines/front_ms 98.50, stage/main/update 93.01, render/thread/ms 74.75, render/busy/ms 73.00, stream/drive/ms 57.21, stage/render/prepare_assets 50.47, stage/render/render 14.55, render/graph/ms 12.02, stage/main/post_update 11.08, stage/render/graph_submit 6.36 |
| warm-up | 4.42 | 136.8 | pipelines/compile_ms 1057, pipelines/front_ms 1021, main/busy/ms 113, render/thread/ms 94.06, render/busy/ms 92.46, stage/main/update 91.64, stage/render/prepare_assets 70.56, stream/drive/ms 57.23, pipelines/driver_ms 35.54, stage/render/render 12.25, stage/main/post_update 11.93, render/graph/ms 9.83 |
| warm-up | 4.50 | 74.7 | pipelines/compile_ms 1237, pipelines/front_ms 1190, render/thread/ms 119, render/busy/ms 117, stage/render/prepare_assets 82.27, pipelines/wait_ms 80.03, main/busy/ms 65.47, pipelines/driver_ms 47.03, stage/main/post_update 28.07, stage/main/update 26.98, stage/render/render 23.26, render/graph/ms 20.75 |
| warm-up | 4.58 | 76.6 | pipelines/compile_ms 386, pipelines/front_ms 363, pipelines/wait_ms 142, render/thread/ms 111, render/busy/ms 108, main/busy/ms 83.11, stage/render/prepare_assets 70.53, stage/main/update 32.39, stage/main/post_update 29.29, pipelines/driver_ms 23.05, stage/render/render 21.92, render/graph/ms 19.46 |
| warm-up | 4.63 | 52.9 | pipelines/compile_ms 516, pipelines/front_ms 497, render/thread/ms 83.57, render/busy/ms 81.94, pipelines/wait_ms 73.26, main/busy/ms 65.73, stage/render/prepare_assets 53.63, stage/main/update 24.57, stage/main/post_update 19.60, pipelines/driver_ms 18.89, stage/main/pre_update 16.84, stage/render/render 10.13 |
| warm-up | 4.66 | 32.6 | pipelines/compile_ms 1930, pipelines/front_ms 1871, pipelines/wait_ms 76.68, render/thread/ms 60.28, pipelines/driver_ms 59.19, render/busy/ms 58.55, main/busy/ms 42.16, stage/render/prepare_assets 31.06, stage/main/update 17.70, stage/main/post_update 15.92, stage/render/render 11.68, render/graph/ms 9.46 |
| warm-up | 4.71 | 52.4 | pipelines/compile_ms 2002, pipelines/front_ms 1941, pipelines/wait_ms 92.54, render/thread/ms 74.24, pipelines/driver_ms 61.57, render/busy/ms 53.94, stage/render/render 31.81, main/busy/ms 30.15, render/graph/ms 27.01, stage/render/graph_submit 23.70, encode/submit 19.76, encode//sum 16.62 |
| warm-up | 4.80 | 67.8 | pipelines/compile_ms 452, pipelines/front_ms 439, main/busy/ms 84.29, render/thread/ms 70.89, render/busy/ms 66.56, stage/main/update 64.03, pipelines/wait_ms 59.89, stream/drive/ms 57.79, stage/render/render 43.59, render/graph/ms 38.78, stage/render/graph_submit 36.12, encode/finish_wall 31.67 |
| warm-up | 4.87 | 68.2 | pipelines/compile_ms 835, pipelines/front_ms 807, main/busy/ms 133, stage/main/update 110, stream/drive/ms 104, render/thread/ms 97.27, render/busy/ms 93.05, stage/render/render 44.26, render/graph/ms 39.42, stage/render/graph_submit 36.94, encode/finish_wall 32.42, pipelines/driver_ms 28.09 |
| warm-up | 4.94 | 68.3 | pipelines/compile_ms 1832, pipelines/front_ms 1778, main/busy/ms 88.35, render/thread/ms 69.33, render/busy/ms 67.99, stage/main/update 57.56, pipelines/driver_ms 54.61, stream/drive/ms 50.47, pipelines/wait_ms 25.17, stage/main/post_update 22.60, stage/render/prepare_assets 20.13, stage/render/prepare_write_phase_buffers 13.24 |
| warm-up | 4.97 | 32.6 | pipelines/compile_ms 1631, pipelines/front_ms 1587, main/busy/ms 64.57, render/thread/ms 56.51, render/busy/ms 54.73, pipelines/driver_ms 43.23, stage/main/update 32.46, stage/main/post_update 19.89, stage/render/prepare_assets 15.30, stage/render/render 13.79, render/graph/ms 11.53, stage/render/prepare_batch_phases 11.10 |
| warm-up | 5.04 | 69.4 | pipelines/compile_ms 1023, pipelines/front_ms 993, main/busy/ms 77.87, render/thread/ms 70.69, render/busy/ms 68.83, stage/main/update 56.91, pipelines/wait_ms 49.07, pipelines/driver_ms 29.92, stage/render/prepare_assets 29.23, stream/drive/ms 14.99, stage/render/render 14.96, render/graph/ms 12.59 |
| warm-up | 5.07 | 31.1 | pipelines/compile_ms 1046, pipelines/front_ms 1018, pipelines/wait_ms 196, render/thread/ms 65.48, render/busy/ms 64.03, main/busy/ms 51.43, stage/main/update 35.62, stage/render/prepare_assets 33.90, pipelines/driver_ms 27.63, stream/drive/ms 15.39, stage/render/render 11.66, render/graph/ms 9.77 |
| warm-up | 5.12 | 28.6 | pipelines/compile_ms 303, pipelines/front_ms 291, render/thread/ms 37.11, render/busy/ms 35.64, main/busy/ms 33.87, pipelines/wait_ms 33.67, stage/main/update 18.95, stage/render/render 13.02, pipelines/driver_ms 11.67, render/graph/ms 11.11, stage/main/post_update 9.38, stage/render/graph_submit 6.78 |
| warm-up | 5.17 | 36.9 | pipelines/compile_ms 212, pipelines/front_ms 197, render/thread/ms 44.63, render/busy/ms 42.78, main/busy/ms 37.28, stage/main/update 22.07, stage/render/render 16.22, stream/drive/ms 15.17, pipelines/driver_ms 14.99, render/graph/ms 13.88, stage/render/graph_submit 10.23, encode//sum 10.11 |
| warm-up | 5.24 | 38.8 | pipelines/compile_ms 160, pipelines/front_ms 155, render/thread/ms 50.91, main/busy/ms 50.68, render/busy/ms 48.80, stage/main/update 28.40, stage/render/prepare_assets 19.20, stage/main/post_update 15.06, stage/render/render 14.74, render/graph/ms 12.77, stage/render/graph_submit 8.85, encode//sum 6.89 |
| warm-up | 5.28 | 45.5 | pipelines/compile_ms 830, pipelines/front_ms 809, render/thread/ms 63.31, render/busy/ms 61.72, main/busy/ms 55.10, stage/render/prepare_assets 34.17, stage/main/update 27.45, pipelines/driver_ms 21.22, stage/main/post_update 20.97, stage/render/render 14.02, render/graph/ms 11.62, stage/render/graph_submit 8.08 |
| warm-up | 5.32 | 35.5 | pipelines/compile_ms 1239, pipelines/front_ms 1210, render/thread/ms 64.63, render/busy/ms 62.66, main/busy/ms 39.61, pipelines/driver_ms 29.86, stage/render/prepare_assets 24.94, stage/main/post_update 18.74, stage/render/render 18.64, render/graph/ms 16.04, stage/main/update 14.66, stage/render/graph_submit 12.64 |
| warm-up | 5.38 | 60.8 | pipelines/compile_ms 946, pipelines/front_ms 922, render/thread/ms 79.72, render/busy/ms 77.93, main/busy/ms 58.31, stage/main/post_update 32.95, stage/render/prepare_resources 23.85, pipelines/driver_ms 23.79, stage/render/prepare_assets 22.29, stage/render/render 17.71, render/graph/ms 15.46, stage/main/update 14.06 |
| warm-up | 5.44 | 60.9 | pipelines/compile_ms 1325, pipelines/front_ms 1290, render/thread/ms 82.84, render/busy/ms 81.36, pipelines/wait_ms 68.54, main/busy/ms 62.15, stage/render/prepare_assets 36.21, pipelines/driver_ms 35.11, stage/main/post_update 32.64, stage/render/prepare_resources 22.88, stage/main/update 14.72, stage/render/render 10.05 |
| warm-up | 5.53 | 34.1 | pipelines/compile_ms 1001, pipelines/front_ms 975, render/thread/ms 34.76, main/busy/ms 33.36, render/busy/ms 33.07, pipelines/driver_ms 26.49, stage/main/update 18.90, stage/render/prepare_assets 14.29, stream/drive/ms 12.20, stage/render/render 10.00, stage/main/post_update 8.58, render/graph/ms 7.75 |
| warm-up | 5.57 | 29.0 | pipelines/compile_ms 625, pipelines/front_ms 612, pipelines/wait_ms 93.06, render/thread/ms 41.25, render/busy/ms 40.09, main/busy/ms 26.44, stage/render/prepare_assets 20.98, pipelines/driver_ms 12.32, stage/main/update 11.16, stage/render/render 10.25, stage/main/post_update 9.24, render/graph/ms 8.66 |
| warm-up | 5.63 | 35.4 | pipelines/compile_ms 204, pipelines/front_ms 201, render/thread/ms 46.41, render/busy/ms 45.09, main/busy/ms 35.96, stage/render/prepare_assets 23.46, stage/main/update 18.40, stage/render/render 12.58, stage/main/post_update 11.35, render/graph/ms 10.76, stage/render/graph_submit 7.77, encode//sum 7.57 |
| warm-up | 5.69 | 41.5 | pipelines/compile_ms 58.05, render/thread/ms 55.94, render/busy/ms 54.69, pipelines/front_ms 54.69, main/busy/ms 31.22, stage/render/prepare_assets 25.58, stage/render/render 19.06, render/graph/ms 17.23, stage/render/graph_submit 15.08, stage/main/update 13.47, stage/main/post_update 11.77, encode//sum 7.78 |
| warm-up | 5.74 | 26.5 | pipelines/compile_ms 212, pipelines/front_ms 202, render/thread/ms 44.60, render/busy/ms 43.26, main/busy/ms 27.20, stage/render/prepare_assets 19.20, stage/main/update 12.12, stage/render/render 11.08, stage/main/post_update 9.76, pipelines/driver_ms 9.16, render/graph/ms 9.16, encode//sum 6.97 |
| warm-up | 5.81 | 44.8 | render/thread/ms 60.62, render/busy/ms 58.29, main/busy/ms 40.91, stage/render/prepare_assets 27.76, stage/render/render 16.65, stage/main/update 15.72, stage/main/post_update 15.19, render/graph/ms 13.63, stage/render/graph_submit 8.97, encode//sum 8.18, encode/submit 5.92, stream/drive/ms 5.55 |
| warm-up | 5.85 | 42.9 | render/thread/ms 66.79, render/busy/ms 64.70, stage/render/prepare_assets 39.65, main/busy/ms 35.53, stage/main/update 15.46, stage/render/render 13.22, stage/main/post_update 10.73, render/graph/ms 10.48, encode//sum 8.33, stage/render/graph_submit 7.12, stream/drive/ms 5.34, encode/submit 4.23 |
| warm-up | 5.89 | 37.7 | pipelines/compile_ms 1120, pipelines/front_ms 1090, render/thread/ms 59.86, render/busy/ms 58.73, stage/render/prepare_assets 40.96, main/busy/ms 35.82, pipelines/driver_ms 30.09, stage/main/update 16.73, stage/main/post_update 11.44, stage/render/render 8.91, render/graph/ms 7.32, encode//sum 7.22 |
| warm-up | 5.94 | 29.1 | pipelines/compile_ms 155, pipelines/front_ms 148, render/thread/ms 40.63, render/busy/ms 38.97, main/busy/ms 36.26, stage/main/update 17.99, stage/main/post_update 12.99, stage/render/render 12.59, render/graph/ms 10.74, stage/render/prepare_assets 9.69, stage/render/graph_submit 8.02, pipelines/driver_ms 6.75 |
| warm-up | 5.99 | 25.9 | main/busy/ms 39.14, render/thread/ms 33.29, render/busy/ms 31.49, stage/main/update 22.56, stream/drive/ms 15.04, stage/render/render 14.35, render/graph/ms 12.09, stage/main/post_update 10.20, stage/render/graph_submit 9.13, stage/render/prepare_assets 8.94, encode//sum 7.86, encode/submit 6.84 |
| warm-up | 6.04 | 36.9 | render/thread/ms 43.11, render/busy/ms 41.11, main/busy/ms 34.54, stage/render/render 16.99, render/graph/ms 14.53, stage/main/post_update 14.32, stage/main/update 13.35, stage/render/graph_submit 10.31, stage/render/prepare_assets 9.91, encode//sum 8.06, encode/submit 6.70, stream/drive/ms 4.11 |
| warm-up | 6.08 | 32.6 | render/thread/ms 51.15, render/busy/ms 49.31, main/busy/ms 43.13, stage/main/update 23.66, stage/render/prepare_assets 17.51, pipelines/compile_ms 14.61, stage/render/render 14.22, pipelines/front_ms 13.28, stage/main/post_update 12.17, render/graph/ms 11.82, stage/render/graph_submit 8.38, stream/drive/ms 7.88 |
| warm-up | 6.12 | 45.3 | pipelines/compile_ms 975, pipelines/front_ms 949, render/thread/ms 52.43, render/busy/ms 51.09, main/busy/ms 40.81, stage/render/prepare_assets 31.00, pipelines/driver_ms 25.55, stage/main/update 23.65, stage/main/post_update 10.37, stage/render/render 9.08, stream/drive/ms 7.88, render/graph/ms 7.19 |
| warm-up | 6.29 | 40.6 | pipelines/compile_ms 95.43, pipelines/front_ms 91.44, main/busy/ms 53.48, render/thread/ms 36.02, render/busy/ms 34.65, stage/main/update 31.96, stream/drive/ms 21.59, stage/main/post_update 15.29, stage/render/render 13.56, render/graph/ms 11.73, stage/render/prepare_assets 10.16, stage/render/graph_submit 8.93 |
| warm-up | 6.40 | 28.8 | pipelines/compile_ms 360, pipelines/front_ms 352, main/busy/ms 34.28, render/thread/ms 30.53, render/busy/ms 28.98, stage/main/update 17.51, stage/render/render 11.72, stage/main/post_update 11.64, stage/render/prepare_assets 10.91, render/graph/ms 9.76, pipelines/driver_ms 7.78, encode//sum 7.26 |
| warm-up | 6.64 | 26.0 | main/busy/ms 42.07, render/thread/ms 39.78, render/busy/ms 37.84, stage/main/update 20.50, stage/render/render 17.59, stage/main/post_update 15.76, render/graph/ms 14.93, stream/drive/ms 11.36, encode//sum 10.89, stage/render/prepare_assets 10.84, stage/render/graph_submit 10.54, encode/submit 6.39 |
| warm-up | 6.69 | 44.6 | render/thread/ms 48.26, render/busy/ms 46.64, main/busy/ms 41.78, stage/render/prepare_assets 25.07, stage/main/update 21.27, stage/main/post_update 14.56, stage/render/render 13.90, stream/drive/ms 11.89, render/graph/ms 11.87, stage/render/graph_submit 8.72, encode//sum 7.72, encode/submit 5.42 |
| warm-up | 7.02 | 31.6 | render/thread/ms 38.53, render/busy/ms 37.60, main/busy/ms 15.09, stage/render/specialize 9.53, stage/render/render 9.17, render/graph/ms 7.79, stage/main/post_update 7.62, stage/render/queue 6.49, encode//sum 6.46, stage/render/graph_submit 5.80, stage/main/update 4.19, stage/render/prepare_assets 4.09 |
| warm-up | 8.23 | 29.2 | render/thread/ms 35.97, render/busy/ms 33.16, main/busy/ms 13.84, stage/render/render 9.97, render/graph/ms 8.71, stage/render/specialize 8.17, stage/main/post_update 7.83, stage/render/graph_submit 6.61, encode//sum 5.87, stage/render/queue 5.72, stage/render/prepare_meshes 4.46, stage/main/update 3.89 |
| measure | 0.85 | 27.3 | render/thread/ms 34.14, render/busy/ms 32.05, main/busy/ms 12.42, stage/render/render 9.70, stage/render/specialize 8.85, render/graph/ms 8.50, encode//sum 7.64, stage/main/post_update 7.08, stage/render/graph_submit 6.52, stage/render/queue 5.79, encode/submit 3.96, stage/render/prepare_meshes 3.27 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set select.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=select-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 1%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 84%) · 2625 MHz (min 2625) · memory 8101 MHz · 97.8 W · 77 °C (max 80) · VRAM 6424 MiB · throttled 100% of samples

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
