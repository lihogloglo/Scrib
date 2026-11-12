# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near,far,farfield · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 9428 frames over 65.6 s · **median 6.93 ms (144 fps)** · mean 6.96 · p95 7.87 · p99 8.32 (1% low 120 fps) · worst 10.99 ms · 2 spikes over 1.5× median · 0.9 stutters/min (work 0.9/min)
- warm-up: 950 frames over 7.6 s, worst 580.6 ms
- hitches (> 25.0 ms): **0** measured, 3 in warm-up
- timed GPU passes: 6.66 ms of a 6.96 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 6.93 ms, limit 11 ms
- pass frame_ms: 1 stutters (> 1.5x moving median of 60) in 1.09 min = 0.91/min, limit 1/min
- pass worst frame 10.99 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.172 | 2.460 | 0.177 |
| main_opaque_pass_3d | 1.087 | 1.579 | 0.009 |
| atmosphere_luts | 0.810 | 1.110 | 0.003 |
| lightprobe_irradiance_map | 0.432 | 0.564 | 0.001 |
| bloom | 0.331 | 0.339 | 0.056 |
| lightprobe_radiance_map | 0.233 | 0.237 | 0.003 |
| ssao | 0.220 | 0.221 | 0.006 |
| clouds_trace | 0.150 | 0.244 | 0.001 |
| water_fft | 0.130 | 0.128 | 0.002 |
| early prepass | 0.124 | 0.132 | 0.015 |
| atmos_froxel_inject | 0.101 | 0.106 | 0.001 |
| tonemapping | 0.087 | 0.090 | 0.003 |
| water_velocity | 0.086 | 0.086 | 0.001 |
| main_transparent_pass_3d | 0.081 | 0.254 | 0.009 |
| upscaling | 0.079 | 0.115 | 0.004 |
| clustering | 0.066 | 0.073 | 0.019 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| surface_trail | 0.060 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| clouds_resolve | 0.036 | 0.039 | 0.001 |
| clouds_shadow | 0.031 | 0.034 | 0.003 |
| atmos_froxel_integrate | 0.031 | 0.034 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.034 | 0.006 |
| early_mesh_preprocessing | 0.022 | 0.022 | 0.005 |
| uniform_allocation | 0.015 | 0.015 | 0.005 |
| lightprobe_copy | 0.015 | 0.017 | 0.003 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| bin_unpacking | 0.011 | 0.011 | 0.005 |
| atmosphere_environment | 0.011 | 0.014 | 0.003 |
| clouds_fog_light | 0.011 | 0.011 | 0.002 |
| night_sky | 0.008 | 0.016 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.002 |
| blas_build | 0.002 | 0.002 | 0.005 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.295 | 7.194 | 10.31 | 59346.7 |
| render/busy/ms | 4.222 | 5.042 | 8.71 | 39802.4 |
| main/busy/ms | 4.171 | 4.880 | 10.76 | 39328.8 |
| render/graph/ms | 2.568 | 3.324 | 6.37 | 24214.1 |
| render/acquire/ms | 1.641 | 2.691 | 4.22 | 15472.4 |
| render/present/ms | 0.432 | 0.531 | 1.81 | 4072.0 |
| terrain/update/ms | 0.298 | 0.379 | 1.93 | 2814.2 |
| terrain/select/ms | 0.266 | 0.338 | 0.75 | 2507.8 |
| bigspace/total/ms | 0.238 | 0.456 | 1.27 | 2245.7 |
| stream/drive/ms | 0.226 | 0.310 | 2.13 | 2129.5 |
| stream/tick/ms | 0.189 | 0.251 | 0.57 | 1782.0 |
| bigspace/high_precision/ms | 0.120 | 0.260 | 1.04 | 1134.9 |
| bigspace/low_precision/ms | 0.050 | 0.165 | 1.11 | 467.5 |
| weather/map/ms | 0.024 | 0.032 | 0.21 | 225.5 |
| physics/insert/ms | 0.016 | 0.000 | 1.02 | 150.7 |
| weather/eval/ms | 0.010 | 0.014 | 0.17 | 91.1 |
| physics/step/ms | 0.010 | 0.025 | 0.66 | 89.7 |
| script/update/ms | 0.006 | 0.007 | 0.20 | 54.1 |
| script/input/ms | 0.005 | 0.006 | 0.17 | 45.4 |
| script/ui/ms | 0.003 | 0.005 | 0.06 | 27.9 |
| anim/evaluate/ms | 0.002 | 0.002 | 0.19 | 18.6 |
| controller/move/ms | 0.002 | 0.003 | 0.04 | 17.0 |
| stream/deactivate/ms | 0.001 | 0.000 | 0.62 | 12.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.07 | 8.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.16 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.4 |
| anim/apply/ms | 0.001 | 0.001 | 0.10 | 5.1 |
| stream/spawn/ms | 0.000 | 0.000 | 0.17 | 2.9 |
| controller/feed/ms | 0.000 | 0.001 | 0.10 | 2.8 |
| weather/apply/ms | 0.000 | 0.000 | 0.02 | 2.6 |
| stream/despawn/ms | 0.000 | 0.000 | 0.16 | 2.6 |
| controller/avoid/ms | 0.000 | 0.000 | 0.03 | 1.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.02 | 0.5 |
| controller/write/ms | 0.000 | 0.000 | 0.00 | 0.4 |
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
| ecs/entities | count | 10.9 k | 11.2 k | 10.8 k |
| frame/work/ms | ms | 6.68 | 10.76 | 6.35 |
| gpu/timed/ms | ms | 6.68 | 7.85 | 6.35 |
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
| process/cpu_usage | % | 22.12 | 37.98 | 16.83 |
| process/mem_usage | GiB | 2.11 | 2.14 | 2.12 |
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
| stream/latency/s | s | 0.02 | 3.80 | 0.02 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 1188 | 1277 | 1116 |
| stream/oldest_request/s | s | 0 | 0.12 | 0 |
| stream/page_cache/bytes | bytes | 29.61 M | 29.61 M | 29.61 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1476 | 1483 | 1472 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 26.77 | 49.45 | 26.01 |
| system/mem_usage | % | 51.67 | 51.81 | 51.56 |
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
| assets/Mesh/added | count | 0.15 | 155 | 1408 |
| assets/Mesh/removed | count | 0.46 | 1012 | 4314 |
| assets/StandardMaterial/added | count | 0.01 | 18 | 107 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.01 | 136 | -91 |
| physics/inserts | count | 0.24 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.25 | 161 | 2335 |
| physics/steps | count | 0.42 | 1 | 3939 |
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
| warm-up | 0.58 | 580.6 | main/busy/ms 556, surface/advance_cpu_ms 65.17, surface/cell_upload_ms 0.71, stream/drive/ms 0.49, weather/map/ms 0.14, stream/tick/ms 0.09, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.90 | 316.6 | main/busy/ms 589, render/thread/ms 284, render/busy/ms 212, render/acquire/ms 69.42, surface/advance_cpu_ms 65.17, render/graph/ms 4.68, render/present/ms 2.53, stream/drive/ms 1.25, surface/cell_upload_ms 0.71, stream/tick/ms 0.55, terrain/update/ms 0.52, terrain/select/ms 0.39 |
| warm-up | 0.98 | 79.3 | pipelines/wait_ms 1235, render/thread/ms 294, render/busy/ms 221, pipelines/compile_ms 82.31, pipelines/front_ms 74.31, render/acquire/ms 69.68, main/busy/ms 44.87, terrain/update/ms 18.61, pipelines/driver_ms 7.99, render/graph/ms 7.49, render/present/ms 3.38, stream/drive/ms 1.59 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\floor_near_far_farfield\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=near,far,farfield
- before the run: GPU 0%, CPU 1%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **98%** (min 97%) · 2577 MHz (min 2565) · memory 8001 MHz · 99.9 W · 81 °C (max 83) · VRAM 3213 MiB · throttled 100% of samples
