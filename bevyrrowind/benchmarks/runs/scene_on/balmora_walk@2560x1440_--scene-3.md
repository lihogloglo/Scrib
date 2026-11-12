# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4283 frames over 65.6 s · **median 14.90 ms (67 fps)** · mean 15.33 · p95 18.55 · p99 20.93 (1% low 48 fps) · worst 62.39 ms · 19 spikes over 1.5× median · 10.1 stutters/min (work 13.7/min)
- warm-up: 362 frames over 6.4 s, worst 607.2 ms
- hitches (> 37.3 ms): **1** measured, 5 in warm-up
- timed GPU passes: 8.18 ms of a 15.33 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 14.90 ms, limit 11 ms
- **FAIL** frame_ms: 11 stutters (> 1.5x moving median of 60) in 1.09 min = 10.05/min, limit 1/min
- **FAIL** worst frame 62.39 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.388 | 3.176 | 0.202 |
| main_opaque_pass_3d | 1.551 | 2.507 | 0.056 |
| early prepass | 0.547 | 1.206 | 0.090 |
| lightprobe_irradiance_map | 0.495 | 0.982 | 0.001 |
| bloom | 0.369 | 0.720 | 0.077 |
| atmosphere_luts | 0.352 | 0.703 | 0.004 |
| lightprobe_radiance_map | 0.246 | 0.395 | 0.005 |
| ssao | 0.243 | 0.553 | 0.008 |
| auxiliary_views | 0.228 | 0.837 | 0.219 |
| uniform_allocation | 0.173 | 0.172 | 0.021 |
| upscaling | 0.139 | 0.177 | 0.005 |
| water_fft | 0.137 | 0.129 | 0.003 |
| bin_unpacking | 0.130 | 0.132 | 0.016 |
| clouds_trace | 0.127 | 0.215 | 0.001 |
| atmos_froxel_inject | 0.103 | 0.110 | 0.002 |
| tonemapping | 0.098 | 0.110 | 0.004 |
| water_velocity | 0.098 | 0.239 | 0.002 |
| surface_trail | 0.086 | 0.317 | 0.002 |
| early_mesh_preprocessing | 0.071 | 0.074 | 0.004 |
| clustering | 0.071 | 0.069 | 0.022 |
| clouds_composite | 0.066 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.064 | 0.113 | 0.117 |
| render_sky | 0.055 | 0.054 | 0.002 |
| gpu_instance_materialization | 0.051 | 0.049 | 0.005 |
| per_view_shadow_pass | 0.048 | 0.040 | 0.009 |
| clouds_resolve | 0.035 | 0.041 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.031 | 0.001 |
| water_mips | 0.026 | 0.024 | 0.002 |
| water_snapshot | 0.025 | 0.024 | 0.002 |
| range_unpacking | 0.021 | 0.019 | 0.004 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.013 | 0.001 |
| atmosphere_environment | 0.012 | 0.012 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.004 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.006 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 12.339 | 15.294 | 59.83 | 52849.3 |
| render/busy/ms | 11.748 | 14.632 | 58.79 | 50317.7 |
| main/busy/ms | 10.082 | 12.327 | 18.51 | 43179.4 |
| render/graph/ms | 5.356 | 7.038 | 49.96 | 22939.5 |
| bigspace/total/ms | 0.524 | 0.945 | 3.44 | 2245.4 |
| render/present/ms | 0.493 | 0.621 | 3.60 | 2112.0 |
| controller/feed/ms | 0.319 | 0.479 | 0.92 | 1365.8 |
| terrain/update/ms | 0.292 | 0.370 | 1.62 | 1252.6 |
| terrain/select/ms | 0.255 | 0.321 | 0.87 | 1091.2 |
| stream/drive/ms | 0.240 | 0.387 | 2.04 | 1028.3 |
| anim/evaluate/ms | 0.193 | 0.446 | 1.77 | 828.7 |
| far/select/ms | 0.190 | 0.284 | 0.71 | 815.2 |
| anim/evaluate_cpu/ms | 0.188 | 0.256 | 0.92 | 807.2 |
| stream/tick/ms | 0.187 | 0.250 | 0.75 | 802.4 |
| bigspace/low_precision/ms | 0.180 | 0.323 | 2.10 | 769.2 |
| controller/move/ms | 0.145 | 0.332 | 0.75 | 619.0 |
| bigspace/high_precision/ms | 0.124 | 0.282 | 1.79 | 532.0 |
| anim/apply/ms | 0.118 | 0.278 | 1.80 | 503.4 |
| render/acquire/ms | 0.098 | 0.131 | 0.49 | 419.7 |
| physics/step/ms | 0.068 | 0.099 | 0.59 | 289.2 |
| pipelines/compile_ms | 0.067 | 0.000 | 160.87 | 287.1 |
| pipelines/front_ms | 0.065 | 0.000 | 155.80 | 278.0 |
| physics/insert/ms | 0.049 | 0.400 | 1.13 | 211.5 |
| weather/map/ms | 0.029 | 0.037 | 0.32 | 122.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.07 | 48.4 |
| controller/avoid/ms | 0.011 | 0.014 | 0.28 | 46.6 |
| script/update/ms | 0.008 | 0.011 | 0.16 | 33.4 |
| stream/despawn/ms | 0.006 | 0.000 | 0.92 | 26.7 |
| script/ui/ms | 0.006 | 0.009 | 0.17 | 26.5 |
| script/input/ms | 0.006 | 0.008 | 0.29 | 25.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.14 | 23.2 |
| controller/write/ms | 0.003 | 0.005 | 0.03 | 15.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.63 | 13.9 |
| pipelines/driver_ms | 0.002 | 0.000 | 5.07 | 9.1 |
| stream/upload/ms | 0.001 | 0.000 | 0.47 | 6.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 4.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 4.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 1.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.09 | 0.3 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 20 | 21 | 19 |
| ecs/entities | count | 70.7 k | 79.8 k | 71.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 11.37 | 58.79 | 10.28 |
| gpu/timed/ms | ms | 8.13 | 12.13 | 8.37 |
| mesh_allocator_allocations |  meshes | 13.4 k | 14.7 k | 13.4 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 323.10 M | 323.10 M | 323.10 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 185 | 401 | 393 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 761 | 1201 | 597 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 185 | 401 | 393 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 761 | 1201 | 597 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/opaque3d/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d/bins | count | 1024 | 1173 | 1097 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1999 | 2074 | 2074 |
| phases/opaque3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/opaque3d/items | count | 3925 | 5688 | 3596 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1024 | 1173 | 1097 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1999 | 2074 | 2074 |
| phases/opaque3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/opaque3d_prepass/items | count | 3925 | 5688 | 3596 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 50 | 225 | 41 |
| phases/shadow/bins | count | 1956 | 2865 | 1308 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 7995 | 8370 | 8370 |
| phases/shadow/instance_slots | count | 49.2 k | 49.2 k | 49.2 k |
| phases/shadow/items | count | 8183 | 10.8 k | 5299 |
| phases/shadow/pipelines | count | 104 | 227 | 31 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 196 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 165 | 166 | 166 |
| pipelines/waiting | count | 0 | 1 | 0 |
| preprocess/bind_groups | count | 189 | 577 | 169 |
| preprocess/bind_groups_created | count | 182 | 570 | 162 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 21.15 | 42.31 | 17.31 |
| process/mem_usage | GiB | 2.67 | 2.71 | 2.69 |
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1594 | 1624 | 1624 |
| scene/chains/live | count | 1594 | 1624 | 1624 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 12.1 k | 12.7 k | 12.7 k |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 9654 | 10.3 k | 10.3 k |
| scene/slots/batch_capacity | count | 16.4 k | 16.4 k | 16.4 k |
| scene/slots/batch_capacity_bytes | bytes | 1.05 M | 1.05 M | 1.05 M |
| scene/slots/batch_live | count | 9654 | 10.3 k | 10.3 k |
| scene/slots/scene_bump | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/scene_live | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 5354 | 5458 | 5458 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 387 | 481 | 347 |
| statics/materials_bound | count | 387 | 481 | 347 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.06 | 0.11 | 0.07 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 618 | 712 | 541 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 36.27 | 59.82 | 32.19 |
| system/mem_usage | % | 63.47 | 63.76 | 63.66 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 101 | 271 | 91 |
| views/all/bins | count | 4338 | 5341 | 4288 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 13.3 k | 13.9 k | 13.9 k |
| views/all/instance_slots | count | 81.9 k | 81.9 k | 81.9 k |
| views/all/items | count | 18.0 k | 23.9 k | 13.7 k |
| views/all/pipelines | count | 144 | 267 | 69 |
| views/all/pipelines_distinct | count | 61 | 66 | 53 |
| views/all/work_items | count | 99.7 k | 105.4 k | 95.9 k |
| views/cascade/batch_sets | count | 46 | 57 | 41 |
| views/cascade/bins | count | 1746 | 2208 | 1308 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 7995 | 8370 | 8370 |
| views/cascade/instance_slots | count | 49.2 k | 49.2 k | 49.2 k |
| views/cascade/items | count | 7722 | 9943 | 5299 |
| views/cascade/pipelines | count | 37 | 48 | 31 |
| views/cascade/work_items | count | 56.9 k | 59.1 k | 54.4 k |
| views/lamp_face/batch_sets | count | 4 | 176 | 0 |
| views/lamp_face/bins | count | 214 | 780 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 365 | 1134 | 0 |
| views/lamp_face/pipelines | count | 63 | 184 | 0 |
| views/lamp_face/work_items | count | 34 | 996 | 0 |
| views/main/batch_sets | count | 50 | 54 | 50 |
| views/main/bins | count | 2464 | 2992 | 2980 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 5330 | 5580 | 5580 |
| views/main/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| views/main/items | count | 9396 | 13.7 k | 8386 |
| views/main/pipelines | count | 40 | 42 | 38 |
| views/main/work_items | count | 42.3 k | 46.6 k | 41.5 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 42.8 k | 49.5 k | 42.8 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 58.47 | 285 | 250.4 k |
| anim/events/count | count | 0.35 | 4 | 1515 |
| assets/Image/added | count | 0.07 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.83 | 232 | 3573 |
| assets/Mesh/removed | count | 1.14 | 1220 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 488 |
| assets/StandardMaterial/modified | count | 5.05 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.10 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.69 | 1562 | 7238 |
| physics/inserts | count | 0.56 | 93 | 2417 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.55 | 161 | 2335 |
| physics/steps | count | 0.92 | 3 | 3938 |
| physics/steps_dropped | count | 0.00 | 1 | 1 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 5 |
| scene/overflow/cells | count | 0.01 | 1 | 27 |
| scene/slots/overflow | count | 0.69 | 520 | 2950 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.08 | 16 | 351 |
| stream/upload/bytes | bytes | 7904 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1733 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 37.3 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.61 | 607.2 | main/busy/ms 582, surface/advance_cpu_ms 82.81, surface/cell_upload_ms 0.78, stream/drive/ms 0.49, weather/map/ms 0.16, stream/tick/ms 0.11, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.96 | 352.0 | main/busy/ms 619, render/thread/ms 307, render/busy/ms 233, surface/advance_cpu_ms 82.81, render/acquire/ms 71.86, render/graph/ms 6.14, render/present/ms 2.33, stream/drive/ms 1.16, surface/cell_upload_ms 0.78, stream/tick/ms 0.50, terrain/update/ms 0.40, weather/map/ms 0.27 |
| warm-up | 1.05 | 95.2 | render/thread/ms 331, pipelines/wait_ms 326, render/busy/ms 254, pipelines/compile_ms 90.31, pipelines/front_ms 83.92, render/acquire/ms 72.41, main/busy/ms 58.70, terrain/update/ms 19.96, render/graph/ms 11.92, pipelines/driver_ms 6.39, render/present/ms 3.78, stream/drive/ms 1.27 |
| warm-up | 1.33 | 108.2 | pipelines/wait_ms 1292, pipelines/compile_ms 129, pipelines/front_ms 117, main/busy/ms 78.97, render/thread/ms 63.50, render/busy/ms 61.05, pipelines/driver_ms 11.80, render/graph/ms 5.41, terrain/update/ms 4.92, stream/drive/ms 3.41, stream/tick/ms 2.53, render/present/ms 2.15 |
| warm-up | 3.73 | 38.7 | pipelines/wait_ms 6270, pipelines/compile_ms 210, pipelines/front_ms 200, main/busy/ms 38.43, render/thread/ms 27.12, render/busy/ms 25.17, pipelines/driver_ms 10.07, render/graph/ms 5.40, stream/drive/ms 1.74, render/present/ms 1.73, bigspace/total/ms 1.11, anim/evaluate_cpu/ms 0.88 |
| measure | 0.43 | 62.4 | render/thread/ms 72.84, render/busy/ms 69.19, render/graph/ms 54.85, main/busy/ms 20.43, render/present/ms 3.36, physics/insert/ms 1.13, bigspace/total/ms 1.00, terrain/update/ms 0.62, terrain/select/ms 0.55, stream/drive/ms 0.45, controller/feed/ms 0.40, stream/tick/ms 0.39 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\scene_on\balmora_walk@2560x1440_--scene-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 17%
- during the run: other processes 15% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **67%** (min 43%) · 2589 MHz (min 2580) · memory 8001 MHz · 64.2 W · 65 °C (max 71) · VRAM 4140 MiB · throttled 100% of samples
