# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cpuculltag · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1647 frames over 15.0 s · **median 9.03 ms (111 fps)** · mean 9.11 · p95 10.14 · p99 11.08 (1% low 90 fps) · worst 34.21 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 383 frames over 4.5 s, worst 532.7 ms
- hitches (> 25.0 ms): **1** measured, 7 in warm-up
- timed GPU passes: 8.47 ms of a 9.11 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.03 ms, limit 11 ms
- **FAIL** worst frame 34.21 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.135 | 2.442 | 0.045 |
| dlss_super_resolution | 2.076 | 2.307 | 0.191 |
| atmosphere_luts | 1.094 | 1.320 | 0.003 |
| early prepass | 0.606 | 0.876 | 0.081 |
| lightprobe_irradiance_map | 0.414 | 0.417 | 0.001 |
| bloom | 0.325 | 0.338 | 0.069 |
| lightprobe_radiance_map | 0.223 | 0.232 | 0.004 |
| ssao | 0.221 | 0.222 | 0.007 |
| main_transparent_pass_3d | 0.173 | 0.178 | 0.074 |
| water_fft | 0.128 | 0.128 | 0.003 |
| uniform_allocation | 0.127 | 0.132 | 0.026 |
| upscaling | 0.127 | 0.132 | 0.004 |
| bin_unpacking | 0.093 | 0.099 | 0.018 |
| tonemapping | 0.088 | 0.091 | 0.003 |
| atmos_froxel_inject | 0.076 | 0.078 | 0.002 |
| water_velocity | 0.076 | 0.076 | 0.002 |
| clouds_trace | 0.067 | 0.087 | 0.001 |
| clustering | 0.064 | 0.077 | 0.023 |
| clouds_composite | 0.064 | 0.066 | 0.002 |
| render_sky | 0.054 | 0.055 | 0.002 |
| clouds_resolve | 0.035 | 0.037 | 0.001 |
| early_mesh_preprocessing | 0.032 | 0.034 | 0.006 |
| water_mips | 0.024 | 0.024 | 0.002 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.022 | 0.025 | 0.001 |
| per_view_shadow_pass | 0.015 | 0.020 | 0.006 |
| lightprobe_copy | 0.015 | 0.016 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| clouds_fog_light | 0.013 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.012 | 0.011 | 0.001 |
| atmosphere_environment | 0.012 | 0.014 | 0.003 |
| night_sky | 0.007 | 0.012 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.003 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| blas_build | 0.001 | 0.002 | 0.006 |
| surface_trail | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 39.383 | 0.000 | 4206.37 | 64863.1 |
| render/thread/ms | 8.228 | 9.209 | 33.29 | 13550.8 |
| main/busy/ms | 6.671 | 7.784 | 32.32 | 10986.5 |
| render/busy/ms | 6.341 | 7.767 | 32.66 | 10443.6 |
| render/graph/ms | 3.471 | 4.582 | 29.86 | 5715.9 |
| render/acquire/ms | 1.427 | 2.524 | 3.90 | 2350.1 |
| pipelines/compile_ms | 0.716 | 0.000 | 132.95 | 1179.5 |
| pipelines/front_ms | 0.695 | 0.000 | 128.95 | 1144.7 |
| render/present/ms | 0.460 | 0.593 | 2.81 | 757.0 |
| bigspace/total/ms | 0.423 | 0.902 | 25.58 | 696.6 |
| terrain/update/ms | 0.233 | 0.295 | 0.69 | 384.6 |
| terrain/select/ms | 0.203 | 0.256 | 0.66 | 334.1 |
| stream/drive/ms | 0.181 | 0.266 | 0.60 | 297.9 |
| far/select/ms | 0.159 | 0.232 | 0.63 | 262.4 |
| stream/tick/ms | 0.156 | 0.229 | 0.57 | 256.2 |
| controller/feed/ms | 0.154 | 0.215 | 0.44 | 254.1 |
| controller/move/ms | 0.151 | 0.275 | 0.57 | 247.9 |
| bigspace/high_precision/ms | 0.150 | 0.428 | 1.70 | 246.6 |
| bigspace/low_precision/ms | 0.095 | 0.225 | 1.27 | 156.7 |
| anim/evaluate/ms | 0.082 | 0.220 | 1.15 | 135.7 |
| anim/evaluate_cpu/ms | 0.063 | 0.087 | 0.30 | 103.5 |
| physics/step/ms | 0.033 | 0.063 | 0.89 | 53.6 |
| pipelines/driver_ms | 0.021 | 0.000 | 4.75 | 34.8 |
| physics/insert/ms | 0.018 | 0.209 | 0.88 | 30.4 |
| weather/map/ms | 0.017 | 0.021 | 0.07 | 27.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.04 | 17.3 |
| script/update/ms | 0.007 | 0.010 | 0.04 | 10.9 |
| controller/avoid/ms | 0.006 | 0.008 | 0.13 | 9.8 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.6 |
| anim/apply/ms | 0.005 | 0.006 | 0.22 | 7.5 |
| script/ui/ms | 0.004 | 0.007 | 0.22 | 6.9 |
| controller/write/ms | 0.002 | 0.002 | 0.21 | 3.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.7 |
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
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 18 | 7 |
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
| frame/work/ms | ms | 8.41 | 32.66 | 8.50 |
| gpu/timed/ms | ms | 8.41 | 9.73 | 8.50 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.72 M | 433.72 M | 433.72 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d/bins | count | 282 | 282 | 282 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 876 | 876 | 876 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 282 | 282 | 282 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 876 | 876 | 876 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 14 | 15 | 14 |
| phases/opaque3d/bins | count | 1307 | 1308 | 1307 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3780 | 3780 | 3780 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 13 | 14 | 13 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/bins | count | 1307 | 1308 | 1307 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3780 | 3780 | 3780 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 13 | 14 | 13 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 16 | 16 | 16 |
| phases/shadow/bins | count | 988 | 988 | 988 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 2009 | 2009 | 2009 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 16 | 16 | 16 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1717 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 154 | 154 | 154 |
| pipelines/waiting | count | 0 | 19 | 0 |
| preprocess/bind_groups | count | 0 | 98 | 0 |
| preprocess/bind_groups_created | count | 0 | 98 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 139 | 139 | 139 |
| process/cpu_usage | % | 25.23 | 49.52 | 26.48 |
| process/mem_usage | GiB | 2.18 | 2.22 | 2.22 |
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
| stream/latency/s | s | 0.03 | 0.03 | 0.03 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.02 | 57.47 | 30.25 |
| system/mem_usage | % | 50.19 | 50.29 | 50.29 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 60 | 62 | 60 |
| views/all/bins | count | 4166 | 4166 | 4166 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.3 k | 11.3 k | 11.3 k |
| views/all/pipelines | count | 56 | 58 | 56 |
| views/all/pipelines_distinct | count | 55 | 57 | 55 |
| views/all/work_items | count | 11.5 k | 11.5 k | 11.5 k |
| views/cascade/batch_sets | count | 16 | 16 | 16 |
| views/cascade/bins | count | 988 | 988 | 988 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2009 | 2009 | 2009 |
| views/cascade/pipelines | count | 16 | 16 | 16 |
| views/cascade/work_items | count | 2009 | 2009 | 2009 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 44 | 46 | 44 |
| views/main/bins | count | 3178 | 3180 | 3178 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9312 | 9312 | 9312 |
| views/main/pipelines | count | 40 | 42 | 40 |
| views/main/work_items | count | 9490 | 9490 | 9490 |
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
| anim/bones_written/count | count | 0.10 | 165 | 165 |
| anim/events/count | count | 0.08 | 3 | 135 |
| assets/Image/added | count | 0.03 | 25 | 56 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.12 | 65 | 204 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.04 | 26 | 64 |
| assets/StandardMaterial/modified | count | 3.01 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 3.67 | 1082 | 6040 |
| physics/inserts | count | 1.02 | 170 | 1686 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.55 | 2 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
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

## Hitches (> 25.0 ms; slowest 8, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.53 | 532.7 | main/busy/ms 509, surface/advance_cpu_ms 0.78, stream/drive/ms 0.45, surface/cell_upload_ms 0.28, stream/tick/ms 0.16, bigspace/total/ms 0.07, weather/map/ms 0.05, bigspace/high_precision/ms 0.04, script/update/ms 0.04, weather/eval/ms 0.03, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.81 | 275.7 | main/busy/ms 542, render/thread/ms 241, render/busy/ms 162, render/acquire/ms 76.74, render/graph/ms 4.58, render/present/ms 2.15, stream/drive/ms 1.00, surface/advance_cpu_ms 0.78, stream/tick/ms 0.53, terrain/update/ms 0.32, surface/cell_upload_ms 0.28, terrain/select/ms 0.20 |
| warm-up | 0.97 | 156.8 | pipelines/wait_ms 1441, render/thread/ms 251, render/busy/ms 171, pipelines/compile_ms 164, pipelines/front_ms 151, render/acquire/ms 76.90, main/busy/ms 50.37, terrain/update/ms 18.11, pipelines/driver_ms 13.18, render/graph/ms 8.08, render/present/ms 3.06, stream/drive/ms 1.19 |
| warm-up | 1.22 | 94.7 | pipelines/wait_ms 274, main/busy/ms 64.48, render/thread/ms 58.83, render/busy/ms 57.08, pipelines/compile_ms 15.44, pipelines/front_ms 14.20, render/graph/ms 3.64, render/present/ms 1.52, pipelines/driver_ms 1.24, stream/drive/ms 1.22, stream/tick/ms 1.04, terrain/update/ms 0.81 |
| warm-up | 1.26 | 29.7 | render/thread/ms 32.39, render/busy/ms 31.19, main/busy/ms 16.91, render/graph/ms 2.45, stream/drive/ms 1.81, render/present/ms 0.99, terrain/update/ms 0.72, stream/tick/ms 0.65, terrain/select/ms 0.54, stream/spawn/ms 0.52, bigspace/total/ms 0.26, stream/upload/ms 0.22 |
| warm-up | 2.16 | 26.3 | main/busy/ms 32.34, render/thread/ms 16.74, render/busy/ms 15.38, render/graph/ms 4.16, stream/drive/ms 2.87, stream/spawn/ms 1.66, render/present/ms 1.17, stream/tick/ms 0.71, terrain/update/ms 0.57, bigspace/total/ms 0.48, terrain/select/ms 0.46, bigspace/high_precision/ms 0.21 |
| warm-up | 2.22 | 30.8 | pipelines/wait_ms 3504, pipelines/compile_ms 88.39, pipelines/front_ms 82.64, main/busy/ms 39.48, render/thread/ms 21.63, render/busy/ms 20.12, pipelines/driver_ms 5.75, render/graph/ms 4.74, stream/drive/ms 3.96, stream/spawn/ms 2.41, render/present/ms 1.28, stream/tick/ms 0.92 |
| measure | 8.00 | 34.2 | render/thread/ms 41.06, render/busy/ms 39.94, main/busy/ms 38.88, render/graph/ms 34.38, bigspace/total/ms 25.89, render/present/ms 0.97, stream/drive/ms 0.51, terrain/update/ms 0.45, stream/tick/ms 0.44, far/select/ms 0.40, terrain/select/ms 0.39, controller/feed/ms 0.32 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\push_nocull\balmora_night@2560x1440_off=cpuculltag-3 --bench-baseline none --off cpuculltag
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 18%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 89%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.0 W · 72 °C (max 75) · VRAM 3928 MiB · throttled 100% of samples
