# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 957 frames over 15.0 s · **median 15.55 ms (64 fps)** · mean 15.69 · p95 16.82 · p99 18.20 (1% low 55 fps) · worst 84.61 ms · 3 spikes over 1.5× median · 12.0 stutters/min (work 12.0/min)
- warm-up: 455 frames over 9.2 s, worst 2636.3 ms
- hitches (> 38.9 ms): **2** measured, 20 in warm-up
- timed GPU passes: 13.61 ms of a 15.69 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 15.55 ms, limit 11 ms
- **FAIL** worst frame 84.61 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 6.725 | 6.753 | 7.005 | 1.00 | 0.055 |
| early prepass | 0.971 | 0.975 | 1.316 | 1.00 | 0.094 |
| ssao | 0.789 | 0.792 | 1.094 | 1.00 | 0.006 |
| main_transmissive_pass_3d | 0.724 | 0.728 | 1.096 | 1.00 | 0.006 |
| clouds_composite | 0.495 | 0.497 | 0.835 | 1.00 | 0.001 |
| clouds_trace | 0.446 | 0.448 | 0.505 | 1.00 | 0.001 |
| taa | 0.408 | 0.410 | 0.600 | 1.00 | 0.002 |
| atmosphere_luts | 0.355 | 0.357 | 0.635 | 1.00 | 0.004 |
| bloom | 0.330 | 0.332 | 0.340 | 1.00 | 0.059 |
| water_sspr | 0.300 | 0.301 | 0.292 | 1.00 | 0.002 |
| water_velocity | 0.273 | 0.274 | 0.497 | 1.00 | 0.001 |
| render_sky | 0.209 | 0.210 | 0.205 | 1.00 | 0.001 |
| clouds_resolve | 0.196 | 0.197 | 0.205 | 1.00 | 0.001 |
| water_snapshot | 0.186 | 0.186 | 0.226 | 1.00 | 0.001 |
| uniform_allocation | 0.179 | 0.180 | 0.184 | 1.00 | 0.029 |
| atmos_froxel_inject | 0.155 | 0.156 | 0.155 | 1.00 | 0.001 |
| water_fft | 0.141 | 0.142 | 0.143 | 1.00 | 0.002 |
| bin_unpacking | 0.137 | 0.138 | 0.142 | 1.00 | 0.021 |
| upscaling | 0.128 | 0.128 | 0.158 | 1.00 | 0.004 |
| tonemapping | 0.092 | 0.092 | 0.095 | 1.00 | 0.003 |
| clustering | 0.066 | 0.066 | 0.082 | 1.00 | 0.020 |
| lightprobe_irradiance_map | 0.053 | 0.427 | 0.428 | 0.12 | 0.001 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.043 | 1.00 | 0.005 |
| atmos_froxel_integrate | 0.033 | 0.033 | 0.033 | 1.00 | 0.001 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.030 | 0.242 | 0.377 | 0.12 | 0.003 |
| main_transparent_pass_3d | 0.024 | 0.024 | 0.028 | 1.00 | 0.069 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.023 | 0.030 | 1.00 | 0.006 |
| atmosphere_environment | 0.010 | 0.011 | 0.014 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.011 | 0.011 | 1.00 | 0.002 |
| night_sky | 0.010 | 0.010 | 0.013 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.016 | 0.018 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 14.932 | 0.000 | 6308.88 | 14289.9 |
| render/thread/ms | 14.708 | 15.874 | 83.63 | 14075.2 |
| render/busy/ms | 7.368 | 8.504 | 77.36 | 7051.5 |
| render/acquire/ms | 6.850 | 8.066 | 24.80 | 6555.5 |
| main/busy/ms | 6.123 | 6.636 | 63.39 | 5860.2 |
| render/graph/ms | 3.933 | 5.026 | 7.22 | 3764.2 |
| pipelines/compile_ms | 0.501 | 0.000 | 132.62 | 479.8 |
| render/present/ms | 0.489 | 0.570 | 3.15 | 468.3 |
| pipelines/front_ms | 0.487 | 0.000 | 128.96 | 466.2 |
| bigspace/total/ms | 0.277 | 0.351 | 1.49 | 264.9 |
| terrain/update/ms | 0.223 | 0.266 | 0.49 | 212.9 |
| terrain/select/ms | 0.193 | 0.229 | 0.46 | 185.0 |
| stream/drive/ms | 0.178 | 0.212 | 0.46 | 170.2 |
| far/select/ms | 0.156 | 0.190 | 0.39 | 149.3 |
| stream/tick/ms | 0.151 | 0.182 | 0.43 | 144.9 |
| controller/feed/ms | 0.145 | 0.192 | 0.37 | 138.5 |
| controller/move/ms | 0.141 | 0.263 | 0.51 | 134.6 |
| bigspace/high_precision/ms | 0.092 | 0.114 | 0.31 | 88.0 |
| anim/evaluate_cpu/ms | 0.063 | 0.077 | 0.31 | 60.7 |
| bigspace/low_precision/ms | 0.061 | 0.078 | 0.81 | 58.4 |
| anim/evaluate/ms | 0.052 | 0.067 | 0.29 | 49.6 |
| physics/step/ms | 0.047 | 0.065 | 0.32 | 44.9 |
| weather/map/ms | 0.024 | 0.033 | 0.09 | 22.8 |
| pipelines/driver_ms | 0.014 | 0.000 | 3.67 | 13.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.15 | 10.5 |
| script/update/ms | 0.007 | 0.008 | 0.03 | 6.4 |
| controller/avoid/ms | 0.006 | 0.007 | 0.03 | 5.4 |
| script/input/ms | 0.005 | 0.006 | 0.19 | 4.8 |
| anim/apply/ms | 0.004 | 0.005 | 0.02 | 3.8 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 3.3 |
| physics/insert/ms | 0.002 | 0.000 | 0.59 | 1.9 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 1.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 0.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.3 |
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
| anim/a2/count | count | 0 | 5 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 21 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 11 | 8 |
| crowd/c0/joints/count | count | 272 | 374 | 272 |
| crowd/c0/meshes/count | count | 48 | 66 | 48 |
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 283 | 283 |
| crowd/c2/joints/count | count | 4916 | 4916 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1373 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 7362 | 4212 |
| crowd/joints_uploaded/count | count | 4212 | 4212 | 4212 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.0 k | 47.0 k | 47.0 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.09 M | 189.09 M | 189.09 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 14.27 | 77.36 | 13.67 |
| gpu/timed/ms | ms | 14.27 | 17.41 | 13.67 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 486.97 M | 486.97 M | 486.97 M |
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
| phases/opaque3d/batch_sets | count | 22 | 22 | 22 |
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 21 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 22 | 22 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 21 | 21 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 31 | 36 | 31 |
| phases/shadow/bins | count | 2028 | 2336 | 2029 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5345 | 5672 | 5346 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 31 | 36 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 66 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 184 | 184 | 184 |
| pipelines/waiting | count | 0 | 6 | 0 |
| preprocess/bind_groups | count | 0 | 56 | 0 |
| preprocess/bind_groups_created | count | 0 | 56 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 201 | 201 | 201 |
| process/cpu_usage | % | 21.88 | 38.46 | 19.20 |
| process/mem_usage | GiB | 4.17 | 4.21 | 4.21 |
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
| started_load_count |  | 3547 | 3549 | 3549 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 386 | 386 | 386 |
| statics/materials_bound | count | 386 | 386 | 386 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.03 | 0.03 | 0.03 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 26.09 | 41.71 | 31.02 |
| system/mem_usage | % | 72.92 | 73.14 | 73.07 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2480 | 2480 | 2480 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 89 | 94 | 89 |
| views/all/bins | count | 5198 | 5506 | 5199 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 15.0 k | 14.6 k |
| views/all/pipelines | count | 85 | 90 | 85 |
| views/all/pipelines_distinct | count | 75 | 75 | 75 |
| views/all/work_items | count | 14.8 k | 15.1 k | 14.8 k |
| views/cascade/batch_sets | count | 31 | 36 | 31 |
| views/cascade/bins | count | 2028 | 2336 | 2029 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5345 | 5672 | 5346 |
| views/cascade/pipelines | count | 31 | 36 | 31 |
| views/cascade/work_items | count | 5345 | 5672 | 5346 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 58 | 58 | 58 |
| views/main/bins | count | 3170 | 3170 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 54 | 54 | 54 |
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
| world/meshes | count | 24.6 k | 24.6 k | 24.6 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0.10 | 99 | 99 |
| anim/events/count | count | 0.16 | 3 | 152 |
| assets/Image/added | count | 0.02 | 12 | 22 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.08 | 40 | 79 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.03 | 13 | 26 |
| assets/StandardMaterial/modified | count | 5.16 | 11 | 4939 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.90 | 684 | 861 |
| physics/inserts | count | 0.07 | 22 | 70 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.94 | 3 | 898 |
| physics/steps_dropped | count | 0.00 | 2 | 3 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 2 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 38.9 ms; slowest 22, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.64 | 2636.3 | main/busy/ms 2611, surface/advance_cpu_ms 69.27, surface/cell_upload_ms 0.75, stream/drive/ms 0.27, weather/map/ms 0.15, bigspace/total/ms 0.09, stream/tick/ms 0.06, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.25 | 617.2 | main/busy/ms 2647, render/thread/ms 580, render/busy/ms 555, surface/advance_cpu_ms 69.27, render/acquire/ms 20.87, render/graph/ms 8.60, render/present/ms 3.75, stream/drive/ms 1.24, surface/cell_upload_ms 0.75, stream/tick/ms 0.73, terrain/update/ms 0.56, terrain/select/ms 0.37 |
| warm-up | 3.48 | 223.9 | pipelines/wait_ms 680, render/thread/ms 615, render/busy/ms 589, pipelines/compile_ms 219, pipelines/front_ms 206, main/busy/ms 55.58, render/graph/ms 35.00, render/acquire/ms 21.09, terrain/update/ms 18.97, pipelines/driver_ms 12.64, render/present/ms 5.04, stream/drive/ms 1.55 |
| warm-up | 3.70 | 103.9 | pipelines/compile_ms 195, pipelines/front_ms 180, main/busy/ms 74.76, render/thread/ms 56.07, render/busy/ms 54.32, pipelines/wait_ms 42.65, pipelines/driver_ms 14.82, render/graph/ms 3.25, stream/drive/ms 1.77, render/present/ms 1.55, stream/tick/ms 1.30, terrain/update/ms 0.59 |
| warm-up | 3.91 | 58.5 | pipelines/wait_ms 1017, pipelines/compile_ms 207, pipelines/front_ms 193, render/thread/ms 44.99, render/busy/ms 42.69, main/busy/ms 39.69, stream/drive/ms 25.50, pipelines/driver_ms 13.52, render/graph/ms 3.85, render/present/ms 2.03, stream/spawn/ms 1.73, terrain/update/ms 0.85 |
| warm-up | 4.02 | 75.3 | pipelines/wait_ms 823, render/thread/ms 78.03, render/busy/ms 74.52, pipelines/compile_ms 70.97, pipelines/front_ms 65.34, main/busy/ms 35.76, render/graph/ms 7.70, terrain/update/ms 6.62, pipelines/driver_ms 5.64, stream/drive/ms 5.42, render/present/ms 3.15, stream/spawn/ms 1.76 |
| warm-up | 4.18 | 126.9 | pipelines/wait_ms 5243, pipelines/compile_ms 265, pipelines/front_ms 208, render/thread/ms 94.78, render/busy/ms 93.11, pipelines/driver_ms 56.80, main/busy/ms 51.83, stream/drive/ms 36.68, render/graph/ms 4.42, render/present/ms 1.49, stream/spawn/ms 1.21, stream/tick/ms 0.99 |
| warm-up | 4.25 | 57.5 | main/busy/ms 47.32, render/thread/ms 34.56, render/busy/ms 33.02, stream/drive/ms 20.03, render/graph/ms 2.63, stream/spawn/ms 1.90, render/present/ms 1.32, physics/insert/ms 0.66, terrain/update/ms 0.62, stream/activate/ms 0.61, stream/tick/ms 0.59, terrain/select/ms 0.49 |
| warm-up | 4.40 | 55.3 | pipelines/wait_ms 5661, main/busy/ms 63.38, stream/drive/ms 53.04, pipelines/compile_ms 45.44, pipelines/front_ms 42.49, render/thread/ms 19.37, render/busy/ms 17.83, render/graph/ms 3.76, pipelines/driver_ms 2.95, render/present/ms 1.37, stream/tick/ms 0.94, terrain/update/ms 0.70 |
| warm-up | 4.47 | 52.3 | main/busy/ms 52.39, stream/drive/ms 35.94, render/thread/ms 19.31, render/busy/ms 18.20, render/graph/ms 2.39, stream/spawn/ms 2.16, render/present/ms 0.95, terrain/update/ms 0.78, terrain/select/ms 0.61, stream/tick/ms 0.60, bigspace/total/ms 0.34, stream/activate/ms 0.31 |
| warm-up | 5.10 | 40.6 | pipelines/wait_ms 8912, pipelines/compile_ms 88.39, pipelines/front_ms 82.78, render/thread/ms 50.13, render/busy/ms 48.24, main/busy/ms 28.18, pipelines/driver_ms 5.62, render/graph/ms 5.41, stream/drive/ms 3.32, stream/spawn/ms 2.09, render/present/ms 1.60, bigspace/total/ms 0.89 |
| warm-up | 5.16 | 39.8 | pipelines/wait_ms 1587, render/thread/ms 38.66, main/busy/ms 37.93, render/busy/ms 37.38, pipelines/compile_ms 28.50, pipelines/front_ms 27.28, stream/drive/ms 16.78, render/graph/ms 3.64, stream/spawn/ms 2.62, pipelines/driver_ms 1.22, render/present/ms 1.13, stream/tick/ms 1.02 |
| warm-up | 5.59 | 39.6 | pipelines/wait_ms 1506, pipelines/compile_ms 148, pipelines/front_ms 144, render/thread/ms 38.83, render/busy/ms 37.09, main/busy/ms 29.84, stream/drive/ms 9.24, render/graph/ms 5.08, pipelines/driver_ms 3.50, stream/spawn/ms 1.43, render/present/ms 1.38, stream/tick/ms 0.89 |
| warm-up | 7.94 | 63.2 | render/thread/ms 68.07, render/acquire/ms 55.18, render/busy/ms 11.89, main/busy/ms 10.54, render/graph/ms 5.56, render/present/ms 1.01, terrain/update/ms 0.45, bigspace/total/ms 0.39, terrain/select/ms 0.38, physics/insert/ms 0.37, stream/drive/ms 0.33, far/select/ms 0.32 |
| warm-up | 7.99 | 57.5 | pipelines/wait_ms 2465, render/thread/ms 119, render/acquire/ms 105, pipelines/compile_ms 100, pipelines/front_ms 96.95, render/busy/ms 13.03, main/busy/ms 10.92, render/graph/ms 6.36, pipelines/driver_ms 3.15, render/present/ms 1.13, bigspace/total/ms 0.46, terrain/update/ms 0.43 |
| warm-up | 8.06 | 64.0 | pipelines/wait_ms 10.7 k, pipelines/compile_ms 198, pipelines/front_ms 191, render/thread/ms 120, render/acquire/ms 107, render/busy/ms 12.17, main/busy/ms 11.31, pipelines/driver_ms 7.29, render/graph/ms 5.67, render/present/ms 0.95, bigspace/total/ms 0.47, terrain/update/ms 0.44 |
| warm-up | 8.12 | 65.4 | pipelines/wait_ms 21.3 k, pipelines/compile_ms 144, pipelines/front_ms 138, render/thread/ms 127, render/acquire/ms 114, render/busy/ms 12.53, main/busy/ms 11.41, pipelines/driver_ms 6.39, render/graph/ms 5.82, render/present/ms 1.13, bigspace/total/ms 0.44, terrain/update/ms 0.43 |
| warm-up | 8.19 | 65.7 | pipelines/wait_ms 13.0 k, render/thread/ms 129, render/acquire/ms 114, pipelines/compile_ms 46.09, pipelines/front_ms 43.84, render/busy/ms 13.07, main/busy/ms 11.35, render/graph/ms 6.21, pipelines/driver_ms 2.25, render/present/ms 1.27, bigspace/total/ms 0.44, terrain/update/ms 0.43 |
| warm-up | 8.26 | 68.8 | pipelines/wait_ms 18.1 k, render/thread/ms 132, render/acquire/ms 116, pipelines/compile_ms 115, pipelines/front_ms 109, render/busy/ms 15.66, main/busy/ms 11.41, render/graph/ms 8.60, pipelines/driver_ms 5.34, render/present/ms 1.16, stream/drive/ms 0.50, stream/tick/ms 0.44 |
| warm-up | 8.32 | 66.2 | pipelines/wait_ms 20.2 k, pipelines/compile_ms 217, pipelines/front_ms 210, render/thread/ms 133, render/acquire/ms 116, main/busy/ms 23.42, render/busy/ms 15.57, render/graph/ms 8.38, pipelines/driver_ms 7.39, render/present/ms 0.97, stream/drive/ms 0.51, physics/insert/ms 0.51 |
| measure | 2.49 | 84.6 | render/thread/ms 99.16, render/busy/ms 85.93, main/busy/ms 12.53, render/acquire/ms 12.25, render/graph/ms 11.05, render/present/ms 0.98, bigspace/total/ms 0.58, terrain/update/ms 0.49, terrain/select/ms 0.43, stream/drive/ms 0.34, far/select/ms 0.33, controller/feed/ms 0.32 |
| measure | 3.60 | 58.8 | main/busy/ms 69.53, render/thread/ms 23.30, render/busy/ms 15.36, render/graph/ms 8.82, render/acquire/ms 6.97, render/present/ms 0.97, bigspace/total/ms 0.47, terrain/update/ms 0.46, terrain/select/ms 0.40, far/select/ms 0.39, stream/drive/ms 0.35, stream/tick/ms 0.30 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\claude\3866c27c11752a1d\4ff90442-bcd4-4820-b03c-4dd8a81992a4\scratchpad\bin\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\tc_pbr\balmora@2560x1440_off=normalmaps_orm_parallax-1 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 13%
- during the run: other processes 4% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2577 MHz (min 2565) · memory 8001 MHz · 99.9 W · 82 °C (max 84) · VRAM 4852 MiB · throttled 100% of samples
