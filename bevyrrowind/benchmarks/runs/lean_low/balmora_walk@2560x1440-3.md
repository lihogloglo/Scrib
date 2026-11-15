# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6025 frames over 65.6 s · **median 10.69 ms (94 fps)** · mean 10.90 · p95 13.40 · p99 14.95 (1% low 67 fps) · worst 25.89 ms · 24 spikes over 1.5× median · 21.0 stutters/min (work 21.0/min)
- warm-up: 453 frames over 9.6 s, worst 2741.8 ms
- hitches (> 26.7 ms): **0** measured, 23 in warm-up
- timed GPU passes: 5.66 ms of a 10.90 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.69 ms, limit 11 ms
- **FAIL** frame_ms: 23 stutters (> 1.5x moving median of 60) in 1.09 min = 21.02/min, limit 1/min
- pass worst frame 25.89 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.075 | 2.078 | 2.113 | 1.00 | 0.194 |
| main_opaque_pass_3d | 1.004 | 1.006 | 1.398 | 1.00 | 0.068 |
| early prepass | 0.361 | 0.362 | 0.563 | 1.00 | 0.112 |
| atmosphere_luts | 0.313 | 0.314 | 0.372 | 1.00 | 0.004 |
| water_velocity | 0.271 | 0.271 | 0.338 | 1.00 | 0.002 |
| uniform_allocation | 0.220 | 0.220 | 0.248 | 1.00 | 0.044 |
| bin_unpacking | 0.178 | 0.179 | 0.202 | 1.00 | 0.031 |
| atmos_froxel_inject | 0.149 | 0.149 | 0.151 | 1.00 | 0.002 |
| clouds_trace | 0.136 | 0.137 | 0.200 | 1.00 | 0.001 |
| upscaling | 0.121 | 0.122 | 0.132 | 1.00 | 0.005 |
| tonemapping | 0.118 | 0.118 | 0.122 | 1.00 | 0.005 |
| clouds_composite | 0.108 | 0.108 | 0.110 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.072 | 0.072 | 0.120 | 1.00 | 0.009 |
| clustering | 0.065 | 0.066 | 0.069 | 1.00 | 0.024 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.418 | 0.421 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.040 | 0.057 | 0.057 | 0.71 | 0.003 |
| water_fft | 0.035 | 0.035 | 0.037 | 1.00 | 0.003 |
| early_mesh_preprocessing | 0.032 | 0.032 | 0.035 | 1.00 | 0.006 |
| water_snapshot | 0.032 | 0.032 | 0.034 | 1.00 | 0.002 |
| clouds_shadow | 0.030 | 0.030 | 0.033 | 1.00 | 0.004 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.029 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.224 | 0.234 | 0.12 | 0.005 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 1.00 | 0.007 |
| water_mips | 0.017 | 0.017 | 0.018 | 1.00 | 0.003 |
| main_transparent_pass_3d | 0.011 | 0.011 | 0.026 | 1.00 | 0.075 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.004 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 1.00 | 0.004 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 11.045 | 0.000 | 11723.47 | 66544.5 |
| render/thread/ms | 9.907 | 12.239 | 20.15 | 59690.7 |
| render/busy/ms | 9.325 | 11.530 | 19.46 | 56183.7 |
| main/busy/ms | 7.834 | 10.114 | 19.97 | 47199.9 |
| render/graph/ms | 5.400 | 6.792 | 9.23 | 32535.2 |
| bigspace/total/ms | 0.505 | 0.969 | 4.39 | 3044.7 |
| render/present/ms | 0.484 | 0.622 | 8.74 | 2914.9 |
| terrain/update/ms | 0.317 | 0.422 | 2.09 | 1909.2 |
| terrain/select/ms | 0.281 | 0.373 | 0.78 | 1692.5 |
| stream/drive/ms | 0.273 | 0.434 | 12.38 | 1647.2 |
| pipelines/compile_ms | 0.270 | 0.000 | 159.53 | 1629.3 |
| pipelines/front_ms | 0.260 | 0.000 | 154.20 | 1569.5 |
| controller/feed/ms | 0.229 | 0.321 | 0.59 | 1380.6 |
| bigspace/low_precision/ms | 0.227 | 0.507 | 3.89 | 1369.9 |
| stream/tick/ms | 0.219 | 0.341 | 0.88 | 1320.6 |
| anim/evaluate_cpu/ms | 0.187 | 0.257 | 0.68 | 1129.3 |
| anim/evaluate/ms | 0.142 | 0.276 | 4.10 | 858.0 |
| bigspace/high_precision/ms | 0.134 | 0.295 | 3.08 | 806.9 |
| far/select/ms | 0.121 | 0.184 | 0.42 | 731.1 |
| controller/move/ms | 0.120 | 0.218 | 0.48 | 720.1 |
| render/acquire/ms | 0.098 | 0.145 | 0.36 | 592.1 |
| physics/step/ms | 0.053 | 0.110 | 0.72 | 317.7 |
| physics/insert/ms | 0.035 | 0.368 | 0.87 | 209.5 |
| weather/map/ms | 0.029 | 0.043 | 0.37 | 177.4 |
| weather/eval/ms | 0.012 | 0.018 | 0.06 | 73.4 |
| anim/apply/ms | 0.011 | 0.020 | 0.24 | 68.5 |
| controller/avoid/ms | 0.011 | 0.014 | 0.34 | 63.7 |
| pipelines/driver_ms | 0.010 | 0.000 | 5.33 | 59.8 |
| script/update/ms | 0.008 | 0.014 | 0.22 | 50.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.14 | 32.4 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 31.4 |
| script/ui/ms | 0.005 | 0.008 | 0.24 | 29.9 |
| controller/write/ms | 0.004 | 0.005 | 0.22 | 22.5 |
| stream/spawn/ms | 0.004 | 0.000 | 1.06 | 22.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.65 | 15.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 6.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 333 | 344 | 338 |
| anim/characters/count | count | 335 | 344 | 338 |
| anim/evaluated/count | count | 19 | 31 | 20 |
| controller/movers/count | count | 20 | 21 | 20 |
| crowd/c0/count | count | 27 | 43 | 11 |
| crowd/c0/joints/count | count | 1301 | 2262 | 489 |
| crowd/c0/meshes/count | count | 353 | 646 | 128 |
| crowd/c1/count | count | 99 | 111 | 103 |
| crowd/c1/joints/count | count | 3981 | 4427 | 4116 |
| crowd/c1/meshes/count | count | 898 | 1019 | 932 |
| crowd/c2/count | count | 198 | 224 | 224 |
| crowd/c2/joints/count | count | 2652 | 3399 | 3399 |
| crowd/c2/meshes/count | count | 699 | 921 | 921 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 111 | 155 | 141 |
| crowd/joints_extracted/count | count | 2680 | 9171 | 4058 |
| crowd/joints_uploaded/count | count | 2664 | 4592 | 4058 |
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
| frame/work/ms | ms | 9.17 | 19.97 | 7.53 |
| gpu/timed/ms | ms | 6.20 | 7.71 | 6.96 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 10.2 k | 11.1 k | 10.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 447.37 M | 450.81 M | 450.81 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 168 | 260 | 236 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 467 | 872 | 324 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 168 | 260 | 236 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 467 | 872 | 324 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 24 | 31 | 24 |
| phases/opaque3d/bins | count | 861 | 1039 | 922 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 2015 | 2672 | 1955 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 22 | 24 | 23 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 24 | 31 | 24 |
| phases/opaque3d_prepass/bins | count | 861 | 1039 | 922 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 2015 | 2672 | 1955 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 51 | 59 | 40 |
| phases/shadow/bins | count | 1952 | 2204 | 1707 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 4903 | 5929 | 3687 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 51 | 56 | 40 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2705 | 3268 | 2349 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 312 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 188 | 195 | 195 |
| pipelines/waiting | count | 0 | 18 | 0 |
| preprocess/bind_groups | count | 8 | 273 | 7 |
| preprocess/bind_groups_created | count | 8 | 273 | 7 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 234 | 280 | 224 |
| process/cpu_usage | % | 23.38 | 36.54 | 21.63 |
| process/mem_usage | GiB | 3.62 | 3.67 | 3.67 |
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
| started_load_count |  | 3779 | 3847 | 3847 |
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
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 48 | 0 |
| stream/models | count | 287 | 376 | 201 |
| stream/oldest_request/s | s | 0 | 0.24 | 0 |
| stream/page_cache/bytes | bytes | 15.23 M | 15.23 M | 15.23 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 701 | 708 | 697 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 43.03 | 71.05 | 39.85 |
| system/mem_usage | % | 70.66 | 71.38 | 70.59 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 1565 | 1584 | 1581 |
| textures/images | count | 2129 | 2161 | 2156 |
| views/all/batch_sets | count | 111 | 130 | 104 |
| views/all/bins | count | 4014 | 4394 | 4023 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 10.1 k | 12.2 k | 8245 |
| views/all/pipelines | count | 104 | 116 | 98 |
| views/all/pipelines_distinct | count | 81 | 85 | 81 |
| views/all/work_items | count | 10.2 k | 12.3 k | 8424 |
| views/cascade/batch_sets | count | 51 | 59 | 40 |
| views/cascade/bins | count | 1952 | 2204 | 1707 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 4903 | 5929 | 3687 |
| views/cascade/pipelines | count | 51 | 56 | 40 |
| views/cascade/work_items | count | 4903 | 5929 | 3687 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 64 | 76 | 64 |
| views/main/bins | count | 2168 | 2378 | 2316 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 4930 | 6710 | 4558 |
| views/main/pipelines | count | 58 | 62 | 58 |
| views/main/work_items | count | 5056 | 6861 | 4737 |
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
| anim/bones_written/count | count | 53.45 | 230 | 322.1 k |
| anim/events/count | count | 0.09 | 3 | 556 |
| assets/Image/added | count | 0.08 | 43 | 501 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.55 | 139 | 3302 |
| assets/Mesh/removed | count | 0.82 | 1250 | 4924 |
| assets/StandardMaterial/added | count | 0.08 | 45 | 460 |
| assets/StandardMaterial/modified | count | 1.93 | 6 | 11.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 29 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 2 | 26 |
| ecs/entities_delta | count | 0.38 | 1328 | 2299 |
| physics/inserts | count | 0.42 | 104 | 2559 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 161 | 2335 |
| physics/steps | count | 0.65 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 6 | 19 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 75 |
| stream/spawn/items | count | 0.06 | 20 | 340 |
| stream/upload/bytes | bytes | 4877 | 2.83 M | 29.38 M |
| stream/upload/items | count | 0.01 | 2 | 49 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1232 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.7 ms; slowest 23, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.74 | 2741.8 | main/busy/ms 2714, surface/advance_cpu_ms 68.93, surface/cell_upload_ms 0.79, stream/drive/ms 0.35, weather/map/ms 0.13, stream/tick/ms 0.09, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.05, script/update/ms 0.02, anim/evaluate/ms 0.01, weather/apply/ms 0.01 |
| warm-up | 3.25 | 505.9 | main/busy/ms 2756, render/thread/ms 459, render/busy/ms 357, render/acquire/ms 100, surface/advance_cpu_ms 68.93, render/graph/ms 6.79, render/present/ms 2.10, stream/drive/ms 1.03, surface/cell_upload_ms 0.79, terrain/update/ms 0.58, stream/tick/ms 0.49, terrain/select/ms 0.44 |
| warm-up | 3.43 | 182.7 | pipelines/wait_ms 728, render/thread/ms 474, render/busy/ms 371, pipelines/compile_ms 196, pipelines/front_ms 181, render/acquire/ms 100, main/busy/ms 61.29, terrain/update/ms 22.95, pipelines/driver_ms 14.38, render/graph/ms 10.58, render/present/ms 3.03, stream/drive/ms 1.23 |
| warm-up | 3.73 | 132.5 | pipelines/compile_ms 253, pipelines/front_ms 233, pipelines/wait_ms 182, render/thread/ms 118, render/busy/ms 108, main/busy/ms 85.72, pipelines/driver_ms 19.48, render/acquire/ms 8.37, render/graph/ms 4.42, stream/drive/ms 4.34, stream/tick/ms 2.72, render/present/ms 1.59 |
| warm-up | 3.82 | 64.5 | render/thread/ms 84.31, render/busy/ms 82.82, main/busy/ms 31.60, stream/drive/ms 3.07, render/graph/ms 2.48, terrain/update/ms 1.75, terrain/select/ms 1.29, stream/tick/ms 1.24, render/present/ms 1.18, stream/spawn/ms 1.08, render/acquire/ms 0.31, bigspace/total/ms 0.24 |
| warm-up | 3.94 | 60.3 | pipelines/wait_ms 2045, pipelines/compile_ms 269, pipelines/front_ms 258, render/thread/ms 57.79, render/busy/ms 56.33, main/busy/ms 52.66, stream/drive/ms 33.84, pipelines/driver_ms 11.19, render/graph/ms 3.11, stream/tick/ms 1.87, render/present/ms 1.21, stream/spawn/ms 0.79 |
| warm-up | 4.02 | 47.8 | pipelines/wait_ms 8381, pipelines/compile_ms 105, pipelines/front_ms 96.21, main/busy/ms 50.84, render/thread/ms 50.28, render/busy/ms 47.61, stream/drive/ms 32.30, pipelines/driver_ms 8.88, render/graph/ms 6.04, render/present/ms 2.14, stream/spawn/ms 1.15, terrain/update/ms 0.96 |
| warm-up | 4.11 | 78.0 | pipelines/wait_ms 3872, pipelines/compile_ms 84.73, pipelines/front_ms 79.84, render/thread/ms 60.48, render/busy/ms 58.09, main/busy/ms 36.65, stream/drive/ms 11.29, render/graph/ms 7.60, terrain/update/ms 5.39, pipelines/driver_ms 4.89, render/present/ms 2.06, stream/tick/ms 1.66 |
| warm-up | 4.22 | 83.7 | pipelines/wait_ms 1316, render/thread/ms 62.13, render/busy/ms 60.84, main/busy/ms 58.84, stream/drive/ms 35.87, pipelines/compile_ms 20.25, pipelines/front_ms 18.64, render/graph/ms 4.02, terrain/update/ms 2.43, pipelines/driver_ms 1.60, stream/spawn/ms 1.10, render/present/ms 1.09 |
| warm-up | 4.43 | 98.1 | pipelines/wait_ms 7228, main/busy/ms 116, pipelines/compile_ms 72.05, pipelines/front_ms 65.62, stream/drive/ms 62.72, render/thread/ms 35.98, render/busy/ms 33.76, pipelines/driver_ms 6.43, render/graph/ms 5.54, terrain/update/ms 5.24, stream/spawn/ms 4.45, render/present/ms 1.92 |
| warm-up | 4.48 | 49.8 | pipelines/wait_ms 6663, main/busy/ms 107, stream/drive/ms 64.24, pipelines/compile_ms 59.35, pipelines/front_ms 53.26, render/thread/ms 52.40, render/busy/ms 50.27, pipelines/driver_ms 6.09, render/graph/ms 5.91, terrain/update/ms 5.26, stream/spawn/ms 5.10, render/present/ms 1.80 |
| warm-up | 4.55 | 71.8 | pipelines/wait_ms 632, pipelines/compile_ms 160, pipelines/front_ms 146, main/busy/ms 91.48, stream/drive/ms 62.07, render/thread/ms 52.10, render/busy/ms 50.69, pipelines/driver_ms 13.74, render/graph/ms 5.04, stream/spawn/ms 2.75, render/present/ms 1.17, stream/tick/ms 1.13 |
| warm-up | 4.63 | 55.4 | pipelines/wait_ms 3577, pipelines/compile_ms 86.11, pipelines/front_ms 82.05, main/busy/ms 62.56, stream/drive/ms 41.85, render/thread/ms 32.47, render/busy/ms 30.50, render/graph/ms 5.91, pipelines/driver_ms 4.06, stream/spawn/ms 2.06, render/present/ms 1.53, stream/activate/ms 1.11 |
| warm-up | 4.68 | 37.6 | pipelines/wait_ms 1169, pipelines/compile_ms 63.20, pipelines/front_ms 59.15, main/busy/ms 42.11, stream/drive/ms 27.63, render/thread/ms 21.27, render/busy/ms 19.89, render/graph/ms 5.53, pipelines/driver_ms 4.06, stream/spawn/ms 1.36, render/present/ms 1.12, stream/activate/ms 0.92 |
| warm-up | 5.25 | 30.7 | pipelines/wait_ms 1594, pipelines/compile_ms 200, pipelines/front_ms 184, main/busy/ms 36.66, render/thread/ms 20.62, stream/drive/ms 20.06, render/busy/ms 18.92, pipelines/driver_ms 16.07, render/graph/ms 6.35, render/present/ms 1.45, stream/spawn/ms 1.23, stream/tick/ms 0.91 |
| warm-up | 5.46 | 34.2 | render/thread/ms 42.64, render/busy/ms 41.11, main/busy/ms 27.30, render/graph/ms 5.88, stream/drive/ms 4.70, stream/spawn/ms 2.07, stream/tick/ms 1.39, render/present/ms 1.30, bigspace/total/ms 0.87, terrain/update/ms 0.86, terrain/select/ms 0.74, bigspace/high_precision/ms 0.41 |
| warm-up | 5.62 | 31.6 | pipelines/wait_ms 5311, pipelines/compile_ms 145, pipelines/front_ms 140, main/busy/ms 37.50, render/thread/ms 27.40, render/busy/ms 26.04, stream/drive/ms 20.35, pipelines/driver_ms 4.91, render/graph/ms 4.46, stream/spawn/ms 2.32, stream/tick/ms 1.22, render/present/ms 1.14 |
| warm-up | 6.39 | 31.2 | pipelines/wait_ms 5270, pipelines/compile_ms 82.90, pipelines/front_ms 78.82, main/busy/ms 35.08, render/thread/ms 23.19, render/busy/ms 21.63, render/graph/ms 7.87, pipelines/driver_ms 4.07, render/present/ms 1.39, stream/drive/ms 1.16, physics/insert/ms 0.66, bigspace/total/ms 0.59 |
| warm-up | 6.43 | 29.3 | main/busy/ms 36.28, render/thread/ms 29.55, render/busy/ms 27.77, render/graph/ms 7.76, render/present/ms 1.54, stream/drive/ms 1.18, bigspace/total/ms 0.89, terrain/update/ms 0.60, terrain/select/ms 0.52, bigspace/low_precision/ms 0.50, stream/tick/ms 0.48, physics/insert/ms 0.48 |
| warm-up | 6.46 | 31.6 | main/busy/ms 46.34, render/thread/ms 42.60, render/busy/ms 40.94, render/graph/ms 7.28, stream/drive/ms 1.50, render/present/ms 1.31, bigspace/total/ms 1.19, terrain/update/ms 0.70, bigspace/low_precision/ms 0.69, anim/evaluate_cpu/ms 0.65, terrain/select/ms 0.61, physics/insert/ms 0.56 |
| warm-up | 6.50 | 37.4 | main/busy/ms 53.93, render/thread/ms 49.07, render/busy/ms 47.50, render/graph/ms 7.89, stream/drive/ms 1.43, render/present/ms 1.24, bigspace/total/ms 1.16, anim/evaluate_cpu/ms 1.03, terrain/update/ms 0.73, terrain/select/ms 0.65, physics/insert/ms 0.62, bigspace/low_precision/ms 0.61 |
| warm-up | 6.53 | 30.3 | pipelines/wait_ms 2238, pipelines/compile_ms 162, pipelines/front_ms 157, render/thread/ms 41.73, render/busy/ms 40.39, main/busy/ms 40.20, render/graph/ms 6.00, pipelines/driver_ms 5.15, stream/drive/ms 1.15, render/present/ms 1.09, bigspace/total/ms 0.98, physics/insert/ms 0.63 |
| warm-up | 9.59 | 27.6 | pipelines/wait_ms 15.9 k, pipelines/compile_ms 47.42, pipelines/front_ms 45.94, render/thread/ms 32.64, main/busy/ms 31.86, render/busy/ms 31.29, render/graph/ms 11.97, terrain/update/ms 2.56, pipelines/driver_ms 1.48, bigspace/total/ms 1.26, render/present/ms 1.12, physics/insert/ms 1.07 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_low\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_QUALITY=low
- before the run: GPU 0%, CPU 11%
- during the run: other processes 20% CPU (system minus this process)
- nvidia-smi over the window (250 samples): GPU load **69%** (min 51%) · 2579 MHz (min 2565) · memory 8001 MHz · 71.5 W · 76 °C (max 80) · VRAM 4468 MiB · throttled 100% of samples
