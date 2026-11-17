# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.day_gate=on (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6087 frames over 65.6 s · **median 10.55 ms (95 fps)** · mean 10.78 · p95 13.24 · p99 16.72 (1% low 60 fps) · worst 46.36 ms · 75 spikes over 1.5× median · 61.2 stutters/min (work 51.2/min)
- warm-up: 324 frames over 9.1 s, worst 2577.0 ms
- hitches (> 26.4 ms): **34** measured, 40 in warm-up
- timed GPU passes: 6.59 ms of a 10.78 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.55 ms, limit 11 ms
- **FAIL** frame_ms: 67 stutters (> 1.5x moving median of 60) in 1.09 min = 61.24/min, limit 1/min
- **FAIL** worst frame 46.36 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.853 | 2.037 | 2.070 | 0.91 | 0.176 |
| main_opaque_pass_3d | 1.519 | 1.670 | 2.058 | 0.91 | 0.059 |
| early prepass | 0.446 | 0.490 | 0.717 | 0.91 | 0.087 |
| bloom | 0.286 | 0.315 | 0.321 | 0.91 | 0.062 |
| atmosphere_luts | 0.276 | 0.304 | 0.367 | 0.91 | 0.003 |
| water_velocity | 0.255 | 0.280 | 0.347 | 0.91 | 0.002 |
| water_sspr | 0.184 | 0.202 | 0.203 | 0.91 | 0.002 |
| ssao | 0.171 | 0.188 | 0.193 | 0.91 | 0.007 |
| auxiliary_views | 0.134 | 0.147 | 0.541 | 0.91 | 0.142 |
| atmos_froxel_inject | 0.132 | 0.145 | 0.147 | 0.91 | 0.001 |
| clouds_trace | 0.128 | 0.140 | 0.233 | 0.91 | 0.001 |
| water_fft | 0.120 | 0.131 | 0.138 | 0.91 | 0.003 |
| uniform_allocation | 0.118 | 0.129 | 0.179 | 0.91 | 0.024 |
| upscaling | 0.108 | 0.119 | 0.126 | 0.91 | 0.004 |
| clouds_composite | 0.096 | 0.106 | 0.109 | 0.91 | 0.002 |
| bin_unpacking | 0.095 | 0.105 | 0.142 | 0.91 | 0.018 |
| main_transmissive_pass_3d | 0.077 | 0.085 | 0.179 | 0.91 | 0.009 |
| tonemapping | 0.077 | 0.085 | 0.088 | 0.91 | 0.004 |
| clustering | 0.058 | 0.064 | 0.067 | 0.91 | 0.018 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.91 | 0.002 |
| clouds_resolve | 0.044 | 0.049 | 0.049 | 0.91 | 0.001 |
| lightprobe_irradiance_map | 0.042 | 0.412 | 0.416 | 0.10 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.91 | 0.001 |
| surface_trail | 0.036 | 0.056 | 0.057 | 0.64 | 0.002 |
| surface_exposure_far | 0.034 | 0.042 | 0.045 | 0.80 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.91 | 0.003 |
| early_mesh_preprocessing | 0.027 | 0.030 | 0.047 | 0.91 | 0.004 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.032 | 0.91 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.91 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.219 | 0.230 | 0.10 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.91 | 0.002 |
| night_sky | 0.013 | 0.015 | 0.034 | 0.91 | 0.001 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.91 | 0.122 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.91 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.91 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.91 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.91 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.242 | 11.302 | 39.58 | 56258.1 |
| main/busy/ms | 8.959 | 10.545 | 28.80 | 54531.9 |
| render/busy/ms | 8.691 | 10.820 | 38.95 | 52904.6 |
| stage/main/post_update | 5.546 | 6.840 | 11.19 | 33760.0 |
| stage/render/render | 5.387 | 6.847 | 18.46 | 32789.6 |
| render/graph/ms | 4.645 | 6.095 | 17.22 | 28275.2 |
| encode//sum | 3.663 | 4.759 | 38.52 | 22294.6 |
| stage/render/graph_submit | 3.475 | 4.617 | 12.99 | 21153.7 |
| encode/submit | 2.217 | 3.126 | 6.62 | 13497.0 |
| stage/main/update | 2.152 | 2.566 | 22.21 | 13099.4 |
| encode/finish_wall | 1.329 | 1.920 | 7.30 | 8088.7 |
| stage/render/camera_driver | 1.117 | 1.534 | 6.72 | 6798.6 |
| encode/pass:shadow_point_light/sum | 0.851 | 3.185 | 17.22 | 5179.5 |
| encode/pass:shadow_directional_light/sum | 0.845 | 1.528 | 3.12 | 5142.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.767 | 1.066 | 2.28 | 4666.7 |
| stage/render/prepare_write_phase_buffers | 0.754 | 1.072 | 2.82 | 4587.9 |
| encode/pass:early prepass/sum | 0.742 | 1.055 | 4.35 | 4515.6 |
| stage/render/prepare_assets | 0.556 | 0.716 | 9.82 | 3382.5 |
| bigspace/total/ms | 0.545 | 1.328 | 6.38 | 3320.1 |
| render/present/ms | 0.456 | 1.165 | 2.25 | 2775.0 |
| stage/render/prepare_resources | 0.449 | 0.531 | 1.34 | 2733.0 |
| stage/render/prepare_meshes | 0.444 | 0.565 | 9.76 | 2700.1 |
| stage/main/first | 0.397 | 0.518 | 5.14 | 2416.4 |
| stage/main/pre_update | 0.355 | 0.522 | 1.60 | 2161.4 |
| stage/render/specialize | 0.312 | 0.397 | 11.27 | 1899.3 |
| terrain/update/ms | 0.305 | 0.394 | 1.88 | 1856.5 |
| stage/render/queue | 0.279 | 0.405 | 7.42 | 1698.5 |
| stage/render/prepare_bind_groups | 0.274 | 0.474 | 2.52 | 1667.4 |
| terrain/select/ms | 0.272 | 0.351 | 0.94 | 1653.6 |
| controller/feed/ms | 0.249 | 0.312 | 0.90 | 1513.9 |
| stream/drive/ms | 0.246 | 0.325 | 19.84 | 1500.1 |
| encode/pass:main_transparent_pass_3d/sum | 0.230 | 0.334 | 1.00 | 1398.6 |
| bigspace/low_precision/ms | 0.228 | 0.770 | 6.18 | 1389.2 |
| stage/main/fixed_main_loop | 0.214 | 0.368 | 1.35 | 1301.8 |
| far/select/ms | 0.199 | 0.263 | 1.17 | 1209.3 |
| stage/render/create_views | 0.198 | 0.257 | 1.38 | 1206.2 |
| stream/tick/ms | 0.190 | 0.259 | 0.74 | 1157.1 |
| stage/render/prepare_batch_phases | 0.188 | 0.269 | 0.55 | 1145.6 |
| stage/render/extract_commands | 0.169 | 0.198 | 0.42 | 1027.2 |
| anim/evaluate/ms | 0.165 | 0.292 | 1.51 | 1003.0 |
| stage/render/prepare_flush | 0.161 | 0.303 | 0.96 | 978.0 |
| bigspace/high_precision/ms | 0.153 | 0.656 | 2.51 | 930.3 |
| anim/evaluate_cpu/ms | 0.151 | 0.189 | 0.70 | 921.9 |
| stage/render/prepare_views | 0.139 | 0.173 | 0.46 | 845.6 |
| controller/move/ms | 0.127 | 0.226 | 0.42 | 773.8 |
| stage/main/last | 0.115 | 0.156 | 1.67 | 697.3 |
| render/acquire/ms | 0.095 | 0.120 | 0.42 | 578.4 |
| encode/pass:main_transmissive_pass_3d/sum | 0.094 | 0.150 | 0.68 | 571.7 |
| stage/render/prepare_collect_phase_buffers | 0.062 | 0.129 | 0.53 | 377.9 |
| pipelines/compile_ms | 0.047 | 0.000 | 123.68 | 285.2 |
| physics/step/ms | 0.046 | 0.087 | 0.58 | 283.0 |
| pipelines/front_ms | 0.045 | 0.000 | 119.45 | 273.8 |
| stage/render/cleanup | 0.034 | 0.110 | 0.77 | 209.6 |
| stage/render/phase_sort | 0.032 | 0.054 | 0.42 | 192.1 |
| physics/insert/ms | 0.031 | 0.353 | 0.82 | 191.5 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.46 | 189.6 |
| weather/map/ms | 0.025 | 0.033 | 0.09 | 150.9 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.48 | 88.1 |
| anim/apply/ms | 0.012 | 0.021 | 0.44 | 70.5 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.46 | 66.7 |
| controller/avoid/ms | 0.011 | 0.015 | 0.28 | 65.4 |
| weather/eval/ms | 0.010 | 0.014 | 0.24 | 62.1 |
| script/update/ms | 0.007 | 0.009 | 0.15 | 42.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.20 | 31.2 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 31.2 |
| script/ui/ms | 0.005 | 0.006 | 0.39 | 29.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 28.6 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 20.9 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.62 | 13.8 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.23 | 11.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1306 | 2262 | 528 |
| crowd/c0/meshes/count | count | 354 | 646 | 135 |
| crowd/c1/count | count | 145 | 156 | 149 |
| crowd/c1/joints/count | count | 5531 | 5991 | 5668 |
| crowd/c1/meshes/count | count | 1177 | 1295 | 1206 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4605 | 10.3 k | 6528 |
| crowd/joints_uploaded/count | count | 4594 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.39 | 6.00 | 0.38 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.48 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.46 | 0.02 |
| encode/finish_critical | ms | 0.96 | 6.00 | 1.39 |
| encode/pass:early prepass/max | ms | 0.76 | 4.35 | 0.98 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.78 | 2.28 | 1.11 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.68 | 0.06 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 1.00 | 0.18 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 3.12 | 1.39 |
| encode/pass:shadow_point_light/max | ms | 0.79 | 1.40 | 0.84 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.46 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.21 | 38.95 | 8.35 |
| gpu/timed/ms | ms | 7.99 | 11.11 | 8.16 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 811 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 811 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1297 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3399 | 5143 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1297 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3399 | 5143 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 16 | 203 | 29 |
| phases/shadow/bins | count | 5004 | 8884 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.3 k | 26.3 k | 6380 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 204 | 256 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 323 | 324 | 324 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 12 | 563 | 54 |
| preprocess/bind_groups_created | count | 12 | 563 | 54 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 231 | 127 |
| process/cpu_usage | % | 26.44 | 44.59 | 32.69 |
| process/mem_usage | GiB | 5.51 | 5.68 | 5.68 |
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
| skin/blocks_changed/count | count | 14 | 136 | 14 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 368.4 k | 808.7 k | 496.4 k |
| skin/joints_written/count | count | 5756 | 12.6 k | 7756 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4916 | 5010 | 5010 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.71 | 50.53 | 30.14 |
| system/mem_usage | % | 54.17 | 54.76 | 54.69 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 65 | 252 | 79 |
| views/all/bins | count | 7578 | 11.7 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 251 | 303 | 75 |
| views/all/pipelines_distinct | count | 70 | 74 | 61 |
| views/all/work_items | count | 10.7 k | 33.3 k | 14.3 k |
| views/cascade/batch_sets | count | 9 | 50 | 29 |
| views/cascade/bins | count | 2788 | 6216 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8899 | 23.5 k | 6380 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 415 | 22.6 k | 6380 |
| views/lamp_face/batch_sets | count | 0 | 192 | 0 |
| views/lamp_face/bins | count | 2153 | 2857 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2412 | 3152 | 0 |
| views/lamp_face/pipelines | count | 168 | 210 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2876 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2888 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8408 | 13.0 k | 7610 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8574 | 13.2 k | 7918 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 59.00 | 304 | 359.1 k |
| anim/events/count | count | 0.09 | 3 | 529 |
| assets/Image/added | count | 0.10 | 40 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.56 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1309 | 4675 |
| encode//count | count | 33.46 | 220 | 203.7 k |
| encode/bind_walk/init_textures | count | 12.5 k | 114.7 k | 76.26 M |
| encode/bind_walk/init_textures_skipped | count | 59.3 k | 138.7 k | 361.12 M |
| encode/bind_walk/inits | count | 458 | 1125 | 2.79 M |
| encode/bind_walk/inits_skipped | count | 104 | 165 | 632.7 k |
| encode/bind_walk/merge_textures | count | 12.5 k | 114.6 k | 75.82 M |
| encode/bind_walk/merge_textures_skipped | count | 5367 | 6012 | 32.67 M |
| encode/bind_walk/merges | count | 98.51 | 334 | 599.6 k |
| encode/bind_walk/merges_skipped | count | 82.16 | 129 | 500.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6087 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6087 |
| encode/pass:early prepass/count | count | 1 | 1 | 6087 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6087 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6087 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6087 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6087 |
| encode/pass:shadow_point_light/count | count | 1.38 | 48 | 8396 |
| encode/queue/command_buffers | count | 44.86 | 278 | 273.1 k |
| encode/queue/submits | count | 2.02 | 4 | 12.3 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.88 M | 11.46 G |
| encode/queue/write_buffer/calls | count | 259 | 842 | 1.58 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.6 k | 3.52 M | 1.08 G |
| encode/queue/write_buffer_with/calls | count | 10.99 | 465 | 66.9 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6087 |
| physics/inserts | count | 0.37 | 95 | 2262 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2335 |
| physics/steps | count | 0.65 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 22 | 378 |
| stream/upload/bytes | bytes | 5561 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 55 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1219 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.4 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2577.0 | main/busy/ms 2544, stage/main/post_update 2435, stage/main/update 101, surface/advance_cpu_ms 68.89, stage/main/pre_update 4.60, stage/main/last 1.06, surface/cell_upload_ms 0.81, stage/main/first 0.61, stream/drive/ms 0.39, stage/main/fixed_main_loop 0.37, weather/map/ms 0.16, stream/tick/ms 0.11 |
| warm-up | 3.29 | 711.5 | main/busy/ms 2724, stage/main/post_update 2461, render/thread/ms 678, render/busy/ms 562, stage/render/prepare_assets 533, stage/main/update 248, stage/render/prepare_views 112, render/acquire/ms 112, surface/advance_cpu_ms 68.89, stage/render/render 13.65, stage/render/create_views 8.96, render/graph/ms 8.80 |
| warm-up | 3.41 | 121.6 | render/thread/ms 782, render/busy/ms 602, stage/render/prepare_assets 557, main/busy/ms 203, stage/main/update 152, pipelines/compile_ms 139, stage/render/prepare_views 112, render/acquire/ms 112, pipelines/front_ms 85.21, stage/render/render 84.32, render/present/ms 67.68, pipelines/driver_ms 54.00 |
| warm-up | 3.55 | 119.0 | pipelines/wait_ms 3584, pipelines/compile_ms 1234, pipelines/front_ms 1117, main/busy/ms 165, pipelines/driver_ms 117, render/thread/ms 102, render/busy/ms 93.71, stage/main/update 74.33, stage/main/post_update 60.81, stage/render/prepare_bind_groups 26.66, stage/render/render 20.69, stage/render/prepare_assets 20.68 |
| warm-up | 3.66 | 102.6 | pipelines/wait_ms 5120, pipelines/compile_ms 2047, pipelines/front_ms 1923, render/thread/ms 151, render/busy/ms 148, main/busy/ms 137, pipelines/driver_ms 123, stage/main/update 84.45, stage/render/prepare_assets 80.43, stage/render/prepare_bind_groups 26.61, stage/main/post_update 22.05, stage/render/prepare_write_phase_buffers 19.45 |
| warm-up | 3.73 | 78.0 | pipelines/compile_ms 2036, pipelines/front_ms 1835, pipelines/wait_ms 1649, pipelines/driver_ms 201, render/thread/ms 146, render/busy/ms 144, stage/render/prepare_assets 129, main/busy/ms 40.51, stage/main/update 17.13, stage/main/post_update 16.66, terrain/update/ms 15.52, stage/render/render 9.52 |
| warm-up | 3.88 | 56.3 | pipelines/compile_ms 59.26, pipelines/front_ms 56.36, render/thread/ms 47.02, render/busy/ms 45.45, main/busy/ms 44.97, stage/main/update 33.80, stage/render/prepare_assets 32.17, stream/drive/ms 29.70, stage/render/render 8.43, stage/main/post_update 7.17, render/graph/ms 6.60, encode//sum 5.47 |
| warm-up | 3.97 | 58.6 | pipelines/compile_ms 751, pipelines/front_ms 715, render/thread/ms 59.14, render/busy/ms 57.46, main/busy/ms 51.59, stage/main/update 39.09, stage/render/prepare_assets 37.21, pipelines/driver_ms 36.20, stream/drive/ms 33.96, stage/render/render 13.82, render/graph/ms 11.51, stage/render/graph_submit 8.84 |
| warm-up | 4.09 | 96.5 | pipelines/compile_ms 494, pipelines/front_ms 475, render/thread/ms 76.35, render/busy/ms 73.85, main/busy/ms 56.23, stage/render/prepare_assets 49.00, stage/main/update 28.58, pipelines/driver_ms 19.15, stage/main/post_update 15.83, stage/render/render 15.61, stream/drive/ms 15.01, render/graph/ms 13.16 |
| warm-up | 4.14 | 50.4 | pipelines/compile_ms 688, pipelines/front_ms 663, render/thread/ms 89.58, render/busy/ms 87.98, stage/render/prepare_assets 67.17, main/busy/ms 52.34, stage/main/update 30.87, pipelines/driver_ms 24.82, stage/main/post_update 13.36, stage/render/render 11.59, render/graph/ms 9.40, stage/render/graph_submit 6.99 |
| warm-up | 4.19 | 49.1 | pipelines/compile_ms 1227, pipelines/front_ms 1186, render/thread/ms 63.86, render/busy/ms 62.21, main/busy/ms 46.96, stage/render/prepare_assets 41.71, pipelines/driver_ms 40.54, stage/main/update 30.18, stage/render/render 10.55, stage/main/post_update 8.50, render/graph/ms 8.21, encode//sum 6.33 |
| warm-up | 4.27 | 48.1 | pipelines/compile_ms 104, pipelines/front_ms 99.15, render/thread/ms 54.25, render/busy/ms 52.08, main/busy/ms 42.72, stage/main/update 23.02, stage/render/prepare_assets 19.06, stage/render/render 17.42, render/graph/ms 14.72, stage/render/graph_submit 11.92, stage/main/post_update 11.81, encode//sum 6.72 |
| warm-up | 4.35 | 87.9 | pipelines/compile_ms 1442, pipelines/front_ms 1399, pipelines/wait_ms 209, render/thread/ms 111, render/busy/ms 108, pipelines/driver_ms 42.64, main/busy/ms 39.11, stage/render/prepare_write_phase_buffers 36.08, stage/render/prepare_assets 25.00, stage/render/render 15.23, render/graph/ms 12.53, stage/main/update 11.36 |
| warm-up | 4.44 | 82.0 | pipelines/compile_ms 2455, pipelines/front_ms 2373, pipelines/wait_ms 482, render/thread/ms 141, render/busy/ms 138, pipelines/driver_ms 82.31, main/busy/ms 66.43, stage/main/post_update 40.22, stage/render/prepare_write_phase_buffers 35.98, stage/render/prepare_assets 28.11, encode//sum 21.40, stage/render/render 20.33 |
| warm-up | 4.49 | 56.0 | pipelines/compile_ms 1527, pipelines/front_ms 1470, pipelines/wait_ms 325, render/thread/ms 112, render/busy/ms 109, main/busy/ms 72.97, pipelines/driver_ms 56.99, stage/main/post_update 43.65, stage/render/prepare_assets 34.80, encode//sum 34.49, stage/render/render 24.39, render/graph/ms 21.71 |
| warm-up | 4.55 | 54.9 | pipelines/compile_ms 941, pipelines/front_ms 910, pipelines/wait_ms 176, main/busy/ms 131, stage/main/update 101, render/thread/ms 96.81, render/busy/ms 94.86, encode//sum 70.12, stream/drive/ms 56.54, stage/render/render 35.06, render/graph/ms 32.36, pipelines/driver_ms 31.54 |
| warm-up | 4.68 | 138.4 | pipelines/compile_ms 2614, pipelines/front_ms 2520, pipelines/wait_ms 371, main/busy/ms 119, render/thread/ms 113, render/busy/ms 111, stage/main/update 94.87, pipelines/driver_ms 93.64, stream/drive/ms 57.72, encode//sum 57.42, stage/render/prepare_assets 45.34, stage/render/render 31.84 |
| warm-up | 4.73 | 46.4 | pipelines/compile_ms 2827, pipelines/front_ms 2733, pipelines/wait_ms 315, render/thread/ms 100, render/busy/ms 97.54, main/busy/ms 94.48, pipelines/driver_ms 94.43, stage/main/update 74.99, stream/drive/ms 65.21, stage/render/prepare_assets 47.25, stage/render/render 20.07, render/graph/ms 16.64 |
| warm-up | 4.83 | 103.6 | pipelines/compile_ms 1186, pipelines/front_ms 1151, main/busy/ms 104, pipelines/wait_ms 103, render/thread/ms 80.35, stage/main/update 78.86, render/busy/ms 78.01, stream/drive/ms 65.26, stage/render/prepare_assets 48.49, pipelines/driver_ms 34.39, stage/main/post_update 16.38, stage/render/render 15.04 |
| warm-up | 4.91 | 72.5 | pipelines/compile_ms 1613, pipelines/front_ms 1562, render/thread/ms 108, render/busy/ms 107, main/busy/ms 92.10, stage/main/update 62.91, pipelines/driver_ms 51.32, stage/render/prepare_assets 51.18, pipelines/wait_ms 45.81, stage/render/render 45.32, render/graph/ms 43.40, stream/drive/ms 42.17 |
| warm-up | 4.95 | 47.5 | pipelines/compile_ms 1327, pipelines/front_ms 1289, render/thread/ms 97.64, render/busy/ms 96.26, main/busy/ms 80.32, stage/main/update 59.39, stage/render/render 46.25, render/graph/ms 44.35, stage/render/graph_submit 41.43, stream/drive/ms 41.30, encode//sum 41.29, stage/render/prepare_assets 37.86 |
| warm-up | 5.03 | 49.7 | pipelines/compile_ms 541, pipelines/front_ms 522, main/busy/ms 56.64, stage/main/update 41.15, render/thread/ms 35.56, render/busy/ms 34.26, stream/drive/ms 29.43, pipelines/driver_ms 18.42, stage/render/prepare_assets 12.45, stage/render/render 10.19, stage/main/post_update 9.61, render/graph/ms 8.37 |
| warm-up | 5.18 | 41.8 | pipelines/compile_ms 298, pipelines/front_ms 291, render/thread/ms 51.96, render/busy/ms 50.54, main/busy/ms 39.24, stage/render/prepare_assets 27.53, stage/main/update 20.17, stage/main/post_update 12.75, stage/render/render 12.29, render/graph/ms 10.31, stage/render/graph_submit 7.74, pipelines/driver_ms 7.10 |
| warm-up | 5.22 | 37.6 | pipelines/compile_ms 537, pipelines/front_ms 507, render/thread/ms 61.95, render/busy/ms 60.59, main/busy/ms 43.39, stage/render/prepare_assets 36.28, pipelines/driver_ms 29.40, stage/main/update 22.76, stage/render/render 14.17, stage/main/post_update 13.00, render/graph/ms 12.03, stage/render/graph_submit 9.45 |
| warm-up | 5.27 | 52.8 | pipelines/compile_ms 618, pipelines/front_ms 584, render/thread/ms 81.90, render/busy/ms 80.49, main/busy/ms 70.67, pipelines/driver_ms 34.13, stage/main/update 28.48, stage/main/post_update 27.92, stage/render/prepare_assets 25.65, stage/render/render 20.84, render/graph/ms 18.67, encode//sum 16.68 |
| warm-up | 5.34 | 36.6 | pipelines/compile_ms 192, pipelines/front_ms 187, render/thread/ms 59.35, render/busy/ms 57.27, main/busy/ms 43.39, stage/main/post_update 25.27, stage/render/prepare_assets 24.05, stage/render/render 12.76, stage/main/update 12.22, render/graph/ms 10.69, encode//sum 8.56, stage/render/graph_submit 7.77 |
| warm-up | 5.37 | 29.1 | pipelines/compile_ms 383, pipelines/front_ms 372, render/thread/ms 60.01, render/busy/ms 58.44, main/busy/ms 40.04, stage/render/prepare_assets 30.66, stage/main/post_update 23.80, stage/render/render 11.90, pipelines/driver_ms 10.61, stage/main/update 10.29, render/graph/ms 9.64, encode//sum 8.62 |
| warm-up | 5.39 | 29.0 | pipelines/compile_ms 1631, pipelines/front_ms 1585, render/thread/ms 54.30, render/busy/ms 52.47, pipelines/driver_ms 46.09, main/busy/ms 39.14, stage/render/prepare_assets 25.98, stage/main/post_update 19.35, stage/render/render 12.89, stage/main/update 12.87, render/graph/ms 10.63, encode//sum 8.38 |
| warm-up | 5.46 | 40.5 | pipelines/compile_ms 763, pipelines/front_ms 747, render/thread/ms 50.58, render/busy/ms 49.05, main/busy/ms 42.47, stage/render/prepare_assets 29.79, stage/main/update 22.74, pipelines/driver_ms 16.51, stage/main/post_update 12.79, stage/render/render 12.55, stream/drive/ms 10.39, render/graph/ms 10.08 |
| warm-up | 5.49 | 29.5 | pipelines/compile_ms 194, pipelines/front_ms 190, render/thread/ms 54.02, render/busy/ms 52.80, stage/render/prepare_assets 34.05, main/busy/ms 30.88, stage/main/update 13.87, stage/main/post_update 11.29, stage/render/render 9.36, encode//sum 7.95, render/graph/ms 7.54, stage/render/graph_submit 5.54 |
| warm-up | 6.09 | 32.2 | render/thread/ms 39.48, render/busy/ms 38.43, main/busy/ms 27.38, stage/render/prepare_assets 17.03, stage/main/update 10.66, stage/main/post_update 10.64, stage/render/render 8.59, render/graph/ms 7.11, encode//sum 7.11, stage/render/graph_submit 4.99, stream/drive/ms 3.80, stage/render/specialize 3.62 |
| warm-up | 6.31 | 30.0 | pipelines/compile_ms 147, pipelines/front_ms 141, render/thread/ms 46.84, render/busy/ms 45.41, main/busy/ms 41.16, stage/main/update 16.95, stage/main/post_update 16.06, stage/render/render 13.95, render/graph/ms 11.90, stage/render/prepare_assets 9.55, encode//sum 9.50, stage/render/graph_submit 7.87 |
| warm-up | 6.42 | 40.3 | main/busy/ms 45.58, render/thread/ms 31.05, render/busy/ms 29.75, stage/main/update 25.33, stream/drive/ms 15.30, stage/main/post_update 14.38, stage/render/render 11.68, render/graph/ms 9.70, encode//sum 8.48, stage/render/prepare_assets 6.78, stage/render/graph_submit 6.54, encode/submit 3.80 |
| warm-up | 6.85 | 35.7 | main/busy/ms 42.28, render/thread/ms 24.31, render/busy/ms 23.25, stage/main/update 22.77, stream/drive/ms 14.12, stage/main/post_update 13.62, stream/spawn/ms 12.75, stage/render/render 9.71, render/graph/ms 8.28, encode//sum 7.29, stage/render/graph_submit 5.80, stage/render/prepare_assets 5.70 |
| warm-up | 6.93 | 37.9 | render/thread/ms 45.60, render/busy/ms 44.52, main/busy/ms 26.83, stage/render/render 12.45, stage/render/specialize 11.61, render/graph/ms 10.88, stage/main/update 10.83, stage/main/post_update 10.59, stage/render/queue 8.98, stage/render/graph_submit 8.58, encode//sum 7.68, encode/submit 4.49 |
| warm-up | 8.23 | 31.8 | render/thread/ms 39.56, render/busy/ms 38.51, main/busy/ms 18.34, stage/render/render 11.93, stage/main/post_update 10.64, render/graph/ms 10.46, stage/render/specialize 9.92, stage/render/graph_submit 7.97, encode//sum 7.37, stage/render/queue 6.75, stage/main/update 4.96, encode/submit 4.58 |
| measure | 0.42 | 32.2 | render/thread/ms 35.96, render/busy/ms 34.97, main/busy/ms 16.40, stage/render/render 11.36, stage/main/post_update 10.04, render/graph/ms 9.86, stage/render/specialize 9.15, stage/render/graph_submit 7.78, encode//sum 7.14, stage/render/queue 5.92, encode/submit 5.13, stage/main/update 3.66 |
| measure | 1.73 | 30.3 | render/thread/ms 38.07, render/busy/ms 37.01, main/busy/ms 19.15, stage/render/render 12.33, stage/main/post_update 12.11, render/graph/ms 10.88, stage/render/specialize 9.11, encode//sum 8.68, stage/render/graph_submit 7.64, stage/render/queue 5.71, encode/submit 4.62, stage/main/update 4.25 |
| measure | 2.72 | 32.1 | encode//sum 42.07, render/thread/ms 36.53, render/busy/ms 35.23, stage/render/render 23.55, render/graph/ms 21.69, main/busy/ms 19.07, stage/render/graph_submit 16.22, encode/pass:shadow_point_light/sum 14.29, stage/main/post_update 11.23, encode/finish_wall 8.37, encode/submit 7.91, stage/main/update 5.57 |
| measure | 3.09 | 28.6 | render/thread/ms 33.90, render/busy/ms 32.81, main/busy/ms 18.29, stage/main/post_update 12.09, stage/render/render 9.78, stage/render/specialize 9.35, render/graph/ms 8.22, encode//sum 7.11, stage/render/graph_submit 6.12, stage/render/queue 6.00, stage/main/update 3.82, encode/submit 3.57 |
| measure | 4.44 | 29.8 | render/thread/ms 36.17, render/busy/ms 35.18, main/busy/ms 18.91, stage/main/post_update 12.47, stage/render/render 11.62, render/graph/ms 10.17, stage/render/specialize 9.47, encode//sum 8.31, stage/render/graph_submit 7.71, stage/render/queue 6.05, encode/submit 4.58, encode/pass:shadow_point_light/sum 3.89 |
| measure | 5.77 | 29.0 | render/thread/ms 34.60, render/busy/ms 33.51, main/busy/ms 19.13, stage/main/post_update 12.35, stage/render/render 9.76, stage/render/specialize 9.38, render/graph/ms 8.26, encode//sum 6.77, stage/render/queue 6.46, stage/render/graph_submit 6.22, stage/main/update 4.08, encode/submit 3.34 |
| measure | 7.05 | 28.3 | render/thread/ms 36.39, render/busy/ms 35.38, main/busy/ms 16.94, stage/render/render 12.20, render/graph/ms 10.65, stage/main/post_update 10.64, stage/render/specialize 9.47, encode//sum 7.92, stage/render/graph_submit 7.91, stage/render/queue 5.79, encode/submit 4.67, stage/main/update 3.89 |
| measure | 9.57 | 29.9 | render/thread/ms 35.12, render/busy/ms 33.97, main/busy/ms 18.39, stage/main/post_update 11.36, stage/render/render 10.71, stage/render/specialize 9.23, render/graph/ms 9.22, encode//sum 7.44, stage/render/graph_submit 6.81, stage/render/queue 6.04, stage/main/update 4.15, encode/submit 3.83 |
| measure | 15.74 | 31.9 | render/thread/ms 40.57, render/busy/ms 38.49, main/busy/ms 18.48, stage/render/render 13.82, render/graph/ms 11.31, stage/main/post_update 11.24, stage/render/specialize 10.47, stage/render/graph_submit 8.69, encode//sum 8.25, stage/render/queue 7.05, encode/submit 5.51, encode/pass:shadow_point_light/sum 5.32 |
| measure | 17.15 | 28.8 | render/thread/ms 37.25, render/busy/ms 36.17, main/busy/ms 19.11, stage/render/render 11.73, stage/main/post_update 11.70, render/graph/ms 10.18, stage/render/specialize 9.47, encode//sum 8.94, stage/render/graph_submit 7.46, stage/render/queue 6.46, encode/pass:shadow_point_light/sum 5.49, encode/submit 4.87 |
| measure | 18.01 | 28.6 | render/thread/ms 37.78, render/busy/ms 36.68, main/busy/ms 18.93, stage/render/render 12.12, stage/main/post_update 11.79, render/graph/ms 10.54, stage/render/specialize 9.32, encode//sum 8.59, stage/render/graph_submit 7.92, stage/render/queue 6.45, encode/submit 4.90, encode/pass:shadow_point_light/sum 4.39 |
| measure | 18.18 | 30.4 | main/busy/ms 38.66, stage/main/update 24.25, render/thread/ms 21.93, render/busy/ms 20.92, stream/drive/ms 20.08, stage/render/render 13.42, render/graph/ms 11.97, stage/main/post_update 11.40, encode//sum 9.13, stage/render/graph_submit 9.02, encode/submit 6.10, encode/pass:shadow_point_light/sum 5.95 |
| measure | 19.46 | 30.9 | render/thread/ms 40.62, render/busy/ms 39.58, main/busy/ms 19.95, stage/render/render 12.06, stage/main/post_update 11.41, render/graph/ms 10.47, stage/render/specialize 9.59, encode//sum 9.15, stage/render/graph_submit 7.86, stage/render/queue 7.24, stage/main/update 5.92, encode/submit 4.97 |
| measure | 20.82 | 29.5 | render/thread/ms 38.35, render/busy/ms 37.33, main/busy/ms 19.28, stage/render/render 11.94, stage/main/post_update 11.59, render/graph/ms 10.37, stage/render/specialize 9.95, encode//sum 9.45, stage/render/graph_submit 7.82, stage/render/queue 6.99, stage/main/update 5.14, encode/submit 4.62 |
| measure | 22.99 | 36.4 | render/thread/ms 41.58, render/busy/ms 40.62, main/busy/ms 27.91, stage/main/post_update 12.58, stage/render/specialize 11.52, stage/render/render 10.29, render/graph/ms 8.93, stage/render/queue 7.85, encode//sum 7.42, stage/render/graph_submit 6.77, stage/main/update 6.40, stage/main/first 4.59 |
| measure | 24.42 | 28.7 | render/thread/ms 35.93, render/busy/ms 34.82, main/busy/ms 18.92, stage/main/post_update 11.61, stage/render/render 10.34, stage/render/specialize 10.02, render/graph/ms 8.84, stage/render/graph_submit 6.88, stage/render/queue 6.40, encode//sum 6.35, stage/main/update 4.86, encode/submit 3.80 |
| measure | 27.28 | 37.5 | render/thread/ms 43.78, render/busy/ms 42.70, main/busy/ms 26.45, stage/render/render 14.46, stage/main/post_update 14.03, render/graph/ms 12.89, encode//sum 12.46, stage/render/specialize 11.59, stage/render/graph_submit 9.93, encode/pass:shadow_point_light/sum 7.86, stage/render/queue 7.45, stage/main/update 6.67 |
| measure | 28.68 | 29.4 | render/thread/ms 35.87, render/busy/ms 34.80, main/busy/ms 19.10, stage/main/post_update 11.70, stage/render/render 10.47, stage/render/specialize 9.74, render/graph/ms 8.98, encode//sum 7.21, stage/render/graph_submit 6.91, stage/render/queue 6.63, stage/main/update 4.48, encode/submit 3.86 |
| measure | 30.45 | 31.9 | render/thread/ms 36.52, render/busy/ms 35.47, main/busy/ms 20.63, stage/main/post_update 11.93, stage/render/render 10.36, stage/render/specialize 10.33, render/graph/ms 8.87, encode//sum 7.42, stage/render/graph_submit 6.83, stage/render/queue 6.55, stage/main/update 5.74, encode/submit 3.95 |
| measure | 35.36 | 31.2 | render/thread/ms 41.36, render/busy/ms 40.30, main/busy/ms 21.87, stage/render/render 13.41, stage/main/post_update 13.21, render/graph/ms 11.90, stage/render/specialize 10.60, stage/render/graph_submit 9.15, encode//sum 8.59, stage/render/queue 6.79, encode/pass:shadow_point_light/sum 5.89, encode/submit 5.61 |
| measure | 36.85 | 30.1 | render/thread/ms 38.72, render/busy/ms 37.63, main/busy/ms 18.33, stage/render/render 12.42, stage/main/post_update 11.63, render/graph/ms 10.88, stage/render/specialize 10.21, encode//sum 8.80, stage/render/graph_submit 7.96, encode/pass:shadow_point_light/sum 6.96, stage/render/queue 6.39, encode/submit 5.27 |
| measure | 37.48 | 30.7 | render/thread/ms 39.24, render/busy/ms 38.15, main/busy/ms 21.89, stage/render/render 13.13, stage/main/post_update 11.73, render/graph/ms 11.53, stage/render/specialize 10.26, encode//sum 9.96, stage/render/graph_submit 8.87, stage/render/queue 6.27, stage/main/update 6.03, encode/pass:shadow_point_light/sum 5.79 |
| measure | 40.78 | 32.0 | render/thread/ms 38.01, render/busy/ms 36.83, main/busy/ms 19.06, stage/main/post_update 10.97, stage/render/render 10.48, stage/render/specialize 9.81, render/graph/ms 9.04, encode//sum 7.23, stage/render/graph_submit 7.13, stage/render/queue 6.41, stage/main/update 4.53, encode/submit 3.79 |
| measure | 43.22 | 31.7 | render/thread/ms 37.27, render/busy/ms 36.13, main/busy/ms 16.36, stage/render/render 10.48, stage/render/specialize 10.27, stage/main/post_update 9.41, render/graph/ms 9.06, encode//sum 7.76, stage/render/graph_submit 7.22, stage/render/queue 6.47, stage/main/update 4.47, encode/finish_wall 3.73 |
| measure | 45.67 | 33.0 | render/thread/ms 37.55, render/busy/ms 36.57, main/busy/ms 17.58, stage/render/specialize 10.22, stage/main/post_update 9.96, stage/render/render 9.83, render/graph/ms 8.45, encode//sum 7.59, stage/render/graph_submit 6.67, stage/render/queue 6.27, stage/main/update 4.71, stage/render/prepare_meshes 4.52 |
| measure | 59.03 | 46.4 | render/thread/ms 56.52, render/busy/ms 55.04, main/busy/ms 32.57, stage/main/post_update 18.80, stage/render/render 17.28, render/graph/ms 15.35, stage/render/graph_submit 12.40, encode//sum 10.89, stage/render/prepare_meshes 10.11, stage/render/specialize 9.55, stage/main/update 9.23, encode/submit 6.46 |
| measure | 59.06 | 29.5 | render/thread/ms 58.41, render/busy/ms 57.35, main/busy/ms 26.32, stage/render/prepare_assets 15.26, stage/main/post_update 13.60, stage/render/prepare_meshes 10.74, stage/render/render 9.82, stage/render/specialize 9.69, render/graph/ms 8.49, stage/main/update 7.45, encode//sum 7.18, stage/render/graph_submit 6.54 |
| measure | 62.57 | 31.4 | render/thread/ms 38.07, render/busy/ms 37.14, main/busy/ms 18.59, stage/render/render 10.62, stage/render/specialize 10.58, stage/main/post_update 9.82, render/graph/ms 9.32, encode//sum 8.07, stage/render/graph_submit 7.22, stage/render/queue 6.36, stage/main/update 6.13, stage/render/prepare_meshes 4.31 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.day_gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=daygate-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 7%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **79%** (min 54%) · 2620 MHz (min 2610) · memory 8101 MHz · 80.1 W · 77 °C (max 82) · VRAM 6806 MiB · throttled 100% of samples

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
- `shadow.lamp.day_gate=on (command line: --set)`
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
