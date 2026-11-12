# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5786 frames over 65.6 s · **median 11.06 ms (90 fps)** · mean 11.34 · p95 13.59 · p99 14.92 (1% low 67 fps) · worst 28.90 ms · 18 spikes over 1.5× median · 13.7 stutters/min (work 17.4/min)
- warm-up: 364 frames over 4.7 s, worst 577.2 ms
- hitches (> 27.6 ms): **1** measured, 5 in warm-up
- timed GPU passes: 7.72 ms of a 11.34 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.06 ms, limit 11 ms
- **FAIL** frame_ms: 15 stutters (> 1.5x moving median of 60) in 1.09 min = 13.71/min, limit 1/min
- pass worst frame 28.90 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.226 | 2.599 | 0.179 |
| main_opaque_pass_3d | 1.492 | 2.103 | 0.047 |
| early prepass | 0.561 | 0.847 | 0.079 |
| lightprobe_irradiance_map | 0.433 | 0.574 | 0.001 |
| bloom | 0.336 | 0.486 | 0.064 |
| atmosphere_luts | 0.330 | 0.636 | 0.003 |
| lightprobe_radiance_map | 0.229 | 0.234 | 0.004 |
| ssao | 0.216 | 0.210 | 0.007 |
| auxiliary_views | 0.175 | 0.634 | 0.150 |
| uniform_allocation | 0.157 | 0.162 | 0.015 |
| water_fft | 0.132 | 0.128 | 0.002 |
| bin_unpacking | 0.129 | 0.130 | 0.012 |
| clouds_trace | 0.127 | 0.214 | 0.001 |
| upscaling | 0.127 | 0.134 | 0.004 |
| atmos_froxel_inject | 0.099 | 0.106 | 0.001 |
| gpu_instance_materialization | 0.093 | 0.096 | 0.004 |
| tonemapping | 0.092 | 0.091 | 0.003 |
| early_mesh_preprocessing | 0.087 | 0.099 | 0.003 |
| water_velocity | 0.087 | 0.085 | 0.001 |
| clustering | 0.073 | 0.070 | 0.018 |
| clouds_composite | 0.065 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.060 | 0.105 | 0.082 |
| surface_trail | 0.059 | 0.058 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| per_view_shadow_pass | 0.038 | 0.039 | 0.008 |
| clouds_resolve | 0.035 | 0.041 | 0.001 |
| clouds_shadow | 0.032 | 0.034 | 0.003 |
| range_unpacking | 0.029 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.034 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
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
| pipelines/wait_ms | 12.668 | 0.000 | 7674.49 | 73297.7 |
| render/thread/ms | 9.809 | 11.960 | 29.72 | 56752.7 |
| render/busy/ms | 9.273 | 11.418 | 24.84 | 53653.1 |
| main/busy/ms | 8.162 | 9.740 | 13.93 | 47226.8 |
| render/graph/ms | 4.583 | 5.808 | 13.32 | 26520.1 |
| render/present/ms | 0.447 | 0.559 | 16.72 | 2588.6 |
| bigspace/total/ms | 0.401 | 0.603 | 2.55 | 2322.7 |
| terrain/update/ms | 0.289 | 0.367 | 1.48 | 1674.8 |
| stream/drive/ms | 0.268 | 0.435 | 1.76 | 1550.8 |
| controller/feed/ms | 0.263 | 0.353 | 0.78 | 1522.2 |
| terrain/select/ms | 0.254 | 0.322 | 0.59 | 1472.2 |
| pipelines/compile_ms | 0.236 | 0.000 | 171.19 | 1364.0 |
| pipelines/front_ms | 0.228 | 0.000 | 163.02 | 1318.2 |
| far/select/ms | 0.218 | 0.362 | 0.78 | 1260.3 |
| stream/tick/ms | 0.216 | 0.365 | 0.74 | 1249.8 |
| anim/evaluate_cpu/ms | 0.172 | 0.227 | 0.57 | 992.5 |
| bigspace/low_precision/ms | 0.154 | 0.237 | 2.21 | 888.5 |
| controller/move/ms | 0.146 | 0.272 | 0.70 | 843.2 |
| anim/evaluate/ms | 0.145 | 0.308 | 1.44 | 840.3 |
| bigspace/high_precision/ms | 0.099 | 0.170 | 1.46 | 572.7 |
| render/acquire/ms | 0.088 | 0.115 | 0.37 | 511.0 |
| anim/apply/ms | 0.087 | 0.159 | 1.50 | 503.0 |
| physics/insert/ms | 0.050 | 0.431 | 1.02 | 288.1 |
| physics/step/ms | 0.048 | 0.083 | 1.05 | 279.9 |
| weather/map/ms | 0.027 | 0.035 | 0.24 | 157.6 |
| controller/avoid/ms | 0.012 | 0.017 | 0.22 | 66.7 |
| weather/eval/ms | 0.011 | 0.015 | 0.18 | 61.7 |
| pipelines/driver_ms | 0.008 | 0.000 | 8.17 | 45.8 |
| script/update/ms | 0.007 | 0.013 | 0.30 | 41.8 |
| stream/despawn/ms | 0.006 | 0.000 | 1.29 | 32.7 |
| script/ui/ms | 0.005 | 0.009 | 0.12 | 29.9 |
| script/input/ms | 0.005 | 0.007 | 0.14 | 28.4 |
| stream/upload/ms | 0.004 | 0.000 | 1.41 | 21.7 |
| controller/write/ms | 0.004 | 0.006 | 0.22 | 21.1 |
| stream/spawn/ms | 0.003 | 0.000 | 1.06 | 18.7 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.72 | 15.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.05 | 6.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.5 |
| shapes/activate/ms | 0.001 | 0.002 | 0.02 | 5.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 11 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 33 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| ecs/entities | count | 48.6 k | 56.7 k | 46.9 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 9.04 | 24.84 | 8.19 |
| gpu/timed/ms | ms | 7.74 | 9.72 | 7.99 |
| mesh_allocator_allocations |  meshes | 10.6 k | 11.8 k | 10.4 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 362.91 M | 362.91 M | 362.91 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 157 | 388 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 336 | 588 | 516 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 157 | 388 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 336 | 588 | 516 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/opaque3d/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d/bins | count | 758 | 973 | 757 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2516 | 3571 | 2452 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 758 | 973 | 757 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2516 | 3571 | 2452 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 50 | 210 | 40 |
| phases/shadow/bins | count | 1647 | 2425 | 951 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 19.3 k | 20.3 k | 20.3 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 5126 | 7015 | 3848 |
| phases/shadow/pipelines | count | 104 | 216 | 30 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 186 | 476 | 19 |
| physics/pending | count | 0 | 1558 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 170 | 171 | 171 |
| pipelines/waiting | count | 0 | 19 | 0 |
| preprocess/bind_groups | count | 189 | 547 | 167 |
| preprocess/bind_groups_created | count | 182 | 540 | 160 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 23.08 | 40.10 | 29.33 |
| process/mem_usage | GiB | 2.60 | 2.66 | 2.66 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2979 | 3169 | 3169 |
| scene/batch/masked/live | count | 2979 | 3169 | 3169 |
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
| scene/batch/opaque/capacity | count | 41.0 k | 41.0 k | 41.0 k |
| scene/batch/opaque/count | count | 2 | 2 | 2 |
| scene/batch/opaque/high_water | count | 18.8 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 18.8 k | 19.9 k | 19.9 k |
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
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1594 | 1624 | 1624 |
| scene/chains/live | count | 1594 | 1624 | 1624 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/scene_bump | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/scene_live | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 5023 | 5115 | 5115 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 237 | 333 | 180 |
| statics/materials_bound | count | 237 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.05 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.24 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.74 | 48.69 | 29.35 |
| system/mem_usage | % | 49.57 | 50.15 | 49.55 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 101 | 256 | 90 |
| views/all/bins | count | 3494 | 4310 | 3225 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 32.1 k | 33.9 k | 33.9 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 10.8 k | 13.9 k | 9784 |
| views/all/pipelines | count | 144 | 256 | 68 |
| views/all/pipelines_distinct | count | 61 | 66 | 53 |
| views/all/work_items | count | 256.4 k | 259.0 k | 255.8 k |
| views/cascade/batch_sets | count | 46 | 57 | 40 |
| views/cascade/bins | count | 1435 | 1794 | 951 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 19.3 k | 20.3 k | 20.3 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 4749 | 6077 | 3848 |
| views/cascade/pipelines | count | 38 | 48 | 30 |
| views/cascade/work_items | count | 152.2 k | 153.5 k | 151.3 k |
| views/lamp_face/batch_sets | count | 4 | 161 | 0 |
| views/lamp_face/bins | count | 218 | 703 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 346 | 1059 | 0 |
| views/lamp_face/pipelines | count | 64 | 175 | 0 |
| views/lamp_face/work_items | count | 29 | 792 | 0 |
| views/main/batch_sets | count | 50 | 54 | 50 |
| views/main/bins | count | 2022 | 2318 | 2274 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.9 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5952 | 7562 | 5936 |
| views/main/pipelines | count | 38 | 42 | 38 |
| views/main/work_items | count | 104.5 k | 106.0 k | 104.5 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 20.5 k | 26.4 k | 18.5 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 52.22 | 287 | 302.1 k |
| anim/events/count | count | 0.27 | 4 | 1542 |
| assets/Image/added | count | 0.06 | 35 | 353 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.54 | 245 | 3116 |
| assets/Mesh/removed | count | 0.85 | 1228 | 4910 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 526 |
| assets/StandardMaterial/modified | count | 3.75 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.74 | 1379 | 4263 |
| physics/inserts | count | 0.65 | 157 | 3772 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 129 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 3 | 8 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 20 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5851 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1283 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.6 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 577.2 | main/busy/ms 555, surface/advance_cpu_ms 64.41, surface/cell_upload_ms 0.71, stream/drive/ms 0.37, weather/map/ms 0.22, bigspace/total/ms 0.10, weather/eval/ms 0.08, bigspace/high_precision/ms 0.07, stream/tick/ms 0.07, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.89 | 310.0 | main/busy/ms 589, render/thread/ms 270, render/busy/ms 199, render/acquire/ms 67.44, surface/advance_cpu_ms 64.41, render/graph/ms 5.88, render/present/ms 3.34, stream/drive/ms 1.04, surface/cell_upload_ms 0.71, terrain/update/ms 0.59, stream/tick/ms 0.47, terrain/select/ms 0.42 |
| warm-up | 0.99 | 98.4 | pipelines/wait_ms 751, render/thread/ms 289, render/busy/ms 217, pipelines/compile_ms 94.31, pipelines/front_ms 86.72, render/acquire/ms 67.56, main/busy/ms 54.23, terrain/update/ms 20.86, render/graph/ms 10.89, pipelines/driver_ms 7.59, render/present/ms 4.78, bigspace/total/ms 3.94 |
| warm-up | 1.22 | 83.5 | pipelines/wait_ms 577, pipelines/compile_ms 92.90, pipelines/front_ms 86.63, main/busy/ms 62.89, render/thread/ms 49.66, render/busy/ms 47.96, pipelines/driver_ms 6.27, render/graph/ms 5.12, stream/drive/ms 1.86, stream/tick/ms 1.55, render/present/ms 1.48, terrain/update/ms 1.07 |
| warm-up | 3.40 | 31.1 | main/busy/ms 46.81, render/thread/ms 21.88, render/busy/ms 20.72, render/graph/ms 6.54, stream/drive/ms 1.44, bigspace/total/ms 1.08, anim/evaluate_cpu/ms 0.98, render/present/ms 0.97, stream/tick/ms 0.80, terrain/update/ms 0.61, anim/evaluate/ms 0.54, bigspace/low_precision/ms 0.53 |
| measure | 56.37 | 28.9 | render/thread/ms 36.15, render/busy/ms 18.82, main/busy/ms 17.31, render/present/ms 17.13, render/graph/ms 9.10, anim/evaluate/ms 0.95, bigspace/total/ms 0.85, controller/feed/ms 0.69, terrain/update/ms 0.57, stream/drive/ms 0.56, terrain/select/ms 0.51, stream/tick/ms 0.49 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\fix4_on\balmora_walk@2560x1440_--scene-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (256 samples): GPU load **80%** (min 63%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.9 W · 71 °C (max 76) · VRAM 3813 MiB · throttled 100% of samples
