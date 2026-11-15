# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 3981 frames over 65.7 s · **median 16.00 ms (63 fps)** · mean 16.49 · p95 20.04 · p99 23.51 (1% low 43 fps) · worst 40.82 ms · 34 spikes over 1.5× median · 21.0 stutters/min (work 15.5/min)
- warm-up: 427 frames over 10.4 s, worst 2620.9 ms
- hitches (> 40.0 ms): **1** measured, 15 in warm-up
- timed GPU passes: 8.13 ms of a 16.49 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 16.00 ms, limit 11 ms
- **FAIL** frame_ms: 23 stutters (> 1.5x moving median of 60) in 1.09 min = 21.02/min, limit 1/min
- **FAIL** worst frame 40.82 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.186 | 2.188 | 2.566 | 1.00 | 0.192 |
| main_opaque_pass_3d | 1.722 | 1.723 | 2.375 | 1.00 | 0.081 |
| early prepass | 0.540 | 0.540 | 0.801 | 1.00 | 0.136 |
| uniform_allocation | 0.390 | 0.390 | 0.408 | 1.00 | 0.061 |
| bloom | 0.337 | 0.337 | 0.487 | 1.00 | 0.072 |
| atmosphere_luts | 0.332 | 0.332 | 0.616 | 1.00 | 0.004 |
| bin_unpacking | 0.302 | 0.302 | 0.583 | 1.00 | 0.042 |
| water_velocity | 0.301 | 0.301 | 0.561 | 1.00 | 0.002 |
| water_sspr | 0.222 | 0.222 | 0.224 | 1.00 | 0.003 |
| ssao | 0.204 | 0.204 | 0.210 | 1.00 | 0.008 |
| auxiliary_views | 0.164 | 0.164 | 0.632 | 1.00 | 0.158 |
| atmos_froxel_inject | 0.157 | 0.157 | 0.153 | 1.00 | 0.002 |
| clouds_trace | 0.141 | 0.141 | 0.232 | 1.00 | 0.001 |
| water_fft | 0.137 | 0.137 | 0.138 | 1.00 | 0.003 |
| upscaling | 0.124 | 0.124 | 0.153 | 1.00 | 0.005 |
| clouds_composite | 0.111 | 0.111 | 0.111 | 1.00 | 0.002 |
| tonemapping | 0.094 | 0.094 | 0.094 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.092 | 0.092 | 0.171 | 1.00 | 0.009 |
| clustering | 0.069 | 0.069 | 0.070 | 1.00 | 0.022 |
| surface_trail | 0.059 | 0.062 | 0.057 | 0.95 | 0.002 |
| early_mesh_preprocessing | 0.056 | 0.056 | 0.061 | 1.00 | 0.006 |
| render_sky | 0.055 | 0.056 | 0.055 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.054 | 0.430 | 0.428 | 0.13 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| water_snapshot | 0.038 | 0.038 | 0.047 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.226 | 0.234 | 0.13 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.032 | 1.00 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.024 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.017 | 0.017 | 0.031 | 1.00 | 0.155 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 1.00 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 1.00 | 0.002 |
| lightprobe_downsampling_first_pass | 0.002 | 0.015 | 0.016 | 0.13 | 0.001 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.13 | 0.004 |
| lightprobe_downsampling_second_pass | 0.002 | 0.012 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 25.925 | 0.000 | 18035.03 | 103208.2 |
| render/thread/ms | 15.169 | 18.518 | 34.83 | 60388.9 |
| render/busy/ms | 14.577 | 17.873 | 32.59 | 58029.2 |
| main/busy/ms | 9.840 | 13.350 | 29.38 | 39173.5 |
| render/graph/ms | 9.204 | 11.217 | 25.98 | 36641.3 |
| render/present/ms | 0.496 | 0.591 | 18.17 | 1975.2 |
| pipelines/compile_ms | 0.463 | 0.000 | 141.93 | 1843.1 |
| bigspace/total/ms | 0.448 | 0.738 | 4.42 | 1784.0 |
| pipelines/front_ms | 0.448 | 0.000 | 137.46 | 1783.8 |
| controller/feed/ms | 0.295 | 0.402 | 0.80 | 1175.8 |
| terrain/update/ms | 0.293 | 0.370 | 1.47 | 1167.2 |
| terrain/select/ms | 0.259 | 0.327 | 0.70 | 1032.9 |
| stream/drive/ms | 0.259 | 0.396 | 16.62 | 1030.2 |
| far/select/ms | 0.202 | 0.281 | 0.62 | 804.8 |
| stream/tick/ms | 0.190 | 0.268 | 0.67 | 758.2 |
| anim/evaluate/ms | 0.188 | 0.337 | 1.65 | 747.7 |
| bigspace/low_precision/ms | 0.164 | 0.296 | 3.41 | 652.7 |
| anim/evaluate_cpu/ms | 0.145 | 0.191 | 0.52 | 577.0 |
| controller/move/ms | 0.121 | 0.263 | 0.53 | 481.0 |
| bigspace/high_precision/ms | 0.118 | 0.253 | 4.09 | 469.8 |
| render/acquire/ms | 0.097 | 0.128 | 0.43 | 384.6 |
| physics/step/ms | 0.068 | 0.100 | 0.51 | 270.2 |
| physics/insert/ms | 0.064 | 0.457 | 0.96 | 254.6 |
| weather/map/ms | 0.025 | 0.034 | 0.16 | 99.7 |
| pipelines/driver_ms | 0.015 | 0.000 | 4.95 | 59.3 |
| anim/apply/ms | 0.011 | 0.021 | 0.06 | 44.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.08 | 43.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.20 | 42.6 |
| stream/despawn/ms | 0.008 | 0.000 | 1.06 | 30.9 |
| script/update/ms | 0.007 | 0.010 | 0.16 | 28.6 |
| stream/spawn/ms | 0.007 | 0.000 | 1.20 | 26.8 |
| script/ui/ms | 0.006 | 0.007 | 0.12 | 22.2 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 19.2 |
| stream/deactivate/ms | 0.004 | 0.000 | 0.69 | 15.7 |
| controller/write/ms | 0.004 | 0.005 | 0.10 | 14.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 4.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.11 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.05 | 1.9 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.06 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 561 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1306 | 2262 | 601 |
| crowd/c0/meshes/count | count | 355 | 646 | 159 |
| crowd/c1/count | count | 145 | 158 | 148 |
| crowd/c1/joints/count | count | 5507 | 6092 | 5595 |
| crowd/c1/meshes/count | count | 1166 | 1338 | 1182 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 10.4 k | 6179 |
| crowd/joints_uploaded/count | count | 4230 | 6643 | 6179 |
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
| frame/work/ms | ms | 14.14 | 32.59 | 13.96 |
| gpu/timed/ms | ms | 8.78 | 11.99 | 9.02 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 10 | 12 | 9 |
| phases/alpha_mask3d/bins | count | 241 | 478 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3836 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 10 | 12 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 241 | 478 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3836 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 38 | 43 | 40 |
| phases/opaque3d/bins | count | 4081 | 4327 | 4326 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 38 | 43 | 40 |
| phases/opaque3d_prepass/bins | count | 4081 | 4327 | 4326 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 84 | 320 | 80 |
| phases/shadow/bins | count | 12.3 k | 15.8 k | 12.0 k |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 38.9 k | 42.6 k | 39.4 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 159 | 320 | 54 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 248 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 226 | 232 | 232 |
| pipelines/waiting | count | 0 | 17 | 0 |
| preprocess/bind_groups | count | 21 | 905 | 6 |
| preprocess/bind_groups_created | count | 21 | 905 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 363 | 490 | 373 |
| process/cpu_usage | % | 21.63 | 41.96 | 18.75 |
| process/mem_usage | GiB | 5.17 | 5.35 | 5.35 |
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
| started_load_count |  | 4765 | 4859 | 4859 |
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
| stream/latency/s | s | 0.05 | 0.12 | 0.05 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.38 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.30 | 63.04 | 32.12 |
| system/mem_usage | % | 61.33 | 65.27 | 65.27 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 182 | 420 | 178 |
| views/all/bins | count | 21.3 k | 24.2 k | 21.6 k |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 67.5 k | 70.5 k | 68.9 k |
| views/all/pipelines | count | 225 | 390 | 120 |
| views/all/pipelines_distinct | count | 99 | 110 | 91 |
| views/all/work_items | count | 66.7 k | 70.0 k | 69.6 k |
| views/cascade/batch_sets | count | 83 | 99 | 80 |
| views/cascade/bins | count | 11.8 k | 12.4 k | 12.0 k |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 38.2 k | 39.6 k | 39.4 k |
| views/cascade/pipelines | count | 62 | 79 | 54 |
| views/cascade/work_items | count | 38.2 k | 39.6 k | 39.4 k |
| views/lamp_face/batch_sets | count | 0 | 232 | 0 |
| views/lamp_face/bins | count | 1665 | 3748 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1800 | 4120 | 0 |
| views/lamp_face/pipelines | count | 92 | 252 | 0 |
| views/lamp_face/work_items | count | 0 | 3729 | 0 |
| views/main/batch_sets | count | 98 | 108 | 98 |
| views/main/bins | count | 8644 | 9594 | 9594 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.0 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 66 | 72 | 66 |
| views/main/work_items | count | 28.5 k | 30.4 k | 30.3 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 56.54 | 287 | 225.1 k |
| anim/events/count | count | 0.13 | 4 | 530 |
| assets/Image/added | count | 0.15 | 43 | 614 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 1.04 | 152 | 4133 |
| assets/Mesh/removed | count | 1.23 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.12 | 35 | 488 |
| assets/StandardMaterial/modified | count | 5.44 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.10 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.26 | 1492 | 5025 |
| physics/inserts | count | 0.63 | 111 | 2490 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.59 | 145 | 2335 |
| physics/steps | count | 0.99 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 16 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.06 | 21 | 219 |
| stream/deactivate/items | count | 0.06 | 21 | 223 |
| stream/despawn/items | count | 0.06 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.09 | 20 | 378 |
| stream/upload/bytes | bytes | 8503 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 54 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1864 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 40.0 ms; slowest 16, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.62 | 2620.9 | main/busy/ms 2592, surface/advance_cpu_ms 67.56, surface/cell_upload_ms 1.06, stream/drive/ms 0.32, bigspace/total/ms 0.09, stream/tick/ms 0.09, weather/map/ms 0.08, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, far/select/ms 0.02, script/update/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 3.36 | 743.0 | main/busy/ms 2631, render/thread/ms 704, render/busy/ms 557, render/acquire/ms 144, surface/advance_cpu_ms 67.56, render/graph/ms 5.21, render/present/ms 2.34, surface/cell_upload_ms 1.06, stream/drive/ms 1.05, stream/tick/ms 0.54, terrain/update/ms 0.52, terrain/select/ms 0.37 |
| warm-up | 3.44 | 78.2 | render/thread/ms 717, render/busy/ms 569, pipelines/wait_ms 500, render/acquire/ms 144, pipelines/compile_ms 85.40, pipelines/front_ms 76.70, main/busy/ms 56.70, terrain/update/ms 22.87, render/graph/ms 9.32, pipelines/driver_ms 8.70, render/present/ms 3.40, stream/drive/ms 1.34 |
| warm-up | 3.68 | 94.3 | main/busy/ms 67.63, render/thread/ms 57.37, render/busy/ms 54.89, render/graph/ms 5.09, render/present/ms 2.25, stream/drive/ms 1.87, stream/tick/ms 1.21, terrain/update/ms 0.93, terrain/select/ms 0.77, stream/spawn/ms 0.29, bigspace/total/ms 0.26, render/acquire/ms 0.23 |
| warm-up | 3.87 | 73.5 | render/thread/ms 56.97, render/busy/ms 55.60, main/busy/ms 46.08, stream/drive/ms 33.03, render/graph/ms 2.72, render/present/ms 1.10, stream/tick/ms 0.98, terrain/update/ms 0.86, terrain/select/ms 0.72, stream/spawn/ms 0.29, render/acquire/ms 0.27, bigspace/total/ms 0.22 |
| warm-up | 4.04 | 55.4 | pipelines/wait_ms 5438, pipelines/compile_ms 100, pipelines/front_ms 94.58, main/busy/ms 44.26, render/thread/ms 40.99, render/busy/ms 38.76, stream/drive/ms 24.61, pipelines/driver_ms 5.76, render/graph/ms 3.71, terrain/update/ms 3.40, render/present/ms 2.02, stream/spawn/ms 1.04 |
| warm-up | 4.18 | 52.9 | render/thread/ms 59.02, render/busy/ms 56.73, main/busy/ms 38.14, render/graph/ms 17.35, stream/drive/ms 15.81, render/present/ms 2.00, stream/tick/ms 1.25, stream/spawn/ms 1.06, terrain/update/ms 0.70, terrain/select/ms 0.56, bigspace/total/ms 0.30, render/acquire/ms 0.29 |
| warm-up | 4.31 | 67.5 | pipelines/wait_ms 2269, main/busy/ms 69.00, stream/drive/ms 48.51, render/thread/ms 35.20, pipelines/compile_ms 34.76, render/busy/ms 33.67, pipelines/front_ms 33.42, render/graph/ms 4.20, stream/spawn/ms 2.35, stream/tick/ms 1.40, pipelines/driver_ms 1.35, render/present/ms 1.29 |
| warm-up | 4.41 | 77.7 | pipelines/wait_ms 9216, pipelines/compile_ms 198, pipelines/front_ms 183, main/busy/ms 65.16, render/thread/ms 47.30, render/busy/ms 45.94, stream/drive/ms 44.39, pipelines/driver_ms 15.70, render/graph/ms 6.27, stream/spawn/ms 4.05, stream/tick/ms 1.65, stream/activate/ms 1.07 |
| warm-up | 4.48 | 66.3 | pipelines/wait_ms 10.2 k, pipelines/compile_ms 203, pipelines/front_ms 187, render/thread/ms 82.64, main/busy/ms 78.62, stream/drive/ms 52.06, render/busy/ms 51.52, render/present/ms 30.92, pipelines/driver_ms 16.06, render/graph/ms 6.35, stream/spawn/ms 3.03, stream/tick/ms 1.62 |
| warm-up | 4.55 | 69.8 | pipelines/wait_ms 1659, pipelines/compile_ms 133, pipelines/front_ms 125, render/thread/ms 96.13, main/busy/ms 78.92, render/busy/ms 64.98, stream/drive/ms 50.23, render/present/ms 30.97, pipelines/driver_ms 7.90, render/graph/ms 5.63, terrain/update/ms 5.29, stream/spawn/ms 2.11 |
| warm-up | 4.73 | 40.6 | render/thread/ms 53.31, render/busy/ms 50.94, main/busy/ms 36.51, render/graph/ms 8.73, stream/drive/ms 6.79, stream/spawn/ms 3.35, render/present/ms 2.02, stream/tick/ms 1.92, terrain/update/ms 0.83, terrain/select/ms 0.68, bigspace/total/ms 0.39, render/acquire/ms 0.35 |
| warm-up | 4.79 | 54.3 | render/thread/ms 72.99, render/busy/ms 70.91, main/busy/ms 56.85, render/graph/ms 7.85, stream/drive/ms 7.12, stream/spawn/ms 3.75, stream/tick/ms 2.11, render/present/ms 1.70, terrain/update/ms 0.90, terrain/select/ms 0.72, bigspace/total/ms 0.53, render/acquire/ms 0.37 |
| warm-up | 4.85 | 58.6 | pipelines/wait_ms 777, pipelines/compile_ms 196, pipelines/front_ms 190, render/thread/ms 78.48, main/busy/ms 76.82, render/busy/ms 76.67, render/graph/ms 6.90, pipelines/driver_ms 6.00, stream/drive/ms 5.75, stream/spawn/ms 3.67, render/present/ms 1.47, stream/tick/ms 1.21 |
| warm-up | 4.89 | 40.8 | pipelines/wait_ms 2157, pipelines/compile_ms 247, pipelines/front_ms 239, render/thread/ms 67.01, render/busy/ms 65.61, main/busy/ms 52.32, pipelines/driver_ms 8.22, render/graph/ms 6.04, stream/drive/ms 3.55, stream/spawn/ms 2.06, render/present/ms 1.19, bigspace/total/ms 0.98 |
| measure | 59.09 | 40.8 | render/thread/ms 51.21, render/busy/ms 49.93, main/busy/ms 23.42, render/graph/ms 19.85, render/present/ms 1.07, bigspace/total/ms 0.87, controller/feed/ms 0.81, terrain/update/ms 0.75, terrain/select/ms 0.67, stream/drive/ms 0.54, far/select/ms 0.46, stream/tick/ms 0.46 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\h1_new\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 7%
- during the run: other processes 11% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **65%** (min 45%) · 2580 MHz (min 2580) · memory 8001 MHz · 64.4 W · 71 °C (max 76) · VRAM 5650 MiB · throttled 100% of samples
