# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: clouds,fog,froxels,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1544 frames over 15.0 s · **median 9.66 ms (104 fps)** · mean 9.72 · p95 10.86 · p99 11.75 (1% low 85 fps) · worst 23.97 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 296 frames over 7.5 s, worst 2515.8 ms
- hitches (> 25.0 ms): **0** measured, 32 in warm-up
- timed GPU passes: 8.00 ms of a 9.72 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.66 ms, limit 11 ms
- pass worst frame 23.97 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.561 | 2.596 | 3.004 | 0.99 | 0.057 |
| dlss_super_resolution | 2.148 | 2.177 | 2.458 | 0.99 | 0.187 |
| early prepass | 0.600 | 0.608 | 0.945 | 0.99 | 0.094 |
| atmosphere_luts | 0.346 | 0.351 | 0.479 | 0.99 | 0.003 |
| bloom | 0.338 | 0.342 | 0.677 | 0.99 | 0.064 |
| main_transmissive_pass_3d | 0.290 | 0.294 | 0.305 | 0.99 | 0.009 |
| water_velocity | 0.252 | 0.255 | 0.271 | 0.99 | 0.002 |
| water_sspr | 0.218 | 0.221 | 0.218 | 0.99 | 0.002 |
| ssao | 0.198 | 0.201 | 0.210 | 0.99 | 0.007 |
| uniform_allocation | 0.183 | 0.186 | 0.188 | 0.99 | 0.034 |
| bin_unpacking | 0.140 | 0.142 | 0.146 | 0.99 | 0.025 |
| water_fft | 0.137 | 0.139 | 0.138 | 0.99 | 0.003 |
| upscaling | 0.123 | 0.124 | 0.129 | 0.99 | 0.004 |
| tonemapping | 0.094 | 0.095 | 0.097 | 0.99 | 0.004 |
| clustering | 0.068 | 0.069 | 0.076 | 0.99 | 0.022 |
| lightprobe_irradiance_map | 0.057 | 0.468 | 0.769 | 0.12 | 0.001 |
| render_sky | 0.055 | 0.056 | 0.056 | 0.99 | 0.002 |
| early_mesh_preprocessing | 0.040 | 0.041 | 0.042 | 0.99 | 0.006 |
| lightprobe_radiance_map | 0.032 | 0.259 | 0.574 | 0.12 | 0.004 |
| water_snapshot | 0.028 | 0.029 | 0.046 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.023 | 0.026 | 0.99 | 0.007 |
| main_transparent_pass_3d | 0.021 | 0.021 | 0.024 | 0.99 | 0.056 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.99 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.008 | 0.99 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.015 | 0.014 | 0.12 | 0.001 |
| lightprobe_copy | 0.002 | 0.014 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_second_pass | 0.002 | 0.013 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 86.203 | 0.000 | 9492.05 | 133097.2 |
| render/thread/ms | 8.841 | 9.970 | 23.04 | 13650.3 |
| render/busy/ms | 7.198 | 8.489 | 10.32 | 11114.2 |
| main/busy/ms | 7.019 | 8.096 | 10.11 | 10836.8 |
| render/graph/ms | 4.108 | 5.167 | 7.14 | 6342.3 |
| pipelines/compile_ms | 1.547 | 0.000 | 150.34 | 2388.8 |
| pipelines/front_ms | 1.495 | 0.000 | 144.48 | 2308.7 |
| render/acquire/ms | 1.165 | 2.298 | 3.16 | 1798.3 |
| render/present/ms | 0.478 | 0.574 | 13.81 | 737.8 |
| bigspace/total/ms | 0.350 | 0.727 | 2.18 | 541.0 |
| terrain/update/ms | 0.230 | 0.299 | 0.55 | 355.0 |
| terrain/select/ms | 0.200 | 0.259 | 0.52 | 308.6 |
| stream/drive/ms | 0.191 | 0.278 | 0.68 | 294.4 |
| far/select/ms | 0.165 | 0.245 | 0.64 | 255.3 |
| stream/tick/ms | 0.164 | 0.243 | 0.62 | 252.6 |
| controller/feed/ms | 0.162 | 0.219 | 0.49 | 250.7 |
| bigspace/high_precision/ms | 0.128 | 0.272 | 1.92 | 198.0 |
| controller/move/ms | 0.119 | 0.202 | 0.69 | 183.7 |
| bigspace/low_precision/ms | 0.085 | 0.179 | 1.38 | 130.9 |
| anim/evaluate/ms | 0.080 | 0.196 | 1.05 | 123.2 |
| anim/evaluate_cpu/ms | 0.062 | 0.081 | 0.20 | 95.1 |
| pipelines/driver_ms | 0.052 | 0.000 | 5.86 | 80.1 |
| physics/step/ms | 0.030 | 0.064 | 0.47 | 46.4 |
| weather/eval/ms | 0.011 | 0.016 | 0.17 | 17.7 |
| script/update/ms | 0.007 | 0.010 | 0.20 | 11.4 |
| controller/avoid/ms | 0.006 | 0.007 | 0.19 | 8.7 |
| script/input/ms | 0.005 | 0.007 | 0.08 | 8.0 |
| anim/apply/ms | 0.005 | 0.006 | 0.18 | 7.1 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.2 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 3.0 |
| physics/insert/ms | 0.001 | 0.000 | 0.79 | 2.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.13 | 1.2 |
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
| anim/a2/count | count | 0 | 4 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 9 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 12 | 8 |
| crowd/c0/joints/count | count | 272 | 408 | 272 |
| crowd/c0/meshes/count | count | 48 | 72 | 48 |
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1080 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 6491 | 4212 |
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
| frame/work/ms | ms | 8.78 | 10.32 | 8.46 |
| gpu/timed/ms | ms | 8.75 | 9.70 | 8.46 |
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
| phases/opaque3d/batch_sets | count | 22 | 23 | 22 |
| phases/opaque3d/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 22 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 23 | 22 |
| phases/opaque3d_prepass/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 22 | 21 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 33 | 38 | 33 |
| phases/shadow/bins | count | 2039 | 2347 | 2039 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5403 | 5704 | 5403 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 33 | 38 | 33 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 129 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 174 | 174 | 174 |
| pipelines/waiting | count | 0 | 36 | 0 |
| preprocess/bind_groups | count | 0 | 96 | 0 |
| preprocess/bind_groups_created | count | 0 | 96 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 205 | 205 | 205 |
| process/cpu_usage | % | 24.52 | 42.31 | 18.75 |
| process/mem_usage | GiB | 4.17 | 4.28 | 4.28 |
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
| system/cpu_usage | % | 30.23 | 61.30 | 27.70 |
| system/mem_usage | % | 55.83 | 56.11 | 56.11 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2465 | 2465 | 2465 |
| textures/images | count | 1913 | 1913 | 1913 |
| views/all/batch_sets | count | 91 | 96 | 91 |
| views/all/bins | count | 5209 | 5485 | 5209 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.7 k | 14.8 k | 14.7 k |
| views/all/pipelines | count | 87 | 92 | 87 |
| views/all/pipelines_distinct | count | 77 | 79 | 77 |
| views/all/work_items | count | 14.9 k | 15.0 k | 14.9 k |
| views/cascade/batch_sets | count | 33 | 38 | 33 |
| views/cascade/bins | count | 2039 | 2347 | 2039 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5403 | 5704 | 5403 |
| views/cascade/pipelines | count | 33 | 38 | 33 |
| views/cascade/work_items | count | 5403 | 5704 | 5403 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 58 | 60 | 58 |
| views/main/bins | count | 3170 | 3172 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 54 | 56 | 54 |
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
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 3 | 153 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.21 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.66 | 450 | 1021 |
| physics/inserts | count | 0.08 | 65 | 129 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 2 | 900 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 32, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2515.8 | main/busy/ms 2490, surface/advance_cpu_ms 71.55, surface/cell_upload_ms 0.72, stream/drive/ms 0.44, stream/tick/ms 0.10, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.03, far/select/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.39 | 870.5 | main/busy/ms 2528, render/thread/ms 838, render/busy/ms 724, render/acquire/ms 111, surface/advance_cpu_ms 71.55, render/graph/ms 6.10, render/present/ms 2.21, stream/drive/ms 0.97, surface/cell_upload_ms 0.72, terrain/update/ms 0.51, stream/tick/ms 0.45, terrain/select/ms 0.30 |
| warm-up | 3.63 | 84.4 | pipelines/wait_ms 375, pipelines/compile_ms 226, pipelines/front_ms 207, main/busy/ms 58.61, render/thread/ms 43.06, render/busy/ms 41.71, pipelines/driver_ms 19.97, render/graph/ms 3.46, stream/drive/ms 1.35, render/present/ms 1.10, stream/tick/ms 1.08, terrain/update/ms 1.02 |
| warm-up | 3.66 | 25.5 | pipelines/wait_ms 375, pipelines/compile_ms 226, pipelines/front_ms 207, render/thread/ms 62.83, render/busy/ms 61.44, main/busy/ms 26.67, pipelines/driver_ms 19.97, render/graph/ms 10.32, stream/drive/ms 2.85, stream/tick/ms 1.33, render/present/ms 1.17, terrain/update/ms 0.88 |
| warm-up | 3.81 | 78.2 | pipelines/wait_ms 585, pipelines/compile_ms 205, pipelines/front_ms 190, render/thread/ms 60.62, render/busy/ms 59.15, main/busy/ms 53.07, stream/drive/ms 37.81, pipelines/driver_ms 14.32, render/graph/ms 4.30, render/present/ms 1.20, stream/tick/ms 1.05, terrain/update/ms 0.59 |
| warm-up | 3.89 | 61.6 | pipelines/wait_ms 2194, pipelines/compile_ms 99.65, pipelines/front_ms 94.34, render/thread/ms 42.21, render/busy/ms 40.43, main/busy/ms 30.16, stream/drive/ms 11.30, pipelines/driver_ms 5.31, terrain/update/ms 5.11, render/graph/ms 4.35, render/present/ms 1.54, stream/spawn/ms 1.35 |
| warm-up | 3.93 | 38.9 | pipelines/wait_ms 3159, pipelines/compile_ms 120, pipelines/front_ms 113, render/thread/ms 59.15, render/busy/ms 57.06, main/busy/ms 34.93, stream/drive/ms 12.42, pipelines/driver_ms 6.81, render/graph/ms 5.71, terrain/update/ms 4.98, stream/spawn/ms 2.63, render/present/ms 1.83 |
| warm-up | 3.99 | 61.7 | pipelines/wait_ms 6382, pipelines/compile_ms 105, pipelines/front_ms 98.66, render/thread/ms 69.52, render/busy/ms 67.59, main/busy/ms 38.07, stream/drive/ms 12.62, render/graph/ms 6.83, pipelines/driver_ms 6.32, stream/spawn/ms 1.93, render/present/ms 1.67, stream/tick/ms 1.36 |
| warm-up | 4.02 | 25.2 | pipelines/wait_ms 6580, pipelines/compile_ms 97.40, pipelines/front_ms 91.80, main/busy/ms 66.45, render/thread/ms 58.96, render/busy/ms 56.67, stream/drive/ms 47.03, render/graph/ms 5.61, pipelines/driver_ms 5.60, stream/spawn/ms 1.91, render/present/ms 1.91, stream/tick/ms 1.09 |
| warm-up | 4.09 | 77.2 | pipelines/wait_ms 2349, main/busy/ms 72.64, pipelines/compile_ms 50.26, stream/drive/ms 48.08, pipelines/front_ms 47.60, render/thread/ms 42.86, render/busy/ms 40.76, render/graph/ms 3.73, stream/spawn/ms 3.16, pipelines/driver_ms 2.66, render/present/ms 1.63, stream/tick/ms 0.92 |
| warm-up | 4.13 | 35.0 | pipelines/wait_ms 2498, pipelines/compile_ms 107, pipelines/front_ms 103, render/thread/ms 47.14, render/busy/ms 45.73, main/busy/ms 44.01, stream/drive/ms 21.59, render/graph/ms 3.86, pipelines/driver_ms 3.80, stream/spawn/ms 2.10, render/present/ms 1.10, terrain/update/ms 0.83 |
| warm-up | 4.19 | 34.4 | main/busy/ms 37.74, stream/drive/ms 28.48, render/thread/ms 15.19, render/busy/ms 14.12, render/graph/ms 3.12, render/present/ms 0.91, stream/activate/ms 0.75, terrain/update/ms 0.58, stream/tick/ms 0.58, terrain/select/ms 0.47, physics/insert/ms 0.28, bigspace/total/ms 0.27 |
| warm-up | 4.30 | 69.0 | pipelines/wait_ms 3991, pipelines/compile_ms 205, pipelines/front_ms 192, main/busy/ms 72.38, stream/drive/ms 56.29, render/thread/ms 22.82, render/busy/ms 21.51, pipelines/driver_ms 13.14, render/graph/ms 3.66, terrain/update/ms 2.76, render/present/ms 1.11, stream/tick/ms 1.07 |
| warm-up | 4.37 | 59.0 | pipelines/wait_ms 4300, main/busy/ms 53.21, pipelines/compile_ms 39.00, stream/drive/ms 38.76, pipelines/front_ms 36.12, render/thread/ms 24.46, render/busy/ms 23.15, render/graph/ms 3.51, pipelines/driver_ms 2.89, render/present/ms 1.03, stream/spawn/ms 0.99, stream/tick/ms 0.76 |
| warm-up | 4.45 | 26.8 | render/thread/ms 37.85, render/busy/ms 25.24, main/busy/ms 19.76, render/present/ms 12.39, stream/drive/ms 3.57, render/graph/ms 3.31, stream/spawn/ms 1.85, stream/tick/ms 0.75, terrain/update/ms 0.48, terrain/select/ms 0.38, bigspace/total/ms 0.32, render/acquire/ms 0.22 |
| warm-up | 4.58 | 30.0 | render/thread/ms 35.51, render/busy/ms 34.11, main/busy/ms 23.19, stream/drive/ms 4.40, render/graph/ms 3.48, stream/spawn/ms 2.13, render/present/ms 1.10, stream/tick/ms 0.91, bigspace/total/ms 0.65, terrain/update/ms 0.63, terrain/select/ms 0.49, stream/activate/ms 0.48 |
| warm-up | 4.80 | 47.2 | render/thread/ms 51.39, render/busy/ms 49.33, main/busy/ms 34.34, render/graph/ms 7.28, stream/drive/ms 3.81, stream/spawn/ms 2.33, render/present/ms 1.71, bigspace/total/ms 0.71, stream/tick/ms 0.69, terrain/update/ms 0.67, physics/insert/ms 0.56, terrain/select/ms 0.51 |
| warm-up | 4.83 | 25.0 | render/thread/ms 54.94, render/busy/ms 53.08, main/busy/ms 29.25, render/graph/ms 6.26, stream/drive/ms 4.10, stream/spawn/ms 2.32, render/present/ms 1.51, stream/tick/ms 0.73, bigspace/total/ms 0.68, terrain/update/ms 0.65, terrain/select/ms 0.50, render/acquire/ms 0.35 |
| warm-up | 4.88 | 55.6 | pipelines/wait_ms 2523, pipelines/compile_ms 178, pipelines/front_ms 171, render/thread/ms 61.01, render/busy/ms 59.36, main/busy/ms 53.53, pipelines/driver_ms 6.85, render/graph/ms 5.54, stream/drive/ms 3.92, stream/spawn/ms 2.08, render/present/ms 1.30, bigspace/total/ms 1.10 |
| warm-up | 4.95 | 65.4 | pipelines/wait_ms 3998, pipelines/compile_ms 179, pipelines/front_ms 172, render/thread/ms 91.34, render/busy/ms 89.73, main/busy/ms 72.26, pipelines/driver_ms 7.11, render/graph/ms 5.81, stream/drive/ms 4.66, stream/spawn/ms 2.09, render/present/ms 1.26, stream/tick/ms 1.15 |
| warm-up | 4.99 | 36.5 | pipelines/wait_ms 1474, render/thread/ms 81.55, render/busy/ms 80.26, main/busy/ms 43.99, render/graph/ms 4.61, stream/drive/ms 4.44, stream/spawn/ms 2.06, pipelines/compile_ms 1.40, pipelines/front_ms 1.15, render/present/ms 1.04, stream/tick/ms 0.95, terrain/update/ms 0.79 |
| warm-up | 5.01 | 27.5 | render/thread/ms 53.85, render/busy/ms 52.46, main/busy/ms 20.03, render/graph/ms 3.67, stream/drive/ms 3.62, stream/spawn/ms 2.11, render/present/ms 1.16, bigspace/total/ms 0.59, stream/tick/ms 0.58, terrain/update/ms 0.58, terrain/select/ms 0.45, bigspace/high_precision/ms 0.32 |
| warm-up | 5.07 | 34.6 | pipelines/wait_ms 9789, pipelines/compile_ms 196, pipelines/front_ms 178, render/thread/ms 43.59, render/busy/ms 41.62, main/busy/ms 29.13, pipelines/driver_ms 17.62, render/graph/ms 7.35, stream/drive/ms 6.54, stream/spawn/ms 2.96, render/present/ms 1.67, stream/tick/ms 0.80 |
| warm-up | 5.10 | 34.8 | pipelines/wait_ms 10.3 k, pipelines/compile_ms 48.46, render/thread/ms 46.83, render/busy/ms 45.30, pipelines/front_ms 43.11, main/busy/ms 24.71, render/graph/ms 6.96, pipelines/driver_ms 5.35, stream/drive/ms 4.09, stream/spawn/ms 2.47, render/present/ms 1.32, stream/tick/ms 0.83 |
| warm-up | 5.13 | 31.0 | pipelines/wait_ms 4974, render/thread/ms 43.56, render/busy/ms 42.25, main/busy/ms 31.29, stream/drive/ms 14.34, pipelines/compile_ms 8.45, pipelines/front_ms 6.95, render/graph/ms 5.07, stream/spawn/ms 2.18, pipelines/driver_ms 1.50, render/present/ms 1.10, stream/tick/ms 0.69 |
| warm-up | 5.27 | 32.3 | main/busy/ms 32.10, render/thread/ms 23.43, render/busy/ms 22.06, stream/drive/ms 18.23, render/graph/ms 6.27, render/present/ms 1.09, stream/spawn/ms 0.91, stream/tick/ms 0.64, terrain/update/ms 0.49, bigspace/total/ms 0.46, terrain/select/ms 0.40, render/acquire/ms 0.28 |
| warm-up | 5.52 | 25.0 | main/busy/ms 29.86, render/thread/ms 22.72, render/busy/ms 20.92, stream/drive/ms 9.75, render/graph/ms 5.51, stream/spawn/ms 2.03, render/present/ms 1.55, stream/tick/ms 0.86, bigspace/total/ms 0.79, terrain/update/ms 0.71, terrain/select/ms 0.56, bigspace/high_precision/ms 0.39 |
| warm-up | 5.55 | 29.5 | render/thread/ms 31.56, render/busy/ms 29.93, main/busy/ms 25.01, render/graph/ms 5.55, stream/drive/ms 3.65, stream/spawn/ms 2.15, render/present/ms 1.32, stream/tick/ms 0.87, bigspace/total/ms 0.86, terrain/update/ms 0.82, terrain/select/ms 0.63, bigspace/high_precision/ms 0.39 |
| warm-up | 5.97 | 34.9 | pipelines/wait_ms 2482, main/busy/ms 50.91, render/thread/ms 43.46, render/busy/ms 41.75, pipelines/compile_ms 23.62, pipelines/front_ms 22.47, render/graph/ms 8.47, render/present/ms 1.43, stream/drive/ms 1.22, pipelines/driver_ms 1.14, anim/evaluate_cpu/ms 0.80, bigspace/total/ms 0.77 |
| warm-up | 6.02 | 52.2 | main/busy/ms 64.11, render/thread/ms 60.05, render/busy/ms 58.31, render/graph/ms 9.37, bigspace/total/ms 2.69, bigspace/low_precision/ms 2.02, stream/drive/ms 1.90, render/present/ms 1.38, stream/tick/ms 1.01, anim/evaluate_cpu/ms 0.85, stream/spawn/ms 0.53, anim/evaluate/ms 0.52 |
| warm-up | 6.05 | 30.2 | render/thread/ms 54.66, render/busy/ms 53.07, main/busy/ms 44.13, render/graph/ms 8.10, bigspace/total/ms 2.54, stream/drive/ms 1.96, bigspace/low_precision/ms 1.89, render/present/ms 1.26, stream/tick/ms 1.05, terrain/update/ms 0.68, terrain/select/ms 0.53, stream/spawn/ms 0.53 |
| warm-up | 6.96 | 31.8 | pipelines/wait_ms 7958, pipelines/compile_ms 43.98, pipelines/front_ms 41.98, main/busy/ms 37.05, render/thread/ms 28.39, render/busy/ms 27.14, render/graph/ms 9.01, pipelines/driver_ms 1.99, stream/drive/ms 1.18, render/present/ms 1.05, bigspace/total/ms 0.79, physics/insert/ms 0.65 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_fog\balmora@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels-1 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 17%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **98%** (min 96%) · 2580 MHz (min 2580) · memory 8001 MHz · 95.5 W · 79 °C (max 82) · VRAM 5750 MiB · throttled 100% of samples
