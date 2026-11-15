# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4522 frames over 65.7 s · **median 13.92 ms (72 fps)** · mean 14.52 · p95 18.29 · p99 21.12 (1% low 47 fps) · worst 34.36 ms · 48 spikes over 1.5× median · 15.5 stutters/min (work 17.4/min)
- warm-up: 365 frames over 9.1 s, worst 2503.7 ms
- hitches (> 34.8 ms): **0** measured, 17 in warm-up
- timed GPU passes: 7.59 ms of a 14.52 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.92 ms, limit 11 ms
- **FAIL** frame_ms: 17 stutters (> 1.5x moving median of 60) in 1.09 min = 15.53/min, limit 1/min
- **FAIL** worst frame 34.36 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.066 | 2.068 | 2.110 | 1.00 | 0.186 |
| main_opaque_pass_3d | 1.624 | 1.625 | 2.054 | 1.00 | 0.078 |
| early prepass | 0.517 | 0.517 | 0.745 | 1.00 | 0.126 |
| bloom | 0.320 | 0.321 | 0.328 | 1.00 | 0.069 |
| uniform_allocation | 0.316 | 0.317 | 0.333 | 1.00 | 0.056 |
| atmosphere_luts | 0.313 | 0.313 | 0.371 | 1.00 | 0.003 |
| water_velocity | 0.282 | 0.283 | 0.353 | 1.00 | 0.002 |
| bin_unpacking | 0.248 | 0.248 | 0.267 | 1.00 | 0.038 |
| water_sspr | 0.210 | 0.210 | 0.211 | 1.00 | 0.002 |
| ssao | 0.191 | 0.191 | 0.194 | 1.00 | 0.008 |
| auxiliary_views | 0.161 | 0.161 | 0.612 | 1.00 | 0.156 |
| atmos_froxel_inject | 0.149 | 0.149 | 0.150 | 1.00 | 0.002 |
| clouds_trace | 0.141 | 0.141 | 0.229 | 1.00 | 0.001 |
| water_fft | 0.131 | 0.132 | 0.137 | 1.00 | 0.003 |
| upscaling | 0.121 | 0.121 | 0.139 | 1.00 | 0.004 |
| clouds_composite | 0.108 | 0.108 | 0.111 | 1.00 | 0.002 |
| tonemapping | 0.087 | 0.087 | 0.090 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.087 | 0.087 | 0.166 | 1.00 | 0.008 |
| clustering | 0.065 | 0.065 | 0.068 | 1.00 | 0.020 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.417 | 0.420 | 0.12 | 0.001 |
| surface_trail | 0.050 | 0.057 | 0.057 | 0.89 | 0.002 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.048 | 1.00 | 0.005 |
| water_snapshot | 0.031 | 0.031 | 0.034 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.224 | 0.231 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.025 | 0.025 | 0.028 | 1.00 | 0.001 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.023 | 1.00 | 0.006 |
| main_transparent_pass_3d | 0.014 | 0.014 | 0.030 | 1.00 | 0.110 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 57.894 | 0.000 | 33825.05 | 261797.6 |
| render/thread/ms | 13.274 | 16.843 | 33.18 | 60023.6 |
| render/busy/ms | 12.710 | 16.243 | 32.35 | 57474.6 |
| main/busy/ms | 9.755 | 12.997 | 25.17 | 44110.1 |
| render/graph/ms | 7.938 | 10.207 | 17.97 | 35895.4 |
| pipelines/compile_ms | 0.677 | 0.000 | 158.18 | 3061.0 |
| pipelines/front_ms | 0.653 | 0.000 | 151.95 | 2953.6 |
| render/present/ms | 0.471 | 0.571 | 3.25 | 2131.1 |
| bigspace/total/ms | 0.437 | 0.661 | 4.91 | 1974.2 |
| terrain/update/ms | 0.290 | 0.363 | 1.45 | 1313.3 |
| controller/feed/ms | 0.268 | 0.377 | 0.67 | 1212.9 |
| stream/drive/ms | 0.258 | 0.382 | 17.04 | 1165.8 |
| terrain/select/ms | 0.258 | 0.323 | 0.63 | 1165.5 |
| stream/tick/ms | 0.194 | 0.272 | 0.62 | 877.1 |
| far/select/ms | 0.194 | 0.273 | 0.60 | 876.3 |
| bigspace/low_precision/ms | 0.166 | 0.283 | 4.66 | 752.3 |
| anim/evaluate_cpu/ms | 0.162 | 0.215 | 0.45 | 734.8 |
| controller/move/ms | 0.146 | 0.294 | 0.60 | 661.6 |
| anim/evaluate/ms | 0.133 | 0.277 | 3.16 | 602.5 |
| bigspace/high_precision/ms | 0.108 | 0.205 | 4.54 | 490.4 |
| render/acquire/ms | 0.092 | 0.122 | 0.29 | 417.9 |
| physics/insert/ms | 0.066 | 0.484 | 1.40 | 297.5 |
| physics/step/ms | 0.063 | 0.094 | 0.86 | 286.4 |
| weather/map/ms | 0.025 | 0.034 | 0.19 | 111.6 |
| pipelines/driver_ms | 0.024 | 0.000 | 6.23 | 107.5 |
| anim/apply/ms | 0.011 | 0.021 | 0.08 | 50.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.23 | 47.6 |
| controller/avoid/ms | 0.010 | 0.014 | 0.06 | 46.2 |
| stream/despawn/ms | 0.007 | 0.000 | 0.91 | 31.2 |
| script/update/ms | 0.007 | 0.009 | 0.05 | 30.6 |
| stream/spawn/ms | 0.006 | 0.000 | 1.09 | 26.5 |
| script/ui/ms | 0.005 | 0.007 | 0.08 | 22.6 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 22.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.72 | 15.7 |
| controller/write/ms | 0.003 | 0.004 | 0.21 | 15.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.06 | 4.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 11 |
| crowd/c0/joints/count | count | 1272 | 2262 | 489 |
| crowd/c0/meshes/count | count | 349 | 646 | 128 |
| crowd/c1/count | count | 146 | 156 | 150 |
| crowd/c1/joints/count | count | 5531 | 5982 | 5707 |
| crowd/c1/meshes/count | count | 1166 | 1284 | 1213 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4255 | 9674 | 6179 |
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
| frame/work/ms | ms | 12.17 | 32.35 | 13.01 |
| gpu/timed/ms | ms | 8.28 | 11.91 | 8.54 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 811 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 811 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 39 | 42 | 39 |
| phases/opaque3d/bins | count | 1290 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3403 | 5118 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 39 | 42 | 39 |
| phases/opaque3d_prepass/bins | count | 1290 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3403 | 5118 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 72 | 309 | 63 |
| phases/shadow/bins | count | 3815 | 5424 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9579 | 12.4 k | 5667 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 146 | 311 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1658 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 46 | 0 |
| preprocess/bind_groups | count | 15 | 867 | 12 |
| preprocess/bind_groups_created | count | 15 | 867 | 12 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 330 | 450 | 325 |
| process/cpu_usage | % | 23.08 | 41.35 | 17.54 |
| process/mem_usage | GiB | 5.14 | 5.31 | 5.31 |
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
| started_load_count |  | 4766 | 4859 | 4859 |
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
| stream/oldest_request/s | s | 0 | 0.35 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.85 | 69.97 | 42.30 |
| system/mem_usage | % | 69.16 | 70.90 | 70.82 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 163 | 401 | 157 |
| views/all/bins | count | 6469 | 8189 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.2 k | 24.9 k | 13.3 k |
| views/all/pipelines | count | 210 | 375 | 113 |
| views/all/pipelines_distinct | count | 98 | 109 | 91 |
| views/all/work_items | count | 17.3 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 67 | 86 | 63 |
| views/cascade/bins | count | 2607 | 3194 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8128 | 10.0 k | 5667 |
| views/cascade/pipelines | count | 56 | 73 | 47 |
| views/cascade/work_items | count | 8128 | 10.0 k | 5667 |
| views/lamp_face/batch_sets | count | 0 | 238 | 0 |
| views/lamp_face/bins | count | 1089 | 2482 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1211 | 2851 | 0 |
| views/lamp_face/pipelines | count | 89 | 249 | 0 |
| views/lamp_face/work_items | count | 0 | 2515 | 0 |
| views/main/batch_sets | count | 92 | 100 | 94 |
| views/main/bins | count | 2870 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8421 | 13.0 k | 7600 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8582 | 13.2 k | 7908 |
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
| anim/bones_written/count | count | 55.12 | 277 | 249.2 k |
| anim/events/count | count | 0.11 | 3 | 515 |
| assets/Image/added | count | 0.14 | 43 | 614 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.91 | 152 | 4127 |
| assets/Mesh/removed | count | 1.08 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 487 |
| assets/StandardMaterial/modified | count | 4.79 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.40 | 1328 | 6313 |
| physics/inserts | count | 0.84 | 165 | 3805 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.52 | 109 | 2335 |
| physics/steps | count | 0.87 | 2 | 3940 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 17 | 378 |
| stream/upload/bytes | bytes | 7486 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1641 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 34.8 ms; slowest 17, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2503.7 | main/busy/ms 2480, surface/advance_cpu_ms 69.23, surface/cell_upload_ms 0.70, stream/drive/ms 0.36, weather/map/ms 0.17, stream/tick/ms 0.12, bigspace/total/ms 0.07, weather/eval/ms 0.04, bigspace/high_precision/ms 0.04, script/update/ms 0.02, far/select/ms 0.02, script/input/ms 0.01 |
| warm-up | 3.24 | 737.1 | main/busy/ms 2518, render/thread/ms 703, render/busy/ms 546, render/acquire/ms 155, surface/advance_cpu_ms 69.23, render/graph/ms 4.58, render/present/ms 2.18, stream/drive/ms 1.02, surface/cell_upload_ms 0.70, stream/tick/ms 0.48, terrain/update/ms 0.42, weather/map/ms 0.27 |
| warm-up | 3.45 | 209.9 | pipelines/wait_ms 1132, render/thread/ms 723, render/busy/ms 564, pipelines/compile_ms 216, pipelines/front_ms 200, render/acquire/ms 156, main/busy/ms 57.17, terrain/update/ms 21.78, pipelines/driver_ms 15.21, render/graph/ms 13.59, render/present/ms 3.35, stream/drive/ms 1.43 |
| warm-up | 3.74 | 125.2 | pipelines/compile_ms 313, pipelines/front_ms 280, pipelines/wait_ms 208, render/thread/ms 109, render/busy/ms 107, main/busy/ms 86.19, pipelines/driver_ms 32.64, render/graph/ms 3.86, bigspace/total/ms 2.64, stream/drive/ms 1.93, bigspace/high_precision/ms 1.75, render/present/ms 1.52 |
| warm-up | 3.79 | 38.6 | render/thread/ms 45.42, render/busy/ms 43.89, main/busy/ms 30.86, terrain/update/ms 6.16, render/graph/ms 3.54, stream/drive/ms 3.02, stream/spawn/ms 1.36, render/present/ms 1.25, stream/tick/ms 0.99, terrain/select/ms 0.84, bigspace/total/ms 0.50, render/acquire/ms 0.28 |
| warm-up | 3.90 | 73.3 | pipelines/wait_ms 2392, pipelines/compile_ms 188, pipelines/front_ms 176, render/thread/ms 42.11, render/busy/ms 41.03, main/busy/ms 40.66, stream/drive/ms 28.30, pipelines/driver_ms 12.38, render/graph/ms 2.65, render/present/ms 0.94, stream/tick/ms 0.87, terrain/update/ms 0.73 |
| warm-up | 4.03 | 86.3 | pipelines/wait_ms 2617, pipelines/compile_ms 78.33, pipelines/front_ms 72.12, render/thread/ms 59.10, render/busy/ms 57.90, main/busy/ms 46.03, stream/drive/ms 25.05, pipelines/driver_ms 6.21, render/graph/ms 2.59, stream/tick/ms 1.93, stream/spawn/ms 1.45, render/present/ms 1.00 |
| warm-up | 4.15 | 82.9 | pipelines/wait_ms 2236, pipelines/compile_ms 163, pipelines/front_ms 155, main/busy/ms 55.79, render/thread/ms 55.63, render/busy/ms 54.35, stream/drive/ms 28.81, pipelines/driver_ms 7.63, render/graph/ms 2.57, stream/spawn/ms 2.35, render/present/ms 1.10, terrain/update/ms 1.09 |
| warm-up | 4.19 | 45.5 | pipelines/wait_ms 4712, pipelines/compile_ms 198, pipelines/front_ms 185, render/thread/ms 71.81, render/busy/ms 70.38, main/busy/ms 33.23, pipelines/driver_ms 12.41, stream/drive/ms 4.63, render/graph/ms 4.06, stream/spawn/ms 1.69, stream/tick/ms 1.15, render/present/ms 1.12 |
| warm-up | 4.29 | 36.6 | render/thread/ms 56.39, render/busy/ms 54.09, render/graph/ms 26.10, main/busy/ms 19.07, stream/drive/ms 4.58, render/present/ms 2.07, stream/spawn/ms 1.77, stream/tick/ms 0.88, stream/activate/ms 0.71, terrain/update/ms 0.64, terrain/select/ms 0.53, physics/insert/ms 0.35 |
| warm-up | 4.36 | 45.3 | main/busy/ms 71.58, stream/drive/ms 51.92, render/thread/ms 43.27, render/busy/ms 41.87, render/graph/ms 3.87, stream/spawn/ms 2.44, stream/tick/ms 1.34, render/present/ms 1.16, terrain/update/ms 1.15, terrain/select/ms 1.02, bigspace/total/ms 0.48, stream/activate/ms 0.27 |
| warm-up | 4.42 | 59.1 | pipelines/wait_ms 715, pipelines/compile_ms 188, pipelines/front_ms 177, main/busy/ms 68.30, stream/drive/ms 51.29, render/thread/ms 42.59, render/busy/ms 41.53, pipelines/driver_ms 11.18, render/graph/ms 2.80, stream/spawn/ms 2.15, terrain/update/ms 1.28, terrain/select/ms 1.12 |
| warm-up | 4.51 | 76.5 | pipelines/wait_ms 2581, pipelines/compile_ms 125, pipelines/front_ms 115, main/busy/ms 69.60, stream/drive/ms 48.02, render/thread/ms 35.65, render/busy/ms 34.47, pipelines/driver_ms 10.28, render/graph/ms 3.10, stream/spawn/ms 2.18, render/present/ms 0.90, stream/tick/ms 0.79 |
| warm-up | 4.57 | 48.3 | main/busy/ms 45.54, render/thread/ms 31.58, stream/drive/ms 30.93, render/busy/ms 30.05, render/graph/ms 3.82, render/present/ms 1.31, stream/activate/ms 1.04, stream/spawn/ms 1.00, terrain/update/ms 0.92, stream/tick/ms 0.88, terrain/select/ms 0.76, physics/insert/ms 0.37 |
| warm-up | 4.62 | 38.4 | main/busy/ms 40.23, stream/drive/ms 29.19, render/thread/ms 18.78, render/busy/ms 17.56, render/graph/ms 3.43, stream/spawn/ms 1.23, stream/activate/ms 1.04, render/present/ms 0.98, stream/tick/ms 0.78, terrain/update/ms 0.65, terrain/select/ms 0.55, physics/insert/ms 0.43 |
| warm-up | 5.38 | 59.2 | pipelines/wait_ms 9248, pipelines/compile_ms 122, pipelines/front_ms 114, render/thread/ms 71.21, render/busy/ms 69.31, main/busy/ms 56.41, render/graph/ms 8.61, pipelines/driver_ms 7.81, stream/drive/ms 4.69, stream/spawn/ms 2.80, render/present/ms 1.53, stream/tick/ms 0.87 |
| warm-up | 5.46 | 46.8 | render/thread/ms 52.79, render/busy/ms 51.47, main/busy/ms 25.97, render/graph/ms 5.12, stream/drive/ms 4.45, stream/spawn/ms 2.38, render/present/ms 1.14, stream/tick/ms 0.87, terrain/update/ms 0.80, terrain/select/ms 0.67, bigspace/total/ms 0.58, bigspace/high_precision/ms 0.29 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\h1_old\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **68%** (min 49%) · 2580 MHz (min 2580) · memory 8001 MHz · 69.1 W · 73 °C (max 78) · VRAM 5419 MiB · throttled 100% of samples
