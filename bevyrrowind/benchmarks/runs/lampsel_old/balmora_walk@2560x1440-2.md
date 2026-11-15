# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5094 frames over 65.7 s · **median 12.41 ms (81 fps)** · mean 12.89 · p95 15.88 · p99 18.16 (1% low 55 fps) · worst 32.60 ms · 46 spikes over 1.5× median · 23.8 stutters/min (work 21.9/min)
- warm-up: 335 frames over 8.3 s, worst 2548.6 ms
- hitches (> 31.0 ms): **4** measured, 24 in warm-up
- timed GPU passes: 7.81 ms of a 12.89 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.41 ms, limit 11 ms
- **FAIL** frame_ms: 26 stutters (> 1.5x moving median of 60) in 1.09 min = 23.76/min, limit 1/min
- pass worst frame 32.60 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.159 | 2.183 | 2.491 | 0.99 | 0.180 |
| main_opaque_pass_3d | 1.713 | 1.732 | 2.361 | 0.99 | 0.060 |
| early prepass | 0.537 | 0.543 | 0.820 | 0.99 | 0.104 |
| bloom | 0.334 | 0.338 | 0.496 | 0.99 | 0.064 |
| atmosphere_luts | 0.326 | 0.329 | 0.508 | 0.99 | 0.003 |
| water_velocity | 0.300 | 0.303 | 0.540 | 0.99 | 0.001 |
| uniform_allocation | 0.263 | 0.266 | 0.269 | 0.99 | 0.048 |
| water_sspr | 0.218 | 0.220 | 0.221 | 0.99 | 0.002 |
| ssao | 0.202 | 0.204 | 0.209 | 0.99 | 0.008 |
| bin_unpacking | 0.198 | 0.200 | 0.212 | 0.99 | 0.030 |
| auxiliary_views | 0.182 | 0.183 | 0.686 | 0.99 | 0.159 |
| atmos_froxel_inject | 0.154 | 0.156 | 0.152 | 0.99 | 0.002 |
| clouds_trace | 0.139 | 0.141 | 0.229 | 0.99 | 0.001 |
| water_fft | 0.136 | 0.137 | 0.137 | 0.99 | 0.003 |
| upscaling | 0.125 | 0.126 | 0.157 | 0.99 | 0.004 |
| clouds_composite | 0.110 | 0.111 | 0.112 | 0.99 | 0.002 |
| tonemapping | 0.092 | 0.093 | 0.093 | 0.99 | 0.003 |
| main_transmissive_pass_3d | 0.090 | 0.091 | 0.169 | 0.99 | 0.008 |
| clustering | 0.066 | 0.067 | 0.069 | 0.99 | 0.019 |
| render_sky | 0.054 | 0.055 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.439 | 0.754 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.99 | 0.001 |
| surface_trail | 0.047 | 0.059 | 0.057 | 0.80 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.043 | 0.048 | 0.99 | 0.005 |
| water_snapshot | 0.036 | 0.037 | 0.044 | 0.99 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.246 | 0.560 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.026 | 0.030 | 0.99 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 0.99 | 0.006 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.033 | 0.99 | 0.109 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.99 | 0.001 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 39.557 | 0.000 | 14241.01 | 201504.4 |
| render/thread/ms | 11.700 | 14.569 | 31.65 | 59597.7 |
| render/busy/ms | 11.116 | 13.972 | 30.71 | 56626.0 |
| main/busy/ms | 9.117 | 11.789 | 28.80 | 46442.5 |
| render/graph/ms | 6.787 | 8.763 | 24.75 | 34570.7 |
| pipelines/compile_ms | 0.671 | 0.000 | 150.54 | 3419.0 |
| pipelines/front_ms | 0.648 | 0.000 | 144.53 | 3300.2 |
| render/present/ms | 0.492 | 0.581 | 16.70 | 2505.9 |
| bigspace/total/ms | 0.465 | 0.750 | 4.75 | 2370.3 |
| terrain/update/ms | 0.294 | 0.371 | 2.13 | 1495.6 |
| controller/feed/ms | 0.267 | 0.359 | 0.70 | 1361.2 |
| terrain/select/ms | 0.261 | 0.330 | 0.68 | 1328.8 |
| stream/drive/ms | 0.249 | 0.357 | 20.65 | 1267.8 |
| far/select/ms | 0.190 | 0.260 | 0.57 | 966.1 |
| stream/tick/ms | 0.189 | 0.266 | 0.63 | 963.4 |
| bigspace/low_precision/ms | 0.186 | 0.318 | 4.50 | 947.5 |
| anim/evaluate_cpu/ms | 0.167 | 0.207 | 0.45 | 849.8 |
| controller/move/ms | 0.138 | 0.272 | 0.60 | 702.5 |
| anim/evaluate/ms | 0.133 | 0.275 | 3.31 | 679.6 |
| bigspace/high_precision/ms | 0.114 | 0.224 | 3.66 | 578.4 |
| render/acquire/ms | 0.091 | 0.117 | 0.36 | 465.8 |
| physics/insert/ms | 0.062 | 0.473 | 1.26 | 313.8 |
| physics/step/ms | 0.057 | 0.088 | 0.95 | 290.9 |
| weather/map/ms | 0.024 | 0.032 | 0.22 | 121.6 |
| pipelines/driver_ms | 0.023 | 0.000 | 6.01 | 118.8 |
| anim/apply/ms | 0.011 | 0.020 | 0.19 | 58.3 |
| controller/avoid/ms | 0.011 | 0.015 | 0.22 | 55.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.04 | 52.3 |
| script/update/ms | 0.007 | 0.009 | 0.16 | 33.8 |
| stream/despawn/ms | 0.006 | 0.000 | 1.18 | 30.1 |
| script/input/ms | 0.005 | 0.007 | 0.20 | 26.3 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 25.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 25.3 |
| controller/write/ms | 0.004 | 0.005 | 0.19 | 18.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.64 | 14.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.30 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 14 | 0 |
| anim/a2/count | count | 0 | 12 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 31 | 20 |
| controller/movers/count | count | 20 | 23 | 20 |
| crowd/c0/count | count | 26 | 42 | 13 |
| crowd/c0/joints/count | count | 1301 | 2218 | 593 |
| crowd/c0/meshes/count | count | 353 | 633 | 156 |
| crowd/c1/count | count | 145 | 154 | 148 |
| crowd/c1/joints/count | count | 5445 | 5881 | 5603 |
| crowd/c1/meshes/count | count | 1150 | 1241 | 1185 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1821 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4266 | 13.2 k | 6179 |
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
| frame/work/ms | ms | 10.73 | 30.71 | 9.53 |
| gpu/timed/ms | ms | 8.61 | 12.70 | 9.07 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 804 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 804 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d/bins | count | 1289 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3393 | 5112 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1289 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3393 | 5112 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 59 | 293 | 47 |
| phases/shadow/bins | count | 3740 | 5417 | 2236 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9392 | 12.4 k | 5687 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 146 | 311 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2349 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1706 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 47 | 0 |
| preprocess/bind_groups | count | 12 | 779 | 6 |
| preprocess/bind_groups_created | count | 12 | 779 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 259 | 379 | 251 |
| process/cpu_usage | % | 24.04 | 47.37 | 19.71 |
| process/mem_usage | GiB | 5.12 | 5.29 | 5.29 |
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
| started_load_count |  | 4764 | 4858 | 4858 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.35 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.36 | 43.34 | 27.98 |
| system/mem_usage | % | 63.79 | 64.40 | 64.40 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 128 | 357 | 117 |
| views/all/bins | count | 6306 | 8324 | 5716 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.0 k | 13.3 k |
| views/all/pipelines | count | 210 | 371 | 113 |
| views/all/pipelines_distinct | count | 97 | 109 | 91 |
| views/all/work_items | count | 17.2 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 55 | 72 | 47 |
| views/cascade/bins | count | 2594 | 3188 | 2236 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8123 | 10.0 k | 5687 |
| views/cascade/pipelines | count | 55 | 72 | 47 |
| views/cascade/work_items | count | 8123 | 10.0 k | 5687 |
| views/lamp_face/batch_sets | count | 0 | 233 | 0 |
| views/lamp_face/bins | count | 1074 | 2472 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1197 | 2825 | 0 |
| views/lamp_face/pipelines | count | 87 | 243 | 0 |
| views/lamp_face/work_items | count | 0 | 2553 | 0 |
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 2878 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8391 | 13.0 k | 7592 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8559 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 56.55 | 567 | 288.1 k |
| anim/events/count | count | 0.09 | 3 | 470 |
| assets/Image/added | count | 0.13 | 43 | 649 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.84 | 152 | 4302 |
| assets/Mesh/removed | count | 0.96 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 36 | 525 |
| assets/StandardMaterial/modified | count | 4.25 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.94 | 1587 | 9899 |
| physics/inserts | count | 0.76 | 189 | 3869 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.46 | 145 | 2335 |
| physics/steps | count | 0.77 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.01 | 6 | 26 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 6646 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1457 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.0 ms; slowest 28, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2548.6 | main/busy/ms 2523, surface/advance_cpu_ms 67.07, surface/cell_upload_ms 0.69, stream/drive/ms 0.31, weather/map/ms 0.13, stream/tick/ms 0.10, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.03, script/input/ms 0.01, script/ui/ms 0.01 |
| warm-up | 3.24 | 693.7 | main/busy/ms 2561, render/thread/ms 661, render/busy/ms 524, render/acquire/ms 134, surface/advance_cpu_ms 67.07, render/graph/ms 6.56, render/present/ms 2.28, stream/drive/ms 1.12, surface/cell_upload_ms 0.69, stream/tick/ms 0.58, terrain/update/ms 0.39, bigspace/total/ms 0.27 |
| warm-up | 3.60 | 355.1 | pipelines/wait_ms 5393, render/thread/ms 673, render/busy/ms 535, pipelines/compile_ms 367, pipelines/front_ms 342, render/acquire/ms 135, main/busy/ms 54.20, pipelines/driver_ms 24.46, terrain/update/ms 19.41, render/graph/ms 10.68, render/present/ms 3.33, stream/drive/ms 1.29 |
| warm-up | 3.81 | 87.4 | main/busy/ms 63.07, render/thread/ms 46.18, render/busy/ms 44.65, render/graph/ms 3.55, render/present/ms 1.31, stream/drive/ms 1.26, stream/tick/ms 0.99, terrain/update/ms 0.71, terrain/select/ms 0.57, bigspace/total/ms 0.26, render/acquire/ms 0.22, weather/map/ms 0.11 |
| warm-up | 3.91 | 31.9 | pipelines/wait_ms 252, render/thread/ms 40.97, main/busy/ms 40.82, render/busy/ms 38.94, stream/drive/ms 22.11, pipelines/compile_ms 10.10, pipelines/front_ms 9.24, render/graph/ms 5.20, render/present/ms 1.79, stream/tick/ms 1.00, terrain/update/ms 0.88, pipelines/driver_ms 0.85 |
| warm-up | 3.95 | 32.3 | render/thread/ms 49.38, main/busy/ms 48.81, render/busy/ms 47.71, stream/drive/ms 32.84, render/graph/ms 5.05, terrain/update/ms 4.63, render/present/ms 1.52, stream/tick/ms 0.85, terrain/select/ms 0.55, stream/spawn/ms 0.45, bigspace/total/ms 0.20, render/acquire/ms 0.16 |
| warm-up | 4.02 | 48.0 | render/thread/ms 52.14, render/busy/ms 50.27, main/busy/ms 40.06, stream/drive/ms 23.81, render/graph/ms 5.39, stream/tick/ms 2.23, render/present/ms 1.50, stream/spawn/ms 1.26, terrain/update/ms 0.68, terrain/select/ms 0.56, bigspace/total/ms 0.41, render/acquire/ms 0.36 |
| warm-up | 4.05 | 35.8 | render/thread/ms 57.30, render/busy/ms 48.06, main/busy/ms 29.09, render/present/ms 8.81, stream/drive/ms 6.72, render/graph/ms 6.56, stream/tick/ms 2.89, stream/spawn/ms 2.28, terrain/update/ms 0.91, terrain/select/ms 0.73, bigspace/total/ms 0.51, render/acquire/ms 0.43 |
| warm-up | 4.11 | 60.9 | pipelines/wait_ms 2566, pipelines/compile_ms 260, pipelines/front_ms 231, render/thread/ms 58.48, render/busy/ms 49.41, main/busy/ms 34.05, pipelines/driver_ms 28.31, render/present/ms 8.73, render/graph/ms 6.25, stream/drive/ms 5.47, stream/tick/ms 2.13, stream/spawn/ms 1.83 |
| warm-up | 4.16 | 43.2 | pipelines/wait_ms 2566, pipelines/compile_ms 260, pipelines/front_ms 231, render/thread/ms 64.89, render/busy/ms 63.19, main/busy/ms 31.57, pipelines/driver_ms 28.31, render/graph/ms 5.61, stream/drive/ms 4.32, render/present/ms 1.49, stream/tick/ms 1.42, terrain/update/ms 0.88 |
| warm-up | 4.20 | 41.3 | render/thread/ms 65.76, render/busy/ms 64.40, main/busy/ms 22.26, render/graph/ms 5.55, stream/drive/ms 5.35, stream/spawn/ms 1.86, terrain/update/ms 1.14, render/present/ms 1.13, stream/activate/ms 1.12, terrain/select/ms 0.96, stream/tick/ms 0.96, bigspace/total/ms 0.30 |
| warm-up | 4.25 | 34.9 | pipelines/wait_ms 894, pipelines/compile_ms 148, pipelines/front_ms 144, main/busy/ms 49.29, render/thread/ms 32.10, render/busy/ms 30.82, stream/drive/ms 30.03, pipelines/driver_ms 3.81, render/graph/ms 3.80, stream/spawn/ms 1.64, render/present/ms 1.08, stream/tick/ms 0.96 |
| warm-up | 4.29 | 40.0 | pipelines/wait_ms 2155, pipelines/compile_ms 150, pipelines/front_ms 146, main/busy/ms 50.23, render/thread/ms 38.29, render/busy/ms 37.15, stream/drive/ms 30.70, pipelines/driver_ms 4.14, render/graph/ms 3.27, stream/spawn/ms 1.48, stream/tick/ms 1.31, render/present/ms 0.96 |
| warm-up | 4.38 | 45.0 | main/busy/ms 76.17, stream/drive/ms 60.97, render/thread/ms 57.37, render/busy/ms 54.91, render/graph/ms 14.13, render/present/ms 2.14, stream/tick/ms 1.52, stream/spawn/ms 1.20, terrain/update/ms 1.01, terrain/select/ms 0.86, bigspace/total/ms 0.33, render/acquire/ms 0.33 |
| warm-up | 4.46 | 78.5 | pipelines/wait_ms 422, pipelines/compile_ms 144, pipelines/front_ms 136, render/thread/ms 76.58, render/busy/ms 74.01, main/busy/ms 63.39, stream/drive/ms 48.75, render/graph/ms 14.04, pipelines/driver_ms 7.55, render/present/ms 2.28, stream/tick/ms 1.29, terrain/update/ms 1.01 |
| warm-up | 4.53 | 66.6 | pipelines/wait_ms 2577, pipelines/compile_ms 162, pipelines/front_ms 147, main/busy/ms 62.93, stream/drive/ms 43.63, render/thread/ms 27.11, render/busy/ms 25.80, pipelines/driver_ms 15.39, render/graph/ms 4.29, stream/spawn/ms 2.11, render/present/ms 1.13, terrain/update/ms 1.01 |
| warm-up | 4.60 | 44.9 | pipelines/wait_ms 2538, main/busy/ms 46.59, render/thread/ms 36.82, render/busy/ms 35.35, stream/drive/ms 32.88, pipelines/compile_ms 14.74, pipelines/front_ms 13.40, render/graph/ms 4.38, stream/spawn/ms 1.58, pipelines/driver_ms 1.34, render/present/ms 1.19, stream/tick/ms 0.89 |
| warm-up | 4.64 | 31.2 | main/busy/ms 35.24, stream/drive/ms 23.41, render/thread/ms 19.51, render/busy/ms 18.41, render/graph/ms 4.36, stream/activate/ms 1.22, render/present/ms 0.89, stream/tick/ms 0.84, stream/spawn/ms 0.71, terrain/update/ms 0.70, terrain/select/ms 0.57, physics/insert/ms 0.46 |
| warm-up | 5.09 | 33.7 | main/busy/ms 34.45, render/thread/ms 34.35, render/busy/ms 32.56, render/graph/ms 8.54, terrain/update/ms 4.68, stream/drive/ms 1.75, render/present/ms 1.41, stream/tick/ms 0.95, bigspace/total/ms 0.52, terrain/select/ms 0.52, render/acquire/ms 0.38, anim/evaluate_cpu/ms 0.38 |
| warm-up | 5.36 | 33.8 | pipelines/wait_ms 2320, pipelines/compile_ms 103, pipelines/front_ms 98.40, render/thread/ms 48.53, render/busy/ms 46.71, main/busy/ms 22.80, render/graph/ms 8.52, pipelines/driver_ms 5.06, stream/drive/ms 4.13, stream/spawn/ms 1.62, render/present/ms 1.54, terrain/update/ms 0.73 |
| warm-up | 5.43 | 40.0 | render/thread/ms 47.35, render/busy/ms 46.11, main/busy/ms 25.33, render/graph/ms 5.11, stream/drive/ms 4.83, stream/spawn/ms 2.03, stream/tick/ms 1.59, render/present/ms 1.00, physics/insert/ms 0.76, terrain/update/ms 0.65, bigspace/total/ms 0.55, terrain/select/ms 0.54 |
| warm-up | 5.75 | 32.6 | main/busy/ms 31.78, render/thread/ms 27.81, render/busy/ms 26.61, stream/drive/ms 13.87, render/graph/ms 4.48, stream/spawn/ms 2.04, physics/insert/ms 1.21, render/present/ms 0.96, terrain/update/ms 0.77, stream/tick/ms 0.77, terrain/select/ms 0.63, bigspace/total/ms 0.44 |
| warm-up | 5.92 | 54.8 | pipelines/wait_ms 10.3 k, pipelines/compile_ms 152, pipelines/front_ms 139, main/busy/ms 52.40, render/thread/ms 36.33, render/busy/ms 27.72, pipelines/driver_ms 13.69, stream/drive/ms 9.17, render/graph/ms 8.71, render/present/ms 8.30, stream/spawn/ms 2.00, physics/insert/ms 1.50 |
| warm-up | 5.99 | 46.2 | pipelines/wait_ms 1310, main/busy/ms 46.75, render/thread/ms 44.59, render/busy/ms 42.59, pipelines/compile_ms 9.86, render/graph/ms 8.89, stream/drive/ms 8.34, pipelines/front_ms 6.42, pipelines/driver_ms 3.44, render/present/ms 1.79, physics/insert/ms 1.08, stream/tick/ms 0.69 |
| measure | 2.73 | 32.6 | render/thread/ms 43.11, render/busy/ms 41.65, render/graph/ms 21.16, main/busy/ms 19.43, render/present/ms 1.26, bigspace/total/ms 0.90, stream/drive/ms 0.70, physics/insert/ms 0.69, stream/tick/ms 0.64, terrain/update/ms 0.63, terrain/select/ms 0.56, controller/feed/ms 0.51 |
| measure | 19.28 | 31.5 | render/thread/ms 43.50, render/busy/ms 42.17, render/graph/ms 26.85, main/busy/ms 19.50, render/present/ms 1.13, bigspace/total/ms 0.79, controller/feed/ms 0.67, terrain/update/ms 0.54, terrain/select/ms 0.49, stream/drive/ms 0.49, far/select/ms 0.45, stream/tick/ms 0.38 |
| measure | 23.38 | 31.3 | render/thread/ms 44.31, render/busy/ms 42.97, render/graph/ms 33.24, main/busy/ms 21.48, render/present/ms 1.14, bigspace/total/ms 0.80, terrain/update/ms 0.59, terrain/select/ms 0.53, controller/feed/ms 0.53, physics/insert/ms 0.53, far/select/ms 0.52, stream/drive/ms 0.38 |
| measure | 24.91 | 31.5 | render/thread/ms 44.45, main/busy/ms 27.30, render/busy/ms 27.03, render/graph/ms 17.66, render/present/ms 17.23, bigspace/total/ms 1.00, stream/drive/ms 0.65, terrain/update/ms 0.58, stream/tick/ms 0.58, controller/feed/ms 0.53, terrain/select/ms 0.52, far/select/ms 0.52 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_old.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lampsel_old\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 11%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **77%** (min 53%) · 2580 MHz (min 2565) · memory 8001 MHz · 76.5 W · 76 °C (max 80) · VRAM 5278 MiB · throttled 100% of samples
