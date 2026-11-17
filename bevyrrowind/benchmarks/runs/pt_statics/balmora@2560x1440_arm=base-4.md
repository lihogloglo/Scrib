# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1554 frames over 15.0 s · **median 9.68 ms (103 fps)** · mean 9.66 · p95 11.08 · p99 11.53 (1% low 87 fps) · worst 26.86 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 279 frames over 8.4 s, worst 2486.5 ms
- hitches (> 25.0 ms): **1** measured, 59 in warm-up
- timed GPU passes: 8.48 ms of a 9.66 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.68 ms, limit 11 ms
- pass worst frame 26.86 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.880 | 2.898 | 2.956 | 0.99 | 0.072 |
| dlss_super_resolution | 2.007 | 2.020 | 2.041 | 0.99 | 0.185 |
| early prepass | 0.600 | 0.604 | 0.613 | 0.99 | 0.094 |
| atmosphere_luts | 0.316 | 0.318 | 0.340 | 0.99 | 0.003 |
| bloom | 0.311 | 0.313 | 0.328 | 0.99 | 0.060 |
| main_transmissive_pass_3d | 0.308 | 0.310 | 0.332 | 0.99 | 0.008 |
| water_velocity | 0.238 | 0.239 | 0.263 | 0.99 | 0.002 |
| water_sspr | 0.210 | 0.211 | 0.213 | 0.99 | 0.002 |
| ssao | 0.188 | 0.190 | 0.196 | 0.99 | 0.007 |
| clouds_trace | 0.148 | 0.149 | 0.172 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.145 | 0.146 | 0.153 | 0.99 | 0.001 |
| water_fft | 0.130 | 0.131 | 0.135 | 0.99 | 0.003 |
| uniform_allocation | 0.119 | 0.120 | 0.163 | 0.99 | 0.022 |
| upscaling | 0.112 | 0.113 | 0.118 | 0.99 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.113 | 0.99 | 0.002 |
| bin_unpacking | 0.090 | 0.090 | 0.122 | 0.99 | 0.016 |
| tonemapping | 0.085 | 0.086 | 0.090 | 0.99 | 0.003 |
| clustering | 0.063 | 0.063 | 0.073 | 0.99 | 0.020 |
| render_sky | 0.051 | 0.052 | 0.052 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.411 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 0.99 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 0.99 | 0.005 |
| lightprobe_radiance_map | 0.028 | 0.225 | 0.231 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 0.99 | 0.082 |
| water_snapshot | 0.025 | 0.025 | 0.026 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.046 | 0.32 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.99 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.012 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.014 | 0.014 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.739 | 10.177 | 25.59 | 13580.1 |
| main/busy/ms | 7.138 | 7.986 | 9.64 | 11092.3 |
| render/busy/ms | 6.929 | 7.898 | 25.02 | 10767.6 |
| stage/render/render | 4.626 | 5.433 | 8.70 | 7188.4 |
| stage/main/post_update | 4.256 | 4.962 | 6.67 | 6613.3 |
| render/graph/ms | 4.049 | 4.816 | 6.68 | 6292.0 |
| encode//sum | 3.904 | 4.910 | 15.44 | 6066.5 |
| stage/render/graph_submit | 3.050 | 3.770 | 5.56 | 4740.0 |
| encode/submit | 1.917 | 2.304 | 4.91 | 2979.4 |
| stage/main/update | 1.777 | 2.191 | 3.34 | 2761.5 |
| stage/render/prepare_views | 1.493 | 3.137 | 3.83 | 2320.3 |
| render/acquire/ms | 1.450 | 3.093 | 3.77 | 2253.9 |
| encode/finish_wall | 1.163 | 1.633 | 2.42 | 1808.0 |
| stage/render/camera_driver | 0.944 | 1.084 | 1.86 | 1466.9 |
| encode/pass:shadow_directional_light/sum | 0.740 | 1.278 | 1.88 | 1149.2 |
| encode/pass:early prepass/sum | 0.704 | 0.957 | 2.17 | 1094.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.695 | 0.970 | 1.22 | 1079.5 |
| stage/render/prepare_write_phase_buffers | 0.552 | 0.764 | 1.05 | 858.0 |
| stage/render/prepare_assets | 0.452 | 0.573 | 1.76 | 702.6 |
| stage/render/prepare_resources | 0.384 | 0.443 | 0.87 | 596.4 |
| bigspace/total/ms | 0.365 | 0.848 | 1.60 | 566.6 |
| render/present/ms | 0.359 | 0.429 | 2.14 | 558.5 |
| stage/main/pre_update | 0.341 | 0.436 | 0.91 | 529.2 |
| stage/main/first | 0.293 | 0.350 | 1.63 | 455.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.239 | 0.346 | 1.67 | 370.9 |
| terrain/update/ms | 0.233 | 0.300 | 0.65 | 361.8 |
| stage/render/prepare_bind_groups | 0.205 | 0.279 | 0.46 | 318.9 |
| terrain/select/ms | 0.203 | 0.262 | 0.61 | 315.5 |
| stage/render/prepare_meshes | 0.196 | 0.238 | 2.38 | 304.3 |
| stream/drive/ms | 0.185 | 0.245 | 0.60 | 287.2 |
| stage/main/fixed_main_loop | 0.178 | 0.332 | 0.83 | 276.9 |
| stage/render/prepare_batch_phases | 0.174 | 0.232 | 0.35 | 270.0 |
| stage/render/extract_commands | 0.173 | 0.205 | 0.30 | 268.7 |
| stage/render/create_views | 0.167 | 0.213 | 0.85 | 260.0 |
| far/select/ms | 0.165 | 0.224 | 0.71 | 256.9 |
| stream/tick/ms | 0.157 | 0.211 | 0.57 | 244.4 |
| controller/feed/ms | 0.152 | 0.199 | 0.61 | 236.9 |
| stage/render/prepare_flush | 0.151 | 0.294 | 0.59 | 234.9 |
| bigspace/high_precision/ms | 0.132 | 0.334 | 1.41 | 205.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.119 | 0.184 | 0.33 | 184.6 |
| controller/move/ms | 0.119 | 0.169 | 0.63 | 184.2 |
| stage/main/last | 0.116 | 0.154 | 0.61 | 180.4 |
| stage/render/queue | 0.106 | 0.137 | 5.72 | 164.2 |
| bigspace/low_precision/ms | 0.089 | 0.219 | 1.16 | 137.6 |
| stage/render/specialize | 0.084 | 0.105 | 8.45 | 130.3 |
| anim/evaluate/ms | 0.083 | 0.183 | 0.90 | 128.5 |
| anim/evaluate_cpu/ms | 0.060 | 0.076 | 0.44 | 93.6 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.115 | 0.22 | 90.7 |
| encode/resolve_diagnostics/sum | 0.055 | 0.089 | 0.43 | 85.9 |
| physics/step/ms | 0.028 | 0.055 | 0.41 | 42.8 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.047 | 0.10 | 42.7 |
| weather/map/ms | 0.025 | 0.032 | 0.46 | 38.2 |
| encode/camera_driver_span_end/sum | 0.024 | 0.040 | 0.33 | 37.5 |
| stage/render/cleanup | 0.023 | 0.031 | 0.36 | 35.2 |
| stage/render/phase_sort | 0.020 | 0.035 | 0.07 | 30.7 |
| weather/eval/ms | 0.010 | 0.014 | 0.04 | 16.3 |
| script/update/ms | 0.007 | 0.009 | 0.17 | 11.1 |
| controller/avoid/ms | 0.006 | 0.007 | 0.42 | 9.2 |
| script/input/ms | 0.005 | 0.006 | 0.06 | 8.2 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 7.0 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 5.5 |
| controller/write/ms | 0.002 | 0.002 | 0.03 | 3.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.07 | 0.9 |
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
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.53 | 2.04 | 0.46 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.10 | 0.04 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.33 | 0.02 |
| encode/finish_critical | ms | 0.84 | 2.17 | 0.78 |
| encode/pass:early prepass/max | ms | 0.70 | 2.17 | 0.75 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.71 | 1.22 | 0.78 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.33 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.67 | 0.27 |
| encode/pass:shadow_directional_light/max | ms | 0.68 | 1.88 | 0.50 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.43 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.17 | 25.02 | 9.26 |
| gpu/timed/ms | ms | 9.17 | 10.45 | 9.26 |
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
| phases/opaque3d/bins | count | 1307 | 1307 | 1307 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/bins | count | 1307 | 1307 | 1307 |
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
| process/cpu_usage | % | 25.48 | 37.95 | 20.67 |
| process/mem_usage | GiB | 4.67 | 4.69 | 4.69 |
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
| skin/blocks_changed/count | count | 5 | 6 | 4 |
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
| system/cpu_usage | % | 29.32 | 56.02 | 28.71 |
| system/mem_usage | % | 54.29 | 54.38 | 54.38 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 53 | 82 | 53 |
| views/all/bins | count | 5326 | 8063 | 5326 |
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
| views/main/bins | count | 3174 | 3174 | 3174 |
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
| anim/events/count | count | 0.10 | 3 | 160 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.19 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 29 | 41.3 k |
| encode/bind_walk/init_textures | count | 7164 | 14.0 k | 11.13 M |
| encode/bind_walk/init_textures_skipped | count | 39.0 k | 50.6 k | 60.60 M |
| encode/bind_walk/inits | count | 345 | 437 | 536.7 k |
| encode/bind_walk/inits_skipped | count | 70.50 | 81 | 109.5 k |
| encode/bind_walk/merge_textures | count | 7100 | 13.9 k | 11.03 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.34 M |
| encode/bind_walk/merges | count | 82.25 | 103 | 127.8 k |
| encode/bind_walk/merges_skipped | count | 52.50 | 57 | 81.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1554 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1554 |
| encode/pass:early prepass/count | count | 1 | 1 | 1554 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1554 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1554 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1554 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1554 |
| encode/queue/command_buffers | count | 36.57 | 39 | 56.8 k |
| encode/queue/submits | count | 2 | 2 | 3108 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.65 M | 2.23 G |
| encode/queue/write_buffer/calls | count | 217 | 287 | 337.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.72 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 14.0 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1554 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 2 | 901 |
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

## Hitches (> 25.0 ms; slowest 60, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2486.5 | main/busy/ms 2457, stage/main/post_update 2349, stage/main/update 99.87, surface/advance_cpu_ms 70.09, stage/main/pre_update 4.78, stage/main/last 1.02, surface/cell_upload_ms 0.71, stream/drive/ms 0.65, stage/main/first 0.48, stage/main/fixed_main_loop 0.35, weather/map/ms 0.24, stream/tick/ms 0.17 |
| warm-up | 3.32 | 836.1 | main/busy/ms 2624, stage/main/post_update 2371, render/thread/ms 805, render/busy/ms 637, stage/render/prepare_assets 589, stage/main/update 239, stage/render/prepare_views 162, render/acquire/ms 162, surface/advance_cpu_ms 70.09, stage/render/create_views 28.35, stage/render/render 15.56, render/graph/ms 8.61 |
| warm-up | 3.44 | 114.2 | render/thread/ms 903, render/busy/ms 683, stage/render/prepare_assets 622, main/busy/ms 190, stage/render/prepare_views 163, render/acquire/ms 163, stage/main/update 145, stage/render/render 72.33, render/present/ms 57.73, stage/main/post_update 33.06, stage/render/create_views 29.01, terrain/update/ms 16.22 |
| warm-up | 3.46 | 27.4 | pipelines/compile_ms 247, pipelines/front_ms 209, render/thread/ms 115, stage/render/render 68.85, main/busy/ms 62.81, render/busy/ms 61.04, render/present/ms 53.63, stage/main/post_update 44.19, pipelines/driver_ms 38.25, stage/render/prepare_assets 33.60, pipelines/wait_ms 21.75, terrain/update/ms 21.54 |
| warm-up | 3.60 | 132.5 | pipelines/wait_ms 3700, pipelines/compile_ms 1340, pipelines/front_ms 1233, render/thread/ms 135, render/busy/ms 132, pipelines/driver_ms 107, main/busy/ms 70.86, stage/main/post_update 59.95, stage/render/prepare_assets 49.57, stage/render/prepare_resources 39.43, stage/render/render 19.38, render/graph/ms 15.87 |
| warm-up | 3.66 | 66.8 | pipelines/wait_ms 3911, pipelines/compile_ms 1759, pipelines/front_ms 1649, render/thread/ms 182, render/busy/ms 180, main/busy/ms 116, pipelines/driver_ms 110, stage/render/prepare_assets 64.05, encode//sum 59.51, stage/main/update 58.62, stage/render/prepare_resources 39.53, stage/main/post_update 36.70 |
| warm-up | 3.80 | 132.2 | pipelines/compile_ms 2128, pipelines/front_ms 1942, pipelines/wait_ms 319, encode//sum 274, pipelines/driver_ms 186, render/thread/ms 167, render/busy/ms 164, main/busy/ms 104, stage/render/prepare_assets 82.40, stage/main/update 62.97, stage/render/render 56.82, render/graph/ms 54.34 |
| warm-up | 3.86 | 43.6 | pipelines/compile_ms 1022, pipelines/front_ms 975, render/thread/ms 54.95, render/busy/ms 53.29, pipelines/driver_ms 47.07, pipelines/wait_ms 41.95, main/busy/ms 37.40, stage/render/prepare_assets 33.72, stage/main/update 17.14, stage/main/post_update 14.73, stage/render/render 12.51, render/graph/ms 10.32 |
| warm-up | 3.92 | 43.5 | pipelines/compile_ms 110, pipelines/front_ms 104, render/thread/ms 54.21, render/busy/ms 52.69, main/busy/ms 32.69, stage/render/prepare_assets 31.37, stage/main/post_update 17.03, stage/render/render 12.66, encode//sum 12.19, render/graph/ms 10.67, terrain/update/ms 10.12, stage/render/graph_submit 8.31 |
| warm-up | 4.01 | 78.6 | pipelines/compile_ms 958, pipelines/front_ms 919, pipelines/wait_ms 57.53, main/busy/ms 52.04, render/thread/ms 48.59, render/busy/ms 47.14, stage/main/update 42.96, pipelines/driver_ms 39.26, stream/drive/ms 38.89, stage/render/prepare_assets 28.06, stage/render/render 14.28, render/graph/ms 12.55 |
| warm-up | 4.09 | 71.4 | pipelines/compile_ms 442, pipelines/front_ms 425, render/thread/ms 53.69, render/busy/ms 52.07, main/busy/ms 39.10, stage/render/prepare_assets 37.33, stage/main/update 25.56, stream/drive/ms 21.33, pipelines/driver_ms 16.71, stage/render/render 9.76, render/graph/ms 8.02, stage/main/post_update 7.26 |
| warm-up | 4.12 | 25.5 | pipelines/compile_ms 431, pipelines/front_ms 421, render/thread/ms 64.60, render/busy/ms 63.47, stage/render/prepare_assets 50.51, main/busy/ms 39.87, stage/main/update 26.60, stream/drive/ms 22.14, pipelines/driver_ms 10.93, stage/main/post_update 8.17, stage/render/render 7.87, render/graph/ms 6.42 |
| warm-up | 4.20 | 81.5 | pipelines/compile_ms 362, pipelines/front_ms 354, render/thread/ms 65.36, render/busy/ms 63.99, main/busy/ms 53.87, stage/render/prepare_assets 43.93, stage/main/update 33.60, stream/drive/ms 23.25, stage/main/post_update 11.54, stage/render/render 10.15, pipelines/driver_ms 8.37, render/graph/ms 8.37 |
| warm-up | 4.27 | 70.6 | pipelines/compile_ms 937, pipelines/front_ms 911, main/busy/ms 122, stage/main/update 98.31, render/thread/ms 89.32, render/busy/ms 87.68, stream/drive/ms 61.81, stage/render/prepare_assets 58.74, pipelines/driver_ms 25.60, stage/render/render 16.99, render/graph/ms 14.73, stage/main/post_update 13.65 |
| warm-up | 4.44 | 172.9 | pipelines/compile_ms 2572, pipelines/front_ms 2374, pipelines/driver_ms 198, render/thread/ms 124, render/busy/ms 123, main/busy/ms 116, stage/main/update 96.20, stage/render/prepare_assets 64.80, stream/drive/ms 61.93, stage/render/render 49.17, render/graph/ms 47.20, stage/render/graph_submit 44.90 |
| warm-up | 4.50 | 50.9 | pipelines/compile_ms 1675, pipelines/front_ms 1501, pipelines/driver_ms 174, render/thread/ms 115, render/busy/ms 114, stage/render/prepare_assets 56.46, main/busy/ms 51.68, stage/render/render 47.48, pipelines/wait_ms 46.02, render/graph/ms 45.60, stage/render/graph_submit 43.35, encode//sum 38.74 |
| warm-up | 4.54 | 41.8 | pipelines/compile_ms 104, pipelines/front_ms 97.26, render/thread/ms 73.98, render/busy/ms 71.04, main/busy/ms 58.33, pipelines/wait_ms 46.20, stage/main/update 32.64, stage/render/prepare_assets 25.81, stage/render/render 24.55, render/graph/ms 22.52, stage/render/graph_submit 20.14, stage/main/post_update 16.93 |
| warm-up | 4.58 | 41.3 | render/thread/ms 69.59, render/busy/ms 66.85, pipelines/compile_ms 64.72, pipelines/front_ms 59.35, main/busy/ms 46.60, stage/main/update 30.11, stage/render/prepare_assets 23.65, stage/render/render 22.33, render/graph/ms 20.27, stage/render/graph_submit 17.97, encode/submit 15.00, stage/main/post_update 10.33 |
| warm-up | 4.61 | 28.0 | pipelines/compile_ms 742, pipelines/front_ms 717, render/thread/ms 51.30, render/busy/ms 49.99, main/busy/ms 33.60, stage/render/prepare_assets 27.26, pipelines/driver_ms 24.26, stage/main/update 18.94, stage/render/render 10.32, stage/main/post_update 8.62, render/graph/ms 8.40, stage/render/graph_submit 6.14 |
| warm-up | 4.63 | 26.8 | pipelines/compile_ms 960, pipelines/front_ms 928, render/thread/ms 45.63, render/busy/ms 43.88, pipelines/wait_ms 37.43, main/busy/ms 36.11, pipelines/driver_ms 32.22, stage/main/post_update 16.78, stage/main/update 13.71, stage/render/render 11.11, stage/render/prepare_resources 10.86, stage/render/prepare_assets 10.52 |
| warm-up | 4.66 | 27.2 | pipelines/compile_ms 626, pipelines/front_ms 607, pipelines/wait_ms 109, render/thread/ms 45.75, render/busy/ms 43.86, main/busy/ms 40.37, pipelines/driver_ms 18.99, stage/main/post_update 18.03, stage/main/update 14.24, stage/render/render 13.65, stage/render/prepare_resources 12.17, render/graph/ms 11.48 |
| warm-up | 4.70 | 39.9 | pipelines/compile_ms 1209, pipelines/front_ms 1172, main/busy/ms 111, stage/main/update 94.05, pipelines/wait_ms 89.25, stream/drive/ms 88.00, render/thread/ms 52.41, render/busy/ms 50.89, pipelines/driver_ms 37.00, stage/render/render 14.60, stage/render/prepare_assets 13.35, render/graph/ms 12.40 |
| warm-up | 4.79 | 90.4 | pipelines/compile_ms 1179, pipelines/front_ms 1143, main/busy/ms 105, stage/main/update 92.41, stream/drive/ms 87.00, render/thread/ms 52.70, render/busy/ms 51.28, pipelines/driver_ms 36.60, stage/render/prepare_assets 18.88, pipelines/wait_ms 16.53, stage/render/render 12.54, render/graph/ms 10.50 |
| warm-up | 4.82 | 32.6 | pipelines/compile_ms 832, pipelines/front_ms 804, render/thread/ms 50.58, render/busy/ms 49.31, main/busy/ms 39.15, pipelines/driver_ms 28.37, stage/render/prepare_assets 14.31, stage/main/update 12.27, stage/render/prepare_bind_groups 11.34, stage/render/render 10.06, stage/main/fixed_main_loop 9.88, stage/main/post_update 9.40 |
| warm-up | 4.85 | 26.1 | pipelines/compile_ms 1015, pipelines/front_ms 973, render/thread/ms 52.86, render/busy/ms 51.68, main/busy/ms 48.92, pipelines/driver_ms 42.45, stage/main/post_update 15.81, stage/render/prepare_resources 15.35, stage/main/update 13.18, stage/render/prepare_bind_groups 12.21, stage/render/render 10.88, stage/main/fixed_main_loop 9.92 |
| warm-up | 4.93 | 67.3 | pipelines/compile_ms 576, pipelines/front_ms 557, main/busy/ms 69.98, stage/main/update 51.30, render/thread/ms 44.53, render/busy/ms 42.88, stage/render/prepare_assets 25.53, pipelines/driver_ms 19.17, stream/drive/ms 11.97, stage/render/render 11.27, stage/main/post_update 10.37, render/graph/ms 9.19 |
| warm-up | 4.98 | 44.3 | pipelines/compile_ms 583, pipelines/front_ms 565, render/thread/ms 65.38, render/busy/ms 64.01, main/busy/ms 45.11, stage/render/prepare_assets 35.05, stage/main/update 30.27, pipelines/driver_ms 17.76, stage/render/render 12.55, render/graph/ms 10.82, stage/render/graph_submit 8.45, stage/main/post_update 8.14 |
| warm-up | 5.03 | 27.9 | pipelines/compile_ms 824, pipelines/front_ms 799, pipelines/wait_ms 60.13, main/busy/ms 43.93, render/thread/ms 40.72, render/busy/ms 39.35, stage/main/update 31.51, pipelines/driver_ms 25.45, stage/render/render 17.71, render/graph/ms 15.78, stream/drive/ms 15.59, stage/render/graph_submit 10.71 |
| warm-up | 5.06 | 32.8 | pipelines/compile_ms 271, pipelines/front_ms 262, render/thread/ms 48.57, render/busy/ms 46.99, main/busy/ms 41.08, stage/main/update 25.32, pipelines/wait_ms 18.97, stage/render/render 18.27, render/graph/ms 16.00, stream/drive/ms 15.64, stage/render/graph_submit 12.39, stage/render/prepare_assets 11.11 |
| warm-up | 5.08 | 25.5 | pipelines/compile_ms 63.25, pipelines/front_ms 59.40, render/thread/ms 49.24, render/busy/ms 47.66, main/busy/ms 33.80, stage/main/update 16.07, stage/render/render 15.26, render/graph/ms 13.14, stage/main/post_update 11.51, encode//sum 9.49, stage/render/graph_submit 9.37, stage/render/prepare_write_phase_buffers 8.83 |
| warm-up | 5.12 | 34.6 | pipelines/compile_ms 304, pipelines/front_ms 292, main/busy/ms 47.87, render/thread/ms 45.93, render/busy/ms 44.45, stage/main/update 29.83, stage/render/prepare_assets 16.35, pipelines/driver_ms 12.01, stage/main/post_update 11.80, stage/render/render 11.16, render/graph/ms 9.34, encode//sum 7.94 |
| warm-up | 5.17 | 48.4 | pipelines/compile_ms 508, pipelines/front_ms 489, render/thread/ms 56.92, render/busy/ms 55.34, main/busy/ms 47.27, stage/render/prepare_assets 32.58, stage/main/update 27.98, pipelines/driver_ms 19.49, stage/main/post_update 12.75, stage/render/render 10.76, render/graph/ms 8.66, stream/drive/ms 6.64 |
| warm-up | 5.20 | 32.0 | pipelines/compile_ms 241, pipelines/front_ms 231, render/thread/ms 62.04, render/busy/ms 60.37, main/busy/ms 32.78, stage/render/prepare_assets 23.89, stage/render/render 17.97, render/graph/ms 15.58, stage/main/update 14.30, stage/render/graph_submit 12.36, stage/main/post_update 12.10, pipelines/driver_ms 9.64 |
| warm-up | 5.24 | 38.6 | pipelines/compile_ms 638, pipelines/front_ms 620, render/thread/ms 55.43, render/busy/ms 53.88, main/busy/ms 33.09, stage/render/prepare_assets 22.11, pipelines/driver_ms 18.29, stage/render/render 16.69, stage/main/update 16.50, render/graph/ms 14.45, stage/render/graph_submit 11.28, stage/main/post_update 10.73 |
| warm-up | 5.28 | 42.8 | pipelines/compile_ms 1035, pipelines/front_ms 1007, render/thread/ms 62.32, render/busy/ms 61.02, main/busy/ms 43.07, stage/render/prepare_assets 40.30, pipelines/driver_ms 27.38, stage/main/update 15.89, stage/main/post_update 13.16, stage/main/pre_update 9.91, stage/render/render 9.27, render/graph/ms 7.31 |
| warm-up | 5.31 | 26.8 | pipelines/compile_ms 671, pipelines/front_ms 652, render/thread/ms 60.38, render/busy/ms 58.66, main/busy/ms 45.40, stage/render/prepare_assets 27.13, stage/main/post_update 19.20, pipelines/driver_ms 18.62, stage/main/update 12.22, stage/render/render 11.83, stage/main/pre_update 9.74, render/graph/ms 9.52 |
| warm-up | 5.36 | 26.3 | pipelines/compile_ms 295, pipelines/front_ms 287, render/thread/ms 48.66, render/busy/ms 47.39, main/busy/ms 36.66, stage/main/update 16.67, stage/main/post_update 15.22, pipelines/wait_ms 11.26, stage/render/render 11.02, stream/drive/ms 11.02, render/graph/ms 9.36, stage/render/prepare_resources 8.90 |
| warm-up | 5.40 | 36.2 | pipelines/compile_ms 841, pipelines/front_ms 818, render/thread/ms 58.92, render/busy/ms 57.70, main/busy/ms 39.28, stage/render/prepare_assets 24.73, stage/main/update 24.35, pipelines/driver_ms 22.16, pipelines/wait_ms 12.26, stream/drive/ms 11.55, stage/render/render 10.79, stage/main/post_update 9.88 |
| warm-up | 5.44 | 26.7 | pipelines/compile_ms 821, pipelines/front_ms 802, render/thread/ms 41.87, render/busy/ms 40.55, main/busy/ms 30.42, pipelines/wait_ms 27.28, stage/render/prepare_assets 20.07, pipelines/driver_ms 18.46, stage/main/update 13.18, stage/main/post_update 10.77, stage/render/render 10.36, render/graph/ms 8.71 |
| warm-up | 5.50 | 41.8 | pipelines/compile_ms 964, pipelines/front_ms 946, render/thread/ms 50.67, render/busy/ms 49.49, main/busy/ms 35.90, pipelines/wait_ms 32.31, stage/render/prepare_assets 28.32, stage/main/update 19.08, pipelines/driver_ms 17.58, stage/render/render 13.47, render/graph/ms 11.88, stage/main/post_update 11.23 |
| warm-up | 5.55 | 26.4 | render/thread/ms 38.75, render/busy/ms 37.28, main/busy/ms 24.30, stage/render/render 14.51, stage/render/prepare_assets 12.76, render/graph/ms 12.65, stage/main/update 9.99, stage/render/graph_submit 9.94, stage/main/post_update 9.31, encode//sum 7.83, pipelines/compile_ms 7.21, pipelines/front_ms 6.72 |
| warm-up | 5.58 | 34.2 | pipelines/compile_ms 107, pipelines/front_ms 101, render/thread/ms 54.81, render/busy/ms 52.97, main/busy/ms 26.73, stage/render/prepare_assets 22.09, stage/render/render 15.89, render/graph/ms 13.61, stage/main/update 10.51, stage/main/post_update 10.42, stage/render/graph_submit 10.27, encode//sum 7.70 |
| warm-up | 5.61 | 25.3 | pipelines/compile_ms 99.94, pipelines/front_ms 94.07, render/thread/ms 51.69, render/busy/ms 49.97, main/busy/ms 29.29, stage/render/prepare_assets 21.35, stage/render/render 12.98, stage/main/update 11.38, stage/main/post_update 11.16, render/graph/ms 10.61, encode//sum 8.31, stage/render/graph_submit 6.94 |
| warm-up | 5.65 | 43.6 | render/thread/ms 52.88, render/busy/ms 51.58, main/busy/ms 31.71, stage/render/prepare_assets 30.27, stage/main/update 14.85, stage/main/post_update 10.74, stage/render/render 8.93, render/graph/ms 7.28, encode//sum 6.63, stage/render/graph_submit 4.81, stream/drive/ms 4.54, encode/submit 2.64 |
| warm-up | 5.68 | 29.8 | render/thread/ms 59.47, render/busy/ms 58.38, stage/render/prepare_assets 39.57, main/busy/ms 30.64, stage/main/update 13.74, stage/main/post_update 10.99, stage/render/render 7.99, encode//sum 6.52, render/graph/ms 6.50, stream/drive/ms 4.50, stage/render/graph_submit 4.46, encode/submit 2.57 |
| warm-up | 5.71 | 31.9 | pipelines/compile_ms 342, pipelines/front_ms 334, render/thread/ms 53.36, render/busy/ms 52.15, stage/render/prepare_assets 35.06, main/busy/ms 28.81, stage/main/update 12.05, stage/main/post_update 10.57, stage/render/render 8.62, pipelines/driver_ms 8.06, render/graph/ms 6.83, encode//sum 6.70 |
| warm-up | 5.74 | 28.9 | pipelines/compile_ms 1056, pipelines/front_ms 1032, render/thread/ms 51.63, render/busy/ms 50.30, main/busy/ms 34.22, stage/render/prepare_assets 32.82, pipelines/driver_ms 24.69, stage/main/update 15.31, stage/main/post_update 13.41, stage/render/render 9.74, render/graph/ms 7.90, encode//sum 6.54 |
| warm-up | 5.77 | 29.7 | pipelines/compile_ms 881, pipelines/front_ms 857, render/thread/ms 51.46, render/busy/ms 47.46, main/busy/ms 41.65, pipelines/driver_ms 23.97, stage/main/update 23.37, stage/render/prepare_assets 20.15, stage/main/post_update 12.52, stage/render/render 12.14, render/graph/ms 10.39, stage/render/graph_submit 7.57 |
| warm-up | 5.80 | 33.3 | pipelines/compile_ms 166, pipelines/front_ms 159, render/thread/ms 52.54, render/busy/ms 48.42, main/busy/ms 42.54, stage/main/update 22.13, stage/render/render 14.99, stage/render/prepare_assets 14.16, stage/main/post_update 13.72, render/graph/ms 13.11, encode//sum 10.09, stage/render/graph_submit 9.72 |
| warm-up | 5.87 | 51.2 | main/busy/ms 44.72, render/thread/ms 37.89, render/busy/ms 36.53, stage/main/update 23.33, stage/render/prepare_assets 18.54, stream/drive/ms 13.99, stage/main/post_update 13.86, stage/render/render 9.38, render/graph/ms 7.48, encode//sum 7.20, stage/render/graph_submit 5.10, encode/submit 3.09 |
| warm-up | 5.91 | 40.0 | render/thread/ms 53.94, render/busy/ms 52.59, main/busy/ms 46.35, stage/render/prepare_assets 28.27, stage/main/update 23.79, stage/main/post_update 15.11, pipelines/compile_ms 13.05, pipelines/front_ms 11.91, stage/render/render 10.88, render/graph/ms 8.91, stream/drive/ms 8.48, encode//sum 7.63 |
| warm-up | 5.94 | 27.6 | pipelines/compile_ms 933, pipelines/front_ms 909, render/thread/ms 49.93, render/busy/ms 48.51, main/busy/ms 44.59, stage/main/update 23.86, pipelines/driver_ms 23.67, stage/render/prepare_assets 23.18, stage/main/post_update 13.96, stage/render/render 12.27, render/graph/ms 10.45, stream/drive/ms 7.84 |
| warm-up | 6.11 | 38.5 | pipelines/compile_ms 572, pipelines/front_ms 553, main/busy/ms 49.92, render/thread/ms 33.43, render/busy/ms 32.12, stage/main/update 31.91, stream/drive/ms 23.09, pipelines/driver_ms 19.54, stage/render/render 12.62, stage/main/post_update 10.95, render/graph/ms 10.70, stage/render/prepare_assets 10.50 |
| warm-up | 6.20 | 28.3 | pipelines/compile_ms 182, pipelines/front_ms 176, main/busy/ms 36.42, render/thread/ms 28.61, render/busy/ms 27.29, stage/main/update 18.21, stage/main/post_update 12.10, stage/render/render 10.83, stage/render/prepare_assets 10.74, render/graph/ms 8.98, encode//sum 8.77, stage/render/graph_submit 6.05 |
| warm-up | 6.40 | 26.9 | pipelines/compile_ms 139, pipelines/front_ms 134, main/busy/ms 33.05, render/thread/ms 30.85, render/busy/ms 29.30, stage/render/render 16.43, stage/main/update 15.89, render/graph/ms 14.17, stage/main/post_update 12.96, encode//sum 11.10, stage/render/graph_submit 10.55, encode/submit 6.88 |
| warm-up | 6.43 | 25.6 | main/busy/ms 43.76, render/thread/ms 41.41, render/busy/ms 39.41, stage/main/update 20.60, stage/render/render 18.38, stage/main/post_update 17.19, render/graph/ms 15.34, stream/drive/ms 12.02, stage/render/prepare_assets 11.42, encode//sum 11.03, stage/render/graph_submit 10.91, encode/submit 7.13 |
| warm-up | 6.47 | 43.8 | render/thread/ms 49.28, render/busy/ms 47.80, main/busy/ms 41.98, stage/render/prepare_assets 25.58, stage/main/update 19.02, stage/main/post_update 16.85, stage/render/render 14.09, stream/drive/ms 12.35, render/graph/ms 11.86, encode//sum 9.06, stage/render/graph_submit 8.42, encode/submit 5.50 |
| warm-up | 6.83 | 30.8 | pipelines/compile_ms 130, pipelines/front_ms 126, render/thread/ms 37.22, render/busy/ms 35.70, main/busy/ms 14.68, stage/render/render 10.64, stage/render/specialize 9.40, render/graph/ms 9.32, stage/main/post_update 7.94, stage/render/graph_submit 7.45, encode//sum 6.98, stage/render/queue 6.71 |
| warm-up | 8.03 | 29.2 | render/thread/ms 36.01, render/busy/ms 33.26, main/busy/ms 14.50, stage/render/render 9.93, render/graph/ms 8.61, stage/main/post_update 8.13, stage/render/specialize 8.04, stage/render/graph_submit 6.71, encode//sum 6.07, stage/render/queue 5.87, stage/render/prepare_meshes 4.59, stage/main/update 4.16 |
| measure | 0.83 | 26.9 | render/thread/ms 34.47, render/busy/ms 31.51, main/busy/ms 12.92, stage/render/render 10.41, render/graph/ms 8.92, stage/render/specialize 8.53, stage/main/post_update 7.81, encode//sum 7.27, stage/render/graph_submit 6.92, stage/render/queue 5.85, encode/submit 4.25, stage/main/update 3.00 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=base-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 9%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 84%) · 2625 MHz (min 2625) · memory 8101 MHz · 99.0 W · 79 °C (max 82) · VRAM 6445 MiB · throttled 100% of samples

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
