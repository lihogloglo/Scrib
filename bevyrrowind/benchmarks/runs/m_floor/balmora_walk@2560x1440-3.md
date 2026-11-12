# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 9011 frames over 65.6 s · **median 7.27 ms (138 fps)** · mean 7.29 · p95 8.36 · p99 8.99 (1% low 111 fps) · worst 66.61 ms · 7 spikes over 1.5× median · 11.0 stutters/min (work 3.7/min)
- warm-up: 337 frames over 3.4 s, worst 585.0 ms
- hitches (> 25.0 ms): **2** measured, 4 in warm-up
- timed GPU passes: 6.10 ms of a 7.29 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 7.27 ms, limit 11 ms
- **FAIL** frame_ms: 12 stutters (> 1.5x moving median of 60) in 1.09 min = 10.97/min, limit 1/min
- **FAIL** worst frame 66.61 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.212 | 2.221 | 2.713 | 1.00 | 0.177 |
| main_opaque_pass_3d | 1.298 | 1.304 | 1.901 | 1.00 | 0.022 |
| atmosphere_luts | 0.364 | 0.366 | 0.553 | 1.00 | 0.003 |
| bloom | 0.339 | 0.340 | 0.487 | 1.00 | 0.058 |
| early prepass | 0.337 | 0.339 | 0.586 | 1.00 | 0.034 |
| ssao | 0.222 | 0.223 | 0.221 | 1.00 | 0.007 |
| clouds_trace | 0.151 | 0.151 | 0.247 | 1.00 | 0.001 |
| water_fft | 0.132 | 0.133 | 0.127 | 1.00 | 0.002 |
| atmos_froxel_inject | 0.099 | 0.100 | 0.104 | 1.00 | 0.001 |
| tonemapping | 0.089 | 0.090 | 0.091 | 1.00 | 0.003 |
| water_velocity | 0.088 | 0.088 | 0.086 | 1.00 | 0.001 |
| main_transparent_pass_3d | 0.082 | 0.083 | 0.254 | 1.00 | 0.013 |
| upscaling | 0.081 | 0.081 | 0.118 | 1.00 | 0.004 |
| clustering | 0.068 | 0.069 | 0.075 | 1.00 | 0.019 |
| clouds_composite | 0.065 | 0.065 | 0.063 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.055 | 0.440 | 0.755 | 0.12 | 0.001 |
| render_sky | 0.054 | 0.054 | 0.053 | 1.00 | 0.002 |
| uniform_allocation | 0.042 | 0.042 | 0.051 | 1.00 | 0.012 |
| clouds_resolve | 0.035 | 0.035 | 0.039 | 1.00 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.003 |
| surface_trail | 0.030 | 0.060 | 0.057 | 0.50 | 0.002 |
| lightprobe_radiance_map | 0.030 | 0.239 | 0.240 | 0.12 | 0.004 |
| bin_unpacking | 0.029 | 0.029 | 0.031 | 1.00 | 0.010 |
| atmos_froxel_integrate | 0.025 | 0.025 | 0.031 | 1.00 | 0.001 |
| water_snapshot | 0.024 | 0.025 | 0.024 | 1.00 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.024 | 0.024 | 0.024 | 1.00 | 0.006 |
| per_view_shadow_pass | 0.024 | 0.024 | 0.034 | 1.00 | 0.007 |
| clouds_fog_light | 0.013 | 0.013 | 0.013 | 1.00 | 0.002 |
| atmosphere_environment | 0.012 | 0.012 | 0.014 | 1.00 | 0.003 |
| night_sky | 0.008 | 0.008 | 0.016 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.016 | 0.014 | 0.12 | 0.001 |
| lightprobe_copy | 0.002 | 0.016 | 0.016 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.01 | 0.004 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.570 | 7.635 | 65.80 | 59201.5 |
| render/busy/ms | 4.882 | 5.652 | 65.12 | 43990.6 |
| main/busy/ms | 4.775 | 5.405 | 57.29 | 43029.0 |
| render/graph/ms | 2.931 | 3.602 | 63.20 | 26410.5 |
| render/acquire/ms | 1.229 | 2.250 | 3.47 | 11075.4 |
| render/present/ms | 0.459 | 0.548 | 17.21 | 4135.4 |
| terrain/update/ms | 0.297 | 0.383 | 2.15 | 2680.8 |
| terrain/select/ms | 0.265 | 0.341 | 0.85 | 2386.1 |
| bigspace/total/ms | 0.247 | 0.550 | 1.71 | 2221.5 |
| stream/drive/ms | 0.212 | 0.309 | 2.09 | 1912.1 |
| stream/tick/ms | 0.178 | 0.257 | 0.64 | 1603.8 |
| far/select/ms | 0.170 | 0.239 | 0.87 | 1529.4 |
| bigspace/high_precision/ms | 0.129 | 0.303 | 1.62 | 1166.4 |
| bigspace/low_precision/ms | 0.052 | 0.157 | 1.30 | 469.3 |
| weather/map/ms | 0.025 | 0.034 | 0.24 | 225.2 |
| physics/insert/ms | 0.020 | 0.163 | 0.96 | 183.6 |
| weather/eval/ms | 0.010 | 0.015 | 0.32 | 93.7 |
| physics/step/ms | 0.010 | 0.026 | 0.55 | 92.1 |
| script/update/ms | 0.007 | 0.009 | 0.16 | 63.3 |
| script/input/ms | 0.005 | 0.006 | 0.25 | 44.5 |
| script/ui/ms | 0.003 | 0.004 | 0.18 | 27.5 |
| controller/move/ms | 0.002 | 0.003 | 0.05 | 18.1 |
| anim/evaluate/ms | 0.002 | 0.003 | 0.02 | 17.0 |
| stream/deactivate/ms | 0.001 | 0.000 | 0.64 | 13.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.13 | 8.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.10 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.7 |
| anim/apply/ms | 0.001 | 0.001 | 0.02 | 4.6 |
| stream/spawn/ms | 0.000 | 0.000 | 0.54 | 3.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.9 |
| controller/feed/ms | 0.000 | 0.001 | 0.12 | 2.8 |
| stream/despawn/ms | 0.000 | 0.000 | 0.17 | 2.6 |
| controller/avoid/ms | 0.000 | 0.000 | 0.01 | 1.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.13 | 0.7 |
| controller/write/ms | 0.000 | 0.000 | 0.02 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| anim/evaluate_cpu/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 0 | 0 | 0 |
| anim/characters/count | count | 0 | 0 | 0 |
| anim/evaluated/count | count | 0 | 0 | 0 |
| controller/movers/count | count | 0 | 0 | 0 |
| crowd/c0/count | count | 0 | 0 | 0 |
| crowd/c0/joints/count | count | 0 | 0 | 0 |
| crowd/c0/meshes/count | count | 0 | 0 | 0 |
| crowd/c1/count | count | 0 | 0 | 0 |
| crowd/c1/joints/count | count | 0 | 0 | 0 |
| crowd/c1/meshes/count | count | 0 | 0 | 0 |
| crowd/c2/count | count | 0 | 0 | 0 |
| crowd/c2/joints/count | count | 0 | 0 | 0 |
| crowd/c2/meshes/count | count | 0 | 0 | 0 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 0 | 0 | 0 |
| crowd/joints_extracted/count | count | 0 | 0 | 0 |
| crowd/joints_uploaded/count | count | 0 | 0 | 0 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 8063 | 8416 | 7948 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 6.84 | 65.12 | 7.31 |
| gpu/timed/ms | ms | 6.84 | 8.15 | 7.31 |
| mesh_allocator_allocations |  meshes | 9048 | 9822 | 8524 |
| mesh_allocator_slabs |  slabs | 11 | 12 | 11 |
| mesh_allocator_slabs_size |  bytes | 298.98 M | 300.03 M | 298.98 M |
| phases/alpha_mask3d/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d/bins | count | 125 | 368 | 361 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 125 | 368 | 361 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/bins | count | 125 | 368 | 361 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 125 | 368 | 361 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d/bins | count | 147 | 428 | 419 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 327 | 617 | 603 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d_prepass/bins | count | 147 | 428 | 419 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 327 | 617 | 603 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 5 | 7 | 6 |
| phases/shadow/bins | count | 85 | 108 | 75 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 575 | 615 | 572 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 5 | 7 | 6 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2727 | 3248 | 2329 |
| physics/items | count | 248 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 102 | 102 | 102 |
| pipelines/waiting | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 2 | 38 | 2 |
| preprocess/bind_groups_created | count | 2 | 38 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 51 | 58 | 54 |
| process/cpu_usage | % | 23.60 | 47.57 | 38.16 |
| process/mem_usage | GiB | 2.15 | 2.18 | 2.16 |
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
| started_load_count |  | 3013 | 3047 | 3047 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 0 | 0 | 0 |
| statics/material_classes | count | 0 | 0 | 0 |
| statics/material_keys | count | 0 | 0 | 0 |
| statics/material_pipelines | count | 0 | 0 | 0 |
| statics/materials | count | 0 | 0 | 0 |
| statics/materials_bound | count | 0 | 0 | 0 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.02 | 0.04 | 0.02 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 758 | 851 | 683 |
| stream/oldest_request/s | s | 0 | 0.13 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.73 | 45.20 | 34.51 |
| system/mem_usage | % | 52.45 | 52.56 | 52.50 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 17 | 19 | 18 |
| views/all/bins | count | 631 | 1667 | 1635 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 1501 | 2536 | 2500 |
| views/all/pipelines | count | 13 | 15 | 14 |
| views/all/pipelines_distinct | count | 11 | 11 | 11 |
| views/all/work_items | count | 1602 | 2746 | 2701 |
| views/cascade/batch_sets | count | 5 | 7 | 6 |
| views/cascade/bins | count | 85 | 108 | 75 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 575 | 615 | 572 |
| views/cascade/pipelines | count | 5 | 7 | 6 |
| views/cascade/work_items | count | 575 | 615 | 572 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 12 | 12 | 12 |
| views/main/bins | count | 542 | 1586 | 1560 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 906 | 1960 | 1928 |
| views/main/pipelines | count | 8 | 8 | 8 |
| views/main/work_items | count | 1008 | 2172 | 2129 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 3200 | 3212 | 3203 |
| world/point_lights | count | 0 | 0 | 0 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0 | 0 | 0 |
| assets/Image/added | count | 0.01 | 18 | 125 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.23 | 195 | 2087 |
| assets/Mesh/removed | count | 0.49 | 1162 | 4426 |
| assets/StandardMaterial/added | count | 0.01 | 18 | 128 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 137 | -32 |
| physics/inserts | count | 0.26 | 111 | 2385 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.26 | 145 | 2335 |
| physics/steps | count | 0.44 | 3 | 3938 |
| physics/steps_dropped | count | 0.00 | 1 | 1 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.02 | 21 | 219 |
| stream/deactivate/items | count | 0.02 | 21 | 223 |
| stream/despawn/items | count | 0.02 | 21 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.04 | 27 | 351 |
| stream/upload/bytes | bytes | 3757 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 16 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 824 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 585.0 | main/busy/ms 563, surface/advance_cpu_ms 67.65, surface/cell_upload_ms 0.71, stream/drive/ms 0.27, weather/map/ms 0.14, bigspace/total/ms 0.09, stream/tick/ms 0.06, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.03, far/select/ms 0.02, script/input/ms 0.01 |
| warm-up | 0.96 | 371.1 | main/busy/ms 600, render/thread/ms 332, render/busy/ms 253, render/acquire/ms 77.09, surface/advance_cpu_ms 67.65, render/graph/ms 6.14, render/present/ms 1.95, stream/drive/ms 0.99, surface/cell_upload_ms 0.71, terrain/update/ms 0.62, stream/tick/ms 0.47, terrain/select/ms 0.42 |
| warm-up | 1.03 | 72.7 | pipelines/wait_ms 509, render/thread/ms 343, render/busy/ms 263, pipelines/compile_ms 79.09, render/acquire/ms 77.28, pipelines/front_ms 70.73, main/busy/ms 52.56, terrain/update/ms 22.99, render/graph/ms 9.42, pipelines/driver_ms 8.36, render/present/ms 2.80, stream/drive/ms 1.60 |
| warm-up | 1.34 | 89.0 | main/busy/ms 60.76, render/thread/ms 51.50, render/busy/ms 49.97, render/graph/ms 4.15, stream/drive/ms 1.86, render/present/ms 1.34, terrain/update/ms 1.04, stream/tick/ms 1.00, terrain/select/ms 0.84, stream/spawn/ms 0.42, bigspace/total/ms 0.20, render/acquire/ms 0.19 |
| measure | 4.10 | 66.6 | render/thread/ms 72.89, render/busy/ms 70.58, render/graph/ms 66.84, main/busy/ms 61.93, render/acquire/ms 1.33, render/present/ms 0.99, terrain/update/ms 0.57, terrain/select/ms 0.50, stream/drive/ms 0.50, stream/tick/ms 0.43, bigspace/total/ms 0.41, far/select/ms 0.29 |
| measure | 56.33 | 25.1 | render/thread/ms 31.77, render/present/ms 17.83, render/busy/ms 11.16, main/busy/ms 10.17, render/graph/ms 6.97, render/acquire/ms 2.78, terrain/update/ms 0.62, terrain/select/ms 0.54, bigspace/total/ms 0.54, far/select/ms 0.48, stream/drive/ms 0.37, bigspace/high_precision/ms 0.34 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\m_floor\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=near
- before the run: GPU 3%, CPU 8%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **97%** (min 44%) · 2572 MHz (min 2565) · memory 8001 MHz · 101.0 W · 81 °C (max 84) · VRAM 3806 MiB · throttled 100% of samples
