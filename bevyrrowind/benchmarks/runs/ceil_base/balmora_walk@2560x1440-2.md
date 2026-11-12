# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5801 frames over 65.6 s · **median 10.95 ms (91 fps)** · mean 11.32 · p95 13.79 · p99 15.25 (1% low 66 fps) · worst 29.20 ms · 34 spikes over 1.5× median · 15.5 stutters/min (work 23.8/min)
- warm-up: 460 frames over 5.6 s, worst 587.9 ms
- hitches (> 27.4 ms): **1** measured, 4 in warm-up
- timed GPU passes: 7.48 ms of a 11.32 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.95 ms, limit 11 ms
- **FAIL** frame_ms: 17 stutters (> 1.5x moving median of 60) in 1.09 min = 15.54/min, limit 1/min
- pass worst frame 29.20 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.243 | 2.680 | 0.179 |
| main_opaque_pass_3d | 1.466 | 2.058 | 0.044 |
| early prepass | 0.517 | 0.990 | 0.079 |
| lightprobe_irradiance_map | 0.436 | 0.577 | 0.001 |
| bloom | 0.341 | 0.504 | 0.064 |
| atmosphere_luts | 0.333 | 0.603 | 0.003 |
| lightprobe_radiance_map | 0.225 | 0.233 | 0.004 |
| ssao | 0.218 | 0.212 | 0.007 |
| uniform_allocation | 0.181 | 0.183 | 0.016 |
| auxiliary_views | 0.140 | 0.599 | 0.123 |
| water_fft | 0.132 | 0.129 | 0.003 |
| upscaling | 0.130 | 0.157 | 0.004 |
| clouds_trace | 0.127 | 0.214 | 0.001 |
| bin_unpacking | 0.126 | 0.139 | 0.012 |
| atmos_froxel_inject | 0.098 | 0.105 | 0.002 |
| tonemapping | 0.093 | 0.092 | 0.003 |
| water_velocity | 0.088 | 0.085 | 0.001 |
| clustering | 0.067 | 0.068 | 0.019 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.061 | 0.104 | 0.105 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.045 | 0.048 | 0.005 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| clouds_shadow | 0.032 | 0.034 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.029 | 0.001 |
| water_mips | 0.025 | 0.024 | 0.002 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| per_view_shadow_pass | 0.021 | 0.024 | 0.006 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.013 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.668 | 12.064 | 27.51 | 56082.2 |
| render/busy/ms | 9.133 | 11.510 | 21.72 | 52982.3 |
| main/busy/ms | 8.331 | 10.075 | 16.19 | 48325.7 |
| pipelines/wait_ms | 5.308 | 0.000 | 4449.03 | 30791.8 |
| render/graph/ms | 4.583 | 6.024 | 12.62 | 26586.2 |
| bigspace/total/ms | 0.448 | 0.748 | 2.11 | 2600.0 |
| render/present/ms | 0.442 | 0.535 | 18.09 | 2562.9 |
| terrain/update/ms | 0.292 | 0.371 | 1.72 | 1694.5 |
| stream/drive/ms | 0.264 | 0.451 | 2.12 | 1532.1 |
| terrain/select/ms | 0.257 | 0.326 | 0.87 | 1489.7 |
| controller/feed/ms | 0.248 | 0.324 | 0.80 | 1439.4 |
| far/select/ms | 0.216 | 0.371 | 0.68 | 1255.6 |
| stream/tick/ms | 0.213 | 0.374 | 0.84 | 1235.1 |
| anim/evaluate_cpu/ms | 0.175 | 0.218 | 0.60 | 1015.2 |
| bigspace/low_precision/ms | 0.173 | 0.297 | 1.64 | 1001.1 |
| pipelines/compile_ms | 0.157 | 0.000 | 150.18 | 908.6 |
| pipelines/front_ms | 0.151 | 0.000 | 144.43 | 877.9 |
| controller/move/ms | 0.144 | 0.289 | 0.69 | 836.9 |
| anim/evaluate/ms | 0.118 | 0.197 | 1.39 | 683.4 |
| bigspace/high_precision/ms | 0.104 | 0.207 | 1.54 | 605.2 |
| render/acquire/ms | 0.093 | 0.121 | 0.49 | 537.0 |
| anim/apply/ms | 0.081 | 0.119 | 1.38 | 472.7 |
| physics/step/ms | 0.049 | 0.086 | 0.85 | 286.0 |
| physics/insert/ms | 0.049 | 0.425 | 0.80 | 283.5 |
| weather/map/ms | 0.026 | 0.034 | 0.24 | 153.0 |
| controller/avoid/ms | 0.011 | 0.017 | 0.24 | 66.0 |
| weather/eval/ms | 0.011 | 0.014 | 0.19 | 61.1 |
| script/update/ms | 0.008 | 0.015 | 0.11 | 43.6 |
| stream/despawn/ms | 0.006 | 0.000 | 1.04 | 34.4 |
| pipelines/driver_ms | 0.005 | 0.000 | 5.75 | 30.7 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 27.9 |
| script/input/ms | 0.005 | 0.006 | 0.20 | 27.8 |
| script/ui/ms | 0.005 | 0.006 | 0.12 | 27.1 |
| controller/write/ms | 0.003 | 0.006 | 0.10 | 20.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.61 | 16.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.1 |
| shapes/activate/ms | 0.001 | 0.002 | 0.21 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.7 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 17 |
| controller/movers/count | count | 19 | 22 | 17 |
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
| frame/work/ms | ms | 8.88 | 21.72 | 8.19 |
| gpu/timed/ms | ms | 7.51 | 9.49 | 8.19 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 817 | 1420 | 617 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 817 | 1420 | 617 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1290 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3412 | 5131 | 3179 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1290 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3412 | 5131 | 3179 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 37 | 237 | 28 |
| phases/shadow/bins | count | 3833 | 5436 | 2222 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9656 | 12.8 k | 5673 |
| phases/shadow/pipelines | count | 112 | 259 | 28 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2346 |
| physics/items | count | 204 | 476 | 19 |
| physics/pending | count | 0 | 290 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 172 | 174 | 174 |
| pipelines/waiting | count | 0 | 10 | 0 |
| preprocess/bind_groups | count | 184 | 622 | 164 |
| preprocess/bind_groups_created | count | 184 | 622 | 164 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 24.04 | 38.28 | 16.22 |
| process/mem_usage | GiB | 2.63 | 2.69 | 2.69 |
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
| started_load_count |  | 3469 | 3536 | 3536 |
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
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.24 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.12 | 56.12 | 29.40 |
| system/mem_usage | % | 49.45 | 50.58 | 50.51 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 82 | 277 | 72 |
| views/all/bins | count | 6511 | 8214 | 5702 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.3 k | 25.2 k | 13.3 k |
| views/all/pipelines | count | 154 | 297 | 68 |
| views/all/pipelines_distinct | count | 62 | 68 | 55 |
| views/all/work_items | count | 17.3 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 28 |
| views/cascade/bins | count | 2607 | 3147 | 2222 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8164 | 10.2 k | 5673 |
| views/cascade/pipelines | count | 36 | 50 | 28 |
| views/cascade/work_items | count | 8164 | 10.2 k | 5673 |
| views/lamp_face/batch_sets | count | 0 | 198 | 0 |
| views/lamp_face/bins | count | 1101 | 2510 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1269 | 2860 | 0 |
| views/lamp_face/pipelines | count | 73 | 213 | 0 |
| views/lamp_face/work_items | count | 0 | 2572 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2884 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8442 | 13.0 k | 7592 |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 8602 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 48.83 | 278 | 283.2 k |
| anim/events/count | count | 0.27 | 4 | 1552 |
| assets/Image/added | count | 0.05 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.70 | 160 | 4086 |
| assets/Mesh/removed | count | 0.84 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.73 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.82 | 1309 | 4739 |
| physics/inserts | count | 0.44 | 93 | 2532 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.68 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 7 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 26 | 378 |
| stream/upload/bytes | bytes | 5836 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1279 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.4 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 587.9 | main/busy/ms 567, surface/advance_cpu_ms 66.76, surface/cell_upload_ms 0.73, stream/drive/ms 0.32, weather/map/ms 0.13, bigspace/total/ms 0.09, stream/tick/ms 0.07, bigspace/high_precision/ms 0.05, weather/eval/ms 0.02, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.96 | 371.0 | main/busy/ms 601, render/thread/ms 334, render/busy/ms 258, render/acquire/ms 73.61, surface/advance_cpu_ms 66.76, render/graph/ms 6.79, render/present/ms 1.82, stream/drive/ms 1.07, surface/cell_upload_ms 0.73, stream/tick/ms 0.55, terrain/update/ms 0.52, terrain/select/ms 0.36 |
| warm-up | 1.04 | 86.0 | pipelines/wait_ms 769, render/thread/ms 352, render/busy/ms 275, pipelines/compile_ms 78.17, render/acquire/ms 73.76, pipelines/front_ms 72.32, main/busy/ms 52.87, terrain/update/ms 18.50, render/graph/ms 11.39, pipelines/driver_ms 5.85, render/present/ms 3.37, stream/drive/ms 1.18 |
| warm-up | 1.28 | 101.1 | main/busy/ms 62.82, render/thread/ms 62.14, render/busy/ms 58.11, render/graph/ms 4.40, render/acquire/ms 2.26, render/present/ms 1.77, stream/drive/ms 1.25, stream/tick/ms 1.06, terrain/update/ms 0.87, terrain/select/ms 0.75, bigspace/total/ms 0.34, bigspace/low_precision/ms 0.14 |
| measure | 52.56 | 29.2 | render/thread/ms 37.76, render/busy/ms 19.11, render/present/ms 18.46, main/busy/ms 17.09, render/graph/ms 9.42, controller/feed/ms 0.79, bigspace/total/ms 0.71, terrain/update/ms 0.61, terrain/select/ms 0.54, anim/evaluate_cpu/ms 0.48, stream/drive/ms 0.45, far/select/ms 0.41 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ceil_base\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 8%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **78%** (min 58%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.5 W · 71 °C (max 75) · VRAM 4050 MiB · throttled 100% of samples
