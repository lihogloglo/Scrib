# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1428 frames over 15.0 s · **median 10.42 ms (96 fps)** · mean 10.51 · p95 11.76 · p99 12.95 (1% low 77 fps) · worst 23.60 ms · 3 spikes over 1.5× median · 12.0 stutters/min (work 0.0/min)
- warm-up: 477 frames over 9.5 s, worst 2721.7 ms
- hitches (> 26.0 ms): **0** measured, 31 in warm-up
- timed GPU passes: 8.61 ms of a 10.51 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.42 ms, limit 11 ms
- pass worst frame 23.60 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.584 | 2.598 | 2.995 | 0.99 | 0.062 |
| dlss_super_resolution | 2.173 | 2.186 | 2.458 | 0.99 | 0.188 |
| early prepass | 0.611 | 0.614 | 0.946 | 0.99 | 0.095 |
| atmosphere_luts | 0.342 | 0.344 | 0.635 | 0.99 | 0.004 |
| bloom | 0.332 | 0.334 | 0.347 | 0.99 | 0.065 |
| main_transmissive_pass_3d | 0.285 | 0.287 | 0.301 | 0.99 | 0.008 |
| water_velocity | 0.254 | 0.256 | 0.272 | 0.99 | 0.002 |
| water_sspr | 0.230 | 0.231 | 0.248 | 0.99 | 0.002 |
| ssao | 0.205 | 0.207 | 0.216 | 0.99 | 0.008 |
| uniform_allocation | 0.178 | 0.179 | 0.183 | 0.99 | 0.034 |
| clouds_trace | 0.153 | 0.154 | 0.190 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.153 | 0.153 | 0.163 | 0.99 | 0.002 |
| water_fft | 0.139 | 0.140 | 0.139 | 0.99 | 0.003 |
| bin_unpacking | 0.137 | 0.137 | 0.142 | 0.99 | 0.024 |
| upscaling | 0.120 | 0.121 | 0.125 | 0.99 | 0.004 |
| clouds_composite | 0.117 | 0.118 | 0.117 | 0.99 | 0.002 |
| tonemapping | 0.091 | 0.091 | 0.097 | 0.99 | 0.003 |
| clustering | 0.069 | 0.070 | 0.080 | 0.99 | 0.022 |
| render_sky | 0.055 | 0.055 | 0.056 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.430 | 0.423 | 0.12 | 0.001 |
| clouds_resolve | 0.052 | 0.052 | 0.054 | 0.99 | 0.001 |
| water_snapshot | 0.041 | 0.041 | 0.056 | 0.99 | 0.002 |
| early_mesh_preprocessing | 0.039 | 0.039 | 0.041 | 0.99 | 0.007 |
| clouds_shadow | 0.031 | 0.032 | 0.035 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.233 | 0.239 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.031 | 0.99 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.022 | 0.022 | 0.024 | 0.99 | 0.075 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.029 | 0.99 | 0.008 |
| atmosphere_environment | 0.012 | 0.012 | 0.014 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.008 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.018 | 0.017 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.015 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 17.934 | 0.000 | 4390.92 | 25609.1 |
| render/thread/ms | 9.586 | 10.829 | 22.49 | 13689.5 |
| render/busy/ms | 7.462 | 9.306 | 11.96 | 10655.9 |
| main/busy/ms | 6.986 | 8.266 | 12.29 | 9976.2 |
| render/graph/ms | 4.273 | 5.785 | 7.72 | 6101.3 |
| render/acquire/ms | 1.647 | 2.759 | 3.80 | 2352.4 |
| render/present/ms | 0.477 | 0.569 | 12.29 | 681.2 |
| bigspace/total/ms | 0.383 | 0.699 | 1.88 | 547.5 |
| pipelines/compile_ms | 0.285 | 0.000 | 125.53 | 407.1 |
| pipelines/front_ms | 0.277 | 0.000 | 122.32 | 394.9 |
| terrain/update/ms | 0.230 | 0.296 | 0.58 | 328.9 |
| terrain/select/ms | 0.199 | 0.256 | 0.53 | 284.9 |
| stream/drive/ms | 0.178 | 0.227 | 0.45 | 254.0 |
| controller/feed/ms | 0.167 | 0.227 | 0.46 | 238.0 |
| far/select/ms | 0.155 | 0.212 | 0.40 | 221.6 |
| stream/tick/ms | 0.152 | 0.197 | 0.41 | 217.1 |
| bigspace/high_precision/ms | 0.133 | 0.290 | 1.47 | 190.4 |
| controller/move/ms | 0.112 | 0.172 | 0.31 | 160.4 |
| bigspace/low_precision/ms | 0.089 | 0.215 | 1.55 | 126.6 |
| anim/evaluate/ms | 0.081 | 0.210 | 1.21 | 115.9 |
| anim/evaluate_cpu/ms | 0.067 | 0.090 | 0.31 | 95.9 |
| physics/step/ms | 0.031 | 0.059 | 0.18 | 43.9 |
| weather/map/ms | 0.024 | 0.033 | 0.06 | 34.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.04 | 15.3 |
| pipelines/driver_ms | 0.009 | 0.000 | 3.62 | 12.2 |
| script/update/ms | 0.007 | 0.009 | 0.05 | 10.2 |
| controller/avoid/ms | 0.006 | 0.007 | 0.14 | 8.6 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 7.4 |
| anim/apply/ms | 0.005 | 0.006 | 0.26 | 6.7 |
| script/ui/ms | 0.004 | 0.006 | 0.05 | 5.6 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| anim/evaluated/count | count | 7 | 27 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 15 | 8 |
| crowd/c0/joints/count | count | 272 | 510 | 272 |
| crowd/c0/meshes/count | count | 48 | 90 | 48 |
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4894 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1090 | 1088 |
| crowd/c2/count | count | 283 | 283 | 283 |
| crowd/c2/joints/count | count | 4916 | 4916 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1373 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 7714 | 4212 |
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
| frame/work/ms | ms | 9.25 | 12.29 | 9.35 |
| gpu/timed/ms | ms | 9.23 | 11.26 | 9.35 |
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
| phases/shadow/batch_sets | count | 31 | 36 | 31 |
| phases/shadow/bins | count | 2029 | 2336 | 2029 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5346 | 5654 | 5346 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 31 | 36 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 182 | 182 | 182 |
| pipelines/waiting | count | 0 | 6 | 0 |
| preprocess/bind_groups | count | 0 | 122 | 0 |
| preprocess/bind_groups_created | count | 0 | 122 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 201 | 201 | 201 |
| process/cpu_usage | % | 25.36 | 41.83 | 21.15 |
| process/mem_usage | GiB | 4.32 | 4.42 | 4.39 |
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
| system/cpu_usage | % | 31.93 | 61.16 | 26.47 |
| system/mem_usage | % | 73.18 | 73.45 | 73.31 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2480 | 2480 | 2480 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 89 | 94 | 89 |
| views/all/bins | count | 5199 | 5432 | 5199 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.7 k | 14.6 k |
| views/all/pipelines | count | 85 | 90 | 85 |
| views/all/pipelines_distinct | count | 75 | 79 | 75 |
| views/all/work_items | count | 14.8 k | 14.9 k | 14.8 k |
| views/cascade/batch_sets | count | 31 | 36 | 31 |
| views/cascade/bins | count | 2029 | 2336 | 2029 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5346 | 5654 | 5346 |
| views/cascade/pipelines | count | 31 | 36 | 31 |
| views/cascade/work_items | count | 5346 | 5654 | 5346 |
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
| anim/bones_written/count | count | 0.14 | 198 | 198 |
| anim/events/count | count | 0.08 | 2 | 119 |
| assets/Image/added | count | 0.01 | 19 | 19 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.02 | 35 | 35 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.02 | 23 | 23 |
| assets/StandardMaterial/modified | count | 3.47 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.81 | 908 | 1162 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.63 | 1 | 901 |
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
| surface/shelter_redraws | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.0 ms; slowest 31, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.72 | 2721.7 | main/busy/ms 2697, surface/advance_cpu_ms 81.82, surface/cell_upload_ms 0.75, stream/drive/ms 0.30, weather/map/ms 0.27, bigspace/total/ms 0.09, stream/tick/ms 0.06, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, far/select/ms 0.03, script/update/ms 0.03, anim/evaluate/ms 0.01 |
| warm-up | 3.73 | 1003.4 | main/busy/ms 2732, render/thread/ms 964, render/busy/ms 846, render/acquire/ms 116, surface/advance_cpu_ms 81.82, render/graph/ms 5.32, render/present/ms 2.00, stream/drive/ms 0.88, surface/cell_upload_ms 0.75, stream/tick/ms 0.45, terrain/update/ms 0.38, weather/map/ms 0.35 |
| warm-up | 3.98 | 99.3 | main/busy/ms 66.89, render/thread/ms 57.50, render/busy/ms 55.08, render/graph/ms 8.18, stream/drive/ms 2.17, render/present/ms 2.10, stream/tick/ms 1.88, terrain/update/ms 0.56, terrain/select/ms 0.44, render/acquire/ms 0.33, bigspace/total/ms 0.31, weather/map/ms 0.09 |
| warm-up | 4.02 | 29.5 | pipelines/compile_ms 256, pipelines/front_ms 245, render/thread/ms 35.46, render/busy/ms 33.70, main/busy/ms 19.00, pipelines/driver_ms 10.91, render/graph/ms 3.76, stream/drive/ms 2.42, render/present/ms 1.57, stream/spawn/ms 1.17, terrain/update/ms 1.09, terrain/select/ms 0.87 |
| warm-up | 4.13 | 32.0 | render/thread/ms 48.66, render/busy/ms 46.74, main/busy/ms 45.94, stream/drive/ms 32.55, render/graph/ms 6.70, render/present/ms 1.72, stream/tick/ms 0.91, terrain/update/ms 0.65, terrain/select/ms 0.50, stream/spawn/ms 0.38, bigspace/total/ms 0.24, render/acquire/ms 0.20 |
| warm-up | 4.16 | 32.5 | render/thread/ms 50.79, render/busy/ms 49.03, main/busy/ms 36.63, stream/drive/ms 24.30, render/graph/ms 5.90, render/present/ms 1.53, terrain/update/ms 0.73, stream/tick/ms 0.70, terrain/select/ms 0.56, bigspace/total/ms 0.24, render/acquire/ms 0.23, stream/spawn/ms 0.21 |
| warm-up | 4.24 | 61.4 | pipelines/wait_ms 2993, pipelines/compile_ms 279, pipelines/front_ms 252, render/thread/ms 48.76, render/busy/ms 46.73, main/busy/ms 32.52, pipelines/driver_ms 26.44, stream/drive/ms 14.14, render/graph/ms 6.39, render/present/ms 1.74, stream/spawn/ms 1.19, stream/tick/ms 0.97 |
| warm-up | 4.27 | 36.5 | pipelines/wait_ms 3228, pipelines/compile_ms 91.05, pipelines/front_ms 79.63, render/thread/ms 71.82, render/busy/ms 69.33, main/busy/ms 33.01, pipelines/driver_ms 11.42, stream/drive/ms 9.26, render/graph/ms 8.91, stream/spawn/ms 2.14, render/present/ms 2.12, stream/tick/ms 0.86 |
| warm-up | 4.37 | 94.3 | pipelines/wait_ms 479, render/thread/ms 96.10, render/busy/ms 94.19, main/busy/ms 48.53, stream/drive/ms 30.04, pipelines/compile_ms 8.27, pipelines/front_ms 7.73, render/graph/ms 5.58, stream/spawn/ms 1.76, render/present/ms 1.62, terrain/update/ms 0.71, stream/tick/ms 0.60 |
| warm-up | 4.42 | 52.5 | pipelines/wait_ms 4003, pipelines/compile_ms 165, pipelines/front_ms 155, render/thread/ms 102, render/busy/ms 100, main/busy/ms 60.87, stream/drive/ms 43.61, pipelines/driver_ms 10.11, render/graph/ms 4.65, stream/spawn/ms 2.16, render/present/ms 1.32, stream/tick/ms 0.74 |
| warm-up | 4.47 | 28.1 | pipelines/wait_ms 1763, pipelines/compile_ms 38.12, render/thread/ms 35.61, pipelines/front_ms 34.95, render/busy/ms 34.39, main/busy/ms 28.24, stream/drive/ms 10.11, pipelines/driver_ms 3.17, render/graph/ms 3.07, stream/spawn/ms 1.07, render/present/ms 0.94, stream/tick/ms 0.89 |
| warm-up | 4.53 | 41.3 | pipelines/wait_ms 4717, pipelines/compile_ms 62.51, pipelines/front_ms 57.40, main/busy/ms 42.40, stream/drive/ms 27.16, render/thread/ms 20.18, render/busy/ms 18.40, pipelines/driver_ms 5.11, render/graph/ms 4.01, terrain/update/ms 2.81, render/present/ms 1.57, stream/tick/ms 0.84 |
| warm-up | 4.57 | 27.2 | pipelines/wait_ms 2885, main/busy/ms 27.95, pipelines/compile_ms 25.44, pipelines/front_ms 23.31, render/thread/ms 22.59, render/busy/ms 21.42, stream/drive/ms 15.56, render/graph/ms 4.84, pipelines/driver_ms 2.13, render/present/ms 0.96, stream/tick/ms 0.81, terrain/update/ms 0.75 |
| warm-up | 4.72 | 97.6 | pipelines/wait_ms 8818, pipelines/compile_ms 146, pipelines/front_ms 136, main/busy/ms 97.70, stream/drive/ms 79.05, render/thread/ms 32.97, render/busy/ms 31.74, pipelines/driver_ms 10.19, render/graph/ms 4.03, stream/spawn/ms 2.04, render/present/ms 1.05, terrain/update/ms 0.98 |
| warm-up | 4.91 | 29.0 | pipelines/wait_ms 1462, pipelines/compile_ms 140, pipelines/front_ms 136, render/thread/ms 33.37, render/busy/ms 31.84, main/busy/ms 28.05, stream/drive/ms 10.08, render/graph/ms 5.92, pipelines/driver_ms 4.29, stream/spawn/ms 2.36, render/present/ms 1.32, stream/tick/ms 0.83 |
| warm-up | 4.95 | 26.4 | pipelines/wait_ms 3226, pipelines/compile_ms 43.75, pipelines/front_ms 40.90, render/thread/ms 32.68, render/busy/ms 31.55, main/busy/ms 22.21, stream/drive/ms 4.27, render/graph/ms 3.81, pipelines/driver_ms 2.85, stream/spawn/ms 2.36, stream/tick/ms 1.12, render/present/ms 0.88 |
| warm-up | 5.09 | 27.9 | render/thread/ms 35.27, render/busy/ms 33.64, main/busy/ms 20.80, render/graph/ms 5.24, stream/drive/ms 4.74, stream/spawn/ms 2.09, render/present/ms 1.36, bigspace/total/ms 0.79, stream/tick/ms 0.64, terrain/update/ms 0.57, terrain/select/ms 0.46, bigspace/high_precision/ms 0.33 |
| warm-up | 5.13 | 28.3 | pipelines/wait_ms 933, pipelines/compile_ms 133, pipelines/front_ms 124, render/thread/ms 43.66, render/busy/ms 42.06, main/busy/ms 27.00, pipelines/driver_ms 8.34, render/graph/ms 5.83, stream/drive/ms 3.82, stream/spawn/ms 2.05, render/present/ms 1.32, stream/tick/ms 0.77 |
| warm-up | 5.16 | 27.1 | render/thread/ms 48.83, render/busy/ms 47.21, main/busy/ms 23.78, render/graph/ms 5.37, stream/drive/ms 4.09, stream/spawn/ms 2.03, render/present/ms 1.33, stream/tick/ms 0.94, bigspace/total/ms 0.86, terrain/update/ms 0.62, bigspace/high_precision/ms 0.51, terrain/select/ms 0.50 |
| warm-up | 5.19 | 32.2 | render/thread/ms 47.01, render/busy/ms 45.87, main/busy/ms 23.79, stream/drive/ms 4.01, render/graph/ms 3.97, stream/spawn/ms 2.09, render/present/ms 0.93, stream/tick/ms 0.85, bigspace/total/ms 0.73, terrain/update/ms 0.66, terrain/select/ms 0.52, bigspace/high_precision/ms 0.40 |
| warm-up | 5.27 | 36.6 | pipelines/wait_ms 3880, pipelines/compile_ms 163, pipelines/front_ms 157, render/thread/ms 50.95, render/busy/ms 49.04, main/busy/ms 30.50, render/graph/ms 7.15, stream/drive/ms 6.33, pipelines/driver_ms 5.46, stream/spawn/ms 2.23, render/present/ms 1.68, bigspace/total/ms 0.97 |
| warm-up | 5.31 | 33.3 | pipelines/wait_ms 1479, render/thread/ms 61.09, render/busy/ms 58.45, main/busy/ms 48.47, pipelines/compile_ms 12.83, pipelines/front_ms 11.80, render/graph/ms 10.22, stream/drive/ms 5.91, render/present/ms 2.34, stream/spawn/ms 2.24, pipelines/driver_ms 1.03, bigspace/total/ms 0.91 |
| warm-up | 5.36 | 49.7 | render/thread/ms 62.54, render/busy/ms 59.92, main/busy/ms 49.02, render/graph/ms 10.59, stream/drive/ms 3.53, render/present/ms 2.31, stream/spawn/ms 2.08, terrain/update/ms 0.93, bigspace/total/ms 0.76, terrain/select/ms 0.72, stream/tick/ms 0.69, bigspace/high_precision/ms 0.40 |
| warm-up | 5.39 | 34.8 | render/thread/ms 57.53, render/busy/ms 55.31, main/busy/ms 44.60, stream/drive/ms 14.18, render/graph/ms 9.21, stream/spawn/ms 2.11, render/present/ms 1.88, terrain/update/ms 0.86, bigspace/total/ms 0.77, stream/tick/ms 0.70, terrain/select/ms 0.68, bigspace/high_precision/ms 0.37 |
| warm-up | 5.44 | 49.6 | pipelines/wait_ms 3944, pipelines/compile_ms 186, pipelines/front_ms 180, render/thread/ms 57.87, render/busy/ms 56.37, main/busy/ms 40.85, stream/drive/ms 14.13, render/graph/ms 6.51, pipelines/driver_ms 5.84, stream/spawn/ms 2.14, render/present/ms 1.21, bigspace/total/ms 1.14 |
| warm-up | 5.50 | 38.2 | main/busy/ms 52.04, render/thread/ms 40.23, render/busy/ms 38.60, render/graph/ms 6.89, stream/drive/ms 6.31, stream/spawn/ms 2.10, render/present/ms 1.39, terrain/update/ms 0.81, bigspace/total/ms 0.67, terrain/select/ms 0.64, stream/tick/ms 0.62, bigspace/high_precision/ms 0.34 |
| warm-up | 5.53 | 30.9 | main/busy/ms 46.36, render/thread/ms 43.52, render/busy/ms 42.27, stream/drive/ms 6.90, render/graph/ms 4.69, stream/spawn/ms 2.42, render/present/ms 1.04, bigspace/total/ms 0.98, bigspace/high_precision/ms 0.74, stream/tick/ms 0.71, terrain/update/ms 0.66, terrain/select/ms 0.52 |
| warm-up | 5.61 | 26.3 | main/busy/ms 29.75, render/thread/ms 22.40, render/busy/ms 21.13, stream/drive/ms 16.32, render/graph/ms 6.19, stream/spawn/ms 1.33, render/present/ms 0.99, stream/tick/ms 0.83, bigspace/total/ms 0.47, terrain/update/ms 0.45, terrain/select/ms 0.38, render/acquire/ms 0.27 |
| warm-up | 5.89 | 43.2 | main/busy/ms 40.22, render/thread/ms 36.57, render/busy/ms 34.77, stream/drive/ms 14.65, render/graph/ms 5.52, stream/spawn/ms 1.76, render/present/ms 1.61, stream/tick/ms 1.25, bigspace/total/ms 1.10, terrain/update/ms 0.71, terrain/select/ms 0.60, bigspace/low_precision/ms 0.39 |
| warm-up | 7.26 | 30.4 | pipelines/wait_ms 11.7 k, pipelines/compile_ms 91.03, pipelines/front_ms 87.24, main/busy/ms 35.50, render/thread/ms 35.37, render/busy/ms 34.09, render/graph/ms 5.40, pipelines/driver_ms 3.80, stream/drive/ms 1.13, render/present/ms 1.08, physics/insert/ms 0.82, bigspace/total/ms 0.71 |
| warm-up | 8.09 | 31.2 | main/busy/ms 37.15, render/thread/ms 29.91, render/busy/ms 28.74, render/graph/ms 8.24, render/present/ms 0.99, bigspace/total/ms 0.93, physics/insert/ms 0.72, anim/evaluate_cpu/ms 0.56, stream/drive/ms 0.51, terrain/update/ms 0.51, bigspace/low_precision/ms 0.51, stream/tick/ms 0.45 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_pbr\balmora@2560x1440_off=normalmaps_orm_parallax-2 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 90%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.4 W · 78 °C (max 80) · VRAM 5698 MiB · throttled 100% of samples
