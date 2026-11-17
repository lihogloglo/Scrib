# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1496 frames over 15.0 s · **median 10.07 ms (99 fps)** · mean 10.03 · p95 11.45 · p99 11.95 (1% low 84 fps) · worst 25.13 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 279 frames over 8.7 s, worst 2465.2 ms
- hitches (> 25.2 ms): **0** measured, 65 in warm-up
- timed GPU passes: 8.56 ms of a 10.03 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.07 ms, limit 11 ms
- pass worst frame 25.13 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.865 | 2.885 | 2.907 | 0.99 | 0.065 |
| dlss_super_resolution | 2.013 | 2.026 | 2.050 | 0.99 | 0.182 |
| early prepass | 0.599 | 0.603 | 0.613 | 0.99 | 0.094 |
| atmosphere_luts | 0.338 | 0.340 | 0.396 | 0.99 | 0.003 |
| bloom | 0.313 | 0.315 | 0.332 | 0.99 | 0.065 |
| main_transmissive_pass_3d | 0.310 | 0.312 | 0.333 | 0.99 | 0.009 |
| water_velocity | 0.239 | 0.241 | 0.264 | 0.99 | 0.002 |
| water_sspr | 0.209 | 0.211 | 0.214 | 0.99 | 0.002 |
| ssao | 0.189 | 0.190 | 0.199 | 0.99 | 0.007 |
| clouds_trace | 0.149 | 0.150 | 0.182 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.145 | 0.146 | 0.151 | 0.99 | 0.001 |
| uniform_allocation | 0.142 | 0.143 | 0.196 | 0.99 | 0.023 |
| water_fft | 0.131 | 0.132 | 0.136 | 0.99 | 0.003 |
| upscaling | 0.113 | 0.113 | 0.119 | 0.99 | 0.004 |
| clouds_composite | 0.112 | 0.112 | 0.115 | 0.99 | 0.002 |
| bin_unpacking | 0.103 | 0.104 | 0.134 | 0.99 | 0.016 |
| tonemapping | 0.086 | 0.087 | 0.092 | 0.99 | 0.003 |
| clustering | 0.064 | 0.065 | 0.077 | 0.99 | 0.021 |
| lightprobe_irradiance_map | 0.052 | 0.415 | 0.417 | 0.12 | 0.001 |
| render_sky | 0.052 | 0.052 | 0.053 | 0.99 | 0.002 |
| clouds_resolve | 0.050 | 0.050 | 0.052 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.039 | 0.039 | 0.052 | 0.99 | 0.005 |
| water_snapshot | 0.033 | 0.033 | 0.035 | 0.99 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.99 | 0.001 |
| main_transparent_pass_3d | 0.029 | 0.029 | 0.031 | 0.99 | 0.138 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.230 | 0.12 | 0.004 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| surface_exposure_far | 0.014 | 0.043 | 0.045 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 0.99 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.008 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.965 | 10.416 | 23.83 | 13411.2 |
| render/busy/ms | 7.133 | 8.450 | 23.25 | 10671.7 |
| main/busy/ms | 7.077 | 8.049 | 9.10 | 10587.7 |
| stage/render/render | 4.671 | 5.897 | 8.03 | 6988.5 |
| stage/main/post_update | 4.129 | 4.924 | 6.28 | 6177.2 |
| render/graph/ms | 4.052 | 5.206 | 6.79 | 6061.9 |
| encode//sum | 3.546 | 5.311 | 8.64 | 5305.4 |
| stage/render/graph_submit | 2.981 | 4.065 | 5.69 | 4459.0 |
| encode/submit | 1.869 | 2.603 | 4.75 | 2795.5 |
| stage/main/update | 1.806 | 2.262 | 3.89 | 2702.5 |
| stage/render/prepare_views | 1.520 | 3.268 | 4.30 | 2274.1 |
| render/acquire/ms | 1.473 | 3.209 | 4.25 | 2203.9 |
| encode/finish_wall | 1.133 | 1.645 | 2.46 | 1694.3 |
| stage/render/camera_driver | 1.013 | 1.169 | 1.55 | 1515.0 |
| encode/pass:shadow_directional_light/sum | 0.751 | 1.299 | 1.99 | 1124.0 |
| encode/pass:early prepass/sum | 0.717 | 1.003 | 1.42 | 1071.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.697 | 1.002 | 1.45 | 1043.3 |
| stage/render/prepare_write_phase_buffers | 0.575 | 0.819 | 1.12 | 860.5 |
| stage/render/prepare_assets | 0.455 | 0.585 | 1.54 | 680.9 |
| stage/render/prepare_resources | 0.394 | 0.459 | 0.81 | 590.1 |
| bigspace/total/ms | 0.368 | 0.787 | 2.29 | 551.2 |
| render/present/ms | 0.358 | 0.431 | 1.89 | 535.7 |
| stage/main/pre_update | 0.353 | 0.473 | 0.91 | 527.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.297 | 0.479 | 0.85 | 445.0 |
| stage/main/first | 0.296 | 0.363 | 1.62 | 442.4 |
| stage/render/prepare_batch_phases | 0.239 | 0.314 | 0.48 | 358.0 |
| terrain/update/ms | 0.237 | 0.302 | 0.69 | 354.3 |
| stage/render/prepare_meshes | 0.224 | 0.277 | 1.01 | 334.9 |
| stage/render/prepare_bind_groups | 0.216 | 0.309 | 0.41 | 322.9 |
| terrain/select/ms | 0.206 | 0.262 | 0.66 | 308.0 |
| stream/drive/ms | 0.195 | 0.271 | 0.75 | 291.9 |
| stage/main/fixed_main_loop | 0.188 | 0.343 | 0.83 | 281.7 |
| stage/render/create_views | 0.175 | 0.227 | 0.63 | 262.1 |
| stage/render/extract_commands | 0.172 | 0.198 | 0.44 | 257.1 |
| far/select/ms | 0.170 | 0.238 | 0.72 | 254.7 |
| stream/tick/ms | 0.165 | 0.234 | 0.70 | 247.4 |
| stage/render/prepare_flush | 0.159 | 0.285 | 0.56 | 238.2 |
| controller/feed/ms | 0.151 | 0.201 | 0.56 | 226.1 |
| bigspace/high_precision/ms | 0.134 | 0.336 | 1.96 | 200.7 |
| stage/main/last | 0.117 | 0.162 | 0.64 | 175.4 |
| stage/render/queue | 0.117 | 0.151 | 5.74 | 175.1 |
| controller/move/ms | 0.113 | 0.183 | 0.58 | 169.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.106 | 0.174 | 0.42 | 158.9 |
| stage/render/cleanup | 0.104 | 0.158 | 0.24 | 155.8 |
| stage/render/specialize | 0.096 | 0.123 | 8.33 | 144.0 |
| anim/evaluate/ms | 0.090 | 0.202 | 1.11 | 134.1 |
| bigspace/low_precision/ms | 0.087 | 0.195 | 1.23 | 130.3 |
| anim/evaluate_cpu/ms | 0.064 | 0.078 | 0.53 | 95.3 |
| stage/render/prepare_collect_phase_buffers | 0.061 | 0.121 | 0.29 | 91.0 |
| encode/resolve_diagnostics/sum | 0.043 | 0.077 | 0.62 | 64.6 |
| physics/step/ms | 0.028 | 0.055 | 0.49 | 41.7 |
| stage/render/phase_sort | 0.027 | 0.043 | 0.09 | 40.1 |
| weather/map/ms | 0.026 | 0.034 | 0.42 | 38.5 |
| encode/camera_driver_span_begin/sum | 0.021 | 0.038 | 0.40 | 31.5 |
| encode/camera_driver_span_end/sum | 0.018 | 0.035 | 0.38 | 27.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.06 | 17.0 |
| script/update/ms | 0.007 | 0.010 | 0.41 | 11.1 |
| controller/avoid/ms | 0.006 | 0.007 | 0.02 | 8.6 |
| script/input/ms | 0.005 | 0.007 | 0.05 | 8.1 |
| anim/apply/ms | 0.005 | 0.006 | 0.02 | 7.0 |
| script/ui/ms | 0.005 | 0.007 | 0.02 | 6.8 |
| controller/write/ms | 0.002 | 0.002 | 0.08 | 2.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 1.0 |
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
| encode//max | ms | 0.45 | 1.86 | 0.61 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.40 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.38 | 0.01 |
| encode/finish_critical | ms | 0.85 | 1.99 | 0.85 |
| encode/pass:early prepass/max | ms | 0.71 | 1.42 | 0.85 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.70 | 1.45 | 0.78 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.42 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.28 | 0.85 | 0.30 |
| encode/pass:shadow_directional_light/max | ms | 0.68 | 1.99 | 0.63 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.62 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.25 | 23.25 | 9.23 |
| gpu/timed/ms | ms | 9.25 | 10.05 | 9.23 |
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
| process/cpu_usage | % | 29.81 | 43.27 | 24.24 |
| process/mem_usage | GiB | 4.74 | 4.84 | 4.82 |
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
| skin/blocks_changed/count | count | 4 | 6 | 5 |
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
| system/cpu_usage | % | 34.23 | 43.21 | 37.33 |
| system/mem_usage | % | 54.27 | 54.60 | 54.57 |
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
| anim/events/count | count | 0.10 | 2 | 152 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.31 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 26.57 | 29 | 39.7 k |
| encode/bind_walk/init_textures | count | 7180 | 14.0 k | 10.74 M |
| encode/bind_walk/init_textures_skipped | count | 36.2 k | 50.8 k | 54.21 M |
| encode/bind_walk/inits | count | 422 | 515 | 631.1 k |
| encode/bind_walk/inits_skipped | count | 97.50 | 111 | 145.9 k |
| encode/bind_walk/merge_textures | count | 7116 | 13.9 k | 10.65 M |
| encode/bind_walk/merge_textures_skipped | count | 4225 | 4225 | 6.32 M |
| encode/bind_walk/merges | count | 84.26 | 105 | 126.1 k |
| encode/bind_walk/merges_skipped | count | 81 | 87 | 121.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1496 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1496 |
| encode/pass:early prepass/count | count | 1 | 1 | 1496 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1496 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1496 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1496 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1496 |
| encode/queue/command_buffers | count | 36.57 | 39 | 54.7 k |
| encode/queue/submits | count | 2 | 2 | 2992 |
| encode/queue/write_buffer/bytes | bytes | 1.84 M | 4.56 M | 2.76 G |
| encode/queue/write_buffer/calls | count | 233 | 305 | 348.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 48.83 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.5 k |
| encode/queue/write_texture/bytes | bytes | 12.0 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1496 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.60 | 2 | 900 |
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

## Hitches (> 25.2 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2465.2 | main/busy/ms 2437, stage/main/post_update 2333, stage/main/update 95.78, surface/advance_cpu_ms 67.31, stage/main/pre_update 4.72, stage/main/last 1.16, surface/cell_upload_ms 0.68, stream/drive/ms 0.52, stage/main/first 0.48, stage/main/fixed_main_loop 0.36, weather/map/ms 0.13, bigspace/total/ms 0.12 |
| warm-up | 3.28 | 815.3 | main/busy/ms 2577, stage/main/post_update 2358, render/thread/ms 785, render/busy/ms 622, stage/render/prepare_assets 575, stage/main/update 205, stage/render/prepare_views 160, render/acquire/ms 160, surface/advance_cpu_ms 67.31, stage/render/create_views 28.35, stage/render/render 10.78, render/graph/ms 6.83 |
| warm-up | 3.38 | 102.2 | render/thread/ms 871, render/busy/ms 660, stage/render/prepare_assets 600, stage/render/prepare_views 161, render/acquire/ms 161, main/busy/ms 159, stage/main/update 113, stage/render/render 63.28, render/present/ms 50.48, pipelines/compile_ms 41.26, pipelines/front_ms 38.13, stage/main/post_update 36.08 |
| warm-up | 3.42 | 34.0 | pipelines/compile_ms 267, pipelines/front_ms 240, render/thread/ms 113, stage/render/render 73.06, render/present/ms 57.41, main/busy/ms 55.57, render/busy/ms 54.60, pipelines/wait_ms 42.57, stage/main/post_update 40.27, pipelines/driver_ms 27.02, stage/render/prepare_assets 26.43, terrain/update/ms 23.89 |
| warm-up | 3.54 | 120.3 | pipelines/wait_ms 4033, pipelines/compile_ms 1088, pipelines/front_ms 1017, render/thread/ms 141, render/busy/ms 129, pipelines/driver_ms 71.40, main/busy/ms 65.39, stage/main/post_update 49.80, stage/render/prepare_assets 32.64, stage/render/prepare_write_phase_buffers 27.96, stage/render/prepare_resources 26.78, stage/render/render 25.52 |
| warm-up | 3.60 | 64.0 | pipelines/wait_ms 4442, pipelines/compile_ms 1782, pipelines/front_ms 1678, render/thread/ms 173, render/busy/ms 170, main/busy/ms 115, pipelines/driver_ms 104, stage/main/update 68.70, stage/render/prepare_resources 55.97, stage/render/prepare_assets 39.89, stage/render/prepare_write_phase_buffers 38.97, stage/main/post_update 30.46 |
| warm-up | 3.70 | 95.8 | pipelines/compile_ms 1550, pipelines/front_ms 1472, pipelines/wait_ms 585, render/thread/ms 122, render/busy/ms 120, main/busy/ms 101, pipelines/driver_ms 78.41, stage/main/update 71.62, stage/render/prepare_assets 61.75, stage/render/prepare_resources 31.86, stage/main/post_update 16.92, terrain/update/ms 14.74 |
| warm-up | 3.77 | 68.7 | pipelines/compile_ms 2363, pipelines/front_ms 2264, pipelines/wait_ms 362, render/thread/ms 130, render/busy/ms 126, pipelines/driver_ms 98.61, stage/render/prepare_assets 97.93, main/busy/ms 35.50, stage/render/render 22.61, render/graph/ms 18.09, encode//sum 18.01, stage/main/post_update 15.87 |
| warm-up | 3.79 | 27.6 | pipelines/compile_ms 1789, pipelines/front_ms 1705, pipelines/wait_ms 237, render/thread/ms 92.85, render/busy/ms 88.67, pipelines/driver_ms 83.94, stage/render/prepare_assets 51.45, main/busy/ms 32.56, stage/render/render 31.43, render/graph/ms 26.94, encode//sum 19.09, stage/render/graph_submit 15.25 |
| warm-up | 3.83 | 27.1 | pipelines/compile_ms 66.21, pipelines/front_ms 61.39, main/busy/ms 37.22, render/thread/ms 31.74, render/busy/ms 30.17, stage/main/post_update 17.85, stage/render/render 10.76, stage/render/prepare_assets 9.20, terrain/update/ms 9.06, render/graph/ms 8.59, encode//sum 7.37, stage/main/pre_update 7.32 |
| warm-up | 3.88 | 45.7 | pipelines/compile_ms 747, pipelines/front_ms 705, main/busy/ms 62.69, render/thread/ms 61.06, render/busy/ms 59.33, stage/main/update 43.68, pipelines/driver_ms 42.03, stage/render/prepare_assets 40.82, stream/drive/ms 39.02, stage/render/render 11.81, stage/main/post_update 10.04, render/graph/ms 9.55 |
| warm-up | 3.92 | 44.3 | pipelines/compile_ms 938, pipelines/front_ms 893, render/thread/ms 75.85, render/busy/ms 68.38, main/busy/ms 53.76, stage/render/prepare_assets 51.87, pipelines/driver_ms 45.70, stage/main/update 43.18, stream/drive/ms 39.28, pipelines/wait_ms 18.62, stage/render/render 17.75, render/graph/ms 9.99 |
| warm-up | 4.04 | 95.8 | pipelines/compile_ms 662, pipelines/front_ms 644, render/thread/ms 64.41, render/busy/ms 63.22, main/busy/ms 62.58, stage/main/update 49.93, stage/render/prepare_assets 45.44, stream/drive/ms 44.89, pipelines/driver_ms 18.29, stage/render/render 9.52, stage/main/post_update 7.75, render/graph/ms 7.09 |
| warm-up | 4.13 | 81.1 | pipelines/compile_ms 737, pipelines/front_ms 719, main/busy/ms 52.12, stage/main/update 40.84, render/thread/ms 37.06, render/busy/ms 35.86, stream/drive/ms 33.24, stage/render/prepare_assets 22.27, pipelines/driver_ms 17.89, stage/render/render 9.22, render/graph/ms 7.65, stage/main/post_update 6.77 |
| warm-up | 4.21 | 54.7 | pipelines/compile_ms 120, pipelines/front_ms 112, main/busy/ms 55.68, render/thread/ms 48.49, render/busy/ms 46.52, stage/main/update 41.86, stream/drive/ms 20.90, stage/render/prepare_assets 18.44, stage/render/render 12.66, render/graph/ms 10.15, stage/main/post_update 8.73, pipelines/driver_ms 8.36 |
| warm-up | 4.25 | 45.1 | render/thread/ms 68.40, render/busy/ms 66.10, pipelines/compile_ms 59.17, pipelines/front_ms 56.40, main/busy/ms 38.11, stage/render/prepare_assets 22.90, stage/main/update 16.89, stage/main/post_update 13.24, stage/render/render 12.58, stage/render/prepare_resources 10.77, stage/render/extract_commands 10.14, render/graph/ms 9.87 |
| warm-up | 4.30 | 52.1 | pipelines/compile_ms 75.82, pipelines/front_ms 72.35, render/thread/ms 71.31, render/busy/ms 69.69, main/busy/ms 46.47, stage/render/prepare_assets 28.77, stage/main/update 23.35, stage/main/post_update 12.50, stage/render/prepare_resources 10.46, stage/render/render 10.28, stage/render/extract_commands 10.22, render/graph/ms 8.12 |
| warm-up | 4.36 | 58.3 | pipelines/compile_ms 1068, pipelines/front_ms 1036, render/thread/ms 66.02, render/busy/ms 64.64, stage/render/prepare_assets 44.77, main/busy/ms 42.37, pipelines/driver_ms 31.66, stage/main/update 22.01, stage/main/post_update 9.85, stage/render/render 9.50, render/graph/ms 7.45, encode//sum 6.48 |
| warm-up | 4.43 | 71.0 | pipelines/compile_ms 1934, pipelines/front_ms 1872, render/thread/ms 101, pipelines/wait_ms 101, render/busy/ms 96.29, main/busy/ms 71.59, pipelines/driver_ms 61.79, stage/render/prepare_assets 55.31, stage/main/update 33.50, stage/main/post_update 25.24, stage/render/render 18.79, render/graph/ms 16.35 |
| warm-up | 4.50 | 68.1 | pipelines/compile_ms 1178, pipelines/front_ms 1138, render/thread/ms 120, render/busy/ms 116, pipelines/wait_ms 103, main/busy/ms 101, stage/render/prepare_assets 52.11, stage/main/post_update 47.34, stage/main/update 42.84, pipelines/driver_ms 39.33, stage/render/render 19.86, render/graph/ms 17.62 |
| warm-up | 4.57 | 71.9 | pipelines/compile_ms 529, pipelines/front_ms 514, render/thread/ms 87.69, render/busy/ms 86.22, main/busy/ms 68.01, stage/render/prepare_assets 41.11, stage/main/post_update 31.50, stage/main/update 27.95, pipelines/driver_ms 15.12, stage/render/render 13.27, stage/render/prepare_flush 12.57, stage/render/prepare_resources 9.99 |
| warm-up | 4.63 | 62.2 | pipelines/compile_ms 1757, pipelines/front_ms 1705, main/busy/ms 92.92, render/thread/ms 85.37, render/busy/ms 83.99, stage/main/update 76.24, stream/drive/ms 65.87, pipelines/wait_ms 62.83, stage/render/prepare_assets 53.85, pipelines/driver_ms 51.79, stage/render/render 12.15, stage/render/prepare_resources 9.60 |
| warm-up | 4.69 | 53.9 | pipelines/compile_ms 2132, pipelines/front_ms 2070, main/busy/ms 163, stage/main/update 126, stream/drive/ms 114, render/thread/ms 79.37, render/busy/ms 77.96, pipelines/wait_ms 66.27, pipelines/driver_ms 62.24, stage/render/prepare_assets 44.47, stage/main/post_update 27.55, stage/render/render 11.52 |
| warm-up | 4.79 | 105.0 | pipelines/compile_ms 1891, pipelines/front_ms 1828, main/busy/ms 113, pipelines/wait_ms 102, stage/main/update 71.36, pipelines/driver_ms 63.28, render/thread/ms 61.65, render/busy/ms 60.03, stream/drive/ms 52.14, stage/main/post_update 28.86, stage/render/prepare_assets 20.12, stage/render/prepare_resources 18.97 |
| warm-up | 4.84 | 42.4 | pipelines/compile_ms 1925, pipelines/front_ms 1857, pipelines/wait_ms 121, render/thread/ms 71.50, render/busy/ms 69.22, pipelines/driver_ms 68.00, main/busy/ms 45.17, stage/render/prepare_assets 21.19, stage/render/prepare_resources 18.85, stage/main/update 18.17, stage/render/render 17.82, render/graph/ms 15.60 |
| warm-up | 4.87 | 31.2 | pipelines/compile_ms 914, pipelines/front_ms 883, main/busy/ms 83.37, render/thread/ms 58.98, stage/main/update 57.14, render/busy/ms 56.52, pipelines/driver_ms 30.36, pipelines/wait_ms 24.89, stage/render/prepare_assets 20.65, stage/render/render 20.17, render/graph/ms 17.66, stage/main/post_update 13.97 |
| warm-up | 4.98 | 109.2 | pipelines/compile_ms 1437, pipelines/front_ms 1403, pipelines/wait_ms 145, main/busy/ms 81.94, render/thread/ms 72.44, render/busy/ms 70.58, stage/main/update 61.55, stage/render/prepare_assets 37.56, pipelines/driver_ms 33.96, stage/render/render 17.70, render/graph/ms 15.16, stream/drive/ms 12.70 |
| warm-up | 5.01 | 29.3 | pipelines/compile_ms 1414, pipelines/front_ms 1380, pipelines/wait_ms 146, render/thread/ms 74.21, render/busy/ms 72.47, main/busy/ms 38.40, stage/render/prepare_assets 34.57, pipelines/driver_ms 34.34, stage/main/update 18.75, stage/render/render 15.06, render/graph/ms 13.00, stage/main/post_update 11.61 |
| warm-up | 5.05 | 46.0 | pipelines/compile_ms 691, pipelines/front_ms 648, render/thread/ms 67.90, render/busy/ms 66.02, main/busy/ms 43.77, pipelines/driver_ms 42.41, stage/main/update 18.33, stage/main/post_update 17.97, stage/render/prepare_flush 14.27, stage/render/render 11.67, stage/render/prepare_resources 11.56, render/graph/ms 9.86 |
| warm-up | 5.09 | 34.7 | pipelines/compile_ms 1059, pipelines/front_ms 1010, render/thread/ms 71.11, render/busy/ms 69.23, main/busy/ms 59.05, pipelines/driver_ms 49.30, stage/main/update 34.34, stage/render/render 19.65, stream/drive/ms 18.77, render/graph/ms 17.44, stage/main/post_update 16.61, stage/render/prepare_flush 14.12 |
| warm-up | 5.14 | 52.5 | pipelines/compile_ms 880, pipelines/front_ms 856, render/thread/ms 71.16, render/busy/ms 69.45, main/busy/ms 61.16, stage/main/update 35.33, pipelines/driver_ms 24.39, stage/render/prepare_assets 20.17, stream/drive/ms 19.07, stage/render/render 18.57, stage/main/post_update 17.07, render/graph/ms 16.29 |
| warm-up | 5.18 | 45.2 | pipelines/compile_ms 728, pipelines/front_ms 706, render/thread/ms 82.94, render/busy/ms 81.37, main/busy/ms 43.94, stage/render/prepare_assets 24.32, pipelines/driver_ms 22.60, stage/main/post_update 19.07, stage/main/update 16.55, stage/render/prepare_write_phase_buffers 10.94, stage/render/render 10.55, render/graph/ms 8.70 |
| warm-up | 5.22 | 39.8 | pipelines/compile_ms 1072, pipelines/front_ms 1044, render/thread/ms 69.47, render/busy/ms 67.53, main/busy/ms 38.06, pipelines/driver_ms 27.62, stage/render/prepare_assets 18.67, stage/main/update 15.46, stage/render/render 15.03, stage/main/post_update 14.40, render/graph/ms 12.69, stage/render/graph_submit 9.51 |
| warm-up | 5.27 | 49.8 | pipelines/compile_ms 1424, pipelines/front_ms 1387, pipelines/wait_ms 110, render/thread/ms 71.56, render/busy/ms 69.72, main/busy/ms 50.16, pipelines/driver_ms 37.21, stage/render/prepare_assets 28.99, stage/main/update 27.96, stage/main/post_update 15.78, stage/render/render 14.79, render/graph/ms 12.30 |
| warm-up | 5.31 | 36.6 | pipelines/compile_ms 1815, pipelines/front_ms 1768, pipelines/wait_ms 110, render/thread/ms 68.63, render/busy/ms 67.20, pipelines/driver_ms 46.64, main/busy/ms 46.00, stage/render/prepare_assets 31.06, stage/main/update 26.15, stage/main/post_update 13.28, stage/render/render 9.80, render/graph/ms 7.83 |
| warm-up | 5.34 | 31.5 | pipelines/compile_ms 1161, pipelines/front_ms 1131, render/thread/ms 52.61, render/busy/ms 51.05, main/busy/ms 35.44, pipelines/driver_ms 29.54, stage/render/prepare_assets 23.00, stage/main/update 17.03, stage/main/post_update 11.75, stage/render/render 11.29, render/graph/ms 9.36, encode//sum 7.57 |
| warm-up | 5.37 | 25.6 | pipelines/compile_ms 161, pipelines/front_ms 155, render/thread/ms 45.63, render/busy/ms 44.13, main/busy/ms 30.51, stage/render/prepare_assets 18.76, stage/main/update 14.62, stage/render/render 11.90, render/graph/ms 10.09, stage/main/post_update 10.02, stage/render/graph_submit 7.52, encode//sum 7.50 |
| warm-up | 5.41 | 30.0 | pipelines/compile_ms 711, pipelines/front_ms 693, render/thread/ms 43.35, render/busy/ms 41.70, stage/render/render 25.66, render/graph/ms 23.41, main/busy/ms 21.09, stage/render/graph_submit 20.94, pipelines/driver_ms 17.82, stage/main/update 8.49, stage/main/post_update 8.16, encode//sum 6.95 |
| warm-up | 5.49 | 44.5 | pipelines/compile_ms 55.48, pipelines/front_ms 53.04, main/busy/ms 48.39, render/thread/ms 43.63, render/busy/ms 42.39, stage/main/update 24.72, stage/main/post_update 19.02, stage/render/prepare_assets 12.93, stream/drive/ms 12.58, stage/render/render 9.92, render/graph/ms 8.12, stage/render/phase_sort 7.87 |
| warm-up | 5.55 | 37.8 | pipelines/compile_ms 184, pipelines/front_ms 177, render/thread/ms 51.66, render/busy/ms 50.26, main/busy/ms 32.54, stage/render/prepare_assets 21.07, stage/main/post_update 14.45, stage/main/update 11.45, stage/render/render 10.64, render/graph/ms 8.70, pipelines/driver_ms 7.32, encode//sum 7.32 |
| warm-up | 5.58 | 31.3 | pipelines/compile_ms 184, pipelines/front_ms 177, render/thread/ms 64.46, render/busy/ms 63.10, main/busy/ms 42.35, stage/main/post_update 17.89, stage/main/update 17.67, stage/render/prepare_assets 16.62, stage/render/prepare_resources 14.99, stage/render/render 11.69, render/graph/ms 9.75, encode//sum 7.32 |
| warm-up | 5.63 | 47.8 | pipelines/compile_ms 1214, pipelines/front_ms 1184, render/thread/ms 72.35, render/busy/ms 70.94, main/busy/ms 42.59, stage/render/prepare_assets 32.39, pipelines/driver_ms 30.31, stage/main/update 18.80, stage/main/post_update 16.85, stage/render/prepare_resources 14.55, stage/render/render 11.78, render/graph/ms 9.77 |
| warm-up | 5.67 | 29.8 | pipelines/compile_ms 754, pipelines/front_ms 740, render/thread/ms 41.49, render/busy/ms 40.35, main/busy/ms 26.68, stage/render/prepare_assets 16.51, pipelines/driver_ms 14.76, stage/main/update 12.09, stage/main/post_update 8.84, stage/render/render 8.73, render/graph/ms 7.31, encode//sum 6.56 |
| warm-up | 5.70 | 29.4 | pipelines/compile_ms 351, pipelines/front_ms 344, render/thread/ms 53.69, render/busy/ms 52.00, main/busy/ms 26.30, stage/render/prepare_assets 17.98, stage/render/render 12.22, stage/main/update 11.76, stage/main/post_update 9.99, render/graph/ms 9.77, stage/render/specialize 9.11, encode//sum 8.10 |
| warm-up | 5.73 | 31.0 | render/thread/ms 54.47, render/busy/ms 52.72, main/busy/ms 26.23, stage/render/prepare_assets 18.07, stage/render/render 13.73, render/graph/ms 11.09, stage/main/update 10.99, stage/main/post_update 9.65, encode//sum 9.02, stage/render/graph_submit 7.69, stage/render/specialize 7.20, encode/submit 4.93 |
| warm-up | 5.78 | 43.1 | render/thread/ms 58.46, render/busy/ms 57.26, main/busy/ms 30.65, stage/render/prepare_assets 30.15, stage/main/update 13.71, stage/main/post_update 10.69, stage/render/render 9.19, encode//sum 8.13, render/graph/ms 7.56, stage/render/specialize 5.15, stage/render/graph_submit 4.86, stream/drive/ms 4.31 |
| warm-up | 5.81 | 34.0 | render/thread/ms 62.28, render/busy/ms 61.18, stage/render/prepare_assets 38.06, main/busy/ms 32.29, stage/main/update 15.62, stage/main/post_update 11.17, stage/render/render 7.82, encode//sum 6.93, render/graph/ms 6.35, stage/render/specialize 4.84, stream/drive/ms 4.51, stage/render/graph_submit 4.40 |
| warm-up | 5.84 | 31.8 | render/thread/ms 57.24, render/busy/ms 56.28, stage/render/prepare_assets 33.85, main/busy/ms 29.55, stage/main/update 13.66, stage/main/post_update 10.46, stage/render/render 7.70, render/graph/ms 6.27, encode//sum 6.18, stage/render/specialize 4.56, stage/render/graph_submit 4.41, stream/drive/ms 3.84 |
| warm-up | 5.88 | 33.8 | pipelines/compile_ms 308, pipelines/front_ms 299, render/thread/ms 58.06, render/busy/ms 56.53, main/busy/ms 38.82, stage/render/prepare_assets 25.04, stage/main/update 18.20, stage/main/post_update 14.35, stage/render/render 13.60, render/graph/ms 11.21, pipelines/driver_ms 9.27, stage/render/graph_submit 8.67 |
| warm-up | 5.91 | 37.2 | pipelines/compile_ms 414, pipelines/front_ms 399, render/thread/ms 59.44, render/busy/ms 57.79, main/busy/ms 39.57, stage/render/prepare_assets 19.97, stage/main/update 18.52, pipelines/driver_ms 14.28, stage/render/render 14.17, stage/main/post_update 12.86, render/graph/ms 11.66, stage/render/specialize 9.24 |
| warm-up | 5.95 | 34.1 | pipelines/compile_ms 105, pipelines/front_ms 100, render/thread/ms 56.86, render/busy/ms 55.39, main/busy/ms 36.59, stage/render/prepare_assets 18.88, stage/main/post_update 14.87, stage/main/update 13.25, stage/render/render 11.45, render/graph/ms 9.26, stage/render/specialize 9.18, encode//sum 9.18 |
| warm-up | 6.03 | 63.4 | pipelines/compile_ms 53.94, pipelines/front_ms 51.20, main/busy/ms 51.19, render/thread/ms 49.86, render/busy/ms 48.35, stage/main/update 29.35, stream/drive/ms 17.97, stage/render/prepare_assets 17.71, stage/main/post_update 15.92, stage/render/render 15.18, render/graph/ms 13.17, stage/render/graph_submit 9.81 |
| warm-up | 6.07 | 38.1 | pipelines/compile_ms 1190, pipelines/front_ms 1149, render/thread/ms 62.93, render/busy/ms 61.26, main/busy/ms 48.80, pipelines/driver_ms 40.89, stage/main/update 26.28, stage/render/prepare_assets 24.27, stage/render/render 16.50, stage/main/post_update 15.26, render/graph/ms 13.93, encode//sum 11.14 |
| warm-up | 6.10 | 35.8 | pipelines/compile_ms 1136, pipelines/front_ms 1098, render/thread/ms 54.40, render/busy/ms 52.90, main/busy/ms 44.88, pipelines/driver_ms 38.14, stage/main/update 24.69, stage/render/prepare_assets 23.87, stage/render/render 13.02, stage/main/post_update 11.92, render/graph/ms 10.66, stream/drive/ms 8.85 |
| warm-up | 6.30 | 35.2 | pipelines/compile_ms 914, pipelines/front_ms 888, main/busy/ms 47.77, render/thread/ms 36.23, render/busy/ms 34.98, stage/main/update 29.02, pipelines/driver_ms 25.55, stream/drive/ms 20.86, stage/render/render 12.45, stage/main/post_update 12.10, render/graph/ms 10.83, stage/render/prepare_assets 8.53 |
| warm-up | 6.39 | 31.4 | render/thread/ms 37.05, render/busy/ms 35.81, main/busy/ms 31.43, stage/main/update 17.07, stage/render/render 11.56, stage/render/prepare_assets 9.91, render/graph/ms 9.88, stage/main/post_update 9.21, encode//sum 7.73, stage/render/graph_submit 6.36, encode/submit 4.20, stream/drive/ms 3.35 |
| warm-up | 6.49 | 26.4 | render/thread/ms 40.23, render/busy/ms 39.25, main/busy/ms 20.32, stage/render/prepare_assets 18.17, stage/render/render 9.78, stage/main/post_update 8.35, render/graph/ms 8.30, stage/main/update 7.58, encode//sum 7.54, stage/render/graph_submit 5.55, encode/submit 3.35, stream/drive/ms 2.67 |
| warm-up | 6.60 | 29.1 | render/thread/ms 42.03, render/busy/ms 40.89, main/busy/ms 29.92, stage/render/prepare_assets 16.53, stage/main/update 12.52, stage/main/post_update 12.01, stage/render/render 9.24, render/graph/ms 7.87, encode//sum 6.37, stage/render/graph_submit 5.73, stage/render/specialize 4.89, encode/submit 3.91 |
| warm-up | 6.63 | 29.3 | render/thread/ms 51.96, render/busy/ms 50.40, main/busy/ms 33.97, stage/render/prepare_assets 17.43, stage/main/update 14.45, stage/main/post_update 13.63, stage/render/render 12.53, render/graph/ms 10.41, encode//sum 8.56, stage/render/specialize 7.64, stage/render/graph_submit 7.41, encode/submit 4.85 |
| warm-up | 6.66 | 31.6 | render/thread/ms 53.12, render/busy/ms 51.39, main/busy/ms 43.53, stage/main/update 23.07, stage/render/render 15.36, stage/main/post_update 14.51, render/graph/ms 12.77, stream/drive/ms 12.54, stage/render/prepare_assets 11.96, encode//sum 10.28, stage/render/graph_submit 8.69, stage/render/specialize 6.39 |
| warm-up | 6.70 | 44.4 | render/thread/ms 57.22, render/busy/ms 55.99, main/busy/ms 40.71, stage/render/prepare_assets 29.27, stage/main/update 19.94, stage/main/post_update 15.41, stream/drive/ms 12.97, stage/render/render 11.15, render/graph/ms 9.16, encode//sum 8.71, stage/render/graph_submit 5.91, stage/render/prepare_bind_groups 3.40 |
| warm-up | 7.07 | 32.7 | render/thread/ms 39.45, render/busy/ms 37.77, main/busy/ms 14.83, stage/render/render 11.93, render/graph/ms 10.34, stage/render/specialize 9.94, stage/render/graph_submit 8.37, stage/main/post_update 7.69, encode//sum 6.70, stage/render/queue 6.68, encode/submit 5.06, stage/main/update 4.31 |
| warm-up | 8.33 | 28.8 | render/thread/ms 36.35, render/busy/ms 34.01, main/busy/ms 13.89, stage/render/render 12.42, render/graph/ms 10.96, stage/render/specialize 9.11, stage/render/graph_submit 8.97, encode//sum 7.66, stage/main/post_update 7.47, stage/render/queue 5.47, encode/submit 5.34, stage/main/update 4.14 |
| warm-up | 8.47 | 54.2 | render/thread/ms 62.56, render/busy/ms 61.08, stage/render/render 54.77, render/graph/ms 53.32, stage/render/graph_submit 51.08, encode//sum 50.02, encode/finish_wall 44.83, encode/pass:early prepass/sum 44.46, encode/pass:main_opaque_pass_3d/sum 43.60, main/busy/ms 14.89, stage/main/post_update 8.05, encode/submit 6.29 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=nocull-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 5%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **97%** (min 63%) · 2613 MHz (min 2610) · memory 8101 MHz · 97.3 W · 80 °C (max 83) · VRAM 6447 MiB · throttled 100% of samples

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
