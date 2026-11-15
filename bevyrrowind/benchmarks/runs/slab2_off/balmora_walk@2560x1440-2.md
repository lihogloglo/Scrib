# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4780 frames over 65.6 s · **median 13.40 ms (75 fps)** · mean 13.73 · p95 16.43 · p99 18.85 (1% low 53 fps) · worst 49.05 ms · 27 spikes over 1.5× median · 24.7 stutters/min (work 20.1/min)
- warm-up: 341 frames over 8.2 s, worst 2513.0 ms
- hitches (> 33.5 ms): **4** measured, 15 in warm-up
- timed GPU passes: 7.96 ms of a 13.73 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.40 ms, limit 11 ms
- **FAIL** frame_ms: 27 stutters (> 1.5x moving median of 60) in 1.09 min = 24.68/min, limit 1/min
- **FAIL** worst frame 49.05 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.179 | 2.193 | 2.567 | 0.99 | 0.182 |
| main_opaque_pass_3d | 1.750 | 1.760 | 2.421 | 0.99 | 0.086 |
| early prepass | 0.540 | 0.543 | 0.777 | 0.99 | 0.124 |
| bloom | 0.339 | 0.341 | 0.503 | 0.99 | 0.067 |
| atmosphere_luts | 0.325 | 0.327 | 0.470 | 0.99 | 0.004 |
| uniform_allocation | 0.313 | 0.315 | 0.609 | 0.99 | 0.059 |
| water_velocity | 0.301 | 0.303 | 0.549 | 0.99 | 0.002 |
| bin_unpacking | 0.238 | 0.240 | 0.262 | 0.99 | 0.040 |
| water_sspr | 0.221 | 0.222 | 0.219 | 0.99 | 0.002 |
| ssao | 0.204 | 0.205 | 0.210 | 0.99 | 0.008 |
| atmos_froxel_inject | 0.156 | 0.157 | 0.153 | 0.99 | 0.002 |
| auxiliary_views | 0.152 | 0.153 | 0.641 | 0.99 | 0.139 |
| clouds_trace | 0.140 | 0.141 | 0.229 | 0.99 | 0.001 |
| water_fft | 0.139 | 0.140 | 0.141 | 0.99 | 0.003 |
| upscaling | 0.125 | 0.126 | 0.137 | 0.99 | 0.005 |
| clouds_composite | 0.110 | 0.111 | 0.111 | 0.99 | 0.002 |
| main_transmissive_pass_3d | 0.094 | 0.094 | 0.173 | 0.99 | 0.009 |
| tonemapping | 0.091 | 0.092 | 0.093 | 0.99 | 0.004 |
| clustering | 0.071 | 0.072 | 0.069 | 0.99 | 0.020 |
| render_sky | 0.054 | 0.055 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.053 | 0.437 | 0.749 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 0.99 | 0.001 |
| surface_trail | 0.049 | 0.058 | 0.057 | 0.85 | 0.002 |
| early_mesh_preprocessing | 0.044 | 0.044 | 0.048 | 0.99 | 0.006 |
| water_snapshot | 0.036 | 0.036 | 0.044 | 0.99 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.029 | 0.234 | 0.237 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.031 | 0.99 | 0.001 |
| per_view_shadow_pass | 0.023 | 0.023 | 0.024 | 0.99 | 0.007 |
| water_mips | 0.023 | 0.023 | 0.024 | 0.99 | 0.003 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 0.99 | 0.110 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 59.101 | 0.000 | 16652.14 | 282502.1 |
| render/thread/ms | 12.528 | 15.079 | 47.72 | 59881.6 |
| render/busy/ms | 11.927 | 14.440 | 33.37 | 57010.5 |
| main/busy/ms | 9.488 | 12.135 | 29.82 | 45351.0 |
| render/graph/ms | 7.267 | 9.030 | 18.32 | 34735.6 |
| pipelines/compile_ms | 0.801 | 0.000 | 154.65 | 3828.5 |
| pipelines/front_ms | 0.771 | 0.000 | 148.36 | 3687.8 |
| render/present/ms | 0.510 | 0.602 | 34.19 | 2437.3 |
| bigspace/total/ms | 0.446 | 0.677 | 4.16 | 2133.3 |
| terrain/update/ms | 0.290 | 0.366 | 1.72 | 1387.7 |
| controller/feed/ms | 0.266 | 0.358 | 0.77 | 1272.8 |
| stream/drive/ms | 0.262 | 0.379 | 22.14 | 1251.9 |
| terrain/select/ms | 0.257 | 0.325 | 0.71 | 1229.6 |
| stream/tick/ms | 0.199 | 0.273 | 0.96 | 950.3 |
| far/select/ms | 0.194 | 0.275 | 0.64 | 925.0 |
| bigspace/low_precision/ms | 0.169 | 0.288 | 3.69 | 806.2 |
| anim/evaluate_cpu/ms | 0.159 | 0.204 | 0.49 | 759.3 |
| anim/evaluate/ms | 0.148 | 0.295 | 3.46 | 708.7 |
| controller/move/ms | 0.137 | 0.362 | 0.70 | 652.7 |
| bigspace/high_precision/ms | 0.105 | 0.202 | 3.61 | 500.4 |
| render/acquire/ms | 0.091 | 0.117 | 0.42 | 433.9 |
| physics/step/ms | 0.061 | 0.091 | 1.13 | 290.9 |
| physics/insert/ms | 0.051 | 0.428 | 0.81 | 243.3 |
| pipelines/driver_ms | 0.029 | 0.000 | 6.29 | 140.7 |
| weather/map/ms | 0.024 | 0.032 | 0.24 | 114.4 |
| anim/apply/ms | 0.011 | 0.020 | 0.16 | 53.3 |
| controller/avoid/ms | 0.010 | 0.014 | 0.19 | 48.7 |
| weather/eval/ms | 0.010 | 0.014 | 0.04 | 47.7 |
| script/update/ms | 0.007 | 0.009 | 0.13 | 35.1 |
| stream/despawn/ms | 0.006 | 0.000 | 1.02 | 29.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 25.9 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 24.7 |
| script/ui/ms | 0.004 | 0.006 | 0.16 | 21.3 |
| controller/write/ms | 0.003 | 0.004 | 0.05 | 16.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.70 | 15.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.45 | 5.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 4.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.2 |
| weather/apply/ms | 0.000 | 0.000 | 0.01 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 10 | 0 |
| anim/a2/count | count | 1 | 15 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 540 |
| crowd/c0/meshes/count | count | 353 | 646 | 131 |
| crowd/c1/count | count | 146 | 155 | 149 |
| crowd/c1/joints/count | count | 5507 | 5936 | 5656 |
| crowd/c1/meshes/count | count | 1165 | 1276 | 1210 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4266 | 16.7 k | 6179 |
| crowd/joints_uploaded/count | count | 4243 | 6643 | 6179 |
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
| frame/work/ms | ms | 11.67 | 33.37 | 10.30 |
| gpu/timed/ms | ms | 8.66 | 12.80 | 9.11 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 3072 | 3142 | 3142 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 2 | 2 | 2 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 277.8 k | 282.2 k | 282.2 k |
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
| phases/alpha_mask3d/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 34 | 40 | 33 |
| phases/opaque3d/bins | count | 1288 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3405 | 5117 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 34 | 40 | 33 |
| phases/opaque3d_prepass/bins | count | 1288 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3405 | 5117 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 71 | 309 | 57 |
| phases/shadow/bins | count | 3754 | 5275 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9342 | 12.3 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 140 | 299 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2348 |
| physics/items | count | 189 | 476 | 19 |
| physics/pending | count | 0 | 2310 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 226 | 232 | 232 |
| pipelines/waiting | count | 0 | 55 | 0 |
| preprocess/bind_groups | count | 12 | 855 | 6 |
| preprocess/bind_groups_created | count | 12 | 855 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 305 | 452 | 291 |
| process/cpu_usage | % | 23.22 | 44.39 | 21.05 |
| process/mem_usage | GiB | 5.17 | 5.36 | 5.36 |
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
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.06 | 43.04 | 29.26 |
| system/mem_usage | % | 58.90 | 59.79 | 59.73 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 157 | 395 | 137 |
| views/all/bins | count | 6248 | 8044 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.0 k | 24.9 k | 13.3 k |
| views/all/pipelines | count | 205 | 359 | 113 |
| views/all/pipelines_distinct | count | 96 | 109 | 91 |
| views/all/work_items | count | 17.0 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 69 | 84 | 57 |
| views/cascade/bins | count | 2585 | 3113 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8141 | 9813 | 5663 |
| views/cascade/pipelines | count | 55 | 70 | 47 |
| views/cascade/work_items | count | 8141 | 9813 | 5663 |
| views/lamp_face/batch_sets | count | 0 | 236 | 0 |
| views/lamp_face/bins | count | 1058 | 2364 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1189 | 2732 | 0 |
| views/lamp_face/pipelines | count | 83 | 229 | 0 |
| views/lamp_face/work_items | count | 0 | 2474 | 0 |
| views/main/batch_sets | count | 84 | 96 | 80 |
| views/main/bins | count | 2866 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8428 | 13.0 k | 7594 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8588 | 13.2 k | 7902 |
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
| anim/bones_written/count | count | 47.72 | 342 | 228.1 k |
| anim/events/count | count | 0.10 | 3 | 495 |
| assets/Image/added | count | 0.14 | 43 | 662 |
| assets/Image/modified | count | 0.00 | 3 | 18 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.91 | 165 | 4341 |
| assets/Mesh/removed | count | 1.02 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 539 |
| assets/StandardMaterial/modified | count | 4.53 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.29 | 1642 | 11.0 k |
| physics/inserts | count | 0.86 | 155 | 4134 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.49 | 145 | 2335 |
| physics/steps | count | 0.82 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.01 | 6 | 26 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 18 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 17 | 378 |
| stream/upload/bytes | bytes | 7082 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1553 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 33.5 ms; slowest 19, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2513.0 | main/busy/ms 2486, surface/advance_cpu_ms 69.21, surface/cell_upload_ms 0.84, stream/drive/ms 0.60, stream/tick/ms 0.20, weather/map/ms 0.12, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.03, script/input/ms 0.02, far/select/ms 0.01 |
| warm-up | 3.05 | 537.1 | main/busy/ms 2518, render/thread/ms 504, render/busy/ms 382, render/acquire/ms 120, surface/advance_cpu_ms 69.21, render/graph/ms 5.26, render/present/ms 2.22, stream/drive/ms 1.41, surface/cell_upload_ms 0.84, stream/tick/ms 0.64, terrain/update/ms 0.51, terrain/select/ms 0.34 |
| warm-up | 3.26 | 209.4 | pipelines/wait_ms 1266, render/thread/ms 516, render/busy/ms 393, pipelines/compile_ms 223, pipelines/front_ms 209, render/acquire/ms 120, main/busy/ms 48.92, terrain/update/ms 16.43, pipelines/driver_ms 14.52, render/graph/ms 9.70, render/present/ms 2.97, stream/drive/ms 1.66 |
| warm-up | 3.50 | 83.3 | pipelines/compile_ms 229, pipelines/front_ms 218, main/busy/ms 57.27, render/thread/ms 51.19, render/busy/ms 49.64, pipelines/driver_ms 11.28, render/graph/ms 4.41, stream/drive/ms 1.92, stream/tick/ms 1.35, render/present/ms 1.33, terrain/update/ms 0.77, terrain/select/ms 0.62 |
| warm-up | 3.64 | 59.0 | main/busy/ms 44.26, render/thread/ms 34.23, render/busy/ms 32.98, stream/drive/ms 32.00, terrain/update/ms 6.96, render/graph/ms 3.45, render/present/ms 1.06, stream/tick/ms 0.88, terrain/select/ms 0.81, bigspace/total/ms 0.28, render/acquire/ms 0.18, bigspace/high_precision/ms 0.13 |
| warm-up | 3.76 | 38.7 | main/busy/ms 55.93, render/thread/ms 42.25, render/busy/ms 40.74, stream/drive/ms 34.02, render/graph/ms 4.39, stream/tick/ms 1.28, render/present/ms 1.27, stream/spawn/ms 1.09, terrain/update/ms 1.04, terrain/select/ms 0.89, stream/upload/ms 0.54, bigspace/total/ms 0.39 |
| warm-up | 3.82 | 58.2 | render/thread/ms 59.95, render/busy/ms 58.64, main/busy/ms 50.79, stream/drive/ms 32.67, render/graph/ms 4.73, render/present/ms 1.13, stream/tick/ms 1.02, terrain/update/ms 0.86, terrain/select/ms 0.72, stream/activate/ms 0.53, bigspace/total/ms 0.24, render/acquire/ms 0.18 |
| warm-up | 3.93 | 40.3 | pipelines/wait_ms 4040, pipelines/compile_ms 249, pipelines/front_ms 235, render/thread/ms 47.55, render/busy/ms 45.45, main/busy/ms 34.63, stream/drive/ms 18.65, pipelines/driver_ms 14.50, render/graph/ms 8.40, render/present/ms 1.83, stream/tick/ms 1.40, stream/spawn/ms 1.02 |
| warm-up | 4.00 | 50.6 | pipelines/wait_ms 2610, pipelines/compile_ms 77.23, pipelines/front_ms 73.36, render/thread/ms 46.35, render/busy/ms 44.80, main/busy/ms 27.09, stream/drive/ms 5.32, render/graph/ms 4.12, pipelines/driver_ms 3.87, stream/spawn/ms 2.33, stream/tick/ms 1.77, render/present/ms 1.24 |
| warm-up | 4.09 | 77.2 | main/busy/ms 78.83, stream/drive/ms 56.31, render/thread/ms 35.23, render/busy/ms 33.46, render/graph/ms 6.87, stream/spawn/ms 3.34, render/present/ms 1.49, stream/tick/ms 1.28, terrain/update/ms 0.96, terrain/select/ms 0.79, bigspace/total/ms 0.50, stream/upload/ms 0.36 |
| warm-up | 4.21 | 83.2 | pipelines/wait_ms 11.4 k, pipelines/compile_ms 238, pipelines/front_ms 215, main/busy/ms 84.25, stream/drive/ms 62.43, render/thread/ms 57.38, render/busy/ms 55.93, pipelines/driver_ms 23.44, render/graph/ms 5.20, stream/spawn/ms 2.09, stream/tick/ms 1.28, render/present/ms 1.06 |
| warm-up | 4.27 | 35.0 | main/busy/ms 49.19, render/thread/ms 36.81, stream/drive/ms 35.42, render/busy/ms 35.32, render/graph/ms 4.16, stream/spawn/ms 1.31, render/present/ms 1.21, stream/activate/ms 1.04, stream/tick/ms 0.84, terrain/update/ms 0.76, terrain/select/ms 0.63, physics/insert/ms 0.40 |
| warm-up | 5.59 | 41.4 | render/thread/ms 46.61, render/busy/ms 45.41, main/busy/ms 34.46, stream/drive/ms 17.26, render/graph/ms 6.01, stream/spawn/ms 1.42, render/present/ms 1.03, bigspace/total/ms 0.70, stream/tick/ms 0.67, terrain/update/ms 0.61, terrain/select/ms 0.51, far/select/ms 0.32 |
| warm-up | 6.72 | 56.9 | main/busy/ms 78.13, render/thread/ms 41.40, render/busy/ms 39.94, render/graph/ms 14.83, terrain/update/ms 4.05, anim/evaluate_cpu/ms 1.54, stream/drive/ms 1.52, physics/insert/ms 1.23, render/present/ms 1.20, bigspace/total/ms 1.07, anim/evaluate/ms 0.75, stream/tick/ms 0.65 |
| warm-up | 6.77 | 53.0 | pipelines/wait_ms 4479, pipelines/compile_ms 145, pipelines/front_ms 140, render/thread/ms 54.59, render/busy/ms 53.26, main/busy/ms 53.22, render/graph/ms 12.05, terrain/update/ms 5.95, pipelines/driver_ms 5.45, stream/drive/ms 1.35, physics/insert/ms 1.33, render/present/ms 1.07 |
| measure | 2.72 | 35.3 | pipelines/wait_ms 6719, pipelines/compile_ms 81.17, pipelines/front_ms 78.19, render/thread/ms 47.49, render/busy/ms 46.09, render/graph/ms 24.04, main/busy/ms 18.77, pipelines/driver_ms 2.99, render/present/ms 1.17, bigspace/total/ms 0.88, physics/insert/ms 0.78, terrain/update/ms 0.53 |
| measure | 13.41 | 49.0 | render/thread/ms 61.98, render/present/ms 34.62, render/busy/ms 27.17, main/busy/ms 19.49, render/graph/ms 17.11, bigspace/total/ms 0.76, stream/drive/ms 0.57, terrain/update/ms 0.54, stream/tick/ms 0.51, controller/feed/ms 0.49, terrain/select/ms 0.48, far/select/ms 0.41 |
| measure | 13.69 | 34.2 | render/thread/ms 48.37, render/busy/ms 30.05, main/busy/ms 22.51, render/graph/ms 19.52, render/present/ms 18.14, bigspace/total/ms 0.91, terrain/update/ms 0.58, far/select/ms 0.55, stream/drive/ms 0.54, controller/feed/ms 0.53, terrain/select/ms 0.51, stream/tick/ms 0.49 |
| measure | 59.05 | 35.0 | render/thread/ms 44.46, render/busy/ms 43.21, main/busy/ms 29.23, render/graph/ms 12.96, bigspace/total/ms 1.29, stream/drive/ms 1.21, stream/tick/ms 1.09, render/present/ms 1.02, controller/feed/ms 0.93, far/select/ms 0.91, terrain/update/ms 0.67, anim/evaluate_cpu/ms 0.64 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-off\tex-slab-off.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\slab2_off\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 9%, CPU 14%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **75%** (min 56%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.0 W · 75 °C (max 79) · VRAM 5293 MiB · throttled 100% of samples
