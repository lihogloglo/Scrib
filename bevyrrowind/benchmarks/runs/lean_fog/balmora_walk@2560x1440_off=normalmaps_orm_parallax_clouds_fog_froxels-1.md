# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: clouds,fog,froxels,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5858 frames over 65.7 s · **median 10.91 ms (92 fps)** · mean 11.21 · p95 13.94 · p99 15.95 (1% low 63 fps) · worst 31.73 ms · 42 spikes over 1.5× median · 30.2 stutters/min (work 33.8/min)
- warm-up: 339 frames over 8.2 s, worst 2564.8 ms
- hitches (> 27.3 ms): **5** measured, 32 in warm-up
- timed GPU passes: 6.74 ms of a 11.21 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.91 ms, limit 11 ms
- **FAIL** frame_ms: 33 stutters (> 1.5x moving median of 60) in 1.09 min = 30.16/min, limit 1/min
- pass worst frame 31.73 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.100 | 2.225 | 2.776 | 0.94 | 0.180 |
| main_opaque_pass_3d | 1.479 | 1.568 | 2.425 | 0.94 | 0.055 |
| early prepass | 0.477 | 0.505 | 0.741 | 0.94 | 0.093 |
| bloom | 0.322 | 0.342 | 0.491 | 0.94 | 0.065 |
| atmosphere_luts | 0.308 | 0.327 | 0.456 | 0.94 | 0.003 |
| water_velocity | 0.289 | 0.306 | 0.518 | 0.94 | 0.002 |
| uniform_allocation | 0.216 | 0.229 | 0.238 | 0.94 | 0.039 |
| water_sspr | 0.207 | 0.220 | 0.208 | 0.94 | 0.002 |
| ssao | 0.195 | 0.207 | 0.209 | 0.94 | 0.008 |
| bin_unpacking | 0.173 | 0.184 | 0.193 | 0.94 | 0.028 |
| auxiliary_views | 0.166 | 0.176 | 0.663 | 0.94 | 0.156 |
| water_fft | 0.133 | 0.141 | 0.138 | 0.94 | 0.003 |
| upscaling | 0.125 | 0.133 | 0.148 | 0.94 | 0.004 |
| tonemapping | 0.087 | 0.092 | 0.093 | 0.94 | 0.003 |
| main_transmissive_pass_3d | 0.085 | 0.090 | 0.167 | 0.94 | 0.008 |
| clustering | 0.065 | 0.069 | 0.071 | 0.94 | 0.020 |
| render_sky | 0.052 | 0.055 | 0.054 | 0.94 | 0.002 |
| lightprobe_irradiance_map | 0.046 | 0.437 | 0.722 | 0.10 | 0.001 |
| early_mesh_preprocessing | 0.042 | 0.044 | 0.049 | 0.94 | 0.005 |
| surface_trail | 0.040 | 0.058 | 0.057 | 0.69 | 0.002 |
| water_snapshot | 0.025 | 0.026 | 0.036 | 0.94 | 0.002 |
| lightprobe_radiance_map | 0.025 | 0.235 | 0.238 | 0.10 | 0.004 |
| water_mips | 0.024 | 0.025 | 0.024 | 0.94 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.023 | 0.024 | 0.94 | 0.006 |
| main_transparent_pass_3d | 0.012 | 0.013 | 0.027 | 0.94 | 0.076 |
| atmosphere_environment | 0.010 | 0.011 | 0.012 | 0.94 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.94 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.94 | 0.002 |
| lightprobe_downsampling_first_pass | 0.001 | 0.014 | 0.013 | 0.10 | 0.001 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.10 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 21.359 | 0.000 | 10113.15 | 125119.4 |
| render/thread/ms | 10.028 | 12.621 | 30.44 | 58746.6 |
| render/busy/ms | 9.432 | 12.041 | 25.48 | 55252.4 |
| main/busy/ms | 8.603 | 10.678 | 28.57 | 50396.8 |
| render/graph/ms | 5.344 | 7.066 | 13.97 | 31302.4 |
| pipelines/compile_ms | 0.634 | 0.000 | 1177.72 | 3713.8 |
| bigspace/total/ms | 0.548 | 1.493 | 2.91 | 3212.6 |
| render/present/ms | 0.505 | 1.166 | 16.93 | 2955.5 |
| pipelines/front_ms | 0.436 | 0.000 | 152.66 | 2557.0 |
| terrain/update/ms | 0.298 | 0.383 | 2.04 | 1745.2 |
| controller/feed/ms | 0.277 | 0.380 | 0.79 | 1623.8 |
| terrain/select/ms | 0.264 | 0.340 | 0.69 | 1549.3 |
| stream/drive/ms | 0.252 | 0.353 | 20.57 | 1477.3 |
| bigspace/low_precision/ms | 0.232 | 0.656 | 2.49 | 1358.2 |
| pipelines/driver_ms | 0.197 | 0.000 | 1065.96 | 1156.9 |
| stream/tick/ms | 0.197 | 0.273 | 0.79 | 1154.8 |
| far/select/ms | 0.196 | 0.275 | 0.59 | 1146.0 |
| anim/evaluate/ms | 0.186 | 0.321 | 2.28 | 1088.6 |
| bigspace/high_precision/ms | 0.162 | 0.581 | 2.56 | 948.4 |
| anim/evaluate_cpu/ms | 0.147 | 0.185 | 0.46 | 861.3 |
| controller/move/ms | 0.133 | 0.263 | 0.50 | 780.9 |
| render/acquire/ms | 0.092 | 0.123 | 0.39 | 538.6 |
| physics/step/ms | 0.050 | 0.089 | 1.09 | 292.5 |
| physics/insert/ms | 0.050 | 0.427 | 0.93 | 291.5 |
| weather/eval/ms | 0.011 | 0.015 | 0.07 | 65.1 |
| anim/apply/ms | 0.011 | 0.019 | 0.27 | 64.2 |
| controller/avoid/ms | 0.010 | 0.014 | 0.07 | 60.9 |
| script/update/ms | 0.007 | 0.010 | 0.14 | 42.0 |
| stream/despawn/ms | 0.005 | 0.000 | 1.25 | 30.0 |
| script/input/ms | 0.005 | 0.007 | 0.20 | 29.4 |
| script/ui/ms | 0.005 | 0.006 | 0.12 | 27.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.09 | 26.5 |
| controller/write/ms | 0.004 | 0.005 | 0.07 | 21.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 14.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 13 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 40 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1272 | 2262 | 595 |
| crowd/c0/meshes/count | count | 352 | 646 | 158 |
| crowd/c1/count | count | 146 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5941 | 5601 |
| crowd/c1/meshes/count | count | 1167 | 1283 | 1183 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4268 | 9682 | 6179 |
| crowd/joints_uploaded/count | count | 4248 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.2 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.26 | 28.57 | 8.14 |
| gpu/timed/ms | ms | 7.86 | 10.41 | 8.02 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 809 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 809 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1291 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3400 | 5131 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1291 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3400 | 5131 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 54 | 296 | 42 |
| phases/shadow/bins | count | 3761 | 5489 | 2234 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9668 | 12.6 k | 5735 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 135 | 315 | 42 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2346 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 1170 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 200 | 205 | 205 |
| pipelines/waiting | count | 0 | 27 | 0 |
| preprocess/bind_groups | count | 11 | 761 | 10 |
| preprocess/bind_groups_created | count | 11 | 761 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 230 | 343 | 217 |
| process/cpu_usage | % | 25.71 | 42.79 | 27.67 |
| process/mem_usage | GiB | 5.18 | 5.43 | 5.43 |
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
| started_load_count |  | 4750 | 4844 | 4844 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 440 | 535 | 402 |
| statics/materials_bound | count | 440 | 535 | 402 |
| statics/materials_unclassed | count | 6 | 8 | 6 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.99 | 54.57 | 31.01 |
| system/mem_usage | % | 58.90 | 59.87 | 59.61 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2808 | 2919 | 2919 |
| textures/images | count | 2585 | 2638 | 2638 |
| views/all/batch_sets | count | 112 | 348 | 102 |
| views/all/bins | count | 6470 | 8235 | 5714 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.3 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 188 | 369 | 98 |
| views/all/pipelines_distinct | count | 83 | 96 | 80 |
| views/all/work_items | count | 17.4 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 53 | 68 | 42 |
| views/cascade/bins | count | 2622 | 3195 | 2234 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8246 | 9932 | 5735 |
| views/cascade/pipelines | count | 53 | 68 | 42 |
| views/cascade/work_items | count | 8246 | 9932 | 5735 |
| views/lamp_face/batch_sets | count | 0 | 240 | 0 |
| views/lamp_face/bins | count | 1059 | 2526 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1194 | 2902 | 0 |
| views/lamp_face/pipelines | count | 81 | 254 | 0 |
| views/lamp_face/work_items | count | 0 | 2554 | 0 |
| views/main/batch_sets | count | 58 | 62 | 60 |
| views/main/bins | count | 2880 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8411 | 13.0 k | 7600 |
| views/main/pipelines | count | 54 | 58 | 56 |
| views/main/work_items | count | 8575 | 13.2 k | 7908 |
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
| anim/bones_written/count | count | 49.06 | 455 | 287.4 k |
| anim/events/count | count | 0.09 | 5 | 551 |
| assets/Image/added | count | 0.12 | 40 | 678 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 8 | 46 |
| assets/Mesh/added | count | 0.73 | 152 | 4304 |
| assets/Mesh/removed | count | 0.83 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 49 | 564 |
| assets/StandardMaterial/modified | count | 3.72 | 22 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.64 | 1389 | 9599 |
| physics/inserts | count | 0.59 | 135 | 3436 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.67 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 16 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5779 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1267 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.3 ms; slowest 37, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2564.8 | main/busy/ms 2535, surface/advance_cpu_ms 68.18, surface/cell_upload_ms 0.77, stream/drive/ms 0.30, bigspace/total/ms 0.12, stream/tick/ms 0.08, bigspace/high_precision/ms 0.08, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.22 | 659.1 | main/busy/ms 2574, render/thread/ms 621, render/busy/ms 487, render/acquire/ms 132, surface/advance_cpu_ms 68.18, render/graph/ms 5.18, render/present/ms 2.30, stream/drive/ms 1.29, surface/cell_upload_ms 0.77, stream/tick/ms 0.64, terrain/update/ms 0.57, terrain/select/ms 0.37 |
| warm-up | 3.30 | 75.2 | pipelines/wait_ms 684, render/thread/ms 633, render/busy/ms 498, render/acquire/ms 132, pipelines/compile_ms 81.70, pipelines/front_ms 74.81, main/busy/ms 54.97, terrain/update/ms 20.71, render/graph/ms 8.63, pipelines/driver_ms 6.90, render/present/ms 3.09, stream/drive/ms 1.81 |
| warm-up | 3.56 | 102.7 | pipelines/wait_ms 377, pipelines/compile_ms 220, pipelines/front_ms 206, main/busy/ms 65.48, render/thread/ms 56.40, render/busy/ms 55.05, pipelines/driver_ms 14.17, terrain/update/ms 8.00, render/graph/ms 3.71, terrain/select/ms 1.56, stream/drive/ms 1.53, stream/tick/ms 1.24 |
| warm-up | 3.60 | 28.7 | render/thread/ms 40.23, render/busy/ms 38.44, main/busy/ms 28.50, terrain/update/ms 6.81, stream/drive/ms 3.87, render/graph/ms 3.36, stream/tick/ms 1.61, render/present/ms 1.55, stream/spawn/ms 1.33, terrain/select/ms 0.68, bigspace/total/ms 0.39, render/acquire/ms 0.24 |
| warm-up | 3.72 | 70.4 | pipelines/wait_ms 401, main/busy/ms 48.01, render/thread/ms 40.97, render/busy/ms 39.20, stream/drive/ms 33.08, render/graph/ms 2.95, pipelines/compile_ms 2.92, pipelines/front_ms 2.67, render/present/ms 1.55, terrain/update/ms 1.22, stream/tick/ms 1.05, terrain/select/ms 0.89 |
| warm-up | 3.80 | 42.0 | pipelines/wait_ms 1266, render/thread/ms 57.51, pipelines/compile_ms 44.84, pipelines/front_ms 42.59, main/busy/ms 40.26, render/busy/ms 34.19, stream/drive/ms 25.02, render/present/ms 23.08, render/graph/ms 4.44, pipelines/driver_ms 2.25, stream/tick/ms 1.47, terrain/update/ms 0.80 |
| warm-up | 3.84 | 43.9 | pipelines/wait_ms 437, render/thread/ms 76.06, render/busy/ms 52.36, main/busy/ms 44.32, stream/drive/ms 26.31, render/present/ms 23.36, pipelines/compile_ms 11.52, pipelines/front_ms 10.96, render/graph/ms 7.11, stream/tick/ms 1.42, stream/spawn/ms 1.24, terrain/update/ms 0.98 |
| warm-up | 3.98 | 111.7 | pipelines/wait_ms 10.9 k, pipelines/compile_ms 199, pipelines/front_ms 184, render/thread/ms 81.14, render/busy/ms 53.11, main/busy/ms 50.24, stream/drive/ms 31.90, render/present/ms 27.87, pipelines/driver_ms 15.20, render/graph/ms 3.15, stream/spawn/ms 2.16, terrain/update/ms 1.11 |
| warm-up | 4.02 | 36.2 | pipelines/wait_ms 12.2 k, pipelines/compile_ms 162, pipelines/front_ms 147, render/thread/ms 88.86, render/busy/ms 60.72, main/busy/ms 31.94, render/present/ms 27.86, pipelines/driver_ms 14.96, stream/drive/ms 4.49, render/graph/ms 3.98, stream/spawn/ms 1.43, terrain/update/ms 1.16 |
| warm-up | 4.05 | 34.9 | pipelines/wait_ms 5445, pipelines/compile_ms 93.74, pipelines/front_ms 85.05, render/thread/ms 53.42, render/busy/ms 51.79, main/busy/ms 30.34, render/graph/ms 11.15, pipelines/driver_ms 8.69, terrain/update/ms 3.94, stream/drive/ms 3.93, render/present/ms 1.33, stream/tick/ms 1.06 |
| warm-up | 4.08 | 29.3 | pipelines/wait_ms 2120, render/thread/ms 51.51, render/busy/ms 49.82, main/busy/ms 21.44, render/graph/ms 12.27, pipelines/compile_ms 11.18, pipelines/front_ms 10.23, terrain/update/ms 3.76, stream/drive/ms 2.79, render/present/ms 1.30, stream/activate/ms 1.03, stream/tick/ms 0.99 |
| warm-up | 4.11 | 29.4 | render/thread/ms 52.07, render/busy/ms 49.73, main/busy/ms 25.19, render/graph/ms 16.64, terrain/update/ms 5.62, stream/drive/ms 4.73, stream/spawn/ms 2.62, render/present/ms 1.90, stream/tick/ms 0.84, terrain/select/ms 0.57, stream/activate/ms 0.51, render/acquire/ms 0.44 |
| warm-up | 4.19 | 47.7 | main/busy/ms 74.71, stream/drive/ms 58.23, render/thread/ms 46.40, render/busy/ms 44.71, render/graph/ms 4.36, terrain/update/ms 3.28, stream/spawn/ms 2.15, stream/tick/ms 1.76, render/present/ms 1.35, terrain/select/ms 0.81, bigspace/total/ms 0.47, render/acquire/ms 0.34 |
| warm-up | 4.25 | 63.5 | pipelines/wait_ms 2476, pipelines/compile_ms 190, pipelines/front_ms 172, main/busy/ms 74.54, stream/drive/ms 58.19, render/thread/ms 44.53, render/busy/ms 43.22, pipelines/driver_ms 18.44, render/graph/ms 3.69, stream/spawn/ms 2.11, stream/tick/ms 1.49, render/present/ms 1.08 |
| warm-up | 4.34 | 73.2 | pipelines/wait_ms 4633, pipelines/compile_ms 141, pipelines/front_ms 131, main/busy/ms 66.20, stream/drive/ms 43.33, render/thread/ms 30.12, render/busy/ms 28.99, pipelines/driver_ms 10.57, render/graph/ms 3.50, stream/spawn/ms 2.27, render/present/ms 0.96, stream/tick/ms 0.84 |
| warm-up | 4.42 | 54.3 | pipelines/wait_ms 1007, main/busy/ms 66.05, render/thread/ms 42.36, render/busy/ms 40.52, stream/drive/ms 33.76, pipelines/compile_ms 5.28, render/graph/ms 4.76, pipelines/front_ms 4.74, render/present/ms 1.56, stream/activate/ms 1.11, terrain/update/ms 0.93, stream/tick/ms 0.87 |
| warm-up | 4.49 | 61.1 | pipelines/wait_ms 5371, pipelines/compile_ms 144, pipelines/front_ms 139, main/busy/ms 59.83, render/thread/ms 34.61, render/busy/ms 33.05, stream/drive/ms 25.58, render/graph/ms 5.36, pipelines/driver_ms 5.01, render/present/ms 1.27, stream/activate/ms 1.05, physics/insert/ms 0.99 |
| warm-up | 4.53 | 35.5 | pipelines/wait_ms 14.4 k, pipelines/compile_ms 196, pipelines/front_ms 186, render/thread/ms 49.77, render/busy/ms 48.28, main/busy/ms 32.88, pipelines/driver_ms 10.23, render/graph/ms 5.16, stream/drive/ms 4.51, stream/activate/ms 1.45, render/present/ms 1.17, stream/spawn/ms 0.89 |
| warm-up | 4.56 | 33.4 | pipelines/wait_ms 12.4 k, pipelines/compile_ms 73.93, pipelines/front_ms 66.89, main/busy/ms 52.37, render/thread/ms 52.11, render/busy/ms 50.51, pipelines/driver_ms 7.04, render/graph/ms 5.98, stream/drive/ms 4.45, stream/activate/ms 1.44, render/present/ms 1.28, stream/spawn/ms 0.89 |
| warm-up | 4.62 | 61.1 | pipelines/wait_ms 3402, render/thread/ms 57.17, render/busy/ms 55.55, main/busy/ms 55.29, pipelines/compile_ms 21.87, pipelines/front_ms 20.05, render/graph/ms 6.05, stream/drive/ms 3.68, pipelines/driver_ms 1.82, render/present/ms 1.33, stream/spawn/ms 1.15, terrain/update/ms 0.86 |
| warm-up | 4.73 | 29.5 | pipelines/wait_ms 3712, pipelines/compile_ms 154, pipelines/front_ms 143, render/thread/ms 39.42, render/busy/ms 38.24, main/busy/ms 24.69, stream/drive/ms 11.40, pipelines/driver_ms 10.19, render/graph/ms 4.71, stream/spawn/ms 1.59, render/present/ms 1.02, stream/tick/ms 0.95 |
| warm-up | 5.35 | 27.4 | pipelines/wait_ms 3999, render/thread/ms 43.86, render/busy/ms 41.36, pipelines/compile_ms 36.30, pipelines/front_ms 32.70, main/busy/ms 30.73, render/graph/ms 10.03, stream/drive/ms 5.41, pipelines/driver_ms 3.60, stream/spawn/ms 2.76, render/present/ms 2.14, stream/tick/ms 1.74 |
| warm-up | 5.39 | 41.3 | pipelines/wait_ms 3999, render/thread/ms 44.81, render/busy/ms 43.13, pipelines/compile_ms 36.30, pipelines/front_ms 32.70, main/busy/ms 25.79, render/graph/ms 7.74, stream/drive/ms 4.67, pipelines/driver_ms 3.60, stream/spawn/ms 2.25, stream/tick/ms 1.59, render/present/ms 1.38 |
| warm-up | 5.62 | 40.4 | render/thread/ms 60.03, render/busy/ms 57.98, main/busy/ms 55.56, render/graph/ms 31.90, stream/drive/ms 6.95, stream/spawn/ms 2.29, render/present/ms 1.63, physics/insert/ms 1.30, bigspace/total/ms 1.00, stream/tick/ms 0.93, terrain/update/ms 0.65, terrain/select/ms 0.55 |
| warm-up | 5.67 | 28.3 | render/thread/ms 37.39, render/busy/ms 36.05, main/busy/ms 28.49, render/graph/ms 7.81, stream/drive/ms 4.34, stream/spawn/ms 2.11, stream/tick/ms 1.08, render/present/ms 1.07, anim/evaluate/ms 1.04, terrain/update/ms 0.97, terrain/select/ms 0.83, bigspace/total/ms 0.71 |
| warm-up | 5.75 | 28.1 | render/thread/ms 33.16, render/busy/ms 31.93, main/busy/ms 24.38, render/graph/ms 7.92, stream/drive/ms 3.78, stream/spawn/ms 2.08, render/present/ms 1.01, terrain/update/ms 0.84, stream/tick/ms 0.78, bigspace/total/ms 0.73, terrain/select/ms 0.72, bigspace/high_precision/ms 0.33 |
| warm-up | 5.95 | 54.4 | render/thread/ms 49.26, render/busy/ms 47.99, main/busy/ms 39.33, stream/drive/ms 17.27, render/graph/ms 7.25, stream/spawn/ms 2.19, render/present/ms 1.08, stream/tick/ms 0.80, terrain/update/ms 0.66, bigspace/total/ms 0.63, terrain/select/ms 0.57, bigspace/high_precision/ms 0.26 |
| warm-up | 6.11 | 32.7 | render/thread/ms 36.28, render/busy/ms 34.51, main/busy/ms 30.49, render/graph/ms 9.51, stream/drive/ms 3.97, stream/spawn/ms 2.40, render/present/ms 1.52, bigspace/total/ms 0.78, stream/tick/ms 0.78, terrain/update/ms 0.73, terrain/select/ms 0.64, render/acquire/ms 0.25 |
| warm-up | 8.04 | 30.1 | pipelines/wait_ms 13.5 k, pipelines/compile_ms 1390, pipelines/driver_ms 1222, pipelines/front_ms 169, main/busy/ms 45.89, render/thread/ms 29.63, render/busy/ms 28.18, render/graph/ms 10.24, render/present/ms 1.23, stream/drive/ms 1.00, anim/evaluate_cpu/ms 0.94, bigspace/total/ms 0.79 |
| warm-up | 8.08 | 33.7 | pipelines/wait_ms 14.8 k, pipelines/compile_ms 63.76, pipelines/front_ms 60.79, main/busy/ms 59.78, render/thread/ms 35.45, render/busy/ms 34.25, render/graph/ms 7.31, pipelines/driver_ms 2.97, anim/evaluate_cpu/ms 1.26, bigspace/total/ms 1.09, render/present/ms 0.98, stream/drive/ms 0.98 |
| warm-up | 8.11 | 33.2 | pipelines/wait_ms 13.0 k, pipelines/compile_ms 50.11, main/busy/ms 48.15, pipelines/front_ms 47.98, render/thread/ms 31.60, render/busy/ms 30.48, render/graph/ms 7.27, pipelines/driver_ms 2.14, stream/drive/ms 1.01, physics/insert/ms 1.00, bigspace/total/ms 0.98, render/present/ms 0.96 |
| measure | 10.51 | 27.9 | render/thread/ms 38.72, render/busy/ms 22.93, main/busy/ms 20.18, render/present/ms 15.62, render/graph/ms 12.90, bigspace/total/ms 1.17, terrain/update/ms 0.75, controller/feed/ms 0.71, bigspace/low_precision/ms 0.69, terrain/select/ms 0.67, stream/drive/ms 0.57, far/select/ms 0.48 |
| measure | 18.18 | 30.0 | main/busy/ms 38.12, render/thread/ms 22.26, render/busy/ms 21.26, stream/drive/ms 20.79, render/graph/ms 11.74, render/present/ms 0.82, bigspace/total/ms 0.79, terrain/update/ms 0.67, controller/feed/ms 0.63, terrain/select/ms 0.61, far/select/ms 0.39, stream/tick/ms 0.35 |
| measure | 19.27 | 27.5 | render/thread/ms 36.25, render/busy/ms 35.02, render/graph/ms 19.28, main/busy/ms 16.77, render/present/ms 1.06, bigspace/total/ms 0.84, terrain/update/ms 0.66, physics/insert/ms 0.65, terrain/select/ms 0.60, controller/feed/ms 0.58, anim/evaluate/ms 0.57, stream/drive/ms 0.43 |
| measure | 25.48 | 31.7 | render/thread/ms 42.62, render/busy/ms 24.94, main/busy/ms 21.91, render/present/ms 17.45, render/graph/ms 14.54, bigspace/total/ms 2.11, bigspace/low_precision/ms 1.08, terrain/update/ms 0.64, controller/feed/ms 0.59, terrain/select/ms 0.58, stream/drive/ms 0.52, far/select/ms 0.49 |
| measure | 59.00 | 28.2 | render/thread/ms 37.88, render/busy/ms 36.21, main/busy/ms 28.83, render/graph/ms 18.97, stream/drive/ms 1.48, render/present/ms 1.40, bigspace/total/ms 1.36, controller/feed/ms 1.17, stream/spawn/ms 0.74, anim/evaluate/ms 0.73, far/select/ms 0.71, anim/evaluate_cpu/ms 0.70 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_fog\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels-1 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 12%, CPU 12%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **82%** (min 61%) · 2578 MHz (min 2565) · memory 8001 MHz · 78.7 W · 76 °C (max 82) · VRAM 6287 MiB · throttled 100% of samples
