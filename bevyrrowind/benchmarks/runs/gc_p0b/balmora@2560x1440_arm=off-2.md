# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: groundcover · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1291 frames over 15.0 s · **median 11.71 ms (85 fps)** · mean 11.63 · p95 12.99 · p99 13.43 (1% low 74 fps) · worst 23.18 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 313 frames over 8.7 s, worst 2571.3 ms
- hitches (> 29.3 ms): **0** measured, 26 in warm-up
- timed GPU passes: 10.22 ms of a 11.63 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.71 ms, limit 11 ms
- pass worst frame 23.18 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.238 | 3.241 | 3.335 | 1.00 | 0.063 |
| dlss_super_resolution | 2.022 | 2.024 | 2.045 | 1.00 | 0.182 |
| sun_visibility | 0.782 | 0.782 | 0.807 | 1.00 | 0.006 |
| early prepass | 0.650 | 0.650 | 0.657 | 1.00 | 0.094 |
| main_transmissive_pass_3d | 0.401 | 0.401 | 0.412 | 1.00 | 0.008 |
| atmosphere_luts | 0.373 | 0.373 | 0.380 | 1.00 | 0.003 |
| bloom | 0.314 | 0.314 | 0.331 | 1.00 | 0.060 |
| atmos_froxel_inject | 0.310 | 0.310 | 0.309 | 1.00 | 0.001 |
| water_velocity | 0.307 | 0.307 | 0.311 | 1.00 | 0.001 |
| water_sspr | 0.213 | 0.213 | 0.213 | 1.00 | 0.002 |
| ssao | 0.188 | 0.188 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.151 | 0.151 | 0.171 | 1.00 | 0.001 |
| water_fft | 0.136 | 0.136 | 0.138 | 1.00 | 0.002 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| uniform_allocation | 0.116 | 0.116 | 0.147 | 1.00 | 0.018 |
| upscaling | 0.115 | 0.116 | 0.121 | 1.00 | 0.004 |
| bin_unpacking | 0.091 | 0.091 | 0.114 | 1.00 | 0.013 |
| tonemapping | 0.086 | 0.086 | 0.089 | 1.00 | 0.003 |
| gpu_instance_materialization | 0.072 | 0.072 | 0.096 | 1.00 | 0.004 |
| clustering | 0.063 | 0.063 | 0.074 | 1.00 | 0.019 |
| render_sky | 0.062 | 0.062 | 0.063 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.057 | 0.057 | 0.081 | 1.00 | 0.003 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.413 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.030 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.222 | 0.232 | 0.12 | 0.003 |
| water_snapshot | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| range_unpacking | 0.022 | 0.022 | 0.028 | 1.00 | 0.003 |
| water_mips | 0.022 | 0.022 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.019 | 0.019 | 0.037 | 1.00 | 0.005 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.025 | 1.00 | 0.188 |
| surface_exposure_far | 0.012 | 0.040 | 0.042 | 0.30 | 0.002 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.001 |
| surface_exposure | 0.011 | 0.040 | 0.040 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.502 | 11.883 | 22.07 | 13558.1 |
| render/busy/ms | 6.987 | 8.029 | 16.92 | 9020.4 |
| main/busy/ms | 6.511 | 7.715 | 8.82 | 8406.3 |
| stage/render/render | 4.673 | 5.468 | 14.81 | 6033.5 |
| encode//sum | 4.542 | 5.806 | 15.66 | 5863.2 |
| render/graph/ms | 4.089 | 4.813 | 14.18 | 5278.9 |
| stage/main/post_update | 3.635 | 4.566 | 5.66 | 4692.9 |
| stage/render/prepare_views | 3.209 | 4.814 | 5.82 | 4143.4 |
| render/acquire/ms | 3.159 | 4.763 | 5.78 | 4078.4 |
| stage/render/graph_submit | 2.948 | 3.652 | 12.99 | 3805.4 |
| pipelines/compile_ms | 2.410 | 0.000 | 120.22 | 3111.0 |
| pipelines/front_ms | 2.339 | 0.000 | 117.12 | 3019.6 |
| encode/submit | 1.817 | 2.203 | 2.95 | 2346.0 |
| stage/main/update | 1.312 | 1.650 | 2.25 | 1694.1 |
| encode/finish_wall | 1.147 | 1.554 | 10.20 | 1480.6 |
| stage/render/camera_driver | 1.071 | 1.241 | 1.52 | 1382.3 |
| encode/pass:shadow_directional_light/sum | 0.728 | 1.289 | 1.77 | 940.4 |
| encode/pass:early prepass/sum | 0.723 | 0.995 | 9.13 | 933.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.686 | 0.974 | 1.58 | 885.8 |
| stage/render/prepare_write_phase_buffers | 0.484 | 0.707 | 1.02 | 624.7 |
| stage/render/prepare_resources | 0.427 | 0.507 | 0.67 | 551.4 |
| stage/main/fixed_main_loop | 0.427 | 0.644 | 1.62 | 550.8 |
| stage/main/pre_update | 0.398 | 0.467 | 0.72 | 513.5 |
| stage/render/prepare_assets | 0.388 | 0.463 | 1.58 | 500.9 |
| render/present/ms | 0.356 | 0.418 | 0.76 | 459.3 |
| bigspace/total/ms | 0.341 | 0.640 | 1.51 | 439.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.317 | 0.435 | 0.70 | 408.9 |
| stage/main/first | 0.316 | 0.376 | 1.77 | 407.9 |
| stage/render/prepare_flush | 0.270 | 0.377 | 0.80 | 348.4 |
| stage/render/prepare_batch_phases | 0.255 | 0.358 | 0.62 | 328.9 |
| stage/main/last | 0.251 | 0.362 | 0.80 | 323.5 |
| stage/render/prepare_bind_groups | 0.247 | 0.336 | 0.46 | 319.0 |
| stage/render/prepare_meshes | 0.211 | 0.266 | 1.10 | 272.5 |
| controller/feed/ms | 0.180 | 0.231 | 0.66 | 232.2 |
| stage/render/extract_commands | 0.172 | 0.205 | 0.28 | 222.4 |
| stage/render/create_views | 0.155 | 0.188 | 0.32 | 200.1 |
| stream/drive/ms | 0.133 | 0.153 | 0.57 | 171.5 |
| bigspace/high_precision/ms | 0.121 | 0.303 | 0.84 | 156.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.117 | 0.175 | 0.29 | 151.2 |
| stream/tick/ms | 0.107 | 0.124 | 0.28 | 138.0 |
| stage/render/queue | 0.103 | 0.139 | 2.24 | 132.7 |
| bigspace/low_precision/ms | 0.091 | 0.209 | 1.05 | 117.9 |
| stage/render/specialize | 0.082 | 0.138 | 3.50 | 106.0 |
| controller/move/ms | 0.074 | 0.149 | 0.24 | 96.0 |
| pipelines/driver_ms | 0.071 | 0.000 | 4.64 | 91.4 |
| anim/evaluate/ms | 0.070 | 0.104 | 0.82 | 90.7 |
| anim/evaluate_cpu/ms | 0.060 | 0.074 | 0.50 | 77.0 |
| encode/resolve_diagnostics/sum | 0.057 | 0.085 | 0.16 | 73.2 |
| controller/write/ms | 0.050 | 0.064 | 0.12 | 64.0 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.081 | 0.28 | 58.9 |
| terrain/update/ms | 0.035 | 0.047 | 0.40 | 45.0 |
| physics/step/ms | 0.032 | 0.052 | 0.49 | 40.7 |
| stage/render/phase_sort | 0.030 | 0.051 | 0.12 | 38.4 |
| stage/render/cleanup | 0.025 | 0.034 | 0.28 | 32.9 |
| encode/camera_driver_span_begin/sum | 0.023 | 0.035 | 0.10 | 29.9 |
| weather/map/ms | 0.023 | 0.031 | 0.07 | 29.5 |
| encode/camera_driver_span_end/sum | 0.021 | 0.034 | 0.18 | 26.8 |
| weather/eval/ms | 0.010 | 0.014 | 0.03 | 13.1 |
| script/update/ms | 0.006 | 0.008 | 0.02 | 7.9 |
| script/input/ms | 0.005 | 0.006 | 0.01 | 6.4 |
| anim/apply/ms | 0.005 | 0.006 | 0.03 | 5.9 |
| controller/avoid/ms | 0.004 | 0.006 | 0.03 | 4.8 |
| far/select/ms | 0.003 | 0.004 | 0.03 | 4.5 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 3.9 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.02 | 2.2 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| character/finish/ms | 0.001 | 0.002 | 0.02 | 1.5 |
| stream/promote/ms | 0.001 | 0.001 | 0.01 | 1.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 1.0 |
| pipelines/wait_ms | 0.001 | 0.000 | 0.03 | 0.8 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.01 | 0.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| terrain/select/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| weather/apply/ms | 0.000 | 0.000 | 0.03 | 0.4 |
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
| crowd/c1/count | count | 122 | 123 | 123 |
| crowd/c1/joints/count | count | 4848 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1080 | 1088 | 1088 |
| crowd/c2/count | count | 284 | 284 | 283 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 147 | 181 | 147 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.9 k | 37.9 k | 37.9 k |
| encode//max | ms | 0.56 | 9.84 | 0.47 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.10 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.18 | 0.02 |
| encode/finish_critical | ms | 0.86 | 9.84 | 0.77 |
| encode/pass:early prepass/max | ms | 0.72 | 9.13 | 0.77 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.69 | 1.58 | 0.54 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.29 | 0.13 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.31 | 0.70 | 0.35 |
| encode/pass:shadow_directional_light/max | ms | 0.68 | 1.77 | 0.73 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.16 | 0.04 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 10.82 | 16.92 | 10.78 |
| gpu/timed/ms | ms | 10.82 | 12.13 | 10.78 |
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
| phases/opaque3d/bins | count | 875 | 875 | 875 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2784 | 2784 | 2784 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/opaque3d_prepass/bins | count | 875 | 875 | 875 |
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
| precache/throttled | count | 0 | 28 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 23.22 | 46.15 | 21.53 |
| process/mem_usage | GiB | 3.00 | 3.05 | 3.05 |
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
| skin/blocks_skipped/count | count | 175 | 179 | 175 |
| skin/blocks_walked/count | count | 6 | 7 | 6 |
| skin/joint_bytes_written/bytes | bytes | 12.5 k | 12.5 k | 10.2 k |
| skin/joints_written/count | count | 196 | 196 | 160 |
| skin/upload_writes/count | count | 6 | 6 | 6 |
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
| system/cpu_usage | % | 28.91 | 37.79 | 24.57 |
| system/mem_usage | % | 49.85 | 50.02 | 50.02 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2545 | 2545 | 2545 |
| textures/images | count | 1998 | 1998 | 1998 |
| views/all/batch_sets | count | 64 | 101 | 64 |
| views/all/bins | count | 3142 | 3755 | 3142 |
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
| views/main/bins | count | 2246 | 2246 | 2246 |
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
| anim/events/count | count | 0.13 | 2 | 165 |
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
| encode//count | count | 28.55 | 31 | 36.9 k |
| encode/bind_walk/init_textures | count | 7188 | 12.1 k | 9.28 M |
| encode/bind_walk/init_textures_skipped | count | 37.7 k | 45.0 k | 48.72 M |
| encode/bind_walk/inits | count | 504 | 608 | 650.5 k |
| encode/bind_walk/inits_skipped | count | 124 | 134 | 159.4 k |
| encode/bind_walk/merge_textures | count | 7123 | 12.0 k | 9.20 M |
| encode/bind_walk/merge_textures_skipped | count | 6436 | 12.2 k | 8.31 M |
| encode/bind_walk/merges | count | 91.26 | 111 | 117.8 k |
| encode/bind_walk/merges_skipped | count | 110 | 127 | 142.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1291 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1291 |
| encode/pass:early prepass/count | count | 1 | 1 | 1291 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1291 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1291 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1291 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1291 |
| encode/queue/command_buffers | count | 38.55 | 41 | 49.8 k |
| encode/queue/submits | count | 2 | 2 | 2582 |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 3.36 M | 2.42 G |
| encode/queue/write_buffer/calls | count | 234 | 305 | 302.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 42.14 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 11.6 k |
| encode/queue/write_texture/bytes | bytes | 20.4 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1291 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.70 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.04 | 1 | 48 |
| pipelines/created | count | 0.04 | 1 | 48 |
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

## Hitches (> 29.3 ms; slowest 26, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2571.3 | main/busy/ms 2541, stage/main/post_update 2423, stage/main/update 104, surface/advance_cpu_ms 70.89, stage/main/pre_update 9.57, stage/main/last 1.42, surface/cell_upload_ms 0.81, stream/drive/ms 0.70, stage/main/first 0.51, stage/main/fixed_main_loop 0.42, weather/map/ms 0.29, stream/tick/ms 0.18 |
| warm-up | 3.31 | 739.7 | main/busy/ms 2587, stage/main/post_update 2456, render/thread/ms 675, render/busy/ms 523, stage/render/prepare_assets 461, stage/render/prepare_views 149, render/acquire/ms 148, stage/main/update 111, surface/advance_cpu_ms 70.89, stage/render/create_views 31.14, stage/render/render 22.47, render/graph/ms 17.14 |
| warm-up | 3.42 | 109.7 | render/thread/ms 782, render/busy/ms 553, stage/render/prepare_assets 470, stage/render/prepare_views 149, render/acquire/ms 149, main/busy/ms 111, stage/render/render 109, render/present/ms 79.66, stage/main/post_update 53.72, stage/render/create_views 31.45, stage/main/last 30.64, render/graph/ms 27.00 |
| warm-up | 3.47 | 53.9 | pipelines/compile_ms 405, pipelines/front_ms 286, render/thread/ms 157, pipelines/driver_ms 119, stage/render/render 103, main/busy/ms 90.63, render/busy/ms 79.40, render/present/ms 77.22, pipelines/wait_ms 67.10, stage/main/post_update 36.50, stage/render/prepare_assets 35.71, stage/main/last 31.40 |
| warm-up | 3.61 | 135.7 | pipelines/wait_ms 4202, pipelines/compile_ms 1709, pipelines/front_ms 1521, pipelines/driver_ms 188, render/thread/ms 171, render/busy/ms 168, stage/render/prepare_write_phase_buffers 80.19, main/busy/ms 59.08, stage/render/prepare_assets 44.71, stage/main/post_update 40.19, stage/render/render 22.29, render/graph/ms 18.45 |
| warm-up | 3.68 | 66.2 | pipelines/wait_ms 4263, pipelines/compile_ms 1976, pipelines/front_ms 1872, render/thread/ms 184, render/busy/ms 181, pipelines/driver_ms 104, main/busy/ms 104, stage/render/prepare_write_phase_buffers 83.57, stage/main/update 58.89, stage/render/prepare_assets 41.15, stage/main/post_update 35.89, stage/render/render 26.08 |
| warm-up | 3.82 | 140.5 | pipelines/compile_ms 2601, pipelines/front_ms 2379, pipelines/wait_ms 323, pipelines/driver_ms 222, render/thread/ms 192, render/busy/ms 184, stage/render/prepare_assets 94.57, main/busy/ms 86.61, stage/main/update 61.17, stage/render/prepare_flush 37.15, stage/render/render 34.93, render/graph/ms 26.43 |
| warm-up | 3.95 | 52.3 | pipelines/compile_ms 431, pipelines/front_ms 414, pipelines/wait_ms 105, render/thread/ms 64.97, render/busy/ms 62.34, main/busy/ms 46.44, stage/render/prepare_assets 36.69, stage/main/update 20.00, pipelines/driver_ms 17.38, stage/main/post_update 17.34, stage/render/render 16.96, render/graph/ms 13.91 |
| warm-up | 4.00 | 44.5 | pipelines/compile_ms 89.40, render/thread/ms 85.74, pipelines/front_ms 85.71, render/busy/ms 82.96, stage/render/prepare_assets 55.19, main/busy/ms 54.67, stage/main/update 18.83, stage/render/render 18.63, stage/main/post_update 17.74, render/graph/ms 15.06, stage/main/pre_update 12.36, stage/render/graph_submit 11.28 |
| warm-up | 4.07 | 69.5 | pipelines/compile_ms 1316, pipelines/front_ms 1254, render/thread/ms 107, render/busy/ms 105, stage/render/prepare_assets 77.92, pipelines/driver_ms 62.15, main/busy/ms 42.36, stage/render/render 17.86, render/graph/ms 15.04, stage/main/post_update 12.80, stage/main/pre_update 11.80, stage/main/update 9.61 |
| warm-up | 4.13 | 63.3 | pipelines/compile_ms 1623, pipelines/front_ms 1553, render/thread/ms 129, render/busy/ms 127, stage/render/prepare_assets 79.81, pipelines/driver_ms 70.10, stage/render/prepare_resources 28.29, main/busy/ms 24.90, pipelines/wait_ms 18.03, pipelines/lock_ms 16.51, stage/render/render 14.61, render/graph/ms 12.64 |
| warm-up | 4.16 | 34.5 | pipelines/compile_ms 810, pipelines/front_ms 790, render/thread/ms 95.46, render/busy/ms 94.16, stage/render/prepare_assets 53.51, stage/render/prepare_resources 27.75, main/busy/ms 26.33, pipelines/driver_ms 20.26, pipelines/wait_ms 16.85, pipelines/lock_ms 16.29, stage/main/post_update 11.14, stage/main/update 10.74 |
| warm-up | 4.32 | 38.3 | render/thread/ms 58.07, render/busy/ms 55.78, stage/render/prepare_assets 35.35, main/busy/ms 27.80, stage/render/render 15.25, stage/main/update 14.22, render/graph/ms 12.40, encode//sum 9.12, stage/render/graph_submit 8.47, stage/main/post_update 8.03, encode/submit 5.79, stage/render/camera_driver 3.61 |
| warm-up | 4.41 | 64.6 | pipelines/compile_ms 408, pipelines/front_ms 392, render/thread/ms 81.06, render/busy/ms 79.90, stage/render/prepare_assets 43.65, main/busy/ms 33.50, stage/render/prepare_resources 26.22, stage/main/update 17.58, pipelines/driver_ms 16.05, stage/main/post_update 8.80, stage/render/render 7.96, encode//sum 6.69 |
| warm-up | 4.50 | 82.3 | pipelines/compile_ms 331, pipelines/front_ms 318, main/busy/ms 74.59, stage/main/update 56.92, render/thread/ms 56.80, render/busy/ms 54.87, stage/render/prepare_assets 29.88, stage/render/render 15.14, character/finish/ms 15.04, pipelines/driver_ms 13.62, render/graph/ms 12.88, stage/main/post_update 10.46 |
| warm-up | 4.61 | 110.3 | pipelines/compile_ms 715, pipelines/front_ms 682, pipelines/wait_ms 381, render/thread/ms 142, render/busy/ms 140, main/busy/ms 129, stage/render/prepare_assets 96.99, stage/main/update 85.45, pipelines/driver_ms 32.70, stage/main/post_update 23.89, stage/render/render 18.37, render/graph/ms 16.03 |
| warm-up | 4.73 | 119.2 | pipelines/compile_ms 2742, pipelines/front_ms 2661, pipelines/wait_ms 637, render/thread/ms 212, render/busy/ms 210, main/busy/ms 141, stage/render/prepare_assets 129, pipelines/driver_ms 80.59, stage/main/update 74.34, stage/render/prepare_write_phase_buffers 34.40, stage/main/post_update 34.17, stage/render/render 24.14 |
| warm-up | 4.76 | 32.6 | pipelines/compile_ms 2671, pipelines/front_ms 2596, pipelines/wait_ms 295, render/thread/ms 148, render/busy/ms 146, pipelines/driver_ms 75.51, main/busy/ms 72.48, stage/render/prepare_assets 65.24, stage/render/prepare_write_phase_buffers 30.48, stage/main/update 28.80, stage/render/render 27.39, render/graph/ms 25.26 |
| warm-up | 4.80 | 32.4 | pipelines/compile_ms 333, pipelines/front_ms 319, render/thread/ms 61.72, render/busy/ms 59.92, main/busy/ms 51.20, pipelines/wait_ms 38.33, stage/main/update 30.02, stage/render/prepare_assets 20.35, stage/render/render 19.40, render/graph/ms 17.34, pipelines/driver_ms 14.47, stage/render/graph_submit 14.12 |
| warm-up | 4.83 | 37.7 | pipelines/compile_ms 243, pipelines/front_ms 237, render/thread/ms 64.02, render/busy/ms 62.53, main/busy/ms 49.50, stage/render/prepare_assets 27.44, stage/main/update 26.62, pipelines/wait_ms 18.32, stage/render/render 14.75, render/graph/ms 12.90, stage/main/post_update 11.18, stage/render/graph_submit 9.93 |
| warm-up | 4.87 | 36.7 | pipelines/compile_ms 223, pipelines/front_ms 217, render/thread/ms 68.37, render/busy/ms 66.98, main/busy/ms 46.03, stage/main/update 25.14, pipelines/wait_ms 17.75, stage/render/prepare_resources 17.30, stage/render/prepare_assets 15.59, stage/render/render 13.47, render/graph/ms 11.65, stage/main/post_update 10.15 |
| warm-up | 5.15 | 30.6 | pipelines/compile_ms 193, pipelines/front_ms 188, render/thread/ms 45.55, render/busy/ms 43.73, main/busy/ms 32.74, stage/render/render 18.57, render/graph/ms 16.18, stage/main/update 15.78, stage/render/graph_submit 11.87, stage/main/post_update 11.01, encode//sum 8.83, encode/submit 8.40 |
| warm-up | 5.38 | 37.0 | pipelines/compile_ms 154, pipelines/front_ms 149, render/thread/ms 57.70, render/busy/ms 55.64, main/busy/ms 45.29, stage/main/update 22.91, stage/render/prepare_assets 19.35, stage/render/render 18.77, render/graph/ms 15.65, stage/main/post_update 14.90, encode//sum 13.20, stage/render/graph_submit 10.59 |
| warm-up | 5.41 | 34.3 | pipelines/compile_ms 154, pipelines/front_ms 149, render/thread/ms 65.41, render/busy/ms 63.89, main/busy/ms 46.14, stage/render/prepare_assets 37.64, stage/main/update 23.45, stage/render/render 13.07, stage/main/post_update 12.32, render/graph/ms 10.73, encode//sum 9.99, stage/render/graph_submit 6.83 |
| warm-up | 5.47 | 56.9 | render/thread/ms 87.08, render/busy/ms 86.10, stage/render/prepare_assets 41.62, main/busy/ms 37.89, stage/render/render 34.76, render/graph/ms 33.47, encode//sum 32.09, stage/render/graph_submit 31.18, encode/finish_wall 26.04, encode/pass:main_transparent_pass_3d/sum 25.20, encode/pass:main_transmissive_pass_3d/sum 25.18, stage/main/update 20.80 |
| warm-up | 5.74 | 30.8 | main/busy/ms 38.13, render/thread/ms 37.18, render/busy/ms 35.93, stage/main/update 20.58, stage/render/prepare_assets 13.81, stage/render/render 12.53, stage/main/post_update 11.17, render/graph/ms 10.64, encode//sum 9.18, stage/render/graph_submit 7.06, encode/submit 4.62, stream/drive/ms 4.61 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\gc_p0b\balmora@2560x1440_arm=off-2 --bench-baseline none --off groundcover
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (266 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 92.3 W · 75 °C (max 77) · VRAM 5347 MiB · throttled 100% of samples

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
