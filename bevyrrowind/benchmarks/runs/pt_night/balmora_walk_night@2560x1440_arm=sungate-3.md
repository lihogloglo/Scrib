# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.gate=on (command line: --set); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6129 frames over 65.6 s · **median 10.32 ms (97 fps)** · mean 10.71 · p95 14.39 · p99 16.25 (1% low 62 fps) · worst 31.48 ms · 117 spikes over 1.5× median · 21.9 stutters/min (work 23.8/min)
- warm-up: 323 frames over 9.3 s, worst 2465.1 ms
- hitches (> 25.8 ms): **2** measured, 48 in warm-up
- timed GPU passes: 6.51 ms of a 10.71 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.32 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.94/min, limit 1/min
- pass worst frame 31.48 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.960 | 2.030 | 2.063 | 0.97 | 0.175 |
| main_opaque_pass_3d | 1.254 | 1.298 | 1.647 | 0.97 | 0.063 |
| early prepass | 0.467 | 0.483 | 0.699 | 0.97 | 0.085 |
| bloom | 0.305 | 0.316 | 0.322 | 0.97 | 0.063 |
| atmosphere_luts | 0.299 | 0.310 | 0.374 | 0.97 | 0.003 |
| auxiliary_views | 0.259 | 0.268 | 1.127 | 0.97 | 0.245 |
| water_velocity | 0.253 | 0.262 | 0.350 | 0.97 | 0.002 |
| water_sspr | 0.195 | 0.201 | 0.203 | 0.97 | 0.002 |
| ssao | 0.182 | 0.189 | 0.193 | 0.97 | 0.007 |
| upscaling | 0.125 | 0.130 | 0.137 | 0.97 | 0.004 |
| water_fft | 0.124 | 0.128 | 0.137 | 0.97 | 0.003 |
| atmos_froxel_inject | 0.123 | 0.128 | 0.129 | 0.97 | 0.001 |
| uniform_allocation | 0.121 | 0.125 | 0.153 | 0.97 | 0.024 |
| clouds_composite | 0.101 | 0.105 | 0.108 | 0.97 | 0.002 |
| bin_unpacking | 0.094 | 0.097 | 0.119 | 0.97 | 0.018 |
| tonemapping | 0.084 | 0.087 | 0.091 | 0.97 | 0.003 |
| main_transmissive_pass_3d | 0.076 | 0.079 | 0.157 | 0.97 | 0.008 |
| clustering | 0.062 | 0.064 | 0.067 | 0.97 | 0.019 |
| clouds_trace | 0.054 | 0.055 | 0.074 | 0.97 | 0.001 |
| render_sky | 0.050 | 0.051 | 0.052 | 0.97 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.97 | 0.001 |
| lightprobe_irradiance_map | 0.043 | 0.412 | 0.416 | 0.10 | 0.001 |
| surface_trail | 0.038 | 0.056 | 0.057 | 0.67 | 0.002 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.031 | 0.97 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.028 | 0.036 | 0.97 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.029 | 0.97 | 0.002 |
| lightprobe_radiance_map | 0.023 | 0.218 | 0.229 | 0.10 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.97 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.035 | 0.97 | 0.001 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.024 | 0.97 | 0.123 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 0.97 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.97 | 0.005 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.97 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.97 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.299 | 12.969 | 24.23 | 56992.5 |
| main/busy/ms | 8.905 | 10.925 | 28.72 | 54581.2 |
| render/busy/ms | 8.786 | 12.439 | 23.46 | 53851.7 |
| stage/render/render | 5.630 | 8.521 | 16.37 | 34508.1 |
| stage/main/post_update | 5.543 | 7.202 | 10.18 | 33974.4 |
| render/graph/ms | 4.935 | 7.757 | 15.31 | 30245.7 |
| encode//sum | 3.950 | 6.077 | 28.76 | 24206.9 |
| stage/render/graph_submit | 3.675 | 5.682 | 11.31 | 22522.1 |
| encode/submit | 2.387 | 3.894 | 6.11 | 14626.9 |
| stage/main/update | 2.125 | 2.590 | 22.26 | 13021.4 |
| encode/pass:shadow_point_light/sum | 1.341 | 5.327 | 16.88 | 8216.7 |
| encode/finish_wall | 1.328 | 1.876 | 5.47 | 8140.5 |
| stage/render/camera_driver | 1.208 | 2.071 | 4.20 | 7403.4 |
| encode/pass:shadow_directional_light/sum | 0.749 | 1.235 | 1.99 | 4592.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.739 | 1.052 | 1.66 | 4531.4 |
| stage/render/prepare_write_phase_buffers | 0.739 | 1.092 | 2.84 | 4528.2 |
| encode/pass:early prepass/sum | 0.722 | 1.044 | 1.75 | 4425.0 |
| stage/render/prepare_assets | 0.570 | 0.720 | 8.19 | 3492.1 |
| bigspace/total/ms | 0.550 | 1.364 | 2.44 | 3372.7 |
| stage/render/prepare_resources | 0.440 | 0.527 | 1.33 | 2696.2 |
| render/present/ms | 0.420 | 0.532 | 2.23 | 2572.7 |
| stage/render/prepare_meshes | 0.406 | 0.517 | 2.21 | 2489.4 |
| stage/main/first | 0.394 | 0.515 | 5.98 | 2417.3 |
| stage/main/pre_update | 0.350 | 0.482 | 1.26 | 2142.6 |
| terrain/update/ms | 0.310 | 0.399 | 2.53 | 1900.0 |
| terrain/select/ms | 0.277 | 0.357 | 1.17 | 1695.3 |
| controller/feed/ms | 0.263 | 0.324 | 1.02 | 1609.7 |
| stage/render/prepare_bind_groups | 0.253 | 0.434 | 2.28 | 1553.6 |
| stage/render/specialize | 0.247 | 0.375 | 2.45 | 1513.9 |
| stream/drive/ms | 0.246 | 0.331 | 20.17 | 1506.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.231 | 0.341 | 0.94 | 1414.0 |
| stage/render/queue | 0.228 | 0.359 | 2.53 | 1394.6 |
| bigspace/low_precision/ms | 0.223 | 0.700 | 1.80 | 1367.6 |
| stage/main/fixed_main_loop | 0.209 | 0.358 | 1.09 | 1282.6 |
| far/select/ms | 0.200 | 0.267 | 0.83 | 1228.0 |
| stage/render/create_views | 0.191 | 0.246 | 0.88 | 1168.9 |
| stream/tick/ms | 0.190 | 0.260 | 0.66 | 1166.9 |
| stage/render/prepare_batch_phases | 0.185 | 0.262 | 0.78 | 1131.5 |
| stage/render/extract_commands | 0.168 | 0.197 | 0.50 | 1031.8 |
| anim/evaluate/ms | 0.168 | 0.309 | 1.55 | 1027.7 |
| stage/render/prepare_flush | 0.165 | 0.339 | 1.34 | 1010.4 |
| bigspace/high_precision/ms | 0.164 | 0.713 | 1.83 | 1002.6 |
| anim/evaluate_cpu/ms | 0.154 | 0.193 | 0.69 | 946.1 |
| controller/move/ms | 0.139 | 0.308 | 0.85 | 849.4 |
| stage/render/prepare_views | 0.136 | 0.169 | 0.55 | 831.8 |
| stage/main/last | 0.112 | 0.152 | 1.64 | 689.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.096 | 0.156 | 0.71 | 585.9 |
| render/acquire/ms | 0.093 | 0.119 | 0.51 | 568.2 |
| pipelines/compile_ms | 0.089 | 0.000 | 135.24 | 543.1 |
| pipelines/front_ms | 0.086 | 0.000 | 131.77 | 528.6 |
| stage/render/prepare_collect_phase_buffers | 0.059 | 0.126 | 0.51 | 362.2 |
| physics/step/ms | 0.045 | 0.083 | 0.41 | 277.6 |
| stage/render/cleanup | 0.034 | 0.113 | 0.71 | 206.4 |
| physics/insert/ms | 0.033 | 0.358 | 1.00 | 201.4 |
| stage/render/phase_sort | 0.032 | 0.056 | 0.28 | 198.9 |
| encode/resolve_diagnostics/sum | 0.030 | 0.040 | 0.65 | 182.2 |
| weather/map/ms | 0.016 | 0.020 | 0.29 | 97.5 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.07 | 85.7 |
| anim/apply/ms | 0.011 | 0.020 | 0.49 | 66.0 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.58 | 65.9 |
| controller/avoid/ms | 0.011 | 0.014 | 0.45 | 64.4 |
| weather/eval/ms | 0.010 | 0.013 | 0.07 | 59.3 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 42.2 |
| script/input/ms | 0.005 | 0.006 | 0.17 | 31.6 |
| stream/despawn/ms | 0.005 | 0.000 | 1.17 | 30.3 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 27.9 |
| script/ui/ms | 0.005 | 0.006 | 0.40 | 27.7 |
| controller/write/ms | 0.003 | 0.004 | 0.20 | 20.9 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.47 | 14.6 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.62 | 13.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.22 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.47 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.26 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
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
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1258 | 2262 | 658 |
| crowd/c0/meshes/count | count | 343 | 646 | 188 |
| crowd/c1/count | count | 146 | 155 | 147 |
| crowd/c1/joints/count | count | 5531 | 5894 | 5538 |
| crowd/c1/meshes/count | count | 1153 | 1254 | 1153 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 163 | 216 | 204 |
| crowd/joints_extracted/count | count | 3671 | 9292 | 5731 |
| crowd/joints_uploaded/count | count | 3635 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.39 | 1.60 | 0.36 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.07 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.58 | 0.01 |
| encode/finish_critical | ms | 0.93 | 1.99 | 1.03 |
| encode/pass:early prepass/max | ms | 0.75 | 1.75 | 1.03 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 1.66 | 0.70 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.71 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 0.94 | 0.22 |
| encode/pass:shadow_directional_light/max | ms | 0.76 | 1.99 | 0.98 |
| encode/pass:shadow_point_light/max | ms | 0.79 | 1.42 | 0.89 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.65 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.02 | 28.72 | 8.46 |
| gpu/timed/ms | ms | 7.44 | 10.69 | 7.71 |
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
| phases/alpha_mask3d/items | count | 793 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 793 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1290 | 1399 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3341 | 5150 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1290 | 1399 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3341 | 5150 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 196 | 19 |
| phases/shadow/bins | count | 3686 | 4380 | 1481 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6099 | 7364 | 3025 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 204 | 234 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 204 | 476 | 19 |
| physics/pending | count | 0 | 189 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 323 | 324 | 324 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 1 | 0 |
| preprocess/bind_groups | count | 11 | 553 | 37 |
| preprocess/bind_groups_created | count | 11 | 553 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 133 | 298 | 127 |
| process/cpu_usage | % | 26.44 | 42.79 | 35.41 |
| process/mem_usage | GiB | 5.44 | 5.60 | 5.60 |
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
| skin/blocks_changed/count | count | 13 | 116 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 163 | 216 | 204 |
| skin/joint_bytes_written/bytes | bytes | 278.1 k | 809.1 k | 448.5 k |
| skin/joints_written/count | count | 4345 | 12.6 k | 7008 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4919 | 5010 | 5010 |
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
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.94 | 53.70 | 34.73 |
| system/mem_usage | % | 54.19 | 54.69 | 54.64 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2900 | 3016 | 3016 |
| textures/images | count | 2680 | 2740 | 2740 |
| views/all/batch_sets | count | 69 | 245 | 71 |
| views/all/bins | count | 6177 | 7310 | 4989 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.5 k | 20.2 k | 10.6 k |
| views/all/pipelines | count | 248 | 281 | 67 |
| views/all/pipelines_distinct | count | 71 | 75 | 63 |
| views/all/work_items | count | 10.1 k | 18.1 k | 10.9 k |
| views/cascade/batch_sets | count | 8 | 27 | 19 |
| views/cascade/bins | count | 1442 | 1558 | 1481 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3392 | 4424 | 3025 |
| views/cascade/pipelines | count | 22 | 27 | 19 |
| views/cascade/work_items | count | 324 | 4424 | 3025 |
| views/lamp_face/batch_sets | count | 0 | 188 | 0 |
| views/lamp_face/bins | count | 2436 | 2956 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2694 | 3272 | 0 |
| views/lamp_face/pipelines | count | 183 | 208 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2883 | 0 |
| views/main/batch_sets | count | 51 | 53 | 52 |
| views/main/bins | count | 2888 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8276 | 13.0 k | 7614 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8436 | 13.2 k | 7922 |
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
| anim/bones_written/count | count | 45.83 | 244 | 280.9 k |
| anim/events/count | count | 0.09 | 4 | 556 |
| assets/Image/added | count | 0.10 | 32 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.66 | 152 | 4061 |
| assets/Mesh/removed | count | 0.79 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.07 | 29 | 457 |
| assets/StandardMaterial/modified | count | 3.53 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.76 | 1312 | 4675 |
| encode//count | count | 37.69 | 219 | 231.0 k |
| encode/bind_walk/init_textures | count | 14.8 k | 112.9 k | 90.64 M |
| encode/bind_walk/init_textures_skipped | count | 54.4 k | 116.3 k | 333.72 M |
| encode/bind_walk/inits | count | 469 | 1116 | 2.87 M |
| encode/bind_walk/inits_skipped | count | 99.98 | 150 | 612.8 k |
| encode/bind_walk/merge_textures | count | 14.7 k | 112.9 k | 90.25 M |
| encode/bind_walk/merge_textures_skipped | count | 5395 | 6012 | 33.07 M |
| encode/bind_walk/merges | count | 104 | 339 | 640.4 k |
| encode/bind_walk/merges_skipped | count | 80.14 | 123 | 491.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6129 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6129 |
| encode/pass:early prepass/count | count | 1 | 1 | 6129 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6129 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6129 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6129 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6129 |
| encode/pass:shadow_point_light/count | count | 2.69 | 48 | 16.5 k |
| encode/queue/command_buffers | count | 50.40 | 277 | 308.9 k |
| encode/queue/submits | count | 2.02 | 4 | 12.4 k |
| encode/queue/write_buffer/bytes | bytes | 1.81 M | 2.95 M | 11.11 G |
| encode/queue/write_buffer/calls | count | 264 | 825 | 1.62 M |
| encode/queue/write_buffer_with/bytes | bytes | 169.7 k | 3.53 M | 1.04 G |
| encode/queue/write_buffer_with/calls | count | 10.97 | 465 | 67.3 k |
| encode/queue/write_texture/bytes | bytes | 10.3 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6129 |
| physics/inserts | count | 0.37 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2335 |
| physics/steps | count | 0.64 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 6 |
| pipelines/created | count | 0.00 | 1 | 7 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5523 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1211 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.8 ms; slowest 50, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2465.1 | main/busy/ms 2435, stage/main/post_update 2393, stage/main/update 32.25, stage/main/pre_update 5.55, stage/main/last 1.12, surface/advance_cpu_ms 0.74, stream/drive/ms 0.55, stage/main/first 0.54, stage/main/fixed_main_loop 0.36, surface/cell_upload_ms 0.30, stream/tick/ms 0.14, bigspace/total/ms 0.08 |
| warm-up | 3.22 | 756.5 | main/busy/ms 2582, stage/main/post_update 2422, render/thread/ms 718, render/busy/ms 538, stage/render/prepare_assets 487, stage/render/prepare_views 174, render/acquire/ms 174, stage/main/update 145, stage/render/create_views 30.57, stage/render/render 15.10, render/graph/ms 8.28, stage/main/pre_update 7.53 |
| warm-up | 3.36 | 141.7 | render/thread/ms 840, render/busy/ms 595, stage/render/prepare_assets 523, main/busy/ms 220, stage/render/prepare_views 174, render/acquire/ms 174, pipelines/compile_ms 129, stage/main/update 117, stage/render/render 94.35, render/present/ms 71.50, pipelines/front_ms 67.40, pipelines/driver_ms 61.33 |
| warm-up | 3.41 | 47.0 | pipelines/compile_ms 470, pipelines/front_ms 322, render/thread/ms 163, pipelines/driver_ms 148, pipelines/wait_ms 126, main/busy/ms 120, stage/render/render 113, render/present/ms 92.78, render/busy/ms 69.80, stage/main/post_update 60.35, stage/render/prepare_assets 37.76, stage/main/pre_update 37.13 |
| warm-up | 3.53 | 115.4 | pipelines/wait_ms 3805, pipelines/compile_ms 1526, pipelines/front_ms 1377, render/thread/ms 151, pipelines/driver_ms 149, render/busy/ms 123, main/busy/ms 109, stage/main/post_update 54.27, stage/render/render 50.89, stage/main/update 34.84, render/present/ms 27.87, stage/render/prepare_flush 27.10 |
| warm-up | 3.58 | 56.7 | pipelines/wait_ms 4159, pipelines/compile_ms 1826, pipelines/front_ms 1730, render/thread/ms 164, render/busy/ms 162, main/busy/ms 155, pipelines/driver_ms 95.29, stage/main/update 89.65, stage/render/prepare_assets 38.41, stage/render/prepare_resources 28.24, stage/render/prepare_flush 27.16, encode//sum 26.79 |
| warm-up | 3.70 | 115.1 | pipelines/compile_ms 1435, pipelines/front_ms 1358, pipelines/wait_ms 598, render/thread/ms 123, render/busy/ms 121, main/busy/ms 117, stage/render/prepare_assets 82.27, pipelines/driver_ms 77.56, stage/main/update 68.03, stage/main/post_update 22.21, stage/main/pre_update 21.72, stage/render/prepare_resources 17.81 |
| warm-up | 3.76 | 64.8 | pipelines/compile_ms 1881, pipelines/front_ms 1809, pipelines/wait_ms 364, render/thread/ms 122, render/busy/ms 121, stage/render/prepare_assets 102, main/busy/ms 77.37, pipelines/driver_ms 71.99, stage/main/pre_update 41.87, stage/main/post_update 18.81, encode//sum 13.99, stage/render/render 12.07 |
| warm-up | 3.80 | 35.2 | pipelines/compile_ms 1353, pipelines/front_ms 1309, pipelines/wait_ms 247, render/thread/ms 85.50, main/busy/ms 70.17, render/busy/ms 66.93, pipelines/driver_ms 43.64, stage/main/pre_update 41.47, stage/render/prepare_assets 41.40, stage/render/render 31.26, encode//sum 28.30, encode/submit 17.31 |
| warm-up | 3.84 | 29.3 | pipelines/compile_ms 40.93, pipelines/front_ms 39.07, render/thread/ms 34.23, render/busy/ms 32.96, main/busy/ms 25.50, stage/render/prepare_assets 18.65, stage/main/post_update 16.39, terrain/update/ms 8.89, stage/render/render 8.44, encode//sum 7.12, render/graph/ms 6.67, stage/main/update 5.39 |
| warm-up | 3.92 | 64.4 | pipelines/compile_ms 848, pipelines/front_ms 811, main/busy/ms 54.64, render/thread/ms 49.30, render/busy/ms 47.87, stage/main/update 43.48, stream/drive/ms 39.81, pipelines/driver_ms 37.53, stage/render/prepare_assets 31.00, stage/render/render 11.30, render/graph/ms 9.54, stage/main/post_update 7.45 |
| warm-up | 3.98 | 34.7 | pipelines/compile_ms 478, pipelines/front_ms 459, render/thread/ms 44.45, render/busy/ms 40.14, main/busy/ms 33.62, stage/render/prepare_assets 21.69, stage/main/update 20.03, pipelines/driver_ms 19.39, stage/render/render 15.23, pipelines/wait_ms 14.20, pipelines/lock_ms 13.87, render/graph/ms 10.88 |
| warm-up | 4.06 | 57.7 | pipelines/compile_ms 68.94, pipelines/front_ms 65.68, render/thread/ms 58.37, render/busy/ms 56.62, main/busy/ms 48.62, stage/render/prepare_assets 37.64, stage/main/update 32.93, stream/drive/ms 26.79, stage/render/render 10.44, render/graph/ms 8.21, stage/main/post_update 6.61, encode//sum 5.28 |
| warm-up | 4.10 | 35.1 | pipelines/compile_ms 111, pipelines/front_ms 105, render/thread/ms 67.31, render/busy/ms 64.28, main/busy/ms 40.94, stage/render/prepare_assets 38.59, stage/main/update 17.17, stage/render/render 12.58, stage/main/post_update 10.55, render/graph/ms 10.14, stage/render/graph_submit 6.98, encode//sum 6.41 |
| warm-up | 4.18 | 83.3 | pipelines/compile_ms 1366, pipelines/front_ms 1320, render/thread/ms 81.05, render/busy/ms 77.90, stage/render/prepare_assets 50.35, pipelines/driver_ms 45.55, main/busy/ms 44.69, stage/main/update 19.65, stage/render/render 15.72, pipelines/wait_ms 14.36, stage/main/post_update 14.24, pipelines/lock_ms 13.66 |
| warm-up | 4.27 | 94.1 | pipelines/compile_ms 2047, pipelines/front_ms 1973, render/thread/ms 140, render/busy/ms 137, stage/render/prepare_assets 84.01, main/busy/ms 75.71, pipelines/driver_ms 74.27, stage/main/update 45.99, stage/render/render 41.59, render/graph/ms 38.33, stage/render/graph_submit 32.83, encode/submit 28.76 |
| warm-up | 4.37 | 99.6 | pipelines/compile_ms 1807, pipelines/front_ms 1746, pipelines/wait_ms 223, render/thread/ms 156, render/busy/ms 153, stage/render/prepare_assets 92.31, main/busy/ms 83.01, pipelines/driver_ms 61.01, stage/main/update 43.89, stage/render/render 39.50, render/graph/ms 36.40, stage/render/graph_submit 32.57 |
| warm-up | 4.47 | 98.7 | pipelines/compile_ms 2663, pipelines/front_ms 2589, pipelines/wait_ms 533, render/thread/ms 161, render/busy/ms 159, pipelines/driver_ms 73.79, main/busy/ms 68.80, stage/render/prepare_assets 54.93, encode//sum 51.62, stage/render/render 38.23, stage/main/update 37.76, render/graph/ms 35.77 |
| warm-up | 4.57 | 97.4 | pipelines/compile_ms 2516, pipelines/front_ms 2444, pipelines/wait_ms 609, render/thread/ms 187, render/busy/ms 184, main/busy/ms 87.10, stage/render/render 77.99, encode//sum 75.86, render/graph/ms 75.42, pipelines/driver_ms 72.32, stage/main/update 50.58, stage/render/camera_driver 48.70 |
| warm-up | 4.64 | 68.9 | pipelines/compile_ms 2491, pipelines/front_ms 2397, pipelines/wait_ms 1088, render/thread/ms 142, render/busy/ms 139, pipelines/driver_ms 93.88, main/busy/ms 85.65, stage/render/render 57.58, render/graph/ms 55.28, stage/render/prepare_assets 45.99, stage/render/camera_driver 39.61, encode//sum 37.41 |
| warm-up | 4.72 | 83.5 | pipelines/compile_ms 2257, pipelines/front_ms 2165, pipelines/wait_ms 886, main/busy/ms 113, render/thread/ms 104, render/busy/ms 102, pipelines/driver_ms 92.25, stage/main/update 76.10, stream/drive/ms 56.28, stage/render/prepare_assets 47.07, stage/render/render 24.87, render/graph/ms 22.56 |
| warm-up | 4.77 | 50.3 | pipelines/compile_ms 1165, pipelines/front_ms 1127, pipelines/wait_ms 166, main/busy/ms 96.01, render/thread/ms 84.60, render/busy/ms 83.01, stage/main/update 70.29, stream/drive/ms 57.95, pipelines/driver_ms 38.33, stage/render/prepare_assets 36.09, stage/render/render 20.60, render/graph/ms 18.08 |
| warm-up | 4.81 | 33.3 | pipelines/compile_ms 1315, pipelines/front_ms 1280, pipelines/wait_ms 583, main/busy/ms 119, stage/main/update 94.12, render/thread/ms 58.61, render/busy/ms 57.04, stream/drive/ms 47.94, pipelines/driver_ms 35.02, stage/render/render 15.51, stage/main/post_update 14.82, stage/render/prepare_assets 13.84 |
| warm-up | 4.96 | 149.6 | pipelines/compile_ms 1706, pipelines/front_ms 1661, pipelines/wait_ms 515, main/busy/ms 116, stage/main/update 95.12, render/thread/ms 85.60, render/busy/ms 83.97, stream/drive/ms 46.70, pipelines/driver_ms 44.72, stage/render/prepare_assets 39.73, stage/render/render 23.43, render/graph/ms 21.21 |
| warm-up | 5.03 | 72.0 | pipelines/compile_ms 1182, pipelines/front_ms 1137, render/thread/ms 125, render/busy/ms 123, main/busy/ms 80.68, stage/render/prepare_assets 65.01, stage/main/update 62.76, pipelines/driver_ms 44.81, stage/render/render 40.45, render/graph/ms 38.18, stream/drive/ms 36.39, stage/render/graph_submit 25.76 |
| warm-up | 5.08 | 50.4 | pipelines/compile_ms 331, pipelines/front_ms 307, render/thread/ms 101, render/busy/ms 99.54, main/busy/ms 76.23, stage/main/update 57.96, stage/render/prepare_assets 50.13, stream/drive/ms 36.15, stage/render/render 33.21, render/graph/ms 31.13, pipelines/driver_ms 24.08, stage/render/graph_submit 19.04 |
| warm-up | 5.15 | 56.6 | pipelines/compile_ms 820, pipelines/front_ms 796, main/busy/ms 57.36, stage/main/update 45.76, stream/drive/ms 35.46, render/thread/ms 33.80, render/busy/ms 32.03, pipelines/driver_ms 23.43, stage/render/render 13.99, render/graph/ms 11.49, stage/render/prepare_assets 9.63, encode//sum 9.05 |
| warm-up | 5.18 | 29.5 | pipelines/compile_ms 1103, pipelines/front_ms 1071, render/thread/ms 46.89, render/busy/ms 45.46, main/busy/ms 35.86, pipelines/driver_ms 32.43, stage/main/post_update 18.07, stage/render/render 16.33, render/graph/ms 14.43, stage/main/update 12.96, stage/render/prepare_assets 10.53, encode//sum 10.20 |
| warm-up | 5.26 | 27.8 | render/thread/ms 46.77, render/busy/ms 45.13, main/busy/ms 30.30, stage/render/render 19.10, render/graph/ms 16.99, stage/main/post_update 12.50, stage/main/update 12.34, stage/render/graph_submit 12.21, encode//sum 8.99, encode/submit 8.43, stage/render/prepare_assets 7.79, stream/drive/ms 4.98 |
| warm-up | 5.29 | 30.5 | pipelines/compile_ms 191, pipelines/front_ms 185, render/thread/ms 53.97, render/busy/ms 52.12, main/busy/ms 46.63, stage/render/render 24.84, render/graph/ms 22.40, stage/main/update 19.12, stage/render/graph_submit 16.34, stage/main/post_update 14.81, encode//sum 12.43, encode/submit 10.29 |
| warm-up | 5.34 | 56.1 | pipelines/compile_ms 1050, pipelines/front_ms 1025, render/thread/ms 67.23, render/busy/ms 65.54, main/busy/ms 45.83, stage/render/prepare_assets 31.38, pipelines/driver_ms 25.37, stage/render/render 21.74, stage/main/update 19.70, render/graph/ms 19.51, stage/render/graph_submit 14.48, stage/main/post_update 13.46 |
| warm-up | 5.38 | 32.0 | pipelines/compile_ms 1403, pipelines/front_ms 1366, render/thread/ms 68.74, render/busy/ms 67.09, main/busy/ms 43.71, pipelines/driver_ms 37.06, stage/render/prepare_assets 34.11, stage/main/update 24.79, stage/render/render 19.58, render/graph/ms 17.39, stream/drive/ms 14.19, stage/main/post_update 12.78 |
| warm-up | 5.41 | 32.4 | pipelines/compile_ms 544, pipelines/front_ms 527, render/thread/ms 55.99, render/busy/ms 54.56, main/busy/ms 40.31, stage/render/prepare_assets 23.22, stage/main/update 22.33, pipelines/driver_ms 17.63, stage/render/render 17.02, render/graph/ms 14.92, stream/drive/ms 12.91, stage/main/post_update 11.42 |
| warm-up | 5.70 | 26.8 | pipelines/compile_ms 144, pipelines/front_ms 142, render/thread/ms 35.08, render/busy/ms 34.04, main/busy/ms 27.83, stage/main/update 16.23, stage/render/prepare_assets 13.47, stage/render/render 12.24, encode//sum 11.94, render/graph/ms 10.60, stream/drive/ms 9.14, stage/main/post_update 7.92 |
| warm-up | 5.77 | 32.6 | main/busy/ms 34.52, render/thread/ms 31.06, render/busy/ms 29.84, stage/main/update 22.29, stream/drive/ms 14.21, stage/render/render 11.17, pipelines/compile_ms 10.94, pipelines/front_ms 9.97, encode//sum 9.79, stage/main/post_update 9.33, render/graph/ms 9.29, stage/render/prepare_assets 8.52 |
| warm-up | 6.04 | 28.4 | main/busy/ms 32.69, render/thread/ms 24.63, render/busy/ms 23.70, stage/main/update 20.06, stream/drive/ms 14.94, stage/render/render 10.82, render/graph/ms 9.30, encode//sum 8.49, stage/main/post_update 8.27, stage/render/graph_submit 5.87, stage/render/prepare_assets 4.79, encode/pass:shadow_point_light/sum 4.02 |
| warm-up | 6.17 | 31.4 | render/thread/ms 42.22, render/busy/ms 41.12, main/busy/ms 25.56, stage/render/prepare_assets 19.36, stage/main/update 10.16, stage/render/render 9.89, stage/main/post_update 8.82, encode//sum 8.73, render/graph/ms 8.47, stage/render/graph_submit 5.77, encode/pass:shadow_point_light/sum 4.60, stream/drive/ms 4.33 |
| warm-up | 6.37 | 25.8 | main/busy/ms 38.28, render/thread/ms 37.23, render/busy/ms 36.06, stage/main/update 18.22, stage/main/post_update 15.46, stage/render/render 11.79, render/graph/ms 9.88, encode//sum 8.38, stage/render/prepare_assets 8.09, stage/render/graph_submit 6.72, stream/drive/ms 5.67, encode/pass:shadow_point_light/sum 4.69 |
| warm-up | 6.40 | 27.8 | render/thread/ms 44.81, render/busy/ms 43.62, main/busy/ms 42.31, stage/main/update 18.33, stage/main/post_update 18.15, stage/render/render 16.80, render/graph/ms 15.03, encode//sum 11.53, stage/render/prepare_assets 11.22, stage/render/graph_submit 10.46, encode/submit 7.31, encode/pass:shadow_point_light/sum 7.01 |
| warm-up | 6.43 | 29.3 | render/thread/ms 46.76, render/busy/ms 45.40, main/busy/ms 39.91, stage/render/render 19.01, stage/main/update 18.03, render/graph/ms 16.94, stage/main/post_update 15.31, stage/render/prepare_assets 13.73, encode//sum 12.28, stage/render/graph_submit 11.71, encode/pass:shadow_point_light/sum 8.95, encode/submit 8.26 |
| warm-up | 6.45 | 28.1 | render/thread/ms 43.42, render/busy/ms 42.17, main/busy/ms 32.12, stage/render/prepare_assets 17.81, stage/main/update 13.96, stage/render/render 13.23, stage/main/post_update 11.56, render/graph/ms 11.45, encode//sum 10.03, stage/render/graph_submit 7.75, encode/pass:shadow_point_light/sum 7.10, encode/submit 4.83 |
| warm-up | 6.48 | 26.3 | render/thread/ms 45.72, render/busy/ms 44.14, main/busy/ms 37.34, stage/render/render 19.99, render/graph/ms 17.97, stage/main/post_update 15.69, stage/main/update 15.56, encode//sum 13.89, stage/render/graph_submit 13.10, stage/render/prepare_assets 10.55, encode/pass:shadow_point_light/sum 9.71, encode/submit 8.91 |
| warm-up | 6.53 | 30.2 | pipelines/compile_ms 188, pipelines/front_ms 183, render/thread/ms 38.60, render/busy/ms 37.58, main/busy/ms 33.82, stage/main/update 18.11, stage/render/prepare_assets 16.56, stage/render/render 11.23, stage/main/post_update 10.41, render/graph/ms 9.51, encode//sum 8.54, encode/pass:shadow_point_light/sum 5.63 |
| warm-up | 6.60 | 30.4 | render/thread/ms 43.48, render/busy/ms 42.37, main/busy/ms 23.86, stage/render/prepare_assets 20.80, stage/render/render 12.22, stage/main/post_update 11.18, render/graph/ms 10.75, encode//sum 9.61, stage/main/update 9.02, stage/render/graph_submit 7.30, encode/submit 4.76, encode/pass:shadow_point_light/sum 4.64 |
| warm-up | 6.68 | 36.8 | pipelines/compile_ms 151, pipelines/front_ms 147, render/thread/ms 50.40, render/busy/ms 49.19, main/busy/ms 44.59, stage/main/update 26.26, stage/render/prepare_assets 25.43, stream/drive/ms 16.38, stage/main/post_update 13.42, stage/render/render 13.25, render/graph/ms 11.38, encode//sum 10.75 |
| warm-up | 6.71 | 25.8 | pipelines/compile_ms 151, pipelines/front_ms 147, render/thread/ms 54.45, render/busy/ms 53.22, stage/render/prepare_assets 31.16, main/busy/ms 30.30, stage/main/update 12.83, stage/main/post_update 12.58, stage/render/render 11.71, render/graph/ms 9.98, encode//sum 9.38, stage/render/graph_submit 6.83 |
| warm-up | 6.76 | 28.5 | render/thread/ms 43.53, render/busy/ms 42.55, main/busy/ms 25.29, stage/render/prepare_assets 25.09, pipelines/compile_ms 19.86, pipelines/front_ms 18.75, stage/render/render 10.78, stage/main/update 10.19, stage/main/post_update 9.78, render/graph/ms 9.23, encode//sum 8.16, stage/render/graph_submit 6.29 |
| warm-up | 6.88 | 27.6 | render/thread/ms 36.65, render/busy/ms 35.44, main/busy/ms 34.58, stage/render/render 18.67, stage/main/post_update 17.48, render/graph/ms 16.92, encode//sum 15.10, stage/render/graph_submit 10.98, stage/main/update 10.67, encode/pass:shadow_point_light/sum 10.66, encode/submit 6.75, stage/render/camera_driver 5.76 |
| measure | 0.72 | 27.7 | main/busy/ms 34.72, render/thread/ms 26.84, render/busy/ms 25.72, stage/main/update 17.62, stage/main/post_update 13.87, stage/render/render 13.32, stream/drive/ms 13.32, render/graph/ms 11.90, encode//sum 9.20, stage/render/graph_submit 8.31, encode/pass:shadow_point_light/sum 6.96, stage/render/prepare_assets 5.79 |
| measure | 18.18 | 31.5 | main/busy/ms 38.90, stage/main/update 24.40, render/thread/ms 23.07, render/busy/ms 22.05, stream/drive/ms 20.43, stage/render/render 14.11, render/graph/ms 12.44, stage/main/post_update 11.94, encode//sum 10.86, stage/render/graph_submit 8.99, encode/pass:shadow_point_light/sum 8.79, encode/submit 5.87 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.sun.gate=on --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=sungate-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 8%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **73%** (min 42%) · 2625 MHz (min 2625) · memory 8101 MHz · 76.7 W · 76 °C (max 82) · VRAM 7014 MiB · throttled 100% of samples

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
