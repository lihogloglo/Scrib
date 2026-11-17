# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1543 frames over 15.0 s · **median 9.74 ms (103 fps)** · mean 9.73 · p95 11.11 · p99 11.54 (1% low 87 fps) · worst 26.39 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 278 frames over 8.3 s, worst 2474.7 ms
- hitches (> 25.0 ms): **1** measured, 54 in warm-up
- timed GPU passes: 8.59 ms of a 9.73 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.74 ms, limit 11 ms
- pass worst frame 26.39 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.928 | 2.935 | 2.978 | 1.00 | 0.064 |
| dlss_super_resolution | 2.015 | 2.020 | 2.044 | 1.00 | 0.185 |
| early prepass | 0.618 | 0.620 | 0.631 | 1.00 | 0.091 |
| atmosphere_luts | 0.333 | 0.334 | 0.344 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.314 | 0.315 | 0.336 | 1.00 | 0.008 |
| bloom | 0.313 | 0.314 | 0.330 | 1.00 | 0.060 |
| water_velocity | 0.239 | 0.240 | 0.265 | 1.00 | 0.002 |
| water_sspr | 0.207 | 0.208 | 0.210 | 1.00 | 0.002 |
| ssao | 0.191 | 0.191 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.148 | 0.149 | 0.169 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.147 | 0.153 | 1.00 | 0.001 |
| water_fft | 0.131 | 0.132 | 0.135 | 1.00 | 0.003 |
| uniform_allocation | 0.119 | 0.120 | 0.162 | 1.00 | 0.021 |
| upscaling | 0.113 | 0.114 | 0.119 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.092 | 0.092 | 0.122 | 1.00 | 0.016 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.412 | 0.13 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 1.00 | 0.005 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.030 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.221 | 0.230 | 0.13 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 1.00 | 0.083 |
| water_snapshot | 0.024 | 0.024 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.043 | 0.046 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 1.00 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.13 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.806 | 10.215 | 25.13 | 13587.0 |
| main/busy/ms | 7.026 | 7.931 | 9.10 | 10841.9 |
| render/busy/ms | 6.820 | 7.818 | 24.51 | 10523.5 |
| stage/render/render | 4.541 | 5.358 | 12.86 | 7006.2 |
| stage/main/post_update | 4.189 | 4.909 | 6.51 | 6464.0 |
| render/graph/ms | 3.976 | 4.772 | 6.86 | 6134.9 |
| encode//sum | 3.735 | 4.968 | 9.30 | 5763.8 |
| stage/render/graph_submit | 2.993 | 3.737 | 5.92 | 4618.3 |
| encode/submit | 1.873 | 2.296 | 9.48 | 2889.5 |
| stage/main/update | 1.740 | 2.158 | 3.36 | 2684.2 |
| stage/render/prepare_views | 1.683 | 3.355 | 4.53 | 2597.0 |
| render/acquire/ms | 1.638 | 3.315 | 4.45 | 2528.0 |
| encode/finish_wall | 1.141 | 1.599 | 2.26 | 1760.7 |
| stage/render/camera_driver | 0.927 | 1.066 | 1.33 | 1430.8 |
| encode/pass:shadow_directional_light/sum | 0.729 | 1.240 | 2.06 | 1125.4 |
| encode/pass:early prepass/sum | 0.693 | 0.954 | 1.23 | 1068.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.692 | 0.972 | 1.25 | 1067.6 |
| stage/render/prepare_write_phase_buffers | 0.544 | 0.746 | 0.92 | 838.6 |
| stage/render/prepare_assets | 0.441 | 0.564 | 1.57 | 680.0 |
| stage/render/prepare_resources | 0.380 | 0.448 | 0.56 | 586.6 |
| bigspace/total/ms | 0.351 | 0.665 | 1.47 | 542.2 |
| render/present/ms | 0.347 | 0.413 | 5.78 | 535.5 |
| stage/main/pre_update | 0.336 | 0.423 | 0.80 | 517.9 |
| stage/main/first | 0.289 | 0.345 | 1.53 | 446.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.231 | 0.332 | 0.54 | 356.8 |
| terrain/update/ms | 0.230 | 0.293 | 0.69 | 355.2 |
| stage/render/prepare_bind_groups | 0.205 | 0.279 | 0.55 | 316.5 |
| terrain/select/ms | 0.200 | 0.254 | 0.34 | 308.5 |
| stage/render/prepare_meshes | 0.196 | 0.243 | 2.29 | 303.0 |
| stream/drive/ms | 0.183 | 0.232 | 0.62 | 281.9 |
| stage/main/fixed_main_loop | 0.177 | 0.322 | 0.90 | 273.6 |
| stage/render/prepare_batch_phases | 0.174 | 0.234 | 0.46 | 268.9 |
| stage/render/extract_commands | 0.166 | 0.196 | 0.45 | 256.9 |
| stage/render/create_views | 0.165 | 0.213 | 0.76 | 254.6 |
| far/select/ms | 0.159 | 0.201 | 0.61 | 245.2 |
| stream/tick/ms | 0.155 | 0.197 | 0.59 | 239.5 |
| controller/feed/ms | 0.153 | 0.200 | 0.60 | 236.5 |
| stage/render/prepare_flush | 0.148 | 0.304 | 0.66 | 228.3 |
| bigspace/high_precision/ms | 0.127 | 0.318 | 1.24 | 195.9 |
| controller/move/ms | 0.120 | 0.177 | 0.53 | 184.8 |
| stage/main/last | 0.117 | 0.157 | 0.65 | 181.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.114 | 0.170 | 0.43 | 175.2 |
| stage/render/queue | 0.106 | 0.134 | 5.45 | 163.9 |
| bigspace/low_precision/ms | 0.085 | 0.222 | 1.18 | 131.3 |
| anim/evaluate/ms | 0.083 | 0.190 | 1.10 | 128.3 |
| stage/render/specialize | 0.082 | 0.103 | 8.13 | 126.9 |
| anim/evaluate_cpu/ms | 0.061 | 0.076 | 0.13 | 94.5 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.113 | 0.18 | 89.5 |
| encode/resolve_diagnostics/sum | 0.054 | 0.082 | 0.15 | 82.6 |
| physics/step/ms | 0.026 | 0.053 | 0.29 | 40.7 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.038 | 0.17 | 39.9 |
| weather/map/ms | 0.024 | 0.032 | 0.40 | 37.5 |
| encode/camera_driver_span_end/sum | 0.023 | 0.035 | 0.14 | 34.8 |
| stage/render/cleanup | 0.022 | 0.031 | 0.29 | 34.7 |
| stage/render/phase_sort | 0.019 | 0.033 | 0.15 | 29.0 |
| weather/eval/ms | 0.011 | 0.014 | 0.43 | 16.6 |
| script/update/ms | 0.006 | 0.008 | 0.03 | 9.5 |
| controller/avoid/ms | 0.006 | 0.008 | 0.02 | 9.0 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 8.2 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 6.8 |
| script/ui/ms | 0.004 | 0.005 | 0.35 | 6.0 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 1.0 |
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
| crowd/c2/count | count | 283 | 283 | 283 |
| crowd/c2/joints/count | count | 4916 | 4916 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1373 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.52 | 1.95 | 1.14 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.17 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.14 | 0.03 |
| encode/finish_critical | ms | 0.82 | 2.06 | 1.14 |
| encode/pass:early prepass/max | ms | 0.69 | 1.23 | 0.84 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.71 | 1.25 | 0.99 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.43 | 0.16 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 0.54 | 0.29 |
| encode/pass:shadow_directional_light/max | ms | 0.67 | 2.06 | 0.72 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.15 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.24 | 24.51 | 9.41 |
| gpu/timed/ms | ms | 9.24 | 10.76 | 9.41 |
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
| process/cpu_usage | % | 25.96 | 35.96 | 27.88 |
| process/mem_usage | GiB | 4.66 | 4.72 | 4.72 |
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
| skin/blocks_changed/count | count | 5 | 6 | 6 |
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
| system/cpu_usage | % | 29.17 | 42.97 | 33.42 |
| system/mem_usage | % | 54.21 | 54.40 | 54.40 |
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
| anim/events/count | count | 0.09 | 3 | 144 |
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
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 29 | 41.0 k |
| encode/bind_walk/init_textures | count | 7164 | 14.0 k | 11.05 M |
| encode/bind_walk/init_textures_skipped | count | 31.7 k | 44.8 k | 48.94 M |
| encode/bind_walk/inits | count | 345 | 437 | 532.9 k |
| encode/bind_walk/inits_skipped | count | 62.99 | 75 | 97.2 k |
| encode/bind_walk/merge_textures | count | 7100 | 13.9 k | 10.96 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.30 M |
| encode/bind_walk/merges | count | 82.25 | 103 | 126.9 k |
| encode/bind_walk/merges_skipped | count | 48.75 | 54 | 75.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1543 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1543 |
| encode/pass:early prepass/count | count | 1 | 1 | 1543 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1543 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1543 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1543 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1543 |
| encode/queue/command_buffers | count | 36.57 | 39 | 56.4 k |
| encode/queue/submits | count | 2 | 2 | 3086 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.65 M | 2.22 G |
| encode/queue/write_buffer/calls | count | 217 | 287 | 335.3 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.36 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.9 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1543 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 1 | 901 |
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

## Hitches (> 25.0 ms; slowest 55, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2474.7 | main/busy/ms 2446, stage/main/post_update 2342, stage/main/update 96.00, surface/advance_cpu_ms 67.12, stage/main/pre_update 4.68, stage/main/last 1.18, surface/cell_upload_ms 0.75, stream/drive/ms 0.51, stage/main/first 0.44, stage/main/fixed_main_loop 0.39, weather/map/ms 0.17, stream/tick/ms 0.12 |
| warm-up | 3.21 | 732.5 | main/busy/ms 2587, stage/main/post_update 2370, render/thread/ms 703, render/busy/ms 534, stage/render/prepare_assets 486, stage/main/update 203, stage/render/prepare_views 164, render/acquire/ms 163, surface/advance_cpu_ms 67.12, stage/render/create_views 29.67, stage/render/render 13.56, render/graph/ms 7.29 |
| warm-up | 3.38 | 173.8 | render/thread/ms 860, render/busy/ms 602, stage/render/prepare_assets 538, main/busy/ms 168, stage/render/prepare_views 164, render/acquire/ms 164, stage/main/update 113, stage/render/render 108, render/present/ms 94.40, stage/main/post_update 42.18, stage/render/create_views 29.96, pipelines/compile_ms 22.80 |
| warm-up | 3.59 | 187.2 | pipelines/wait_ms 4636, pipelines/compile_ms 2253, pipelines/front_ms 1986, pipelines/driver_ms 267, main/busy/ms 148, render/thread/ms 118, render/busy/ms 116, stage/render/prepare_assets 94.25, stage/main/post_update 79.81, stage/main/update 62.62, terrain/update/ms 14.27, stage/render/render 12.44 |
| warm-up | 3.66 | 64.3 | pipelines/wait_ms 4907, pipelines/compile_ms 2754, pipelines/front_ms 2544, pipelines/driver_ms 210, render/thread/ms 163, render/busy/ms 161, stage/render/prepare_assets 108, main/busy/ms 89.81, stage/main/post_update 71.59, encode//sum 42.07, stage/render/render 33.16, render/graph/ms 31.07 |
| warm-up | 3.70 | 44.3 | pipelines/compile_ms 806, pipelines/front_ms 743, pipelines/wait_ms 274, render/thread/ms 100, render/busy/ms 97.12, pipelines/driver_ms 63.66, main/busy/ms 52.32, encode//sum 42.09, stage/render/prepare_assets 41.90, stage/main/post_update 34.90, stage/render/render 34.75, render/graph/ms 31.91 |
| warm-up | 3.76 | 46.3 | pipelines/compile_ms 691, pipelines/front_ms 658, pipelines/wait_ms 112, render/thread/ms 52.38, render/busy/ms 50.89, pipelines/driver_ms 32.19, main/busy/ms 31.10, stage/render/prepare_assets 29.46, stage/main/post_update 17.04, stage/render/render 12.05, render/graph/ms 10.26, terrain/update/ms 9.08 |
| warm-up | 3.79 | 30.3 | pipelines/compile_ms 1667, pipelines/front_ms 1558, pipelines/wait_ms 120, encode//sum 111, pipelines/driver_ms 109, render/thread/ms 67.42, render/busy/ms 65.42, main/busy/ms 58.68, stage/main/update 42.36, stream/drive/ms 39.09, stage/render/prepare_assets 29.73, stage/render/render 28.08 |
| warm-up | 3.85 | 55.5 | pipelines/compile_ms 1387, pipelines/front_ms 1301, encode//sum 110, pipelines/driver_ms 86.16, render/thread/ms 60.03, main/busy/ms 59.02, render/busy/ms 57.84, stage/main/update 41.64, stream/drive/ms 37.91, stage/render/render 28.12, pipelines/wait_ms 27.01, render/graph/ms 25.57 |
| warm-up | 3.92 | 52.4 | pipelines/compile_ms 362, pipelines/front_ms 343, render/thread/ms 58.31, render/busy/ms 55.00, main/busy/ms 37.91, stage/render/prepare_assets 26.49, stage/main/update 25.14, stream/drive/ms 20.87, stage/render/render 20.13, pipelines/driver_ms 19.05, render/graph/ms 15.01, stage/render/graph_submit 10.40 |
| warm-up | 3.95 | 27.1 | pipelines/compile_ms 739, pipelines/front_ms 707, main/busy/ms 91.60, render/thread/ms 59.27, render/busy/ms 54.77, stage/main/update 51.58, stream/drive/ms 46.74, stage/render/prepare_assets 35.49, pipelines/driver_ms 32.75, stage/main/post_update 30.69, stage/render/render 15.22, render/graph/ms 10.24 |
| warm-up | 4.10 | 152.1 | pipelines/compile_ms 1099, pipelines/front_ms 1062, main/busy/ms 120, render/thread/ms 81.55, render/busy/ms 77.27, stage/main/update 74.16, stage/render/prepare_assets 59.91, stream/drive/ms 48.13, pipelines/driver_ms 36.95, stage/main/post_update 33.50, stage/render/render 13.04, render/graph/ms 8.18 |
| warm-up | 4.22 | 115.1 | pipelines/compile_ms 2028, pipelines/front_ms 1969, render/thread/ms 143, render/busy/ms 142, main/busy/ms 97.41, stage/render/prepare_assets 80.11, stage/main/update 75.08, pipelines/driver_ms 59.33, stream/drive/ms 38.86, stage/render/prepare_bind_groups 30.55, stage/render/render 22.19, render/graph/ms 20.15 |
| warm-up | 4.27 | 51.1 | pipelines/compile_ms 1785, pipelines/front_ms 1733, render/thread/ms 119, render/busy/ms 117, main/busy/ms 70.93, stage/render/prepare_assets 55.16, stage/main/update 54.63, pipelines/driver_ms 52.51, stream/drive/ms 36.84, stage/render/prepare_bind_groups 30.39, stage/render/render 21.89, render/graph/ms 20.02 |
| warm-up | 4.31 | 39.2 | pipelines/compile_ms 942, pipelines/front_ms 908, pipelines/wait_ms 87.63, render/thread/ms 71.57, render/busy/ms 70.29, stage/render/prepare_assets 44.95, pipelines/driver_ms 33.64, main/busy/ms 32.21, stage/main/update 17.74, stage/render/render 10.51, render/graph/ms 8.53, stage/main/post_update 6.74 |
| warm-up | 4.38 | 70.4 | pipelines/compile_ms 793, pipelines/front_ms 761, pipelines/wait_ms 184, render/thread/ms 102, render/busy/ms 101, main/busy/ms 39.22, stage/render/prepare_write_phase_buffers 34.20, pipelines/driver_ms 31.34, stage/render/prepare_assets 27.37, stage/main/update 18.83, stage/main/post_update 11.88, stage/render/prepare_resources 11.82 |
| warm-up | 4.42 | 40.4 | pipelines/compile_ms 868, pipelines/front_ms 835, pipelines/wait_ms 110, render/thread/ms 102, render/busy/ms 100, main/busy/ms 45.32, stage/render/prepare_write_phase_buffers 36.25, pipelines/driver_ms 33.03, stage/render/prepare_assets 26.42, stage/main/post_update 19.38, stage/main/update 18.86, stage/render/prepare_resources 11.91 |
| warm-up | 4.49 | 71.2 | pipelines/compile_ms 1431, pipelines/front_ms 1385, render/thread/ms 80.20, render/busy/ms 78.79, stage/render/prepare_assets 56.26, pipelines/driver_ms 45.63, main/busy/ms 44.60, stage/main/post_update 21.44, pipelines/wait_ms 18.97, stage/main/update 16.14, stage/render/render 9.59, render/graph/ms 7.77 |
| warm-up | 4.52 | 32.0 | pipelines/compile_ms 1924, pipelines/front_ms 1860, pipelines/wait_ms 120, render/thread/ms 73.85, render/busy/ms 71.90, pipelines/driver_ms 63.89, main/busy/ms 47.49, stage/render/prepare_assets 39.70, stage/main/update 22.14, stage/main/post_update 17.66, stage/render/render 15.16, render/graph/ms 12.68 |
| warm-up | 4.55 | 34.1 | pipelines/compile_ms 1119, pipelines/front_ms 1079, pipelines/wait_ms 102, render/thread/ms 53.60, render/busy/ms 51.33, main/busy/ms 41.88, pipelines/driver_ms 40.97, stage/main/update 22.54, stage/render/render 17.69, render/graph/ms 14.81, stage/render/prepare_assets 14.67, stage/main/post_update 12.32 |
| warm-up | 4.59 | 34.7 | pipelines/compile_ms 396, pipelines/front_ms 384, main/busy/ms 83.82, stage/main/update 69.40, stream/drive/ms 62.81, pipelines/wait_ms 58.16, render/thread/ms 50.55, render/busy/ms 48.84, stage/render/prepare_assets 26.31, pipelines/driver_ms 12.16, stage/render/render 11.70, render/graph/ms 9.42 |
| warm-up | 4.67 | 81.1 | pipelines/compile_ms 878, pipelines/front_ms 852, main/busy/ms 122, stage/main/update 109, stream/drive/ms 102, pipelines/wait_ms 99.25, render/thread/ms 54.77, render/busy/ms 52.82, pipelines/driver_ms 26.01, stage/render/prepare_assets 25.25, stage/render/render 17.42, render/graph/ms 14.49 |
| warm-up | 4.72 | 47.8 | pipelines/compile_ms 737, pipelines/front_ms 714, main/busy/ms 68.47, stage/main/update 55.29, render/thread/ms 48.92, render/busy/ms 47.06, stream/drive/ms 42.53, pipelines/wait_ms 41.46, pipelines/driver_ms 22.09, stage/render/render 17.99, stage/render/prepare_assets 15.23, render/graph/ms 14.96 |
| warm-up | 4.75 | 34.5 | pipelines/compile_ms 1428, pipelines/front_ms 1389, render/thread/ms 48.58, render/busy/ms 46.99, pipelines/driver_ms 39.71, main/busy/ms 35.61, stage/render/prepare_assets 16.72, stage/main/update 16.26, stage/render/render 12.41, stage/main/post_update 11.48, render/graph/ms 10.17, encode//sum 8.58 |
| warm-up | 4.82 | 53.1 | pipelines/compile_ms 820, pipelines/front_ms 800, main/busy/ms 59.08, stage/main/update 41.73, render/thread/ms 36.69, render/busy/ms 34.92, pipelines/driver_ms 20.23, stage/render/prepare_assets 17.74, stage/render/render 11.41, stream/drive/ms 9.76, stage/main/post_update 9.28, render/graph/ms 9.17 |
| warm-up | 4.86 | 39.9 | pipelines/compile_ms 372, pipelines/front_ms 364, render/thread/ms 51.65, render/busy/ms 50.08, main/busy/ms 44.18, stage/main/update 32.14, stage/render/prepare_assets 28.17, stage/render/render 11.01, stream/drive/ms 10.25, render/graph/ms 9.11, pipelines/driver_ms 7.92, encode//sum 7.70 |
| warm-up | 4.95 | 32.1 | pipelines/compile_ms 92.50, pipelines/front_ms 87.53, render/thread/ms 39.83, render/busy/ms 38.36, main/busy/ms 35.65, stage/main/update 22.36, stage/render/prepare_assets 14.23, stream/drive/ms 13.99, stage/render/render 12.82, render/graph/ms 10.76, stage/render/graph_submit 8.42, stage/main/post_update 7.77 |
| warm-up | 5.01 | 44.7 | pipelines/compile_ms 282, pipelines/front_ms 273, render/thread/ms 46.45, render/busy/ms 43.93, main/busy/ms 42.50, stage/render/prepare_assets 19.04, stage/main/update 18.78, stage/render/render 16.48, stage/main/post_update 14.80, render/graph/ms 12.59, pipelines/driver_ms 8.74, stage/render/graph_submit 8.69 |
| warm-up | 5.05 | 42.6 | pipelines/compile_ms 653, pipelines/front_ms 635, render/thread/ms 70.59, render/busy/ms 68.59, stage/render/prepare_assets 42.84, main/busy/ms 35.36, pipelines/driver_ms 17.67, stage/main/update 15.29, stage/render/render 15.19, render/graph/ms 12.56, stage/main/post_update 11.89, stage/render/graph_submit 8.99 |
| warm-up | 5.10 | 52.8 | pipelines/compile_ms 1124, pipelines/front_ms 1094, render/thread/ms 77.37, render/busy/ms 75.96, stage/render/prepare_assets 44.12, main/busy/ms 40.93, pipelines/driver_ms 29.52, stage/main/update 19.84, stage/main/post_update 12.73, stage/render/render 11.40, render/graph/ms 9.44, encode//sum 7.53 |
| warm-up | 5.15 | 46.9 | pipelines/compile_ms 1011, pipelines/front_ms 981, render/thread/ms 79.34, render/busy/ms 77.94, main/busy/ms 52.88, pipelines/driver_ms 30.11, stage/main/update 23.47, stage/render/prepare_assets 23.44, stage/render/prepare_flush 19.92, stage/main/post_update 12.67, stage/main/fixed_main_loop 11.21, stage/render/render 9.92 |
| warm-up | 5.18 | 29.8 | pipelines/compile_ms 623, pipelines/front_ms 603, render/thread/ms 65.43, render/busy/ms 63.73, main/busy/ms 53.50, stage/render/prepare_flush 23.74, pipelines/driver_ms 19.47, stage/main/post_update 16.94, stage/main/update 16.34, stage/main/fixed_main_loop 16.27, stage/render/prepare_assets 13.46, stage/render/render 9.25 |
| warm-up | 5.21 | 34.4 | pipelines/compile_ms 723, pipelines/front_ms 700, render/thread/ms 54.70, render/busy/ms 52.97, main/busy/ms 51.65, pipelines/wait_ms 26.27, pipelines/driver_ms 22.71, stage/main/update 22.21, stage/render/prepare_assets 22.15, stage/main/post_update 17.83, stream/drive/ms 12.15, stage/render/render 10.26 |
| warm-up | 5.25 | 40.7 | pipelines/compile_ms 1190, pipelines/front_ms 1151, render/thread/ms 64.23, render/busy/ms 62.69, pipelines/driver_ms 39.47, main/busy/ms 38.56, stage/render/prepare_assets 32.32, pipelines/wait_ms 27.68, stage/main/update 21.46, stream/drive/ms 13.79, stage/render/render 13.50, render/graph/ms 10.87 |
| warm-up | 5.28 | 27.4 | pipelines/compile_ms 1308, pipelines/front_ms 1270, render/thread/ms 61.28, render/busy/ms 58.70, pipelines/driver_ms 37.75, stage/render/prepare_assets 30.57, main/busy/ms 28.26, stage/render/render 16.37, render/graph/ms 12.57, stage/main/update 12.33, pipelines/wait_ms 11.49, stage/main/post_update 9.49 |
| warm-up | 5.31 | 27.0 | pipelines/compile_ms 672, pipelines/front_ms 656, render/thread/ms 44.18, render/busy/ms 41.66, main/busy/ms 31.20, stage/render/prepare_assets 20.32, pipelines/driver_ms 15.28, stage/render/render 13.65, stage/main/update 13.62, render/graph/ms 10.36, stage/main/post_update 9.94, pipelines/wait_ms 9.59 |
| warm-up | 5.35 | 36.0 | pipelines/compile_ms 592, pipelines/front_ms 579, render/thread/ms 47.12, render/busy/ms 45.83, main/busy/ms 35.38, stage/render/prepare_assets 29.63, pipelines/wait_ms 16.95, stage/main/update 14.88, pipelines/driver_ms 13.32, stage/main/post_update 13.00, stage/render/render 9.16, render/graph/ms 7.33 |
| warm-up | 5.38 | 33.0 | pipelines/compile_ms 396, pipelines/front_ms 387, render/thread/ms 56.84, render/busy/ms 55.82, stage/render/prepare_assets 40.03, main/busy/ms 28.82, pipelines/wait_ms 16.79, stage/main/update 12.11, stage/main/post_update 10.55, pipelines/driver_ms 9.08, stage/render/render 7.86, render/graph/ms 6.45 |
| warm-up | 5.43 | 29.0 | render/thread/ms 47.92, render/busy/ms 46.61, main/busy/ms 35.48, stage/render/prepare_assets 22.60, stage/main/update 16.44, stage/main/post_update 12.45, stage/render/render 10.78, render/graph/ms 8.99, encode//sum 6.88, stage/render/graph_submit 6.03, stream/drive/ms 4.52, stage/render/specialize 4.28 |
| warm-up | 5.48 | 48.3 | pipelines/compile_ms 482, pipelines/front_ms 457, render/thread/ms 71.42, render/busy/ms 70.13, main/busy/ms 41.58, stage/render/prepare_assets 36.24, pipelines/driver_ms 24.95, pipelines/wait_ms 19.91, stage/main/post_update 14.21, stage/main/update 13.67, stage/render/render 10.24, stage/render/prepare_resources 8.76 |
| warm-up | 5.52 | 35.8 | pipelines/compile_ms 675, pipelines/front_ms 642, render/thread/ms 76.58, render/busy/ms 75.11, main/busy/ms 47.53, pipelines/wait_ms 47.49, stage/render/prepare_assets 35.06, pipelines/driver_ms 32.34, stage/main/post_update 18.27, stage/render/render 13.61, stage/main/update 13.55, render/graph/ms 11.12 |
| warm-up | 5.55 | 36.1 | pipelines/compile_ms 193, pipelines/front_ms 186, render/thread/ms 63.07, render/busy/ms 61.71, main/busy/ms 37.26, stage/render/prepare_assets 33.43, pipelines/wait_ms 27.58, stage/main/update 14.20, stage/main/post_update 14.04, stage/render/render 12.60, render/graph/ms 10.26, encode//sum 7.84 |
| warm-up | 5.60 | 43.8 | pipelines/compile_ms 167, pipelines/front_ms 164, render/thread/ms 61.24, render/busy/ms 60.16, stage/render/prepare_assets 41.69, main/busy/ms 32.47, stage/main/update 14.35, stage/main/post_update 10.63, stage/render/render 8.13, render/graph/ms 6.62, encode//sum 6.49, stage/render/graph_submit 4.58 |
| warm-up | 5.71 | 31.2 | render/thread/ms 42.07, render/busy/ms 40.27, main/busy/ms 39.96, stage/main/update 19.89, stage/render/render 14.28, stage/main/post_update 13.80, stage/render/prepare_assets 13.18, render/graph/ms 11.91, encode//sum 9.96, stage/render/graph_submit 7.87, stream/drive/ms 5.99, encode/submit 5.14 |
| warm-up | 5.73 | 27.6 | render/thread/ms 44.63, render/busy/ms 42.73, main/busy/ms 36.02, stage/render/render 16.54, stage/main/post_update 15.81, stage/render/prepare_assets 13.92, render/graph/ms 13.82, stage/main/update 12.74, encode//sum 10.23, stage/render/graph_submit 9.93, encode/submit 6.79, stream/drive/ms 3.94 |
| warm-up | 5.76 | 30.7 | main/busy/ms 48.13, render/thread/ms 43.86, render/busy/ms 41.84, stage/main/update 27.91, stage/render/render 18.56, render/graph/ms 15.87, stream/drive/ms 14.93, stage/render/prepare_assets 14.25, stage/main/post_update 13.62, stage/render/graph_submit 11.70, encode//sum 10.07, encode/submit 7.41 |
| warm-up | 5.81 | 44.7 | pipelines/compile_ms 53.93, main/busy/ms 53.06, pipelines/front_ms 51.27, render/thread/ms 48.00, render/busy/ms 46.15, stage/main/update 33.29, stage/render/prepare_assets 21.52, stage/render/render 16.39, stream/drive/ms 15.18, render/graph/ms 14.16, stage/main/post_update 13.01, stage/render/graph_submit 10.32 |
| warm-up | 5.86 | 30.1 | render/thread/ms 40.92, render/busy/ms 39.67, main/busy/ms 31.59, stage/render/prepare_assets 17.20, pipelines/compile_ms 17.10, pipelines/front_ms 16.39, stage/main/update 14.35, stage/render/render 11.18, stage/main/post_update 10.55, render/graph/ms 9.36, stream/drive/ms 7.86, encode//sum 7.05 |
| warm-up | 5.99 | 25.6 | pipelines/compile_ms 139, pipelines/front_ms 133, main/busy/ms 55.48, stage/main/update 35.47, render/thread/ms 32.00, render/busy/ms 30.50, stream/drive/ms 21.82, stage/render/render 13.57, stage/main/post_update 13.26, render/graph/ms 11.64, stage/render/prepare_assets 9.05, stage/render/graph_submit 7.92 |
| warm-up | 6.03 | 41.7 | pipelines/compile_ms 434, pipelines/front_ms 420, main/busy/ms 49.98, render/thread/ms 35.78, render/busy/ms 34.46, stage/main/update 29.78, stream/drive/ms 21.64, pipelines/driver_ms 14.43, stage/main/post_update 14.02, stage/render/prepare_assets 12.97, stage/render/render 11.69, render/graph/ms 9.75 |
| warm-up | 6.12 | 31.1 | pipelines/compile_ms 367, pipelines/front_ms 355, main/busy/ms 35.93, render/thread/ms 32.75, render/busy/ms 31.24, stage/main/update 17.97, stage/render/prepare_assets 13.60, stage/render/render 12.01, stage/main/post_update 11.86, pipelines/driver_ms 11.69, render/graph/ms 9.96, encode//sum 8.81 |
| warm-up | 6.43 | 46.6 | main/busy/ms 44.23, render/thread/ms 44.17, render/busy/ms 42.71, stage/render/prepare_assets 22.92, stage/main/update 21.50, stage/main/post_update 16.44, stage/render/render 13.34, stream/drive/ms 11.93, render/graph/ms 11.32, stage/render/graph_submit 8.65, encode//sum 7.25, encode/submit 6.06 |
| warm-up | 6.77 | 30.0 | render/thread/ms 36.69, render/busy/ms 35.67, main/busy/ms 16.73, stage/render/render 10.74, stage/render/specialize 9.71, render/graph/ms 9.42, stage/main/post_update 7.56, stage/render/graph_submit 7.42, encode//sum 6.49, stage/render/queue 5.86, encode/submit 4.36, stage/main/update 4.17 |
| warm-up | 7.98 | 28.4 | render/thread/ms 35.40, render/busy/ms 32.39, main/busy/ms 13.84, stage/render/render 9.86, render/graph/ms 8.60, stage/render/specialize 8.31, stage/main/post_update 7.87, stage/render/graph_submit 6.80, encode//sum 5.88, stage/render/queue 5.72, encode/submit 4.14, stage/render/prepare_meshes 3.64 |
| measure | 0.84 | 26.4 | render/thread/ms 34.18, render/busy/ms 31.33, main/busy/ms 13.14, stage/render/render 10.54, render/graph/ms 9.16, stage/render/specialize 8.22, encode//sum 8.02, stage/main/post_update 7.64, stage/render/graph_submit 7.07, stage/render/queue 5.56, encode/submit 4.17, stage/main/update 3.02 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=base-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 1%
- during the run: other processes 3% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 94%) · 2625 MHz (min 2625) · memory 8101 MHz · 98.5 W · 79 °C (max 82) · VRAM 6445 MiB · throttled 100% of samples

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
