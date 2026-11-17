# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1553 frames over 15.0 s · **median 9.71 ms (103 fps)** · mean 9.66 · p95 10.99 · p99 11.53 (1% low 87 fps) · worst 28.11 ms · 3 spikes over 1.5× median · 12.0 stutters/min (work 4.0/min)
- warm-up: 279 frames over 8.6 s, worst 2570.2 ms
- hitches (> 25.0 ms): **1** measured, 52 in warm-up
- timed GPU passes: 8.53 ms of a 9.66 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.71 ms, limit 11 ms
- pass worst frame 28.11 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.889 | 2.897 | 2.984 | 1.00 | 0.063 |
| dlss_super_resolution | 2.019 | 2.024 | 2.044 | 1.00 | 0.185 |
| early prepass | 0.605 | 0.607 | 0.615 | 1.00 | 0.095 |
| atmosphere_luts | 0.325 | 0.326 | 0.341 | 1.00 | 0.003 |
| bloom | 0.314 | 0.315 | 0.331 | 1.00 | 0.062 |
| main_transmissive_pass_3d | 0.309 | 0.309 | 0.331 | 1.00 | 0.008 |
| water_velocity | 0.239 | 0.240 | 0.264 | 1.00 | 0.001 |
| water_sspr | 0.208 | 0.208 | 0.210 | 1.00 | 0.002 |
| ssao | 0.190 | 0.191 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.149 | 0.149 | 0.168 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.147 | 0.152 | 1.00 | 0.001 |
| water_fft | 0.131 | 0.131 | 0.135 | 1.00 | 0.003 |
| uniform_allocation | 0.119 | 0.119 | 0.160 | 1.00 | 0.022 |
| upscaling | 0.112 | 0.113 | 0.119 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.112 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.090 | 0.091 | 0.122 | 1.00 | 0.015 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.063 | 0.063 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.412 | 0.416 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.032 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.028 | 0.223 | 0.234 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 1.00 | 0.081 |
| water_snapshot | 0.024 | 0.024 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.045 | 0.047 | 0.32 | 0.002 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 1.00 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.014 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.727 | 10.113 | 26.85 | 13552.5 |
| main/busy/ms | 7.077 | 7.960 | 9.90 | 10991.1 |
| render/busy/ms | 6.861 | 7.908 | 26.33 | 10654.9 |
| stage/render/render | 4.588 | 5.417 | 10.09 | 7125.0 |
| stage/main/post_update | 4.224 | 4.963 | 6.92 | 6560.4 |
| render/graph/ms | 4.017 | 4.800 | 6.71 | 6238.2 |
| encode//sum | 3.749 | 4.899 | 12.49 | 5822.8 |
| stage/render/graph_submit | 3.028 | 3.790 | 5.77 | 4703.2 |
| encode/submit | 1.889 | 2.321 | 6.91 | 2934.2 |
| stage/main/update | 1.750 | 2.172 | 3.18 | 2718.1 |
| stage/render/prepare_views | 1.558 | 3.132 | 4.19 | 2419.5 |
| render/acquire/ms | 1.513 | 3.076 | 4.14 | 2349.9 |
| encode/finish_wall | 1.160 | 1.651 | 3.76 | 1800.9 |
| stage/render/camera_driver | 0.933 | 1.072 | 1.89 | 1449.0 |
| encode/pass:shadow_directional_light/sum | 0.744 | 1.295 | 1.93 | 1155.8 |
| encode/pass:early prepass/sum | 0.710 | 0.984 | 3.38 | 1102.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.700 | 0.975 | 1.19 | 1086.5 |
| stage/render/prepare_write_phase_buffers | 0.543 | 0.754 | 0.94 | 843.3 |
| stage/render/prepare_assets | 0.439 | 0.564 | 1.53 | 682.5 |
| stage/render/prepare_resources | 0.382 | 0.441 | 0.67 | 593.1 |
| render/present/ms | 0.353 | 0.422 | 3.16 | 547.7 |
| bigspace/total/ms | 0.352 | 0.675 | 1.66 | 546.7 |
| stage/main/pre_update | 0.337 | 0.447 | 0.86 | 523.0 |
| stage/main/first | 0.292 | 0.353 | 1.81 | 453.6 |
| terrain/update/ms | 0.232 | 0.299 | 0.74 | 360.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.231 | 0.331 | 0.91 | 358.4 |
| stage/render/prepare_bind_groups | 0.205 | 0.279 | 0.46 | 317.8 |
| terrain/select/ms | 0.202 | 0.260 | 0.71 | 314.3 |
| stage/render/prepare_meshes | 0.194 | 0.243 | 2.24 | 300.8 |
| stream/drive/ms | 0.185 | 0.236 | 0.67 | 286.6 |
| stage/main/fixed_main_loop | 0.178 | 0.332 | 1.03 | 276.8 |
| stage/render/prepare_batch_phases | 0.175 | 0.232 | 0.36 | 271.5 |
| stage/render/extract_commands | 0.168 | 0.199 | 0.52 | 261.1 |
| stage/render/create_views | 0.165 | 0.210 | 0.65 | 256.9 |
| far/select/ms | 0.161 | 0.209 | 0.66 | 250.1 |
| stream/tick/ms | 0.157 | 0.204 | 0.65 | 243.8 |
| controller/feed/ms | 0.149 | 0.197 | 0.59 | 231.1 |
| stage/render/prepare_flush | 0.148 | 0.266 | 0.55 | 229.3 |
| bigspace/high_precision/ms | 0.125 | 0.300 | 1.39 | 194.0 |
| stage/main/last | 0.118 | 0.161 | 0.62 | 182.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.113 | 0.172 | 0.41 | 174.8 |
| controller/move/ms | 0.112 | 0.170 | 0.58 | 173.6 |
| stage/render/queue | 0.106 | 0.134 | 5.98 | 163.9 |
| anim/evaluate/ms | 0.087 | 0.221 | 0.98 | 135.1 |
| bigspace/low_precision/ms | 0.084 | 0.177 | 1.30 | 130.9 |
| stage/render/specialize | 0.083 | 0.103 | 8.87 | 128.9 |
| anim/evaluate_cpu/ms | 0.059 | 0.074 | 0.54 | 91.5 |
| stage/render/prepare_collect_phase_buffers | 0.057 | 0.114 | 0.18 | 88.9 |
| encode/resolve_diagnostics/sum | 0.054 | 0.079 | 0.31 | 83.5 |
| physics/step/ms | 0.028 | 0.056 | 0.11 | 43.1 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.038 | 0.08 | 39.6 |
| weather/map/ms | 0.025 | 0.033 | 0.21 | 38.2 |
| stage/render/cleanup | 0.023 | 0.032 | 0.25 | 36.1 |
| encode/camera_driver_span_end/sum | 0.023 | 0.035 | 0.08 | 35.2 |
| stage/render/phase_sort | 0.019 | 0.034 | 0.13 | 29.5 |
| weather/eval/ms | 0.011 | 0.015 | 0.03 | 16.7 |
| script/update/ms | 0.007 | 0.009 | 0.34 | 10.5 |
| controller/avoid/ms | 0.006 | 0.007 | 0.03 | 8.9 |
| script/input/ms | 0.005 | 0.006 | 0.10 | 8.2 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 6.9 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 6.3 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 0.9 |
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
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 283 | 283 |
| crowd/c2/joints/count | count | 4916 | 4916 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1373 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.51 | 3.59 | 0.57 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.08 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.08 | 0.02 |
| encode/finish_critical | ms | 0.84 | 3.59 | 0.85 |
| encode/pass:early prepass/max | ms | 0.71 | 3.38 | 0.68 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.71 | 1.19 | 0.85 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.41 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 0.91 | 0.34 |
| encode/pass:shadow_directional_light/max | ms | 0.69 | 1.93 | 0.59 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.31 | 0.07 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.18 | 26.33 | 9.13 |
| gpu/timed/ms | ms | 9.18 | 10.83 | 9.13 |
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
| pipelines/total | count | 315 | 315 | 315 |
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
| process/cpu_usage | % | 26.24 | 38.94 | 13.94 |
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
| skin/blocks_changed/count | count | 5 | 6 | 3 |
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
| system/cpu_usage | % | 31.76 | 40.57 | 27.83 |
| system/mem_usage | % | 60.18 | 60.40 | 60.40 |
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
| anim/events/count | count | 0.11 | 3 | 166 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.19 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 29 | 41.3 k |
| encode/bind_walk/init_textures | count | 7165 | 14.0 k | 11.13 M |
| encode/bind_walk/init_textures_skipped | count | 39.5 k | 52.6 k | 61.32 M |
| encode/bind_walk/inits | count | 345 | 437 | 536.4 k |
| encode/bind_walk/inits_skipped | count | 71.00 | 83 | 110.3 k |
| encode/bind_walk/merge_textures | count | 7101 | 13.9 k | 11.03 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.34 M |
| encode/bind_walk/merges | count | 82.26 | 103 | 127.7 k |
| encode/bind_walk/merges_skipped | count | 52.75 | 58 | 81.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1553 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1553 |
| encode/pass:early prepass/count | count | 1 | 1 | 1553 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1553 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1553 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1553 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1553 |
| encode/queue/command_buffers | count | 36.57 | 39 | 56.8 k |
| encode/queue/submits | count | 2 | 2 | 3106 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.86 M | 2.23 G |
| encode/queue/write_buffer/calls | count | 217 | 288 | 337.5 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.69 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 14.0 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1553 |
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

## Hitches (> 25.0 ms; slowest 53, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2570.2 | main/busy/ms 2541, stage/main/post_update 2435, stage/main/update 96.92, surface/advance_cpu_ms 66.43, stage/main/pre_update 5.78, stage/main/last 1.16, stream/drive/ms 0.78, surface/cell_upload_ms 0.74, stage/main/first 0.46, stage/main/fixed_main_loop 0.37, weather/map/ms 0.16, stream/tick/ms 0.15 |
| warm-up | 3.32 | 751.0 | main/busy/ms 2680, stage/main/post_update 2460, render/thread/ms 717, render/busy/ms 544, stage/render/prepare_assets 496, stage/main/update 204, stage/render/prepare_views 167, render/acquire/ms 167, surface/advance_cpu_ms 66.43, stage/render/create_views 27.98, stage/render/render 15.63, render/graph/ms 8.70 |
| warm-up | 3.50 | 183.1 | render/thread/ms 883, render/busy/ms 611, stage/render/prepare_assets 547, main/busy/ms 168, stage/render/prepare_views 167, render/acquire/ms 167, stage/render/render 120, stage/main/update 113, render/present/ms 105, stage/main/post_update 39.76, stage/render/create_views 28.32, pipelines/compile_ms 24.58 |
| warm-up | 3.74 | 219.3 | pipelines/wait_ms 5119, pipelines/compile_ms 2681, pipelines/front_ms 2395, pipelines/driver_ms 285, render/thread/ms 183, render/busy/ms 180, stage/render/prepare_assets 152, main/busy/ms 92.16, stage/main/update 51.72, pipelines/lock_ms 29.02, stage/main/post_update 25.58, stage/render/render 18.46 |
| warm-up | 3.78 | 38.7 | pipelines/wait_ms 5232, pipelines/compile_ms 2682, pipelines/front_ms 2511, render/thread/ms 203, render/busy/ms 200, pipelines/driver_ms 171, stage/render/prepare_assets 167, main/busy/ms 70.06, encode//sum 60.90, pipelines/lock_ms 29.05, stage/main/post_update 23.99, stage/render/render 23.88 |
| warm-up | 3.86 | 82.3 | pipelines/compile_ms 565, pipelines/front_ms 544, pipelines/wait_ms 151, render/thread/ms 110, render/busy/ms 107, stage/render/prepare_assets 81.87, main/busy/ms 74.37, encode//sum 60.66, stage/main/post_update 33.76, pipelines/driver_ms 21.63, stage/render/render 18.80, stage/main/update 16.61 |
| warm-up | 3.90 | 30.5 | pipelines/compile_ms 71.02, pipelines/front_ms 66.12, main/busy/ms 41.93, render/thread/ms 35.48, render/busy/ms 33.36, stage/main/post_update 23.63, encode//sum 19.09, stage/render/render 16.49, render/graph/ms 14.23, stage/main/update 13.74, terrain/update/ms 11.87, stage/render/prepare_resources 10.27 |
| warm-up | 3.93 | 27.2 | pipelines/compile_ms 776, pipelines/front_ms 742, pipelines/wait_ms 87.50, main/busy/ms 48.15, render/thread/ms 48.01, render/busy/ms 45.86, pipelines/driver_ms 33.99, stage/main/post_update 26.14, stage/render/render 21.23, encode//sum 20.37, render/graph/ms 18.46, stage/main/update 15.27 |
| warm-up | 3.97 | 39.4 | pipelines/compile_ms 1670, pipelines/front_ms 1608, pipelines/wait_ms 89.62, pipelines/driver_ms 61.74, render/thread/ms 59.57, render/busy/ms 57.80, main/busy/ms 29.64, stage/render/prepare_assets 29.11, stage/main/post_update 16.55, stage/render/render 14.46, render/graph/ms 12.03, terrain/update/ms 9.60 |
| warm-up | 4.00 | 28.1 | pipelines/compile_ms 1446, pipelines/front_ms 1401, render/thread/ms 63.67, main/busy/ms 62.05, render/busy/ms 60.78, stage/main/update 50.71, stream/drive/ms 47.98, pipelines/driver_ms 44.09, stage/render/prepare_assets 36.09, stage/render/render 20.93, render/graph/ms 17.84, stage/render/graph_submit 14.75 |
| warm-up | 4.06 | 66.1 | pipelines/compile_ms 750, pipelines/front_ms 729, main/busy/ms 63.23, render/thread/ms 59.98, render/busy/ms 57.30, stage/main/update 51.82, stream/drive/ms 47.87, stage/render/prepare_assets 34.84, pipelines/driver_ms 20.81, stage/render/render 19.36, render/graph/ms 16.55, stage/render/graph_submit 13.74 |
| warm-up | 4.13 | 55.8 | pipelines/compile_ms 177, pipelines/front_ms 170, main/busy/ms 39.87, render/thread/ms 39.81, render/busy/ms 38.21, stage/main/update 25.62, stage/render/prepare_assets 22.40, stream/drive/ms 21.70, stage/render/render 11.70, render/graph/ms 9.57, encode//sum 8.86, stage/main/post_update 7.97 |
| warm-up | 4.18 | 47.1 | pipelines/compile_ms 746, pipelines/front_ms 719, render/thread/ms 70.01, render/busy/ms 67.95, stage/render/prepare_assets 48.10, main/busy/ms 44.59, pipelines/driver_ms 26.98, stage/main/update 25.65, stream/drive/ms 20.51, stage/render/render 14.79, render/graph/ms 11.98, stage/main/post_update 11.10 |
| warm-up | 4.25 | 75.6 | pipelines/compile_ms 569, pipelines/front_ms 549, render/thread/ms 85.83, render/busy/ms 83.77, main/busy/ms 72.36, stage/render/prepare_assets 58.13, stage/main/update 51.74, stream/drive/ms 20.81, pipelines/driver_ms 19.59, stage/render/render 17.18, render/graph/ms 14.58, stage/main/post_update 12.47 |
| warm-up | 4.42 | 170.1 | render/thread/ms 183, render/busy/ms 181, pipelines/compile_ms 159, pipelines/front_ms 152, main/busy/ms 132, stage/main/update 108, stage/render/prepare_assets 87.17, stream/drive/ms 66.66, stage/render/prepare_bind_groups 53.09, stage/render/render 32.24, render/graph/ms 29.25, stage/render/graph_submit 25.56 |
| warm-up | 4.49 | 66.9 | pipelines/compile_ms 1248, pipelines/front_ms 1210, render/thread/ms 182, render/busy/ms 180, main/busy/ms 106, stage/render/prepare_assets 89.70, stage/main/update 85.73, stream/drive/ms 66.58, pipelines/wait_ms 58.97, stage/render/prepare_bind_groups 52.65, pipelines/driver_ms 38.62, pipelines/lock_ms 32.24 |
| warm-up | 4.57 | 82.7 | pipelines/compile_ms 1270, pipelines/front_ms 1231, pipelines/wait_ms 177, render/thread/ms 97.04, render/busy/ms 95.17, stage/render/prepare_assets 72.31, main/busy/ms 50.71, pipelines/driver_ms 39.17, stage/main/update 28.55, pipelines/lock_ms 27.37, stream/drive/ms 13.74, stage/main/post_update 12.67 |
| warm-up | 4.61 | 38.5 | pipelines/compile_ms 927, pipelines/front_ms 891, pipelines/wait_ms 159, render/thread/ms 84.99, render/busy/ms 81.99, main/busy/ms 70.35, stage/render/prepare_assets 42.68, stage/main/update 36.52, pipelines/driver_ms 35.87, pipelines/lock_ms 33.65, stage/main/post_update 22.39, stage/render/render 16.71 |
| warm-up | 4.65 | 36.6 | pipelines/compile_ms 784, pipelines/front_ms 754, render/thread/ms 62.64, render/busy/ms 59.68, main/busy/ms 54.00, pipelines/wait_ms 34.28, pipelines/lock_ms 32.24, pipelines/driver_ms 29.74, stage/main/update 25.81, stage/render/prepare_assets 19.94, stage/main/post_update 17.55, stage/render/render 16.86 |
| warm-up | 4.68 | 32.2 | pipelines/compile_ms 272, pipelines/front_ms 262, render/thread/ms 52.59, render/busy/ms 51.24, main/busy/ms 32.04, stage/render/prepare_assets 25.21, stage/main/update 17.14, stage/render/render 11.16, render/graph/ms 9.26, pipelines/driver_ms 9.24, stage/main/post_update 8.57, encode//sum 6.70 |
| warm-up | 4.71 | 31.1 | pipelines/compile_ms 869, pipelines/front_ms 843, render/thread/ms 53.93, render/busy/ms 52.35, main/busy/ms 48.02, pipelines/driver_ms 26.34, stage/main/update 22.84, stage/main/post_update 20.41, stage/render/render 14.57, render/graph/ms 12.51, stage/render/prepare_assets 12.03, stage/render/graph_submit 9.09 |
| warm-up | 4.75 | 37.0 | pipelines/compile_ms 1100, pipelines/front_ms 1070, main/busy/ms 53.57, render/thread/ms 50.47, render/busy/ms 48.77, pipelines/driver_ms 29.55, stage/main/post_update 26.54, stage/main/update 21.18, stage/render/render 16.70, render/graph/ms 14.37, pipelines/wait_ms 11.54, stage/render/prepare_assets 11.18 |
| warm-up | 4.84 | 67.7 | pipelines/compile_ms 1110, pipelines/front_ms 1079, main/busy/ms 133, stage/main/update 112, stream/drive/ms 99.90, render/thread/ms 39.44, render/busy/ms 37.96, pipelines/driver_ms 31.89, stage/main/post_update 16.19, stage/render/prepare_assets 13.20, stage/render/render 12.21, render/graph/ms 9.85 |
| warm-up | 4.96 | 125.7 | pipelines/compile_ms 2042, pipelines/front_ms 1986, main/busy/ms 114, pipelines/wait_ms 94.36, render/thread/ms 85.58, render/busy/ms 84.15, stage/main/update 63.83, pipelines/driver_ms 55.31, stream/drive/ms 51.48, stage/main/post_update 43.58, stage/render/prepare_assets 23.38, stage/render/prepare_flush 23.12 |
| warm-up | 4.99 | 27.5 | pipelines/compile_ms 2238, pipelines/front_ms 2175, pipelines/wait_ms 110, main/busy/ms 89.11, render/thread/ms 87.73, render/busy/ms 85.81, pipelines/driver_ms 63.56, stage/main/update 42.05, stage/main/post_update 36.32, stage/render/prepare_flush 22.82, stage/render/prepare_assets 19.53, stage/render/prepare_resources 18.68 |
| warm-up | 5.08 | 87.3 | pipelines/compile_ms 1763, pipelines/front_ms 1712, main/busy/ms 69.58, render/thread/ms 64.49, render/busy/ms 62.59, stage/main/update 51.06, pipelines/driver_ms 50.90, stage/render/prepare_assets 29.77, pipelines/wait_ms 18.27, stage/render/render 15.50, render/graph/ms 13.04, stream/drive/ms 11.93 |
| warm-up | 5.15 | 27.1 | pipelines/compile_ms 434, pipelines/front_ms 419, pipelines/wait_ms 89.60, render/thread/ms 39.97, main/busy/ms 39.39, render/busy/ms 38.35, stage/main/update 23.13, pipelines/driver_ms 15.01, stage/render/render 11.50, stage/main/post_update 10.15, render/graph/ms 9.78, stage/render/graph_submit 6.96 |
| warm-up | 5.24 | 44.7 | pipelines/compile_ms 287, pipelines/front_ms 276, main/busy/ms 33.11, render/thread/ms 23.63, render/busy/ms 22.79, stage/main/post_update 13.45, stage/main/update 13.43, pipelines/driver_ms 10.79, stage/render/prepare_assets 9.10, stage/render/render 5.08, stream/drive/ms 4.20, render/graph/ms 4.05 |
| warm-up | 5.28 | 35.3 | pipelines/compile_ms 303, pipelines/front_ms 292, main/busy/ms 49.27, render/thread/ms 48.50, render/busy/ms 46.51, stage/main/post_update 24.59, stage/render/prepare_assets 21.88, stage/main/update 16.95, pipelines/driver_ms 11.31, stage/render/render 10.32, render/graph/ms 8.10, encode//sum 6.07 |
| warm-up | 5.34 | 58.5 | pipelines/compile_ms 1021, pipelines/front_ms 990, render/thread/ms 69.38, render/busy/ms 67.22, main/busy/ms 54.99, stage/render/prepare_assets 36.58, pipelines/driver_ms 31.21, stage/main/post_update 31.08, stage/render/render 17.90, stage/main/update 16.87, render/graph/ms 15.47, stage/render/graph_submit 12.77 |
| warm-up | 5.39 | 57.8 | pipelines/compile_ms 1611, pipelines/front_ms 1562, render/thread/ms 96.39, render/busy/ms 94.68, pipelines/driver_ms 48.15, main/busy/ms 47.16, stage/render/prepare_assets 34.93, pipelines/wait_ms 28.42, stage/render/prepare_resources 21.43, stage/main/post_update 20.30, stage/render/render 18.68, stage/main/update 16.87 |
| warm-up | 5.44 | 49.5 | pipelines/compile_ms 950, pipelines/front_ms 920, render/thread/ms 83.40, render/busy/ms 81.99, main/busy/ms 40.19, stage/render/prepare_assets 32.63, pipelines/driver_ms 30.06, pipelines/wait_ms 27.55, stage/render/prepare_resources 21.42, stage/main/update 19.53, stage/render/render 10.98, stage/main/post_update 10.41 |
| warm-up | 5.47 | 29.8 | pipelines/compile_ms 606, pipelines/front_ms 586, render/thread/ms 54.21, render/busy/ms 52.85, stage/render/prepare_assets 32.16, main/busy/ms 29.13, pipelines/driver_ms 19.77, stage/main/update 14.39, stage/render/render 10.13, stage/main/post_update 8.63, render/graph/ms 8.22, encode//sum 7.14 |
| warm-up | 5.69 | 44.0 | render/thread/ms 55.10, render/busy/ms 53.35, main/busy/ms 35.15, stage/render/prepare_assets 26.27, stage/render/render 20.24, stage/main/update 18.13, render/graph/ms 17.94, stage/render/graph_submit 11.37, stage/main/post_update 10.19, encode//sum 9.91, encode/submit 6.95, stream/drive/ms 5.40 |
| warm-up | 5.72 | 35.5 | pipelines/compile_ms 279, pipelines/front_ms 266, render/thread/ms 67.60, render/busy/ms 65.93, stage/render/prepare_assets 36.38, main/busy/ms 31.49, stage/render/render 18.45, render/graph/ms 16.14, stage/main/update 15.18, pipelines/driver_ms 12.64, stage/main/post_update 10.74, encode//sum 9.61 |
| warm-up | 5.77 | 35.6 | pipelines/compile_ms 167, pipelines/front_ms 162, render/thread/ms 49.37, render/busy/ms 47.78, main/busy/ms 28.41, stage/render/prepare_assets 22.51, stage/render/render 12.45, stage/main/update 12.42, render/graph/ms 10.32, stage/main/post_update 9.27, encode//sum 8.05, stage/render/graph_submit 6.86 |
| warm-up | 5.80 | 28.0 | render/thread/ms 58.58, render/busy/ms 56.19, main/busy/ms 32.91, stage/render/prepare_assets 20.21, stage/render/render 17.25, render/graph/ms 14.30, stage/main/update 13.43, stage/main/post_update 11.23, stage/render/graph_submit 10.07, encode//sum 9.97, encode/submit 6.74, stage/render/prepare_write_phase_buffers 5.49 |
| warm-up | 5.85 | 48.5 | render/thread/ms 64.86, render/busy/ms 62.78, main/busy/ms 35.71, stage/render/prepare_assets 31.59, stage/render/render 15.16, stage/main/update 15.10, stage/main/post_update 12.71, render/graph/ms 12.66, stage/render/graph_submit 8.93, encode//sum 8.87, encode/submit 5.94, stream/drive/ms 5.51 |
| warm-up | 5.88 | 30.0 | pipelines/compile_ms 1155, pipelines/front_ms 1130, render/thread/ms 62.74, render/busy/ms 61.47, stage/render/prepare_assets 43.31, main/busy/ms 31.97, pipelines/driver_ms 24.15, stage/main/update 13.60, stage/main/post_update 11.41, stage/render/render 9.42, render/graph/ms 7.71, encode//sum 6.82 |
| warm-up | 5.91 | 32.2 | pipelines/compile_ms 1155, pipelines/front_ms 1130, render/thread/ms 49.97, render/busy/ms 48.85, stage/render/prepare_assets 33.30, main/busy/ms 29.69, pipelines/driver_ms 24.15, stage/main/update 11.95, stage/main/post_update 10.73, stage/render/render 8.04, render/graph/ms 6.54, encode//sum 6.28 |
| warm-up | 5.94 | 27.2 | render/thread/ms 50.41, render/busy/ms 48.96, main/busy/ms 38.27, stage/render/prepare_assets 24.08, stage/main/post_update 16.41, stage/main/update 16.37, stage/render/render 13.08, render/graph/ms 11.18, stage/render/graph_submit 8.49, encode//sum 6.99, stream/drive/ms 5.83, encode/submit 4.71 |
| warm-up | 6.04 | 40.2 | main/busy/ms 40.70, render/thread/ms 33.83, render/busy/ms 32.12, stage/main/update 20.56, stage/render/render 13.88, stream/drive/ms 13.62, stage/main/post_update 13.11, render/graph/ms 11.54, encode//sum 8.46, stage/render/graph_submit 8.18, stage/render/prepare_assets 7.08, pipelines/compile_ms 5.15 |
| warm-up | 6.07 | 36.6 | render/thread/ms 48.43, render/busy/ms 46.59, main/busy/ms 37.95, stage/main/update 16.07, stage/render/prepare_assets 15.62, stage/render/render 15.10, stage/main/post_update 15.03, render/graph/ms 12.69, stream/drive/ms 9.45, stage/render/graph_submit 8.94, encode//sum 8.68, encode/submit 5.24 |
| warm-up | 6.12 | 41.2 | pipelines/compile_ms 327, pipelines/front_ms 318, render/thread/ms 53.56, render/busy/ms 52.07, main/busy/ms 46.41, stage/render/prepare_assets 29.40, stage/main/update 22.81, stage/main/post_update 15.87, stage/render/render 10.84, pipelines/driver_ms 9.26, render/graph/ms 9.10, stream/drive/ms 9.08 |
| warm-up | 6.14 | 28.5 | pipelines/compile_ms 899, pipelines/front_ms 879, render/thread/ms 48.75, render/busy/ms 47.47, main/busy/ms 37.05, stage/render/prepare_assets 29.24, pipelines/driver_ms 20.30, stage/main/update 17.63, stage/main/post_update 12.18, stage/render/render 10.29, render/graph/ms 8.70, encode//sum 6.55 |
| warm-up | 6.31 | 52.6 | pipelines/compile_ms 267, pipelines/front_ms 259, main/busy/ms 58.99, render/thread/ms 35.16, render/busy/ms 33.78, stage/main/update 30.53, stage/main/post_update 22.46, stream/drive/ms 20.27, pipelines/wait_ms 14.72, stage/render/prepare_assets 14.25, stage/render/render 10.21, render/graph/ms 8.30 |
| warm-up | 6.41 | 27.9 | main/busy/ms 37.42, render/thread/ms 32.45, render/busy/ms 31.14, stage/main/update 16.77, stage/main/post_update 13.86, stage/render/render 13.17, render/graph/ms 11.41, stage/render/prepare_assets 11.03, stage/render/graph_submit 7.88, encode//sum 7.67, encode/submit 5.25, stream/drive/ms 3.97 |
| warm-up | 6.63 | 27.8 | main/busy/ms 34.52, render/thread/ms 32.57, render/busy/ms 31.13, pipelines/compile_ms 21.75, pipelines/front_ms 20.77, stage/main/update 16.37, stage/render/render 15.12, stage/main/post_update 14.04, render/graph/ms 12.73, encode//sum 10.16, stage/render/graph_submit 9.12, stage/render/prepare_assets 7.05 |
| warm-up | 6.65 | 25.6 | main/busy/ms 44.74, render/thread/ms 40.98, render/busy/ms 38.91, pipelines/compile_ms 21.75, pipelines/front_ms 20.77, stage/main/update 19.69, stage/render/render 18.52, stage/main/post_update 17.82, render/graph/ms 15.49, stage/render/graph_submit 11.85, stage/render/prepare_assets 11.46, encode//sum 10.50 |
| warm-up | 6.70 | 44.2 | render/thread/ms 47.78, render/busy/ms 46.16, main/busy/ms 42.00, stage/render/prepare_assets 25.75, stage/main/update 19.57, stage/main/post_update 15.60, stage/render/render 13.89, render/graph/ms 11.82, stream/drive/ms 9.90, stage/render/graph_submit 9.00, encode//sum 7.98, encode/submit 6.17 |
| warm-up | 7.03 | 31.4 | render/thread/ms 38.15, render/busy/ms 37.21, main/busy/ms 16.26, stage/render/render 9.28, stage/render/specialize 8.69, stage/main/post_update 8.29, render/graph/ms 8.05, encode//sum 6.71, stage/render/queue 6.49, stage/render/graph_submit 6.23, stage/main/update 4.33, stage/render/prepare_assets 4.29 |
| warm-up | 8.24 | 29.3 | render/thread/ms 36.30, render/busy/ms 33.01, main/busy/ms 14.10, stage/render/render 10.07, render/graph/ms 8.39, stage/render/specialize 8.33, stage/main/post_update 7.83, stage/render/graph_submit 6.37, stage/render/queue 5.81, encode//sum 5.44, stage/render/prepare_meshes 4.22, stage/main/update 3.83 |
| measure | 0.83 | 28.1 | render/thread/ms 35.47, render/busy/ms 32.54, main/busy/ms 12.90, stage/render/render 10.68, render/graph/ms 9.37, stage/render/specialize 8.93, encode//sum 8.15, stage/main/post_update 7.65, stage/render/graph_submit 7.36, stage/render/queue 6.06, encode/submit 4.20, encode/finish_wall 3.19 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=base-5 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 0%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 83%) · 2625 MHz (min 2625) · memory 8101 MHz · 98.0 W · 77 °C (max 78) · VRAM 6428 MiB · throttled 100% of samples

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
