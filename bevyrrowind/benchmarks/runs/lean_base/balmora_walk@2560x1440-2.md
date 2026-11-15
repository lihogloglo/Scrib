# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4277 frames over 65.6 s · **median 14.87 ms (67 fps)** · mean 15.35 · p95 19.16 · p99 21.87 (1% low 46 fps) · worst 39.74 ms · 36 spikes over 1.5× median · 20.1 stutters/min (work 18.3/min)
- warm-up: 353 frames over 9.7 s, worst 3369.0 ms
- hitches (> 37.2 ms): **4** measured, 17 in warm-up
- timed GPU passes: 8.12 ms of a 15.35 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 14.87 ms, limit 11 ms
- **FAIL** frame_ms: 22 stutters (> 1.5x moving median of 60) in 1.09 min = 20.11/min, limit 1/min
- **FAIL** worst frame 39.74 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.206 | 2.208 | 2.674 | 1.00 | 0.189 |
| main_opaque_pass_3d | 1.754 | 1.756 | 2.441 | 1.00 | 0.097 |
| early prepass | 0.543 | 0.544 | 1.023 | 1.00 | 0.130 |
| bloom | 0.341 | 0.341 | 0.494 | 1.00 | 0.072 |
| uniform_allocation | 0.334 | 0.334 | 0.460 | 1.00 | 0.066 |
| atmosphere_luts | 0.329 | 0.329 | 0.459 | 1.00 | 0.004 |
| water_velocity | 0.305 | 0.305 | 0.548 | 1.00 | 0.002 |
| bin_unpacking | 0.264 | 0.264 | 0.285 | 1.00 | 0.045 |
| water_sspr | 0.226 | 0.226 | 0.228 | 1.00 | 0.003 |
| auxiliary_views | 0.203 | 0.203 | 0.740 | 1.00 | 0.196 |
| ssao | 0.202 | 0.203 | 0.209 | 1.00 | 0.008 |
| atmos_froxel_inject | 0.160 | 0.160 | 0.152 | 1.00 | 0.002 |
| clouds_trace | 0.141 | 0.142 | 0.236 | 1.00 | 0.001 |
| water_fft | 0.136 | 0.136 | 0.137 | 1.00 | 0.003 |
| upscaling | 0.127 | 0.127 | 0.145 | 1.00 | 0.004 |
| clouds_composite | 0.112 | 0.112 | 0.111 | 1.00 | 0.002 |
| tonemapping | 0.092 | 0.092 | 0.091 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.090 | 0.091 | 0.170 | 1.00 | 0.009 |
| clustering | 0.069 | 0.069 | 0.069 | 1.00 | 0.021 |
| lightprobe_irradiance_map | 0.056 | 0.451 | 0.761 | 0.13 | 0.001 |
| surface_trail | 0.055 | 0.061 | 0.057 | 0.91 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.054 | 1.00 | 0.002 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.044 | 0.044 | 0.048 | 1.00 | 0.007 |
| water_snapshot | 0.036 | 0.036 | 0.043 | 1.00 | 0.002 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.004 |
| lightprobe_radiance_map | 0.030 | 0.236 | 0.236 | 0.13 | 0.005 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 1.00 | 0.001 |
| water_mips | 0.025 | 0.025 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.024 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.031 | 1.00 | 0.115 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.016 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 65.670 | 0.000 | 27438.74 | 280869.8 |
| render/thread/ms | 14.049 | 17.729 | 35.48 | 60087.1 |
| render/busy/ms | 13.465 | 17.110 | 34.65 | 57590.2 |
| main/busy/ms | 10.241 | 13.453 | 28.77 | 43801.0 |
| render/graph/ms | 8.214 | 10.510 | 24.18 | 35131.7 |
| pipelines/compile_ms | 0.838 | 0.000 | 201.54 | 3585.7 |
| pipelines/front_ms | 0.807 | 0.000 | 194.03 | 3453.3 |
| render/present/ms | 0.490 | 0.591 | 16.28 | 2096.8 |
| bigspace/total/ms | 0.481 | 0.760 | 5.41 | 2055.9 |
| controller/feed/ms | 0.303 | 0.416 | 0.76 | 1294.5 |
| terrain/update/ms | 0.295 | 0.372 | 2.26 | 1260.5 |
| stream/drive/ms | 0.262 | 0.390 | 16.81 | 1119.4 |
| terrain/select/ms | 0.261 | 0.328 | 0.70 | 1115.4 |
| far/select/ms | 0.213 | 0.312 | 0.79 | 912.8 |
| stream/tick/ms | 0.195 | 0.277 | 0.67 | 835.6 |
| bigspace/low_precision/ms | 0.187 | 0.324 | 5.04 | 800.5 |
| anim/evaluate_cpu/ms | 0.172 | 0.226 | 1.03 | 733.6 |
| controller/move/ms | 0.150 | 0.264 | 0.53 | 642.5 |
| anim/evaluate/ms | 0.145 | 0.290 | 4.57 | 620.3 |
| bigspace/high_precision/ms | 0.114 | 0.236 | 4.55 | 487.4 |
| render/acquire/ms | 0.094 | 0.124 | 0.36 | 400.2 |
| physics/insert/ms | 0.078 | 0.497 | 1.31 | 332.3 |
| physics/step/ms | 0.070 | 0.106 | 0.57 | 299.2 |
| pipelines/driver_ms | 0.031 | 0.000 | 7.51 | 132.4 |
| weather/map/ms | 0.026 | 0.035 | 0.33 | 110.4 |
| anim/apply/ms | 0.013 | 0.022 | 0.15 | 54.5 |
| controller/avoid/ms | 0.011 | 0.014 | 0.13 | 45.8 |
| weather/eval/ms | 0.010 | 0.014 | 0.15 | 44.7 |
| script/update/ms | 0.008 | 0.010 | 0.21 | 33.0 |
| stream/despawn/ms | 0.007 | 0.000 | 1.11 | 30.1 |
| stream/spawn/ms | 0.006 | 0.000 | 1.05 | 27.2 |
| script/input/ms | 0.005 | 0.007 | 0.27 | 23.1 |
| script/ui/ms | 0.004 | 0.006 | 0.05 | 19.0 |
| controller/write/ms | 0.004 | 0.005 | 0.07 | 15.8 |
| stream/deactivate/ms | 0.004 | 0.000 | 0.67 | 15.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.33 | 4.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.08 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 15 | 0 |
| anim/a2/count | count | 1 | 16 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 72 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 26 | 54 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 595 |
| crowd/c0/meshes/count | count | 353 | 646 | 158 |
| crowd/c1/count | count | 147 | 158 | 148 |
| crowd/c1/joints/count | count | 5507 | 6045 | 5601 |
| crowd/c1/meshes/count | count | 1173 | 1313 | 1183 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4268 | 9821 | 6179 |
| crowd/joints_uploaded/count | count | 4250 | 6643 | 6179 |
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
| frame/work/ms | ms | 13.04 | 34.65 | 11.62 |
| gpu/timed/ms | ms | 8.81 | 12.73 | 9.24 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 602.13 M | 603.31 M | 603.31 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 8 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 809 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 809 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 38 | 41 | 39 |
| phases/opaque3d/bins | count | 1285 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3400 | 5118 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 38 | 41 | 39 |
| phases/opaque3d_prepass/bins | count | 1285 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3400 | 5118 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 75 | 328 | 58 |
| phases/shadow/bins | count | 3771 | 5384 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9523 | 12.7 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 148 | 306 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2640 | 3268 | 2349 |
| physics/items | count | 137 | 476 | 19 |
| physics/pending | count | 0 | 1011 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 220 | 226 | 226 |
| pipelines/waiting | count | 0 | 53 | 0 |
| preprocess/bind_groups | count | 13 | 901 | 6 |
| preprocess/bind_groups_created | count | 13 | 901 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 324 | 473 | 321 |
| process/cpu_usage | % | 22.77 | 47.30 | 30.77 |
| process/mem_usage | GiB | 5.16 | 5.32 | 5.32 |
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
| started_load_count |  | 4765 | 4859 | 4859 |
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
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.11 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.36 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.62 | 60.44 | 27.47 |
| system/mem_usage | % | 76.19 | 76.83 | 76.66 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 163 | 418 | 152 |
| views/all/bins | count | 6367 | 8178 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.0 k | 13.3 k |
| views/all/pipelines | count | 212 | 371 | 113 |
| views/all/pipelines_distinct | count | 97 | 110 | 91 |
| views/all/work_items | count | 17.3 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 68 | 86 | 58 |
| views/cascade/bins | count | 2601 | 3142 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8127 | 10.1 k | 5663 |
| views/cascade/pipelines | count | 55 | 70 | 47 |
| views/cascade/work_items | count | 8127 | 10.1 k | 5663 |
| views/lamp_face/batch_sets | count | 0 | 251 | 0 |
| views/lamp_face/bins | count | 1084 | 2436 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1227 | 2812 | 0 |
| views/lamp_face/pipelines | count | 90 | 245 | 0 |
| views/lamp_face/work_items | count | 0 | 2575 | 0 |
| views/main/batch_sets | count | 92 | 98 | 94 |
| views/main/bins | count | 2872 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8410 | 13.0 k | 7594 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8573 | 13.2 k | 7902 |
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
| anim/bones_written/count | count | 57.80 | 726 | 247.2 k |
| anim/events/count | count | 0.12 | 3 | 527 |
| assets/Image/added | count | 0.16 | 46 | 689 |
| assets/Image/modified | count | 0.00 | 3 | 18 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 1.02 | 152 | 4376 |
| assets/Mesh/removed | count | 1.14 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.13 | 47 | 574 |
| assets/StandardMaterial/modified | count | 5.07 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.10 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.33 | 3371 | 9985 |
| physics/inserts | count | 0.74 | 105 | 3159 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.54 | 129 | 2327 |
| physics/steps | count | 0.92 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.09 | 24 | 378 |
| stream/upload/bytes | bytes | 7915 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1735 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 37.2 ms; slowest 21, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 3.37 | 3369.0 | main/busy/ms 3343, surface/advance_cpu_ms 140, surface/cell_upload_ms 0.72, stream/drive/ms 0.61, weather/map/ms 0.20, stream/tick/ms 0.16, bigspace/total/ms 0.15, bigspace/high_precision/ms 0.09, weather/eval/ms 0.04, script/update/ms 0.03, anim/evaluate/ms 0.02, script/input/ms 0.02 |
| warm-up | 4.16 | 786.5 | main/busy/ms 3378, render/thread/ms 752, render/busy/ms 594, render/acquire/ms 155, surface/advance_cpu_ms 140, render/graph/ms 6.43, render/present/ms 2.50, stream/drive/ms 1.45, surface/cell_upload_ms 0.72, stream/tick/ms 0.70, terrain/update/ms 0.55, terrain/select/ms 0.36 |
| warm-up | 4.38 | 224.7 | pipelines/wait_ms 2694, render/thread/ms 763, render/busy/ms 604, pipelines/compile_ms 242, pipelines/front_ms 226, render/acquire/ms 155, main/busy/ms 53.28, terrain/update/ms 19.02, pipelines/driver_ms 15.72, render/graph/ms 11.05, render/present/ms 3.40, stream/drive/ms 1.56 |
| warm-up | 4.73 | 77.3 | pipelines/wait_ms 238, main/busy/ms 60.34, pipelines/compile_ms 53.74, pipelines/front_ms 50.18, render/thread/ms 44.36, render/busy/ms 42.90, terrain/update/ms 5.35, render/graph/ms 4.29, pipelines/driver_ms 3.56, stream/drive/ms 1.59, render/present/ms 1.24, stream/tick/ms 1.06 |
| warm-up | 4.85 | 61.7 | pipelines/wait_ms 731, pipelines/compile_ms 170, pipelines/front_ms 155, main/busy/ms 45.99, render/thread/ms 44.63, render/busy/ms 43.25, stream/drive/ms 32.93, pipelines/driver_ms 15.10, render/graph/ms 4.08, render/present/ms 1.17, stream/tick/ms 0.85, terrain/update/ms 0.75 |
| warm-up | 4.92 | 56.2 | pipelines/wait_ms 916, pipelines/compile_ms 98.54, pipelines/front_ms 91.89, render/thread/ms 38.19, render/busy/ms 36.42, main/busy/ms 33.00, stream/drive/ms 16.85, pipelines/driver_ms 6.66, render/graph/ms 4.53, render/present/ms 1.55, stream/spawn/ms 1.18, stream/tick/ms 0.90 |
| warm-up | 4.98 | 47.3 | pipelines/wait_ms 2386, pipelines/compile_ms 72.38, pipelines/front_ms 67.52, render/thread/ms 46.63, render/busy/ms 44.51, main/busy/ms 31.99, stream/drive/ms 11.38, render/graph/ms 5.40, pipelines/driver_ms 4.87, render/present/ms 1.85, stream/spawn/ms 1.72, stream/tick/ms 1.24 |
| warm-up | 5.03 | 49.5 | pipelines/wait_ms 3983, pipelines/compile_ms 104, pipelines/front_ms 97.84, render/thread/ms 74.81, render/busy/ms 72.77, main/busy/ms 54.29, stream/drive/ms 31.46, render/graph/ms 9.05, pipelines/driver_ms 6.27, stream/tick/ms 1.96, render/present/ms 1.79, terrain/update/ms 0.90 |
| warm-up | 5.12 | 88.7 | pipelines/wait_ms 2096, render/thread/ms 109, render/busy/ms 107, pipelines/compile_ms 53.16, pipelines/front_ms 50.52, main/busy/ms 47.80, stream/drive/ms 31.18, render/graph/ms 9.54, pipelines/driver_ms 2.64, stream/tick/ms 1.54, stream/activate/ms 1.39, render/present/ms 1.38 |
| warm-up | 5.16 | 38.4 | pipelines/wait_ms 1810, pipelines/compile_ms 132, pipelines/front_ms 125, render/thread/ms 104, render/busy/ms 102, main/busy/ms 28.38, render/graph/ms 18.82, stream/drive/ms 12.09, pipelines/driver_ms 7.32, render/present/ms 1.91, stream/activate/ms 1.53, stream/spawn/ms 0.91 |
| warm-up | 5.36 | 89.4 | main/busy/ms 73.69, render/thread/ms 55.06, render/busy/ms 53.24, stream/drive/ms 51.55, render/graph/ms 6.42, stream/spawn/ms 3.19, stream/tick/ms 1.54, render/present/ms 1.48, terrain/update/ms 1.09, terrain/select/ms 0.86, stream/activate/ms 0.52, bigspace/total/ms 0.46 |
| warm-up | 5.46 | 72.7 | pipelines/wait_ms 784, pipelines/compile_ms 233, pipelines/front_ms 222, main/busy/ms 55.70, render/thread/ms 48.08, render/busy/ms 46.70, stream/drive/ms 40.32, pipelines/driver_ms 11.30, render/graph/ms 3.37, stream/spawn/ms 1.83, render/present/ms 1.20, stream/activate/ms 1.12 |
| warm-up | 5.54 | 57.5 | pipelines/wait_ms 6305, pipelines/compile_ms 149, pipelines/front_ms 133, main/busy/ms 61.88, stream/drive/ms 40.51, render/thread/ms 33.83, render/busy/ms 32.65, pipelines/driver_ms 16.14, render/graph/ms 6.13, stream/spawn/ms 1.86, render/present/ms 0.98, terrain/update/ms 0.90 |
| warm-up | 5.59 | 39.0 | pipelines/wait_ms 1139, pipelines/compile_ms 42.51, main/busy/ms 41.60, pipelines/front_ms 41.23, stream/drive/ms 29.60, render/thread/ms 25.67, render/busy/ms 23.95, render/graph/ms 4.70, render/present/ms 1.48, pipelines/driver_ms 1.28, stream/tick/ms 0.90, stream/activate/ms 0.88 |
| warm-up | 5.86 | 39.2 | render/thread/ms 40.15, render/busy/ms 38.56, main/busy/ms 33.21, render/graph/ms 8.32, stream/drive/ms 2.61, render/present/ms 1.38, stream/tick/ms 0.94, terrain/update/ms 0.78, terrain/select/ms 0.69, stream/spawn/ms 0.57, bigspace/total/ms 0.51, bigspace/high_precision/ms 0.28 |
| warm-up | 6.85 | 54.6 | pipelines/wait_ms 6557, main/busy/ms 59.56, render/thread/ms 42.33, render/busy/ms 40.67, pipelines/compile_ms 37.69, pipelines/front_ms 35.55, stream/drive/ms 14.39, render/graph/ms 7.33, stream/spawn/ms 2.65, pipelines/driver_ms 2.14, render/present/ms 1.37, physics/insert/ms 1.18 |
| warm-up | 6.91 | 53.9 | pipelines/wait_ms 4890, pipelines/compile_ms 105, pipelines/front_ms 99.86, render/thread/ms 63.07, render/busy/ms 47.98, main/busy/ms 36.21, render/present/ms 14.89, stream/drive/ms 7.35, render/graph/ms 5.53, pipelines/driver_ms 5.24, stream/spawn/ms 2.06, physics/insert/ms 1.37 |
| measure | 1.05 | 39.7 | main/busy/ms 49.71, render/thread/ms 49.68, render/busy/ms 48.14, render/graph/ms 17.64, render/present/ms 1.18, bigspace/total/ms 0.90, anim/evaluate_cpu/ms 0.89, stream/drive/ms 0.63, terrain/update/ms 0.61, stream/tick/ms 0.57, anim/evaluate/ms 0.56, terrain/select/ms 0.55 |
| measure | 2.18 | 38.6 | render/thread/ms 51.30, render/busy/ms 49.65, main/busy/ms 28.51, render/graph/ms 27.14, render/present/ms 1.45, bigspace/total/ms 1.00, controller/feed/ms 0.76, stream/drive/ms 0.66, stream/tick/ms 0.58, terrain/update/ms 0.57, terrain/select/ms 0.51, physics/insert/ms 0.51 |
| measure | 59.08 | 38.6 | render/thread/ms 45.27, render/busy/ms 43.87, main/busy/ms 25.47, render/graph/ms 19.86, controller/feed/ms 1.17, render/present/ms 1.17, bigspace/total/ms 1.11, terrain/update/ms 0.89, terrain/select/ms 0.78, stream/drive/ms 0.70, controller/move/ms 0.65, stream/tick/ms 0.61 |
| measure | 59.12 | 39.4 | render/thread/ms 46.81, render/busy/ms 45.65, main/busy/ms 20.78, render/graph/ms 15.33, bigspace/total/ms 1.16, render/present/ms 0.97, controller/feed/ms 0.93, terrain/update/ms 0.80, terrain/select/ms 0.71, stream/drive/ms 0.59, stream/tick/ms 0.51, controller/move/ms 0.51 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_base\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **69%** (min 47%) · 2580 MHz (min 2580) · memory 8001 MHz · 68.6 W · 72 °C (max 76) · VRAM 6297 MiB · throttled 100% of samples
