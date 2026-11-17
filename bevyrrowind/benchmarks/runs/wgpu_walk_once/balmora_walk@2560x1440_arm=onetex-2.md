# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: texvariety · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6141 frames over 65.6 s · **median 10.26 ms (97 fps)** · mean 10.69 · p95 13.86 · p99 24.05 (1% low 42 fps) · worst 58.27 ms · 177 spikes over 1.5× median · 60.3 stutters/min (work 51.2/min)
- warm-up: 321 frames over 14.1 s, worst 2471.9 ms
- hitches (> 25.7 ms): **51** measured, 173 in warm-up
- timed GPU passes: 6.25 ms of a 10.69 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.26 ms, limit 11 ms
- **FAIL** frame_ms: 66 stutters (> 1.5x moving median of 60) in 1.09 min = 60.32/min, limit 1/min
- **FAIL** worst frame 58.27 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.827 | 2.026 | 2.051 | 0.90 | 0.184 |
| main_opaque_pass_3d | 1.284 | 1.424 | 1.870 | 0.90 | 0.061 |
| early prepass | 0.438 | 0.486 | 0.710 | 0.90 | 0.091 |
| bloom | 0.279 | 0.309 | 0.317 | 0.90 | 0.067 |
| atmosphere_luts | 0.275 | 0.305 | 0.367 | 0.90 | 0.004 |
| water_velocity | 0.243 | 0.269 | 0.347 | 0.90 | 0.002 |
| water_sspr | 0.182 | 0.202 | 0.203 | 0.90 | 0.002 |
| ssao | 0.171 | 0.189 | 0.193 | 0.90 | 0.007 |
| atmos_froxel_inject | 0.132 | 0.146 | 0.148 | 0.90 | 0.001 |
| clouds_trace | 0.124 | 0.137 | 0.234 | 0.90 | 0.001 |
| uniform_allocation | 0.117 | 0.130 | 0.180 | 0.90 | 0.027 |
| water_fft | 0.117 | 0.129 | 0.138 | 0.90 | 0.003 |
| auxiliary_views | 0.110 | 0.122 | 0.494 | 0.90 | 0.131 |
| upscaling | 0.101 | 0.112 | 0.121 | 0.90 | 0.004 |
| clouds_composite | 0.095 | 0.106 | 0.109 | 0.90 | 0.002 |
| bin_unpacking | 0.095 | 0.105 | 0.143 | 0.90 | 0.019 |
| tonemapping | 0.076 | 0.085 | 0.088 | 0.90 | 0.003 |
| main_transmissive_pass_3d | 0.072 | 0.080 | 0.159 | 0.90 | 0.009 |
| clustering | 0.058 | 0.064 | 0.068 | 0.90 | 0.019 |
| render_sky | 0.046 | 0.051 | 0.052 | 0.90 | 0.002 |
| clouds_resolve | 0.044 | 0.049 | 0.049 | 0.90 | 0.001 |
| surface_exposure | 0.040 | 0.045 | 0.045 | 0.90 | 0.001 |
| lightprobe_irradiance_map | 0.040 | 0.417 | 0.417 | 0.09 | 0.001 |
| surface_exposure_far | 0.035 | 0.044 | 0.046 | 0.80 | 0.002 |
| surface_trail | 0.035 | 0.056 | 0.057 | 0.61 | 0.002 |
| early_mesh_preprocessing | 0.028 | 0.031 | 0.048 | 0.90 | 0.005 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.90 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.031 | 0.032 | 0.90 | 0.001 |
| water_snapshot | 0.026 | 0.029 | 0.033 | 0.90 | 0.002 |
| lightprobe_radiance_map | 0.021 | 0.220 | 0.230 | 0.09 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.024 | 0.90 | 0.002 |
| night_sky | 0.012 | 0.013 | 0.034 | 0.90 | 0.002 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.90 | 0.002 |
| main_transparent_pass_3d | 0.010 | 0.011 | 0.026 | 0.90 | 0.125 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.90 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.90 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.90 | 0.003 |
| lightprobe_copy | 0.001 | 0.015 | 0.015 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 25.025 | 0.000 | 10713.83 | 153677.2 |
| pipelines/compile_ms | 17.898 | 0.000 | 7359.27 | 109914.6 |
| pipelines/driver_ms | 15.885 | 0.000 | 6983.26 | 97550.9 |
| render/thread/ms | 9.060 | 11.668 | 54.59 | 55639.3 |
| main/busy/ms | 9.018 | 11.187 | 49.44 | 55377.8 |
| render/busy/ms | 8.551 | 11.196 | 53.83 | 52509.2 |
| encode//sum | 5.596 | 7.747 | 39.61 | 34367.6 |
| stage/main/post_update | 5.499 | 7.128 | 33.15 | 33770.2 |
| stage/render/render | 4.960 | 6.574 | 24.77 | 30461.8 |
| render/graph/ms | 4.254 | 5.819 | 23.63 | 26126.8 |
| stage/render/graph_submit | 3.056 | 4.265 | 20.47 | 18766.0 |
| stage/main/update | 2.212 | 2.846 | 24.90 | 13582.8 |
| pipelines/front_ms | 2.013 | 0.000 | 430.62 | 12363.6 |
| encode/submit | 1.984 | 2.939 | 7.35 | 12181.5 |
| stage/render/camera_driver | 1.144 | 1.641 | 14.73 | 7023.3 |
| encode/finish_wall | 1.132 | 1.653 | 16.93 | 6949.4 |
| stage/render/prepare_write_phase_buffers | 0.770 | 1.124 | 9.75 | 4731.0 |
| stage/render/prepare_assets | 0.586 | 0.775 | 16.97 | 3597.2 |
| bigspace/total/ms | 0.508 | 1.079 | 9.33 | 3119.7 |
| stage/render/prepare_resources | 0.459 | 0.565 | 12.34 | 2819.4 |
| stage/render/prepare_meshes | 0.452 | 0.573 | 7.42 | 2775.4 |
| stage/main/first | 0.415 | 0.572 | 5.32 | 2547.3 |
| render/present/ms | 0.409 | 1.017 | 2.16 | 2509.6 |
| stage/render/specialize | 0.402 | 0.606 | 16.36 | 2471.3 |
| stage/main/pre_update | 0.371 | 0.553 | 4.19 | 2280.1 |
| terrain/update/ms | 0.306 | 0.392 | 1.48 | 1881.6 |
| stage/render/queue | 0.294 | 0.434 | 13.49 | 1808.0 |
| stage/render/prepare_bind_groups | 0.291 | 0.520 | 5.71 | 1784.3 |
| controller/feed/ms | 0.273 | 0.357 | 1.01 | 1678.6 |
| terrain/select/ms | 0.272 | 0.349 | 0.87 | 1673.2 |
| stream/drive/ms | 0.258 | 0.387 | 21.42 | 1582.6 |
| stage/main/fixed_main_loop | 0.217 | 0.404 | 1.65 | 1331.8 |
| far/select/ms | 0.211 | 0.305 | 0.83 | 1294.7 |
| stage/render/create_views | 0.207 | 0.281 | 2.12 | 1271.8 |
| bigspace/low_precision/ms | 0.207 | 0.590 | 2.44 | 1270.2 |
| stage/render/prepare_batch_phases | 0.206 | 0.291 | 4.79 | 1263.2 |
| stream/tick/ms | 0.199 | 0.296 | 0.60 | 1223.3 |
| stage/render/prepare_flush | 0.179 | 0.369 | 9.27 | 1098.9 |
| stage/render/extract_commands | 0.178 | 0.223 | 0.57 | 1091.2 |
| anim/evaluate/ms | 0.157 | 0.274 | 3.11 | 965.5 |
| anim/evaluate_cpu/ms | 0.150 | 0.195 | 0.70 | 921.8 |
| stage/render/prepare_views | 0.146 | 0.193 | 1.01 | 896.0 |
| bigspace/high_precision/ms | 0.143 | 0.533 | 8.74 | 879.5 |
| controller/move/ms | 0.137 | 0.270 | 0.77 | 842.2 |
| stage/main/last | 0.122 | 0.188 | 1.65 | 746.5 |
| render/acquire/ms | 0.101 | 0.133 | 0.97 | 620.4 |
| stage/render/prepare_collect_phase_buffers | 0.066 | 0.142 | 0.57 | 404.5 |
| physics/step/ms | 0.047 | 0.091 | 0.56 | 288.0 |
| encode/resolve_diagnostics/sum | 0.034 | 0.050 | 0.47 | 210.0 |
| stage/render/cleanup | 0.033 | 0.091 | 1.04 | 201.8 |
| physics/insert/ms | 0.033 | 0.354 | 1.21 | 200.9 |
| stage/render/phase_sort | 0.028 | 0.047 | 1.46 | 174.2 |
| weather/map/ms | 0.026 | 0.036 | 0.23 | 160.7 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.024 | 0.42 | 96.9 |
| encode/camera_driver_span_end/sum | 0.012 | 0.020 | 0.63 | 73.0 |
| anim/apply/ms | 0.011 | 0.020 | 0.41 | 70.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.10 | 69.3 |
| controller/avoid/ms | 0.011 | 0.015 | 0.16 | 68.7 |
| script/update/ms | 0.008 | 0.011 | 0.05 | 46.4 |
| script/ui/ms | 0.006 | 0.008 | 0.08 | 34.9 |
| stream/despawn/ms | 0.005 | 0.000 | 1.00 | 31.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 31.3 |
| script/input/ms | 0.005 | 0.007 | 0.18 | 28.7 |
| controller/write/ms | 0.004 | 0.005 | 0.22 | 22.9 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 14.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.15 | 6.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.44 | 5.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.13 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.09 | 0.9 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.16 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.08 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1258 | 2262 | 583 |
| crowd/c0/meshes/count | count | 348 | 646 | 162 |
| crowd/c1/count | count | 147 | 157 | 148 |
| crowd/c1/joints/count | count | 5535 | 6037 | 5613 |
| crowd/c1/meshes/count | count | 1167 | 1311 | 1179 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4631 | 10.3 k | 6528 |
| crowd/joints_uploaded/count | count | 4613 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.77 | 15.71 | 0.69 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.42 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.63 | 0.03 |
| encode/finish_critical | ms | 0.77 | 15.71 | 0.69 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.47 | 0.05 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.06 | 53.83 | 8.09 |
| gpu/timed/ms | ms | 7.61 | 10.67 | 8.09 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 750 | 763 | 755 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 795 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 795 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1286 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3345 | 5112 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1286 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3345 | 5112 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 14 | 229 | 4 |
| phases/shadow/bins | count | 5084 | 8733 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6376 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 207 | 259 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 203 | 476 | 19 |
| physics/pending | count | 0 | 186 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 282 | 324 | 324 |
| pipelines/waiting | count | 1 | 25 | 0 |
| precache/draw_waiting | count | 0 | 17 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 1 | 7 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 16 | 70 | 0 |
| preprocess/bind_groups | count | 12 | 617 | 2 |
| preprocess/bind_groups_created | count | 12 | 617 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 128 | 204 | 127 |
| process/cpu_usage | % | 31.88 | 97.12 | 27.88 |
| process/mem_usage | GiB | 4.25 | 4.41 | 4.41 |
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
| skin/blocks_changed/count | count | 13 | 133 | 10 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 201 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 370.9 k | 812.5 k | 497.7 k |
| skin/joints_written/count | count | 5796 | 12.7 k | 7776 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4993 | 5084 | 5084 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 432 | 528 | 396 |
| statics/materials_bound | count | 432 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.14 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 38.44 | 99.82 | 35.99 |
| system/mem_usage | % | 62.40 | 62.81 | 62.80 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 1447 | 1452 | 1451 |
| textures/images | count | 820 | 830 | 825 |
| views/all/batch_sets | count | 63 | 276 | 54 |
| views/all/bins | count | 7625 | 11.6 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.3 k | 38.4 k | 14.0 k |
| views/all/pipelines | count | 252 | 306 | 75 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 10.6 k | 35.2 k | 8119 |
| views/cascade/batch_sets | count | 9 | 51 | 4 |
| views/cascade/bins | count | 2787 | 6131 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8917 | 23.5 k | 6376 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 407 | 23.1 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 192 | 0 |
| views/lamp_face/bins | count | 2225 | 2724 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2475 | 3021 | 0 |
| views/lamp_face/pipelines | count | 171 | 208 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2903 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2882 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8288 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8453 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 47.76 | 239 | 293.3 k |
| anim/events/count | count | 0.09 | 3 | 528 |
| assets/Image/added | count | 0.10 | 43 | 609 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.14 | 86 | 848 |
| assets/Mesh/added | count | 0.66 | 152 | 4061 |
| assets/Mesh/removed | count | 0.79 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.52 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 98.40 | 3260 | 604.3 k |
| ecs/entities_delta | count | 0.76 | 1304 | 4676 |
| encode//count | count | 38.52 | 273 | 236.5 k |
| encode/bind_walk/init_textures | count | 22.4 k | 63.4 k | 137.82 M |
| encode/bind_walk/init_textures_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/inits | count | 562 | 1336 | 3.45 M |
| encode/bind_walk/inits_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/merge_textures | count | 5467 | 35.3 k | 33.57 M |
| encode/bind_walk/merge_textures_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/merges | count | 182 | 438 | 1.12 M |
| encode/bind_walk/merges_skipped | count | 0 | 0 | 0 |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6141 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6141 |
| encode/queue/command_buffers | count | 43.54 | 278 | 267.4 k |
| encode/queue/submits | count | 2.02 | 4 | 12.4 k |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 4.89 M | 11.58 G |
| encode/queue/write_buffer/calls | count | 252 | 920 | 1.55 M |
| encode/queue/write_buffer_with/bytes | bytes | 180.2 k | 3.52 M | 1.11 G |
| encode/queue/write_buffer_with/calls | count | 10.97 | 465 | 67.4 k |
| encode/queue/write_texture/bytes | bytes | 10.2 k | 8.39 M | 62.48 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 562 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6141 |
| physics/inserts | count | 0.37 | 93 | 2272 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2335 |
| physics/steps | count | 0.64 | 3 | 3937 |
| physics/steps_dropped | count | 0.00 | 1 | 2 |
| pipelines/after_reveal | count | 0.00 | 1 | 8 |
| pipelines/created | count | 0.02 | 2 | 111 |
| precache/hit | count | 0.00 | 2 | 3 |
| precache/missed | count | 0.00 | 2 | 7 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5513 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 28 |
| surface/shelter_redraws | count | 0.01 | 1 | 54 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1209 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.7 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2471.9 | main/busy/ms 2444, stage/main/post_update 2328, stage/main/update 108, surface/advance_cpu_ms 76.45, stage/main/pre_update 3.99, stage/main/last 1.00, surface/cell_upload_ms 0.73, stage/main/first 0.61, stage/main/fixed_main_loop 0.53, stream/drive/ms 0.44, weather/map/ms 0.18, bigspace/total/ms 0.11 |
| warm-up | 3.29 | 821.6 | main/busy/ms 2571, stage/main/post_update 2350, render/thread/ms 788, render/busy/ms 651, stage/render/prepare_assets 603, stage/main/update 208, stage/render/prepare_views 135, render/acquire/ms 134, surface/advance_cpu_ms 76.45, stage/render/create_views 27.77, stage/render/render 12.56, encode//sum 10.38 |
| warm-up | 3.42 | 128.7 | render/thread/ms 894, render/busy/ms 701, stage/render/prepare_assets 635, pipelines/compile_ms 329, pipelines/driver_ms 249, main/busy/ms 155, stage/render/prepare_views 135, render/acquire/ms 135, stage/main/update 109, pipelines/front_ms 80.22, stage/render/render 72.74, render/present/ms 58.20 |
| warm-up | 3.47 | 47.1 | pipelines/compile_ms 707, pipelines/driver_ms 561, render/thread/ms 151, pipelines/front_ms 146, render/busy/ms 93.45, stage/render/render 67.14, main/busy/ms 66.09, render/present/ms 56.81, stage/main/post_update 39.09, stage/render/prepare_assets 37.63, stage/render/prepare_batch_phases 24.04, terrain/update/ms 22.38 |
| warm-up | 3.53 | 64.1 | pipelines/compile_ms 445, pipelines/driver_ms 366, render/thread/ms 107, render/busy/ms 104, main/busy/ms 95.31, pipelines/front_ms 78.20, stage/main/post_update 59.71, stage/main/update 27.34, stage/render/prepare_assets 25.31, stage/render/prepare_bind_groups 24.74, stage/render/prepare_batch_phases 24.62, stage/render/prepare_flush 13.34 |
| warm-up | 3.62 | 83.5 | pipelines/compile_ms 722, pipelines/driver_ms 638, main/busy/ms 146, render/thread/ms 130, render/busy/ms 128, stage/main/update 84.21, pipelines/front_ms 84.16, pipelines/wait_ms 68.88, stage/main/post_update 42.67, stage/render/prepare_assets 37.85, stage/render/prepare_bind_groups 36.75, stage/render/prepare_resources 18.37 |
| warm-up | 3.71 | 92.4 | pipelines/compile_ms 1925, pipelines/driver_ms 1679, pipelines/wait_ms 600, pipelines/front_ms 246, render/thread/ms 136, render/busy/ms 135, main/busy/ms 103, stage/render/prepare_assets 76.76, stage/main/update 66.64, stage/render/prepare_resources 18.35, stage/main/post_update 16.44, stage/main/pre_update 15.81 |
| warm-up | 3.88 | 68.7 | pipelines/wait_ms 1206, pipelines/compile_ms 302, pipelines/driver_ms 237, render/thread/ms 90.10, render/busy/ms 88.10, main/busy/ms 68.73, pipelines/front_ms 65.23, stage/render/prepare_resources 32.52, stage/main/post_update 28.83, stage/render/prepare_assets 22.34, stage/main/update 19.49, stage/render/prepare_flush 14.54 |
| warm-up | 3.94 | 68.6 | pipelines/wait_ms 1320, pipelines/compile_ms 532, pipelines/driver_ms 427, render/thread/ms 132, render/busy/ms 131, main/busy/ms 128, pipelines/front_ms 105, stage/main/update 68.84, stage/render/prepare_resources 62.57, stream/drive/ms 44.71, stage/main/post_update 40.05, stage/render/prepare_assets 24.65 |
| warm-up | 4.10 | 152.1 | pipelines/wait_ms 3482, pipelines/compile_ms 750, pipelines/driver_ms 574, render/thread/ms 190, render/busy/ms 188, pipelines/front_ms 176, main/busy/ms 136, stage/render/prepare_resources 63.34, stage/main/post_update 56.25, stage/main/update 55.09, stage/render/prepare_assets 48.86, stream/drive/ms 44.68 |
| warm-up | 4.16 | 62.5 | pipelines/wait_ms 2685, pipelines/compile_ms 435, pipelines/driver_ms 332, render/thread/ms 181, render/busy/ms 179, pipelines/front_ms 103, main/busy/ms 86.70, stage/render/prepare_resources 63.92, stage/main/post_update 45.04, stage/render/prepare_assets 39.54, stage/render/prepare_flush 28.86, stage/main/fixed_main_loop 19.13 |
| warm-up | 4.21 | 49.0 | pipelines/compile_ms 548, pipelines/driver_ms 483, pipelines/wait_ms 157, render/thread/ms 102, render/busy/ms 101, pipelines/front_ms 65.37, main/busy/ms 62.73, stage/render/prepare_resources 45.69, stage/main/update 24.32, stage/render/prepare_flush 19.83, stage/main/post_update 16.81, stage/main/fixed_main_loop 16.54 |
| warm-up | 4.30 | 97.2 | pipelines/compile_ms 548, pipelines/driver_ms 483, pipelines/wait_ms 157, render/thread/ms 139, render/busy/ms 138, main/busy/ms 99.74, pipelines/front_ms 65.37, stage/main/update 40.69, stage/main/post_update 39.15, stage/render/prepare_resources 37.38, stage/render/prepare_assets 35.31, stage/render/prepare_flush 30.42 |
| warm-up | 4.37 | 63.1 | render/thread/ms 135, render/busy/ms 134, main/busy/ms 121, stage/main/post_update 70.11, stage/main/update 33.91, stage/render/prepare_resources 31.38, stage/render/prepare_assets 30.98, stream/drive/ms 29.69, stage/render/prepare_flush 29.39, stage/render/prepare_write_phase_buffers 18.62, stage/render/render 12.05, stage/render/prepare_bind_groups 7.33 |
| warm-up | 4.49 | 119.7 | pipelines/wait_ms 3308, pipelines/compile_ms 1243, pipelines/driver_ms 1081, pipelines/front_ms 162, render/thread/ms 124, render/busy/ms 123, main/busy/ms 120, stage/main/post_update 53.95, stage/render/prepare_assets 41.67, stage/main/update 37.90, stage/render/prepare_resources 32.18, stage/render/prepare_flush 23.35 |
| warm-up | 4.56 | 76.6 | pipelines/wait_ms 5347, pipelines/compile_ms 1299, pipelines/driver_ms 1122, pipelines/front_ms 176, render/thread/ms 139, render/busy/ms 137, main/busy/ms 93.09, stage/render/prepare_assets 57.68, stage/main/update 42.12, stage/render/prepare_resources 30.76, stage/main/post_update 29.86, stage/render/prepare_flush 19.13 |
| warm-up | 4.63 | 69.3 | pipelines/wait_ms 2039, render/thread/ms 120, render/busy/ms 117, pipelines/compile_ms 56.22, main/busy/ms 51.23, pipelines/driver_ms 41.89, stage/render/prepare_assets 38.07, stage/render/prepare_write_phase_buffers 29.07, stage/render/prepare_resources 20.23, stage/main/update 19.20, stage/main/post_update 18.43, pipelines/front_ms 14.33 |
| warm-up | 4.68 | 47.1 | pipelines/compile_ms 1414, pipelines/driver_ms 1321, pipelines/wait_ms 1069, render/thread/ms 103, render/busy/ms 100, pipelines/front_ms 93.15, main/busy/ms 46.62, encode//sum 37.80, stage/render/prepare_write_phase_buffers 21.51, stage/render/prepare_assets 20.72, stage/main/update 20.29, stage/render/prepare_resources 19.93 |
| warm-up | 4.76 | 79.0 | pipelines/wait_ms 2823, pipelines/compile_ms 1495, pipelines/driver_ms 1373, pipelines/front_ms 123, render/thread/ms 114, render/busy/ms 112, encode//sum 39.30, main/busy/ms 35.10, stage/render/prepare_resources 28.76, stage/render/prepare_batch_phases 19.26, stage/render/prepare_assets 18.66, stage/render/render 17.77 |
| warm-up | 4.83 | 66.7 | pipelines/wait_ms 4310, pipelines/compile_ms 158, render/thread/ms 139, render/busy/ms 137, pipelines/driver_ms 116, main/busy/ms 50.33, pipelines/front_ms 41.28, stage/render/prepare_assets 32.42, stage/render/prepare_resources 27.96, stage/main/post_update 22.44, stage/main/update 20.84, stage/render/prepare_batch_phases 19.59 |
| warm-up | 4.89 | 60.6 | pipelines/wait_ms 2780, pipelines/compile_ms 157, pipelines/driver_ms 118, render/thread/ms 112, render/busy/ms 110, main/busy/ms 70.54, stage/render/prepare_assets 55.82, pipelines/front_ms 39.18, stage/main/post_update 34.14, stage/main/update 21.74, stage/render/render 15.41, encode//sum 14.54 |
| warm-up | 4.98 | 94.0 | pipelines/wait_ms 529, pipelines/compile_ms 168, render/thread/ms 136, render/busy/ms 135, pipelines/driver_ms 112, main/busy/ms 109, stage/main/update 60.18, pipelines/front_ms 56.36, stream/drive/ms 52.38, stage/render/prepare_assets 40.24, stage/main/post_update 31.75, stage/render/prepare_resources 26.97 |
| warm-up | 5.07 | 85.5 | pipelines/wait_ms 1154, pipelines/compile_ms 179, render/thread/ms 170, render/busy/ms 169, main/busy/ms 155, pipelines/driver_ms 119, stage/main/update 103, pipelines/front_ms 59.70, stage/render/prepare_resources 55.31, stream/drive/ms 51.90, stage/main/post_update 35.65, stage/render/prepare_bind_groups 32.87 |
| warm-up | 5.24 | 171.0 | pipelines/wait_ms 849, render/thread/ms 235, render/busy/ms 233, main/busy/ms 152, stage/main/update 109, stage/render/prepare_resources 95.22, pipelines/compile_ms 90.69, pipelines/driver_ms 60.07, stream/drive/ms 56.70, stage/render/prepare_assets 41.14, pipelines/front_ms 30.62, stage/main/post_update 26.99 |
| warm-up | 5.29 | 54.5 | render/thread/ms 197, render/busy/ms 195, main/busy/ms 104, stage/render/prepare_resources 72.83, stage/main/update 66.78, stream/drive/ms 57.99, stage/render/prepare_assets 50.32, stage/render/prepare_write_phase_buffers 21.51, stage/render/render 11.91, stage/main/post_update 11.89, stage/render/specialize 11.86, stage/main/pre_update 10.79 |
| warm-up | 5.39 | 93.4 | pipelines/compile_ms 1339, pipelines/driver_ms 1276, pipelines/wait_ms 534, main/busy/ms 134, render/thread/ms 133, render/busy/ms 132, stage/main/update 74.25, pipelines/front_ms 62.97, stream/drive/ms 51.07, stage/render/prepare_assets 38.49, stage/main/post_update 34.38, stage/render/prepare_resources 26.07 |
| warm-up | 5.47 | 84.7 | pipelines/compile_ms 1339, pipelines/driver_ms 1276, pipelines/wait_ms 534, main/busy/ms 150, render/thread/ms 147, render/busy/ms 145, stage/main/update 97.49, pipelines/front_ms 62.97, stream/drive/ms 50.80, stage/render/prepare_assets 47.45, stage/main/post_update 37.69, stage/render/prepare_flush 22.90 |
| warm-up | 5.57 | 100.5 | pipelines/wait_ms 1873, render/thread/ms 149, render/busy/ms 147, pipelines/compile_ms 134, main/busy/ms 122, pipelines/driver_ms 93.12, stage/main/update 79.33, stage/render/prepare_assets 42.85, pipelines/front_ms 41.06, stream/drive/ms 39.06, stage/render/prepare_write_phase_buffers 34.26, stage/main/post_update 24.05 |
| warm-up | 5.63 | 61.5 | pipelines/wait_ms 6019, pipelines/compile_ms 286, pipelines/driver_ms 212, render/thread/ms 137, render/busy/ms 136, main/busy/ms 112, pipelines/front_ms 73.10, stage/main/update 65.92, stream/drive/ms 38.57, stage/render/prepare_write_phase_buffers 35.89, stage/render/prepare_assets 34.32, stage/main/post_update 29.63 |
| warm-up | 5.80 | 46.5 | render/thread/ms 83.12, render/busy/ms 81.66, main/busy/ms 61.37, stage/main/post_update 30.16, stage/render/prepare_resources 30.11, stage/main/update 20.33, stage/render/prepare_write_phase_buffers 13.51, stage/render/prepare_assets 12.52, stage/render/render 9.00, stream/drive/ms 7.53, render/graph/ms 6.69, stage/main/fixed_main_loop 6.09 |
| warm-up | 5.86 | 54.1 | render/thread/ms 93.71, render/busy/ms 92.11, main/busy/ms 79.51, stage/main/post_update 46.09, stage/render/prepare_resources 27.32, stage/render/prepare_write_phase_buffers 20.41, stage/main/update 19.59, stage/render/prepare_assets 17.94, stage/render/render 12.43, stage/main/fixed_main_loop 7.85, stream/drive/ms 6.92, render/graph/ms 6.47 |
| warm-up | 5.95 | 50.7 | pipelines/wait_ms 4614, pipelines/compile_ms 220, pipelines/driver_ms 156, render/thread/ms 83.46, render/busy/ms 81.76, pipelines/front_ms 64.11, main/busy/ms 53.13, stage/render/prepare_resources 32.90, stage/main/post_update 27.50, bigspace/total/ms 15.80, stage/main/update 15.46, bigspace/high_precision/ms 14.84 |
| warm-up | 6.01 | 60.5 | pipelines/wait_ms 2397, render/thread/ms 106, render/busy/ms 104, main/busy/ms 75.62, stage/main/post_update 46.09, stage/render/prepare_resources 45.21, pipelines/compile_ms 40.26, bigspace/total/ms 29.14, bigspace/high_precision/ms 27.28, pipelines/front_ms 20.76, stage/main/update 19.78, pipelines/driver_ms 19.50 |
| warm-up | 6.24 | 108.6 | render/thread/ms 134, render/busy/ms 132, main/busy/ms 120, stage/main/post_update 53.48, stage/main/update 46.34, stage/render/prepare_write_phase_buffers 32.94, stage/render/prepare_assets 32.46, stage/render/prepare_batch_phases 29.80, stage/render/prepare_flush 13.42, stage/render/render 11.90, render/graph/ms 9.64, stream/drive/ms 5.58 |
| warm-up | 6.30 | 61.2 | render/thread/ms 149, render/busy/ms 147, main/busy/ms 108, stage/main/post_update 56.02, stage/render/prepare_assets 47.05, stage/main/update 30.63, stage/render/prepare_write_phase_buffers 25.53, stage/render/prepare_resources 21.03, stage/render/prepare_batch_phases 20.16, stage/render/prepare_flush 10.84, stage/render/render 10.10, stream/drive/ms 8.45 |
| warm-up | 6.35 | 47.7 | render/thread/ms 103, render/busy/ms 101, main/busy/ms 64.12, stage/render/prepare_assets 46.01, stage/main/post_update 29.08, stage/main/update 20.09, stage/render/prepare_resources 18.75, stage/render/render 10.05, stage/main/pre_update 9.06, stage/render/prepare_bind_groups 8.19, render/graph/ms 7.69, stream/drive/ms 7.25 |
| warm-up | 6.45 | 65.3 | render/thread/ms 94.92, render/busy/ms 93.48, main/busy/ms 75.10, stage/render/prepare_resources 35.68, stage/main/update 33.86, stage/main/post_update 28.61, stage/render/prepare_assets 17.47, stream/drive/ms 15.64, stage/render/prepare_batch_phases 10.34, stage/render/render 9.78, stage/render/prepare_flush 8.09, render/graph/ms 7.18 |
| warm-up | 6.52 | 64.0 | render/thread/ms 123, render/busy/ms 122, main/busy/ms 82.74, stage/render/prepare_assets 44.65, stage/main/update 37.74, stage/main/post_update 29.21, stage/render/prepare_resources 28.69, stream/drive/ms 15.02, stage/render/prepare_bind_groups 13.12, stage/main/fixed_main_loop 10.42, stage/render/prepare_batch_phases 8.69, stage/render/render 8.65 |
| warm-up | 6.59 | 70.9 | render/thread/ms 128, render/busy/ms 127, main/busy/ms 88.55, stage/main/post_update 50.95, stage/render/prepare_assets 48.15, stage/render/prepare_resources 33.99, bigspace/total/ms 24.46, stage/main/fixed_main_loop 16.82, bigspace/low_precision/ms 15.68, stage/main/update 14.41, stage/render/render 11.55, stage/render/prepare_write_phase_buffers 10.97 |
| warm-up | 6.66 | 69.4 | render/thread/ms 131, render/busy/ms 129, main/busy/ms 103, stage/main/post_update 55.22, stage/render/prepare_resources 47.30, stage/render/prepare_assets 43.32, stage/main/update 31.75, bigspace/total/ms 24.19, bigspace/low_precision/ms 15.56, stage/render/prepare_write_phase_buffers 13.62, stage/render/render 12.52, render/graph/ms 9.99 |
| warm-up | 6.98 | 48.6 | pipelines/compile_ms 3574, pipelines/driver_ms 3252, pipelines/front_ms 322, render/thread/ms 67.07, render/busy/ms 65.30, main/busy/ms 56.31, stage/main/update 29.33, stream/drive/ms 22.61, stage/main/post_update 20.66, stage/render/prepare_write_phase_buffers 15.96, stage/render/render 12.72, stage/render/prepare_assets 10.66 |
| warm-up | 7.20 | 55.8 | pipelines/wait_ms 3610, render/thread/ms 81.96, render/busy/ms 80.17, pipelines/compile_ms 42.10, main/busy/ms 40.70, pipelines/driver_ms 34.27, stage/render/prepare_resources 30.00, stage/main/post_update 27.80, stage/render/prepare_write_phase_buffers 18.69, stage/render/render 16.23, render/graph/ms 13.42, stage/render/graph_submit 9.73 |
| warm-up | 7.31 | 54.4 | pipelines/wait_ms 3652, pipelines/compile_ms 117, pipelines/driver_ms 99.58, render/thread/ms 81.00, render/busy/ms 79.52, main/busy/ms 33.92, stage/main/post_update 19.89, stage/render/prepare_resources 19.56, pipelines/front_ms 17.86, stage/render/render 16.76, render/graph/ms 14.53, stage/render/prepare_assets 12.39 |
| warm-up | 7.37 | 53.6 | pipelines/wait_ms 7643, pipelines/compile_ms 114, render/thread/ms 104, render/busy/ms 102, pipelines/driver_ms 92.80, main/busy/ms 57.87, stage/render/prepare_assets 35.04, stage/main/post_update 34.66, pipelines/front_ms 21.37, stage/render/prepare_resources 20.79, stage/render/render 18.17, render/graph/ms 15.74 |
| warm-up | 7.58 | 76.8 | pipelines/wait_ms 16.0 k, pipelines/compile_ms 95.16, main/busy/ms 84.92, render/thread/ms 75.01, render/busy/ms 73.59, pipelines/driver_ms 61.89, stage/main/update 41.04, pipelines/front_ms 33.27, stream/drive/ms 28.76, stage/main/post_update 23.87, stage/render/prepare_batch_phases 17.07, stage/main/fixed_main_loop 14.38 |
| warm-up | 7.78 | 64.3 | pipelines/wait_ms 8280, pipelines/compile_ms 204, pipelines/driver_ms 169, render/thread/ms 98.08, render/busy/ms 96.24, main/busy/ms 63.53, pipelines/front_ms 34.26, stage/render/prepare_resources 33.33, stage/main/update 21.59, stage/main/post_update 21.32, stage/main/fixed_main_loop 12.73, stage/render/prepare_assets 12.00 |
| warm-up | 8.06 | 44.9 | pipelines/wait_ms 39.7 k, pipelines/compile_ms 437, pipelines/driver_ms 357, pipelines/front_ms 80.06, render/thread/ms 77.26, render/busy/ms 75.35, main/busy/ms 65.76, stage/main/post_update 24.62, stage/main/update 22.19, stage/render/render 16.89, render/graph/ms 14.56, encode//sum 13.48 |
| warm-up | 8.11 | 54.3 | pipelines/wait_ms 26.3 k, pipelines/compile_ms 293, pipelines/driver_ms 228, render/thread/ms 89.29, render/busy/ms 87.48, main/busy/ms 71.59, pipelines/front_ms 64.54, stage/main/update 31.02, stage/main/post_update 22.20, stage/render/render 20.36, render/graph/ms 17.80, encode//sum 17.64 |
| warm-up | 8.26 | 47.2 | pipelines/wait_ms 7730, pipelines/compile_ms 4105, pipelines/driver_ms 3800, pipelines/front_ms 305, render/thread/ms 74.09, render/busy/ms 72.43, main/busy/ms 66.12, stage/main/post_update 31.07, stage/render/prepare_resources 28.21, stage/main/update 27.93, stage/render/render 19.36, render/graph/ms 15.57 |
| warm-up | 8.30 | 46.2 | pipelines/wait_ms 4237, pipelines/compile_ms 3792, pipelines/driver_ms 3560, pipelines/front_ms 232, render/thread/ms 82.21, render/busy/ms 80.56, main/busy/ms 62.95, stage/main/update 26.03, stage/render/prepare_resources 25.73, stage/main/post_update 22.58, stage/render/prepare_assets 19.38, stage/render/render 17.74 |
| warm-up | 8.35 | 49.3 | pipelines/wait_ms 4287, pipelines/compile_ms 88.95, render/thread/ms 84.71, render/busy/ms 83.06, main/busy/ms 62.22, pipelines/driver_ms 51.17, pipelines/front_ms 37.78, stage/main/post_update 34.40, stage/render/prepare_assets 17.77, stage/render/prepare_resources 15.80, stage/render/render 15.26, stage/main/update 14.10 |
| warm-up | 8.41 | 53.9 | pipelines/wait_ms 4287, main/busy/ms 103, render/thread/ms 96.96, render/busy/ms 95.38, pipelines/compile_ms 88.95, pipelines/driver_ms 51.17, stage/main/post_update 44.95, pipelines/front_ms 37.78, stage/render/prepare_resources 30.04, stage/main/update 20.66, stage/main/pre_update 17.43, stage/main/last 15.90 |
| warm-up | 8.54 | 128.4 | render/thread/ms 163, render/busy/ms 161, main/busy/ms 155, stage/main/post_update 72.32, stage/render/prepare_resources 49.46, stage/render/prepare_write_phase_buffers 43.57, stage/main/update 39.96, stream/drive/ms 23.95, stage/render/prepare_assets 22.53, stage/main/pre_update 17.33, stage/main/last 16.08, stage/render/render 14.16 |
| warm-up | 8.67 | 45.5 | render/thread/ms 66.02, render/busy/ms 64.39, main/busy/ms 53.41, stage/main/post_update 27.62, stage/render/prepare_flush 19.71, stage/main/update 18.26, stage/render/render 14.59, render/graph/ms 12.11, encode//sum 11.14, stage/render/prepare_write_phase_buffers 10.83, stage/render/graph_submit 8.23, stage/render/prepare_assets 6.93 |
| warm-up | 9.31 | 63.2 | render/thread/ms 68.43, render/busy/ms 66.80, main/busy/ms 47.39, stage/main/update 23.68, stage/render/specialize 17.30, stage/main/post_update 16.66, stage/render/render 14.89, stage/render/queue 13.86, encode//sum 13.56, stream/drive/ms 13.41, render/graph/ms 12.27, stage/render/graph_submit 8.49 |
| warm-up | 10.05 | 45.2 | main/busy/ms 59.19, stage/main/post_update 45.64, render/thread/ms 31.58, render/busy/ms 30.19, stage/render/render 12.01, encode//sum 10.15, render/graph/ms 9.86, stage/main/update 9.02, stage/render/graph_submit 6.21, stage/render/prepare_assets 5.09, encode/submit 4.45, stage/render/prepare_resources 4.31 |
| warm-up | 11.77 | 56.6 | pipelines/compile_ms 2549, pipelines/driver_ms 2360, pipelines/front_ms 189, render/thread/ms 71.66, render/busy/ms 69.96, main/busy/ms 39.40, stage/main/post_update 21.58, stage/render/render 18.53, stage/render/specialize 17.65, render/graph/ms 15.97, encode//sum 14.30, stage/render/queue 13.22 |
| warm-up | 12.21 | 56.8 | render/thread/ms 92.19, render/busy/ms 90.44, main/busy/ms 49.41, stage/render/prepare_resources 36.23, stage/main/post_update 32.77, stage/render/render 21.64, render/graph/ms 19.27, stage/render/graph_submit 15.83, encode/submit 13.74, encode//sum 12.28, stage/main/update 10.56, stage/render/prepare_assets 9.97 |
| warm-up | 12.39 | 78.2 | render/thread/ms 103, render/busy/ms 102, main/busy/ms 44.33, stage/render/prepare_resources 30.35, stage/render/prepare_write_phase_buffers 29.08, stage/main/post_update 27.28, encode//sum 12.37, stage/render/render 12.32, stage/render/prepare_bind_groups 12.32, render/graph/ms 10.13, stage/main/update 9.48, stage/render/prepare_flush 8.71 |
| warm-up | 12.45 | 58.7 | render/thread/ms 127, render/busy/ms 126, main/busy/ms 50.97, stage/render/prepare_resources 40.06, stage/render/prepare_write_phase_buffers 31.45, stage/main/post_update 26.15, stage/render/render 19.97, render/graph/ms 17.50, stage/render/prepare_flush 14.19, encode//sum 13.39, stage/main/update 10.27, stage/render/camera_driver 9.41 |
| warm-up | 12.92 | 60.3 | render/thread/ms 99.42, render/busy/ms 97.75, main/busy/ms 47.00, stage/render/prepare_resources 25.51, stage/main/update 22.26, stage/main/post_update 19.59, stage/render/prepare_bind_groups 17.68, stage/render/render 17.19, render/graph/ms 14.92, stage/render/prepare_write_phase_buffers 14.81, encode//sum 13.81, stage/render/prepare_flush 12.93 |
| measure | 0.96 | 53.1 | pipelines/wait_ms 9696, pipelines/compile_ms 118, pipelines/driver_ms 79.71, render/thread/ms 78.65, render/busy/ms 77.12, main/busy/ms 67.71, stage/main/post_update 42.74, pipelines/front_ms 37.86, stage/render/render 33.80, render/graph/ms 31.64, encode//sum 19.61, stage/main/update 18.04 |
| measure | 1.01 | 45.5 | pipelines/wait_ms 9696, pipelines/compile_ms 118, render/thread/ms 91.27, render/busy/ms 89.83, pipelines/driver_ms 79.71, main/busy/ms 71.17, stage/main/post_update 47.56, pipelines/front_ms 37.86, stage/render/render 35.23, render/graph/ms 32.94, encode//sum 21.99, stage/render/prepare_assets 17.89 |
| measure | 1.11 | 58.3 | render/thread/ms 73.45, render/busy/ms 71.91, main/busy/ms 35.80, stage/main/post_update 22.30, stage/render/render 20.90, encode//sum 19.44, render/graph/ms 18.61, stage/render/specialize 17.05, stage/render/queue 13.89, stage/render/graph_submit 13.53, encode/submit 9.79, stage/main/update 8.54 |

## Run

- `../target/profiling/walkonce.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\wgpu_walk_once\balmora_walk@2560x1440_arm=onetex-2 --bench-baseline none --off texvariety
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1364 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **77%** (min 20%) · 2615 MHz (min 2100) · memory 8101 MHz · 80.0 W · 77 °C (max 83) · VRAM 5976 MiB · throttled 97% of samples

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
