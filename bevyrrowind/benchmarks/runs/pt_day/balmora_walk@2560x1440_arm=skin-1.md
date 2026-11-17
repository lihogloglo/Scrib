# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.skin.skip_unchanged=on (command line: --set); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6083 frames over 65.6 s · **median 10.59 ms (94 fps)** · mean 10.79 · p95 13.31 · p99 16.13 (1% low 62 fps) · worst 41.45 ms · 68 spikes over 1.5× median · 57.6 stutters/min (work 51.2/min)
- warm-up: 323 frames over 9.0 s, worst 2543.6 ms
- hitches (> 26.5 ms): **32** measured, 43 in warm-up
- timed GPU passes: 6.59 ms of a 10.79 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.59 ms, limit 11 ms
- **FAIL** frame_ms: 63 stutters (> 1.5x moving median of 60) in 1.09 min = 57.58/min, limit 1/min
- **FAIL** worst frame 41.45 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.856 | 2.037 | 2.072 | 0.91 | 0.175 |
| main_opaque_pass_3d | 1.490 | 1.637 | 2.063 | 0.91 | 0.066 |
| early prepass | 0.451 | 0.496 | 0.731 | 0.91 | 0.086 |
| bloom | 0.287 | 0.315 | 0.321 | 0.91 | 0.062 |
| atmosphere_luts | 0.277 | 0.305 | 0.367 | 0.91 | 0.003 |
| water_velocity | 0.255 | 0.280 | 0.347 | 0.91 | 0.002 |
| water_sspr | 0.191 | 0.210 | 0.211 | 0.91 | 0.002 |
| ssao | 0.172 | 0.189 | 0.193 | 0.91 | 0.007 |
| auxiliary_views | 0.137 | 0.151 | 0.520 | 0.91 | 0.150 |
| atmos_froxel_inject | 0.133 | 0.146 | 0.148 | 0.91 | 0.001 |
| clouds_trace | 0.127 | 0.140 | 0.230 | 0.91 | 0.001 |
| water_fft | 0.119 | 0.131 | 0.138 | 0.91 | 0.003 |
| uniform_allocation | 0.119 | 0.131 | 0.183 | 0.91 | 0.025 |
| upscaling | 0.106 | 0.117 | 0.124 | 0.91 | 0.004 |
| bin_unpacking | 0.096 | 0.106 | 0.144 | 0.91 | 0.019 |
| clouds_composite | 0.096 | 0.106 | 0.109 | 0.91 | 0.002 |
| main_transmissive_pass_3d | 0.078 | 0.086 | 0.179 | 0.91 | 0.008 |
| tonemapping | 0.078 | 0.085 | 0.089 | 0.91 | 0.003 |
| clustering | 0.058 | 0.064 | 0.068 | 0.91 | 0.018 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.91 | 0.002 |
| lightprobe_irradiance_map | 0.045 | 0.412 | 0.417 | 0.11 | 0.001 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.91 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.91 | 0.001 |
| surface_trail | 0.036 | 0.056 | 0.057 | 0.63 | 0.002 |
| surface_exposure_far | 0.034 | 0.042 | 0.045 | 0.81 | 0.002 |
| early_mesh_preprocessing | 0.028 | 0.031 | 0.047 | 0.91 | 0.005 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.91 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.031 | 0.91 | 0.001 |
| lightprobe_radiance_map | 0.024 | 0.219 | 0.229 | 0.11 | 0.004 |
| water_snapshot | 0.022 | 0.025 | 0.028 | 0.91 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.024 | 0.91 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.91 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.91 | 0.122 |
| clouds_fog_light | 0.010 | 0.012 | 0.012 | 0.91 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.91 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.91 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.91 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.311 | 11.495 | 37.03 | 56638.2 |
| main/busy/ms | 8.955 | 10.670 | 25.77 | 54472.8 |
| render/busy/ms | 8.744 | 10.986 | 36.47 | 53191.2 |
| stage/main/post_update | 5.580 | 6.917 | 13.17 | 33942.8 |
| stage/render/render | 5.446 | 6.909 | 16.29 | 33129.5 |
| render/graph/ms | 4.690 | 6.149 | 14.85 | 28529.6 |
| encode//sum | 3.902 | 5.141 | 31.60 | 23738.3 |
| stage/render/graph_submit | 3.513 | 4.684 | 11.00 | 21368.1 |
| encode/submit | 2.270 | 3.186 | 6.01 | 13807.5 |
| stage/main/update | 2.113 | 2.530 | 19.35 | 12852.4 |
| encode/finish_wall | 1.331 | 1.920 | 5.05 | 8095.0 |
| stage/render/camera_driver | 1.124 | 1.563 | 4.59 | 6838.1 |
| encode/pass:shadow_directional_light/sum | 0.827 | 1.517 | 3.89 | 5032.1 |
| encode/pass:shadow_point_light/sum | 0.818 | 3.058 | 15.32 | 4976.0 |
| stage/render/prepare_write_phase_buffers | 0.756 | 1.095 | 2.55 | 4597.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.743 | 1.049 | 1.83 | 4516.7 |
| encode/pass:early prepass/sum | 0.725 | 1.046 | 2.26 | 4409.9 |
| stage/render/prepare_assets | 0.560 | 0.720 | 11.25 | 3403.9 |
| bigspace/total/ms | 0.523 | 1.280 | 2.37 | 3182.9 |
| render/present/ms | 0.472 | 1.242 | 2.33 | 2872.0 |
| stage/render/prepare_resources | 0.441 | 0.536 | 1.18 | 2680.4 |
| stage/render/prepare_meshes | 0.429 | 0.536 | 7.03 | 2609.4 |
| stage/main/first | 0.398 | 0.521 | 5.70 | 2422.6 |
| stage/main/pre_update | 0.359 | 0.514 | 2.52 | 2183.8 |
| stage/render/specialize | 0.318 | 0.408 | 12.21 | 1934.4 |
| terrain/update/ms | 0.309 | 0.398 | 2.08 | 1877.2 |
| stage/render/queue | 0.289 | 0.414 | 7.56 | 1755.0 |
| stage/render/prepare_bind_groups | 0.279 | 0.494 | 2.40 | 1697.5 |
| terrain/select/ms | 0.275 | 0.356 | 0.92 | 1671.1 |
| controller/feed/ms | 0.249 | 0.310 | 1.00 | 1515.1 |
| stream/drive/ms | 0.246 | 0.321 | 17.50 | 1497.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.237 | 0.351 | 1.04 | 1444.2 |
| stage/main/fixed_main_loop | 0.212 | 0.366 | 1.03 | 1287.4 |
| bigspace/low_precision/ms | 0.205 | 0.669 | 1.82 | 1250.1 |
| far/select/ms | 0.201 | 0.272 | 0.58 | 1223.6 |
| stage/render/create_views | 0.196 | 0.255 | 0.92 | 1192.3 |
| stream/tick/ms | 0.190 | 0.255 | 0.66 | 1154.6 |
| stage/render/prepare_batch_phases | 0.189 | 0.272 | 0.68 | 1146.7 |
| stage/render/extract_commands | 0.172 | 0.200 | 0.46 | 1044.8 |
| pipelines/compile_ms | 0.171 | 0.000 | 141.77 | 1041.9 |
| pipelines/front_ms | 0.166 | 0.000 | 138.98 | 1011.7 |
| stage/render/prepare_flush | 0.162 | 0.316 | 1.06 | 986.9 |
| anim/evaluate/ms | 0.157 | 0.265 | 2.00 | 953.6 |
| bigspace/high_precision/ms | 0.156 | 0.682 | 1.85 | 948.9 |
| anim/evaluate_cpu/ms | 0.148 | 0.187 | 0.81 | 897.9 |
| stage/render/prepare_views | 0.139 | 0.174 | 0.52 | 844.9 |
| stage/main/last | 0.116 | 0.164 | 1.61 | 707.4 |
| controller/move/ms | 0.115 | 0.214 | 0.49 | 699.4 |
| encode/pass:main_transmissive_pass_3d/sum | 0.098 | 0.160 | 0.83 | 598.7 |
| render/acquire/ms | 0.095 | 0.121 | 0.48 | 575.0 |
| stage/render/prepare_collect_phase_buffers | 0.064 | 0.137 | 0.46 | 390.9 |
| physics/step/ms | 0.045 | 0.085 | 0.52 | 274.4 |
| stage/render/phase_sort | 0.031 | 0.055 | 0.24 | 189.4 |
| physics/insert/ms | 0.031 | 0.341 | 1.01 | 187.3 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.55 | 183.4 |
| stage/render/cleanup | 0.030 | 0.081 | 0.75 | 182.3 |
| weather/map/ms | 0.025 | 0.033 | 0.30 | 153.4 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.52 | 88.5 |
| anim/apply/ms | 0.011 | 0.021 | 0.45 | 69.5 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.48 | 68.0 |
| weather/eval/ms | 0.010 | 0.014 | 0.11 | 63.0 |
| controller/avoid/ms | 0.010 | 0.013 | 0.17 | 61.9 |
| script/update/ms | 0.007 | 0.009 | 0.18 | 44.0 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 32.6 |
| stream/despawn/ms | 0.005 | 0.000 | 1.19 | 31.6 |
| pipelines/driver_ms | 0.005 | 0.000 | 5.02 | 30.3 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 30.2 |
| script/ui/ms | 0.004 | 0.006 | 0.42 | 26.0 |
| controller/write/ms | 0.003 | 0.004 | 0.19 | 19.9 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 14.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.40 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 4.8 |
| stage/render/prepare | 0.001 | 0.003 | 0.18 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.11 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 28 | 17 |
| controller/movers/count | count | 19 | 21 | 17 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1277 | 2262 | 528 |
| crowd/c0/meshes/count | count | 352 | 646 | 135 |
| crowd/c1/count | count | 146 | 157 | 149 |
| crowd/c1/joints/count | count | 5531 | 6046 | 5668 |
| crowd/c1/meshes/count | count | 1177 | 1322 | 1206 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 558 | 6901 | 478 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.41 | 2.48 | 0.33 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.52 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.48 | 0.01 |
| encode/finish_critical | ms | 0.94 | 3.89 | 0.94 |
| encode/pass:early prepass/max | ms | 0.74 | 2.26 | 0.94 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 1.83 | 0.86 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.83 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 1.04 | 0.30 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 3.89 | 0.85 |
| encode/pass:shadow_point_light/max | ms | 0.77 | 1.36 | 0.79 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.55 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.23 | 36.47 | 8.23 |
| gpu/timed/ms | ms | 7.92 | 11.73 | 8.23 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 805 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 805 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1290 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3394 | 5117 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1290 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3394 | 5117 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 198 | 4 |
| phases/shadow/bins | count | 5034 | 8769 | 2383 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.3 k | 26.4 k | 6340 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 202 | 256 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 12 | 557 | 9 |
| preprocess/bind_groups_created | count | 12 | 557 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 208 | 120 |
| process/cpu_usage | % | 26.92 | 41.35 | 19.23 |
| process/mem_usage | GiB | 5.43 | 5.60 | 5.60 |
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
| skin/blocks_changed/count | count | 13 | 130 | 11 |
| skin/blocks_skipped/count | count | 187 | 241 | 227 |
| skin/blocks_walked/count | count | 12 | 17 | 11 |
| skin/joint_bytes_written/bytes | bytes | 37.9 k | 808.4 k | 31.5 k |
| skin/joints_written/count | count | 592 | 12.6 k | 492 |
| skin/upload_writes/count | count | 7 | 28 | 8 |
| started_load_count |  | 4918 | 5011 | 5011 |
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
| stream/fetch/queue | count | 0 | 2 | 0 |
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
| system/cpu_usage | % | 32.07 | 50.76 | 29.69 |
| system/mem_usage | % | 54.37 | 54.95 | 54.90 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 68 | 247 | 54 |
| views/all/bins | count | 7562 | 11.6 k | 5881 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 249 | 303 | 74 |
| views/all/pipelines_distinct | count | 71 | 75 | 61 |
| views/all/work_items | count | 10.6 k | 35.6 k | 8127 |
| views/cascade/batch_sets | count | 9 | 53 | 4 |
| views/cascade/bins | count | 2786 | 6165 | 2383 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8887 | 23.5 k | 6340 |
| views/cascade/pipelines | count | 36 | 54 | 28 |
| views/cascade/work_items | count | 395 | 22.8 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 190 | 0 |
| views/lamp_face/bins | count | 2170 | 2697 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2427 | 2999 | 0 |
| views/lamp_face/pipelines | count | 166 | 204 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2873 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2876 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8395 | 13.0 k | 7618 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8564 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 51.71 | 243 | 314.5 k |
| anim/events/count | count | 0.09 | 3 | 521 |
| assets/Image/added | count | 0.10 | 38 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.56 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1312 | 4675 |
| encode//count | count | 33.45 | 220 | 203.5 k |
| encode/bind_walk/init_textures | count | 12.5 k | 114.7 k | 76.25 M |
| encode/bind_walk/init_textures_skipped | count | 65.9 k | 138.6 k | 400.86 M |
| encode/bind_walk/inits | count | 459 | 1120 | 2.79 M |
| encode/bind_walk/inits_skipped | count | 109 | 202 | 664.0 k |
| encode/bind_walk/merge_textures | count | 12.5 k | 114.6 k | 75.81 M |
| encode/bind_walk/merge_textures_skipped | count | 5372 | 10.7 k | 32.68 M |
| encode/bind_walk/merges | count | 98.62 | 333 | 599.9 k |
| encode/bind_walk/merges_skipped | count | 84.81 | 152 | 515.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 6083 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 6083 |
| encode/pass:early prepass/count | count | 1 | 2 | 6083 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 6083 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 6083 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 6083 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 6083 |
| encode/pass:shadow_point_light/count | count | 1.38 | 48 | 8387 |
| encode/queue/command_buffers | count | 44.85 | 278 | 272.8 k |
| encode/queue/submits | count | 2.02 | 4 | 12.3 k |
| encode/queue/write_buffer/bytes | bytes | 1.53 M | 4.30 M | 9.31 G |
| encode/queue/write_buffer/calls | count | 266 | 834 | 1.62 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.5 k | 3.52 M | 1.08 G |
| encode/queue/write_buffer_with/calls | count | 10.99 | 465 | 66.8 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 6083 |
| physics/inserts | count | 0.37 | 106 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2335 |
| physics/steps | count | 0.65 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 10 |
| pipelines/created | count | 0.00 | 2 | 14 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 19 | 378 |
| stream/upload/bytes | bytes | 5565 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1220 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.5 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2543.6 | main/busy/ms 2512, stage/main/post_update 2407, stage/main/update 96.04, surface/advance_cpu_ms 66.22, stage/main/pre_update 4.66, stage/main/last 1.12, stage/main/first 0.79, surface/cell_upload_ms 0.78, stream/drive/ms 0.60, stage/main/fixed_main_loop 0.40, stream/tick/ms 0.20, weather/map/ms 0.13 |
| warm-up | 3.30 | 757.2 | main/busy/ms 2662, stage/main/post_update 2439, render/thread/ms 717, render/busy/ms 547, stage/render/prepare_assets 497, stage/main/update 208, stage/render/prepare_views 166, render/acquire/ms 166, surface/advance_cpu_ms 66.22, stage/render/create_views 29.53, stage/render/render 12.03, render/graph/ms 7.67 |
| warm-up | 3.42 | 123.2 | render/thread/ms 821, render/busy/ms 589, stage/render/prepare_assets 523, main/busy/ms 170, stage/render/prepare_views 167, render/acquire/ms 167, pipelines/compile_ms 128, stage/main/update 117, stage/render/render 81.40, pipelines/front_ms 78.31, render/present/ms 65.50, pipelines/driver_ms 49.83 |
| warm-up | 3.45 | 28.9 | pipelines/compile_ms 399, pipelines/front_ms 293, render/thread/ms 127, pipelines/driver_ms 106, stage/render/render 86.30, render/present/ms 71.30, main/busy/ms 63.55, render/busy/ms 55.33, stage/main/post_update 46.56, pipelines/wait_ms 33.19, terrain/update/ms 31.37, stage/render/prepare_assets 26.09 |
| warm-up | 3.56 | 110.2 | pipelines/wait_ms 3180, pipelines/compile_ms 1332, pipelines/front_ms 1207, main/busy/ms 180, pipelines/driver_ms 125, render/thread/ms 104, render/busy/ms 93.94, stage/main/update 77.00, stage/main/post_update 51.48, stage/render/render 38.82, stage/main/pre_update 38.69, render/graph/ms 27.87 |
| warm-up | 3.71 | 142.2 | pipelines/wait_ms 3950, pipelines/compile_ms 2150, pipelines/front_ms 2021, render/thread/ms 163, main/busy/ms 162, render/busy/ms 161, pipelines/driver_ms 128, stage/render/prepare_assets 79.19, stage/main/update 79.09, stage/main/pre_update 39.82, stage/render/render 31.17, stage/main/post_update 31.08 |
| warm-up | 3.79 | 30.4 | pipelines/compile_ms 1126, pipelines/front_ms 1081, pipelines/wait_ms 116, render/thread/ms 50.15, render/busy/ms 48.65, pipelines/driver_ms 45.03, main/busy/ms 29.40, stage/render/prepare_assets 19.55, stage/main/post_update 17.08, stage/render/prepare_resources 11.30, stage/render/render 10.60, terrain/update/ms 9.89 |
| warm-up | 3.86 | 41.1 | pipelines/compile_ms 459, pipelines/front_ms 436, main/busy/ms 79.08, render/thread/ms 65.16, render/busy/ms 55.46, stage/main/update 54.02, stream/drive/ms 45.64, stage/render/prepare_assets 31.39, pipelines/driver_ms 22.74, stage/main/post_update 19.99, stage/render/render 13.94, pipelines/wait_ms 12.58 |
| warm-up | 3.93 | 68.0 | pipelines/compile_ms 876, pipelines/front_ms 844, render/thread/ms 75.51, render/busy/ms 73.99, main/busy/ms 69.27, stage/main/update 53.03, stage/render/prepare_assets 52.39, stream/drive/ms 45.47, pipelines/driver_ms 32.60, stage/render/render 13.38, stage/main/post_update 12.04, render/graph/ms 11.47 |
| warm-up | 4.07 | 68.4 | pipelines/compile_ms 507, pipelines/front_ms 489, render/thread/ms 59.34, render/busy/ms 57.49, main/busy/ms 49.38, stage/render/prepare_assets 35.40, stage/main/update 33.01, stream/drive/ms 27.74, pipelines/driver_ms 18.26, stage/render/render 14.05, render/graph/ms 11.77, stage/main/post_update 10.46 |
| warm-up | 4.10 | 39.2 | pipelines/compile_ms 275, pipelines/front_ms 263, render/thread/ms 62.80, render/busy/ms 60.70, main/busy/ms 37.90, stage/render/prepare_assets 36.53, stage/render/render 17.14, stage/main/post_update 15.43, stage/main/update 15.24, render/graph/ms 14.44, pipelines/driver_ms 11.80, stage/render/graph_submit 10.36 |
| warm-up | 4.18 | 74.4 | pipelines/compile_ms 751, pipelines/front_ms 726, render/thread/ms 75.92, main/busy/ms 75.03, render/busy/ms 73.81, stage/main/update 47.18, stage/render/prepare_assets 47.03, pipelines/driver_ms 24.82, stage/main/post_update 19.89, stage/render/render 16.35, render/graph/ms 13.60, stage/render/graph_submit 7.64 |
| warm-up | 4.24 | 65.1 | pipelines/compile_ms 1831, pipelines/front_ms 1780, render/thread/ms 95.01, render/busy/ms 92.74, main/busy/ms 70.07, stage/render/prepare_assets 55.68, pipelines/driver_ms 51.29, stage/main/update 48.71, stage/render/render 16.92, stage/main/post_update 14.41, render/graph/ms 14.33, stage/render/graph_submit 9.46 |
| warm-up | 4.30 | 52.3 | pipelines/compile_ms 1209, pipelines/front_ms 1177, render/thread/ms 86.40, render/busy/ms 84.37, main/busy/ms 56.53, pipelines/wait_ms 45.87, stage/render/prepare_assets 40.62, pipelines/driver_ms 32.35, stage/main/update 22.93, stage/main/post_update 18.62, stage/render/render 15.59, render/graph/ms 13.29 |
| warm-up | 4.40 | 103.2 | pipelines/compile_ms 959, pipelines/front_ms 931, pipelines/wait_ms 502, render/thread/ms 134, render/busy/ms 133, main/busy/ms 104, stage/main/post_update 62.26, stage/render/prepare_resources 43.44, stage/render/prepare_assets 33.93, pipelines/driver_ms 28.39, encode//sum 25.34, stage/render/render 21.28 |
| warm-up | 4.47 | 74.5 | pipelines/compile_ms 2741, pipelines/front_ms 2647, pipelines/wait_ms 1284, render/thread/ms 163, render/busy/ms 160, main/busy/ms 128, pipelines/driver_ms 94.42, stage/main/post_update 77.86, encode//sum 72.26, stage/render/render 44.05, render/graph/ms 40.62, stage/render/prepare_resources 37.90 |
| warm-up | 4.56 | 86.8 | pipelines/compile_ms 2703, pipelines/front_ms 2609, pipelines/wait_ms 1981, render/thread/ms 144, render/busy/ms 142, pipelines/driver_ms 94.37, main/busy/ms 76.53, encode//sum 69.55, stage/render/render 42.57, render/graph/ms 39.28, stage/render/prepare_assets 38.90, stage/main/post_update 33.07 |
| warm-up | 4.63 | 72.8 | pipelines/compile_ms 1730, pipelines/front_ms 1679, pipelines/wait_ms 1511, render/thread/ms 116, render/busy/ms 115, main/busy/ms 89.59, stage/main/update 68.72, stage/render/prepare_assets 59.98, stream/drive/ms 54.96, pipelines/driver_ms 50.89, encode//sum 19.76, stage/render/render 18.44 |
| warm-up | 4.69 | 60.0 | pipelines/compile_ms 1926, pipelines/front_ms 1871, pipelines/wait_ms 912, main/busy/ms 106, stage/main/update 83.31, render/thread/ms 69.43, render/busy/ms 67.32, stream/drive/ms 55.05, pipelines/driver_ms 54.84, stage/render/prepare_assets 37.51, stage/render/render 18.92, render/graph/ms 15.74 |
| warm-up | 4.76 | 67.6 | pipelines/compile_ms 1830, pipelines/front_ms 1778, pipelines/wait_ms 1006, main/busy/ms 134, stage/main/update 111, stream/drive/ms 70.34, render/thread/ms 62.09, render/busy/ms 57.94, pipelines/driver_ms 51.84, stage/render/prepare_assets 26.43, stage/render/render 24.25, render/graph/ms 18.86 |
| warm-up | 4.88 | 118.1 | pipelines/compile_ms 1303, pipelines/front_ms 1267, pipelines/wait_ms 451, main/busy/ms 130, stage/main/update 107, render/thread/ms 87.72, render/busy/ms 84.20, stream/drive/ms 70.76, stage/render/prepare_assets 39.52, pipelines/driver_ms 35.59, stage/render/render 24.55, render/graph/ms 20.19 |
| warm-up | 4.92 | 37.4 | pipelines/compile_ms 1171, pipelines/front_ms 1142, main/busy/ms 92.40, render/thread/ms 80.73, render/busy/ms 79.25, stage/main/update 67.44, stream/drive/ms 44.02, stage/render/prepare_assets 31.43, pipelines/driver_ms 29.35, stage/render/render 21.00, render/graph/ms 18.91, stage/main/post_update 14.82 |
| warm-up | 4.99 | 74.0 | pipelines/compile_ms 1579, pipelines/front_ms 1537, main/busy/ms 81.88, stage/main/update 62.29, render/thread/ms 54.66, render/busy/ms 53.34, stream/drive/ms 43.76, pipelines/driver_ms 41.58, stage/render/prepare_assets 27.80, stage/render/render 11.61, stage/main/post_update 11.55, render/graph/ms 9.68 |
| warm-up | 5.06 | 50.6 | pipelines/compile_ms 350, pipelines/front_ms 338, main/busy/ms 58.79, stage/main/update 42.44, render/thread/ms 36.39, render/busy/ms 35.02, stream/drive/ms 31.37, stage/render/render 11.40, pipelines/driver_ms 11.40, stage/render/prepare_assets 11.22, stage/main/post_update 9.53, render/graph/ms 9.42 |
| warm-up | 5.12 | 32.8 | render/thread/ms 52.87, render/busy/ms 51.30, main/busy/ms 47.43, stage/main/post_update 24.38, stage/render/render 18.93, render/graph/ms 17.04, stage/main/update 16.78, stage/render/graph_submit 14.02, encode//sum 7.93, stage/render/prepare_assets 7.72, encode/finish_wall 7.68, encode/submit 6.23 |
| warm-up | 5.18 | 42.5 | pipelines/compile_ms 1007, pipelines/front_ms 983, render/thread/ms 57.48, render/busy/ms 55.97, main/busy/ms 29.83, stage/render/prepare_assets 27.94, pipelines/driver_ms 23.63, stage/render/render 13.48, stage/main/update 11.90, render/graph/ms 11.11, stage/main/post_update 10.77, encode//sum 9.08 |
| warm-up | 5.21 | 28.8 | pipelines/compile_ms 1015, pipelines/front_ms 991, render/thread/ms 65.03, render/busy/ms 62.91, main/busy/ms 41.06, stage/render/prepare_assets 34.65, stage/main/update 26.48, pipelines/driver_ms 24.43, stream/drive/ms 19.55, stage/render/render 13.95, render/graph/ms 11.44, stage/main/post_update 8.84 |
| warm-up | 5.25 | 35.3 | pipelines/compile_ms 498, pipelines/front_ms 485, render/thread/ms 53.15, render/busy/ms 50.97, main/busy/ms 42.32, stage/main/update 26.16, stage/render/prepare_assets 19.50, stream/drive/ms 18.80, stage/render/render 18.28, render/graph/ms 15.61, pipelines/driver_ms 13.74, stage/main/post_update 10.70 |
| warm-up | 5.30 | 31.4 | pipelines/compile_ms 315, pipelines/front_ms 305, render/thread/ms 44.47, render/busy/ms 42.99, main/busy/ms 31.15, stage/render/prepare_assets 18.58, stage/main/update 14.12, stage/render/render 12.07, stage/main/post_update 11.35, render/graph/ms 10.12, pipelines/driver_ms 9.73, encode//sum 8.32 |
| warm-up | 5.53 | 31.9 | render/thread/ms 40.56, render/busy/ms 39.50, main/busy/ms 32.15, stage/render/prepare_assets 21.73, stage/main/post_update 13.19, stage/main/update 12.31, stage/render/render 8.80, render/graph/ms 7.22, encode//sum 6.30, stream/drive/ms 4.88, stage/render/graph_submit 4.39, encode/submit 2.90 |
| warm-up | 6.04 | 31.5 | pipelines/compile_ms 143, pipelines/front_ms 140, render/thread/ms 38.80, render/busy/ms 37.78, main/busy/ms 29.40, stage/render/prepare_assets 17.10, stage/main/update 12.19, stage/main/post_update 9.43, stage/render/render 8.10, encode//sum 6.83, render/graph/ms 6.66, stage/render/graph_submit 4.34 |
| warm-up | 6.23 | 31.6 | pipelines/compile_ms 145, pipelines/front_ms 140, main/busy/ms 40.21, render/thread/ms 39.39, render/busy/ms 37.76, stage/main/update 21.69, stage/render/render 12.51, stage/main/post_update 11.86, render/graph/ms 10.25, encode//sum 8.98, stage/render/prepare_assets 8.76, stage/render/graph_submit 6.56 |
| warm-up | 6.26 | 32.6 | render/thread/ms 50.93, render/busy/ms 49.32, main/busy/ms 36.36, stage/render/prepare_assets 16.69, stage/main/update 15.95, stage/main/post_update 13.29, stage/render/render 12.61, render/graph/ms 10.10, pipelines/compile_ms 9.28, encode//sum 9.15, pipelines/front_ms 8.52, stage/render/graph_submit 6.43 |
| warm-up | 6.50 | 40.5 | render/thread/ms 49.98, main/busy/ms 49.43, render/busy/ms 48.60, pipelines/compile_ms 33.46, pipelines/front_ms 31.45, stage/main/update 28.60, stage/render/prepare_assets 26.36, stream/drive/ms 14.41, stage/main/post_update 13.79, stage/render/render 11.87, render/graph/ms 9.72, encode//sum 7.47 |
| warm-up | 6.54 | 45.6 | render/thread/ms 65.33, render/busy/ms 64.14, main/busy/ms 47.21, stage/render/prepare_assets 44.61, pipelines/compile_ms 33.46, pipelines/front_ms 31.45, stage/main/update 27.67, stream/drive/ms 17.23, stage/main/post_update 13.55, stage/render/render 10.81, render/graph/ms 9.00, encode//sum 7.44 |
| warm-up | 6.90 | 33.5 | render/thread/ms 39.34, render/busy/ms 38.19, pipelines/compile_ms 23.75, pipelines/front_ms 22.70, main/busy/ms 19.38, stage/render/render 11.65, stage/main/post_update 10.82, stage/render/specialize 10.09, render/graph/ms 10.07, encode//sum 7.84, stage/render/graph_submit 7.64, stage/render/queue 7.45 |
| warm-up | 8.16 | 31.2 | pipelines/compile_ms 135, pipelines/front_ms 132, render/thread/ms 36.92, render/busy/ms 35.85, main/busy/ms 18.23, stage/main/post_update 10.82, stage/render/render 10.76, stage/render/specialize 9.91, render/graph/ms 9.23, encode//sum 7.74, stage/render/graph_submit 6.85, stage/render/queue 6.38 |
| measure | 0.43 | 28.9 | render/thread/ms 40.80, render/busy/ms 39.63, encode//sum 25.31, main/busy/ms 21.38, stage/render/render 15.64, stage/main/post_update 14.19, render/graph/ms 13.93, stage/render/graph_submit 11.64, stage/render/specialize 9.04, stage/render/queue 6.51, encode/submit 6.40, encode/finish_wall 5.30 |
| measure | 1.69 | 31.2 | render/thread/ms 35.06, render/busy/ms 33.93, main/busy/ms 16.87, stage/main/post_update 10.50, stage/render/render 9.42, stage/render/specialize 9.02, render/graph/ms 7.97, encode//sum 6.74, stage/render/graph_submit 6.01, stage/render/queue 5.95, stage/render/prepare_meshes 4.81, stage/main/update 3.93 |
| measure | 3.02 | 29.1 | render/thread/ms 35.94, render/busy/ms 34.96, main/busy/ms 19.40, stage/main/post_update 12.49, stage/render/render 11.69, render/graph/ms 10.13, stage/render/specialize 9.33, encode//sum 7.80, stage/render/graph_submit 7.61, stage/render/queue 5.86, encode/submit 4.68, stage/main/update 4.07 |
| measure | 4.39 | 29.3 | render/thread/ms 35.67, render/busy/ms 34.61, main/busy/ms 18.10, stage/main/post_update 11.75, stage/render/render 10.38, stage/render/specialize 9.51, render/graph/ms 8.84, encode//sum 7.60, stage/render/graph_submit 6.74, stage/render/queue 6.49, encode/submit 4.02, stage/main/update 3.84 |
| measure | 5.74 | 30.6 | render/thread/ms 35.26, render/busy/ms 34.18, main/busy/ms 21.70, stage/main/post_update 12.71, stage/render/specialize 10.16, stage/render/render 10.16, render/graph/ms 8.63, encode//sum 7.39, stage/render/queue 6.49, stage/render/graph_submit 6.44, stage/main/update 4.31, encode/submit 3.79 |
| measure | 7.12 | 31.0 | render/thread/ms 40.65, render/busy/ms 39.62, main/busy/ms 20.82, stage/main/post_update 12.72, stage/render/render 12.45, stage/render/specialize 11.20, render/graph/ms 10.90, encode//sum 9.32, stage/render/graph_submit 8.34, stage/render/queue 7.31, encode/submit 4.95, stage/main/update 4.55 |
| measure | 8.43 | 30.1 | render/thread/ms 35.93, render/busy/ms 34.88, main/busy/ms 17.18, stage/main/post_update 10.87, stage/render/render 10.68, stage/render/specialize 9.59, render/graph/ms 9.09, encode//sum 7.83, stage/render/queue 6.89, stage/render/graph_submit 6.75, encode/submit 3.91, stage/main/update 3.89 |
| measure | 9.78 | 32.6 | render/thread/ms 42.98, render/busy/ms 41.88, main/busy/ms 20.36, stage/render/render 13.49, stage/main/post_update 12.14, render/graph/ms 11.86, stage/render/specialize 10.67, encode//sum 9.31, stage/render/graph_submit 8.93, stage/render/queue 7.33, encode/submit 5.68, stage/main/update 5.47 |
| measure | 11.17 | 29.6 | render/thread/ms 37.56, render/busy/ms 36.51, main/busy/ms 18.22, stage/render/render 12.39, stage/main/post_update 11.38, render/graph/ms 10.80, stage/render/specialize 9.53, encode//sum 9.14, stage/render/graph_submit 8.04, stage/render/queue 6.59, encode/submit 4.93, encode/pass:shadow_point_light/sum 4.66 |
| measure | 15.72 | 30.1 | render/thread/ms 38.43, render/busy/ms 37.42, main/busy/ms 19.13, stage/render/render 12.24, stage/main/post_update 12.16, render/graph/ms 10.86, stage/render/specialize 10.14, encode//sum 8.43, stage/render/graph_submit 8.24, stage/render/queue 6.76, encode/submit 5.14, encode/pass:shadow_point_light/sum 4.41 |
| measure | 18.01 | 30.4 | render/thread/ms 39.14, render/busy/ms 38.09, main/busy/ms 17.99, stage/render/render 12.61, stage/main/post_update 11.13, render/graph/ms 11.05, stage/render/specialize 9.81, encode//sum 8.89, stage/render/graph_submit 7.84, stage/render/queue 6.65, encode/submit 4.90, stage/main/update 4.05 |
| measure | 19.41 | 33.3 | render/thread/ms 38.40, render/busy/ms 37.37, main/busy/ms 20.67, stage/main/post_update 12.04, stage/render/specialize 12.00, stage/render/render 10.19, render/graph/ms 8.76, encode//sum 7.46, stage/render/graph_submit 6.80, stage/render/queue 6.77, stage/main/update 4.71, encode/submit 3.66 |
| measure | 20.76 | 31.0 | render/thread/ms 38.10, render/busy/ms 37.01, main/busy/ms 16.87, stage/render/render 11.28, stage/main/post_update 10.64, stage/render/specialize 10.45, render/graph/ms 9.66, encode//sum 8.16, stage/render/graph_submit 7.14, stage/render/queue 6.99, encode/submit 4.20, stage/main/update 3.86 |
| measure | 23.01 | 33.2 | render/thread/ms 43.10, render/busy/ms 42.04, main/busy/ms 24.86, stage/main/post_update 12.67, stage/render/specialize 12.45, stage/render/render 12.40, render/graph/ms 10.93, stage/render/graph_submit 8.66, encode//sum 8.66, stage/render/queue 7.55, stage/main/update 6.28, encode/submit 5.21 |
| measure | 24.46 | 30.7 | render/thread/ms 36.63, render/busy/ms 35.50, main/busy/ms 19.19, stage/main/post_update 11.77, stage/render/render 10.92, stage/render/specialize 9.69, render/graph/ms 9.41, encode//sum 8.54, stage/render/graph_submit 7.02, stage/render/queue 6.73, stage/main/update 4.77, encode/pass:shadow_point_light/sum 4.49 |
| measure | 27.28 | 37.1 | render/thread/ms 44.51, render/busy/ms 43.42, main/busy/ms 25.21, stage/render/render 15.12, render/graph/ms 13.57, stage/main/post_update 12.76, stage/render/specialize 11.61, encode//sum 11.40, stage/render/graph_submit 10.68, stage/render/queue 7.77, encode/pass:shadow_point_light/sum 6.72, encode/submit 6.56 |
| measure | 28.73 | 31.8 | render/thread/ms 41.06, render/busy/ms 40.03, main/busy/ms 18.30, stage/render/render 13.65, render/graph/ms 12.18, stage/main/post_update 11.56, encode//sum 10.54, stage/render/specialize 9.77, stage/render/graph_submit 8.95, stage/render/queue 7.45, encode/pass:shadow_point_light/sum 6.36, encode/submit 5.17 |
| measure | 30.47 | 33.0 | render/thread/ms 40.17, render/busy/ms 39.03, main/busy/ms 20.97, stage/main/post_update 12.20, stage/render/render 12.17, stage/render/specialize 10.65, render/graph/ms 10.64, encode//sum 9.18, stage/render/graph_submit 8.52, stage/render/queue 7.47, stage/main/update 5.60, encode/submit 4.69 |
| measure | 35.35 | 34.1 | render/thread/ms 38.32, render/busy/ms 37.14, main/busy/ms 20.74, stage/main/post_update 13.52, stage/render/render 10.81, stage/render/specialize 10.30, render/graph/ms 9.21, encode//sum 7.66, stage/render/graph_submit 7.10, stage/render/queue 6.85, stage/main/update 4.28, encode/submit 4.05 |
| measure | 36.70 | 29.0 | render/thread/ms 37.54, render/busy/ms 36.42, main/busy/ms 19.87, stage/main/post_update 12.64, stage/render/render 11.37, stage/render/specialize 10.67, render/graph/ms 9.86, encode//sum 7.81, stage/render/graph_submit 7.62, stage/render/queue 6.41, encode/submit 5.06, stage/main/update 4.28 |
| measure | 37.48 | 29.7 | render/thread/ms 37.66, render/busy/ms 36.56, main/busy/ms 17.94, stage/render/render 11.77, stage/main/post_update 11.49, render/graph/ms 10.27, stage/render/specialize 9.46, encode//sum 9.15, stage/render/graph_submit 7.94, stage/render/queue 6.85, encode/submit 4.53, stage/main/update 4.09 |
| measure | 40.78 | 31.2 | render/thread/ms 37.00, render/busy/ms 35.93, main/busy/ms 18.61, stage/main/post_update 10.79, stage/render/specialize 9.94, stage/render/render 9.27, render/graph/ms 7.88, encode//sum 7.29, stage/render/queue 6.62, stage/render/graph_submit 6.03, stage/main/update 4.49, encode/submit 3.63 |
| measure | 43.21 | 30.4 | render/thread/ms 38.62, render/busy/ms 37.61, main/busy/ms 17.00, stage/render/render 11.59, render/graph/ms 10.19, stage/render/specialize 9.99, stage/main/post_update 9.95, stage/render/graph_submit 8.00, encode//sum 7.73, stage/render/queue 7.02, encode/submit 4.32, stage/main/update 4.16 |
| measure | 45.67 | 33.1 | render/thread/ms 39.04, render/busy/ms 38.00, main/busy/ms 18.37, stage/render/specialize 10.62, stage/main/post_update 10.52, stage/render/render 10.49, render/graph/ms 9.06, encode//sum 8.09, stage/render/graph_submit 7.14, stage/render/queue 6.71, stage/main/update 4.59, stage/render/prepare_meshes 4.22 |
| measure | 59.03 | 41.4 | render/thread/ms 55.42, render/busy/ms 53.65, main/busy/ms 33.98, stage/main/post_update 20.66, stage/render/render 19.19, render/graph/ms 17.08, stage/render/graph_submit 13.80, encode//sum 12.48, stage/render/specialize 12.41, stage/main/update 8.34, stage/render/prepare_meshes 7.36, encode/finish_wall 6.98 |
| measure | 59.06 | 31.2 | render/thread/ms 57.58, render/busy/ms 56.43, main/busy/ms 26.14, stage/render/prepare_assets 14.26, stage/main/post_update 13.19, stage/render/specialize 12.62, stage/render/render 10.21, render/graph/ms 8.81, encode//sum 8.19, stage/render/prepare_meshes 8.16, stage/main/update 7.30, stage/render/graph_submit 6.83 |
| measure | 62.58 | 34.1 | render/thread/ms 37.29, render/busy/ms 35.49, main/busy/ms 18.82, stage/render/render 10.69, stage/render/specialize 10.63, stage/main/post_update 10.43, render/graph/ms 8.65, encode//sum 7.35, stage/render/queue 6.81, stage/render/graph_submit 6.62, stage/main/update 5.80, stage/render/prepare_meshes 4.04 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set render.skin.skip_unchanged=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=skin-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 4%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **78%** (min 53%) · 2620 MHz (min 2610) · memory 8101 MHz · 80.7 W · 78 °C (max 83) · VRAM 7050 MiB · throttled 100% of samples

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
- `render.skin.skip_unchanged=on (command line: --set)`
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
