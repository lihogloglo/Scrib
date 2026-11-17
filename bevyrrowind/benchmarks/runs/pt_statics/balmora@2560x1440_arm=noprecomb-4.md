# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: precombine · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1552 frames over 15.0 s · **median 9.66 ms (103 fps)** · mean 9.67 · p95 10.91 · p99 11.41 (1% low 88 fps) · worst 40.79 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 311 frames over 8.3 s, worst 2445.6 ms
- hitches (> 25.0 ms): **1** measured, 40 in warm-up
- timed GPU passes: 8.57 ms of a 9.67 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.66 ms, limit 11 ms
- **FAIL** worst frame 40.79 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.891 | 2.895 | 2.948 | 1.00 | 0.063 |
| dlss_super_resolution | 2.022 | 2.025 | 2.050 | 1.00 | 0.179 |
| early prepass | 0.602 | 0.603 | 0.614 | 1.00 | 0.096 |
| atmosphere_luts | 0.340 | 0.340 | 0.397 | 1.00 | 0.003 |
| bloom | 0.314 | 0.315 | 0.331 | 1.00 | 0.062 |
| main_transmissive_pass_3d | 0.310 | 0.310 | 0.334 | 1.00 | 0.009 |
| water_velocity | 0.239 | 0.240 | 0.264 | 1.00 | 0.002 |
| water_sspr | 0.216 | 0.217 | 0.215 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.150 | 0.150 | 0.181 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.146 | 0.152 | 1.00 | 0.001 |
| water_fft | 0.131 | 0.131 | 0.135 | 1.00 | 0.003 |
| uniform_allocation | 0.120 | 0.120 | 0.161 | 1.00 | 0.021 |
| upscaling | 0.113 | 0.113 | 0.119 | 1.00 | 0.004 |
| clouds_composite | 0.112 | 0.112 | 0.114 | 1.00 | 0.002 |
| bin_unpacking | 0.095 | 0.095 | 0.123 | 1.00 | 0.015 |
| tonemapping | 0.086 | 0.086 | 0.094 | 1.00 | 0.003 |
| clustering | 0.065 | 0.065 | 0.082 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.053 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.413 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.052 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.038 | 0.039 | 0.052 | 1.00 | 0.004 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.229 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 1.00 | 0.085 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.013 | 0.043 | 0.046 | 0.30 | 0.002 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.008 | 0.008 | 0.020 | 1.00 | 0.005 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| auxiliary_views | 0.001 | 0.001 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.709 | 9.992 | 38.39 | 13516.2 |
| main/busy/ms | 7.639 | 8.555 | 10.08 | 11856.0 |
| render/busy/ms | 6.778 | 7.916 | 37.89 | 10519.0 |
| stage/main/post_update | 4.751 | 5.533 | 6.55 | 7373.0 |
| stage/render/render | 4.475 | 5.503 | 8.16 | 6945.7 |
| render/graph/ms | 3.909 | 4.901 | 6.36 | 6066.5 |
| encode//sum | 3.166 | 4.894 | 7.95 | 4914.1 |
| stage/render/graph_submit | 2.917 | 3.931 | 5.32 | 4526.9 |
| encode/submit | 1.836 | 2.583 | 4.41 | 2849.8 |
| stage/main/update | 1.789 | 2.269 | 4.42 | 2776.4 |
| stage/render/prepare_views | 1.632 | 3.167 | 4.13 | 2532.1 |
| render/acquire/ms | 1.586 | 3.119 | 4.08 | 2461.0 |
| encode/finish_wall | 1.098 | 1.512 | 2.34 | 1704.5 |
| pipelines/compile_ms | 1.092 | 0.000 | 136.55 | 1695.1 |
| pipelines/front_ms | 1.063 | 0.000 | 133.80 | 1649.2 |
| stage/render/camera_driver | 0.938 | 1.090 | 1.32 | 1455.2 |
| encode/pass:early prepass/sum | 0.740 | 1.009 | 1.45 | 1148.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.721 | 0.968 | 1.48 | 1119.3 |
| stage/render/prepare_write_phase_buffers | 0.526 | 0.737 | 0.98 | 816.7 |
| stage/render/prepare_assets | 0.441 | 0.560 | 1.52 | 684.9 |
| bigspace/total/ms | 0.411 | 0.767 | 1.76 | 637.9 |
| stage/render/prepare_resources | 0.384 | 0.445 | 1.96 | 595.7 |
| render/present/ms | 0.346 | 0.422 | 1.60 | 536.2 |
| stage/main/pre_update | 0.339 | 0.422 | 1.69 | 526.0 |
| encode/pass:shadow_directional_light/sum | 0.296 | 1.092 | 1.66 | 458.7 |
| stage/main/first | 0.291 | 0.349 | 1.48 | 452.2 |
| terrain/update/ms | 0.232 | 0.290 | 0.91 | 359.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.209 | 0.331 | 0.68 | 325.1 |
| stage/render/prepare_bind_groups | 0.206 | 0.282 | 0.43 | 319.7 |
| stage/render/prepare_meshes | 0.203 | 0.248 | 4.77 | 315.8 |
| terrain/select/ms | 0.202 | 0.253 | 0.73 | 313.4 |
| stream/drive/ms | 0.188 | 0.247 | 0.62 | 291.5 |
| stage/render/prepare_batch_phases | 0.176 | 0.237 | 0.40 | 273.7 |
| stage/main/fixed_main_loop | 0.174 | 0.325 | 0.89 | 269.6 |
| stage/render/extract_commands | 0.167 | 0.196 | 0.37 | 258.6 |
| stage/render/create_views | 0.165 | 0.212 | 0.71 | 255.8 |
| far/select/ms | 0.163 | 0.220 | 0.69 | 252.7 |
| stream/tick/ms | 0.158 | 0.212 | 0.57 | 244.5 |
| controller/move/ms | 0.149 | 0.292 | 0.69 | 231.5 |
| stage/render/prepare_flush | 0.148 | 0.276 | 0.86 | 230.3 |
| controller/feed/ms | 0.143 | 0.190 | 0.75 | 221.4 |
| bigspace/high_precision/ms | 0.129 | 0.318 | 1.33 | 200.1 |
| stage/main/last | 0.120 | 0.158 | 0.52 | 186.7 |
| stage/render/queue | 0.118 | 0.148 | 9.71 | 183.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.102 | 0.163 | 0.52 | 157.6 |
| bigspace/low_precision/ms | 0.098 | 0.221 | 1.43 | 151.6 |
| stage/render/specialize | 0.093 | 0.122 | 15.23 | 145.0 |
| anim/evaluate/ms | 0.085 | 0.209 | 1.13 | 131.6 |
| anim/evaluate_cpu/ms | 0.061 | 0.075 | 0.48 | 94.8 |
| stage/render/prepare_collect_phase_buffers | 0.057 | 0.110 | 0.22 | 88.2 |
| encode/resolve_diagnostics/sum | 0.043 | 0.073 | 0.47 | 66.1 |
| pipelines/driver_ms | 0.030 | 0.000 | 4.10 | 45.9 |
| physics/step/ms | 0.026 | 0.056 | 0.16 | 40.0 |
| weather/map/ms | 0.024 | 0.033 | 0.07 | 37.7 |
| stage/render/cleanup | 0.023 | 0.033 | 0.38 | 35.1 |
| encode/camera_driver_span_begin/sum | 0.020 | 0.035 | 0.08 | 30.9 |
| stage/render/phase_sort | 0.019 | 0.035 | 0.12 | 29.4 |
| encode/camera_driver_span_end/sum | 0.017 | 0.032 | 0.40 | 26.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.14 | 16.3 |
| script/update/ms | 0.006 | 0.008 | 0.03 | 10.0 |
| controller/avoid/ms | 0.006 | 0.007 | 0.42 | 9.1 |
| script/input/ms | 0.005 | 0.006 | 0.09 | 8.2 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 6.8 |
| script/ui/ms | 0.004 | 0.006 | 0.49 | 6.4 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.06 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.03 | 0.4 |
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
| encode//max | ms | 0.43 | 1.72 | 0.46 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.08 | 0.04 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.40 | 0.03 |
| encode/finish_critical | ms | 0.81 | 1.72 | 0.87 |
| encode/pass:early prepass/max | ms | 0.74 | 1.45 | 0.87 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.72 | 1.48 | 0.55 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.52 | 0.14 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.20 | 0.68 | 0.24 |
| encode/pass:shadow_directional_light/max | ms | 0.08 | 1.66 | 0.11 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.47 | 0.05 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.21 | 37.89 | 9.31 |
| gpu/timed/ms | ms | 9.21 | 10.69 | 9.31 |
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
| phases/opaque3d/bins | count | 1408 | 1408 | 1408 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 6339 | 6339 | 6339 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/bins | count | 1408 | 1408 | 1408 |
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
| precache/throttled | count | 0 | 8 | 0 |
| preprocess/bind_groups | count | 0 | 76 | 0 |
| preprocess/bind_groups_created | count | 0 | 76 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 26.92 | 44.23 | 27.06 |
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
| skin/blocks_changed/count | count | 5 | 6 | 4 |
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
| stream/latency/s | s | 0.49 | 0.49 | 0.49 |
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
| system/cpu_usage | % | 32.48 | 41.21 | 33.50 |
| system/mem_usage | % | 54.28 | 54.48 | 54.48 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 52 | 81 | 52 |
| views/all/bins | count | 5404 | 7687 | 5404 |
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
| views/main/bins | count | 3376 | 3376 | 3376 |
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
| anim/events/count | count | 0.10 | 3 | 153 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.18 | 11 | 4939 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.55 | 29 | 41.2 k |
| encode/bind_walk/init_textures | count | 5225 | 12.1 k | 8.11 M |
| encode/bind_walk/init_textures_skipped | count | 40.5 k | 56.5 k | 62.79 M |
| encode/bind_walk/inits | count | 336 | 429 | 521.3 k |
| encode/bind_walk/inits_skipped | count | 72 | 87 | 111.7 k |
| encode/bind_walk/merge_textures | count | 5162 | 12.0 k | 8.01 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.33 M |
| encode/bind_walk/merges | count | 78.26 | 99 | 121.5 k |
| encode/bind_walk/merges_skipped | count | 53.25 | 60 | 82.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1552 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1552 |
| encode/pass:early prepass/count | count | 1 | 1 | 1552 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1552 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1552 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1552 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1552 |
| encode/queue/command_buffers | count | 36.55 | 39 | 56.7 k |
| encode/queue/submits | count | 2 | 2 | 3104 |
| encode/queue/write_buffer/bytes | bytes | 1.49 M | 5.49 M | 2.32 G |
| encode/queue/write_buffer/calls | count | 212 | 283 | 328.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.66 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 14.0 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1552 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 3 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 24 |
| pipelines/created | count | 0.02 | 1 | 24 |
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

## Hitches (> 25.0 ms; slowest 41, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.45 | 2445.6 | main/busy/ms 2418, stage/main/post_update 2311, stage/main/update 95.06, surface/advance_cpu_ms 67.25, stage/main/pre_update 7.44, stage/main/last 1.18, surface/cell_upload_ms 0.71, stage/main/first 0.57, stream/drive/ms 0.41, stage/main/fixed_main_loop 0.35, weather/map/ms 0.12, stream/tick/ms 0.10 |
| warm-up | 3.28 | 835.8 | main/busy/ms 2549, stage/main/post_update 2335, render/thread/ms 798, render/busy/ms 614, stage/render/prepare_assets 567, stage/main/update 198, stage/render/prepare_views 180, render/acquire/ms 180, surface/advance_cpu_ms 67.25, stage/render/create_views 28.77, stage/render/render 11.98, stage/main/pre_update 9.18 |
| warm-up | 3.39 | 104.2 | render/thread/ms 885, render/busy/ms 652, stage/render/prepare_assets 590, stage/render/prepare_views 181, render/acquire/ms 181, main/busy/ms 151, stage/main/update 107, stage/render/render 66.47, pipelines/compile_ms 61.43, pipelines/front_ms 54.89, render/present/ms 52.62, stage/main/post_update 34.75 |
| warm-up | 3.42 | 37.7 | pipelines/compile_ms 319, pipelines/front_ms 286, render/thread/ms 115, stage/render/render 76.03, pipelines/wait_ms 75.73, main/busy/ms 63.00, render/busy/ms 58.19, render/present/ms 55.63, stage/main/post_update 48.12, pipelines/driver_ms 32.71, terrain/update/ms 23.81, stage/render/prepare_assets 23.45 |
| warm-up | 3.55 | 124.4 | pipelines/wait_ms 3821, pipelines/compile_ms 1073, pipelines/front_ms 1005, render/thread/ms 107, render/busy/ms 98.92, main/busy/ms 83.99, pipelines/driver_ms 68.02, stage/main/post_update 47.04, stage/main/update 31.63, stage/render/render 25.42, stage/render/prepare_flush 24.55, stage/render/prepare_resources 18.39 |
| warm-up | 3.62 | 75.1 | pipelines/wait_ms 6148, pipelines/compile_ms 2129, pipelines/front_ms 2017, render/thread/ms 152, render/busy/ms 151, main/busy/ms 146, pipelines/driver_ms 112, stage/main/update 78.04, stage/main/pre_update 38.02, stage/render/prepare_assets 35.63, stage/render/prepare_resources 34.19, stage/render/prepare_write_phase_buffers 30.45 |
| warm-up | 3.72 | 98.9 | pipelines/wait_ms 2650, pipelines/compile_ms 2549, pipelines/front_ms 2425, render/thread/ms 135, render/busy/ms 133, pipelines/driver_ms 124, main/busy/ms 121, stage/render/prepare_assets 70.52, stage/main/update 54.67, stage/main/pre_update 38.50, stage/render/prepare_resources 18.00, stage/main/post_update 17.78 |
| warm-up | 3.76 | 42.3 | pipelines/compile_ms 1849, pipelines/front_ms 1640, pipelines/wait_ms 319, pipelines/driver_ms 210, render/thread/ms 103, render/busy/ms 102, stage/render/prepare_assets 82.86, main/busy/ms 32.25, encode//sum 18.50, stage/main/post_update 16.65, stage/render/render 13.50, render/graph/ms 11.58 |
| warm-up | 3.89 | 38.4 | pipelines/compile_ms 712, pipelines/front_ms 679, main/busy/ms 44.98, stage/main/update 36.91, stream/drive/ms 33.41, pipelines/driver_ms 32.84, render/thread/ms 23.06, render/busy/ms 21.51, stage/render/prepare_assets 8.97, stage/render/render 8.02, render/graph/ms 6.28, encode//sum 5.06 |
| warm-up | 3.96 | 27.5 | pipelines/compile_ms 427, pipelines/front_ms 413, main/busy/ms 30.58, render/thread/ms 22.38, stage/main/update 21.70, render/busy/ms 21.10, stream/drive/ms 18.10, pipelines/driver_ms 14.23, stage/render/render 8.28, stage/render/prepare_assets 8.02, render/graph/ms 6.63, encode//sum 5.72 |
| warm-up | 4.00 | 33.2 | pipelines/compile_ms 189, pipelines/front_ms 184, render/thread/ms 34.95, render/busy/ms 33.35, main/busy/ms 28.63, stage/main/update 16.82, stage/render/prepare_assets 15.48, stream/drive/ms 12.36, stage/render/render 9.86, encode//sum 7.77, stage/main/post_update 7.74, render/graph/ms 7.73 |
| warm-up | 4.03 | 25.5 | pipelines/compile_ms 189, pipelines/front_ms 184, render/thread/ms 39.22, render/busy/ms 37.88, main/busy/ms 26.17, stage/render/prepare_assets 23.32, stage/main/update 12.23, stage/render/render 9.08, stage/main/post_update 8.97, render/graph/ms 7.38, pipelines/driver_ms 5.73, encode//sum 5.63 |
| warm-up | 4.06 | 30.6 | main/busy/ms 44.63, render/thread/ms 43.01, render/busy/ms 41.35, stage/main/update 31.41, pipelines/compile_ms 20.59, pipelines/front_ms 19.74, stage/render/prepare_assets 17.46, stage/render/render 16.92, render/graph/ms 14.96, stage/render/graph_submit 12.58, stage/main/post_update 8.52, encode/submit 5.66 |
| warm-up | 4.16 | 41.3 | pipelines/compile_ms 70.58, pipelines/front_ms 59.02, main/busy/ms 38.78, stage/main/update 23.72, render/thread/ms 21.34, render/busy/ms 19.83, stream/drive/ms 19.12, pipelines/driver_ms 11.55, stage/main/post_update 9.15, stage/render/prepare_assets 7.98, stage/render/render 7.16, render/graph/ms 5.14 |
| warm-up | 4.21 | 32.5 | pipelines/compile_ms 43.68, main/busy/ms 41.56, pipelines/front_ms 41.15, render/thread/ms 40.97, render/busy/ms 39.32, stage/render/render 19.44, stage/main/post_update 17.74, render/graph/ms 17.31, stage/main/update 10.74, stage/render/prepare_assets 9.66, stage/main/pre_update 9.24, stage/render/camera_driver 8.39 |
| warm-up | 4.25 | 38.7 | pipelines/compile_ms 261, pipelines/front_ms 251, render/thread/ms 60.77, main/busy/ms 60.14, render/busy/ms 59.05, stage/main/update 23.08, stage/render/render 19.43, stage/main/post_update 18.72, render/graph/ms 17.06, stage/main/pre_update 14.35, stage/render/prepare_flush 12.15, stage/render/prepare_assets 11.09 |
| warm-up | 4.30 | 46.6 | pipelines/compile_ms 1252, pipelines/front_ms 1216, render/thread/ms 67.70, render/busy/ms 65.87, main/busy/ms 42.43, pipelines/driver_ms 36.09, stage/main/update 23.64, stage/render/prepare_assets 21.78, stage/render/render 14.33, pipelines/wait_ms 12.05, render/graph/ms 12.00, stage/render/prepare_flush 11.70 |
| warm-up | 4.33 | 34.6 | pipelines/compile_ms 1954, pipelines/front_ms 1897, render/thread/ms 64.70, render/busy/ms 62.92, pipelines/driver_ms 56.85, pipelines/wait_ms 55.44, main/busy/ms 38.92, stage/render/prepare_assets 22.77, stage/main/update 16.15, stage/render/render 13.85, render/graph/ms 11.61, pipelines/lock_ms 9.48 |
| warm-up | 4.39 | 62.2 | pipelines/compile_ms 1225, pipelines/front_ms 1185, render/thread/ms 87.95, render/busy/ms 86.58, pipelines/wait_ms 83.28, main/busy/ms 50.41, pipelines/driver_ms 40.48, stage/render/prepare_assets 25.32, stage/main/fixed_main_loop 22.12, stage/render/prepare_resources 15.75, stage/main/update 12.39, stage/render/prepare_write_phase_buffers 12.28 |
| warm-up | 4.43 | 41.4 | pipelines/compile_ms 299, pipelines/front_ms 288, render/thread/ms 97.37, render/busy/ms 96.10, main/busy/ms 47.84, pipelines/wait_ms 39.13, stage/render/prepare_assets 37.03, stage/render/prepare_resources 24.23, stage/main/fixed_main_loop 18.79, stage/main/post_update 16.56, pipelines/driver_ms 11.90, stage/render/render 11.84 |
| warm-up | 4.49 | 57.2 | pipelines/compile_ms 1158, pipelines/front_ms 1127, render/thread/ms 95.24, render/busy/ms 86.18, main/busy/ms 50.70, pipelines/wait_ms 43.93, pipelines/driver_ms 30.54, stage/render/prepare_resources 22.55, stage/main/post_update 20.17, stage/render/render 19.05, stage/render/prepare_assets 18.24, stage/render/prepare_bind_groups 14.12 |
| warm-up | 4.53 | 39.4 | pipelines/compile_ms 1932, pipelines/front_ms 1878, render/thread/ms 94.50, render/busy/ms 85.27, pipelines/wait_ms 83.58, main/busy/ms 56.48, pipelines/driver_ms 54.10, stage/render/prepare_flush 30.28, stage/main/post_update 29.18, stage/render/render 20.05, stage/render/prepare_resources 15.69, stage/main/pre_update 14.21 |
| warm-up | 4.56 | 30.1 | pipelines/compile_ms 1172, pipelines/front_ms 1138, pipelines/wait_ms 66.13, render/thread/ms 63.78, render/busy/ms 61.94, main/busy/ms 51.26, pipelines/driver_ms 34.61, stage/main/post_update 32.14, stage/render/prepare_flush 23.54, stage/render/render 14.98, render/graph/ms 12.41, stage/main/update 12.15 |
| warm-up | 4.61 | 49.0 | pipelines/compile_ms 851, pipelines/front_ms 827, pipelines/wait_ms 56.56, render/thread/ms 53.63, render/busy/ms 52.02, main/busy/ms 32.30, pipelines/driver_ms 23.65, stage/render/prepare_assets 22.29, stage/render/render 14.29, stage/main/post_update 13.17, stage/main/update 12.55, render/graph/ms 12.09 |
| warm-up | 4.66 | 53.5 | pipelines/compile_ms 756, pipelines/front_ms 731, pipelines/wait_ms 88.95, render/thread/ms 69.17, render/busy/ms 67.85, stage/render/prepare_assets 40.24, main/busy/ms 31.05, pipelines/driver_ms 25.16, stage/render/render 22.41, render/graph/ms 20.57, stage/render/graph_submit 18.19, stage/main/update 15.86 |
| warm-up | 4.69 | 25.6 | pipelines/compile_ms 944, pipelines/front_ms 908, pipelines/wait_ms 259, render/thread/ms 59.84, render/busy/ms 58.44, pipelines/driver_ms 36.64, main/busy/ms 32.35, stage/render/prepare_assets 28.27, stage/render/render 22.33, render/graph/ms 20.41, stage/render/graph_submit 17.98, stage/main/update 17.41 |
| warm-up | 4.81 | 56.8 | pipelines/compile_ms 826, pipelines/front_ms 806, main/busy/ms 50.58, stage/main/update 36.86, render/thread/ms 36.28, render/busy/ms 34.86, stream/drive/ms 22.44, pipelines/driver_ms 20.48, stage/render/prepare_assets 14.72, stage/render/render 10.99, pipelines/wait_ms 9.53, stage/main/post_update 9.11 |
| warm-up | 4.86 | 29.2 | pipelines/compile_ms 389, pipelines/front_ms 375, main/busy/ms 38.41, render/thread/ms 30.76, render/busy/ms 29.27, stage/main/update 25.60, stream/drive/ms 18.88, stage/render/render 14.14, pipelines/driver_ms 13.86, render/graph/ms 12.27, stage/render/graph_submit 9.32, stage/render/prepare_assets 8.93 |
| warm-up | 4.94 | 54.4 | pipelines/compile_ms 59.46, pipelines/front_ms 56.05, main/busy/ms 46.46, render/thread/ms 40.86, render/busy/ms 39.44, stage/main/update 32.67, stream/drive/ms 23.93, stage/render/render 12.23, stage/main/post_update 10.49, render/graph/ms 10.43, stage/render/prepare_bind_groups 7.53, stage/render/graph_submit 6.48 |
| warm-up | 4.99 | 48.6 | pipelines/compile_ms 244, pipelines/front_ms 234, render/thread/ms 75.77, render/busy/ms 74.52, main/busy/ms 40.59, stage/main/update 19.10, stage/render/prepare_assets 18.20, stage/main/post_update 17.16, stage/render/prepare_meshes 15.47, stage/render/render 11.77, stage/render/prepare_bind_groups 11.17, render/graph/ms 10.20 |
| warm-up | 5.03 | 36.7 | pipelines/compile_ms 1826, pipelines/front_ms 1776, pipelines/wait_ms 105, render/thread/ms 69.63, render/busy/ms 68.10, pipelines/driver_ms 50.10, main/busy/ms 42.08, stage/render/prepare_assets 28.19, stage/main/post_update 17.84, stage/main/update 16.79, stage/render/render 10.20, stage/render/prepare_meshes 9.71 |
| warm-up | 5.39 | 58.8 | pipelines/compile_ms 154, pipelines/front_ms 149, main/busy/ms 58.74, render/thread/ms 43.63, stage/main/update 43.10, render/busy/ms 41.89, stream/drive/ms 13.90, stage/render/prepare_assets 13.19, stage/render/render 12.08, stage/main/post_update 10.04, render/graph/ms 9.84, encode//sum 7.62 |
| warm-up | 5.46 | 35.3 | main/busy/ms 78.01, stage/main/update 67.40, stream/drive/ms 62.40, render/thread/ms 19.81, render/busy/ms 18.68, stage/render/render 8.82, render/graph/ms 7.41, stage/main/post_update 7.09, encode//sum 6.75, stage/render/graph_submit 5.35, stage/render/prepare_assets 4.63, encode/submit 3.39 |
| warm-up | 5.51 | 50.5 | pipelines/compile_ms 206, pipelines/front_ms 199, pipelines/wait_ms 80.74, main/busy/ms 52.93, stage/main/update 40.42, stream/drive/ms 34.83, render/thread/ms 23.03, render/busy/ms 22.00, stage/render/prepare_assets 9.51, stage/main/post_update 8.00, stage/render/render 7.10, pipelines/driver_ms 7.01 |
| warm-up | 6.26 | 29.0 | pipelines/compile_ms 146, pipelines/front_ms 144, render/thread/ms 28.67, render/busy/ms 27.70, main/busy/ms 19.29, stage/render/prepare_assets 14.27, stage/main/post_update 9.45, stage/render/render 7.58, render/graph/ms 6.33, encode//sum 5.90, stage/main/update 5.75, stage/render/graph_submit 4.51 |
| warm-up | 6.44 | 26.8 | main/busy/ms 28.53, render/thread/ms 28.12, render/busy/ms 26.89, stage/main/post_update 12.34, stage/render/render 11.36, stage/main/update 11.22, render/graph/ms 9.67, stage/render/prepare_assets 6.87, encode//sum 6.83, stage/render/graph_submit 6.76, encode/submit 4.74, stage/main/pre_update 3.15 |
| warm-up | 6.55 | 29.9 | render/thread/ms 28.24, render/busy/ms 27.14, main/busy/ms 20.38, stage/main/post_update 10.99, stage/render/prepare_assets 10.82, stage/render/render 10.52, render/graph/ms 8.87, encode//sum 7.21, stage/main/update 6.16, stage/render/graph_submit 5.48, stage/render/camera_driver 3.22, encode/submit 3.02 |
| warm-up | 7.25 | 26.4 | main/busy/ms 29.98, render/thread/ms 24.60, pipelines/compile_ms 23.80, render/busy/ms 22.94, pipelines/front_ms 22.86, stage/main/update 17.48, stream/drive/ms 13.19, stage/render/render 11.51, render/graph/ms 10.14, stage/main/post_update 10.13, encode//sum 8.92, stage/render/graph_submit 7.59 |
| warm-up | 7.49 | 43.2 | render/thread/ms 49.93, render/busy/ms 46.85, main/busy/ms 19.95, stage/render/specialize 14.89, stage/render/render 10.18, stage/render/queue 9.71, stage/main/post_update 9.50, render/graph/ms 8.85, stage/main/update 6.82, encode//sum 6.81, stage/render/graph_submit 6.79, stage/render/prepare_meshes 5.18 |
| warm-up | 7.72 | 27.2 | pipelines/compile_ms 143, pipelines/front_ms 138, render/thread/ms 35.77, render/busy/ms 34.47, main/busy/ms 29.20, stage/render/prepare_assets 15.51, stage/main/post_update 14.79, stage/render/render 12.33, render/graph/ms 10.11, encode//sum 8.64, stage/main/update 8.52, stage/render/graph_submit 7.04 |
| measure | 0.52 | 40.8 | pipelines/compile_ms 117, pipelines/front_ms 113, render/thread/ms 46.43, render/busy/ms 43.91, main/busy/ms 16.96, stage/render/specialize 15.35, stage/main/post_update 9.81, stage/render/queue 9.80, stage/render/render 9.21, render/graph/ms 7.93, stage/render/graph_submit 6.04, encode//sum 5.68 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora@2560x1440_arm=noprecomb-4 --bench-baseline none --off precombine
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 4%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 83%) · 2616 MHz (min 2610) · memory 8101 MHz · 98.0 W · 79 °C (max 81) · VRAM 6189 MiB · throttled 100% of samples

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
