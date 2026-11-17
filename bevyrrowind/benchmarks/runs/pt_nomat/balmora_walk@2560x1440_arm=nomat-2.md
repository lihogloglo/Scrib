# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.depth_only.skip_material=on (command line: --set); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6097 frames over 65.6 s · **median 10.48 ms (95 fps)** · mean 10.77 · p95 13.20 · p99 16.40 (1% low 61 fps) · worst 42.11 ms · 77 spikes over 1.5× median · 60.3 stutters/min (work 52.1/min)
- warm-up: 325 frames over 9.4 s, worst 2619.2 ms
- hitches (> 26.2 ms): **34** measured, 55 in warm-up
- timed GPU passes: 6.68 ms of a 10.77 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.48 ms, limit 11 ms
- **FAIL** frame_ms: 66 stutters (> 1.5x moving median of 60) in 1.09 min = 60.32/min, limit 1/min
- **FAIL** worst frame 42.11 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.895 | 2.035 | 2.067 | 0.93 | 0.176 |
| main_opaque_pass_3d | 1.515 | 1.627 | 2.035 | 0.93 | 0.063 |
| early prepass | 0.457 | 0.490 | 0.713 | 0.93 | 0.086 |
| bloom | 0.293 | 0.315 | 0.321 | 0.93 | 0.061 |
| atmosphere_luts | 0.284 | 0.304 | 0.366 | 0.93 | 0.003 |
| water_velocity | 0.259 | 0.278 | 0.348 | 0.93 | 0.001 |
| water_sspr | 0.188 | 0.202 | 0.203 | 0.93 | 0.002 |
| ssao | 0.175 | 0.188 | 0.191 | 0.93 | 0.007 |
| atmos_froxel_inject | 0.135 | 0.145 | 0.147 | 0.93 | 0.002 |
| clouds_trace | 0.130 | 0.140 | 0.233 | 0.93 | 0.001 |
| auxiliary_views | 0.126 | 0.135 | 0.502 | 0.93 | 0.133 |
| uniform_allocation | 0.121 | 0.130 | 0.181 | 0.93 | 0.025 |
| water_fft | 0.120 | 0.129 | 0.136 | 0.93 | 0.003 |
| upscaling | 0.109 | 0.117 | 0.125 | 0.93 | 0.004 |
| clouds_composite | 0.098 | 0.106 | 0.108 | 0.93 | 0.002 |
| bin_unpacking | 0.097 | 0.104 | 0.142 | 0.93 | 0.018 |
| tonemapping | 0.079 | 0.085 | 0.088 | 0.93 | 0.004 |
| main_transmissive_pass_3d | 0.079 | 0.085 | 0.177 | 0.93 | 0.008 |
| clustering | 0.060 | 0.064 | 0.067 | 0.93 | 0.018 |
| render_sky | 0.048 | 0.051 | 0.052 | 0.93 | 0.002 |
| clouds_resolve | 0.045 | 0.049 | 0.049 | 0.93 | 0.001 |
| lightprobe_irradiance_map | 0.043 | 0.415 | 0.415 | 0.10 | 0.001 |
| surface_exposure | 0.041 | 0.044 | 0.045 | 0.93 | 0.001 |
| surface_trail | 0.037 | 0.056 | 0.057 | 0.65 | 0.002 |
| surface_exposure_far | 0.035 | 0.043 | 0.045 | 0.82 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.93 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.047 | 0.93 | 0.005 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.032 | 0.93 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.93 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.219 | 0.229 | 0.10 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.93 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.027 | 0.93 | 0.123 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.93 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.93 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.93 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.93 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.002 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.234 | 11.282 | 38.08 | 56302.2 |
| main/busy/ms | 8.954 | 10.570 | 28.68 | 54593.6 |
| render/busy/ms | 8.700 | 10.781 | 37.50 | 53046.4 |
| stage/main/post_update | 5.537 | 6.848 | 18.41 | 33757.1 |
| stage/render/render | 5.340 | 6.692 | 17.49 | 32556.9 |
| render/graph/ms | 4.615 | 5.948 | 16.87 | 28138.6 |
| encode//sum | 3.907 | 5.250 | 19.05 | 23818.1 |
| stage/render/graph_submit | 3.449 | 4.513 | 15.32 | 21029.9 |
| encode/submit | 2.193 | 3.047 | 6.39 | 13369.5 |
| stage/main/update | 2.165 | 2.568 | 22.54 | 13197.8 |
| encode/finish_wall | 1.313 | 1.906 | 12.38 | 8007.5 |
| stage/render/camera_driver | 1.113 | 1.532 | 4.52 | 6788.9 |
| encode/pass:shadow_directional_light/sum | 0.813 | 1.478 | 2.54 | 4956.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.785 | 1.071 | 1.65 | 4786.1 |
| encode/pass:early prepass/sum | 0.767 | 1.045 | 11.68 | 4678.8 |
| stage/render/prepare_write_phase_buffers | 0.764 | 1.072 | 2.72 | 4656.2 |
| stage/render/prepare_assets | 0.566 | 0.727 | 11.12 | 3451.6 |
| bigspace/total/ms | 0.544 | 1.281 | 2.22 | 3317.3 |
| stage/render/prepare_resources | 0.457 | 0.549 | 0.90 | 2787.0 |
| render/present/ms | 0.439 | 0.635 | 2.30 | 2674.1 |
| stage/render/prepare_meshes | 0.435 | 0.546 | 5.87 | 2653.7 |
| stage/main/first | 0.396 | 0.520 | 5.16 | 2411.6 |
| stage/main/pre_update | 0.354 | 0.513 | 2.83 | 2157.7 |
| stage/render/specialize | 0.315 | 0.408 | 12.72 | 1920.6 |
| terrain/update/ms | 0.308 | 0.395 | 1.94 | 1875.8 |
| stage/render/queue | 0.286 | 0.413 | 9.32 | 1744.4 |
| stage/render/prepare_bind_groups | 0.277 | 0.487 | 2.18 | 1691.8 |
| terrain/select/ms | 0.274 | 0.354 | 1.05 | 1669.5 |
| encode/pass:shadow_point_light/sum | 0.269 | 1.512 | 7.63 | 1638.2 |
| controller/feed/ms | 0.268 | 0.337 | 0.71 | 1636.6 |
| stream/drive/ms | 0.249 | 0.334 | 20.51 | 1518.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.239 | 0.350 | 1.29 | 1455.0 |
| bigspace/low_precision/ms | 0.230 | 0.738 | 1.83 | 1402.2 |
| stage/main/fixed_main_loop | 0.214 | 0.374 | 0.98 | 1306.8 |
| far/select/ms | 0.201 | 0.272 | 0.59 | 1228.1 |
| stage/render/create_views | 0.198 | 0.257 | 0.86 | 1205.1 |
| stream/tick/ms | 0.192 | 0.264 | 0.54 | 1173.4 |
| stage/render/prepare_batch_phases | 0.189 | 0.272 | 0.62 | 1154.8 |
| stage/render/extract_commands | 0.171 | 0.202 | 0.86 | 1045.4 |
| stage/render/prepare_flush | 0.160 | 0.293 | 1.09 | 975.8 |
| anim/evaluate/ms | 0.159 | 0.272 | 1.46 | 971.0 |
| bigspace/high_precision/ms | 0.153 | 0.644 | 1.73 | 934.2 |
| anim/evaluate_cpu/ms | 0.151 | 0.186 | 0.68 | 920.0 |
| controller/move/ms | 0.148 | 0.349 | 0.92 | 901.0 |
| stage/render/prepare_views | 0.139 | 0.172 | 0.45 | 847.0 |
| stage/main/last | 0.113 | 0.154 | 1.49 | 689.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.099 | 0.162 | 0.74 | 601.1 |
| render/acquire/ms | 0.095 | 0.121 | 0.40 | 581.7 |
| stage/render/prepare_collect_phase_buffers | 0.063 | 0.132 | 0.31 | 383.7 |
| pipelines/compile_ms | 0.047 | 0.000 | 122.04 | 286.8 |
| physics/step/ms | 0.047 | 0.088 | 0.61 | 285.2 |
| pipelines/front_ms | 0.045 | 0.000 | 117.74 | 273.8 |
| physics/insert/ms | 0.033 | 0.357 | 1.03 | 198.6 |
| stage/render/cleanup | 0.032 | 0.099 | 0.69 | 197.5 |
| stage/render/phase_sort | 0.032 | 0.055 | 0.50 | 197.4 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.28 | 183.8 |
| weather/map/ms | 0.024 | 0.032 | 0.07 | 149.0 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.52 | 89.4 |
| encode/camera_driver_span_end/sum | 0.012 | 0.019 | 0.54 | 70.4 |
| anim/apply/ms | 0.011 | 0.020 | 0.43 | 69.1 |
| controller/avoid/ms | 0.011 | 0.014 | 0.09 | 64.5 |
| weather/eval/ms | 0.010 | 0.014 | 0.20 | 63.4 |
| script/update/ms | 0.007 | 0.009 | 0.19 | 42.6 |
| script/input/ms | 0.005 | 0.006 | 0.16 | 31.2 |
| stream/despawn/ms | 0.005 | 0.000 | 0.97 | 30.6 |
| script/ui/ms | 0.005 | 0.006 | 0.21 | 28.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 28.5 |
| controller/write/ms | 0.004 | 0.005 | 0.04 | 22.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 13.9 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.90 | 13.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 6.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.22 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 20 | 29 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 15 |
| crowd/c0/joints/count | count | 1301 | 2262 | 707 |
| crowd/c0/meshes/count | count | 353 | 646 | 189 |
| crowd/c1/count | count | 145 | 155 | 145 |
| crowd/c1/joints/count | count | 5531 | 5941 | 5457 |
| crowd/c1/meshes/count | count | 1166 | 1283 | 1144 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4605 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4585 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.41 | 12.30 | 0.63 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.52 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.54 | 0.01 |
| encode/finish_critical | ms | 0.94 | 12.30 | 0.91 |
| encode/pass:early prepass/max | ms | 0.77 | 11.68 | 0.84 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.80 | 1.65 | 0.91 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.74 | 0.15 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.29 | 0.27 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 2.54 | 0.80 |
| encode/pass:shadow_point_light/max | ms | 0.12 | 1.22 | 0.88 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.28 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.11 | 37.50 | 8.34 |
| gpu/timed/ms | ms | 7.91 | 9.14 | 8.19 |
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
| phases/alpha_mask3d/items | count | 820 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 820 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1292 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3423 | 5142 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1292 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3423 | 5142 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 15 | 199 | 4 |
| phases/shadow/bins | count | 4992 | 8902 | 2421 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6368 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 201 | 246 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 186 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 323 | 324 | 324 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 555 | 6 |
| preprocess/bind_groups_created | count | 13 | 555 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 199 | 123 |
| process/cpu_usage | % | 26.32 | 42.79 | 29.49 |
| process/mem_usage | GiB | 5.52 | 5.69 | 5.69 |
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
| skin/joint_bytes_written/bytes | bytes | 369.4 k | 810.2 k | 490.2 k |
| skin/joints_written/count | count | 5772 | 12.7 k | 7660 |
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
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.80 | 48.80 | 31.93 |
| system/mem_usage | % | 61.32 | 61.92 | 61.82 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 64 | 246 | 54 |
| views/all/bins | count | 7666 | 11.8 k | 5919 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 246 | 293 | 75 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 34.7 k | 8119 |
| views/cascade/batch_sets | count | 9 | 54 | 4 |
| views/cascade/bins | count | 2801 | 6187 | 2421 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8908 | 23.5 k | 6368 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 413 | 22.9 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 191 | 0 |
| views/lamp_face/bins | count | 2123 | 2798 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2380 | 3108 | 0 |
| views/lamp_face/pipelines | count | 164 | 203 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2913 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2874 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8484 | 13.1 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8642 | 13.3 k | 7918 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.9 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 56.42 | 313 | 344.0 k |
| anim/events/count | count | 0.09 | 3 | 523 |
| assets/Image/added | count | 0.10 | 40 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.79 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.55 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1296 | 4676 |
| encode//count | count | 33.28 | 220 | 202.9 k |
| encode/bind_walk/init_textures | count | 9808 | 41.0 k | 59.80 M |
| encode/bind_walk/init_textures_skipped | count | 46.8 k | 65.7 k | 285.61 M |
| encode/bind_walk/inits | count | 455 | 1086 | 2.77 M |
| encode/bind_walk/inits_skipped | count | 100 | 149 | 610.9 k |
| encode/bind_walk/merge_textures | count | 9736 | 40.9 k | 59.36 M |
| encode/bind_walk/merge_textures_skipped | count | 7844 | 18.7 k | 47.83 M |
| encode/bind_walk/merges | count | 97.09 | 301 | 592.0 k |
| encode/bind_walk/merges_skipped | count | 89.42 | 142 | 545.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6097 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6097 |
| encode/pass:early prepass/count | count | 1 | 1 | 6097 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6097 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6097 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6097 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6097 |
| encode/pass:shadow_point_light/count | count | 1.33 | 48 | 8134 |
| encode/queue/command_buffers | count | 44.63 | 278 | 272.1 k |
| encode/queue/submits | count | 2.02 | 4 | 12.3 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.77 M | 11.47 G |
| encode/queue/write_buffer/calls | count | 256 | 829 | 1.56 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.5 k | 3.52 M | 1.08 G |
| encode/queue/write_buffer_with/calls | count | 10.98 | 465 | 67.0 k |
| encode/queue/write_texture/bytes | bytes | 11.7 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6097 |
| physics/inserts | count | 0.37 | 110 | 2267 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 161 | 2335 |
| physics/steps | count | 0.65 | 2 | 3939 |
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
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5552 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1217 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.2 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.62 | 2619.2 | main/busy/ms 2589, stage/main/post_update 2479, stage/main/update 100, surface/advance_cpu_ms 68.90, stage/main/pre_update 5.80, stage/main/last 1.05, surface/cell_upload_ms 0.76, stage/main/first 0.61, stage/main/fixed_main_loop 0.38, stream/drive/ms 0.36, weather/map/ms 0.13, bigspace/total/ms 0.10 |
| warm-up | 3.36 | 737.3 | main/busy/ms 2735, stage/main/post_update 2506, render/thread/ms 705, render/busy/ms 547, stage/render/prepare_assets 497, stage/main/update 214, stage/render/prepare_views 155, render/acquire/ms 155, surface/advance_cpu_ms 68.90, stage/render/create_views 28.74, stage/render/render 13.61, render/graph/ms 8.52 |
| warm-up | 3.49 | 130.0 | render/thread/ms 818, render/busy/ms 595, stage/render/prepare_assets 522, main/busy/ms 171, stage/render/prepare_views 156, render/acquire/ms 156, pipelines/compile_ms 126, stage/main/update 119, stage/render/render 88.59, render/present/ms 66.83, pipelines/driver_ms 63.87, pipelines/front_ms 62.06 |
| warm-up | 3.67 | 160.4 | pipelines/wait_ms 5237, pipelines/compile_ms 2092, pipelines/front_ms 1892, pipelines/driver_ms 199, render/thread/ms 138, render/busy/ms 134, main/busy/ms 94.86, stage/render/render 49.73, render/graph/ms 45.67, stage/main/update 42.80, stage/render/graph_submit 35.61, encode/finish_wall 31.59 |
| warm-up | 3.72 | 40.8 | pipelines/wait_ms 5374, pipelines/compile_ms 1921, pipelines/front_ms 1772, render/thread/ms 152, render/busy/ms 150, pipelines/driver_ms 150, main/busy/ms 118, stage/main/update 61.99, stage/render/render 48.24, render/graph/ms 45.88, stage/render/graph_submit 43.56, encode/finish_wall 40.36 |
| warm-up | 3.83 | 111.7 | pipelines/compile_ms 1229, pipelines/front_ms 1174, pipelines/wait_ms 475, render/thread/ms 109, render/busy/ms 107, main/busy/ms 84.79, stage/render/prepare_assets 78.44, stage/main/update 56.33, pipelines/driver_ms 54.81, stage/render/render 17.81, stage/main/post_update 17.43, render/graph/ms 15.57 |
| warm-up | 3.88 | 50.6 | pipelines/compile_ms 2266, pipelines/front_ms 1965, pipelines/wait_ms 341, pipelines/driver_ms 301, render/thread/ms 122, render/busy/ms 120, stage/render/prepare_assets 68.82, stage/render/render 43.59, render/graph/ms 41.17, stage/render/camera_driver 35.47, main/busy/ms 34.03, encode//sum 18.60 |
| warm-up | 3.92 | 38.3 | pipelines/compile_ms 1532, pipelines/front_ms 1246, pipelines/driver_ms 286, render/thread/ms 86.17, render/busy/ms 72.86, stage/render/render 58.60, render/graph/ms 54.78, stage/render/camera_driver 35.83, main/busy/ms 31.17, encode//sum 29.82, pipelines/wait_ms 19.86, stage/render/graph_submit 18.77 |
| warm-up | 3.96 | 31.0 | pipelines/compile_ms 87.25, pipelines/front_ms 83.03, render/thread/ms 36.08, render/busy/ms 34.76, main/busy/ms 24.41, stage/render/prepare_assets 18.33, stage/main/post_update 15.01, stage/render/render 10.07, terrain/update/ms 8.87, render/graph/ms 8.13, encode//sum 7.81, stage/render/graph_submit 5.70 |
| warm-up | 4.06 | 73.2 | pipelines/compile_ms 646, pipelines/front_ms 622, main/busy/ms 58.37, render/thread/ms 55.63, render/busy/ms 52.76, stage/main/update 45.82, stream/drive/ms 40.78, stage/render/prepare_assets 37.55, pipelines/driver_ms 23.75, stage/render/render 11.60, render/graph/ms 8.40, stage/main/post_update 8.27 |
| warm-up | 4.10 | 30.6 | pipelines/compile_ms 250, pipelines/front_ms 237, render/thread/ms 38.71, render/busy/ms 37.14, main/busy/ms 29.34, stage/main/update 16.54, stage/render/prepare_assets 15.45, stage/render/render 14.96, render/graph/ms 12.81, pipelines/driver_ms 12.59, stream/drive/ms 10.81, stage/render/graph_submit 8.81 |
| warm-up | 4.14 | 34.4 | pipelines/compile_ms 484, pipelines/front_ms 462, render/thread/ms 54.31, main/busy/ms 48.67, render/busy/ms 45.12, stage/main/update 31.79, stream/drive/ms 25.15, stage/render/render 24.93, pipelines/driver_ms 22.63, stage/render/prepare_assets 22.40, render/graph/ms 15.08, stage/main/post_update 10.14 |
| warm-up | 4.21 | 70.5 | pipelines/compile_ms 715, pipelines/front_ms 687, render/thread/ms 75.93, render/busy/ms 66.60, main/busy/ms 53.38, stage/render/prepare_assets 45.22, stage/main/update 32.31, pipelines/driver_ms 27.41, stream/drive/ms 26.56, stage/render/render 23.79, render/graph/ms 13.86, stage/main/post_update 13.41 |
| warm-up | 4.25 | 45.7 | pipelines/compile_ms 528, pipelines/front_ms 508, render/thread/ms 76.70, render/busy/ms 74.75, main/busy/ms 60.07, stage/render/prepare_assets 51.24, stage/main/update 39.19, pipelines/driver_ms 20.21, stage/render/render 12.88, stage/main/post_update 11.75, render/graph/ms 9.97, encode//sum 7.56 |
| warm-up | 4.40 | 148.0 | pipelines/compile_ms 1754, pipelines/front_ms 1674, render/thread/ms 123, render/busy/ms 121, pipelines/driver_ms 79.84, main/busy/ms 63.68, stage/render/prepare_assets 58.79, stage/main/update 43.30, stage/render/prepare_bind_groups 35.18, stage/render/render 16.10, render/graph/ms 12.94, stage/main/post_update 12.49 |
| warm-up | 4.47 | 72.3 | pipelines/compile_ms 2384, pipelines/front_ms 2207, pipelines/wait_ms 216, pipelines/driver_ms 176, render/thread/ms 155, render/busy/ms 153, stage/render/prepare_assets 83.43, main/busy/ms 61.39, stage/render/prepare_bind_groups 34.02, stage/main/update 31.17, stage/render/render 17.64, render/graph/ms 15.19 |
| warm-up | 4.55 | 74.2 | pipelines/compile_ms 781, pipelines/front_ms 676, pipelines/wait_ms 368, render/thread/ms 118, render/busy/ms 116, pipelines/driver_ms 105, encode//sum 76.25, main/busy/ms 68.70, stage/render/prepare_assets 64.25, stage/main/update 32.01, stage/render/render 26.73, render/graph/ms 24.52 |
| warm-up | 4.65 | 103.4 | pipelines/compile_ms 2329, pipelines/front_ms 2265, pipelines/wait_ms 1010, render/thread/ms 154, render/busy/ms 152, encode//sum 76.14, stage/render/prepare_assets 64.92, pipelines/driver_ms 64.89, main/busy/ms 61.84, stage/main/post_update 40.48, stage/render/render 26.58, render/graph/ms 24.30 |
| warm-up | 4.70 | 47.0 | pipelines/compile_ms 2876, pipelines/front_ms 2790, pipelines/wait_ms 974, render/thread/ms 144, render/busy/ms 140, pipelines/driver_ms 85.58, main/busy/ms 57.76, encode//sum 53.78, stage/render/prepare_assets 42.81, stage/main/post_update 36.14, stage/render/render 27.86, render/graph/ms 25.37 |
| warm-up | 4.74 | 37.4 | pipelines/compile_ms 658, pipelines/front_ms 631, pipelines/wait_ms 121, render/thread/ms 72.67, render/busy/ms 69.06, encode//sum 50.35, main/busy/ms 38.44, pipelines/driver_ms 26.75, stage/render/render 23.47, stage/render/prepare_assets 21.25, render/graph/ms 21.07, stage/main/update 16.44 |
| warm-up | 4.85 | 81.4 | pipelines/compile_ms 1224, pipelines/front_ms 1184, pipelines/wait_ms 535, main/busy/ms 110, stage/main/update 82.99, stream/drive/ms 60.45, render/thread/ms 47.67, render/busy/ms 44.49, pipelines/driver_ms 39.63, stage/render/prepare_assets 16.98, stage/render/render 15.21, stage/main/post_update 13.97 |
| warm-up | 4.92 | 75.8 | pipelines/compile_ms 2129, pipelines/front_ms 2060, pipelines/wait_ms 779, main/busy/ms 126, stage/main/update 97.50, render/thread/ms 71.46, pipelines/driver_ms 69.48, render/busy/ms 67.63, stream/drive/ms 53.28, stage/render/prepare_assets 31.30, stage/render/render 17.69, render/graph/ms 13.88 |
| warm-up | 5.09 | 167.0 | pipelines/compile_ms 1889, pipelines/front_ms 1833, pipelines/wait_ms 322, render/thread/ms 137, render/busy/ms 135, main/busy/ms 115, stage/main/update 87.37, stage/render/prepare_assets 71.66, pipelines/driver_ms 56.13, stream/drive/ms 51.70, stage/render/render 40.99, render/graph/ms 37.67 |
| warm-up | 5.15 | 58.8 | pipelines/compile_ms 1852, pipelines/front_ms 1747, render/thread/ms 143, render/busy/ms 139, pipelines/driver_ms 105, main/busy/ms 89.20, stage/render/render 67.49, render/graph/ms 61.56, stage/main/update 59.14, stage/render/prepare_assets 55.11, stage/render/graph_submit 52.57, stream/drive/ms 37.86 |
| warm-up | 5.20 | 56.2 | pipelines/compile_ms 1103, pipelines/front_ms 1018, pipelines/driver_ms 84.95, render/thread/ms 83.80, main/busy/ms 80.61, render/busy/ms 78.99, stage/main/update 58.39, stage/render/render 40.84, stream/drive/ms 37.59, render/graph/ms 35.25, stage/render/graph_submit 30.01, encode/submit 29.66 |
| warm-up | 5.28 | 55.8 | pipelines/compile_ms 238, pipelines/front_ms 231, main/busy/ms 63.12, stage/main/update 48.55, render/thread/ms 38.70, stream/drive/ms 37.49, render/busy/ms 37.30, stage/render/render 13.14, stage/render/prepare_assets 10.64, render/graph/ms 10.57, encode//sum 8.13, stage/main/post_update 8.00 |
| warm-up | 5.36 | 29.9 | pipelines/compile_ms 175, pipelines/front_ms 167, render/thread/ms 41.41, render/busy/ms 39.92, main/busy/ms 33.44, stage/main/post_update 14.39, stage/main/update 13.50, stage/render/render 13.32, stage/render/prepare_assets 12.83, render/graph/ms 11.00, encode//sum 8.35, pipelines/driver_ms 7.98 |
| warm-up | 5.39 | 32.9 | pipelines/compile_ms 249, pipelines/front_ms 241, render/thread/ms 53.86, render/busy/ms 51.51, main/busy/ms 35.36, stage/render/render 16.95, stage/main/post_update 15.75, stage/render/prepare_assets 14.15, render/graph/ms 14.05, stage/main/update 13.36, stage/render/graph_submit 10.06, encode//sum 8.93 |
| warm-up | 5.47 | 50.1 | pipelines/compile_ms 302, pipelines/front_ms 291, render/thread/ms 60.34, render/busy/ms 58.99, main/busy/ms 42.03, stage/render/prepare_assets 34.94, stage/main/update 21.64, stage/main/post_update 12.34, pipelines/driver_ms 10.90, stage/render/render 10.89, render/graph/ms 8.86, encode//sum 7.79 |
| warm-up | 5.51 | 42.4 | pipelines/compile_ms 362, pipelines/front_ms 349, render/thread/ms 78.16, render/busy/ms 76.79, main/busy/ms 42.84, stage/render/prepare_assets 32.08, stage/main/update 18.03, stage/render/prepare_resources 13.46, stage/main/post_update 12.83, stage/render/render 12.59, pipelines/driver_ms 12.15, render/graph/ms 10.49 |
| warm-up | 5.56 | 46.9 | pipelines/compile_ms 1080, pipelines/front_ms 1048, render/thread/ms 85.24, render/busy/ms 81.48, main/busy/ms 64.33, pipelines/driver_ms 31.73, pipelines/wait_ms 29.46, stage/main/update 21.10, stage/main/post_update 19.62, stage/render/prepare_bind_groups 18.10, stage/render/render 16.62, stage/main/pre_update 16.05 |
| warm-up | 5.59 | 30.4 | pipelines/compile_ms 841, pipelines/front_ms 817, render/thread/ms 71.13, render/busy/ms 67.43, main/busy/ms 59.94, pipelines/wait_ms 29.01, pipelines/driver_ms 23.69, stage/main/update 23.05, stage/render/prepare_assets 19.56, stage/main/post_update 18.40, stage/render/prepare_bind_groups 17.81, stage/render/render 15.73 |
| warm-up | 5.62 | 36.0 | pipelines/compile_ms 399, pipelines/front_ms 389, render/thread/ms 53.69, render/busy/ms 52.41, main/busy/ms 35.02, stage/render/prepare_assets 27.62, stage/main/update 16.50, stage/main/post_update 11.69, stage/render/render 11.08, pipelines/driver_ms 9.80, render/graph/ms 8.94, encode//sum 8.45 |
| warm-up | 5.65 | 32.0 | pipelines/compile_ms 611, pipelines/front_ms 598, render/thread/ms 57.13, render/busy/ms 55.90, stage/render/prepare_assets 34.28, main/busy/ms 31.63, stage/main/update 13.98, pipelines/driver_ms 13.43, stage/main/post_update 11.51, stage/render/render 10.38, render/graph/ms 8.32, encode//sum 8.17 |
| warm-up | 5.73 | 31.7 | pipelines/compile_ms 407, pipelines/front_ms 397, render/thread/ms 46.87, render/busy/ms 45.40, main/busy/ms 38.57, stage/render/prepare_assets 22.22, stage/main/update 20.43, stage/render/render 12.95, stage/main/post_update 11.61, render/graph/ms 10.49, pipelines/driver_ms 9.95, stream/drive/ms 9.65 |
| warm-up | 5.76 | 32.2 | render/thread/ms 49.11, render/busy/ms 47.81, stage/render/prepare_assets 26.16, main/busy/ms 24.64, stage/main/post_update 11.43, stage/render/render 10.11, encode//sum 8.11, render/graph/ms 8.08, stage/main/update 7.76, stage/render/graph_submit 5.71, encode/submit 3.37, stage/render/specialize 2.42 |
| warm-up | 5.87 | 33.1 | pipelines/compile_ms 338, pipelines/front_ms 318, render/thread/ms 51.85, render/busy/ms 49.71, stage/render/prepare_assets 31.04, main/busy/ms 23.00, pipelines/driver_ms 20.30, stage/render/render 13.59, render/graph/ms 10.69, stage/main/post_update 9.47, stage/main/update 8.93, stage/render/graph_submit 8.04 |
| warm-up | 6.43 | 38.6 | pipelines/compile_ms 158, pipelines/front_ms 154, render/thread/ms 46.43, render/busy/ms 43.65, main/busy/ms 32.72, stage/render/prepare_assets 16.94, stage/main/update 13.71, stage/render/render 13.68, stage/main/post_update 13.40, render/graph/ms 10.24, encode//sum 9.97, stage/render/graph_submit 6.72 |
| warm-up | 6.65 | 31.5 | render/thread/ms 44.54, render/busy/ms 42.84, main/busy/ms 41.50, stage/main/update 17.26, stage/render/render 16.56, stage/main/post_update 15.79, render/graph/ms 14.04, encode//sum 10.67, stage/render/graph_submit 8.71, stage/render/prepare_assets 6.71, stream/drive/ms 6.26, encode/submit 5.15 |
| warm-up | 6.70 | 29.4 | pipelines/compile_ms 117, pipelines/front_ms 110, render/thread/ms 41.73, render/busy/ms 40.63, main/busy/ms 35.41, stage/render/prepare_assets 17.10, stage/main/update 14.19, stage/main/post_update 13.94, stage/render/render 10.70, encode//sum 9.55, render/graph/ms 9.07, pipelines/driver_ms 7.41 |
| warm-up | 6.75 | 35.5 | main/busy/ms 45.30, render/thread/ms 31.31, render/busy/ms 30.00, stage/main/update 25.11, stream/drive/ms 14.87, stage/main/post_update 14.67, stage/render/render 11.75, encode//sum 9.76, render/graph/ms 9.74, stage/render/prepare_assets 7.00, stage/render/graph_submit 6.69, encode/submit 4.13 |
| warm-up | 6.81 | 29.2 | pipelines/compile_ms 179, pipelines/front_ms 172, main/busy/ms 40.55, render/thread/ms 39.74, render/busy/ms 38.43, stage/main/update 21.37, stage/render/prepare_assets 15.44, stage/render/render 12.80, stage/main/post_update 12.42, render/graph/ms 10.61, encode//sum 8.64, stage/render/graph_submit 7.43 |
| warm-up | 7.01 | 31.4 | pipelines/compile_ms 146, pipelines/front_ms 141, render/thread/ms 45.39, render/busy/ms 44.15, main/busy/ms 31.69, stage/render/prepare_assets 24.25, stage/main/update 13.86, stage/main/post_update 12.78, stage/render/render 12.27, render/graph/ms 10.49, encode//sum 8.92, stream/drive/ms 7.59 |
| warm-up | 7.19 | 32.3 | main/busy/ms 38.04, render/thread/ms 23.15, render/busy/ms 21.90, stage/main/update 20.09, pipelines/compile_ms 16.79, pipelines/front_ms 15.67, stream/drive/ms 12.95, stage/main/post_update 12.75, stream/spawn/ms 11.71, stage/render/render 10.74, render/graph/ms 9.19, encode//sum 7.71 |
| warm-up | 7.27 | 34.0 | render/thread/ms 40.41, render/busy/ms 39.30, main/busy/ms 22.35, stage/render/render 10.56, stage/render/specialize 9.84, stage/main/post_update 9.76, encode//sum 9.34, stage/main/update 8.98, render/graph/ms 8.95, stage/render/queue 7.01, stage/render/graph_submit 6.60, stage/render/prepare_assets 4.79 |
| warm-up | 8.53 | 29.1 | pipelines/compile_ms 139, pipelines/front_ms 136, render/thread/ms 35.44, render/busy/ms 34.48, main/busy/ms 18.51, stage/main/post_update 10.77, stage/render/render 9.69, stage/render/specialize 9.03, render/graph/ms 8.34, encode//sum 7.04, stage/render/graph_submit 6.10, stage/render/queue 5.75 |
| measure | 0.77 | 29.0 | main/busy/ms 33.37, render/thread/ms 23.24, render/busy/ms 22.15, stage/main/update 16.82, stage/main/post_update 12.80, stream/drive/ms 12.48, stage/render/render 10.46, render/graph/ms 8.84, encode//sum 7.92, stage/render/prepare_assets 6.03, stage/render/graph_submit 5.73, encode/submit 3.49 |
| measure | 1.71 | 30.8 | render/thread/ms 38.38, render/busy/ms 37.40, main/busy/ms 17.39, stage/render/render 13.37, render/graph/ms 11.85, stage/main/post_update 10.89, encode//sum 10.20, stage/render/graph_submit 8.40, stage/render/specialize 8.15, stage/render/queue 5.88, encode/submit 5.57, encode/pass:shadow_point_light/sum 4.17 |
| measure | 5.81 | 30.3 | render/thread/ms 34.44, render/busy/ms 33.43, main/busy/ms 19.62, stage/main/post_update 12.73, stage/render/render 9.86, stage/render/specialize 9.03, render/graph/ms 8.35, encode//sum 6.51, stage/render/queue 6.40, stage/render/graph_submit 6.37, stage/main/update 4.15, encode/submit 3.58 |
| measure | 7.09 | 28.7 | render/thread/ms 36.00, render/busy/ms 33.96, main/busy/ms 18.15, stage/render/render 12.64, stage/main/post_update 11.37, render/graph/ms 10.12, stage/render/specialize 8.68, encode//sum 8.05, stage/render/graph_submit 7.63, stage/render/queue 5.90, encode/submit 4.49, stage/main/update 4.25 |
| measure | 18.17 | 31.1 | main/busy/ms 37.99, stage/main/update 24.56, stream/drive/ms 20.77, render/thread/ms 20.16, render/busy/ms 19.13, stage/render/render 10.97, stage/main/post_update 10.81, render/graph/ms 9.51, encode//sum 7.32, stage/render/graph_submit 7.11, encode/submit 4.75, encode/finish_wall 2.40 |
| measure | 20.85 | 28.9 | render/thread/ms 37.52, render/busy/ms 36.48, main/busy/ms 18.83, stage/main/post_update 11.88, stage/render/render 11.74, render/graph/ms 10.33, stage/render/specialize 9.09, encode//sum 8.74, stage/render/graph_submit 7.70, stage/render/queue 7.19, encode/submit 5.17, stage/main/update 4.30 |
| measure | 23.02 | 31.7 | render/thread/ms 41.44, render/busy/ms 40.38, main/busy/ms 24.70, stage/render/render 12.96, stage/main/post_update 12.38, render/graph/ms 11.47, encode//sum 10.65, stage/render/specialize 10.48, stage/render/graph_submit 8.33, stage/render/queue 7.26, stage/main/update 6.41, encode/submit 5.18 |
| measure | 27.28 | 33.9 | render/thread/ms 41.46, render/busy/ms 40.29, main/busy/ms 24.96, stage/render/render 13.62, stage/main/post_update 13.00, encode//sum 12.62, render/graph/ms 12.06, stage/render/specialize 10.55, stage/render/graph_submit 9.30, stage/render/queue 7.18, encode/submit 5.93, stage/main/update 5.87 |
| measure | 28.72 | 29.5 | render/thread/ms 36.68, render/busy/ms 35.71, main/busy/ms 19.60, stage/main/post_update 12.58, stage/render/render 10.81, stage/render/specialize 9.51, render/graph/ms 9.41, encode//sum 7.58, stage/render/graph_submit 7.27, stage/render/queue 6.47, stage/main/update 4.44, encode/submit 4.26 |
| measure | 30.46 | 33.5 | render/thread/ms 37.25, render/busy/ms 36.19, main/busy/ms 20.40, stage/main/post_update 11.97, stage/render/render 10.87, stage/render/specialize 9.81, render/graph/ms 9.39, encode//sum 7.39, stage/render/graph_submit 7.22, stage/render/queue 6.60, stage/main/update 5.69, encode/submit 3.90 |
| measure | 35.34 | 29.8 | render/thread/ms 37.02, render/busy/ms 35.96, main/busy/ms 19.35, stage/main/post_update 11.62, stage/render/render 11.17, render/graph/ms 9.72, stage/render/specialize 9.29, encode//sum 9.00, stage/render/graph_submit 7.57, stage/render/queue 6.86, stage/main/update 4.92, encode/submit 3.88 |
| measure | 40.81 | 32.9 | render/thread/ms 39.22, render/busy/ms 38.20, main/busy/ms 18.75, stage/main/post_update 10.76, stage/render/render 10.31, stage/render/specialize 9.68, render/graph/ms 8.67, encode//sum 8.17, stage/render/graph_submit 6.85, stage/render/queue 6.64, stage/main/update 4.64, encode/submit 3.92 |
| measure | 43.22 | 29.2 | render/thread/ms 36.71, render/busy/ms 35.69, main/busy/ms 17.58, stage/render/render 10.29, stage/main/post_update 10.19, stage/render/specialize 9.91, render/graph/ms 8.86, encode//sum 7.09, stage/render/graph_submit 6.88, stage/render/queue 6.64, stage/main/update 4.45, encode/submit 3.75 |
| measure | 45.68 | 32.2 | render/thread/ms 37.94, render/busy/ms 36.94, main/busy/ms 16.97, stage/render/render 10.43, stage/main/post_update 10.08, stage/render/specialize 9.41, render/graph/ms 9.12, encode//sum 8.03, stage/render/graph_submit 7.14, stage/render/queue 6.71, stage/render/prepare_meshes 4.40, stage/main/update 4.33 |
| measure | 56.44 | 29.6 | render/thread/ms 37.09, render/busy/ms 35.92, main/busy/ms 15.11, stage/render/render 10.81, stage/render/specialize 9.25, render/graph/ms 9.17, stage/main/post_update 8.55, encode//sum 7.76, stage/render/graph_submit 6.99, stage/render/queue 6.44, stage/main/update 4.38, stage/render/prepare_meshes 3.63 |
| measure | 59.04 | 42.1 | render/thread/ms 52.97, render/busy/ms 51.47, main/busy/ms 28.15, stage/main/post_update 16.06, stage/render/render 14.88, stage/render/specialize 13.00, render/graph/ms 12.89, encode//sum 11.38, stage/render/graph_submit 9.67, stage/render/queue 9.50, stage/main/update 7.87, stage/render/prepare_meshes 6.27 |
| measure | 59.10 | 34.7 | render/thread/ms 37.67, render/busy/ms 36.73, stage/render/prepare_assets 19.71, main/busy/ms 18.91, stage/main/post_update 9.47, stage/render/render 9.06, encode//sum 9.00, render/graph/ms 7.86, stage/render/graph_submit 5.83, stage/main/update 5.01, encode/submit 3.78, encode/finish_wall 2.09 |
| measure | 62.57 | 31.8 | render/thread/ms 39.24, render/busy/ms 38.15, main/busy/ms 18.76, stage/render/specialize 10.89, stage/render/render 10.83, stage/main/post_update 9.72, render/graph/ms 9.36, encode//sum 7.94, stage/render/graph_submit 7.22, stage/render/queue 6.56, stage/main/update 5.91, stage/render/prepare_meshes 4.11 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set render.depth_only.skip_material=on --resolution 2560x1440 --bench-out target/perf\pt_nomat\balmora_walk@2560x1440_arm=nomat-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 9%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **78%** (min 46%) · 2625 MHz (min 2625) · memory 8101 MHz · 80.6 W · 76 °C (max 80) · VRAM 6749 MiB · throttled 100% of samples

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
- `render.depth_only.skip_material=on (command line: --set)`
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
