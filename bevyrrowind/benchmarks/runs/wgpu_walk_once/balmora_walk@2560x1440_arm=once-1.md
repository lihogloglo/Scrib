# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); render.wgpu.bind_group_walk_once=on (command line: --set); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5606 frames over 65.7 s · **median 11.26 ms (89 fps)** · mean 11.71 · p95 14.90 · p99 19.14 (1% low 52 fps) · worst 44.79 ms · 125 spikes over 1.5× median · 61.2 stutters/min (work 48.4/min)
- warm-up: 322 frames over 15.0 s, worst 2655.4 ms
- hitches (> 28.2 ms): **36** measured, 153 in warm-up
- timed GPU passes: 6.94 ms of a 11.71 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.26 ms, limit 11 ms
- **FAIL** frame_ms: 67 stutters (> 1.5x moving median of 60) in 1.09 min = 61.23/min, limit 1/min
- **FAIL** worst frame 44.79 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.961 | 2.035 | 2.067 | 0.96 | 0.184 |
| main_opaque_pass_3d | 1.586 | 1.646 | 2.044 | 0.96 | 0.079 |
| early prepass | 0.486 | 0.505 | 0.736 | 0.96 | 0.095 |
| bloom | 0.304 | 0.315 | 0.323 | 0.96 | 0.067 |
| atmosphere_luts | 0.293 | 0.304 | 0.367 | 0.96 | 0.004 |
| water_velocity | 0.257 | 0.267 | 0.347 | 0.96 | 0.002 |
| water_sspr | 0.202 | 0.210 | 0.211 | 0.96 | 0.003 |
| ssao | 0.182 | 0.189 | 0.193 | 0.96 | 0.008 |
| atmos_froxel_inject | 0.141 | 0.146 | 0.148 | 0.96 | 0.002 |
| clouds_trace | 0.135 | 0.141 | 0.230 | 0.96 | 0.001 |
| uniform_allocation | 0.127 | 0.132 | 0.183 | 0.96 | 0.030 |
| water_fft | 0.125 | 0.130 | 0.138 | 0.96 | 0.003 |
| auxiliary_views | 0.115 | 0.120 | 0.522 | 0.96 | 0.131 |
| upscaling | 0.113 | 0.117 | 0.124 | 0.96 | 0.005 |
| bin_unpacking | 0.103 | 0.106 | 0.142 | 0.96 | 0.022 |
| clouds_composite | 0.102 | 0.106 | 0.109 | 0.96 | 0.002 |
| tonemapping | 0.082 | 0.085 | 0.089 | 0.96 | 0.004 |
| main_transmissive_pass_3d | 0.078 | 0.081 | 0.162 | 0.96 | 0.009 |
| clustering | 0.062 | 0.065 | 0.069 | 0.96 | 0.020 |
| render_sky | 0.050 | 0.052 | 0.053 | 0.96 | 0.002 |
| lightprobe_irradiance_map | 0.046 | 0.415 | 0.416 | 0.11 | 0.001 |
| clouds_resolve | 0.046 | 0.048 | 0.049 | 0.96 | 0.001 |
| surface_exposure | 0.042 | 0.044 | 0.045 | 0.96 | 0.001 |
| surface_trail | 0.040 | 0.056 | 0.057 | 0.72 | 0.003 |
| surface_exposure_far | 0.036 | 0.042 | 0.045 | 0.85 | 0.003 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.96 | 0.004 |
| early_mesh_preprocessing | 0.030 | 0.031 | 0.047 | 0.96 | 0.006 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.031 | 0.96 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.219 | 0.230 | 0.11 | 0.005 |
| water_snapshot | 0.024 | 0.025 | 0.028 | 0.96 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.96 | 0.003 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.96 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.026 | 0.96 | 0.128 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.96 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.96 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.96 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.96 | 0.004 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/compile_ms | 16.262 | 0.000 | 6403.54 | 91162.7 |
| pipelines/driver_ms | 14.288 | 0.000 | 5968.05 | 80095.8 |
| pipelines/wait_ms | 11.168 | 0.000 | 18302.83 | 62608.7 |
| render/thread/ms | 10.167 | 13.084 | 40.31 | 56993.7 |
| render/busy/ms | 9.678 | 12.578 | 39.70 | 54253.6 |
| main/busy/ms | 9.620 | 12.010 | 29.20 | 53927.1 |
| encode//sum | 7.697 | 10.890 | 48.56 | 43148.7 |
| stage/main/post_update | 5.915 | 7.612 | 12.87 | 33159.0 |
| stage/render/render | 5.765 | 7.549 | 21.97 | 32316.8 |
| render/graph/ms | 5.069 | 6.770 | 20.76 | 28418.8 |
| stage/render/graph_submit | 3.812 | 5.103 | 13.85 | 21367.3 |
| encode/submit | 2.395 | 3.378 | 9.09 | 13425.5 |
| stage/main/update | 2.338 | 2.952 | 22.62 | 13104.2 |
| pipelines/front_ms | 1.974 | 0.000 | 435.49 | 11066.8 |
| encode/finish_wall | 1.456 | 2.134 | 5.29 | 8159.7 |
| stage/render/camera_driver | 1.201 | 1.699 | 10.11 | 6734.1 |
| stage/render/prepare_write_phase_buffers | 0.837 | 1.231 | 3.32 | 4693.5 |
| stage/render/prepare_assets | 0.643 | 0.836 | 11.03 | 3606.5 |
| bigspace/total/ms | 0.538 | 1.296 | 2.42 | 3015.3 |
| stage/render/prepare_meshes | 0.492 | 0.637 | 7.18 | 2760.7 |
| stage/render/prepare_resources | 0.480 | 0.604 | 1.19 | 2690.8 |
| stage/render/specialize | 0.461 | 0.651 | 14.11 | 2585.5 |
| stage/main/first | 0.428 | 0.584 | 6.89 | 2398.0 |
| render/present/ms | 0.390 | 0.525 | 2.50 | 2187.9 |
| stage/main/pre_update | 0.380 | 0.552 | 1.73 | 2129.6 |
| stage/render/queue | 0.326 | 0.477 | 8.97 | 1828.6 |
| stage/render/prepare_bind_groups | 0.314 | 0.589 | 2.99 | 1762.7 |
| terrain/update/ms | 0.311 | 0.403 | 1.91 | 1742.4 |
| controller/feed/ms | 0.291 | 0.379 | 0.97 | 1632.7 |
| terrain/select/ms | 0.275 | 0.356 | 1.02 | 1542.0 |
| stream/drive/ms | 0.264 | 0.383 | 20.47 | 1477.7 |
| stage/main/fixed_main_loop | 0.246 | 0.429 | 1.32 | 1376.8 |
| bigspace/low_precision/ms | 0.225 | 0.669 | 1.80 | 1258.9 |
| far/select/ms | 0.216 | 0.323 | 0.86 | 1210.5 |
| stage/render/create_views | 0.213 | 0.286 | 1.25 | 1192.7 |
| stage/render/prepare_batch_phases | 0.204 | 0.295 | 0.91 | 1145.5 |
| stream/tick/ms | 0.202 | 0.287 | 1.25 | 1130.6 |
| stage/render/extract_commands | 0.185 | 0.228 | 0.49 | 1037.4 |
| stage/render/prepare_flush | 0.173 | 0.353 | 1.05 | 971.1 |
| anim/evaluate/ms | 0.165 | 0.278 | 2.82 | 925.8 |
| anim/evaluate_cpu/ms | 0.159 | 0.212 | 1.06 | 893.5 |
| bigspace/high_precision/ms | 0.148 | 0.577 | 1.87 | 831.4 |
| stage/render/prepare_views | 0.144 | 0.186 | 0.74 | 805.9 |
| controller/move/ms | 0.139 | 0.288 | 0.60 | 781.7 |
| stage/main/last | 0.126 | 0.201 | 2.10 | 708.4 |
| render/acquire/ms | 0.098 | 0.129 | 0.67 | 552.2 |
| stage/render/prepare_collect_phase_buffers | 0.070 | 0.147 | 0.43 | 394.9 |
| physics/step/ms | 0.055 | 0.097 | 0.50 | 306.4 |
| physics/insert/ms | 0.037 | 0.392 | 1.08 | 208.8 |
| encode/resolve_diagnostics/sum | 0.036 | 0.048 | 0.59 | 200.3 |
| stage/render/cleanup | 0.035 | 0.103 | 3.16 | 197.3 |
| weather/map/ms | 0.027 | 0.036 | 0.31 | 148.6 |
| stage/render/phase_sort | 0.026 | 0.046 | 0.28 | 148.0 |
| encode/camera_driver_span_begin/sum | 0.017 | 0.025 | 0.59 | 96.0 |
| anim/apply/ms | 0.013 | 0.023 | 0.43 | 72.9 |
| encode/camera_driver_span_end/sum | 0.013 | 0.022 | 0.58 | 70.7 |
| weather/eval/ms | 0.012 | 0.015 | 0.14 | 64.6 |
| controller/avoid/ms | 0.011 | 0.015 | 0.10 | 63.4 |
| script/update/ms | 0.008 | 0.010 | 0.09 | 43.6 |
| stream/despawn/ms | 0.006 | 0.000 | 1.03 | 31.9 |
| stream/spawn/ms | 0.005 | 0.000 | 1.15 | 30.2 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 27.9 |
| script/ui/ms | 0.005 | 0.007 | 0.12 | 27.0 |
| controller/write/ms | 0.003 | 0.004 | 0.11 | 19.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.73 | 14.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.45 | 5.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 4.7 |
| stage/render/prepare | 0.001 | 0.003 | 0.18 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.12 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 42 | 12 |
| crowd/c0/joints/count | count | 1301 | 2204 | 547 |
| crowd/c0/meshes/count | count | 353 | 634 | 140 |
| crowd/c1/count | count | 147 | 156 | 149 |
| crowd/c1/joints/count | count | 5545 | 5995 | 5649 |
| crowd/c1/meshes/count | count | 1190 | 1297 | 1201 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4621 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4603 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.05 | 4.06 | 1.16 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.59 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.58 | 0.01 |
| encode/finish_critical | ms | 1.05 | 4.06 | 1.16 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.59 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.79 | 39.70 | 9.27 |
| gpu/timed/ms | ms | 7.89 | 11.31 | 8.23 |
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
| phases/alpha_mask3d/items | count | 802 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 802 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1292 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3380 | 5130 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1292 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3380 | 5130 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 14 | 228 | 29 |
| phases/shadow/bins | count | 5002 | 8804 | 2405 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.3 k | 26.3 k | 6366 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 202 | 252 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 184 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 263 | 325 | 325 |
| pipelines/waiting | count | 1 | 9 | 0 |
| precache/draw_waiting | count | 0 | 7 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 1 | 2 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 32 | 75 | 0 |
| preprocess/bind_groups | count | 13 | 619 | 61 |
| preprocess/bind_groups_created | count | 13 | 619 | 61 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 128 | 249 | 120 |
| process/cpu_usage | % | 32.21 | 78.85 | 28.37 |
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
| skin/blocks_changed/count | count | 14 | 125 | 14 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 201 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 369.9 k | 810.2 k | 495.1 k |
| skin/joints_written/count | count | 5780 | 12.7 k | 7736 |
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
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.03 |
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
| system/cpu_usage | % | 38.88 | 92.88 | 31.81 |
| system/mem_usage | % | 73.58 | 74.41 | 74.19 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 64 | 277 | 79 |
| views/all/bins | count | 7562 | 11.7 k | 5903 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.2 k | 14.0 k |
| views/all/pipelines | count | 248 | 299 | 75 |
| views/all/pipelines_distinct | count | 71 | 74 | 61 |
| views/all/work_items | count | 10.6 k | 35.4 k | 14.3 k |
| views/cascade/batch_sets | count | 9 | 50 | 29 |
| views/cascade/bins | count | 2795 | 6157 | 2405 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8901 | 23.5 k | 6366 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 400 | 22.5 k | 6366 |
| views/lamp_face/batch_sets | count | 0 | 191 | 0 |
| views/lamp_face/bins | count | 2142 | 2694 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2402 | 3012 | 0 |
| views/lamp_face/pipelines | count | 165 | 204 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2855 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2876 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8370 | 13.0 k | 7618 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8538 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 54.16 | 261 | 303.6 k |
| anim/events/count | count | 0.10 | 3 | 567 |
| assets/Image/added | count | 0.11 | 43 | 592 |
| assets/Image/modified | count | 0.00 | 3 | 17 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.72 | 152 | 4061 |
| assets/Mesh/removed | count | 0.86 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.86 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 36.46 | 1667 | 204.4 k |
| ecs/entities_delta | count | 0.83 | 1274 | 4674 |
| encode//count | count | 38.65 | 275 | 216.7 k |
| encode/bind_walk/init_textures | count | 12.1 k | 119.2 k | 67.66 M |
| encode/bind_walk/init_textures_skipped | count | 73.7 k | 172.7 k | 412.92 M |
| encode/bind_walk/inits | count | 456 | 1220 | 2.55 M |
| encode/bind_walk/inits_skipped | count | 115 | 183 | 643.3 k |
| encode/bind_walk/merge_textures | count | 12.0 k | 119.1 k | 67.26 M |
| encode/bind_walk/merge_textures_skipped | count | 5379 | 6012 | 30.16 M |
| encode/bind_walk/merges | count | 96.99 | 353 | 543.7 k |
| encode/bind_walk/merges_skipped | count | 87.25 | 132 | 489.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5606 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5606 |
| encode/queue/command_buffers | count | 43.67 | 280 | 244.8 k |
| encode/queue/submits | count | 2.02 | 4 | 11.3 k |
| encode/queue/write_buffer/bytes | bytes | 1.90 M | 4.89 M | 10.63 G |
| encode/queue/write_buffer/calls | count | 256 | 924 | 1.44 M |
| encode/queue/write_buffer_with/bytes | bytes | 180.3 k | 3.52 M | 1.01 G |
| encode/queue/write_buffer_with/calls | count | 11.16 | 465 | 62.5 k |
| encode/queue/write_texture/bytes | bytes | 11.3 k | 8.39 M | 63.53 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5606 |
| physics/inserts | count | 0.41 | 111 | 2289 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 145 | 2335 |
| physics/steps | count | 0.70 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 10 |
| pipelines/created | count | 0.02 | 2 | 118 |
| precache/hit | count | 0.00 | 2 | 3 |
| precache/missed | count | 0.00 | 2 | 7 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 25 | 378 |
| stream/upload/bytes | bytes | 6039 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 28 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1324 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.2 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.66 | 2655.4 | main/busy/ms 2626, stage/main/post_update 2523, stage/main/update 95.19, surface/advance_cpu_ms 68.03, stage/main/pre_update 4.04, stage/main/last 1.07, surface/cell_upload_ms 0.74, stream/drive/ms 0.66, stage/main/first 0.54, stage/main/fixed_main_loop 0.44, stream/tick/ms 0.30, bigspace/total/ms 0.12 |
| warm-up | 3.48 | 828.6 | main/busy/ms 2764, stage/main/post_update 2544, render/thread/ms 798, render/busy/ms 621, stage/render/prepare_assets 571, stage/main/update 206, stage/render/prepare_views 174, render/acquire/ms 173, surface/advance_cpu_ms 68.03, stage/render/create_views 28.28, stage/render/render 14.10, render/graph/ms 9.55 |
| warm-up | 3.61 | 126.0 | render/thread/ms 902, render/busy/ms 672, stage/render/prepare_assets 599, pipelines/compile_ms 332, pipelines/driver_ms 256, stage/render/prepare_views 174, render/acquire/ms 174, main/busy/ms 166, stage/main/update 117, stage/render/render 76.62, pipelines/front_ms 76.24, render/present/ms 55.94 |
| warm-up | 3.73 | 77.0 | pipelines/compile_ms 470, pipelines/driver_ms 378, render/thread/ms 103, render/busy/ms 99.39, pipelines/front_ms 92.30, main/busy/ms 85.14, stage/main/post_update 65.05, stage/render/prepare_resources 28.94, stage/render/render 25.73, terrain/update/ms 22.22, render/graph/ms 21.77, stage/render/prepare_assets 18.83 |
| warm-up | 3.84 | 108.4 | pipelines/compile_ms 1253, pipelines/driver_ms 1094, pipelines/wait_ms 290, render/thread/ms 180, render/busy/ms 178, pipelines/front_ms 158, main/busy/ms 154, stage/main/update 61.43, stage/main/post_update 56.03, stage/render/prepare_flush 54.41, stage/render/prepare_assets 48.22, stage/render/prepare_resources 29.15 |
| warm-up | 3.97 | 135.9 | pipelines/compile_ms 2090, pipelines/driver_ms 1814, pipelines/wait_ms 1733, pipelines/front_ms 276, render/thread/ms 202, main/busy/ms 201, render/busy/ms 200, stage/render/prepare_assets 102, stage/main/pre_update 73.48, stage/main/update 65.42, stage/render/prepare_flush 54.56, stage/main/post_update 34.30 |
| warm-up | 4.03 | 62.3 | pipelines/wait_ms 3087, pipelines/compile_ms 1231, pipelines/driver_ms 1041, pipelines/front_ms 191, render/thread/ms 155, render/busy/ms 153, main/busy/ms 144, stage/render/prepare_assets 87.66, stage/main/pre_update 72.83, stage/main/post_update 44.95, stage/render/prepare_write_phase_buffers 24.56, stage/render/prepare_resources 24.08 |
| warm-up | 4.10 | 61.4 | pipelines/wait_ms 2488, pipelines/compile_ms 397, pipelines/driver_ms 320, render/thread/ms 120, render/busy/ms 118, main/busy/ms 87.44, pipelines/front_ms 76.82, stage/main/post_update 59.84, stage/render/prepare_resources 41.54, stage/render/prepare_write_phase_buffers 24.46, stage/render/prepare_assets 18.96, stage/main/update 18.04 |
| warm-up | 4.15 | 55.4 | pipelines/wait_ms 1290, pipelines/compile_ms 347, pipelines/driver_ms 259, render/thread/ms 111, render/busy/ms 109, pipelines/front_ms 88.00, main/busy/ms 75.36, stage/main/post_update 59.87, stage/render/prepare_resources 45.55, stage/render/prepare_bind_groups 16.41, stage/render/prepare_batch_phases 12.14, stage/render/prepare_flush 10.10 |
| warm-up | 4.21 | 57.6 | pipelines/wait_ms 836, pipelines/compile_ms 452, pipelines/driver_ms 355, render/thread/ms 106, render/busy/ms 104, pipelines/front_ms 96.84, main/busy/ms 70.72, stage/main/post_update 57.36, stage/render/prepare_resources 32.34, stage/render/prepare_assets 29.72, bigspace/total/ms 14.86, bigspace/low_precision/ms 14.62 |
| warm-up | 4.35 | 94.7 | pipelines/wait_ms 1977, pipelines/compile_ms 282, pipelines/driver_ms 214, render/thread/ms 100, render/busy/ms 98.53, main/busy/ms 98.36, stage/main/update 77.71, pipelines/front_ms 67.87, stream/drive/ms 48.86, stage/render/prepare_assets 48.60, stage/render/prepare_flush 18.85, stage/main/post_update 14.73 |
| warm-up | 4.46 | 115.6 | pipelines/wait_ms 701, pipelines/compile_ms 141, pipelines/driver_ms 100, main/busy/ms 87.44, render/thread/ms 71.22, render/busy/ms 70.62, stage/main/update 49.04, pipelines/front_ms 40.23, stage/render/prepare_assets 20.43, stage/main/fixed_main_loop 19.82, stage/render/prepare_flush 16.62, stage/main/post_update 15.07 |
| warm-up | 4.54 | 79.0 | pipelines/compile_ms 1057, pipelines/driver_ms 902, pipelines/wait_ms 701, pipelines/front_ms 155, render/thread/ms 145, render/busy/ms 143, main/busy/ms 64.39, stage/render/prepare_assets 56.44, stage/render/prepare_flush 25.47, stage/main/update 24.46, stage/render/prepare_bind_groups 22.00, stage/main/fixed_main_loop 20.63 |
| warm-up | 4.61 | 66.9 | pipelines/wait_ms 1759, pipelines/compile_ms 1076, pipelines/driver_ms 916, pipelines/front_ms 161, render/thread/ms 130, render/busy/ms 128, stage/render/prepare_assets 50.59, main/busy/ms 50.44, stage/main/post_update 31.12, stage/render/prepare_resources 28.69, stage/render/prepare_bind_groups 16.36, bigspace/total/ms 12.62 |
| warm-up | 4.71 | 102.6 | pipelines/wait_ms 3748, pipelines/compile_ms 309, pipelines/driver_ms 208, render/thread/ms 152, render/busy/ms 150, pipelines/front_ms 101, main/busy/ms 65.86, stage/render/prepare_resources 60.10, stage/render/prepare_assets 46.32, stage/main/post_update 44.88, stage/render/prepare_flush 15.13, stage/render/prepare_write_phase_buffers 14.03 |
| warm-up | 4.79 | 79.4 | pipelines/wait_ms 1989, render/thread/ms 165, render/busy/ms 164, pipelines/compile_ms 149, main/busy/ms 94.11, pipelines/driver_ms 94.04, stage/render/prepare_assets 58.04, pipelines/front_ms 55.18, stage/main/update 53.49, stage/render/prepare_resources 37.16, stage/main/post_update 32.67, stage/render/prepare_write_phase_buffers 27.28 |
| warm-up | 4.88 | 92.1 | pipelines/compile_ms 1318, pipelines/driver_ms 1260, render/thread/ms 140, render/busy/ms 138, main/busy/ms 111, stage/render/prepare_assets 60.59, pipelines/front_ms 58.11, stage/main/update 53.85, stage/main/fixed_main_loop 25.73, stage/render/prepare_write_phase_buffers 21.65, stage/main/post_update 18.22, stage/render/prepare_flush 17.64 |
| warm-up | 4.97 | 90.9 | pipelines/compile_ms 2512, pipelines/driver_ms 2367, pipelines/wait_ms 668, render/thread/ms 153, render/busy/ms 151, pipelines/front_ms 145, main/busy/ms 93.62, stage/render/prepare_resources 42.90, stage/render/prepare_assets 37.11, stage/main/post_update 35.82, stage/main/fixed_main_loop 27.50, stage/render/prepare_write_phase_buffers 22.21 |
| warm-up | 5.04 | 69.6 | pipelines/wait_ms 6135, pipelines/compile_ms 1307, pipelines/driver_ms 1192, render/thread/ms 151, render/busy/ms 148, pipelines/front_ms 115, main/busy/ms 101, stage/main/post_update 72.78, stage/render/prepare_resources 44.31, stage/render/prepare_flush 31.46, stage/render/prepare_write_phase_buffers 20.17, stage/main/update 17.63 |
| warm-up | 5.14 | 92.3 | pipelines/wait_ms 6888, pipelines/compile_ms 195, render/thread/ms 152, render/busy/ms 150, pipelines/driver_ms 126, main/busy/ms 94.07, pipelines/front_ms 68.69, stage/main/post_update 58.89, stage/render/prepare_assets 49.80, stage/render/prepare_flush 35.19, stage/main/update 26.00, stream/drive/ms 20.96 |
| warm-up | 5.23 | 98.0 | pipelines/wait_ms 2923, render/thread/ms 176, render/busy/ms 174, pipelines/compile_ms 142, pipelines/driver_ms 79.11, pipelines/front_ms 62.93, main/busy/ms 61.86, encode//sum 60.93, stage/render/prepare_assets 45.50, stage/render/render 32.97, render/graph/ms 29.64, stage/main/update 28.37 |
| warm-up | 5.36 | 123.4 | pipelines/wait_ms 4699, render/thread/ms 208, render/busy/ms 206, pipelines/compile_ms 155, pipelines/driver_ms 119, main/busy/ms 105, stage/main/update 70.23, encode//sum 52.42, stage/render/prepare_resources 39.64, pipelines/front_ms 36.35, stage/render/prepare_write_phase_buffers 35.45, stage/render/prepare_flush 34.02 |
| warm-up | 5.51 | 149.9 | pipelines/wait_ms 3197, render/thread/ms 230, render/busy/ms 228, main/busy/ms 155, pipelines/compile_ms 93.82, stage/main/update 93.41, stage/render/prepare_assets 92.26, pipelines/driver_ms 80.14, stage/main/post_update 35.07, stage/render/prepare_write_phase_buffers 33.60, stage/render/prepare_resources 30.82, stream/drive/ms 30.51 |
| warm-up | 5.64 | 130.0 | render/thread/ms 222, render/busy/ms 221, main/busy/ms 153, stage/main/update 98.27, stage/render/prepare_assets 97.35, stream/drive/ms 83.74, stage/render/prepare_resources 41.79, stage/main/post_update 30.99, stage/render/prepare_write_phase_buffers 19.20, stage/render/render 17.98, stream/spawn/ms 16.20, stage/main/fixed_main_loop 15.40 |
| warm-up | 5.73 | 90.1 | render/thread/ms 180, render/busy/ms 178, main/busy/ms 119, stage/render/prepare_assets 79.00, stage/main/update 76.36, stream/drive/ms 67.28, stage/render/prepare_resources 33.88, stage/main/post_update 28.34, stage/render/prepare_write_phase_buffers 22.05, stream/spawn/ms 15.96, stage/render/render 11.65, stage/render/prepare_bind_groups 9.26 |
| warm-up | 5.83 | 98.7 | render/thread/ms 166, render/busy/ms 164, main/busy/ms 161, stage/main/update 119, stream/drive/ms 105, stage/render/prepare_assets 68.98, stage/render/prepare_bind_groups 28.90, stage/main/post_update 28.86, stage/render/prepare_flush 19.94, stage/render/prepare_write_phase_buffers 19.03, stage/render/render 12.93, render/graph/ms 10.41 |
| warm-up | 5.98 | 157.0 | pipelines/compile_ms 1325, pipelines/driver_ms 1225, pipelines/wait_ms 986, main/busy/ms 181, render/thread/ms 168, render/busy/ms 166, stage/main/update 138, stream/drive/ms 123, pipelines/front_ms 99.83, stage/render/prepare_assets 63.22, stage/render/prepare_bind_groups 30.03, stage/render/prepare_write_phase_buffers 22.35 |
| warm-up | 6.12 | 135.7 | pipelines/wait_ms 8072, pipelines/compile_ms 1439, pipelines/driver_ms 1332, render/thread/ms 179, render/busy/ms 177, main/busy/ms 140, stage/render/prepare_assets 110, pipelines/front_ms 107, stage/main/update 72.48, stream/drive/ms 50.22, stage/main/post_update 41.18, stage/main/fixed_main_loop 20.33 |
| warm-up | 6.25 | 136.1 | pipelines/wait_ms 9511, render/thread/ms 232, render/busy/ms 230, pipelines/compile_ms 199, main/busy/ms 167, pipelines/driver_ms 161, stage/render/prepare_assets 109, stage/main/post_update 75.59, stage/main/update 71.15, stream/drive/ms 39.07, pipelines/front_ms 37.95, stage/render/create_views 35.67 |
| warm-up | 6.32 | 68.6 | pipelines/wait_ms 2425, render/thread/ms 182, render/busy/ms 181, main/busy/ms 124, pipelines/compile_ms 85.28, stage/render/prepare_assets 76.84, pipelines/driver_ms 54.76, stage/main/post_update 54.08, stage/main/update 39.93, stage/render/create_views 35.89, pipelines/front_ms 30.52, stage/render/prepare_meshes 23.12 |
| warm-up | 6.42 | 50.0 | pipelines/wait_ms 2510, pipelines/compile_ms 235, pipelines/driver_ms 206, render/thread/ms 79.42, render/busy/ms 77.75, main/busy/ms 43.52, stage/render/prepare_assets 37.16, pipelines/front_ms 28.67, stage/main/update 17.47, stage/main/post_update 14.47, stage/render/render 12.79, render/graph/ms 10.01 |
| warm-up | 6.52 | 51.4 | pipelines/wait_ms 5621, pipelines/compile_ms 3120, pipelines/driver_ms 2939, pipelines/front_ms 181, render/thread/ms 86.09, render/busy/ms 84.52, main/busy/ms 58.67, stage/render/prepare_assets 29.26, stage/main/update 24.79, stage/main/post_update 20.50, stage/render/prepare_resources 16.88, stage/render/render 11.50 |
| warm-up | 6.57 | 58.2 | pipelines/wait_ms 10.1 k, pipelines/compile_ms 320, pipelines/driver_ms 252, render/thread/ms 99.86, render/busy/ms 98.24, main/busy/ms 69.42, pipelines/front_ms 68.40, stage/render/prepare_resources 35.20, stage/main/post_update 26.50, stage/render/prepare_assets 24.76, stage/main/update 23.05, stage/render/render 14.23 |
| warm-up | 6.62 | 49.7 | pipelines/wait_ms 10.4 k, pipelines/compile_ms 235, pipelines/driver_ms 177, render/thread/ms 100, render/busy/ms 98.45, main/busy/ms 63.38, pipelines/front_ms 58.66, stage/render/prepare_resources 30.88, stage/render/prepare_assets 24.73, stage/main/update 24.06, stage/main/post_update 22.01, stage/render/render 14.33 |
| warm-up | 6.70 | 73.1 | pipelines/wait_ms 15.0 k, render/thread/ms 109, render/busy/ms 107, pipelines/compile_ms 99.08, main/busy/ms 71.41, pipelines/driver_ms 66.75, stage/render/prepare_assets 58.47, stage/main/post_update 41.31, pipelines/front_ms 32.33, stage/main/update 20.14, bigspace/total/ms 15.70, bigspace/high_precision/ms 14.92 |
| warm-up | 6.80 | 63.5 | pipelines/compile_ms 2794, pipelines/driver_ms 2518, pipelines/wait_ms 1844, pipelines/front_ms 276, render/thread/ms 93.36, render/busy/ms 91.75, main/busy/ms 55.17, stage/render/prepare_assets 48.94, stage/main/post_update 25.16, stage/render/render 11.90, stage/main/update 11.63, stage/render/queue 10.19 |
| warm-up | 6.98 | 62.3 | pipelines/wait_ms 13.1 k, pipelines/compile_ms 3838, pipelines/driver_ms 3472, pipelines/front_ms 367, render/thread/ms 95.63, render/busy/ms 93.83, main/busy/ms 57.82, stage/render/prepare_assets 39.90, stage/main/post_update 24.87, stage/main/update 23.45, stage/render/prepare_resources 17.71, stage/render/render 15.55 |
| warm-up | 7.04 | 66.2 | pipelines/compile_ms 6341, pipelines/wait_ms 6291, pipelines/driver_ms 5832, pipelines/front_ms 510, render/thread/ms 112, render/busy/ms 110, stage/render/prepare_assets 67.42, main/busy/ms 67.27, stage/main/post_update 28.97, stage/main/update 28.89, stream/drive/ms 16.99, stage/render/render 14.84 |
| warm-up | 7.19 | 65.6 | render/thread/ms 86.70, render/busy/ms 83.78, main/busy/ms 69.94, stage/main/update 33.41, stage/render/prepare_assets 27.84, stage/main/post_update 23.42, stage/render/render 23.37, render/graph/ms 19.78, stage/render/prepare_flush 15.89, stream/drive/ms 15.05, encode//sum 9.65, stage/render/graph_submit 8.54 |
| warm-up | 7.24 | 49.1 | render/thread/ms 98.07, render/busy/ms 96.44, main/busy/ms 58.90, stage/main/post_update 32.33, stage/render/prepare_assets 25.71, stage/render/render 17.20, stage/render/prepare_resources 16.45, stage/main/update 15.47, encode//sum 15.43, render/graph/ms 14.58, stage/render/prepare_write_phase_buffers 11.99, stage/render/graph_submit 11.99 |
| warm-up | 7.31 | 70.4 | pipelines/compile_ms 3749, pipelines/driver_ms 3451, pipelines/front_ms 298, render/thread/ms 108, render/busy/ms 106, main/busy/ms 73.16, stage/main/post_update 50.46, stage/render/prepare_assets 41.18, stage/render/prepare_resources 18.10, bigspace/total/ms 17.75, stage/render/render 17.11, bigspace/high_precision/ms 17.06 |
| warm-up | 7.36 | 48.1 | pipelines/compile_ms 3749, pipelines/driver_ms 3451, pipelines/front_ms 298, render/thread/ms 103, render/busy/ms 101, main/busy/ms 62.80, stage/render/prepare_assets 48.62, stage/main/post_update 44.29, bigspace/total/ms 18.36, stage/render/prepare_resources 18.24, bigspace/high_precision/ms 17.75, stage/render/render 11.78 |
| warm-up | 7.42 | 62.8 | render/thread/ms 103, render/busy/ms 101, main/busy/ms 58.71, stage/render/prepare_resources 28.49, stage/render/prepare_assets 28.40, stage/main/post_update 23.04, stage/render/prepare_write_phase_buffers 16.88, stage/main/fixed_main_loop 11.57, stage/main/update 10.81, stage/render/render 10.41, render/graph/ms 7.97, encode//sum 7.26 |
| warm-up | 7.54 | 76.2 | render/thread/ms 109, render/busy/ms 108, main/busy/ms 37.88, stage/render/prepare_assets 25.82, stage/render/prepare_write_phase_buffers 25.48, stage/render/prepare_resources 18.70, stage/render/render 18.54, encode//sum 17.37, render/graph/ms 16.05, stage/main/post_update 14.62, stage/render/graph_submit 13.22, stage/render/prepare_flush 12.35 |
| warm-up | 8.23 | 66.8 | pipelines/wait_ms 3900, pipelines/compile_ms 1252, pipelines/driver_ms 1196, render/thread/ms 69.05, main/busy/ms 68.84, render/busy/ms 67.18, pipelines/front_ms 56.75, stage/main/update 28.52, stage/main/post_update 24.64, stream/drive/ms 20.46, stage/render/prepare_assets 13.92, stage/render/prepare_write_phase_buffers 11.92 |
| warm-up | 8.48 | 54.1 | render/thread/ms 82.04, render/busy/ms 80.40, main/busy/ms 45.55, stage/render/prepare_resources 37.88, stage/main/update 17.74, stage/main/post_update 16.73, stage/render/render 13.03, render/graph/ms 10.60, encode//sum 9.70, stage/render/prepare_write_phase_buffers 8.10, stage/render/graph_submit 7.70, stage/render/specialize 6.48 |
| warm-up | 8.55 | 48.1 | pipelines/compile_ms 4718, pipelines/driver_ms 4413, pipelines/wait_ms 805, pipelines/front_ms 304, render/thread/ms 57.49, render/busy/ms 55.99, main/busy/ms 37.89, stage/render/prepare_assets 26.74, stage/main/post_update 16.89, stage/main/update 13.68, stage/render/render 10.86, encode//sum 9.43 |
| warm-up | 8.85 | 105.4 | pipelines/compile_ms 1798, pipelines/driver_ms 1624, pipelines/wait_ms 578, pipelines/front_ms 174, main/busy/ms 126, stage/main/update 96.20, render/thread/ms 57.70, render/busy/ms 56.17, stage/render/prepare_assets 17.60, stage/main/post_update 15.74, stage/render/render 10.38, stage/main/pre_update 9.72 |
| warm-up | 9.16 | 71.6 | pipelines/compile_ms 7992, pipelines/driver_ms 7550, pipelines/wait_ms 589, pipelines/front_ms 442, render/thread/ms 99.15, render/busy/ms 97.60, main/busy/ms 86.14, stage/main/post_update 52.40, stage/render/prepare_write_phase_buffers 28.43, stage/render/prepare_resources 24.23, stage/main/update 20.88, stage/render/render 15.53 |
| warm-up | 9.26 | 58.0 | pipelines/wait_ms 435, pipelines/compile_ms 108, render/thread/ms 76.38, render/busy/ms 74.96, pipelines/driver_ms 63.97, main/busy/ms 48.49, pipelines/front_ms 44.42, stage/render/prepare_assets 36.08, stage/main/post_update 20.09, stage/main/update 15.31, encode//sum 11.28, stage/render/render 10.69 |
| warm-up | 9.54 | 58.0 | render/thread/ms 77.30, render/busy/ms 75.85, main/busy/ms 39.15, stage/render/prepare_assets 37.67, stage/main/update 15.88, stage/main/post_update 14.61, stage/render/prepare_bind_groups 10.36, stage/render/render 9.46, encode//sum 8.69, stage/render/prepare_resources 8.32, render/graph/ms 7.22, stage/render/graph_submit 4.84 |
| warm-up | 9.95 | 53.5 | render/thread/ms 79.05, render/busy/ms 77.63, main/busy/ms 45.55, stage/render/prepare_assets 39.56, stage/main/update 20.14, stage/main/post_update 19.83, stage/render/prepare_resources 11.19, stage/render/render 11.15, encode//sum 9.67, render/graph/ms 8.79, stage/render/graph_submit 6.08, stream/drive/ms 5.03 |
| warm-up | 10.02 | 67.3 | render/thread/ms 115, render/busy/ms 113, main/busy/ms 50.33, stage/render/prepare_assets 45.10, stage/main/update 23.41, stage/main/post_update 20.67, stage/render/specialize 15.68, stage/render/queue 13.60, stage/render/render 12.92, encode//sum 11.68, render/graph/ms 10.31, stage/render/prepare_resources 8.54 |
| warm-up | 10.15 | 90.0 | pipelines/compile_ms 3634, pipelines/driver_ms 3323, pipelines/wait_ms 914, pipelines/front_ms 311, render/thread/ms 72.42, render/busy/ms 70.90, stage/render/cleanup 46.14, stage/render/prepare_assets 44.51, main/busy/ms 42.38, stage/main/update 17.54, stage/main/post_update 16.93, stage/render/render 11.22 |
| warm-up | 10.23 | 55.2 | pipelines/wait_ms 865, pipelines/compile_ms 83.34, render/thread/ms 64.88, main/busy/ms 64.67, render/busy/ms 63.41, pipelines/driver_ms 50.10, pipelines/front_ms 33.24, stage/main/update 32.86, stage/main/post_update 22.44, stream/drive/ms 21.00, stage/render/render 17.09, render/graph/ms 14.87 |
| warm-up | 10.35 | 52.5 | render/thread/ms 76.17, render/busy/ms 74.76, main/busy/ms 68.06, stage/main/post_update 43.42, stage/render/prepare_assets 19.38, stage/render/prepare_flush 16.79, stage/main/update 12.57, stage/render/render 9.81, stage/render/prepare_resources 9.78, encode//sum 9.53, stage/render/prepare_write_phase_buffers 8.24, render/graph/ms 7.41 |
| warm-up | 12.70 | 60.5 | render/thread/ms 74.63, render/busy/ms 73.12, main/busy/ms 32.71, encode//sum 19.29, stage/main/post_update 18.30, stage/render/render 18.14, stage/render/specialize 16.82, stage/render/queue 16.08, render/graph/ms 15.91, stage/render/graph_submit 12.36, stage/main/update 9.76, encode/submit 6.77 |
| warm-up | 13.74 | 62.8 | render/thread/ms 68.17, render/busy/ms 66.68, encode//sum 39.28, main/busy/ms 37.61, stage/render/render 35.71, render/graph/ms 33.55, stage/render/graph_submit 30.07, encode/finish_wall 24.65, stage/main/post_update 21.57, stage/render/prepare_assets 12.39, stage/main/update 9.84, stage/render/prepare_write_phase_buffers 6.13 |
| warm-up | 13.87 | 60.4 | pipelines/wait_ms 8897, pipelines/compile_ms 4874, pipelines/driver_ms 4441, pipelines/front_ms 433, render/thread/ms 77.13, render/busy/ms 75.16, main/busy/ms 45.32, stage/main/post_update 32.64, encode//sum 27.41, stage/render/render 24.68, stage/render/prepare_resources 23.35, render/graph/ms 22.40 |
| warm-up | 14.11 | 50.9 | pipelines/wait_ms 6596, pipelines/compile_ms 1819, pipelines/driver_ms 1688, pipelines/front_ms 130, render/thread/ms 90.41, render/busy/ms 88.90, main/busy/ms 48.35, encode//sum 32.12, stage/render/render 29.31, render/graph/ms 26.95, stage/render/graph_submit 23.34, stage/main/post_update 19.92 |
| warm-up | 14.17 | 66.8 | pipelines/wait_ms 9063, pipelines/compile_ms 189, pipelines/front_ms 186, render/thread/ms 108, render/busy/ms 107, main/busy/ms 61.77, stage/main/post_update 33.72, stage/render/prepare_resources 26.21, stage/render/render 25.67, render/graph/ms 23.23, encode//sum 18.10, stage/render/prepare_flush 16.72 |
| warm-up | 14.22 | 51.0 | pipelines/wait_ms 17.4 k, pipelines/compile_ms 257, pipelines/front_ms 213, render/thread/ms 109, render/busy/ms 107, main/busy/ms 78.73, stage/main/post_update 53.26, pipelines/driver_ms 44.86, stage/render/prepare_write_phase_buffers 32.72, stage/render/prepare_resources 32.22, bigspace/total/ms 26.49, stage/render/render 23.94 |
| warm-up | 14.27 | 49.6 | pipelines/wait_ms 17.6 k, pipelines/compile_ms 125, render/thread/ms 88.22, render/busy/ms 79.22, main/busy/ms 74.41, pipelines/driver_ms 72.85, pipelines/front_ms 51.70, stage/main/post_update 50.09, stage/render/render 22.33, stage/render/prepare_write_phase_buffers 20.20, bigspace/total/ms 19.59, encode//sum 15.86 |
| warm-up | 14.39 | 65.6 | render/thread/ms 74.57, render/busy/ms 73.12, main/busy/ms 73.08, stage/render/prepare_assets 27.98, stage/main/first 26.83, encode//sum 23.87, stage/render/render 21.20, render/graph/ms 18.95, stage/main/post_update 17.10, stage/main/pre_update 15.56, stage/render/graph_submit 14.75, stage/render/prepare_bind_groups 12.30 |

## Run

- `../target/profiling/walkonce.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set render.wgpu.bind_group_walk_once=on --resolution 2560x1440 --bench-out target/perf\wgpu_walk_once\balmora_walk@2560x1440_arm=once-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 14%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1386 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **72%** (min 39%) · 2618 MHz (min 1905) · memory 8101 MHz · 75.8 W · 75 °C (max 81) · VRAM 6598 MiB · throttled 100% of samples

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
