# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1428 frames over 15.0 s · **median 10.46 ms (96 fps)** · mean 10.51 · p95 11.59 · p99 12.18 (1% low 82 fps) · worst 19.58 ms · 3 spikes over 1.5× median · 12.0 stutters/min (work 4.0/min)
- warm-up: 291 frames over 7.7 s, worst 2628.8 ms
- hitches (> 26.2 ms): **0** measured, 30 in warm-up
- timed GPU passes: 8.67 ms of a 10.51 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.46 ms, limit 11 ms
- pass worst frame 19.58 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.614 | 2.627 | 3.030 | 1.00 | 0.061 |
| dlss_super_resolution | 2.163 | 2.173 | 2.446 | 1.00 | 0.189 |
| early prepass | 0.599 | 0.602 | 0.942 | 1.00 | 0.095 |
| atmosphere_luts | 0.366 | 0.368 | 0.673 | 1.00 | 0.003 |
| bloom | 0.337 | 0.339 | 0.495 | 1.00 | 0.064 |
| main_transmissive_pass_3d | 0.296 | 0.297 | 0.429 | 1.00 | 0.008 |
| water_velocity | 0.253 | 0.254 | 0.269 | 1.00 | 0.002 |
| water_sspr | 0.237 | 0.238 | 0.382 | 1.00 | 0.002 |
| ssao | 0.209 | 0.210 | 0.330 | 1.00 | 0.008 |
| uniform_allocation | 0.180 | 0.181 | 0.182 | 1.00 | 0.034 |
| atmos_froxel_inject | 0.154 | 0.154 | 0.164 | 1.00 | 0.002 |
| clouds_trace | 0.147 | 0.148 | 0.168 | 1.00 | 0.001 |
| water_fft | 0.144 | 0.145 | 0.143 | 1.00 | 0.003 |
| bin_unpacking | 0.136 | 0.136 | 0.144 | 1.00 | 0.025 |
| upscaling | 0.122 | 0.123 | 0.126 | 1.00 | 0.004 |
| clouds_composite | 0.115 | 0.116 | 0.117 | 1.00 | 0.002 |
| tonemapping | 0.092 | 0.092 | 0.094 | 1.00 | 0.004 |
| clustering | 0.068 | 0.068 | 0.076 | 1.00 | 0.021 |
| render_sky | 0.056 | 0.056 | 0.056 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.054 | 0.433 | 0.560 | 0.12 | 0.001 |
| clouds_resolve | 0.051 | 0.052 | 0.053 | 1.00 | 0.001 |
| water_snapshot | 0.040 | 0.041 | 0.056 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.040 | 0.040 | 0.042 | 1.00 | 0.006 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.029 | 0.234 | 0.239 | 0.12 | 0.004 |
| per_view_shadow_pass | 0.024 | 0.024 | 0.030 | 1.00 | 0.007 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.023 | 0.023 | 0.024 | 1.00 | 0.074 |
| atmosphere_environment | 0.011 | 0.011 | 0.014 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.003 | 0.004 | 0.004 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.018 | 0.12 | 0.005 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 92.128 | 0.000 | 13245.74 | 131558.3 |
| render/thread/ms | 9.543 | 10.647 | 18.25 | 13626.7 |
| render/busy/ms | 7.755 | 8.788 | 14.11 | 11074.3 |
| main/busy/ms | 7.347 | 8.458 | 12.43 | 10492.2 |
| render/graph/ms | 4.576 | 5.420 | 9.55 | 6534.3 |
| pipelines/compile_ms | 1.372 | 0.000 | 128.94 | 1959.5 |
| pipelines/front_ms | 1.326 | 0.000 | 124.53 | 1894.1 |
| render/acquire/ms | 1.309 | 2.422 | 4.21 | 1868.8 |
| render/present/ms | 0.479 | 0.574 | 8.84 | 683.6 |
| bigspace/total/ms | 0.354 | 0.647 | 1.86 | 506.1 |
| terrain/update/ms | 0.230 | 0.292 | 0.56 | 328.4 |
| terrain/select/ms | 0.201 | 0.255 | 0.53 | 286.4 |
| stream/drive/ms | 0.185 | 0.253 | 0.64 | 264.4 |
| controller/move/ms | 0.175 | 0.352 | 1.02 | 249.6 |
| far/select/ms | 0.165 | 0.256 | 0.60 | 236.3 |
| controller/feed/ms | 0.160 | 0.216 | 0.43 | 228.2 |
| stream/tick/ms | 0.159 | 0.222 | 0.62 | 227.2 |
| bigspace/high_precision/ms | 0.117 | 0.274 | 1.28 | 166.6 |
| bigspace/low_precision/ms | 0.084 | 0.166 | 1.22 | 120.3 |
| anim/evaluate/ms | 0.084 | 0.209 | 1.45 | 119.7 |
| anim/evaluate_cpu/ms | 0.061 | 0.080 | 0.38 | 86.6 |
| pipelines/driver_ms | 0.046 | 0.000 | 4.42 | 65.4 |
| physics/step/ms | 0.034 | 0.061 | 0.84 | 48.4 |
| weather/map/ms | 0.025 | 0.033 | 0.26 | 35.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.06 | 15.7 |
| physics/insert/ms | 0.008 | 0.039 | 0.59 | 11.8 |
| script/update/ms | 0.007 | 0.008 | 0.02 | 9.7 |
| controller/avoid/ms | 0.006 | 0.007 | 0.28 | 8.0 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.2 |
| anim/apply/ms | 0.004 | 0.006 | 0.19 | 6.3 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 5.7 |
| controller/write/ms | 0.002 | 0.002 | 0.13 | 2.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.16 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.4 |
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
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 12 | 8 |
| crowd/c0/joints/count | count | 272 | 408 | 272 |
| crowd/c0/meshes/count | count | 48 | 72 | 48 |
| crowd/c1/count | count | 122 | 123 | 123 |
| crowd/c1/joints/count | count | 4848 | 4896 | 4880 |
| crowd/c1/meshes/count | count | 1080 | 1099 | 1088 |
| crowd/c2/count | count | 284 | 284 | 283 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 7170 | 4212 |
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
| frame/work/ms | ms | 9.29 | 14.11 | 9.25 |
| gpu/timed/ms | ms | 9.29 | 11.11 | 9.25 |
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
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 22 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 23 | 22 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 22 | 21 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 31 | 36 | 31 |
| phases/shadow/bins | count | 2029 | 2336 | 2029 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5346 | 5631 | 5346 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 31 | 36 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1234 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 178 | 178 | 178 |
| pipelines/waiting | count | 0 | 35 | 0 |
| preprocess/bind_groups | count | 0 | 202 | 0 |
| preprocess/bind_groups_created | count | 0 | 202 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 201 | 201 | 201 |
| process/cpu_usage | % | 24.85 | 39.19 | 36.06 |
| process/mem_usage | GiB | 4.31 | 4.38 | 4.38 |
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
| started_load_count |  | 3547 | 3549 | 3549 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 386 | 386 | 386 |
| statics/materials_bound | count | 386 | 386 | 386 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
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
| system/cpu_usage | % | 33.05 | 54.70 | 33.59 |
| system/mem_usage | % | 72.95 | 73.11 | 73.11 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2480 | 2480 | 2480 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 89 | 96 | 89 |
| views/all/bins | count | 5199 | 5464 | 5199 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.9 k | 14.6 k |
| views/all/pipelines | count | 85 | 92 | 85 |
| views/all/pipelines_distinct | count | 75 | 77 | 75 |
| views/all/work_items | count | 14.8 k | 15.0 k | 14.8 k |
| views/cascade/batch_sets | count | 31 | 36 | 31 |
| views/cascade/bins | count | 2029 | 2336 | 2029 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5346 | 5631 | 5346 |
| views/cascade/pipelines | count | 31 | 36 | 31 |
| views/cascade/work_items | count | 5346 | 5631 | 5346 |
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
| views/main/bins | count | 3170 | 3170 | 3170 |
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
| anim/bones_written/count | count | 0.02 | 33 | 33 |
| anim/events/count | count | 0.10 | 3 | 137 |
| assets/Image/added | count | 0.00 | 2 | 2 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.02 | 25 | 25 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.00 | 2 | 2 |
| assets/StandardMaterial/modified | count | 3.47 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.84 | 405 | 1205 |
| physics/inserts | count | 0.71 | 120 | 1018 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.63 | 2 | 900 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 3 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.2 ms; slowest 30, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.63 | 2628.8 | main/busy/ms 2605, surface/advance_cpu_ms 68.44, surface/cell_upload_ms 0.76, stream/drive/ms 0.50, stream/tick/ms 0.14, bigspace/total/ms 0.10, weather/map/ms 0.10, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.03, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.33 | 698.3 | main/busy/ms 2640, render/thread/ms 662, render/busy/ms 520, render/acquire/ms 139, surface/advance_cpu_ms 68.44, render/graph/ms 4.72, render/present/ms 2.05, stream/drive/ms 1.19, surface/cell_upload_ms 0.76, stream/tick/ms 0.63, terrain/update/ms 0.56, terrain/select/ms 0.37 |
| warm-up | 3.63 | 300.7 | pipelines/wait_ms 4917, render/thread/ms 674, render/busy/ms 531, pipelines/compile_ms 322, pipelines/front_ms 299, render/acquire/ms 140, main/busy/ms 53.36, pipelines/driver_ms 23.70, terrain/update/ms 19.78, render/graph/ms 8.78, render/present/ms 2.84, stream/drive/ms 1.25 |
| warm-up | 3.88 | 99.1 | pipelines/compile_ms 238, pipelines/front_ms 230, render/thread/ms 75.21, render/busy/ms 73.38, main/busy/ms 71.52, pipelines/driver_ms 8.08, render/graph/ms 4.00, stream/drive/ms 1.95, render/present/ms 1.55, stream/tick/ms 1.25, terrain/update/ms 0.74, terrain/select/ms 0.57 |
| warm-up | 3.98 | 33.2 | main/busy/ms 37.98, render/thread/ms 33.09, render/busy/ms 31.04, stream/drive/ms 26.43, render/graph/ms 6.37, render/present/ms 1.85, stream/tick/ms 0.77, terrain/update/ms 0.53, terrain/select/ms 0.43, bigspace/total/ms 0.21, render/acquire/ms 0.21, bigspace/high_precision/ms 0.11 |
| warm-up | 4.01 | 32.5 | render/thread/ms 50.11, render/busy/ms 48.93, main/busy/ms 46.35, stream/drive/ms 33.57, render/graph/ms 4.82, render/present/ms 0.99, stream/tick/ms 0.72, terrain/update/ms 0.66, terrain/select/ms 0.51, bigspace/total/ms 0.25, render/acquire/ms 0.19, bigspace/high_precision/ms 0.12 |
| warm-up | 4.08 | 33.9 | pipelines/wait_ms 2338, pipelines/compile_ms 192, pipelines/front_ms 176, main/busy/ms 39.78, render/thread/ms 31.27, render/busy/ms 29.93, stream/drive/ms 29.75, pipelines/driver_ms 16.57, render/graph/ms 3.87, render/present/ms 1.11, stream/tick/ms 0.89, terrain/update/ms 0.59 |
| warm-up | 4.16 | 63.4 | pipelines/wait_ms 3173, pipelines/compile_ms 78.10, pipelines/front_ms 74.08, main/busy/ms 50.22, render/thread/ms 40.05, render/busy/ms 38.83, stream/drive/ms 32.96, pipelines/driver_ms 4.02, render/graph/ms 2.86, stream/spawn/ms 2.11, stream/tick/ms 1.70, render/present/ms 1.02 |
| warm-up | 4.19 | 31.9 | pipelines/wait_ms 3846, pipelines/compile_ms 82.09, pipelines/front_ms 77.43, render/thread/ms 51.71, render/busy/ms 50.16, main/busy/ms 39.56, stream/drive/ms 22.79, pipelines/driver_ms 4.66, render/graph/ms 4.01, stream/spawn/ms 1.91, stream/tick/ms 1.78, render/present/ms 1.27 |
| warm-up | 4.28 | 44.6 | render/thread/ms 50.40, render/busy/ms 48.27, main/busy/ms 30.75, stream/drive/ms 7.81, render/graph/ms 6.65, stream/spawn/ms 4.13, stream/tick/ms 1.86, render/present/ms 1.73, terrain/update/ms 0.62, stream/activate/ms 0.57, terrain/select/ms 0.50, render/acquire/ms 0.39 |
| warm-up | 4.32 | 42.8 | render/thread/ms 59.42, render/busy/ms 57.53, main/busy/ms 48.72, stream/drive/ms 29.75, render/graph/ms 5.76, stream/spawn/ms 2.94, stream/tick/ms 2.11, render/present/ms 1.52, stream/activate/ms 0.80, terrain/update/ms 0.72, terrain/select/ms 0.57, render/acquire/ms 0.37 |
| warm-up | 4.42 | 99.6 | pipelines/wait_ms 3013, pipelines/compile_ms 181, pipelines/front_ms 172, render/thread/ms 90.20, render/busy/ms 66.23, main/busy/ms 45.83, stream/drive/ms 27.07, render/present/ms 23.73, pipelines/driver_ms 8.29, render/graph/ms 3.27, stream/spawn/ms 1.24, stream/tick/ms 0.98 |
| warm-up | 4.52 | 54.8 | pipelines/wait_ms 699, pipelines/compile_ms 148, pipelines/front_ms 139, main/busy/ms 93.01, stream/drive/ms 77.99, render/thread/ms 32.74, render/busy/ms 31.47, pipelines/driver_ms 8.97, render/graph/ms 3.49, render/present/ms 1.09, stream/tick/ms 0.88, terrain/update/ms 0.58 |
| warm-up | 4.56 | 39.9 | pipelines/wait_ms 3047, pipelines/compile_ms 229, pipelines/front_ms 216, main/busy/ms 55.56, stream/drive/ms 42.72, render/thread/ms 25.17, render/busy/ms 23.80, pipelines/driver_ms 13.05, render/graph/ms 3.61, render/present/ms 1.21, stream/spawn/ms 1.20, stream/tick/ms 0.82 |
| warm-up | 4.62 | 29.7 | main/busy/ms 25.34, render/thread/ms 22.97, render/busy/ms 21.62, stream/drive/ms 10.58, render/graph/ms 3.66, stream/spawn/ms 1.45, render/present/ms 1.18, stream/tick/ms 0.89, terrain/update/ms 0.55, bigspace/total/ms 0.48, terrain/select/ms 0.43, render/acquire/ms 0.17 |
| warm-up | 4.76 | 31.1 | render/thread/ms 35.85, render/busy/ms 34.08, main/busy/ms 31.29, stream/drive/ms 11.78, render/graph/ms 6.55, stream/spawn/ms 3.14, render/present/ms 1.43, terrain/update/ms 0.96, terrain/select/ms 0.79, stream/tick/ms 0.76, stream/activate/ms 0.53, bigspace/total/ms 0.49 |
| warm-up | 4.94 | 28.8 | pipelines/wait_ms 1145, pipelines/compile_ms 140, pipelines/front_ms 134, render/thread/ms 38.81, render/busy/ms 36.96, main/busy/ms 23.59, stream/drive/ms 9.17, render/graph/ms 7.64, pipelines/driver_ms 5.92, stream/spawn/ms 2.03, render/present/ms 1.60, bigspace/total/ms 0.57 |
| warm-up | 4.98 | 34.0 | pipelines/wait_ms 8487, pipelines/compile_ms 183, pipelines/front_ms 172, render/thread/ms 52.68, render/busy/ms 50.82, main/busy/ms 28.15, pipelines/driver_ms 10.04, stream/drive/ms 8.97, render/graph/ms 7.75, stream/spawn/ms 2.05, render/present/ms 1.60, stream/tick/ms 1.06 |
| warm-up | 5.01 | 35.0 | pipelines/wait_ms 11.1 k, pipelines/compile_ms 61.62, render/thread/ms 54.43, render/busy/ms 53.09, pipelines/front_ms 52.84, main/busy/ms 31.43, pipelines/driver_ms 8.77, render/graph/ms 5.49, stream/drive/ms 4.65, stream/spawn/ms 2.06, stream/tick/ms 1.15, render/present/ms 1.12 |
| warm-up | 5.04 | 32.2 | pipelines/wait_ms 3752, render/thread/ms 53.69, render/busy/ms 52.47, main/busy/ms 31.36, pipelines/compile_ms 18.82, pipelines/front_ms 14.17, terrain/update/ms 6.82, stream/drive/ms 5.62, render/graph/ms 4.78, pipelines/driver_ms 4.64, stream/spawn/ms 2.42, render/present/ms 0.96 |
| warm-up | 5.10 | 40.2 | main/busy/ms 45.69, render/thread/ms 43.42, render/busy/ms 42.03, render/graph/ms 6.22, stream/drive/ms 3.87, terrain/update/ms 3.17, stream/spawn/ms 2.08, render/present/ms 1.09, bigspace/total/ms 0.83, stream/tick/ms 0.74, terrain/select/ms 0.51, bigspace/high_precision/ms 0.49 |
| warm-up | 5.15 | 48.5 | pipelines/wait_ms 1073, pipelines/compile_ms 152, pipelines/front_ms 147, render/thread/ms 70.10, render/busy/ms 68.63, main/busy/ms 53.98, stream/drive/ms 5.97, render/graph/ms 5.62, pipelines/driver_ms 4.92, stream/spawn/ms 2.11, render/present/ms 1.09, bigspace/total/ms 1.01 |
| warm-up | 5.19 | 40.7 | pipelines/wait_ms 2565, pipelines/compile_ms 168, pipelines/front_ms 163, render/thread/ms 73.68, render/busy/ms 71.57, main/busy/ms 60.66, render/graph/ms 6.88, stream/drive/ms 6.22, pipelines/driver_ms 5.68, stream/spawn/ms 2.70, render/present/ms 1.62, bigspace/total/ms 0.98 |
| warm-up | 5.25 | 58.6 | pipelines/wait_ms 5879, pipelines/compile_ms 109, pipelines/front_ms 102, render/thread/ms 72.49, render/busy/ms 70.60, main/busy/ms 61.22, pipelines/driver_ms 6.59, render/graph/ms 6.29, stream/drive/ms 4.82, stream/spawn/ms 3.16, render/present/ms 1.50, bigspace/total/ms 1.09 |
| warm-up | 5.30 | 32.0 | main/busy/ms 37.84, render/thread/ms 34.38, render/busy/ms 33.05, stream/drive/ms 16.34, render/graph/ms 3.89, stream/spawn/ms 2.11, render/present/ms 1.06, terrain/update/ms 0.81, stream/tick/ms 0.74, bigspace/total/ms 0.71, terrain/select/ms 0.63, bigspace/high_precision/ms 0.39 |
| warm-up | 5.42 | 32.5 | pipelines/wait_ms 1449, main/busy/ms 36.86, render/thread/ms 20.97, render/busy/ms 19.65, stream/drive/ms 19.56, pipelines/compile_ms 12.29, pipelines/front_ms 11.07, render/graph/ms 5.90, stream/spawn/ms 2.33, pipelines/driver_ms 1.22, render/present/ms 1.15, stream/tick/ms 0.62 |
| warm-up | 5.69 | 43.7 | main/busy/ms 38.91, render/thread/ms 38.60, render/busy/ms 36.91, stream/drive/ms 13.20, render/graph/ms 5.39, stream/spawn/ms 2.29, render/present/ms 1.43, stream/tick/ms 1.31, bigspace/total/ms 0.94, terrain/update/ms 0.69, terrain/select/ms 0.55, bigspace/high_precision/ms 0.44 |
| warm-up | 6.51 | 31.0 | pipelines/wait_ms 5609, pipelines/compile_ms 63.24, pipelines/front_ms 61.16, main/busy/ms 43.37, render/thread/ms 37.69, render/busy/ms 36.23, render/graph/ms 10.82, pipelines/driver_ms 2.08, stream/drive/ms 1.18, render/present/ms 1.10, physics/insert/ms 0.85, bigspace/total/ms 0.80 |
| warm-up | 6.54 | 29.4 | pipelines/wait_ms 5609, pipelines/compile_ms 63.24, pipelines/front_ms 61.16, main/busy/ms 46.56, render/thread/ms 40.95, render/busy/ms 38.97, render/graph/ms 7.54, pipelines/driver_ms 2.08, stream/drive/ms 1.39, render/present/ms 1.08, physics/insert/ms 0.90, render/acquire/ms 0.90 |
| warm-up | 6.57 | 33.7 | render/thread/ms 45.68, main/busy/ms 45.46, render/busy/ms 43.69, render/graph/ms 7.49, stream/drive/ms 1.46, bigspace/total/ms 1.21, render/present/ms 1.06, render/acquire/ms 0.93, physics/insert/ms 0.87, stream/tick/ms 0.70, anim/evaluate_cpu/ms 0.57, terrain/update/ms 0.48 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_pbr\balmora@2560x1440_off=normalmaps_orm_parallax-3 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 15%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 97%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.0 W · 74 °C (max 78) · VRAM 5713 MiB · throttled 100% of samples
