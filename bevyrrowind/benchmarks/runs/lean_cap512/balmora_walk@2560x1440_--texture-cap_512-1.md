# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4563 frames over 65.6 s · **median 13.85 ms (72 fps)** · mean 14.39 · p95 17.87 · p99 19.98 (1% low 50 fps) · worst 39.90 ms · 31 spikes over 1.5× median · 13.7 stutters/min (work 10.1/min)
- warm-up: 331 frames over 8.1 s, worst 2637.5 ms
- hitches (> 34.6 ms): **1** measured, 16 in warm-up
- timed GPU passes: 7.98 ms of a 14.39 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.85 ms, limit 11 ms
- **FAIL** frame_ms: 15 stutters (> 1.5x moving median of 60) in 1.09 min = 13.71/min, limit 1/min
- **FAIL** worst frame 39.90 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.179 | 2.180 | 2.483 | 1.00 | 0.183 |
| main_opaque_pass_3d | 1.669 | 1.670 | 2.273 | 1.00 | 0.088 |
| early prepass | 0.538 | 0.538 | 0.792 | 1.00 | 0.122 |
| bloom | 0.339 | 0.339 | 0.499 | 1.00 | 0.067 |
| atmosphere_luts | 0.327 | 0.327 | 0.469 | 1.00 | 0.004 |
| uniform_allocation | 0.320 | 0.320 | 0.609 | 1.00 | 0.062 |
| water_velocity | 0.299 | 0.299 | 0.359 | 1.00 | 0.002 |
| bin_unpacking | 0.246 | 0.246 | 0.266 | 1.00 | 0.041 |
| auxiliary_views | 0.229 | 0.229 | 0.852 | 1.00 | 0.210 |
| water_sspr | 0.221 | 0.221 | 0.223 | 1.00 | 0.002 |
| ssao | 0.203 | 0.203 | 0.209 | 1.00 | 0.008 |
| atmos_froxel_inject | 0.158 | 0.158 | 0.154 | 1.00 | 0.002 |
| clouds_trace | 0.145 | 0.145 | 0.233 | 1.00 | 0.001 |
| water_fft | 0.138 | 0.138 | 0.138 | 1.00 | 0.003 |
| upscaling | 0.125 | 0.125 | 0.140 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.111 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.093 | 0.093 | 0.175 | 1.00 | 0.009 |
| tonemapping | 0.091 | 0.091 | 0.093 | 1.00 | 0.004 |
| clustering | 0.068 | 0.068 | 0.069 | 1.00 | 0.021 |
| render_sky | 0.057 | 0.057 | 0.058 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.055 | 0.437 | 0.733 | 0.13 | 0.001 |
| surface_trail | 0.051 | 0.058 | 0.057 | 0.87 | 0.002 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.046 | 0.046 | 0.048 | 1.00 | 0.006 |
| water_snapshot | 0.036 | 0.036 | 0.045 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.004 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.029 | 0.232 | 0.236 | 0.13 | 0.005 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 1.00 | 0.111 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 57.241 | 0.000 | 41283.48 | 261190.8 |
| render/thread/ms | 13.161 | 16.477 | 38.78 | 60054.0 |
| render/busy/ms | 12.586 | 15.863 | 37.93 | 57430.6 |
| main/busy/ms | 9.615 | 12.368 | 28.09 | 43872.8 |
| render/graph/ms | 7.690 | 9.781 | 19.30 | 35087.7 |
| pipelines/compile_ms | 0.827 | 0.000 | 152.76 | 3775.2 |
| pipelines/front_ms | 0.799 | 0.000 | 146.00 | 3645.4 |
| render/present/ms | 0.483 | 0.579 | 11.69 | 2204.7 |
| bigspace/total/ms | 0.445 | 0.707 | 3.95 | 2029.9 |
| terrain/update/ms | 0.289 | 0.358 | 2.26 | 1318.3 |
| controller/feed/ms | 0.270 | 0.371 | 0.66 | 1232.7 |
| stream/drive/ms | 0.263 | 0.372 | 19.97 | 1199.3 |
| terrain/select/ms | 0.256 | 0.312 | 0.75 | 1166.9 |
| stream/tick/ms | 0.199 | 0.274 | 0.61 | 908.4 |
| far/select/ms | 0.199 | 0.279 | 0.78 | 907.4 |
| anim/evaluate_cpu/ms | 0.169 | 0.220 | 0.48 | 770.2 |
| bigspace/low_precision/ms | 0.166 | 0.291 | 3.01 | 757.4 |
| anim/evaluate/ms | 0.124 | 0.271 | 1.63 | 566.2 |
| controller/move/ms | 0.124 | 0.244 | 0.54 | 566.0 |
| bigspace/high_precision/ms | 0.109 | 0.221 | 3.45 | 497.0 |
| render/acquire/ms | 0.092 | 0.120 | 0.43 | 418.7 |
| physics/insert/ms | 0.064 | 0.464 | 1.10 | 293.2 |
| physics/step/ms | 0.063 | 0.097 | 0.60 | 287.5 |
| pipelines/driver_ms | 0.028 | 0.000 | 6.76 | 129.8 |
| weather/map/ms | 0.025 | 0.034 | 0.27 | 112.9 |
| anim/apply/ms | 0.011 | 0.022 | 0.15 | 51.0 |
| controller/avoid/ms | 0.010 | 0.014 | 0.19 | 47.6 |
| weather/eval/ms | 0.010 | 0.014 | 0.08 | 46.7 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 33.4 |
| stream/despawn/ms | 0.006 | 0.000 | 1.11 | 29.4 |
| stream/spawn/ms | 0.006 | 0.000 | 1.15 | 26.5 |
| script/input/ms | 0.005 | 0.007 | 0.20 | 24.1 |
| script/ui/ms | 0.004 | 0.006 | 0.06 | 19.2 |
| controller/write/ms | 0.003 | 0.005 | 0.16 | 16.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.63 | 14.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 4.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.10 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 11 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 33 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 26 | 44 | 12 |
| crowd/c0/joints/count | count | 1277 | 2262 | 528 |
| crowd/c0/meshes/count | count | 352 | 646 | 135 |
| crowd/c1/count | count | 146 | 155 | 149 |
| crowd/c1/joints/count | count | 5507 | 5936 | 5668 |
| crowd/c1/meshes/count | count | 1166 | 1268 | 1206 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 9408 | 6179 |
| crowd/joints_uploaded/count | count | 4243 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.2 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 12.10 | 37.93 | 10.91 |
| gpu/timed/ms | ms | 8.68 | 12.17 | 8.36 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 803 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 803 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 34 | 38 | 36 |
| phases/opaque3d/bins | count | 1291 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3386 | 5128 | 3181 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 34 | 38 | 36 |
| phases/opaque3d_prepass/bins | count | 1291 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3386 | 5128 | 3181 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 71 | 360 | 58 |
| phases/shadow/bins | count | 3728 | 5607 | 2220 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9501 | 12.8 k | 5671 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 147 | 328 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 184 | 476 | 19 |
| physics/pending | count | 0 | 1672 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 56 | 0 |
| preprocess/bind_groups | count | 13 | 949 | 5 |
| preprocess/bind_groups_created | count | 13 | 949 | 5 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 310 | 481 | 306 |
| process/cpu_usage | % | 23.08 | 47.39 | 25.96 |
| process/mem_usage | GiB | 4.51 | 4.64 | 4.64 |
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
| started_load_count |  | 4768 | 4862 | 4862 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.34 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.62 | 49.10 | 26.24 |
| system/mem_usage | % | 73.32 | 73.80 | 73.79 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 1728 | 1802 | 1802 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 157 | 442 | 144 |
| views/all/bins | count | 6492 | 8380 | 5700 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.2 k | 13.3 k |
| views/all/pipelines | count | 210 | 394 | 113 |
| views/all/pipelines_distinct | count | 97 | 110 | 91 |
| views/all/work_items | count | 17.4 k | 24.1 k | 13.6 k |
| views/cascade/batch_sets | count | 70 | 88 | 58 |
| views/cascade/bins | count | 2621 | 3221 | 2220 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8084 | 10.1 k | 5671 |
| views/cascade/pipelines | count | 56 | 73 | 47 |
| views/cascade/work_items | count | 8084 | 10.1 k | 5671 |
| views/lamp_face/batch_sets | count | 0 | 286 | 0 |
| views/lamp_face/bins | count | 1066 | 2637 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1212 | 2969 | 0 |
| views/lamp_face/pipelines | count | 89 | 266 | 0 |
| views/lamp_face/work_items | count | 0 | 2748 | 0 |
| views/main/batch_sets | count | 84 | 92 | 86 |
| views/main/bins | count | 2878 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8376 | 13.0 k | 7596 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8544 | 13.2 k | 7904 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.6 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 48.70 | 525 | 222.2 k |
| anim/events/count | count | 0.12 | 4 | 557 |
| assets/Image/added | count | 0.13 | 43 | 605 |
| assets/Image/modified | count | 0.01 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.91 | 152 | 4136 |
| assets/Mesh/removed | count | 1.07 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 477 |
| assets/StandardMaterial/modified | count | 4.75 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 40 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.59 | 1566 | 7240 |
| physics/inserts | count | 0.80 | 156 | 3670 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.51 | 145 | 2335 |
| physics/steps | count | 0.86 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 20 | 378 |
| stream/upload/bytes | bytes | 7419 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1627 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 34.6 ms; slowest 17, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.64 | 2637.5 | main/busy/ms 2613, surface/advance_cpu_ms 70.05, surface/cell_upload_ms 0.86, stream/drive/ms 0.32, weather/map/ms 0.14, bigspace/total/ms 0.11, stream/tick/ms 0.09, bigspace/high_precision/ms 0.07, script/update/ms 0.03, weather/eval/ms 0.03, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.21 | 570.8 | main/busy/ms 2652, render/thread/ms 532, render/busy/ms 431, render/acquire/ms 98.18, surface/advance_cpu_ms 70.05, render/graph/ms 4.85, render/present/ms 2.67, stream/drive/ms 1.30, surface/cell_upload_ms 0.86, stream/tick/ms 0.69, terrain/update/ms 0.63, terrain/select/ms 0.44 |
| warm-up | 3.30 | 88.5 | pipelines/wait_ms 1675, render/thread/ms 546, render/busy/ms 444, render/acquire/ms 98.42, pipelines/compile_ms 88.27, pipelines/front_ms 79.92, main/busy/ms 56.27, terrain/update/ms 23.92, render/graph/ms 8.57, pipelines/driver_ms 8.35, render/present/ms 3.58, stream/drive/ms 1.82 |
| warm-up | 3.43 | 44.7 | render/thread/ms 67.03, render/busy/ms 53.02, main/busy/ms 15.12, render/acquire/ms 12.94, render/graph/ms 2.34, stream/drive/ms 1.70, stream/tick/ms 1.48, render/present/ms 1.07, terrain/update/ms 0.63, terrain/select/ms 0.51, bigspace/total/ms 0.16, bigspace/high_precision/ms 0.09 |
| warm-up | 3.59 | 104.9 | pipelines/compile_ms 274, pipelines/front_ms 258, render/thread/ms 104, render/busy/ms 95.63, main/busy/ms 88.81, pipelines/driver_ms 16.43, render/acquire/ms 6.61, render/graph/ms 4.19, stream/drive/ms 2.26, stream/tick/ms 1.94, render/present/ms 1.49, terrain/update/ms 0.88 |
| warm-up | 3.74 | 54.8 | render/thread/ms 45.61, render/busy/ms 44.12, main/busy/ms 41.68, stream/drive/ms 29.36, render/graph/ms 4.08, render/present/ms 1.15, terrain/update/ms 0.72, stream/tick/ms 0.69, terrain/select/ms 0.59, bigspace/total/ms 0.55, stream/spawn/ms 0.51, bigspace/low_precision/ms 0.40 |
| warm-up | 3.81 | 55.7 | pipelines/wait_ms 4464, pipelines/compile_ms 222, pipelines/front_ms 199, main/busy/ms 40.71, render/thread/ms 36.36, render/busy/ms 34.59, pipelines/driver_ms 22.18, stream/drive/ms 20.64, terrain/update/ms 4.94, render/graph/ms 4.48, render/present/ms 1.54, stream/spawn/ms 1.36 |
| warm-up | 3.89 | 73.2 | pipelines/wait_ms 497, render/thread/ms 52.30, render/busy/ms 50.94, main/busy/ms 39.44, stream/drive/ms 15.96, pipelines/compile_ms 14.20, pipelines/front_ms 13.31, render/graph/ms 3.97, stream/spawn/ms 1.63, stream/tick/ms 1.45, render/present/ms 1.15, terrain/update/ms 0.93 |
| warm-up | 3.93 | 36.7 | render/thread/ms 72.88, render/busy/ms 71.61, main/busy/ms 51.31, stream/drive/ms 31.11, render/graph/ms 5.05, stream/tick/ms 1.54, render/present/ms 1.04, stream/activate/ms 0.79, terrain/update/ms 0.76, stream/spawn/ms 0.64, terrain/select/ms 0.63, physics/insert/ms 0.47 |
| warm-up | 3.99 | 55.0 | pipelines/wait_ms 3494, pipelines/compile_ms 177, pipelines/front_ms 160, render/thread/ms 63.32, render/busy/ms 62.06, main/busy/ms 55.78, stream/drive/ms 31.05, pipelines/driver_ms 17.44, render/graph/ms 8.68, stream/tick/ms 1.28, render/present/ms 1.08, stream/spawn/ms 0.84 |
| warm-up | 4.20 | 64.0 | pipelines/wait_ms 3052, pipelines/compile_ms 165, pipelines/front_ms 159, main/busy/ms 65.76, stream/drive/ms 44.55, render/thread/ms 36.75, render/busy/ms 35.66, pipelines/driver_ms 6.06, render/graph/ms 4.04, stream/spawn/ms 1.81, stream/tick/ms 1.12, render/present/ms 0.93 |
| warm-up | 4.27 | 56.6 | pipelines/wait_ms 6920, main/busy/ms 62.75, stream/drive/ms 45.70, pipelines/compile_ms 41.72, pipelines/front_ms 38.89, render/thread/ms 24.75, render/busy/ms 23.46, render/graph/ms 3.66, pipelines/driver_ms 2.82, stream/spawn/ms 2.29, stream/tick/ms 1.08, render/present/ms 1.06 |
| warm-up | 4.33 | 37.9 | main/busy/ms 41.97, stream/drive/ms 31.49, render/thread/ms 28.44, render/busy/ms 27.21, render/graph/ms 5.62, stream/spawn/ms 2.09, render/present/ms 1.04, stream/tick/ms 0.73, terrain/update/ms 0.66, terrain/select/ms 0.53, bigspace/total/ms 0.24, render/acquire/ms 0.18 |
| warm-up | 5.51 | 43.7 | pipelines/wait_ms 3157, pipelines/compile_ms 149, pipelines/front_ms 146, render/thread/ms 48.20, render/busy/ms 46.85, main/busy/ms 35.23, stream/drive/ms 20.36, render/graph/ms 6.73, pipelines/driver_ms 3.76, render/present/ms 1.15, physics/insert/ms 0.88, terrain/update/ms 0.71 |
| warm-up | 6.85 | 35.7 | main/busy/ms 52.38, render/thread/ms 36.55, render/busy/ms 35.03, render/graph/ms 15.00, physics/insert/ms 1.29, render/present/ms 1.29, stream/drive/ms 0.97, bigspace/total/ms 0.71, terrain/update/ms 0.57, physics/step/ms 0.56, terrain/select/ms 0.49, stream/tick/ms 0.47 |
| warm-up | 6.88 | 35.1 | render/thread/ms 41.98, render/busy/ms 40.48, main/busy/ms 38.06, render/graph/ms 13.43, physics/insert/ms 1.49, render/present/ms 1.25, stream/drive/ms 1.17, bigspace/total/ms 0.77, stream/tick/ms 0.63, physics/step/ms 0.60, terrain/update/ms 0.55, terrain/select/ms 0.48 |
| measure | 2.74 | 39.9 | pipelines/wait_ms 46.1 k, pipelines/compile_ms 59.25, pipelines/front_ms 56.83, render/thread/ms 53.41, render/busy/ms 51.95, render/graph/ms 25.26, main/busy/ms 21.93, pipelines/driver_ms 2.43, render/present/ms 1.26, bigspace/total/ms 0.94, physics/insert/ms 0.71, terrain/update/ms 0.57 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --texture-cap 512 --resolution 2560x1440 --bench-out target/perf\lean_cap512\balmora_walk@2560x1440_--texture-cap_512-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 7%, CPU 7%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **72%** (min 55%) · 2580 MHz (min 2580) · memory 8001 MHz · 70.3 W · 73 °C (max 78) · VRAM 5308 MiB · throttled 100% of samples
