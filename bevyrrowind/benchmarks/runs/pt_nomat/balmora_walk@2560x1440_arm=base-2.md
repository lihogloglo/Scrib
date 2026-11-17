# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5955 frames over 65.6 s · **median 10.73 ms (93 fps)** · mean 11.02 · p95 13.70 · p99 18.08 (1% low 55 fps) · worst 47.24 ms · 85 spikes over 1.5× median · 77.7 stutters/min (work 59.4/min)
- warm-up: 323 frames over 9.5 s, worst 2604.7 ms
- hitches (> 26.8 ms): **33** measured, 51 in warm-up
- timed GPU passes: 6.56 ms of a 11.02 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.73 ms, limit 11 ms
- **FAIL** frame_ms: 85 stutters (> 1.5x moving median of 60) in 1.09 min = 77.69/min, limit 1/min
- **FAIL** worst frame 47.24 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.867 | 2.034 | 2.066 | 0.92 | 0.173 |
| main_opaque_pass_3d | 1.490 | 1.623 | 2.031 | 0.92 | 0.076 |
| early prepass | 0.448 | 0.488 | 0.717 | 0.92 | 0.087 |
| bloom | 0.289 | 0.315 | 0.321 | 0.92 | 0.062 |
| atmosphere_luts | 0.279 | 0.304 | 0.365 | 0.92 | 0.004 |
| water_velocity | 0.256 | 0.279 | 0.347 | 0.92 | 0.002 |
| water_sspr | 0.185 | 0.201 | 0.203 | 0.92 | 0.002 |
| ssao | 0.173 | 0.188 | 0.191 | 0.92 | 0.008 |
| atmos_froxel_inject | 0.133 | 0.145 | 0.147 | 0.92 | 0.002 |
| clouds_trace | 0.129 | 0.140 | 0.232 | 0.92 | 0.001 |
| auxiliary_views | 0.123 | 0.134 | 0.539 | 0.92 | 0.140 |
| water_fft | 0.120 | 0.130 | 0.137 | 0.92 | 0.003 |
| uniform_allocation | 0.118 | 0.129 | 0.182 | 0.92 | 0.028 |
| upscaling | 0.107 | 0.117 | 0.125 | 0.92 | 0.004 |
| clouds_composite | 0.097 | 0.105 | 0.108 | 0.92 | 0.002 |
| bin_unpacking | 0.096 | 0.104 | 0.141 | 0.92 | 0.021 |
| tonemapping | 0.078 | 0.085 | 0.088 | 0.92 | 0.004 |
| main_transmissive_pass_3d | 0.078 | 0.085 | 0.178 | 0.92 | 0.009 |
| clustering | 0.059 | 0.064 | 0.067 | 0.92 | 0.019 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.92 | 0.002 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.92 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.92 | 0.001 |
| surface_trail | 0.037 | 0.056 | 0.057 | 0.65 | 0.002 |
| surface_exposure_far | 0.035 | 0.043 | 0.045 | 0.82 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.033 | 0.92 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.047 | 0.92 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.032 | 0.92 | 0.001 |
| lightprobe_irradiance_map | 0.027 | 0.411 | 0.416 | 0.07 | 0.001 |
| water_snapshot | 0.022 | 0.024 | 0.028 | 0.92 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.024 | 0.92 | 0.002 |
| lightprobe_radiance_map | 0.015 | 0.219 | 0.229 | 0.07 | 0.004 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.92 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.026 | 0.92 | 0.122 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.92 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.92 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.92 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.91 | 0.003 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.07 | 0.001 |
| lightprobe_copy | 0.001 | 0.012 | 0.013 | 0.07 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.07 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.522 | 11.797 | 39.61 | 56701.4 |
| main/busy/ms | 9.061 | 10.795 | 25.32 | 53955.7 |
| render/busy/ms | 8.983 | 11.287 | 38.97 | 53495.7 |
| stage/main/post_update | 5.622 | 7.029 | 10.29 | 33480.0 |
| stage/render/render | 5.498 | 7.075 | 16.47 | 32740.2 |
| render/graph/ms | 4.763 | 6.314 | 15.80 | 28364.3 |
| encode//sum | 3.778 | 4.897 | 28.79 | 22496.8 |
| stage/render/graph_submit | 3.568 | 4.780 | 14.42 | 21245.2 |
| encode/submit | 2.254 | 3.216 | 13.26 | 13420.5 |
| stage/main/update | 2.163 | 2.555 | 18.92 | 12882.8 |
| encode/finish_wall | 1.380 | 2.015 | 4.64 | 8217.2 |
| stage/render/camera_driver | 1.142 | 1.590 | 4.23 | 6801.1 |
| encode/pass:shadow_directional_light/sum | 0.898 | 1.589 | 2.56 | 5347.7 |
| encode/pass:shadow_point_light/sum | 0.805 | 3.253 | 17.07 | 4795.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.804 | 1.113 | 1.80 | 4786.3 |
| stage/render/prepare_write_phase_buffers | 0.790 | 1.145 | 2.29 | 4703.8 |
| encode/pass:early prepass/sum | 0.778 | 1.088 | 1.92 | 4633.5 |
| stage/render/prepare_assets | 0.582 | 0.758 | 10.23 | 3467.4 |
| bigspace/total/ms | 0.536 | 1.299 | 2.43 | 3189.1 |
| stage/render/prepare_meshes | 0.463 | 0.601 | 6.66 | 2759.6 |
| stage/render/prepare_resources | 0.457 | 0.546 | 1.69 | 2722.8 |
| render/present/ms | 0.446 | 0.860 | 2.96 | 2658.5 |
| stage/main/first | 0.399 | 0.522 | 5.32 | 2377.1 |
| stage/main/pre_update | 0.361 | 0.511 | 2.92 | 2152.4 |
| stage/render/specialize | 0.339 | 0.434 | 11.86 | 2019.2 |
| stage/render/queue | 0.309 | 0.440 | 8.31 | 1842.5 |
| terrain/update/ms | 0.304 | 0.394 | 2.46 | 1807.7 |
| stage/render/prepare_bind_groups | 0.287 | 0.508 | 2.62 | 1707.4 |
| controller/feed/ms | 0.280 | 0.348 | 0.74 | 1665.0 |
| terrain/select/ms | 0.270 | 0.351 | 1.02 | 1609.2 |
| stream/drive/ms | 0.254 | 0.338 | 16.88 | 1509.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.245 | 0.356 | 1.00 | 1459.0 |
| bigspace/low_precision/ms | 0.225 | 0.691 | 1.86 | 1341.7 |
| stage/main/fixed_main_loop | 0.223 | 0.377 | 1.03 | 1325.8 |
| stage/render/create_views | 0.198 | 0.256 | 0.91 | 1181.6 |
| far/select/ms | 0.197 | 0.274 | 0.73 | 1175.3 |
| stream/tick/ms | 0.196 | 0.270 | 0.58 | 1168.4 |
| stage/render/prepare_batch_phases | 0.190 | 0.272 | 0.79 | 1129.2 |
| stage/render/extract_commands | 0.179 | 0.210 | 0.44 | 1064.6 |
| stage/render/prepare_flush | 0.163 | 0.306 | 1.90 | 968.7 |
| anim/evaluate/ms | 0.158 | 0.260 | 1.58 | 941.6 |
| anim/evaluate_cpu/ms | 0.152 | 0.188 | 0.77 | 905.1 |
| bigspace/high_precision/ms | 0.146 | 0.602 | 2.08 | 868.4 |
| controller/move/ms | 0.145 | 0.323 | 0.73 | 863.5 |
| stage/render/prepare_views | 0.136 | 0.172 | 0.53 | 811.5 |
| stage/main/last | 0.115 | 0.165 | 5.03 | 687.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.101 | 0.154 | 0.77 | 603.0 |
| render/acquire/ms | 0.092 | 0.118 | 0.49 | 547.1 |
| stage/render/prepare_collect_phase_buffers | 0.063 | 0.134 | 0.47 | 377.9 |
| pipelines/compile_ms | 0.050 | 0.000 | 125.59 | 298.8 |
| physics/step/ms | 0.049 | 0.089 | 0.54 | 290.5 |
| pipelines/front_ms | 0.048 | 0.000 | 121.81 | 287.0 |
| encode/resolve_diagnostics/sum | 0.032 | 0.043 | 0.53 | 192.6 |
| stage/render/cleanup | 0.032 | 0.097 | 0.97 | 192.2 |
| physics/insert/ms | 0.031 | 0.355 | 0.94 | 183.2 |
| stage/render/phase_sort | 0.026 | 0.046 | 0.29 | 157.1 |
| weather/map/ms | 0.025 | 0.033 | 0.18 | 149.0 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.022 | 0.54 | 92.6 |
| encode/camera_driver_span_end/sum | 0.012 | 0.020 | 0.55 | 72.4 |
| anim/apply/ms | 0.012 | 0.021 | 0.15 | 70.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.41 | 64.1 |
| weather/eval/ms | 0.010 | 0.014 | 0.13 | 60.6 |
| script/update/ms | 0.008 | 0.009 | 0.05 | 45.4 |
| script/input/ms | 0.005 | 0.007 | 0.23 | 32.6 |
| stream/despawn/ms | 0.005 | 0.000 | 1.01 | 30.4 |
| stream/spawn/ms | 0.005 | 0.000 | 1.15 | 28.7 |
| script/ui/ms | 0.004 | 0.005 | 0.41 | 24.2 |
| controller/write/ms | 0.003 | 0.005 | 0.03 | 20.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.66 | 13.6 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.02 | 11.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.10 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.11 | 0.3 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 20 | 30 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1301 | 2262 | 661 |
| crowd/c0/meshes/count | count | 353 | 646 | 173 |
| crowd/c1/count | count | 145 | 155 | 147 |
| crowd/c1/joints/count | count | 5531 | 5928 | 5535 |
| crowd/c1/meshes/count | count | 1166 | 1270 | 1168 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4613 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.41 | 1.94 | 0.56 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.54 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.55 | 0.01 |
| encode/finish_critical | ms | 0.99 | 2.56 | 1.59 |
| encode/pass:early prepass/max | ms | 0.79 | 1.88 | 1.17 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.81 | 1.65 | 0.76 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.77 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.00 | 0.31 |
| encode/pass:shadow_directional_light/max | ms | 0.83 | 2.56 | 1.59 |
| encode/pass:shadow_point_light/max | ms | 0.83 | 1.54 | 0.94 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.53 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.31 | 38.97 | 8.84 |
| gpu/timed/ms | ms | 7.88 | 11.38 | 8.13 |
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
| phases/alpha_mask3d/items | count | 822 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 822 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1291 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3437 | 5118 | 3189 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1291 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3437 | 5118 | 3189 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 17 | 201 | 29 |
| phases/shadow/bins | count | 4995 | 8706 | 2413 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.3 k | 6360 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 202 | 253 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 267 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 12 | 563 | 58 |
| preprocess/bind_groups_created | count | 12 | 563 | 58 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 239 | 123 |
| process/cpu_usage | % | 25.84 | 40.87 | 24.52 |
| process/mem_usage | GiB | 5.48 | 5.65 | 5.65 |
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
| skin/blocks_changed/count | count | 14 | 128 | 12 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 368.4 k | 808.7 k | 496.9 k |
| skin/joints_written/count | count | 5756 | 12.6 k | 7764 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4919 | 5014 | 5014 |
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
| stream/latency/s | s | 0.04 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.99 | 42.01 | 32.65 |
| system/mem_usage | % | 62.32 | 62.91 | 62.87 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 66 | 250 | 79 |
| views/all/bins | count | 7572 | 11.6 k | 5911 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.2 k | 14.0 k |
| views/all/pipelines | count | 249 | 300 | 75 |
| views/all/pipelines_distinct | count | 71 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 35.7 k | 14.3 k |
| views/cascade/batch_sets | count | 9 | 51 | 29 |
| views/cascade/bins | count | 2789 | 6194 | 2413 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8920 | 23.5 k | 6360 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 409 | 23.5 k | 6360 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2147 | 2723 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2398 | 3021 | 0 |
| views/lamp_face/pipelines | count | 165 | 203 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2933 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2882 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8514 | 13.0 k | 7612 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8670 | 13.2 k | 7920 |
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
| anim/bones_written/count | count | 53.23 | 222 | 317.0 k |
| anim/events/count | count | 0.08 | 2 | 501 |
| assets/Image/added | count | 0.10 | 37 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.68 | 152 | 4061 |
| assets/Mesh/removed | count | 0.81 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.64 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.79 | 1272 | 4676 |
| encode//count | count | 32.88 | 220 | 195.8 k |
| encode/bind_walk/init_textures | count | 12.2 k | 114.7 k | 72.66 M |
| encode/bind_walk/init_textures_skipped | count | 65.8 k | 156.8 k | 391.83 M |
| encode/bind_walk/inits | count | 457 | 1129 | 2.72 M |
| encode/bind_walk/inits_skipped | count | 108 | 294 | 642.8 k |
| encode/bind_walk/merge_textures | count | 12.1 k | 114.6 k | 72.23 M |
| encode/bind_walk/merge_textures_skipped | count | 5362 | 12.0 k | 31.93 M |
| encode/bind_walk/merges | count | 97.64 | 336 | 581.4 k |
| encode/bind_walk/merges_skipped | count | 83.62 | 251 | 498.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 5955 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 5955 |
| encode/pass:early prepass/count | count | 1 | 2 | 5955 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 5955 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 5955 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 5955 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 5955 |
| encode/pass:shadow_point_light/count | count | 1.23 | 48 | 7323 |
| encode/queue/command_buffers | count | 44.12 | 278 | 262.7 k |
| encode/queue/submits | count | 2.02 | 4 | 12.0 k |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 4.89 M | 11.24 G |
| encode/queue/write_buffer/calls | count | 258 | 840 | 1.54 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.6 k | 3.52 M | 1.06 G |
| encode/queue/write_buffer_with/calls | count | 11.03 | 465 | 65.7 k |
| encode/queue/write_texture/bytes | bytes | 12.0 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 5955 |
| physics/inserts | count | 0.38 | 110 | 2249 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 3 | 3938 |
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
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 19 | 378 |
| stream/upload/bytes | bytes | 5685 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1246 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.8 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.60 | 2604.7 | main/busy/ms 2573, stage/main/post_update 2467, stage/main/update 96.91, surface/advance_cpu_ms 67.77, stage/main/pre_update 5.62, stage/main/last 1.12, surface/cell_upload_ms 0.77, stream/drive/ms 0.64, stage/main/first 0.49, stage/main/fixed_main_loop 0.38, weather/map/ms 0.24, stream/tick/ms 0.17 |
| warm-up | 3.36 | 758.3 | main/busy/ms 2710, stage/main/post_update 2491, render/thread/ms 716, render/busy/ms 543, stage/render/prepare_assets 493, stage/main/update 204, stage/render/prepare_views 167, render/acquire/ms 167, surface/advance_cpu_ms 67.77, stage/render/create_views 28.37, stage/render/render 16.31, render/graph/ms 9.23 |
| warm-up | 3.54 | 179.4 | render/thread/ms 878, render/busy/ms 619, stage/render/prepare_assets 547, main/busy/ms 168, stage/render/prepare_views 167, render/acquire/ms 167, pipelines/compile_ms 147, stage/main/update 115, stage/render/render 114, render/present/ms 91.74, pipelines/driver_ms 79.03, pipelines/front_ms 67.94 |
| warm-up | 3.72 | 151.7 | pipelines/wait_ms 3030, pipelines/compile_ms 1648, pipelines/front_ms 1460, pipelines/driver_ms 188, main/busy/ms 167, render/thread/ms 124, render/busy/ms 118, stage/main/post_update 77.94, stage/main/update 64.93, stage/render/prepare_resources 59.52, stage/render/prepare_assets 24.25, stage/render/render 20.92 |
| warm-up | 3.80 | 86.9 | pipelines/wait_ms 3284, pipelines/compile_ms 2306, pipelines/front_ms 2184, render/thread/ms 175, render/busy/ms 172, main/busy/ms 138, pipelines/driver_ms 122, stage/render/prepare_assets 82.15, stage/main/update 66.17, stage/render/prepare_resources 58.48, stage/main/post_update 48.45, stage/render/render 16.32 |
| warm-up | 3.83 | 29.9 | pipelines/compile_ms 1258, pipelines/front_ms 1191, pipelines/wait_ms 366, render/thread/ms 98.20, render/busy/ms 96.43, stage/render/prepare_assets 77.31, pipelines/driver_ms 66.69, main/busy/ms 36.90, stage/main/post_update 16.34, terrain/update/ms 14.73, stage/render/render 11.74, stage/main/update 11.41 |
| warm-up | 3.86 | 31.3 | pipelines/compile_ms 601, pipelines/front_ms 577, pipelines/wait_ms 148, render/thread/ms 57.78, render/busy/ms 55.27, main/busy/ms 46.89, pipelines/driver_ms 23.75, stage/render/prepare_assets 23.69, stage/main/post_update 22.35, encode//sum 17.66, stage/render/render 13.13, stage/main/pre_update 11.07 |
| warm-up | 3.94 | 29.8 | pipelines/compile_ms 438, pipelines/front_ms 425, render/thread/ms 42.64, render/busy/ms 40.89, main/busy/ms 31.63, stage/render/prepare_assets 17.79, stage/main/post_update 16.08, pipelines/driver_ms 12.45, stage/render/render 11.32, pipelines/wait_ms 10.00, terrain/update/ms 9.05, render/graph/ms 9.03 |
| warm-up | 4.06 | 92.0 | pipelines/compile_ms 1242, pipelines/front_ms 1195, render/thread/ms 66.26, main/busy/ms 65.21, render/busy/ms 64.29, stage/main/update 50.52, pipelines/driver_ms 47.01, stream/drive/ms 45.12, stage/render/prepare_assets 41.17, pipelines/wait_ms 26.31, stage/render/render 12.58, render/graph/ms 10.29 |
| warm-up | 4.09 | 34.1 | pipelines/compile_ms 1701, pipelines/front_ms 1634, render/thread/ms 78.56, render/busy/ms 76.51, pipelines/driver_ms 66.99, main/busy/ms 52.08, stage/render/prepare_assets 42.81, stage/main/update 34.72, pipelines/wait_ms 31.95, stage/render/render 27.93, render/graph/ms 25.48, stream/drive/ms 12.07 |
| warm-up | 4.14 | 48.0 | pipelines/compile_ms 585, pipelines/front_ms 560, render/thread/ms 64.97, render/busy/ms 62.98, main/busy/ms 49.39, stage/main/update 32.95, stage/render/prepare_assets 29.50, stage/render/render 27.31, pipelines/driver_ms 25.13, render/graph/ms 24.91, pipelines/wait_ms 23.55, pipelines/lock_ms 16.44 |
| warm-up | 4.18 | 37.8 | pipelines/compile_ms 176, pipelines/front_ms 168, render/thread/ms 70.13, render/busy/ms 68.47, main/busy/ms 53.12, stage/render/prepare_assets 42.14, stage/main/update 35.81, stream/drive/ms 28.61, pipelines/wait_ms 19.59, stage/render/render 13.09, pipelines/lock_ms 12.48, stage/main/post_update 11.36 |
| warm-up | 4.25 | 72.6 | pipelines/compile_ms 107, pipelines/front_ms 102, render/thread/ms 89.07, render/busy/ms 86.99, main/busy/ms 77.71, stage/main/update 54.05, stage/render/prepare_assets 47.77, stream/drive/ms 30.00, stage/render/render 18.07, stage/main/post_update 15.46, render/graph/ms 15.35, stage/render/prepare_bind_groups 10.31 |
| warm-up | 4.32 | 68.0 | pipelines/compile_ms 1116, pipelines/front_ms 1084, render/thread/ms 101, render/busy/ms 98.93, main/busy/ms 89.20, stage/main/update 63.76, stage/render/prepare_assets 56.19, pipelines/driver_ms 31.72, pipelines/wait_ms 27.71, stage/render/render 26.82, render/graph/ms 22.60, pipelines/lock_ms 21.99 |
| warm-up | 4.47 | 157.6 | pipelines/compile_ms 2291, pipelines/front_ms 2203, render/thread/ms 163, render/busy/ms 133, pipelines/driver_ms 88.29, main/busy/ms 73.07, stage/render/prepare_assets 62.96, pipelines/wait_ms 54.42, stage/render/render 50.55, stage/main/update 50.28, stage/render/prepare_bind_groups 36.42, render/present/ms 29.79 |
| warm-up | 4.54 | 70.0 | pipelines/compile_ms 1814, pipelines/front_ms 1725, pipelines/wait_ms 426, render/thread/ms 174, render/busy/ms 143, pipelines/driver_ms 88.41, stage/render/prepare_assets 70.76, main/busy/ms 54.01, stage/render/render 42.79, stage/render/prepare_bind_groups 37.51, render/present/ms 30.79, stage/main/update 27.52 |
| warm-up | 4.66 | 116.2 | pipelines/compile_ms 2218, pipelines/front_ms 2141, pipelines/wait_ms 457, render/thread/ms 163, render/busy/ms 159, main/busy/ms 92.34, pipelines/driver_ms 76.73, stage/render/prepare_assets 69.66, stage/main/post_update 44.22, stage/main/update 37.62, encode//sum 37.26, stage/render/render 35.06 |
| warm-up | 4.74 | 83.7 | pipelines/compile_ms 2766, pipelines/front_ms 2685, pipelines/wait_ms 380, render/thread/ms 177, render/busy/ms 175, pipelines/driver_ms 80.63, stage/render/prepare_assets 78.57, main/busy/ms 75.62, stage/main/post_update 43.30, encode//sum 39.39, stage/render/render 37.86, render/graph/ms 35.28 |
| warm-up | 4.82 | 71.7 | pipelines/compile_ms 1447, pipelines/front_ms 1405, pipelines/wait_ms 412, render/thread/ms 142, render/busy/ms 140, stage/render/prepare_assets 66.36, pipelines/driver_ms 42.52, main/busy/ms 40.71, stage/render/render 31.79, render/graph/ms 28.90, stage/render/graph_submit 22.11, stage/main/post_update 18.82 |
| warm-up | 4.85 | 35.3 | pipelines/compile_ms 1074, pipelines/front_ms 1041, pipelines/wait_ms 197, render/thread/ms 93.36, render/busy/ms 91.57, main/busy/ms 42.10, stage/render/prepare_assets 35.38, pipelines/driver_ms 33.38, stage/render/render 24.43, render/graph/ms 21.79, stage/main/update 18.55, stage/render/graph_submit 16.74 |
| warm-up | 4.88 | 31.0 | pipelines/compile_ms 776, pipelines/front_ms 752, pipelines/wait_ms 106, main/busy/ms 82.27, stage/main/update 63.25, stream/drive/ms 52.95, render/thread/ms 47.50, render/busy/ms 45.58, pipelines/driver_ms 24.63, stage/render/prepare_assets 21.04, stage/render/render 11.36, stage/main/post_update 9.79 |
| warm-up | 4.97 | 84.8 | pipelines/compile_ms 1510, pipelines/front_ms 1472, pipelines/wait_ms 131, main/busy/ms 110, stage/main/update 90.61, render/thread/ms 63.66, render/busy/ms 61.76, stream/drive/ms 51.76, pipelines/driver_ms 37.59, stage/render/render 34.67, render/graph/ms 31.60, stage/render/camera_driver 19.75 |
| warm-up | 5.02 | 57.5 | pipelines/compile_ms 2000, pipelines/front_ms 1947, pipelines/wait_ms 139, main/busy/ms 138, stage/main/update 113, render/thread/ms 74.88, render/busy/ms 73.22, stream/drive/ms 54.36, pipelines/driver_ms 52.85, stage/render/render 34.41, render/graph/ms 31.54, stage/render/prepare_assets 26.75 |
| warm-up | 5.15 | 124.2 | pipelines/compile_ms 1587, pipelines/front_ms 1545, main/busy/ms 105, stage/main/update 82.95, render/thread/ms 76.21, render/busy/ms 74.71, stream/drive/ms 55.12, stage/render/prepare_assets 45.13, pipelines/driver_ms 41.77, pipelines/wait_ms 16.71, pipelines/lock_ms 15.80, stage/render/render 12.43 |
| warm-up | 5.20 | 53.6 | pipelines/compile_ms 1550, pipelines/front_ms 1512, render/thread/ms 85.77, render/busy/ms 83.87, main/busy/ms 78.84, stage/main/update 58.11, stream/drive/ms 43.62, pipelines/driver_ms 38.11, stage/render/prepare_assets 37.88, stage/render/render 15.45, render/graph/ms 12.69, stage/main/post_update 11.62 |
| warm-up | 5.28 | 77.9 | pipelines/compile_ms 486, pipelines/front_ms 468, main/busy/ms 98.15, render/thread/ms 89.49, render/busy/ms 87.37, stage/main/update 66.81, stream/drive/ms 44.02, stage/render/prepare_assets 29.82, stage/render/render 29.15, render/graph/ms 26.33, stage/render/graph_submit 22.54, stage/main/post_update 22.01 |
| warm-up | 5.32 | 35.9 | pipelines/compile_ms 316, pipelines/front_ms 302, main/busy/ms 84.04, render/thread/ms 70.40, render/busy/ms 68.68, stage/main/update 57.06, stream/drive/ms 36.02, stage/render/prepare_assets 27.86, stage/render/render 25.80, render/graph/ms 23.52, stage/render/graph_submit 20.48, stage/main/post_update 19.64 |
| warm-up | 5.37 | 53.5 | pipelines/compile_ms 518, pipelines/front_ms 503, main/busy/ms 68.85, stage/main/update 52.30, render/thread/ms 47.03, render/busy/ms 45.60, stream/drive/ms 35.22, stage/render/prepare_assets 19.33, pipelines/driver_ms 14.97, stage/render/render 11.06, stage/main/post_update 9.71, render/graph/ms 8.94 |
| warm-up | 5.41 | 37.4 | pipelines/compile_ms 235, pipelines/front_ms 228, render/thread/ms 55.44, render/busy/ms 53.73, main/busy/ms 49.94, stage/main/update 31.27, stage/render/prepare_assets 13.21, stage/render/render 12.84, stage/main/post_update 11.67, render/graph/ms 10.76, stream/drive/ms 8.81, encode//sum 7.99 |
| warm-up | 5.44 | 31.6 | render/thread/ms 55.93, render/busy/ms 54.31, main/busy/ms 45.06, stage/main/update 25.56, stage/render/prepare_assets 15.45, stage/render/render 14.01, render/graph/ms 11.80, stage/main/post_update 11.53, encode//sum 9.13, stream/drive/ms 8.88, stage/render/graph_submit 8.58, encode/submit 6.09 |
| warm-up | 5.50 | 38.8 | pipelines/compile_ms 920, pipelines/front_ms 892, render/thread/ms 56.91, render/busy/ms 55.23, main/busy/ms 40.51, pipelines/driver_ms 28.81, stage/render/prepare_assets 19.81, stage/main/update 18.85, stage/render/render 18.26, render/graph/ms 15.92, stage/main/post_update 15.36, stage/render/graph_submit 9.22 |
| warm-up | 5.57 | 41.6 | pipelines/compile_ms 267, pipelines/front_ms 258, render/thread/ms 50.55, render/busy/ms 49.21, main/busy/ms 35.55, stage/render/prepare_assets 17.95, stage/main/update 17.05, stage/main/post_update 11.92, stage/render/render 11.77, render/graph/ms 9.67, pipelines/driver_ms 8.68, encode//sum 7.90 |
| warm-up | 5.63 | 31.0 | pipelines/compile_ms 526, pipelines/front_ms 509, render/thread/ms 51.97, render/busy/ms 50.46, main/busy/ms 37.12, stage/render/prepare_assets 20.56, pipelines/driver_ms 16.95, stage/main/post_update 16.53, stage/main/update 14.15, stage/render/render 13.34, render/graph/ms 11.32, encode//sum 8.26 |
| warm-up | 5.68 | 36.8 | pipelines/compile_ms 658, pipelines/front_ms 635, render/thread/ms 53.13, render/busy/ms 51.45, main/busy/ms 38.01, pipelines/driver_ms 22.51, stage/render/render 18.84, stage/main/update 17.25, render/graph/ms 16.15, stage/main/post_update 15.72, stage/render/prepare_write_phase_buffers 13.83, stage/render/graph_submit 11.71 |
| warm-up | 5.75 | 41.5 | pipelines/compile_ms 423, pipelines/front_ms 415, render/thread/ms 51.97, render/busy/ms 50.66, main/busy/ms 33.82, stage/render/prepare_assets 27.88, stage/main/update 15.71, stage/render/render 11.79, stage/main/post_update 11.12, render/graph/ms 9.73, pipelines/driver_ms 8.32, encode//sum 8.04 |
| warm-up | 5.85 | 38.5 | render/thread/ms 49.67, render/busy/ms 47.98, main/busy/ms 37.16, stage/render/prepare_assets 23.29, stage/main/update 20.46, stage/render/render 13.79, render/graph/ms 11.37, stage/main/post_update 11.17, encode//sum 9.79, stream/drive/ms 9.65, stage/render/graph_submit 8.08, encode/submit 5.72 |
| warm-up | 5.92 | 39.2 | pipelines/compile_ms 75.72, pipelines/front_ms 70.55, render/thread/ms 53.31, render/busy/ms 51.58, main/busy/ms 29.97, stage/render/prepare_assets 27.97, stage/main/post_update 14.14, stage/render/render 14.04, render/graph/ms 11.50, stage/main/update 10.49, stage/render/graph_submit 8.68, encode//sum 8.15 |
| warm-up | 5.98 | 34.2 | pipelines/compile_ms 591, pipelines/front_ms 560, render/thread/ms 55.26, render/busy/ms 53.81, stage/render/prepare_assets 35.02, pipelines/driver_ms 31.30, main/busy/ms 29.67, stage/main/update 13.01, stage/render/render 12.06, stage/main/post_update 11.08, render/graph/ms 9.98, encode//sum 8.28 |
| warm-up | 6.03 | 32.0 | render/thread/ms 44.71, render/busy/ms 43.39, pipelines/compile_ms 31.20, pipelines/front_ms 29.67, main/busy/ms 24.88, stage/render/prepare_assets 22.44, stage/render/render 12.13, stage/main/post_update 10.70, render/graph/ms 9.98, stage/main/update 9.25, encode//sum 8.11, stage/render/graph_submit 6.67 |
| warm-up | 6.22 | 43.4 | pipelines/compile_ms 206, pipelines/front_ms 201, main/busy/ms 39.60, render/thread/ms 31.79, render/busy/ms 30.42, stage/main/update 24.00, stream/drive/ms 17.98, stage/render/render 13.16, render/graph/ms 11.17, stage/main/post_update 10.05, encode//sum 8.28, stage/render/graph_submit 8.01 |
| warm-up | 6.46 | 30.4 | main/busy/ms 35.83, render/thread/ms 22.83, render/busy/ms 21.56, stage/main/update 21.42, stream/drive/ms 14.38, stage/main/post_update 9.70, stage/render/render 9.43, render/graph/ms 7.62, encode//sum 7.62, stage/render/prepare_assets 5.27, stage/render/graph_submit 5.14, encode/submit 2.94 |
| warm-up | 6.60 | 31.9 | render/thread/ms 45.30, render/busy/ms 43.98, main/busy/ms 30.29, stage/render/prepare_assets 19.00, stage/main/update 12.97, stage/render/render 9.69, stage/main/post_update 9.27, encode//sum 8.01, render/graph/ms 8.00, stage/render/graph_submit 5.71, stream/drive/ms 4.49, stage/render/specialize 4.17 |
| warm-up | 7.32 | 33.7 | main/busy/ms 38.79, render/thread/ms 20.60, stage/main/update 20.29, render/busy/ms 19.58, stream/drive/ms 13.99, stage/main/post_update 13.10, stream/spawn/ms 12.70, stage/render/render 8.74, render/graph/ms 7.28, encode//sum 7.07, stage/render/graph_submit 5.08, stage/render/prepare_assets 4.85 |
| warm-up | 7.41 | 32.8 | render/thread/ms 45.68, render/busy/ms 44.07, main/busy/ms 21.97, stage/render/render 11.19, stage/render/specialize 10.97, stage/main/post_update 10.22, render/graph/ms 9.07, stage/main/update 8.38, stage/render/queue 7.98, encode//sum 6.93, stage/render/graph_submit 6.90, stage/render/prepare_assets 6.40 |
| measure | 0.44 | 31.4 | render/thread/ms 42.04, render/busy/ms 40.72, main/busy/ms 18.59, encode//sum 18.21, stage/render/render 15.32, render/graph/ms 13.45, stage/main/post_update 11.62, stage/render/graph_submit 10.88, stage/render/specialize 9.49, stage/render/queue 7.92, encode/submit 6.70, encode/finish_wall 4.23 |
| measure | 1.75 | 30.7 | render/thread/ms 36.62, render/busy/ms 35.57, main/busy/ms 18.82, stage/render/render 11.85, stage/main/post_update 10.94, render/graph/ms 10.28, encode//sum 9.70, stage/render/specialize 8.86, stage/render/graph_submit 7.70, stage/render/queue 5.61, stage/main/update 4.63, encode/submit 4.56 |
| measure | 3.21 | 30.4 | render/thread/ms 39.82, render/busy/ms 38.85, main/busy/ms 19.84, stage/main/post_update 13.48, stage/render/render 13.28, render/graph/ms 11.87, stage/render/specialize 10.26, encode//sum 9.99, stage/render/graph_submit 8.76, encode/pass:shadow_point_light/sum 6.25, stage/render/queue 6.16, encode/submit 5.37 |
| measure | 5.97 | 31.6 | render/thread/ms 37.50, render/busy/ms 36.44, main/busy/ms 21.19, stage/main/post_update 12.28, stage/render/render 10.98, stage/render/specialize 10.16, render/graph/ms 9.45, encode//sum 7.52, stage/render/graph_submit 7.34, stage/render/queue 6.70, encode/submit 4.28, stage/main/update 3.82 |
| measure | 7.30 | 29.8 | render/thread/ms 37.30, render/busy/ms 36.32, main/busy/ms 17.69, stage/render/render 11.41, stage/main/post_update 11.08, render/graph/ms 9.98, stage/render/specialize 9.46, encode//sum 8.66, stage/render/graph_submit 7.53, stage/render/queue 6.87, encode/submit 4.65, stage/main/update 4.04 |
| measure | 15.73 | 30.4 | render/thread/ms 39.44, render/busy/ms 38.45, main/busy/ms 19.11, stage/render/render 12.49, stage/main/post_update 12.41, render/graph/ms 10.96, stage/render/specialize 10.35, stage/render/graph_submit 8.50, encode//sum 8.49, stage/render/queue 6.93, encode/submit 4.96, stage/main/update 3.89 |
| measure | 17.08 | 29.7 | render/thread/ms 39.49, render/busy/ms 37.55, main/busy/ms 18.04, stage/render/render 13.22, stage/main/post_update 11.15, render/graph/ms 10.72, stage/render/specialize 9.61, encode//sum 8.94, stage/render/graph_submit 8.03, stage/render/queue 6.86, encode/submit 4.84, stage/main/update 4.52 |
| measure | 23.02 | 36.3 | render/thread/ms 43.23, render/busy/ms 42.12, main/busy/ms 25.81, stage/main/post_update 14.07, stage/render/specialize 12.15, stage/render/render 11.87, render/graph/ms 10.44, stage/render/queue 8.63, stage/render/graph_submit 8.18, encode//sum 7.67, stage/main/update 6.47, encode/submit 4.50 |
| measure | 27.29 | 33.9 | render/thread/ms 45.06, render/busy/ms 44.05, main/busy/ms 25.32, stage/render/render 15.15, render/graph/ms 13.75, stage/main/post_update 12.39, stage/render/specialize 11.58, stage/render/graph_submit 10.91, encode//sum 9.11, stage/render/queue 7.93, encode/pass:shadow_point_light/sum 7.32, encode/submit 6.66 |
| measure | 28.69 | 30.8 | render/thread/ms 37.11, render/busy/ms 35.93, main/busy/ms 20.45, stage/main/post_update 12.61, stage/render/render 10.43, stage/render/specialize 10.11, render/graph/ms 8.81, encode//sum 7.79, stage/render/queue 6.91, stage/render/graph_submit 6.78, stage/main/update 4.65, encode/submit 4.17 |
| measure | 30.49 | 34.9 | render/thread/ms 43.84, render/busy/ms 42.70, main/busy/ms 21.92, stage/render/render 14.39, render/graph/ms 12.86, stage/main/post_update 12.70, stage/render/specialize 11.15, stage/render/graph_submit 9.77, encode//sum 8.90, stage/render/queue 7.80, encode/pass:shadow_point_light/sum 7.11, stage/main/update 6.22 |
| measure | 35.35 | 33.0 | render/thread/ms 40.86, render/busy/ms 39.74, main/busy/ms 20.19, stage/main/post_update 12.96, stage/render/render 12.64, render/graph/ms 11.01, stage/render/specialize 10.63, stage/render/graph_submit 8.65, encode//sum 8.26, stage/render/queue 7.16, encode/submit 4.94, stage/main/update 4.30 |
| measure | 36.72 | 32.1 | render/thread/ms 38.04, render/busy/ms 36.99, main/busy/ms 18.64, stage/main/post_update 11.91, stage/render/render 11.89, render/graph/ms 10.31, stage/render/specialize 9.80, encode//sum 8.77, stage/render/graph_submit 8.05, stage/render/queue 7.02, encode/submit 4.42, stage/main/update 3.97 |
| measure | 37.48 | 30.6 | render/thread/ms 37.85, render/busy/ms 36.90, main/busy/ms 17.82, stage/render/render 11.57, stage/main/post_update 11.40, render/graph/ms 10.15, stage/render/specialize 10.04, encode//sum 8.45, stage/render/graph_submit 7.96, stage/render/queue 6.94, encode/submit 4.38, stage/main/update 4.01 |
| measure | 40.79 | 33.0 | render/thread/ms 38.27, render/busy/ms 37.24, main/busy/ms 18.46, stage/main/post_update 10.44, stage/render/specialize 10.37, stage/render/render 10.05, render/graph/ms 8.70, stage/render/graph_submit 6.82, stage/render/queue 6.70, encode//sum 6.62, stage/main/update 4.68, encode/submit 3.99 |
| measure | 43.21 | 30.0 | render/thread/ms 38.46, render/busy/ms 37.50, main/busy/ms 17.82, stage/render/render 11.14, stage/main/post_update 10.65, render/graph/ms 9.94, stage/render/specialize 9.87, stage/render/graph_submit 7.97, encode//sum 7.71, stage/render/queue 6.75, stage/main/update 4.48, encode/submit 4.04 |
| measure | 45.66 | 34.4 | render/thread/ms 39.80, render/busy/ms 38.77, main/busy/ms 17.69, stage/render/render 11.16, stage/render/specialize 10.17, stage/main/post_update 9.72, render/graph/ms 9.71, encode//sum 7.87, stage/render/graph_submit 7.80, stage/render/queue 6.98, stage/main/update 4.66, stage/render/prepare_meshes 4.57 |
| measure | 59.05 | 47.2 | render/thread/ms 55.90, render/busy/ms 54.22, main/busy/ms 33.29, stage/main/post_update 19.20, stage/render/render 16.21, render/graph/ms 14.09, stage/render/specialize 11.83, stage/render/graph_submit 10.73, encode//sum 10.22, stage/main/update 9.78, stage/render/prepare_meshes 7.12, stage/render/queue 6.68 |
| measure | 59.08 | 31.5 | render/thread/ms 58.89, render/busy/ms 57.73, main/busy/ms 26.88, stage/render/prepare_assets 16.02, stage/main/post_update 13.96, stage/render/specialize 11.98, stage/render/render 10.28, render/graph/ms 8.83, stage/render/prepare_meshes 7.71, encode//sum 7.51, stage/main/update 7.32, stage/render/graph_submit 6.82 |
| measure | 62.57 | 31.2 | render/thread/ms 38.44, render/busy/ms 37.35, main/busy/ms 18.89, stage/render/render 10.99, stage/main/post_update 10.58, stage/render/specialize 10.37, render/graph/ms 9.66, stage/render/graph_submit 7.62, encode//sum 7.37, stage/render/queue 6.74, stage/main/update 5.99, stage/render/prepare_meshes 4.18 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_nomat\balmora_walk@2560x1440_arm=base-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 20%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **76%** (min 55%) · 2624 MHz (min 2610) · memory 8101 MHz · 79.2 W · 75 °C (max 81) · VRAM 6740 MiB · throttled 100% of samples

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
