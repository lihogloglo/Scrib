# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5926 frames over 65.6 s · **median 10.78 ms (93 fps)** · mean 11.08 · p95 13.75 · p99 17.69 (1% low 57 fps) · worst 48.34 ms · 87 spikes over 1.5× median · 64.9 stutters/min (work 66.7/min)
- warm-up: 324 frames over 9.5 s, worst 2574.9 ms
- hitches (> 26.9 ms): **31** measured, 57 in warm-up
- timed GPU passes: 6.82 ms of a 11.08 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.78 ms, limit 11 ms
- **FAIL** frame_ms: 71 stutters (> 1.5x moving median of 60) in 1.09 min = 64.90/min, limit 1/min
- **FAIL** worst frame 48.34 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.915 | 2.040 | 2.072 | 0.94 | 0.176 |
| main_opaque_pass_3d | 1.531 | 1.630 | 2.037 | 0.94 | 0.065 |
| early prepass | 0.465 | 0.496 | 0.722 | 0.94 | 0.084 |
| bloom | 0.296 | 0.315 | 0.321 | 0.94 | 0.063 |
| atmosphere_luts | 0.286 | 0.305 | 0.367 | 0.94 | 0.003 |
| water_velocity | 0.262 | 0.279 | 0.347 | 0.94 | 0.002 |
| water_sspr | 0.190 | 0.202 | 0.204 | 0.94 | 0.002 |
| ssao | 0.177 | 0.189 | 0.193 | 0.94 | 0.007 |
| auxiliary_views | 0.146 | 0.156 | 0.561 | 0.94 | 0.157 |
| uniform_allocation | 0.146 | 0.155 | 0.212 | 0.94 | 0.026 |
| atmos_froxel_inject | 0.137 | 0.146 | 0.147 | 0.94 | 0.001 |
| clouds_trace | 0.132 | 0.141 | 0.232 | 0.94 | 0.001 |
| water_fft | 0.123 | 0.131 | 0.138 | 0.94 | 0.003 |
| upscaling | 0.111 | 0.118 | 0.125 | 0.94 | 0.004 |
| bin_unpacking | 0.105 | 0.112 | 0.151 | 0.94 | 0.019 |
| clouds_composite | 0.099 | 0.106 | 0.109 | 0.94 | 0.002 |
| main_transmissive_pass_3d | 0.081 | 0.086 | 0.179 | 0.94 | 0.008 |
| tonemapping | 0.080 | 0.085 | 0.088 | 0.94 | 0.003 |
| clustering | 0.060 | 0.064 | 0.067 | 0.94 | 0.019 |
| render_sky | 0.048 | 0.052 | 0.053 | 0.94 | 0.002 |
| clouds_resolve | 0.046 | 0.049 | 0.049 | 0.94 | 0.001 |
| surface_exposure | 0.041 | 0.044 | 0.045 | 0.94 | 0.001 |
| lightprobe_irradiance_map | 0.040 | 0.414 | 0.417 | 0.10 | 0.001 |
| surface_trail | 0.038 | 0.056 | 0.057 | 0.67 | 0.002 |
| early_mesh_preprocessing | 0.038 | 0.040 | 0.059 | 0.94 | 0.005 |
| surface_exposure_far | 0.035 | 0.042 | 0.045 | 0.83 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.94 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.032 | 0.94 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.94 | 0.002 |
| lightprobe_radiance_map | 0.021 | 0.220 | 0.230 | 0.10 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.94 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.031 | 0.94 | 0.171 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.94 | 0.002 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.94 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.94 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.94 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.94 | 0.003 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.668 | 12.182 | 44.01 | 57294.2 |
| render/busy/ms | 9.132 | 11.646 | 43.32 | 54113.3 |
| main/busy/ms | 8.677 | 10.296 | 28.45 | 51419.9 |
| stage/render/render | 5.539 | 7.014 | 17.33 | 32821.5 |
| stage/main/post_update | 5.237 | 6.495 | 9.37 | 31036.7 |
| render/graph/ms | 4.776 | 6.212 | 16.17 | 28303.3 |
| encode//sum | 4.109 | 5.454 | 45.44 | 24349.8 |
| stage/render/graph_submit | 3.524 | 4.606 | 12.26 | 20881.3 |
| encode/submit | 2.225 | 3.049 | 6.59 | 13186.2 |
| stage/main/update | 2.167 | 2.561 | 22.11 | 12840.6 |
| encode/finish_wall | 1.363 | 1.949 | 6.90 | 8078.4 |
| stage/render/camera_driver | 1.198 | 1.683 | 3.92 | 7100.2 |
| encode/pass:shadow_point_light/sum | 0.926 | 3.295 | 16.66 | 5490.4 |
| encode/pass:shadow_directional_light/sum | 0.834 | 1.526 | 2.50 | 4942.2 |
| stage/render/prepare_write_phase_buffers | 0.781 | 1.197 | 2.31 | 4626.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.753 | 1.080 | 1.72 | 4462.4 |
| encode/pass:early prepass/sum | 0.725 | 1.066 | 1.55 | 4296.3 |
| stage/render/prepare_assets | 0.565 | 0.732 | 14.19 | 3350.2 |
| stage/render/prepare_meshes | 0.519 | 0.708 | 3.29 | 3077.6 |
| bigspace/total/ms | 0.495 | 1.140 | 2.17 | 2933.5 |
| stage/render/prepare_resources | 0.450 | 0.536 | 0.96 | 2669.0 |
| render/present/ms | 0.443 | 0.660 | 2.20 | 2627.6 |
| stage/main/first | 0.397 | 0.523 | 5.64 | 2351.0 |
| stage/main/pre_update | 0.357 | 0.517 | 1.65 | 2116.2 |
| stage/render/specialize | 0.350 | 0.451 | 13.75 | 2073.5 |
| terrain/update/ms | 0.308 | 0.396 | 2.10 | 1827.3 |
| stage/render/queue | 0.308 | 0.445 | 13.42 | 1823.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.299 | 0.440 | 1.10 | 1771.2 |
| stage/render/prepare_bind_groups | 0.293 | 0.536 | 2.44 | 1735.3 |
| terrain/select/ms | 0.275 | 0.354 | 1.05 | 1628.8 |
| stage/render/prepare_batch_phases | 0.262 | 0.373 | 0.94 | 1550.4 |
| controller/feed/ms | 0.261 | 0.330 | 1.26 | 1545.3 |
| stream/drive/ms | 0.241 | 0.317 | 20.35 | 1430.2 |
| stage/main/fixed_main_loop | 0.220 | 0.372 | 1.18 | 1305.8 |
| bigspace/low_precision/ms | 0.202 | 0.560 | 1.95 | 1196.7 |
| stage/render/create_views | 0.199 | 0.257 | 0.87 | 1178.8 |
| far/select/ms | 0.195 | 0.266 | 0.56 | 1154.1 |
| stream/tick/ms | 0.185 | 0.249 | 0.70 | 1095.2 |
| stage/render/extract_commands | 0.168 | 0.197 | 0.45 | 998.5 |
| stage/render/prepare_flush | 0.168 | 0.316 | 0.83 | 995.4 |
| anim/evaluate/ms | 0.154 | 0.257 | 1.40 | 912.1 |
| controller/move/ms | 0.153 | 0.354 | 0.58 | 904.7 |
| anim/evaluate_cpu/ms | 0.150 | 0.186 | 0.59 | 887.6 |
| stage/render/prepare_views | 0.136 | 0.170 | 0.46 | 808.2 |
| bigspace/high_precision/ms | 0.131 | 0.479 | 1.72 | 775.8 |
| stage/main/last | 0.118 | 0.163 | 1.70 | 698.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.099 | 0.162 | 0.88 | 589.3 |
| render/acquire/ms | 0.093 | 0.119 | 0.43 | 553.3 |
| stage/render/cleanup | 0.076 | 0.198 | 0.81 | 450.0 |
| stage/render/prepare_collect_phase_buffers | 0.062 | 0.133 | 0.31 | 366.1 |
| stage/render/phase_sort | 0.060 | 0.088 | 0.32 | 354.6 |
| pipelines/compile_ms | 0.051 | 0.000 | 126.83 | 300.7 |
| pipelines/front_ms | 0.049 | 0.000 | 122.48 | 287.8 |
| physics/step/ms | 0.047 | 0.087 | 0.49 | 281.2 |
| physics/insert/ms | 0.033 | 0.373 | 0.93 | 195.7 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.61 | 185.1 |
| weather/map/ms | 0.025 | 0.033 | 0.21 | 147.0 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.27 | 87.9 |
| anim/apply/ms | 0.012 | 0.020 | 0.22 | 68.5 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.24 | 66.1 |
| controller/avoid/ms | 0.011 | 0.014 | 0.21 | 63.0 |
| weather/eval/ms | 0.010 | 0.014 | 0.23 | 60.5 |
| script/update/ms | 0.007 | 0.009 | 0.20 | 41.5 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 30.7 |
| stream/despawn/ms | 0.005 | 0.000 | 1.15 | 30.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.16 | 29.8 |
| script/ui/ms | 0.005 | 0.006 | 0.03 | 28.2 |
| controller/write/ms | 0.003 | 0.005 | 0.25 | 20.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.67 | 14.4 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.35 | 13.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.7 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 593 |
| crowd/c0/meshes/count | count | 353 | 646 | 156 |
| crowd/c1/count | count | 146 | 155 | 147 |
| crowd/c1/joints/count | count | 5531 | 5892 | 5571 |
| crowd/c1/meshes/count | count | 1180 | 1255 | 1177 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4614 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4593 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.43 | 2.04 | 0.52 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.27 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.24 | 0.01 |
| encode/finish_critical | ms | 0.96 | 2.50 | 0.94 |
| encode/pass:early prepass/max | ms | 0.74 | 1.55 | 0.62 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.77 | 1.72 | 0.94 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.88 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.28 | 1.10 | 0.27 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 2.50 | 0.59 |
| encode/pass:shadow_point_light/max | ms | 0.81 | 1.40 | 0.80 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.61 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.04 | 43.32 | 8.22 |
| gpu/timed/ms | ms | 7.98 | 11.26 | 8.22 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 249 | 478 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3902 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 249 | 478 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3902 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 4103 | 4330 | 4330 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 4103 | 4330 | 4330 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 23 | 204 | 5 |
| phases/shadow/bins | count | 14.4 k | 19.1 k | 12.1 k |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 40.4 k | 56.7 k | 39.8 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 205 | 250 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 199 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 18 | 573 | 6 |
| preprocess/bind_groups_created | count | 18 | 573 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 134 | 236 | 125 |
| process/cpu_usage | % | 25.24 | 41.35 | 24.52 |
| process/mem_usage | GiB | 5.58 | 5.75 | 5.75 |
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
| skin/blocks_changed/count | count | 14 | 124 | 15 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 366.1 k | 811.3 k | 493.3 k |
| skin/joints_written/count | count | 5720 | 12.7 k | 7708 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4918 | 5012 | 5012 |
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
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.19 | 38.85 | 32.18 |
| system/mem_usage | % | 67.23 | 67.96 | 67.72 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 74 | 255 | 55 |
| views/all/bins | count | 23.1 k | 27.7 k | 21.7 k |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 69.0 k | 85.3 k | 69.4 k |
| views/all/pipelines | count | 253 | 299 | 77 |
| views/all/pipelines_distinct | count | 71 | 77 | 61 |
| views/all/work_items | count | 41.4 k | 82.6 k | 42.2 k |
| views/cascade/batch_sets | count | 11 | 48 | 5 |
| views/cascade/bins | count | 12.0 k | 15.3 k | 12.1 k |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 39.0 k | 53.1 k | 39.8 k |
| views/cascade/pipelines | count | 38 | 55 | 31 |
| views/cascade/work_items | count | 11.7 k | 52.5 k | 11.9 k |
| views/lamp_face/batch_sets | count | 0 | 195 | 0 |
| views/lamp_face/bins | count | 3129 | 4306 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3388 | 4592 | 0 |
| views/lamp_face/pipelines | count | 167 | 201 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 4138 | 0 |
| views/main/batch_sets | count | 51 | 55 | 50 |
| views/main/bins | count | 8670 | 9602 | 9602 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.0 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 48 | 51 | 46 |
| views/main/work_items | count | 28.5 k | 30.4 k | 30.3 k |
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
| anim/bones_written/count | count | 55.95 | 230 | 331.5 k |
| anim/events/count | count | 0.09 | 3 | 540 |
| assets/Image/added | count | 0.10 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.69 | 152 | 4061 |
| assets/Mesh/removed | count | 0.82 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.65 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.79 | 1328 | 4676 |
| encode//count | count | 33.94 | 220 | 201.1 k |
| encode/bind_walk/init_textures | count | 12.9 k | 114.7 k | 76.29 M |
| encode/bind_walk/init_textures_skipped | count | 56.0 k | 136.2 k | 332.13 M |
| encode/bind_walk/inits | count | 525 | 1254 | 3.11 M |
| encode/bind_walk/inits_skipped | count | 126 | 192 | 749.3 k |
| encode/bind_walk/merge_textures | count | 12.8 k | 114.6 k | 75.86 M |
| encode/bind_walk/merge_textures_skipped | count | 5501 | 6026 | 32.60 M |
| encode/bind_walk/merges | count | 102 | 339 | 603.2 k |
| encode/bind_walk/merges_skipped | count | 106 | 151 | 628.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5926 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5926 |
| encode/pass:early prepass/count | count | 1 | 1 | 5926 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5926 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5926 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5926 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5926 |
| encode/pass:shadow_point_light/count | count | 1.49 | 48 | 8853 |
| encode/queue/command_buffers | count | 45.45 | 278 | 269.4 k |
| encode/queue/submits | count | 2.02 | 4 | 12.0 k |
| encode/queue/write_buffer/bytes | bytes | 2.34 M | 6.26 M | 13.85 G |
| encode/queue/write_buffer/calls | count | 270 | 855 | 1.60 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.6 k | 3.52 M | 1.05 G |
| encode/queue/write_buffer_with/calls | count | 11.04 | 465 | 65.4 k |
| encode/queue/write_texture/bytes | bytes | 12.1 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5926 |
| physics/inserts | count | 0.38 | 115 | 2267 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5713 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1252 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.9 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2574.9 | main/busy/ms 2544, stage/main/post_update 2438, stage/main/update 95.68, surface/advance_cpu_ms 66.78, stage/main/pre_update 4.96, stage/main/last 1.11, stage/main/fixed_main_loop 0.77, surface/cell_upload_ms 0.74, stage/main/first 0.59, stream/drive/ms 0.37, weather/map/ms 0.22, bigspace/total/ms 0.08 |
| warm-up | 3.40 | 823.9 | main/busy/ms 2694, stage/main/post_update 2469, render/thread/ms 791, render/busy/ms 625, stage/render/prepare_assets 575, stage/main/update 209, stage/render/prepare_views 163, render/acquire/ms 163, surface/advance_cpu_ms 66.78, stage/render/create_views 28.56, stage/render/render 13.70, render/graph/ms 9.11 |
| warm-up | 3.51 | 114.1 | render/thread/ms 888, render/busy/ms 668, stage/render/prepare_assets 603, main/busy/ms 173, stage/render/prepare_views 163, render/acquire/ms 163, stage/main/update 118, pipelines/compile_ms 105, stage/render/render 73.40, pipelines/front_ms 64.22, render/present/ms 56.93, stage/main/post_update 42.74 |
| warm-up | 3.56 | 50.7 | pipelines/compile_ms 453, pipelines/front_ms 347, pipelines/wait_ms 168, render/thread/ms 146, pipelines/driver_ms 106, stage/render/render 102, render/busy/ms 90.15, main/busy/ms 65.08, render/present/ms 55.05, stage/main/post_update 47.64, render/graph/ms 45.84, stage/render/graph_submit 35.53 |
| warm-up | 3.63 | 70.1 | pipelines/wait_ms 2401, pipelines/compile_ms 912, pipelines/front_ms 811, render/thread/ms 118, render/busy/ms 115, pipelines/driver_ms 101, main/busy/ms 81.12, stage/main/post_update 68.46, stage/render/render 61.14, render/graph/ms 57.75, stage/render/graph_submit 47.67, encode/finish_wall 38.18 |
| warm-up | 3.72 | 85.5 | pipelines/wait_ms 5365, pipelines/compile_ms 1424, pipelines/front_ms 1328, render/thread/ms 152, render/busy/ms 150, main/busy/ms 125, pipelines/driver_ms 96.35, stage/main/update 62.56, stage/render/prepare_resources 62.16, stage/main/post_update 50.28, stage/render/render 31.71, render/graph/ms 29.41 |
| warm-up | 3.79 | 75.7 | pipelines/wait_ms 3242, pipelines/compile_ms 1584, pipelines/front_ms 1458, render/thread/ms 153, render/busy/ms 151, pipelines/driver_ms 126, main/busy/ms 113, stage/render/prepare_assets 66.29, stage/main/update 63.72, stage/main/post_update 36.09, stage/render/prepare_resources 29.81, stage/render/render 26.94 |
| warm-up | 3.88 | 54.3 | pipelines/compile_ms 987, pipelines/front_ms 938, pipelines/wait_ms 82.19, main/busy/ms 66.03, render/thread/ms 61.61, render/busy/ms 59.46, pipelines/driver_ms 48.53, stage/render/prepare_assets 32.23, stage/main/pre_update 24.21, stage/render/render 19.68, stage/main/post_update 19.02, render/graph/ms 17.19 |
| warm-up | 3.91 | 32.3 | pipelines/compile_ms 840, pipelines/front_ms 812, pipelines/wait_ms 203, main/busy/ms 69.94, render/thread/ms 68.23, render/busy/ms 65.92, stage/main/post_update 39.09, encode//sum 30.05, pipelines/driver_ms 28.03, stage/render/render 23.44, stage/main/pre_update 22.29, stage/render/prepare_assets 21.69 |
| warm-up | 4.06 | 71.2 | pipelines/compile_ms 1295, pipelines/front_ms 1231, pipelines/driver_ms 63.83, render/thread/ms 61.12, main/busy/ms 59.27, render/busy/ms 54.74, stage/main/update 45.45, stream/drive/ms 39.51, stage/render/prepare_assets 37.74, stage/render/render 11.03, stage/main/post_update 9.16, render/graph/ms 9.00 |
| warm-up | 4.09 | 34.3 | pipelines/compile_ms 954, pipelines/front_ms 911, render/thread/ms 65.82, render/busy/ms 62.97, pipelines/driver_ms 43.42, main/busy/ms 36.93, stage/render/prepare_assets 30.74, stage/main/update 20.10, stage/render/render 15.04, render/graph/ms 11.84, stream/drive/ms 10.59, stage/main/post_update 10.51 |
| warm-up | 4.16 | 44.2 | render/thread/ms 59.30, render/busy/ms 57.48, pipelines/compile_ms 53.57, pipelines/front_ms 51.77, main/busy/ms 43.46, stage/render/prepare_assets 41.41, stage/main/update 28.08, stream/drive/ms 24.07, stage/render/render 10.84, stage/main/post_update 10.18, render/graph/ms 8.72, encode//sum 6.52 |
| warm-up | 4.20 | 43.0 | render/thread/ms 69.60, render/busy/ms 67.32, main/busy/ms 49.94, stage/render/prepare_assets 45.06, pipelines/compile_ms 40.84, pipelines/front_ms 39.55, stage/main/update 32.23, stream/drive/ms 28.14, stage/render/render 14.16, render/graph/ms 11.19, stage/main/post_update 10.90, stage/render/graph_submit 8.22 |
| warm-up | 4.25 | 47.6 | pipelines/compile_ms 226, pipelines/front_ms 219, render/thread/ms 56.52, render/busy/ms 54.65, stage/render/prepare_assets 34.21, main/busy/ms 31.83, stage/main/update 12.24, stage/render/render 12.19, stage/main/post_update 10.35, render/graph/ms 9.42, stream/drive/ms 7.81, pipelines/driver_ms 7.67 |
| warm-up | 4.31 | 57.5 | pipelines/compile_ms 891, pipelines/front_ms 863, render/thread/ms 62.71, render/busy/ms 61.03, stage/render/prepare_assets 44.35, main/busy/ms 40.62, pipelines/driver_ms 27.83, stage/main/update 17.31, stage/main/post_update 13.73, stage/render/render 9.78, render/graph/ms 7.56, encode//sum 6.01 |
| warm-up | 4.37 | 58.0 | pipelines/compile_ms 1016, pipelines/front_ms 981, render/thread/ms 88.27, render/busy/ms 82.41, main/busy/ms 79.05, stage/main/update 57.32, stage/render/prepare_assets 38.35, pipelines/driver_ms 34.26, stage/render/render 31.92, render/graph/ms 25.48, stage/render/camera_driver 15.86, stage/main/post_update 12.96 |
| warm-up | 4.46 | 88.9 | pipelines/compile_ms 939, pipelines/front_ms 908, render/thread/ms 104, render/busy/ms 98.28, main/busy/ms 73.20, pipelines/wait_ms 57.21, stage/main/update 52.99, stage/render/prepare_assets 45.61, stage/render/render 34.45, pipelines/driver_ms 30.99, render/graph/ms 27.97, stage/render/camera_driver 16.93 |
| warm-up | 4.54 | 79.2 | pipelines/compile_ms 1454, pipelines/front_ms 1412, pipelines/wait_ms 269, render/thread/ms 129, render/busy/ms 128, main/busy/ms 74.77, stage/render/prepare_assets 57.90, pipelines/driver_ms 42.33, stage/main/post_update 36.92, stage/main/update 29.09, stage/render/prepare_batch_phases 20.13, stage/render/render 17.17 |
| warm-up | 4.61 | 73.8 | pipelines/compile_ms 2053, pipelines/front_ms 1986, pipelines/wait_ms 398, render/thread/ms 134, render/busy/ms 132, main/busy/ms 76.73, pipelines/driver_ms 66.96, stage/render/prepare_assets 48.29, stage/main/post_update 41.76, encode//sum 29.69, stage/main/update 28.33, stage/render/render 21.79 |
| warm-up | 4.70 | 87.2 | pipelines/compile_ms 1846, pipelines/front_ms 1779, pipelines/wait_ms 241, render/thread/ms 132, render/busy/ms 130, pipelines/driver_ms 66.95, main/busy/ms 51.31, stage/render/prepare_assets 45.10, encode//sum 36.05, stage/render/render 32.87, render/graph/ms 30.25, stage/main/post_update 29.72 |
| warm-up | 4.76 | 63.9 | pipelines/compile_ms 1957, pipelines/front_ms 1896, pipelines/wait_ms 262, render/thread/ms 129, render/busy/ms 127, main/busy/ms 97.58, stage/main/update 62.53, pipelines/driver_ms 61.71, stream/drive/ms 54.07, stage/render/prepare_assets 36.24, stage/render/render 32.26, render/graph/ms 29.61 |
| warm-up | 4.81 | 53.8 | pipelines/compile_ms 2356, pipelines/front_ms 2287, pipelines/wait_ms 486, main/busy/ms 111, render/thread/ms 90.56, render/busy/ms 88.65, stage/main/update 86.64, pipelines/driver_ms 68.85, stream/drive/ms 55.16, stage/render/render 31.24, render/graph/ms 28.19, stage/render/prepare_assets 23.92 |
| warm-up | 4.92 | 104.3 | pipelines/compile_ms 1519, pipelines/front_ms 1474, pipelines/wait_ms 369, main/busy/ms 162, stage/main/update 121, render/thread/ms 114, render/busy/ms 112, stream/drive/ms 60.53, stage/render/render 57.02, render/graph/ms 53.72, pipelines/driver_ms 44.47, stage/render/prepare_assets 34.63 |
| warm-up | 5.03 | 114.7 | pipelines/compile_ms 2252, pipelines/front_ms 2193, pipelines/wait_ms 428, main/busy/ms 136, render/thread/ms 125, render/busy/ms 123, stage/main/update 97.38, stream/drive/ms 60.19, pipelines/driver_ms 59.66, stage/render/prepare_assets 53.77, stage/render/render 44.54, render/graph/ms 41.68 |
| warm-up | 5.08 | 48.5 | pipelines/compile_ms 2111, pipelines/front_ms 2053, pipelines/wait_ms 349, main/busy/ms 93.18, render/thread/ms 86.21, render/busy/ms 84.63, stage/main/update 67.40, pipelines/driver_ms 57.31, stream/drive/ms 50.56, stage/render/prepare_assets 44.45, stage/render/render 15.53, stage/main/post_update 14.83 |
| warm-up | 5.17 | 84.0 | pipelines/compile_ms 1079, pipelines/front_ms 1047, main/busy/ms 99.54, stage/main/update 73.90, render/thread/ms 72.39, render/busy/ms 70.86, stream/drive/ms 50.36, stage/render/prepare_assets 37.37, pipelines/driver_ms 32.28, stage/main/post_update 16.94, stage/render/render 13.99, render/graph/ms 11.83 |
| warm-up | 5.20 | 38.6 | pipelines/compile_ms 1093, pipelines/front_ms 1060, main/busy/ms 76.35, render/thread/ms 61.83, render/busy/ms 60.34, stage/main/update 57.15, stream/drive/ms 36.87, pipelines/driver_ms 33.91, stage/render/prepare_assets 32.74, stage/render/render 12.11, stage/main/post_update 11.12, render/graph/ms 9.84 |
| warm-up | 5.26 | 57.4 | pipelines/compile_ms 944, pipelines/front_ms 916, main/busy/ms 71.99, stage/main/update 55.08, render/thread/ms 54.25, render/busy/ms 52.84, stream/drive/ms 36.88, pipelines/driver_ms 28.15, stage/render/prepare_assets 22.48, stage/render/render 11.40, render/graph/ms 9.19, encode//sum 8.92 |
| warm-up | 5.29 | 32.4 | pipelines/compile_ms 649, pipelines/front_ms 630, render/thread/ms 53.24, render/busy/ms 51.73, main/busy/ms 41.45, stage/main/update 20.54, pipelines/driver_ms 19.17, stage/render/prepare_assets 15.14, stage/main/post_update 13.83, stage/render/render 12.60, render/graph/ms 10.46, encode//sum 9.34 |
| warm-up | 5.37 | 47.5 | pipelines/compile_ms 227, pipelines/front_ms 221, render/thread/ms 68.73, render/busy/ms 67.17, main/busy/ms 32.73, stage/render/prepare_assets 26.78, stage/main/update 14.72, stage/render/render 13.61, stage/main/post_update 11.65, render/graph/ms 11.40, encode//sum 9.55, stage/render/graph_submit 8.39 |
| warm-up | 5.41 | 36.9 | pipelines/compile_ms 525, pipelines/front_ms 508, render/thread/ms 77.71, render/busy/ms 75.32, main/busy/ms 40.37, stage/render/prepare_assets 24.28, stage/main/post_update 18.96, pipelines/driver_ms 17.40, stage/render/render 16.93, stage/main/update 14.68, render/graph/ms 14.21, encode//sum 10.93 |
| warm-up | 5.44 | 37.6 | pipelines/compile_ms 449, pipelines/front_ms 433, render/thread/ms 66.01, render/busy/ms 63.57, main/busy/ms 43.57, stage/render/render 21.74, render/graph/ms 18.97, stage/main/post_update 18.04, stage/main/update 17.28, pipelines/driver_ms 16.47, stage/render/prepare_assets 13.81, stage/render/graph_submit 13.73 |
| warm-up | 5.52 | 79.9 | pipelines/compile_ms 1033, pipelines/front_ms 1005, render/thread/ms 98.13, render/busy/ms 96.45, main/busy/ms 66.44, stage/render/prepare_assets 39.39, stage/main/update 38.42, pipelines/driver_ms 28.47, stage/render/render 18.31, render/graph/ms 15.87, stage/main/post_update 15.82, stage/render/prepare_resources 12.38 |
| warm-up | 5.61 | 31.4 | pipelines/compile_ms 197, pipelines/front_ms 191, render/thread/ms 50.60, render/busy/ms 47.91, main/busy/ms 35.63, stage/render/render 17.00, stage/main/update 16.97, render/graph/ms 14.20, stage/main/post_update 12.63, encode//sum 11.39, stage/render/graph_submit 10.67, encode/submit 7.60 |
| warm-up | 5.65 | 48.0 | pipelines/compile_ms 564, pipelines/front_ms 549, render/thread/ms 66.36, render/busy/ms 64.27, main/busy/ms 38.38, stage/render/prepare_assets 30.23, stage/main/update 19.99, pipelines/driver_ms 14.96, stage/render/render 13.78, stage/main/post_update 11.82, render/graph/ms 11.50, encode//sum 10.40 |
| warm-up | 5.69 | 30.5 | pipelines/compile_ms 1231, pipelines/front_ms 1201, render/thread/ms 64.56, render/busy/ms 62.80, pipelines/wait_ms 59.06, main/busy/ms 37.80, stage/render/prepare_assets 33.52, pipelines/driver_ms 29.45, stage/main/update 18.42, stage/render/render 13.36, stage/main/post_update 12.51, render/graph/ms 10.84 |
| warm-up | 5.74 | 33.1 | pipelines/compile_ms 336, pipelines/front_ms 325, render/thread/ms 49.36, render/busy/ms 47.79, main/busy/ms 37.81, pipelines/wait_ms 36.08, stage/render/prepare_assets 19.60, stage/main/update 16.57, stage/main/post_update 15.07, stage/render/render 14.58, render/graph/ms 12.39, pipelines/driver_ms 10.70 |
| warm-up | 5.81 | 41.6 | pipelines/compile_ms 229, pipelines/front_ms 223, render/thread/ms 57.23, render/busy/ms 55.65, main/busy/ms 35.46, stage/render/prepare_assets 27.75, stage/render/render 14.39, stage/main/update 14.19, stage/main/post_update 13.58, render/graph/ms 12.07, encode//sum 8.78, stage/render/graph_submit 8.77 |
| warm-up | 5.87 | 34.9 | render/thread/ms 54.33, render/busy/ms 52.70, pipelines/compile_ms 33.01, pipelines/front_ms 31.40, main/busy/ms 28.03, stage/render/prepare_assets 25.96, stage/render/render 15.04, stage/main/post_update 12.74, render/graph/ms 12.43, encode//sum 10.61, stage/render/graph_submit 8.94, stage/main/update 8.86 |
| warm-up | 5.93 | 37.6 | pipelines/compile_ms 1092, pipelines/front_ms 1053, render/thread/ms 54.79, render/busy/ms 52.89, pipelines/driver_ms 39.37, stage/render/prepare_assets 31.82, main/busy/ms 29.65, stage/main/post_update 15.91, stage/render/render 14.33, render/graph/ms 11.78, stage/render/graph_submit 8.72, stage/main/update 8.56 |
| warm-up | 6.50 | 36.4 | render/thread/ms 50.14, render/busy/ms 49.11, main/busy/ms 31.05, stage/render/prepare_assets 18.54, stage/main/update 13.49, stage/main/post_update 9.53, stage/render/render 9.32, stage/render/specialize 8.73, encode//sum 7.82, render/graph/ms 7.77, stage/render/graph_submit 5.71, stream/drive/ms 4.64 |
| warm-up | 6.75 | 33.2 | pipelines/compile_ms 79.24, pipelines/front_ms 71.08, render/thread/ms 50.06, render/busy/ms 48.82, main/busy/ms 38.50, stage/main/update 16.58, stage/main/post_update 14.80, stage/render/specialize 10.92, stage/render/render 9.80, stage/render/prepare_assets 8.85, pipelines/driver_ms 8.16, render/graph/ms 8.14 |
| warm-up | 6.85 | 36.3 | main/busy/ms 49.50, render/thread/ms 40.43, render/busy/ms 38.62, stage/main/update 24.87, stage/main/post_update 19.28, stage/render/render 14.83, stream/drive/ms 14.47, render/graph/ms 12.14, stage/render/graph_submit 8.84, encode//sum 7.89, encode/submit 6.59, stage/render/prepare_assets 6.18 |
| warm-up | 6.89 | 36.1 | pipelines/compile_ms 164, pipelines/front_ms 158, render/thread/ms 49.68, render/busy/ms 48.32, main/busy/ms 43.31, stage/main/post_update 19.37, stage/main/update 18.02, stage/render/prepare_assets 14.31, stage/render/render 13.65, render/graph/ms 11.33, encode//sum 8.76, stage/render/graph_submit 7.42 |
| warm-up | 7.11 | 30.6 | pipelines/compile_ms 153, pipelines/front_ms 148, render/thread/ms 42.72, render/busy/ms 41.51, main/busy/ms 28.27, stage/render/prepare_assets 22.04, stage/main/post_update 13.60, stage/render/render 10.83, stage/main/update 9.83, render/graph/ms 9.08, encode//sum 8.01, stage/render/graph_submit 6.32 |
| warm-up | 7.17 | 34.8 | render/thread/ms 50.42, render/busy/ms 49.49, main/busy/ms 30.31, stage/render/prepare_assets 24.04, stage/main/update 12.53, stage/main/post_update 12.06, stage/render/render 10.11, encode//sum 8.92, render/graph/ms 8.51, stage/render/graph_submit 5.84, stage/render/specialize 5.15, stream/drive/ms 3.57 |
| warm-up | 7.34 | 38.3 | main/busy/ms 43.22, render/thread/ms 34.45, render/busy/ms 33.01, stage/main/update 24.62, stream/drive/ms 13.89, stage/render/prepare_assets 12.65, stage/render/render 11.34, stage/main/post_update 10.90, render/graph/ms 9.27, encode//sum 8.52, stage/render/graph_submit 6.28, encode/submit 3.39 |
| warm-up | 7.42 | 32.3 | pipelines/compile_ms 147, pipelines/front_ms 144, render/thread/ms 45.80, render/busy/ms 44.51, main/busy/ms 24.25, stage/render/render 11.87, stage/main/post_update 11.85, stage/render/specialize 11.70, render/graph/ms 10.01, encode//sum 8.05, stage/main/update 7.93, stage/render/queue 7.75 |
| measure | 9.79 | 31.7 | render/thread/ms 42.02, render/busy/ms 41.02, main/busy/ms 20.17, stage/render/render 12.51, stage/main/post_update 11.72, render/graph/ms 10.97, stage/render/specialize 10.91, encode//sum 9.39, stage/render/graph_submit 8.49, stage/render/queue 7.47, stage/main/update 5.34, encode/submit 5.02 |
| measure | 15.72 | 33.5 | render/thread/ms 35.18, render/busy/ms 34.27, main/busy/ms 19.61, stage/main/post_update 11.86, stage/render/specialize 10.61, stage/render/render 10.10, render/graph/ms 8.68, encode//sum 8.31, stage/render/queue 6.73, stage/render/graph_submit 6.44, stage/main/update 4.45, encode/submit 3.70 |
| measure | 18.16 | 31.8 | main/busy/ms 37.03, stage/main/update 23.87, stream/drive/ms 20.58, render/thread/ms 18.47, render/busy/ms 17.54, stage/main/post_update 10.59, stage/render/render 9.68, render/graph/ms 8.25, encode//sum 7.14, stage/render/graph_submit 5.72, encode/submit 3.60, stage/render/camera_driver 2.42 |
| measure | 19.45 | 32.1 | render/thread/ms 40.26, render/busy/ms 39.14, main/busy/ms 18.92, stage/render/render 12.38, stage/main/post_update 11.57, encode//sum 10.86, render/graph/ms 10.75, stage/render/specialize 10.26, stage/render/graph_submit 7.68, stage/render/queue 7.62, encode/pass:shadow_point_light/sum 5.79, encode/submit 4.89 |
| measure | 23.03 | 32.9 | render/thread/ms 44.05, render/busy/ms 42.92, main/busy/ms 26.30, stage/main/post_update 14.37, stage/render/render 13.93, render/graph/ms 12.20, stage/render/specialize 11.67, encode//sum 11.07, stage/render/graph_submit 8.92, stage/render/queue 8.15, stage/main/update 6.38, encode/pass:shadow_point_light/sum 5.17 |
| measure | 27.29 | 33.8 | render/thread/ms 44.32, render/busy/ms 43.20, main/busy/ms 27.11, stage/render/render 16.12, render/graph/ms 14.49, stage/main/post_update 14.45, encode//sum 12.30, stage/render/graph_submit 11.38, stage/render/specialize 11.12, encode/pass:shadow_point_light/sum 8.30, stage/render/queue 7.56, stage/main/update 6.97 |
| measure | 28.82 | 29.8 | render/thread/ms 41.10, render/busy/ms 40.06, main/busy/ms 18.74, stage/render/render 14.43, render/graph/ms 12.89, encode//sum 12.83, stage/main/post_update 11.59, stage/render/specialize 10.08, stage/render/graph_submit 9.30, encode/pass:shadow_point_light/sum 8.03, stage/render/queue 6.89, encode/submit 5.98 |
| measure | 30.48 | 34.7 | render/thread/ms 40.21, render/busy/ms 39.21, main/busy/ms 21.61, stage/main/post_update 13.26, stage/render/specialize 11.84, encode//sum 11.83, stage/render/render 11.59, render/graph/ms 10.06, stage/render/graph_submit 7.63, stage/render/queue 7.48, stage/main/update 5.52, encode/submit 4.27 |
| measure | 35.35 | 32.0 | render/thread/ms 40.35, render/busy/ms 39.23, main/busy/ms 20.13, stage/main/post_update 12.97, stage/render/render 12.92, render/graph/ms 11.29, stage/render/specialize 11.15, encode//sum 8.25, stage/render/graph_submit 8.14, stage/render/queue 7.41, encode/submit 4.88, stage/main/update 4.28 |
| measure | 36.79 | 29.8 | render/thread/ms 38.60, render/busy/ms 37.60, main/busy/ms 18.37, stage/render/render 12.45, stage/main/post_update 11.78, render/graph/ms 10.98, stage/render/specialize 10.51, encode//sum 9.64, stage/render/graph_submit 7.84, stage/render/queue 7.62, encode/submit 4.95, encode/pass:shadow_point_light/sum 4.33 |
| measure | 40.80 | 32.6 | render/thread/ms 38.60, render/busy/ms 37.59, main/busy/ms 17.90, stage/render/specialize 10.62, stage/render/render 10.45, stage/main/post_update 10.26, render/graph/ms 9.01, encode//sum 7.78, stage/render/queue 7.32, stage/render/graph_submit 6.90, stage/main/update 4.26, encode/submit 4.16 |
| measure | 43.23 | 30.8 | render/thread/ms 36.47, render/busy/ms 35.46, main/busy/ms 16.91, stage/render/render 10.64, stage/main/post_update 10.34, stage/render/specialize 10.32, render/graph/ms 9.23, encode//sum 7.80, stage/render/graph_submit 7.22, stage/render/queue 6.65, encode/submit 4.23, stage/main/update 4.04 |
| measure | 45.68 | 31.5 | render/thread/ms 40.21, render/busy/ms 39.23, main/busy/ms 18.11, stage/render/render 11.77, stage/render/specialize 11.54, stage/main/post_update 10.48, render/graph/ms 10.21, encode//sum 8.72, stage/render/graph_submit 8.00, stage/render/queue 7.14, stage/main/update 5.00, encode/submit 4.46 |
| measure | 59.04 | 48.3 | render/thread/ms 57.25, render/busy/ms 55.92, main/busy/ms 25.79, stage/render/render 14.41, stage/render/specialize 13.99, stage/main/post_update 13.89, stage/render/queue 13.59, render/graph/ms 12.67, encode//sum 10.16, stage/render/graph_submit 9.94, stage/main/update 7.42, encode/submit 5.66 |
| measure | 59.10 | 39.1 | render/thread/ms 39.37, render/busy/ms 38.34, stage/render/prepare_assets 20.17, main/busy/ms 17.13, stage/render/render 10.14, render/graph/ms 8.81, stage/main/post_update 8.79, encode//sum 8.59, stage/render/graph_submit 6.69, encode/submit 4.40, stage/main/update 4.36, encode/finish_wall 2.36 |
| measure | 62.58 | 30.9 | render/thread/ms 36.93, render/busy/ms 35.97, main/busy/ms 18.72, stage/render/specialize 10.95, stage/main/post_update 10.46, stage/render/render 10.39, render/graph/ms 9.17, encode//sum 8.45, stage/render/graph_submit 7.06, stage/render/queue 6.87, stage/main/update 5.53, encode/submit 3.64 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=nocull-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **78%** (min 38%) · 2619 MHz (min 2610) · memory 8101 MHz · 79.9 W · 78 °C (max 84) · VRAM 7027 MiB · throttled 100% of samples

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
