# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6074 frames over 65.6 s · **median 10.52 ms (95 fps)** · mean 10.81 · p95 13.35 · p99 16.18 (1% low 62 fps) · worst 48.50 ms · 67 spikes over 1.5× median · 56.7 stutters/min (work 54.8/min)
- warm-up: 326 frames over 9.3 s, worst 2575.1 ms
- hitches (> 26.3 ms): **34** measured, 48 in warm-up
- timed GPU passes: 6.48 ms of a 10.81 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.52 ms, limit 11 ms
- **FAIL** frame_ms: 62 stutters (> 1.5x moving median of 60) in 1.09 min = 56.67/min, limit 1/min
- **FAIL** worst frame 48.50 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.845 | 2.036 | 2.071 | 0.91 | 0.174 |
| main_opaque_pass_3d | 1.474 | 1.626 | 2.052 | 0.91 | 0.059 |
| early prepass | 0.449 | 0.495 | 0.733 | 0.91 | 0.087 |
| bloom | 0.285 | 0.315 | 0.321 | 0.91 | 0.063 |
| atmosphere_luts | 0.275 | 0.304 | 0.367 | 0.91 | 0.003 |
| water_velocity | 0.254 | 0.280 | 0.347 | 0.91 | 0.002 |
| water_sspr | 0.183 | 0.202 | 0.203 | 0.91 | 0.002 |
| ssao | 0.171 | 0.188 | 0.193 | 0.91 | 0.007 |
| atmos_froxel_inject | 0.132 | 0.146 | 0.148 | 0.91 | 0.001 |
| clouds_trace | 0.126 | 0.139 | 0.234 | 0.91 | 0.001 |
| water_fft | 0.121 | 0.133 | 0.140 | 0.91 | 0.003 |
| uniform_allocation | 0.115 | 0.127 | 0.178 | 0.91 | 0.024 |
| upscaling | 0.107 | 0.118 | 0.126 | 0.91 | 0.004 |
| auxiliary_views | 0.099 | 0.109 | 0.483 | 0.91 | 0.110 |
| clouds_composite | 0.096 | 0.106 | 0.109 | 0.91 | 0.002 |
| bin_unpacking | 0.094 | 0.104 | 0.142 | 0.91 | 0.018 |
| main_transmissive_pass_3d | 0.078 | 0.086 | 0.179 | 0.91 | 0.008 |
| tonemapping | 0.077 | 0.085 | 0.088 | 0.91 | 0.003 |
| clustering | 0.058 | 0.064 | 0.067 | 0.91 | 0.018 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.91 | 0.002 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.91 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.91 | 0.001 |
| surface_trail | 0.036 | 0.056 | 0.057 | 0.63 | 0.002 |
| lightprobe_irradiance_map | 0.036 | 0.412 | 0.417 | 0.09 | 0.001 |
| surface_exposure_far | 0.034 | 0.043 | 0.045 | 0.81 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.91 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.047 | 0.91 | 0.004 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.031 | 0.91 | 0.001 |
| water_snapshot | 0.022 | 0.025 | 0.028 | 0.91 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.91 | 0.002 |
| lightprobe_radiance_map | 0.019 | 0.219 | 0.230 | 0.09 | 0.004 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.91 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.91 | 0.125 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.91 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.91 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.91 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.90 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.185 | 11.178 | 43.27 | 55788.5 |
| main/busy/ms | 9.068 | 10.706 | 28.13 | 55079.8 |
| render/busy/ms | 8.635 | 10.674 | 42.71 | 52447.8 |
| stage/main/post_update | 5.646 | 6.955 | 10.04 | 34296.0 |
| stage/render/render | 5.326 | 6.744 | 16.65 | 32349.1 |
| render/graph/ms | 4.587 | 5.985 | 15.50 | 27864.3 |
| encode//sum | 4.087 | 5.425 | 27.78 | 24826.9 |
| stage/render/graph_submit | 3.445 | 4.618 | 11.12 | 20924.0 |
| encode/submit | 2.196 | 3.166 | 6.41 | 13338.7 |
| stage/main/update | 2.149 | 2.552 | 21.78 | 13053.0 |
| encode/finish_wall | 1.316 | 1.914 | 4.82 | 7995.0 |
| stage/render/camera_driver | 1.090 | 1.487 | 8.06 | 6621.6 |
| encode/pass:shadow_directional_light/sum | 0.826 | 1.506 | 2.56 | 5016.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.744 | 1.052 | 1.57 | 4520.2 |
| stage/render/prepare_write_phase_buffers | 0.735 | 1.043 | 2.40 | 4466.1 |
| encode/pass:early prepass/sum | 0.722 | 1.045 | 1.75 | 4387.9 |
| encode/pass:shadow_point_light/sum | 0.605 | 2.725 | 15.01 | 3673.2 |
| bigspace/total/ms | 0.574 | 1.349 | 2.53 | 3487.0 |
| stage/render/prepare_assets | 0.568 | 0.734 | 13.87 | 3450.0 |
| render/present/ms | 0.455 | 0.650 | 2.56 | 2764.9 |
| stage/render/prepare_resources | 0.450 | 0.540 | 4.26 | 2732.4 |
| stage/render/prepare_meshes | 0.442 | 0.565 | 5.28 | 2685.1 |
| stage/main/first | 0.404 | 0.523 | 6.51 | 2452.4 |
| stage/main/pre_update | 0.358 | 0.517 | 3.64 | 2176.3 |
| stage/render/specialize | 0.316 | 0.415 | 12.00 | 1921.6 |
| terrain/update/ms | 0.315 | 0.412 | 2.14 | 1915.8 |
| terrain/select/ms | 0.282 | 0.370 | 0.92 | 1711.9 |
| stage/render/queue | 0.281 | 0.400 | 11.35 | 1707.6 |
| stage/render/prepare_bind_groups | 0.274 | 0.480 | 2.87 | 1666.0 |
| controller/feed/ms | 0.258 | 0.329 | 0.74 | 1565.9 |
| stream/drive/ms | 0.244 | 0.322 | 19.97 | 1480.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.237 | 0.352 | 0.92 | 1440.3 |
| bigspace/low_precision/ms | 0.234 | 0.824 | 1.87 | 1419.3 |
| stage/main/fixed_main_loop | 0.215 | 0.371 | 0.97 | 1305.2 |
| far/select/ms | 0.197 | 0.265 | 0.58 | 1199.5 |
| stage/render/create_views | 0.196 | 0.250 | 0.90 | 1192.3 |
| stream/tick/ms | 0.188 | 0.259 | 0.74 | 1143.8 |
| stage/render/prepare_batch_phases | 0.186 | 0.265 | 0.50 | 1128.8 |
| bigspace/high_precision/ms | 0.176 | 0.810 | 1.74 | 1067.9 |
| anim/evaluate/ms | 0.168 | 0.316 | 2.14 | 1023.2 |
| stage/render/extract_commands | 0.168 | 0.199 | 0.49 | 1018.4 |
| stage/render/prepare_flush | 0.164 | 0.329 | 0.87 | 995.2 |
| anim/evaluate_cpu/ms | 0.154 | 0.186 | 0.76 | 932.7 |
| stage/render/prepare_views | 0.138 | 0.174 | 0.57 | 839.6 |
| controller/move/ms | 0.135 | 0.312 | 0.57 | 822.9 |
| stage/main/last | 0.115 | 0.158 | 1.60 | 699.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.102 | 0.170 | 0.83 | 617.3 |
| render/acquire/ms | 0.095 | 0.122 | 0.51 | 575.8 |
| stage/render/prepare_collect_phase_buffers | 0.064 | 0.136 | 0.54 | 389.0 |
| pipelines/compile_ms | 0.050 | 0.000 | 133.33 | 302.3 |
| pipelines/front_ms | 0.048 | 0.000 | 128.70 | 290.2 |
| physics/step/ms | 0.046 | 0.088 | 0.44 | 282.1 |
| stage/render/phase_sort | 0.034 | 0.057 | 0.27 | 205.1 |
| physics/insert/ms | 0.033 | 0.363 | 0.81 | 200.3 |
| stage/render/cleanup | 0.032 | 0.094 | 0.98 | 193.1 |
| encode/resolve_diagnostics/sum | 0.030 | 0.040 | 0.66 | 183.6 |
| weather/map/ms | 0.025 | 0.033 | 0.20 | 151.9 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.28 | 86.1 |
| anim/apply/ms | 0.011 | 0.020 | 0.42 | 68.9 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.47 | 66.8 |
| controller/avoid/ms | 0.011 | 0.014 | 0.35 | 64.5 |
| weather/eval/ms | 0.010 | 0.014 | 0.10 | 61.4 |
| script/update/ms | 0.007 | 0.009 | 0.17 | 41.1 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 31.2 |
| stream/despawn/ms | 0.005 | 0.000 | 1.00 | 30.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.09 | 28.6 |
| script/ui/ms | 0.004 | 0.006 | 0.09 | 27.2 |
| controller/write/ms | 0.003 | 0.005 | 0.15 | 21.1 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 14.2 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.64 | 12.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 6.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.11 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.10 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 20 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 15 |
| crowd/c0/joints/count | count | 1306 | 2262 | 707 |
| crowd/c0/meshes/count | count | 355 | 646 | 189 |
| crowd/c1/count | count | 145 | 156 | 145 |
| crowd/c1/joints/count | count | 5531 | 6023 | 5457 |
| crowd/c1/meshes/count | count | 1172 | 1297 | 1144 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4614 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.42 | 1.45 | 0.40 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.28 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.47 | 0.01 |
| encode/finish_critical | ms | 0.94 | 2.56 | 1.15 |
| encode/pass:early prepass/max | ms | 0.73 | 1.75 | 0.79 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 1.57 | 1.07 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.83 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.92 | 0.27 |
| encode/pass:shadow_directional_light/max | ms | 0.78 | 2.56 | 1.15 |
| encode/pass:shadow_point_light/max | ms | 0.74 | 1.61 | 0.97 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.66 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.26 | 42.71 | 8.50 |
| gpu/timed/ms | ms | 7.86 | 11.32 | 8.19 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 824 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 824 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1280 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3436 | 5140 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1280 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3436 | 5140 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 13 | 202 | 29 |
| phases/shadow/bins | count | 5033 | 8795 | 2421 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6372 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 203 | 252 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 187 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 12 | 561 | 58 |
| preprocess/bind_groups_created | count | 12 | 561 | 58 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 128 | 228 | 123 |
| process/cpu_usage | % | 26.91 | 40.38 | 23.92 |
| process/mem_usage | GiB | 5.47 | 5.64 | 5.64 |
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
| skin/blocks_changed/count | count | 13 | 123 | 15 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 369.9 k | 807.4 k | 489.7 k |
| skin/joints_written/count | count | 5780 | 12.6 k | 7652 |
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
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.13 | 44.36 | 32.62 |
| system/mem_usage | % | 66.97 | 67.42 | 67.39 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 62 | 249 | 79 |
| views/all/bins | count | 7614 | 11.7 k | 5919 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 248 | 299 | 75 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 10.7 k | 34.9 k | 14.3 k |
| views/cascade/batch_sets | count | 9 | 51 | 29 |
| views/cascade/bins | count | 2809 | 6184 | 2421 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8918 | 23.5 k | 6372 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 409 | 23.2 k | 6372 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2148 | 2751 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2402 | 3046 | 0 |
| views/lamp_face/pipelines | count | 168 | 209 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2933 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2858 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8524 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8682 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 51.48 | 235 | 312.7 k |
| anim/events/count | count | 0.09 | 3 | 536 |
| assets/Image/added | count | 0.10 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 9 | 25 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.56 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 60 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1245 | 4675 |
| encode//count | count | 32.17 | 220 | 195.4 k |
| encode/bind_walk/init_textures | count | 11.8 k | 114.7 k | 71.37 M |
| encode/bind_walk/init_textures_skipped | count | 63.8 k | 136.0 k | 387.70 M |
| encode/bind_walk/inits | count | 451 | 1125 | 2.74 M |
| encode/bind_walk/inits_skipped | count | 108 | 160 | 653.6 k |
| encode/bind_walk/merge_textures | count | 11.7 k | 114.6 k | 70.93 M |
| encode/bind_walk/merge_textures_skipped | count | 5363 | 6012 | 32.58 M |
| encode/bind_walk/merges | count | 96.56 | 334 | 586.5 k |
| encode/bind_walk/merges_skipped | count | 84.04 | 133 | 510.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6074 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6074 |
| encode/pass:early prepass/count | count | 1 | 1 | 6074 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6074 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6074 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6074 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6074 |
| encode/pass:shadow_point_light/count | count | 1.05 | 48 | 6406 |
| encode/queue/command_buffers | count | 43.24 | 278 | 262.6 k |
| encode/queue/submits | count | 2.02 | 4 | 12.2 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.99 M | 11.42 G |
| encode/queue/write_buffer/calls | count | 251 | 838 | 1.53 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.6 k | 3.53 M | 1.08 G |
| encode/queue/write_buffer_with/calls | count | 10.99 | 465 | 66.8 k |
| encode/queue/write_texture/bytes | bytes | 11.9 k | 8.44 M | 72.50 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 565 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6074 |
| physics/inserts | count | 0.37 | 108 | 2256 |
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
| stream/spawn/items | count | 0.06 | 18 | 378 |
| stream/upload/bytes | bytes | 5573 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1222 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.3 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2575.1 | main/busy/ms 2542, stage/main/post_update 2429, stage/main/update 105, surface/advance_cpu_ms 72.26, stage/main/pre_update 4.56, stage/main/last 1.07, surface/cell_upload_ms 0.77, stream/drive/ms 0.56, stage/main/first 0.52, stage/main/fixed_main_loop 0.46, stream/tick/ms 0.17, weather/map/ms 0.13 |
| warm-up | 3.41 | 830.6 | main/busy/ms 2714, stage/main/post_update 2454, render/thread/ms 797, render/busy/ms 627, stage/render/prepare_assets 579, stage/main/update 246, stage/render/prepare_views 164, render/acquire/ms 163, surface/advance_cpu_ms 72.26, stage/render/create_views 27.97, stage/render/render 14.96, render/graph/ms 8.14 |
| warm-up | 3.52 | 110.9 | render/thread/ms 890, render/busy/ms 667, stage/render/prepare_assets 603, main/busy/ms 194, stage/render/prepare_views 164, render/acquire/ms 164, stage/main/update 147, pipelines/compile_ms 86.38, stage/render/render 74.65, render/present/ms 59.47, pipelines/front_ms 47.70, pipelines/driver_ms 38.68 |
| warm-up | 3.55 | 28.7 | pipelines/compile_ms 356, pipelines/front_ms 290, render/thread/ms 121, main/busy/ms 89.33, stage/render/render 80.26, pipelines/driver_ms 66.04, render/busy/ms 62.00, render/present/ms 57.90, stage/main/post_update 38.37, stage/main/update 31.69, stage/render/prepare_assets 25.27, pipelines/wait_ms 25.03 |
| warm-up | 3.70 | 151.9 | pipelines/wait_ms 4516, pipelines/compile_ms 1627, pipelines/front_ms 1505, main/busy/ms 169, render/thread/ms 132, render/busy/ms 125, pipelines/driver_ms 122, stage/main/update 103, stage/main/post_update 49.09, stage/render/render 47.41, render/graph/ms 29.56, stage/render/prepare_resources 22.22 |
| warm-up | 3.76 | 60.9 | pipelines/wait_ms 5088, pipelines/compile_ms 1881, pipelines/front_ms 1742, render/thread/ms 162, render/busy/ms 159, pipelines/driver_ms 139, main/busy/ms 130, stage/main/update 81.36, stage/render/prepare_assets 68.38, stage/main/post_update 34.35, stage/render/render 32.64, stage/render/prepare_resources 21.86 |
| warm-up | 3.79 | 35.2 | pipelines/compile_ms 899, pipelines/front_ms 835, pipelines/wait_ms 745, render/thread/ms 91.90, render/busy/ms 89.51, pipelines/driver_ms 64.35, stage/render/prepare_assets 49.53, main/busy/ms 44.90, encode//sum 31.48, stage/render/render 26.42, render/graph/ms 24.12, stage/main/post_update 17.28 |
| warm-up | 3.83 | 36.8 | pipelines/compile_ms 1003, pipelines/front_ms 962, pipelines/wait_ms 231, render/thread/ms 66.18, render/busy/ms 64.16, pipelines/driver_ms 41.28, main/busy/ms 38.68, encode//sum 30.51, stage/render/prepare_assets 26.36, stage/render/render 24.86, render/graph/ms 22.60, stage/render/graph_submit 14.82 |
| warm-up | 3.90 | 49.9 | pipelines/compile_ms 1026, pipelines/front_ms 982, render/thread/ms 71.64, render/busy/ms 69.66, main/busy/ms 65.08, pipelines/driver_ms 44.53, stage/render/prepare_flush 26.08, stage/main/post_update 21.57, stage/render/prepare_assets 16.27, stage/main/update 14.20, stage/main/pre_update 12.63, stage/render/render 11.72 |
| warm-up | 4.07 | 105.3 | pipelines/compile_ms 1246, pipelines/front_ms 1195, main/busy/ms 63.03, render/thread/ms 61.19, render/busy/ms 59.61, pipelines/driver_ms 51.06, stage/main/update 49.58, stream/drive/ms 43.75, stage/render/prepare_assets 40.04, stage/render/render 13.87, render/graph/ms 11.89, pipelines/wait_ms 10.53 |
| warm-up | 4.11 | 33.1 | pipelines/compile_ms 83.52, pipelines/front_ms 74.94, render/thread/ms 35.97, render/busy/ms 34.52, main/busy/ms 28.01, stage/render/prepare_assets 15.95, stage/main/update 15.10, stage/render/render 13.41, render/graph/ms 11.48, stream/drive/ms 10.43, stage/render/graph_submit 8.67, pipelines/driver_ms 8.58 |
| warm-up | 4.22 | 79.7 | pipelines/compile_ms 533, pipelines/front_ms 516, render/thread/ms 71.85, render/busy/ms 69.83, main/busy/ms 58.93, stage/render/prepare_assets 45.78, stage/main/update 36.69, stream/drive/ms 29.96, stage/render/render 18.36, pipelines/driver_ms 16.87, render/graph/ms 15.85, stage/main/post_update 10.93 |
| warm-up | 4.26 | 39.3 | pipelines/compile_ms 517, pipelines/front_ms 501, render/thread/ms 72.32, render/busy/ms 70.43, stage/render/prepare_assets 51.97, main/busy/ms 32.64, pipelines/driver_ms 16.27, stage/main/update 13.48, stage/render/render 12.61, render/graph/ms 10.19, stage/main/post_update 8.20, stage/render/graph_submit 7.63 |
| warm-up | 4.33 | 74.2 | pipelines/compile_ms 1208, pipelines/front_ms 1168, render/thread/ms 73.42, render/busy/ms 71.36, main/busy/ms 64.10, stage/render/prepare_assets 53.22, pipelines/driver_ms 39.64, stage/main/update 38.83, stage/main/post_update 14.31, stage/render/render 10.90, render/graph/ms 8.40, pipelines/wait_ms 5.91 |
| warm-up | 4.40 | 69.8 | pipelines/compile_ms 1660, pipelines/front_ms 1608, render/thread/ms 96.89, render/busy/ms 94.87, stage/render/prepare_assets 70.63, main/busy/ms 68.27, pipelines/driver_ms 51.10, stage/main/update 42.21, stage/main/post_update 15.73, stage/render/render 9.82, pipelines/wait_ms 7.93, render/graph/ms 7.44 |
| warm-up | 4.53 | 124.2 | pipelines/compile_ms 1635, pipelines/front_ms 1590, render/thread/ms 165, render/busy/ms 163, pipelines/wait_ms 114, main/busy/ms 91.62, stage/render/prepare_assets 58.07, stage/main/post_update 57.17, pipelines/driver_ms 44.63, stage/render/render 35.19, render/graph/ms 32.66, stage/render/prepare_resources 26.00 |
| warm-up | 4.62 | 94.5 | pipelines/compile_ms 2909, pipelines/front_ms 2829, pipelines/wait_ms 951, render/thread/ms 199, render/busy/ms 197, main/busy/ms 93.56, pipelines/driver_ms 79.83, stage/main/post_update 56.45, stage/render/prepare_assets 55.76, stage/render/prepare_resources 46.56, stage/render/render 37.52, render/graph/ms 35.18 |
| warm-up | 4.70 | 78.1 | pipelines/compile_ms 2327, pipelines/front_ms 2260, pipelines/wait_ms 1980, render/thread/ms 156, render/busy/ms 154, pipelines/driver_ms 67.61, encode//sum 55.71, main/busy/ms 41.92, stage/render/prepare_resources 40.16, stage/render/prepare_assets 39.27, stage/render/render 30.98, render/graph/ms 28.61 |
| warm-up | 4.77 | 67.9 | pipelines/compile_ms 1491, pipelines/front_ms 1444, pipelines/wait_ms 1405, render/thread/ms 133, render/busy/ms 130, encode//sum 76.84, main/busy/ms 52.28, pipelines/driver_ms 46.66, stage/render/render 40.94, render/graph/ms 38.23, stage/render/graph_submit 27.74, stage/render/prepare_resources 23.01 |
| warm-up | 4.81 | 40.7 | pipelines/compile_ms 1610, pipelines/front_ms 1558, pipelines/wait_ms 733, main/busy/ms 105, render/thread/ms 82.50, render/busy/ms 80.56, stage/main/update 71.03, stream/drive/ms 60.96, pipelines/driver_ms 52.29, encode//sum 31.46, stage/render/render 23.58, stage/render/prepare_assets 21.95 |
| warm-up | 4.90 | 97.7 | pipelines/compile_ms 1872, pipelines/front_ms 1814, pipelines/wait_ms 634, main/busy/ms 131, stage/main/update 104, render/thread/ms 65.68, render/busy/ms 63.65, stream/drive/ms 62.45, pipelines/driver_ms 57.69, stage/render/render 29.64, render/graph/ms 26.33, stage/render/prepare_assets 16.06 |
| warm-up | 4.99 | 88.9 | pipelines/compile_ms 1712, pipelines/front_ms 1665, pipelines/wait_ms 174, main/busy/ms 141, stage/main/update 116, render/thread/ms 91.37, render/busy/ms 89.21, stream/drive/ms 52.87, pipelines/driver_ms 47.50, stage/render/render 35.52, render/graph/ms 32.23, stage/render/prepare_assets 27.09 |
| warm-up | 5.11 | 114.2 | pipelines/compile_ms 1999, pipelines/front_ms 1950, main/busy/ms 115, render/thread/ms 107, render/busy/ms 106, stage/main/update 88.53, stage/render/prepare_assets 60.77, stream/drive/ms 51.63, pipelines/driver_ms 48.91, stage/render/render 20.73, render/graph/ms 18.09, stage/main/post_update 15.72 |
| warm-up | 5.19 | 81.7 | pipelines/compile_ms 2017, pipelines/front_ms 1900, render/thread/ms 132, render/busy/ms 121, pipelines/driver_ms 117, main/busy/ms 87.94, stage/main/update 60.12, stage/render/render 59.77, stage/render/prepare_assets 51.96, encode/submit 48.66, render/graph/ms 48.07, stage/render/graph_submit 43.43 |
| warm-up | 5.24 | 45.8 | pipelines/compile_ms 531, pipelines/front_ms 449, render/thread/ms 103, render/busy/ms 92.69, pipelines/driver_ms 81.88, main/busy/ms 77.96, stage/main/update 57.65, stage/render/render 56.14, encode/submit 47.77, render/graph/ms 44.76, stage/render/graph_submit 41.48, stream/drive/ms 38.65 |
| warm-up | 5.26 | 29.3 | main/busy/ms 70.18, render/thread/ms 54.20, stage/main/update 52.78, render/busy/ms 52.66, stream/drive/ms 34.90, stage/render/prepare_assets 24.86, stage/render/render 14.07, render/graph/ms 11.96, stage/main/post_update 9.34, stage/render/graph_submit 9.27, encode//sum 8.06, encode/submit 6.49 |
| warm-up | 5.31 | 49.5 | pipelines/compile_ms 724, pipelines/front_ms 703, main/busy/ms 59.98, stage/main/update 45.59, render/thread/ms 43.68, render/busy/ms 42.07, stream/drive/ms 34.70, pipelines/driver_ms 21.09, stage/render/prepare_assets 14.90, stage/render/render 13.10, render/graph/ms 10.90, stage/main/post_update 8.22 |
| warm-up | 5.44 | 32.2 | pipelines/compile_ms 598, pipelines/front_ms 581, render/thread/ms 47.63, render/busy/ms 45.98, main/busy/ms 40.83, pipelines/driver_ms 17.40, stage/main/update 17.21, stage/main/post_update 15.18, stage/render/render 13.22, stage/render/prepare_assets 12.41, render/graph/ms 10.81, encode//sum 9.09 |
| warm-up | 5.47 | 33.7 | pipelines/compile_ms 718, pipelines/front_ms 697, render/thread/ms 53.58, render/busy/ms 52.21, main/busy/ms 41.03, stage/render/prepare_assets 25.89, stage/main/update 22.34, pipelines/driver_ms 20.97, stage/render/render 11.34, stage/main/post_update 10.53, render/graph/ms 9.20, encode//sum 8.33 |
| warm-up | 5.51 | 38.3 | pipelines/compile_ms 611, pipelines/front_ms 593, render/thread/ms 58.81, render/busy/ms 57.46, stage/render/prepare_assets 38.29, main/busy/ms 37.44, stage/main/update 21.52, pipelines/driver_ms 18.33, stage/render/render 10.95, stage/main/post_update 10.53, render/graph/ms 9.03, encode//sum 7.34 |
| warm-up | 5.61 | 35.8 | render/thread/ms 45.73, render/busy/ms 43.92, main/busy/ms 32.96, stage/render/prepare_assets 21.73, pipelines/compile_ms 17.37, pipelines/front_ms 15.69, stage/main/update 14.88, stage/render/render 13.02, stage/main/post_update 11.45, render/graph/ms 10.71, encode//sum 8.53, stage/render/graph_submit 7.95 |
| warm-up | 5.70 | 28.6 | render/thread/ms 47.59, render/busy/ms 46.27, main/busy/ms 30.49, stage/render/prepare_assets 24.28, stage/main/post_update 13.69, stage/main/update 12.16, stage/render/render 9.23, encode//sum 7.87, render/graph/ms 7.41, stage/render/graph_submit 5.01, stage/render/specialize 3.12, encode/submit 2.86 |
| warm-up | 5.73 | 29.6 | render/thread/ms 51.78, render/busy/ms 50.39, main/busy/ms 27.32, stage/render/prepare_assets 27.03, stage/main/post_update 12.72, stage/main/update 9.84, stage/render/render 9.24, encode//sum 8.20, render/graph/ms 7.39, stage/render/graph_submit 4.98, stage/render/specialize 3.40, encode/submit 2.88 |
| warm-up | 5.76 | 29.9 | render/thread/ms 50.29, render/busy/ms 48.96, stage/render/prepare_assets 28.11, main/busy/ms 24.92, stage/render/render 11.94, stage/main/post_update 10.44, render/graph/ms 10.00, stage/main/update 9.18, encode//sum 8.40, stage/render/graph_submit 7.23, encode/submit 3.73, encode/finish_wall 3.51 |
| warm-up | 5.81 | 42.6 | pipelines/compile_ms 280, pipelines/front_ms 262, render/thread/ms 64.02, render/busy/ms 62.54, stage/render/prepare_assets 41.46, main/busy/ms 40.47, pipelines/driver_ms 17.50, stage/main/update 15.38, stage/render/render 13.47, stage/main/post_update 11.93, render/graph/ms 11.39, stage/main/pre_update 8.76 |
| warm-up | 6.36 | 30.5 | render/thread/ms 41.03, render/busy/ms 40.15, main/busy/ms 29.57, stage/render/prepare_assets 19.03, stage/main/update 12.12, stage/main/post_update 10.87, stage/render/render 7.97, encode//sum 6.98, render/graph/ms 6.61, stage/render/graph_submit 4.58, stream/drive/ms 4.24, stage/render/specialize 3.30 |
| warm-up | 6.57 | 29.5 | render/thread/ms 46.68, render/busy/ms 45.01, main/busy/ms 41.07, stage/main/post_update 16.88, stage/main/update 16.86, stage/render/render 13.47, render/graph/ms 11.32, stage/render/prepare_assets 9.54, encode//sum 9.27, stage/render/graph_submit 7.89, stream/drive/ms 5.88, stage/render/specialize 4.90 |
| warm-up | 6.72 | 28.5 | main/busy/ms 40.52, render/thread/ms 37.46, render/busy/ms 35.68, stage/main/post_update 17.88, stage/main/update 16.69, stage/render/render 15.95, render/graph/ms 13.39, encode//sum 10.28, stage/render/graph_submit 9.60, stage/render/prepare_assets 9.10, pipelines/compile_ms 8.05, pipelines/front_ms 7.48 |
| warm-up | 7.17 | 46.6 | render/thread/ms 50.29, render/busy/ms 49.03, main/busy/ms 33.07, stage/main/update 14.85, stage/render/prepare_assets 12.25, stage/render/render 11.74, stage/main/post_update 10.05, render/graph/ms 9.93, stage/render/specialize 9.49, encode//sum 8.48, stage/render/queue 7.50, stage/render/graph_submit 7.31 |
| warm-up | 8.46 | 31.1 | render/thread/ms 37.11, render/busy/ms 36.02, main/busy/ms 17.99, stage/render/render 10.64, stage/main/post_update 10.61, stage/render/specialize 9.66, render/graph/ms 9.20, encode//sum 6.87, stage/render/queue 6.83, stage/render/graph_submit 6.57, stage/main/update 4.73, stage/render/prepare_meshes 3.61 |
| measure | 0.39 | 40.0 | render/thread/ms 43.94, render/busy/ms 42.87, encode//sum 19.85, main/busy/ms 18.54, stage/render/render 15.87, render/graph/ms 14.21, stage/main/post_update 11.49, stage/render/graph_submit 10.30, encode/pass:shadow_point_light/sum 10.11, stage/render/specialize 8.89, stage/render/queue 6.81, encode/submit 5.62 |
| measure | 1.68 | 30.6 | render/thread/ms 35.96, render/busy/ms 34.99, main/busy/ms 16.45, stage/main/post_update 10.47, stage/render/render 10.08, stage/render/specialize 9.15, render/graph/ms 8.68, encode//sum 7.25, stage/render/graph_submit 6.66, stage/render/queue 5.70, stage/render/prepare_meshes 4.59, encode/submit 3.69 |
| measure | 3.09 | 30.8 | render/thread/ms 39.50, render/busy/ms 38.50, main/busy/ms 21.46, stage/main/post_update 13.42, stage/render/render 12.41, render/graph/ms 10.88, stage/render/specialize 9.34, encode//sum 8.41, stage/render/graph_submit 8.06, stage/render/queue 7.12, encode/submit 4.81, stage/main/update 4.49 |
| measure | 5.87 | 29.3 | render/thread/ms 34.39, render/busy/ms 33.37, main/busy/ms 19.18, stage/main/post_update 12.65, stage/render/render 9.76, stage/render/specialize 9.45, render/graph/ms 8.34, encode//sum 7.49, stage/render/graph_submit 6.43, stage/render/queue 6.14, stage/main/update 3.82, encode/submit 3.65 |
| measure | 8.47 | 29.1 | render/thread/ms 35.85, render/busy/ms 34.49, main/busy/ms 18.24, stage/main/post_update 11.67, stage/render/render 11.03, stage/render/specialize 9.35, render/graph/ms 9.29, encode//sum 8.21, stage/render/graph_submit 7.07, stage/render/queue 6.38, encode/submit 4.38, stage/main/update 4.22 |
| measure | 15.72 | 31.4 | render/thread/ms 39.71, render/busy/ms 38.62, main/busy/ms 19.41, stage/render/render 12.58, stage/main/post_update 11.60, render/graph/ms 11.06, stage/render/specialize 10.59, encode//sum 9.65, stage/render/graph_submit 8.48, stage/render/queue 7.39, encode/submit 5.25, stage/main/update 3.97 |
| measure | 17.06 | 28.4 | render/thread/ms 35.23, render/busy/ms 34.09, main/busy/ms 18.15, stage/main/post_update 11.73, stage/render/render 10.67, stage/render/specialize 9.63, render/graph/ms 9.17, encode//sum 8.94, stage/render/graph_submit 6.71, stage/render/queue 6.16, encode/submit 4.25, stage/main/update 3.96 |
| measure | 18.18 | 31.8 | main/busy/ms 37.78, stage/main/update 23.80, render/thread/ms 20.54, stream/drive/ms 20.20, render/busy/ms 19.59, stage/render/render 11.37, stage/main/post_update 11.33, encode//sum 10.29, render/graph/ms 9.87, stage/render/graph_submit 7.27, encode/submit 4.39, encode/pass:shadow_point_light/sum 3.50 |
| measure | 23.02 | 34.0 | render/thread/ms 40.71, render/busy/ms 39.49, main/busy/ms 24.18, stage/main/post_update 12.35, stage/render/render 11.73, stage/render/specialize 11.38, render/graph/ms 10.15, encode//sum 9.82, stage/render/graph_submit 7.92, stage/render/queue 7.43, stage/main/update 7.21, encode/submit 4.50 |
| measure | 24.49 | 31.0 | render/thread/ms 37.69, render/busy/ms 36.64, main/busy/ms 23.48, stage/main/post_update 12.83, stage/render/render 11.58, stage/render/specialize 10.14, render/graph/ms 10.10, encode//sum 9.09, stage/render/graph_submit 7.80, stage/render/queue 6.49, encode/submit 4.73, stage/main/update 4.41 |
| measure | 27.29 | 34.3 | render/thread/ms 45.09, render/busy/ms 44.04, main/busy/ms 25.50, stage/render/render 15.15, encode//sum 14.19, render/graph/ms 13.69, stage/main/post_update 13.12, stage/render/specialize 11.47, stage/render/graph_submit 10.68, stage/render/queue 7.84, encode/pass:shadow_point_light/sum 6.95, encode/submit 6.76 |
| measure | 28.75 | 30.7 | render/thread/ms 38.90, render/busy/ms 37.76, main/busy/ms 19.11, stage/render/render 12.81, stage/main/post_update 12.59, render/graph/ms 11.25, encode//sum 10.08, stage/render/specialize 9.52, stage/render/graph_submit 8.44, stage/render/queue 6.78, encode/pass:shadow_point_light/sum 5.42, encode/submit 5.05 |
| measure | 30.49 | 31.7 | render/thread/ms 38.96, render/busy/ms 37.71, main/busy/ms 22.19, stage/main/post_update 13.13, stage/render/render 11.60, stage/render/specialize 10.23, render/graph/ms 9.91, encode//sum 8.93, stage/render/graph_submit 7.74, stage/render/queue 7.00, stage/main/update 6.18, encode/submit 4.31 |
| measure | 31.90 | 30.1 | render/thread/ms 36.41, render/busy/ms 35.47, main/busy/ms 19.60, stage/main/post_update 12.17, stage/render/render 10.55, stage/render/specialize 9.84, render/graph/ms 9.19, encode//sum 8.25, stage/render/graph_submit 6.99, stage/render/queue 6.44, stage/main/update 4.36, encode/submit 4.05 |
| measure | 35.35 | 33.6 | render/thread/ms 39.59, render/busy/ms 38.55, main/busy/ms 19.96, stage/main/post_update 12.34, stage/render/render 11.15, stage/render/specialize 10.74, render/graph/ms 9.65, encode//sum 8.50, stage/render/graph_submit 7.67, stage/render/queue 7.28, stage/main/update 4.46, encode/submit 4.26 |
| measure | 36.67 | 29.0 | render/thread/ms 36.24, render/busy/ms 35.26, main/busy/ms 19.80, stage/main/post_update 12.64, stage/render/render 10.27, stage/render/specialize 9.85, render/graph/ms 8.92, encode//sum 8.03, stage/render/queue 6.94, stage/render/graph_submit 6.92, stage/main/update 4.34, encode/submit 3.66 |
| measure | 37.48 | 29.8 | render/thread/ms 37.07, render/busy/ms 35.94, main/busy/ms 18.87, stage/main/post_update 11.55, stage/render/render 10.83, stage/render/specialize 9.99, render/graph/ms 9.33, encode//sum 9.12, stage/render/graph_submit 7.08, stage/render/queue 6.48, stage/main/update 4.63, encode/submit 3.86 |
| measure | 40.80 | 34.6 | render/thread/ms 38.42, render/busy/ms 37.36, main/busy/ms 19.08, stage/main/post_update 10.88, stage/render/render 10.46, stage/render/specialize 10.08, encode//sum 9.98, render/graph/ms 8.94, stage/render/graph_submit 7.14, stage/render/queue 6.80, stage/main/update 4.69, encode/submit 4.21 |
| measure | 43.21 | 30.2 | render/thread/ms 37.51, render/busy/ms 36.39, main/busy/ms 16.25, stage/render/render 11.00, stage/render/specialize 9.91, stage/main/post_update 9.81, render/graph/ms 9.58, encode//sum 8.06, stage/render/graph_submit 7.75, stage/render/queue 6.43, encode/submit 4.09, stage/main/update 3.90 |
| measure | 45.67 | 33.5 | render/thread/ms 39.95, render/busy/ms 38.83, main/busy/ms 18.44, stage/render/render 11.05, stage/render/specialize 10.65, stage/main/post_update 10.57, render/graph/ms 9.60, encode//sum 9.45, stage/render/graph_submit 7.64, stage/render/queue 6.93, stage/render/prepare_meshes 4.74, stage/main/update 4.40 |
| measure | 56.44 | 29.5 | render/thread/ms 36.89, render/busy/ms 35.71, main/busy/ms 14.75, stage/render/render 11.43, render/graph/ms 10.00, stage/render/specialize 9.48, stage/main/post_update 8.58, stage/render/graph_submit 7.86, encode//sum 7.50, stage/render/queue 6.11, encode/submit 4.66, stage/main/update 4.09 |
| measure | 59.03 | 48.5 | render/thread/ms 55.59, render/busy/ms 54.20, main/busy/ms 23.96, stage/render/render 15.72, render/graph/ms 13.93, stage/main/post_update 13.60, encode//sum 12.59, stage/render/specialize 12.17, stage/render/queue 11.49, stage/render/graph_submit 10.36, stage/main/update 7.03, stage/render/prepare_meshes 5.69 |
| measure | 59.09 | 38.4 | render/thread/ms 37.64, render/busy/ms 36.58, stage/render/prepare_assets 20.32, main/busy/ms 17.94, stage/render/render 9.42, encode//sum 9.22, stage/main/post_update 9.13, render/graph/ms 8.02, stage/render/graph_submit 6.05, stage/main/update 4.55, encode/submit 3.91, encode/finish_wall 2.18 |
| measure | 62.58 | 32.2 | render/thread/ms 38.51, render/busy/ms 37.50, main/busy/ms 19.26, stage/render/render 10.51, stage/render/specialize 10.49, stage/main/post_update 10.44, render/graph/ms 9.24, encode//sum 8.86, stage/render/graph_submit 7.16, stage/render/queue 6.36, stage/main/update 6.16, stage/render/prepare_meshes 4.79 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=base-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **78%** (min 47%) · 2620 MHz (min 2610) · memory 8101 MHz · 81.0 W · 78 °C (max 84) · VRAM 6759 MiB · throttled 100% of samples

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
