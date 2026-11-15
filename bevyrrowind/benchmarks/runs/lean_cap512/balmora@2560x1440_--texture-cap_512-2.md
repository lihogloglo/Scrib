# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1339 frames over 15.0 s · **median 11.10 ms (90 fps)** · mean 11.21 · p95 12.63 · p99 13.52 (1% low 74 fps) · worst 17.82 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 527 frames over 9.7 s, worst 2482.9 ms
- hitches (> 27.8 ms): **0** measured, 24 in warm-up
- timed GPU passes: 8.96 ms of a 11.21 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.10 ms, limit 11 ms
- pass worst frame 17.82 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.881 | 2.957 | 3.333 | 0.97 | 0.079 |
| dlss_super_resolution | 2.119 | 2.174 | 2.464 | 0.97 | 0.184 |
| early prepass | 0.615 | 0.631 | 0.970 | 0.97 | 0.121 |
| atmosphere_luts | 0.353 | 0.362 | 0.660 | 0.97 | 0.004 |
| bloom | 0.331 | 0.339 | 0.652 | 0.97 | 0.065 |
| main_transmissive_pass_3d | 0.293 | 0.300 | 0.306 | 0.97 | 0.008 |
| water_velocity | 0.251 | 0.257 | 0.273 | 0.97 | 0.002 |
| water_sspr | 0.238 | 0.244 | 0.334 | 0.97 | 0.002 |
| uniform_allocation | 0.236 | 0.242 | 0.248 | 0.97 | 0.044 |
| ssao | 0.201 | 0.207 | 0.213 | 0.97 | 0.008 |
| bin_unpacking | 0.187 | 0.192 | 0.197 | 0.97 | 0.031 |
| atmos_froxel_inject | 0.150 | 0.154 | 0.165 | 0.97 | 0.002 |
| clouds_trace | 0.146 | 0.150 | 0.172 | 0.97 | 0.001 |
| water_fft | 0.135 | 0.139 | 0.137 | 0.97 | 0.003 |
| upscaling | 0.117 | 0.120 | 0.149 | 0.97 | 0.005 |
| clouds_composite | 0.114 | 0.117 | 0.117 | 0.97 | 0.002 |
| tonemapping | 0.086 | 0.089 | 0.091 | 0.97 | 0.004 |
| clustering | 0.068 | 0.069 | 0.076 | 0.97 | 0.022 |
| render_sky | 0.057 | 0.058 | 0.059 | 0.97 | 0.002 |
| lightprobe_irradiance_map | 0.053 | 0.442 | 0.758 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.052 | 0.053 | 0.97 | 0.001 |
| early_mesh_preprocessing | 0.040 | 0.041 | 0.041 | 0.97 | 0.007 |
| water_snapshot | 0.040 | 0.041 | 0.056 | 0.97 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.035 | 0.97 | 0.003 |
| main_transparent_pass_3d | 0.030 | 0.030 | 0.042 | 0.97 | 0.074 |
| lightprobe_radiance_map | 0.029 | 0.236 | 0.235 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.027 | 0.030 | 0.97 | 0.001 |
| water_mips | 0.024 | 0.025 | 0.024 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.024 | 0.97 | 0.007 |
| atmosphere_environment | 0.011 | 0.011 | 0.012 | 0.97 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.97 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.97 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.008 | 0.97 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.002 | 0.003 | 0.97 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.007 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 53.744 | 0.000 | 11354.37 | 71963.6 |
| render/thread/ms | 10.281 | 11.684 | 15.16 | 13765.7 |
| render/busy/ms | 9.097 | 10.409 | 14.29 | 12180.9 |
| main/busy/ms | 7.552 | 9.228 | 13.10 | 10112.5 |
| render/graph/ms | 5.606 | 6.667 | 8.15 | 7506.6 |
| render/acquire/ms | 0.691 | 1.836 | 3.04 | 925.7 |
| render/present/ms | 0.492 | 0.621 | 2.09 | 659.0 |
| pipelines/compile_ms | 0.452 | 0.000 | 130.64 | 605.3 |
| pipelines/front_ms | 0.437 | 0.000 | 126.86 | 585.1 |
| bigspace/total/ms | 0.341 | 0.565 | 3.23 | 457.2 |
| terrain/update/ms | 0.225 | 0.285 | 0.64 | 301.6 |
| stream/drive/ms | 0.198 | 0.287 | 0.55 | 264.8 |
| terrain/select/ms | 0.195 | 0.247 | 0.44 | 261.6 |
| far/select/ms | 0.170 | 0.254 | 0.44 | 228.1 |
| stream/tick/ms | 0.170 | 0.251 | 0.51 | 227.4 |
| controller/feed/ms | 0.155 | 0.208 | 0.44 | 208.0 |
| controller/move/ms | 0.135 | 0.224 | 0.54 | 181.4 |
| bigspace/high_precision/ms | 0.114 | 0.242 | 2.82 | 153.0 |
| anim/evaluate/ms | 0.092 | 0.204 | 1.11 | 122.5 |
| bigspace/low_precision/ms | 0.081 | 0.164 | 2.12 | 107.8 |
| anim/evaluate_cpu/ms | 0.053 | 0.068 | 0.22 | 71.4 |
| physics/step/ms | 0.032 | 0.059 | 0.30 | 43.5 |
| weather/map/ms | 0.025 | 0.035 | 0.18 | 33.6 |
| pipelines/driver_ms | 0.015 | 0.000 | 4.09 | 20.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.06 | 14.3 |
| script/update/ms | 0.007 | 0.011 | 0.17 | 10.0 |
| controller/avoid/ms | 0.006 | 0.008 | 0.04 | 8.2 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 6.7 |
| anim/apply/ms | 0.004 | 0.006 | 0.04 | 5.6 |
| script/ui/ms | 0.004 | 0.006 | 0.06 | 5.2 |
| controller/write/ms | 0.002 | 0.002 | 0.13 | 2.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.9 |
| physics/insert/ms | 0.000 | 0.000 | 0.33 | 0.5 |
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
| anim/a2/count | count | 0 | 3 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 16 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 11 | 8 |
| crowd/c0/joints/count | count | 272 | 374 | 272 |
| crowd/c0/meshes/count | count | 48 | 66 | 48 |
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 6279 | 4212 |
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
| frame/work/ms | ms | 9.85 | 14.29 | 9.60 |
| gpu/timed/ms | ms | 9.76 | 12.04 | 9.31 |
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
| phases/opaque3d/batch_sets | count | 33 | 35 | 33 |
| phases/opaque3d/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 28 | 29 | 28 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 33 | 35 | 33 |
| phases/opaque3d_prepass/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 28 | 29 | 28 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 43 | 50 | 43 |
| phases/shadow/bins | count | 2029 | 2336 | 2028 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5346 | 5666 | 5345 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 37 | 44 | 37 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 2 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 197 | 197 | 197 |
| pipelines/waiting | count | 0 | 14 | 0 |
| preprocess/bind_groups | count | 0 | 267 | 0 |
| preprocess/bind_groups_created | count | 0 | 267 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 269 | 269 | 269 |
| process/cpu_usage | % | 24.71 | 40.87 | 29.81 |
| process/mem_usage | GiB | 3.70 | 3.79 | 3.79 |
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
| statics/material_keys | count | 4 | 4 | 4 |
| statics/material_pipelines | count | 4 | 4 | 4 |
| statics/materials | count | 386 | 386 | 386 |
| statics/materials_bound | count | 386 | 386 | 386 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
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
| system/cpu_usage | % | 32.37 | 53.04 | 34.47 |
| system/mem_usage | % | 59.44 | 59.65 | 59.65 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 1490 | 1490 | 1490 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 123 | 130 | 123 |
| views/all/bins | count | 5199 | 5506 | 5198 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.9 k | 14.6 k |
| views/all/pipelines | count | 105 | 112 | 105 |
| views/all/pipelines_distinct | count | 93 | 95 | 93 |
| views/all/work_items | count | 14.8 k | 15.1 k | 14.8 k |
| views/cascade/batch_sets | count | 43 | 50 | 43 |
| views/cascade/bins | count | 2029 | 2336 | 2028 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5346 | 5666 | 5345 |
| views/cascade/pipelines | count | 37 | 44 | 37 |
| views/cascade/work_items | count | 5346 | 5666 | 5345 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 80 | 84 | 80 |
| views/main/bins | count | 3170 | 3172 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 68 | 70 | 68 |
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
| anim/events/count | count | 0.10 | 2 | 133 |
| assets/Image/added | count | 0.02 | 14 | 23 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.02 | 23 | 32 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.02 | 16 | 24 |
| assets/StandardMaterial/modified | count | 3.70 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.53 | 463 | 712 |
| physics/inserts | count | 0.00 | 1 | 3 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.67 | 1 | 900 |
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

## Hitches (> 27.8 ms; slowest 24, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.48 | 2482.9 | main/busy/ms 2458, surface/advance_cpu_ms 67.28, surface/cell_upload_ms 0.88, stream/drive/ms 0.38, weather/map/ms 0.16, bigspace/total/ms 0.10, stream/tick/ms 0.08, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 2.95 | 466.3 | main/busy/ms 2492, render/thread/ms 426, render/busy/ms 318, render/acquire/ms 105, surface/advance_cpu_ms 67.28, render/graph/ms 6.58, render/present/ms 3.16, stream/drive/ms 1.12, surface/cell_upload_ms 0.88, stream/tick/ms 0.57, terrain/update/ms 0.34, weather/map/ms 0.27 |
| warm-up | 3.16 | 210.5 | pipelines/wait_ms 1444, render/thread/ms 440, render/busy/ms 331, pipelines/compile_ms 223, pipelines/front_ms 210, render/acquire/ms 105, main/busy/ms 52.61, terrain/update/ms 17.79, pipelines/driver_ms 13.15, render/graph/ms 10.62, render/present/ms 4.02, stream/drive/ms 1.38 |
| warm-up | 3.22 | 28.0 | render/thread/ms 38.75, render/acquire/ms 20.49, main/busy/ms 18.76, render/busy/ms 16.28, terrain/update/ms 12.55, render/graph/ms 5.61, render/present/ms 1.99, stream/drive/ms 0.97, stream/tick/ms 0.79, terrain/select/ms 0.54, bigspace/total/ms 0.17, bigspace/high_precision/ms 0.09 |
| warm-up | 3.30 | 48.7 | render/thread/ms 71.13, render/busy/ms 43.07, main/busy/ms 32.75, render/acquire/ms 26.72, terrain/update/ms 4.24, render/graph/ms 3.89, stream/drive/ms 1.49, render/present/ms 1.34, stream/tick/ms 1.28, bigspace/total/ms 1.05, terrain/select/ms 0.43, bigspace/high_precision/ms 0.36 |
| warm-up | 3.41 | 96.0 | pipelines/compile_ms 227, pipelines/front_ms 213, main/busy/ms 74.58, render/thread/ms 72.68, render/busy/ms 69.85, pipelines/driver_ms 13.69, render/graph/ms 6.78, render/present/ms 2.10, terrain/update/ms 1.50, stream/drive/ms 1.05, terrain/select/ms 1.01, stream/tick/ms 0.82 |
| warm-up | 3.57 | 53.7 | render/thread/ms 44.50, main/busy/ms 43.98, render/busy/ms 43.29, stream/drive/ms 32.10, render/graph/ms 2.41, render/present/ms 1.00, stream/tick/ms 0.81, terrain/update/ms 0.60, terrain/select/ms 0.48, bigspace/total/ms 0.31, render/acquire/ms 0.21, bigspace/high_precision/ms 0.20 |
| warm-up | 3.61 | 30.6 | pipelines/wait_ms 2339, pipelines/compile_ms 194, pipelines/front_ms 181, main/busy/ms 35.71, render/thread/ms 26.05, render/busy/ms 24.41, stream/drive/ms 23.92, pipelines/driver_ms 12.71, render/graph/ms 3.82, render/present/ms 1.34, stream/tick/ms 0.67, bigspace/total/ms 0.65 |
| warm-up | 3.68 | 46.1 | pipelines/wait_ms 4078, pipelines/compile_ms 70.56, pipelines/front_ms 65.80, main/busy/ms 52.29, render/thread/ms 48.76, render/busy/ms 46.78, stream/drive/ms 38.21, pipelines/driver_ms 4.77, render/graph/ms 4.07, terrain/update/ms 2.07, render/present/ms 1.76, stream/spawn/ms 1.04 |
| warm-up | 3.72 | 35.3 | main/busy/ms 31.66, render/thread/ms 27.56, render/busy/ms 26.34, stream/drive/ms 18.41, stream/spawn/ms 4.61, render/graph/ms 3.71, render/present/ms 1.00, stream/tick/ms 0.95, terrain/update/ms 0.55, terrain/select/ms 0.45, bigspace/total/ms 0.30, render/acquire/ms 0.22 |
| warm-up | 3.77 | 29.9 | render/thread/ms 33.19, render/busy/ms 31.13, main/busy/ms 23.70, render/graph/ms 6.33, stream/drive/ms 5.30, stream/spawn/ms 2.24, render/present/ms 1.81, stream/tick/ms 1.46, terrain/update/ms 1.04, terrain/select/ms 0.84, stream/activate/ms 0.69, bigspace/total/ms 0.31 |
| warm-up | 3.83 | 41.2 | pipelines/wait_ms 453, pipelines/compile_ms 145, pipelines/front_ms 140, main/busy/ms 40.29, render/thread/ms 32.43, render/busy/ms 31.30, stream/drive/ms 26.55, pipelines/driver_ms 5.36, render/graph/ms 3.43, terrain/update/ms 0.96, render/present/ms 0.95, terrain/select/ms 0.75 |
| warm-up | 3.90 | 41.4 | pipelines/wait_ms 2036, pipelines/compile_ms 58.02, pipelines/front_ms 56.00, render/thread/ms 36.96, render/busy/ms 34.37, main/busy/ms 25.99, render/graph/ms 8.55, terrain/update/ms 5.37, stream/drive/ms 3.95, render/present/ms 2.27, pipelines/driver_ms 2.03, stream/spawn/ms 1.74 |
| warm-up | 3.95 | 37.3 | render/thread/ms 31.91, render/busy/ms 30.49, main/busy/ms 14.73, render/graph/ms 3.96, render/present/ms 1.21, stream/drive/ms 1.13, terrain/update/ms 0.83, stream/tick/ms 0.75, terrain/select/ms 0.63, bigspace/total/ms 0.26, render/acquire/ms 0.21, stream/activate/ms 0.16 |
| warm-up | 4.09 | 94.9 | pipelines/wait_ms 3008, main/busy/ms 97.47, stream/drive/ms 79.12, pipelines/compile_ms 27.65, pipelines/front_ms 25.58, render/thread/ms 24.41, render/busy/ms 22.86, render/graph/ms 3.56, stream/spawn/ms 2.26, pipelines/driver_ms 2.07, render/present/ms 1.35, stream/tick/ms 0.77 |
| warm-up | 4.30 | 37.8 | main/busy/ms 44.84, render/thread/ms 43.40, render/busy/ms 41.29, stream/drive/ms 14.17, render/graph/ms 6.63, stream/spawn/ms 4.69, render/present/ms 1.45, stream/tick/ms 0.86, terrain/update/ms 0.81, render/acquire/ms 0.66, terrain/select/ms 0.62, stream/activate/ms 0.62 |
| warm-up | 4.33 | 30.7 | render/thread/ms 46.00, render/busy/ms 44.29, main/busy/ms 34.77, stream/drive/ms 5.55, render/graph/ms 4.38, stream/spawn/ms 3.55, render/present/ms 1.04, stream/tick/ms 0.82, terrain/update/ms 0.82, render/acquire/ms 0.68, terrain/select/ms 0.63, bigspace/total/ms 0.48 |
| warm-up | 4.50 | 36.4 | pipelines/wait_ms 3300, pipelines/compile_ms 68.74, pipelines/front_ms 65.82, render/thread/ms 45.29, render/busy/ms 43.53, main/busy/ms 30.10, render/graph/ms 8.53, stream/drive/ms 4.89, pipelines/driver_ms 2.93, stream/spawn/ms 2.02, render/present/ms 1.50, terrain/update/ms 0.72 |
| warm-up | 4.57 | 51.7 | render/thread/ms 52.12, render/busy/ms 50.21, main/busy/ms 38.74, render/graph/ms 7.75, stream/drive/ms 3.81, stream/spawn/ms 2.30, render/present/ms 1.55, stream/tick/ms 0.74, bigspace/total/ms 0.61, terrain/update/ms 0.54, terrain/select/ms 0.46, render/acquire/ms 0.36 |
| warm-up | 4.64 | 49.5 | render/thread/ms 56.95, render/busy/ms 55.52, main/busy/ms 29.34, render/graph/ms 5.67, stream/drive/ms 4.56, stream/spawn/ms 2.10, stream/tick/ms 1.21, render/present/ms 1.16, bigspace/total/ms 0.68, terrain/update/ms 0.64, terrain/select/ms 0.53, bigspace/high_precision/ms 0.33 |
| warm-up | 4.79 | 35.0 | render/thread/ms 47.96, render/busy/ms 46.21, main/busy/ms 28.36, render/graph/ms 6.68, stream/drive/ms 4.41, stream/spawn/ms 2.62, render/present/ms 1.52, bigspace/total/ms 1.29, stream/tick/ms 0.97, bigspace/high_precision/ms 0.96, terrain/update/ms 0.50, terrain/select/ms 0.42 |
| warm-up | 5.26 | 34.7 | pipelines/wait_ms 1991, pipelines/compile_ms 86.43, pipelines/front_ms 80.89, render/thread/ms 40.41, render/busy/ms 38.77, main/busy/ms 36.07, stream/drive/ms 13.11, render/graph/ms 7.18, pipelines/driver_ms 5.54, stream/spawn/ms 2.36, stream/tick/ms 1.45, render/present/ms 1.42 |
| warm-up | 5.87 | 40.0 | main/busy/ms 44.80, render/thread/ms 33.74, render/busy/ms 32.34, render/graph/ms 10.77, stream/drive/ms 1.35, render/present/ms 1.21, stream/tick/ms 0.73, bigspace/total/ms 0.63, terrain/update/ms 0.51, terrain/select/ms 0.43, physics/insert/ms 0.38, far/select/ms 0.32 |
| warm-up | 6.45 | 36.6 | main/busy/ms 40.90, render/thread/ms 34.01, render/busy/ms 32.19, render/graph/ms 9.85, render/present/ms 1.63, bigspace/total/ms 1.40, stream/drive/ms 1.33, terrain/update/ms 0.62, anim/evaluate_cpu/ms 0.60, stream/tick/ms 0.58, terrain/select/ms 0.52, bigspace/low_precision/ms 0.52 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --texture-cap 512 --resolution 2560x1440 --bench-out target/perf\lean_cap512\balmora@2560x1440_--texture-cap_512-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **97%** (min 93%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.0 W · 77 °C (max 79) · VRAM 4731 MiB · throttled 100% of samples
