# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1271 frames over 15.0 s · **median 11.74 ms (85 fps)** · mean 11.80 · p95 13.23 · p99 14.29 (1% low 70 fps) · worst 18.34 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 455 frames over 9.1 s, worst 2468.8 ms
- hitches (> 29.4 ms): **0** measured, 27 in warm-up
- timed GPU passes: 9.58 ms of a 11.80 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.74 ms, limit 11 ms
- pass worst frame 18.34 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.075 | 3.099 | 3.681 | 0.99 | 0.078 |
| dlss_super_resolution | 2.264 | 2.282 | 2.791 | 0.99 | 0.199 |
| early prepass | 0.675 | 0.680 | 1.303 | 0.99 | 0.131 |
| atmosphere_luts | 0.375 | 0.378 | 0.888 | 0.99 | 0.004 |
| bloom | 0.350 | 0.352 | 0.498 | 0.99 | 0.070 |
| main_transmissive_pass_3d | 0.315 | 0.318 | 0.313 | 0.99 | 0.009 |
| water_sspr | 0.265 | 0.267 | 0.851 | 0.99 | 0.002 |
| water_velocity | 0.264 | 0.266 | 0.271 | 0.99 | 0.002 |
| uniform_allocation | 0.256 | 0.258 | 0.256 | 0.99 | 0.046 |
| ssao | 0.213 | 0.214 | 0.213 | 0.99 | 0.008 |
| bin_unpacking | 0.198 | 0.199 | 0.207 | 0.99 | 0.033 |
| atmos_froxel_inject | 0.159 | 0.160 | 0.153 | 0.99 | 0.002 |
| clouds_trace | 0.150 | 0.151 | 0.174 | 0.99 | 0.001 |
| water_fft | 0.150 | 0.151 | 0.140 | 0.99 | 0.003 |
| upscaling | 0.126 | 0.127 | 0.146 | 0.99 | 0.005 |
| clouds_composite | 0.118 | 0.119 | 0.117 | 0.99 | 0.002 |
| tonemapping | 0.092 | 0.093 | 0.091 | 0.99 | 0.004 |
| clustering | 0.071 | 0.071 | 0.076 | 0.99 | 0.023 |
| render_sky | 0.061 | 0.062 | 0.060 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.057 | 0.465 | 1.084 | 0.12 | 0.001 |
| clouds_resolve | 0.051 | 0.051 | 0.053 | 0.99 | 0.001 |
| water_snapshot | 0.042 | 0.042 | 0.056 | 0.99 | 0.002 |
| early_mesh_preprocessing | 0.041 | 0.041 | 0.042 | 0.99 | 0.006 |
| main_transparent_pass_3d | 0.034 | 0.034 | 0.043 | 0.99 | 0.076 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 0.99 | 0.004 |
| lightprobe_radiance_map | 0.031 | 0.248 | 0.239 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.030 | 0.99 | 0.001 |
| water_mips | 0.026 | 0.026 | 0.024 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.025 | 0.026 | 0.030 | 0.99 | 0.007 |
| atmosphere_environment | 0.013 | 0.013 | 0.013 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.005 | 0.005 | 0.009 | 0.99 | 0.002 |
| lightprobe_downsampling_first_pass | 0.002 | 0.018 | 0.014 | 0.12 | 0.001 |
| lightprobe_copy | 0.002 | 0.016 | 0.016 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 64.423 | 0.000 | 11700.09 | 81881.3 |
| render/thread/ms | 10.832 | 12.255 | 17.39 | 13767.6 |
| render/busy/ms | 9.460 | 10.726 | 15.11 | 12023.1 |
| main/busy/ms | 7.665 | 9.427 | 12.70 | 9742.5 |
| render/graph/ms | 5.853 | 6.891 | 10.99 | 7439.2 |
| pipelines/compile_ms | 1.009 | 0.000 | 141.94 | 1282.4 |
| pipelines/front_ms | 0.976 | 0.000 | 136.78 | 1240.2 |
| render/acquire/ms | 0.873 | 2.173 | 3.61 | 1109.7 |
| render/present/ms | 0.499 | 0.603 | 6.48 | 634.8 |
| bigspace/total/ms | 0.337 | 0.552 | 2.88 | 427.9 |
| terrain/update/ms | 0.229 | 0.288 | 0.49 | 290.6 |
| terrain/select/ms | 0.199 | 0.251 | 0.47 | 252.8 |
| stream/drive/ms | 0.193 | 0.295 | 0.61 | 245.4 |
| far/select/ms | 0.177 | 0.273 | 0.47 | 225.2 |
| stream/tick/ms | 0.165 | 0.255 | 0.50 | 209.4 |
| controller/feed/ms | 0.162 | 0.220 | 0.46 | 206.5 |
| controller/move/ms | 0.115 | 0.182 | 0.32 | 146.3 |
| bigspace/high_precision/ms | 0.113 | 0.250 | 2.71 | 143.7 |
| anim/evaluate/ms | 0.083 | 0.217 | 0.87 | 105.9 |
| bigspace/low_precision/ms | 0.078 | 0.175 | 2.31 | 99.0 |
| anim/evaluate_cpu/ms | 0.062 | 0.077 | 0.32 | 78.5 |
| physics/step/ms | 0.038 | 0.066 | 0.41 | 47.9 |
| pipelines/driver_ms | 0.033 | 0.000 | 5.16 | 42.2 |
| weather/map/ms | 0.026 | 0.035 | 0.18 | 32.6 |
| weather/eval/ms | 0.012 | 0.015 | 0.22 | 15.0 |
| script/update/ms | 0.007 | 0.011 | 0.05 | 9.5 |
| controller/avoid/ms | 0.006 | 0.009 | 0.12 | 8.3 |
| physics/insert/ms | 0.006 | 0.015 | 0.54 | 7.0 |
| script/input/ms | 0.005 | 0.006 | 0.16 | 6.7 |
| script/ui/ms | 0.005 | 0.007 | 0.02 | 6.4 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 5.4 |
| controller/write/ms | 0.002 | 0.003 | 0.11 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.07 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.05 | 0.4 |
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
| anim/evaluated/count | count | 7 | 23 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 10 | 8 |
| crowd/c0/joints/count | count | 272 | 340 | 272 |
| crowd/c0/meshes/count | count | 48 | 60 | 48 |
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1080 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 6134 | 4212 |
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
| frame/work/ms | ms | 10.30 | 15.11 | 10.26 |
| gpu/timed/ms | ms | 10.17 | 12.16 | 10.26 |
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
| phases/opaque3d/batch_sets | count | 35 | 37 | 35 |
| phases/opaque3d/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 28 | 29 | 28 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 35 | 37 | 35 |
| phases/opaque3d_prepass/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 28 | 29 | 28 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 43 | 50 | 43 |
| phases/shadow/bins | count | 2029 | 2336 | 2029 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5346 | 5672 | 5346 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 37 | 44 | 37 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 242 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 197 | 197 | 197 |
| pipelines/waiting | count | 0 | 19 | 0 |
| preprocess/bind_groups | count | 0 | 222 | 0 |
| preprocess/bind_groups_created | count | 0 | 222 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 277 | 277 | 277 |
| process/cpu_usage | % | 30.20 | 42.38 | 31.88 |
| process/mem_usage | GiB | 3.82 | 3.91 | 3.89 |
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
| system/cpu_usage | % | 38.00 | 62.38 | 34.27 |
| system/mem_usage | % | 64.82 | 65.21 | 65.11 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 1490 | 1490 | 1490 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 127 | 134 | 127 |
| views/all/bins | count | 5199 | 5506 | 5199 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 15.0 k | 14.6 k |
| views/all/pipelines | count | 105 | 112 | 105 |
| views/all/pipelines_distinct | count | 93 | 95 | 93 |
| views/all/work_items | count | 14.8 k | 15.1 k | 14.8 k |
| views/cascade/batch_sets | count | 43 | 50 | 43 |
| views/cascade/bins | count | 2029 | 2336 | 2029 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5346 | 5672 | 5346 |
| views/cascade/pipelines | count | 37 | 44 | 37 |
| views/cascade/work_items | count | 5346 | 5672 | 5346 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 84 | 88 | 84 |
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
| anim/bones_written/count | count | 0.08 | 99 | 99 |
| anim/events/count | count | 0.10 | 3 | 129 |
| assets/Image/added | count | 0.01 | 12 | 16 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.06 | 40 | 73 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.01 | 13 | 18 |
| assets/StandardMaterial/modified | count | 3.89 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.93 | 659 | 1186 |
| physics/inserts | count | 0.13 | 50 | 165 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.71 | 1 | 900 |
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

## Hitches (> 29.4 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2468.8 | main/busy/ms 2443, surface/advance_cpu_ms 72.78, surface/cell_upload_ms 0.69, stream/drive/ms 0.32, weather/map/ms 0.18, bigspace/total/ms 0.09, stream/tick/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01, far/select/ms 0.01 |
| warm-up | 2.93 | 464.6 | main/busy/ms 2486, render/thread/ms 422, render/busy/ms 315, render/acquire/ms 103, surface/advance_cpu_ms 72.78, render/graph/ms 6.96, render/present/ms 4.31, stream/drive/ms 1.10, surface/cell_upload_ms 0.69, stream/tick/ms 0.65, terrain/update/ms 0.35, weather/map/ms 0.28 |
| warm-up | 3.15 | 220.9 | pipelines/wait_ms 1204, render/thread/ms 438, render/busy/ms 329, pipelines/compile_ms 230, pipelines/front_ms 213, render/acquire/ms 103, main/busy/ms 63.50, terrain/update/ms 25.25, pipelines/driver_ms 16.58, render/graph/ms 10.45, render/present/ms 5.37, stream/drive/ms 1.15 |
| warm-up | 3.22 | 30.6 | render/thread/ms 35.01, main/busy/ms 18.31, render/acquire/ms 16.67, render/busy/ms 15.98, terrain/update/ms 9.22, render/graph/ms 5.13, render/present/ms 2.36, stream/drive/ms 1.00, stream/tick/ms 0.81, terrain/select/ms 0.43, bigspace/total/ms 0.19, bigspace/high_precision/ms 0.10 |
| warm-up | 3.29 | 49.3 | render/thread/ms 72.04, render/busy/ms 41.16, main/busy/ms 38.17, render/acquire/ms 29.14, terrain/update/ms 4.59, render/graph/ms 4.23, stream/drive/ms 2.25, stream/tick/ms 1.97, render/present/ms 1.74, bigspace/total/ms 0.71, bigspace/high_precision/ms 0.55, terrain/select/ms 0.42 |
| warm-up | 3.35 | 31.5 | main/busy/ms 66.75, render/thread/ms 52.13, render/busy/ms 32.97, render/acquire/ms 17.30, render/graph/ms 4.23, render/present/ms 1.86, stream/drive/ms 1.31, terrain/update/ms 1.29, stream/tick/ms 1.10, terrain/select/ms 1.02, bigspace/total/ms 0.61, bigspace/high_precision/ms 0.27 |
| warm-up | 3.45 | 103.7 | pipelines/compile_ms 279, pipelines/front_ms 254, render/thread/ms 106, render/busy/ms 91.74, main/busy/ms 73.45, pipelines/driver_ms 25.11, render/acquire/ms 11.98, render/graph/ms 5.05, render/present/ms 1.92, stream/drive/ms 1.55, stream/tick/ms 1.16, terrain/update/ms 0.61 |
| warm-up | 3.49 | 30.4 | pipelines/wait_ms 254, pipelines/compile_ms 55.80, pipelines/front_ms 51.87, render/thread/ms 35.74, render/busy/ms 34.55, main/busy/ms 14.00, pipelines/driver_ms 3.93, render/graph/ms 3.41, stream/drive/ms 1.61, render/present/ms 0.97, terrain/update/ms 0.95, terrain/select/ms 0.70 |
| warm-up | 3.59 | 68.6 | render/thread/ms 42.43, render/busy/ms 41.16, main/busy/ms 41.04, stream/drive/ms 28.63, render/graph/ms 2.39, render/present/ms 1.04, stream/tick/ms 0.67, terrain/update/ms 0.48, terrain/select/ms 0.38, stream/spawn/ms 0.33, render/acquire/ms 0.23, bigspace/total/ms 0.22 |
| warm-up | 3.65 | 48.4 | pipelines/wait_ms 4251, pipelines/compile_ms 212, pipelines/front_ms 192, render/thread/ms 38.84, render/busy/ms 36.81, main/busy/ms 33.52, pipelines/driver_ms 19.58, stream/drive/ms 19.00, render/graph/ms 5.94, render/present/ms 1.76, stream/spawn/ms 1.19, terrain/update/ms 0.95 |
| warm-up | 3.74 | 73.2 | pipelines/wait_ms 478, render/thread/ms 69.04, render/busy/ms 66.89, main/busy/ms 36.26, pipelines/compile_ms 17.51, pipelines/front_ms 16.87, stream/drive/ms 11.18, render/graph/ms 6.75, terrain/update/ms 2.72, stream/spawn/ms 2.00, render/present/ms 1.85, stream/tick/ms 1.71 |
| warm-up | 3.77 | 31.2 | render/thread/ms 77.25, render/busy/ms 75.26, main/busy/ms 49.22, stream/drive/ms 29.49, render/graph/ms 7.02, terrain/update/ms 2.80, render/present/ms 1.71, stream/tick/ms 1.36, terrain/select/ms 0.56, stream/activate/ms 0.33, render/acquire/ms 0.28, bigspace/total/ms 0.26 |
| warm-up | 3.89 | 120.2 | pipelines/wait_ms 6324, pipelines/compile_ms 203, pipelines/front_ms 191, render/thread/ms 108, render/busy/ms 71.54, main/busy/ms 49.99, render/present/ms 36.04, stream/drive/ms 31.92, pipelines/driver_ms 11.93, render/graph/ms 3.26, stream/spawn/ms 2.28, stream/activate/ms 1.28 |
| warm-up | 3.95 | 35.9 | pipelines/wait_ms 1348, pipelines/compile_ms 67.01, pipelines/driver_ms 46.55, main/busy/ms 37.60, render/thread/ms 37.58, render/busy/ms 35.99, pipelines/front_ms 20.46, stream/drive/ms 12.08, terrain/update/ms 4.29, render/graph/ms 3.80, stream/spawn/ms 1.48, stream/tick/ms 1.42 |
| warm-up | 4.00 | 35.4 | main/busy/ms 32.71, render/thread/ms 27.03, render/busy/ms 25.77, stream/drive/ms 19.68, render/graph/ms 3.04, render/present/ms 0.99, stream/tick/ms 0.93, stream/activate/ms 0.85, terrain/update/ms 0.66, terrain/select/ms 0.54, physics/insert/ms 0.38, render/acquire/ms 0.26 |
| warm-up | 4.05 | 38.6 | pipelines/wait_ms 1284, pipelines/compile_ms 176, pipelines/front_ms 162, main/busy/ms 44.39, stream/drive/ms 29.88, render/thread/ms 17.65, render/busy/ms 16.21, pipelines/driver_ms 13.48, render/graph/ms 3.35, stream/spawn/ms 2.30, render/present/ms 1.12, terrain/update/ms 0.68 |
| warm-up | 4.13 | 66.9 | pipelines/wait_ms 2376, main/busy/ms 69.84, pipelines/compile_ms 59.36, pipelines/front_ms 56.45, stream/drive/ms 50.31, render/thread/ms 21.34, render/busy/ms 20.25, pipelines/driver_ms 2.91, render/graph/ms 2.53, stream/spawn/ms 1.83, render/present/ms 0.91, stream/tick/ms 0.70 |
| warm-up | 4.18 | 41.4 | main/busy/ms 46.47, stream/drive/ms 33.99, render/thread/ms 23.20, render/busy/ms 22.03, render/graph/ms 4.67, stream/activate/ms 1.04, render/present/ms 0.91, stream/spawn/ms 0.79, stream/tick/ms 0.70, terrain/update/ms 0.47, terrain/select/ms 0.37, physics/insert/ms 0.27 |
| warm-up | 4.36 | 30.3 | render/thread/ms 26.73, main/busy/ms 25.40, render/busy/ms 25.31, stream/drive/ms 10.74, render/graph/ms 3.24, stream/spawn/ms 2.81, render/present/ms 1.24, stream/tick/ms 0.73, terrain/update/ms 0.60, terrain/select/ms 0.49, bigspace/total/ms 0.40, render/acquire/ms 0.18 |
| warm-up | 4.54 | 37.5 | pipelines/wait_ms 2295, pipelines/compile_ms 63.41, pipelines/front_ms 60.58, render/thread/ms 45.61, render/busy/ms 44.26, main/busy/ms 17.68, render/graph/ms 5.22, stream/drive/ms 3.80, pipelines/driver_ms 2.84, stream/spawn/ms 2.15, render/present/ms 1.01, stream/tick/ms 0.91 |
| warm-up | 4.58 | 30.7 | pipelines/wait_ms 1393, render/thread/ms 37.50, render/busy/ms 36.39, main/busy/ms 20.81, stream/drive/ms 3.86, render/graph/ms 3.83, stream/spawn/ms 2.27, pipelines/compile_ms 1.58, pipelines/front_ms 1.27, render/present/ms 0.90, bigspace/total/ms 0.73, stream/tick/ms 0.67 |
| warm-up | 4.65 | 29.7 | pipelines/wait_ms 12.3 k, pipelines/compile_ms 59.04, pipelines/front_ms 50.29, render/thread/ms 47.88, render/busy/ms 46.60, main/busy/ms 30.69, pipelines/driver_ms 8.75, render/graph/ms 4.60, stream/drive/ms 3.50, stream/spawn/ms 1.59, render/present/ms 1.04, stream/tick/ms 0.77 |
| warm-up | 4.69 | 37.3 | pipelines/wait_ms 11.4 k, render/thread/ms 58.70, render/busy/ms 57.14, pipelines/compile_ms 45.08, main/busy/ms 42.45, pipelines/front_ms 40.77, stream/drive/ms 7.43, render/graph/ms 6.09, terrain/update/ms 4.48, pipelines/driver_ms 4.31, stream/spawn/ms 2.71, render/present/ms 1.23 |
| warm-up | 4.76 | 40.4 | pipelines/wait_ms 3029, pipelines/compile_ms 73.35, pipelines/front_ms 71.69, main/busy/ms 51.35, render/thread/ms 47.86, render/busy/ms 46.13, render/graph/ms 6.59, stream/drive/ms 3.70, stream/spawn/ms 2.43, pipelines/driver_ms 1.66, render/present/ms 1.50, physics/insert/ms 1.05 |
| warm-up | 5.21 | 43.2 | pipelines/wait_ms 8590, pipelines/compile_ms 129, pipelines/front_ms 119, render/thread/ms 45.11, render/busy/ms 43.29, main/busy/ms 41.95, stream/drive/ms 13.18, pipelines/driver_ms 10.20, render/graph/ms 7.40, stream/spawn/ms 2.42, render/present/ms 1.54, stream/tick/ms 1.18 |
| warm-up | 5.45 | 35.0 | main/busy/ms 46.34, render/thread/ms 43.78, render/busy/ms 41.57, render/graph/ms 7.42, render/present/ms 1.92, stream/drive/ms 1.72, stream/tick/ms 0.86, bigspace/total/ms 0.78, terrain/update/ms 0.54, terrain/select/ms 0.47, stream/spawn/ms 0.41, anim/evaluate_cpu/ms 0.31 |
| warm-up | 9.07 | 34.8 | pipelines/wait_ms 5834, main/busy/ms 44.86, render/thread/ms 43.08, render/busy/ms 41.62, pipelines/compile_ms 39.21, pipelines/front_ms 36.99, render/graph/ms 14.79, terrain/update/ms 6.38, pipelines/driver_ms 2.22, render/present/ms 1.27, physics/insert/ms 1.03, bigspace/total/ms 0.89 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --texture-cap 512 --resolution 2560x1440 --bench-out target/perf\lean_cap512\balmora@2560x1440_--texture-cap_512-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 8%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 90%) · 2580 MHz (min 2580) · memory 8001 MHz · 91.6 W · 77 °C (max 80) · VRAM 4717 MiB · throttled 100% of samples
