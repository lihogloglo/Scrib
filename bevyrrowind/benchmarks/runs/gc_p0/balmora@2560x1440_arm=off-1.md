# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: groundcover · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1287 frames over 15.0 s · **median 11.75 ms (85 fps)** · mean 11.66 · p95 13.12 · p99 13.83 (1% low 72 fps) · worst 17.84 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 317 frames over 8.8 s, worst 2566.1 ms
- hitches (> 29.4 ms): **0** measured, 32 in warm-up
- timed GPU passes: 10.25 ms of a 11.66 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.75 ms, limit 11 ms
- pass worst frame 17.84 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.244 | 3.247 | 3.325 | 1.00 | 0.067 |
| dlss_super_resolution | 2.022 | 2.023 | 2.047 | 1.00 | 0.180 |
| sun_visibility | 0.806 | 0.807 | 0.810 | 1.00 | 0.006 |
| early prepass | 0.647 | 0.648 | 0.656 | 1.00 | 0.093 |
| main_transmissive_pass_3d | 0.401 | 0.402 | 0.413 | 1.00 | 0.008 |
| atmosphere_luts | 0.368 | 0.368 | 0.377 | 1.00 | 0.003 |
| bloom | 0.315 | 0.315 | 0.331 | 1.00 | 0.059 |
| atmos_froxel_inject | 0.309 | 0.310 | 0.309 | 1.00 | 0.001 |
| water_velocity | 0.305 | 0.306 | 0.310 | 1.00 | 0.001 |
| water_sspr | 0.219 | 0.219 | 0.214 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.150 | 0.150 | 0.170 | 1.00 | 0.001 |
| water_fft | 0.134 | 0.134 | 0.135 | 1.00 | 0.002 |
| uniform_allocation | 0.116 | 0.116 | 0.147 | 1.00 | 0.018 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| upscaling | 0.113 | 0.114 | 0.119 | 1.00 | 0.004 |
| bin_unpacking | 0.090 | 0.090 | 0.114 | 1.00 | 0.014 |
| tonemapping | 0.086 | 0.086 | 0.089 | 1.00 | 0.003 |
| gpu_instance_materialization | 0.079 | 0.079 | 0.096 | 1.00 | 0.004 |
| clustering | 0.064 | 0.064 | 0.075 | 1.00 | 0.020 |
| render_sky | 0.062 | 0.062 | 0.062 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.060 | 0.060 | 0.088 | 1.00 | 0.003 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.412 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.030 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.221 | 0.230 | 0.12 | 0.003 |
| water_snapshot | 0.023 | 0.023 | 0.024 | 1.00 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| range_unpacking | 0.022 | 0.022 | 0.028 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.018 | 0.018 | 0.036 | 1.00 | 0.005 |
| main_transparent_pass_3d | 0.017 | 0.017 | 0.026 | 1.00 | 0.192 |
| surface_exposure_far | 0.011 | 0.038 | 0.039 | 0.30 | 0.002 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.002 |
| surface_exposure | 0.011 | 0.039 | 0.040 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.543 | 12.041 | 16.52 | 13568.9 |
| render/busy/ms | 7.044 | 8.170 | 15.27 | 9065.2 |
| main/busy/ms | 6.561 | 7.866 | 12.38 | 8444.2 |
| stage/render/render | 4.728 | 5.693 | 7.47 | 6084.3 |
| encode//sum | 4.527 | 6.106 | 10.61 | 5826.6 |
| render/graph/ms | 4.148 | 5.044 | 6.64 | 5339.0 |
| stage/main/post_update | 3.649 | 4.624 | 9.95 | 4696.6 |
| stage/render/prepare_views | 3.200 | 4.902 | 5.63 | 4118.6 |
| render/acquire/ms | 3.150 | 4.857 | 5.58 | 4053.9 |
| stage/render/graph_submit | 2.999 | 3.811 | 5.36 | 3860.0 |
| encode/submit | 1.852 | 2.273 | 3.32 | 2382.9 |
| pipelines/compile_ms | 1.440 | 0.000 | 132.54 | 1853.8 |
| pipelines/front_ms | 1.401 | 0.000 | 129.77 | 1802.9 |
| stage/main/update | 1.330 | 1.708 | 2.64 | 1711.5 |
| encode/finish_wall | 1.161 | 1.631 | 3.39 | 1494.7 |
| stage/render/camera_driver | 1.078 | 1.253 | 1.76 | 1387.2 |
| encode/pass:shadow_directional_light/sum | 0.745 | 1.314 | 1.90 | 959.0 |
| encode/pass:early prepass/sum | 0.733 | 1.018 | 1.39 | 942.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.702 | 1.010 | 1.36 | 903.0 |
| stage/render/prepare_write_phase_buffers | 0.483 | 0.715 | 0.93 | 621.1 |
| stage/main/fixed_main_loop | 0.452 | 0.689 | 1.33 | 581.2 |
| stage/render/prepare_resources | 0.423 | 0.499 | 0.73 | 544.5 |
| stage/main/pre_update | 0.399 | 0.471 | 0.81 | 514.0 |
| stage/render/prepare_assets | 0.389 | 0.468 | 2.03 | 501.1 |
| render/present/ms | 0.349 | 0.410 | 1.14 | 449.8 |
| bigspace/total/ms | 0.344 | 0.624 | 1.35 | 442.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.321 | 0.450 | 0.66 | 412.8 |
| stage/main/first | 0.312 | 0.370 | 1.70 | 401.4 |
| stage/render/prepare_flush | 0.274 | 0.387 | 0.67 | 352.5 |
| stage/render/prepare_batch_phases | 0.259 | 0.360 | 0.51 | 333.6 |
| stage/render/prepare_bind_groups | 0.251 | 0.339 | 0.58 | 323.1 |
| stage/main/last | 0.249 | 0.347 | 1.45 | 320.1 |
| stage/render/prepare_meshes | 0.212 | 0.265 | 0.98 | 272.8 |
| controller/feed/ms | 0.174 | 0.223 | 0.57 | 224.1 |
| stage/render/extract_commands | 0.173 | 0.204 | 0.28 | 222.8 |
| stage/render/create_views | 0.153 | 0.187 | 0.28 | 196.3 |
| stream/drive/ms | 0.133 | 0.156 | 0.55 | 171.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.119 | 0.176 | 0.30 | 153.6 |
| bigspace/high_precision/ms | 0.115 | 0.276 | 1.04 | 148.5 |
| stream/tick/ms | 0.108 | 0.127 | 0.50 | 138.7 |
| stage/render/queue | 0.103 | 0.136 | 2.38 | 132.6 |
| bigspace/low_precision/ms | 0.098 | 0.243 | 1.09 | 126.2 |
| controller/move/ms | 0.094 | 0.190 | 0.42 | 121.6 |
| stage/render/specialize | 0.076 | 0.107 | 3.34 | 97.3 |
| anim/evaluate/ms | 0.074 | 0.112 | 1.13 | 95.8 |
| anim/evaluate_cpu/ms | 0.061 | 0.074 | 0.44 | 78.3 |
| encode/resolve_diagnostics/sum | 0.057 | 0.082 | 0.26 | 73.7 |
| controller/write/ms | 0.053 | 0.068 | 0.14 | 68.3 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.081 | 0.22 | 59.3 |
| pipelines/driver_ms | 0.040 | 0.000 | 3.80 | 50.9 |
| terrain/update/ms | 0.036 | 0.046 | 0.45 | 45.8 |
| physics/step/ms | 0.030 | 0.052 | 0.11 | 38.8 |
| stage/render/cleanup | 0.026 | 0.034 | 0.47 | 33.3 |
| stage/render/phase_sort | 0.024 | 0.043 | 0.12 | 31.4 |
| encode/camera_driver_span_begin/sum | 0.024 | 0.038 | 0.10 | 31.3 |
| weather/map/ms | 0.022 | 0.030 | 0.05 | 28.2 |
| encode/camera_driver_span_end/sum | 0.022 | 0.034 | 0.08 | 27.7 |
| weather/eval/ms | 0.010 | 0.014 | 0.08 | 13.2 |
| script/update/ms | 0.006 | 0.008 | 0.03 | 7.8 |
| script/input/ms | 0.005 | 0.006 | 0.01 | 6.0 |
| anim/apply/ms | 0.005 | 0.006 | 0.45 | 6.0 |
| controller/avoid/ms | 0.004 | 0.006 | 0.02 | 5.2 |
| far/select/ms | 0.004 | 0.004 | 0.04 | 4.7 |
| script/ui/ms | 0.003 | 0.005 | 0.05 | 3.7 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.02 | 2.3 |
| character/finish/ms | 0.002 | 0.002 | 0.46 | 2.1 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.00 | 1.7 |
| stream/promote/ms | 0.001 | 0.002 | 0.00 | 1.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 1.1 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.02 | 0.8 |
| terrain/select/ms | 0.001 | 0.001 | 0.02 | 0.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.4 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| assets/cutout_decode/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| scene/load/chains/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| scene/load/plan/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| scene/load/wanted/ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| crowd/joints_extracted/count | count | 180 | 181 | 180 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.9 k | 37.9 k | 37.9 k |
| encode//max | ms | 0.56 | 3.23 | 0.52 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.10 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.08 | 0.02 |
| encode/finish_critical | ms | 0.87 | 3.23 | 0.84 |
| encode/pass:early prepass/max | ms | 0.73 | 1.39 | 0.67 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.70 | 1.36 | 0.84 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.30 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.31 | 0.66 | 0.22 |
| encode/pass:shadow_directional_light/max | ms | 0.69 | 1.90 | 0.59 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.26 | 0.10 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 10.87 | 15.27 | 10.78 |
| gpu/timed/ms | ms | 10.87 | 12.26 | 10.78 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1929 | 1929 | 1929 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 214.7 k | 214.7 k | 214.7 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 11.3 k | 11.3 k | 11.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 321.65 M | 321.65 M | 321.65 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d/bins | count | 248 | 248 | 248 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 563 | 563 | 563 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 319 | 319 | 319 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 248 | 248 | 248 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 563 | 563 | 563 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 319 | 319 | 319 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 23 | 23 | 23 |
| phases/opaque3d/bins | count | 876 | 876 | 876 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2784 | 2784 | 2784 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/opaque3d_prepass/bins | count | 876 | 876 | 876 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2784 | 2784 | 2784 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 4 | 41 | 4 |
| phases/shadow/bins | count | 896 | 1509 | 896 |
| phases/shadow/instance_batches | count | 4 | 12 | 4 |
| phases/shadow/instance_draws | count | 5861 | 17.6 k | 5861 |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| phases/shadow/items | count | 3202 | 8161 | 3202 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 27 | 34 | 27 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 332 | 332 | 332 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 14 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 24.04 | 37.74 | 14.42 |
| process/mem_usage | GiB | 3.03 | 3.09 | 3.09 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2386 | 2386 | 2386 |
| scene/batch/masked/live | count | 2386 | 2386 | 2386 |
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
| scene/batch/opaque/high_water | count | 16.8 k | 16.8 k | 16.8 k |
| scene/batch/opaque/live | count | 16.8 k | 16.8 k | 16.8 k |
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
| scene/cells/resident | count | 101 | 101 | 101 |
| scene/chains/high_water | count | 1639 | 1639 | 1639 |
| scene/chains/live | count | 1639 | 1639 | 1639 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 0 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 0 | 0 | 0 |
| shadow_atlas/live_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/live_tiles | count | 0 | 0 | 0 |
| shadow_atlas/mover_draws | count | 0 | 0 | 0 |
| shadow_atlas/movers | count | 0 | 0 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 0 | 0 | 0 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/live_direct | count | 0 | 0 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 0 | 0 | 0 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/static_tiles | count | 0 | 0 | 0 |
| shadow_atlas/views | count | 0 | 0 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 6 | 7 | 6 |
| skin/blocks_skipped/count | count | 175 | 178 | 175 |
| skin/blocks_walked/count | count | 6 | 7 | 6 |
| skin/joint_bytes_written/bytes | bytes | 12.5 k | 12.5 k | 12.5 k |
| skin/joints_written/count | count | 196 | 196 | 196 |
| skin/upload_writes/count | count | 5 | 6 | 5 |
| started_load_count |  | 5288 | 5290 | 5290 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 174 | 174 | 174 |
| statics/materials_bound | count | 174 | 174 | 174 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.05 | 0.05 | 0.05 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 187 | 187 | 187 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.10 | 41.19 | 26.08 |
| system/mem_usage | % | 56.03 | 56.09 | 56.08 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2545 | 2545 | 2545 |
| textures/images | count | 1998 | 1998 | 1998 |
| views/all/batch_sets | count | 64 | 101 | 64 |
| views/all/bins | count | 3144 | 3757 | 3144 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 12 | 20 | 12 |
| views/all/instance_draws | count | 17.6 k | 29.3 k | 17.6 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 147.5 k |
| views/all/items | count | 9408 | 14.4 k | 9408 |
| views/all/pipelines | count | 75 | 82 | 75 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 153.9 k | 260.4 k | 153.9 k |
| views/cascade/batch_sets | count | 4 | 41 | 4 |
| views/cascade/bins | count | 896 | 1509 | 896 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 4 |
| views/cascade/instance_draws | count | 5861 | 17.6 k | 5861 |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| views/cascade/items | count | 3202 | 8161 | 3202 |
| views/cascade/pipelines | count | 27 | 34 | 27 |
| views/cascade/work_items | count | 49.2 k | 155.6 k | 49.2 k |
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
| views/main/batch_sets | count | 60 | 60 | 60 |
| views/main/bins | count | 2248 | 2248 | 2248 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 11.7 k | 11.7 k | 11.7 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6206 | 6206 | 6206 |
| views/main/pipelines | count | 48 | 48 | 48 |
| views/main/work_items | count | 104.8 k | 104.8 k | 104.8 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 15.5 k | 15.5 k | 15.5 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.12 | 3 | 157 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 0.00 | 4 | 4 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 28.55 | 31 | 36.7 k |
| encode/bind_walk/init_textures | count | 7188 | 12.1 k | 9.25 M |
| encode/bind_walk/init_textures_skipped | count | 44.0 k | 52.7 k | 56.62 M |
| encode/bind_walk/inits | count | 504 | 608 | 648.5 k |
| encode/bind_walk/inits_skipped | count | 129 | 140 | 165.7 k |
| encode/bind_walk/merge_textures | count | 7123 | 12.0 k | 9.17 M |
| encode/bind_walk/merge_textures_skipped | count | 9326 | 18.0 k | 12.00 M |
| encode/bind_walk/merges | count | 91.26 | 111 | 117.5 k |
| encode/bind_walk/merges_skipped | count | 115 | 135 | 148.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1287 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1287 |
| encode/pass:early prepass/count | count | 1 | 1 | 1287 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1287 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1287 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1287 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1287 |
| encode/queue/command_buffers | count | 38.55 | 41 | 49.6 k |
| encode/queue/submits | count | 2 | 2 | 2574 |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 3.36 M | 2.42 G |
| encode/queue/write_buffer/calls | count | 234 | 305 | 301.0 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 42.01 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 11.6 k |
| encode/queue/write_texture/bytes | bytes | 20.5 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1287 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.70 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 28 |
| pipelines/created | count | 0.02 | 1 | 28 |
| precache/hit | count | 0 | 0 | 0 |
| precache/missed | count | 0 | 0 | 0 |
| precache/requested | count | 0 | 0 | 0 |
| scene/load/deferred | count | 0 | 0 | 0 |
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

## Hitches (> 29.4 ms; slowest 32, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2566.1 | main/busy/ms 2535, stage/main/post_update 2426, stage/main/update 95.64, surface/advance_cpu_ms 66.95, stage/main/pre_update 10.12, stage/main/last 1.07, surface/cell_upload_ms 0.73, stream/drive/ms 0.44, stage/main/first 0.43, stage/main/fixed_main_loop 0.38, bigspace/total/ms 0.14, weather/map/ms 0.14 |
| warm-up | 3.30 | 735.4 | main/busy/ms 2579, stage/main/post_update 2457, render/thread/ms 672, render/busy/ms 531, stage/render/prepare_assets 470, stage/render/prepare_views 138, render/acquire/ms 138, stage/main/update 102, surface/advance_cpu_ms 66.95, stage/render/create_views 29.01, stage/render/render 24.52, render/graph/ms 19.85 |
| warm-up | 3.39 | 87.8 | render/thread/ms 757, render/busy/ms 556, stage/render/prepare_assets 474, stage/render/prepare_views 139, render/acquire/ms 138, stage/render/render 95.60, main/busy/ms 85.35, render/present/ms 62.47, stage/main/post_update 50.03, render/graph/ms 31.16, stage/render/create_views 29.29, terrain/update/ms 22.38 |
| warm-up | 3.43 | 38.7 | pipelines/compile_ms 294, pipelines/front_ms 213, render/thread/ms 122, pipelines/driver_ms 80.45, stage/render/render 75.38, main/busy/ms 67.13, render/busy/ms 61.10, render/present/ms 60.12, stage/main/post_update 36.71, stage/render/prepare_assets 30.56, terrain/update/ms 29.40, render/graph/ms 13.97 |
| warm-up | 3.46 | 30.5 | pipelines/compile_ms 515, pipelines/front_ms 418, pipelines/wait_ms 320, pipelines/driver_ms 97.47, render/thread/ms 64.74, render/busy/ms 62.08, main/busy/ms 60.85, stage/render/prepare_assets 42.25, stage/main/post_update 29.08, terrain/update/ms 15.45, stage/main/update 11.69, stage/main/first 9.31 |
| warm-up | 3.54 | 81.6 | pipelines/wait_ms 1887, pipelines/compile_ms 773, pipelines/front_ms 727, render/thread/ms 101, render/busy/ms 96.63, main/busy/ms 50.18, pipelines/driver_ms 45.03, stage/render/prepare_assets 32.81, stage/main/post_update 20.18, stage/render/prepare_resources 17.82, stage/render/prepare_bind_groups 16.35, stage/render/prepare_write_phase_buffers 15.80 |
| warm-up | 3.63 | 86.3 | pipelines/wait_ms 5622, pipelines/compile_ms 1648, pipelines/front_ms 1538, render/thread/ms 157, render/busy/ms 154, main/busy/ms 114, pipelines/driver_ms 110, stage/main/update 86.49, stage/render/prepare_resources 44.75, stage/render/prepare_bind_groups 44.13, stage/render/render 21.87, stage/render/prepare_assets 21.37 |
| warm-up | 3.71 | 86.4 | pipelines/wait_ms 4866, pipelines/compile_ms 2133, pipelines/front_ms 2012, render/thread/ms 152, render/busy/ms 151, pipelines/driver_ms 122, main/busy/ms 120, stage/main/update 89.86, stage/render/prepare_assets 60.11, stage/render/prepare_resources 29.33, stage/render/prepare_bind_groups 28.25, stage/render/render 24.45 |
| warm-up | 3.75 | 37.1 | pipelines/compile_ms 1605, pipelines/front_ms 1533, pipelines/wait_ms 989, render/thread/ms 103, render/busy/ms 101, pipelines/driver_ms 72.20, stage/render/prepare_assets 70.61, main/busy/ms 56.62, stage/main/post_update 25.43, stage/render/render 21.71, render/graph/ms 19.06, stage/main/first 13.05 |
| warm-up | 3.84 | 31.2 | pipelines/compile_ms 84.76, pipelines/front_ms 81.01, render/thread/ms 43.06, render/busy/ms 41.40, main/busy/ms 27.04, stage/main/update 13.05, stage/render/render 12.22, stage/render/prepare_assets 10.15, render/graph/ms 10.02, stage/render/prepare_resources 9.84, stage/main/post_update 9.72, encode//sum 7.81 |
| warm-up | 3.88 | 41.8 | pipelines/compile_ms 442, pipelines/front_ms 423, render/thread/ms 70.08, render/busy/ms 67.93, pipelines/wait_ms 50.96, main/busy/ms 48.31, stage/render/prepare_assets 22.41, stage/main/update 21.78, pipelines/driver_ms 19.31, stage/main/post_update 19.02, stage/render/render 17.10, render/graph/ms 14.34 |
| warm-up | 3.95 | 71.8 | pipelines/compile_ms 1564, pipelines/front_ms 1486, pipelines/wait_ms 99.22, render/thread/ms 97.86, render/busy/ms 95.10, pipelines/driver_ms 78.08, stage/render/prepare_assets 53.27, main/busy/ms 49.09, stage/render/render 25.59, stage/main/update 23.28, render/graph/ms 22.02, stage/render/graph_submit 17.83 |
| warm-up | 4.00 | 48.8 | pipelines/compile_ms 1507, pipelines/front_ms 1434, render/thread/ms 101, render/busy/ms 98.68, pipelines/driver_ms 73.56, stage/render/prepare_assets 67.47, pipelines/wait_ms 56.89, stage/render/render 23.56, main/busy/ms 22.49, render/graph/ms 20.24, stage/render/graph_submit 16.48, stage/main/update 8.57 |
| warm-up | 4.07 | 73.0 | pipelines/compile_ms 616, pipelines/front_ms 593, render/thread/ms 114, render/busy/ms 112, stage/render/prepare_assets 61.85, stage/render/render 44.09, render/graph/ms 41.52, stage/render/graph_submit 38.07, encode//sum 35.00, encode/finish_wall 31.27, encode/pass:main_transmissive_pass_3d/sum 27.15, encode/pass:main_transparent_pass_3d/sum 27.09 |
| warm-up | 4.10 | 30.4 | pipelines/compile_ms 266, pipelines/front_ms 256, render/thread/ms 98.61, render/busy/ms 97.27, stage/render/prepare_assets 53.62, stage/render/render 39.35, render/graph/ms 37.51, stage/render/graph_submit 34.55, encode//sum 34.46, encode/finish_wall 28.15, encode/pass:main_transparent_pass_3d/sum 27.07, encode/pass:main_transmissive_pass_3d/sum 27.04 |
| warm-up | 4.18 | 67.1 | pipelines/compile_ms 1673, pipelines/front_ms 1448, encode//sum 367, pipelines/driver_ms 225, render/thread/ms 69.83, main/busy/ms 69.33, render/busy/ms 68.45, stage/render/render 63.17, render/graph/ms 61.56, stage/main/update 60.82, stage/render/graph_submit 58.83, encode/finish_wall 52.80 |
| warm-up | 4.27 | 30.0 | render/thread/ms 44.28, render/busy/ms 42.43, main/busy/ms 26.10, stage/render/prepare_assets 22.42, stage/render/render 14.98, stage/main/update 12.87, render/graph/ms 12.44, encode//sum 9.25, stage/render/graph_submit 8.83, stage/main/post_update 7.84, encode/submit 5.97, stage/render/camera_driver 3.37 |
| warm-up | 4.30 | 29.6 | render/thread/ms 54.67, render/busy/ms 52.98, main/busy/ms 36.48, stage/render/prepare_assets 31.69, stage/render/render 15.32, stage/main/update 14.45, render/graph/ms 12.65, stage/main/post_update 11.91, encode//sum 9.49, stage/render/graph_submit 8.47, encode/submit 5.58, stage/render/camera_driver 3.94 |
| warm-up | 4.34 | 38.4 | render/thread/ms 64.24, render/busy/ms 62.66, main/busy/ms 46.48, stage/render/prepare_assets 35.87, stage/main/update 22.05, stage/render/render 18.97, render/graph/ms 16.73, stage/main/post_update 14.21, stage/render/graph_submit 12.71, pipelines/compile_ms 8.31, encode/submit 8.14, pipelines/front_ms 7.93 |
| warm-up | 4.41 | 41.1 | pipelines/compile_ms 604, pipelines/front_ms 572, render/thread/ms 65.18, render/busy/ms 63.46, main/busy/ms 46.87, stage/main/update 33.38, pipelines/driver_ms 31.85, stage/render/prepare_assets 24.01, stage/render/render 21.79, render/graph/ms 20.02, stage/render/camera_driver 10.24, stage/render/prepare_resources 9.66 |
| warm-up | 4.44 | 32.6 | pipelines/compile_ms 571, pipelines/front_ms 541, render/thread/ms 69.39, render/busy/ms 64.81, main/busy/ms 47.75, pipelines/driver_ms 30.75, stage/main/update 26.30, stage/render/render 20.14, render/graph/ms 18.36, stage/render/prepare_assets 16.54, stage/main/post_update 12.74, stage/render/prepare_resources 10.57 |
| warm-up | 4.50 | 58.1 | pipelines/compile_ms 113, pipelines/front_ms 108, render/thread/ms 87.66, render/busy/ms 82.14, main/busy/ms 69.24, stage/main/update 40.31, stage/render/prepare_assets 27.15, stage/render/render 20.12, stage/main/post_update 18.70, render/graph/ms 17.17, stage/render/graph_submit 14.14, encode//sum 9.44 |
| warm-up | 4.59 | 97.1 | pipelines/compile_ms 1406, pipelines/front_ms 1361, pipelines/wait_ms 254, render/thread/ms 149, render/busy/ms 146, main/busy/ms 130, stage/main/update 90.48, stage/render/prepare_assets 44.91, pipelines/driver_ms 44.90, stage/render/prepare_resources 30.92, stage/render/render 27.76, render/graph/ms 23.92 |
| warm-up | 4.69 | 96.6 | pipelines/compile_ms 2806, pipelines/front_ms 2730, pipelines/wait_ms 474, render/thread/ms 186, render/busy/ms 184, main/busy/ms 123, stage/main/update 78.94, stage/render/prepare_assets 76.00, pipelines/driver_ms 75.89, stage/render/prepare_write_phase_buffers 27.98, stage/render/prepare_resources 26.39, stage/main/post_update 24.84 |
| warm-up | 4.74 | 54.6 | pipelines/compile_ms 2156, pipelines/front_ms 2102, pipelines/wait_ms 298, render/thread/ms 145, render/busy/ms 144, stage/render/prepare_assets 69.44, main/busy/ms 69.03, pipelines/driver_ms 54.10, stage/render/prepare_write_phase_buffers 28.67, stage/main/update 25.45, stage/render/render 21.96, render/graph/ms 19.97 |
| warm-up | 4.79 | 42.8 | pipelines/compile_ms 952, pipelines/front_ms 907, pipelines/wait_ms 167, render/thread/ms 92.76, render/busy/ms 91.44, main/busy/ms 76.78, pipelines/driver_ms 44.95, stage/render/prepare_assets 30.38, stage/render/render 24.68, stage/main/update 23.40, render/graph/ms 22.75, stage/main/post_update 19.88 |
| warm-up | 4.87 | 80.5 | pipelines/compile_ms 617, pipelines/front_ms 583, render/thread/ms 118, render/busy/ms 117, pipelines/wait_ms 90.75, main/busy/ms 74.21, stage/render/prepare_assets 34.83, pipelines/driver_ms 33.70, stage/main/update 33.37, stage/render/prepare_write_phase_buffers 32.90, stage/main/post_update 19.63, stage/render/render 15.61 |
| warm-up | 4.91 | 38.0 | pipelines/compile_ms 1448, pipelines/front_ms 1400, render/thread/ms 103, render/busy/ms 101, main/busy/ms 60.14, pipelines/driver_ms 47.89, stage/main/update 37.96, stage/render/prepare_write_phase_buffers 32.81, stage/render/prepare_assets 27.96, stage/render/render 11.22, stage/main/post_update 10.69, stage/render/prepare_meshes 10.15 |
| warm-up | 5.21 | 62.6 | pipelines/compile_ms 378, pipelines/front_ms 364, render/thread/ms 84.88, render/busy/ms 83.52, main/busy/ms 60.02, stage/main/post_update 32.58, stage/main/update 20.06, stage/render/queue 16.84, stage/render/prepare_write_phase_buffers 14.71, pipelines/driver_ms 14.31, stage/render/prepare_assets 13.73, stage/render/render 12.44 |
| warm-up | 5.44 | 32.5 | render/thread/ms 52.22, render/busy/ms 50.32, main/busy/ms 44.68, stage/main/update 21.26, stage/render/render 19.74, stage/render/prepare_assets 17.96, render/graph/ms 17.04, stage/main/post_update 14.44, stage/render/graph_submit 12.19, encode//sum 9.76, encode/submit 7.70, encode/finish_wall 4.59 |
| warm-up | 5.47 | 37.8 | render/thread/ms 65.12, render/busy/ms 63.50, main/busy/ms 57.83, stage/render/prepare_assets 38.62, stage/main/update 30.05, stage/main/post_update 15.29, stage/render/render 13.10, render/graph/ms 10.85, encode//sum 7.56, stage/render/graph_submit 7.47, stream/drive/ms 6.08, stage/main/pre_update 5.25 |
| warm-up | 6.56 | 54.7 | render/thread/ms 63.26, render/busy/ms 57.86, stage/render/render 51.43, render/graph/ms 50.07, stage/render/graph_submit 47.68, encode/submit 45.71, main/busy/ms 16.56, stage/main/post_update 7.46, encode//sum 6.78, stage/main/update 5.42, stage/render/prepare_views 4.64, render/acquire/ms 4.54 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\gc_p0\balmora@2560x1440_arm=off-1 --bench-baseline none --off groundcover
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 5%, CPU 7%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 91.0 W · 75 °C (max 77) · VRAM 5429 MiB · throttled 100% of samples

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
