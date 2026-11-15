# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5799 frames over 65.6 s · **median 10.92 ms (92 fps)** · mean 11.32 · p95 14.36 · p99 17.04 (1% low 59 fps) · worst 37.08 ms · 89 spikes over 1.5× median · 31.1 stutters/min (work 34.7/min)
- warm-up: 339 frames over 8.5 s, worst 2688.3 ms
- hitches (> 27.3 ms): **5** measured, 29 in warm-up
- timed GPU passes: 5.89 ms of a 11.32 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.92 ms, limit 11 ms
- **FAIL** frame_ms: 34 stutters (> 1.5x moving median of 60) in 1.09 min = 31.08/min, limit 1/min
- **FAIL** worst frame 37.08 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.077 | 2.078 | 2.322 | 1.00 | 0.187 |
| main_opaque_pass_3d | 1.290 | 1.290 | 1.725 | 1.00 | 0.064 |
| early prepass | 0.417 | 0.418 | 0.627 | 1.00 | 0.100 |
| atmosphere_luts | 0.317 | 0.317 | 0.380 | 1.00 | 0.004 |
| water_velocity | 0.283 | 0.283 | 0.351 | 1.00 | 0.002 |
| uniform_allocation | 0.216 | 0.216 | 0.229 | 1.00 | 0.043 |
| water_sspr | 0.212 | 0.212 | 0.212 | 1.00 | 0.003 |
| bin_unpacking | 0.170 | 0.170 | 0.188 | 1.00 | 0.030 |
| auxiliary_views | 0.158 | 0.158 | 0.624 | 1.00 | 0.161 |
| upscaling | 0.146 | 0.146 | 0.182 | 1.00 | 0.005 |
| water_fft | 0.134 | 0.134 | 0.138 | 1.00 | 0.003 |
| tonemapping | 0.124 | 0.124 | 0.132 | 1.00 | 0.005 |
| main_transmissive_pass_3d | 0.084 | 0.084 | 0.147 | 1.00 | 0.009 |
| clustering | 0.066 | 0.066 | 0.069 | 1.00 | 0.022 |
| render_sky | 0.054 | 0.054 | 0.054 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.049 | 1.00 | 0.006 |
| water_snapshot | 0.031 | 0.031 | 0.033 | 1.00 | 0.002 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.022 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.011 | 0.011 | 0.022 | 1.00 | 0.080 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.004 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 1.00 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 15.090 | 0.000 | 15443.51 | 87507.9 |
| render/thread/ms | 10.082 | 13.000 | 27.16 | 58462.8 |
| render/busy/ms | 9.524 | 12.377 | 26.35 | 55229.6 |
| main/busy/ms | 8.909 | 11.259 | 26.53 | 51663.7 |
| render/graph/ms | 5.255 | 7.055 | 16.36 | 30473.1 |
| bigspace/total/ms | 0.579 | 1.622 | 3.17 | 3358.1 |
| render/present/ms | 0.462 | 0.580 | 2.24 | 2678.3 |
| terrain/update/ms | 0.311 | 0.410 | 2.11 | 1804.9 |
| pipelines/compile_ms | 0.296 | 0.000 | 152.36 | 1716.5 |
| controller/feed/ms | 0.288 | 0.397 | 0.68 | 1667.5 |
| pipelines/front_ms | 0.285 | 0.000 | 146.68 | 1650.4 |
| terrain/select/ms | 0.276 | 0.362 | 1.14 | 1603.2 |
| stream/drive/ms | 0.253 | 0.365 | 18.05 | 1469.1 |
| bigspace/low_precision/ms | 0.244 | 0.669 | 2.91 | 1413.2 |
| far/select/ms | 0.203 | 0.300 | 0.78 | 1178.0 |
| stream/tick/ms | 0.198 | 0.288 | 0.53 | 1147.0 |
| bigspace/high_precision/ms | 0.171 | 0.563 | 2.64 | 993.3 |
| anim/evaluate_cpu/ms | 0.167 | 0.217 | 0.54 | 969.4 |
| controller/move/ms | 0.143 | 0.322 | 0.80 | 829.4 |
| anim/evaluate/ms | 0.139 | 0.274 | 2.02 | 807.9 |
| render/acquire/ms | 0.096 | 0.129 | 1.97 | 554.9 |
| physics/step/ms | 0.051 | 0.095 | 0.52 | 297.3 |
| physics/insert/ms | 0.044 | 0.413 | 0.97 | 253.3 |
| anim/apply/ms | 0.012 | 0.022 | 0.15 | 68.7 |
| weather/eval/ms | 0.012 | 0.016 | 0.43 | 67.6 |
| pipelines/driver_ms | 0.011 | 0.000 | 5.68 | 66.1 |
| controller/avoid/ms | 0.011 | 0.014 | 0.05 | 61.0 |
| script/update/ms | 0.008 | 0.011 | 0.08 | 44.8 |
| script/input/ms | 0.005 | 0.008 | 0.15 | 31.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.09 | 30.9 |
| script/ui/ms | 0.005 | 0.007 | 0.14 | 28.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 26.5 |
| controller/write/ms | 0.003 | 0.004 | 0.22 | 20.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.72 | 14.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 601 |
| crowd/c0/meshes/count | count | 353 | 646 | 159 |
| crowd/c1/count | count | 145 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5882 | 5595 |
| crowd/c1/meshes/count | count | 1181 | 1254 | 1182 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4266 | 10.0 k | 6179 |
| crowd/joints_uploaded/count | count | 4248 | 6643 | 6179 |
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
| frame/work/ms | ms | 9.33 | 26.53 | 8.93 |
| gpu/timed/ms | ms | 5.91 | 10.18 | 6.11 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 803 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 803 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1290 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3380 | 5158 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1290 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3380 | 5158 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 55 | 286 | 42 |
| phases/shadow/bins | count | 3778 | 5541 | 2226 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9665 | 12.7 k | 5723 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 139 | 316 | 42 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2346 |
| physics/items | count | 199 | 476 | 19 |
| physics/pending | count | 0 | 615 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 193 | 198 | 198 |
| pipelines/waiting | count | 0 | 26 | 0 |
| preprocess/bind_groups | count | 11 | 741 | 6 |
| preprocess/bind_groups_created | count | 11 | 741 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 229 | 328 | 221 |
| process/cpu_usage | % | 25.48 | 39.34 | 28.57 |
| process/mem_usage | GiB | 4.71 | 4.85 | 4.85 |
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
| started_load_count |  | 4751 | 4844 | 4844 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 440 | 535 | 402 |
| statics/materials_bound | count | 440 | 535 | 402 |
| statics/materials_unclassed | count | 6 | 8 | 6 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 38.96 | 74.40 | 54.51 |
| system/mem_usage | % | 63.79 | 68.72 | 68.71 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2806 | 2917 | 2917 |
| textures/images | count | 2582 | 2635 | 2635 |
| views/all/batch_sets | count | 112 | 338 | 102 |
| views/all/bins | count | 6477 | 8230 | 5706 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 191 | 369 | 98 |
| views/all/pipelines_distinct | count | 83 | 95 | 80 |
| views/all/work_items | count | 17.2 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 52 | 62 | 42 |
| views/cascade/bins | count | 2612 | 2987 | 2226 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8231 | 9725 | 5723 |
| views/cascade/pipelines | count | 52 | 62 | 42 |
| views/cascade/work_items | count | 8231 | 9725 | 5723 |
| views/lamp_face/batch_sets | count | 0 | 230 | 0 |
| views/lamp_face/bins | count | 1071 | 2642 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1216 | 2991 | 0 |
| views/lamp_face/pipelines | count | 85 | 254 | 0 |
| views/lamp_face/work_items | count | 0 | 2532 | 0 |
| views/main/batch_sets | count | 58 | 62 | 60 |
| views/main/bins | count | 2880 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8368 | 13.1 k | 7592 |
| views/main/pipelines | count | 54 | 58 | 56 |
| views/main/work_items | count | 8535 | 13.3 k | 7900 |
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
| anim/bones_written/count | count | 52.45 | 242 | 304.2 k |
| anim/events/count | count | 0.10 | 3 | 572 |
| assets/Image/added | count | 0.10 | 37 | 598 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 8 | 46 |
| assets/Mesh/added | count | 0.71 | 152 | 4101 |
| assets/Mesh/removed | count | 0.84 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 471 |
| assets/StandardMaterial/modified | count | 3.76 | 20 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.05 | 1495 | 6112 |
| physics/inserts | count | 0.49 | 138 | 2850 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 16 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 5838 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1280 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.3 ms; slowest 34, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.69 | 2688.3 | main/busy/ms 2664, surface/advance_cpu_ms 67.93, surface/cell_upload_ms 0.93, stream/drive/ms 0.29, script/update/ms 0.12, bigspace/total/ms 0.09, stream/tick/ms 0.07, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, weather/apply/ms 0.01, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.49 | 799.2 | main/busy/ms 2697, render/thread/ms 764, render/busy/ms 601, render/acquire/ms 161, surface/advance_cpu_ms 67.93, render/graph/ms 4.44, render/present/ms 2.04, stream/drive/ms 1.06, surface/cell_upload_ms 0.93, stream/tick/ms 0.55, terrain/update/ms 0.53, terrain/select/ms 0.39 |
| warm-up | 3.61 | 119.6 | render/thread/ms 775, render/busy/ms 611, render/acquire/ms 161, pipelines/wait_ms 134, pipelines/compile_ms 121, pipelines/front_ms 114, main/busy/ms 49.24, terrain/update/ms 17.93, render/graph/ms 8.12, pipelines/driver_ms 6.84, render/present/ms 2.89, stream/drive/ms 1.33 |
| warm-up | 3.85 | 86.0 | pipelines/compile_ms 199, pipelines/front_ms 182, main/busy/ms 64.25, render/thread/ms 47.41, render/busy/ms 44.94, pipelines/driver_ms 17.62, render/graph/ms 6.07, render/present/ms 2.25, stream/drive/ms 2.23, stream/tick/ms 1.49, pipelines/wait_ms 1.26, terrain/update/ms 0.71 |
| warm-up | 4.00 | 50.8 | pipelines/compile_ms 212, pipelines/front_ms 203, pipelines/wait_ms 183, render/thread/ms 44.67, render/busy/ms 43.17, main/busy/ms 41.60, stream/drive/ms 30.04, pipelines/driver_ms 9.41, render/graph/ms 3.07, render/present/ms 1.22, stream/tick/ms 0.84, terrain/update/ms 0.68 |
| warm-up | 4.10 | 66.8 | pipelines/wait_ms 793, pipelines/compile_ms 62.21, render/thread/ms 58.97, pipelines/front_ms 57.04, render/busy/ms 56.56, main/busy/ms 41.84, stream/drive/ms 26.42, render/graph/ms 7.63, pipelines/driver_ms 5.17, render/present/ms 2.16, stream/tick/ms 1.34, terrain/update/ms 0.81 |
| warm-up | 4.14 | 32.2 | pipelines/wait_ms 1327, pipelines/compile_ms 112, pipelines/front_ms 105, render/thread/ms 62.84, render/busy/ms 60.34, main/busy/ms 29.08, stream/drive/ms 11.98, render/graph/ms 8.11, pipelines/driver_ms 7.27, render/present/ms 2.23, stream/spawn/ms 1.59, stream/tick/ms 0.88 |
| warm-up | 4.19 | 52.9 | pipelines/wait_ms 4236, pipelines/compile_ms 145, pipelines/front_ms 136, render/thread/ms 52.83, render/busy/ms 50.88, main/busy/ms 50.50, stream/drive/ms 30.91, pipelines/driver_ms 9.32, render/graph/ms 5.48, stream/spawn/ms 3.12, render/present/ms 1.73, stream/tick/ms 0.81 |
| warm-up | 4.25 | 63.9 | pipelines/wait_ms 3316, render/thread/ms 81.18, render/busy/ms 59.58, main/busy/ms 56.17, pipelines/compile_ms 53.51, pipelines/front_ms 50.41, stream/drive/ms 30.31, render/present/ms 21.36, render/graph/ms 3.83, pipelines/driver_ms 3.10, stream/spawn/ms 2.56, terrain/update/ms 0.88 |
| warm-up | 4.29 | 39.5 | pipelines/wait_ms 1180, pipelines/compile_ms 89.48, render/thread/ms 79.27, pipelines/front_ms 64.16, render/busy/ms 57.75, main/busy/ms 31.47, pipelines/driver_ms 25.32, render/present/ms 21.17, stream/drive/ms 4.40, render/graph/ms 3.92, stream/spawn/ms 1.57, terrain/update/ms 1.08 |
| warm-up | 4.32 | 30.3 | pipelines/wait_ms 1180, pipelines/compile_ms 89.48, pipelines/front_ms 64.16, render/thread/ms 55.18, render/busy/ms 53.23, pipelines/driver_ms 25.32, main/busy/ms 21.71, render/graph/ms 7.99, stream/drive/ms 4.14, render/present/ms 1.65, terrain/update/ms 1.20, stream/activate/ms 1.13 |
| warm-up | 4.37 | 44.0 | render/thread/ms 57.32, render/busy/ms 55.01, main/busy/ms 33.56, stream/drive/ms 17.95, render/graph/ms 12.75, render/present/ms 2.06, stream/spawn/ms 1.65, terrain/update/ms 1.11, stream/activate/ms 1.07, stream/tick/ms 0.98, terrain/select/ms 0.93, physics/insert/ms 0.39 |
| warm-up | 4.40 | 35.7 | pipelines/wait_ms 3967, pipelines/compile_ms 162, pipelines/front_ms 154, main/busy/ms 69.85, render/thread/ms 60.73, render/busy/ms 58.33, stream/drive/ms 55.24, render/graph/ms 12.92, pipelines/driver_ms 8.11, render/present/ms 2.02, stream/tick/ms 1.89, stream/spawn/ms 1.81 |
| warm-up | 4.46 | 61.7 | pipelines/wait_ms 6868, pipelines/compile_ms 207, pipelines/front_ms 196, main/busy/ms 57.40, render/thread/ms 53.08, render/busy/ms 51.41, stream/drive/ms 40.74, pipelines/driver_ms 11.12, render/graph/ms 6.78, stream/tick/ms 1.72, render/present/ms 1.38, stream/spawn/ms 1.18 |
| warm-up | 4.53 | 51.9 | pipelines/wait_ms 1690, main/busy/ms 61.32, pipelines/compile_ms 48.02, pipelines/front_ms 46.48, stream/drive/ms 46.27, render/thread/ms 21.78, render/busy/ms 20.37, render/graph/ms 2.84, stream/spawn/ms 2.84, pipelines/driver_ms 1.54, render/present/ms 1.23, terrain/update/ms 0.82 |
| warm-up | 4.55 | 28.0 | pipelines/wait_ms 1690, main/busy/ms 57.22, pipelines/compile_ms 48.02, pipelines/front_ms 46.48, stream/drive/ms 38.08, render/thread/ms 32.14, render/busy/ms 30.50, render/graph/ms 3.79, stream/spawn/ms 2.77, pipelines/driver_ms 1.54, render/present/ms 1.27, stream/tick/ms 0.79 |
| warm-up | 4.60 | 43.3 | main/busy/ms 54.22, render/thread/ms 36.70, render/busy/ms 35.36, stream/drive/ms 33.29, render/graph/ms 3.79, stream/spawn/ms 2.31, bigspace/total/ms 1.29, bigspace/low_precision/ms 1.02, render/present/ms 0.91, stream/tick/ms 0.85, terrain/update/ms 0.60, stream/activate/ms 0.53 |
| warm-up | 4.65 | 32.9 | pipelines/wait_ms 6500, pipelines/compile_ms 143, pipelines/front_ms 137, main/busy/ms 39.01, render/thread/ms 25.58, render/busy/ms 24.23, stream/drive/ms 24.09, pipelines/driver_ms 6.00, render/graph/ms 3.98, stream/activate/ms 1.05, render/present/ms 1.00, stream/tick/ms 0.71 |
| warm-up | 5.33 | 30.9 | pipelines/wait_ms 5200, render/thread/ms 42.98, render/busy/ms 41.20, main/busy/ms 26.55, pipelines/compile_ms 26.10, pipelines/front_ms 24.24, render/graph/ms 7.05, stream/drive/ms 5.23, stream/spawn/ms 2.42, pipelines/driver_ms 1.86, render/present/ms 1.43, stream/tick/ms 0.87 |
| warm-up | 5.40 | 44.8 | render/thread/ms 54.13, render/busy/ms 52.84, main/busy/ms 43.82, stream/drive/ms 5.65, render/graph/ms 4.69, stream/spawn/ms 2.49, stream/tick/ms 1.61, physics/insert/ms 1.19, render/present/ms 1.01, terrain/update/ms 0.80, terrain/select/ms 0.69, bigspace/total/ms 0.60 |
| warm-up | 5.48 | 49.1 | pipelines/wait_ms 623, pipelines/compile_ms 143, pipelines/front_ms 138, render/thread/ms 59.08, render/busy/ms 57.79, main/busy/ms 35.64, render/graph/ms 5.04, pipelines/driver_ms 4.65, stream/drive/ms 3.53, stream/spawn/ms 1.92, physics/insert/ms 1.14, render/present/ms 1.01 |
| warm-up | 5.52 | 33.2 | pipelines/wait_ms 4972, pipelines/compile_ms 83.29, pipelines/front_ms 77.96, render/thread/ms 43.75, render/busy/ms 42.52, main/busy/ms 19.26, render/graph/ms 5.51, pipelines/driver_ms 5.33, stream/drive/ms 3.88, stream/spawn/ms 1.84, render/present/ms 0.93, terrain/update/ms 0.74 |
| warm-up | 5.56 | 30.4 | render/thread/ms 35.04, render/busy/ms 33.83, main/busy/ms 26.71, stream/drive/ms 9.13, render/graph/ms 5.33, stream/spawn/ms 1.50, render/present/ms 0.96, stream/tick/ms 0.81, terrain/update/ms 0.65, terrain/select/ms 0.56, bigspace/total/ms 0.52, render/acquire/ms 0.25 |
| warm-up | 5.72 | 46.8 | pipelines/wait_ms 2028, main/busy/ms 57.99, render/thread/ms 46.33, render/busy/ms 44.95, pipelines/compile_ms 15.48, pipelines/front_ms 13.47, render/graph/ms 4.84, stream/drive/ms 4.29, stream/spawn/ms 2.16, pipelines/driver_ms 2.01, stream/tick/ms 1.15, physics/insert/ms 1.11 |
| warm-up | 5.77 | 48.8 | main/busy/ms 67.43, render/thread/ms 62.23, render/busy/ms 60.65, render/graph/ms 5.96, stream/drive/ms 4.42, stream/spawn/ms 2.54, physics/insert/ms 1.21, render/present/ms 1.20, bigspace/total/ms 0.81, anim/evaluate_cpu/ms 0.79, stream/tick/ms 0.76, terrain/update/ms 0.65 |
| warm-up | 5.82 | 34.3 | pipelines/wait_ms 3613, pipelines/compile_ms 146, pipelines/front_ms 142, main/busy/ms 37.51, render/thread/ms 33.30, render/busy/ms 32.00, stream/drive/ms 16.83, render/graph/ms 4.85, pipelines/driver_ms 3.82, stream/spawn/ms 2.04, stream/tick/ms 1.10, physics/insert/ms 1.10 |
| warm-up | 7.54 | 39.3 | main/busy/ms 47.29, render/thread/ms 36.15, render/busy/ms 34.45, render/graph/ms 10.98, render/present/ms 1.42, stream/drive/ms 1.32, bigspace/total/ms 0.98, anim/evaluate_cpu/ms 0.85, terrain/update/ms 0.74, terrain/select/ms 0.65, stream/tick/ms 0.57, physics/insert/ms 0.54 |
| warm-up | 7.58 | 32.3 | main/busy/ms 50.51, render/thread/ms 45.85, render/busy/ms 44.39, render/graph/ms 7.86, stream/drive/ms 1.36, render/present/ms 1.17, bigspace/total/ms 1.01, anim/evaluate_cpu/ms 0.94, stream/tick/ms 0.68, terrain/update/ms 0.55, physics/insert/ms 0.48, terrain/select/ms 0.48 |
| warm-up | 7.60 | 27.4 | render/thread/ms 40.02, render/busy/ms 38.62, main/busy/ms 36.79, render/graph/ms 7.51, stream/drive/ms 1.40, render/present/ms 1.14, bigspace/total/ms 0.79, stream/tick/ms 0.78, terrain/update/ms 0.59, anim/evaluate_cpu/ms 0.52, terrain/select/ms 0.51, physics/insert/ms 0.50 |
| measure | 0.73 | 27.4 | main/busy/ms 34.08, render/thread/ms 28.39, render/busy/ms 27.07, stream/drive/ms 13.98, render/graph/ms 11.05, physics/insert/ms 1.05, render/present/ms 1.03, bigspace/total/ms 0.73, terrain/update/ms 0.57, stream/tick/ms 0.55, terrain/select/ms 0.51, far/select/ms 0.44 |
| measure | 2.17 | 28.5 | render/thread/ms 37.36, render/busy/ms 35.95, render/graph/ms 21.99, main/busy/ms 18.86, pipelines/compile_ms 13.91, pipelines/front_ms 13.55, render/present/ms 1.23, bigspace/total/ms 1.01, terrain/update/ms 0.62, terrain/select/ms 0.55, controller/feed/ms 0.50, stream/drive/ms 0.44 |
| measure | 18.19 | 29.4 | main/busy/ms 37.00, render/thread/ms 25.89, render/busy/ms 24.64, stream/drive/ms 18.29, render/graph/ms 13.25, render/present/ms 1.05, bigspace/total/ms 0.84, controller/feed/ms 0.82, terrain/update/ms 0.59, terrain/select/ms 0.53, far/select/ms 0.39, stream/tick/ms 0.36 |
| measure | 59.05 | 33.4 | render/thread/ms 35.27, render/busy/ms 33.65, main/busy/ms 28.23, render/graph/ms 12.54, render/present/ms 1.37, bigspace/total/ms 1.21, controller/feed/ms 1.02, stream/drive/ms 0.88, terrain/update/ms 0.87, terrain/select/ms 0.79, stream/tick/ms 0.77, far/select/ms 0.73 |
| measure | 59.09 | 37.1 | render/thread/ms 38.90, render/busy/ms 37.67, main/busy/ms 21.07, render/graph/ms 8.85, bigspace/total/ms 1.03, render/present/ms 1.03, terrain/update/ms 0.80, controller/feed/ms 0.74, terrain/select/ms 0.71, stream/drive/ms 0.69, stream/tick/ms 0.62, far/select/ms 0.60 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_post\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface-3 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 13%
- during the run: other processes 13% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **69%** (min 46%) · 2580 MHz (min 2580) · memory 8001 MHz · 69.3 W · 73 °C (max 79) · VRAM 6013 MiB · throttled 100% of samples
