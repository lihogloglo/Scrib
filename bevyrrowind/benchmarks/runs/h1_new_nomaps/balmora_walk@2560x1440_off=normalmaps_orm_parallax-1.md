# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5685 frames over 65.6 s · **median 11.27 ms (89 fps)** · mean 11.55 · p95 13.97 · p99 17.12 (1% low 58 fps) · worst 49.54 ms · 66 spikes over 1.5× median · 48.4 stutters/min (work 47.5/min)
- warm-up: 403 frames over 8.8 s, worst 2511.5 ms
- hitches (> 28.2 ms): **5** measured, 32 in warm-up
- timed GPU passes: 7.20 ms of a 11.55 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.27 ms, limit 11 ms
- **FAIL** frame_ms: 53 stutters (> 1.5x moving median of 60) in 1.09 min = 48.44/min, limit 1/min
- **FAIL** worst frame 49.54 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.074 | 2.191 | 2.523 | 0.95 | 0.180 |
| main_opaque_pass_3d | 1.478 | 1.561 | 2.225 | 0.95 | 0.059 |
| early prepass | 0.470 | 0.497 | 0.759 | 0.95 | 0.095 |
| bloom | 0.319 | 0.337 | 0.488 | 0.95 | 0.066 |
| atmosphere_luts | 0.310 | 0.327 | 0.461 | 0.95 | 0.003 |
| water_velocity | 0.286 | 0.302 | 0.379 | 0.95 | 0.002 |
| uniform_allocation | 0.245 | 0.258 | 0.261 | 0.95 | 0.049 |
| water_sspr | 0.218 | 0.231 | 0.235 | 0.95 | 0.002 |
| ssao | 0.192 | 0.203 | 0.211 | 0.95 | 0.008 |
| bin_unpacking | 0.178 | 0.188 | 0.196 | 0.95 | 0.027 |
| atmos_froxel_inject | 0.148 | 0.156 | 0.152 | 0.95 | 0.002 |
| clouds_trace | 0.133 | 0.141 | 0.233 | 0.95 | 0.001 |
| water_fft | 0.132 | 0.139 | 0.138 | 0.95 | 0.003 |
| upscaling | 0.120 | 0.127 | 0.151 | 0.95 | 0.004 |
| clouds_composite | 0.105 | 0.111 | 0.111 | 0.95 | 0.002 |
| auxiliary_views | 0.098 | 0.103 | 0.464 | 0.95 | 0.101 |
| tonemapping | 0.087 | 0.091 | 0.092 | 0.95 | 0.004 |
| main_transmissive_pass_3d | 0.086 | 0.091 | 0.171 | 0.95 | 0.008 |
| clustering | 0.065 | 0.069 | 0.070 | 0.95 | 0.020 |
| early_mesh_preprocessing | 0.054 | 0.057 | 0.061 | 0.95 | 0.005 |
| render_sky | 0.052 | 0.055 | 0.054 | 0.95 | 0.002 |
| clouds_resolve | 0.047 | 0.050 | 0.051 | 0.95 | 0.001 |
| lightprobe_irradiance_map | 0.045 | 0.443 | 0.759 | 0.10 | 0.001 |
| surface_trail | 0.042 | 0.059 | 0.057 | 0.70 | 0.002 |
| water_snapshot | 0.033 | 0.035 | 0.046 | 0.95 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.95 | 0.003 |
| atmos_froxel_integrate | 0.026 | 0.028 | 0.031 | 0.95 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.248 | 0.561 | 0.10 | 0.004 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.95 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.023 | 0.024 | 0.95 | 0.006 |
| main_transparent_pass_3d | 0.017 | 0.018 | 0.032 | 0.95 | 0.154 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.95 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.95 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.95 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.95 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.007 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 19.786 | 0.000 | 9688.22 | 112483.7 |
| render/thread/ms | 10.269 | 12.516 | 48.30 | 58382.0 |
| render/busy/ms | 9.628 | 11.835 | 47.67 | 54732.7 |
| main/busy/ms | 8.338 | 10.025 | 24.67 | 47399.8 |
| render/graph/ms | 5.147 | 6.409 | 43.37 | 29260.4 |
| render/present/ms | 0.547 | 1.371 | 13.24 | 3109.3 |
| bigspace/total/ms | 0.447 | 0.714 | 2.89 | 2543.9 |
| pipelines/compile_ms | 0.366 | 0.000 | 161.40 | 2082.2 |
| pipelines/front_ms | 0.353 | 0.000 | 155.26 | 2007.6 |
| controller/feed/ms | 0.299 | 0.394 | 0.84 | 1702.2 |
| terrain/update/ms | 0.298 | 0.379 | 2.00 | 1695.4 |
| terrain/select/ms | 0.265 | 0.336 | 0.68 | 1507.8 |
| stream/drive/ms | 0.248 | 0.342 | 17.18 | 1410.1 |
| far/select/ms | 0.197 | 0.275 | 0.77 | 1120.7 |
| stream/tick/ms | 0.194 | 0.265 | 0.72 | 1100.2 |
| bigspace/low_precision/ms | 0.172 | 0.313 | 2.47 | 976.5 |
| anim/evaluate_cpu/ms | 0.159 | 0.199 | 0.44 | 904.1 |
| controller/move/ms | 0.139 | 0.303 | 0.69 | 787.9 |
| anim/evaluate/ms | 0.132 | 0.264 | 2.43 | 750.4 |
| bigspace/high_precision/ms | 0.108 | 0.224 | 1.67 | 616.7 |
| render/acquire/ms | 0.095 | 0.125 | 0.43 | 540.0 |
| physics/step/ms | 0.050 | 0.089 | 0.69 | 281.4 |
| physics/insert/ms | 0.041 | 0.408 | 0.89 | 232.0 |
| weather/map/ms | 0.025 | 0.033 | 0.34 | 139.5 |
| pipelines/driver_ms | 0.013 | 0.000 | 9.08 | 74.7 |
| anim/apply/ms | 0.012 | 0.022 | 0.36 | 65.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.38 | 60.4 |
| controller/avoid/ms | 0.010 | 0.014 | 0.11 | 58.9 |
| script/update/ms | 0.007 | 0.009 | 0.15 | 39.8 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 30.2 |
| stream/despawn/ms | 0.005 | 0.000 | 1.08 | 29.1 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 28.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.09 | 25.9 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 19.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.66 | 13.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 7 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 18 | 41 | 18 |
| controller/movers/count | count | 19 | 21 | 18 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1281 | 2262 | 537 |
| crowd/c0/meshes/count | count | 352 | 646 | 146 |
| crowd/c1/count | count | 146 | 156 | 149 |
| crowd/c1/joints/count | count | 5531 | 5995 | 5659 |
| crowd/c1/meshes/count | count | 1166 | 1297 | 1195 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4253 | 7315 | 6179 |
| crowd/joints_uploaded/count | count | 4230 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.40 | 47.67 | 8.93 |
| gpu/timed/ms | ms | 8.33 | 11.92 | 8.93 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 245 | 479 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3887 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 245 | 479 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3887 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 4100 | 4327 | 4326 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 4100 | 4327 | 4326 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 53 | 291 | 42 |
| phases/shadow/bins | count | 12.1 k | 15.6 k | 11.9 k |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 38.9 k | 42.2 k | 39.3 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 145 | 309 | 42 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2679 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 309 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 204 | 208 | 208 |
| pipelines/waiting | count | 0 | 28 | 0 |
| preprocess/bind_groups | count | 12 | 755 | 6 |
| preprocess/bind_groups_created | count | 12 | 755 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 228 | 330 | 221 |
| process/cpu_usage | % | 24.87 | 40.87 | 32.35 |
| process/mem_usage | GiB | 5.21 | 5.50 | 5.50 |
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
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.88 | 51.41 | 39.52 |
| system/mem_usage | % | 63.87 | 64.75 | 64.71 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2591 | 2644 | 2644 |
| views/all/batch_sets | count | 111 | 345 | 102 |
| views/all/bins | count | 21.3 k | 23.9 k | 21.5 k |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 67.5 k | 70.5 k | 68.9 k |
| views/all/pipelines | count | 199 | 362 | 98 |
| views/all/pipelines_distinct | count | 83 | 93 | 77 |
| views/all/work_items | count | 66.7 k | 69.7 k | 69.6 k |
| views/cascade/batch_sets | count | 52 | 67 | 42 |
| views/cascade/bins | count | 11.8 k | 12.5 k | 11.9 k |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 38.3 k | 39.9 k | 39.3 k |
| views/cascade/pipelines | count | 52 | 67 | 42 |
| views/cascade/work_items | count | 38.3 k | 39.9 k | 39.3 k |
| views/lamp_face/batch_sets | count | 0 | 234 | 0 |
| views/lamp_face/bins | count | 1650 | 3771 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1773 | 4130 | 0 |
| views/lamp_face/pipelines | count | 87 | 249 | 0 |
| views/lamp_face/work_items | count | 0 | 3811 | 0 |
| views/main/batch_sets | count | 60 | 64 | 60 |
| views/main/bins | count | 8662 | 9594 | 9594 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.0 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 56 | 60 | 56 |
| views/main/work_items | count | 28.5 k | 30.4 k | 30.3 k |
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
| anim/bones_written/count | count | 53.08 | 291 | 301.8 k |
| anim/events/count | count | 0.09 | 3 | 516 |
| assets/Image/added | count | 0.11 | 43 | 627 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.73 | 152 | 4160 |
| assets/Mesh/removed | count | 0.86 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 500 |
| assets/StandardMaterial/modified | count | 3.83 | 29 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.46 | 1877 | 8293 |
| physics/inserts | count | 0.45 | 111 | 2571 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 145 | 2335 |
| physics/steps | count | 0.69 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 11 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 19 | 378 |
| stream/upload/bytes | bytes | 5955 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1306 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.2 ms; slowest 37, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2511.5 | main/busy/ms 2486, surface/advance_cpu_ms 71.72, surface/cell_upload_ms 0.74, stream/drive/ms 0.33, weather/map/ms 0.11, bigspace/total/ms 0.11, stream/tick/ms 0.09, bigspace/high_precision/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.02, script/input/ms 0.01 |
| warm-up | 3.04 | 528.3 | main/busy/ms 2520, render/thread/ms 496, render/busy/ms 379, render/acquire/ms 116, surface/advance_cpu_ms 71.72, render/graph/ms 4.87, render/present/ms 1.86, stream/drive/ms 1.27, stream/tick/ms 0.76, surface/cell_upload_ms 0.74, terrain/update/ms 0.38, terrain/select/ms 0.25 |
| warm-up | 3.31 | 268.1 | pipelines/wait_ms 3062, render/thread/ms 509, render/busy/ms 391, pipelines/compile_ms 284, pipelines/front_ms 265, render/acquire/ms 116, main/busy/ms 50.84, pipelines/driver_ms 18.39, terrain/update/ms 17.81, render/graph/ms 9.46, render/present/ms 2.71, stream/drive/ms 1.69 |
| warm-up | 3.56 | 114.2 | pipelines/compile_ms 247, pipelines/front_ms 235, pipelines/wait_ms 234, main/busy/ms 73.92, render/thread/ms 62.92, render/busy/ms 60.89, pipelines/driver_ms 12.25, render/graph/ms 6.30, stream/drive/ms 2.18, render/present/ms 1.74, stream/tick/ms 1.71, terrain/update/ms 0.96 |
| warm-up | 3.61 | 32.7 | render/thread/ms 43.89, render/busy/ms 41.88, main/busy/ms 38.21, terrain/update/ms 6.50, stream/drive/ms 4.16, render/graph/ms 3.61, stream/tick/ms 1.76, render/present/ms 1.68, stream/spawn/ms 1.63, terrain/select/ms 0.79, bigspace/total/ms 0.59, render/acquire/ms 0.33 |
| warm-up | 3.73 | 60.3 | main/busy/ms 44.53, render/thread/ms 39.98, render/busy/ms 38.55, stream/drive/ms 31.18, render/graph/ms 3.42, stream/spawn/ms 1.47, render/present/ms 1.22, stream/tick/ms 0.79, terrain/update/ms 0.69, terrain/select/ms 0.58, bigspace/total/ms 0.22, render/acquire/ms 0.20 |
| warm-up | 3.82 | 42.4 | pipelines/wait_ms 3435, pipelines/compile_ms 101, pipelines/front_ms 91.25, render/thread/ms 39.10, render/busy/ms 36.86, main/busy/ms 23.06, pipelines/driver_ms 9.42, render/graph/ms 5.61, stream/drive/ms 2.41, render/present/ms 1.86, stream/tick/ms 1.22, terrain/update/ms 0.79 |
| warm-up | 3.93 | 82.8 | render/thread/ms 73.23, render/busy/ms 71.32, main/busy/ms 42.14, stream/drive/ms 24.49, render/graph/ms 4.22, render/present/ms 1.65, stream/spawn/ms 1.07, stream/tick/ms 0.93, terrain/update/ms 0.78, terrain/select/ms 0.65, stream/activate/ms 0.39, bigspace/total/ms 0.27 |
| warm-up | 4.06 | 115.1 | pipelines/wait_ms 5969, pipelines/compile_ms 216, pipelines/front_ms 207, render/thread/ms 76.85, render/busy/ms 54.10, main/busy/ms 45.44, stream/drive/ms 27.59, render/present/ms 22.57, pipelines/driver_ms 9.12, render/graph/ms 3.40, stream/spawn/ms 1.79, stream/tick/ms 1.08 |
| warm-up | 4.10 | 34.5 | pipelines/wait_ms 7386, pipelines/compile_ms 274, pipelines/front_ms 226, render/thread/ms 90.88, render/busy/ms 67.70, pipelines/driver_ms 47.26, main/busy/ms 26.96, render/present/ms 22.88, render/graph/ms 6.28, stream/drive/ms 4.01, stream/tick/ms 1.23, terrain/update/ms 0.77 |
| warm-up | 4.13 | 29.9 | pipelines/wait_ms 1417, pipelines/compile_ms 57.75, render/thread/ms 52.27, render/busy/ms 50.37, pipelines/driver_ms 38.14, main/busy/ms 24.78, pipelines/front_ms 19.62, render/graph/ms 13.05, terrain/update/ms 4.87, stream/drive/ms 3.83, render/present/ms 1.49, stream/tick/ms 1.09 |
| warm-up | 4.18 | 31.2 | pipelines/wait_ms 836, render/thread/ms 34.41, render/busy/ms 33.18, main/busy/ms 27.87, stream/drive/ms 14.80, render/graph/ms 8.30, pipelines/compile_ms 3.15, pipelines/front_ms 2.68, render/present/ms 1.04, stream/tick/ms 0.95, terrain/update/ms 0.77, terrain/select/ms 0.63 |
| warm-up | 4.24 | 46.2 | pipelines/wait_ms 3624, pipelines/compile_ms 204, pipelines/front_ms 189, main/busy/ms 49.68, stream/drive/ms 37.76, render/thread/ms 26.15, render/busy/ms 24.61, pipelines/driver_ms 15.56, render/graph/ms 6.07, render/present/ms 1.27, stream/spawn/ms 1.19, stream/tick/ms 0.92 |
| warm-up | 4.31 | 62.3 | pipelines/wait_ms 1873, pipelines/compile_ms 73.07, pipelines/front_ms 68.12, main/busy/ms 64.14, stream/drive/ms 46.01, render/thread/ms 24.83, render/busy/ms 23.61, pipelines/driver_ms 4.95, render/graph/ms 3.66, stream/spawn/ms 2.09, render/present/ms 1.06, stream/tick/ms 0.98 |
| warm-up | 4.38 | 47.0 | pipelines/wait_ms 7413, main/busy/ms 54.81, pipelines/compile_ms 49.46, pipelines/front_ms 43.47, render/thread/ms 40.02, render/busy/ms 38.53, stream/drive/ms 35.40, pipelines/driver_ms 5.99, render/graph/ms 5.11, stream/spawn/ms 2.11, stream/tick/ms 1.33, render/present/ms 1.10 |
| warm-up | 4.43 | 33.1 | main/busy/ms 37.83, stream/drive/ms 25.45, render/thread/ms 24.67, render/busy/ms 23.34, render/graph/ms 3.89, render/present/ms 1.10, stream/activate/ms 1.10, stream/tick/ms 0.85, terrain/update/ms 0.66, physics/insert/ms 0.63, terrain/select/ms 0.55, stream/spawn/ms 0.29 |
| warm-up | 4.57 | 36.7 | pipelines/wait_ms 1029, pipelines/compile_ms 178, pipelines/front_ms 156, render/thread/ms 43.64, render/busy/ms 41.79, main/busy/ms 29.48, pipelines/driver_ms 21.49, stream/drive/ms 7.19, render/graph/ms 6.12, stream/spawn/ms 4.05, stream/tick/ms 1.85, render/present/ms 1.55 |
| warm-up | 5.15 | 30.6 | pipelines/wait_ms 9763, pipelines/compile_ms 72.94, pipelines/front_ms 67.79, render/thread/ms 48.33, render/busy/ms 46.46, main/busy/ms 22.00, pipelines/driver_ms 5.15, render/graph/ms 4.78, stream/drive/ms 4.52, stream/spawn/ms 1.93, render/present/ms 1.58, terrain/update/ms 0.76 |
| warm-up | 5.22 | 41.5 | pipelines/wait_ms 2620, render/thread/ms 49.00, render/busy/ms 47.73, pipelines/compile_ms 29.38, pipelines/front_ms 25.63, main/busy/ms 24.76, render/graph/ms 4.99, stream/drive/ms 4.47, pipelines/driver_ms 3.75, stream/spawn/ms 2.00, stream/tick/ms 1.27, render/present/ms 1.05 |
| warm-up | 5.25 | 31.5 | pipelines/wait_ms 6816, pipelines/compile_ms 82.26, pipelines/front_ms 75.46, render/thread/ms 58.46, render/busy/ms 57.16, main/busy/ms 39.55, pipelines/driver_ms 6.81, render/graph/ms 4.79, stream/drive/ms 3.40, physics/insert/ms 1.39, stream/spawn/ms 1.14, render/present/ms 1.04 |
| warm-up | 5.29 | 36.4 | pipelines/wait_ms 15.8 k, pipelines/compile_ms 119, pipelines/front_ms 110, render/thread/ms 61.05, render/busy/ms 59.71, main/busy/ms 37.12, pipelines/driver_ms 9.24, render/graph/ms 5.58, stream/drive/ms 2.97, physics/insert/ms 1.43, stream/spawn/ms 1.26, render/present/ms 1.08 |
| warm-up | 5.35 | 49.0 | main/busy/ms 44.44, render/thread/ms 41.22, render/busy/ms 39.62, render/graph/ms 6.44, stream/drive/ms 4.32, stream/spawn/ms 2.17, physics/insert/ms 1.95, render/present/ms 1.41, stream/tick/ms 0.87, terrain/update/ms 0.66, bigspace/total/ms 0.56, terrain/select/ms 0.55 |
| warm-up | 5.41 | 40.8 | render/thread/ms 51.93, render/busy/ms 47.91, main/busy/ms 47.90, stream/drive/ms 8.70, render/graph/ms 6.21, render/present/ms 3.71, stream/spawn/ms 1.64, physics/insert/ms 1.31, bigspace/total/ms 0.90, stream/tick/ms 0.88, terrain/update/ms 0.79, terrain/select/ms 0.69 |
| warm-up | 5.46 | 45.4 | pipelines/wait_ms 8122, pipelines/compile_ms 176, pipelines/front_ms 170, render/thread/ms 66.66, render/busy/ms 62.73, main/busy/ms 38.17, stream/drive/ms 9.82, pipelines/driver_ms 5.53, render/graph/ms 5.36, render/present/ms 3.68, stream/spawn/ms 2.26, physics/insert/ms 1.78 |
| warm-up | 5.49 | 35.5 | pipelines/wait_ms 8122, pipelines/compile_ms 176, pipelines/front_ms 170, render/thread/ms 64.35, render/busy/ms 62.52, main/busy/ms 35.23, render/graph/ms 5.82, pipelines/driver_ms 5.53, stream/drive/ms 4.70, stream/spawn/ms 2.30, physics/insert/ms 1.77, render/present/ms 1.58 |
| warm-up | 5.53 | 31.1 | pipelines/wait_ms 2120, pipelines/compile_ms 57.39, render/thread/ms 54.69, pipelines/front_ms 54.25, render/busy/ms 53.18, main/busy/ms 35.50, render/graph/ms 5.65, stream/drive/ms 4.47, pipelines/driver_ms 3.14, stream/spawn/ms 2.10, stream/tick/ms 1.51, physics/insert/ms 1.35 |
| warm-up | 5.63 | 34.3 | render/thread/ms 46.04, render/busy/ms 44.69, main/busy/ms 30.43, stream/drive/ms 4.66, render/graph/ms 4.62, stream/spawn/ms 3.10, physics/insert/ms 2.15, render/present/ms 1.01, bigspace/total/ms 0.89, stream/tick/ms 0.79, terrain/update/ms 0.61, terrain/select/ms 0.53 |
| warm-up | 5.69 | 33.9 | pipelines/wait_ms 1414, pipelines/compile_ms 171, pipelines/front_ms 167, render/thread/ms 43.88, render/busy/ms 42.74, main/busy/ms 31.42, stream/drive/ms 13.69, render/graph/ms 5.03, pipelines/driver_ms 3.22, stream/spawn/ms 2.09, physics/insert/ms 1.21, render/present/ms 0.90 |
| warm-up | 5.96 | 32.2 | render/thread/ms 41.10, render/busy/ms 39.61, main/busy/ms 30.95, render/graph/ms 9.14, stream/drive/ms 2.28, render/present/ms 1.21, bigspace/total/ms 1.11, physics/insert/ms 0.98, stream/tick/ms 0.95, terrain/update/ms 0.65, bigspace/high_precision/ms 0.59, terrain/select/ms 0.56 |
| warm-up | 6.00 | 31.8 | render/thread/ms 55.96, render/busy/ms 54.51, main/busy/ms 36.82, render/graph/ms 6.85, stream/drive/ms 2.25, render/present/ms 1.11, stream/tick/ms 0.99, bigspace/total/ms 0.93, physics/insert/ms 0.85, terrain/update/ms 0.66, stream/spawn/ms 0.57, terrain/select/ms 0.57 |
| warm-up | 6.04 | 42.6 | render/thread/ms 55.82, render/busy/ms 54.57, main/busy/ms 31.78, render/graph/ms 5.75, stream/drive/ms 2.78, stream/spawn/ms 1.37, render/present/ms 0.99, bigspace/total/ms 0.75, stream/tick/ms 0.75, physics/insert/ms 0.73, terrain/update/ms 0.68, terrain/select/ms 0.59 |
| warm-up | 8.35 | 28.8 | pipelines/wait_ms 4648, pipelines/compile_ms 42.81, pipelines/front_ms 40.15, main/busy/ms 38.37, render/thread/ms 31.34, render/busy/ms 30.12, render/graph/ms 10.73, pipelines/driver_ms 2.66, render/present/ms 1.06, bigspace/total/ms 1.03, physics/insert/ms 0.91, anim/evaluate_cpu/ms 0.78 |
| measure | 0.39 | 30.1 | render/thread/ms 35.58, render/busy/ms 33.93, render/graph/ms 20.18, main/busy/ms 19.01, anim/evaluate/ms 2.51, render/present/ms 1.47, physics/insert/ms 0.99, bigspace/total/ms 0.83, terrain/update/ms 0.64, terrain/select/ms 0.59, stream/drive/ms 0.40, controller/feed/ms 0.38 |
| measure | 1.74 | 49.5 | render/thread/ms 58.65, render/busy/ms 57.41, render/graph/ms 48.81, main/busy/ms 17.80, render/present/ms 1.03, bigspace/total/ms 0.84, terrain/update/ms 0.55, controller/feed/ms 0.53, terrain/select/ms 0.50, stream/drive/ms 0.47, stream/tick/ms 0.41, far/select/ms 0.36 |
| measure | 2.17 | 31.7 | render/thread/ms 40.92, render/busy/ms 39.38, render/graph/ms 19.02, main/busy/ms 16.17, pipelines/compile_ms 11.44, pipelines/front_ms 11.21, render/present/ms 1.36, bigspace/total/ms 0.85, terrain/update/ms 0.56, terrain/select/ms 0.51, controller/feed/ms 0.43, stream/drive/ms 0.43 |
| measure | 19.27 | 28.4 | render/thread/ms 37.76, render/busy/ms 36.59, render/graph/ms 20.15, main/busy/ms 17.32, render/present/ms 0.97, bigspace/total/ms 0.71, controller/feed/ms 0.60, terrain/update/ms 0.56, terrain/select/ms 0.50, stream/drive/ms 0.43, stream/tick/ms 0.37, far/select/ms 0.34 |
| measure | 59.03 | 30.7 | render/thread/ms 40.13, render/busy/ms 39.02, main/busy/ms 23.50, render/graph/ms 9.50, controller/feed/ms 1.08, bigspace/total/ms 1.07, terrain/update/ms 1.07, far/select/ms 0.97, terrain/select/ms 0.95, render/present/ms 0.89, stream/drive/ms 0.58, stream/tick/ms 0.51 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\h1_new_nomaps\balmora_walk@2560x1440_off=normalmaps_orm_parallax-1 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 9%, CPU 20%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **86%** (min 58%) · 2574 MHz (min 2565) · memory 8001 MHz · 81.8 W · 77 °C (max 82) · VRAM 5299 MiB · throttled 100% of samples
