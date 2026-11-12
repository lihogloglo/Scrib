# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cpuculltag · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6491 frames over 65.6 s · **median 9.98 ms (100 fps)** · mean 10.11 · p95 12.17 · p99 13.53 (1% low 74 fps) · worst 30.93 ms · 20 spikes over 1.5× median · 13.7 stutters/min (work 14.6/min)
- warm-up: 393 frames over 4.8 s, worst 588.5 ms
- hitches (> 25.0 ms): **2** measured, 5 in warm-up
- timed GPU passes: 7.81 ms of a 10.11 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.98 ms, limit 11 ms
- **FAIL** frame_ms: 15 stutters (> 1.5x moving median of 60) in 1.09 min = 13.71/min, limit 1/min
- pass worst frame 30.93 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.094 | 2.334 | 0.176 |
| main_opaque_pass_3d | 1.397 | 1.756 | 0.043 |
| atmosphere_luts | 0.844 | 1.050 | 0.003 |
| early prepass | 0.536 | 1.010 | 0.075 |
| lightprobe_irradiance_map | 0.442 | 0.686 | 0.001 |
| bloom | 0.323 | 0.329 | 0.063 |
| lightprobe_radiance_map | 0.224 | 0.234 | 0.004 |
| auxiliary_views | 0.210 | 0.708 | 0.174 |
| ssao | 0.207 | 0.208 | 0.007 |
| uniform_allocation | 0.183 | 0.188 | 0.030 |
| bin_unpacking | 0.131 | 0.142 | 0.023 |
| clouds_trace | 0.128 | 0.217 | 0.001 |
| water_fft | 0.126 | 0.128 | 0.003 |
| upscaling | 0.121 | 0.131 | 0.004 |
| atmos_froxel_inject | 0.093 | 0.098 | 0.001 |
| tonemapping | 0.087 | 0.090 | 0.003 |
| water_velocity | 0.081 | 0.085 | 0.001 |
| clustering | 0.069 | 0.070 | 0.018 |
| clouds_composite | 0.063 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.062 | 0.105 | 0.105 |
| surface_trail | 0.059 | 0.057 | 0.002 |
| render_sky | 0.053 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.044 | 0.049 | 0.005 |
| clouds_resolve | 0.034 | 0.035 | 0.001 |
| clouds_shadow | 0.032 | 0.034 | 0.003 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.023 | 0.025 | 0.001 |
| water_mips | 0.022 | 0.023 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.024 | 0.006 |
| lightprobe_copy | 0.014 | 0.015 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.011 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 12.391 | 0.000 | 7578.88 | 80427.5 |
| render/thread/ms | 8.920 | 10.801 | 29.72 | 57898.4 |
| render/busy/ms | 8.262 | 10.259 | 29.20 | 53629.2 |
| main/busy/ms | 8.023 | 9.595 | 29.34 | 52076.2 |
| render/graph/ms | 4.501 | 5.904 | 25.17 | 29216.3 |
| bigspace/total/ms | 0.521 | 1.161 | 2.40 | 3379.2 |
| render/present/ms | 0.512 | 1.231 | 3.52 | 3322.9 |
| terrain/update/ms | 0.291 | 0.371 | 2.44 | 1889.7 |
| controller/feed/ms | 0.259 | 0.336 | 0.64 | 1684.4 |
| terrain/select/ms | 0.258 | 0.330 | 0.75 | 1677.7 |
| stream/drive/ms | 0.235 | 0.326 | 2.08 | 1525.0 |
| bigspace/low_precision/ms | 0.216 | 0.631 | 1.66 | 1399.1 |
| pipelines/compile_ms | 0.215 | 0.000 | 154.35 | 1394.8 |
| pipelines/front_ms | 0.208 | 0.000 | 149.12 | 1350.4 |
| stream/tick/ms | 0.192 | 0.261 | 0.71 | 1244.1 |
| far/select/ms | 0.188 | 0.258 | 0.66 | 1220.6 |
| anim/evaluate_cpu/ms | 0.172 | 0.214 | 0.44 | 1118.5 |
| render/acquire/ms | 0.146 | 0.590 | 2.03 | 946.3 |
| bigspace/high_precision/ms | 0.139 | 0.511 | 1.68 | 902.8 |
| controller/move/ms | 0.137 | 0.288 | 0.63 | 887.4 |
| anim/evaluate/ms | 0.122 | 0.220 | 1.46 | 793.7 |
| physics/step/ms | 0.045 | 0.086 | 0.73 | 289.4 |
| physics/insert/ms | 0.036 | 0.376 | 0.99 | 233.5 |
| weather/map/ms | 0.027 | 0.035 | 0.31 | 172.4 |
| anim/apply/ms | 0.012 | 0.022 | 0.24 | 80.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.27 | 71.6 |
| controller/avoid/ms | 0.011 | 0.014 | 0.22 | 69.2 |
| script/update/ms | 0.007 | 0.010 | 0.18 | 44.4 |
| pipelines/driver_ms | 0.007 | 0.000 | 5.23 | 44.4 |
| script/input/ms | 0.005 | 0.006 | 0.10 | 30.9 |
| stream/despawn/ms | 0.005 | 0.000 | 0.99 | 29.7 |
| script/ui/ms | 0.004 | 0.006 | 0.32 | 28.7 |
| stream/spawn/ms | 0.004 | 0.000 | 1.06 | 25.5 |
| controller/write/ms | 0.003 | 0.004 | 0.20 | 22.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.62 | 13.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 6.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 5.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.45 | 4.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 12 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| ecs/entities | count | 59.4 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 8.44 | 29.34 | 8.86 |
| gpu/timed/ms | ms | 7.85 | 10.82 | 8.05 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 823 | 1420 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 823 | 1420 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1290 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3428 | 5140 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1290 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3428 | 5140 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 42 | 228 | 28 |
| phases/shadow/bins | count | 3829 | 5519 | 2230 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9642 | 12.8 k | 5685 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 119 | 256 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2679 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 345 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 167 | 169 | 169 |
| pipelines/waiting | count | 0 | 22 | 0 |
| preprocess/bind_groups | count | 10 | 599 | 9 |
| preprocess/bind_groups_created | count | 10 | 599 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 176 | 279 | 155 |
| process/cpu_usage | % | 27.66 | 42.03 | 34.63 |
| process/mem_usage | GiB | 2.78 | 2.91 | 2.91 |
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
| started_load_count |  | 3490 | 3549 | 3549 |
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
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.21 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.86 | 49.72 | 37.69 |
| system/mem_usage | % | 52.02 | 52.27 | 52.27 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 87 | 268 | 72 |
| views/all/bins | count | 6563 | 8180 | 5710 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.4 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 160 | 296 | 68 |
| views/all/pipelines_distinct | count | 61 | 69 | 55 |
| views/all/work_items | count | 17.6 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 28 |
| views/cascade/bins | count | 2630 | 3182 | 2230 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8178 | 10.2 k | 5685 |
| views/cascade/pipelines | count | 36 | 50 | 28 |
| views/cascade/work_items | count | 8178 | 10.2 k | 5685 |
| views/lamp_face/batch_sets | count | 5 | 189 | 0 |
| views/lamp_face/bins | count | 1117 | 2579 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1265 | 2948 | 0 |
| views/lamp_face/pipelines | count | 78 | 212 | 0 |
| views/lamp_face/work_items | count | 96 | 2528 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2882 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8498 | 13.0 k | 7600 |
| views/main/pipelines | count | 40 | 44 | 40 |
| views/main/work_items | count | 8657 | 13.2 k | 7908 |
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
| anim/bones_written/count | count | 61.06 | 303 | 396.4 k |
| anim/events/count | count | 0.26 | 5 | 1667 |
| assets/Image/added | count | 0.05 | 35 | 337 |
| assets/Image/modified | count | 0.00 | 9 | 17 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.66 | 160 | 4253 |
| assets/Mesh/removed | count | 0.75 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 498 |
| assets/StandardMaterial/modified | count | 3.34 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.94 | 1311 | 6115 |
| physics/inserts | count | 0.40 | 92 | 2587 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 129 | 2335 |
| physics/steps | count | 0.61 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 3 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 21 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 24 | 378 |
| stream/upload/bytes | bytes | 5215 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1143 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 7, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 588.5 | main/busy/ms 566, surface/advance_cpu_ms 66.26, surface/cell_upload_ms 0.70, stream/drive/ms 0.35, weather/map/ms 0.12, stream/tick/ms 0.10, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 0.91 | 325.3 | main/busy/ms 602, render/thread/ms 287, render/busy/ms 211, render/acquire/ms 73.70, surface/advance_cpu_ms 66.26, render/graph/ms 4.62, render/present/ms 2.05, stream/drive/ms 1.14, surface/cell_upload_ms 0.70, stream/tick/ms 0.53, terrain/update/ms 0.40, bigspace/total/ms 0.33 |
| warm-up | 0.98 | 70.4 | pipelines/wait_ms 827, render/thread/ms 297, render/busy/ms 220, render/acquire/ms 73.98, pipelines/compile_ms 70.51, pipelines/front_ms 64.05, main/busy/ms 51.06, terrain/update/ms 21.88, render/graph/ms 8.36, pipelines/driver_ms 6.46, render/present/ms 2.98, stream/drive/ms 1.29 |
| warm-up | 1.22 | 82.8 | pipelines/compile_ms 192, pipelines/front_ms 176, main/busy/ms 61.69, render/thread/ms 55.50, render/busy/ms 53.85, pipelines/driver_ms 16.36, render/graph/ms 3.74, stream/drive/ms 1.83, stream/tick/ms 1.57, render/present/ms 1.41, terrain/update/ms 0.75, terrain/select/ms 0.61 |
| warm-up | 1.38 | 26.5 | pipelines/wait_ms 2196, pipelines/compile_ms 92.54, pipelines/front_ms 85.44, render/thread/ms 33.47, render/busy/ms 32.23, main/busy/ms 14.49, pipelines/driver_ms 7.10, render/graph/ms 3.13, stream/drive/ms 2.78, render/present/ms 1.04, stream/tick/ms 0.95, stream/spawn/ms 0.72 |
| measure | 2.72 | 25.6 | render/thread/ms 29.54, render/busy/ms 28.30, main/busy/ms 19.15, render/graph/ms 17.17, render/present/ms 1.09, bigspace/total/ms 0.92, physics/insert/ms 0.87, terrain/update/ms 0.53, terrain/select/ms 0.48, controller/feed/ms 0.47, stream/drive/ms 0.45, stream/tick/ms 0.40 |
| measure | 19.82 | 30.9 | render/thread/ms 40.38, render/busy/ms 39.37, main/busy/ms 38.99, render/graph/ms 30.71, bigspace/total/ms 0.86, render/present/ms 0.82, stream/drive/ms 0.57, physics/insert/ms 0.53, terrain/update/ms 0.53, controller/feed/ms 0.51, terrain/select/ms 0.47, far/select/ms 0.46 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\push_nocull\balmora_walk@2560x1440_off=cpuculltag-3 --bench-baseline none --off cpuculltag
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (256 samples): GPU load **89%** (min 58%) · 2580 MHz (min 2580) · memory 8001 MHz · 83.4 W · 73 °C (max 78) · VRAM 4225 MiB · throttled 100% of samples
