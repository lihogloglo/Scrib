# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5795 frames over 65.6 s · **median 11.07 ms (90 fps)** · mean 11.33 · p95 13.92 · p99 16.25 (1% low 62 fps) · worst 33.95 ms · 49 spikes over 1.5× median · 32.9 stutters/min (work 36.6/min)
- warm-up: 409 frames over 9.0 s, worst 2688.9 ms
- hitches (> 27.7 ms): **7** measured, 26 in warm-up
- timed GPU passes: 7.15 ms of a 11.33 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.07 ms, limit 11 ms
- **FAIL** frame_ms: 36 stutters (> 1.5x moving median of 60) in 1.09 min = 32.90/min, limit 1/min
- **FAIL** worst frame 33.95 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.065 | 2.195 | 2.588 | 0.94 | 0.181 |
| main_opaque_pass_3d | 1.470 | 1.563 | 2.257 | 0.94 | 0.056 |
| early prepass | 0.465 | 0.494 | 0.755 | 0.94 | 0.094 |
| bloom | 0.315 | 0.335 | 0.338 | 0.94 | 0.066 |
| atmosphere_luts | 0.310 | 0.329 | 0.470 | 0.94 | 0.004 |
| water_velocity | 0.286 | 0.304 | 0.499 | 0.94 | 0.002 |
| water_sspr | 0.217 | 0.231 | 0.236 | 0.94 | 0.002 |
| uniform_allocation | 0.210 | 0.223 | 0.227 | 0.94 | 0.039 |
| ssao | 0.190 | 0.202 | 0.209 | 0.94 | 0.008 |
| bin_unpacking | 0.169 | 0.180 | 0.183 | 0.94 | 0.028 |
| atmos_froxel_inject | 0.148 | 0.157 | 0.154 | 0.94 | 0.002 |
| auxiliary_views | 0.142 | 0.151 | 0.600 | 0.94 | 0.143 |
| clouds_trace | 0.134 | 0.142 | 0.236 | 0.94 | 0.001 |
| water_fft | 0.133 | 0.142 | 0.139 | 0.94 | 0.003 |
| upscaling | 0.124 | 0.132 | 0.160 | 0.94 | 0.004 |
| clouds_composite | 0.104 | 0.111 | 0.111 | 0.94 | 0.002 |
| tonemapping | 0.087 | 0.092 | 0.091 | 0.94 | 0.004 |
| main_transmissive_pass_3d | 0.085 | 0.090 | 0.172 | 0.94 | 0.009 |
| clustering | 0.065 | 0.069 | 0.070 | 0.94 | 0.020 |
| render_sky | 0.052 | 0.055 | 0.054 | 0.94 | 0.002 |
| clouds_resolve | 0.047 | 0.050 | 0.052 | 0.94 | 0.001 |
| lightprobe_irradiance_map | 0.046 | 0.444 | 0.754 | 0.10 | 0.001 |
| early_mesh_preprocessing | 0.041 | 0.044 | 0.049 | 0.94 | 0.005 |
| surface_trail | 0.041 | 0.059 | 0.057 | 0.69 | 0.002 |
| water_snapshot | 0.034 | 0.036 | 0.046 | 0.94 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.94 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.028 | 0.031 | 0.94 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.238 | 0.240 | 0.10 | 0.005 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.94 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.022 | 0.023 | 0.94 | 0.006 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.031 | 0.94 | 0.111 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.94 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.010 | 0.94 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.94 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.94 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 13.822 | 0.000 | 8678.81 | 80098.6 |
| render/thread/ms | 10.105 | 12.527 | 32.75 | 58556.2 |
| render/busy/ms | 9.471 | 11.953 | 31.99 | 54884.9 |
| main/busy/ms | 8.697 | 10.717 | 28.37 | 50400.5 |
| render/graph/ms | 5.274 | 6.785 | 14.91 | 30562.4 |
| pipelines/compile_ms | 2.104 | 0.000 | 2581.90 | 12192.3 |
| pipelines/driver_ms | 1.744 | 0.000 | 2452.36 | 10107.9 |
| render/present/ms | 0.525 | 1.320 | 16.64 | 3043.5 |
| bigspace/total/ms | 0.516 | 1.082 | 2.98 | 2987.7 |
| pipelines/front_ms | 0.360 | 0.000 | 147.36 | 2084.4 |
| terrain/update/ms | 0.299 | 0.384 | 2.07 | 1731.3 |
| controller/feed/ms | 0.288 | 0.382 | 0.81 | 1667.3 |
| terrain/select/ms | 0.266 | 0.341 | 0.90 | 1541.0 |
| stream/drive/ms | 0.256 | 0.361 | 20.54 | 1486.2 |
| bigspace/low_precision/ms | 0.215 | 0.555 | 2.19 | 1245.1 |
| stream/tick/ms | 0.199 | 0.281 | 1.04 | 1151.3 |
| far/select/ms | 0.198 | 0.279 | 0.61 | 1144.6 |
| anim/evaluate_cpu/ms | 0.167 | 0.208 | 0.51 | 965.8 |
| anim/evaluate/ms | 0.132 | 0.265 | 1.97 | 764.5 |
| bigspace/high_precision/ms | 0.132 | 0.323 | 2.69 | 763.7 |
| controller/move/ms | 0.130 | 0.316 | 0.74 | 756.2 |
| render/acquire/ms | 0.108 | 0.142 | 1.89 | 627.7 |
| physics/step/ms | 0.050 | 0.088 | 0.58 | 289.3 |
| physics/insert/ms | 0.046 | 0.422 | 1.15 | 268.3 |
| weather/map/ms | 0.025 | 0.034 | 0.21 | 143.4 |
| anim/apply/ms | 0.012 | 0.021 | 0.13 | 66.7 |
| weather/eval/ms | 0.011 | 0.015 | 0.12 | 61.9 |
| controller/avoid/ms | 0.010 | 0.014 | 0.23 | 60.2 |
| script/update/ms | 0.007 | 0.010 | 0.15 | 42.0 |
| script/input/ms | 0.005 | 0.007 | 0.15 | 29.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.04 | 29.7 |
| script/ui/ms | 0.005 | 0.007 | 0.05 | 29.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 27.9 |
| controller/write/ms | 0.004 | 0.005 | 0.31 | 21.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 14.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 4.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.05 | 1.9 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.08 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 11 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 45 | 16 |
| controller/movers/count | count | 19 | 22 | 16 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 547 |
| crowd/c0/meshes/count | count | 355 | 646 | 140 |
| crowd/c1/count | count | 145 | 155 | 149 |
| crowd/c1/joints/count | count | 5475 | 5951 | 5649 |
| crowd/c1/meshes/count | count | 1166 | 1289 | 1201 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 9816 | 6179 |
| crowd/joints_uploaded/count | count | 4247 | 6643 | 6179 |
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
| frame/work/ms | ms | 9.33 | 31.99 | 8.63 |
| gpu/timed/ms | ms | 8.29 | 12.32 | 8.63 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 819 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 819 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1290 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3417 | 5131 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1290 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3417 | 5131 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 50 | 285 | 39 |
| phases/shadow/bins | count | 3792 | 5445 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9595 | 12.9 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 136 | 302 | 39 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2345 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 1406 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 199 | 203 | 203 |
| pipelines/waiting | count | 0 | 20 | 0 |
| preprocess/bind_groups | count | 11 | 739 | 6 |
| preprocess/bind_groups_created | count | 11 | 739 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 224 | 320 | 215 |
| process/cpu_usage | % | 26.44 | 48.28 | 34.84 |
| process/mem_usage | GiB | 5.25 | 5.50 | 5.49 |
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
| started_load_count |  | 4766 | 4860 | 4860 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.34 | 56.11 | 30.36 |
| system/mem_usage | % | 75.67 | 76.52 | 76.42 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 108 | 337 | 99 |
| views/all/bins | count | 6486 | 8287 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.4 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 190 | 356 | 95 |
| views/all/pipelines_distinct | count | 81 | 93 | 77 |
| views/all/work_items | count | 17.4 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 48 | 60 | 39 |
| views/cascade/bins | count | 2613 | 3103 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8159 | 10.2 k | 5663 |
| views/cascade/pipelines | count | 48 | 60 | 39 |
| views/cascade/work_items | count | 8159 | 10.2 k | 5663 |
| views/lamp_face/batch_sets | count | 0 | 233 | 0 |
| views/lamp_face/bins | count | 1075 | 2506 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1219 | 2863 | 0 |
| views/lamp_face/pipelines | count | 86 | 242 | 0 |
| views/lamp_face/work_items | count | 0 | 2558 | 0 |
| views/main/batch_sets | count | 58 | 62 | 60 |
| views/main/bins | count | 2868 | 3586 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8468 | 13.0 k | 7592 |
| views/main/pipelines | count | 54 | 58 | 56 |
| views/main/work_items | count | 8627 | 13.2 k | 7900 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 46.41 | 449 | 268.9 k |
| anim/events/count | count | 0.10 | 5 | 558 |
| assets/Image/added | count | 0.10 | 43 | 599 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.71 | 152 | 4108 |
| assets/Mesh/removed | count | 0.84 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 474 |
| assets/StandardMaterial/modified | count | 3.76 | 22 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.33 | 1389 | 7710 |
| physics/inserts | count | 0.57 | 111 | 3331 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 5 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 5842 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1281 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.7 ms; slowest 33, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.69 | 2688.9 | main/busy/ms 2664, surface/advance_cpu_ms 71.00, surface/cell_upload_ms 0.74, stream/drive/ms 0.32, weather/map/ms 0.13, stream/tick/ms 0.09, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.45 | 761.8 | main/busy/ms 2701, render/thread/ms 722, render/busy/ms 580, render/acquire/ms 140, surface/advance_cpu_ms 71.00, render/graph/ms 4.98, render/present/ms 1.94, stream/drive/ms 1.10, surface/cell_upload_ms 0.74, stream/tick/ms 0.51, terrain/update/ms 0.48, terrain/select/ms 0.33 |
| warm-up | 3.69 | 238.4 | pipelines/wait_ms 2787, render/thread/ms 734, render/busy/ms 590, pipelines/compile_ms 247, pipelines/front_ms 231, render/acquire/ms 141, main/busy/ms 52.50, terrain/update/ms 19.17, pipelines/driver_ms 16.24, render/graph/ms 8.70, render/present/ms 3.25, stream/drive/ms 1.36 |
| warm-up | 3.93 | 78.5 | main/busy/ms 63.88, render/thread/ms 54.36, render/busy/ms 52.69, render/graph/ms 4.38, render/present/ms 1.38, stream/drive/ms 1.13, terrain/update/ms 0.88, stream/tick/ms 0.86, terrain/select/ms 0.74, render/acquire/ms 0.29, bigspace/total/ms 0.26, bigspace/high_precision/ms 0.09 |
| warm-up | 4.09 | 59.4 | pipelines/wait_ms 384, render/thread/ms 42.60, render/busy/ms 41.36, main/busy/ms 35.89, stream/drive/ms 24.23, render/graph/ms 3.40, pipelines/compile_ms 1.44, pipelines/front_ms 1.16, render/present/ms 1.07, stream/tick/ms 1.04, terrain/update/ms 0.68, terrain/select/ms 0.56 |
| warm-up | 4.15 | 59.3 | pipelines/wait_ms 3196, pipelines/compile_ms 235, pipelines/front_ms 217, main/busy/ms 40.45, render/thread/ms 34.62, render/busy/ms 32.72, stream/drive/ms 27.43, pipelines/driver_ms 17.83, render/graph/ms 5.60, render/present/ms 1.64, stream/spawn/ms 1.41, stream/tick/ms 1.23 |
| warm-up | 4.18 | 29.4 | pipelines/wait_ms 2959, pipelines/compile_ms 61.87, pipelines/front_ms 54.70, render/thread/ms 50.42, render/busy/ms 48.11, main/busy/ms 34.26, stream/drive/ms 12.08, pipelines/driver_ms 7.17, render/graph/ms 6.17, stream/spawn/ms 2.32, render/present/ms 2.00, stream/tick/ms 1.63 |
| warm-up | 4.26 | 77.0 | render/thread/ms 66.75, render/busy/ms 65.05, main/busy/ms 38.19, stream/drive/ms 11.85, render/graph/ms 4.53, stream/tick/ms 1.91, stream/spawn/ms 1.85, render/present/ms 1.45, terrain/update/ms 0.84, terrain/select/ms 0.68, bigspace/total/ms 0.32, render/acquire/ms 0.24 |
| warm-up | 4.30 | 34.6 | pipelines/wait_ms 209, pipelines/compile_ms 155, pipelines/front_ms 148, render/thread/ms 67.84, render/busy/ms 66.61, main/busy/ms 46.89, stream/drive/ms 27.83, pipelines/driver_ms 7.14, render/graph/ms 3.61, stream/tick/ms 1.29, render/present/ms 1.02, stream/spawn/ms 0.83 |
| warm-up | 4.34 | 46.8 | pipelines/wait_ms 2394, pipelines/compile_ms 207, pipelines/front_ms 197, render/thread/ms 46.65, render/busy/ms 45.46, main/busy/ms 43.87, stream/drive/ms 27.77, pipelines/driver_ms 10.35, render/graph/ms 3.04, stream/spawn/ms 1.05, render/present/ms 1.00, terrain/update/ms 0.95 |
| warm-up | 4.44 | 30.7 | pipelines/wait_ms 2044, render/thread/ms 40.02, render/busy/ms 37.98, main/busy/ms 29.04, pipelines/compile_ms 24.11, pipelines/front_ms 22.32, stream/drive/ms 15.04, render/graph/ms 10.67, stream/spawn/ms 2.40, render/present/ms 1.82, pipelines/driver_ms 1.79, stream/tick/ms 1.46 |
| warm-up | 4.47 | 34.5 | main/busy/ms 64.64, render/thread/ms 53.91, render/busy/ms 50.52, stream/drive/ms 48.47, render/graph/ms 16.44, stream/spawn/ms 3.69, render/present/ms 3.10, stream/tick/ms 1.92, terrain/update/ms 0.80, terrain/select/ms 0.68, bigspace/total/ms 0.44, render/acquire/ms 0.29 |
| warm-up | 4.56 | 87.2 | pipelines/wait_ms 1311, pipelines/compile_ms 152, pipelines/front_ms 149, render/thread/ms 74.45, render/busy/ms 71.67, main/busy/ms 66.54, stream/drive/ms 45.60, render/graph/ms 14.82, pipelines/driver_ms 3.41, render/present/ms 2.52, stream/spawn/ms 1.44, stream/tick/ms 1.40 |
| warm-up | 4.68 | 102.7 | pipelines/wait_ms 15.4 k, pipelines/compile_ms 139, pipelines/front_ms 123, main/busy/ms 69.31, render/thread/ms 62.97, render/busy/ms 60.63, stream/drive/ms 48.81, pipelines/driver_ms 16.31, render/graph/ms 7.52, stream/spawn/ms 2.29, render/present/ms 2.12, stream/tick/ms 1.02 |
| warm-up | 4.71 | 28.8 | pipelines/wait_ms 15.5 k, pipelines/compile_ms 154, pipelines/front_ms 136, render/thread/ms 76.75, render/busy/ms 74.82, main/busy/ms 53.16, stream/drive/ms 36.29, pipelines/driver_ms 17.66, render/graph/ms 10.02, stream/spawn/ms 2.21, render/present/ms 1.63, stream/tick/ms 0.98 |
| warm-up | 4.75 | 39.3 | pipelines/wait_ms 1703, main/busy/ms 48.39, render/thread/ms 42.42, render/busy/ms 40.62, stream/drive/ms 35.75, pipelines/compile_ms 33.46, pipelines/front_ms 31.35, render/graph/ms 7.52, pipelines/driver_ms 2.11, stream/spawn/ms 1.81, render/present/ms 1.51, stream/tick/ms 0.96 |
| warm-up | 4.79 | 34.4 | main/busy/ms 39.16, stream/drive/ms 24.46, render/thread/ms 20.05, render/busy/ms 18.93, render/graph/ms 4.68, stream/spawn/ms 1.36, stream/activate/ms 1.06, render/present/ms 0.92, terrain/update/ms 0.86, stream/tick/ms 0.85, terrain/select/ms 0.69, physics/insert/ms 0.58 |
| warm-up | 4.93 | 30.8 | pipelines/wait_ms 2342, pipelines/compile_ms 73.61, pipelines/front_ms 69.75, render/thread/ms 40.46, render/busy/ms 39.35, main/busy/ms 16.67, render/graph/ms 3.90, pipelines/driver_ms 3.86, stream/drive/ms 2.79, stream/spawn/ms 1.06, render/present/ms 0.83, stream/tick/ms 0.74 |
| warm-up | 5.26 | 37.7 | main/busy/ms 40.42, render/thread/ms 25.64, render/busy/ms 24.49, stream/drive/ms 12.04, render/graph/ms 4.25, render/present/ms 0.99, stream/tick/ms 0.87, terrain/update/ms 0.73, terrain/select/ms 0.62, bigspace/total/ms 0.42, bigspace/high_precision/ms 0.19, render/acquire/ms 0.17 |
| warm-up | 5.76 | 28.9 | render/thread/ms 36.45, render/busy/ms 35.34, main/busy/ms 27.99, stream/drive/ms 5.23, render/graph/ms 5.14, stream/spawn/ms 2.05, stream/tick/ms 2.02, render/present/ms 0.86, terrain/update/ms 0.83, terrain/select/ms 0.67, bigspace/total/ms 0.60, render/acquire/ms 0.25 |
| warm-up | 5.97 | 28.9 | render/thread/ms 46.48, render/busy/ms 45.17, main/busy/ms 35.15, stream/drive/ms 15.99, render/graph/ms 6.35, stream/spawn/ms 1.50, render/present/ms 1.03, bigspace/total/ms 0.74, stream/tick/ms 0.68, terrain/update/ms 0.68, terrain/select/ms 0.56, render/acquire/ms 0.28 |
| warm-up | 7.48 | 35.1 | pipelines/compile_ms 1606, pipelines/wait_ms 1568, pipelines/driver_ms 1468, pipelines/front_ms 138, main/busy/ms 49.89, render/thread/ms 30.67, render/busy/ms 28.76, render/graph/ms 12.89, render/present/ms 1.62, stream/drive/ms 1.21, physics/insert/ms 1.17, bigspace/total/ms 0.67 |
| warm-up | 7.51 | 35.7 | main/busy/ms 38.40, render/thread/ms 37.97, render/busy/ms 36.33, render/graph/ms 10.36, render/present/ms 1.36, stream/drive/ms 1.18, physics/insert/ms 1.05, bigspace/total/ms 0.78, physics/step/ms 0.67, stream/tick/ms 0.57, terrain/update/ms 0.55, terrain/select/ms 0.47 |
| warm-up | 7.64 | 30.4 | pipelines/wait_ms 2307, render/thread/ms 42.65, render/busy/ms 41.23, main/busy/ms 33.96, pipelines/compile_ms 26.04, pipelines/front_ms 24.16, render/graph/ms 7.47, pipelines/driver_ms 1.88, physics/insert/ms 1.15, render/present/ms 1.02, stream/drive/ms 1.02, bigspace/total/ms 0.71 |
| warm-up | 7.93 | 27.7 | main/busy/ms 35.15, render/thread/ms 32.25, render/busy/ms 30.96, render/graph/ms 12.02, physics/insert/ms 1.42, render/present/ms 1.12, stream/drive/ms 1.08, bigspace/total/ms 1.06, anim/evaluate_cpu/ms 0.64, anim/evaluate/ms 0.54, terrain/update/ms 0.54, stream/tick/ms 0.50 |
| warm-up | 8.56 | 29.3 | pipelines/wait_ms 4699, pipelines/compile_ms 42.15, pipelines/front_ms 40.26, main/busy/ms 38.38, render/thread/ms 25.76, render/busy/ms 24.47, render/graph/ms 10.61, terrain/update/ms 5.05, pipelines/driver_ms 1.88, physics/insert/ms 1.87, anim/evaluate/ms 1.09, render/present/ms 1.03 |
| measure | 0.79 | 33.9 | pipelines/wait_ms 5465, pipelines/compile_ms 65.94, pipelines/driver_ms 48.32, main/busy/ms 39.45, render/thread/ms 30.35, render/busy/ms 28.77, pipelines/front_ms 17.62, stream/drive/ms 16.57, render/graph/ms 9.79, render/present/ms 1.27, physics/insert/ms 1.05, bigspace/total/ms 0.86 |
| measure | 2.73 | 33.9 | render/thread/ms 43.94, render/busy/ms 42.62, render/graph/ms 20.43, main/busy/ms 20.10, render/present/ms 1.15, bigspace/total/ms 0.90, controller/feed/ms 0.59, terrain/update/ms 0.58, terrain/select/ms 0.52, stream/drive/ms 0.45, physics/insert/ms 0.43, bigspace/low_precision/ms 0.41 |
| measure | 18.18 | 30.9 | main/busy/ms 37.37, render/thread/ms 22.39, render/busy/ms 21.16, stream/drive/ms 20.77, render/graph/ms 10.81, render/present/ms 1.05, bigspace/total/ms 0.84, terrain/update/ms 0.68, controller/feed/ms 0.63, terrain/select/ms 0.61, far/select/ms 0.43, bigspace/low_precision/ms 0.41 |
| measure | 23.10 | 32.9 | render/thread/ms 46.07, render/busy/ms 44.46, main/busy/ms 35.52, render/graph/ms 17.60, anim/evaluate/ms 2.12, bigspace/total/ms 1.27, stream/drive/ms 1.22, render/present/ms 1.18, controller/feed/ms 0.97, anim/evaluate_cpu/ms 0.59, stream/tick/ms 0.57, terrain/update/ms 0.54 |
| measure | 39.06 | 30.6 | render/thread/ms 41.46, render/busy/ms 23.89, main/busy/ms 20.55, render/present/ms 17.17, render/graph/ms 12.46, bigspace/total/ms 1.19, controller/feed/ms 0.77, terrain/update/ms 0.71, stream/drive/ms 0.64, terrain/select/ms 0.63, stream/tick/ms 0.55, far/select/ms 0.48 |
| measure | 59.02 | 29.2 | render/thread/ms 40.82, render/busy/ms 38.90, main/busy/ms 31.35, render/graph/ms 19.00, stream/drive/ms 2.18, bigspace/total/ms 1.69, render/present/ms 1.53, stream/tick/ms 1.22, controller/feed/ms 1.14, anim/evaluate/ms 1.01, far/select/ms 0.98, stream/spawn/ms 0.83 |
| measure | 59.05 | 29.6 | render/thread/ms 43.19, render/busy/ms 41.79, main/busy/ms 27.34, render/graph/ms 13.45, bigspace/total/ms 1.49, stream/drive/ms 1.41, stream/tick/ms 1.30, render/present/ms 1.09, controller/feed/ms 0.98, terrain/update/ms 0.83, terrain/select/ms 0.74, far/select/ms 0.68 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_pbr\balmora_walk@2560x1440_off=normalmaps_orm_parallax-1 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **86%** (min 69%) · 2580 MHz (min 2580) · memory 8001 MHz · 81.8 W · 76 °C (max 81) · VRAM 6283 MiB · throttled 100% of samples
