# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.face_cull=on (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6003 frames over 65.6 s · **median 10.60 ms (94 fps)** · mean 10.93 · p95 13.91 · p99 15.80 (1% low 63 fps) · worst 32.91 ms · 55 spikes over 1.5× median · 23.8 stutters/min (work 20.1/min)
- warm-up: 325 frames over 9.4 s, worst 2460.7 ms
- hitches (> 26.5 ms): **3** measured, 48 in warm-up
- timed GPU passes: 6.71 ms of a 10.93 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.60 ms, limit 11 ms
- **FAIL** frame_ms: 26 stutters (> 1.5x moving median of 60) in 1.09 min = 23.77/min, limit 1/min
- pass worst frame 32.91 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.019 | 2.030 | 2.063 | 0.99 | 0.176 |
| main_opaque_pass_3d | 1.296 | 1.303 | 1.638 | 0.99 | 0.070 |
| early prepass | 0.487 | 0.490 | 0.701 | 0.99 | 0.088 |
| bloom | 0.314 | 0.316 | 0.322 | 0.99 | 0.063 |
| atmosphere_luts | 0.308 | 0.310 | 0.374 | 0.99 | 0.003 |
| water_velocity | 0.261 | 0.263 | 0.350 | 0.99 | 0.002 |
| auxiliary_views | 0.248 | 0.249 | 0.915 | 0.99 | 0.241 |
| water_sspr | 0.200 | 0.201 | 0.203 | 0.99 | 0.002 |
| ssao | 0.187 | 0.188 | 0.193 | 0.99 | 0.007 |
| upscaling | 0.128 | 0.129 | 0.136 | 0.99 | 0.004 |
| water_fft | 0.127 | 0.128 | 0.136 | 0.99 | 0.003 |
| atmos_froxel_inject | 0.126 | 0.127 | 0.129 | 0.99 | 0.001 |
| uniform_allocation | 0.126 | 0.126 | 0.154 | 1.00 | 0.026 |
| clouds_composite | 0.104 | 0.105 | 0.108 | 0.99 | 0.002 |
| bin_unpacking | 0.097 | 0.097 | 0.119 | 1.00 | 0.019 |
| tonemapping | 0.086 | 0.086 | 0.091 | 0.99 | 0.004 |
| main_transmissive_pass_3d | 0.078 | 0.078 | 0.154 | 0.99 | 0.009 |
| clustering | 0.063 | 0.064 | 0.067 | 0.99 | 0.019 |
| clouds_trace | 0.056 | 0.056 | 0.074 | 0.99 | 0.001 |
| render_sky | 0.051 | 0.052 | 0.053 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.412 | 0.416 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 0.99 | 0.001 |
| surface_trail | 0.040 | 0.056 | 0.057 | 0.71 | 0.002 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.028 | 0.028 | 0.037 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.228 | 0.12 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.029 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.039 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.013 | 0.013 | 0.025 | 0.99 | 0.125 |
| clouds_fog_light | 0.012 | 0.012 | 0.013 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.99 | 0.004 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.632 | 12.555 | 23.65 | 57821.5 |
| main/busy/ms | 9.157 | 11.064 | 29.75 | 54968.4 |
| render/busy/ms | 9.116 | 11.994 | 23.00 | 54723.1 |
| stage/render/render | 5.811 | 7.958 | 15.87 | 34882.0 |
| stage/main/post_update | 5.726 | 7.313 | 10.20 | 34374.2 |
| render/graph/ms | 5.110 | 7.202 | 14.86 | 30673.1 |
| encode//sum | 4.638 | 6.809 | 27.94 | 27842.6 |
| stage/render/graph_submit | 3.815 | 5.286 | 10.73 | 22899.2 |
| encode/submit | 2.453 | 3.549 | 6.21 | 14722.4 |
| stage/main/update | 2.171 | 2.639 | 22.67 | 13030.7 |
| encode/finish_wall | 1.398 | 1.888 | 4.84 | 8393.8 |
| encode/pass:shadow_point_light/sum | 1.352 | 4.435 | 15.02 | 8117.0 |
| stage/render/camera_driver | 1.242 | 1.914 | 4.36 | 7455.3 |
| stage/render/prepare_write_phase_buffers | 0.761 | 1.081 | 2.26 | 4567.7 |
| encode/pass:shadow_directional_light/sum | 0.753 | 1.257 | 1.94 | 4522.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.741 | 1.091 | 2.25 | 4447.0 |
| encode/pass:early prepass/sum | 0.716 | 1.063 | 1.66 | 4300.1 |
| stage/render/prepare_assets | 0.581 | 0.731 | 11.35 | 3489.5 |
| bigspace/total/ms | 0.562 | 1.417 | 2.58 | 3373.9 |
| stage/render/prepare_resources | 0.464 | 0.560 | 0.93 | 2785.8 |
| stage/render/prepare_meshes | 0.432 | 0.539 | 2.10 | 2596.2 |
| render/present/ms | 0.421 | 0.515 | 2.36 | 2526.7 |
| stage/main/first | 0.394 | 0.516 | 6.13 | 2365.0 |
| stage/main/pre_update | 0.357 | 0.496 | 1.87 | 2141.3 |
| terrain/update/ms | 0.321 | 0.432 | 3.03 | 1929.9 |
| terrain/select/ms | 0.287 | 0.388 | 1.12 | 1724.5 |
| controller/feed/ms | 0.271 | 0.337 | 0.84 | 1624.5 |
| stage/render/specialize | 0.268 | 0.401 | 2.63 | 1610.8 |
| stage/render/prepare_bind_groups | 0.261 | 0.447 | 2.13 | 1566.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.251 | 0.370 | 0.88 | 1506.4 |
| stream/drive/ms | 0.250 | 0.331 | 20.20 | 1503.3 |
| stage/render/queue | 0.249 | 0.391 | 2.35 | 1494.7 |
| bigspace/low_precision/ms | 0.234 | 0.773 | 2.04 | 1406.7 |
| stage/main/fixed_main_loop | 0.216 | 0.365 | 1.19 | 1298.9 |
| far/select/ms | 0.198 | 0.263 | 0.49 | 1191.2 |
| stage/render/create_views | 0.194 | 0.252 | 0.89 | 1167.0 |
| stream/tick/ms | 0.194 | 0.262 | 0.91 | 1162.1 |
| stage/render/prepare_batch_phases | 0.187 | 0.263 | 0.61 | 1122.1 |
| stage/render/extract_commands | 0.175 | 0.203 | 0.42 | 1049.8 |
| anim/evaluate/ms | 0.171 | 0.275 | 1.66 | 1027.5 |
| stage/render/prepare_flush | 0.164 | 0.313 | 1.02 | 984.7 |
| bigspace/high_precision/ms | 0.163 | 0.729 | 2.06 | 977.1 |
| anim/evaluate_cpu/ms | 0.162 | 0.199 | 0.79 | 971.6 |
| controller/move/ms | 0.144 | 0.313 | 0.77 | 863.5 |
| stage/render/prepare_views | 0.138 | 0.172 | 0.44 | 831.3 |
| stage/main/last | 0.115 | 0.155 | 3.16 | 688.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.108 | 0.182 | 0.68 | 650.3 |
| render/acquire/ms | 0.095 | 0.121 | 0.39 | 571.8 |
| stage/render/prepare_collect_phase_buffers | 0.062 | 0.133 | 0.60 | 370.5 |
| physics/step/ms | 0.048 | 0.086 | 0.48 | 286.7 |
| stage/render/phase_sort | 0.036 | 0.061 | 0.24 | 214.3 |
| physics/insert/ms | 0.036 | 0.375 | 0.99 | 213.8 |
| stage/render/cleanup | 0.033 | 0.107 | 0.62 | 198.5 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.59 | 183.9 |
| weather/map/ms | 0.016 | 0.021 | 0.19 | 94.3 |
| pipelines/compile_ms | 0.015 | 0.000 | 79.52 | 91.6 |
| pipelines/front_ms | 0.015 | 0.000 | 76.77 | 88.6 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.17 | 87.2 |
| anim/apply/ms | 0.012 | 0.021 | 0.45 | 70.1 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.29 | 65.0 |
| controller/avoid/ms | 0.011 | 0.014 | 0.21 | 64.1 |
| weather/eval/ms | 0.010 | 0.013 | 0.05 | 57.8 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 42.8 |
| script/input/ms | 0.006 | 0.007 | 0.03 | 33.1 |
| stream/despawn/ms | 0.005 | 0.000 | 1.26 | 30.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.14 | 29.1 |
| script/ui/ms | 0.005 | 0.006 | 0.40 | 28.5 |
| controller/write/ms | 0.003 | 0.004 | 0.02 | 19.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.58 | 13.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.06 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.8 |
| stage/render/prepare | 0.001 | 0.002 | 0.40 | 4.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 2.75 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1272 | 2262 | 563 |
| crowd/c0/meshes/count | count | 349 | 646 | 144 |
| crowd/c1/count | count | 147 | 157 | 149 |
| crowd/c1/joints/count | count | 5549 | 6037 | 5633 |
| crowd/c1/meshes/count | count | 1166 | 1311 | 1197 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6402 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 163 | 216 | 205 |
| crowd/joints_extracted/count | count | 3697 | 7862 | 5775 |
| crowd/joints_uploaded/count | count | 3642 | 6206 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.43 | 1.31 | 0.34 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.17 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.29 | 0.01 |
| encode/finish_critical | ms | 0.95 | 2.25 | 0.93 |
| encode/pass:early prepass/max | ms | 0.74 | 1.66 | 0.91 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 2.25 | 0.93 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.68 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.24 | 0.88 | 0.38 |
| encode/pass:shadow_directional_light/max | ms | 0.76 | 1.94 | 0.78 |
| encode/pass:shadow_point_light/max | ms | 0.80 | 1.31 | 0.69 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.59 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.35 | 29.75 | 7.77 |
| gpu/timed/ms | ms | 7.43 | 9.90 | 7.77 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 797 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 797 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1295 | 1399 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3360 | 5136 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1295 | 1399 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3360 | 5136 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 21 | 183 | 4 |
| phases/shadow/bins | count | 3756 | 4427 | 1494 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6111 | 7285 | 3038 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 201 | 237 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 211 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 12 | 526 | 6 |
| preprocess/bind_groups_created | count | 12 | 526 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 133 | 268 | 127 |
| process/cpu_usage | % | 26.92 | 39.42 | 28.85 |
| process/mem_usage | GiB | 5.40 | 5.57 | 5.57 |
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
| skin/blocks_changed/count | count | 13 | 92 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 163 | 216 | 205 |
| skin/joint_bytes_written/bytes | bytes | 282.9 k | 808.2 k | 440.8 k |
| skin/joints_written/count | count | 4420 | 12.6 k | 6888 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4918 | 5009 | 5009 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 432 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.87 | 61.68 | 29.18 |
| system/mem_usage | % | 54.37 | 54.90 | 54.85 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 70 | 234 | 56 |
| views/all/bins | count | 6252 | 7289 | 5002 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 20.2 k | 10.7 k |
| views/all/pipelines | count | 246 | 286 | 67 |
| views/all/pipelines_distinct | count | 70 | 75 | 63 |
| views/all/work_items | count | 10.4 k | 17.6 k | 8146 |
| views/cascade/batch_sets | count | 8 | 27 | 4 |
| views/cascade/bins | count | 1450 | 1528 | 1494 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3398 | 4457 | 3038 |
| views/cascade/pipelines | count | 23 | 27 | 19 |
| views/cascade/work_items | count | 330 | 4457 | 222 |
| views/lamp_face/batch_sets | count | 0 | 172 | 0 |
| views/lamp_face/bins | count | 2460 | 2991 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 24 | 48 | 0 |
| views/lamp_face/drawn | count | 0 | 44 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2760 | 3234 | 0 |
| views/lamp_face/pipelines | count | 178 | 210 | 0 |
| views/lamp_face/revealed | count | 0 | 4 | 0 |
| views/lamp_face/work_items | count | 0 | 2675 | 0 |
| views/main/batch_sets | count | 50 | 53 | 52 |
| views/main/bins | count | 2894 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8324 | 13.0 k | 7616 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8489 | 13.3 k | 7924 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.9 k | 30.8 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 57.73 | 261 | 346.5 k |
| anim/events/count | count | 0.09 | 4 | 551 |
| assets/Image/added | count | 0.10 | 32 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.68 | 152 | 4061 |
| assets/Mesh/removed | count | 0.81 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.61 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.78 | 1274 | 4675 |
| encode//count | count | 37.37 | 203 | 224.3 k |
| encode/bind_walk/init_textures | count | 14.7 k | 104.1 k | 87.95 M |
| encode/bind_walk/init_textures_skipped | count | 53.8 k | 133.6 k | 322.87 M |
| encode/bind_walk/inits | count | 467 | 1071 | 2.80 M |
| encode/bind_walk/inits_skipped | count | 99.46 | 164 | 597.0 k |
| encode/bind_walk/merge_textures | count | 14.6 k | 104.0 k | 87.58 M |
| encode/bind_walk/merge_textures_skipped | count | 5387 | 6012 | 32.34 M |
| encode/bind_walk/merges | count | 104 | 319 | 623.4 k |
| encode/bind_walk/merges_skipped | count | 79.93 | 123 | 479.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6003 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6003 |
| encode/pass:early prepass/count | count | 1 | 1 | 6003 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6003 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6003 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6003 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6003 |
| encode/pass:shadow_point_light/count | count | 2.60 | 44 | 15.6 k |
| encode/queue/command_buffers | count | 49.99 | 257 | 300.1 k |
| encode/queue/submits | count | 2.02 | 4 | 12.1 k |
| encode/queue/write_buffer/bytes | bytes | 1.81 M | 2.75 M | 10.89 G |
| encode/queue/write_buffer/calls | count | 262 | 786 | 1.57 M |
| encode/queue/write_buffer_with/bytes | bytes | 166.6 k | 3.53 M | 1.00 G |
| encode/queue/write_buffer_with/calls | count | 11.01 | 465 | 66.1 k |
| encode/queue/write_texture/bytes | bytes | 10.5 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6003 |
| physics/inserts | count | 0.38 | 111 | 2276 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 2 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5639 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1236 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.5 ms; slowest 51, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.46 | 2460.7 | main/busy/ms 2430, stage/main/post_update 2390, stage/main/update 30.33, stage/main/pre_update 4.53, stage/main/last 1.19, surface/advance_cpu_ms 0.72, stage/main/first 0.63, stage/main/fixed_main_loop 0.61, stream/drive/ms 0.51, surface/cell_upload_ms 0.29, stream/tick/ms 0.23, bigspace/total/ms 0.11 |
| warm-up | 3.20 | 741.6 | main/busy/ms 2581, stage/main/post_update 2421, render/thread/ms 709, render/busy/ms 539, stage/render/prepare_assets 491, stage/render/prepare_views 165, render/acquire/ms 165, stage/main/update 145, stage/render/create_views 29.22, stage/render/render 14.28, render/graph/ms 7.89, stage/main/pre_update 6.53 |
| warm-up | 3.34 | 132.8 | render/thread/ms 826, render/busy/ms 598, stage/render/prepare_assets 539, main/busy/ms 180, stage/render/prepare_views 166, render/acquire/ms 165, stage/main/update 120, pipelines/compile_ms 104, stage/render/render 75.30, render/present/ms 62.64, pipelines/front_ms 53.77, pipelines/driver_ms 50.01 |
| warm-up | 3.38 | 43.4 | pipelines/compile_ms 509, pipelines/front_ms 390, pipelines/wait_ms 233, render/thread/ms 159, pipelines/driver_ms 119, stage/render/render 96.87, render/busy/ms 94.37, render/present/ms 63.83, stage/render/prepare_assets 49.26, main/busy/ms 45.54, render/graph/ms 32.09, terrain/update/ms 31.04 |
| warm-up | 3.50 | 122.8 | pipelines/wait_ms 3987, pipelines/compile_ms 1303, pipelines/front_ms 1186, render/thread/ms 162, render/busy/ms 154, pipelines/driver_ms 117, stage/render/prepare_resources 56.51, main/busy/ms 55.39, stage/render/render 46.72, stage/main/post_update 42.93, render/graph/ms 38.34, stage/render/graph_submit 30.77 |
| warm-up | 3.55 | 48.1 | pipelines/wait_ms 3756, pipelines/compile_ms 1534, pipelines/front_ms 1448, render/thread/ms 167, render/busy/ms 164, main/busy/ms 115, pipelines/driver_ms 85.82, stage/main/update 62.20, stage/render/prepare_resources 57.59, stage/main/post_update 44.40, stage/render/prepare_assets 36.03, stage/render/render 35.58 |
| warm-up | 3.67 | 124.9 | pipelines/compile_ms 1943, pipelines/front_ms 1764, pipelines/wait_ms 277, encode//sum 224, pipelines/driver_ms 179, render/thread/ms 136, render/busy/ms 133, main/busy/ms 94.26, stage/render/prepare_assets 72.50, stage/main/update 63.11, stage/render/render 51.75, render/graph/ms 49.06 |
| warm-up | 3.77 | 72.5 | pipelines/compile_ms 893, pipelines/front_ms 857, render/thread/ms 91.92, render/busy/ms 75.29, stage/render/prepare_assets 48.62, pipelines/driver_ms 36.13, main/busy/ms 24.12, encode//sum 18.57, stage/render/render 15.65, stage/render/prepare_views 14.59, render/acquire/ms 14.44, stage/main/post_update 13.71 |
| warm-up | 3.80 | 30.1 | pipelines/compile_ms 971, pipelines/front_ms 921, render/thread/ms 99.90, render/busy/ms 70.48, pipelines/driver_ms 49.85, stage/render/prepare_assets 44.42, main/busy/ms 35.14, stage/render/render 29.61, stage/main/post_update 23.97, encode/submit 18.45, encode//sum 17.96, stage/render/prepare_views 15.20 |
| warm-up | 3.93 | 87.1 | pipelines/compile_ms 512, pipelines/front_ms 497, main/busy/ms 50.58, render/thread/ms 49.18, render/busy/ms 46.35, stage/main/update 40.49, stream/drive/ms 36.15, stage/render/prepare_assets 35.47, pipelines/driver_ms 14.16, pipelines/wait_ms 11.70, stage/render/render 8.76, stage/main/post_update 6.76 |
| warm-up | 3.97 | 27.1 | pipelines/compile_ms 266, pipelines/front_ms 251, render/thread/ms 35.52, render/busy/ms 34.07, main/busy/ms 24.97, stage/render/prepare_assets 14.90, pipelines/driver_ms 14.15, stage/render/render 14.07, stage/main/update 13.74, render/graph/ms 12.17, stream/drive/ms 9.83, stage/render/graph_submit 9.55 |
| warm-up | 4.07 | 69.0 | pipelines/compile_ms 433, pipelines/front_ms 415, render/thread/ms 72.27, render/busy/ms 70.37, main/busy/ms 43.90, stage/render/prepare_assets 40.05, stage/main/update 31.25, stream/drive/ms 26.85, pipelines/driver_ms 18.38, stage/render/render 16.25, render/graph/ms 13.70, stage/render/graph_submit 10.93 |
| warm-up | 4.10 | 27.6 | pipelines/compile_ms 259, pipelines/front_ms 249, render/thread/ms 71.10, render/busy/ms 68.58, main/busy/ms 44.79, stage/render/prepare_assets 40.23, stage/main/post_update 24.77, stage/render/render 18.54, render/graph/ms 15.98, stage/render/graph_submit 13.42, stage/main/update 12.24, pipelines/driver_ms 10.02 |
| warm-up | 4.23 | 131.6 | pipelines/compile_ms 1416, pipelines/front_ms 1349, render/thread/ms 110, render/busy/ms 108, pipelines/driver_ms 66.81, main/busy/ms 56.26, stage/render/render 53.20, render/graph/ms 51.29, stage/render/prepare_assets 46.25, stage/render/graph_submit 43.63, encode/finish_wall 38.93, encode//sum 37.67 |
| warm-up | 4.31 | 77.0 | pipelines/compile_ms 1588, pipelines/front_ms 1514, render/thread/ms 158, render/busy/ms 156, pipelines/driver_ms 74.03, stage/render/render 67.30, main/busy/ms 66.69, render/graph/ms 64.27, stage/render/prepare_assets 60.40, stage/render/graph_submit 56.47, pipelines/wait_ms 46.13, stage/main/update 44.87 |
| warm-up | 4.39 | 87.8 | pipelines/compile_ms 2678, pipelines/front_ms 2587, pipelines/wait_ms 225, render/thread/ms 129, render/busy/ms 126, pipelines/driver_ms 91.44, main/busy/ms 83.79, stage/render/prepare_assets 55.69, stage/main/post_update 37.91, stage/main/update 37.13, stage/render/render 30.26, render/graph/ms 26.97 |
| warm-up | 4.47 | 80.8 | pipelines/compile_ms 2828, pipelines/front_ms 2736, pipelines/wait_ms 752, render/thread/ms 136, render/busy/ms 134, pipelines/driver_ms 91.80, main/busy/ms 88.79, stage/main/post_update 57.74, stage/render/prepare_assets 47.16, stage/render/render 28.26, render/graph/ms 25.90, encode//sum 25.57 |
| warm-up | 4.55 | 79.6 | pipelines/compile_ms 1279, pipelines/front_ms 1242, pipelines/wait_ms 1033, render/thread/ms 153, render/busy/ms 151, main/busy/ms 118, stage/main/post_update 80.10, stage/render/render 42.70, encode//sum 42.03, render/graph/ms 40.24, pipelines/driver_ms 36.96, bigspace/total/ms 30.12 |
| warm-up | 4.62 | 63.3 | pipelines/compile_ms 2413, pipelines/front_ms 2340, pipelines/wait_ms 1233, render/thread/ms 126, render/busy/ms 124, main/busy/ms 92.24, pipelines/driver_ms 72.94, stage/main/post_update 65.98, stage/render/render 32.42, stage/render/prepare_assets 30.43, bigspace/total/ms 30.15, render/graph/ms 30.10 |
| warm-up | 4.67 | 57.2 | pipelines/compile_ms 2280, pipelines/front_ms 2208, pipelines/wait_ms 1825, render/thread/ms 104, render/busy/ms 103, main/busy/ms 94.51, pipelines/driver_ms 72.28, stage/main/update 67.90, stream/drive/ms 57.67, stage/render/prepare_assets 39.67, stage/render/render 24.90, render/graph/ms 22.28 |
| warm-up | 4.73 | 57.9 | pipelines/compile_ms 1380, pipelines/wait_ms 1347, pipelines/front_ms 1328, main/busy/ms 116, stage/main/update 90.13, render/thread/ms 85.56, render/busy/ms 83.73, stream/drive/ms 59.23, pipelines/driver_ms 51.44, stage/render/prepare_assets 24.97, stage/render/render 22.82, render/graph/ms 19.58 |
| warm-up | 4.81 | 81.2 | pipelines/compile_ms 1284, pipelines/front_ms 1242, pipelines/wait_ms 434, main/busy/ms 123, stage/main/update 97.28, render/thread/ms 80.06, render/busy/ms 77.69, stream/drive/ms 51.41, pipelines/driver_ms 41.84, stage/render/prepare_assets 34.24, stage/render/render 17.38, render/graph/ms 13.94 |
| warm-up | 4.96 | 141.7 | pipelines/compile_ms 1803, pipelines/front_ms 1750, pipelines/wait_ms 387, render/thread/ms 136, render/busy/ms 134, main/busy/ms 106, stage/main/update 81.10, stage/render/prepare_assets 69.00, pipelines/driver_ms 52.62, stream/drive/ms 49.74, stage/render/render 43.80, render/graph/ms 41.11 |
| warm-up | 5.00 | 46.0 | pipelines/compile_ms 1347, pipelines/front_ms 1290, pipelines/wait_ms 248, render/thread/ms 128, render/busy/ms 125, main/busy/ms 82.19, stage/render/render 60.94, stage/main/update 58.44, render/graph/ms 57.43, pipelines/driver_ms 56.38, stage/render/graph_submit 52.04, stage/render/prepare_assets 50.14 |
| warm-up | 5.06 | 58.4 | pipelines/compile_ms 222, pipelines/front_ms 202, main/busy/ms 85.25, render/thread/ms 75.38, render/busy/ms 72.30, stage/main/update 68.36, stream/drive/ms 42.03, stage/render/render 33.12, render/graph/ms 29.63, stage/render/graph_submit 24.29, stage/render/prepare_assets 22.56, encode/submit 22.45 |
| warm-up | 5.09 | 34.8 | pipelines/compile_ms 476, pipelines/front_ms 462, main/busy/ms 72.03, render/thread/ms 58.80, render/busy/ms 57.35, stage/main/update 56.49, stream/drive/ms 33.22, stage/render/prepare_assets 27.54, pipelines/driver_ms 14.41, stage/render/render 13.08, render/graph/ms 11.25, encode//sum 10.82 |
| warm-up | 5.13 | 35.4 | pipelines/compile_ms 476, pipelines/front_ms 462, main/busy/ms 55.00, render/thread/ms 41.71, render/busy/ms 40.40, stage/main/update 39.91, stream/drive/ms 32.80, pipelines/driver_ms 14.41, stage/render/prepare_assets 13.85, stage/render/render 13.62, render/graph/ms 11.87, encode//sum 11.05 |
| warm-up | 5.16 | 28.0 | pipelines/compile_ms 337, pipelines/front_ms 326, render/thread/ms 43.20, render/busy/ms 41.78, main/busy/ms 30.31, stage/render/render 18.95, render/graph/ms 16.87, encode//sum 12.58, stage/main/post_update 12.39, stage/main/update 12.30, stage/render/graph_submit 11.59, pipelines/driver_ms 11.34 |
| warm-up | 5.21 | 31.0 | pipelines/compile_ms 106, pipelines/front_ms 102, render/thread/ms 40.76, render/busy/ms 39.50, main/busy/ms 28.69, stage/render/prepare_assets 14.05, stage/render/render 13.21, stage/main/post_update 11.99, render/graph/ms 11.44, stage/main/update 11.18, encode//sum 10.22, stage/render/graph_submit 7.10 |
| warm-up | 5.24 | 31.5 | pipelines/compile_ms 177, pipelines/front_ms 171, render/thread/ms 53.93, render/busy/ms 51.58, main/busy/ms 30.09, stage/render/render 18.00, render/graph/ms 15.57, stage/render/prepare_assets 14.83, stage/main/update 13.01, stage/main/post_update 11.63, stage/render/graph_submit 11.05, encode//sum 10.38 |
| warm-up | 5.27 | 28.8 | pipelines/compile_ms 557, pipelines/front_ms 539, render/thread/ms 53.22, render/busy/ms 50.83, main/busy/ms 34.38, pipelines/driver_ms 17.71, stage/render/render 17.67, stage/main/update 15.96, render/graph/ms 15.27, stage/render/prepare_assets 12.00, encode//sum 11.28, stage/render/graph_submit 10.49 |
| warm-up | 5.31 | 41.5 | pipelines/compile_ms 896, pipelines/front_ms 873, render/thread/ms 53.80, render/busy/ms 52.49, main/busy/ms 36.09, stage/render/prepare_assets 28.46, pipelines/driver_ms 23.31, stage/main/update 19.78, stage/render/render 12.95, render/graph/ms 11.07, stream/drive/ms 10.54, encode//sum 10.22 |
| warm-up | 5.41 | 31.1 | pipelines/compile_ms 394, pipelines/front_ms 386, render/thread/ms 44.72, render/busy/ms 43.43, main/busy/ms 31.64, stage/render/prepare_assets 21.27, stage/main/update 14.00, stage/render/render 12.09, stage/main/post_update 11.85, encode//sum 11.38, render/graph/ms 10.05, pipelines/driver_ms 8.55 |
| warm-up | 5.44 | 27.4 | pipelines/compile_ms 382, pipelines/front_ms 373, render/thread/ms 47.45, render/busy/ms 46.14, stage/render/prepare_assets 28.14, main/busy/ms 26.56, stage/main/update 11.78, stage/render/render 11.02, stage/main/post_update 9.49, encode//sum 9.41, render/graph/ms 9.24, pipelines/driver_ms 8.78 |
| warm-up | 5.52 | 33.3 | pipelines/compile_ms 334, pipelines/front_ms 326, render/thread/ms 53.58, render/busy/ms 52.05, main/busy/ms 40.79, stage/main/update 22.96, stage/render/prepare_assets 20.73, stage/render/render 15.64, render/graph/ms 13.30, stage/main/post_update 12.83, encode//sum 12.16, stream/drive/ms 11.74 |
| warm-up | 5.55 | 31.2 | render/thread/ms 60.25, render/busy/ms 58.45, main/busy/ms 31.06, stage/render/render 25.64, render/graph/ms 23.07, stage/render/prepare_assets 18.52, stage/render/graph_submit 17.52, stage/main/post_update 14.06, stage/main/update 12.25, pipelines/compile_ms 11.73, encode//sum 11.43, pipelines/front_ms 10.58 |
| warm-up | 5.61 | 61.7 | pipelines/compile_ms 428, pipelines/front_ms 404, render/thread/ms 81.23, render/busy/ms 79.62, main/busy/ms 48.35, stage/render/prepare_assets 33.12, pipelines/wait_ms 29.86, stage/main/post_update 24.19, pipelines/driver_ms 23.59, stage/render/render 23.41, render/graph/ms 21.06, stage/render/graph_submit 16.71 |
| warm-up | 5.64 | 36.3 | pipelines/compile_ms 815, pipelines/front_ms 777, render/thread/ms 82.61, render/busy/ms 81.16, main/busy/ms 56.52, stage/render/prepare_assets 42.77, pipelines/driver_ms 38.07, stage/main/post_update 30.36, pipelines/wait_ms 29.78, stage/render/render 15.29, render/graph/ms 12.82, stage/main/update 12.04 |
| warm-up | 5.67 | 28.0 | pipelines/compile_ms 773, pipelines/front_ms 747, render/thread/ms 55.29, render/busy/ms 53.84, stage/render/prepare_assets 32.40, main/busy/ms 31.24, pipelines/driver_ms 26.16, stage/main/post_update 16.49, stage/render/render 13.08, render/graph/ms 10.72, encode//sum 10.35, stage/main/update 7.49 |
| warm-up | 5.89 | 27.1 | pipelines/compile_ms 684, pipelines/front_ms 663, main/busy/ms 36.89, render/thread/ms 29.83, render/busy/ms 28.40, pipelines/driver_ms 20.95, stage/main/update 20.09, stream/drive/ms 15.12, stage/render/render 15.00, render/graph/ms 13.04, stage/main/post_update 11.61, encode//sum 10.75 |
| warm-up | 6.11 | 38.4 | render/thread/ms 34.84, render/busy/ms 33.77, main/busy/ms 31.50, stage/render/render 23.53, render/graph/ms 21.81, stage/main/update 19.25, stage/render/graph_submit 18.82, encode/submit 16.67, stream/drive/ms 12.77, stage/main/post_update 8.59, encode//sum 8.22, stage/render/prepare_assets 4.42 |
| warm-up | 6.25 | 30.8 | render/thread/ms 38.47, render/busy/ms 37.45, main/busy/ms 23.16, stage/render/prepare_assets 16.13, stage/render/render 10.88, render/graph/ms 9.34, stage/main/update 8.89, encode//sum 8.65, stage/main/post_update 8.57, stage/render/graph_submit 6.55, encode/pass:shadow_point_light/sum 4.22, encode/submit 4.21 |
| warm-up | 6.50 | 32.7 | render/thread/ms 46.27, render/busy/ms 44.75, main/busy/ms 39.62, stage/render/prepare_assets 17.02, stage/main/update 16.98, stage/render/render 16.32, stage/main/post_update 16.04, render/graph/ms 14.12, encode//sum 11.66, stage/render/graph_submit 10.12, encode/pass:shadow_point_light/sum 8.05, encode/submit 6.94 |
| warm-up | 6.58 | 37.0 | main/busy/ms 44.79, render/thread/ms 36.45, render/busy/ms 34.68, stage/main/update 24.01, stage/render/render 15.48, stage/main/post_update 15.17, stream/drive/ms 15.02, pipelines/compile_ms 14.87, pipelines/front_ms 14.04, render/graph/ms 12.90, encode//sum 11.07, stage/render/graph_submit 8.63 |
| warm-up | 6.60 | 28.4 | render/thread/ms 44.30, render/busy/ms 42.44, main/busy/ms 38.92, pipelines/compile_ms 20.68, pipelines/front_ms 20.37, stage/render/render 18.98, stage/main/post_update 16.80, render/graph/ms 16.26, stage/main/update 15.80, stage/render/graph_submit 11.39, encode//sum 11.03, stage/render/prepare_assets 10.53 |
| warm-up | 6.63 | 26.7 | render/thread/ms 42.21, render/busy/ms 40.77, main/busy/ms 39.30, pipelines/compile_ms 20.68, pipelines/front_ms 20.37, stage/main/update 18.02, stage/render/render 15.79, stage/render/prepare_assets 15.24, stage/main/post_update 14.56, render/graph/ms 13.76, stage/render/graph_submit 10.08, encode//sum 8.86 |
| warm-up | 6.77 | 28.6 | pipelines/compile_ms 158, pipelines/front_ms 154, render/thread/ms 42.81, render/busy/ms 41.55, main/busy/ms 24.84, stage/render/prepare_assets 16.64, stage/render/render 14.91, stage/main/post_update 13.14, render/graph/ms 12.95, encode//sum 12.05, stage/render/graph_submit 8.80, stage/main/update 7.94 |
| warm-up | 7.01 | 33.5 | main/busy/ms 40.37, render/thread/ms 26.55, render/busy/ms 25.47, stage/main/update 20.70, stage/main/post_update 14.39, stream/drive/ms 14.25, stream/spawn/ms 12.98, stage/render/render 12.34, render/graph/ms 10.92, encode//sum 8.40, stage/render/graph_submit 7.39, stage/render/prepare_assets 5.68 |
| measure | 0.77 | 27.4 | main/busy/ms 35.41, render/thread/ms 29.15, render/busy/ms 27.88, stage/main/update 17.94, stage/render/render 15.19, stage/main/post_update 14.28, render/graph/ms 13.28, stream/drive/ms 12.51, encode//sum 10.63, stage/render/graph_submit 9.26, encode/pass:shadow_point_light/sum 7.20, encode/submit 6.46 |
| measure | 18.18 | 32.9 | main/busy/ms 39.86, stage/main/update 25.09, render/thread/ms 22.52, render/busy/ms 21.46, stream/drive/ms 20.44, stage/render/render 13.05, stage/main/post_update 12.18, render/graph/ms 11.47, encode//sum 10.45, stage/render/graph_submit 8.43, encode/submit 5.64, encode/pass:shadow_point_light/sum 4.15 |
| measure | 59.05 | 30.5 | render/thread/ms 40.45, render/busy/ms 39.35, main/busy/ms 25.07, stage/render/prepare_assets 15.60, stage/render/render 13.81, stage/main/post_update 13.12, render/graph/ms 12.37, encode//sum 11.70, stage/render/graph_submit 8.92, stage/main/update 7.50, encode/submit 4.78, encode/finish_wall 4.19 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.face_cull=on --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=facecull-5 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 9%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **72%** (min 49%) · 2625 MHz (min 2625) · memory 8101 MHz · 75.8 W · 76 °C (max 81) · VRAM 6790 MiB · throttled 100% of samples

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
- `shadow.lamp.face_cull=on (command line: --set)`
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
