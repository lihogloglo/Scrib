# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4180 frames over 65.6 s · **median 13.41 ms (75 fps)** · mean 15.70 · p95 26.02 · p99 29.07 (1% low 34 fps) · worst 49.28 ms · 814 spikes over 1.5× median · 36.6 stutters/min (work 34.7/min)
- warm-up: 526 frames over 12.1 s, worst 3271.9 ms
- hitches (> 33.5 ms): **10** measured, 20 in warm-up
- timed GPU passes: 8.00 ms of a 15.70 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.41 ms, limit 11 ms
- **FAIL** frame_ms: 40 stutters (> 1.5x moving median of 60) in 1.09 min = 36.56/min, limit 1/min
- **FAIL** worst frame 49.28 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.204 | 2.225 | 2.758 | 0.99 | 0.201 |
| main_opaque_pass_3d | 1.788 | 1.804 | 2.429 | 0.99 | 0.074 |
| early prepass | 0.565 | 0.570 | 0.999 | 0.99 | 0.120 |
| bloom | 0.339 | 0.342 | 0.490 | 0.99 | 0.076 |
| atmosphere_luts | 0.331 | 0.334 | 0.510 | 0.99 | 0.004 |
| water_velocity | 0.290 | 0.292 | 0.525 | 0.99 | 0.002 |
| uniform_allocation | 0.264 | 0.266 | 0.384 | 0.99 | 0.056 |
| water_sspr | 0.223 | 0.225 | 0.243 | 0.99 | 0.003 |
| ssao | 0.204 | 0.205 | 0.210 | 0.99 | 0.008 |
| bin_unpacking | 0.202 | 0.204 | 0.216 | 0.99 | 0.036 |
| auxiliary_views | 0.199 | 0.200 | 0.775 | 0.99 | 0.225 |
| atmos_froxel_inject | 0.156 | 0.157 | 0.154 | 0.99 | 0.002 |
| water_fft | 0.140 | 0.141 | 0.140 | 0.99 | 0.003 |
| clouds_trace | 0.137 | 0.138 | 0.199 | 0.99 | 0.001 |
| upscaling | 0.130 | 0.131 | 0.155 | 0.99 | 0.005 |
| clouds_composite | 0.110 | 0.111 | 0.111 | 0.99 | 0.002 |
| tonemapping | 0.092 | 0.093 | 0.092 | 0.99 | 0.004 |
| main_transmissive_pass_3d | 0.090 | 0.091 | 0.169 | 0.99 | 0.009 |
| clustering | 0.067 | 0.068 | 0.069 | 0.99 | 0.023 |
| render_sky | 0.056 | 0.056 | 0.055 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.053 | 0.452 | 0.762 | 0.12 | 0.001 |
| surface_trail | 0.050 | 0.058 | 0.057 | 0.86 | 0.003 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.043 | 0.043 | 0.049 | 0.99 | 0.006 |
| water_snapshot | 0.038 | 0.039 | 0.045 | 0.99 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.99 | 0.004 |
| lightprobe_radiance_map | 0.029 | 0.246 | 0.530 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.028 | 0.031 | 0.99 | 0.001 |
| water_mips | 0.025 | 0.025 | 0.024 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 0.99 | 0.008 |
| main_transparent_pass_3d | 0.017 | 0.017 | 0.032 | 0.99 | 0.122 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.99 | 0.004 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 0.99 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.004 |
| night_sky | 0.006 | 0.006 | 0.009 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.007 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 24.293 | 0.000 | 12461.89 | 101544.2 |
| render/thread/ms | 14.314 | 23.951 | 47.41 | 59833.5 |
| render/busy/ms | 13.641 | 22.922 | 46.52 | 57019.4 |
| main/busy/ms | 11.008 | 17.647 | 44.39 | 46013.9 |
| render/graph/ms | 8.281 | 13.832 | 28.27 | 34612.8 |
| pipelines/compile_ms | 3.031 | 0.000 | 2799.71 | 12671.2 |
| pipelines/driver_ms | 2.594 | 0.000 | 2637.07 | 10843.5 |
| render/present/ms | 0.566 | 0.804 | 16.28 | 2365.2 |
| bigspace/total/ms | 0.515 | 0.863 | 4.27 | 2151.0 |
| pipelines/front_ms | 0.437 | 0.000 | 212.16 | 1827.7 |
| controller/feed/ms | 0.326 | 0.497 | 1.10 | 1361.1 |
| terrain/update/ms | 0.322 | 0.447 | 2.41 | 1346.7 |
| stream/drive/ms | 0.291 | 0.510 | 32.02 | 1217.7 |
| terrain/select/ms | 0.285 | 0.393 | 0.90 | 1192.0 |
| far/select/ms | 0.219 | 0.344 | 0.87 | 913.5 |
| stream/tick/ms | 0.212 | 0.348 | 0.76 | 887.3 |
| bigspace/low_precision/ms | 0.195 | 0.358 | 4.01 | 813.3 |
| anim/evaluate_cpu/ms | 0.183 | 0.289 | 0.64 | 765.7 |
| controller/move/ms | 0.153 | 0.284 | 0.64 | 638.9 |
| anim/evaluate/ms | 0.131 | 0.299 | 3.27 | 549.0 |
| bigspace/high_precision/ms | 0.122 | 0.246 | 2.88 | 509.3 |
| render/acquire/ms | 0.107 | 0.163 | 0.57 | 448.9 |
| physics/insert/ms | 0.082 | 0.534 | 1.14 | 341.4 |
| physics/step/ms | 0.075 | 0.169 | 0.72 | 314.2 |
| weather/map/ms | 0.027 | 0.039 | 0.31 | 114.3 |
| anim/apply/ms | 0.013 | 0.030 | 0.25 | 53.7 |
| controller/avoid/ms | 0.012 | 0.015 | 0.23 | 48.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.08 | 46.3 |
| stream/despawn/ms | 0.009 | 0.000 | 1.26 | 37.4 |
| script/update/ms | 0.009 | 0.015 | 0.12 | 37.2 |
| stream/spawn/ms | 0.008 | 0.000 | 1.14 | 32.0 |
| script/ui/ms | 0.007 | 0.010 | 0.08 | 27.3 |
| script/input/ms | 0.006 | 0.009 | 0.27 | 26.6 |
| stream/deactivate/ms | 0.005 | 0.000 | 0.82 | 19.0 |
| controller/write/ms | 0.004 | 0.005 | 0.13 | 15.3 |
| stream/activate/ms | 0.002 | 0.000 | 0.53 | 6.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 4.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.5 |
| stream/upload/ms | 0.000 | 0.000 | 0.18 | 1.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 0 | 8 | 0 |
| anim/a3/count | count | 569 | 579 | 578 |
| anim/characters/count | count | 571 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 25 | 43 | 10 |
| crowd/c0/joints/count | count | 1173 | 2262 | 434 |
| crowd/c0/meshes/count | count | 332 | 646 | 101 |
| crowd/c1/count | count | 149 | 156 | 151 |
| crowd/c1/joints/count | count | 5683 | 5957 | 5762 |
| crowd/c1/meshes/count | count | 1202 | 1283 | 1240 |
| crowd/c2/count | count | 400 | 417 | 417 |
| crowd/c2/joints/count | count | 6416 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1824 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 189 | 236 | 222 |
| crowd/joints_extracted/count | count | 4320 | 9951 | 6124 |
| crowd/joints_uploaded/count | count | 4276 | 6643 | 6124 |
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
| frame/work/ms | ms | 11.65 | 46.52 | 9.67 |
| gpu/timed/ms | ms | 8.72 | 12.88 | 9.26 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2533 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 248 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 750 | 1416 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 248 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 750 | 1416 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 28 | 30 | 28 |
| phases/opaque3d/bins | count | 1298 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3298 | 5152 | 3181 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1298 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3298 | 5152 | 3181 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 54 | 281 | 47 |
| phases/shadow/bins | count | 4380 | 5792 | 2220 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9646 | 13.1 k | 5641 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 205 | 319 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2630 | 3268 | 2347 |
| physics/items | count | 94.50 | 476 | 19 |
| physics/pending | count | 0 | 344 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 17 | 0 |
| preprocess/bind_groups | count | 12 | 759 | 4 |
| preprocess/bind_groups_created | count | 12 | 759 | 4 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 254 | 394 | 253 |
| process/cpu_usage | % | 24.04 | 44.44 | 21.17 |
| process/mem_usage | GiB | 5.19 | 5.36 | 5.36 |
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
| started_load_count |  | 4788 | 4859 | 4859 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 423 | 528 | 396 |
| statics/materials_bound | count | 423 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.16 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 323 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.48 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1126 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 38.77 | 99.98 | 28.50 |
| system/mem_usage | % | 61.63 | 65.37 | 60.03 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2832 | 2934 | 2934 |
| textures/images | count | 2600 | 2644 | 2644 |
| views/all/batch_sets | count | 123 | 347 | 117 |
| views/all/bins | count | 7198 | 8648 | 5700 |
| views/all/count | count | 34 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 17.0 k | 25.6 k | 13.2 k |
| views/all/pipelines | count | 261 | 385 | 113 |
| views/all/pipelines_distinct | count | 102 | 111 | 91 |
| views/all/work_items | count | 15.0 k | 23.2 k | 13.5 k |
| views/cascade/batch_sets | count | 54 | 74 | 47 |
| views/cascade/bins | count | 2593 | 3203 | 2220 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7647 | 10.1 k | 5641 |
| views/cascade/pipelines | count | 54 | 74 | 47 |
| views/cascade/work_items | count | 7647 | 10.1 k | 5641 |
| views/lamp_face/batch_sets | count | 0 | 225 | 0 |
| views/lamp_face/bins | count | 1767 | 2859 | 0 |
| views/lamp_face/count | count | 30 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1890 | 3179 | 0 |
| views/lamp_face/pipelines | count | 151 | 252 | 0 |
| views/lamp_face/work_items | count | 0 | 2888 | 0 |
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 2910 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7882 | 13.1 k | 7596 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8204 | 13.3 k | 7904 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 30.9 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 49.40 | 283 | 206.5 k |
| anim/events/count | count | 0.13 | 3 | 525 |
| assets/Image/added | count | 0.14 | 43 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.97 | 152 | 4075 |
| assets/Mesh/removed | count | 1.17 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 466 |
| assets/StandardMaterial/modified | count | 5.18 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.10 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0.23 | 20 | 960 |
| ecs/entities_delta | count | 1.13 | 1485 | 4718 |
| physics/inserts | count | 0.61 | 111 | 2570 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.56 | 145 | 2326 |
| physics/steps | count | 0.94 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.09 | 17 | 378 |
| stream/upload/bytes | bytes | 8099 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 54 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1776 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 33.5 ms; slowest 30, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 3.27 | 3271.9 | main/busy/ms 3231, surface/advance_cpu_ms 69.67, surface/cell_upload_ms 1.14, stream/drive/ms 0.34, stream/tick/ms 0.12, weather/map/ms 0.11, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.02, script/input/ms 0.02, far/select/ms 0.01 |
| warm-up | 4.19 | 916.5 | main/busy/ms 3282, render/thread/ms 862, render/busy/ms 679, render/acquire/ms 180, surface/advance_cpu_ms 69.67, render/graph/ms 9.19, render/present/ms 3.33, surface/cell_upload_ms 1.14, stream/drive/ms 1.10, terrain/update/ms 0.60, stream/tick/ms 0.52, terrain/select/ms 0.38 |
| warm-up | 4.56 | 366.9 | pipelines/wait_ms 3323, render/thread/ms 880, render/busy/ms 695, pipelines/compile_ms 394, pipelines/front_ms 369, render/acquire/ms 180, main/busy/ms 75.60, terrain/update/ms 28.66, pipelines/driver_ms 24.35, render/graph/ms 14.33, render/present/ms 4.85, stream/drive/ms 1.80 |
| warm-up | 4.83 | 103.6 | main/busy/ms 71.82, render/thread/ms 66.00, render/busy/ms 63.86, render/graph/ms 5.47, stream/drive/ms 2.51, stream/tick/ms 1.94, render/present/ms 1.70, terrain/update/ms 0.90, terrain/select/ms 0.74, render/acquire/ms 0.44, bigspace/total/ms 0.39, bigspace/high_precision/ms 0.21 |
| warm-up | 5.01 | 64.5 | render/thread/ms 61.60, render/busy/ms 57.90, main/busy/ms 55.53, stream/drive/ms 40.38, terrain/update/ms 6.52, render/graph/ms 5.38, render/acquire/ms 1.89, render/present/ms 1.80, stream/tick/ms 1.14, stream/spawn/ms 0.69, terrain/select/ms 0.66, bigspace/total/ms 0.27 |
| warm-up | 5.10 | 66.3 | pipelines/wait_ms 836, pipelines/compile_ms 216, pipelines/front_ms 197, render/thread/ms 57.98, render/busy/ms 55.56, main/busy/ms 45.42, stream/drive/ms 21.53, pipelines/driver_ms 19.60, render/graph/ms 5.81, render/present/ms 2.12, stream/tick/ms 1.20, stream/spawn/ms 1.17 |
| warm-up | 5.23 | 109.5 | pipelines/wait_ms 1877, pipelines/compile_ms 161, pipelines/front_ms 155, render/thread/ms 73.26, render/busy/ms 71.46, main/busy/ms 55.21, stream/drive/ms 31.00, pipelines/driver_ms 6.35, render/graph/ms 3.63, stream/spawn/ms 2.25, stream/tick/ms 2.06, render/present/ms 1.54 |
| warm-up | 5.26 | 34.0 | pipelines/wait_ms 1384, pipelines/compile_ms 127, pipelines/front_ms 122, render/thread/ms 80.83, render/busy/ms 79.04, main/busy/ms 73.68, stream/drive/ms 48.75, pipelines/driver_ms 4.56, render/graph/ms 4.36, stream/spawn/ms 2.31, stream/tick/ms 2.06, render/present/ms 1.52 |
| warm-up | 5.35 | 90.9 | main/busy/ms 72.37, render/thread/ms 52.02, render/busy/ms 50.20, stream/drive/ms 47.61, render/graph/ms 4.25, stream/spawn/ms 1.97, stream/tick/ms 1.71, render/present/ms 1.55, terrain/update/ms 1.02, terrain/select/ms 0.83, bigspace/total/ms 0.48, stream/upload/ms 0.32 |
| warm-up | 5.39 | 37.5 | render/thread/ms 59.38, render/busy/ms 57.12, main/busy/ms 28.35, render/graph/ms 5.46, stream/drive/ms 4.58, render/present/ms 1.85, stream/tick/ms 1.69, terrain/update/ms 1.03, terrain/select/ms 0.84, stream/spawn/ms 0.83, bigspace/total/ms 0.81, stream/activate/ms 0.54 |
| warm-up | 5.54 | 41.6 | pipelines/wait_ms 6850, pipelines/compile_ms 341, pipelines/front_ms 319, main/busy/ms 75.30, stream/drive/ms 54.55, render/thread/ms 40.36, render/busy/ms 38.08, pipelines/driver_ms 22.68, render/graph/ms 4.72, stream/spawn/ms 2.11, render/present/ms 2.02, stream/tick/ms 1.16 |
| warm-up | 5.62 | 80.4 | pipelines/wait_ms 4403, main/busy/ms 79.09, stream/drive/ms 53.73, pipelines/compile_ms 51.32, render/thread/ms 47.88, pipelines/front_ms 47.23, render/busy/ms 46.51, pipelines/driver_ms 4.09, render/graph/ms 3.55, stream/spawn/ms 1.60, render/present/ms 1.18, stream/tick/ms 0.87 |
| warm-up | 5.73 | 81.5 | pipelines/wait_ms 3969, pipelines/compile_ms 190, pipelines/front_ms 184, main/busy/ms 77.49, stream/drive/ms 57.64, render/thread/ms 40.03, render/busy/ms 38.73, pipelines/driver_ms 5.87, render/graph/ms 4.34, stream/spawn/ms 1.71, render/present/ms 1.07, stream/tick/ms 1.01 |
| warm-up | 5.80 | 44.6 | pipelines/wait_ms 9960, pipelines/compile_ms 62.90, pipelines/front_ms 58.40, main/busy/ms 54.17, stream/drive/ms 40.77, render/thread/ms 40.69, render/busy/ms 38.65, render/graph/ms 7.26, pipelines/driver_ms 4.50, render/present/ms 1.79, stream/spawn/ms 1.57, stream/activate/ms 1.07 |
| warm-up | 5.84 | 33.8 | pipelines/wait_ms 3364, main/busy/ms 39.07, stream/drive/ms 27.37, pipelines/compile_ms 22.54, pipelines/front_ms 20.80, render/thread/ms 18.15, render/busy/ms 16.88, render/graph/ms 4.63, pipelines/driver_ms 1.74, stream/activate/ms 1.08, render/present/ms 1.05, stream/tick/ms 0.77 |
| warm-up | 5.96 | 34.0 | pipelines/wait_ms 3996, pipelines/compile_ms 62.03, pipelines/front_ms 56.79, render/thread/ms 47.19, render/busy/ms 45.44, main/busy/ms 34.45, render/graph/ms 10.13, terrain/update/ms 6.80, stream/drive/ms 6.07, pipelines/driver_ms 5.24, stream/spawn/ms 3.32, stream/tick/ms 1.68 |
| warm-up | 7.14 | 35.1 | render/thread/ms 36.30, render/busy/ms 35.18, main/busy/ms 32.11, stream/drive/ms 16.65, render/graph/ms 6.23, stream/spawn/ms 1.14, render/present/ms 0.91, terrain/update/ms 0.66, stream/tick/ms 0.66, terrain/select/ms 0.54, bigspace/total/ms 0.51, render/acquire/ms 0.21 |
| warm-up | 7.19 | 35.7 | pipelines/wait_ms 2145, pipelines/compile_ms 138, pipelines/front_ms 127, render/thread/ms 39.66, render/busy/ms 38.51, main/busy/ms 20.72, pipelines/driver_ms 10.60, render/graph/ms 6.88, stream/drive/ms 3.01, stream/spawn/ms 1.47, render/present/ms 0.98, stream/tick/ms 0.64 |
| warm-up | 7.31 | 39.3 | main/busy/ms 45.86, render/thread/ms 29.06, render/busy/ms 27.78, stream/drive/ms 13.20, render/graph/ms 6.09, render/present/ms 0.98, physics/insert/ms 0.92, stream/tick/ms 0.70, terrain/update/ms 0.60, bigspace/total/ms 0.59, stream/spawn/ms 0.54, terrain/select/ms 0.52 |
| warm-up | 7.74 | 34.1 | render/thread/ms 43.16, render/busy/ms 41.80, main/busy/ms 41.42, render/graph/ms 8.32, stream/drive/ms 1.37, render/present/ms 1.01, bigspace/total/ms 0.82, stream/tick/ms 0.76, terrain/update/ms 0.68, terrain/select/ms 0.56, anim/evaluate_cpu/ms 0.42, render/acquire/ms 0.35 |
| measure | 2.73 | 36.2 | render/thread/ms 48.94, render/busy/ms 47.69, render/graph/ms 25.26, main/busy/ms 20.09, render/present/ms 1.09, bigspace/total/ms 0.79, physics/insert/ms 0.67, terrain/update/ms 0.62, controller/feed/ms 0.60, terrain/select/ms 0.55, stream/drive/ms 0.49, stream/tick/ms 0.44 |
| measure | 4.10 | 34.6 | render/thread/ms 47.40, render/busy/ms 30.30, main/busy/ms 24.73, render/graph/ms 17.58, render/present/ms 16.86, physics/insert/ms 1.13, bigspace/total/ms 0.96, controller/feed/ms 0.58, terrain/update/ms 0.57, stream/drive/ms 0.56, terrain/select/ms 0.51, stream/tick/ms 0.49 |
| measure | 6.05 | 34.4 | render/thread/ms 59.47, render/busy/ms 50.44, main/busy/ms 34.98, render/graph/ms 29.85, render/present/ms 8.75, bigspace/total/ms 1.43, terrain/update/ms 0.93, controller/feed/ms 0.89, stream/drive/ms 0.89, terrain/select/ms 0.84, stream/tick/ms 0.72, bigspace/low_precision/ms 0.69 |
| measure | 7.05 | 34.2 | render/thread/ms 54.92, render/busy/ms 44.66, main/busy/ms 37.64, render/graph/ms 26.16, render/present/ms 9.99, bigspace/total/ms 1.64, controller/feed/ms 1.00, terrain/update/ms 0.85, stream/drive/ms 0.83, physics/insert/ms 0.79, terrain/select/ms 0.76, stream/tick/ms 0.75 |
| measure | 7.16 | 33.6 | render/thread/ms 54.79, render/busy/ms 52.99, main/busy/ms 35.66, render/graph/ms 25.10, stream/drive/ms 1.72, bigspace/total/ms 1.52, render/present/ms 1.51, physics/insert/ms 0.97, controller/feed/ms 0.91, anim/evaluate/ms 0.90, terrain/update/ms 0.88, stream/tick/ms 0.84 |
| measure | 8.18 | 49.3 | render/thread/ms 69.41, render/busy/ms 67.66, render/graph/ms 41.58, main/busy/ms 37.97, bigspace/total/ms 1.66, render/present/ms 1.47, controller/feed/ms 1.30, terrain/update/ms 0.79, controller/move/ms 0.77, stream/drive/ms 0.72, terrain/select/ms 0.70, far/select/ms 0.59 |
| measure | 11.10 | 35.4 | render/thread/ms 46.16, render/busy/ms 44.39, main/busy/ms 30.07, render/graph/ms 27.36, render/present/ms 1.47, bigspace/total/ms 1.31, physics/insert/ms 1.07, controller/feed/ms 0.93, terrain/update/ms 0.84, terrain/select/ms 0.75, stream/drive/ms 0.75, stream/tick/ms 0.66 |
| measure | 18.24 | 46.7 | main/busy/ms 59.89, render/thread/ms 43.16, render/busy/ms 41.51, stream/drive/ms 32.40, render/graph/ms 23.04, bigspace/total/ms 1.36, render/present/ms 1.33, controller/feed/ms 0.94, terrain/update/ms 0.81, terrain/select/ms 0.72, far/select/ms 0.68, stream/tick/ms 0.55 |
| measure | 18.38 | 35.1 | render/thread/ms 52.89, render/busy/ms 51.13, main/busy/ms 36.37, render/graph/ms 22.78, render/present/ms 1.43, controller/feed/ms 1.27, bigspace/total/ms 1.25, terrain/update/ms 0.80, terrain/select/ms 0.71, anim/evaluate_cpu/ms 0.64, far/select/ms 0.63, stream/drive/ms 0.57 |
| measure | 23.32 | 35.0 | render/thread/ms 54.51, render/busy/ms 53.15, main/busy/ms 39.86, render/graph/ms 19.08, bigspace/total/ms 1.41, controller/feed/ms 1.28, physics/insert/ms 1.18, render/present/ms 1.04, stream/drive/ms 0.92, terrain/update/ms 0.78, terrain/select/ms 0.71, anim/evaluate_cpu/ms 0.66 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_new.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lampsel_new\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 18%
- during the run: other processes 15% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **67%** (min 32%) · 2580 MHz (min 2580) · memory 8001 MHz · 68.0 W · 74 °C (max 80) · VRAM 5284 MiB · throttled 100% of samples
