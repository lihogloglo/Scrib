# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.skin.skip_unchanged=on (command line: --set); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1565 frames over 15.0 s · **median 9.58 ms (104 fps)** · mean 9.59 · p95 11.04 · p99 11.68 (1% low 86 fps) · worst 26.31 ms · 4 spikes over 1.5× median · 16.0 stutters/min (work 8.0/min)
- warm-up: 280 frames over 8.4 s, worst 2558.4 ms
- hitches (> 25.0 ms): **1** measured, 56 in warm-up
- timed GPU passes: 8.40 ms of a 9.59 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.58 ms, limit 11 ms
- pass worst frame 26.31 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.805 | 2.823 | 2.941 | 0.99 | 0.069 |
| dlss_super_resolution | 2.007 | 2.020 | 2.043 | 0.99 | 0.177 |
| early prepass | 0.608 | 0.612 | 0.631 | 0.99 | 0.094 |
| atmosphere_luts | 0.323 | 0.325 | 0.392 | 0.99 | 0.003 |
| bloom | 0.312 | 0.314 | 0.330 | 0.99 | 0.061 |
| main_transmissive_pass_3d | 0.294 | 0.296 | 0.331 | 0.99 | 0.008 |
| water_velocity | 0.237 | 0.239 | 0.265 | 0.99 | 0.002 |
| water_sspr | 0.205 | 0.206 | 0.212 | 0.99 | 0.002 |
| ssao | 0.187 | 0.188 | 0.197 | 0.99 | 0.007 |
| clouds_trace | 0.148 | 0.149 | 0.169 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.143 | 0.144 | 0.152 | 0.99 | 0.001 |
| water_fft | 0.130 | 0.131 | 0.134 | 0.99 | 0.003 |
| uniform_allocation | 0.119 | 0.120 | 0.163 | 0.99 | 0.022 |
| upscaling | 0.112 | 0.113 | 0.118 | 0.99 | 0.004 |
| clouds_composite | 0.112 | 0.112 | 0.114 | 0.99 | 0.002 |
| bin_unpacking | 0.089 | 0.090 | 0.121 | 0.99 | 0.017 |
| tonemapping | 0.085 | 0.086 | 0.092 | 0.99 | 0.004 |
| clustering | 0.064 | 0.065 | 0.077 | 0.99 | 0.021 |
| render_sky | 0.052 | 0.052 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.412 | 0.414 | 0.13 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.052 | 0.99 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.036 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.033 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 0.99 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.229 | 0.13 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 0.99 | 0.082 |
| water_snapshot | 0.024 | 0.025 | 0.026 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| surface_exposure_far | 0.014 | 0.045 | 0.047 | 0.31 | 0.002 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.99 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.007 | 0.007 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.13 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.003 | 0.99 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.654 | 10.112 | 25.11 | 13543.8 |
| main/busy/ms | 7.113 | 7.976 | 11.48 | 11132.0 |
| render/busy/ms | 6.865 | 8.101 | 24.57 | 10743.2 |
| stage/render/render | 4.568 | 5.613 | 11.55 | 7148.5 |
| stage/main/post_update | 4.192 | 4.962 | 8.37 | 6559.9 |
| render/graph/ms | 3.988 | 4.955 | 10.11 | 6241.1 |
| encode//sum | 3.488 | 5.236 | 17.28 | 5458.7 |
| stage/render/graph_submit | 2.996 | 3.952 | 8.33 | 4688.0 |
| encode/submit | 1.895 | 2.526 | 6.78 | 2964.9 |
| stage/main/update | 1.815 | 2.217 | 3.42 | 2840.0 |
| stage/render/prepare_views | 1.473 | 3.084 | 4.51 | 2304.5 |
| render/acquire/ms | 1.428 | 3.032 | 4.46 | 2235.1 |
| encode/finish_wall | 1.126 | 1.586 | 5.13 | 1762.7 |
| stage/render/camera_driver | 0.938 | 1.092 | 1.77 | 1467.2 |
| encode/pass:shadow_directional_light/sum | 0.720 | 1.223 | 4.51 | 1126.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.703 | 0.982 | 1.94 | 1099.7 |
| encode/pass:early prepass/sum | 0.699 | 0.954 | 4.13 | 1094.1 |
| stage/render/prepare_write_phase_buffers | 0.547 | 0.750 | 1.09 | 856.4 |
| stage/render/prepare_assets | 0.450 | 0.578 | 1.70 | 703.7 |
| stage/render/prepare_resources | 0.376 | 0.438 | 0.91 | 589.1 |
| bigspace/total/ms | 0.376 | 0.869 | 5.09 | 588.5 |
| render/present/ms | 0.361 | 0.435 | 4.69 | 565.4 |
| stage/main/pre_update | 0.341 | 0.450 | 1.24 | 533.1 |
| stage/main/first | 0.286 | 0.360 | 1.55 | 447.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.243 | 0.377 | 0.76 | 379.6 |
| terrain/update/ms | 0.233 | 0.299 | 0.76 | 364.6 |
| stage/render/prepare_bind_groups | 0.210 | 0.297 | 0.44 | 328.3 |
| stage/render/prepare_meshes | 0.205 | 0.261 | 2.55 | 320.5 |
| terrain/select/ms | 0.203 | 0.260 | 0.73 | 318.1 |
| stream/drive/ms | 0.185 | 0.245 | 0.79 | 289.1 |
| stage/main/fixed_main_loop | 0.182 | 0.340 | 1.15 | 284.2 |
| stage/render/prepare_batch_phases | 0.179 | 0.242 | 0.40 | 280.0 |
| stage/render/extract_commands | 0.170 | 0.203 | 0.43 | 265.3 |
| stage/render/create_views | 0.165 | 0.211 | 0.64 | 258.6 |
| far/select/ms | 0.162 | 0.205 | 0.64 | 253.9 |
| controller/move/ms | 0.162 | 0.303 | 0.83 | 253.2 |
| stream/tick/ms | 0.157 | 0.212 | 0.59 | 245.1 |
| controller/feed/ms | 0.149 | 0.199 | 0.63 | 233.3 |
| stage/render/prepare_flush | 0.146 | 0.276 | 0.63 | 228.3 |
| bigspace/high_precision/ms | 0.134 | 0.410 | 1.69 | 209.4 |
| stage/main/last | 0.116 | 0.152 | 1.39 | 181.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.111 | 0.177 | 0.48 | 173.0 |
| stage/render/queue | 0.106 | 0.134 | 5.39 | 165.5 |
| bigspace/low_precision/ms | 0.094 | 0.209 | 1.40 | 146.5 |
| anim/evaluate/ms | 0.084 | 0.191 | 1.36 | 132.1 |
| stage/render/specialize | 0.084 | 0.104 | 8.61 | 131.6 |
| stage/render/prepare_collect_phase_buffers | 0.062 | 0.119 | 0.23 | 97.2 |
| anim/evaluate_cpu/ms | 0.060 | 0.074 | 0.53 | 94.4 |
| encode/resolve_diagnostics/sum | 0.045 | 0.080 | 0.29 | 70.1 |
| physics/step/ms | 0.028 | 0.058 | 0.11 | 43.4 |
| weather/map/ms | 0.025 | 0.032 | 0.41 | 39.1 |
| stage/render/cleanup | 0.022 | 0.031 | 0.31 | 35.2 |
| encode/camera_driver_span_begin/sum | 0.021 | 0.035 | 0.19 | 32.6 |
| stage/render/phase_sort | 0.018 | 0.034 | 0.21 | 28.5 |
| encode/camera_driver_span_end/sum | 0.018 | 0.032 | 0.16 | 28.0 |
| weather/eval/ms | 0.011 | 0.014 | 0.43 | 16.8 |
| script/update/ms | 0.007 | 0.009 | 0.39 | 11.6 |
| controller/avoid/ms | 0.006 | 0.007 | 0.02 | 8.9 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 8.4 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 7.0 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 6.0 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 1.0 |
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
| crowd/c1/count | count | 123 | 123 | 122 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1080 |
| crowd/c2/count | count | 283 | 284 | 284 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 136 | 181 | 112 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.45 | 4.93 | 0.35 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.19 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.16 | 0.02 |
| encode/finish_critical | ms | 0.82 | 4.93 | 0.88 |
| encode/pass:early prepass/max | ms | 0.69 | 4.13 | 0.88 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.71 | 1.71 | 0.63 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.48 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.70 | 0.30 |
| encode/pass:shadow_directional_light/max | ms | 0.65 | 4.51 | 0.51 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.29 | 0.03 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.13 | 24.57 | 9.07 |
| gpu/timed/ms | ms | 9.13 | 10.47 | 9.07 |
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
| process/cpu_usage | % | 24.55 | 34.62 | 21.05 |
| process/mem_usage | GiB | 4.67 | 4.72 | 4.72 |
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
| skin/blocks_skipped/count | count | 176 | 178 | 177 |
| skin/blocks_walked/count | count | 5 | 7 | 4 |
| skin/joint_bytes_written/bytes | bytes | 9472 | 12.5 k | 7680 |
| skin/joints_written/count | count | 148 | 196 | 120 |
| skin/upload_writes/count | count | 3 | 4 | 3 |
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
| system/cpu_usage | % | 29.06 | 44.91 | 27.91 |
| system/mem_usage | % | 58.05 | 58.24 | 58.22 |
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
| anim/events/count | count | 0.10 | 3 | 154 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.16 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.56 | 52 | 41.6 k |
| encode/bind_walk/init_textures | count | 7165 | 16.3 k | 11.21 M |
| encode/bind_walk/init_textures_skipped | count | 35.6 k | 79.9 k | 55.72 M |
| encode/bind_walk/inits | count | 345 | 721 | 540.5 k |
| encode/bind_walk/inits_skipped | count | 67.00 | 142 | 104.9 k |
| encode/bind_walk/merge_textures | count | 7101 | 16.2 k | 11.11 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 8162 | 6.39 M |
| encode/bind_walk/merges | count | 82.26 | 173 | 128.7 k |
| encode/bind_walk/merges_skipped | count | 50.75 | 105 | 79.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1565 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1565 |
| encode/pass:early prepass/count | count | 1 | 2 | 1565 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1565 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 1565 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 1565 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 1565 |
| encode/queue/command_buffers | count | 36.56 | 72 | 57.2 k |
| encode/queue/submits | count | 2 | 4 | 3130 |
| encode/queue/write_buffer/bytes | bytes | 1.14 M | 3.35 M | 1.78 G |
| encode/queue/write_buffer/calls | count | 219 | 471 | 343.2 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 65.3 k | 51.08 M |
| encode/queue/write_buffer_with/calls | count | 9 | 18 | 14.1 k |
| encode/queue/write_texture/bytes | bytes | 11.5 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1565 |
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

## Hitches (> 25.0 ms; slowest 57, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2558.4 | main/busy/ms 2528, stage/main/post_update 2421, stage/main/update 97.83, surface/advance_cpu_ms 66.38, stage/main/pre_update 4.99, stage/main/last 1.27, surface/cell_upload_ms 0.75, stage/main/first 0.64, stage/main/fixed_main_loop 0.46, stream/drive/ms 0.35, weather/map/ms 0.24, stream/tick/ms 0.09 |
| warm-up | 3.31 | 749.6 | main/busy/ms 2670, stage/main/post_update 2446, render/thread/ms 711, render/busy/ms 546, stage/render/prepare_assets 496, stage/main/update 209, stage/render/prepare_views 162, render/acquire/ms 162, surface/advance_cpu_ms 66.38, stage/render/create_views 28.67, stage/render/render 12.62, render/graph/ms 8.13 |
| warm-up | 3.42 | 110.3 | render/thread/ms 804, render/busy/ms 586, stage/render/prepare_assets 520, main/busy/ms 165, stage/render/prepare_views 162, render/acquire/ms 162, stage/main/update 116, stage/render/render 72.47, render/present/ms 56.03, pipelines/compile_ms 43.89, pipelines/front_ms 41.05, stage/main/post_update 38.61 |
| warm-up | 3.45 | 36.2 | pipelines/compile_ms 271, pipelines/front_ms 246, render/thread/ms 117, main/busy/ms 79.06, stage/render/render 76.68, render/present/ms 63.91, render/busy/ms 52.16, stage/main/post_update 49.46, terrain/update/ms 26.50, stage/render/prepare_assets 25.25, pipelines/driver_ms 25.07, stage/main/last 13.62 |
| warm-up | 3.60 | 140.5 | pipelines/wait_ms 3752, pipelines/compile_ms 1387, pipelines/front_ms 1297, render/thread/ms 141, render/busy/ms 129, main/busy/ms 112, pipelines/driver_ms 89.57, stage/main/post_update 75.56, stage/render/render 49.43, encode//sum 43.90, stage/render/prepare_resources 38.61, render/graph/ms 36.65 |
| warm-up | 3.64 | 48.7 | pipelines/wait_ms 5142, pipelines/compile_ms 2011, pipelines/front_ms 1903, render/thread/ms 148, render/busy/ms 146, main/busy/ms 121, pipelines/driver_ms 108, stage/main/update 52.83, stage/main/post_update 50.85, encode//sum 47.26, stage/render/prepare_resources 38.50, stage/render/render 36.96 |
| warm-up | 3.76 | 120.2 | pipelines/compile_ms 2195, pipelines/front_ms 2094, pipelines/wait_ms 1551, render/thread/ms 111, render/busy/ms 109, pipelines/driver_ms 101, main/busy/ms 82.02, stage/render/prepare_assets 68.70, stage/main/update 55.09, stage/render/render 20.98, stage/main/post_update 19.43, render/graph/ms 18.67 |
| warm-up | 3.81 | 46.1 | pipelines/compile_ms 1878, pipelines/front_ms 1800, pipelines/wait_ms 216, render/thread/ms 122, render/busy/ms 120, stage/render/prepare_assets 88.37, pipelines/driver_ms 77.81, main/busy/ms 63.96, stage/main/pre_update 31.38, stage/render/render 25.20, render/graph/ms 22.89, stage/main/post_update 19.12 |
| warm-up | 3.85 | 40.3 | pipelines/compile_ms 601, pipelines/front_ms 567, render/thread/ms 76.37, render/busy/ms 73.42, pipelines/wait_ms 65.12, main/busy/ms 62.85, stage/render/prepare_assets 48.25, pipelines/driver_ms 34.56, stage/main/pre_update 30.91, stage/main/post_update 17.82, encode//sum 15.89, stage/render/render 15.81 |
| warm-up | 3.90 | 26.5 | pipelines/compile_ms 275, pipelines/front_ms 263, render/thread/ms 42.99, render/busy/ms 39.80, main/busy/ms 32.24, stage/render/prepare_assets 15.98, stage/main/post_update 15.19, stage/render/render 15.01, pipelines/driver_ms 12.19, render/graph/ms 11.32, pipelines/wait_ms 10.39, pipelines/lock_ms 10.06 |
| warm-up | 3.94 | 43.2 | pipelines/compile_ms 610, pipelines/front_ms 588, render/thread/ms 65.71, main/busy/ms 63.60, render/busy/ms 62.39, stage/main/update 44.86, stage/render/prepare_assets 42.17, stream/drive/ms 41.01, pipelines/driver_ms 22.10, stage/render/render 13.36, render/graph/ms 9.62, terrain/update/ms 8.96 |
| warm-up | 3.99 | 52.9 | pipelines/compile_ms 1394, pipelines/front_ms 1328, render/thread/ms 79.15, render/busy/ms 74.13, pipelines/driver_ms 66.47, main/busy/ms 56.16, stage/render/prepare_assets 53.71, stage/main/update 45.08, stream/drive/ms 41.08, stage/render/render 15.39, render/graph/ms 10.19, stage/render/graph_submit 7.56 |
| warm-up | 4.05 | 35.8 | pipelines/compile_ms 163, pipelines/front_ms 156, render/thread/ms 52.17, render/busy/ms 50.17, main/busy/ms 36.53, stage/main/update 23.33, stream/drive/ms 19.32, stage/render/prepare_assets 19.17, stage/render/render 15.26, render/graph/ms 12.69, stage/render/graph_submit 9.21, stage/render/prepare_resources 9.03 |
| warm-up | 4.09 | 30.3 | pipelines/compile_ms 236, pipelines/front_ms 228, render/thread/ms 54.97, render/busy/ms 53.21, stage/render/prepare_assets 35.13, main/busy/ms 29.75, stage/render/render 12.54, stage/main/post_update 12.43, render/graph/ms 10.04, stage/main/update 9.70, pipelines/driver_ms 8.00, stage/render/graph_submit 7.08 |
| warm-up | 4.15 | 63.3 | pipelines/compile_ms 419, pipelines/front_ms 406, render/thread/ms 73.94, render/busy/ms 72.27, main/busy/ms 70.50, stage/render/prepare_assets 52.13, stage/main/update 45.13, stream/drive/ms 18.60, stage/main/post_update 18.35, stage/render/render 13.00, pipelines/driver_ms 12.77, render/graph/ms 10.76 |
| warm-up | 4.21 | 61.4 | pipelines/compile_ms 967, pipelines/front_ms 938, render/thread/ms 86.62, render/busy/ms 84.80, main/busy/ms 68.41, stage/render/prepare_assets 60.76, stage/main/update 45.83, pipelines/driver_ms 28.93, stream/drive/ms 19.89, stage/main/post_update 14.65, stage/render/render 14.02, render/graph/ms 11.51 |
| warm-up | 4.25 | 39.5 | pipelines/compile_ms 1194, pipelines/front_ms 1160, main/busy/ms 89.48, stage/main/update 69.79, render/thread/ms 66.96, render/busy/ms 65.23, stream/drive/ms 53.45, stage/render/prepare_assets 42.34, pipelines/driver_ms 34.09, stage/render/render 13.19, stage/main/post_update 10.74, render/graph/ms 10.53 |
| warm-up | 4.37 | 116.4 | pipelines/compile_ms 1078, pipelines/front_ms 1038, main/busy/ms 95.30, render/thread/ms 72.46, stage/main/update 72.33, render/busy/ms 70.85, stream/drive/ms 54.06, stage/render/prepare_assets 48.72, pipelines/driver_ms 40.09, stage/main/post_update 13.76, stage/render/render 11.64, render/graph/ms 9.36 |
| warm-up | 4.44 | 74.2 | pipelines/compile_ms 1192, pipelines/front_ms 1144, pipelines/wait_ms 134, render/thread/ms 103, render/busy/ms 101, main/busy/ms 66.46, stage/render/prepare_assets 58.77, pipelines/driver_ms 48.40, stage/main/update 40.03, stage/main/post_update 16.02, stream/drive/ms 14.94, stage/render/prepare_resources 14.61 |
| warm-up | 4.53 | 86.6 | pipelines/compile_ms 898, pipelines/front_ms 867, pipelines/wait_ms 510, render/thread/ms 117, render/busy/ms 116, main/busy/ms 75.34, stage/render/prepare_assets 65.49, stage/main/update 51.98, pipelines/driver_ms 31.20, stage/render/prepare_resources 17.83, stage/main/post_update 15.69, stream/drive/ms 13.78 |
| warm-up | 4.58 | 55.6 | pipelines/compile_ms 1611, pipelines/front_ms 1561, pipelines/wait_ms 469, render/thread/ms 106, render/busy/ms 105, stage/render/prepare_assets 66.88, main/busy/ms 60.08, pipelines/driver_ms 49.95, stage/main/update 31.67, stage/main/post_update 11.67, stage/main/first 10.47, stage/render/prepare_resources 9.96 |
| warm-up | 4.62 | 42.8 | pipelines/compile_ms 2145, pipelines/front_ms 2067, pipelines/wait_ms 209, render/thread/ms 82.68, render/busy/ms 78.17, pipelines/driver_ms 77.29, main/busy/ms 39.58, stage/render/prepare_assets 34.78, stage/render/render 26.36, render/graph/ms 21.36, encode/submit 19.24, stage/render/graph_submit 18.59 |
| warm-up | 4.65 | 26.3 | pipelines/compile_ms 1078, pipelines/front_ms 1032, pipelines/wait_ms 126, render/thread/ms 60.36, render/busy/ms 55.79, pipelines/driver_ms 45.71, main/busy/ms 34.07, stage/render/render 27.65, render/graph/ms 22.55, stage/render/graph_submit 19.53, encode/submit 19.46, stage/main/update 14.68 |
| warm-up | 4.69 | 41.0 | pipelines/compile_ms 604, pipelines/front_ms 586, pipelines/wait_ms 65.77, render/thread/ms 60.08, render/busy/ms 58.52, main/busy/ms 53.16, stage/main/post_update 21.60, pipelines/driver_ms 18.87, stage/main/update 18.44, stage/render/render 12.07, stage/render/prepare_batch_phases 9.91, stage/render/prepare_write_phase_buffers 9.62 |
| warm-up | 4.72 | 26.0 | pipelines/compile_ms 415, pipelines/front_ms 402, main/busy/ms 129, stage/main/update 99.01, stream/drive/ms 88.62, pipelines/wait_ms 58.44, render/thread/ms 53.83, render/busy/ms 52.44, stage/render/prepare_assets 16.33, stage/main/post_update 15.23, pipelines/driver_ms 13.05, stage/render/render 11.57 |
| warm-up | 4.83 | 107.3 | pipelines/compile_ms 1787, pipelines/front_ms 1734, pipelines/wait_ms 159, main/busy/ms 111, stage/main/update 96.72, stream/drive/ms 89.71, pipelines/driver_ms 53.08, render/thread/ms 39.63, render/busy/ms 38.31, stage/render/prepare_assets 22.13, stage/render/render 9.26, stage/main/post_update 8.51 |
| warm-up | 4.93 | 56.0 | pipelines/compile_ms 1023, pipelines/front_ms 991, main/busy/ms 61.69, render/thread/ms 48.37, render/busy/ms 46.69, stage/main/update 43.50, pipelines/driver_ms 31.60, pipelines/wait_ms 28.22, stage/render/prepare_assets 20.69, stage/render/render 13.32, render/graph/ms 11.13, stage/main/post_update 10.71 |
| warm-up | 4.97 | 39.4 | pipelines/compile_ms 1163, pipelines/front_ms 1130, pipelines/wait_ms 77.92, render/thread/ms 57.54, render/busy/ms 56.19, main/busy/ms 45.27, pipelines/driver_ms 33.35, stage/render/prepare_assets 32.09, stage/main/update 31.86, stream/drive/ms 9.57, stage/render/render 9.40, stage/main/post_update 7.67 |
| warm-up | 5.01 | 26.7 | pipelines/compile_ms 263, pipelines/front_ms 253, render/thread/ms 42.15, render/busy/ms 40.33, main/busy/ms 36.92, stage/main/update 20.31, stage/render/render 12.64, stage/main/post_update 10.97, render/graph/ms 10.53, pipelines/driver_ms 10.04, stage/render/graph_submit 7.90, encode//sum 7.13 |
| warm-up | 5.07 | 36.4 | pipelines/compile_ms 307, pipelines/front_ms 286, main/busy/ms 46.87, render/thread/ms 43.91, render/busy/ms 41.64, stage/main/update 33.22, pipelines/driver_ms 20.69, stream/drive/ms 15.52, stage/render/render 13.10, encode//sum 11.92, render/graph/ms 10.36, stage/render/prepare_assets 8.97 |
| warm-up | 5.10 | 32.1 | pipelines/compile_ms 268, pipelines/front_ms 249, render/thread/ms 51.01, render/busy/ms 49.57, main/busy/ms 47.85, stage/main/update 27.56, pipelines/driver_ms 19.39, stage/main/post_update 12.65, stage/render/prepare_assets 11.27, stage/render/render 10.76, render/graph/ms 8.93, encode//sum 7.11 |
| warm-up | 5.15 | 46.1 | pipelines/compile_ms 386, pipelines/front_ms 373, main/busy/ms 59.06, render/thread/ms 58.94, render/busy/ms 57.66, stage/main/update 34.58, stage/render/prepare_assets 25.18, stage/main/post_update 16.05, pipelines/driver_ms 12.87, stage/render/render 12.74, render/graph/ms 10.90, encode//sum 8.22 |
| warm-up | 5.20 | 47.9 | pipelines/compile_ms 1228, pipelines/front_ms 1176, render/thread/ms 76.03, render/busy/ms 74.70, main/busy/ms 63.85, pipelines/driver_ms 52.02, stage/render/prepare_assets 47.80, stage/main/update 35.53, stage/main/post_update 21.71, stage/render/render 12.17, render/graph/ms 10.22, encode//sum 7.42 |
| warm-up | 5.25 | 52.3 | pipelines/compile_ms 1568, pipelines/front_ms 1510, render/thread/ms 87.12, render/busy/ms 85.72, main/busy/ms 60.37, stage/render/prepare_assets 58.73, pipelines/driver_ms 58.70, stage/main/update 33.06, stage/main/post_update 19.91, stage/render/render 9.68, render/graph/ms 7.77, stream/drive/ms 6.19 |
| warm-up | 5.29 | 43.3 | pipelines/compile_ms 1725, pipelines/front_ms 1678, render/thread/ms 72.35, render/busy/ms 71.02, pipelines/driver_ms 47.22, stage/render/prepare_assets 47.18, main/busy/ms 47.08, pipelines/wait_ms 46.56, stage/main/update 26.76, stage/main/post_update 10.36, stage/render/render 9.57, render/graph/ms 7.74 |
| warm-up | 5.40 | 25.9 | pipelines/compile_ms 846, pipelines/front_ms 824, render/thread/ms 45.32, render/busy/ms 44.02, main/busy/ms 38.72, pipelines/driver_ms 22.03, stage/main/update 18.86, stage/render/prepare_assets 15.67, stage/main/post_update 14.89, stream/drive/ms 11.57, stage/render/render 11.04, render/graph/ms 9.26 |
| warm-up | 5.44 | 25.2 | pipelines/compile_ms 576, pipelines/front_ms 563, pipelines/wait_ms 63.52, render/thread/ms 39.04, render/busy/ms 37.70, main/busy/ms 29.28, stage/render/prepare_assets 16.48, pipelines/driver_ms 13.91, stage/main/update 12.24, stage/main/post_update 11.13, stage/render/render 10.11, render/graph/ms 8.36 |
| warm-up | 5.49 | 28.7 | render/thread/ms 45.00, render/busy/ms 43.73, main/busy/ms 37.52, stage/render/prepare_assets 16.19, stage/main/post_update 15.49, stage/main/update 14.91, stage/render/render 13.41, render/graph/ms 11.34, encode//sum 9.85, stage/render/graph_submit 8.11, stage/render/prepare_write_phase_buffers 5.09, encode/submit 4.79 |
| warm-up | 5.52 | 32.9 | pipelines/compile_ms 390, pipelines/front_ms 379, render/thread/ms 55.11, render/busy/ms 53.62, main/busy/ms 39.59, stage/render/prepare_assets 28.81, stage/main/update 18.71, stage/main/post_update 13.66, stage/render/render 13.45, render/graph/ms 11.13, pipelines/driver_ms 10.59, stage/render/graph_submit 8.10 |
| warm-up | 5.56 | 40.0 | pipelines/compile_ms 735, pipelines/front_ms 712, render/thread/ms 65.01, render/busy/ms 63.49, main/busy/ms 41.62, stage/render/prepare_assets 39.76, stage/main/update 26.38, pipelines/driver_ms 23.24, stage/render/render 11.09, stage/main/post_update 9.32, render/graph/ms 9.12, encode//sum 6.61 |
| warm-up | 5.63 | 43.6 | pipelines/compile_ms 578, pipelines/front_ms 560, render/thread/ms 57.03, render/busy/ms 55.47, main/busy/ms 39.66, stage/render/prepare_assets 24.88, stage/main/update 17.61, pipelines/driver_ms 17.60, stage/render/render 15.02, stage/main/post_update 12.83, render/graph/ms 12.79, encode//sum 10.48 |
| warm-up | 5.66 | 34.3 | pipelines/compile_ms 783, pipelines/front_ms 761, render/thread/ms 60.92, render/busy/ms 59.60, main/busy/ms 38.14, stage/render/prepare_assets 35.64, pipelines/driver_ms 22.01, stage/main/update 17.49, stage/main/post_update 11.90, stage/render/render 9.23, render/graph/ms 7.45, encode//sum 7.05 |
| warm-up | 5.70 | 34.5 | pipelines/compile_ms 206, pipelines/front_ms 201, render/thread/ms 56.39, render/busy/ms 55.05, stage/render/prepare_assets 34.23, main/busy/ms 32.33, stage/main/update 13.46, stage/main/post_update 11.71, stage/render/render 8.92, render/graph/ms 7.16, encode//sum 6.95, stage/render/graph_submit 4.70 |
| warm-up | 5.76 | 34.9 | pipelines/compile_ms 771, pipelines/front_ms 751, render/thread/ms 50.03, render/busy/ms 48.76, main/busy/ms 29.37, stage/render/prepare_assets 26.18, pipelines/driver_ms 19.81, stage/render/render 14.22, render/graph/ms 12.47, stage/main/update 12.37, stage/main/post_update 11.40, stage/render/graph_submit 10.00 |
| warm-up | 5.80 | 41.0 | pipelines/compile_ms 111, pipelines/front_ms 106, render/thread/ms 63.48, render/busy/ms 61.53, main/busy/ms 44.19, stage/render/prepare_assets 24.86, stage/main/update 24.29, stage/render/render 15.34, stage/main/post_update 14.16, render/graph/ms 13.29, stage/render/graph_submit 10.24, pipelines/wait_ms 9.32 |
| warm-up | 5.84 | 43.1 | pipelines/compile_ms 111, pipelines/front_ms 106, render/thread/ms 65.11, render/busy/ms 63.26, main/busy/ms 53.39, stage/render/prepare_assets 25.31, stage/main/update 24.27, stage/main/post_update 20.06, stage/render/render 10.79, pipelines/wait_ms 9.32, render/graph/ms 8.78, pipelines/lock_ms 7.91 |
| warm-up | 5.87 | 32.0 | render/thread/ms 57.28, render/busy/ms 55.86, main/busy/ms 48.85, stage/render/prepare_assets 24.55, stage/main/post_update 24.49, stage/render/render 15.08, stage/main/update 14.27, render/graph/ms 12.96, encode//sum 10.72, stage/render/graph_submit 8.79, encode/submit 5.44, stream/drive/ms 5.15 |
| warm-up | 5.91 | 36.5 | main/busy/ms 60.29, render/thread/ms 50.49, render/busy/ms 48.51, stage/main/update 29.95, stream/drive/ms 19.38, stage/render/render 19.04, pipelines/compile_ms 18.87, stage/main/post_update 18.34, pipelines/front_ms 17.63, stage/render/prepare_assets 17.45, render/graph/ms 16.36, stage/render/graph_submit 12.25 |
| warm-up | 5.95 | 46.2 | pipelines/compile_ms 1632, pipelines/front_ms 1570, pipelines/driver_ms 62.18, main/busy/ms 59.73, render/thread/ms 49.66, render/busy/ms 47.95, stage/main/update 34.30, stage/render/prepare_assets 20.73, stream/drive/ms 19.41, stage/render/render 15.49, stage/main/post_update 13.85, render/graph/ms 13.18 |
| warm-up | 6.01 | 27.0 | pipelines/compile_ms 60.10, pipelines/front_ms 57.48, render/thread/ms 39.83, render/busy/ms 38.34, main/busy/ms 37.46, stage/main/update 19.94, stage/render/render 15.74, render/graph/ms 13.85, stage/render/prepare_assets 13.24, stage/main/post_update 11.21, stage/render/graph_submit 10.17, encode//sum 8.33 |
| warm-up | 6.17 | 34.4 | pipelines/compile_ms 800, pipelines/front_ms 779, main/busy/ms 46.35, render/thread/ms 32.94, render/busy/ms 31.62, stage/main/update 28.69, pipelines/driver_ms 21.73, stream/drive/ms 20.01, stage/render/render 12.46, stage/main/post_update 11.29, render/graph/ms 10.70, stage/render/prepare_assets 8.89 |
| warm-up | 6.25 | 29.7 | pipelines/compile_ms 361, pipelines/front_ms 354, main/busy/ms 36.43, render/thread/ms 29.25, render/busy/ms 27.88, stage/main/update 18.98, stage/render/render 12.09, stage/main/post_update 11.58, stage/render/prepare_assets 10.47, render/graph/ms 10.26, stage/render/graph_submit 7.82, encode//sum 7.46 |
| warm-up | 6.45 | 36.3 | render/thread/ms 43.73, render/busy/ms 42.08, main/busy/ms 29.05, stage/render/prepare_assets 20.24, stage/render/render 13.86, stage/main/post_update 12.33, stage/main/update 12.18, render/graph/ms 11.65, encode//sum 10.05, stage/render/graph_submit 7.54, encode/submit 4.56, stage/render/camera_driver 3.95 |
| warm-up | 6.51 | 50.2 | render/thread/ms 44.12, render/busy/ms 42.94, main/busy/ms 38.09, stage/render/prepare_assets 27.85, stage/main/update 18.90, stage/main/post_update 12.94, stream/drive/ms 10.39, stage/render/render 8.97, render/graph/ms 7.41, encode//sum 6.30, stage/render/graph_submit 5.55, encode/submit 3.63 |
| warm-up | 6.87 | 32.5 | render/thread/ms 35.95, render/busy/ms 35.08, main/busy/ms 16.20, stage/render/render 9.06, stage/render/specialize 9.05, stage/main/post_update 8.00, render/graph/ms 7.78, encode//sum 6.61, stage/render/graph_submit 5.87, stage/render/queue 5.71, stage/main/update 5.63, stage/render/prepare_assets 3.40 |
| warm-up | 8.08 | 30.6 | render/thread/ms 37.30, render/busy/ms 34.58, main/busy/ms 14.86, stage/render/render 10.20, stage/render/specialize 9.41, render/graph/ms 9.00, stage/main/post_update 8.87, stage/render/graph_submit 7.10, stage/render/queue 6.35, encode//sum 5.93, encode/submit 4.12, stage/render/prepare_meshes 3.88 |
| measure | 0.82 | 26.3 | render/thread/ms 34.05, render/busy/ms 30.80, main/busy/ms 12.54, stage/render/render 9.34, stage/render/specialize 8.70, render/graph/ms 7.98, stage/main/post_update 7.17, encode//sum 6.40, stage/render/graph_submit 5.96, stage/render/queue 5.46, encode/submit 3.52, stage/main/update 3.20 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set render.skin.skip_unchanged=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=skin-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 98.2 W · 77 °C (max 79) · VRAM 6428 MiB · throttled 100% of samples

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
