# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=performance · resolution: 2560x1440 · rings: near None far None
- 4607 frames over 65.6 s · **median 13.81 ms (72 fps)** · mean 14.25 · p95 18.07 · p99 20.87 (1% low 48 fps) · worst 35.22 ms · 53 spikes over 1.5× median · 21.0 stutters/min (work 21.9/min)
- warm-up: 329 frames over 8.6 s, worst 2590.9 ms
- hitches (> 34.5 ms): **1** measured, 15 in warm-up
- timed GPU passes: 8.28 ms of a 14.25 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.81 ms, limit 11 ms
- **FAIL** frame_ms: 23 stutters (> 1.5x moving median of 60) in 1.09 min = 21.02/min, limit 1/min
- **FAIL** worst frame 35.22 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.712 | 2.750 | 2.899 | 0.99 | 0.187 |
| main_opaque_pass_3d | 1.643 | 1.666 | 2.061 | 0.99 | 0.081 |
| early prepass | 0.521 | 0.528 | 0.755 | 0.99 | 0.120 |
| atmosphere_luts | 0.310 | 0.314 | 0.373 | 0.99 | 0.003 |
| uniform_allocation | 0.309 | 0.313 | 0.421 | 0.99 | 0.059 |
| bloom | 0.308 | 0.312 | 0.315 | 0.99 | 0.068 |
| water_velocity | 0.279 | 0.283 | 0.353 | 0.99 | 0.002 |
| bin_unpacking | 0.235 | 0.238 | 0.268 | 0.99 | 0.041 |
| auxiliary_views | 0.234 | 0.238 | 0.790 | 0.99 | 0.220 |
| water_sspr | 0.207 | 0.210 | 0.211 | 0.99 | 0.002 |
| ssao | 0.191 | 0.194 | 0.194 | 0.99 | 0.008 |
| atmos_froxel_inject | 0.147 | 0.149 | 0.150 | 0.99 | 0.002 |
| clouds_trace | 0.139 | 0.141 | 0.229 | 0.99 | 0.001 |
| water_fft | 0.130 | 0.132 | 0.136 | 0.99 | 0.003 |
| upscaling | 0.122 | 0.124 | 0.131 | 0.99 | 0.004 |
| clouds_composite | 0.106 | 0.108 | 0.110 | 0.99 | 0.002 |
| main_transmissive_pass_3d | 0.086 | 0.088 | 0.165 | 0.99 | 0.009 |
| tonemapping | 0.086 | 0.087 | 0.091 | 0.99 | 0.004 |
| clustering | 0.065 | 0.066 | 0.069 | 0.99 | 0.020 |
| render_sky | 0.053 | 0.054 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.049 | 0.420 | 0.423 | 0.12 | 0.001 |
| surface_trail | 0.049 | 0.057 | 0.057 | 0.86 | 0.002 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.043 | 0.043 | 0.048 | 0.99 | 0.006 |
| water_snapshot | 0.031 | 0.031 | 0.035 | 0.99 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.027 | 0.229 | 0.236 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.026 | 0.029 | 0.99 | 0.001 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 0.99 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.029 | 0.99 | 0.108 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 40.566 | 0.000 | 18020.60 | 186885.8 |
| render/thread/ms | 13.012 | 16.606 | 32.87 | 59944.3 |
| render/busy/ms | 12.426 | 16.010 | 31.98 | 57248.6 |
| main/busy/ms | 9.564 | 12.358 | 32.54 | 44063.6 |
| render/graph/ms | 7.665 | 10.130 | 17.54 | 35313.7 |
| pipelines/compile_ms | 0.776 | 0.000 | 175.55 | 3575.3 |
| pipelines/front_ms | 0.748 | 0.000 | 169.10 | 3446.1 |
| render/present/ms | 0.494 | 0.594 | 3.23 | 2274.4 |
| bigspace/total/ms | 0.452 | 0.694 | 4.55 | 2081.5 |
| terrain/update/ms | 0.293 | 0.370 | 1.68 | 1350.2 |
| controller/feed/ms | 0.283 | 0.382 | 1.13 | 1304.3 |
| stream/drive/ms | 0.264 | 0.366 | 24.26 | 1215.9 |
| terrain/select/ms | 0.260 | 0.328 | 0.61 | 1196.5 |
| stream/tick/ms | 0.199 | 0.276 | 0.69 | 915.9 |
| far/select/ms | 0.196 | 0.281 | 0.85 | 904.5 |
| anim/evaluate/ms | 0.175 | 0.331 | 2.59 | 806.9 |
| bigspace/low_precision/ms | 0.172 | 0.290 | 3.89 | 790.7 |
| anim/evaluate_cpu/ms | 0.152 | 0.207 | 0.68 | 699.6 |
| controller/move/ms | 0.132 | 0.251 | 0.46 | 606.8 |
| bigspace/high_precision/ms | 0.110 | 0.220 | 2.79 | 505.0 |
| render/acquire/ms | 0.091 | 0.119 | 0.35 | 421.3 |
| physics/step/ms | 0.064 | 0.099 | 0.70 | 293.8 |
| physics/insert/ms | 0.062 | 0.461 | 1.04 | 287.5 |
| pipelines/driver_ms | 0.028 | 0.000 | 6.54 | 129.2 |
| weather/map/ms | 0.024 | 0.034 | 0.26 | 112.5 |
| anim/apply/ms | 0.012 | 0.022 | 0.20 | 53.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.20 | 49.5 |
| weather/eval/ms | 0.010 | 0.015 | 0.05 | 48.1 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 32.8 |
| stream/despawn/ms | 0.007 | 0.000 | 0.94 | 31.5 |
| stream/spawn/ms | 0.006 | 0.000 | 1.14 | 26.1 |
| script/input/ms | 0.005 | 0.007 | 0.20 | 23.8 |
| script/ui/ms | 0.004 | 0.006 | 0.04 | 19.6 |
| controller/write/ms | 0.004 | 0.005 | 0.06 | 17.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.68 | 15.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 4.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.15 | 1.5 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 3 | 0 |
| anim/a1/count | count | 0 | 34 | 0 |
| anim/a2/count | count | 0 | 17 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 108 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 75 | 12 |
| crowd/c0/joints/count | count | 1236 | 2939 | 537 |
| crowd/c0/meshes/count | count | 343 | 686 | 146 |
| crowd/c1/count | count | 147 | 155 | 149 |
| crowd/c1/joints/count | count | 5569 | 5889 | 5659 |
| crowd/c1/meshes/count | count | 1175 | 1278 | 1195 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4283 | 8979 | 6179 |
| crowd/joints_uploaded/count | count | 4259 | 6643 | 6179 |
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
| frame/work/ms | ms | 12.10 | 32.54 | 10.20 |
| gpu/timed/ms | ms | 9.04 | 12.25 | 9.14 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 796 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 796 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 34 | 39 | 34 |
| phases/opaque3d/bins | count | 1292 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3351 | 5140 | 3181 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 28 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 34 | 39 | 34 |
| phases/opaque3d_prepass/bins | count | 1292 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3351 | 5140 | 3181 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 28 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 72 | 331 | 56 |
| phases/shadow/bins | count | 3757 | 5559 | 2220 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9371 | 13.0 k | 5671 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 141 | 326 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 137 | 476 | 19 |
| physics/pending | count | 0 | 876 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 220 | 226 | 226 |
| pipelines/waiting | count | 0 | 48 | 0 |
| preprocess/bind_groups | count | 13 | 899 | 6 |
| preprocess/bind_groups_created | count | 13 | 899 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 308 | 464 | 293 |
| process/cpu_usage | % | 22.60 | 55.45 | 16.35 |
| process/mem_usage | GiB | 5.16 | 5.34 | 5.34 |
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
| started_load_count |  | 4768 | 4859 | 4859 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.11 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.34 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.78 | 66.24 | 34.62 |
| system/mem_usage | % | 75.22 | 76.26 | 76.26 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 160 | 417 | 138 |
| views/all/bins | count | 6324 | 8347 | 5700 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 17.7 k | 25.5 k | 13.3 k |
| views/all/pipelines | count | 204 | 392 | 113 |
| views/all/pipelines_distinct | count | 96 | 108 | 91 |
| views/all/work_items | count | 17.1 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 69 | 90 | 56 |
| views/cascade/bins | count | 2610 | 3183 | 2220 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8119 | 10.2 k | 5671 |
| views/cascade/pipelines | count | 56 | 73 | 47 |
| views/cascade/work_items | count | 8119 | 10.2 k | 5671 |
| views/lamp_face/batch_sets | count | 0 | 256 | 0 |
| views/lamp_face/bins | count | 1115 | 2619 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1225 | 2981 | 0 |
| views/lamp_face/pipelines | count | 84 | 261 | 0 |
| views/lamp_face/work_items | count | 0 | 2662 | 0 |
| views/main/batch_sets | count | 82 | 94 | 82 |
| views/main/bins | count | 2884 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8300 | 13.1 k | 7596 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8468 | 13.3 k | 7904 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.6 k | 37.7 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 60.78 | 1542 | 280.0 k |
| anim/events/count | count | 0.13 | 3 | 581 |
| assets/Image/added | count | 0.16 | 38 | 747 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.96 | 168 | 4437 |
| assets/Mesh/removed | count | 1.06 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.14 | 42 | 634 |
| assets/StandardMaterial/modified | count | 4.70 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 4.28 | 4800 | 19.7 k |
| physics/inserts | count | 0.66 | 109 | 3047 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.51 | 145 | 2335 |
| physics/steps | count | 0.85 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 19 | 378 |
| stream/upload/bytes | bytes | 7348 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1611 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 34.5 ms; slowest 16, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.59 | 2590.9 | main/busy/ms 2563, surface/advance_cpu_ms 75.06, surface/cell_upload_ms 0.75, stream/drive/ms 0.43, weather/map/ms 0.13, stream/tick/ms 0.11, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.03, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.49 | 897.0 | main/busy/ms 2604, render/thread/ms 855, render/busy/ms 709, render/acquire/ms 143, surface/advance_cpu_ms 75.06, render/graph/ms 5.66, render/present/ms 3.30, stream/drive/ms 1.21, surface/cell_upload_ms 0.75, stream/tick/ms 0.60, terrain/update/ms 0.56, terrain/select/ms 0.43 |
| warm-up | 3.55 | 66.6 | render/thread/ms 869, render/busy/ms 722, pipelines/wait_ms 420, render/acquire/ms 143, pipelines/compile_ms 72.89, pipelines/front_ms 66.17, main/busy/ms 58.58, terrain/update/ms 22.86, render/graph/ms 9.17, pipelines/driver_ms 6.72, render/present/ms 4.19, stream/drive/ms 1.49 |
| warm-up | 3.82 | 112.6 | pipelines/compile_ms 254, pipelines/front_ms 242, render/thread/ms 104, render/busy/ms 92.59, main/busy/ms 83.15, pipelines/driver_ms 11.86, render/acquire/ms 10.05, render/graph/ms 4.51, stream/drive/ms 3.38, stream/tick/ms 2.86, render/present/ms 1.49, terrain/update/ms 0.66 |
| warm-up | 3.96 | 46.1 | render/thread/ms 48.78, render/busy/ms 47.33, main/busy/ms 37.39, stream/drive/ms 25.79, render/graph/ms 4.01, render/present/ms 1.24, stream/tick/ms 0.85, stream/spawn/ms 0.64, terrain/update/ms 0.62, terrain/select/ms 0.52, render/acquire/ms 0.21, bigspace/total/ms 0.18 |
| warm-up | 4.03 | 67.6 | pipelines/wait_ms 5162, pipelines/compile_ms 206, pipelines/front_ms 190, render/thread/ms 48.42, render/busy/ms 46.84, main/busy/ms 39.72, stream/drive/ms 27.34, pipelines/driver_ms 15.79, render/graph/ms 4.40, stream/spawn/ms 1.42, render/present/ms 1.32, stream/tick/ms 1.29 |
| warm-up | 4.11 | 65.2 | pipelines/wait_ms 1515, pipelines/compile_ms 72.83, pipelines/front_ms 70.33, render/thread/ms 49.66, render/busy/ms 47.77, main/busy/ms 38.88, stream/drive/ms 12.14, render/graph/ms 5.72, pipelines/driver_ms 2.50, stream/spawn/ms 1.85, render/present/ms 1.65, stream/tick/ms 1.28 |
| warm-up | 4.16 | 46.3 | pipelines/wait_ms 1515, render/thread/ms 73.92, pipelines/compile_ms 72.83, render/busy/ms 72.06, pipelines/front_ms 70.33, main/busy/ms 64.34, stream/drive/ms 33.13, render/graph/ms 6.22, terrain/update/ms 5.17, pipelines/driver_ms 2.50, stream/tick/ms 2.37, render/present/ms 1.59 |
| warm-up | 4.27 | 116.2 | pipelines/wait_ms 1623, pipelines/compile_ms 153, pipelines/front_ms 144, render/thread/ms 115, render/busy/ms 85.20, main/busy/ms 58.95, stream/drive/ms 33.07, render/present/ms 29.62, pipelines/driver_ms 9.03, terrain/update/ms 5.11, render/graph/ms 3.87, stream/tick/ms 1.87 |
| warm-up | 4.34 | 35.9 | pipelines/wait_ms 5398, pipelines/compile_ms 153, pipelines/driver_ms 86.60, pipelines/front_ms 65.92, render/thread/ms 51.76, render/busy/ms 49.71, main/busy/ms 33.47, render/graph/ms 8.67, stream/drive/ms 4.12, render/present/ms 1.74, stream/tick/ms 1.42, terrain/update/ms 0.99 |
| warm-up | 4.50 | 67.7 | pipelines/wait_ms 3242, pipelines/compile_ms 152, pipelines/front_ms 143, main/busy/ms 63.82, render/thread/ms 46.75, stream/drive/ms 44.80, render/busy/ms 44.13, pipelines/driver_ms 8.90, render/graph/ms 7.32, stream/spawn/ms 2.52, render/present/ms 2.37, stream/tick/ms 1.37 |
| warm-up | 4.64 | 115.9 | pipelines/wait_ms 17.8 k, pipelines/compile_ms 153, pipelines/front_ms 139, main/busy/ms 102, render/thread/ms 56.79, stream/drive/ms 55.73, render/busy/ms 55.47, pipelines/driver_ms 13.55, render/graph/ms 6.09, terrain/update/ms 3.82, stream/spawn/ms 2.08, render/present/ms 1.14 |
| warm-up | 4.70 | 59.4 | pipelines/wait_ms 14.3 k, pipelines/compile_ms 123, pipelines/front_ms 112, render/thread/ms 92.41, render/busy/ms 91.08, main/busy/ms 79.16, stream/drive/ms 32.19, pipelines/driver_ms 11.12, render/graph/ms 6.20, terrain/update/ms 3.74, render/present/ms 1.13, stream/activate/ms 1.10 |
| warm-up | 5.86 | 35.2 | main/busy/ms 33.19, render/thread/ms 29.90, render/busy/ms 28.56, stream/drive/ms 13.42, render/graph/ms 4.62, stream/spawn/ms 2.13, render/present/ms 1.06, terrain/update/ms 0.84, stream/tick/ms 0.79, terrain/select/ms 0.72, bigspace/total/ms 0.70, bigspace/high_precision/ms 0.29 |
| warm-up | 6.02 | 35.1 | render/thread/ms 52.92, render/busy/ms 51.53, main/busy/ms 30.21, stream/drive/ms 8.09, render/graph/ms 7.17, bigspace/total/ms 1.82, stream/spawn/ms 1.80, bigspace/high_precision/ms 1.30, render/present/ms 1.15, stream/tick/ms 0.77, terrain/update/ms 0.68, terrain/select/ms 0.59 |
| measure | 0.36 | 35.2 | pipelines/wait_ms 4202, render/thread/ms 37.29, render/busy/ms 36.15, main/busy/ms 34.35, render/graph/ms 15.29, pipelines/compile_ms 10.06, pipelines/front_ms 9.33, physics/insert/ms 1.21, render/present/ms 0.93, bigspace/total/ms 0.86, anim/evaluate_cpu/ms 0.78, pipelines/driver_ms 0.73 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_dlssperf\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=performance
- before the run: GPU 0%, CPU 6%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **75%** (min 50%) · 2578 MHz (min 2565) · memory 8001 MHz · 77.4 W · 75 °C (max 81) · VRAM 5505 MiB · throttled 100% of samples
