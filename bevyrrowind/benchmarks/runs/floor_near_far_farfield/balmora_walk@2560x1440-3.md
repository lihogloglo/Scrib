# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near,far,farfield · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 9433 frames over 65.6 s · **median 6.93 ms (144 fps)** · mean 6.96 · p95 7.85 · p99 8.24 (1% low 121 fps) · worst 10.46 ms · 2 spikes over 1.5× median · 1.8 stutters/min (work 0.9/min)
- warm-up: 873 frames over 7.2 s, worst 577.4 ms
- hitches (> 25.0 ms): **0** measured, 3 in warm-up
- timed GPU passes: 6.67 ms of a 6.96 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 6.93 ms, limit 11 ms
- **FAIL** frame_ms: 2 stutters (> 1.5x moving median of 60) in 1.09 min = 1.83/min, limit 1/min
- pass worst frame 10.46 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.140 | 2.446 | 0.177 |
| main_opaque_pass_3d | 1.102 | 1.509 | 0.009 |
| atmosphere_luts | 0.846 | 1.133 | 0.003 |
| lightprobe_irradiance_map | 0.426 | 0.423 | 0.001 |
| bloom | 0.326 | 0.334 | 0.056 |
| lightprobe_radiance_map | 0.230 | 0.236 | 0.003 |
| ssao | 0.215 | 0.216 | 0.006 |
| clouds_trace | 0.149 | 0.245 | 0.001 |
| water_fft | 0.133 | 0.130 | 0.002 |
| early prepass | 0.117 | 0.127 | 0.015 |
| atmos_froxel_inject | 0.104 | 0.109 | 0.001 |
| upscaling | 0.096 | 0.117 | 0.004 |
| tonemapping | 0.089 | 0.090 | 0.003 |
| water_velocity | 0.084 | 0.085 | 0.001 |
| main_transparent_pass_3d | 0.080 | 0.253 | 0.009 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| clustering | 0.063 | 0.072 | 0.019 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| clouds_resolve | 0.036 | 0.040 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.033 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| early_mesh_preprocessing | 0.022 | 0.022 | 0.005 |
| per_view_shadow_pass | 0.022 | 0.031 | 0.006 |
| uniform_allocation | 0.015 | 0.015 | 0.005 |
| lightprobe_copy | 0.015 | 0.016 | 0.003 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| bin_unpacking | 0.011 | 0.011 | 0.005 |
| atmosphere_environment | 0.011 | 0.013 | 0.003 |
| clouds_fog_light | 0.011 | 0.011 | 0.002 |
| night_sky | 0.008 | 0.016 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.002 |
| blas_build | 0.002 | 0.002 | 0.005 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.285 | 7.175 | 12.94 | 59290.7 |
| main/busy/ms | 4.185 | 4.902 | 10.89 | 39479.4 |
| render/busy/ms | 4.178 | 4.845 | 9.46 | 39411.1 |
| render/graph/ms | 2.513 | 3.126 | 5.92 | 23708.9 |
| render/acquire/ms | 1.674 | 2.666 | 3.96 | 15793.2 |
| render/present/ms | 0.433 | 0.533 | 1.61 | 4086.3 |
| terrain/update/ms | 0.299 | 0.378 | 1.70 | 2817.2 |
| terrain/select/ms | 0.266 | 0.338 | 0.65 | 2511.2 |
| bigspace/total/ms | 0.239 | 0.459 | 1.28 | 2255.2 |
| stream/drive/ms | 0.222 | 0.294 | 2.04 | 2090.1 |
| stream/tick/ms | 0.186 | 0.238 | 0.62 | 1753.3 |
| bigspace/high_precision/ms | 0.121 | 0.270 | 0.83 | 1144.7 |
| bigspace/low_precision/ms | 0.050 | 0.166 | 1.11 | 472.0 |
| weather/map/ms | 0.024 | 0.031 | 0.26 | 222.3 |
| physics/insert/ms | 0.016 | 0.000 | 1.11 | 152.6 |
| weather/eval/ms | 0.010 | 0.013 | 0.04 | 92.2 |
| physics/step/ms | 0.009 | 0.025 | 0.60 | 89.2 |
| script/update/ms | 0.006 | 0.007 | 0.17 | 53.5 |
| script/input/ms | 0.005 | 0.006 | 0.21 | 47.7 |
| script/ui/ms | 0.003 | 0.005 | 0.10 | 26.6 |
| anim/evaluate/ms | 0.002 | 0.003 | 0.12 | 18.7 |
| controller/move/ms | 0.002 | 0.003 | 0.23 | 16.7 |
| stream/deactivate/ms | 0.001 | 0.000 | 0.60 | 11.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.06 | 8.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.17 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.2 |
| anim/apply/ms | 0.001 | 0.001 | 0.01 | 4.9 |
| stream/spawn/ms | 0.000 | 0.000 | 0.16 | 3.0 |
| controller/feed/ms | 0.000 | 0.000 | 0.20 | 2.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.6 |
| stream/despawn/ms | 0.000 | 0.000 | 0.21 | 2.6 |
| controller/avoid/ms | 0.000 | 0.000 | 0.01 | 0.9 |
| stream/upload/ms | 0.000 | 0.000 | 0.02 | 0.5 |
| controller/write/ms | 0.000 | 0.000 | 0.16 | 0.5 |
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
| frame/work/ms | ms | 6.71 | 10.89 | 6.33 |
| gpu/timed/ms | ms | 6.71 | 7.77 | 6.33 |
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
| phases/opaque3d/items | count | 180 | 195 | 186 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 1 | 1 | 1 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 1 | 1 | 1 |
| phases/opaque3d_prepass/bins | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 180 | 195 | 186 |
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
| process/cpu_usage | % | 20.48 | 40.41 | 25.48 |
| process/mem_usage | GiB | 2.09 | 2.12 | 2.10 |
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
| stream/latency/s | s | 0.02 | 3.21 | 0.02 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 1188 | 1277 | 1116 |
| stream/oldest_request/s | s | 0 | 0.13 | 0 |
| stream/page_cache/bytes | bytes | 29.61 M | 29.61 M | 29.61 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1476 | 1483 | 1472 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 26.53 | 45.81 | 26.88 |
| system/mem_usage | % | 50.82 | 51.00 | 50.77 |
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
| views/main/items | count | 360 | 390 | 372 |
| views/main/pipelines | count | 2 | 2 | 2 |
| views/main/work_items | count | 361 | 391 | 373 |
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
| assets/Mesh/removed | count | 0.46 | 1092 | 4314 |
| assets/StandardMaterial/added | count | 0.01 | 18 | 107 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.01 | 136 | -91 |
| physics/inserts | count | 0.24 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.25 | 161 | 2335 |
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
| stream/upload/bytes | bytes | 2199 | 1.72 M | 20.74 M |
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
| warm-up | 0.58 | 577.4 | main/busy/ms 556, surface/advance_cpu_ms 66.48, surface/cell_upload_ms 0.85, stream/drive/ms 0.52, weather/map/ms 0.10, stream/tick/ms 0.08, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.91 | 336.8 | main/busy/ms 590, render/thread/ms 298, render/busy/ms 220, render/acquire/ms 75.10, surface/advance_cpu_ms 66.48, render/graph/ms 4.60, render/present/ms 2.46, stream/drive/ms 1.26, surface/cell_upload_ms 0.85, terrain/update/ms 0.60, stream/tick/ms 0.53, terrain/select/ms 0.37 |
| warm-up | 1.01 | 91.6 | pipelines/wait_ms 847, render/thread/ms 324, render/busy/ms 245, pipelines/compile_ms 81.98, render/acquire/ms 75.32, pipelines/front_ms 74.10, main/busy/ms 59.49, render/graph/ms 21.11, terrain/update/ms 19.19, pipelines/driver_ms 7.88, render/present/ms 3.16, stream/drive/ms 2.11 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\floor_near_far_farfield\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=near,far,farfield
- before the run: GPU 0%, CPU 4%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (254 samples): GPU load **98%** (min 97%) · 2580 MHz (min 2580) · memory 8001 MHz · 100.1 W · 81 °C (max 83) · VRAM 3214 MiB · throttled 100% of samples
