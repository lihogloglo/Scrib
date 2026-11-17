# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.gate=on (command line: --set); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5878 frames over 65.6 s · **median 10.75 ms (93 fps)** · mean 11.17 · p95 14.77 · p99 16.76 (1% low 60 fps) · worst 31.96 ms · 101 spikes over 1.5× median · 27.4 stutters/min (work 22.8/min)
- warm-up: 323 frames over 9.4 s, worst 2400.7 ms
- hitches (> 26.9 ms): **3** measured, 39 in warm-up
- timed GPU passes: 6.69 ms of a 11.17 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.75 ms, limit 11 ms
- **FAIL** frame_ms: 30 stutters (> 1.5x moving median of 60) in 1.09 min = 27.42/min, limit 1/min
- pass worst frame 31.96 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.995 | 2.030 | 2.063 | 0.98 | 0.180 |
| main_opaque_pass_3d | 1.278 | 1.300 | 1.638 | 0.98 | 0.069 |
| early prepass | 0.480 | 0.488 | 0.700 | 0.98 | 0.084 |
| bloom | 0.311 | 0.316 | 0.322 | 0.98 | 0.064 |
| auxiliary_views | 0.305 | 0.310 | 1.158 | 0.98 | 0.291 |
| atmosphere_luts | 0.304 | 0.309 | 0.371 | 0.98 | 0.003 |
| water_velocity | 0.257 | 0.262 | 0.349 | 0.98 | 0.002 |
| water_sspr | 0.198 | 0.201 | 0.203 | 0.98 | 0.002 |
| ssao | 0.185 | 0.189 | 0.193 | 0.98 | 0.007 |
| upscaling | 0.127 | 0.130 | 0.136 | 0.98 | 0.004 |
| water_fft | 0.126 | 0.128 | 0.137 | 0.98 | 0.003 |
| atmos_froxel_inject | 0.125 | 0.127 | 0.129 | 0.98 | 0.001 |
| uniform_allocation | 0.124 | 0.126 | 0.154 | 0.98 | 0.026 |
| clouds_composite | 0.103 | 0.105 | 0.108 | 0.98 | 0.002 |
| bin_unpacking | 0.097 | 0.098 | 0.120 | 0.98 | 0.018 |
| tonemapping | 0.085 | 0.087 | 0.091 | 0.98 | 0.003 |
| main_transmissive_pass_3d | 0.077 | 0.078 | 0.154 | 0.98 | 0.008 |
| clustering | 0.062 | 0.063 | 0.067 | 0.98 | 0.019 |
| clouds_trace | 0.054 | 0.055 | 0.073 | 0.98 | 0.001 |
| render_sky | 0.051 | 0.051 | 0.052 | 0.98 | 0.002 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 0.98 | 0.001 |
| lightprobe_irradiance_map | 0.046 | 0.410 | 0.414 | 0.11 | 0.001 |
| surface_trail | 0.040 | 0.056 | 0.057 | 0.71 | 0.002 |
| atmos_froxel_integrate | 0.030 | 0.031 | 0.032 | 0.98 | 0.001 |
| water_snapshot | 0.030 | 0.030 | 0.034 | 0.98 | 0.002 |
| early_mesh_preprocessing | 0.027 | 0.028 | 0.036 | 0.98 | 0.005 |
| lightprobe_radiance_map | 0.025 | 0.218 | 0.228 | 0.11 | 0.004 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.98 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.036 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.013 | 0.013 | 0.024 | 0.98 | 0.125 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.98 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.98 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.98 | 0.004 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.98 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.98 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.827 | 13.301 | 24.81 | 57761.2 |
| render/busy/ms | 9.307 | 12.746 | 24.01 | 54703.7 |
| main/busy/ms | 9.206 | 11.151 | 29.59 | 54112.1 |
| stage/render/render | 5.962 | 8.681 | 16.14 | 35046.1 |
| stage/main/post_update | 5.752 | 7.355 | 10.17 | 33809.2 |
| render/graph/ms | 5.257 | 7.889 | 14.80 | 30898.3 |
| encode//sum | 4.660 | 7.270 | 28.64 | 27389.1 |
| stage/render/graph_submit | 3.918 | 5.792 | 12.97 | 23031.1 |
| encode/submit | 2.535 | 3.889 | 6.07 | 14898.2 |
| stage/main/update | 2.182 | 2.629 | 22.93 | 12826.9 |
| encode/pass:shadow_point_light/sum | 1.589 | 5.226 | 17.46 | 9337.9 |
| encode/finish_wall | 1.425 | 2.010 | 10.23 | 8374.3 |
| stage/render/camera_driver | 1.285 | 2.095 | 3.94 | 7554.4 |
| stage/render/prepare_write_phase_buffers | 0.787 | 1.122 | 2.53 | 4626.9 |
| encode/pass:shadow_directional_light/sum | 0.756 | 1.257 | 1.94 | 4440.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.734 | 1.074 | 1.76 | 4313.5 |
| encode/pass:early prepass/sum | 0.714 | 1.064 | 9.32 | 4196.8 |
| stage/render/prepare_assets | 0.593 | 0.743 | 10.14 | 3486.2 |
| bigspace/total/ms | 0.564 | 1.422 | 2.57 | 3315.6 |
| stage/render/prepare_resources | 0.463 | 0.554 | 1.24 | 2723.0 |
| stage/render/prepare_meshes | 0.433 | 0.541 | 1.63 | 2545.9 |
| render/present/ms | 0.427 | 0.524 | 2.30 | 2507.3 |
| stage/main/first | 0.397 | 0.517 | 6.05 | 2334.8 |
| stage/main/pre_update | 0.362 | 0.510 | 1.24 | 2126.1 |
| terrain/update/ms | 0.318 | 0.422 | 2.12 | 1871.2 |
| terrain/select/ms | 0.284 | 0.379 | 0.93 | 1670.5 |
| stage/render/prepare_bind_groups | 0.269 | 0.469 | 2.54 | 1578.7 |
| stage/render/specialize | 0.268 | 0.411 | 2.24 | 1577.5 |
| controller/feed/ms | 0.266 | 0.333 | 1.29 | 1563.1 |
| stage/render/queue | 0.249 | 0.394 | 2.44 | 1462.3 |
| stream/drive/ms | 0.248 | 0.329 | 20.85 | 1458.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.247 | 0.366 | 0.94 | 1453.7 |
| bigspace/low_precision/ms | 0.235 | 0.771 | 1.94 | 1380.2 |
| stage/main/fixed_main_loop | 0.221 | 0.368 | 1.13 | 1297.0 |
| far/select/ms | 0.202 | 0.272 | 0.68 | 1190.0 |
| stage/render/create_views | 0.195 | 0.255 | 0.91 | 1148.5 |
| stream/tick/ms | 0.190 | 0.258 | 0.62 | 1119.5 |
| stage/render/prepare_batch_phases | 0.189 | 0.271 | 0.62 | 1112.5 |
| stage/render/extract_commands | 0.174 | 0.204 | 0.54 | 1024.3 |
| anim/evaluate/ms | 0.170 | 0.274 | 2.07 | 1001.2 |
| stage/render/prepare_flush | 0.164 | 0.311 | 1.00 | 965.9 |
| bigspace/high_precision/ms | 0.163 | 0.708 | 1.94 | 955.3 |
| anim/evaluate_cpu/ms | 0.162 | 0.199 | 0.69 | 955.1 |
| controller/move/ms | 0.155 | 0.364 | 0.65 | 909.9 |
| stage/render/prepare_views | 0.137 | 0.172 | 0.37 | 807.0 |
| stage/main/last | 0.114 | 0.155 | 1.61 | 670.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.107 | 0.180 | 0.75 | 627.4 |
| render/acquire/ms | 0.094 | 0.119 | 0.30 | 550.2 |
| stage/render/prepare_collect_phase_buffers | 0.062 | 0.130 | 0.49 | 362.2 |
| physics/step/ms | 0.049 | 0.088 | 0.48 | 287.3 |
| physics/insert/ms | 0.036 | 0.381 | 0.93 | 211.9 |
| stage/render/phase_sort | 0.032 | 0.055 | 0.24 | 191.0 |
| stage/render/cleanup | 0.032 | 0.096 | 0.63 | 187.5 |
| encode/resolve_diagnostics/sum | 0.031 | 0.041 | 0.56 | 181.4 |
| pipelines/compile_ms | 0.016 | 0.000 | 83.52 | 95.0 |
| weather/map/ms | 0.016 | 0.021 | 0.20 | 93.6 |
| pipelines/front_ms | 0.016 | 0.000 | 80.54 | 91.7 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.55 | 87.0 |
| anim/apply/ms | 0.012 | 0.021 | 0.31 | 67.9 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.17 | 63.8 |
| controller/avoid/ms | 0.011 | 0.014 | 0.20 | 61.7 |
| weather/eval/ms | 0.010 | 0.013 | 0.04 | 58.2 |
| script/update/ms | 0.007 | 0.009 | 0.16 | 42.0 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 32.1 |
| stream/despawn/ms | 0.005 | 0.000 | 0.98 | 30.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 28.9 |
| script/ui/ms | 0.005 | 0.006 | 0.22 | 26.9 |
| controller/write/ms | 0.003 | 0.004 | 0.03 | 20.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 13.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.07 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.40 | 3.5 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.99 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.1 |
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
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1257 | 2262 | 563 |
| crowd/c0/meshes/count | count | 349 | 646 | 144 |
| crowd/c1/count | count | 147 | 157 | 148 |
| crowd/c1/joints/count | count | 5549 | 6037 | 5601 |
| crowd/c1/meshes/count | count | 1166 | 1311 | 1189 |
| crowd/c2/count | count | 399 | 418 | 418 |
| crowd/c2/joints/count | count | 6406 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1822 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 165 | 216 | 205 |
| crowd/joints_extracted/count | count | 3708 | 7161 | 5775 |
| crowd/joints_uploaded/count | count | 3648 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.42 | 9.96 | 0.41 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.55 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.17 | 0.02 |
| encode/finish_critical | ms | 0.95 | 9.96 | 1.23 |
| encode/pass:early prepass/max | ms | 0.74 | 9.32 | 0.82 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 1.76 | 0.63 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.75 | 0.17 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.24 | 0.94 | 0.22 |
| encode/pass:shadow_directional_light/max | ms | 0.76 | 1.94 | 1.23 |
| encode/pass:shadow_point_light/max | ms | 0.80 | 1.49 | 0.66 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.56 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.38 | 29.59 | 7.63 |
| gpu/timed/ms | ms | 7.47 | 10.78 | 7.63 |
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
| phases/alpha_mask3d/items | count | 792 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 792 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1297 | 1398 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3342 | 5133 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1297 | 1398 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3342 | 5133 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 22 | 197 | 19 |
| phases/shadow/bins | count | 3737 | 4409 | 1494 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6210 | 7389 | 3038 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 203 | 241 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 555 | 37 |
| preprocess/bind_groups_created | count | 13 | 555 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 134 | 291 | 127 |
| process/cpu_usage | % | 26.61 | 46.15 | 24.39 |
| process/mem_usage | GiB | 5.43 | 5.60 | 5.60 |
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
| skin/blocks_changed/count | count | 13 | 81 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 164 | 216 | 205 |
| skin/joint_bytes_written/bytes | bytes | 282.9 k | 807.2 k | 447.0 k |
| skin/joints_written/count | count | 4420 | 12.6 k | 6984 |
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
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 327 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.74 | 43.96 | 34.26 |
| system/mem_usage | % | 54.11 | 54.68 | 54.61 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 71 | 246 | 71 |
| views/all/bins | count | 6213 | 7314 | 5002 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.5 k | 20.3 k | 10.7 k |
| views/all/pipelines | count | 249 | 286 | 67 |
| views/all/pipelines_distinct | count | 70 | 75 | 63 |
| views/all/work_items | count | 10.4 k | 17.9 k | 11.0 k |
| views/cascade/batch_sets | count | 8 | 27 | 19 |
| views/cascade/bins | count | 1450 | 1528 | 1494 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3388 | 4457 | 3038 |
| views/cascade/pipelines | count | 23 | 27 | 19 |
| views/cascade/work_items | count | 330 | 4457 | 3038 |
| views/lamp_face/batch_sets | count | 0 | 189 | 0 |
| views/lamp_face/bins | count | 2449 | 2981 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2782 | 3291 | 0 |
| views/lamp_face/pipelines | count | 180 | 215 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2873 | 0 |
| views/main/batch_sets | count | 50 | 53 | 52 |
| views/main/bins | count | 2896 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8270 | 13.0 k | 7614 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8434 | 13.3 k | 7922 |
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
| anim/bones_written/count | count | 56.57 | 286 | 332.5 k |
| anim/events/count | count | 0.09 | 3 | 517 |
| assets/Image/added | count | 0.10 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.69 | 152 | 4061 |
| assets/Mesh/removed | count | 0.82 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.69 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.80 | 1304 | 4675 |
| encode//count | count | 39.75 | 219 | 233.7 k |
| encode/bind_walk/init_textures | count | 16.1 k | 112.9 k | 94.48 M |
| encode/bind_walk/init_textures_skipped | count | 58.6 k | 144.0 k | 344.32 M |
| encode/bind_walk/inits | count | 478 | 1119 | 2.81 M |
| encode/bind_walk/inits_skipped | count | 102 | 167 | 598.6 k |
| encode/bind_walk/merge_textures | count | 16.0 k | 112.9 k | 94.11 M |
| encode/bind_walk/merge_textures_skipped | count | 5396 | 6012 | 31.72 M |
| encode/bind_walk/merges | count | 108 | 340 | 632.3 k |
| encode/bind_walk/merges_skipped | count | 80.38 | 123 | 472.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5878 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5878 |
| encode/pass:early prepass/count | count | 1 | 1 | 5878 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5878 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5878 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5878 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5878 |
| encode/pass:shadow_point_light/count | count | 3.20 | 48 | 18.8 k |
| encode/queue/command_buffers | count | 52.97 | 277 | 311.3 k |
| encode/queue/submits | count | 2.02 | 4 | 11.9 k |
| encode/queue/write_buffer/bytes | bytes | 1.82 M | 3.02 M | 10.71 G |
| encode/queue/write_buffer/calls | count | 272 | 829 | 1.60 M |
| encode/queue/write_buffer_with/bytes | bytes | 169.9 k | 3.53 M | 998.81 M |
| encode/queue/write_buffer_with/calls | count | 11.06 | 465 | 65.0 k |
| encode/queue/write_texture/bytes | bytes | 10.7 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5878 |
| physics/inserts | count | 0.39 | 111 | 2269 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.67 | 2 | 3939 |
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
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 22 | 378 |
| stream/upload/bytes | bytes | 5759 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1263 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.9 ms; slowest 42, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.40 | 2400.7 | main/busy/ms 2372, stage/main/post_update 2332, stage/main/update 30.95, stage/main/pre_update 4.67, stage/main/last 1.62, stream/drive/ms 0.75, surface/advance_cpu_ms 0.72, stage/main/first 0.48, stage/main/fixed_main_loop 0.45, surface/cell_upload_ms 0.29, stream/tick/ms 0.20, bigspace/total/ms 0.08 |
| warm-up | 3.23 | 830.7 | main/busy/ms 2547, stage/main/post_update 2358, render/thread/ms 799, render/busy/ms 631, stage/render/prepare_assets 583, stage/main/update 175, stage/render/prepare_views 163, render/acquire/ms 162, stage/render/create_views 28.17, stage/render/render 14.73, render/graph/ms 8.05, stage/render/camera_driver 6.40 |
| warm-up | 3.37 | 143.2 | render/thread/ms 926, render/busy/ms 691, stage/render/prepare_assets 631, main/busy/ms 200, stage/render/prepare_views 163, render/acquire/ms 163, stage/main/update 150, pipelines/compile_ms 115, stage/render/render 86.65, render/present/ms 72.92, pipelines/driver_ms 61.79, pipelines/front_ms 53.18 |
| warm-up | 3.54 | 140.0 | pipelines/wait_ms 3772, pipelines/compile_ms 1810, pipelines/front_ms 1631, main/busy/ms 183, pipelines/driver_ms 180, render/thread/ms 128, render/busy/ms 122, stage/main/update 75.46, stage/main/post_update 72.70, stage/render/prepare_resources 42.36, stage/main/pre_update 29.78, stage/render/render 25.48 |
| warm-up | 3.64 | 99.4 | pipelines/wait_ms 4071, pipelines/compile_ms 2162, pipelines/front_ms 2018, render/thread/ms 176, render/busy/ms 174, main/busy/ms 156, pipelines/driver_ms 143, stage/main/update 77.90, stage/render/prepare_assets 77.88, stage/main/post_update 42.54, stage/render/prepare_resources 42.17, stage/main/pre_update 30.66 |
| warm-up | 3.66 | 29.1 | pipelines/compile_ms 980, pipelines/front_ms 918, pipelines/wait_ms 319, render/thread/ms 94.78, render/busy/ms 92.76, stage/render/prepare_assets 74.39, pipelines/driver_ms 61.94, main/busy/ms 35.55, stage/main/post_update 16.47, terrain/update/ms 14.06, stage/main/update 11.52, stage/render/render 10.65 |
| warm-up | 3.87 | 82.7 | pipelines/compile_ms 1734, pipelines/front_ms 1658, pipelines/driver_ms 75.96, render/thread/ms 68.13, render/busy/ms 66.60, main/busy/ms 62.31, stage/main/update 47.25, stream/drive/ms 42.26, stage/render/prepare_assets 38.75, stage/render/render 13.44, render/graph/ms 11.63, stage/main/post_update 9.38 |
| warm-up | 3.90 | 32.5 | pipelines/compile_ms 1423, pipelines/front_ms 1357, render/thread/ms 72.86, render/busy/ms 71.04, pipelines/driver_ms 66.40, main/busy/ms 44.30, stage/render/prepare_assets 43.08, stage/main/update 29.65, stage/render/render 22.49, render/graph/ms 20.51, stage/render/graph_submit 18.09, stream/drive/ms 11.76 |
| warm-up | 3.93 | 30.2 | pipelines/compile_ms 379, pipelines/front_ms 362, render/thread/ms 49.59, render/busy/ms 47.83, main/busy/ms 41.41, pipelines/wait_ms 31.05, stage/main/update 29.17, pipelines/lock_ms 24.13, stage/render/render 21.76, stage/render/prepare_assets 20.21, render/graph/ms 19.78, stage/render/graph_submit 17.19 |
| warm-up | 4.04 | 81.0 | pipelines/compile_ms 253, pipelines/front_ms 243, render/thread/ms 81.95, render/busy/ms 80.59, main/busy/ms 54.37, stage/render/prepare_assets 51.26, stage/main/update 35.99, stream/drive/ms 31.37, stage/render/render 13.01, stage/main/post_update 11.58, render/graph/ms 11.11, pipelines/driver_ms 10.12 |
| warm-up | 4.09 | 48.1 | pipelines/compile_ms 275, pipelines/front_ms 262, render/thread/ms 93.95, render/busy/ms 91.84, stage/render/prepare_assets 50.08, main/busy/ms 43.94, stage/main/update 17.70, stage/render/render 17.17, stage/main/post_update 16.31, stage/render/prepare_resources 14.59, render/graph/ms 14.41, pipelines/driver_ms 13.18 |
| warm-up | 4.18 | 90.4 | pipelines/compile_ms 2143, pipelines/front_ms 2076, render/thread/ms 98.06, render/busy/ms 95.48, main/busy/ms 67.87, pipelines/driver_ms 66.44, stage/render/prepare_assets 48.07, stage/main/update 43.11, stage/render/render 27.34, pipelines/wait_ms 24.52, render/graph/ms 24.30, pipelines/lock_ms 21.93 |
| warm-up | 4.25 | 77.2 | pipelines/compile_ms 2022, pipelines/front_ms 1962, render/thread/ms 115, render/busy/ms 112, stage/render/prepare_assets 73.01, main/busy/ms 65.25, pipelines/driver_ms 60.29, pipelines/wait_ms 51.71, stage/main/update 39.36, stage/render/render 29.05, render/graph/ms 25.94, pipelines/lock_ms 23.31 |
| warm-up | 4.30 | 49.1 | pipelines/compile_ms 609, pipelines/front_ms 584, render/thread/ms 96.02, render/busy/ms 93.62, pipelines/wait_ms 90.40, main/busy/ms 63.16, stage/render/prepare_assets 52.36, stage/main/post_update 25.73, pipelines/driver_ms 24.56, stage/render/render 18.48, encode//sum 17.67, stage/main/update 16.01 |
| warm-up | 4.39 | 87.2 | pipelines/compile_ms 1794, pipelines/front_ms 1728, pipelines/wait_ms 277, render/thread/ms 123, render/busy/ms 121, main/busy/ms 65.57, pipelines/driver_ms 65.34, encode//sum 50.48, stage/render/prepare_assets 30.94, stage/render/prepare_resources 28.39, stage/render/render 28.15, render/graph/ms 25.78 |
| warm-up | 4.45 | 56.6 | pipelines/compile_ms 1530, pipelines/front_ms 1475, pipelines/wait_ms 302, render/thread/ms 133, render/busy/ms 131, encode//sum 71.53, pipelines/driver_ms 55.00, main/busy/ms 52.27, stage/render/prepare_resources 47.52, stage/render/render 36.88, render/graph/ms 34.33, stage/render/graph_submit 23.56 |
| warm-up | 4.51 | 63.1 | pipelines/compile_ms 1436, pipelines/front_ms 1371, pipelines/wait_ms 417, render/thread/ms 105, render/busy/ms 103, pipelines/driver_ms 64.14, main/busy/ms 46.99, encode//sum 40.07, stage/render/render 27.67, render/graph/ms 25.31, stage/render/prepare_assets 22.99, stage/render/prepare_resources 21.86 |
| warm-up | 4.55 | 39.4 | pipelines/compile_ms 1668, pipelines/front_ms 1602, pipelines/wait_ms 371, main/busy/ms 94.06, render/thread/ms 82.37, render/busy/ms 80.60, stage/main/update 67.69, pipelines/driver_ms 65.68, stream/drive/ms 55.52, stage/render/prepare_assets 27.23, stage/render/render 21.74, render/graph/ms 19.37 |
| warm-up | 4.66 | 115.2 | pipelines/compile_ms 1778, pipelines/front_ms 1728, pipelines/wait_ms 680, main/busy/ms 112, render/thread/ms 99.59, render/busy/ms 97.66, stage/main/update 83.99, stream/drive/ms 57.76, pipelines/driver_ms 49.30, stage/render/prepare_write_phase_buffers 31.97, stage/render/prepare_flush 19.50, stage/render/render 18.79 |
| warm-up | 4.72 | 52.9 | pipelines/compile_ms 2143, pipelines/front_ms 2082, pipelines/wait_ms 1258, main/busy/ms 123, render/thread/ms 108, render/busy/ms 106, stage/main/update 97.97, pipelines/driver_ms 61.02, stream/drive/ms 51.47, stage/render/prepare_write_phase_buffers 30.60, stage/render/prepare_assets 22.74, stage/render/prepare_flush 18.99 |
| warm-up | 4.88 | 162.0 | pipelines/compile_ms 2136, pipelines/front_ms 2082, pipelines/wait_ms 637, render/thread/ms 131, render/busy/ms 129, main/busy/ms 106, stage/main/update 83.80, stage/render/prepare_assets 62.86, pipelines/driver_ms 54.85, stream/drive/ms 49.90, stage/render/render 49.50, render/graph/ms 47.01 |
| warm-up | 4.92 | 42.2 | pipelines/compile_ms 1504, pipelines/front_ms 1466, render/thread/ms 131, render/busy/ms 128, main/busy/ms 85.71, stage/render/render 62.67, stage/main/update 60.79, render/graph/ms 59.14, stage/render/graph_submit 54.03, stage/render/prepare_assets 51.66, stream/drive/ms 45.21, encode//sum 41.57 |
| warm-up | 4.99 | 72.6 | pipelines/compile_ms 236, pipelines/front_ms 228, main/busy/ms 91.04, render/thread/ms 75.06, render/busy/ms 71.91, stage/main/update 70.53, stream/drive/ms 44.58, stage/render/render 30.52, render/graph/ms 27.00, stage/render/prepare_assets 25.15, stage/render/graph_submit 22.27, encode/submit 20.68 |
| warm-up | 5.03 | 35.9 | pipelines/compile_ms 919, pipelines/front_ms 891, main/busy/ms 68.12, render/thread/ms 59.37, render/busy/ms 57.96, stage/main/update 52.87, stream/drive/ms 30.69, stage/render/prepare_assets 30.36, pipelines/driver_ms 28.20, stage/render/render 12.84, render/graph/ms 10.79, encode//sum 9.31 |
| warm-up | 5.06 | 33.3 | pipelines/compile_ms 896, pipelines/front_ms 869, main/busy/ms 50.61, render/thread/ms 38.34, render/busy/ms 36.99, stage/main/update 36.67, stream/drive/ms 30.59, pipelines/driver_ms 27.64, stage/render/prepare_assets 13.47, stage/render/render 12.69, render/graph/ms 10.87, encode//sum 9.20 |
| warm-up | 5.15 | 36.6 | pipelines/compile_ms 117, pipelines/front_ms 111, render/thread/ms 50.39, render/busy/ms 48.98, main/busy/ms 36.15, stage/main/update 18.11, stage/render/prepare_assets 16.55, stage/render/render 14.44, render/graph/ms 12.41, stage/main/post_update 12.30, encode//sum 10.12, stage/render/graph_submit 8.34 |
| warm-up | 5.21 | 57.1 | pipelines/compile_ms 120, pipelines/front_ms 116, render/thread/ms 83.59, render/busy/ms 82.11, main/busy/ms 59.94, stage/main/post_update 32.09, stage/main/update 21.84, stage/render/prepare_write_phase_buffers 20.29, stage/render/prepare_assets 18.85, stage/render/render 14.69, render/graph/ms 12.62, stage/render/prepare_resources 12.22 |
| warm-up | 5.26 | 52.2 | pipelines/compile_ms 721, pipelines/front_ms 702, render/thread/ms 101, render/busy/ms 96.81, main/busy/ms 56.45, stage/render/render 38.59, render/graph/ms 34.48, stage/main/post_update 29.69, pipelines/driver_ms 19.86, stage/render/prepare_write_phase_buffers 19.82, stage/main/update 18.82, stage/render/graph_submit 16.71 |
| warm-up | 5.31 | 50.0 | pipelines/compile_ms 1857, pipelines/front_ms 1809, render/thread/ms 83.51, render/busy/ms 79.91, pipelines/wait_ms 58.00, pipelines/driver_ms 48.06, main/busy/ms 44.71, stage/render/render 38.64, render/graph/ms 34.53, stage/render/prepare_assets 27.89, stage/main/update 23.15, stage/render/graph_submit 17.07 |
| warm-up | 5.42 | 39.2 | pipelines/compile_ms 551, pipelines/front_ms 539, render/thread/ms 52.79, render/busy/ms 51.47, main/busy/ms 32.38, stage/render/prepare_assets 26.07, stage/render/render 14.29, stage/main/update 13.42, render/graph/ms 12.37, stage/main/post_update 12.30, pipelines/driver_ms 11.53, encode//sum 11.39 |
| warm-up | 5.56 | 30.3 | render/thread/ms 49.90, render/busy/ms 48.61, stage/render/prepare_assets 27.36, main/busy/ms 25.76, stage/main/post_update 10.84, stage/render/render 10.67, stage/main/update 10.56, encode//sum 9.83, render/graph/ms 8.90, stage/render/graph_submit 5.62, encode/pass:shadow_point_light/sum 3.43, encode/submit 3.23 |
| warm-up | 5.59 | 30.5 | render/thread/ms 52.81, render/busy/ms 51.34, stage/render/prepare_assets 29.72, main/busy/ms 23.02, stage/render/render 14.68, render/graph/ms 12.57, stage/main/post_update 10.37, encode//sum 9.80, pipelines/compile_ms 9.75, stage/render/graph_submit 9.06, pipelines/front_ms 9.02, stage/main/update 8.13 |
| warm-up | 5.62 | 33.4 | pipelines/compile_ms 367, pipelines/front_ms 346, render/thread/ms 54.17, render/busy/ms 52.72, stage/render/prepare_assets 28.53, main/busy/ms 22.69, pipelines/driver_ms 21.28, stage/render/render 16.91, render/graph/ms 14.58, encode//sum 10.82, stage/render/graph_submit 10.70, stage/main/post_update 9.40 |
| warm-up | 6.18 | 29.7 | render/thread/ms 36.57, render/busy/ms 35.49, main/busy/ms 25.84, stage/render/prepare_assets 15.89, stage/main/post_update 10.25, stage/main/update 10.03, stage/render/render 9.99, render/graph/ms 8.54, encode//sum 7.78, stage/render/graph_submit 5.79, encode/pass:shadow_point_light/sum 4.91, stream/drive/ms 4.34 |
| warm-up | 6.41 | 27.6 | render/thread/ms 41.17, render/busy/ms 39.91, main/busy/ms 38.83, stage/render/render 21.85, render/graph/ms 19.89, stage/main/update 16.67, stage/main/post_update 15.84, encode//sum 14.21, stage/render/graph_submit 13.98, encode/submit 10.10, encode/pass:shadow_point_light/sum 8.17, stream/drive/ms 5.78 |
| warm-up | 6.54 | 29.7 | render/thread/ms 42.23, render/busy/ms 40.99, main/busy/ms 38.59, stage/main/update 18.92, stage/render/prepare_assets 16.35, stage/main/post_update 12.94, stage/render/render 11.83, render/graph/ms 10.01, encode//sum 8.86, encode/pass:shadow_point_light/sum 6.10, stage/render/graph_submit 5.69, stream/drive/ms 4.28 |
| warm-up | 6.74 | 28.9 | render/thread/ms 50.57, render/busy/ms 49.42, main/busy/ms 29.75, stage/render/prepare_assets 27.77, stage/render/render 13.49, stage/main/post_update 12.49, stage/main/update 12.47, encode//sum 11.95, render/graph/ms 11.60, stage/render/graph_submit 7.46, encode/pass:shadow_point_light/sum 7.31, stream/drive/ms 6.53 |
| warm-up | 6.79 | 28.4 | pipelines/compile_ms 148, pipelines/front_ms 145, render/thread/ms 42.72, render/busy/ms 41.65, main/busy/ms 25.38, stage/render/prepare_assets 24.51, stage/main/post_update 10.52, stage/render/render 10.28, stage/main/update 9.39, render/graph/ms 8.71, encode//sum 8.11, stage/render/graph_submit 5.69 |
| warm-up | 6.95 | 34.0 | pipelines/compile_ms 145, pipelines/front_ms 138, main/busy/ms 41.85, render/thread/ms 27.38, render/busy/ms 26.24, stage/main/update 21.55, stage/main/post_update 14.58, stage/render/render 14.57, stream/drive/ms 14.53, stream/spawn/ms 13.38, render/graph/ms 13.05, encode//sum 10.29 |
| measure | 18.18 | 32.0 | main/busy/ms 39.58, stage/main/update 25.08, render/thread/ms 23.68, render/busy/ms 22.72, stream/drive/ms 21.09, stage/render/render 14.69, render/graph/ms 13.20, encode//sum 11.55, stage/main/post_update 11.51, stage/render/graph_submit 9.56, encode/pass:shadow_point_light/sum 7.78, encode/submit 6.21 |
| measure | 23.08 | 27.3 | render/thread/ms 38.44, render/busy/ms 37.24, main/busy/ms 29.69, stage/render/render 15.50, encode//sum 14.86, stage/main/post_update 14.48, render/graph/ms 13.64, stage/main/update 11.33, encode/pass:shadow_point_light/sum 10.09, stage/render/graph_submit 9.22, stage/render/prepare_assets 8.44, encode/submit 5.46 |
| measure | 59.04 | 27.1 | render/thread/ms 36.89, render/busy/ms 35.72, main/busy/ms 27.26, stage/render/prepare_assets 14.64, stage/main/post_update 14.24, stage/render/render 11.69, render/graph/ms 10.23, encode//sum 9.96, stage/main/update 7.47, stage/render/graph_submit 6.96, encode/submit 3.73, encode/finish_wall 3.26 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.sun.gate=on --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=sungate-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 10%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **71%** (min 46%) · 2625 MHz (min 2625) · memory 8101 MHz · 75.2 W · 76 °C (max 82) · VRAM 6758 MiB · throttled 100% of samples

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
- `shadow.sun.gate=on (command line: --set)`
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
