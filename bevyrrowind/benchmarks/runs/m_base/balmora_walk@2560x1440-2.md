# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6454 frames over 65.6 s · **median 9.96 ms (100 fps)** · mean 10.17 · p95 12.58 · p99 13.99 (1% low 71 fps) · worst 27.52 ms · 32 spikes over 1.5× median · 19.2 stutters/min (work 14.6/min)
- warm-up: 337 frames over 4.5 s, worst 587.4 ms
- hitches (> 25.0 ms): **2** measured, 5 in warm-up
- timed GPU passes: 6.25 ms of a 10.17 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.96 ms, limit 11 ms
- **FAIL** frame_ms: 21 stutters (> 1.5x moving median of 60) in 1.09 min = 19.19/min, limit 1/min
- pass worst frame 27.52 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.033 | 2.239 | 2.789 | 0.91 | 0.175 |
| main_opaque_pass_3d | 1.331 | 1.466 | 2.100 | 0.91 | 0.046 |
| early prepass | 0.461 | 0.507 | 0.861 | 0.91 | 0.075 |
| bloom | 0.309 | 0.340 | 0.489 | 0.91 | 0.064 |
| atmosphere_luts | 0.300 | 0.330 | 0.512 | 0.91 | 0.003 |
| ssao | 0.201 | 0.221 | 0.352 | 0.91 | 0.007 |
| auxiliary_views | 0.171 | 0.188 | 0.632 | 0.91 | 0.165 |
| uniform_allocation | 0.169 | 0.186 | 0.187 | 0.91 | 0.030 |
| water_fft | 0.120 | 0.132 | 0.128 | 0.91 | 0.003 |
| bin_unpacking | 0.119 | 0.131 | 0.140 | 0.91 | 0.022 |
| upscaling | 0.119 | 0.131 | 0.158 | 0.91 | 0.004 |
| clouds_trace | 0.115 | 0.127 | 0.215 | 0.91 | 0.001 |
| atmos_froxel_inject | 0.090 | 0.099 | 0.108 | 0.91 | 0.001 |
| tonemapping | 0.084 | 0.092 | 0.092 | 0.91 | 0.003 |
| water_velocity | 0.079 | 0.087 | 0.085 | 0.91 | 0.002 |
| clustering | 0.063 | 0.069 | 0.070 | 0.91 | 0.019 |
| clouds_composite | 0.058 | 0.064 | 0.063 | 0.91 | 0.002 |
| main_transparent_pass_3d | 0.057 | 0.063 | 0.113 | 0.91 | 0.105 |
| render_sky | 0.049 | 0.054 | 0.054 | 0.91 | 0.002 |
| early_mesh_preprocessing | 0.040 | 0.044 | 0.048 | 0.91 | 0.005 |
| lightprobe_irradiance_map | 0.039 | 0.435 | 0.575 | 0.09 | 0.001 |
| surface_trail | 0.036 | 0.060 | 0.058 | 0.61 | 0.002 |
| clouds_resolve | 0.031 | 0.035 | 0.037 | 0.91 | 0.001 |
| clouds_shadow | 0.029 | 0.032 | 0.035 | 0.91 | 0.003 |
| atmos_froxel_integrate | 0.022 | 0.025 | 0.030 | 0.91 | 0.001 |
| water_snapshot | 0.022 | 0.024 | 0.024 | 0.91 | 0.001 |
| water_mips | 0.021 | 0.024 | 0.024 | 0.91 | 0.002 |
| lightprobe_radiance_map | 0.021 | 0.232 | 0.237 | 0.09 | 0.004 |
| per_view_shadow_pass | 0.020 | 0.023 | 0.025 | 0.91 | 0.006 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.91 | 0.002 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.90 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.91 | 0.003 |
| night_sky | 0.005 | 0.006 | 0.013 | 0.91 | 0.002 |
| lightprobe_copy | 0.001 | 0.014 | 0.014 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 19.376 | 0.000 | 7218.75 | 125055.8 |
| render/thread/ms | 8.969 | 11.274 | 28.56 | 57885.5 |
| render/busy/ms | 8.358 | 10.735 | 22.32 | 53940.7 |
| main/busy/ms | 8.119 | 9.870 | 15.23 | 52402.6 |
| render/graph/ms | 4.559 | 6.096 | 12.71 | 29421.3 |
| bigspace/total/ms | 0.525 | 1.189 | 2.72 | 3388.1 |
| render/present/ms | 0.514 | 1.198 | 14.94 | 3320.3 |
| pipelines/compile_ms | 0.348 | 0.000 | 153.60 | 2245.8 |
| pipelines/front_ms | 0.337 | 0.000 | 147.76 | 2175.8 |
| terrain/update/ms | 0.294 | 0.374 | 2.08 | 1899.0 |
| terrain/select/ms | 0.261 | 0.332 | 0.81 | 1687.2 |
| controller/feed/ms | 0.260 | 0.345 | 0.72 | 1675.7 |
| stream/drive/ms | 0.231 | 0.324 | 7.02 | 1488.0 |
| bigspace/low_precision/ms | 0.208 | 0.602 | 2.10 | 1343.2 |
| far/select/ms | 0.192 | 0.263 | 0.68 | 1239.9 |
| stream/tick/ms | 0.187 | 0.257 | 0.59 | 1205.2 |
| anim/evaluate_cpu/ms | 0.166 | 0.225 | 0.56 | 1070.6 |
| anim/evaluate/ms | 0.161 | 0.271 | 2.05 | 1039.0 |
| bigspace/high_precision/ms | 0.155 | 0.576 | 1.95 | 999.1 |
| controller/move/ms | 0.124 | 0.227 | 0.61 | 800.1 |
| render/acquire/ms | 0.097 | 0.124 | 1.40 | 624.4 |
| physics/step/ms | 0.043 | 0.082 | 0.54 | 279.9 |
| physics/insert/ms | 0.035 | 0.374 | 0.83 | 223.7 |
| weather/map/ms | 0.027 | 0.036 | 0.19 | 176.3 |
| anim/apply/ms | 0.012 | 0.022 | 0.20 | 77.7 |
| weather/eval/ms | 0.011 | 0.015 | 0.29 | 73.1 |
| pipelines/driver_ms | 0.011 | 0.000 | 5.84 | 70.0 |
| controller/avoid/ms | 0.011 | 0.014 | 0.23 | 69.0 |
| script/update/ms | 0.007 | 0.009 | 0.18 | 44.4 |
| script/input/ms | 0.005 | 0.006 | 0.18 | 30.3 |
| stream/despawn/ms | 0.004 | 0.000 | 1.04 | 28.3 |
| stream/spawn/ms | 0.004 | 0.000 | 1.01 | 23.7 |
| script/ui/ms | 0.004 | 0.005 | 0.22 | 23.1 |
| controller/write/ms | 0.003 | 0.004 | 0.13 | 21.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.62 | 13.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.13 | 6.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.02 | 0.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 7 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1306 | 2262 | 659 |
| crowd/c0/meshes/count | count | 355 | 646 | 171 |
| crowd/c1/count | count | 145 | 156 | 146 |
| crowd/c1/joints/count | count | 5527 | 5982 | 5505 |
| crowd/c1/meshes/count | count | 1166 | 1284 | 1162 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4263 | 10.6 k | 6179 |
| crowd/joints_uploaded/count | count | 4245 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.2 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 8.41 | 22.32 | 8.29 |
| gpu/timed/ms | ms | 7.56 | 9.88 | 7.82 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 812 | 1420 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1420 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1292 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3401 | 5175 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1292 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3401 | 5175 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 41 | 247 | 28 |
| phases/shadow/bins | count | 3761 | 5658 | 2222 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9530 | 13.0 k | 5673 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 109 | 270 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2346 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 605 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 173 | 175 | 175 |
| pipelines/waiting | count | 0 | 38 | 0 |
| preprocess/bind_groups | count | 10 | 637 | 6 |
| preprocess/bind_groups_created | count | 10 | 637 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 172 | 271 | 158 |
| process/cpu_usage | % | 26.92 | 41.83 | 33.93 |
| process/mem_usage | GiB | 2.72 | 2.89 | 2.89 |
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
| started_load_count |  | 3507 | 3575 | 3575 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 435 | 530 | 398 |
| statics/materials_bound | count | 435 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.23 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.84 | 51.38 | 36.74 |
| system/mem_usage | % | 55.08 | 55.49 | 55.20 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 86 | 287 | 72 |
| views/all/bins | count | 6516 | 8432 | 5702 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.2 k | 25.5 k | 13.3 k |
| views/all/pipelines | count | 151 | 307 | 68 |
| views/all/pipelines_distinct | count | 62 | 69 | 55 |
| views/all/work_items | count | 17.4 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 28 |
| views/cascade/bins | count | 2614 | 3201 | 2222 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8163 | 10.3 k | 5673 |
| views/cascade/pipelines | count | 36 | 50 | 28 |
| views/cascade/work_items | count | 8163 | 10.3 k | 5673 |
| views/lamp_face/batch_sets | count | 5 | 208 | 0 |
| views/lamp_face/bins | count | 1102 | 2701 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1227 | 3074 | 0 |
| views/lamp_face/pipelines | count | 72 | 222 | 0 |
| views/lamp_face/work_items | count | 75 | 2656 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2890 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8422 | 13.1 k | 7592 |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 8582 | 13.3 k | 7900 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 67.77 | 318 | 437.4 k |
| anim/events/count | count | 0.26 | 6 | 1672 |
| assets/Image/added | count | 0.05 | 35 | 310 |
| assets/Image/modified | count | 0.00 | 3 | 11 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.64 | 160 | 4100 |
| assets/Mesh/removed | count | 0.76 | 1242 | 4892 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 467 |
| assets/StandardMaterial/modified | count | 3.36 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.91 | 1388 | 5859 |
| physics/inserts | count | 0.43 | 111 | 2763 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 145 | 2335 |
| physics/steps | count | 0.61 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 7 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 21 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 24 | 378 |
| stream/upload/bytes | bytes | 5245 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1150 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 7, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 587.4 | main/busy/ms 565, surface/advance_cpu_ms 69.33, surface/cell_upload_ms 0.72, stream/drive/ms 0.33, weather/map/ms 0.14, stream/tick/ms 0.10, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.91 | 326.0 | main/busy/ms 600, render/thread/ms 292, render/busy/ms 222, surface/advance_cpu_ms 69.33, render/acquire/ms 68.03, render/graph/ms 6.05, render/present/ms 2.08, stream/drive/ms 1.33, stream/tick/ms 0.74, surface/cell_upload_ms 0.72, terrain/update/ms 0.61, terrain/select/ms 0.43 |
| warm-up | 0.99 | 79.1 | pipelines/wait_ms 562, render/thread/ms 303, render/busy/ms 231, pipelines/compile_ms 90.23, pipelines/front_ms 81.97, render/acquire/ms 68.21, main/busy/ms 51.13, terrain/update/ms 18.65, render/graph/ms 9.60, pipelines/driver_ms 8.27, render/present/ms 3.24, stream/drive/ms 1.52 |
| warm-up | 1.22 | 84.4 | main/busy/ms 61.04, render/thread/ms 50.98, render/busy/ms 49.34, render/graph/ms 3.90, stream/drive/ms 2.76, stream/tick/ms 2.37, render/present/ms 1.19, terrain/update/ms 0.87, terrain/select/ms 0.72, bigspace/total/ms 0.46, render/acquire/ms 0.45, bigspace/high_precision/ms 0.21 |
| warm-up | 3.34 | 26.5 | main/busy/ms 36.89, render/thread/ms 23.15, render/busy/ms 22.06, render/graph/ms 4.51, terrain/update/ms 2.65, bigspace/total/ms 1.98, stream/drive/ms 1.17, bigspace/high_precision/ms 1.05, render/present/ms 0.90, physics/insert/ms 0.75, anim/evaluate_cpu/ms 0.71, stream/tick/ms 0.56 |
| measure | 14.73 | 26.1 | render/thread/ms 34.84, render/busy/ms 19.55, main/busy/ms 18.16, render/present/ms 15.04, render/graph/ms 11.35, bigspace/total/ms 1.08, stream/drive/ms 0.58, controller/feed/ms 0.57, terrain/update/ms 0.57, stream/tick/ms 0.51, terrain/select/ms 0.51, far/select/ms 0.44 |
| measure | 44.68 | 27.5 | render/thread/ms 35.53, render/busy/ms 19.78, main/busy/ms 18.80, render/present/ms 15.56, render/graph/ms 11.37, bigspace/total/ms 0.97, controller/feed/ms 0.74, stream/drive/ms 0.60, terrain/update/ms 0.55, far/select/ms 0.52, stream/tick/ms 0.52, terrain/select/ms 0.49 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\m_base\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 17%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **87%** (min 63%) · 2580 MHz (min 2580) · memory 8001 MHz · 83.3 W · 75 °C (max 80) · VRAM 4391 MiB · throttled 100% of samples
