# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1604 frames over 15.0 s · **median 9.31 ms (107 fps)** · mean 9.36 · p95 10.40 · p99 10.95 (1% low 91 fps) · worst 16.59 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 305 frames over 4.0 s, worst 516.7 ms
- hitches (> 25.0 ms): **0** measured, 5 in warm-up
- timed GPU passes: 8.57 ms of a 9.36 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.31 ms, limit 11 ms
- pass worst frame 16.59 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.158 | 2.604 | 0.046 |
| dlss_super_resolution | 2.073 | 2.369 | 0.195 |
| atmosphere_luts | 1.062 | 1.298 | 0.003 |
| early prepass | 0.607 | 0.759 | 0.081 |
| lightprobe_irradiance_map | 0.425 | 0.560 | 0.001 |
| bloom | 0.324 | 0.338 | 0.071 |
| ssao | 0.267 | 0.602 | 0.008 |
| lightprobe_radiance_map | 0.223 | 0.231 | 0.005 |
| main_transparent_pass_3d | 0.177 | 0.184 | 0.119 |
| uniform_allocation | 0.147 | 0.154 | 0.026 |
| water_fft | 0.134 | 0.128 | 0.003 |
| upscaling | 0.119 | 0.130 | 0.005 |
| bin_unpacking | 0.100 | 0.105 | 0.018 |
| tonemapping | 0.089 | 0.095 | 0.004 |
| atmos_froxel_inject | 0.077 | 0.079 | 0.002 |
| water_velocity | 0.076 | 0.076 | 0.002 |
| clouds_trace | 0.067 | 0.086 | 0.001 |
| clouds_composite | 0.065 | 0.066 | 0.002 |
| clustering | 0.065 | 0.079 | 0.022 |
| render_sky | 0.055 | 0.055 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.044 | 0.006 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| water_mips | 0.029 | 0.026 | 0.002 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.022 | 0.026 | 0.001 |
| atmosphere_environment | 0.022 | 0.017 | 0.003 |
| per_view_shadow_pass | 0.021 | 0.023 | 0.005 |
| lightprobe_copy | 0.014 | 0.015 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| night_sky | 0.007 | 0.012 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.003 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| blas_build | 0.001 | 0.001 | 0.005 |
| surface_trail | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 49.087 | 0.000 | 8225.22 | 78735.1 |
| render/thread/ms | 8.340 | 9.408 | 15.52 | 13377.9 |
| render/busy/ms | 6.661 | 7.873 | 14.94 | 10684.4 |
| main/busy/ms | 6.658 | 7.590 | 14.34 | 10679.4 |
| render/graph/ms | 3.664 | 4.694 | 12.08 | 5876.3 |
| render/acquire/ms | 1.225 | 2.315 | 3.81 | 1965.1 |
| pipelines/compile_ms | 1.016 | 0.000 | 141.33 | 1629.8 |
| pipelines/front_ms | 0.983 | 0.000 | 138.22 | 1576.0 |
| render/present/ms | 0.454 | 0.563 | 2.00 | 728.3 |
| bigspace/total/ms | 0.394 | 0.863 | 8.36 | 631.6 |
| terrain/update/ms | 0.234 | 0.297 | 0.69 | 375.4 |
| terrain/select/ms | 0.203 | 0.255 | 0.65 | 325.1 |
| stream/drive/ms | 0.190 | 0.290 | 0.55 | 304.1 |
| controller/move/ms | 0.175 | 0.426 | 0.81 | 280.6 |
| far/select/ms | 0.170 | 0.257 | 0.47 | 272.3 |
| stream/tick/ms | 0.163 | 0.255 | 0.49 | 261.0 |
| controller/feed/ms | 0.139 | 0.196 | 0.54 | 222.7 |
| bigspace/high_precision/ms | 0.134 | 0.317 | 1.46 | 214.4 |
| bigspace/low_precision/ms | 0.096 | 0.224 | 8.12 | 154.4 |
| anim/evaluate/ms | 0.081 | 0.201 | 1.01 | 129.6 |
| anim/evaluate_cpu/ms | 0.061 | 0.083 | 0.25 | 97.8 |
| pipelines/driver_ms | 0.034 | 0.000 | 4.64 | 53.8 |
| physics/step/ms | 0.032 | 0.062 | 0.93 | 51.6 |
| physics/insert/ms | 0.018 | 0.167 | 0.72 | 29.1 |
| weather/map/ms | 0.017 | 0.022 | 0.20 | 27.4 |
| weather/eval/ms | 0.011 | 0.014 | 0.05 | 17.5 |
| script/update/ms | 0.007 | 0.010 | 0.03 | 10.9 |
| controller/avoid/ms | 0.006 | 0.008 | 0.03 | 9.4 |
| script/ui/ms | 0.005 | 0.007 | 0.24 | 7.9 |
| script/input/ms | 0.005 | 0.006 | 0.06 | 7.9 |
| anim/apply/ms | 0.005 | 0.006 | 0.15 | 7.5 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.7 |
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
| anim/a2/count | count | 0 | 16 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 43 | 7 |
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
| frame/work/ms | ms | 8.52 | 14.94 | 8.55 |
| gpu/timed/ms | ms | 8.52 | 9.88 | 8.55 |
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
| phases/opaque3d/batch_sets | count | 14 | 15 | 14 |
| phases/opaque3d/bins | count | 3899 | 3900 | 3899 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 9299 | 9299 | 9299 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 13 | 14 | 13 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/bins | count | 3899 | 3900 | 3899 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 9299 | 9299 | 9299 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 13 | 14 | 13 |
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
| physics/pending | count | 0 | 1871 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 30 | 0 |
| preprocess/bind_groups | count | 0 | 101 | 0 |
| preprocess/bind_groups_created | count | 0 | 101 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 145 | 145 | 145 |
| process/cpu_usage | % | 30.95 | 46.63 | 23.98 |
| process/mem_usage | GiB | 2.19 | 2.26 | 2.26 |
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
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.46 | 52.92 | 37.90 |
| system/mem_usage | % | 50.38 | 50.64 | 50.64 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 63 | 65 | 63 |
| views/all/bins | count | 15.2 k | 15.2 k | 15.2 k |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 45.0 k | 45.0 k | 45.0 k |
| views/all/pipelines | count | 59 | 61 | 59 |
| views/all/pipelines_distinct | count | 55 | 57 | 55 |
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
| views/main/batch_sets | count | 44 | 46 | 44 |
| views/main/bins | count | 8472 | 8474 | 8472 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 25.0 k | 25.0 k | 25.0 k |
| views/main/pipelines | count | 40 | 42 | 40 |
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
| anim/bones_written/count | count | 0.23 | 363 | 363 |
| anim/events/count | count | 0.07 | 2 | 113 |
| assets/Image/added | count | 0.00 | 3 | 6 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.04 | 39 | 59 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.00 | 3 | 6 |
| assets/StandardMaterial/modified | count | 3.09 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.30 | 2192 | 3683 |
| physics/inserts | count | 1.14 | 208 | 1836 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.56 | 1 | 900 |
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

## Hitches (> 25.0 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.52 | 516.7 | main/busy/ms 494, surface/advance_cpu_ms 0.73, stream/drive/ms 0.31, surface/cell_upload_ms 0.28, stream/tick/ms 0.09, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.04, weather/map/ms 0.03, weather/eval/ms 0.02, script/update/ms 0.02, script/ui/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.75 | 236.3 | main/busy/ms 529, render/thread/ms 199, render/busy/ms 125, render/acquire/ms 72.14, render/graph/ms 5.12, render/present/ms 2.02, stream/drive/ms 0.85, surface/advance_cpu_ms 0.73, terrain/update/ms 0.50, stream/tick/ms 0.42, terrain/select/ms 0.33, surface/cell_upload_ms 0.28 |
| warm-up | 0.94 | 182.6 | pipelines/wait_ms 2497, render/thread/ms 208, pipelines/compile_ms 187, pipelines/front_ms 172, render/busy/ms 133, render/acquire/ms 72.34, main/busy/ms 50.35, terrain/update/ms 20.70, pipelines/driver_ms 15.13, render/graph/ms 7.98, render/present/ms 2.98, stream/drive/ms 1.01 |
| warm-up | 1.17 | 83.0 | main/busy/ms 61.49, render/thread/ms 55.22, render/busy/ms 53.60, render/graph/ms 3.69, render/present/ms 1.35, stream/drive/ms 1.23, stream/tick/ms 0.98, terrain/update/ms 0.49, terrain/select/ms 0.39, render/acquire/ms 0.27, bigspace/total/ms 0.21, bigspace/high_precision/ms 0.09 |
| warm-up | 2.79 | 25.1 | pipelines/wait_ms 5015, main/busy/ms 42.77, render/thread/ms 26.27, render/busy/ms 24.64, pipelines/compile_ms 18.89, pipelines/front_ms 17.38, render/graph/ms 4.79, terrain/update/ms 2.55, pipelines/driver_ms 1.52, render/present/ms 1.41, stream/drive/ms 1.09, bigspace/total/ms 1.09 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\push_all\balmora_night@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 13%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.6 W · 74 °C (max 76) · VRAM 3668 MiB · throttled 100% of samples
