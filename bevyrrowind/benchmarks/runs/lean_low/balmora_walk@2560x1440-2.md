# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6488 frames over 65.6 s · **median 9.93 ms (101 fps)** · mean 10.12 · p95 12.03 · p99 13.35 (1% low 75 fps) · worst 28.57 ms · 17 spikes over 1.5× median · 12.8 stutters/min (work 16.5/min)
- warm-up: 493 frames over 8.6 s, worst 2598.8 ms
- hitches (> 25.0 ms): **1** measured, 21 in warm-up
- timed GPU passes: 5.64 ms of a 10.12 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.93 ms, limit 11 ms
- **FAIL** frame_ms: 14 stutters (> 1.5x moving median of 60) in 1.09 min = 12.80/min, limit 1/min
- pass worst frame 28.57 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.066 | 2.076 | 2.118 | 1.00 | 0.178 |
| main_opaque_pass_3d | 1.001 | 1.006 | 1.399 | 1.00 | 0.065 |
| early prepass | 0.368 | 0.369 | 0.580 | 1.00 | 0.102 |
| atmosphere_luts | 0.311 | 0.312 | 0.370 | 1.00 | 0.004 |
| water_velocity | 0.271 | 0.273 | 0.338 | 1.00 | 0.002 |
| uniform_allocation | 0.219 | 0.220 | 0.249 | 1.00 | 0.041 |
| bin_unpacking | 0.181 | 0.182 | 0.211 | 1.00 | 0.029 |
| atmos_froxel_inject | 0.148 | 0.148 | 0.151 | 1.00 | 0.002 |
| clouds_trace | 0.137 | 0.137 | 0.203 | 1.00 | 0.001 |
| upscaling | 0.122 | 0.123 | 0.140 | 1.00 | 0.004 |
| tonemapping | 0.116 | 0.116 | 0.121 | 1.00 | 0.005 |
| clouds_composite | 0.107 | 0.108 | 0.110 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.072 | 0.072 | 0.119 | 1.00 | 0.008 |
| clustering | 0.065 | 0.066 | 0.069 | 1.00 | 0.022 |
| render_sky | 0.053 | 0.053 | 0.053 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.419 | 0.421 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.038 | 0.057 | 0.057 | 0.67 | 0.003 |
| water_fft | 0.036 | 0.036 | 0.039 | 1.00 | 0.003 |
| early_mesh_preprocessing | 0.032 | 0.032 | 0.035 | 1.00 | 0.006 |
| water_snapshot | 0.031 | 0.031 | 0.034 | 1.00 | 0.002 |
| clouds_shadow | 0.030 | 0.030 | 0.033 | 1.00 | 0.004 |
| atmos_froxel_integrate | 0.027 | 0.028 | 0.029 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.224 | 0.233 | 0.12 | 0.005 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 1.00 | 0.007 |
| water_mips | 0.017 | 0.017 | 0.018 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.011 | 0.026 | 1.00 | 0.069 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.010 | 1.00 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 1.00 | 0.004 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.191 | 11.008 | 16.19 | 59634.2 |
| pipelines/wait_ms | 8.833 | 0.000 | 7325.63 | 57306.5 |
| render/busy/ms | 8.644 | 10.380 | 15.64 | 56079.8 |
| main/busy/ms | 7.291 | 9.100 | 22.11 | 47304.0 |
| render/graph/ms | 4.973 | 6.135 | 8.27 | 32264.0 |
| bigspace/total/ms | 0.485 | 0.917 | 3.82 | 3144.1 |
| render/present/ms | 0.457 | 0.567 | 2.93 | 2965.5 |
| terrain/update/ms | 0.303 | 0.394 | 2.14 | 1965.8 |
| pipelines/compile_ms | 0.273 | 0.000 | 163.25 | 1771.2 |
| terrain/select/ms | 0.269 | 0.347 | 0.79 | 1745.4 |
| pipelines/front_ms | 0.262 | 0.000 | 154.95 | 1700.9 |
| stream/drive/ms | 0.251 | 0.370 | 12.35 | 1630.9 |
| controller/feed/ms | 0.227 | 0.306 | 0.57 | 1474.6 |
| bigspace/low_precision/ms | 0.220 | 0.504 | 3.41 | 1426.4 |
| stream/tick/ms | 0.200 | 0.297 | 0.62 | 1297.4 |
| anim/evaluate_cpu/ms | 0.167 | 0.218 | 0.52 | 1082.6 |
| anim/evaluate/ms | 0.150 | 0.280 | 3.34 | 976.4 |
| controller/move/ms | 0.131 | 0.293 | 0.84 | 847.9 |
| bigspace/high_precision/ms | 0.131 | 0.314 | 3.15 | 847.7 |
| far/select/ms | 0.113 | 0.166 | 0.45 | 736.1 |
| render/acquire/ms | 0.091 | 0.127 | 0.34 | 588.9 |
| physics/step/ms | 0.045 | 0.092 | 0.64 | 292.4 |
| physics/insert/ms | 0.038 | 0.380 | 0.96 | 247.8 |
| weather/map/ms | 0.027 | 0.038 | 0.27 | 175.7 |
| weather/eval/ms | 0.011 | 0.016 | 0.06 | 71.6 |
| pipelines/driver_ms | 0.011 | 0.000 | 8.30 | 70.2 |
| anim/apply/ms | 0.011 | 0.019 | 0.20 | 69.2 |
| controller/avoid/ms | 0.010 | 0.014 | 0.14 | 67.3 |
| script/update/ms | 0.008 | 0.012 | 0.16 | 51.2 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 33.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.20 | 30.6 |
| script/ui/ms | 0.004 | 0.006 | 0.10 | 27.8 |
| controller/write/ms | 0.003 | 0.004 | 0.12 | 22.4 |
| stream/spawn/ms | 0.003 | 0.000 | 1.11 | 22.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.70 | 14.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.13 | 7.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.14 | 5.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 333 | 344 | 338 |
| anim/characters/count | count | 335 | 344 | 338 |
| anim/evaluated/count | count | 19 | 32 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1277 | 2262 | 657 |
| crowd/c0/meshes/count | count | 352 | 646 | 178 |
| crowd/c1/count | count | 99 | 111 | 99 |
| crowd/c1/joints/count | count | 3981 | 4373 | 3916 |
| crowd/c1/meshes/count | count | 898 | 1005 | 874 |
| crowd/c2/count | count | 198 | 225 | 225 |
| crowd/c2/joints/count | count | 2652 | 3431 | 3431 |
| crowd/c2/meshes/count | count | 699 | 929 | 929 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 111 | 154 | 141 |
| crowd/joints_extracted/count | count | 2693 | 9135 | 4058 |
| crowd/joints_uploaded/count | count | 2673 | 4558 | 4058 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 39.7 k | 47.4 k | 39.2 k |
| far/entities | count | 1724 | 1724 | 1724 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 54.2 k | 54.2 k | 54.2 k |
| far/lights | count | 6362 | 6362 | 6362 |
| far/nodes/l1 | count | 685 | 685 | 685 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 160.39 M | 160.39 M | 160.39 M |
| far/triangles | count | 3.88 M | 3.88 M | 3.88 M |
| frame/work/ms | ms | 8.52 | 22.11 | 7.32 |
| gpu/timed/ms | ms | 6.18 | 7.64 | 6.71 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 10.2 k | 11.1 k | 10.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 447.37 M | 450.81 M | 450.81 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 169 | 260 | 236 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 466 | 872 | 324 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 169 | 260 | 236 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 466 | 872 | 324 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 24 | 31 | 24 |
| phases/opaque3d/bins | count | 865 | 1025 | 922 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 2003 | 2657 | 1955 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 22 | 23 | 23 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 24 | 31 | 24 |
| phases/opaque3d_prepass/bins | count | 865 | 1025 | 922 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 2003 | 2657 | 1955 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 22 | 23 | 23 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 50 | 58 | 40 |
| phases/shadow/bins | count | 1940 | 2192 | 1695 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 4884 | 5934 | 3675 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 50 | 58 | 40 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2679 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 363 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 188 | 195 | 195 |
| pipelines/waiting | count | 0 | 16 | 0 |
| preprocess/bind_groups | count | 7 | 285 | 7 |
| preprocess/bind_groups_created | count | 7 | 285 | 7 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 234 | 290 | 224 |
| process/cpu_usage | % | 23.21 | 38.29 | 20.87 |
| process/mem_usage | GiB | 3.61 | 3.67 | 3.67 |
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
| started_load_count |  | 3780 | 3848 | 3848 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 4 | 3 |
| statics/material_pipelines | count | 3 | 4 | 3 |
| statics/materials | count | 395 | 493 | 345 |
| statics/materials_bound | count | 395 | 493 | 345 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 48 | 0 |
| stream/models | count | 287 | 376 | 201 |
| stream/oldest_request/s | s | 0 | 0.23 | 0 |
| stream/page_cache/bytes | bytes | 15.23 M | 15.23 M | 15.23 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 701 | 708 | 697 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 36.28 | 64.29 | 30.30 |
| system/mem_usage | % | 70.36 | 70.75 | 70.56 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 1565 | 1584 | 1581 |
| textures/images | count | 2129 | 2161 | 2156 |
| views/all/batch_sets | count | 111 | 134 | 104 |
| views/all/bins | count | 4022 | 4369 | 4011 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 10.0 k | 12.2 k | 8233 |
| views/all/pipelines | count | 104 | 118 | 98 |
| views/all/pipelines_distinct | count | 81 | 83 | 81 |
| views/all/work_items | count | 10.1 k | 12.4 k | 8412 |
| views/cascade/batch_sets | count | 50 | 58 | 40 |
| views/cascade/bins | count | 1940 | 2192 | 1695 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 4884 | 5934 | 3675 |
| views/cascade/pipelines | count | 50 | 58 | 40 |
| views/cascade/work_items | count | 4884 | 5934 | 3675 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 64 | 78 | 64 |
| views/main/bins | count | 2156 | 2378 | 2316 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 4900 | 6680 | 4558 |
| views/main/pipelines | count | 58 | 60 | 58 |
| views/main/work_items | count | 5022 | 6831 | 4737 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 22.4 k | 27.7 k | 21.1 k |
| world/point_lights | count | 189 | 251 | 182 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 53.03 | 233 | 344.0 k |
| anim/events/count | count | 0.08 | 3 | 551 |
| assets/Image/added | count | 0.08 | 44 | 505 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.51 | 139 | 3323 |
| assets/Mesh/removed | count | 0.76 | 1250 | 4924 |
| assets/StandardMaterial/added | count | 0.07 | 45 | 466 |
| assets/StandardMaterial/modified | count | 1.79 | 6 | 11.6 k |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 29 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 2 | 30 |
| ecs/entities_delta | count | 0.40 | 1492 | 2588 |
| physics/inserts | count | 0.39 | 111 | 2553 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 161 | 2335 |
| physics/steps | count | 0.61 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 11 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 21 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 75 |
| stream/spawn/items | count | 0.05 | 21 | 340 |
| stream/upload/bytes | bytes | 4529 | 2.83 M | 29.38 M |
| stream/upload/items | count | 0.01 | 1 | 49 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1144 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 22, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.60 | 2598.8 | main/busy/ms 2574, surface/advance_cpu_ms 68.83, surface/cell_upload_ms 0.78, stream/drive/ms 0.31, weather/map/ms 0.14, stream/tick/ms 0.10, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.03, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.05 | 451.6 | main/busy/ms 2615, render/thread/ms 417, render/busy/ms 316, render/acquire/ms 98.21, surface/advance_cpu_ms 68.83, render/graph/ms 4.78, render/present/ms 3.20, stream/drive/ms 0.97, surface/cell_upload_ms 0.78, terrain/update/ms 0.67, stream/tick/ms 0.49, terrain/select/ms 0.47 |
| warm-up | 3.23 | 176.1 | pipelines/wait_ms 1309, render/thread/ms 432, render/busy/ms 330, pipelines/compile_ms 187, pipelines/front_ms 174, render/acquire/ms 98.50, main/busy/ms 61.05, terrain/update/ms 21.93, pipelines/driver_ms 13.68, render/graph/ms 9.27, render/present/ms 4.28, stream/drive/ms 1.38 |
| warm-up | 3.32 | 25.5 | render/thread/ms 42.15, render/acquire/ms 27.03, main/busy/ms 19.61, render/busy/ms 13.81, render/graph/ms 3.06, render/present/ms 1.31, stream/drive/ms 0.88, terrain/update/ms 0.81, stream/tick/ms 0.73, terrain/select/ms 0.62, bigspace/total/ms 0.25, bigspace/high_precision/ms 0.16 |
| warm-up | 3.37 | 25.6 | main/busy/ms 73.44, render/thread/ms 45.34, render/busy/ms 37.70, terrain/update/ms 6.61, render/acquire/ms 5.64, render/graph/ms 5.41, stream/drive/ms 3.62, stream/tick/ms 2.68, render/present/ms 2.00, terrain/select/ms 0.71, bigspace/total/ms 0.23, weather/map/ms 0.16 |
| warm-up | 3.50 | 126.0 | pipelines/compile_ms 266, pipelines/front_ms 238, pipelines/wait_ms 172, render/thread/ms 110, render/busy/ms 107, main/busy/ms 85.60, pipelines/driver_ms 27.44, render/graph/ms 6.55, stream/drive/ms 3.67, stream/tick/ms 2.73, render/present/ms 2.43, bigspace/total/ms 1.01 |
| warm-up | 3.54 | 27.4 | render/thread/ms 40.58, render/busy/ms 39.14, main/busy/ms 21.54, render/graph/ms 3.47, stream/drive/ms 2.17, render/present/ms 1.20, stream/tick/ms 1.11, terrain/update/ms 0.95, terrain/select/ms 0.78, stream/spawn/ms 0.70, bigspace/total/ms 0.34, render/acquire/ms 0.24 |
| warm-up | 3.68 | 82.5 | pipelines/wait_ms 1462, pipelines/compile_ms 222, pipelines/front_ms 209, render/thread/ms 59.68, render/busy/ms 58.27, main/busy/ms 56.60, stream/drive/ms 42.11, pipelines/driver_ms 13.64, render/graph/ms 4.20, render/present/ms 1.23, stream/tick/ms 1.13, stream/spawn/ms 0.67 |
| warm-up | 3.75 | 49.6 | pipelines/wait_ms 4023, pipelines/compile_ms 90.20, pipelines/front_ms 81.47, render/thread/ms 48.98, render/busy/ms 47.12, main/busy/ms 44.58, stream/drive/ms 11.49, pipelines/driver_ms 8.73, render/graph/ms 4.77, render/present/ms 1.58, terrain/update/ms 1.09, stream/tick/ms 0.92 |
| warm-up | 3.78 | 25.8 | pipelines/wait_ms 3150, pipelines/compile_ms 56.35, render/thread/ms 54.24, main/busy/ms 53.08, render/busy/ms 52.20, pipelines/front_ms 52.17, stream/drive/ms 18.40, terrain/update/ms 6.55, render/graph/ms 4.89, pipelines/driver_ms 4.18, stream/spawn/ms 1.85, render/present/ms 1.73 |
| warm-up | 3.82 | 41.1 | render/thread/ms 50.72, render/busy/ms 49.20, main/busy/ms 39.53, stream/drive/ms 19.45, terrain/update/ms 6.28, render/graph/ms 3.77, stream/spawn/ms 2.01, render/present/ms 1.27, stream/tick/ms 0.89, terrain/select/ms 0.47, bigspace/total/ms 0.31, stream/activate/ms 0.29 |
| warm-up | 3.85 | 32.6 | pipelines/wait_ms 513, pipelines/compile_ms 77.63, pipelines/front_ms 74.51, render/thread/ms 58.13, render/busy/ms 56.96, main/busy/ms 24.16, stream/drive/ms 4.69, render/graph/ms 3.74, pipelines/driver_ms 3.13, stream/spawn/ms 1.79, stream/tick/ms 1.16, render/present/ms 0.95 |
| warm-up | 3.91 | 38.4 | main/busy/ms 41.96, render/thread/ms 31.47, render/busy/ms 30.36, stream/drive/ms 28.34, render/graph/ms 2.58, stream/tick/ms 1.15, stream/activate/ms 1.04, terrain/update/ms 0.94, render/present/ms 0.89, terrain/select/ms 0.79, bigspace/total/ms 0.28, physics/insert/ms 0.28 |
| warm-up | 4.09 | 94.7 | pipelines/wait_ms 2284, pipelines/compile_ms 103, pipelines/front_ms 98.22, main/busy/ms 68.25, stream/drive/ms 55.00, render/thread/ms 49.93, render/busy/ms 48.21, pipelines/driver_ms 4.96, render/graph/ms 3.97, render/present/ms 1.48, stream/tick/ms 1.39, terrain/update/ms 0.75 |
| warm-up | 4.15 | 56.4 | main/busy/ms 59.85, stream/drive/ms 43.09, render/thread/ms 19.48, render/busy/ms 18.34, render/graph/ms 3.65, stream/spawn/ms 2.81, render/present/ms 0.98, terrain/update/ms 0.85, terrain/select/ms 0.71, stream/tick/ms 0.57, bigspace/total/ms 0.37, render/acquire/ms 0.16 |
| warm-up | 4.21 | 40.2 | pipelines/wait_ms 3964, pipelines/compile_ms 145, pipelines/front_ms 140, main/busy/ms 51.54, render/thread/ms 30.70, stream/drive/ms 30.49, render/busy/ms 28.87, pipelines/driver_ms 5.14, render/graph/ms 3.67, stream/spawn/ms 2.18, render/present/ms 1.52, stream/tick/ms 0.67 |
| warm-up | 4.25 | 35.3 | main/busy/ms 39.86, stream/drive/ms 28.95, render/thread/ms 15.20, render/busy/ms 14.14, render/graph/ms 3.24, stream/activate/ms 1.15, render/present/ms 0.87, stream/tick/ms 0.65, terrain/update/ms 0.58, terrain/select/ms 0.48, physics/insert/ms 0.41, stream/spawn/ms 0.30 |
| warm-up | 4.91 | 31.8 | render/thread/ms 38.58, render/busy/ms 37.20, main/busy/ms 20.24, render/graph/ms 5.14, stream/drive/ms 3.59, stream/spawn/ms 1.52, render/present/ms 1.18, bigspace/total/ms 1.00, stream/tick/ms 0.74, bigspace/high_precision/ms 0.69, terrain/update/ms 0.68, terrain/select/ms 0.57 |
| warm-up | 5.05 | 32.0 | main/busy/ms 33.89, render/thread/ms 26.50, render/busy/ms 25.37, stream/drive/ms 16.91, render/graph/ms 5.38, stream/spawn/ms 2.22, render/present/ms 0.93, bigspace/total/ms 0.82, terrain/update/ms 0.73, terrain/select/ms 0.62, stream/tick/ms 0.60, bigspace/high_precision/ms 0.49 |
| warm-up | 5.89 | 41.5 | main/busy/ms 48.07, render/thread/ms 38.91, render/busy/ms 37.40, render/graph/ms 6.91, stream/drive/ms 1.39, render/present/ms 1.27, bigspace/total/ms 1.13, anim/evaluate_cpu/ms 0.97, terrain/update/ms 0.80, terrain/select/ms 0.66, physics/insert/ms 0.65, stream/tick/ms 0.59 |
| warm-up | 6.62 | 30.5 | pipelines/wait_ms 4281, pipelines/compile_ms 151, pipelines/front_ms 147, main/busy/ms 34.64, render/thread/ms 26.20, render/busy/ms 24.95, render/graph/ms 7.84, pipelines/driver_ms 3.55, render/present/ms 1.03, physics/step/ms 0.90, physics/insert/ms 0.68, terrain/update/ms 0.63 |
| measure | 24.01 | 28.6 | main/busy/ms 32.17, render/thread/ms 29.28, render/busy/ms 27.87, render/graph/ms 11.23, stream/drive/ms 1.33, render/present/ms 1.18, bigspace/total/ms 1.16, physics/insert/ms 0.87, stream/tick/ms 0.73, terrain/update/ms 0.69, terrain/select/ms 0.58, controller/feed/ms 0.49 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_low\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_QUALITY=low
- before the run: GPU 0%, CPU 14%
- during the run: other processes 13% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **74%** (min 56%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.7 W · 75 °C (max 80) · VRAM 4461 MiB · throttled 100% of samples
