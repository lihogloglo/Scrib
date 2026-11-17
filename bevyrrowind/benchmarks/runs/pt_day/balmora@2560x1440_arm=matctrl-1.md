# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); mw.matctrl.gate=on (command line: --set); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1534 frames over 15.0 s · **median 9.73 ms (103 fps)** · mean 9.78 · p95 11.32 · p99 12.39 (1% low 81 fps) · worst 26.36 ms · 5 spikes over 1.5× median · 20.0 stutters/min (work 12.0/min)
- warm-up: 280 frames over 8.6 s, worst 2545.4 ms
- hitches (> 25.0 ms): **1** measured, 58 in warm-up
- timed GPU passes: 8.46 ms of a 9.78 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.73 ms, limit 11 ms
- pass worst frame 26.36 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.820 | 2.838 | 2.993 | 0.99 | 0.073 |
| dlss_super_resolution | 2.022 | 2.035 | 2.050 | 0.99 | 0.184 |
| early prepass | 0.612 | 0.616 | 0.630 | 0.99 | 0.096 |
| atmosphere_luts | 0.323 | 0.325 | 0.394 | 0.99 | 0.003 |
| bloom | 0.313 | 0.315 | 0.330 | 0.99 | 0.061 |
| main_transmissive_pass_3d | 0.292 | 0.294 | 0.331 | 0.99 | 0.009 |
| water_velocity | 0.238 | 0.240 | 0.265 | 0.99 | 0.002 |
| water_sspr | 0.204 | 0.206 | 0.213 | 0.99 | 0.002 |
| ssao | 0.190 | 0.191 | 0.198 | 0.99 | 0.007 |
| clouds_trace | 0.152 | 0.153 | 0.189 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.144 | 0.145 | 0.152 | 0.99 | 0.001 |
| water_fft | 0.130 | 0.131 | 0.134 | 0.99 | 0.003 |
| uniform_allocation | 0.120 | 0.121 | 0.163 | 0.99 | 0.022 |
| upscaling | 0.112 | 0.113 | 0.120 | 0.99 | 0.004 |
| clouds_composite | 0.112 | 0.113 | 0.114 | 0.99 | 0.002 |
| bin_unpacking | 0.092 | 0.093 | 0.122 | 0.99 | 0.016 |
| tonemapping | 0.086 | 0.086 | 0.092 | 0.99 | 0.003 |
| clustering | 0.065 | 0.066 | 0.079 | 0.99 | 0.021 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.415 | 0.13 | 0.001 |
| clouds_resolve | 0.051 | 0.051 | 0.052 | 0.99 | 0.001 |
| water_snapshot | 0.034 | 0.034 | 0.035 | 0.99 | 0.002 |
| clouds_shadow | 0.032 | 0.032 | 0.036 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.033 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.030 | 0.030 | 0.041 | 0.99 | 0.005 |
| lightprobe_radiance_map | 0.028 | 0.220 | 0.231 | 0.13 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.026 | 0.029 | 0.99 | 0.082 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.046 | 0.31 | 0.002 |
| surface_exposure | 0.012 | 0.045 | 0.043 | 0.27 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.99 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.13 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.824 | 10.402 | 25.14 | 13535.5 |
| main/busy/ms | 7.276 | 8.163 | 11.03 | 11160.8 |
| render/busy/ms | 7.193 | 8.446 | 24.61 | 11034.5 |
| stage/render/render | 4.934 | 5.940 | 10.73 | 7568.5 |
| render/graph/ms | 4.329 | 5.266 | 9.07 | 6641.4 |
| stage/main/post_update | 4.281 | 5.016 | 6.45 | 6567.7 |
| encode//sum | 4.047 | 5.411 | 14.81 | 6208.5 |
| stage/render/graph_submit | 3.315 | 4.227 | 7.08 | 5085.3 |
| encode/submit | 2.126 | 2.736 | 5.88 | 3261.8 |
| stage/main/update | 1.857 | 2.282 | 3.95 | 2849.3 |
| stage/render/prepare_views | 1.295 | 3.103 | 4.79 | 1985.8 |
| render/acquire/ms | 1.248 | 3.061 | 4.74 | 1914.0 |
| encode/finish_wall | 1.218 | 1.722 | 2.74 | 1868.4 |
| stage/render/camera_driver | 0.957 | 1.100 | 1.84 | 1468.7 |
| encode/pass:shadow_directional_light/sum | 0.765 | 1.336 | 1.93 | 1172.8 |
| encode/pass:early prepass/sum | 0.740 | 1.027 | 1.49 | 1135.0 |
| encode/pass:main_opaque_pass_3d/sum | 0.739 | 1.045 | 1.63 | 1133.2 |
| stage/render/prepare_write_phase_buffers | 0.552 | 0.774 | 1.29 | 846.8 |
| bigspace/total/ms | 0.408 | 1.050 | 2.12 | 626.2 |
| stage/render/prepare_assets | 0.403 | 0.498 | 1.44 | 618.2 |
| stage/render/prepare_resources | 0.395 | 0.468 | 0.95 | 606.2 |
| render/present/ms | 0.383 | 0.459 | 2.68 | 587.0 |
| stage/main/pre_update | 0.352 | 0.491 | 0.91 | 540.7 |
| stage/main/first | 0.293 | 0.361 | 1.48 | 449.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.277 | 0.420 | 0.83 | 425.0 |
| terrain/update/ms | 0.235 | 0.304 | 0.65 | 360.3 |
| stage/render/prepare_meshes | 0.211 | 0.269 | 3.13 | 323.2 |
| stage/render/prepare_bind_groups | 0.210 | 0.292 | 0.49 | 322.4 |
| terrain/select/ms | 0.204 | 0.262 | 0.60 | 312.5 |
| stream/drive/ms | 0.193 | 0.265 | 0.69 | 295.8 |
| stage/main/fixed_main_loop | 0.191 | 0.378 | 1.02 | 292.3 |
| stage/render/prepare_batch_phases | 0.179 | 0.238 | 0.40 | 273.9 |
| stage/render/create_views | 0.175 | 0.233 | 0.63 | 268.2 |
| stage/render/extract_commands | 0.173 | 0.208 | 0.57 | 264.7 |
| far/select/ms | 0.171 | 0.236 | 0.62 | 262.3 |
| stream/tick/ms | 0.164 | 0.233 | 0.66 | 252.0 |
| stage/render/prepare_flush | 0.155 | 0.262 | 0.76 | 238.5 |
| bigspace/high_precision/ms | 0.155 | 0.666 | 1.83 | 237.7 |
| controller/move/ms | 0.146 | 0.279 | 0.72 | 223.8 |
| controller/feed/ms | 0.146 | 0.196 | 0.49 | 223.4 |
| encode/pass:main_transmissive_pass_3d/sum | 0.128 | 0.203 | 0.49 | 196.1 |
| stage/main/last | 0.119 | 0.156 | 0.71 | 182.0 |
| bigspace/low_precision/ms | 0.105 | 0.237 | 1.29 | 160.5 |
| stage/render/queue | 0.097 | 0.126 | 5.33 | 148.6 |
| anim/evaluate/ms | 0.097 | 0.240 | 1.41 | 148.2 |
| stage/render/specialize | 0.072 | 0.089 | 8.14 | 109.9 |
| anim/evaluate_cpu/ms | 0.062 | 0.076 | 0.52 | 95.1 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.112 | 0.18 | 88.6 |
| encode/resolve_diagnostics/sum | 0.056 | 0.090 | 0.30 | 86.6 |
| physics/step/ms | 0.029 | 0.059 | 0.44 | 44.7 |
| encode/camera_driver_span_begin/sum | 0.028 | 0.044 | 0.10 | 42.2 |
| weather/map/ms | 0.025 | 0.033 | 0.43 | 39.0 |
| encode/camera_driver_span_end/sum | 0.024 | 0.042 | 0.11 | 36.6 |
| stage/render/cleanup | 0.023 | 0.033 | 0.29 | 35.1 |
| stage/render/phase_sort | 0.019 | 0.035 | 0.08 | 28.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.42 | 17.0 |
| script/update/ms | 0.007 | 0.010 | 0.04 | 10.8 |
| controller/avoid/ms | 0.006 | 0.008 | 0.09 | 9.1 |
| script/input/ms | 0.006 | 0.007 | 0.07 | 8.5 |
| anim/apply/ms | 0.005 | 0.006 | 0.02 | 6.9 |
| script/ui/ms | 0.005 | 0.006 | 0.04 | 6.9 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 283 | 283 |
| crowd/c2/joints/count | count | 4916 | 4916 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1373 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.55 | 2.38 | 0.34 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.10 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.11 | 0.01 |
| encode/finish_critical | ms | 0.89 | 2.38 | 0.91 |
| encode/pass:early prepass/max | ms | 0.73 | 1.44 | 0.69 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.73 | 1.56 | 0.74 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.49 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.26 | 0.83 | 0.23 |
| encode/pass:shadow_directional_light/max | ms | 0.69 | 1.90 | 0.91 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.30 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.13 | 24.61 | 8.99 |
| gpu/timed/ms | ms | 9.13 | 11.75 | 8.99 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1943 | 1943 | 1943 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 207.2 k | 207.2 k | 207.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 13.3 k | 13.3 k | 13.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 485.99 M | 485.99 M | 485.99 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 21 | 21 | 21 |
| phases/opaque3d/bins | count | 1306 | 1306 | 1306 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/bins | count | 1306 | 1306 | 1306 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 1 | 30 | 23 |
| phases/shadow/bins | count | 2152 | 4889 | 2152 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6053 | 17.7 k | 6053 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 23 | 30 | 23 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 320 | 320 | 320 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 0 | 62 | 46 |
| preprocess/bind_groups_created | count | 0 | 62 | 46 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 127 | 127 | 127 |
| process/cpu_usage | % | 25.48 | 39.42 | 24.04 |
| process/mem_usage | GiB | 4.67 | 4.71 | 4.71 |
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
| skin/blocks_changed/count | count | 5 | 7 | 4 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 181 | 181 | 181 |
| skin/joint_bytes_written/bytes | bytes | 309.5 k | 309.5 k | 309.5 k |
| skin/joints_written/count | count | 4836 | 4836 | 4836 |
| skin/upload_writes/count | count | 1 | 1 | 1 |
| started_load_count |  | 3704 | 3706 | 3706 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 386 | 386 | 386 |
| statics/materials_bound | count | 386 | 386 | 386 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.19 | 44.92 | 26.79 |
| system/mem_usage | % | 58.72 | 58.88 | 58.88 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 53 | 82 | 75 |
| views/all/bins | count | 5324 | 8061 | 5324 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 15.3 k | 27.0 k | 15.3 k |
| views/all/pipelines | count | 71 | 78 | 71 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 9496 | 27.2 k | 15.5 k |
| views/cascade/batch_sets | count | 1 | 30 | 23 |
| views/cascade/bins | count | 2152 | 4889 | 2152 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 6053 | 17.7 k | 6053 |
| views/cascade/pipelines | count | 23 | 30 | 23 |
| views/cascade/work_items | count | 34 | 17.7 k | 6053 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 52 | 52 | 52 |
| views/main/bins | count | 3172 | 3172 | 3172 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 48 | 48 | 48 |
| views/main/work_items | count | 9462 | 9462 | 9462 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.09 | 4 | 140 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 0.00 | 4 | 4 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.56 | 52 | 40.7 k |
| encode/bind_walk/init_textures | count | 7167 | 16.3 k | 10.99 M |
| encode/bind_walk/init_textures_skipped | count | 43.4 k | 95.5 k | 66.53 M |
| encode/bind_walk/inits | count | 345 | 721 | 529.8 k |
| encode/bind_walk/inits_skipped | count | 75.01 | 158 | 115.1 k |
| encode/bind_walk/merge_textures | count | 7103 | 16.2 k | 10.90 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 8162 | 6.26 M |
| encode/bind_walk/merges | count | 82.27 | 173 | 126.2 k |
| encode/bind_walk/merges_skipped | count | 54.75 | 113 | 84.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1534 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1534 |
| encode/pass:early prepass/count | count | 1 | 2 | 1534 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1534 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 1534 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 1534 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 1534 |
| encode/queue/command_buffers | count | 36.56 | 72 | 56.1 k |
| encode/queue/submits | count | 2 | 4 | 3068 |
| encode/queue/write_buffer/bytes | bytes | 1.38 M | 3.65 M | 2.12 G |
| encode/queue/write_buffer/calls | count | 217 | 468 | 332.9 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 65.3 k | 50.07 M |
| encode/queue/write_buffer_with/calls | count | 9 | 18 | 13.8 k |
| encode/queue/write_texture/bytes | bytes | 11.7 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1534 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.59 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| precache/hit | count | 0 | 0 | 0 |
| precache/missed | count | 0 | 0 | 0 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0 | 0 | 0 |
| stream/deactivate/items | count | 0 | 0 | 0 |
| stream/despawn/items | count | 0 | 0 | 0 |
| stream/fetch/items | count | 0 | 0 | 0 |
| stream/spawn/items | count | 0 | 0 | 0 |
| stream/upload/bytes | bytes | 0 | 0 | 0 |
| stream/upload/items | count | 0 | 0 | 0 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 1 |
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 59, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2545.4 | main/busy/ms 2514, stage/main/post_update 2404, stage/main/update 101, surface/advance_cpu_ms 68.12, stage/main/pre_update 5.49, stage/main/last 1.19, stream/drive/ms 0.81, surface/cell_upload_ms 0.78, stage/main/first 0.54, stage/main/fixed_main_loop 0.51, weather/map/ms 0.19, stream/tick/ms 0.13 |
| warm-up | 3.41 | 865.0 | main/busy/ms 2692, stage/main/post_update 2428, render/thread/ms 824, render/busy/ms 659, stage/render/prepare_assets 613, stage/main/update 248, stage/render/prepare_views 162, render/acquire/ms 162, surface/advance_cpu_ms 68.12, stage/render/create_views 27.83, stage/render/render 11.70, stage/main/pre_update 7.51 |
| warm-up | 3.53 | 116.9 | render/thread/ms 924, render/busy/ms 701, stage/render/prepare_assets 639, main/busy/ms 200, stage/render/prepare_views 163, render/acquire/ms 162, stage/main/update 152, stage/render/render 74.75, pipelines/compile_ms 66.09, pipelines/front_ms 62.39, render/present/ms 60.35, stage/main/post_update 37.60 |
| warm-up | 3.57 | 44.1 | pipelines/compile_ms 468, pipelines/front_ms 426, pipelines/wait_ms 327, render/thread/ms 139, stage/render/render 96.45, render/busy/ms 74.08, main/busy/ms 70.60, render/present/ms 64.12, stage/main/post_update 54.49, pipelines/driver_ms 41.97, render/graph/ms 31.16, stage/render/prepare_assets 27.29 |
| warm-up | 3.70 | 132.1 | pipelines/wait_ms 4548, pipelines/compile_ms 1469, pipelines/front_ms 1374, render/thread/ms 163, render/busy/ms 155, pipelines/driver_ms 94.95, main/busy/ms 73.70, stage/render/render 57.73, stage/main/post_update 52.97, render/graph/ms 49.02, stage/render/graph_submit 38.60, stage/render/prepare_resources 37.97 |
| warm-up | 3.75 | 48.8 | pipelines/wait_ms 4577, pipelines/compile_ms 1804, pipelines/front_ms 1709, render/thread/ms 164, render/busy/ms 162, main/busy/ms 110, pipelines/driver_ms 95.25, encode//sum 75.50, stage/main/update 60.37, stage/render/prepare_resources 48.85, stage/render/render 44.65, render/graph/ms 41.15 |
| warm-up | 3.90 | 145.9 | pipelines/compile_ms 2483, pipelines/front_ms 2281, pipelines/wait_ms 476, encode//sum 238, pipelines/driver_ms 203, render/thread/ms 138, render/busy/ms 135, main/busy/ms 101, stage/render/prepare_assets 63.67, stage/main/update 62.75, stage/render/render 49.96, render/graph/ms 46.41 |
| warm-up | 3.93 | 31.9 | pipelines/compile_ms 1757, pipelines/front_ms 1592, encode//sum 189, pipelines/driver_ms 165, pipelines/wait_ms 147, render/thread/ms 127, render/busy/ms 124, stage/render/prepare_assets 75.47, stage/render/render 44.34, render/graph/ms 40.97, stage/render/graph_submit 38.58, main/busy/ms 32.14 |
| warm-up | 3.96 | 26.6 | pipelines/compile_ms 286, pipelines/front_ms 271, render/thread/ms 55.71, render/busy/ms 50.31, main/busy/ms 41.97, pipelines/wait_ms 27.55, stage/render/render 24.83, stage/main/post_update 22.44, render/graph/ms 18.83, stage/render/graph_submit 15.99, pipelines/driver_ms 15.22, stage/render/prepare_assets 13.78 |
| warm-up | 4.04 | 45.5 | pipelines/compile_ms 230, pipelines/front_ms 210, render/thread/ms 61.00, render/busy/ms 59.16, main/busy/ms 57.00, stage/main/update 45.20, stream/drive/ms 40.84, stage/render/prepare_assets 40.23, pipelines/driver_ms 20.29, stage/render/render 12.36, render/graph/ms 10.11, terrain/update/ms 9.62 |
| warm-up | 4.09 | 43.0 | pipelines/compile_ms 963, pipelines/front_ms 920, render/thread/ms 75.89, render/busy/ms 73.86, main/busy/ms 53.94, stage/render/prepare_assets 50.30, stage/main/update 45.30, pipelines/driver_ms 42.79, stream/drive/ms 40.71, pipelines/wait_ms 29.58, stage/render/render 17.83, render/graph/ms 15.31 |
| warm-up | 4.11 | 27.3 | pipelines/compile_ms 1262, pipelines/front_ms 1223, render/thread/ms 58.47, render/busy/ms 56.47, pipelines/driver_ms 39.08, main/busy/ms 33.57, pipelines/wait_ms 32.88, stage/main/update 23.55, stage/render/prepare_assets 21.19, stream/drive/ms 18.89, stage/render/render 18.54, render/graph/ms 16.18 |
| warm-up | 4.14 | 28.6 | pipelines/compile_ms 531, pipelines/front_ms 515, render/thread/ms 47.50, render/busy/ms 45.76, main/busy/ms 38.47, stage/main/update 25.79, stream/drive/ms 20.49, pipelines/driver_ms 16.35, stage/render/render 14.40, stage/render/prepare_assets 13.84, render/graph/ms 12.10, stage/render/graph_submit 8.08 |
| warm-up | 4.19 | 44.7 | pipelines/compile_ms 223, pipelines/front_ms 214, render/thread/ms 59.47, render/busy/ms 57.88, stage/render/prepare_assets 37.68, main/busy/ms 31.56, stage/render/render 14.60, stage/main/update 14.55, render/graph/ms 11.54, encode//sum 10.46, stream/drive/ms 8.63, pipelines/driver_ms 8.55 |
| warm-up | 4.24 | 51.9 | pipelines/compile_ms 402, pipelines/front_ms 390, main/busy/ms 119, stage/main/update 99.47, render/thread/ms 78.10, render/busy/ms 76.28, stage/render/prepare_assets 54.89, stream/drive/ms 43.20, stage/render/render 15.61, render/graph/ms 12.53, pipelines/driver_ms 12.00, stage/main/post_update 10.46 |
| warm-up | 4.43 | 193.1 | pipelines/compile_ms 1734, pipelines/front_ms 1689, render/thread/ms 134, render/busy/ms 132, main/busy/ms 122, stage/render/prepare_assets 114, stage/main/update 101, pipelines/driver_ms 45.12, stream/drive/ms 39.23, stage/main/post_update 12.77, stage/render/render 12.06, render/graph/ms 9.93 |
| warm-up | 4.48 | 45.8 | pipelines/compile_ms 1688, pipelines/front_ms 1628, render/thread/ms 135, render/busy/ms 129, stage/render/prepare_assets 90.26, main/busy/ms 81.44, pipelines/driver_ms 60.32, stage/main/update 57.98, stream/drive/ms 45.29, stage/render/render 33.29, render/graph/ms 26.55, stage/render/graph_submit 23.96 |
| warm-up | 4.53 | 54.0 | pipelines/compile_ms 295, pipelines/front_ms 270, pipelines/wait_ms 197, main/busy/ms 74.32, render/thread/ms 65.43, render/busy/ms 59.01, stage/main/update 54.24, stream/drive/ms 45.62, stage/render/render 32.27, render/graph/ms 25.58, pipelines/driver_ms 25.04, stage/render/graph_submit 23.10 |
| warm-up | 4.57 | 36.3 | pipelines/wait_ms 181, pipelines/compile_ms 170, pipelines/front_ms 162, render/thread/ms 54.75, render/busy/ms 53.29, main/busy/ms 45.90, stage/main/update 26.24, stage/render/prepare_assets 16.55, stage/main/post_update 11.61, stage/render/render 9.27, pipelines/driver_ms 7.52, render/graph/ms 7.49 |
| warm-up | 4.64 | 75.1 | pipelines/compile_ms 250, pipelines/front_ms 240, render/thread/ms 98.20, render/busy/ms 96.58, main/busy/ms 89.14, pipelines/wait_ms 73.85, stage/main/update 54.82, stage/render/prepare_write_phase_buffers 34.20, stage/render/prepare_flush 17.27, stage/main/post_update 16.28, stage/render/prepare_assets 11.89, stage/main/last 10.39 |
| warm-up | 4.69 | 41.7 | pipelines/compile_ms 1967, pipelines/front_ms 1908, pipelines/wait_ms 304, render/thread/ms 95.45, render/busy/ms 93.84, main/busy/ms 74.90, pipelines/driver_ms 58.39, stage/main/update 41.97, stage/render/prepare_write_phase_buffers 34.22, stage/render/prepare_assets 27.52, stage/main/post_update 16.55, stage/render/prepare_flush 13.88 |
| warm-up | 4.72 | 38.5 | pipelines/compile_ms 2200, pipelines/front_ms 2137, pipelines/wait_ms 284, pipelines/driver_ms 62.92, render/thread/ms 52.78, render/busy/ms 51.40, stage/render/prepare_assets 35.79, main/busy/ms 34.70, stage/main/post_update 15.24, stage/main/update 11.54, stage/render/render 9.53, render/graph/ms 7.75 |
| warm-up | 4.76 | 40.8 | pipelines/compile_ms 935, pipelines/front_ms 906, pipelines/wait_ms 105, render/thread/ms 63.41, render/busy/ms 61.24, main/busy/ms 53.34, pipelines/driver_ms 29.11, stage/main/post_update 26.36, stage/main/update 19.88, stage/render/prepare_assets 19.45, stage/render/render 16.61, render/graph/ms 14.11 |
| warm-up | 4.80 | 39.7 | pipelines/compile_ms 679, pipelines/front_ms 657, pipelines/wait_ms 118, render/thread/ms 66.32, render/busy/ms 64.09, main/busy/ms 53.90, stage/main/post_update 23.94, pipelines/driver_ms 21.58, stage/main/update 21.49, stage/render/render 18.28, stage/render/prepare_assets 15.63, render/graph/ms 15.19 |
| warm-up | 4.84 | 38.3 | pipelines/compile_ms 420, pipelines/front_ms 409, pipelines/wait_ms 150, main/busy/ms 97.83, stage/main/update 80.52, stream/drive/ms 73.68, render/thread/ms 52.58, render/busy/ms 51.08, stage/render/prepare_assets 27.55, stage/render/render 11.76, pipelines/driver_ms 10.81, render/graph/ms 9.27 |
| warm-up | 4.92 | 81.3 | pipelines/compile_ms 1518, pipelines/front_ms 1478, pipelines/wait_ms 207, main/busy/ms 93.87, stage/main/update 80.64, stream/drive/ms 73.53, render/thread/ms 45.04, render/busy/ms 43.33, pipelines/driver_ms 40.20, stage/render/prepare_assets 24.19, stage/render/render 10.74, render/graph/ms 8.61 |
| warm-up | 4.96 | 33.2 | pipelines/compile_ms 1375, pipelines/front_ms 1337, pipelines/wait_ms 125, render/thread/ms 50.92, render/busy/ms 49.43, pipelines/driver_ms 38.64, main/busy/ms 32.03, stage/render/prepare_assets 13.89, stage/render/render 13.59, stage/main/update 12.40, stage/render/prepare_bind_groups 12.08, stage/main/post_update 10.50 |
| warm-up | 5.00 | 39.8 | pipelines/compile_ms 458, pipelines/front_ms 442, render/thread/ms 66.21, render/busy/ms 64.83, main/busy/ms 53.87, stage/main/update 31.29, pipelines/driver_ms 16.62, stage/render/prepare_flush 14.47, stage/render/render 13.78, stage/render/prepare_bind_groups 12.67, stage/main/post_update 10.40, render/graph/ms 10.03 |
| warm-up | 5.04 | 46.8 | pipelines/compile_ms 380, pipelines/front_ms 367, main/busy/ms 73.61, render/thread/ms 66.82, render/busy/ms 65.40, stage/main/update 55.95, stage/render/prepare_assets 25.58, pipelines/driver_ms 12.97, stream/drive/ms 12.40, stage/render/prepare_flush 11.67, stage/render/render 11.60, render/graph/ms 9.76 |
| warm-up | 5.07 | 28.4 | main/busy/ms 52.82, render/thread/ms 50.33, render/busy/ms 49.05, stage/main/update 35.29, stage/render/prepare_assets 26.32, stream/drive/ms 12.63, stage/render/render 10.18, stage/main/post_update 9.64, render/graph/ms 8.36, encode//sum 6.11, stage/render/graph_submit 5.72, encode/submit 3.88 |
| warm-up | 5.10 | 26.9 | render/thread/ms 42.46, render/busy/ms 40.95, main/busy/ms 28.36, stage/render/render 11.85, stage/main/update 10.97, stage/main/post_update 10.39, render/graph/ms 9.94, stage/render/prepare_assets 9.59, stage/render/graph_submit 7.33, encode//sum 6.14, stage/render/specialize 4.70, stream/drive/ms 4.30 |
| warm-up | 5.13 | 30.6 | pipelines/compile_ms 1421, pipelines/front_ms 1384, render/thread/ms 49.77, render/busy/ms 47.87, main/busy/ms 37.54, pipelines/driver_ms 36.78, pipelines/wait_ms 32.72, stage/main/update 17.93, stage/render/render 14.06, render/graph/ms 11.97, stage/main/post_update 9.70, stage/render/graph_submit 9.18 |
| warm-up | 5.15 | 25.2 | pipelines/compile_ms 1862, pipelines/front_ms 1817, main/busy/ms 53.83, render/thread/ms 48.97, render/busy/ms 45.20, pipelines/driver_ms 45.00, pipelines/wait_ms 33.45, stage/main/update 33.19, stage/render/render 17.92, stream/drive/ms 15.78, render/graph/ms 13.89, stage/render/graph_submit 10.28 |
| warm-up | 5.19 | 39.6 | pipelines/compile_ms 504, pipelines/front_ms 492, render/thread/ms 49.64, main/busy/ms 46.90, render/busy/ms 45.92, stage/main/update 29.84, stage/render/render 20.95, render/graph/ms 16.66, stream/drive/ms 16.48, stage/main/post_update 11.70, pipelines/driver_ms 11.69, stage/render/graph_submit 11.21 |
| warm-up | 5.22 | 30.1 | pipelines/compile_ms 239, pipelines/front_ms 229, render/thread/ms 52.79, render/busy/ms 51.01, main/busy/ms 40.42, stage/main/post_update 17.43, stage/main/update 16.63, stage/render/render 16.42, render/graph/ms 14.20, stage/render/prepare_assets 14.14, encode//sum 11.58, pipelines/driver_ms 10.18 |
| warm-up | 5.26 | 32.7 | pipelines/compile_ms 334, pipelines/front_ms 323, main/busy/ms 49.57, render/thread/ms 48.94, render/busy/ms 47.13, stage/main/update 26.43, stage/render/prepare_assets 19.04, stage/main/post_update 16.24, stage/render/render 13.10, pipelines/driver_ms 11.43, render/graph/ms 10.99, encode//sum 8.43 |
| warm-up | 5.30 | 40.4 | pipelines/compile_ms 179, pipelines/front_ms 173, render/thread/ms 53.79, render/busy/ms 51.84, main/busy/ms 47.44, stage/main/update 27.97, stage/render/prepare_assets 26.59, stage/main/post_update 12.97, stage/render/render 12.33, render/graph/ms 9.91, stream/drive/ms 6.95, stage/render/graph_submit 6.87 |
| warm-up | 5.34 | 39.4 | pipelines/compile_ms 810, pipelines/front_ms 790, render/thread/ms 64.65, render/busy/ms 62.79, main/busy/ms 45.35, stage/render/render 23.56, stage/render/prepare_assets 21.73, render/graph/ms 21.27, pipelines/wait_ms 20.32, pipelines/driver_ms 20.16, stage/main/update 18.96, stage/main/post_update 18.43 |
| warm-up | 5.38 | 38.2 | pipelines/compile_ms 1020, pipelines/front_ms 994, render/thread/ms 62.54, render/busy/ms 60.85, main/busy/ms 44.93, pipelines/driver_ms 25.73, stage/render/render 23.10, stage/render/prepare_assets 22.94, render/graph/ms 21.05, pipelines/wait_ms 20.35, stage/main/update 18.97, stage/main/post_update 15.36 |
| warm-up | 5.42 | 46.1 | pipelines/compile_ms 698, pipelines/front_ms 681, render/thread/ms 66.17, render/busy/ms 64.71, main/busy/ms 39.31, stage/render/prepare_assets 33.03, pipelines/driver_ms 17.53, stage/main/post_update 15.30, stage/main/update 14.34, stage/render/render 12.02, render/graph/ms 9.86, encode//sum 8.30 |
| warm-up | 5.51 | 27.2 | pipelines/compile_ms 252, pipelines/front_ms 243, render/thread/ms 41.83, render/busy/ms 40.34, main/busy/ms 34.52, stage/main/update 18.62, stage/render/render 12.41, stream/drive/ms 11.69, render/graph/ms 10.61, stage/main/post_update 10.56, pipelines/driver_ms 9.34, encode//sum 7.67 |
| warm-up | 5.57 | 25.1 | pipelines/compile_ms 404, pipelines/front_ms 397, render/thread/ms 37.77, render/busy/ms 36.54, main/busy/ms 24.66, stage/render/prepare_assets 18.16, stage/main/update 10.58, stage/main/post_update 9.23, stage/render/render 8.83, render/graph/ms 7.18, pipelines/driver_ms 6.92, encode//sum 6.55 |
| warm-up | 5.65 | 32.5 | pipelines/compile_ms 375, pipelines/front_ms 367, render/thread/ms 50.68, render/busy/ms 49.24, pipelines/wait_ms 42.79, main/busy/ms 33.02, stage/render/prepare_assets 30.95, stage/main/update 16.27, stage/render/render 11.41, stage/main/post_update 10.59, render/graph/ms 9.60, pipelines/driver_ms 8.04 |
| warm-up | 5.69 | 40.7 | pipelines/compile_ms 120, pipelines/front_ms 113, render/thread/ms 63.70, render/busy/ms 62.18, main/busy/ms 39.34, stage/render/prepare_assets 39.34, stage/main/update 22.46, stage/render/render 11.87, render/graph/ms 10.02, stage/main/post_update 9.99, stage/render/graph_submit 7.66, pipelines/driver_ms 6.91 |
| warm-up | 5.74 | 26.1 | pipelines/compile_ms 161, pipelines/front_ms 154, render/thread/ms 46.34, render/busy/ms 44.85, main/busy/ms 29.13, stage/render/prepare_assets 18.52, stage/main/update 13.57, stage/render/render 11.56, render/graph/ms 9.51, stage/main/post_update 9.45, encode//sum 7.16, stage/render/graph_submit 6.74 |
| warm-up | 5.77 | 33.1 | pipelines/compile_ms 161, pipelines/front_ms 154, render/thread/ms 56.20, render/busy/ms 50.56, main/busy/ms 35.24, stage/render/prepare_assets 20.38, stage/render/render 17.83, stage/main/update 14.64, stage/main/post_update 13.73, render/graph/ms 11.41, stage/render/graph_submit 7.98, encode//sum 7.93 |
| warm-up | 5.81 | 43.3 | pipelines/compile_ms 171, pipelines/front_ms 167, render/thread/ms 65.96, render/busy/ms 60.47, main/busy/ms 37.11, stage/render/prepare_assets 34.16, stage/render/render 16.08, stage/main/update 14.89, stage/main/post_update 14.79, render/graph/ms 9.92, encode//sum 7.98, stage/render/graph_submit 6.85 |
| warm-up | 5.85 | 39.9 | pipelines/compile_ms 1291, pipelines/front_ms 1261, render/thread/ms 62.91, render/busy/ms 61.62, stage/render/prepare_assets 40.17, main/busy/ms 31.94, pipelines/driver_ms 29.76, stage/main/update 13.35, stage/main/post_update 11.75, stage/render/render 10.20, render/graph/ms 8.45, encode//sum 7.93 |
| warm-up | 5.93 | 30.9 | pipelines/compile_ms 124, pipelines/front_ms 118, render/thread/ms 46.72, render/busy/ms 45.29, main/busy/ms 36.30, stage/render/prepare_assets 24.52, stage/main/update 18.92, stage/main/post_update 12.26, stage/render/render 11.01, render/graph/ms 9.40, encode//sum 8.15, stage/render/graph_submit 6.83 |
| warm-up | 5.97 | 35.0 | pipelines/compile_ms 124, pipelines/front_ms 118, render/thread/ms 54.28, render/busy/ms 52.93, main/busy/ms 40.15, stage/render/prepare_assets 26.42, stage/main/update 18.42, stage/main/post_update 14.75, stage/render/render 13.73, render/graph/ms 12.01, encode//sum 9.92, stage/render/graph_submit 8.60 |
| warm-up | 6.03 | 51.2 | main/busy/ms 46.16, render/thread/ms 38.31, render/busy/ms 36.97, stage/main/update 23.87, stage/render/prepare_assets 17.70, stage/main/post_update 15.33, stream/drive/ms 15.14, stage/render/render 9.41, pipelines/compile_ms 8.68, pipelines/front_ms 8.08, render/graph/ms 7.69, encode//sum 7.02 |
| warm-up | 6.07 | 35.2 | render/thread/ms 50.20, render/busy/ms 48.87, main/busy/ms 44.09, stage/render/prepare_assets 24.55, stage/main/update 21.36, pipelines/compile_ms 18.54, pipelines/front_ms 17.16, stage/main/post_update 14.85, stage/render/render 10.37, render/graph/ms 8.44, stream/drive/ms 8.13, encode//sum 7.34 |
| warm-up | 6.10 | 31.8 | pipelines/compile_ms 965, pipelines/front_ms 939, render/thread/ms 49.56, render/busy/ms 48.24, main/busy/ms 43.04, pipelines/driver_ms 26.33, stage/render/prepare_assets 22.69, stage/main/update 22.47, stage/render/render 12.73, stage/main/post_update 12.22, render/graph/ms 10.85, stage/render/graph_submit 7.78 |
| warm-up | 6.29 | 50.3 | pipelines/compile_ms 652, pipelines/front_ms 628, main/busy/ms 59.61, stage/main/update 41.02, render/thread/ms 39.83, render/busy/ms 37.91, pipelines/wait_ms 24.93, pipelines/driver_ms 23.20, stream/drive/ms 20.41, stage/render/render 17.78, render/graph/ms 15.31, stage/main/post_update 12.32 |
| warm-up | 6.63 | 27.6 | main/busy/ms 43.70, render/thread/ms 40.85, render/busy/ms 38.94, stage/main/update 20.55, stage/render/render 18.08, stage/main/post_update 16.29, render/graph/ms 15.43, stage/render/prepare_assets 12.61, stage/render/graph_submit 11.38, encode//sum 11.29, stream/drive/ms 11.09, encode/submit 7.29 |
| warm-up | 6.67 | 42.3 | render/thread/ms 49.48, render/busy/ms 48.04, main/busy/ms 44.59, stage/render/prepare_assets 28.83, stage/main/update 22.04, stage/main/post_update 15.68, stage/render/render 12.16, stream/drive/ms 10.86, render/graph/ms 10.12, encode//sum 9.17, stage/render/graph_submit 7.11, encode/submit 3.75 |
| warm-up | 6.99 | 32.1 | render/thread/ms 38.09, render/busy/ms 37.17, main/busy/ms 15.24, stage/render/specialize 9.33, stage/render/render 9.31, render/graph/ms 8.06, stage/main/post_update 7.92, encode//sum 7.66, stage/render/queue 6.59, stage/render/graph_submit 6.16, stage/main/update 4.44, stage/render/prepare_meshes 3.57 |
| warm-up | 8.21 | 29.0 | render/thread/ms 36.80, render/busy/ms 33.73, main/busy/ms 15.26, stage/render/render 10.59, render/graph/ms 9.26, stage/main/post_update 8.77, stage/render/specialize 8.28, stage/render/graph_submit 7.11, encode//sum 7.06, stage/render/queue 4.99, stage/main/update 4.09, stage/render/prepare_meshes 3.73 |
| measure | 0.83 | 26.4 | render/thread/ms 34.30, render/busy/ms 31.06, main/busy/ms 14.16, stage/render/render 9.56, stage/main/post_update 8.55, render/graph/ms 8.27, stage/render/specialize 8.21, encode//sum 6.33, stage/render/graph_submit 6.27, stage/render/queue 5.41, encode/submit 3.65, stage/main/update 3.38 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set mw.matctrl.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=matctrl-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 11%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 97%) · 2625 MHz (min 2625) · memory 8101 MHz · 97.7 W · 78 °C (max 80) · VRAM 6427 MiB · throttled 100% of samples

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
