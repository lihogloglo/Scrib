# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4083 frames over 65.7 s · **median 13.18 ms (76 fps)** · mean 16.08 · p95 31.18 · p99 70.40 (1% low 14 fps) · worst 137.10 ms · 361 spikes over 1.5× median · 17.4 stutters/min (work 11.9/min)
- warm-up: 331 frames over 15.8 s, worst 2874.5 ms
- hitches (> 33.0 ms): **162** measured, 167 in warm-up
- timed GPU passes: 10.04 ms of a 16.08 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.18 ms, limit 11 ms
- **FAIL** frame_ms: 19 stutters (> 1.5x moving median of 60) in 1.09 min = 17.36/min, limit 1/min
- **FAIL** worst frame 137.10 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.362 | 2.372 | 2.914 | 1.00 | 0.184 |
| main_opaque_pass_3d | 2.167 | 2.175 | 4.135 | 1.00 | 0.079 |
| early prepass | 0.901 | 0.905 | 3.246 | 1.00 | 0.123 |
| bin_unpacking | 0.553 | 0.555 | 2.421 | 1.00 | 0.039 |
| auxiliary_views | 0.533 | 0.535 | 2.881 | 1.00 | 0.127 |
| uniform_allocation | 0.393 | 0.394 | 0.971 | 1.00 | 0.057 |
| bloom | 0.361 | 0.363 | 0.690 | 1.00 | 0.066 |
| atmosphere_luts | 0.351 | 0.352 | 0.656 | 1.00 | 0.004 |
| water_velocity | 0.303 | 0.304 | 0.727 | 1.00 | 0.002 |
| water_sspr | 0.247 | 0.248 | 0.384 | 1.00 | 0.002 |
| ssao | 0.216 | 0.217 | 0.343 | 1.00 | 0.008 |
| early_mesh_preprocessing | 0.192 | 0.193 | 1.245 | 1.00 | 0.005 |
| atmos_froxel_inject | 0.169 | 0.170 | 0.156 | 1.00 | 0.002 |
| water_fft | 0.137 | 0.138 | 0.138 | 1.00 | 0.003 |
| clouds_trace | 0.136 | 0.136 | 0.179 | 1.00 | 0.001 |
| upscaling | 0.133 | 0.134 | 0.156 | 1.00 | 0.004 |
| clouds_composite | 0.114 | 0.114 | 0.111 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.111 | 0.111 | 0.319 | 1.00 | 0.008 |
| clustering | 0.101 | 0.102 | 0.281 | 1.00 | 0.021 |
| tonemapping | 0.098 | 0.098 | 0.098 | 1.00 | 0.003 |
| lightprobe_irradiance_map | 0.058 | 0.474 | 0.953 | 0.12 | 0.001 |
| render_sky | 0.056 | 0.056 | 0.055 | 1.00 | 0.002 |
| surface_trail | 0.052 | 0.060 | 0.057 | 0.86 | 0.002 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 1.00 | 0.001 |
| water_snapshot | 0.040 | 0.040 | 0.048 | 1.00 | 0.002 |
| water_mips | 0.032 | 0.032 | 0.024 | 1.00 | 0.002 |
| lightprobe_radiance_map | 0.032 | 0.258 | 0.560 | 0.12 | 0.004 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.032 | 1.00 | 0.001 |
| per_view_shadow_pass | 0.024 | 0.024 | 0.039 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.018 | 0.018 | 0.031 | 1.00 | 0.110 |
| atmosphere_environment | 0.013 | 0.013 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.008 | 1.00 | 0.001 |
| lightprobe_copy | 0.003 | 0.023 | 0.072 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.015 | 0.033 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 14.871 | 29.814 | 134.99 | 60718.3 |
| render/busy/ms | 12.107 | 15.857 | 77.67 | 49434.6 |
| main/busy/ms | 9.306 | 12.042 | 29.33 | 37996.7 |
| render/graph/ms | 7.390 | 9.744 | 29.71 | 30173.0 |
| render/acquire/ms | 2.259 | 14.188 | 108.64 | 9225.4 |
| render/present/ms | 0.504 | 0.608 | 16.25 | 2058.3 |
| bigspace/total/ms | 0.444 | 0.676 | 6.18 | 1814.3 |
| terrain/update/ms | 0.297 | 0.380 | 1.57 | 1211.2 |
| controller/feed/ms | 0.273 | 0.361 | 0.69 | 1113.4 |
| stream/drive/ms | 0.264 | 0.399 | 20.81 | 1079.8 |
| terrain/select/ms | 0.262 | 0.335 | 0.90 | 1071.4 |
| pipelines/compile_ms | 0.229 | 0.000 | 154.26 | 934.6 |
| pipelines/front_ms | 0.221 | 0.000 | 148.33 | 900.6 |
| far/select/ms | 0.198 | 0.279 | 0.64 | 809.3 |
| stream/tick/ms | 0.197 | 0.268 | 0.62 | 802.9 |
| anim/evaluate_cpu/ms | 0.171 | 0.225 | 0.74 | 698.6 |
| bigspace/low_precision/ms | 0.168 | 0.289 | 4.33 | 684.0 |
| controller/move/ms | 0.157 | 0.383 | 0.84 | 642.0 |
| anim/evaluate/ms | 0.136 | 0.297 | 4.59 | 553.4 |
| bigspace/high_precision/ms | 0.108 | 0.194 | 5.54 | 439.5 |
| pipelines/wait_ms | 0.090 | 0.000 | 121.68 | 368.4 |
| physics/step/ms | 0.068 | 0.128 | 0.58 | 278.9 |
| physics/insert/ms | 0.059 | 0.460 | 1.14 | 241.2 |
| weather/map/ms | 0.023 | 0.032 | 0.28 | 95.6 |
| anim/apply/ms | 0.011 | 0.020 | 0.14 | 45.1 |
| controller/avoid/ms | 0.011 | 0.014 | 0.15 | 44.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.24 | 41.9 |
| pipelines/driver_ms | 0.008 | 0.000 | 5.93 | 34.1 |
| stream/despawn/ms | 0.007 | 0.000 | 0.92 | 29.6 |
| script/update/ms | 0.007 | 0.010 | 0.27 | 29.3 |
| stream/spawn/ms | 0.006 | 0.000 | 1.07 | 26.3 |
| script/input/ms | 0.005 | 0.007 | 0.23 | 22.0 |
| script/ui/ms | 0.005 | 0.006 | 0.15 | 19.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.66 | 14.2 |
| controller/write/ms | 0.003 | 0.004 | 0.10 | 13.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.40 | 4.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 4.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 569 | 579 | 578 |
| anim/characters/count | count | 571 | 579 | 578 |
| anim/evaluated/count | count | 20 | 30 | 18 |
| controller/movers/count | count | 20 | 23 | 18 |
| crowd/c0/count | count | 25 | 43 | 11 |
| crowd/c0/joints/count | count | 1236 | 2262 | 482 |
| crowd/c0/meshes/count | count | 343 | 646 | 119 |
| crowd/c1/count | count | 148 | 156 | 150 |
| crowd/c1/joints/count | count | 5596 | 5983 | 5714 |
| crowd/c1/meshes/count | count | 1181 | 1297 | 1222 |
| crowd/c2/count | count | 400 | 417 | 417 |
| crowd/c2/joints/count | count | 6416 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1824 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 194 | 236 | 223 |
| crowd/joints_extracted/count | count | 4340 | 9933 | 6179 |
| crowd/joints_uploaded/count | count | 4316 | 6643 | 6179 |
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
| frame/work/ms | ms | 11.47 | 77.67 | 11.50 |
| gpu/timed/ms | ms | 8.88 | 67.71 | 9.23 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 3086 | 3146 | 3146 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 2 | 2 | 2 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 279.7 k | 282.2 k | 282.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 244 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 749 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 244 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 749 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 34 | 38 | 35 |
| phases/opaque3d/bins | count | 1291 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3292 | 5117 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 34 | 38 | 35 |
| phases/opaque3d_prepass/bins | count | 1291 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3292 | 5117 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 67 | 343 | 56 |
| phases/shadow/bins | count | 2605 | 5406 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 7958 | 12.4 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 54 | 316 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2632 | 3268 | 2347 |
| physics/items | count | 137 | 476 | 19 |
| physics/pending | count | 0 | 340 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 12 | 915 | 6 |
| preprocess/bind_groups_created | count | 12 | 915 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 303 | 458 | 297 |
| process/cpu_usage | % | 21.63 | 44.85 | 24.04 |
| process/mem_usage | GiB | 6.45 | 6.62 | 6.62 |
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
| started_load_count |  | 4790 | 4860 | 4860 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 423 | 528 | 396 |
| statics/materials_bound | count | 423 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.30 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 320 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.51 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1126 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.84 | 52.02 | 27.74 |
| system/mem_usage | % | 62.92 | 75.01 | 63.26 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2832 | 2938 | 2938 |
| textures/images | count | 2600 | 2649 | 2649 |
| views/all/batch_sets | count | 147 | 425 | 140 |
| views/all/bins | count | 5906 | 8253 | 5692 |
| views/all/count | count | 4 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.9 k | 25.1 k | 13.3 k |
| views/all/pipelines | count | 120 | 376 | 113 |
| views/all/pipelines_distinct | count | 95 | 109 | 91 |
| views/all/work_items | count | 15.0 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 66 | 84 | 56 |
| views/cascade/bins | count | 2572 | 3186 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7698 | 10.0 k | 5663 |
| views/cascade/pipelines | count | 54 | 71 | 47 |
| views/cascade/work_items | count | 7698 | 10.0 k | 5663 |
| views/lamp_face/batch_sets | count | 0 | 269 | 0 |
| views/lamp_face/bins | count | 0 | 2451 | 0 |
| views/lamp_face/count | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 2818 | 0 |
| views/lamp_face/pipelines | count | 0 | 249 | 0 |
| views/lamp_face/work_items | count | 0 | 2663 | 0 |
| views/main/batch_sets | count | 82 | 92 | 84 |
| views/main/bins | count | 2902 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7876 | 13.0 k | 7592 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8194 | 13.2 k | 7900 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.4 k | 37.8 k | 30.7 k |
| world/point_lights | count | 289 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 48.25 | 259 | 197.0 k |
| anim/events/count | count | 0.14 | 3 | 572 |
| assets/Image/added | count | 0.15 | 42 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 1.00 | 152 | 4075 |
| assets/Mesh/removed | count | 1.19 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 466 |
| assets/StandardMaterial/modified | count | 4.94 | 11 | 20.2 k |
| assets/StandardMaterial/removed | count | 0.10 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.16 | 1492 | 4716 |
| physics/inserts | count | 0.57 | 107 | 2342 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.55 | 145 | 2243 |
| physics/steps | count | 0.93 | 3 | 3805 |
| physics/steps_dropped | count | 0.03 | 6 | 137 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.09 | 19 | 378 |
| stream/upload/bytes | bytes | 8291 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 47 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1818 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 33.0 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.87 | 2874.5 | main/busy/ms 2836, surface/advance_cpu_ms 69.59, surface/cell_upload_ms 1.66, stream/drive/ms 0.35, weather/map/ms 0.16, stream/tick/ms 0.11, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.03, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.50 | 622.5 | main/busy/ms 2885, render/thread/ms 573, render/busy/ms 444, render/acquire/ms 126, surface/advance_cpu_ms 69.59, render/graph/ms 6.74, render/present/ms 2.62, surface/cell_upload_ms 1.66, stream/drive/ms 1.53, stream/tick/ms 0.84, terrain/update/ms 0.83, terrain/select/ms 0.58 |
| warm-up | 3.84 | 345.1 | pipelines/wait_ms 3983, render/thread/ms 588, render/busy/ms 458, pipelines/compile_ms 376, pipelines/front_ms 343, render/acquire/ms 127, main/busy/ms 72.64, pipelines/driver_ms 32.78, terrain/update/ms 24.95, render/graph/ms 11.30, render/present/ms 3.92, stream/drive/ms 1.92 |
| warm-up | 4.12 | 104.7 | main/busy/ms 74.35, render/thread/ms 57.02, render/busy/ms 54.73, render/graph/ms 6.07, stream/drive/ms 3.14, render/present/ms 1.93, stream/tick/ms 1.44, terrain/update/ms 1.12, terrain/select/ms 0.93, stream/spawn/ms 0.72, render/acquire/ms 0.35, bigspace/total/ms 0.35 |
| warm-up | 4.41 | 76.4 | render/thread/ms 85.21, render/busy/ms 78.35, main/busy/ms 55.65, stream/drive/ms 35.55, render/graph/ms 9.15, render/present/ms 6.33, stream/spawn/ms 1.86, stream/tick/ms 1.09, terrain/update/ms 0.94, terrain/select/ms 0.81, render/acquire/ms 0.54, bigspace/total/ms 0.45 |
| warm-up | 4.55 | 116.0 | pipelines/wait_ms 4241, pipelines/compile_ms 320, pipelines/front_ms 296, render/thread/ms 63.32, render/busy/ms 62.38, main/busy/ms 40.16, pipelines/driver_ms 23.87, stream/drive/ms 13.26, render/graph/ms 4.82, stream/tick/ms 1.72, stream/spawn/ms 1.68, terrain/update/ms 1.10 |
| warm-up | 4.92 | 74.6 | pipelines/wait_ms 4747, pipelines/compile_ms 103, pipelines/front_ms 96.83, main/busy/ms 90.21, stream/drive/ms 66.83, render/thread/ms 57.24, render/busy/ms 55.23, render/graph/ms 19.85, pipelines/driver_ms 5.77, stream/spawn/ms 3.39, render/present/ms 1.71, stream/tick/ms 0.95 |
| warm-up | 5.09 | 131.0 | pipelines/wait_ms 780, pipelines/compile_ms 196, pipelines/front_ms 190, main/busy/ms 101, render/thread/ms 79.26, render/busy/ms 77.76, stream/drive/ms 70.24, render/graph/ms 7.35, pipelines/driver_ms 6.23, stream/spawn/ms 3.42, stream/tick/ms 2.37, terrain/update/ms 1.26 |
| warm-up | 6.55 | 98.1 | pipelines/wait_ms 2467, pipelines/compile_ms 185, pipelines/front_ms 175, render/thread/ms 115, render/acquire/ms 77.36, render/busy/ms 36.34, main/busy/ms 24.41, pipelines/driver_ms 10.00, render/graph/ms 6.03, stream/drive/ms 3.65, stream/spawn/ms 2.09, render/present/ms 1.29 |
| warm-up | 7.57 | 70.8 | render/thread/ms 85.53, render/busy/ms 62.87, main/busy/ms 55.14, render/present/ms 22.36, render/graph/ms 11.51, stream/drive/ms 1.99, stream/tick/ms 1.04, bigspace/total/ms 0.99, physics/insert/ms 0.78, anim/evaluate_cpu/ms 0.63, terrain/update/ms 0.62, physics/step/ms 0.61 |
| warm-up | 8.87 | 77.7 | pipelines/wait_ms 4681, pipelines/compile_ms 175, pipelines/front_ms 169, render/thread/ms 109, render/acquire/ms 84.32, render/busy/ms 23.50, main/busy/ms 19.07, render/graph/ms 13.66, pipelines/driver_ms 5.55, render/present/ms 1.18, stream/drive/ms 1.17, terrain/update/ms 0.61 |
| warm-up | 11.45 | 71.2 | pipelines/wait_ms 16.2 k, pipelines/compile_ms 249, pipelines/front_ms 242, render/thread/ms 112, render/acquire/ms 74.27, render/busy/ms 36.14, main/busy/ms 23.39, render/graph/ms 19.78, pipelines/driver_ms 7.09, stream/drive/ms 1.24, bigspace/total/ms 1.23, physics/insert/ms 1.22 |
| measure | 0.53 | 74.5 | render/thread/ms 130, render/acquire/ms 75.49, render/busy/ms 52.96, render/graph/ms 32.71, main/busy/ms 23.16, render/present/ms 1.37, bigspace/total/ms 0.99, terrain/update/ms 0.67, terrain/select/ms 0.60, controller/feed/ms 0.57, stream/drive/ms 0.54, physics/insert/ms 0.54 |
| measure | 0.65 | 72.8 | render/thread/ms 121, render/acquire/ms 82.41, render/busy/ms 37.07, main/busy/ms 27.02, render/graph/ms 22.74, render/present/ms 1.51, bigspace/total/ms 1.04, physics/insert/ms 0.66, terrain/update/ms 0.61, stream/drive/ms 0.57, far/select/ms 0.57, terrain/select/ms 0.53 |
| measure | 1.02 | 69.0 | render/thread/ms 129, render/acquire/ms 90.12, main/busy/ms 37.76, render/busy/ms 37.25, render/graph/ms 20.99, stream/drive/ms 14.17, bigspace/total/ms 1.26, render/present/ms 1.20, terrain/update/ms 0.68, physics/insert/ms 0.67, controller/feed/ms 0.61, terrain/select/ms 0.61 |
| measure | 2.27 | 73.3 | render/thread/ms 126, render/acquire/ms 71.22, render/busy/ms 53.60, render/graph/ms 33.95, main/busy/ms 21.40, pipelines/compile_ms 13.54, pipelines/front_ms 13.33, render/present/ms 1.33, physics/insert/ms 1.28, bigspace/total/ms 1.08, terrain/update/ms 0.74, terrain/select/ms 0.68 |
| measure | 2.39 | 73.5 | render/thread/ms 116, render/acquire/ms 75.16, render/busy/ms 39.02, render/graph/ms 24.77, main/busy/ms 22.57, render/present/ms 1.41, physics/insert/ms 1.19, bigspace/total/ms 1.06, terrain/update/ms 0.64, controller/feed/ms 0.63, stream/drive/ms 0.60, terrain/select/ms 0.56 |
| measure | 2.83 | 83.8 | render/thread/ms 139, render/busy/ms 99.58, render/graph/ms 41.90, render/acquire/ms 37.54, main/busy/ms 30.09, stream/drive/ms 5.32, render/present/ms 1.50, bigspace/total/ms 1.04, terrain/update/ms 0.75, stream/tick/ms 0.75, terrain/select/ms 0.67, stream/spawn/ms 0.58 |
| measure | 3.01 | 137.1 | render/thread/ms 179, render/acquire/ms 129, render/busy/ms 48.38, render/graph/ms 29.14, main/busy/ms 25.52, render/present/ms 1.64, stream/drive/ms 1.34, bigspace/total/ms 0.84, terrain/update/ms 0.61, controller/feed/ms 0.59, far/select/ms 0.57, terrain/select/ms 0.55 |
| measure | 3.09 | 71.3 | render/thread/ms 204, render/acquire/ms 156, render/busy/ms 46.53, render/graph/ms 28.36, main/busy/ms 23.72, render/present/ms 1.37, stream/drive/ms 1.21, bigspace/total/ms 0.83, terrain/update/ms 0.64, terrain/select/ms 0.58, controller/feed/ms 0.47, stream/spawn/ms 0.45 |
| measure | 3.16 | 74.3 | render/thread/ms 141, render/acquire/ms 97.84, render/busy/ms 42.62, main/busy/ms 27.25, render/graph/ms 25.17, stream/drive/ms 1.52, bigspace/total/ms 1.02, render/present/ms 1.01, stream/spawn/ms 0.83, physics/insert/ms 0.69, terrain/update/ms 0.60, controller/feed/ms 0.59 |
| measure | 3.31 | 84.4 | render/thread/ms 148, render/acquire/ms 103, render/busy/ms 44.18, main/busy/ms 24.49, render/graph/ms 23.43, render/present/ms 1.19, stream/drive/ms 0.87, terrain/update/ms 0.81, bigspace/total/ms 0.80, terrain/select/ms 0.61, physics/insert/ms 0.61, physics/step/ms 0.61 |
| measure | 3.46 | 80.9 | render/thread/ms 143, render/acquire/ms 97.35, render/busy/ms 44.78, main/busy/ms 25.49, render/graph/ms 23.89, render/present/ms 1.20, stream/drive/ms 1.16, bigspace/total/ms 0.80, physics/insert/ms 0.70, controller/feed/ms 0.66, stream/spawn/ms 0.63, terrain/update/ms 0.59 |
| measure | 3.53 | 73.7 | render/thread/ms 152, render/acquire/ms 102, render/busy/ms 49.11, render/graph/ms 23.21, main/busy/ms 20.49, render/present/ms 1.28, physics/insert/ms 1.01, bigspace/total/ms 0.83, terrain/update/ms 0.62, terrain/select/ms 0.56, stream/drive/ms 0.47, controller/feed/ms 0.46 |
| measure | 3.73 | 71.3 | render/thread/ms 130, render/acquire/ms 93.96, render/busy/ms 35.05, render/graph/ms 20.14, main/busy/ms 19.94, render/present/ms 1.18, bigspace/total/ms 0.89, stream/drive/ms 0.87, physics/insert/ms 0.79, stream/tick/ms 0.75, terrain/update/ms 0.59, terrain/select/ms 0.53 |
| measure | 3.80 | 72.6 | render/thread/ms 141, render/acquire/ms 102, render/busy/ms 38.08, main/busy/ms 22.75, render/graph/ms 22.44, physics/insert/ms 1.28, render/present/ms 1.11, bigspace/total/ms 1.01, terrain/update/ms 0.63, stream/drive/ms 0.63, terrain/select/ms 0.57, controller/feed/ms 0.50 |
| measure | 4.15 | 78.2 | render/thread/ms 144, render/acquire/ms 105, render/busy/ms 37.39, render/graph/ms 22.54, main/busy/ms 18.86, render/present/ms 1.19, terrain/update/ms 0.76, bigspace/total/ms 0.76, terrain/select/ms 0.69, physics/insert/ms 0.49, controller/feed/ms 0.46, stream/drive/ms 0.42 |
| measure | 4.29 | 80.5 | render/thread/ms 140, render/acquire/ms 103, render/busy/ms 36.38, main/busy/ms 22.53, render/graph/ms 22.12, render/present/ms 1.19, bigspace/total/ms 1.17, terrain/update/ms 0.80, physics/insert/ms 0.74, terrain/select/ms 0.70, controller/feed/ms 0.66, bigspace/low_precision/ms 0.52 |
| measure | 4.43 | 74.0 | render/thread/ms 139, render/acquire/ms 98.16, render/busy/ms 39.31, render/graph/ms 23.27, main/busy/ms 20.96, render/present/ms 1.14, physics/insert/ms 0.87, bigspace/total/ms 0.82, terrain/update/ms 0.58, stream/drive/ms 0.53, terrain/select/ms 0.52, controller/feed/ms 0.48 |
| measure | 4.50 | 69.4 | render/thread/ms 141, render/acquire/ms 104, render/busy/ms 35.63, render/graph/ms 20.74, main/busy/ms 20.00, render/present/ms 1.17, bigspace/total/ms 0.94, stream/drive/ms 0.68, physics/insert/ms 0.63, terrain/update/ms 0.61, stream/tick/ms 0.61, controller/feed/ms 0.57 |
| measure | 4.58 | 75.4 | render/thread/ms 142, render/acquire/ms 110, render/busy/ms 31.42, main/busy/ms 20.01, render/graph/ms 18.38, render/present/ms 1.17, bigspace/total/ms 0.95, stream/drive/ms 0.70, stream/tick/ms 0.62, physics/insert/ms 0.62, terrain/update/ms 0.59, controller/feed/ms 0.58 |
| measure | 4.84 | 69.0 | render/thread/ms 127, render/acquire/ms 94.13, render/busy/ms 31.72, main/busy/ms 20.51, render/graph/ms 19.68, render/present/ms 1.11, physics/insert/ms 0.97, anim/evaluate_cpu/ms 0.95, bigspace/total/ms 0.93, anim/evaluate/ms 0.76, terrain/update/ms 0.67, terrain/select/ms 0.60 |
| measure | 4.91 | 69.9 | render/thread/ms 137, render/acquire/ms 102, render/busy/ms 33.68, render/graph/ms 21.49, main/busy/ms 21.45, render/present/ms 1.24, physics/insert/ms 1.16, anim/evaluate_cpu/ms 0.93, bigspace/total/ms 0.90, anim/evaluate/ms 0.75, terrain/update/ms 0.58, terrain/select/ms 0.52 |
| measure | 5.23 | 75.8 | render/thread/ms 131, render/acquire/ms 101, render/busy/ms 28.77, main/busy/ms 20.25, render/graph/ms 16.90, physics/insert/ms 1.05, render/present/ms 1.04, bigspace/total/ms 0.92, terrain/update/ms 0.67, terrain/select/ms 0.60, stream/drive/ms 0.54, controller/feed/ms 0.49 |
| measure | 5.37 | 69.9 | render/thread/ms 131, render/acquire/ms 100, main/busy/ms 32.84, render/busy/ms 30.22, render/graph/ms 18.62, stream/drive/ms 11.86, physics/insert/ms 1.32, render/present/ms 1.06, bigspace/total/ms 0.81, far/select/ms 0.80, controller/feed/ms 0.67, terrain/update/ms 0.57 |
| measure | 5.44 | 68.4 | render/thread/ms 134, render/acquire/ms 100, main/busy/ms 33.27, render/busy/ms 32.41, render/graph/ms 17.54, stream/drive/ms 12.14, render/present/ms 1.12, physics/insert/ms 1.06, bigspace/total/ms 0.84, far/select/ms 0.82, controller/feed/ms 0.68, stream/tick/ms 0.64 |
| measure | 5.51 | 76.4 | render/thread/ms 140, render/acquire/ms 107, render/busy/ms 32.15, main/busy/ms 19.86, render/graph/ms 17.59, stream/drive/ms 1.31, render/present/ms 1.07, bigspace/total/ms 0.78, physics/insert/ms 0.75, stream/tick/ms 0.71, terrain/update/ms 0.56, controller/feed/ms 0.55 |
| measure | 5.58 | 70.0 | render/thread/ms 144, render/acquire/ms 114, render/busy/ms 28.95, main/busy/ms 23.69, render/graph/ms 17.17, render/present/ms 1.08, stream/drive/ms 1.02, bigspace/total/ms 0.89, controller/feed/ms 0.63, physics/insert/ms 0.57, terrain/update/ms 0.56, terrain/select/ms 0.50 |
| measure | 5.66 | 74.1 | render/thread/ms 141, render/acquire/ms 107, render/busy/ms 33.27, main/busy/ms 24.38, render/graph/ms 19.50, bigspace/total/ms 1.32, render/present/ms 1.21, bigspace/low_precision/ms 0.79, physics/insert/ms 0.61, terrain/update/ms 0.58, stream/drive/ms 0.58, controller/feed/ms 0.55 |
| measure | 5.72 | 68.9 | render/thread/ms 140, render/acquire/ms 106, render/busy/ms 32.83, main/busy/ms 19.43, render/graph/ms 19.23, bigspace/total/ms 1.25, render/present/ms 1.18, bigspace/low_precision/ms 0.76, terrain/update/ms 0.71, terrain/select/ms 0.63, far/select/ms 0.59, stream/drive/ms 0.55 |
| measure | 5.80 | 77.7 | render/thread/ms 144, render/acquire/ms 113, render/busy/ms 30.26, main/busy/ms 19.32, render/graph/ms 17.54, render/present/ms 1.01, bigspace/total/ms 0.89, terrain/update/ms 0.72, terrain/select/ms 0.65, far/select/ms 0.50, controller/feed/ms 0.50, physics/insert/ms 0.50 |
| measure | 5.94 | 75.1 | render/thread/ms 136, render/acquire/ms 106, render/busy/ms 29.33, main/busy/ms 19.03, render/graph/ms 17.59, render/present/ms 1.15, bigspace/total/ms 0.94, terrain/update/ms 0.61, physics/insert/ms 0.56, terrain/select/ms 0.55, far/select/ms 0.54, controller/feed/ms 0.54 |
| measure | 6.20 | 71.0 | render/thread/ms 134, render/acquire/ms 98.09, render/busy/ms 35.00, render/graph/ms 21.45, main/busy/ms 20.36, physics/insert/ms 1.16, render/present/ms 1.13, bigspace/total/ms 0.93, terrain/update/ms 0.65, terrain/select/ms 0.59, controller/feed/ms 0.55, stream/drive/ms 0.48 |
| measure | 6.34 | 77.8 | render/thread/ms 139, render/acquire/ms 105, render/busy/ms 32.66, main/busy/ms 20.47, render/graph/ms 20.15, render/present/ms 1.05, bigspace/total/ms 0.86, stream/drive/ms 0.83, stream/tick/ms 0.71, terrain/update/ms 0.60, controller/feed/ms 0.56, terrain/select/ms 0.54 |
| measure | 6.48 | 74.2 | render/thread/ms 138, render/acquire/ms 105, render/busy/ms 31.98, render/graph/ms 20.43, main/busy/ms 18.64, physics/insert/ms 1.09, render/present/ms 1.04, bigspace/total/ms 0.78, stream/drive/ms 0.69, stream/tick/ms 0.60, terrain/update/ms 0.57, terrain/select/ms 0.49 |
| measure | 6.62 | 74.8 | render/thread/ms 135, render/acquire/ms 106, main/busy/ms 28.79, render/busy/ms 28.02, render/graph/ms 16.65, stream/drive/ms 3.95, physics/insert/ms 1.12, render/present/ms 1.09, bigspace/total/ms 0.92, controller/feed/ms 0.63, terrain/update/ms 0.61, terrain/select/ms 0.55 |
| measure | 6.69 | 73.6 | render/thread/ms 144, render/acquire/ms 108, render/busy/ms 34.74, main/busy/ms 28.49, render/graph/ms 20.00, stream/drive/ms 3.91, render/present/ms 1.18, bigspace/total/ms 0.80, physics/insert/ms 0.75, terrain/update/ms 0.70, terrain/select/ms 0.64, controller/feed/ms 0.62 |
| measure | 6.83 | 79.5 | render/thread/ms 137, render/acquire/ms 105, render/busy/ms 31.05, main/busy/ms 20.82, render/graph/ms 18.94, stream/drive/ms 1.24, render/present/ms 1.04, bigspace/total/ms 0.95, stream/spawn/ms 0.72, terrain/update/ms 0.63, terrain/select/ms 0.57, physics/insert/ms 0.56 |
| measure | 6.90 | 68.6 | render/thread/ms 145, render/acquire/ms 112, render/busy/ms 32.42, render/graph/ms 19.92, main/busy/ms 18.52, render/present/ms 1.05, bigspace/total/ms 0.82, stream/drive/ms 0.71, physics/insert/ms 0.68, terrain/update/ms 0.64, terrain/select/ms 0.57, stream/tick/ms 0.50 |
| measure | 7.45 | 123.5 | render/thread/ms 184, render/acquire/ms 152, render/busy/ms 30.51, main/busy/ms 22.74, render/graph/ms 18.73, stream/drive/ms 2.00, render/present/ms 1.62, stream/despawn/ms 0.94, bigspace/total/ms 0.92, physics/insert/ms 0.73, physics/step/ms 0.70, terrain/update/ms 0.68 |
| measure | 7.52 | 72.1 | render/thread/ms 193, render/acquire/ms 162, render/busy/ms 29.23, main/busy/ms 24.68, render/graph/ms 16.85, render/present/ms 1.49, stream/drive/ms 1.34, physics/insert/ms 1.07, bigspace/total/ms 1.06, controller/feed/ms 0.79, terrain/update/ms 0.65, terrain/select/ms 0.59 |
| measure | 7.61 | 89.1 | render/thread/ms 159, render/acquire/ms 127, render/busy/ms 27.55, main/busy/ms 21.69, render/graph/ms 16.28, render/present/ms 4.15, stream/drive/ms 1.17, physics/insert/ms 1.02, bigspace/total/ms 0.94, terrain/update/ms 0.67, controller/feed/ms 0.65, stream/deactivate/ms 0.61 |
| measure | 7.76 | 79.7 | render/thread/ms 144, render/acquire/ms 113, render/busy/ms 29.55, main/busy/ms 22.32, render/graph/ms 18.28, stream/drive/ms 1.72, physics/insert/ms 1.42, stream/despawn/ms 1.29, bigspace/total/ms 1.20, render/present/ms 1.11, controller/feed/ms 0.67, anim/evaluate_cpu/ms 0.61 |
| measure | 7.84 | 81.3 | render/thread/ms 159, render/acquire/ms 128, render/busy/ms 29.25, main/busy/ms 21.03, render/graph/ms 17.69, render/present/ms 1.30, physics/insert/ms 1.27, bigspace/total/ms 1.24, stream/drive/ms 1.21, stream/despawn/ms 0.78, controller/feed/ms 0.69, terrain/update/ms 0.62 |
| measure | 7.98 | 69.9 | render/thread/ms 135, render/acquire/ms 108, render/busy/ms 26.34, main/busy/ms 20.64, render/graph/ms 15.94, render/present/ms 0.99, bigspace/total/ms 0.90, terrain/update/ms 0.59, controller/feed/ms 0.58, terrain/select/ms 0.54, stream/drive/ms 0.53, physics/insert/ms 0.49 |
| measure | 8.05 | 73.0 | render/thread/ms 141, render/acquire/ms 111, render/busy/ms 28.34, main/busy/ms 19.10, render/graph/ms 16.71, render/present/ms 1.08, bigspace/total/ms 0.82, terrain/update/ms 0.69, terrain/select/ms 0.63, physics/insert/ms 0.60, controller/feed/ms 0.59, stream/drive/ms 0.58 |
| measure | 8.19 | 76.1 | render/thread/ms 139, render/acquire/ms 109, render/busy/ms 29.47, main/busy/ms 19.53, render/graph/ms 18.31, render/present/ms 1.04, stream/drive/ms 0.95, bigspace/total/ms 0.92, physics/insert/ms 0.78, controller/feed/ms 0.66, stream/tick/ms 0.61, terrain/update/ms 0.59 |
| measure | 8.27 | 82.6 | render/thread/ms 157, render/acquire/ms 123, render/busy/ms 32.14, render/graph/ms 21.00, main/busy/ms 19.31, render/present/ms 1.10, bigspace/total/ms 0.91, stream/drive/ms 0.69, terrain/update/ms 0.61, terrain/select/ms 0.55, controller/feed/ms 0.52, physics/insert/ms 0.51 |
| measure | 8.35 | 75.0 | render/thread/ms 155, render/acquire/ms 123, render/busy/ms 31.54, render/graph/ms 20.51, main/busy/ms 17.78, render/present/ms 1.17, bigspace/total/ms 0.88, terrain/update/ms 0.62, terrain/select/ms 0.56, physics/insert/ms 0.51, controller/feed/ms 0.47, stream/drive/ms 0.42 |
| measure | 8.43 | 85.2 | render/thread/ms 158, render/acquire/ms 128, render/busy/ms 28.33, main/busy/ms 19.35, render/graph/ms 18.00, render/present/ms 1.12, bigspace/total/ms 0.94, terrain/update/ms 0.59, terrain/select/ms 0.53, physics/insert/ms 0.51, controller/feed/ms 0.49, stream/drive/ms 0.43 |
| measure | 8.52 | 81.5 | render/thread/ms 164, render/acquire/ms 135, render/busy/ms 27.81, main/busy/ms 21.13, render/graph/ms 17.39, render/present/ms 1.06, bigspace/total/ms 1.00, physics/insert/ms 0.77, terrain/update/ms 0.68, terrain/select/ms 0.61, controller/feed/ms 0.55, stream/drive/ms 0.47 |
| measure | 8.60 | 86.7 | render/thread/ms 166, render/acquire/ms 135, render/busy/ms 27.63, main/busy/ms 20.59, render/graph/ms 17.18, render/present/ms 2.80, physics/insert/ms 1.01, bigspace/total/ms 1.01, anim/evaluate_cpu/ms 0.79, controller/feed/ms 0.76, terrain/update/ms 0.75, terrain/select/ms 0.66 |
| measure | 8.68 | 74.8 | render/thread/ms 159, render/acquire/ms 126, render/busy/ms 29.66, main/busy/ms 21.65, render/graph/ms 19.01, render/present/ms 2.87, bigspace/total/ms 1.09, physics/insert/ms 0.84, anim/evaluate_cpu/ms 0.83, controller/feed/ms 0.80, terrain/update/ms 0.64, anim/evaluate/ms 0.64 |
| measure | 8.75 | 70.3 | render/thread/ms 142, render/acquire/ms 109, render/busy/ms 31.77, main/busy/ms 21.86, render/graph/ms 19.02, render/present/ms 1.17, bigspace/total/ms 1.03, terrain/update/ms 0.66, controller/feed/ms 0.60, terrain/select/ms 0.59, physics/insert/ms 0.58, stream/drive/ms 0.55 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-off\tex-slab-off.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\slab2_off\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 6%, CPU 12%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **83%** (min 58%) · 2580 MHz (min 2580) · memory 8001 MHz · 70.5 W · 74 °C (max 79) · VRAM 7804 MiB · throttled 100% of samples
