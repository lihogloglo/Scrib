# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5424 frames over 65.6 s · **median 11.32 ms (88 fps)** · mean 12.10 · p95 15.34 · p99 24.41 (1% low 41 fps) · worst 88.30 ms · 147 spikes over 1.5× median · 113.3 stutters/min (work 123.4/min)
- warm-up: 327 frames over 6.0 s, worst 587.6 ms
- hitches (> 28.3 ms): **39** measured, 21 in warm-up
- timed GPU passes: 8.18 ms of a 12.10 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.32 ms, limit 11 ms
- **FAIL** frame_ms: 124 stutters (> 1.5x moving median of 60) in 1.09 min = 113.34/min, limit 1/min
- **FAIL** worst frame 88.30 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.176 | 2.691 | 0.183 |
| main_opaque_pass_3d | 1.470 | 2.122 | 0.048 |
| atmosphere_luts | 0.874 | 1.215 | 0.003 |
| early prepass | 0.551 | 1.099 | 0.081 |
| lightprobe_irradiance_map | 0.463 | 0.782 | 0.001 |
| bloom | 0.335 | 0.479 | 0.068 |
| auxiliary_views | 0.258 | 1.012 | 0.151 |
| lightprobe_radiance_map | 0.228 | 0.239 | 0.004 |
| uniform_allocation | 0.211 | 0.348 | 0.033 |
| ssao | 0.210 | 0.211 | 0.008 |
| bin_unpacking | 0.135 | 0.144 | 0.024 |
| water_fft | 0.129 | 0.128 | 0.003 |
| upscaling | 0.128 | 0.141 | 0.004 |
| clouds_trace | 0.127 | 0.214 | 0.001 |
| atmos_froxel_inject | 0.097 | 0.102 | 0.002 |
| tonemapping | 0.090 | 0.091 | 0.004 |
| water_velocity | 0.085 | 0.086 | 0.002 |
| clustering | 0.068 | 0.071 | 0.020 |
| main_transparent_pass_3d | 0.067 | 0.113 | 0.157 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| surface_trail | 0.062 | 0.059 | 0.002 |
| early_mesh_preprocessing | 0.062 | 0.060 | 0.005 |
| render_sky | 0.055 | 0.055 | 0.002 |
| clouds_resolve | 0.034 | 0.037 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.028 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.025 | 0.007 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.745 | 13.746 | 86.72 | 58282.7 |
| render/busy/ms | 10.121 | 13.153 | 86.12 | 54894.9 |
| main/busy/ms | 8.182 | 9.923 | 33.53 | 44379.3 |
| render/graph/ms | 4.801 | 6.134 | 16.65 | 26041.5 |
| render/present/ms | 0.515 | 1.173 | 2.19 | 2791.3 |
| bigspace/total/ms | 0.490 | 0.878 | 2.95 | 2660.4 |
| terrain/update/ms | 0.299 | 0.379 | 6.56 | 1622.8 |
| controller/feed/ms | 0.279 | 0.379 | 0.84 | 1511.2 |
| terrain/select/ms | 0.266 | 0.335 | 6.52 | 1440.1 |
| stream/drive/ms | 0.233 | 0.338 | 1.97 | 1265.8 |
| far/select/ms | 0.194 | 0.279 | 1.86 | 1053.2 |
| stream/tick/ms | 0.189 | 0.254 | 0.71 | 1026.8 |
| bigspace/low_precision/ms | 0.186 | 0.361 | 2.49 | 1010.0 |
| anim/evaluate_cpu/ms | 0.175 | 0.223 | 0.70 | 951.4 |
| controller/move/ms | 0.144 | 0.288 | 0.56 | 781.0 |
| anim/evaluate/ms | 0.134 | 0.279 | 1.81 | 726.9 |
| bigspace/high_precision/ms | 0.119 | 0.267 | 1.95 | 643.4 |
| render/acquire/ms | 0.110 | 0.143 | 3.37 | 596.5 |
| physics/step/ms | 0.050 | 0.093 | 0.55 | 273.5 |
| pipelines/compile_ms | 0.046 | 0.000 | 132.33 | 248.4 |
| pipelines/front_ms | 0.044 | 0.000 | 127.43 | 240.0 |
| physics/insert/ms | 0.042 | 0.393 | 1.02 | 229.8 |
| weather/map/ms | 0.028 | 0.036 | 0.44 | 149.2 |
| weather/eval/ms | 0.011 | 0.016 | 0.21 | 62.3 |
| anim/apply/ms | 0.011 | 0.021 | 0.19 | 62.0 |
| controller/avoid/ms | 0.011 | 0.014 | 0.09 | 57.6 |
| script/update/ms | 0.007 | 0.010 | 0.24 | 39.7 |
| script/ui/ms | 0.005 | 0.007 | 0.20 | 28.4 |
| stream/despawn/ms | 0.005 | 0.000 | 1.20 | 27.4 |
| script/input/ms | 0.005 | 0.006 | 0.14 | 26.5 |
| stream/spawn/ms | 0.004 | 0.000 | 1.13 | 23.8 |
| controller/write/ms | 0.004 | 0.005 | 0.03 | 19.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 12.7 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.91 | 8.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.25 | 4.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
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
| frame/work/ms | ms | 9.40 | 86.12 | 8.71 |
| gpu/timed/ms | ms | 8.18 | 17.73 | 8.63 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 258 | 486 | 478 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3950 | 4350 | 4314 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 258 | 486 | 478 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3950 | 4350 | 4314 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 4106 | 4334 | 4333 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 4106 | 4334 | 4333 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 38 | 352 | 30 |
| phases/shadow/bins | count | 63.2 k | 174.3 k | 12.0 k |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 206.7 k | 559.9 k | 39.4 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 146 | 372 | 30 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2346 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 242 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 173 | 174 | 174 |
| pipelines/waiting | count | 0 | 1 | 0 |
| preprocess/bind_groups | count | 11 | 847 | 6 |
| preprocess/bind_groups_created | count | 11 | 847 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 170 | 285 | 162 |
| process/cpu_usage | % | 23.31 | 37.80 | 17.79 |
| process/mem_usage | GiB | 2.78 | 2.96 | 2.96 |
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
| started_load_count |  | 3466 | 3526 | 3526 |
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
| stream/deactivate/queue | count | 0 | 2 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.10 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.70 | 61.65 | 31.34 |
| system/mem_usage | % | 52.02 | 53.51 | 53.30 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 83 | 392 | 74 |
| views/all/bins | count | 71.3 k | 182.5 k | 21.6 k |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 233.4 k | 588.1 k | 69.0 k |
| views/all/pipelines | count | 188 | 411 | 70 |
| views/all/pipelines_distinct | count | 62 | 70 | 55 |
| views/all/work_items | count | 69.2 k | 575.5 k | 69.7 k |
| views/cascade/batch_sets | count | 37 | 50 | 30 |
| views/cascade/bins | count | 11.9 k | 12.5 k | 12.0 k |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 38.4 k | 39.9 k | 39.4 k |
| views/cascade/pipelines | count | 37 | 50 | 30 |
| views/cascade/work_items | count | 38.4 k | 39.9 k | 39.4 k |
| views/lamp_face/batch_sets | count | 0 | 311 | 0 |
| views/lamp_face/bins | count | 53.0 k | 162.3 k | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 173.1 k | 521.8 k | 0 |
| views/lamp_face/pipelines | count | 110 | 326 | 0 |
| views/lamp_face/work_items | count | 0 | 510.2 k | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 8734 | 9622 | 9622 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.0 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 28.5 k | 30.5 k | 30.3 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.6 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 48.01 | 275 | 260.4 k |
| anim/events/count | count | 0.31 | 5 | 1676 |
| assets/Image/added | count | 0.06 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.75 | 176 | 4086 |
| assets/Mesh/removed | count | 0.90 | 1242 | 4892 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.91 | 11 | 21.2 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 6 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.87 | 1494 | 4738 |
| physics/inserts | count | 0.46 | 111 | 2484 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.43 | 161 | 2335 |
| physics/steps | count | 0.72 | 3 | 3890 |
| physics/steps_dropped | count | 0.01 | 2 | 48 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 5 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 6241 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1368 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.3 ms; slowest 60, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 587.6 | main/busy/ms 564, surface/advance_cpu_ms 66.71, surface/cell_upload_ms 0.72, stream/drive/ms 0.30, weather/map/ms 0.12, bigspace/total/ms 0.09, stream/tick/ms 0.07, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.03, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 1.01 | 419.0 | main/busy/ms 600, render/thread/ms 385, render/busy/ms 298, render/acquire/ms 83.78, surface/advance_cpu_ms 66.71, render/graph/ms 4.60, render/present/ms 2.72, stream/drive/ms 1.23, surface/cell_upload_ms 0.72, stream/tick/ms 0.66, terrain/update/ms 0.44, terrain/select/ms 0.28 |
| warm-up | 1.08 | 78.1 | pipelines/wait_ms 901, render/thread/ms 399, render/busy/ms 311, render/acquire/ms 84.07, pipelines/compile_ms 76.53, pipelines/front_ms 70.10, main/busy/ms 52.06, terrain/update/ms 19.62, render/graph/ms 8.20, pipelines/driver_ms 6.42, render/present/ms 3.86, stream/drive/ms 1.61 |
| warm-up | 1.36 | 113.5 | pipelines/compile_ms 203, pipelines/front_ms 185, render/thread/ms 102, render/busy/ms 91.25, main/busy/ms 62.43, pipelines/driver_ms 18.07, render/acquire/ms 9.00, render/graph/ms 3.91, stream/drive/ms 3.28, stream/tick/ms 2.40, render/present/ms 1.52, terrain/update/ms 0.80 |
| warm-up | 1.76 | 35.3 | pipelines/wait_ms 1807, render/thread/ms 49.95, render/busy/ms 48.73, main/busy/ms 17.47, pipelines/compile_ms 16.17, pipelines/front_ms 15.03, render/graph/ms 4.84, stream/drive/ms 4.53, stream/spawn/ms 1.85, pipelines/driver_ms 1.14, stream/activate/ms 1.01, render/present/ms 1.00 |
| warm-up | 1.79 | 29.9 | render/thread/ms 62.85, render/busy/ms 61.68, main/busy/ms 25.45, stream/drive/ms 5.07, render/graph/ms 4.56, stream/spawn/ms 1.86, stream/activate/ms 1.50, render/present/ms 0.90, stream/tick/ms 0.81, terrain/update/ms 0.62, terrain/select/ms 0.51, physics/insert/ms 0.33 |
| warm-up | 1.84 | 36.1 | pipelines/wait_ms 2923, pipelines/compile_ms 137, pipelines/front_ms 129, render/thread/ms 52.03, render/busy/ms 50.89, main/busy/ms 16.59, pipelines/driver_ms 8.52, terrain/update/ms 8.33, render/graph/ms 5.10, stream/drive/ms 2.70, render/present/ms 0.94, stream/tick/ms 0.84 |
| warm-up | 2.32 | 28.4 | render/thread/ms 36.29, render/busy/ms 35.13, main/busy/ms 13.88, render/graph/ms 5.23, stream/drive/ms 3.19, stream/spawn/ms 1.45, render/present/ms 0.93, stream/tick/ms 0.71, terrain/update/ms 0.54, bigspace/total/ms 0.48, terrain/select/ms 0.47, render/acquire/ms 0.23 |
| warm-up | 2.36 | 43.6 | pipelines/wait_ms 3355, pipelines/compile_ms 135, pipelines/front_ms 130, render/thread/ms 69.34, render/busy/ms 68.16, main/busy/ms 14.58, render/graph/ms 5.14, pipelines/driver_ms 4.23, stream/drive/ms 3.91, stream/spawn/ms 1.77, render/present/ms 0.92, stream/tick/ms 0.85 |
| warm-up | 2.40 | 32.5 | pipelines/wait_ms 4221, pipelines/compile_ms 149, pipelines/front_ms 144, render/thread/ms 73.39, render/busy/ms 72.16, main/busy/ms 15.04, render/graph/ms 4.97, pipelines/driver_ms 4.91, stream/drive/ms 3.63, stream/spawn/ms 2.07, render/present/ms 0.92, stream/tick/ms 0.86 |
| warm-up | 2.46 | 41.0 | render/thread/ms 62.10, render/busy/ms 60.95, main/busy/ms 12.92, render/graph/ms 5.23, stream/drive/ms 2.18, stream/spawn/ms 1.00, render/present/ms 0.89, stream/tick/ms 0.72, terrain/update/ms 0.57, terrain/select/ms 0.49, bigspace/total/ms 0.42, render/acquire/ms 0.26 |
| warm-up | 2.63 | 40.0 | render/thread/ms 56.52, render/busy/ms 55.24, main/busy/ms 18.32, render/graph/ms 5.35, stream/drive/ms 3.15, stream/spawn/ms 1.97, render/present/ms 0.99, physics/step/ms 0.87, stream/tick/ms 0.69, terrain/update/ms 0.55, bigspace/total/ms 0.52, terrain/select/ms 0.47 |
| warm-up | 2.67 | 35.5 | pipelines/wait_ms 4116, pipelines/compile_ms 131, pipelines/front_ms 125, render/thread/ms 72.19, render/busy/ms 70.85, main/busy/ms 18.45, pipelines/driver_ms 5.88, render/graph/ms 5.86, stream/drive/ms 3.42, stream/spawn/ms 2.19, render/present/ms 1.09, stream/tick/ms 0.71 |
| warm-up | 2.71 | 42.5 | pipelines/wait_ms 4116, pipelines/compile_ms 131, pipelines/front_ms 125, render/thread/ms 74.92, render/busy/ms 73.74, main/busy/ms 18.73, pipelines/driver_ms 5.88, render/graph/ms 5.42, stream/drive/ms 3.61, stream/spawn/ms 2.06, render/present/ms 0.89, stream/tick/ms 0.72 |
| warm-up | 2.89 | 35.5 | render/thread/ms 52.94, render/busy/ms 51.78, main/busy/ms 18.20, render/graph/ms 4.99, stream/drive/ms 3.21, stream/spawn/ms 1.78, render/present/ms 0.90, bigspace/total/ms 0.73, stream/tick/ms 0.70, terrain/update/ms 0.63, terrain/select/ms 0.55, bigspace/high_precision/ms 0.30 |
| warm-up | 2.93 | 41.5 | pipelines/wait_ms 13.1 k, pipelines/compile_ms 155, pipelines/front_ms 148, render/thread/ms 73.57, render/busy/ms 72.35, main/busy/ms 20.24, pipelines/driver_ms 6.98, render/graph/ms 5.95, stream/drive/ms 3.55, stream/spawn/ms 2.13, render/present/ms 0.94, terrain/update/ms 0.81 |
| warm-up | 2.99 | 65.3 | pipelines/wait_ms 13.1 k, pipelines/compile_ms 155, pipelines/front_ms 148, render/thread/ms 102, render/busy/ms 101, main/busy/ms 22.95, pipelines/driver_ms 6.98, render/graph/ms 6.72, stream/drive/ms 3.91, stream/spawn/ms 2.12, render/present/ms 1.02, terrain/update/ms 0.76 |
| warm-up | 3.03 | 41.1 | render/thread/ms 102, render/busy/ms 100, main/busy/ms 47.84, render/graph/ms 6.56, stream/drive/ms 4.13, stream/spawn/ms 2.10, render/present/ms 1.06, stream/tick/ms 0.76, bigspace/total/ms 0.70, physics/insert/ms 0.66, terrain/update/ms 0.63, terrain/select/ms 0.53 |
| warm-up | 3.10 | 69.7 | pipelines/wait_ms 5316, pipelines/compile_ms 155, pipelines/front_ms 145, render/thread/ms 106, render/busy/ms 104, main/busy/ms 53.52, pipelines/driver_ms 9.67, render/graph/ms 7.07, stream/drive/ms 4.15, stream/spawn/ms 2.08, render/present/ms 1.17, physics/insert/ms 0.99 |
| warm-up | 3.18 | 36.1 | render/thread/ms 49.91, render/busy/ms 48.64, main/busy/ms 18.77, render/graph/ms 6.75, stream/drive/ms 2.24, render/present/ms 0.98, stream/spawn/ms 0.96, physics/step/ms 0.89, stream/tick/ms 0.80, terrain/update/ms 0.64, bigspace/total/ms 0.62, terrain/select/ms 0.55 |
| warm-up | 3.30 | 29.6 | pipelines/wait_ms 2107, pipelines/compile_ms 67.96, pipelines/front_ms 63.88, render/thread/ms 50.81, render/busy/ms 49.54, main/busy/ms 31.79, render/graph/ms 7.23, pipelines/driver_ms 4.08, stream/drive/ms 2.39, render/present/ms 1.04, stream/spawn/ms 0.98, stream/tick/ms 0.69 |
| measure | 0.43 | 73.7 | render/thread/ms 80.55, render/busy/ms 78.87, render/graph/ms 16.82, main/busy/ms 14.32, render/present/ms 1.46, bigspace/total/ms 1.44, bigspace/low_precision/ms 1.02, physics/insert/ms 0.69, terrain/update/ms 0.55, terrain/select/ms 0.50, stream/drive/ms 0.39, anim/evaluate_cpu/ms 0.36 |
| measure | 0.83 | 52.1 | render/thread/ms 64.99, render/busy/ms 63.62, main/busy/ms 22.83, render/graph/ms 9.69, render/present/ms 1.16, physics/insert/ms 0.97, stream/drive/ms 0.74, bigspace/total/ms 0.73, terrain/update/ms 0.57, terrain/select/ms 0.51, controller/feed/ms 0.41, stream/tick/ms 0.38 |
| measure | 2.23 | 77.9 | render/thread/ms 87.18, render/busy/ms 86.05, render/graph/ms 15.83, main/busy/ms 13.94, pipelines/compile_ms 11.33, pipelines/front_ms 10.99, bigspace/total/ms 0.97, render/present/ms 0.96, terrain/update/ms 0.54, terrain/select/ms 0.48, stream/drive/ms 0.40, controller/feed/ms 0.37 |
| measure | 2.77 | 82.8 | render/thread/ms 93.85, render/busy/ms 92.70, render/graph/ms 18.12, main/busy/ms 16.38, bigspace/total/ms 1.04, render/present/ms 0.93, physics/insert/ms 0.79, terrain/update/ms 0.55, terrain/select/ms 0.48, stream/drive/ms 0.43, bigspace/low_precision/ms 0.42, controller/feed/ms 0.38 |
| measure | 2.82 | 33.9 | render/thread/ms 43.73, render/busy/ms 42.00, main/busy/ms 19.08, render/graph/ms 15.45, render/present/ms 1.53, bigspace/total/ms 0.85, terrain/update/ms 0.69, terrain/select/ms 0.61, physics/insert/ms 0.48, stream/drive/ms 0.43, stream/tick/ms 0.38, controller/feed/ms 0.36 |
| measure | 3.17 | 28.4 | render/thread/ms 40.63, render/busy/ms 39.54, main/busy/ms 19.26, render/graph/ms 12.56, stream/drive/ms 1.06, bigspace/total/ms 0.92, render/present/ms 0.88, terrain/update/ms 0.65, stream/spawn/ms 0.58, terrain/select/ms 0.58, bigspace/low_precision/ms 0.49, controller/feed/ms 0.44 |
| measure | 6.06 | 65.1 | render/thread/ms 72.37, render/busy/ms 71.30, main/busy/ms 14.19, render/graph/ms 9.15, bigspace/total/ms 1.00, render/present/ms 0.83, terrain/update/ms 0.54, terrain/select/ms 0.49, controller/feed/ms 0.47, stream/drive/ms 0.44, bigspace/high_precision/ms 0.41, stream/tick/ms 0.38 |
| measure | 8.17 | 63.5 | render/thread/ms 70.65, render/busy/ms 69.67, main/busy/ms 13.90, render/graph/ms 8.93, bigspace/total/ms 1.08, render/present/ms 0.79, terrain/update/ms 0.66, terrain/select/ms 0.59, bigspace/low_precision/ms 0.55, controller/feed/ms 0.44, stream/drive/ms 0.42, stream/tick/ms 0.37 |
| measure | 9.35 | 79.9 | render/thread/ms 89.97, render/busy/ms 88.86, main/busy/ms 16.59, render/graph/ms 11.54, render/present/ms 0.89, terrain/update/ms 0.88, bigspace/total/ms 0.85, terrain/select/ms 0.78, far/select/ms 0.51, controller/feed/ms 0.49, stream/drive/ms 0.44, stream/tick/ms 0.39 |
| measure | 9.43 | 70.9 | render/thread/ms 81.99, render/busy/ms 80.89, main/busy/ms 17.37, render/graph/ms 11.12, bigspace/total/ms 1.16, render/present/ms 0.90, controller/feed/ms 0.51, terrain/update/ms 0.51, stream/drive/ms 0.47, terrain/select/ms 0.46, bigspace/low_precision/ms 0.44, far/select/ms 0.42 |
| measure | 10.81 | 29.0 | render/thread/ms 49.60, render/busy/ms 48.17, main/busy/ms 21.77, render/graph/ms 8.32, bigspace/total/ms 1.20, render/present/ms 1.12, controller/feed/ms 0.96, stream/drive/ms 0.88, stream/tick/ms 0.80, bigspace/low_precision/ms 0.62, far/select/ms 0.60, terrain/update/ms 0.57 |
| measure | 11.56 | 66.9 | render/thread/ms 73.86, render/busy/ms 72.71, main/busy/ms 14.11, render/graph/ms 8.11, render/present/ms 0.94, bigspace/total/ms 0.80, terrain/update/ms 0.55, terrain/select/ms 0.49, controller/feed/ms 0.46, stream/drive/ms 0.41, bigspace/low_precision/ms 0.37, stream/tick/ms 0.37 |
| measure | 12.02 | 67.7 | render/thread/ms 76.05, render/busy/ms 74.89, main/busy/ms 14.63, render/graph/ms 9.45, render/present/ms 0.92, bigspace/total/ms 0.84, terrain/update/ms 0.53, controller/feed/ms 0.48, terrain/select/ms 0.47, stream/drive/ms 0.41, anim/evaluate_cpu/ms 0.41, stream/tick/ms 0.36 |
| measure | 16.19 | 68.7 | render/thread/ms 76.38, render/busy/ms 75.30, main/busy/ms 14.42, render/graph/ms 9.53, render/present/ms 0.87, bigspace/total/ms 0.86, physics/insert/ms 0.55, terrain/update/ms 0.53, controller/feed/ms 0.52, terrain/select/ms 0.47, far/select/ms 0.40, stream/drive/ms 0.39 |
| measure | 16.42 | 73.1 | render/thread/ms 80.97, render/busy/ms 79.76, main/busy/ms 15.32, render/graph/ms 9.61, render/present/ms 0.99, bigspace/total/ms 0.77, controller/feed/ms 0.76, terrain/update/ms 0.54, terrain/select/ms 0.48, anim/evaluate_cpu/ms 0.42, stream/drive/ms 0.40, far/select/ms 0.36 |
| measure | 16.99 | 68.1 | render/thread/ms 76.32, render/busy/ms 75.10, main/busy/ms 14.59, render/graph/ms 9.80, render/present/ms 1.00, bigspace/total/ms 0.73, controller/feed/ms 0.56, terrain/update/ms 0.55, terrain/select/ms 0.49, stream/drive/ms 0.43, stream/tick/ms 0.37, anim/evaluate_cpu/ms 0.37 |
| measure | 18.13 | 69.2 | render/thread/ms 78.29, render/busy/ms 77.24, main/busy/ms 15.47, render/graph/ms 9.93, render/present/ms 0.86, bigspace/total/ms 0.71, terrain/update/ms 0.56, terrain/select/ms 0.50, controller/feed/ms 0.49, stream/drive/ms 0.40, anim/evaluate_cpu/ms 0.36, stream/tick/ms 0.34 |
| measure | 18.82 | 81.2 | render/thread/ms 93.39, render/busy/ms 92.28, main/busy/ms 14.95, render/graph/ms 13.91, render/present/ms 0.90, bigspace/total/ms 0.75, controller/feed/ms 0.71, terrain/update/ms 0.55, terrain/select/ms 0.49, stream/drive/ms 0.42, stream/tick/ms 0.37, anim/evaluate_cpu/ms 0.36 |
| measure | 19.28 | 72.4 | render/thread/ms 81.04, render/busy/ms 79.99, main/busy/ms 16.57, render/graph/ms 10.70, render/present/ms 0.87, bigspace/total/ms 0.73, physics/insert/ms 0.73, terrain/update/ms 0.61, terrain/select/ms 0.53, controller/feed/ms 0.51, stream/drive/ms 0.44, stream/tick/ms 0.38 |
| measure | 19.38 | 88.3 | render/thread/ms 98.72, render/busy/ms 97.64, render/graph/ms 18.93, main/busy/ms 18.89, render/present/ms 0.85, bigspace/total/ms 0.83, physics/insert/ms 0.78, terrain/update/ms 0.59, terrain/select/ms 0.53, controller/feed/ms 0.51, stream/drive/ms 0.44, far/select/ms 0.40 |
| measure | 20.17 | 75.6 | render/thread/ms 84.20, render/busy/ms 83.09, main/busy/ms 17.15, render/graph/ms 9.99, render/present/ms 0.89, physics/insert/ms 0.86, bigspace/total/ms 0.73, controller/feed/ms 0.57, terrain/update/ms 0.57, terrain/select/ms 0.51, stream/drive/ms 0.40, stream/tick/ms 0.35 |
| measure | 21.01 | 71.9 | render/thread/ms 82.33, render/busy/ms 81.15, main/busy/ms 17.64, render/graph/ms 11.61, bigspace/total/ms 2.14, bigspace/high_precision/ms 1.52, render/present/ms 0.97, terrain/update/ms 0.63, terrain/select/ms 0.57, anim/evaluate_cpu/ms 0.52, controller/feed/ms 0.50, stream/drive/ms 0.44 |
| measure | 22.29 | 71.0 | render/thread/ms 80.49, render/busy/ms 79.49, main/busy/ms 15.77, render/graph/ms 9.54, render/present/ms 0.80, bigspace/total/ms 0.73, controller/feed/ms 0.59, terrain/update/ms 0.57, terrain/select/ms 0.51, stream/drive/ms 0.44, stream/tick/ms 0.38, anim/evaluate_cpu/ms 0.34 |
| measure | 22.48 | 74.1 | render/thread/ms 83.98, render/busy/ms 82.79, main/busy/ms 17.77, render/graph/ms 10.01, bigspace/total/ms 1.06, render/present/ms 0.96, controller/feed/ms 0.71, terrain/update/ms 0.60, terrain/select/ms 0.54, far/select/ms 0.47, anim/evaluate_cpu/ms 0.46, bigspace/high_precision/ms 0.39 |
| measure | 22.78 | 70.9 | render/thread/ms 80.09, render/busy/ms 78.98, main/busy/ms 15.15, render/graph/ms 9.56, render/present/ms 0.91, bigspace/total/ms 0.74, terrain/update/ms 0.57, controller/feed/ms 0.56, terrain/select/ms 0.51, stream/drive/ms 0.44, anim/evaluate_cpu/ms 0.37, stream/tick/ms 0.35 |
| measure | 23.23 | 76.0 | render/thread/ms 85.65, render/busy/ms 84.57, main/busy/ms 17.02, render/graph/ms 10.50, bigspace/total/ms 0.99, physics/insert/ms 0.92, render/present/ms 0.88, controller/feed/ms 0.59, stream/drive/ms 0.57, terrain/update/ms 0.53, terrain/select/ms 0.48, anim/evaluate_cpu/ms 0.41 |
| measure | 25.76 | 72.3 | render/thread/ms 79.95, render/busy/ms 78.81, main/busy/ms 15.21, render/graph/ms 9.20, bigspace/total/ms 0.97, render/present/ms 0.92, terrain/update/ms 0.66, terrain/select/ms 0.60, controller/feed/ms 0.56, stream/drive/ms 0.47, stream/tick/ms 0.42, far/select/ms 0.40 |
| measure | 26.14 | 72.0 | render/thread/ms 81.34, render/busy/ms 80.27, main/busy/ms 15.18, render/graph/ms 10.67, bigspace/total/ms 0.89, render/present/ms 0.85, terrain/update/ms 0.55, controller/feed/ms 0.52, terrain/select/ms 0.49, stream/drive/ms 0.41, stream/tick/ms 0.36, anim/evaluate_cpu/ms 0.34 |
| measure | 26.22 | 72.8 | render/thread/ms 81.98, render/busy/ms 80.93, main/busy/ms 15.65, render/graph/ms 10.74, bigspace/total/ms 1.04, render/present/ms 0.80, controller/feed/ms 0.59, terrain/update/ms 0.54, terrain/select/ms 0.49, far/select/ms 0.43, stream/drive/ms 0.41, stream/tick/ms 0.36 |
| measure | 26.33 | 77.0 | render/thread/ms 88.98, render/busy/ms 87.87, main/busy/ms 16.22, render/graph/ms 11.66, bigspace/total/ms 1.09, render/present/ms 0.89, controller/feed/ms 0.63, terrain/update/ms 0.62, terrain/select/ms 0.56, bigspace/low_precision/ms 0.51, stream/drive/ms 0.40, anim/evaluate_cpu/ms 0.38 |
| measure | 26.40 | 74.7 | render/thread/ms 149, render/busy/ms 148, main/busy/ms 16.09, render/graph/ms 12.77, render/present/ms 0.85, bigspace/total/ms 0.81, terrain/update/ms 0.57, controller/feed/ms 0.56, terrain/select/ms 0.50, stream/drive/ms 0.46, stream/tick/ms 0.41, anim/evaluate_cpu/ms 0.39 |
| measure | 26.50 | 79.7 | render/thread/ms 94.04, render/busy/ms 92.90, main/busy/ms 16.94, render/graph/ms 14.23, bigspace/total/ms 0.94, render/present/ms 0.92, controller/feed/ms 0.61, terrain/update/ms 0.59, terrain/select/ms 0.53, stream/drive/ms 0.39, anim/evaluate_cpu/ms 0.37, stream/tick/ms 0.35 |
| measure | 26.81 | 30.7 | render/thread/ms 41.96, render/busy/ms 40.75, main/busy/ms 17.56, render/graph/ms 10.87, bigspace/total/ms 1.06, render/present/ms 0.99, terrain/update/ms 0.55, controller/feed/ms 0.50, terrain/select/ms 0.49, bigspace/high_precision/ms 0.43, anim/evaluate_cpu/ms 0.41, stream/drive/ms 0.40 |
| measure | 29.54 | 74.8 | render/thread/ms 84.33, render/busy/ms 83.36, main/busy/ms 16.57, render/graph/ms 11.58, bigspace/total/ms 0.98, render/present/ms 0.76, terrain/update/ms 0.57, controller/feed/ms 0.54, terrain/select/ms 0.51, stream/drive/ms 0.41, stream/tick/ms 0.36, far/select/ms 0.32 |
| measure | 30.01 | 73.0 | render/thread/ms 81.74, render/busy/ms 80.73, main/busy/ms 16.10, render/graph/ms 10.30, bigspace/total/ms 1.05, render/present/ms 0.82, terrain/update/ms 0.55, anim/evaluate/ms 0.53, controller/feed/ms 0.53, terrain/select/ms 0.50, stream/drive/ms 0.42, stream/tick/ms 0.37 |
| measure | 33.01 | 73.8 | render/thread/ms 82.53, render/busy/ms 81.28, main/busy/ms 14.83, render/graph/ms 10.54, render/present/ms 1.03, bigspace/total/ms 0.97, terrain/update/ms 0.57, terrain/select/ms 0.51, controller/feed/ms 0.49, stream/drive/ms 0.42, anim/evaluate_cpu/ms 0.41, far/select/ms 0.40 |
| measure | 33.15 | 72.0 | render/thread/ms 80.82, render/busy/ms 79.71, main/busy/ms 16.12, render/graph/ms 9.69, bigspace/total/ms 1.21, render/present/ms 0.91, bigspace/low_precision/ms 0.64, controller/feed/ms 0.59, terrain/update/ms 0.59, terrain/select/ms 0.53, stream/drive/ms 0.40, anim/evaluate_cpu/ms 0.39 |
| measure | 34.35 | 74.2 | render/thread/ms 83.89, render/busy/ms 82.86, main/busy/ms 16.01, render/graph/ms 10.79, bigspace/total/ms 0.95, render/present/ms 0.81, terrain/update/ms 0.55, controller/feed/ms 0.54, bigspace/low_precision/ms 0.49, terrain/select/ms 0.49, stream/drive/ms 0.40, anim/evaluate_cpu/ms 0.39 |
| measure | 51.88 | 38.5 | render/thread/ms 51.94, render/busy/ms 50.03, main/busy/ms 44.06, render/graph/ms 18.53, far/select/ms 2.06, bigspace/total/ms 1.97, render/present/ms 1.68, terrain/update/ms 0.91, terrain/select/ms 0.83, controller/feed/ms 0.80, stream/drive/ms 0.71, stream/tick/ms 0.63 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\push_all\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 19%
- during the run: other processes 10% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **79%** (min 20%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.9 W · 72 °C (max 78) · VRAM 4503 MiB · throttled 100% of samples
