# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5809 frames over 65.6 s · **median 10.88 ms (92 fps)** · mean 11.30 · p95 14.53 · p99 16.33 (1% low 61 fps) · worst 30.48 ms · 59 spikes over 1.5× median · 21.9 stutters/min (work 21.0/min)
- warm-up: 323 frames over 9.4 s, worst 2480.9 ms
- hitches (> 27.2 ms): **3** measured, 45 in warm-up
- timed GPU passes: 6.80 ms of a 11.30 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.88 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.94/min, limit 1/min
- pass worst frame 30.48 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.026 | 2.030 | 2.063 | 1.00 | 0.175 |
| main_opaque_pass_3d | 1.300 | 1.302 | 1.656 | 1.00 | 0.072 |
| early prepass | 0.491 | 0.492 | 0.701 | 1.00 | 0.085 |
| bloom | 0.315 | 0.316 | 0.322 | 1.00 | 0.063 |
| auxiliary_views | 0.314 | 0.314 | 1.094 | 1.00 | 0.305 |
| atmosphere_luts | 0.308 | 0.309 | 0.372 | 1.00 | 0.003 |
| water_velocity | 0.262 | 0.262 | 0.349 | 1.00 | 0.002 |
| water_sspr | 0.201 | 0.201 | 0.203 | 1.00 | 0.002 |
| ssao | 0.188 | 0.188 | 0.193 | 1.00 | 0.007 |
| upscaling | 0.129 | 0.129 | 0.137 | 1.00 | 0.004 |
| water_fft | 0.128 | 0.129 | 0.137 | 1.00 | 0.003 |
| atmos_froxel_inject | 0.127 | 0.127 | 0.128 | 1.00 | 0.001 |
| uniform_allocation | 0.126 | 0.126 | 0.153 | 1.00 | 0.027 |
| clouds_composite | 0.104 | 0.105 | 0.108 | 1.00 | 0.002 |
| bin_unpacking | 0.096 | 0.096 | 0.118 | 1.00 | 0.019 |
| tonemapping | 0.086 | 0.086 | 0.091 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.078 | 0.078 | 0.153 | 1.00 | 0.009 |
| clustering | 0.063 | 0.063 | 0.067 | 1.00 | 0.019 |
| clouds_trace | 0.056 | 0.056 | 0.075 | 1.00 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.413 | 0.416 | 0.12 | 0.001 |
| render_sky | 0.051 | 0.052 | 0.052 | 1.00 | 0.002 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.041 | 0.056 | 0.057 | 0.73 | 0.002 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.028 | 0.028 | 0.036 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.218 | 0.230 | 0.12 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.029 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.038 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.013 | 0.013 | 0.025 | 1.00 | 0.125 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 1.00 | 0.005 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.012 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.975 | 13.126 | 25.24 | 57945.3 |
| render/busy/ms | 9.468 | 12.573 | 24.64 | 54998.0 |
| main/busy/ms | 9.233 | 11.119 | 28.47 | 53635.7 |
| stage/render/render | 6.032 | 8.494 | 16.65 | 35042.4 |
| stage/main/post_update | 5.772 | 7.344 | 9.73 | 33530.1 |
| render/graph/ms | 5.337 | 7.690 | 15.77 | 31001.3 |
| encode//sum | 4.623 | 6.775 | 29.95 | 26853.7 |
| stage/render/graph_submit | 3.973 | 5.600 | 11.60 | 23077.1 |
| encode/submit | 2.553 | 3.786 | 6.43 | 14829.0 |
| stage/main/update | 2.188 | 2.617 | 21.96 | 12709.0 |
| encode/pass:shadow_point_light/sum | 1.666 | 5.117 | 17.71 | 9679.1 |
| encode/finish_wall | 1.450 | 1.954 | 5.35 | 8423.2 |
| stage/render/camera_driver | 1.310 | 2.063 | 4.10 | 7611.2 |
| stage/render/prepare_write_phase_buffers | 0.802 | 1.136 | 2.28 | 4659.2 |
| encode/pass:shadow_directional_light/sum | 0.764 | 1.275 | 2.39 | 4440.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.744 | 1.095 | 2.26 | 4319.4 |
| encode/pass:early prepass/sum | 0.730 | 1.100 | 3.21 | 4241.2 |
| stage/render/prepare_assets | 0.601 | 0.756 | 10.10 | 3493.1 |
| bigspace/total/ms | 0.545 | 1.339 | 2.37 | 3167.1 |
| stage/render/prepare_resources | 0.472 | 0.563 | 1.14 | 2739.9 |
| stage/render/prepare_meshes | 0.448 | 0.567 | 1.66 | 2604.7 |
| render/present/ms | 0.413 | 0.501 | 1.78 | 2398.3 |
| stage/main/first | 0.394 | 0.516 | 5.69 | 2288.5 |
| stage/main/pre_update | 0.361 | 0.499 | 2.44 | 2096.3 |
| terrain/update/ms | 0.317 | 0.412 | 2.00 | 1841.8 |
| controller/feed/ms | 0.285 | 0.356 | 0.95 | 1656.0 |
| terrain/select/ms | 0.283 | 0.369 | 0.92 | 1641.7 |
| stage/render/specialize | 0.282 | 0.418 | 2.33 | 1638.1 |
| stage/render/prepare_bind_groups | 0.275 | 0.481 | 2.50 | 1595.4 |
| stage/render/queue | 0.253 | 0.392 | 2.27 | 1470.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.250 | 0.367 | 0.99 | 1452.7 |
| stream/drive/ms | 0.247 | 0.324 | 19.99 | 1433.0 |
| bigspace/low_precision/ms | 0.228 | 0.682 | 1.76 | 1325.7 |
| stage/main/fixed_main_loop | 0.226 | 0.373 | 0.93 | 1312.9 |
| far/select/ms | 0.203 | 0.270 | 0.56 | 1176.8 |
| stage/render/create_views | 0.197 | 0.255 | 0.98 | 1141.8 |
| stream/tick/ms | 0.190 | 0.256 | 0.62 | 1102.4 |
| stage/render/prepare_batch_phases | 0.189 | 0.268 | 0.86 | 1098.7 |
| stage/render/extract_commands | 0.177 | 0.205 | 0.46 | 1030.1 |
| anim/evaluate/ms | 0.170 | 0.275 | 2.01 | 987.3 |
| stage/render/prepare_flush | 0.166 | 0.326 | 1.08 | 962.9 |
| anim/evaluate_cpu/ms | 0.163 | 0.203 | 0.71 | 948.0 |
| controller/move/ms | 0.154 | 0.358 | 0.98 | 895.2 |
| bigspace/high_precision/ms | 0.150 | 0.629 | 1.66 | 869.4 |
| stage/render/prepare_views | 0.138 | 0.171 | 0.65 | 799.8 |
| stage/main/last | 0.117 | 0.158 | 2.86 | 677.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.108 | 0.177 | 0.70 | 625.8 |
| render/acquire/ms | 0.095 | 0.120 | 0.61 | 549.0 |
| stage/render/prepare_collect_phase_buffers | 0.063 | 0.134 | 0.54 | 367.7 |
| physics/step/ms | 0.050 | 0.088 | 0.56 | 290.4 |
| physics/insert/ms | 0.037 | 0.389 | 0.92 | 215.0 |
| stage/render/cleanup | 0.033 | 0.106 | 0.91 | 191.2 |
| stage/render/phase_sort | 0.032 | 0.055 | 0.49 | 187.9 |
| encode/resolve_diagnostics/sum | 0.032 | 0.043 | 0.58 | 185.2 |
| pipelines/compile_ms | 0.017 | 0.000 | 86.11 | 97.9 |
| pipelines/front_ms | 0.016 | 0.000 | 83.12 | 94.7 |
| weather/map/ms | 0.016 | 0.021 | 0.06 | 90.6 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.12 | 86.9 |
| anim/apply/ms | 0.011 | 0.022 | 0.08 | 66.7 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.63 | 66.4 |
| controller/avoid/ms | 0.011 | 0.015 | 0.26 | 64.3 |
| weather/eval/ms | 0.010 | 0.013 | 0.04 | 56.1 |
| script/update/ms | 0.007 | 0.009 | 0.25 | 42.5 |
| script/input/ms | 0.005 | 0.006 | 0.22 | 31.4 |
| stream/despawn/ms | 0.005 | 0.000 | 1.22 | 30.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 28.4 |
| script/ui/ms | 0.004 | 0.006 | 0.19 | 25.2 |
| controller/write/ms | 0.003 | 0.005 | 0.06 | 20.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.64 | 15.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.7 |
| stage/render/prepare | 0.001 | 0.002 | 0.21 | 3.2 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.99 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 28 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 11 |
| crowd/c0/joints/count | count | 1257 | 2262 | 508 |
| crowd/c0/meshes/count | count | 349 | 646 | 117 |
| crowd/c1/count | count | 148 | 157 | 150 |
| crowd/c1/joints/count | count | 5581 | 6037 | 5688 |
| crowd/c1/meshes/count | count | 1181 | 1311 | 1224 |
| crowd/c2/count | count | 399 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 164 | 216 | 205 |
| crowd/joints_extracted/count | count | 3722 | 8334 | 5775 |
| crowd/joints_uploaded/count | count | 3676 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.42 | 3.33 | 0.49 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.12 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.63 | 0.02 |
| encode/finish_critical | ms | 0.96 | 3.33 | 0.86 |
| encode/pass:early prepass/max | ms | 0.76 | 3.21 | 0.86 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 2.26 | 0.71 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.70 | 0.12 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.24 | 0.99 | 0.23 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 2.39 | 0.86 |
| encode/pass:shadow_point_light/max | ms | 0.81 | 1.55 | 0.67 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.58 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.52 | 28.47 | 8.28 |
| gpu/timed/ms | ms | 7.48 | 10.77 | 7.72 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 793 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 793 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1298 | 1398 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3343 | 5136 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1298 | 1398 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3343 | 5136 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 22 | 201 | 4 |
| phases/shadow/bins | count | 3737 | 4459 | 1494 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6239 | 7540 | 3038 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 202 | 241 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 192 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 323 | 324 | 324 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 563 | 6 |
| preprocess/bind_groups_created | count | 13 | 563 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 134 | 292 | 127 |
| process/cpu_usage | % | 25.96 | 43.75 | 27.40 |
| process/mem_usage | GiB | 5.40 | 5.58 | 5.57 |
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
| skin/blocks_changed/count | count | 14 | 110 | 9 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 164 | 216 | 205 |
| skin/joint_bytes_written/bytes | bytes | 281.6 k | 807.2 k | 445.7 k |
| skin/joints_written/count | count | 4400 | 12.6 k | 6964 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4919 | 5009 | 5009 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 431 | 528 | 396 |
| statics/materials_bound | count | 431 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.48 | 39.58 | 34.33 |
| system/mem_usage | % | 54.53 | 55.11 | 55.06 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 72 | 250 | 56 |
| views/all/bins | count | 6258 | 7377 | 5002 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 20.4 k | 10.7 k |
| views/all/pipelines | count | 246 | 286 | 67 |
| views/all/pipelines_distinct | count | 70 | 75 | 63 |
| views/all/work_items | count | 10.5 k | 17.7 k | 8146 |
| views/cascade/batch_sets | count | 8 | 27 | 4 |
| views/cascade/bins | count | 1447 | 1522 | 1494 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3384 | 4446 | 3038 |
| views/cascade/pipelines | count | 23 | 27 | 19 |
| views/cascade/work_items | count | 337 | 4443 | 222 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2460 | 3036 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2801 | 3378 | 0 |
| views/lamp_face/pipelines | count | 178 | 215 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2862 | 0 |
| views/main/batch_sets | count | 50 | 53 | 52 |
| views/main/bins | count | 2900 | 3590 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8276 | 13.0 k | 7616 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8439 | 13.3 k | 7924 |
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
| anim/bones_written/count | count | 59.19 | 273 | 343.8 k |
| anim/events/count | count | 0.10 | 3 | 568 |
| assets/Image/added | count | 0.10 | 33 | 592 |
| assets/Image/modified | count | 0.00 | 7 | 22 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.70 | 152 | 4061 |
| assets/Mesh/removed | count | 0.83 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.73 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.80 | 1296 | 4675 |
| encode//count | count | 40.17 | 219 | 233.3 k |
| encode/bind_walk/init_textures | count | 16.3 k | 112.9 k | 94.62 M |
| encode/bind_walk/init_textures_skipped | count | 67.2 k | 154.4 k | 390.36 M |
| encode/bind_walk/inits | count | 481 | 1129 | 2.79 M |
| encode/bind_walk/inits_skipped | count | 110 | 175 | 637.6 k |
| encode/bind_walk/merge_textures | count | 16.2 k | 112.9 k | 94.25 M |
| encode/bind_walk/merge_textures_skipped | count | 5394 | 6012 | 31.33 M |
| encode/bind_walk/merges | count | 108 | 342 | 629.6 k |
| encode/bind_walk/merges_skipped | count | 84.81 | 128 | 492.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5809 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5809 |
| encode/pass:early prepass/count | count | 1 | 1 | 5809 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5809 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5809 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5809 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5809 |
| encode/pass:shadow_point_light/count | count | 3.30 | 48 | 19.1 k |
| encode/queue/command_buffers | count | 53.48 | 277 | 310.7 k |
| encode/queue/submits | count | 2.02 | 4 | 11.7 k |
| encode/queue/write_buffer/bytes | bytes | 1.83 M | 2.76 M | 10.61 G |
| encode/queue/write_buffer/calls | count | 274 | 841 | 1.59 M |
| encode/queue/write_buffer_with/bytes | bytes | 170.0 k | 3.53 M | 987.74 M |
| encode/queue/write_buffer_with/calls | count | 11.08 | 465 | 64.4 k |
| encode/queue/write_texture/bytes | bytes | 11.0 k | 8.39 M | 64.12 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5809 |
| physics/inserts | count | 0.39 | 109 | 2275 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 2 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 5828 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1278 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.2 ms; slowest 48, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.48 | 2480.9 | main/busy/ms 2448, stage/main/post_update 2409, stage/main/update 29.62, stage/main/pre_update 4.75, stage/main/last 1.35, surface/advance_cpu_ms 0.72, stage/main/first 0.59, stream/drive/ms 0.53, stage/main/fixed_main_loop 0.38, surface/cell_upload_ms 0.32, stream/tick/ms 0.13, bigspace/total/ms 0.12 |
| warm-up | 3.22 | 735.9 | main/busy/ms 2597, stage/main/post_update 2435, render/thread/ms 703, render/busy/ms 540, stage/render/prepare_assets 493, stage/render/prepare_views 160, render/acquire/ms 160, stage/main/update 147, stage/render/create_views 28.82, stage/render/render 11.65, render/graph/ms 7.31, stage/main/pre_update 6.49 |
| warm-up | 3.33 | 109.5 | render/thread/ms 794, render/busy/ms 580, stage/render/prepare_assets 518, main/busy/ms 170, stage/render/prepare_views 161, render/acquire/ms 161, stage/main/update 122, pipelines/compile_ms 120, pipelines/front_ms 78.68, stage/render/render 68.06, render/present/ms 54.23, pipelines/driver_ms 41.59 |
| warm-up | 3.36 | 35.6 | pipelines/compile_ms 328, pipelines/front_ms 250, render/thread/ms 108, pipelines/driver_ms 78.76, stage/render/render 67.76, main/busy/ms 60.06, render/present/ms 54.76, render/busy/ms 53.06, stage/main/post_update 43.58, terrain/update/ms 26.61, stage/render/prepare_assets 26.31, render/graph/ms 11.58 |
| warm-up | 3.49 | 127.4 | pipelines/wait_ms 4515, pipelines/compile_ms 1449, pipelines/front_ms 1346, render/thread/ms 136, render/busy/ms 131, pipelines/driver_ms 103, main/busy/ms 95.69, stage/main/post_update 47.62, stage/render/prepare_resources 45.56, stage/main/update 39.19, stage/render/render 22.44, stage/render/prepare_flush 20.26 |
| warm-up | 3.54 | 51.5 | pipelines/wait_ms 4806, pipelines/compile_ms 1509, pipelines/front_ms 1433, render/thread/ms 157, render/busy/ms 155, main/busy/ms 141, stage/main/update 90.32, pipelines/driver_ms 76.30, stage/render/prepare_resources 57.22, stage/main/post_update 26.32, stage/render/prepare_assets 23.09, stage/render/prepare_flush 21.08 |
| warm-up | 3.66 | 114.5 | pipelines/compile_ms 1766, pipelines/front_ms 1683, pipelines/wait_ms 561, main/busy/ms 103, render/thread/ms 103, render/busy/ms 100, pipelines/driver_ms 82.88, stage/render/prepare_assets 66.47, stage/main/update 61.10, stage/main/post_update 18.74, stage/main/fixed_main_loop 14.79, terrain/update/ms 14.41 |
| warm-up | 3.70 | 47.7 | pipelines/compile_ms 2384, pipelines/front_ms 2171, pipelines/wait_ms 262, pipelines/driver_ms 213, render/thread/ms 111, render/busy/ms 109, stage/render/prepare_assets 64.57, main/busy/ms 33.01, stage/render/create_views 28.24, encode//sum 18.00, stage/main/post_update 16.25, stage/render/render 11.17 |
| warm-up | 3.73 | 30.2 | pipelines/compile_ms 1097, pipelines/front_ms 947, pipelines/driver_ms 150, render/thread/ms 75.42, render/busy/ms 72.88, main/busy/ms 31.72, stage/render/render 30.13, encode//sum 30.10, stage/render/create_views 28.44, render/graph/ms 27.22, stage/render/graph_submit 24.26, stage/main/post_update 19.26 |
| warm-up | 3.77 | 28.0 | pipelines/compile_ms 66.28, pipelines/front_ms 63.80, render/thread/ms 31.85, render/busy/ms 30.44, main/busy/ms 22.22, stage/render/prepare_assets 15.37, stage/main/post_update 13.44, stage/render/render 9.11, terrain/update/ms 8.24, encode//sum 7.53, render/graph/ms 7.24, stage/main/update 5.46 |
| warm-up | 3.87 | 85.3 | pipelines/compile_ms 849, pipelines/front_ms 817, main/busy/ms 57.04, stage/main/update 47.54, render/thread/ms 45.41, render/busy/ms 44.06, stream/drive/ms 42.69, pipelines/driver_ms 31.91, stage/render/prepare_assets 30.55, stage/render/render 9.16, render/graph/ms 7.69, encode//sum 6.84 |
| warm-up | 3.91 | 31.1 | pipelines/compile_ms 97.37, pipelines/front_ms 89.40, render/thread/ms 34.08, render/busy/ms 32.61, main/busy/ms 25.96, stage/render/prepare_assets 16.26, stage/main/update 13.72, stage/render/render 11.74, render/graph/ms 9.79, stream/drive/ms 9.49, pipelines/driver_ms 7.97, stage/render/graph_submit 7.21 |
| warm-up | 4.01 | 76.5 | pipelines/compile_ms 264, pipelines/front_ms 254, render/thread/ms 67.39, render/busy/ms 65.52, main/busy/ms 53.39, stage/render/prepare_assets 41.08, stage/main/update 38.46, stream/drive/ms 31.39, stage/render/render 13.78, render/graph/ms 11.83, pipelines/driver_ms 10.44, encode//sum 10.14 |
| warm-up | 4.04 | 28.0 | render/thread/ms 69.08, render/busy/ms 67.10, pipelines/compile_ms 55.37, pipelines/front_ms 51.97, stage/render/prepare_assets 39.12, main/busy/ms 34.67, stage/render/render 16.90, stage/main/post_update 14.70, render/graph/ms 14.67, stage/main/update 11.77, stage/render/graph_submit 10.30, encode//sum 8.85 |
| warm-up | 4.13 | 86.7 | pipelines/compile_ms 1162, pipelines/front_ms 1115, render/thread/ms 72.92, render/busy/ms 71.13, main/busy/ms 67.42, pipelines/driver_ms 46.89, stage/render/prepare_assets 46.48, stage/main/update 41.01, stage/main/post_update 18.87, stage/render/render 13.28, render/graph/ms 11.00, stage/render/graph_submit 6.68 |
| warm-up | 4.24 | 109.1 | pipelines/compile_ms 2085, pipelines/front_ms 2008, render/thread/ms 130, render/busy/ms 125, stage/render/prepare_assets 92.00, pipelines/driver_ms 76.70, main/busy/ms 68.60, stage/main/update 45.69, stage/render/render 26.30, render/graph/ms 21.78, stage/render/graph_submit 19.31, encode/submit 16.96 |
| warm-up | 4.30 | 60.6 | pipelines/compile_ms 1286, pipelines/front_ms 1244, pipelines/wait_ms 187, render/thread/ms 130, render/busy/ms 126, stage/render/prepare_assets 74.34, pipelines/driver_ms 42.49, main/busy/ms 37.94, stage/render/render 27.80, render/graph/ms 23.31, stage/render/graph_submit 19.93, encode/submit 17.94 |
| warm-up | 4.38 | 87.0 | pipelines/compile_ms 1188, pipelines/front_ms 1153, pipelines/wait_ms 478, render/thread/ms 125, render/busy/ms 123, main/busy/ms 86.78, stage/render/render 47.83, render/graph/ms 45.68, stage/main/post_update 41.88, stage/render/graph_submit 40.69, stage/render/prepare_assets 37.10, encode/finish_wall 34.88 |
| warm-up | 4.44 | 54.9 | pipelines/compile_ms 2308, pipelines/front_ms 2233, pipelines/wait_ms 862, render/thread/ms 124, render/busy/ms 122, main/busy/ms 99.43, pipelines/driver_ms 75.08, encode//sum 70.74, stage/render/render 60.32, render/graph/ms 58.04, stage/render/graph_submit 51.15, stage/main/post_update 47.54 |
| warm-up | 4.50 | 59.3 | pipelines/compile_ms 1795, pipelines/front_ms 1726, pipelines/wait_ms 1272, render/thread/ms 99.82, render/busy/ms 98.04, pipelines/driver_ms 69.25, main/busy/ms 57.81, encode//sum 50.69, stage/render/prepare_assets 31.08, stage/render/render 29.66, render/graph/ms 27.25, stage/main/post_update 25.35 |
| warm-up | 4.60 | 101.9 | pipelines/compile_ms 2007, pipelines/front_ms 1942, pipelines/wait_ms 1642, render/thread/ms 142, render/busy/ms 141, main/busy/ms 111, stage/main/update 70.18, pipelines/driver_ms 65.54, stream/drive/ms 55.87, stage/render/prepare_resources 37.34, stage/render/prepare_assets 31.36, stage/main/post_update 27.56 |
| warm-up | 4.65 | 46.1 | pipelines/compile_ms 2388, pipelines/front_ms 2312, pipelines/wait_ms 1522, main/busy/ms 136, render/thread/ms 131, render/busy/ms 128, stage/main/update 98.89, pipelines/driver_ms 76.12, stream/drive/ms 57.53, stage/render/prepare_resources 39.17, stage/render/render 32.98, render/graph/ms 28.36 |
| warm-up | 4.73 | 86.0 | pipelines/compile_ms 1635, pipelines/front_ms 1582, pipelines/wait_ms 1011, main/busy/ms 122, stage/main/update 95.64, render/thread/ms 82.35, render/busy/ms 78.94, pipelines/driver_ms 53.58, stream/drive/ms 50.02, stage/render/prepare_assets 27.28, stage/render/render 27.21, render/graph/ms 22.64 |
| warm-up | 4.84 | 105.0 | pipelines/compile_ms 1537, pipelines/front_ms 1498, pipelines/wait_ms 618, render/thread/ms 96.67, main/busy/ms 96.48, render/busy/ms 94.53, stage/main/update 69.31, stage/render/prepare_assets 57.28, stream/drive/ms 49.08, pipelines/driver_ms 38.79, stage/render/render 18.90, render/graph/ms 16.08 |
| warm-up | 4.92 | 83.1 | pipelines/compile_ms 1401, pipelines/front_ms 1347, pipelines/wait_ms 199, render/thread/ms 130, render/busy/ms 129, main/busy/ms 92.65, stage/main/update 66.22, stage/render/render 62.74, render/graph/ms 60.58, stage/render/graph_submit 55.27, pipelines/driver_ms 53.97, stage/render/prepare_assets 52.97 |
| warm-up | 4.97 | 47.6 | pipelines/compile_ms 972, pipelines/front_ms 921, render/thread/ms 109, render/busy/ms 108, main/busy/ms 88.42, stage/main/update 70.27, stage/render/render 61.21, render/graph/ms 59.12, stage/render/graph_submit 54.21, pipelines/driver_ms 50.86, encode//sum 49.12, stream/drive/ms 46.60 |
| warm-up | 5.00 | 31.4 | pipelines/compile_ms 412, pipelines/front_ms 394, main/busy/ms 62.30, render/thread/ms 55.87, render/busy/ms 54.42, stage/main/update 47.44, stream/drive/ms 29.31, stage/render/prepare_assets 24.50, pipelines/driver_ms 18.17, stage/render/render 13.93, render/graph/ms 11.84, encode//sum 10.10 |
| warm-up | 5.03 | 33.1 | pipelines/compile_ms 754, pipelines/front_ms 734, main/busy/ms 53.34, render/thread/ms 41.78, render/busy/ms 40.40, stage/main/update 39.37, stream/drive/ms 28.52, pipelines/driver_ms 20.28, stage/render/render 14.45, render/graph/ms 12.37, stage/render/prepare_assets 11.96, encode//sum 10.42 |
| warm-up | 5.06 | 28.4 | pipelines/compile_ms 767, pipelines/front_ms 747, render/thread/ms 42.28, render/busy/ms 40.80, main/busy/ms 31.75, pipelines/driver_ms 20.39, stage/render/render 17.58, render/graph/ms 15.54, stage/main/update 14.77, stage/main/post_update 12.18, encode//sum 11.12, stage/render/graph_submit 10.77 |
| warm-up | 5.11 | 34.0 | pipelines/compile_ms 130, pipelines/front_ms 125, render/thread/ms 45.57, render/busy/ms 43.90, main/busy/ms 33.58, stage/render/render 15.32, stage/render/prepare_assets 15.08, stage/main/post_update 14.75, render/graph/ms 13.04, stage/main/update 12.80, encode//sum 10.48, stage/render/graph_submit 8.41 |
| warm-up | 5.16 | 43.1 | pipelines/compile_ms 178, pipelines/front_ms 172, render/thread/ms 67.72, render/busy/ms 65.73, main/busy/ms 40.55, stage/main/post_update 18.91, stage/render/render 17.97, stage/render/prepare_assets 16.45, stage/main/update 15.50, render/graph/ms 15.42, encode//sum 11.76, stage/render/prepare_resources 11.29 |
| warm-up | 5.19 | 31.4 | pipelines/compile_ms 308, pipelines/front_ms 297, render/thread/ms 66.31, render/busy/ms 64.58, main/busy/ms 39.86, stage/render/render 17.48, stage/main/post_update 16.46, stage/main/update 16.35, stage/render/prepare_assets 15.27, render/graph/ms 15.13, pipelines/driver_ms 11.46, encode//sum 11.44 |
| warm-up | 5.24 | 49.8 | pipelines/compile_ms 333, pipelines/front_ms 322, render/thread/ms 61.89, render/busy/ms 60.56, main/busy/ms 38.00, stage/render/prepare_assets 34.10, stage/main/update 20.70, stage/render/render 13.87, render/graph/ms 11.85, pipelines/driver_ms 11.52, stream/drive/ms 11.18, stage/main/post_update 10.35 |
| warm-up | 5.27 | 30.1 | pipelines/compile_ms 859, pipelines/front_ms 833, render/thread/ms 64.77, render/busy/ms 63.38, main/busy/ms 40.27, stage/render/prepare_assets 27.29, pipelines/driver_ms 26.21, stage/main/update 21.27, stage/render/render 15.62, render/graph/ms 13.57, stage/main/post_update 12.88, stream/drive/ms 11.27 |
| warm-up | 5.30 | 28.5 | pipelines/compile_ms 1320, pipelines/front_ms 1280, render/thread/ms 54.39, render/busy/ms 52.57, main/busy/ms 40.75, pipelines/driver_ms 40.51, pipelines/wait_ms 31.59, stage/render/render 18.59, stage/main/post_update 18.10, stage/main/update 16.34, render/graph/ms 16.22, encode//sum 12.20 |
| warm-up | 5.36 | 37.5 | pipelines/compile_ms 410, pipelines/front_ms 401, render/thread/ms 50.53, render/busy/ms 49.32, main/busy/ms 30.00, stage/render/prepare_assets 26.93, stage/main/update 12.45, stage/main/post_update 11.45, stage/render/render 10.71, encode//sum 10.03, render/graph/ms 8.89, pipelines/driver_ms 8.85 |
| warm-up | 5.51 | 33.7 | render/thread/ms 50.98, render/busy/ms 49.74, main/busy/ms 28.08, stage/render/prepare_assets 25.80, stage/main/post_update 12.80, stage/render/render 12.46, render/graph/ms 10.75, encode//sum 10.20, stage/main/update 10.16, stage/render/graph_submit 7.34, encode/submit 4.85, encode/pass:shadow_point_light/sum 4.84 |
| warm-up | 5.54 | 34.3 | render/thread/ms 56.68, render/busy/ms 55.37, stage/render/prepare_assets 33.97, main/busy/ms 25.00, stage/render/render 13.66, render/graph/ms 11.73, stage/main/post_update 10.56, stage/main/update 8.91, pipelines/compile_ms 8.47, encode//sum 8.45, stage/render/graph_submit 8.26, pipelines/front_ms 7.53 |
| warm-up | 6.07 | 32.5 | render/thread/ms 44.55, render/busy/ms 43.05, main/busy/ms 31.87, stage/render/render 17.98, render/graph/ms 16.11, stage/main/update 14.85, encode//sum 13.85, stage/main/post_update 13.22, stage/render/graph_submit 11.44, encode/pass:shadow_point_light/sum 8.06, encode/submit 7.96, stream/drive/ms 5.38 |
| warm-up | 6.12 | 32.2 | render/thread/ms 38.59, render/busy/ms 37.62, main/busy/ms 25.82, stage/render/prepare_assets 13.15, stage/render/render 11.63, render/graph/ms 10.20, stage/main/post_update 9.92, stage/main/update 9.28, encode//sum 8.91, stage/render/graph_submit 7.04, encode/pass:shadow_point_light/sum 5.03, encode/submit 4.51 |
| warm-up | 6.36 | 29.4 | render/thread/ms 45.89, render/busy/ms 44.66, main/busy/ms 43.41, stage/main/update 18.97, stage/render/render 18.24, stage/main/post_update 17.52, render/graph/ms 16.46, encode//sum 13.13, stage/render/graph_submit 11.13, stage/render/prepare_assets 9.46, encode/pass:shadow_point_light/sum 8.36, encode/submit 7.00 |
| warm-up | 6.47 | 41.2 | main/busy/ms 44.21, render/thread/ms 33.70, render/busy/ms 32.49, stage/main/update 25.01, stream/drive/ms 14.51, stage/main/post_update 13.87, stage/render/render 13.23, render/graph/ms 11.43, encode//sum 10.11, stage/render/prepare_assets 8.66, stage/render/graph_submit 7.69, encode/pass:shadow_point_light/sum 7.35 |
| warm-up | 6.49 | 28.3 | render/thread/ms 42.69, render/busy/ms 41.47, main/busy/ms 37.86, stage/main/update 16.86, stage/main/post_update 15.12, stage/render/prepare_assets 14.77, stage/render/render 14.76, render/graph/ms 12.84, encode//sum 12.22, encode/pass:shadow_point_light/sum 10.15, stage/render/graph_submit 8.16, stage/render/camera_driver 4.52 |
| warm-up | 6.71 | 27.3 | pipelines/compile_ms 149, pipelines/front_ms 144, render/thread/ms 48.07, render/busy/ms 46.86, main/busy/ms 29.20, stage/render/prepare_assets 26.16, stage/main/post_update 12.61, stage/render/render 12.18, stage/main/update 11.35, render/graph/ms 10.34, encode//sum 9.89, stage/render/graph_submit 7.27 |
| warm-up | 6.92 | 39.9 | main/busy/ms 45.12, render/thread/ms 32.00, render/busy/ms 30.88, stage/main/update 24.83, stream/drive/ms 14.30, stage/main/post_update 12.71, stage/render/prepare_assets 12.22, stage/render/render 11.80, encode//sum 10.35, render/graph/ms 10.22, stage/render/graph_submit 7.10, encode/pass:shadow_point_light/sum 6.98 |
| measure | 18.18 | 30.5 | main/busy/ms 38.45, stage/main/update 24.20, render/thread/ms 24.13, render/busy/ms 22.99, stream/drive/ms 20.26, stage/render/render 14.27, render/graph/ms 12.72, stage/main/post_update 11.58, encode//sum 11.30, stage/render/graph_submit 9.16, encode/pass:shadow_point_light/sum 7.34, encode/submit 6.20 |
| measure | 23.08 | 27.4 | render/thread/ms 38.89, render/busy/ms 37.68, main/busy/ms 30.14, encode//sum 18.07, stage/render/render 15.68, stage/main/post_update 14.56, render/graph/ms 13.91, stage/main/update 11.81, encode/pass:shadow_point_light/sum 10.16, stage/render/graph_submit 9.78, stage/render/prepare_assets 9.29, encode/submit 5.10 |
| measure | 59.04 | 29.3 | render/thread/ms 37.41, render/busy/ms 36.23, main/busy/ms 27.66, stage/main/post_update 15.03, stage/render/prepare_assets 14.28, stage/render/render 12.78, encode//sum 11.55, render/graph/ms 11.33, stage/render/graph_submit 7.99, stage/main/update 7.84, encode/submit 4.23, encode/finish_wall 3.81 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=base-5 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 20%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **70%** (min 44%) · 2625 MHz (min 2625) · memory 8101 MHz · 74.1 W · 75 °C (max 81) · VRAM 6790 MiB · throttled 100% of samples

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
