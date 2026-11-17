# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1571 frames over 15.0 s · **median 9.47 ms (106 fps)** · mean 9.55 · p95 10.88 · p99 11.95 (1% low 84 fps) · worst 18.65 ms · 1 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 323 frames over 8.6 s, worst 2475.4 ms
- hitches (> 25.0 ms): **0** measured, 45 in warm-up
- timed GPU passes: 8.21 ms of a 9.55 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.47 ms, limit 11 ms
- pass worst frame 18.65 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.492 | 2.536 | 2.558 | 0.98 | 0.073 |
| dlss_super_resolution | 1.985 | 2.020 | 2.044 | 0.98 | 0.186 |
| early prepass | 0.638 | 0.649 | 0.656 | 0.98 | 0.101 |
| atmosphere_luts | 0.331 | 0.337 | 0.394 | 0.98 | 0.003 |
| bloom | 0.311 | 0.316 | 0.332 | 0.98 | 0.064 |
| main_transmissive_pass_3d | 0.307 | 0.313 | 0.334 | 0.98 | 0.009 |
| water_velocity | 0.236 | 0.240 | 0.263 | 0.98 | 0.002 |
| water_sspr | 0.206 | 0.210 | 0.213 | 0.98 | 0.002 |
| ssao | 0.186 | 0.190 | 0.198 | 0.98 | 0.007 |
| clouds_trace | 0.145 | 0.148 | 0.167 | 0.98 | 0.001 |
| atmos_froxel_inject | 0.143 | 0.146 | 0.151 | 0.98 | 0.001 |
| water_fft | 0.130 | 0.132 | 0.136 | 0.98 | 0.003 |
| uniform_allocation | 0.111 | 0.113 | 0.147 | 0.98 | 0.020 |
| clouds_composite | 0.110 | 0.112 | 0.113 | 0.98 | 0.002 |
| upscaling | 0.107 | 0.109 | 0.119 | 0.98 | 0.004 |
| gpu_instance_materialization | 0.087 | 0.089 | 0.090 | 0.98 | 0.004 |
| bin_unpacking | 0.086 | 0.087 | 0.115 | 0.98 | 0.015 |
| tonemapping | 0.085 | 0.086 | 0.089 | 0.98 | 0.004 |
| clustering | 0.065 | 0.066 | 0.075 | 0.98 | 0.021 |
| early_mesh_preprocessing | 0.060 | 0.061 | 0.087 | 0.98 | 0.004 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.413 | 0.12 | 0.001 |
| render_sky | 0.051 | 0.052 | 0.052 | 0.98 | 0.002 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 0.98 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 0.98 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.030 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.218 | 0.227 | 0.12 | 0.004 |
| water_snapshot | 0.023 | 0.023 | 0.025 | 0.98 | 0.002 |
| range_unpacking | 0.023 | 0.023 | 0.027 | 0.98 | 0.003 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.019 | 0.020 | 0.043 | 0.98 | 0.006 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.98 | 0.002 |
| surface_exposure_far | 0.011 | 0.039 | 0.040 | 0.29 | 0.002 |
| atmosphere_environment | 0.010 | 0.011 | 0.014 | 0.98 | 0.003 |
| surface_exposure | 0.010 | 0.039 | 0.040 | 0.27 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.005 | 0.005 | 0.018 | 0.98 | 0.047 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.98 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.624 | 10.003 | 17.56 | 13548.7 |
| render/busy/ms | 7.563 | 8.855 | 17.01 | 11881.5 |
| main/busy/ms | 7.335 | 8.154 | 10.91 | 11523.5 |
| stage/render/render | 4.985 | 6.024 | 8.46 | 7831.1 |
| render/graph/ms | 4.392 | 5.345 | 7.55 | 6900.6 |
| encode//sum | 4.218 | 5.630 | 8.11 | 6627.2 |
| stage/main/post_update | 4.120 | 4.884 | 7.46 | 6472.9 |
| stage/render/graph_submit | 3.360 | 4.289 | 6.53 | 5279.2 |
| encode/submit | 2.135 | 2.672 | 4.62 | 3353.4 |
| stage/main/update | 1.910 | 2.317 | 2.80 | 3001.3 |
| encode/finish_wall | 1.260 | 1.808 | 3.23 | 1978.9 |
| pipelines/compile_ms | 0.970 | 0.000 | 146.94 | 1524.5 |
| stage/render/camera_driver | 0.968 | 1.103 | 1.38 | 1521.4 |
| pipelines/front_ms | 0.945 | 0.000 | 143.38 | 1484.1 |
| encode/pass:shadow_directional_light/sum | 0.825 | 1.466 | 2.31 | 1295.8 |
| encode/pass:early prepass/sum | 0.775 | 1.079 | 2.87 | 1217.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.770 | 1.088 | 2.00 | 1209.2 |
| stage/render/prepare_views | 0.740 | 2.101 | 3.02 | 1162.5 |
| render/acquire/ms | 0.692 | 2.047 | 2.97 | 1087.0 |
| stage/render/prepare_write_phase_buffers | 0.538 | 0.748 | 0.98 | 844.8 |
| stage/render/prepare_assets | 0.468 | 0.649 | 1.55 | 735.9 |
| bigspace/total/ms | 0.398 | 1.066 | 1.75 | 625.5 |
| stage/render/prepare_resources | 0.394 | 0.459 | 0.65 | 619.2 |
| render/present/ms | 0.369 | 0.444 | 1.83 | 580.1 |
| stage/main/pre_update | 0.369 | 0.591 | 0.97 | 579.1 |
| stage/main/first | 0.304 | 0.377 | 1.97 | 478.4 |
| stage/render/prepare_flush | 0.275 | 0.446 | 0.81 | 432.6 |
| stage/render/prepare_bind_groups | 0.265 | 0.375 | 0.56 | 416.7 |
| stage/main/last | 0.254 | 0.308 | 0.66 | 398.4 |
| terrain/update/ms | 0.241 | 0.316 | 0.49 | 378.4 |
| stage/render/prepare_meshes | 0.220 | 0.327 | 1.00 | 346.3 |
| stage/render/prepare_batch_phases | 0.218 | 0.315 | 0.43 | 342.0 |
| stream/drive/ms | 0.210 | 0.306 | 0.74 | 330.6 |
| terrain/select/ms | 0.209 | 0.273 | 0.46 | 328.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.189 | 0.277 | 0.45 | 297.5 |
| stage/main/fixed_main_loop | 0.189 | 0.391 | 0.86 | 297.3 |
| stage/render/extract_commands | 0.185 | 0.222 | 0.69 | 290.8 |
| far/select/ms | 0.185 | 0.263 | 0.69 | 290.4 |
| stream/tick/ms | 0.179 | 0.268 | 0.70 | 281.4 |
| stage/render/create_views | 0.172 | 0.225 | 0.75 | 269.9 |
| controller/feed/ms | 0.163 | 0.214 | 0.57 | 256.1 |
| bigspace/high_precision/ms | 0.154 | 0.681 | 1.55 | 241.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.125 | 0.193 | 0.38 | 195.9 |
| controller/move/ms | 0.115 | 0.181 | 0.32 | 180.9 |
| bigspace/low_precision/ms | 0.113 | 0.508 | 1.38 | 177.2 |
| stage/render/specialize | 0.102 | 0.141 | 3.42 | 160.1 |
| anim/evaluate/ms | 0.092 | 0.244 | 1.16 | 145.0 |
| anim/evaluate_cpu/ms | 0.064 | 0.079 | 0.20 | 100.6 |
| encode/resolve_diagnostics/sum | 0.061 | 0.091 | 0.35 | 96.1 |
| stage/render/queue | 0.061 | 0.085 | 2.57 | 95.5 |
| stage/render/phase_sort | 0.057 | 0.078 | 0.29 | 89.8 |
| stage/render/prepare_collect_phase_buffers | 0.045 | 0.073 | 0.13 | 71.2 |
| weather/map/ms | 0.028 | 0.036 | 0.50 | 43.4 |
| physics/step/ms | 0.027 | 0.057 | 0.46 | 43.1 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.041 | 0.11 | 40.8 |
| pipelines/driver_ms | 0.026 | 0.000 | 4.11 | 40.3 |
| encode/camera_driver_span_end/sum | 0.024 | 0.040 | 0.11 | 37.3 |
| stage/render/cleanup | 0.023 | 0.031 | 0.38 | 35.5 |
| weather/eval/ms | 0.012 | 0.016 | 0.41 | 18.8 |
| script/update/ms | 0.008 | 0.012 | 0.07 | 12.4 |
| controller/avoid/ms | 0.006 | 0.008 | 0.04 | 9.9 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 8.4 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 7.7 |
| anim/apply/ms | 0.005 | 0.006 | 0.03 | 7.1 |
| controller/write/ms | 0.002 | 0.003 | 0.02 | 3.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.39 | 1.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.6 |
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
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.4 k | 37.4 k | 37.4 k |
| encode//max | ms | 0.60 | 2.75 | 0.73 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.11 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.11 | 0.03 |
| encode/finish_critical | ms | 0.93 | 2.87 | 1.21 |
| encode/pass:early prepass/max | ms | 0.76 | 2.87 | 0.39 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.77 | 2.00 | 0.56 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.38 | 0.15 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.18 | 0.45 | 0.12 |
| encode/pass:shadow_directional_light/max | ms | 0.76 | 2.31 | 1.21 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.35 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 8.99 | 17.01 | 9.02 |
| gpu/timed/ms | ms | 8.99 | 9.67 | 9.02 |
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
| process/cpu_usage | % | 32.64 | 42.79 | 36.54 |
| process/mem_usage | GiB | 4.74 | 4.78 | 4.77 |
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
| skin/blocks_changed/count | count | 5 | 7 | 5 |
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
| system/cpu_usage | % | 39.11 | 47.00 | 41.78 |
| system/mem_usage | % | 54.61 | 54.79 | 54.75 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2552 | 2552 | 2552 |
| textures/images | count | 1998 | 1998 | 1998 |
| views/all/batch_sets | count | 64 | 101 | 94 |
| views/all/bins | count | 3148 | 3761 | 3148 |
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
| anim/events/count | count | 0.09 | 3 | 147 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.15 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 27.54 | 30 | 43.3 k |
| encode/bind_walk/init_textures | count | 7096 | 13.9 k | 11.15 M |
| encode/bind_walk/init_textures_skipped | count | 39.1 k | 52.1 k | 61.39 M |
| encode/bind_walk/inits | count | 330 | 435 | 518.2 k |
| encode/bind_walk/inits_skipped | count | 51.00 | 63 | 80.1 k |
| encode/bind_walk/merge_textures | count | 7032 | 13.8 k | 11.05 M |
| encode/bind_walk/merge_textures_skipped | count | 3925 | 3925 | 6.17 M |
| encode/bind_walk/merges | count | 79.26 | 100 | 124.5 k |
| encode/bind_walk/merges_skipped | count | 32.75 | 38 | 51.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1571 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1571 |
| encode/pass:early prepass/count | count | 1 | 1 | 1571 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1571 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1571 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1571 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1571 |
| encode/queue/command_buffers | count | 37.54 | 40 | 59.0 k |
| encode/queue/submits | count | 2 | 2 | 3142 |
| encode/queue/write_buffer/bytes | bytes | 2.21 M | 3.62 M | 3.47 G |
| encode/queue/write_buffer/calls | count | 229 | 302 | 360.3 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 51.28 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 14.1 k |
| encode/queue/write_texture/bytes | bytes | 11.4 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1571 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.57 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.01 | 1 | 21 |
| pipelines/created | count | 0.01 | 1 | 21 |
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

## Hitches (> 25.0 ms; slowest 45, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.48 | 2475.4 | main/busy/ms 2447, stage/main/post_update 2340, stage/main/update 97.95, surface/advance_cpu_ms 67.92, stage/main/pre_update 4.43, stage/main/last 1.01, surface/cell_upload_ms 0.71, stage/main/first 0.60, stage/main/fixed_main_loop 0.41, stream/drive/ms 0.33, weather/map/ms 0.24, stream/tick/ms 0.11 |
| warm-up | 3.20 | 729.6 | main/busy/ms 2579, stage/main/post_update 2365, render/thread/ms 698, render/busy/ms 533, stage/render/prepare_assets 484, stage/main/update 199, stage/render/prepare_views 159, render/acquire/ms 159, surface/advance_cpu_ms 67.92, stage/render/create_views 28.07, stage/render/render 15.31, render/graph/ms 8.37 |
| warm-up | 3.31 | 102.0 | render/thread/ms 784, render/busy/ms 573, stage/render/prepare_assets 510, stage/render/prepare_views 159, render/acquire/ms 159, main/busy/ms 150, stage/main/update 106, stage/render/render 66.26, render/present/ms 51.24, pipelines/compile_ms 45.91, pipelines/front_ms 38.76, stage/main/post_update 33.64 |
| warm-up | 3.45 | 123.6 | pipelines/wait_ms 3619, pipelines/compile_ms 935, pipelines/front_ms 860, render/thread/ms 119, render/busy/ms 117, main/busy/ms 76.83, pipelines/driver_ms 75.21, stage/main/post_update 64.35, stage/render/prepare_assets 28.90, stage/render/prepare_resources 28.75, stage/render/render 21.08, stage/render/prepare_flush 20.95 |
| warm-up | 3.51 | 52.7 | pipelines/wait_ms 4097, pipelines/compile_ms 1451, pipelines/front_ms 1380, render/thread/ms 149, render/busy/ms 146, main/busy/ms 105, pipelines/driver_ms 70.53, encode//sum 63.15, stage/main/update 58.32, stage/render/prepare_assets 45.20, stage/main/post_update 37.95, stage/render/prepare_resources 28.59 |
| warm-up | 3.59 | 87.7 | pipelines/compile_ms 1661, pipelines/front_ms 1579, pipelines/wait_ms 759, render/thread/ms 113, render/busy/ms 111, main/busy/ms 92.42, pipelines/driver_ms 81.29, stage/render/prepare_assets 66.29, encode//sum 63.13, stage/main/update 59.49, stage/main/post_update 23.56, stage/render/render 23.29 |
| warm-up | 3.62 | 29.4 | pipelines/compile_ms 1516, pipelines/front_ms 1437, pipelines/wait_ms 264, render/thread/ms 88.95, render/busy/ms 86.98, pipelines/driver_ms 79.52, stage/render/prepare_assets 66.20, main/busy/ms 37.74, stage/main/post_update 21.44, stage/render/render 13.81, render/graph/ms 11.27, terrain/update/ms 10.59 |
| warm-up | 3.67 | 26.8 | pipelines/compile_ms 769, pipelines/front_ms 746, pipelines/wait_ms 125, render/thread/ms 46.49, render/busy/ms 44.82, main/busy/ms 40.99, pipelines/driver_ms 22.88, stage/main/update 19.84, stage/main/post_update 17.32, stage/render/render 12.37, stage/render/prepare_assets 10.44, render/graph/ms 10.17 |
| warm-up | 3.71 | 41.1 | pipelines/compile_ms 719, pipelines/front_ms 701, pipelines/wait_ms 111, main/busy/ms 111, stage/main/update 85.27, render/thread/ms 59.37, render/busy/ms 56.93, stage/main/post_update 20.67, pipelines/driver_ms 17.85, stage/render/prepare_assets 17.23, stage/render/render 17.02, render/graph/ms 14.14 |
| warm-up | 3.87 | 157.7 | pipelines/compile_ms 1959, pipelines/front_ms 1865, pipelines/wait_ms 143, render/thread/ms 108, render/busy/ms 105, pipelines/driver_ms 93.92, main/busy/ms 93.71, stage/main/update 75.64, stage/render/prepare_assets 73.88, stage/render/render 21.30, render/graph/ms 17.89, stage/render/graph_submit 12.69 |
| warm-up | 3.92 | 45.3 | pipelines/compile_ms 2079, pipelines/front_ms 1980, pipelines/wait_ms 159, render/thread/ms 101, pipelines/driver_ms 99.24, render/busy/ms 98.95, stage/render/prepare_assets 76.11, main/busy/ms 28.12, stage/render/render 15.31, pipelines/lock_ms 13.40, render/graph/ms 12.90, stage/main/update 12.72 |
| warm-up | 3.99 | 73.2 | pipelines/compile_ms 414, pipelines/front_ms 400, render/thread/ms 69.95, render/busy/ms 68.56, stage/render/prepare_assets 52.15, main/busy/ms 23.79, pipelines/wait_ms 17.31, pipelines/driver_ms 13.64, pipelines/lock_ms 12.42, stage/render/render 11.25, stage/main/update 9.73, render/graph/ms 9.55 |
| warm-up | 4.04 | 53.9 | pipelines/compile_ms 420, pipelines/front_ms 340, render/thread/ms 96.25, render/busy/ms 94.69, pipelines/driver_ms 80.26, stage/render/render 46.34, render/graph/ms 44.52, stage/render/graph_submit 42.17, encode/submit 37.87, stage/render/prepare_assets 34.00, main/busy/ms 19.86, stage/main/update 9.98 |
| warm-up | 4.08 | 38.7 | pipelines/compile_ms 864, pipelines/front_ms 768, pipelines/driver_ms 96.70, render/thread/ms 89.97, render/busy/ms 66.62, stage/render/render 52.99, render/graph/ms 50.99, stage/render/graph_submit 48.77, encode/submit 45.37, stage/render/prepare_views 22.14, render/acquire/ms 21.92, main/busy/ms 17.53 |
| warm-up | 4.16 | 69.7 | pipelines/compile_ms 688, pipelines/front_ms 668, main/busy/ms 58.04, stage/main/update 45.27, stream/drive/ms 36.59, render/thread/ms 29.89, render/busy/ms 28.77, pipelines/driver_ms 20.02, stage/render/prepare_assets 16.64, stage/main/post_update 8.29, stage/render/render 7.72, encode//sum 6.81 |
| warm-up | 4.19 | 31.5 | pipelines/compile_ms 357, pipelines/front_ms 345, main/busy/ms 127, stage/main/update 111, render/thread/ms 45.61, render/busy/ms 44.06, stage/render/prepare_assets 24.40, stream/drive/ms 15.34, stage/render/render 13.56, pipelines/driver_ms 12.71, render/graph/ms 11.33, stage/main/post_update 9.92 |
| warm-up | 4.33 | 142.6 | pipelines/compile_ms 528, pipelines/front_ms 511, main/busy/ms 134, stage/main/update 115, render/thread/ms 55.72, render/busy/ms 54.08, stage/render/prepare_assets 34.21, stream/drive/ms 22.37, pipelines/driver_ms 17.10, stage/render/render 13.31, render/graph/ms 11.35, stage/main/post_update 9.72 |
| warm-up | 4.37 | 36.7 | pipelines/compile_ms 864, pipelines/front_ms 833, render/thread/ms 64.34, render/busy/ms 62.36, main/busy/ms 55.41, stage/render/prepare_assets 33.10, pipelines/driver_ms 31.24, stage/main/update 26.91, stage/main/post_update 18.34, stage/render/render 16.62, render/graph/ms 14.44, stream/drive/ms 12.10 |
| warm-up | 4.41 | 42.3 | pipelines/compile_ms 653, pipelines/front_ms 619, main/busy/ms 68.25, render/thread/ms 63.21, render/busy/ms 61.09, stage/main/update 42.65, pipelines/driver_ms 34.12, stage/render/render 22.25, render/graph/ms 19.79, stage/main/post_update 18.14, stage/render/prepare_assets 17.53, stage/render/graph_submit 15.16 |
| warm-up | 4.47 | 56.2 | pipelines/compile_ms 219, pipelines/front_ms 200, render/thread/ms 66.32, render/busy/ms 64.74, main/busy/ms 55.17, stage/main/update 39.46, stage/render/prepare_assets 28.70, pipelines/driver_ms 19.35, stage/render/render 18.11, render/graph/ms 16.01, stage/render/graph_submit 12.97, pipelines/wait_ms 10.62 |
| warm-up | 4.50 | 34.0 | pipelines/compile_ms 135, pipelines/front_ms 130, render/thread/ms 62.74, render/busy/ms 61.30, main/busy/ms 40.24, stage/render/prepare_assets 27.76, stage/main/update 23.58, stage/render/render 14.35, render/graph/ms 12.53, stage/main/post_update 9.91, stage/render/graph_submit 9.68, encode//sum 7.90 |
| warm-up | 4.56 | 55.0 | pipelines/compile_ms 944, pipelines/front_ms 908, render/thread/ms 75.56, render/busy/ms 74.11, main/busy/ms 52.03, stage/render/prepare_assets 40.24, pipelines/driver_ms 36.70, stage/main/update 31.95, stage/main/post_update 12.88, stage/render/render 12.32, render/graph/ms 10.45, encode//sum 8.39 |
| warm-up | 4.65 | 91.6 | pipelines/compile_ms 2281, pipelines/front_ms 2205, render/thread/ms 127, render/busy/ms 126, main/busy/ms 85.12, pipelines/driver_ms 76.05, stage/render/prepare_assets 48.79, pipelines/wait_ms 39.40, stage/main/update 38.51, stage/main/post_update 25.17, stage/render/prepare_resources 24.00, stage/render/prepare_write_phase_buffers 18.53 |
| warm-up | 4.71 | 59.1 | pipelines/compile_ms 1610, pipelines/front_ms 1565, render/thread/ms 130, render/busy/ms 128, main/busy/ms 69.07, stage/render/prepare_assets 47.04, pipelines/driver_ms 45.26, pipelines/wait_ms 37.97, stage/main/update 26.77, stage/render/prepare_resources 24.55, stage/render/prepare_write_phase_buffers 21.66, stage/main/post_update 21.51 |
| warm-up | 4.77 | 59.3 | pipelines/compile_ms 1640, pipelines/front_ms 1601, render/thread/ms 83.22, render/busy/ms 81.96, pipelines/wait_ms 65.41, stage/render/prepare_assets 54.54, pipelines/driver_ms 39.25, main/busy/ms 26.61, stage/main/update 13.23, stage/render/render 11.40, render/graph/ms 9.51, encode//sum 8.45 |
| warm-up | 4.80 | 35.4 | pipelines/compile_ms 1661, pipelines/front_ms 1617, pipelines/wait_ms 90.90, render/thread/ms 66.28, render/busy/ms 64.67, main/busy/ms 50.17, pipelines/driver_ms 43.59, stage/main/update 34.95, stage/render/prepare_assets 30.65, stage/render/render 21.29, render/graph/ms 19.21, stream/drive/ms 12.30 |
| warm-up | 4.84 | 42.5 | pipelines/compile_ms 718, pipelines/front_ms 690, main/busy/ms 70.29, render/thread/ms 61.23, render/busy/ms 59.38, stage/main/update 47.43, pipelines/driver_ms 27.81, pipelines/wait_ms 27.50, stage/render/render 25.40, render/graph/ms 22.99, stage/render/prepare_assets 15.62, stream/drive/ms 13.30 |
| warm-up | 4.89 | 49.8 | pipelines/compile_ms 905, pipelines/front_ms 875, main/busy/ms 64.38, render/thread/ms 64.11, render/busy/ms 62.61, stage/main/update 42.79, pipelines/driver_ms 30.24, stage/render/prepare_assets 28.41, stage/render/render 14.09, render/graph/ms 11.99, stage/main/post_update 8.68, stage/render/graph_submit 8.62 |
| warm-up | 4.95 | 31.1 | pipelines/compile_ms 99.94, pipelines/front_ms 94.24, render/thread/ms 47.33, main/busy/ms 46.08, render/busy/ms 45.69, stage/main/update 31.28, stream/drive/ms 13.49, stage/render/render 13.36, stage/render/prepare_assets 12.00, render/graph/ms 11.58, stage/main/post_update 9.19, stage/render/graph_submit 8.77 |
| warm-up | 5.05 | 33.7 | render/thread/ms 45.57, render/busy/ms 44.10, main/busy/ms 39.42, stage/main/update 23.41, stage/render/render 12.55, stage/render/prepare_assets 11.23, render/graph/ms 10.70, stage/main/post_update 9.91, encode//sum 8.03, stage/render/graph_submit 7.56, stage/render/prepare_bind_groups 4.88, encode/submit 4.54 |
| warm-up | 5.08 | 31.3 | pipelines/compile_ms 135, pipelines/front_ms 130, render/thread/ms 54.11, render/busy/ms 52.39, main/busy/ms 39.82, stage/main/update 24.38, stage/render/prepare_assets 15.40, stage/render/render 11.57, render/graph/ms 9.79, stage/main/post_update 8.64, encode//sum 8.39, stage/render/graph_submit 6.94 |
| warm-up | 5.11 | 30.6 | pipelines/compile_ms 342, pipelines/front_ms 331, render/thread/ms 53.89, render/busy/ms 52.15, main/busy/ms 40.94, stage/main/update 18.67, stage/render/render 12.74, pipelines/driver_ms 11.37, render/graph/ms 10.95, stage/main/post_update 10.76, stage/render/prepare_assets 10.38, encode//sum 8.10 |
| warm-up | 5.19 | 42.7 | pipelines/compile_ms 1031, pipelines/front_ms 1005, render/thread/ms 59.83, render/busy/ms 58.38, pipelines/driver_ms 26.34, main/busy/ms 25.47, stage/render/prepare_write_phase_buffers 13.78, stage/render/render 13.12, stage/render/prepare_resources 12.47, render/graph/ms 11.23, stage/main/update 10.58, stage/main/post_update 10.58 |
| warm-up | 5.21 | 26.8 | pipelines/compile_ms 2092, pipelines/front_ms 2038, render/thread/ms 62.83, render/busy/ms 61.41, pipelines/driver_ms 54.51, pipelines/wait_ms 31.98, main/busy/ms 27.05, stage/render/prepare_assets 13.28, stage/render/prepare_write_phase_buffers 13.23, stage/main/update 13.02, stage/render/render 12.09, render/graph/ms 10.13 |
| warm-up | 5.25 | 37.5 | pipelines/compile_ms 1149, pipelines/front_ms 1116, render/thread/ms 57.15, render/busy/ms 55.53, main/busy/ms 33.76, pipelines/driver_ms 32.97, pipelines/wait_ms 31.67, stage/render/prepare_assets 16.19, stage/main/update 15.79, stage/render/render 14.74, render/graph/ms 12.49, stage/main/post_update 12.37 |
| warm-up | 5.32 | 41.1 | pipelines/compile_ms 218, pipelines/front_ms 214, render/thread/ms 42.93, render/busy/ms 41.79, pipelines/wait_ms 40.83, main/busy/ms 35.76, stage/render/prepare_assets 21.24, stage/main/update 20.28, stage/render/render 9.20, stage/main/post_update 8.73, render/graph/ms 7.17, encode//sum 6.73 |
| warm-up | 5.50 | 27.9 | render/thread/ms 39.70, render/busy/ms 38.29, main/busy/ms 32.94, stage/main/update 15.57, stage/render/render 14.75, render/graph/ms 12.68, stage/main/post_update 12.39, stage/render/prepare_assets 11.46, encode//sum 10.30, stage/render/graph_submit 8.87, encode/submit 5.79, stream/drive/ms 3.63 |
| warm-up | 5.54 | 37.7 | pipelines/compile_ms 158, pipelines/front_ms 153, render/thread/ms 54.32, render/busy/ms 52.66, main/busy/ms 39.38, stage/render/prepare_assets 22.51, stage/main/update 16.95, stage/main/post_update 16.19, stage/render/render 14.95, render/graph/ms 12.25, encode//sum 12.20, stage/render/graph_submit 8.43 |
| warm-up | 5.57 | 30.8 | pipelines/compile_ms 158, pipelines/front_ms 153, render/thread/ms 49.51, render/busy/ms 48.06, main/busy/ms 33.02, stage/render/prepare_assets 24.05, stage/main/update 14.19, stage/main/post_update 13.37, stage/render/render 12.19, render/graph/ms 9.82, encode//sum 9.30, stage/render/graph_submit 7.09 |
| warm-up | 5.60 | 31.9 | render/thread/ms 39.50, render/busy/ms 38.61, stage/render/prepare_assets 22.92, main/busy/ms 21.40, stage/main/update 9.78, stage/main/post_update 7.06, stage/render/render 6.79, encode//sum 5.60, render/graph/ms 5.50, stage/render/graph_submit 3.63, stage/render/prepare_meshes 3.23, stream/drive/ms 2.45 |
| warm-up | 6.04 | 25.6 | render/thread/ms 34.96, render/busy/ms 34.06, main/busy/ms 28.99, stage/main/update 18.40, stage/render/render 11.21, stage/render/prepare_assets 10.71, render/graph/ms 9.83, encode//sum 7.98, stage/main/post_update 6.85, stage/render/graph_submit 6.46, stage/render/prepare_meshes 4.43, encode/submit 3.98 |
| warm-up | 6.25 | 25.6 | render/thread/ms 35.91, render/busy/ms 34.16, main/busy/ms 30.98, stage/render/render 16.37, stage/main/post_update 14.34, render/graph/ms 14.04, stage/main/update 13.13, stage/render/graph_submit 10.60, encode//sum 8.10, encode/submit 7.25, stage/render/prepare_assets 5.53, encode/finish_wall 3.51 |
| warm-up | 6.28 | 27.6 | render/thread/ms 44.02, render/busy/ms 42.21, main/busy/ms 36.46, stage/render/render 16.16, stage/main/post_update 15.86, stage/main/update 14.57, render/graph/ms 13.50, stage/render/prepare_assets 10.55, encode//sum 9.43, stage/render/graph_submit 9.22, encode/submit 5.88, stage/render/prepare_meshes 5.57 |
| warm-up | 6.31 | 27.5 | pipelines/compile_ms 155, pipelines/front_ms 152, render/thread/ms 39.89, render/busy/ms 38.82, main/busy/ms 29.84, stage/render/prepare_assets 16.81, stage/main/post_update 12.60, stage/main/update 10.69, stage/render/render 8.62, encode//sum 7.12, render/graph/ms 6.97, stage/render/prepare_meshes 5.64 |
| warm-up | 6.58 | 37.1 | main/busy/ms 31.87, render/thread/ms 27.69, render/busy/ms 26.59, pipelines/compile_ms 23.93, pipelines/front_ms 22.14, stage/main/update 18.85, stage/render/prepare_assets 9.31, stage/main/post_update 9.13, stage/render/render 8.83, render/graph/ms 7.39, encode//sum 6.91, stage/render/graph_submit 5.21 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=scene-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 2%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **97%** (min 92%) · 2625 MHz (min 2625) · memory 8101 MHz · 97.8 W · 80 °C (max 83) · VRAM 6441 MiB · throttled 100% of samples

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
