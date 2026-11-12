# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cpuculltag · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1493 frames over 15.0 s · **median 9.98 ms (100 fps)** · mean 10.05 · p95 11.08 · p99 11.62 (1% low 86 fps) · worst 20.81 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 453 frames over 5.2 s, worst 582.3 ms
- hitches (> 25.0 ms): **0** measured, 3 in warm-up
- timed GPU passes: 9.11 ms of a 10.05 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.98 ms, limit 11 ms
- pass worst frame 20.81 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.493 | 2.997 | 0.046 |
| dlss_super_resolution | 2.095 | 2.429 | 0.189 |
| atmosphere_luts | 1.164 | 1.427 | 0.003 |
| early prepass | 0.629 | 0.922 | 0.087 |
| lightprobe_irradiance_map | 0.426 | 0.559 | 0.001 |
| bloom | 0.325 | 0.337 | 0.065 |
| lightprobe_radiance_map | 0.224 | 0.234 | 0.004 |
| ssao | 0.217 | 0.220 | 0.007 |
| main_transparent_pass_3d | 0.196 | 0.206 | 0.073 |
| uniform_allocation | 0.144 | 0.150 | 0.028 |
| clouds_trace | 0.135 | 0.158 | 0.001 |
| water_fft | 0.127 | 0.128 | 0.003 |
| upscaling | 0.119 | 0.124 | 0.004 |
| bin_unpacking | 0.101 | 0.109 | 0.020 |
| atmos_froxel_inject | 0.089 | 0.090 | 0.001 |
| tonemapping | 0.088 | 0.094 | 0.003 |
| water_velocity | 0.075 | 0.076 | 0.001 |
| clouds_composite | 0.064 | 0.066 | 0.002 |
| clustering | 0.064 | 0.077 | 0.022 |
| render_sky | 0.054 | 0.055 | 0.002 |
| early_mesh_preprocessing | 0.040 | 0.041 | 0.006 |
| clouds_resolve | 0.035 | 0.037 | 0.001 |
| clouds_shadow | 0.034 | 0.041 | 0.003 |
| water_mips | 0.026 | 0.024 | 0.002 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.022 | 0.024 | 0.001 |
| per_view_shadow_pass | 0.022 | 0.030 | 0.007 |
| lightprobe_copy | 0.015 | 0.016 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| atmosphere_environment | 0.013 | 0.015 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.004 | 0.009 | 0.002 |
| blas_build | 0.002 | 0.002 | 0.006 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 15.792 | 0.000 | 4018.42 | 23577.5 |
| render/thread/ms | 9.162 | 10.179 | 19.90 | 13678.7 |
| main/busy/ms | 6.572 | 7.679 | 8.80 | 9812.7 |
| render/busy/ms | 6.449 | 7.527 | 11.25 | 9627.7 |
| render/graph/ms | 3.505 | 4.484 | 7.62 | 5232.7 |
| render/acquire/ms | 2.268 | 3.292 | 4.23 | 3386.8 |
| render/present/ms | 0.445 | 0.553 | 6.54 | 664.2 |
| bigspace/total/ms | 0.370 | 0.643 | 1.82 | 553.0 |
| terrain/update/ms | 0.238 | 0.307 | 0.62 | 355.1 |
| pipelines/compile_ms | 0.211 | 0.000 | 124.48 | 314.7 |
| terrain/select/ms | 0.207 | 0.266 | 0.59 | 308.6 |
| pipelines/front_ms | 0.203 | 0.000 | 119.79 | 302.9 |
| controller/move/ms | 0.182 | 0.286 | 0.66 | 271.8 |
| stream/drive/ms | 0.171 | 0.198 | 0.42 | 255.8 |
| far/select/ms | 0.152 | 0.180 | 0.34 | 227.5 |
| stream/tick/ms | 0.147 | 0.170 | 0.39 | 218.8 |
| bigspace/high_precision/ms | 0.135 | 0.295 | 1.29 | 201.1 |
| controller/feed/ms | 0.112 | 0.162 | 0.29 | 167.9 |
| bigspace/low_precision/ms | 0.093 | 0.221 | 1.12 | 139.2 |
| anim/evaluate/ms | 0.067 | 0.152 | 1.15 | 100.7 |
| anim/evaluate_cpu/ms | 0.065 | 0.081 | 0.36 | 96.7 |
| physics/step/ms | 0.033 | 0.060 | 0.98 | 48.9 |
| weather/map/ms | 0.025 | 0.032 | 0.24 | 37.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.04 | 16.4 |
| pipelines/driver_ms | 0.008 | 0.000 | 4.70 | 11.8 |
| script/update/ms | 0.006 | 0.008 | 0.25 | 9.4 |
| physics/insert/ms | 0.006 | 0.000 | 0.58 | 9.2 |
| controller/avoid/ms | 0.006 | 0.007 | 0.03 | 8.3 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.0 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 6.3 |
| script/ui/ms | 0.003 | 0.005 | 0.06 | 4.7 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| ecs/entities | count | 47.0 k | 47.0 k | 47.0 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 9.05 | 11.25 | 8.94 |
| gpu/timed/ms | ms | 9.05 | 10.73 | 8.94 |
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
| phases/opaque3d/batch_sets | count | 14 | 14 | 14 |
| phases/opaque3d/bins | count | 1309 | 1309 | 1309 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3771 | 3771 | 3771 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 13 | 13 | 13 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 14 | 14 |
| phases/opaque3d_prepass/bins | count | 1309 | 1309 | 1309 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3771 | 3771 | 3771 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 13 | 13 | 13 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 23 | 23 | 23 |
| phases/shadow/bins | count | 2036 | 2036 | 2036 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5353 | 5353 | 5353 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 23 | 23 | 23 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 602 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 6 | 0 |
| preprocess/bind_groups | count | 0 | 4 | 0 |
| preprocess/bind_groups_created | count | 0 | 4 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 154 | 154 | 154 |
| process/cpu_usage | % | 22.96 | 39.42 | 28.85 |
| process/mem_usage | GiB | 2.16 | 2.18 | 2.18 |
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
| started_load_count |  | 2597 | 2599 | 2599 |
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
| system/cpu_usage | % | 30.97 | 49.01 | 35.81 |
| system/mem_usage | % | 50.21 | 50.28 | 50.28 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 67 | 67 | 67 |
| views/all/bins | count | 5218 | 5218 | 5218 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.6 k | 14.6 k |
| views/all/pipelines | count | 63 | 63 | 63 |
| views/all/pipelines_distinct | count | 55 | 55 | 55 |
| views/all/work_items | count | 14.8 k | 14.8 k | 14.8 k |
| views/cascade/batch_sets | count | 23 | 23 | 23 |
| views/cascade/bins | count | 2036 | 2036 | 2036 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5353 | 5353 | 5353 |
| views/cascade/pipelines | count | 23 | 23 | 23 |
| views/cascade/work_items | count | 5353 | 5353 | 5353 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 44 | 44 | 44 |
| views/main/bins | count | 3182 | 3182 | 3182 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9294 | 9294 | 9294 |
| views/main/pipelines | count | 40 | 40 | 40 |
| views/main/work_items | count | 9472 | 9472 | 9472 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.6 k | 24.6 k | 24.6 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 3 | 156 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.32 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| physics/inserts | count | 0.53 | 167 | 785 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.60 | 1 | 900 |
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
| surface/shelter_redraws | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 3, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 582.3 | main/busy/ms 559, surface/advance_cpu_ms 65.25, surface/cell_upload_ms 0.70, stream/drive/ms 0.36, weather/map/ms 0.13, stream/tick/ms 0.11, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.03, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.99 | 411.4 | main/busy/ms 600, render/thread/ms 362, render/busy/ms 288, render/acquire/ms 72.38, surface/advance_cpu_ms 65.25, render/graph/ms 4.88, render/present/ms 1.99, stream/drive/ms 0.94, surface/cell_upload_ms 0.70, terrain/update/ms 0.48, stream/tick/ms 0.47, bigspace/total/ms 0.39 |
| warm-up | 1.31 | 74.7 | pipelines/wait_ms 187, pipelines/compile_ms 95.26, pipelines/front_ms 87.31, main/busy/ms 58.72, render/thread/ms 51.46, render/busy/ms 49.80, pipelines/driver_ms 7.95, render/graph/ms 3.64, stream/drive/ms 1.53, render/present/ms 1.39, stream/tick/ms 1.00, terrain/update/ms 0.67 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\push_nocull\balmora@2560x1440_off=cpuculltag-1 --bench-baseline none --off cpuculltag
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 13%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.7 W · 74 °C (max 77) · VRAM 3920 MiB · throttled 100% of samples
