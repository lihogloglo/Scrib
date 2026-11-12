# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1633 frames over 15.0 s · **median 9.16 ms (109 fps)** · mean 9.19 · p95 10.28 · p99 11.21 (1% low 89 fps) · worst 29.31 ms · 6 spikes over 1.5× median · 28.0 stutters/min (work 28.0/min)
- warm-up: 293 frames over 3.8 s, worst 517.1 ms
- hitches (> 25.0 ms): **1** measured, 5 in warm-up
- timed GPU passes: 8.37 ms of a 9.19 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.16 ms, limit 11 ms
- pass worst frame 29.31 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.081 | 2.404 | 0.191 |
| main_opaque_pass_3d | 2.062 | 2.613 | 0.044 |
| atmosphere_luts | 0.962 | 1.294 | 0.003 |
| early prepass | 0.586 | 0.759 | 0.077 |
| lightprobe_irradiance_map | 0.432 | 0.561 | 0.001 |
| bloom | 0.326 | 0.339 | 0.067 |
| ssao | 0.272 | 0.600 | 0.007 |
| lightprobe_radiance_map | 0.230 | 0.233 | 0.004 |
| main_transparent_pass_3d | 0.167 | 0.182 | 0.108 |
| uniform_allocation | 0.149 | 0.155 | 0.024 |
| water_fft | 0.140 | 0.133 | 0.003 |
| upscaling | 0.115 | 0.129 | 0.004 |
| bin_unpacking | 0.099 | 0.105 | 0.018 |
| tonemapping | 0.089 | 0.095 | 0.004 |
| water_velocity | 0.077 | 0.077 | 0.002 |
| atmos_froxel_inject | 0.077 | 0.079 | 0.002 |
| clouds_trace | 0.069 | 0.088 | 0.001 |
| clustering | 0.065 | 0.078 | 0.022 |
| clouds_composite | 0.064 | 0.066 | 0.002 |
| render_sky | 0.054 | 0.055 | 0.002 |
| early_mesh_preprocessing | 0.039 | 0.044 | 0.005 |
| clouds_resolve | 0.035 | 0.041 | 0.001 |
| water_mips | 0.027 | 0.025 | 0.002 |
| atmos_froxel_integrate | 0.024 | 0.031 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| atmosphere_environment | 0.019 | 0.015 | 0.003 |
| per_view_shadow_pass | 0.019 | 0.023 | 0.005 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| lightprobe_copy | 0.014 | 0.014 | 0.004 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| night_sky | 0.008 | 0.012 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| clouds_shadow | 0.004 | 0.005 | 0.004 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | 0.001 | 0.002 | 0.005 |
| surface_trail | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 57.683 | 0.000 | 6755.68 | 94196.1 |
| render/thread/ms | 8.170 | 9.255 | 28.23 | 13341.3 |
| render/busy/ms | 6.507 | 7.652 | 27.57 | 10625.7 |
| main/busy/ms | 6.500 | 7.306 | 14.86 | 10613.9 |
| render/graph/ms | 3.596 | 4.607 | 24.29 | 5872.5 |
| render/acquire/ms | 1.207 | 2.358 | 3.95 | 1970.9 |
| pipelines/compile_ms | 1.137 | 0.000 | 135.99 | 1857.5 |
| pipelines/front_ms | 1.098 | 0.000 | 130.84 | 1792.5 |
| render/present/ms | 0.456 | 0.577 | 4.52 | 744.8 |
| bigspace/total/ms | 0.397 | 0.899 | 4.57 | 648.1 |
| terrain/update/ms | 0.234 | 0.301 | 0.81 | 382.7 |
| terrain/select/ms | 0.203 | 0.260 | 0.77 | 332.1 |
| stream/drive/ms | 0.183 | 0.264 | 0.51 | 298.6 |
| controller/move/ms | 0.168 | 0.284 | 0.54 | 274.2 |
| far/select/ms | 0.163 | 0.240 | 0.54 | 265.8 |
| stream/tick/ms | 0.157 | 0.230 | 0.47 | 255.9 |
| bigspace/high_precision/ms | 0.146 | 0.381 | 1.89 | 238.3 |
| controller/feed/ms | 0.129 | 0.185 | 0.47 | 211.4 |
| bigspace/low_precision/ms | 0.090 | 0.201 | 4.37 | 147.3 |
| anim/evaluate/ms | 0.077 | 0.204 | 1.27 | 125.5 |
| anim/evaluate_cpu/ms | 0.060 | 0.081 | 0.31 | 98.8 |
| pipelines/driver_ms | 0.040 | 0.000 | 5.14 | 65.1 |
| physics/step/ms | 0.030 | 0.058 | 1.25 | 49.1 |
| physics/insert/ms | 0.018 | 0.115 | 0.90 | 29.5 |
| weather/map/ms | 0.016 | 0.022 | 0.09 | 26.9 |
| weather/eval/ms | 0.010 | 0.013 | 0.19 | 17.1 |
| controller/avoid/ms | 0.006 | 0.008 | 0.22 | 10.5 |
| script/update/ms | 0.006 | 0.009 | 0.04 | 10.2 |
| script/input/ms | 0.005 | 0.006 | 0.14 | 7.8 |
| anim/apply/ms | 0.004 | 0.006 | 0.08 | 7.2 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 6.4 |
| controller/write/ms | 0.002 | 0.002 | 0.16 | 3.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 10 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 32 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 8.49 | 27.57 | 8.18 |
| gpu/timed/ms | ms | 8.49 | 9.71 | 8.18 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.72 M | 433.72 M | 433.72 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d/bins | count | 337 | 337 | 337 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3196 | 3196 | 3196 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 337 | 337 | 337 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3196 | 3196 | 3196 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 14 | 16 | 14 |
| phases/opaque3d/bins | count | 3899 | 3900 | 3899 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 9299 | 9299 | 9299 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 13 | 15 | 13 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 16 | 14 |
| phases/opaque3d_prepass/bins | count | 3899 | 3900 | 3899 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 9299 | 9299 | 9299 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 13 | 15 | 13 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 19 | 19 |
| phases/shadow/bins | count | 6714 | 6714 | 6714 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 20.0 k | 20.0 k | 20.0 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 19 | 19 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1789 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 29 | 0 |
| preprocess/bind_groups | count | 0 | 150 | 0 |
| preprocess/bind_groups_created | count | 0 | 150 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 145 | 145 | 145 |
| process/cpu_usage | % | 26.50 | 40.62 | 32.21 |
| process/mem_usage | GiB | 2.11 | 2.26 | 2.26 |
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
| started_load_count |  | 2595 | 2597 | 2597 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 388 | 388 | 388 |
| statics/materials_bound | count | 388 | 388 | 388 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.06 | 0.06 | 0.06 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 34.25 | 49.44 | 36.86 |
| system/mem_usage | % | 50.13 | 50.60 | 50.60 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 63 | 67 | 63 |
| views/all/bins | count | 15.2 k | 15.2 k | 15.2 k |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 45.0 k | 45.0 k | 45.0 k |
| views/all/pipelines | count | 59 | 63 | 59 |
| views/all/pipelines_distinct | count | 55 | 59 | 55 |
| views/all/work_items | count | 45.4 k | 45.4 k | 45.4 k |
| views/cascade/batch_sets | count | 19 | 19 | 19 |
| views/cascade/bins | count | 6714 | 6714 | 6714 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 20.0 k | 20.0 k | 20.0 k |
| views/cascade/pipelines | count | 19 | 19 | 19 |
| views/cascade/work_items | count | 20.0 k | 20.0 k | 20.0 k |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 8472 | 8474 | 8472 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 25.0 k | 25.0 k | 25.0 k |
| views/main/pipelines | count | 40 | 44 | 40 |
| views/main/work_items | count | 25.4 k | 25.4 k | 25.4 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 222 | 222 | 222 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0.02 | 33 | 33 |
| anim/events/count | count | 0.08 | 3 | 124 |
| assets/Image/added | count | 0.05 | 31 | 85 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.19 | 107 | 304 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.06 | 34 | 91 |
| assets/StandardMaterial/modified | count | 3.03 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.82 | 1895 | 4606 |
| physics/inserts | count | 0.86 | 154 | 1408 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.55 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.01 | 6 | 11 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0 | 0 | 0 |
| stream/deactivate/items | count | 0 | 0 | 0 |
| stream/despawn/items | count | 0 | 0 | 0 |
| stream/fetch/items | count | 0 | 0 | 0 |
| stream/spawn/items | count | 0 | 0 | 0 |
| stream/upload/bytes | bytes | 0 | 0 | 0 |
| stream/upload/items | count | 0 | 0 | 0 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.52 | 517.1 | main/busy/ms 494, surface/advance_cpu_ms 0.73, stream/drive/ms 0.30, surface/cell_upload_ms 0.28, bigspace/total/ms 0.08, stream/tick/ms 0.07, bigspace/high_precision/ms 0.05, weather/eval/ms 0.05, weather/map/ms 0.04, script/update/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.87 | 356.7 | main/busy/ms 526, render/thread/ms 321, render/busy/ms 246, render/acquire/ms 72.78, render/graph/ms 5.47, render/present/ms 2.15, stream/drive/ms 0.82, surface/advance_cpu_ms 0.73, stream/tick/ms 0.40, terrain/update/ms 0.40, terrain/select/ms 0.28, surface/cell_upload_ms 0.28 |
| warm-up | 0.95 | 71.5 | pipelines/wait_ms 959, render/thread/ms 331, render/busy/ms 255, pipelines/compile_ms 87.37, pipelines/front_ms 81.90, render/acquire/ms 73.03, main/busy/ms 47.34, terrain/update/ms 17.38, render/graph/ms 8.29, pipelines/driver_ms 5.47, render/present/ms 3.22, stream/drive/ms 1.31 |
| warm-up | 1.17 | 76.9 | pipelines/wait_ms 467, pipelines/compile_ms 76.72, pipelines/front_ms 70.19, main/busy/ms 57.61, render/thread/ms 46.72, render/busy/ms 45.25, pipelines/driver_ms 6.52, render/graph/ms 4.14, render/present/ms 1.24, stream/drive/ms 1.18, stream/tick/ms 1.00, terrain/update/ms 0.54 |
| warm-up | 3.72 | 32.8 | pipelines/wait_ms 5064, main/busy/ms 41.46, pipelines/compile_ms 30.13, pipelines/front_ms 28.65, render/thread/ms 22.18, render/busy/ms 20.94, render/graph/ms 5.80, terrain/update/ms 4.21, pipelines/driver_ms 1.48, render/present/ms 1.06, physics/insert/ms 0.98, stream/drive/ms 0.94 |
| measure | 5.97 | 29.3 | render/thread/ms 36.82, render/busy/ms 35.50, render/graph/ms 29.53, main/busy/ms 13.68, render/present/ms 1.01, bigspace/total/ms 0.59, controller/move/ms 0.54, terrain/update/ms 0.53, terrain/select/ms 0.46, far/select/ms 0.40, stream/drive/ms 0.35, render/acquire/ms 0.31 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\push_all\balmora_night@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **97%** (min 70%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.3 W · 74 °C (max 75) · VRAM 3924 MiB · throttled 100% of samples
