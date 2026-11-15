# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: waterssr,shorewaves,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1102 frames over 15.0 s · **median 14.58 ms (69 fps)** · mean 13.61 · p95 16.00 · p99 17.19 (1% low 58 fps) · worst 24.55 ms · 1 spikes over 1.5× median · 100.0 stutters/min (work 124.0/min)
- warm-up: 290 frames over 7.0 s, worst 2465.8 ms
- hitches (> 36.5 ms): **0** measured, 20 in warm-up
- timed GPU passes: 11.25 ms of a 13.61 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 14.58 ms, limit 11 ms
- pass worst frame 24.55 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 5.664 | 5.669 | 7.013 | 1.00 | 0.058 |
| early prepass | 0.825 | 0.826 | 1.295 | 1.00 | 0.088 |
| ssao | 0.649 | 0.805 | 1.091 | 0.81 | 0.007 |
| clouds_trace | 0.452 | 0.452 | 0.505 | 1.00 | 0.001 |
| clouds_composite | 0.452 | 0.491 | 0.814 | 0.92 | 0.001 |
| main_transmissive_pass_3d | 0.404 | 0.404 | 0.473 | 1.00 | 0.007 |
| taa | 0.401 | 0.401 | 0.725 | 1.00 | 0.002 |
| bloom | 0.299 | 0.330 | 0.340 | 0.90 | 0.060 |
| atmosphere_luts | 0.292 | 0.357 | 0.613 | 0.82 | 0.003 |
| render_sky | 0.214 | 0.214 | 0.207 | 1.00 | 0.002 |
| clouds_resolve | 0.193 | 0.193 | 0.204 | 1.00 | 0.001 |
| water_snapshot | 0.180 | 0.180 | 0.229 | 1.00 | 0.002 |
| uniform_allocation | 0.155 | 0.181 | 0.182 | 0.86 | 0.032 |
| atmos_froxel_inject | 0.139 | 0.145 | 0.153 | 0.96 | 0.001 |
| bin_unpacking | 0.132 | 0.132 | 0.142 | 1.00 | 0.022 |
| water_fft | 0.122 | 0.137 | 0.139 | 0.89 | 0.002 |
| water_velocity | 0.121 | 0.121 | 0.126 | 1.00 | 0.001 |
| upscaling | 0.121 | 0.121 | 0.126 | 1.00 | 0.004 |
| tonemapping | 0.075 | 0.092 | 0.095 | 0.82 | 0.003 |
| clustering | 0.064 | 0.065 | 0.073 | 0.98 | 0.022 |
| lightprobe_irradiance_map | 0.050 | 0.423 | 0.419 | 0.12 | 0.001 |
| early_mesh_preprocessing | 0.038 | 0.039 | 0.042 | 1.00 | 0.006 |
| atmos_froxel_integrate | 0.030 | 0.031 | 0.034 | 0.96 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.224 | 0.233 | 0.12 | 0.003 |
| clouds_shadow | 0.026 | 0.031 | 0.033 | 0.86 | 0.003 |
| night_sky | 0.023 | 0.023 | 0.090 | 1.00 | 0.001 |
| water_mips | 0.021 | 0.023 | 0.024 | 0.89 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.024 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.020 | 0.020 | 0.025 | 1.00 | 0.067 |
| atmosphere_environment | 0.011 | 0.011 | 0.012 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.95 | 0.002 |
| sparse buffer updates | 0.008 | 0.008 | 0.008 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.13 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.003 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 218.006 | 0.000 | 10210.97 | 240242.9 |
| render/thread/ms | 12.674 | 15.112 | 22.87 | 13966.3 |
| render/busy/ms | 6.908 | 8.006 | 12.50 | 7612.6 |
| main/busy/ms | 6.172 | 6.910 | 12.95 | 6801.0 |
| render/acquire/ms | 5.267 | 7.402 | 9.64 | 5804.8 |
| render/graph/ms | 3.615 | 4.542 | 8.74 | 3984.2 |
| pipelines/compile_ms | 2.199 | 0.000 | 159.35 | 2423.6 |
| pipelines/front_ms | 2.115 | 0.000 | 147.41 | 2330.8 |
| render/present/ms | 0.498 | 0.581 | 15.84 | 548.9 |
| bigspace/total/ms | 0.291 | 0.447 | 1.06 | 320.5 |
| terrain/update/ms | 0.221 | 0.266 | 0.55 | 244.0 |
| terrain/select/ms | 0.193 | 0.235 | 0.52 | 212.2 |
| stream/drive/ms | 0.182 | 0.255 | 0.64 | 200.5 |
| stream/tick/ms | 0.154 | 0.219 | 0.45 | 170.0 |
| far/select/ms | 0.152 | 0.176 | 0.47 | 167.6 |
| controller/feed/ms | 0.150 | 0.187 | 0.33 | 165.3 |
| controller/move/ms | 0.106 | 0.213 | 0.41 | 117.2 |
| bigspace/high_precision/ms | 0.102 | 0.147 | 0.69 | 112.4 |
| pipelines/driver_ms | 0.084 | 0.000 | 11.94 | 92.8 |
| bigspace/low_precision/ms | 0.060 | 0.083 | 0.54 | 65.8 |
| anim/evaluate/ms | 0.057 | 0.073 | 1.11 | 62.6 |
| physics/insert/ms | 0.054 | 0.295 | 0.84 | 59.8 |
| anim/evaluate_cpu/ms | 0.052 | 0.071 | 0.40 | 57.8 |
| physics/step/ms | 0.042 | 0.061 | 1.01 | 46.8 |
| weather/map/ms | 0.024 | 0.033 | 0.31 | 26.5 |
| weather/eval/ms | 0.011 | 0.015 | 0.04 | 12.0 |
| script/update/ms | 0.007 | 0.009 | 0.18 | 8.1 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 5.8 |
| controller/avoid/ms | 0.005 | 0.007 | 0.01 | 5.6 |
| anim/apply/ms | 0.004 | 0.005 | 0.10 | 4.4 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 3.8 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 1.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 0.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.7 |
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
| anim/a2/count | count | 0 | 11 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 26 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 19 | 8 |
| crowd/c0/joints/count | count | 272 | 646 | 272 |
| crowd/c0/meshes/count | count | 48 | 114 | 48 |
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1101 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 9708 | 4212 |
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
| frame/work/ms | ms | 13.51 | 16.58 | 13.54 |
| gpu/timed/ms | ms | 13.51 | 16.58 | 13.54 |
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
| phases/opaque3d/batch_sets | count | 22 | 24 | 22 |
| phases/opaque3d/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 22 |
| phases/opaque3d_prepass/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 31 | 36 | 31 |
| phases/shadow/bins | count | 2029 | 2336 | 2029 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5346 | 5666 | 5346 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 31 | 36 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1584 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 182 | 182 | 182 |
| pipelines/waiting | count | 0 | 57 | 0 |
| preprocess/bind_groups | count | 0 | 208 | 0 |
| preprocess/bind_groups_created | count | 0 | 208 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 201 | 201 | 201 |
| process/cpu_usage | % | 19.44 | 38.46 | 15.71 |
| process/mem_usage | GiB | 3.98 | 4.14 | 4.14 |
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
| started_load_count |  | 3546 | 3549 | 3549 |
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
| system/cpu_usage | % | 26.44 | 42.96 | 19.24 |
| system/mem_usage | % | 72.21 | 72.86 | 72.78 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2452 | 2452 | 2452 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 89 | 98 | 89 |
| views/all/bins | count | 5199 | 5438 | 5199 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.8 k | 14.6 k |
| views/all/pipelines | count | 85 | 94 | 85 |
| views/all/pipelines_distinct | count | 75 | 79 | 75 |
| views/all/work_items | count | 14.8 k | 14.9 k | 14.8 k |
| views/cascade/batch_sets | count | 31 | 36 | 31 |
| views/cascade/bins | count | 2029 | 2336 | 2029 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5346 | 5666 | 5346 |
| views/cascade/pipelines | count | 31 | 36 | 31 |
| views/cascade/work_items | count | 5346 | 5666 | 5346 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 58 | 62 | 58 |
| views/main/bins | count | 3170 | 3172 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 54 | 58 | 54 |
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
| anim/bones_written/count | count | 0.30 | 231 | 330 |
| anim/events/count | count | 0.09 | 3 | 101 |
| assets/Image/added | count | 0.02 | 19 | 26 |
| assets/Image/modified | count | 0.00 | 1 | 3 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.08 | 42 | 90 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.03 | 23 | 30 |
| assets/StandardMaterial/modified | count | 4.49 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 4.20 | 1400 | 4628 |
| physics/inserts | count | 1.27 | 155 | 1395 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.82 | 2 | 900 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 3 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 36.5 ms; slowest 20, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2465.8 | main/busy/ms 2440, surface/advance_cpu_ms 66.22, surface/cell_upload_ms 0.87, stream/drive/ms 0.30, weather/map/ms 0.11, stream/tick/ms 0.08, bigspace/total/ms 0.07, script/input/ms 0.05, bigspace/high_precision/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.01 |
| warm-up | 3.03 | 564.9 | main/busy/ms 2477, render/thread/ms 525, render/busy/ms 503, surface/advance_cpu_ms 66.22, render/acquire/ms 20.49, render/graph/ms 3.94, render/present/ms 2.10, stream/drive/ms 0.91, surface/cell_upload_ms 0.87, terrain/update/ms 0.49, stream/tick/ms 0.47, terrain/select/ms 0.30 |
| warm-up | 3.14 | 112.7 | pipelines/wait_ms 1166, render/thread/ms 562, render/busy/ms 537, pipelines/compile_ms 93.27, pipelines/front_ms 86.03, main/busy/ms 53.45, render/graph/ms 31.72, terrain/update/ms 22.42, render/acquire/ms 20.71, pipelines/driver_ms 7.24, render/present/ms 3.64, stream/drive/ms 1.24 |
| warm-up | 3.35 | 91.7 | main/busy/ms 62.40, render/thread/ms 59.64, render/busy/ms 58.17, terrain/update/ms 6.28, render/graph/ms 3.30, stream/drive/ms 1.72, stream/tick/ms 1.36, render/present/ms 1.24, terrain/select/ms 0.64, bigspace/total/ms 0.30, render/acquire/ms 0.23, bigspace/high_precision/ms 0.15 |
| warm-up | 3.51 | 64.6 | render/thread/ms 41.61, render/busy/ms 40.52, main/busy/ms 40.02, stream/drive/ms 30.40, render/graph/ms 2.09, render/present/ms 0.93, stream/tick/ms 0.67, terrain/update/ms 0.47, terrain/select/ms 0.38, render/acquire/ms 0.16, bigspace/total/ms 0.16, stream/spawn/ms 0.14 |
| warm-up | 3.58 | 53.7 | pipelines/wait_ms 1813, pipelines/compile_ms 186, pipelines/front_ms 167, main/busy/ms 60.70, render/thread/ms 44.19, stream/drive/ms 43.41, render/busy/ms 42.11, pipelines/driver_ms 19.50, render/graph/ms 5.73, render/present/ms 1.82, stream/spawn/ms 1.41, stream/tick/ms 0.93 |
| warm-up | 3.62 | 41.8 | pipelines/wait_ms 1813, pipelines/compile_ms 186, pipelines/front_ms 167, main/busy/ms 74.05, render/thread/ms 65.40, render/busy/ms 63.37, stream/drive/ms 47.21, pipelines/driver_ms 19.50, render/graph/ms 6.61, stream/tick/ms 1.96, render/present/ms 1.73, stream/spawn/ms 1.60 |
| warm-up | 3.69 | 74.7 | render/thread/ms 72.32, render/busy/ms 70.69, main/busy/ms 62.96, stream/drive/ms 30.68, terrain/update/ms 5.62, render/graph/ms 4.24, stream/tick/ms 2.11, render/present/ms 1.37, terrain/select/ms 0.57, bigspace/total/ms 0.48, stream/spawn/ms 0.40, bigspace/high_precision/ms 0.30 |
| warm-up | 3.77 | 81.3 | pipelines/wait_ms 402, pipelines/compile_ms 223, pipelines/front_ms 213, render/thread/ms 108, render/busy/ms 107, main/busy/ms 48.75, stream/drive/ms 27.37, pipelines/driver_ms 9.76, terrain/update/ms 7.64, render/graph/ms 1.93, stream/tick/ms 1.13, render/present/ms 0.88 |
| warm-up | 3.91 | 49.3 | main/busy/ms 52.00, stream/drive/ms 32.69, render/thread/ms 30.06, render/busy/ms 28.71, stream/spawn/ms 4.78, render/graph/ms 2.46, stream/tick/ms 1.47, render/present/ms 1.16, terrain/update/ms 0.57, terrain/select/ms 0.47, bigspace/total/ms 0.33, stream/upload/ms 0.26 |
| warm-up | 3.95 | 39.3 | pipelines/wait_ms 418, pipelines/compile_ms 145, pipelines/front_ms 139, render/thread/ms 44.61, render/busy/ms 42.86, main/busy/ms 36.64, stream/drive/ms 8.76, pipelines/driver_ms 6.15, render/graph/ms 3.96, stream/spawn/ms 3.08, stream/tick/ms 2.33, render/present/ms 1.51 |
| warm-up | 4.00 | 47.8 | pipelines/wait_ms 418, pipelines/compile_ms 145, pipelines/front_ms 139, render/thread/ms 56.32, render/busy/ms 54.90, main/busy/ms 45.63, stream/drive/ms 16.35, pipelines/driver_ms 6.15, render/graph/ms 3.49, stream/tick/ms 1.69, render/present/ms 1.20, stream/activate/ms 1.09 |
| warm-up | 4.07 | 40.6 | pipelines/wait_ms 1472, pipelines/compile_ms 152, pipelines/front_ms 142, main/busy/ms 89.59, stream/drive/ms 75.98, render/thread/ms 34.24, render/busy/ms 33.18, pipelines/driver_ms 10.33, render/graph/ms 2.25, stream/spawn/ms 1.45, render/present/ms 0.92, stream/tick/ms 0.82 |
| warm-up | 4.12 | 49.8 | pipelines/wait_ms 6459, pipelines/compile_ms 215, pipelines/front_ms 200, main/busy/ms 53.54, stream/drive/ms 36.53, render/thread/ms 25.93, render/busy/ms 24.92, pipelines/driver_ms 14.86, render/graph/ms 2.27, stream/spawn/ms 2.15, stream/tick/ms 0.91, render/present/ms 0.85 |
| warm-up | 4.55 | 47.3 | pipelines/wait_ms 5332, render/thread/ms 57.18, render/busy/ms 55.16, main/busy/ms 34.33, pipelines/compile_ms 28.74, pipelines/front_ms 23.92, render/graph/ms 5.93, pipelines/driver_ms 4.82, stream/drive/ms 3.23, stream/spawn/ms 2.03, render/present/ms 1.65, bigspace/total/ms 1.21 |
| warm-up | 4.65 | 69.6 | pipelines/wait_ms 5593, pipelines/compile_ms 101, pipelines/front_ms 96.74, render/thread/ms 61.39, render/busy/ms 59.99, main/busy/ms 50.27, pipelines/driver_ms 4.59, stream/drive/ms 4.21, render/graph/ms 3.61, stream/spawn/ms 2.10, render/present/ms 1.23, stream/tick/ms 0.93 |
| warm-up | 4.69 | 48.1 | pipelines/wait_ms 2844, render/thread/ms 76.92, render/busy/ms 75.27, main/busy/ms 50.71, pipelines/compile_ms 38.35, pipelines/front_ms 36.53, render/graph/ms 4.87, stream/drive/ms 4.22, stream/spawn/ms 2.06, pipelines/driver_ms 1.82, render/present/ms 1.43, bigspace/total/ms 0.93 |
| warm-up | 4.89 | 45.8 | render/thread/ms 53.68, render/busy/ms 52.04, main/busy/ms 40.45, stream/drive/ms 17.36, render/graph/ms 4.84, stream/spawn/ms 3.03, render/present/ms 1.33, bigspace/total/ms 1.22, bigspace/high_precision/ms 0.79, stream/tick/ms 0.69, terrain/update/ms 0.57, terrain/select/ms 0.48 |
| warm-up | 5.34 | 49.4 | main/busy/ms 43.41, render/thread/ms 42.11, render/busy/ms 40.51, stream/drive/ms 15.30, render/graph/ms 4.08, stream/spawn/ms 3.79, render/present/ms 1.37, stream/tick/ms 1.31, bigspace/total/ms 0.91, terrain/update/ms 0.83, terrain/select/ms 0.67, bigspace/high_precision/ms 0.40 |
| warm-up | 6.03 | 47.2 | main/busy/ms 55.48, render/thread/ms 46.36, render/busy/ms 44.72, render/graph/ms 7.47, render/present/ms 1.37, stream/drive/ms 1.24, physics/insert/ms 1.02, bigspace/total/ms 1.02, anim/evaluate_cpu/ms 0.61, stream/tick/ms 0.58, bigspace/low_precision/ms 0.49, terrain/update/ms 0.46 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\claude\3866c27c11752a1d\4ff90442-bcd4-4820-b03c-4dd8a81992a4\scratchpad\bin\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\tc_pbrwater\balmora@2560x1440_off=normalmaps_orm_parallax_waterssr_shorewaves-3 --bench-baseline none --off normalmaps,orm,parallax,waterssr,shorewaves
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **93%** (min 43%) · 2573 MHz (min 2355) · memory 8001 MHz · 92.4 W · 76 °C (max 80) · VRAM 5144 MiB · throttled 97% of samples
