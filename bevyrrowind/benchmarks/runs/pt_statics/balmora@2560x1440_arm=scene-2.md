# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1574 frames over 15.0 s · **median 9.57 ms (104 fps)** · mean 9.53 · p95 10.93 · p99 11.36 (1% low 88 fps) · worst 17.32 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 309 frames over 8.4 s, worst 2443.5 ms
- hitches (> 25.0 ms): **0** measured, 41 in warm-up
- timed GPU passes: 8.31 ms of a 9.53 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.57 ms, limit 11 ms
- pass worst frame 17.32 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.540 | 2.551 | 2.592 | 1.00 | 0.066 |
| dlss_super_resolution | 2.017 | 2.026 | 2.052 | 1.00 | 0.180 |
| early prepass | 0.634 | 0.637 | 0.645 | 1.00 | 0.096 |
| atmosphere_luts | 0.323 | 0.325 | 0.340 | 1.00 | 0.003 |
| bloom | 0.314 | 0.316 | 0.332 | 1.00 | 0.060 |
| main_transmissive_pass_3d | 0.309 | 0.310 | 0.332 | 1.00 | 0.008 |
| water_velocity | 0.239 | 0.240 | 0.264 | 1.00 | 0.001 |
| water_sspr | 0.211 | 0.212 | 0.214 | 1.00 | 0.002 |
| ssao | 0.191 | 0.192 | 0.198 | 1.00 | 0.007 |
| clouds_trace | 0.148 | 0.149 | 0.169 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.147 | 0.153 | 1.00 | 0.001 |
| water_fft | 0.131 | 0.132 | 0.135 | 1.00 | 0.003 |
| upscaling | 0.113 | 0.114 | 0.120 | 1.00 | 0.004 |
| uniform_allocation | 0.112 | 0.113 | 0.148 | 1.00 | 0.019 |
| clouds_composite | 0.111 | 0.112 | 0.114 | 1.00 | 0.002 |
| bin_unpacking | 0.091 | 0.091 | 0.115 | 1.00 | 0.015 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| gpu_instance_materialization | 0.067 | 0.067 | 0.090 | 1.00 | 0.004 |
| clustering | 0.065 | 0.066 | 0.076 | 1.00 | 0.020 |
| early_mesh_preprocessing | 0.059 | 0.059 | 0.084 | 1.00 | 0.003 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.412 | 0.414 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.032 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.223 | 0.230 | 0.12 | 0.004 |
| range_unpacking | 0.023 | 0.023 | 0.029 | 1.00 | 0.003 |
| water_snapshot | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.019 | 0.019 | 0.038 | 1.00 | 0.006 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| surface_exposure_far | 0.012 | 0.039 | 0.041 | 0.30 | 0.002 |
| surface_exposure | 0.011 | 0.039 | 0.040 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| main_transparent_pass_3d | 0.005 | 0.005 | 0.017 | 1.00 | 0.045 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.640 | 10.034 | 16.44 | 13599.5 |
| render/busy/ms | 7.102 | 8.190 | 15.88 | 11178.6 |
| main/busy/ms | 7.065 | 7.916 | 10.78 | 11120.8 |
| stage/render/render | 4.644 | 5.437 | 11.84 | 7309.2 |
| render/graph/ms | 4.079 | 4.867 | 7.09 | 6420.0 |
| stage/main/post_update | 4.023 | 4.794 | 7.68 | 6332.5 |
| encode//sum | 3.879 | 5.098 | 8.46 | 6105.8 |
| stage/render/graph_submit | 3.100 | 3.854 | 6.11 | 4879.9 |
| encode/submit | 1.946 | 2.362 | 8.30 | 3063.4 |
| stage/main/update | 1.794 | 2.218 | 2.77 | 2823.8 |
| stage/render/prepare_views | 1.235 | 2.848 | 4.59 | 1943.8 |
| render/acquire/ms | 1.190 | 2.798 | 4.54 | 1873.2 |
| encode/finish_wall | 1.177 | 1.639 | 4.07 | 1853.0 |
| pipelines/compile_ms | 0.989 | 0.000 | 132.88 | 1557.1 |
| pipelines/front_ms | 0.962 | 0.000 | 129.63 | 1514.2 |
| stage/render/camera_driver | 0.917 | 1.045 | 1.33 | 1442.9 |
| encode/pass:shadow_directional_light/sum | 0.773 | 1.354 | 2.18 | 1216.8 |
| encode/pass:early prepass/sum | 0.744 | 1.018 | 3.65 | 1170.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.722 | 1.009 | 1.47 | 1135.8 |
| stage/render/prepare_write_phase_buffers | 0.531 | 0.733 | 0.97 | 836.0 |
| stage/render/prepare_assets | 0.436 | 0.561 | 1.58 | 686.1 |
| stage/render/prepare_resources | 0.382 | 0.439 | 0.66 | 601.4 |
| bigspace/total/ms | 0.371 | 0.917 | 1.61 | 584.2 |
| render/present/ms | 0.348 | 0.415 | 4.62 | 547.8 |
| stage/main/pre_update | 0.337 | 0.442 | 0.89 | 530.0 |
| stage/main/first | 0.297 | 0.361 | 1.79 | 467.2 |
| stage/render/prepare_flush | 0.267 | 0.405 | 0.63 | 420.2 |
| stage/render/prepare_bind_groups | 0.257 | 0.340 | 0.52 | 403.8 |
| stage/main/last | 0.253 | 0.314 | 0.83 | 398.2 |
| terrain/update/ms | 0.233 | 0.299 | 0.67 | 366.7 |
| stage/render/prepare_batch_phases | 0.204 | 0.297 | 0.43 | 320.7 |
| terrain/select/ms | 0.203 | 0.260 | 0.37 | 318.8 |
| stage/render/prepare_meshes | 0.195 | 0.247 | 0.98 | 307.5 |
| stream/drive/ms | 0.190 | 0.260 | 0.67 | 299.0 |
| stage/main/fixed_main_loop | 0.180 | 0.349 | 0.92 | 283.3 |
| stage/render/extract_commands | 0.175 | 0.210 | 0.58 | 275.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.173 | 0.248 | 0.45 | 272.4 |
| far/select/ms | 0.169 | 0.221 | 0.59 | 265.3 |
| stream/tick/ms | 0.163 | 0.226 | 0.64 | 256.5 |
| stage/render/create_views | 0.159 | 0.207 | 0.67 | 250.0 |
| controller/feed/ms | 0.154 | 0.204 | 0.59 | 242.3 |
| bigspace/high_precision/ms | 0.142 | 0.559 | 1.27 | 223.7 |
| controller/move/ms | 0.123 | 0.213 | 0.60 | 193.4 |
| encode/pass:main_transmissive_pass_3d/sum | 0.113 | 0.174 | 0.39 | 177.3 |
| bigspace/low_precision/ms | 0.098 | 0.274 | 1.36 | 154.4 |
| stage/render/specialize | 0.094 | 0.130 | 3.86 | 148.5 |
| anim/evaluate/ms | 0.088 | 0.236 | 1.39 | 138.1 |
| anim/evaluate_cpu/ms | 0.061 | 0.075 | 0.50 | 96.6 |
| stage/render/phase_sort | 0.059 | 0.082 | 0.20 | 93.6 |
| stage/render/queue | 0.059 | 0.082 | 2.13 | 92.1 |
| encode/resolve_diagnostics/sum | 0.054 | 0.079 | 0.57 | 85.1 |
| stage/render/prepare_collect_phase_buffers | 0.043 | 0.071 | 0.15 | 68.1 |
| physics/step/ms | 0.028 | 0.056 | 0.22 | 43.5 |
| pipelines/driver_ms | 0.027 | 0.000 | 4.37 | 42.9 |
| weather/map/ms | 0.026 | 0.034 | 0.51 | 40.6 |
| encode/camera_driver_span_begin/sum | 0.024 | 0.037 | 0.09 | 37.8 |
| stage/render/cleanup | 0.022 | 0.031 | 0.47 | 35.2 |
| encode/camera_driver_span_end/sum | 0.022 | 0.036 | 0.30 | 33.9 |
| weather/eval/ms | 0.011 | 0.014 | 0.05 | 16.9 |
| script/update/ms | 0.007 | 0.009 | 0.05 | 10.5 |
| controller/avoid/ms | 0.006 | 0.008 | 0.02 | 9.5 |
| script/input/ms | 0.005 | 0.006 | 0.08 | 7.9 |
| anim/apply/ms | 0.005 | 0.006 | 0.02 | 7.2 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.5 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.6 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.4 |
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
| ecs/entities | count | 37.4 k | 37.4 k | 37.4 k |
| encode//max | ms | 0.53 | 3.80 | 0.59 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.09 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.30 | 0.02 |
| encode/finish_critical | ms | 0.87 | 3.80 | 0.75 |
| encode/pass:early prepass/max | ms | 0.74 | 3.65 | 0.49 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.73 | 1.47 | 0.75 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.39 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.17 | 0.45 | 0.12 |
| encode/pass:shadow_directional_light/max | ms | 0.71 | 2.18 | 0.44 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.57 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 8.97 | 15.88 | 8.97 |
| gpu/timed/ms | ms | 8.97 | 10.68 | 8.97 |
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
| precache/throttled | count | 0 | 8 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 26.44 | 39.42 | 16.96 |
| process/mem_usage | GiB | 4.65 | 4.73 | 4.73 |
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
| skin/blocks_changed/count | count | 5 | 6 | 5 |
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
| system/cpu_usage | % | 32.51 | 41.64 | 29.56 |
| system/mem_usage | % | 54.20 | 54.48 | 54.48 |
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
| anim/events/count | count | 0.09 | 2 | 141 |
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
| encode/bind_walk/init_textures | count | 7097 | 13.9 k | 11.17 M |
| encode/bind_walk/init_textures_skipped | count | 44.9 k | 57.9 k | 70.61 M |
| encode/bind_walk/inits | count | 330 | 435 | 519.2 k |
| encode/bind_walk/inits_skipped | count | 57.01 | 69 | 89.7 k |
| encode/bind_walk/merge_textures | count | 7033 | 13.8 k | 11.07 M |
| encode/bind_walk/merge_textures_skipped | count | 3925 | 3925 | 6.18 M |
| encode/bind_walk/merges | count | 79.27 | 100 | 124.8 k |
| encode/bind_walk/merges_skipped | count | 35.75 | 41 | 56.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1574 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1574 |
| encode/pass:early prepass/count | count | 1 | 1 | 1574 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1574 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1574 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1574 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1574 |
| encode/queue/command_buffers | count | 37.55 | 40 | 59.1 k |
| encode/queue/submits | count | 2 | 2 | 3148 |
| encode/queue/write_buffer/bytes | bytes | 2.21 M | 3.62 M | 3.47 G |
| encode/queue/write_buffer/calls | count | 229 | 302 | 361.0 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 51.38 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 14.2 k |
| encode/queue/write_texture/bytes | bytes | 11.4 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1574 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.57 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 24 |
| pipelines/created | count | 0.02 | 1 | 24 |
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

## Hitches (> 25.0 ms; slowest 41, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.44 | 2443.5 | main/busy/ms 2416, stage/main/post_update 2311, stage/main/update 95.91, surface/advance_cpu_ms 68.28, stage/main/pre_update 4.50, stage/main/last 1.00, surface/cell_upload_ms 0.71, stage/main/first 0.55, stage/main/fixed_main_loop 0.52, stream/drive/ms 0.37, weather/map/ms 0.14, bigspace/total/ms 0.09 |
| warm-up | 3.08 | 637.5 | main/busy/ms 2584, stage/main/post_update 2337, render/thread/ms 597, render/busy/ms 491, stage/render/prepare_assets 464, stage/main/update 234, stage/render/prepare_views 104, render/acquire/ms 103, surface/advance_cpu_ms 68.28, stage/render/render 11.44, stage/render/create_views 8.19, render/graph/ms 7.39 |
| warm-up | 3.19 | 110.8 | render/thread/ms 692, render/busy/ms 530, stage/render/prepare_assets 490, main/busy/ms 188, stage/main/update 142, stage/render/prepare_views 104, render/acquire/ms 104, stage/render/render 71.14, render/present/ms 58.24, pipelines/compile_ms 41.65, pipelines/front_ms 38.08, stage/main/post_update 37.13 |
| warm-up | 3.24 | 44.8 | pipelines/compile_ms 317, pipelines/front_ms 273, render/thread/ms 125, pipelines/wait_ms 93.89, stage/render/render 83.98, render/present/ms 66.43, main/busy/ms 63.54, render/busy/ms 57.60, stage/main/post_update 49.43, pipelines/driver_ms 44.27, stage/render/prepare_assets 27.02, terrain/update/ms 25.79 |
| warm-up | 3.36 | 125.5 | pipelines/wait_ms 4862, pipelines/compile_ms 1530, pipelines/front_ms 1422, render/thread/ms 152, main/busy/ms 146, render/busy/ms 140, stage/main/post_update 135, pipelines/driver_ms 108, stage/render/render 44.25, stage/render/prepare_resources 43.41, render/graph/ms 31.62, stage/render/graph_submit 27.64 |
| warm-up | 3.41 | 49.9 | pipelines/wait_ms 5138, pipelines/compile_ms 1735, pipelines/front_ms 1634, main/busy/ms 192, render/thread/ms 155, render/busy/ms 153, stage/main/post_update 106, pipelines/driver_ms 100, stage/main/update 57.98, stage/render/prepare_resources 45.99, stage/render/prepare_assets 32.03, stage/main/fixed_main_loop 24.83 |
| warm-up | 3.54 | 126.9 | pipelines/compile_ms 1846, pipelines/front_ms 1758, pipelines/wait_ms 610, main/busy/ms 102, render/thread/ms 99.77, render/busy/ms 97.19, pipelines/driver_ms 87.23, stage/render/prepare_assets 62.90, stage/main/update 59.33, stage/main/fixed_main_loop 24.94, stage/render/prepare_write_phase_buffers 15.66, stage/main/post_update 14.81 |
| warm-up | 3.57 | 33.6 | pipelines/compile_ms 2103, pipelines/front_ms 1942, pipelines/wait_ms 315, pipelines/driver_ms 161, render/thread/ms 98.19, render/busy/ms 95.35, stage/render/prepare_assets 77.70, main/busy/ms 27.99, encode//sum 14.64, stage/main/post_update 14.48, stage/render/render 13.56, render/graph/ms 10.63 |
| warm-up | 3.65 | 32.6 | main/busy/ms 64.65, pipelines/compile_ms 45.62, render/thread/ms 45.49, render/busy/ms 43.71, pipelines/front_ms 43.49, stage/main/post_update 33.01, stage/main/update 24.78, stage/render/prepare_assets 11.80, stage/render/render 10.87, stage/render/prepare_resources 10.33, render/graph/ms 8.31, terrain/update/ms 8.17 |
| warm-up | 3.72 | 77.5 | main/busy/ms 117, stage/main/update 81.86, render/thread/ms 81.11, render/busy/ms 78.41, pipelines/compile_ms 60.31, pipelines/front_ms 56.84, stage/render/prepare_assets 43.74, stage/main/post_update 23.11, stage/render/render 22.15, render/graph/ms 18.69, stage/render/graph_submit 15.33, encode//sum 10.45 |
| warm-up | 3.88 | 151.7 | pipelines/compile_ms 1628, pipelines/front_ms 1542, render/thread/ms 132, render/busy/ms 129, pipelines/driver_ms 86.59, main/busy/ms 83.56, stage/render/prepare_assets 79.39, stage/main/update 65.78, pipelines/wait_ms 27.48, stage/render/create_views 24.73, stage/render/render 20.14, render/graph/ms 17.25 |
| warm-up | 3.93 | 50.4 | pipelines/compile_ms 1907, pipelines/front_ms 1813, render/thread/ms 107, render/busy/ms 105, pipelines/driver_ms 93.72, stage/render/prepare_assets 68.28, pipelines/wait_ms 32.24, stage/render/create_views 24.53, main/busy/ms 20.58, stage/render/render 8.96, stage/main/update 8.54, render/graph/ms 7.35 |
| warm-up | 4.00 | 29.4 | main/busy/ms 124, stage/main/update 114, render/thread/ms 26.18, render/busy/ms 24.95, stream/drive/ms 17.20, pipelines/compile_ms 14.93, pipelines/front_ms 14.17, stage/render/render 9.39, stage/render/prepare_assets 9.09, encode//sum 7.91, render/graph/ms 7.31, stage/main/post_update 6.52 |
| warm-up | 4.13 | 134.6 | pipelines/compile_ms 1187, pipelines/front_ms 1154, main/busy/ms 123, stage/main/update 108, render/thread/ms 53.61, render/busy/ms 51.50, pipelines/driver_ms 33.20, stage/render/prepare_assets 27.31, stage/render/render 16.96, render/graph/ms 14.11, stage/main/post_update 9.45, stage/render/graph_submit 9.29 |
| warm-up | 4.16 | 31.3 | pipelines/compile_ms 1280, pipelines/front_ms 1242, render/thread/ms 60.54, render/busy/ms 58.00, main/busy/ms 56.02, stage/main/update 38.37, pipelines/driver_ms 37.75, stage/render/prepare_assets 28.61, stage/render/render 18.75, render/graph/ms 15.47, stage/render/graph_submit 10.00, stage/main/post_update 9.43 |
| warm-up | 4.25 | 87.2 | pipelines/compile_ms 108, pipelines/front_ms 103, render/thread/ms 67.35, render/busy/ms 65.73, main/busy/ms 56.68, stage/render/prepare_assets 46.47, stage/main/update 38.79, stage/main/post_update 10.19, stage/render/render 10.01, render/graph/ms 7.82, encode//sum 5.35, pipelines/driver_ms 5.32 |
| warm-up | 4.28 | 29.5 | render/thread/ms 69.98, render/busy/ms 68.73, stage/render/prepare_assets 45.91, main/busy/ms 44.69, pipelines/compile_ms 43.23, pipelines/front_ms 41.13, stage/main/update 23.42, stage/main/post_update 13.51, stage/render/render 9.88, render/graph/ms 8.19, encode//sum 6.25, stage/render/graph_submit 5.78 |
| warm-up | 4.34 | 55.0 | pipelines/compile_ms 846, pipelines/front_ms 810, render/thread/ms 66.41, render/busy/ms 64.78, main/busy/ms 48.02, pipelines/driver_ms 35.60, stage/render/prepare_assets 32.68, stage/main/update 26.80, stage/render/render 12.50, stage/main/post_update 12.45, render/graph/ms 10.49, stage/render/graph_submit 7.50 |
| warm-up | 4.41 | 71.3 | pipelines/compile_ms 1548, pipelines/front_ms 1485, pipelines/wait_ms 98.20, render/thread/ms 87.58, render/busy/ms 84.42, pipelines/driver_ms 62.21, main/busy/ms 45.26, stage/render/prepare_assets 34.53, stage/main/update 27.27, stage/render/prepare_write_phase_buffers 13.97, stage/render/extract_commands 12.45, stage/render/render 11.97 |
| warm-up | 4.49 | 78.1 | pipelines/compile_ms 1249, pipelines/front_ms 1202, pipelines/wait_ms 182, render/thread/ms 104, render/busy/ms 101, main/busy/ms 71.60, stage/main/update 55.32, pipelines/driver_ms 46.13, stream/drive/ms 36.45, stage/render/prepare_assets 30.13, stage/render/prepare_resources 23.82, stage/render/prepare_write_phase_buffers 20.19 |
| warm-up | 4.58 | 91.8 | pipelines/compile_ms 2598, pipelines/front_ms 2502, pipelines/wait_ms 245, render/thread/ms 135, render/busy/ms 134, main/busy/ms 128, stage/main/update 96.17, pipelines/driver_ms 95.88, stream/drive/ms 53.36, stage/render/prepare_resources 36.07, stage/render/prepare_assets 35.51, stage/render/prepare_batch_phases 18.72 |
| warm-up | 4.65 | 72.7 | pipelines/compile_ms 2479, pipelines/front_ms 2384, pipelines/wait_ms 232, main/busy/ms 150, render/thread/ms 130, render/busy/ms 128, stage/main/update 110, pipelines/driver_ms 95.21, stage/render/prepare_assets 42.98, stream/drive/ms 30.35, stage/render/render 25.75, render/graph/ms 22.73 |
| warm-up | 4.73 | 84.2 | pipelines/compile_ms 1436, pipelines/front_ms 1388, pipelines/wait_ms 284, main/busy/ms 106, render/thread/ms 102, render/busy/ms 99.77, stage/main/update 77.02, stage/render/prepare_assets 65.07, pipelines/driver_ms 48.39, stage/render/render 22.42, render/graph/ms 19.79, stage/main/post_update 16.99 |
| warm-up | 4.80 | 63.7 | pipelines/compile_ms 1612, pipelines/front_ms 1559, pipelines/wait_ms 216, render/thread/ms 101, render/busy/ms 97.65, main/busy/ms 63.47, pipelines/driver_ms 52.86, stage/render/prepare_assets 51.46, stage/main/update 36.05, stage/render/render 24.92, render/graph/ms 21.45, stage/render/graph_submit 18.49 |
| warm-up | 4.83 | 35.3 | pipelines/compile_ms 981, pipelines/front_ms 945, render/thread/ms 78.40, render/busy/ms 75.36, main/busy/ms 49.38, pipelines/driver_ms 35.62, stage/render/prepare_assets 29.32, stage/main/update 28.15, stage/render/render 23.40, render/graph/ms 20.16, stage/render/graph_submit 17.59, encode/submit 16.63 |
| warm-up | 4.89 | 32.3 | render/thread/ms 44.97, main/busy/ms 44.48, render/busy/ms 43.53, stage/main/update 30.12, stream/drive/ms 14.13, pipelines/compile_ms 13.93, pipelines/front_ms 12.99, stage/render/render 12.87, render/graph/ms 10.85, stage/render/prepare_assets 9.17, stage/main/post_update 8.12, stage/render/graph_submit 7.56 |
| warm-up | 5.00 | 26.4 | pipelines/compile_ms 397, pipelines/front_ms 386, render/thread/ms 38.24, render/busy/ms 36.65, main/busy/ms 35.15, stage/main/update 21.91, stage/render/render 12.88, stage/render/prepare_assets 11.47, render/graph/ms 11.02, pipelines/driver_ms 10.93, stage/render/graph_submit 8.20, stage/main/post_update 7.80 |
| warm-up | 5.03 | 28.4 | pipelines/compile_ms 224, pipelines/front_ms 217, render/thread/ms 44.90, render/busy/ms 43.35, main/busy/ms 39.02, stage/main/update 17.67, stage/render/render 16.44, render/graph/ms 14.36, stage/main/post_update 12.75, stage/render/graph_submit 11.28, stage/render/prepare_assets 7.54, pipelines/driver_ms 6.95 |
| warm-up | 5.08 | 54.3 | pipelines/compile_ms 608, pipelines/front_ms 584, render/thread/ms 75.98, render/busy/ms 74.48, main/busy/ms 51.12, stage/main/update 30.70, pipelines/driver_ms 23.67, stage/render/render 22.05, render/graph/ms 19.94, stage/render/prepare_assets 16.96, stage/render/graph_submit 16.69, encode//sum 13.37 |
| warm-up | 5.12 | 41.0 | pipelines/compile_ms 1007, pipelines/front_ms 969, render/thread/ms 85.68, render/busy/ms 84.25, main/busy/ms 43.92, pipelines/driver_ms 38.10, stage/main/update 23.04, stage/render/prepare_assets 21.78, pipelines/wait_ms 21.38, stage/render/render 18.44, render/graph/ms 16.58, encode//sum 14.29 |
| warm-up | 5.15 | 29.5 | pipelines/compile_ms 455, pipelines/front_ms 438, render/thread/ms 63.23, render/busy/ms 61.89, main/busy/ms 39.24, pipelines/wait_ms 21.09, pipelines/driver_ms 17.36, stage/main/update 13.78, stage/render/render 13.04, stage/render/prepare_assets 12.79, stage/render/prepare_resources 11.46, stage/render/prepare_flush 11.45 |
| warm-up | 5.23 | 31.8 | pipelines/compile_ms 981, pipelines/front_ms 952, render/thread/ms 39.97, render/busy/ms 38.30, main/busy/ms 37.42, pipelines/driver_ms 28.87, stage/main/update 21.54, stage/render/render 16.06, render/graph/ms 13.80, stage/main/post_update 10.51, stage/render/graph_submit 9.57, stage/render/prepare_assets 9.07 |
| warm-up | 5.25 | 25.8 | pipelines/compile_ms 598, pipelines/front_ms 580, pipelines/wait_ms 68.84, render/thread/ms 43.57, render/busy/ms 42.18, main/busy/ms 32.23, pipelines/driver_ms 18.25, stage/render/prepare_assets 15.26, stage/main/update 14.62, stage/render/render 12.13, stage/main/post_update 10.22, render/graph/ms 10.14 |
| warm-up | 5.28 | 33.8 | pipelines/compile_ms 587, pipelines/front_ms 572, pipelines/wait_ms 68.41, render/thread/ms 42.45, render/busy/ms 41.56, main/busy/ms 23.52, stage/render/prepare_assets 19.49, pipelines/driver_ms 15.17, stage/main/update 9.05, stage/render/render 8.28, encode//sum 7.02, render/graph/ms 6.94 |
| warm-up | 5.48 | 57.0 | render/thread/ms 60.90, render/busy/ms 59.09, main/busy/ms 41.23, stage/render/prepare_assets 29.95, stage/main/update 24.16, stage/render/render 15.30, render/graph/ms 12.73, stage/main/post_update 10.69, stage/render/graph_submit 9.05, encode//sum 8.66, stream/drive/ms 6.88, encode/submit 6.53 |
| warm-up | 5.55 | 44.0 | pipelines/compile_ms 163, pipelines/front_ms 159, render/thread/ms 46.07, render/busy/ms 44.94, main/busy/ms 27.45, stage/render/prepare_assets 23.57, stage/main/update 15.61, stage/render/render 8.75, encode//sum 7.35, render/graph/ms 7.27, stage/main/post_update 6.98, stage/render/graph_submit 5.08 |
| warm-up | 5.80 | 39.7 | pipelines/compile_ms 299, pipelines/front_ms 288, main/busy/ms 48.04, stage/main/update 33.12, render/thread/ms 32.35, render/busy/ms 30.99, stream/drive/ms 13.64, stage/main/post_update 11.01, stage/render/render 10.98, pipelines/driver_ms 10.46, stage/render/prepare_assets 9.26, render/graph/ms 9.15 |
| warm-up | 6.25 | 27.4 | render/thread/ms 37.18, render/busy/ms 35.33, main/busy/ms 34.57, stage/main/update 16.81, stage/render/render 16.17, stage/main/post_update 13.81, render/graph/ms 13.55, stage/render/graph_submit 10.14, encode//sum 7.92, encode/submit 7.23, stage/render/prepare_assets 4.97, stage/render/prepare_meshes 4.55 |
| warm-up | 6.28 | 30.1 | render/thread/ms 46.36, render/busy/ms 44.47, main/busy/ms 38.27, stage/main/update 16.52, stage/main/post_update 15.36, stage/render/render 14.35, render/graph/ms 12.01, stage/render/prepare_assets 11.22, stage/render/graph_submit 8.69, stage/render/prepare_meshes 8.59, encode//sum 8.33, encode/submit 6.27 |
| warm-up | 6.31 | 32.7 | pipelines/compile_ms 166, pipelines/front_ms 163, render/thread/ms 42.43, render/busy/ms 41.33, main/busy/ms 27.54, stage/render/prepare_assets 21.07, stage/main/post_update 11.03, stage/main/update 9.93, stage/render/render 8.23, encode//sum 7.35, render/graph/ms 6.67, stage/render/prepare_meshes 4.54 |
| warm-up | 6.37 | 26.5 | main/busy/ms 36.56, render/thread/ms 35.15, render/busy/ms 34.04, pipelines/compile_ms 26.88, pipelines/front_ms 25.64, stage/main/update 21.80, stage/render/render 15.67, render/graph/ms 13.78, stage/main/post_update 11.34, stage/render/graph_submit 9.92, encode//sum 9.89, stage/render/prepare_assets 8.10 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=scene-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 5%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 94%) · 2616 MHz (min 2610) · memory 8101 MHz · 97.8 W · 80 °C (max 83) · VRAM 6441 MiB · throttled 100% of samples

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
