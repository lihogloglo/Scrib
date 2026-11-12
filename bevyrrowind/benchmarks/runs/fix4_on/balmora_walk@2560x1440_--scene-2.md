# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5791 frames over 65.6 s · **median 11.07 ms (90 fps)** · mean 11.34 · p95 13.42 · p99 14.83 (1% low 67 fps) · worst 30.25 ms · 22 spikes over 1.5× median · 13.7 stutters/min (work 17.4/min)
- warm-up: 364 frames over 4.8 s, worst 586.7 ms
- hitches (> 27.7 ms): **1** measured, 6 in warm-up
- timed GPU passes: 7.79 ms of a 11.34 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.07 ms, limit 11 ms
- **FAIL** frame_ms: 15 stutters (> 1.5x moving median of 60) in 1.09 min = 13.71/min, limit 1/min
- pass worst frame 30.25 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.276 | 2.806 | 0.178 |
| main_opaque_pass_3d | 1.498 | 2.131 | 0.047 |
| early prepass | 0.564 | 0.912 | 0.079 |
| lightprobe_irradiance_map | 0.444 | 0.789 | 0.001 |
| bloom | 0.340 | 0.489 | 0.064 |
| atmosphere_luts | 0.336 | 0.643 | 0.003 |
| lightprobe_radiance_map | 0.232 | 0.236 | 0.004 |
| ssao | 0.218 | 0.210 | 0.007 |
| uniform_allocation | 0.158 | 0.163 | 0.015 |
| auxiliary_views | 0.145 | 0.581 | 0.125 |
| water_fft | 0.135 | 0.128 | 0.002 |
| upscaling | 0.132 | 0.144 | 0.004 |
| bin_unpacking | 0.129 | 0.132 | 0.012 |
| clouds_trace | 0.127 | 0.213 | 0.001 |
| atmos_froxel_inject | 0.101 | 0.106 | 0.001 |
| tonemapping | 0.093 | 0.090 | 0.003 |
| gpu_instance_materialization | 0.092 | 0.094 | 0.004 |
| early_mesh_preprocessing | 0.090 | 0.101 | 0.004 |
| water_velocity | 0.087 | 0.085 | 0.001 |
| clustering | 0.071 | 0.069 | 0.018 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| surface_trail | 0.061 | 0.059 | 0.002 |
| main_transparent_pass_3d | 0.060 | 0.105 | 0.081 |
| render_sky | 0.054 | 0.054 | 0.002 |
| per_view_shadow_pass | 0.038 | 0.039 | 0.008 |
| clouds_resolve | 0.035 | 0.040 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| range_unpacking | 0.028 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.034 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.013 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 13.243 | 0.000 | 7745.15 | 76690.7 |
| render/thread/ms | 9.755 | 11.772 | 28.37 | 56489.2 |
| render/busy/ms | 9.217 | 11.211 | 22.47 | 53376.3 |
| main/busy/ms | 8.115 | 9.624 | 14.11 | 46991.6 |
| render/graph/ms | 4.535 | 5.708 | 16.10 | 26259.8 |
| render/present/ms | 0.447 | 0.550 | 18.07 | 2591.2 |
| bigspace/total/ms | 0.410 | 0.631 | 2.14 | 2373.8 |
| terrain/update/ms | 0.286 | 0.360 | 1.64 | 1658.2 |
| stream/drive/ms | 0.265 | 0.440 | 1.74 | 1536.9 |
| controller/feed/ms | 0.261 | 0.340 | 0.65 | 1511.1 |
| terrain/select/ms | 0.252 | 0.315 | 0.72 | 1457.5 |
| pipelines/compile_ms | 0.235 | 0.000 | 150.64 | 1358.1 |
| pipelines/front_ms | 0.227 | 0.000 | 144.23 | 1314.1 |
| far/select/ms | 0.213 | 0.360 | 0.70 | 1232.3 |
| stream/tick/ms | 0.213 | 0.367 | 0.80 | 1230.7 |
| anim/evaluate_cpu/ms | 0.173 | 0.226 | 0.51 | 1003.2 |
| bigspace/low_precision/ms | 0.158 | 0.243 | 1.54 | 917.7 |
| controller/move/ms | 0.136 | 0.264 | 0.56 | 788.8 |
| anim/evaluate/ms | 0.135 | 0.296 | 1.48 | 784.1 |
| bigspace/high_precision/ms | 0.100 | 0.161 | 1.59 | 576.9 |
| render/acquire/ms | 0.090 | 0.119 | 0.38 | 521.7 |
| anim/apply/ms | 0.085 | 0.159 | 1.27 | 494.8 |
| physics/step/ms | 0.049 | 0.086 | 1.07 | 282.2 |
| physics/insert/ms | 0.043 | 0.403 | 0.94 | 251.8 |
| weather/map/ms | 0.027 | 0.035 | 0.16 | 155.5 |
| controller/avoid/ms | 0.011 | 0.017 | 0.06 | 66.0 |
| weather/eval/ms | 0.011 | 0.014 | 0.06 | 61.5 |
| pipelines/driver_ms | 0.008 | 0.000 | 6.40 | 44.0 |
| script/update/ms | 0.007 | 0.013 | 0.14 | 41.1 |
| stream/despawn/ms | 0.005 | 0.000 | 1.24 | 31.5 |
| script/ui/ms | 0.005 | 0.008 | 0.11 | 29.7 |
| script/input/ms | 0.005 | 0.006 | 0.14 | 28.4 |
| stream/upload/ms | 0.004 | 0.000 | 1.09 | 21.6 |
| controller/write/ms | 0.004 | 0.006 | 0.06 | 21.4 |
| stream/spawn/ms | 0.003 | 0.000 | 1.25 | 19.6 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.82 | 16.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.12 | 6.2 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.44 | 5.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
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
| frame/work/ms | ms | 8.99 | 22.47 | 8.05 |
| gpu/timed/ms | ms | 7.81 | 10.32 | 8.05 |
| mesh_allocator_allocations |  meshes | 10.6 k | 11.8 k | 10.4 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 311.90 M | 311.90 M | 311.90 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 157 | 388 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 333 | 588 | 516 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 157 | 388 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 333 | 588 | 516 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/opaque3d/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d/bins | count | 764 | 973 | 757 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2516 | 3576 | 2450 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 764 | 973 | 757 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2516 | 3576 | 2450 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 48 | 217 | 40 |
| phases/shadow/bins | count | 1612 | 2371 | 959 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 19.3 k | 20.3 k | 20.3 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 5139 | 7040 | 3856 |
| phases/shadow/pipelines | count | 106 | 220 | 30 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2668 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 938 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 170 | 171 | 171 |
| pipelines/waiting | count | 0 | 24 | 0 |
| preprocess/bind_groups | count | 186 | 561 | 167 |
| preprocess/bind_groups_created | count | 179 | 554 | 160 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 23.16 | 37.80 | 19.02 |
| process/mem_usage | GiB | 2.65 | 2.71 | 2.71 |
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
| started_load_count |  | 5017 | 5110 | 5110 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 237 | 333 | 180 |
| statics/materials_bound | count | 237 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.05 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.24 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.60 | 46.96 | 27.53 |
| system/mem_usage | % | 50.00 | 50.54 | 50.54 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 99 | 263 | 90 |
| views/all/bins | count | 3491 | 4369 | 3233 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 32.1 k | 33.9 k | 33.9 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 10.9 k | 13.9 k | 9788 |
| views/all/pipelines | count | 144 | 258 | 68 |
| views/all/pipelines_distinct | count | 60 | 65 | 53 |
| views/all/work_items | count | 256.3 k | 258.9 k | 255.8 k |
| views/cascade/batch_sets | count | 46 | 57 | 40 |
| views/cascade/bins | count | 1426 | 1796 | 959 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 19.3 k | 20.3 k | 20.3 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 4748 | 6074 | 3856 |
| views/cascade/pipelines | count | 37 | 48 | 30 |
| views/cascade/work_items | count | 152.2 k | 153.5 k | 151.3 k |
| views/lamp_face/batch_sets | count | 0 | 168 | 0 |
| views/lamp_face/bins | count | 194 | 683 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 349 | 1029 | 0 |
| views/lamp_face/pipelines | count | 67 | 176 | 0 |
| views/lamp_face/work_items | count | 0 | 861 | 0 |
| views/main/batch_sets | count | 50 | 54 | 50 |
| views/main/bins | count | 2022 | 2318 | 2274 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.9 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5952 | 7572 | 5932 |
| views/main/pipelines | count | 40 | 42 | 38 |
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
| anim/bones_written/count | count | 49.63 | 199 | 287.4 k |
| anim/events/count | count | 0.27 | 4 | 1565 |
| assets/Image/added | count | 0.05 | 35 | 307 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.50 | 235 | 2908 |
| assets/Mesh/removed | count | 0.85 | 1228 | 4910 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 473 |
| assets/StandardMaterial/modified | count | 3.74 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 48 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.26 | 1328 | 1531 |
| physics/inserts | count | 0.54 | 117 | 3137 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 129 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 5 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5846 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1282 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.7 ms; slowest 7, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 586.7 | main/busy/ms 564, surface/advance_cpu_ms 69.80, surface/cell_upload_ms 0.69, stream/drive/ms 0.41, stream/tick/ms 0.10, bigspace/total/ms 0.09, weather/map/ms 0.09, bigspace/high_precision/ms 0.07, weather/eval/ms 0.03, script/update/ms 0.02, script/ui/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.91 | 321.8 | main/busy/ms 603, render/thread/ms 285, render/busy/ms 214, surface/advance_cpu_ms 69.80, render/acquire/ms 67.84, render/graph/ms 4.97, render/present/ms 2.35, stream/drive/ms 1.12, surface/cell_upload_ms 0.69, stream/tick/ms 0.53, terrain/update/ms 0.39, bigspace/total/ms 0.29 |
| warm-up | 1.00 | 92.2 | pipelines/wait_ms 1002, render/thread/ms 302, render/busy/ms 230, pipelines/compile_ms 100, pipelines/front_ms 92.38, render/acquire/ms 68.11, main/busy/ms 56.77, terrain/update/ms 23.48, render/graph/ms 10.74, pipelines/driver_ms 8.08, render/present/ms 3.96, stream/drive/ms 1.12 |
| warm-up | 1.23 | 81.0 | pipelines/wait_ms 319, pipelines/compile_ms 134, pipelines/front_ms 122, main/busy/ms 62.41, render/thread/ms 56.07, render/busy/ms 54.37, pipelines/driver_ms 12.70, render/graph/ms 4.58, stream/drive/ms 1.63, render/present/ms 1.49, stream/tick/ms 1.45, terrain/update/ms 0.81 |
| warm-up | 3.59 | 30.2 | pipelines/wait_ms 4553, render/thread/ms 43.58, main/busy/ms 40.05, render/busy/ms 25.16, render/present/ms 18.15, pipelines/compile_ms 17.88, pipelines/front_ms 12.28, render/graph/ms 8.89, pipelines/driver_ms 5.59, stream/drive/ms 2.25, physics/insert/ms 1.75, stream/tick/ms 1.16 |
| warm-up | 3.64 | 29.2 | main/busy/ms 39.62, render/thread/ms 38.34, render/busy/ms 37.00, render/graph/ms 7.95, terrain/update/ms 6.42, physics/insert/ms 1.13, render/present/ms 1.12, stream/drive/ms 1.03, bigspace/total/ms 0.83, anim/evaluate_cpu/ms 0.49, physics/step/ms 0.49, stream/tick/ms 0.48 |
| measure | 33.75 | 30.3 | render/thread/ms 40.25, render/busy/ms 21.51, main/busy/ms 18.65, render/present/ms 18.56, render/graph/ms 10.79, bigspace/total/ms 0.99, controller/feed/ms 0.75, terrain/update/ms 0.56, stream/drive/ms 0.53, anim/evaluate_cpu/ms 0.50, terrain/select/ms 0.49, stream/tick/ms 0.46 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\fix4_on\balmora_walk@2560x1440_--scene-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 4%, CPU 12%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **81%** (min 68%) · 2580 MHz (min 2580) · memory 8001 MHz · 78.3 W · 72 °C (max 76) · VRAM 3820 MiB · throttled 100% of samples
