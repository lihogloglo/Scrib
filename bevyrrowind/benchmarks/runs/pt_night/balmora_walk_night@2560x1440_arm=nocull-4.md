# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5873 frames over 65.6 s · **median 10.43 ms (96 fps)** · mean 11.18 · p95 15.31 · p99 17.05 (1% low 59 fps) · worst 28.25 ms · 217 spikes over 1.5× median · 26.5 stutters/min (work 32.0/min)
- warm-up: 324 frames over 9.2 s, worst 2469.1 ms
- hitches (> 26.1 ms): **5** measured, 56 in warm-up
- timed GPU passes: 6.70 ms of a 11.18 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.43 ms, limit 11 ms
- **FAIL** frame_ms: 29 stutters (> 1.5x moving median of 60) in 1.09 min = 26.50/min, limit 1/min
- pass worst frame 28.25 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.003 | 2.030 | 2.063 | 0.99 | 0.175 |
| main_opaque_pass_3d | 1.276 | 1.293 | 1.634 | 0.99 | 0.065 |
| early prepass | 0.482 | 0.489 | 0.698 | 0.99 | 0.087 |
| bloom | 0.312 | 0.316 | 0.322 | 0.99 | 0.064 |
| atmosphere_luts | 0.306 | 0.310 | 0.373 | 0.99 | 0.003 |
| water_velocity | 0.258 | 0.262 | 0.350 | 0.99 | 0.002 |
| auxiliary_views | 0.254 | 0.258 | 1.205 | 0.99 | 0.239 |
| water_sspr | 0.206 | 0.209 | 0.210 | 0.99 | 0.002 |
| ssao | 0.186 | 0.188 | 0.193 | 0.99 | 0.007 |
| uniform_allocation | 0.146 | 0.148 | 0.176 | 0.99 | 0.026 |
| upscaling | 0.128 | 0.130 | 0.137 | 0.99 | 0.004 |
| water_fft | 0.127 | 0.129 | 0.137 | 0.99 | 0.003 |
| atmos_froxel_inject | 0.126 | 0.127 | 0.128 | 0.99 | 0.001 |
| clouds_composite | 0.103 | 0.105 | 0.108 | 0.99 | 0.002 |
| bin_unpacking | 0.103 | 0.105 | 0.128 | 0.99 | 0.019 |
| tonemapping | 0.086 | 0.087 | 0.091 | 0.99 | 0.004 |
| main_transmissive_pass_3d | 0.077 | 0.078 | 0.157 | 0.99 | 0.009 |
| clustering | 0.062 | 0.063 | 0.067 | 0.99 | 0.019 |
| clouds_trace | 0.055 | 0.056 | 0.073 | 0.99 | 0.001 |
| render_sky | 0.051 | 0.051 | 0.052 | 0.99 | 0.002 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 0.99 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.412 | 0.416 | 0.12 | 0.001 |
| surface_trail | 0.040 | 0.056 | 0.057 | 0.71 | 0.002 |
| early_mesh_preprocessing | 0.036 | 0.036 | 0.045 | 0.99 | 0.004 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.219 | 0.230 | 0.12 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.028 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.034 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.014 | 0.014 | 0.028 | 0.99 | 0.176 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.99 | 0.005 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.99 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.795 | 13.760 | 26.92 | 57528.5 |
| render/busy/ms | 9.283 | 13.230 | 25.89 | 54521.8 |
| main/busy/ms | 8.781 | 10.984 | 25.75 | 51568.9 |
| stage/render/render | 5.786 | 8.896 | 17.45 | 33984.0 |
| stage/main/post_update | 5.331 | 7.134 | 12.79 | 31308.1 |
| render/graph/ms | 5.062 | 8.092 | 16.07 | 29726.4 |
| encode//sum | 4.647 | 8.706 | 32.16 | 27289.6 |
| stage/render/graph_submit | 3.713 | 5.837 | 11.54 | 21808.7 |
| encode/submit | 2.344 | 3.846 | 6.54 | 13767.7 |
| stage/main/update | 2.172 | 2.628 | 19.49 | 12759.1 |
| encode/finish_wall | 1.404 | 2.075 | 5.54 | 8246.4 |
| stage/render/camera_driver | 1.293 | 2.234 | 4.67 | 7595.2 |
| encode/pass:shadow_point_light/sum | 1.248 | 5.351 | 16.38 | 7328.2 |
| encode/pass:shadow_directional_light/sum | 0.766 | 1.253 | 2.30 | 4499.4 |
| stage/render/prepare_write_phase_buffers | 0.762 | 1.164 | 2.81 | 4475.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.747 | 1.085 | 1.57 | 4387.1 |
| encode/pass:early prepass/sum | 0.729 | 1.068 | 5.46 | 4282.7 |
| stage/render/prepare_assets | 0.587 | 0.763 | 9.75 | 3447.9 |
| bigspace/total/ms | 0.523 | 1.189 | 2.88 | 3074.5 |
| stage/render/prepare_meshes | 0.489 | 0.655 | 2.68 | 2873.6 |
| stage/render/prepare_resources | 0.451 | 0.544 | 1.75 | 2650.3 |
| render/present/ms | 0.417 | 0.511 | 1.97 | 2450.7 |
| stage/main/first | 0.394 | 0.522 | 6.08 | 2316.1 |
| stage/main/pre_update | 0.363 | 0.512 | 1.51 | 2129.3 |
| terrain/update/ms | 0.311 | 0.407 | 2.09 | 1829.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.302 | 0.448 | 1.35 | 1771.2 |
| stage/render/specialize | 0.288 | 0.442 | 4.50 | 1689.4 |
| terrain/select/ms | 0.278 | 0.365 | 0.95 | 1632.2 |
| stage/render/prepare_bind_groups | 0.273 | 0.489 | 2.40 | 1600.5 |
| stage/render/queue | 0.269 | 0.431 | 2.74 | 1580.9 |
| stage/render/prepare_batch_phases | 0.266 | 0.374 | 0.76 | 1562.5 |
| controller/feed/ms | 0.254 | 0.318 | 0.95 | 1490.3 |
| stream/drive/ms | 0.242 | 0.317 | 17.38 | 1420.3 |
| stage/main/fixed_main_loop | 0.223 | 0.372 | 1.11 | 1307.0 |
| bigspace/low_precision/ms | 0.212 | 0.592 | 2.27 | 1242.3 |
| far/select/ms | 0.202 | 0.271 | 0.62 | 1186.9 |
| stage/render/create_views | 0.194 | 0.252 | 1.09 | 1140.5 |
| stream/tick/ms | 0.186 | 0.248 | 0.94 | 1093.6 |
| stage/render/prepare_flush | 0.171 | 0.334 | 1.00 | 1004.6 |
| stage/render/extract_commands | 0.170 | 0.201 | 0.49 | 997.4 |
| anim/evaluate/ms | 0.157 | 0.264 | 1.73 | 921.5 |
| anim/evaluate_cpu/ms | 0.153 | 0.194 | 0.70 | 898.6 |
| bigspace/high_precision/ms | 0.146 | 0.589 | 2.42 | 858.3 |
| stage/render/prepare_views | 0.138 | 0.171 | 0.42 | 809.5 |
| controller/move/ms | 0.126 | 0.211 | 0.45 | 741.0 |
| stage/main/last | 0.117 | 0.161 | 1.53 | 687.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.102 | 0.172 | 0.83 | 600.9 |
| render/acquire/ms | 0.095 | 0.121 | 0.34 | 556.0 |
| stage/render/cleanup | 0.084 | 0.205 | 0.75 | 491.1 |
| stage/render/prepare_collect_phase_buffers | 0.064 | 0.137 | 0.52 | 375.3 |
| stage/render/phase_sort | 0.064 | 0.092 | 0.42 | 375.0 |
| physics/step/ms | 0.049 | 0.087 | 0.72 | 288.9 |
| physics/insert/ms | 0.037 | 0.385 | 0.92 | 218.2 |
| encode/resolve_diagnostics/sum | 0.030 | 0.040 | 0.53 | 174.4 |
| pipelines/compile_ms | 0.016 | 0.000 | 81.75 | 93.4 |
| weather/map/ms | 0.016 | 0.021 | 0.08 | 92.8 |
| pipelines/front_ms | 0.015 | 0.000 | 79.01 | 90.4 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.022 | 0.22 | 83.9 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.46 | 66.1 |
| anim/apply/ms | 0.011 | 0.021 | 0.41 | 65.1 |
| controller/avoid/ms | 0.011 | 0.014 | 0.09 | 61.7 |
| weather/eval/ms | 0.010 | 0.013 | 0.18 | 57.3 |
| script/update/ms | 0.007 | 0.009 | 0.27 | 40.4 |
| script/input/ms | 0.005 | 0.007 | 0.15 | 31.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.11 | 30.6 |
| script/ui/ms | 0.005 | 0.006 | 0.33 | 30.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.25 | 28.3 |
| controller/write/ms | 0.004 | 0.005 | 0.05 | 20.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.61 | 13.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.8 |
| stage/render/prepare | 0.001 | 0.003 | 0.24 | 3.3 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.75 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 2 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 11 |
| crowd/c0/joints/count | count | 1247 | 2262 | 492 |
| crowd/c0/meshes/count | count | 343 | 646 | 113 |
| crowd/c1/count | count | 147 | 158 | 150 |
| crowd/c1/joints/count | count | 5584 | 6045 | 5704 |
| crowd/c1/meshes/count | count | 1203 | 1313 | 1228 |
| crowd/c2/count | count | 399 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 164 | 216 | 204 |
| crowd/joints_extracted/count | count | 3685 | 7862 | 5731 |
| crowd/joints_uploaded/count | count | 3642 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.43 | 5.45 | 0.33 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.22 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.46 | 0.02 |
| encode/finish_critical | ms | 0.95 | 5.46 | 1.07 |
| encode/pass:early prepass/max | ms | 0.75 | 5.46 | 1.07 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 1.57 | 0.53 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.83 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.29 | 1.35 | 0.25 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 2.30 | 1.00 |
| encode/pass:shadow_point_light/max | ms | 0.79 | 1.61 | 0.88 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.53 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 8.86 | 25.89 | 7.68 |
| gpu/timed/ms | ms | 7.48 | 10.79 | 7.68 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 254 | 479 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3958 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 254 | 479 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3958 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 4122 | 4335 | 4335 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 4122 | 4335 | 4335 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 18 | 211 | 5 |
| phases/shadow/bins | count | 10.4 k | 11.9 k | 7980 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 27.2 k | 29.0 k | 25.7 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 198 | 233 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 189 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 17 | 592 | 6 |
| preprocess/bind_groups_created | count | 17 | 592 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 136 | 282 | 129 |
| process/cpu_usage | % | 25.71 | 51.18 | 22.06 |
| process/mem_usage | GiB | 5.47 | 5.63 | 5.63 |
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
| skin/blocks_changed/count | count | 13 | 92 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 163 | 216 | 204 |
| skin/joint_bytes_written/bytes | bytes | 282.6 k | 808.1 k | 448.5 k |
| skin/joints_written/count | count | 4416 | 12.6 k | 7008 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4920 | 5010 | 5010 |
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
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 327 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.38 | 61.28 | 31.66 |
| system/mem_usage | % | 54.48 | 55.06 | 55.00 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 70 | 264 | 57 |
| views/all/bins | count | 19.0 k | 20.5 k | 17.6 k |
| views/all/count | count | 45 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 55.0 k | 57.5 k | 55.2 k |
| views/all/pipelines | count | 245 | 277 | 68 |
| views/all/pipelines_distinct | count | 71 | 76 | 63 |
| views/all/work_items | count | 41.5 k | 56.1 k | 42.2 k |
| views/cascade/batch_sets | count | 11 | 28 | 5 |
| views/cascade/bins | count | 7663 | 7994 | 7980 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 24.5 k | 25.7 k | 25.7 k |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 11.7 k | 25.7 k | 11.9 k |
| views/lamp_face/batch_sets | count | 0 | 188 | 0 |
| views/lamp_face/bins | count | 3414 | 4505 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3629 | 4832 | 0 |
| views/lamp_face/pipelines | count | 173 | 207 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 4033 | 0 |
| views/main/batch_sets | count | 52 | 55 | 52 |
| views/main/bins | count | 8730 | 9612 | 9612 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.1 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 48 | 51 | 48 |
| views/main/work_items | count | 28.6 k | 30.4 k | 30.3 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.6 k | 37.9 k | 30.8 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 50.50 | 252 | 296.6 k |
| anim/events/count | count | 0.10 | 3 | 577 |
| assets/Image/added | count | 0.10 | 28 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.69 | 152 | 4061 |
| assets/Mesh/removed | count | 0.82 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 29 | 457 |
| assets/StandardMaterial/modified | count | 3.69 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.80 | 1328 | 4675 |
| encode//count | count | 37.77 | 219 | 221.8 k |
| encode/bind_walk/init_textures | count | 14.8 k | 115.1 k | 87.08 M |
| encode/bind_walk/init_textures_skipped | count | 57.6 k | 123.2 k | 338.24 M |
| encode/bind_walk/inits | count | 531 | 1246 | 3.12 M |
| encode/bind_walk/inits_skipped | count | 127 | 189 | 747.6 k |
| encode/bind_walk/merge_textures | count | 14.8 k | 115.1 k | 86.71 M |
| encode/bind_walk/merge_textures_skipped | count | 5536 | 6026 | 32.51 M |
| encode/bind_walk/merges | count | 107 | 347 | 626.4 k |
| encode/bind_walk/merges_skipped | count | 106 | 151 | 623.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5873 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5873 |
| encode/pass:early prepass/count | count | 1 | 1 | 5873 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5873 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5873 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5873 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5873 |
| encode/pass:shadow_point_light/count | count | 2.70 | 48 | 15.9 k |
| encode/queue/command_buffers | count | 50.49 | 277 | 296.5 k |
| encode/queue/submits | count | 2.02 | 4 | 11.8 k |
| encode/queue/write_buffer/bytes | bytes | 2.26 M | 4.52 M | 13.26 G |
| encode/queue/write_buffer/calls | count | 272 | 884 | 1.60 M |
| encode/queue/write_buffer_with/bytes | bytes | 170.3 k | 3.52 M | 999.92 M |
| encode/queue/write_buffer_with/calls | count | 11.06 | 465 | 64.9 k |
| encode/queue/write_texture/bytes | bytes | 10.6 k | 8.39 M | 62.48 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 562 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5873 |
| physics/inserts | count | 0.39 | 111 | 2280 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.67 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 2 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5764 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1264 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.1 ms; slowest 61, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2469.1 | main/busy/ms 2435, stage/main/post_update 2394, stage/main/update 32.18, stage/main/pre_update 5.39, stage/main/last 1.16, surface/advance_cpu_ms 0.73, stage/main/first 0.55, stream/drive/ms 0.46, stage/main/fixed_main_loop 0.34, surface/cell_upload_ms 0.31, stream/tick/ms 0.10, bigspace/total/ms 0.07 |
| warm-up | 3.30 | 826.3 | main/busy/ms 2608, stage/main/post_update 2418, render/thread/ms 792, render/busy/ms 653, stage/render/prepare_assets 604, stage/main/update 175, stage/render/prepare_views 136, render/acquire/ms 136, stage/render/create_views 28.75, stage/render/render 12.28, render/graph/ms 7.81, stage/main/pre_update 7.35 |
| warm-up | 3.40 | 104.9 | render/thread/ms 880, render/busy/ms 692, stage/render/prepare_assets 630, main/busy/ms 189, stage/main/update 147, stage/render/prepare_views 137, render/acquire/ms 136, pipelines/compile_ms 73.88, stage/render/render 65.86, render/present/ms 51.78, pipelines/driver_ms 42.22, stage/main/post_update 32.48 |
| warm-up | 3.44 | 42.1 | pipelines/compile_ms 429, pipelines/front_ms 330, pipelines/wait_ms 202, render/thread/ms 128, pipelines/driver_ms 98.62, stage/render/render 88.28, render/busy/ms 66.75, render/present/ms 61.00, main/busy/ms 44.92, stage/main/post_update 30.22, render/graph/ms 26.08, stage/render/prepare_assets 25.87 |
| warm-up | 3.56 | 119.6 | pipelines/wait_ms 4829, pipelines/compile_ms 1239, pipelines/front_ms 1112, render/thread/ms 159, render/busy/ms 145, pipelines/driver_ms 128, main/busy/ms 54.49, stage/render/render 45.62, stage/main/post_update 42.78, render/graph/ms 31.23, stage/render/prepare_flush 29.21, stage/render/prepare_batch_phases 27.33 |
| warm-up | 3.61 | 44.6 | pipelines/wait_ms 5124, pipelines/compile_ms 1692, pipelines/front_ms 1575, render/thread/ms 153, render/busy/ms 151, pipelines/driver_ms 117, main/busy/ms 115, stage/main/update 60.45, stage/render/prepare_batch_phases 37.86, stage/main/post_update 34.28, stage/render/prepare_flush 29.39, stage/render/prepare_resources 23.88 |
| warm-up | 3.72 | 113.9 | pipelines/compile_ms 1890, pipelines/front_ms 1801, pipelines/wait_ms 635, main/busy/ms 104, render/thread/ms 97.87, render/busy/ms 95.82, pipelines/driver_ms 89.36, stage/main/update 61.99, stage/render/prepare_assets 58.38, stage/main/post_update 21.02, stage/render/render 16.82, terrain/update/ms 15.71 |
| warm-up | 3.78 | 55.7 | pipelines/compile_ms 1781, pipelines/front_ms 1616, pipelines/driver_ms 165, pipelines/wait_ms 149, render/thread/ms 116, render/busy/ms 113, stage/render/prepare_assets 87.40, main/busy/ms 53.61, stage/main/pre_update 27.94, stage/render/render 22.14, render/graph/ms 19.11, stage/main/post_update 14.25 |
| warm-up | 3.80 | 27.6 | pipelines/compile_ms 1045, pipelines/front_ms 892, pipelines/driver_ms 153, render/thread/ms 80.52, render/busy/ms 77.86, main/busy/ms 52.78, stage/render/prepare_assets 40.79, stage/render/render 31.41, render/graph/ms 28.48, stage/main/pre_update 27.56, stage/render/graph_submit 20.98, encode/submit 15.58 |
| warm-up | 3.88 | 26.6 | pipelines/compile_ms 378, pipelines/front_ms 354, main/busy/ms 53.76, stage/main/update 43.15, render/thread/ms 43.13, render/busy/ms 41.26, stream/drive/ms 38.81, pipelines/driver_ms 23.57, stage/render/prepare_assets 18.72, stage/render/render 12.01, render/graph/ms 9.83, encode//sum 7.53 |
| warm-up | 3.95 | 66.0 | pipelines/compile_ms 1460, pipelines/front_ms 1409, render/thread/ms 63.31, render/busy/ms 61.83, main/busy/ms 54.00, pipelines/driver_ms 50.88, stage/main/update 43.17, stage/render/prepare_assets 40.66, stream/drive/ms 38.96, stage/render/render 11.12, render/graph/ms 9.17, encode//sum 7.17 |
| warm-up | 4.03 | 66.6 | pipelines/compile_ms 256, pipelines/front_ms 245, main/busy/ms 51.87, render/thread/ms 45.78, render/busy/ms 43.90, stage/main/update 39.49, stream/drive/ms 34.09, stage/render/prepare_assets 23.69, stage/render/render 13.42, render/graph/ms 11.22, pipelines/driver_ms 11.06, stage/render/graph_submit 7.92 |
| warm-up | 4.06 | 27.3 | pipelines/compile_ms 449, pipelines/front_ms 435, render/thread/ms 57.51, render/busy/ms 55.60, main/busy/ms 36.84, stage/render/prepare_assets 32.09, stage/main/update 17.90, stage/render/render 15.63, pipelines/driver_ms 13.88, stage/main/post_update 13.24, render/graph/ms 13.22, stream/drive/ms 12.31 |
| warm-up | 4.12 | 67.2 | pipelines/compile_ms 212, pipelines/front_ms 205, render/thread/ms 73.49, render/busy/ms 71.04, stage/render/prepare_assets 40.24, main/busy/ms 38.42, stage/render/render 20.05, stage/main/update 18.65, render/graph/ms 16.72, stream/drive/ms 13.26, stage/main/post_update 13.24, stage/render/graph_submit 13.24 |
| warm-up | 4.17 | 47.6 | pipelines/compile_ms 508, pipelines/front_ms 487, render/thread/ms 82.85, render/busy/ms 80.54, stage/render/prepare_assets 54.05, main/busy/ms 34.55, pipelines/driver_ms 21.66, stage/render/render 16.12, stage/main/update 15.19, render/graph/ms 13.10, stage/main/post_update 12.78, stage/render/graph_submit 10.38 |
| warm-up | 4.24 | 72.8 | pipelines/compile_ms 1183, pipelines/front_ms 1141, render/thread/ms 77.61, render/busy/ms 75.83, stage/render/prepare_assets 58.13, main/busy/ms 46.17, pipelines/driver_ms 41.80, stage/main/update 24.38, stage/main/post_update 14.04, stage/render/render 10.38, render/graph/ms 8.28, stage/render/graph_submit 6.11 |
| warm-up | 4.31 | 61.1 | pipelines/compile_ms 760, pipelines/front_ms 735, render/thread/ms 94.52, render/busy/ms 92.85, main/busy/ms 76.16, stage/render/prepare_assets 69.24, stage/main/update 55.35, pipelines/driver_ms 24.88, stage/render/render 11.86, stage/main/post_update 11.57, render/graph/ms 9.97, stage/render/graph_submit 7.18 |
| warm-up | 4.37 | 69.4 | pipelines/compile_ms 921, pipelines/front_ms 896, render/thread/ms 101, render/busy/ms 100, main/busy/ms 73.54, stage/render/prepare_assets 72.54, stage/main/update 52.65, pipelines/driver_ms 25.95, stage/render/render 11.95, render/graph/ms 10.14, stage/main/post_update 9.78, stage/render/graph_submit 7.19 |
| warm-up | 4.42 | 46.8 | pipelines/compile_ms 985, pipelines/front_ms 956, render/thread/ms 91.53, render/busy/ms 89.85, main/busy/ms 74.72, stage/render/prepare_assets 47.49, stage/main/update 37.21, pipelines/driver_ms 29.01, stage/main/post_update 21.91, pipelines/wait_ms 21.14, stage/render/render 12.79, stage/render/specialize 11.16 |
| warm-up | 4.53 | 106.8 | pipelines/compile_ms 1529, pipelines/front_ms 1462, pipelines/wait_ms 518, render/thread/ms 133, main/busy/ms 133, render/busy/ms 131, pipelines/driver_ms 66.83, stage/main/update 58.40, stage/main/post_update 55.82, stage/render/prepare_write_phase_buffers 24.77, stream/drive/ms 21.84, stage/render/prepare_assets 19.86 |
| warm-up | 4.61 | 84.6 | pipelines/compile_ms 2433, pipelines/front_ms 2342, pipelines/wait_ms 855, render/thread/ms 158, render/busy/ms 156, main/busy/ms 135, pipelines/driver_ms 91.24, stage/main/update 73.94, stream/drive/ms 55.10, stage/main/post_update 42.95, stage/render/prepare_assets 31.08, stage/render/prepare_write_phase_buffers 30.53 |
| warm-up | 4.68 | 64.4 | pipelines/compile_ms 2059, pipelines/front_ms 1999, pipelines/wait_ms 479, render/thread/ms 114, render/busy/ms 112, main/busy/ms 76.87, pipelines/driver_ms 59.66, stage/main/update 52.82, encode//sum 43.17, stage/render/prepare_assets 42.11, stream/drive/ms 37.14, stage/render/render 32.64 |
| warm-up | 4.72 | 46.2 | pipelines/compile_ms 1952, pipelines/front_ms 1894, pipelines/wait_ms 558, render/thread/ms 88.68, main/busy/ms 87.11, render/busy/ms 86.52, stage/main/update 67.20, pipelines/driver_ms 58.33, stream/drive/ms 52.57, encode//sum 46.42, stage/render/prepare_assets 35.59, stage/render/render 30.66 |
| warm-up | 4.80 | 71.8 | pipelines/compile_ms 1409, pipelines/front_ms 1364, pipelines/wait_ms 489, main/busy/ms 145, stage/main/update 126, stream/drive/ms 77.85, render/thread/ms 72.12, render/busy/ms 69.84, pipelines/driver_ms 45.29, stage/render/prepare_assets 27.12, stage/render/render 24.51, encode//sum 22.15 |
| warm-up | 4.91 | 113.0 | pipelines/compile_ms 1140, pipelines/front_ms 1105, main/busy/ms 122, stage/main/update 95.56, render/thread/ms 82.31, render/busy/ms 80.18, pipelines/wait_ms 65.06, stage/render/prepare_assets 49.86, stream/drive/ms 45.29, pipelines/driver_ms 35.62, stage/render/render 16.58, stage/main/post_update 14.32 |
| warm-up | 4.95 | 43.1 | pipelines/compile_ms 1816, pipelines/front_ms 1766, render/thread/ms 87.17, render/busy/ms 85.34, main/busy/ms 84.65, stage/main/update 54.39, pipelines/driver_ms 49.63, stage/render/prepare_assets 44.40, stream/drive/ms 42.13, stage/render/render 20.24, render/graph/ms 17.82, pipelines/wait_ms 17.73 |
| warm-up | 4.99 | 37.7 | pipelines/compile_ms 1769, pipelines/front_ms 1721, main/busy/ms 78.81, render/thread/ms 66.39, render/busy/ms 64.46, stage/main/update 57.35, pipelines/driver_ms 48.24, stream/drive/ms 33.90, stage/render/render 23.63, render/graph/ms 20.67, stage/render/graph_submit 15.37, stage/render/prepare_assets 12.33 |
| warm-up | 5.04 | 48.9 | pipelines/compile_ms 860, pipelines/front_ms 837, render/thread/ms 61.50, render/busy/ms 59.78, main/busy/ms 57.25, stage/main/update 40.99, pipelines/driver_ms 23.62, stage/render/prepare_assets 21.88, stage/render/render 17.97, render/graph/ms 15.20, encode//sum 13.35, stream/drive/ms 12.34 |
| warm-up | 5.07 | 32.0 | pipelines/compile_ms 369, pipelines/front_ms 361, render/thread/ms 57.82, render/busy/ms 56.48, main/busy/ms 37.12, stage/render/prepare_assets 24.69, stage/main/update 20.56, stage/render/render 13.89, render/graph/ms 11.90, encode//sum 11.13, stage/main/post_update 10.46, pipelines/driver_ms 8.25 |
| warm-up | 5.13 | 38.1 | pipelines/compile_ms 762, pipelines/front_ms 736, render/thread/ms 57.21, render/busy/ms 55.66, main/busy/ms 35.49, pipelines/driver_ms 25.44, stage/render/render 20.04, render/graph/ms 17.85, stage/main/update 15.53, stage/main/post_update 13.62, stage/render/graph_submit 13.00, encode//sum 12.09 |
| warm-up | 5.17 | 37.9 | pipelines/compile_ms 397, pipelines/front_ms 384, render/thread/ms 66.65, render/busy/ms 64.90, main/busy/ms 39.52, stage/render/render 22.71, render/graph/ms 20.45, stage/render/prepare_assets 20.02, stage/main/update 18.91, stage/render/graph_submit 15.45, stage/main/post_update 13.66, pipelines/driver_ms 13.03 |
| warm-up | 5.23 | 59.1 | pipelines/compile_ms 362, pipelines/front_ms 350, render/thread/ms 80.84, render/busy/ms 79.27, main/busy/ms 58.21, stage/main/post_update 31.43, stage/render/prepare_assets 27.70, stage/main/update 16.45, stage/render/render 16.29, render/graph/ms 14.29, pipelines/driver_ms 12.41, encode//sum 10.60 |
| warm-up | 5.26 | 32.4 | pipelines/compile_ms 358, pipelines/front_ms 347, render/thread/ms 78.45, render/busy/ms 77.12, main/busy/ms 61.93, stage/main/post_update 34.03, stage/main/update 16.34, stage/render/prepare_write_phase_buffers 16.33, stage/render/prepare_assets 15.55, stage/render/render 15.03, render/graph/ms 13.09, encode//sum 12.13 |
| warm-up | 5.30 | 31.8 | pipelines/compile_ms 513, pipelines/front_ms 499, render/thread/ms 59.89, render/busy/ms 58.43, main/busy/ms 43.28, stage/main/post_update 22.24, stage/render/render 15.71, pipelines/driver_ms 14.32, stage/main/update 13.75, render/graph/ms 13.47, stage/render/prepare_assets 12.16, encode//sum 11.65 |
| warm-up | 5.39 | 29.3 | pipelines/compile_ms 762, pipelines/front_ms 739, pipelines/wait_ms 59.63, render/thread/ms 46.10, render/busy/ms 44.69, main/busy/ms 29.23, stage/render/render 24.22, pipelines/driver_ms 23.73, render/graph/ms 22.17, stage/main/post_update 15.84, stage/render/graph_submit 11.63, encode//sum 10.57 |
| warm-up | 5.44 | 35.8 | pipelines/compile_ms 488, pipelines/front_ms 475, pipelines/wait_ms 73.78, render/thread/ms 47.89, render/busy/ms 46.54, main/busy/ms 30.73, stage/render/prepare_assets 22.83, stage/main/post_update 13.49, stage/render/render 13.45, pipelines/driver_ms 13.34, stage/main/update 11.65, render/graph/ms 11.54 |
| warm-up | 5.48 | 27.7 | render/thread/ms 40.14, render/busy/ms 38.81, main/busy/ms 27.62, stage/render/render 17.93, render/graph/ms 15.98, encode//sum 14.32, stage/render/prepare_assets 13.68, stage/main/post_update 12.85, stage/render/graph_submit 11.03, stage/main/update 9.22, encode/submit 6.79, encode/pass:shadow_point_light/sum 4.97 |
| warm-up | 5.52 | 30.7 | render/thread/ms 52.44, render/busy/ms 50.70, main/busy/ms 43.68, stage/main/update 26.23, stage/render/prepare_assets 18.86, stage/render/render 18.67, render/graph/ms 15.86, stream/drive/ms 13.80, stage/main/post_update 11.70, pipelines/compile_ms 11.44, stage/render/graph_submit 10.92, encode//sum 10.76 |
| warm-up | 5.56 | 45.7 | pipelines/compile_ms 563, pipelines/front_ms 549, render/thread/ms 61.01, render/busy/ms 59.36, main/busy/ms 45.04, stage/render/prepare_assets 27.47, stage/main/update 26.08, stage/render/render 18.10, render/graph/ms 15.42, pipelines/driver_ms 13.76, stream/drive/ms 13.38, stage/main/post_update 12.70 |
| warm-up | 5.59 | 28.3 | pipelines/compile_ms 922, pipelines/front_ms 895, render/thread/ms 59.01, render/busy/ms 57.52, main/busy/ms 27.84, pipelines/driver_ms 26.05, stage/render/prepare_assets 24.11, stage/render/render 19.78, render/graph/ms 17.17, stage/main/post_update 13.03, stage/render/graph_submit 12.97, encode//sum 9.91 |
| warm-up | 5.63 | 36.5 | pipelines/compile_ms 671, pipelines/front_ms 641, render/thread/ms 58.78, render/busy/ms 57.17, main/busy/ms 36.30, pipelines/driver_ms 29.30, stage/render/render 21.47, stage/render/prepare_assets 21.35, render/graph/ms 18.68, stage/render/graph_submit 13.93, stage/main/post_update 13.39, encode//sum 9.88 |
| warm-up | 5.65 | 28.1 | pipelines/compile_ms 498, pipelines/front_ms 477, render/thread/ms 56.81, render/busy/ms 55.38, main/busy/ms 34.05, stage/render/prepare_assets 31.95, pipelines/driver_ms 21.21, stage/render/render 15.18, render/graph/ms 13.02, stage/main/post_update 11.42, stage/main/first 9.43, encode//sum 9.21 |
| warm-up | 5.69 | 35.3 | pipelines/compile_ms 212, pipelines/front_ms 206, render/thread/ms 57.14, render/busy/ms 55.92, stage/render/prepare_assets 36.34, main/busy/ms 25.57, stage/render/render 12.83, render/graph/ms 10.94, stage/main/post_update 10.36, stage/main/update 9.96, encode//sum 9.83, stage/render/graph_submit 6.94 |
| warm-up | 5.84 | 27.8 | pipelines/compile_ms 842, pipelines/front_ms 822, render/thread/ms 39.34, render/busy/ms 38.02, main/busy/ms 28.58, pipelines/driver_ms 20.31, stage/render/render 15.65, render/graph/ms 13.75, stage/render/prepare_assets 12.42, stage/main/post_update 11.64, stage/main/update 10.78, stage/render/graph_submit 10.21 |
| warm-up | 6.23 | 33.3 | render/thread/ms 49.85, render/busy/ms 48.02, main/busy/ms 30.52, stage/render/render 17.39, render/graph/ms 15.15, stage/main/post_update 13.59, pipelines/compile_ms 13.28, encode//sum 12.55, stage/main/update 12.46, pipelines/front_ms 11.70, stage/render/graph_submit 10.79, stage/render/specialize 9.30 |
| warm-up | 6.28 | 34.3 | render/thread/ms 38.60, render/busy/ms 37.58, main/busy/ms 23.22, stage/render/prepare_assets 13.63, stage/render/render 11.10, render/graph/ms 9.50, stage/main/post_update 9.40, stage/main/update 8.72, encode//sum 8.67, stage/render/graph_submit 6.55, encode/submit 4.47, encode/pass:shadow_point_light/sum 3.97 |
| warm-up | 6.40 | 28.4 | render/thread/ms 42.44, render/busy/ms 41.27, main/busy/ms 30.36, pipelines/compile_ms 20.16, pipelines/front_ms 18.74, stage/main/update 15.38, stage/render/render 12.07, stage/main/post_update 11.55, stage/render/prepare_assets 11.11, render/graph/ms 10.49, encode//sum 9.18, stage/render/graph_submit 6.37 |
| warm-up | 6.54 | 31.2 | render/thread/ms 49.59, render/busy/ms 48.29, main/busy/ms 36.93, stage/main/update 16.25, stage/main/post_update 14.89, stage/render/render 13.77, render/graph/ms 12.03, encode//sum 10.30, stage/render/graph_submit 7.53, stage/render/specialize 7.12, stage/render/prepare_assets 6.50, encode/pass:shadow_point_light/sum 5.97 |
| warm-up | 6.57 | 27.5 | render/thread/ms 48.32, render/busy/ms 47.09, main/busy/ms 36.55, stage/main/update 17.36, stage/render/prepare_assets 15.25, stage/main/post_update 13.22, stage/render/render 12.48, render/graph/ms 10.70, encode//sum 9.30, encode/pass:shadow_point_light/sum 6.33, stage/render/graph_submit 6.33, stage/render/prepare_meshes 4.55 |
| warm-up | 6.60 | 28.9 | pipelines/compile_ms 147, pipelines/front_ms 143, render/thread/ms 42.39, render/busy/ms 41.27, main/busy/ms 30.75, stage/render/prepare_assets 17.94, stage/main/update 14.76, stage/render/render 10.97, stage/main/post_update 9.77, render/graph/ms 9.36, encode//sum 8.82, stage/render/graph_submit 6.20 |
| warm-up | 6.65 | 32.7 | render/thread/ms 45.07, main/busy/ms 44.53, render/busy/ms 43.69, stage/main/update 24.92, stage/render/render 17.62, render/graph/ms 15.35, stage/main/post_update 14.36, stream/drive/ms 13.88, stage/render/graph_submit 11.63, encode//sum 10.77, encode/submit 8.77, encode/pass:shadow_point_light/sum 6.73 |
| warm-up | 6.68 | 31.7 | render/thread/ms 50.57, render/busy/ms 49.39, main/busy/ms 44.56, stage/main/update 23.86, stage/main/post_update 14.04, stage/render/render 12.53, stage/render/prepare_assets 11.40, render/graph/ms 10.73, encode//sum 9.51, stream/drive/ms 7.37, stage/render/graph_submit 7.05, encode/pass:shadow_point_light/sum 6.71 |
| warm-up | 6.72 | 39.3 | render/thread/ms 54.40, render/busy/ms 53.34, main/busy/ms 45.62, stage/main/update 28.50, stage/render/prepare_assets 19.60, stage/render/render 11.09, stage/main/post_update 10.83, render/graph/ms 9.61, encode//sum 8.13, stream/drive/ms 7.05, stage/render/graph_submit 6.61, stage/render/specialize 6.51 |
| warm-up | 6.92 | 28.9 | pipelines/compile_ms 147, pipelines/front_ms 143, render/thread/ms 49.53, render/busy/ms 48.38, main/busy/ms 30.65, stage/render/prepare_assets 26.21, stage/main/update 13.50, stage/main/post_update 12.18, stage/render/render 12.13, encode//sum 11.23, render/graph/ms 10.21, encode/pass:shadow_point_light/sum 7.34 |
| warm-up | 6.95 | 28.7 | pipelines/compile_ms 155, pipelines/front_ms 151, render/thread/ms 52.87, render/busy/ms 51.79, main/busy/ms 26.63, stage/render/prepare_assets 25.43, stage/render/render 12.43, stage/main/update 11.95, render/graph/ms 10.85, stage/main/post_update 10.36, encode//sum 9.97, stage/render/graph_submit 7.74 |
| warm-up | 7.14 | 39.3 | main/busy/ms 43.66, render/thread/ms 36.49, render/busy/ms 35.33, stage/main/update 24.91, pipelines/compile_ms 19.17, pipelines/front_ms 18.23, stream/drive/ms 13.69, stage/render/prepare_assets 13.60, stage/render/render 11.43, stage/main/post_update 11.05, render/graph/ms 9.77, encode//sum 9.67 |
| measure | 2.17 | 26.9 | encode//sum 38.73, render/thread/ms 37.63, render/busy/ms 36.20, stage/render/render 24.46, render/graph/ms 22.55, encode/pass:shadow_point_light/sum 19.92, main/busy/ms 18.95, stage/render/graph_submit 16.41, pipelines/compile_ms 11.63, pipelines/front_ms 11.41, stage/main/post_update 11.23, encode/submit 9.19 |
| measure | 2.72 | 28.3 | render/thread/ms 38.79, render/busy/ms 37.27, encode//sum 30.95, stage/render/render 25.12, render/graph/ms 23.07, encode/pass:shadow_point_light/sum 20.24, main/busy/ms 19.18, stage/render/graph_submit 16.44, stage/main/post_update 12.12, encode/submit 10.15, stage/render/camera_driver 6.50, encode/finish_wall 6.33 |
| measure | 18.18 | 27.8 | main/busy/ms 36.30, render/thread/ms 25.23, render/busy/ms 24.24, stage/main/update 21.32, stream/drive/ms 17.62, stage/render/render 15.11, encode//sum 14.53, render/graph/ms 13.59, stage/main/post_update 12.27, stage/render/graph_submit 9.97, encode/pass:shadow_point_light/sum 8.54, encode/submit 6.46 |
| measure | 59.02 | 26.1 | render/thread/ms 36.39, render/busy/ms 34.81, main/busy/ms 33.66, stage/main/post_update 21.33, stage/render/render 17.94, render/graph/ms 15.76, encode//sum 12.42, stage/render/graph_submit 11.35, stage/main/update 7.93, encode/submit 7.27, stage/render/camera_driver 4.23, encode/finish_wall 4.20 |
| measure | 59.04 | 27.5 | render/thread/ms 40.64, render/busy/ms 39.51, main/busy/ms 30.02, stage/main/post_update 18.31, stage/render/prepare_assets 13.03, stage/render/render 12.78, render/graph/ms 11.21, encode//sum 9.51, stage/render/graph_submit 8.00, stage/main/update 6.70, encode/submit 4.66, encode/finish_wall 3.36 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=nocull-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 14%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **72%** (min 54%) · 2625 MHz (min 2625) · memory 8101 MHz · 74.9 W · 75 °C (max 81) · VRAM 7018 MiB · throttled 100% of samples

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
