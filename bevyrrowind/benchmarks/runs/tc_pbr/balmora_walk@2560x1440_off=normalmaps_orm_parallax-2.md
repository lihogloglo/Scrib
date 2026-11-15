# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4986 frames over 65.7 s · **median 12.98 ms (77 fps)** · mean 13.17 · p95 15.69 · p99 18.88 (1% low 53 fps) · worst 39.02 ms · 40 spikes over 1.5× median · 25.6 stutters/min (work 22.8/min)
- warm-up: 474 frames over 9.8 s, worst 2836.1 ms
- hitches (> 32.4 ms): **4** measured, 20 in warm-up
- timed GPU passes: 9.65 ms of a 13.17 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.98 ms, limit 11 ms
- **FAIL** frame_ms: 28 stutters (> 1.5x moving median of 60) in 1.09 min = 25.59/min, limit 1/min
- **FAIL** worst frame 39.02 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.793 | 4.021 | 5.056 | 0.94 | 0.066 |
| ssao | 0.757 | 0.803 | 1.101 | 0.94 | 0.008 |
| early prepass | 0.650 | 0.689 | 1.050 | 0.94 | 0.103 |
| clouds_composite | 0.438 | 0.465 | 0.793 | 0.94 | 0.002 |
| taa | 0.390 | 0.414 | 0.740 | 0.94 | 0.002 |
| clouds_trace | 0.321 | 0.340 | 0.606 | 0.94 | 0.001 |
| bloom | 0.317 | 0.337 | 0.503 | 0.94 | 0.074 |
| atmosphere_luts | 0.310 | 0.328 | 0.501 | 0.94 | 0.004 |
| water_velocity | 0.291 | 0.308 | 0.480 | 0.94 | 0.002 |
| water_sspr | 0.262 | 0.278 | 0.411 | 0.94 | 0.002 |
| uniform_allocation | 0.213 | 0.225 | 0.232 | 0.94 | 0.042 |
| render_sky | 0.199 | 0.211 | 0.210 | 0.94 | 0.002 |
| clouds_resolve | 0.182 | 0.193 | 0.202 | 0.94 | 0.001 |
| water_snapshot | 0.172 | 0.183 | 0.245 | 0.94 | 0.002 |
| auxiliary_views | 0.166 | 0.176 | 0.674 | 0.94 | 0.170 |
| bin_unpacking | 0.163 | 0.173 | 0.185 | 0.94 | 0.030 |
| atmos_froxel_inject | 0.152 | 0.162 | 0.159 | 0.94 | 0.002 |
| water_fft | 0.148 | 0.157 | 0.157 | 0.94 | 0.003 |
| upscaling | 0.127 | 0.135 | 0.167 | 0.94 | 0.005 |
| main_transmissive_pass_3d | 0.111 | 0.118 | 0.365 | 0.94 | 0.009 |
| tonemapping | 0.087 | 0.092 | 0.093 | 0.94 | 0.004 |
| clustering | 0.065 | 0.069 | 0.075 | 0.94 | 0.021 |
| lightprobe_irradiance_map | 0.049 | 0.438 | 0.756 | 0.11 | 0.001 |
| surface_trail | 0.047 | 0.061 | 0.058 | 0.78 | 0.003 |
| early_mesh_preprocessing | 0.043 | 0.046 | 0.049 | 0.94 | 0.006 |
| atmos_froxel_integrate | 0.033 | 0.035 | 0.038 | 0.94 | 0.001 |
| clouds_shadow | 0.029 | 0.030 | 0.033 | 0.94 | 0.004 |
| lightprobe_radiance_map | 0.026 | 0.235 | 0.239 | 0.11 | 0.005 |
| per_view_shadow_pass | 0.023 | 0.025 | 0.032 | 0.94 | 0.008 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.94 | 0.003 |
| night_sky | 0.018 | 0.020 | 0.048 | 0.94 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.031 | 0.94 | 0.119 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.94 | 0.003 |
| clouds_fog_light | 0.010 | 0.011 | 0.011 | 0.94 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.94 | 0.003 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.015 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.003 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.864 | 14.279 | 35.95 | 59152.0 |
| render/busy/ms | 10.713 | 13.385 | 31.01 | 53416.8 |
| pipelines/wait_ms | 9.826 | 0.000 | 6239.07 | 48993.8 |
| main/busy/ms | 9.702 | 11.592 | 30.13 | 48376.5 |
| render/graph/ms | 6.011 | 7.786 | 20.51 | 29969.5 |
| render/present/ms | 0.609 | 1.558 | 23.62 | 3034.7 |
| render/acquire/ms | 0.542 | 2.043 | 3.85 | 2700.6 |
| bigspace/total/ms | 0.532 | 1.103 | 3.86 | 2654.2 |
| controller/feed/ms | 0.325 | 0.437 | 0.83 | 1620.8 |
| terrain/update/ms | 0.320 | 0.422 | 2.48 | 1593.2 |
| terrain/select/ms | 0.284 | 0.374 | 0.84 | 1416.1 |
| pipelines/compile_ms | 0.263 | 0.000 | 163.00 | 1310.2 |
| stream/drive/ms | 0.259 | 0.376 | 21.47 | 1292.4 |
| pipelines/front_ms | 0.254 | 0.000 | 157.00 | 1265.2 |
| bigspace/low_precision/ms | 0.207 | 0.393 | 3.55 | 1033.0 |
| far/select/ms | 0.204 | 0.303 | 0.76 | 1017.3 |
| stream/tick/ms | 0.197 | 0.282 | 0.55 | 982.0 |
| anim/evaluate_cpu/ms | 0.180 | 0.231 | 0.60 | 897.5 |
| anim/evaluate/ms | 0.138 | 0.274 | 1.78 | 687.7 |
| bigspace/high_precision/ms | 0.135 | 0.293 | 2.71 | 674.6 |
| controller/move/ms | 0.131 | 0.215 | 0.48 | 653.2 |
| physics/step/ms | 0.059 | 0.096 | 0.66 | 296.1 |
| physics/insert/ms | 0.048 | 0.430 | 0.94 | 239.7 |
| weather/map/ms | 0.027 | 0.036 | 0.27 | 134.0 |
| anim/apply/ms | 0.013 | 0.023 | 0.28 | 64.8 |
| weather/eval/ms | 0.012 | 0.015 | 0.23 | 58.4 |
| controller/avoid/ms | 0.011 | 0.014 | 0.19 | 54.2 |
| pipelines/driver_ms | 0.009 | 0.000 | 6.00 | 45.0 |
| script/update/ms | 0.008 | 0.011 | 0.19 | 40.0 |
| stream/despawn/ms | 0.006 | 0.000 | 1.20 | 31.8 |
| script/ui/ms | 0.006 | 0.008 | 0.16 | 29.6 |
| stream/spawn/ms | 0.006 | 0.000 | 1.09 | 27.5 |
| script/input/ms | 0.005 | 0.007 | 0.11 | 27.0 |
| controller/write/ms | 0.004 | 0.005 | 0.14 | 19.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.80 | 16.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.14 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.05 | 4.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.5 |
| stream/upload/ms | 0.000 | 0.000 | 0.08 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.06 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 18 | 30 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1306 | 2262 | 583 |
| crowd/c0/meshes/count | count | 355 | 646 | 162 |
| crowd/c1/count | count | 145 | 156 | 148 |
| crowd/c1/joints/count | count | 5507 | 5992 | 5613 |
| crowd/c1/meshes/count | count | 1167 | 1308 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4258 | 11.7 k | 6179 |
| crowd/joints_uploaded/count | count | 4230 | 6643 | 6179 |
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
| frame/work/ms | ms | 11.33 | 31.01 | 11.19 |
| gpu/timed/ms | ms | 11.06 | 15.21 | 11.19 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 189 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 849 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 189 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 849 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1280 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3496 | 5116 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1280 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3496 | 5116 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 52 | 289 | 39 |
| phases/shadow/bins | count | 3902 | 5412 | 2236 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9713 | 12.9 k | 5687 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 151 | 299 | 39 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2722 | 3268 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 626 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 201 | 205 | 205 |
| pipelines/waiting | count | 0 | 13 | 0 |
| preprocess/bind_groups | count | 12 | 747 | 6 |
| preprocess/bind_groups_created | count | 12 | 747 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 223 | 339 | 215 |
| process/cpu_usage | % | 29.33 | 46.61 | 29.33 |
| process/mem_usage | GiB | 5.28 | 5.71 | 5.71 |
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
| started_load_count |  | 4765 | 4860 | 4860 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 434 | 528 | 396 |
| statics/materials_bound | count | 434 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.11 | 0.04 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 337 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.37 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 45.18 | 65.18 | 58.65 |
| system/mem_usage | % | 78.55 | 80.02 | 80.02 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 110 | 341 | 99 |
| views/all/bins | count | 6536 | 8256 | 5716 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.6 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 204 | 353 | 95 |
| views/all/pipelines_distinct | count | 83 | 92 | 77 |
| views/all/work_items | count | 17.6 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 48 | 63 | 39 |
| views/cascade/bins | count | 2621 | 3171 | 2236 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8224 | 10.2 k | 5687 |
| views/cascade/pipelines | count | 48 | 63 | 39 |
| views/cascade/work_items | count | 8224 | 10.2 k | 5687 |
| views/lamp_face/batch_sets | count | 0 | 237 | 0 |
| views/lamp_face/bins | count | 1152 | 2502 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1324 | 2865 | 0 |
| views/lamp_face/pipelines | count | 96 | 242 | 0 |
| views/lamp_face/work_items | count | 0 | 2607 | 0 |
| views/main/batch_sets | count | 58 | 62 | 60 |
| views/main/bins | count | 2860 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8705 | 13.0 k | 7592 |
| views/main/pipelines | count | 54 | 58 | 56 |
| views/main/work_items | count | 8858 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 57.38 | 240 | 286.1 k |
| anim/events/count | count | 0.12 | 3 | 580 |
| assets/Image/added | count | 0.12 | 43 | 615 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.84 | 152 | 4197 |
| assets/Mesh/removed | count | 0.98 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 491 |
| assets/StandardMaterial/modified | count | 4.37 | 18 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.08 | 48 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.24 | 1312 | 6167 |
| physics/inserts | count | 0.56 | 147 | 2813 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.47 | 145 | 2335 |
| physics/steps | count | 0.79 | 3 | 3940 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 3 | 16 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.08 | 18 | 378 |
| stream/upload/bytes | bytes | 6789 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1489 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 32.4 ms; slowest 24, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.84 | 2836.1 | main/busy/ms 2810, surface/advance_cpu_ms 70.46, surface/cell_upload_ms 0.79, stream/drive/ms 0.33, weather/map/ms 0.17, stream/tick/ms 0.11, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.03, weather/apply/ms 0.02, far/select/ms 0.01 |
| warm-up | 3.33 | 498.9 | main/busy/ms 2845, render/thread/ms 456, render/busy/ms 433, surface/advance_cpu_ms 70.46, render/acquire/ms 21.30, render/graph/ms 12.50, render/present/ms 2.13, stream/drive/ms 1.35, surface/cell_upload_ms 0.79, stream/tick/ms 0.74, terrain/update/ms 0.42, bigspace/total/ms 0.39 |
| warm-up | 3.58 | 244.3 | pipelines/wait_ms 1090, render/thread/ms 498, render/busy/ms 473, pipelines/compile_ms 238, pipelines/front_ms 221, main/busy/ms 52.64, render/graph/ms 45.47, render/acquire/ms 21.47, terrain/update/ms 18.48, pipelines/driver_ms 16.82, render/present/ms 3.75, stream/drive/ms 1.59 |
| warm-up | 3.79 | 97.3 | pipelines/wait_ms 193, main/busy/ms 61.26, render/thread/ms 55.00, render/busy/ms 52.90, render/graph/ms 3.63, pipelines/compile_ms 2.46, pipelines/front_ms 1.80, render/present/ms 1.71, stream/drive/ms 1.47, stream/tick/ms 1.17, terrain/update/ms 0.81, terrain/select/ms 0.68 |
| warm-up | 3.95 | 70.8 | render/thread/ms 49.16, render/busy/ms 47.69, main/busy/ms 45.30, stream/drive/ms 30.79, render/graph/ms 2.11, render/present/ms 1.25, stream/tick/ms 1.07, terrain/update/ms 0.86, terrain/select/ms 0.69, stream/spawn/ms 0.54, bigspace/total/ms 0.26, render/acquire/ms 0.22 |
| warm-up | 4.07 | 69.9 | pipelines/wait_ms 2521, pipelines/compile_ms 222, pipelines/front_ms 213, render/thread/ms 51.45, render/busy/ms 49.35, main/busy/ms 45.77, stream/drive/ms 24.28, pipelines/driver_ms 9.28, render/graph/ms 3.52, stream/spawn/ms 1.96, render/present/ms 1.81, stream/tick/ms 1.03 |
| warm-up | 4.11 | 39.0 | pipelines/wait_ms 2521, pipelines/compile_ms 222, pipelines/front_ms 213, render/thread/ms 72.99, render/busy/ms 70.66, main/busy/ms 61.95, stream/drive/ms 39.90, pipelines/driver_ms 9.28, render/graph/ms 5.89, stream/spawn/ms 3.03, render/present/ms 2.02, stream/tick/ms 1.49 |
| warm-up | 4.25 | 135.6 | pipelines/wait_ms 2670, pipelines/compile_ms 187, pipelines/front_ms 182, render/thread/ms 123, render/busy/ms 122, main/busy/ms 62.52, stream/drive/ms 39.20, pipelines/driver_ms 5.24, render/graph/ms 5.05, stream/spawn/ms 2.13, stream/tick/ms 1.45, render/present/ms 1.37 |
| warm-up | 4.34 | 38.7 | render/thread/ms 45.66, render/busy/ms 43.62, main/busy/ms 19.01, render/graph/ms 6.58, stream/drive/ms 3.22, render/present/ms 1.84, stream/tick/ms 1.06, stream/activate/ms 1.04, terrain/update/ms 1.01, terrain/select/ms 0.84, physics/insert/ms 0.75, bigspace/total/ms 0.21 |
| warm-up | 4.42 | 46.4 | pipelines/wait_ms 1177, pipelines/compile_ms 162, pipelines/front_ms 144, render/thread/ms 66.41, render/busy/ms 64.07, main/busy/ms 28.10, pipelines/driver_ms 18.30, render/graph/ms 17.58, stream/drive/ms 6.12, stream/spawn/ms 3.26, render/present/ms 2.01, stream/tick/ms 1.58 |
| warm-up | 4.46 | 43.6 | pipelines/wait_ms 1610, pipelines/compile_ms 98.38, pipelines/front_ms 93.38, main/busy/ms 75.06, render/thread/ms 66.80, render/busy/ms 65.27, stream/drive/ms 53.00, render/graph/ms 5.12, pipelines/driver_ms 5.00, stream/tick/ms 1.73, stream/spawn/ms 1.34, render/present/ms 1.24 |
| warm-up | 4.52 | 59.3 | pipelines/wait_ms 8270, pipelines/compile_ms 166, pipelines/front_ms 152, main/busy/ms 73.93, stream/drive/ms 53.70, render/thread/ms 50.98, render/busy/ms 49.60, pipelines/driver_ms 14.08, render/graph/ms 3.74, stream/spawn/ms 2.23, stream/tick/ms 1.26, render/present/ms 1.13 |
| warm-up | 4.61 | 78.7 | pipelines/wait_ms 8163, pipelines/compile_ms 105, pipelines/front_ms 99.54, main/busy/ms 69.59, stream/drive/ms 45.95, render/thread/ms 38.96, render/busy/ms 37.81, pipelines/driver_ms 5.76, render/graph/ms 3.34, stream/spawn/ms 2.23, render/present/ms 0.97, terrain/update/ms 0.91 |
| warm-up | 4.70 | 74.5 | pipelines/wait_ms 7801, pipelines/compile_ms 78.89, pipelines/front_ms 74.04, main/busy/ms 58.13, render/thread/ms 38.79, render/busy/ms 37.73, stream/drive/ms 35.89, pipelines/driver_ms 4.85, render/graph/ms 3.81, stream/spawn/ms 2.07, stream/activate/ms 1.06, stream/tick/ms 0.94 |
| warm-up | 4.74 | 32.6 | main/busy/ms 35.58, stream/drive/ms 23.29, render/thread/ms 14.96, render/busy/ms 13.97, render/graph/ms 3.05, stream/spawn/ms 1.20, render/present/ms 0.81, stream/activate/ms 0.74, stream/tick/ms 0.72, physics/insert/ms 0.68, terrain/update/ms 0.61, terrain/select/ms 0.49 |
| warm-up | 5.06 | 36.5 | pipelines/wait_ms 5642, pipelines/compile_ms 80.26, pipelines/front_ms 76.93, render/thread/ms 55.23, render/busy/ms 53.73, main/busy/ms 45.70, stream/drive/ms 9.14, render/graph/ms 7.22, pipelines/driver_ms 3.33, render/present/ms 1.30, terrain/update/ms 1.12, physics/insert/ms 1.10 |
| warm-up | 5.09 | 37.7 | pipelines/wait_ms 12.5 k, pipelines/compile_ms 148, pipelines/front_ms 139, render/thread/ms 64.80, render/busy/ms 63.18, main/busy/ms 58.73, stream/drive/ms 9.22, pipelines/driver_ms 8.47, render/graph/ms 6.86, render/present/ms 1.33, bigspace/total/ms 1.22, physics/insert/ms 1.01 |
| warm-up | 7.00 | 42.7 | main/busy/ms 53.85, render/thread/ms 31.35, render/busy/ms 29.54, render/graph/ms 8.84, physics/insert/ms 1.90, render/present/ms 1.57, stream/drive/ms 1.14, bigspace/total/ms 0.87, terrain/update/ms 0.68, stream/tick/ms 0.58, terrain/select/ms 0.56, anim/evaluate_cpu/ms 0.50 |
| warm-up | 7.81 | 41.2 | main/busy/ms 60.17, render/thread/ms 42.54, render/busy/ms 41.27, render/graph/ms 8.41, bigspace/total/ms 1.77, stream/drive/ms 1.41, anim/evaluate_cpu/ms 1.33, physics/insert/ms 1.11, render/present/ms 1.02, bigspace/low_precision/ms 0.85, stream/tick/ms 0.74, terrain/update/ms 0.65 |
| warm-up | 7.84 | 34.1 | main/busy/ms 44.00, render/thread/ms 41.71, render/busy/ms 40.51, render/graph/ms 5.46, bigspace/total/ms 1.54, stream/drive/ms 1.21, physics/insert/ms 1.08, render/present/ms 0.89, anim/evaluate_cpu/ms 0.78, bigspace/low_precision/ms 0.75, stream/tick/ms 0.60, terrain/update/ms 0.58 |
| measure | 19.27 | 33.0 | render/thread/ms 41.48, render/busy/ms 40.23, render/graph/ms 25.29, main/busy/ms 18.70, render/present/ms 1.04, bigspace/total/ms 1.03, physics/insert/ms 0.65, terrain/update/ms 0.56, controller/feed/ms 0.54, terrain/select/ms 0.51, stream/drive/ms 0.48, bigspace/low_precision/ms 0.40 |
| measure | 58.08 | 37.3 | render/thread/ms 48.76, render/busy/ms 24.33, render/present/ms 24.20, main/busy/ms 20.66, render/graph/ms 14.10, bigspace/total/ms 1.08, controller/feed/ms 0.81, terrain/update/ms 0.71, far/select/ms 0.67, terrain/select/ms 0.63, stream/drive/ms 0.62, stream/tick/ms 0.54 |
| measure | 59.06 | 39.0 | render/thread/ms 41.25, render/busy/ms 39.61, main/busy/ms 24.56, render/graph/ms 14.37, bigspace/total/ms 1.74, render/present/ms 1.39, controller/feed/ms 1.02, terrain/update/ms 0.89, stream/drive/ms 0.86, terrain/select/ms 0.79, stream/tick/ms 0.53, anim/evaluate_cpu/ms 0.51 |
| measure | 59.10 | 34.7 | render/thread/ms 42.17, render/busy/ms 40.90, main/busy/ms 18.37, render/graph/ms 11.18, render/present/ms 1.11, bigspace/total/ms 1.01, controller/feed/ms 1.01, terrain/update/ms 0.69, terrain/select/ms 0.61, stream/drive/ms 0.55, far/select/ms 0.52, stream/tick/ms 0.48 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\claude\3866c27c11752a1d\4ff90442-bcd4-4820-b03c-4dd8a81992a4\scratchpad\bin\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\tc_pbr\balmora_walk@2560x1440_off=normalmaps_orm_parallax-2 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 16% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **94%** (min 66%) · 2566 MHz (min 2565) · memory 8001 MHz · 91.6 W · 82 °C (max 86) · VRAM 5686 MiB · throttled 100% of samples
