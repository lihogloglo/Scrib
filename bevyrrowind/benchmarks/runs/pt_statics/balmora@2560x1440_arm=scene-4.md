# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1579 frames over 15.0 s · **median 9.49 ms (105 fps)** · mean 9.50 · p95 10.84 · p99 11.46 (1% low 87 fps) · worst 17.64 ms · 4 spikes over 1.5× median · 16.0 stutters/min (work 12.0/min)
- warm-up: 311 frames over 8.6 s, worst 2481.6 ms
- hitches (> 25.0 ms): **0** measured, 37 in warm-up
- timed GPU passes: 8.27 ms of a 9.50 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.49 ms, limit 11 ms
- pass worst frame 17.64 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.513 | 2.525 | 2.575 | 0.99 | 0.078 |
| dlss_super_resolution | 2.010 | 2.020 | 2.043 | 0.99 | 0.180 |
| early prepass | 0.646 | 0.649 | 0.657 | 0.99 | 0.098 |
| atmosphere_luts | 0.324 | 0.325 | 0.341 | 0.99 | 0.003 |
| bloom | 0.313 | 0.315 | 0.331 | 0.99 | 0.060 |
| main_transmissive_pass_3d | 0.308 | 0.310 | 0.331 | 0.99 | 0.008 |
| water_velocity | 0.237 | 0.238 | 0.263 | 0.99 | 0.002 |
| water_sspr | 0.206 | 0.207 | 0.210 | 0.99 | 0.002 |
| ssao | 0.190 | 0.191 | 0.197 | 0.99 | 0.007 |
| clouds_trace | 0.148 | 0.148 | 0.168 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.147 | 0.153 | 0.99 | 0.001 |
| water_fft | 0.132 | 0.133 | 0.137 | 0.99 | 0.003 |
| uniform_allocation | 0.115 | 0.116 | 0.147 | 0.99 | 0.021 |
| upscaling | 0.113 | 0.114 | 0.120 | 0.99 | 0.004 |
| clouds_composite | 0.111 | 0.112 | 0.113 | 0.99 | 0.002 |
| bin_unpacking | 0.090 | 0.091 | 0.114 | 0.99 | 0.015 |
| tonemapping | 0.085 | 0.086 | 0.089 | 0.99 | 0.003 |
| clustering | 0.064 | 0.064 | 0.075 | 0.99 | 0.020 |
| gpu_instance_materialization | 0.062 | 0.063 | 0.089 | 0.99 | 0.004 |
| early_mesh_preprocessing | 0.059 | 0.060 | 0.083 | 0.99 | 0.004 |
| render_sky | 0.052 | 0.052 | 0.052 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.412 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 0.99 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.221 | 0.233 | 0.12 | 0.004 |
| range_unpacking | 0.024 | 0.024 | 0.030 | 0.99 | 0.003 |
| water_snapshot | 0.023 | 0.023 | 0.025 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.018 | 0.018 | 0.037 | 0.99 | 0.006 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| surface_exposure_far | 0.012 | 0.040 | 0.041 | 0.29 | 0.002 |
| surface_exposure | 0.011 | 0.040 | 0.041 | 0.27 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.99 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.005 | 0.005 | 0.017 | 0.99 | 0.045 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.600 | 9.972 | 16.74 | 13580.1 |
| render/busy/ms | 7.254 | 8.389 | 16.17 | 11453.8 |
| main/busy/ms | 7.198 | 8.067 | 11.38 | 11365.9 |
| stage/render/render | 4.752 | 5.602 | 12.34 | 7503.6 |
| render/graph/ms | 4.185 | 5.010 | 10.61 | 6608.6 |
| encode//sum | 4.128 | 5.444 | 12.17 | 6518.2 |
| stage/main/post_update | 4.060 | 4.811 | 8.33 | 6410.1 |
| stage/render/graph_submit | 3.179 | 3.967 | 9.40 | 5020.3 |
| encode/submit | 1.985 | 2.401 | 7.62 | 3133.7 |
| stage/main/update | 1.863 | 2.289 | 2.83 | 2942.0 |
| encode/finish_wall | 1.222 | 1.745 | 6.33 | 1928.8 |
| stage/render/prepare_views | 1.043 | 2.503 | 3.89 | 1647.3 |
| pipelines/compile_ms | 1.039 | 0.000 | 147.30 | 1640.9 |
| pipelines/front_ms | 1.012 | 0.000 | 143.94 | 1597.7 |
| render/acquire/ms | 0.998 | 2.458 | 3.84 | 1575.1 |
| stage/render/camera_driver | 0.945 | 1.073 | 1.28 | 1491.8 |
| encode/pass:shadow_directional_light/sum | 0.793 | 1.396 | 4.08 | 1251.7 |
| encode/pass:early prepass/sum | 0.745 | 1.030 | 5.53 | 1176.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.738 | 1.031 | 1.42 | 1165.9 |
| stage/render/prepare_write_phase_buffers | 0.530 | 0.740 | 1.04 | 837.6 |
| stage/render/prepare_assets | 0.445 | 0.578 | 1.65 | 703.2 |
| stage/render/prepare_resources | 0.386 | 0.447 | 0.59 | 609.0 |
| bigspace/total/ms | 0.383 | 1.010 | 2.01 | 605.3 |
| stage/main/pre_update | 0.353 | 0.495 | 0.83 | 557.2 |
| render/present/ms | 0.349 | 0.413 | 5.11 | 551.2 |
| stage/main/first | 0.299 | 0.367 | 1.63 | 472.5 |
| stage/render/prepare_flush | 0.266 | 0.367 | 0.72 | 419.4 |
| stage/render/prepare_bind_groups | 0.263 | 0.363 | 0.51 | 414.8 |
| stage/main/last | 0.251 | 0.307 | 1.05 | 397.0 |
| terrain/update/ms | 0.232 | 0.297 | 0.53 | 366.0 |
| stage/render/prepare_meshes | 0.215 | 0.279 | 1.23 | 339.2 |
| stage/render/prepare_batch_phases | 0.206 | 0.297 | 0.47 | 325.5 |
| terrain/select/ms | 0.202 | 0.260 | 0.50 | 318.9 |
| stream/drive/ms | 0.189 | 0.263 | 0.71 | 297.9 |
| stage/main/fixed_main_loop | 0.186 | 0.379 | 1.04 | 294.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.185 | 0.270 | 0.47 | 291.5 |
| stage/render/extract_commands | 0.179 | 0.219 | 0.56 | 283.4 |
| far/select/ms | 0.175 | 0.240 | 0.63 | 275.6 |
| stage/render/create_views | 0.163 | 0.212 | 0.61 | 256.9 |
| stream/tick/ms | 0.160 | 0.228 | 0.66 | 252.7 |
| controller/feed/ms | 0.153 | 0.202 | 0.54 | 241.9 |
| bigspace/high_precision/ms | 0.148 | 0.594 | 1.46 | 234.2 |
| controller/move/ms | 0.146 | 0.229 | 0.76 | 231.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.123 | 0.189 | 0.25 | 193.5 |
| bigspace/low_precision/ms | 0.103 | 0.237 | 1.57 | 162.3 |
| stage/render/specialize | 0.097 | 0.135 | 3.45 | 153.2 |
| anim/evaluate/ms | 0.089 | 0.239 | 1.03 | 140.8 |
| anim/evaluate_cpu/ms | 0.061 | 0.076 | 0.13 | 96.0 |
| encode/resolve_diagnostics/sum | 0.060 | 0.092 | 0.27 | 95.0 |
| stage/render/queue | 0.058 | 0.079 | 2.28 | 91.0 |
| stage/render/phase_sort | 0.055 | 0.075 | 0.19 | 87.3 |
| stage/render/prepare_collect_phase_buffers | 0.044 | 0.071 | 0.22 | 68.9 |
| physics/step/ms | 0.029 | 0.059 | 0.46 | 46.2 |
| pipelines/driver_ms | 0.027 | 0.000 | 4.54 | 43.2 |
| weather/map/ms | 0.026 | 0.035 | 0.43 | 41.1 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.039 | 0.37 | 40.5 |
| encode/camera_driver_span_end/sum | 0.023 | 0.037 | 0.30 | 36.1 |
| stage/render/cleanup | 0.022 | 0.031 | 0.50 | 35.0 |
| weather/eval/ms | 0.011 | 0.015 | 0.07 | 17.6 |
| script/update/ms | 0.008 | 0.010 | 0.37 | 11.9 |
| controller/avoid/ms | 0.006 | 0.008 | 0.05 | 9.7 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 8.7 |
| anim/apply/ms | 0.005 | 0.006 | 0.05 | 7.3 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 6.5 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 1.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.04 | 0.4 |
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
| encode//max | ms | 0.57 | 6.21 | 0.70 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.37 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.30 | 0.02 |
| encode/finish_critical | ms | 0.88 | 6.21 | 1.24 |
| encode/pass:early prepass/max | ms | 0.73 | 5.53 | 0.73 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.74 | 1.42 | 0.50 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.25 | 0.14 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.18 | 0.47 | 0.21 |
| encode/pass:shadow_directional_light/max | ms | 0.72 | 4.08 | 1.24 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.27 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 8.94 | 16.17 | 8.92 |
| gpu/timed/ms | ms | 8.94 | 10.57 | 8.92 |
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
| phases/opaque3d/bins | count | 878 | 878 | 878 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/opaque3d_prepass/bins | count | 878 | 878 | 878 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 4 | 41 | 34 |
| phases/shadow/bins | count | 896 | 1509 | 896 |
| phases/shadow/instance_batches | count | 4 | 12 | 12 |
| phases/shadow/instance_draws | count | 5861 | 17.6 k | 17.6 k |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 147.5 k |
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
| preprocess/bind_groups | count | 5 | 69 | 53 |
| preprocess/bind_groups_created | count | 0 | 62 | 46 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 7 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 27.88 | 39.90 | 18.27 |
| process/mem_usage | GiB | 4.64 | 4.72 | 4.72 |
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
| skin/blocks_changed/count | count | 6 | 7 | 4 |
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
| system/cpu_usage | % | 32.28 | 50.19 | 29.57 |
| system/mem_usage | % | 54.24 | 54.50 | 54.50 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2552 | 2552 | 2552 |
| textures/images | count | 1998 | 1998 | 1998 |
| views/all/batch_sets | count | 64 | 101 | 94 |
| views/all/bins | count | 3150 | 3763 | 3150 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 12 | 20 | 20 |
| views/all/instance_draws | count | 17.6 k | 29.3 k | 29.3 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 245.8 k |
| views/all/items | count | 9462 | 14.4 k | 9462 |
| views/all/pipelines | count | 75 | 82 | 75 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 153.8 k | 260.3 k | 255.3 k |
| views/cascade/batch_sets | count | 4 | 41 | 34 |
| views/cascade/bins | count | 896 | 1509 | 896 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 12 |
| views/cascade/instance_draws | count | 5861 | 17.6 k | 17.6 k |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 3202 | 8161 | 3202 |
| views/cascade/pipelines | count | 27 | 34 | 27 |
| views/cascade/work_items | count | 49.2 k | 155.6 k | 150.6 k |
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
| views/main/bins | count | 2254 | 2254 | 2254 |
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
| anim/events/count | count | 0.10 | 3 | 152 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.13 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 27.55 | 30 | 43.5 k |
| encode/bind_walk/init_textures | count | 7096 | 13.9 k | 11.20 M |
| encode/bind_walk/init_textures_skipped | count | 35.7 k | 50.1 k | 56.38 M |
| encode/bind_walk/inits | count | 330 | 435 | 520.9 k |
| encode/bind_walk/inits_skipped | count | 47.50 | 61 | 75.0 k |
| encode/bind_walk/merge_textures | count | 7032 | 13.8 k | 11.10 M |
| encode/bind_walk/merge_textures_skipped | count | 3925 | 3925 | 6.20 M |
| encode/bind_walk/merges | count | 79.26 | 100 | 125.2 k |
| encode/bind_walk/merges_skipped | count | 31.00 | 37 | 49.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1579 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1579 |
| encode/pass:early prepass/count | count | 1 | 1 | 1579 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1579 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1579 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1579 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1579 |
| encode/queue/command_buffers | count | 37.55 | 40 | 59.3 k |
| encode/queue/submits | count | 2 | 2 | 3158 |
| encode/queue/write_buffer/bytes | bytes | 2.21 M | 3.62 M | 3.49 G |
| encode/queue/write_buffer/calls | count | 229 | 302 | 362.1 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 51.54 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 14.2 k |
| encode/queue/write_texture/bytes | bytes | 11.4 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1579 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.57 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.01 | 1 | 23 |
| pipelines/created | count | 0.01 | 1 | 23 |
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

## Hitches (> 25.0 ms; slowest 37, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.48 | 2481.6 | main/busy/ms 2452, stage/main/post_update 2347, stage/main/update 96.31, surface/advance_cpu_ms 67.54, stage/main/pre_update 4.63, stage/main/last 1.05, surface/cell_upload_ms 0.79, stage/main/first 0.53, stream/drive/ms 0.45, stage/main/fixed_main_loop 0.42, weather/map/ms 0.16, stream/tick/ms 0.11 |
| warm-up | 3.22 | 742.3 | main/busy/ms 2590, stage/main/post_update 2369, render/thread/ms 711, render/busy/ms 533, stage/render/prepare_assets 485, stage/main/update 206, stage/render/prepare_views 173, render/acquire/ms 173, surface/advance_cpu_ms 67.54, stage/render/create_views 28.15, stage/render/render 14.72, render/graph/ms 8.20 |
| warm-up | 3.40 | 171.7 | render/thread/ms 867, render/busy/ms 595, stage/render/prepare_assets 533, stage/render/prepare_views 174, render/acquire/ms 173, main/busy/ms 166, stage/main/update 116, stage/render/render 112, render/present/ms 97.97, stage/main/post_update 36.05, stage/render/create_views 28.64, pipelines/compile_ms 24.25 |
| warm-up | 3.53 | 118.7 | pipelines/wait_ms 3978, pipelines/compile_ms 1630, pipelines/front_ms 1388, pipelines/driver_ms 242, main/busy/ms 164, render/thread/ms 125, render/busy/ms 122, stage/render/prepare_assets 60.78, stage/main/pre_update 60.40, stage/main/update 55.38, stage/main/post_update 42.31, stage/render/prepare_resources 24.94 |
| warm-up | 3.64 | 108.2 | pipelines/wait_ms 5029, pipelines/compile_ms 2264, pipelines/front_ms 2124, render/thread/ms 186, render/busy/ms 184, main/busy/ms 178, pipelines/driver_ms 139, stage/render/prepare_assets 121, stage/main/update 70.35, stage/main/pre_update 60.60, stage/main/post_update 43.62, stage/render/prepare_resources 24.01 |
| warm-up | 3.68 | 34.2 | pipelines/wait_ms 1205, pipelines/compile_ms 1192, pipelines/front_ms 1139, render/thread/ms 105, render/busy/ms 103, stage/render/prepare_assets 84.29, pipelines/driver_ms 52.87, main/busy/ms 51.47, stage/main/update 28.54, stage/main/post_update 17.13, terrain/update/ms 14.89, stage/render/render 10.82 |
| warm-up | 3.70 | 25.2 | pipelines/compile_ms 780, pipelines/front_ms 739, pipelines/wait_ms 190, render/thread/ms 54.59, render/busy/ms 53.05, pipelines/driver_ms 40.53, stage/render/prepare_assets 37.09, main/busy/ms 31.93, stage/main/post_update 15.31, stage/main/update 11.75, terrain/update/ms 10.19, stage/render/render 8.47 |
| warm-up | 3.81 | 44.3 | pipelines/compile_ms 491, pipelines/front_ms 472, main/busy/ms 98.31, stage/main/update 72.69, render/thread/ms 64.84, render/busy/ms 62.64, stage/render/prepare_assets 31.43, pipelines/wait_ms 28.56, pipelines/driver_ms 18.94, stage/render/render 15.93, stage/main/post_update 15.59, render/graph/ms 13.32 |
| warm-up | 3.96 | 150.5 | pipelines/compile_ms 2161, pipelines/front_ms 2060, pipelines/wait_ms 173, render/thread/ms 109, main/busy/ms 107, render/busy/ms 107, pipelines/driver_ms 101, stage/main/update 88.02, stage/render/prepare_assets 83.48, stage/render/render 13.73, stage/main/post_update 11.64, render/graph/ms 11.16 |
| warm-up | 4.05 | 96.9 | pipelines/compile_ms 2265, pipelines/front_ms 2161, pipelines/wait_ms 180, render/thread/ms 133, render/busy/ms 131, stage/render/prepare_assets 116, pipelines/driver_ms 104, main/busy/ms 34.37, pipelines/lock_ms 26.65, stage/main/update 20.23, stage/render/render 11.35, render/graph/ms 9.42 |
| warm-up | 4.10 | 47.2 | pipelines/compile_ms 468, pipelines/front_ms 454, render/thread/ms 107, render/busy/ms 106, stage/render/prepare_assets 75.76, pipelines/wait_ms 27.21, stage/render/render 26.76, pipelines/lock_ms 26.36, render/graph/ms 25.20, main/busy/ms 25.05, stage/render/camera_driver 18.21, pipelines/driver_ms 14.62 |
| warm-up | 4.17 | 67.8 | pipelines/compile_ms 746, pipelines/front_ms 726, render/thread/ms 108, render/busy/ms 73.52, stage/render/render 52.47, render/graph/ms 49.69, stage/render/prepare_views 32.34, render/acquire/ms 32.22, encode//sum 31.59, stage/render/graph_submit 31.31, main/busy/ms 28.18, encode/submit 26.49 |
| warm-up | 4.32 | 106.4 | pipelines/compile_ms 309, pipelines/front_ms 301, main/busy/ms 91.60, stage/main/update 81.82, render/thread/ms 41.65, render/busy/ms 33.78, stage/render/prepare_assets 21.13, stream/drive/ms 13.90, stage/render/render 9.45, render/graph/ms 7.82, pipelines/driver_ms 7.54, stage/render/prepare_views 6.89 |
| warm-up | 4.35 | 26.4 | pipelines/compile_ms 122, pipelines/front_ms 117, main/busy/ms 101, stage/main/update 76.45, render/thread/ms 46.45, render/busy/ms 45.11, stream/drive/ms 44.14, stage/render/prepare_assets 24.38, stage/main/post_update 14.43, stage/render/render 14.24, render/graph/ms 12.51, stage/render/graph_submit 8.33 |
| warm-up | 4.48 | 129.9 | pipelines/compile_ms 360, pipelines/front_ms 346, main/busy/ms 122, stage/main/update 93.28, render/thread/ms 64.38, render/busy/ms 62.88, stream/drive/ms 50.64, stage/render/prepare_assets 31.95, stage/main/post_update 17.13, stage/render/render 16.47, render/graph/ms 14.60, pipelines/driver_ms 14.19 |
| warm-up | 4.53 | 50.2 | pipelines/compile_ms 520, pipelines/front_ms 501, render/thread/ms 74.34, render/busy/ms 72.95, main/busy/ms 52.03, stage/render/prepare_assets 47.67, stage/main/update 33.96, pipelines/driver_ms 18.40, stream/drive/ms 11.10, stage/render/render 10.65, stage/main/post_update 9.82, pipelines/wait_ms 9.35 |
| warm-up | 4.56 | 35.4 | pipelines/compile_ms 883, pipelines/front_ms 859, render/thread/ms 55.54, render/busy/ms 54.21, main/busy/ms 47.42, stage/render/prepare_assets 37.23, stage/main/update 28.25, pipelines/driver_ms 24.38, stage/main/post_update 10.23, stage/render/render 8.71, render/graph/ms 6.95, pipelines/wait_ms 6.84 |
| warm-up | 4.62 | 61.3 | pipelines/compile_ms 661, pipelines/front_ms 643, render/thread/ms 67.48, render/busy/ms 65.51, main/busy/ms 56.15, stage/render/prepare_assets 39.52, stage/main/update 38.37, pipelines/driver_ms 18.28, stage/render/render 10.93, stage/main/post_update 10.27, render/graph/ms 8.47, encode//sum 6.26 |
| warm-up | 4.68 | 51.4 | pipelines/compile_ms 532, pipelines/front_ms 511, render/thread/ms 81.37, render/busy/ms 79.52, main/busy/ms 67.42, stage/render/prepare_assets 46.91, pipelines/wait_ms 36.94, stage/main/post_update 33.71, stage/main/update 27.27, pipelines/driver_ms 20.63, stage/render/render 12.21, render/graph/ms 9.76 |
| warm-up | 4.75 | 74.4 | pipelines/compile_ms 1107, pipelines/front_ms 1068, pipelines/wait_ms 248, render/thread/ms 104, render/busy/ms 103, main/busy/ms 52.16, pipelines/driver_ms 38.63, stage/main/post_update 33.52, stage/render/prepare_assets 29.50, stage/render/prepare_flush 22.32, stage/render/prepare_write_phase_buffers 17.90, stage/render/prepare_resources 17.63 |
| warm-up | 4.80 | 46.0 | pipelines/compile_ms 1857, pipelines/front_ms 1809, pipelines/wait_ms 219, render/thread/ms 111, render/busy/ms 110, main/busy/ms 51.40, pipelines/driver_ms 47.73, stage/render/prepare_write_phase_buffers 30.47, stage/render/prepare_resources 23.84, stage/render/prepare_flush 23.01, stage/render/prepare_assets 19.55, stage/main/post_update 19.39 |
| warm-up | 4.89 | 88.8 | pipelines/compile_ms 1966, pipelines/front_ms 1918, pipelines/wait_ms 191, render/thread/ms 124, render/busy/ms 122, main/busy/ms 75.81, stage/render/prepare_assets 49.19, pipelines/driver_ms 48.22, stage/main/post_update 35.68, stage/main/update 21.50, stage/render/prepare_resources 19.89, stage/render/prepare_write_phase_buffers 15.81 |
| warm-up | 4.96 | 75.4 | pipelines/compile_ms 2058, pipelines/front_ms 2002, pipelines/wait_ms 941, render/thread/ms 145, render/busy/ms 143, main/busy/ms 96.64, stage/main/update 61.69, stage/render/prepare_assets 57.87, pipelines/driver_ms 56.05, stage/render/prepare_resources 40.14, stage/main/post_update 24.48, stage/render/prepare_flush 15.40 |
| warm-up | 5.08 | 120.7 | pipelines/compile_ms 2993, pipelines/front_ms 2924, pipelines/wait_ms 1332, render/thread/ms 147, render/busy/ms 146, main/busy/ms 75.43, pipelines/driver_ms 68.90, stage/render/prepare_assets 60.61, stage/main/update 57.36, stage/render/prepare_resources 28.65, stage/render/prepare_bind_groups 26.29, stream/drive/ms 15.69 |
| warm-up | 5.13 | 45.3 | pipelines/compile_ms 1867, pipelines/front_ms 1809, pipelines/wait_ms 584, render/thread/ms 125, render/busy/ms 122, pipelines/driver_ms 58.06, main/busy/ms 52.81, stage/render/prepare_assets 40.43, stage/render/render 38.04, stage/main/update 35.95, render/graph/ms 34.52, stage/render/graph_submit 30.90 |
| warm-up | 5.23 | 37.1 | pipelines/compile_ms 124, pipelines/front_ms 120, render/thread/ms 54.41, render/busy/ms 53.02, main/busy/ms 36.61, stage/render/prepare_assets 24.71, stage/main/update 19.80, stage/main/post_update 11.83, stage/render/render 11.35, render/graph/ms 9.64, encode//sum 7.40, stage/render/graph_submit 6.84 |
| warm-up | 5.26 | 26.0 | pipelines/compile_ms 479, pipelines/front_ms 463, render/thread/ms 56.59, render/busy/ms 54.93, main/busy/ms 44.83, stage/main/update 28.06, stage/render/prepare_assets 24.50, pipelines/driver_ms 16.18, stage/render/render 13.00, stage/main/post_update 11.02, render/graph/ms 10.83, encode//sum 9.06 |
| warm-up | 5.34 | 28.9 | pipelines/compile_ms 84.40, pipelines/front_ms 80.01, render/thread/ms 48.11, render/busy/ms 46.61, main/busy/ms 34.66, stage/render/render 19.61, render/graph/ms 17.66, stage/render/graph_submit 14.01, stage/main/update 9.87, encode//sum 9.82, stage/main/post_update 8.71, stage/main/pre_update 6.50 |
| warm-up | 5.41 | 32.1 | pipelines/compile_ms 384, pipelines/front_ms 376, render/thread/ms 48.27, render/busy/ms 46.49, main/busy/ms 45.70, stage/main/update 25.95, stage/render/render 16.92, render/graph/ms 14.74, stage/main/post_update 14.38, stage/render/graph_submit 10.38, encode//sum 8.28, pipelines/driver_ms 8.11 |
| warm-up | 5.45 | 34.8 | pipelines/compile_ms 195, pipelines/front_ms 191, render/thread/ms 53.64, render/busy/ms 51.88, main/busy/ms 48.11, stage/main/update 27.18, stage/render/render 15.43, stage/main/post_update 14.24, stage/render/prepare_assets 13.87, render/graph/ms 13.24, stage/render/graph_submit 9.48, encode//sum 9.28 |
| warm-up | 5.48 | 34.3 | pipelines/compile_ms 600, pipelines/front_ms 587, render/thread/ms 47.31, render/busy/ms 46.07, main/busy/ms 34.56, stage/render/prepare_assets 21.32, stage/main/update 17.27, pipelines/driver_ms 13.06, stage/render/render 9.99, stage/main/post_update 9.62, render/graph/ms 8.28, encode//sum 8.17 |
| warm-up | 5.63 | 27.7 | render/thread/ms 40.44, render/busy/ms 39.05, main/busy/ms 38.60, stage/main/update 21.42, stage/render/render 12.67, stage/render/prepare_assets 12.13, render/graph/ms 10.67, stage/main/post_update 10.45, encode//sum 9.23, stage/render/graph_submit 7.33, stage/render/prepare_meshes 4.88, encode/submit 4.28 |
| warm-up | 5.69 | 52.5 | render/thread/ms 56.27, render/busy/ms 54.89, main/busy/ms 45.56, stage/render/prepare_assets 30.61, stage/main/update 24.49, stage/render/render 12.05, stage/main/post_update 11.63, render/graph/ms 10.05, encode//sum 9.21, stage/render/graph_submit 7.31, stream/drive/ms 5.56, stage/main/pre_update 4.58 |
| warm-up | 5.73 | 41.1 | render/thread/ms 59.68, render/busy/ms 58.64, stage/render/prepare_assets 40.03, main/busy/ms 38.56, stage/main/update 22.30, stage/main/post_update 9.96, stage/render/render 9.04, render/graph/ms 7.62, encode//sum 6.34, stage/render/graph_submit 5.70, stream/drive/ms 4.32, encode/submit 3.78 |
| warm-up | 5.85 | 26.3 | render/thread/ms 36.21, render/busy/ms 35.05, main/busy/ms 32.27, stage/main/update 18.21, stage/render/render 11.86, render/graph/ms 10.31, stage/main/post_update 9.14, encode//sum 8.16, stage/render/graph_submit 6.92, stage/render/prepare_meshes 6.63, stage/render/prepare_assets 5.07, encode/submit 4.07 |
| warm-up | 6.44 | 26.2 | render/thread/ms 42.13, render/busy/ms 40.19, main/busy/ms 32.35, stage/main/post_update 15.07, stage/render/render 14.03, stage/main/update 11.93, render/graph/ms 11.60, stage/render/prepare_assets 9.17, encode//sum 8.81, stage/render/prepare_meshes 8.67, stage/render/graph_submit 8.25, encode/submit 5.58 |
| warm-up | 6.47 | 31.3 | render/thread/ms 40.24, render/busy/ms 39.14, main/busy/ms 26.59, stage/render/prepare_assets 19.50, stage/main/post_update 11.71, stage/main/update 9.14, stage/render/render 8.16, encode//sum 6.84, render/graph/ms 6.40, stage/render/prepare_meshes 4.53, stage/render/graph_submit 4.28, encode/submit 2.26 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=scene-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 15%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **97%** (min 92%) · 2625 MHz (min 2625) · memory 8101 MHz · 97.5 W · 79 °C (max 82) · VRAM 6439 MiB · throttled 100% of samples

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
