# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); render.wgpu.bind_group_walk_once=on (command line: --set); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5963 frames over 65.6 s · **median 10.71 ms (93 fps)** · mean 11.01 · p95 13.82 · p99 17.82 (1% low 56 fps) · worst 42.05 ms · 100 spikes over 1.5× median · 57.6 stutters/min (work 55.8/min)
- warm-up: 321 frames over 14.9 s, worst 2509.6 ms
- hitches (> 26.8 ms): **36** measured, 144 in warm-up
- timed GPU passes: 6.64 ms of a 11.01 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.71 ms, limit 11 ms
- **FAIL** frame_ms: 63 stutters (> 1.5x moving median of 60) in 1.09 min = 57.58/min, limit 1/min
- **FAIL** worst frame 42.05 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.880 | 2.041 | 2.074 | 0.92 | 0.179 |
| main_opaque_pass_3d | 1.522 | 1.653 | 2.079 | 0.92 | 0.059 |
| early prepass | 0.465 | 0.505 | 0.740 | 0.92 | 0.090 |
| bloom | 0.290 | 0.315 | 0.322 | 0.92 | 0.066 |
| atmosphere_luts | 0.281 | 0.305 | 0.368 | 0.92 | 0.003 |
| water_velocity | 0.246 | 0.268 | 0.346 | 0.92 | 0.002 |
| water_sspr | 0.186 | 0.202 | 0.203 | 0.92 | 0.002 |
| ssao | 0.174 | 0.189 | 0.193 | 0.92 | 0.007 |
| atmos_froxel_inject | 0.135 | 0.146 | 0.148 | 0.92 | 0.001 |
| clouds_trace | 0.130 | 0.141 | 0.233 | 0.92 | 0.001 |
| uniform_allocation | 0.121 | 0.132 | 0.180 | 0.92 | 0.025 |
| water_fft | 0.119 | 0.129 | 0.137 | 0.92 | 0.003 |
| auxiliary_views | 0.113 | 0.123 | 0.490 | 0.92 | 0.129 |
| upscaling | 0.108 | 0.117 | 0.125 | 0.92 | 0.004 |
| clouds_composite | 0.097 | 0.106 | 0.109 | 0.92 | 0.002 |
| bin_unpacking | 0.096 | 0.105 | 0.142 | 0.92 | 0.018 |
| tonemapping | 0.078 | 0.085 | 0.088 | 0.92 | 0.004 |
| main_transmissive_pass_3d | 0.075 | 0.081 | 0.162 | 0.92 | 0.009 |
| clustering | 0.059 | 0.064 | 0.067 | 0.92 | 0.019 |
| render_sky | 0.047 | 0.051 | 0.052 | 0.92 | 0.002 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.92 | 0.001 |
| surface_exposure | 0.041 | 0.045 | 0.045 | 0.92 | 0.001 |
| lightprobe_irradiance_map | 0.041 | 0.413 | 0.417 | 0.10 | 0.001 |
| surface_trail | 0.037 | 0.056 | 0.057 | 0.66 | 0.002 |
| surface_exposure_far | 0.036 | 0.044 | 0.046 | 0.83 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.92 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.031 | 0.047 | 0.92 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.031 | 0.92 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.92 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.220 | 0.230 | 0.10 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.92 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.92 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.92 | 0.123 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.92 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.92 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.92 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.92 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/compile_ms | 14.713 | 0.000 | 3925.52 | 87731.5 |
| pipelines/driver_ms | 12.906 | 0.000 | 3754.56 | 76959.4 |
| render/thread/ms | 9.509 | 11.998 | 36.31 | 56701.3 |
| main/busy/ms | 9.151 | 11.135 | 31.37 | 54565.0 |
| render/busy/ms | 8.990 | 11.515 | 35.77 | 53605.6 |
| pipelines/wait_ms | 8.438 | 0.000 | 8178.55 | 50315.9 |
| encode//sum | 7.005 | 9.974 | 46.31 | 41768.3 |
| stage/main/post_update | 5.626 | 7.183 | 24.12 | 33550.5 |
| stage/render/render | 5.450 | 7.064 | 16.06 | 32497.2 |
| render/graph/ms | 4.731 | 6.318 | 15.25 | 28211.7 |
| stage/render/graph_submit | 3.548 | 4.825 | 13.61 | 21154.1 |
| encode/submit | 2.285 | 3.247 | 6.84 | 13624.1 |
| stage/main/update | 2.218 | 2.721 | 24.56 | 13227.5 |
| pipelines/front_ms | 1.806 | 0.000 | 227.15 | 10772.1 |
| encode/finish_wall | 1.335 | 1.992 | 10.96 | 7962.0 |
| stage/render/camera_driver | 1.128 | 1.576 | 4.96 | 6728.8 |
| stage/render/prepare_write_phase_buffers | 0.760 | 1.111 | 2.35 | 4532.0 |
| stage/render/prepare_assets | 0.581 | 0.760 | 10.66 | 3466.4 |
| bigspace/total/ms | 0.532 | 1.301 | 2.38 | 3173.8 |
| stage/render/prepare_resources | 0.453 | 0.553 | 1.52 | 2701.9 |
| stage/render/prepare_meshes | 0.449 | 0.575 | 8.32 | 2676.2 |
| render/present/ms | 0.422 | 1.072 | 2.99 | 2514.5 |
| stage/render/specialize | 0.421 | 0.597 | 13.06 | 2511.2 |
| stage/main/first | 0.418 | 0.555 | 6.53 | 2492.5 |
| stage/main/pre_update | 0.367 | 0.555 | 1.64 | 2186.1 |
| terrain/update/ms | 0.304 | 0.387 | 2.03 | 1812.8 |
| stage/render/queue | 0.296 | 0.440 | 7.13 | 1766.7 |
| controller/feed/ms | 0.283 | 0.365 | 0.79 | 1688.6 |
| stage/render/prepare_bind_groups | 0.283 | 0.511 | 3.03 | 1686.9 |
| terrain/select/ms | 0.270 | 0.343 | 0.85 | 1607.0 |
| stream/drive/ms | 0.254 | 0.359 | 20.70 | 1516.1 |
| bigspace/low_precision/ms | 0.220 | 0.686 | 1.60 | 1313.7 |
| stage/main/fixed_main_loop | 0.218 | 0.382 | 1.60 | 1300.9 |
| far/select/ms | 0.204 | 0.281 | 0.51 | 1218.8 |
| stage/render/create_views | 0.204 | 0.272 | 1.16 | 1214.8 |
| stream/tick/ms | 0.196 | 0.277 | 0.62 | 1170.7 |
| stage/render/prepare_batch_phases | 0.195 | 0.278 | 0.69 | 1165.5 |
| stage/render/extract_commands | 0.175 | 0.214 | 0.57 | 1043.2 |
| stage/render/prepare_flush | 0.166 | 0.351 | 1.58 | 989.7 |
| bigspace/high_precision/ms | 0.154 | 0.580 | 2.15 | 917.5 |
| anim/evaluate/ms | 0.153 | 0.266 | 1.38 | 912.7 |
| anim/evaluate_cpu/ms | 0.146 | 0.186 | 0.61 | 867.7 |
| stage/render/prepare_views | 0.142 | 0.180 | 0.45 | 844.8 |
| stage/main/last | 0.122 | 0.192 | 1.50 | 730.2 |
| controller/move/ms | 0.118 | 0.201 | 0.52 | 704.9 |
| render/acquire/ms | 0.097 | 0.125 | 0.39 | 581.2 |
| stage/render/prepare_collect_phase_buffers | 0.070 | 0.144 | 0.42 | 419.4 |
| physics/step/ms | 0.047 | 0.087 | 0.59 | 282.5 |
| encode/resolve_diagnostics/sum | 0.033 | 0.047 | 0.52 | 199.6 |
| physics/insert/ms | 0.033 | 0.371 | 0.79 | 198.6 |
| stage/render/cleanup | 0.032 | 0.093 | 0.79 | 190.0 |
| stage/render/phase_sort | 0.028 | 0.049 | 0.23 | 164.9 |
| weather/map/ms | 0.026 | 0.034 | 0.21 | 153.1 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.024 | 0.50 | 93.9 |
| encode/camera_driver_span_end/sum | 0.011 | 0.021 | 0.58 | 68.4 |
| anim/apply/ms | 0.011 | 0.021 | 0.18 | 66.5 |
| controller/avoid/ms | 0.011 | 0.015 | 0.21 | 66.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.31 | 65.7 |
| script/update/ms | 0.007 | 0.010 | 0.12 | 43.8 |
| script/ui/ms | 0.005 | 0.007 | 0.02 | 32.6 |
| stream/despawn/ms | 0.005 | 0.000 | 1.01 | 30.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.14 | 29.6 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 27.1 |
| controller/write/ms | 0.004 | 0.005 | 0.02 | 21.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.69 | 14.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.53 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.8 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.20 | 0.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1258 | 2262 | 583 |
| crowd/c0/meshes/count | count | 343 | 646 | 162 |
| crowd/c1/count | count | 147 | 156 | 148 |
| crowd/c1/joints/count | count | 5535 | 5982 | 5613 |
| crowd/c1/meshes/count | count | 1173 | 1284 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4613 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4597 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.97 | 10.23 | 0.96 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.50 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.58 | 0.02 |
| encode/finish_critical | ms | 0.97 | 10.23 | 0.96 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.52 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.42 | 35.77 | 8.32 |
| gpu/timed/ms | ms | 7.93 | 9.94 | 8.32 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 801 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 801 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1286 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3364 | 5113 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1286 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3364 | 5113 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 14 | 200 | 4 |
| phases/shadow/bins | count | 5074 | 8751 | 2423 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6374 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 206 | 259 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 187 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 273 | 329 | 329 |
| pipelines/waiting | count | 1 | 9 | 0 |
| precache/draw_waiting | count | 0 | 7 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 1 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 31 | 76 | 0 |
| preprocess/bind_groups | count | 12 | 557 | 9 |
| preprocess/bind_groups_created | count | 12 | 557 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 128 | 204 | 120 |
| process/cpu_usage | % | 32.64 | 82.69 | 25.96 |
| process/mem_usage | GiB | 5.66 | 5.91 | 5.91 |
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
| skin/blocks_changed/count | count | 13 | 128 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 366.6 k | 807.7 k | 489.5 k |
| skin/joints_written/count | count | 5728 | 12.6 k | 7648 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4917 | 5010 | 5010 |
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
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 38.64 | 84.66 | 29.95 |
| system/mem_usage | % | 64.36 | 64.99 | 64.97 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 63 | 247 | 54 |
| views/all/bins | count | 7622 | 11.7 k | 5921 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 251 | 306 | 75 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 10.8 k | 33.7 k | 8127 |
| views/cascade/batch_sets | count | 9 | 51 | 4 |
| views/cascade/bins | count | 2788 | 6146 | 2423 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8919 | 23.5 k | 6374 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 409 | 23.1 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 192 | 0 |
| views/lamp_face/bins | count | 2216 | 2743 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2474 | 3037 | 0 |
| views/lamp_face/pipelines | count | 170 | 208 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2929 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2882 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8338 | 13.0 k | 7618 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8506 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 47.78 | 193 | 284.9 k |
| anim/events/count | count | 0.09 | 3 | 509 |
| assets/Image/added | count | 0.10 | 38 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.68 | 152 | 4061 |
| assets/Mesh/removed | count | 0.81 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.63 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 23.03 | 1377 | 137.3 k |
| ecs/entities_delta | count | 0.78 | 1191 | 4674 |
| encode//count | count | 38.79 | 273 | 231.3 k |
| encode/bind_walk/init_textures | count | 12.1 k | 114.7 k | 71.92 M |
| encode/bind_walk/init_textures_skipped | count | 59.5 k | 128.2 k | 354.70 M |
| encode/bind_walk/inits | count | 454 | 1123 | 2.71 M |
| encode/bind_walk/inits_skipped | count | 103 | 156 | 613.9 k |
| encode/bind_walk/merge_textures | count | 12.0 k | 114.6 k | 71.49 M |
| encode/bind_walk/merge_textures_skipped | count | 5379 | 6012 | 32.08 M |
| encode/bind_walk/merges | count | 97.12 | 336 | 579.1 k |
| encode/bind_walk/merges_skipped | count | 81.21 | 127 | 484.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5963 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5963 |
| encode/queue/command_buffers | count | 43.80 | 278 | 261.2 k |
| encode/queue/submits | count | 2.02 | 4 | 12.0 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.87 M | 11.23 G |
| encode/queue/write_buffer/calls | count | 253 | 831 | 1.51 M |
| encode/queue/write_buffer_with/bytes | bytes | 178.0 k | 3.52 M | 1.06 G |
| encode/queue/write_buffer_with/calls | count | 11.03 | 465 | 65.8 k |
| encode/queue/write_texture/bytes | bytes | 10.5 k | 8.39 M | 62.48 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 562 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5963 |
| physics/inserts | count | 0.38 | 110 | 2287 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 12 |
| pipelines/created | count | 0.02 | 2 | 120 |
| precache/hit | count | 0.00 | 2 | 3 |
| precache/missed | count | 0.00 | 2 | 7 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 18 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 21 | 378 |
| stream/upload/bytes | bytes | 5677 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1245 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.8 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2509.6 | main/busy/ms 2482, stage/main/post_update 2377, stage/main/update 97.37, surface/advance_cpu_ms 68.11, stage/main/pre_update 4.38, stage/main/last 1.02, surface/cell_upload_ms 0.70, stage/main/first 0.60, stage/main/fixed_main_loop 0.45, stream/drive/ms 0.37, weather/map/ms 0.23, bigspace/total/ms 0.08 |
| warm-up | 3.25 | 744.7 | main/busy/ms 2618, stage/main/post_update 2398, render/thread/ms 712, render/busy/ms 531, stage/render/prepare_assets 484, stage/main/update 207, stage/render/prepare_views 177, render/acquire/ms 177, surface/advance_cpu_ms 68.11, stage/render/create_views 26.07, stage/render/render 14.64, render/graph/ms 9.29 |
| warm-up | 3.40 | 149.9 | render/thread/ms 840, render/busy/ms 605, stage/render/prepare_assets 528, pipelines/compile_ms 423, pipelines/driver_ms 328, stage/render/prepare_views 178, render/acquire/ms 177, main/busy/ms 160, stage/main/update 115, pipelines/front_ms 94.40, stage/render/render 88.62, render/present/ms 57.75 |
| warm-up | 3.57 | 113.3 | pipelines/compile_ms 738, pipelines/driver_ms 622, pipelines/wait_ms 203, main/busy/ms 195, render/thread/ms 145, stage/main/update 123, pipelines/front_ms 116, render/busy/ms 108, stage/render/render 73.94, stage/main/post_update 63.02, render/present/ms 36.42, render/graph/ms 36.42 |
| warm-up | 3.73 | 165.4 | pipelines/compile_ms 1976, pipelines/wait_ms 1872, pipelines/driver_ms 1711, pipelines/front_ms 264, render/thread/ms 206, render/busy/ms 172, main/busy/ms 167, stage/main/update 126, stage/render/prepare_assets 98.14, stage/render/render 39.52, render/present/ms 34.31, stage/main/post_update 33.76 |
| warm-up | 3.89 | 50.7 | pipelines/wait_ms 1036, pipelines/compile_ms 209, pipelines/driver_ms 183, render/thread/ms 87.67, render/busy/ms 85.96, main/busy/ms 48.72, stage/main/post_update 34.61, pipelines/front_ms 25.77, stage/render/prepare_resources 23.30, stage/render/prepare_write_phase_buffers 22.33, stage/render/prepare_assets 18.63, stage/render/render 13.49 |
| warm-up | 3.95 | 63.3 | pipelines/wait_ms 804, pipelines/compile_ms 140, pipelines/driver_ms 115, render/thread/ms 111, render/busy/ms 109, main/busy/ms 93.30, stage/main/post_update 65.60, stage/render/prepare_resources 38.69, stage/render/prepare_assets 33.14, pipelines/front_ms 25.11, stage/main/update 23.15, stage/render/render 14.19 |
| warm-up | 4.02 | 74.9 | pipelines/wait_ms 4099, pipelines/compile_ms 761, pipelines/driver_ms 650, render/thread/ms 134, render/busy/ms 132, main/busy/ms 124, pipelines/front_ms 112, stage/main/update 67.17, stage/render/prepare_resources 62.63, stream/drive/ms 49.52, stage/main/post_update 43.50, stage/render/prepare_flush 27.14 |
| warm-up | 4.13 | 108.6 | pipelines/wait_ms 7936, pipelines/compile_ms 909, pipelines/driver_ms 697, pipelines/front_ms 212, render/thread/ms 168, render/busy/ms 166, main/busy/ms 105, stage/render/prepare_assets 68.82, stage/main/update 62.49, stream/drive/ms 48.90, stage/render/prepare_resources 39.28, stage/render/prepare_flush 37.21 |
| warm-up | 4.20 | 73.1 | pipelines/wait_ms 4702, pipelines/compile_ms 968, pipelines/driver_ms 733, pipelines/front_ms 235, render/thread/ms 165, render/busy/ms 163, stage/render/prepare_assets 82.28, main/busy/ms 71.34, stage/main/update 34.79, stage/render/prepare_resources 24.71, stage/main/post_update 23.24, stage/render/prepare_write_phase_buffers 18.25 |
| warm-up | 4.26 | 55.5 | pipelines/wait_ms 1811, pipelines/compile_ms 1008, pipelines/driver_ms 819, pipelines/front_ms 188, render/thread/ms 109, render/busy/ms 107, main/busy/ms 60.59, stage/render/prepare_assets 48.33, stage/main/update 39.32, stage/render/prepare_write_phase_buffers 18.46, stage/render/prepare_resources 16.87, stage/main/post_update 11.18 |
| warm-up | 4.39 | 132.3 | pipelines/wait_ms 5638, pipelines/compile_ms 1501, pipelines/driver_ms 1288, pipelines/front_ms 214, render/thread/ms 168, render/busy/ms 167, main/busy/ms 76.44, stage/render/prepare_assets 52.13, stage/main/update 49.23, stage/render/prepare_write_phase_buffers 41.79, stream/drive/ms 30.60, stage/render/prepare_resources 30.07 |
| warm-up | 4.48 | 85.7 | pipelines/wait_ms 5276, pipelines/compile_ms 1220, pipelines/driver_ms 1074, render/thread/ms 204, render/busy/ms 203, pipelines/front_ms 147, main/busy/ms 97.32, stage/render/prepare_assets 57.89, stage/render/prepare_write_phase_buffers 56.87, stage/main/post_update 50.41, stage/render/prepare_resources 45.98, stage/main/update 37.73 |
| warm-up | 4.69 | 213.5 | pipelines/wait_ms 1757, pipelines/compile_ms 1339, pipelines/driver_ms 1243, render/thread/ms 243, render/busy/ms 241, main/busy/ms 121, stage/render/prepare_assets 102, pipelines/front_ms 96.19, stage/render/prepare_resources 55.92, stage/main/post_update 54.10, stage/main/update 42.27, bigspace/total/ms 31.33 |
| warm-up | 4.87 | 174.4 | pipelines/compile_ms 1406, pipelines/wait_ms 1311, pipelines/driver_ms 1279, render/thread/ms 323, render/busy/ms 319, pipelines/front_ms 127, stage/render/prepare_assets 110, main/busy/ms 103, stage/render/prepare_resources 65.52, stage/render/render 64.95, render/graph/ms 60.13, stage/render/graph_submit 56.53 |
| warm-up | 4.93 | 60.6 | pipelines/wait_ms 528, render/thread/ms 212, render/busy/ms 208, pipelines/compile_ms 86.46, stage/render/render 68.98, render/graph/ms 63.76, stage/render/graph_submit 58.63, stage/render/prepare_assets 55.68, encode/submit 52.42, pipelines/driver_ms 51.93, main/busy/ms 49.21, stage/render/prepare_resources 35.36 |
| warm-up | 4.99 | 59.9 | render/thread/ms 108, render/busy/ms 106, main/busy/ms 47.49, stage/render/prepare_assets 32.13, encode//sum 20.01, stage/main/post_update 18.29, stage/main/update 17.73, stage/render/prepare_resources 16.13, stage/render/render 16.01, render/graph/ms 12.98, stage/render/prepare_write_phase_buffers 10.99, stage/render/prepare_bind_groups 8.46 |
| warm-up | 5.07 | 86.4 | render/thread/ms 129, render/busy/ms 127, main/busy/ms 96.51, stage/main/post_update 69.60, stage/render/prepare_assets 51.38, encode//sum 30.05, anim/evaluate/ms 22.52, stage/render/render 18.12, stage/main/update 16.40, stage/render/prepare_flush 16.26, render/graph/ms 15.26, stage/render/prepare_write_phase_buffers 12.36 |
| warm-up | 5.13 | 52.5 | render/thread/ms 126, render/busy/ms 124, main/busy/ms 114, stage/main/post_update 78.45, stage/render/prepare_assets 56.75, anim/evaluate/ms 30.71, encode//sum 18.95, stage/main/update 17.18, stage/render/prepare_resources 15.64, stage/main/fixed_main_loop 14.98, stage/render/render 14.17, stage/render/prepare_flush 13.71 |
| warm-up | 5.18 | 54.7 | render/thread/ms 91.53, render/busy/ms 90.07, main/busy/ms 75.47, stage/render/prepare_assets 35.98, stage/main/post_update 26.69, stage/main/update 23.78, stage/main/fixed_main_loop 16.06, stage/render/prepare_resources 14.01, stage/render/prepare_flush 11.78, anim/evaluate/ms 8.39, stage/render/render 8.30, stage/render/prepare_bind_groups 7.43 |
| warm-up | 5.42 | 191.3 | pipelines/wait_ms 10.1 k, pipelines/compile_ms 1232, pipelines/driver_ms 1112, main/busy/ms 222, render/thread/ms 157, render/busy/ms 155, stage/main/update 124, pipelines/front_ms 120, stream/drive/ms 66.03, stage/main/post_update 52.87, stage/render/prepare_bind_groups 44.82, stage/render/prepare_assets 30.13 |
| warm-up | 5.65 | 224.1 | pipelines/wait_ms 22.2 k, pipelines/compile_ms 1552, pipelines/driver_ms 1415, render/thread/ms 287, render/busy/ms 284, main/busy/ms 210, pipelines/front_ms 137, stage/render/prepare_assets 134, stage/main/update 120, stream/drive/ms 72.67, stage/render/prepare_bind_groups 46.78, stage/main/post_update 45.06 |
| warm-up | 5.71 | 59.8 | pipelines/wait_ms 13.8 k, pipelines/compile_ms 2688, pipelines/driver_ms 2498, render/thread/ms 223, render/busy/ms 221, pipelines/front_ms 190, stage/render/prepare_assets 136, main/busy/ms 130, stage/main/update 96.43, stream/drive/ms 72.85, stage/render/render 26.29, render/graph/ms 23.73 |
| warm-up | 5.82 | 117.7 | pipelines/wait_ms 10.1 k, pipelines/compile_ms 1424, pipelines/driver_ms 1301, render/thread/ms 158, render/busy/ms 156, main/busy/ms 130, pipelines/front_ms 122, stage/main/update 86.92, stream/drive/ms 56.37, stage/render/prepare_assets 48.59, stage/render/prepare_write_phase_buffers 36.44, stage/render/render 35.91 |
| warm-up | 5.91 | 81.5 | pipelines/wait_ms 9196, render/thread/ms 174, render/busy/ms 173, main/busy/ms 127, pipelines/compile_ms 93.72, pipelines/driver_ms 85.33, stage/main/update 83.55, stage/render/prepare_assets 60.34, stream/drive/ms 55.08, stage/render/render 36.02, stage/render/prepare_write_phase_buffers 34.18, encode//sum 33.73 |
| warm-up | 5.99 | 84.1 | pipelines/wait_ms 2336, pipelines/compile_ms 229, pipelines/driver_ms 198, render/thread/ms 142, render/busy/ms 141, main/busy/ms 96.02, stage/main/update 66.33, stage/render/prepare_assets 56.14, stream/drive/ms 46.85, stage/render/prepare_resources 34.83, pipelines/front_ms 31.29, stage/main/post_update 21.82 |
| warm-up | 6.14 | 57.9 | render/thread/ms 94.51, render/busy/ms 92.84, main/busy/ms 58.53, stage/main/post_update 34.24, stage/render/prepare_resources 30.29, stage/render/prepare_assets 16.94, stage/main/update 15.63, stage/render/render 13.17, encode//sum 11.74, bigspace/total/ms 11.59, stage/render/prepare_write_phase_buffers 11.16, render/graph/ms 10.76 |
| warm-up | 6.20 | 57.0 | render/thread/ms 103, render/busy/ms 101, main/busy/ms 48.62, stage/render/prepare_assets 30.33, stage/main/post_update 23.60, stage/render/prepare_write_phase_buffers 19.19, stage/main/update 16.77, stage/render/prepare_resources 14.92, stage/render/render 13.70, encode//sum 11.56, render/graph/ms 11.20, stage/render/graph_submit 8.54 |
| warm-up | 6.32 | 117.9 | render/thread/ms 169, render/busy/ms 165, main/busy/ms 71.60, stage/main/post_update 47.13, stage/render/prepare_resources 39.11, stage/render/prepare_write_phase_buffers 27.68, stage/render/prepare_assets 26.86, stage/main/update 17.17, stage/render/prepare_flush 16.61, stage/render/queue 14.33, stage/render/render 12.84, stage/render/prepare_bind_groups 11.45 |
| warm-up | 6.43 | 106.2 | pipelines/compile_ms 3155, pipelines/driver_ms 2975, pipelines/wait_ms 1400, render/thread/ms 215, render/busy/ms 209, pipelines/front_ms 180, main/busy/ms 73.26, stage/render/prepare_resources 59.43, stage/main/post_update 45.06, stage/render/prepare_flush 26.59, stage/render/prepare_assets 24.64, stage/render/prepare_bind_groups 22.75 |
| warm-up | 6.53 | 102.0 | pipelines/compile_ms 3155, pipelines/driver_ms 2975, pipelines/wait_ms 1400, pipelines/front_ms 180, render/thread/ms 178, render/busy/ms 173, main/busy/ms 88.56, stage/render/prepare_assets 50.24, stage/main/post_update 48.03, stage/render/prepare_resources 47.99, stage/main/update 30.68, stage/render/render 18.40 |
| warm-up | 6.58 | 53.1 | pipelines/compile_ms 3311, pipelines/driver_ms 3024, pipelines/front_ms 286, render/thread/ms 127, render/busy/ms 126, main/busy/ms 94.19, stage/render/prepare_resources 43.75, stage/render/prepare_assets 41.00, stage/main/post_update 39.91, stage/main/update 36.95, stream/drive/ms 16.91, stage/render/prepare_write_phase_buffers 11.42 |
| warm-up | 6.67 | 91.2 | pipelines/compile_ms 8030, pipelines/driver_ms 7329, pipelines/wait_ms 1018, pipelines/front_ms 701, render/thread/ms 129, render/busy/ms 128, main/busy/ms 89.60, stage/render/prepare_resources 48.80, stage/render/prepare_assets 45.16, stage/main/update 40.44, stage/main/post_update 32.08, stream/drive/ms 15.90 |
| warm-up | 6.78 | 64.3 | pipelines/wait_ms 5805, pipelines/compile_ms 113, render/thread/ms 101, render/busy/ms 98.62, pipelines/driver_ms 91.12, main/busy/ms 75.83, stage/render/prepare_assets 43.24, stage/main/post_update 29.61, stage/main/pre_update 24.05, pipelines/front_ms 21.54, stage/render/prepare_resources 18.46, stage/render/render 13.36 |
| warm-up | 6.84 | 52.8 | pipelines/wait_ms 5805, pipelines/compile_ms 113, render/thread/ms 107, render/busy/ms 104, pipelines/driver_ms 91.12, main/busy/ms 71.14, stage/render/prepare_assets 37.94, stage/main/post_update 28.32, pipelines/front_ms 21.54, stage/render/prepare_flush 20.21, stage/render/prepare_resources 18.53, stage/main/update 16.16 |
| warm-up | 6.91 | 73.5 | render/thread/ms 119, render/busy/ms 117, main/busy/ms 76.78, stage/render/prepare_flush 32.81, stage/main/post_update 28.33, stage/main/fixed_main_loop 27.49, stage/render/prepare_assets 26.19, stage/render/prepare_resources 25.71, stage/main/update 17.72, stage/render/render 13.50, render/graph/ms 11.08, encode//sum 8.50 |
| warm-up | 7.02 | 68.0 | pipelines/compile_ms 3633, pipelines/driver_ms 3303, pipelines/front_ms 330, render/thread/ms 105, render/busy/ms 104, main/busy/ms 61.70, stage/render/prepare_assets 39.20, stage/main/post_update 28.20, stage/render/prepare_write_phase_buffers 15.47, stage/main/fixed_main_loop 15.11, stage/main/update 14.98, stage/render/render 12.61 |
| warm-up | 7.17 | 56.3 | render/thread/ms 82.30, render/busy/ms 80.72, main/busy/ms 66.14, stage/render/prepare_assets 40.11, stage/main/post_update 29.63, stage/main/update 22.10, stage/render/render 11.57, anim/evaluate/ms 10.04, stage/render/prepare_meshes 9.35, render/graph/ms 9.34, stage/main/pre_update 9.05, bigspace/total/ms 8.12 |
| warm-up | 7.45 | 80.5 | pipelines/wait_ms 11.8 k, pipelines/compile_ms 115, render/thread/ms 114, render/busy/ms 112, pipelines/driver_ms 103, main/busy/ms 84.63, stage/main/post_update 54.86, stage/render/prepare_assets 30.55, stage/render/prepare_write_phase_buffers 24.84, stage/main/update 21.50, stage/render/render 16.33, render/graph/ms 13.34 |
| warm-up | 7.53 | 75.2 | pipelines/wait_ms 7875, render/thread/ms 147, render/busy/ms 145, main/busy/ms 114, pipelines/compile_ms 98.07, pipelines/driver_ms 86.10, stage/main/post_update 62.45, stage/render/prepare_flush 44.00, stage/render/prepare_assets 37.44, stage/main/update 25.18, stage/render/prepare_write_phase_buffers 20.08, stream/drive/ms 18.87 |
| warm-up | 7.63 | 55.8 | pipelines/compile_ms 2797, pipelines/driver_ms 2595, pipelines/wait_ms 614, pipelines/front_ms 202, render/thread/ms 88.50, render/busy/ms 86.92, main/busy/ms 36.94, stage/render/prepare_assets 33.30, stage/main/post_update 20.78, stage/render/prepare_write_phase_buffers 16.07, stage/render/render 16.00, render/graph/ms 13.44 |
| warm-up | 8.05 | 83.7 | render/thread/ms 87.28, render/busy/ms 85.47, main/busy/ms 45.76, stage/render/prepare_resources 28.58, stage/main/update 24.92, stage/render/prepare_flush 19.04, stage/main/post_update 13.79, stage/render/render 13.73, render/graph/ms 11.27, encode//sum 10.61, stage/render/prepare_write_phase_buffers 10.12, stream/drive/ms 8.96 |
| warm-up | 8.11 | 64.7 | pipelines/wait_ms 4517, render/thread/ms 123, render/busy/ms 120, pipelines/compile_ms 119, pipelines/driver_ms 102, main/busy/ms 43.01, stage/render/prepare_resources 35.93, stage/render/prepare_assets 27.35, stage/main/update 20.50, stage/render/render 18.14, pipelines/front_ms 17.41, stage/main/post_update 17.12 |
| warm-up | 8.43 | 66.0 | render/thread/ms 92.39, render/busy/ms 90.76, main/busy/ms 52.71, stage/render/prepare_resources 35.21, stage/main/post_update 21.22, stage/main/update 19.99, stage/render/render 14.92, encode//sum 13.87, stage/render/prepare_assets 13.41, render/graph/ms 12.76, stage/render/graph_submit 9.96, stream/drive/ms 7.33 |
| warm-up | 8.52 | 94.4 | pipelines/compile_ms 2862, pipelines/driver_ms 2728, pipelines/wait_ms 2191, render/thread/ms 151, render/busy/ms 150, pipelines/front_ms 133, stage/render/prepare_resources 60.29, main/busy/ms 42.53, stage/render/prepare_write_phase_buffers 22.18, stage/render/prepare_assets 19.35, stage/main/update 16.55, stage/render/render 14.26 |
| warm-up | 8.61 | 86.3 | pipelines/compile_ms 2967, pipelines/wait_ms 2956, pipelines/driver_ms 2808, render/thread/ms 161, pipelines/front_ms 160, render/busy/ms 159, main/busy/ms 66.24, stage/render/prepare_resources 53.67, stage/render/prepare_assets 36.53, stage/main/post_update 25.73, stage/render/prepare_write_phase_buffers 21.58, stage/main/update 19.28 |
| warm-up | 8.66 | 52.6 | pipelines/wait_ms 11.5 k, pipelines/compile_ms 4510, pipelines/driver_ms 4093, pipelines/front_ms 417, render/thread/ms 119, render/busy/ms 117, main/busy/ms 68.68, stage/render/prepare_resources 39.07, stage/render/prepare_assets 34.42, stage/main/post_update 26.00, stage/render/render 20.15, encode//sum 18.72 |
| warm-up | 8.74 | 75.3 | pipelines/wait_ms 17.5 k, pipelines/compile_ms 5382, pipelines/driver_ms 4846, pipelines/front_ms 536, render/thread/ms 121, render/busy/ms 119, main/busy/ms 56.10, stage/main/post_update 31.43, stage/render/render 29.57, encode//sum 27.90, render/graph/ms 27.13, stage/render/prepare_resources 27.13 |
| warm-up | 8.80 | 65.1 | pipelines/wait_ms 13.3 k, pipelines/compile_ms 1110, pipelines/driver_ms 926, pipelines/front_ms 184, render/thread/ms 135, render/busy/ms 133, main/busy/ms 80.86, stage/main/post_update 41.22, stage/render/prepare_flush 27.45, stage/render/prepare_resources 24.33, stage/render/render 24.16, encode//sum 23.70 |
| warm-up | 9.25 | 71.7 | pipelines/wait_ms 13.4 k, pipelines/compile_ms 2862, pipelines/driver_ms 2660, pipelines/front_ms 202, render/thread/ms 97.23, render/busy/ms 95.63, main/busy/ms 91.31, stage/main/post_update 47.57, stage/render/prepare_resources 35.49, stage/main/update 33.46, stage/render/render 18.82, render/graph/ms 16.62 |
| warm-up | 9.32 | 71.0 | pipelines/wait_ms 24.4 k, pipelines/compile_ms 4821, pipelines/driver_ms 4417, pipelines/front_ms 404, render/thread/ms 132, render/busy/ms 130, main/busy/ms 91.32, stage/main/post_update 49.06, stage/render/prepare_assets 33.37, stage/render/prepare_resources 30.89, stage/main/update 30.85, stage/render/render 21.73 |
| warm-up | 9.37 | 51.7 | pipelines/wait_ms 11.0 k, pipelines/compile_ms 1960, pipelines/driver_ms 1757, pipelines/front_ms 202, render/thread/ms 104, render/busy/ms 102, stage/render/prepare_assets 50.74, main/busy/ms 43.30, encode//sum 20.12, stage/main/post_update 19.83, stage/render/render 17.04, stage/main/update 15.38 |
| warm-up | 9.55 | 53.3 | pipelines/wait_ms 11.1 k, pipelines/compile_ms 3526, pipelines/driver_ms 2995, pipelines/front_ms 532, render/thread/ms 77.42, render/busy/ms 75.70, main/busy/ms 71.63, stage/main/post_update 37.24, stage/render/prepare_assets 21.26, stage/main/update 20.20, stage/render/prepare_resources 20.19, stage/render/render 14.16 |
| warm-up | 9.63 | 81.0 | pipelines/wait_ms 21.6 k, pipelines/compile_ms 540, pipelines/driver_ms 373, pipelines/front_ms 167, render/thread/ms 121, render/busy/ms 120, main/busy/ms 68.37, stage/render/prepare_write_phase_buffers 35.95, stage/main/post_update 34.40, stage/render/prepare_resources 26.96, stage/render/prepare_assets 25.04, encode//sum 18.50 |
| warm-up | 9.70 | 63.1 | pipelines/wait_ms 25.9 k, pipelines/compile_ms 389, pipelines/driver_ms 260, render/thread/ms 135, render/busy/ms 134, pipelines/front_ms 129, main/busy/ms 49.40, stage/render/prepare_write_phase_buffers 43.94, stage/render/prepare_assets 35.91, stage/render/prepare_resources 22.53, stage/main/post_update 22.17, encode//sum 21.69 |
| warm-up | 9.76 | 66.1 | pipelines/wait_ms 11.0 k, render/thread/ms 120, render/busy/ms 119, stage/render/prepare_assets 64.39, pipelines/compile_ms 53.18, main/busy/ms 52.58, pipelines/front_ms 36.93, stage/main/post_update 25.81, encode//sum 18.38, stage/main/update 17.62, pipelines/driver_ms 16.26, stage/render/prepare_resources 15.64 |
| warm-up | 9.91 | 123.3 | pipelines/wait_ms 20.0 k, pipelines/compile_ms 333, pipelines/driver_ms 251, render/thread/ms 137, render/busy/ms 124, main/busy/ms 107, pipelines/front_ms 81.59, stage/main/post_update 79.13, stage/render/prepare_write_phase_buffers 32.21, stage/render/prepare_assets 30.94, bigspace/total/ms 26.44, stage/render/prepare_resources 21.89 |
| warm-up | 9.98 | 76.5 | pipelines/wait_ms 10.2 k, render/thread/ms 194, render/busy/ms 181, main/busy/ms 125, stage/main/post_update 88.69, pipelines/compile_ms 51.96, stage/render/prepare_write_phase_buffers 40.12, stage/render/prepare_assets 37.68, stage/render/prepare_flush 36.07, stage/render/prepare_resources 33.26, bigspace/total/ms 32.63, pipelines/front_ms 29.38 |
| warm-up | 10.06 | 76.8 | render/thread/ms 147, render/busy/ms 146, main/busy/ms 89.81, stage/render/prepare_assets 48.74, stage/main/update 41.10, stage/main/post_update 31.64, stage/render/prepare_resources 25.58, stream/drive/ms 25.03, stage/render/prepare_flush 24.60, stage/render/render 18.11, encode//sum 16.71, render/graph/ms 15.62 |
| warm-up | 10.51 | 62.1 | render/thread/ms 82.11, render/busy/ms 80.52, main/busy/ms 62.31, stage/main/update 35.82, stream/drive/ms 24.94, stream/spawn/ms 22.68, encode//sum 22.58, stage/render/prepare_flush 21.35, stage/render/render 19.60, stage/main/post_update 19.39, stage/render/prepare_assets 18.12, render/graph/ms 17.19 |
| warm-up | 10.73 | 91.7 | pipelines/wait_ms 11.4 k, pipelines/compile_ms 143, render/thread/ms 102, render/busy/ms 101, pipelines/driver_ms 99.59, main/busy/ms 51.97, pipelines/front_ms 43.82, stage/main/post_update 25.98, encode//sum 25.17, stage/render/render 22.28, render/graph/ms 19.90, stage/render/specialize 17.37 |
| warm-up | 12.95 | 114.7 | pipelines/wait_ms 19.4 k, pipelines/compile_ms 444, pipelines/front_ms 307, render/thread/ms 157, render/busy/ms 155, pipelines/driver_ms 137, stage/render/render 58.11, render/graph/ms 55.71, encode//sum 52.58, stage/render/graph_submit 48.08, stage/render/prepare_write_phase_buffers 41.04, main/busy/ms 37.64 |
| warm-up | 13.03 | 78.9 | pipelines/wait_ms 19.4 k, pipelines/compile_ms 444, pipelines/front_ms 307, render/thread/ms 190, render/busy/ms 189, pipelines/driver_ms 137, stage/render/prepare_write_phase_buffers 70.12, encode//sum 40.01, stage/render/prepare_resources 39.26, main/busy/ms 36.58, stage/render/render 34.83, render/graph/ms 32.24 |
| warm-up | 13.54 | 58.6 | render/thread/ms 71.37, render/busy/ms 69.66, main/busy/ms 35.62, stage/main/post_update 20.70, encode//sum 19.55, stage/render/specialize 19.34, stage/render/render 18.18, render/graph/ms 16.00, stage/render/queue 14.97, stage/render/graph_submit 11.91, stage/main/update 10.90, encode/submit 6.83 |

## Run

- `../target/profiling/walkonce.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set render.wgpu.bind_group_walk_once=on --resolution 2560x1440 --bench-out target/perf\wgpu_walk_once\balmora_walk@2560x1440_arm=once-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1386 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **76%** (min 41%) · 2620 MHz (min 2610) · memory 8101 MHz · 79.7 W · 76 °C (max 83) · VRAM 6694 MiB · throttled 100% of samples

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
- `render.wgpu.bind_group_walk_once=on (command line: --set)`
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
