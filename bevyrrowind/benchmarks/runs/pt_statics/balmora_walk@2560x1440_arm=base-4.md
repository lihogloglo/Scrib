# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5999 frames over 65.7 s · **median 10.64 ms (94 fps)** · mean 10.94 · p95 13.62 · p99 16.76 (1% low 60 fps) · worst 40.73 ms · 75 spikes over 1.5× median · 59.4 stutters/min (work 51.2/min)
- warm-up: 324 frames over 9.0 s, worst 2467.6 ms
- hitches (> 26.6 ms): **31** measured, 53 in warm-up
- timed GPU passes: 6.68 ms of a 10.94 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.64 ms, limit 11 ms
- **FAIL** frame_ms: 65 stutters (> 1.5x moving median of 60) in 1.09 min = 59.40/min, limit 1/min
- **FAIL** worst frame 40.73 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.885 | 2.037 | 2.067 | 0.93 | 0.176 |
| main_opaque_pass_3d | 1.512 | 1.634 | 2.038 | 0.93 | 0.070 |
| early prepass | 0.455 | 0.492 | 0.716 | 0.93 | 0.089 |
| bloom | 0.291 | 0.315 | 0.321 | 0.93 | 0.063 |
| atmosphere_luts | 0.282 | 0.305 | 0.367 | 0.93 | 0.003 |
| water_velocity | 0.258 | 0.279 | 0.347 | 0.93 | 0.002 |
| water_sspr | 0.187 | 0.202 | 0.203 | 0.93 | 0.002 |
| ssao | 0.174 | 0.188 | 0.193 | 0.93 | 0.007 |
| auxiliary_views | 0.148 | 0.160 | 0.605 | 0.93 | 0.161 |
| atmos_froxel_inject | 0.135 | 0.146 | 0.147 | 0.93 | 0.001 |
| clouds_trace | 0.129 | 0.140 | 0.230 | 0.93 | 0.001 |
| uniform_allocation | 0.122 | 0.132 | 0.185 | 0.93 | 0.026 |
| water_fft | 0.121 | 0.131 | 0.138 | 0.93 | 0.003 |
| upscaling | 0.108 | 0.117 | 0.124 | 0.93 | 0.004 |
| clouds_composite | 0.098 | 0.106 | 0.108 | 0.93 | 0.002 |
| bin_unpacking | 0.097 | 0.105 | 0.142 | 0.93 | 0.019 |
| main_transmissive_pass_3d | 0.079 | 0.086 | 0.179 | 0.93 | 0.009 |
| tonemapping | 0.079 | 0.085 | 0.089 | 0.93 | 0.003 |
| clustering | 0.059 | 0.064 | 0.068 | 0.93 | 0.018 |
| render_sky | 0.048 | 0.052 | 0.053 | 0.93 | 0.002 |
| clouds_resolve | 0.045 | 0.049 | 0.049 | 0.93 | 0.001 |
| lightprobe_irradiance_map | 0.043 | 0.414 | 0.416 | 0.10 | 0.001 |
| surface_exposure | 0.040 | 0.043 | 0.044 | 0.93 | 0.001 |
| surface_trail | 0.037 | 0.056 | 0.057 | 0.65 | 0.002 |
| surface_exposure_far | 0.035 | 0.043 | 0.045 | 0.82 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.93 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.046 | 0.93 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.031 | 0.93 | 0.001 |
| lightprobe_radiance_map | 0.023 | 0.220 | 0.231 | 0.10 | 0.004 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.93 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.93 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.013 | 0.027 | 0.93 | 0.124 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.93 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.93 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.93 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.92 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.488 | 11.893 | 36.89 | 56916.8 |
| main/busy/ms | 9.072 | 10.845 | 28.17 | 54422.6 |
| render/busy/ms | 8.943 | 11.397 | 36.39 | 53648.4 |
| stage/main/post_update | 5.643 | 7.067 | 11.03 | 33849.9 |
| stage/render/render | 5.546 | 7.238 | 16.00 | 33268.7 |
| render/graph/ms | 4.809 | 6.471 | 15.02 | 28850.6 |
| encode//sum | 3.932 | 5.317 | 27.62 | 23585.4 |
| stage/render/graph_submit | 3.603 | 4.874 | 11.07 | 21612.8 |
| encode/submit | 2.316 | 3.306 | 7.03 | 13895.3 |
| stage/main/update | 2.157 | 2.586 | 21.67 | 12937.3 |
| encode/finish_wall | 1.354 | 1.948 | 4.82 | 8122.1 |
| stage/render/camera_driver | 1.154 | 1.632 | 4.02 | 6920.4 |
| encode/pass:shadow_point_light/sum | 0.927 | 3.484 | 18.09 | 5558.8 |
| encode/pass:shadow_directional_light/sum | 0.844 | 1.535 | 2.61 | 5063.3 |
| stage/render/prepare_write_phase_buffers | 0.775 | 1.102 | 2.61 | 4650.0 |
| encode/pass:main_opaque_pass_3d/sum | 0.755 | 1.072 | 1.71 | 4527.8 |
| encode/pass:early prepass/sum | 0.732 | 1.058 | 2.08 | 4391.2 |
| stage/render/prepare_assets | 0.571 | 0.744 | 10.41 | 3423.4 |
| bigspace/total/ms | 0.545 | 1.336 | 2.90 | 3271.7 |
| stage/render/prepare_meshes | 0.453 | 0.570 | 6.33 | 2717.9 |
| stage/render/prepare_resources | 0.452 | 0.540 | 1.05 | 2709.6 |
| render/present/ms | 0.450 | 0.670 | 2.77 | 2702.0 |
| stage/main/first | 0.401 | 0.524 | 5.25 | 2405.2 |
| stage/main/pre_update | 0.362 | 0.520 | 1.66 | 2174.2 |
| stage/render/specialize | 0.324 | 0.420 | 12.29 | 1943.2 |
| terrain/update/ms | 0.312 | 0.398 | 2.26 | 1869.0 |
| stage/render/queue | 0.289 | 0.415 | 8.16 | 1734.0 |
| stage/render/prepare_bind_groups | 0.287 | 0.521 | 2.37 | 1722.1 |
| terrain/select/ms | 0.277 | 0.355 | 1.00 | 1664.7 |
| controller/feed/ms | 0.269 | 0.339 | 0.83 | 1612.4 |
| stream/drive/ms | 0.254 | 0.340 | 19.99 | 1526.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.238 | 0.353 | 1.10 | 1425.6 |
| bigspace/low_precision/ms | 0.226 | 0.751 | 1.87 | 1357.6 |
| stage/main/fixed_main_loop | 0.217 | 0.372 | 1.05 | 1303.8 |
| far/select/ms | 0.198 | 0.271 | 0.55 | 1190.3 |
| stream/tick/ms | 0.197 | 0.270 | 0.69 | 1180.1 |
| stage/render/create_views | 0.195 | 0.253 | 1.27 | 1172.1 |
| stage/render/prepare_batch_phases | 0.192 | 0.276 | 0.64 | 1154.1 |
| stage/render/extract_commands | 0.172 | 0.200 | 0.41 | 1029.4 |
| anim/evaluate/ms | 0.165 | 0.282 | 1.84 | 989.4 |
| bigspace/high_precision/ms | 0.156 | 0.692 | 2.29 | 936.7 |
| anim/evaluate_cpu/ms | 0.155 | 0.194 | 0.71 | 931.4 |
| stage/render/prepare_flush | 0.153 | 0.301 | 0.93 | 920.1 |
| controller/move/ms | 0.143 | 0.319 | 0.57 | 856.2 |
| stage/render/prepare_views | 0.138 | 0.171 | 0.47 | 825.4 |
| stage/main/last | 0.115 | 0.163 | 1.59 | 691.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.100 | 0.159 | 0.80 | 597.1 |
| render/acquire/ms | 0.094 | 0.120 | 0.31 | 566.4 |
| stage/render/prepare_collect_phase_buffers | 0.070 | 0.139 | 0.74 | 417.5 |
| pipelines/compile_ms | 0.050 | 0.000 | 120.40 | 298.9 |
| physics/step/ms | 0.048 | 0.088 | 0.54 | 286.7 |
| pipelines/front_ms | 0.048 | 0.000 | 116.33 | 286.3 |
| physics/insert/ms | 0.033 | 0.373 | 0.94 | 196.0 |
| stage/render/cleanup | 0.033 | 0.098 | 0.93 | 195.0 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.60 | 187.0 |
| stage/render/phase_sort | 0.030 | 0.052 | 0.27 | 182.4 |
| weather/map/ms | 0.025 | 0.033 | 0.19 | 151.9 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.50 | 88.1 |
| anim/apply/ms | 0.012 | 0.021 | 0.20 | 70.1 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.55 | 68.1 |
| controller/avoid/ms | 0.011 | 0.014 | 0.18 | 64.1 |
| weather/eval/ms | 0.010 | 0.014 | 0.25 | 61.9 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 43.1 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 32.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.06 | 32.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.15 | 28.3 |
| script/ui/ms | 0.004 | 0.006 | 0.42 | 26.7 |
| controller/write/ms | 0.003 | 0.004 | 0.21 | 19.9 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 14.5 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.46 | 12.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.07 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.37 | 4.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.09 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.11 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 33 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 26 | 42 | 14 |
| crowd/c0/joints/count | count | 1272 | 2218 | 641 |
| crowd/c0/meshes/count | count | 353 | 633 | 174 |
| crowd/c1/count | count | 146 | 155 | 146 |
| crowd/c1/joints/count | count | 5531 | 5894 | 5523 |
| crowd/c1/meshes/count | count | 1167 | 1254 | 1159 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4618 | 10.0 k | 6528 |
| crowd/joints_uploaded/count | count | 4597 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.41 | 1.53 | 0.43 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.50 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.55 | 0.01 |
| encode/finish_critical | ms | 0.96 | 2.61 | 1.33 |
| encode/pass:early prepass/max | ms | 0.75 | 2.08 | 0.87 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.77 | 1.71 | 0.80 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.80 | 0.12 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.10 | 0.33 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.61 | 1.33 |
| encode/pass:shadow_point_light/max | ms | 0.80 | 1.52 | 0.91 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.60 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.27 | 36.39 | 9.51 |
| gpu/timed/ms | ms | 7.91 | 9.29 | 8.16 |
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
| phases/alpha_mask3d/items | count | 809 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 809 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1296 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3399 | 5110 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1296 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3399 | 5110 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 20 | 207 | 30 |
| phases/shadow/bins | count | 5030 | 8911 | 2423 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6378 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 206 | 257 | 30 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2349 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 173 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 571 | 63 |
| preprocess/bind_groups_created | count | 13 | 571 | 63 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 244 | 120 |
| process/cpu_usage | % | 26.46 | 41.26 | 32.21 |
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
| skin/blocks_changed/count | count | 14 | 119 | 14 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 366.6 k | 811.5 k | 495.1 k |
| skin/joints_written/count | count | 5728 | 12.7 k | 7736 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4918 | 5011 | 5011 |
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
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.33 | 48.07 | 29.91 |
| system/mem_usage | % | 62.41 | 63.03 | 62.92 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 70 | 256 | 80 |
| views/all/bins | count | 7641 | 11.8 k | 5921 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 253 | 304 | 76 |
| views/all/pipelines_distinct | count | 71 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 34.9 k | 14.3 k |
| views/cascade/batch_sets | count | 9 | 51 | 30 |
| views/cascade/bins | count | 2806 | 6183 | 2423 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8908 | 23.5 k | 6378 |
| views/cascade/pipelines | count | 36 | 54 | 30 |
| views/cascade/work_items | count | 412 | 23.1 k | 6378 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2159 | 2760 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2417 | 3058 | 0 |
| views/lamp_face/pipelines | count | 168 | 209 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2933 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2888 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8406 | 13.0 k | 7618 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8575 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 56.63 | 218 | 339.7 k |
| anim/events/count | count | 0.09 | 4 | 537 |
| assets/Image/added | count | 0.10 | 39 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.68 | 152 | 4061 |
| assets/Mesh/removed | count | 0.81 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.61 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.78 | 1296 | 4675 |
| encode//count | count | 34.00 | 220 | 204.0 k |
| encode/bind_walk/init_textures | count | 12.9 k | 114.7 k | 77.41 M |
| encode/bind_walk/init_textures_skipped | count | 56.4 k | 146.4 k | 338.08 M |
| encode/bind_walk/inits | count | 462 | 1131 | 2.77 M |
| encode/bind_walk/inits_skipped | count | 101 | 152 | 607.4 k |
| encode/bind_walk/merge_textures | count | 12.8 k | 114.6 k | 76.97 M |
| encode/bind_walk/merge_textures_skipped | count | 5367 | 6012 | 32.20 M |
| encode/bind_walk/merges | count | 99.47 | 336 | 596.7 k |
| encode/bind_walk/merges_skipped | count | 80.77 | 129 | 484.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5999 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5999 |
| encode/pass:early prepass/count | count | 1 | 1 | 5999 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5999 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5999 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5999 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5999 |
| encode/pass:shadow_point_light/count | count | 1.51 | 48 | 9079 |
| encode/queue/command_buffers | count | 45.53 | 278 | 273.1 k |
| encode/queue/submits | count | 2.02 | 4 | 12.1 k |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 4.89 M | 11.31 G |
| encode/queue/write_buffer/calls | count | 263 | 854 | 1.58 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.7 k | 3.52 M | 1.07 G |
| encode/queue/write_buffer_with/calls | count | 11.02 | 465 | 66.1 k |
| encode/queue/write_texture/bytes | bytes | 11.8 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5999 |
| physics/inserts | count | 0.37 | 111 | 2246 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 161 | 2335 |
| physics/steps | count | 0.66 | 3 | 3939 |
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
| stream/upload/bytes | bytes | 5643 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1237 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.6 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2467.6 | main/busy/ms 2438, stage/main/post_update 2334, stage/main/update 95.29, surface/advance_cpu_ms 66.36, stage/main/pre_update 4.52, stage/main/last 1.12, surface/cell_upload_ms 0.76, stage/main/first 0.54, stage/main/fixed_main_loop 0.38, stream/drive/ms 0.29, weather/map/ms 0.14, bigspace/total/ms 0.11 |
| warm-up | 3.17 | 701.1 | main/busy/ms 2611, stage/main/post_update 2356, render/thread/ms 671, render/busy/ms 559, stage/render/prepare_assets 532, stage/main/update 240, stage/render/prepare_views 109, render/acquire/ms 109, surface/advance_cpu_ms 66.36, stage/render/render 11.45, stage/render/create_views 8.33, render/graph/ms 7.39 |
| warm-up | 3.28 | 111.8 | render/thread/ms 765, render/busy/ms 598, stage/render/prepare_assets 559, main/busy/ms 189, stage/main/update 149, pipelines/compile_ms 121, stage/render/prepare_views 110, render/acquire/ms 109, pipelines/front_ms 78.63, stage/render/render 71.28, render/present/ms 58.04, pipelines/driver_ms 42.36 |
| warm-up | 3.46 | 147.4 | pipelines/wait_ms 4562, pipelines/compile_ms 1152, pipelines/front_ms 1066, render/thread/ms 143, render/busy/ms 136, pipelines/driver_ms 86.30, main/busy/ms 82.11, stage/render/prepare_flush 52.37, stage/main/post_update 48.59, stage/render/render 29.65, stage/render/prepare_assets 24.48, render/graph/ms 22.15 |
| warm-up | 3.50 | 41.8 | pipelines/wait_ms 4718, pipelines/compile_ms 1882, pipelines/front_ms 1760, render/thread/ms 151, render/busy/ms 146, pipelines/driver_ms 122, main/busy/ms 111, stage/main/update 57.49, stage/render/prepare_flush 52.56, stage/render/prepare_assets 30.49, stage/render/prepare_resources 23.03, stage/render/render 22.91 |
| warm-up | 3.61 | 108.1 | pipelines/compile_ms 1985, pipelines/front_ms 1877, pipelines/wait_ms 397, pipelines/driver_ms 109, render/thread/ms 96.87, render/busy/ms 91.61, main/busy/ms 89.34, stage/main/update 60.39, stage/render/prepare_assets 58.09, stage/render/render 19.76, stage/main/post_update 14.30, render/graph/ms 13.91 |
| warm-up | 3.67 | 66.1 | pipelines/compile_ms 2178, pipelines/front_ms 1939, pipelines/driver_ms 239, pipelines/wait_ms 209, render/thread/ms 126, render/busy/ms 123, stage/render/prepare_assets 88.99, stage/render/render 29.78, main/busy/ms 27.87, render/graph/ms 26.06, stage/render/graph_submit 21.95, encode/submit 12.65 |
| warm-up | 3.75 | 33.6 | pipelines/compile_ms 37.43, render/thread/ms 36.76, pipelines/front_ms 35.71, render/busy/ms 35.30, main/busy/ms 26.70, stage/render/prepare_assets 18.67, stage/main/post_update 15.38, pipelines/wait_ms 9.74, stage/render/render 9.71, pipelines/lock_ms 9.62, terrain/update/ms 9.21, encode//sum 9.05 |
| warm-up | 3.84 | 83.5 | pipelines/compile_ms 1243, pipelines/front_ms 1201, main/busy/ms 56.41, render/thread/ms 50.90, render/busy/ms 49.03, stage/main/update 43.70, pipelines/driver_ms 42.61, stream/drive/ms 38.85, stage/render/prepare_assets 32.77, stage/render/render 11.18, render/graph/ms 8.83, stage/main/post_update 7.74 |
| warm-up | 4.00 | 75.8 | pipelines/compile_ms 482, pipelines/front_ms 464, render/thread/ms 72.67, render/busy/ms 70.11, main/busy/ms 51.68, stage/render/prepare_assets 45.75, stage/main/update 33.08, stream/drive/ms 29.31, pipelines/driver_ms 17.94, stage/render/render 13.75, stage/main/post_update 10.89, render/graph/ms 10.65 |
| warm-up | 4.04 | 38.0 | pipelines/compile_ms 283, pipelines/front_ms 271, render/thread/ms 71.20, render/busy/ms 69.65, stage/render/prepare_assets 52.31, main/busy/ms 28.51, stage/render/render 11.77, pipelines/driver_ms 11.64, stage/main/update 10.99, render/graph/ms 9.51, stage/main/post_update 9.43, stage/render/graph_submit 7.20 |
| warm-up | 4.10 | 66.0 | pipelines/compile_ms 1177, pipelines/front_ms 1127, render/thread/ms 68.33, render/busy/ms 66.80, stage/render/prepare_assets 51.80, pipelines/driver_ms 49.81, main/busy/ms 37.73, pipelines/wait_ms 20.43, stage/main/update 18.05, stage/main/post_update 12.32, stage/render/render 10.05, render/graph/ms 8.02 |
| warm-up | 4.19 | 88.6 | pipelines/compile_ms 1638, pipelines/front_ms 1567, render/thread/ms 124, render/busy/ms 122, main/busy/ms 97.79, pipelines/driver_ms 70.59, stage/render/prepare_assets 57.32, stage/main/update 45.04, stage/render/prepare_resources 33.02, stage/main/post_update 25.17, stage/main/pre_update 23.11, pipelines/wait_ms 19.46 |
| warm-up | 4.27 | 84.6 | pipelines/compile_ms 1708, pipelines/front_ms 1647, render/thread/ms 136, render/busy/ms 134, main/busy/ms 112, pipelines/wait_ms 88.61, pipelines/driver_ms 60.88, stage/render/prepare_assets 55.11, stage/main/update 41.43, stage/render/prepare_resources 36.46, stage/main/post_update 32.45, stage/main/pre_update 31.99 |
| warm-up | 4.34 | 62.0 | pipelines/compile_ms 2724, pipelines/front_ms 2626, pipelines/wait_ms 447, render/thread/ms 113, render/busy/ms 110, pipelines/driver_ms 98.61, main/busy/ms 71.15, stage/render/prepare_assets 38.24, encode//sum 36.00, stage/main/post_update 30.25, stage/main/update 24.04, stage/render/render 23.13 |
| warm-up | 4.44 | 106.3 | pipelines/compile_ms 2140, pipelines/front_ms 2057, pipelines/wait_ms 572, render/thread/ms 154, render/busy/ms 150, pipelines/driver_ms 83.05, main/busy/ms 61.73, encode//sum 40.65, stage/main/post_update 31.83, stage/render/prepare_write_phase_buffers 27.27, stage/render/render 25.08, stage/main/update 23.80 |
| warm-up | 4.49 | 45.7 | pipelines/compile_ms 1695, pipelines/front_ms 1641, pipelines/wait_ms 1053, render/thread/ms 132, render/busy/ms 131, main/busy/ms 55.41, pipelines/driver_ms 53.33, stage/render/prepare_assets 31.52, stage/render/prepare_write_phase_buffers 28.47, stage/main/post_update 27.84, stage/main/update 20.15, stage/render/prepare_resources 19.83 |
| warm-up | 4.56 | 73.4 | pipelines/compile_ms 2364, pipelines/front_ms 2298, pipelines/wait_ms 2105, main/busy/ms 96.56, render/thread/ms 78.64, render/busy/ms 77.25, pipelines/driver_ms 65.39, stage/main/update 62.39, stream/drive/ms 50.66, stage/render/prepare_assets 41.66, stage/render/render 17.75, stage/main/post_update 17.50 |
| warm-up | 4.64 | 79.7 | pipelines/compile_ms 1749, pipelines/front_ms 1701, pipelines/wait_ms 1474, main/busy/ms 110, render/thread/ms 90.97, render/busy/ms 89.43, stage/main/update 75.13, stage/render/prepare_assets 57.75, stream/drive/ms 51.44, pipelines/driver_ms 48.39, stage/main/post_update 18.14, stage/render/render 13.59 |
| warm-up | 4.69 | 45.7 | pipelines/compile_ms 1360, pipelines/front_ms 1323, pipelines/wait_ms 618, main/busy/ms 119, stage/main/update 91.23, render/thread/ms 80.68, render/busy/ms 78.99, stream/drive/ms 54.95, stage/render/prepare_assets 48.47, pipelines/driver_ms 37.35, stage/main/post_update 15.93, stage/render/render 14.32 |
| warm-up | 4.78 | 94.1 | pipelines/compile_ms 1738, pipelines/front_ms 1700, pipelines/wait_ms 756, main/busy/ms 92.55, stage/main/update 73.65, render/thread/ms 69.82, render/busy/ms 68.22, stream/drive/ms 54.27, pipelines/driver_ms 38.56, stage/render/prepare_assets 35.07, stage/render/render 19.78, render/graph/ms 17.42 |
| warm-up | 4.89 | 75.8 | pipelines/compile_ms 460, pipelines/front_ms 441, main/busy/ms 83.27, stage/main/update 59.41, render/thread/ms 56.01, render/busy/ms 53.89, stream/drive/ms 39.43, stage/render/prepare_assets 20.95, stage/render/render 20.18, pipelines/driver_ms 19.63, render/graph/ms 17.26, stage/main/post_update 15.59 |
| warm-up | 4.93 | 41.5 | pipelines/compile_ms 498, pipelines/front_ms 479, main/busy/ms 79.77, stage/main/update 59.88, render/thread/ms 59.02, render/busy/ms 57.49, stream/drive/ms 33.80, stage/render/prepare_assets 30.35, pipelines/driver_ms 18.23, stage/main/post_update 12.89, stage/render/render 11.43, render/graph/ms 9.27 |
| warm-up | 4.99 | 61.2 | pipelines/compile_ms 842, pipelines/front_ms 815, main/busy/ms 69.24, render/thread/ms 54.86, stage/main/update 54.01, render/busy/ms 53.11, stream/drive/ms 34.48, pipelines/driver_ms 27.19, stage/render/prepare_assets 25.13, stage/render/render 11.11, render/graph/ms 8.65, stage/main/post_update 8.11 |
| warm-up | 5.13 | 33.9 | pipelines/compile_ms 834, pipelines/front_ms 809, render/thread/ms 50.97, render/busy/ms 49.20, main/busy/ms 41.40, pipelines/driver_ms 25.40, stage/main/update 17.71, stage/render/prepare_assets 14.30, stage/render/render 13.68, stage/main/post_update 12.46, render/graph/ms 11.37, encode//sum 8.87 |
| warm-up | 5.17 | 42.9 | pipelines/compile_ms 766, pipelines/front_ms 741, render/thread/ms 61.78, render/busy/ms 60.30, main/busy/ms 44.72, stage/render/prepare_assets 32.28, pipelines/driver_ms 24.57, stage/main/update 22.27, stage/render/render 12.43, stage/main/post_update 10.69, render/graph/ms 10.35, encode//sum 8.33 |
| warm-up | 5.28 | 35.4 | pipelines/compile_ms 553, pipelines/front_ms 541, render/thread/ms 48.89, render/busy/ms 47.45, main/busy/ms 35.58, stage/render/prepare_assets 22.58, stage/main/update 16.75, stage/render/render 13.24, stage/main/post_update 11.94, pipelines/driver_ms 11.51, render/graph/ms 11.16, encode//sum 8.52 |
| warm-up | 5.38 | 39.2 | pipelines/compile_ms 551, pipelines/front_ms 540, render/thread/ms 50.11, render/busy/ms 48.74, main/busy/ms 36.10, stage/render/prepare_assets 25.76, stage/main/update 19.99, stage/render/render 12.08, pipelines/driver_ms 11.47, stage/main/post_update 11.27, render/graph/ms 10.03, encode//sum 9.13 |
| warm-up | 5.47 | 34.4 | pipelines/compile_ms 115, pipelines/front_ms 109, render/thread/ms 51.55, render/busy/ms 50.27, main/busy/ms 39.73, stage/render/prepare_assets 23.11, stage/main/update 22.67, stage/render/render 10.92, stage/main/post_update 10.29, render/graph/ms 9.02, encode//sum 7.75, stage/render/prepare_meshes 6.50 |
| warm-up | 5.51 | 41.2 | pipelines/compile_ms 814, pipelines/front_ms 782, render/thread/ms 66.97, render/busy/ms 65.58, main/busy/ms 42.89, stage/render/prepare_assets 35.62, pipelines/driver_ms 31.22, pipelines/wait_ms 30.99, stage/main/update 23.90, stage/render/render 13.46, stage/main/post_update 12.98, render/graph/ms 11.27 |
| warm-up | 5.55 | 29.6 | render/thread/ms 38.05, render/busy/ms 36.61, main/busy/ms 27.75, pipelines/compile_ms 17.49, stage/render/prepare_assets 16.26, pipelines/front_ms 16.00, stage/render/render 14.47, stage/main/update 12.83, render/graph/ms 12.27, stage/main/post_update 9.81, stage/render/graph_submit 8.97, encode//sum 8.86 |
| warm-up | 6.24 | 29.5 | main/busy/ms 47.94, render/thread/ms 40.60, render/busy/ms 39.07, stage/main/update 23.21, stage/main/post_update 16.46, stage/render/render 15.17, render/graph/ms 12.70, stage/render/graph_submit 9.76, encode//sum 9.31, stage/render/prepare_assets 5.87, encode/submit 5.71, stream/drive/ms 5.69 |
| warm-up | 6.27 | 33.3 | pipelines/compile_ms 79.52, pipelines/front_ms 75.56, main/busy/ms 44.24, render/thread/ms 42.98, render/busy/ms 41.44, stage/main/update 19.26, stage/main/post_update 15.40, stage/render/render 13.43, stage/render/prepare_assets 12.56, render/graph/ms 11.13, encode//sum 9.50, stage/render/graph_submit 8.41 |
| warm-up | 6.30 | 30.0 | pipelines/compile_ms 228, pipelines/front_ms 219, render/thread/ms 41.41, render/busy/ms 40.40, main/busy/ms 32.58, stage/render/prepare_assets 19.42, stage/main/update 13.07, stage/main/post_update 12.40, pipelines/driver_ms 8.22, stage/render/render 7.90, encode//sum 7.22, render/graph/ms 6.39 |
| warm-up | 6.39 | 39.1 | main/busy/ms 44.20, render/thread/ms 36.52, render/busy/ms 34.89, stage/main/update 26.90, pipelines/compile_ms 15.92, pipelines/front_ms 15.12, stage/render/prepare_assets 14.08, stream/drive/ms 14.00, stage/render/render 12.30, stage/main/post_update 12.07, render/graph/ms 10.07, encode//sum 8.39 |
| warm-up | 6.88 | 31.1 | pipelines/compile_ms 141, pipelines/front_ms 138, render/thread/ms 39.05, render/busy/ms 37.90, main/busy/ms 18.49, stage/render/render 11.24, stage/main/post_update 10.01, render/graph/ms 9.74, stage/render/specialize 9.52, encode//sum 8.02, stage/render/graph_submit 7.30, stage/render/queue 6.91 |
| warm-up | 8.17 | 32.0 | render/thread/ms 37.30, render/busy/ms 36.28, main/busy/ms 18.62, stage/render/render 10.34, stage/main/post_update 10.24, stage/render/specialize 10.04, render/graph/ms 8.91, encode//sum 7.37, stage/render/graph_submit 6.77, stage/render/queue 6.17, stage/main/update 5.34, stage/render/prepare_meshes 4.28 |
| measure | 0.41 | 31.7 | render/thread/ms 49.29, render/busy/ms 47.53, encode//sum 26.15, stage/render/render 21.27, render/graph/ms 18.47, main/busy/ms 16.60, stage/render/graph_submit 14.46, encode/pass:shadow_point_light/sum 11.42, stage/main/post_update 10.06, stage/render/specialize 9.39, encode/submit 8.51, stage/render/queue 6.00 |
| measure | 1.67 | 31.4 | render/thread/ms 35.12, render/busy/ms 33.83, main/busy/ms 17.86, stage/main/post_update 10.86, stage/render/render 10.06, stage/render/specialize 8.94, render/graph/ms 8.37, encode//sum 7.48, stage/render/graph_submit 6.20, stage/render/queue 5.99, stage/main/update 4.46, stage/render/prepare_meshes 4.24 |
| measure | 4.50 | 32.1 | render/thread/ms 38.28, render/busy/ms 37.16, main/busy/ms 19.23, stage/main/post_update 12.72, stage/render/render 12.60, render/graph/ms 10.99, encode//sum 10.52, stage/render/specialize 9.41, stage/render/graph_submit 8.01, stage/render/queue 6.45, encode/pass:shadow_point_light/sum 5.16, encode/submit 4.74 |
| measure | 5.83 | 31.5 | render/thread/ms 35.00, render/busy/ms 33.69, main/busy/ms 19.69, stage/main/post_update 12.67, stage/render/render 10.05, stage/render/specialize 9.06, render/graph/ms 8.26, encode//sum 7.04, stage/render/queue 6.71, stage/render/graph_submit 6.37, stage/main/update 4.22, encode/submit 3.56 |
| measure | 8.50 | 30.8 | render/thread/ms 36.12, render/busy/ms 35.04, main/busy/ms 18.64, stage/main/post_update 11.82, stage/render/render 11.63, render/graph/ms 9.96, stage/render/specialize 9.12, encode//sum 8.64, stage/render/graph_submit 7.61, stage/render/queue 6.49, encode/submit 4.43, stage/main/update 4.06 |
| measure | 9.87 | 30.4 | render/thread/ms 36.49, render/busy/ms 35.39, main/busy/ms 18.14, stage/render/render 10.62, stage/main/post_update 10.60, stage/render/specialize 9.43, render/graph/ms 9.06, encode//sum 7.98, stage/render/graph_submit 6.79, stage/render/queue 6.56, stage/main/update 4.92, encode/submit 4.01 |
| measure | 11.18 | 31.8 | render/thread/ms 38.39, render/busy/ms 37.22, main/busy/ms 18.53, stage/render/render 12.91, stage/main/post_update 11.69, render/graph/ms 11.23, encode//sum 9.76, stage/render/specialize 9.43, stage/render/graph_submit 8.83, stage/render/queue 6.49, encode/submit 5.76, stage/main/update 4.15 |
| measure | 15.72 | 33.4 | render/thread/ms 37.25, render/busy/ms 36.13, main/busy/ms 18.57, stage/main/post_update 11.71, stage/render/render 10.57, stage/render/specialize 10.47, render/graph/ms 9.07, stage/render/queue 7.56, stage/render/graph_submit 7.12, encode//sum 6.60, stage/main/update 4.07, encode/submit 3.86 |
| measure | 17.03 | 32.9 | render/thread/ms 34.65, render/busy/ms 33.62, main/busy/ms 18.70, stage/main/post_update 11.05, stage/render/specialize 10.18, stage/render/render 9.14, render/graph/ms 7.77, encode//sum 7.24, stage/render/queue 6.77, stage/render/graph_submit 5.78, stage/main/update 3.91, encode/submit 3.38 |
| measure | 18.00 | 30.5 | render/thread/ms 35.64, render/busy/ms 34.31, main/busy/ms 17.57, stage/main/post_update 11.20, stage/render/render 10.38, stage/render/specialize 9.35, render/graph/ms 8.59, encode//sum 6.86, stage/render/queue 6.56, stage/render/graph_submit 6.44, encode/submit 3.86, stage/main/update 3.64 |
| measure | 18.18 | 32.0 | main/busy/ms 37.61, stage/main/update 23.33, render/thread/ms 20.44, stream/drive/ms 20.24, render/busy/ms 19.27, stage/main/post_update 11.66, stage/render/render 11.10, render/graph/ms 9.45, encode//sum 8.77, stage/render/graph_submit 6.88, encode/submit 4.31, encode/pass:shadow_point_light/sum 3.71 |
| measure | 19.39 | 30.4 | render/thread/ms 32.97, render/busy/ms 32.07, main/busy/ms 18.10, stage/main/post_update 11.05, stage/render/specialize 9.59, stage/render/render 9.23, render/graph/ms 7.80, encode//sum 6.42, stage/render/queue 6.09, stage/render/graph_submit 5.98, stage/main/update 4.05, encode/submit 3.57 |
| measure | 20.81 | 30.8 | render/thread/ms 37.91, render/busy/ms 36.67, main/busy/ms 21.32, stage/main/post_update 12.52, stage/render/render 11.12, stage/render/specialize 10.41, encode//sum 9.61, render/graph/ms 9.54, stage/render/graph_submit 7.20, stage/render/queue 7.06, stage/main/update 5.85, encode/submit 4.35 |
| measure | 23.02 | 33.6 | render/thread/ms 42.50, render/busy/ms 41.46, main/busy/ms 25.01, stage/main/post_update 12.55, stage/render/render 12.25, stage/render/specialize 11.93, render/graph/ms 10.74, encode//sum 9.34, stage/render/queue 8.02, stage/render/graph_submit 7.71, stage/main/update 6.49, encode/submit 4.72 |
| measure | 27.29 | 37.2 | render/thread/ms 46.61, render/busy/ms 45.53, main/busy/ms 25.65, stage/render/render 15.34, render/graph/ms 13.82, stage/main/post_update 13.31, stage/render/specialize 12.36, encode//sum 12.26, stage/render/graph_submit 10.46, encode/pass:shadow_point_light/sum 9.23, stage/render/queue 8.46, encode/submit 6.61 |
| measure | 28.88 | 31.5 | render/thread/ms 40.53, render/busy/ms 39.36, main/busy/ms 19.19, stage/render/render 13.90, stage/main/post_update 12.51, render/graph/ms 12.28, encode//sum 11.02, stage/render/specialize 9.67, stage/render/graph_submit 9.44, stage/render/queue 6.83, encode/pass:shadow_point_light/sum 6.36, encode/submit 5.74 |
| measure | 30.48 | 33.6 | render/thread/ms 42.52, render/busy/ms 41.38, main/busy/ms 21.92, stage/render/render 13.14, stage/main/post_update 12.49, render/graph/ms 11.39, stage/render/specialize 11.07, encode//sum 9.54, stage/render/graph_submit 8.98, stage/render/queue 7.77, stage/main/update 6.53, encode/submit 5.07 |
| measure | 31.96 | 30.4 | render/thread/ms 39.91, render/busy/ms 38.92, main/busy/ms 19.48, stage/main/post_update 12.33, stage/render/render 12.16, stage/render/specialize 10.66, render/graph/ms 10.57, encode//sum 8.22, stage/render/graph_submit 8.15, stage/render/queue 7.35, encode/submit 4.64, stage/main/update 4.18 |
| measure | 35.37 | 31.1 | render/thread/ms 39.03, render/busy/ms 37.93, main/busy/ms 20.58, stage/render/render 12.23, stage/main/post_update 12.00, render/graph/ms 10.68, stage/render/specialize 10.59, stage/render/graph_submit 8.30, encode//sum 8.00, stage/render/queue 6.98, stage/main/update 4.87, encode/submit 4.66 |
| measure | 37.23 | 36.4 | render/thread/ms 44.20, render/busy/ms 43.18, main/busy/ms 24.25, stage/render/render 13.21, stage/main/post_update 13.06, stage/render/specialize 11.81, render/graph/ms 11.67, encode//sum 9.11, stage/render/graph_submit 8.93, stage/render/queue 7.91, stage/main/update 7.43, encode/submit 5.11 |
| measure | 40.79 | 31.5 | render/thread/ms 37.82, render/busy/ms 36.84, main/busy/ms 17.80, stage/render/specialize 10.08, stage/render/render 9.90, stage/main/post_update 9.84, render/graph/ms 8.55, encode//sum 6.81, stage/render/queue 6.77, stage/render/graph_submit 6.62, stage/main/update 4.59, encode/submit 3.94 |
| measure | 43.22 | 30.3 | render/thread/ms 38.45, render/busy/ms 37.44, main/busy/ms 17.33, stage/render/render 11.01, stage/render/specialize 10.30, stage/main/post_update 10.10, render/graph/ms 9.59, stage/render/graph_submit 7.77, encode//sum 7.32, stage/render/queue 6.62, stage/main/update 4.24, encode/submit 4.08 |
| measure | 45.67 | 32.8 | render/thread/ms 37.97, render/busy/ms 35.73, main/busy/ms 16.88, stage/render/render 10.53, stage/render/specialize 10.04, stage/main/post_update 9.73, render/graph/ms 7.90, encode//sum 7.61, stage/render/queue 6.94, stage/render/graph_submit 6.00, stage/render/prepare_meshes 4.35, stage/main/update 4.33 |
| measure | 56.44 | 29.6 | render/thread/ms 37.08, render/busy/ms 35.21, main/busy/ms 15.68, stage/render/render 10.58, stage/render/specialize 10.26, stage/main/post_update 9.58, render/graph/ms 8.49, encode//sum 7.52, stage/render/queue 6.88, stage/render/graph_submit 6.49, encode/submit 4.64, stage/main/update 4.06 |
| measure | 59.04 | 40.7 | render/thread/ms 53.78, render/busy/ms 52.29, main/busy/ms 33.73, stage/main/post_update 19.83, stage/render/render 17.03, render/graph/ms 15.09, stage/render/specialize 12.57, stage/render/graph_submit 11.89, encode//sum 10.52, stage/main/update 9.50, encode/submit 7.19, stage/render/prepare_meshes 6.82 |
| measure | 59.07 | 29.8 | render/thread/ms 56.40, render/busy/ms 55.39, main/busy/ms 29.09, stage/main/post_update 15.30, stage/render/prepare_assets 13.39, stage/render/specialize 12.58, stage/render/render 10.23, render/graph/ms 8.91, stage/main/update 8.29, encode//sum 7.90, stage/render/prepare_meshes 7.53, stage/render/graph_submit 6.83 |
| measure | 62.57 | 31.7 | render/thread/ms 39.29, render/busy/ms 38.23, main/busy/ms 18.53, stage/render/render 10.92, stage/render/specialize 10.20, render/graph/ms 9.67, stage/main/post_update 9.44, encode//sum 7.94, stage/render/graph_submit 7.60, stage/render/queue 6.69, stage/main/update 6.06, stage/render/prepare_meshes 4.67 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=base-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 14%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **77%** (min 41%) · 2623 MHz (min 2610) · memory 8101 MHz · 80.1 W · 78 °C (max 84) · VRAM 6765 MiB · throttled 100% of samples

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
