# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: precombine · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1558 frames over 15.0 s · **median 9.61 ms (104 fps)** · mean 9.63 · p95 10.99 · p99 11.84 (1% low 84 fps) · worst 42.06 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 313 frames over 8.1 s, worst 2471.8 ms
- hitches (> 25.0 ms): **1** measured, 36 in warm-up
- timed GPU passes: 8.40 ms of a 9.63 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.61 ms, limit 11 ms
- **FAIL** worst frame 42.06 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.822 | 2.864 | 2.894 | 0.99 | 0.063 |
| dlss_super_resolution | 1.993 | 2.022 | 2.048 | 0.99 | 0.185 |
| early prepass | 0.579 | 0.588 | 0.600 | 0.99 | 0.101 |
| atmosphere_luts | 0.334 | 0.339 | 0.395 | 0.99 | 0.003 |
| bloom | 0.309 | 0.314 | 0.330 | 0.99 | 0.066 |
| main_transmissive_pass_3d | 0.305 | 0.310 | 0.332 | 0.99 | 0.009 |
| water_velocity | 0.237 | 0.240 | 0.264 | 0.99 | 0.002 |
| water_sspr | 0.214 | 0.217 | 0.220 | 0.99 | 0.002 |
| ssao | 0.186 | 0.189 | 0.197 | 0.99 | 0.007 |
| clouds_trace | 0.148 | 0.151 | 0.182 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.144 | 0.146 | 0.151 | 0.99 | 0.001 |
| water_fft | 0.130 | 0.131 | 0.135 | 0.99 | 0.003 |
| uniform_allocation | 0.118 | 0.120 | 0.161 | 0.99 | 0.021 |
| upscaling | 0.112 | 0.113 | 0.119 | 0.99 | 0.004 |
| clouds_composite | 0.110 | 0.112 | 0.114 | 0.99 | 0.002 |
| bin_unpacking | 0.090 | 0.091 | 0.123 | 0.99 | 0.015 |
| tonemapping | 0.085 | 0.086 | 0.092 | 0.99 | 0.004 |
| clustering | 0.063 | 0.064 | 0.077 | 0.99 | 0.021 |
| render_sky | 0.051 | 0.052 | 0.053 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.416 | 0.418 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.052 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.038 | 0.038 | 0.051 | 0.99 | 0.005 |
| clouds_shadow | 0.031 | 0.032 | 0.035 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.220 | 0.230 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.026 | 0.028 | 0.99 | 0.084 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| surface_exposure_far | 0.013 | 0.043 | 0.046 | 0.29 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| surface_exposure | 0.011 | 0.042 | 0.043 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.007 | 0.008 | 0.020 | 0.99 | 0.005 |
| night_sky | 0.006 | 0.006 | 0.008 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| lightprobe_copy | 0.002 | 0.014 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.003 | 0.99 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.003 | 0.004 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.587 | 9.921 | 40.23 | 13378.7 |
| main/busy/ms | 7.900 | 8.816 | 10.34 | 12308.9 |
| render/busy/ms | 7.037 | 8.242 | 39.67 | 10964.4 |
| stage/main/post_update | 4.827 | 5.595 | 7.25 | 7520.2 |
| stage/render/render | 4.653 | 5.834 | 9.81 | 7248.9 |
| render/graph/ms | 4.048 | 5.066 | 7.96 | 6306.2 |
| encode//sum | 3.236 | 5.153 | 14.01 | 5041.1 |
| stage/render/graph_submit | 3.029 | 4.097 | 6.83 | 4718.6 |
| pipelines/compile_ms | 2.212 | 0.000 | 139.86 | 3446.8 |
| pipelines/front_ms | 2.149 | 0.000 | 136.55 | 3348.6 |
| encode/submit | 1.952 | 2.804 | 6.44 | 3041.7 |
| stage/main/update | 1.927 | 2.370 | 4.24 | 3001.9 |
| stage/render/prepare_views | 1.220 | 2.713 | 3.67 | 1900.0 |
| render/acquire/ms | 1.171 | 2.659 | 3.62 | 1825.1 |
| encode/finish_wall | 1.115 | 1.502 | 3.18 | 1736.9 |
| stage/render/camera_driver | 0.963 | 1.121 | 1.36 | 1500.9 |
| encode/pass:early prepass/sum | 0.763 | 1.052 | 1.56 | 1189.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.739 | 1.010 | 2.25 | 1151.7 |
| stage/render/prepare_write_phase_buffers | 0.543 | 0.762 | 1.04 | 846.7 |
| stage/render/prepare_assets | 0.466 | 0.606 | 1.65 | 725.6 |
| bigspace/total/ms | 0.444 | 1.011 | 1.91 | 691.4 |
| stage/render/prepare_resources | 0.395 | 0.455 | 0.78 | 615.9 |
| render/present/ms | 0.378 | 0.477 | 2.05 | 589.2 |
| stage/main/pre_update | 0.359 | 0.475 | 1.00 | 559.1 |
| encode/pass:shadow_directional_light/sum | 0.296 | 1.093 | 1.91 | 461.2 |
| stage/main/first | 0.293 | 0.361 | 1.47 | 457.1 |
| terrain/update/ms | 0.234 | 0.301 | 0.60 | 365.0 |
| stage/render/prepare_bind_groups | 0.217 | 0.304 | 0.44 | 337.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.216 | 0.357 | 1.81 | 337.1 |
| stage/render/prepare_meshes | 0.213 | 0.267 | 4.98 | 331.3 |
| terrain/select/ms | 0.204 | 0.261 | 0.56 | 317.2 |
| stream/drive/ms | 0.198 | 0.279 | 0.69 | 307.8 |
| stage/main/fixed_main_loop | 0.186 | 0.358 | 0.84 | 290.1 |
| stage/render/prepare_batch_phases | 0.181 | 0.241 | 0.42 | 281.6 |
| stage/render/create_views | 0.176 | 0.226 | 0.72 | 273.6 |
| far/select/ms | 0.174 | 0.244 | 0.67 | 271.6 |
| stage/render/extract_commands | 0.172 | 0.203 | 0.71 | 268.1 |
| stream/tick/ms | 0.166 | 0.237 | 0.64 | 258.3 |
| controller/move/ms | 0.161 | 0.303 | 0.80 | 250.2 |
| bigspace/high_precision/ms | 0.157 | 0.600 | 1.61 | 244.5 |
| controller/feed/ms | 0.150 | 0.200 | 0.55 | 233.3 |
| stage/render/prepare_flush | 0.146 | 0.294 | 0.67 | 228.2 |
| stage/main/last | 0.123 | 0.161 | 0.37 | 191.8 |
| stage/render/queue | 0.123 | 0.154 | 10.52 | 191.0 |
| stage/render/specialize | 0.108 | 0.167 | 15.90 | 167.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.104 | 0.172 | 1.83 | 161.6 |
| bigspace/low_precision/ms | 0.099 | 0.255 | 1.49 | 153.8 |
| anim/evaluate/ms | 0.090 | 0.208 | 1.19 | 140.2 |
| pipelines/driver_ms | 0.063 | 0.000 | 4.33 | 98.2 |
| anim/evaluate_cpu/ms | 0.062 | 0.079 | 0.49 | 96.9 |
| stage/render/prepare_collect_phase_buffers | 0.062 | 0.117 | 0.22 | 95.9 |
| encode/resolve_diagnostics/sum | 0.042 | 0.074 | 0.19 | 64.9 |
| weather/map/ms | 0.027 | 0.034 | 0.72 | 41.5 |
| physics/step/ms | 0.026 | 0.055 | 0.49 | 41.2 |
| stage/render/cleanup | 0.024 | 0.041 | 0.30 | 37.3 |
| encode/camera_driver_span_begin/sum | 0.020 | 0.036 | 0.08 | 31.0 |
| stage/render/phase_sort | 0.019 | 0.034 | 0.06 | 29.4 |
| encode/camera_driver_span_end/sum | 0.017 | 0.033 | 0.32 | 25.8 |
| weather/eval/ms | 0.012 | 0.015 | 0.44 | 18.4 |
| script/update/ms | 0.007 | 0.010 | 0.39 | 11.6 |
| controller/avoid/ms | 0.006 | 0.008 | 0.03 | 9.1 |
| script/input/ms | 0.006 | 0.007 | 0.46 | 9.0 |
| script/ui/ms | 0.005 | 0.007 | 0.38 | 8.0 |
| anim/apply/ms | 0.005 | 0.006 | 0.02 | 7.0 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.52 | 2.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 0.9 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1080 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 68.4 k | 68.4 k | 68.4 k |
| encode//max | ms | 0.42 | 2.84 | 0.30 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.08 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.32 | 0.01 |
| encode/finish_critical | ms | 0.84 | 2.84 | 0.82 |
| encode/pass:early prepass/max | ms | 0.75 | 1.56 | 0.82 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.73 | 2.25 | 0.79 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 1.83 | 0.12 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.20 | 1.81 | 0.13 |
| encode/pass:shadow_directional_light/max | ms | 0.08 | 1.91 | 0.07 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.19 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.17 | 39.67 | 9.71 |
| gpu/timed/ms | ms | 9.16 | 10.08 | 9.71 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1943 | 1943 | 1943 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 207.2 k | 207.2 k | 207.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 10.5 k | 10.5 k | 10.5 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 321.65 M | 321.65 M | 321.65 M |
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
| phases/opaque3d/bins | count | 1409 | 1409 | 1409 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 6339 | 6339 | 6339 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/bins | count | 1409 | 1409 | 1409 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 6339 | 6339 | 6339 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 0 | 29 | 0 |
| phases/shadow/bins | count | 2028 | 4311 | 2028 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.6 k | 33.4 k | 10.6 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 22 | 29 | 22 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 320 | 320 | 320 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 28 | 0 |
| preprocess/bind_groups | count | 0 | 76 | 0 |
| preprocess/bind_groups_created | count | 0 | 76 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 33.65 | 48.56 | 31.25 |
| process/mem_usage | GiB | 4.73 | 4.81 | 4.81 |
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
| skin/blocks_changed/count | count | 4 | 6 | 4 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 181 | 181 | 181 |
| skin/joint_bytes_written/bytes | bytes | 309.5 k | 309.5 k | 309.5 k |
| skin/joints_written/count | count | 4836 | 4836 | 4836 |
| skin/upload_writes/count | count | 1 | 1 | 1 |
| started_load_count |  | 4026 | 4028 | 4028 |
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
| stream/latency/s | s | 0.53 | 0.53 | 0.53 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 656 | 656 | 656 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 36.12 | 47.43 | 35.05 |
| system/mem_usage | % | 54.34 | 54.52 | 54.51 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 52 | 81 | 52 |
| views/all/bins | count | 5406 | 7689 | 5406 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 25.1 k | 47.8 k | 25.1 k |
| views/all/pipelines | count | 70 | 77 | 70 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 14.6 k | 48.0 k | 14.6 k |
| views/cascade/batch_sets | count | 0 | 29 | 0 |
| views/cascade/bins | count | 2028 | 4311 | 2028 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 10.6 k | 33.4 k | 10.6 k |
| views/cascade/pipelines | count | 22 | 29 | 22 |
| views/cascade/work_items | count | 0 | 33.4 k | 0 |
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
| views/main/bins | count | 3378 | 3378 | 3378 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 14.4 k | 14.4 k | 14.4 k |
| views/main/pipelines | count | 48 | 48 | 48 |
| views/main/work_items | count | 14.6 k | 14.6 k | 14.6 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 46.1 k | 46.1 k | 46.1 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 2 | 149 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.18 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.54 | 29 | 41.4 k |
| encode/bind_walk/init_textures | count | 5227 | 12.1 k | 8.14 M |
| encode/bind_walk/init_textures_skipped | count | 42.4 k | 52.6 k | 66.06 M |
| encode/bind_walk/inits | count | 336 | 429 | 523.3 k |
| encode/bind_walk/inits_skipped | count | 74.00 | 83 | 115.3 k |
| encode/bind_walk/merge_textures | count | 5163 | 12.0 k | 8.04 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.36 M |
| encode/bind_walk/merges | count | 78.27 | 99 | 121.9 k |
| encode/bind_walk/merges_skipped | count | 54.25 | 58 | 84.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1558 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1558 |
| encode/pass:early prepass/count | count | 1 | 1 | 1558 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1558 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1558 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1558 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1558 |
| encode/queue/command_buffers | count | 36.54 | 39 | 56.9 k |
| encode/queue/submits | count | 2 | 2 | 3116 |
| encode/queue/write_buffer/bytes | bytes | 1.49 M | 5.49 M | 2.32 G |
| encode/queue/write_buffer/calls | count | 212 | 283 | 330.0 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.85 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 14.0 k |
| encode/queue/write_texture/bytes | bytes | 11.5 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1558 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 2 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.03 | 1 | 48 |
| pipelines/created | count | 0.03 | 1 | 48 |
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

## Hitches (> 25.0 ms; slowest 37, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2471.8 | main/busy/ms 2444, stage/main/post_update 2337, stage/main/update 97.87, surface/advance_cpu_ms 66.58, stage/main/pre_update 5.13, stage/main/last 0.98, surface/cell_upload_ms 0.72, stage/main/first 0.66, stage/main/fixed_main_loop 0.50, stream/drive/ms 0.41, weather/map/ms 0.24, bigspace/total/ms 0.11 |
| warm-up | 3.22 | 747.0 | main/busy/ms 2580, stage/main/post_update 2363, render/thread/ms 717, render/busy/ms 547, stage/render/prepare_assets 501, stage/main/update 202, stage/render/prepare_views 165, render/acquire/ms 165, surface/advance_cpu_ms 66.58, stage/render/create_views 27.42, stage/render/render 13.79, render/graph/ms 7.23 |
| warm-up | 3.40 | 179.5 | render/thread/ms 878, render/busy/ms 614, stage/render/prepare_assets 553, stage/render/prepare_views 165, render/acquire/ms 165, main/busy/ms 163, stage/render/render 113, stage/main/update 110, render/present/ms 99.82, pipelines/compile_ms 45.57, pipelines/front_ms 42.39, stage/main/post_update 39.97 |
| warm-up | 3.44 | 39.8 | pipelines/compile_ms 433, pipelines/front_ms 282, render/thread/ms 193, pipelines/driver_ms 151, main/busy/ms 134, stage/render/render 125, render/present/ms 96.42, render/busy/ms 96.37, stage/main/update 86.89, stage/render/prepare_assets 53.20, pipelines/wait_ms 44.14, stage/main/post_update 33.52 |
| warm-up | 3.61 | 174.4 | pipelines/wait_ms 4158, pipelines/compile_ms 2226, pipelines/front_ms 1968, pipelines/driver_ms 258, main/busy/ms 146, render/thread/ms 130, render/busy/ms 126, stage/main/update 87.65, stage/render/prepare_assets 86.87, stage/main/post_update 49.14, stage/render/render 32.14, render/graph/ms 28.08 |
| warm-up | 3.69 | 77.9 | pipelines/wait_ms 4465, pipelines/compile_ms 2841, pipelines/front_ms 2505, pipelines/driver_ms 336, render/thread/ms 174, render/busy/ms 172, stage/render/prepare_assets 148, main/busy/ms 59.82, stage/main/post_update 38.39, stage/render/render 16.46, encode//sum 14.81, terrain/update/ms 14.50 |
| warm-up | 3.72 | 28.2 | pipelines/compile_ms 1031, pipelines/front_ms 804, pipelines/wait_ms 351, pipelines/driver_ms 228, render/thread/ms 102, render/busy/ms 100, stage/render/prepare_assets 78.69, main/busy/ms 30.28, stage/render/render 15.60, encode//sum 14.63, stage/main/post_update 14.42, terrain/update/ms 14.22 |
| warm-up | 3.84 | 38.1 | pipelines/compile_ms 578, pipelines/front_ms 547, pipelines/wait_ms 96.73, main/busy/ms 42.50, stage/main/update 34.28, render/thread/ms 31.96, pipelines/driver_ms 31.10, stream/drive/ms 30.92, render/busy/ms 30.32, stage/render/render 15.95, render/graph/ms 14.16, stage/render/prepare_assets 10.37 |
| warm-up | 3.90 | 25.9 | pipelines/compile_ms 639, pipelines/front_ms 621, main/busy/ms 30.64, stage/main/update 21.60, pipelines/driver_ms 18.10, render/thread/ms 17.96, stream/drive/ms 16.91, render/busy/ms 16.73, stage/render/render 7.46, render/graph/ms 6.08, stage/main/post_update 5.68, stage/render/prepare_assets 5.38 |
| warm-up | 3.95 | 36.7 | main/busy/ms 37.87, render/thread/ms 28.11, render/busy/ms 26.60, stage/main/update 24.30, stream/drive/ms 19.33, stage/render/prepare_assets 11.80, stage/render/render 11.32, render/graph/ms 9.20, stage/main/post_update 8.84, stage/render/graph_submit 6.77, encode//sum 6.57, encode/submit 4.54 |
| warm-up | 3.98 | 28.7 | pipelines/compile_ms 180, pipelines/front_ms 174, render/thread/ms 38.53, render/busy/ms 36.62, main/busy/ms 28.24, stage/render/prepare_assets 20.33, stage/render/render 12.23, stage/main/post_update 11.81, stage/main/update 10.01, render/graph/ms 9.85, encode//sum 7.39, stage/render/graph_submit 7.04 |
| warm-up | 4.01 | 34.5 | pipelines/compile_ms 180, pipelines/front_ms 174, render/thread/ms 48.48, render/busy/ms 46.23, main/busy/ms 34.29, stage/render/prepare_assets 24.49, stage/main/update 14.42, stage/render/render 12.63, stage/main/post_update 11.43, render/graph/ms 9.75, stage/render/graph_submit 7.11, pipelines/driver_ms 6.71 |
| warm-up | 4.06 | 40.4 | pipelines/compile_ms 707, pipelines/front_ms 682, render/thread/ms 50.77, render/busy/ms 48.93, main/busy/ms 46.13, stage/main/update 28.65, stage/render/prepare_assets 27.06, pipelines/driver_ms 25.09, stage/render/render 11.11, render/graph/ms 8.71, stage/main/post_update 7.96, stage/render/graph_submit 6.22 |
| warm-up | 4.09 | 33.2 | pipelines/compile_ms 916, pipelines/front_ms 886, render/thread/ms 44.49, render/busy/ms 43.00, main/busy/ms 39.15, stage/render/prepare_assets 29.84, pipelines/driver_ms 29.45, stage/main/update 24.14, stage/render/render 7.98, stage/main/post_update 6.53, render/graph/ms 5.97, encode//sum 4.98 |
| warm-up | 4.16 | 51.5 | pipelines/compile_ms 136, pipelines/front_ms 130, render/thread/ms 56.63, render/busy/ms 54.93, main/busy/ms 52.39, pipelines/wait_ms 41.54, stage/main/update 29.84, stage/render/prepare_resources 17.03, stage/main/post_update 15.43, stage/render/render 13.05, stage/render/prepare_assets 12.96, render/graph/ms 11.04 |
| warm-up | 4.18 | 25.7 | pipelines/compile_ms 337, pipelines/front_ms 323, pipelines/wait_ms 75.17, render/thread/ms 64.47, render/busy/ms 63.14, main/busy/ms 53.51, stage/main/update 28.86, stage/render/prepare_resources 24.27, stage/main/post_update 17.97, pipelines/driver_ms 13.84, stage/render/prepare_assets 12.96, stage/render/render 9.52 |
| warm-up | 4.25 | 28.4 | pipelines/compile_ms 480, pipelines/front_ms 464, pipelines/wait_ms 108, render/thread/ms 47.97, render/busy/ms 46.32, main/busy/ms 31.68, stage/main/post_update 18.54, stage/render/prepare_resources 16.16, pipelines/driver_ms 15.84, stage/render/render 10.02, stage/main/update 9.78, render/graph/ms 7.80 |
| warm-up | 4.33 | 26.0 | pipelines/compile_ms 1434, pipelines/front_ms 1391, pipelines/wait_ms 45.18, pipelines/driver_ms 42.91, render/thread/ms 39.13, main/busy/ms 38.63, render/busy/ms 37.30, stage/main/post_update 17.15, stage/render/render 15.03, stage/main/update 14.72, render/graph/ms 12.95, stage/render/graph_submit 8.84 |
| warm-up | 4.38 | 50.0 | pipelines/compile_ms 700, pipelines/front_ms 677, render/thread/ms 67.19, render/busy/ms 65.42, main/busy/ms 55.78, pipelines/wait_ms 36.96, stage/main/post_update 29.52, pipelines/driver_ms 22.32, stage/main/update 19.32, stage/render/render 14.87, render/graph/ms 12.61, stage/render/prepare_assets 12.60 |
| warm-up | 4.42 | 44.2 | pipelines/compile_ms 500, pipelines/front_ms 487, render/thread/ms 80.38, render/busy/ms 78.36, main/busy/ms 57.19, pipelines/wait_ms 54.01, stage/main/post_update 31.58, stage/render/prepare_assets 18.90, stage/main/update 18.86, stage/render/render 15.32, stage/render/prepare_resources 14.14, pipelines/driver_ms 13.68 |
| warm-up | 4.48 | 30.4 | render/thread/ms 47.43, render/busy/ms 46.26, main/busy/ms 36.56, stage/main/post_update 17.67, stage/render/prepare_resources 13.81, stage/render/render 10.50, stage/render/prepare_assets 9.83, stage/main/pre_update 9.35, render/graph/ms 9.00, encode//sum 6.88, stage/render/graph_submit 6.62, stage/main/update 5.44 |
| warm-up | 4.53 | 35.9 | pipelines/compile_ms 2204, pipelines/front_ms 2128, pipelines/wait_ms 116, pipelines/driver_ms 76.65, render/thread/ms 53.11, render/busy/ms 51.08, main/busy/ms 31.32, stage/render/prepare_resources 20.80, pipelines/lock_ms 18.58, stage/main/post_update 14.74, stage/render/render 14.04, render/graph/ms 11.49 |
| warm-up | 4.58 | 32.5 | pipelines/compile_ms 226, pipelines/front_ms 219, render/thread/ms 32.95, render/busy/ms 31.71, main/busy/ms 18.30, stage/render/prepare_assets 16.72, stage/render/render 8.89, render/graph/ms 7.24, stage/main/post_update 6.86, stage/main/update 6.86, pipelines/driver_ms 6.80, encode//sum 6.32 |
| warm-up | 4.66 | 44.7 | pipelines/compile_ms 436, pipelines/front_ms 419, pipelines/wait_ms 73.09, main/busy/ms 60.87, stage/main/update 43.98, render/thread/ms 35.02, render/busy/ms 32.63, stream/drive/ms 28.60, pipelines/driver_ms 17.14, stage/render/render 13.07, stage/render/prepare_assets 12.58, stage/main/post_update 11.45 |
| warm-up | 4.70 | 39.4 | pipelines/compile_ms 645, pipelines/front_ms 627, pipelines/wait_ms 179, main/busy/ms 46.65, render/thread/ms 42.83, render/busy/ms 40.68, stage/main/update 35.78, stream/drive/ms 28.40, stage/render/prepare_assets 22.01, pipelines/driver_ms 18.08, stage/render/render 10.89, render/graph/ms 8.18 |
| warm-up | 4.74 | 26.4 | pipelines/compile_ms 800, pipelines/front_ms 776, main/busy/ms 30.72, pipelines/driver_ms 24.04, stage/main/update 18.82, render/thread/ms 18.22, render/busy/ms 16.87, encode//sum 16.86, stream/drive/ms 13.91, stage/render/render 10.60, render/graph/ms 8.96, stage/main/post_update 7.94 |
| warm-up | 5.13 | 32.8 | main/busy/ms 34.53, render/thread/ms 34.41, render/busy/ms 33.18, stage/main/update 18.11, stage/main/post_update 10.93, stage/render/specialize 10.58, stage/render/render 9.26, render/graph/ms 7.43, encode//sum 6.58, stage/render/graph_submit 4.85, stage/render/prepare_meshes 3.45, encode/submit 3.21 |
| warm-up | 5.17 | 35.1 | pipelines/compile_ms 147, pipelines/front_ms 143, main/busy/ms 62.04, render/thread/ms 53.55, render/busy/ms 52.07, stage/main/update 46.85, stream/drive/ms 15.06, stage/render/specialize 13.76, stage/render/render 12.45, stage/render/prepare_assets 10.84, stage/main/post_update 10.25, render/graph/ms 10.08 |
| warm-up | 5.21 | 37.5 | pipelines/compile_ms 184, pipelines/front_ms 178, main/busy/ms 48.86, render/thread/ms 44.96, render/busy/ms 43.73, pipelines/wait_ms 40.18, stage/main/update 36.67, stage/render/prepare_assets 21.09, stream/drive/ms 14.91, stage/render/render 10.10, render/graph/ms 8.25, stage/main/post_update 6.99 |
| warm-up | 5.25 | 36.1 | pipelines/compile_ms 218, pipelines/front_ms 209, main/busy/ms 73.24, stage/main/update 63.26, stream/drive/ms 57.96, pipelines/wait_ms 20.33, render/thread/ms 20.23, render/busy/ms 19.14, stage/render/render 9.33, pipelines/driver_ms 8.57, render/graph/ms 7.79, stage/main/post_update 7.21 |
| warm-up | 5.30 | 43.3 | pipelines/compile_ms 218, pipelines/front_ms 209, main/busy/ms 49.36, stage/main/update 38.36, stream/drive/ms 32.45, render/thread/ms 22.26, render/busy/ms 21.17, pipelines/wait_ms 20.33, pipelines/driver_ms 8.57, stage/render/render 8.08, stage/main/post_update 7.24, stage/render/prepare_assets 6.85 |
| warm-up | 6.05 | 31.3 | pipelines/compile_ms 142, pipelines/front_ms 139, render/thread/ms 31.03, render/busy/ms 30.07, main/busy/ms 18.50, stage/render/prepare_assets 16.50, stage/main/post_update 8.83, stage/render/render 6.96, render/graph/ms 5.69, encode//sum 5.61, stage/main/update 5.32, stage/render/graph_submit 3.83 |
| warm-up | 7.04 | 25.7 | main/busy/ms 28.78, render/thread/ms 23.69, pipelines/compile_ms 19.87, render/busy/ms 19.82, pipelines/front_ms 18.38, stage/main/update 17.17, stream/drive/ms 13.25, stage/main/post_update 8.87, stage/render/render 8.59, render/graph/ms 7.10, encode//sum 6.49, stage/render/prepare_assets 6.47 |
| warm-up | 7.11 | 25.0 | render/thread/ms 27.49, render/busy/ms 26.56, main/busy/ms 26.12, stage/main/post_update 11.26, stage/main/update 11.24, stage/render/prepare_assets 10.59, stage/render/render 9.81, render/graph/ms 8.34, encode//sum 7.88, stage/render/graph_submit 5.89, encode/finish_wall 2.95, encode/submit 2.94 |
| warm-up | 7.25 | 40.2 | render/thread/ms 46.98, render/busy/ms 45.94, main/busy/ms 17.06, stage/render/specialize 13.91, stage/render/render 9.58, stage/main/post_update 9.44, stage/render/queue 9.25, render/graph/ms 8.20, stage/render/graph_submit 6.46, encode//sum 5.96, stage/render/prepare_meshes 5.20, stage/main/update 4.64 |
| warm-up | 7.47 | 30.7 | render/thread/ms 36.04, render/busy/ms 34.28, main/busy/ms 30.48, stage/main/post_update 15.23, stage/render/prepare_assets 14.13, stage/render/render 13.68, render/graph/ms 11.63, stage/main/update 9.78, stage/render/graph_submit 8.62, encode//sum 8.56, encode/submit 5.60, encode/finish_wall 3.10 |
| measure | 0.51 | 42.1 | render/thread/ms 48.80, render/busy/ms 46.29, stage/render/specialize 16.09, main/busy/ms 14.91, stage/render/queue 10.64, stage/render/render 9.44, stage/main/post_update 8.97, render/graph/ms 8.16, stage/render/graph_submit 6.21, encode//sum 5.95, stage/render/prepare_meshes 5.18, encode/submit 3.90 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=noprecomb-1 --bench-baseline none --off precombine
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 16%
- during the run: other processes 2% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **97%** (min 82%) · 2621 MHz (min 2610) · memory 8101 MHz · 99.2 W · 80 °C (max 82) · VRAM 6189 MiB · throttled 100% of samples

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
