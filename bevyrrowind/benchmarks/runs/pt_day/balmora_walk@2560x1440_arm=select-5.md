# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); select.gate=on (command line: --set); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6219 frames over 65.6 s · **median 10.31 ms (97 fps)** · mean 10.56 · p95 13.15 · p99 16.05 (1% low 62 fps) · worst 44.75 ms · 77 spikes over 1.5× median · 56.7 stutters/min (work 55.8/min)
- warm-up: 324 frames over 9.1 s, worst 2618.9 ms
- hitches (> 25.8 ms): **34** measured, 49 in warm-up
- timed GPU passes: 6.35 ms of a 10.56 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.31 ms, limit 11 ms
- **FAIL** frame_ms: 62 stutters (> 1.5x moving median of 60) in 1.09 min = 56.66/min, limit 1/min
- **FAIL** worst frame 44.75 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.799 | 2.035 | 2.065 | 0.88 | 0.172 |
| main_opaque_pass_3d | 1.430 | 1.618 | 2.022 | 0.88 | 0.059 |
| early prepass | 0.433 | 0.490 | 0.718 | 0.88 | 0.087 |
| bloom | 0.278 | 0.315 | 0.321 | 0.88 | 0.062 |
| atmosphere_luts | 0.267 | 0.302 | 0.365 | 0.88 | 0.003 |
| water_velocity | 0.247 | 0.279 | 0.347 | 0.88 | 0.002 |
| water_sspr | 0.178 | 0.202 | 0.203 | 0.88 | 0.002 |
| ssao | 0.167 | 0.189 | 0.193 | 0.88 | 0.007 |
| auxiliary_views | 0.134 | 0.152 | 0.601 | 0.88 | 0.147 |
| atmos_froxel_inject | 0.129 | 0.146 | 0.148 | 0.88 | 0.001 |
| clouds_trace | 0.124 | 0.140 | 0.232 | 0.88 | 0.001 |
| water_fft | 0.117 | 0.133 | 0.139 | 0.88 | 0.003 |
| uniform_allocation | 0.116 | 0.131 | 0.183 | 0.88 | 0.024 |
| upscaling | 0.105 | 0.119 | 0.126 | 0.88 | 0.004 |
| clouds_composite | 0.093 | 0.106 | 0.108 | 0.88 | 0.002 |
| bin_unpacking | 0.093 | 0.105 | 0.142 | 0.88 | 0.018 |
| tonemapping | 0.075 | 0.085 | 0.089 | 0.88 | 0.003 |
| main_transmissive_pass_3d | 0.075 | 0.085 | 0.177 | 0.88 | 0.009 |
| clustering | 0.057 | 0.064 | 0.067 | 0.88 | 0.018 |
| render_sky | 0.046 | 0.052 | 0.053 | 0.88 | 0.002 |
| clouds_resolve | 0.043 | 0.049 | 0.049 | 0.88 | 0.001 |
| surface_exposure | 0.039 | 0.044 | 0.045 | 0.88 | 0.001 |
| surface_trail | 0.034 | 0.056 | 0.057 | 0.60 | 0.002 |
| surface_exposure_far | 0.033 | 0.042 | 0.045 | 0.79 | 0.002 |
| lightprobe_irradiance_map | 0.032 | 0.413 | 0.416 | 0.08 | 0.001 |
| atmos_froxel_integrate | 0.027 | 0.031 | 0.033 | 0.88 | 0.001 |
| clouds_shadow | 0.027 | 0.031 | 0.033 | 0.88 | 0.003 |
| water_snapshot | 0.027 | 0.030 | 0.035 | 0.88 | 0.002 |
| early_mesh_preprocessing | 0.027 | 0.030 | 0.046 | 0.88 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.88 | 0.002 |
| lightprobe_radiance_map | 0.017 | 0.219 | 0.229 | 0.08 | 0.004 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.88 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.026 | 0.88 | 0.122 |
| clouds_fog_light | 0.010 | 0.012 | 0.012 | 0.88 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.88 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.88 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.88 | 0.003 |
| lightprobe_copy | 0.001 | 0.015 | 0.016 | 0.08 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.08 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.08 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.134 | 11.475 | 39.80 | 56804.6 |
| main/busy/ms | 8.568 | 10.199 | 28.28 | 53286.2 |
| render/busy/ms | 8.561 | 10.964 | 39.18 | 53238.5 |
| stage/main/post_update | 5.383 | 6.703 | 9.19 | 33479.3 |
| stage/render/render | 5.330 | 6.992 | 14.44 | 33145.9 |
| render/graph/ms | 4.566 | 6.228 | 13.37 | 28395.5 |
| encode//sum | 3.694 | 4.933 | 21.36 | 22973.1 |
| stage/render/graph_submit | 3.404 | 4.715 | 9.76 | 21171.1 |
| encode/submit | 2.201 | 3.231 | 6.28 | 13688.5 |
| stage/main/update | 1.932 | 2.307 | 21.85 | 12012.1 |
| encode/finish_wall | 1.304 | 1.923 | 4.12 | 8111.7 |
| stage/render/camera_driver | 1.109 | 1.567 | 3.54 | 6899.4 |
| encode/pass:shadow_point_light/sum | 0.847 | 3.311 | 15.76 | 5270.2 |
| encode/pass:shadow_directional_light/sum | 0.822 | 1.495 | 2.47 | 5110.1 |
| stage/render/prepare_write_phase_buffers | 0.758 | 1.077 | 3.21 | 4713.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.726 | 1.030 | 1.91 | 4514.7 |
| encode/pass:early prepass/sum | 0.711 | 1.020 | 1.70 | 4422.4 |
| stage/render/prepare_assets | 0.547 | 0.710 | 10.27 | 3398.8 |
| bigspace/total/ms | 0.503 | 1.200 | 2.63 | 3128.1 |
| render/present/ms | 0.480 | 1.263 | 2.42 | 2982.6 |
| stage/render/prepare_resources | 0.440 | 0.521 | 5.72 | 2734.4 |
| stage/render/prepare_meshes | 0.433 | 0.547 | 7.41 | 2691.6 |
| stage/main/first | 0.395 | 0.520 | 5.84 | 2458.4 |
| stage/main/pre_update | 0.354 | 0.526 | 2.96 | 2200.4 |
| stage/render/specialize | 0.307 | 0.402 | 12.73 | 1906.6 |
| stage/render/queue | 0.275 | 0.407 | 8.53 | 1712.8 |
| stage/render/prepare_bind_groups | 0.274 | 0.478 | 2.44 | 1701.3 |
| controller/feed/ms | 0.262 | 0.331 | 0.77 | 1630.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.229 | 0.338 | 0.99 | 1425.2 |
| stage/main/fixed_main_loop | 0.208 | 0.369 | 0.94 | 1294.9 |
| bigspace/low_precision/ms | 0.206 | 0.603 | 1.60 | 1282.3 |
| stream/drive/ms | 0.200 | 0.274 | 19.79 | 1246.9 |
| stage/render/create_views | 0.192 | 0.248 | 1.06 | 1196.5 |
| stage/render/prepare_batch_phases | 0.185 | 0.264 | 0.63 | 1147.9 |
| stage/render/extract_commands | 0.168 | 0.195 | 0.59 | 1044.4 |
| stage/render/prepare_flush | 0.163 | 0.282 | 0.83 | 1010.7 |
| anim/evaluate/ms | 0.153 | 0.238 | 1.64 | 952.7 |
| anim/evaluate_cpu/ms | 0.151 | 0.186 | 0.59 | 939.7 |
| stream/tick/ms | 0.145 | 0.210 | 0.75 | 903.0 |
| bigspace/high_precision/ms | 0.137 | 0.527 | 1.77 | 852.5 |
| stage/render/prepare_views | 0.137 | 0.170 | 0.38 | 851.8 |
| controller/move/ms | 0.135 | 0.302 | 0.56 | 838.3 |
| stage/main/last | 0.116 | 0.162 | 2.46 | 723.9 |
| render/acquire/ms | 0.094 | 0.120 | 0.32 | 583.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.094 | 0.155 | 0.70 | 583.3 |
| terrain/update/ms | 0.056 | 0.283 | 1.71 | 347.4 |
| stage/render/prepare_collect_phase_buffers | 0.056 | 0.119 | 0.43 | 345.7 |
| pipelines/compile_ms | 0.047 | 0.000 | 125.37 | 291.8 |
| physics/step/ms | 0.045 | 0.087 | 0.45 | 281.5 |
| pipelines/front_ms | 0.045 | 0.000 | 121.01 | 279.2 |
| stage/render/cleanup | 0.033 | 0.102 | 1.17 | 206.4 |
| stage/render/phase_sort | 0.031 | 0.053 | 0.34 | 191.3 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.53 | 186.9 |
| physics/insert/ms | 0.030 | 0.337 | 0.79 | 184.2 |
| weather/map/ms | 0.026 | 0.034 | 0.14 | 161.6 |
| terrain/select/ms | 0.022 | 0.252 | 0.75 | 138.8 |
| far/select/ms | 0.016 | 0.166 | 0.53 | 98.1 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.49 | 91.4 |
| anim/apply/ms | 0.011 | 0.019 | 0.18 | 70.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.53 | 68.3 |
| controller/avoid/ms | 0.011 | 0.015 | 0.21 | 68.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.04 | 65.2 |
| script/update/ms | 0.007 | 0.009 | 0.07 | 41.8 |
| script/input/ms | 0.005 | 0.006 | 0.17 | 32.5 |
| stream/despawn/ms | 0.005 | 0.000 | 0.97 | 30.0 |
| script/ui/ms | 0.004 | 0.006 | 0.04 | 27.7 |
| stream/spawn/ms | 0.004 | 0.000 | 1.12 | 27.4 |
| controller/write/ms | 0.004 | 0.005 | 0.27 | 22.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.58 | 14.0 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.36 | 12.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 6.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.42 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.23 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.12 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.10 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 20 | 34 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 26 | 42 | 14 |
| crowd/c0/joints/count | count | 1277 | 2218 | 641 |
| crowd/c0/meshes/count | count | 353 | 633 | 174 |
| crowd/c1/count | count | 146 | 155 | 146 |
| crowd/c1/joints/count | count | 5531 | 5902 | 5523 |
| crowd/c1/meshes/count | count | 1170 | 1256 | 1159 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4612 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4589 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.40 | 2.10 | 0.51 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.49 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.53 | 0.02 |
| encode/finish_critical | ms | 0.93 | 2.47 | 0.92 |
| encode/pass:early prepass/max | ms | 0.72 | 1.70 | 0.61 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.74 | 1.91 | 0.92 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.70 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 0.99 | 0.20 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 2.47 | 0.60 |
| encode/pass:shadow_point_light/max | ms | 0.77 | 1.57 | 0.91 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.53 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 8.83 | 39.18 | 8.22 |
| gpu/timed/ms | ms | 7.92 | 8.92 | 8.22 |
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
| phases/alpha_mask3d/items | count | 819 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 819 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1291 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3418 | 5112 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1291 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3418 | 5112 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 18 | 202 | 4 |
| phases/shadow/bins | count | 5039 | 8903 | 2423 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.2 k | 6374 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 206 | 255 | 30 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3182 | 2349 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 11 | 565 | 9 |
| preprocess/bind_groups_created | count | 11 | 565 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 244 | 120 |
| process/cpu_usage | % | 27.40 | 38.84 | 30.77 |
| process/mem_usage | GiB | 5.50 | 5.74 | 5.74 |
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
| skin/blocks_changed/count | count | 14 | 128 | 14 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 367.1 k | 809.5 k | 494.1 k |
| skin/joints_written/count | count | 5736 | 12.6 k | 7720 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4917 | 5011 | 5011 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 4 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 3 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.16 | 42.44 | 29.56 |
| system/mem_usage | % | 55.42 | 56.25 | 55.50 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 69 | 251 | 54 |
| views/all/bins | count | 7662 | 11.8 k | 5921 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.1 k | 14.0 k |
| views/all/pipelines | count | 253 | 302 | 76 |
| views/all/pipelines_distinct | count | 71 | 74 | 61 |
| views/all/work_items | count | 10.7 k | 35.3 k | 8127 |
| views/cascade/batch_sets | count | 9 | 50 | 4 |
| views/cascade/bins | count | 2808 | 6171 | 2423 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8917 | 23.2 k | 6374 |
| views/cascade/pipelines | count | 36 | 54 | 30 |
| views/cascade/work_items | count | 411 | 22.6 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2160 | 2761 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2418 | 3058 | 0 |
| views/lamp_face/pipelines | count | 168 | 209 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2930 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2880 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8468 | 13.0 k | 7618 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8626 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 56.64 | 203 | 352.3 k |
| anim/events/count | count | 0.08 | 2 | 516 |
| assets/Image/added | count | 0.10 | 34 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.65 | 201 | 4061 |
| assets/Mesh/removed | count | 0.78 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.48 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 60 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.75 | 1328 | 4675 |
| encode//count | count | 33.59 | 220 | 208.9 k |
| encode/bind_walk/init_textures | count | 12.6 k | 114.7 k | 78.66 M |
| encode/bind_walk/init_textures_skipped | count | 65.9 k | 162.0 k | 409.56 M |
| encode/bind_walk/inits | count | 460 | 1131 | 2.86 M |
| encode/bind_walk/inits_skipped | count | 108 | 163 | 668.8 k |
| encode/bind_walk/merge_textures | count | 12.6 k | 114.6 k | 78.21 M |
| encode/bind_walk/merge_textures_skipped | count | 5358 | 6012 | 33.32 M |
| encode/bind_walk/merges | count | 98.92 | 336 | 615.2 k |
| encode/bind_walk/merges_skipped | count | 83.17 | 133 | 517.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6219 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6219 |
| encode/pass:early prepass/count | count | 1 | 1 | 6219 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6219 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6219 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6219 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6219 |
| encode/pass:shadow_point_light/count | count | 1.41 | 48 | 8789 |
| encode/queue/command_buffers | count | 45.01 | 278 | 279.9 k |
| encode/queue/submits | count | 2.02 | 4 | 12.5 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.99 M | 11.67 G |
| encode/queue/write_buffer/calls | count | 262 | 844 | 1.63 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.6 k | 3.52 M | 1.10 G |
| encode/queue/write_buffer_with/calls | count | 10.94 | 612 | 68.1 k |
| encode/queue/write_texture/bytes | bytes | 11.4 k | 8.59 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6219 |
| physics/inserts | count | 0.36 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 175 | 2335 |
| physics/steps | count | 0.63 | 3 | 3939 |
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
| stream/deactivate/items | count | 0.04 | 31 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5443 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 54 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1193 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.8 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.62 | 2618.9 | main/busy/ms 2585, stage/main/post_update 2477, stage/main/update 96.24, surface/advance_cpu_ms 66.49, stage/main/pre_update 8.18, stage/main/last 1.20, surface/cell_upload_ms 0.76, stage/main/first 0.61, stream/drive/ms 0.47, stage/main/fixed_main_loop 0.33, weather/map/ms 0.13, bigspace/total/ms 0.09 |
| warm-up | 3.39 | 771.4 | main/busy/ms 2769, stage/main/post_update 2502, render/thread/ms 738, render/busy/ms 603, stage/render/prepare_assets 554, stage/main/update 248, stage/render/prepare_views 132, render/acquire/ms 132, surface/advance_cpu_ms 66.49, stage/render/create_views 28.42, stage/render/render 11.98, stage/main/pre_update 10.19 |
| warm-up | 3.50 | 111.8 | render/thread/ms 830, render/busy/ms 644, stage/render/prepare_assets 583, main/busy/ms 205, stage/main/update 156, stage/render/prepare_views 133, render/acquire/ms 133, pipelines/compile_ms 120, pipelines/front_ms 83.11, stage/render/render 65.96, render/present/ms 52.70, stage/main/post_update 37.56 |
| warm-up | 3.62 | 93.9 | pipelines/wait_ms 1877, pipelines/compile_ms 848, pipelines/front_ms 764, render/thread/ms 107, main/busy/ms 106, render/busy/ms 103, pipelines/driver_ms 83.62, stage/main/post_update 55.25, stage/main/update 34.72, stage/render/prepare_resources 32.66, stage/render/render 25.96, pipelines/lock_ms 21.95 |
| warm-up | 3.70 | 79.0 | pipelines/wait_ms 4281, pipelines/compile_ms 1452, pipelines/front_ms 1342, main/busy/ms 160, render/thread/ms 151, render/busy/ms 148, pipelines/driver_ms 110, stage/main/update 84.97, stage/render/prepare_resources 59.27, stage/main/post_update 38.51, stage/render/prepare_assets 32.27, stage/main/pre_update 31.76 |
| warm-up | 3.79 | 82.1 | pipelines/wait_ms 2725, pipelines/compile_ms 1665, pipelines/front_ms 1563, render/thread/ms 126, render/busy/ms 124, main/busy/ms 112, pipelines/driver_ms 102, stage/render/prepare_assets 69.26, stage/main/update 57.36, stage/render/prepare_resources 29.31, stage/main/pre_update 28.58, stage/main/post_update 21.12 |
| warm-up | 3.82 | 30.6 | pipelines/compile_ms 1120, pipelines/front_ms 1065, pipelines/wait_ms 341, render/thread/ms 87.03, render/busy/ms 85.14, encode//sum 55.90, pipelines/driver_ms 55.29, stage/render/prepare_assets 50.69, main/busy/ms 50.24, stage/main/post_update 29.00, stage/render/render 24.37, render/graph/ms 21.94 |
| warm-up | 3.85 | 32.4 | pipelines/compile_ms 876, pipelines/front_ms 836, pipelines/wait_ms 107, render/thread/ms 59.44, render/busy/ms 57.50, encode//sum 55.92, pipelines/driver_ms 40.64, main/busy/ms 36.06, stage/render/prepare_assets 24.76, stage/main/post_update 22.90, stage/render/render 22.87, render/graph/ms 20.67 |
| warm-up | 3.88 | 34.4 | pipelines/compile_ms 1227, pipelines/front_ms 1174, pipelines/wait_ms 178, render/thread/ms 63.53, render/busy/ms 61.02, pipelines/driver_ms 52.22, main/busy/ms 48.19, stage/main/update 24.41, stage/render/prepare_assets 21.16, stage/main/post_update 17.81, stage/render/prepare_batch_phases 16.24, stage/render/render 13.78 |
| warm-up | 4.03 | 76.6 | pipelines/compile_ms 1230, pipelines/front_ms 1180, main/busy/ms 59.90, render/thread/ms 52.96, render/busy/ms 51.41, pipelines/driver_ms 49.64, stage/main/update 49.18, stream/drive/ms 44.37, stage/render/prepare_assets 36.47, pipelines/wait_ms 10.59, stage/render/render 10.32, render/graph/ms 8.32 |
| warm-up | 4.07 | 32.4 | pipelines/compile_ms 69.79, pipelines/front_ms 63.27, render/thread/ms 38.53, render/busy/ms 36.90, main/busy/ms 24.04, stage/render/render 17.33, render/graph/ms 15.35, stage/render/prepare_assets 14.61, stage/main/update 13.46, stage/render/graph_submit 10.71, stream/drive/ms 9.74, pipelines/driver_ms 6.52 |
| warm-up | 4.17 | 73.6 | pipelines/compile_ms 725, pipelines/front_ms 700, render/thread/ms 65.58, render/busy/ms 63.88, main/busy/ms 53.08, stage/render/prepare_assets 40.96, stage/main/update 34.53, stream/drive/ms 27.21, pipelines/driver_ms 24.76, stage/render/render 14.57, render/graph/ms 11.88, stage/main/post_update 9.41 |
| warm-up | 4.29 | 83.9 | pipelines/compile_ms 761, pipelines/front_ms 736, render/thread/ms 71.73, render/busy/ms 69.70, main/busy/ms 65.68, stage/render/prepare_assets 48.87, stage/main/update 39.00, pipelines/driver_ms 25.41, stage/main/post_update 17.45, stage/render/render 10.59, render/graph/ms 8.14, stage/render/graph_submit 5.61 |
| warm-up | 4.36 | 76.6 | pipelines/compile_ms 1905, pipelines/front_ms 1848, render/thread/ms 110, render/busy/ms 109, stage/render/prepare_assets 88.91, main/busy/ms 65.37, pipelines/driver_ms 57.56, stage/main/update 40.65, stage/main/post_update 16.74, stage/render/render 9.73, render/graph/ms 7.62, pipelines/wait_ms 6.58 |
| warm-up | 4.43 | 63.2 | pipelines/compile_ms 1795, pipelines/front_ms 1731, render/thread/ms 112, render/busy/ms 110, pipelines/wait_ms 88.75, stage/render/prepare_assets 65.76, pipelines/driver_ms 64.49, main/busy/ms 48.18, stage/render/render 24.51, stage/main/update 22.61, render/graph/ms 22.35, stage/render/graph_submit 18.18 |
| warm-up | 4.52 | 89.4 | pipelines/compile_ms 872, pipelines/front_ms 832, pipelines/wait_ms 311, render/thread/ms 129, render/busy/ms 127, main/busy/ms 62.62, pipelines/driver_ms 39.41, stage/render/prepare_resources 34.46, stage/render/prepare_assets 33.19, stage/render/render 28.92, render/graph/ms 26.49, stage/main/update 23.04 |
| warm-up | 4.63 | 110.9 | pipelines/compile_ms 2523, pipelines/front_ms 2433, pipelines/wait_ms 1195, render/thread/ms 180, render/busy/ms 178, main/busy/ms 108, pipelines/driver_ms 89.25, encode//sum 85.96, stage/main/post_update 70.14, stage/render/prepare_resources 67.95, stage/render/prepare_assets 34.93, stage/render/render 31.63 |
| warm-up | 4.68 | 58.3 | pipelines/compile_ms 3060, pipelines/front_ms 2959, pipelines/wait_ms 1159, render/thread/ms 151, render/busy/ms 149, encode//sum 105, pipelines/driver_ms 101, main/busy/ms 95.90, stage/main/post_update 69.18, stage/render/render 39.51, render/graph/ms 36.71, stage/render/prepare_resources 35.67 |
| warm-up | 4.75 | 67.7 | pipelines/compile_ms 1056, pipelines/front_ms 1026, pipelines/wait_ms 313, main/busy/ms 92.21, render/thread/ms 87.77, render/busy/ms 86.08, stage/main/update 67.61, stream/drive/ms 58.33, stage/render/prepare_assets 40.35, encode//sum 34.15, pipelines/driver_ms 29.54, stage/render/render 23.54 |
| warm-up | 4.82 | 71.6 | pipelines/compile_ms 1627, pipelines/front_ms 1569, pipelines/wait_ms 607, main/busy/ms 111, stage/main/update 88.75, render/thread/ms 74.10, render/busy/ms 72.24, pipelines/driver_ms 58.88, stream/drive/ms 58.49, stage/render/prepare_assets 32.74, stage/render/render 19.23, render/graph/ms 15.81 |
| warm-up | 4.87 | 49.3 | pipelines/compile_ms 2180, pipelines/front_ms 2110, pipelines/wait_ms 725, main/busy/ms 122, stage/main/update 101, pipelines/driver_ms 70.09, render/thread/ms 64.27, render/busy/ms 62.29, stream/drive/ms 53.68, stage/render/prepare_assets 23.77, stage/render/render 22.13, render/graph/ms 18.89 |
| warm-up | 4.98 | 111.8 | pipelines/compile_ms 1089, pipelines/front_ms 1062, pipelines/wait_ms 243, main/busy/ms 112, stage/main/update 88.28, render/thread/ms 79.18, render/busy/ms 77.72, stream/drive/ms 53.50, stage/render/prepare_assets 44.17, pipelines/driver_ms 27.77, stage/render/render 15.55, render/graph/ms 13.35 |
| warm-up | 5.03 | 44.2 | pipelines/compile_ms 935, pipelines/front_ms 905, main/busy/ms 93.81, render/thread/ms 83.59, render/busy/ms 82.22, stage/main/update 68.46, stream/drive/ms 45.93, stage/render/prepare_assets 37.94, pipelines/driver_ms 29.51, stage/render/render 17.99, render/graph/ms 15.86, stage/main/post_update 15.61 |
| warm-up | 5.10 | 67.5 | pipelines/compile_ms 1596, pipelines/front_ms 1553, main/busy/ms 81.08, stage/main/update 63.88, render/thread/ms 60.32, render/busy/ms 59.02, stream/drive/ms 45.93, pipelines/driver_ms 43.27, stage/render/prepare_assets 25.76, stage/render/render 16.02, render/graph/ms 14.06, encode//sum 11.59 |
| warm-up | 5.16 | 46.7 | pipelines/compile_ms 456, pipelines/front_ms 440, main/busy/ms 53.28, stage/main/update 39.56, render/thread/ms 38.43, render/busy/ms 36.75, stream/drive/ms 27.65, pipelines/driver_ms 15.97, stage/render/render 13.22, stage/render/prepare_assets 11.42, render/graph/ms 10.98, pipelines/wait_ms 10.42 |
| warm-up | 5.21 | 44.6 | pipelines/compile_ms 222, pipelines/front_ms 213, render/thread/ms 60.42, render/busy/ms 55.06, main/busy/ms 50.28, stage/main/update 28.69, stream/drive/ms 18.52, stream/activate/ms 13.89, stage/render/render 13.59, stage/main/post_update 13.17, stage/render/prepare_assets 12.96, render/graph/ms 11.50 |
| warm-up | 5.25 | 44.5 | render/thread/ms 74.91, render/busy/ms 69.56, main/busy/ms 57.75, pipelines/compile_ms 41.46, pipelines/front_ms 38.44, stage/main/update 28.04, stage/main/post_update 20.91, stage/render/prepare_assets 18.86, stream/drive/ms 17.78, stage/render/render 13.63, stream/activate/ms 13.58, encode//sum 11.91 |
| warm-up | 5.28 | 28.6 | pipelines/compile_ms 543, pipelines/front_ms 517, render/thread/ms 62.61, render/busy/ms 61.30, main/busy/ms 37.92, pipelines/driver_ms 25.79, stage/main/post_update 19.49, stage/render/prepare_assets 17.24, stage/main/update 12.87, stage/render/render 11.60, render/graph/ms 9.89, encode//sum 9.42 |
| warm-up | 5.32 | 35.6 | pipelines/compile_ms 942, pipelines/front_ms 905, render/thread/ms 58.77, render/busy/ms 57.26, pipelines/driver_ms 36.78, main/busy/ms 36.02, stage/main/post_update 16.80, stage/render/render 15.06, stage/main/update 13.01, render/graph/ms 12.28, stage/render/graph_submit 9.30, stage/render/prepare_assets 8.75 |
| warm-up | 5.39 | 43.3 | pipelines/compile_ms 1754, pipelines/front_ms 1711, render/thread/ms 57.55, render/busy/ms 55.78, pipelines/driver_ms 43.21, main/busy/ms 38.82, stage/render/prepare_assets 25.35, stage/render/render 16.78, stage/main/update 16.46, render/graph/ms 14.29, stage/main/post_update 12.99, stage/render/graph_submit 10.78 |
| warm-up | 5.43 | 38.4 | pipelines/compile_ms 1764, pipelines/front_ms 1718, render/thread/ms 64.61, render/busy/ms 61.20, pipelines/driver_ms 45.79, main/busy/ms 41.01, stage/render/prepare_assets 38.86, stage/main/update 23.75, stage/render/render 14.44, stream/drive/ms 13.64, stage/main/post_update 11.37, render/graph/ms 10.72 |
| warm-up | 5.46 | 34.1 | pipelines/compile_ms 249, pipelines/front_ms 234, render/thread/ms 58.31, render/busy/ms 54.78, main/busy/ms 43.50, stage/render/prepare_assets 31.49, stage/main/update 23.55, pipelines/driver_ms 15.28, stage/render/render 13.57, stream/drive/ms 12.49, stage/main/post_update 11.28, render/graph/ms 9.71 |
| warm-up | 6.11 | 29.9 | pipelines/compile_ms 389, pipelines/front_ms 377, main/busy/ms 41.46, render/thread/ms 28.25, render/busy/ms 26.96, stage/main/update 25.95, stream/drive/ms 19.00, stage/render/render 12.56, pipelines/driver_ms 11.33, render/graph/ms 10.62, stage/main/post_update 9.37, encode//sum 8.59 |
| warm-up | 6.22 | 28.7 | pipelines/compile_ms 549, pipelines/front_ms 528, render/thread/ms 44.18, render/busy/ms 42.42, main/busy/ms 36.92, pipelines/driver_ms 21.64, stage/main/post_update 14.67, stage/render/render 14.32, stage/main/update 12.84, render/graph/ms 11.86, stage/render/prepare_assets 9.91, stage/render/graph_submit 8.41 |
| warm-up | 6.26 | 34.6 | pipelines/compile_ms 1323, pipelines/front_ms 1287, render/thread/ms 47.55, render/busy/ms 46.38, pipelines/driver_ms 36.50, main/busy/ms 33.36, stage/render/prepare_assets 19.48, stage/main/update 13.71, stage/main/post_update 10.98, pipelines/wait_ms 10.53, stage/render/render 9.60, pipelines/lock_ms 9.44 |
| warm-up | 6.45 | 30.1 | main/busy/ms 34.65, render/thread/ms 33.13, render/busy/ms 31.90, stage/main/update 18.35, stage/render/render 12.20, stage/main/post_update 11.43, render/graph/ms 10.52, stage/render/graph_submit 8.07, encode//sum 7.30, encode/submit 5.80, stream/drive/ms 5.40, stage/render/prepare_assets 5.23 |
| warm-up | 6.48 | 28.7 | pipelines/compile_ms 131, pipelines/front_ms 118, render/thread/ms 44.79, render/busy/ms 43.20, main/busy/ms 41.22, stage/main/update 17.07, stage/main/post_update 15.80, stage/render/render 13.86, pipelines/driver_ms 12.98, render/graph/ms 11.52, encode//sum 10.18, stage/render/prepare_assets 9.87 |
| warm-up | 6.61 | 31.8 | pipelines/compile_ms 159, pipelines/front_ms 155, render/thread/ms 36.62, render/busy/ms 35.58, main/busy/ms 33.53, stage/main/update 16.60, stage/render/prepare_assets 15.68, stage/main/post_update 11.42, stage/render/render 10.30, encode//sum 9.78, render/graph/ms 8.45, stage/render/graph_submit 5.36 |
| warm-up | 6.99 | 38.1 | main/busy/ms 41.56, render/thread/ms 29.39, render/busy/ms 28.36, stage/main/update 23.01, stage/render/prepare_assets 11.95, stage/main/post_update 10.47, stage/render/render 9.44, render/graph/ms 7.91, encode//sum 7.30, stage/render/graph_submit 5.57, stream/drive/ms 3.82, encode/submit 2.86 |
| warm-up | 7.07 | 33.8 | render/thread/ms 43.91, render/busy/ms 42.93, main/busy/ms 21.91, stage/render/render 11.20, stage/render/specialize 10.26, stage/main/post_update 9.79, render/graph/ms 9.77, stage/main/update 8.49, stage/render/graph_submit 7.44, encode//sum 7.31, stage/render/queue 7.21, stage/render/prepare_assets 5.94 |
| warm-up | 8.27 | 30.9 | render/thread/ms 34.64, render/busy/ms 33.75, main/busy/ms 17.02, stage/main/post_update 10.04, stage/render/render 9.50, stage/render/specialize 9.11, render/graph/ms 8.23, encode//sum 6.41, stage/render/graph_submit 6.31, stage/render/queue 6.22, stage/main/update 4.73, stage/render/prepare_meshes 3.42 |
| measure | 1.65 | 32.3 | render/thread/ms 36.82, render/busy/ms 35.81, main/busy/ms 23.47, stage/main/post_update 13.47, stage/render/specialize 10.19, stage/render/render 10.17, render/graph/ms 8.83, encode//sum 6.82, stage/render/graph_submit 6.74, stage/render/queue 6.54, stage/main/update 6.15, encode/submit 4.11 |
| measure | 3.02 | 28.8 | render/thread/ms 36.35, render/busy/ms 35.33, main/busy/ms 18.02, stage/main/post_update 11.72, stage/render/render 11.52, render/graph/ms 10.15, encode//sum 8.89, stage/render/specialize 8.77, stage/render/graph_submit 7.36, stage/render/queue 5.83, encode/submit 4.69, encode/pass:shadow_point_light/sum 4.48 |
| measure | 9.46 | 31.0 | render/thread/ms 36.87, render/busy/ms 35.25, main/busy/ms 19.23, stage/render/render 10.99, stage/main/post_update 10.82, stage/render/specialize 10.16, render/graph/ms 8.99, encode//sum 7.67, stage/render/graph_submit 6.74, stage/render/queue 6.38, stage/main/update 5.22, encode/submit 4.00 |
| measure | 15.82 | 33.1 | render/thread/ms 37.35, render/busy/ms 36.27, main/busy/ms 17.52, stage/main/post_update 10.98, stage/render/render 10.13, stage/render/specialize 9.75, stage/render/queue 8.79, render/graph/ms 8.69, encode//sum 7.03, stage/render/graph_submit 6.70, stage/main/update 3.99, encode/submit 3.44 |
| measure | 18.01 | 29.5 | render/thread/ms 36.75, render/busy/ms 35.63, main/busy/ms 16.63, stage/main/post_update 10.51, stage/render/render 10.23, stage/render/specialize 9.99, render/graph/ms 8.68, encode//sum 7.32, stage/render/graph_submit 6.46, stage/render/queue 6.42, encode/submit 3.67, stage/main/update 3.55 |
| measure | 18.29 | 30.0 | main/busy/ms 37.73, stage/main/update 23.82, stream/drive/ms 19.99, render/thread/ms 18.88, render/busy/ms 17.84, stage/main/post_update 10.84, stage/render/render 9.94, render/graph/ms 8.53, encode//sum 8.16, stage/render/graph_submit 6.40, encode/submit 3.82, encode/finish_wall 2.63 |
| measure | 19.33 | 29.8 | render/thread/ms 35.11, render/busy/ms 34.06, main/busy/ms 17.37, encode//sum 11.41, stage/main/post_update 11.05, stage/render/render 10.03, stage/render/specialize 9.36, render/graph/ms 8.59, stage/render/queue 6.72, stage/render/graph_submit 6.69, encode/submit 3.76, stage/main/update 3.53 |
| measure | 20.65 | 29.3 | render/thread/ms 35.58, render/busy/ms 34.64, main/busy/ms 17.77, stage/main/post_update 10.96, stage/render/render 10.12, stage/render/specialize 9.57, render/graph/ms 8.80, encode//sum 7.25, stage/render/queue 6.87, stage/render/graph_submit 6.71, stage/main/update 4.21, encode/submit 3.97 |
| measure | 23.02 | 33.6 | render/thread/ms 44.11, render/busy/ms 42.95, main/busy/ms 23.43, stage/render/render 13.86, render/graph/ms 12.31, stage/main/post_update 11.99, stage/render/specialize 11.83, stage/render/graph_submit 9.69, encode//sum 9.34, stage/render/queue 7.83, stage/main/update 5.91, encode/pass:shadow_point_light/sum 5.43 |
| measure | 24.52 | 29.7 | render/thread/ms 39.07, render/busy/ms 38.07, main/busy/ms 21.95, stage/render/render 12.69, stage/main/post_update 12.03, render/graph/ms 11.25, stage/render/specialize 9.83, encode//sum 9.65, stage/render/graph_submit 8.68, encode/pass:shadow_point_light/sum 6.86, stage/render/queue 6.49, encode/submit 5.28 |
| measure | 27.27 | 35.1 | render/thread/ms 41.37, render/busy/ms 40.30, main/busy/ms 24.74, stage/render/render 13.68, stage/main/post_update 12.79, render/graph/ms 12.22, stage/render/specialize 11.27, encode//sum 9.32, stage/render/graph_submit 9.20, encode/pass:shadow_point_light/sum 7.71, stage/render/queue 7.22, encode/submit 5.74 |
| measure | 28.82 | 31.7 | render/thread/ms 43.38, render/busy/ms 42.37, main/busy/ms 19.26, stage/render/render 15.61, render/graph/ms 14.12, encode//sum 12.48, stage/main/post_update 12.28, stage/render/graph_submit 10.23, stage/render/specialize 9.92, encode/pass:shadow_point_light/sum 8.30, stage/render/queue 7.27, encode/submit 6.46 |
| measure | 30.61 | 31.8 | render/thread/ms 41.69, render/busy/ms 40.61, main/busy/ms 20.63, stage/render/render 14.27, render/graph/ms 12.78, stage/main/post_update 12.53, stage/render/specialize 10.47, encode//sum 10.10, stage/render/graph_submit 9.61, encode/pass:shadow_point_light/sum 8.80, stage/render/queue 7.04, encode/submit 6.07 |
| measure | 32.04 | 33.2 | render/thread/ms 37.26, render/busy/ms 36.12, main/busy/ms 17.84, stage/main/post_update 10.97, stage/render/render 10.78, stage/render/specialize 9.67, render/graph/ms 9.23, encode//sum 8.48, stage/render/queue 6.96, stage/render/graph_submit 6.91, encode/submit 4.33, stage/main/update 3.90 |
| measure | 35.96 | 30.9 | render/thread/ms 35.73, render/busy/ms 34.57, main/busy/ms 21.68, stage/main/post_update 11.61, stage/render/specialize 10.05, stage/render/render 9.98, render/graph/ms 8.48, encode//sum 7.11, stage/render/queue 6.57, stage/render/graph_submit 6.45, stage/main/update 4.58, encode/submit 3.65 |
| measure | 37.48 | 29.7 | render/thread/ms 37.16, render/busy/ms 36.03, main/busy/ms 17.74, stage/main/post_update 11.64, stage/render/render 11.59, render/graph/ms 9.94, stage/render/specialize 9.26, encode//sum 7.70, stage/render/graph_submit 7.61, stage/render/queue 6.34, encode/submit 4.59, stage/main/update 3.57 |
| measure | 41.14 | 32.9 | render/thread/ms 39.13, render/busy/ms 37.99, main/busy/ms 18.47, stage/render/render 11.08, stage/main/post_update 10.67, stage/render/specialize 10.25, render/graph/ms 9.57, stage/render/graph_submit 7.86, encode//sum 6.97, stage/render/queue 6.41, stage/main/update 4.35, encode/submit 4.08 |
| measure | 43.85 | 30.6 | render/thread/ms 37.12, render/busy/ms 36.03, main/busy/ms 17.20, stage/render/render 11.08, stage/main/post_update 9.93, render/graph/ms 9.59, stage/render/specialize 9.55, encode//sum 8.14, stage/render/graph_submit 7.62, stage/render/queue 6.41, stage/main/update 4.04, encode/finish_wall 4.01 |
| measure | 46.14 | 35.0 | render/thread/ms 37.71, render/busy/ms 36.75, main/busy/ms 17.65, stage/main/post_update 10.46, stage/render/specialize 10.34, stage/render/render 10.10, render/graph/ms 8.67, encode//sum 7.47, stage/render/graph_submit 6.71, stage/render/queue 6.42, stage/render/prepare_meshes 4.57, stage/main/update 4.29 |
| measure | 56.43 | 29.6 | render/thread/ms 34.33, render/busy/ms 33.31, main/busy/ms 15.13, stage/render/render 10.25, render/graph/ms 8.94, stage/main/post_update 8.72, stage/render/specialize 8.72, stage/render/graph_submit 6.88, encode//sum 6.49, stage/render/queue 6.32, encode/submit 4.35, stage/main/update 3.91 |
| measure | 59.22 | 44.7 | render/thread/ms 56.58, render/busy/ms 55.07, main/busy/ms 25.95, stage/render/render 16.59, render/graph/ms 14.75, stage/main/post_update 14.04, stage/render/specialize 12.90, stage/render/graph_submit 11.79, encode//sum 9.68, stage/main/update 8.21, stage/render/prepare_meshes 7.84, encode/submit 7.05 |
| measure | 59.28 | 30.2 | render/thread/ms 35.86, render/busy/ms 34.87, stage/render/prepare_assets 19.32, main/busy/ms 18.26, stage/main/post_update 9.56, stage/render/render 8.76, render/graph/ms 7.57, encode//sum 6.99, stage/render/graph_submit 5.59, stage/main/update 4.45, encode/submit 3.64, encode/finish_wall 1.97 |
| measure | 63.20 | 34.2 | render/thread/ms 37.45, render/busy/ms 36.34, main/busy/ms 19.61, stage/render/render 10.70, stage/render/specialize 10.39, stage/main/post_update 9.90, render/graph/ms 9.30, encode//sum 7.40, stage/render/graph_submit 7.16, stage/main/update 7.00, stage/render/queue 6.33, stage/render/prepare_meshes 4.30 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set select.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=select-5 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 10%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **81%** (min 56%) · 2624 MHz (min 2610) · memory 8101 MHz · 82.4 W · 77 °C (max 82) · VRAM 7058 MiB · throttled 100% of samples

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
- `select.gate=on (command line: --set)`
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
