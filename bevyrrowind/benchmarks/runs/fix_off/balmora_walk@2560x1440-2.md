# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5929 frames over 65.6 s · **median 10.69 ms (94 fps)** · mean 11.07 · p95 13.39 · p99 14.80 (1% low 68 fps) · worst 26.47 ms · 28 spikes over 1.5× median · 14.6 stutters/min (work 17.4/min)
- warm-up: 468 frames over 5.8 s, worst 584.8 ms
- hitches (> 26.7 ms): **0** measured, 5 in warm-up
- timed GPU passes: 7.13 ms of a 11.07 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.69 ms, limit 11 ms
- **FAIL** frame_ms: 16 stutters (> 1.5x moving median of 60) in 1.09 min = 14.62/min, limit 1/min
- pass worst frame 26.47 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.075 | 2.100 | 0.177 |
| main_opaque_pass_3d | 1.386 | 1.732 | 0.044 |
| early prepass | 0.540 | 1.091 | 0.075 |
| lightprobe_irradiance_map | 0.447 | 0.756 | 0.001 |
| bloom | 0.318 | 0.324 | 0.063 |
| atmosphere_luts | 0.308 | 0.369 | 0.003 |
| lightprobe_radiance_map | 0.224 | 0.236 | 0.004 |
| ssao | 0.204 | 0.208 | 0.007 |
| uniform_allocation | 0.168 | 0.179 | 0.016 |
| auxiliary_views | 0.137 | 0.545 | 0.123 |
| clouds_trace | 0.127 | 0.216 | 0.001 |
| water_fft | 0.126 | 0.128 | 0.002 |
| bin_unpacking | 0.122 | 0.139 | 0.012 |
| upscaling | 0.118 | 0.127 | 0.004 |
| atmos_froxel_inject | 0.094 | 0.099 | 0.001 |
| tonemapping | 0.086 | 0.087 | 0.003 |
| water_velocity | 0.081 | 0.085 | 0.001 |
| clustering | 0.072 | 0.067 | 0.018 |
| clouds_composite | 0.062 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.059 | 0.101 | 0.102 |
| surface_trail | 0.057 | 0.057 | 0.002 |
| render_sky | 0.053 | 0.053 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.048 | 0.005 |
| clouds_resolve | 0.034 | 0.036 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.023 | 0.027 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.023 | 0.006 |
| lightprobe_copy | 0.013 | 0.014 | 0.003 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.451 | 11.751 | 24.78 | 56034.0 |
| render/busy/ms | 8.935 | 11.217 | 24.03 | 52973.8 |
| main/busy/ms | 8.202 | 9.892 | 17.20 | 48631.4 |
| render/graph/ms | 4.454 | 5.835 | 12.75 | 26409.3 |
| pipelines/wait_ms | 3.049 | 0.000 | 4503.26 | 18074.6 |
| bigspace/total/ms | 0.438 | 0.723 | 2.04 | 2594.3 |
| render/present/ms | 0.428 | 0.522 | 2.06 | 2539.7 |
| terrain/update/ms | 0.285 | 0.364 | 1.85 | 1689.9 |
| stream/drive/ms | 0.261 | 0.431 | 2.23 | 1544.9 |
| controller/feed/ms | 0.251 | 0.318 | 0.57 | 1485.7 |
| terrain/select/ms | 0.250 | 0.319 | 0.97 | 1482.1 |
| far/select/ms | 0.214 | 0.366 | 0.63 | 1269.7 |
| stream/tick/ms | 0.211 | 0.366 | 0.72 | 1253.1 |
| bigspace/low_precision/ms | 0.175 | 0.326 | 1.72 | 1037.0 |
| anim/evaluate_cpu/ms | 0.162 | 0.204 | 0.60 | 960.2 |
| controller/move/ms | 0.137 | 0.242 | 0.47 | 814.5 |
| anim/evaluate/ms | 0.128 | 0.216 | 1.20 | 760.7 |
| pipelines/compile_ms | 0.128 | 0.000 | 152.38 | 760.2 |
| pipelines/front_ms | 0.124 | 0.000 | 147.12 | 733.0 |
| bigspace/high_precision/ms | 0.101 | 0.182 | 1.43 | 598.5 |
| render/acquire/ms | 0.088 | 0.112 | 0.32 | 520.5 |
| anim/apply/ms | 0.080 | 0.120 | 1.28 | 476.4 |
| physics/step/ms | 0.047 | 0.083 | 0.61 | 280.6 |
| physics/insert/ms | 0.045 | 0.412 | 1.01 | 269.6 |
| weather/map/ms | 0.027 | 0.034 | 0.15 | 158.6 |
| controller/avoid/ms | 0.012 | 0.018 | 0.27 | 68.5 |
| weather/eval/ms | 0.011 | 0.015 | 0.06 | 63.6 |
| script/update/ms | 0.007 | 0.014 | 0.12 | 42.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.19 | 32.0 |
| script/input/ms | 0.005 | 0.006 | 0.18 | 27.9 |
| pipelines/driver_ms | 0.005 | 0.000 | 5.26 | 27.3 |
| stream/spawn/ms | 0.005 | 0.000 | 1.04 | 26.8 |
| script/ui/ms | 0.004 | 0.006 | 0.17 | 25.7 |
| controller/write/ms | 0.004 | 0.006 | 0.26 | 21.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.70 | 16.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.6 |
| shapes/activate/ms | 0.001 | 0.002 | 0.25 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.44 | 5.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| ecs/entities | count | 59.2 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 8.68 | 24.03 | 8.10 |
| gpu/timed/ms | ms | 7.14 | 10.54 | 7.36 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 823 | 1420 | 617 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 823 | 1420 | 617 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1298 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3432 | 5140 | 3180 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1298 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3432 | 5140 | 3180 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 37 | 234 | 28 |
| phases/shadow/bins | count | 3860 | 5338 | 2214 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9668 | 12.7 k | 5665 |
| phases/shadow/pipelines | count | 116 | 248 | 28 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2659 | 3268 | 2349 |
| physics/items | count | 211 | 476 | 19 |
| physics/pending | count | 0 | 296 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 172 | 174 | 174 |
| pipelines/waiting | count | 0 | 7 | 0 |
| preprocess/bind_groups | count | 182 | 618 | 164 |
| preprocess/bind_groups_created | count | 182 | 618 | 164 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 23.92 | 37.32 | 25.50 |
| process/mem_usage | GiB | 2.94 | 3.00 | 3.00 |
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
| started_load_count |  | 3474 | 3530 | 3530 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 435 | 530 | 398 |
| statics/materials_bound | count | 435 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.23 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.63 | 44.47 | 27.77 |
| system/mem_usage | % | 43.66 | 43.73 | 43.73 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 81 | 274 | 72 |
| views/all/bins | count | 6513 | 8203 | 5694 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.4 k | 25.0 k | 13.3 k |
| views/all/pipelines | count | 158 | 287 | 68 |
| views/all/pipelines_distinct | count | 60 | 69 | 55 |
| views/all/work_items | count | 17.5 k | 24.5 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 28 |
| views/cascade/bins | count | 2606 | 3167 | 2214 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8160 | 10.1 k | 5665 |
| views/cascade/pipelines | count | 36 | 50 | 28 |
| views/cascade/work_items | count | 8160 | 10.1 k | 5665 |
| views/lamp_face/batch_sets | count | 0 | 189 | 0 |
| views/lamp_face/bins | count | 1114 | 2413 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1259 | 2771 | 0 |
| views/lamp_face/pipelines | count | 76 | 202 | 0 |
| views/lamp_face/work_items | count | 0 | 2481 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2902 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8498 | 13.1 k | 7594 |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 8661 | 13.3 k | 7902 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 47.64 | 221 | 282.5 k |
| anim/events/count | count | 0.27 | 3 | 1591 |
| assets/Image/added | count | 0.05 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.69 | 160 | 4086 |
| assets/Mesh/removed | count | 0.83 | 1242 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.65 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.80 | 1378 | 4739 |
| physics/inserts | count | 0.43 | 111 | 2538 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 161 | 2335 |
| physics/steps | count | 0.66 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 7 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 22 | 378 |
| stream/upload/bytes | bytes | 5710 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1252 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.7 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 584.8 | main/busy/ms 563, surface/advance_cpu_ms 67.60, surface/cell_upload_ms 0.70, stream/drive/ms 0.43, weather/map/ms 0.19, stream/tick/ms 0.11, bigspace/total/ms 0.10, bigspace/high_precision/ms 0.05, script/update/ms 0.02, weather/eval/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.99 | 408.3 | main/busy/ms 597, render/thread/ms 374, render/busy/ms 298, render/acquire/ms 73.93, surface/advance_cpu_ms 67.60, render/graph/ms 5.23, render/present/ms 1.85, stream/drive/ms 1.10, surface/cell_upload_ms 0.70, stream/tick/ms 0.50, terrain/update/ms 0.47, terrain/select/ms 0.33 |
| warm-up | 1.25 | 116.3 | pipelines/compile_ms 237, pipelines/front_ms 195, pipelines/wait_ms 195, render/thread/ms 76.35, render/busy/ms 74.50, main/busy/ms 62.71, pipelines/driver_ms 41.85, render/graph/ms 4.10, stream/drive/ms 1.98, render/present/ms 1.36, stream/tick/ms 1.12, terrain/update/ms 0.84 |
| warm-up | 1.29 | 31.8 | pipelines/wait_ms 207, pipelines/compile_ms 48.25, pipelines/front_ms 44.94, render/thread/ms 40.91, render/busy/ms 39.69, main/busy/ms 20.83, stream/drive/ms 3.39, pipelines/driver_ms 3.32, render/graph/ms 2.39, stream/tick/ms 1.44, render/present/ms 0.99, stream/spawn/ms 0.90 |
| warm-up | 2.84 | 31.2 | main/busy/ms 48.55, render/thread/ms 24.07, render/busy/ms 22.59, render/graph/ms 5.71, stream/drive/ms 1.26, render/present/ms 1.24, bigspace/total/ms 0.80, stream/tick/ms 0.64, terrain/update/ms 0.59, terrain/select/ms 0.52, physics/insert/ms 0.47, anim/evaluate_cpu/ms 0.46 |

## Run

- `../target/profiling/bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\fix_off\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 2%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (256 samples): GPU load **76%** (min 56%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.2 W · 69 °C (max 74) · VRAM 3717 MiB · throttled 100% of samples
