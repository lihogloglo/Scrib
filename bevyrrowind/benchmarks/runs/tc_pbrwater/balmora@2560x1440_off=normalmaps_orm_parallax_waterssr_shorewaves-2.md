# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: waterssr,shorewaves,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1003 frames over 15.0 s · **median 14.88 ms (67 fps)** · mean 14.96 · p95 16.29 · p99 17.21 (1% low 58 fps) · worst 21.13 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 292 frames over 8.0 s, worst 2492.0 ms
- hitches (> 37.2 ms): **0** measured, 17 in warm-up
- timed GPU passes: 13.07 ms of a 14.96 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 14.88 ms, limit 11 ms
- pass worst frame 21.13 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 6.821 | 6.821 | 7.506 | 1.00 | 0.051 |
| early prepass | 0.961 | 0.961 | 1.317 | 1.00 | 0.087 |
| ssao | 0.797 | 0.797 | 1.100 | 1.00 | 0.006 |
| clouds_composite | 0.489 | 0.489 | 0.823 | 1.00 | 0.001 |
| main_transmissive_pass_3d | 0.489 | 0.489 | 0.840 | 1.00 | 0.006 |
| clouds_trace | 0.447 | 0.447 | 0.500 | 1.00 | 0.001 |
| taa | 0.414 | 0.414 | 0.758 | 1.00 | 0.002 |
| atmosphere_luts | 0.367 | 0.367 | 0.649 | 1.00 | 0.003 |
| bloom | 0.337 | 0.337 | 0.657 | 1.00 | 0.058 |
| render_sky | 0.213 | 0.213 | 0.207 | 1.00 | 0.001 |
| clouds_resolve | 0.195 | 0.195 | 0.204 | 1.00 | 0.001 |
| water_snapshot | 0.193 | 0.193 | 0.233 | 1.00 | 0.001 |
| uniform_allocation | 0.187 | 0.187 | 0.183 | 1.00 | 0.028 |
| atmos_froxel_inject | 0.158 | 0.158 | 0.155 | 1.00 | 0.002 |
| bin_unpacking | 0.140 | 0.140 | 0.144 | 1.00 | 0.021 |
| water_fft | 0.139 | 0.139 | 0.139 | 1.00 | 0.002 |
| upscaling | 0.128 | 0.128 | 0.148 | 1.00 | 0.004 |
| water_velocity | 0.124 | 0.124 | 0.125 | 1.00 | 0.001 |
| tonemapping | 0.095 | 0.095 | 0.093 | 1.00 | 0.003 |
| clustering | 0.069 | 0.069 | 0.077 | 1.00 | 0.019 |
| lightprobe_irradiance_map | 0.056 | 0.444 | 0.762 | 0.13 | 0.001 |
| early_mesh_preprocessing | 0.043 | 0.043 | 0.042 | 1.00 | 0.004 |
| atmos_froxel_integrate | 0.033 | 0.033 | 0.035 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.030 | 0.240 | 0.241 | 0.13 | 0.003 |
| water_mips | 0.025 | 0.025 | 0.024 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.023 | 0.023 | 0.027 | 1.00 | 0.066 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.026 | 1.00 | 0.007 |
| night_sky | 0.011 | 0.011 | 0.013 | 1.00 | 0.001 |
| atmosphere_environment | 0.011 | 0.011 | 0.012 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.13 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.015 | 0.13 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.01 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 83.047 | 0.000 | 11580.15 | 83295.8 |
| render/thread/ms | 14.058 | 15.409 | 20.13 | 14099.7 |
| render/busy/ms | 6.862 | 7.721 | 14.56 | 6882.8 |
| render/acquire/ms | 6.727 | 8.090 | 9.89 | 6746.8 |
| main/busy/ms | 5.887 | 6.422 | 12.51 | 5904.9 |
| render/graph/ms | 3.606 | 4.270 | 10.94 | 3617.3 |
| pipelines/compile_ms | 1.078 | 0.000 | 126.98 | 1080.9 |
| pipelines/front_ms | 1.041 | 0.000 | 123.47 | 1044.2 |
| render/present/ms | 0.469 | 0.567 | 2.05 | 470.1 |
| bigspace/total/ms | 0.274 | 0.355 | 0.58 | 274.3 |
| terrain/update/ms | 0.227 | 0.266 | 0.50 | 227.7 |
| terrain/select/ms | 0.198 | 0.233 | 0.47 | 199.0 |
| stream/drive/ms | 0.174 | 0.201 | 0.40 | 174.3 |
| far/select/ms | 0.152 | 0.173 | 0.44 | 152.4 |
| stream/tick/ms | 0.149 | 0.171 | 0.32 | 149.0 |
| controller/feed/ms | 0.135 | 0.167 | 0.31 | 135.6 |
| controller/move/ms | 0.103 | 0.189 | 0.29 | 103.3 |
| bigspace/high_precision/ms | 0.092 | 0.113 | 0.36 | 91.8 |
| bigspace/low_precision/ms | 0.061 | 0.079 | 0.21 | 61.3 |
| anim/evaluate_cpu/ms | 0.059 | 0.076 | 0.24 | 59.4 |
| anim/evaluate/ms | 0.057 | 0.074 | 0.22 | 57.3 |
| physics/step/ms | 0.045 | 0.063 | 0.59 | 45.2 |
| pipelines/driver_ms | 0.037 | 0.000 | 4.99 | 36.7 |
| weather/map/ms | 0.022 | 0.031 | 0.06 | 22.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.03 | 10.1 |
| physics/insert/ms | 0.007 | 0.016 | 0.51 | 7.0 |
| script/update/ms | 0.006 | 0.008 | 0.02 | 6.2 |
| controller/avoid/ms | 0.005 | 0.007 | 0.02 | 5.0 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 4.5 |
| anim/apply/ms | 0.004 | 0.005 | 0.01 | 3.8 |
| script/ui/ms | 0.003 | 0.004 | 0.11 | 3.0 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 1.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 0.9 |
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
| anim/a2/count | count | 0 | 6 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 11 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 14 | 8 |
| crowd/c0/joints/count | count | 272 | 476 | 272 |
| crowd/c0/meshes/count | count | 48 | 84 | 48 |
| crowd/c1/count | count | 122 | 123 | 122 |
| crowd/c1/joints/count | count | 4848 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1089 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 5856 | 4212 |
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
| frame/work/ms | ms | 13.60 | 16.75 | 14.94 |
| gpu/timed/ms | ms | 13.60 | 16.75 | 14.94 |
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
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 22 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
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
| phases/shadow/items | count | 5346 | 5672 | 5346 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 31 | 36 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 294 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 182 | 182 | 182 |
| pipelines/waiting | count | 0 | 20 | 0 |
| preprocess/bind_groups | count | 0 | 102 | 0 |
| preprocess/bind_groups_created | count | 0 | 102 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 201 | 201 | 201 |
| process/cpu_usage | % | 18.79 | 46.11 | 3.98 |
| process/mem_usage | GiB | 4.06 | 4.18 | 4.15 |
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
| stream/latency/s | s | 0.06 | 0.06 | 0.06 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 25.41 | 39.60 | 19.63 |
| system/mem_usage | % | 72.42 | 72.95 | 72.94 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2452 | 2452 | 2452 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 89 | 94 | 89 |
| views/all/bins | count | 5199 | 5406 | 5199 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.8 k | 14.6 k |
| views/all/pipelines | count | 85 | 90 | 85 |
| views/all/pipelines_distinct | count | 75 | 79 | 75 |
| views/all/work_items | count | 14.8 k | 14.9 k | 14.8 k |
| views/cascade/batch_sets | count | 31 | 36 | 31 |
| views/cascade/bins | count | 2029 | 2336 | 2029 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5346 | 5672 | 5346 |
| views/cascade/pipelines | count | 31 | 36 | 31 |
| views/cascade/work_items | count | 5346 | 5672 | 5346 |
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
| views/main/bins | count | 3170 | 3170 | 3170 |
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
| anim/bones_written/count | count | 0.13 | 66 | 132 |
| anim/events/count | count | 0.14 | 3 | 145 |
| assets/Image/added | count | 0.02 | 13 | 20 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.05 | 25 | 48 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.02 | 16 | 24 |
| assets/StandardMaterial/modified | count | 4.94 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.66 | 779 | 1669 |
| physics/inserts | count | 0.30 | 79 | 296 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.90 | 1 | 900 |
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

## Hitches (> 37.2 ms; slowest 17, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2492.0 | main/busy/ms 2470, surface/advance_cpu_ms 66.99, surface/cell_upload_ms 0.92, stream/drive/ms 0.31, weather/map/ms 0.19, bigspace/total/ms 0.16, stream/tick/ms 0.09, bigspace/high_precision/ms 0.07, weather/eval/ms 0.03, script/update/ms 0.03, far/select/ms 0.02, script/input/ms 0.01 |
| warm-up | 2.96 | 463.1 | main/busy/ms 2505, render/thread/ms 431, render/busy/ms 409, surface/advance_cpu_ms 66.99, render/acquire/ms 20.35, render/graph/ms 4.17, render/present/ms 1.89, stream/drive/ms 1.45, surface/cell_upload_ms 0.92, stream/tick/ms 0.92, terrain/update/ms 0.33, weather/map/ms 0.27 |
| warm-up | 3.16 | 200.0 | pipelines/wait_ms 496, render/thread/ms 465, render/busy/ms 441, pipelines/compile_ms 188, pipelines/front_ms 177, main/busy/ms 52.55, render/graph/ms 29.04, render/acquire/ms 20.61, terrain/update/ms 20.24, pipelines/driver_ms 11.71, render/present/ms 3.09, stream/drive/ms 1.70 |
| warm-up | 3.37 | 85.2 | pipelines/compile_ms 206, pipelines/front_ms 192, main/busy/ms 60.89, render/thread/ms 53.71, render/busy/ms 52.07, pipelines/wait_ms 29.87, pipelines/driver_ms 14.66, render/graph/ms 3.04, render/present/ms 1.43, stream/drive/ms 1.39, stream/tick/ms 1.14, terrain/update/ms 0.59 |
| warm-up | 3.55 | 72.9 | render/thread/ms 40.92, main/busy/ms 39.99, render/busy/ms 39.85, stream/drive/ms 29.25, render/graph/ms 1.85, stream/tick/ms 0.93, render/present/ms 0.91, terrain/update/ms 0.46, terrain/select/ms 0.37, bigspace/total/ms 0.19, render/acquire/ms 0.16, stream/spawn/ms 0.10 |
| warm-up | 3.63 | 71.8 | pipelines/wait_ms 3482, pipelines/compile_ms 262, pipelines/front_ms 249, render/thread/ms 48.80, render/busy/ms 46.21, main/busy/ms 35.29, stream/drive/ms 19.63, pipelines/driver_ms 13.29, render/graph/ms 5.44, render/present/ms 2.32, stream/spawn/ms 1.04, stream/tick/ms 1.01 |
| warm-up | 3.76 | 115.8 | pipelines/wait_ms 4781, pipelines/compile_ms 285, pipelines/driver_ms 161, pipelines/front_ms 124, render/thread/ms 106, render/busy/ms 105, main/busy/ms 41.36, stream/drive/ms 11.64, render/graph/ms 2.78, stream/spawn/ms 1.42, render/present/ms 1.29, stream/tick/ms 1.03 |
| warm-up | 3.81 | 42.5 | pipelines/wait_ms 4781, pipelines/compile_ms 285, pipelines/driver_ms 161, render/thread/ms 125, pipelines/front_ms 124, render/busy/ms 123, main/busy/ms 44.02, stream/drive/ms 13.19, render/graph/ms 3.42, stream/spawn/ms 3.13, render/present/ms 1.52, terrain/update/ms 1.10 |
| warm-up | 3.97 | 129.8 | render/thread/ms 102, render/busy/ms 67.48, main/busy/ms 53.63, stream/drive/ms 35.70, render/present/ms 34.23, render/graph/ms 3.68, stream/spawn/ms 1.31, stream/tick/ms 0.96, terrain/update/ms 0.94, terrain/select/ms 0.72, stream/activate/ms 0.48, bigspace/total/ms 0.39 |
| warm-up | 4.06 | 40.8 | pipelines/wait_ms 4273, pipelines/compile_ms 73.20, pipelines/front_ms 69.15, main/busy/ms 42.23, stream/drive/ms 25.40, render/thread/ms 18.74, render/busy/ms 17.58, pipelines/driver_ms 4.05, render/graph/ms 2.72, render/present/ms 1.01, stream/tick/ms 0.84, terrain/update/ms 0.53 |
| warm-up | 4.16 | 58.0 | pipelines/wait_ms 12.3 k, pipelines/compile_ms 89.95, pipelines/front_ms 84.03, main/busy/ms 62.98, stream/drive/ms 51.93, render/thread/ms 29.35, render/busy/ms 28.03, pipelines/driver_ms 5.92, render/graph/ms 4.36, render/present/ms 1.07, stream/tick/ms 0.89, terrain/update/ms 0.48 |
| warm-up | 4.22 | 55.4 | main/busy/ms 51.65, stream/drive/ms 34.77, render/thread/ms 25.72, render/busy/ms 24.11, render/graph/ms 4.34, stream/spawn/ms 2.18, render/present/ms 1.46, terrain/update/ms 0.77, stream/tick/ms 0.67, terrain/select/ms 0.62, bigspace/total/ms 0.38, render/acquire/ms 0.16 |
| warm-up | 4.80 | 42.5 | pipelines/wait_ms 12.6 k, pipelines/compile_ms 102, pipelines/front_ms 97.20, render/thread/ms 58.56, render/busy/ms 57.02, main/busy/ms 51.79, render/graph/ms 7.89, stream/drive/ms 4.93, pipelines/driver_ms 4.92, stream/spawn/ms 1.98, stream/tick/ms 1.29, render/present/ms 1.24 |
| warm-up | 4.86 | 61.8 | pipelines/wait_ms 7850, pipelines/compile_ms 106, pipelines/front_ms 102, render/thread/ms 80.98, render/busy/ms 79.28, main/busy/ms 76.06, render/graph/ms 8.64, stream/drive/ms 5.71, terrain/update/ms 5.04, pipelines/driver_ms 4.57, stream/spawn/ms 2.01, render/present/ms 1.37 |
| warm-up | 4.91 | 49.9 | pipelines/wait_ms 4765, pipelines/compile_ms 92.20, pipelines/front_ms 88.77, render/thread/ms 87.99, render/busy/ms 86.16, main/busy/ms 66.26, terrain/update/ms 7.69, render/graph/ms 6.24, stream/drive/ms 5.43, pipelines/driver_ms 3.43, stream/spawn/ms 2.13, render/present/ms 1.47 |
| warm-up | 4.96 | 43.1 | pipelines/wait_ms 1676, render/thread/ms 74.89, render/busy/ms 73.04, main/busy/ms 45.20, pipelines/compile_ms 11.26, pipelines/front_ms 10.69, render/graph/ms 7.14, stream/drive/ms 3.80, terrain/update/ms 3.32, stream/spawn/ms 2.47, render/present/ms 1.50, physics/insert/ms 1.20 |
| warm-up | 5.49 | 46.6 | render/thread/ms 44.01, render/busy/ms 42.11, main/busy/ms 37.17, stream/drive/ms 12.19, render/graph/ms 9.35, stream/spawn/ms 1.72, render/present/ms 1.67, stream/tick/ms 1.32, bigspace/total/ms 1.24, terrain/update/ms 0.70, bigspace/high_precision/ms 0.69, terrain/select/ms 0.58 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\claude\3866c27c11752a1d\4ff90442-bcd4-4820-b03c-4dd8a81992a4\scratchpad\bin\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\tc_pbrwater\balmora@2560x1440_off=normalmaps_orm_parallax_waterssr_shorewaves-2 --bench-baseline none --off normalmaps,orm,parallax,waterssr,shorewaves
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 5%, CPU 7%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2580 MHz (min 2580) · memory 8001 MHz · 99.0 W · 80 °C (max 81) · VRAM 5163 MiB · throttled 100% of samples
