# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1563 frames over 15.0 s · **median 9.59 ms (104 fps)** · mean 9.60 · p95 10.99 · p99 11.53 (1% low 87 fps) · worst 25.73 ms · 4 spikes over 1.5× median · 16.0 stutters/min (work 4.0/min)
- warm-up: 279 frames over 8.6 s, worst 2590.1 ms
- hitches (> 25.0 ms): **1** measured, 61 in warm-up
- timed GPU passes: 8.40 ms of a 9.60 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.59 ms, limit 11 ms
- pass worst frame 25.73 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.810 | 2.830 | 2.952 | 0.99 | 0.063 |
| dlss_super_resolution | 2.007 | 2.021 | 2.043 | 0.99 | 0.182 |
| early prepass | 0.606 | 0.610 | 0.626 | 0.99 | 0.091 |
| atmosphere_luts | 0.318 | 0.320 | 0.392 | 0.99 | 0.003 |
| bloom | 0.312 | 0.314 | 0.330 | 0.99 | 0.060 |
| main_transmissive_pass_3d | 0.289 | 0.291 | 0.331 | 0.99 | 0.008 |
| water_velocity | 0.238 | 0.239 | 0.264 | 0.99 | 0.001 |
| water_sspr | 0.206 | 0.207 | 0.218 | 0.99 | 0.002 |
| ssao | 0.189 | 0.190 | 0.197 | 0.99 | 0.007 |
| clouds_trace | 0.149 | 0.150 | 0.171 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.143 | 0.144 | 0.152 | 0.99 | 0.001 |
| water_fft | 0.132 | 0.133 | 0.136 | 0.99 | 0.003 |
| uniform_allocation | 0.118 | 0.119 | 0.161 | 0.99 | 0.021 |
| upscaling | 0.112 | 0.112 | 0.118 | 0.99 | 0.004 |
| clouds_composite | 0.112 | 0.112 | 0.114 | 0.99 | 0.002 |
| bin_unpacking | 0.092 | 0.093 | 0.122 | 0.99 | 0.015 |
| tonemapping | 0.085 | 0.086 | 0.092 | 0.99 | 0.003 |
| clustering | 0.066 | 0.067 | 0.079 | 0.99 | 0.020 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.415 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.051 | 0.052 | 0.99 | 0.001 |
| clouds_shadow | 0.031 | 0.032 | 0.035 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.032 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 0.99 | 0.005 |
| lightprobe_radiance_map | 0.028 | 0.221 | 0.229 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 0.99 | 0.079 |
| water_snapshot | 0.024 | 0.025 | 0.026 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.046 | 0.31 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 0.99 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.014 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.008 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.673 | 10.103 | 24.51 | 13556.3 |
| main/busy/ms | 7.116 | 7.940 | 10.06 | 11122.6 |
| render/busy/ms | 6.997 | 8.072 | 23.93 | 10936.0 |
| stage/render/render | 4.738 | 5.720 | 10.91 | 7405.6 |
| stage/main/post_update | 4.188 | 4.889 | 6.36 | 6545.6 |
| render/graph/ms | 4.153 | 5.085 | 10.21 | 6491.3 |
| encode//sum | 3.797 | 5.276 | 7.96 | 5935.0 |
| stage/render/graph_submit | 3.185 | 4.105 | 9.30 | 4977.8 |
| encode/submit | 2.044 | 2.657 | 7.60 | 3195.2 |
| stage/main/update | 1.818 | 2.215 | 3.38 | 2842.0 |
| stage/render/prepare_views | 1.353 | 3.014 | 4.35 | 2115.2 |
| render/acquire/ms | 1.309 | 2.967 | 4.30 | 2046.1 |
| encode/finish_wall | 1.167 | 1.650 | 2.44 | 1824.1 |
| stage/render/camera_driver | 0.914 | 1.043 | 1.44 | 1428.0 |
| encode/pass:shadow_directional_light/sum | 0.744 | 1.285 | 2.12 | 1162.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.722 | 1.005 | 1.44 | 1128.1 |
| encode/pass:early prepass/sum | 0.714 | 1.003 | 1.62 | 1115.9 |
| stage/render/prepare_write_phase_buffers | 0.531 | 0.736 | 1.00 | 830.5 |
| stage/render/prepare_assets | 0.442 | 0.564 | 1.68 | 690.5 |
| bigspace/total/ms | 0.387 | 0.980 | 1.96 | 604.4 |
| stage/render/prepare_resources | 0.382 | 0.438 | 0.95 | 597.5 |
| render/present/ms | 0.367 | 0.433 | 4.27 | 574.2 |
| stage/main/pre_update | 0.338 | 0.450 | 0.92 | 528.5 |
| stage/main/first | 0.293 | 0.355 | 1.60 | 458.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.259 | 0.394 | 0.62 | 404.3 |
| terrain/update/ms | 0.230 | 0.292 | 0.65 | 360.0 |
| stage/render/prepare_meshes | 0.204 | 0.258 | 2.48 | 318.8 |
| stage/render/prepare_bind_groups | 0.201 | 0.275 | 0.43 | 314.2 |
| terrain/select/ms | 0.201 | 0.255 | 0.62 | 314.1 |
| stream/drive/ms | 0.188 | 0.249 | 0.59 | 293.8 |
| stage/main/fixed_main_loop | 0.180 | 0.344 | 0.86 | 280.9 |
| stage/render/prepare_batch_phases | 0.176 | 0.237 | 0.34 | 275.4 |
| stage/render/extract_commands | 0.169 | 0.200 | 0.60 | 264.3 |
| stage/render/create_views | 0.168 | 0.216 | 0.67 | 262.6 |
| controller/move/ms | 0.166 | 0.307 | 0.74 | 260.0 |
| far/select/ms | 0.164 | 0.223 | 0.55 | 256.5 |
| stream/tick/ms | 0.160 | 0.216 | 0.55 | 250.1 |
| stage/render/prepare_flush | 0.149 | 0.269 | 0.84 | 233.3 |
| controller/feed/ms | 0.148 | 0.190 | 0.55 | 231.7 |
| bigspace/high_precision/ms | 0.144 | 0.554 | 1.66 | 225.6 |
| stage/main/last | 0.118 | 0.161 | 2.45 | 184.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.117 | 0.181 | 0.32 | 182.2 |
| stage/render/queue | 0.105 | 0.134 | 5.33 | 164.3 |
| bigspace/low_precision/ms | 0.098 | 0.220 | 1.17 | 152.8 |
| anim/evaluate/ms | 0.085 | 0.193 | 1.08 | 133.1 |
| stage/render/specialize | 0.082 | 0.102 | 8.33 | 127.9 |
| anim/evaluate_cpu/ms | 0.061 | 0.077 | 0.44 | 94.7 |
| stage/render/prepare_collect_phase_buffers | 0.057 | 0.114 | 0.18 | 88.3 |
| encode/resolve_diagnostics/sum | 0.054 | 0.084 | 0.19 | 83.9 |
| physics/step/ms | 0.027 | 0.054 | 0.40 | 41.9 |
| encode/camera_driver_span_begin/sum | 0.025 | 0.038 | 0.17 | 39.6 |
| weather/map/ms | 0.024 | 0.033 | 0.06 | 37.9 |
| stage/render/cleanup | 0.022 | 0.030 | 0.34 | 34.6 |
| encode/camera_driver_span_end/sum | 0.022 | 0.036 | 0.09 | 34.4 |
| stage/render/phase_sort | 0.018 | 0.033 | 0.08 | 28.0 |
| weather/eval/ms | 0.011 | 0.014 | 0.03 | 16.4 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 10.2 |
| controller/avoid/ms | 0.006 | 0.007 | 0.06 | 8.8 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 8.2 |
| anim/apply/ms | 0.005 | 0.006 | 0.02 | 7.1 |
| script/ui/ms | 0.004 | 0.005 | 0.03 | 6.1 |
| controller/write/ms | 0.002 | 0.002 | 0.03 | 2.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.06 | 0.9 |
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
| encode//max | ms | 0.52 | 1.74 | 0.34 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.17 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.09 | 0.02 |
| encode/finish_critical | ms | 0.86 | 2.12 | 1.07 |
| encode/pass:early prepass/max | ms | 0.71 | 1.62 | 0.70 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.73 | 1.44 | 0.56 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.32 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 0.62 | 0.19 |
| encode/pass:shadow_directional_light/max | ms | 0.68 | 2.12 | 1.07 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.19 | 0.02 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.10 | 23.93 | 9.00 |
| gpu/timed/ms | ms | 9.10 | 10.43 | 9.00 |
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
| process/cpu_usage | % | 27.99 | 35.58 | 23.67 |
| process/mem_usage | GiB | 4.67 | 4.73 | 4.73 |
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
| skin/blocks_changed/count | count | 5 | 7 | 5 |
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
| system/cpu_usage | % | 32.05 | 38.84 | 26.38 |
| system/mem_usage | % | 58.05 | 58.22 | 58.22 |
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
| anim/events/count | count | 0.10 | 2 | 153 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.17 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.56 | 29 | 41.5 k |
| encode/bind_walk/init_textures | count | 7166 | 14.0 k | 11.20 M |
| encode/bind_walk/init_textures_skipped | count | 35.6 k | 48.7 k | 55.66 M |
| encode/bind_walk/inits | count | 345 | 437 | 539.8 k |
| encode/bind_walk/inits_skipped | count | 67.00 | 79 | 104.7 k |
| encode/bind_walk/merge_textures | count | 7102 | 13.9 k | 11.10 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.38 M |
| encode/bind_walk/merges | count | 82.26 | 103 | 128.6 k |
| encode/bind_walk/merges_skipped | count | 50.75 | 56 | 79.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1563 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1563 |
| encode/pass:early prepass/count | count | 1 | 1 | 1563 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1563 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1563 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1563 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1563 |
| encode/queue/command_buffers | count | 36.56 | 39 | 57.1 k |
| encode/queue/submits | count | 2 | 2 | 3126 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.86 M | 2.25 G |
| encode/queue/write_buffer/calls | count | 217 | 288 | 339.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 51.02 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 14.1 k |
| encode/queue/write_texture/bytes | bytes | 11.5 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1563 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 1 | 900 |
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

## Hitches (> 25.0 ms; slowest 62, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.59 | 2590.1 | main/busy/ms 2558, stage/main/post_update 2440, stage/main/update 109, surface/advance_cpu_ms 77.61, stage/main/pre_update 5.38, stage/main/last 1.24, stage/main/first 0.74, surface/cell_upload_ms 0.73, stream/drive/ms 0.46, stage/main/fixed_main_loop 0.41, weather/map/ms 0.18, stream/tick/ms 0.08 |
| warm-up | 3.38 | 786.3 | main/busy/ms 2708, stage/main/post_update 2465, render/thread/ms 747, render/busy/ms 552, stage/render/prepare_assets 500, stage/main/update 228, stage/render/prepare_views 189, render/acquire/ms 189, surface/advance_cpu_ms 77.61, stage/render/create_views 28.82, stage/render/render 16.81, render/graph/ms 9.56 |
| warm-up | 3.56 | 185.8 | render/thread/ms 916, render/busy/ms 630, stage/render/prepare_assets 562, stage/render/prepare_views 189, render/acquire/ms 189, main/busy/ms 186, stage/main/update 125, stage/render/render 113, render/present/ms 96.67, stage/main/post_update 45.49, stage/render/create_views 29.27, pipelines/compile_ms 26.86 |
| warm-up | 3.59 | 25.8 | pipelines/compile_ms 380, pipelines/front_ms 258, render/thread/ms 193, pipelines/driver_ms 121, stage/render/render 113, render/busy/ms 100, main/busy/ms 99.33, render/present/ms 91.85, stage/render/prepare_assets 64.15, stage/main/update 54.68, pipelines/wait_ms 34.85, stage/main/post_update 30.31 |
| warm-up | 3.81 | 220.8 | pipelines/wait_ms 4787, pipelines/compile_ms 2766, pipelines/front_ms 2491, pipelines/driver_ms 275, render/thread/ms 176, render/busy/ms 174, stage/render/prepare_assets 138, main/busy/ms 123, stage/main/post_update 53.58, stage/main/update 50.96, stage/render/render 26.25, render/graph/ms 23.46 |
| warm-up | 3.88 | 69.9 | pipelines/wait_ms 5012, pipelines/compile_ms 3033, pipelines/front_ms 2825, render/thread/ms 221, render/busy/ms 219, pipelines/driver_ms 208, encode//sum 164, stage/render/prepare_assets 140, main/busy/ms 86.10, stage/render/render 54.44, stage/main/post_update 52.43, render/graph/ms 51.66 |
| warm-up | 3.98 | 100.1 | pipelines/compile_ms 998, pipelines/front_ms 918, pipelines/wait_ms 267, render/thread/ms 166, render/busy/ms 163, encode//sum 159, stage/render/prepare_assets 91.39, pipelines/driver_ms 79.66, stage/render/render 48.95, render/graph/ms 46.48, main/busy/ms 37.84, stage/render/graph_submit 35.20 |
| warm-up | 4.03 | 33.5 | pipelines/compile_ms 624, pipelines/front_ms 585, pipelines/wait_ms 97.00, pipelines/driver_ms 39.45, main/busy/ms 36.02, render/thread/ms 33.06, render/busy/ms 27.75, stage/main/post_update 17.63, encode//sum 17.22, stage/render/render 15.04, render/graph/ms 12.94, stage/main/update 10.41 |
| warm-up | 4.06 | 30.9 | pipelines/compile_ms 1761, pipelines/front_ms 1681, pipelines/wait_ms 125, pipelines/driver_ms 80.21, render/thread/ms 50.03, render/busy/ms 32.60, main/busy/ms 28.33, encode//sum 17.81, stage/main/post_update 16.82, stage/render/prepare_views 15.94, render/acquire/ms 15.78, stage/render/render 15.55 |
| warm-up | 4.10 | 43.7 | pipelines/compile_ms 1708, pipelines/front_ms 1653, main/busy/ms 71.22, render/thread/ms 70.87, render/busy/ms 56.14, pipelines/driver_ms 54.75, stage/main/update 49.62, stream/drive/ms 45.32, pipelines/wait_ms 39.15, stage/render/prepare_assets 29.94, stage/render/render 19.00, render/graph/ms 16.23 |
| warm-up | 4.16 | 63.8 | pipelines/compile_ms 639, pipelines/front_ms 622, render/thread/ms 83.38, render/busy/ms 81.22, main/busy/ms 64.07, stage/render/prepare_assets 57.54, stage/main/update 49.99, stream/drive/ms 45.41, pipelines/wait_ms 21.06, stage/render/render 18.12, pipelines/driver_ms 17.00, render/graph/ms 15.55 |
| warm-up | 4.23 | 58.0 | pipelines/compile_ms 357, pipelines/front_ms 343, render/thread/ms 43.63, render/busy/ms 41.69, main/busy/ms 38.68, stage/main/update 26.07, stage/render/prepare_assets 23.44, stream/drive/ms 20.88, pipelines/driver_ms 13.77, stage/render/render 12.97, render/graph/ms 10.58, encode//sum 7.50 |
| warm-up | 4.26 | 30.9 | pipelines/compile_ms 357, pipelines/front_ms 343, render/thread/ms 56.83, render/busy/ms 55.17, main/busy/ms 42.97, stage/render/prepare_assets 36.11, stage/main/update 25.25, stream/drive/ms 19.66, pipelines/driver_ms 13.77, stage/render/render 12.77, render/graph/ms 10.33, stage/main/post_update 9.91 |
| warm-up | 4.35 | 87.2 | pipelines/compile_ms 808, pipelines/front_ms 782, render/thread/ms 76.11, render/busy/ms 74.59, stage/render/prepare_assets 55.80, main/busy/ms 55.56, stage/main/update 33.40, pipelines/driver_ms 25.72, stream/drive/ms 20.83, pipelines/wait_ms 13.57, pipelines/lock_ms 13.15, stage/render/render 13.01 |
| warm-up | 4.42 | 65.1 | pipelines/compile_ms 932, pipelines/front_ms 901, main/busy/ms 113, render/thread/ms 96.29, render/busy/ms 94.65, stage/main/update 90.97, stage/render/prepare_assets 75.09, stream/drive/ms 59.73, pipelines/driver_ms 31.14, pipelines/wait_ms 17.08, pipelines/lock_ms 16.35, stage/render/render 11.81 |
| warm-up | 4.54 | 127.8 | pipelines/compile_ms 498, pipelines/front_ms 477, main/busy/ms 124, render/thread/ms 105, render/busy/ms 103, stage/main/update 98.72, stage/render/prepare_assets 76.25, stream/drive/ms 59.62, pipelines/driver_ms 21.31, stage/render/render 13.92, stage/main/post_update 13.66, render/graph/ms 11.46 |
| warm-up | 4.60 | 58.0 | pipelines/compile_ms 1185, pipelines/front_ms 1141, render/thread/ms 107, render/busy/ms 105, stage/render/prepare_assets 74.94, pipelines/wait_ms 69.92, main/busy/ms 69.25, pipelines/driver_ms 43.76, stage/main/update 39.64, stream/drive/ms 14.97, stage/main/post_update 14.63, stage/render/render 14.29 |
| warm-up | 4.69 | 82.2 | pipelines/compile_ms 1001, pipelines/front_ms 966, pipelines/wait_ms 167, render/thread/ms 102, render/busy/ms 98.98, main/busy/ms 71.32, stage/render/prepare_assets 63.90, stage/main/update 39.02, pipelines/driver_ms 34.98, stage/render/render 21.41, render/graph/ms 18.28, stage/main/post_update 16.85 |
| warm-up | 4.75 | 65.4 | pipelines/compile_ms 471, pipelines/front_ms 453, pipelines/wait_ms 168, render/thread/ms 105, render/busy/ms 104, main/busy/ms 79.72, stage/render/prepare_assets 56.52, stage/main/update 35.55, stage/main/post_update 34.59, stage/render/render 20.90, render/graph/ms 19.01, pipelines/driver_ms 17.84 |
| warm-up | 4.79 | 42.4 | pipelines/compile_ms 1425, pipelines/front_ms 1361, pipelines/wait_ms 95.45, render/thread/ms 84.13, render/busy/ms 81.12, pipelines/driver_ms 63.92, main/busy/ms 52.92, stage/render/prepare_assets 32.46, stage/main/post_update 27.84, stage/render/render 23.71, render/graph/ms 20.32, stage/main/update 18.85 |
| warm-up | 4.84 | 26.2 | pipelines/compile_ms 851, pipelines/front_ms 819, render/thread/ms 41.45, render/busy/ms 37.93, pipelines/driver_ms 31.82, main/busy/ms 30.45, stage/render/render 15.45, stage/main/update 12.03, render/graph/ms 11.31, stage/main/post_update 10.26, encode//sum 7.36, stage/render/graph_submit 7.30 |
| warm-up | 4.88 | 35.3 | pipelines/compile_ms 1129, pipelines/front_ms 1090, main/busy/ms 113, stage/main/update 99.02, stream/drive/ms 91.17, pipelines/wait_ms 70.58, render/thread/ms 42.44, render/busy/ms 39.50, pipelines/driver_ms 38.91, stage/render/prepare_assets 14.37, stage/render/render 11.30, render/graph/ms 7.73 |
| warm-up | 4.99 | 113.1 | pipelines/compile_ms 1589, pipelines/front_ms 1542, main/busy/ms 110, stage/main/update 97.48, stream/drive/ms 90.30, pipelines/wait_ms 69.82, pipelines/driver_ms 46.39, render/thread/ms 44.73, render/busy/ms 43.32, stage/render/prepare_assets 22.87, stage/render/render 11.22, render/graph/ms 9.07 |
| warm-up | 5.01 | 25.9 | pipelines/compile_ms 1310, pipelines/front_ms 1265, render/thread/ms 48.72, render/busy/ms 47.22, main/busy/ms 45.61, pipelines/driver_ms 45.29, stage/main/update 24.01, stage/render/render 23.03, render/graph/ms 20.97, stage/render/graph_submit 17.45, stage/render/prepare_assets 14.48, stage/main/post_update 13.03 |
| warm-up | 5.06 | 42.2 | pipelines/compile_ms 1179, pipelines/front_ms 1136, main/busy/ms 51.16, render/thread/ms 50.04, render/busy/ms 48.50, pipelines/driver_ms 43.03, stage/main/update 24.61, stage/render/render 21.86, render/graph/ms 20.02, stage/render/graph_submit 16.36, pipelines/wait_ms 15.77, stage/main/post_update 14.56 |
| warm-up | 5.14 | 70.4 | pipelines/compile_ms 246, pipelines/front_ms 237, main/busy/ms 70.97, stage/main/update 52.57, render/thread/ms 45.80, render/busy/ms 43.84, stage/render/prepare_assets 24.36, stream/drive/ms 13.86, stage/render/render 11.45, stage/main/post_update 10.24, render/graph/ms 9.49, pipelines/driver_ms 8.81 |
| warm-up | 5.18 | 37.6 | pipelines/compile_ms 607, pipelines/front_ms 593, render/thread/ms 59.08, render/busy/ms 57.74, main/busy/ms 46.27, stage/render/prepare_assets 35.69, stage/main/update 30.56, pipelines/driver_ms 13.61, stage/render/render 10.51, render/graph/ms 8.65, stage/main/post_update 8.36, stage/render/graph_submit 6.31 |
| warm-up | 5.27 | 50.6 | pipelines/compile_ms 647, pipelines/front_ms 630, main/busy/ms 51.98, render/thread/ms 43.22, pipelines/wait_ms 41.99, render/busy/ms 41.69, stage/main/update 35.28, stream/drive/ms 19.39, pipelines/driver_ms 16.92, stage/render/render 15.88, render/graph/ms 13.47, stage/main/post_update 11.36 |
| warm-up | 5.30 | 30.3 | pipelines/compile_ms 276, pipelines/front_ms 266, render/thread/ms 52.58, render/busy/ms 51.00, main/busy/ms 40.35, stage/render/prepare_assets 20.60, stage/main/update 18.12, stage/main/post_update 15.85, stage/render/render 13.91, render/graph/ms 11.40, pipelines/driver_ms 9.34, encode//sum 8.65 |
| warm-up | 5.33 | 33.1 | pipelines/compile_ms 350, pipelines/front_ms 334, render/thread/ms 47.49, render/busy/ms 45.90, main/busy/ms 39.51, stage/render/prepare_assets 24.74, stage/main/update 20.26, pipelines/driver_ms 15.25, stage/main/post_update 12.65, stage/render/render 9.56, render/graph/ms 7.45, encode//sum 6.61 |
| warm-up | 5.37 | 39.8 | pipelines/compile_ms 324, pipelines/front_ms 310, render/thread/ms 56.18, render/busy/ms 54.57, main/busy/ms 40.98, stage/render/prepare_assets 26.37, stage/main/update 22.39, pipelines/driver_ms 13.40, stage/main/post_update 12.21, stage/render/render 10.46, render/graph/ms 8.34, stream/drive/ms 7.16 |
| warm-up | 5.41 | 34.2 | pipelines/compile_ms 615, pipelines/front_ms 599, render/thread/ms 60.50, render/busy/ms 58.93, main/busy/ms 43.71, stage/render/prepare_assets 22.98, stage/main/update 19.79, stage/main/post_update 16.86, pipelines/driver_ms 16.14, stage/render/render 12.98, render/graph/ms 10.73, stream/drive/ms 7.35 |
| warm-up | 5.44 | 36.5 | pipelines/compile_ms 817, pipelines/front_ms 795, render/thread/ms 55.35, render/busy/ms 53.60, main/busy/ms 41.75, stage/render/prepare_assets 25.49, pipelines/driver_ms 22.07, stage/main/post_update 17.77, stage/main/update 17.18, stage/render/render 12.20, render/graph/ms 10.01, encode//sum 7.35 |
| warm-up | 5.48 | 33.7 | pipelines/compile_ms 663, pipelines/front_ms 644, render/thread/ms 54.63, render/busy/ms 52.91, main/busy/ms 32.01, stage/render/prepare_assets 25.93, pipelines/driver_ms 18.45, stage/main/update 13.29, stage/main/post_update 12.93, stage/render/render 11.19, render/graph/ms 9.30, encode//sum 7.85 |
| warm-up | 5.50 | 26.2 | pipelines/compile_ms 684, pipelines/front_ms 666, render/thread/ms 51.68, render/busy/ms 50.20, main/busy/ms 29.97, pipelines/driver_ms 17.29, stage/render/render 14.64, render/graph/ms 12.76, stage/main/post_update 12.11, stage/render/prepare_assets 11.58, stage/main/update 10.37, stage/render/prepare_resources 9.82 |
| warm-up | 5.53 | 29.9 | pipelines/compile_ms 708, pipelines/front_ms 690, render/thread/ms 53.60, render/busy/ms 52.23, main/busy/ms 34.33, stage/render/prepare_resources 17.74, pipelines/driver_ms 17.56, stage/render/render 14.73, stage/main/post_update 13.23, render/graph/ms 12.77, stage/main/update 11.08, stage/render/graph_submit 9.80 |
| warm-up | 5.56 | 26.1 | pipelines/compile_ms 478, pipelines/front_ms 465, render/thread/ms 53.28, render/busy/ms 51.93, main/busy/ms 39.85, stage/main/update 20.72, stage/render/prepare_resources 14.06, pipelines/driver_ms 13.53, stage/render/render 13.47, stream/drive/ms 11.96, render/graph/ms 11.53, stage/main/post_update 11.47 |
| warm-up | 5.58 | 26.5 | pipelines/compile_ms 249, pipelines/front_ms 243, render/thread/ms 48.87, render/busy/ms 46.65, main/busy/ms 39.96, stage/main/update 19.35, stage/main/post_update 15.69, stage/render/prepare_assets 13.92, stage/render/render 13.04, stream/drive/ms 11.69, render/graph/ms 10.50, encode//sum 8.49 |
| warm-up | 5.63 | 29.6 | pipelines/compile_ms 1210, pipelines/front_ms 1179, render/thread/ms 39.14, render/busy/ms 37.78, pipelines/driver_ms 30.59, main/busy/ms 26.38, pipelines/wait_ms 17.29, stage/render/prepare_assets 16.29, stage/render/render 11.10, stage/main/update 10.50, stage/main/post_update 10.24, render/graph/ms 9.30 |
| warm-up | 5.67 | 27.3 | pipelines/compile_ms 379, pipelines/front_ms 371, pipelines/wait_ms 51.22, render/thread/ms 38.03, render/busy/ms 36.65, main/busy/ms 31.29, stage/render/prepare_assets 14.22, stage/main/update 12.74, stage/main/post_update 12.63, stage/render/render 11.63, render/graph/ms 9.65, encode//sum 8.21 |
| warm-up | 5.70 | 26.4 | pipelines/compile_ms 371, pipelines/front_ms 364, render/thread/ms 47.92, render/busy/ms 46.57, pipelines/wait_ms 36.28, main/busy/ms 33.41, stage/render/prepare_assets 25.10, stage/main/update 16.09, stage/main/post_update 11.36, stage/render/render 11.10, render/graph/ms 9.09, encode//sum 7.32 |
| warm-up | 5.75 | 46.0 | pipelines/compile_ms 313, pipelines/front_ms 303, render/thread/ms 63.13, render/busy/ms 61.84, stage/render/prepare_assets 36.89, main/busy/ms 35.27, stage/main/update 20.04, stage/render/render 10.49, pipelines/driver_ms 10.40, stage/main/post_update 9.86, render/graph/ms 8.73, pipelines/wait_ms 8.37 |
| warm-up | 5.80 | 38.6 | render/thread/ms 49.24, render/busy/ms 47.51, main/busy/ms 33.83, stage/render/prepare_assets 21.53, stage/render/render 14.06, stage/main/post_update 13.28, stage/main/update 12.67, render/graph/ms 11.77, stage/render/graph_submit 8.55, encode//sum 8.08, encode/submit 6.62, stream/drive/ms 5.22 |
| warm-up | 5.84 | 34.2 | pipelines/compile_ms 170, pipelines/front_ms 166, render/thread/ms 56.43, render/busy/ms 55.18, main/busy/ms 34.48, stage/render/prepare_assets 32.86, stage/main/update 14.37, stage/main/post_update 11.90, stage/render/render 9.95, render/graph/ms 8.12, encode//sum 7.49, stage/render/graph_submit 5.63 |
| warm-up | 5.88 | 39.9 | pipelines/compile_ms 704, pipelines/front_ms 690, render/thread/ms 58.98, render/busy/ms 57.85, stage/render/prepare_assets 38.20, main/busy/ms 31.81, pipelines/driver_ms 14.44, stage/main/update 14.38, stage/main/post_update 10.97, stage/render/render 9.09, render/graph/ms 7.48, encode//sum 6.93 |
| warm-up | 5.93 | 28.3 | pipelines/compile_ms 570, pipelines/front_ms 559, render/thread/ms 42.16, render/busy/ms 40.93, main/busy/ms 27.08, stage/render/prepare_assets 24.72, stage/main/update 11.88, pipelines/driver_ms 10.69, stage/render/render 9.66, stage/main/post_update 9.31, render/graph/ms 8.09, encode//sum 6.75 |
| warm-up | 5.96 | 31.2 | pipelines/compile_ms 124, pipelines/front_ms 118, render/thread/ms 52.57, render/busy/ms 50.93, main/busy/ms 44.04, stage/main/update 27.08, stage/render/prepare_assets 21.19, stage/render/render 15.85, render/graph/ms 13.92, stage/main/post_update 11.63, stage/render/graph_submit 10.22, encode//sum 8.42 |
| warm-up | 6.00 | 39.4 | pipelines/compile_ms 124, pipelines/front_ms 118, render/thread/ms 56.55, render/busy/ms 55.04, main/busy/ms 49.09, stage/main/update 27.09, stage/render/prepare_assets 20.07, stage/render/render 15.44, stage/main/post_update 15.27, render/graph/ms 13.26, stage/render/graph_submit 9.17, encode//sum 9.05 |
| warm-up | 6.03 | 28.9 | render/thread/ms 53.54, render/busy/ms 51.81, main/busy/ms 44.06, stage/main/post_update 20.14, stage/render/prepare_assets 18.37, stage/render/render 17.53, stage/main/update 15.73, render/graph/ms 14.90, encode//sum 11.22, stage/render/graph_submit 10.78, encode/submit 7.70, stage/render/prepare_write_phase_buffers 5.12 |
| warm-up | 6.06 | 35.7 | main/busy/ms 63.45, render/thread/ms 49.92, render/busy/ms 47.87, stage/main/update 32.36, stage/main/post_update 22.55, stage/render/render 19.09, stage/render/prepare_assets 16.50, render/graph/ms 16.30, stream/drive/ms 14.64, pipelines/compile_ms 12.91, pipelines/front_ms 12.33, encode//sum 12.24 |
| warm-up | 6.12 | 55.0 | pipelines/compile_ms 204, pipelines/front_ms 197, main/busy/ms 58.50, render/thread/ms 52.03, render/busy/ms 50.52, stage/main/update 31.86, stage/render/prepare_assets 27.02, stage/main/post_update 18.34, stream/drive/ms 14.48, stage/render/render 12.12, render/graph/ms 9.92, encode//sum 9.06 |
| warm-up | 6.14 | 25.1 | pipelines/compile_ms 992, pipelines/front_ms 968, render/thread/ms 48.18, render/busy/ms 46.81, main/busy/ms 41.06, pipelines/driver_ms 24.35, stage/render/prepare_assets 21.56, stage/main/update 18.68, stage/main/post_update 13.47, stage/render/render 12.53, render/graph/ms 10.68, stream/drive/ms 8.37 |
| warm-up | 6.17 | 25.1 | pipelines/compile_ms 865, pipelines/front_ms 845, render/thread/ms 40.40, render/busy/ms 39.01, main/busy/ms 38.66, pipelines/driver_ms 20.10, stage/main/update 18.94, stage/render/render 13.18, stage/main/post_update 12.24, stage/render/prepare_assets 12.15, render/graph/ms 11.44, stream/drive/ms 8.33 |
| warm-up | 6.31 | 27.6 | pipelines/compile_ms 130, pipelines/front_ms 125, main/busy/ms 52.13, render/thread/ms 39.63, render/busy/ms 37.84, stage/main/update 29.11, stream/drive/ms 21.45, stage/main/post_update 15.94, stage/render/render 15.37, render/graph/ms 13.20, stage/render/graph_submit 9.94, stage/render/prepare_resources 8.96 |
| warm-up | 6.34 | 32.8 | pipelines/compile_ms 792, pipelines/front_ms 771, main/busy/ms 48.90, render/thread/ms 38.84, render/busy/ms 37.29, stage/main/update 30.27, stream/drive/ms 21.31, pipelines/driver_ms 20.90, stage/render/render 13.29, stage/main/post_update 12.53, render/graph/ms 11.30, stage/render/prepare_resources 8.69 |
| warm-up | 6.42 | 29.7 | main/busy/ms 38.94, render/thread/ms 34.53, render/busy/ms 33.17, stage/main/update 19.39, stage/render/render 13.71, stage/main/post_update 13.20, stage/render/prepare_assets 12.01, render/graph/ms 11.56, encode//sum 10.45, stage/render/graph_submit 8.44, encode/submit 5.32, stream/drive/ms 3.53 |
| warm-up | 6.66 | 32.6 | main/busy/ms 36.84, render/thread/ms 36.48, render/busy/ms 34.62, stage/render/render 17.81, stage/main/update 17.61, render/graph/ms 15.04, stage/main/post_update 14.98, encode//sum 12.21, stage/render/graph_submit 10.58, stage/render/prepare_assets 9.09, encode/submit 5.97, encode/finish_wall 4.69 |
| warm-up | 6.69 | 28.4 | render/thread/ms 44.05, main/busy/ms 42.80, render/busy/ms 42.57, stage/main/update 21.30, stage/render/prepare_assets 18.06, stage/main/post_update 15.22, stage/render/render 14.00, stream/drive/ms 11.69, render/graph/ms 11.55, encode//sum 10.66, stage/render/graph_submit 7.55, encode/finish_wall 3.93 |
| warm-up | 6.73 | 42.4 | render/thread/ms 48.08, render/busy/ms 47.13, main/busy/ms 39.95, stage/render/prepare_assets 30.94, stage/main/update 21.03, stage/main/post_update 12.77, stream/drive/ms 11.64, stage/render/render 8.23, render/graph/ms 6.73, encode//sum 6.72, stage/render/graph_submit 4.09, stage/render/camera_driver 2.50 |
| warm-up | 7.06 | 29.3 | render/thread/ms 35.79, render/busy/ms 34.17, main/busy/ms 17.45, stage/main/post_update 10.43, stage/render/render 9.82, stage/render/specialize 8.97, render/graph/ms 8.47, stage/render/graph_submit 6.64, encode//sum 6.38, stage/render/queue 6.31, stage/main/update 4.13, encode/submit 4.09 |
| warm-up | 8.27 | 29.8 | render/thread/ms 36.16, render/busy/ms 33.47, main/busy/ms 13.93, stage/render/render 9.95, render/graph/ms 8.72, stage/render/specialize 8.30, stage/main/post_update 8.13, stage/render/graph_submit 6.57, encode//sum 6.14, stage/render/queue 5.91, stage/render/prepare_meshes 3.98, encode/submit 3.90 |
| measure | 0.83 | 25.7 | render/thread/ms 33.61, render/busy/ms 30.01, main/busy/ms 13.42, stage/render/render 8.94, stage/render/specialize 8.38, stage/main/post_update 7.85, render/graph/ms 7.67, stage/render/graph_submit 5.87, encode//sum 5.71, stage/render/queue 5.45, encode/submit 3.48, stage/main/update 3.20 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=base-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 3%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 97%) · 2624 MHz (min 2610) · memory 8101 MHz · 98.5 W · 77 °C (max 80) · VRAM 6424 MiB · throttled 100% of samples

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
