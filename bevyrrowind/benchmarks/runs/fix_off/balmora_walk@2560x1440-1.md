# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5881 frames over 65.6 s · **median 10.84 ms (92 fps)** · mean 11.16 · p95 13.31 · p99 14.95 (1% low 67 fps) · worst 24.37 ms · 30 spikes over 1.5× median · 15.5 stutters/min (work 28.3/min)
- warm-up: 473 frames over 5.8 s, worst 576.9 ms
- hitches (> 27.1 ms): **0** measured, 4 in warm-up
- timed GPU passes: 7.17 ms of a 11.16 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.84 ms, limit 11 ms
- **FAIL** frame_ms: 17 stutters (> 1.5x moving median of 60) in 1.09 min = 15.54/min, limit 1/min
- pass worst frame 24.37 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.073 | 2.097 | 0.173 |
| main_opaque_pass_3d | 1.389 | 1.745 | 0.044 |
| early prepass | 0.539 | 1.106 | 0.074 |
| lightprobe_irradiance_map | 0.447 | 0.762 | 0.001 |
| bloom | 0.317 | 0.323 | 0.062 |
| atmosphere_luts | 0.308 | 0.369 | 0.003 |
| lightprobe_radiance_map | 0.224 | 0.237 | 0.004 |
| ssao | 0.204 | 0.208 | 0.007 |
| uniform_allocation | 0.169 | 0.180 | 0.015 |
| auxiliary_views | 0.164 | 0.526 | 0.150 |
| clouds_trace | 0.126 | 0.212 | 0.001 |
| water_fft | 0.126 | 0.128 | 0.002 |
| bin_unpacking | 0.124 | 0.139 | 0.011 |
| upscaling | 0.118 | 0.127 | 0.004 |
| atmos_froxel_inject | 0.093 | 0.099 | 0.001 |
| tonemapping | 0.086 | 0.088 | 0.003 |
| water_velocity | 0.081 | 0.085 | 0.001 |
| clustering | 0.072 | 0.067 | 0.018 |
| clouds_composite | 0.063 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.062 | 0.104 | 0.103 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.052 | 0.053 | 0.002 |
| early_mesh_preprocessing | 0.041 | 0.048 | 0.004 |
| clouds_resolve | 0.035 | 0.037 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.024 | 0.027 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.023 | 0.006 |
| lightprobe_copy | 0.013 | 0.014 | 0.003 |
| lightprobe_downsampling_first_pass | 0.013 | 0.013 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.012 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.540 | 11.628 | 22.74 | 56104.7 |
| render/busy/ms | 9.028 | 11.102 | 22.09 | 53093.3 |
| main/busy/ms | 8.214 | 9.908 | 17.23 | 48304.7 |
| render/graph/ms | 4.588 | 5.818 | 12.79 | 26979.4 |
| pipelines/wait_ms | 4.538 | 0.000 | 7333.91 | 26686.9 |
| bigspace/total/ms | 0.437 | 0.749 | 1.95 | 2572.4 |
| render/present/ms | 0.425 | 0.512 | 4.20 | 2499.9 |
| terrain/update/ms | 0.284 | 0.365 | 1.51 | 1672.0 |
| stream/drive/ms | 0.260 | 0.434 | 7.90 | 1530.9 |
| terrain/select/ms | 0.249 | 0.319 | 0.77 | 1466.4 |
| controller/feed/ms | 0.244 | 0.302 | 0.67 | 1435.4 |
| far/select/ms | 0.213 | 0.365 | 0.76 | 1255.0 |
| stream/tick/ms | 0.209 | 0.362 | 0.62 | 1228.4 |
| anim/evaluate/ms | 0.173 | 0.261 | 2.07 | 1018.8 |
| bigspace/low_precision/ms | 0.171 | 0.308 | 1.70 | 1005.6 |
| anim/evaluate_cpu/ms | 0.156 | 0.197 | 0.47 | 916.6 |
| pipelines/compile_ms | 0.148 | 0.000 | 155.59 | 872.8 |
| pipelines/front_ms | 0.143 | 0.000 | 148.90 | 841.9 |
| controller/move/ms | 0.132 | 0.239 | 0.53 | 777.8 |
| bigspace/high_precision/ms | 0.102 | 0.182 | 1.48 | 600.4 |
| render/acquire/ms | 0.087 | 0.111 | 0.34 | 511.5 |
| anim/apply/ms | 0.078 | 0.106 | 0.99 | 460.9 |
| physics/insert/ms | 0.053 | 0.444 | 1.12 | 314.4 |
| physics/step/ms | 0.048 | 0.084 | 0.78 | 280.8 |
| weather/map/ms | 0.026 | 0.035 | 0.21 | 155.6 |
| controller/avoid/ms | 0.011 | 0.017 | 0.33 | 66.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.04 | 63.0 |
| script/update/ms | 0.007 | 0.014 | 0.19 | 41.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.35 | 31.8 |
| pipelines/driver_ms | 0.005 | 0.000 | 6.69 | 30.9 |
| script/input/ms | 0.005 | 0.006 | 0.07 | 27.3 |
| stream/spawn/ms | 0.004 | 0.000 | 1.09 | 25.9 |
| script/ui/ms | 0.004 | 0.006 | 0.11 | 25.6 |
| controller/write/ms | 0.003 | 0.005 | 0.16 | 19.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.66 | 15.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 6.0 |
| shapes/activate/ms | 0.001 | 0.002 | 0.03 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.53 | 5.4 |
| weather/apply/ms | 0.000 | 0.000 | 0.02 | 1.5 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
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
| frame/work/ms | ms | 8.80 | 22.09 | 8.10 |
| gpu/timed/ms | ms | 7.16 | 10.39 | 7.81 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 813 | 1420 | 617 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 813 | 1420 | 617 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1290 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3409 | 5129 | 3179 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1290 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3409 | 5129 | 3179 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 41 | 238 | 29 |
| phases/shadow/bins | count | 3830 | 5399 | 2252 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9628 | 12.8 k | 5703 |
| phases/shadow/pipelines | count | 115 | 248 | 29 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1176 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 172 | 174 | 174 |
| pipelines/waiting | count | 0 | 7 | 0 |
| preprocess/bind_groups | count | 191 | 626 | 166 |
| preprocess/bind_groups_created | count | 191 | 626 | 166 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 23.94 | 37.93 | 17.31 |
| process/mem_usage | GiB | 2.97 | 3.03 | 3.03 |
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
| started_load_count |  | 3473 | 3541 | 3541 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 435 | 530 | 398 |
| statics/materials_bound | count | 435 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.27 | 49.77 | 27.28 |
| system/mem_usage | % | 43.60 | 43.75 | 43.75 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 85 | 278 | 73 |
| views/all/bins | count | 6506 | 8275 | 5732 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.2 k | 25.4 k | 13.3 k |
| views/all/pipelines | count | 157 | 286 | 69 |
| views/all/pipelines_distinct | count | 60 | 68 | 55 |
| views/all/work_items | count | 17.4 k | 24.6 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 29 |
| views/cascade/bins | count | 2618 | 3154 | 2252 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8160 | 10.2 k | 5703 |
| views/cascade/pipelines | count | 36 | 50 | 29 |
| views/cascade/work_items | count | 8160 | 10.2 k | 5703 |
| views/lamp_face/batch_sets | count | 5 | 193 | 0 |
| views/lamp_face/bins | count | 1101 | 2449 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1238 | 2817 | 0 |
| views/lamp_face/pipelines | count | 75 | 201 | 0 |
| views/lamp_face/work_items | count | 71 | 2523 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2882 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8434 | 13.0 k | 7592 |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 8597 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 58.17 | 340 | 342.1 k |
| anim/events/count | count | 0.26 | 4 | 1515 |
| assets/Image/added | count | 0.05 | 28 | 313 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.70 | 160 | 4116 |
| assets/Mesh/removed | count | 0.83 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 29 | 470 |
| assets/StandardMaterial/modified | count | 3.68 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.88 | 1299 | 5194 |
| physics/inserts | count | 0.53 | 110 | 3091 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 161 | 2335 |
| physics/steps | count | 0.67 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 7 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 24 | 378 |
| stream/upload/bytes | bytes | 5756 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1262 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.1 ms; slowest 4, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 576.9 | main/busy/ms 556, surface/advance_cpu_ms 64.22, surface/cell_upload_ms 0.69, stream/drive/ms 0.38, weather/map/ms 0.13, bigspace/total/ms 0.10, stream/tick/ms 0.10, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.99 | 412.6 | main/busy/ms 590, render/thread/ms 378, render/busy/ms 308, render/acquire/ms 67.42, surface/advance_cpu_ms 64.22, render/graph/ms 4.51, render/present/ms 2.15, stream/drive/ms 1.24, surface/cell_upload_ms 0.69, bigspace/total/ms 0.59, stream/tick/ms 0.58, terrain/update/ms 0.57 |
| warm-up | 1.24 | 87.0 | pipelines/compile_ms 191, pipelines/front_ms 174, main/busy/ms 54.05, render/thread/ms 50.81, render/busy/ms 49.04, pipelines/driver_ms 17.01, pipelines/wait_ms 13.01, render/graph/ms 3.84, stream/drive/ms 2.07, stream/tick/ms 1.79, render/present/ms 1.37, terrain/update/ms 0.72 |
| warm-up | 3.17 | 27.7 | pipelines/wait_ms 10.5 k, main/busy/ms 34.60, render/thread/ms 32.42, render/busy/ms 31.16, pipelines/compile_ms 29.01, pipelines/front_ms 26.74, render/graph/ms 5.80, pipelines/driver_ms 2.27, stream/drive/ms 1.30, bigspace/total/ms 1.04, render/present/ms 1.01, stream/tick/ms 0.58 |

## Run

- `../target/profiling/bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\fix_off\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 2%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **76%** (min 55%) · 2583 MHz (min 2580) · memory 8001 MHz · 75.7 W · 68 °C (max 73) · VRAM 3974 MiB · throttled 100% of samples
