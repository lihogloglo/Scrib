# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: clouds,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1460 frames over 15.0 s · **median 10.20 ms (98 fps)** · mean 10.28 · p95 11.33 · p99 11.95 (1% low 84 fps) · worst 24.44 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 0.0/min)
- warm-up: 298 frames over 7.4 s, worst 2497.1 ms
- hitches (> 25.5 ms): **0** measured, 27 in warm-up
- timed GPU passes: 8.47 ms of a 10.28 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.20 ms, limit 11 ms
- pass worst frame 24.44 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.574 | 2.577 | 2.974 | 1.00 | 0.055 |
| dlss_super_resolution | 2.179 | 2.182 | 2.464 | 1.00 | 0.187 |
| early prepass | 0.606 | 0.606 | 0.948 | 1.00 | 0.094 |
| atmosphere_luts | 0.361 | 0.361 | 0.663 | 1.00 | 0.003 |
| bloom | 0.338 | 0.338 | 0.508 | 1.00 | 0.064 |
| main_transmissive_pass_3d | 0.288 | 0.289 | 0.299 | 1.00 | 0.009 |
| water_velocity | 0.259 | 0.259 | 0.273 | 1.00 | 0.002 |
| water_sspr | 0.241 | 0.241 | 0.374 | 1.00 | 0.002 |
| ssao | 0.204 | 0.204 | 0.214 | 1.00 | 0.008 |
| uniform_allocation | 0.180 | 0.180 | 0.183 | 1.00 | 0.031 |
| water_fft | 0.141 | 0.141 | 0.140 | 1.00 | 0.003 |
| atmos_froxel_inject | 0.139 | 0.140 | 0.136 | 1.00 | 0.002 |
| bin_unpacking | 0.139 | 0.139 | 0.144 | 1.00 | 0.023 |
| upscaling | 0.125 | 0.125 | 0.160 | 1.00 | 0.004 |
| clouds_composite | 0.107 | 0.107 | 0.108 | 1.00 | 0.002 |
| tonemapping | 0.095 | 0.095 | 0.096 | 1.00 | 0.003 |
| clustering | 0.068 | 0.068 | 0.079 | 1.00 | 0.022 |
| lightprobe_irradiance_map | 0.055 | 0.440 | 0.758 | 0.13 | 0.001 |
| render_sky | 0.055 | 0.055 | 0.056 | 1.00 | 0.002 |
| clouds_resolve | 0.050 | 0.050 | 0.052 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.043 | 0.043 | 0.042 | 1.00 | 0.006 |
| water_snapshot | 0.038 | 0.038 | 0.055 | 1.00 | 0.002 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.029 | 0.228 | 0.233 | 0.13 | 0.004 |
| clouds_trace | 0.026 | 0.026 | 0.033 | 1.00 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.022 | 0.022 | 0.024 | 1.00 | 0.073 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.029 | 1.00 | 0.007 |
| atmosphere_environment | 0.011 | 0.011 | 0.014 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 1.00 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.009 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.015 | 0.13 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 60.377 | 0.000 | 6182.04 | 88149.9 |
| render/thread/ms | 9.352 | 10.415 | 23.07 | 13653.5 |
| render/busy/ms | 7.200 | 8.509 | 11.21 | 10511.5 |
| main/busy/ms | 6.786 | 7.998 | 9.99 | 9907.8 |
| render/graph/ms | 4.061 | 5.191 | 8.09 | 5928.4 |
| render/acquire/ms | 1.687 | 2.859 | 3.69 | 2463.4 |
| pipelines/compile_ms | 1.535 | 0.000 | 144.58 | 2241.8 |
| pipelines/front_ms | 1.485 | 0.000 | 140.27 | 2168.2 |
| render/present/ms | 0.465 | 0.556 | 13.66 | 678.7 |
| bigspace/total/ms | 0.369 | 0.642 | 1.85 | 539.1 |
| terrain/update/ms | 0.228 | 0.295 | 0.70 | 332.9 |
| terrain/select/ms | 0.198 | 0.256 | 0.66 | 289.0 |
| stream/drive/ms | 0.181 | 0.235 | 0.62 | 263.7 |
| far/select/ms | 0.159 | 0.222 | 0.53 | 231.9 |
| stream/tick/ms | 0.155 | 0.204 | 0.57 | 226.1 |
| controller/feed/ms | 0.143 | 0.198 | 0.63 | 208.1 |
| bigspace/high_precision/ms | 0.133 | 0.295 | 1.67 | 194.6 |
| controller/move/ms | 0.091 | 0.179 | 0.32 | 133.3 |
| bigspace/low_precision/ms | 0.087 | 0.221 | 1.28 | 126.4 |
| anim/evaluate/ms | 0.070 | 0.176 | 1.09 | 102.8 |
| anim/evaluate_cpu/ms | 0.061 | 0.083 | 0.26 | 88.5 |
| pipelines/driver_ms | 0.050 | 0.000 | 5.50 | 73.6 |
| physics/step/ms | 0.031 | 0.057 | 0.79 | 45.6 |
| weather/map/ms | 0.023 | 0.033 | 0.10 | 34.1 |
| physics/insert/ms | 0.011 | 0.053 | 0.64 | 15.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.04 | 15.8 |
| script/update/ms | 0.006 | 0.008 | 0.03 | 9.1 |
| controller/avoid/ms | 0.005 | 0.007 | 0.04 | 7.9 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 6.9 |
| anim/apply/ms | 0.004 | 0.006 | 0.06 | 6.4 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 5.1 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
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
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 4660 | 4212 |
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
| frame/work/ms | ms | 9.06 | 11.21 | 9.54 |
| gpu/timed/ms | ms | 9.05 | 11.12 | 9.54 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 478.47 M | 478.47 M | 478.47 M |
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
| phases/shadow/bins | count | 2029 | 2336 | 2028 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5345 | 5656 | 5345 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 31 | 36 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 728 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 182 | 182 | 182 |
| pipelines/waiting | count | 0 | 36 | 0 |
| preprocess/bind_groups | count | 0 | 195 | 0 |
| preprocess/bind_groups_created | count | 0 | 195 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 201 | 201 | 201 |
| process/cpu_usage | % | 22.01 | 40.38 | 27.75 |
| process/mem_usage | GiB | 4.28 | 4.33 | 4.33 |
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
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 27.24 | 52.35 | 26.77 |
| system/mem_usage | % | 56.06 | 56.22 | 56.22 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2480 | 2480 | 2480 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 89 | 94 | 89 |
| views/all/bins | count | 5199 | 5506 | 5198 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.9 k | 14.6 k |
| views/all/pipelines | count | 85 | 90 | 85 |
| views/all/pipelines_distinct | count | 75 | 75 | 75 |
| views/all/work_items | count | 14.8 k | 15.1 k | 14.8 k |
| views/cascade/batch_sets | count | 31 | 36 | 31 |
| views/cascade/bins | count | 2029 | 2336 | 2028 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5345 | 5656 | 5345 |
| views/cascade/pipelines | count | 31 | 36 | 31 |
| views/cascade/work_items | count | 5345 | 5656 | 5345 |
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
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.09 | 2 | 133 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.39 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.10 | 148 | 150 |
| physics/inserts | count | 0.52 | 125 | 760 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.62 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.5 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2497.1 | main/busy/ms 2472, surface/advance_cpu_ms 68.82, surface/cell_upload_ms 1.02, stream/drive/ms 0.48, weather/map/ms 0.14, stream/tick/ms 0.12, bigspace/total/ms 0.10, bigspace/high_precision/ms 0.07, weather/eval/ms 0.03, script/update/ms 0.03, anim/evaluate/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.03 | 537.4 | main/busy/ms 2514, render/thread/ms 506, render/busy/ms 386, render/acquire/ms 117, surface/advance_cpu_ms 68.82, render/graph/ms 6.00, render/present/ms 2.53, surface/cell_upload_ms 1.02, stream/drive/ms 0.97, stream/tick/ms 0.43, terrain/update/ms 0.34, bigspace/total/ms 0.28 |
| warm-up | 3.24 | 200.6 | pipelines/wait_ms 853, render/thread/ms 516, render/busy/ms 396, pipelines/compile_ms 209, pipelines/front_ms 196, render/acquire/ms 117, main/busy/ms 58.05, terrain/update/ms 23.57, pipelines/driver_ms 13.31, render/graph/ms 9.61, render/present/ms 3.48, stream/drive/ms 1.15 |
| warm-up | 3.49 | 83.6 | pipelines/compile_ms 244, pipelines/front_ms 230, main/busy/ms 70.02, render/thread/ms 55.54, render/busy/ms 53.29, pipelines/driver_ms 13.65, render/graph/ms 4.83, stream/drive/ms 2.17, render/present/ms 1.93, stream/tick/ms 1.70, terrain/update/ms 0.61, terrain/select/ms 0.46 |
| warm-up | 3.63 | 60.7 | main/busy/ms 42.59, render/thread/ms 39.02, render/busy/ms 37.54, stream/drive/ms 32.11, terrain/update/ms 4.90, render/graph/ms 2.87, render/present/ms 1.27, stream/tick/ms 0.81, terrain/select/ms 0.44, stream/spawn/ms 0.21, render/acquire/ms 0.21, bigspace/total/ms 0.20 |
| warm-up | 3.71 | 71.6 | pipelines/wait_ms 3205, pipelines/compile_ms 212, pipelines/front_ms 191, render/thread/ms 52.25, render/busy/ms 50.08, main/busy/ms 38.10, stream/drive/ms 22.06, pipelines/driver_ms 20.68, render/graph/ms 6.00, render/present/ms 1.85, stream/tick/ms 1.17, stream/spawn/ms 1.13 |
| warm-up | 3.83 | 93.7 | pipelines/wait_ms 4848, pipelines/compile_ms 130, pipelines/front_ms 122, render/thread/ms 79.21, render/busy/ms 77.32, main/busy/ms 31.09, stream/drive/ms 10.36, pipelines/driver_ms 8.27, render/graph/ms 4.27, terrain/update/ms 3.24, stream/spawn/ms 1.87, render/present/ms 1.69 |
| warm-up | 3.92 | 71.8 | main/busy/ms 54.71, render/thread/ms 45.71, render/busy/ms 43.78, stream/drive/ms 36.04, render/graph/ms 8.15, stream/spawn/ms 2.91, render/present/ms 1.70, stream/activate/ms 0.90, terrain/update/ms 0.79, stream/tick/ms 0.77, terrain/select/ms 0.61, physics/insert/ms 0.36 |
| warm-up | 3.95 | 27.3 | render/thread/ms 51.73, main/busy/ms 51.45, render/busy/ms 49.36, stream/drive/ms 27.94, render/graph/ms 5.11, stream/spawn/ms 3.31, render/present/ms 2.10, stream/tick/ms 1.80, terrain/update/ms 0.67, stream/activate/ms 0.66, terrain/select/ms 0.53, bigspace/total/ms 0.45 |
| warm-up | 4.02 | 74.4 | pipelines/wait_ms 5044, pipelines/compile_ms 176, pipelines/front_ms 167, render/thread/ms 51.50, main/busy/ms 51.41, render/busy/ms 49.89, stream/drive/ms 27.84, pipelines/driver_ms 9.08, render/graph/ms 5.20, stream/spawn/ms 2.58, stream/tick/ms 2.40, render/present/ms 1.37 |
| warm-up | 4.09 | 57.0 | pipelines/wait_ms 6327, pipelines/compile_ms 95.33, pipelines/front_ms 90.86, main/busy/ms 69.11, stream/drive/ms 29.36, render/thread/ms 23.33, render/busy/ms 21.98, terrain/update/ms 4.74, pipelines/driver_ms 4.47, render/graph/ms 3.36, render/present/ms 1.14, stream/tick/ms 0.98 |
| warm-up | 4.21 | 62.7 | pipelines/wait_ms 2616, pipelines/compile_ms 102, pipelines/front_ms 96.57, main/busy/ms 63.79, stream/drive/ms 52.62, render/thread/ms 24.30, render/busy/ms 23.03, pipelines/driver_ms 5.49, render/graph/ms 3.22, stream/tick/ms 1.13, render/present/ms 1.09, terrain/update/ms 0.60 |
| warm-up | 4.28 | 60.1 | main/busy/ms 53.91, stream/drive/ms 35.04, render/thread/ms 26.28, render/busy/ms 24.94, render/graph/ms 4.55, stream/spawn/ms 2.28, render/present/ms 1.10, stream/tick/ms 0.84, terrain/update/ms 0.65, bigspace/total/ms 0.61, terrain/select/ms 0.55, render/acquire/ms 0.24 |
| warm-up | 4.57 | 41.0 | pipelines/wait_ms 1838, pipelines/compile_ms 177, pipelines/front_ms 172, render/thread/ms 40.61, render/busy/ms 38.95, main/busy/ms 23.94, render/graph/ms 6.56, pipelines/driver_ms 5.39, stream/drive/ms 4.67, stream/spawn/ms 2.84, render/present/ms 1.45, bigspace/total/ms 0.98 |
| warm-up | 4.60 | 28.6 | pipelines/wait_ms 1838, pipelines/compile_ms 177, pipelines/front_ms 172, render/thread/ms 49.74, render/busy/ms 48.65, main/busy/ms 21.77, pipelines/driver_ms 5.39, render/graph/ms 4.22, stream/drive/ms 4.03, stream/spawn/ms 2.36, bigspace/total/ms 0.96, render/present/ms 0.93 |
| warm-up | 4.75 | 29.7 | render/thread/ms 44.70, render/busy/ms 42.92, main/busy/ms 23.01, render/graph/ms 6.43, stream/drive/ms 4.06, stream/spawn/ms 2.27, render/present/ms 1.53, stream/tick/ms 0.82, bigspace/total/ms 0.81, terrain/update/ms 0.62, terrain/select/ms 0.52, bigspace/high_precision/ms 0.31 |
| warm-up | 4.78 | 26.1 | render/thread/ms 47.23, render/busy/ms 45.73, main/busy/ms 21.82, render/graph/ms 5.37, stream/drive/ms 3.92, stream/spawn/ms 2.04, render/present/ms 1.27, stream/tick/ms 0.90, terrain/update/ms 0.86, terrain/select/ms 0.72, bigspace/total/ms 0.71, bigspace/high_precision/ms 0.27 |
| warm-up | 4.81 | 31.9 | render/thread/ms 43.77, render/busy/ms 42.56, main/busy/ms 22.12, render/graph/ms 4.56, stream/drive/ms 4.47, stream/spawn/ms 2.15, render/present/ms 1.05, stream/tick/ms 0.82, terrain/update/ms 0.81, bigspace/total/ms 0.79, terrain/select/ms 0.67, bigspace/high_precision/ms 0.51 |
| warm-up | 4.89 | 28.8 | render/thread/ms 35.49, render/busy/ms 33.39, main/busy/ms 31.45, stream/drive/ms 8.04, render/graph/ms 7.57, stream/spawn/ms 3.63, render/present/ms 1.77, stream/tick/ms 1.51, bigspace/total/ms 0.83, terrain/update/ms 0.77, terrain/select/ms 0.65, bigspace/high_precision/ms 0.38 |
| warm-up | 4.93 | 39.2 | pipelines/wait_ms 1565, pipelines/compile_ms 75.34, pipelines/front_ms 72.51, render/thread/ms 41.75, render/busy/ms 39.97, main/busy/ms 29.80, render/graph/ms 5.42, stream/drive/ms 5.33, pipelines/driver_ms 2.83, stream/spawn/ms 2.80, stream/tick/ms 1.76, render/present/ms 1.51 |
| warm-up | 5.09 | 28.6 | pipelines/wait_ms 1791, main/busy/ms 32.97, render/thread/ms 21.92, render/busy/ms 20.58, stream/drive/ms 16.78, pipelines/compile_ms 14.44, pipelines/front_ms 13.81, render/graph/ms 6.15, stream/spawn/ms 2.26, render/present/ms 1.12, bigspace/total/ms 0.80, stream/tick/ms 0.77 |
| warm-up | 5.39 | 43.0 | render/thread/ms 37.28, main/busy/ms 36.38, render/busy/ms 35.48, stream/drive/ms 10.90, render/graph/ms 5.56, stream/spawn/ms 1.89, render/present/ms 1.44, stream/tick/ms 1.30, terrain/update/ms 0.80, bigspace/total/ms 0.74, terrain/select/ms 0.65, render/acquire/ms 0.35 |
| warm-up | 6.08 | 36.2 | pipelines/wait_ms 4907, pipelines/compile_ms 167, pipelines/front_ms 158, main/busy/ms 55.31, render/thread/ms 27.83, render/busy/ms 26.17, render/graph/ms 9.27, pipelines/driver_ms 8.84, render/present/ms 1.48, stream/drive/ms 1.29, bigspace/total/ms 0.71, anim/evaluate_cpu/ms 0.59 |
| warm-up | 6.14 | 58.1 | pipelines/wait_ms 7742, pipelines/compile_ms 212, pipelines/front_ms 201, main/busy/ms 63.27, render/thread/ms 49.52, render/busy/ms 47.89, pipelines/driver_ms 11.15, render/graph/ms 8.63, render/present/ms 1.38, stream/drive/ms 1.15, bigspace/total/ms 1.02, anim/evaluate_cpu/ms 0.69 |
| warm-up | 6.18 | 38.0 | pipelines/wait_ms 12.8 k, pipelines/compile_ms 96.87, pipelines/front_ms 91.50, render/thread/ms 54.56, render/busy/ms 53.29, main/busy/ms 44.39, render/graph/ms 7.49, pipelines/driver_ms 5.37, stream/drive/ms 1.21, bigspace/total/ms 1.04, render/present/ms 1.02, stream/tick/ms 0.55 |
| warm-up | 6.37 | 27.8 | pipelines/wait_ms 2505, main/busy/ms 37.17, render/thread/ms 31.05, render/busy/ms 29.52, pipelines/compile_ms 21.59, pipelines/front_ms 20.22, render/graph/ms 8.51, stream/drive/ms 1.91, pipelines/driver_ms 1.37, bigspace/total/ms 1.31, render/present/ms 1.26, stream/tick/ms 1.03 |
| warm-up | 6.40 | 28.4 | pipelines/wait_ms 7676, pipelines/compile_ms 55.94, pipelines/front_ms 53.47, main/busy/ms 42.87, render/thread/ms 39.44, render/busy/ms 38.19, render/graph/ms 7.66, pipelines/driver_ms 2.47, stream/drive/ms 1.92, physics/insert/ms 1.17, stream/tick/ms 1.15, render/present/ms 0.98 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_clouds\balmora@2560x1440_off=normalmaps_orm_parallax_clouds-2 --bench-baseline none --off normalmaps,orm,parallax,clouds
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 12%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **98%** (min 91%) · 2578 MHz (min 2565) · memory 8001 MHz · 95.0 W · 78 °C (max 80) · VRAM 5700 MiB · throttled 100% of samples
