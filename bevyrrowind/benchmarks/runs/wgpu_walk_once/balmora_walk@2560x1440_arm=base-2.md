# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5301 frames over 65.7 s · **median 12.05 ms (83 fps)** · mean 12.38 · p95 15.55 · p99 19.50 (1% low 51 fps) · worst 44.24 ms · 62 spikes over 1.5× median · 49.4 stutters/min (work 48.4/min)
- warm-up: 322 frames over 16.1 s, worst 2471.0 ms
- hitches (> 30.1 ms): **30** measured, 132 in warm-up
- timed GPU passes: 6.96 ms of a 12.38 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.05 ms, limit 11 ms
- **FAIL** frame_ms: 54 stutters (> 1.5x moving median of 60) in 1.09 min = 49.35/min, limit 1/min
- **FAIL** worst frame 44.24 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.970 | 2.037 | 2.070 | 0.97 | 0.180 |
| main_opaque_pass_3d | 1.596 | 1.650 | 2.052 | 0.97 | 0.059 |
| early prepass | 0.481 | 0.498 | 0.716 | 0.97 | 0.090 |
| bloom | 0.304 | 0.315 | 0.321 | 0.97 | 0.067 |
| atmosphere_luts | 0.295 | 0.305 | 0.366 | 0.97 | 0.003 |
| water_velocity | 0.255 | 0.264 | 0.347 | 0.97 | 0.002 |
| water_sspr | 0.195 | 0.202 | 0.203 | 0.97 | 0.002 |
| ssao | 0.182 | 0.188 | 0.193 | 0.97 | 0.007 |
| atmos_froxel_inject | 0.141 | 0.146 | 0.147 | 0.97 | 0.001 |
| clouds_trace | 0.136 | 0.141 | 0.230 | 0.97 | 0.001 |
| auxiliary_views | 0.133 | 0.137 | 0.501 | 0.97 | 0.141 |
| water_fft | 0.127 | 0.131 | 0.139 | 0.97 | 0.003 |
| uniform_allocation | 0.126 | 0.131 | 0.184 | 0.97 | 0.025 |
| upscaling | 0.114 | 0.118 | 0.127 | 0.97 | 0.004 |
| bin_unpacking | 0.102 | 0.106 | 0.143 | 0.97 | 0.018 |
| clouds_composite | 0.102 | 0.106 | 0.109 | 0.97 | 0.002 |
| tonemapping | 0.082 | 0.085 | 0.089 | 0.97 | 0.004 |
| main_transmissive_pass_3d | 0.077 | 0.080 | 0.160 | 0.97 | 0.009 |
| clustering | 0.062 | 0.064 | 0.068 | 0.97 | 0.019 |
| render_sky | 0.050 | 0.052 | 0.052 | 0.97 | 0.002 |
| clouds_resolve | 0.047 | 0.048 | 0.049 | 0.97 | 0.001 |
| surface_exposure | 0.043 | 0.045 | 0.045 | 0.97 | 0.001 |
| surface_trail | 0.042 | 0.056 | 0.057 | 0.75 | 0.002 |
| lightprobe_irradiance_map | 0.041 | 0.413 | 0.416 | 0.10 | 0.001 |
| surface_exposure_far | 0.037 | 0.044 | 0.046 | 0.85 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.033 | 0.97 | 0.003 |
| early_mesh_preprocessing | 0.029 | 0.030 | 0.046 | 0.97 | 0.004 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.97 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.028 | 0.97 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.219 | 0.230 | 0.10 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.97 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.97 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.013 | 0.026 | 0.97 | 0.124 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.97 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.96 | 0.003 |
| lightprobe_copy | 0.001 | 0.012 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/compile_ms | 13.394 | 0.000 | 7375.38 | 71003.6 |
| pipelines/driver_ms | 11.529 | 0.000 | 6938.78 | 61115.4 |
| render/thread/ms | 11.032 | 14.012 | 37.56 | 58482.8 |
| encode//sum | 10.636 | 15.159 | 56.64 | 56378.9 |
| render/busy/ms | 10.554 | 13.520 | 37.01 | 55948.5 |
| main/busy/ms | 9.577 | 12.244 | 28.98 | 50765.3 |
| pipelines/wait_ms | 7.981 | 0.000 | 16967.83 | 42308.8 |
| stage/render/render | 6.924 | 8.937 | 17.65 | 36702.1 |
| render/graph/ms | 6.249 | 8.207 | 16.70 | 33128.5 |
| stage/main/post_update | 6.018 | 8.312 | 11.59 | 31899.3 |
| stage/render/graph_submit | 5.045 | 6.719 | 12.22 | 26742.2 |
| encode/finish_wall | 2.803 | 4.117 | 6.51 | 14859.1 |
| encode/submit | 2.276 | 3.163 | 6.30 | 12062.5 |
| stage/main/update | 2.231 | 2.695 | 22.17 | 11825.6 |
| pipelines/front_ms | 1.865 | 0.000 | 436.60 | 9888.2 |
| stage/render/camera_driver | 1.147 | 1.624 | 4.41 | 6078.0 |
| stage/render/prepare_write_phase_buffers | 0.753 | 1.120 | 3.04 | 3991.7 |
| stage/render/prepare_assets | 0.583 | 0.752 | 10.24 | 3090.1 |
| bigspace/total/ms | 0.580 | 1.686 | 4.78 | 3076.0 |
| stage/render/prepare_meshes | 0.451 | 0.580 | 7.43 | 2391.3 |
| stage/render/prepare_resources | 0.448 | 0.552 | 1.26 | 2373.7 |
| stage/render/specialize | 0.445 | 0.630 | 15.66 | 2358.9 |
| stage/main/first | 0.420 | 0.555 | 5.79 | 2228.7 |
| render/present/ms | 0.379 | 0.466 | 1.87 | 2010.9 |
| stage/main/pre_update | 0.364 | 0.538 | 3.57 | 1932.0 |
| stage/render/queue | 0.319 | 0.465 | 8.33 | 1688.9 |
| terrain/update/ms | 0.305 | 0.386 | 2.64 | 1617.1 |
| stage/render/prepare_bind_groups | 0.293 | 0.529 | 2.93 | 1553.1 |
| terrain/select/ms | 0.270 | 0.342 | 0.94 | 1431.0 |
| controller/feed/ms | 0.269 | 0.351 | 0.71 | 1424.3 |
| stream/drive/ms | 0.253 | 0.363 | 17.77 | 1340.4 |
| stage/main/fixed_main_loop | 0.240 | 0.376 | 1.39 | 1271.2 |
| bigspace/low_precision/ms | 0.239 | 0.683 | 3.95 | 1266.9 |
| far/select/ms | 0.206 | 0.287 | 0.64 | 1092.0 |
| stage/render/create_views | 0.203 | 0.270 | 1.24 | 1076.7 |
| stage/render/prepare_batch_phases | 0.198 | 0.281 | 0.59 | 1047.2 |
| stream/tick/ms | 0.193 | 0.276 | 0.66 | 1023.3 |
| bigspace/high_precision/ms | 0.181 | 0.606 | 4.45 | 961.2 |
| stage/render/extract_commands | 0.175 | 0.210 | 0.50 | 929.1 |
| anim/evaluate/ms | 0.167 | 0.282 | 3.30 | 887.2 |
| stage/render/prepare_flush | 0.166 | 0.342 | 1.12 | 881.8 |
| anim/evaluate_cpu/ms | 0.151 | 0.193 | 0.77 | 800.3 |
| stage/render/prepare_views | 0.143 | 0.179 | 0.54 | 756.8 |
| controller/move/ms | 0.136 | 0.283 | 0.58 | 721.1 |
| stage/main/last | 0.123 | 0.192 | 1.63 | 654.5 |
| render/acquire/ms | 0.099 | 0.126 | 0.31 | 523.4 |
| stage/render/prepare_collect_phase_buffers | 0.067 | 0.140 | 0.37 | 352.5 |
| physics/step/ms | 0.053 | 0.087 | 0.54 | 278.4 |
| stage/render/cleanup | 0.045 | 0.108 | 23.88 | 235.9 |
| physics/insert/ms | 0.037 | 0.393 | 0.96 | 193.6 |
| encode/resolve_diagnostics/sum | 0.033 | 0.046 | 0.67 | 176.2 |
| stage/render/phase_sort | 0.028 | 0.048 | 0.19 | 148.5 |
| weather/map/ms | 0.025 | 0.033 | 0.21 | 133.9 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.023 | 0.18 | 83.0 |
| anim/apply/ms | 0.013 | 0.023 | 0.43 | 66.4 |
| encode/camera_driver_span_end/sum | 0.012 | 0.021 | 0.41 | 62.0 |
| weather/eval/ms | 0.011 | 0.015 | 0.04 | 58.8 |
| controller/avoid/ms | 0.011 | 0.014 | 0.06 | 57.3 |
| script/update/ms | 0.007 | 0.010 | 0.15 | 38.4 |
| stream/despawn/ms | 0.006 | 0.000 | 0.98 | 31.2 |
| script/ui/ms | 0.005 | 0.007 | 0.18 | 28.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 28.5 |
| script/input/ms | 0.004 | 0.006 | 0.03 | 23.3 |
| controller/write/ms | 0.004 | 0.005 | 0.04 | 18.7 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.72 | 14.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.06 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.11 | 3.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.10 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1277 | 2262 | 535 |
| crowd/c0/meshes/count | count | 352 | 646 | 144 |
| crowd/c1/count | count | 146 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5940 | 5629 |
| crowd/c1/meshes/count | count | 1165 | 1270 | 1189 |
| crowd/c2/count | count | 396 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1822 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4625 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4603 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.27 | 5.82 | 2.16 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.18 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.41 | 0.01 |
| encode/finish_critical | ms | 2.27 | 5.82 | 2.16 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.67 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 10.46 | 37.01 | 8.93 |
| gpu/timed/ms | ms | 7.92 | 11.16 | 8.24 |
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
| phases/alpha_mask3d/items | count | 799 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 799 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1296 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3361 | 5129 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1296 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3361 | 5129 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 18 | 227 | 4 |
| phases/shadow/bins | count | 5081 | 8788 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.6 k | 6380 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 206 | 256 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 279 | 329 | 329 |
| pipelines/waiting | count | 1 | 6 | 0 |
| precache/draw_waiting | count | 0 | 5 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 1 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 29 | 75 | 0 |
| preprocess/bind_groups | count | 15 | 613 | 9 |
| preprocess/bind_groups_created | count | 15 | 613 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 128 | 211 | 120 |
| process/cpu_usage | % | 31.25 | 54.33 | 29.15 |
| process/mem_usage | GiB | 5.65 | 5.94 | 5.94 |
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
| skin/blocks_changed/count | count | 13 | 127 | 9 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 201 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 370.7 k | 807.2 k | 493.3 k |
| skin/joints_written/count | count | 5792 | 12.6 k | 7708 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4922 | 5013 | 5013 |
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
| stream/latency/s | s | 0.04 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.20 | 64.57 | 29.62 |
| system/mem_usage | % | 66.59 | 67.58 | 67.54 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2900 | 3016 | 3016 |
| textures/images | count | 2680 | 2740 | 2740 |
| views/all/batch_sets | count | 67 | 274 | 54 |
| views/all/bins | count | 7646 | 11.5 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.5 k | 14.0 k |
| views/all/pipelines | count | 251 | 303 | 75 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 36.0 k | 8127 |
| views/cascade/batch_sets | count | 9 | 47 | 4 |
| views/cascade/bins | count | 2801 | 6106 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8909 | 23.5 k | 6380 |
| views/cascade/pipelines | count | 36 | 53 | 29 |
| views/cascade/work_items | count | 422 | 22.7 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 190 | 0 |
| views/lamp_face/bins | count | 2226 | 2867 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2472 | 3161 | 0 |
| views/lamp_face/pipelines | count | 170 | 209 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2873 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2888 | 3560 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8320 | 13.0 k | 7618 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8488 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 62.08 | 247 | 329.1 k |
| anim/events/count | count | 0.10 | 3 | 547 |
| assets/Image/added | count | 0.11 | 38 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 13 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.77 | 168 | 4061 |
| assets/Mesh/removed | count | 0.91 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 464 |
| assets/StandardMaterial/modified | count | 4.09 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 17.79 | 3173 | 94.3 k |
| ecs/entities_delta | count | 0.88 | 1272 | 4674 |
| encode//count | count | 39.76 | 272 | 210.7 k |
| encode/bind_walk/init_textures | count | 80.6 k | 228.8 k | 427.29 M |
| encode/bind_walk/init_textures_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/inits | count | 569 | 1333 | 3.02 M |
| encode/bind_walk/inits_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/merge_textures | count | 17.8 k | 123.9 k | 94.23 M |
| encode/bind_walk/merge_textures_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/merges | count | 184 | 438 | 975.0 k |
| encode/bind_walk/merges_skipped | count | 0 | 0 | 0 |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5301 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5301 |
| encode/queue/command_buffers | count | 44.77 | 277 | 237.4 k |
| encode/queue/submits | count | 2.02 | 4 | 10.7 k |
| encode/queue/write_buffer/bytes | bytes | 1.90 M | 4.88 M | 10.06 G |
| encode/queue/write_buffer/calls | count | 257 | 914 | 1.36 M |
| encode/queue/write_buffer_with/bytes | bytes | 180.4 k | 3.52 M | 956.05 M |
| encode/queue/write_buffer_with/calls | count | 11.28 | 513 | 59.8 k |
| encode/queue/write_texture/bytes | bytes | 10.2 k | 8.39 M | 54.09 M |
| encode/queue/write_texture/calls | count | 0.11 | 42 | 561 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5301 |
| physics/inserts | count | 0.43 | 109 | 2283 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.44 | 145 | 2335 |
| physics/steps | count | 0.74 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 12 |
| pipelines/created | count | 0.02 | 2 | 120 |
| precache/hit | count | 0.00 | 2 | 3 |
| precache/missed | count | 0.00 | 2 | 7 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6386 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.01 | 1 | 28 |
| surface/shelter_redraws | count | 0.01 | 1 | 55 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1400 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.1 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2471.0 | main/busy/ms 2434, stage/main/post_update 2323, stage/main/update 100, surface/advance_cpu_ms 68.05, stage/main/pre_update 5.81, stage/main/last 1.60, surface/cell_upload_ms 0.74, stage/main/first 0.57, stage/main/fixed_main_loop 0.35, stream/drive/ms 0.34, weather/map/ms 0.23, bigspace/total/ms 0.11 |
| warm-up | 3.20 | 725.9 | main/busy/ms 2568, stage/main/post_update 2345, render/thread/ms 690, render/busy/ms 528, stage/render/prepare_assets 478, stage/main/update 207, stage/render/prepare_views 159, render/acquire/ms 159, surface/advance_cpu_ms 68.05, stage/render/create_views 28.35, stage/render/render 14.58, render/graph/ms 9.45 |
| warm-up | 3.37 | 168.6 | render/thread/ms 837, render/busy/ms 602, pipelines/compile_ms 589, stage/render/prepare_assets 537, pipelines/driver_ms 495, main/busy/ms 169, stage/render/prepare_views 159, render/acquire/ms 159, stage/main/update 116, pipelines/front_ms 93.55, stage/render/render 90.92, render/present/ms 75.60 |
| warm-up | 3.50 | 96.9 | pipelines/compile_ms 1047, pipelines/driver_ms 881, main/busy/ms 174, pipelines/front_ms 166, pipelines/wait_ms 113, render/thread/ms 106, render/busy/ms 101, stage/main/update 97.92, stage/main/post_update 51.83, stage/render/render 28.68, stage/render/prepare_flush 25.15, stage/render/prepare_assets 24.54 |
| warm-up | 3.64 | 144.2 | pipelines/compile_ms 1853, pipelines/driver_ms 1559, pipelines/wait_ms 1189, pipelines/front_ms 294, main/busy/ms 166, render/thread/ms 158, render/busy/ms 156, stage/render/prepare_assets 91.36, stage/main/update 74.00, stage/main/post_update 60.73, stage/render/prepare_flush 24.77, stage/render/prepare_bind_groups 16.49 |
| warm-up | 3.98 | 84.7 | pipelines/wait_ms 140, render/thread/ms 130, render/busy/ms 127, pipelines/compile_ms 115, main/busy/ms 102, pipelines/driver_ms 91.17, stage/render/prepare_assets 57.44, stage/main/update 56.68, stream/drive/ms 45.71, stage/main/post_update 27.78, stage/render/prepare_flush 25.31, pipelines/front_ms 23.60 |
| warm-up | 4.11 | 100.8 | pipelines/compile_ms 480, pipelines/driver_ms 408, pipelines/wait_ms 154, render/thread/ms 103, render/busy/ms 102, pipelines/front_ms 72.04, main/busy/ms 62.66, stage/render/prepare_assets 41.08, stage/main/update 32.24, stage/main/post_update 23.48, stage/render/prepare_write_phase_buffers 19.64, stage/render/prepare_resources 15.80 |
| warm-up | 4.22 | 107.4 | render/thread/ms 173, render/busy/ms 171, stage/render/prepare_assets 70.29, main/busy/ms 54.90, stage/render/prepare_resources 51.77, stage/main/update 36.66, stream/drive/ms 28.34, stage/render/prepare_write_phase_buffers 19.35, stage/render/prepare_bind_groups 19.27, stage/main/post_update 11.04, stage/render/render 7.69, render/graph/ms 5.45 |
| warm-up | 4.38 | 101.1 | render/thread/ms 129, render/busy/ms 128, main/busy/ms 59.59, stage/render/prepare_assets 58.68, stage/render/prepare_resources 30.48, stage/main/post_update 20.33, stage/main/update 18.45, stage/render/prepare_bind_groups 14.49, stage/main/pre_update 14.23, stage/render/prepare_flush 9.45, stage/render/render 7.94, render/graph/ms 5.63 |
| warm-up | 4.46 | 79.1 | render/thread/ms 130, render/busy/ms 129, main/busy/ms 119, stage/main/update 55.15, stage/render/prepare_assets 49.28, stage/main/post_update 40.03, stage/render/prepare_bind_groups 35.22, stage/render/prepare_resources 28.64, bigspace/total/ms 19.26, bigspace/low_precision/ms 19.00, stage/main/pre_update 10.75, stage/main/fixed_main_loop 9.58 |
| warm-up | 4.61 | 144.2 | render/thread/ms 146, render/busy/ms 144, main/busy/ms 131, stage/render/prepare_assets 69.76, stage/main/post_update 54.79, stage/main/update 54.02, stage/render/prepare_bind_groups 22.13, stage/render/prepare_write_phase_buffers 20.69, bigspace/total/ms 19.44, bigspace/low_precision/ms 19.10, stage/main/fixed_main_loop 16.87, stage/render/render 7.75 |
| warm-up | 4.71 | 97.9 | pipelines/compile_ms 1372, pipelines/driver_ms 1311, pipelines/wait_ms 943, render/thread/ms 180, render/busy/ms 177, stage/render/prepare_assets 74.54, main/busy/ms 68.96, pipelines/front_ms 60.62, stage/main/post_update 33.14, stage/render/prepare_write_phase_buffers 29.81, stage/main/update 18.83, stage/render/prepare_views 16.43 |
| warm-up | 4.82 | 108.9 | pipelines/wait_ms 3336, pipelines/compile_ms 2833, pipelines/driver_ms 2716, render/thread/ms 184, render/busy/ms 181, pipelines/front_ms 116, main/busy/ms 103, stage/main/post_update 75.57, stage/render/prepare_assets 68.30, stage/render/prepare_resources 27.26, stage/render/prepare_write_phase_buffers 26.36, stage/main/update 18.37 |
| warm-up | 4.92 | 108.4 | pipelines/wait_ms 3805, pipelines/compile_ms 1540, pipelines/driver_ms 1471, render/thread/ms 199, render/busy/ms 197, main/busy/ms 155, stage/main/post_update 122, pipelines/front_ms 68.67, stage/render/prepare_assets 54.73, stage/render/prepare_resources 51.38, encode//sum 23.20, stage/main/update 22.47 |
| warm-up | 5.06 | 139.3 | pipelines/wait_ms 1411, render/thread/ms 221, render/busy/ms 219, main/busy/ms 149, stage/main/post_update 104, pipelines/compile_ms 79.44, pipelines/driver_ms 66.45, stage/render/prepare_resources 50.52, stage/render/render 38.60, render/graph/ms 35.67, encode//sum 32.55, stage/render/prepare_write_phase_buffers 31.14 |
| warm-up | 5.24 | 174.2 | pipelines/wait_ms 2392, pipelines/compile_ms 1174, pipelines/driver_ms 1087, render/thread/ms 259, render/busy/ms 257, main/busy/ms 174, pipelines/front_ms 87.39, stage/main/update 80.05, stage/render/prepare_assets 68.94, stream/drive/ms 64.64, stage/main/post_update 62.60, stage/render/prepare_resources 54.37 |
| warm-up | 5.36 | 120.1 | pipelines/wait_ms 2392, pipelines/compile_ms 1174, pipelines/driver_ms 1087, render/thread/ms 257, render/busy/ms 255, main/busy/ms 168, stage/render/prepare_resources 112, stage/main/update 106, pipelines/front_ms 87.39, stage/render/prepare_assets 67.84, stream/drive/ms 64.29, stage/main/post_update 30.98 |
| warm-up | 5.53 | 176.6 | render/thread/ms 272, render/busy/ms 270, main/busy/ms 176, stage/main/update 117, stage/render/prepare_assets 100, stage/render/prepare_resources 98.26, stream/drive/ms 56.25, stage/render/prepare_write_phase_buffers 28.89, stage/main/pre_update 25.45, stage/main/post_update 19.92, stage/render/render 15.53, render/graph/ms 12.17 |
| warm-up | 5.64 | 104.1 | render/thread/ms 245, render/busy/ms 242, main/busy/ms 138, stage/render/prepare_assets 129, stage/main/update 87.76, stream/drive/ms 56.87, stage/render/prepare_resources 50.62, stage/main/pre_update 20.85, stage/main/post_update 20.79, stage/render/render 13.72, stage/render/prepare_write_phase_buffers 13.33, stage/render/specialize 10.98 |
| warm-up | 5.73 | 87.5 | render/thread/ms 167, render/busy/ms 165, main/busy/ms 112, stage/main/update 65.82, stage/render/prepare_assets 62.86, stream/drive/ms 48.84, stage/main/post_update 29.85, stage/render/prepare_resources 29.32, stage/render/prepare_write_phase_buffers 17.64, stage/render/prepare_flush 17.20, stage/main/fixed_main_loop 11.20, stage/render/prepare_bind_groups 9.79 |
| warm-up | 5.81 | 82.8 | render/thread/ms 141, render/busy/ms 138, main/busy/ms 126, stage/main/update 68.36, stage/render/prepare_assets 61.09, stream/drive/ms 49.49, stage/main/post_update 41.81, stage/render/prepare_write_phase_buffers 19.19, stage/render/prepare_flush 14.51, stage/render/prepare_bind_groups 14.36, stage/main/fixed_main_loop 11.54, stage/render/render 8.92 |
| warm-up | 5.91 | 100.4 | render/thread/ms 153, render/busy/ms 150, main/busy/ms 118, stage/render/prepare_assets 101, stage/main/update 61.94, stream/drive/ms 42.03, stage/main/post_update 34.92, stage/main/fixed_main_loop 15.22, stage/render/prepare_resources 10.47, stage/render/render 10.36, bigspace/total/ms 9.05, stage/render/prepare_bind_groups 7.41 |
| warm-up | 6.19 | 72.1 | pipelines/compile_ms 2759, pipelines/driver_ms 2609, pipelines/front_ms 150, render/thread/ms 118, render/busy/ms 116, main/busy/ms 75.45, stage/render/prepare_assets 58.36, stage/main/update 43.69, pipelines/wait_ms 35.41, stage/main/post_update 24.64, stage/render/prepare_write_phase_buffers 22.37, stage/render/prepare_resources 12.16 |
| warm-up | 6.27 | 84.4 | pipelines/compile_ms 2837, pipelines/wait_ms 2830, pipelines/driver_ms 2656, pipelines/front_ms 181, render/thread/ms 138, render/busy/ms 136, main/busy/ms 109, stage/render/prepare_assets 75.54, stage/main/post_update 57.21, stage/main/update 43.72, stage/render/prepare_resources 20.65, stage/render/render 13.82 |
| warm-up | 6.37 | 68.2 | render/thread/ms 91.48, render/busy/ms 89.63, main/busy/ms 51.44, stage/render/prepare_assets 45.69, stage/main/post_update 28.34, stage/main/update 14.83, stage/render/prepare_resources 13.93, bigspace/total/ms 12.37, bigspace/high_precision/ms 11.96, stage/render/render 9.65, render/graph/ms 6.97, stage/main/fixed_main_loop 4.78 |
| warm-up | 6.72 | 109.7 | pipelines/compile_ms 6282, pipelines/driver_ms 5917, pipelines/front_ms 364, pipelines/wait_ms 255, render/thread/ms 139, render/busy/ms 138, main/busy/ms 54.30, stage/render/prepare_resources 41.26, stage/render/prepare_assets 36.07, stage/main/post_update 32.12, stage/render/prepare_flush 26.97, stage/main/update 14.99 |
| warm-up | 6.84 | 118.7 | pipelines/compile_ms 6282, pipelines/driver_ms 5917, pipelines/front_ms 364, pipelines/wait_ms 255, render/thread/ms 205, render/busy/ms 203, main/busy/ms 105, stage/main/post_update 55.22, stage/render/prepare_resources 43.31, stage/render/prepare_assets 41.06, stage/render/prepare_bind_groups 40.35, stage/render/render 39.39 |
| warm-up | 6.94 | 94.7 | pipelines/wait_ms 3243, pipelines/compile_ms 3080, pipelines/driver_ms 2868, pipelines/front_ms 212, render/thread/ms 186, render/busy/ms 176, main/busy/ms 168, stage/main/update 70.79, stage/main/post_update 58.78, stage/render/render 50.05, stage/render/prepare_assets 43.64, render/graph/ms 40.59 |
| warm-up | 7.01 | 77.0 | pipelines/compile_ms 7149, pipelines/driver_ms 6591, pipelines/wait_ms 5251, pipelines/front_ms 558, render/thread/ms 158, render/busy/ms 149, main/busy/ms 148, stage/main/update 63.15, stage/main/post_update 51.74, stage/render/prepare_assets 51.32, stage/render/prepare_resources 30.75, stage/main/fixed_main_loop 23.79 |
| warm-up | 7.11 | 92.7 | pipelines/compile_ms 4069, pipelines/driver_ms 3723, pipelines/wait_ms 2008, pipelines/front_ms 345, render/thread/ms 155, render/busy/ms 153, main/busy/ms 130, stage/main/post_update 89.43, stage/render/prepare_assets 44.74, stage/render/prepare_resources 41.69, stage/render/prepare_write_phase_buffers 37.24, stage/main/update 30.38 |
| warm-up | 7.18 | 74.8 | render/thread/ms 153, render/busy/ms 151, main/busy/ms 125, stage/main/post_update 86.40, stage/render/prepare_assets 58.52, stage/render/prepare_resources 39.44, stage/render/prepare_write_phase_buffers 23.91, stage/main/fixed_main_loop 18.43, stage/main/update 12.10, stage/render/render 9.99, stage/render/prepare_flush 9.26, render/graph/ms 6.90 |
| warm-up | 7.36 | 74.5 | render/thread/ms 117, render/busy/ms 115, main/busy/ms 82.60, stage/main/post_update 62.70, stage/render/prepare_assets 55.39, bigspace/total/ms 27.09, stage/render/prepare_resources 25.10, bigspace/low_precision/ms 24.81, stage/render/render 14.18, stage/main/update 12.73, render/graph/ms 11.35, encode//sum 8.95 |
| warm-up | 7.88 | 74.2 | render/thread/ms 112, render/busy/ms 110, main/busy/ms 47.33, stage/render/prepare_resources 40.32, stage/render/prepare_assets 30.12, stage/main/post_update 28.42, stage/render/prepare_write_phase_buffers 18.54, stage/main/update 10.95, stage/render/render 10.00, render/graph/ms 6.92, stage/render/graph_submit 4.68, stage/main/fixed_main_loop 4.47 |
| warm-up | 7.96 | 82.9 | render/thread/ms 142, render/busy/ms 140, main/busy/ms 71.39, stage/render/prepare_resources 49.78, stage/render/prepare_assets 44.05, stage/main/post_update 36.19, stage/render/prepare_write_phase_buffers 21.24, stage/main/update 18.15, stage/main/fixed_main_loop 13.35, stage/render/render 13.06, render/graph/ms 9.95, encode//sum 7.89 |
| warm-up | 8.14 | 97.6 | pipelines/compile_ms 7967, pipelines/driver_ms 7502, pipelines/wait_ms 6215, pipelines/front_ms 465, render/thread/ms 123, render/busy/ms 121, main/busy/ms 41.81, stage/render/prepare_resources 39.44, stage/render/prepare_assets 28.26, stage/main/post_update 22.31, stage/render/prepare_bind_groups 16.69, stage/render/render 13.03 |
| warm-up | 8.24 | 103.9 | pipelines/wait_ms 39.8 k, pipelines/compile_ms 12.4 k, pipelines/driver_ms 11.4 k, pipelines/front_ms 984, render/thread/ms 191, render/busy/ms 188, stage/render/prepare_resources 73.40, main/busy/ms 38.08, stage/render/prepare_assets 36.10, stage/render/prepare_write_phase_buffers 28.87, stage/render/prepare_flush 19.52, stage/render/prepare_bind_groups 16.42 |
| warm-up | 8.35 | 108.0 | pipelines/wait_ms 62.4 k, pipelines/compile_ms 4793, pipelines/driver_ms 4140, pipelines/front_ms 653, render/thread/ms 209, render/busy/ms 206, main/busy/ms 113, stage/render/prepare_resources 75.19, stage/main/post_update 63.09, stage/render/prepare_write_phase_buffers 44.04, stage/render/prepare_flush 38.76, stage/render/prepare_assets 32.21 |
| warm-up | 8.43 | 80.5 | pipelines/wait_ms 32.4 k, pipelines/compile_ms 456, pipelines/driver_ms 290, main/busy/ms 182, render/thread/ms 178, render/busy/ms 175, pipelines/front_ms 166, stage/main/post_update 81.52, stage/render/prepare_assets 50.81, stage/main/pre_update 42.47, stage/render/prepare_flush 40.71, stage/main/update 35.28 |
| warm-up | 8.68 | 248.7 | pipelines/wait_ms 25.9 k, pipelines/compile_ms 620, pipelines/driver_ms 481, render/thread/ms 290, render/busy/ms 287, stage/render/prepare_assets 158, pipelines/front_ms 139, main/busy/ms 138, stage/main/post_update 47.15, stage/render/prepare_flush 44.65, stage/main/pre_update 39.77, stage/render/prepare_resources 34.70 |
| warm-up | 8.76 | 83.2 | pipelines/wait_ms 23.5 k, pipelines/compile_ms 3892, pipelines/driver_ms 3519, pipelines/front_ms 373, render/thread/ms 299, render/busy/ms 297, stage/render/prepare_assets 151, main/busy/ms 96.79, stage/render/prepare_resources 61.50, stage/main/post_update 42.67, stage/render/prepare_flush 30.22, stage/render/prepare_bind_groups 23.39 |
| warm-up | 8.83 | 71.6 | pipelines/compile_ms 3340, pipelines/driver_ms 3076, pipelines/wait_ms 1089, pipelines/front_ms 265, render/thread/ms 147, render/busy/ms 145, main/busy/ms 113, stage/main/post_update 68.65, stage/render/prepare_resources 51.50, bigspace/total/ms 25.59, stage/render/prepare_write_phase_buffers 25.38, stage/render/prepare_assets 22.17 |
| warm-up | 8.91 | 80.6 | render/thread/ms 143, render/busy/ms 141, main/busy/ms 103, stage/main/post_update 56.87, stage/render/prepare_resources 51.26, stage/render/prepare_write_phase_buffers 34.51, stage/main/update 21.27, stage/main/pre_update 15.46, stage/render/create_views 13.62, stage/render/render 12.33, bigspace/total/ms 9.68, render/graph/ms 9.65 |
| warm-up | 9.11 | 94.8 | render/thread/ms 137, render/busy/ms 135, main/busy/ms 85.09, stage/main/post_update 41.13, stage/render/prepare_assets 32.92, stage/main/update 30.62, stage/render/prepare_write_phase_buffers 26.05, stage/render/prepare_resources 18.45, stage/render/prepare_flush 14.42, stage/render/prepare_batch_phases 12.88, bigspace/total/ms 11.71, stage/render/render 11.42 |
| warm-up | 9.18 | 70.3 | pipelines/wait_ms 16.8 k, pipelines/compile_ms 5248, pipelines/driver_ms 4920, pipelines/front_ms 329, render/thread/ms 153, render/busy/ms 151, main/busy/ms 101, stage/main/post_update 65.99, stage/render/prepare_assets 44.76, stage/render/prepare_write_phase_buffers 22.72, stage/main/update 19.56, stage/render/prepare_batch_phases 18.04 |
| warm-up | 9.42 | 82.7 | pipelines/wait_ms 23.9 k, pipelines/compile_ms 3113, pipelines/driver_ms 2847, pipelines/front_ms 266, render/thread/ms 114, render/busy/ms 111, main/busy/ms 60.96, stage/render/prepare_bind_groups 30.91, stage/render/prepare_resources 30.35, stage/main/post_update 25.35, stage/main/update 18.55, stage/render/prepare_flush 12.80 |
| warm-up | 9.56 | 146.4 | pipelines/wait_ms 14.5 k, pipelines/compile_ms 2950, pipelines/driver_ms 2719, pipelines/front_ms 231, render/thread/ms 195, render/busy/ms 106, stage/render/render 98.07, render/present/ms 88.65, main/busy/ms 49.03, stage/render/prepare_resources 38.07, stage/main/update 18.85, stage/render/prepare_assets 18.07 |
| warm-up | 9.76 | 66.8 | pipelines/wait_ms 31.7 k, pipelines/compile_ms 5999, pipelines/driver_ms 5450, pipelines/front_ms 549, render/thread/ms 103, render/busy/ms 101, main/busy/ms 68.24, stage/main/post_update 38.68, stage/render/prepare_resources 23.34, stage/render/prepare_write_phase_buffers 20.86, stage/main/update 19.95, stage/render/prepare_assets 18.12 |
| warm-up | 9.89 | 85.8 | pipelines/wait_ms 34.3 k, pipelines/compile_ms 348, pipelines/driver_ms 253, render/thread/ms 117, render/busy/ms 115, main/busy/ms 97.76, pipelines/front_ms 94.81, stage/main/post_update 37.67, stage/main/update 31.59, stage/render/prepare_assets 27.86, stage/render/phase_sort 23.20, stage/main/pre_update 20.22 |
| warm-up | 10.01 | 88.5 | pipelines/wait_ms 6403, pipelines/compile_ms 193, pipelines/driver_ms 168, render/thread/ms 112, render/busy/ms 111, main/busy/ms 54.71, stage/render/prepare_resources 33.17, stage/render/prepare_assets 32.58, pipelines/front_ms 24.36, stage/main/update 19.48, stage/main/post_update 16.60, stage/render/render 11.22 |
| warm-up | 10.15 | 142.1 | pipelines/wait_ms 33.1 k, pipelines/compile_ms 355, pipelines/driver_ms 271, render/thread/ms 217, render/busy/ms 215, main/busy/ms 112, pipelines/front_ms 84.03, stage/render/prepare_write_phase_buffers 55.59, stage/render/prepare_assets 47.17, stage/main/post_update 44.12, stage/render/prepare_resources 42.04, stage/main/update 29.31 |
| warm-up | 10.29 | 84.7 | render/thread/ms 130, render/busy/ms 128, main/busy/ms 68.88, stage/main/post_update 28.82, stage/render/prepare_assets 28.54, stage/render/prepare_batch_phases 25.94, stage/render/prepare_resources 24.36, stage/main/update 23.97, stage/render/render 13.07, stage/render/prepare_bind_groups 12.57, stream/drive/ms 11.20, stage/render/prepare_write_phase_buffers 10.89 |
| warm-up | 10.37 | 76.8 | pipelines/wait_ms 384, pipelines/compile_ms 218, pipelines/front_ms 213, render/thread/ms 134, render/busy/ms 132, main/busy/ms 60.08, stage/render/prepare_resources 33.47, stage/render/prepare_assets 27.23, stage/render/prepare_batch_phases 25.98, stage/main/update 25.21, stage/main/post_update 19.87, stage/render/render 12.49 |
| warm-up | 10.44 | 67.9 | pipelines/wait_ms 384, pipelines/compile_ms 218, pipelines/front_ms 213, render/thread/ms 114, render/busy/ms 113, main/busy/ms 53.74, stage/render/prepare_assets 39.99, stage/render/prepare_resources 23.41, stage/main/post_update 23.13, stage/render/prepare_write_phase_buffers 19.08, stage/main/update 18.48, stage/render/render 10.02 |
| warm-up | 10.61 | 79.6 | pipelines/compile_ms 7231, pipelines/driver_ms 6990, pipelines/front_ms 242, render/thread/ms 110, render/busy/ms 108, main/busy/ms 94.88, stage/main/update 46.88, stage/main/post_update 38.62, stage/render/prepare_meshes 17.94, stage/render/prepare_assets 17.54, stage/render/create_views 16.34, stage/render/prepare_resources 13.24 |
| warm-up | 10.72 | 106.1 | pipelines/wait_ms 935, pipelines/compile_ms 187, render/thread/ms 158, render/busy/ms 156, pipelines/driver_ms 128, main/busy/ms 74.70, pipelines/front_ms 59.23, stage/main/post_update 32.08, stage/render/prepare_resources 30.53, stage/render/prepare_assets 30.49, stage/main/update 30.47, stage/render/prepare_flush 27.95 |
| warm-up | 10.80 | 75.3 | pipelines/wait_ms 15.6 k, pipelines/compile_ms 253, pipelines/driver_ms 190, render/thread/ms 166, render/busy/ms 162, main/busy/ms 101, pipelines/front_ms 62.94, stage/main/post_update 53.18, stage/render/prepare_resources 46.61, stage/render/prepare_assets 44.11, stage/render/prepare_flush 31.86, stage/main/update 19.12 |
| warm-up | 10.93 | 73.3 | pipelines/wait_ms 14.8 k, render/thread/ms 118, render/busy/ms 111, main/busy/ms 89.08, pipelines/compile_ms 50.07, pipelines/driver_ms 35.32, stage/main/post_update 34.48, stage/render/prepare_resources 34.41, stage/main/update 33.32, stage/render/prepare_assets 26.70, stage/render/render 19.29, pipelines/front_ms 14.76 |
| warm-up | 11.33 | 85.6 | pipelines/wait_ms 35.4 k, pipelines/compile_ms 5596, pipelines/driver_ms 4955, pipelines/front_ms 641, render/thread/ms 144, render/busy/ms 142, main/busy/ms 90.00, stage/main/post_update 53.56, stage/render/prepare_resources 53.42, stage/render/prepare_write_phase_buffers 36.68, bigspace/total/ms 22.83, bigspace/low_precision/ms 22.07 |
| warm-up | 11.40 | 71.9 | pipelines/wait_ms 11.5 k, pipelines/compile_ms 4767, pipelines/driver_ms 4194, pipelines/front_ms 573, render/thread/ms 150, render/busy/ms 149, stage/render/prepare_resources 59.03, main/busy/ms 56.61, stage/main/post_update 29.79, stage/render/prepare_bind_groups 25.96, stage/render/render 16.85, encode//sum 15.24 |
| warm-up | 11.50 | 99.1 | pipelines/wait_ms 13.2 k, pipelines/compile_ms 248, render/thread/ms 165, render/busy/ms 164, pipelines/driver_ms 155, pipelines/front_ms 92.23, main/busy/ms 57.92, stage/render/prepare_resources 52.47, stage/main/post_update 34.80, stage/render/prepare_bind_groups 31.32, stage/render/prepare_flush 19.21, stage/render/render 19.04 |
| warm-up | 11.67 | 113.9 | pipelines/wait_ms 24.3 k, pipelines/compile_ms 313, pipelines/driver_ms 280, render/thread/ms 156, render/busy/ms 154, main/busy/ms 154, stage/main/post_update 59.56, stage/render/prepare_assets 42.09, stage/main/fixed_main_loop 38.73, stage/render/prepare_write_phase_buffers 37.37, pipelines/front_ms 32.35, stage/main/update 29.35 |
| warm-up | 11.80 | 68.1 | render/thread/ms 120, render/busy/ms 118, main/busy/ms 74.95, stage/main/post_update 43.81, stage/render/prepare_assets 33.83, stage/render/prepare_resources 33.33, stage/main/update 14.64, stage/render/render 12.69, encode//sum 11.22, stage/render/prepare_flush 10.91, render/graph/ms 10.31, stage/render/prepare_batch_phases 9.64 |
| warm-up | 11.88 | 80.6 | pipelines/wait_ms 10.9 k, pipelines/compile_ms 3570, pipelines/driver_ms 3268, pipelines/front_ms 301, render/thread/ms 143, render/busy/ms 141, main/busy/ms 100, stage/main/post_update 46.63, stage/render/prepare_assets 40.35, stage/main/update 34.04, stream/drive/ms 21.40, stage/render/prepare_resources 21.00 |
| warm-up | 14.11 | 70.6 | pipelines/wait_ms 16.8 k, pipelines/compile_ms 192, pipelines/driver_ms 132, render/thread/ms 86.98, render/busy/ms 85.41, pipelines/front_ms 60.26, main/busy/ms 43.51, stage/render/prepare_resources 28.19, stage/main/post_update 28.13, encode//sum 26.03, stage/render/prepare_assets 22.26, stage/render/render 19.56 |

## Run

- `../target/profiling/walkonce.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\wgpu_walk_once\balmora_walk@2560x1440_arm=base-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1386 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **69%** (min 41%) · 2623 MHz (min 2610) · memory 8101 MHz · 73.3 W · 75 °C (max 80) · VRAM 6966 MiB · throttled 100% of samples

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
