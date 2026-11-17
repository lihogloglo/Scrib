# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); mw.matctrl.gate=on (command line: --set); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6115 frames over 65.6 s · **median 10.45 ms (96 fps)** · mean 10.73 · p95 13.25 · p99 16.56 (1% low 60 fps) · worst 45.76 ms · 70 spikes over 1.5× median · 56.7 stutters/min (work 50.3/min)
- warm-up: 324 frames over 9.2 s, worst 2545.8 ms
- hitches (> 26.1 ms): **34** measured, 46 in warm-up
- timed GPU passes: 6.56 ms of a 10.73 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.45 ms, limit 11 ms
- **FAIL** frame_ms: 62 stutters (> 1.5x moving median of 60) in 1.09 min = 56.67/min, limit 1/min
- **FAIL** worst frame 45.76 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.863 | 2.037 | 2.071 | 0.91 | 0.174 |
| main_opaque_pass_3d | 1.493 | 1.631 | 2.044 | 0.92 | 0.066 |
| early prepass | 0.449 | 0.491 | 0.718 | 0.92 | 0.087 |
| bloom | 0.288 | 0.315 | 0.321 | 0.91 | 0.062 |
| atmosphere_luts | 0.278 | 0.304 | 0.367 | 0.92 | 0.003 |
| water_velocity | 0.255 | 0.279 | 0.347 | 0.91 | 0.002 |
| water_sspr | 0.184 | 0.202 | 0.203 | 0.91 | 0.003 |
| ssao | 0.172 | 0.188 | 0.193 | 0.92 | 0.007 |
| atmos_froxel_inject | 0.133 | 0.146 | 0.147 | 0.91 | 0.001 |
| clouds_trace | 0.128 | 0.140 | 0.232 | 0.91 | 0.001 |
| water_fft | 0.119 | 0.130 | 0.137 | 0.92 | 0.003 |
| uniform_allocation | 0.119 | 0.130 | 0.182 | 0.92 | 0.025 |
| auxiliary_views | 0.109 | 0.119 | 0.435 | 0.92 | 0.120 |
| upscaling | 0.107 | 0.117 | 0.124 | 0.91 | 0.004 |
| clouds_composite | 0.097 | 0.106 | 0.109 | 0.91 | 0.002 |
| bin_unpacking | 0.096 | 0.105 | 0.142 | 0.92 | 0.018 |
| tonemapping | 0.078 | 0.085 | 0.089 | 0.91 | 0.003 |
| main_transmissive_pass_3d | 0.077 | 0.084 | 0.176 | 0.91 | 0.008 |
| clustering | 0.059 | 0.064 | 0.068 | 0.92 | 0.019 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.91 | 0.002 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.91 | 0.001 |
| lightprobe_irradiance_map | 0.041 | 0.412 | 0.416 | 0.10 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.91 | 0.001 |
| surface_trail | 0.036 | 0.056 | 0.057 | 0.64 | 0.002 |
| surface_exposure_far | 0.034 | 0.042 | 0.045 | 0.81 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.92 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.046 | 0.92 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.032 | 0.91 | 0.001 |
| water_snapshot | 0.022 | 0.025 | 0.028 | 0.92 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.219 | 0.229 | 0.10 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.92 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.91 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.91 | 0.122 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.91 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.92 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.92 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.91 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.105 | 10.974 | 40.60 | 55675.5 |
| main/busy/ms | 8.987 | 10.610 | 29.96 | 54955.5 |
| render/busy/ms | 8.543 | 10.471 | 39.99 | 52243.3 |
| stage/main/post_update | 5.585 | 6.857 | 9.86 | 34150.1 |
| stage/render/render | 5.407 | 6.743 | 15.53 | 33062.7 |
| render/graph/ms | 4.653 | 5.996 | 14.37 | 28455.8 |
| encode//sum | 3.841 | 4.962 | 26.13 | 23487.6 |
| stage/render/graph_submit | 3.498 | 4.625 | 10.46 | 21390.4 |
| encode/submit | 2.227 | 3.170 | 6.82 | 13619.2 |
| stage/main/update | 2.143 | 2.561 | 23.41 | 13104.2 |
| encode/finish_wall | 1.348 | 1.946 | 8.14 | 8240.4 |
| stage/render/camera_driver | 1.103 | 1.493 | 3.85 | 6742.2 |
| encode/pass:shadow_directional_light/sum | 0.866 | 1.542 | 3.10 | 5294.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.781 | 1.082 | 1.61 | 4773.0 |
| encode/pass:early prepass/sum | 0.763 | 1.074 | 7.64 | 4662.9 |
| stage/render/prepare_write_phase_buffers | 0.748 | 1.051 | 2.96 | 4571.8 |
| encode/pass:shadow_point_light/sum | 0.726 | 2.830 | 15.88 | 4438.1 |
| bigspace/total/ms | 0.582 | 1.387 | 3.28 | 3561.7 |
| stage/render/prepare_assets | 0.520 | 0.677 | 8.22 | 3178.5 |
| render/present/ms | 0.466 | 1.144 | 2.83 | 2847.8 |
| stage/render/prepare_resources | 0.457 | 0.543 | 1.00 | 2794.4 |
| stage/render/prepare_meshes | 0.419 | 0.524 | 8.60 | 2559.9 |
| stage/main/first | 0.399 | 0.520 | 5.92 | 2439.6 |
| stage/main/pre_update | 0.361 | 0.541 | 1.85 | 2209.5 |
| terrain/update/ms | 0.307 | 0.398 | 1.69 | 1879.4 |
| stage/render/prepare_bind_groups | 0.278 | 0.494 | 2.73 | 1703.0 |
| terrain/select/ms | 0.273 | 0.355 | 0.94 | 1672.3 |
| controller/feed/ms | 0.262 | 0.334 | 0.82 | 1601.4 |
| stream/drive/ms | 0.259 | 0.344 | 21.42 | 1581.4 |
| stage/render/specialize | 0.256 | 0.324 | 12.31 | 1563.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.237 | 0.351 | 1.02 | 1450.7 |
| bigspace/low_precision/ms | 0.230 | 0.895 | 3.06 | 1408.8 |
| stage/render/queue | 0.228 | 0.311 | 7.07 | 1394.5 |
| stage/main/fixed_main_loop | 0.210 | 0.364 | 1.23 | 1286.9 |
| stream/tick/ms | 0.199 | 0.271 | 0.68 | 1219.3 |
| stage/render/create_views | 0.194 | 0.251 | 0.94 | 1187.8 |
| far/select/ms | 0.194 | 0.266 | 0.69 | 1186.8 |
| stage/render/prepare_batch_phases | 0.191 | 0.270 | 0.69 | 1169.8 |
| bigspace/high_precision/ms | 0.183 | 0.935 | 2.14 | 1119.7 |
| anim/evaluate/ms | 0.177 | 0.453 | 1.86 | 1083.5 |
| stage/render/extract_commands | 0.172 | 0.200 | 0.43 | 1052.7 |
| stage/render/prepare_flush | 0.163 | 0.321 | 1.40 | 994.4 |
| anim/evaluate_cpu/ms | 0.151 | 0.190 | 0.79 | 923.1 |
| stage/render/prepare_views | 0.140 | 0.177 | 0.50 | 855.9 |
| controller/move/ms | 0.119 | 0.246 | 0.65 | 725.3 |
| stage/main/last | 0.114 | 0.156 | 1.57 | 695.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.099 | 0.162 | 0.70 | 602.9 |
| render/acquire/ms | 0.096 | 0.125 | 0.45 | 584.4 |
| stage/render/prepare_collect_phase_buffers | 0.064 | 0.133 | 0.45 | 391.2 |
| pipelines/compile_ms | 0.049 | 0.000 | 121.06 | 299.8 |
| pipelines/front_ms | 0.047 | 0.000 | 117.23 | 287.6 |
| physics/step/ms | 0.044 | 0.084 | 0.58 | 269.8 |
| stage/render/cleanup | 0.034 | 0.108 | 0.80 | 207.6 |
| physics/insert/ms | 0.030 | 0.352 | 0.86 | 186.0 |
| stage/render/phase_sort | 0.030 | 0.052 | 0.27 | 185.5 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.92 | 185.5 |
| weather/map/ms | 0.026 | 0.034 | 0.22 | 156.9 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.54 | 90.7 |
| anim/apply/ms | 0.012 | 0.021 | 0.20 | 71.0 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.50 | 68.1 |
| controller/avoid/ms | 0.010 | 0.013 | 0.06 | 62.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.05 | 62.4 |
| script/update/ms | 0.007 | 0.010 | 0.28 | 45.2 |
| stream/despawn/ms | 0.006 | 0.000 | 1.16 | 36.6 |
| script/input/ms | 0.005 | 0.007 | 0.07 | 32.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.18 | 32.0 |
| script/ui/ms | 0.005 | 0.006 | 0.19 | 28.1 |
| controller/write/ms | 0.003 | 0.004 | 0.05 | 19.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 13.8 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.21 | 12.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.11 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.10 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 11 |
| crowd/c0/joints/count | count | 1298 | 2262 | 492 |
| crowd/c0/meshes/count | count | 355 | 646 | 113 |
| crowd/c1/count | count | 145 | 156 | 150 |
| crowd/c1/joints/count | count | 5531 | 5935 | 5704 |
| crowd/c1/meshes/count | count | 1163 | 1259 | 1228 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 237 |
| crowd/joints_extracted/count | count | 4597 | 10.3 k | 6494 |
| crowd/joints_uploaded/count | count | 4578 | 6835 | 6494 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.40 | 7.54 | 0.49 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.54 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.50 | 0.01 |
| encode/finish_critical | ms | 0.97 | 7.64 | 1.43 |
| encode/pass:early prepass/max | ms | 0.78 | 7.64 | 0.87 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.80 | 1.61 | 1.06 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.70 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.02 | 0.30 |
| encode/pass:shadow_directional_light/max | ms | 0.82 | 3.10 | 1.43 |
| encode/pass:shadow_point_light/max | ms | 0.78 | 1.64 | 1.05 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.92 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.20 | 39.99 | 9.30 |
| gpu/timed/ms | ms | 7.87 | 11.32 | 8.14 |
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
| phases/alpha_mask3d/items | count | 817 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 817 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1289 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3414 | 5095 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1289 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3414 | 5095 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 15 | 225 | 28 |
| phases/shadow/bins | count | 5006 | 8920 | 2383 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.3 k | 6324 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 200 | 252 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 182 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 12 | 613 | 74 |
| preprocess/bind_groups_created | count | 12 | 613 | 74 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 218 | 105 |
| process/cpu_usage | % | 27.40 | 41.83 | 26.58 |
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
| skin/blocks_changed/count | count | 12 | 132 | 12 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 237 |
| skin/joint_bytes_written/bytes | bytes | 364.8 k | 808.7 k | 489.0 k |
| skin/joints_written/count | count | 5700 | 12.6 k | 7640 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4916 | 5010 | 5010 |
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
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.05 | 44.23 | 32.18 |
| system/mem_usage | % | 54.52 | 55.06 | 55.04 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 66 | 274 | 78 |
| views/all/bins | count | 7586 | 11.8 k | 5881 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.2 k | 13.9 k |
| views/all/pipelines | count | 247 | 298 | 74 |
| views/all/pipelines_distinct | count | 71 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 35.3 k | 14.2 k |
| views/cascade/batch_sets | count | 9 | 49 | 28 |
| views/cascade/bins | count | 2809 | 6168 | 2383 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8906 | 23.5 k | 6324 |
| views/cascade/pipelines | count | 36 | 53 | 28 |
| views/cascade/work_items | count | 414 | 22.7 k | 6324 |
| views/lamp_face/batch_sets | count | 0 | 190 | 0 |
| views/lamp_face/bins | count | 2148 | 2833 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2407 | 3115 | 0 |
| views/lamp_face/pipelines | count | 164 | 204 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2873 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2882 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8448 | 13.0 k | 7614 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8612 | 13.2 k | 7922 |
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
| anim/bones_written/count | count | 55.58 | 230 | 339.9 k |
| anim/events/count | count | 0.08 | 3 | 472 |
| assets/Image/added | count | 0.10 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.66 | 152 | 4061 |
| assets/Mesh/removed | count | 0.79 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.23 | 10 | 7528 |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.76 | 1303 | 4675 |
| encode//count | count | 32.46 | 220 | 198.5 k |
| encode/bind_walk/init_textures | count | 11.9 k | 119.2 k | 73.01 M |
| encode/bind_walk/init_textures_skipped | count | 64.1 k | 138.7 k | 391.91 M |
| encode/bind_walk/inits | count | 453 | 1200 | 2.77 M |
| encode/bind_walk/inits_skipped | count | 108 | 162 | 658.6 k |
| encode/bind_walk/merge_textures | count | 11.9 k | 119.1 k | 72.57 M |
| encode/bind_walk/merge_textures_skipped | count | 5366 | 6012 | 32.81 M |
| encode/bind_walk/merges | count | 96.91 | 351 | 592.6 k |
| encode/bind_walk/merges_skipped | count | 84.08 | 132 | 514.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6115 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6115 |
| encode/pass:early prepass/count | count | 1 | 1 | 6115 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6115 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6115 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6115 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6115 |
| encode/pass:shadow_point_light/count | count | 1.13 | 48 | 6904 |
| encode/queue/command_buffers | count | 43.60 | 278 | 266.6 k |
| encode/queue/submits | count | 2.02 | 4 | 12.3 k |
| encode/queue/write_buffer/bytes | bytes | 1.87 M | 4.69 M | 11.41 G |
| encode/queue/write_buffer/calls | count | 255 | 914 | 1.56 M |
| encode/queue/write_buffer_with/bytes | bytes | 179.1 k | 3.52 M | 1.09 G |
| encode/queue/write_buffer_with/calls | count | 10.98 | 465 | 67.1 k |
| encode/queue/write_texture/bytes | bytes | 11.7 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6115 |
| physics/inserts | count | 0.37 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2335 |
| physics/steps | count | 0.64 | 3 | 3939 |
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
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 18 | 378 |
| stream/upload/bytes | bytes | 5536 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1214 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.1 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2545.8 | main/busy/ms 2515, stage/main/post_update 2411, stage/main/update 95.57, surface/advance_cpu_ms 66.89, stage/main/pre_update 4.54, stage/main/last 1.12, surface/cell_upload_ms 0.79, stream/drive/ms 0.66, stage/main/first 0.60, stage/main/fixed_main_loop 0.41, stream/tick/ms 0.17, weather/map/ms 0.14 |
| warm-up | 3.30 | 751.4 | main/busy/ms 2656, stage/main/post_update 2438, render/thread/ms 712, render/busy/ms 541, stage/render/prepare_assets 490, stage/main/update 204, stage/render/prepare_views 165, render/acquire/ms 165, surface/advance_cpu_ms 66.89, stage/render/create_views 28.24, stage/render/render 16.35, render/graph/ms 9.28 |
| warm-up | 3.48 | 180.4 | render/thread/ms 873, render/busy/ms 608, stage/render/prepare_assets 538, main/busy/ms 295, stage/render/prepare_views 166, render/acquire/ms 165, pipelines/compile_ms 152, stage/render/render 119, stage/main/update 115, render/present/ms 99.71, pipelines/driver_ms 85.55, stage/main/last 74.39 |
| warm-up | 3.51 | 32.0 | pipelines/compile_ms 546, pipelines/front_ms 342, main/busy/ms 206, pipelines/driver_ms 205, render/thread/ms 189, stage/render/render 123, render/present/ms 103, render/busy/ms 84.79, stage/main/last 80.51, stage/main/post_update 58.80, pipelines/wait_ms 54.89, stage/render/prepare_assets 49.06 |
| warm-up | 3.56 | 54.3 | pipelines/wait_ms 1424, pipelines/compile_ms 1093, pipelines/front_ms 922, main/busy/ms 193, pipelines/driver_ms 171, stage/main/post_update 102, stage/main/update 76.73, render/thread/ms 58.89, render/busy/ms 47.79, pipelines/lock_ms 30.23, stage/render/render 25.14, stage/render/prepare_assets 18.82 |
| warm-up | 3.75 | 185.7 | pipelines/wait_ms 4130, pipelines/compile_ms 2210, pipelines/front_ms 2063, main/busy/ms 169, pipelines/driver_ms 147, render/thread/ms 97.91, render/busy/ms 95.73, stage/main/post_update 82.56, stage/main/update 79.01, stage/render/prepare_assets 69.05, pipelines/lock_ms 25.56, stage/render/render 16.33 |
| warm-up | 3.80 | 47.4 | pipelines/wait_ms 2846, pipelines/compile_ms 2037, pipelines/front_ms 1912, pipelines/driver_ms 124, render/thread/ms 112, render/busy/ms 110, stage/render/prepare_assets 85.61, main/busy/ms 42.90, stage/main/post_update 23.53, terrain/update/ms 16.41, stage/render/render 15.87, render/graph/ms 13.52 |
| warm-up | 3.83 | 30.4 | pipelines/compile_ms 952, pipelines/front_ms 897, pipelines/wait_ms 94.78, render/thread/ms 72.60, render/busy/ms 70.74, pipelines/driver_ms 55.33, stage/render/prepare_assets 52.36, main/busy/ms 34.78, stage/main/post_update 16.47, terrain/update/ms 12.04, stage/render/render 10.48, render/graph/ms 8.23 |
| warm-up | 3.86 | 31.6 | pipelines/compile_ms 1076, pipelines/front_ms 1029, pipelines/wait_ms 116, render/thread/ms 56.50, render/busy/ms 53.96, pipelines/driver_ms 46.52, main/busy/ms 40.69, stage/render/prepare_assets 19.07, encode//sum 17.15, stage/main/post_update 16.73, stage/render/render 15.75, stage/main/update 13.25 |
| warm-up | 3.99 | 82.8 | pipelines/compile_ms 1153, pipelines/front_ms 1106, render/thread/ms 67.83, render/busy/ms 65.57, main/busy/ms 65.34, stage/main/update 50.28, pipelines/driver_ms 46.79, stream/drive/ms 45.44, stage/render/prepare_assets 42.89, stage/render/render 15.89, render/graph/ms 13.48, pipelines/wait_ms 12.00 |
| warm-up | 4.05 | 32.2 | pipelines/compile_ms 566, pipelines/front_ms 538, render/thread/ms 42.69, main/busy/ms 41.92, render/busy/ms 40.86, pipelines/driver_ms 28.04, stage/main/update 24.77, stage/render/prepare_assets 15.45, stream/drive/ms 13.25, stage/render/render 13.16, render/graph/ms 11.15, stage/main/post_update 10.40 |
| warm-up | 4.09 | 35.5 | main/busy/ms 55.70, render/thread/ms 51.15, render/busy/ms 49.60, pipelines/compile_ms 43.26, pipelines/front_ms 38.32, stage/main/update 33.50, stream/drive/ms 27.34, stage/render/prepare_assets 27.12, stage/main/post_update 16.36, stage/render/render 15.77, render/graph/ms 13.66, stage/render/graph_submit 7.48 |
| warm-up | 4.17 | 83.0 | pipelines/compile_ms 269, pipelines/front_ms 259, render/thread/ms 84.13, render/busy/ms 82.48, main/busy/ms 65.87, stage/render/prepare_assets 57.45, stage/main/update 36.14, stream/drive/ms 29.26, stage/main/post_update 21.08, stage/render/render 17.56, render/graph/ms 15.27, pipelines/driver_ms 10.04 |
| warm-up | 4.22 | 52.2 | pipelines/compile_ms 1171, pipelines/front_ms 1128, render/thread/ms 83.79, render/busy/ms 81.90, main/busy/ms 74.63, stage/render/prepare_assets 61.70, stage/main/update 49.67, pipelines/driver_ms 42.86, stage/main/post_update 13.41, stage/render/render 13.16, render/graph/ms 10.63, stage/render/graph_submit 7.72 |
| warm-up | 4.36 | 136.5 | pipelines/compile_ms 1838, pipelines/front_ms 1780, render/thread/ms 120, render/busy/ms 118, main/busy/ms 72.47, pipelines/driver_ms 58.57, stage/render/prepare_assets 58.43, stage/main/update 50.46, stage/render/prepare_bind_groups 34.29, pipelines/wait_ms 16.52, stage/render/render 16.13, pipelines/lock_ms 15.84 |
| warm-up | 4.42 | 64.0 | pipelines/compile_ms 1851, pipelines/front_ms 1746, pipelines/wait_ms 179, render/thread/ms 146, render/busy/ms 145, pipelines/driver_ms 105, stage/render/prepare_assets 80.52, main/busy/ms 54.27, stage/render/prepare_bind_groups 35.35, stage/main/update 30.89, pipelines/lock_ms 18.16, stage/render/render 16.00 |
| warm-up | 4.48 | 56.3 | pipelines/compile_ms 999, pipelines/front_ms 917, pipelines/wait_ms 396, render/thread/ms 98.89, render/busy/ms 96.93, pipelines/driver_ms 81.95, main/busy/ms 55.91, stage/render/prepare_assets 55.53, encode//sum 41.11, stage/main/update 34.69, stage/render/render 20.95, render/graph/ms 18.58 |
| warm-up | 4.57 | 87.8 | pipelines/compile_ms 1479, pipelines/front_ms 1431, pipelines/wait_ms 260, render/thread/ms 124, render/busy/ms 122, encode//sum 60.46, pipelines/driver_ms 47.64, stage/render/prepare_assets 32.26, main/busy/ms 31.99, stage/render/render 28.29, render/graph/ms 25.70, stage/render/prepare_resources 24.92 |
| warm-up | 4.62 | 52.9 | pipelines/compile_ms 2237, pipelines/front_ms 2140, pipelines/wait_ms 1361, render/thread/ms 134, render/busy/ms 132, pipelines/driver_ms 97.46, encode//sum 50.75, main/busy/ms 35.67, stage/render/render 30.79, render/graph/ms 27.79, stage/render/prepare_resources 26.91, stage/render/prepare_assets 21.08 |
| warm-up | 4.67 | 45.9 | pipelines/wait_ms 1755, pipelines/compile_ms 1609, pipelines/front_ms 1535, render/thread/ms 84.50, render/busy/ms 82.74, pipelines/driver_ms 74.33, main/busy/ms 40.19, encode//sum 29.74, stage/render/render 23.74, stage/main/update 23.73, stage/render/prepare_assets 23.61, render/graph/ms 21.19 |
| warm-up | 4.78 | 91.7 | pipelines/compile_ms 1023, pipelines/front_ms 991, pipelines/wait_ms 601, main/busy/ms 108, stage/main/update 79.88, render/thread/ms 68.55, render/busy/ms 66.96, stream/drive/ms 50.02, pipelines/driver_ms 32.13, stage/render/prepare_resources 22.22, stage/render/prepare_assets 22.16, stage/render/render 12.39 |
| warm-up | 4.83 | 45.9 | pipelines/compile_ms 1793, pipelines/front_ms 1743, pipelines/wait_ms 1088, main/busy/ms 131, stage/main/update 104, render/thread/ms 76.07, render/busy/ms 74.41, stream/drive/ms 53.41, pipelines/driver_ms 49.46, stage/render/prepare_assets 28.45, stage/render/prepare_resources 21.61, stage/render/render 15.59 |
| warm-up | 4.96 | 127.6 | pipelines/compile_ms 1692, pipelines/front_ms 1645, pipelines/wait_ms 554, main/busy/ms 99.87, render/thread/ms 81.85, render/busy/ms 80.37, stage/main/update 79.85, stream/drive/ms 53.31, pipelines/driver_ms 46.91, stage/render/prepare_assets 35.81, stage/render/render 28.96, render/graph/ms 26.64 |
| warm-up | 5.01 | 50.6 | pipelines/compile_ms 914, pipelines/front_ms 884, render/thread/ms 86.68, render/busy/ms 85.18, main/busy/ms 79.37, stage/main/update 58.92, stream/drive/ms 45.99, stage/render/prepare_assets 39.63, pipelines/driver_ms 30.22, stage/render/render 26.95, render/graph/ms 24.71, stage/render/graph_submit 12.85 |
| warm-up | 5.08 | 71.9 | pipelines/compile_ms 1465, pipelines/front_ms 1425, main/busy/ms 91.44, stage/main/update 66.70, render/thread/ms 65.90, render/busy/ms 64.44, stream/drive/ms 46.37, pipelines/driver_ms 40.30, stage/render/prepare_assets 40.10, stage/main/post_update 15.75, stage/render/render 12.93, render/graph/ms 10.60 |
| warm-up | 5.11 | 33.2 | pipelines/compile_ms 1282, pipelines/front_ms 1245, main/busy/ms 75.37, stage/main/update 55.69, render/thread/ms 54.52, render/busy/ms 53.17, pipelines/driver_ms 36.91, stream/drive/ms 35.69, stage/render/prepare_assets 28.85, stage/main/post_update 13.33, stage/render/render 12.68, render/graph/ms 10.67 |
| warm-up | 5.17 | 54.8 | pipelines/compile_ms 410, pipelines/front_ms 400, main/busy/ms 65.98, stage/main/update 52.16, render/thread/ms 46.46, render/busy/ms 45.11, stream/drive/ms 35.49, stage/render/prepare_assets 18.03, stage/render/render 12.32, render/graph/ms 10.37, pipelines/driver_ms 9.55, encode//sum 8.24 |
| warm-up | 5.25 | 28.8 | pipelines/compile_ms 438, pipelines/front_ms 422, render/thread/ms 44.04, render/busy/ms 42.56, main/busy/ms 35.11, pipelines/driver_ms 15.96, stage/main/update 15.23, stage/main/post_update 13.00, stage/render/render 12.75, render/graph/ms 10.73, stage/render/prepare_assets 9.77, encode//sum 8.88 |
| warm-up | 5.34 | 46.9 | pipelines/compile_ms 482, pipelines/front_ms 466, render/thread/ms 46.31, render/busy/ms 44.42, main/busy/ms 39.62, stage/render/prepare_assets 22.62, stage/main/update 20.62, pipelines/driver_ms 15.78, stage/render/render 14.28, stage/main/post_update 11.90, render/graph/ms 11.84, stream/drive/ms 9.17 |
| warm-up | 5.38 | 47.6 | pipelines/compile_ms 409, pipelines/front_ms 396, render/thread/ms 66.55, render/busy/ms 65.08, main/busy/ms 42.89, stage/render/prepare_assets 38.07, stage/main/update 22.96, stream/drive/ms 14.79, stage/main/post_update 14.63, pipelines/driver_ms 12.68, stage/render/render 12.14, render/graph/ms 10.09 |
| warm-up | 5.42 | 32.2 | pipelines/compile_ms 757, pipelines/front_ms 735, render/thread/ms 65.82, render/busy/ms 64.40, main/busy/ms 48.17, stage/render/prepare_assets 29.59, stage/main/post_update 21.66, pipelines/driver_ms 21.61, stage/main/update 20.95, stream/drive/ms 13.71, stage/render/render 11.25, render/graph/ms 9.26 |
| warm-up | 6.12 | 30.9 | pipelines/compile_ms 172, pipelines/front_ms 169, render/thread/ms 42.61, render/busy/ms 40.60, main/busy/ms 34.67, stage/render/render 17.67, stage/main/post_update 15.17, render/graph/ms 14.97, stage/main/update 14.92, encode//sum 11.28, stage/render/graph_submit 10.26, encode/submit 7.05 |
| warm-up | 6.18 | 31.6 | pipelines/compile_ms 669, pipelines/front_ms 647, render/thread/ms 42.20, render/busy/ms 41.13, main/busy/ms 28.06, pipelines/driver_ms 21.98, stage/render/prepare_assets 15.84, stage/main/update 11.14, stage/main/post_update 9.92, stage/render/render 9.80, render/graph/ms 8.32, encode//sum 7.40 |
| warm-up | 6.41 | 31.0 | render/thread/ms 48.65, render/busy/ms 47.35, main/busy/ms 41.88, stage/main/update 19.70, stage/render/render 16.01, pipelines/compile_ms 15.93, stage/main/post_update 15.64, pipelines/front_ms 14.59, render/graph/ms 14.19, stage/render/prepare_assets 12.57, stage/render/graph_submit 10.63, encode//sum 9.56 |
| warm-up | 6.73 | 36.1 | render/thread/ms 55.49, render/busy/ms 54.30, main/busy/ms 40.41, stage/render/prepare_assets 32.99, stage/main/update 23.74, stream/drive/ms 14.03, stage/render/render 11.80, stage/main/post_update 11.71, render/graph/ms 9.70, encode//sum 8.69, stage/render/graph_submit 6.50, encode/submit 3.94 |
| warm-up | 6.96 | 33.7 | pipelines/compile_ms 145, pipelines/front_ms 142, main/busy/ms 40.57, render/thread/ms 24.70, render/busy/ms 23.53, stage/main/update 20.62, stage/main/post_update 14.17, stream/drive/ms 13.66, stream/spawn/ms 12.47, stage/render/render 11.66, render/graph/ms 10.00, encode//sum 9.37 |
| warm-up | 7.06 | 32.2 | render/thread/ms 43.77, render/busy/ms 42.72, main/busy/ms 20.74, stage/main/post_update 11.31, stage/render/render 11.24, stage/render/specialize 10.37, render/graph/ms 9.64, encode//sum 8.08, stage/render/graph_submit 7.11, stage/render/queue 7.09, stage/main/update 6.38, stage/render/prepare_assets 4.97 |
| warm-up | 8.38 | 34.9 | render/thread/ms 39.75, render/busy/ms 38.56, main/busy/ms 18.58, stage/render/specialize 12.07, stage/render/render 10.99, stage/main/post_update 10.52, render/graph/ms 9.43, encode//sum 7.94, stage/render/graph_submit 6.99, stage/render/queue 6.62, stage/main/update 5.05, encode/submit 3.89 |
| measure | 1.75 | 32.4 | render/thread/ms 35.51, render/busy/ms 34.55, main/busy/ms 18.05, stage/main/post_update 11.68, stage/render/render 10.37, stage/render/specialize 9.18, render/graph/ms 8.88, encode//sum 7.92, stage/render/graph_submit 6.65, stage/render/queue 5.79, stage/render/prepare_meshes 4.60, stage/main/update 4.05 |
| measure | 3.13 | 35.2 | render/thread/ms 41.37, render/busy/ms 40.41, main/busy/ms 20.79, stage/main/post_update 14.54, stage/render/render 10.48, render/graph/ms 8.75, stage/render/specialize 8.64, stage/render/prepare_assets 7.31, encode//sum 7.17, stage/render/queue 6.37, stage/render/graph_submit 6.19, stage/main/update 3.82 |
| measure | 4.48 | 30.4 | render/thread/ms 35.54, render/busy/ms 34.48, main/busy/ms 19.44, stage/main/post_update 12.14, stage/render/render 10.41, stage/render/specialize 9.18, render/graph/ms 8.87, encode//sum 7.39, stage/render/queue 6.71, stage/render/graph_submit 6.69, stage/main/update 3.94, encode/submit 3.90 |
| measure | 5.82 | 28.9 | render/thread/ms 34.61, render/busy/ms 33.37, main/busy/ms 19.21, stage/main/post_update 12.36, stage/render/render 11.05, render/graph/ms 9.41, stage/render/specialize 8.72, encode//sum 8.60, stage/render/graph_submit 7.22, stage/render/queue 5.72, encode/submit 4.31, stage/main/update 3.80 |
| measure | 7.15 | 33.8 | render/thread/ms 40.15, render/busy/ms 39.07, main/busy/ms 20.17, stage/render/render 12.93, render/graph/ms 11.37, stage/main/post_update 11.23, stage/render/specialize 10.19, encode//sum 9.70, stage/render/graph_submit 8.83, stage/render/queue 6.84, stage/main/update 5.60, encode/submit 5.02 |
| measure | 8.45 | 30.2 | render/thread/ms 36.13, render/busy/ms 35.04, main/busy/ms 18.33, stage/render/render 11.82, stage/main/post_update 11.39, encode//sum 10.21, render/graph/ms 10.17, stage/render/specialize 9.30, stage/render/graph_submit 7.51, stage/render/queue 6.17, stage/main/update 4.52, encode/submit 4.45 |
| measure | 9.74 | 30.0 | render/thread/ms 40.29, render/busy/ms 39.01, main/busy/ms 19.69, stage/main/post_update 12.13, stage/render/render 11.81, render/graph/ms 10.00, stage/render/specialize 9.99, encode//sum 7.95, stage/render/graph_submit 7.73, stage/render/queue 6.25, stage/main/update 4.11, encode/submit 4.08 |
| measure | 11.06 | 29.9 | render/thread/ms 35.85, render/busy/ms 34.75, main/busy/ms 18.53, stage/main/post_update 11.64, stage/render/render 10.78, stage/render/specialize 9.50, render/graph/ms 9.21, encode//sum 8.82, stage/render/graph_submit 6.92, stage/render/queue 6.48, encode/submit 4.20, stage/main/update 4.08 |
| measure | 15.73 | 32.5 | render/thread/ms 37.47, render/busy/ms 36.14, main/busy/ms 19.29, stage/main/post_update 12.17, stage/render/render 11.29, stage/render/specialize 9.78, render/graph/ms 9.63, encode//sum 8.78, stage/render/graph_submit 7.18, stage/render/queue 7.00, encode/submit 4.33, stage/main/update 4.11 |
| measure | 18.00 | 28.9 | render/thread/ms 36.52, render/busy/ms 35.36, main/busy/ms 18.15, stage/main/post_update 10.97, stage/render/render 10.67, stage/render/specialize 9.54, render/graph/ms 9.04, encode//sum 7.67, stage/render/graph_submit 7.06, stage/render/queue 6.33, encode/submit 4.45, stage/main/update 3.99 |
| measure | 18.17 | 31.2 | main/busy/ms 39.07, stage/main/update 25.47, stream/drive/ms 21.67, render/thread/ms 19.56, render/busy/ms 18.58, stage/render/render 11.46, stage/main/post_update 10.94, render/graph/ms 10.03, encode//sum 8.82, stage/render/graph_submit 7.48, encode/submit 5.13, encode/pass:shadow_point_light/sum 2.96 |
| measure | 19.38 | 30.6 | render/thread/ms 38.78, render/busy/ms 37.63, main/busy/ms 19.04, stage/main/post_update 12.16, stage/render/render 11.81, render/graph/ms 10.20, stage/render/specialize 10.05, encode//sum 9.54, stage/render/graph_submit 7.78, stage/render/queue 6.88, encode/submit 4.86, stage/main/update 4.16 |
| measure | 20.77 | 30.1 | render/thread/ms 36.92, render/busy/ms 35.84, main/busy/ms 18.56, stage/render/render 11.70, stage/main/post_update 11.17, encode//sum 10.59, render/graph/ms 10.28, stage/render/specialize 9.35, stage/render/graph_submit 7.77, stage/render/queue 6.80, encode/submit 4.61, stage/main/update 4.59 |
| measure | 23.00 | 32.9 | render/thread/ms 39.60, render/busy/ms 38.48, main/busy/ms 24.67, stage/main/post_update 12.29, stage/render/specialize 12.02, stage/render/render 10.87, render/graph/ms 9.38, encode//sum 8.46, stage/render/graph_submit 7.39, stage/render/queue 7.26, stage/main/update 6.20, encode/submit 4.24 |
| measure | 27.28 | 36.4 | render/thread/ms 43.16, render/busy/ms 42.09, main/busy/ms 26.23, stage/render/render 14.64, stage/main/post_update 13.47, render/graph/ms 12.96, stage/render/specialize 11.88, encode//sum 11.80, stage/render/graph_submit 9.83, encode/pass:shadow_point_light/sum 7.56, stage/render/queue 7.20, stage/main/update 6.57 |
| measure | 28.67 | 29.0 | render/thread/ms 35.61, render/busy/ms 34.64, main/busy/ms 18.49, stage/main/post_update 11.69, stage/render/render 10.95, render/graph/ms 9.57, stage/render/specialize 9.48, encode//sum 7.44, stage/render/graph_submit 7.17, stage/render/queue 5.98, stage/main/update 4.28, encode/submit 3.94 |
| measure | 30.48 | 31.9 | render/thread/ms 37.20, render/busy/ms 36.07, main/busy/ms 21.20, stage/main/post_update 12.69, stage/render/render 10.57, stage/render/specialize 10.32, render/graph/ms 9.08, stage/render/graph_submit 7.21, encode//sum 7.12, stage/render/queue 6.59, stage/main/update 4.92, encode/submit 3.86 |
| measure | 35.35 | 30.8 | render/thread/ms 38.70, render/busy/ms 37.72, main/busy/ms 21.43, stage/main/post_update 13.15, stage/render/render 12.01, render/graph/ms 10.64, stage/render/specialize 10.59, encode//sum 8.91, stage/render/graph_submit 8.18, stage/render/queue 6.66, stage/main/update 4.73, encode/submit 4.55 |
| measure | 36.72 | 29.4 | render/thread/ms 38.59, render/busy/ms 37.50, main/busy/ms 19.96, stage/main/post_update 12.89, stage/render/render 12.30, render/graph/ms 10.60, stage/render/specialize 9.57, stage/render/graph_submit 7.89, encode//sum 7.87, stage/render/queue 6.89, encode/submit 4.79, stage/main/update 4.33 |
| measure | 37.48 | 29.4 | render/thread/ms 34.68, render/busy/ms 33.61, main/busy/ms 19.00, stage/main/post_update 11.99, stage/render/render 10.36, stage/render/specialize 9.07, render/graph/ms 8.77, encode//sum 8.40, stage/render/queue 6.60, stage/render/graph_submit 6.56, stage/main/update 4.09, encode/submit 3.95 |
| measure | 40.80 | 34.0 | render/thread/ms 40.02, render/busy/ms 38.89, main/busy/ms 21.64, stage/main/post_update 12.17, stage/render/specialize 10.67, stage/render/render 10.42, render/graph/ms 8.85, encode//sum 8.49, stage/render/graph_submit 6.92, stage/render/queue 6.69, stage/main/update 5.31, encode/submit 4.13 |
| measure | 43.20 | 30.2 | render/thread/ms 37.68, render/busy/ms 36.74, main/busy/ms 16.44, stage/render/render 10.66, stage/render/specialize 9.78, stage/main/post_update 9.74, render/graph/ms 9.33, stage/render/graph_submit 7.65, encode//sum 7.00, stage/render/queue 6.80, stage/main/update 4.11, encode/submit 3.89 |
| measure | 45.62 | 32.8 | render/thread/ms 38.60, render/busy/ms 37.49, main/busy/ms 18.11, stage/render/render 10.35, stage/render/specialize 10.27, stage/main/post_update 10.25, render/graph/ms 8.94, encode//sum 7.07, stage/render/graph_submit 7.05, stage/render/queue 6.98, stage/main/update 4.70, stage/render/prepare_meshes 4.34 |
| measure | 56.44 | 28.7 | render/thread/ms 36.45, render/busy/ms 35.41, main/busy/ms 17.00, stage/main/post_update 10.93, stage/render/render 10.76, stage/render/specialize 9.57, render/graph/ms 9.45, encode//sum 7.48, stage/render/graph_submit 7.43, stage/render/queue 6.57, encode/submit 4.04, stage/main/update 3.83 |
| measure | 59.03 | 45.8 | render/thread/ms 56.79, render/busy/ms 55.43, main/busy/ms 30.82, stage/main/post_update 18.28, stage/render/render 16.98, render/graph/ms 15.11, stage/render/specialize 12.52, stage/render/graph_submit 11.91, encode//sum 11.83, stage/render/prepare_meshes 8.94, stage/main/update 8.42, stage/render/queue 6.80 |
| measure | 62.59 | 31.5 | render/thread/ms 39.06, render/busy/ms 38.05, main/busy/ms 18.86, stage/render/render 11.39, stage/render/specialize 10.09, render/graph/ms 10.02, stage/main/post_update 9.81, encode//sum 7.90, stage/render/graph_submit 7.86, stage/main/update 6.39, stage/render/queue 6.28, stage/render/prepare_meshes 4.77 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set mw.matctrl.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=matctrl-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 2%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **78%** (min 41%) · 2620 MHz (min 2610) · memory 8101 MHz · 81.2 W · 78 °C (max 83) · VRAM 7056 MiB · throttled 100% of samples

## Tunables

- `far.handoff_scale=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.bias=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_distance_m=24 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_px=2 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `mw.matctrl.gate=on (command line: --set)`
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
