# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.enabled=on (command line: --set); shadow.lamp.atlas.max_lamps=8 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6619 frames over 65.6 s · **median 9.79 ms (102 fps)** · mean 9.92 · p95 11.62 · p99 13.39 (1% low 75 fps) · worst 32.92 ms · 39 spikes over 1.5× median · 36.6 stutters/min (work 22.8/min)
- warm-up: 353 frames over 9.1 s, worst 2531.1 ms
- hitches (> 25.0 ms): **4** measured, 45 in warm-up
- timed GPU passes: 6.31 ms of a 9.92 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.79 ms, limit 11 ms
- **FAIL** frame_ms: 40 stutters (> 1.5x moving median of 60) in 1.09 min = 36.56/min, limit 1/min
- pass worst frame 32.92 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.940 | 2.187 | 2.614 | 0.89 | 0.175 |
| main_opaque_pass_3d | 1.217 | 1.372 | 1.942 | 0.89 | 0.068 |
| early prepass | 0.470 | 0.530 | 0.769 | 0.89 | 0.093 |
| bloom | 0.305 | 0.344 | 0.663 | 0.89 | 0.066 |
| atmosphere_luts | 0.304 | 0.343 | 0.644 | 0.89 | 0.003 |
| water_velocity | 0.214 | 0.241 | 0.249 | 0.89 | 0.002 |
| water_sspr | 0.196 | 0.221 | 0.349 | 0.89 | 0.002 |
| ssao | 0.175 | 0.198 | 0.205 | 0.89 | 0.008 |
| atmos_froxel_inject | 0.122 | 0.138 | 0.131 | 0.89 | 0.002 |
| upscaling | 0.122 | 0.137 | 0.161 | 0.89 | 0.004 |
| water_fft | 0.114 | 0.129 | 0.127 | 0.89 | 0.003 |
| point_shadow_atlas | 0.107 | 0.201 | 0.393 | 0.53 | 0.113 |
| uniform_allocation | 0.104 | 0.117 | 0.139 | 0.89 | 0.023 |
| clouds_composite | 0.095 | 0.107 | 0.109 | 0.89 | 0.002 |
| bin_unpacking | 0.091 | 0.102 | 0.116 | 0.89 | 0.017 |
| tonemapping | 0.082 | 0.092 | 0.094 | 0.89 | 0.004 |
| gpu_instance_materialization | 0.081 | 0.092 | 0.095 | 0.89 | 0.004 |
| main_transmissive_pass_3d | 0.069 | 0.078 | 0.152 | 0.89 | 0.009 |
| clustering | 0.059 | 0.067 | 0.068 | 0.89 | 0.020 |
| early_mesh_preprocessing | 0.053 | 0.060 | 0.073 | 0.89 | 0.004 |
| clouds_trace | 0.049 | 0.056 | 0.079 | 0.89 | 0.001 |
| render_sky | 0.049 | 0.055 | 0.055 | 0.89 | 0.002 |
| clouds_resolve | 0.044 | 0.050 | 0.051 | 0.89 | 0.001 |
| surface_trail | 0.034 | 0.058 | 0.057 | 0.58 | 0.002 |
| lightprobe_irradiance_map | 0.029 | 0.460 | 0.762 | 0.06 | 0.001 |
| atmos_froxel_integrate | 0.028 | 0.032 | 0.033 | 0.89 | 0.001 |
| water_snapshot | 0.023 | 0.026 | 0.034 | 0.89 | 0.002 |
| range_unpacking | 0.022 | 0.025 | 0.029 | 0.89 | 0.003 |
| water_mips | 0.020 | 0.023 | 0.024 | 0.89 | 0.002 |
| per_view_shadow_pass | 0.016 | 0.018 | 0.027 | 0.89 | 0.005 |
| main_transparent_pass_3d | 0.016 | 0.018 | 0.030 | 0.89 | 0.188 |
| night_sky | 0.015 | 0.017 | 0.044 | 0.89 | 0.002 |
| point_shadow_atlas/early_mesh_preprocessing | 0.014 | 0.027 | 0.049 | 0.53 | 0.006 |
| lightprobe_radiance_map | 0.014 | 0.218 | 0.229 | 0.06 | 0.004 |
| atmosphere_environment | 0.010 | 0.011 | 0.012 | 0.89 | 0.003 |
| point_shadow_atlas/uniform_allocation | 0.010 | 0.018 | 0.039 | 0.53 | 0.006 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.89 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.007 | 0.014 | 0.025 | 0.53 | 0.004 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.89 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.53 | 0.002 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.89 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.89 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.06 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.06 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.06 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.690 | 10.027 | 26.93 | 57519.4 |
| render/busy/ms | 8.146 | 9.429 | 24.22 | 53917.0 |
| main/busy/ms | 8.120 | 9.221 | 30.79 | 53744.6 |
| stage/render/render | 5.050 | 6.115 | 22.23 | 33427.3 |
| stage/main/post_update | 4.557 | 5.308 | 15.86 | 30161.9 |
| render/graph/ms | 4.357 | 5.228 | 10.34 | 28838.4 |
| encode//sum | 3.302 | 4.069 | 8.58 | 21855.3 |
| stage/render/graph_submit | 3.093 | 3.908 | 7.66 | 20472.4 |
| encode/submit | 1.959 | 2.863 | 5.73 | 12966.5 |
| stage/main/update | 1.867 | 2.343 | 24.74 | 12360.5 |
| encode/finish_wall | 1.224 | 1.574 | 3.25 | 8103.6 |
| stage/render/camera_driver | 1.201 | 1.411 | 2.80 | 7948.5 |
| encode/pass:early prepass/sum | 0.837 | 1.085 | 1.85 | 5541.2 |
| encode/pass:shadow_directional_light/sum | 0.834 | 1.255 | 1.92 | 5517.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.827 | 1.102 | 1.94 | 5475.9 |
| stage/render/prepare_write_phase_buffers | 0.572 | 0.833 | 1.49 | 3785.9 |
| bigspace/total/ms | 0.561 | 1.313 | 2.07 | 3712.1 |
| stage/render/prepare_assets | 0.529 | 0.658 | 10.29 | 3502.5 |
| stage/main/fixed_main_loop | 0.522 | 0.994 | 2.04 | 3457.0 |
| render/present/ms | 0.453 | 1.229 | 15.97 | 2999.8 |
| stage/render/prepare_meshes | 0.445 | 0.568 | 3.54 | 2946.2 |
| stage/render/prepare_resources | 0.422 | 0.524 | 1.53 | 2796.5 |
| stage/main/pre_update | 0.379 | 0.505 | 3.71 | 2505.3 |
| stage/main/first | 0.355 | 0.453 | 5.68 | 2352.3 |
| controller/feed/ms | 0.333 | 0.433 | 0.81 | 2206.9 |
| stage/render/prepare_bind_groups | 0.302 | 0.483 | 1.38 | 1996.5 |
| stage/render/prepare_flush | 0.299 | 0.517 | 3.59 | 1980.5 |
| stage/render/prepare_batch_phases | 0.279 | 0.433 | 0.88 | 1845.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.270 | 0.437 | 0.91 | 1789.5 |
| stage/main/last | 0.257 | 0.310 | 2.01 | 1704.0 |
| bigspace/low_precision/ms | 0.242 | 0.718 | 1.67 | 1602.6 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.212 | 0.874 | 2.07 | 1400.4 |
| stream/drive/ms | 0.208 | 0.298 | 10.93 | 1379.4 |
| stage/render/create_views | 0.206 | 0.281 | 0.63 | 1364.7 |
| stage/render/extract_commands | 0.188 | 0.225 | 0.58 | 1241.3 |
| anim/evaluate/ms | 0.172 | 0.321 | 1.52 | 1136.7 |
| bigspace/high_precision/ms | 0.168 | 0.694 | 1.76 | 1113.5 |
| anim/evaluate_cpu/ms | 0.158 | 0.198 | 0.48 | 1046.3 |
| stream/tick/ms | 0.154 | 0.229 | 0.53 | 1022.0 |
| stage/render/specialize | 0.151 | 0.243 | 2.01 | 1001.7 |
| pipelines/compile_ms | 0.134 | 0.000 | 134.36 | 883.9 |
| stage/render/prepare_views | 0.133 | 0.176 | 0.52 | 883.0 |
| pipelines/front_ms | 0.130 | 0.000 | 131.13 | 860.3 |
| stage/render/queue | 0.095 | 0.167 | 1.50 | 632.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.093 | 0.148 | 0.79 | 616.3 |
| render/acquire/ms | 0.091 | 0.123 | 0.49 | 602.6 |
| controller/move/ms | 0.083 | 0.282 | 0.63 | 547.7 |
| stage/render/phase_sort | 0.081 | 0.111 | 1.20 | 534.8 |
| controller/write/ms | 0.074 | 0.101 | 0.39 | 492.2 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.074 | 0.280 | 1.32 | 491.0 |
| terrain/update/ms | 0.059 | 0.287 | 1.82 | 390.5 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.073 | 0.35 | 305.0 |
| physics/step/ms | 0.039 | 0.083 | 0.57 | 259.0 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.29 | 211.4 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.028 | 0.000 | 2.81 | 184.8 |
| physics/insert/ms | 0.024 | 0.281 | 0.82 | 160.0 |
| stage/render/cleanup | 0.024 | 0.037 | 0.31 | 155.7 |
| terrain/select/ms | 0.022 | 0.253 | 0.71 | 143.6 |
| weather/map/ms | 0.018 | 0.024 | 0.28 | 117.0 |
| far/select/ms | 0.015 | 0.167 | 0.44 | 100.6 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.16 | 85.0 |
| anim/apply/ms | 0.012 | 0.021 | 0.13 | 82.0 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.27 | 71.7 |
| weather/eval/ms | 0.011 | 0.014 | 0.14 | 70.6 |
| script/update/ms | 0.008 | 0.012 | 0.28 | 52.9 |
| controller/avoid/ms | 0.006 | 0.012 | 0.06 | 40.8 |
| stream/despawn/ms | 0.006 | 0.000 | 1.28 | 37.1 |
| script/input/ms | 0.005 | 0.006 | 0.26 | 33.3 |
| pipelines/driver_ms | 0.004 | 0.000 | 3.56 | 23.5 |
| script/ui/ms | 0.003 | 0.005 | 0.20 | 22.8 |
| stream/spawn/ms | 0.003 | 0.000 | 1.12 | 22.6 |
| stream/upload/ms | 0.003 | 0.000 | 1.05 | 19.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 14.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.24 | 7.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.06 | 4.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.6 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 33 | 19 |
| controller/movers/count | count | 20 | 23 | 19 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1303 | 2262 | 593 |
| crowd/c0/meshes/count | count | 356 | 646 | 156 |
| crowd/c1/count | count | 145 | 156 | 148 |
| crowd/c1/joints/count | count | 5487 | 5991 | 5603 |
| crowd/c1/meshes/count | count | 1166 | 1295 | 1185 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6372 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 161 | 216 | 205 |
| crowd/joints_extracted/count | count | 726 | 7214 | 607 |
| crowd/joints_uploaded/count | count | 3611 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.4 k | 47.7 k |
| encode//max | ms | 0.41 | 1.26 | 0.48 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.16 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.27 | 0.01 |
| encode/finish_critical | ms | 0.98 | 2.81 | 0.99 |
| encode/pass:early prepass/max | ms | 0.84 | 1.85 | 0.68 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.84 | 1.94 | 0.99 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.79 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.26 | 0.91 | 0.41 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.64 | 2.07 | 0.79 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.20 | 1.32 | 0.12 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 0.95 | 2.81 | 1.03 |
| encode/pass:shadow_directional_light/max | ms | 0.82 | 1.92 | 0.67 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.29 | 0.04 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.52 | 30.79 | 7.99 |
| gpu/timed/ms | ms | 7.94 | 9.90 | 7.99 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.5 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 153 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 153 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 770 | 973 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2487 | 3553 | 2455 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 770 | 973 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2487 | 3553 | 2455 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 27 | 173 | 8 |
| phases/shadow/bins | count | 751 | 991 | 671 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2181 | 2578 | 2027 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 31 | 74 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2704 | 3182 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 196 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 329 | 330 | 330 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 14 | 239 | 11 |
| preprocess/bind_groups_created | count | 9 | 233 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 128 | 191 | 123 |
| process/cpu_usage | % | 25.79 | 37.50 | 24.52 |
| process/mem_usage | GiB | 5.08 | 5.39 | 5.39 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2976 | 3166 | 3166 |
| scene/batch/masked/live | count | 2976 | 3166 | 3166 |
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
| scene/batch/opaque/capacity | count | 41.0 k | 41.0 k | 41.0 k |
| scene/batch/opaque/count | count | 2 | 2 | 2 |
| scene/batch/opaque/high_water | count | 18.8 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 18.8 k | 19.9 k | 19.9 k |
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
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1591 | 1621 | 1621 |
| scene/chains/live | count | 1591 | 1621 | 1621 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 6 | 0 |
| shadow_atlas/empty_faces | count | 0 | 1 | 0 |
| shadow_atlas/lamps | count | 8 | 9 | 0 |
| shadow_atlas/live_occ_pct | % | 37.50 | 99.22 | 0 |
| shadow_atlas/live_tiles | count | 16 | 37 | 0 |
| shadow_atlas/mover_draws | count | 8 | 12 | 0 |
| shadow_atlas/movers | count | 123 | 434 | 0 |
| shadow_atlas/moving_lamps | count | 1 | 2 | 0 |
| shadow_atlas/render/blits | count | 8 | 12 | 0 |
| shadow_atlas/render/clears | count | 0 | 16 | 0 |
| shadow_atlas/render/lamps | count | 8 | 9 | 0 |
| shadow_atlas/render/live_direct | count | 8 | 12 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 11 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 14 | 39 | 0 |
| shadow_atlas/resizes | count | 26 | 35 | 35 |
| shadow_atlas/static_occ_pct | % | 14.65 | 31.84 | 0 |
| shadow_atlas/static_tiles | count | 36 | 53 | 0 |
| shadow_atlas/views | count | 14 | 39 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 15 | 131 | 13 |
| skin/blocks_skipped/count | count | 145 | 204 | 192 |
| skin/blocks_walked/count | count | 15 | 29 | 13 |
| skin/joint_bytes_written/bytes | bytes | 50.9 k | 811.6 k | 44.0 k |
| skin/joints_written/count | count | 796 | 12.7 k | 688 |
| skin/upload_writes/count | count | 8 | 25 | 10 |
| started_load_count |  | 6442 | 6567 | 6567 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 237 | 333 | 180 |
| statics/materials_bound | count | 237 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.06 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.77 | 43.37 | 32.28 |
| system/mem_usage | % | 55.26 | 56.16 | 56.15 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 83 | 232 | 66 |
| views/all/bins | count | 2843 | 3096 | 2961 |
| views/all/count | count | 17 | 42 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 7993 | 9960 | 7967 |
| views/all/pipelines | count | 77 | 121 | 66 |
| views/all/pipelines_distinct | count | 63 | 72 | 61 |
| views/all/work_items | count | 154.0 k | 230.4 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 34 | 8 |
| views/cascade/bins | count | 726 | 879 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2127 | 2442 | 2027 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 5 | 154 | 0 |
| views/lamp_face/bins | count | 11 | 222 | 0 |
| views/lamp_face/count | count | 14 | 39 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 66 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 23 | 342 | 0 |
| views/lamp_face/pipelines | count | 5 | 51 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 10 | 24.9 k | 0 |
| views/main/batch_sets | count | 57 | 61 | 58 |
| views/main/bins | count | 2012 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5906 | 7526 | 5940 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 104.6 k | 106.0 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 21.3 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 53.13 | 205 | 351.7 k |
| anim/events/count | count | 0.09 | 3 | 594 |
| assets/Image/added | count | 0.09 | 35 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.43 | 297 | 2874 |
| assets/Mesh/removed | count | 0.73 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.13 | 10 | 7475 |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.22 | 1154 | 1489 |
| encode//count | count | 27.91 | 30 | 184.7 k |
| encode/bind_walk/init_textures | count | 8730 | 23.0 k | 57.79 M |
| encode/bind_walk/init_textures_skipped | count | 51.4 k | 75.7 k | 340.32 M |
| encode/bind_walk/inits | count | 549 | 780 | 3.64 M |
| encode/bind_walk/inits_skipped | count | 145 | 233 | 962.4 k |
| encode/bind_walk/merge_textures | count | 8668 | 22.9 k | 57.37 M |
| encode/bind_walk/merge_textures_skipped | count | 9901 | 26.3 k | 65.53 M |
| encode/bind_walk/merges | count | 102 | 157 | 672.8 k |
| encode/bind_walk/merges_skipped | count | 131 | 227 | 868.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6619 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6619 |
| encode/pass:early prepass/count | count | 1 | 1 | 6619 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6619 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6619 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6619 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.32 | 1 | 2089 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.23 | 1 | 1541 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.03 | 1 | 193 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6619 |
| encode/queue/command_buffers | count | 38.50 | 42 | 254.8 k |
| encode/queue/submits | count | 2.01 | 4 | 13.3 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.32 M | 15.38 G |
| encode/queue/write_buffer/calls | count | 260 | 399 | 1.72 M |
| encode/queue/write_buffer_with/bytes | bytes | 126.8 k | 1.25 M | 839.37 M |
| encode/queue/write_buffer_with/calls | count | 10.29 | 900 | 68.1 k |
| encode/queue/write_texture/bytes | bytes | 1835 | 589.8 k | 12.15 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 556 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6619 |
| physics/inserts | count | 0.34 | 112 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 163 | 2335 |
| physics/steps | count | 0.60 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 10 |
| pipelines/created | count | 0.00 | 1 | 11 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 25 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5114 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1121 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 49, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.53 | 2531.1 | main/busy/ms 2500, stage/main/post_update 2460, stage/main/update 30.14, stage/main/pre_update 6.52, stage/main/last 1.17, surface/advance_cpu_ms 0.72, stage/main/first 0.59, stream/drive/ms 0.50, surface/cell_upload_ms 0.45, stage/main/fixed_main_loop 0.37, stream/tick/ms 0.17, bigspace/total/ms 0.10 |
| warm-up | 3.06 | 531.4 | main/busy/ms 2646, stage/main/post_update 2492, render/thread/ms 494, render/busy/ms 409, stage/render/prepare_assets 368, stage/main/update 137, stage/render/prepare_views 82.76, render/acquire/ms 82.57, stage/render/render 16.18, stage/render/create_views 15.95, render/graph/ms 13.04, stage/render/camera_driver 11.27 |
| warm-up | 3.13 | 63.5 | render/thread/ms 540, render/busy/ms 454, stage/render/prepare_assets 399, main/busy/ms 171, stage/main/update 115, stage/render/prepare_views 83.19, render/acquire/ms 82.87, stage/main/post_update 44.09, terrain/update/ms 24.47, stage/render/render 22.35, render/graph/ms 17.45, stage/render/create_views 16.28 |
| warm-up | 3.18 | 53.0 | pipelines/wait_ms 286, pipelines/compile_ms 221, pipelines/front_ms 197, render/thread/ms 97.23, render/busy/ms 93.74, stage/render/render 51.82, render/graph/ms 47.57, main/busy/ms 44.24, stage/render/prepare_assets 31.43, terrain/update/ms 29.87, stage/main/post_update 24.19, pipelines/driver_ms 23.52 |
| warm-up | 3.33 | 149.0 | pipelines/wait_ms 5591, pipelines/compile_ms 1502, pipelines/front_ms 1419, render/thread/ms 168, render/busy/ms 162, main/busy/ms 93.60, stage/render/render 85.24, pipelines/driver_ms 83.04, render/graph/ms 67.83, stage/main/post_update 56.13, stage/render/graph_submit 32.51, stage/render/prepare_bind_groups 27.74 |
| warm-up | 3.39 | 66.7 | pipelines/wait_ms 6822, pipelines/compile_ms 2415, pipelines/front_ms 2304, render/thread/ms 178, render/busy/ms 173, main/busy/ms 147, pipelines/driver_ms 111, stage/main/update 56.49, stage/main/post_update 54.99, stage/render/render 45.49, stage/render/prepare_assets 40.81, stage/render/prepare_resources 37.78 |
| warm-up | 3.47 | 76.8 | pipelines/compile_ms 1709, pipelines/front_ms 1620, pipelines/wait_ms 1582, render/thread/ms 123, render/busy/ms 121, main/busy/ms 92.66, pipelines/driver_ms 88.35, stage/render/prepare_assets 68.67, stage/main/update 60.49, stage/main/post_update 20.50, stage/render/prepare_resources 18.45, terrain/update/ms 14.54 |
| warm-up | 3.50 | 26.6 | pipelines/compile_ms 997, pipelines/front_ms 945, render/thread/ms 86.52, render/busy/ms 84.54, pipelines/wait_ms 67.20, stage/render/prepare_assets 63.57, pipelines/driver_ms 52.65, main/busy/ms 43.34, stage/main/post_update 23.19, encode//sum 16.78, stage/render/render 14.35, stage/main/update 13.07 |
| warm-up | 3.53 | 34.4 | pipelines/compile_ms 959, pipelines/front_ms 926, render/thread/ms 57.28, render/busy/ms 55.60, main/busy/ms 38.61, stage/render/prepare_assets 33.89, pipelines/driver_ms 32.53, stage/main/post_update 20.37, pipelines/wait_ms 17.32, encode//sum 16.95, stage/render/render 14.50, render/graph/ms 12.48 |
| warm-up | 3.68 | 105.9 | pipelines/compile_ms 1230, pipelines/front_ms 1172, pipelines/wait_ms 118, main/busy/ms 76.06, stage/main/update 58.93, pipelines/driver_ms 57.91, render/thread/ms 54.32, render/busy/ms 51.64, stage/render/prepare_assets 28.53, stage/render/render 15.11, render/graph/ms 12.18, stage/main/post_update 11.56 |
| warm-up | 3.72 | 34.0 | pipelines/compile_ms 1585, pipelines/front_ms 1516, pipelines/wait_ms 119, pipelines/driver_ms 68.22, render/thread/ms 63.00, render/busy/ms 60.43, stage/render/prepare_assets 35.89, main/busy/ms 28.17, stage/render/render 17.77, render/graph/ms 14.64, stage/main/update 13.02, stage/render/graph_submit 10.69 |
| warm-up | 3.75 | 33.3 | pipelines/compile_ms 775, pipelines/front_ms 749, render/thread/ms 54.40, render/busy/ms 52.19, main/busy/ms 29.78, stage/render/prepare_assets 28.42, pipelines/driver_ms 25.94, stage/render/render 17.72, render/graph/ms 14.80, pipelines/wait_ms 14.55, pipelines/lock_ms 14.18, stage/render/graph_submit 10.96 |
| warm-up | 3.79 | 42.8 | pipelines/compile_ms 421, pipelines/front_ms 401, render/thread/ms 53.19, render/busy/ms 51.78, stage/render/prepare_assets 35.37, main/busy/ms 23.57, pipelines/driver_ms 20.56, pipelines/wait_ms 13.77, pipelines/lock_ms 13.37, stage/render/render 11.24, render/graph/ms 9.35, stage/main/update 7.91 |
| warm-up | 3.88 | 58.8 | pipelines/compile_ms 256, pipelines/front_ms 247, main/busy/ms 57.51, render/thread/ms 55.91, render/busy/ms 54.16, stage/main/update 29.45, stage/render/prepare_assets 24.64, stage/main/post_update 22.11, stream/drive/ms 19.10, stage/render/render 15.87, render/graph/ms 13.31, stage/render/graph_submit 9.90 |
| warm-up | 3.92 | 42.9 | pipelines/compile_ms 461, pipelines/front_ms 447, render/thread/ms 72.44, render/busy/ms 70.18, main/busy/ms 53.35, stage/render/prepare_assets 35.91, stage/main/post_update 26.09, stage/main/update 17.91, stage/render/render 14.79, pipelines/driver_ms 14.76, render/graph/ms 11.67, encode//sum 9.01 |
| warm-up | 4.01 | 86.7 | pipelines/compile_ms 1770, pipelines/front_ms 1716, render/thread/ms 74.95, render/busy/ms 72.65, main/busy/ms 65.83, pipelines/driver_ms 53.92, stage/render/prepare_assets 43.01, stage/main/update 41.98, stage/main/post_update 14.36, stage/render/render 14.00, render/graph/ms 11.17, pipelines/wait_ms 8.48 |
| warm-up | 4.14 | 130.2 | pipelines/compile_ms 2167, pipelines/front_ms 2100, main/busy/ms 173, stage/main/update 150, render/thread/ms 125, render/busy/ms 123, stage/render/prepare_assets 71.71, pipelines/driver_ms 67.47, stage/render/prepare_bind_groups 29.01, stage/render/cleanup 13.71, stage/render/render 12.70, stage/main/post_update 12.34 |
| warm-up | 4.23 | 92.7 | pipelines/compile_ms 1416, pipelines/front_ms 1359, main/busy/ms 149, stage/main/update 128, render/thread/ms 119, render/busy/ms 117, stage/render/prepare_assets 67.62, pipelines/driver_ms 57.17, stage/render/prepare_bind_groups 28.54, stage/render/cleanup 13.77, stage/render/render 12.71, render/graph/ms 10.43 |
| warm-up | 4.33 | 99.3 | pipelines/compile_ms 2019, pipelines/front_ms 1922, pipelines/wait_ms 207, render/thread/ms 135, render/busy/ms 132, main/busy/ms 98.76, pipelines/driver_ms 97.37, stage/main/update 74.30, stage/render/prepare_write_phase_buffers 57.88, stage/render/prepare_assets 25.58, stage/render/render 18.03, render/graph/ms 15.99 |
| warm-up | 4.39 | 61.5 | pipelines/compile_ms 1549, pipelines/front_ms 1478, pipelines/wait_ms 204, render/thread/ms 139, render/busy/ms 136, main/busy/ms 103, stage/main/update 77.35, pipelines/driver_ms 70.40, stage/render/prepare_write_phase_buffers 57.92, stage/render/render 30.76, render/graph/ms 28.46, stage/render/graph_submit 21.71 |
| warm-up | 4.45 | 57.6 | pipelines/compile_ms 906, pipelines/front_ms 874, render/thread/ms 90.72, render/busy/ms 88.99, main/busy/ms 57.16, pipelines/driver_ms 32.42, stage/render/prepare_assets 32.17, stage/render/render 31.33, render/graph/ms 29.20, stage/render/graph_submit 22.64, stage/main/update 22.21, stage/main/pre_update 16.84 |
| warm-up | 4.50 | 53.3 | pipelines/compile_ms 1335, pipelines/front_ms 1287, pipelines/wait_ms 105, render/thread/ms 99.90, render/busy/ms 98.40, main/busy/ms 75.21, pipelines/driver_ms 47.95, stage/render/render 39.35, render/graph/ms 31.75, stage/render/graph_submit 25.91, stage/main/update 25.35, stage/render/prepare_assets 19.05 |
| warm-up | 4.58 | 74.2 | pipelines/compile_ms 1362, pipelines/front_ms 1322, pipelines/wait_ms 240, render/thread/ms 122, render/busy/ms 118, main/busy/ms 101, stage/main/update 58.53, pipelines/driver_ms 40.96, stage/render/render 36.98, render/graph/ms 28.69, stage/render/prepare_resources 27.70, stage/main/post_update 23.30 |
| warm-up | 4.65 | 72.3 | pipelines/compile_ms 2086, pipelines/front_ms 2011, pipelines/wait_ms 198, render/thread/ms 116, render/busy/ms 113, main/busy/ms 89.29, pipelines/driver_ms 74.95, stage/main/update 62.61, stage/render/prepare_assets 37.56, stage/render/prepare_resources 26.86, stage/render/render 16.07, stage/main/post_update 14.55 |
| warm-up | 4.71 | 54.5 | pipelines/compile_ms 1951, pipelines/front_ms 1881, main/busy/ms 105, render/thread/ms 82.39, render/busy/ms 80.93, stage/main/update 79.12, pipelines/driver_ms 69.38, pipelines/wait_ms 53.81, stream/drive/ms 51.22, stage/render/prepare_assets 49.39, stage/render/render 14.24, stage/main/post_update 13.78 |
| warm-up | 4.79 | 81.6 | pipelines/compile_ms 770, pipelines/front_ms 750, main/busy/ms 104, stage/main/update 77.98, render/thread/ms 70.08, render/busy/ms 68.76, stream/drive/ms 57.07, stage/render/prepare_assets 34.27, pipelines/driver_ms 20.06, stage/main/post_update 15.71, stage/render/render 13.10, render/graph/ms 11.13 |
| warm-up | 4.82 | 36.3 | pipelines/compile_ms 792, pipelines/front_ms 764, render/thread/ms 62.27, render/busy/ms 60.87, main/busy/ms 56.22, stage/main/update 36.48, pipelines/driver_ms 27.89, stage/render/prepare_assets 20.66, stage/render/render 12.73, stage/main/post_update 10.97, render/graph/ms 10.81, stream/drive/ms 10.08 |
| warm-up | 4.86 | 33.4 | pipelines/compile_ms 573, pipelines/front_ms 550, render/thread/ms 56.16, render/busy/ms 54.68, main/busy/ms 44.66, stage/main/update 26.43, pipelines/driver_ms 22.78, stage/render/prepare_assets 22.10, stage/render/render 12.96, render/graph/ms 11.08, stage/main/post_update 8.86, encode//sum 7.80 |
| warm-up | 5.04 | 33.6 | pipelines/compile_ms 360, pipelines/front_ms 346, render/thread/ms 50.13, render/busy/ms 47.79, main/busy/ms 34.01, stage/render/render 16.66, pipelines/driver_ms 14.83, stage/render/prepare_assets 14.72, stage/main/update 14.59, render/graph/ms 13.80, stage/main/post_update 10.90, stage/render/graph_submit 10.89 |
| warm-up | 5.09 | 48.7 | pipelines/compile_ms 251, pipelines/front_ms 239, render/thread/ms 72.70, render/busy/ms 71.06, main/busy/ms 54.92, stage/main/update 28.29, stage/render/prepare_assets 20.15, stage/render/render 15.64, render/graph/ms 13.37, stream/drive/ms 12.88, stage/render/prepare_resources 12.29, pipelines/driver_ms 11.58 |
| warm-up | 5.12 | 26.7 | pipelines/compile_ms 879, pipelines/front_ms 849, render/thread/ms 62.16, render/busy/ms 60.57, main/busy/ms 52.69, pipelines/wait_ms 31.12, pipelines/driver_ms 29.28, stage/main/update 25.93, stage/render/prepare_assets 15.27, stream/drive/ms 12.82, stage/render/prepare_resources 12.31, stage/render/render 10.55 |
| warm-up | 5.35 | 30.0 | main/busy/ms 33.82, render/thread/ms 23.53, render/busy/ms 22.45, stage/main/update 19.45, stream/drive/ms 12.63, stage/main/post_update 10.69, stage/render/render 8.65, render/graph/ms 7.12, encode//sum 6.56, stage/render/graph_submit 4.75, stage/render/prepare_assets 3.67, encode/submit 2.67 |
| warm-up | 5.60 | 27.2 | main/busy/ms 31.88, render/thread/ms 22.72, render/busy/ms 21.76, stage/main/update 21.27, stream/drive/ms 13.89, stage/render/render 10.08, render/graph/ms 8.57, encode//sum 7.63, stage/main/post_update 7.02, stage/render/graph_submit 5.36, stage/render/prepare_assets 4.87, stage/render/camera_driver 3.02 |
| warm-up | 5.73 | 28.2 | render/thread/ms 40.63, render/busy/ms 38.95, main/busy/ms 29.96, stage/render/render 14.26, stage/main/update 13.81, stage/render/prepare_assets 13.33, pipelines/compile_ms 13.29, pipelines/front_ms 12.33, render/graph/ms 12.00, stage/main/post_update 11.98, encode//sum 8.95, stage/render/graph_submit 7.56 |
| warm-up | 5.78 | 33.0 | render/thread/ms 41.10, render/busy/ms 39.72, main/busy/ms 28.64, stage/render/prepare_assets 21.34, pipelines/compile_ms 18.59, pipelines/front_ms 17.38, stage/main/update 11.60, stage/main/post_update 10.21, stage/render/render 10.04, render/graph/ms 8.05, encode//sum 7.45, stage/render/graph_submit 5.21 |
| warm-up | 5.86 | 27.9 | main/busy/ms 38.85, render/thread/ms 37.50, render/busy/ms 36.36, stage/main/update 24.55, stage/render/render 12.48, render/graph/ms 10.80, stage/main/post_update 9.16, stage/render/prepare_assets 8.49, encode//sum 8.19, stage/render/graph_submit 7.19, stream/drive/ms 6.34, stage/render/prepare_meshes 5.60 |
| warm-up | 5.99 | 29.5 | main/busy/ms 41.30, render/thread/ms 39.10, render/busy/ms 37.68, stage/main/update 21.52, stage/render/render 17.51, render/graph/ms 15.54, stage/main/post_update 14.34, pipelines/compile_ms 11.81, pipelines/front_ms 10.78, stage/render/graph_submit 10.69, encode//sum 9.14, encode/submit 7.33 |
| warm-up | 6.02 | 31.2 | render/thread/ms 49.09, render/busy/ms 47.58, main/busy/ms 46.79, stage/main/update 26.57, stage/render/render 16.35, render/graph/ms 14.26, stage/main/post_update 13.80, stage/render/prepare_assets 11.02, stage/render/graph_submit 9.56, encode//sum 9.18, encode/submit 6.47, stage/render/prepare_meshes 5.78 |
| warm-up | 6.06 | 38.6 | render/thread/ms 50.53, render/busy/ms 49.50, main/busy/ms 40.17, stage/render/prepare_assets 22.98, stage/main/update 22.49, stage/main/post_update 9.76, stage/render/render 9.11, render/graph/ms 7.54, encode//sum 6.81, stage/render/prepare_meshes 6.15, stage/render/graph_submit 4.76, stream/drive/ms 3.73 |
| warm-up | 6.09 | 31.9 | render/thread/ms 57.32, render/busy/ms 55.86, main/busy/ms 38.61, stage/render/prepare_assets 18.50, stage/render/render 18.35, stage/main/update 17.05, render/graph/ms 16.30, stage/main/post_update 14.49, stage/render/graph_submit 11.76, encode//sum 9.62, encode/submit 8.49, stage/render/prepare_meshes 5.48 |
| warm-up | 6.15 | 36.1 | pipelines/compile_ms 150, pipelines/front_ms 146, render/thread/ms 37.75, render/busy/ms 36.77, main/busy/ms 32.11, stage/main/update 18.92, stage/render/prepare_assets 16.09, stage/render/render 9.88, render/graph/ms 8.09, stage/main/post_update 7.97, encode//sum 7.36, stream/drive/ms 4.65 |
| warm-up | 6.29 | 43.7 | main/busy/ms 59.16, stage/main/update 42.83, render/thread/ms 35.46, render/busy/ms 34.26, stream/drive/ms 16.69, stage/render/render 11.80, stage/main/post_update 10.64, render/graph/ms 9.96, stage/render/prepare_assets 8.59, encode//sum 7.19, stage/render/graph_submit 7.02, stage/render/prepare_meshes 5.66 |
| warm-up | 6.34 | 44.7 | pipelines/compile_ms 139, pipelines/front_ms 134, main/busy/ms 48.71, render/thread/ms 48.51, render/busy/ms 47.34, stage/main/update 31.71, stage/render/prepare_assets 20.57, stage/render/render 11.89, render/graph/ms 10.01, stage/main/post_update 9.94, encode//sum 7.15, stream/drive/ms 7.07 |
| warm-up | 6.40 | 25.2 | main/busy/ms 37.78, render/thread/ms 25.26, render/busy/ms 24.20, stage/main/update 19.29, stage/render/render 12.07, stage/main/post_update 11.36, render/graph/ms 10.48, encode//sum 7.14, stage/render/graph_submit 7.05, stream/drive/ms 5.01, encode/submit 4.40, stage/render/camera_driver 3.29 |
| warm-up | 6.45 | 27.4 | render/thread/ms 35.45, main/busy/ms 34.47, render/busy/ms 34.20, stage/main/update 21.12, stage/render/render 13.31, render/graph/ms 11.67, stage/main/post_update 8.54, encode//sum 8.26, stage/render/graph_submit 8.03, stage/render/prepare_assets 6.23, stage/render/prepare_meshes 5.86, stream/drive/ms 5.46 |
| measure | 7.45 | 28.3 | render/thread/ms 36.95, stage/render/render 28.01, main/busy/ms 20.47, render/busy/ms 20.33, render/present/ms 16.45, render/graph/ms 10.80, stage/main/post_update 10.34, stage/render/graph_submit 7.83, encode//sum 7.46, stage/main/update 6.25, encode/submit 5.09, stage/render/camera_driver 2.83 |
| measure | 18.39 | 32.9 | main/busy/ms 40.38, stage/main/update 26.46, render/thread/ms 20.21, render/busy/ms 19.28, stage/render/render 10.21, render/graph/ms 8.98, stage/main/post_update 8.69, stage/render/graph_submit 6.71, encode//sum 6.21, encode/submit 4.47, stage/render/prepare_assets 3.71, encode/finish_wall 2.28 |
| measure | 59.09 | 30.1 | render/thread/ms 42.15, main/busy/ms 41.11, render/busy/ms 40.70, stage/main/post_update 25.41, stage/render/render 19.70, render/graph/ms 17.51, encode//sum 12.64, stage/render/graph_submit 11.92, stage/main/update 8.49, encode/submit 7.87, stage/render/prepare_assets 7.39, stage/render/camera_driver 5.39 |
| measure | 59.12 | 28.6 | render/thread/ms 44.19, render/busy/ms 43.11, main/busy/ms 33.49, stage/main/post_update 20.47, stage/render/prepare_assets 16.19, stage/render/render 13.80, render/graph/ms 12.01, encode//sum 9.72, stage/render/graph_submit 7.81, stage/main/update 5.56, encode/submit 4.29, stage/render/camera_driver 4.02 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=8 --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\balmora_walk_night@2560x1440_arm=atlas8-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 18%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1536 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **87%** (min 53%) · 2625 MHz (min 2625) · memory 8101 MHz · 83.5 W · 76 °C (max 80) · VRAM 6476 MiB · throttled 100% of samples

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
- `sf.graphics_preset=high (preset: level requested)`
- `sg.effects=high (preset: level requested)`
- `sg.lighting=high (preset: level requested)`
- `sg.post=high (preset: level requested)`
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
- `shadow.lamp.atlas.enabled=on (command line: --set)`
- `shadow.lamp.atlas.max_lamps=8 (command line: --set)`
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
