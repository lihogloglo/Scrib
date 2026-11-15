# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: clouds,fog,froxels,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1536 frames over 15.0 s · **median 9.66 ms (104 fps)** · mean 9.77 · p95 11.09 · p99 12.08 (1% low 83 fps) · worst 25.00 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 295 frames over 8.1 s, worst 2667.8 ms
- hitches (> 25.0 ms): **1** measured, 36 in warm-up
- timed GPU passes: 7.75 ms of a 9.77 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.66 ms, limit 11 ms
- pass worst frame 25.00 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.447 | 2.536 | 2.963 | 0.96 | 0.056 |
| dlss_super_resolution | 2.111 | 2.188 | 2.556 | 0.96 | 0.193 |
| early prepass | 0.576 | 0.597 | 0.932 | 0.96 | 0.096 |
| bloom | 0.330 | 0.342 | 0.641 | 0.96 | 0.067 |
| atmosphere_luts | 0.319 | 0.331 | 0.477 | 0.96 | 0.004 |
| main_transmissive_pass_3d | 0.273 | 0.283 | 0.311 | 0.96 | 0.009 |
| water_velocity | 0.251 | 0.260 | 0.426 | 0.96 | 0.002 |
| water_sspr | 0.220 | 0.228 | 0.344 | 0.96 | 0.002 |
| ssao | 0.199 | 0.207 | 0.213 | 0.96 | 0.008 |
| uniform_allocation | 0.177 | 0.184 | 0.187 | 0.96 | 0.033 |
| bin_unpacking | 0.135 | 0.140 | 0.145 | 0.96 | 0.024 |
| water_fft | 0.135 | 0.140 | 0.139 | 0.96 | 0.003 |
| upscaling | 0.124 | 0.128 | 0.132 | 0.96 | 0.005 |
| tonemapping | 0.090 | 0.093 | 0.096 | 0.96 | 0.004 |
| clustering | 0.072 | 0.074 | 0.080 | 0.96 | 0.023 |
| render_sky | 0.053 | 0.055 | 0.056 | 0.96 | 0.002 |
| lightprobe_irradiance_map | 0.053 | 0.439 | 0.754 | 0.12 | 0.001 |
| early_mesh_preprocessing | 0.038 | 0.040 | 0.041 | 0.96 | 0.006 |
| lightprobe_radiance_map | 0.028 | 0.236 | 0.266 | 0.12 | 0.005 |
| water_snapshot | 0.027 | 0.028 | 0.049 | 0.96 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.96 | 0.002 |
| main_transparent_pass_3d | 0.021 | 0.022 | 0.025 | 0.96 | 0.057 |
| per_view_shadow_pass | 0.021 | 0.022 | 0.028 | 0.96 | 0.007 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.96 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.96 | 0.003 |
| night_sky | 0.003 | 0.003 | 0.006 | 0.96 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.96 | 0.003 |
| lightprobe_copy | 0.002 | 0.014 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 103.107 | 0.000 | 10032.16 | 158372.9 |
| render/thread/ms | 8.864 | 10.173 | 23.94 | 13615.5 |
| render/busy/ms | 7.731 | 9.078 | 11.33 | 11874.1 |
| main/busy/ms | 7.172 | 8.331 | 10.97 | 11015.5 |
| render/graph/ms | 4.582 | 5.702 | 7.21 | 7038.6 |
| pipelines/compile_ms | 1.505 | 0.000 | 147.16 | 2312.4 |
| pipelines/front_ms | 1.455 | 0.000 | 143.87 | 2234.7 |
| render/acquire/ms | 0.604 | 1.774 | 2.86 | 927.7 |
| render/present/ms | 0.530 | 0.729 | 15.97 | 813.7 |
| bigspace/total/ms | 0.376 | 1.008 | 2.28 | 577.4 |
| terrain/update/ms | 0.237 | 0.312 | 0.62 | 364.1 |
| stream/drive/ms | 0.208 | 0.296 | 0.53 | 319.4 |
| terrain/select/ms | 0.206 | 0.270 | 0.58 | 316.8 |
| stream/tick/ms | 0.179 | 0.262 | 0.48 | 275.0 |
| far/select/ms | 0.176 | 0.257 | 0.45 | 269.9 |
| controller/feed/ms | 0.167 | 0.220 | 0.56 | 256.6 |
| bigspace/high_precision/ms | 0.144 | 0.391 | 1.85 | 221.4 |
| controller/move/ms | 0.113 | 0.172 | 0.41 | 173.0 |
| bigspace/low_precision/ms | 0.098 | 0.231 | 1.91 | 151.2 |
| anim/evaluate/ms | 0.079 | 0.171 | 1.79 | 121.0 |
| anim/evaluate_cpu/ms | 0.065 | 0.080 | 0.37 | 100.4 |
| pipelines/driver_ms | 0.051 | 0.000 | 5.44 | 77.6 |
| physics/step/ms | 0.028 | 0.058 | 0.47 | 43.5 |
| weather/eval/ms | 0.013 | 0.016 | 0.07 | 19.2 |
| physics/insert/ms | 0.012 | 0.064 | 0.55 | 18.2 |
| script/update/ms | 0.008 | 0.012 | 0.21 | 12.3 |
| controller/avoid/ms | 0.006 | 0.008 | 0.14 | 9.1 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 7.9 |
| script/ui/ms | 0.005 | 0.007 | 0.02 | 7.1 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 6.9 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.11 | 1.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.6 |
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
| anim/a2/count | count | 0 | 7 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 18 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 15 | 8 |
| crowd/c0/joints/count | count | 272 | 510 | 272 |
| crowd/c0/meshes/count | count | 48 | 90 | 48 |
| crowd/c1/count | count | 122 | 123 | 122 |
| crowd/c1/joints/count | count | 4848 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1089 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 5805 | 4212 |
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
| frame/work/ms | ms | 8.66 | 11.33 | 8.61 |
| gpu/timed/ms | ms | 8.63 | 10.67 | 8.61 |
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
| phases/shadow/items | count | 5404 | 5724 | 5404 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 33 | 38 | 33 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 368 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 174 | 174 | 174 |
| pipelines/waiting | count | 0 | 37 | 0 |
| preprocess/bind_groups | count | 0 | 206 | 0 |
| preprocess/bind_groups_created | count | 0 | 206 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 205 | 205 | 205 |
| process/cpu_usage | % | 26.44 | 37.98 | 22.60 |
| process/mem_usage | GiB | 4.16 | 4.30 | 4.30 |
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
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 39.20 | 59.43 | 49.61 |
| system/mem_usage | % | 61.08 | 61.74 | 61.74 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2465 | 2465 | 2465 |
| textures/images | count | 1913 | 1913 | 1913 |
| views/all/batch_sets | count | 91 | 98 | 91 |
| views/all/bins | count | 5210 | 5517 | 5210 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.7 k | 15.0 k | 14.7 k |
| views/all/pipelines | count | 87 | 94 | 87 |
| views/all/pipelines_distinct | count | 77 | 81 | 77 |
| views/all/work_items | count | 14.9 k | 15.2 k | 14.9 k |
| views/cascade/batch_sets | count | 33 | 38 | 33 |
| views/cascade/bins | count | 2040 | 2347 | 2040 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5404 | 5724 | 5404 |
| views/cascade/pipelines | count | 33 | 38 | 33 |
| views/cascade/work_items | count | 5404 | 5724 | 5404 |
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
| anim/bones_written/count | count | 0.43 | 231 | 660 |
| anim/events/count | count | 0.09 | 2 | 136 |
| assets/Image/added | count | 0.00 | 1 | 2 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.02 | 14 | 27 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.00 | 1 | 2 |
| assets/StandardMaterial/modified | count | 3.23 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.87 | 730 | 2875 |
| physics/inserts | count | 0.24 | 140 | 369 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.59 | 2 | 902 |
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

## Hitches (> 25.0 ms; slowest 37, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.67 | 2667.8 | main/busy/ms 2641, surface/advance_cpu_ms 72.07, surface/cell_upload_ms 0.69, stream/drive/ms 0.41, stream/tick/ms 0.10, bigspace/total/ms 0.10, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.53 | 866.7 | main/busy/ms 2683, render/thread/ms 820, render/busy/ms 712, render/acquire/ms 105, surface/advance_cpu_ms 72.07, render/graph/ms 6.01, render/present/ms 2.65, stream/drive/ms 1.07, terrain/update/ms 0.71, surface/cell_upload_ms 0.69, stream/tick/ms 0.54, terrain/select/ms 0.49 |
| warm-up | 3.81 | 112.8 | pipelines/compile_ms 236, pipelines/front_ms 214, main/busy/ms 78.36, render/thread/ms 57.09, render/busy/ms 55.03, pipelines/driver_ms 22.07, render/graph/ms 6.02, stream/drive/ms 2.22, stream/tick/ms 1.88, render/present/ms 1.80, terrain/update/ms 0.79, terrain/select/ms 0.61 |
| warm-up | 3.85 | 26.2 | pipelines/wait_ms 214, pipelines/compile_ms 57.36, pipelines/front_ms 52.49, render/thread/ms 36.05, render/busy/ms 34.68, main/busy/ms 20.93, pipelines/driver_ms 4.87, stream/drive/ms 2.80, render/graph/ms 2.77, stream/tick/ms 1.39, render/present/ms 1.18, terrain/update/ms 0.75 |
| warm-up | 3.93 | 37.3 | main/busy/ms 41.39, render/thread/ms 39.55, render/busy/ms 37.88, stream/drive/ms 28.62, render/graph/ms 5.57, render/present/ms 1.37, stream/tick/ms 0.84, terrain/update/ms 0.60, terrain/select/ms 0.47, render/acquire/ms 0.30, bigspace/total/ms 0.25, bigspace/high_precision/ms 0.10 |
| warm-up | 4.01 | 32.2 | render/thread/ms 42.36, render/busy/ms 40.65, main/busy/ms 33.04, stream/drive/ms 19.17, render/graph/ms 5.15, render/present/ms 1.44, stream/spawn/ms 1.25, stream/tick/ms 0.89, terrain/update/ms 0.62, terrain/select/ms 0.50, render/acquire/ms 0.28, bigspace/total/ms 0.22 |
| warm-up | 4.12 | 90.7 | pipelines/wait_ms 2726, pipelines/compile_ms 273, pipelines/front_ms 256, render/thread/ms 74.84, render/busy/ms 72.81, main/busy/ms 40.34, stream/drive/ms 20.21, pipelines/driver_ms 16.92, render/graph/ms 5.55, render/present/ms 1.76, stream/spawn/ms 1.09, stream/tick/ms 0.96 |
| warm-up | 4.16 | 37.1 | pipelines/wait_ms 2726, pipelines/compile_ms 273, pipelines/front_ms 256, render/thread/ms 87.55, render/busy/ms 85.76, main/busy/ms 62.82, stream/drive/ms 44.36, pipelines/driver_ms 16.92, render/graph/ms 6.88, stream/spawn/ms 1.85, render/present/ms 1.47, stream/tick/ms 0.90 |
| warm-up | 4.29 | 128.6 | pipelines/wait_ms 4995, pipelines/compile_ms 157, pipelines/front_ms 147, render/thread/ms 98.28, render/busy/ms 96.85, main/busy/ms 76.02, stream/drive/ms 58.76, pipelines/driver_ms 9.70, render/graph/ms 5.21, stream/spawn/ms 2.14, render/present/ms 1.15, stream/tick/ms 0.79 |
| warm-up | 4.33 | 35.9 | main/busy/ms 37.12, render/thread/ms 29.83, render/busy/ms 28.05, stream/drive/ms 11.19, render/graph/ms 3.01, stream/spawn/ms 2.65, render/present/ms 1.35, stream/tick/ms 1.20, terrain/update/ms 0.73, terrain/select/ms 0.57, stream/activate/ms 0.43, render/acquire/ms 0.43 |
| warm-up | 4.36 | 26.6 | main/busy/ms 55.68, render/thread/ms 39.27, render/busy/ms 37.77, stream/drive/ms 31.14, stream/spawn/ms 3.20, render/graph/ms 3.03, stream/tick/ms 1.20, render/present/ms 1.04, stream/activate/ms 0.93, terrain/update/ms 0.71, terrain/select/ms 0.55, render/acquire/ms 0.45 |
| warm-up | 4.40 | 43.9 | pipelines/wait_ms 1454, pipelines/compile_ms 192, pipelines/front_ms 170, main/busy/ms 48.24, render/thread/ms 33.21, render/busy/ms 31.89, stream/drive/ms 29.91, pipelines/driver_ms 22.89, render/graph/ms 3.06, stream/spawn/ms 2.13, render/present/ms 1.11, stream/activate/ms 0.97 |
| warm-up | 4.43 | 25.8 | pipelines/wait_ms 1454, pipelines/compile_ms 192, pipelines/front_ms 170, render/thread/ms 34.05, render/busy/ms 32.71, pipelines/driver_ms 22.89, main/busy/ms 19.83, stream/drive/ms 3.05, render/graph/ms 2.80, render/present/ms 1.06, stream/tick/ms 0.78, stream/activate/ms 0.78 |
| warm-up | 4.53 | 63.6 | pipelines/wait_ms 1527, pipelines/compile_ms 141, pipelines/front_ms 131, main/busy/ms 102, stream/drive/ms 91.23, render/thread/ms 27.05, render/busy/ms 25.66, pipelines/driver_ms 9.68, render/graph/ms 3.49, render/present/ms 1.16, stream/tick/ms 0.98, terrain/update/ms 0.56 |
| warm-up | 4.57 | 47.2 | pipelines/wait_ms 2479, pipelines/compile_ms 192, pipelines/front_ms 180, main/busy/ms 53.26, stream/drive/ms 38.43, render/thread/ms 26.92, render/busy/ms 25.64, pipelines/driver_ms 12.43, render/graph/ms 3.40, stream/spawn/ms 1.11, render/present/ms 0.98, stream/tick/ms 0.79 |
| warm-up | 4.62 | 26.1 | pipelines/wait_ms 4870, pipelines/compile_ms 51.46, pipelines/front_ms 47.71, main/busy/ms 26.54, render/thread/ms 21.57, render/busy/ms 20.41, stream/drive/ms 9.82, pipelines/driver_ms 3.75, render/graph/ms 3.01, stream/spawn/ms 1.73, render/present/ms 0.94, terrain/update/ms 0.64 |
| warm-up | 4.76 | 31.2 | render/thread/ms 35.87, render/busy/ms 33.99, main/busy/ms 28.50, render/graph/ms 6.55, stream/drive/ms 4.55, stream/tick/ms 1.61, render/present/ms 1.60, stream/spawn/ms 1.59, terrain/update/ms 0.76, terrain/select/ms 0.57, bigspace/total/ms 0.46, render/acquire/ms 0.28 |
| warm-up | 4.79 | 31.8 | render/thread/ms 46.02, render/busy/ms 44.14, main/busy/ms 30.01, stream/drive/ms 6.59, render/graph/ms 5.63, stream/spawn/ms 3.12, stream/tick/ms 1.62, render/present/ms 1.49, terrain/update/ms 0.81, terrain/select/ms 0.63, stream/activate/ms 0.56, bigspace/total/ms 0.41 |
| warm-up | 5.02 | 33.4 | pipelines/wait_ms 2634, pipelines/compile_ms 157, pipelines/front_ms 152, render/thread/ms 50.43, render/busy/ms 48.84, main/busy/ms 22.87, pipelines/driver_ms 5.17, render/graph/ms 4.76, stream/drive/ms 4.24, render/present/ms 1.27, stream/tick/ms 0.91, stream/spawn/ms 0.83 |
| warm-up | 5.07 | 28.0 | main/busy/ms 43.88, render/thread/ms 39.22, render/busy/ms 37.03, terrain/update/ms 7.50, render/graph/ms 6.52, stream/drive/ms 4.81, stream/spawn/ms 3.07, render/present/ms 1.84, bigspace/total/ms 1.72, bigspace/high_precision/ms 1.31, physics/insert/ms 0.98, terrain/select/ms 0.81 |
| warm-up | 5.11 | 41.3 | render/thread/ms 54.56, render/busy/ms 52.05, main/busy/ms 42.63, terrain/update/ms 9.76, render/graph/ms 7.12, stream/drive/ms 5.50, stream/spawn/ms 3.06, render/present/ms 2.07, bigspace/total/ms 1.87, bigspace/high_precision/ms 1.44, physics/insert/ms 1.19, stream/tick/ms 1.15 |
| warm-up | 5.17 | 58.9 | pipelines/wait_ms 3203, render/thread/ms 75.86, render/busy/ms 73.30, main/busy/ms 64.76, render/graph/ms 7.63, stream/drive/ms 5.36, pipelines/compile_ms 3.60, terrain/update/ms 3.29, stream/spawn/ms 2.17, render/present/ms 2.04, pipelines/driver_ms 1.81, pipelines/front_ms 1.79 |
| warm-up | 5.25 | 76.9 | pipelines/wait_ms 18.1 k, pipelines/compile_ms 233, pipelines/front_ms 217, render/thread/ms 95.47, render/busy/ms 93.65, main/busy/ms 78.09, pipelines/driver_ms 15.74, render/graph/ms 6.53, stream/drive/ms 5.76, stream/spawn/ms 2.45, render/present/ms 1.40, stream/tick/ms 1.27 |
| warm-up | 5.29 | 35.6 | pipelines/wait_ms 14.9 k, pipelines/compile_ms 229, pipelines/front_ms 215, render/thread/ms 81.72, render/busy/ms 80.30, main/busy/ms 58.98, pipelines/driver_ms 13.93, stream/drive/ms 5.24, render/graph/ms 4.96, stream/spawn/ms 2.39, render/present/ms 1.18, stream/tick/ms 1.03 |
| warm-up | 5.32 | 30.9 | render/thread/ms 51.00, render/busy/ms 49.32, main/busy/ms 37.72, render/graph/ms 4.93, stream/drive/ms 4.05, stream/spawn/ms 2.08, render/present/ms 1.31, stream/tick/ms 0.94, terrain/update/ms 0.72, bigspace/total/ms 0.71, terrain/select/ms 0.62, render/acquire/ms 0.36 |
| warm-up | 5.40 | 41.5 | render/thread/ms 49.06, render/busy/ms 45.46, main/busy/ms 31.30, render/graph/ms 10.52, stream/drive/ms 4.62, render/present/ms 3.06, stream/spawn/ms 2.88, bigspace/total/ms 1.37, stream/tick/ms 1.09, terrain/update/ms 0.77, bigspace/high_precision/ms 0.77, terrain/select/ms 0.60 |
| warm-up | 5.43 | 26.6 | render/thread/ms 47.18, render/busy/ms 44.06, main/busy/ms 41.41, stream/drive/ms 16.91, render/graph/ms 8.97, render/present/ms 2.80, stream/spawn/ms 2.70, stream/tick/ms 1.21, bigspace/total/ms 0.96, terrain/update/ms 0.70, terrain/select/ms 0.55, bigspace/high_precision/ms 0.39 |
| warm-up | 5.47 | 40.0 | pipelines/wait_ms 3511, pipelines/compile_ms 279, pipelines/front_ms 265, render/thread/ms 40.69, main/busy/ms 39.10, render/busy/ms 38.91, stream/drive/ms 16.00, pipelines/driver_ms 14.84, render/graph/ms 5.31, stream/spawn/ms 2.12, render/present/ms 1.56, bigspace/total/ms 1.03 |
| warm-up | 5.61 | 32.0 | main/busy/ms 37.91, render/thread/ms 26.26, render/busy/ms 24.47, stream/drive/ms 20.49, render/graph/ms 6.89, stream/spawn/ms 2.17, render/present/ms 1.41, stream/tick/ms 0.94, bigspace/total/ms 0.64, terrain/update/ms 0.61, terrain/select/ms 0.50, render/acquire/ms 0.38 |
| warm-up | 5.90 | 26.0 | main/busy/ms 42.42, render/thread/ms 36.46, render/busy/ms 34.18, stream/drive/ms 14.75, render/graph/ms 9.58, stream/spawn/ms 2.21, render/present/ms 2.03, stream/tick/ms 1.29, bigspace/total/ms 0.98, terrain/update/ms 0.80, terrain/select/ms 0.64, bigspace/high_precision/ms 0.43 |
| warm-up | 5.94 | 46.0 | pipelines/wait_ms 4630, pipelines/compile_ms 68.83, pipelines/front_ms 67.39, render/thread/ms 45.63, render/busy/ms 43.95, main/busy/ms 43.08, stream/drive/ms 15.40, render/graph/ms 5.70, stream/spawn/ms 2.89, pipelines/driver_ms 1.44, render/present/ms 1.43, stream/tick/ms 1.31 |
| warm-up | 7.07 | 34.1 | pipelines/wait_ms 8714, pipelines/compile_ms 50.59, pipelines/front_ms 48.03, main/busy/ms 37.62, render/thread/ms 35.58, render/busy/ms 33.77, render/graph/ms 10.52, pipelines/driver_ms 2.56, render/present/ms 1.62, stream/drive/ms 1.36, bigspace/total/ms 0.96, stream/tick/ms 0.52 |
| warm-up | 7.26 | 25.0 | pipelines/wait_ms 6234, render/thread/ms 31.52, render/busy/ms 29.86, main/busy/ms 27.77, pipelines/compile_ms 27.62, pipelines/front_ms 25.95, render/graph/ms 11.16, pipelines/driver_ms 1.68, stream/drive/ms 1.64, render/present/ms 1.35, stream/tick/ms 0.73, bigspace/total/ms 0.71 |
| warm-up | 7.97 | 29.5 | pipelines/wait_ms 3516, main/busy/ms 43.89, render/thread/ms 31.80, render/busy/ms 30.20, render/graph/ms 11.35, pipelines/compile_ms 7.27, pipelines/front_ms 6.94, physics/insert/ms 1.50, render/present/ms 1.28, stream/drive/ms 1.06, bigspace/total/ms 0.76, physics/step/ms 0.58 |
| warm-up | 7.99 | 27.8 | main/busy/ms 38.59, render/thread/ms 36.30, render/busy/ms 34.84, render/graph/ms 8.53, terrain/update/ms 4.78, stream/drive/ms 1.23, render/present/ms 1.14, physics/insert/ms 1.11, physics/step/ms 1.01, bigspace/total/ms 0.70, stream/tick/ms 0.58, anim/evaluate_cpu/ms 0.42 |
| warm-up | 8.15 | 35.6 | pipelines/wait_ms 2917, main/busy/ms 36.19, render/thread/ms 31.23, render/busy/ms 29.51, pipelines/compile_ms 28.21, pipelines/front_ms 27.29, render/graph/ms 10.97, render/present/ms 1.53, bigspace/total/ms 1.05, physics/step/ms 0.93, pipelines/driver_ms 0.91, anim/evaluate_cpu/ms 0.60 |
| measure | 11.86 | 25.0 | render/thread/ms 34.56, render/busy/ms 17.82, render/present/ms 16.55, main/busy/ms 14.07, render/graph/ms 11.47, bigspace/total/ms 0.68, terrain/update/ms 0.64, terrain/select/ms 0.56, stream/drive/ms 0.53, stream/tick/ms 0.46, far/select/ms 0.44, controller/feed/ms 0.38 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_fog\balmora@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels-2 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 13% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **95%** (min 63%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.8 W · 76 °C (max 78) · VRAM 5736 MiB · throttled 100% of samples
