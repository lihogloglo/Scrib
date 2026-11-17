# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1518 frames over 15.0 s · **median 9.91 ms (101 fps)** · mean 9.89 · p95 11.21 · p99 11.79 (1% low 85 fps) · worst 28.48 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 281 frames over 8.4 s, worst 2480.6 ms
- hitches (> 25.0 ms): **1** measured, 62 in warm-up
- timed GPU passes: 8.57 ms of a 9.89 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.91 ms, limit 11 ms
- pass worst frame 28.48 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.891 | 2.898 | 2.965 | 1.00 | 0.068 |
| dlss_super_resolution | 2.013 | 2.019 | 2.043 | 1.00 | 0.183 |
| early prepass | 0.600 | 0.601 | 0.610 | 1.00 | 0.091 |
| atmosphere_luts | 0.328 | 0.329 | 0.394 | 1.00 | 0.003 |
| bloom | 0.315 | 0.315 | 0.331 | 1.00 | 0.062 |
| main_transmissive_pass_3d | 0.309 | 0.309 | 0.331 | 1.00 | 0.009 |
| water_velocity | 0.238 | 0.239 | 0.263 | 1.00 | 0.002 |
| water_sspr | 0.210 | 0.211 | 0.213 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.148 | 0.149 | 0.169 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.147 | 0.153 | 1.00 | 0.001 |
| uniform_allocation | 0.143 | 0.144 | 0.195 | 1.00 | 0.023 |
| water_fft | 0.132 | 0.132 | 0.135 | 1.00 | 0.003 |
| upscaling | 0.117 | 0.117 | 0.123 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.112 | 1.00 | 0.002 |
| bin_unpacking | 0.101 | 0.102 | 0.135 | 1.00 | 0.016 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.075 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.414 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.035 | 0.036 | 0.052 | 1.00 | 0.005 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 1.00 | 0.001 |
| main_transparent_pass_3d | 0.028 | 0.028 | 0.031 | 1.00 | 0.134 |
| lightprobe_radiance_map | 0.028 | 0.221 | 0.232 | 0.12 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.043 | 0.045 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.025 | 1.00 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| surface_exposure | 0.012 | 0.041 | 0.043 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.841 | 10.208 | 27.20 | 13420.3 |
| render/busy/ms | 7.215 | 8.253 | 26.63 | 10951.9 |
| main/busy/ms | 7.073 | 7.941 | 10.55 | 10737.3 |
| stage/render/render | 4.789 | 5.630 | 8.64 | 7270.1 |
| render/graph/ms | 4.178 | 4.989 | 6.35 | 6341.8 |
| encode//sum | 4.138 | 5.512 | 9.15 | 6281.3 |
| stage/main/post_update | 4.095 | 4.834 | 7.23 | 6216.3 |
| stage/render/graph_submit | 3.123 | 3.879 | 5.26 | 4741.4 |
| encode/submit | 1.938 | 2.381 | 5.00 | 2941.8 |
| stage/main/update | 1.844 | 2.256 | 3.19 | 2799.5 |
| stage/render/prepare_views | 1.311 | 2.931 | 4.52 | 1990.5 |
| render/acquire/ms | 1.268 | 2.868 | 4.46 | 1924.4 |
| encode/finish_wall | 1.209 | 1.687 | 2.38 | 1834.7 |
| stage/render/camera_driver | 0.998 | 1.126 | 1.62 | 1515.0 |
| encode/pass:shadow_directional_light/sum | 0.771 | 1.330 | 2.28 | 1169.9 |
| encode/pass:early prepass/sum | 0.740 | 1.030 | 1.38 | 1123.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.721 | 1.030 | 1.52 | 1094.8 |
| stage/render/prepare_write_phase_buffers | 0.583 | 0.824 | 1.06 | 885.7 |
| stage/render/prepare_assets | 0.453 | 0.580 | 1.58 | 688.2 |
| stage/render/prepare_resources | 0.387 | 0.447 | 0.60 | 587.4 |
| bigspace/total/ms | 0.366 | 0.885 | 1.75 | 555.3 |
| render/present/ms | 0.358 | 0.424 | 2.52 | 544.0 |
| stage/main/pre_update | 0.353 | 0.481 | 0.90 | 535.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.317 | 0.465 | 0.67 | 481.3 |
| stage/main/first | 0.291 | 0.351 | 1.96 | 442.2 |
| terrain/update/ms | 0.233 | 0.300 | 0.64 | 353.0 |
| stage/render/prepare_batch_phases | 0.230 | 0.302 | 0.44 | 348.4 |
| stage/render/prepare_meshes | 0.221 | 0.278 | 1.60 | 336.1 |
| stage/render/prepare_bind_groups | 0.212 | 0.297 | 0.44 | 321.6 |
| terrain/select/ms | 0.203 | 0.262 | 0.61 | 308.0 |
| stream/drive/ms | 0.191 | 0.254 | 0.73 | 289.8 |
| stage/main/fixed_main_loop | 0.186 | 0.338 | 1.33 | 282.4 |
| stage/render/extract_commands | 0.170 | 0.201 | 0.45 | 258.3 |
| stage/render/create_views | 0.169 | 0.217 | 0.66 | 256.7 |
| far/select/ms | 0.169 | 0.228 | 0.65 | 256.2 |
| stream/tick/ms | 0.163 | 0.223 | 0.71 | 247.3 |
| stage/render/prepare_flush | 0.158 | 0.319 | 0.65 | 239.7 |
| controller/feed/ms | 0.149 | 0.196 | 0.68 | 225.8 |
| controller/move/ms | 0.140 | 0.289 | 0.64 | 212.9 |
| bigspace/high_precision/ms | 0.133 | 0.345 | 1.42 | 202.3 |
| stage/main/last | 0.121 | 0.165 | 0.90 | 183.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.120 | 0.180 | 0.36 | 182.5 |
| stage/render/queue | 0.113 | 0.141 | 6.45 | 171.5 |
| stage/render/cleanup | 0.105 | 0.160 | 0.29 | 159.8 |
| stage/render/specialize | 0.095 | 0.119 | 9.34 | 144.6 |
| bigspace/low_precision/ms | 0.087 | 0.201 | 1.54 | 132.3 |
| anim/evaluate/ms | 0.086 | 0.202 | 1.00 | 130.8 |
| anim/evaluate_cpu/ms | 0.061 | 0.075 | 0.44 | 93.3 |
| stage/render/prepare_collect_phase_buffers | 0.060 | 0.118 | 0.24 | 90.8 |
| encode/resolve_diagnostics/sum | 0.057 | 0.081 | 0.27 | 86.4 |
| physics/step/ms | 0.027 | 0.053 | 0.47 | 41.2 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.039 | 0.09 | 40.9 |
| weather/map/ms | 0.025 | 0.033 | 0.08 | 37.6 |
| stage/render/phase_sort | 0.024 | 0.040 | 0.08 | 36.9 |
| encode/camera_driver_span_end/sum | 0.024 | 0.037 | 0.21 | 36.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.04 | 16.4 |
| script/update/ms | 0.007 | 0.009 | 0.40 | 10.7 |
| controller/avoid/ms | 0.006 | 0.008 | 0.04 | 9.1 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 8.1 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 6.8 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.0 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 1.1 |
| stage/render/prepare | 0.001 | 0.002 | 0.05 | 0.8 |
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
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.56 | 1.94 | 0.61 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.09 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.21 | 0.03 |
| encode/finish_critical | ms | 0.88 | 2.28 | 0.96 |
| encode/pass:early prepass/max | ms | 0.74 | 1.38 | 0.83 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.72 | 1.52 | 0.96 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.36 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.30 | 0.67 | 0.22 |
| encode/pass:shadow_directional_light/max | ms | 0.72 | 2.28 | 0.77 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.27 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.23 | 26.63 | 9.23 |
| gpu/timed/ms | ms | 9.23 | 10.49 | 9.23 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1943 | 1943 | 1943 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 207.2 k | 207.2 k | 207.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 13.3 k | 13.3 k | 13.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 485.99 M | 485.99 M | 485.99 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d/bins | count | 330 | 330 | 330 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3189 | 3189 | 3189 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 330 | 330 | 330 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3189 | 3189 | 3189 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 21 | 21 | 21 |
| phases/opaque3d/bins | count | 3895 | 3895 | 3895 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 9283 | 9283 | 9283 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/bins | count | 3895 | 3895 | 3895 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 9283 | 9283 | 9283 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 4 | 34 | 4 |
| phases/shadow/bins | count | 10.2 k | 12.9 k | 10.2 k |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 31.2 k | 42.9 k | 31.2 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 27 | 34 | 27 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 321 | 321 | 321 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 0 | 64 | 0 |
| preprocess/bind_groups_created | count | 0 | 64 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 137 | 137 | 137 |
| process/cpu_usage | % | 25.79 | 36.06 | 18.48 |
| process/mem_usage | GiB | 4.69 | 4.77 | 4.77 |
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
| system/cpu_usage | % | 31.16 | 41.46 | 27.08 |
| system/mem_usage | % | 54.50 | 54.75 | 54.74 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 58 | 88 | 58 |
| views/all/bins | count | 18.6 k | 21.4 k | 18.6 k |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 56.2 k | 67.8 k | 56.2 k |
| views/all/pipelines | count | 77 | 84 | 77 |
| views/all/pipelines_distinct | count | 65 | 65 | 65 |
| views/all/work_items | count | 34.7 k | 68.3 k | 34.7 k |
| views/cascade/batch_sets | count | 4 | 34 | 4 |
| views/cascade/bins | count | 10.2 k | 12.9 k | 10.2 k |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 31.2 k | 42.9 k | 31.2 k |
| views/cascade/pipelines | count | 27 | 34 | 27 |
| views/cascade/work_items | count | 9346 | 42.9 k | 9346 |
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
| views/main/batch_sets | count | 54 | 54 | 54 |
| views/main/bins | count | 8450 | 8450 | 8450 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 24.9 k | 24.9 k | 24.9 k |
| views/main/pipelines | count | 50 | 50 | 50 |
| views/main/work_items | count | 25.4 k | 25.4 k | 25.4 k |
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
| assets/StandardMaterial/modified | count | 3.26 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 29 | 40.3 k |
| encode/bind_walk/init_textures | count | 7181 | 14.0 k | 10.90 M |
| encode/bind_walk/init_textures_skipped | count | 47.9 k | 62.4 k | 72.68 M |
| encode/bind_walk/inits | count | 422 | 515 | 640.4 k |
| encode/bind_walk/inits_skipped | count | 110 | 123 | 166.2 k |
| encode/bind_walk/merge_textures | count | 7117 | 13.9 k | 10.80 M |
| encode/bind_walk/merge_textures_skipped | count | 4225 | 4225 | 6.41 M |
| encode/bind_walk/merges | count | 84.27 | 105 | 127.9 k |
| encode/bind_walk/merges_skipped | count | 87.00 | 93 | 132.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1518 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1518 |
| encode/pass:early prepass/count | count | 1 | 1 | 1518 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1518 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1518 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1518 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1518 |
| encode/queue/command_buffers | count | 36.57 | 39 | 55.5 k |
| encode/queue/submits | count | 2 | 2 | 3036 |
| encode/queue/write_buffer/bytes | bytes | 1.84 M | 4.77 M | 2.80 G |
| encode/queue/write_buffer/calls | count | 233 | 306 | 353.8 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 49.55 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.7 k |
| encode/queue/write_texture/bytes | bytes | 11.8 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1518 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.59 | 2 | 900 |
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

## Hitches (> 25.0 ms; slowest 63, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.48 | 2480.6 | main/busy/ms 2452, stage/main/post_update 2347, stage/main/update 95.10, surface/advance_cpu_ms 66.23, stage/main/pre_update 5.54, stage/main/last 1.02, surface/cell_upload_ms 0.70, stage/main/first 0.54, stream/drive/ms 0.45, stage/main/fixed_main_loop 0.35, stream/tick/ms 0.12, weather/map/ms 0.10 |
| warm-up | 3.21 | 733.6 | main/busy/ms 2585, stage/main/post_update 2370, render/thread/ms 705, render/busy/ms 547, stage/render/prepare_assets 500, stage/main/update 200, stage/render/prepare_views 155, render/acquire/ms 154, surface/advance_cpu_ms 66.23, stage/render/create_views 28.34, stage/render/render 12.28, render/graph/ms 8.11 |
| warm-up | 3.32 | 104.9 | render/thread/ms 793, render/busy/ms 585, stage/render/prepare_assets 523, stage/render/prepare_views 155, render/acquire/ms 155, main/busy/ms 152, stage/main/update 109, stage/render/render 68.52, render/present/ms 52.96, pipelines/compile_ms 44.10, pipelines/front_ms 39.53, stage/main/post_update 32.95 |
| warm-up | 3.36 | 36.5 | pipelines/compile_ms 253, pipelines/front_ms 229, render/thread/ms 107, stage/render/render 67.98, main/busy/ms 64.61, render/present/ms 55.77, render/busy/ms 50.17, stage/main/post_update 48.59, pipelines/driver_ms 24.95, stage/render/prepare_assets 24.03, terrain/update/ms 22.15, pipelines/wait_ms 18.72 |
| warm-up | 3.46 | 102.2 | pipelines/wait_ms 3054, pipelines/compile_ms 1006, pipelines/front_ms 946, render/thread/ms 103, render/busy/ms 95.71, main/busy/ms 72.24, pipelines/driver_ms 60.08, stage/main/post_update 51.12, stage/render/prepare_flush 28.89, stage/render/render 17.94, stage/render/prepare_assets 16.16, stage/render/prepare_write_phase_buffers 15.95 |
| warm-up | 3.56 | 99.7 | pipelines/wait_ms 6107, pipelines/compile_ms 1859, pipelines/front_ms 1758, render/thread/ms 174, render/busy/ms 170, main/busy/ms 124, pipelines/driver_ms 101, stage/main/update 73.64, stage/render/prepare_write_phase_buffers 52.74, stage/render/prepare_assets 43.00, stage/main/post_update 29.41, stage/render/prepare_flush 29.07 |
| warm-up | 3.64 | 80.7 | pipelines/wait_ms 3205, pipelines/compile_ms 2086, pipelines/front_ms 1995, render/thread/ms 156, render/busy/ms 153, main/busy/ms 118, pipelines/driver_ms 90.97, stage/render/prepare_assets 85.99, stage/main/update 80.30, stage/render/prepare_write_phase_buffers 37.35, stage/main/post_update 24.95, terrain/update/ms 16.87 |
| warm-up | 3.68 | 42.1 | pipelines/compile_ms 1909, pipelines/front_ms 1843, pipelines/wait_ms 188, render/thread/ms 108, render/busy/ms 106, stage/render/prepare_assets 84.78, pipelines/driver_ms 65.86, main/busy/ms 41.33, stage/main/update 17.81, encode//sum 17.09, stage/main/post_update 16.96, stage/render/render 13.62 |
| warm-up | 3.71 | 26.2 | pipelines/compile_ms 1212, pipelines/front_ms 1153, pipelines/wait_ms 148, render/thread/ms 64.92, render/busy/ms 62.95, pipelines/driver_ms 59.43, stage/render/prepare_assets 43.94, main/busy/ms 28.64, encode//sum 17.51, stage/render/render 12.96, stage/main/post_update 12.72, stage/main/update 11.06 |
| warm-up | 3.81 | 32.6 | main/busy/ms 53.34, pipelines/compile_ms 51.91, pipelines/front_ms 49.90, render/thread/ms 49.78, render/busy/ms 48.06, stage/main/update 41.67, stream/drive/ms 38.66, stage/render/prepare_assets 31.21, stage/render/render 11.79, render/graph/ms 9.66, stage/main/post_update 7.90, stage/render/graph_submit 7.27 |
| warm-up | 3.86 | 51.9 | pipelines/compile_ms 1154, pipelines/front_ms 1106, render/thread/ms 63.07, render/busy/ms 61.46, main/busy/ms 51.15, pipelines/driver_ms 47.51, stage/render/prepare_assets 45.78, stage/main/update 42.36, stream/drive/ms 38.68, pipelines/wait_ms 20.70, stage/render/render 11.15, render/graph/ms 9.20 |
| warm-up | 3.93 | 52.6 | pipelines/compile_ms 312, pipelines/front_ms 301, render/thread/ms 48.68, render/busy/ms 46.50, main/busy/ms 36.12, stage/render/prepare_assets 25.30, stage/main/update 22.71, stream/drive/ms 18.57, stage/render/render 17.00, render/graph/ms 14.52, pipelines/driver_ms 11.38, stage/render/graph_submit 9.66 |
| warm-up | 3.96 | 31.3 | pipelines/compile_ms 468, pipelines/front_ms 450, main/busy/ms 72.66, render/thread/ms 57.76, render/busy/ms 55.68, stage/main/update 55.47, stream/drive/ms 49.72, stage/render/prepare_assets 37.44, pipelines/driver_ms 17.66, stage/render/render 12.72, render/graph/ms 9.90, stage/main/post_update 7.75 |
| warm-up | 4.08 | 123.4 | pipelines/compile_ms 1149, pipelines/front_ms 1109, main/busy/ms 96.97, stage/main/update 78.90, render/thread/ms 76.69, render/busy/ms 75.01, stage/render/prepare_assets 58.32, stream/drive/ms 49.78, pipelines/driver_ms 40.42, stage/render/render 10.35, stage/main/post_update 8.28, render/graph/ms 8.03 |
| warm-up | 4.14 | 62.2 | pipelines/compile_ms 1044, pipelines/front_ms 1008, render/thread/ms 106, render/busy/ms 104, stage/render/prepare_assets 83.27, main/busy/ms 70.14, stage/main/update 51.71, pipelines/driver_ms 36.62, stream/drive/ms 19.36, stage/render/render 13.22, render/graph/ms 10.82, stage/main/post_update 9.50 |
| warm-up | 4.18 | 38.2 | pipelines/compile_ms 116, pipelines/front_ms 111, render/thread/ms 83.88, render/busy/ms 81.51, stage/render/prepare_assets 56.11, main/busy/ms 51.48, stage/main/update 32.69, stream/drive/ms 19.84, stage/render/render 15.40, render/graph/ms 12.88, stage/main/post_update 9.68, stage/render/graph_submit 9.03 |
| warm-up | 4.26 | 79.6 | pipelines/compile_ms 457, pipelines/front_ms 439, pipelines/wait_ms 168, render/thread/ms 85.57, render/busy/ms 84.11, main/busy/ms 65.81, stage/render/prepare_assets 44.93, stage/main/pre_update 28.21, stage/main/update 20.61, pipelines/driver_ms 17.98, stage/render/render 13.15, stage/main/post_update 12.99 |
| warm-up | 4.30 | 41.3 | pipelines/compile_ms 1297, pipelines/front_ms 1250, pipelines/wait_ms 174, render/thread/ms 87.09, render/busy/ms 85.75, main/busy/ms 76.69, pipelines/driver_ms 46.53, stage/render/prepare_assets 44.86, stage/main/pre_update 27.87, stage/main/post_update 22.90, stage/main/update 17.58, stage/render/render 10.95 |
| warm-up | 4.35 | 50.3 | pipelines/compile_ms 876, pipelines/front_ms 845, render/thread/ms 76.68, render/busy/ms 75.07, main/busy/ms 64.84, stage/main/post_update 40.24, stage/render/prepare_assets 33.66, pipelines/driver_ms 30.68, stage/render/prepare_batch_phases 14.15, stage/main/update 13.51, stage/render/render 9.02, render/graph/ms 7.19 |
| warm-up | 4.40 | 48.9 | pipelines/compile_ms 824, pipelines/front_ms 794, render/thread/ms 81.11, render/busy/ms 79.39, main/busy/ms 64.30, stage/render/prepare_assets 39.51, stage/main/post_update 37.68, pipelines/driver_ms 29.89, pipelines/wait_ms 23.03, stage/main/update 17.27, stage/render/prepare_batch_phases 14.30, stage/render/render 9.38 |
| warm-up | 4.46 | 61.3 | pipelines/compile_ms 1544, pipelines/front_ms 1492, pipelines/wait_ms 123, render/thread/ms 79.80, render/busy/ms 77.73, main/busy/ms 53.51, pipelines/driver_ms 51.58, stage/render/prepare_assets 47.40, stage/main/post_update 22.51, stage/main/update 22.41, stage/render/render 13.84, render/graph/ms 10.91 |
| warm-up | 4.51 | 42.8 | pipelines/compile_ms 1665, pipelines/front_ms 1618, pipelines/wait_ms 354, render/thread/ms 70.66, render/busy/ms 68.51, pipelines/driver_ms 47.35, main/busy/ms 43.70, stage/render/prepare_assets 38.54, stage/main/update 20.41, stage/main/post_update 15.32, stage/render/render 14.42, render/graph/ms 11.39 |
| warm-up | 4.55 | 38.5 | pipelines/compile_ms 1186, pipelines/front_ms 1155, pipelines/wait_ms 370, main/busy/ms 74.14, stage/main/update 58.34, render/thread/ms 53.05, render/busy/ms 51.55, stream/drive/ms 49.61, pipelines/driver_ms 30.94, stage/render/prepare_assets 30.61, stage/render/render 9.87, stage/main/post_update 7.85 |
| warm-up | 4.60 | 55.9 | pipelines/compile_ms 802, pipelines/front_ms 780, main/busy/ms 120, pipelines/wait_ms 117, stage/main/update 96.39, stream/drive/ms 88.87, render/thread/ms 43.65, render/busy/ms 42.31, pipelines/driver_ms 22.14, stage/render/prepare_assets 21.77, stage/main/post_update 17.43, stage/render/render 10.32 |
| warm-up | 4.69 | 92.5 | pipelines/compile_ms 1110, pipelines/front_ms 1066, main/busy/ms 95.81, render/thread/ms 62.07, render/busy/ms 60.75, stage/main/update 51.05, pipelines/driver_ms 44.17, stream/drive/ms 43.88, stage/main/post_update 33.44, stage/render/prepare_resources 16.68, stage/render/prepare_assets 14.89, bigspace/total/ms 12.26 |
| warm-up | 4.75 | 54.1 | pipelines/compile_ms 1662, pipelines/front_ms 1598, render/thread/ms 89.63, render/busy/ms 88.28, pipelines/wait_ms 85.50, main/busy/ms 81.33, pipelines/driver_ms 63.88, stage/main/update 42.94, stage/render/prepare_resources 38.44, stage/main/post_update 24.03, stage/render/prepare_assets 16.63, bigspace/total/ms 12.31 |
| warm-up | 4.81 | 66.4 | pipelines/compile_ms 1932, pipelines/front_ms 1880, render/thread/ms 96.37, render/busy/ms 94.90, pipelines/wait_ms 86.48, main/busy/ms 72.28, stage/main/update 52.27, pipelines/driver_ms 52.09, stage/render/prepare_assets 37.14, stage/render/prepare_resources 23.02, stream/drive/ms 13.83, stage/render/render 12.50 |
| warm-up | 4.87 | 28.0 | pipelines/compile_ms 753, pipelines/front_ms 734, pipelines/wait_ms 72.70, render/thread/ms 42.20, render/busy/ms 40.53, main/busy/ms 33.12, pipelines/driver_ms 18.78, stage/main/update 17.08, stage/render/render 11.74, stage/render/prepare_assets 11.18, stage/main/post_update 10.35, render/graph/ms 9.91 |
| warm-up | 4.94 | 27.7 | pipelines/compile_ms 526, pipelines/front_ms 510, main/busy/ms 41.04, render/thread/ms 38.95, render/busy/ms 37.45, pipelines/wait_ms 30.46, stage/main/update 26.28, pipelines/driver_ms 16.15, stream/drive/ms 16.07, stage/render/prepare_assets 10.07, stage/render/render 10.01, stage/main/post_update 8.39 |
| warm-up | 4.97 | 28.7 | pipelines/compile_ms 509, pipelines/front_ms 494, render/thread/ms 44.10, render/busy/ms 42.74, main/busy/ms 35.11, pipelines/wait_ms 29.87, stage/main/update 18.88, pipelines/driver_ms 14.51, stage/render/render 9.78, stage/render/prepare_assets 9.46, stage/main/post_update 9.38, render/graph/ms 8.01 |
| warm-up | 5.01 | 42.6 | pipelines/compile_ms 207, pipelines/front_ms 200, render/thread/ms 63.05, render/busy/ms 61.42, main/busy/ms 47.87, stage/main/post_update 24.24, stage/main/update 15.90, stage/render/render 13.83, stage/render/prepare_resources 13.44, stage/render/prepare_assets 13.34, render/graph/ms 11.67, encode//sum 8.66 |
| warm-up | 5.11 | 79.8 | pipelines/compile_ms 1543, pipelines/front_ms 1502, main/busy/ms 61.48, render/thread/ms 59.91, render/busy/ms 57.96, pipelines/driver_ms 40.77, stage/main/update 34.68, stage/render/prepare_assets 30.58, stage/main/post_update 20.02, stream/drive/ms 15.96, stage/render/render 14.05, stream/spawn/ms 13.53 |
| warm-up | 5.15 | 31.8 | pipelines/compile_ms 1528, pipelines/front_ms 1489, render/thread/ms 66.59, render/busy/ms 65.11, main/busy/ms 39.22, pipelines/driver_ms 39.04, stage/render/prepare_assets 37.78, stage/main/post_update 16.49, stage/main/update 15.98, stage/render/render 12.90, render/graph/ms 10.79, encode//sum 8.05 |
| warm-up | 5.17 | 28.3 | pipelines/compile_ms 745, pipelines/front_ms 730, render/thread/ms 46.64, render/busy/ms 45.30, main/busy/ms 28.34, stage/render/prepare_assets 19.93, pipelines/driver_ms 14.97, stage/main/update 13.60, stage/render/render 11.18, render/graph/ms 9.41, stage/main/post_update 8.43, encode//sum 7.77 |
| warm-up | 5.20 | 25.2 | pipelines/compile_ms 657, pipelines/front_ms 642, render/thread/ms 44.74, render/busy/ms 37.26, main/busy/ms 33.50, stage/main/update 18.08, pipelines/driver_ms 15.58, stage/render/prepare_assets 13.00, stage/render/render 11.26, stage/main/post_update 9.97, render/graph/ms 9.43, encode//sum 7.92 |
| warm-up | 5.26 | 31.2 | render/thread/ms 38.88, render/busy/ms 37.02, main/busy/ms 32.17, stage/main/update 18.22, stage/render/prepare_assets 13.56, stage/render/render 12.73, stream/drive/ms 12.24, render/graph/ms 10.28, stage/main/post_update 9.12, stage/render/graph_submit 7.47, encode//sum 7.15, encode/submit 5.23 |
| warm-up | 5.31 | 30.7 | pipelines/compile_ms 733, pipelines/front_ms 713, render/thread/ms 44.60, render/busy/ms 43.33, main/busy/ms 26.96, pipelines/driver_ms 20.18, stage/render/prepare_assets 17.99, stage/main/update 11.59, stage/main/post_update 9.77, stage/render/render 8.53, render/graph/ms 6.98, encode//sum 6.66 |
| warm-up | 5.36 | 25.1 | pipelines/compile_ms 721, pipelines/front_ms 702, render/thread/ms 41.33, render/busy/ms 39.82, main/busy/ms 29.61, pipelines/driver_ms 18.62, stage/render/prepare_assets 16.10, stage/main/update 14.35, stage/render/render 10.58, stage/main/post_update 9.46, render/graph/ms 8.58, encode//sum 7.76 |
| warm-up | 5.38 | 25.0 | pipelines/compile_ms 721, pipelines/front_ms 702, render/thread/ms 44.46, render/busy/ms 43.18, main/busy/ms 34.11, stage/render/prepare_assets 25.29, pipelines/driver_ms 18.62, stage/main/update 17.64, stage/main/post_update 9.80, stage/render/render 9.05, render/graph/ms 7.26, encode//sum 6.92 |
| warm-up | 5.42 | 37.2 | render/thread/ms 54.99, render/busy/ms 53.63, stage/render/prepare_assets 32.76, main/busy/ms 31.99, stage/main/update 15.61, stage/main/post_update 9.34, stage/render/render 9.01, render/graph/ms 7.21, encode//sum 6.80, stream/drive/ms 5.69, stage/render/graph_submit 4.87, stage/render/specialize 4.13 |
| warm-up | 5.47 | 28.3 | render/thread/ms 44.19, render/busy/ms 42.66, main/busy/ms 26.51, stage/render/prepare_assets 15.28, stage/main/update 12.69, stage/render/render 12.24, render/graph/ms 10.14, encode//sum 9.49, stage/main/post_update 8.90, stage/render/graph_submit 6.67, stream/drive/ms 3.99, encode/submit 3.98 |
| warm-up | 5.50 | 29.0 | render/thread/ms 52.02, render/busy/ms 50.20, main/busy/ms 31.71, stage/render/prepare_assets 18.31, stage/render/render 15.91, render/graph/ms 13.25, stage/main/update 12.34, stage/main/post_update 12.20, encode//sum 10.85, stage/render/graph_submit 9.17, encode/submit 6.00, stream/drive/ms 5.05 |
| warm-up | 5.54 | 43.3 | render/thread/ms 58.93, render/busy/ms 57.45, main/busy/ms 37.05, stage/render/prepare_assets 25.86, stage/main/post_update 15.00, stage/main/update 14.79, stage/render/render 12.98, render/graph/ms 10.98, encode//sum 9.84, stage/render/graph_submit 7.97, stage/render/specialize 6.58, encode/submit 5.36 |
| warm-up | 5.58 | 37.2 | pipelines/compile_ms 170, pipelines/front_ms 165, render/thread/ms 64.77, render/busy/ms 63.75, stage/render/prepare_assets 40.64, main/busy/ms 32.03, stage/main/update 14.24, stage/main/post_update 11.82, stage/render/render 9.47, encode//sum 8.09, render/graph/ms 7.88, stage/render/graph_submit 5.90 |
| warm-up | 5.63 | 33.0 | render/thread/ms 47.78, render/busy/ms 44.28, main/busy/ms 25.77, stage/render/prepare_assets 21.18, stage/render/render 11.47, stage/main/update 10.42, stage/main/post_update 10.13, render/graph/ms 7.57, pipelines/compile_ms 7.47, pipelines/front_ms 7.03, encode//sum 6.05, stage/render/graph_submit 5.24 |
| warm-up | 5.67 | 34.7 | pipelines/compile_ms 126, pipelines/front_ms 118, render/thread/ms 60.12, render/busy/ms 56.37, main/busy/ms 45.12, stage/main/update 25.51, stage/render/prepare_assets 17.85, stage/render/render 14.22, stage/main/post_update 12.46, render/graph/ms 9.81, pipelines/wait_ms 9.09, stage/render/specialize 8.90 |
| warm-up | 5.71 | 43.3 | pipelines/compile_ms 118, pipelines/front_ms 111, render/thread/ms 64.24, render/busy/ms 62.85, main/busy/ms 50.36, stage/main/update 27.53, stage/render/prepare_assets 17.51, stage/main/post_update 13.54, stage/render/specialize 11.39, stage/render/render 11.15, render/graph/ms 9.14, pipelines/wait_ms 9.07 |
| warm-up | 5.75 | 33.7 | render/thread/ms 62.12, render/busy/ms 60.61, main/busy/ms 40.83, stage/render/prepare_assets 17.52, stage/main/post_update 16.27, stage/main/update 15.13, stage/render/render 12.85, render/graph/ms 10.98, encode//sum 9.00, stage/render/specialize 8.87, stage/render/graph_submit 7.63, stream/drive/ms 5.89 |
| warm-up | 5.78 | 38.7 | render/thread/ms 57.44, render/busy/ms 55.92, main/busy/ms 45.57, stage/main/update 23.65, stage/render/prepare_assets 17.16, stage/main/post_update 15.00, stream/drive/ms 14.06, stage/render/render 13.38, render/graph/ms 11.35, encode//sum 9.01, stage/render/graph_submit 7.58, stage/render/prepare_write_phase_buffers 6.29 |
| warm-up | 5.82 | 38.8 | pipelines/compile_ms 779, pipelines/front_ms 753, render/thread/ms 56.48, render/busy/ms 54.88, main/busy/ms 49.57, stage/main/update 28.96, pipelines/driver_ms 25.71, stage/render/prepare_assets 22.33, stage/render/render 14.00, stage/main/post_update 13.84, stream/drive/ms 13.70, render/graph/ms 11.77 |
| warm-up | 5.85 | 28.9 | pipelines/compile_ms 1175, pipelines/front_ms 1141, render/thread/ms 51.02, render/busy/ms 49.34, main/busy/ms 43.79, pipelines/driver_ms 34.04, stage/main/update 22.84, stage/render/prepare_assets 17.85, stage/render/render 14.38, stage/main/post_update 13.59, render/graph/ms 11.96, encode//sum 10.20 |
| warm-up | 5.88 | 29.9 | pipelines/compile_ms 471, pipelines/front_ms 459, render/thread/ms 46.10, render/busy/ms 44.70, main/busy/ms 32.26, stage/main/update 16.79, stage/render/prepare_assets 15.11, pipelines/driver_ms 12.08, stage/render/render 10.55, stage/main/post_update 9.64, render/graph/ms 8.61, stream/drive/ms 8.24 |
| warm-up | 5.94 | 25.6 | render/thread/ms 39.76, render/busy/ms 38.41, pipelines/compile_ms 34.65, main/busy/ms 33.23, pipelines/front_ms 32.79, stage/main/update 14.20, stage/main/post_update 12.37, stage/render/render 11.69, render/graph/ms 9.98, encode//sum 7.75, stage/render/graph_submit 7.37, stage/render/prepare_assets 7.31 |
| warm-up | 5.99 | 28.7 | pipelines/compile_ms 263, pipelines/front_ms 254, render/thread/ms 45.24, render/busy/ms 43.88, main/busy/ms 38.68, stage/main/update 14.48, stage/main/post_update 14.16, stage/render/render 13.67, render/graph/ms 11.99, stage/render/prepare_assets 9.35, pipelines/driver_ms 9.12, stage/render/graph_submit 8.95 |
| warm-up | 6.04 | 25.6 | pipelines/compile_ms 726, pipelines/front_ms 705, main/busy/ms 48.04, render/thread/ms 40.95, render/busy/ms 39.46, stage/main/update 30.69, pipelines/driver_ms 20.82, stream/drive/ms 20.74, stage/render/render 14.42, render/graph/ms 12.56, stage/main/post_update 10.95, stage/render/graph_submit 9.62 |
| warm-up | 6.07 | 34.8 | pipelines/compile_ms 546, pipelines/front_ms 532, main/busy/ms 46.59, render/thread/ms 43.96, render/busy/ms 42.49, stage/main/update 30.12, stream/drive/ms 20.99, pipelines/driver_ms 14.10, stage/render/render 12.82, render/graph/ms 10.87, stage/render/prepare_assets 10.38, stage/main/post_update 10.01 |
| warm-up | 6.17 | 38.9 | pipelines/compile_ms 906, pipelines/front_ms 884, main/busy/ms 40.98, render/thread/ms 40.39, render/busy/ms 38.84, pipelines/driver_ms 21.11, stage/main/update 20.69, stage/main/post_update 14.15, stage/render/render 13.94, render/graph/ms 11.72, stage/render/prepare_assets 10.18, encode//sum 9.33 |
| warm-up | 6.33 | 25.7 | pipelines/compile_ms 127, pipelines/front_ms 122, render/thread/ms 37.01, render/busy/ms 35.88, main/busy/ms 26.50, stage/render/prepare_assets 12.77, stage/main/update 11.57, stage/main/post_update 10.01, stage/render/render 9.55, render/graph/ms 8.09, encode//sum 7.20, stage/render/graph_submit 5.95 |
| warm-up | 6.40 | 41.4 | render/thread/ms 52.15, render/busy/ms 50.20, main/busy/ms 35.26, stage/main/post_update 17.05, stage/render/render 16.79, render/graph/ms 14.25, stage/main/update 13.51, stage/render/graph_submit 10.32, encode//sum 9.39, stage/render/prepare_assets 7.39, stage/render/specialize 7.36, encode/submit 6.44 |
| warm-up | 6.47 | 50.9 | render/thread/ms 49.94, render/busy/ms 49.02, main/busy/ms 37.18, stage/render/prepare_assets 27.80, stage/main/update 17.49, stage/main/post_update 13.39, stage/render/render 9.08, stream/drive/ms 8.74, render/graph/ms 7.76, encode//sum 6.96, pipelines/compile_ms 6.07, pipelines/front_ms 5.69 |
| warm-up | 6.81 | 31.4 | pipelines/compile_ms 142, pipelines/front_ms 138, render/thread/ms 38.52, render/busy/ms 37.52, main/busy/ms 14.66, stage/render/render 9.17, stage/render/specialize 9.08, stage/render/queue 7.74, render/graph/ms 7.74, encode//sum 7.61, stage/main/post_update 7.23, stage/render/graph_submit 5.64 |
| warm-up | 8.04 | 25.6 | render/thread/ms 32.65, render/busy/ms 30.53, main/busy/ms 14.17, stage/render/render 9.91, stage/render/specialize 8.63, render/graph/ms 8.58, stage/main/post_update 7.65, stage/render/graph_submit 6.63, encode//sum 6.02, stage/render/queue 5.57, encode/submit 3.98, stage/main/update 3.95 |
| measure | 0.83 | 28.5 | render/thread/ms 37.06, render/busy/ms 34.65, main/busy/ms 13.87, stage/render/render 12.20, render/graph/ms 10.68, stage/render/specialize 9.42, encode//sum 9.25, stage/render/graph_submit 8.36, stage/main/post_update 7.81, stage/render/queue 6.57, encode/submit 5.30, stage/main/update 3.37 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=nocull-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 4%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 98.1 W · 79 °C (max 82) · VRAM 6447 MiB · throttled 100% of samples

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
