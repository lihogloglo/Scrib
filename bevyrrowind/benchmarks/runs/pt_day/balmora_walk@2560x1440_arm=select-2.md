# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); select.gate=on (command line: --set); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6036 frames over 65.6 s · **median 10.56 ms (95 fps)** · mean 10.88 · p95 13.15 · p99 16.95 (1% low 59 fps) · worst 58.39 ms · 80 spikes over 1.5× median · 68.6 stutters/min (work 56.7/min)
- warm-up: 325 frames over 9.2 s, worst 2611.3 ms
- hitches (> 26.4 ms): **34** measured, 50 in warm-up
- timed GPU passes: 6.90 ms of a 10.88 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.56 ms, limit 11 ms
- **FAIL** frame_ms: 75 stutters (> 1.5x moving median of 60) in 1.09 min = 68.55/min, limit 1/min
- **FAIL** worst frame 58.39 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.955 | 2.036 | 2.066 | 0.96 | 0.175 |
| main_opaque_pass_3d | 1.565 | 1.630 | 2.042 | 0.96 | 0.075 |
| early prepass | 0.472 | 0.492 | 0.716 | 0.96 | 0.088 |
| bloom | 0.303 | 0.315 | 0.321 | 0.96 | 0.063 |
| atmosphere_luts | 0.292 | 0.304 | 0.366 | 0.96 | 0.003 |
| water_velocity | 0.268 | 0.279 | 0.347 | 0.96 | 0.002 |
| water_sspr | 0.193 | 0.201 | 0.202 | 0.96 | 0.003 |
| ssao | 0.181 | 0.189 | 0.193 | 0.96 | 0.007 |
| atmos_froxel_inject | 0.140 | 0.146 | 0.147 | 0.96 | 0.002 |
| clouds_trace | 0.135 | 0.140 | 0.232 | 0.96 | 0.001 |
| uniform_allocation | 0.127 | 0.132 | 0.182 | 0.96 | 0.028 |
| water_fft | 0.125 | 0.130 | 0.137 | 0.96 | 0.003 |
| auxiliary_views | 0.123 | 0.128 | 0.493 | 0.96 | 0.136 |
| upscaling | 0.112 | 0.117 | 0.124 | 0.96 | 0.004 |
| clouds_composite | 0.101 | 0.105 | 0.108 | 0.96 | 0.002 |
| bin_unpacking | 0.101 | 0.105 | 0.141 | 0.96 | 0.021 |
| main_transmissive_pass_3d | 0.083 | 0.086 | 0.179 | 0.96 | 0.009 |
| tonemapping | 0.082 | 0.085 | 0.089 | 0.96 | 0.004 |
| clustering | 0.061 | 0.064 | 0.067 | 0.96 | 0.019 |
| render_sky | 0.050 | 0.052 | 0.052 | 0.96 | 0.002 |
| clouds_resolve | 0.046 | 0.048 | 0.049 | 0.96 | 0.001 |
| lightprobe_irradiance_map | 0.044 | 0.414 | 0.416 | 0.11 | 0.001 |
| surface_exposure | 0.042 | 0.044 | 0.045 | 0.96 | 0.001 |
| surface_trail | 0.038 | 0.056 | 0.057 | 0.68 | 0.002 |
| surface_exposure_far | 0.036 | 0.042 | 0.045 | 0.86 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.96 | 0.004 |
| early_mesh_preprocessing | 0.030 | 0.031 | 0.047 | 0.96 | 0.006 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.031 | 0.96 | 0.001 |
| water_snapshot | 0.024 | 0.025 | 0.028 | 0.96 | 0.002 |
| lightprobe_radiance_map | 0.023 | 0.219 | 0.229 | 0.11 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.96 | 0.002 |
| night_sky | 0.014 | 0.014 | 0.033 | 0.96 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.026 | 0.96 | 0.123 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.96 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.96 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.96 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.96 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.528 | 11.538 | 57.06 | 57509.4 |
| render/busy/ms | 9.001 | 11.025 | 56.53 | 54328.7 |
| main/busy/ms | 8.763 | 10.358 | 28.83 | 52893.2 |
| stage/render/render | 5.544 | 6.860 | 52.51 | 33464.2 |
| stage/main/post_update | 5.529 | 6.820 | 11.47 | 33370.6 |
| render/graph/ms | 4.824 | 6.090 | 51.74 | 29117.8 |
| encode//sum | 3.880 | 4.883 | 51.45 | 23420.0 |
| stage/render/graph_submit | 3.620 | 4.613 | 50.51 | 21848.9 |
| encode/submit | 2.270 | 3.016 | 6.79 | 13700.3 |
| stage/main/update | 1.964 | 2.371 | 22.30 | 11857.4 |
| encode/finish_wall | 1.408 | 2.034 | 47.61 | 8497.5 |
| stage/render/camera_driver | 1.151 | 1.547 | 4.27 | 6946.0 |
| encode/pass:shadow_directional_light/sum | 0.896 | 1.618 | 2.81 | 5408.5 |
| encode/pass:shadow_point_light/sum | 0.813 | 2.972 | 18.12 | 4905.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.807 | 1.138 | 1.78 | 4873.8 |
| stage/render/prepare_write_phase_buffers | 0.794 | 1.104 | 2.44 | 4795.3 |
| encode/pass:early prepass/sum | 0.789 | 1.119 | 3.84 | 4762.0 |
| stage/render/prepare_assets | 0.575 | 0.748 | 10.87 | 3471.3 |
| bigspace/total/ms | 0.496 | 1.123 | 2.64 | 2991.4 |
| stage/render/prepare_resources | 0.456 | 0.543 | 1.03 | 2752.9 |
| stage/render/prepare_meshes | 0.448 | 0.560 | 6.92 | 2704.9 |
| render/present/ms | 0.435 | 0.644 | 2.56 | 2626.4 |
| stage/main/first | 0.396 | 0.517 | 5.24 | 2388.6 |
| stage/main/pre_update | 0.361 | 0.517 | 1.34 | 2181.2 |
| stage/render/specialize | 0.330 | 0.421 | 13.27 | 1991.1 |
| stage/render/queue | 0.301 | 0.434 | 8.34 | 1816.2 |
| stage/render/prepare_bind_groups | 0.285 | 0.500 | 2.44 | 1719.1 |
| controller/feed/ms | 0.266 | 0.334 | 0.76 | 1604.1 |
| encode/pass:main_transparent_pass_3d/sum | 0.245 | 0.359 | 1.24 | 1478.6 |
| stage/main/fixed_main_loop | 0.218 | 0.377 | 0.95 | 1318.1 |
| stream/drive/ms | 0.209 | 0.297 | 19.87 | 1259.4 |
| bigspace/low_precision/ms | 0.207 | 0.599 | 1.92 | 1250.8 |
| stage/render/create_views | 0.199 | 0.258 | 1.03 | 1199.0 |
| stage/render/prepare_batch_phases | 0.188 | 0.272 | 0.55 | 1135.4 |
| stage/render/extract_commands | 0.177 | 0.205 | 0.47 | 1066.0 |
| pipelines/compile_ms | 0.174 | 0.000 | 139.90 | 1048.2 |
| pipelines/front_ms | 0.169 | 0.000 | 136.32 | 1017.8 |
| stage/render/prepare_flush | 0.165 | 0.306 | 0.96 | 994.5 |
| anim/evaluate_cpu/ms | 0.155 | 0.196 | 0.61 | 936.9 |
| anim/evaluate/ms | 0.154 | 0.237 | 2.05 | 929.3 |
| stream/tick/ms | 0.151 | 0.220 | 0.49 | 910.6 |
| stage/render/prepare_views | 0.135 | 0.168 | 0.39 | 817.2 |
| controller/move/ms | 0.132 | 0.265 | 0.59 | 797.1 |
| bigspace/high_precision/ms | 0.127 | 0.331 | 2.03 | 767.0 |
| stage/main/last | 0.116 | 0.165 | 1.60 | 701.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.104 | 0.166 | 0.77 | 628.5 |
| render/acquire/ms | 0.092 | 0.117 | 0.31 | 554.3 |
| stage/render/prepare_collect_phase_buffers | 0.061 | 0.133 | 0.54 | 370.9 |
| terrain/update/ms | 0.057 | 0.284 | 2.20 | 344.3 |
| physics/step/ms | 0.047 | 0.087 | 0.59 | 282.1 |
| encode/resolve_diagnostics/sum | 0.035 | 0.052 | 0.63 | 211.0 |
| physics/insert/ms | 0.031 | 0.353 | 0.83 | 188.0 |
| stage/render/cleanup | 0.031 | 0.091 | 0.88 | 187.3 |
| stage/render/phase_sort | 0.028 | 0.048 | 0.26 | 166.0 |
| weather/map/ms | 0.025 | 0.033 | 0.29 | 152.5 |
| terrain/select/ms | 0.024 | 0.254 | 1.03 | 142.4 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.022 | 0.09 | 93.9 |
| far/select/ms | 0.016 | 0.165 | 0.57 | 93.8 |
| anim/apply/ms | 0.012 | 0.021 | 0.38 | 72.6 |
| encode/camera_driver_span_end/sum | 0.012 | 0.020 | 0.51 | 72.1 |
| weather/eval/ms | 0.010 | 0.014 | 0.06 | 62.2 |
| controller/avoid/ms | 0.010 | 0.014 | 0.20 | 61.4 |
| script/update/ms | 0.008 | 0.009 | 0.14 | 46.9 |
| script/input/ms | 0.006 | 0.007 | 0.03 | 33.6 |
| pipelines/driver_ms | 0.005 | 0.000 | 4.07 | 30.4 |
| stream/despawn/ms | 0.005 | 0.000 | 0.89 | 30.1 |
| stream/spawn/ms | 0.005 | 0.000 | 1.09 | 27.7 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 23.2 |
| controller/write/ms | 0.003 | 0.004 | 0.03 | 20.1 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 14.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.28 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.9 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 562 | 579 | 578 |
| anim/evaluated/count | count | 18 | 32 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 593 |
| crowd/c0/meshes/count | count | 353 | 646 | 156 |
| crowd/c1/count | count | 145 | 155 | 148 |
| crowd/c1/joints/count | count | 5477 | 5936 | 5603 |
| crowd/c1/meshes/count | count | 1150 | 1268 | 1185 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4597 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4572 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.43 | 47.51 | 0.52 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.09 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.51 | 0.01 |
| encode/finish_critical | ms | 1.00 | 47.51 | 2.06 |
| encode/pass:early prepass/max | ms | 0.81 | 3.84 | 1.28 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 1.78 | 0.77 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.77 | 0.14 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.24 | 0.31 |
| encode/pass:shadow_directional_light/max | ms | 0.83 | 2.81 | 2.06 |
| encode/pass:shadow_point_light/max | ms | 0.83 | 1.50 | 0.84 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.63 | 0.08 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.14 | 56.53 | 8.94 |
| gpu/timed/ms | ms | 7.88 | 9.48 | 8.28 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 820 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 820 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1280 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3424 | 5095 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1280 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3424 | 5095 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 20 | 204 | 28 |
| phases/shadow/bins | count | 5050 | 8748 | 2369 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.0 k | 6316 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 205 | 246 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3182 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 10 | 565 | 56 |
| preprocess/bind_groups_created | count | 10 | 565 | 56 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 223 | 123 |
| process/cpu_usage | % | 26.57 | 40.91 | 21.15 |
| process/mem_usage | GiB | 5.47 | 5.64 | 5.64 |
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
| skin/blocks_changed/count | count | 14 | 132 | 12 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 366.3 k | 807.7 k | 498.2 k |
| skin/joints_written/count | count | 5724 | 12.6 k | 7784 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4918 | 5012 | 5012 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 4 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 3 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.27 | 42.43 | 31.68 |
| system/mem_usage | % | 54.56 | 55.18 | 55.14 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 68 | 251 | 78 |
| views/all/bins | count | 7580 | 11.6 k | 5867 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.1 k | 13.9 k |
| views/all/pipelines | count | 250 | 293 | 74 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.6 k | 33.5 k | 14.2 k |
| views/cascade/batch_sets | count | 9 | 54 | 28 |
| views/cascade/bins | count | 2799 | 6171 | 2369 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8908 | 23.1 k | 6316 |
| views/cascade/pipelines | count | 36 | 54 | 28 |
| views/cascade/work_items | count | 404 | 23.1 k | 6316 |
| views/lamp_face/batch_sets | count | 0 | 196 | 0 |
| views/lamp_face/bins | count | 2200 | 2750 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2448 | 3048 | 0 |
| views/lamp_face/pipelines | count | 169 | 202 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2912 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2858 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8485 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8642 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 55.48 | 279 | 334.9 k |
| anim/events/count | count | 0.08 | 4 | 484 |
| assets/Image/added | count | 0.10 | 29 | 592 |
| assets/Image/modified | count | 0.00 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 201 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.59 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1299 | 4675 |
| encode//count | count | 32.82 | 219 | 198.1 k |
| encode/bind_walk/init_textures | count | 12.1 k | 114.7 k | 73.19 M |
| encode/bind_walk/init_textures_skipped | count | 64.2 k | 146.4 k | 387.37 M |
| encode/bind_walk/inits | count | 455 | 1132 | 2.75 M |
| encode/bind_walk/inits_skipped | count | 108 | 276 | 649.7 k |
| encode/bind_walk/merge_textures | count | 12.1 k | 114.6 k | 72.75 M |
| encode/bind_walk/merge_textures_skipped | count | 5360 | 12.0 k | 32.35 M |
| encode/bind_walk/merges | count | 97.74 | 338 | 589.9 k |
| encode/bind_walk/merges_skipped | count | 83.92 | 244 | 506.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 6036 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 6036 |
| encode/pass:early prepass/count | count | 1 | 2 | 6036 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 6036 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 6036 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 6036 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 6036 |
| encode/pass:shadow_point_light/count | count | 1.22 | 48 | 7344 |
| encode/queue/command_buffers | count | 44.05 | 277 | 265.9 k |
| encode/queue/submits | count | 2.02 | 4 | 12.2 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.88 M | 11.34 G |
| encode/queue/write_buffer/calls | count | 256 | 843 | 1.54 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.5 k | 3.52 M | 1.07 G |
| encode/queue/write_buffer_with/calls | count | 11.00 | 612 | 66.4 k |
| encode/queue/write_texture/bytes | bytes | 11.9 k | 8.39 M | 71.91 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 6036 |
| physics/inserts | count | 0.37 | 95 | 2259 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 175 | 2335 |
| physics/steps | count | 0.65 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 10 |
| pipelines/created | count | 0.00 | 2 | 14 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 28 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5608 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 54 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1230 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.4 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.61 | 2611.3 | main/busy/ms 2579, stage/main/post_update 2474, stage/main/update 96.00, surface/advance_cpu_ms 67.82, stage/main/pre_update 5.04, stage/main/last 1.06, surface/cell_upload_ms 0.76, stage/main/first 0.59, stage/main/fixed_main_loop 0.35, stream/drive/ms 0.34, weather/map/ms 0.12, bigspace/total/ms 0.08 |
| warm-up | 3.50 | 893.0 | main/busy/ms 2753, stage/main/post_update 2505, render/thread/ms 859, render/busy/ms 719, stage/render/prepare_assets 671, stage/main/update 233, stage/render/prepare_views 138, render/acquire/ms 138, surface/advance_cpu_ms 67.82, stage/render/create_views 28.83, stage/render/render 11.00, render/graph/ms 6.99 |
| warm-up | 3.61 | 110.6 | render/thread/ms 954, render/busy/ms 759, stage/render/prepare_assets 697, main/busy/ms 200, stage/main/update 141, stage/render/prepare_views 139, render/acquire/ms 138, pipelines/compile_ms 120, pipelines/front_ms 76.81, stage/render/render 69.99, render/present/ms 56.36, pipelines/driver_ms 43.12 |
| warm-up | 3.66 | 41.2 | pipelines/compile_ms 455, pipelines/front_ms 362, render/thread/ms 131, main/busy/ms 96.90, pipelines/driver_ms 92.87, stage/render/render 89.26, pipelines/wait_ms 84.35, render/present/ms 72.93, stage/main/post_update 65.10, render/busy/ms 57.77, terrain/update/ms 30.96, stage/render/prepare_assets 27.23 |
| warm-up | 3.73 | 71.5 | pipelines/wait_ms 2112, pipelines/compile_ms 840, pipelines/front_ms 761, main/busy/ms 183, stage/main/post_update 79.77, pipelines/driver_ms 78.25, render/thread/ms 73.58, stage/main/update 54.84, render/busy/ms 53.06, stage/render/render 51.18, render/graph/ms 30.06, stage/main/fixed_main_loop 26.15 |
| warm-up | 3.89 | 165.6 | pipelines/wait_ms 4545, pipelines/compile_ms 1845, pipelines/front_ms 1744, main/busy/ms 135, render/thread/ms 119, render/busy/ms 117, pipelines/driver_ms 101, stage/render/prepare_assets 62.96, stage/main/update 58.59, stage/render/render 44.89, render/graph/ms 42.61, stage/render/graph_submit 40.70 |
| warm-up | 3.94 | 42.6 | pipelines/wait_ms 2633, pipelines/compile_ms 2178, pipelines/front_ms 2066, render/thread/ms 122, render/busy/ms 121, pipelines/driver_ms 112, stage/render/prepare_assets 75.14, encode//sum 58.71, main/busy/ms 42.77, stage/render/render 36.85, render/graph/ms 34.46, stage/render/graph_submit 32.03 |
| warm-up | 3.97 | 36.9 | pipelines/compile_ms 1560, pipelines/front_ms 1501, pipelines/wait_ms 280, render/thread/ms 74.56, render/busy/ms 72.59, pipelines/driver_ms 59.48, encode//sum 57.63, stage/render/prepare_assets 41.80, main/busy/ms 32.89, stage/render/render 24.76, render/graph/ms 22.41, stage/main/post_update 18.32 |
| warm-up | 4.11 | 68.1 | pipelines/compile_ms 846, pipelines/front_ms 811, main/busy/ms 58.67, render/thread/ms 48.47, render/busy/ms 45.88, stage/main/update 45.68, stream/drive/ms 40.87, encode//sum 39.38, pipelines/driver_ms 35.72, pipelines/wait_ms 26.77, stage/render/prepare_assets 23.19, stage/render/render 17.12 |
| warm-up | 4.16 | 35.8 | pipelines/compile_ms 608, pipelines/front_ms 591, render/thread/ms 39.66, render/busy/ms 38.16, main/busy/ms 22.87, stage/render/prepare_assets 20.86, pipelines/driver_ms 16.98, stage/main/update 13.01, stage/render/render 11.66, render/graph/ms 9.82, stream/drive/ms 9.44, stage/render/graph_submit 7.66 |
| warm-up | 4.25 | 73.5 | pipelines/compile_ms 493, pipelines/front_ms 471, main/busy/ms 76.69, render/thread/ms 47.52, render/busy/ms 45.73, stage/main/update 41.98, stream/drive/ms 28.47, stage/main/post_update 25.07, stage/render/prepare_assets 23.66, pipelines/driver_ms 21.82, stage/render/render 15.52, render/graph/ms 13.09 |
| warm-up | 4.32 | 71.7 | pipelines/compile_ms 563, pipelines/front_ms 543, main/busy/ms 73.22, render/thread/ms 68.56, render/busy/ms 66.77, stage/main/update 44.12, stage/render/prepare_assets 40.25, pipelines/driver_ms 20.68, stage/main/post_update 19.04, stage/render/render 14.35, render/graph/ms 12.04, stage/render/graph_submit 7.53 |
| warm-up | 4.42 | 101.5 | pipelines/compile_ms 1583, pipelines/front_ms 1530, render/thread/ms 104, render/busy/ms 102, stage/render/prepare_assets 69.93, pipelines/wait_ms 60.91, main/busy/ms 59.03, pipelines/driver_ms 52.79, stage/main/update 36.92, stage/render/render 15.28, render/graph/ms 12.83, stage/main/post_update 12.80 |
| warm-up | 4.50 | 78.1 | pipelines/compile_ms 1889, pipelines/front_ms 1797, pipelines/wait_ms 228, render/thread/ms 142, render/busy/ms 138, pipelines/driver_ms 91.10, encode//sum 76.61, stage/render/prepare_assets 53.09, main/busy/ms 49.31, stage/render/render 45.81, render/graph/ms 41.07, stage/render/graph_submit 36.17 |
| warm-up | 4.58 | 74.1 | pipelines/compile_ms 1745, pipelines/front_ms 1664, pipelines/wait_ms 711, render/thread/ms 131, render/busy/ms 127, pipelines/driver_ms 81.41, encode//sum 75.53, main/busy/ms 52.36, stage/render/render 43.61, render/graph/ms 39.27, stage/render/graph_submit 33.90, stage/render/prepare_assets 28.83 |
| warm-up | 4.65 | 75.2 | pipelines/compile_ms 2057, pipelines/front_ms 1995, pipelines/wait_ms 778, render/thread/ms 126, render/busy/ms 124, pipelines/driver_ms 62.38, encode//sum 59.71, main/busy/ms 41.44, stage/render/prepare_assets 38.31, stage/render/render 29.96, render/graph/ms 27.49, stage/render/prepare_resources 27.41 |
| warm-up | 4.69 | 39.0 | pipelines/compile_ms 1816, pipelines/front_ms 1764, pipelines/wait_ms 342, render/thread/ms 106, render/busy/ms 104, encode//sum 56.77, pipelines/driver_ms 52.47, main/busy/ms 47.74, stage/render/prepare_resources 34.72, stage/render/render 30.09, render/graph/ms 27.67, stage/main/post_update 24.73 |
| warm-up | 4.72 | 30.4 | pipelines/compile_ms 916, pipelines/front_ms 891, pipelines/wait_ms 105, render/thread/ms 59.09, render/busy/ms 57.60, main/busy/ms 49.44, pipelines/driver_ms 25.43, stage/main/post_update 24.71, stage/main/update 17.80, stage/render/prepare_resources 16.62, stage/render/render 15.93, render/graph/ms 13.98 |
| warm-up | 4.82 | 71.3 | pipelines/compile_ms 1446, pipelines/front_ms 1408, pipelines/wait_ms 280, main/busy/ms 95.53, stage/main/update 70.22, render/thread/ms 50.62, render/busy/ms 48.83, stream/drive/ms 48.64, pipelines/driver_ms 38.31, stage/render/prepare_assets 16.80, stage/render/render 14.84, stage/main/post_update 13.18 |
| warm-up | 4.90 | 75.1 | pipelines/compile_ms 1624, pipelines/front_ms 1578, pipelines/wait_ms 241, main/busy/ms 118, stage/main/update 93.34, render/thread/ms 80.20, render/busy/ms 78.08, stream/drive/ms 54.08, pipelines/driver_ms 46.74, stage/render/prepare_assets 28.86, stage/render/render 22.47, render/graph/ms 19.41 |
| warm-up | 5.04 | 140.7 | pipelines/compile_ms 1517, pipelines/front_ms 1470, render/thread/ms 133, render/busy/ms 131, main/busy/ms 111, stage/main/update 84.83, stage/render/prepare_assets 66.16, stream/drive/ms 53.24, pipelines/driver_ms 46.90, stage/render/render 44.07, render/graph/ms 41.57, stage/render/graph_submit 37.78 |
| warm-up | 5.08 | 47.1 | pipelines/compile_ms 1271, pipelines/front_ms 1233, render/thread/ms 124, render/busy/ms 120, main/busy/ms 87.64, stage/main/update 60.78, stage/render/render 54.30, stage/render/prepare_assets 53.40, render/graph/ms 49.78, stage/render/graph_submit 46.30, stream/drive/ms 41.74, pipelines/driver_ms 37.50 |
| warm-up | 5.15 | 64.6 | pipelines/compile_ms 167, pipelines/front_ms 163, main/busy/ms 77.11, render/thread/ms 74.89, render/busy/ms 71.15, stage/main/update 58.66, stream/drive/ms 41.66, stage/render/prepare_assets 30.88, stage/render/render 28.04, render/graph/ms 23.50, stage/render/graph_submit 20.33, encode/submit 19.91 |
| warm-up | 5.21 | 38.7 | pipelines/compile_ms 1451, pipelines/front_ms 1416, main/busy/ms 54.58, stage/main/update 40.08, render/thread/ms 36.07, pipelines/driver_ms 35.37, render/busy/ms 34.63, stream/drive/ms 34.15, stage/render/render 15.05, render/graph/ms 13.13, stage/render/graph_submit 9.79, stage/main/post_update 9.39 |
| warm-up | 5.31 | 29.9 | pipelines/compile_ms 144, pipelines/front_ms 139, render/thread/ms 53.25, render/busy/ms 51.96, stage/render/prepare_assets 33.65, main/busy/ms 28.63, stage/main/update 16.28, stage/render/render 10.70, encode//sum 9.32, render/graph/ms 8.77, stage/main/post_update 7.77, stage/render/graph_submit 6.43 |
| warm-up | 5.40 | 34.2 | pipelines/compile_ms 751, pipelines/front_ms 729, render/thread/ms 45.94, render/busy/ms 44.53, main/busy/ms 38.68, pipelines/driver_ms 22.49, stage/main/post_update 17.77, stage/render/prepare_assets 15.84, stage/main/update 14.44, stage/render/render 13.85, render/graph/ms 11.79, stage/render/graph_submit 8.26 |
| warm-up | 5.43 | 29.9 | pipelines/compile_ms 735, pipelines/front_ms 713, render/thread/ms 55.42, render/busy/ms 53.71, main/busy/ms 43.52, stage/main/post_update 22.20, pipelines/driver_ms 21.92, stage/render/prepare_assets 17.00, stage/main/update 14.67, stage/render/render 13.62, render/graph/ms 11.18, encode//sum 9.25 |
| warm-up | 5.46 | 34.5 | render/thread/ms 59.50, render/busy/ms 57.68, main/busy/ms 45.78, stage/main/post_update 20.21, stage/main/update 16.07, stage/render/render 14.58, stage/render/prepare_resources 14.27, render/graph/ms 12.19, encode//sum 9.64, stage/render/graph_submit 8.98, stage/render/prepare_assets 8.54, encode/submit 6.59 |
| warm-up | 5.52 | 61.8 | render/thread/ms 78.19, render/busy/ms 76.78, main/busy/ms 50.43, stage/render/prepare_assets 36.62, stage/main/update 22.10, stage/main/post_update 18.32, stage/render/prepare_resources 13.23, stage/render/render 13.04, render/graph/ms 10.97, encode//sum 8.70, stage/render/graph_submit 7.85, stream/drive/ms 5.72 |
| warm-up | 5.56 | 41.7 | pipelines/compile_ms 1260, pipelines/front_ms 1229, render/thread/ms 82.41, render/busy/ms 81.13, stage/render/prepare_assets 57.65, main/busy/ms 49.35, pipelines/driver_ms 30.52, stage/main/update 22.38, stage/main/post_update 19.20, stage/render/render 11.92, render/graph/ms 9.79, stream/drive/ms 8.82 |
| warm-up | 5.60 | 31.2 | pipelines/compile_ms 1903, pipelines/front_ms 1858, render/thread/ms 58.37, render/busy/ms 57.17, pipelines/driver_ms 44.93, stage/render/prepare_assets 36.72, main/busy/ms 33.78, stage/main/post_update 13.94, stage/main/update 13.70, stage/render/render 10.23, encode//sum 8.86, render/graph/ms 8.29 |
| warm-up | 5.67 | 32.0 | pipelines/compile_ms 355, pipelines/front_ms 345, render/thread/ms 47.57, render/busy/ms 46.31, main/busy/ms 26.46, stage/render/prepare_assets 22.26, stage/render/render 11.72, stage/main/update 10.79, stage/main/post_update 10.75, pipelines/driver_ms 10.55, render/graph/ms 10.02, encode//sum 9.50 |
| warm-up | 5.73 | 39.1 | render/thread/ms 45.56, render/busy/ms 44.32, pipelines/compile_ms 26.64, stage/render/prepare_assets 26.03, pipelines/front_ms 25.30, main/busy/ms 20.70, stage/render/render 11.08, render/graph/ms 9.48, stage/main/post_update 8.35, encode//sum 8.26, stage/render/graph_submit 6.93, stage/main/update 6.80 |
| warm-up | 6.24 | 30.7 | render/thread/ms 38.70, render/busy/ms 37.54, main/busy/ms 28.32, stage/render/prepare_assets 17.18, stage/main/update 11.78, stage/main/post_update 9.60, stage/render/render 7.88, render/graph/ms 6.45, encode//sum 6.12, stage/render/graph_submit 4.44, stage/render/specialize 4.05, stage/main/pre_update 3.77 |
| warm-up | 6.46 | 30.6 | pipelines/compile_ms 147, pipelines/front_ms 143, render/thread/ms 48.63, render/busy/ms 47.00, main/busy/ms 40.05, stage/main/update 16.89, stage/render/render 16.28, stage/main/post_update 16.03, render/graph/ms 14.09, stage/render/prepare_assets 12.05, stage/render/graph_submit 10.42, encode//sum 8.58 |
| warm-up | 6.51 | 31.1 | pipelines/compile_ms 94.44, pipelines/front_ms 89.26, render/thread/ms 40.72, render/busy/ms 39.65, main/busy/ms 31.48, stage/render/prepare_assets 17.52, stage/main/update 14.29, stage/main/post_update 11.06, stage/render/render 9.38, render/graph/ms 7.98, encode//sum 7.56, stage/render/graph_submit 5.50 |
| warm-up | 6.59 | 29.7 | render/thread/ms 37.61, render/busy/ms 35.63, main/busy/ms 30.43, stage/render/prepare_assets 17.05, stage/main/update 15.54, stage/main/post_update 10.24, stage/render/render 9.30, render/graph/ms 6.89, encode//sum 6.41, stage/render/graph_submit 4.41, stream/drive/ms 3.66, encode/submit 2.47 |
| warm-up | 6.78 | 39.6 | render/thread/ms 53.34, render/busy/ms 51.95, main/busy/ms 37.64, stage/render/prepare_assets 30.36, stage/main/update 21.71, stream/drive/ms 16.03, stage/render/render 12.90, stage/main/post_update 11.22, render/graph/ms 11.13, encode//sum 8.98, stage/render/graph_submit 8.60, encode/submit 6.25 |
| warm-up | 7.00 | 35.4 | main/busy/ms 40.51, render/thread/ms 29.15, render/busy/ms 28.14, stage/main/update 22.52, stage/render/prepare_assets 12.24, stage/main/post_update 10.73, stage/render/render 9.25, render/graph/ms 7.73, encode//sum 6.47, stage/render/graph_submit 5.25, stream/drive/ms 3.71, encode/submit 3.11 |
| warm-up | 7.11 | 38.2 | render/thread/ms 46.77, render/busy/ms 45.63, pipelines/compile_ms 24.69, pipelines/front_ms 23.45, main/busy/ms 20.33, stage/render/specialize 15.82, stage/render/render 12.60, stage/main/post_update 11.07, render/graph/ms 11.02, stage/render/graph_submit 8.73, encode//sum 7.67, stage/render/queue 6.98 |
| warm-up | 8.39 | 31.5 | pipelines/compile_ms 133, pipelines/front_ms 131, render/thread/ms 37.88, render/busy/ms 36.60, main/busy/ms 18.63, stage/render/specialize 10.94, stage/render/render 10.09, stage/main/post_update 10.02, render/graph/ms 8.52, encode//sum 7.07, stage/render/graph_submit 6.30, stage/render/queue 6.28 |
| measure | 0.38 | 30.6 | render/thread/ms 36.00, render/busy/ms 34.89, main/busy/ms 17.30, stage/render/render 11.18, stage/main/post_update 10.63, render/graph/ms 9.73, stage/render/specialize 9.23, stage/render/graph_submit 7.61, encode//sum 7.08, stage/render/queue 6.95, encode/submit 4.51, stage/main/update 3.63 |
| measure | 1.65 | 36.0 | render/thread/ms 39.93, render/busy/ms 39.01, main/busy/ms 24.61, stage/main/post_update 14.26, stage/render/specialize 11.30, stage/render/render 10.61, render/graph/ms 9.26, encode//sum 7.68, stage/render/queue 7.18, stage/render/graph_submit 7.16, stage/main/update 6.35, encode/submit 3.65 |
| measure | 3.00 | 31.4 | render/thread/ms 37.96, render/busy/ms 36.84, main/busy/ms 18.95, stage/main/post_update 12.02, stage/render/render 11.28, render/graph/ms 9.79, stage/render/specialize 9.10, encode//sum 7.86, stage/render/graph_submit 7.38, stage/render/queue 6.08, stage/render/prepare_meshes 4.40, encode/submit 4.08 |
| measure | 4.45 | 58.4 | render/thread/ms 84.52, render/busy/ms 83.43, stage/render/render 58.37, render/graph/ms 56.77, encode//sum 55.68, stage/render/graph_submit 54.37, encode/finish_wall 49.31, main/busy/ms 19.32, stage/main/post_update 12.77, stage/render/specialize 9.40, stage/render/queue 6.00, encode/submit 5.11 |
| measure | 9.72 | 38.2 | render/thread/ms 53.32, render/busy/ms 52.24, main/busy/ms 24.22, stage/render/specialize 14.31, stage/main/post_update 14.06, stage/render/queue 12.27, stage/render/render 11.22, render/graph/ms 9.79, encode//sum 8.48, stage/render/graph_submit 7.27, stage/main/update 6.56, stage/render/prepare_assets 4.60 |
| measure | 15.82 | 32.2 | render/thread/ms 36.76, render/busy/ms 35.63, main/busy/ms 20.04, stage/main/post_update 12.76, stage/render/render 10.90, stage/render/specialize 10.39, render/graph/ms 9.33, encode//sum 8.36, stage/render/graph_submit 6.94, stage/render/queue 6.79, encode/submit 4.41, stage/main/update 4.05 |
| measure | 18.33 | 31.4 | main/busy/ms 37.60, stage/main/update 23.86, stream/drive/ms 20.07, render/thread/ms 19.32, render/busy/ms 18.38, stage/main/post_update 10.76, stage/render/render 10.69, render/graph/ms 9.23, encode//sum 7.44, stage/render/graph_submit 6.75, encode/submit 4.47, encode/pass:shadow_point_light/sum 2.75 |
| measure | 20.76 | 29.7 | render/thread/ms 38.11, render/busy/ms 37.15, main/busy/ms 17.81, stage/main/post_update 11.65, stage/render/render 10.98, stage/render/specialize 10.30, render/graph/ms 9.65, encode//sum 7.83, stage/render/graph_submit 7.43, stage/render/queue 7.11, encode/submit 4.96, stage/main/update 3.52 |
| measure | 23.01 | 35.1 | render/thread/ms 43.23, render/busy/ms 42.22, main/busy/ms 23.73, stage/render/render 11.99, stage/main/post_update 11.68, stage/render/specialize 11.59, render/graph/ms 10.58, stage/render/graph_submit 8.28, stage/render/queue 8.24, encode//sum 7.89, stage/main/update 6.33, encode/submit 4.80 |
| measure | 24.47 | 30.5 | render/thread/ms 38.40, render/busy/ms 37.44, main/busy/ms 18.03, stage/render/render 11.51, stage/main/post_update 11.46, render/graph/ms 10.18, stage/render/specialize 9.91, stage/render/graph_submit 7.86, stage/render/queue 7.28, encode//sum 7.20, encode/submit 4.23, stage/main/update 3.89 |
| measure | 27.28 | 34.2 | render/thread/ms 43.01, render/busy/ms 41.89, main/busy/ms 24.47, stage/render/render 13.87, stage/main/post_update 12.45, render/graph/ms 12.28, stage/render/specialize 11.69, encode//sum 9.47, stage/render/graph_submit 9.45, stage/render/queue 7.94, encode/pass:shadow_point_light/sum 6.58, stage/main/update 6.02 |
| measure | 30.61 | 34.9 | render/thread/ms 42.44, render/busy/ms 41.34, main/busy/ms 21.03, stage/render/render 13.68, stage/main/post_update 12.39, render/graph/ms 12.13, stage/render/specialize 10.71, stage/render/graph_submit 9.28, encode//sum 8.64, stage/render/queue 7.37, stage/main/update 5.78, encode/submit 5.36 |
| measure | 32.04 | 31.5 | render/thread/ms 39.88, render/busy/ms 38.80, main/busy/ms 18.53, stage/render/render 12.72, stage/main/post_update 12.29, render/graph/ms 11.23, stage/render/specialize 9.78, encode//sum 9.59, stage/render/graph_submit 8.56, stage/render/queue 7.62, encode/pass:shadow_point_light/sum 5.15, encode/submit 5.00 |
| measure | 35.96 | 30.5 | render/thread/ms 41.29, render/busy/ms 40.30, main/busy/ms 19.83, stage/render/render 13.49, render/graph/ms 12.00, stage/main/post_update 11.91, stage/render/specialize 9.79, encode//sum 9.70, stage/render/graph_submit 9.30, stage/render/queue 7.53, encode/submit 5.58, stage/main/update 4.96 |
| measure | 37.39 | 31.2 | render/thread/ms 42.56, render/busy/ms 41.40, main/busy/ms 19.45, stage/render/render 13.73, stage/main/post_update 12.57, render/graph/ms 12.08, encode//sum 10.76, stage/render/specialize 10.66, stage/render/graph_submit 9.22, stage/render/queue 7.82, encode/submit 5.06, encode/finish_wall 4.21 |
| measure | 37.49 | 32.9 | render/thread/ms 40.20, render/busy/ms 39.06, main/busy/ms 20.99, stage/render/render 13.24, render/graph/ms 11.71, stage/main/post_update 11.16, encode//sum 10.13, stage/render/specialize 10.04, stage/render/graph_submit 9.02, stage/render/queue 6.79, stage/main/update 6.66, encode/submit 5.33 |
| measure | 41.16 | 34.5 | render/thread/ms 42.42, render/busy/ms 41.44, main/busy/ms 18.44, stage/render/render 12.29, render/graph/ms 10.84, stage/render/specialize 10.83, stage/main/post_update 10.77, stage/render/graph_submit 8.87, encode//sum 7.91, stage/render/queue 7.24, encode/finish_wall 4.83, stage/main/update 4.59 |
| measure | 43.86 | 30.5 | render/thread/ms 37.58, render/busy/ms 36.58, main/busy/ms 16.07, stage/render/render 10.61, stage/render/specialize 10.10, stage/main/post_update 9.14, render/graph/ms 9.12, stage/render/graph_submit 7.30, encode//sum 6.88, stage/render/queue 6.86, stage/main/update 4.32, encode/submit 4.05 |
| measure | 46.14 | 33.7 | render/thread/ms 40.35, render/busy/ms 39.36, main/busy/ms 18.04, stage/render/specialize 11.45, stage/main/post_update 10.21, stage/render/render 10.11, render/graph/ms 8.77, stage/render/queue 7.37, encode//sum 7.07, stage/render/graph_submit 6.84, stage/main/update 4.84, stage/render/prepare_meshes 4.55 |
| measure | 56.44 | 30.4 | render/thread/ms 36.92, render/busy/ms 35.78, main/busy/ms 15.60, stage/render/render 10.23, stage/main/post_update 9.51, stage/render/specialize 9.39, render/graph/ms 8.86, stage/render/queue 7.24, encode//sum 7.13, stage/render/graph_submit 6.84, stage/render/prepare_meshes 3.85, stage/main/update 3.71 |
| measure | 59.23 | 44.6 | render/thread/ms 55.82, render/busy/ms 54.19, main/busy/ms 32.67, stage/main/post_update 19.37, stage/render/render 17.07, render/graph/ms 14.65, encode//sum 13.63, stage/render/specialize 13.56, stage/render/graph_submit 11.42, stage/main/update 9.18, stage/render/prepare_meshes 7.35, stage/render/queue 7.18 |
| measure | 59.26 | 32.7 | render/thread/ms 58.68, render/busy/ms 57.66, main/busy/ms 24.08, stage/render/specialize 13.50, stage/render/prepare_assets 12.89, stage/main/post_update 12.54, stage/render/render 10.40, render/graph/ms 9.04, encode//sum 8.22, stage/render/prepare_meshes 7.97, stage/render/graph_submit 7.01, stage/render/queue 7.01 |
| measure | 63.17 | 32.6 | render/thread/ms 40.24, render/busy/ms 39.23, main/busy/ms 19.01, stage/render/render 11.11, stage/render/specialize 10.85, stage/main/post_update 10.00, render/graph/ms 9.81, encode//sum 7.95, stage/render/graph_submit 7.66, stage/render/queue 6.72, stage/main/update 5.99, stage/render/prepare_meshes 4.23 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set select.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=select-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 2%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **78%** (min 44%) · 2625 MHz (min 2625) · memory 8101 MHz · 80.8 W · 77 °C (max 81) · VRAM 7054 MiB · throttled 100% of samples

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
- `select.gate=on (command line: --set)`
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
