# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: clouds,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1442 frames over 15.0 s · **median 10.33 ms (97 fps)** · mean 10.40 · p95 11.60 · p99 12.28 (1% low 81 fps) · worst 22.38 ms · 4 spikes over 1.5× median · 16.0 stutters/min (work 8.0/min)
- warm-up: 376 frames over 8.4 s, worst 2513.8 ms
- hitches (> 25.8 ms): **0** measured, 27 in warm-up
- timed GPU passes: 8.52 ms of a 10.40 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.33 ms, limit 11 ms
- pass worst frame 22.38 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.615 | 2.637 | 3.100 | 0.99 | 0.056 |
| dlss_super_resolution | 2.168 | 2.186 | 2.600 | 0.99 | 0.190 |
| early prepass | 0.606 | 0.611 | 0.948 | 0.99 | 0.095 |
| atmosphere_luts | 0.361 | 0.364 | 0.541 | 0.99 | 0.003 |
| bloom | 0.343 | 0.346 | 0.678 | 0.99 | 0.065 |
| main_transmissive_pass_3d | 0.296 | 0.298 | 0.303 | 0.99 | 0.009 |
| water_velocity | 0.257 | 0.259 | 0.269 | 0.99 | 0.002 |
| water_sspr | 0.244 | 0.246 | 0.335 | 0.99 | 0.002 |
| ssao | 0.209 | 0.210 | 0.219 | 0.99 | 0.008 |
| uniform_allocation | 0.180 | 0.182 | 0.183 | 0.99 | 0.033 |
| atmos_froxel_inject | 0.141 | 0.143 | 0.136 | 0.99 | 0.002 |
| water_fft | 0.140 | 0.141 | 0.139 | 0.99 | 0.003 |
| bin_unpacking | 0.139 | 0.140 | 0.144 | 0.99 | 0.024 |
| upscaling | 0.123 | 0.124 | 0.162 | 0.99 | 0.004 |
| clouds_composite | 0.107 | 0.108 | 0.106 | 0.99 | 0.002 |
| tonemapping | 0.094 | 0.094 | 0.094 | 0.99 | 0.003 |
| clustering | 0.068 | 0.068 | 0.076 | 0.99 | 0.022 |
| render_sky | 0.056 | 0.056 | 0.055 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.053 | 0.434 | 0.423 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.051 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.041 | 0.041 | 0.042 | 0.99 | 0.006 |
| water_snapshot | 0.039 | 0.039 | 0.055 | 0.99 | 0.002 |
| atmos_froxel_integrate | 0.030 | 0.031 | 0.032 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.230 | 0.233 | 0.12 | 0.004 |
| clouds_trace | 0.026 | 0.026 | 0.028 | 0.99 | 0.001 |
| water_mips | 0.024 | 0.025 | 0.024 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.022 | 0.022 | 0.024 | 0.99 | 0.074 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.029 | 0.99 | 0.007 |
| atmosphere_environment | 0.011 | 0.011 | 0.014 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.007 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.016 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.003 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 54.477 | 0.000 | 6226.40 | 78556.0 |
| render/thread/ms | 9.390 | 10.613 | 21.37 | 13540.7 |
| render/busy/ms | 7.396 | 8.730 | 20.71 | 10664.9 |
| main/busy/ms | 7.035 | 8.213 | 18.81 | 10143.8 |
| render/graph/ms | 4.261 | 5.317 | 17.01 | 6144.0 |
| render/acquire/ms | 1.517 | 2.723 | 3.81 | 2187.4 |
| pipelines/compile_ms | 1.064 | 0.000 | 149.13 | 1534.3 |
| pipelines/front_ms | 1.028 | 0.000 | 144.33 | 1483.0 |
| render/present/ms | 0.477 | 0.577 | 4.16 | 688.4 |
| bigspace/total/ms | 0.375 | 0.693 | 1.76 | 540.4 |
| terrain/update/ms | 0.226 | 0.291 | 0.45 | 326.5 |
| terrain/select/ms | 0.197 | 0.253 | 0.40 | 283.5 |
| stream/drive/ms | 0.183 | 0.262 | 0.75 | 264.4 |
| far/select/ms | 0.162 | 0.250 | 0.51 | 234.2 |
| controller/feed/ms | 0.158 | 0.217 | 0.38 | 228.5 |
| stream/tick/ms | 0.157 | 0.225 | 0.70 | 226.1 |
| bigspace/high_precision/ms | 0.132 | 0.309 | 1.56 | 191.0 |
| controller/move/ms | 0.110 | 0.177 | 0.42 | 158.5 |
| bigspace/low_precision/ms | 0.090 | 0.215 | 1.32 | 130.1 |
| anim/evaluate/ms | 0.078 | 0.198 | 0.99 | 113.0 |
| anim/evaluate_cpu/ms | 0.059 | 0.083 | 0.25 | 85.8 |
| physics/step/ms | 0.037 | 0.065 | 0.94 | 53.2 |
| pipelines/driver_ms | 0.036 | 0.000 | 4.79 | 51.3 |
| weather/map/ms | 0.025 | 0.034 | 0.16 | 35.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.28 | 15.8 |
| physics/insert/ms | 0.009 | 0.017 | 0.66 | 13.6 |
| script/update/ms | 0.007 | 0.009 | 0.05 | 9.7 |
| controller/avoid/ms | 0.006 | 0.008 | 0.03 | 8.0 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 7.0 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 6.4 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 5.7 |
| controller/write/ms | 0.002 | 0.002 | 0.18 | 2.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.14 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
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
| anim/a2/count | count | 0 | 11 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 12 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 19 | 8 |
| crowd/c0/joints/count | count | 272 | 646 | 272 |
| crowd/c0/meshes/count | count | 48 | 114 | 48 |
| crowd/c1/count | count | 122 | 124 | 122 |
| crowd/c1/joints/count | count | 4848 | 4915 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1118 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 6880 | 4212 |
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
| frame/work/ms | ms | 9.15 | 20.71 | 9.98 |
| gpu/timed/ms | ms | 9.14 | 10.93 | 9.98 |
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
| phases/shadow/batch_sets | count | 31 | 31 | 31 |
| phases/shadow/bins | count | 2028 | 2029 | 2028 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5345 | 5346 | 5345 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 31 | 31 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 898 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 182 | 182 | 182 |
| pipelines/waiting | count | 0 | 24 | 0 |
| preprocess/bind_groups | count | 0 | 148 | 0 |
| preprocess/bind_groups_created | count | 0 | 148 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 201 | 201 | 201 |
| process/cpu_usage | % | 25.56 | 38.46 | 26.44 |
| process/mem_usage | GiB | 4.30 | 4.36 | 4.36 |
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
| system/cpu_usage | % | 31.19 | 57.02 | 25.58 |
| system/mem_usage | % | 56.19 | 56.56 | 56.47 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2480 | 2480 | 2480 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 89 | 93 | 89 |
| views/all/bins | count | 5198 | 5201 | 5198 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.6 k | 14.6 k |
| views/all/pipelines | count | 85 | 89 | 85 |
| views/all/pipelines_distinct | count | 75 | 79 | 75 |
| views/all/work_items | count | 14.8 k | 14.8 k | 14.8 k |
| views/cascade/batch_sets | count | 31 | 31 | 31 |
| views/cascade/bins | count | 2028 | 2029 | 2028 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5345 | 5346 | 5345 |
| views/cascade/pipelines | count | 31 | 31 | 31 |
| views/cascade/work_items | count | 5345 | 5346 | 5345 |
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
| anim/bones_written/count | count | 0.09 | 66 | 132 |
| anim/events/count | count | 0.10 | 3 | 144 |
| assets/Image/added | count | 0.01 | 19 | 21 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.04 | 35 | 62 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.02 | 23 | 25 |
| assets/StandardMaterial/modified | count | 3.43 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.45 | 1121 | 2093 |
| physics/inserts | count | 0.67 | 155 | 973 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.62 | 2 | 900 |
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

## Hitches (> 25.8 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2513.8 | main/busy/ms 2489, surface/advance_cpu_ms 69.07, surface/cell_upload_ms 0.68, stream/drive/ms 0.28, weather/map/ms 0.14, bigspace/total/ms 0.10, stream/tick/ms 0.06, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.19 | 677.5 | main/busy/ms 2529, render/thread/ms 645, render/busy/ms 538, render/acquire/ms 105, surface/advance_cpu_ms 69.07, render/graph/ms 5.95, render/present/ms 2.59, stream/drive/ms 0.81, surface/cell_upload_ms 0.68, stream/tick/ms 0.42, terrain/update/ms 0.37, terrain/select/ms 0.23 |
| warm-up | 3.40 | 212.1 | pipelines/wait_ms 1661, render/thread/ms 656, render/busy/ms 548, pipelines/compile_ms 227, pipelines/front_ms 213, render/acquire/ms 105, main/busy/ms 55.74, terrain/update/ms 21.62, pipelines/driver_ms 14.42, render/graph/ms 9.97, render/present/ms 3.82, stream/drive/ms 1.09 |
| warm-up | 3.65 | 90.5 | pipelines/compile_ms 241, pipelines/front_ms 231, main/busy/ms 62.57, render/thread/ms 53.84, render/busy/ms 51.84, pipelines/driver_ms 10.78, render/graph/ms 4.06, render/present/ms 1.77, stream/drive/ms 1.60, stream/tick/ms 1.36, terrain/update/ms 0.69, terrain/select/ms 0.53 |
| warm-up | 3.70 | 37.3 | pipelines/wait_ms 231, pipelines/compile_ms 51.23, pipelines/front_ms 46.67, render/thread/ms 40.99, render/busy/ms 33.32, main/busy/ms 21.86, render/present/ms 7.43, terrain/update/ms 5.63, render/graph/ms 4.85, pipelines/driver_ms 4.56, stream/drive/ms 2.47, stream/tick/ms 1.22 |
| warm-up | 3.81 | 47.8 | render/thread/ms 44.88, render/busy/ms 43.37, main/busy/ms 38.83, stream/drive/ms 28.68, render/graph/ms 2.99, render/present/ms 1.28, stream/tick/ms 0.60, terrain/update/ms 0.47, terrain/select/ms 0.37, stream/spawn/ms 0.29, render/acquire/ms 0.23, bigspace/total/ms 0.19 |
| warm-up | 3.85 | 27.1 | pipelines/wait_ms 1523, pipelines/compile_ms 189, pipelines/front_ms 168, render/thread/ms 39.18, render/busy/ms 37.39, main/busy/ms 30.77, pipelines/driver_ms 20.75, stream/drive/ms 14.97, render/graph/ms 4.77, stream/spawn/ms 1.62, render/present/ms 1.54, stream/tick/ms 0.83 |
| warm-up | 3.88 | 30.0 | pipelines/wait_ms 1098, pipelines/compile_ms 188, pipelines/front_ms 167, render/thread/ms 42.27, render/busy/ms 40.71, main/busy/ms 20.44, pipelines/driver_ms 20.37, render/graph/ms 4.23, stream/drive/ms 3.24, render/present/ms 1.30, stream/spawn/ms 1.23, stream/tick/ms 1.21 |
| warm-up | 3.91 | 32.3 | render/thread/ms 45.10, render/busy/ms 43.10, main/busy/ms 33.71, stream/drive/ms 15.40, render/graph/ms 4.84, render/present/ms 1.72, stream/spawn/ms 1.27, terrain/update/ms 0.99, stream/tick/ms 0.98, terrain/select/ms 0.67, bigspace/total/ms 0.29, render/acquire/ms 0.28 |
| warm-up | 3.96 | 45.0 | pipelines/wait_ms 428, pipelines/compile_ms 80.24, pipelines/front_ms 77.60, render/thread/ms 55.34, render/busy/ms 53.47, main/busy/ms 35.53, stream/drive/ms 16.13, render/graph/ms 3.93, pipelines/driver_ms 2.63, stream/spawn/ms 2.28, render/present/ms 1.63, terrain/update/ms 0.71 |
| warm-up | 4.08 | 96.2 | pipelines/wait_ms 506, main/busy/ms 69.08, render/thread/ms 51.08, render/busy/ms 49.76, stream/drive/ms 47.46, pipelines/compile_ms 45.04, pipelines/front_ms 42.91, render/graph/ms 4.75, stream/spawn/ms 3.58, pipelines/driver_ms 2.13, render/present/ms 1.11, stream/tick/ms 0.99 |
| warm-up | 4.16 | 80.1 | pipelines/wait_ms 1667, pipelines/compile_ms 149, pipelines/front_ms 135, render/thread/ms 101, render/busy/ms 99.92, main/busy/ms 43.79, pipelines/driver_ms 13.59, stream/drive/ms 13.30, render/graph/ms 4.04, stream/spawn/ms 2.66, stream/tick/ms 1.34, stream/activate/ms 1.12 |
| warm-up | 4.21 | 35.3 | main/busy/ms 44.70, render/thread/ms 38.49, render/busy/ms 37.19, stream/drive/ms 28.44, render/graph/ms 5.33, terrain/update/ms 2.39, render/present/ms 1.11, stream/spawn/ms 0.84, stream/tick/ms 0.71, terrain/select/ms 0.62, stream/activate/ms 0.57, physics/insert/ms 0.31 |
| warm-up | 4.24 | 27.0 | main/busy/ms 41.27, render/thread/ms 31.47, render/busy/ms 30.43, stream/drive/ms 28.30, render/graph/ms 2.43, terrain/update/ms 2.20, render/present/ms 0.85, stream/tick/ms 0.73, stream/activate/ms 0.72, terrain/select/ms 0.49, stream/spawn/ms 0.43, physics/insert/ms 0.30 |
| warm-up | 4.33 | 50.5 | pipelines/wait_ms 1559, pipelines/compile_ms 200, pipelines/front_ms 192, main/busy/ms 61.97, stream/drive/ms 52.49, render/thread/ms 20.62, render/busy/ms 19.45, pipelines/driver_ms 7.56, render/graph/ms 2.63, render/present/ms 0.97, stream/tick/ms 0.79, terrain/update/ms 0.46 |
| warm-up | 4.40 | 58.4 | pipelines/wait_ms 5076, pipelines/compile_ms 108, pipelines/front_ms 100, main/busy/ms 54.42, stream/drive/ms 35.94, render/thread/ms 23.00, render/busy/ms 21.91, pipelines/driver_ms 7.38, stream/spawn/ms 2.46, render/graph/ms 2.30, render/present/ms 0.89, terrain/update/ms 0.82 |
| warm-up | 4.81 | 39.5 | pipelines/wait_ms 9081, pipelines/compile_ms 63.29, pipelines/front_ms 58.84, render/thread/ms 47.49, render/busy/ms 46.15, main/busy/ms 38.10, stream/drive/ms 6.10, render/graph/ms 5.05, pipelines/driver_ms 4.44, stream/spawn/ms 2.42, render/present/ms 1.09, stream/tick/ms 0.88 |
| warm-up | 4.88 | 28.6 | pipelines/wait_ms 1397, render/thread/ms 43.80, render/busy/ms 41.83, main/busy/ms 24.21, pipelines/compile_ms 17.34, pipelines/front_ms 16.35, render/graph/ms 6.98, stream/drive/ms 4.71, stream/spawn/ms 2.05, render/present/ms 1.68, stream/tick/ms 1.36, pipelines/driver_ms 0.99 |
| warm-up | 4.91 | 36.0 | render/thread/ms 49.63, render/busy/ms 47.89, main/busy/ms 24.75, render/graph/ms 5.87, stream/drive/ms 4.99, stream/spawn/ms 2.13, render/present/ms 1.44, stream/tick/ms 1.43, bigspace/total/ms 0.76, terrain/update/ms 0.68, terrain/select/ms 0.56, bigspace/high_precision/ms 0.33 |
| warm-up | 4.94 | 30.9 | render/thread/ms 52.60, render/busy/ms 51.18, main/busy/ms 23.17, render/graph/ms 4.83, stream/drive/ms 4.83, stream/spawn/ms 2.15, render/present/ms 1.13, terrain/update/ms 0.73, stream/tick/ms 0.72, bigspace/total/ms 0.69, terrain/select/ms 0.60, bigspace/high_precision/ms 0.32 |
| warm-up | 5.03 | 27.1 | pipelines/wait_ms 980, pipelines/compile_ms 164, pipelines/front_ms 159, render/thread/ms 41.61, render/busy/ms 39.80, main/busy/ms 35.16, stream/drive/ms 6.77, render/graph/ms 6.18, pipelines/driver_ms 5.07, stream/spawn/ms 3.24, terrain/update/ms 1.61, render/present/ms 1.53 |
| warm-up | 5.08 | 43.0 | render/thread/ms 49.52, render/busy/ms 47.24, main/busy/ms 38.11, render/graph/ms 6.60, stream/drive/ms 5.37, stream/spawn/ms 3.34, render/present/ms 1.94, terrain/update/ms 1.72, terrain/select/ms 1.54, stream/tick/ms 1.14, physics/insert/ms 1.06, bigspace/total/ms 0.82 |
| warm-up | 5.12 | 38.3 | main/busy/ms 53.62, render/thread/ms 53.50, render/busy/ms 50.89, stream/drive/ms 17.35, render/graph/ms 7.47, stream/spawn/ms 2.69, render/present/ms 2.30, stream/tick/ms 1.28, physics/insert/ms 1.24, bigspace/total/ms 0.95, terrain/update/ms 0.93, terrain/select/ms 0.74 |
| warm-up | 5.16 | 47.3 | pipelines/wait_ms 1740, main/busy/ms 51.01, render/thread/ms 47.30, render/busy/ms 45.00, stream/drive/ms 16.55, render/graph/ms 7.03, stream/spawn/ms 2.34, render/present/ms 1.96, pipelines/compile_ms 1.92, pipelines/front_ms 1.66, bigspace/total/ms 0.95, stream/tick/ms 0.94 |
| warm-up | 5.28 | 28.8 | main/busy/ms 34.54, render/thread/ms 22.65, render/busy/ms 21.17, stream/drive/ms 16.72, render/graph/ms 5.72, stream/spawn/ms 2.17, render/present/ms 1.23, stream/tick/ms 0.65, bigspace/total/ms 0.62, terrain/update/ms 0.47, terrain/select/ms 0.37, bigspace/high_precision/ms 0.27 |
| warm-up | 5.58 | 43.0 | pipelines/wait_ms 9353, pipelines/compile_ms 66.16, pipelines/front_ms 62.09, render/thread/ms 40.24, render/busy/ms 38.58, main/busy/ms 36.55, stream/drive/ms 14.18, render/graph/ms 5.41, pipelines/driver_ms 4.07, stream/spawn/ms 3.05, render/present/ms 1.37, stream/tick/ms 1.23 |
| warm-up | 8.24 | 32.5 | pipelines/wait_ms 1591, main/busy/ms 43.38, render/thread/ms 36.75, render/busy/ms 34.36, render/graph/ms 15.04, pipelines/compile_ms 14.89, pipelines/front_ms 13.78, render/acquire/ms 1.34, pipelines/driver_ms 1.11, render/present/ms 1.05, bigspace/total/ms 0.72, terrain/update/ms 0.49 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_clouds\balmora@2560x1440_off=normalmaps_orm_parallax_clouds-3 --bench-baseline none --off normalmaps,orm,parallax,clouds
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **97%** (min 75%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.0 W · 77 °C (max 80) · VRAM 5715 MiB · throttled 100% of samples
