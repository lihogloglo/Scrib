# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: texvariety · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5950 frames over 65.6 s · **median 10.78 ms (93 fps)** · mean 11.03 · p95 13.82 · p99 18.25 (1% low 55 fps) · worst 45.99 ms · 103 spikes over 1.5× median · 59.4 stutters/min (work 52.1/min)
- warm-up: 322 frames over 14.8 s, worst 2569.8 ms
- hitches (> 27.0 ms): **33** measured, 157 in warm-up
- timed GPU passes: 6.50 ms of a 11.03 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.78 ms, limit 11 ms
- **FAIL** frame_ms: 65 stutters (> 1.5x moving median of 60) in 1.09 min = 59.41/min, limit 1/min
- **FAIL** worst frame 45.99 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.876 | 2.022 | 2.050 | 0.93 | 0.182 |
| main_opaque_pass_3d | 1.358 | 1.463 | 1.911 | 0.93 | 0.079 |
| early prepass | 0.456 | 0.492 | 0.724 | 0.93 | 0.095 |
| bloom | 0.286 | 0.308 | 0.315 | 0.93 | 0.067 |
| atmosphere_luts | 0.283 | 0.304 | 0.367 | 0.93 | 0.004 |
| water_velocity | 0.249 | 0.268 | 0.347 | 0.93 | 0.002 |
| water_sspr | 0.195 | 0.210 | 0.210 | 0.93 | 0.003 |
| ssao | 0.175 | 0.188 | 0.191 | 0.93 | 0.008 |
| auxiliary_views | 0.141 | 0.152 | 0.507 | 0.93 | 0.167 |
| atmos_froxel_inject | 0.135 | 0.145 | 0.147 | 0.93 | 0.002 |
| clouds_trace | 0.127 | 0.137 | 0.233 | 0.93 | 0.001 |
| uniform_allocation | 0.122 | 0.131 | 0.183 | 0.93 | 0.029 |
| water_fft | 0.121 | 0.130 | 0.138 | 0.93 | 0.003 |
| upscaling | 0.104 | 0.113 | 0.121 | 0.93 | 0.004 |
| bin_unpacking | 0.099 | 0.107 | 0.143 | 0.93 | 0.021 |
| clouds_composite | 0.098 | 0.105 | 0.109 | 0.93 | 0.002 |
| tonemapping | 0.079 | 0.085 | 0.087 | 0.93 | 0.004 |
| main_transmissive_pass_3d | 0.075 | 0.081 | 0.160 | 0.93 | 0.009 |
| clustering | 0.060 | 0.064 | 0.068 | 0.93 | 0.019 |
| render_sky | 0.048 | 0.051 | 0.052 | 0.93 | 0.003 |
| clouds_resolve | 0.045 | 0.048 | 0.049 | 0.93 | 0.001 |
| lightprobe_irradiance_map | 0.042 | 0.413 | 0.416 | 0.10 | 0.001 |
| surface_exposure | 0.041 | 0.045 | 0.045 | 0.93 | 0.001 |
| surface_trail | 0.037 | 0.056 | 0.057 | 0.66 | 0.003 |
| surface_exposure_far | 0.036 | 0.044 | 0.046 | 0.81 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.93 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.031 | 0.047 | 0.93 | 0.006 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.031 | 0.93 | 0.001 |
| water_snapshot | 0.022 | 0.024 | 0.027 | 0.93 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.219 | 0.230 | 0.10 | 0.005 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.93 | 0.002 |
| night_sky | 0.012 | 0.013 | 0.034 | 0.93 | 0.002 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.011 | 0.026 | 0.93 | 0.126 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.93 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.93 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.92 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/compile_ms | 16.186 | 0.000 | 9240.80 | 96303.8 |
| pipelines/driver_ms | 14.282 | 0.000 | 8837.40 | 84978.1 |
| pipelines/wait_ms | 10.935 | 0.000 | 14087.37 | 65065.2 |
| render/thread/ms | 9.538 | 12.125 | 41.84 | 56749.7 |
| main/busy/ms | 9.218 | 11.242 | 34.28 | 54846.3 |
| render/busy/ms | 9.050 | 11.581 | 41.20 | 53847.2 |
| encode//sum | 6.226 | 8.461 | 36.38 | 37041.8 |
| stage/main/post_update | 5.654 | 7.125 | 29.15 | 33639.6 |
| stage/render/render | 5.274 | 6.821 | 18.60 | 31377.6 |
| render/graph/ms | 4.587 | 6.075 | 17.05 | 27294.3 |
| stage/render/graph_submit | 3.312 | 4.443 | 11.42 | 19708.6 |
| stage/main/update | 2.234 | 2.793 | 22.93 | 13289.5 |
| encode/submit | 2.102 | 2.918 | 6.01 | 12507.8 |
| pipelines/front_ms | 1.903 | 0.000 | 424.30 | 11325.7 |
| encode/finish_wall | 1.253 | 1.868 | 5.56 | 7456.3 |
| stage/render/camera_driver | 1.220 | 1.711 | 5.53 | 7256.1 |
| stage/render/prepare_write_phase_buffers | 0.815 | 1.188 | 3.11 | 4849.0 |
| stage/render/prepare_assets | 0.605 | 0.798 | 10.13 | 3598.9 |
| bigspace/total/ms | 0.499 | 1.020 | 2.85 | 2966.9 |
| stage/render/prepare_meshes | 0.477 | 0.601 | 7.13 | 2835.3 |
| stage/render/prepare_resources | 0.462 | 0.574 | 1.17 | 2750.5 |
| stage/render/specialize | 0.441 | 0.621 | 15.07 | 2621.2 |
| stage/main/first | 0.418 | 0.560 | 6.34 | 2486.9 |
| render/present/ms | 0.390 | 0.586 | 4.46 | 2317.7 |
| stage/main/pre_update | 0.379 | 0.568 | 3.84 | 2252.1 |
| stage/render/queue | 0.309 | 0.446 | 9.56 | 1837.7 |
| stage/render/prepare_bind_groups | 0.309 | 0.569 | 3.39 | 1836.5 |
| terrain/update/ms | 0.308 | 0.403 | 1.89 | 1829.9 |
| controller/feed/ms | 0.292 | 0.380 | 0.81 | 1735.0 |
| terrain/select/ms | 0.273 | 0.357 | 0.98 | 1625.6 |
| stream/drive/ms | 0.256 | 0.371 | 20.64 | 1524.6 |
| stage/main/fixed_main_loop | 0.228 | 0.402 | 1.78 | 1358.9 |
| far/select/ms | 0.211 | 0.309 | 0.90 | 1256.9 |
| stage/render/create_views | 0.211 | 0.283 | 1.09 | 1256.5 |
| stage/render/prepare_batch_phases | 0.205 | 0.298 | 0.61 | 1222.1 |
| bigspace/low_precision/ms | 0.202 | 0.537 | 2.42 | 1204.3 |
| stream/tick/ms | 0.197 | 0.285 | 0.59 | 1174.7 |
| stage/render/extract_commands | 0.185 | 0.227 | 0.54 | 1100.1 |
| stage/render/prepare_flush | 0.169 | 0.325 | 0.90 | 1006.3 |
| anim/evaluate/ms | 0.159 | 0.273 | 1.38 | 945.6 |
| anim/evaluate_cpu/ms | 0.155 | 0.200 | 0.66 | 924.9 |
| stage/render/prepare_views | 0.144 | 0.185 | 0.54 | 854.3 |
| controller/move/ms | 0.135 | 0.248 | 0.47 | 801.5 |
| bigspace/high_precision/ms | 0.134 | 0.479 | 1.73 | 796.2 |
| stage/main/last | 0.123 | 0.190 | 1.75 | 734.4 |
| render/acquire/ms | 0.098 | 0.129 | 0.47 | 584.8 |
| stage/render/prepare_collect_phase_buffers | 0.070 | 0.148 | 0.43 | 414.2 |
| physics/step/ms | 0.050 | 0.094 | 0.46 | 295.5 |
| encode/resolve_diagnostics/sum | 0.035 | 0.049 | 0.56 | 211.1 |
| physics/insert/ms | 0.034 | 0.370 | 0.76 | 201.1 |
| stage/render/cleanup | 0.032 | 0.091 | 1.04 | 192.1 |
| stage/render/phase_sort | 0.028 | 0.048 | 0.20 | 166.2 |
| weather/map/ms | 0.027 | 0.036 | 0.22 | 160.9 |
| encode/camera_driver_span_begin/sum | 0.017 | 0.025 | 0.72 | 100.1 |
| anim/apply/ms | 0.012 | 0.024 | 0.35 | 74.1 |
| encode/camera_driver_span_end/sum | 0.012 | 0.021 | 0.65 | 73.1 |
| controller/avoid/ms | 0.011 | 0.015 | 0.05 | 66.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.13 | 65.6 |
| script/update/ms | 0.008 | 0.010 | 0.14 | 45.1 |
| stream/despawn/ms | 0.005 | 0.000 | 1.06 | 31.4 |
| stream/spawn/ms | 0.005 | 0.000 | 1.23 | 30.6 |
| script/input/ms | 0.005 | 0.007 | 0.10 | 29.8 |
| script/ui/ms | 0.005 | 0.007 | 0.08 | 29.4 |
| controller/write/ms | 0.004 | 0.005 | 0.26 | 21.1 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.74 | 14.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.13 | 6.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.7 |
| stage/render/prepare | 0.001 | 0.003 | 0.51 | 3.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.10 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 33 | 19 |
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1292 | 2262 | 535 |
| crowd/c0/meshes/count | count | 353 | 646 | 144 |
| crowd/c1/count | count | 146 | 155 | 149 |
| crowd/c1/joints/count | count | 5531 | 5902 | 5661 |
| crowd/c1/meshes/count | count | 1152 | 1256 | 1197 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6402 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4621 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4597 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.86 | 2.23 | 0.92 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.72 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.65 | 0.01 |
| encode/finish_critical | ms | 0.86 | 2.23 | 0.92 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.56 | 0.05 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.47 | 41.20 | 8.10 |
| gpu/timed/ms | ms | 7.69 | 11.24 | 8.10 |
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
| phases/alpha_mask3d/items | count | 798 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 798 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1296 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3356 | 5117 | 3189 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1296 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3356 | 5117 | 3189 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 18 | 203 | 29 |
| phases/shadow/bins | count | 5008 | 8821 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.3 k | 26.4 k | 6376 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 203 | 255 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 193 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 266 | 330 | 330 |
| pipelines/waiting | count | 1 | 11 | 0 |
| precache/draw_waiting | count | 0 | 8 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 1 | 3 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 27 | 74 | 0 |
| preprocess/bind_groups | count | 13 | 567 | 58 |
| preprocess/bind_groups_created | count | 13 | 567 | 58 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 211 | 123 |
| process/cpu_usage | % | 31.80 | 87.50 | 25.96 |
| process/mem_usage | GiB | 4.20 | 4.36 | 4.36 |
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
| skin/blocks_changed/count | count | 14 | 122 | 12 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 366.3 k | 809.2 k | 497.2 k |
| skin/joints_written/count | count | 5724 | 12.6 k | 7768 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4990 | 5081 | 5081 |
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
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 38.52 | 90.85 | 31.61 |
| system/mem_usage | % | 60.28 | 61.76 | 60.18 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 1447 | 1452 | 1447 |
| textures/images | count | 820 | 830 | 820 |
| views/all/batch_sets | count | 68 | 252 | 79 |
| views/all/bins | count | 7527 | 11.7 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.4 k | 14.0 k |
| views/all/pipelines | count | 249 | 302 | 75 |
| views/all/pipelines_distinct | count | 70 | 75 | 61 |
| views/all/work_items | count | 10.6 k | 35.9 k | 14.3 k |
| views/cascade/batch_sets | count | 9 | 53 | 29 |
| views/cascade/bins | count | 2795 | 6162 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8876 | 23.5 k | 6376 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 413 | 23.5 k | 6376 |
| views/lamp_face/batch_sets | count | 0 | 197 | 0 |
| views/lamp_face/bins | count | 2171 | 2820 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2410 | 3114 | 0 |
| views/lamp_face/pipelines | count | 168 | 204 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2943 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2890 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8308 | 13.0 k | 7612 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8477 | 13.2 k | 7920 |
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
| anim/bones_written/count | count | 53.89 | 254 | 320.6 k |
| anim/events/count | count | 0.09 | 3 | 517 |
| assets/Image/added | count | 0.10 | 39 | 611 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.14 | 86 | 850 |
| assets/Mesh/added | count | 0.68 | 152 | 4061 |
| assets/Mesh/removed | count | 0.81 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.64 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 54.29 | 3403 | 323.0 k |
| ecs/entities_delta | count | 0.79 | 1303 | 4674 |
| encode//count | count | 40.05 | 273 | 238.3 k |
| encode/bind_walk/init_textures | count | 24.9 k | 59.6 k | 147.89 M |
| encode/bind_walk/init_textures_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/inits | count | 575 | 1256 | 3.42 M |
| encode/bind_walk/inits_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/merge_textures | count | 5657 | 34.0 k | 33.66 M |
| encode/bind_walk/merge_textures_skipped | count | 0 | 0 | 0 |
| encode/bind_walk/merges | count | 186 | 418 | 1.11 M |
| encode/bind_walk/merges_skipped | count | 0 | 0 | 0 |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5950 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5950 |
| encode/queue/command_buffers | count | 45.06 | 278 | 268.1 k |
| encode/queue/submits | count | 2.02 | 4 | 12.0 k |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 4.88 M | 11.24 G |
| encode/queue/write_buffer/calls | count | 260 | 846 | 1.55 M |
| encode/queue/write_buffer_with/bytes | bytes | 178.2 k | 3.52 M | 1.06 G |
| encode/queue/write_buffer_with/calls | count | 11.03 | 465 | 65.6 k |
| encode/queue/write_texture/bytes | bytes | 10.5 k | 8.39 M | 62.48 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 562 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5950 |
| physics/inserts | count | 0.39 | 95 | 2294 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 7 |
| pipelines/created | count | 0.02 | 2 | 115 |
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
| stream/spawn/items | count | 0.06 | 24 | 378 |
| stream/upload/bytes | bytes | 5689 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 28 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1247 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.0 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2569.8 | main/busy/ms 2538, stage/main/post_update 2432, stage/main/update 98.61, surface/advance_cpu_ms 69.24, stage/main/pre_update 3.99, stage/main/last 1.20, surface/cell_upload_ms 0.75, stage/main/first 0.54, stage/main/fixed_main_loop 0.43, stream/drive/ms 0.38, stream/tick/ms 0.13, weather/map/ms 0.13 |
| warm-up | 3.40 | 827.6 | main/busy/ms 2696, stage/main/post_update 2457, render/thread/ms 795, render/busy/ms 653, stage/render/prepare_assets 605, stage/main/update 225, stage/render/prepare_views 139, render/acquire/ms 138, surface/advance_cpu_ms 69.24, stage/render/create_views 26.98, stage/render/render 13.37, render/graph/ms 9.07 |
| warm-up | 3.53 | 128.7 | render/thread/ms 904, render/busy/ms 705, stage/render/prepare_assets 643, pipelines/compile_ms 360, pipelines/driver_ms 277, main/busy/ms 198, stage/render/prepare_views 139, render/acquire/ms 139, stage/main/update 133, pipelines/front_ms 82.85, stage/render/render 75.85, render/present/ms 61.21 |
| warm-up | 3.62 | 62.7 | pipelines/compile_ms 493, pipelines/driver_ms 400, pipelines/front_ms 92.47, render/thread/ms 77.24, render/busy/ms 68.27, main/busy/ms 43.07, stage/main/post_update 27.90, stage/render/prepare_resources 23.90, stage/render/render 20.80, pipelines/wait_ms 13.35, terrain/update/ms 12.74, stage/render/prepare_assets 11.86 |
| warm-up | 3.76 | 147.4 | pipelines/compile_ms 1267, pipelines/driver_ms 1032, pipelines/wait_ms 819, pipelines/front_ms 235, render/thread/ms 198, render/busy/ms 196, main/busy/ms 164, stage/render/prepare_resources 78.53, stage/main/update 62.20, stage/main/post_update 60.90, stage/render/prepare_flush 30.95, stage/main/pre_update 21.96 |
| warm-up | 3.86 | 92.6 | pipelines/compile_ms 1824, pipelines/wait_ms 1602, pipelines/driver_ms 1507, pipelines/front_ms 317, render/thread/ms 235, render/busy/ms 233, main/busy/ms 190, stage/main/update 65.68, stage/render/prepare_assets 64.27, stage/render/prepare_resources 62.36, stage/main/post_update 59.01, stage/main/fixed_main_loop 41.85 |
| warm-up | 3.92 | 60.4 | pipelines/wait_ms 992, pipelines/compile_ms 831, pipelines/driver_ms 695, render/thread/ms 146, render/busy/ms 144, pipelines/front_ms 136, stage/render/prepare_assets 88.56, main/busy/ms 66.71, stage/main/post_update 27.27, stage/main/fixed_main_loop 26.47, stage/render/prepare_resources 16.80, stage/render/prepare_flush 13.84 |
| warm-up | 3.97 | 54.8 | pipelines/wait_ms 1454, pipelines/compile_ms 929, pipelines/driver_ms 807, pipelines/front_ms 121, render/thread/ms 107, render/busy/ms 105, main/busy/ms 32.85, stage/render/prepare_flush 28.89, stage/render/prepare_assets 26.57, stage/main/post_update 21.29, stage/render/render 19.24, render/graph/ms 16.99 |
| warm-up | 4.03 | 55.8 | pipelines/wait_ms 2431, pipelines/compile_ms 866, pipelines/driver_ms 756, pipelines/front_ms 110, render/thread/ms 87.69, render/busy/ms 85.88, main/busy/ms 32.58, stage/render/prepare_resources 25.81, stage/render/prepare_flush 20.82, stage/main/post_update 20.81, stage/render/render 20.56, render/graph/ms 18.22 |
| warm-up | 4.10 | 71.5 | pipelines/wait_ms 2472, pipelines/compile_ms 305, pipelines/driver_ms 227, render/thread/ms 107, render/busy/ms 106, pipelines/front_ms 78.16, main/busy/ms 62.47, stage/render/prepare_resources 38.70, stage/main/post_update 36.64, stage/render/prepare_assets 26.03, stage/main/update 20.36, stage/render/prepare_flush 12.92 |
| warm-up | 4.18 | 80.4 | pipelines/wait_ms 2183, pipelines/compile_ms 442, pipelines/driver_ms 337, render/thread/ms 141, render/busy/ms 139, main/busy/ms 121, pipelines/front_ms 105, stage/main/update 72.00, stream/drive/ms 52.49, stage/render/prepare_resources 49.28, stage/main/post_update 43.24, stage/render/prepare_assets 34.05 |
| warm-up | 4.31 | 126.8 | pipelines/wait_ms 2447, pipelines/compile_ms 471, pipelines/driver_ms 372, render/thread/ms 176, render/busy/ms 174, main/busy/ms 122, pipelines/front_ms 99.13, stage/render/prepare_assets 63.46, stage/main/update 58.47, stage/main/post_update 52.17, stream/drive/ms 52.04, stage/render/prepare_resources 51.71 |
| warm-up | 4.36 | 57.3 | pipelines/wait_ms 2320, pipelines/compile_ms 236, pipelines/driver_ms 181, render/thread/ms 161, render/busy/ms 146, main/busy/ms 76.85, stage/render/prepare_assets 58.92, pipelines/front_ms 54.83, stage/main/post_update 44.94, bigspace/total/ms 38.95, bigspace/low_precision/ms 29.52, stage/render/prepare_resources 25.06 |
| warm-up | 4.46 | 101.4 | pipelines/wait_ms 1159, render/thread/ms 141, render/busy/ms 126, pipelines/compile_ms 99.01, main/busy/ms 79.41, pipelines/driver_ms 66.52, stage/render/prepare_assets 61.63, stage/main/post_update 40.80, pipelines/front_ms 32.49, stage/render/prepare_flush 20.86, stage/main/update 17.90, stage/render/render 16.86 |
| warm-up | 4.60 | 85.6 | render/thread/ms 117, render/busy/ms 116, main/busy/ms 49.09, stage/render/prepare_resources 35.06, stage/main/post_update 30.87, stage/render/prepare_assets 27.94, stage/render/prepare_write_phase_buffers 25.13, stage/render/prepare_flush 9.12, stage/main/update 9.08, stage/render/prepare_bind_groups 8.50, stage/render/render 7.96, render/graph/ms 5.67 |
| warm-up | 4.66 | 64.1 | pipelines/compile_ms 876, pipelines/driver_ms 795, pipelines/wait_ms 212, render/thread/ms 133, render/busy/ms 131, pipelines/front_ms 80.60, main/busy/ms 63.62, stage/render/prepare_resources 41.95, stage/main/update 35.79, stage/render/prepare_write_phase_buffers 29.02, stage/render/prepare_bind_groups 21.83, stage/render/prepare_assets 18.74 |
| warm-up | 4.75 | 90.0 | pipelines/wait_ms 3454, pipelines/compile_ms 2263, pipelines/driver_ms 2062, pipelines/front_ms 201, render/thread/ms 133, render/busy/ms 131, main/busy/ms 72.67, stage/render/prepare_bind_groups 39.35, stage/main/update 36.25, stage/render/prepare_resources 34.05, stage/render/prepare_assets 31.53, stage/main/post_update 28.21 |
| warm-up | 4.85 | 100.5 | pipelines/wait_ms 5721, pipelines/compile_ms 1481, pipelines/driver_ms 1350, render/thread/ms 170, render/busy/ms 167, pipelines/front_ms 131, main/busy/ms 99.83, stage/main/update 71.81, stage/render/prepare_write_phase_buffers 53.18, stage/render/prepare_assets 31.77, stage/render/prepare_resources 26.98, stage/render/prepare_bind_groups 20.91 |
| warm-up | 4.93 | 79.0 | pipelines/wait_ms 2479, render/thread/ms 170, render/busy/ms 169, main/busy/ms 109, pipelines/compile_ms 92.95, pipelines/driver_ms 82.88, stage/main/update 71.68, stage/render/prepare_write_phase_buffers 60.38, stage/render/prepare_resources 37.33, stage/render/prepare_assets 29.69, stage/main/post_update 28.47, stage/render/prepare_flush 14.83 |
| warm-up | 5.00 | 68.5 | pipelines/wait_ms 1308, render/thread/ms 136, render/busy/ms 134, pipelines/compile_ms 113, main/busy/ms 75.70, pipelines/driver_ms 71.44, pipelines/front_ms 41.91, stage/render/prepare_assets 40.50, stage/main/post_update 39.79, stage/render/prepare_resources 29.26, stage/main/update 27.21, stage/render/render 22.66 |
| warm-up | 5.09 | 86.7 | pipelines/wait_ms 4483, pipelines/compile_ms 1403, pipelines/driver_ms 1275, render/thread/ms 140, render/busy/ms 138, pipelines/front_ms 128, main/busy/ms 118, stage/main/update 60.26, stage/render/prepare_resources 53.93, stage/main/post_update 45.90, stream/drive/ms 33.93, stage/render/render 26.82 |
| warm-up | 5.21 | 124.7 | pipelines/wait_ms 4694, pipelines/compile_ms 1352, pipelines/driver_ms 1250, render/thread/ms 190, render/busy/ms 188, main/busy/ms 181, pipelines/front_ms 102, stage/main/update 91.91, stage/render/prepare_resources 88.54, stage/main/post_update 77.26, stage/render/prepare_batch_phases 39.34, bigspace/total/ms 35.75 |
| warm-up | 5.43 | 215.3 | pipelines/wait_ms 4804, render/thread/ms 289, render/busy/ms 286, pipelines/compile_ms 200, main/busy/ms 191, pipelines/driver_ms 151, stage/render/prepare_assets 142, stage/main/update 96.39, stage/main/post_update 54.77, stream/drive/ms 53.35, stage/render/prepare_resources 50.10, pipelines/front_ms 49.56 |
| warm-up | 5.51 | 83.9 | pipelines/wait_ms 6896, pipelines/compile_ms 331, pipelines/driver_ms 256, render/thread/ms 253, render/busy/ms 250, stage/render/prepare_assets 163, main/busy/ms 118, pipelines/front_ms 74.31, stage/main/update 56.00, stream/drive/ms 45.40, stage/render/render 35.42, render/graph/ms 31.38 |
| warm-up | 5.60 | 93.5 | pipelines/wait_ms 5523, pipelines/compile_ms 346, pipelines/driver_ms 264, render/thread/ms 169, render/busy/ms 168, main/busy/ms 117, pipelines/front_ms 81.50, stage/render/prepare_resources 75.55, stage/main/update 63.21, stream/drive/ms 50.28, stage/render/prepare_assets 37.70, stage/main/post_update 37.28 |
| warm-up | 5.75 | 148.8 | pipelines/wait_ms 1912, render/thread/ms 230, render/busy/ms 228, main/busy/ms 219, pipelines/compile_ms 152, stage/main/update 116, pipelines/driver_ms 112, stage/render/prepare_resources 105, stream/drive/ms 101, stage/main/post_update 86.92, stage/render/prepare_flush 41.45, pipelines/front_ms 40.41 |
| warm-up | 5.87 | 114.5 | render/thread/ms 228, render/busy/ms 226, main/busy/ms 180, stage/main/post_update 84.24, stage/main/update 74.48, stage/render/prepare_assets 64.21, stream/drive/ms 63.70, stage/render/prepare_flush 61.65, stage/render/prepare_resources 56.19, stage/render/render 18.82, anim/evaluate/ms 15.67, stage/render/prepare_write_phase_buffers 13.68 |
| warm-up | 6.00 | 133.6 | render/thread/ms 216, render/busy/ms 208, main/busy/ms 177, stage/main/update 76.31, stage/render/prepare_assets 54.65, stage/render/prepare_write_phase_buffers 53.81, stream/drive/ms 46.25, stage/main/post_update 43.85, stage/render/prepare_flush 28.26, stage/main/fixed_main_loop 24.78, stage/main/pre_update 18.36, stage/render/render 17.86 |
| warm-up | 6.08 | 80.3 | render/thread/ms 192, render/busy/ms 184, main/busy/ms 180, stage/main/update 83.73, stage/render/prepare_write_phase_buffers 49.08, stream/drive/ms 45.65, stage/main/post_update 39.97, stage/render/prepare_assets 28.92, stage/render/prepare_resources 27.83, stage/render/render 25.57, stage/main/fixed_main_loop 24.72, stage/render/prepare_bind_groups 23.54 |
| warm-up | 6.17 | 87.3 | render/thread/ms 141, render/busy/ms 140, main/busy/ms 89.11, stage/render/prepare_assets 43.11, stage/main/update 41.04, stage/render/prepare_resources 39.44, stage/main/post_update 25.94, stage/render/render 22.25, render/graph/ms 19.90, stage/main/pre_update 12.68, stage/render/prepare_bind_groups 12.49, stage/render/camera_driver 10.79 |
| warm-up | 6.22 | 51.4 | render/thread/ms 118, render/busy/ms 117, main/busy/ms 63.50, stage/render/prepare_assets 45.59, stage/main/update 34.26, stage/render/prepare_resources 20.42, stage/render/prepare_write_phase_buffers 15.26, stage/main/post_update 14.93, stage/render/render 13.96, render/graph/ms 11.69, encode//sum 9.54, stage/render/graph_submit 9.25 |
| warm-up | 6.32 | 53.8 | render/thread/ms 89.87, render/busy/ms 84.40, main/busy/ms 52.00, stage/render/prepare_resources 23.56, stage/main/update 23.18, stage/render/prepare_assets 18.26, stage/main/post_update 17.05, stage/render/render 13.87, render/graph/ms 11.38, stage/render/prepare_flush 9.24, stage/render/graph_submit 8.73, encode//sum 8.72 |
| warm-up | 6.42 | 58.5 | pipelines/compile_ms 2207, pipelines/driver_ms 2021, pipelines/wait_ms 331, pipelines/front_ms 186, render/thread/ms 92.75, render/busy/ms 90.96, main/busy/ms 49.71, stage/render/prepare_resources 23.01, stage/main/post_update 20.67, stage/main/update 20.46, stage/render/prepare_flush 18.66, stage/render/prepare_write_phase_buffers 12.56 |
| warm-up | 6.50 | 87.4 | pipelines/compile_ms 4587, pipelines/driver_ms 4218, pipelines/wait_ms 530, pipelines/front_ms 369, render/thread/ms 137, render/busy/ms 135, main/busy/ms 81.11, stage/main/post_update 38.28, stage/render/prepare_flush 26.90, stage/render/prepare_resources 26.48, stage/render/prepare_write_phase_buffers 19.00, stage/main/fixed_main_loop 18.72 |
| warm-up | 6.56 | 59.7 | pipelines/wait_ms 2778, pipelines/compile_ms 2419, pipelines/driver_ms 2211, pipelines/front_ms 207, render/thread/ms 136, render/busy/ms 134, main/busy/ms 106, stage/main/post_update 59.98, stage/render/prepare_batch_phases 33.01, bigspace/total/ms 30.81, bigspace/low_precision/ms 27.38, stage/render/prepare_resources 25.05 |
| warm-up | 6.83 | 55.8 | pipelines/wait_ms 4772, pipelines/compile_ms 4709, pipelines/driver_ms 4400, pipelines/front_ms 309, render/thread/ms 88.98, render/busy/ms 87.41, main/busy/ms 72.10, stage/render/prepare_assets 37.50, stage/main/update 27.41, stage/main/post_update 22.31, stage/main/pre_update 16.57, stage/render/prepare_write_phase_buffers 13.97 |
| warm-up | 7.00 | 57.8 | pipelines/wait_ms 8063, pipelines/compile_ms 3698, pipelines/driver_ms 3320, pipelines/front_ms 378, render/thread/ms 86.93, render/busy/ms 85.59, main/busy/ms 83.35, stage/main/update 40.70, stage/render/prepare_resources 30.80, stream/drive/ms 30.67, stage/main/post_update 20.87, stage/render/prepare_assets 17.46 |
| warm-up | 7.06 | 62.3 | pipelines/wait_ms 6681, pipelines/compile_ms 3709, pipelines/driver_ms 3336, pipelines/front_ms 373, render/thread/ms 113, render/busy/ms 112, main/busy/ms 101, stage/main/update 43.87, stage/render/prepare_resources 43.44, stage/render/prepare_assets 37.39, stage/main/post_update 37.25, stream/drive/ms 32.24 |
| warm-up | 7.13 | 66.4 | pipelines/compile_ms 3814, pipelines/driver_ms 3471, pipelines/wait_ms 1531, pipelines/front_ms 343, render/thread/ms 119, render/busy/ms 118, main/busy/ms 95.02, stage/main/post_update 62.33, stage/render/prepare_assets 38.26, stage/render/prepare_resources 32.76, bigspace/total/ms 17.19, stage/main/update 15.49 |
| warm-up | 7.19 | 63.2 | pipelines/compile_ms 3758, pipelines/driver_ms 3432, pipelines/wait_ms 793, pipelines/front_ms 326, render/thread/ms 120, render/busy/ms 118, main/busy/ms 80.11, stage/main/post_update 57.62, stage/render/prepare_assets 38.99, stage/render/prepare_resources 29.75, bigspace/total/ms 14.80, stage/main/update 13.57 |
| warm-up | 7.25 | 56.3 | render/thread/ms 110, render/busy/ms 109, main/busy/ms 70.85, stage/render/prepare_assets 59.76, stage/main/post_update 51.93, bigspace/total/ms 21.15, stage/render/prepare_resources 19.92, bigspace/high_precision/ms 16.14, stage/main/update 12.03, stage/render/render 9.14, render/graph/ms 6.86, stage/render/prepare_write_phase_buffers 5.26 |
| warm-up | 7.43 | 57.5 | pipelines/wait_ms 12.2 k, pipelines/compile_ms 6306, pipelines/driver_ms 5858, pipelines/front_ms 448, render/thread/ms 90.41, render/busy/ms 88.76, main/busy/ms 65.08, stage/main/post_update 41.61, stage/render/prepare_assets 41.58, stage/render/prepare_write_phase_buffers 18.14, stage/main/update 15.35, stage/render/prepare_resources 13.59 |
| warm-up | 7.54 | 58.4 | pipelines/compile_ms 3597, pipelines/driver_ms 3405, pipelines/wait_ms 402, pipelines/front_ms 192, render/thread/ms 99.17, render/busy/ms 97.58, main/busy/ms 60.51, stage/render/prepare_assets 40.97, stage/main/post_update 24.67, stage/main/update 21.60, stage/render/prepare_resources 17.70, stage/render/render 14.57 |
| warm-up | 7.61 | 73.9 | pipelines/wait_ms 12.4 k, pipelines/compile_ms 3666, pipelines/driver_ms 3454, pipelines/front_ms 212, render/thread/ms 124, render/busy/ms 123, main/busy/ms 53.27, stage/render/prepare_assets 43.80, stage/render/prepare_resources 32.61, stage/main/post_update 26.33, stage/main/update 18.31, stage/render/render 17.73 |
| warm-up | 7.96 | 80.1 | pipelines/wait_ms 7431, pipelines/compile_ms 4433, pipelines/driver_ms 4084, pipelines/front_ms 349, render/thread/ms 107, render/busy/ms 105, main/busy/ms 90.58, stage/main/post_update 77.81, stage/render/prepare_resources 29.51, stage/render/prepare_batch_phases 28.33, bigspace/total/ms 17.56, stage/render/prepare_assets 14.24 |
| warm-up | 8.14 | 49.4 | pipelines/wait_ms 4475, pipelines/compile_ms 119, pipelines/driver_ms 104, render/thread/ms 84.40, render/busy/ms 82.92, main/busy/ms 76.50, stage/main/post_update 46.03, stage/render/prepare_resources 26.62, stage/render/prepare_write_phase_buffers 20.44, stage/main/update 16.63, pipelines/front_ms 14.71, stage/render/prepare_assets 11.03 |
| warm-up | 8.27 | 60.5 | pipelines/wait_ms 4593, pipelines/compile_ms 122, pipelines/driver_ms 103, render/thread/ms 92.44, render/busy/ms 90.92, main/busy/ms 81.52, stage/main/update 43.33, stage/render/prepare_resources 24.77, stream/drive/ms 23.48, pipelines/front_ms 19.40, stage/render/prepare_bind_groups 14.80, stage/main/pre_update 13.88 |
| warm-up | 8.40 | 70.7 | pipelines/wait_ms 14.3 k, pipelines/compile_ms 123, pipelines/driver_ms 107, render/thread/ms 88.69, stage/render/prepare_views 55.29, render/acquire/ms 55.10, main/busy/ms 32.74, render/busy/ms 32.32, pipelines/front_ms 15.54, stage/main/post_update 14.90, stage/main/update 11.36, stage/render/render 10.47 |
| warm-up | 8.50 | 66.8 | pipelines/wait_ms 19.5 k, pipelines/compile_ms 106, render/thread/ms 95.63, pipelines/driver_ms 72.94, render/busy/ms 61.10, main/busy/ms 56.57, pipelines/front_ms 32.85, stage/render/prepare_views 32.02, render/acquire/ms 31.87, stage/main/post_update 29.04, stage/main/update 14.70, stage/render/prepare_assets 14.03 |
| warm-up | 8.64 | 99.5 | render/thread/ms 137, render/busy/ms 136, main/busy/ms 109, stage/main/post_update 84.25, stage/render/prepare_batch_phases 35.54, stage/render/queue 24.67, anim/evaluate/ms 21.23, stage/render/prepare_resources 19.64, stage/render/prepare_write_phase_buffers 16.27, stage/main/update 15.55, stage/render/render 9.87, encode//sum 7.80 |
| warm-up | 8.71 | 63.2 | pipelines/compile_ms 1779, pipelines/driver_ms 1643, pipelines/wait_ms 1363, render/thread/ms 145, render/busy/ms 143, pipelines/front_ms 136, main/busy/ms 128, stage/main/post_update 96.97, stage/render/prepare_batch_phases 36.56, stage/render/queue 24.88, stage/render/prepare_assets 24.52, anim/evaluate/ms 21.28 |
| warm-up | 8.97 | 53.5 | pipelines/wait_ms 13.9 k, pipelines/compile_ms 287, pipelines/driver_ms 224, render/thread/ms 91.26, render/busy/ms 89.78, main/busy/ms 64.99, pipelines/front_ms 62.87, stage/render/prepare_resources 36.14, stage/main/update 26.00, stage/main/post_update 23.84, stage/render/prepare_write_phase_buffers 15.40, stream/drive/ms 12.67 |
| warm-up | 9.28 | 57.8 | pipelines/compile_ms 1842, pipelines/driver_ms 1701, pipelines/wait_ms 282, pipelines/front_ms 141, main/busy/ms 62.95, render/thread/ms 57.30, render/busy/ms 55.36, stage/main/update 30.45, stage/main/post_update 20.38, stream/drive/ms 18.70, stage/render/render 12.69, stage/render/prepare_resources 11.15 |
| warm-up | 9.34 | 54.7 | pipelines/compile_ms 1842, pipelines/driver_ms 1701, pipelines/wait_ms 282, pipelines/front_ms 141, render/thread/ms 87.60, render/busy/ms 86.25, main/busy/ms 55.65, stage/render/prepare_resources 38.21, stage/main/update 20.10, stage/main/post_update 17.85, stage/render/prepare_assets 13.70, encode//sum 10.91 |
| warm-up | 9.49 | 52.6 | render/thread/ms 71.68, render/busy/ms 70.06, main/busy/ms 48.38, stage/render/prepare_assets 29.30, stage/main/post_update 28.09, stage/main/update 14.42, encode//sum 12.82, stage/render/render 11.67, bigspace/total/ms 9.47, stage/render/prepare_resources 9.20, render/graph/ms 8.95, bigspace/high_precision/ms 8.63 |
| warm-up | 9.93 | 51.3 | pipelines/wait_ms 1192, pipelines/compile_ms 177, pipelines/driver_ms 133, render/thread/ms 80.34, render/busy/ms 78.51, main/busy/ms 60.97, pipelines/front_ms 43.57, stage/render/prepare_assets 31.63, stage/main/post_update 28.30, stage/main/update 18.68, stage/render/prepare_meshes 12.73, stage/render/render 12.40 |
| warm-up | 10.03 | 63.0 | pipelines/wait_ms 10.5 k, pipelines/compile_ms 2393, pipelines/driver_ms 2289, pipelines/front_ms 104, render/thread/ms 94.75, render/busy/ms 93.14, main/busy/ms 44.04, stage/render/prepare_assets 29.59, stage/main/post_update 18.12, stage/main/update 16.68, stage/render/specialize 16.01, stage/render/render 14.59 |
| warm-up | 10.31 | 53.6 | render/thread/ms 79.59, render/busy/ms 77.94, main/busy/ms 71.39, stage/main/post_update 32.93, stage/render/prepare_resources 30.94, stage/main/update 22.43, anim/evaluate/ms 14.44, stage/render/render 14.31, stage/render/prepare_assets 14.07, render/graph/ms 12.05, encode//sum 11.92, stage/render/graph_submit 8.44 |
| warm-up | 10.83 | 74.4 | pipelines/wait_ms 19.4 k, pipelines/compile_ms 10.3 k, pipelines/driver_ms 9718, pipelines/front_ms 580, render/thread/ms 90.61, render/busy/ms 89.26, main/busy/ms 77.16, stage/main/post_update 63.78, stage/render/render 23.49, encode//sum 22.56, render/graph/ms 21.10, stage/render/graph_submit 17.58 |
| warm-up | 10.89 | 60.4 | pipelines/wait_ms 31.5 k, pipelines/compile_ms 10.4 k, pipelines/driver_ms 9807, pipelines/front_ms 616, render/thread/ms 132, render/busy/ms 130, main/busy/ms 77.08, stage/main/post_update 63.87, stage/render/prepare_resources 30.22, stage/render/prepare_bind_groups 24.85, stage/render/render 24.01, encode//sum 22.48 |
| warm-up | 10.94 | 49.0 | pipelines/wait_ms 22.3 k, pipelines/compile_ms 1730, pipelines/driver_ms 1457, pipelines/front_ms 274, render/thread/ms 105, render/busy/ms 103, main/busy/ms 30.84, stage/render/prepare_flush 30.07, stage/render/prepare_resources 23.76, stage/main/post_update 18.36, stage/render/prepare_bind_groups 16.85, stage/render/render 14.58 |
| warm-up | 12.97 | 53.4 | render/thread/ms 66.83, render/busy/ms 64.92, main/busy/ms 29.35, stage/render/render 16.99, encode//sum 16.96, stage/main/post_update 16.44, stage/render/specialize 15.90, render/graph/ms 14.66, stage/render/queue 14.30, stage/render/graph_submit 10.30, stage/main/update 8.89, encode/submit 6.89 |
| warm-up | 14.21 | 53.2 | pipelines/wait_ms 17.9 k, pipelines/compile_ms 141, pipelines/driver_ms 99.31, render/thread/ms 92.38, render/busy/ms 90.63, main/busy/ms 50.58, pipelines/front_ms 42.06, stage/main/post_update 24.93, stage/render/prepare_resources 23.94, stage/render/prepare_assets 16.55, stage/render/render 15.60, stage/render/prepare_write_phase_buffers 14.83 |
| warm-up | 14.34 | 56.6 | render/thread/ms 94.70, render/busy/ms 93.35, main/busy/ms 50.21, stage/main/post_update 37.13, stage/render/prepare_resources 31.08, stage/render/render 19.58, render/graph/ms 17.45, encode//sum 14.86, stage/render/prepare_write_phase_buffers 12.71, stage/render/prepare_flush 11.13, stage/render/camera_driver 9.96, anim/evaluate/ms 9.16 |

## Run

- `../target/profiling/walkonce.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\wgpu_walk_once\balmora_walk@2560x1440_arm=onetex-3 --bench-baseline none --off texvariety
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1386 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **75%** (min 46%) · 2625 MHz (min 2595) · memory 8101 MHz · 78.1 W · 76 °C (max 83) · VRAM 5884 MiB · throttled 99% of samples

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
