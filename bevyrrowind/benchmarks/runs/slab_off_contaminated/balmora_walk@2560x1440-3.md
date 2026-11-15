# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 788 frames over 65.8 s · **median 79.73 ms (13 fps)** · mean 83.52 · p95 111.99 · p99 119.37 (1% low 8 fps) · worst 158.16 ms · 7 spikes over 1.5× median · 4.6 stutters/min (work 2.7/min)
- warm-up: 335 frames over 16.6 s, worst 3213.3 ms
- hitches (> 199.3 ms): **0** measured, 3 in warm-up
- timed GPU passes: 59.47 ms of a 83.52 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 79.73 ms, limit 11 ms
- **FAIL** frame_ms: 5 stutters (> 1.5x moving median of 60) in 1.10 min = 4.56/min, limit 1/min
- **FAIL** worst frame 158.16 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 14.316 | 14.371 | 29.239 | 1.00 | 0.082 |
| early prepass | 12.745 | 12.793 | 27.963 | 1.00 | 0.132 |
| dlss_super_resolution | 5.750 | 5.780 | 7.672 | 0.99 | 0.220 |
| water_fft | 5.431 | 5.451 | 6.837 | 1.00 | 0.003 |
| bin_unpacking | 5.047 | 5.053 | 7.529 | 1.00 | 0.037 |
| surface_trail | 2.339 | 2.363 | 3.690 | 0.99 | 0.002 |
| auxiliary_views | 2.174 | 2.182 | 7.114 | 1.00 | 0.315 |
| uniform_allocation | 1.712 | 1.714 | 2.788 | 1.00 | 0.059 |
| early_mesh_preprocessing | 1.491 | 1.493 | 2.444 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 1.108 | 1.112 | 1.698 | 1.00 | 0.008 |
| water_velocity | 1.049 | 1.054 | 1.724 | 0.99 | 0.001 |
| bloom | 1.011 | 1.016 | 1.624 | 0.99 | 0.068 |
| atmosphere_luts | 0.869 | 0.872 | 1.581 | 1.00 | 0.004 |
| atmos_froxel_inject | 0.668 | 0.671 | 1.380 | 0.99 | 0.002 |
| water_snapshot | 0.629 | 0.631 | 1.315 | 1.00 | 0.002 |
| clustering | 0.610 | 0.612 | 1.256 | 1.00 | 0.021 |
| ssao | 0.497 | 0.499 | 1.138 | 1.00 | 0.008 |
| clouds_trace | 0.399 | 0.401 | 0.537 | 0.99 | 0.001 |
| water_sspr | 0.265 | 0.266 | 0.804 | 1.00 | 0.002 |
| water_mips | 0.187 | 0.187 | 0.715 | 1.00 | 0.002 |
| upscaling | 0.157 | 0.158 | 0.683 | 0.99 | 0.004 |
| lightprobe_irradiance_map | 0.147 | 1.180 | 1.688 | 0.12 | 0.001 |
| per_view_shadow_pass | 0.134 | 0.134 | 0.675 | 1.00 | 0.006 |
| clouds_shadow | 0.128 | 0.128 | 0.225 | 1.00 | 0.003 |
| tonemapping | 0.115 | 0.115 | 0.117 | 0.99 | 0.003 |
| clouds_composite | 0.111 | 0.111 | 0.112 | 0.99 | 0.002 |
| lightprobe_radiance_map | 0.084 | 0.673 | 1.974 | 0.12 | 0.004 |
| render_sky | 0.057 | 0.057 | 0.056 | 0.99 | 0.002 |
| clouds_resolve | 0.051 | 0.051 | 0.056 | 0.99 | 0.001 |
| main_transparent_pass_3d | 0.043 | 0.044 | 0.082 | 0.99 | 0.114 |
| atmos_froxel_integrate | 0.033 | 0.033 | 0.036 | 0.99 | 0.001 |
| atmosphere_environment | 0.030 | 0.030 | 0.014 | 1.00 | 0.004 |
| lightprobe_downsampling_first_pass | 0.023 | 0.186 | 0.894 | 0.12 | 0.001 |
| lightprobe_copy | 0.018 | 0.144 | 0.652 | 0.12 | 0.004 |
| sparse buffer updates | 0.016 | 0.016 | 0.011 | 1.00 | 0.004 |
| clouds_fog_light | 0.013 | 0.013 | 0.012 | 0.99 | 0.002 |
| night_sky | 0.008 | 0.008 | 0.014 | 0.99 | 0.001 |
| lightprobe_downsampling_second_pass | 0.004 | 0.029 | 0.012 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.12 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 81.965 | 110.625 | 156.69 | 64588.4 |
| render/acquire/ms | 64.453 | 96.208 | 137.92 | 50789.2 |
| render/busy/ms | 16.849 | 22.886 | 45.02 | 13276.9 |
| main/busy/ms | 10.459 | 13.968 | 35.58 | 8242.0 |
| render/graph/ms | 10.298 | 14.259 | 30.35 | 8114.6 |
| pipelines/compile_ms | 1.103 | 0.000 | 141.69 | 869.4 |
| pipelines/front_ms | 1.063 | 0.000 | 135.02 | 837.6 |
| render/present/ms | 0.663 | 0.819 | 17.13 | 522.3 |
| bigspace/total/ms | 0.460 | 0.666 | 1.81 | 362.5 |
| stream/drive/ms | 0.452 | 0.977 | 25.11 | 355.9 |
| terrain/update/ms | 0.356 | 0.505 | 1.61 | 280.2 |
| controller/feed/ms | 0.310 | 0.452 | 1.29 | 244.0 |
| terrain/select/ms | 0.299 | 0.412 | 0.68 | 235.8 |
| physics/insert/ms | 0.245 | 0.634 | 0.99 | 193.3 |
| stream/tick/ms | 0.207 | 0.354 | 0.68 | 163.1 |
| pipelines/wait_ms | 0.207 | 0.000 | 112.97 | 163.0 |
| far/select/ms | 0.203 | 0.324 | 0.56 | 159.6 |
| anim/evaluate_cpu/ms | 0.195 | 0.311 | 0.55 | 154.0 |
| physics/step/ms | 0.172 | 0.268 | 0.67 | 135.3 |
| bigspace/low_precision/ms | 0.171 | 0.265 | 0.53 | 134.9 |
| anim/evaluate/ms | 0.140 | 0.227 | 0.51 | 110.6 |
| controller/move/ms | 0.127 | 0.217 | 0.57 | 99.9 |
| bigspace/high_precision/ms | 0.107 | 0.153 | 0.62 | 84.0 |
| pipelines/driver_ms | 0.040 | 0.000 | 6.66 | 31.8 |
| stream/despawn/ms | 0.038 | 0.511 | 0.93 | 30.1 |
| stream/spawn/ms | 0.034 | 0.251 | 1.33 | 26.7 |
| weather/map/ms | 0.028 | 0.040 | 0.09 | 22.3 |
| stream/deactivate/ms | 0.014 | 0.000 | 0.61 | 11.4 |
| anim/apply/ms | 0.014 | 0.025 | 0.04 | 10.7 |
| weather/eval/ms | 0.012 | 0.017 | 0.07 | 9.2 |
| controller/avoid/ms | 0.012 | 0.015 | 0.51 | 9.2 |
| script/update/ms | 0.009 | 0.014 | 0.09 | 7.4 |
| stream/activate/ms | 0.008 | 0.000 | 0.53 | 6.1 |
| script/input/ms | 0.006 | 0.009 | 0.02 | 4.7 |
| script/ui/ms | 0.005 | 0.009 | 0.07 | 4.3 |
| controller/write/ms | 0.004 | 0.005 | 0.09 | 3.0 |
| stream/upload/ms | 0.001 | 0.009 | 0.09 | 1.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 0.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.07 | 0.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 2 | 9 | 0 |
| anim/a3/count | count | 557 | 579 | 578 |
| anim/characters/count | count | 560 | 579 | 578 |
| anim/evaluated/count | count | 19 | 36 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1313 | 2262 | 590 |
| crowd/c0/meshes/count | count | 355 | 646 | 171 |
| crowd/c1/count | count | 144 | 156 | 148 |
| crowd/c1/joints/count | count | 5445 | 5995 | 5606 |
| crowd/c1/meshes/count | count | 1150 | 1297 | 1170 |
| crowd/c2/count | count | 375 | 417 | 417 |
| crowd/c2/joints/count | count | 6238 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1787 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 236 | 223 |
| crowd/joints_extracted/count | count | 4274 | 10.2 k | 6179 |
| crowd/joints_uploaded/count | count | 4176 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 56.29 | 118 | 80.61 |
| gpu/timed/ms | ms | 56.29 | 118 | 80.61 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 3075 | 3147 | 3147 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 2 | 2 | 2 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 277.8 k | 282.2 k | 282.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 170 | 403 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 920 | 1415 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 170 | 403 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 920 | 1415 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 37 | 41 | 36 |
| phases/opaque3d/bins | count | 1276 | 1392 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3590 | 5090 | 3181 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 26 | 28 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 37 | 41 | 36 |
| phases/opaque3d_prepass/bins | count | 1276 | 1392 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3590 | 5090 | 3181 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 26 | 28 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 91 | 305 | 62 |
| phases/shadow/bins | count | 4025 | 5356 | 2249 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9964 | 12.7 k | 5700 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 162 | 309 | 50 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2622 | 2986 | 2348 |
| physics/items | count | 132 | 267 | 19 |
| physics/pending | count | 106 | 637 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 224 | 230 | 230 |
| pipelines/waiting | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 53 | 855 | 10 |
| preprocess/bind_groups_created | count | 53 | 855 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 315 | 442 | 309 |
| process/cpu_usage | % | 10.94 | 27.23 | 2.23 |
| process/mem_usage | GiB | 6.58 | 6.83 | 6.80 |
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
| started_load_count |  | 4768 | 4864 | 4864 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 435 | 528 | 396 |
| statics/materials_bound | count | 435 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 4 | 0 |
| stream/deactivate/queue | count | 0 | 2 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 3 | 0 |
| stream/latency/s | s | 0.23 | 0.82 | 0.27 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 339 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 1.61 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1129 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.99 | 60.73 | 17.63 |
| system/mem_usage | % | 87.85 | 89.99 | 89.73 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2774 | 2934 | 2934 |
| textures/images | count | 2471 | 2644 | 2644 |
| views/all/batch_sets | count | 181 | 395 | 148 |
| views/all/bins | count | 6716 | 8187 | 5729 |
| views/all/count | count | 28 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.4 k | 25.6 k | 13.3 k |
| views/all/pipelines | count | 228 | 377 | 116 |
| views/all/pipelines_distinct | count | 99 | 108 | 91 |
| views/all/work_items | count | 18.0 k | 23.7 k | 13.6 k |
| views/cascade/batch_sets | count | 71 | 85 | 62 |
| views/cascade/bins | count | 2654 | 3095 | 2249 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8306 | 10.2 k | 5700 |
| views/cascade/pipelines | count | 57 | 70 | 50 |
| views/cascade/work_items | count | 8306 | 10.2 k | 5700 |
| views/lamp_face/batch_sets | count | 17 | 230 | 0 |
| views/lamp_face/bins | count | 1360 | 2459 | 0 |
| views/lamp_face/count | count | 24 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1521 | 2829 | 0 |
| views/lamp_face/pipelines | count | 105 | 246 | 0 |
| views/lamp_face/work_items | count | 196 | 2465 | 0 |
| views/main/batch_sets | count | 90 | 98 | 86 |
| views/main/bins | count | 2847 | 3556 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9011 | 12.9 k | 7596 |
| views/main/pipelines | count | 64 | 70 | 66 |
| views/main/work_items | count | 9168 | 13.2 k | 7904 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.7 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 61.44 | 245 | 48.4 k |
| anim/events/count | count | 0.67 | 4 | 530 |
| assets/Image/added | count | 0.75 | 41 | 594 |
| assets/Image/modified | count | 0.02 | 1 | 14 |
| assets/Image/removed | count | 0.06 | 6 | 46 |
| assets/Mesh/added | count | 5.20 | 191 | 4096 |
| assets/Mesh/removed | count | 6.24 | 1254 | 4920 |
| assets/StandardMaterial/added | count | 0.59 | 35 | 466 |
| assets/StandardMaterial/modified | count | 11 | 11 | 8668 |
| assets/StandardMaterial/removed | count | 0.52 | 38 | 408 |
| assets/WorldAsset/added | count | 0.04 | 5 | 28 |
| assets/WorldAsset/removed | count | 0.11 | 6 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 5.98 | 1494 | 4716 |
| physics/inserts | count | 2.55 | 111 | 2009 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 2.49 | 120 | 1960 |
| physics/steps | count | 2.99 | 3 | 2359 |
| physics/steps_dropped | count | 2.01 | 6 | 1586 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.02 | 2 | 17 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.28 | 20 | 219 |
| stream/deactivate/items | count | 0.28 | 21 | 223 |
| stream/despawn/items | count | 0.28 | 6 | 222 |
| stream/fetch/items | count | 0.11 | 3 | 83 |
| stream/spawn/items | count | 0.48 | 18 | 378 |
| stream/upload/bytes | bytes | 43.0 k | 3.04 M | 33.85 M |
| stream/upload/items | count | 0.07 | 4 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.04 | 1 | 31 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 9419 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 199.3 ms; slowest 3, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 3.21 | 3213.3 | main/busy/ms 3184, surface/advance_cpu_ms 71.60, surface/cell_upload_ms 0.89, stream/drive/ms 0.36, weather/map/ms 0.23, stream/tick/ms 0.12, bigspace/total/ms 0.11, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.03, anim/evaluate/ms 0.01, far/select/ms 0.01 |
| warm-up | 4.15 | 939.1 | main/busy/ms 3234, render/thread/ms 883, render/busy/ms 752, render/acquire/ms 130, surface/advance_cpu_ms 71.60, render/graph/ms 4.88, render/present/ms 2.09, stream/drive/ms 1.29, surface/cell_upload_ms 0.89, stream/tick/ms 0.71, terrain/update/ms 0.62, terrain/select/ms 0.45 |
| warm-up | 4.38 | 229.6 | render/thread/ms 895, pipelines/wait_ms 765, render/busy/ms 763, pipelines/compile_ms 235, pipelines/front_ms 221, render/acquire/ms 130, main/busy/ms 67.88, terrain/update/ms 28.89, pipelines/driver_ms 13.88, render/graph/ms 9.65, render/present/ms 3.00, stream/drive/ms 1.43 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-off\tex-slab-off.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\slab_off\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 18%
- during the run: other processes 18% CPU (system minus this process)
- nvidia-smi over the window (254 samples): GPU load **98%** (min 50%) · 2580 MHz (min 2580) · memory 8001 MHz · 46.7 W · 67 °C (max 69) · VRAM 7850 MiB · throttled 100% of samples
