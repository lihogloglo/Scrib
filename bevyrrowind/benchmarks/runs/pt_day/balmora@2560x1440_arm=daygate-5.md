# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.day_gate=on (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1544 frames over 15.0 s · **median 9.73 ms (103 fps)** · mean 9.72 · p95 11.07 · p99 11.54 (1% low 87 fps) · worst 25.76 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 280 frames over 8.6 s, worst 2567.8 ms
- hitches (> 25.0 ms): **1** measured, 61 in warm-up
- timed GPU passes: 8.58 ms of a 9.72 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.73 ms, limit 11 ms
- pass worst frame 25.76 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.938 | 2.944 | 2.986 | 1.00 | 0.064 |
| dlss_super_resolution | 2.017 | 2.021 | 2.044 | 1.00 | 0.184 |
| early prepass | 0.605 | 0.607 | 0.617 | 1.00 | 0.092 |
| atmosphere_luts | 0.326 | 0.326 | 0.341 | 1.00 | 0.003 |
| bloom | 0.315 | 0.315 | 0.331 | 1.00 | 0.060 |
| main_transmissive_pass_3d | 0.311 | 0.311 | 0.333 | 1.00 | 0.008 |
| water_velocity | 0.239 | 0.240 | 0.264 | 1.00 | 0.002 |
| water_sspr | 0.207 | 0.208 | 0.210 | 1.00 | 0.002 |
| ssao | 0.191 | 0.191 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.149 | 0.149 | 0.169 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.146 | 0.146 | 0.152 | 1.00 | 0.001 |
| water_fft | 0.131 | 0.131 | 0.134 | 1.00 | 0.003 |
| uniform_allocation | 0.118 | 0.119 | 0.160 | 1.00 | 0.022 |
| upscaling | 0.113 | 0.113 | 0.119 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.090 | 0.090 | 0.121 | 1.00 | 0.015 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.064 | 0.065 | 0.075 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.412 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.032 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.228 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.030 | 1.00 | 0.079 |
| water_snapshot | 0.025 | 0.025 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.044 | 0.047 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.024 | 1.00 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.014 | 0.014 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.796 | 10.157 | 24.40 | 13580.6 |
| main/busy/ms | 7.069 | 7.900 | 9.39 | 10914.1 |
| render/busy/ms | 6.851 | 7.842 | 23.93 | 10578.0 |
| stage/render/render | 4.586 | 5.373 | 12.17 | 7080.2 |
| stage/main/post_update | 4.231 | 4.951 | 6.50 | 6533.3 |
| render/graph/ms | 4.007 | 4.763 | 7.13 | 6186.6 |
| encode//sum | 3.850 | 5.072 | 7.81 | 5944.8 |
| stage/render/graph_submit | 3.029 | 3.750 | 5.84 | 4676.8 |
| encode/submit | 1.910 | 2.324 | 7.81 | 2949.0 |
| stage/main/update | 1.740 | 2.155 | 2.99 | 2686.7 |
| stage/render/prepare_views | 1.626 | 3.219 | 4.44 | 2510.0 |
| render/acquire/ms | 1.581 | 3.166 | 4.40 | 2441.5 |
| encode/finish_wall | 1.144 | 1.619 | 2.19 | 1767.1 |
| stage/render/camera_driver | 0.922 | 1.063 | 1.33 | 1423.9 |
| encode/pass:shadow_directional_light/sum | 0.722 | 1.266 | 1.80 | 1114.4 |
| encode/pass:early prepass/sum | 0.700 | 0.982 | 1.37 | 1080.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.691 | 0.957 | 1.21 | 1067.3 |
| stage/render/prepare_write_phase_buffers | 0.542 | 0.744 | 0.97 | 836.1 |
| stage/render/prepare_assets | 0.445 | 0.572 | 1.55 | 687.3 |
| stage/render/prepare_resources | 0.380 | 0.435 | 0.56 | 586.4 |
| render/present/ms | 0.363 | 0.422 | 4.75 | 561.1 |
| bigspace/total/ms | 0.353 | 0.710 | 1.54 | 545.3 |
| stage/main/pre_update | 0.334 | 0.409 | 0.90 | 516.3 |
| stage/main/first | 0.290 | 0.353 | 1.49 | 447.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.236 | 0.339 | 0.68 | 364.7 |
| terrain/update/ms | 0.230 | 0.292 | 0.64 | 355.5 |
| stage/render/prepare_bind_groups | 0.203 | 0.275 | 0.39 | 313.9 |
| terrain/select/ms | 0.201 | 0.254 | 0.62 | 309.9 |
| stage/render/prepare_meshes | 0.197 | 0.242 | 2.21 | 304.0 |
| stream/drive/ms | 0.182 | 0.229 | 0.70 | 280.8 |
| stage/main/fixed_main_loop | 0.179 | 0.331 | 1.04 | 276.7 |
| stage/render/prepare_batch_phases | 0.172 | 0.233 | 0.34 | 265.8 |
| stage/render/extract_commands | 0.167 | 0.196 | 0.56 | 258.3 |
| stage/render/create_views | 0.165 | 0.208 | 0.71 | 254.5 |
| far/select/ms | 0.160 | 0.211 | 0.67 | 247.8 |
| controller/feed/ms | 0.157 | 0.205 | 0.65 | 243.1 |
| stream/tick/ms | 0.154 | 0.198 | 0.66 | 237.7 |
| stage/render/prepare_flush | 0.147 | 0.256 | 0.76 | 226.9 |
| bigspace/high_precision/ms | 0.127 | 0.285 | 1.31 | 196.6 |
| controller/move/ms | 0.122 | 0.180 | 0.63 | 188.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.117 | 0.183 | 0.47 | 181.1 |
| stage/main/last | 0.116 | 0.157 | 0.78 | 179.5 |
| stage/render/queue | 0.106 | 0.135 | 5.25 | 163.6 |
| bigspace/low_precision/ms | 0.085 | 0.177 | 1.32 | 130.9 |
| stage/render/specialize | 0.084 | 0.102 | 8.18 | 129.4 |
| anim/evaluate/ms | 0.083 | 0.200 | 1.03 | 128.8 |
| anim/evaluate_cpu/ms | 0.060 | 0.074 | 0.57 | 92.0 |
| stage/render/prepare_collect_phase_buffers | 0.059 | 0.116 | 0.28 | 91.0 |
| encode/resolve_diagnostics/sum | 0.055 | 0.085 | 0.18 | 84.5 |
| physics/step/ms | 0.027 | 0.053 | 0.48 | 41.8 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.040 | 0.14 | 40.0 |
| weather/map/ms | 0.023 | 0.031 | 0.06 | 35.8 |
| encode/camera_driver_span_end/sum | 0.023 | 0.038 | 0.09 | 35.4 |
| stage/render/cleanup | 0.023 | 0.031 | 0.26 | 34.8 |
| stage/render/phase_sort | 0.020 | 0.035 | 0.17 | 30.9 |
| weather/eval/ms | 0.011 | 0.014 | 0.44 | 16.9 |
| script/update/ms | 0.007 | 0.008 | 0.39 | 10.4 |
| controller/avoid/ms | 0.006 | 0.007 | 0.04 | 8.5 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 8.4 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 6.6 |
| script/ui/ms | 0.004 | 0.006 | 0.37 | 6.5 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.07 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
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
| encode//max | ms | 0.54 | 1.67 | 0.74 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.14 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.09 | 0.06 |
| encode/finish_critical | ms | 0.83 | 1.80 | 0.91 |
| encode/pass:early prepass/max | ms | 0.70 | 1.37 | 0.73 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.70 | 1.21 | 0.91 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.47 | 0.26 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.68 | 0.24 |
| encode/pass:shadow_directional_light/max | ms | 0.67 | 1.80 | 0.61 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.18 | 0.15 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.22 | 23.93 | 9.20 |
| gpu/timed/ms | ms | 9.22 | 10.81 | 9.20 |
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
| process/cpu_usage | % | 25.84 | 36.84 | 24.89 |
| process/mem_usage | GiB | 4.63 | 4.71 | 4.71 |
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
| skin/blocks_changed/count | count | 5 | 6 | 5 |
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
| system/cpu_usage | % | 30.53 | 38.28 | 33.55 |
| system/mem_usage | % | 60.12 | 60.36 | 60.36 |
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
| anim/events/count | count | 0.11 | 3 | 164 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.21 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 29 | 41.0 k |
| encode/bind_walk/init_textures | count | 7166 | 14.0 k | 11.06 M |
| encode/bind_walk/init_textures_skipped | count | 43.4 k | 56.5 k | 66.96 M |
| encode/bind_walk/inits | count | 345 | 437 | 533.3 k |
| encode/bind_walk/inits_skipped | count | 75 | 87 | 115.8 k |
| encode/bind_walk/merge_textures | count | 7102 | 13.9 k | 10.97 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.30 M |
| encode/bind_walk/merges | count | 82.26 | 103 | 127.0 k |
| encode/bind_walk/merges_skipped | count | 54.75 | 60 | 84.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1544 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1544 |
| encode/pass:early prepass/count | count | 1 | 1 | 1544 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1544 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1544 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1544 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1544 |
| encode/queue/command_buffers | count | 36.57 | 39 | 56.5 k |
| encode/queue/submits | count | 2 | 2 | 3088 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.65 M | 2.22 G |
| encode/queue/write_buffer/calls | count | 217 | 287 | 335.5 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.40 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.9 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1544 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 2 | 900 |
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
| warm-up | 2.57 | 2567.8 | main/busy/ms 2538, stage/main/post_update 2432, stage/main/update 96.97, surface/advance_cpu_ms 65.66, stage/main/pre_update 4.79, stage/main/last 1.13, surface/cell_upload_ms 0.78, stage/main/first 0.58, stream/drive/ms 0.47, stage/main/fixed_main_loop 0.43, weather/map/ms 0.17, stream/tick/ms 0.10 |
| warm-up | 3.36 | 790.6 | main/busy/ms 2683, stage/main/post_update 2456, render/thread/ms 754, render/busy/ms 606, stage/render/prepare_assets 556, stage/main/update 212, stage/render/prepare_views 145, render/acquire/ms 145, surface/advance_cpu_ms 65.66, stage/render/create_views 28.81, stage/render/render 13.02, render/graph/ms 8.35 |
| warm-up | 3.47 | 111.9 | render/thread/ms 848, render/busy/ms 648, stage/render/prepare_assets 584, main/busy/ms 238, stage/render/prepare_views 145, render/acquire/ms 145, stage/main/update 124, stage/render/render 73.87, pipelines/compile_ms 69.36, pipelines/front_ms 57.96, render/present/ms 55.10, stage/main/post_update 45.67 |
| warm-up | 3.52 | 52.5 | pipelines/compile_ms 374, pipelines/front_ms 323, main/busy/ms 145, render/thread/ms 130, pipelines/wait_ms 104, stage/render/render 90.69, render/busy/ms 66.56, stage/main/post_update 66.36, render/present/ms 63.00, pipelines/driver_ms 51.06, stage/main/last 32.22, encode//sum 31.47 |
| warm-up | 3.66 | 139.0 | pipelines/wait_ms 5137, pipelines/compile_ms 1825, pipelines/front_ms 1672, pipelines/driver_ms 153, render/thread/ms 148, render/busy/ms 136, main/busy/ms 72.06, stage/render/render 63.47, stage/main/post_update 54.67, render/graph/ms 50.35, stage/render/graph_submit 47.14, stage/render/prepare_flush 33.28 |
| warm-up | 3.69 | 33.0 | pipelines/wait_ms 5143, pipelines/compile_ms 2194, pipelines/front_ms 2043, pipelines/driver_ms 151, render/thread/ms 143, render/busy/ms 141, main/busy/ms 89.35, stage/main/update 62.10, stage/render/prepare_resources 39.55, stage/render/render 38.83, render/graph/ms 36.30, stage/render/graph_submit 33.72 |
| warm-up | 3.79 | 100.2 | pipelines/compile_ms 1035, pipelines/front_ms 983, pipelines/wait_ms 248, main/busy/ms 88.53, render/thread/ms 87.14, render/busy/ms 85.15, stage/main/update 61.16, stage/render/prepare_assets 57.51, pipelines/driver_ms 51.94, stage/main/post_update 19.76, terrain/update/ms 14.43, stage/render/render 10.21 |
| warm-up | 3.88 | 89.7 | pipelines/compile_ms 2014, pipelines/front_ms 1708, pipelines/wait_ms 321, pipelines/driver_ms 307, render/thread/ms 144, render/busy/ms 142, stage/render/prepare_assets 112, main/busy/ms 40.38, stage/render/render 23.04, render/graph/ms 20.79, stage/render/graph_submit 18.06, stage/main/post_update 18.05 |
| warm-up | 3.98 | 27.6 | pipelines/compile_ms 415, pipelines/front_ms 394, main/busy/ms 61.09, stage/main/update 48.83, stream/drive/ms 45.42, render/thread/ms 44.08, render/busy/ms 41.78, stage/render/prepare_assets 24.61, pipelines/driver_ms 20.92, stage/render/render 11.82, render/graph/ms 9.33, encode//sum 7.35 |
| warm-up | 4.06 | 75.7 | pipelines/compile_ms 1173, pipelines/front_ms 1126, pipelines/wait_ms 113, render/thread/ms 66.92, render/busy/ms 65.22, main/busy/ms 60.60, stage/main/update 49.46, stage/render/prepare_assets 47.88, pipelines/driver_ms 46.40, stream/drive/ms 45.63, stage/render/render 12.23, render/graph/ms 10.27 |
| warm-up | 4.08 | 26.9 | pipelines/compile_ms 1099, pipelines/front_ms 1056, pipelines/wait_ms 115, render/thread/ms 66.63, render/busy/ms 64.85, main/busy/ms 43.40, pipelines/driver_ms 42.85, stage/render/prepare_assets 36.19, stage/main/update 26.38, stream/drive/ms 21.31, stage/render/render 13.69, render/graph/ms 11.49 |
| warm-up | 4.13 | 51.4 | pipelines/compile_ms 528, pipelines/front_ms 503, render/thread/ms 59.55, render/busy/ms 57.53, main/busy/ms 46.08, stage/main/update 27.46, stage/render/prepare_assets 26.47, pipelines/driver_ms 24.46, stream/drive/ms 22.31, stage/render/render 14.84, render/graph/ms 12.32, stage/main/post_update 9.55 |
| warm-up | 4.17 | 33.9 | pipelines/compile_ms 197, pipelines/front_ms 190, render/thread/ms 59.70, render/busy/ms 57.85, main/busy/ms 45.01, stage/render/prepare_assets 37.53, stage/main/update 28.36, stream/drive/ms 22.89, stage/render/render 13.18, render/graph/ms 10.83, stage/main/post_update 8.67, stage/render/graph_submit 7.90 |
| warm-up | 4.26 | 93.1 | pipelines/compile_ms 409, pipelines/front_ms 399, main/busy/ms 80.66, render/thread/ms 80.35, render/busy/ms 78.54, stage/main/update 60.05, stage/render/prepare_assets 55.91, stream/drive/ms 36.90, stage/render/render 14.84, render/graph/ms 12.39, pipelines/driver_ms 10.95, stage/main/post_update 9.92 |
| warm-up | 4.32 | 62.6 | pipelines/compile_ms 1382, pipelines/front_ms 1346, main/busy/ms 115, render/thread/ms 95.62, render/busy/ms 93.55, stage/main/update 93.31, stage/render/prepare_assets 67.67, stream/drive/ms 58.79, pipelines/driver_ms 36.76, stage/render/render 15.95, render/graph/ms 13.08, stage/main/post_update 10.82 |
| warm-up | 4.43 | 108.1 | pipelines/compile_ms 995, pipelines/front_ms 968, main/busy/ms 91.47, render/thread/ms 86.50, render/busy/ms 81.01, stage/main/update 67.44, stage/render/prepare_assets 51.46, stream/drive/ms 46.61, pipelines/driver_ms 26.89, stage/render/render 19.37, stage/render/cleanup 13.24, render/graph/ms 13.12 |
| warm-up | 4.51 | 76.8 | pipelines/compile_ms 879, pipelines/front_ms 847, pipelines/wait_ms 149, render/thread/ms 95.05, render/busy/ms 89.99, stage/render/prepare_assets 61.99, main/busy/ms 56.32, pipelines/driver_ms 31.84, stage/main/update 28.17, stage/render/render 17.56, stream/drive/ms 16.19, stage/main/post_update 15.04 |
| warm-up | 4.62 | 114.5 | pipelines/compile_ms 1771, pipelines/front_ms 1677, pipelines/wait_ms 356, render/thread/ms 146, render/busy/ms 143, pipelines/driver_ms 94.54, stage/render/prepare_assets 76.16, main/busy/ms 64.98, stage/main/update 38.53, stage/render/render 22.96, render/graph/ms 20.46, stage/render/graph_submit 17.95 |
| warm-up | 4.66 | 40.4 | pipelines/compile_ms 1772, pipelines/front_ms 1673, pipelines/wait_ms 287, render/thread/ms 124, render/busy/ms 122, pipelines/driver_ms 98.59, stage/render/prepare_assets 56.33, main/busy/ms 47.74, stage/main/update 26.87, stage/render/render 21.96, render/graph/ms 19.45, stage/render/prepare_flush 17.12 |
| warm-up | 4.69 | 30.9 | pipelines/compile_ms 1091, pipelines/front_ms 1050, pipelines/wait_ms 133, render/thread/ms 55.63, render/busy/ms 54.19, main/busy/ms 44.23, pipelines/driver_ms 41.08, stage/main/update 22.96, stage/render/prepare_assets 20.55, stage/render/render 13.16, stage/main/post_update 12.04, render/graph/ms 11.33 |
| warm-up | 4.73 | 32.2 | pipelines/compile_ms 840, pipelines/front_ms 805, pipelines/wait_ms 77.39, render/thread/ms 54.77, render/busy/ms 53.20, main/busy/ms 49.32, pipelines/driver_ms 34.65, stage/main/update 21.97, stage/main/post_update 20.06, stage/render/prepare_assets 14.03, stage/render/render 13.08, render/graph/ms 11.13 |
| warm-up | 4.76 | 29.5 | pipelines/compile_ms 1003, pipelines/front_ms 951, pipelines/wait_ms 102, render/thread/ms 52.57, pipelines/driver_ms 51.61, render/busy/ms 50.57, main/busy/ms 38.48, stage/main/post_update 23.34, stage/render/prepare_assets 13.54, stage/render/render 12.24, encode//sum 10.27, render/graph/ms 10.06 |
| warm-up | 4.79 | 28.8 | pipelines/compile_ms 396, pipelines/front_ms 373, main/busy/ms 78.41, pipelines/wait_ms 77.91, stage/main/update 59.32, stream/drive/ms 53.11, render/thread/ms 46.66, render/busy/ms 44.66, pipelines/driver_ms 23.23, stage/render/render 15.18, stage/main/post_update 14.21, render/graph/ms 13.02 |
| warm-up | 4.85 | 64.6 | pipelines/compile_ms 564, pipelines/front_ms 548, main/busy/ms 112, stage/main/update 98.72, stream/drive/ms 90.88, render/thread/ms 38.61, render/busy/ms 36.99, stage/render/prepare_assets 17.29, pipelines/driver_ms 15.46, stage/render/render 12.62, render/graph/ms 10.54, stage/main/post_update 7.97 |
| warm-up | 4.91 | 63.4 | pipelines/compile_ms 1175, pipelines/front_ms 1136, main/busy/ms 68.11, stage/main/update 49.87, stream/drive/ms 42.03, render/thread/ms 41.73, render/busy/ms 40.14, pipelines/driver_ms 38.97, stage/render/prepare_assets 22.93, stage/main/post_update 12.29, stage/render/render 9.46, render/graph/ms 7.40 |
| warm-up | 4.94 | 28.7 | pipelines/compile_ms 1250, pipelines/front_ms 1209, render/thread/ms 49.12, render/busy/ms 46.60, main/busy/ms 44.51, pipelines/driver_ms 41.16, stage/main/update 19.69, stage/render/prepare_assets 19.23, stage/render/render 16.55, stage/main/post_update 13.69, render/graph/ms 10.48, stage/render/graph_submit 7.07 |
| warm-up | 5.05 | 81.6 | pipelines/compile_ms 241, pipelines/front_ms 233, main/busy/ms 69.48, render/thread/ms 61.17, render/busy/ms 59.36, stage/main/update 47.54, stage/render/prepare_assets 31.18, stage/render/render 12.03, stage/main/post_update 11.95, stream/drive/ms 11.89, render/graph/ms 9.69, pipelines/driver_ms 7.85 |
| warm-up | 5.08 | 38.3 | pipelines/compile_ms 1474, pipelines/front_ms 1436, render/thread/ms 72.66, render/busy/ms 71.06, main/busy/ms 51.91, stage/render/prepare_assets 39.30, pipelines/driver_ms 37.73, stage/main/update 34.74, stage/render/render 11.50, stage/main/post_update 10, render/graph/ms 9.47, stage/render/specialize 7.99 |
| warm-up | 5.11 | 25.6 | pipelines/compile_ms 1323, pipelines/front_ms 1289, render/thread/ms 50.17, render/busy/ms 48.59, main/busy/ms 41.32, pipelines/driver_ms 33.95, stage/main/update 22.65, stage/render/prepare_assets 17.01, stage/render/render 11.71, stage/main/post_update 11.55, render/graph/ms 9.56, encode//sum 8.00 |
| warm-up | 5.13 | 25.2 | pipelines/compile_ms 483, pipelines/front_ms 468, main/busy/ms 50.45, render/thread/ms 44.15, render/busy/ms 42.19, stage/main/update 33.60, stream/drive/ms 18.05, stage/render/render 15.01, pipelines/driver_ms 14.79, render/graph/ms 12.53, stage/main/post_update 9.85, stage/render/graph_submit 9.71 |
| warm-up | 5.17 | 36.8 | pipelines/compile_ms 643, pipelines/front_ms 622, main/busy/ms 46.20, render/thread/ms 42.52, render/busy/ms 40.78, stage/main/update 30.49, pipelines/driver_ms 20.97, stream/drive/ms 18.15, stage/render/prepare_assets 13.77, stage/render/render 12.79, render/graph/ms 10.59, stage/main/post_update 9.25 |
| warm-up | 5.20 | 27.7 | pipelines/compile_ms 370, pipelines/front_ms 357, render/thread/ms 45.29, render/busy/ms 43.94, main/busy/ms 33.51, stage/main/update 16.41, stage/render/prepare_assets 16.19, pipelines/driver_ms 13.37, stage/main/post_update 11.55, stage/render/render 10.99, render/graph/ms 9.21, encode//sum 6.95 |
| warm-up | 5.25 | 51.8 | pipelines/compile_ms 386, pipelines/front_ms 373, render/thread/ms 71.47, render/busy/ms 69.55, main/busy/ms 50.45, stage/main/post_update 23.59, stage/main/update 20.80, stage/render/render 19.72, render/graph/ms 17.42, stage/render/graph_submit 14.40, stage/render/prepare_resources 13.91, pipelines/driver_ms 12.26 |
| warm-up | 5.30 | 44.6 | pipelines/compile_ms 822, pipelines/front_ms 800, render/thread/ms 87.32, render/busy/ms 85.20, main/busy/ms 58.03, stage/main/post_update 33.31, pipelines/driver_ms 22.02, stage/render/render 19.49, stage/render/prepare_assets 18.47, stage/main/update 18.38, stage/render/prepare_resources 18.11, render/graph/ms 16.98 |
| warm-up | 5.35 | 54.2 | pipelines/compile_ms 1385, pipelines/front_ms 1347, render/thread/ms 77.32, render/busy/ms 75.55, main/busy/ms 47.26, stage/render/prepare_assets 39.89, pipelines/driver_ms 37.96, stage/main/post_update 25.35, stage/main/update 14.42, stage/render/render 11.66, render/graph/ms 9.50, stage/render/prepare_resources 8.07 |
| warm-up | 5.39 | 44.1 | pipelines/compile_ms 1237, pipelines/front_ms 1202, render/thread/ms 71.15, render/busy/ms 69.57, main/busy/ms 49.84, stage/render/prepare_assets 48.60, pipelines/driver_ms 35.24, stage/main/update 26.07, stage/main/post_update 16.14, stage/render/render 11.25, render/graph/ms 9.15, pipelines/wait_ms 7.78 |
| warm-up | 5.43 | 34.4 | pipelines/compile_ms 980, pipelines/front_ms 955, render/thread/ms 60.71, render/busy/ms 57.27, main/busy/ms 49.30, stage/render/prepare_assets 32.09, pipelines/driver_ms 24.45, stage/main/update 23.19, stage/render/render 13.35, stage/main/post_update 12.84, render/graph/ms 9.13, stage/main/pre_update 8.16 |
| warm-up | 5.51 | 26.7 | render/thread/ms 40.49, render/busy/ms 39.02, main/busy/ms 33.97, stage/main/update 18.71, stage/render/prepare_assets 15.16, stage/render/render 12.53, stream/drive/ms 11.17, render/graph/ms 10.50, stage/main/post_update 10.07, stage/render/graph_submit 7.54, encode//sum 7.26, encode/submit 4.49 |
| warm-up | 5.56 | 27.9 | pipelines/compile_ms 1390, pipelines/front_ms 1351, render/thread/ms 41.34, render/busy/ms 39.92, pipelines/driver_ms 39.59, main/busy/ms 29.85, stage/render/prepare_assets 18.00, stage/main/update 12.10, stage/main/post_update 11.73, stage/render/render 10.10, render/graph/ms 8.35, encode//sum 6.42 |
| warm-up | 5.60 | 28.6 | pipelines/compile_ms 191, pipelines/front_ms 187, render/thread/ms 41.42, render/busy/ms 39.67, main/busy/ms 34.09, stage/render/prepare_assets 16.00, stage/main/update 15.34, stage/render/render 12.69, stage/main/post_update 12.28, render/graph/ms 10.24, encode//sum 9.81, stage/render/graph_submit 6.85 |
| warm-up | 5.63 | 27.4 | pipelines/compile_ms 191, pipelines/front_ms 187, render/thread/ms 50.52, render/busy/ms 49.16, main/busy/ms 37.38, stage/render/prepare_assets 27.67, stage/main/update 19.94, stage/render/render 13.03, render/graph/ms 10.86, stage/main/post_update 10.41, encode//sum 8.73, stage/render/graph_submit 7.72 |
| warm-up | 5.68 | 46.1 | pipelines/compile_ms 141, pipelines/front_ms 133, render/thread/ms 64.51, render/busy/ms 63.19, main/busy/ms 43.25, stage/render/prepare_assets 38.86, stage/main/update 26.30, stage/render/render 12.39, render/graph/ms 10.42, stage/main/post_update 9.36, pipelines/driver_ms 8.03, stage/render/graph_submit 7.78 |
| warm-up | 5.70 | 25.2 | pipelines/compile_ms 141, pipelines/front_ms 133, render/thread/ms 60.88, render/busy/ms 59.23, main/busy/ms 45.74, stage/render/prepare_assets 28.02, stage/main/update 24.22, stage/render/render 14.55, stage/main/post_update 13.25, render/graph/ms 12.15, stage/render/graph_submit 8.39, pipelines/driver_ms 8.03 |
| warm-up | 5.74 | 41.8 | pipelines/compile_ms 189, pipelines/front_ms 183, render/thread/ms 59.94, render/busy/ms 58.22, main/busy/ms 37.77, stage/render/prepare_assets 27.59, stage/main/update 14.75, stage/main/post_update 14.58, stage/render/render 14.11, render/graph/ms 11.85, stage/render/graph_submit 7.84, encode//sum 7.82 |
| warm-up | 5.78 | 35.4 | pipelines/compile_ms 189, pipelines/front_ms 183, render/thread/ms 62.94, render/busy/ms 61.47, stage/render/prepare_assets 36.09, main/busy/ms 35.67, stage/main/update 15.28, stage/main/post_update 12.05, stage/render/render 10.12, render/graph/ms 8.16, encode//sum 7.08, pipelines/driver_ms 6.18 |
| warm-up | 5.82 | 42.4 | pipelines/compile_ms 761, pipelines/front_ms 746, render/thread/ms 61.95, render/busy/ms 60.64, stage/render/prepare_assets 40.08, main/busy/ms 32.26, pipelines/driver_ms 15.20, stage/main/update 13.63, stage/main/post_update 11.68, stage/render/render 9.34, encode//sum 7.74, render/graph/ms 7.57 |
| warm-up | 5.87 | 34.0 | pipelines/compile_ms 412, pipelines/front_ms 404, render/thread/ms 47.37, render/busy/ms 46.08, main/busy/ms 29.50, stage/render/prepare_assets 27.98, stage/main/update 12.35, stage/main/post_update 11.41, stage/render/render 10.32, render/graph/ms 8.50, pipelines/driver_ms 8.43, encode//sum 6.64 |
| warm-up | 5.90 | 27.4 | pipelines/compile_ms 72.27, pipelines/front_ms 67.93, render/thread/ms 53.11, render/busy/ms 51.31, main/busy/ms 46.39, stage/main/update 29.31, stage/render/prepare_assets 22.55, stage/render/render 18.55, render/graph/ms 15.93, stage/render/graph_submit 12.47, stage/main/post_update 11.56, encode//sum 8.31 |
| warm-up | 5.94 | 36.5 | pipelines/compile_ms 139, pipelines/front_ms 131, main/busy/ms 45.33, render/thread/ms 43.82, render/busy/ms 42.15, stage/main/update 29.14, stage/render/render 16.02, stage/render/prepare_assets 15.97, render/graph/ms 13.66, stage/render/graph_submit 10.44, stage/main/post_update 9.82, encode//sum 8.41 |
| warm-up | 6.03 | 50.6 | main/busy/ms 49.01, render/thread/ms 43.27, render/busy/ms 41.76, stage/main/update 25.11, stage/render/prepare_assets 19.79, stream/drive/ms 15.91, stage/main/post_update 14.55, pipelines/compile_ms 11.19, pipelines/front_ms 10.54, stage/render/render 10.29, render/graph/ms 8.11, encode//sum 7.77 |
| warm-up | 6.07 | 39.8 | pipelines/compile_ms 614, pipelines/front_ms 593, render/thread/ms 53, render/busy/ms 51.50, main/busy/ms 47.94, stage/render/prepare_assets 27.34, stage/main/update 24.38, pipelines/driver_ms 20.67, stage/main/post_update 15.33, stage/render/render 11.23, render/graph/ms 9.09, stream/drive/ms 8.46 |
| warm-up | 6.10 | 28.5 | pipelines/compile_ms 1044, pipelines/front_ms 1014, render/thread/ms 47.74, main/busy/ms 47.36, render/busy/ms 45.96, pipelines/driver_ms 29.67, stage/main/update 24.90, stage/render/prepare_assets 22.41, stage/main/post_update 15.08, stage/render/render 12.12, render/graph/ms 9.83, stream/drive/ms 7.58 |
| warm-up | 6.13 | 29.3 | pipelines/compile_ms 497, pipelines/front_ms 485, render/thread/ms 43.02, render/busy/ms 41.48, main/busy/ms 35.93, stage/render/prepare_assets 18.56, stage/main/update 14.50, stage/main/post_update 12.90, pipelines/driver_ms 11.69, stage/render/render 10.97, render/graph/ms 8.88, encode//sum 8.23 |
| warm-up | 6.29 | 42.7 | pipelines/compile_ms 671, pipelines/front_ms 646, main/busy/ms 51.99, stage/main/update 33.98, render/thread/ms 33.67, render/busy/ms 31.95, pipelines/driver_ms 24.14, stream/drive/ms 22.48, pipelines/wait_ms 14.63, stage/render/render 13.55, stage/main/post_update 11.42, render/graph/ms 11.33 |
| warm-up | 6.41 | 30.8 | main/busy/ms 40.25, render/thread/ms 33.00, render/busy/ms 31.70, stage/main/update 20.48, stage/main/post_update 13.22, stage/render/render 13.17, stage/render/prepare_assets 11.45, render/graph/ms 11.22, stage/render/graph_submit 8.34, encode//sum 7.54, encode/submit 5.25, stream/drive/ms 3.85 |
| warm-up | 6.46 | 26.7 | main/busy/ms 24.03, render/thread/ms 14.05, render/busy/ms 13.47, stage/render/render 10.46, stage/main/post_update 10.21, stage/main/update 10.01, encode//sum 8.26, stage/render/graph_submit 5.73, stage/render/prepare_assets 4.85, render/graph/ms 4.49, stream/drive/ms 3.29, encode/submit 3.11 |
| warm-up | 6.63 | 27.8 | render/thread/ms 32.04, main/busy/ms 31.91, render/busy/ms 29.17, stage/render/render 17.31, stage/main/update 13.98, stage/main/post_update 13.84, render/graph/ms 13.75, encode//sum 11.77, stage/render/graph_submit 9.20, encode/submit 6.08, stage/render/prepare_assets 5.66, encode/finish_wall 4.31 |
| warm-up | 6.66 | 27.3 | render/thread/ms 44.02, main/busy/ms 43.19, render/busy/ms 41.84, stage/render/render 21.00, stage/main/update 19.91, render/graph/ms 18.11, stage/main/post_update 16.52, stage/render/graph_submit 13.30, stream/drive/ms 12.64, encode//sum 11.07, stage/render/prepare_assets 9.91, encode/submit 8.94 |
| warm-up | 6.70 | 43.2 | render/thread/ms 50.91, render/busy/ms 49.29, main/busy/ms 46.46, stage/render/prepare_assets 25.89, stage/main/update 23.84, stage/main/post_update 16.32, stage/render/render 15.46, stream/drive/ms 13.66, render/graph/ms 13.40, stage/render/graph_submit 9.91, encode//sum 8.37, encode/submit 7.06 |
| warm-up | 7.02 | 32.0 | render/thread/ms 39.25, render/busy/ms 37.61, main/busy/ms 14.83, stage/render/render 11.23, render/graph/ms 9.99, stage/render/specialize 9.94, stage/render/graph_submit 8.14, stage/main/post_update 7.51, stage/render/queue 6.66, encode//sum 6.32, encode/submit 5.18, stage/main/update 4.78 |
| warm-up | 8.23 | 29.3 | render/thread/ms 35.67, render/busy/ms 32.81, main/busy/ms 13.43, stage/render/render 10.14, render/graph/ms 8.90, stage/render/specialize 8.22, stage/main/post_update 7.62, stage/render/graph_submit 6.89, stage/render/queue 5.73, encode//sum 5.52, encode/submit 4.11, stage/render/prepare_meshes 3.95 |
| measure | 0.82 | 25.8 | render/thread/ms 33.02, render/busy/ms 30.22, main/busy/ms 13.11, stage/render/render 9.80, render/graph/ms 8.61, stage/render/specialize 8.25, stage/main/post_update 7.63, encode//sum 7.34, stage/render/graph_submit 6.71, stage/render/queue 5.37, encode/submit 4.10, stage/main/update 3.24 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set shadow.lamp.day_gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=daygate-5 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 5%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 97%) · 2625 MHz (min 2625) · memory 8101 MHz · 97.3 W · 77 °C (max 80) · VRAM 6425 MiB · throttled 100% of samples

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
- `shadow.lamp.day_gate=on (command line: --set)`
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
