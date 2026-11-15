# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,clouds,fog,froxels,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1614 frames over 15.0 s · **median 9.18 ms (109 fps)** · mean 9.29 · p95 10.68 · p99 11.72 (1% low 85 fps) · worst 16.31 ms · 3 spikes over 1.5× median · 12.0 stutters/min (work 8.0/min)
- warm-up: 303 frames over 7.7 s, worst 2544.5 ms
- hitches (> 25.0 ms): **0** measured, 31 in warm-up
- timed GPU passes: 7.28 ms of a 9.29 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.18 ms, limit 11 ms
- pass worst frame 16.31 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.077 | 2.136 | 2.186 | 0.97 | 0.053 |
| dlss_super_resolution | 2.002 | 2.059 | 2.078 | 0.97 | 0.184 |
| early prepass | 0.783 | 0.805 | 1.105 | 0.97 | 0.098 |
| atmosphere_luts | 0.340 | 0.349 | 0.469 | 0.97 | 0.004 |
| bloom | 0.313 | 0.322 | 0.328 | 0.97 | 0.067 |
| water_velocity | 0.236 | 0.242 | 0.265 | 0.97 | 0.002 |
| main_transmissive_pass_3d | 0.227 | 0.234 | 0.249 | 0.97 | 0.008 |
| water_sspr | 0.216 | 0.222 | 0.225 | 0.97 | 0.002 |
| ssao | 0.188 | 0.193 | 0.199 | 0.97 | 0.008 |
| uniform_allocation | 0.170 | 0.175 | 0.186 | 0.97 | 0.034 |
| water_fft | 0.132 | 0.136 | 0.138 | 0.97 | 0.003 |
| bin_unpacking | 0.130 | 0.133 | 0.144 | 0.97 | 0.024 |
| upscaling | 0.112 | 0.115 | 0.128 | 0.97 | 0.004 |
| tonemapping | 0.085 | 0.088 | 0.092 | 0.97 | 0.004 |
| clustering | 0.075 | 0.077 | 0.079 | 0.97 | 0.023 |
| render_sky | 0.053 | 0.054 | 0.055 | 0.97 | 0.002 |
| early_mesh_preprocessing | 0.039 | 0.040 | 0.042 | 0.97 | 0.006 |
| water_snapshot | 0.033 | 0.034 | 0.036 | 0.97 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.022 | 0.025 | 0.97 | 0.007 |
| main_transparent_pass_3d | 0.019 | 0.020 | 0.020 | 0.97 | 0.056 |
| sparse buffer updates | 0.007 | 0.008 | 0.008 | 0.97 | 0.003 |
| night_sky | 0.003 | 0.003 | 0.004 | 0.97 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.97 | 0.004 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 0.97 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 67.559 | 0.000 | 7431.03 | 109039.7 |
| render/thread/ms | 8.391 | 9.660 | 13.92 | 13543.5 |
| render/busy/ms | 7.378 | 8.880 | 13.29 | 11908.1 |
| main/busy/ms | 7.192 | 8.494 | 15.07 | 11607.1 |
| render/graph/ms | 4.239 | 5.401 | 10.31 | 6841.1 |
| pipelines/compile_ms | 1.000 | 0.000 | 122.94 | 1614.6 |
| pipelines/front_ms | 0.967 | 0.000 | 119.34 | 1560.9 |
| render/acquire/ms | 0.526 | 1.394 | 2.24 | 848.3 |
| render/present/ms | 0.488 | 0.629 | 1.84 | 787.1 |
| bigspace/total/ms | 0.436 | 1.266 | 2.17 | 703.3 |
| terrain/update/ms | 0.240 | 0.313 | 0.50 | 388.1 |
| stream/drive/ms | 0.210 | 0.317 | 0.61 | 339.2 |
| terrain/select/ms | 0.209 | 0.271 | 0.45 | 337.8 |
| controller/move/ms | 0.208 | 0.345 | 0.58 | 335.2 |
| far/select/ms | 0.184 | 0.273 | 0.56 | 296.2 |
| bigspace/high_precision/ms | 0.182 | 0.781 | 1.90 | 293.8 |
| stream/tick/ms | 0.180 | 0.278 | 0.58 | 289.9 |
| controller/feed/ms | 0.177 | 0.234 | 0.60 | 286.1 |
| bigspace/low_precision/ms | 0.122 | 0.480 | 1.67 | 197.5 |
| anim/evaluate/ms | 0.095 | 0.218 | 1.83 | 152.8 |
| anim/evaluate_cpu/ms | 0.061 | 0.076 | 0.27 | 97.7 |
| pipelines/driver_ms | 0.033 | 0.000 | 3.79 | 53.7 |
| physics/step/ms | 0.030 | 0.064 | 0.57 | 48.2 |
| weather/eval/ms | 0.013 | 0.018 | 0.08 | 20.3 |
| script/update/ms | 0.008 | 0.014 | 0.09 | 12.5 |
| controller/avoid/ms | 0.007 | 0.009 | 0.14 | 11.1 |
| script/ui/ms | 0.005 | 0.007 | 0.07 | 8.8 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 8.4 |
| physics/insert/ms | 0.005 | 0.022 | 0.61 | 7.9 |
| anim/apply/ms | 0.004 | 0.006 | 0.04 | 7.0 |
| controller/write/ms | 0.002 | 0.003 | 0.03 | 3.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 15 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 36 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 23 | 8 |
| crowd/c0/joints/count | count | 272 | 782 | 272 |
| crowd/c0/meshes/count | count | 48 | 138 | 48 |
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4948 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1145 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 6052 | 4212 |
| crowd/joints_uploaded/count | count | 4212 | 4212 | 4212 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.0 k | 47.0 k | 47.0 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.09 M | 189.09 M | 189.09 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 7.66 | 15.07 | 7.59 |
| gpu/timed/ms | ms | 7.57 | 8.74 | 7.59 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 478.47 M | 478.47 M | 478.47 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 22 |
| phases/opaque3d/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 22 |
| phases/opaque3d_prepass/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 33 | 38 | 33 |
| phases/shadow/bins | count | 2040 | 2347 | 2040 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5404 | 5730 | 5404 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 33 | 38 | 33 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 303 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 172 | 172 | 172 |
| pipelines/waiting | count | 0 | 33 | 0 |
| preprocess/bind_groups | count | 0 | 100 | 0 |
| preprocess/bind_groups_created | count | 0 | 100 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 205 | 205 | 205 |
| process/cpu_usage | % | 31.49 | 42.38 | 23.08 |
| process/mem_usage | GiB | 4.21 | 4.24 | 4.22 |
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
| started_load_count |  | 3532 | 3534 | 3534 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 392 | 392 | 392 |
| statics/materials_bound | count | 392 | 392 | 392 |
| statics/materials_unclassed | count | 6 | 6 | 6 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.03 | 0.03 | 0.03 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 41.22 | 61.84 | 50.63 |
| system/mem_usage | % | 61.84 | 62.04 | 61.86 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2463 | 2463 | 2463 |
| textures/images | count | 1910 | 1910 | 1910 |
| views/all/batch_sets | count | 91 | 96 | 91 |
| views/all/bins | count | 5210 | 5463 | 5210 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.7 k | 14.9 k | 14.7 k |
| views/all/pipelines | count | 87 | 92 | 87 |
| views/all/pipelines_distinct | count | 77 | 81 | 77 |
| views/all/work_items | count | 14.9 k | 15.1 k | 14.9 k |
| views/cascade/batch_sets | count | 33 | 38 | 33 |
| views/cascade/bins | count | 2040 | 2347 | 2040 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5404 | 5730 | 5404 |
| views/cascade/pipelines | count | 33 | 38 | 33 |
| views/cascade/work_items | count | 5404 | 5730 | 5404 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 58 | 62 | 58 |
| views/main/bins | count | 3170 | 3172 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 54 | 58 | 54 |
| views/main/work_items | count | 9462 | 9462 | 9462 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.6 k | 24.6 k | 24.6 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0.25 | 297 | 396 |
| anim/events/count | count | 0.09 | 2 | 145 |
| assets/Image/added | count | 0.01 | 13 | 24 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.07 | 47 | 118 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.02 | 16 | 28 |
| assets/StandardMaterial/modified | count | 3.07 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.53 | 1656 | 4084 |
| physics/inserts | count | 0.20 | 118 | 319 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.56 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0 | 0 | 0 |
| stream/deactivate/items | count | 0 | 0 | 0 |
| stream/despawn/items | count | 0 | 0 | 0 |
| stream/fetch/items | count | 0 | 0 | 0 |
| stream/spawn/items | count | 0 | 0 | 0 |
| stream/upload/bytes | bytes | 0 | 0 | 0 |
| stream/upload/items | count | 0 | 0 | 0 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 2 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 31, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2544.5 | main/busy/ms 2519, surface/advance_cpu_ms 84.68, surface/cell_upload_ms 0.72, stream/drive/ms 0.29, bigspace/total/ms 0.09, stream/tick/ms 0.08, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.01, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.52 | 972.7 | main/busy/ms 2556, render/thread/ms 937, render/busy/ms 781, render/acquire/ms 153, surface/advance_cpu_ms 84.68, render/graph/ms 4.68, render/present/ms 2.67, stream/drive/ms 1.30, stream/tick/ms 0.79, surface/cell_upload_ms 0.72, terrain/update/ms 0.43, bigspace/total/ms 0.26 |
| warm-up | 3.85 | 89.9 | main/busy/ms 60.06, render/thread/ms 44.96, render/busy/ms 43.57, render/graph/ms 3.02, render/present/ms 1.17, stream/drive/ms 0.92, stream/tick/ms 0.76, terrain/update/ms 0.66, terrain/select/ms 0.53, bigspace/total/ms 0.26, render/acquire/ms 0.22, bigspace/high_precision/ms 0.13 |
| warm-up | 3.97 | 62.2 | pipelines/wait_ms 995, pipelines/compile_ms 225, pipelines/front_ms 214, main/busy/ms 43.62, render/thread/ms 40.53, render/busy/ms 39.21, stream/drive/ms 31.61, pipelines/driver_ms 10.97, render/graph/ms 3.84, stream/tick/ms 1.17, render/present/ms 1.12, terrain/update/ms 0.50 |
| warm-up | 4.03 | 36.0 | pipelines/wait_ms 1713, pipelines/compile_ms 56.76, pipelines/front_ms 51.25, render/thread/ms 45.36, render/busy/ms 43.19, main/busy/ms 38.45, stream/drive/ms 20.51, render/graph/ms 6.24, pipelines/driver_ms 5.51, render/present/ms 1.87, stream/spawn/ms 1.36, stream/tick/ms 0.92 |
| warm-up | 4.06 | 35.1 | pipelines/wait_ms 1894, pipelines/compile_ms 77.29, pipelines/front_ms 72.46, render/thread/ms 52.48, render/busy/ms 49.80, main/busy/ms 41.04, stream/drive/ms 19.55, render/graph/ms 6.58, pipelines/driver_ms 4.83, render/present/ms 2.32, stream/tick/ms 1.83, stream/spawn/ms 1.01 |
| warm-up | 4.13 | 70.1 | pipelines/wait_ms 1451, render/thread/ms 69.70, render/busy/ms 67.54, pipelines/compile_ms 65.61, pipelines/front_ms 61.55, main/busy/ms 44.09, stream/drive/ms 20.24, render/graph/ms 5.56, pipelines/driver_ms 4.06, stream/tick/ms 2.20, render/present/ms 1.86, stream/spawn/ms 1.22 |
| warm-up | 4.17 | 38.3 | main/busy/ms 76.26, render/thread/ms 68.58, render/busy/ms 67.20, stream/drive/ms 52.10, render/graph/ms 3.94, stream/spawn/ms 2.36, stream/tick/ms 1.35, render/present/ms 1.05, terrain/update/ms 1.00, terrain/select/ms 0.79, bigspace/total/ms 0.47, render/acquire/ms 0.33 |
| warm-up | 4.29 | 119.1 | pipelines/wait_ms 2843, pipelines/compile_ms 186, pipelines/front_ms 175, render/thread/ms 83.10, render/busy/ms 81.57, main/busy/ms 76.36, stream/drive/ms 51.79, pipelines/driver_ms 10.93, render/graph/ms 4.49, stream/spawn/ms 3.03, render/present/ms 1.23, terrain/update/ms 1.09 |
| warm-up | 4.32 | 32.9 | pipelines/wait_ms 2843, pipelines/compile_ms 186, pipelines/front_ms 175, render/thread/ms 79.01, render/busy/ms 77.60, main/busy/ms 33.22, stream/drive/ms 11.38, pipelines/driver_ms 10.93, render/graph/ms 4.23, stream/spawn/ms 2.90, render/present/ms 1.17, terrain/update/ms 1.09 |
| warm-up | 4.40 | 73.4 | pipelines/wait_ms 829, render/thread/ms 78.29, render/busy/ms 49.40, main/busy/ms 31.07, render/present/ms 28.62, pipelines/compile_ms 26.59, pipelines/driver_ms 24.83, stream/drive/ms 11.01, render/graph/ms 3.54, stream/spawn/ms 2.14, pipelines/front_ms 1.76, stream/activate/ms 1.05 |
| warm-up | 4.45 | 37.3 | pipelines/wait_ms 2086, pipelines/compile_ms 228, pipelines/front_ms 195, main/busy/ms 42.81, pipelines/driver_ms 33.45, stream/drive/ms 29.01, render/thread/ms 22.86, render/busy/ms 21.56, render/graph/ms 6.25, render/present/ms 1.10, stream/spawn/ms 0.78, stream/tick/ms 0.60 |
| warm-up | 4.56 | 59.7 | pipelines/wait_ms 4426, pipelines/compile_ms 88.30, pipelines/front_ms 83.81, main/busy/ms 66.24, stream/drive/ms 54.36, render/thread/ms 25.56, render/busy/ms 24.25, pipelines/driver_ms 4.49, render/graph/ms 2.90, render/present/ms 1.09, stream/tick/ms 1.00, terrain/update/ms 0.58 |
| warm-up | 4.63 | 56.4 | pipelines/wait_ms 1059, main/busy/ms 56.33, stream/drive/ms 35.79, render/thread/ms 23.84, render/busy/ms 22.51, pipelines/compile_ms 9.18, pipelines/front_ms 8.26, render/graph/ms 4.38, stream/spawn/ms 2.40, render/present/ms 1.10, pipelines/driver_ms 0.92, terrain/update/ms 0.66 |
| warm-up | 4.79 | 31.6 | render/thread/ms 38.64, render/busy/ms 36.70, main/busy/ms 26.68, render/graph/ms 4.81, stream/drive/ms 4.75, stream/spawn/ms 2.03, render/present/ms 1.51, stream/tick/ms 1.25, terrain/update/ms 0.63, terrain/select/ms 0.49, render/acquire/ms 0.43, bigspace/total/ms 0.41 |
| warm-up | 4.81 | 25.1 | render/thread/ms 42.65, render/busy/ms 40.57, main/busy/ms 26.18, stream/drive/ms 5.77, render/graph/ms 5.15, stream/spawn/ms 4.00, render/present/ms 1.61, terrain/update/ms 0.99, stream/tick/ms 0.95, terrain/select/ms 0.65, bigspace/total/ms 0.49, render/acquire/ms 0.47 |
| warm-up | 5.04 | 25.3 | pipelines/wait_ms 2564, render/thread/ms 45.84, render/busy/ms 44.45, main/busy/ms 23.25, pipelines/compile_ms 15.54, pipelines/front_ms 14.45, stream/drive/ms 9.18, render/graph/ms 5.35, stream/spawn/ms 2.15, render/present/ms 1.14, pipelines/driver_ms 1.09, stream/tick/ms 0.60 |
| warm-up | 5.11 | 34.2 | render/thread/ms 43.58, render/busy/ms 41.94, main/busy/ms 28.19, render/graph/ms 6.78, stream/drive/ms 5.17, stream/spawn/ms 2.11, stream/tick/ms 1.53, render/present/ms 1.36, bigspace/total/ms 1.25, terrain/update/ms 0.80, bigspace/high_precision/ms 0.78, terrain/select/ms 0.64 |
| warm-up | 5.14 | 27.9 | pipelines/wait_ms 670, pipelines/compile_ms 141, pipelines/front_ms 137, render/thread/ms 48.62, render/busy/ms 47.20, main/busy/ms 26.12, stream/drive/ms 5.79, render/graph/ms 4.82, pipelines/driver_ms 4.06, stream/spawn/ms 2.14, stream/tick/ms 1.48, bigspace/total/ms 1.30 |
| warm-up | 5.16 | 25.5 | pipelines/wait_ms 670, pipelines/compile_ms 141, pipelines/front_ms 137, render/thread/ms 41.45, render/busy/ms 39.93, main/busy/ms 27.86, stream/drive/ms 5.54, render/graph/ms 4.44, pipelines/driver_ms 4.06, stream/spawn/ms 2.16, stream/tick/ms 1.42, render/present/ms 1.28 |
| warm-up | 5.19 | 29.0 | pipelines/wait_ms 7750, render/thread/ms 45.92, render/busy/ms 44.43, pipelines/compile_ms 36.33, pipelines/front_ms 34.28, main/busy/ms 28.94, stream/drive/ms 5.20, render/graph/ms 4.08, stream/spawn/ms 2.15, pipelines/driver_ms 2.05, stream/tick/ms 1.41, render/present/ms 1.27 |
| warm-up | 5.29 | 75.4 | pipelines/wait_ms 8926, pipelines/compile_ms 108, pipelines/front_ms 102, main/busy/ms 63.91, render/thread/ms 60.05, render/busy/ms 58.02, render/graph/ms 6.65, pipelines/driver_ms 6.30, stream/drive/ms 6.29, stream/spawn/ms 3.04, render/present/ms 1.77, physics/insert/ms 1.22 |
| warm-up | 5.31 | 25.0 | pipelines/wait_ms 12.4 k, pipelines/compile_ms 141, pipelines/front_ms 133, render/thread/ms 63.88, render/busy/ms 61.87, main/busy/ms 51.26, pipelines/driver_ms 8.40, render/graph/ms 6.72, stream/drive/ms 5.02, terrain/update/ms 4.12, stream/spawn/ms 3.44, render/present/ms 1.69 |
| warm-up | 5.35 | 36.4 | pipelines/wait_ms 3461, main/busy/ms 61.23, render/thread/ms 41.61, render/busy/ms 39.23, pipelines/compile_ms 33.22, pipelines/front_ms 31.13, stream/drive/ms 16.35, render/graph/ms 7.69, terrain/update/ms 4.25, stream/spawn/ms 2.74, pipelines/driver_ms 2.10, render/present/ms 2.09 |
| warm-up | 5.41 | 60.7 | main/busy/ms 52.53, render/thread/ms 46.64, render/busy/ms 44.90, stream/drive/ms 15.98, render/graph/ms 6.39, stream/spawn/ms 2.40, render/present/ms 1.55, physics/insert/ms 0.96, bigspace/total/ms 0.79, terrain/update/ms 0.79, stream/tick/ms 0.76, terrain/select/ms 0.63 |
| warm-up | 5.44 | 32.5 | render/thread/ms 48.56, render/busy/ms 47.14, main/busy/ms 43.15, render/graph/ms 4.43, stream/drive/ms 3.63, stream/spawn/ms 2.36, render/present/ms 1.08, stream/tick/ms 0.72, terrain/update/ms 0.61, bigspace/total/ms 0.58, terrain/select/ms 0.51, physics/insert/ms 0.45 |
| warm-up | 5.48 | 37.2 | pipelines/wait_ms 11.2 k, pipelines/compile_ms 183, pipelines/front_ms 176, main/busy/ms 49.45, render/thread/ms 45.73, render/busy/ms 44.28, stream/drive/ms 8.13, pipelines/driver_ms 7.07, render/graph/ms 4.40, stream/spawn/ms 2.21, render/present/ms 1.08, stream/tick/ms 0.74 |
| warm-up | 5.58 | 27.9 | main/busy/ms 33.04, render/thread/ms 28.81, render/busy/ms 27.17, stream/drive/ms 16.48, render/graph/ms 6.69, render/present/ms 1.30, stream/tick/ms 0.61, terrain/update/ms 0.57, bigspace/total/ms 0.57, stream/spawn/ms 0.50, terrain/select/ms 0.48, render/acquire/ms 0.34 |
| warm-up | 5.71 | 29.4 | pipelines/wait_ms 3726, render/thread/ms 39.33, render/busy/ms 38.04, main/busy/ms 23.48, pipelines/compile_ms 14.86, pipelines/front_ms 13.94, render/graph/ms 6.66, stream/drive/ms 4.10, stream/spawn/ms 2.09, render/present/ms 1.08, stream/tick/ms 1.04, pipelines/driver_ms 0.92 |
| warm-up | 5.85 | 29.0 | pipelines/wait_ms 3301, pipelines/compile_ms 133, pipelines/front_ms 128, main/busy/ms 41.36, render/thread/ms 36.84, render/busy/ms 34.66, stream/drive/ms 12.48, render/graph/ms 11.23, pipelines/driver_ms 5.72, stream/spawn/ms 2.37, render/present/ms 1.80, stream/tick/ms 1.16 |
| warm-up | 5.90 | 48.6 | render/thread/ms 51.00, render/busy/ms 49.31, main/busy/ms 46.74, stream/drive/ms 13.51, render/graph/ms 7.75, stream/spawn/ms 3.44, render/present/ms 1.35, stream/tick/ms 1.18, physics/insert/ms 1.06, bigspace/total/ms 0.91, terrain/update/ms 0.80, terrain/select/ms 0.66 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_envmap\balmora@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap-1 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 8%
- during the run: other processes 10% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **96%** (min 53%) · 2580 MHz (min 2580) · memory 8001 MHz · 94.7 W · 78 °C (max 81) · VRAM 5418 MiB · throttled 100% of samples
