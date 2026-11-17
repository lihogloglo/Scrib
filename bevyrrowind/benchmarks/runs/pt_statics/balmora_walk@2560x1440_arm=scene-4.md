# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6085 frames over 65.6 s · **median 10.63 ms (94 fps)** · mean 10.79 · p95 13.22 · p99 17.18 (1% low 58 fps) · worst 34.20 ms · 85 spikes over 1.5× median · 61.2 stutters/min (work 55.8/min)
- warm-up: 352 frames over 9.3 s, worst 2555.1 ms
- hitches (> 26.6 ms): **2** measured, 36 in warm-up
- timed GPU passes: 6.64 ms of a 10.79 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.63 ms, limit 11 ms
- **FAIL** frame_ms: 67 stutters (> 1.5x moving median of 60) in 1.09 min = 61.24/min, limit 1/min
- **FAIL** worst frame 34.20 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.839 | 2.037 | 2.070 | 0.90 | 0.173 |
| main_opaque_pass_3d | 1.437 | 1.591 | 1.951 | 0.90 | 0.074 |
| early prepass | 0.471 | 0.522 | 0.748 | 0.90 | 0.091 |
| bloom | 0.284 | 0.315 | 0.321 | 0.90 | 0.062 |
| atmosphere_luts | 0.276 | 0.305 | 0.367 | 0.90 | 0.003 |
| water_velocity | 0.253 | 0.280 | 0.347 | 0.90 | 0.002 |
| water_sspr | 0.182 | 0.202 | 0.204 | 0.90 | 0.002 |
| ssao | 0.171 | 0.189 | 0.194 | 0.90 | 0.007 |
| auxiliary_views | 0.150 | 0.167 | 0.573 | 0.90 | 0.167 |
| atmos_froxel_inject | 0.133 | 0.147 | 0.150 | 0.90 | 0.001 |
| clouds_trace | 0.126 | 0.140 | 0.236 | 0.90 | 0.001 |
| water_fft | 0.118 | 0.130 | 0.137 | 0.90 | 0.003 |
| uniform_allocation | 0.109 | 0.121 | 0.170 | 0.90 | 0.025 |
| upscaling | 0.105 | 0.116 | 0.124 | 0.90 | 0.004 |
| clouds_composite | 0.095 | 0.106 | 0.109 | 0.90 | 0.002 |
| bin_unpacking | 0.087 | 0.096 | 0.133 | 0.90 | 0.019 |
| gpu_instance_materialization | 0.086 | 0.095 | 0.099 | 0.90 | 0.004 |
| main_transmissive_pass_3d | 0.078 | 0.086 | 0.180 | 0.90 | 0.009 |
| tonemapping | 0.077 | 0.085 | 0.089 | 0.90 | 0.004 |
| clustering | 0.058 | 0.064 | 0.067 | 0.90 | 0.018 |
| early_mesh_preprocessing | 0.052 | 0.057 | 0.091 | 0.90 | 0.004 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.90 | 0.002 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.90 | 0.001 |
| lightprobe_irradiance_map | 0.043 | 0.414 | 0.417 | 0.10 | 0.001 |
| surface_trail | 0.036 | 0.056 | 0.057 | 0.64 | 0.002 |
| surface_exposure | 0.036 | 0.039 | 0.040 | 0.90 | 0.001 |
| surface_exposure_far | 0.032 | 0.040 | 0.041 | 0.81 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.90 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.032 | 0.90 | 0.001 |
| water_snapshot | 0.026 | 0.029 | 0.033 | 0.90 | 0.002 |
| lightprobe_radiance_map | 0.023 | 0.219 | 0.230 | 0.10 | 0.004 |
| range_unpacking | 0.021 | 0.024 | 0.032 | 0.90 | 0.003 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.90 | 0.002 |
| per_view_shadow_pass | 0.016 | 0.018 | 0.037 | 0.90 | 0.006 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.90 | 0.002 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.90 | 0.002 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.90 | 0.003 |
| main_transparent_pass_3d | 0.008 | 0.009 | 0.020 | 0.90 | 0.086 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.90 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.534 | 11.817 | 23.45 | 58013.2 |
| render/busy/ms | 8.973 | 11.304 | 22.84 | 54601.7 |
| main/busy/ms | 8.764 | 10.394 | 30.41 | 53331.1 |
| stage/render/render | 5.595 | 7.089 | 14.49 | 34046.8 |
| stage/main/post_update | 5.172 | 6.464 | 9.85 | 31472.4 |
| render/graph/ms | 4.853 | 6.367 | 13.55 | 29532.1 |
| encode//sum | 4.297 | 5.790 | 20.35 | 26144.7 |
| stage/render/graph_submit | 3.634 | 4.853 | 10.22 | 22113.2 |
| encode/submit | 2.333 | 3.312 | 6.89 | 14194.0 |
| stage/main/update | 2.185 | 2.645 | 24.67 | 13295.2 |
| encode/finish_wall | 1.400 | 2.005 | 4.25 | 8517.2 |
| stage/render/camera_driver | 1.158 | 1.558 | 4.26 | 7044.7 |
| encode/pass:shadow_point_light/sum | 0.962 | 3.276 | 16.79 | 5851.4 |
| encode/pass:shadow_directional_light/sum | 0.864 | 1.568 | 2.72 | 5254.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.784 | 1.122 | 2.17 | 4773.6 |
| encode/pass:early prepass/sum | 0.745 | 1.087 | 1.64 | 4532.8 |
| stage/render/prepare_write_phase_buffers | 0.695 | 1.027 | 2.76 | 4228.0 |
| stage/render/prepare_assets | 0.552 | 0.713 | 9.80 | 3357.6 |
| bigspace/total/ms | 0.509 | 1.297 | 2.62 | 3098.0 |
| render/present/ms | 0.467 | 1.274 | 2.55 | 2838.9 |
| stage/render/prepare_resources | 0.450 | 0.545 | 1.02 | 2739.7 |
| stage/render/prepare_meshes | 0.435 | 0.557 | 4.44 | 2644.9 |
| stage/main/first | 0.401 | 0.521 | 5.58 | 2438.1 |
| stage/main/pre_update | 0.356 | 0.504 | 1.98 | 2164.0 |
| stage/render/prepare_bind_groups | 0.335 | 0.574 | 2.42 | 2038.2 |
| terrain/update/ms | 0.313 | 0.409 | 1.86 | 1901.8 |
| stage/render/prepare_flush | 0.291 | 0.512 | 1.62 | 1770.5 |
| controller/feed/ms | 0.279 | 0.359 | 0.98 | 1698.8 |
| terrain/select/ms | 0.279 | 0.367 | 1.11 | 1697.8 |
| stage/render/specialize | 0.269 | 0.366 | 5.99 | 1636.0 |
| stage/main/last | 0.257 | 0.311 | 0.86 | 1563.5 |
| stream/drive/ms | 0.254 | 0.344 | 10.79 | 1547.4 |
| stage/render/prepare_batch_phases | 0.233 | 0.352 | 0.63 | 1419.9 |
| stage/main/fixed_main_loop | 0.215 | 0.369 | 1.03 | 1309.3 |
| bigspace/low_precision/ms | 0.212 | 0.698 | 2.32 | 1292.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.205 | 0.314 | 1.04 | 1244.7 |
| far/select/ms | 0.201 | 0.279 | 0.57 | 1222.6 |
| stream/tick/ms | 0.199 | 0.273 | 0.59 | 1213.1 |
| stage/render/queue | 0.194 | 0.307 | 3.60 | 1179.0 |
| stage/render/create_views | 0.185 | 0.239 | 0.87 | 1125.2 |
| stage/render/extract_commands | 0.183 | 0.219 | 0.48 | 1116.1 |
| pipelines/compile_ms | 0.179 | 0.000 | 136.82 | 1090.7 |
| pipelines/front_ms | 0.174 | 0.000 | 134.21 | 1059.3 |
| anim/evaluate/ms | 0.161 | 0.258 | 1.56 | 977.6 |
| anim/evaluate_cpu/ms | 0.157 | 0.197 | 0.77 | 953.4 |
| bigspace/high_precision/ms | 0.154 | 0.697 | 2.27 | 939.0 |
| stage/render/prepare_views | 0.140 | 0.176 | 0.53 | 854.0 |
| controller/move/ms | 0.129 | 0.289 | 0.46 | 785.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.103 | 0.170 | 0.75 | 626.1 |
| render/acquire/ms | 0.094 | 0.121 | 0.47 | 572.5 |
| stage/render/phase_sort | 0.079 | 0.109 | 0.34 | 479.2 |
| physics/step/ms | 0.047 | 0.088 | 0.56 | 287.9 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.074 | 0.26 | 284.9 |
| physics/insert/ms | 0.032 | 0.362 | 0.90 | 197.3 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.82 | 191.2 |
| weather/map/ms | 0.026 | 0.034 | 0.22 | 155.9 |
| stage/render/cleanup | 0.024 | 0.034 | 1.01 | 144.9 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.30 | 80.2 |
| anim/apply/ms | 0.012 | 0.023 | 0.25 | 76.0 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.56 | 65.0 |
| weather/eval/ms | 0.011 | 0.014 | 0.20 | 64.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.06 | 63.9 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 44.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.25 | 33.3 |
| script/input/ms | 0.005 | 0.007 | 0.12 | 32.4 |
| pipelines/driver_ms | 0.005 | 0.000 | 3.89 | 31.4 |
| script/ui/ms | 0.005 | 0.006 | 0.20 | 28.1 |
| stream/upload/ms | 0.004 | 0.000 | 1.12 | 22.0 |
| controller/write/ms | 0.003 | 0.004 | 0.18 | 20.8 |
| stream/spawn/ms | 0.003 | 0.000 | 1.09 | 20.3 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 14.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 5.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.07 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.11 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1301 | 2262 | 641 |
| crowd/c0/meshes/count | count | 353 | 646 | 174 |
| crowd/c1/count | count | 145 | 155 | 147 |
| crowd/c1/joints/count | count | 5531 | 5937 | 5555 |
| crowd/c1/meshes/count | count | 1165 | 1281 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4607 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4585 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 48.7 k | 56.8 k | 47.0 k |
| encode//max | ms | 0.43 | 1.83 | 0.42 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.30 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.56 | 0.01 |
| encode/finish_critical | ms | 0.99 | 2.72 | 0.86 |
| encode/pass:early prepass/max | ms | 0.76 | 1.64 | 0.66 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.80 | 2.17 | 0.86 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.75 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.19 | 1.04 | 0.20 |
| encode/pass:shadow_directional_light/max | ms | 0.81 | 2.72 | 0.66 |
| encode/pass:shadow_point_light/max | ms | 0.79 | 1.50 | 0.91 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.82 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.11 | 30.41 | 8.46 |
| gpu/timed/ms | ms | 8.07 | 9.20 | 8.46 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 299.7 k | 299.7 k | 299.7 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 11.9 k | 13.1 k | 11.8 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 158 | 388 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 338 | 588 | 516 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 158 | 388 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 338 | 588 | 516 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 22 |
| phases/opaque3d/bins | count | 765 | 971 | 761 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2519 | 3591 | 2454 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 18 |
| phases/opaque3d_prepass/bins | count | 765 | 971 | 761 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2519 | 3591 | 2454 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 22 | 195 | 8 |
| phases/shadow/bins | count | 1970 | 3253 | 1067 |
| phases/shadow/instance_batches | count | 4 | 12 | 4 |
| phases/shadow/instance_draws | count | 6580 | 20.3 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| phases/shadow/items | count | 6026 | 12.5 k | 4367 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 167 | 229 | 30 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 189 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 327 | 328 | 328 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 18 | 536 | 31 |
| preprocess/bind_groups_created | count | 13 | 529 | 26 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 127 | 229 | 99 |
| process/cpu_usage | % | 26.34 | 36.54 | 34.62 |
| process/mem_usage | GiB | 5.51 | 5.69 | 5.69 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2976 | 3166 | 3166 |
| scene/batch/masked/live | count | 2976 | 3166 | 3166 |
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
| scene/batch/opaque/high_water | count | 18.8 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 18.8 k | 19.9 k | 19.9 k |
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
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1591 | 1621 | 1621 |
| scene/chains/live | count | 1591 | 1621 | 1621 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 13 | 128 | 12 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 367.1 k | 812.0 k | 495.6 k |
| skin/joints_written/count | count | 5736 | 12.7 k | 7744 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 6448 | 6572 | 6572 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.56 | 51.38 | 33.93 |
| system/mem_usage | % | 58.48 | 61.09 | 57.19 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2895 | 3010 | 3010 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 79 | 252 | 64 |
| views/all/bins | count | 3922 | 5313 | 3349 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 12 | 20 | 12 |
| views/all/instance_draws | count | 19.7 k | 33.8 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 147.5 k |
| views/all/items | count | 11.8 k | 19.3 k | 10.3 k |
| views/all/pipelines | count | 213 | 276 | 74 |
| views/all/pipelines_distinct | count | 70 | 74 | 59 |
| views/all/work_items | count | 154.0 k | 264.0 k | 153.8 k |
| views/cascade/batch_sets | count | 13 | 64 | 8 |
| views/cascade/bins | count | 1526 | 2548 | 1067 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 4 |
| views/cascade/instance_draws | count | 6580 | 20.3 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| views/cascade/items | count | 5235 | 11.5 k | 4367 |
| views/cascade/pipelines | count | 38 | 55 | 30 |
| views/cascade/work_items | count | 49.4 k | 158.9 k | 49.3 k |
| views/lamp_face/batch_sets | count | 2 | 146 | 0 |
| views/lamp_face/bins | count | 426 | 768 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 1 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 678 | 1115 | 0 |
| views/lamp_face/pipelines | count | 128 | 175 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 15 | 755 | 0 |
| views/main/batch_sets | count | 57 | 61 | 56 |
| views/main/bins | count | 2024 | 2318 | 2282 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5952 | 7602 | 5940 |
| views/main/pipelines | count | 46 | 49 | 44 |
| views/main/work_items | count | 104.5 k | 106.0 k | 104.5 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 20.6 k | 26.4 k | 18.5 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 68.85 | 297 | 419.0 k |
| anim/events/count | count | 0.09 | 2 | 524 |
| assets/Image/added | count | 0.10 | 43 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.47 | 254 | 2874 |
| assets/Mesh/removed | count | 0.80 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 471 |
| assets/StandardMaterial/modified | count | 3.56 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 9 |
| ecs/entities_delta | count | 0.24 | 1386 | 1489 |
| encode//count | count | 35.19 | 221 | 214.1 k |
| encode/bind_walk/init_textures | count | 12.9 k | 118.4 k | 78.38 M |
| encode/bind_walk/init_textures_skipped | count | 66.1 k | 155.9 k | 402.12 M |
| encode/bind_walk/inits | count | 455 | 1089 | 2.77 M |
| encode/bind_walk/inits_skipped | count | 93.21 | 152 | 567.2 k |
| encode/bind_walk/merge_textures | count | 12.8 k | 118.3 k | 77.94 M |
| encode/bind_walk/merge_textures_skipped | count | 5239 | 5983 | 31.88 M |
| encode/bind_walk/merges | count | 97.28 | 345 | 591.9 k |
| encode/bind_walk/merges_skipped | count | 68.39 | 128 | 416.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6085 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6085 |
| encode/pass:early prepass/count | count | 1 | 1 | 6085 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6085 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6085 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6085 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6085 |
| encode/pass:shadow_point_light/count | count | 1.56 | 48 | 9483 |
| encode/queue/command_buffers | count | 46.76 | 279 | 284.5 k |
| encode/queue/submits | count | 2.01 | 4 | 12.2 k |
| encode/queue/write_buffer/bytes | bytes | 2.71 M | 4.94 M | 16.47 G |
| encode/queue/write_buffer/calls | count | 270 | 807 | 1.65 M |
| encode/queue/write_buffer_with/bytes | bytes | 173.6 k | 1.30 M | 1.06 G |
| encode/queue/write_buffer_with/calls | count | 10.40 | 771 | 63.3 k |
| encode/queue/write_texture/bytes | bytes | 11.7 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6085 |
| physics/inserts | count | 0.37 | 108 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 161 | 2335 |
| physics/steps | count | 0.65 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 9 |
| pipelines/created | count | 0.00 | 2 | 13 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 2 | 6 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5563 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 30 |
| surface/shelter_redraws | count | 0.01 | 1 | 54 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1220 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.6 ms; slowest 38, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2555.1 | main/busy/ms 2522, stage/main/post_update 2417, stage/main/update 95.65, surface/advance_cpu_ms 66.77, stage/main/pre_update 4.92, stage/main/last 1.11, surface/cell_upload_ms 0.77, stage/main/first 0.59, stream/drive/ms 0.57, stage/main/fixed_main_loop 0.39, weather/map/ms 0.11, bigspace/total/ms 0.08 |
| warm-up | 3.33 | 771.5 | main/busy/ms 2685, stage/main/post_update 2444, render/thread/ms 722, render/busy/ms 557, stage/render/prepare_assets 508, stage/main/update 224, stage/render/prepare_views 160, render/acquire/ms 160, surface/advance_cpu_ms 66.77, stage/render/create_views 27.08, stage/render/render 16.20, render/graph/ms 9.76 |
| warm-up | 3.52 | 191.0 | render/thread/ms 897, render/busy/ms 642, stage/render/prepare_assets 559, main/busy/ms 190, stage/render/prepare_views 161, render/acquire/ms 160, encode//sum 141, stage/main/update 135, stage/render/render 129, render/present/ms 94.20, pipelines/compile_ms 42.44, stage/main/post_update 38.85 |
| warm-up | 3.57 | 52.5 | pipelines/compile_ms 551, pipelines/wait_ms 376, pipelines/front_ms 353, render/thread/ms 218, pipelines/driver_ms 198, stage/render/render 149, encode//sum 137, render/busy/ms 124, main/busy/ms 113, render/present/ms 93.75, stage/main/update 56.17, render/graph/ms 54.38 |
| warm-up | 3.78 | 205.8 | pipelines/wait_ms 4486, pipelines/compile_ms 2978, pipelines/front_ms 2659, pipelines/driver_ms 319, render/thread/ms 208, render/busy/ms 202, main/busy/ms 116, stage/render/prepare_assets 110, stage/render/render 86.44, render/graph/ms 80.04, stage/main/update 58.97, stage/render/graph_submit 51.03 |
| warm-up | 3.83 | 53.8 | pipelines/wait_ms 4113, pipelines/compile_ms 2585, pipelines/front_ms 2414, render/thread/ms 216, render/busy/ms 214, pipelines/driver_ms 170, stage/render/prepare_assets 129, encode//sum 115, stage/render/render 77.49, render/graph/ms 75.17, stage/render/graph_submit 69.07, main/busy/ms 50.44 |
| warm-up | 3.92 | 30.0 | pipelines/compile_ms 1161, pipelines/front_ms 1099, pipelines/wait_ms 164, main/busy/ms 86.64, stage/main/update 62.75, pipelines/driver_ms 61.56, render/thread/ms 36.86, render/busy/ms 32.24, stage/main/post_update 18.27, stage/render/render 17.69, render/graph/ms 12.40, encode//sum 9.63 |
| warm-up | 4.04 | 121.5 | pipelines/compile_ms 1535, pipelines/front_ms 1470, main/busy/ms 86.40, render/thread/ms 73.80, render/busy/ms 71.28, pipelines/driver_ms 65.31, stage/main/update 61.81, pipelines/wait_ms 49.96, stage/render/prepare_assets 47.80, stage/main/post_update 19.27, stage/render/render 14.63, render/graph/ms 11.31 |
| warm-up | 4.10 | 51.2 | pipelines/compile_ms 1519, pipelines/front_ms 1465, render/thread/ms 86.20, render/busy/ms 84.06, stage/render/prepare_assets 61.33, pipelines/driver_ms 53.63, pipelines/wait_ms 47.00, main/busy/ms 27.54, stage/render/render 16.76, render/graph/ms 14.35, terrain/update/ms 10.64, stage/main/post_update 10.19 |
| warm-up | 4.13 | 31.4 | pipelines/compile_ms 625, pipelines/front_ms 601, render/thread/ms 61.42, render/busy/ms 59.53, stage/render/prepare_assets 37.49, main/busy/ms 31.39, pipelines/driver_ms 24.18, stage/render/render 16.82, render/graph/ms 14.76, stage/main/post_update 12.11, stage/main/update 10.53, pipelines/wait_ms 10.19 |
| warm-up | 4.19 | 60.8 | pipelines/compile_ms 127, pipelines/front_ms 110, render/thread/ms 83.15, render/busy/ms 50.64, encode//sum 39.27, main/busy/ms 32.32, stage/render/prepare_views 30.40, render/acquire/ms 30.28, stage/render/render 26.09, render/graph/ms 23.30, stage/render/prepare_assets 21.19, stage/render/graph_submit 20.65 |
| warm-up | 4.25 | 41.1 | pipelines/compile_ms 578, pipelines/front_ms 559, main/busy/ms 53.24, render/thread/ms 39.53, render/busy/ms 37.83, stage/main/update 30.54, pipelines/driver_ms 19.11, stage/render/render 17.87, render/graph/ms 15.75, encode//sum 12.23, stage/render/graph_submit 11.86, stream/drive/ms 11.25 |
| warm-up | 4.34 | 89.7 | pipelines/compile_ms 1738, pipelines/front_ms 1671, render/thread/ms 89.25, render/busy/ms 86.85, pipelines/driver_ms 66.26, main/busy/ms 64.94, stage/main/update 41.89, stage/render/render 38.63, stage/render/prepare_assets 36.75, render/graph/ms 35.53, stream/drive/ms 21.80, pipelines/wait_ms 21.42 |
| warm-up | 4.40 | 59.4 | pipelines/compile_ms 1414, pipelines/front_ms 1358, render/thread/ms 96.90, render/busy/ms 94.85, stage/render/prepare_assets 56.52, pipelines/driver_ms 56.21, main/busy/ms 48.12, stage/main/update 31.25, stage/render/render 30.34, render/graph/ms 27.66, stream/drive/ms 21.49, pipelines/wait_ms 21.35 |
| warm-up | 4.50 | 97.0 | pipelines/compile_ms 1065, pipelines/front_ms 1008, main/busy/ms 151, stage/main/update 132, render/thread/ms 113, render/busy/ms 110, stage/render/prepare_assets 91.58, pipelines/driver_ms 56.64, stage/render/render 12.14, render/graph/ms 9.04, stage/main/post_update 8.84, stage/render/graph_submit 6.31 |
| warm-up | 4.63 | 127.9 | pipelines/compile_ms 1368, pipelines/front_ms 1305, main/busy/ms 151, stage/main/update 134, render/thread/ms 124, render/busy/ms 122, stage/render/prepare_assets 100, pipelines/driver_ms 63.09, stage/render/render 12.17, render/graph/ms 9.11, stage/main/post_update 7.85, stage/render/graph_submit 6.43 |
| warm-up | 4.67 | 40.1 | pipelines/compile_ms 924, pipelines/front_ms 888, render/thread/ms 84.59, render/busy/ms 83.12, stage/render/prepare_assets 54.89, main/busy/ms 52.70, pipelines/driver_ms 36.26, stage/main/update 25.89, stage/main/post_update 17.10, pipelines/wait_ms 16.14, stage/render/render 10.98, render/graph/ms 9.14 |
| warm-up | 4.71 | 41.7 | pipelines/compile_ms 1611, pipelines/front_ms 1557, pipelines/wait_ms 199, render/thread/ms 68.33, main/busy/ms 66.86, render/busy/ms 66.80, pipelines/driver_ms 54.77, stage/render/prepare_assets 30.83, stage/main/update 25.34, stage/main/post_update 23.19, stage/render/render 12.29, stage/main/pre_update 10.80 |
| warm-up | 4.76 | 50.5 | pipelines/compile_ms 1329, pipelines/front_ms 1291, pipelines/wait_ms 189, main/busy/ms 101, render/thread/ms 77.15, render/busy/ms 75.54, stage/main/post_update 50.57, pipelines/driver_ms 37.72, stage/main/update 26.66, stage/main/pre_update 18.87, stage/render/render 16.89, stage/render/prepare_assets 16.79 |
| warm-up | 4.85 | 96.1 | pipelines/compile_ms 1204, pipelines/front_ms 1167, render/thread/ms 112, render/busy/ms 111, main/busy/ms 97.91, pipelines/wait_ms 66.73, stage/main/post_update 51.34, pipelines/driver_ms 37.58, stage/main/update 29.44, encode//sum 28.99, stage/render/render 27.20, stage/render/prepare_resources 25.89 |
| warm-up | 4.95 | 90.3 | pipelines/compile_ms 2774, pipelines/front_ms 2690, pipelines/wait_ms 259, render/thread/ms 136, render/busy/ms 134, pipelines/driver_ms 84.55, main/busy/ms 59.83, stage/render/prepare_assets 48.09, encode//sum 32.01, stage/render/render 30.30, render/graph/ms 27.65, stage/main/update 26.29 |
| warm-up | 5.01 | 65.0 | pipelines/compile_ms 1932, pipelines/front_ms 1873, pipelines/wait_ms 200, render/thread/ms 125, render/busy/ms 122, main/busy/ms 91.55, stage/main/update 63.24, pipelines/driver_ms 59.51, stage/render/prepare_assets 47.28, encode//sum 46.50, stage/render/render 44.80, render/graph/ms 41.58 |
| warm-up | 5.14 | 128.2 | pipelines/compile_ms 1307, pipelines/front_ms 1272, main/busy/ms 135, render/thread/ms 121, render/busy/ms 119, stage/main/update 109, pipelines/wait_ms 73.93, stream/drive/ms 57.14, encode//sum 49.86, stage/render/render 44.71, stage/render/prepare_assets 42.64, render/graph/ms 42.04 |
| warm-up | 5.18 | 45.1 | pipelines/compile_ms 1759, pipelines/front_ms 1626, pipelines/driver_ms 133, render/thread/ms 106, render/busy/ms 104, main/busy/ms 95.34, pipelines/wait_ms 74.50, stage/main/update 72.21, stream/drive/ms 56.21, stage/render/prepare_assets 37.73, stage/render/cleanup 31.53, stage/render/render 31.31 |
| warm-up | 5.21 | 28.6 | pipelines/compile_ms 748, pipelines/front_ms 639, pipelines/driver_ms 109, render/thread/ms 63.58, render/busy/ms 61.97, main/busy/ms 38.96, stage/render/render 23.81, stage/main/update 23.31, render/graph/ms 21.81, stage/render/graph_submit 17.97, encode/submit 15.48, stage/render/prepare_assets 11.19 |
| warm-up | 5.24 | 28.6 | pipelines/compile_ms 76.82, pipelines/front_ms 72.74, render/thread/ms 46.87, render/busy/ms 45.42, main/busy/ms 40.39, stage/main/update 25.10, stage/render/prepare_assets 13.14, stage/render/render 13.04, render/graph/ms 11.12, encode//sum 9.19, stage/main/post_update 8.50, stage/render/graph_submit 7.76 |
| warm-up | 5.33 | 32.5 | render/thread/ms 49.15, render/busy/ms 47.84, main/busy/ms 39.26, stage/main/update 20.07, stage/main/post_update 14.08, stage/render/render 13.80, render/graph/ms 11.84, stage/render/prepare_assets 9.84, encode//sum 8.98, stage/render/graph_submit 8.24, encode/submit 5.88, stage/render/prepare_resources 5.09 |
| warm-up | 5.42 | 27.4 | pipelines/compile_ms 556, pipelines/front_ms 540, render/thread/ms 42.71, render/busy/ms 41.36, main/busy/ms 39.90, stage/render/render 22.76, stage/main/post_update 22.51, render/graph/ms 21.08, stage/render/graph_submit 17.78, pipelines/driver_ms 15.67, stage/main/update 13.32, encode//sum 8.37 |
| warm-up | 5.47 | 28.1 | pipelines/compile_ms 280, pipelines/front_ms 271, main/busy/ms 41.31, render/thread/ms 31.01, render/busy/ms 29.75, stage/main/update 21.60, stream/drive/ms 12.82, stage/render/render 11.28, stage/main/post_update 11.09, render/graph/ms 9.34, encode//sum 9.18, pipelines/driver_ms 8.77 |
| warm-up | 6.12 | 28.5 | render/thread/ms 37.01, render/busy/ms 36.02, main/busy/ms 26.54, stage/render/prepare_assets 19.98, stage/main/update 11.76, stage/main/post_update 9.09, stage/render/render 8.43, encode//sum 7.51, render/graph/ms 6.77, stage/render/graph_submit 4.29, stream/drive/ms 4.19, encode/submit 2.42 |
| warm-up | 6.21 | 29.4 | pipelines/compile_ms 150, pipelines/front_ms 147, render/thread/ms 41.99, main/busy/ms 41.23, render/busy/ms 40.84, stage/main/update 26.87, stage/render/prepare_assets 11.80, stage/render/render 11.12, stage/main/post_update 9.68, render/graph/ms 9.30, encode//sum 8.68, stage/render/prepare_meshes 6.06 |
| warm-up | 6.37 | 31.8 | render/thread/ms 43.82, render/busy/ms 42.82, main/busy/ms 32.18, stage/main/update 17.78, stage/render/prepare_assets 17.10, stage/render/render 9.07, stage/main/post_update 8.49, render/graph/ms 7.69, encode//sum 6.80, stage/render/graph_submit 5.48, stage/render/prepare_meshes 5.30, stage/render/specialize 4.03 |
| warm-up | 6.42 | 28.2 | pipelines/compile_ms 145, pipelines/front_ms 141, render/thread/ms 44.82, render/busy/ms 43.15, main/busy/ms 37.21, stage/main/update 19.56, stage/render/render 17.60, render/graph/ms 15.41, stage/main/post_update 13.72, stage/render/graph_submit 11.28, encode//sum 9.38, stage/render/prepare_assets 8.23 |
| warm-up | 6.47 | 28.6 | pipelines/compile_ms 98.71, pipelines/front_ms 93.77, render/thread/ms 34.79, render/busy/ms 33.69, main/busy/ms 27.32, stage/render/prepare_assets 16.10, stage/main/update 13.65, stage/main/post_update 10.08, stage/render/render 7.70, encode//sum 7.61, render/graph/ms 6.12, pipelines/driver_ms 4.94 |
| warm-up | 6.58 | 28.3 | main/busy/ms 41.97, render/thread/ms 27.65, stage/main/update 27.60, render/busy/ms 26.35, stream/drive/ms 14.08, stage/main/post_update 10.98, stage/render/render 10.50, render/graph/ms 8.52, encode//sum 7.80, stage/render/graph_submit 6.06, stage/render/prepare_meshes 4.60, stage/render/prepare_assets 4.22 |
| warm-up | 6.64 | 36.1 | pipelines/compile_ms 173, pipelines/front_ms 166, main/busy/ms 39.46, render/thread/ms 39.43, render/busy/ms 37.95, stage/main/update 24.87, stage/render/prepare_assets 15.54, stage/render/render 10.69, stage/main/post_update 9.33, render/graph/ms 8.53, encode//sum 7.65, pipelines/driver_ms 6.15 |
| measure | 18.15 | 34.2 | main/busy/ms 41.16, stage/main/update 26.50, render/thread/ms 20.95, render/busy/ms 19.96, stage/main/post_update 11.42, stage/render/render 10.09, render/graph/ms 8.69, stage/render/graph_submit 6.42, encode//sum 6.17, stage/render/prepare_assets 4.12, encode/submit 4.00, encode/finish_wall 2.48 |
| measure | 59.01 | 28.0 | render/thread/ms 38.12, render/busy/ms 37.04, main/busy/ms 24.72, stage/render/prepare_assets 15.35, stage/main/post_update 12.20, stage/render/render 11.50, encode//sum 11.09, render/graph/ms 9.94, stage/render/graph_submit 7.10, stage/main/update 6.82, encode/submit 4.37, encode/finish_wall 2.77 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=scene-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 16%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **81%** (min 59%) · 2619 MHz (min 2610) · memory 8101 MHz · 81.4 W · 79 °C (max 84) · VRAM 7023 MiB · throttled 100% of samples

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
