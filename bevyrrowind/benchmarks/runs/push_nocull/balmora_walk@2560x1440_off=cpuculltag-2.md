# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cpuculltag · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5838 frames over 65.6 s · **median 10.88 ms (92 fps)** · mean 11.24 · p95 14.14 · p99 20.82 (1% low 48 fps) · worst 31.77 ms · 127 spikes over 1.5× median · 85.9 stutters/min (work 85.0/min)
- warm-up: 336 frames over 4.5 s, worst 580.0 ms
- hitches (> 27.2 ms): **5** measured, 8 in warm-up
- timed GPU passes: 8.17 ms of a 11.24 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.88 ms, limit 11 ms
- **FAIL** frame_ms: 94 stutters (> 1.5x moving median of 60) in 1.09 min = 85.91/min, limit 1/min
- pass worst frame 31.77 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.241 | 2.869 | 0.188 |
| main_opaque_pass_3d | 1.483 | 2.244 | 0.051 |
| atmosphere_luts | 0.887 | 1.304 | 0.004 |
| early prepass | 0.544 | 1.036 | 0.083 |
| lightprobe_irradiance_map | 0.461 | 0.747 | 0.001 |
| bloom | 0.344 | 0.571 | 0.069 |
| lightprobe_radiance_map | 0.227 | 0.236 | 0.004 |
| ssao | 0.215 | 0.211 | 0.008 |
| uniform_allocation | 0.191 | 0.359 | 0.035 |
| auxiliary_views | 0.178 | 0.653 | 0.164 |
| bin_unpacking | 0.133 | 0.140 | 0.025 |
| water_fft | 0.130 | 0.127 | 0.003 |
| upscaling | 0.129 | 0.147 | 0.004 |
| clouds_trace | 0.129 | 0.220 | 0.001 |
| atmos_froxel_inject | 0.098 | 0.104 | 0.002 |
| tonemapping | 0.093 | 0.093 | 0.004 |
| water_velocity | 0.087 | 0.086 | 0.002 |
| clustering | 0.070 | 0.070 | 0.020 |
| clouds_composite | 0.065 | 0.064 | 0.002 |
| main_transparent_pass_3d | 0.063 | 0.113 | 0.112 |
| surface_trail | 0.061 | 0.058 | 0.002 |
| render_sky | 0.055 | 0.055 | 0.002 |
| early_mesh_preprocessing | 0.045 | 0.048 | 0.006 |
| clouds_resolve | 0.034 | 0.037 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.030 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| per_view_shadow_pass | 0.023 | 0.025 | 0.007 |
| lightprobe_copy | 0.015 | 0.015 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.002 | 0.006 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 10.921 | 0.000 | 6826.69 | 63756.5 |
| render/thread/ms | 9.988 | 12.578 | 29.82 | 58311.3 |
| render/busy/ms | 9.335 | 11.894 | 28.00 | 54498.8 |
| main/busy/ms | 8.829 | 11.070 | 23.50 | 51542.7 |
| render/graph/ms | 5.050 | 6.655 | 16.50 | 29479.5 |
| bigspace/total/ms | 0.549 | 1.235 | 3.25 | 3206.7 |
| render/present/ms | 0.517 | 1.076 | 3.08 | 3016.9 |
| pipelines/compile_ms | 0.310 | 0.000 | 177.12 | 1810.2 |
| terrain/update/ms | 0.304 | 0.396 | 1.68 | 1774.8 |
| pipelines/front_ms | 0.300 | 0.000 | 169.92 | 1750.9 |
| controller/feed/ms | 0.290 | 0.400 | 1.19 | 1693.8 |
| terrain/select/ms | 0.270 | 0.349 | 0.87 | 1574.5 |
| stream/drive/ms | 0.257 | 0.408 | 6.72 | 1499.6 |
| bigspace/low_precision/ms | 0.219 | 0.603 | 2.54 | 1276.9 |
| far/select/ms | 0.209 | 0.311 | 0.86 | 1221.9 |
| stream/tick/ms | 0.208 | 0.302 | 1.49 | 1213.4 |
| anim/evaluate_cpu/ms | 0.182 | 0.236 | 0.74 | 1061.0 |
| controller/move/ms | 0.152 | 0.322 | 0.65 | 887.0 |
| bigspace/high_precision/ms | 0.147 | 0.530 | 2.40 | 855.5 |
| anim/evaluate/ms | 0.141 | 0.272 | 1.71 | 820.3 |
| render/acquire/ms | 0.136 | 0.221 | 18.92 | 795.5 |
| physics/step/ms | 0.051 | 0.096 | 1.38 | 295.4 |
| physics/insert/ms | 0.048 | 0.415 | 1.09 | 279.3 |
| weather/map/ms | 0.029 | 0.038 | 0.40 | 168.3 |
| anim/apply/ms | 0.012 | 0.022 | 0.22 | 71.7 |
| weather/eval/ms | 0.012 | 0.017 | 0.43 | 70.1 |
| controller/avoid/ms | 0.011 | 0.015 | 0.29 | 66.1 |
| pipelines/driver_ms | 0.010 | 0.000 | 7.20 | 59.3 |
| script/update/ms | 0.008 | 0.012 | 0.21 | 46.7 |
| stream/despawn/ms | 0.006 | 0.000 | 1.79 | 32.8 |
| script/input/ms | 0.006 | 0.008 | 0.25 | 32.3 |
| script/ui/ms | 0.005 | 0.008 | 0.43 | 31.3 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 26.9 |
| controller/write/ms | 0.004 | 0.005 | 0.08 | 21.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.68 | 15.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.13 | 6.6 |
| shapes/activate/ms | 0.001 | 0.002 | 0.27 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 5.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 16 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 44 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
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
| frame/work/ms | ms | 9.31 | 32.07 | 8.68 |
| gpu/timed/ms | ms | 8.11 | 32.07 | 8.68 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 818 | 1420 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 818 | 1420 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1288 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3415 | 5122 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1288 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3415 | 5122 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 38 | 231 | 28 |
| phases/shadow/bins | count | 3806 | 5287 | 2222 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9616 | 12.7 k | 5673 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 114 | 245 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2346 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1168 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 167 | 169 | 169 |
| pipelines/waiting | count | 0 | 24 | 0 |
| preprocess/bind_groups | count | 10 | 605 | 6 |
| preprocess/bind_groups_created | count | 10 | 605 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 172 | 257 | 158 |
| process/cpu_usage | % | 26.46 | 41.35 | 22.48 |
| process/mem_usage | GiB | 2.65 | 2.86 | 2.86 |
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
| started_load_count |  | 3476 | 3545 | 3545 |
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
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.23 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 39.97 | 77.98 | 38.39 |
| system/mem_usage | % | 52.69 | 53.33 | 53.31 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 84 | 271 | 72 |
| views/all/bins | count | 6522 | 8148 | 5702 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.3 k | 25.2 k | 13.3 k |
| views/all/pipelines | count | 156 | 285 | 68 |
| views/all/pipelines_distinct | count | 62 | 68 | 55 |
| views/all/work_items | count | 17.3 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 28 |
| views/cascade/bins | count | 2613 | 3154 | 2222 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8164 | 10.2 k | 5673 |
| views/cascade/pipelines | count | 36 | 50 | 28 |
| views/cascade/work_items | count | 8164 | 10.2 k | 5673 |
| views/lamp_face/batch_sets | count | 0 | 192 | 0 |
| views/lamp_face/bins | count | 1106 | 2369 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1265 | 2733 | 0 |
| views/lamp_face/pipelines | count | 74 | 200 | 0 |
| views/lamp_face/work_items | count | 0 | 2529 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2874 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8447 | 13.0 k | 7592 |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 8606 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 53.07 | 563 | 309.8 k |
| anim/events/count | count | 0.28 | 4 | 1625 |
| assets/Image/added | count | 0.06 | 35 | 349 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.72 | 160 | 4188 |
| assets/Mesh/removed | count | 0.84 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 511 |
| assets/StandardMaterial/modified | count | 3.71 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.38 | 1409 | 8043 |
| physics/inserts | count | 0.52 | 106 | 3043 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.67 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 7 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 22 | 378 |
| stream/upload/bytes | bytes | 5799 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 62 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1271 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.2 ms; slowest 13, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 580.0 | main/busy/ms 557, surface/advance_cpu_ms 65.79, surface/cell_upload_ms 0.78, stream/drive/ms 0.33, weather/map/ms 0.11, stream/tick/ms 0.10, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 0.91 | 333.0 | main/busy/ms 592, render/thread/ms 296, render/busy/ms 226, render/acquire/ms 68.32, surface/advance_cpu_ms 65.79, render/graph/ms 5.13, render/present/ms 1.92, stream/drive/ms 1.15, surface/cell_upload_ms 0.78, stream/tick/ms 0.63, terrain/update/ms 0.49, terrain/select/ms 0.26 |
| warm-up | 0.97 | 61.3 | pipelines/wait_ms 953, render/thread/ms 306, render/busy/ms 235, pipelines/compile_ms 81.23, pipelines/front_ms 74.62, render/acquire/ms 68.57, main/busy/ms 53.99, terrain/update/ms 21.08, render/graph/ms 8.95, pipelines/driver_ms 6.61, render/present/ms 2.78, bigspace/total/ms 1.99 |
| warm-up | 1.21 | 90.3 | pipelines/compile_ms 192, pipelines/front_ms 176, main/busy/ms 57.26, render/thread/ms 49.98, render/busy/ms 48.42, pipelines/driver_ms 15.95, render/graph/ms 3.77, stream/drive/ms 1.97, stream/tick/ms 1.62, render/present/ms 1.15, terrain/update/ms 0.77, terrain/select/ms 0.64 |
| warm-up | 1.90 | 30.1 | main/busy/ms 30.02, render/thread/ms 25.17, render/busy/ms 23.78, render/graph/ms 3.74, stream/drive/ms 2.50, render/present/ms 1.19, stream/tick/ms 1.03, bigspace/total/ms 0.58, terrain/update/ms 0.57, stream/spawn/ms 0.57, terrain/select/ms 0.49, render/acquire/ms 0.21 |
| warm-up | 2.08 | 29.2 | pipelines/wait_ms 2081, render/thread/ms 37.23, render/busy/ms 36.07, pipelines/compile_ms 23.72, pipelines/front_ms 22.36, main/busy/ms 18.15, render/graph/ms 3.88, stream/drive/ms 3.25, stream/spawn/ms 1.86, pipelines/driver_ms 1.36, render/present/ms 0.97, terrain/update/ms 0.70 |
| warm-up | 2.45 | 30.4 | main/busy/ms 50.84, render/thread/ms 18.40, render/busy/ms 17.12, render/graph/ms 4.79, stream/drive/ms 3.46, stream/spawn/ms 2.07, render/present/ms 1.11, bigspace/total/ms 0.88, stream/tick/ms 0.75, anim/evaluate_cpu/ms 0.70, terrain/update/ms 0.52, terrain/select/ms 0.45 |
| warm-up | 2.48 | 28.4 | main/busy/ms 39.73, render/thread/ms 23.88, render/busy/ms 22.64, render/graph/ms 4.91, stream/drive/ms 3.48, stream/spawn/ms 2.04, render/present/ms 1.04, bigspace/total/ms 0.99, stream/tick/ms 0.76, anim/evaluate_cpu/ms 0.72, terrain/update/ms 0.53, terrain/select/ms 0.46 |
| measure | 18.13 | 28.5 | render/thread/ms 39.39, render/busy/ms 37.92, main/busy/ms 28.26, render/graph/ms 18.00, render/present/ms 1.25, bigspace/total/ms 1.22, controller/feed/ms 0.70, terrain/update/ms 0.68, terrain/select/ms 0.61, stream/drive/ms 0.50, stream/tick/ms 0.39, bigspace/low_precision/ms 0.39 |
| measure | 23.36 | 27.6 | render/thread/ms 38.68, render/busy/ms 37.01, main/busy/ms 32.85, render/graph/ms 20.17, bigspace/total/ms 1.52, render/present/ms 1.20, physics/insert/ms 1.03, controller/feed/ms 0.76, stream/drive/ms 0.74, stream/tick/ms 0.65, terrain/update/ms 0.57, bigspace/low_precision/ms 0.53 |
| measure | 37.26 | 31.8 | render/thread/ms 42.98, render/busy/ms 40.27, main/busy/ms 34.98, render/graph/ms 16.33, render/present/ms 2.26, stream/drive/ms 1.67, bigspace/total/ms 1.23, stream/tick/ms 0.86, controller/feed/ms 0.78, far/select/ms 0.78, stream/spawn/ms 0.67, anim/evaluate_cpu/ms 0.63 |
| measure | 37.29 | 30.2 | render/thread/ms 56.41, render/busy/ms 53.03, main/busy/ms 41.49, render/graph/ms 21.68, render/present/ms 2.40, stream/drive/ms 1.26, bigspace/total/ms 1.21, controller/feed/ms 1.16, stream/tick/ms 1.12, far/select/ms 1.02, render/acquire/ms 0.98, controller/move/ms 0.73 |
| measure | 45.34 | 28.4 | render/thread/ms 35.93, render/acquire/ms 19.02, render/busy/ms 15.87, main/busy/ms 13.82, render/graph/ms 7.45, render/present/ms 1.03, bigspace/total/ms 0.73, terrain/update/ms 0.66, terrain/select/ms 0.60, controller/feed/ms 0.59, stream/drive/ms 0.53, stream/tick/ms 0.46 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\push_nocull\balmora_walk@2560x1440_off=cpuculltag-2 --bench-baseline none --off cpuculltag
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 14% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **84%** (min 55%) · 2580 MHz (min 2580) · memory 8001 MHz · 78.7 W · 73 °C (max 78) · VRAM 4237 MiB · throttled 100% of samples
