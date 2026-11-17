# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); select.gate=on (command line: --set); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1558 frames over 15.0 s · **median 9.59 ms (104 fps)** · mean 9.63 · p95 11.08 · p99 12.10 (1% low 83 fps) · worst 27.06 ms · 7 spikes over 1.5× median · 28.0 stutters/min (work 12.0/min)
- warm-up: 278 frames over 8.4 s, worst 2564.9 ms
- hitches (> 25.0 ms): **1** measured, 51 in warm-up
- timed GPU passes: 8.30 ms of a 9.63 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.59 ms, limit 11 ms
- pass worst frame 27.06 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.771 | 2.831 | 2.921 | 0.98 | 0.078 |
| dlss_super_resolution | 1.980 | 2.023 | 2.046 | 0.98 | 0.184 |
| early prepass | 0.600 | 0.613 | 0.628 | 0.98 | 0.096 |
| atmosphere_luts | 0.316 | 0.323 | 0.346 | 0.98 | 0.003 |
| bloom | 0.307 | 0.314 | 0.331 | 0.98 | 0.062 |
| main_transmissive_pass_3d | 0.286 | 0.292 | 0.331 | 0.98 | 0.009 |
| water_velocity | 0.235 | 0.241 | 0.267 | 0.98 | 0.002 |
| water_sspr | 0.200 | 0.204 | 0.210 | 0.98 | 0.002 |
| ssao | 0.186 | 0.190 | 0.197 | 0.98 | 0.007 |
| clouds_trace | 0.149 | 0.152 | 0.189 | 0.98 | 0.001 |
| atmos_froxel_inject | 0.141 | 0.144 | 0.152 | 0.98 | 0.001 |
| water_fft | 0.130 | 0.133 | 0.136 | 0.98 | 0.003 |
| uniform_allocation | 0.117 | 0.119 | 0.163 | 0.98 | 0.024 |
| upscaling | 0.112 | 0.114 | 0.120 | 0.98 | 0.004 |
| clouds_composite | 0.110 | 0.112 | 0.114 | 0.98 | 0.002 |
| bin_unpacking | 0.089 | 0.091 | 0.122 | 0.98 | 0.017 |
| tonemapping | 0.087 | 0.088 | 0.095 | 0.98 | 0.003 |
| clustering | 0.065 | 0.066 | 0.078 | 0.98 | 0.020 |
| render_sky | 0.051 | 0.053 | 0.054 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.413 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.052 | 0.98 | 0.001 |
| clouds_shadow | 0.031 | 0.032 | 0.036 | 0.98 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 0.98 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.030 | 0.041 | 0.98 | 0.006 |
| lightprobe_radiance_map | 0.027 | 0.220 | 0.229 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.026 | 0.028 | 0.98 | 0.082 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 0.98 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.98 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.046 | 0.31 | 0.002 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.98 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.014 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.008 | 0.98 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.98 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.669 | 10.140 | 25.88 | 13505.7 |
| render/busy/ms | 7.287 | 8.530 | 25.37 | 11352.9 |
| main/busy/ms | 6.923 | 7.812 | 8.85 | 10785.6 |
| stage/render/render | 4.936 | 6.000 | 11.95 | 7690.5 |
| render/graph/ms | 4.324 | 5.360 | 9.89 | 6737.3 |
| encode//sum | 4.186 | 5.916 | 11.55 | 6521.2 |
| stage/main/post_update | 4.125 | 4.829 | 6.03 | 6426.7 |
| stage/render/graph_submit | 3.299 | 4.284 | 7.81 | 5139.4 |
| encode/submit | 2.085 | 2.690 | 5.61 | 3248.9 |
| stage/main/update | 1.643 | 2.049 | 3.70 | 2559.5 |
| encode/finish_wall | 1.253 | 1.779 | 3.58 | 1952.0 |
| stage/render/prepare_views | 1.039 | 2.676 | 4.05 | 1618.1 |
| render/acquire/ms | 0.993 | 2.628 | 3.99 | 1547.4 |
| stage/render/camera_driver | 0.969 | 1.112 | 1.95 | 1509.8 |
| encode/pass:shadow_directional_light/sum | 0.797 | 1.386 | 2.35 | 1242.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.767 | 1.065 | 2.31 | 1194.4 |
| encode/pass:early prepass/sum | 0.759 | 1.054 | 3.15 | 1182.9 |
| stage/render/prepare_write_phase_buffers | 0.556 | 0.776 | 1.24 | 866.5 |
| stage/render/prepare_assets | 0.469 | 0.619 | 1.52 | 730.4 |
| stage/render/prepare_resources | 0.391 | 0.454 | 0.92 | 609.7 |
| render/present/ms | 0.389 | 0.472 | 1.99 | 605.4 |
| bigspace/total/ms | 0.387 | 0.971 | 1.82 | 602.7 |
| stage/main/pre_update | 0.361 | 0.496 | 0.90 | 562.9 |
| stage/main/first | 0.299 | 0.366 | 1.56 | 465.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.286 | 0.448 | 1.23 | 445.9 |
| stage/render/prepare_bind_groups | 0.214 | 0.296 | 0.54 | 334.0 |
| stage/render/prepare_meshes | 0.214 | 0.270 | 2.46 | 332.7 |
| stage/main/fixed_main_loop | 0.189 | 0.377 | 0.92 | 294.8 |
| stage/render/prepare_batch_phases | 0.180 | 0.240 | 0.38 | 280.0 |
| stage/render/extract_commands | 0.178 | 0.213 | 0.54 | 276.6 |
| stage/render/create_views | 0.173 | 0.224 | 0.71 | 268.9 |
| stage/render/prepare_flush | 0.155 | 0.286 | 0.61 | 241.9 |
| stream/drive/ms | 0.154 | 0.217 | 0.61 | 240.7 |
| controller/feed/ms | 0.147 | 0.192 | 0.63 | 228.7 |
| bigspace/high_precision/ms | 0.144 | 0.570 | 1.54 | 224.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.134 | 0.212 | 0.38 | 208.4 |
| controller/move/ms | 0.130 | 0.293 | 0.59 | 203.0 |
| stream/tick/ms | 0.124 | 0.182 | 0.55 | 193.7 |
| stage/main/last | 0.120 | 0.169 | 1.04 | 187.3 |
| stage/render/queue | 0.109 | 0.140 | 6.27 | 169.7 |
| bigspace/low_precision/ms | 0.100 | 0.226 | 1.24 | 155.6 |
| stage/render/specialize | 0.087 | 0.110 | 8.39 | 135.9 |
| anim/evaluate/ms | 0.085 | 0.187 | 1.51 | 131.8 |
| anim/evaluate_cpu/ms | 0.063 | 0.080 | 0.50 | 98.0 |
| encode/resolve_diagnostics/sum | 0.058 | 0.096 | 0.38 | 90.2 |
| stage/render/prepare_collect_phase_buffers | 0.057 | 0.114 | 0.18 | 89.0 |
| terrain/update/ms | 0.032 | 0.043 | 0.44 | 49.4 |
| physics/step/ms | 0.029 | 0.058 | 0.52 | 45.4 |
| encode/camera_driver_span_begin/sum | 0.028 | 0.048 | 0.20 | 43.5 |
| weather/map/ms | 0.025 | 0.034 | 0.49 | 39.7 |
| encode/camera_driver_span_end/sum | 0.025 | 0.044 | 0.10 | 38.6 |
| stage/render/cleanup | 0.024 | 0.032 | 0.41 | 37.0 |
| stage/render/phase_sort | 0.016 | 0.032 | 0.06 | 24.8 |
| weather/eval/ms | 0.012 | 0.015 | 0.40 | 18.0 |
| script/update/ms | 0.007 | 0.010 | 0.03 | 11.6 |
| controller/avoid/ms | 0.006 | 0.007 | 0.05 | 9.1 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 8.5 |
| anim/apply/ms | 0.005 | 0.006 | 0.03 | 7.1 |
| script/ui/ms | 0.004 | 0.005 | 0.43 | 6.3 |
| far/select/ms | 0.004 | 0.005 | 0.02 | 6.0 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 1.6 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 1.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 1.2 |
| terrain/select/ms | 0.000 | 0.001 | 0.02 | 0.8 |
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
| crowd/c1/count | count | 122 | 123 | 122 |
| crowd/c1/joints/count | count | 4848 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1088 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.57 | 3.53 | 0.68 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.20 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.10 | 0.02 |
| encode/finish_critical | ms | 0.90 | 3.53 | 0.74 |
| encode/pass:early prepass/max | ms | 0.74 | 3.15 | 0.54 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.77 | 2.31 | 0.61 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.38 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.27 | 1.23 | 0.25 |
| encode/pass:shadow_directional_light/max | ms | 0.72 | 2.17 | 0.74 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.38 | 0.05 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.13 | 25.37 | 9.14 |
| gpu/timed/ms | ms | 9.13 | 10.44 | 9.14 |
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
| process/cpu_usage | % | 26.44 | 37.80 | 27.62 |
| process/mem_usage | GiB | 4.68 | 4.76 | 4.76 |
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
| skin/blocks_changed/count | count | 5 | 7 | 5 |
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
| system/cpu_usage | % | 32.42 | 40.14 | 29.28 |
| system/mem_usage | % | 58.58 | 58.85 | 58.85 |
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
| anim/events/count | count | 0.09 | 2 | 148 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.18 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 54 | 41.4 k |
| encode/bind_walk/init_textures | count | 7164 | 16.3 k | 11.16 M |
| encode/bind_walk/init_textures_skipped | count | 35.6 k | 79.9 k | 55.47 M |
| encode/bind_walk/inits | count | 345 | 725 | 538.1 k |
| encode/bind_walk/inits_skipped | count | 66.99 | 142 | 104.4 k |
| encode/bind_walk/merge_textures | count | 7100 | 16.2 k | 11.06 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 8162 | 6.36 M |
| encode/bind_walk/merges | count | 82.25 | 173 | 128.2 k |
| encode/bind_walk/merges_skipped | count | 50.75 | 105 | 79.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1558 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1558 |
| encode/pass:early prepass/count | count | 1 | 2 | 1558 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1558 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 1558 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 1558 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 1558 |
| encode/queue/command_buffers | count | 36.57 | 74 | 57.0 k |
| encode/queue/submits | count | 2 | 4 | 3116 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.65 M | 2.24 G |
| encode/queue/write_buffer/calls | count | 217 | 469 | 338.5 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 65.3 k | 50.85 M |
| encode/queue/write_buffer_with/calls | count | 9 | 18 | 14.0 k |
| encode/queue/write_texture/bytes | bytes | 11.5 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1558 |
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

## Hitches (> 25.0 ms; slowest 52, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2564.9 | main/busy/ms 2534, stage/main/post_update 2429, stage/main/update 96.54, surface/advance_cpu_ms 66.94, stage/main/pre_update 4.30, stage/main/last 1.17, surface/cell_upload_ms 0.85, stage/main/first 0.54, stream/drive/ms 0.41, stage/main/fixed_main_loop 0.39, weather/map/ms 0.16, stream/tick/ms 0.12 |
| warm-up | 3.31 | 744.3 | main/busy/ms 2684, stage/main/post_update 2454, render/thread/ms 711, render/busy/ms 542, stage/render/prepare_assets 494, stage/main/update 215, stage/render/prepare_views 165, render/acquire/ms 165, surface/advance_cpu_ms 66.94, stage/render/create_views 27.67, stage/render/render 13.95, render/graph/ms 9.16 |
| warm-up | 3.48 | 174.3 | render/thread/ms 868, render/busy/ms 607, stage/render/prepare_assets 545, main/busy/ms 180, stage/render/prepare_views 165, render/acquire/ms 165, stage/main/update 124, stage/render/render 110, render/present/ms 95.79, stage/main/post_update 40.82, stage/render/create_views 28.00, pipelines/compile_ms 23.40 |
| warm-up | 3.68 | 174.4 | pipelines/wait_ms 5973, pipelines/compile_ms 2164, pipelines/front_ms 1901, pipelines/driver_ms 263, render/thread/ms 133, render/busy/ms 127, main/busy/ms 126, stage/render/prepare_assets 108, stage/main/update 61.20, pipelines/lock_ms 35.54, stage/main/post_update 27.07, stage/main/pre_update 26.39 |
| warm-up | 3.77 | 84.8 | pipelines/wait_ms 6547, pipelines/compile_ms 2990, pipelines/front_ms 2829, render/thread/ms 201, render/busy/ms 198, pipelines/driver_ms 161, stage/render/prepare_assets 133, main/busy/ms 79.54, stage/render/prepare_resources 45.49, pipelines/lock_ms 32.13, stage/main/pre_update 27.50, stage/main/post_update 26.60 |
| warm-up | 3.82 | 52.4 | pipelines/compile_ms 1178, pipelines/front_ms 1109, pipelines/wait_ms 585, render/thread/ms 132, render/busy/ms 128, pipelines/driver_ms 69.08, main/busy/ms 50.27, stage/render/prepare_assets 45.95, stage/render/prepare_resources 45.67, stage/render/render 28.08, stage/main/post_update 27.49, render/graph/ms 24.34 |
| warm-up | 3.88 | 47.5 | pipelines/compile_ms 1567, pipelines/front_ms 1493, pipelines/wait_ms 139, pipelines/driver_ms 74.14, render/thread/ms 55.39, render/busy/ms 53.89, stage/render/prepare_assets 37.06, main/busy/ms 26.14, stage/main/post_update 13.62, stage/render/render 10.87, terrain/update/ms 9.89, render/graph/ms 9.05 |
| warm-up | 3.97 | 69.6 | pipelines/compile_ms 506, pipelines/front_ms 487, main/busy/ms 63.99, stage/main/update 48.23, pipelines/wait_ms 45.74, stream/drive/ms 44.08, render/thread/ms 42.39, render/busy/ms 39.31, stage/render/prepare_assets 18.92, pipelines/driver_ms 18.33, stage/render/render 14.98, stage/main/post_update 11.82 |
| warm-up | 4.05 | 59.5 | pipelines/compile_ms 467, pipelines/front_ms 449, render/thread/ms 57.93, render/busy/ms 55.78, main/busy/ms 49.82, stage/render/prepare_assets 29.78, stage/main/update 24.85, stream/drive/ms 19.54, pipelines/driver_ms 17.67, stage/render/render 12.27, render/graph/ms 9.52, stage/main/post_update 8.70 |
| warm-up | 4.08 | 28.0 | pipelines/compile_ms 411, pipelines/front_ms 399, render/thread/ms 61.39, render/busy/ms 58.99, main/busy/ms 46.62, stage/render/prepare_assets 39.38, stage/main/update 27.03, stream/drive/ms 21.21, stage/render/render 15.68, render/graph/ms 12.58, pipelines/driver_ms 12.31, encode//sum 8.93 |
| warm-up | 4.15 | 68.0 | pipelines/compile_ms 763, pipelines/front_ms 737, render/thread/ms 60.30, render/busy/ms 58.63, main/busy/ms 56.95, stage/render/prepare_assets 38.82, stage/main/update 34.99, pipelines/driver_ms 25.86, stream/drive/ms 22.20, stage/render/render 13.82, stage/main/post_update 11.64, render/graph/ms 11.58 |
| warm-up | 4.21 | 66.6 | pipelines/compile_ms 1331, pipelines/front_ms 1284, main/busy/ms 122, stage/main/update 96.32, render/thread/ms 82.10, render/busy/ms 80.54, stream/drive/ms 60.10, stage/render/prepare_assets 59.62, pipelines/driver_ms 47.58, stage/main/post_update 13.66, stage/render/render 11.06, render/graph/ms 8.88 |
| warm-up | 4.34 | 126.5 | pipelines/compile_ms 1328, pipelines/front_ms 1282, main/busy/ms 114, render/thread/ms 93.40, render/busy/ms 91.79, stage/main/update 91.62, stage/render/prepare_assets 71.39, stream/drive/ms 59.27, pipelines/driver_ms 46.23, stage/main/post_update 12.40, stage/render/render 11.10, render/graph/ms 9.09 |
| warm-up | 4.43 | 90.9 | pipelines/compile_ms 1148, pipelines/front_ms 1080, pipelines/wait_ms 286, render/thread/ms 132, render/busy/ms 130, stage/render/prepare_assets 84.20, main/busy/ms 72.61, pipelines/driver_ms 67.35, stage/main/post_update 26.78, stage/render/render 25.12, stage/main/update 24.54, render/graph/ms 22.83 |
| warm-up | 4.53 | 95.5 | pipelines/compile_ms 793, pipelines/front_ms 738, pipelines/wait_ms 323, render/thread/ms 158, render/busy/ms 156, main/busy/ms 94.24, stage/render/prepare_assets 75.47, pipelines/driver_ms 55.78, stage/main/update 34.12, stage/main/post_update 33.34, stage/render/prepare_resources 25.96, stage/render/render 24.92 |
| warm-up | 4.57 | 44.5 | pipelines/compile_ms 1967, pipelines/front_ms 1904, pipelines/wait_ms 180, render/thread/ms 106, render/busy/ms 104, main/busy/ms 63.58, pipelines/driver_ms 63.29, stage/render/prepare_assets 50.29, stage/main/update 31.38, stage/render/prepare_resources 22.41, stage/main/post_update 16.53, stage/main/fixed_main_loop 10.79 |
| warm-up | 4.64 | 26.3 | pipelines/compile_ms 431, pipelines/front_ms 419, pipelines/wait_ms 96.60, render/thread/ms 40.94, render/busy/ms 39.31, main/busy/ms 28.21, stage/main/update 16.21, stage/render/render 12.85, pipelines/driver_ms 11.20, render/graph/ms 10.72, stage/render/prepare_assets 10.01, encode//sum 8.14 |
| warm-up | 4.67 | 30.1 | pipelines/compile_ms 386, pipelines/front_ms 376, pipelines/wait_ms 76.16, render/thread/ms 46.30, render/busy/ms 44.32, main/busy/ms 32.12, stage/render/prepare_assets 22.53, stage/main/update 19.97, stream/drive/ms 12.02, stage/render/render 10.56, pipelines/driver_ms 9.89, render/graph/ms 8.73 |
| warm-up | 4.74 | 67.6 | pipelines/compile_ms 399, pipelines/front_ms 388, pipelines/wait_ms 169, main/busy/ms 117, stage/main/update 103, stream/drive/ms 95.11, render/thread/ms 84.10, render/busy/ms 82.26, stage/render/prepare_assets 25.74, stage/render/prepare_write_phase_buffers 25.37, pipelines/driver_ms 10.97, stage/render/render 10.33 |
| warm-up | 4.81 | 68.7 | pipelines/compile_ms 1040, pipelines/front_ms 1007, pipelines/wait_ms 301, main/busy/ms 115, stage/main/update 98.96, stream/drive/ms 86.59, render/thread/ms 80.48, render/busy/ms 79.19, pipelines/driver_ms 33.42, stage/render/prepare_write_phase_buffers 25.83, stage/render/prepare_assets 22.23, stage/main/post_update 9.99 |
| warm-up | 4.84 | 34.9 | pipelines/compile_ms 1861, pipelines/front_ms 1803, pipelines/wait_ms 178, pipelines/driver_ms 57.69, render/thread/ms 51.77, render/busy/ms 50.27, main/busy/ms 49.79, stage/main/update 27.73, stage/render/prepare_assets 19.26, stage/render/render 13.39, render/graph/ms 10.99, stage/main/post_update 10.57 |
| warm-up | 4.93 | 61.8 | pipelines/compile_ms 866, pipelines/front_ms 836, main/busy/ms 68.68, render/thread/ms 53.06, render/busy/ms 51.14, stage/main/update 45.91, pipelines/driver_ms 29.92, stage/render/prepare_assets 21.85, pipelines/wait_ms 21.36, stage/render/render 15.62, render/graph/ms 12.97, stream/drive/ms 12.62 |
| warm-up | 4.97 | 37.4 | pipelines/compile_ms 234, pipelines/front_ms 224, render/thread/ms 61.29, render/busy/ms 59.93, main/busy/ms 47.71, stage/main/update 33.31, stage/render/prepare_assets 30.30, stream/drive/ms 12.14, stage/render/render 11.03, pipelines/driver_ms 9.42, render/graph/ms 9.26, stage/main/post_update 8.19 |
| warm-up | 5.02 | 25.1 | pipelines/compile_ms 869, pipelines/front_ms 846, pipelines/wait_ms 208, render/thread/ms 42.28, render/busy/ms 40.55, main/busy/ms 37.76, stage/main/update 24.84, pipelines/driver_ms 23.07, stage/render/render 12.98, render/graph/ms 10.99, stage/render/prepare_assets 8.88, stage/render/graph_submit 8.38 |
| warm-up | 5.07 | 29.6 | pipelines/compile_ms 355, pipelines/front_ms 341, render/thread/ms 47.41, main/busy/ms 46.17, render/busy/ms 44.69, stage/render/render 16.38, stage/main/post_update 14.23, render/graph/ms 13.95, stage/main/update 13.89, pipelines/driver_ms 13.13, stage/main/pre_update 11.12, stage/render/graph_submit 11.03 |
| warm-up | 5.11 | 44.8 | pipelines/compile_ms 488, pipelines/front_ms 471, render/thread/ms 63.27, render/busy/ms 60.48, main/busy/ms 54.82, stage/main/update 20.70, stage/render/render 19.15, render/graph/ms 16.34, pipelines/driver_ms 16.29, stage/main/post_update 15.03, stage/main/pre_update 11.59, stage/render/graph_submit 11.54 |
| warm-up | 5.15 | 32.3 | pipelines/compile_ms 186, pipelines/front_ms 180, render/thread/ms 62.19, render/busy/ms 60.72, main/busy/ms 37.37, stage/render/prepare_assets 20.10, stage/main/update 17.30, stage/render/render 14.62, render/graph/ms 12.33, stage/main/post_update 12.16, stage/render/prepare_bind_groups 8.11, stage/render/graph_submit 7.86 |
| warm-up | 5.19 | 40.8 | pipelines/compile_ms 568, pipelines/front_ms 553, render/thread/ms 64.21, render/busy/ms 62.66, stage/render/prepare_assets 40.86, main/busy/ms 35.46, stage/main/update 18.60, pipelines/driver_ms 14.65, stage/render/render 12.84, stream/drive/ms 12.69, render/graph/ms 10.69, stage/main/post_update 9.94 |
| warm-up | 5.22 | 29.8 | pipelines/compile_ms 1163, pipelines/front_ms 1132, main/busy/ms 71.78, render/thread/ms 64.11, render/busy/ms 62.48, pipelines/driver_ms 31.65, stage/render/prepare_assets 31.02, stage/main/post_update 29.22, stage/main/update 23.19, stage/render/render 18.91, render/graph/ms 16.56, bigspace/total/ms 15.09 |
| warm-up | 5.26 | 47.0 | pipelines/compile_ms 1143, pipelines/front_ms 1110, main/busy/ms 66.57, render/thread/ms 46.67, render/busy/ms 45.16, pipelines/driver_ms 32.98, stage/main/post_update 32.02, pipelines/wait_ms 25.32, stage/render/render 16.39, bigspace/total/ms 15.10, bigspace/high_precision/ms 14.67, stage/main/update 14.61 |
| warm-up | 5.30 | 41.4 | pipelines/compile_ms 1126, pipelines/front_ms 1092, render/thread/ms 50.29, render/busy/ms 48.71, pipelines/driver_ms 33.96, main/busy/ms 32.15, pipelines/wait_ms 25.02, stage/render/prepare_assets 19.07, stage/main/update 12.35, stage/main/post_update 11.92, stage/render/render 10.58, render/graph/ms 8.58 |
| warm-up | 5.36 | 29.2 | pipelines/compile_ms 70.91, pipelines/front_ms 65.44, render/thread/ms 44.19, render/busy/ms 42.91, main/busy/ms 39.82, stage/main/update 20.59, stage/render/prepare_assets 17.60, stage/main/post_update 12.18, stream/drive/ms 12.05, stage/render/render 10.91, render/graph/ms 9.06, encode//sum 6.92 |
| warm-up | 5.40 | 40.7 | pipelines/compile_ms 639, pipelines/front_ms 619, render/thread/ms 60.78, render/busy/ms 59.17, main/busy/ms 35.55, stage/render/prepare_assets 32.98, pipelines/driver_ms 20.02, stage/main/update 19.25, pipelines/wait_ms 14.84, stream/drive/ms 13.30, stage/render/render 12.07, stage/main/post_update 10.52 |
| warm-up | 5.45 | 42.5 | pipelines/compile_ms 858, pipelines/front_ms 840, render/thread/ms 42.72, render/busy/ms 41.28, main/busy/ms 30.98, pipelines/wait_ms 27.29, stage/render/prepare_assets 23.53, pipelines/driver_ms 17.97, stage/main/update 13.88, stage/render/render 11.91, stage/main/post_update 10.64, render/graph/ms 10.00 |
| warm-up | 5.49 | 33.5 | pipelines/compile_ms 617, pipelines/front_ms 604, render/thread/ms 56.57, render/busy/ms 55.43, stage/render/prepare_assets 38.97, main/busy/ms 27.69, pipelines/wait_ms 26.73, pipelines/driver_ms 13.01, stage/main/update 12.56, stage/main/post_update 9.14, stage/render/render 8.35, render/graph/ms 6.85 |
| warm-up | 5.52 | 28.9 | pipelines/compile_ms 199, pipelines/front_ms 195, render/thread/ms 50.57, render/busy/ms 49.26, stage/render/prepare_assets 31.53, main/busy/ms 22.86, stage/main/update 10.54, stage/render/render 9.15, render/graph/ms 7.45, stage/main/post_update 6.97, encode//sum 6.20, stage/render/graph_submit 5.28 |
| warm-up | 5.56 | 40.8 | render/thread/ms 37.10, render/busy/ms 36.03, main/busy/ms 28.66, stage/render/prepare_assets 19.89, stage/main/update 15.77, stage/render/render 9.43, encode//sum 7.93, render/graph/ms 7.85, stage/main/post_update 7.85, stream/drive/ms 5.44, stage/render/graph_submit 4.81, stage/render/camera_driver 2.90 |
| warm-up | 5.61 | 28.4 | pipelines/compile_ms 152, pipelines/front_ms 148, render/thread/ms 43.31, render/busy/ms 42.00, main/busy/ms 26.18, stage/render/prepare_assets 21.20, stage/main/update 11.19, stage/render/render 10.01, stage/main/post_update 9.77, render/graph/ms 8.23, encode//sum 7.36, stage/render/graph_submit 5.50 |
| warm-up | 5.64 | 29.8 | pipelines/compile_ms 152, pipelines/front_ms 148, render/thread/ms 52.33, render/busy/ms 50.70, main/busy/ms 28.73, stage/render/prepare_assets 26.94, stage/main/update 13.88, stage/render/render 11.01, stage/main/post_update 9.73, render/graph/ms 8.66, encode//sum 8.07, stage/render/graph_submit 5.63 |
| warm-up | 5.70 | 32.2 | pipelines/compile_ms 479, pipelines/front_ms 469, render/thread/ms 39.30, render/busy/ms 38.26, main/busy/ms 26.67, stage/render/prepare_assets 20.24, stage/main/update 11.19, stage/main/post_update 9.89, pipelines/driver_ms 9.73, stage/render/render 7.43, render/graph/ms 6.07, encode//sum 5.93 |
| warm-up | 5.80 | 46.5 | pipelines/compile_ms 188, pipelines/front_ms 180, main/busy/ms 47.90, render/thread/ms 41.06, render/busy/ms 39.29, stage/main/update 30.01, pipelines/wait_ms 16.42, pipelines/lock_ms 15.68, stage/render/prepare_assets 14.74, stage/render/render 14.66, stage/main/post_update 12.89, render/graph/ms 12.04 |
| warm-up | 5.89 | 51.5 | main/busy/ms 50.62, render/thread/ms 40.47, render/busy/ms 39.03, stage/main/update 27.45, stream/drive/ms 18.13, stage/render/prepare_assets 17.78, stage/main/post_update 16.05, stage/render/render 11.15, render/graph/ms 8.99, encode//sum 7.18, stage/render/graph_submit 5.54, encode/submit 3.22 |
| warm-up | 5.93 | 41.0 | render/thread/ms 57.50, render/busy/ms 55.34, main/busy/ms 50.38, stage/main/update 23.41, stage/render/prepare_assets 21.96, stage/main/post_update 18.26, stage/render/render 16.57, pipelines/compile_ms 14.60, render/graph/ms 13.64, pipelines/front_ms 13.42, stream/drive/ms 9.41, stage/render/graph_submit 9.41 |
| warm-up | 5.96 | 28.9 | pipelines/compile_ms 1428, pipelines/front_ms 1394, render/thread/ms 50.11, render/busy/ms 48.15, main/busy/ms 44.97, pipelines/driver_ms 34.21, pipelines/wait_ms 30.24, pipelines/lock_ms 29.39, stage/main/update 22.98, stage/render/prepare_assets 19.41, stage/render/render 13.82, stage/main/post_update 13.10 |
| warm-up | 5.99 | 25.5 | pipelines/compile_ms 1432, pipelines/front_ms 1398, render/thread/ms 38.39, render/busy/ms 37.14, pipelines/driver_ms 33.76, main/busy/ms 33.45, pipelines/wait_ms 30.68, pipelines/lock_ms 29.80, stage/render/prepare_assets 17.06, stage/main/update 15.89, stage/main/post_update 10.48, stage/render/render 9.04 |
| warm-up | 6.13 | 37.2 | pipelines/compile_ms 406, pipelines/front_ms 391, main/busy/ms 47.96, render/thread/ms 34.98, render/busy/ms 33.56, stage/main/update 29.30, stream/drive/ms 20.56, pipelines/driver_ms 14.65, stage/render/render 14.21, render/graph/ms 12.31, stage/main/post_update 11.47, stage/render/prepare_assets 9.44 |
| warm-up | 6.23 | 32.0 | pipelines/compile_ms 363, pipelines/front_ms 355, main/busy/ms 33.60, render/thread/ms 28.95, render/busy/ms 27.87, stage/main/update 17.21, stage/render/prepare_assets 13.09, stage/main/post_update 11.18, stage/render/render 10.38, render/graph/ms 8.90, pipelines/driver_ms 8.19, encode//sum 8.04 |
| warm-up | 6.48 | 26.1 | main/busy/ms 42.34, render/thread/ms 40.78, render/busy/ms 38.74, stage/main/update 19.38, stage/render/render 17.07, stage/main/post_update 16.67, render/graph/ms 14.12, stage/render/prepare_assets 12.23, encode//sum 11.93, stream/drive/ms 10.37, stage/render/graph_submit 10.25, encode/submit 6.69 |
| warm-up | 6.53 | 44.9 | render/thread/ms 49.19, render/busy/ms 47.80, main/busy/ms 42.70, stage/render/prepare_assets 28.29, stage/main/update 20.16, stage/main/post_update 15.67, stage/render/render 10.75, stream/drive/ms 10.22, encode//sum 8.88, render/graph/ms 8.67, stage/render/graph_submit 5.64, encode/submit 2.95 |
| warm-up | 6.87 | 32.7 | render/thread/ms 39.37, render/busy/ms 38.40, main/busy/ms 14.34, stage/render/render 9.77, stage/render/specialize 9.65, render/graph/ms 8.45, encode//sum 7.68, stage/main/post_update 7.38, stage/render/queue 6.86, stage/render/graph_submit 6.47, stage/main/update 4.64, stage/render/prepare_meshes 3.69 |
| warm-up | 8.09 | 30.8 | render/thread/ms 37.10, render/busy/ms 34.66, main/busy/ms 15.20, stage/render/render 10.68, render/graph/ms 9.23, stage/render/specialize 8.64, stage/main/post_update 8.05, stage/render/graph_submit 7.20, stage/render/queue 6.37, encode//sum 6.19, stage/main/update 4.42, encode/submit 4.16 |
| measure | 0.84 | 27.1 | render/thread/ms 34.04, render/busy/ms 31.45, main/busy/ms 13.82, stage/render/render 9.53, stage/render/specialize 8.46, render/graph/ms 8.29, stage/main/post_update 7.05, stage/render/queue 6.38, stage/render/graph_submit 6.31, encode//sum 5.67, stage/main/update 3.95, encode/submit 3.76 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set select.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=select-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 10%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **97%** (min 90%) · 2625 MHz (min 2625) · memory 8101 MHz · 98.4 W · 78 °C (max 81) · VRAM 6430 MiB · throttled 100% of samples

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
