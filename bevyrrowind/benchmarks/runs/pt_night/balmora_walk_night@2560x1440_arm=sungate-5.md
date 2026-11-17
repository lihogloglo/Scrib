# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.gate=on (command line: --set); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6033 frames over 65.6 s · **median 10.41 ms (96 fps)** · mean 10.88 · p95 14.47 · p99 16.12 (1% low 62 fps) · worst 32.96 ms · 109 spikes over 1.5× median · 21.0 stutters/min (work 20.1/min)
- warm-up: 324 frames over 9.5 s, worst 2493.9 ms
- hitches (> 26.0 ms): **4** measured, 51 in warm-up
- timed GPU passes: 6.64 ms of a 10.88 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.41 ms, limit 11 ms
- **FAIL** frame_ms: 23 stutters (> 1.5x moving median of 60) in 1.09 min = 21.02/min, limit 1/min
- pass worst frame 32.96 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.000 | 2.031 | 2.064 | 0.98 | 0.174 |
| main_opaque_pass_3d | 1.282 | 1.302 | 1.636 | 0.99 | 0.065 |
| early prepass | 0.479 | 0.486 | 0.699 | 0.99 | 0.085 |
| bloom | 0.311 | 0.316 | 0.322 | 0.98 | 0.062 |
| atmosphere_luts | 0.304 | 0.308 | 0.371 | 0.99 | 0.003 |
| auxiliary_views | 0.261 | 0.265 | 1.122 | 0.99 | 0.240 |
| water_velocity | 0.258 | 0.262 | 0.350 | 0.98 | 0.002 |
| water_sspr | 0.198 | 0.201 | 0.203 | 0.99 | 0.002 |
| ssao | 0.186 | 0.189 | 0.193 | 0.99 | 0.007 |
| upscaling | 0.127 | 0.129 | 0.137 | 0.98 | 0.004 |
| water_fft | 0.126 | 0.128 | 0.136 | 0.99 | 0.003 |
| atmos_froxel_inject | 0.125 | 0.127 | 0.129 | 0.98 | 0.001 |
| uniform_allocation | 0.124 | 0.125 | 0.154 | 0.99 | 0.026 |
| clouds_composite | 0.103 | 0.105 | 0.108 | 0.98 | 0.002 |
| bin_unpacking | 0.095 | 0.097 | 0.118 | 0.99 | 0.018 |
| tonemapping | 0.085 | 0.086 | 0.091 | 0.98 | 0.003 |
| main_transmissive_pass_3d | 0.077 | 0.078 | 0.156 | 0.99 | 0.008 |
| clustering | 0.062 | 0.063 | 0.067 | 0.99 | 0.019 |
| clouds_trace | 0.055 | 0.056 | 0.074 | 0.98 | 0.001 |
| render_sky | 0.051 | 0.051 | 0.052 | 0.98 | 0.002 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 0.98 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.411 | 0.415 | 0.11 | 0.001 |
| surface_trail | 0.039 | 0.056 | 0.057 | 0.70 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.98 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.028 | 0.036 | 0.99 | 0.004 |
| lightprobe_radiance_map | 0.025 | 0.218 | 0.229 | 0.11 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.029 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.035 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.024 | 0.98 | 0.123 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.99 | 0.005 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 0.99 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.484 | 13.044 | 24.18 | 57215.0 |
| main/busy/ms | 9.003 | 10.933 | 28.81 | 54314.9 |
| render/busy/ms | 8.966 | 12.496 | 23.42 | 54091.9 |
| stage/render/render | 5.743 | 8.642 | 16.12 | 34649.5 |
| stage/main/post_update | 5.633 | 7.208 | 10.16 | 33983.2 |
| render/graph/ms | 5.043 | 7.882 | 14.95 | 30421.8 |
| encode//sum | 4.357 | 6.835 | 27.58 | 26286.3 |
| stage/render/graph_submit | 3.773 | 5.780 | 11.11 | 22765.0 |
| encode/submit | 2.435 | 3.917 | 6.66 | 14691.3 |
| stage/main/update | 2.124 | 2.576 | 22.01 | 12814.0 |
| encode/finish_wall | 1.378 | 1.961 | 4.87 | 8312.6 |
| encode/pass:shadow_point_light/sum | 1.350 | 5.353 | 16.32 | 8147.0 |
| stage/render/camera_driver | 1.216 | 2.026 | 3.95 | 7337.1 |
| encode/pass:shadow_directional_light/sum | 0.760 | 1.249 | 2.13 | 4582.1 |
| stage/render/prepare_write_phase_buffers | 0.755 | 1.101 | 3.28 | 4556.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.748 | 1.084 | 1.67 | 4511.9 |
| encode/pass:early prepass/sum | 0.733 | 1.074 | 1.84 | 4421.2 |
| stage/render/prepare_assets | 0.572 | 0.722 | 10.68 | 3450.1 |
| bigspace/total/ms | 0.561 | 1.371 | 2.47 | 3384.5 |
| stage/render/prepare_resources | 0.456 | 0.549 | 1.05 | 2751.8 |
| render/present/ms | 0.423 | 0.524 | 3.11 | 2551.9 |
| stage/render/prepare_meshes | 0.408 | 0.516 | 1.68 | 2461.9 |
| stage/main/first | 0.393 | 0.513 | 5.77 | 2370.0 |
| stage/main/pre_update | 0.350 | 0.488 | 1.35 | 2109.5 |
| terrain/update/ms | 0.317 | 0.413 | 1.76 | 1910.5 |
| terrain/select/ms | 0.283 | 0.372 | 1.04 | 1706.9 |
| controller/feed/ms | 0.264 | 0.331 | 0.86 | 1593.2 |
| stage/render/specialize | 0.260 | 0.391 | 2.47 | 1568.9 |
| stage/render/prepare_bind_groups | 0.256 | 0.431 | 2.39 | 1545.0 |
| stream/drive/ms | 0.243 | 0.323 | 20.16 | 1465.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.242 | 0.356 | 1.18 | 1462.4 |
| stage/render/queue | 0.240 | 0.380 | 2.45 | 1445.2 |
| bigspace/low_precision/ms | 0.232 | 0.773 | 1.71 | 1397.8 |
| stage/main/fixed_main_loop | 0.215 | 0.367 | 0.91 | 1294.4 |
| far/select/ms | 0.200 | 0.268 | 0.60 | 1203.7 |
| stage/render/create_views | 0.192 | 0.252 | 0.92 | 1159.6 |
| stream/tick/ms | 0.187 | 0.252 | 0.67 | 1129.3 |
| stage/render/prepare_batch_phases | 0.185 | 0.265 | 0.62 | 1117.3 |
| stage/render/extract_commands | 0.170 | 0.198 | 0.42 | 1027.2 |
| stage/render/prepare_flush | 0.168 | 0.349 | 1.76 | 1014.5 |
| anim/evaluate/ms | 0.168 | 0.299 | 1.72 | 1013.3 |
| bigspace/high_precision/ms | 0.165 | 0.736 | 1.99 | 993.5 |
| anim/evaluate_cpu/ms | 0.154 | 0.191 | 0.80 | 929.8 |
| stage/render/prepare_views | 0.138 | 0.170 | 0.98 | 830.8 |
| controller/move/ms | 0.122 | 0.218 | 0.51 | 737.6 |
| stage/main/last | 0.113 | 0.155 | 1.57 | 681.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.102 | 0.170 | 0.68 | 616.4 |
| render/acquire/ms | 0.095 | 0.121 | 0.89 | 571.3 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.126 | 0.47 | 347.4 |
| physics/step/ms | 0.046 | 0.083 | 0.57 | 275.5 |
| stage/render/phase_sort | 0.034 | 0.058 | 0.24 | 206.1 |
| physics/insert/ms | 0.034 | 0.371 | 0.98 | 204.4 |
| stage/render/cleanup | 0.031 | 0.098 | 0.71 | 190.0 |
| encode/resolve_diagnostics/sum | 0.029 | 0.039 | 0.47 | 174.9 |
| pipelines/compile_ms | 0.016 | 0.000 | 86.86 | 99.2 |
| weather/map/ms | 0.016 | 0.020 | 0.18 | 96.9 |
| pipelines/front_ms | 0.016 | 0.000 | 83.92 | 96.1 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.22 | 85.3 |
| anim/apply/ms | 0.011 | 0.019 | 0.39 | 66.0 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.23 | 64.7 |
| controller/avoid/ms | 0.011 | 0.014 | 0.24 | 63.8 |
| weather/eval/ms | 0.009 | 0.012 | 0.20 | 57.2 |
| script/update/ms | 0.007 | 0.009 | 0.29 | 42.2 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 30.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.01 | 30.1 |
| stream/spawn/ms | 0.005 | 0.000 | 1.18 | 29.4 |
| script/ui/ms | 0.004 | 0.006 | 0.22 | 25.9 |
| controller/write/ms | 0.003 | 0.004 | 0.18 | 21.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.67 | 14.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.21 | 3.4 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.94 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.18 | 2.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.11 | 1.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.01 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1258 | 2262 | 658 |
| crowd/c0/meshes/count | count | 343 | 646 | 188 |
| crowd/c1/count | count | 146 | 155 | 147 |
| crowd/c1/joints/count | count | 5531 | 5894 | 5538 |
| crowd/c1/meshes/count | count | 1154 | 1254 | 1153 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 163 | 216 | 204 |
| crowd/joints_extracted/count | count | 3664 | 9436 | 5731 |
| crowd/joints_uploaded/count | count | 3630 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.42 | 1.86 | 0.42 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.22 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.23 | 0.01 |
| encode/finish_critical | ms | 0.95 | 2.13 | 1.06 |
| encode/pass:early prepass/max | ms | 0.76 | 1.84 | 1.00 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.77 | 1.67 | 1.06 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.68 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.18 | 0.30 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 2.13 | 0.61 |
| encode/pass:shadow_point_light/max | ms | 0.79 | 1.57 | 0.91 |
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
| frame/work/ms | ms | 9.16 | 28.81 | 8.01 |
| gpu/timed/ms | ms | 7.43 | 10.02 | 7.74 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
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
| phases/alpha_mask3d/items | count | 796 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 796 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1290 | 1399 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3352 | 5144 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1290 | 1399 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3352 | 5144 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 197 | 4 |
| phases/shadow/bins | count | 3702 | 4380 | 1481 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6119 | 7354 | 3025 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 204 | 234 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 217 | 476 | 19 |
| physics/pending | count | 0 | 185 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 11 | 559 | 6 |
| preprocess/bind_groups_created | count | 11 | 559 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 133 | 303 | 127 |
| process/cpu_usage | % | 26.44 | 41.44 | 25.24 |
| process/mem_usage | GiB | 5.50 | 5.67 | 5.67 |
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
| skin/blocks_changed/count | count | 13 | 125 | 10 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 163 | 216 | 204 |
| skin/joint_bytes_written/bytes | bytes | 281.3 k | 809.2 k | 447.5 k |
| skin/joints_written/count | count | 4396 | 12.6 k | 6992 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4919 | 5010 | 5010 |
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
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.72 | 40.67 | 31.38 |
| system/mem_usage | % | 55.11 | 55.73 | 55.52 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 69 | 248 | 56 |
| views/all/bins | count | 6179 | 7312 | 4989 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.5 k | 20.2 k | 10.6 k |
| views/all/pipelines | count | 251 | 281 | 67 |
| views/all/pipelines_distinct | count | 71 | 75 | 63 |
| views/all/work_items | count | 10.1 k | 18.1 k | 8144 |
| views/cascade/batch_sets | count | 8 | 27 | 4 |
| views/cascade/bins | count | 1442 | 1558 | 1481 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3393 | 4423 | 3025 |
| views/cascade/pipelines | count | 22 | 27 | 19 |
| views/cascade/work_items | count | 324 | 4420 | 222 |
| views/lamp_face/batch_sets | count | 0 | 189 | 0 |
| views/lamp_face/bins | count | 2448 | 2956 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2718 | 3272 | 0 |
| views/lamp_face/pipelines | count | 183 | 208 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2871 | 0 |
| views/main/batch_sets | count | 51 | 53 | 52 |
| views/main/bins | count | 2886 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8294 | 13.0 k | 7614 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8460 | 13.2 k | 7922 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.9 k | 30.8 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 45.54 | 189 | 274.7 k |
| anim/events/count | count | 0.09 | 4 | 551 |
| assets/Image/added | count | 0.10 | 32 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.59 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1304 | 4675 |
| encode//count | count | 37.83 | 219 | 228.3 k |
| encode/bind_walk/init_textures | count | 14.9 k | 112.9 k | 89.59 M |
| encode/bind_walk/init_textures_skipped | count | 54.4 k | 116.3 k | 328.48 M |
| encode/bind_walk/inits | count | 470 | 1123 | 2.83 M |
| encode/bind_walk/inits_skipped | count | 99.95 | 153 | 603.0 k |
| encode/bind_walk/merge_textures | count | 14.8 k | 112.9 k | 89.21 M |
| encode/bind_walk/merge_textures_skipped | count | 5391 | 6012 | 32.52 M |
| encode/bind_walk/merges | count | 105 | 340 | 631.4 k |
| encode/bind_walk/merges_skipped | count | 80.09 | 123 | 483.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6033 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6033 |
| encode/pass:early prepass/count | count | 1 | 1 | 6033 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6033 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6033 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6033 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6033 |
| encode/pass:shadow_point_light/count | count | 2.72 | 48 | 16.4 k |
| encode/queue/command_buffers | count | 50.57 | 277 | 305.1 k |
| encode/queue/submits | count | 2.02 | 4 | 12.2 k |
| encode/queue/write_buffer/bytes | bytes | 1.82 M | 2.71 M | 10.95 G |
| encode/queue/write_buffer/calls | count | 266 | 834 | 1.60 M |
| encode/queue/write_buffer_with/bytes | bytes | 169.8 k | 3.54 M | 1.02 G |
| encode/queue/write_buffer_with/calls | count | 11.00 | 465 | 66.4 k |
| encode/queue/write_texture/bytes | bytes | 10.5 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6033 |
| physics/inserts | count | 0.37 | 92 | 2261 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 161 | 2335 |
| physics/steps | count | 0.65 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 2 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5611 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1230 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.0 ms; slowest 55, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2493.9 | main/busy/ms 2462, stage/main/post_update 2422, stage/main/update 31.41, stage/main/pre_update 4.71, stage/main/last 1.14, surface/advance_cpu_ms 0.79, stage/main/first 0.51, stream/drive/ms 0.42, stage/main/fixed_main_loop 0.36, surface/cell_upload_ms 0.30, stream/tick/ms 0.17, bigspace/total/ms 0.12 |
| warm-up | 3.40 | 905.4 | main/busy/ms 2635, stage/main/post_update 2449, render/thread/ms 869, render/busy/ms 721, stage/render/prepare_assets 670, stage/main/update 170, stage/render/prepare_views 145, render/acquire/ms 145, stage/render/create_views 28.98, stage/render/render 12.50, render/graph/ms 8.03, stage/main/pre_update 7.24 |
| warm-up | 3.50 | 103.5 | render/thread/ms 954, render/busy/ms 761, stage/render/prepare_assets 698, main/busy/ms 193, stage/render/prepare_views 145, render/acquire/ms 145, stage/main/update 143, pipelines/compile_ms 118, pipelines/front_ms 86.34, stage/render/render 61.97, render/present/ms 47.63, stage/main/post_update 38.14 |
| warm-up | 3.66 | 129.4 | pipelines/wait_ms 3973, pipelines/compile_ms 1223, pipelines/front_ms 1138, render/thread/ms 120, main/busy/ms 120, render/busy/ms 87.79, pipelines/driver_ms 84.35, stage/main/post_update 63.01, stage/render/prepare_assets 30.28, stage/render/prepare_views 30.24, render/acquire/ms 29.78, stage/main/update 22.16 |
| warm-up | 3.72 | 64.1 | pipelines/wait_ms 4131, pipelines/compile_ms 1775, pipelines/front_ms 1662, main/busy/ms 176, render/thread/ms 167, render/busy/ms 136, pipelines/driver_ms 113, stage/main/post_update 55.07, stage/main/update 54.68, stage/render/prepare_assets 42.70, stage/render/prepare_resources 36.75, stage/main/pre_update 34.70 |
| warm-up | 3.83 | 112.2 | pipelines/compile_ms 1754, pipelines/front_ms 1657, pipelines/wait_ms 392, render/thread/ms 130, render/busy/ms 128, main/busy/ms 118, pipelines/driver_ms 96.29, stage/render/prepare_assets 71.36, stage/main/update 57.01, stage/main/pre_update 36.69, stage/render/prepare_resources 25.21, stage/main/post_update 19.42 |
| warm-up | 3.92 | 88.0 | pipelines/compile_ms 2380, pipelines/front_ms 2092, pipelines/wait_ms 349, pipelines/driver_ms 288, render/thread/ms 155, render/busy/ms 153, stage/render/prepare_assets 116, main/busy/ms 37.08, stage/render/render 29.90, render/graph/ms 27.41, encode//sum 23.97, stage/render/graph_submit 20.82 |
| warm-up | 3.95 | 28.8 | pipelines/compile_ms 1427, pipelines/front_ms 1188, pipelines/driver_ms 239, pipelines/wait_ms 128, render/thread/ms 113, render/busy/ms 111, stage/render/prepare_assets 68.45, main/busy/ms 32.64, stage/render/render 30.15, render/graph/ms 27.78, encode//sum 23.84, stage/render/graph_submit 21.37 |
| warm-up | 4.00 | 31.3 | pipelines/compile_ms 98.60, pipelines/front_ms 93.14, render/thread/ms 46.77, render/busy/ms 45.05, main/busy/ms 26.45, stage/render/prepare_assets 19.52, stage/main/post_update 15.40, stage/render/render 11.43, terrain/update/ms 9.90, render/graph/ms 9.10, encode//sum 7.97, stage/render/prepare_resources 7.02 |
| warm-up | 4.10 | 86.3 | pipelines/compile_ms 1589, pipelines/front_ms 1537, main/busy/ms 58.19, render/thread/ms 53.17, pipelines/driver_ms 52.08, render/busy/ms 51.59, stage/main/update 45.54, stream/drive/ms 41.42, stage/render/prepare_assets 32.55, stage/render/render 11.09, render/graph/ms 9.14, stage/main/post_update 8.29 |
| warm-up | 4.14 | 34.7 | pipelines/compile_ms 1649, pipelines/front_ms 1594, render/thread/ms 63.33, render/busy/ms 61.53, pipelines/driver_ms 54.53, stage/render/prepare_assets 38.08, main/busy/ms 38.02, stage/main/update 25.80, stage/render/render 17.96, render/graph/ms 15.73, stream/drive/ms 10.84, stage/render/graph_submit 9.74 |
| warm-up | 4.17 | 35.2 | pipelines/compile_ms 340, pipelines/front_ms 325, render/thread/ms 51.34, render/busy/ms 49.43, main/busy/ms 40.09, stage/main/update 25.87, stage/render/prepare_assets 24.98, stage/render/render 18.36, render/graph/ms 16.11, pipelines/driver_ms 14.87, stream/drive/ms 10.69, stage/render/graph_submit 9.36 |
| warm-up | 4.26 | 76.3 | pipelines/compile_ms 73.77, pipelines/front_ms 69.79, render/thread/ms 61.44, render/busy/ms 59.80, main/busy/ms 50.43, stage/render/prepare_assets 35.46, stage/main/update 33.24, stream/drive/ms 27.35, stage/render/render 11.53, stage/main/post_update 9.79, render/graph/ms 9.61, stage/render/graph_submit 7.24 |
| warm-up | 4.31 | 50.0 | pipelines/compile_ms 528, pipelines/front_ms 509, render/thread/ms 84.49, render/busy/ms 82.49, stage/render/prepare_assets 55.15, main/busy/ms 33.50, pipelines/driver_ms 19.76, stage/main/update 12.10, stage/render/render 11.98, stage/main/post_update 11.85, render/graph/ms 9.51, stage/render/graph_submit 6.74 |
| warm-up | 4.37 | 61.0 | pipelines/compile_ms 1391, pipelines/front_ms 1347, render/thread/ms 79.20, render/busy/ms 77.08, main/busy/ms 62.74, stage/render/prepare_assets 54.62, pipelines/driver_ms 44.68, stage/main/update 39.39, stage/main/post_update 15.68, stage/render/render 11.72, render/graph/ms 9.11, encode//sum 6.09 |
| warm-up | 4.45 | 78.1 | pipelines/compile_ms 1536, pipelines/front_ms 1489, render/thread/ms 91.69, render/busy/ms 89.68, main/busy/ms 74.30, pipelines/wait_ms 71.03, stage/render/prepare_assets 60.37, stage/main/update 50.29, pipelines/driver_ms 47.54, stage/main/post_update 14.51, stage/render/render 14.42, render/graph/ms 12.16 |
| warm-up | 4.51 | 59.0 | pipelines/compile_ms 1115, pipelines/front_ms 1073, pipelines/wait_ms 171, render/thread/ms 96.72, render/busy/ms 94.55, encode//sum 70.02, main/busy/ms 53.99, pipelines/driver_ms 42.48, stage/render/prepare_assets 40.85, stage/main/update 32.37, stage/render/render 26.45, render/graph/ms 24.15 |
| warm-up | 4.57 | 64.1 | pipelines/compile_ms 1358, pipelines/front_ms 1299, pipelines/wait_ms 146, render/thread/ms 106, render/busy/ms 104, encode//sum 77.30, pipelines/driver_ms 58.58, main/busy/ms 57.49, stage/render/render 33.40, render/graph/ms 30.92, stage/main/update 28.93, stage/render/prepare_assets 25.71 |
| warm-up | 4.68 | 101.0 | pipelines/compile_ms 2081, pipelines/front_ms 2008, pipelines/wait_ms 652, render/thread/ms 156, render/busy/ms 154, main/busy/ms 123, stage/main/post_update 88.89, pipelines/driver_ms 73.27, encode//sum 51.68, stage/render/render 36.49, render/graph/ms 33.95, stage/render/prepare_bind_groups 26.39 |
| warm-up | 4.73 | 53.0 | pipelines/compile_ms 2371, pipelines/front_ms 2299, pipelines/wait_ms 1398, render/thread/ms 141, render/busy/ms 139, main/busy/ms 108, stage/main/post_update 82.73, pipelines/driver_ms 71.81, encode//sum 50.06, stage/render/render 31.81, render/graph/ms 29.33, stage/render/prepare_assets 26.49 |
| warm-up | 4.80 | 73.1 | pipelines/compile_ms 1915, pipelines/front_ms 1860, pipelines/wait_ms 1014, main/busy/ms 93.30, render/thread/ms 87.46, render/busy/ms 86.00, stage/main/update 70.40, stream/drive/ms 60.62, pipelines/driver_ms 55.44, stage/render/prepare_assets 47.96, stage/render/render 21.12, render/graph/ms 18.83 |
| warm-up | 4.88 | 82.6 | pipelines/compile_ms 1767, pipelines/front_ms 1716, pipelines/wait_ms 1006, main/busy/ms 107, render/thread/ms 84.18, stage/main/update 82.45, render/busy/ms 82.10, stream/drive/ms 60.75, pipelines/driver_ms 51.08, stage/render/prepare_assets 46.00, stage/render/render 17.86, render/graph/ms 15.34 |
| warm-up | 4.96 | 80.4 | pipelines/compile_ms 1826, pipelines/front_ms 1745, pipelines/wait_ms 1129, main/busy/ms 130, render/thread/ms 108, stage/main/update 106, render/busy/ms 103, pipelines/driver_ms 81.02, stage/render/prepare_assets 53.97, stream/drive/ms 48.85, stage/render/render 33.35, render/graph/ms 28.38 |
| warm-up | 5.08 | 110.7 | pipelines/compile_ms 1814, pipelines/front_ms 1735, pipelines/wait_ms 358, render/thread/ms 128, main/busy/ms 128, render/busy/ms 124, stage/main/update 101, pipelines/driver_ms 78.99, stage/render/prepare_assets 59.98, stream/drive/ms 48.41, stage/render/render 45.46, render/graph/ms 40.81 |
| warm-up | 5.11 | 34.7 | pipelines/compile_ms 997, pipelines/front_ms 968, render/thread/ms 89.70, main/busy/ms 88.62, render/busy/ms 88.12, stage/main/update 63.03, stream/drive/ms 45.85, stage/render/render 36.18, render/graph/ms 33.77, stage/render/prepare_assets 31.96, pipelines/driver_ms 28.43, stage/render/camera_driver 17.22 |
| warm-up | 5.18 | 71.3 | pipelines/compile_ms 1370, pipelines/front_ms 1333, main/busy/ms 86.21, stage/main/update 66.97, render/thread/ms 60.29, render/busy/ms 58.58, stream/drive/ms 46.47, pipelines/driver_ms 36.78, stage/render/render 25.86, render/graph/ms 23.62, stage/render/graph_submit 18.15, stage/render/prepare_assets 17.88 |
| warm-up | 5.21 | 32.0 | pipelines/compile_ms 1498, pipelines/front_ms 1458, main/busy/ms 68.58, render/thread/ms 55.36, render/busy/ms 53.69, stage/main/update 51.51, pipelines/driver_ms 40.49, stream/drive/ms 32.16, stage/render/prepare_assets 19.77, stage/render/render 19.17, render/graph/ms 17.16, pipelines/wait_ms 13.84 |
| warm-up | 5.26 | 45.1 | pipelines/compile_ms 371, pipelines/front_ms 358, main/busy/ms 61.60, render/thread/ms 48.22, render/busy/ms 46.80, stage/main/update 45.39, stream/drive/ms 31.82, stage/render/render 17.47, render/graph/ms 15.59, pipelines/driver_ms 12.94, stage/render/prepare_assets 12.68, stage/render/graph_submit 11.46 |
| warm-up | 5.29 | 32.8 | pipelines/compile_ms 428, pipelines/front_ms 414, render/thread/ms 50.16, render/busy/ms 48.83, main/busy/ms 35.94, stage/main/update 16.83, stage/render/render 15.79, pipelines/driver_ms 14.12, render/graph/ms 14.08, stage/render/prepare_assets 11.67, stage/main/post_update 11.46, encode//sum 11.24 |
| warm-up | 5.34 | 28.0 | pipelines/compile_ms 453, pipelines/front_ms 437, render/thread/ms 45.40, render/busy/ms 43.90, main/busy/ms 34.41, stage/render/render 16.72, pipelines/driver_ms 16.58, render/graph/ms 14.56, stage/main/post_update 14.36, stage/main/update 13.73, encode//sum 9.94, stage/render/graph_submit 9.92 |
| warm-up | 5.37 | 28.8 | pipelines/compile_ms 626, pipelines/front_ms 606, render/thread/ms 49.62, render/busy/ms 47.67, main/busy/ms 35.29, stage/render/render 19.64, pipelines/driver_ms 19.34, render/graph/ms 17.14, stage/main/post_update 16.36, stage/main/update 13.51, stage/render/graph_submit 11.83, encode//sum 11.03 |
| warm-up | 5.40 | 29.6 | pipelines/compile_ms 355, pipelines/front_ms 347, render/thread/ms 51.69, render/busy/ms 49.27, main/busy/ms 34.91, stage/render/render 18.44, render/graph/ms 15.43, stage/main/update 15.25, stage/main/post_update 13.19, encode//sum 11.69, stage/render/prepare_assets 11.56, stage/render/graph_submit 11.04 |
| warm-up | 5.44 | 38.3 | pipelines/compile_ms 508, pipelines/front_ms 492, render/thread/ms 54.62, render/busy/ms 52.65, main/busy/ms 39.80, stage/main/update 21.56, stage/render/prepare_assets 21.41, stage/render/render 16.37, pipelines/driver_ms 15.47, render/graph/ms 13.76, stream/drive/ms 12.10, stage/main/post_update 11.26 |
| warm-up | 5.48 | 37.9 | pipelines/compile_ms 818, pipelines/front_ms 775, render/thread/ms 63.94, render/busy/ms 62.51, main/busy/ms 49.12, pipelines/driver_ms 42.70, stage/main/update 32.10, stage/render/prepare_assets 31.96, stage/render/render 18.29, render/graph/ms 16.21, stage/render/graph_submit 12.15, stream/drive/ms 10.52 |
| warm-up | 5.50 | 26.2 | pipelines/compile_ms 548, pipelines/front_ms 512, render/thread/ms 58.10, render/busy/ms 56.73, main/busy/ms 46.77, pipelines/driver_ms 36.08, stage/main/update 27.53, stage/render/prepare_assets 25.21, stage/render/render 15.49, render/graph/ms 13.65, stage/main/post_update 12.13, stage/render/graph_submit 9.69 |
| warm-up | 5.53 | 26.8 | pipelines/compile_ms 54.83, pipelines/front_ms 51.45, render/thread/ms 47.37, render/busy/ms 45.83, main/busy/ms 33.94, stage/render/render 16.66, render/graph/ms 14.52, stage/main/post_update 14.39, stage/main/update 13.22, stage/render/prepare_assets 11.87, encode//sum 10.49, stage/render/graph_submit 10.27 |
| warm-up | 5.56 | 31.9 | pipelines/compile_ms 185, pipelines/front_ms 180, render/thread/ms 52.12, render/busy/ms 50.40, main/busy/ms 41.10, stage/render/render 18.63, stage/main/update 17.93, render/graph/ms 16.16, stage/main/post_update 15.02, stage/render/graph_submit 11.36, encode//sum 10.93, stream/drive/ms 9.33 |
| warm-up | 5.61 | 43.9 | pipelines/compile_ms 790, pipelines/front_ms 773, render/thread/ms 63.49, render/busy/ms 62.00, main/busy/ms 38.06, stage/render/prepare_assets 27.68, pipelines/driver_ms 17.26, stage/main/update 16.60, stage/render/render 14.68, stage/main/post_update 13.28, render/graph/ms 12.66, encode//sum 10.77 |
| warm-up | 5.68 | 30.7 | pipelines/compile_ms 195, pipelines/front_ms 191, render/thread/ms 44.50, render/busy/ms 43.47, main/busy/ms 33.49, stage/render/prepare_assets 21.03, stage/main/update 15.77, stage/render/render 12.26, stage/main/post_update 12.03, render/graph/ms 10.52, encode//sum 10.37, stream/drive/ms 10.04 |
| warm-up | 5.74 | 39.4 | render/thread/ms 54.07, render/busy/ms 52.57, main/busy/ms 26.46, stage/render/prepare_assets 24.06, stage/render/render 18.35, render/graph/ms 16.45, stage/render/graph_submit 12.33, stage/main/update 11.28, stage/main/post_update 10.45, encode//sum 10.14, pipelines/compile_ms 8.09, pipelines/front_ms 7.61 |
| warm-up | 5.77 | 29.3 | render/thread/ms 58.34, render/busy/ms 56.77, pipelines/compile_ms 46.25, pipelines/front_ms 42.79, stage/render/prepare_assets 30.62, main/busy/ms 24.00, stage/render/render 17.92, render/graph/ms 15.84, stage/render/graph_submit 11.91, stage/main/post_update 9.94, encode//sum 9.12, stage/main/update 8.37 |
| warm-up | 5.81 | 39.2 | pipelines/compile_ms 545, pipelines/front_ms 516, render/thread/ms 58.94, render/busy/ms 57.46, stage/render/prepare_assets 32.67, pipelines/driver_ms 29.62, main/busy/ms 25.04, stage/render/render 15.29, render/graph/ms 13.06, stage/main/post_update 10.46, encode//sum 9.89, stage/render/graph_submit 9.05 |
| warm-up | 5.86 | 31.3 | render/thread/ms 43.70, render/busy/ms 42.48, pipelines/compile_ms 27.12, pipelines/front_ms 25.74, main/busy/ms 25.31, stage/render/prepare_assets 21.51, stage/render/render 14.28, render/graph/ms 12.52, stage/main/post_update 12.05, encode//sum 10.29, stage/main/update 8.55, stage/render/graph_submit 8.30 |
| warm-up | 6.03 | 27.8 | pipelines/compile_ms 389, pipelines/front_ms 380, main/busy/ms 36.35, render/thread/ms 29.25, render/busy/ms 27.92, stage/main/update 21.80, stream/drive/ms 17.40, stage/render/render 13.62, render/graph/ms 11.72, encode//sum 11.09, stage/main/post_update 9.88, pipelines/driver_ms 8.93 |
| warm-up | 6.40 | 29.1 | render/thread/ms 35.90, render/busy/ms 34.95, main/busy/ms 24.31, stage/render/prepare_assets 14.80, stage/render/render 10.03, stage/main/update 9.19, encode//sum 8.76, stage/main/post_update 8.71, render/graph/ms 8.66, stage/render/graph_submit 5.73, encode/pass:shadow_point_light/sum 4.28, encode/submit 3.61 |
| warm-up | 6.62 | 27.3 | render/thread/ms 41.09, render/busy/ms 39.59, main/busy/ms 38.74, stage/render/render 22.00, render/graph/ms 19.93, stage/main/update 17.04, stage/main/post_update 16.02, stage/render/graph_submit 13.85, encode//sum 13.54, encode/submit 10.08, encode/pass:shadow_point_light/sum 6.39, stream/drive/ms 5.90 |
| warm-up | 6.65 | 28.1 | render/thread/ms 45.27, render/busy/ms 43.80, main/busy/ms 39.33, stage/main/update 18.55, stage/render/render 17.27, render/graph/ms 15.35, stage/main/post_update 14.64, stage/render/prepare_assets 14.32, encode//sum 11.26, stage/render/graph_submit 10.79, encode/submit 7.81, encode/pass:shadow_point_light/sum 5.38 |
| warm-up | 6.73 | 41.2 | main/busy/ms 44.85, render/thread/ms 34.55, render/busy/ms 33.23, stage/main/update 25.80, stream/drive/ms 14.89, stage/main/post_update 13.93, stage/render/render 13.58, render/graph/ms 11.78, encode//sum 11.47, stage/render/prepare_assets 8.15, stage/render/graph_submit 7.58, encode/pass:shadow_point_light/sum 6.42 |
| warm-up | 6.76 | 32.5 | pipelines/compile_ms 152, pipelines/front_ms 148, render/thread/ms 44.40, render/busy/ms 43.01, main/busy/ms 39.08, stage/main/update 18.78, stage/render/prepare_assets 16.27, stage/main/post_update 14.36, stage/render/render 13.62, render/graph/ms 11.77, encode//sum 10.80, stage/render/graph_submit 6.82 |
| warm-up | 6.97 | 30.8 | pipelines/compile_ms 142, pipelines/front_ms 139, render/thread/ms 48.70, render/busy/ms 47.52, main/busy/ms 29.89, stage/render/prepare_assets 26.41, stage/main/update 13.18, stage/render/render 12.89, stage/main/post_update 12.34, render/graph/ms 10.88, encode//sum 10.76, stream/drive/ms 7.23 |
| warm-up | 7.16 | 36.9 | pipelines/compile_ms 138, pipelines/front_ms 134, main/busy/ms 44.02, render/thread/ms 27.43, render/busy/ms 26.29, stage/main/update 24.13, stream/drive/ms 14.93, stage/main/post_update 14.01, stream/spawn/ms 13.65, stage/render/render 13.17, render/graph/ms 11.51, encode//sum 9.62 |
| measure | 0.77 | 26.5 | main/busy/ms 32.67, render/thread/ms 27.84, render/busy/ms 26.65, stage/main/update 16.91, stage/render/render 14.26, stage/main/post_update 12.61, render/graph/ms 12.61, stream/drive/ms 12.57, encode//sum 10.43, stage/render/graph_submit 8.74, encode/pass:shadow_point_light/sum 6.97, stage/render/prepare_assets 5.98 |
| measure | 18.18 | 33.0 | main/busy/ms 39.59, stage/main/update 23.82, render/thread/ms 23.61, render/busy/ms 22.55, stream/drive/ms 20.40, stage/render/render 15.12, render/graph/ms 13.47, stage/main/post_update 12.66, encode//sum 11.60, stage/render/graph_submit 9.87, encode/pass:shadow_point_light/sum 8.73, encode/submit 6.25 |
| measure | 59.04 | 32.9 | render/thread/ms 34.51, render/busy/ms 33.11, main/busy/ms 23.41, stage/render/render 14.16, stage/main/post_update 13.21, render/graph/ms 12.51, stage/render/prepare_assets 10.82, encode//sum 10.67, stage/render/graph_submit 9.95, encode/submit 6.36, stage/main/update 5.68, encode/finish_wall 3.70 |
| measure | 59.07 | 31.0 | render/thread/ms 35.68, render/busy/ms 34.54, stage/render/prepare_assets 19.28, main/busy/ms 18.24, stage/render/render 9.69, stage/main/post_update 9.53, encode//sum 9.06, render/graph/ms 8.34, stage/render/graph_submit 6.29, stage/main/update 4.88, encode/submit 3.67, encode/finish_wall 2.71 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.sun.gate=on --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=sungate-5 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 2%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **72%** (min 47%) · 2625 MHz (min 2610) · memory 8101 MHz · 76.0 W · 76 °C (max 81) · VRAM 6802 MiB · throttled 100% of samples

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
- `shadow.sun.gate=on (command line: --set)`
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
