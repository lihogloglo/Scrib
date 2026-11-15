# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4876 frames over 65.7 s · **median 12.95 ms (77 fps)** · mean 13.46 · p95 16.94 · p99 19.14 (1% low 52 fps) · worst 32.84 ms · 43 spikes over 1.5× median · 19.2 stutters/min (work 19.2/min)
- warm-up: 471 frames over 9.9 s, worst 2596.2 ms
- hitches (> 32.4 ms): **1** measured, 18 in warm-up
- timed GPU passes: 7.76 ms of a 13.46 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.95 ms, limit 11 ms
- **FAIL** frame_ms: 21 stutters (> 1.5x moving median of 60) in 1.09 min = 19.19/min, limit 1/min
- pass worst frame 32.84 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.178 | 2.209 | 2.687 | 0.99 | 0.180 |
| main_opaque_pass_3d | 1.600 | 1.623 | 2.383 | 0.99 | 0.067 |
| early prepass | 0.548 | 0.556 | 0.902 | 0.99 | 0.116 |
| bloom | 0.336 | 0.341 | 0.491 | 0.99 | 0.067 |
| atmosphere_luts | 0.331 | 0.336 | 0.515 | 0.99 | 0.004 |
| water_velocity | 0.295 | 0.299 | 0.382 | 0.99 | 0.002 |
| uniform_allocation | 0.292 | 0.296 | 0.327 | 0.99 | 0.052 |
| bin_unpacking | 0.224 | 0.227 | 0.263 | 0.99 | 0.036 |
| water_sspr | 0.222 | 0.225 | 0.221 | 0.99 | 0.002 |
| ssao | 0.204 | 0.207 | 0.211 | 0.99 | 0.008 |
| atmos_froxel_inject | 0.160 | 0.162 | 0.153 | 0.99 | 0.002 |
| auxiliary_views | 0.148 | 0.150 | 0.663 | 0.99 | 0.135 |
| clouds_trace | 0.140 | 0.142 | 0.233 | 0.99 | 0.001 |
| water_fft | 0.137 | 0.139 | 0.137 | 0.99 | 0.003 |
| upscaling | 0.127 | 0.129 | 0.140 | 0.99 | 0.004 |
| clouds_composite | 0.112 | 0.114 | 0.111 | 0.99 | 0.002 |
| tonemapping | 0.090 | 0.091 | 0.091 | 0.99 | 0.004 |
| main_transmissive_pass_3d | 0.090 | 0.091 | 0.170 | 0.99 | 0.008 |
| clustering | 0.067 | 0.068 | 0.069 | 0.99 | 0.020 |
| render_sky | 0.053 | 0.054 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.442 | 0.752 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.99 | 0.001 |
| surface_trail | 0.048 | 0.059 | 0.057 | 0.82 | 0.002 |
| early_mesh_preprocessing | 0.044 | 0.044 | 0.049 | 0.99 | 0.005 |
| water_snapshot | 0.035 | 0.035 | 0.042 | 0.99 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.027 | 0.236 | 0.237 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.028 | 0.030 | 0.99 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.023 | 0.99 | 0.006 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.032 | 0.99 | 0.108 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 12.246 | 15.586 | 29.53 | 59710.4 |
| render/busy/ms | 11.670 | 14.983 | 29.00 | 56904.0 |
| main/busy/ms | 9.282 | 12.247 | 25.67 | 45259.7 |
| render/graph/ms | 7.101 | 9.325 | 18.30 | 34625.9 |
| pipelines/wait_ms | 5.715 | 0.000 | 5769.94 | 27863.9 |
| render/present/ms | 0.482 | 0.587 | 12.47 | 2352.1 |
| bigspace/total/ms | 0.449 | 0.716 | 4.07 | 2190.8 |
| terrain/update/ms | 0.291 | 0.369 | 2.00 | 1420.4 |
| controller/feed/ms | 0.273 | 0.368 | 0.77 | 1329.4 |
| terrain/select/ms | 0.259 | 0.327 | 0.59 | 1261.3 |
| stream/drive/ms | 0.257 | 0.373 | 17.56 | 1251.5 |
| pipelines/compile_ms | 0.240 | 0.000 | 149.20 | 1171.4 |
| pipelines/front_ms | 0.232 | 0.000 | 144.08 | 1129.5 |
| stream/tick/ms | 0.196 | 0.274 | 0.70 | 954.7 |
| far/select/ms | 0.194 | 0.270 | 0.67 | 948.3 |
| bigspace/low_precision/ms | 0.172 | 0.299 | 3.64 | 837.3 |
| anim/evaluate_cpu/ms | 0.164 | 0.213 | 0.52 | 801.8 |
| anim/evaluate/ms | 0.130 | 0.276 | 2.98 | 636.1 |
| controller/move/ms | 0.123 | 0.199 | 0.43 | 602.0 |
| bigspace/high_precision/ms | 0.110 | 0.229 | 3.68 | 538.5 |
| render/acquire/ms | 0.093 | 0.122 | 0.38 | 454.4 |
| physics/step/ms | 0.058 | 0.091 | 0.47 | 283.8 |
| physics/insert/ms | 0.054 | 0.435 | 1.07 | 263.3 |
| weather/map/ms | 0.025 | 0.034 | 0.27 | 120.9 |
| anim/apply/ms | 0.012 | 0.023 | 0.08 | 59.0 |
| controller/avoid/ms | 0.010 | 0.014 | 0.17 | 50.7 |
| weather/eval/ms | 0.010 | 0.014 | 0.11 | 49.9 |
| pipelines/driver_ms | 0.009 | 0.000 | 6.36 | 41.8 |
| script/update/ms | 0.007 | 0.010 | 0.06 | 35.1 |
| stream/despawn/ms | 0.006 | 0.000 | 1.12 | 30.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 26.3 |
| script/ui/ms | 0.005 | 0.007 | 0.13 | 24.3 |
| script/input/ms | 0.005 | 0.006 | 0.12 | 23.1 |
| controller/write/ms | 0.004 | 0.005 | 0.12 | 18.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.66 | 15.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.55 | 5.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 4.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.06 | 3.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.5 |
| stream/upload/ms | 0.000 | 0.000 | 0.15 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 5 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 29 | 18 |
| controller/movers/count | count | 19 | 21 | 18 |
| crowd/c0/count | count | 27 | 43 | 15 |
| crowd/c0/joints/count | count | 1277 | 2262 | 733 |
| crowd/c0/meshes/count | count | 353 | 646 | 187 |
| crowd/c1/count | count | 145 | 156 | 146 |
| crowd/c1/joints/count | count | 5498 | 5991 | 5463 |
| crowd/c1/meshes/count | count | 1162 | 1295 | 1154 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4276 | 9951 | 6179 |
| crowd/joints_uploaded/count | count | 4258 | 6643 | 6179 |
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
| frame/work/ms | ms | 11.27 | 29.00 | 10.69 |
| gpu/timed/ms | ms | 8.52 | 12.62 | 9.07 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 602.13 M | 603.31 M | 603.31 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 801 | 1416 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 801 | 1416 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 33 | 39 | 27 |
| phases/opaque3d/bins | count | 1300 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3375 | 5123 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 23 | 26 | 23 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 33 | 39 | 27 |
| phases/opaque3d_prepass/bins | count | 1300 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3375 | 5123 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 23 | 26 | 23 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 66 | 333 | 49 |
| phases/shadow/bins | count | 3697 | 5394 | 2220 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9392 | 12.4 k | 5671 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 143 | 306 | 42 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2347 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 347 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 214 | 218 | 218 |
| pipelines/waiting | count | 0 | 5 | 0 |
| preprocess/bind_groups | count | 12 | 899 | 6 |
| preprocess/bind_groups_created | count | 12 | 899 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 291 | 439 | 251 |
| process/cpu_usage | % | 23.81 | 38.94 | 20.10 |
| process/mem_usage | GiB | 5.18 | 5.36 | 5.36 |
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
| started_load_count |  | 4769 | 4861 | 4861 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 4 | 3 |
| statics/material_pipelines | count | 3 | 4 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.68 | 49.86 | 28.98 |
| system/mem_usage | % | 75.50 | 76.10 | 75.98 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 149 | 417 | 117 |
| views/all/bins | count | 6385 | 8221 | 5700 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.0 k | 25.0 k | 13.3 k |
| views/all/pipelines | count | 201 | 363 | 100 |
| views/all/pipelines_distinct | count | 86 | 100 | 81 |
| views/all/work_items | count | 17.2 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 65 | 84 | 49 |
| views/cascade/bins | count | 2600 | 3180 | 2220 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8138 | 9806 | 5671 |
| views/cascade/pipelines | count | 52 | 67 | 42 |
| views/cascade/work_items | count | 8138 | 9806 | 5671 |
| views/lamp_face/batch_sets | count | 0 | 259 | 0 |
| views/lamp_face/bins | count | 1056 | 2475 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1186 | 2851 | 0 |
| views/lamp_face/pipelines | count | 86 | 249 | 0 |
| views/lamp_face/work_items | count | 0 | 2604 | 0 |
| views/main/batch_sets | count | 80 | 94 | 68 |
| views/main/bins | count | 2898 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8348 | 13.0 k | 7592 |
| views/main/pipelines | count | 58 | 64 | 58 |
| views/main/work_items | count | 8518 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 63.73 | 291 | 310.8 k |
| anim/events/count | count | 0.11 | 3 | 535 |
| assets/Image/added | count | 0.12 | 43 | 594 |
| assets/Image/modified | count | 0.01 | 9 | 25 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.84 | 152 | 4075 |
| assets/Mesh/removed | count | 1.00 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 466 |
| assets/StandardMaterial/modified | count | 4.46 | 22 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.98 | 1492 | 4778 |
| physics/inserts | count | 0.52 | 111 | 2537 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.48 | 161 | 2335 |
| physics/steps | count | 0.81 | 2 | 3940 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 15 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 20 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 17 | 378 |
| stream/upload/bytes | bytes | 6943 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1522 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 32.4 ms; slowest 19, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.60 | 2596.2 | main/busy/ms 2570, surface/advance_cpu_ms 66.29, surface/cell_upload_ms 0.87, stream/drive/ms 0.32, weather/map/ms 0.19, bigspace/total/ms 0.09, stream/tick/ms 0.09, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.27 | 674.6 | main/busy/ms 2605, render/thread/ms 637, render/busy/ms 518, render/acquire/ms 117, surface/advance_cpu_ms 66.29, render/graph/ms 5.19, render/present/ms 2.00, stream/drive/ms 1.00, surface/cell_upload_ms 0.87, terrain/update/ms 0.73, terrain/select/ms 0.54, stream/tick/ms 0.49 |
| warm-up | 3.35 | 78.8 | pipelines/wait_ms 934, render/thread/ms 647, render/busy/ms 527, render/acquire/ms 117, pipelines/compile_ms 87.28, pipelines/front_ms 79.81, main/busy/ms 52.21, terrain/update/ms 18.33, render/graph/ms 8.26, pipelines/driver_ms 7.47, render/present/ms 3.12, stream/drive/ms 1.15 |
| warm-up | 3.60 | 90.5 | pipelines/compile_ms 221, pipelines/front_ms 207, main/busy/ms 65.60, render/thread/ms 56.19, render/busy/ms 54.50, pipelines/driver_ms 13.99, render/graph/ms 5.08, stream/drive/ms 2.09, stream/tick/ms 1.67, render/present/ms 1.46, terrain/update/ms 0.87, terrain/select/ms 0.66 |
| warm-up | 3.72 | 49.0 | main/busy/ms 37.09, render/thread/ms 35.78, render/busy/ms 34.44, stream/drive/ms 24.57, render/graph/ms 4.36, render/present/ms 1.17, terrain/update/ms 0.92, stream/spawn/ms 0.91, stream/tick/ms 0.76, terrain/select/ms 0.74, bigspace/total/ms 0.25, render/acquire/ms 0.18 |
| warm-up | 3.79 | 54.9 | pipelines/wait_ms 4145, pipelines/compile_ms 211, pipelines/front_ms 191, main/busy/ms 46.23, render/thread/ms 38.51, render/busy/ms 36.28, stream/drive/ms 28.87, pipelines/driver_ms 20.43, render/graph/ms 5.10, stream/spawn/ms 2.38, render/present/ms 1.98, stream/tick/ms 1.87 |
| warm-up | 3.84 | 48.6 | pipelines/wait_ms 6332, pipelines/compile_ms 252, pipelines/front_ms 228, render/thread/ms 62.22, render/busy/ms 59.85, main/busy/ms 43.85, pipelines/driver_ms 24.09, stream/drive/ms 16.66, render/graph/ms 6.96, stream/tick/ms 3.06, stream/spawn/ms 2.91, render/present/ms 2.02 |
| warm-up | 3.90 | 63.7 | pipelines/wait_ms 3625, pipelines/compile_ms 103, pipelines/front_ms 96.92, render/thread/ms 62.72, render/busy/ms 61.19, main/busy/ms 38.34, stream/drive/ms 14.20, pipelines/driver_ms 5.64, terrain/update/ms 5.07, render/graph/ms 4.46, stream/tick/ms 2.55, stream/spawn/ms 1.42 |
| warm-up | 3.96 | 57.3 | pipelines/wait_ms 1438, render/thread/ms 69.78, pipelines/compile_ms 61.39, pipelines/front_ms 59.41, render/busy/ms 58.62, main/busy/ms 43.73, stream/drive/ms 27.72, render/present/ms 11.00, terrain/update/ms 4.95, render/graph/ms 3.87, pipelines/driver_ms 1.98, stream/spawn/ms 1.38 |
| warm-up | 4.09 | 40.1 | pipelines/wait_ms 3799, render/thread/ms 50.45, render/busy/ms 48.11, pipelines/compile_ms 34.19, pipelines/front_ms 29.91, main/busy/ms 20.50, render/graph/ms 17.55, stream/drive/ms 4.84, pipelines/driver_ms 4.28, stream/spawn/ms 2.85, render/present/ms 2.12, stream/tick/ms 0.82 |
| warm-up | 4.16 | 40.5 | main/busy/ms 65.42, stream/drive/ms 48.23, render/thread/ms 45.92, render/busy/ms 44.65, render/graph/ms 5.85, stream/spawn/ms 2.08, stream/tick/ms 1.63, terrain/update/ms 1.16, render/present/ms 1.07, terrain/select/ms 1.00, bigspace/total/ms 0.25, render/acquire/ms 0.20 |
| warm-up | 4.22 | 56.4 | pipelines/wait_ms 2944, pipelines/compile_ms 144, pipelines/front_ms 139, main/busy/ms 64.00, stream/drive/ms 47.42, render/thread/ms 44.46, render/busy/ms 43.17, pipelines/driver_ms 4.86, render/graph/ms 3.86, stream/spawn/ms 2.18, render/present/ms 1.10, stream/tick/ms 0.84 |
| warm-up | 4.30 | 71.6 | pipelines/wait_ms 3173, pipelines/compile_ms 102, pipelines/front_ms 98.46, main/busy/ms 74.35, stream/drive/ms 48.52, render/thread/ms 35.47, render/busy/ms 34.16, render/graph/ms 5.10, pipelines/driver_ms 4.01, terrain/update/ms 2.68, stream/spawn/ms 2.01, stream/activate/ms 1.21 |
| warm-up | 4.40 | 81.1 | main/busy/ms 68.48, render/thread/ms 41.80, render/busy/ms 40.34, stream/drive/ms 35.43, render/graph/ms 7.02, stream/activate/ms 1.28, render/present/ms 1.24, physics/insert/ms 0.97, stream/tick/ms 0.86, terrain/update/ms 0.59, stream/spawn/ms 0.57, terrain/select/ms 0.49 |
| warm-up | 4.45 | 37.2 | pipelines/wait_ms 4252, pipelines/compile_ms 144, pipelines/front_ms 133, main/busy/ms 41.01, stream/drive/ms 26.92, render/thread/ms 25.89, render/busy/ms 24.48, pipelines/driver_ms 10.48, render/graph/ms 6.35, stream/spawn/ms 1.27, render/present/ms 1.19, stream/tick/ms 0.87 |
| warm-up | 4.63 | 35.4 | pipelines/wait_ms 3410, pipelines/compile_ms 75.96, pipelines/front_ms 69.37, render/thread/ms 46.22, render/busy/ms 44.86, main/busy/ms 19.52, pipelines/driver_ms 6.60, render/graph/ms 5.46, stream/drive/ms 4.14, stream/spawn/ms 1.99, render/present/ms 1.07, stream/tick/ms 1.01 |
| warm-up | 5.62 | 38.4 | render/thread/ms 49.44, render/busy/ms 47.93, main/busy/ms 43.86, stream/drive/ms 19.16, render/graph/ms 7.19, stream/spawn/ms 2.17, stream/tick/ms 1.30, render/present/ms 1.26, physics/insert/ms 1.17, bigspace/total/ms 1.06, terrain/update/ms 0.67, physics/step/ms 0.63 |
| warm-up | 6.77 | 52.5 | pipelines/wait_ms 8375, main/busy/ms 67.48, render/thread/ms 39.29, render/busy/ms 37.39, pipelines/compile_ms 32.84, pipelines/front_ms 30.88, render/graph/ms 13.10, pipelines/driver_ms 1.96, stream/drive/ms 1.65, render/present/ms 1.56, anim/evaluate_cpu/ms 1.43, bigspace/total/ms 1.38 |
| measure | 59.06 | 32.8 | render/thread/ms 40.24, render/busy/ms 39.04, main/busy/ms 23.61, render/graph/ms 11.45, stream/drive/ms 1.51, bigspace/total/ms 1.05, stream/tick/ms 1.03, render/present/ms 1.01, controller/feed/ms 0.84, terrain/update/ms 0.83, terrain/select/ms 0.73, far/select/ms 0.68 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_pom\balmora_walk@2560x1440_off=parallax-1 --bench-baseline none --off parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 17%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **76%** (min 52%) · 2580 MHz (min 2580) · memory 8001 MHz · 72.9 W · 72 °C (max 78) · VRAM 6293 MiB · throttled 100% of samples
