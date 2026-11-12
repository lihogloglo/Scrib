# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 9201 frames over 65.6 s · **median 7.13 ms (140 fps)** · mean 7.13 · p95 8.15 · p99 8.66 (1% low 115 fps) · worst 27.68 ms · 5 spikes over 1.5× median · 5.5 stutters/min (work 3.7/min)
- warm-up: 332 frames over 3.3 s, worst 581.4 ms
- hitches (> 25.0 ms): **1** measured, 5 in warm-up
- timed GPU passes: 5.99 ms of a 7.13 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 7.13 ms, limit 11 ms
- **FAIL** frame_ms: 6 stutters (> 1.5x moving median of 60) in 1.09 min = 5.48/min, limit 1/min
- pass worst frame 27.68 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.178 | 2.186 | 2.466 | 1.00 | 0.177 |
| main_opaque_pass_3d | 1.285 | 1.289 | 1.838 | 1.00 | 0.019 |
| bloom | 0.335 | 0.336 | 0.494 | 1.00 | 0.058 |
| early prepass | 0.324 | 0.325 | 0.558 | 1.00 | 0.033 |
| atmosphere_luts | 0.314 | 0.315 | 0.505 | 1.00 | 0.003 |
| ssao | 0.217 | 0.218 | 0.217 | 1.00 | 0.007 |
| clouds_trace | 0.152 | 0.152 | 0.248 | 1.00 | 0.001 |
| water_fft | 0.130 | 0.131 | 0.127 | 1.00 | 0.002 |
| upscaling | 0.102 | 0.102 | 0.121 | 1.00 | 0.004 |
| atmos_froxel_inject | 0.099 | 0.099 | 0.105 | 1.00 | 0.001 |
| tonemapping | 0.093 | 0.093 | 0.094 | 1.00 | 0.003 |
| water_velocity | 0.086 | 0.086 | 0.086 | 1.00 | 0.001 |
| main_transparent_pass_3d | 0.078 | 0.079 | 0.259 | 1.00 | 0.013 |
| clustering | 0.066 | 0.066 | 0.074 | 1.00 | 0.019 |
| clouds_composite | 0.064 | 0.064 | 0.063 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.054 | 0.434 | 0.753 | 0.12 | 0.001 |
| render_sky | 0.053 | 0.053 | 0.053 | 1.00 | 0.002 |
| uniform_allocation | 0.041 | 0.041 | 0.050 | 1.00 | 0.010 |
| clouds_resolve | 0.034 | 0.034 | 0.039 | 1.00 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.030 | 0.236 | 0.236 | 0.12 | 0.003 |
| surface_trail | 0.029 | 0.060 | 0.057 | 0.49 | 0.002 |
| bin_unpacking | 0.028 | 0.029 | 0.030 | 1.00 | 0.009 |
| atmos_froxel_integrate | 0.024 | 0.025 | 0.031 | 1.00 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.024 | 0.024 | 0.024 | 1.00 | 0.005 |
| water_snapshot | 0.024 | 0.024 | 0.024 | 1.00 | 0.001 |
| per_view_shadow_pass | 0.023 | 0.023 | 0.031 | 1.00 | 0.006 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.008 | 0.008 | 0.016 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.014 | 0.016 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.01 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.422 | 7.446 | 26.81 | 59092.9 |
| main/busy/ms | 4.683 | 5.366 | 11.61 | 43092.0 |
| render/busy/ms | 4.615 | 5.426 | 10.90 | 42464.6 |
| render/graph/ms | 2.705 | 3.445 | 7.03 | 24892.6 |
| render/acquire/ms | 1.360 | 2.371 | 3.50 | 12514.1 |
| render/present/ms | 0.447 | 0.541 | 19.97 | 4114.2 |
| terrain/update/ms | 0.299 | 0.382 | 1.73 | 2752.1 |
| terrain/select/ms | 0.267 | 0.342 | 0.74 | 2457.1 |
| bigspace/total/ms | 0.241 | 0.503 | 1.63 | 2213.5 |
| stream/drive/ms | 0.213 | 0.299 | 1.88 | 1960.4 |
| stream/tick/ms | 0.181 | 0.248 | 0.68 | 1662.2 |
| far/select/ms | 0.166 | 0.229 | 0.63 | 1531.5 |
| bigspace/high_precision/ms | 0.128 | 0.291 | 1.54 | 1175.4 |
| bigspace/low_precision/ms | 0.052 | 0.167 | 1.29 | 479.3 |
| weather/map/ms | 0.025 | 0.034 | 0.30 | 229.2 |
| physics/insert/ms | 0.020 | 0.173 | 1.01 | 187.5 |
| weather/eval/ms | 0.010 | 0.015 | 0.29 | 96.0 |
| physics/step/ms | 0.007 | 0.016 | 0.52 | 62.0 |
| script/update/ms | 0.006 | 0.008 | 0.40 | 57.2 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 44.1 |
| script/ui/ms | 0.003 | 0.004 | 0.20 | 25.1 |
| controller/move/ms | 0.002 | 0.003 | 0.13 | 17.9 |
| anim/evaluate/ms | 0.002 | 0.002 | 0.10 | 17.4 |
| stream/deactivate/ms | 0.001 | 0.000 | 0.62 | 12.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 8.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.07 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.1 |
| anim/apply/ms | 0.000 | 0.001 | 0.02 | 4.5 |
| stream/spawn/ms | 0.000 | 0.000 | 0.19 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 3.0 |
| controller/feed/ms | 0.000 | 0.001 | 0.01 | 2.9 |
| stream/despawn/ms | 0.000 | 0.000 | 0.19 | 2.6 |
| controller/avoid/ms | 0.000 | 0.000 | 0.01 | 1.7 |
| stream/upload/ms | 0.000 | 0.000 | 0.02 | 0.6 |
| controller/write/ms | 0.000 | 0.000 | 0.00 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
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
| frame/work/ms | ms | 6.68 | 11.61 | 7.18 |
| gpu/timed/ms | ms | 6.67 | 7.92 | 7.18 |
| mesh_allocator_allocations |  meshes | 9048 | 9822 | 8524 |
| mesh_allocator_slabs |  slabs | 11 | 12 | 11 |
| mesh_allocator_slabs_size |  bytes | 298.98 M | 300.03 M | 298.98 M |
| phases/alpha_mask3d/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d/bins | count | 124 | 368 | 361 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 124 | 368 | 361 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/bins | count | 124 | 368 | 361 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 124 | 368 | 361 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d/bins | count | 147 | 429 | 419 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 327 | 617 | 603 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d_prepass/bins | count | 147 | 429 | 419 |
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
| preprocess/bind_groups_reused | count | 52 | 58 | 54 |
| process/cpu_usage | % | 22.95 | 43.27 | 26.79 |
| process/mem_usage | GiB | 2.17 | 2.20 | 2.18 |
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
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.02 | 0.04 | 0.02 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 758 | 851 | 683 |
| stream/oldest_request/s | s | 0 | 0.12 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.06 | 46.99 | 25.73 |
| system/mem_usage | % | 52.39 | 52.60 | 52.39 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 17 | 19 | 18 |
| views/all/bins | count | 629 | 1667 | 1635 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 1497 | 2536 | 2500 |
| views/all/pipelines | count | 13 | 15 | 14 |
| views/all/pipelines_distinct | count | 11 | 11 | 11 |
| views/all/work_items | count | 1598 | 2745 | 2701 |
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
| views/main/bins | count | 540 | 1586 | 1560 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 906 | 1960 | 1928 |
| views/main/pipelines | count | 8 | 8 | 8 |
| views/main/work_items | count | 1004 | 2171 | 2129 |
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
| assets/Mesh/removed | count | 0.48 | 1162 | 4426 |
| assets/StandardMaterial/added | count | 0.01 | 18 | 128 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 137 | -32 |
| physics/inserts | count | 0.26 | 106 | 2374 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.25 | 161 | 2335 |
| physics/steps | count | 0.43 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
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
| stream/upload/bytes | bytes | 3679 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 16 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 807 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 581.4 | main/busy/ms 560, surface/advance_cpu_ms 67.38, surface/cell_upload_ms 0.78, stream/drive/ms 0.38, weather/map/ms 0.20, stream/tick/ms 0.08, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.03, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.85 | 268.5 | main/busy/ms 594, render/thread/ms 230, render/busy/ms 157, render/acquire/ms 70.83, surface/advance_cpu_ms 67.38, render/graph/ms 5.26, render/present/ms 2.15, stream/drive/ms 1.21, surface/cell_upload_ms 0.78, terrain/update/ms 0.59, stream/tick/ms 0.58, terrain/select/ms 0.42 |
| warm-up | 0.99 | 141.1 | pipelines/wait_ms 1147, render/thread/ms 241, render/busy/ms 166, pipelines/compile_ms 162, pipelines/front_ms 149, render/acquire/ms 71.05, main/busy/ms 51.37, terrain/update/ms 19.32, pipelines/driver_ms 13.39, render/graph/ms 8.89, render/present/ms 3.22, stream/drive/ms 1.82 |
| warm-up | 1.23 | 82.8 | pipelines/compile_ms 193, pipelines/front_ms 171, main/busy/ms 60.51, render/thread/ms 47.73, render/busy/ms 45.87, pipelines/wait_ms 37.04, pipelines/driver_ms 21.73, render/graph/ms 3.75, stream/drive/ms 1.80, render/present/ms 1.55, stream/tick/ms 1.47, terrain/update/ms 0.94 |
| warm-up | 1.26 | 26.4 | render/thread/ms 29.24, render/busy/ms 27.18, main/busy/ms 17.10, render/graph/ms 2.85, stream/drive/ms 2.13, render/present/ms 1.84, stream/tick/ms 1.05, terrain/update/ms 0.79, terrain/select/ms 0.67, stream/spawn/ms 0.56, bigspace/total/ms 0.25, render/acquire/ms 0.22 |
| measure | 54.73 | 27.7 | render/thread/ms 34.52, render/present/ms 20.51, render/busy/ms 12.07, main/busy/ms 10.73, render/graph/ms 7.19, render/acquire/ms 1.93, terrain/update/ms 0.63, terrain/select/ms 0.56, stream/drive/ms 0.50, bigspace/total/ms 0.48, stream/tick/ms 0.43, far/select/ms 0.37 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\m_floor\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=near
- before the run: GPU 0%, CPU 7%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **97%** (min 94%) · 2568 MHz (min 2565) · memory 8001 MHz · 101.6 W · 80 °C (max 83) · VRAM 3805 MiB · throttled 100% of samples
