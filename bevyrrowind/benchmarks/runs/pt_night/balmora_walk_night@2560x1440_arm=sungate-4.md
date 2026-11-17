# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.gate=on (command line: --set); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6110 frames over 65.6 s · **median 10.29 ms (97 fps)** · mean 10.74 · p95 13.66 · p99 15.41 (1% low 65 fps) · worst 32.03 ms · 59 spikes over 1.5× median · 25.6 stutters/min (work 21.0/min)
- warm-up: 323 frames over 8.9 s, worst 2480.1 ms
- hitches (> 25.7 ms): **4** measured, 51 in warm-up
- timed GPU passes: 6.66 ms of a 10.74 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.29 ms, limit 11 ms
- **FAIL** frame_ms: 28 stutters (> 1.5x moving median of 60) in 1.09 min = 25.59/min, limit 1/min
- pass worst frame 32.03 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.017 | 2.030 | 2.063 | 0.99 | 0.171 |
| main_opaque_pass_3d | 1.287 | 1.295 | 1.644 | 0.99 | 0.068 |
| early prepass | 0.486 | 0.489 | 0.700 | 0.99 | 0.087 |
| bloom | 0.314 | 0.316 | 0.322 | 0.99 | 0.062 |
| atmosphere_luts | 0.307 | 0.309 | 0.371 | 0.99 | 0.004 |
| water_velocity | 0.261 | 0.263 | 0.350 | 0.99 | 0.002 |
| auxiliary_views | 0.211 | 0.212 | 0.938 | 0.99 | 0.200 |
| water_sspr | 0.200 | 0.201 | 0.203 | 0.99 | 0.002 |
| ssao | 0.187 | 0.188 | 0.191 | 0.99 | 0.007 |
| upscaling | 0.129 | 0.130 | 0.136 | 0.99 | 0.004 |
| water_fft | 0.128 | 0.129 | 0.137 | 0.99 | 0.003 |
| atmos_froxel_inject | 0.126 | 0.127 | 0.128 | 0.99 | 0.001 |
| uniform_allocation | 0.125 | 0.126 | 0.154 | 0.99 | 0.026 |
| clouds_composite | 0.104 | 0.105 | 0.108 | 0.99 | 0.002 |
| bin_unpacking | 0.097 | 0.098 | 0.119 | 0.99 | 0.019 |
| tonemapping | 0.086 | 0.087 | 0.091 | 0.99 | 0.003 |
| main_transmissive_pass_3d | 0.077 | 0.078 | 0.155 | 0.99 | 0.009 |
| clustering | 0.063 | 0.064 | 0.067 | 0.99 | 0.019 |
| clouds_trace | 0.055 | 0.056 | 0.074 | 0.99 | 0.001 |
| render_sky | 0.051 | 0.051 | 0.052 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.050 | 0.412 | 0.415 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 0.99 | 0.001 |
| surface_trail | 0.039 | 0.056 | 0.057 | 0.70 | 0.002 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.028 | 0.036 | 0.99 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.229 | 0.12 | 0.005 |
| water_snapshot | 0.025 | 0.025 | 0.028 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.038 | 0.99 | 0.001 |
| main_transparent_pass_3d | 0.013 | 0.013 | 0.024 | 0.99 | 0.123 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.99 | 0.005 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.313 | 12.244 | 24.79 | 56905.5 |
| main/busy/ms | 8.966 | 10.878 | 29.46 | 54785.2 |
| render/busy/ms | 8.805 | 11.720 | 24.06 | 53798.5 |
| stage/main/post_update | 5.570 | 7.122 | 9.69 | 34031.9 |
| stage/render/render | 5.562 | 7.833 | 15.77 | 33981.5 |
| render/graph/ms | 4.870 | 7.097 | 14.71 | 29755.9 |
| encode//sum | 4.021 | 5.925 | 18.51 | 24567.0 |
| stage/render/graph_submit | 3.632 | 5.168 | 10.30 | 22191.6 |
| encode/submit | 2.334 | 3.506 | 6.40 | 14259.0 |
| stage/main/update | 2.141 | 2.612 | 22.72 | 13079.9 |
| encode/finish_wall | 1.331 | 1.794 | 3.99 | 8133.6 |
| stage/render/camera_driver | 1.185 | 1.888 | 4.34 | 7241.6 |
| encode/pass:shadow_point_light/sum | 1.092 | 4.546 | 17.05 | 6672.9 |
| encode/pass:shadow_directional_light/sum | 0.767 | 1.233 | 2.06 | 4688.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.752 | 1.064 | 1.97 | 4596.7 |
| encode/pass:early prepass/sum | 0.745 | 1.068 | 2.10 | 4553.9 |
| stage/render/prepare_write_phase_buffers | 0.739 | 1.069 | 2.32 | 4516.5 |
| stage/render/prepare_assets | 0.577 | 0.722 | 10.53 | 3523.5 |
| bigspace/total/ms | 0.563 | 1.365 | 2.09 | 3442.2 |
| stage/render/prepare_resources | 0.455 | 0.539 | 1.79 | 2777.0 |
| stage/render/prepare_meshes | 0.427 | 0.547 | 1.60 | 2611.5 |
| render/present/ms | 0.415 | 0.505 | 2.08 | 2535.8 |
| stage/main/first | 0.395 | 0.515 | 5.79 | 2410.8 |
| stage/main/pre_update | 0.356 | 0.500 | 1.35 | 2176.2 |
| terrain/update/ms | 0.312 | 0.402 | 2.21 | 1908.0 |
| terrain/select/ms | 0.279 | 0.360 | 0.92 | 1704.9 |
| controller/feed/ms | 0.271 | 0.336 | 0.75 | 1658.7 |
| stage/render/specialize | 0.258 | 0.392 | 2.21 | 1575.1 |
| stage/render/prepare_bind_groups | 0.258 | 0.439 | 2.40 | 1574.2 |
| stream/drive/ms | 0.248 | 0.324 | 20.67 | 1515.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.243 | 0.360 | 0.91 | 1484.3 |
| stage/render/queue | 0.237 | 0.372 | 2.86 | 1446.5 |
| bigspace/low_precision/ms | 0.234 | 0.737 | 1.83 | 1427.9 |
| stage/main/fixed_main_loop | 0.215 | 0.370 | 1.08 | 1312.9 |
| far/select/ms | 0.199 | 0.263 | 0.60 | 1214.0 |
| stage/render/create_views | 0.193 | 0.250 | 0.94 | 1180.8 |
| stream/tick/ms | 0.191 | 0.257 | 0.54 | 1167.8 |
| stage/render/prepare_batch_phases | 0.186 | 0.261 | 0.64 | 1134.6 |
| stage/render/extract_commands | 0.173 | 0.203 | 0.56 | 1054.4 |
| stage/render/prepare_flush | 0.170 | 0.345 | 1.15 | 1036.5 |
| anim/evaluate/ms | 0.169 | 0.308 | 1.69 | 1035.3 |
| bigspace/high_precision/ms | 0.164 | 0.696 | 1.74 | 1002.1 |
| anim/evaluate_cpu/ms | 0.157 | 0.199 | 0.81 | 962.3 |
| stage/render/prepare_views | 0.137 | 0.170 | 0.40 | 837.3 |
| controller/move/ms | 0.132 | 0.262 | 0.71 | 805.9 |
| stage/main/last | 0.114 | 0.153 | 1.69 | 698.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.101 | 0.166 | 0.73 | 617.7 |
| render/acquire/ms | 0.093 | 0.120 | 0.36 | 571.2 |
| stage/render/prepare_collect_phase_buffers | 0.060 | 0.131 | 0.51 | 365.5 |
| physics/step/ms | 0.047 | 0.086 | 0.59 | 287.8 |
| stage/render/phase_sort | 0.034 | 0.056 | 0.50 | 206.9 |
| physics/insert/ms | 0.033 | 0.360 | 0.88 | 201.5 |
| stage/render/cleanup | 0.031 | 0.093 | 0.75 | 189.3 |
| encode/resolve_diagnostics/sum | 0.029 | 0.039 | 0.61 | 179.9 |
| weather/map/ms | 0.016 | 0.021 | 0.26 | 97.2 |
| pipelines/compile_ms | 0.016 | 0.000 | 84.19 | 95.7 |
| pipelines/front_ms | 0.015 | 0.000 | 81.45 | 92.8 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.51 | 90.2 |
| anim/apply/ms | 0.012 | 0.022 | 0.52 | 72.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.18 | 67.7 |
| controller/avoid/ms | 0.010 | 0.014 | 0.05 | 63.8 |
| weather/eval/ms | 0.010 | 0.012 | 0.09 | 58.6 |
| script/update/ms | 0.007 | 0.009 | 0.19 | 45.5 |
| script/input/ms | 0.005 | 0.007 | 0.18 | 33.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.07 | 31.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.13 | 28.2 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 26.0 |
| controller/write/ms | 0.003 | 0.004 | 0.49 | 21.1 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 14.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 6.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.15 | 3.7 |
| pipelines/driver_ms | 0.000 | 0.000 | 2.74 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1277 | 2262 | 595 |
| crowd/c0/meshes/count | count | 352 | 646 | 158 |
| crowd/c1/count | count | 146 | 156 | 148 |
| crowd/c1/joints/count | count | 5532 | 5991 | 5601 |
| crowd/c1/meshes/count | count | 1167 | 1295 | 1183 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 164 | 216 | 205 |
| crowd/joints_extracted/count | count | 3699 | 8263 | 5775 |
| crowd/joints_uploaded/count | count | 3649 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.40 | 1.84 | 0.49 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.51 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.18 | 0.01 |
| encode/finish_critical | ms | 0.94 | 2.10 | 1.01 |
| encode/pass:early prepass/max | ms | 0.77 | 2.10 | 0.79 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.77 | 1.97 | 1.01 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.73 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.91 | 0.28 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 2.06 | 0.78 |
| encode/pass:shadow_point_light/max | ms | 0.78 | 1.32 | 0.91 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.61 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.04 | 29.46 | 8.61 |
| gpu/timed/ms | ms | 7.38 | 10.78 | 7.72 |
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
| phases/alpha_mask3d/items | count | 799 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 799 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1294 | 1399 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3367 | 5149 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1294 | 1399 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3367 | 5149 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 14 | 215 | 4 |
| phases/shadow/bins | count | 3651 | 4421 | 1494 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6109 | 7443 | 3038 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 201 | 230 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 183 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 12 | 592 | 6 |
| preprocess/bind_groups_created | count | 12 | 592 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 132 | 261 | 127 |
| process/cpu_usage | % | 26.46 | 37.98 | 27.88 |
| process/mem_usage | GiB | 5.41 | 5.57 | 5.57 |
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
| skin/blocks_changed/count | count | 14 | 107 | 5 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 163 | 216 | 205 |
| skin/joint_bytes_written/bytes | bytes | 281.6 k | 808.7 k | 443.1 k |
| skin/joints_written/count | count | 4400 | 12.6 k | 6924 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4919 | 5011 | 5011 |
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
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.68 | 46.45 | 29.85 |
| system/mem_usage | % | 54.29 | 54.75 | 54.75 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 65 | 264 | 56 |
| views/all/bins | count | 6193 | 7336 | 5002 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.5 k | 20.3 k | 10.7 k |
| views/all/pipelines | count | 248 | 271 | 67 |
| views/all/pipelines_distinct | count | 71 | 74 | 63 |
| views/all/work_items | count | 10.5 k | 17.8 k | 8144 |
| views/cascade/batch_sets | count | 8 | 27 | 4 |
| views/cascade/bins | count | 1443 | 1543 | 1494 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3401 | 4441 | 3038 |
| views/cascade/pipelines | count | 22 | 27 | 19 |
| views/cascade/work_items | count | 316 | 4437 | 222 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2359 | 3001 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2709 | 3325 | 0 |
| views/lamp_face/pipelines | count | 179 | 205 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2886 | 0 |
| views/main/batch_sets | count | 51 | 53 | 52 |
| views/main/bins | count | 2888 | 3590 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8332 | 13.0 k | 7614 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8501 | 13.2 k | 7922 |
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
| anim/bones_written/count | count | 58.06 | 257 | 354.8 k |
| anim/events/count | count | 0.09 | 3 | 548 |
| assets/Image/added | count | 0.10 | 39 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.66 | 152 | 4061 |
| assets/Mesh/removed | count | 0.79 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.07 | 29 | 457 |
| assets/StandardMaterial/modified | count | 3.55 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1244 | 4675 |
| encode//count | count | 35.54 | 219 | 217.1 k |
| encode/bind_walk/init_textures | count | 13.5 k | 115.1 k | 82.67 M |
| encode/bind_walk/init_textures_skipped | count | 57.9 k | 117.6 k | 353.80 M |
| encode/bind_walk/inits | count | 461 | 1169 | 2.82 M |
| encode/bind_walk/inits_skipped | count | 103 | 144 | 629.4 k |
| encode/bind_walk/merge_textures | count | 13.5 k | 115.1 k | 82.28 M |
| encode/bind_walk/merge_textures_skipped | count | 5389 | 6012 | 32.93 M |
| encode/bind_walk/merges | count | 101 | 350 | 620.0 k |
| encode/bind_walk/merges_skipped | count | 81.76 | 125 | 499.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6110 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6110 |
| encode/pass:early prepass/count | count | 1 | 1 | 6110 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6110 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6110 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6110 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6110 |
| encode/pass:shadow_point_light/count | count | 2.15 | 48 | 13.1 k |
| encode/queue/command_buffers | count | 47.70 | 277 | 291.5 k |
| encode/queue/submits | count | 2.02 | 4 | 12.3 k |
| encode/queue/write_buffer/bytes | bytes | 1.81 M | 2.74 M | 11.05 G |
| encode/queue/write_buffer/calls | count | 258 | 883 | 1.58 M |
| encode/queue/write_buffer_with/bytes | bytes | 170.0 k | 3.52 M | 1.04 G |
| encode/queue/write_buffer_with/calls | count | 10.98 | 465 | 67.1 k |
| encode/queue/write_texture/bytes | bytes | 10.2 k | 8.39 M | 62.48 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 562 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6110 |
| physics/inserts | count | 0.37 | 87 | 2265 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2335 |
| physics/steps | count | 0.64 | 2 | 3939 |
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
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5540 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1215 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.7 ms; slowest 55, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.48 | 2480.1 | main/busy/ms 2448, stage/main/post_update 2405, stage/main/update 34.37, stage/main/pre_update 5.38, stage/main/last 0.99, surface/advance_cpu_ms 0.74, stage/main/first 0.58, stream/drive/ms 0.40, stage/main/fixed_main_loop 0.37, surface/cell_upload_ms 0.30, bigspace/total/ms 0.09, stream/tick/ms 0.08 |
| warm-up | 3.15 | 671.4 | main/busy/ms 2622, stage/main/post_update 2429, render/thread/ms 638, render/busy/ms 507, stage/render/prepare_assets 478, stage/main/update 178, stage/render/prepare_views 128, render/acquire/ms 127, stage/render/render 11.64, stage/render/create_views 9.16, stage/main/pre_update 7.23, render/graph/ms 7.09 |
| warm-up | 3.26 | 105.0 | render/thread/ms 725, render/busy/ms 547, stage/render/prepare_assets 505, main/busy/ms 194, stage/main/update 148, stage/render/prepare_views 128, render/acquire/ms 128, pipelines/compile_ms 93.12, stage/render/render 62.79, pipelines/front_ms 60.30, render/present/ms 49.98, stage/main/post_update 36.19 |
| warm-up | 3.30 | 42.2 | pipelines/compile_ms 253, pipelines/front_ms 209, render/thread/ms 118, stage/render/render 75.98, render/busy/ms 66.61, main/busy/ms 61.12, render/present/ms 50.68, stage/main/post_update 46.18, pipelines/driver_ms 43.46, stage/render/prepare_assets 28.03, pipelines/wait_ms 26.88, terrain/update/ms 25.81 |
| warm-up | 3.40 | 103.0 | pipelines/wait_ms 3899, pipelines/compile_ms 1318, pipelines/front_ms 1201, pipelines/driver_ms 117, render/thread/ms 112, render/busy/ms 107, main/busy/ms 69.01, stage/main/post_update 52.07, stage/render/render 51.68, render/graph/ms 44.18, stage/render/graph_submit 34.82, stage/render/prepare_resources 30.10 |
| warm-up | 3.48 | 82.1 | pipelines/wait_ms 4936, pipelines/compile_ms 1619, pipelines/front_ms 1493, render/thread/ms 162, render/busy/ms 160, pipelines/driver_ms 127, main/busy/ms 121, stage/main/update 74.34, stage/render/prepare_resources 67.69, stage/main/post_update 35.75, stage/render/render 31.88, render/graph/ms 27.84 |
| warm-up | 3.56 | 79.0 | pipelines/compile_ms 1704, pipelines/front_ms 1616, pipelines/wait_ms 1184, render/thread/ms 141, render/busy/ms 139, main/busy/ms 110, pipelines/driver_ms 88.39, stage/main/update 76.73, stage/render/prepare_assets 59.25, stage/render/prepare_resources 39.89, stage/main/post_update 25.08, terrain/update/ms 14.83 |
| warm-up | 3.63 | 40.4 | pipelines/compile_ms 794, pipelines/front_ms 761, pipelines/wait_ms 72.37, render/thread/ms 60.33, render/busy/ms 58.34, encode//sum 56.03, main/busy/ms 38.90, stage/render/prepare_assets 34.18, pipelines/driver_ms 32.46, stage/main/post_update 17.92, stage/render/render 17.89, render/graph/ms 15.45 |
| warm-up | 3.76 | 76.0 | pipelines/compile_ms 1311, pipelines/front_ms 1260, main/busy/ms 55.72, pipelines/driver_ms 51.01, render/thread/ms 49.43, render/busy/ms 47.39, stage/main/update 44.84, stream/drive/ms 40.37, stage/render/prepare_assets 29.77, stage/render/render 12.12, render/graph/ms 9.82, stage/main/post_update 6.55 |
| warm-up | 3.84 | 59.6 | pipelines/compile_ms 346, pipelines/front_ms 329, render/thread/ms 58.59, render/busy/ms 56.82, main/busy/ms 42.47, stage/render/prepare_assets 33.15, stage/main/update 28.85, stream/drive/ms 24.23, stage/render/render 18.84, pipelines/driver_ms 17.24, render/graph/ms 16.46, stage/render/graph_submit 12.19 |
| warm-up | 3.88 | 36.8 | pipelines/compile_ms 589, pipelines/front_ms 564, render/thread/ms 72.53, render/busy/ms 70.04, main/busy/ms 43.43, stage/render/prepare_assets 40.19, stage/main/update 27.16, pipelines/driver_ms 25.35, stream/drive/ms 21.68, stage/render/render 17.72, render/graph/ms 13.77, pipelines/wait_ms 12.27 |
| warm-up | 3.94 | 64.0 | pipelines/compile_ms 279, pipelines/front_ms 270, render/thread/ms 73.97, render/busy/ms 71.38, main/busy/ms 43.90, stage/render/prepare_assets 38.94, stage/main/update 27.81, stream/drive/ms 22.06, stage/render/render 19.39, render/graph/ms 15.35, stage/render/graph_submit 10.96, encode//sum 10.57 |
| warm-up | 3.98 | 39.2 | pipelines/compile_ms 858, pipelines/front_ms 833, render/thread/ms 76.12, render/busy/ms 73.89, stage/render/prepare_assets 51.46, main/busy/ms 25.69, pipelines/driver_ms 25.47, stage/render/render 15.76, render/graph/ms 12.82, stage/render/graph_submit 10.04, stage/main/update 9.00, stage/main/post_update 8.77 |
| warm-up | 4.04 | 58.8 | pipelines/compile_ms 922, pipelines/front_ms 895, render/thread/ms 77.33, render/busy/ms 75.04, stage/render/prepare_assets 53.05, main/busy/ms 32.15, pipelines/driver_ms 27.68, stage/render/render 16.35, render/graph/ms 13.55, stage/main/post_update 12.62, stage/main/update 10.58, stage/render/graph_submit 8.12 |
| warm-up | 4.09 | 51.5 | pipelines/compile_ms 957, pipelines/front_ms 925, render/thread/ms 90.10, render/busy/ms 88.24, main/busy/ms 81.44, stage/main/update 59.69, stage/render/prepare_assets 43.11, pipelines/driver_ms 32.13, stage/main/post_update 13.81, stage/render/render 13.11, stage/render/prepare_write_phase_buffers 12.44, render/graph/ms 10.91 |
| warm-up | 4.23 | 136.4 | pipelines/compile_ms 2339, pipelines/front_ms 2261, render/thread/ms 135, render/busy/ms 133, pipelines/wait_ms 104, main/busy/ms 82.01, pipelines/driver_ms 78.66, stage/main/update 60.32, stage/render/prepare_assets 58.09, stage/render/prepare_resources 33.66, stage/main/post_update 14.59, stage/render/prepare_write_phase_buffers 14.11 |
| warm-up | 4.31 | 77.3 | pipelines/compile_ms 2858, pipelines/front_ms 2766, pipelines/wait_ms 902, render/thread/ms 163, render/busy/ms 161, pipelines/driver_ms 92.91, main/busy/ms 92.76, stage/main/post_update 63.17, stage/render/prepare_assets 51.99, stage/render/prepare_resources 35.79, stage/render/prepare_batch_phases 23.46, stage/render/render 17.30 |
| warm-up | 4.37 | 64.7 | pipelines/compile_ms 1889, pipelines/front_ms 1831, pipelines/wait_ms 1055, render/thread/ms 129, render/busy/ms 127, main/busy/ms 126, stage/main/post_update 75.60, pipelines/driver_ms 57.54, stage/render/prepare_batch_phases 28.00, encode//sum 26.92, stage/render/prepare_flush 26.88, stage/render/render 23.58 |
| warm-up | 4.44 | 72.6 | pipelines/compile_ms 1353, pipelines/front_ms 1311, pipelines/wait_ms 567, render/thread/ms 117, render/busy/ms 115, main/busy/ms 81.10, stage/render/prepare_assets 44.92, pipelines/driver_ms 41.93, encode//sum 34.13, stage/main/post_update 31.09, stage/render/render 27.04, render/graph/ms 24.28 |
| warm-up | 4.52 | 72.4 | pipelines/compile_ms 2592, pipelines/front_ms 2511, pipelines/wait_ms 1354, render/thread/ms 130, render/busy/ms 128, main/busy/ms 104, encode//sum 93.24, pipelines/driver_ms 81.87, stage/main/update 74.17, stream/drive/ms 62.38, stage/render/render 41.47, stage/render/prepare_assets 40.31 |
| warm-up | 4.56 | 48.9 | pipelines/compile_ms 2043, pipelines/front_ms 1975, pipelines/wait_ms 1236, main/busy/ms 122, render/thread/ms 104, render/busy/ms 102, stage/main/update 87.96, encode//sum 83.89, pipelines/driver_ms 68.75, stream/drive/ms 63.99, stage/render/render 41.55, render/graph/ms 37.20 |
| warm-up | 4.65 | 80.9 | pipelines/compile_ms 783, pipelines/front_ms 759, pipelines/wait_ms 310, main/busy/ms 129, stage/main/update 96.39, render/thread/ms 88.11, render/busy/ms 85.49, stream/drive/ms 54.23, stage/render/prepare_assets 31.30, pipelines/driver_ms 23.71, stage/render/render 23.66, render/graph/ms 19.25 |
| warm-up | 4.79 | 145.4 | pipelines/compile_ms 2125, pipelines/front_ms 2083, pipelines/wait_ms 513, render/thread/ms 141, render/busy/ms 139, main/busy/ms 110, stage/main/update 84.55, stage/render/prepare_assets 69.14, stream/drive/ms 52.63, stage/render/render 46.65, render/graph/ms 44.00, pipelines/driver_ms 41.43 |
| warm-up | 4.89 | 80.7 | main/busy/ms 85.64, stage/main/update 67.35, render/thread/ms 52.29, render/busy/ms 50.87, stream/drive/ms 40.24, pipelines/compile_ms 27.15, pipelines/front_ms 25.10, stage/render/prepare_assets 20.86, stage/render/render 12.76, stage/main/post_update 10.91, render/graph/ms 10.76, encode//sum 10.71 |
| warm-up | 4.94 | 49.9 | pipelines/compile_ms 1117, pipelines/front_ms 1078, render/thread/ms 70.99, render/busy/ms 69.54, main/busy/ms 67.42, stage/main/update 49.06, pipelines/driver_ms 39.51, stage/render/prepare_assets 36.51, stream/drive/ms 26.78, stage/render/render 13.75, render/graph/ms 11.79, stage/main/post_update 11.32 |
| warm-up | 4.99 | 26.0 | render/thread/ms 41.74, render/busy/ms 40.39, main/busy/ms 33.56, stage/render/render 16.12, stage/main/update 15.84, render/graph/ms 14.40, stage/main/post_update 11.59, encode//sum 9.91, stage/render/graph_submit 9.62, encode/submit 6.66, stage/render/prepare_write_phase_buffers 4.71, stage/render/prepare_assets 4.63 |
| warm-up | 5.02 | 27.4 | pipelines/compile_ms 114, pipelines/front_ms 109, render/thread/ms 46.19, render/busy/ms 44.79, main/busy/ms 34.68, stage/main/update 16.17, stage/render/render 15.08, render/graph/ms 13.19, stage/main/post_update 12.26, encode//sum 10.06, stage/render/prepare_assets 9.15, stage/render/graph_submit 9.14 |
| warm-up | 5.04 | 27.3 | pipelines/compile_ms 114, pipelines/front_ms 109, render/thread/ms 45.58, render/busy/ms 44.16, main/busy/ms 33.44, stage/main/update 15.84, stage/render/render 15.47, render/graph/ms 13.42, stage/main/post_update 11.75, stage/render/prepare_assets 11.25, encode//sum 10.21, stage/render/graph_submit 9.68 |
| warm-up | 5.08 | 39.3 | render/thread/ms 58.04, render/busy/ms 56.67, main/busy/ms 43.81, stage/main/post_update 19.92, stage/main/update 17.70, stage/render/render 15.94, render/graph/ms 13.82, stage/render/prepare_write_phase_buffers 13.62, encode//sum 10.29, stage/render/prepare_assets 9.68, stage/render/graph_submit 9.59, encode/submit 7.22 |
| warm-up | 5.11 | 27.9 | pipelines/compile_ms 322, pipelines/front_ms 311, render/thread/ms 57.17, render/busy/ms 55.80, main/busy/ms 44.22, stage/main/post_update 20.96, stage/main/update 17.36, stage/render/render 15.41, render/graph/ms 13.37, stage/render/prepare_write_phase_buffers 13.07, stage/render/prepare_assets 11.60, pipelines/driver_ms 10.77 |
| warm-up | 5.15 | 42.3 | pipelines/compile_ms 1865, pipelines/front_ms 1812, render/thread/ms 54.14, pipelines/driver_ms 53.32, render/busy/ms 52.68, main/busy/ms 48.11, stage/main/update 24.32, stage/render/render 20.89, render/graph/ms 18.71, stage/main/post_update 17.16, stage/render/prepare_assets 16.76, stage/render/graph_submit 12.03 |
| warm-up | 5.18 | 28.3 | pipelines/compile_ms 1659, pipelines/front_ms 1610, render/thread/ms 55.53, render/busy/ms 54.10, main/busy/ms 51.04, pipelines/driver_ms 49.15, stage/main/update 27.41, stage/render/render 19.74, stage/render/prepare_assets 18.37, render/graph/ms 17.67, stage/main/post_update 17.06, stream/drive/ms 11.93 |
| warm-up | 5.32 | 37.4 | pipelines/compile_ms 361, pipelines/front_ms 350, render/thread/ms 52.15, render/busy/ms 50.54, main/busy/ms 48.44, stage/main/update 25.36, stage/render/render 20.28, stage/main/post_update 17.90, render/graph/ms 17.86, stage/render/prepare_assets 16.69, stream/drive/ms 12.92, encode//sum 12.35 |
| warm-up | 5.36 | 36.2 | pipelines/compile_ms 899, pipelines/front_ms 875, render/thread/ms 63.01, render/busy/ms 61.45, main/busy/ms 42.98, stage/render/prepare_assets 30.90, stage/main/update 24.80, pipelines/driver_ms 24.54, stage/render/render 17.99, render/graph/ms 15.55, stage/main/post_update 12.77, stream/drive/ms 12.70 |
| warm-up | 5.47 | 27.7 | pipelines/compile_ms 62.96, pipelines/front_ms 59.77, render/thread/ms 42.51, render/busy/ms 41.25, main/busy/ms 23.65, stage/render/prepare_assets 19.79, stage/render/render 14.58, render/graph/ms 12.63, encode//sum 11.45, stage/main/post_update 11.18, stage/render/graph_submit 8.75, stage/main/update 7.66 |
| warm-up | 5.51 | 27.8 | pipelines/compile_ms 163, pipelines/front_ms 158, render/thread/ms 38.13, render/busy/ms 36.98, main/busy/ms 27.15, stage/render/prepare_assets 17.94, stage/render/render 12.63, stage/main/update 12.20, render/graph/ms 10.91, stage/main/post_update 9.95, encode//sum 8.69, stream/drive/ms 7.25 |
| warm-up | 5.65 | 27.3 | main/busy/ms 32.48, render/thread/ms 25.85, render/busy/ms 24.71, stage/main/update 19.34, stream/drive/ms 14.90, stage/render/render 13.27, render/graph/ms 11.79, encode//sum 10.65, stage/main/post_update 8.49, stage/render/graph_submit 7.90, encode/submit 5.31, stage/render/prepare_assets 5.23 |
| warm-up | 5.88 | 27.0 | pipelines/compile_ms 131, pipelines/front_ms 127, main/busy/ms 31.35, render/thread/ms 23.22, render/busy/ms 22.08, stage/main/update 20.22, stream/drive/ms 14.03, stage/render/render 11.58, render/graph/ms 9.81, encode//sum 8.13, stage/main/post_update 7.39, stage/render/graph_submit 7.03 |
| warm-up | 5.99 | 29.8 | render/thread/ms 35.13, render/busy/ms 33.61, main/busy/ms 31.48, stage/render/render 13.74, stage/main/update 12.47, stage/main/post_update 12.09, render/graph/ms 11.76, stage/render/prepare_assets 9.52, encode//sum 9.40, stage/render/graph_submit 9.07, encode/submit 6.67, encode/pass:shadow_point_light/sum 5.22 |
| warm-up | 6.02 | 28.9 | render/thread/ms 39.92, render/busy/ms 38.91, main/busy/ms 26.92, stage/render/prepare_assets 17.37, stage/main/update 10.80, stage/render/render 9.63, stage/main/post_update 9.00, render/graph/ms 8.27, encode//sum 8.11, stage/render/graph_submit 5.67, encode/pass:shadow_point_light/sum 4.51, stream/drive/ms 4.43 |
| warm-up | 6.12 | 26.7 | pipelines/compile_ms 156, pipelines/front_ms 151, render/thread/ms 37.20, render/busy/ms 36.05, main/busy/ms 31.14, stage/main/update 18.94, stage/render/render 13.11, render/graph/ms 11.36, stage/render/prepare_assets 10.43, encode//sum 10.32, stage/main/post_update 8.63, stage/render/graph_submit 6.99 |
| warm-up | 6.21 | 26.9 | render/thread/ms 36.40, main/busy/ms 35.68, render/busy/ms 35.28, stage/main/update 19.39, stage/render/render 13.15, stage/main/post_update 11.66, encode//sum 11.36, render/graph/ms 11.25, stage/render/prepare_assets 8.06, stage/render/graph_submit 7.17, encode/pass:shadow_point_light/sum 5.61, stream/drive/ms 5.35 |
| warm-up | 6.24 | 26.8 | render/thread/ms 43.71, main/busy/ms 42.11, render/busy/ms 42.07, stage/main/post_update 17.86, stage/main/update 17.36, stage/render/render 15.78, render/graph/ms 13.54, stage/render/prepare_assets 13.05, encode//sum 11.46, stage/render/graph_submit 9.34, encode/pass:shadow_point_light/sum 7.25, encode/submit 6.17 |
| warm-up | 6.29 | 28.7 | render/thread/ms 37.37, render/busy/ms 36.25, main/busy/ms 27.84, stage/render/prepare_assets 16.55, stage/main/update 12.02, stage/render/render 9.85, stage/main/post_update 9.80, encode//sum 8.67, render/graph/ms 8.25, stage/render/graph_submit 5.31, encode/pass:shadow_point_light/sum 5.09, stream/drive/ms 4.58 |
| warm-up | 6.41 | 29.0 | render/thread/ms 48.37, render/busy/ms 47.30, stage/render/prepare_assets 29.35, main/busy/ms 21.88, stage/render/render 10.62, stage/main/post_update 9.40, render/graph/ms 9.01, stage/main/update 8.89, encode//sum 8.73, stage/render/graph_submit 5.98, encode/pass:shadow_point_light/sum 5.44, encode/submit 3.57 |
| warm-up | 6.45 | 33.3 | render/thread/ms 43.74, render/busy/ms 42.39, main/busy/ms 31.99, stage/render/render 17.39, stage/render/prepare_assets 16.05, render/graph/ms 15.09, stage/main/post_update 14.45, encode//sum 12.86, stage/main/update 12.49, stage/render/graph_submit 10.02, encode/pass:shadow_point_light/sum 9.75, encode/submit 6.39 |
| warm-up | 6.49 | 33.8 | pipelines/compile_ms 150, pipelines/front_ms 146, render/thread/ms 57.10, render/busy/ms 55.52, main/busy/ms 49.64, stage/main/update 29.97, stage/render/prepare_assets 27.45, stage/render/render 18.86, render/graph/ms 16.42, stream/drive/ms 14.24, encode//sum 13.11, stage/main/post_update 12.81 |
| warm-up | 6.55 | 59.7 | pipelines/compile_ms 150, pipelines/front_ms 146, render/thread/ms 69.66, render/busy/ms 68.12, main/busy/ms 51.14, stage/render/prepare_assets 43.79, stage/main/update 32.38, stage/render/render 16.60, render/graph/ms 14.42, stream/drive/ms 14.12, stage/main/post_update 12.14, stage/render/graph_submit 10.79 |
| warm-up | 6.60 | 27.9 | render/thread/ms 42.76, render/busy/ms 41.60, main/busy/ms 28.36, pipelines/compile_ms 21.92, stage/render/prepare_assets 21.61, pipelines/front_ms 20.64, stage/main/update 12.01, stage/render/render 11.05, stage/main/post_update 11.02, render/graph/ms 9.51, encode//sum 8.98, encode/pass:shadow_point_light/sum 6.83 |
| warm-up | 6.75 | 26.4 | pipelines/compile_ms 143, pipelines/front_ms 140, render/thread/ms 36.51, render/busy/ms 35.23, main/busy/ms 29.25, stage/render/render 18.91, encode//sum 17.22, render/graph/ms 16.67, stage/main/post_update 14.12, stage/render/graph_submit 10.89, stage/main/update 10.54, encode/pass:shadow_point_light/sum 9.55 |
| warm-up | 6.83 | 27.2 | main/busy/ms 36.74, render/thread/ms 24.60, render/busy/ms 23.21, stage/main/update 22.09, stream/drive/ms 11.94, stream/spawn/ms 10.43, stage/main/post_update 10.27, stage/render/render 9.88, render/graph/ms 8.09, encode//sum 7.67, stage/render/graph_submit 5.67, stage/render/prepare_assets 4.72 |
| measure | 0.74 | 25.8 | main/busy/ms 32.00, render/thread/ms 25.01, render/busy/ms 23.95, stage/main/update 16.83, stream/drive/ms 12.37, stage/main/post_update 12.16, stage/render/render 10.76, render/graph/ms 9.31, encode//sum 7.93, stage/render/graph_submit 6.61, stage/render/prepare_assets 6.23, encode/submit 4.36 |
| measure | 2.72 | 25.9 | render/thread/ms 36.26, render/busy/ms 35.03, stage/render/render 22.99, encode//sum 22.82, render/graph/ms 21.23, encode/pass:shadow_point_light/sum 20.61, main/busy/ms 19.26, stage/render/graph_submit 15.11, stage/main/post_update 12.55, encode/submit 9.61, stage/render/camera_driver 5.99, encode/finish_wall 5.57 |
| measure | 18.16 | 32.0 | main/busy/ms 39.28, stage/main/update 24.65, stream/drive/ms 20.91, render/thread/ms 20.15, render/busy/ms 19.13, stage/main/post_update 12.01, stage/render/render 11.66, render/graph/ms 10.20, encode//sum 9.52, stage/render/graph_submit 7.55, encode/submit 4.90, encode/pass:shadow_point_light/sum 2.93 |
| measure | 59.05 | 28.3 | render/thread/ms 37.61, render/busy/ms 36.25, main/busy/ms 27.01, stage/main/post_update 14.63, stage/render/render 13.98, stage/render/prepare_assets 13.73, render/graph/ms 12.33, encode//sum 9.75, stage/render/graph_submit 9.35, stage/main/update 7.48, encode/submit 6.14, encode/finish_wall 3.33 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.sun.gate=on --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=sungate-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 5%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **73%** (min 45%) · 2625 MHz (min 2625) · memory 8101 MHz · 76.6 W · 76 °C (max 82) · VRAM 6754 MiB · throttled 100% of samples

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
- `shadow.sun.gate=on (command line: --set)`
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
