# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5276 frames over 65.6 s · **median 12.09 ms (83 fps)** · mean 12.44 · p95 15.53 · p99 18.83 (1% low 53 fps) · worst 45.46 ms · 65 spikes over 1.5× median · 48.4 stutters/min (work 47.5/min)
- warm-up: 323 frames over 15.6 s, worst 2507.9 ms
- hitches (> 30.2 ms): **28** measured, 138 in warm-up
- timed GPU passes: 7.01 ms of a 12.44 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.09 ms, limit 11 ms
- **FAIL** frame_ms: 53 stutters (> 1.5x moving median of 60) in 1.09 min = 48.44/min, limit 1/min
- **FAIL** worst frame 45.46 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.975 | 2.034 | 2.065 | 0.97 | 0.181 |
| main_opaque_pass_3d | 1.607 | 1.655 | 2.049 | 0.97 | 0.067 |
| early prepass | 0.483 | 0.498 | 0.715 | 0.97 | 0.093 |
| bloom | 0.306 | 0.315 | 0.321 | 0.97 | 0.066 |
| atmosphere_luts | 0.297 | 0.306 | 0.366 | 0.97 | 0.004 |
| water_velocity | 0.257 | 0.264 | 0.347 | 0.97 | 0.002 |
| water_sspr | 0.196 | 0.202 | 0.203 | 0.97 | 0.002 |
| ssao | 0.183 | 0.189 | 0.193 | 0.97 | 0.007 |
| auxiliary_views | 0.144 | 0.149 | 0.523 | 0.97 | 0.154 |
| atmos_froxel_inject | 0.142 | 0.146 | 0.148 | 0.97 | 0.001 |
| clouds_trace | 0.137 | 0.141 | 0.228 | 0.97 | 0.001 |
| uniform_allocation | 0.128 | 0.132 | 0.184 | 0.97 | 0.026 |
| water_fft | 0.125 | 0.129 | 0.137 | 0.97 | 0.003 |
| upscaling | 0.114 | 0.117 | 0.124 | 0.97 | 0.004 |
| bin_unpacking | 0.103 | 0.106 | 0.142 | 0.97 | 0.019 |
| clouds_composite | 0.102 | 0.105 | 0.108 | 0.97 | 0.002 |
| tonemapping | 0.082 | 0.085 | 0.089 | 0.97 | 0.004 |
| main_transmissive_pass_3d | 0.077 | 0.079 | 0.158 | 0.97 | 0.009 |
| clustering | 0.062 | 0.064 | 0.068 | 0.97 | 0.019 |
| render_sky | 0.050 | 0.051 | 0.052 | 0.97 | 0.002 |
| clouds_resolve | 0.047 | 0.048 | 0.049 | 0.97 | 0.001 |
| surface_exposure | 0.043 | 0.044 | 0.045 | 0.97 | 0.001 |
| surface_trail | 0.043 | 0.056 | 0.057 | 0.76 | 0.002 |
| lightprobe_irradiance_map | 0.041 | 0.412 | 0.415 | 0.10 | 0.001 |
| surface_exposure_far | 0.037 | 0.043 | 0.045 | 0.86 | 0.002 |
| early_mesh_preprocessing | 0.030 | 0.031 | 0.047 | 0.97 | 0.005 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.97 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.97 | 0.001 |
| water_snapshot | 0.029 | 0.030 | 0.034 | 0.97 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.219 | 0.230 | 0.10 | 0.005 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.97 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.032 | 0.97 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.025 | 0.97 | 0.124 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.97 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.97 | 0.003 |
| lightprobe_copy | 0.001 | 0.014 | 0.015 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/compile_ms | 15.168 | 0.000 | 5587.90 | 80026.4 |
| pipelines/driver_ms | 13.227 | 0.000 | 5227.48 | 69786.4 |
| render/thread/ms | 11.080 | 13.985 | 40.08 | 58455.9 |
| render/busy/ms | 10.598 | 13.494 | 39.56 | 55914.5 |
| encode//sum | 10.587 | 15.341 | 53.75 | 55856.7 |
| main/busy/ms | 9.708 | 12.268 | 34.31 | 51220.0 |
| pipelines/wait_ms | 9.378 | 0.000 | 8185.17 | 49479.2 |
| stage/render/render | 6.877 | 8.889 | 17.69 | 36283.0 |
| render/graph/ms | 6.197 | 8.143 | 16.66 | 32693.4 |
| stage/main/post_update | 6.088 | 8.292 | 24.41 | 32121.0 |
| stage/render/graph_submit | 4.965 | 6.602 | 11.99 | 26196.0 |
| encode/finish_wall | 2.666 | 3.927 | 5.88 | 14064.2 |
| encode/submit | 2.336 | 3.252 | 6.46 | 12322.1 |
| stage/main/update | 2.268 | 2.769 | 27.60 | 11966.3 |
| pipelines/front_ms | 1.941 | 0.000 | 360.42 | 10240.0 |
| stage/render/camera_driver | 1.173 | 1.652 | 4.59 | 6188.9 |
| stage/render/prepare_write_phase_buffers | 0.798 | 1.169 | 3.54 | 4209.3 |
| stage/render/prepare_assets | 0.601 | 0.778 | 10.05 | 3172.9 |
| bigspace/total/ms | 0.577 | 1.602 | 5.30 | 3046.6 |
| stage/render/prepare_meshes | 0.466 | 0.590 | 6.77 | 2456.1 |
| stage/render/prepare_resources | 0.460 | 0.562 | 1.02 | 2428.0 |
| stage/render/specialize | 0.438 | 0.620 | 12.93 | 2308.8 |
| stage/main/first | 0.424 | 0.560 | 6.16 | 2238.8 |
| render/present/ms | 0.383 | 0.474 | 3.89 | 2021.3 |
| stage/main/pre_update | 0.369 | 0.549 | 1.21 | 1946.4 |
| stage/render/queue | 0.320 | 0.460 | 8.75 | 1685.8 |
| terrain/update/ms | 0.312 | 0.401 | 1.50 | 1648.0 |
| stage/render/prepare_bind_groups | 0.299 | 0.542 | 2.51 | 1576.2 |
| controller/feed/ms | 0.278 | 0.361 | 0.94 | 1464.8 |
| terrain/select/ms | 0.277 | 0.356 | 1.04 | 1462.1 |
| stream/drive/ms | 0.265 | 0.370 | 25.58 | 1396.2 |
| stage/main/fixed_main_loop | 0.248 | 0.393 | 1.27 | 1309.6 |
| bigspace/low_precision/ms | 0.242 | 0.664 | 5.09 | 1277.2 |
| far/select/ms | 0.210 | 0.296 | 0.66 | 1106.7 |
| stage/render/create_views | 0.205 | 0.274 | 1.05 | 1081.7 |
| stream/tick/ms | 0.202 | 0.282 | 0.48 | 1064.4 |
| stage/render/prepare_batch_phases | 0.199 | 0.283 | 0.80 | 1050.3 |
| stage/render/extract_commands | 0.178 | 0.217 | 0.51 | 940.9 |
| bigspace/high_precision/ms | 0.175 | 0.597 | 4.17 | 925.4 |
| stage/render/prepare_flush | 0.166 | 0.310 | 0.74 | 875.5 |
| anim/evaluate/ms | 0.163 | 0.267 | 2.74 | 861.3 |
| anim/evaluate_cpu/ms | 0.149 | 0.192 | 0.68 | 784.9 |
| stage/render/prepare_views | 0.143 | 0.182 | 0.40 | 755.8 |
| controller/move/ms | 0.133 | 0.250 | 0.57 | 701.0 |
| stage/main/last | 0.126 | 0.202 | 1.96 | 662.6 |
| render/acquire/ms | 0.099 | 0.127 | 0.34 | 520.1 |
| stage/render/prepare_collect_phase_buffers | 0.068 | 0.142 | 0.41 | 359.6 |
| physics/step/ms | 0.055 | 0.093 | 0.50 | 288.5 |
| physics/insert/ms | 0.040 | 0.405 | 0.83 | 208.7 |
| encode/resolve_diagnostics/sum | 0.034 | 0.046 | 0.47 | 178.1 |
| stage/render/cleanup | 0.033 | 0.099 | 0.70 | 171.9 |
| stage/render/phase_sort | 0.027 | 0.048 | 0.18 | 144.9 |
| weather/map/ms | 0.026 | 0.036 | 0.22 | 139.6 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.024 | 0.31 | 84.4 |
| anim/apply/ms | 0.012 | 0.024 | 0.44 | 63.9 |
| encode/camera_driver_span_end/sum | 0.012 | 0.021 | 0.56 | 62.4 |
| controller/avoid/ms | 0.011 | 0.015 | 0.17 | 57.9 |
| weather/eval/ms | 0.011 | 0.015 | 0.21 | 57.8 |
| script/update/ms | 0.008 | 0.010 | 0.15 | 40.2 |
| stream/despawn/ms | 0.006 | 0.000 | 1.05 | 30.2 |
| stream/spawn/ms | 0.006 | 0.000 | 1.12 | 29.5 |
| script/ui/ms | 0.005 | 0.007 | 0.04 | 28.8 |
| script/input/ms | 0.005 | 0.007 | 0.17 | 25.7 |
| controller/write/ms | 0.003 | 0.004 | 0.06 | 17.7 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.67 | 14.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.40 | 4.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 4.4 |
| stage/render/prepare | 0.001 | 0.002 | 0.13 | 3.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.9 |
| stream/upload/ms | 0.000 | 0.000 | 0.33 | 1.5 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.14 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 10 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 19 |
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1285 | 2262 | 535 |
| crowd/c0/meshes/count | count | 349 | 646 | 144 |
| crowd/c1/count | count | 146 | 155 | 149 |
| crowd/c1/joints/count | count | 5531 | 5902 | 5661 |
| crowd/c1/meshes/count | count | 1155 | 1256 | 1197 |
| crowd/c2/count | count | 394 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4614 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4596 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.12 | 5.35 | 1.83 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.31 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.56 | 0.01 |
| encode/finish_critical | ms | 2.12 | 5.35 | 1.83 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.47 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 10.51 | 39.56 | 8.94 |
| gpu/timed/ms | ms | 7.93 | 11.50 | 8.19 |
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
| phases/alpha_mask3d/items | count | 799 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 799 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1297 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3360 | 5117 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1297 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3360 | 5117 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 18 | 203 | 4 |
| phases/shadow/bins | count | 4979 | 8798 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.3 k | 26.4 k | 6376 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 203 | 250 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 290 | 330 | 330 |
| pipelines/waiting | count | 1 | 7 | 0 |
| precache/draw_waiting | count | 0 | 6 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 1 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 27 | 69 | 0 |
| preprocess/bind_groups | count | 15 | 567 | 4 |
| preprocess/bind_groups_created | count | 15 | 567 | 4 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 215 | 125 |
| process/cpu_usage | % | 30.77 | 77.40 | 25 |
| process/mem_usage | GiB | 5.64 | 5.86 | 5.86 |
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
| skin/blocks_changed/count | count | 13 | 128 | 14 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 366.6 k | 807.7 k | 490.5 k |
| skin/joints_written/count | count | 5728 | 12.6 k | 7664 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4920 | 5011 | 5011 |
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
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.68 | 76.99 | 32.49 |
| system/mem_usage | % | 66.92 | 67.63 | 67.52 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 68 | 252 | 54 |
| views/all/bins | count | 7510 | 11.6 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 246 | 299 | 75 |
| views/all/pipelines_distinct | count | 70 | 75 | 61 |
| views/all/work_items | count | 10.6 k | 35.9 k | 8123 |
| views/cascade/batch_sets | count | 9 | 53 | 4 |
| views/cascade/bins | count | 2798 | 6140 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8870 | 23.5 k | 6376 |
| views/cascade/pipelines | count | 36 | 53 | 29 |
| views/cascade/work_items | count | 412 | 23.5 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 195 | 0 |
| views/lamp_face/bins | count | 2168 | 2818 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2401 | 3113 | 0 |
| views/lamp_face/pipelines | count | 168 | 204 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2927 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2896 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8322 | 13.0 k | 7614 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8492 | 13.2 k | 7922 |
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
| anim/bones_written/count | count | 53.20 | 293 | 280.7 k |
| anim/events/count | count | 0.10 | 4 | 526 |
| assets/Image/added | count | 0.11 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.77 | 152 | 4061 |
| assets/Mesh/removed | count | 0.92 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 464 |
| assets/StandardMaterial/modified | count | 4.11 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 10.05 | 1989 | 53.0 k |
| ecs/entities_delta | count | 0.89 | 1272 | 4674 |
| encode//count | count | 40.22 | 273 | 212.2 k |
| encode/bind_walk/init_textures | count | 74.9 k | 199.8 k | 394.96 M |
| encode/bind_walk/init_textures_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/inits | count | 566 | 1246 | 2.99 M |
| encode/bind_walk/inits_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/merge_textures | count | 18.0 k | 119.4 k | 95.06 M |
| encode/bind_walk/merge_textures_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/merges | count | 181 | 413 | 954.8 k |
| encode/bind_walk/merges_skipped | count | 0 | 0 | 0 |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5276 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5276 |
| encode/queue/command_buffers | count | 45.24 | 278 | 238.7 k |
| encode/queue/submits | count | 2.02 | 4 | 10.7 k |
| encode/queue/write_buffer/bytes | bytes | 1.90 M | 4.88 M | 10.02 G |
| encode/queue/write_buffer/calls | count | 261 | 847 | 1.38 M |
| encode/queue/write_buffer_with/bytes | bytes | 178.7 k | 3.52 M | 942.92 M |
| encode/queue/write_buffer_with/calls | count | 11.29 | 465 | 59.6 k |
| encode/queue/write_texture/bytes | bytes | 11.8 k | 8.59 M | 62.48 M |
| encode/queue/write_texture/calls | count | 0.11 | 42 | 562 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5276 |
| physics/inserts | count | 0.43 | 111 | 2290 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.44 | 161 | 2335 |
| physics/steps | count | 0.75 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 7 |
| pipelines/created | count | 0.02 | 2 | 114 |
| precache/hit | count | 0.00 | 2 | 4 |
| precache/missed | count | 0.00 | 2 | 6 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 18 | 378 |
| stream/upload/bytes | bytes | 6416 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.01 | 1 | 28 |
| surface/shelter_redraws | count | 0.01 | 1 | 55 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1407 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.2 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2507.9 | main/busy/ms 2481, stage/main/post_update 2373, stage/main/update 99.40, surface/advance_cpu_ms 69.10, stage/main/pre_update 3.93, stage/main/last 0.99, surface/cell_upload_ms 0.76, stage/main/first 0.60, stage/main/fixed_main_loop 0.49, stream/drive/ms 0.41, weather/map/ms 0.24, stream/tick/ms 0.12 |
| warm-up | 3.22 | 714.0 | main/busy/ms 2614, stage/main/post_update 2396, render/thread/ms 681, render/busy/ms 521, stage/render/prepare_assets 473, stage/main/update 205, stage/render/prepare_views 157, render/acquire/ms 157, surface/advance_cpu_ms 69.10, stage/render/create_views 27.96, stage/render/render 12.19, render/graph/ms 8.16 |
| warm-up | 3.35 | 131.6 | render/thread/ms 788, render/busy/ms 578, stage/render/prepare_assets 498, pipelines/compile_ms 383, pipelines/driver_ms 295, main/busy/ms 160, stage/render/prepare_views 157, render/acquire/ms 157, stage/main/update 111, pipelines/front_ms 87.92, stage/render/render 78.28, render/present/ms 53.62 |
| warm-up | 3.48 | 75.3 | pipelines/compile_ms 486, pipelines/driver_ms 383, render/thread/ms 123, render/busy/ms 106, pipelines/front_ms 103, main/busy/ms 55.24, encode//sum 50.75, stage/render/render 42.46, stage/main/update 25.23, render/graph/ms 24.42, stage/main/post_update 22.61, stage/render/prepare_write_phase_buffers 21.50 |
| warm-up | 3.61 | 132.1 | pipelines/compile_ms 1746, pipelines/driver_ms 1480, pipelines/wait_ms 1095, pipelines/front_ms 266, render/thread/ms 176, render/busy/ms 174, main/busy/ms 117, stage/main/update 85.38, stage/render/prepare_write_phase_buffers 45.11, stage/render/prepare_resources 34.36, stage/render/prepare_assets 32.44, encode//sum 29.36 |
| warm-up | 3.75 | 139.0 | pipelines/wait_ms 3700, pipelines/compile_ms 2386, pipelines/driver_ms 2016, pipelines/front_ms 369, render/thread/ms 237, render/busy/ms 234, main/busy/ms 103, stage/render/prepare_assets 98.44, stage/main/update 69.86, stage/render/prepare_resources 62.23, stage/main/post_update 27.53, stage/render/prepare_write_phase_buffers 25.70 |
| warm-up | 3.83 | 77.8 | pipelines/wait_ms 3385, pipelines/compile_ms 859, pipelines/driver_ms 706, render/thread/ms 210, render/busy/ms 205, pipelines/front_ms 153, stage/render/prepare_assets 114, stage/render/prepare_resources 53.00, main/busy/ms 49.31, stage/main/post_update 21.79, stage/main/fixed_main_loop 15.10, stage/render/prepare_write_phase_buffers 13.79 |
| warm-up | 3.93 | 80.3 | pipelines/wait_ms 229, pipelines/compile_ms 167, pipelines/driver_ms 153, render/thread/ms 94.38, render/busy/ms 92.58, main/busy/ms 56.13, stage/main/post_update 38.48, stage/render/prepare_meshes 22.01, stage/render/prepare_resources 20.50, stage/render/prepare_batch_phases 18.26, bigspace/total/ms 14.17, pipelines/front_ms 13.51 |
| warm-up | 4.06 | 76.0 | pipelines/compile_ms 697, pipelines/driver_ms 578, render/thread/ms 123, render/busy/ms 121, pipelines/front_ms 119, pipelines/wait_ms 117, main/busy/ms 51.67, stage/render/prepare_flush 29.69, stage/main/post_update 29.34, stage/render/prepare_assets 27.81, stage/render/prepare_resources 27.73, stage/render/prepare_write_phase_buffers 12.68 |
| warm-up | 4.22 | 101.5 | pipelines/wait_ms 588, pipelines/compile_ms 220, pipelines/driver_ms 194, main/busy/ms 132, render/thread/ms 129, render/busy/ms 127, stage/main/post_update 71.74, stage/render/prepare_assets 62.04, stage/main/update 52.45, stream/drive/ms 48.25, stage/render/prepare_resources 31.47, bigspace/total/ms 30.64 |
| warm-up | 4.28 | 58.4 | pipelines/wait_ms 1396, pipelines/compile_ms 279, pipelines/driver_ms 238, render/thread/ms 125, render/busy/ms 124, main/busy/ms 111, stage/main/post_update 97.82, stage/render/prepare_assets 76.08, pipelines/front_ms 40.54, bigspace/total/ms 30.64, bigspace/high_precision/ms 30.45, stage/render/prepare_resources 18.24 |
| warm-up | 4.33 | 58.1 | pipelines/wait_ms 808, render/thread/ms 102, render/busy/ms 101, main/busy/ms 84.09, stage/main/post_update 61.02, pipelines/compile_ms 58.50, stage/render/prepare_assets 54.29, pipelines/driver_ms 43.96, stage/render/prepare_resources 21.13, pipelines/front_ms 14.54, bigspace/total/ms 12.44, bigspace/high_precision/ms 12.29 |
| warm-up | 4.40 | 65.0 | pipelines/wait_ms 2875, pipelines/compile_ms 146, render/thread/ms 115, render/busy/ms 114, pipelines/driver_ms 105, main/busy/ms 70.66, stage/main/post_update 45.56, stage/render/prepare_resources 41.49, pipelines/front_ms 41.40, stage/render/prepare_assets 38.39, stage/render/prepare_write_phase_buffers 16.10, bigspace/total/ms 13.40 |
| warm-up | 4.54 | 92.3 | pipelines/wait_ms 2101, pipelines/compile_ms 1378, pipelines/driver_ms 1261, pipelines/front_ms 117, render/thread/ms 107, render/busy/ms 105, main/busy/ms 79.58, stage/main/update 50.94, stage/render/prepare_assets 47.93, stage/main/post_update 19.75, stage/render/prepare_flush 17.93, stage/render/prepare_write_phase_buffers 13.02 |
| warm-up | 4.64 | 96.0 | pipelines/wait_ms 4009, pipelines/compile_ms 1555, pipelines/driver_ms 1380, pipelines/front_ms 175, render/thread/ms 143, render/busy/ms 141, main/busy/ms 73.51, stage/render/prepare_assets 71.32, stage/main/update 34.11, stage/main/post_update 32.93, stage/render/render 25.27, render/graph/ms 22.56 |
| warm-up | 4.75 | 117.6 | pipelines/wait_ms 1987, pipelines/compile_ms 259, render/thread/ms 186, render/busy/ms 184, pipelines/driver_ms 169, main/busy/ms 90.01, pipelines/front_ms 89.86, stage/main/post_update 67.46, stage/render/prepare_resources 49.52, stage/render/render 37.10, render/graph/ms 33.90, stage/render/prepare_assets 31.51 |
| warm-up | 4.82 | 63.5 | render/thread/ms 165, render/busy/ms 162, main/busy/ms 108, pipelines/compile_ms 82.00, pipelines/wait_ms 79.46, stage/main/post_update 65.39, stage/render/prepare_resources 60.59, pipelines/driver_ms 50.53, pipelines/front_ms 31.47, stage/render/prepare_flush 25.97, stage/render/render 24.85, render/graph/ms 21.47 |
| warm-up | 4.89 | 73.6 | pipelines/wait_ms 1748, pipelines/compile_ms 1244, pipelines/driver_ms 1171, render/thread/ms 124, render/busy/ms 122, main/busy/ms 101, pipelines/front_ms 73.31, stage/render/prepare_resources 39.23, stage/main/post_update 38.73, stage/main/update 33.23, stage/render/prepare_assets 28.86, encode//sum 26.78 |
| warm-up | 5.00 | 111.4 | pipelines/wait_ms 1748, pipelines/compile_ms 1244, pipelines/driver_ms 1171, render/thread/ms 158, render/busy/ms 157, main/busy/ms 149, stage/main/post_update 82.78, pipelines/front_ms 73.31, stage/render/prepare_resources 54.29, stage/main/update 40.65, stage/render/prepare_assets 35.71, stream/drive/ms 29.88 |
| warm-up | 5.09 | 89.0 | pipelines/wait_ms 4831, pipelines/compile_ms 187, main/busy/ms 175, render/thread/ms 172, render/busy/ms 170, pipelines/driver_ms 149, stage/main/update 72.61, stage/main/post_update 72.55, stage/render/prepare_resources 48.91, pipelines/front_ms 38.18, stage/render/prepare_flush 35.56, stage/render/prepare_assets 33.70 |
| warm-up | 5.27 | 174.8 | pipelines/wait_ms 8289, pipelines/compile_ms 276, render/thread/ms 212, render/busy/ms 210, pipelines/driver_ms 180, main/busy/ms 157, stage/main/update 102, pipelines/front_ms 95.77, stage/render/prepare_assets 87.68, stage/render/prepare_write_phase_buffers 63.18, stream/drive/ms 54.38, stage/main/post_update 24.07 |
| warm-up | 5.35 | 84.6 | pipelines/wait_ms 3458, render/thread/ms 214, render/busy/ms 213, main/busy/ms 117, pipelines/compile_ms 89.00, stage/render/prepare_assets 77.30, stage/render/prepare_write_phase_buffers 63.33, stage/main/update 57.70, pipelines/front_ms 57.59, stream/drive/ms 43.60, pipelines/driver_ms 31.40, stage/main/post_update 24.66 |
| warm-up | 5.47 | 121.7 | render/thread/ms 196, render/busy/ms 194, main/busy/ms 156, stage/main/update 88.27, stream/drive/ms 60.14, stage/render/prepare_batch_phases 47.51, stage/render/prepare_flush 33.82, stage/main/post_update 32.23, stage/render/prepare_assets 24.85, stage/render/prepare_bind_groups 20.20, stage/render/prepare_resources 19.73, stage/render/prepare_write_phase_buffers 16.52 |
| warm-up | 5.62 | 152.2 | render/thread/ms 244, render/busy/ms 242, main/busy/ms 235, stage/main/update 155, stream/drive/ms 113, stage/render/prepare_batch_phases 70.85, stage/main/post_update 57.32, stage/render/prepare_assets 53.78, stage/render/prepare_flush 27.89, stage/render/prepare_write_phase_buffers 27.61, stage/render/prepare_resources 27.57, bigspace/total/ms 26.49 |
| warm-up | 5.80 | 179.2 | render/thread/ms 264, render/busy/ms 263, main/busy/ms 164, stage/render/prepare_assets 119, stage/main/update 88.89, stream/drive/ms 65.86, stage/main/post_update 50.99, stage/render/prepare_write_phase_buffers 41.78, stage/render/prepare_resources 29.39, bigspace/total/ms 26.46, stage/render/prepare_batch_phases 23.68, stage/main/fixed_main_loop 16.73 |
| warm-up | 5.91 | 105.9 | render/thread/ms 232, render/busy/ms 231, main/busy/ms 138, stage/render/prepare_assets 119, stage/main/update 64.16, stream/drive/ms 43.23, stage/render/prepare_resources 39.12, stage/render/prepare_write_phase_buffers 34.52, stage/main/post_update 26.85, stage/main/pre_update 25.66, stage/main/fixed_main_loop 18.59, stage/render/render 11.28 |
| warm-up | 6.00 | 94.5 | render/thread/ms 180, render/busy/ms 179, main/busy/ms 165, stage/render/prepare_assets 75.66, stage/main/update 69.56, stage/main/post_update 62.89, stream/drive/ms 43.49, stage/render/prepare_resources 39.26, stage/main/pre_update 22.33, stage/render/prepare_write_phase_buffers 20.96, stage/render/prepare_flush 16.04, stage/render/prepare_bind_groups 9.29 |
| warm-up | 6.13 | 72.3 | render/thread/ms 103, render/busy/ms 101, main/busy/ms 75.45, stage/render/prepare_assets 54.73, stage/main/post_update 39.55, stage/main/update 24.12, stage/render/render 9.44, bigspace/total/ms 9.13, stage/render/prepare_write_phase_buffers 9.08, bigspace/high_precision/ms 8.60, stage/render/prepare_resources 8.52, render/graph/ms 7.06 |
| warm-up | 6.21 | 79.8 | render/thread/ms 136, render/busy/ms 134, main/busy/ms 59.31, stage/render/prepare_assets 37.30, stage/main/post_update 28.58, stage/render/prepare_batch_phases 26.56, stage/render/prepare_resources 21.95, stage/main/update 18.06, stage/render/prepare_write_phase_buffers 17.24, stage/render/render 10.40, bigspace/total/ms 9.34, bigspace/high_precision/ms 8.63 |
| warm-up | 6.27 | 65.5 | pipelines/wait_ms 1779, pipelines/compile_ms 1096, pipelines/driver_ms 988, render/thread/ms 135, render/busy/ms 134, pipelines/front_ms 108, main/busy/ms 47.14, stage/render/prepare_resources 43.02, stage/render/prepare_batch_phases 26.63, stage/render/prepare_assets 20.07, stage/main/update 19.88, stage/render/prepare_write_phase_buffers 17.47 |
| warm-up | 6.39 | 62.6 | pipelines/wait_ms 15.0 k, pipelines/compile_ms 2783, pipelines/driver_ms 2653, pipelines/front_ms 130, render/thread/ms 107, render/busy/ms 106, main/busy/ms 65.48, stage/main/post_update 32.88, stage/render/prepare_resources 27.58, stage/render/prepare_assets 22.48, stage/render/prepare_write_phase_buffers 21.64, stage/main/update 20.08 |
| warm-up | 6.47 | 75.0 | pipelines/wait_ms 11.9 k, pipelines/compile_ms 2716, pipelines/driver_ms 2502, pipelines/front_ms 213, render/thread/ms 122, render/busy/ms 120, main/busy/ms 75.08, stage/main/post_update 43.26, stage/render/prepare_assets 36.02, stage/render/queue 25.73, stage/main/update 18.40, stage/render/prepare_write_phase_buffers 16.33 |
| warm-up | 6.77 | 63.7 | pipelines/wait_ms 13.8 k, pipelines/compile_ms 118, render/thread/ms 72.74, render/busy/ms 71.28, pipelines/driver_ms 66.36, main/busy/ms 63.29, pipelines/front_ms 52.07, stage/render/prepare_assets 40.92, stage/main/update 29.66, stage/main/post_update 26.43, stage/render/render 12.61, render/graph/ms 10.43 |
| warm-up | 6.96 | 57.4 | pipelines/compile_ms 3602, pipelines/driver_ms 3283, pipelines/front_ms 319, render/thread/ms 89.76, render/busy/ms 88.38, main/busy/ms 75.11, stage/render/prepare_assets 44.19, stage/main/post_update 37.28, stage/main/update 26.06, stage/render/prepare_resources 16.53, stream/drive/ms 15.50, bigspace/total/ms 13.26 |
| warm-up | 7.07 | 67.0 | pipelines/wait_ms 6982, pipelines/compile_ms 220, pipelines/driver_ms 175, render/thread/ms 100, render/busy/ms 98.82, stage/render/prepare_assets 64.40, main/busy/ms 57.11, pipelines/front_ms 44.76, stage/main/post_update 40.13, bigspace/total/ms 17.20, bigspace/high_precision/ms 16.69, stage/render/render 11.76 |
| warm-up | 7.96 | 65.9 | render/thread/ms 109, render/busy/ms 107, main/busy/ms 59.00, stage/main/post_update 42.99, stage/render/prepare_write_phase_buffers 25.84, stage/render/prepare_assets 22.27, stage/render/prepare_resources 20.53, stage/render/render 13.31, encode//sum 11.66, render/graph/ms 10.74, stage/render/prepare_flush 10.01, stage/main/update 9.98 |
| warm-up | 8.41 | 94.0 | pipelines/wait_ms 13.8 k, pipelines/compile_ms 4911, pipelines/driver_ms 4674, pipelines/front_ms 237, render/thread/ms 135, render/busy/ms 133, main/busy/ms 70.23, stage/main/post_update 44.26, stage/render/prepare_resources 23.40, stage/render/prepare_meshes 22.76, stage/render/prepare_assets 20.46, stage/render/prepare_bind_groups 18.19 |
| warm-up | 8.48 | 77.9 | pipelines/wait_ms 5091, pipelines/compile_ms 3757, pipelines/driver_ms 3537, pipelines/front_ms 220, render/thread/ms 165, render/busy/ms 163, main/busy/ms 77.56, stage/main/post_update 57.37, stage/render/prepare_resources 47.90, stage/render/prepare_assets 27.11, bigspace/total/ms 22.84, stage/render/render 20.18 |
| warm-up | 8.54 | 60.0 | pipelines/compile_ms 1599, pipelines/driver_ms 1442, pipelines/wait_ms 687, pipelines/front_ms 157, render/thread/ms 118, render/busy/ms 116, main/busy/ms 59.95, stage/main/post_update 37.97, stage/render/prepare_resources 37.40, stage/render/prepare_assets 33.77, stage/render/render 21.18, render/graph/ms 18.46 |
| warm-up | 8.72 | 66.4 | render/thread/ms 94.34, render/busy/ms 92.48, main/busy/ms 82.24, stage/main/post_update 38.48, stage/render/prepare_assets 23.34, stage/main/update 19.72, stage/main/last 18.30, stage/render/prepare_flush 18.12, stage/render/render 16.59, encode//sum 14.23, render/graph/ms 13.88, stage/render/prepare_write_phase_buffers 11.37 |
| warm-up | 8.78 | 64.0 | render/thread/ms 124, render/busy/ms 122, main/busy/ms 104, stage/main/post_update 43.38, stage/render/prepare_assets 40.70, stage/main/update 28.33, stage/render/prepare_flush 20.89, stage/main/last 18.26, stage/render/render 15.04, encode//sum 14.55, stage/render/prepare_write_phase_buffers 14.43, stream/drive/ms 12.48 |
| warm-up | 8.85 | 73.5 | render/thread/ms 131, render/busy/ms 115, main/busy/ms 92.47, stage/main/update 42.87, stage/main/post_update 34.97, stage/render/prepare_assets 32.03, stage/render/render 28.86, stage/render/prepare_resources 17.80, render/present/ms 15.49, encode//sum 15.43, stream/drive/ms 12.50, render/graph/ms 12.24 |
| warm-up | 8.97 | 63.4 | render/thread/ms 111, render/busy/ms 110, main/busy/ms 62.75, stage/main/post_update 37.43, stage/render/prepare_assets 28.14, stage/render/prepare_resources 23.73, stage/render/render 23.05, render/graph/ms 20.75, encode//sum 17.79, stage/render/graph_submit 17.43, stage/main/update 16.89, bigspace/total/ms 11.75 |
| warm-up | 9.04 | 64.7 | render/thread/ms 119, render/busy/ms 118, main/busy/ms 74.60, stage/main/post_update 48.01, stage/render/prepare_assets 30.34, stage/render/prepare_bind_groups 25.79, stage/render/render 24.95, render/graph/ms 22.66, encode//sum 19.96, stage/render/graph_submit 19.30, stage/main/update 17.39, encode/finish_wall 13.97 |
| warm-up | 9.09 | 57.5 | render/thread/ms 109, render/busy/ms 107, main/busy/ms 63.01, stage/main/post_update 38.85, stage/render/prepare_assets 23.89, encode//sum 19.86, stage/render/render 17.73, stage/render/prepare_resources 17.49, bigspace/total/ms 17.21, stage/render/prepare_bind_groups 17.12, stage/main/update 16.15, render/graph/ms 15.42 |
| warm-up | 9.15 | 59.3 | render/thread/ms 104, render/busy/ms 102, main/busy/ms 57.09, stage/main/post_update 31.99, stage/render/prepare_assets 24.94, stage/render/prepare_write_phase_buffers 19.51, bigspace/total/ms 18.42, stage/main/update 18.33, stage/render/prepare_resources 17.51, stage/render/render 17.13, encode//sum 16.69, render/graph/ms 14.75 |
| warm-up | 9.28 | 84.6 | pipelines/compile_ms 1996, pipelines/driver_ms 1849, pipelines/wait_ms 279, pipelines/front_ms 147, render/thread/ms 120, render/busy/ms 118, main/busy/ms 99.46, stage/main/update 49.56, stage/render/prepare_write_phase_buffers 49.45, stage/main/post_update 42.30, stream/drive/ms 32.57, stage/render/prepare_assets 17.96 |
| warm-up | 9.36 | 77.7 | pipelines/compile_ms 1996, pipelines/driver_ms 1849, pipelines/wait_ms 279, render/thread/ms 154, render/busy/ms 152, pipelines/front_ms 147, main/busy/ms 98.16, stage/render/prepare_write_phase_buffers 47.00, stage/main/update 46.80, stage/main/post_update 43.32, stream/drive/ms 32.34, stage/render/prepare_assets 29.81 |
| warm-up | 9.54 | 136.4 | render/thread/ms 169, render/busy/ms 168, main/busy/ms 58.88, stage/render/prepare_resources 43.94, stage/render/prepare_batch_phases 30.73, stage/render/render 30.26, stage/main/update 29.75, render/graph/ms 27.97, stage/render/graph_submit 24.51, encode//sum 24.44, stage/render/prepare_flush 23.09, stage/main/post_update 22.04 |
| warm-up | 9.62 | 78.0 | render/thread/ms 207, render/busy/ms 205, main/busy/ms 87.08, stage/main/post_update 50.62, stage/render/prepare_resources 35.64, stage/render/render 30.98, stage/render/prepare_batch_phases 30.49, render/graph/ms 28.54, stage/render/prepare_flush 25.70, stage/render/prepare_assets 25.62, stage/main/update 25.53, encode//sum 25.37 |
| warm-up | 9.73 | 73.1 | pipelines/wait_ms 3406, pipelines/compile_ms 2451, pipelines/driver_ms 2260, pipelines/front_ms 191, render/thread/ms 101, render/busy/ms 98.92, main/busy/ms 70.86, stage/render/prepare_assets 32.02, stage/main/pre_update 25.54, stage/main/post_update 24.54, stage/render/prepare_resources 18.40, encode//sum 15.38 |
| warm-up | 9.88 | 57.9 | pipelines/wait_ms 5858, pipelines/compile_ms 108, render/thread/ms 96.18, render/busy/ms 94.55, pipelines/driver_ms 85.00, main/busy/ms 80.33, stage/main/post_update 41.52, stage/main/update 32.58, stage/render/prepare_resources 24.40, pipelines/front_ms 23.13, stage/render/prepare_assets 18.37, stage/render/prepare_write_phase_buffers 16.34 |
| warm-up | 10.02 | 84.3 | pipelines/wait_ms 2001, render/thread/ms 124, render/busy/ms 123, pipelines/compile_ms 98.91, pipelines/driver_ms 57.71, main/busy/ms 56.17, pipelines/front_ms 41.20, stage/render/prepare_assets 34.27, stage/render/prepare_resources 26.43, stage/render/prepare_bind_groups 25.44, stage/main/post_update 24.72, stage/main/update 21.58 |
| warm-up | 10.10 | 80.3 | pipelines/wait_ms 2276, pipelines/compile_ms 764, pipelines/driver_ms 649, render/thread/ms 152, render/busy/ms 151, pipelines/front_ms 114, main/busy/ms 86.13, stage/render/prepare_write_phase_buffers 34.42, stage/render/prepare_assets 29.23, stage/render/prepare_bind_groups 25.33, stage/main/update 24.97, stage/render/prepare_resources 24.37 |
| warm-up | 10.40 | 86.3 | pipelines/wait_ms 15.4 k, pipelines/compile_ms 6783, pipelines/driver_ms 6292, pipelines/front_ms 492, render/thread/ms 87.13, render/busy/ms 85.31, main/busy/ms 72.98, stage/main/post_update 51.12, bigspace/total/ms 23.53, bigspace/low_precision/ms 16.16, encode//sum 15.92, stage/render/prepare_bind_groups 15.89 |
| warm-up | 10.55 | 84.9 | pipelines/wait_ms 28.2 k, pipelines/compile_ms 359, pipelines/driver_ms 234, render/thread/ms 138, render/busy/ms 137, pipelines/front_ms 125, main/busy/ms 63.68, stage/render/prepare_flush 49.67, stage/render/prepare_resources 25.11, stage/main/post_update 24.97, stage/render/render 23.26, render/graph/ms 20.94 |
| warm-up | 10.67 | 122.8 | pipelines/wait_ms 21.7 k, pipelines/compile_ms 4431, pipelines/driver_ms 3917, pipelines/front_ms 513, render/thread/ms 200, render/busy/ms 198, main/busy/ms 63.82, stage/render/prepare_flush 42.80, stage/main/post_update 32.75, stage/render/prepare_assets 32.10, stage/render/prepare_resources 26.64, encode//sum 21.30 |
| warm-up | 10.74 | 74.4 | pipelines/wait_ms 10.3 k, pipelines/compile_ms 5184, pipelines/driver_ms 4581, pipelines/front_ms 603, render/thread/ms 188, render/busy/ms 187, main/busy/ms 83.14, stage/main/update 43.82, stage/render/prepare_assets 34.71, stage/main/post_update 31.72, stage/render/prepare_flush 31.70, encode//sum 22.79 |
| warm-up | 10.80 | 59.3 | pipelines/wait_ms 18.2 k, pipelines/compile_ms 1146, pipelines/driver_ms 780, pipelines/front_ms 365, render/thread/ms 126, render/busy/ms 124, main/busy/ms 101, stage/render/prepare_assets 46.80, stage/main/update 44.37, stage/main/post_update 44.18, stage/render/prepare_batch_phases 21.90, encode//sum 18.86 |
| warm-up | 10.99 | 141.2 | pipelines/wait_ms 22.0 k, pipelines/compile_ms 1094, pipelines/driver_ms 892, pipelines/front_ms 202, render/thread/ms 179, render/busy/ms 178, main/busy/ms 53.15, stage/render/prepare_resources 49.57, stage/render/prepare_flush 47.04, stage/render/prepare_bind_groups 32.94, stage/main/post_update 27.09, encode//sum 20.01 |
| warm-up | 11.06 | 68.4 | pipelines/wait_ms 12.7 k, render/thread/ms 206, render/busy/ms 205, pipelines/compile_ms 129, main/busy/ms 92.29, pipelines/driver_ms 82.77, stage/render/prepare_flush 51.86, stage/render/prepare_resources 49.59, pipelines/front_ms 46.13, stage/main/update 34.63, stage/main/post_update 28.62, stage/render/prepare_bind_groups 28.29 |
| warm-up | 11.16 | 100.2 | pipelines/wait_ms 17.8 k, pipelines/compile_ms 2958, pipelines/driver_ms 2627, pipelines/front_ms 330, render/thread/ms 159, render/busy/ms 157, main/busy/ms 84.95, stage/main/update 35.97, stage/render/prepare_assets 34.70, stage/render/prepare_flush 31.85, encode//sum 29.40, stage/render/render 27.58 |
| warm-up | 11.41 | 67.6 | pipelines/wait_ms 26.9 k, pipelines/compile_ms 431, pipelines/front_ms 303, pipelines/driver_ms 128, render/thread/ms 93.93, render/busy/ms 92.43, main/busy/ms 62.19, stage/main/update 35.11, stage/render/prepare_assets 29.10, encode//sum 20.61, stage/render/prepare_batch_phases 18.24, stage/main/post_update 17.75 |
| warm-up | 13.72 | 60.3 | render/thread/ms 82.97, render/busy/ms 76.06, main/busy/ms 38.63, stage/render/prepare_write_phase_buffers 30.85, encode//sum 24.82, stage/main/post_update 22.17, stage/render/render 20.51, render/graph/ms 18.29, stage/render/graph_submit 14.80, stage/render/prepare_resources 11.35, stage/main/update 8.04, encode/submit 7.52 |

## Run

- `../target/profiling/walkonce.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\wgpu_walk_once\balmora_walk@2560x1440_arm=base-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 19%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1386 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **69%** (min 47%) · 2625 MHz (min 2610) · memory 8101 MHz · 73.1 W · 75 °C (max 81) · VRAM 6593 MiB · throttled 100% of samples

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
