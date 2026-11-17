# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.skin.skip_unchanged=on (command line: --set); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1542 frames over 15.0 s · **median 9.77 ms (102 fps)** · mean 9.73 · p95 11.16 · p99 11.74 (1% low 85 fps) · worst 29.76 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 280 frames over 8.5 s, worst 2589.2 ms
- hitches (> 25.0 ms): **1** measured, 58 in warm-up
- timed GPU passes: 8.59 ms of a 9.73 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.77 ms, limit 11 ms
- pass worst frame 29.76 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.943 | 2.951 | 2.997 | 1.00 | 0.069 |
| dlss_super_resolution | 2.021 | 2.027 | 2.048 | 1.00 | 0.184 |
| early prepass | 0.606 | 0.607 | 0.615 | 1.00 | 0.095 |
| atmosphere_luts | 0.323 | 0.324 | 0.394 | 1.00 | 0.003 |
| bloom | 0.315 | 0.315 | 0.331 | 1.00 | 0.061 |
| main_transmissive_pass_3d | 0.311 | 0.312 | 0.333 | 1.00 | 0.008 |
| water_velocity | 0.240 | 0.240 | 0.264 | 1.00 | 0.002 |
| water_sspr | 0.208 | 0.208 | 0.210 | 1.00 | 0.002 |
| ssao | 0.190 | 0.190 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.149 | 0.149 | 0.170 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.146 | 0.152 | 1.00 | 0.001 |
| water_fft | 0.131 | 0.132 | 0.135 | 1.00 | 0.003 |
| uniform_allocation | 0.119 | 0.120 | 0.162 | 1.00 | 0.022 |
| upscaling | 0.114 | 0.115 | 0.121 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.091 | 0.091 | 0.123 | 1.00 | 0.016 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.414 | 0.415 | 0.13 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.028 | 0.222 | 0.231 | 0.13 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 1.00 | 0.081 |
| water_snapshot | 0.025 | 0.025 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.046 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 1.00 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.13 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.13 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.806 | 10.244 | 28.21 | 13579.0 |
| main/busy/ms | 7.178 | 8.082 | 10.23 | 11069.2 |
| render/busy/ms | 6.957 | 8.071 | 27.67 | 10727.6 |
| stage/render/render | 4.666 | 5.524 | 8.64 | 7195.6 |
| stage/main/post_update | 4.254 | 5.003 | 6.44 | 6560.0 |
| render/graph/ms | 4.092 | 4.895 | 6.18 | 6309.6 |
| encode//sum | 3.804 | 5.075 | 9.03 | 5866.1 |
| stage/render/graph_submit | 3.099 | 3.857 | 5.28 | 4778.5 |
| encode/submit | 1.931 | 2.369 | 4.77 | 2977.2 |
| stage/main/update | 1.804 | 2.235 | 3.08 | 2781.5 |
| stage/render/prepare_views | 1.537 | 3.186 | 4.37 | 2370.1 |
| render/acquire/ms | 1.492 | 3.137 | 4.32 | 2300.8 |
| encode/finish_wall | 1.191 | 1.650 | 2.48 | 1836.1 |
| stage/render/camera_driver | 0.937 | 1.077 | 1.57 | 1445.0 |
| encode/pass:shadow_directional_light/sum | 0.773 | 1.320 | 2.21 | 1192.5 |
| encode/pass:early prepass/sum | 0.734 | 1.009 | 1.51 | 1131.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.720 | 1.006 | 1.43 | 1110.9 |
| stage/render/prepare_write_phase_buffers | 0.549 | 0.760 | 1.03 | 845.9 |
| stage/render/prepare_assets | 0.454 | 0.587 | 1.59 | 699.5 |
| stage/render/prepare_resources | 0.372 | 0.435 | 0.69 | 573.6 |
| bigspace/total/ms | 0.365 | 0.784 | 2.29 | 563.5 |
| render/present/ms | 0.357 | 0.429 | 2.42 | 550.6 |
| stage/main/pre_update | 0.343 | 0.441 | 1.05 | 529.6 |
| stage/main/first | 0.297 | 0.359 | 1.63 | 458.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.235 | 0.335 | 0.55 | 362.0 |
| terrain/update/ms | 0.233 | 0.297 | 0.66 | 359.0 |
| stage/render/prepare_bind_groups | 0.208 | 0.285 | 0.50 | 320.2 |
| terrain/select/ms | 0.203 | 0.259 | 0.63 | 313.3 |
| stage/render/prepare_meshes | 0.194 | 0.242 | 3.23 | 299.2 |
| stream/drive/ms | 0.186 | 0.241 | 0.60 | 287.5 |
| stage/main/fixed_main_loop | 0.183 | 0.336 | 0.87 | 282.6 |
| stage/render/prepare_batch_phases | 0.176 | 0.234 | 0.35 | 271.0 |
| stage/render/extract_commands | 0.170 | 0.205 | 0.47 | 262.6 |
| stage/render/create_views | 0.166 | 0.213 | 0.65 | 256.7 |
| far/select/ms | 0.163 | 0.218 | 0.73 | 251.4 |
| stream/tick/ms | 0.159 | 0.206 | 0.56 | 244.4 |
| controller/feed/ms | 0.155 | 0.201 | 0.56 | 239.3 |
| stage/render/prepare_flush | 0.149 | 0.268 | 0.64 | 230.2 |
| controller/move/ms | 0.142 | 0.306 | 0.91 | 219.6 |
| bigspace/high_precision/ms | 0.129 | 0.344 | 2.11 | 199.7 |
| stage/main/last | 0.118 | 0.154 | 0.64 | 182.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.116 | 0.178 | 0.42 | 179.3 |
| stage/render/queue | 0.107 | 0.133 | 5.89 | 164.3 |
| anim/evaluate/ms | 0.088 | 0.207 | 1.40 | 135.8 |
| bigspace/low_precision/ms | 0.088 | 0.187 | 1.24 | 135.7 |
| stage/render/specialize | 0.085 | 0.107 | 8.99 | 130.6 |
| anim/evaluate_cpu/ms | 0.060 | 0.076 | 0.47 | 92.7 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.114 | 0.18 | 89.6 |
| encode/resolve_diagnostics/sum | 0.055 | 0.082 | 0.20 | 84.5 |
| physics/step/ms | 0.028 | 0.055 | 0.25 | 42.8 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.039 | 0.09 | 39.7 |
| weather/map/ms | 0.024 | 0.032 | 0.17 | 36.5 |
| encode/camera_driver_span_end/sum | 0.023 | 0.037 | 0.12 | 35.9 |
| stage/render/cleanup | 0.023 | 0.033 | 0.23 | 35.7 |
| stage/render/phase_sort | 0.017 | 0.033 | 0.07 | 26.9 |
| weather/eval/ms | 0.011 | 0.015 | 0.43 | 17.1 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 10.5 |
| controller/avoid/ms | 0.006 | 0.007 | 0.37 | 9.0 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 8.2 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 6.9 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.0 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 3.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.09 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
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
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 102 | 148 | 80 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.53 | 2.41 | 0.99 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.09 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.12 | 0.02 |
| encode/finish_critical | ms | 0.86 | 2.41 | 1.40 |
| encode/pass:early prepass/max | ms | 0.73 | 1.51 | 0.77 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.72 | 1.43 | 0.96 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.42 | 0.15 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.55 | 0.29 |
| encode/pass:shadow_directional_light/max | ms | 0.71 | 2.21 | 1.40 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.20 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.23 | 27.67 | 9.25 |
| gpu/timed/ms | ms | 9.23 | 10.52 | 9.25 |
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
| phases/shadow/batch_sets | count | 1 | 30 | 23 |
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
| preprocess/bind_groups | count | 0 | 62 | 46 |
| preprocess/bind_groups_created | count | 0 | 62 | 46 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 127 | 127 | 127 |
| process/cpu_usage | % | 25.94 | 41.10 | 35.94 |
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
| skin/blocks_changed/count | count | 5 | 6 | 3 |
| skin/blocks_skipped/count | count | 176 | 179 | 178 |
| skin/blocks_walked/count | count | 5 | 6 | 3 |
| skin/joint_bytes_written/bytes | bytes | 7168 | 10.2 k | 5632 |
| skin/joints_written/count | count | 112 | 160 | 88 |
| skin/upload_writes/count | count | 3 | 5 | 3 |
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
| system/cpu_usage | % | 31.59 | 45.05 | 34.98 |
| system/mem_usage | % | 58.46 | 58.74 | 58.74 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 53 | 82 | 75 |
| views/all/bins | count | 5324 | 8061 | 5324 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 15.3 k | 27.0 k | 15.3 k |
| views/all/pipelines | count | 71 | 78 | 71 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 9496 | 27.2 k | 15.5 k |
| views/cascade/batch_sets | count | 1 | 30 | 23 |
| views/cascade/bins | count | 2152 | 4889 | 2152 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 6053 | 17.7 k | 6053 |
| views/cascade/pipelines | count | 23 | 30 | 23 |
| views/cascade/work_items | count | 34 | 17.7 k | 6053 |
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
| anim/events/count | count | 0.10 | 2 | 158 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.21 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.56 | 29 | 41.0 k |
| encode/bind_walk/init_textures | count | 7167 | 14.0 k | 11.05 M |
| encode/bind_walk/init_textures_skipped | count | 39.0 k | 50.6 k | 60.15 M |
| encode/bind_walk/inits | count | 345 | 437 | 532.6 k |
| encode/bind_walk/inits_skipped | count | 70.50 | 81 | 108.7 k |
| encode/bind_walk/merge_textures | count | 7103 | 13.9 k | 10.95 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.29 M |
| encode/bind_walk/merges | count | 82.27 | 103 | 126.9 k |
| encode/bind_walk/merges_skipped | count | 52.50 | 57 | 81.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1542 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1542 |
| encode/pass:early prepass/count | count | 1 | 1 | 1542 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1542 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1542 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1542 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1542 |
| encode/queue/command_buffers | count | 36.56 | 39 | 56.4 k |
| encode/queue/submits | count | 2 | 2 | 3084 |
| encode/queue/write_buffer/bytes | bytes | 1.14 M | 3.34 M | 1.75 G |
| encode/queue/write_buffer/calls | count | 220 | 289 | 338.8 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.33 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.9 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1542 |
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
| warm-up | 2.59 | 2589.2 | main/busy/ms 2559, stage/main/post_update 2453, stage/main/update 97.61, surface/advance_cpu_ms 68.43, stage/main/pre_update 4.88, stage/main/last 1.11, surface/cell_upload_ms 0.76, stage/main/first 0.54, stream/drive/ms 0.40, stage/main/fixed_main_loop 0.38, weather/map/ms 0.10, bigspace/total/ms 0.10 |
| warm-up | 3.34 | 746.5 | main/busy/ms 2707, stage/main/post_update 2478, render/thread/ms 715, render/busy/ms 547, stage/render/prepare_assets 500, stage/main/update 214, stage/render/prepare_views 161, render/acquire/ms 161, surface/advance_cpu_ms 68.43, stage/render/create_views 27.60, stage/render/render 15.81, render/graph/ms 7.67 |
| warm-up | 3.46 | 124.3 | render/thread/ms 821, render/busy/ms 589, stage/render/prepare_assets 528, main/busy/ms 170, stage/render/prepare_views 162, render/acquire/ms 161, stage/main/update 121, stage/render/render 84.42, render/present/ms 70.40, pipelines/compile_ms 48.74, pipelines/front_ms 45.68, stage/main/post_update 37.97 |
| warm-up | 3.51 | 51.0 | pipelines/compile_ms 485, pipelines/front_ms 402, pipelines/wait_ms 223, render/thread/ms 155, stage/render/render 111, render/present/ms 84.06, pipelines/driver_ms 82.49, main/busy/ms 72.59, render/busy/ms 70.26, stage/main/post_update 54.35, stage/render/prepare_assets 28.64, render/graph/ms 26.14 |
| warm-up | 3.63 | 118.9 | pipelines/wait_ms 4258, pipelines/compile_ms 1574, pipelines/front_ms 1419, render/thread/ms 163, pipelines/driver_ms 155, render/busy/ms 142, main/busy/ms 84.46, stage/main/post_update 51.58, stage/render/render 49.91, stage/render/prepare_write_phase_buffers 44.13, render/graph/ms 27.53, stage/main/update 26.22 |
| warm-up | 3.70 | 65.8 | pipelines/wait_ms 4923, pipelines/compile_ms 1770, pipelines/front_ms 1651, render/thread/ms 178, render/busy/ms 176, main/busy/ms 121, pipelines/driver_ms 119, stage/main/update 83.67, stage/render/prepare_write_phase_buffers 44.94, stage/render/prepare_assets 37.28, stage/render/prepare_resources 32.26, stage/main/post_update 20.54 |
| warm-up | 3.79 | 90.3 | pipelines/compile_ms 1587, pipelines/front_ms 1518, pipelines/wait_ms 1035, render/thread/ms 124, render/busy/ms 122, main/busy/ms 107, stage/main/update 70.16, pipelines/driver_ms 69.32, stage/render/prepare_assets 65.50, stage/main/post_update 18.54, stage/render/prepare_batch_phases 17.71, stage/render/render 14.61 |
| warm-up | 3.86 | 78.8 | pipelines/compile_ms 2624, pipelines/front_ms 2303, pipelines/wait_ms 327, pipelines/driver_ms 321, render/thread/ms 139, render/busy/ms 135, stage/render/prepare_assets 100, main/busy/ms 37.06, stage/render/render 29.47, render/graph/ms 26.10, stage/render/graph_submit 23.67, stage/main/post_update 15.74 |
| warm-up | 3.96 | 38.6 | pipelines/compile_ms 208, pipelines/front_ms 194, main/busy/ms 65.74, render/thread/ms 57.04, render/busy/ms 55.06, stage/main/update 47.56, stream/drive/ms 44.02, stage/render/prepare_assets 31.20, pipelines/driver_ms 14.15, stage/render/render 12.91, render/graph/ms 10.45, terrain/update/ms 8.66 |
| warm-up | 4.02 | 58.4 | pipelines/compile_ms 1114, pipelines/front_ms 1074, pipelines/wait_ms 146, render/thread/ms 73.10, render/busy/ms 71.22, main/busy/ms 58.87, stage/render/prepare_assets 54.43, stage/main/update 48.47, stream/drive/ms 44.37, pipelines/driver_ms 40.06, stage/render/render 12.00, render/graph/ms 9.58 |
| warm-up | 4.05 | 26.6 | pipelines/compile_ms 976, pipelines/front_ms 946, pipelines/wait_ms 147, render/thread/ms 58.20, render/busy/ms 56.24, main/busy/ms 39.81, stage/render/prepare_assets 30.14, pipelines/driver_ms 29.34, stage/main/update 27.72, stage/render/render 22.21, render/graph/ms 19.91, stream/drive/ms 18.29 |
| warm-up | 4.09 | 45.4 | pipelines/compile_ms 389, pipelines/front_ms 375, render/thread/ms 51.95, render/busy/ms 50.00, main/busy/ms 42.52, stage/main/update 28.87, stage/render/render 24.28, render/graph/ms 21.88, stage/render/prepare_assets 21.07, stream/drive/ms 19.14, stage/render/graph_submit 16.28, pipelines/driver_ms 13.51 |
| warm-up | 4.13 | 36.9 | pipelines/compile_ms 575, pipelines/front_ms 555, render/thread/ms 59.99, render/busy/ms 57.98, main/busy/ms 44.61, stage/render/prepare_assets 38.79, stage/main/update 27.49, stream/drive/ms 22.19, pipelines/driver_ms 19.61, stage/render/render 13.54, render/graph/ms 11.02, stage/main/post_update 9.08 |
| warm-up | 4.22 | 90.9 | pipelines/compile_ms 437, pipelines/front_ms 423, render/thread/ms 83.52, render/busy/ms 81.60, stage/render/prepare_assets 62.13, main/busy/ms 57.97, stage/main/update 34.84, stream/drive/ms 22.88, pipelines/driver_ms 13.94, stage/render/render 13.73, stage/main/post_update 12.40, render/graph/ms 11.41 |
| warm-up | 4.29 | 63.6 | pipelines/compile_ms 950, pipelines/front_ms 924, main/busy/ms 120, stage/main/update 98.52, render/thread/ms 95.87, render/busy/ms 94.36, stage/render/prepare_assets 71.17, stream/drive/ms 56.83, pipelines/driver_ms 25.92, stage/render/render 13.84, stage/main/post_update 13.29, render/graph/ms 11.74 |
| warm-up | 4.46 | 174.4 | pipelines/compile_ms 1769, pipelines/front_ms 1719, render/thread/ms 125, render/busy/ms 123, main/busy/ms 113, stage/main/update 94.60, stage/render/prepare_assets 68.47, stream/drive/ms 56.99, pipelines/driver_ms 49.99, stage/render/render 44.35, render/graph/ms 41.96, stage/render/graph_submit 31.63 |
| warm-up | 4.52 | 55.1 | pipelines/compile_ms 1403, pipelines/front_ms 1361, render/thread/ms 126, render/busy/ms 124, stage/render/prepare_assets 73.97, main/busy/ms 45.05, pipelines/driver_ms 42.07, stage/render/render 41.59, render/graph/ms 39.42, stage/render/graph_submit 29.07, encode//sum 28.86, encode/finish_wall 24.31 |
| warm-up | 4.55 | 35.5 | pipelines/compile_ms 454, pipelines/front_ms 438, render/thread/ms 73.86, render/busy/ms 72.36, main/busy/ms 58.31, stage/render/prepare_assets 38.31, stage/main/update 33.49, pipelines/wait_ms 17.51, pipelines/driver_ms 16.42, stage/main/post_update 15.17, stage/render/render 10.92, stream/drive/ms 9.53 |
| warm-up | 4.60 | 45.2 | pipelines/compile_ms 101, pipelines/front_ms 95.15, render/thread/ms 66.78, render/busy/ms 63.69, main/busy/ms 55.45, stage/main/update 37.03, stage/render/prepare_assets 26.57, pipelines/wait_ms 25.28, stage/render/render 13.48, stage/main/post_update 10.28, render/graph/ms 9.87, stage/render/prepare_batch_phases 8.22 |
| warm-up | 4.62 | 27.0 | pipelines/compile_ms 778, pipelines/front_ms 753, render/thread/ms 54.85, render/busy/ms 51.77, main/busy/ms 35.94, stage/render/prepare_assets 28.40, pipelines/wait_ms 24.58, pipelines/driver_ms 24.34, stage/main/update 20.27, stage/render/render 11.75, render/graph/ms 7.94, stage/main/post_update 7.92 |
| warm-up | 4.66 | 40.7 | pipelines/compile_ms 1581, pipelines/front_ms 1534, pipelines/wait_ms 77.58, render/thread/ms 58.36, render/busy/ms 56.77, pipelines/driver_ms 47.35, main/busy/ms 35.60, stage/render/render 19.92, render/graph/ms 17.75, stage/main/update 16.32, stage/render/graph_submit 15.03, stage/render/prepare_bind_groups 12.77 |
| warm-up | 4.69 | 28.6 | pipelines/compile_ms 841, pipelines/front_ms 816, pipelines/wait_ms 62.05, render/thread/ms 60.72, render/busy/ms 59.07, main/busy/ms 36.19, pipelines/driver_ms 24.77, stage/render/render 22.43, render/graph/ms 20.11, stage/render/graph_submit 16.96, stage/main/update 15.59, stage/render/prepare_bind_groups 12.68 |
| warm-up | 4.73 | 33.1 | pipelines/compile_ms 610, pipelines/front_ms 589, main/busy/ms 79.79, stage/main/update 62.49, stream/drive/ms 56.62, render/thread/ms 46.08, render/busy/ms 44.64, pipelines/driver_ms 21.28, stage/render/prepare_assets 18.97, stage/render/render 13.03, render/graph/ms 10.72, pipelines/wait_ms 10.08 |
| warm-up | 4.79 | 63.1 | pipelines/compile_ms 1004, pipelines/front_ms 973, main/busy/ms 126, stage/main/update 106, stream/drive/ms 99.10, render/thread/ms 39.81, render/busy/ms 38.48, pipelines/driver_ms 31.49, pipelines/wait_ms 30.79, stage/render/prepare_assets 16.06, stage/main/post_update 12.23, stage/render/render 10.48 |
| warm-up | 4.88 | 87.5 | pipelines/compile_ms 1729, pipelines/front_ms 1670, main/busy/ms 83.05, stage/main/update 62.38, pipelines/driver_ms 59.28, render/thread/ms 50.90, render/busy/ms 49.47, stream/drive/ms 46.67, pipelines/wait_ms 22.54, stage/render/prepare_assets 20.66, stage/main/post_update 13.50, stage/render/prepare_resources 11.66 |
| warm-up | 4.92 | 40.8 | pipelines/compile_ms 1738, pipelines/front_ms 1680, render/thread/ms 69.19, render/busy/ms 67.79, pipelines/driver_ms 58.56, main/busy/ms 53.21, stage/render/prepare_assets 41.12, stage/main/update 31.57, stage/render/prepare_resources 11.62, stage/main/post_update 11.47, stage/render/render 8.04, encode//sum 6.49 |
| warm-up | 4.96 | 39.9 | pipelines/compile_ms 422, pipelines/front_ms 412, main/busy/ms 68.15, render/thread/ms 63.87, render/busy/ms 62.40, stage/main/update 48.15, stage/render/prepare_assets 41.05, stream/drive/ms 12.64, stage/render/render 12.50, pipelines/driver_ms 10.46, stage/main/post_update 10.35, render/graph/ms 10.32 |
| warm-up | 5.00 | 41.0 | pipelines/compile_ms 1197, pipelines/front_ms 1172, render/thread/ms 54.36, render/busy/ms 52.93, main/busy/ms 52.83, stage/main/update 37.97, pipelines/driver_ms 25.34, stage/render/prepare_assets 24.62, pipelines/wait_ms 22.40, stream/drive/ms 12.85, stage/render/render 12.69, render/graph/ms 10.60 |
| warm-up | 5.06 | 25.2 | pipelines/compile_ms 112, pipelines/front_ms 106, main/busy/ms 43.26, render/thread/ms 38.03, render/busy/ms 36.45, stage/main/update 29.91, stream/drive/ms 16.02, stage/render/render 12.79, render/graph/ms 10.40, stage/render/prepare_assets 8.27, stage/main/post_update 8.07, encode//sum 7.96 |
| warm-up | 5.13 | 26.6 | pipelines/compile_ms 185, pipelines/front_ms 174, main/busy/ms 41.63, render/thread/ms 41.02, render/busy/ms 39.28, stage/main/update 19.29, stage/render/render 16.86, stage/main/post_update 15.61, render/graph/ms 14.18, pipelines/driver_ms 11.68, stage/render/graph_submit 9.86, encode//sum 8.71 |
| warm-up | 5.16 | 30.8 | pipelines/compile_ms 340, pipelines/front_ms 326, render/thread/ms 44.33, render/busy/ms 42.60, main/busy/ms 41.88, stage/render/prepare_assets 18.53, stage/main/update 17.32, stage/main/post_update 16.62, pipelines/driver_ms 14.08, stage/render/render 12.57, render/graph/ms 9.98, encode//sum 7.60 |
| warm-up | 5.21 | 49.4 | pipelines/compile_ms 612, pipelines/front_ms 588, render/thread/ms 62.45, render/busy/ms 60.55, main/busy/ms 38.89, stage/render/prepare_assets 31.08, pipelines/driver_ms 24.47, stage/main/post_update 17.28, stage/main/update 14.10, stage/render/render 13.63, render/graph/ms 10.71, encode//sum 9.39 |
| warm-up | 5.24 | 28.8 | pipelines/compile_ms 847, pipelines/front_ms 820, render/thread/ms 63.65, render/busy/ms 61.79, main/busy/ms 38.52, pipelines/driver_ms 27.01, stage/render/prepare_assets 24.12, stage/main/post_update 18.80, stage/render/render 17.77, render/graph/ms 15.12, stage/main/update 12.33, encode//sum 9.27 |
| warm-up | 5.29 | 46.5 | pipelines/compile_ms 684, pipelines/front_ms 668, render/thread/ms 55.47, render/busy/ms 54.01, main/busy/ms 37.15, stage/render/prepare_assets 26.15, stage/main/post_update 17.34, pipelines/driver_ms 16.39, stage/render/render 14.19, render/graph/ms 12.31, stage/main/update 11.81, encode//sum 6.92 |
| warm-up | 5.33 | 42.4 | pipelines/compile_ms 809, pipelines/front_ms 787, render/thread/ms 68.67, render/busy/ms 67.20, main/busy/ms 40.78, stage/render/prepare_assets 32.28, stage/main/update 22.07, pipelines/driver_ms 21.48, stream/drive/ms 12.15, stage/render/render 11.97, stage/main/post_update 11.48, render/graph/ms 9.90 |
| warm-up | 5.36 | 28.2 | pipelines/compile_ms 763, pipelines/front_ms 741, render/thread/ms 63.09, render/busy/ms 61.62, main/busy/ms 38.26, stage/render/prepare_assets 23.13, pipelines/driver_ms 22.11, stage/main/update 21.47, stage/render/render 14.02, stream/drive/ms 12.30, render/graph/ms 11.92, stage/main/post_update 10.83 |
| warm-up | 5.40 | 34.9 | pipelines/compile_ms 488, pipelines/front_ms 473, render/thread/ms 57.38, render/busy/ms 55.98, main/busy/ms 42.26, stage/main/update 18.48, stage/main/post_update 16.30, stage/render/prepare_assets 15.98, pipelines/driver_ms 14.74, stage/render/render 12.83, render/graph/ms 11.05, stage/render/graph_submit 7.97 |
| warm-up | 5.43 | 35.1 | pipelines/compile_ms 680, pipelines/front_ms 661, render/thread/ms 66.44, render/busy/ms 65.15, main/busy/ms 42.71, stage/render/prepare_assets 22.61, pipelines/driver_ms 18.51, stage/main/update 18.00, stage/main/post_update 17.13, stage/render/render 11.23, stage/render/prepare_resources 10.74, render/graph/ms 9.50 |
| warm-up | 5.55 | 34.0 | pipelines/compile_ms 618, pipelines/front_ms 606, pipelines/wait_ms 65.50, render/thread/ms 46.50, render/busy/ms 44.72, main/busy/ms 36.39, stage/render/prepare_assets 22.55, stage/main/update 16.95, stage/render/render 14.80, stage/main/post_update 12.40, render/graph/ms 12.40, pipelines/driver_ms 12.32 |
| warm-up | 5.58 | 31.5 | pipelines/compile_ms 696, pipelines/front_ms 678, pipelines/wait_ms 82.35, render/thread/ms 57.18, render/busy/ms 55.82, stage/render/prepare_assets 34.27, main/busy/ms 32.92, pipelines/driver_ms 18.01, stage/main/update 13.25, stage/main/post_update 12.73, stage/render/render 10.75, render/graph/ms 8.83 |
| warm-up | 5.60 | 26.2 | pipelines/compile_ms 288, pipelines/front_ms 278, render/thread/ms 50.53, render/busy/ms 48.99, main/busy/ms 30.51, stage/render/prepare_assets 21.96, pipelines/wait_ms 16.87, stage/render/render 13.64, stage/main/post_update 12.77, render/graph/ms 11.60, stage/main/update 10.52, pipelines/driver_ms 10.07 |
| warm-up | 5.64 | 34.4 | pipelines/compile_ms 180, pipelines/front_ms 172, render/thread/ms 53.95, render/busy/ms 50.87, main/busy/ms 37.22, stage/render/prepare_assets 21.03, stage/render/render 16.19, stage/main/post_update 14.07, stage/main/update 12.97, render/graph/ms 12.51, stage/render/graph_submit 8.85, encode//sum 8.42 |
| warm-up | 5.68 | 40.0 | pipelines/compile_ms 180, pipelines/front_ms 172, render/thread/ms 58.94, render/busy/ms 56.32, main/busy/ms 38.88, stage/render/prepare_assets 31.67, stage/main/update 15.33, stage/main/post_update 13.31, stage/render/render 11.07, render/graph/ms 7.95, pipelines/driver_ms 7.73, encode//sum 7.29 |
| warm-up | 5.71 | 32.5 | render/thread/ms 57.31, render/busy/ms 56.12, stage/render/prepare_assets 35.01, main/busy/ms 32.59, stage/main/update 14.17, stage/main/post_update 12.02, stage/render/render 9.94, render/graph/ms 8.21, encode//sum 7.86, stage/render/graph_submit 5.90, stream/drive/ms 4.33, encode/submit 3.43 |
| warm-up | 5.76 | 27.8 | pipelines/compile_ms 718, pipelines/front_ms 702, render/thread/ms 44.46, render/busy/ms 43.36, main/busy/ms 30.17, stage/render/prepare_assets 28.32, pipelines/driver_ms 16.02, stage/main/update 12.93, stage/main/post_update 11.30, stage/render/render 8.63, render/graph/ms 7.16, encode//sum 6.85 |
| warm-up | 5.81 | 45.9 | pipelines/compile_ms 324, pipelines/front_ms 309, render/thread/ms 65.90, render/busy/ms 64.48, main/busy/ms 54.97, stage/render/prepare_assets 38.85, stage/main/update 21.67, pipelines/driver_ms 15.22, stage/main/post_update 14.40, stage/main/pre_update 13.76, stage/render/render 12.72, render/graph/ms 10.32 |
| warm-up | 5.84 | 32.6 | pipelines/compile_ms 324, pipelines/front_ms 309, render/thread/ms 67.53, render/busy/ms 66.15, main/busy/ms 56.43, stage/render/prepare_assets 32.69, stage/main/update 19.92, stage/main/post_update 17.45, stage/render/render 16.06, pipelines/driver_ms 15.22, render/graph/ms 13.58, stage/main/pre_update 13.54 |
| warm-up | 5.87 | 31.8 | render/thread/ms 51.52, render/busy/ms 49.84, main/busy/ms 33.87, stage/render/render 18.29, stage/render/prepare_assets 18.08, stage/main/post_update 17.15, render/graph/ms 15.94, stage/render/graph_submit 11.42, stage/main/update 10.74, encode//sum 9.93, encode/submit 7.61, stage/render/camera_driver 4.27 |
| warm-up | 5.90 | 29.6 | main/busy/ms 47.07, render/thread/ms 46.48, render/busy/ms 44.66, stage/main/update 24.41, stage/render/prepare_assets 17.96, stage/render/render 17.10, stage/main/post_update 15.91, stream/drive/ms 15.31, render/graph/ms 14.67, stage/render/graph_submit 10.94, encode//sum 9.14, encode/submit 6.73 |
| warm-up | 5.96 | 52.1 | render/thread/ms 54.40, main/busy/ms 53.87, render/busy/ms 52.61, stage/main/update 32.28, pipelines/compile_ms 28.60, pipelines/front_ms 26.67, stage/render/prepare_assets 22.16, stage/render/render 17.65, stream/drive/ms 15.66, render/graph/ms 15.40, stage/main/post_update 14.23, stage/render/graph_submit 12.10 |
| warm-up | 6.01 | 31.3 | pipelines/compile_ms 1045, pipelines/front_ms 1014, render/thread/ms 40.86, render/busy/ms 39.42, main/busy/ms 32.72, pipelines/driver_ms 30.34, stage/render/prepare_assets 16.24, stage/main/update 13.75, stage/main/post_update 12.28, stage/render/render 12.07, render/graph/ms 10.15, stream/drive/ms 7.38 |
| warm-up | 6.16 | 38.7 | pipelines/compile_ms 1055, pipelines/front_ms 1017, main/busy/ms 54.98, render/thread/ms 54.91, render/busy/ms 53.31, pipelines/driver_ms 38.21, stage/main/update 34.68, stream/drive/ms 26.28, stage/render/render 16.11, render/graph/ms 14.20, stage/main/post_update 13.13, pipelines/wait_ms 12.18 |
| warm-up | 6.19 | 25.6 | pipelines/compile_ms 975, pipelines/front_ms 942, main/busy/ms 53.66, render/thread/ms 52.79, render/busy/ms 51.35, stage/main/update 34.62, pipelines/driver_ms 33.32, stream/drive/ms 26.47, stage/render/render 12.24, stage/main/post_update 11.96, stage/render/prepare_flush 11.33, pipelines/wait_ms 10.45 |
| warm-up | 6.27 | 36.0 | main/busy/ms 38.68, render/thread/ms 32.51, render/busy/ms 30.87, stage/main/update 19.55, stage/main/post_update 13.53, stage/render/render 12.55, stage/render/prepare_assets 12.26, render/graph/ms 10.50, stage/render/graph_submit 7.97, encode//sum 6.55, encode/submit 5.35, stream/drive/ms 3.64 |
| warm-up | 6.49 | 25.6 | main/busy/ms 43.07, render/thread/ms 38.90, render/busy/ms 36.82, pipelines/compile_ms 34.64, pipelines/front_ms 32.44, stage/main/update 20.44, stage/render/render 16.92, stage/main/post_update 15.56, render/graph/ms 13.96, stage/render/prepare_assets 12.36, stream/drive/ms 10.97, encode//sum 10.26 |
| warm-up | 6.53 | 43.4 | render/thread/ms 46.99, render/busy/ms 45.52, main/busy/ms 42.95, pipelines/compile_ms 34.64, pipelines/front_ms 32.44, stage/render/prepare_assets 28.88, stage/main/update 21.78, stage/main/post_update 14.55, stream/drive/ms 11.12, stage/render/render 10.60, render/graph/ms 8.48, encode//sum 8.17 |
| warm-up | 6.91 | 31.8 | render/thread/ms 38.83, render/busy/ms 37.17, pipelines/compile_ms 20.22, pipelines/front_ms 19.41, main/busy/ms 15.86, stage/render/render 11.40, render/graph/ms 10.05, stage/render/specialize 8.91, stage/main/post_update 8.37, stage/render/graph_submit 8.12, encode//sum 7.59, stage/render/queue 6.48 |
| warm-up | 8.11 | 27.9 | render/thread/ms 34.52, render/busy/ms 31.17, main/busy/ms 15.45, stage/render/render 9.53, stage/main/post_update 8.81, render/graph/ms 8.31, stage/render/specialize 8.24, stage/render/graph_submit 6.48, encode//sum 6.07, stage/render/queue 5.00, stage/render/prepare_meshes 3.84, stage/main/update 3.78 |
| measure | 0.82 | 29.8 | render/thread/ms 37.46, render/busy/ms 34.56, main/busy/ms 14.39, stage/render/render 11.91, render/graph/ms 10.61, encode//sum 9.14, stage/render/specialize 9.06, stage/render/graph_submit 8.55, stage/main/post_update 8.47, stage/render/queue 5.97, encode/submit 4.84, encode/finish_wall 3.75 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set render.skin.skip_unchanged=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=skin-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 5%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 85%) · 2625 MHz (min 2625) · memory 8101 MHz · 97.7 W · 78 °C (max 81) · VRAM 6434 MiB · throttled 100% of samples

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
- `render.skin.skip_unchanged=on (command line: --set)`
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
