# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4605 frames over 65.6 s · **median 13.87 ms (72 fps)** · mean 14.26 · p95 17.58 · p99 19.86 (1% low 50 fps) · worst 36.45 ms · 31 spikes over 1.5× median · 14.6 stutters/min (work 13.7/min)
- warm-up: 359 frames over 8.4 s, worst 2587.1 ms
- hitches (> 34.7 ms): **1** measured, 17 in warm-up
- timed GPU passes: 7.99 ms of a 14.26 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.87 ms, limit 11 ms
- **FAIL** frame_ms: 16 stutters (> 1.5x moving median of 60) in 1.09 min = 14.62/min, limit 1/min
- **FAIL** worst frame 36.45 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.205 | 2.207 | 2.674 | 1.00 | 0.183 |
| main_opaque_pass_3d | 1.683 | 1.683 | 2.304 | 1.00 | 0.086 |
| early prepass | 0.540 | 0.540 | 0.818 | 1.00 | 0.124 |
| bloom | 0.342 | 0.342 | 0.497 | 1.00 | 0.067 |
| uniform_allocation | 0.330 | 0.330 | 0.626 | 1.00 | 0.063 |
| atmosphere_luts | 0.328 | 0.328 | 0.459 | 1.00 | 0.004 |
| water_velocity | 0.308 | 0.308 | 0.598 | 1.00 | 0.002 |
| bin_unpacking | 0.252 | 0.252 | 0.276 | 1.00 | 0.042 |
| water_sspr | 0.225 | 0.225 | 0.230 | 1.00 | 0.002 |
| ssao | 0.208 | 0.208 | 0.213 | 1.00 | 0.008 |
| atmos_froxel_inject | 0.160 | 0.160 | 0.155 | 1.00 | 0.002 |
| auxiliary_views | 0.145 | 0.145 | 0.657 | 1.00 | 0.135 |
| clouds_trace | 0.144 | 0.144 | 0.236 | 1.00 | 0.001 |
| water_fft | 0.137 | 0.137 | 0.138 | 1.00 | 0.003 |
| upscaling | 0.126 | 0.126 | 0.150 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.111 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.092 | 0.092 | 0.172 | 1.00 | 0.009 |
| tonemapping | 0.092 | 0.092 | 0.092 | 1.00 | 0.004 |
| clustering | 0.069 | 0.069 | 0.070 | 1.00 | 0.021 |
| render_sky | 0.057 | 0.057 | 0.058 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.055 | 0.442 | 0.757 | 0.13 | 0.001 |
| surface_trail | 0.051 | 0.059 | 0.057 | 0.87 | 0.002 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.049 | 0.049 | 0.049 | 1.00 | 0.006 |
| water_snapshot | 0.038 | 0.038 | 0.045 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.004 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.030 | 0.236 | 0.240 | 0.13 | 0.005 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.024 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.031 | 1.00 | 0.110 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.004 |
| night_sky | 0.006 | 0.006 | 0.013 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 47.944 | 0.000 | 19654.13 | 220784.1 |
| render/thread/ms | 13.040 | 16.184 | 34.89 | 60051.2 |
| render/busy/ms | 12.467 | 15.583 | 33.99 | 57410.9 |
| main/busy/ms | 9.607 | 12.569 | 28.55 | 44241.8 |
| render/graph/ms | 7.618 | 9.609 | 20.63 | 35080.9 |
| pipelines/compile_ms | 0.758 | 0.000 | 159.86 | 3491.2 |
| pipelines/front_ms | 0.732 | 0.000 | 153.39 | 3369.7 |
| render/present/ms | 0.482 | 0.580 | 8.59 | 2219.4 |
| bigspace/total/ms | 0.454 | 0.704 | 4.98 | 2091.8 |
| terrain/update/ms | 0.291 | 0.368 | 1.80 | 1341.4 |
| controller/feed/ms | 0.284 | 0.386 | 0.92 | 1309.8 |
| stream/drive/ms | 0.261 | 0.378 | 20.72 | 1201.2 |
| terrain/select/ms | 0.258 | 0.325 | 0.59 | 1188.4 |
| far/select/ms | 0.198 | 0.287 | 0.63 | 911.8 |
| stream/tick/ms | 0.197 | 0.275 | 0.60 | 906.3 |
| bigspace/low_precision/ms | 0.172 | 0.290 | 4.65 | 790.1 |
| anim/evaluate_cpu/ms | 0.156 | 0.196 | 0.46 | 718.1 |
| anim/evaluate/ms | 0.146 | 0.292 | 1.54 | 670.7 |
| controller/move/ms | 0.138 | 0.271 | 0.55 | 636.4 |
| bigspace/high_precision/ms | 0.111 | 0.223 | 3.66 | 509.6 |
| render/acquire/ms | 0.091 | 0.120 | 0.40 | 420.9 |
| physics/step/ms | 0.063 | 0.096 | 0.75 | 288.7 |
| physics/insert/ms | 0.056 | 0.439 | 1.01 | 255.7 |
| pipelines/driver_ms | 0.026 | 0.000 | 6.47 | 121.5 |
| weather/map/ms | 0.024 | 0.034 | 0.25 | 112.4 |
| anim/apply/ms | 0.011 | 0.020 | 0.19 | 51.5 |
| weather/eval/ms | 0.010 | 0.014 | 0.19 | 47.4 |
| controller/avoid/ms | 0.010 | 0.014 | 0.08 | 46.9 |
| script/update/ms | 0.007 | 0.010 | 0.10 | 34.4 |
| stream/despawn/ms | 0.007 | 0.000 | 1.12 | 29.9 |
| stream/spawn/ms | 0.006 | 0.000 | 1.07 | 25.8 |
| script/input/ms | 0.005 | 0.007 | 0.20 | 24.0 |
| script/ui/ms | 0.004 | 0.006 | 0.14 | 19.9 |
| controller/write/ms | 0.003 | 0.004 | 0.03 | 16.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.64 | 14.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 0 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 38 | 16 |
| controller/movers/count | count | 20 | 22 | 16 |
| crowd/c0/count | count | 27 | 42 | 14 |
| crowd/c0/joints/count | count | 1291 | 2218 | 650 |
| crowd/c0/meshes/count | count | 353 | 633 | 185 |
| crowd/c1/count | count | 145 | 156 | 147 |
| crowd/c1/joints/count | count | 5475 | 6011 | 5546 |
| crowd/c1/meshes/count | count | 1160 | 1297 | 1156 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4290 | 8684 | 6179 |
| crowd/joints_uploaded/count | count | 4252 | 6643 | 6179 |
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
| frame/work/ms | ms | 12.14 | 33.99 | 10.10 |
| gpu/timed/ms | ms | 8.67 | 12.20 | 9.11 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 800 | 1416 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 800 | 1416 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 36 | 42 | 35 |
| phases/opaque3d/bins | count | 1286 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3372 | 5130 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 36 | 42 | 35 |
| phases/opaque3d_prepass/bins | count | 1286 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3372 | 5130 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 73 | 337 | 59 |
| phases/shadow/bins | count | 3740 | 5452 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9431 | 12.9 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 141 | 321 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2345 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1094 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 220 | 226 | 226 |
| pipelines/waiting | count | 0 | 52 | 0 |
| preprocess/bind_groups | count | 13 | 919 | 6 |
| preprocess/bind_groups_created | count | 13 | 919 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 312 | 440 | 303 |
| process/cpu_usage | % | 22.71 | 47.62 | 31.98 |
| process/mem_usage | GiB | 4.50 | 4.63 | 4.63 |
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
| started_load_count |  | 4767 | 4858 | 4858 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.82 | 50.50 | 28.95 |
| system/mem_usage | % | 70.48 | 72.53 | 70.76 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 1728 | 1802 | 1802 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 164 | 427 | 143 |
| views/all/bins | count | 6364 | 8336 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 17.9 k | 25.4 k | 13.3 k |
| views/all/pipelines | count | 203 | 387 | 113 |
| views/all/pipelines_distinct | count | 97 | 109 | 91 |
| views/all/work_items | count | 17.0 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 71 | 92 | 59 |
| views/cascade/bins | count | 2586 | 3144 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8130 | 10.2 k | 5663 |
| views/cascade/pipelines | count | 55 | 71 | 47 |
| views/cascade/work_items | count | 8130 | 10.2 k | 5663 |
| views/lamp_face/batch_sets | count | 0 | 258 | 0 |
| views/lamp_face/bins | count | 1059 | 2485 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1192 | 2824 | 0 |
| views/lamp_face/pipelines | count | 83 | 257 | 0 |
| views/lamp_face/work_items | count | 0 | 2578 | 0 |
| views/main/batch_sets | count | 86 | 100 | 84 |
| views/main/bins | count | 2870 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8340 | 13.0 k | 7592 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8510 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 46.25 | 207 | 213.0 k |
| anim/events/count | count | 0.12 | 3 | 547 |
| assets/Image/added | count | 0.14 | 43 | 633 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.91 | 152 | 4186 |
| assets/Mesh/removed | count | 1.06 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 515 |
| assets/StandardMaterial/modified | count | 4.70 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 48 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.70 | 1492 | 7826 |
| physics/inserts | count | 0.73 | 180 | 3344 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.51 | 145 | 2335 |
| physics/steps | count | 0.86 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 20 | 378 |
| stream/upload/bytes | bytes | 7351 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1612 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 34.7 ms; slowest 18, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.59 | 2587.1 | main/busy/ms 2561, surface/advance_cpu_ms 69.88, surface/cell_upload_ms 0.82, stream/drive/ms 0.31, weather/map/ms 0.14, bigspace/total/ms 0.11, stream/tick/ms 0.07, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.02, far/select/ms 0.01 |
| warm-up | 3.07 | 478.9 | main/busy/ms 2598, render/thread/ms 440, render/busy/ms 326, render/acquire/ms 111, surface/advance_cpu_ms 69.88, render/graph/ms 5.21, render/present/ms 3.29, stream/drive/ms 1.34, surface/cell_upload_ms 0.82, stream/tick/ms 0.71, terrain/update/ms 0.62, terrain/select/ms 0.44 |
| warm-up | 3.27 | 208.7 | pipelines/wait_ms 791, render/thread/ms 455, render/busy/ms 339, pipelines/compile_ms 217, pipelines/front_ms 201, render/acquire/ms 111, main/busy/ms 54.25, terrain/update/ms 19.45, pipelines/driver_ms 16.03, render/graph/ms 9.65, render/present/ms 4.25, stream/drive/ms 1.82 |
| warm-up | 3.41 | 48.2 | render/thread/ms 72.13, render/busy/ms 43.30, main/busy/ms 37.95, render/acquire/ms 27.38, stream/drive/ms 4.40, render/graph/ms 4.26, stream/tick/ms 2.59, render/present/ms 1.46, terrain/update/ms 0.92, stream/spawn/ms 0.86, terrain/select/ms 0.71, bigspace/total/ms 0.49 |
| warm-up | 3.55 | 108.8 | render/thread/ms 110, render/busy/ms 94.20, main/busy/ms 71.04, render/acquire/ms 14.05, terrain/update/ms 8.59, render/graph/ms 5.25, stream/drive/ms 2.96, stream/tick/ms 2.19, render/present/ms 1.60, terrain/select/ms 0.57, bigspace/total/ms 0.32, bigspace/high_precision/ms 0.15 |
| warm-up | 3.68 | 37.7 | main/busy/ms 38.47, render/thread/ms 33.61, render/busy/ms 32.23, stream/drive/ms 27.21, render/graph/ms 3.03, render/present/ms 1.16, stream/tick/ms 0.91, terrain/update/ms 0.66, stream/spawn/ms 0.64, terrain/select/ms 0.55, render/acquire/ms 0.22, bigspace/total/ms 0.19 |
| warm-up | 3.74 | 38.4 | render/thread/ms 51.48, render/busy/ms 49.77, main/busy/ms 41.14, stream/drive/ms 28.42, render/graph/ms 5.35, stream/spawn/ms 1.74, render/present/ms 1.44, stream/tick/ms 1.06, terrain/update/ms 0.61, terrain/select/ms 0.49, bigspace/total/ms 0.38, render/acquire/ms 0.27 |
| warm-up | 3.84 | 86.0 | pipelines/wait_ms 6420, pipelines/compile_ms 269, pipelines/front_ms 238, render/thread/ms 66.60, render/busy/ms 63.92, main/busy/ms 37.06, pipelines/driver_ms 30.65, stream/drive/ms 13.30, render/graph/ms 7.41, render/present/ms 2.38, stream/spawn/ms 2.15, stream/tick/ms 1.68 |
| warm-up | 3.93 | 59.8 | pipelines/wait_ms 2747, pipelines/compile_ms 83.44, pipelines/front_ms 78.81, render/thread/ms 61.81, render/busy/ms 60.53, main/busy/ms 29.85, pipelines/driver_ms 4.63, stream/drive/ms 4.46, render/graph/ms 3.80, terrain/update/ms 2.85, stream/spawn/ms 1.25, stream/activate/ms 1.10 |
| warm-up | 4.00 | 64.9 | main/busy/ms 55.99, render/thread/ms 38.31, render/busy/ms 36.86, stream/drive/ms 29.27, render/graph/ms 3.88, stream/spawn/ms 2.59, render/present/ms 1.21, terrain/update/ms 1.07, stream/tick/ms 1.06, terrain/select/ms 0.88, bigspace/total/ms 0.51, render/acquire/ms 0.24 |
| warm-up | 4.07 | 38.8 | render/thread/ms 48.68, render/busy/ms 47.10, main/busy/ms 40.18, stream/drive/ms 24.11, render/graph/ms 6.07, stream/tick/ms 1.41, render/present/ms 1.31, stream/activate/ms 1.11, terrain/update/ms 1.03, terrain/select/ms 0.86, physics/insert/ms 0.45, render/acquire/ms 0.27 |
| warm-up | 4.13 | 37.9 | pipelines/wait_ms 2700, pipelines/compile_ms 208, pipelines/front_ms 194, main/busy/ms 42.24, render/thread/ms 36.77, render/busy/ms 35.49, stream/drive/ms 28.91, pipelines/driver_ms 14.40, render/graph/ms 12.77, render/present/ms 1.07, stream/spawn/ms 1.07, stream/tick/ms 1.03 |
| warm-up | 4.20 | 56.8 | main/busy/ms 64.78, stream/drive/ms 49.31, render/thread/ms 31.22, render/busy/ms 29.94, render/graph/ms 6.47, stream/spawn/ms 2.04, render/present/ms 1.07, terrain/update/ms 0.91, terrain/select/ms 0.78, stream/tick/ms 0.70, bigspace/total/ms 0.30, render/acquire/ms 0.20 |
| warm-up | 4.25 | 36.4 | main/busy/ms 50.57, stream/drive/ms 33.93, render/thread/ms 27.99, render/busy/ms 26.85, render/graph/ms 3.90, stream/spawn/ms 2.14, stream/activate/ms 0.95, render/present/ms 0.93, stream/tick/ms 0.82, terrain/update/ms 0.74, terrain/select/ms 0.63, physics/insert/ms 0.37 |
| warm-up | 4.88 | 34.9 | main/busy/ms 53.23, render/thread/ms 29.06, render/busy/ms 27.66, stream/drive/ms 14.94, terrain/update/ms 8.34, render/graph/ms 6.42, stream/spawn/ms 1.30, render/present/ms 1.19, physics/insert/ms 1.16, stream/tick/ms 0.73, bigspace/total/ms 0.58, terrain/select/ms 0.57 |
| warm-up | 5.48 | 43.9 | render/thread/ms 58.53, render/busy/ms 57.21, main/busy/ms 45.64, stream/drive/ms 24.29, render/graph/ms 8.23, stream/spawn/ms 2.21, stream/tick/ms 1.23, render/present/ms 1.12, terrain/update/ms 0.80, terrain/select/ms 0.69, bigspace/total/ms 0.63, bigspace/high_precision/ms 0.28 |
| warm-up | 5.80 | 41.0 | main/busy/ms 58.39, render/thread/ms 40.40, render/busy/ms 38.91, render/graph/ms 6.55, stream/drive/ms 2.75, bigspace/total/ms 1.98, stream/spawn/ms 1.57, bigspace/low_precision/ms 1.36, render/present/ms 1.20, anim/evaluate_cpu/ms 0.75, stream/tick/ms 0.64, terrain/update/ms 0.60 |
| measure | 19.28 | 36.4 | render/thread/ms 49.62, render/busy/ms 47.98, render/graph/ms 29.42, main/busy/ms 21.04, render/present/ms 1.48, bigspace/total/ms 0.75, controller/feed/ms 0.63, terrain/update/ms 0.55, terrain/select/ms 0.49, stream/drive/ms 0.44, physics/insert/ms 0.39, far/select/ms 0.35 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --texture-cap 512 --resolution 2560x1440 --bench-out target/perf\lean_cap512\balmora_walk@2560x1440_--texture-cap_512-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **73%** (min 54%) · 2580 MHz (min 2580) · memory 8001 MHz · 71.2 W · 73 °C (max 78) · VRAM 5299 MiB · throttled 100% of samples
