# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4850 frames over 65.6 s · **median 13.01 ms (77 fps)** · mean 13.54 · p95 16.86 · p99 19.27 (1% low 52 fps) · worst 37.42 ms · 41 spikes over 1.5× median · 23.8 stutters/min (work 21.0/min)
- warm-up: 362 frames over 8.9 s, worst 2816.7 ms
- hitches (> 32.5 ms): **3** measured, 16 in warm-up
- timed GPU passes: 7.63 ms of a 13.54 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.01 ms, limit 11 ms
- **FAIL** frame_ms: 26 stutters (> 1.5x moving median of 60) in 1.09 min = 23.76/min, limit 1/min
- **FAIL** worst frame 37.42 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.072 | 2.075 | 2.110 | 1.00 | 0.181 |
| main_opaque_pass_3d | 1.650 | 1.652 | 2.032 | 1.00 | 0.068 |
| early prepass | 0.523 | 0.523 | 0.746 | 1.00 | 0.118 |
| bloom | 0.320 | 0.321 | 0.326 | 1.00 | 0.066 |
| atmosphere_luts | 0.313 | 0.313 | 0.372 | 1.00 | 0.003 |
| uniform_allocation | 0.305 | 0.305 | 0.330 | 1.00 | 0.052 |
| water_velocity | 0.283 | 0.284 | 0.353 | 1.00 | 0.001 |
| bin_unpacking | 0.238 | 0.238 | 0.266 | 1.00 | 0.037 |
| water_sspr | 0.210 | 0.210 | 0.210 | 1.00 | 0.002 |
| ssao | 0.192 | 0.192 | 0.194 | 1.00 | 0.007 |
| auxiliary_views | 0.176 | 0.176 | 0.689 | 1.00 | 0.160 |
| atmos_froxel_inject | 0.148 | 0.148 | 0.150 | 1.00 | 0.002 |
| clouds_trace | 0.141 | 0.141 | 0.230 | 1.00 | 0.001 |
| water_fft | 0.131 | 0.132 | 0.137 | 1.00 | 0.003 |
| upscaling | 0.120 | 0.121 | 0.128 | 1.00 | 0.004 |
| clouds_composite | 0.108 | 0.108 | 0.111 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.088 | 0.088 | 0.166 | 1.00 | 0.008 |
| tonemapping | 0.087 | 0.087 | 0.090 | 1.00 | 0.004 |
| clustering | 0.065 | 0.065 | 0.069 | 1.00 | 0.020 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.420 | 0.420 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.048 | 0.057 | 0.057 | 0.84 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.048 | 1.00 | 0.005 |
| water_snapshot | 0.032 | 0.032 | 0.035 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.225 | 0.234 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.026 | 0.029 | 1.00 | 0.001 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.023 | 1.00 | 0.006 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 1.00 | 0.108 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 37.602 | 0.000 | 26324.30 | 182370.9 |
| render/thread/ms | 12.338 | 15.459 | 36.17 | 59841.1 |
| render/busy/ms | 11.782 | 14.888 | 35.69 | 57142.0 |
| main/busy/ms | 9.232 | 12.055 | 27.16 | 44777.5 |
| render/graph/ms | 7.280 | 9.318 | 31.48 | 35310.2 |
| pipelines/compile_ms | 0.708 | 0.000 | 158.40 | 3435.9 |
| pipelines/front_ms | 0.684 | 0.000 | 151.11 | 3317.6 |
| render/present/ms | 0.465 | 0.556 | 3.73 | 2256.0 |
| bigspace/total/ms | 0.434 | 0.646 | 3.72 | 2103.6 |
| terrain/update/ms | 0.290 | 0.365 | 1.72 | 1407.4 |
| terrain/select/ms | 0.258 | 0.324 | 0.59 | 1251.0 |
| controller/feed/ms | 0.256 | 0.347 | 0.89 | 1239.2 |
| stream/drive/ms | 0.250 | 0.354 | 18.71 | 1214.7 |
| stream/tick/ms | 0.190 | 0.264 | 0.70 | 921.5 |
| far/select/ms | 0.187 | 0.253 | 0.64 | 905.6 |
| anim/evaluate_cpu/ms | 0.170 | 0.218 | 0.53 | 824.2 |
| bigspace/low_precision/ms | 0.167 | 0.283 | 3.29 | 810.6 |
| controller/move/ms | 0.128 | 0.255 | 0.45 | 620.4 |
| anim/evaluate/ms | 0.117 | 0.271 | 3.61 | 567.3 |
| bigspace/high_precision/ms | 0.108 | 0.202 | 3.38 | 525.2 |
| render/acquire/ms | 0.091 | 0.120 | 0.41 | 443.0 |
| physics/insert/ms | 0.060 | 0.461 | 1.07 | 291.9 |
| physics/step/ms | 0.059 | 0.091 | 0.63 | 284.6 |
| weather/map/ms | 0.025 | 0.034 | 0.09 | 119.4 |
| pipelines/driver_ms | 0.024 | 0.000 | 7.29 | 118.3 |
| anim/apply/ms | 0.012 | 0.024 | 0.16 | 58.4 |
| weather/eval/ms | 0.010 | 0.014 | 0.04 | 50.1 |
| controller/avoid/ms | 0.010 | 0.013 | 0.20 | 47.0 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 32.6 |
| stream/despawn/ms | 0.006 | 0.000 | 1.09 | 28.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 25.0 |
| script/input/ms | 0.005 | 0.006 | 0.19 | 23.4 |
| script/ui/ms | 0.005 | 0.006 | 0.15 | 22.9 |
| controller/write/ms | 0.003 | 0.004 | 0.08 | 16.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.62 | 13.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 36 | 0 |
| anim/a2/count | count | 1 | 11 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 96 | 18 |
| controller/movers/count | count | 19 | 21 | 18 |
| crowd/c0/count | count | 26 | 62 | 13 |
| crowd/c0/joints/count | count | 1291 | 2334 | 601 |
| crowd/c0/meshes/count | count | 355 | 646 | 159 |
| crowd/c1/count | count | 145 | 158 | 148 |
| crowd/c1/joints/count | count | 5531 | 6092 | 5595 |
| crowd/c1/meshes/count | count | 1166 | 1338 | 1182 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4263 | 13.6 k | 6179 |
| crowd/joints_uploaded/count | count | 4234 | 6643 | 6179 |
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
| frame/work/ms | ms | 11.31 | 35.69 | 11.75 |
| gpu/timed/ms | ms | 8.32 | 11.91 | 8.45 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 804 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 804 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 33 | 38 | 35 |
| phases/opaque3d/bins | count | 1283 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3385 | 5156 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 33 | 38 | 35 |
| phases/opaque3d_prepass/bins | count | 1283 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3385 | 5156 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 71 | 340 | 57 |
| phases/shadow/bins | count | 3732 | 5459 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9348 | 12.8 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 137 | 322 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2347 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1129 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 220 | 226 | 226 |
| pipelines/waiting | count | 0 | 45 | 0 |
| preprocess/bind_groups | count | 13 | 909 | 6 |
| preprocess/bind_groups_created | count | 13 | 909 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 306 | 486 | 299 |
| process/cpu_usage | % | 23.56 | 43.75 | 21.63 |
| process/mem_usage | GiB | 5.16 | 5.35 | 5.35 |
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
| started_load_count |  | 4767 | 4860 | 4860 |
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
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.34 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.97 | 50.30 | 31.02 |
| system/mem_usage | % | 75.72 | 76.24 | 76.18 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 153 | 422 | 141 |
| views/all/bins | count | 6316 | 8379 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 201 | 388 | 113 |
| views/all/pipelines_distinct | count | 95 | 110 | 91 |
| views/all/work_items | count | 17.1 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 69 | 89 | 57 |
| views/cascade/bins | count | 2597 | 3184 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8150 | 10.2 k | 5663 |
| views/cascade/pipelines | count | 55 | 73 | 47 |
| views/cascade/work_items | count | 8150 | 10.2 k | 5663 |
| views/lamp_face/batch_sets | count | 0 | 266 | 0 |
| views/lamp_face/bins | count | 1064 | 2610 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1181 | 2933 | 0 |
| views/lamp_face/pipelines | count | 81 | 258 | 0 |
| views/lamp_face/work_items | count | 0 | 2638 | 0 |
| views/main/batch_sets | count | 82 | 92 | 84 |
| views/main/bins | count | 2868 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8374 | 13.1 k | 7594 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8544 | 13.3 k | 7902 |
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
| anim/bones_written/count | count | 57.43 | 1223 | 278.5 k |
| anim/events/count | count | 0.11 | 3 | 539 |
| assets/Image/added | count | 0.14 | 43 | 697 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.91 | 152 | 4415 |
| assets/Mesh/removed | count | 1.01 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.12 | 35 | 577 |
| assets/StandardMaterial/modified | count | 4.47 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 3.01 | 4533 | 14.6 k |
| physics/inserts | count | 0.69 | 160 | 3331 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.48 | 129 | 2335 |
| physics/steps | count | 0.81 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.01 | 6 | 26 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 25 | 378 |
| stream/upload/bytes | bytes | 6980 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1530 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 32.5 ms; slowest 19, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.82 | 2816.7 | main/busy/ms 2791, surface/advance_cpu_ms 66.26, surface/cell_upload_ms 0.85, stream/drive/ms 0.33, weather/map/ms 0.12, stream/tick/ms 0.09, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 3.58 | 759.2 | main/busy/ms 2827, render/thread/ms 724, render/busy/ms 559, render/acquire/ms 162, surface/advance_cpu_ms 66.26, render/graph/ms 4.84, render/present/ms 2.64, stream/drive/ms 1.24, surface/cell_upload_ms 0.85, stream/tick/ms 0.63, terrain/update/ms 0.44, terrain/select/ms 0.30 |
| warm-up | 3.87 | 290.6 | pipelines/wait_ms 3989, render/thread/ms 738, render/busy/ms 573, pipelines/compile_ms 302, pipelines/front_ms 281, render/acquire/ms 162, main/busy/ms 55.69, pipelines/driver_ms 21.46, terrain/update/ms 19.76, render/graph/ms 9.16, render/present/ms 3.61, stream/drive/ms 1.69 |
| warm-up | 4.14 | 92.2 | render/thread/ms 78.99, main/busy/ms 71.17, render/busy/ms 68.34, render/acquire/ms 9.15, render/graph/ms 5.31, stream/drive/ms 3.27, stream/tick/ms 2.27, render/present/ms 1.50, bigspace/total/ms 0.74, terrain/update/ms 0.73, terrain/select/ms 0.60, bigspace/high_precision/ms 0.51 |
| warm-up | 4.30 | 75.4 | render/thread/ms 46.98, main/busy/ms 46.25, render/busy/ms 45.64, stream/drive/ms 34.63, render/graph/ms 2.39, render/present/ms 1.12, stream/tick/ms 0.93, terrain/update/ms 0.57, terrain/select/ms 0.47, render/acquire/ms 0.23, bigspace/total/ms 0.20, far/select/ms 0.10 |
| warm-up | 4.42 | 51.0 | pipelines/wait_ms 2021, pipelines/compile_ms 108, pipelines/front_ms 101, render/thread/ms 59.82, render/busy/ms 57.85, main/busy/ms 43.91, stream/drive/ms 17.85, pipelines/driver_ms 7.89, render/graph/ms 5.45, stream/tick/ms 1.75, render/present/ms 1.67, terrain/update/ms 1.03 |
| warm-up | 4.48 | 57.9 | pipelines/wait_ms 4798, pipelines/compile_ms 176, pipelines/front_ms 160, render/thread/ms 77.49, render/busy/ms 75.81, main/busy/ms 42.84, stream/drive/ms 18.97, pipelines/driver_ms 15.56, render/graph/ms 6.06, stream/tick/ms 1.50, render/present/ms 1.44, stream/spawn/ms 1.19 |
| warm-up | 4.52 | 44.0 | pipelines/wait_ms 7655, pipelines/compile_ms 121, pipelines/front_ms 108, render/thread/ms 67.51, render/busy/ms 66.21, main/busy/ms 54.93, stream/drive/ms 28.25, pipelines/driver_ms 12.62, render/graph/ms 4.10, stream/spawn/ms 2.05, stream/tick/ms 1.39, render/present/ms 1.10 |
| warm-up | 4.57 | 42.4 | pipelines/wait_ms 4879, pipelines/compile_ms 53.67, main/busy/ms 53.58, render/thread/ms 50.19, render/busy/ms 48.97, pipelines/front_ms 48.73, stream/drive/ms 27.78, pipelines/driver_ms 4.95, terrain/update/ms 3.16, render/graph/ms 2.82, stream/spawn/ms 1.81, stream/tick/ms 1.51 |
| warm-up | 4.69 | 38.9 | pipelines/wait_ms 369, pipelines/compile_ms 148, pipelines/front_ms 141, render/thread/ms 57.27, render/busy/ms 50.39, main/busy/ms 23.74, render/graph/ms 20.92, pipelines/driver_ms 6.73, render/present/ms 6.63, stream/drive/ms 4.55, stream/spawn/ms 2.00, stream/tick/ms 1.06 |
| warm-up | 4.76 | 44.3 | main/busy/ms 73.80, stream/drive/ms 55.48, render/thread/ms 42.00, render/busy/ms 40.26, render/graph/ms 4.31, stream/spawn/ms 2.42, stream/tick/ms 1.63, render/present/ms 1.39, terrain/update/ms 0.95, terrain/select/ms 0.81, render/acquire/ms 0.35, stream/activate/ms 0.31 |
| warm-up | 4.82 | 66.8 | main/busy/ms 71.72, stream/drive/ms 54.48, render/thread/ms 43.64, render/busy/ms 42.46, render/graph/ms 3.00, stream/spawn/ms 2.15, render/present/ms 0.97, terrain/update/ms 0.93, terrain/select/ms 0.80, stream/tick/ms 0.75, stream/activate/ms 0.62, physics/insert/ms 0.36 |
| warm-up | 4.92 | 90.1 | pipelines/wait_ms 2665, pipelines/compile_ms 204, pipelines/front_ms 190, main/busy/ms 80.38, stream/drive/ms 54.88, render/thread/ms 35.10, render/busy/ms 33.95, pipelines/driver_ms 13.76, render/graph/ms 4.58, stream/spawn/ms 2.05, render/present/ms 0.94, stream/tick/ms 0.94 |
| warm-up | 4.99 | 47.2 | pipelines/wait_ms 1780, pipelines/compile_ms 141, pipelines/front_ms 131, main/busy/ms 50.72, stream/drive/ms 38.32, render/thread/ms 30.18, render/busy/ms 28.93, pipelines/driver_ms 9.37, render/graph/ms 3.86, stream/spawn/ms 1.41, stream/activate/ms 1.15, render/present/ms 1.01 |
| warm-up | 5.79 | 35.4 | pipelines/wait_ms 5943, pipelines/compile_ms 62.28, pipelines/front_ms 58.12, render/thread/ms 50.71, render/busy/ms 48.62, main/busy/ms 24.38, render/graph/ms 9.35, stream/drive/ms 4.48, pipelines/driver_ms 4.16, stream/spawn/ms 1.81, render/present/ms 1.72, terrain/update/ms 0.83 |
| warm-up | 5.86 | 46.9 | render/thread/ms 55.00, render/busy/ms 53.65, main/busy/ms 42.90, render/graph/ms 6.39, stream/drive/ms 4.66, stream/spawn/ms 2.18, stream/tick/ms 1.26, render/present/ms 1.05, terrain/update/ms 0.99, terrain/select/ms 0.81, bigspace/total/ms 0.64, render/acquire/ms 0.30 |
| measure | 2.18 | 33.5 | pipelines/wait_ms 7339, pipelines/compile_ms 149, pipelines/front_ms 146, render/thread/ms 46.96, render/busy/ms 45.37, render/graph/ms 28.21, main/busy/ms 20.23, pipelines/driver_ms 3.24, render/present/ms 1.39, bigspace/total/ms 0.96, terrain/update/ms 0.59, stream/drive/ms 0.54 |
| measure | 2.73 | 36.9 | render/thread/ms 50.64, render/busy/ms 49.19, main/busy/ms 25.75, render/graph/ms 25.33, render/present/ms 1.28, bigspace/total/ms 0.82, physics/insert/ms 0.69, terrain/update/ms 0.58, terrain/select/ms 0.52, stream/drive/ms 0.44, far/select/ms 0.42, controller/feed/ms 0.42 |
| measure | 14.15 | 37.4 | render/thread/ms 48.58, render/busy/ms 47.59, render/graph/ms 39.12, main/busy/ms 20.15, render/present/ms 0.85, bigspace/total/ms 0.76, controller/feed/ms 0.59, terrain/update/ms 0.54, terrain/select/ms 0.48, stream/drive/ms 0.46, stream/tick/ms 0.40, far/select/ms 0.37 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\h1_old\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 11%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **73%** (min 45%) · 2580 MHz (min 2580) · memory 8001 MHz · 72.0 W · 74 °C (max 79) · VRAM 5413 MiB · throttled 100% of samples
