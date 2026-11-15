# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4629 frames over 65.6 s · **median 13.55 ms (74 fps)** · mean 14.18 · p95 18.05 · p99 20.33 (1% low 49 fps) · worst 39.79 ms · 47 spikes over 1.5× median · 19.2 stutters/min (work 17.4/min)
- warm-up: 358 frames over 7.9 s, worst 2521.2 ms
- hitches (> 33.9 ms): **2** measured, 13 in warm-up
- timed GPU passes: 7.99 ms of a 14.18 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.55 ms, limit 11 ms
- **FAIL** frame_ms: 21 stutters (> 1.5x moving median of 60) in 1.09 min = 19.19/min, limit 1/min
- **FAIL** worst frame 39.79 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.199 | 2.202 | 2.671 | 1.00 | 0.186 |
| main_opaque_pass_3d | 1.675 | 1.677 | 2.282 | 1.00 | 0.092 |
| early prepass | 0.537 | 0.538 | 0.859 | 1.00 | 0.128 |
| bloom | 0.341 | 0.342 | 0.498 | 1.00 | 0.068 |
| atmosphere_luts | 0.327 | 0.328 | 0.459 | 1.00 | 0.004 |
| uniform_allocation | 0.326 | 0.326 | 0.627 | 1.00 | 0.063 |
| water_velocity | 0.307 | 0.307 | 0.578 | 1.00 | 0.002 |
| bin_unpacking | 0.259 | 0.259 | 0.286 | 1.00 | 0.043 |
| water_sspr | 0.223 | 0.224 | 0.228 | 1.00 | 0.003 |
| ssao | 0.205 | 0.205 | 0.210 | 1.00 | 0.008 |
| auxiliary_views | 0.166 | 0.166 | 0.752 | 1.00 | 0.158 |
| atmos_froxel_inject | 0.159 | 0.160 | 0.154 | 1.00 | 0.002 |
| clouds_trace | 0.145 | 0.145 | 0.234 | 1.00 | 0.001 |
| water_fft | 0.139 | 0.139 | 0.138 | 1.00 | 0.003 |
| upscaling | 0.125 | 0.125 | 0.145 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.111 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.095 | 0.096 | 0.176 | 1.00 | 0.009 |
| tonemapping | 0.092 | 0.092 | 0.093 | 1.00 | 0.004 |
| clustering | 0.067 | 0.067 | 0.069 | 1.00 | 0.021 |
| render_sky | 0.056 | 0.056 | 0.058 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.055 | 0.446 | 0.755 | 0.12 | 0.001 |
| surface_trail | 0.052 | 0.059 | 0.057 | 0.87 | 0.002 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.044 | 0.044 | 0.048 | 1.00 | 0.006 |
| water_snapshot | 0.037 | 0.037 | 0.045 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.004 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.029 | 0.231 | 0.236 | 0.12 | 0.005 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.024 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 1.00 | 0.113 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.002 | 0.012 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.007 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 62.810 | 0.000 | 12438.50 | 290747.6 |
| render/thread/ms | 12.945 | 16.633 | 38.40 | 59922.1 |
| render/busy/ms | 12.372 | 16.015 | 37.46 | 57268.2 |
| main/busy/ms | 9.671 | 12.488 | 28.78 | 44766.0 |
| render/graph/ms | 7.456 | 9.820 | 19.80 | 34513.3 |
| pipelines/compile_ms | 0.843 | 0.000 | 162.90 | 3902.6 |
| pipelines/front_ms | 0.813 | 0.000 | 157.69 | 3765.1 |
| render/present/ms | 0.481 | 0.568 | 11.16 | 2226.3 |
| bigspace/total/ms | 0.451 | 0.699 | 3.93 | 2085.7 |
| terrain/update/ms | 0.292 | 0.367 | 1.78 | 1352.6 |
| controller/feed/ms | 0.289 | 0.387 | 1.14 | 1337.4 |
| stream/drive/ms | 0.262 | 0.384 | 20.85 | 1211.5 |
| terrain/select/ms | 0.259 | 0.326 | 0.64 | 1200.3 |
| far/select/ms | 0.206 | 0.294 | 0.55 | 953.8 |
| stream/tick/ms | 0.198 | 0.279 | 0.77 | 917.8 |
| bigspace/low_precision/ms | 0.169 | 0.305 | 3.67 | 784.1 |
| anim/evaluate_cpu/ms | 0.162 | 0.205 | 0.51 | 748.7 |
| anim/evaluate/ms | 0.130 | 0.292 | 2.66 | 600.2 |
| controller/move/ms | 0.124 | 0.223 | 0.56 | 572.0 |
| bigspace/high_precision/ms | 0.107 | 0.220 | 3.25 | 497.1 |
| render/acquire/ms | 0.092 | 0.123 | 0.29 | 427.5 |
| physics/insert/ms | 0.079 | 0.490 | 1.18 | 367.1 |
| physics/step/ms | 0.062 | 0.096 | 0.76 | 287.4 |
| pipelines/driver_ms | 0.030 | 0.000 | 5.21 | 137.5 |
| weather/map/ms | 0.025 | 0.035 | 0.20 | 117.0 |
| anim/apply/ms | 0.012 | 0.024 | 0.15 | 56.3 |
| controller/avoid/ms | 0.010 | 0.014 | 0.28 | 48.5 |
| weather/eval/ms | 0.010 | 0.014 | 0.19 | 48.1 |
| script/update/ms | 0.008 | 0.010 | 0.07 | 35.0 |
| stream/despawn/ms | 0.007 | 0.000 | 1.34 | 30.3 |
| stream/spawn/ms | 0.006 | 0.000 | 1.06 | 27.1 |
| script/input/ms | 0.005 | 0.007 | 0.08 | 24.7 |
| script/ui/ms | 0.005 | 0.006 | 0.03 | 20.9 |
| controller/write/ms | 0.003 | 0.004 | 0.02 | 16.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.66 | 14.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 4.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1277 | 2262 | 535 |
| crowd/c0/meshes/count | count | 352 | 646 | 144 |
| crowd/c1/count | count | 146 | 156 | 149 |
| crowd/c1/joints/count | count | 5510 | 5944 | 5661 |
| crowd/c1/meshes/count | count | 1163 | 1281 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4277 | 10.2 k | 6179 |
| crowd/joints_uploaded/count | count | 4247 | 6643 | 6179 |
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
| frame/work/ms | ms | 11.81 | 37.46 | 10.81 |
| gpu/timed/ms | ms | 8.66 | 13.31 | 8.77 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 806 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 806 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 37 | 41 | 37 |
| phases/opaque3d/bins | count | 1291 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3391 | 5154 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 37 | 41 | 37 |
| phases/opaque3d_prepass/bins | count | 1291 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3391 | 5154 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 70 | 335 | 58 |
| phases/shadow/bins | count | 3779 | 5482 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9429 | 12.9 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 144 | 317 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2631 | 3268 | 2347 |
| physics/items | count | 137 | 476 | 19 |
| physics/pending | count | 0 | 1367 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 56 | 0 |
| preprocess/bind_groups | count | 13 | 915 | 6 |
| preprocess/bind_groups_created | count | 13 | 915 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 315 | 468 | 309 |
| process/cpu_usage | % | 22.60 | 43.05 | 19.71 |
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
| started_load_count |  | 4768 | 4862 | 4862 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.11 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.36 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.95 | 50.94 | 27.69 |
| system/mem_usage | % | 70.23 | 70.79 | 70.73 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 1728 | 1802 | 1802 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 158 | 425 | 146 |
| views/all/bins | count | 6430 | 8317 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.4 k | 13.3 k |
| views/all/pipelines | count | 208 | 382 | 113 |
| views/all/pipelines_distinct | count | 96 | 109 | 91 |
| views/all/work_items | count | 17.2 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 69 | 91 | 58 |
| views/cascade/bins | count | 2601 | 3188 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8141 | 10.2 k | 5663 |
| views/cascade/pipelines | count | 56 | 74 | 47 |
| views/cascade/work_items | count | 8141 | 10.2 k | 5663 |
| views/lamp_face/batch_sets | count | 0 | 258 | 0 |
| views/lamp_face/bins | count | 1071 | 2508 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1216 | 2884 | 0 |
| views/lamp_face/pipelines | count | 86 | 247 | 0 |
| views/lamp_face/work_items | count | 0 | 2606 | 0 |
| views/main/batch_sets | count | 88 | 98 | 88 |
| views/main/bins | count | 2876 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8380 | 13.1 k | 7594 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8548 | 13.3 k | 7902 |
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
| anim/bones_written/count | count | 54.00 | 302 | 250.0 k |
| anim/events/count | count | 0.13 | 3 | 589 |
| assets/Image/added | count | 0.13 | 43 | 620 |
| assets/Image/modified | count | 0.01 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.91 | 152 | 4209 |
| assets/Mesh/removed | count | 1.05 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 496 |
| assets/StandardMaterial/modified | count | 4.68 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.27 | 1386 | 5876 |
| physics/inserts | count | 0.77 | 140 | 3568 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.50 | 145 | 2316 |
| physics/steps | count | 0.85 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.01 | 6 | 24 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 17 | 378 |
| stream/upload/bytes | bytes | 7313 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1603 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 33.9 ms; slowest 15, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2521.2 | main/busy/ms 2494, surface/advance_cpu_ms 68.45, surface/cell_upload_ms 0.72, stream/drive/ms 0.33, weather/map/ms 0.15, bigspace/total/ms 0.10, stream/tick/ms 0.09, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 2.98 | 455.0 | main/busy/ms 2529, render/thread/ms 418, render/busy/ms 334, render/acquire/ms 82.48, surface/advance_cpu_ms 68.45, render/graph/ms 4.92, render/present/ms 1.93, stream/drive/ms 1.11, terrain/update/ms 0.77, surface/cell_upload_ms 0.72, terrain/select/ms 0.58, stream/tick/ms 0.57 |
| warm-up | 3.09 | 111.3 | pipelines/wait_ms 2311, render/thread/ms 431, render/busy/ms 345, pipelines/compile_ms 121, pipelines/front_ms 110, render/acquire/ms 82.78, main/busy/ms 53.01, terrain/update/ms 18.90, pipelines/driver_ms 10.74, render/graph/ms 8.75, render/present/ms 3.26, stream/drive/ms 1.37 |
| warm-up | 3.33 | 90.1 | main/busy/ms 63.00, render/thread/ms 61.95, render/busy/ms 57.22, render/graph/ms 4.24, render/acquire/ms 3.41, render/present/ms 1.32, stream/drive/ms 1.01, stream/tick/ms 0.80, terrain/update/ms 0.70, terrain/select/ms 0.59, bigspace/total/ms 0.57, bigspace/high_precision/ms 0.29 |
| warm-up | 3.47 | 44.7 | render/thread/ms 43.60, render/busy/ms 42.44, main/busy/ms 42.21, stream/drive/ms 30.73, render/graph/ms 2.32, stream/tick/ms 1.55, render/present/ms 0.91, terrain/update/ms 0.60, terrain/select/ms 0.49, render/acquire/ms 0.25, bigspace/total/ms 0.19, weather/map/ms 0.10 |
| warm-up | 3.62 | 53.8 | pipelines/wait_ms 5459, pipelines/compile_ms 250, pipelines/front_ms 223, render/thread/ms 70.06, render/busy/ms 67.89, main/busy/ms 30.56, pipelines/driver_ms 27.38, render/graph/ms 7.14, stream/drive/ms 5.54, stream/spawn/ms 2.31, render/present/ms 1.89, stream/tick/ms 1.32 |
| warm-up | 3.74 | 88.3 | pipelines/wait_ms 2624, render/thread/ms 85.94, pipelines/compile_ms 70.17, pipelines/front_ms 66.63, render/busy/ms 58.29, main/busy/ms 56.60, stream/drive/ms 30.57, render/present/ms 27.21, render/graph/ms 6.19, terrain/update/ms 6.14, pipelines/driver_ms 3.54, stream/spawn/ms 2.88 |
| warm-up | 3.89 | 52.0 | pipelines/wait_ms 4579, pipelines/compile_ms 63.25, main/busy/ms 57.96, pipelines/front_ms 57.72, stream/drive/ms 44.32, render/thread/ms 36.82, render/busy/ms 34.85, render/graph/ms 8.35, pipelines/driver_ms 5.52, render/present/ms 1.62, stream/spawn/ms 1.11, terrain/update/ms 0.77 |
| warm-up | 3.96 | 44.9 | pipelines/wait_ms 760, pipelines/compile_ms 71.12, pipelines/front_ms 65.98, main/busy/ms 53.97, render/thread/ms 42.19, render/busy/ms 40.75, stream/drive/ms 37.79, render/graph/ms 16.07, pipelines/driver_ms 5.14, stream/spawn/ms 2.15, render/present/ms 1.23, stream/tick/ms 0.79 |
| warm-up | 4.02 | 47.3 | main/busy/ms 53.37, stream/drive/ms 36.47, render/thread/ms 24.34, render/busy/ms 23.24, render/graph/ms 4.38, stream/spawn/ms 1.48, stream/activate/ms 1.09, render/present/ms 0.93, stream/tick/ms 0.81, terrain/update/ms 0.60, terrain/select/ms 0.51, physics/insert/ms 0.49 |
| warm-up | 4.07 | 34.5 | pipelines/wait_ms 5058, pipelines/compile_ms 142, pipelines/front_ms 137, main/busy/ms 39.53, stream/drive/ms 29.15, render/thread/ms 22.80, render/busy/ms 21.59, pipelines/driver_ms 5.58, render/graph/ms 4.86, stream/activate/ms 1.07, render/present/ms 1.02, stream/tick/ms 0.84 |
| warm-up | 4.54 | 41.7 | render/thread/ms 58.60, main/busy/ms 57.60, render/busy/ms 57.29, render/graph/ms 20.83, stream/drive/ms 6.43, render/present/ms 1.06, stream/tick/ms 0.88, terrain/update/ms 0.85, terrain/select/ms 0.72, bigspace/total/ms 0.71, anim/evaluate_cpu/ms 0.44, bigspace/high_precision/ms 0.31 |
| warm-up | 6.26 | 38.6 | main/busy/ms 48.04, render/thread/ms 40.27, render/busy/ms 38.83, render/graph/ms 7.37, stream/drive/ms 1.33, render/present/ms 1.21, bigspace/total/ms 1.14, anim/evaluate_cpu/ms 1.01, stream/tick/ms 0.66, bigspace/low_precision/ms 0.61, terrain/update/ms 0.58, anim/evaluate/ms 0.56 |
| measure | 2.74 | 39.8 | render/thread/ms 53.12, render/busy/ms 51.59, render/graph/ms 26.93, main/busy/ms 20.94, physics/insert/ms 1.37, render/present/ms 1.35, bigspace/total/ms 0.74, controller/feed/ms 0.59, terrain/update/ms 0.55, terrain/select/ms 0.49, stream/drive/ms 0.43, stream/tick/ms 0.37 |
| measure | 19.28 | 34.1 | render/thread/ms 46.26, render/busy/ms 44.93, render/graph/ms 27.89, main/busy/ms 22.90, render/present/ms 1.16, bigspace/total/ms 0.78, terrain/update/ms 0.53, controller/feed/ms 0.50, stream/drive/ms 0.48, terrain/select/ms 0.48, far/select/ms 0.39, stream/tick/ms 0.37 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --texture-cap 512 --resolution 2560x1440 --bench-out target/perf\lean_cap512\balmora_walk@2560x1440_--texture-cap_512-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **73%** (min 50%) · 2580 MHz (min 2580) · memory 8001 MHz · 71.5 W · 73 °C (max 78) · VRAM 5312 MiB · throttled 100% of samples
