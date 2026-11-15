# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6440 frames over 65.6 s · **median 9.98 ms (100 fps)** · mean 10.19 · p95 12.23 · p99 13.66 (1% low 73 fps) · worst 24.67 ms · 11 spikes over 1.5× median · 8.2 stutters/min (work 11.0/min)
- warm-up: 1094 frames over 14.8 s, worst 2591.4 ms
- hitches (> 25.0 ms): **0** measured, 18 in warm-up
- timed GPU passes: 5.65 ms of a 10.19 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.98 ms, limit 11 ms
- **FAIL** frame_ms: 9 stutters (> 1.5x moving median of 60) in 1.09 min = 8.23/min, limit 1/min
- pass worst frame 24.67 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.069 | 2.081 | 2.200 | 0.99 | 0.184 |
| main_opaque_pass_3d | 1.019 | 1.025 | 1.396 | 0.99 | 0.061 |
| early prepass | 0.364 | 0.366 | 0.565 | 0.99 | 0.102 |
| atmosphere_luts | 0.312 | 0.314 | 0.371 | 0.99 | 0.004 |
| water_velocity | 0.260 | 0.261 | 0.338 | 0.99 | 0.002 |
| uniform_allocation | 0.218 | 0.219 | 0.247 | 0.99 | 0.041 |
| bin_unpacking | 0.183 | 0.184 | 0.206 | 0.99 | 0.029 |
| atmos_froxel_inject | 0.148 | 0.149 | 0.151 | 0.99 | 0.002 |
| clouds_trace | 0.137 | 0.138 | 0.199 | 0.99 | 0.001 |
| upscaling | 0.122 | 0.122 | 0.133 | 0.99 | 0.004 |
| tonemapping | 0.118 | 0.118 | 0.123 | 0.99 | 0.004 |
| clouds_composite | 0.107 | 0.108 | 0.110 | 0.99 | 0.002 |
| main_transmissive_pass_3d | 0.071 | 0.071 | 0.119 | 0.99 | 0.009 |
| clustering | 0.065 | 0.066 | 0.069 | 0.99 | 0.022 |
| render_sky | 0.052 | 0.053 | 0.053 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.423 | 0.422 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 0.99 | 0.001 |
| surface_trail | 0.038 | 0.057 | 0.057 | 0.68 | 0.003 |
| water_fft | 0.036 | 0.036 | 0.039 | 0.99 | 0.003 |
| early_mesh_preprocessing | 0.032 | 0.032 | 0.035 | 0.99 | 0.006 |
| water_snapshot | 0.031 | 0.031 | 0.034 | 0.99 | 0.002 |
| clouds_shadow | 0.030 | 0.030 | 0.033 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.030 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.225 | 0.234 | 0.12 | 0.005 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.023 | 0.99 | 0.007 |
| water_mips | 0.017 | 0.017 | 0.018 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.026 | 0.99 | 0.069 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.010 | 0.99 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.99 | 0.003 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 66.227 | 0.000 | 42061.49 | 426502.6 |
| render/thread/ms | 9.267 | 11.187 | 17.99 | 59682.2 |
| render/busy/ms | 8.711 | 10.540 | 17.33 | 56101.6 |
| main/busy/ms | 7.255 | 9.184 | 20.92 | 46722.9 |
| render/graph/ms | 5.067 | 6.305 | 9.35 | 32629.7 |
| pipelines/compile_ms | 4.070 | 0.000 | 2499.92 | 26209.9 |
| pipelines/driver_ms | 3.645 | 0.000 | 2356.41 | 23476.3 |
| render/present/ms | 0.465 | 0.577 | 2.20 | 2993.1 |
| bigspace/total/ms | 0.446 | 0.844 | 4.03 | 2872.5 |
| pipelines/front_ms | 0.424 | 0.000 | 143.51 | 2733.6 |
| terrain/update/ms | 0.306 | 0.399 | 1.60 | 1967.5 |
| terrain/select/ms | 0.271 | 0.353 | 0.77 | 1748.2 |
| stream/drive/ms | 0.256 | 0.384 | 13.98 | 1646.9 |
| controller/feed/ms | 0.223 | 0.306 | 0.57 | 1433.3 |
| stream/tick/ms | 0.204 | 0.310 | 0.58 | 1312.2 |
| bigspace/low_precision/ms | 0.183 | 0.407 | 3.72 | 1178.3 |
| anim/evaluate_cpu/ms | 0.157 | 0.208 | 0.55 | 1008.6 |
| controller/move/ms | 0.132 | 0.254 | 0.58 | 850.8 |
| anim/evaluate/ms | 0.132 | 0.254 | 2.70 | 847.3 |
| bigspace/high_precision/ms | 0.129 | 0.301 | 3.31 | 833.3 |
| far/select/ms | 0.115 | 0.167 | 0.45 | 741.1 |
| render/acquire/ms | 0.091 | 0.128 | 0.41 | 587.5 |
| physics/step/ms | 0.045 | 0.091 | 1.04 | 289.7 |
| physics/insert/ms | 0.037 | 0.367 | 0.98 | 238.2 |
| weather/map/ms | 0.027 | 0.039 | 0.25 | 172.8 |
| weather/eval/ms | 0.011 | 0.016 | 0.31 | 72.1 |
| controller/avoid/ms | 0.010 | 0.013 | 0.15 | 63.2 |
| anim/apply/ms | 0.010 | 0.019 | 0.20 | 62.0 |
| script/update/ms | 0.008 | 0.012 | 0.23 | 50.2 |
| script/input/ms | 0.005 | 0.007 | 0.13 | 32.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.26 | 31.3 |
| script/ui/ms | 0.004 | 0.007 | 0.03 | 27.4 |
| stream/spawn/ms | 0.004 | 0.000 | 1.11 | 23.1 |
| controller/write/ms | 0.003 | 0.004 | 0.11 | 21.7 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 14.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.53 | 6.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 5.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 334 | 344 | 338 |
| anim/characters/count | count | 335 | 344 | 338 |
| anim/evaluated/count | count | 19 | 31 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1249 | 2262 | 601 |
| crowd/c0/meshes/count | count | 349 | 646 | 159 |
| crowd/c1/count | count | 99 | 111 | 101 |
| crowd/c1/joints/count | count | 3959 | 4364 | 4004 |
| crowd/c1/meshes/count | count | 892 | 994 | 901 |
| crowd/c2/count | count | 203 | 224 | 224 |
| crowd/c2/joints/count | count | 2857 | 3399 | 3399 |
| crowd/c2/meshes/count | count | 758 | 921 | 921 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 111 | 154 | 141 |
| crowd/joints_extracted/count | count | 2673 | 9081 | 4058 |
| crowd/joints_uploaded/count | count | 2662 | 4558 | 4058 |
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
| frame/work/ms | ms | 8.56 | 20.92 | 8.19 |
| gpu/timed/ms | ms | 6.23 | 7.94 | 6.71 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 10.2 k | 11.1 k | 10.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 450.81 M | 450.81 M | 450.81 M |
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
| phases/opaque3d/bins | count | 857 | 1032 | 922 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 2002 | 2696 | 1955 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 22 | 24 | 23 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 24 | 31 | 24 |
| phases/opaque3d_prepass/bins | count | 857 | 1032 | 922 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 2002 | 2696 | 1955 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 50 | 59 | 40 |
| phases/shadow/bins | count | 1934 | 2202 | 1695 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 4877 | 5906 | 3675 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 50 | 57 | 40 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 1370 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 188 | 195 | 195 |
| pipelines/waiting | count | 0 | 36 | 0 |
| preprocess/bind_groups | count | 7 | 277 | 7 |
| preprocess/bind_groups_created | count | 7 | 277 | 7 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 233 | 282 | 224 |
| process/cpu_usage | % | 23.90 | 47.55 | 23.42 |
| process/mem_usage | GiB | 3.73 | 3.82 | 3.82 |
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
| started_load_count |  | 3832 | 3847 | 3847 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 4 | 3 |
| statics/material_pipelines | count | 3 | 4 | 3 |
| statics/materials | count | 395 | 493 | 345 |
| statics/materials_bound | count | 395 | 493 | 345 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 48 | 0 |
| stream/models | count | 287 | 376 | 201 |
| stream/oldest_request/s | s | 0 | 0.22 | 0 |
| stream/page_cache/bytes | bytes | 15.23 M | 15.23 M | 15.23 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 701 | 708 | 697 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 36.26 | 67.75 | 32.07 |
| system/mem_usage | % | 70.70 | 71.18 | 70.97 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 1565 | 1584 | 1581 |
| textures/images | count | 2129 | 2161 | 2156 |
| views/all/batch_sets | count | 110 | 132 | 104 |
| views/all/bins | count | 4007 | 4403 | 4011 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 10.0 k | 12.2 k | 8233 |
| views/all/pipelines | count | 104 | 118 | 98 |
| views/all/pipelines_distinct | count | 81 | 85 | 81 |
| views/all/work_items | count | 10.1 k | 12.3 k | 8412 |
| views/cascade/batch_sets | count | 50 | 59 | 40 |
| views/cascade/bins | count | 1934 | 2202 | 1695 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 4877 | 5906 | 3675 |
| views/cascade/pipelines | count | 50 | 57 | 40 |
| views/cascade/work_items | count | 4877 | 5906 | 3675 |
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
| views/main/bins | count | 2160 | 2378 | 2316 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 4892 | 6758 | 4558 |
| views/main/pipelines | count | 58 | 62 | 58 |
| views/main/work_items | count | 5018 | 6909 | 4737 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 22.3 k | 27.7 k | 21.1 k |
| world/point_lights | count | 189 | 251 | 182 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 47.87 | 235 | 308.3 k |
| anim/events/count | count | 0.08 | 3 | 529 |
| assets/Image/added | count | 0.08 | 43 | 505 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.52 | 139 | 3319 |
| assets/Mesh/removed | count | 0.76 | 1250 | 4924 |
| assets/StandardMaterial/added | count | 0.07 | 45 | 464 |
| assets/StandardMaterial/modified | count | 1.80 | 6 | 11.6 k |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 29 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 72.94 | 1944 | 469.7 k |
| ecs/entities_delta | count | 0.47 | 1485 | 3027 |
| physics/inserts | count | 0.56 | 118 | 3614 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 129 | 2335 |
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
| stream/spawn/items | count | 0.05 | 20 | 340 |
| stream/upload/bytes | bytes | 4562 | 2.83 M | 29.38 M |
| stream/upload/items | count | 0.01 | 1 | 49 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1152 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 18, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.59 | 2591.4 | main/busy/ms 2565, surface/advance_cpu_ms 69.83, surface/cell_upload_ms 0.78, stream/drive/ms 0.46, weather/map/ms 0.14, stream/tick/ms 0.13, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.03, anim/evaluate/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.29 | 697.5 | main/busy/ms 2608, render/thread/ms 658, render/busy/ms 566, render/acquire/ms 90.22, surface/advance_cpu_ms 69.83, render/graph/ms 6.08, render/present/ms 2.14, stream/drive/ms 1.26, surface/cell_upload_ms 0.78, stream/tick/ms 0.54, terrain/update/ms 0.44, bigspace/total/ms 0.36 |
| warm-up | 3.56 | 88.6 | main/busy/ms 70.52, render/thread/ms 56.90, render/busy/ms 54.59, render/graph/ms 5.92, stream/drive/ms 2.22, render/present/ms 1.96, stream/tick/ms 1.36, terrain/update/ms 1.08, terrain/select/ms 0.84, bigspace/total/ms 0.56, stream/spawn/ms 0.36, render/acquire/ms 0.35 |
| warm-up | 3.74 | 72.2 | main/busy/ms 53.26, render/thread/ms 50.42, render/busy/ms 48.85, stream/drive/ms 40.61, render/graph/ms 4.55, render/present/ms 1.35, stream/spawn/ms 1.24, terrain/update/ms 0.78, stream/tick/ms 0.76, terrain/select/ms 0.64, render/acquire/ms 0.22, bigspace/total/ms 0.21 |
| warm-up | 3.80 | 46.5 | main/busy/ms 34.73, render/thread/ms 29.72, render/busy/ms 28.25, stream/drive/ms 19.74, render/graph/ms 4.26, stream/spawn/ms 1.66, render/present/ms 1.23, terrain/update/ms 0.91, terrain/select/ms 0.72, stream/tick/ms 0.67, bigspace/total/ms 0.35, stream/activate/ms 0.26 |
| warm-up | 3.88 | 60.6 | render/thread/ms 65.74, render/busy/ms 63.78, main/busy/ms 40.96, stream/drive/ms 20.66, render/graph/ms 4.62, stream/tick/ms 2.93, render/present/ms 1.72, stream/spawn/ms 1.23, terrain/update/ms 0.87, stream/activate/ms 0.75, terrain/select/ms 0.70, bigspace/total/ms 0.30 |
| warm-up | 3.97 | 70.4 | main/busy/ms 51.97, render/thread/ms 43.93, render/busy/ms 42.80, stream/drive/ms 34.24, render/graph/ms 3.10, stream/spawn/ms 2.32, terrain/update/ms 1.09, stream/tick/ms 1.06, render/present/ms 0.93, terrain/select/ms 0.88, stream/activate/ms 0.57, bigspace/total/ms 0.39 |
| warm-up | 4.14 | 75.1 | main/busy/ms 64.63, stream/drive/ms 45.42, render/thread/ms 33.56, render/busy/ms 31.54, render/graph/ms 3.88, render/present/ms 1.76, stream/spawn/ms 1.52, stream/tick/ms 0.87, terrain/update/ms 0.83, terrain/select/ms 0.70, bigspace/total/ms 0.32, stream/activate/ms 0.27 |
| warm-up | 4.22 | 61.1 | main/busy/ms 67.18, stream/drive/ms 47.33, render/thread/ms 25.88, render/busy/ms 24.26, render/graph/ms 4.46, stream/spawn/ms 2.13, render/present/ms 1.40, terrain/update/ms 0.76, stream/tick/ms 0.70, terrain/select/ms 0.60, stream/activate/ms 0.53, bigspace/total/ms 0.47 |
| warm-up | 4.28 | 41.7 | main/busy/ms 48.65, stream/drive/ms 35.62, render/thread/ms 29.30, render/busy/ms 27.82, render/graph/ms 3.09, stream/spawn/ms 1.70, render/present/ms 1.17, stream/tick/ms 0.76, stream/activate/ms 0.63, terrain/update/ms 0.63, terrain/select/ms 0.53, physics/insert/ms 0.33 |
| warm-up | 4.32 | 31.0 | main/busy/ms 37.35, stream/drive/ms 24.65, render/thread/ms 16.49, render/busy/ms 15.04, render/graph/ms 3.82, render/present/ms 1.21, stream/activate/ms 1.03, bigspace/total/ms 0.73, terrain/update/ms 0.67, physics/insert/ms 0.67, stream/tick/ms 0.63, terrain/select/ms 0.56 |
| warm-up | 4.81 | 31.7 | pipelines/wait_ms 7324, main/busy/ms 43.62, pipelines/compile_ms 36.52, pipelines/front_ms 34.08, render/thread/ms 28.47, render/busy/ms 26.81, render/graph/ms 5.81, stream/drive/ms 3.05, pipelines/driver_ms 2.45, stream/spawn/ms 1.70, render/present/ms 1.36, bigspace/total/ms 1.08 |
| warm-up | 4.84 | 26.4 | pipelines/wait_ms 7361, pipelines/compile_ms 46.52, pipelines/front_ms 44.30, render/thread/ms 34.00, main/busy/ms 33.39, render/busy/ms 32.53, render/graph/ms 5.76, pipelines/driver_ms 2.23, stream/drive/ms 2.21, render/present/ms 1.23, bigspace/total/ms 1.18, stream/tick/ms 0.71 |
| warm-up | 4.89 | 34.6 | pipelines/wait_ms 1499, pipelines/compile_ms 76.47, pipelines/front_ms 75.02, render/thread/ms 40.28, main/busy/ms 38.89, render/busy/ms 38.41, render/graph/ms 6.20, stream/drive/ms 5.97, render/present/ms 1.61, pipelines/driver_ms 1.46, terrain/update/ms 1.27, terrain/select/ms 1.03 |
| warm-up | 4.98 | 36.3 | main/busy/ms 36.88, render/thread/ms 28.84, render/busy/ms 27.40, stream/drive/ms 14.88, render/graph/ms 4.64, render/present/ms 1.24, terrain/update/ms 0.85, stream/spawn/ms 0.78, bigspace/total/ms 0.72, terrain/select/ms 0.66, stream/tick/ms 0.61, physics/insert/ms 0.61 |
| warm-up | 5.34 | 25.6 | pipelines/wait_ms 2487, render/thread/ms 46.67, render/busy/ms 45.29, pipelines/compile_ms 43.18, pipelines/front_ms 41.31, main/busy/ms 33.87, stream/drive/ms 15.66, render/graph/ms 7.98, stream/spawn/ms 2.18, pipelines/driver_ms 1.87, physics/insert/ms 1.72, render/present/ms 1.17 |
| warm-up | 7.05 | 31.1 | pipelines/wait_ms 4719, pipelines/compile_ms 816, pipelines/driver_ms 695, pipelines/front_ms 121, main/busy/ms 68.78, render/thread/ms 41.80, render/busy/ms 39.61, render/graph/ms 8.27, render/present/ms 1.89, anim/evaluate_cpu/ms 1.24, bigspace/total/ms 0.93, stream/drive/ms 0.90 |
| warm-up | 7.11 | 61.6 | pipelines/wait_ms 2749, main/busy/ms 65.67, render/thread/ms 47.40, render/busy/ms 45.36, pipelines/compile_ms 13.76, pipelines/front_ms 12.64, render/graph/ms 7.95, terrain/update/ms 4.88, render/present/ms 1.79, pipelines/driver_ms 1.12, anim/evaluate_cpu/ms 1.04, physics/insert/ms 1.04 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_low\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_QUALITY=low
- before the run: GPU 0%, CPU 11%
- during the run: other processes 12% CPU (system minus this process)
- nvidia-smi over the window (250 samples): GPU load **74%** (min 59%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.6 W · 76 °C (max 80) · VRAM 4461 MiB · throttled 100% of samples
