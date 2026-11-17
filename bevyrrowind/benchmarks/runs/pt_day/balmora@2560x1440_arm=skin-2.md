# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.skin.skip_unchanged=on (command line: --set); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1547 frames over 15.0 s · **median 9.73 ms (103 fps)** · mean 9.70 · p95 11.06 · p99 11.76 (1% low 85 fps) · worst 26.13 ms · 4 spikes over 1.5× median · 12.0 stutters/min (work 4.0/min)
- warm-up: 279 frames over 8.5 s, worst 2549.3 ms
- hitches (> 25.0 ms): **1** measured, 60 in warm-up
- timed GPU passes: 8.54 ms of a 9.70 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.73 ms, limit 11 ms
- pass worst frame 26.13 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.915 | 2.928 | 2.968 | 1.00 | 0.066 |
| dlss_super_resolution | 2.012 | 2.021 | 2.044 | 1.00 | 0.184 |
| early prepass | 0.602 | 0.605 | 0.616 | 1.00 | 0.092 |
| atmosphere_luts | 0.325 | 0.326 | 0.342 | 1.00 | 0.003 |
| bloom | 0.315 | 0.316 | 0.331 | 1.00 | 0.060 |
| main_transmissive_pass_3d | 0.310 | 0.311 | 0.333 | 1.00 | 0.008 |
| water_velocity | 0.239 | 0.240 | 0.264 | 1.00 | 0.002 |
| water_sspr | 0.207 | 0.207 | 0.210 | 1.00 | 0.002 |
| ssao | 0.190 | 0.191 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.148 | 0.149 | 0.168 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.145 | 0.146 | 0.152 | 1.00 | 0.001 |
| water_fft | 0.133 | 0.133 | 0.137 | 1.00 | 0.003 |
| uniform_allocation | 0.119 | 0.120 | 0.162 | 1.00 | 0.022 |
| upscaling | 0.112 | 0.113 | 0.119 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.089 | 0.089 | 0.121 | 1.00 | 0.015 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.413 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.040 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.028 | 0.222 | 0.229 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.025 | 0.028 | 1.00 | 0.082 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.045 | 0.047 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.024 | 1.00 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.014 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.760 | 10.153 | 24.82 | 13552.4 |
| main/busy/ms | 7.152 | 8.023 | 8.98 | 11064.4 |
| render/busy/ms | 6.903 | 7.934 | 24.18 | 10678.6 |
| stage/render/render | 4.642 | 5.497 | 11.16 | 7180.8 |
| stage/main/post_update | 4.242 | 5.019 | 5.94 | 6563.1 |
| render/graph/ms | 4.067 | 4.868 | 10.01 | 6291.9 |
| encode//sum | 3.850 | 5.179 | 8.29 | 5956.7 |
| stage/render/graph_submit | 3.084 | 3.859 | 8.99 | 4770.5 |
| encode/submit | 1.932 | 2.364 | 8.08 | 2988.2 |
| stage/main/update | 1.801 | 2.199 | 3.44 | 2785.9 |
| stage/render/prepare_views | 1.546 | 3.148 | 4.84 | 2390.9 |
| render/acquire/ms | 1.501 | 3.104 | 4.78 | 2322.6 |
| encode/finish_wall | 1.177 | 1.662 | 2.49 | 1821.4 |
| stage/render/camera_driver | 0.928 | 1.044 | 1.27 | 1435.6 |
| encode/pass:shadow_directional_light/sum | 0.774 | 1.330 | 2.09 | 1196.8 |
| encode/pass:early prepass/sum | 0.729 | 1.004 | 1.49 | 1127.0 |
| encode/pass:main_opaque_pass_3d/sum | 0.708 | 0.996 | 1.32 | 1095.5 |
| stage/render/prepare_write_phase_buffers | 0.543 | 0.747 | 0.98 | 839.3 |
| stage/render/prepare_assets | 0.440 | 0.561 | 1.56 | 680.6 |
| stage/render/prepare_resources | 0.368 | 0.428 | 0.60 | 569.4 |
| bigspace/total/ms | 0.359 | 0.799 | 1.78 | 555.7 |
| render/present/ms | 0.356 | 0.420 | 3.98 | 551.2 |
| stage/main/pre_update | 0.339 | 0.432 | 1.01 | 524.3 |
| stage/main/first | 0.296 | 0.357 | 1.71 | 457.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.235 | 0.335 | 0.50 | 362.8 |
| terrain/update/ms | 0.230 | 0.293 | 0.63 | 355.7 |
| stage/render/prepare_bind_groups | 0.206 | 0.279 | 0.38 | 319.4 |
| terrain/select/ms | 0.200 | 0.255 | 0.60 | 310.1 |
| stage/render/prepare_meshes | 0.199 | 0.243 | 2.17 | 308.3 |
| stream/drive/ms | 0.186 | 0.240 | 0.64 | 288.5 |
| stage/main/fixed_main_loop | 0.180 | 0.334 | 0.91 | 278.3 |
| stage/render/prepare_batch_phases | 0.174 | 0.236 | 0.35 | 269.6 |
| stage/render/extract_commands | 0.168 | 0.198 | 0.47 | 259.2 |
| stage/render/create_views | 0.163 | 0.206 | 0.60 | 252.9 |
| controller/move/ms | 0.163 | 0.291 | 0.66 | 251.9 |
| far/select/ms | 0.161 | 0.203 | 0.58 | 248.5 |
| stream/tick/ms | 0.158 | 0.206 | 0.60 | 245.1 |
| controller/feed/ms | 0.155 | 0.201 | 0.54 | 239.5 |
| stage/render/prepare_flush | 0.153 | 0.274 | 0.69 | 236.4 |
| bigspace/high_precision/ms | 0.131 | 0.345 | 1.32 | 202.3 |
| stage/main/last | 0.116 | 0.152 | 0.77 | 180.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.115 | 0.176 | 0.31 | 178.5 |
| stage/render/queue | 0.106 | 0.135 | 5.65 | 163.7 |
| bigspace/low_precision/ms | 0.087 | 0.195 | 1.16 | 134.9 |
| anim/evaluate/ms | 0.086 | 0.193 | 1.17 | 132.6 |
| stage/render/specialize | 0.082 | 0.101 | 8.30 | 126.5 |
| anim/evaluate_cpu/ms | 0.060 | 0.074 | 0.18 | 93.0 |
| stage/render/prepare_collect_phase_buffers | 0.055 | 0.108 | 0.28 | 85.3 |
| encode/resolve_diagnostics/sum | 0.054 | 0.080 | 0.18 | 83.2 |
| physics/step/ms | 0.027 | 0.055 | 0.15 | 41.7 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.039 | 0.37 | 41.1 |
| weather/map/ms | 0.024 | 0.032 | 0.40 | 37.7 |
| encode/camera_driver_span_end/sum | 0.023 | 0.037 | 0.35 | 36.3 |
| stage/render/cleanup | 0.023 | 0.031 | 0.24 | 35.1 |
| stage/render/phase_sort | 0.019 | 0.034 | 0.12 | 28.8 |
| weather/eval/ms | 0.011 | 0.014 | 0.39 | 16.6 |
| script/update/ms | 0.007 | 0.009 | 0.35 | 10.8 |
| controller/avoid/ms | 0.006 | 0.007 | 0.36 | 9.1 |
| script/input/ms | 0.006 | 0.006 | 0.38 | 8.7 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 6.7 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 5.6 |
| controller/write/ms | 0.002 | 0.002 | 0.40 | 3.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.09 | 1.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 1.1 |
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
| crowd/c1/count | count | 123 | 123 | 122 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1080 |
| crowd/c2/count | count | 283 | 284 | 284 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 113 | 180 | 146 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.54 | 2.16 | 0.64 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.37 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.35 | 0.01 |
| encode/finish_critical | ms | 0.86 | 2.16 | 1.03 |
| encode/pass:early prepass/max | ms | 0.73 | 1.49 | 0.62 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.71 | 1.32 | 0.75 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.31 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.50 | 0.27 |
| encode/pass:shadow_directional_light/max | ms | 0.71 | 2.09 | 1.03 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.18 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.22 | 24.18 | 9.24 |
| gpu/timed/ms | ms | 9.22 | 11.00 | 9.24 |
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
| phases/opaque3d/bins | count | 1307 | 1307 | 1307 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/bins | count | 1307 | 1307 | 1307 |
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
| process/cpu_usage | % | 25.96 | 39.42 | 17.57 |
| process/mem_usage | GiB | 4.66 | 4.75 | 4.75 |
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
| skin/blocks_skipped/count | count | 176 | 178 | 176 |
| skin/blocks_walked/count | count | 5 | 6 | 5 |
| skin/joint_bytes_written/bytes | bytes | 7936 | 12.3 k | 9984 |
| skin/joints_written/count | count | 124 | 192 | 156 |
| skin/upload_writes/count | count | 3 | 5 | 3 |
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
| system/cpu_usage | % | 31.22 | 38.04 | 27.99 |
| system/mem_usage | % | 58.59 | 58.87 | 58.87 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 53 | 82 | 75 |
| views/all/bins | count | 5326 | 8063 | 5326 |
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
| views/main/bins | count | 3174 | 3174 | 3174 |
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
| anim/events/count | count | 0.10 | 2 | 152 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.20 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 29 | 41.1 k |
| encode/bind_walk/init_textures | count | 7166 | 14.0 k | 11.09 M |
| encode/bind_walk/init_textures_skipped | count | 35.6 k | 48.7 k | 55.09 M |
| encode/bind_walk/inits | count | 345 | 437 | 534.3 k |
| encode/bind_walk/inits_skipped | count | 67.00 | 79 | 103.7 k |
| encode/bind_walk/merge_textures | count | 7102 | 13.9 k | 10.99 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.31 M |
| encode/bind_walk/merges | count | 82.26 | 103 | 127.3 k |
| encode/bind_walk/merges_skipped | count | 50.75 | 56 | 78.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1547 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1547 |
| encode/pass:early prepass/count | count | 1 | 1 | 1547 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1547 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1547 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1547 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1547 |
| encode/queue/command_buffers | count | 36.57 | 39 | 56.6 k |
| encode/queue/submits | count | 2 | 2 | 3094 |
| encode/queue/write_buffer/bytes | bytes | 1.14 M | 3.35 M | 1.76 G |
| encode/queue/write_buffer/calls | count | 220 | 290 | 339.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.49 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 13.9 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1547 |
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

## Hitches (> 25.0 ms; slowest 61, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2549.3 | main/busy/ms 2520, stage/main/post_update 2410, stage/main/update 101, surface/advance_cpu_ms 70.39, stage/main/pre_update 4.82, stage/main/last 1.24, surface/cell_upload_ms 0.74, stream/drive/ms 0.61, stage/main/first 0.50, stage/main/fixed_main_loop 0.47, weather/map/ms 0.17, stream/tick/ms 0.16 |
| warm-up | 3.29 | 744.1 | main/busy/ms 2661, stage/main/post_update 2434, render/thread/ms 711, render/busy/ms 543, stage/render/prepare_assets 495, stage/main/update 212, stage/render/prepare_views 164, render/acquire/ms 164, surface/advance_cpu_ms 70.39, stage/render/create_views 28.21, stage/render/render 12.45, render/graph/ms 7.84 |
| warm-up | 3.41 | 120.7 | render/thread/ms 813, render/busy/ms 594, stage/render/prepare_assets 521, stage/render/prepare_views 165, render/acquire/ms 164, main/busy/ms 164, stage/main/update 116, stage/render/render 78.90, pipelines/compile_ms 57.02, render/present/ms 55.05, encode//sum 53.89, pipelines/front_ms 44.07 |
| warm-up | 3.46 | 44.8 | pipelines/compile_ms 428, pipelines/front_ms 357, render/thread/ms 140, pipelines/wait_ms 129, stage/render/render 97.01, render/busy/ms 81.95, pipelines/driver_ms 71.45, main/busy/ms 70.59, render/present/ms 57.29, stage/main/post_update 54.32, encode//sum 51.15, render/graph/ms 38.48 |
| warm-up | 3.58 | 118.3 | pipelines/wait_ms 4562, pipelines/compile_ms 1298, pipelines/front_ms 1177, render/thread/ms 148, render/busy/ms 141, pipelines/driver_ms 122, main/busy/ms 107, stage/main/post_update 73.34, stage/render/prepare_flush 43.34, stage/render/prepare_resources 39.50, stage/render/render 36.07, render/graph/ms 28.45 |
| warm-up | 3.64 | 59.9 | pipelines/wait_ms 5100, pipelines/compile_ms 1581, pipelines/front_ms 1490, render/thread/ms 157, render/busy/ms 155, main/busy/ms 138, pipelines/driver_ms 91.27, stage/main/update 76.29, stage/render/prepare_resources 56.20, stage/main/post_update 43.05, stage/render/prepare_flush 43.02, stage/render/prepare_assets 24.22 |
| warm-up | 3.74 | 104.4 | pipelines/compile_ms 1693, pipelines/front_ms 1626, pipelines/wait_ms 801, render/thread/ms 112, render/busy/ms 110, main/busy/ms 98.38, stage/render/prepare_assets 69.25, pipelines/driver_ms 67.01, stage/main/update 60.04, stage/main/post_update 19.60, stage/render/prepare_resources 18.90, stage/main/fixed_main_loop 14.75 |
| warm-up | 3.80 | 59.0 | pipelines/compile_ms 2092, pipelines/front_ms 1852, pipelines/driver_ms 240, pipelines/wait_ms 231, render/thread/ms 123, render/busy/ms 121, stage/render/prepare_assets 102, main/busy/ms 34.37, stage/main/post_update 17.65, encode//sum 15.46, stage/render/render 12.25, terrain/update/ms 10.71 |
| warm-up | 3.84 | 36.9 | pipelines/compile_ms 1413, pipelines/front_ms 1177, pipelines/driver_ms 236, pipelines/wait_ms 185, render/thread/ms 93.21, render/busy/ms 90.97, stage/render/prepare_assets 47.40, main/busy/ms 33.71, stage/render/render 26.07, render/graph/ms 23.56, stage/render/graph_submit 21.00, stage/main/post_update 20.83 |
| warm-up | 3.89 | 30.9 | pipelines/compile_ms 250, pipelines/front_ms 240, render/thread/ms 46.34, render/busy/ms 44.67, main/busy/ms 28.03, stage/render/prepare_assets 21.24, stage/main/post_update 15.31, stage/render/render 11.73, encode//sum 10.69, render/graph/ms 9.57, pipelines/driver_ms 9.52, terrain/update/ms 9.26 |
| warm-up | 3.92 | 30.4 | pipelines/compile_ms 407, pipelines/front_ms 375, render/thread/ms 57.33, main/busy/ms 55.68, render/busy/ms 55.30, stage/main/update 44.56, stream/drive/ms 40.71, stage/render/prepare_assets 40.07, pipelines/driver_ms 31.37, stage/render/render 10.32, terrain/update/ms 9.48, render/graph/ms 7.83 |
| warm-up | 3.97 | 55.4 | pipelines/compile_ms 1137, pipelines/front_ms 1089, pipelines/wait_ms 84.61, render/thread/ms 60.90, render/busy/ms 59.05, main/busy/ms 53.76, pipelines/driver_ms 47.59, stage/render/prepare_assets 44.82, stage/main/update 44.56, stream/drive/ms 40.68, stage/render/render 10.26, render/graph/ms 7.92 |
| warm-up | 4.01 | 32.9 | pipelines/compile_ms 1083, pipelines/front_ms 1054, pipelines/wait_ms 87.00, render/thread/ms 59.99, render/busy/ms 58.26, main/busy/ms 44.95, stage/main/update 32.65, pipelines/driver_ms 28.58, stream/drive/ms 27.67, stage/render/prepare_assets 25.66, stage/render/render 20.00, render/graph/ms 17.98 |
| warm-up | 4.07 | 60.1 | pipelines/compile_ms 555, pipelines/front_ms 535, render/thread/ms 68.24, render/busy/ms 65.99, main/busy/ms 49.86, stage/main/update 34.67, stage/render/prepare_assets 30.27, stream/drive/ms 28.94, stage/render/render 22.86, render/graph/ms 20.19, pipelines/driver_ms 19.82, stage/render/camera_driver 10.33 |
| warm-up | 4.10 | 32.2 | pipelines/compile_ms 212, pipelines/front_ms 204, render/thread/ms 67.81, render/busy/ms 65.19, stage/render/prepare_assets 45.39, main/busy/ms 43.69, stage/main/update 27.00, stream/drive/ms 20.47, stage/render/render 14.82, render/graph/ms 11.76, stage/main/post_update 9.75, stage/render/graph_submit 8.45 |
| warm-up | 4.18 | 77.6 | pipelines/compile_ms 611, pipelines/front_ms 596, render/thread/ms 69.65, render/busy/ms 67.33, main/busy/ms 49.44, stage/render/prepare_assets 46.51, stage/main/update 31.79, stream/drive/ms 20.61, pipelines/driver_ms 14.64, stage/render/render 14.10, render/graph/ms 11.39, stage/main/post_update 9.82 |
| warm-up | 4.23 | 56.5 | pipelines/compile_ms 1330, pipelines/front_ms 1297, main/busy/ms 106, stage/main/update 87.80, render/thread/ms 75.81, render/busy/ms 73.77, stream/drive/ms 53.99, stage/render/prepare_assets 50.84, pipelines/driver_ms 32.38, stage/render/render 15.56, render/graph/ms 12.57, stage/main/post_update 10.65 |
| warm-up | 4.37 | 136.8 | pipelines/compile_ms 1463, pipelines/front_ms 1424, main/busy/ms 108, render/thread/ms 96.06, render/busy/ms 94.01, stage/main/update 87.01, stage/render/prepare_assets 57.84, stream/drive/ms 54.68, pipelines/driver_ms 39.60, stage/render/render 27.63, render/graph/ms 24.70, stage/render/camera_driver 14.03 |
| warm-up | 4.43 | 61.0 | pipelines/compile_ms 1188, pipelines/front_ms 1149, pipelines/wait_ms 114, render/thread/ms 113, render/busy/ms 112, stage/render/prepare_assets 71.28, main/busy/ms 60.87, pipelines/driver_ms 38.94, stage/main/update 26.82, stage/render/render 23.40, stage/main/post_update 22.96, render/graph/ms 21.44 |
| warm-up | 4.50 | 68.3 | pipelines/compile_ms 545, pipelines/front_ms 523, pipelines/wait_ms 230, render/thread/ms 97.69, render/busy/ms 96.31, main/busy/ms 74.42, stage/render/prepare_assets 61.79, stage/main/update 39.42, stage/main/post_update 26.54, pipelines/driver_ms 22.58, stream/drive/ms 14.29, stage/render/prepare_resources 11.17 |
| warm-up | 4.55 | 52.1 | pipelines/compile_ms 495, pipelines/front_ms 479, pipelines/wait_ms 122, render/thread/ms 84.85, render/busy/ms 83.54, main/busy/ms 55.11, stage/render/prepare_assets 53.70, stage/main/update 34.22, pipelines/driver_ms 15.96, stage/main/post_update 14.22, stage/render/render 9.13, render/graph/ms 7.30 |
| warm-up | 4.58 | 30.4 | pipelines/compile_ms 1906, pipelines/front_ms 1851, render/thread/ms 60.51, render/busy/ms 59.14, pipelines/wait_ms 55.19, pipelines/driver_ms 55.18, stage/render/prepare_assets 38.83, main/busy/ms 35.22, stage/main/update 19.14, stage/render/render 8.84, stage/main/post_update 8.35, render/graph/ms 7.07 |
| warm-up | 4.63 | 49.7 | pipelines/compile_ms 2214, pipelines/front_ms 2150, pipelines/wait_ms 96.60, render/thread/ms 71.22, pipelines/driver_ms 63.92, render/busy/ms 47.20, main/busy/ms 28.79, stage/render/render 25.04, encode//sum 22.04, render/graph/ms 21.80, stage/render/prepare_views 21.40, render/acquire/ms 21.28 |
| warm-up | 4.69 | 42.9 | pipelines/compile_ms 196, pipelines/front_ms 191, main/busy/ms 122, stage/main/update 70.47, stream/drive/ms 57.90, stage/main/post_update 45.73, render/thread/ms 40.92, render/busy/ms 39.26, pipelines/wait_ms 37.29, stage/render/render 16.40, render/graph/ms 14.33, stage/render/prepare_assets 14.02 |
| warm-up | 4.80 | 106.7 | pipelines/compile_ms 1189, pipelines/front_ms 1155, main/busy/ms 158, pipelines/wait_ms 117, stage/main/update 105, stream/drive/ms 98.40, render/thread/ms 52.34, render/busy/ms 50.99, stage/main/post_update 44.64, pipelines/driver_ms 33.41, stage/render/prepare_assets 20.80, stage/render/render 13.67 |
| warm-up | 4.86 | 60.1 | pipelines/compile_ms 2109, pipelines/front_ms 2051, pipelines/wait_ms 94.45, main/busy/ms 78.30, pipelines/driver_ms 58.04, stage/main/update 52.71, stream/drive/ms 45.63, render/thread/ms 42.82, render/busy/ms 41.28, stage/render/prepare_assets 15.76, stage/main/post_update 13.59, stage/render/render 10.44 |
| warm-up | 4.89 | 34.9 | pipelines/compile_ms 1553, pipelines/front_ms 1510, main/busy/ms 76.04, stage/main/update 50.11, pipelines/driver_ms 43.37, render/thread/ms 37.24, render/busy/ms 35.16, pipelines/wait_ms 17.28, stage/main/post_update 13.87, stage/render/prepare_assets 13.77, stage/render/render 13.52, render/graph/ms 10.94 |
| warm-up | 5.00 | 106.3 | pipelines/compile_ms 1113, pipelines/front_ms 1081, pipelines/wait_ms 250, main/busy/ms 93.98, stage/main/update 76.00, render/thread/ms 70.60, render/busy/ms 68.63, stage/render/prepare_assets 37.93, pipelines/driver_ms 31.50, stage/render/render 14.24, stream/drive/ms 13.14, render/graph/ms 11.90 |
| warm-up | 5.04 | 42.7 | pipelines/compile_ms 942, pipelines/front_ms 916, pipelines/wait_ms 248, render/thread/ms 88.93, render/busy/ms 87.46, main/busy/ms 75.43, stage/render/prepare_assets 43.05, stage/main/update 41.67, pipelines/driver_ms 26.62, stage/main/post_update 25.04, stage/render/render 15.85, stream/drive/ms 13.43 |
| warm-up | 5.09 | 47.8 | pipelines/compile_ms 1057, pipelines/front_ms 1029, render/thread/ms 86.09, render/busy/ms 84.73, main/busy/ms 64.90, pipelines/driver_ms 28.17, stage/main/post_update 26.60, stage/main/update 21.74, stage/render/render 15.66, stage/render/prepare_assets 14.95, stage/render/prepare_write_phase_buffers 13.32, stage/main/fixed_main_loop 12.68 |
| warm-up | 5.12 | 26.8 | pipelines/compile_ms 1654, pipelines/front_ms 1614, render/thread/ms 67.96, render/busy/ms 66.59, pipelines/wait_ms 53.50, main/busy/ms 44.80, pipelines/driver_ms 39.34, stage/main/update 19.28, stage/render/prepare_write_phase_buffers 13.55, stage/render/prepare_flush 12.41, stage/render/render 12.27, stage/main/post_update 10.82 |
| warm-up | 5.15 | 35.3 | pipelines/compile_ms 1072, pipelines/front_ms 1043, pipelines/wait_ms 54.51, render/thread/ms 47.14, render/busy/ms 45.70, main/busy/ms 44.97, stage/main/update 29.34, pipelines/driver_ms 28.95, stream/drive/ms 20.76, stage/render/prepare_assets 14.17, stage/render/render 13.82, render/graph/ms 11.76 |
| warm-up | 5.18 | 29.5 | pipelines/compile_ms 356, pipelines/front_ms 344, main/busy/ms 52.33, render/thread/ms 50.05, render/busy/ms 48.32, stage/main/update 30.39, stream/drive/ms 22.13, pipelines/wait_ms 16.28, stage/render/render 14.37, stage/render/prepare_assets 14.36, stage/main/post_update 13.42, pipelines/driver_ms 12.57 |
| warm-up | 5.22 | 38.1 | pipelines/compile_ms 599, pipelines/front_ms 571, render/thread/ms 57.81, render/busy/ms 56.16, main/busy/ms 42.85, pipelines/driver_ms 28.09, stage/render/prepare_assets 24.72, stage/main/update 20.02, stage/render/render 14.78, stage/main/post_update 13.89, render/graph/ms 12.94, pipelines/wait_ms 12.75 |
| warm-up | 5.26 | 40.1 | pipelines/compile_ms 1094, pipelines/front_ms 1049, render/thread/ms 68.78, render/busy/ms 67.30, main/busy/ms 46.38, pipelines/driver_ms 44.71, stage/render/prepare_assets 29.71, stage/main/update 28.48, stage/render/render 16.03, render/graph/ms 14.05, pipelines/wait_ms 11.77, stage/main/post_update 11.61 |
| warm-up | 5.30 | 39.5 | pipelines/compile_ms 1122, pipelines/front_ms 1085, render/thread/ms 67.58, render/busy/ms 65.87, pipelines/wait_ms 58.03, main/busy/ms 49.83, pipelines/driver_ms 37.10, stage/main/update 25.56, stage/render/prepare_assets 22.92, stage/render/render 16.82, stage/main/post_update 15.68, render/graph/ms 14.39 |
| warm-up | 5.34 | 40.2 | pipelines/compile_ms 1415, pipelines/front_ms 1375, pipelines/wait_ms 60.52, render/thread/ms 60.48, render/busy/ms 58.86, pipelines/driver_ms 40.63, main/busy/ms 39.48, stage/render/prepare_assets 28.68, stage/main/update 15.76, stage/main/post_update 15.08, stage/render/render 13.87, render/graph/ms 11.64 |
| warm-up | 5.38 | 38.3 | pipelines/compile_ms 1080, pipelines/front_ms 1048, render/thread/ms 57.26, render/busy/ms 54.70, main/busy/ms 35.45, stage/render/prepare_assets 35.10, pipelines/driver_ms 31.33, pipelines/wait_ms 17.78, stage/main/update 17.73, stage/main/post_update 11.60, stage/render/render 10.96, render/graph/ms 8.00 |
| warm-up | 5.41 | 31.0 | pipelines/compile_ms 414, pipelines/front_ms 400, render/thread/ms 52.20, render/busy/ms 49.75, main/busy/ms 32.84, stage/render/prepare_assets 30.25, stage/main/update 15.19, pipelines/driver_ms 14.01, stage/render/render 11.27, stage/main/post_update 11.23, render/graph/ms 8.28, encode//sum 6.78 |
| warm-up | 5.45 | 25.6 | render/thread/ms 44.28, render/busy/ms 42.48, main/busy/ms 23.89, stage/render/prepare_resources 14.37, stage/render/render 12.68, stage/main/post_update 10.84, render/graph/ms 10.26, stage/main/update 8.69, encode//sum 8.08, stage/render/graph_submit 7.58, stage/render/prepare_assets 5.87, encode/submit 5.40 |
| warm-up | 5.48 | 29.2 | render/thread/ms 52.39, render/busy/ms 50.69, main/busy/ms 36.39, stage/main/update 18.49, stage/render/prepare_resources 18.05, stage/render/render 15.06, stage/main/post_update 13.42, render/graph/ms 12.89, stream/drive/ms 12.67, stage/render/graph_submit 8.77, stage/render/prepare_assets 7.82, encode//sum 7.60 |
| warm-up | 5.52 | 33.9 | render/thread/ms 59.43, render/busy/ms 58.23, main/busy/ms 50.09, stage/main/post_update 24.59, stage/main/update 20.50, stage/render/prepare_resources 16.66, stage/render/prepare_assets 15.25, stage/render/render 13.81, stream/drive/ms 13.25, render/graph/ms 12.07, stage/render/graph_submit 8.16, encode//sum 7.27 |
| warm-up | 5.57 | 31.8 | pipelines/compile_ms 1563, pipelines/front_ms 1520, render/thread/ms 47.69, render/busy/ms 46.43, pipelines/driver_ms 42.92, main/busy/ms 29.03, stage/render/prepare_assets 22.22, stage/main/update 12.20, stage/main/post_update 11.38, stage/render/render 9.46, render/graph/ms 7.86, encode//sum 6.14 |
| warm-up | 5.61 | 26.5 | pipelines/compile_ms 712, pipelines/front_ms 693, render/thread/ms 37.65, render/busy/ms 36.52, main/busy/ms 32.90, pipelines/driver_ms 18.55, stage/render/prepare_assets 15.23, stage/main/update 14.51, stage/main/post_update 11.89, stage/render/render 10.19, render/graph/ms 8.53, encode//sum 8.44 |
| warm-up | 5.68 | 41.8 | pipelines/compile_ms 108, pipelines/front_ms 102, render/thread/ms 57.60, render/busy/ms 56.16, main/busy/ms 37.87, stage/render/prepare_assets 31.18, stage/main/update 20.92, stage/render/render 12.62, render/graph/ms 10.70, stage/main/post_update 10.48, stage/render/graph_submit 8.25, encode//sum 7.37 |
| warm-up | 5.73 | 41.0 | render/thread/ms 51.18, render/busy/ms 49.43, main/busy/ms 26.07, stage/render/prepare_assets 19.70, stage/render/render 16.37, render/graph/ms 14.00, stage/main/update 12.10, stage/render/graph_submit 10.78, pipelines/compile_ms 10.36, pipelines/front_ms 9.76, stage/main/post_update 8.91, encode//sum 8.23 |
| warm-up | 5.76 | 25.9 | render/thread/ms 61.92, render/busy/ms 60.23, main/busy/ms 31.15, stage/render/prepare_assets 29.28, pipelines/compile_ms 26.55, pipelines/front_ms 24.94, stage/render/render 16.07, render/graph/ms 13.61, stage/main/update 13.40, stage/main/post_update 10.99, stage/render/graph_submit 10.29, encode//sum 8.90 |
| warm-up | 5.81 | 50.9 | pipelines/compile_ms 546, pipelines/front_ms 527, render/thread/ms 62.52, render/busy/ms 61.30, stage/render/prepare_assets 38.85, main/busy/ms 36.05, pipelines/driver_ms 18.83, stage/main/update 15.20, stage/main/post_update 12.60, stage/render/render 9.89, encode//sum 8.51, render/graph/ms 8.16 |
| warm-up | 5.86 | 29.5 | pipelines/compile_ms 761, pipelines/front_ms 746, render/thread/ms 41.39, render/busy/ms 40.43, main/busy/ms 28.12, stage/render/prepare_assets 26.54, pipelines/driver_ms 15.75, stage/main/update 12.34, stage/main/post_update 9.85, stage/render/render 7.13, encode//sum 6.06, render/graph/ms 5.79 |
| warm-up | 5.89 | 26.5 | render/thread/ms 47.46, render/busy/ms 45.93, main/busy/ms 45.12, stage/render/prepare_assets 21.34, stage/main/update 20.30, stage/main/post_update 19.92, stage/render/render 14.48, render/graph/ms 12.42, stage/render/graph_submit 8.95, encode//sum 7.95, stream/drive/ms 5.27, encode/finish_wall 4.55 |
| warm-up | 5.92 | 36.0 | pipelines/compile_ms 105, pipelines/front_ms 101, render/thread/ms 44.03, main/busy/ms 43.02, render/busy/ms 42.50, stage/main/post_update 19.11, pipelines/wait_ms 19.08, stage/main/update 18.84, pipelines/lock_ms 18.54, stage/render/prepare_assets 16.93, stage/render/render 14.60, render/graph/ms 12.48 |
| warm-up | 6.02 | 57.8 | pipelines/compile_ms 167, pipelines/front_ms 160, main/busy/ms 48.48, render/thread/ms 44.06, render/busy/ms 42.88, stage/main/update 26.28, stage/render/prepare_assets 18.56, stream/drive/ms 16.30, stage/main/post_update 14.30, stage/render/render 11.18, render/graph/ms 9.30, encode//sum 8.64 |
| warm-up | 6.05 | 35.3 | pipelines/compile_ms 182, pipelines/front_ms 174, render/thread/ms 56.69, render/busy/ms 55.28, main/busy/ms 46.47, stage/main/update 22.91, stage/render/prepare_assets 20.28, stage/render/render 17.20, render/graph/ms 15.00, stage/main/post_update 14.85, stage/render/graph_submit 10.84, encode//sum 9.94 |
| warm-up | 6.09 | 34.0 | pipelines/compile_ms 1140, pipelines/front_ms 1120, render/thread/ms 47.58, render/busy/ms 46.39, main/busy/ms 42.43, stage/main/update 22.38, stage/render/prepare_assets 20.90, pipelines/driver_ms 20.04, stage/render/render 14.02, render/graph/ms 12.31, stage/main/post_update 12.10, stage/render/graph_submit 9.04 |
| warm-up | 6.23 | 32.5 | main/busy/ms 41.33, render/thread/ms 27.53, stage/main/update 26.92, render/busy/ms 26.24, stream/drive/ms 19.48, stage/render/prepare_assets 10.25, stage/main/post_update 9.17, stage/render/render 8.75, render/graph/ms 7.04, encode//sum 6.21, stage/render/graph_submit 4.70, encode/submit 2.85 |
| warm-up | 6.30 | 26.4 | main/busy/ms 33.84, render/thread/ms 28.88, render/busy/ms 27.69, stage/main/update 17.31, stage/render/render 11.42, stage/render/prepare_assets 10.83, stage/main/post_update 10.79, render/graph/ms 9.61, encode//sum 8.03, stage/render/graph_submit 6.76, encode/submit 4.29, stream/drive/ms 3.51 |
| warm-up | 6.50 | 27.6 | render/thread/ms 31.13, main/busy/ms 30.83, render/busy/ms 29.85, stage/render/render 14.81, stage/main/update 13.55, stage/main/post_update 13.45, render/graph/ms 12.87, encode//sum 11.47, stage/render/graph_submit 8.76, stage/render/prepare_assets 6.85, encode/submit 5.04, stage/render/camera_driver 3.91 |
| warm-up | 6.58 | 57.1 | main/busy/ms 47.45, render/thread/ms 46.85, render/busy/ms 45.17, stage/render/prepare_assets 26.05, stage/main/update 25.11, stage/main/post_update 15.31, stream/drive/ms 13.00, stage/render/render 12.42, render/graph/ms 10.14, encode//sum 9.26, stage/render/graph_submit 7.15, encode/submit 4.42 |
| warm-up | 6.92 | 29.9 | render/thread/ms 37.23, render/busy/ms 35.65, main/busy/ms 15.69, stage/render/render 10.94, render/graph/ms 9.54, stage/render/specialize 8.63, stage/main/post_update 8.60, stage/render/graph_submit 7.69, encode//sum 6.08, stage/render/queue 5.66, encode/submit 4.84, stage/render/prepare_meshes 4.52 |
| warm-up | 8.13 | 32.9 | render/thread/ms 39.46, render/busy/ms 37.09, main/busy/ms 14.37, stage/render/specialize 11.88, stage/render/render 10.17, render/graph/ms 8.92, stage/main/post_update 8.27, stage/render/graph_submit 7.00, encode//sum 5.98, stage/render/queue 5.50, stage/render/prepare_meshes 4.46, encode/submit 4.20 |
| measure | 0.83 | 26.1 | render/thread/ms 33.52, render/busy/ms 31.01, main/busy/ms 13.61, stage/render/render 10.22, render/graph/ms 8.97, stage/render/specialize 8.36, encode//sum 8.01, stage/main/post_update 7.55, stage/render/graph_submit 6.99, stage/render/queue 5.75, encode/submit 4.27, stage/main/update 3.82 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set render.skin.skip_unchanged=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=skin-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 11%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 97%) · 2625 MHz (min 2625) · memory 8101 MHz · 98.1 W · 77 °C (max 80) · VRAM 6423 MiB · throttled 100% of samples

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
- `render.skin.skip_unchanged=on (command line: --set)`
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
