# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); mw.matctrl.gate=on (command line: --set); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1550 frames over 15.0 s · **median 9.70 ms (103 fps)** · mean 9.68 · p95 11.01 · p99 11.51 (1% low 87 fps) · worst 26.80 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 279 frames over 8.5 s, worst 2585.7 ms
- hitches (> 25.0 ms): **1** measured, 58 in warm-up
- timed GPU passes: 8.54 ms of a 9.68 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.70 ms, limit 11 ms
- pass worst frame 26.80 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.911 | 2.924 | 2.994 | 1.00 | 0.067 |
| dlss_super_resolution | 2.011 | 2.021 | 2.044 | 1.00 | 0.185 |
| early prepass | 0.599 | 0.602 | 0.612 | 1.00 | 0.096 |
| atmosphere_luts | 0.332 | 0.333 | 0.394 | 1.00 | 0.003 |
| bloom | 0.313 | 0.314 | 0.331 | 1.00 | 0.063 |
| main_transmissive_pass_3d | 0.309 | 0.311 | 0.332 | 1.00 | 0.009 |
| water_velocity | 0.240 | 0.241 | 0.264 | 1.00 | 0.002 |
| water_sspr | 0.208 | 0.209 | 0.210 | 1.00 | 0.002 |
| ssao | 0.188 | 0.189 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.149 | 0.149 | 0.172 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.145 | 0.146 | 0.152 | 1.00 | 0.001 |
| water_fft | 0.132 | 0.132 | 0.136 | 1.00 | 0.003 |
| uniform_allocation | 0.120 | 0.120 | 0.163 | 1.00 | 0.022 |
| upscaling | 0.112 | 0.113 | 0.118 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.090 | 0.090 | 0.122 | 1.00 | 0.016 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.004 |
| clustering | 0.064 | 0.064 | 0.075 | 1.00 | 0.021 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.412 | 0.13 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.032 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.028 | 0.220 | 0.230 | 0.13 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.025 | 0.028 | 1.00 | 0.082 |
| water_snapshot | 0.025 | 0.025 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.046 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 1.00 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.014 | 0.014 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.13 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.003 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.731 | 10.089 | 25.66 | 13532.5 |
| main/busy/ms | 7.342 | 8.240 | 12.85 | 11379.9 |
| render/busy/ms | 7.022 | 8.049 | 25.04 | 10884.0 |
| stage/render/render | 4.767 | 5.618 | 11.49 | 7389.2 |
| stage/main/post_update | 4.326 | 5.077 | 10.10 | 6704.6 |
| render/graph/ms | 4.182 | 4.984 | 10.89 | 6481.5 |
| encode//sum | 3.985 | 5.132 | 16.78 | 6176.2 |
| stage/render/graph_submit | 3.173 | 3.943 | 9.88 | 4918.6 |
| encode/submit | 1.979 | 2.417 | 4.64 | 3066.9 |
| stage/main/update | 1.883 | 2.302 | 3.68 | 2919.2 |
| stage/render/prepare_views | 1.391 | 2.944 | 4.22 | 2156.6 |
| render/acquire/ms | 1.346 | 2.892 | 4.19 | 2085.7 |
| encode/finish_wall | 1.218 | 1.690 | 6.90 | 1887.6 |
| stage/render/camera_driver | 0.953 | 1.091 | 1.36 | 1477.4 |
| encode/pass:shadow_directional_light/sum | 0.786 | 1.354 | 1.94 | 1217.7 |
| encode/pass:early prepass/sum | 0.750 | 1.042 | 5.87 | 1162.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.724 | 1.007 | 1.40 | 1122.4 |
| stage/render/prepare_write_phase_buffers | 0.547 | 0.758 | 0.93 | 848.4 |
| stage/render/prepare_assets | 0.399 | 0.478 | 1.46 | 619.1 |
| stage/render/prepare_resources | 0.391 | 0.458 | 0.72 | 605.6 |
| bigspace/total/ms | 0.389 | 0.958 | 1.91 | 602.3 |
| render/present/ms | 0.363 | 0.438 | 1.53 | 562.9 |
| stage/main/pre_update | 0.351 | 0.465 | 1.14 | 544.3 |
| stage/main/first | 0.296 | 0.363 | 1.85 | 459.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.251 | 0.349 | 1.05 | 389.1 |
| terrain/update/ms | 0.235 | 0.299 | 1.09 | 364.6 |
| stage/render/prepare_bind_groups | 0.211 | 0.291 | 0.45 | 326.4 |
| terrain/select/ms | 0.205 | 0.261 | 0.98 | 317.4 |
| stage/render/prepare_meshes | 0.203 | 0.253 | 2.75 | 314.2 |
| stream/drive/ms | 0.192 | 0.262 | 0.64 | 298.1 |
| stage/main/fixed_main_loop | 0.186 | 0.361 | 0.97 | 287.7 |
| stage/render/prepare_batch_phases | 0.183 | 0.239 | 0.48 | 283.9 |
| stage/render/create_views | 0.171 | 0.221 | 0.60 | 265.8 |
| stage/render/extract_commands | 0.171 | 0.202 | 0.59 | 265.7 |
| far/select/ms | 0.168 | 0.228 | 0.56 | 260.3 |
| stream/tick/ms | 0.163 | 0.224 | 0.61 | 252.7 |
| stage/render/prepare_flush | 0.155 | 0.276 | 0.85 | 240.3 |
| controller/feed/ms | 0.152 | 0.197 | 0.33 | 235.3 |
| controller/move/ms | 0.147 | 0.240 | 0.73 | 228.3 |
| bigspace/high_precision/ms | 0.147 | 0.548 | 1.70 | 227.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.119 | 0.183 | 0.39 | 184.3 |
| stage/main/last | 0.118 | 0.157 | 0.85 | 183.3 |
| stage/render/queue | 0.098 | 0.124 | 5.32 | 151.5 |
| anim/evaluate/ms | 0.092 | 0.212 | 1.49 | 142.2 |
| bigspace/low_precision/ms | 0.091 | 0.177 | 1.09 | 140.4 |
| stage/render/specialize | 0.071 | 0.089 | 8.50 | 109.9 |
| anim/evaluate_cpu/ms | 0.062 | 0.078 | 0.15 | 96.0 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.112 | 0.22 | 90.5 |
| encode/resolve_diagnostics/sum | 0.057 | 0.083 | 0.21 | 87.6 |
| physics/step/ms | 0.029 | 0.058 | 0.21 | 44.5 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.039 | 0.19 | 41.1 |
| weather/map/ms | 0.026 | 0.034 | 0.40 | 40.0 |
| encode/camera_driver_span_end/sum | 0.023 | 0.036 | 0.17 | 36.0 |
| stage/render/cleanup | 0.022 | 0.032 | 0.24 | 34.4 |
| stage/render/phase_sort | 0.017 | 0.033 | 0.08 | 27.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.03 | 17.2 |
| script/update/ms | 0.007 | 0.010 | 0.03 | 10.5 |
| controller/avoid/ms | 0.006 | 0.008 | 0.40 | 9.6 |
| script/input/ms | 0.005 | 0.006 | 0.10 | 8.2 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 6.9 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.7 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.32 | 1.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.5 |
| stage/render/prepare | 0.001 | 0.003 | 0.09 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.6 |
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
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1080 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.58 | 6.67 | 0.64 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.19 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.17 | 0.02 |
| encode/finish_critical | ms | 0.89 | 6.67 | 0.86 |
| encode/pass:early prepass/max | ms | 0.74 | 5.87 | 0.55 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.73 | 1.40 | 0.86 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.39 | 0.18 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 1.05 | 0.21 |
| encode/pass:shadow_directional_light/max | ms | 0.72 | 1.94 | 0.54 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.21 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.21 | 25.04 | 9.31 |
| gpu/timed/ms | ms | 9.21 | 10.47 | 9.31 |
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
| phases/opaque3d/bins | count | 1307 | 1307 | 1307 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/bins | count | 1307 | 1307 | 1307 |
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
| process/cpu_usage | % | 27.48 | 37.02 | 23.92 |
| process/mem_usage | GiB | 4.68 | 4.73 | 4.73 |
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
| skin/blocks_changed/count | count | 5 | 6 | 4 |
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
| system/cpu_usage | % | 39.84 | 51.29 | 40.99 |
| system/mem_usage | % | 58.15 | 58.43 | 58.43 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 53 | 82 | 53 |
| views/all/bins | count | 5326 | 8063 | 5326 |
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
| views/main/bins | count | 3174 | 3174 | 3174 |
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
| anim/events/count | count | 0.09 | 3 | 132 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 0.00 | 4 | 4 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 29 | 41.2 k |
| encode/bind_walk/init_textures | count | 7164 | 14.0 k | 11.10 M |
| encode/bind_walk/init_textures_skipped | count | 43.4 k | 56.5 k | 67.21 M |
| encode/bind_walk/inits | count | 345 | 437 | 535.3 k |
| encode/bind_walk/inits_skipped | count | 74.99 | 87 | 116.2 k |
| encode/bind_walk/merge_textures | count | 7100 | 13.9 k | 11.01 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.33 M |
| encode/bind_walk/merges | count | 82.25 | 103 | 127.5 k |
| encode/bind_walk/merges_skipped | count | 54.75 | 60 | 84.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1550 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1550 |
| encode/pass:early prepass/count | count | 1 | 1 | 1550 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1550 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1550 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1550 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1550 |
| encode/queue/command_buffers | count | 36.57 | 39 | 56.7 k |
| encode/queue/submits | count | 2 | 2 | 3100 |
| encode/queue/write_buffer/bytes | bytes | 1.38 M | 3.65 M | 2.14 G |
| encode/queue/write_buffer/calls | count | 217 | 287 | 336.3 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.59 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.9 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1550 |
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

## Hitches (> 25.0 ms; slowest 59, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.59 | 2585.7 | main/busy/ms 2555, stage/main/post_update 2453, stage/main/update 93.63, surface/advance_cpu_ms 64.54, stage/main/pre_update 4.28, stage/main/last 1.25, surface/cell_upload_ms 0.76, stream/drive/ms 0.66, stage/main/fixed_main_loop 0.62, stage/main/first 0.47, weather/map/ms 0.26, stream/tick/ms 0.12 |
| warm-up | 3.33 | 744.4 | main/busy/ms 2713, stage/main/post_update 2481, render/thread/ms 705, render/busy/ms 534, stage/render/prepare_assets 487, stage/main/update 216, stage/render/prepare_views 166, render/acquire/ms 165, surface/advance_cpu_ms 64.54, stage/render/create_views 28.29, stage/render/render 13.86, render/graph/ms 7.41 |
| warm-up | 3.51 | 179.3 | render/thread/ms 867, render/busy/ms 601, stage/render/prepare_assets 539, main/busy/ms 187, stage/render/prepare_views 166, render/acquire/ms 166, stage/main/update 129, stage/render/render 114, render/present/ms 101, stage/main/post_update 42.09, stage/render/create_views 28.60, pipelines/compile_ms 23.32 |
| warm-up | 3.70 | 180.1 | pipelines/wait_ms 3917, pipelines/compile_ms 2296, pipelines/front_ms 2007, pipelines/driver_ms 289, main/busy/ms 152, render/thread/ms 118, render/busy/ms 115, stage/render/prepare_assets 95.33, stage/main/update 75.79, stage/main/post_update 35.29, stage/main/fixed_main_loop 30.58, terrain/update/ms 22.87 |
| warm-up | 3.81 | 104.3 | pipelines/wait_ms 4503, pipelines/compile_ms 3167, pipelines/front_ms 2653, pipelines/driver_ms 514, render/thread/ms 207, render/busy/ms 205, stage/render/prepare_assets 188, main/busy/ms 90.35, stage/main/fixed_main_loop 29.16, stage/main/update 26.21, stage/main/post_update 24.69, terrain/update/ms 22.58 |
| warm-up | 3.86 | 28.9 | pipelines/compile_ms 353, pipelines/front_ms 335, render/thread/ms 48.17, main/busy/ms 46.59, render/busy/ms 45.58, pipelines/wait_ms 43.18, stage/main/post_update 36.34, stage/render/render 23.15, render/graph/ms 20.31, pipelines/driver_ms 18.17, encode//sum 14.81, stage/render/prepare_assets 14.09 |
| warm-up | 3.89 | 33.7 | pipelines/compile_ms 613, pipelines/front_ms 572, main/busy/ms 55.30, render/thread/ms 53.00, render/busy/ms 50.20, stage/main/post_update 43.54, pipelines/driver_ms 40.94, pipelines/wait_ms 26.84, stage/render/render 25.38, render/graph/ms 22.23, encode//sum 15.30, stage/render/graph_submit 13.43 |
| warm-up | 3.92 | 30.1 | pipelines/compile_ms 570, pipelines/front_ms 531, render/thread/ms 53.35, render/busy/ms 51.70, pipelines/driver_ms 39.08, main/busy/ms 35.76, stage/main/post_update 24.26, stage/render/prepare_assets 20.53, stage/render/render 10.89, pipelines/wait_ms 9.36, terrain/update/ms 9.21, render/graph/ms 8.66 |
| warm-up | 3.95 | 25.5 | pipelines/compile_ms 870, pipelines/front_ms 838, pipelines/wait_ms 319, main/busy/ms 58.85, render/thread/ms 52.27, render/busy/ms 49.80, stage/main/update 39.74, stream/drive/ms 36.99, pipelines/driver_ms 32.65, stage/render/prepare_assets 30.40, stage/render/render 14.53, stage/main/post_update 14.21 |
| warm-up | 4.01 | 63.0 | pipelines/compile_ms 1375, pipelines/front_ms 1331, pipelines/wait_ms 320, render/thread/ms 61.63, render/busy/ms 58.97, main/busy/ms 51.26, pipelines/driver_ms 44.83, stage/main/update 40.26, stage/render/prepare_assets 39.96, stream/drive/ms 37.07, stage/render/render 15.03, render/graph/ms 12.31 |
| warm-up | 4.09 | 52.0 | pipelines/compile_ms 331, pipelines/front_ms 318, render/thread/ms 62.46, render/busy/ms 60.57, main/busy/ms 38.85, stage/render/prepare_assets 29.44, stage/render/render 26.13, stage/main/update 24.68, render/graph/ms 23.65, stream/drive/ms 19.36, stage/render/graph_submit 18.58, pipelines/driver_ms 12.67 |
| warm-up | 4.14 | 49.9 | main/busy/ms 83.34, render/thread/ms 82.45, render/busy/ms 80.12, stage/main/update 64.37, pipelines/compile_ms 61.35, pipelines/front_ms 58.38, stage/render/prepare_assets 37.03, stage/render/render 35.97, stream/drive/ms 35.80, render/graph/ms 31.47, stage/render/graph_submit 25.08, encode//sum 19.21 |
| warm-up | 4.32 | 184.1 | pipelines/compile_ms 1800, pipelines/front_ms 1724, render/thread/ms 163, render/busy/ms 161, stage/render/prepare_assets 121, main/busy/ms 101, pipelines/driver_ms 76.26, stage/main/update 74.03, stream/drive/ms 36.75, stage/render/render 31.56, render/graph/ms 27.72, stage/render/graph_submit 21.85 |
| warm-up | 4.39 | 69.1 | pipelines/compile_ms 1945, pipelines/front_ms 1860, render/thread/ms 177, render/busy/ms 175, stage/render/prepare_assets 127, main/busy/ms 90.50, pipelines/driver_ms 85.03, stage/main/update 65.04, stream/drive/ms 44.32, stage/render/render 40.44, render/graph/ms 38.08, pipelines/wait_ms 22.63 |
| warm-up | 4.53 | 138.8 | pipelines/compile_ms 893, pipelines/front_ms 859, pipelines/wait_ms 149, render/thread/ms 148, render/busy/ms 146, main/busy/ms 92.27, stage/main/update 73.09, stage/render/prepare_assets 70.59, stage/render/render 67.37, render/graph/ms 64.72, stream/drive/ms 60.61, stage/render/graph_submit 45.48 |
| warm-up | 4.58 | 50.3 | pipelines/compile_ms 2101, pipelines/front_ms 1999, pipelines/wait_ms 199, render/thread/ms 135, render/busy/ms 131, pipelines/driver_ms 102, stage/render/prepare_assets 65.00, main/busy/ms 64.08, stage/render/render 53.43, render/graph/ms 49.77, stage/render/graph_submit 46.92, stage/main/update 38.73 |
| warm-up | 4.61 | 28.1 | pipelines/compile_ms 1605, pipelines/front_ms 1521, pipelines/wait_ms 121, pipelines/driver_ms 84.42, render/thread/ms 67.73, render/busy/ms 64.31, main/busy/ms 54.63, stage/main/update 30.67, stage/render/render 27.47, render/graph/ms 23.84, stage/render/graph_submit 20.96, pipelines/lock_ms 20.03 |
| warm-up | 4.66 | 53.9 | pipelines/compile_ms 222, pipelines/front_ms 215, render/thread/ms 70.02, render/busy/ms 68.66, pipelines/wait_ms 54.59, main/busy/ms 40.20, stage/main/update 25.28, stage/render/prepare_assets 21.60, stage/render/prepare_resources 17.17, stage/main/post_update 8.97, stage/render/render 8.85, stage/render/prepare_write_phase_buffers 7.18 |
| warm-up | 4.70 | 43.0 | pipelines/compile_ms 367, pipelines/front_ms 356, render/thread/ms 81.51, render/busy/ms 80.22, main/busy/ms 41.96, stage/render/prepare_assets 27.23, stage/render/prepare_resources 19.13, stage/main/update 15.20, stage/main/post_update 14.88, stage/render/render 13.08, render/graph/ms 11.39, pipelines/driver_ms 10.69 |
| warm-up | 4.73 | 26.1 | pipelines/compile_ms 571, pipelines/front_ms 555, main/busy/ms 67.51, render/thread/ms 59.70, render/busy/ms 57.42, stage/main/post_update 34.01, stage/main/update 22.35, stage/render/render 17.21, stage/render/prepare_assets 16.53, pipelines/driver_ms 16.14, render/graph/ms 15.48, pipelines/wait_ms 12.05 |
| warm-up | 4.79 | 61.4 | pipelines/compile_ms 1813, pipelines/front_ms 1763, main/busy/ms 60.85, render/thread/ms 50.78, pipelines/driver_ms 50.21, render/busy/ms 48.20, pipelines/wait_ms 35.19, stage/main/post_update 29.74, stage/main/update 24.38, stage/render/render 17.99, stage/render/prepare_assets 15.84, render/graph/ms 15.71 |
| warm-up | 4.83 | 34.1 | pipelines/compile_ms 2312, pipelines/front_ms 2251, main/busy/ms 71.08, pipelines/driver_ms 60.77, render/thread/ms 54.45, render/busy/ms 52.81, stage/main/update 52.67, stream/drive/ms 45.79, pipelines/wait_ms 34.80, stage/render/prepare_assets 20.24, stage/render/render 14.37, render/graph/ms 12.14 |
| warm-up | 4.87 | 44.1 | pipelines/compile_ms 915, pipelines/front_ms 892, main/busy/ms 102, stage/main/update 87.70, stream/drive/ms 82.26, pipelines/wait_ms 64.22, render/thread/ms 43.20, render/busy/ms 41.71, pipelines/driver_ms 23.07, stage/render/prepare_assets 15.29, stage/render/render 10.69, render/graph/ms 8.64 |
| warm-up | 4.93 | 62.3 | pipelines/compile_ms 649, pipelines/front_ms 627, pipelines/wait_ms 140, main/busy/ms 70.27, stage/main/update 47.30, stream/drive/ms 41.68, render/thread/ms 38.68, render/busy/ms 36.99, pipelines/driver_ms 21.65, stage/render/render 16.73, stage/main/post_update 16.18, render/graph/ms 14.49 |
| warm-up | 4.97 | 35.6 | pipelines/compile_ms 502, pipelines/front_ms 485, pipelines/wait_ms 88.81, main/busy/ms 59.19, render/thread/ms 53.78, render/busy/ms 51.79, stage/main/update 27.47, stage/render/render 24.03, render/graph/ms 21.57, stage/main/post_update 19.85, pipelines/driver_ms 16.62, stage/render/graph_submit 13.50 |
| warm-up | 5.03 | 59.2 | pipelines/compile_ms 1271, pipelines/front_ms 1240, main/busy/ms 77.72, render/thread/ms 66.37, render/busy/ms 64.46, stage/main/update 55.34, pipelines/driver_ms 30.87, stage/render/prepare_assets 29.13, pipelines/wait_ms 21.99, stage/render/render 18.52, render/graph/ms 16.18, stage/render/graph_submit 13.12 |
| warm-up | 5.09 | 59.6 | pipelines/compile_ms 1241, pipelines/front_ms 1211, render/thread/ms 84.01, render/busy/ms 82.36, main/busy/ms 53.28, stage/main/update 38.99, stage/render/prepare_assets 35.16, stage/render/render 33.56, render/graph/ms 31.53, pipelines/driver_ms 30.79, stage/render/graph_submit 29.04, encode/finish_wall 24.93 |
| warm-up | 5.12 | 31.1 | pipelines/compile_ms 805, pipelines/front_ms 783, render/thread/ms 78.17, render/busy/ms 76.54, stage/render/render 33.88, render/graph/ms 31.78, main/busy/ms 31.49, stage/render/graph_submit 29.20, encode/finish_wall 24.73, encode/pass:main_transmissive_pass_3d/sum 23.91, pipelines/driver_ms 21.55, stage/main/update 16.90 |
| warm-up | 5.14 | 25.6 | pipelines/compile_ms 969, pipelines/front_ms 945, render/thread/ms 50.01, render/busy/ms 48.48, main/busy/ms 31.84, pipelines/driver_ms 23.49, stage/main/update 17.18, stage/render/prepare_write_phase_buffers 13.48, stage/render/render 11.17, render/graph/ms 9.26, stage/main/post_update 8.68, stage/render/graph_submit 6.69 |
| warm-up | 5.18 | 39.2 | pipelines/compile_ms 394, pipelines/front_ms 384, render/thread/ms 57.56, render/busy/ms 56.16, main/busy/ms 44.99, stage/main/update 24.01, stage/render/prepare_resources 21.10, stage/main/post_update 16.06, stream/drive/ms 15.09, stage/render/render 11.92, pipelines/driver_ms 10.28, render/graph/ms 10.21 |
| warm-up | 5.21 | 27.9 | pipelines/compile_ms 726, pipelines/front_ms 695, render/thread/ms 60.16, render/busy/ms 58.81, main/busy/ms 48.42, pipelines/driver_ms 30.80, stage/main/update 27.01, stage/render/prepare_resources 21.88, stream/drive/ms 15.82, stage/main/post_update 15.60, stage/render/render 12.96, render/graph/ms 11.15 |
| warm-up | 5.24 | 26.8 | pipelines/compile_ms 906, pipelines/front_ms 869, render/thread/ms 45.76, render/busy/ms 44.20, main/busy/ms 37.27, pipelines/driver_ms 36.82, stage/main/update 20.08, stage/render/prepare_assets 12.36, stage/render/render 11.56, stage/main/post_update 10.83, render/graph/ms 9.85, encode//sum 7.46 |
| warm-up | 5.27 | 28.1 | pipelines/compile_ms 580, pipelines/front_ms 561, render/thread/ms 46.23, render/busy/ms 44.21, main/busy/ms 40.66, stage/main/update 20.82, pipelines/driver_ms 19.35, stage/render/render 13.36, stage/main/post_update 13.16, render/graph/ms 11.27, stage/render/prepare_assets 11.24, stage/render/graph_submit 8.04 |
| warm-up | 5.30 | 31.5 | pipelines/compile_ms 243, pipelines/front_ms 232, render/thread/ms 50.83, render/busy/ms 48.67, main/busy/ms 48.08, stage/main/post_update 20.22, stage/main/update 18.27, stage/render/render 16.58, stage/render/prepare_assets 14.91, render/graph/ms 13.76, pipelines/driver_ms 11.13, stage/render/graph_submit 9.83 |
| warm-up | 5.34 | 44.6 | pipelines/compile_ms 231, pipelines/front_ms 222, render/thread/ms 57.76, render/busy/ms 55.96, main/busy/ms 50.47, stage/main/post_update 26.21, stage/render/prepare_assets 20.17, stage/render/render 15.84, stage/main/update 14.93, render/graph/ms 13.06, pipelines/driver_ms 9.35, stream/drive/ms 8.55 |
| warm-up | 5.38 | 42.4 | pipelines/compile_ms 1340, pipelines/front_ms 1311, render/thread/ms 57.13, render/busy/ms 55.58, main/busy/ms 37.45, pipelines/driver_ms 29.01, stage/render/prepare_assets 27.95, stage/main/post_update 16.52, stage/main/update 14.06, stage/render/render 13.84, render/graph/ms 11.59, stage/render/graph_submit 7.39 |
| warm-up | 5.41 | 30.3 | pipelines/compile_ms 1392, pipelines/front_ms 1358, render/thread/ms 50.41, render/busy/ms 48.92, pipelines/driver_ms 33.86, main/busy/ms 30.25, stage/render/prepare_assets 26.52, stage/main/update 15.44, stage/render/render 13.49, render/graph/ms 11.58, stage/render/graph_submit 8.81, pipelines/wait_ms 8.70 |
| warm-up | 5.48 | 31.0 | pipelines/compile_ms 132, pipelines/front_ms 126, render/thread/ms 48.82, render/busy/ms 47.12, main/busy/ms 42.87, stage/main/update 24.72, stream/drive/ms 17.69, stage/render/prepare_resources 15.08, stage/main/post_update 13.00, stage/render/render 12.40, render/graph/ms 10.21, pipelines/wait_ms 9.59 |
| warm-up | 5.52 | 35.4 | render/thread/ms 62.59, render/busy/ms 61.27, main/busy/ms 60.31, stage/main/update 37.52, pipelines/compile_ms 34.48, pipelines/front_ms 33.20, stage/render/prepare_assets 23.25, stream/drive/ms 18.03, stage/main/post_update 17.64, stage/render/prepare_resources 11.81, stage/render/render 10.77, render/graph/ms 9.04 |
| warm-up | 5.55 | 25.3 | pipelines/compile_ms 198, pipelines/front_ms 193, render/thread/ms 54.95, render/busy/ms 53.65, main/busy/ms 44.65, stage/render/prepare_assets 31.34, stage/main/update 23.67, stage/main/post_update 15.17, stage/render/render 9.86, render/graph/ms 8.06, encode//sum 6.54, pipelines/driver_ms 5.38 |
| warm-up | 5.58 | 37.7 | pipelines/compile_ms 807, pipelines/front_ms 787, render/thread/ms 58.48, render/busy/ms 44.63, main/busy/ms 38.47, stage/render/prepare_assets 22.03, stage/main/post_update 21.08, pipelines/driver_ms 19.33, stage/render/prepare_views 12.87, render/acquire/ms 12.63, stage/main/update 11.89, stage/render/render 10.66 |
| warm-up | 5.63 | 30.2 | pipelines/compile_ms 969, pipelines/front_ms 947, render/thread/ms 42.90, render/busy/ms 41.41, main/busy/ms 30.94, pipelines/driver_ms 21.96, stage/render/prepare_assets 17.37, stage/main/update 13.85, stage/render/render 11.60, stage/main/post_update 10.44, render/graph/ms 9.62, encode//sum 8.01 |
| warm-up | 5.70 | 42.9 | pipelines/compile_ms 682, pipelines/front_ms 662, render/thread/ms 55.31, render/busy/ms 54.09, stage/render/prepare_assets 33.92, main/busy/ms 30.52, pipelines/driver_ms 20.47, stage/main/update 15.18, stage/render/render 10.22, stage/main/post_update 9.57, render/graph/ms 8.61, encode//sum 7.08 |
| warm-up | 5.74 | 33.8 | pipelines/compile_ms 170, pipelines/front_ms 165, render/thread/ms 42.12, render/busy/ms 40.80, main/busy/ms 25.29, stage/render/prepare_assets 18.88, stage/render/render 10.80, stage/main/update 10.18, stage/main/post_update 10.10, render/graph/ms 9.04, encode//sum 6.60, stage/render/graph_submit 6.42 |
| warm-up | 5.81 | 44.2 | render/thread/ms 62.41, render/busy/ms 60.48, main/busy/ms 35.94, stage/render/prepare_assets 30.41, pipelines/compile_ms 29.21, pipelines/front_ms 27.41, stage/render/render 16.20, stage/main/update 14.88, stage/main/post_update 13.67, render/graph/ms 13.47, encode//sum 10.01, stage/render/graph_submit 9.94 |
| warm-up | 5.84 | 30.4 | pipelines/compile_ms 891, pipelines/front_ms 868, render/thread/ms 60.90, render/busy/ms 59.63, stage/render/prepare_assets 38.38, main/busy/ms 37.93, pipelines/driver_ms 22.62, stage/main/update 16.52, stage/main/post_update 13.44, stage/render/render 11.24, render/graph/ms 9.42, encode//sum 8.40 |
| warm-up | 5.88 | 40.8 | pipelines/compile_ms 1246, pipelines/front_ms 1216, render/thread/ms 54.30, render/busy/ms 53.14, stage/render/prepare_assets 36.93, main/busy/ms 33.54, pipelines/driver_ms 29.83, stage/main/post_update 13.36, stage/main/update 12.83, stage/render/render 8.34, render/graph/ms 6.78, encode//sum 6.50 |
| warm-up | 5.95 | 28.6 | pipelines/compile_ms 120, pipelines/front_ms 115, main/busy/ms 46.29, render/thread/ms 42.69, render/busy/ms 40.90, stage/render/render 21.51, stage/main/update 21.33, render/graph/ms 18.43, stage/main/post_update 16.59, stage/render/graph_submit 13.26, stage/render/prepare_assets 7.95, encode//sum 7.78 |
| warm-up | 6.01 | 30.4 | pipelines/compile_ms 325, pipelines/front_ms 313, main/busy/ms 42.37, render/thread/ms 30.81, render/busy/ms 29.28, stage/main/update 23.07, stream/drive/ms 17.05, stage/main/post_update 12.52, pipelines/driver_ms 12.42, stage/render/render 12.37, render/graph/ms 10.29, stage/render/prepare_assets 8.05 |
| warm-up | 6.07 | 37.5 | render/thread/ms 46.88, render/busy/ms 44.93, main/busy/ms 42.34, stage/render/render 18.07, stage/main/update 17.80, stage/main/post_update 16.42, render/graph/ms 15.67, stage/render/prepare_assets 12.76, stage/render/graph_submit 12.03, encode//sum 10.42, stream/drive/ms 9.27, encode/submit 7.45 |
| warm-up | 6.10 | 32.7 | render/thread/ms 49.43, render/busy/ms 47.64, main/busy/ms 43.83, stage/render/prepare_assets 19.15, stage/main/update 18.91, stage/main/post_update 15.96, pipelines/compile_ms 15.87, pipelines/front_ms 14.29, stage/render/render 13.68, render/graph/ms 11.41, encode//sum 9.28, stage/render/graph_submit 8.04 |
| warm-up | 6.14 | 37.5 | pipelines/compile_ms 1395, pipelines/front_ms 1365, render/thread/ms 45.58, render/busy/ms 44.31, main/busy/ms 38.16, pipelines/driver_ms 29.56, pipelines/wait_ms 26.35, stage/render/prepare_assets 25.47, stage/main/update 19.54, stage/main/post_update 11.63, stage/render/render 8.59, encode//sum 7.38 |
| warm-up | 6.25 | 25.5 | pipelines/compile_ms 163, pipelines/front_ms 159, main/busy/ms 34.80, render/thread/ms 24.95, render/busy/ms 23.98, stage/main/update 22.05, stream/drive/ms 15.83, stage/render/render 9.62, render/graph/ms 8.14, stage/main/post_update 8.02, stage/render/prepare_assets 7.04, encode//sum 6.82 |
| warm-up | 6.34 | 27.5 | main/busy/ms 32.31, render/thread/ms 25.46, render/busy/ms 23.32, stage/main/update 16.39, stage/render/render 11.15, stage/main/post_update 10.79, stage/render/prepare_assets 9.19, render/graph/ms 8.56, encode//sum 7.72, stage/render/graph_submit 6.30, encode/submit 4.15, stream/drive/ms 3.41 |
| warm-up | 6.55 | 30.8 | render/thread/ms 36.23, main/busy/ms 34.83, render/busy/ms 34.61, stage/render/render 17.56, stage/main/post_update 15.79, stage/main/update 15.31, render/graph/ms 14.88, stage/render/graph_submit 9.92, encode//sum 9.63, stage/render/prepare_assets 9.00, encode/submit 5.48, stage/render/camera_driver 4.76 |
| warm-up | 6.62 | 51.5 | render/thread/ms 45.60, render/busy/ms 44.07, main/busy/ms 43.30, stage/render/prepare_assets 25.51, stage/main/update 21.02, stage/main/post_update 15.84, stage/render/render 12.00, stream/drive/ms 11.52, render/graph/ms 10.01, encode//sum 9.28, stage/render/graph_submit 7.04, encode/submit 4.52 |
| warm-up | 6.95 | 30.0 | render/thread/ms 36.48, render/busy/ms 35.24, main/busy/ms 14.10, stage/render/render 11.33, render/graph/ms 9.92, stage/render/specialize 8.97, stage/render/graph_submit 8.00, stage/main/post_update 7.33, encode//sum 6.76, stage/render/queue 5.65, encode/submit 5.11, stage/main/update 4.27 |
| warm-up | 8.15 | 29.5 | render/thread/ms 35.54, render/busy/ms 33.43, main/busy/ms 14.58, stage/render/render 10.76, render/graph/ms 9.46, stage/render/specialize 8.42, stage/main/post_update 8.07, stage/render/graph_submit 7.38, encode//sum 6.37, stage/render/queue 5.40, encode/submit 4.26, stage/render/prepare_meshes 4.01 |
| measure | 0.83 | 26.8 | render/thread/ms 34.05, render/busy/ms 31.70, main/busy/ms 15.06, stage/render/render 10.31, render/graph/ms 8.97, encode//sum 8.88, stage/render/specialize 8.56, stage/main/post_update 8.10, stage/render/graph_submit 7.08, stage/render/queue 5.44, stage/main/update 4.25, encode/submit 4.23 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set mw.matctrl.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=matctrl-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 0%
- during the run: other processes 12% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 97.8 W · 77 °C (max 79) · VRAM 6423 MiB · throttled 100% of samples

## Tunables

- `far.handoff_scale=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.bias=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_distance_m=24 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_px=2 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `mw.matctrl.gate=on (command line: --set)`
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
