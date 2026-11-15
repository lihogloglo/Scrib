# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4992 frames over 65.6 s · **median 12.73 ms (79 fps)** · mean 13.15 · p95 16.40 · p99 18.54 (1% low 54 fps) · worst 31.69 ms · 38 spikes over 1.5× median · 19.2 stutters/min (work 15.5/min)
- warm-up: 428 frames over 9.4 s, worst 2731.5 ms
- hitches (> 31.8 ms): **0** measured, 17 in warm-up
- timed GPU passes: 7.84 ms of a 13.15 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.73 ms, limit 11 ms
- **FAIL** frame_ms: 21 stutters (> 1.5x moving median of 60) in 1.09 min = 19.19/min, limit 1/min
- pass worst frame 31.69 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.192 | 2.215 | 2.763 | 0.99 | 0.180 |
| main_opaque_pass_3d | 1.606 | 1.623 | 2.264 | 0.99 | 0.067 |
| early prepass | 0.544 | 0.549 | 0.904 | 0.99 | 0.113 |
| bloom | 0.338 | 0.341 | 0.492 | 0.99 | 0.066 |
| atmosphere_luts | 0.332 | 0.335 | 0.623 | 0.99 | 0.003 |
| water_velocity | 0.300 | 0.303 | 0.487 | 0.99 | 0.002 |
| uniform_allocation | 0.291 | 0.294 | 0.315 | 0.99 | 0.052 |
| bin_unpacking | 0.222 | 0.224 | 0.250 | 0.99 | 0.035 |
| water_sspr | 0.222 | 0.224 | 0.222 | 0.99 | 0.002 |
| ssao | 0.205 | 0.207 | 0.211 | 0.99 | 0.008 |
| auxiliary_views | 0.201 | 0.203 | 0.756 | 0.99 | 0.170 |
| atmos_froxel_inject | 0.157 | 0.159 | 0.153 | 0.99 | 0.002 |
| clouds_trace | 0.139 | 0.141 | 0.229 | 0.99 | 0.001 |
| water_fft | 0.138 | 0.139 | 0.137 | 0.99 | 0.003 |
| upscaling | 0.127 | 0.128 | 0.141 | 0.99 | 0.004 |
| clouds_composite | 0.111 | 0.112 | 0.111 | 0.99 | 0.002 |
| tonemapping | 0.091 | 0.092 | 0.092 | 0.99 | 0.004 |
| main_transmissive_pass_3d | 0.088 | 0.089 | 0.168 | 0.99 | 0.008 |
| clustering | 0.068 | 0.069 | 0.069 | 0.99 | 0.020 |
| render_sky | 0.054 | 0.055 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.443 | 0.758 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 0.99 | 0.001 |
| surface_trail | 0.047 | 0.058 | 0.057 | 0.81 | 0.002 |
| early_mesh_preprocessing | 0.044 | 0.045 | 0.049 | 0.99 | 0.005 |
| water_snapshot | 0.035 | 0.035 | 0.045 | 0.99 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.244 | 0.561 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.030 | 0.99 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 0.99 | 0.007 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.032 | 0.99 | 0.108 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.004 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.012 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 19.787 | 0.000 | 11303.10 | 98776.7 |
| render/thread/ms | 11.943 | 15.058 | 30.65 | 59617.4 |
| render/busy/ms | 11.368 | 14.437 | 29.90 | 56750.6 |
| main/busy/ms | 9.270 | 12.085 | 29.08 | 46276.3 |
| render/graph/ms | 6.886 | 8.928 | 16.70 | 34374.3 |
| render/present/ms | 0.482 | 0.576 | 16.04 | 2407.8 |
| bigspace/total/ms | 0.473 | 0.785 | 6.69 | 2359.4 |
| pipelines/compile_ms | 0.443 | 0.000 | 163.74 | 2209.4 |
| pipelines/front_ms | 0.427 | 0.000 | 157.35 | 2132.1 |
| terrain/update/ms | 0.291 | 0.370 | 1.78 | 1454.7 |
| controller/feed/ms | 0.277 | 0.377 | 1.24 | 1381.5 |
| terrain/select/ms | 0.258 | 0.328 | 0.65 | 1290.4 |
| stream/drive/ms | 0.251 | 0.344 | 20.93 | 1252.3 |
| far/select/ms | 0.195 | 0.271 | 0.82 | 971.8 |
| stream/tick/ms | 0.191 | 0.265 | 0.64 | 954.6 |
| bigspace/low_precision/ms | 0.188 | 0.323 | 3.89 | 938.9 |
| anim/evaluate/ms | 0.186 | 0.338 | 3.21 | 927.9 |
| anim/evaluate_cpu/ms | 0.144 | 0.184 | 0.49 | 717.3 |
| controller/move/ms | 0.135 | 0.239 | 0.67 | 674.3 |
| bigspace/high_precision/ms | 0.116 | 0.222 | 6.08 | 579.3 |
| render/acquire/ms | 0.092 | 0.123 | 0.44 | 459.0 |
| physics/step/ms | 0.055 | 0.086 | 0.58 | 276.6 |
| physics/insert/ms | 0.053 | 0.439 | 0.89 | 266.9 |
| weather/map/ms | 0.024 | 0.033 | 0.30 | 121.7 |
| pipelines/driver_ms | 0.015 | 0.000 | 6.39 | 77.4 |
| anim/apply/ms | 0.011 | 0.020 | 0.28 | 55.4 |
| controller/avoid/ms | 0.010 | 0.014 | 0.35 | 51.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.09 | 51.8 |
| script/update/ms | 0.007 | 0.010 | 0.16 | 35.4 |
| stream/despawn/ms | 0.006 | 0.000 | 1.16 | 30.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.05 | 24.8 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 24.4 |
| script/ui/ms | 0.005 | 0.006 | 0.11 | 23.5 |
| controller/write/ms | 0.004 | 0.005 | 0.18 | 17.9 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.70 | 14.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.06 | 3.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.5 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 21 | 0 |
| anim/a2/count | count | 0 | 11 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 46 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 50 | 13 |
| crowd/c0/joints/count | count | 1281 | 2262 | 583 |
| crowd/c0/meshes/count | count | 353 | 646 | 162 |
| crowd/c1/count | count | 146 | 156 | 148 |
| crowd/c1/joints/count | count | 5531 | 5992 | 5613 |
| crowd/c1/meshes/count | count | 1181 | 1308 | 1179 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6402 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4270 | 8439 | 6179 |
| crowd/joints_uploaded/count | count | 4253 | 6643 | 6179 |
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
| frame/work/ms | ms | 11.04 | 29.90 | 11.22 |
| gpu/timed/ms | ms | 8.57 | 12.79 | 8.64 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 800 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 800 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 33 | 39 | 29 |
| phases/opaque3d/bins | count | 1291 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3372 | 5158 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 23 | 26 | 23 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 33 | 39 | 29 |
| phases/opaque3d_prepass/bins | count | 1291 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3372 | 5158 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 23 | 26 | 23 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 62 | 325 | 48 |
| phases/shadow/bins | count | 3789 | 5446 | 2220 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9379 | 12.4 k | 5671 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 141 | 320 | 42 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 702 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 214 | 218 | 218 |
| pipelines/waiting | count | 0 | 26 | 0 |
| preprocess/bind_groups | count | 12 | 887 | 6 |
| preprocess/bind_groups_created | count | 12 | 887 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 288 | 434 | 257 |
| process/cpu_usage | % | 24.04 | 49.52 | 26.13 |
| process/mem_usage | GiB | 5.15 | 5.33 | 5.33 |
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
| started_load_count |  | 4767 | 4858 | 4858 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 4 | 3 |
| statics/material_pipelines | count | 3 | 4 | 3 |
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
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.89 | 51.06 | 29.15 |
| system/mem_usage | % | 75.43 | 75.98 | 75.87 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2591 | 2644 | 2644 |
| views/all/batch_sets | count | 146 | 411 | 120 |
| views/all/bins | count | 6358 | 8364 | 5700 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 17.9 k | 25.4 k | 13.3 k |
| views/all/pipelines | count | 197 | 381 | 100 |
| views/all/pipelines_distinct | count | 89 | 100 | 81 |
| views/all/work_items | count | 17.2 k | 24.1 k | 13.6 k |
| views/cascade/batch_sets | count | 59 | 73 | 48 |
| views/cascade/bins | count | 2584 | 3173 | 2220 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8134 | 10.1 k | 5671 |
| views/cascade/pipelines | count | 51 | 64 | 42 |
| views/cascade/work_items | count | 8134 | 10.1 k | 5671 |
| views/lamp_face/batch_sets | count | 0 | 259 | 0 |
| views/lamp_face/bins | count | 1076 | 2609 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1195 | 2902 | 0 |
| views/lamp_face/pipelines | count | 88 | 264 | 0 |
| views/lamp_face/work_items | count | 0 | 2741 | 0 |
| views/main/batch_sets | count | 82 | 94 | 72 |
| views/main/bins | count | 2877 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8342 | 13.1 k | 7592 |
| views/main/pipelines | count | 58 | 64 | 58 |
| views/main/work_items | count | 8512 | 13.3 k | 7900 |
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
| anim/bones_written/count | count | 50.64 | 756 | 252.8 k |
| anim/events/count | count | 0.11 | 3 | 569 |
| assets/Image/added | count | 0.12 | 43 | 611 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.83 | 168 | 4168 |
| assets/Mesh/removed | count | 0.98 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 488 |
| assets/StandardMaterial/modified | count | 4.35 | 16 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.90 | 1883 | 9488 |
| physics/inserts | count | 0.58 | 111 | 2889 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.47 | 145 | 2335 |
| physics/steps | count | 0.79 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 15 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 22 | 378 |
| stream/upload/bytes | bytes | 6781 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1487 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.8 ms; slowest 17, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.73 | 2731.5 | main/busy/ms 2704, surface/advance_cpu_ms 69.10, surface/cell_upload_ms 0.78, stream/drive/ms 0.31, weather/map/ms 0.13, bigspace/total/ms 0.12, stream/tick/ms 0.09, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 3.63 | 894.9 | main/busy/ms 2742, render/thread/ms 853, render/busy/ms 733, render/acquire/ms 118, surface/advance_cpu_ms 69.10, render/graph/ms 4.96, render/present/ms 2.13, stream/drive/ms 1.15, surface/cell_upload_ms 0.78, stream/tick/ms 0.63, terrain/update/ms 0.56, terrain/select/ms 0.36 |
| warm-up | 3.70 | 75.8 | render/thread/ms 866, render/busy/ms 744, pipelines/wait_ms 200, render/acquire/ms 119, pipelines/compile_ms 79.60, pipelines/front_ms 73.06, main/busy/ms 54.74, terrain/update/ms 21.64, render/graph/ms 8.81, pipelines/driver_ms 6.54, render/present/ms 3.51, stream/drive/ms 1.29 |
| warm-up | 3.93 | 84.7 | pipelines/compile_ms 223, pipelines/front_ms 214, main/busy/ms 60.07, render/thread/ms 48.50, render/busy/ms 46.56, pipelines/driver_ms 8.18, render/graph/ms 4.60, render/present/ms 1.69, stream/drive/ms 1.63, stream/tick/ms 1.23, terrain/update/ms 0.71, terrain/select/ms 0.60 |
| warm-up | 4.07 | 50.7 | main/busy/ms 39.13, render/thread/ms 37.23, render/busy/ms 35.98, stream/drive/ms 27.00, render/graph/ms 3.23, render/present/ms 0.99, stream/tick/ms 0.91, stream/spawn/ms 0.87, terrain/update/ms 0.70, terrain/select/ms 0.58, render/acquire/ms 0.26, bigspace/total/ms 0.24 |
| warm-up | 4.14 | 52.2 | pipelines/wait_ms 257, pipelines/compile_ms 160, pipelines/front_ms 150, render/thread/ms 44.52, main/busy/ms 43.33, render/busy/ms 42.44, stream/drive/ms 29.29, pipelines/driver_ms 10.67, render/graph/ms 5.49, render/present/ms 1.79, stream/spawn/ms 1.17, stream/tick/ms 0.86 |
| warm-up | 4.22 | 68.8 | pipelines/wait_ms 1871, pipelines/compile_ms 120, pipelines/front_ms 111, render/thread/ms 55.42, render/busy/ms 52.58, main/busy/ms 32.31, stream/drive/ms 12.29, pipelines/driver_ms 9.11, render/graph/ms 6.40, render/present/ms 2.51, stream/tick/ms 2.22, stream/spawn/ms 1.74 |
| warm-up | 4.27 | 48.5 | pipelines/wait_ms 4986, pipelines/compile_ms 161, pipelines/front_ms 149, render/thread/ms 79.52, render/busy/ms 77.33, main/busy/ms 59.51, stream/drive/ms 35.66, pipelines/driver_ms 12.01, render/graph/ms 6.30, stream/tick/ms 2.51, render/present/ms 1.93, stream/spawn/ms 1.01 |
| warm-up | 4.37 | 99.8 | pipelines/wait_ms 4815, render/thread/ms 99.26, render/busy/ms 98.14, pipelines/compile_ms 87.19, pipelines/front_ms 82.42, main/busy/ms 54.02, stream/drive/ms 33.99, pipelines/driver_ms 4.78, terrain/update/ms 4.67, render/graph/ms 3.74, stream/tick/ms 2.04, render/present/ms 0.94 |
| warm-up | 4.57 | 83.4 | pipelines/wait_ms 2826, main/busy/ms 61.04, render/thread/ms 55.64, render/busy/ms 53.67, pipelines/compile_ms 52.10, stream/drive/ms 50.11, pipelines/front_ms 47.31, render/graph/ms 10.83, pipelines/driver_ms 4.79, render/present/ms 1.73, stream/tick/ms 1.47, terrain/update/ms 0.66 |
| warm-up | 4.60 | 33.7 | pipelines/wait_ms 1556, render/thread/ms 64.56, render/busy/ms 62.82, main/busy/ms 56.23, stream/drive/ms 43.10, pipelines/compile_ms 19.56, pipelines/front_ms 17.88, render/graph/ms 15.47, pipelines/driver_ms 1.68, render/present/ms 1.52, stream/spawn/ms 1.46, stream/tick/ms 0.89 |
| warm-up | 4.64 | 38.8 | pipelines/wait_ms 4202, pipelines/compile_ms 143, pipelines/front_ms 136, main/busy/ms 59.74, render/thread/ms 46.49, render/busy/ms 44.88, stream/drive/ms 44.33, render/graph/ms 13.81, pipelines/driver_ms 7.19, stream/spawn/ms 2.59, render/present/ms 1.34, stream/tick/ms 0.95 |
| warm-up | 4.72 | 61.0 | pipelines/wait_ms 3503, main/busy/ms 57.59, render/thread/ms 31.34, stream/drive/ms 31.27, render/busy/ms 30.08, pipelines/compile_ms 21.17, pipelines/front_ms 19.80, render/graph/ms 5.10, terrain/update/ms 2.61, stream/spawn/ms 2.50, pipelines/driver_ms 1.37, render/present/ms 1.06 |
| warm-up | 4.80 | 58.1 | pipelines/wait_ms 2826, pipelines/compile_ms 130, pipelines/front_ms 123, main/busy/ms 63.24, render/thread/ms 34.38, render/busy/ms 33.20, stream/drive/ms 29.28, pipelines/driver_ms 7.52, render/graph/ms 3.77, terrain/update/ms 2.34, physics/insert/ms 1.05, stream/activate/ms 1.03 |
| warm-up | 5.54 | 39.4 | pipelines/wait_ms 4536, pipelines/compile_ms 141, pipelines/front_ms 133, render/thread/ms 46.70, render/busy/ms 45.50, main/busy/ms 26.11, pipelines/driver_ms 7.96, stream/drive/ms 4.76, render/graph/ms 4.30, stream/spawn/ms 2.32, stream/tick/ms 1.35, render/present/ms 0.98 |
| warm-up | 5.89 | 35.1 | pipelines/wait_ms 7670, pipelines/compile_ms 187, pipelines/front_ms 179, main/busy/ms 32.94, render/thread/ms 28.84, render/busy/ms 27.60, stream/drive/ms 15.02, pipelines/driver_ms 8.07, render/graph/ms 3.81, stream/spawn/ms 2.20, render/present/ms 1.04, stream/tick/ms 0.96 |
| warm-up | 6.06 | 35.6 | render/thread/ms 36.06, main/busy/ms 35.79, render/busy/ms 34.31, stream/drive/ms 9.92, render/graph/ms 7.56, render/present/ms 1.49, stream/spawn/ms 1.24, stream/tick/ms 0.97, terrain/update/ms 0.58, bigspace/total/ms 0.52, terrain/select/ms 0.50, render/acquire/ms 0.25 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_pom\balmora_walk@2560x1440_off=parallax-3 --bench-baseline none --off parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **78%** (min 55%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.6 W · 75 °C (max 80) · VRAM 6283 MiB · throttled 100% of samples
