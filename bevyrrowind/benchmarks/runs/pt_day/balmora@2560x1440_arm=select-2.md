# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); select.gate=on (command line: --set); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1540 frames over 15.0 s · **median 9.75 ms (103 fps)** · mean 9.74 · p95 11.13 · p99 11.60 (1% low 86 fps) · worst 27.96 ms · 3 spikes over 1.5× median · 12.0 stutters/min (work 8.0/min)
- warm-up: 279 frames over 8.7 s, worst 2636.4 ms
- hitches (> 25.0 ms): **1** measured, 64 in warm-up
- timed GPU passes: 8.58 ms of a 9.74 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.75 ms, limit 11 ms
- pass worst frame 27.96 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.932 | 2.945 | 2.989 | 1.00 | 0.071 |
| dlss_super_resolution | 2.012 | 2.021 | 2.044 | 1.00 | 0.184 |
| early prepass | 0.614 | 0.617 | 0.627 | 1.00 | 0.094 |
| atmosphere_luts | 0.334 | 0.335 | 0.394 | 1.00 | 0.003 |
| bloom | 0.313 | 0.315 | 0.331 | 1.00 | 0.061 |
| main_transmissive_pass_3d | 0.311 | 0.313 | 0.335 | 1.00 | 0.009 |
| water_velocity | 0.239 | 0.240 | 0.264 | 1.00 | 0.002 |
| water_sspr | 0.208 | 0.209 | 0.210 | 1.00 | 0.002 |
| ssao | 0.189 | 0.190 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.148 | 0.149 | 0.170 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.146 | 0.152 | 1.00 | 0.001 |
| water_fft | 0.130 | 0.131 | 0.134 | 1.00 | 0.003 |
| uniform_allocation | 0.119 | 0.120 | 0.162 | 1.00 | 0.023 |
| upscaling | 0.112 | 0.113 | 0.118 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.090 | 0.091 | 0.121 | 1.00 | 0.017 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.063 | 0.063 | 0.074 | 1.00 | 0.021 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.414 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.229 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.025 | 0.028 | 1.00 | 0.084 |
| water_snapshot | 0.025 | 0.025 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.045 | 0.047 | 0.32 | 0.003 |
| surface_exposure | 0.012 | 0.043 | 0.043 | 0.28 | 0.001 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 1.00 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.007 | 0.007 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.002 | 0.003 | 1.00 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.803 | 10.178 | 26.81 | 13556.3 |
| render/busy/ms | 6.994 | 8.048 | 26.15 | 10770.3 |
| main/busy/ms | 6.761 | 7.666 | 9.24 | 10412.5 |
| stage/render/render | 4.667 | 5.522 | 14.76 | 7187.5 |
| render/graph/ms | 4.077 | 4.866 | 14.12 | 6277.9 |
| stage/main/post_update | 4.071 | 4.867 | 6.33 | 6269.9 |
| encode//sum | 3.980 | 5.237 | 13.69 | 6128.6 |
| stage/render/graph_submit | 3.067 | 3.827 | 13.05 | 4723.2 |
| encode/submit | 1.901 | 2.296 | 5.42 | 2927.9 |
| stage/main/update | 1.568 | 2.004 | 3.16 | 2414.2 |
| stage/render/prepare_views | 1.482 | 3.056 | 3.90 | 2282.5 |
| render/acquire/ms | 1.439 | 3.019 | 3.86 | 2216.5 |
| encode/finish_wall | 1.191 | 1.666 | 9.91 | 1834.4 |
| stage/render/camera_driver | 0.953 | 1.096 | 2.34 | 1467.0 |
| encode/pass:shadow_directional_light/sum | 0.761 | 1.274 | 2.09 | 1171.6 |
| encode/pass:early prepass/sum | 0.734 | 1.001 | 9.48 | 1130.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.713 | 0.995 | 1.55 | 1098.6 |
| stage/render/prepare_write_phase_buffers | 0.554 | 0.780 | 1.03 | 853.4 |
| stage/render/prepare_assets | 0.455 | 0.589 | 1.63 | 701.0 |
| stage/render/prepare_resources | 0.397 | 0.468 | 0.89 | 611.0 |
| render/present/ms | 0.370 | 0.446 | 2.58 | 569.5 |
| bigspace/total/ms | 0.348 | 0.687 | 1.51 | 536.4 |
| stage/main/pre_update | 0.347 | 0.447 | 0.86 | 533.7 |
| stage/main/first | 0.296 | 0.361 | 1.49 | 456.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.246 | 0.362 | 0.56 | 379.2 |
| stage/render/prepare_bind_groups | 0.211 | 0.294 | 0.62 | 324.9 |
| stage/render/prepare_meshes | 0.192 | 0.243 | 2.39 | 296.3 |
| stage/main/fixed_main_loop | 0.181 | 0.336 | 0.98 | 278.1 |
| stage/render/prepare_batch_phases | 0.177 | 0.239 | 0.36 | 272.2 |
| stage/render/extract_commands | 0.177 | 0.210 | 0.65 | 272.0 |
| stage/render/create_views | 0.170 | 0.217 | 0.74 | 261.6 |
| controller/feed/ms | 0.157 | 0.206 | 0.55 | 241.1 |
| stage/render/prepare_flush | 0.151 | 0.274 | 0.62 | 232.6 |
| stream/drive/ms | 0.143 | 0.193 | 0.52 | 220.4 |
| bigspace/high_precision/ms | 0.124 | 0.290 | 1.15 | 191.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.124 | 0.192 | 0.33 | 190.3 |
| stage/main/last | 0.119 | 0.161 | 0.77 | 183.1 |
| controller/move/ms | 0.117 | 0.165 | 0.66 | 179.6 |
| stream/tick/ms | 0.114 | 0.157 | 0.49 | 176.2 |
| stage/render/queue | 0.108 | 0.141 | 5.69 | 166.0 |
| stage/render/specialize | 0.086 | 0.108 | 8.59 | 133.0 |
| anim/evaluate/ms | 0.085 | 0.206 | 0.82 | 130.4 |
| bigspace/low_precision/ms | 0.079 | 0.179 | 1.20 | 121.6 |
| anim/evaluate_cpu/ms | 0.062 | 0.078 | 0.49 | 94.8 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.115 | 0.22 | 88.6 |
| encode/resolve_diagnostics/sum | 0.057 | 0.086 | 0.30 | 87.3 |
| terrain/update/ms | 0.030 | 0.040 | 0.13 | 46.9 |
| encode/camera_driver_span_begin/sum | 0.028 | 0.041 | 0.20 | 42.8 |
| physics/step/ms | 0.026 | 0.052 | 0.16 | 39.7 |
| encode/camera_driver_span_end/sum | 0.025 | 0.040 | 0.10 | 37.9 |
| weather/map/ms | 0.024 | 0.032 | 0.40 | 37.2 |
| stage/render/cleanup | 0.023 | 0.033 | 0.25 | 36.2 |
| stage/render/phase_sort | 0.020 | 0.035 | 0.16 | 31.5 |
| weather/eval/ms | 0.011 | 0.015 | 0.03 | 16.9 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 10.7 |
| controller/avoid/ms | 0.006 | 0.007 | 0.43 | 9.2 |
| script/input/ms | 0.005 | 0.007 | 0.07 | 8.5 |
| anim/apply/ms | 0.004 | 0.006 | 0.06 | 6.9 |
| far/select/ms | 0.004 | 0.004 | 0.39 | 5.9 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 5.8 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.15 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.06 | 1.1 |
| terrain/select/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.6 |
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
| encode//max | ms | 0.53 | 9.65 | 0.61 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.20 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.10 | 0.02 |
| encode/finish_critical | ms | 0.86 | 9.65 | 0.91 |
| encode/pass:early prepass/max | ms | 0.73 | 9.48 | 0.65 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.72 | 1.55 | 0.63 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.33 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.24 | 0.56 | 0.35 |
| encode/pass:shadow_directional_light/max | ms | 0.70 | 2.09 | 0.91 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.30 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.25 | 26.15 | 9.31 |
| gpu/timed/ms | ms | 9.25 | 10.41 | 9.31 |
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
| process/cpu_usage | % | 26.92 | 37.98 | 24.76 |
| process/mem_usage | GiB | 4.63 | 4.72 | 4.72 |
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
| skin/blocks_changed/count | count | 4 | 5 | 3 |
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
| system/cpu_usage | % | 31.36 | 41.89 | 28.78 |
| system/mem_usage | % | 59.04 | 59.25 | 59.25 |
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
| anim/events/count | count | 0.10 | 3 | 149 |
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
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 26.57 | 52 | 40.9 k |
| encode/bind_walk/init_textures | count | 7166 | 14.0 k | 11.04 M |
| encode/bind_walk/init_textures_skipped | count | 26.9 k | 47.0 k | 41.39 M |
| encode/bind_walk/inits | count | 345 | 652 | 531.9 k |
| encode/bind_walk/inits_skipped | count | 58 | 110 | 89.3 k |
| encode/bind_walk/merge_textures | count | 7102 | 13.9 k | 10.94 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 8162 | 6.28 M |
| encode/bind_walk/merges | count | 82.26 | 156 | 126.7 k |
| encode/bind_walk/merges_skipped | count | 46.25 | 90 | 71.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1540 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1540 |
| encode/pass:early prepass/count | count | 1 | 2 | 1540 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1540 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 1540 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 1540 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 1540 |
| encode/queue/command_buffers | count | 36.57 | 72 | 56.3 k |
| encode/queue/submits | count | 2 | 4 | 3080 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.65 M | 2.21 G |
| encode/queue/write_buffer/calls | count | 217 | 400 | 334.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 65.3 k | 50.27 M |
| encode/queue/write_buffer_with/calls | count | 9 | 18 | 13.9 k |
| encode/queue/write_texture/bytes | bytes | 11.7 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1540 |
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

## Hitches (> 25.0 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.64 | 2636.4 | main/busy/ms 2601, stage/main/post_update 2496, stage/main/update 95.44, surface/advance_cpu_ms 65.49, stage/main/pre_update 5.30, stage/main/last 1.29, surface/cell_upload_ms 0.75, stream/drive/ms 0.67, stage/main/first 0.49, stage/main/fixed_main_loop 0.35, stream/tick/ms 0.19, bigspace/total/ms 0.09 |
| warm-up | 3.45 | 816.6 | main/busy/ms 2778, stage/main/post_update 2521, render/thread/ms 786, render/busy/ms 614, stage/render/prepare_assets 567, stage/main/update 242, stage/render/prepare_views 166, render/acquire/ms 166, surface/advance_cpu_ms 65.49, stage/render/create_views 28.29, stage/render/render 14.99, render/graph/ms 8.04 |
| warm-up | 3.60 | 147.7 | render/thread/ms 918, render/busy/ms 676, stage/render/prepare_assets 617, main/busy/ms 210, stage/render/prepare_views 166, render/acquire/ms 166, stage/main/update 152, stage/render/render 89.87, render/present/ms 75.67, pipelines/compile_ms 45.73, stage/main/post_update 38.83, pipelines/front_ms 37.04 |
| warm-up | 3.63 | 25.4 | pipelines/compile_ms 343, pipelines/front_ms 250, render/thread/ms 156, pipelines/driver_ms 93.71, stage/render/render 92.38, render/busy/ms 83.59, main/busy/ms 75.44, render/present/ms 71.72, stage/render/prepare_assets 50.10, stage/main/post_update 33.85, pipelines/wait_ms 27.26, terrain/update/ms 26.15 |
| warm-up | 3.77 | 142.3 | pipelines/wait_ms 4713, pipelines/compile_ms 1759, pipelines/front_ms 1584, pipelines/driver_ms 174, main/busy/ms 135, render/thread/ms 114, render/busy/ms 111, stage/main/update 89.07, stage/main/post_update 39.17, stage/render/prepare_resources 37.58, stage/render/render 33.59, render/graph/ms 29.86 |
| warm-up | 3.85 | 83.2 | pipelines/wait_ms 5008, pipelines/compile_ms 2036, pipelines/front_ms 1925, render/thread/ms 164, render/busy/ms 162, main/busy/ms 118, pipelines/driver_ms 111, stage/render/prepare_assets 83.06, stage/main/update 76.97, stage/render/prepare_resources 36.99, stage/main/post_update 31.66, stage/render/render 21.76 |
| warm-up | 3.88 | 32.2 | pipelines/compile_ms 1039, pipelines/front_ms 998, pipelines/wait_ms 351, render/thread/ms 105, render/busy/ms 103, stage/render/prepare_assets 85.55, main/busy/ms 41.26, pipelines/driver_ms 40.21, stage/main/post_update 20.11, terrain/update/ms 14.91, stage/main/update 10.45, stage/render/render 9.95 |
| warm-up | 3.91 | 30.3 | pipelines/compile_ms 829, pipelines/front_ms 787, render/thread/ms 59.10, render/busy/ms 57.68, pipelines/wait_ms 53.12, pipelines/driver_ms 41.73, main/busy/ms 38.11, stage/render/prepare_assets 34.70, stage/main/post_update 17.48, stage/render/render 14.98, render/graph/ms 13.24, terrain/update/ms 9.88 |
| warm-up | 3.95 | 37.5 | pipelines/compile_ms 997, pipelines/front_ms 940, pipelines/wait_ms 88.58, render/thread/ms 65.23, render/busy/ms 63.59, pipelines/driver_ms 56.37, main/busy/ms 32.54, stage/render/render 23.87, render/graph/ms 21.87, encode//sum 16.77, stage/render/prepare_resources 15.32, stage/main/post_update 15.19 |
| warm-up | 3.98 | 26.4 | pipelines/compile_ms 860, pipelines/front_ms 819, pipelines/wait_ms 75.06, render/thread/ms 61.39, render/busy/ms 59.00, pipelines/driver_ms 40.16, main/busy/ms 35.32, stage/render/render 20.55, stage/render/prepare_resources 19.39, render/graph/ms 17.58, encode//sum 17.44, stage/main/post_update 15.48 |
| warm-up | 4.02 | 46.6 | pipelines/compile_ms 466, pipelines/front_ms 449, render/thread/ms 68.37, render/busy/ms 66.25, stage/render/prepare_assets 34.85, main/busy/ms 34.51, pipelines/driver_ms 16.89, stage/main/post_update 15.55, stage/render/render 14.02, stage/main/update 11.55, render/graph/ms 11.20, pipelines/wait_ms 11.06 |
| warm-up | 4.11 | 72.0 | pipelines/compile_ms 1352, pipelines/front_ms 1301, main/busy/ms 51.32, pipelines/driver_ms 51.24, render/thread/ms 50.11, render/busy/ms 48.56, stage/main/update 40.35, stream/drive/ms 35.96, stage/render/prepare_assets 28.53, pipelines/wait_ms 22.91, stage/render/render 14.17, render/graph/ms 12.32 |
| warm-up | 4.14 | 27.3 | pipelines/compile_ms 1322, pipelines/front_ms 1276, render/thread/ms 62.10, render/busy/ms 60.23, pipelines/driver_ms 45.95, main/busy/ms 37.86, stage/render/prepare_assets 28.28, pipelines/wait_ms 27.37, stage/main/update 25.37, stream/drive/ms 21.16, stage/render/render 16.57, render/graph/ms 13.13 |
| warm-up | 4.19 | 48.4 | pipelines/compile_ms 391, pipelines/front_ms 375, render/thread/ms 61.98, render/busy/ms 59.70, main/busy/ms 40.24, stage/main/update 26.35, stage/render/prepare_assets 25.94, stream/drive/ms 22.39, stage/render/render 17.83, pipelines/driver_ms 16.27, render/graph/ms 13.88, stage/render/prepare_bind_groups 9.77 |
| warm-up | 4.23 | 42.5 | render/thread/ms 71.26, render/busy/ms 69.08, stage/render/prepare_assets 47.11, main/busy/ms 45.11, pipelines/compile_ms 36.18, pipelines/front_ms 34.95, stage/main/update 28.09, stream/drive/ms 23.07, stage/render/render 16.04, render/graph/ms 13.21, stage/main/post_update 9.92, stage/render/graph_submit 9.80 |
| warm-up | 4.33 | 98.9 | pipelines/compile_ms 227, pipelines/front_ms 221, main/busy/ms 105, render/thread/ms 102, render/busy/ms 99.10, stage/render/prepare_assets 65.77, stage/main/update 65.77, stream/drive/ms 38.90, stage/main/post_update 27.06, stage/render/render 25.08, render/graph/ms 21.30, stage/render/graph_submit 15.25 |
| warm-up | 4.45 | 123.3 | pipelines/compile_ms 1481, pipelines/front_ms 1444, render/thread/ms 157, render/busy/ms 155, main/busy/ms 126, stage/main/update 90.38, stage/render/prepare_assets 89.84, stream/drive/ms 58.31, pipelines/driver_ms 36.91, stage/render/prepare_bind_groups 34.82, stage/main/post_update 23.34, stage/render/render 20.92 |
| warm-up | 4.52 | 69.3 | pipelines/compile_ms 1985, pipelines/front_ms 1916, render/thread/ms 137, render/busy/ms 134, stage/render/prepare_assets 82.48, main/busy/ms 69.12, pipelines/driver_ms 68.78, stage/main/update 55.57, stream/drive/ms 43.18, stage/render/prepare_bind_groups 34.54, pipelines/wait_ms 18.91, stage/render/render 10.82 |
| warm-up | 4.56 | 36.5 | pipelines/compile_ms 838, pipelines/front_ms 795, render/thread/ms 78.38, render/busy/ms 73.73, main/busy/ms 63.64, stage/main/update 46.84, pipelines/driver_ms 43.16, stage/render/prepare_assets 38.75, stage/render/render 30.72, render/graph/ms 25.65, pipelines/wait_ms 17.75, stream/drive/ms 13.71 |
| warm-up | 4.65 | 88.8 | pipelines/compile_ms 154, pipelines/front_ms 142, render/thread/ms 82.52, render/busy/ms 78.32, main/busy/ms 64.57, stage/main/update 47.69, stage/render/render 41.65, render/graph/ms 37.02, stage/render/prepare_assets 28.29, stage/render/graph_submit 22.14, encode/submit 15.44, stream/drive/ms 12.93 |
| warm-up | 4.70 | 54.7 | pipelines/compile_ms 370, pipelines/front_ms 350, pipelines/wait_ms 162, render/thread/ms 100, render/busy/ms 98.28, main/busy/ms 51.26, stage/render/prepare_assets 37.91, stage/main/update 27.80, stage/render/prepare_resources 23.15, stage/render/render 22.04, pipelines/driver_ms 20.12, render/graph/ms 19.62 |
| warm-up | 4.75 | 48.1 | pipelines/compile_ms 2038, pipelines/front_ms 1965, pipelines/wait_ms 405, render/thread/ms 93.95, render/busy/ms 92.51, pipelines/driver_ms 72.55, main/busy/ms 61.83, stage/render/prepare_assets 35.53, stage/main/update 29.92, stage/render/prepare_resources 25.77, stage/main/post_update 23.78, stage/render/render 10.19 |
| warm-up | 4.82 | 70.3 | pipelines/compile_ms 1793, pipelines/front_ms 1729, pipelines/wait_ms 384, render/thread/ms 107, render/busy/ms 106, pipelines/driver_ms 63.19, main/busy/ms 56.92, stage/render/prepare_assets 32.68, stage/main/update 31.79, stage/main/post_update 16.31, stage/render/prepare_flush 15.68, stage/render/prepare_bind_groups 11.74 |
| warm-up | 4.89 | 75.3 | pipelines/compile_ms 700, pipelines/front_ms 677, pipelines/wait_ms 206, render/thread/ms 127, render/busy/ms 125, main/busy/ms 61.75, stage/main/update 36.56, stage/render/prepare_assets 33.39, stage/render/prepare_flush 26.85, pipelines/driver_ms 23.65, stage/main/post_update 16.82, stage/render/prepare_resources 15.40 |
| warm-up | 4.94 | 42.5 | pipelines/compile_ms 1449, pipelines/front_ms 1403, pipelines/wait_ms 203, main/busy/ms 136, stage/main/update 113, render/thread/ms 97.91, render/busy/ms 96.57, stream/drive/ms 86.23, pipelines/driver_ms 45.61, stage/render/prepare_assets 37.44, stage/render/prepare_resources 17.52, stage/main/post_update 17.39 |
| warm-up | 5.04 | 101.0 | pipelines/compile_ms 2507, pipelines/front_ms 2439, pipelines/wait_ms 366, main/busy/ms 127, stage/main/update 103, stream/drive/ms 86.35, pipelines/driver_ms 67.71, render/thread/ms 57.33, render/busy/ms 55.09, stage/render/prepare_assets 20.80, stage/render/render 16.40, stage/main/post_update 16.22 |
| warm-up | 5.07 | 34.3 | pipelines/compile_ms 2157, pipelines/front_ms 2095, pipelines/wait_ms 349, main/busy/ms 65.08, pipelines/driver_ms 62.05, render/thread/ms 48.30, render/busy/ms 45.20, stage/main/update 23.33, stage/render/render 20.24, stage/main/post_update 18.72, render/graph/ms 16.67, stage/render/prepare_assets 16.04 |
| warm-up | 5.14 | 64.8 | pipelines/compile_ms 956, pipelines/front_ms 921, pipelines/wait_ms 146, main/busy/ms 74.40, render/thread/ms 57.99, render/busy/ms 55.66, stage/main/update 40.77, pipelines/driver_ms 35.13, stage/render/prepare_assets 31.62, stage/render/render 14.70, stage/main/post_update 11.84, render/graph/ms 11.84 |
| warm-up | 5.21 | 68.8 | pipelines/compile_ms 520, pipelines/front_ms 504, render/thread/ms 91.84, render/busy/ms 87.52, main/busy/ms 68.20, stage/main/update 40.40, stage/render/prepare_assets 37.14, pipelines/wait_ms 27.88, stage/render/prepare_resources 23.38, pipelines/driver_ms 16.24, stage/main/post_update 14.93, stream/drive/ms 14.48 |
| warm-up | 5.24 | 30.8 | pipelines/compile_ms 663, pipelines/front_ms 643, render/thread/ms 86.10, render/busy/ms 81.79, main/busy/ms 58.80, stage/render/prepare_resources 28.45, stage/main/update 24.37, pipelines/driver_ms 19.44, stage/render/prepare_assets 19.22, stage/main/post_update 17.53, stream/drive/ms 14.94, stage/render/render 11.06 |
| warm-up | 5.28 | 40.0 | pipelines/compile_ms 1284, pipelines/front_ms 1249, render/thread/ms 65.50, render/busy/ms 63.93, pipelines/wait_ms 57.88, main/busy/ms 38.11, pipelines/driver_ms 34.65, stage/main/update 17.03, stage/render/render 12.16, stage/render/prepare_batch_phases 12.01, stage/render/specialize 11.24, stage/main/post_update 11.03 |
| warm-up | 5.31 | 31.1 | pipelines/compile_ms 662, pipelines/front_ms 646, render/thread/ms 64.74, render/busy/ms 63.13, pipelines/wait_ms 56.37, main/busy/ms 36.56, stage/main/update 22.77, pipelines/driver_ms 16.38, stage/render/prepare_batch_phases 11.93, stage/render/render 11.90, stage/render/specialize 11.17, stage/render/prepare_resources 10.91 |
| warm-up | 5.34 | 33.3 | pipelines/compile_ms 1329, pipelines/front_ms 1294, render/thread/ms 54.65, main/busy/ms 53.39, render/busy/ms 52.33, stage/main/update 38.89, pipelines/driver_ms 34.63, stream/drive/ms 15.51, stage/render/render 13.97, stage/render/prepare_assets 12.59, render/graph/ms 10.94, stage/render/prepare_resources 9.62 |
| warm-up | 5.37 | 26.4 | pipelines/compile_ms 1329, pipelines/front_ms 1294, main/busy/ms 53.20, render/thread/ms 48.20, render/busy/ms 44.47, pipelines/driver_ms 34.63, stage/main/update 33.45, stage/render/render 18.70, stream/drive/ms 15.26, render/graph/ms 14.36, stage/main/post_update 13.47, stage/render/graph_submit 11.15 |
| warm-up | 5.40 | 34.6 | pipelines/compile_ms 253, pipelines/front_ms 226, render/thread/ms 51.95, render/busy/ms 48.73, main/busy/ms 48.55, pipelines/wait_ms 29.30, pipelines/driver_ms 26.88, stage/main/update 24.55, stage/main/post_update 16.27, stage/render/render 15.83, render/graph/ms 12.52, stage/render/queue 11.46 |
| warm-up | 5.45 | 46.1 | pipelines/compile_ms 664, pipelines/front_ms 621, render/thread/ms 70.05, render/busy/ms 68.21, pipelines/wait_ms 62.66, main/busy/ms 47.76, pipelines/driver_ms 42.56, stage/main/update 27.23, stage/render/prepare_assets 22.89, stage/main/post_update 12.21, stage/render/render 11.87, stage/render/queue 11.58 |
| warm-up | 5.52 | 46.2 | pipelines/compile_ms 1114, pipelines/front_ms 1081, render/thread/ms 54.41, render/busy/ms 52.26, main/busy/ms 39.95, pipelines/wait_ms 35.78, pipelines/driver_ms 33.14, stage/main/update 22.02, stage/render/prepare_assets 20.70, stage/render/render 19.19, render/graph/ms 16.21, stream/drive/ms 13.54 |
| warm-up | 5.55 | 28.4 | pipelines/compile_ms 636, pipelines/front_ms 612, render/thread/ms 59.47, render/busy/ms 54.81, main/busy/ms 42.33, stage/render/render 25.83, pipelines/driver_ms 23.44, stage/main/post_update 22.44, render/graph/ms 20.45, stage/render/prepare_assets 19.79, stage/render/graph_submit 15.12, stage/main/update 12.58 |
| warm-up | 5.59 | 39.6 | pipelines/compile_ms 783, pipelines/front_ms 759, render/thread/ms 52.31, render/busy/ms 48.24, pipelines/wait_ms 48.12, main/busy/ms 44.47, pipelines/driver_ms 23.58, stage/render/prepare_assets 22.91, stage/main/post_update 21.87, stage/render/render 19.44, pipelines/lock_ms 19.16, stage/main/update 15.44 |
| warm-up | 5.62 | 28.9 | pipelines/compile_ms 723, pipelines/front_ms 703, render/thread/ms 48.91, render/busy/ms 47.67, pipelines/wait_ms 46.59, main/busy/ms 37.94, stage/render/prepare_assets 28.41, stage/main/update 22.01, pipelines/driver_ms 19.86, pipelines/lock_ms 17.78, stream/drive/ms 11.82, stage/render/render 9.67 |
| warm-up | 5.68 | 25.2 | pipelines/compile_ms 210, pipelines/front_ms 204, render/thread/ms 39.73, render/busy/ms 38.48, main/busy/ms 28.61, stage/render/prepare_assets 14.83, stage/main/update 13.88, stage/render/render 10.45, render/graph/ms 8.77, stage/main/post_update 8.43, encode//sum 6.84, stage/render/graph_submit 6.08 |
| warm-up | 5.71 | 26.9 | render/thread/ms 48.20, render/busy/ms 46.75, main/busy/ms 35.94, stage/render/prepare_assets 18.67, stage/main/update 16.13, stage/render/render 13.80, stage/main/post_update 12.83, render/graph/ms 11.78, encode//sum 8.55, stage/render/graph_submit 8.11, encode/submit 5.13, stream/drive/ms 3.90 |
| warm-up | 5.74 | 29.9 | pipelines/compile_ms 767, pipelines/front_ms 739, render/thread/ms 51.89, render/busy/ms 50.50, main/busy/ms 42.99, pipelines/driver_ms 27.89, stage/render/prepare_assets 21.45, stage/main/update 18.47, stage/main/post_update 17.22, pipelines/wait_ms 15.30, pipelines/lock_ms 14.97, stage/render/render 13.84 |
| warm-up | 5.80 | 32.1 | pipelines/compile_ms 351, pipelines/front_ms 343, render/thread/ms 48.43, render/busy/ms 46.64, main/busy/ms 26.53, stage/render/prepare_assets 22.75, stage/render/render 14.64, render/graph/ms 12.40, stage/main/update 11.11, stage/render/graph_submit 9.79, stage/main/post_update 8.95, pipelines/driver_ms 8.52 |
| warm-up | 5.84 | 45.8 | pipelines/compile_ms 653, pipelines/front_ms 634, render/thread/ms 70.09, render/busy/ms 67.93, main/busy/ms 37.29, stage/render/prepare_assets 33.83, stage/render/render 20.45, pipelines/driver_ms 18.85, stage/main/update 18.53, render/graph/ms 17.46, stage/render/graph_submit 14.17, encode//sum 8.05 |
| warm-up | 5.91 | 47.6 | pipelines/compile_ms 199, pipelines/front_ms 194, render/thread/ms 57.41, render/busy/ms 55.68, main/busy/ms 38.62, stage/render/prepare_assets 27.01, stage/main/update 17.21, stage/render/render 14.18, stage/main/post_update 12.86, render/graph/ms 11.81, stage/render/graph_submit 8.69, encode//sum 8.30 |
| warm-up | 5.94 | 25.3 | pipelines/compile_ms 207, pipelines/front_ms 202, render/thread/ms 54.78, render/busy/ms 53.54, stage/render/prepare_assets 33.57, main/busy/ms 32.83, stage/main/update 14.91, stage/main/post_update 11.51, stage/render/render 10.51, render/graph/ms 8.60, encode//sum 7.98, stage/render/graph_submit 6.27 |
| warm-up | 6.02 | 28.1 | pipelines/compile_ms 1060, pipelines/front_ms 1025, render/thread/ms 45.04, main/busy/ms 44.41, render/busy/ms 43.36, pipelines/driver_ms 35.08, stage/main/update 25.49, stage/main/post_update 13.74, stage/render/render 13.41, render/graph/ms 11.53, stage/render/prepare_assets 9.84, stage/render/graph_submit 7.92 |
| warm-up | 6.06 | 40.8 | pipelines/compile_ms 304, pipelines/front_ms 295, render/thread/ms 56.41, main/busy/ms 55.24, render/busy/ms 54.50, stage/main/update 24.12, stage/main/post_update 23.78, stage/render/render 15.73, stage/render/prepare_assets 14.87, render/graph/ms 13.15, pipelines/driver_ms 9.37, stage/render/graph_submit 9.01 |
| warm-up | 6.08 | 26.2 | main/busy/ms 64.32, render/thread/ms 52.59, render/busy/ms 50.70, stage/main/update 29.76, stage/main/post_update 24.50, stream/drive/ms 18.53, stage/render/prepare_assets 16.66, stage/render/render 16.26, render/graph/ms 13.41, stage/render/graph_submit 9.53, encode//sum 9.12, encode/submit 5.80 |
| warm-up | 6.13 | 43.2 | main/busy/ms 49.30, render/thread/ms 41.49, render/busy/ms 39.87, stage/main/update 27.35, stream/drive/ms 18.08, stage/render/prepare_assets 14.87, stage/main/post_update 13.15, stage/render/render 12.09, render/graph/ms 9.75, encode//sum 8.45, stage/render/graph_submit 6.94, encode/submit 3.62 |
| warm-up | 6.16 | 38.3 | render/thread/ms 51.49, render/busy/ms 49.62, main/busy/ms 39.62, stage/main/update 18.21, stage/render/prepare_assets 17.37, pipelines/compile_ms 15.96, pipelines/front_ms 14.54, stage/render/render 14.51, stage/main/post_update 13.42, render/graph/ms 12.02, stage/render/graph_submit 8.73, encode//sum 7.50 |
| warm-up | 6.19 | 30.2 | pipelines/compile_ms 1217, pipelines/front_ms 1177, render/thread/ms 51.50, render/busy/ms 49.80, main/busy/ms 41.29, pipelines/driver_ms 40.45, stage/render/prepare_assets 21.84, stage/main/update 19.13, stage/main/post_update 14.10, stage/render/render 12.76, render/graph/ms 10.68, stage/render/graph_submit 7.67 |
| warm-up | 6.22 | 25.5 | pipelines/compile_ms 1232, pipelines/front_ms 1191, render/thread/ms 41.69, pipelines/driver_ms 40.74, render/busy/ms 40.30, main/busy/ms 29.42, stage/render/prepare_assets 20.54, stage/main/update 12.84, stage/main/post_update 11.11, stage/render/render 9.60, render/graph/ms 7.88, encode//sum 6.83 |
| warm-up | 6.28 | 25.6 | pipelines/compile_ms 43.92, pipelines/front_ms 42.35, main/busy/ms 35.25, render/thread/ms 34.84, render/busy/ms 33.38, stage/main/update 17.21, stage/render/render 15.03, render/graph/ms 13.22, stage/main/post_update 11.30, encode//sum 10.49, stage/render/graph_submit 9.59, stage/render/prepare_assets 6.91 |
| warm-up | 6.37 | 35.0 | pipelines/compile_ms 450, pipelines/front_ms 436, render/thread/ms 45.38, main/busy/ms 44.22, render/busy/ms 43.61, stage/main/update 23.92, stage/render/render 19.01, render/graph/ms 16.63, pipelines/driver_ms 14.81, stage/main/post_update 11.40, stage/render/prepare_assets 9.07, stage/render/graph_submit 8.01 |
| warm-up | 6.40 | 27.0 | pipelines/compile_ms 450, pipelines/front_ms 436, render/thread/ms 51.67, render/busy/ms 50.27, main/busy/ms 30.53, stage/render/prepare_assets 20.32, pipelines/driver_ms 14.81, stage/render/render 11.94, stage/main/update 10.87, stage/main/post_update 10.27, render/graph/ms 9.80, encode//sum 7.70 |
| warm-up | 6.45 | 27.0 | pipelines/compile_ms 570, pipelines/front_ms 556, render/thread/ms 46.01, render/busy/ms 44.83, main/busy/ms 28.88, stage/render/prepare_assets 28.38, stage/main/update 13.77, pipelines/driver_ms 13.74, stage/main/post_update 9.05, stage/render/render 9.01, stream/drive/ms 8.04, encode//sum 7.42 |
| warm-up | 6.57 | 27.7 | render/thread/ms 32.41, render/busy/ms 31.12, main/busy/ms 30.82, stage/main/update 19.09, stage/render/prepare_assets 14.65, stream/drive/ms 13.93, stage/render/render 11.32, render/graph/ms 9.66, encode//sum 8.43, stage/main/post_update 7.72, stage/render/graph_submit 7.20, encode/submit 4.25 |
| warm-up | 6.76 | 31.8 | render/thread/ms 34.80, main/busy/ms 34.63, render/busy/ms 33.15, stage/render/render 17.48, stage/main/update 16.12, render/graph/ms 14.92, stage/main/post_update 13.99, encode//sum 11.25, stage/render/graph_submit 10.68, stage/render/prepare_assets 6.94, encode/submit 5.60, encode/finish_wall 5.18 |
| warm-up | 6.83 | 45.1 | render/thread/ms 44.19, render/busy/ms 42.83, main/busy/ms 38.18, stage/render/prepare_assets 25.31, stage/main/update 17.55, stage/main/post_update 14.19, stream/drive/ms 11.14, stage/render/render 11.11, render/graph/ms 9.41, encode//sum 8.17, stage/render/graph_submit 6.61, encode/submit 4.06 |
| warm-up | 7.17 | 39.0 | render/thread/ms 45.81, render/busy/ms 44.93, stage/render/render 18.56, encode//sum 17.77, render/graph/ms 17.35, stage/render/graph_submit 15.56, main/busy/ms 15.37, encode/finish_wall 12.64, encode/pass:early prepass/sum 12.12, stage/render/specialize 8.73, stage/main/post_update 7.35, stage/render/queue 5.33 |
| warm-up | 8.38 | 29.9 | render/thread/ms 36.49, render/busy/ms 33.81, main/busy/ms 14.69, stage/render/render 10.15, render/graph/ms 8.99, stage/render/specialize 8.49, stage/main/post_update 7.94, stage/render/graph_submit 7.00, encode//sum 6.29, stage/render/queue 5.97, encode/submit 4.37, stage/render/prepare_meshes 4.28 |
| measure | 0.83 | 28.0 | render/thread/ms 35.79, render/busy/ms 32.93, main/busy/ms 13.90, stage/render/render 11.75, render/graph/ms 9.73, encode//sum 8.77, stage/render/specialize 8.66, stage/main/post_update 8.22, stage/render/graph_submit 7.75, stage/render/queue 5.81, encode/submit 4.65, stage/main/update 3.19 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set select.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=select-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 14%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 89%) · 2625 MHz (min 2625) · memory 8101 MHz · 97.1 W · 77 °C (max 79) · VRAM 6444 MiB · throttled 100% of samples

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
