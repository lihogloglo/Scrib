# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4818 frames over 65.6 s · **median 13.15 ms (76 fps)** · mean 13.63 · p95 16.82 · p99 18.98 (1% low 53 fps) · worst 38.91 ms · 33 spikes over 1.5× median · 20.1 stutters/min (work 19.2/min)
- warm-up: 710 frames over 13.3 s, worst 2889.9 ms
- hitches (> 32.9 ms): **3** measured, 20 in warm-up
- timed GPU passes: 7.50 ms of a 13.63 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.15 ms, limit 11 ms
- **FAIL** frame_ms: 22 stutters (> 1.5x moving median of 60) in 1.09 min = 20.11/min, limit 1/min
- **FAIL** worst frame 38.91 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.068 | 2.070 | 2.104 | 1.00 | 0.179 |
| main_opaque_pass_3d | 1.649 | 1.650 | 2.043 | 1.00 | 0.064 |
| early prepass | 0.523 | 0.524 | 0.748 | 1.00 | 0.103 |
| bloom | 0.320 | 0.320 | 0.326 | 1.00 | 0.064 |
| atmosphere_luts | 0.313 | 0.313 | 0.371 | 1.00 | 0.004 |
| water_velocity | 0.273 | 0.273 | 0.353 | 1.00 | 0.002 |
| uniform_allocation | 0.257 | 0.257 | 0.265 | 1.00 | 0.047 |
| water_sspr | 0.209 | 0.209 | 0.210 | 1.00 | 0.002 |
| bin_unpacking | 0.196 | 0.196 | 0.212 | 1.00 | 0.031 |
| ssao | 0.192 | 0.192 | 0.194 | 1.00 | 0.007 |
| auxiliary_views | 0.162 | 0.162 | 0.640 | 1.00 | 0.148 |
| atmos_froxel_inject | 0.148 | 0.148 | 0.150 | 1.00 | 0.002 |
| clouds_trace | 0.141 | 0.141 | 0.229 | 1.00 | 0.001 |
| water_fft | 0.130 | 0.131 | 0.136 | 1.00 | 0.003 |
| upscaling | 0.119 | 0.119 | 0.126 | 1.00 | 0.004 |
| clouds_composite | 0.108 | 0.108 | 0.110 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.088 | 0.088 | 0.167 | 1.00 | 0.008 |
| tonemapping | 0.087 | 0.087 | 0.091 | 1.00 | 0.003 |
| clustering | 0.065 | 0.065 | 0.068 | 1.00 | 0.020 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.419 | 0.421 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.048 | 0.057 | 0.057 | 0.84 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.049 | 1.00 | 0.005 |
| water_snapshot | 0.032 | 0.032 | 0.035 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.225 | 0.232 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.026 | 0.028 | 1.00 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.023 | 1.00 | 0.006 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.032 | 1.00 | 0.108 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 97.185 | 0.000 | 27421.15 | 468237.7 |
| render/thread/ms | 12.359 | 15.380 | 35.64 | 59544.2 |
| render/busy/ms | 11.790 | 14.703 | 34.75 | 56805.8 |
| main/busy/ms | 9.695 | 12.765 | 29.21 | 46708.4 |
| render/graph/ms | 7.303 | 9.218 | 24.19 | 35185.9 |
| pipelines/compile_ms | 6.816 | 0.000 | 2796.64 | 32841.3 |
| pipelines/driver_ms | 6.014 | 0.000 | 2650.85 | 28975.8 |
| pipelines/front_ms | 0.802 | 0.000 | 272.62 | 3865.5 |
| render/present/ms | 0.478 | 0.563 | 4.26 | 2301.8 |
| bigspace/total/ms | 0.450 | 0.680 | 4.57 | 2169.1 |
| terrain/update/ms | 0.293 | 0.368 | 1.81 | 1413.1 |
| controller/feed/ms | 0.263 | 0.339 | 0.63 | 1265.8 |
| terrain/select/ms | 0.261 | 0.327 | 0.93 | 1255.6 |
| stream/drive/ms | 0.250 | 0.362 | 20.73 | 1205.8 |
| far/select/ms | 0.195 | 0.268 | 0.57 | 938.1 |
| stream/tick/ms | 0.189 | 0.264 | 0.61 | 910.6 |
| bigspace/low_precision/ms | 0.169 | 0.282 | 4.02 | 815.7 |
| anim/evaluate_cpu/ms | 0.153 | 0.201 | 1.64 | 737.3 |
| anim/evaluate/ms | 0.127 | 0.258 | 2.89 | 613.8 |
| controller/move/ms | 0.125 | 0.237 | 0.62 | 603.5 |
| bigspace/high_precision/ms | 0.106 | 0.189 | 4.21 | 512.7 |
| render/acquire/ms | 0.091 | 0.117 | 0.38 | 436.6 |
| physics/insert/ms | 0.075 | 0.488 | 1.22 | 363.3 |
| physics/step/ms | 0.060 | 0.091 | 2.61 | 288.4 |
| weather/map/ms | 0.024 | 0.034 | 0.21 | 116.3 |
| anim/apply/ms | 0.010 | 0.020 | 0.13 | 50.6 |
| weather/eval/ms | 0.010 | 0.014 | 0.15 | 48.7 |
| controller/avoid/ms | 0.010 | 0.013 | 0.23 | 48.2 |
| script/update/ms | 0.007 | 0.009 | 0.09 | 33.5 |
| stream/despawn/ms | 0.006 | 0.000 | 1.50 | 30.1 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 25.1 |
| script/input/ms | 0.005 | 0.006 | 0.08 | 24.8 |
| script/ui/ms | 0.005 | 0.007 | 0.04 | 22.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 1.31 | 16.0 |
| controller/write/ms | 0.003 | 0.004 | 0.02 | 15.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.06 | 4.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.7 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 0 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 30 | 17 |
| controller/movers/count | count | 19 | 22 | 17 |
| crowd/c0/count | count | 25 | 42 | 13 |
| crowd/c0/joints/count | count | 1236 | 2218 | 613 |
| crowd/c0/meshes/count | count | 349 | 633 | 155 |
| crowd/c1/count | count | 147 | 156 | 148 |
| crowd/c1/joints/count | count | 5501 | 5983 | 5583 |
| crowd/c1/meshes/count | count | 1163 | 1297 | 1186 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 222 |
| crowd/joints_extracted/count | count | 4262 | 8931 | 6135 |
| crowd/joints_uploaded/count | count | 4243 | 6643 | 6135 |
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
| frame/work/ms | ms | 11.43 | 34.75 | 10.51 |
| gpu/timed/ms | ms | 8.22 | 11.62 | 8.46 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 802 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 802 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d/bins | count | 1287 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3380 | 5143 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1287 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3380 | 5143 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 56 | 278 | 47 |
| phases/shadow/bins | count | 4769 | 5777 | 2207 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.5 k | 13.2 k | 5655 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 248 | 314 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2631 | 3268 | 2346 |
| physics/items | count | 162 | 476 | 19 |
| physics/pending | count | 0 | 1421 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 220 | 226 | 226 |
| pipelines/waiting | count | 0 | 48 | 0 |
| preprocess/bind_groups | count | 11 | 753 | 6 |
| preprocess/bind_groups_created | count | 11 | 753 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 258 | 376 | 251 |
| process/cpu_usage | % | 24.82 | 51.92 | 24.39 |
| process/mem_usage | GiB | 5.25 | 5.36 | 5.36 |
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
| started_load_count |  | 4769 | 4860 | 4860 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
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
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.34 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.17 | 57.25 | 27.02 |
| system/mem_usage | % | 70.42 | 70.78 | 70.68 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 125 | 344 | 117 |
| views/all/bins | count | 7309 | 8611 | 5687 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.9 k | 25.7 k | 13.2 k |
| views/all/pipelines | count | 309 | 374 | 113 |
| views/all/pipelines_distinct | count | 102 | 109 | 91 |
| views/all/work_items | count | 17.1 k | 23.4 k | 13.6 k |
| views/cascade/batch_sets | count | 55 | 72 | 47 |
| views/cascade/bins | count | 2588 | 3053 | 2207 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7993 | 10.2 k | 5655 |
| views/cascade/pipelines | count | 55 | 72 | 47 |
| views/cascade/work_items | count | 7993 | 10.2 k | 5655 |
| views/lamp_face/batch_sets | count | 0 | 222 | 0 |
| views/lamp_face/bins | count | 2142 | 2859 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2401 | 3153 | 0 |
| views/lamp_face/pipelines | count | 191 | 247 | 0 |
| views/lamp_face/work_items | count | 0 | 2915 | 0 |
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 2864 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8370 | 13.1 k | 7594 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8536 | 13.3 k | 7902 |
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
| anim/bones_written/count | count | 50.45 | 253 | 243.1 k |
| anim/events/count | count | 0.10 | 3 | 497 |
| assets/Image/added | count | 0.13 | 43 | 622 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.87 | 152 | 4191 |
| assets/Mesh/removed | count | 1.01 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 501 |
| assets/StandardMaterial/modified | count | 4.50 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 159 | 2379 | 768.3 k |
| ecs/entities_delta | count | 1.55 | 1562 | 7460 |
| physics/inserts | count | 0.75 | 134 | 3601 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.48 | 145 | 2335 |
| physics/steps | count | 0.82 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 3 | 12 |
| pipelines/created | count | 0.01 | 3 | 26 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 19 | 378 |
| stream/upload/bytes | bytes | 7026 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1540 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 32.9 ms; slowest 23, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.89 | 2889.9 | main/busy/ms 2807, surface/advance_cpu_ms 68.95, stream/drive/ms 1.62, surface/cell_upload_ms 1.17, stream/tick/ms 1.01, bigspace/total/ms 0.61, bigspace/high_precision/ms 0.44, weather/map/ms 0.41, script/input/ms 0.19, far/select/ms 0.15, weather/eval/ms 0.04, script/update/ms 0.02 |
| warm-up | 3.67 | 778.2 | main/busy/ms 2884, render/thread/ms 740, render/busy/ms 572, render/acquire/ms 163, surface/advance_cpu_ms 68.95, render/graph/ms 11.07, render/present/ms 5.71, stream/drive/ms 4.36, stream/tick/ms 2.28, terrain/update/ms 1.24, surface/cell_upload_ms 1.17, bigspace/total/ms 0.90 |
| warm-up | 3.88 | 209.7 | pipelines/wait_ms 1158, render/thread/ms 755, render/busy/ms 585, pipelines/compile_ms 219, pipelines/front_ms 200, render/acquire/ms 163, main/busy/ms 96.48, pipelines/driver_ms 18.18, terrain/update/ms 17.98, render/graph/ms 14.24, render/present/ms 6.54, stream/drive/ms 3.60 |
| warm-up | 4.14 | 98.1 | pipelines/compile_ms 247, pipelines/front_ms 229, main/busy/ms 73.98, render/thread/ms 67.83, render/busy/ms 66.07, pipelines/driver_ms 18.55, render/graph/ms 5.07, stream/drive/ms 2.34, stream/tick/ms 1.85, render/present/ms 1.50, terrain/update/ms 0.87, weather/map/ms 0.79 |
| warm-up | 4.30 | 71.3 | main/busy/ms 55.82, stream/drive/ms 44.59, render/thread/ms 43.83, render/busy/ms 42.37, render/graph/ms 3.06, render/present/ms 1.27, stream/tick/ms 0.96, terrain/update/ms 0.77, terrain/select/ms 0.66, physics/insert/ms 0.60, stream/activate/ms 0.59, stream/spawn/ms 0.24 |
| warm-up | 4.36 | 37.4 | pipelines/wait_ms 1121, pipelines/compile_ms 213, pipelines/front_ms 194, render/thread/ms 41.91, render/busy/ms 39.69, main/busy/ms 32.34, pipelines/driver_ms 18.41, stream/drive/ms 18.30, render/graph/ms 5.56, render/present/ms 1.88, stream/tick/ms 1.48, stream/spawn/ms 1.38 |
| warm-up | 4.39 | 34.8 | pipelines/wait_ms 1759, pipelines/compile_ms 74.33, pipelines/front_ms 65.06, render/thread/ms 50.30, render/busy/ms 47.98, main/busy/ms 28.35, pipelines/driver_ms 9.27, stream/drive/ms 6.08, render/graph/ms 5.18, stream/spawn/ms 2.79, stream/tick/ms 2.02, render/present/ms 2.02 |
| warm-up | 4.44 | 49.3 | pipelines/wait_ms 918, render/thread/ms 56.85, render/busy/ms 54.67, main/busy/ms 32.64, pipelines/compile_ms 25.66, pipelines/front_ms 20.40, stream/drive/ms 5.85, render/graph/ms 5.41, pipelines/driver_ms 5.26, stream/spawn/ms 2.67, render/present/ms 1.95, stream/tick/ms 1.59 |
| warm-up | 4.49 | 45.8 | pipelines/wait_ms 477, pipelines/compile_ms 109, pipelines/front_ms 96.28, render/thread/ms 63.29, render/busy/ms 61.65, main/busy/ms 34.12, pipelines/driver_ms 12.28, stream/drive/ms 4.68, render/graph/ms 4.30, stream/spawn/ms 1.60, render/present/ms 1.43, stream/tick/ms 1.22 |
| warm-up | 4.54 | 49.5 | pipelines/wait_ms 5432, pipelines/compile_ms 187, pipelines/front_ms 167, render/thread/ms 72.54, render/busy/ms 70.94, main/busy/ms 63.46, stream/drive/ms 31.52, pipelines/driver_ms 19.40, render/graph/ms 4.99, stream/spawn/ms 2.47, render/present/ms 1.35, terrain/update/ms 1.02 |
| warm-up | 4.80 | 92.8 | main/busy/ms 76.95, stream/drive/ms 52.35, render/thread/ms 46.01, render/busy/ms 43.91, render/graph/ms 6.28, stream/spawn/ms 3.84, terrain/update/ms 3.54, render/present/ms 1.82, stream/tick/ms 1.42, terrain/select/ms 0.54, bigspace/total/ms 0.49, stream/upload/ms 0.34 |
| warm-up | 4.92 | 91.4 | pipelines/wait_ms 2707, pipelines/compile_ms 233, pipelines/front_ms 209, main/busy/ms 73.82, render/thread/ms 52.66, render/busy/ms 51.52, stream/drive/ms 49.49, pipelines/driver_ms 23.76, render/graph/ms 4.56, stream/spawn/ms 2.17, terrain/update/ms 1.06, stream/activate/ms 1.05 |
| warm-up | 4.98 | 41.9 | main/busy/ms 44.87, render/thread/ms 34.15, render/busy/ms 32.56, stream/drive/ms 30.79, render/graph/ms 4.52, render/present/ms 1.32, stream/activate/ms 1.11, stream/tick/ms 0.73, terrain/update/ms 0.68, terrain/select/ms 0.55, physics/insert/ms 0.54, render/acquire/ms 0.27 |
| warm-up | 5.03 | 36.3 | pipelines/wait_ms 6300, pipelines/compile_ms 142, pipelines/front_ms 128, main/busy/ms 37.86, stream/drive/ms 24.66, render/thread/ms 16.43, render/busy/ms 15.28, pipelines/driver_ms 13.92, render/graph/ms 3.80, stream/activate/ms 1.43, render/present/ms 0.97, stream/spawn/ms 0.91 |
| warm-up | 5.71 | 39.6 | pipelines/wait_ms 6633, pipelines/compile_ms 65.29, main/busy/ms 45.18, render/thread/ms 42.59, render/busy/ms 40.78, pipelines/front_ms 37.32, pipelines/driver_ms 27.97, render/graph/ms 10.52, terrain/update/ms 5.60, stream/drive/ms 1.82, render/present/ms 1.44, stream/tick/ms 0.82 |
| warm-up | 6.20 | 49.0 | main/busy/ms 37.84, render/thread/ms 33.15, render/busy/ms 31.85, stream/drive/ms 17.92, render/graph/ms 5.51, stream/spawn/ms 2.17, stream/tick/ms 1.27, render/present/ms 1.12, terrain/update/ms 0.98, terrain/select/ms 0.82, bigspace/total/ms 0.57, bigspace/high_precision/ms 0.27 |
| warm-up | 6.38 | 33.1 | render/thread/ms 46.43, render/busy/ms 45.01, main/busy/ms 29.15, render/graph/ms 6.62, stream/drive/ms 3.53, stream/spawn/ms 1.60, render/present/ms 1.21, stream/tick/ms 1.02, bigspace/total/ms 0.73, terrain/update/ms 0.69, terrain/select/ms 0.60, physics/insert/ms 0.43 |
| warm-up | 8.75 | 40.2 | main/busy/ms 60.82, render/thread/ms 31.55, render/busy/ms 29.80, render/graph/ms 13.73, render/present/ms 1.56, stream/drive/ms 1.06, physics/insert/ms 0.91, anim/evaluate_cpu/ms 0.88, bigspace/total/ms 0.85, terrain/update/ms 0.59, far/select/ms 0.56, terrain/select/ms 0.52 |
| warm-up | 8.79 | 44.4 | pipelines/wait_ms 3801, main/busy/ms 51.73, pipelines/compile_ms 50.97, render/thread/ms 42.23, render/busy/ms 40.66, pipelines/driver_ms 37.99, pipelines/front_ms 12.97, render/graph/ms 11.14, bigspace/total/ms 1.39, render/present/ms 1.39, physics/insert/ms 1.29, stream/drive/ms 1.18 |
| warm-up | 13.25 | 34.9 | pipelines/wait_ms 16.5 k, pipelines/compile_ms 84.70, pipelines/driver_ms 59.20, render/thread/ms 37.37, render/busy/ms 35.81, main/busy/ms 33.31, pipelines/front_ms 25.50, render/graph/ms 14.84, render/present/ms 1.37, bigspace/total/ms 0.88, physics/insert/ms 0.69, terrain/update/ms 0.68 |
| measure | 2.18 | 34.5 | render/thread/ms 46.50, render/busy/ms 45.02, render/graph/ms 28.36, main/busy/ms 19.82, render/present/ms 1.28, physics/insert/ms 0.75, bigspace/total/ms 0.59, terrain/update/ms 0.59, terrain/select/ms 0.53, far/select/ms 0.50, stream/drive/ms 0.43, stream/tick/ms 0.36 |
| measure | 2.74 | 36.8 | pipelines/wait_ms 533, render/thread/ms 54.26, render/busy/ms 52.75, pipelines/compile_ms 26.29, main/busy/ms 22.27, render/graph/ms 21.15, pipelines/front_ms 13.74, pipelines/driver_ms 12.55, render/present/ms 1.29, physics/insert/ms 1.20, bigspace/total/ms 0.79, terrain/update/ms 0.62 |
| measure | 59.05 | 38.9 | render/thread/ms 39.07, render/busy/ms 37.74, main/busy/ms 22.29, render/graph/ms 15.99, bigspace/total/ms 1.53, render/present/ms 1.09, bigspace/low_precision/ms 0.90, terrain/update/ms 0.75, controller/feed/ms 0.75, stream/drive/ms 0.69, terrain/select/ms 0.67, stream/tick/ms 0.60 |

## Run

- `C:\Users\leogloglo\AppData\Local\Temp\depth-keys-off\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\dkeys_off\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 4% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **71%** (min 47%) · 2580 MHz (min 2580) · memory 8001 MHz · 71.9 W · 74 °C (max 79) · VRAM 5613 MiB · throttled 100% of samples
