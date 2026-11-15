# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: clouds,fog,froxels,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5718 frames over 65.6 s · **median 11.09 ms (90 fps)** · mean 11.48 · p95 14.79 · p99 17.04 (1% low 59 fps) · worst 32.16 ms · 80 spikes over 1.5× median · 25.6 stutters/min (work 26.5/min)
- warm-up: 334 frames over 8.1 s, worst 2521.6 ms
- hitches (> 27.7 ms): **4** measured, 25 in warm-up
- timed GPU passes: 6.90 ms of a 11.48 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.09 ms, limit 11 ms
- **FAIL** frame_ms: 28 stutters (> 1.5x moving median of 60) in 1.09 min = 25.59/min, limit 1/min
- pass worst frame 32.16 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.136 | 2.216 | 2.676 | 0.96 | 0.178 |
| main_opaque_pass_3d | 1.507 | 1.564 | 2.204 | 0.96 | 0.062 |
| early prepass | 0.484 | 0.502 | 0.822 | 0.96 | 0.095 |
| bloom | 0.327 | 0.340 | 0.489 | 0.96 | 0.066 |
| atmosphere_luts | 0.318 | 0.329 | 0.463 | 0.96 | 0.004 |
| water_velocity | 0.292 | 0.303 | 0.481 | 0.96 | 0.002 |
| uniform_allocation | 0.219 | 0.227 | 0.237 | 0.96 | 0.042 |
| auxiliary_views | 0.214 | 0.222 | 0.758 | 0.96 | 0.201 |
| water_sspr | 0.209 | 0.217 | 0.208 | 0.96 | 0.002 |
| ssao | 0.201 | 0.208 | 0.212 | 0.96 | 0.008 |
| bin_unpacking | 0.179 | 0.185 | 0.199 | 0.96 | 0.029 |
| water_fft | 0.134 | 0.139 | 0.138 | 0.96 | 0.003 |
| upscaling | 0.127 | 0.132 | 0.139 | 0.96 | 0.005 |
| tonemapping | 0.090 | 0.094 | 0.095 | 0.96 | 0.003 |
| main_transmissive_pass_3d | 0.085 | 0.089 | 0.164 | 0.96 | 0.009 |
| clustering | 0.065 | 0.068 | 0.070 | 0.96 | 0.020 |
| render_sky | 0.052 | 0.054 | 0.054 | 0.96 | 0.002 |
| lightprobe_irradiance_map | 0.044 | 0.435 | 0.578 | 0.10 | 0.001 |
| early_mesh_preprocessing | 0.043 | 0.045 | 0.049 | 0.96 | 0.006 |
| surface_trail | 0.043 | 0.059 | 0.057 | 0.72 | 0.003 |
| water_snapshot | 0.025 | 0.026 | 0.035 | 0.96 | 0.002 |
| lightprobe_radiance_map | 0.025 | 0.242 | 0.243 | 0.10 | 0.005 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.96 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.023 | 0.024 | 0.96 | 0.007 |
| main_transparent_pass_3d | 0.013 | 0.013 | 0.026 | 0.96 | 0.078 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.96 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.96 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.96 | 0.002 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 26.759 | 0.000 | 12319.27 | 153008.5 |
| render/thread/ms | 10.290 | 13.459 | 28.61 | 58839.4 |
| render/busy/ms | 9.708 | 12.871 | 28.00 | 55512.8 |
| main/busy/ms | 8.737 | 11.083 | 29.60 | 49959.8 |
| render/graph/ms | 5.504 | 7.506 | 16.05 | 31471.7 |
| bigspace/total/ms | 0.532 | 1.355 | 7.80 | 3040.6 |
| render/present/ms | 0.489 | 0.618 | 16.48 | 2794.4 |
| pipelines/compile_ms | 0.485 | 0.000 | 163.59 | 2774.0 |
| pipelines/front_ms | 0.469 | 0.000 | 157.93 | 2682.9 |
| terrain/update/ms | 0.300 | 0.389 | 2.17 | 1716.0 |
| controller/feed/ms | 0.294 | 0.393 | 0.71 | 1681.9 |
| terrain/select/ms | 0.266 | 0.345 | 0.70 | 1522.5 |
| stream/drive/ms | 0.260 | 0.377 | 21.03 | 1484.0 |
| bigspace/low_precision/ms | 0.215 | 0.559 | 2.43 | 1231.2 |
| stream/tick/ms | 0.203 | 0.283 | 0.77 | 1160.4 |
| far/select/ms | 0.201 | 0.286 | 0.67 | 1150.4 |
| anim/evaluate_cpu/ms | 0.169 | 0.213 | 0.52 | 966.3 |
| bigspace/high_precision/ms | 0.160 | 0.585 | 2.24 | 912.3 |
| anim/evaluate/ms | 0.157 | 0.291 | 2.18 | 895.5 |
| controller/move/ms | 0.136 | 0.241 | 0.61 | 777.6 |
| render/acquire/ms | 0.093 | 0.126 | 0.39 | 532.1 |
| physics/step/ms | 0.051 | 0.091 | 0.56 | 293.5 |
| physics/insert/ms | 0.042 | 0.409 | 0.81 | 241.4 |
| pipelines/driver_ms | 0.016 | 0.000 | 5.87 | 91.1 |
| anim/apply/ms | 0.012 | 0.023 | 0.32 | 69.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.17 | 63.3 |
| controller/avoid/ms | 0.011 | 0.014 | 0.28 | 61.5 |
| script/update/ms | 0.008 | 0.011 | 0.29 | 43.3 |
| stream/despawn/ms | 0.005 | 0.000 | 1.12 | 29.6 |
| script/input/ms | 0.005 | 0.007 | 0.07 | 29.5 |
| script/ui/ms | 0.004 | 0.006 | 0.04 | 25.7 |
| stream/spawn/ms | 0.004 | 0.000 | 1.07 | 25.5 |
| controller/write/ms | 0.004 | 0.005 | 0.03 | 20.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.66 | 15.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 5.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 4.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 7 | 0 |
| anim/a2/count | count | 1 | 10 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 11 |
| crowd/c0/joints/count | count | 1281 | 2262 | 492 |
| crowd/c0/meshes/count | count | 349 | 646 | 113 |
| crowd/c1/count | count | 147 | 155 | 150 |
| crowd/c1/joints/count | count | 5531 | 5949 | 5704 |
| crowd/c1/meshes/count | count | 1175 | 1281 | 1228 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 222 |
| crowd/joints_extracted/count | count | 4262 | 9933 | 6124 |
| crowd/joints_uploaded/count | count | 4248 | 6643 | 6124 |
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
| frame/work/ms | ms | 9.46 | 29.60 | 8.50 |
| gpu/timed/ms | ms | 7.86 | 11.49 | 8.50 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 198 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 796 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 198 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 796 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1288 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3360 | 5156 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1288 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3360 | 5156 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 58 | 304 | 42 |
| phases/shadow/bins | count | 3772 | 5543 | 2226 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9434 | 12.7 k | 5693 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 138 | 331 | 42 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 290 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 195 | 200 | 200 |
| pipelines/waiting | count | 0 | 38 | 0 |
| preprocess/bind_groups | count | 11 | 777 | 6 |
| preprocess/bind_groups_created | count | 11 | 777 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 232 | 347 | 221 |
| process/cpu_usage | % | 25.40 | 42.31 | 25.48 |
| process/mem_usage | GiB | 5.15 | 5.32 | 5.32 |
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
| started_load_count |  | 4752 | 4843 | 4843 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 440 | 535 | 402 |
| statics/materials_bound | count | 440 | 535 | 402 |
| statics/materials_unclassed | count | 6 | 8 | 6 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.34 | 52.00 | 29.71 |
| system/mem_usage | % | 59.01 | 59.67 | 59.59 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2808 | 2919 | 2919 |
| textures/images | count | 2585 | 2638 | 2638 |
| views/all/batch_sets | count | 115 | 356 | 102 |
| views/all/bins | count | 6446 | 8475 | 5706 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 17.8 k | 25.6 k | 13.3 k |
| views/all/pipelines | count | 192 | 387 | 98 |
| views/all/pipelines_distinct | count | 84 | 95 | 80 |
| views/all/work_items | count | 17.2 k | 24.1 k | 13.6 k |
| views/cascade/batch_sets | count | 52 | 69 | 42 |
| views/cascade/bins | count | 2619 | 3201 | 2226 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8223 | 10.1 k | 5693 |
| views/cascade/pipelines | count | 52 | 69 | 42 |
| views/cascade/work_items | count | 8223 | 10.1 k | 5693 |
| views/lamp_face/batch_sets | count | 0 | 247 | 0 |
| views/lamp_face/bins | count | 1092 | 2623 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1237 | 2950 | 0 |
| views/lamp_face/pipelines | count | 84 | 274 | 0 |
| views/lamp_face/work_items | count | 0 | 2658 | 0 |
| views/main/batch_sets | count | 58 | 62 | 60 |
| views/main/bins | count | 2882 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8316 | 13.1 k | 7592 |
| views/main/pipelines | count | 56 | 58 | 56 |
| views/main/work_items | count | 8476 | 13.3 k | 7900 |
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
| anim/bones_written/count | count | 60.32 | 310 | 344.9 k |
| anim/events/count | count | 0.09 | 5 | 522 |
| assets/Image/added | count | 0.10 | 33 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 8 | 46 |
| assets/Mesh/added | count | 0.71 | 152 | 4075 |
| assets/Mesh/removed | count | 0.85 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.81 | 18 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.91 | 1495 | 5214 |
| physics/inserts | count | 0.44 | 110 | 2532 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 145 | 2335 |
| physics/steps | count | 0.69 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 16 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 5920 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1298 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.7 ms; slowest 29, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2521.6 | main/busy/ms 2496, surface/advance_cpu_ms 70.51, surface/cell_upload_ms 0.97, stream/drive/ms 0.29, stream/tick/ms 0.09, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.02, anim/evaluate/ms 0.01, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.40 | 881.4 | main/busy/ms 2531, render/thread/ms 849, render/busy/ms 726, render/acquire/ms 121, surface/advance_cpu_ms 70.51, render/graph/ms 5.86, render/present/ms 2.14, stream/drive/ms 0.98, surface/cell_upload_ms 0.97, stream/tick/ms 0.51, terrain/update/ms 0.46, terrain/select/ms 0.32 |
| warm-up | 3.62 | 81.5 | pipelines/compile_ms 203, pipelines/front_ms 189, pipelines/wait_ms 187, main/busy/ms 59.98, render/thread/ms 50.07, render/busy/ms 48.37, pipelines/driver_ms 14.54, render/graph/ms 3.83, stream/drive/ms 2.53, stream/tick/ms 1.55, render/present/ms 1.42, terrain/update/ms 0.87 |
| warm-up | 3.78 | 48.3 | render/thread/ms 39.58, render/busy/ms 38.30, main/busy/ms 37.42, stream/drive/ms 25.73, render/graph/ms 2.84, render/present/ms 1.06, stream/tick/ms 0.83, terrain/update/ms 0.66, terrain/select/ms 0.55, stream/spawn/ms 0.42, bigspace/total/ms 0.24, render/acquire/ms 0.22 |
| warm-up | 3.85 | 62.3 | pipelines/wait_ms 932, pipelines/compile_ms 215, pipelines/front_ms 207, render/thread/ms 42.43, render/busy/ms 40.70, main/busy/ms 37.06, stream/drive/ms 26.66, pipelines/driver_ms 8.54, render/graph/ms 4.91, render/present/ms 1.52, stream/spawn/ms 1.06, stream/tick/ms 0.72 |
| warm-up | 3.94 | 81.6 | pipelines/wait_ms 1715, pipelines/compile_ms 89.18, pipelines/front_ms 83.53, render/thread/ms 63.70, render/busy/ms 61.21, main/busy/ms 33.65, stream/drive/ms 12.91, render/graph/ms 7.10, pipelines/driver_ms 5.64, stream/spawn/ms 3.27, render/present/ms 2.23, stream/tick/ms 2.21 |
| warm-up | 4.00 | 57.0 | pipelines/wait_ms 1319, render/thread/ms 92.09, render/busy/ms 89.74, pipelines/compile_ms 49.58, pipelines/front_ms 47.03, main/busy/ms 30.67, render/graph/ms 7.77, stream/drive/ms 6.83, stream/spawn/ms 2.87, pipelines/driver_ms 2.55, stream/tick/ms 2.37, render/present/ms 2.02 |
| warm-up | 4.10 | 106.1 | pipelines/wait_ms 3957, pipelines/compile_ms 216, pipelines/front_ms 209, render/thread/ms 104, render/busy/ms 102, main/busy/ms 26.72, pipelines/driver_ms 7.04, stream/drive/ms 4.13, render/graph/ms 4.00, stream/tick/ms 1.78, terrain/update/ms 1.45, terrain/select/ms 1.15 |
| warm-up | 4.19 | 49.2 | pipelines/wait_ms 1876, main/busy/ms 52.72, render/thread/ms 31.47, stream/drive/ms 30.48, render/busy/ms 30.03, pipelines/compile_ms 20.75, pipelines/front_ms 19.65, render/graph/ms 3.29, stream/spawn/ms 1.37, render/present/ms 1.23, pipelines/driver_ms 1.10, stream/tick/ms 0.89 |
| warm-up | 4.25 | 34.5 | pipelines/wait_ms 2260, pipelines/compile_ms 59.91, pipelines/front_ms 57.66, render/thread/ms 36.88, render/busy/ms 35.02, main/busy/ms 34.86, stream/drive/ms 22.27, render/graph/ms 8.25, pipelines/driver_ms 2.24, render/present/ms 1.66, terrain/update/ms 1.48, terrain/select/ms 1.32 |
| warm-up | 4.31 | 36.8 | main/busy/ms 42.04, render/thread/ms 35.60, render/busy/ms 34.30, stream/drive/ms 25.90, render/graph/ms 11.81, stream/spawn/ms 1.97, render/present/ms 1.11, stream/tick/ms 0.77, terrain/update/ms 0.65, terrain/select/ms 0.53, bigspace/total/ms 0.26, render/acquire/ms 0.18 |
| warm-up | 4.41 | 83.5 | pipelines/wait_ms 547, pipelines/compile_ms 129, pipelines/front_ms 121, main/busy/ms 82.41, stream/drive/ms 63.09, render/thread/ms 26.43, render/busy/ms 25.31, pipelines/driver_ms 8.34, render/graph/ms 3.93, stream/spawn/ms 2.20, render/present/ms 0.91, terrain/update/ms 0.75 |
| warm-up | 4.46 | 32.1 | main/busy/ms 38.30, render/thread/ms 37.32, render/busy/ms 35.70, stream/drive/ms 25.81, render/graph/ms 7.05, stream/spawn/ms 2.02, render/present/ms 1.41, stream/tick/ms 0.92, terrain/update/ms 0.57, stream/activate/ms 0.51, terrain/select/ms 0.48, physics/insert/ms 0.24 |
| warm-up | 4.50 | 29.7 | pipelines/wait_ms 5893, pipelines/compile_ms 185, pipelines/front_ms 167, main/busy/ms 35.56, render/thread/ms 23.87, render/busy/ms 22.48, stream/drive/ms 20.60, pipelines/driver_ms 18.28, render/graph/ms 4.36, stream/activate/ms 1.15, render/present/ms 1.08, stream/spawn/ms 0.84 |
| warm-up | 4.61 | 30.8 | pipelines/wait_ms 9529, pipelines/compile_ms 128, pipelines/front_ms 113, render/thread/ms 31.83, render/busy/ms 30.42, main/busy/ms 24.96, pipelines/driver_ms 15.19, stream/drive/ms 5.47, render/graph/ms 5.06, stream/spawn/ms 3.00, stream/tick/ms 1.43, render/present/ms 1.19 |
| warm-up | 4.64 | 34.7 | pipelines/wait_ms 9982, pipelines/compile_ms 71.03, pipelines/front_ms 64.64, render/thread/ms 44.15, render/busy/ms 42.83, main/busy/ms 33.40, stream/drive/ms 7.48, pipelines/driver_ms 6.39, render/graph/ms 5.46, stream/spawn/ms 1.89, stream/tick/ms 1.26, render/present/ms 1.04 |
| warm-up | 4.70 | 35.9 | main/busy/ms 48.72, render/thread/ms 45.86, render/busy/ms 44.33, terrain/update/ms 6.40, render/graph/ms 5.83, stream/drive/ms 4.64, stream/tick/ms 1.73, stream/spawn/ms 1.49, render/present/ms 1.19, bigspace/total/ms 0.68, terrain/select/ms 0.62, render/acquire/ms 0.34 |
| warm-up | 4.81 | 57.3 | pipelines/wait_ms 5024, pipelines/compile_ms 171, pipelines/front_ms 167, render/thread/ms 56.99, render/busy/ms 55.16, main/busy/ms 40.11, stream/drive/ms 10.88, render/graph/ms 5.57, pipelines/driver_ms 4.04, render/present/ms 1.57, stream/tick/ms 1.04, terrain/update/ms 0.88 |
| warm-up | 4.87 | 36.2 | pipelines/wait_ms 3857, render/thread/ms 46.10, render/busy/ms 44.61, main/busy/ms 34.62, pipelines/compile_ms 22.11, pipelines/front_ms 20.46, render/graph/ms 5.64, stream/drive/ms 1.79, pipelines/driver_ms 1.65, render/present/ms 1.18, physics/insert/ms 0.98, stream/tick/ms 0.73 |
| warm-up | 5.20 | 34.0 | pipelines/wait_ms 5956, main/busy/ms 36.23, pipelines/compile_ms 28.91, pipelines/front_ms 27.03, render/thread/ms 24.39, render/busy/ms 22.92, stream/drive/ms 8.44, render/graph/ms 5.29, pipelines/driver_ms 1.88, physics/insert/ms 1.23, stream/tick/ms 1.16, render/present/ms 1.15 |
| warm-up | 5.36 | 29.3 | render/thread/ms 37.00, render/busy/ms 35.63, main/busy/ms 22.78, render/graph/ms 4.62, stream/drive/ms 3.95, stream/spawn/ms 2.08, render/present/ms 1.09, stream/tick/ms 0.96, terrain/update/ms 0.85, terrain/select/ms 0.73, bigspace/total/ms 0.63, physics/insert/ms 0.36 |
| warm-up | 5.75 | 28.0 | render/thread/ms 44.97, render/busy/ms 43.07, main/busy/ms 41.97, stream/drive/ms 18.77, render/graph/ms 7.44, stream/spawn/ms 2.05, stream/tick/ms 1.69, render/present/ms 1.57, terrain/update/ms 0.83, bigspace/total/ms 0.76, terrain/select/ms 0.72, bigspace/high_precision/ms 0.40 |
| warm-up | 5.79 | 43.2 | pipelines/wait_ms 5260, pipelines/compile_ms 175, pipelines/front_ms 169, render/thread/ms 51.13, render/busy/ms 49.73, main/busy/ms 39.12, stream/drive/ms 18.69, render/graph/ms 6.14, pipelines/driver_ms 5.16, stream/spawn/ms 2.17, stream/tick/ms 1.21, render/present/ms 1.19 |
| warm-up | 6.79 | 38.2 | pipelines/wait_ms 5180, main/busy/ms 44.69, render/thread/ms 34.64, render/busy/ms 33.43, pipelines/compile_ms 24.15, pipelines/front_ms 22.74, render/graph/ms 8.87, pipelines/driver_ms 1.41, stream/drive/ms 1.14, physics/insert/ms 1.08, render/present/ms 1.00, bigspace/total/ms 0.96 |
| warm-up | 7.09 | 34.0 | pipelines/wait_ms 2326, main/busy/ms 51.25, render/thread/ms 35.77, render/busy/ms 34.35, render/graph/ms 10.03, pipelines/compile_ms 8.83, pipelines/front_ms 8.34, terrain/update/ms 2.58, stream/drive/ms 1.58, render/present/ms 1.16, physics/insert/ms 1.07, bigspace/total/ms 0.94 |
| measure | 0.79 | 30.2 | main/busy/ms 34.20, render/thread/ms 25.84, render/busy/ms 24.35, stream/drive/ms 12.73, render/graph/ms 10.25, render/present/ms 1.27, physics/insert/ms 0.93, bigspace/total/ms 0.90, controller/feed/ms 0.58, terrain/update/ms 0.55, far/select/ms 0.51, terrain/select/ms 0.50 |
| measure | 18.19 | 32.2 | main/busy/ms 38.88, render/thread/ms 23.69, render/busy/ms 22.49, stream/drive/ms 21.26, render/graph/ms 12.29, render/present/ms 1.02, controller/feed/ms 0.91, bigspace/total/ms 0.84, terrain/update/ms 0.55, terrain/select/ms 0.50, far/select/ms 0.49, stream/tick/ms 0.35 |
| measure | 19.28 | 30.8 | render/thread/ms 42.03, render/busy/ms 38.53, main/busy/ms 26.69, render/graph/ms 21.99, bigspace/total/ms 8.63, render/present/ms 3.28, controller/feed/ms 0.80, bigspace/low_precision/ms 0.78, terrain/update/ms 0.58, terrain/select/ms 0.51, stream/drive/ms 0.50, far/select/ms 0.47 |
| measure | 39.07 | 27.9 | render/thread/ms 36.65, main/busy/ms 19.77, render/busy/ms 19.51, render/present/ms 16.92, render/graph/ms 11.40, bigspace/total/ms 3.45, bigspace/low_precision/ms 2.33, bigspace/high_precision/ms 0.79, controller/feed/ms 0.72, terrain/update/ms 0.57, terrain/select/ms 0.52, stream/drive/ms 0.50 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_fog\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels-2 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 14%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **80%** (min 54%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.3 W · 76 °C (max 82) · VRAM 6270 MiB · throttled 100% of samples
