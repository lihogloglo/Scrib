# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4798 frames over 65.6 s · **median 13.27 ms (75 fps)** · mean 13.68 · p95 16.59 · p99 18.97 (1% low 53 fps) · worst 35.16 ms · 34 spikes over 1.5× median · 20.1 stutters/min (work 20.1/min)
- warm-up: 447 frames over 9.3 s, worst 2496.5 ms
- hitches (> 33.2 ms): **2** measured, 19 in warm-up
- timed GPU passes: 8.06 ms of a 13.68 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.27 ms, limit 11 ms
- **FAIL** frame_ms: 22 stutters (> 1.5x moving median of 60) in 1.09 min = 20.11/min, limit 1/min
- **FAIL** worst frame 35.16 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.209 | 2.214 | 2.789 | 1.00 | 0.184 |
| main_opaque_pass_3d | 1.749 | 1.752 | 2.401 | 1.00 | 0.074 |
| early prepass | 0.542 | 0.543 | 0.845 | 1.00 | 0.120 |
| bloom | 0.341 | 0.342 | 0.493 | 1.00 | 0.067 |
| uniform_allocation | 0.329 | 0.330 | 0.604 | 1.00 | 0.056 |
| atmosphere_luts | 0.329 | 0.329 | 0.458 | 1.00 | 0.003 |
| water_velocity | 0.305 | 0.305 | 0.541 | 1.00 | 0.002 |
| bin_unpacking | 0.250 | 0.250 | 0.276 | 1.00 | 0.039 |
| water_sspr | 0.223 | 0.223 | 0.220 | 1.00 | 0.002 |
| ssao | 0.207 | 0.208 | 0.211 | 1.00 | 0.007 |
| atmos_froxel_inject | 0.158 | 0.159 | 0.153 | 1.00 | 0.002 |
| auxiliary_views | 0.158 | 0.158 | 0.668 | 1.00 | 0.140 |
| clouds_trace | 0.141 | 0.141 | 0.229 | 1.00 | 0.001 |
| water_fft | 0.140 | 0.140 | 0.137 | 1.00 | 0.003 |
| upscaling | 0.129 | 0.129 | 0.135 | 1.00 | 0.004 |
| clouds_composite | 0.112 | 0.112 | 0.111 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.093 | 0.093 | 0.172 | 1.00 | 0.008 |
| tonemapping | 0.093 | 0.093 | 0.092 | 1.00 | 0.004 |
| clustering | 0.071 | 0.071 | 0.070 | 1.00 | 0.020 |
| render_sky | 0.055 | 0.055 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.055 | 0.439 | 0.746 | 0.12 | 0.001 |
| surface_trail | 0.050 | 0.059 | 0.057 | 0.85 | 0.002 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.046 | 0.046 | 0.048 | 1.00 | 0.005 |
| water_snapshot | 0.038 | 0.038 | 0.045 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.029 | 0.237 | 0.240 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.030 | 1.00 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.023 | 1.00 | 0.006 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 1.00 | 0.109 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.018 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 36.094 | 0.000 | 21953.17 | 173177.0 |
| render/thread/ms | 12.486 | 15.241 | 33.14 | 59907.5 |
| render/busy/ms | 11.908 | 14.634 | 32.31 | 57136.2 |
| main/busy/ms | 9.352 | 12.106 | 28.88 | 44873.1 |
| render/graph/ms | 7.279 | 9.048 | 18.33 | 34922.8 |
| pipelines/compile_ms | 0.520 | 0.000 | 147.66 | 2495.8 |
| pipelines/front_ms | 0.502 | 0.000 | 143.58 | 2407.9 |
| render/present/ms | 0.486 | 0.581 | 17.71 | 2332.5 |
| bigspace/total/ms | 0.453 | 0.684 | 4.61 | 2173.1 |
| terrain/update/ms | 0.288 | 0.364 | 1.61 | 1384.0 |
| controller/feed/ms | 0.268 | 0.362 | 0.76 | 1285.6 |
| stream/drive/ms | 0.258 | 0.367 | 20.75 | 1236.3 |
| terrain/select/ms | 0.256 | 0.322 | 0.74 | 1228.1 |
| stream/tick/ms | 0.197 | 0.272 | 0.84 | 944.2 |
| far/select/ms | 0.193 | 0.273 | 0.55 | 925.7 |
| bigspace/low_precision/ms | 0.176 | 0.292 | 4.34 | 846.3 |
| anim/evaluate_cpu/ms | 0.174 | 0.215 | 0.61 | 833.2 |
| anim/evaluate/ms | 0.133 | 0.296 | 2.81 | 636.5 |
| controller/move/ms | 0.126 | 0.210 | 0.45 | 603.4 |
| bigspace/high_precision/ms | 0.109 | 0.220 | 3.34 | 525.2 |
| render/acquire/ms | 0.091 | 0.122 | 0.38 | 438.8 |
| physics/step/ms | 0.060 | 0.089 | 0.50 | 287.1 |
| physics/insert/ms | 0.050 | 0.425 | 0.80 | 239.0 |
| weather/map/ms | 0.024 | 0.033 | 0.29 | 116.8 |
| pipelines/driver_ms | 0.018 | 0.000 | 6.32 | 87.9 |
| anim/apply/ms | 0.011 | 0.021 | 0.21 | 54.0 |
| controller/avoid/ms | 0.010 | 0.014 | 0.05 | 49.1 |
| weather/eval/ms | 0.010 | 0.014 | 0.19 | 49.1 |
| script/update/ms | 0.007 | 0.010 | 0.14 | 35.0 |
| stream/despawn/ms | 0.006 | 0.000 | 0.98 | 29.4 |
| stream/spawn/ms | 0.005 | 0.000 | 1.04 | 25.9 |
| script/input/ms | 0.005 | 0.007 | 0.20 | 24.8 |
| script/ui/ms | 0.005 | 0.006 | 0.11 | 21.7 |
| controller/write/ms | 0.004 | 0.004 | 0.15 | 17.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.63 | 14.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.6 |
| weather/apply/ms | 0.000 | 0.000 | 0.21 | 1.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 12 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 20 | 21 | 19 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1277 | 2262 | 555 |
| crowd/c0/meshes/count | count | 352 | 646 | 143 |
| crowd/c1/count | count | 146 | 157 | 149 |
| crowd/c1/joints/count | count | 5531 | 6046 | 5641 |
| crowd/c1/meshes/count | count | 1166 | 1322 | 1198 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 12.9 k | 6179 |
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
| frame/work/ms | ms | 11.55 | 32.31 | 10.42 |
| gpu/timed/ms | ms | 8.73 | 12.17 | 9.25 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 3074 | 3144 | 3144 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 2 | 2 | 2 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 277.8 k | 282.2 k | 282.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 602.13 M | 603.31 M | 603.31 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 35 | 41 | 37 |
| phases/opaque3d/bins | count | 1280 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3403 | 5145 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 35 | 41 | 37 |
| phases/opaque3d_prepass/bins | count | 1280 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3403 | 5145 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 70 | 306 | 59 |
| phases/shadow/bins | count | 3732 | 5491 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9396 | 12.7 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 139 | 328 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 194 | 476 | 19 |
| physics/pending | count | 0 | 1024 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 39 | 0 |
| preprocess/bind_groups | count | 12 | 853 | 7 |
| preprocess/bind_groups_created | count | 12 | 853 | 7 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 315 | 448 | 310 |
| process/cpu_usage | % | 22.97 | 42.28 | 25.84 |
| process/mem_usage | GiB | 5.16 | 5.33 | 5.33 |
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
| started_load_count |  | 4765 | 4858 | 4858 |
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
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.04 | 52.01 | 30.23 |
| system/mem_usage | % | 63.95 | 64.38 | 64.30 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2591 | 2644 | 2644 |
| views/all/batch_sets | count | 157 | 394 | 147 |
| views/all/bins | count | 6406 | 8180 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.2 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 203 | 388 | 113 |
| views/all/pipelines_distinct | count | 96 | 110 | 91 |
| views/all/work_items | count | 17.2 k | 23.7 k | 13.6 k |
| views/cascade/batch_sets | count | 67 | 89 | 59 |
| views/cascade/bins | count | 2590 | 3162 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8151 | 10.2 k | 5663 |
| views/cascade/pipelines | count | 54 | 73 | 47 |
| views/cascade/work_items | count | 8151 | 10.2 k | 5663 |
| views/lamp_face/batch_sets | count | 0 | 232 | 0 |
| views/lamp_face/bins | count | 1059 | 2532 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1181 | 2877 | 0 |
| views/lamp_face/pipelines | count | 80 | 255 | 0 |
| views/lamp_face/work_items | count | 0 | 2455 | 0 |
| views/main/batch_sets | count | 86 | 98 | 88 |
| views/main/bins | count | 2858 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8424 | 13.1 k | 7592 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8586 | 13.3 k | 7900 |
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
| anim/bones_written/count | count | 50.43 | 233 | 242.0 k |
| anim/events/count | count | 0.11 | 3 | 532 |
| assets/Image/added | count | 0.13 | 43 | 618 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.88 | 152 | 4211 |
| assets/Mesh/removed | count | 1.02 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 494 |
| assets/StandardMaterial/modified | count | 4.51 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.37 | 1379 | 6576 |
| physics/inserts | count | 0.68 | 201 | 3249 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.49 | 145 | 2335 |
| physics/steps | count | 0.82 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 3 | 21 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 17 | 378 |
| stream/upload/bytes | bytes | 7056 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1547 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 33.2 ms; slowest 21, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2496.5 | main/busy/ms 2472, surface/advance_cpu_ms 71.97, surface/cell_upload_ms 0.87, stream/drive/ms 0.33, weather/map/ms 0.24, stream/tick/ms 0.09, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.03, script/ui/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.16 | 664.9 | main/busy/ms 2507, render/thread/ms 627, render/busy/ms 507, render/acquire/ms 117, surface/advance_cpu_ms 71.97, render/graph/ms 5.11, render/present/ms 2.17, stream/drive/ms 1.17, surface/cell_upload_ms 0.87, stream/tick/ms 0.59, terrain/update/ms 0.53, weather/map/ms 0.35 |
| warm-up | 3.23 | 72.3 | render/thread/ms 639, render/busy/ms 518, pipelines/wait_ms 446, render/acquire/ms 118, pipelines/compile_ms 72.36, pipelines/front_ms 66.33, main/busy/ms 50.60, terrain/update/ms 19.02, render/graph/ms 8.83, pipelines/driver_ms 6.03, render/present/ms 3.11, stream/drive/ms 1.45 |
| warm-up | 3.47 | 92.9 | pipelines/compile_ms 225, pipelines/front_ms 214, render/thread/ms 66.86, main/busy/ms 59.30, render/busy/ms 59.01, pipelines/driver_ms 10.40, render/acquire/ms 5.68, render/graph/ms 5.06, render/present/ms 2.17, stream/drive/ms 1.31, stream/tick/ms 0.81, terrain/update/ms 0.72 |
| warm-up | 3.64 | 33.7 | render/thread/ms 43.27, render/busy/ms 41.73, main/busy/ms 40.85, stream/drive/ms 29.22, render/graph/ms 3.51, render/present/ms 1.31, stream/tick/ms 0.70, terrain/update/ms 0.67, terrain/select/ms 0.55, bigspace/total/ms 0.24, render/acquire/ms 0.23, stream/activate/ms 0.13 |
| warm-up | 3.70 | 38.8 | pipelines/wait_ms 1547, pipelines/compile_ms 203, pipelines/front_ms 184, render/thread/ms 49.25, main/busy/ms 47.90, render/busy/ms 47.37, stream/drive/ms 23.87, pipelines/driver_ms 19.55, render/graph/ms 4.74, stream/spawn/ms 1.59, render/present/ms 1.55, stream/tick/ms 1.34 |
| warm-up | 3.78 | 77.8 | pipelines/wait_ms 1547, pipelines/compile_ms 203, pipelines/front_ms 184, render/thread/ms 94.04, render/busy/ms 90.65, main/busy/ms 38.99, pipelines/driver_ms 19.55, stream/drive/ms 12.42, render/graph/ms 8.70, terrain/update/ms 5.74, render/present/ms 2.89, stream/spawn/ms 1.70 |
| warm-up | 3.81 | 33.4 | render/thread/ms 79.69, render/busy/ms 76.85, main/busy/ms 17.71, render/graph/ms 6.67, terrain/update/ms 5.87, stream/drive/ms 3.08, render/present/ms 2.50, stream/tick/ms 1.12, terrain/select/ms 0.76, stream/spawn/ms 0.66, render/acquire/ms 0.35, bigspace/total/ms 0.23 |
| warm-up | 3.89 | 81.6 | pipelines/wait_ms 645, render/thread/ms 77.25, render/busy/ms 53.86, main/busy/ms 43.51, stream/drive/ms 28.13, render/present/ms 23.13, pipelines/compile_ms 3.72, render/graph/ms 2.72, pipelines/driver_ms 2.54, stream/spawn/ms 1.30, pipelines/front_ms 1.18, stream/tick/ms 0.86 |
| warm-up | 4.06 | 42.2 | pipelines/wait_ms 4324, pipelines/compile_ms 169, pipelines/front_ms 152, main/busy/ms 69.23, render/thread/ms 56.42, render/busy/ms 54.23, stream/drive/ms 51.78, pipelines/driver_ms 16.64, render/graph/ms 14.31, stream/spawn/ms 2.58, render/present/ms 1.92, stream/tick/ms 1.67 |
| warm-up | 4.16 | 104.1 | pipelines/wait_ms 13.2 k, pipelines/compile_ms 365, pipelines/front_ms 249, pipelines/driver_ms 116, render/thread/ms 87.21, render/busy/ms 85.57, main/busy/ms 69.26, stream/drive/ms 51.39, render/graph/ms 37.55, stream/spawn/ms 2.10, stream/tick/ms 1.73, render/present/ms 1.41 |
| warm-up | 4.25 | 71.8 | pipelines/wait_ms 2732, main/busy/ms 63.70, stream/drive/ms 44.74, render/thread/ms 43.32, render/busy/ms 42.11, pipelines/compile_ms 17.05, pipelines/front_ms 15.23, render/graph/ms 3.99, stream/spawn/ms 2.19, pipelines/driver_ms 1.83, render/present/ms 1.04, stream/tick/ms 0.68 |
| warm-up | 4.33 | 63.3 | pipelines/wait_ms 7714, pipelines/compile_ms 159, pipelines/front_ms 153, main/busy/ms 54.72, render/thread/ms 34.77, stream/drive/ms 34.19, render/busy/ms 33.07, pipelines/driver_ms 6.68, render/graph/ms 5.41, stream/spawn/ms 1.60, render/present/ms 1.50, stream/tick/ms 1.42 |
| warm-up | 4.37 | 33.5 | pipelines/wait_ms 6017, pipelines/compile_ms 51.59, pipelines/front_ms 48.16, main/busy/ms 36.90, stream/drive/ms 25.24, render/thread/ms 18.00, render/busy/ms 16.51, render/graph/ms 4.30, pipelines/driver_ms 3.43, render/present/ms 1.09, stream/activate/ms 1.04, stream/tick/ms 0.81 |
| warm-up | 5.11 | 36.8 | pipelines/wait_ms 1797, render/thread/ms 53.11, render/busy/ms 51.00, main/busy/ms 33.98, render/graph/ms 9.49, pipelines/compile_ms 6.92, pipelines/front_ms 6.39, stream/drive/ms 4.74, stream/spawn/ms 2.23, render/present/ms 1.75, stream/tick/ms 1.42, physics/insert/ms 1.17 |
| warm-up | 5.15 | 37.9 | render/thread/ms 51.87, render/busy/ms 50.49, main/busy/ms 40.64, render/graph/ms 6.17, stream/drive/ms 4.74, stream/spawn/ms 2.14, stream/tick/ms 1.46, physics/insert/ms 1.17, render/present/ms 1.09, terrain/update/ms 0.79, terrain/select/ms 0.65, physics/step/ms 0.61 |
| warm-up | 5.47 | 39.6 | main/busy/ms 46.70, render/thread/ms 32.20, render/busy/ms 30.35, stream/drive/ms 15.94, render/graph/ms 10.51, terrain/update/ms 2.81, stream/spawn/ms 2.53, stream/tick/ms 2.02, render/present/ms 1.59, physics/insert/ms 1.38, bigspace/total/ms 0.82, terrain/select/ms 0.59 |
| warm-up | 5.55 | 35.3 | pipelines/wait_ms 4519, pipelines/compile_ms 49.54, main/busy/ms 47.44, pipelines/front_ms 46.44, render/thread/ms 38.25, render/busy/ms 36.69, render/graph/ms 10.05, stream/drive/ms 3.98, pipelines/driver_ms 3.11, stream/spawn/ms 2.35, physics/insert/ms 1.52, render/present/ms 1.19 |
| warm-up | 6.61 | 33.9 | render/thread/ms 40.23, main/busy/ms 40.01, render/busy/ms 35.40, render/graph/ms 8.12, render/present/ms 4.58, stream/drive/ms 1.88, physics/insert/ms 1.18, stream/spawn/ms 0.84, bigspace/total/ms 0.81, stream/tick/ms 0.58, anim/evaluate_cpu/ms 0.57, terrain/update/ms 0.55 |
| measure | 2.18 | 34.2 | render/thread/ms 47.02, render/busy/ms 45.69, render/graph/ms 24.52, main/busy/ms 17.27, pipelines/compile_ms 11.78, pipelines/front_ms 11.56, render/present/ms 1.14, stream/drive/ms 0.87, bigspace/total/ms 0.82, stream/tick/ms 0.80, terrain/update/ms 0.53, terrain/select/ms 0.48 |
| measure | 59.06 | 35.2 | render/thread/ms 45.03, render/busy/ms 43.82, main/busy/ms 23.35, render/graph/ms 13.94, stream/drive/ms 1.15, stream/tick/ms 1.02, bigspace/total/ms 0.97, render/present/ms 0.95, controller/feed/ms 0.94, terrain/update/ms 0.86, terrain/select/ms 0.77, far/select/ms 0.61 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-off\tex-slab-off.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\slab_off\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **77%** (min 53%) · 2580 MHz (min 2580) · memory 8001 MHz · 72.9 W · 73 °C (max 79) · VRAM 5299 MiB · throttled 100% of samples
