# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1537 frames over 15.0 s · **median 9.76 ms (102 fps)** · mean 9.76 · p95 11.21 · p99 12.01 (1% low 83 fps) · worst 25.97 ms · 4 spikes over 1.5× median · 16.0 stutters/min (work 4.0/min)
- warm-up: 280 frames over 8.4 s, worst 2604.6 ms
- hitches (> 25.0 ms): **1** measured, 54 in warm-up
- timed GPU passes: 8.47 ms of a 9.76 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.76 ms, limit 11 ms
- pass worst frame 25.97 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.827 | 2.842 | 2.974 | 0.99 | 0.062 |
| dlss_super_resolution | 2.012 | 2.022 | 2.045 | 0.99 | 0.187 |
| early prepass | 0.624 | 0.627 | 0.642 | 0.99 | 0.094 |
| atmosphere_luts | 0.323 | 0.324 | 0.394 | 0.99 | 0.003 |
| bloom | 0.313 | 0.314 | 0.331 | 0.99 | 0.062 |
| main_transmissive_pass_3d | 0.292 | 0.293 | 0.332 | 0.99 | 0.008 |
| water_velocity | 0.238 | 0.239 | 0.265 | 0.99 | 0.002 |
| water_sspr | 0.203 | 0.204 | 0.213 | 0.99 | 0.002 |
| ssao | 0.191 | 0.192 | 0.198 | 0.99 | 0.007 |
| clouds_trace | 0.158 | 0.158 | 0.193 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.144 | 0.144 | 0.152 | 0.99 | 0.001 |
| water_fft | 0.132 | 0.133 | 0.136 | 0.99 | 0.003 |
| uniform_allocation | 0.119 | 0.120 | 0.161 | 0.99 | 0.021 |
| upscaling | 0.112 | 0.113 | 0.118 | 0.99 | 0.004 |
| clouds_composite | 0.112 | 0.112 | 0.115 | 0.99 | 0.002 |
| bin_unpacking | 0.091 | 0.091 | 0.121 | 0.99 | 0.015 |
| tonemapping | 0.086 | 0.086 | 0.093 | 0.99 | 0.003 |
| clustering | 0.066 | 0.066 | 0.078 | 0.99 | 0.020 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.413 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.051 | 0.052 | 0.99 | 0.001 |
| water_snapshot | 0.034 | 0.034 | 0.036 | 0.99 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.030 | 0.030 | 0.041 | 0.99 | 0.004 |
| lightprobe_radiance_map | 0.027 | 0.221 | 0.232 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.026 | 0.029 | 0.99 | 0.082 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.047 | 0.31 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 0.99 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.014 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.008 | 0.99 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.825 | 10.299 | 24.71 | 13563.6 |
| main/busy/ms | 7.138 | 7.986 | 10.36 | 10971.0 |
| render/busy/ms | 7.058 | 8.194 | 24.16 | 10848.4 |
| stage/render/render | 4.772 | 5.749 | 12.39 | 7334.5 |
| stage/main/post_update | 4.246 | 4.964 | 7.27 | 6525.9 |
| render/graph/ms | 4.185 | 5.087 | 7.52 | 6432.1 |
| encode//sum | 3.832 | 5.124 | 25.25 | 5889.7 |
| stage/render/graph_submit | 3.200 | 4.103 | 6.53 | 4917.7 |
| encode/submit | 2.046 | 2.633 | 8.90 | 3144.7 |
| stage/main/update | 1.786 | 2.197 | 3.49 | 2745.3 |
| stage/render/prepare_views | 1.442 | 3.133 | 4.88 | 2216.2 |
| render/acquire/ms | 1.397 | 3.083 | 4.74 | 2147.8 |
| encode/finish_wall | 1.178 | 1.676 | 4.30 | 1810.9 |
| stage/render/camera_driver | 0.930 | 1.068 | 2.31 | 1429.4 |
| encode/pass:shadow_directional_light/sum | 0.746 | 1.296 | 3.92 | 1147.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.725 | 0.988 | 1.66 | 1113.6 |
| encode/pass:early prepass/sum | 0.724 | 1.021 | 2.75 | 1112.4 |
| stage/render/prepare_write_phase_buffers | 0.541 | 0.746 | 0.92 | 831.8 |
| stage/render/prepare_assets | 0.447 | 0.571 | 1.58 | 687.1 |
| stage/render/prepare_resources | 0.383 | 0.444 | 0.63 | 589.0 |
| bigspace/total/ms | 0.378 | 0.899 | 1.74 | 580.9 |
| render/present/ms | 0.369 | 0.441 | 5.05 | 567.4 |
| stage/main/pre_update | 0.339 | 0.444 | 0.93 | 520.7 |
| stage/main/first | 0.291 | 0.354 | 1.40 | 447.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.262 | 0.402 | 0.59 | 402.8 |
| terrain/update/ms | 0.231 | 0.294 | 0.63 | 355.7 |
| stage/render/prepare_bind_groups | 0.203 | 0.275 | 0.42 | 311.9 |
| terrain/select/ms | 0.201 | 0.257 | 0.60 | 309.7 |
| stage/render/prepare_meshes | 0.200 | 0.253 | 2.33 | 307.5 |
| stream/drive/ms | 0.185 | 0.256 | 0.75 | 285.0 |
| stage/main/fixed_main_loop | 0.181 | 0.339 | 0.84 | 277.9 |
| stage/render/prepare_batch_phases | 0.178 | 0.235 | 0.39 | 273.1 |
| stage/render/extract_commands | 0.169 | 0.201 | 0.51 | 259.1 |
| stage/render/create_views | 0.168 | 0.218 | 0.60 | 258.8 |
| far/select/ms | 0.163 | 0.217 | 0.68 | 250.7 |
| stream/tick/ms | 0.157 | 0.223 | 0.72 | 241.7 |
| stage/render/prepare_flush | 0.153 | 0.301 | 0.54 | 235.5 |
| bigspace/high_precision/ms | 0.146 | 0.551 | 1.40 | 224.1 |
| controller/feed/ms | 0.141 | 0.187 | 0.61 | 216.6 |
| controller/move/ms | 0.140 | 0.263 | 0.58 | 214.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.120 | 0.189 | 0.48 | 183.9 |
| stage/main/last | 0.116 | 0.154 | 0.66 | 178.0 |
| stage/render/queue | 0.110 | 0.140 | 5.44 | 169.2 |
| bigspace/low_precision/ms | 0.088 | 0.214 | 1.11 | 135.8 |
| anim/evaluate/ms | 0.088 | 0.208 | 1.23 | 135.6 |
| stage/render/specialize | 0.085 | 0.104 | 8.12 | 130.3 |
| anim/evaluate_cpu/ms | 0.061 | 0.076 | 0.45 | 94.0 |
| stage/render/prepare_collect_phase_buffers | 0.057 | 0.113 | 0.18 | 87.8 |
| encode/resolve_diagnostics/sum | 0.054 | 0.086 | 0.36 | 82.8 |
| physics/step/ms | 0.027 | 0.052 | 0.38 | 40.8 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.043 | 0.31 | 40.2 |
| weather/map/ms | 0.025 | 0.032 | 0.20 | 37.8 |
| encode/camera_driver_span_end/sum | 0.023 | 0.040 | 0.10 | 34.7 |
| stage/render/cleanup | 0.023 | 0.032 | 0.27 | 34.6 |
| stage/render/phase_sort | 0.019 | 0.035 | 0.11 | 30.0 |
| weather/eval/ms | 0.011 | 0.014 | 0.04 | 16.3 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 10.3 |
| controller/avoid/ms | 0.006 | 0.008 | 0.35 | 9.7 |
| script/input/ms | 0.005 | 0.006 | 0.08 | 8.1 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 6.8 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.2 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 3.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.45 | 1.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.6 |
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
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.52 | 3.73 | 0.43 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.31 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.10 | 0.02 |
| encode/finish_critical | ms | 0.86 | 3.92 | 0.79 |
| encode/pass:early prepass/max | ms | 0.72 | 2.75 | 0.54 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.73 | 1.66 | 0.79 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.48 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 0.59 | 0.17 |
| encode/pass:shadow_directional_light/max | ms | 0.68 | 3.92 | 0.38 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.36 | 0.03 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.17 | 24.16 | 9.22 |
| gpu/timed/ms | ms | 9.17 | 10.88 | 9.22 |
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
| phases/shadow/batch_sets | count | 1 | 30 | 1 |
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
| preprocess/bind_groups | count | 0 | 62 | 0 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 127 | 127 | 127 |
| process/cpu_usage | % | 24.66 | 37.62 | 18.27 |
| process/mem_usage | GiB | 4.67 | 4.74 | 4.74 |
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
| skin/blocks_changed/count | count | 4 | 6 | 3 |
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
| system/cpu_usage | % | 30.64 | 41.74 | 30.21 |
| system/mem_usage | % | 58.69 | 58.92 | 58.91 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 53 | 82 | 53 |
| views/all/bins | count | 5324 | 8061 | 5324 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 15.3 k | 27.0 k | 15.3 k |
| views/all/pipelines | count | 71 | 78 | 71 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 9496 | 27.2 k | 9496 |
| views/cascade/batch_sets | count | 1 | 30 | 1 |
| views/cascade/bins | count | 2152 | 4889 | 2152 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 6053 | 17.7 k | 6053 |
| views/cascade/pipelines | count | 23 | 30 | 23 |
| views/cascade/work_items | count | 34 | 17.7 k | 34 |
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
| anim/events/count | count | 0.10 | 3 | 155 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.22 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.56 | 29 | 40.8 k |
| encode/bind_walk/init_textures | count | 7165 | 14.0 k | 11.01 M |
| encode/bind_walk/init_textures_skipped | count | 39.5 k | 52.6 k | 60.69 M |
| encode/bind_walk/inits | count | 345 | 437 | 530.8 k |
| encode/bind_walk/inits_skipped | count | 71.00 | 83 | 109.1 k |
| encode/bind_walk/merge_textures | count | 7101 | 13.9 k | 10.91 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.27 M |
| encode/bind_walk/merges | count | 82.26 | 103 | 126.4 k |
| encode/bind_walk/merges_skipped | count | 52.75 | 58 | 81.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1537 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1537 |
| encode/pass:early prepass/count | count | 1 | 1 | 1537 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1537 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1537 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1537 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1537 |
| encode/queue/command_buffers | count | 36.56 | 39 | 56.2 k |
| encode/queue/submits | count | 2 | 2 | 3074 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.86 M | 2.21 G |
| encode/queue/write_buffer/calls | count | 217 | 288 | 334.0 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.17 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.8 k |
| encode/queue/write_texture/bytes | bytes | 11.7 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1537 |
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

## Hitches (> 25.0 ms; slowest 55, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.60 | 2604.6 | main/busy/ms 2573, stage/main/post_update 2458, stage/main/update 107, surface/advance_cpu_ms 77.39, stage/main/pre_update 4.65, stage/main/last 1.07, surface/cell_upload_ms 0.78, stage/main/first 0.56, stage/main/fixed_main_loop 0.43, stream/drive/ms 0.38, weather/map/ms 0.13, bigspace/total/ms 0.09 |
| warm-up | 3.35 | 743.9 | main/busy/ms 2709, stage/main/post_update 2481, render/thread/ms 701, render/busy/ms 538, stage/render/prepare_assets 490, stage/main/update 213, stage/render/prepare_views 160, render/acquire/ms 159, surface/advance_cpu_ms 77.39, stage/render/create_views 27.90, stage/render/render 12.92, render/graph/ms 7.99 |
| warm-up | 3.46 | 106.7 | render/thread/ms 791, render/busy/ms 580, stage/render/prepare_assets 516, stage/render/prepare_views 160, render/acquire/ms 160, main/busy/ms 155, stage/main/update 110, pipelines/compile_ms 83.66, pipelines/front_ms 75.82, stage/render/render 66.61, render/present/ms 51.21, stage/main/post_update 34.19 |
| warm-up | 3.50 | 43.1 | pipelines/compile_ms 363, pipelines/front_ms 331, render/thread/ms 119, stage/render/render 76.04, main/busy/ms 68.97, render/busy/ms 60.34, render/present/ms 58.01, stage/main/post_update 53.12, pipelines/wait_ms 36.00, pipelines/driver_ms 31.75, stage/render/prepare_assets 27.28, terrain/update/ms 22.98 |
| warm-up | 3.63 | 132.8 | pipelines/wait_ms 4909, pipelines/compile_ms 1520, pipelines/front_ms 1430, main/busy/ms 206, render/thread/ms 125, render/busy/ms 114, stage/main/post_update 101, pipelines/driver_ms 90.74, stage/main/update 86.05, stage/render/prepare_resources 64.48, stage/render/render 27.59, render/graph/ms 15.55 |
| warm-up | 3.76 | 124.7 | pipelines/wait_ms 5119, pipelines/compile_ms 2204, pipelines/front_ms 2090, main/busy/ms 182, render/thread/ms 157, render/busy/ms 156, pipelines/driver_ms 114, stage/main/update 91.46, stage/main/post_update 72.59, stage/render/prepare_resources 64.32, stage/render/prepare_assets 58.33, terrain/update/ms 13.72 |
| warm-up | 3.80 | 42.9 | pipelines/compile_ms 1724, pipelines/front_ms 1640, pipelines/wait_ms 381, render/thread/ms 102, render/busy/ms 99.86, pipelines/driver_ms 83.71, stage/render/prepare_assets 74.86, main/busy/ms 55.87, encode//sum 38.87, stage/main/post_update 19.14, stage/main/update 17.14, stage/render/render 16.95 |
| warm-up | 3.88 | 26.6 | pipelines/compile_ms 285, pipelines/front_ms 274, render/thread/ms 35.84, render/busy/ms 34.17, main/busy/ms 26.20, stage/main/post_update 16.54, stage/render/render 14.61, stage/render/prepare_assets 13.24, render/graph/ms 12.39, pipelines/driver_ms 11.23, terrain/update/ms 9.24, encode//sum 8.79 |
| warm-up | 3.98 | 77.5 | pipelines/compile_ms 961, pipelines/front_ms 920, render/thread/ms 57.08, main/busy/ms 55.21, render/busy/ms 55.19, stage/main/update 44.94, pipelines/driver_ms 40.85, stream/drive/ms 40.74, stage/render/prepare_assets 39.61, stage/render/render 11.32, render/graph/ms 9.14, encode//sum 8.36 |
| warm-up | 4.07 | 73.6 | pipelines/compile_ms 673, pipelines/front_ms 651, render/thread/ms 67.56, render/busy/ms 65.25, main/busy/ms 49.67, stage/render/prepare_assets 35.94, stage/main/update 34.37, stream/drive/ms 29.96, stage/render/render 23.67, pipelines/driver_ms 22.16, render/graph/ms 20.99, stage/render/graph_submit 11.10 |
| warm-up | 4.10 | 30.7 | pipelines/compile_ms 370, pipelines/front_ms 357, render/thread/ms 65.73, render/busy/ms 63.86, stage/render/prepare_assets 43.86, main/busy/ms 27.00, stage/render/render 13.12, pipelines/driver_ms 12.28, stage/main/post_update 10.76, render/graph/ms 10.65, stage/main/update 9.15, stage/render/graph_submit 7.47 |
| warm-up | 4.19 | 80.6 | pipelines/compile_ms 823, pipelines/front_ms 786, main/busy/ms 85.30, render/thread/ms 78.23, render/busy/ms 76.22, stage/main/update 64.72, stage/render/prepare_assets 49.50, stream/drive/ms 40.13, pipelines/driver_ms 37.13, stage/render/render 17.64, render/graph/ms 14.88, stage/main/post_update 11.48 |
| warm-up | 4.26 | 76.5 | pipelines/compile_ms 956, pipelines/front_ms 904, render/thread/ms 109, render/busy/ms 107, main/busy/ms 87.56, stage/main/update 69.93, stage/render/prepare_assets 68.21, pipelines/driver_ms 52.26, stream/drive/ms 39.40, stage/render/render 29.37, render/graph/ms 25.94, stage/render/graph_submit 21.72 |
| warm-up | 4.29 | 27.1 | pipelines/compile_ms 174, pipelines/front_ms 157, render/thread/ms 72.92, render/busy/ms 70.31, main/busy/ms 64.69, stage/main/update 47.39, stream/drive/ms 33.77, stage/render/prepare_assets 33.06, stage/render/render 29.44, render/graph/ms 26.41, stage/render/graph_submit 23.29, pipelines/driver_ms 17.15 |
| warm-up | 4.36 | 72.1 | pipelines/compile_ms 474, pipelines/front_ms 455, main/busy/ms 68.08, render/thread/ms 63.09, render/busy/ms 61.35, stage/main/update 47.99, stream/drive/ms 34.98, stage/render/prepare_assets 34.32, pipelines/wait_ms 34.06, pipelines/driver_ms 19.41, stage/render/render 17.36, render/graph/ms 15.32 |
| warm-up | 4.39 | 30.2 | pipelines/compile_ms 1073, pipelines/front_ms 1035, render/thread/ms 63.73, render/busy/ms 62.40, stage/render/prepare_assets 41.27, pipelines/wait_ms 39.95, main/busy/ms 38.08, pipelines/driver_ms 37.98, stage/main/update 19.38, stage/main/post_update 11.78, pipelines/lock_ms 10.13, stage/render/render 9.22 |
| warm-up | 4.48 | 88.2 | pipelines/compile_ms 1219, pipelines/front_ms 1164, pipelines/wait_ms 137, render/thread/ms 95.50, render/busy/ms 94.11, pipelines/driver_ms 55.40, stage/render/prepare_assets 53.39, main/busy/ms 51.08, stage/main/update 31.35, stage/render/prepare_resources 12.64, stage/main/post_update 11.72, stage/render/render 9.97 |
| warm-up | 4.53 | 46.3 | pipelines/compile_ms 1531, pipelines/front_ms 1467, pipelines/wait_ms 225, render/thread/ms 106, render/busy/ms 104, stage/render/prepare_assets 63.94, pipelines/driver_ms 63.70, main/busy/ms 53.19, stage/main/update 31.15, stage/main/post_update 14.06, stage/render/prepare_resources 12.77, stage/render/render 10.89 |
| warm-up | 4.57 | 44.7 | pipelines/compile_ms 1779, pipelines/front_ms 1730, pipelines/wait_ms 129, render/thread/ms 63.43, render/busy/ms 61.45, pipelines/driver_ms 49.68, stage/render/prepare_assets 43.78, main/busy/ms 37.72, stage/main/update 15.19, stage/main/post_update 14.92, stage/render/render 10.81, render/graph/ms 8.29 |
| warm-up | 4.60 | 25.2 | pipelines/compile_ms 1250, pipelines/front_ms 1217, render/thread/ms 50.36, render/busy/ms 48.23, main/busy/ms 40.38, pipelines/wait_ms 35.30, pipelines/driver_ms 32.95, stage/render/prepare_assets 25.09, stage/main/update 19.84, stage/render/render 14.60, stage/main/post_update 13.43, render/graph/ms 12.35 |
| warm-up | 4.66 | 60.3 | pipelines/compile_ms 635, pipelines/front_ms 616, render/thread/ms 73.76, render/busy/ms 71.52, main/busy/ms 52.97, stage/main/update 35.17, stage/render/render 31.18, render/graph/ms 28.27, stage/render/graph_submit 22.96, encode//sum 19.91, pipelines/driver_ms 19.12, stream/drive/ms 18.47 |
| warm-up | 4.77 | 115.4 | pipelines/compile_ms 1200, pipelines/front_ms 1160, render/thread/ms 158, render/busy/ms 156, main/busy/ms 131, stage/main/update 115, stream/drive/ms 108, stage/render/prepare_assets 42.63, pipelines/driver_ms 39.98, stage/render/prepare_resources 32.01, stage/render/render 26.27, stage/render/prepare_flush 24.03 |
| warm-up | 4.82 | 49.9 | pipelines/compile_ms 2042, pipelines/front_ms 1975, pipelines/wait_ms 151, render/thread/ms 141, render/busy/ms 139, main/busy/ms 119, stage/main/update 99.91, stream/drive/ms 93.41, pipelines/driver_ms 66.26, stage/render/prepare_assets 57.45, stage/render/prepare_resources 31.66, stage/render/prepare_bind_groups 13.18 |
| warm-up | 4.88 | 36.6 | pipelines/compile_ms 871, pipelines/front_ms 842, main/busy/ms 57.26, pipelines/wait_ms 56.10, render/thread/ms 45.33, render/busy/ms 43.24, pipelines/driver_ms 29.17, stage/main/update 27.34, stage/main/post_update 19.46, stage/render/render 15.92, render/graph/ms 13.08, stage/render/graph_submit 9.25 |
| warm-up | 4.94 | 64.6 | pipelines/compile_ms 454, pipelines/front_ms 436, main/busy/ms 72.58, render/thread/ms 69.45, render/busy/ms 67.21, pipelines/wait_ms 54.36, stage/main/update 49.09, stage/render/prepare_assets 23.18, stage/render/render 21.45, render/graph/ms 18.26, pipelines/driver_ms 18.26, stage/main/post_update 14.98 |
| warm-up | 4.99 | 43.6 | pipelines/compile_ms 1080, pipelines/front_ms 1049, pipelines/wait_ms 527, render/thread/ms 73.88, render/busy/ms 71.95, main/busy/ms 59.27, stage/render/prepare_assets 35.62, stage/main/update 35.00, pipelines/driver_ms 31.42, stage/render/render 18.17, stage/main/post_update 17.89, render/graph/ms 15.67 |
| warm-up | 5.05 | 39.4 | pipelines/compile_ms 471, pipelines/front_ms 456, render/thread/ms 56.21, render/busy/ms 54.67, main/busy/ms 50.32, stage/main/update 31.65, stage/render/render 15.98, pipelines/driver_ms 15.76, stage/main/post_update 13.18, render/graph/ms 12.08, stream/drive/ms 8.96, stage/render/graph_submit 8.52 |
| warm-up | 5.08 | 26.4 | pipelines/compile_ms 800, pipelines/front_ms 776, render/thread/ms 53.03, render/busy/ms 51.60, main/busy/ms 50.71, stage/main/update 36.23, pipelines/driver_ms 24.12, stage/render/render 18.28, stream/drive/ms 15.04, render/graph/ms 14.43, stage/render/prepare_assets 11.97, stage/render/graph_submit 10.78 |
| warm-up | 5.11 | 36.9 | pipelines/compile_ms 521, pipelines/front_ms 507, render/thread/ms 48.92, main/busy/ms 48.20, render/busy/ms 47.07, stage/main/update 28.08, stage/render/prepare_assets 18.49, pipelines/driver_ms 13.84, stage/main/post_update 13.70, stage/render/render 12.66, stream/drive/ms 11.04, render/graph/ms 10.94 |
| warm-up | 5.17 | 57.5 | pipelines/compile_ms 579, pipelines/front_ms 564, render/thread/ms 82.52, render/busy/ms 80.46, main/busy/ms 66.96, stage/main/post_update 38.81, stage/main/update 22.25, stage/render/prepare_batch_phases 18.84, stage/render/prepare_assets 18.45, stage/render/prepare_resources 15.49, pipelines/driver_ms 15.06, stage/render/render 11.53 |
| warm-up | 5.20 | 25.8 | pipelines/compile_ms 1027, pipelines/front_ms 998, render/thread/ms 74.54, render/busy/ms 73.14, main/busy/ms 62.67, stage/main/post_update 35.20, pipelines/driver_ms 28.49, stage/main/update 21.02, stage/render/prepare_batch_phases 19.93, stage/render/prepare_assets 14.33, stage/render/prepare_resources 13.34, stage/render/render 11.96 |
| warm-up | 5.24 | 43.5 | pipelines/compile_ms 1280, pipelines/front_ms 1248, render/thread/ms 53.96, render/busy/ms 52.50, main/busy/ms 38.15, pipelines/driver_ms 31.45, stage/render/prepare_assets 28.06, stage/main/update 19.62, stage/main/post_update 11.59, stage/render/render 9.95, render/graph/ms 8.07, encode//sum 6.33 |
| warm-up | 5.28 | 36.3 | pipelines/compile_ms 1182, pipelines/front_ms 1147, render/thread/ms 63.60, render/busy/ms 61.89, main/busy/ms 46.39, pipelines/driver_ms 34.76, stage/main/update 26.02, stage/render/prepare_assets 25.79, stage/render/render 14.06, stage/main/post_update 12.83, render/graph/ms 11.93, stage/render/graph_submit 8.61 |
| warm-up | 5.31 | 29.5 | pipelines/compile_ms 1191, pipelines/front_ms 1157, render/thread/ms 51.88, render/busy/ms 50.50, main/busy/ms 42.93, pipelines/driver_ms 33.39, pipelines/wait_ms 31.77, stage/main/update 24.64, stage/render/prepare_assets 19.02, stage/render/render 12.89, render/graph/ms 10.98, stage/main/post_update 10.02 |
| warm-up | 5.39 | 25.2 | render/thread/ms 33.03, render/busy/ms 31.95, main/busy/ms 27.45, stage/main/update 14.92, stage/render/prepare_assets 13.19, stage/render/render 11.00, render/graph/ms 9.51, stream/drive/ms 9.02, encode//sum 8.64, stage/main/post_update 8.31, stage/render/graph_submit 6.87, encode/submit 3.96 |
| warm-up | 5.48 | 32.4 | render/thread/ms 41.62, render/busy/ms 40.39, main/busy/ms 33.21, stage/render/prepare_assets 17.59, stage/main/update 17.18, stage/render/render 13.22, render/graph/ms 11.06, encode//sum 9.90, stage/main/post_update 9.75, stage/render/graph_submit 7.75, encode/submit 4.91, stream/drive/ms 4.47 |
| warm-up | 5.56 | 61.1 | render/thread/ms 71.94, pipelines/compile_ms 71.57, render/busy/ms 70.53, pipelines/front_ms 66.98, stage/render/prepare_assets 28.64, stage/render/render 26.93, main/busy/ms 25.88, render/graph/ms 25.18, pipelines/wait_ms 16.40, stage/render/graph_submit 13.48, stage/render/camera_driver 11.54, stage/main/update 11.23 |
| warm-up | 5.63 | 55.6 | pipelines/compile_ms 204, pipelines/front_ms 199, render/thread/ms 58.11, render/busy/ms 56.87, stage/render/prepare_assets 28.19, main/busy/ms 27.78, stage/main/update 11.66, stage/render/render 10.76, stage/main/post_update 10.01, render/graph/ms 9.13, encode//sum 7.00, stage/render/graph_submit 6.64 |
| warm-up | 5.69 | 43.0 | render/thread/ms 54.52, render/busy/ms 53.37, main/busy/ms 32.00, stage/render/prepare_assets 28.35, stage/main/update 14.92, stage/render/render 12.70, stage/main/post_update 11.60, render/graph/ms 10.99, encode//sum 9.49, stage/render/graph_submit 7.35, encode/submit 4.98, stream/drive/ms 4.98 |
| warm-up | 5.72 | 25.8 | pipelines/compile_ms 692, pipelines/front_ms 673, render/thread/ms 59.74, render/busy/ms 58.19, main/busy/ms 39.84, stage/render/prepare_assets 29.40, stage/main/post_update 20.64, stage/render/render 19.76, pipelines/driver_ms 18.98, render/graph/ms 17.52, stage/main/update 13.92, stage/render/graph_submit 11.14 |
| warm-up | 5.75 | 34.0 | pipelines/compile_ms 1189, pipelines/front_ms 1151, render/thread/ms 53.68, render/busy/ms 52.13, main/busy/ms 51.02, pipelines/driver_ms 37.05, stage/main/post_update 24.76, stage/main/update 19.88, stage/render/prepare_assets 19.82, stage/render/render 19.51, render/graph/ms 17.20, pipelines/wait_ms 10.82 |
| warm-up | 5.78 | 31.9 | pipelines/compile_ms 497, pipelines/front_ms 479, render/thread/ms 54.99, render/busy/ms 53.69, main/busy/ms 44.83, stage/render/prepare_assets 27.62, stage/main/update 20.38, pipelines/driver_ms 18.08, stage/main/post_update 17.57, stage/render/render 12.25, render/graph/ms 10.12, encode//sum 8.45 |
| warm-up | 5.82 | 32.3 | render/thread/ms 50.50, render/busy/ms 48.72, main/busy/ms 38.90, stage/render/prepare_assets 18.95, stage/main/post_update 17.88, stage/render/render 16.54, render/graph/ms 13.95, stage/main/update 13.92, encode//sum 9.80, stage/render/graph_submit 9.32, encode/submit 6.15, stage/render/camera_driver 4.43 |
| warm-up | 5.85 | 30.2 | main/busy/ms 54.25, render/thread/ms 45.43, render/busy/ms 43.54, stage/main/update 31.87, stage/render/prepare_assets 19.25, stream/drive/ms 17.54, stage/main/post_update 15.72, stage/render/render 14.55, render/graph/ms 12.10, encode//sum 8.97, stage/render/graph_submit 8.24, encode/submit 5.46 |
| warm-up | 5.90 | 50.1 | pipelines/compile_ms 598, pipelines/front_ms 581, main/busy/ms 49.02, render/thread/ms 42.63, render/busy/ms 41.15, stage/main/update 30.18, stage/render/prepare_assets 22.47, stream/drive/ms 18.01, pipelines/driver_ms 16.98, stage/main/post_update 12.35, stage/render/render 10.59, render/graph/ms 8.68 |
| warm-up | 5.92 | 26.1 | pipelines/compile_ms 819, pipelines/front_ms 795, render/thread/ms 44.25, render/busy/ms 42.70, main/busy/ms 38.59, pipelines/driver_ms 23.33, stage/main/update 19.45, stage/render/prepare_assets 17.08, stage/render/render 16.21, render/graph/ms 14.25, stage/main/post_update 12.50, stage/render/graph_submit 11.20 |
| warm-up | 5.95 | 25.6 | pipelines/compile_ms 258, pipelines/front_ms 250, render/thread/ms 39.42, main/busy/ms 37.96, render/busy/ms 37.85, stage/main/update 19.14, stage/render/render 16.04, render/graph/ms 14.03, stage/render/prepare_assets 12.34, stage/main/post_update 12.12, stage/render/graph_submit 10.77, pipelines/driver_ms 7.33 |
| warm-up | 6.08 | 30.2 | pipelines/compile_ms 380, pipelines/front_ms 368, main/busy/ms 44.66, render/thread/ms 28.49, stage/main/update 27.80, render/busy/ms 27.19, stream/drive/ms 18.89, pipelines/driver_ms 12.08, stage/render/render 11.34, stage/main/post_update 11.01, render/graph/ms 9.56, encode//sum 8.58 |
| warm-up | 6.12 | 34.6 | pipelines/compile_ms 1064, pipelines/front_ms 1034, main/busy/ms 46.83, pipelines/driver_ms 30.40, render/thread/ms 30.27, render/busy/ms 29.14, stage/main/update 28.40, stream/drive/ms 19.03, stage/render/prepare_assets 11.98, stage/main/post_update 11.52, stage/render/render 9.34, render/graph/ms 7.64 |
| warm-up | 6.19 | 28.6 | pipelines/compile_ms 178, pipelines/front_ms 173, main/busy/ms 35.97, render/thread/ms 28.00, render/busy/ms 26.61, stage/main/update 19.19, stage/render/render 11.98, stage/main/post_update 11.37, render/graph/ms 10.06, stage/render/prepare_assets 9.81, stage/render/graph_submit 7.23, encode//sum 7.08 |
| warm-up | 6.39 | 38.0 | render/thread/ms 43.54, render/busy/ms 41.79, main/busy/ms 29.10, stage/render/prepare_assets 18.52, stage/render/render 15.59, stage/main/update 13.16, render/graph/ms 13.10, stage/main/post_update 11.43, encode//sum 10.19, stage/render/graph_submit 9.04, encode/submit 5.60, stage/render/camera_driver 3.86 |
| warm-up | 6.47 | 56.1 | render/thread/ms 50.21, render/busy/ms 49.01, main/busy/ms 41.88, stage/render/prepare_assets 31.55, stage/main/update 22.30, stage/main/post_update 12.94, pipelines/compile_ms 11.73, stream/drive/ms 11.25, pipelines/front_ms 11.05, stage/render/render 10.15, render/graph/ms 8.42, encode//sum 6.90 |
| warm-up | 6.82 | 30.0 | render/thread/ms 36.60, render/busy/ms 35.57, main/busy/ms 14.62, stage/render/render 10.81, render/graph/ms 9.51, stage/render/specialize 9.04, stage/main/post_update 7.93, stage/render/graph_submit 7.74, encode//sum 6.58, stage/render/queue 6.12, encode/submit 4.85, stage/main/update 4.37 |
| warm-up | 8.05 | 28.3 | render/thread/ms 34.90, render/busy/ms 32.03, main/busy/ms 14.17, stage/render/render 9.97, render/graph/ms 8.62, stage/render/specialize 8.49, stage/main/post_update 8.02, stage/render/graph_submit 6.63, encode//sum 6.17, stage/render/queue 5.22, stage/render/prepare_meshes 3.91, encode/submit 3.83 |
| measure | 0.83 | 26.0 | render/thread/ms 33.64, render/busy/ms 30.28, main/busy/ms 13.98, stage/render/render 9.83, render/graph/ms 8.52, stage/render/specialize 8.19, stage/main/post_update 8.00, stage/render/graph_submit 6.55, encode//sum 5.71, stage/render/queue 5.56, encode/submit 3.70, stage/main/update 3.44 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=base-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 8%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 85%) · 2624 MHz (min 2610) · memory 8101 MHz · 97.6 W · 78 °C (max 80) · VRAM 6427 MiB · throttled 100% of samples

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
