# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: waterssr,shorewaves,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5250 frames over 65.7 s · **median 12.36 ms (81 fps)** · mean 12.51 · p95 14.93 · p99 17.72 (1% low 56 fps) · worst 33.25 ms · 38 spikes over 1.5× median · 23.8 stutters/min (work 18.3/min)
- warm-up: 335 frames over 8.4 s, worst 2615.8 ms
- hitches (> 30.9 ms): **3** measured, 21 in warm-up
- timed GPU passes: 9.32 ms of a 12.51 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.36 ms, limit 11 ms
- **FAIL** frame_ms: 26 stutters (> 1.5x moving median of 60) in 1.09 min = 23.76/min, limit 1/min
- **FAIL** worst frame 33.25 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.855 | 4.046 | 5.052 | 0.95 | 0.063 |
| ssao | 0.765 | 0.803 | 1.106 | 0.95 | 0.008 |
| early prepass | 0.668 | 0.701 | 1.153 | 0.95 | 0.097 |
| clouds_composite | 0.442 | 0.464 | 0.795 | 0.95 | 0.002 |
| taa | 0.395 | 0.415 | 0.741 | 0.95 | 0.002 |
| clouds_trace | 0.319 | 0.335 | 0.618 | 0.95 | 0.001 |
| bloom | 0.318 | 0.334 | 0.343 | 0.95 | 0.069 |
| atmosphere_luts | 0.316 | 0.332 | 0.534 | 0.95 | 0.004 |
| uniform_allocation | 0.212 | 0.223 | 0.227 | 0.95 | 0.039 |
| render_sky | 0.204 | 0.215 | 0.213 | 0.95 | 0.002 |
| auxiliary_views | 0.195 | 0.205 | 0.729 | 0.95 | 0.184 |
| clouds_resolve | 0.187 | 0.197 | 0.204 | 0.95 | 0.001 |
| water_snapshot | 0.175 | 0.184 | 0.325 | 0.95 | 0.002 |
| bin_unpacking | 0.165 | 0.173 | 0.183 | 0.95 | 0.028 |
| atmos_froxel_inject | 0.154 | 0.162 | 0.159 | 0.95 | 0.002 |
| water_fft | 0.135 | 0.142 | 0.140 | 0.95 | 0.003 |
| upscaling | 0.127 | 0.133 | 0.141 | 0.95 | 0.004 |
| water_velocity | 0.105 | 0.111 | 0.113 | 0.95 | 0.002 |
| main_transmissive_pass_3d | 0.092 | 0.096 | 0.250 | 0.95 | 0.008 |
| tonemapping | 0.089 | 0.094 | 0.094 | 0.95 | 0.003 |
| clustering | 0.065 | 0.068 | 0.075 | 0.95 | 0.020 |
| lightprobe_irradiance_map | 0.050 | 0.438 | 0.724 | 0.12 | 0.001 |
| surface_trail | 0.045 | 0.059 | 0.058 | 0.76 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.045 | 0.049 | 0.95 | 0.006 |
| atmos_froxel_integrate | 0.034 | 0.035 | 0.037 | 0.95 | 0.001 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.95 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.246 | 0.546 | 0.12 | 0.004 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.95 | 0.003 |
| per_view_shadow_pass | 0.022 | 0.023 | 0.032 | 0.95 | 0.007 |
| night_sky | 0.019 | 0.020 | 0.047 | 0.95 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.031 | 0.95 | 0.113 |
| clouds_fog_light | 0.010 | 0.011 | 0.011 | 0.95 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.95 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.95 | 0.003 |
| lightprobe_copy | 0.002 | 0.016 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 18.411 | 0.000 | 7875.69 | 96658.3 |
| render/thread/ms | 11.299 | 13.632 | 29.72 | 59317.9 |
| render/busy/ms | 10.035 | 12.627 | 28.94 | 52685.5 |
| main/busy/ms | 9.122 | 10.933 | 26.83 | 47892.8 |
| render/graph/ms | 5.606 | 7.349 | 16.77 | 29431.0 |
| render/acquire/ms | 0.697 | 2.318 | 5.42 | 3658.7 |
| render/present/ms | 0.566 | 1.306 | 16.34 | 2973.7 |
| bigspace/total/ms | 0.506 | 1.053 | 3.58 | 2655.1 |
| pipelines/compile_ms | 0.452 | 0.000 | 172.51 | 2372.7 |
| pipelines/front_ms | 0.437 | 0.000 | 166.49 | 2294.4 |
| terrain/update/ms | 0.309 | 0.398 | 2.32 | 1624.2 |
| controller/feed/ms | 0.302 | 0.407 | 0.84 | 1587.8 |
| terrain/select/ms | 0.275 | 0.352 | 0.69 | 1445.8 |
| stream/drive/ms | 0.251 | 0.363 | 18.82 | 1317.3 |
| bigspace/low_precision/ms | 0.199 | 0.355 | 3.14 | 1044.8 |
| stream/tick/ms | 0.192 | 0.277 | 0.72 | 1006.9 |
| far/select/ms | 0.192 | 0.281 | 0.59 | 1005.9 |
| anim/evaluate_cpu/ms | 0.176 | 0.224 | 0.53 | 923.9 |
| controller/move/ms | 0.135 | 0.233 | 0.52 | 707.1 |
| bigspace/high_precision/ms | 0.126 | 0.254 | 2.65 | 659.7 |
| anim/evaluate/ms | 0.123 | 0.265 | 1.68 | 643.9 |
| physics/step/ms | 0.055 | 0.090 | 0.66 | 287.9 |
| physics/insert/ms | 0.042 | 0.409 | 0.80 | 219.5 |
| weather/map/ms | 0.025 | 0.034 | 0.37 | 132.1 |
| pipelines/driver_ms | 0.015 | 0.000 | 6.02 | 78.4 |
| anim/apply/ms | 0.012 | 0.022 | 0.15 | 61.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.04 | 56.6 |
| controller/avoid/ms | 0.011 | 0.014 | 0.17 | 55.5 |
| script/update/ms | 0.007 | 0.009 | 0.20 | 39.2 |
| stream/despawn/ms | 0.006 | 0.000 | 1.23 | 31.2 |
| script/ui/ms | 0.005 | 0.007 | 0.24 | 27.3 |
| script/input/ms | 0.005 | 0.007 | 0.14 | 26.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 25.5 |
| controller/write/ms | 0.004 | 0.004 | 0.10 | 18.5 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.70 | 15.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.44 | 5.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 4.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 11 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 561 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1302 | 2262 | 593 |
| crowd/c0/meshes/count | count | 355 | 646 | 156 |
| crowd/c1/count | count | 145 | 158 | 147 |
| crowd/c1/joints/count | count | 5505 | 6105 | 5571 |
| crowd/c1/meshes/count | count | 1166 | 1331 | 1177 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 185 | 236 | 222 |
| crowd/joints_extracted/count | count | 4245 | 8457 | 6135 |
| crowd/joints_uploaded/count | count | 4229 | 6643 | 6135 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.81 | 28.94 | 10.75 |
| gpu/timed/ms | ms | 10.58 | 14.84 | 10.75 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 559.62 M | 560.80 M | 560.80 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 189 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 850 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 189 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 850 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1276 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3475 | 5145 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1276 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3475 | 5145 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 55 | 289 | 39 |
| phases/shadow/bins | count | 3821 | 5372 | 2199 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9668 | 12.4 k | 5650 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 140 | 304 | 39 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2718 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 338 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 206 | 210 | 210 |
| pipelines/waiting | count | 0 | 30 | 0 |
| preprocess/bind_groups | count | 11 | 747 | 6 |
| preprocess/bind_groups_created | count | 11 | 747 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 225 | 323 | 215 |
| process/cpu_usage | % | 29.02 | 46.63 | 18.84 |
| process/mem_usage | GiB | 5.25 | 5.66 | 5.66 |
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
| started_load_count |  | 4765 | 4859 | 4859 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 337 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 35.52 | 54.18 | 26.44 |
| system/mem_usage | % | 76.06 | 77.44 | 77.37 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2794 | 2906 | 2906 |
| textures/images | count | 2589 | 2644 | 2644 |
| views/all/batch_sets | count | 111 | 341 | 99 |
| views/all/bins | count | 6497 | 8250 | 5679 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.5 k | 25.2 k | 13.2 k |
| views/all/pipelines | count | 195 | 360 | 95 |
| views/all/pipelines_distinct | count | 82 | 91 | 77 |
| views/all/work_items | count | 17.5 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 48 | 61 | 39 |
| views/cascade/bins | count | 2600 | 3104 | 2199 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8223 | 10.0 k | 5650 |
| views/cascade/pipelines | count | 48 | 61 | 39 |
| views/cascade/work_items | count | 8223 | 10.0 k | 5650 |
| views/lamp_face/batch_sets | count | 7 | 236 | 0 |
| views/lamp_face/bins | count | 1112 | 2456 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1290 | 2817 | 0 |
| views/lamp_face/pipelines | count | 90 | 251 | 0 |
| views/lamp_face/work_items | count | 62 | 2559 | 0 |
| views/main/batch_sets | count | 58 | 62 | 60 |
| views/main/bins | count | 2852 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8708 | 13.0 k | 7592 |
| views/main/pipelines | count | 54 | 58 | 56 |
| views/main/work_items | count | 8860 | 13.3 k | 7900 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 56.73 | 440 | 297.8 k |
| anim/events/count | count | 0.09 | 3 | 487 |
| assets/Image/added | count | 0.11 | 43 | 597 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.78 | 168 | 4105 |
| assets/Mesh/removed | count | 0.93 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 469 |
| assets/StandardMaterial/modified | count | 4.15 | 22 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.47 | 1485 | 7734 |
| physics/inserts | count | 0.49 | 111 | 2584 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.44 | 145 | 2335 |
| physics/steps | count | 0.75 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6448 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1414 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.9 ms; slowest 24, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.62 | 2615.8 | main/busy/ms 2593, surface/advance_cpu_ms 66.69, surface/cell_upload_ms 0.78, stream/drive/ms 0.33, weather/map/ms 0.21, bigspace/total/ms 0.11, stream/tick/ms 0.09, bigspace/high_precision/ms 0.07, weather/eval/ms 0.03, script/update/ms 0.03, far/select/ms 0.01, script/ui/ms 0.01 |
| warm-up | 3.15 | 537.7 | main/busy/ms 2635, render/thread/ms 506, render/busy/ms 486, surface/advance_cpu_ms 66.69, render/acquire/ms 18.00, render/graph/ms 3.72, render/present/ms 2.01, stream/drive/ms 1.19, surface/cell_upload_ms 0.78, stream/tick/ms 0.60, terrain/update/ms 0.55, terrain/select/ms 0.38 |
| warm-up | 3.50 | 346.8 | pipelines/wait_ms 7905, render/thread/ms 547, render/busy/ms 525, pipelines/compile_ms 338, pipelines/front_ms 315, main/busy/ms 58.64, render/graph/ms 35.51, terrain/update/ms 23.53, pipelines/driver_ms 23.36, render/acquire/ms 18.17, render/present/ms 3.46, stream/drive/ms 1.44 |
| warm-up | 3.71 | 82.2 | pipelines/compile_ms 192, pipelines/front_ms 172, main/busy/ms 61.27, render/thread/ms 52.32, render/busy/ms 50.80, pipelines/wait_ms 38.05, pipelines/driver_ms 20.19, render/graph/ms 3.17, stream/drive/ms 1.41, render/present/ms 1.31, stream/tick/ms 0.94, terrain/update/ms 0.75 |
| warm-up | 3.86 | 71.3 | pipelines/wait_ms 1299, pipelines/compile_ms 189, pipelines/front_ms 182, main/busy/ms 47.30, render/thread/ms 36.80, render/busy/ms 35.80, stream/drive/ms 34.78, pipelines/driver_ms 6.89, render/graph/ms 2.01, terrain/update/ms 1.00, render/present/ms 0.84, terrain/select/ms 0.82 |
| warm-up | 3.93 | 48.7 | render/thread/ms 46.41, render/busy/ms 43.85, main/busy/ms 37.33, stream/drive/ms 22.17, render/graph/ms 10.09, render/present/ms 2.30, stream/tick/ms 1.64, stream/spawn/ms 1.33, terrain/update/ms 0.62, terrain/select/ms 0.50, render/acquire/ms 0.26, bigspace/total/ms 0.23 |
| warm-up | 3.96 | 35.7 | render/thread/ms 59.67, render/busy/ms 57.66, main/busy/ms 25.63, render/graph/ms 5.69, stream/drive/ms 4.85, stream/tick/ms 2.43, render/present/ms 1.77, stream/spawn/ms 1.55, terrain/update/ms 0.64, terrain/select/ms 0.53, bigspace/total/ms 0.26, render/acquire/ms 0.24 |
| warm-up | 4.01 | 49.0 | render/thread/ms 55.86, render/busy/ms 54.55, main/busy/ms 33.96, stream/drive/ms 12.45, render/graph/ms 4.04, stream/tick/ms 1.71, render/present/ms 1.12, terrain/update/ms 0.79, terrain/select/ms 0.66, stream/activate/ms 0.55, stream/spawn/ms 0.38, bigspace/total/ms 0.26 |
| warm-up | 4.06 | 49.7 | render/thread/ms 61.95, render/busy/ms 60.79, main/busy/ms 33.65, stream/drive/ms 12.41, render/graph/ms 2.49, terrain/update/ms 1.02, render/present/ms 0.99, stream/activate/ms 0.96, stream/tick/ms 0.95, stream/spawn/ms 0.86, terrain/select/ms 0.83, bigspace/total/ms 0.36 |
| warm-up | 4.13 | 50.6 | pipelines/wait_ms 2615, pipelines/compile_ms 236, pipelines/front_ms 228, main/busy/ms 43.87, stream/drive/ms 29.59, render/thread/ms 24.44, render/busy/ms 23.42, pipelines/driver_ms 7.94, render/graph/ms 2.94, stream/spawn/ms 1.38, terrain/update/ms 0.88, render/present/ms 0.84 |
| warm-up | 4.22 | 38.3 | render/thread/ms 55.42, render/busy/ms 52.67, render/graph/ms 26.94, main/busy/ms 17.72, stream/drive/ms 4.05, render/present/ms 2.53, stream/spawn/ms 2.22, terrain/update/ms 0.89, stream/tick/ms 0.88, terrain/select/ms 0.76, bigspace/total/ms 0.28, render/acquire/ms 0.22 |
| warm-up | 4.29 | 42.5 | pipelines/wait_ms 3581, pipelines/compile_ms 190, pipelines/front_ms 180, main/busy/ms 74.06, stream/drive/ms 52.02, render/thread/ms 49.98, render/busy/ms 48.91, pipelines/driver_ms 10.42, render/graph/ms 8.30, stream/tick/ms 1.52, stream/spawn/ms 1.23, terrain/update/ms 1.02 |
| warm-up | 4.35 | 57.6 | pipelines/wait_ms 4324, pipelines/compile_ms 244, pipelines/front_ms 230, main/busy/ms 71.64, stream/drive/ms 52.85, render/thread/ms 42.04, render/busy/ms 41.09, pipelines/driver_ms 14.57, render/graph/ms 2.76, stream/spawn/ms 2.62, terrain/update/ms 0.90, render/present/ms 0.81 |
| warm-up | 4.45 | 78.8 | pipelines/wait_ms 9650, pipelines/compile_ms 137, pipelines/front_ms 127, main/busy/ms 78.62, stream/drive/ms 51.87, render/thread/ms 38.72, render/busy/ms 37.57, pipelines/driver_ms 10.07, render/graph/ms 5.36, terrain/update/ms 4.39, stream/spawn/ms 2.05, stream/tick/ms 0.99 |
| warm-up | 4.51 | 46.6 | pipelines/wait_ms 741, main/busy/ms 48.01, stream/drive/ms 34.32, render/thread/ms 27.69, render/busy/ms 26.51, pipelines/compile_ms 14.58, pipelines/front_ms 13.90, render/graph/ms 5.21, stream/activate/ms 1.14, render/present/ms 1.00, stream/tick/ms 0.94, stream/spawn/ms 0.75 |
| warm-up | 5.70 | 33.5 | pipelines/wait_ms 7454, pipelines/compile_ms 166, pipelines/front_ms 153, render/thread/ms 35.81, render/busy/ms 32.78, main/busy/ms 22.31, pipelines/driver_ms 12.96, render/graph/ms 6.72, stream/drive/ms 4.20, render/present/ms 2.83, stream/spawn/ms 2.16, stream/tick/ms 1.07 |
| warm-up | 5.82 | 32.2 | pipelines/wait_ms 13.5 k, pipelines/compile_ms 59.28, main/busy/ms 58.36, pipelines/front_ms 55.22, render/thread/ms 32.64, render/busy/ms 31.02, stream/drive/ms 7.73, render/graph/ms 6.45, pipelines/driver_ms 4.06, physics/insert/ms 1.81, render/present/ms 1.36, stream/tick/ms 0.69 |
| warm-up | 5.87 | 48.7 | pipelines/wait_ms 5717, main/busy/ms 69.28, pipelines/compile_ms 49.84, pipelines/front_ms 47.14, render/thread/ms 44.64, render/busy/ms 42.76, render/graph/ms 8.97, stream/drive/ms 4.09, pipelines/driver_ms 2.70, stream/spawn/ms 2.68, physics/insert/ms 2.09, render/present/ms 1.53 |
| warm-up | 5.92 | 50.3 | pipelines/wait_ms 6088, pipelines/compile_ms 94.08, pipelines/front_ms 91.55, main/busy/ms 65.55, render/thread/ms 54.14, render/busy/ms 52.34, render/graph/ms 8.31, stream/drive/ms 5.34, stream/spawn/ms 3.71, pipelines/driver_ms 2.53, physics/insert/ms 1.48, render/present/ms 1.44 |
| warm-up | 5.97 | 55.1 | pipelines/wait_ms 4232, pipelines/compile_ms 86.81, pipelines/front_ms 84.47, render/thread/ms 61.76, render/busy/ms 60.49, main/busy/ms 46.60, render/graph/ms 5.45, stream/drive/ms 3.65, pipelines/driver_ms 2.34, stream/spawn/ms 2.29, physics/insert/ms 1.13, render/present/ms 1.02 |
| warm-up | 6.73 | 48.0 | pipelines/wait_ms 11.1 k, main/busy/ms 64.40, pipelines/compile_ms 45.44, pipelines/front_ms 42.59, render/thread/ms 40.13, render/busy/ms 38.41, render/graph/ms 10.59, terrain/update/ms 5.54, pipelines/driver_ms 2.86, stream/drive/ms 1.72, render/present/ms 1.47, anim/evaluate_cpu/ms 1.38 |
| measure | 0.76 | 31.7 | pipelines/wait_ms 6498, main/busy/ms 40.45, render/thread/ms 31.23, render/busy/ms 29.73, pipelines/compile_ms 26.69, pipelines/front_ms 25.04, stream/drive/ms 16.66, render/graph/ms 13.17, pipelines/driver_ms 1.65, render/present/ms 1.31, bigspace/total/ms 1.03, controller/feed/ms 0.67 |
| measure | 19.27 | 31.1 | render/thread/ms 40.72, render/busy/ms 39.32, render/graph/ms 23.14, main/busy/ms 18.47, render/present/ms 1.24, bigspace/total/ms 0.88, controller/feed/ms 0.74, terrain/update/ms 0.68, terrain/select/ms 0.61, stream/drive/ms 0.58, stream/tick/ms 0.48, far/select/ms 0.42 |
| measure | 59.05 | 33.3 | render/thread/ms 40.05, render/busy/ms 38.81, main/busy/ms 26.96, render/graph/ms 10.25, stream/drive/ms 1.43, stream/tick/ms 1.32, bigspace/total/ms 1.10, controller/feed/ms 1.03, render/present/ms 1.03, terrain/update/ms 0.88, terrain/select/ms 0.78, far/select/ms 0.75 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\claude\3866c27c11752a1d\4ff90442-bcd4-4820-b03c-4dd8a81992a4\scratchpad\bin\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\tc_pbrwater\balmora_walk@2560x1440_off=normalmaps_orm_parallax_waterssr_shorewaves-3 --bench-baseline none --off normalmaps,orm,parallax,waterssr,shorewaves
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (254 samples): GPU load **95%** (min 63%) · 2572 MHz (min 2565) · memory 8001 MHz · 93.4 W · 83 °C (max 86) · VRAM 5443 MiB · throttled 100% of samples
