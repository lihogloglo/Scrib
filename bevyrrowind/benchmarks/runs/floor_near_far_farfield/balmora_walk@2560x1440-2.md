# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near,far,farfield · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 9427 frames over 65.6 s · **median 6.93 ms (144 fps)** · mean 6.96 · p95 7.88 · p99 8.29 (1% low 121 fps) · worst 10.78 ms · 1 spikes over 1.5× median · 0.9 stutters/min (work 0.9/min)
- warm-up: 867 frames over 7.2 s, worst 576.7 ms
- hitches (> 25.0 ms): **0** measured, 3 in warm-up
- timed GPU passes: 6.67 ms of a 6.96 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 6.93 ms, limit 11 ms
- pass frame_ms: 1 stutters (> 1.5x moving median of 60) in 1.09 min = 0.91/min, limit 1/min
- pass worst frame 10.78 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.170 | 2.467 | 0.176 |
| main_opaque_pass_3d | 1.091 | 1.520 | 0.009 |
| atmosphere_luts | 0.794 | 1.105 | 0.003 |
| lightprobe_irradiance_map | 0.434 | 0.665 | 0.001 |
| bloom | 0.333 | 0.341 | 0.057 |
| lightprobe_radiance_map | 0.238 | 0.352 | 0.003 |
| ssao | 0.218 | 0.221 | 0.007 |
| clouds_trace | 0.148 | 0.241 | 0.001 |
| water_fft | 0.127 | 0.126 | 0.002 |
| early prepass | 0.119 | 0.128 | 0.015 |
| atmos_froxel_inject | 0.102 | 0.108 | 0.001 |
| upscaling | 0.097 | 0.115 | 0.004 |
| tonemapping | 0.091 | 0.093 | 0.003 |
| water_velocity | 0.086 | 0.086 | 0.001 |
| main_transparent_pass_3d | 0.078 | 0.245 | 0.009 |
| clustering | 0.066 | 0.073 | 0.019 |
| clouds_composite | 0.065 | 0.065 | 0.002 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| clouds_resolve | 0.036 | 0.039 | 0.001 |
| clouds_shadow | 0.031 | 0.034 | 0.003 |
| atmos_froxel_integrate | 0.031 | 0.035 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| per_view_shadow_pass | 0.024 | 0.035 | 0.006 |
| water_mips | 0.023 | 0.024 | 0.002 |
| early_mesh_preprocessing | 0.022 | 0.022 | 0.005 |
| uniform_allocation | 0.015 | 0.015 | 0.006 |
| lightprobe_copy | 0.014 | 0.015 | 0.003 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| atmosphere_environment | 0.011 | 0.014 | 0.003 |
| bin_unpacking | 0.011 | 0.011 | 0.005 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| clouds_fog_light | 0.010 | 0.011 | 0.002 |
| night_sky | 0.008 | 0.016 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | 0.002 | 0.002 | 0.004 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.297 | 7.223 | 14.00 | 59363.8 |
| render/busy/ms | 4.430 | 5.158 | 10.03 | 41763.1 |
| main/busy/ms | 4.332 | 5.021 | 11.33 | 40837.2 |
| render/graph/ms | 2.733 | 3.383 | 6.24 | 25761.5 |
| render/acquire/ms | 1.426 | 2.341 | 3.52 | 13441.2 |
| render/present/ms | 0.441 | 0.546 | 1.92 | 4159.5 |
| terrain/update/ms | 0.302 | 0.386 | 2.01 | 2843.2 |
| terrain/select/ms | 0.268 | 0.343 | 1.00 | 2526.6 |
| bigspace/total/ms | 0.252 | 0.492 | 1.43 | 2378.3 |
| stream/drive/ms | 0.230 | 0.321 | 1.83 | 2172.5 |
| stream/tick/ms | 0.194 | 0.262 | 0.69 | 1826.1 |
| bigspace/high_precision/ms | 0.126 | 0.280 | 1.33 | 1189.6 |
| bigspace/low_precision/ms | 0.053 | 0.172 | 0.90 | 502.1 |
| weather/map/ms | 0.024 | 0.032 | 0.31 | 225.8 |
| physics/insert/ms | 0.016 | 0.000 | 1.02 | 152.4 |
| weather/eval/ms | 0.010 | 0.014 | 0.20 | 93.4 |
| physics/step/ms | 0.008 | 0.024 | 0.50 | 72.8 |
| script/update/ms | 0.006 | 0.008 | 0.21 | 58.2 |
| script/input/ms | 0.005 | 0.006 | 0.13 | 48.3 |
| script/ui/ms | 0.003 | 0.005 | 0.31 | 30.0 |
| anim/evaluate/ms | 0.002 | 0.003 | 0.33 | 19.3 |
| controller/move/ms | 0.002 | 0.003 | 0.14 | 17.6 |
| stream/deactivate/ms | 0.001 | 0.000 | 0.66 | 12.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 8.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.47 | 5.4 |
| anim/apply/ms | 0.001 | 0.001 | 0.12 | 5.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.19 | 3.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.8 |
| stream/despawn/ms | 0.000 | 0.000 | 0.17 | 2.6 |
| controller/feed/ms | 0.000 | 0.001 | 0.00 | 2.4 |
| controller/avoid/ms | 0.000 | 0.000 | 0.01 | 1.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.6 |
| controller/write/ms | 0.000 | 0.000 | 0.01 | 0.4 |
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
| ecs/entities | count | 10.9 k | 11.2 k | 10.8 k |
| frame/work/ms | ms | 6.70 | 11.33 | 6.72 |
| gpu/timed/ms | ms | 6.70 | 8.01 | 6.72 |
| mesh_allocator_allocations |  meshes | 11.7 k | 12.4 k | 11.1 k |
| mesh_allocator_slabs |  slabs | 10 | 11 | 10 |
| mesh_allocator_slabs_size |  bytes | 73.03 M | 74.08 M | 73.03 M |
| phases/alpha_mask3d/batch_sets | count | 0 | 0 | 0 |
| phases/alpha_mask3d/bins | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 0 | 0 | 0 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 0 | 0 | 0 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/bins | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 1 | 1 | 1 |
| phases/opaque3d/bins | count | 2 | 2 | 2 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 180 | 196 | 186 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 1 | 1 | 1 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 1 | 1 | 1 |
| phases/opaque3d_prepass/bins | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 180 | 196 | 186 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 1 | 1 | 1 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 3 | 3 | 3 |
| phases/shadow/bins | count | 6 | 6 | 6 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 497 | 535 | 503 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 3 | 3 | 3 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2738 | 3248 | 2329 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 92 | 92 | 92 |
| pipelines/waiting | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 1 | 7 | 2 |
| preprocess/bind_groups_created | count | 1 | 7 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 23 | 24 | 22 |
| process/cpu_usage | % | 23.08 | 37.86 | 24.15 |
| process/mem_usage | GiB | 2.10 | 2.12 | 2.11 |
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
| started_load_count |  | 2384 | 2409 | 2409 |
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
| stream/latency/s | s | 0.02 | 3.20 | 0.02 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 1188 | 1277 | 1116 |
| stream/oldest_request/s | s | 0 | 0.12 | 0 |
| stream/page_cache/bytes | bytes | 29.61 M | 29.61 M | 29.61 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1476 | 1483 | 1472 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 27.73 | 44.99 | 28.22 |
| system/mem_usage | % | 51.45 | 51.58 | 50.66 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 5 | 5 | 5 |
| views/all/bins | count | 10 | 10 | 10 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 857 | 918 | 875 |
| views/all/pipelines | count | 5 | 5 | 5 |
| views/all/pipelines_distinct | count | 3 | 3 | 3 |
| views/all/work_items | count | 858 | 919 | 876 |
| views/cascade/batch_sets | count | 3 | 3 | 3 |
| views/cascade/bins | count | 6 | 6 | 6 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 497 | 535 | 503 |
| views/cascade/pipelines | count | 3 | 3 | 3 |
| views/cascade/work_items | count | 497 | 535 | 503 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 2 | 2 | 2 |
| views/main/bins | count | 4 | 4 | 4 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 360 | 392 | 372 |
| views/main/pipelines | count | 2 | 2 | 2 |
| views/main/work_items | count | 361 | 393 | 373 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 519 | 531 | 522 |
| world/point_lights | count | 0 | 0 | 0 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0 | 0 | 0 |
| assets/Image/added | count | 0.01 | 18 | 104 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.15 | 169 | 1408 |
| assets/Mesh/removed | count | 0.46 | 1012 | 4314 |
| assets/StandardMaterial/added | count | 0.01 | 18 | 107 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.01 | 136 | -91 |
| physics/inserts | count | 0.24 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.25 | 145 | 2335 |
| physics/steps | count | 0.42 | 1 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.02 | 21 | 219 |
| stream/deactivate/items | count | 0.02 | 21 | 223 |
| stream/despawn/items | count | 0.02 | 20 | 230 |
| stream/fetch/items | count | 0.01 | 2 | 87 |
| stream/spawn/items | count | 0.03 | 19 | 274 |
| stream/upload/bytes | bytes | 2200 | 1.72 M | 20.74 M |
| stream/upload/items | count | 0.01 | 2 | 61 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 15 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 787 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 3, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 576.7 | main/busy/ms 556, surface/advance_cpu_ms 65.14, surface/cell_upload_ms 0.89, stream/drive/ms 0.31, weather/map/ms 0.13, bigspace/total/ms 0.10, stream/tick/ms 0.08, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 1.05 | 470.4 | main/busy/ms 594, render/thread/ms 436, render/busy/ms 359, render/acquire/ms 74.23, surface/advance_cpu_ms 65.14, render/graph/ms 6.16, render/present/ms 1.97, stream/drive/ms 1.73, stream/tick/ms 0.93, surface/cell_upload_ms 0.89, bigspace/total/ms 0.44, terrain/update/ms 0.37 |
| warm-up | 4.92 | 29.3 | render/thread/ms 35.22, render/busy/ms 32.46, main/busy/ms 31.43, render/graph/ms 28.37, render/acquire/ms 1.80, stream/drive/ms 0.98, render/present/ms 0.96, stream/tick/ms 0.65, terrain/update/ms 0.56, terrain/select/ms 0.50, bigspace/total/ms 0.37, bigspace/high_precision/ms 0.21 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\floor_near_far_farfield\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=near,far,farfield
- before the run: GPU 1%, CPU 2%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **98%** (min 97%) · 2566 MHz (min 2565) · memory 8001 MHz · 100.0 W · 81 °C (max 83) · VRAM 3212 MiB · throttled 100% of samples
