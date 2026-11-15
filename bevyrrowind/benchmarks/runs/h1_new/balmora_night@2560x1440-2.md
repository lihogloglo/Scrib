# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1402 frames over 15.0 s · **median 10.63 ms (94 fps)** · mean 10.70 · p95 12.00 · p99 12.81 (1% low 78 fps) · worst 16.21 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 574 frames over 10.4 s, worst 2588.8 ms
- hitches (> 26.6 ms): **0** measured, 30 in warm-up
- timed GPU passes: 8.71 ms of a 10.70 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.63 ms, limit 11 ms
- pass worst frame 16.21 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.694 | 2.751 | 3.133 | 0.98 | 0.076 |
| dlss_super_resolution | 2.114 | 2.159 | 2.431 | 0.98 | 0.184 |
| early prepass | 0.633 | 0.647 | 0.977 | 0.98 | 0.118 |
| atmosphere_luts | 0.353 | 0.361 | 0.657 | 0.98 | 0.003 |
| bloom | 0.335 | 0.342 | 0.651 | 0.98 | 0.065 |
| main_transmissive_pass_3d | 0.278 | 0.284 | 0.282 | 0.98 | 0.008 |
| water_velocity | 0.268 | 0.274 | 0.269 | 0.98 | 0.002 |
| uniform_allocation | 0.247 | 0.252 | 0.258 | 0.98 | 0.041 |
| water_sspr | 0.241 | 0.246 | 0.392 | 0.98 | 0.002 |
| ssao | 0.200 | 0.205 | 0.211 | 0.98 | 0.008 |
| bin_unpacking | 0.183 | 0.187 | 0.197 | 0.98 | 0.028 |
| water_fft | 0.140 | 0.143 | 0.138 | 0.98 | 0.003 |
| atmos_froxel_inject | 0.136 | 0.139 | 0.135 | 0.98 | 0.002 |
| upscaling | 0.129 | 0.131 | 0.158 | 0.98 | 0.004 |
| clouds_composite | 0.106 | 0.108 | 0.108 | 0.98 | 0.002 |
| tonemapping | 0.091 | 0.093 | 0.096 | 0.98 | 0.004 |
| clouds_trace | 0.069 | 0.071 | 0.092 | 0.98 | 0.001 |
| clustering | 0.067 | 0.068 | 0.080 | 0.98 | 0.021 |
| render_sky | 0.054 | 0.055 | 0.056 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.438 | 0.744 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.051 | 0.053 | 0.98 | 0.001 |
| water_snapshot | 0.041 | 0.042 | 0.057 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.041 | 0.042 | 0.047 | 0.98 | 0.114 |
| early_mesh_preprocessing | 0.041 | 0.042 | 0.043 | 0.98 | 0.006 |
| atmos_froxel_integrate | 0.030 | 0.031 | 0.033 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.029 | 0.241 | 0.440 | 0.12 | 0.005 |
| water_mips | 0.023 | 0.024 | 0.023 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.016 | 0.016 | 0.020 | 0.98 | 0.005 |
| atmosphere_environment | 0.011 | 0.011 | 0.014 | 0.98 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.98 | 0.002 |
| night_sky | 0.007 | 0.007 | 0.009 | 0.98 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.98 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.98 | 0.004 |
| lightprobe_copy | 0.002 | 0.016 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | — | — | — | - | — |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 34.480 | 0.000 | 13268.85 | 48340.3 |
| render/thread/ms | 9.661 | 10.929 | 25.05 | 13544.5 |
| render/busy/ms | 8.526 | 9.760 | 18.57 | 11953.6 |
| main/busy/ms | 7.057 | 8.389 | 10.64 | 9894.3 |
| render/graph/ms | 5.079 | 6.169 | 11.24 | 7121.4 |
| render/acquire/ms | 0.640 | 1.705 | 2.59 | 896.8 |
| render/present/ms | 0.495 | 0.589 | 5.39 | 694.1 |
| bigspace/total/ms | 0.323 | 0.520 | 2.06 | 453.0 |
| pipelines/compile_ms | 0.308 | 0.000 | 126.36 | 431.5 |
| pipelines/front_ms | 0.299 | 0.000 | 123.83 | 418.6 |
| terrain/update/ms | 0.223 | 0.281 | 0.49 | 312.6 |
| stream/drive/ms | 0.194 | 0.274 | 0.51 | 272.2 |
| terrain/select/ms | 0.194 | 0.244 | 0.46 | 271.7 |
| controller/feed/ms | 0.172 | 0.225 | 0.35 | 241.6 |
| far/select/ms | 0.168 | 0.245 | 0.43 | 235.2 |
| stream/tick/ms | 0.166 | 0.239 | 0.47 | 232.1 |
| controller/move/ms | 0.129 | 0.271 | 0.51 | 180.4 |
| bigspace/high_precision/ms | 0.109 | 0.221 | 1.85 | 153.4 |
| bigspace/low_precision/ms | 0.077 | 0.165 | 1.72 | 107.6 |
| anim/evaluate/ms | 0.068 | 0.152 | 1.28 | 95.4 |
| anim/evaluate_cpu/ms | 0.066 | 0.079 | 0.34 | 92.2 |
| physics/step/ms | 0.032 | 0.059 | 0.22 | 44.4 |
| weather/map/ms | 0.016 | 0.021 | 0.05 | 22.0 |
| weather/eval/ms | 0.010 | 0.013 | 0.11 | 13.6 |
| pipelines/driver_ms | 0.009 | 0.000 | 3.96 | 12.8 |
| script/update/ms | 0.007 | 0.010 | 0.22 | 10.2 |
| controller/avoid/ms | 0.007 | 0.008 | 0.20 | 9.4 |
| script/input/ms | 0.005 | 0.006 | 0.12 | 7.2 |
| script/ui/ms | 0.005 | 0.006 | 0.03 | 6.6 |
| anim/apply/ms | 0.004 | 0.005 | 0.06 | 6.0 |
| controller/write/ms | 0.002 | 0.002 | 0.03 | 2.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.05 | 1.1 |
| weather/apply/ms | 0.001 | 0.001 | 0.15 | 0.8 |
| physics/insert/ms | 0.000 | 0.000 | 0.06 | 0.1 |
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
| crowd/joint_blocks/count | count | 118 | 118 | 118 |
| crowd/joints_extracted/count | count | 2958 | 2958 | 2958 |
| crowd/joints_uploaded/count | count | 2958 | 2958 | 2958 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.09 M | 189.09 M | 189.09 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.50 | 18.57 | 9.01 |
| gpu/timed/ms | ms | 9.46 | 11.18 | 9.01 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 478.47 M | 478.47 M | 478.47 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d/bins | count | 330 | 330 | 330 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3189 | 3189 | 3189 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 330 | 330 | 330 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3189 | 3189 | 3189 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 34 | 34 | 34 |
| phases/opaque3d/bins | count | 3892 | 3892 | 3892 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 9292 | 9292 | 9292 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 27 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 34 | 34 | 34 |
| phases/opaque3d_prepass/bins | count | 3892 | 3892 | 3892 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 9292 | 9292 | 9292 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 27 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 36 | 36 | 36 |
| phases/shadow/bins | count | 6686 | 6686 | 6686 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 20.0 k | 20.0 k | 20.0 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 31 | 31 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 20 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 200 | 200 | 200 |
| pipelines/waiting | count | 0 | 9 | 0 |
| preprocess/bind_groups | count | 0 | 0 | 0 |
| preprocess/bind_groups_created | count | 0 | 0 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 262 | 262 | 262 |
| process/cpu_usage | % | 26.44 | 41.11 | 41 |
| process/mem_usage | GiB | 4.43 | 4.49 | 4.49 |
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
| started_load_count |  | 3545 | 3547 | 3547 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 4 | 4 |
| statics/material_pipelines | count | 4 | 4 | 4 |
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
| system/cpu_usage | % | 33.35 | 41.77 | 34.86 |
| system/mem_usage | % | 62.97 | 63.21 | 63.20 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2478 | 2478 | 2478 |
| textures/images | count | 1917 | 1917 | 1917 |
| views/all/batch_sets | count | 120 | 120 | 120 |
| views/all/bins | count | 15.1 k | 15.1 k | 15.1 k |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 45.0 k | 45.0 k | 45.0 k |
| views/all/pipelines | count | 99 | 99 | 99 |
| views/all/pipelines_distinct | count | 91 | 91 | 91 |
| views/all/work_items | count | 45.4 k | 45.4 k | 45.4 k |
| views/cascade/batch_sets | count | 36 | 36 | 36 |
| views/cascade/bins | count | 6686 | 6686 | 6686 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 20.0 k | 20.0 k | 20.0 k |
| views/cascade/pipelines | count | 31 | 31 | 31 |
| views/cascade/work_items | count | 20.0 k | 20.0 k | 20.0 k |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 84 | 84 | 84 |
| views/main/bins | count | 8444 | 8444 | 8444 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 25.0 k | 25.0 k | 25.0 k |
| views/main/pipelines | count | 68 | 68 | 68 |
| views/main/work_items | count | 25.4 k | 25.4 k | 25.4 k |
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
| world/point_lights | count | 222 | 222 | 222 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.09 | 2 | 120 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.53 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| physics/inserts | count | 0.01 | 20 | 20 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.64 | 1 | 900 |
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
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.6 ms; slowest 30, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.59 | 2588.8 | main/busy/ms 2564, surface/advance_cpu_ms 0.73, surface/cell_upload_ms 0.31, stream/drive/ms 0.28, bigspace/total/ms 0.13, bigspace/high_precision/ms 0.09, stream/tick/ms 0.08, weather/map/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.29 | 703.4 | main/busy/ms 2601, render/thread/ms 667, render/busy/ms 527, render/acquire/ms 139, render/graph/ms 5.90, render/present/ms 2.08, stream/drive/ms 0.87, surface/advance_cpu_ms 0.73, stream/tick/ms 0.49, terrain/update/ms 0.46, terrain/select/ms 0.32, surface/cell_upload_ms 0.31 |
| warm-up | 3.50 | 203.0 | pipelines/wait_ms 1514, render/thread/ms 678, render/busy/ms 536, pipelines/compile_ms 217, pipelines/front_ms 203, render/acquire/ms 139, main/busy/ms 54.43, terrain/update/ms 21.63, pipelines/driver_ms 14.30, render/graph/ms 9.58, render/present/ms 2.94, stream/drive/ms 1.18 |
| warm-up | 3.81 | 88.1 | pipelines/wait_ms 257, pipelines/compile_ms 256, pipelines/front_ms 242, main/busy/ms 58.45, render/thread/ms 46.98, render/busy/ms 45.37, pipelines/driver_ms 14.49, render/graph/ms 2.71, render/present/ms 1.43, stream/drive/ms 1.12, stream/tick/ms 0.83, terrain/update/ms 0.67 |
| warm-up | 3.95 | 53.8 | pipelines/wait_ms 453, render/thread/ms 61.72, render/busy/ms 59.71, main/busy/ms 57.08, stream/drive/ms 43.32, render/graph/ms 5.72, render/present/ms 1.71, pipelines/compile_ms 1.45, pipelines/front_ms 1.07, stream/tick/ms 0.92, terrain/update/ms 0.52, terrain/select/ms 0.42 |
| warm-up | 3.99 | 36.7 | pipelines/wait_ms 718, pipelines/compile_ms 191, pipelines/front_ms 168, render/thread/ms 69.92, render/busy/ms 68.14, main/busy/ms 41.22, stream/drive/ms 23.06, pipelines/driver_ms 22.43, render/graph/ms 6.40, render/present/ms 1.53, stream/spawn/ms 1.38, stream/tick/ms 1.34 |
| warm-up | 4.03 | 46.1 | pipelines/wait_ms 265, pipelines/compile_ms 189, pipelines/front_ms 167, render/thread/ms 52.92, render/busy/ms 51.01, main/busy/ms 33.99, pipelines/driver_ms 22.04, stream/drive/ms 15.43, render/graph/ms 5.20, stream/spawn/ms 1.85, render/present/ms 1.64, stream/tick/ms 1.21 |
| warm-up | 4.10 | 63.7 | render/thread/ms 67.66, render/busy/ms 65.07, main/busy/ms 50.30, stream/drive/ms 33.14, render/graph/ms 7.02, render/present/ms 2.27, stream/spawn/ms 1.92, terrain/update/ms 0.84, stream/tick/ms 0.66, terrain/select/ms 0.64, bigspace/total/ms 0.35, render/acquire/ms 0.33 |
| warm-up | 4.13 | 30.2 | render/thread/ms 65.51, render/busy/ms 63.13, main/busy/ms 63.10, stream/drive/ms 46.34, render/graph/ms 6.36, stream/spawn/ms 2.41, render/present/ms 2.13, terrain/update/ms 0.80, stream/tick/ms 0.63, terrain/select/ms 0.62, stream/activate/ms 0.30, bigspace/total/ms 0.30 |
| warm-up | 4.18 | 49.5 | pipelines/wait_ms 1092, pipelines/compile_ms 214, pipelines/front_ms 206, main/busy/ms 50.62, render/thread/ms 45.60, render/busy/ms 44.14, stream/drive/ms 33.15, pipelines/driver_ms 8.10, render/graph/ms 3.28, stream/spawn/ms 1.32, render/present/ms 1.30, stream/activate/ms 0.68 |
| warm-up | 4.24 | 27.0 | pipelines/wait_ms 2028, pipelines/compile_ms 78.73, pipelines/front_ms 74.44, main/busy/ms 53.08, stream/drive/ms 35.60, render/thread/ms 34.99, render/busy/ms 32.74, render/graph/ms 5.78, pipelines/driver_ms 4.29, stream/spawn/ms 2.70, render/present/ms 1.92, stream/tick/ms 1.45 |
| warm-up | 4.30 | 66.6 | pipelines/wait_ms 1405, main/busy/ms 54.99, render/thread/ms 48.67, render/busy/ms 46.83, stream/drive/ms 34.15, pipelines/compile_ms 23.73, pipelines/front_ms 22.26, render/graph/ms 4.56, stream/tick/ms 1.59, render/present/ms 1.56, pipelines/driver_ms 1.46, stream/spawn/ms 1.41 |
| warm-up | 4.39 | 47.2 | pipelines/wait_ms 5541, pipelines/compile_ms 172, pipelines/front_ms 166, main/busy/ms 64.00, stream/drive/ms 52.90, render/thread/ms 20.97, render/busy/ms 19.65, pipelines/driver_ms 6.34, render/graph/ms 3.27, render/present/ms 1.13, stream/tick/ms 0.78, stream/spawn/ms 0.73 |
| warm-up | 4.47 | 49.3 | pipelines/wait_ms 4332, pipelines/compile_ms 50.99, main/busy/ms 49.63, pipelines/front_ms 47.32, stream/drive/ms 35.55, render/thread/ms 26.17, render/busy/ms 24.94, pipelines/driver_ms 3.67, render/graph/ms 2.86, stream/spawn/ms 1.01, render/present/ms 0.91, stream/tick/ms 0.87 |
| warm-up | 4.66 | 29.8 | pipelines/wait_ms 3289, render/thread/ms 43.79, render/busy/ms 38.21, main/busy/ms 30.92, pipelines/compile_ms 27.83, pipelines/front_ms 26.02, stream/drive/ms 10.64, render/present/ms 5.11, render/graph/ms 4.66, stream/spawn/ms 3.33, pipelines/driver_ms 1.82, terrain/update/ms 0.85 |
| warm-up | 4.69 | 27.2 | render/thread/ms 40.11, render/busy/ms 38.99, main/busy/ms 26.06, stream/drive/ms 9.45, render/graph/ms 3.01, stream/spawn/ms 2.38, render/present/ms 0.87, terrain/update/ms 0.70, stream/tick/ms 0.61, terrain/select/ms 0.54, stream/activate/ms 0.49, physics/insert/ms 0.43 |
| warm-up | 4.93 | 38.6 | render/thread/ms 45.78, render/busy/ms 44.14, main/busy/ms 22.60, render/graph/ms 5.13, stream/drive/ms 4.03, stream/spawn/ms 2.08, render/present/ms 1.39, stream/tick/ms 0.93, bigspace/total/ms 0.62, terrain/update/ms 0.58, terrain/select/ms 0.46, bigspace/high_precision/ms 0.27 |
| warm-up | 4.96 | 33.5 | pipelines/wait_ms 3919, pipelines/compile_ms 93.13, pipelines/front_ms 90.10, render/thread/ms 53.84, render/busy/ms 52.76, main/busy/ms 23.29, stream/drive/ms 4.35, render/graph/ms 4.16, pipelines/driver_ms 3.03, stream/spawn/ms 2.12, render/present/ms 0.90, stream/tick/ms 0.73 |
| warm-up | 5.05 | 34.4 | pipelines/wait_ms 1430, render/thread/ms 51.68, pipelines/compile_ms 50.03, render/busy/ms 49.55, pipelines/front_ms 47.22, main/busy/ms 27.74, stream/drive/ms 8.20, render/graph/ms 6.19, pipelines/driver_ms 2.81, stream/spawn/ms 2.68, render/present/ms 1.77, stream/tick/ms 1.34 |
| warm-up | 5.09 | 40.1 | render/thread/ms 60.02, render/busy/ms 58.16, main/busy/ms 29.55, stream/drive/ms 6.36, render/graph/ms 6.16, stream/spawn/ms 2.89, render/present/ms 1.54, stream/tick/ms 1.31, bigspace/total/ms 0.61, terrain/update/ms 0.58, terrain/select/ms 0.45, render/acquire/ms 0.31 |
| warm-up | 5.13 | 38.4 | pipelines/wait_ms 3079, pipelines/compile_ms 107, pipelines/front_ms 100, render/thread/ms 56.09, render/busy/ms 54.73, main/busy/ms 24.32, pipelines/driver_ms 6.72, render/graph/ms 4.67, stream/drive/ms 3.90, stream/spawn/ms 2.48, render/present/ms 1.11, terrain/update/ms 0.68 |
| warm-up | 5.19 | 36.1 | render/thread/ms 43.65, render/busy/ms 42.46, main/busy/ms 31.83, stream/drive/ms 15.33, render/graph/ms 5.25, stream/spawn/ms 2.52, render/present/ms 1.02, stream/tick/ms 0.67, bigspace/total/ms 0.59, terrain/update/ms 0.58, terrain/select/ms 0.44, bigspace/high_precision/ms 0.31 |
| warm-up | 5.59 | 27.1 | render/thread/ms 32.31, render/busy/ms 31.04, main/busy/ms 30.80, render/graph/ms 5.08, stream/drive/ms 3.12, stream/spawn/ms 1.66, render/present/ms 1.09, bigspace/total/ms 0.69, stream/tick/ms 0.56, physics/step/ms 0.53, terrain/update/ms 0.49, physics/insert/ms 0.47 |
| warm-up | 5.62 | 30.2 | render/thread/ms 45.31, render/busy/ms 34.22, main/busy/ms 22.47, render/present/ms 10.86, render/graph/ms 7.18, stream/drive/ms 3.22, stream/spawn/ms 2.05, bigspace/total/ms 0.81, terrain/update/ms 0.52, stream/tick/ms 0.50, physics/insert/ms 0.42, terrain/select/ms 0.39 |
| warm-up | 5.69 | 47.2 | render/thread/ms 47.76, render/busy/ms 46.48, main/busy/ms 27.75, stream/drive/ms 8.60, render/graph/ms 5.75, stream/spawn/ms 1.33, render/present/ms 1.02, bigspace/total/ms 0.69, terrain/update/ms 0.68, stream/tick/ms 0.65, terrain/select/ms 0.52, physics/insert/ms 0.34 |
| warm-up | 6.31 | 29.4 | main/busy/ms 42.33, render/thread/ms 35.70, render/busy/ms 34.13, render/graph/ms 8.04, terrain/update/ms 4.98, render/present/ms 1.23, stream/drive/ms 1.20, bigspace/total/ms 0.69, stream/tick/ms 0.51, terrain/select/ms 0.43, anim/evaluate_cpu/ms 0.38, stream/spawn/ms 0.37 |
| warm-up | 6.36 | 46.2 | pipelines/wait_ms 4974, pipelines/compile_ms 101, pipelines/front_ms 94.70, main/busy/ms 58.42, render/thread/ms 52.81, render/busy/ms 51.13, render/graph/ms 8.60, terrain/update/ms 6.81, pipelines/driver_ms 5.94, stream/drive/ms 1.36, render/present/ms 1.35, bigspace/total/ms 0.86 |
| warm-up | 6.39 | 36.1 | pipelines/wait_ms 4974, pipelines/compile_ms 101, pipelines/front_ms 94.70, render/thread/ms 52.72, render/busy/ms 51.43, main/busy/ms 43.16, render/graph/ms 6.08, pipelines/driver_ms 5.94, terrain/update/ms 2.39, stream/drive/ms 1.31, render/present/ms 1.08, bigspace/total/ms 0.75 |
| warm-up | 7.27 | 32.9 | pipelines/wait_ms 1638, render/thread/ms 43.20, main/busy/ms 42.96, render/busy/ms 41.85, pipelines/compile_ms 23.22, pipelines/front_ms 22.09, render/graph/ms 10.43, bigspace/total/ms 1.34, pipelines/driver_ms 1.13, render/present/ms 1.06, stream/drive/ms 1.01, anim/evaluate_cpu/ms 1.01 |
| warm-up | 7.34 | 29.7 | main/busy/ms 36.02, render/thread/ms 23.97, render/busy/ms 22.76, render/graph/ms 7.67, render/present/ms 1.04, stream/drive/ms 0.89, bigspace/total/ms 0.64, terrain/update/ms 0.50, terrain/select/ms 0.40, stream/tick/ms 0.39, far/select/ms 0.35, physics/insert/ms 0.32 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\h1_new\balmora_night@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 3%, CPU 10%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **97%** (min 91%) · 2580 MHz (min 2580) · memory 8001 MHz · 91.4 W · 76 °C (max 78) · VRAM 5109 MiB · throttled 100% of samples
