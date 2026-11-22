# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=custom (preset: derived from its members); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=0 (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6807 frames over 65.6 s · **median 9.52 ms (105 fps)** · mean 9.64 · p95 11.13 · p99 13.01 (1% low 77 fps) · worst 32.22 ms · 37 spikes over 1.5× median · 32.9 stutters/min (work 26.5/min)
- warm-up: 353 frames over 8.8 s, worst 2313.4 ms
- hitches (> 25.0 ms): **3** measured, 41 in warm-up
- timed GPU passes: 6.32 ms of a 9.64 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.52 ms, limit 11 ms
- **FAIL** frame_ms: 36 stutters (> 1.5x moving median of 60) in 1.09 min = 32.91/min, limit 1/min
- pass worst frame 32.22 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.886 | 2.038 | 2.071 | 0.93 | 0.175 |
| main_opaque_pass_3d | 1.415 | 1.529 | 2.351 | 0.93 | 0.081 |
| early prepass | 0.472 | 0.510 | 0.754 | 0.93 | 0.093 |
| bloom | 0.294 | 0.318 | 0.325 | 0.93 | 0.062 |
| atmosphere_luts | 0.291 | 0.314 | 0.378 | 0.93 | 0.004 |
| water_velocity | 0.209 | 0.225 | 0.247 | 0.93 | 0.002 |
| water_sspr | 0.187 | 0.202 | 0.203 | 0.93 | 0.002 |
| ssao | 0.175 | 0.189 | 0.193 | 0.93 | 0.008 |
| atmos_froxel_inject | 0.119 | 0.128 | 0.130 | 0.93 | 0.002 |
| upscaling | 0.119 | 0.128 | 0.135 | 0.93 | 0.004 |
| water_fft | 0.114 | 0.123 | 0.127 | 0.93 | 0.003 |
| uniform_allocation | 0.109 | 0.118 | 0.142 | 0.93 | 0.025 |
| clouds_composite | 0.097 | 0.105 | 0.109 | 0.93 | 0.002 |
| bin_unpacking | 0.086 | 0.093 | 0.114 | 0.93 | 0.018 |
| gpu_instance_materialization | 0.085 | 0.092 | 0.096 | 0.93 | 0.004 |
| tonemapping | 0.080 | 0.087 | 0.092 | 0.93 | 0.004 |
| main_transmissive_pass_3d | 0.066 | 0.071 | 0.131 | 0.93 | 0.008 |
| clustering | 0.059 | 0.064 | 0.068 | 0.93 | 0.021 |
| early_mesh_preprocessing | 0.051 | 0.055 | 0.071 | 0.93 | 0.004 |
| clouds_trace | 0.051 | 0.055 | 0.078 | 0.93 | 0.001 |
| render_sky | 0.049 | 0.053 | 0.054 | 0.93 | 0.002 |
| clouds_resolve | 0.046 | 0.050 | 0.051 | 0.93 | 0.001 |
| lightprobe_irradiance_map | 0.037 | 0.416 | 0.418 | 0.09 | 0.001 |
| surface_trail | 0.034 | 0.056 | 0.057 | 0.59 | 0.002 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.031 | 0.93 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.027 | 0.93 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.93 | 0.002 |
| range_unpacking | 0.020 | 0.022 | 0.026 | 0.93 | 0.004 |
| lightprobe_radiance_map | 0.020 | 0.220 | 0.230 | 0.09 | 0.004 |
| night_sky | 0.016 | 0.017 | 0.045 | 0.93 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.017 | 0.026 | 0.93 | 0.006 |
| main_transparent_pass_3d | 0.013 | 0.014 | 0.031 | 0.93 | 0.183 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.93 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.93 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.92 | 0.004 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.93 | 0.004 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.93 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.445 | 9.663 | 21.11 | 57486.8 |
| render/busy/ms | 7.930 | 9.054 | 20.08 | 53978.3 |
| main/busy/ms | 7.903 | 8.992 | 28.73 | 53794.6 |
| stage/render/render | 4.905 | 5.840 | 10.50 | 33386.0 |
| stage/main/post_update | 4.428 | 5.197 | 9.03 | 30141.2 |
| render/graph/ms | 4.261 | 5.019 | 9.29 | 29001.9 |
| encode//sum | 3.437 | 4.143 | 8.80 | 23394.9 |
| stage/render/graph_submit | 3.064 | 3.734 | 6.69 | 20859.3 |
| encode/submit | 1.890 | 2.659 | 4.26 | 12864.9 |
| stage/main/update | 1.847 | 2.328 | 22.07 | 12571.3 |
| encode/finish_wall | 1.253 | 1.590 | 3.09 | 8532.2 |
| stage/render/camera_driver | 1.134 | 1.356 | 2.48 | 7722.0 |
| encode/pass:early prepass/sum | 0.868 | 1.107 | 2.20 | 5905.1 |
| encode/pass:shadow_directional_light/sum | 0.865 | 1.260 | 2.52 | 5886.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.852 | 1.105 | 2.45 | 5798.2 |
| bigspace/total/ms | 0.575 | 1.369 | 2.27 | 3917.4 |
| stage/render/prepare_write_phase_buffers | 0.559 | 0.772 | 1.49 | 3803.2 |
| stage/render/prepare_assets | 0.529 | 0.656 | 12.14 | 3603.7 |
| stage/main/fixed_main_loop | 0.500 | 0.950 | 1.87 | 3400.6 |
| stage/render/prepare_meshes | 0.447 | 0.556 | 4.34 | 3039.8 |
| render/present/ms | 0.424 | 1.161 | 2.08 | 2886.4 |
| stage/render/prepare_resources | 0.418 | 0.498 | 0.98 | 2846.4 |
| stage/main/pre_update | 0.387 | 0.496 | 2.40 | 2636.8 |
| controller/feed/ms | 0.334 | 0.415 | 0.88 | 2270.9 |
| stage/main/first | 0.308 | 0.379 | 5.60 | 2094.6 |
| stage/render/prepare_bind_groups | 0.292 | 0.454 | 1.65 | 1989.2 |
| stage/render/prepare_flush | 0.287 | 0.508 | 1.20 | 1953.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.278 | 0.442 | 1.07 | 1890.7 |
| stage/render/prepare_batch_phases | 0.269 | 0.410 | 1.24 | 1832.5 |
| stage/main/last | 0.254 | 0.299 | 0.80 | 1726.8 |
| bigspace/low_precision/ms | 0.242 | 0.723 | 2.06 | 1648.8 |
| stream/drive/ms | 0.212 | 0.294 | 10.09 | 1443.3 |
| stage/render/extract_commands | 0.192 | 0.223 | 0.67 | 1305.0 |
| anim/evaluate/ms | 0.182 | 0.439 | 1.68 | 1240.6 |
| bigspace/high_precision/ms | 0.181 | 0.804 | 1.81 | 1233.6 |
| stage/render/create_views | 0.167 | 0.209 | 0.49 | 1134.8 |
| stream/tick/ms | 0.158 | 0.227 | 0.46 | 1074.7 |
| anim/evaluate_cpu/ms | 0.157 | 0.198 | 0.92 | 1070.1 |
| stage/render/prepare_views | 0.134 | 0.174 | 1.12 | 911.5 |
| stage/render/specialize | 0.131 | 0.244 | 1.46 | 893.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.097 | 0.151 | 0.74 | 663.6 |
| render/acquire/ms | 0.091 | 0.120 | 1.04 | 622.1 |
| stage/render/queue | 0.090 | 0.160 | 1.49 | 614.8 |
| stage/render/phase_sort | 0.077 | 0.106 | 0.40 | 521.1 |
| controller/write/ms | 0.075 | 0.103 | 0.50 | 510.8 |
| controller/move/ms | 0.071 | 0.222 | 0.69 | 481.4 |
| terrain/update/ms | 0.059 | 0.290 | 1.67 | 398.9 |
| stage/render/prepare_collect_phase_buffers | 0.045 | 0.072 | 0.35 | 308.4 |
| physics/step/ms | 0.039 | 0.082 | 0.72 | 265.0 |
| encode/resolve_diagnostics/sum | 0.033 | 0.045 | 0.62 | 226.5 |
| physics/insert/ms | 0.024 | 0.261 | 0.92 | 162.6 |
| pipelines/compile_ms | 0.024 | 0.000 | 82.97 | 162.3 |
| stage/render/cleanup | 0.023 | 0.032 | 0.54 | 159.6 |
| pipelines/front_ms | 0.023 | 0.000 | 80.27 | 156.8 |
| terrain/select/ms | 0.021 | 0.256 | 0.93 | 145.2 |
| weather/map/ms | 0.019 | 0.025 | 0.44 | 126.1 |
| far/select/ms | 0.015 | 0.167 | 0.50 | 101.0 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.66 | 99.8 |
| anim/apply/ms | 0.012 | 0.023 | 0.46 | 84.8 |
| encode/camera_driver_span_end/sum | 0.012 | 0.020 | 0.61 | 78.5 |
| weather/eval/ms | 0.011 | 0.014 | 0.37 | 73.2 |
| script/update/ms | 0.008 | 0.012 | 0.08 | 56.6 |
| controller/avoid/ms | 0.006 | 0.012 | 0.23 | 40.1 |
| script/input/ms | 0.005 | 0.006 | 0.18 | 36.4 |
| stream/despawn/ms | 0.005 | 0.000 | 1.21 | 36.3 |
| script/ui/ms | 0.003 | 0.005 | 0.21 | 23.5 |
| stream/spawn/ms | 0.003 | 0.000 | 1.15 | 22.7 |
| stream/upload/ms | 0.003 | 0.000 | 1.35 | 20.3 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 13.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 7.1 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.75 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.13 | 4.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 32 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1302 | 2262 | 583 |
| crowd/c0/meshes/count | count | 355 | 646 | 162 |
| crowd/c1/count | count | 145 | 157 | 148 |
| crowd/c1/joints/count | count | 5497 | 5990 | 5613 |
| crowd/c1/meshes/count | count | 1166 | 1286 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6372 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 216 | 205 |
| crowd/joints_extracted/count | count | 693 | 6856 | 573 |
| crowd/joints_uploaded/count | count | 3573 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.4 k | 47.7 k |
| encode//max | ms | 0.42 | 1.20 | 0.61 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.66 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.61 | 0.02 |
| encode/finish_critical | ms | 0.98 | 2.52 | 0.97 |
| encode/pass:early prepass/max | ms | 0.86 | 2.20 | 0.88 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.85 | 2.45 | 0.97 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.74 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.26 | 1.07 | 0.26 |
| encode/pass:shadow_directional_light/max | ms | 0.84 | 2.52 | 0.83 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.62 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.27 | 28.73 | 8.42 |
| gpu/timed/ms | ms | 7.31 | 9.44 | 8.42 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.5 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 149 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 315 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 149 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 315 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 768 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2482 | 3550 | 2457 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 768 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2482 | 3550 | 2457 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 13 | 34 | 8 |
| phases/shadow/bins | count | 716 | 858 | 671 |
| phases/shadow/instance_batches | count | 4 | 8 | 4 |
| phases/shadow/instance_draws | count | 6557 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| phases/shadow/items | count | 2116 | 2402 | 2027 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 24 | 28 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2708 | 3182 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 332 | 333 | 333 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 172 | 11 |
| preprocess/bind_groups_created | count | 8 | 166 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 5 |
| preprocess/bind_groups_reused | count | 125 | 147 | 123 |
| process/cpu_usage | % | 25.48 | 40.29 | 36.06 |
| process/mem_usage | GiB | 4.95 | 5.24 | 5.24 |
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
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 0 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 0 | 0 | 0 |
| shadow_atlas/live_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/live_tiles | count | 0 | 0 | 0 |
| shadow_atlas/mover_draws | count | 0 | 0 | 0 |
| shadow_atlas/movers | count | 0 | 0 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 0 | 0 | 0 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/live_direct | count | 0 | 0 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 0 | 0 | 0 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/static_tiles | count | 0 | 0 | 0 |
| shadow_atlas/views | count | 0 | 0 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 119 | 12 |
| skin/blocks_skipped/count | count | 145 | 206 | 193 |
| skin/blocks_walked/count | count | 14 | 27 | 12 |
| skin/joint_bytes_written/bytes | bytes | 48.6 k | 811.6 k | 37.9 k |
| skin/joints_written/count | count | 760 | 12.7 k | 592 |
| skin/upload_writes/count | count | 6 | 23 | 8 |
| started_load_count |  | 6439 | 6563 | 6563 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 237 | 333 | 180 |
| statics/materials_bound | count | 237 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.05 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.24 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.76 | 52.86 | 35.17 |
| system/mem_usage | % | 46.48 | 47.27 | 47.27 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2885 | 2997 | 2997 |
| textures/images | count | 2664 | 2719 | 2719 |
| views/all/batch_sets | count | 72 | 94 | 66 |
| views/all/bins | count | 2756 | 3043 | 2961 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 12 | 16 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 147.5 k |
| views/all/items | count | 7964 | 9730 | 7971 |
| views/all/pipelines | count | 73 | 76 | 66 |
| views/all/pipelines_distinct | count | 62 | 64 | 61 |
| views/all/work_items | count | 154.0 k | 206.5 k | 154.0 k |
| views/cascade/batch_sets | count | 13 | 34 | 8 |
| views/cascade/bins | count | 716 | 858 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6557 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2116 | 2402 | 2027 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
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
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2014 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5908 | 7522 | 5944 |
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
| world/meshes | count | 21.5 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 46.87 | 201 | 319.0 k |
| anim/events/count | count | 0.08 | 2 | 549 |
| assets/Image/added | count | 0.09 | 37 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.42 | 296 | 2874 |
| assets/Mesh/removed | count | 0.71 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.10 | 10 | 7481 |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 9 |
| ecs/entities_delta | count | 0.22 | 1154 | 1490 |
| encode//count | count | 27.90 | 30 | 189.9 k |
| encode/bind_walk/init_textures | count | 8477 | 11.1 k | 57.70 M |
| encode/bind_walk/init_textures_skipped | count | 53.2 k | 75.6 k | 362.09 M |
| encode/bind_walk/inits | count | 524 | 769 | 3.57 M |
| encode/bind_walk/inits_skipped | count | 135 | 234 | 921.2 k |
| encode/bind_walk/merge_textures | count | 8414 | 11.0 k | 57.27 M |
| encode/bind_walk/merge_textures_skipped | count | 8395 | 14.6 k | 57.14 M |
| encode/bind_walk/merges | count | 90.84 | 103 | 618.4 k |
| encode/bind_walk/merges_skipped | count | 121 | 225 | 821.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6807 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6807 |
| encode/pass:early prepass/count | count | 1 | 1 | 6807 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6807 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6807 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6807 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6807 |
| encode/queue/command_buffers | count | 37.91 | 40 | 258.1 k |
| encode/queue/submits | count | 2.01 | 4 | 13.7 k |
| encode/queue/write_buffer/bytes | bytes | 2.31 M | 3.43 M | 15.72 G |
| encode/queue/write_buffer/calls | count | 244 | 318 | 1.66 M |
| encode/queue/write_buffer_with/bytes | bytes | 19.1 k | 1.14 M | 130.08 M |
| encode/queue/write_buffer_with/calls | count | 10.25 | 897 | 69.8 k |
| encode/queue/write_texture/bytes | bytes | 1784 | 589.8 k | 12.15 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 556 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6807 |
| physics/inserts | count | 0.33 | 110 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.34 | 175 | 2335 |
| physics/steps | count | 0.58 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 2 |
| precache/hit | count | 0.00 | 1 | 1 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 33 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 4973 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1090 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 44, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.31 | 2313.4 | main/busy/ms 2285, stage/main/post_update 2242, stage/main/update 31.42, stage/main/pre_update 8.43, stage/main/last 1.19, surface/advance_cpu_ms 0.79, stage/main/first 0.59, stream/drive/ms 0.44, stage/main/fixed_main_loop 0.42, surface/cell_upload_ms 0.31, stream/tick/ms 0.10, bigspace/total/ms 0.09 |
| warm-up | 3.04 | 723.0 | main/busy/ms 2422, stage/main/post_update 2265, render/thread/ms 690, render/busy/ms 545, stage/render/prepare_assets 484, stage/render/prepare_views 143, render/acquire/ms 142, stage/main/update 138, stage/render/create_views 28.70, stage/render/render 22.92, render/graph/ms 19.39, stage/render/camera_driver 17.11 |
| warm-up | 3.20 | 167.2 | render/thread/ms 838, render/busy/ms 605, stage/render/prepare_assets 530, main/busy/ms 164, stage/render/prepare_views 143, render/acquire/ms 143, stage/render/render 116, stage/main/update 115, pipelines/compile_ms 93.44, render/present/ms 90.44, pipelines/driver_ms 77.80, stage/main/post_update 35.35 |
| warm-up | 3.49 | 275.4 | pipelines/wait_ms 3609, pipelines/compile_ms 2715, pipelines/front_ms 2482, pipelines/driver_ms 233, main/busy/ms 156, render/thread/ms 156, render/busy/ms 154, stage/render/prepare_assets 133, stage/main/update 90.25, stage/main/post_update 23.38, stage/main/fixed_main_loop 19.65, terrain/update/ms 15.01 |
| warm-up | 3.52 | 28.1 | pipelines/wait_ms 3611, pipelines/compile_ms 2538, pipelines/front_ms 2416, render/thread/ms 171, render/busy/ms 169, stage/render/prepare_assets 146, pipelines/driver_ms 123, main/busy/ms 47.39, stage/main/post_update 21.53, terrain/update/ms 15.91, stage/render/render 15.53, stage/main/update 13.79 |
| warm-up | 3.59 | 31.8 | pipelines/compile_ms 94.98, pipelines/front_ms 88.86, render/thread/ms 47.55, render/busy/ms 45.30, main/busy/ms 44.74, stage/main/update 24.88, pipelines/wait_ms 21.10, encode//sum 19.12, stage/render/prepare_write_phase_buffers 18.94, stage/main/post_update 16.03, stage/render/render 15.27, render/graph/ms 13.03 |
| warm-up | 3.65 | 54.4 | pipelines/compile_ms 361, pipelines/front_ms 342, pipelines/wait_ms 92.55, main/busy/ms 84.70, render/thread/ms 83.50, render/busy/ms 81.64, stage/main/update 56.45, stage/main/post_update 23.58, stage/render/prepare_write_phase_buffers 20.01, stage/render/prepare_assets 19.46, pipelines/driver_ms 18.75, stage/render/prepare_flush 13.42 |
| warm-up | 3.72 | 76.3 | pipelines/compile_ms 1475, pipelines/front_ms 1416, render/thread/ms 106, render/busy/ms 104, pipelines/wait_ms 97.14, main/busy/ms 91.79, stage/main/update 61.87, pipelines/driver_ms 58.78, stage/render/prepare_assets 55.51, stage/main/post_update 23.52, stage/render/render 15.88, stage/render/prepare_flush 13.32 |
| warm-up | 3.76 | 39.2 | pipelines/compile_ms 2032, pipelines/front_ms 1956, render/thread/ms 80.68, render/busy/ms 78.78, pipelines/driver_ms 76.49, stage/render/prepare_assets 52.75, main/busy/ms 48.33, stage/main/update 31.54, pipelines/wait_ms 30.76, stage/render/render 17.89, render/graph/ms 15.51, stage/render/graph_submit 12.37 |
| warm-up | 3.81 | 46.2 | pipelines/compile_ms 1238, pipelines/front_ms 1182, render/thread/ms 58.35, render/busy/ms 56.90, pipelines/driver_ms 55.69, stage/render/prepare_assets 36.79, pipelines/wait_ms 34.29, main/busy/ms 23.08, stage/render/render 15.04, render/graph/ms 13.45, stage/render/graph_submit 10.97, stage/main/update 10.42 |
| warm-up | 3.93 | 74.9 | pipelines/compile_ms 267, pipelines/front_ms 257, render/thread/ms 68.03, render/busy/ms 66.19, main/busy/ms 46.71, stage/main/update 29.78, stream/drive/ms 18.73, stage/render/prepare_assets 17.26, stage/render/render 15.16, stage/render/prepare_resources 14.55, render/graph/ms 12.97, stage/main/post_update 12.84 |
| warm-up | 3.97 | 42.6 | pipelines/compile_ms 1071, pipelines/front_ms 1030, render/thread/ms 82.09, render/busy/ms 80.22, pipelines/driver_ms 40.40, main/busy/ms 39.32, stage/render/prepare_assets 29.35, stage/main/update 20.39, stage/render/render 15.44, stage/render/prepare_resources 15.34, render/graph/ms 12.76, stage/main/post_update 12.36 |
| warm-up | 4.04 | 69.4 | pipelines/compile_ms 1793, pipelines/front_ms 1732, main/busy/ms 150, stage/main/update 131, render/thread/ms 71.96, render/busy/ms 70.19, pipelines/driver_ms 61.26, stage/render/prepare_assets 44.20, pipelines/wait_ms 17.14, stage/render/render 16.08, render/graph/ms 13.54, stage/render/graph_submit 10.36 |
| warm-up | 4.18 | 143.3 | pipelines/compile_ms 1870, pipelines/front_ms 1820, main/busy/ms 143, stage/main/update 127, render/thread/ms 75.36, render/busy/ms 73.75, stage/render/prepare_assets 54.55, pipelines/driver_ms 50.02, pipelines/wait_ms 13.15, stage/render/render 12.99, render/graph/ms 10.96, stage/render/graph_submit 8.60 |
| warm-up | 4.29 | 67.5 | pipelines/compile_ms 96.18, pipelines/front_ms 91.37, main/busy/ms 70.52, render/thread/ms 49.34, render/busy/ms 42.57, stage/main/update 33.66, stage/main/post_update 29.11, stage/render/render 17.78, stage/render/prepare_assets 14.40, stream/drive/ms 12.49, stage/render/cleanup 11.17, render/graph/ms 10.66 |
| warm-up | 4.35 | 55.3 | pipelines/compile_ms 126, pipelines/front_ms 118, render/thread/ms 76.42, render/busy/ms 71.21, main/busy/ms 71.02, stage/main/post_update 31.91, stage/main/update 27.86, stage/render/prepare_assets 24.13, pipelines/wait_ms 19.74, stage/render/prepare_resources 18.22, stage/render/render 15.01, stream/drive/ms 12.17 |
| warm-up | 4.41 | 68.4 | pipelines/compile_ms 1426, pipelines/front_ms 1379, render/thread/ms 93.16, render/busy/ms 91.72, main/busy/ms 59.90, pipelines/driver_ms 46.77, stage/render/prepare_assets 35.98, stage/main/update 29.29, pipelines/wait_ms 21.56, stage/render/render 20.54, render/graph/ms 18.53, stage/render/prepare_resources 18.06 |
| warm-up | 4.47 | 51.2 | pipelines/compile_ms 2111, pipelines/front_ms 2047, render/thread/ms 86.70, render/busy/ms 85.09, pipelines/driver_ms 63.86, main/busy/ms 54.30, stage/render/prepare_assets 47.51, stage/main/update 26.65, stage/render/render 22.12, render/graph/ms 19.84, stage/main/post_update 13.17, pipelines/wait_ms 12.61 |
| warm-up | 4.52 | 56.8 | pipelines/compile_ms 1208, pipelines/front_ms 1176, main/busy/ms 76.10, render/thread/ms 75.87, render/busy/ms 73.94, stage/main/update 53.11, stage/render/prepare_assets 40.53, pipelines/driver_ms 32.45, stage/render/render 19.65, render/graph/ms 16.95, encode//sum 14.64, stage/render/graph_submit 11.95 |
| warm-up | 4.59 | 64.6 | pipelines/compile_ms 1097, pipelines/front_ms 1065, main/busy/ms 121, stage/main/update 94.05, render/thread/ms 71.32, render/busy/ms 69.41, stream/drive/ms 45.48, stage/render/prepare_assets 40.38, pipelines/driver_ms 32.63, stage/render/render 18.84, render/graph/ms 16.31, encode//sum 14.35 |
| warm-up | 4.65 | 65.5 | pipelines/compile_ms 1159, pipelines/front_ms 1128, main/busy/ms 98.24, stage/main/update 61.66, render/thread/ms 59.66, render/busy/ms 57.99, stream/drive/ms 50.13, pipelines/driver_ms 31.24, stage/render/prepare_assets 28.52, stage/main/post_update 22.51, stage/render/render 16.30, render/graph/ms 14.21 |
| warm-up | 4.70 | 46.2 | pipelines/compile_ms 615, pipelines/front_ms 601, main/busy/ms 79.72, render/thread/ms 63.65, render/busy/ms 60.30, stage/main/update 43.05, pipelines/wait_ms 29.73, stage/render/render 24.47, stage/main/post_update 21.50, render/graph/ms 20.66, stage/render/graph_submit 16.38, pipelines/driver_ms 14.44 |
| warm-up | 4.76 | 59.0 | pipelines/compile_ms 632, pipelines/front_ms 614, render/thread/ms 70.34, render/busy/ms 67.22, main/busy/ms 66.95, stage/main/update 41.57, pipelines/wait_ms 27.61, stage/render/prepare_assets 21.65, stage/render/render 18.67, pipelines/driver_ms 18.63, render/graph/ms 15.09, stage/render/graph_submit 11.49 |
| warm-up | 4.84 | 35.5 | pipelines/compile_ms 643, pipelines/front_ms 621, render/thread/ms 45.79, render/busy/ms 44.14, main/busy/ms 38.00, pipelines/driver_ms 21.87, stage/main/update 16.56, stage/main/post_update 15.56, stage/render/render 14.62, stage/render/prepare_resources 13.07, render/graph/ms 12.45, stage/render/graph_submit 9.02 |
| warm-up | 4.93 | 29.3 | pipelines/compile_ms 199, pipelines/front_ms 193, main/busy/ms 34.35, render/thread/ms 31.36, render/busy/ms 29.87, stage/main/update 15.89, stage/render/render 11.98, stage/main/post_update 10.15, render/graph/ms 10.00, pipelines/wait_ms 8.76, pipelines/lock_ms 8.44, encode//sum 7.42 |
| warm-up | 4.97 | 27.6 | pipelines/compile_ms 180, pipelines/front_ms 174, render/thread/ms 40.65, render/busy/ms 39.31, main/busy/ms 37.00, stage/main/update 22.14, stage/render/render 11.98, stream/drive/ms 11.98, render/graph/ms 10.18, stage/main/post_update 9.39, stage/render/prepare_resources 7.77, stage/render/graph_submit 7.19 |
| warm-up | 5.50 | 33.9 | pipelines/compile_ms 903, pipelines/front_ms 882, main/busy/ms 43.07, render/thread/ms 28.02, render/busy/ms 26.29, stage/main/update 22.21, pipelines/driver_ms 21.09, stage/render/render 16.24, stage/main/post_update 15.05, stream/drive/ms 14.36, render/graph/ms 13.97, encode//sum 7.83 |
| warm-up | 5.53 | 27.1 | pipelines/compile_ms 293, pipelines/front_ms 286, render/thread/ms 41.12, render/busy/ms 39.31, main/busy/ms 36.65, stage/main/post_update 17.39, stage/render/render 14.61, pipelines/wait_ms 13.14, stage/main/update 12.47, render/graph/ms 12.28, stage/render/extract_commands 10.12, stage/render/prepare_assets 9.06 |
| warm-up | 5.57 | 33.8 | pipelines/compile_ms 197, pipelines/front_ms 189, render/thread/ms 56.85, render/busy/ms 55.55, main/busy/ms 45.15, pipelines/wait_ms 36.28, stage/main/post_update 20.63, stage/render/prepare_flush 16.67, stage/main/update 16.47, pipelines/lock_ms 11.79, stage/render/render 11.69, stage/render/extract_commands 10.15 |
| warm-up | 5.63 | 26.6 | render/thread/ms 41.27, render/busy/ms 40.18, main/busy/ms 34.81, stage/main/post_update 16.62, stage/main/update 12.28, stage/render/render 10.90, render/graph/ms 9.30, stage/render/prepare_resources 8.96, stage/render/prepare_assets 8.15, encode//sum 7.67, bigspace/total/ms 7.37, stage/render/graph_submit 6.35 |
| warm-up | 5.75 | 30.9 | pipelines/compile_ms 565, pipelines/front_ms 550, render/thread/ms 38.85, render/busy/ms 37.88, main/busy/ms 29.87, stage/render/prepare_assets 20.88, pipelines/driver_ms 15.69, stage/main/update 14.35, stage/render/render 8.07, stage/main/post_update 8.04, encode//sum 7.73, render/graph/ms 6.75 |
| warm-up | 5.85 | 25.0 | pipelines/compile_ms 260, pipelines/front_ms 251, main/busy/ms 38.12, render/thread/ms 36.88, render/busy/ms 35.77, stage/main/update 25.04, stage/render/render 10.24, pipelines/driver_ms 9.28, render/graph/ms 8.81, stage/render/prepare_assets 8.31, encode//sum 7.45, stage/main/post_update 7.29 |
| warm-up | 5.94 | 27.0 | main/busy/ms 38.45, render/thread/ms 33.92, render/busy/ms 32.55, stage/main/update 21.22, stage/render/render 14.78, render/graph/ms 12.81, stage/main/post_update 11.95, encode//sum 10.98, stage/render/graph_submit 8.14, stage/render/prepare_assets 5.46, encode/submit 4.81, stage/render/camera_driver 4.46 |
| warm-up | 5.97 | 30.5 | render/thread/ms 45.23, render/busy/ms 43.84, main/busy/ms 41.26, stage/main/update 22.67, pipelines/compile_ms 21.47, pipelines/front_ms 21.02, stage/render/prepare_assets 13.89, stage/render/render 13.42, stage/main/post_update 12.08, render/graph/ms 11.44, encode//sum 9.83, stage/render/graph_submit 7.16 |
| warm-up | 6.01 | 33.8 | render/thread/ms 47.17, render/busy/ms 46.26, main/busy/ms 40.27, pipelines/compile_ms 21.47, stage/render/prepare_assets 21.28, pipelines/front_ms 21.02, stage/main/update 19.72, stage/main/post_update 11.23, stage/render/render 8.65, stage/render/prepare_meshes 7.68, render/graph/ms 7.52, encode//sum 7.28 |
| warm-up | 6.04 | 32.9 | render/thread/ms 54.20, render/busy/ms 52.84, main/busy/ms 41.79, stage/render/render 19.65, stage/main/update 18.53, render/graph/ms 18.06, stage/render/prepare_assets 15.98, stage/render/graph_submit 13.84, stage/main/post_update 13.82, encode//sum 12.93, encode/submit 8.68, stage/render/prepare_meshes 6.18 |
| warm-up | 6.09 | 33.8 | render/thread/ms 34.88, render/busy/ms 33.88, main/busy/ms 31.75, stage/main/update 17.57, stage/render/prepare_assets 14.68, stage/render/render 9.63, encode//sum 9.02, stage/main/post_update 8.41, render/graph/ms 8.36, stage/render/graph_submit 5.88, stream/drive/ms 4.55, encode/submit 3.41 |
| warm-up | 6.23 | 44.5 | main/busy/ms 54.62, stage/main/update 39.11, render/thread/ms 34.49, render/busy/ms 33.22, stream/drive/ms 14.12, stage/render/render 11.11, stage/render/prepare_assets 10.15, stage/main/post_update 9.93, render/graph/ms 9.41, encode//sum 8.23, stage/render/graph_submit 6.56, stage/render/prepare_meshes 5.04 |
| warm-up | 6.26 | 32.5 | pipelines/compile_ms 138, pipelines/front_ms 133, main/busy/ms 45.18, render/thread/ms 42.91, render/busy/ms 41.67, stage/main/update 27.26, stage/render/prepare_assets 19.27, stage/render/render 10.66, stage/main/post_update 9.05, render/graph/ms 9.00, encode//sum 7.90, stream/drive/ms 7.88 |
| warm-up | 6.29 | 26.9 | pipelines/compile_ms 156, pipelines/front_ms 150, render/thread/ms 38.72, render/busy/ms 37.70, main/busy/ms 34.03, stage/render/prepare_assets 19.68, stage/main/update 18.18, stage/main/post_update 8.39, stage/render/render 8.27, stream/drive/ms 7.73, encode//sum 6.87, render/graph/ms 6.87 |
| warm-up | 6.35 | 25.6 | main/busy/ms 34.98, render/thread/ms 27.13, render/busy/ms 26.06, stage/main/update 20.41, stage/render/render 10.07, stage/main/post_update 8.93, render/graph/ms 8.54, encode//sum 7.47, stage/render/graph_submit 5.89, stage/render/prepare_assets 5.80, stage/render/prepare_meshes 4.92, stream/drive/ms 4.63 |
| measure | 18.28 | 30.4 | main/busy/ms 38.20, stage/main/update 24.18, render/thread/ms 21.51, render/busy/ms 20.47, stage/render/render 10.11, render/graph/ms 8.79, stage/main/post_update 8.66, encode//sum 7.28, stage/render/graph_submit 6.26, stage/render/prepare_assets 4.06, encode/submit 3.93, stage/render/camera_driver 2.38 |
| measure | 22.99 | 27.4 | main/busy/ms 29.14, render/thread/ms 28.11, render/busy/ms 26.87, stage/main/update 13.72, stage/render/render 12.32, stage/main/post_update 10.89, render/graph/ms 10.73, encode//sum 10.03, stage/render/prepare_assets 8.82, stage/render/graph_submit 7.47, encode/submit 4.19, encode/finish_wall 3.30 |
| measure | 59.19 | 32.2 | render/thread/ms 35.94, render/busy/ms 34.87, main/busy/ms 23.95, stage/render/prepare_assets 17.53, stage/main/post_update 11.20, stage/render/render 9.25, render/graph/ms 8.04, encode//sum 7.25, stage/main/update 5.75, stage/render/graph_submit 5.62, encode/submit 3.37, stage/main/fixed_main_loop 3.15 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.count=0 --resolution 2560x1440 --bench-out target/perf\shadow_review2\balmora_walk_night@2560x1440_arm=nolamps-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **85%** (min 54%) · 2618 MHz (min 2610) · memory 8101 MHz · 85.8 W · 80 °C (max 85) · VRAM 5622 MiB · throttled 100% of samples

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
- `sg.post=high (preset: level requested)`
- `sg.shadows=custom (preset: derived from its members)`
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
- `shadow.lamp.count=0 (command line: --set)`
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
