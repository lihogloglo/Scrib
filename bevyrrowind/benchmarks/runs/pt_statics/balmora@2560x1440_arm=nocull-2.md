# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1514 frames over 15.0 s · **median 9.90 ms (101 fps)** · mean 9.91 · p95 11.32 · p99 11.91 (1% low 84 fps) · worst 28.06 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 279 frames over 8.5 s, worst 2452.9 ms
- hitches (> 25.0 ms): **1** measured, 56 in warm-up
- timed GPU passes: 8.59 ms of a 9.91 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.90 ms, limit 11 ms
- pass worst frame 28.06 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.896 | 2.905 | 2.965 | 1.00 | 0.072 |
| dlss_super_resolution | 2.012 | 2.019 | 2.042 | 1.00 | 0.184 |
| early prepass | 0.603 | 0.605 | 0.614 | 1.00 | 0.091 |
| atmosphere_luts | 0.339 | 0.340 | 0.398 | 1.00 | 0.003 |
| bloom | 0.314 | 0.315 | 0.331 | 1.00 | 0.061 |
| main_transmissive_pass_3d | 0.309 | 0.310 | 0.331 | 1.00 | 0.009 |
| water_velocity | 0.238 | 0.239 | 0.263 | 1.00 | 0.002 |
| water_sspr | 0.215 | 0.216 | 0.214 | 1.00 | 0.002 |
| ssao | 0.188 | 0.189 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.148 | 0.149 | 0.169 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.147 | 0.155 | 1.00 | 0.001 |
| uniform_allocation | 0.142 | 0.143 | 0.193 | 1.00 | 0.024 |
| water_fft | 0.131 | 0.131 | 0.135 | 1.00 | 0.003 |
| upscaling | 0.113 | 0.114 | 0.120 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.102 | 0.102 | 0.134 | 1.00 | 0.017 |
| tonemapping | 0.086 | 0.086 | 0.089 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.075 | 1.00 | 0.021 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.412 | 0.414 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.039 | 0.040 | 0.053 | 1.00 | 0.005 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 1.00 | 0.001 |
| main_transparent_pass_3d | 0.028 | 0.028 | 0.031 | 1.00 | 0.136 |
| lightprobe_radiance_map | 0.027 | 0.220 | 0.231 | 0.12 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.013 | 0.042 | 0.045 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.024 | 1.00 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| surface_exposure | 0.012 | 0.041 | 0.043 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.797 | 10.249 | 26.62 | 13318.0 |
| render/busy/ms | 7.310 | 8.382 | 26.01 | 11067.8 |
| main/busy/ms | 7.100 | 7.939 | 12.20 | 10749.1 |
| stage/render/render | 4.869 | 5.723 | 8.73 | 7371.9 |
| render/graph/ms | 4.255 | 5.024 | 6.96 | 6442.0 |
| encode//sum | 4.188 | 5.476 | 9.71 | 6341.2 |
| stage/main/post_update | 4.133 | 4.880 | 8.97 | 6257.7 |
| stage/render/graph_submit | 3.188 | 3.917 | 5.61 | 4826.1 |
| encode/submit | 1.992 | 2.420 | 5.06 | 3016.2 |
| stage/main/update | 1.833 | 2.252 | 3.62 | 2775.6 |
| encode/finish_wall | 1.225 | 1.725 | 2.49 | 1854.2 |
| stage/render/prepare_views | 1.167 | 2.712 | 3.91 | 1766.7 |
| render/acquire/ms | 1.123 | 2.671 | 3.86 | 1700.9 |
| stage/render/camera_driver | 1.011 | 1.143 | 1.67 | 1530.9 |
| encode/pass:shadow_directional_light/sum | 0.785 | 1.352 | 2.14 | 1189.1 |
| encode/pass:early prepass/sum | 0.734 | 1.013 | 1.83 | 1111.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.725 | 1.001 | 1.47 | 1097.2 |
| stage/render/prepare_write_phase_buffers | 0.585 | 0.829 | 1.11 | 885.0 |
| stage/render/prepare_assets | 0.459 | 0.591 | 1.56 | 695.6 |
| stage/render/prepare_resources | 0.388 | 0.449 | 0.85 | 587.9 |
| bigspace/total/ms | 0.378 | 0.946 | 2.05 | 572.4 |
| render/present/ms | 0.363 | 0.434 | 1.50 | 549.2 |
| stage/main/pre_update | 0.350 | 0.479 | 0.82 | 530.1 |
| encode/pass:main_transparent_pass_3d/sum | 0.321 | 0.460 | 0.88 | 486.0 |
| stage/main/first | 0.289 | 0.354 | 1.34 | 438.0 |
| terrain/update/ms | 0.235 | 0.303 | 0.62 | 355.4 |
| stage/render/prepare_meshes | 0.230 | 0.286 | 1.31 | 348.4 |
| stage/render/prepare_batch_phases | 0.230 | 0.297 | 0.45 | 347.7 |
| stage/render/prepare_bind_groups | 0.216 | 0.305 | 0.46 | 327.0 |
| terrain/select/ms | 0.205 | 0.265 | 0.59 | 310.3 |
| stage/main/fixed_main_loop | 0.193 | 0.365 | 0.90 | 291.9 |
| stream/drive/ms | 0.192 | 0.262 | 0.64 | 291.1 |
| stage/render/extract_commands | 0.170 | 0.200 | 0.27 | 258.0 |
| stage/render/create_views | 0.169 | 0.215 | 0.64 | 256.5 |
| far/select/ms | 0.169 | 0.230 | 0.61 | 255.2 |
| stream/tick/ms | 0.162 | 0.225 | 0.60 | 245.6 |
| controller/feed/ms | 0.158 | 0.206 | 0.65 | 238.6 |
| stage/render/prepare_flush | 0.157 | 0.295 | 0.78 | 237.8 |
| bigspace/high_precision/ms | 0.136 | 0.399 | 1.75 | 205.8 |
| controller/move/ms | 0.131 | 0.186 | 0.66 | 198.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.123 | 0.188 | 0.36 | 186.2 |
| stage/main/last | 0.119 | 0.170 | 0.72 | 179.7 |
| stage/render/cleanup | 0.113 | 0.172 | 0.36 | 171.2 |
| stage/render/queue | 0.110 | 0.139 | 6.34 | 167.0 |
| bigspace/low_precision/ms | 0.098 | 0.232 | 1.22 | 148.6 |
| stage/render/specialize | 0.095 | 0.121 | 8.74 | 143.1 |
| anim/evaluate/ms | 0.083 | 0.186 | 1.12 | 125.4 |
| anim/evaluate_cpu/ms | 0.060 | 0.075 | 0.21 | 90.9 |
| stage/render/prepare_collect_phase_buffers | 0.060 | 0.118 | 0.18 | 90.5 |
| encode/resolve_diagnostics/sum | 0.055 | 0.079 | 0.45 | 83.9 |
| physics/step/ms | 0.029 | 0.058 | 0.10 | 44.3 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.039 | 0.35 | 41.0 |
| stage/render/phase_sort | 0.026 | 0.043 | 0.11 | 39.0 |
| weather/map/ms | 0.025 | 0.033 | 0.42 | 37.6 |
| encode/camera_driver_span_end/sum | 0.024 | 0.037 | 0.30 | 36.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.10 | 16.6 |
| script/update/ms | 0.008 | 0.010 | 0.45 | 11.6 |
| controller/avoid/ms | 0.006 | 0.007 | 0.44 | 9.1 |
| script/input/ms | 0.006 | 0.007 | 0.03 | 8.4 |
| anim/apply/ms | 0.005 | 0.006 | 0.40 | 7.1 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.0 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 0.8 |
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
| crowd/c1/count | count | 122 | 123 | 123 |
| crowd/c1/joints/count | count | 4848 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1080 | 1088 | 1088 |
| crowd/c2/count | count | 284 | 284 | 283 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.56 | 1.89 | 0.47 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.35 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.30 | 0.03 |
| encode/finish_critical | ms | 0.88 | 2.14 | 0.73 |
| encode/pass:early prepass/max | ms | 0.72 | 1.83 | 0.61 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.73 | 1.47 | 0.69 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.36 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.31 | 0.88 | 0.22 |
| encode/pass:shadow_directional_light/max | ms | 0.72 | 2.14 | 0.73 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.45 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.25 | 26.01 | 9.34 |
| gpu/timed/ms | ms | 9.25 | 10.58 | 9.34 |
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
| process/cpu_usage | % | 27.88 | 40.38 | 23.77 |
| process/mem_usage | GiB | 4.74 | 4.78 | 4.75 |
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
| skin/blocks_changed/count | count | 6 | 7 | 5 |
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
| system/cpu_usage | % | 33.16 | 42.68 | 29.73 |
| system/mem_usage | % | 54.34 | 54.47 | 54.41 |
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
| anim/events/count | count | 0.11 | 3 | 167 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.27 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 29 | 40.2 k |
| encode/bind_walk/init_textures | count | 7178 | 14.0 k | 10.87 M |
| encode/bind_walk/init_textures_skipped | count | 32.3 k | 46.9 k | 48.98 M |
| encode/bind_walk/inits | count | 422 | 515 | 638.7 k |
| encode/bind_walk/inits_skipped | count | 93.49 | 107 | 141.6 k |
| encode/bind_walk/merge_textures | count | 7115 | 13.9 k | 10.77 M |
| encode/bind_walk/merge_textures_skipped | count | 4225 | 4225 | 6.40 M |
| encode/bind_walk/merges | count | 84.25 | 105 | 127.6 k |
| encode/bind_walk/merges_skipped | count | 79.00 | 85 | 119.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1514 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1514 |
| encode/pass:early prepass/count | count | 1 | 1 | 1514 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1514 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1514 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1514 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1514 |
| encode/queue/command_buffers | count | 36.57 | 39 | 55.4 k |
| encode/queue/submits | count | 2 | 2 | 3028 |
| encode/queue/write_buffer/bytes | bytes | 1.85 M | 4.77 M | 2.79 G |
| encode/queue/write_buffer/calls | count | 233 | 306 | 352.8 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 49.42 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.6 k |
| encode/queue/write_texture/bytes | bytes | 11.9 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1514 |
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

## Hitches (> 25.0 ms; slowest 57, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.45 | 2452.9 | main/busy/ms 2424, stage/main/post_update 2319, stage/main/update 96.08, surface/advance_cpu_ms 66.84, stage/main/pre_update 5.15, stage/main/last 0.98, surface/cell_upload_ms 0.76, stage/main/first 0.50, stream/drive/ms 0.40, stage/main/fixed_main_loop 0.37, weather/map/ms 0.13, stream/tick/ms 0.13 |
| warm-up | 3.17 | 716.6 | main/busy/ms 2557, stage/main/post_update 2343, render/thread/ms 685, render/busy/ms 523, stage/render/prepare_assets 477, stage/main/update 199, stage/render/prepare_views 159, render/acquire/ms 159, surface/advance_cpu_ms 66.84, stage/render/create_views 27.44, stage/render/render 11.10, stage/main/pre_update 7.13 |
| warm-up | 3.27 | 103.7 | render/thread/ms 773, render/busy/ms 562, stage/render/prepare_assets 504, stage/render/prepare_views 160, render/acquire/ms 160, main/busy/ms 151, stage/main/update 107, stage/render/render 63.95, render/present/ms 51.07, pipelines/compile_ms 38.48, pipelines/front_ms 34.28, stage/main/post_update 33.83 |
| warm-up | 3.30 | 29.2 | pipelines/compile_ms 246, pipelines/front_ms 219, render/thread/ms 106, stage/render/render 65.72, main/busy/ms 62.83, render/busy/ms 53.55, render/present/ms 51.97, stage/main/post_update 48.63, stage/render/prepare_assets 28.04, pipelines/driver_ms 26.89, terrain/update/ms 23.57, pipelines/wait_ms 16.81 |
| warm-up | 3.45 | 145.5 | pipelines/wait_ms 4649, pipelines/compile_ms 1591, pipelines/front_ms 1481, render/thread/ms 136, render/busy/ms 131, pipelines/driver_ms 110, main/busy/ms 88.68, stage/main/post_update 64.51, stage/render/prepare_resources 40.96, stage/render/render 32.45, terrain/update/ms 30.20, render/graph/ms 27.01 |
| warm-up | 3.50 | 53.8 | pipelines/wait_ms 4705, pipelines/compile_ms 1567, pipelines/front_ms 1470, render/thread/ms 154, render/busy/ms 152, main/busy/ms 120, pipelines/driver_ms 97.37, stage/main/update 56.09, stage/render/prepare_resources 41.61, stage/main/post_update 36.59, stage/render/render 31.84, terrain/update/ms 31.80 |
| warm-up | 3.62 | 117.0 | pipelines/compile_ms 1277, pipelines/front_ms 1225, render/thread/ms 109, render/busy/ms 107, pipelines/wait_ms 105, main/busy/ms 93.13, stage/render/prepare_assets 76.67, stage/main/update 59.67, pipelines/driver_ms 52.04, stage/render/render 22.41, render/graph/ms 19.91, stage/main/post_update 18.71 |
| warm-up | 3.67 | 48.1 | pipelines/compile_ms 2272, pipelines/front_ms 2110, pipelines/wait_ms 243, pipelines/driver_ms 162, render/thread/ms 119, render/busy/ms 116, stage/render/prepare_assets 83.26, main/busy/ms 32.74, stage/render/render 27.62, render/graph/ms 24.48, stage/render/graph_submit 18.10, stage/main/post_update 17.29 |
| warm-up | 3.78 | 78.8 | pipelines/compile_ms 997, pipelines/front_ms 792, encode//sum 427, pipelines/driver_ms 206, render/thread/ms 94.04, render/busy/ms 91.64, stage/render/render 76.09, render/graph/ms 73.39, stage/render/graph_submit 68.65, encode/finish_wall 64.33, encode/pass:early prepass/sum 60.68, encode/pass:main_opaque_pass_3d/sum 60.14 |
| warm-up | 3.84 | 46.9 | pipelines/compile_ms 826, pipelines/front_ms 792, render/thread/ms 60.80, render/busy/ms 59.08, main/busy/ms 56.24, stage/render/prepare_assets 44.01, stage/main/update 43.83, stream/drive/ms 40.63, pipelines/driver_ms 33.23, pipelines/wait_ms 14.75, stage/render/render 10.34, render/graph/ms 8.55 |
| warm-up | 3.89 | 41.8 | pipelines/compile_ms 722, pipelines/front_ms 692, render/thread/ms 74.03, render/busy/ms 72.37, main/busy/ms 58.71, stage/render/prepare_assets 57.15, stage/main/update 45.43, stream/drive/ms 40.86, pipelines/driver_ms 30.52, stage/render/render 9.61, render/graph/ms 7.91, pipelines/wait_ms 7.21 |
| warm-up | 3.94 | 46.1 | render/thread/ms 37.69, render/busy/ms 36.34, main/busy/ms 35.70, stage/main/update 23.54, stream/drive/ms 19.62, stage/render/prepare_assets 16.75, stage/render/render 11.54, render/graph/ms 9.42, encode//sum 7.11, stage/main/post_update 6.74, stage/render/graph_submit 6.53, encode/submit 4.27 |
| warm-up | 3.98 | 33.8 | render/thread/ms 52.33, render/busy/ms 50.65, main/busy/ms 43.77, stage/render/prepare_assets 29.29, stage/main/update 26.60, stream/drive/ms 22.11, stage/render/render 12.37, render/graph/ms 9.79, stage/main/post_update 9.01, encode//sum 6.97, stage/render/graph_submit 6.42, encode/submit 4.34 |
| warm-up | 4.07 | 95.7 | pipelines/compile_ms 1679, pipelines/front_ms 1633, render/thread/ms 76.74, render/busy/ms 75.18, stage/render/prepare_assets 55.14, main/busy/ms 51.69, pipelines/driver_ms 46.02, stage/main/update 31.05, stream/drive/ms 21.73, stage/render/render 13.10, stage/main/post_update 11.45, render/graph/ms 10.88 |
| warm-up | 4.13 | 55.9 | pipelines/compile_ms 1774, pipelines/front_ms 1724, main/busy/ms 99.44, render/thread/ms 89.39, render/busy/ms 87.76, stage/main/update 81.20, stage/render/prepare_assets 65.14, stream/drive/ms 54.58, pipelines/driver_ms 50.36, stage/render/render 12.15, render/graph/ms 9.95, stage/main/post_update 9.64 |
| warm-up | 4.22 | 94.8 | main/busy/ms 107, pipelines/compile_ms 106, pipelines/front_ms 101, stage/main/update 90.70, render/thread/ms 71.74, render/busy/ms 69.99, stream/drive/ms 54.76, stage/render/prepare_assets 50.48, stage/render/render 10.16, stage/main/post_update 8.41, render/graph/ms 7.88, stage/render/graph_submit 5.50 |
| warm-up | 4.28 | 58.3 | pipelines/compile_ms 951, pipelines/front_ms 917, render/thread/ms 82.42, main/busy/ms 79.78, render/busy/ms 77.35, stage/render/prepare_assets 39.40, stage/main/update 35.08, stage/main/post_update 34.68, pipelines/driver_ms 33.47, stage/render/render 28.47, render/graph/ms 22.77, stage/render/graph_submit 12.80 |
| warm-up | 4.33 | 53.1 | pipelines/compile_ms 1553, pipelines/front_ms 1501, pipelines/wait_ms 111, render/thread/ms 90.52, main/busy/ms 88.71, render/busy/ms 85.40, pipelines/driver_ms 52.02, stage/main/post_update 50.43, stage/render/render 29.44, stage/main/update 23.76, render/graph/ms 23.61, stage/render/prepare_assets 20.14 |
| warm-up | 4.44 | 103.2 | pipelines/compile_ms 1146, pipelines/front_ms 1112, pipelines/wait_ms 270, render/thread/ms 130, render/busy/ms 129, main/busy/ms 105, stage/render/prepare_assets 62.69, stage/main/post_update 55.64, stage/main/update 37.71, pipelines/driver_ms 34.89, stage/render/prepare_resources 25.70, stage/render/prepare_write_phase_buffers 18.61 |
| warm-up | 4.49 | 53.9 | pipelines/compile_ms 2197, pipelines/front_ms 2138, pipelines/wait_ms 269, render/thread/ms 124, render/busy/ms 123, main/busy/ms 89.34, stage/render/prepare_assets 77.85, pipelines/driver_ms 59.78, stage/main/update 41.55, stage/main/post_update 39.10, stage/render/prepare_resources 14.70, stage/render/render 9.16 |
| warm-up | 4.53 | 40.7 | pipelines/compile_ms 2077, pipelines/front_ms 2019, pipelines/wait_ms 168, render/thread/ms 69.57, render/busy/ms 68.06, pipelines/driver_ms 57.78, main/busy/ms 42.32, stage/render/prepare_assets 38.34, stage/main/update 20.34, stage/main/post_update 13.29, stage/render/render 10.31, render/graph/ms 8.34 |
| warm-up | 4.57 | 39.1 | pipelines/compile_ms 627, pipelines/front_ms 608, pipelines/wait_ms 81.10, render/thread/ms 67.16, render/busy/ms 65.68, main/busy/ms 45.46, stage/render/prepare_assets 33.41, stage/main/post_update 24.59, pipelines/driver_ms 18.58, stage/main/update 14.28, stage/render/render 11.35, render/graph/ms 9.38 |
| warm-up | 4.61 | 41.7 | pipelines/compile_ms 215, pipelines/front_ms 210, main/busy/ms 121, stage/main/update 96.23, stream/drive/ms 90.50, render/thread/ms 55.08, render/busy/ms 53.55, stage/render/prepare_assets 23.39, pipelines/wait_ms 18.84, stage/main/post_update 18.69, stage/render/render 15.01, stage/render/cleanup 14.92 |
| warm-up | 4.71 | 100.4 | pipelines/compile_ms 1627, pipelines/front_ms 1587, main/busy/ms 113, stage/main/update 98.82, stream/drive/ms 91.65, render/thread/ms 48.68, render/busy/ms 47.15, pipelines/driver_ms 39.69, pipelines/wait_ms 30.16, stage/render/prepare_assets 22.40, stage/render/cleanup 14.90, stage/render/render 14.80 |
| warm-up | 4.82 | 65.7 | pipelines/compile_ms 830, pipelines/front_ms 810, main/busy/ms 64.41, render/thread/ms 61.50, render/busy/ms 59.86, stage/main/update 46.67, pipelines/wait_ms 28.82, stage/render/prepare_assets 21.87, pipelines/driver_ms 19.80, stage/render/prepare_meshes 14.95, stage/render/render 13.25, stream/drive/ms 11.73 |
| warm-up | 4.86 | 43.1 | pipelines/compile_ms 807, pipelines/front_ms 788, render/thread/ms 73.84, render/busy/ms 72.52, main/busy/ms 50.46, stage/main/update 36.27, stage/render/prepare_assets 31.51, pipelines/wait_ms 28.36, pipelines/driver_ms 18.86, stage/render/prepare_meshes 16.64, stream/drive/ms 12.77, stage/render/render 9.74 |
| warm-up | 4.92 | 37.4 | pipelines/compile_ms 462, pipelines/front_ms 443, render/thread/ms 54.22, render/busy/ms 52.70, main/busy/ms 44.55, stage/main/update 23.14, pipelines/driver_ms 19.16, stage/main/post_update 15.76, stage/render/render 12.72, render/graph/ms 10.72, stage/render/graph_submit 8.17, encode//sum 7.79 |
| warm-up | 4.95 | 29.0 | pipelines/compile_ms 700, pipelines/front_ms 673, render/thread/ms 57.05, main/busy/ms 56.19, render/busy/ms 55.45, stage/main/update 32.25, pipelines/driver_ms 27.13, stream/drive/ms 17.20, stage/main/post_update 13.31, stage/render/render 13.19, stage/render/prepare_assets 12.98, render/graph/ms 11.11 |
| warm-up | 5.01 | 57.2 | pipelines/compile_ms 631, pipelines/front_ms 610, render/thread/ms 71.57, render/busy/ms 69.58, main/busy/ms 45.15, stage/render/prepare_assets 31.54, stage/main/update 24.94, pipelines/driver_ms 20.66, stream/drive/ms 17.53, stage/render/render 12.12, stage/main/post_update 10.30, render/graph/ms 9.47 |
| warm-up | 5.04 | 31.4 | pipelines/compile_ms 789, pipelines/front_ms 764, render/thread/ms 74.68, render/busy/ms 72.86, main/busy/ms 38.37, stage/render/prepare_assets 30.76, pipelines/driver_ms 24.92, stage/main/update 18.63, stage/main/post_update 13.87, stage/render/render 12.32, render/graph/ms 9.85, encode//sum 7.72 |
| warm-up | 5.09 | 45.0 | pipelines/compile_ms 971, pipelines/front_ms 939, render/thread/ms 60.65, render/busy/ms 59.35, main/busy/ms 48.89, pipelines/driver_ms 32.12, stage/render/prepare_assets 23.25, stage/main/update 22.61, stage/main/post_update 19.07, stage/render/render 12.69, render/graph/ms 10.65, encode//sum 8.26 |
| warm-up | 5.12 | 30.9 | pipelines/compile_ms 977, pipelines/front_ms 950, render/thread/ms 61.56, render/busy/ms 60.03, main/busy/ms 44.56, pipelines/driver_ms 26.50, stage/render/prepare_assets 23.16, stage/main/post_update 20.07, stage/main/update 17.41, stage/render/render 13.89, render/graph/ms 11.70, encode//sum 8.26 |
| warm-up | 5.15 | 36.9 | pipelines/compile_ms 1199, pipelines/front_ms 1172, pipelines/wait_ms 87.62, render/thread/ms 58.09, render/busy/ms 56.49, main/busy/ms 50.58, stage/main/update 30.12, pipelines/driver_ms 27.39, stage/render/prepare_assets 15.45, stage/render/render 14.90, stage/main/post_update 14.23, render/graph/ms 12.77 |
| warm-up | 5.20 | 44.8 | pipelines/compile_ms 1238, pipelines/front_ms 1209, pipelines/wait_ms 81.66, render/thread/ms 59.32, render/busy/ms 57.99, main/busy/ms 44.53, pipelines/driver_ms 29.08, stage/render/prepare_assets 29.07, stage/main/update 27.72, stage/render/render 11.03, stage/main/post_update 10.39, render/graph/ms 9.09 |
| warm-up | 5.29 | 25.1 | pipelines/compile_ms 244, pipelines/front_ms 236, render/thread/ms 39.57, render/busy/ms 38.24, main/busy/ms 33.79, stage/main/update 20.08, stage/render/prepare_assets 15.26, stage/render/render 12.04, stream/drive/ms 11.91, render/graph/ms 10.29, stage/main/post_update 9.17, encode//sum 7.93 |
| warm-up | 5.39 | 28.0 | pipelines/compile_ms 181, pipelines/front_ms 176, render/thread/ms 43.18, render/busy/ms 42.02, main/busy/ms 29.84, stage/render/prepare_assets 15.34, stage/main/update 13.80, stage/render/render 11.63, stage/main/post_update 10.06, render/graph/ms 9.79, encode//sum 8.52, stage/render/graph_submit 7.14 |
| warm-up | 5.41 | 28.9 | pipelines/compile_ms 181, pipelines/front_ms 176, render/thread/ms 51.32, render/busy/ms 50.14, main/busy/ms 35.24, stage/render/prepare_assets 27.44, stage/main/update 16.81, stage/main/post_update 12.35, stage/render/render 10.85, encode//sum 8.93, render/graph/ms 8.89, stage/render/graph_submit 6.17 |
| warm-up | 5.45 | 33.6 | pipelines/compile_ms 543, pipelines/front_ms 529, render/thread/ms 56.86, render/busy/ms 55.66, stage/render/prepare_assets 34.55, main/busy/ms 31.10, pipelines/driver_ms 14.33, stage/main/update 14.01, stage/main/post_update 11.45, stage/render/render 8.57, render/graph/ms 6.88, encode//sum 6.69 |
| warm-up | 5.51 | 38.5 | pipelines/compile_ms 166, pipelines/front_ms 161, render/thread/ms 55.87, render/busy/ms 54.09, main/busy/ms 24.83, stage/render/prepare_assets 22.20, stage/render/render 13.48, render/graph/ms 11.13, stage/main/update 10.90, encode//sum 8.60, stage/main/post_update 8.24, stage/render/graph_submit 8.16 |
| warm-up | 5.58 | 48.8 | pipelines/compile_ms 342, pipelines/front_ms 332, render/thread/ms 56.59, render/busy/ms 55.52, main/busy/ms 32.43, stage/render/prepare_assets 31.94, stage/main/update 13.54, stage/main/post_update 11.81, pipelines/driver_ms 10.21, stage/render/render 8.95, render/graph/ms 7.55, encode//sum 6.91 |
| warm-up | 5.64 | 40.3 | pipelines/compile_ms 152, pipelines/front_ms 149, render/thread/ms 53.55, render/busy/ms 52.51, stage/render/prepare_assets 27.77, main/busy/ms 26.24, stage/main/update 10.92, stage/main/post_update 10.23, stage/render/render 8.39, encode//sum 6.82, render/graph/ms 6.79, stage/render/specialize 5.13 |
| warm-up | 5.67 | 30.1 | render/thread/ms 61.53, render/busy/ms 59.72, main/busy/ms 34.17, stage/render/prepare_assets 22.61, stage/main/update 17.91, stage/render/render 11.75, stage/main/post_update 11.57, stage/render/specialize 10.74, render/graph/ms 9.35, encode//sum 7.41, stage/render/graph_submit 7.28, stream/drive/ms 5.31 |
| warm-up | 5.70 | 30.9 | pipelines/compile_ms 153, pipelines/front_ms 145, render/thread/ms 49.19, render/busy/ms 47.40, main/busy/ms 34.92, stage/main/update 20.30, stage/render/prepare_assets 15.07, stage/render/render 11.09, stage/main/post_update 9.42, stage/render/specialize 8.87, render/graph/ms 8.78, encode//sum 7.22 |
| warm-up | 5.73 | 31.1 | pipelines/compile_ms 153, pipelines/front_ms 145, render/thread/ms 47.77, render/busy/ms 46.50, main/busy/ms 34.53, stage/main/update 15.13, stage/render/prepare_assets 14.82, stage/render/render 12.47, stage/main/post_update 12.39, render/graph/ms 10.68, encode//sum 9.12, pipelines/driver_ms 7.15 |
| warm-up | 5.76 | 27.2 | main/busy/ms 72.41, render/thread/ms 50.27, stage/main/update 49.94, render/busy/ms 48.62, stage/render/render 18.89, render/graph/ms 16.83, stream/drive/ms 15.76, stage/main/post_update 14.70, stage/render/graph_submit 12.86, encode//sum 10.36, stage/render/prepare_assets 8.46, pipelines/compile_ms 7.36 |
| warm-up | 5.84 | 82.4 | pipelines/compile_ms 74.14, main/busy/ms 73.64, pipelines/front_ms 68.42, render/thread/ms 57.45, render/busy/ms 55.83, stage/main/update 49.37, stage/render/prepare_assets 22.66, stage/main/post_update 17.71, stage/render/render 16.14, stream/drive/ms 15.64, render/graph/ms 14.04, stage/render/graph_submit 10.76 |
| warm-up | 5.87 | 31.2 | pipelines/compile_ms 1032, pipelines/front_ms 994, render/thread/ms 58.87, render/busy/ms 57.44, main/busy/ms 45.92, pipelines/driver_ms 37.44, stage/render/prepare_assets 27.18, stage/main/update 21.16, stage/main/post_update 16.51, stage/render/render 10.80, stream/drive/ms 9.11, render/graph/ms 8.74 |
| warm-up | 5.90 | 29.9 | pipelines/compile_ms 965, pipelines/front_ms 933, render/thread/ms 47.84, render/busy/ms 46.50, main/busy/ms 42.12, pipelines/driver_ms 32.04, stage/main/update 23.07, stage/render/prepare_assets 21.27, stage/main/post_update 11.14, stage/render/render 9.55, stream/drive/ms 8.72, render/graph/ms 7.59 |
| warm-up | 6.04 | 27.1 | pipelines/compile_ms 213, pipelines/front_ms 206, main/busy/ms 52.24, render/thread/ms 39.38, render/busy/ms 37.89, stage/main/update 32.41, stream/drive/ms 21.62, stage/render/render 15.67, stage/main/post_update 14.35, render/graph/ms 13.87, stage/render/graph_submit 10.60, encode//sum 8.05 |
| warm-up | 6.08 | 37.8 | pipelines/compile_ms 403, pipelines/front_ms 388, main/busy/ms 52.31, render/thread/ms 40.96, render/busy/ms 39.53, stage/main/update 32.92, stream/drive/ms 22.06, pipelines/driver_ms 14.73, stage/render/render 13.17, stage/main/post_update 12.29, render/graph/ms 11.34, stage/render/prepare_assets 10.15 |
| warm-up | 6.22 | 38.9 | pipelines/compile_ms 1062, pipelines/front_ms 1040, render/thread/ms 37.50, render/busy/ms 36.33, main/busy/ms 36.32, pipelines/driver_ms 22.04, stage/main/update 19.38, stage/main/post_update 11.40, stage/render/render 11.33, stage/render/prepare_assets 10.05, render/graph/ms 9.57, encode//sum 8.87 |
| warm-up | 6.45 | 36.9 | render/thread/ms 47.43, render/busy/ms 45.96, main/busy/ms 30.72, stage/render/render 15.38, stage/main/update 14.29, render/graph/ms 13.07, stage/main/post_update 12.59, encode//sum 10.53, stage/render/graph_submit 9.77, stage/render/specialize 7.08, stage/render/prepare_assets 6.86, encode/submit 6.58 |
| warm-up | 6.47 | 25.3 | render/thread/ms 51.64, render/busy/ms 50.21, main/busy/ms 38.07, stage/main/update 18.25, stage/render/render 15.14, stage/render/prepare_assets 13.85, stage/main/post_update 12.83, render/graph/ms 12.80, encode//sum 10.87, stream/drive/ms 9.68, stage/render/graph_submit 9.20, stage/render/specialize 6.58 |
| warm-up | 6.53 | 52.7 | render/thread/ms 54.91, render/busy/ms 53.93, main/busy/ms 41.38, stage/render/prepare_assets 32.78, stage/main/update 17.72, stage/main/post_update 15.79, stream/drive/ms 9.77, stage/render/render 8.08, render/graph/ms 6.66, encode//sum 6.49, stage/render/graph_submit 4.12, stage/render/specialize 3.86 |
| warm-up | 6.86 | 30.1 | render/thread/ms 37.89, render/busy/ms 36.96, main/busy/ms 17.23, stage/main/post_update 10.24, stage/render/render 10.05, stage/render/specialize 8.73, render/graph/ms 8.72, encode//sum 7.76, stage/render/graph_submit 6.27, stage/render/queue 6.03, stage/main/update 4.44, encode/finish_wall 3.40 |
| warm-up | 8.09 | 26.8 | render/thread/ms 33.90, render/busy/ms 31.61, main/busy/ms 13.98, stage/render/render 10.25, render/graph/ms 9.01, stage/render/specialize 8.57, stage/main/post_update 7.68, stage/render/graph_submit 6.97, stage/render/queue 6.60, encode//sum 6.58, stage/main/update 4.13, encode/submit 3.95 |
| measure | 0.85 | 28.1 | render/thread/ms 35.72, render/busy/ms 33.71, main/busy/ms 13.27, stage/render/render 12.46, encode//sum 12.43, render/graph/ms 11.01, stage/render/graph_submit 8.90, stage/render/specialize 8.83, stage/main/post_update 7.49, stage/render/queue 6.43, encode/submit 4.92, encode/finish_wall 4.06 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=nocull-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 1%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **97%** (min 85%) · 2625 MHz (min 2625) · memory 8101 MHz · 97.7 W · 79 °C (max 82) · VRAM 6447 MiB · throttled 100% of samples

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
