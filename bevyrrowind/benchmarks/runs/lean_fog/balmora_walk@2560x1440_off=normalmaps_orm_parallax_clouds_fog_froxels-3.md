# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: clouds,fog,froxels,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5812 frames over 65.6 s · **median 10.88 ms (92 fps)** · mean 11.30 · p95 14.42 · p99 16.69 (1% low 60 fps) · worst 28.19 ms · 74 spikes over 1.5× median · 24.7 stutters/min (work 25.6/min)
- warm-up: 340 frames over 8.6 s, worst 2617.5 ms
- hitches (> 27.2 ms): **1** measured, 33 in warm-up
- timed GPU passes: 6.85 ms of a 11.30 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.88 ms, limit 11 ms
- **FAIL** frame_ms: 27 stutters (> 1.5x moving median of 60) in 1.09 min = 24.68/min, limit 1/min
- pass worst frame 28.19 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.128 | 2.198 | 2.543 | 0.97 | 0.177 |
| main_opaque_pass_3d | 1.499 | 1.548 | 2.181 | 0.97 | 0.055 |
| early prepass | 0.488 | 0.504 | 0.748 | 0.97 | 0.092 |
| bloom | 0.327 | 0.337 | 0.491 | 0.97 | 0.066 |
| atmosphere_luts | 0.316 | 0.327 | 0.458 | 0.97 | 0.003 |
| water_velocity | 0.291 | 0.301 | 0.520 | 0.97 | 0.002 |
| uniform_allocation | 0.219 | 0.226 | 0.240 | 0.97 | 0.040 |
| water_sspr | 0.210 | 0.217 | 0.209 | 0.97 | 0.002 |
| ssao | 0.199 | 0.205 | 0.211 | 0.97 | 0.008 |
| bin_unpacking | 0.176 | 0.182 | 0.195 | 0.97 | 0.028 |
| auxiliary_views | 0.168 | 0.174 | 0.706 | 0.97 | 0.158 |
| water_fft | 0.137 | 0.141 | 0.140 | 0.97 | 0.003 |
| upscaling | 0.128 | 0.132 | 0.145 | 0.97 | 0.004 |
| tonemapping | 0.090 | 0.093 | 0.094 | 0.97 | 0.003 |
| main_transmissive_pass_3d | 0.088 | 0.090 | 0.169 | 0.97 | 0.009 |
| clustering | 0.065 | 0.067 | 0.070 | 0.97 | 0.019 |
| render_sky | 0.053 | 0.054 | 0.054 | 0.97 | 0.002 |
| lightprobe_irradiance_map | 0.045 | 0.441 | 0.755 | 0.10 | 0.001 |
| early_mesh_preprocessing | 0.043 | 0.045 | 0.049 | 0.97 | 0.005 |
| surface_trail | 0.041 | 0.059 | 0.057 | 0.71 | 0.002 |
| water_snapshot | 0.025 | 0.026 | 0.035 | 0.97 | 0.002 |
| lightprobe_radiance_map | 0.024 | 0.235 | 0.238 | 0.10 | 0.004 |
| water_mips | 0.024 | 0.024 | 0.024 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.023 | 0.97 | 0.006 |
| main_transparent_pass_3d | 0.013 | 0.013 | 0.027 | 0.97 | 0.077 |
| atmosphere_environment | 0.011 | 0.011 | 0.012 | 0.97 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.97 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.97 | 0.002 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.10 | 0.005 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 23.750 | 0.000 | 11898.17 | 138034.9 |
| render/thread/ms | 10.122 | 13.063 | 26.57 | 58829.2 |
| render/busy/ms | 9.546 | 12.471 | 25.03 | 55480.1 |
| main/busy/ms | 8.646 | 10.868 | 24.83 | 50253.1 |
| render/graph/ms | 5.452 | 7.304 | 14.67 | 31687.3 |
| bigspace/total/ms | 0.553 | 1.544 | 3.21 | 3211.6 |
| render/present/ms | 0.484 | 0.595 | 15.22 | 2812.1 |
| pipelines/compile_ms | 0.426 | 0.000 | 153.95 | 2478.5 |
| pipelines/front_ms | 0.413 | 0.000 | 148.65 | 2398.0 |
| terrain/update/ms | 0.300 | 0.389 | 1.79 | 1743.0 |
| controller/feed/ms | 0.274 | 0.377 | 0.66 | 1592.0 |
| terrain/select/ms | 0.267 | 0.344 | 0.64 | 1550.0 |
| stream/drive/ms | 0.252 | 0.352 | 16.94 | 1465.6 |
| bigspace/low_precision/ms | 0.232 | 0.652 | 2.86 | 1346.4 |
| far/select/ms | 0.197 | 0.278 | 0.59 | 1144.5 |
| stream/tick/ms | 0.197 | 0.270 | 0.81 | 1144.2 |
| bigspace/high_precision/ms | 0.166 | 0.605 | 2.61 | 965.6 |
| controller/move/ms | 0.165 | 0.409 | 0.88 | 957.4 |
| anim/evaluate_cpu/ms | 0.162 | 0.211 | 0.49 | 943.4 |
| anim/evaluate/ms | 0.138 | 0.274 | 1.95 | 804.1 |
| render/acquire/ms | 0.092 | 0.123 | 0.46 | 537.0 |
| physics/step/ms | 0.050 | 0.090 | 0.92 | 291.5 |
| physics/insert/ms | 0.038 | 0.384 | 0.95 | 220.9 |
| pipelines/driver_ms | 0.014 | 0.000 | 5.29 | 80.5 |
| anim/apply/ms | 0.012 | 0.021 | 0.30 | 69.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.27 | 64.1 |
| controller/avoid/ms | 0.010 | 0.013 | 0.19 | 58.8 |
| script/update/ms | 0.007 | 0.011 | 0.14 | 43.5 |
| script/input/ms | 0.005 | 0.007 | 0.15 | 30.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.04 | 30.3 |
| script/ui/ms | 0.005 | 0.007 | 0.07 | 27.9 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 26.7 |
| controller/write/ms | 0.003 | 0.004 | 0.16 | 20.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.60 | 15.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 5.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 9 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 42 | 13 |
| crowd/c0/joints/count | count | 1272 | 2218 | 592 |
| crowd/c0/meshes/count | count | 353 | 633 | 173 |
| crowd/c1/count | count | 146 | 155 | 147 |
| crowd/c1/joints/count | count | 5510 | 5937 | 5572 |
| crowd/c1/meshes/count | count | 1157 | 1281 | 1160 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1822 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4258 | 8318 | 6179 |
| crowd/joints_uploaded/count | count | 4242 | 6643 | 6179 |
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
| frame/work/ms | ms | 9.27 | 25.03 | 9.17 |
| gpu/timed/ms | ms | 7.82 | 9.55 | 7.98 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 602.13 M | 603.31 M | 603.31 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 809 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 809 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1292 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3400 | 5132 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1292 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3400 | 5132 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 53 | 299 | 42 |
| phases/shadow/bins | count | 3788 | 5510 | 2234 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9617 | 12.6 k | 5731 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 134 | 323 | 42 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 823 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 200 | 205 | 205 |
| pipelines/waiting | count | 0 | 33 | 0 |
| preprocess/bind_groups | count | 11 | 767 | 6 |
| preprocess/bind_groups_created | count | 11 | 767 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 231 | 373 | 221 |
| process/cpu_usage | % | 25 | 36.32 | 19.23 |
| process/mem_usage | GiB | 5.12 | 5.30 | 5.29 |
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
| started_load_count |  | 4753 | 4846 | 4846 |
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
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.40 | 59.66 | 29.15 |
| system/mem_usage | % | 58.84 | 59.55 | 59.28 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2808 | 2919 | 2919 |
| textures/images | count | 2585 | 2638 | 2638 |
| views/all/batch_sets | count | 111 | 351 | 102 |
| views/all/bins | count | 6506 | 8242 | 5714 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.3 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 188 | 369 | 98 |
| views/all/pipelines_distinct | count | 84 | 96 | 80 |
| views/all/work_items | count | 17.4 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 52 | 71 | 42 |
| views/cascade/bins | count | 2626 | 3118 | 2234 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8237 | 10.2 k | 5731 |
| views/cascade/pipelines | count | 52 | 71 | 42 |
| views/cascade/work_items | count | 8237 | 10.2 k | 5731 |
| views/lamp_face/batch_sets | count | 0 | 242 | 0 |
| views/lamp_face/bins | count | 1058 | 2606 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1185 | 2936 | 0 |
| views/lamp_face/pipelines | count | 80 | 254 | 0 |
| views/lamp_face/work_items | count | 0 | 2624 | 0 |
| views/main/batch_sets | count | 60 | 62 | 60 |
| views/main/bins | count | 2886 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8418 | 13.0 k | 7592 |
| views/main/pipelines | count | 56 | 58 | 56 |
| views/main/work_items | count | 8579 | 13.3 k | 7900 |
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
| anim/bones_written/count | count | 64.28 | 322 | 373.6 k |
| anim/events/count | count | 0.08 | 3 | 484 |
| assets/Image/added | count | 0.10 | 37 | 598 |
| assets/Image/modified | count | 0.00 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 8 | 46 |
| assets/Mesh/added | count | 0.71 | 152 | 4101 |
| assets/Mesh/removed | count | 0.84 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 471 |
| assets/StandardMaterial/modified | count | 3.75 | 29 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.17 | 1562 | 6774 |
| physics/inserts | count | 0.53 | 110 | 3067 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 161 | 2335 |
| physics/steps | count | 0.68 | 2 | 3938 |
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
| stream/upload/bytes | bytes | 5825 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1277 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.2 ms; slowest 34, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.62 | 2617.5 | main/busy/ms 2593, surface/advance_cpu_ms 71.61, surface/cell_upload_ms 1.06, stream/drive/ms 0.35, bigspace/total/ms 0.13, stream/tick/ms 0.12, bigspace/high_precision/ms 0.10, weather/eval/ms 0.03, script/update/ms 0.03, script/input/ms 0.01, anim/evaluate/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.30 | 680.9 | main/busy/ms 2633, render/thread/ms 641, render/busy/ms 481, render/acquire/ms 158, surface/advance_cpu_ms 71.61, render/graph/ms 4.94, render/present/ms 2.13, stream/drive/ms 1.18, surface/cell_upload_ms 1.06, stream/tick/ms 0.58, terrain/update/ms 0.38, bigspace/total/ms 0.31 |
| warm-up | 3.65 | 354.6 | pipelines/wait_ms 8675, render/thread/ms 653, render/busy/ms 492, pipelines/compile_ms 390, pipelines/front_ms 363, render/acquire/ms 158, main/busy/ms 57.58, pipelines/driver_ms 26.94, terrain/update/ms 21.88, render/graph/ms 8.80, render/present/ms 3.06, stream/drive/ms 1.41 |
| warm-up | 3.88 | 72.8 | main/busy/ms 56.92, render/thread/ms 51.04, render/busy/ms 48.57, render/graph/ms 4.81, render/present/ms 1.90, stream/drive/ms 1.29, stream/tick/ms 1.04, terrain/update/ms 0.94, terrain/select/ms 0.81, render/acquire/ms 0.57, bigspace/total/ms 0.35, bigspace/low_precision/ms 0.13 |
| warm-up | 4.06 | 59.3 | pipelines/wait_ms 2454, pipelines/compile_ms 89.54, pipelines/front_ms 81.86, main/busy/ms 51.76, render/thread/ms 43.73, render/busy/ms 42.19, stream/drive/ms 37.45, pipelines/driver_ms 7.68, render/graph/ms 4.82, render/present/ms 1.36, stream/tick/ms 1.15, terrain/update/ms 0.98 |
| warm-up | 4.16 | 43.1 | main/busy/ms 40.50, render/thread/ms 37.26, render/busy/ms 35.68, stream/drive/ms 21.00, render/graph/ms 5.13, stream/spawn/ms 1.46, stream/tick/ms 1.36, render/present/ms 1.29, terrain/update/ms 0.70, terrain/select/ms 0.56, render/acquire/ms 0.30, bigspace/total/ms 0.28 |
| warm-up | 4.29 | 108.3 | pipelines/wait_ms 679, pipelines/compile_ms 170, pipelines/front_ms 161, render/thread/ms 64.94, main/busy/ms 45.52, render/busy/ms 39.11, stream/drive/ms 28.27, render/present/ms 25.63, pipelines/driver_ms 8.52, render/graph/ms 2.88, stream/spawn/ms 1.92, stream/tick/ms 1.15 |
| warm-up | 4.32 | 35.9 | pipelines/wait_ms 679, pipelines/compile_ms 170, pipelines/front_ms 161, render/thread/ms 80.66, render/busy/ms 54.68, render/present/ms 25.68, main/busy/ms 24.42, pipelines/driver_ms 8.52, stream/drive/ms 4.77, render/graph/ms 3.82, stream/spawn/ms 1.53, stream/tick/ms 1.22 |
| warm-up | 4.35 | 27.6 | render/thread/ms 47.46, render/busy/ms 46.04, main/busy/ms 20.83, render/graph/ms 10.06, stream/drive/ms 3.98, render/present/ms 1.17, terrain/update/ms 1.12, stream/activate/ms 1.02, stream/tick/ms 0.98, terrain/select/ms 0.92, stream/spawn/ms 0.70, physics/insert/ms 0.35 |
| warm-up | 4.39 | 36.3 | pipelines/wait_ms 1265, pipelines/compile_ms 173, pipelines/front_ms 161, render/thread/ms 52.28, render/busy/ms 50.63, main/busy/ms 24.22, render/graph/ms 12.87, pipelines/driver_ms 11.58, stream/drive/ms 2.52, render/present/ms 1.44, terrain/update/ms 0.95, stream/tick/ms 0.83 |
| warm-up | 4.42 | 32.5 | pipelines/wait_ms 1265, pipelines/compile_ms 173, pipelines/front_ms 161, render/thread/ms 60.69, render/busy/ms 58.29, main/busy/ms 33.60, render/graph/ms 20.96, pipelines/driver_ms 11.58, stream/drive/ms 3.88, render/present/ms 2.12, stream/spawn/ms 1.88, terrain/update/ms 0.85 |
| warm-up | 4.50 | 53.5 | main/busy/ms 77.56, stream/drive/ms 58.15, render/thread/ms 51.30, render/busy/ms 49.60, terrain/update/ms 6.87, render/graph/ms 4.92, stream/spawn/ms 1.78, stream/tick/ms 1.57, render/present/ms 1.43, terrain/select/ms 0.72, bigspace/total/ms 0.35, render/acquire/ms 0.27 |
| warm-up | 4.56 | 63.4 | pipelines/wait_ms 1335, pipelines/compile_ms 158, pipelines/front_ms 155, main/busy/ms 82.64, stream/drive/ms 57.85, render/thread/ms 50.99, render/busy/ms 49.62, render/graph/ms 3.85, pipelines/driver_ms 3.41, stream/spawn/ms 1.66, stream/tick/ms 1.25, render/present/ms 1.17 |
| warm-up | 4.65 | 71.8 | main/busy/ms 62.66, stream/drive/ms 46.43, render/thread/ms 35.06, render/busy/ms 33.96, render/graph/ms 3.83, stream/spawn/ms 1.73, render/present/ms 0.90, stream/tick/ms 0.79, terrain/update/ms 0.62, terrain/select/ms 0.52, stream/activate/ms 0.47, bigspace/total/ms 0.29 |
| warm-up | 4.72 | 51.6 | pipelines/wait_ms 2445, pipelines/compile_ms 173, pipelines/front_ms 158, main/busy/ms 50.53, render/thread/ms 32.38, stream/drive/ms 31.81, render/busy/ms 31.00, pipelines/driver_ms 14.51, render/graph/ms 4.35, render/present/ms 1.13, stream/activate/ms 1.13, stream/spawn/ms 0.98 |
| warm-up | 4.78 | 50.0 | pipelines/wait_ms 5569, pipelines/compile_ms 93.14, pipelines/front_ms 88.32, main/busy/ms 55.79, render/thread/ms 29.19, stream/drive/ms 27.65, render/busy/ms 27.37, render/graph/ms 6.19, pipelines/driver_ms 4.82, render/present/ms 1.54, physics/insert/ms 1.11, stream/spawn/ms 1.04 |
| warm-up | 4.88 | 39.6 | pipelines/wait_ms 2775, render/thread/ms 50.32, render/busy/ms 48.30, pipelines/compile_ms 42.32, pipelines/front_ms 40.04, main/busy/ms 25.95, stream/drive/ms 10.47, render/graph/ms 5.21, pipelines/driver_ms 2.28, render/present/ms 1.77, stream/tick/ms 0.93, bigspace/total/ms 0.90 |
| warm-up | 5.14 | 28.7 | pipelines/wait_ms 3585, pipelines/compile_ms 166, pipelines/front_ms 159, main/busy/ms 33.82, render/thread/ms 31.71, render/busy/ms 29.84, stream/drive/ms 12.77, stream/spawn/ms 10.33, pipelines/driver_ms 6.65, render/graph/ms 6.51, render/present/ms 1.61, stream/tick/ms 1.28 |
| warm-up | 5.55 | 29.4 | pipelines/wait_ms 1170, render/thread/ms 37.69, render/busy/ms 36.25, main/busy/ms 23.43, pipelines/compile_ms 13.74, pipelines/front_ms 13.37, render/graph/ms 4.69, stream/drive/ms 4.07, stream/spawn/ms 2.05, stream/tick/ms 1.27, render/present/ms 1.23, terrain/update/ms 0.88 |
| warm-up | 5.79 | 32.4 | render/thread/ms 44.09, main/busy/ms 42.99, render/busy/ms 42.38, render/graph/ms 6.68, stream/drive/ms 4.07, stream/spawn/ms 1.95, render/present/ms 1.48, physics/insert/ms 1.45, stream/tick/ms 1.12, terrain/update/ms 1.06, bigspace/total/ms 0.97, terrain/select/ms 0.89 |
| warm-up | 5.83 | 38.1 | render/thread/ms 55.12, render/busy/ms 53.82, main/busy/ms 38.25, render/graph/ms 5.86, stream/drive/ms 3.94, physics/insert/ms 1.83, stream/spawn/ms 1.73, stream/tick/ms 1.17, render/present/ms 1.10, bigspace/total/ms 0.98, terrain/update/ms 0.82, terrain/select/ms 0.68 |
| warm-up | 5.91 | 36.4 | render/thread/ms 54.18, render/busy/ms 52.31, main/busy/ms 28.88, render/graph/ms 7.64, stream/drive/ms 4.89, physics/insert/ms 2.36, render/present/ms 1.55, stream/spawn/ms 1.53, terrain/update/ms 0.72, bigspace/total/ms 0.67, stream/tick/ms 0.65, terrain/select/ms 0.60 |
| warm-up | 5.98 | 48.6 | pipelines/wait_ms 5629, pipelines/compile_ms 190, pipelines/front_ms 176, render/thread/ms 50.56, render/busy/ms 49.30, main/busy/ms 34.68, pipelines/driver_ms 13.75, render/graph/ms 6.03, stream/drive/ms 5.20, stream/spawn/ms 2.67, physics/insert/ms 1.76, stream/tick/ms 1.21 |
| warm-up | 6.02 | 34.3 | render/thread/ms 59.00, render/busy/ms 57.78, main/busy/ms 32.74, render/graph/ms 5.58, stream/drive/ms 4.86, physics/insert/ms 2.67, stream/spawn/ms 2.07, stream/tick/ms 1.20, render/present/ms 1.02, terrain/update/ms 0.94, bigspace/total/ms 0.80, terrain/select/ms 0.76 |
| warm-up | 6.05 | 29.1 | render/thread/ms 53.41, render/busy/ms 52.21, main/busy/ms 43.97, stream/drive/ms 19.96, render/graph/ms 5.97, physics/insert/ms 2.61, stream/spawn/ms 2.15, render/present/ms 0.95, terrain/update/ms 0.93, stream/tick/ms 0.81, terrain/select/ms 0.75, bigspace/total/ms 0.64 |
| warm-up | 6.08 | 36.1 | render/thread/ms 57.12, render/busy/ms 55.92, main/busy/ms 44.00, stream/drive/ms 23.26, render/graph/ms 5.86, stream/spawn/ms 2.15, physics/insert/ms 2.14, render/present/ms 0.97, terrain/update/ms 0.75, stream/tick/ms 0.75, terrain/select/ms 0.62, bigspace/total/ms 0.54 |
| warm-up | 6.13 | 28.5 | pipelines/wait_ms 2191, render/thread/ms 39.62, main/busy/ms 23.31, render/busy/ms 22.79, render/present/ms 16.57, pipelines/compile_ms 13.68, pipelines/driver_ms 8.16, render/graph/ms 7.30, pipelines/front_ms 5.53, stream/drive/ms 3.65, stream/spawn/ms 2.04, physics/insert/ms 1.74 |
| warm-up | 6.43 | 42.7 | main/busy/ms 58.30, render/thread/ms 32.74, render/busy/ms 30.87, render/graph/ms 9.94, physics/insert/ms 1.63, render/present/ms 1.62, stream/drive/ms 1.58, stream/tick/ms 0.66, terrain/update/ms 0.65, bigspace/total/ms 0.63, stream/spawn/ms 0.57, terrain/select/ms 0.55 |
| warm-up | 6.48 | 44.1 | render/thread/ms 47.68, render/busy/ms 45.96, main/busy/ms 40.89, render/graph/ms 9.32, render/present/ms 1.48, stream/drive/ms 1.38, physics/insert/ms 1.36, bigspace/total/ms 0.71, stream/tick/ms 0.64, terrain/update/ms 0.61, terrain/select/ms 0.53, anim/evaluate_cpu/ms 0.43 |
| warm-up | 6.90 | 32.8 | main/busy/ms 47.80, render/thread/ms 42.25, render/busy/ms 40.42, render/graph/ms 15.20, terrain/update/ms 4.29, stream/drive/ms 1.53, render/present/ms 1.46, bigspace/total/ms 1.00, physics/insert/ms 0.97, anim/evaluate_cpu/ms 0.73, stream/tick/ms 0.64, physics/step/ms 0.56 |
| warm-up | 6.94 | 37.0 | main/busy/ms 59.91, render/thread/ms 48.82, render/busy/ms 46.97, render/graph/ms 11.32, stream/drive/ms 1.45, render/present/ms 1.41, physics/insert/ms 1.06, anim/evaluate_cpu/ms 1.05, bigspace/total/ms 1.01, physics/step/ms 0.71, stream/tick/ms 0.68, anim/evaluate/ms 0.66 |
| warm-up | 6.98 | 41.0 | pipelines/wait_ms 6397, pipelines/compile_ms 150, pipelines/front_ms 146, render/thread/ms 47.34, render/busy/ms 46.02, main/busy/ms 41.64, render/graph/ms 6.90, pipelines/driver_ms 4.04, stream/drive/ms 1.63, physics/insert/ms 1.17, render/present/ms 1.05, stream/tick/ms 0.82 |
| warm-up | 7.32 | 38.6 | render/thread/ms 46.80, render/busy/ms 45.73, render/graph/ms 36.38, main/busy/ms 16.32, physics/insert/ms 1.43, stream/drive/ms 1.03, render/present/ms 0.88, terrain/update/ms 0.56, stream/tick/ms 0.50, terrain/select/ms 0.50, bigspace/total/ms 0.49, stream/spawn/ms 0.26 |
| measure | 32.47 | 28.2 | render/thread/ms 37.79, render/busy/ms 22.43, main/busy/ms 21.35, render/present/ms 15.10, render/graph/ms 12.63, bigspace/total/ms 2.40, bigspace/low_precision/ms 1.87, stream/drive/ms 1.77, stream/despawn/ms 1.23, controller/feed/ms 0.74, terrain/update/ms 0.58, terrain/select/ms 0.52 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_fog\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels-3 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **80%** (min 61%) · 2575 MHz (min 2565) · memory 8001 MHz · 77.9 W · 76 °C (max 82) · VRAM 6284 MiB · throttled 100% of samples
