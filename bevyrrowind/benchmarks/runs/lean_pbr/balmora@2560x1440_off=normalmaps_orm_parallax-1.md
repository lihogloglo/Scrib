# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1372 frames over 15.0 s · **median 10.92 ms (92 fps)** · mean 10.94 · p95 12.22 · p99 12.78 (1% low 78 fps) · worst 25.32 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 293 frames over 7.8 s, worst 2761.1 ms
- hitches (> 27.3 ms): **0** measured, 25 in warm-up
- timed GPU passes: 8.95 ms of a 10.94 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.92 ms, limit 11 ms
- pass worst frame 25.32 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.709 | 2.733 | 3.342 | 0.99 | 0.053 |
| dlss_super_resolution | 2.249 | 2.269 | 2.877 | 0.99 | 0.193 |
| early prepass | 0.604 | 0.609 | 1.207 | 0.99 | 0.094 |
| atmosphere_luts | 0.371 | 0.375 | 0.689 | 0.99 | 0.004 |
| bloom | 0.347 | 0.350 | 0.494 | 0.99 | 0.068 |
| main_transmissive_pass_3d | 0.299 | 0.302 | 0.302 | 0.99 | 0.008 |
| water_velocity | 0.262 | 0.265 | 0.271 | 0.99 | 0.001 |
| water_sspr | 0.244 | 0.246 | 0.334 | 0.99 | 0.002 |
| ssao | 0.212 | 0.214 | 0.218 | 0.99 | 0.008 |
| uniform_allocation | 0.185 | 0.186 | 0.184 | 0.99 | 0.034 |
| atmos_froxel_inject | 0.157 | 0.159 | 0.152 | 0.99 | 0.002 |
| clouds_trace | 0.151 | 0.153 | 0.189 | 0.99 | 0.001 |
| water_fft | 0.148 | 0.149 | 0.139 | 0.99 | 0.003 |
| bin_unpacking | 0.144 | 0.146 | 0.146 | 0.99 | 0.024 |
| upscaling | 0.123 | 0.125 | 0.128 | 0.99 | 0.004 |
| clouds_composite | 0.120 | 0.121 | 0.117 | 0.99 | 0.003 |
| tonemapping | 0.096 | 0.097 | 0.100 | 0.99 | 0.003 |
| clustering | 0.074 | 0.074 | 0.082 | 0.99 | 0.023 |
| lightprobe_irradiance_map | 0.058 | 0.471 | 1.084 | 0.12 | 0.001 |
| render_sky | 0.057 | 0.058 | 0.056 | 0.99 | 0.002 |
| clouds_resolve | 0.052 | 0.052 | 0.054 | 0.99 | 0.001 |
| water_snapshot | 0.042 | 0.043 | 0.056 | 0.99 | 0.002 |
| early_mesh_preprocessing | 0.040 | 0.041 | 0.042 | 0.99 | 0.006 |
| clouds_shadow | 0.032 | 0.032 | 0.036 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.031 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.030 | 0.241 | 0.241 | 0.12 | 0.004 |
| water_mips | 0.025 | 0.025 | 0.024 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.024 | 0.030 | 0.99 | 0.007 |
| main_transparent_pass_3d | 0.023 | 0.023 | 0.024 | 0.99 | 0.073 |
| atmosphere_environment | 0.010 | 0.010 | 0.014 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 0.99 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.008 | 0.99 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.004 |
| lightprobe_copy | 0.002 | 0.017 | 0.018 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 78.551 | 0.000 | 11406.57 | 107772.5 |
| render/thread/ms | 9.980 | 11.243 | 24.00 | 13692.8 |
| render/busy/ms | 7.524 | 8.990 | 12.01 | 10323.5 |
| main/busy/ms | 7.016 | 8.312 | 12.02 | 9626.3 |
| render/graph/ms | 4.245 | 5.540 | 8.86 | 5823.5 |
| render/acquire/ms | 1.961 | 3.583 | 4.39 | 2690.3 |
| pipelines/compile_ms | 1.333 | 0.000 | 128.05 | 1829.0 |
| pipelines/front_ms | 1.290 | 0.000 | 124.24 | 1769.6 |
| render/present/ms | 0.495 | 0.594 | 15.71 | 679.0 |
| bigspace/total/ms | 0.370 | 0.637 | 2.09 | 507.1 |
| terrain/update/ms | 0.237 | 0.312 | 0.59 | 325.6 |
| terrain/select/ms | 0.207 | 0.270 | 0.55 | 283.4 |
| stream/drive/ms | 0.189 | 0.279 | 0.55 | 259.7 |
| far/select/ms | 0.168 | 0.240 | 0.45 | 230.0 |
| stream/tick/ms | 0.162 | 0.244 | 0.45 | 221.8 |
| controller/feed/ms | 0.157 | 0.218 | 0.99 | 215.0 |
| bigspace/high_precision/ms | 0.134 | 0.300 | 1.78 | 183.3 |
| controller/move/ms | 0.118 | 0.182 | 0.30 | 161.8 |
| bigspace/low_precision/ms | 0.089 | 0.222 | 1.13 | 122.4 |
| anim/evaluate/ms | 0.085 | 0.171 | 1.05 | 116.6 |
| anim/evaluate_cpu/ms | 0.062 | 0.082 | 0.27 | 85.7 |
| pipelines/driver_ms | 0.043 | 0.000 | 4.42 | 59.4 |
| physics/step/ms | 0.034 | 0.063 | 0.38 | 46.8 |
| weather/map/ms | 0.025 | 0.034 | 0.09 | 34.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.17 | 15.3 |
| script/update/ms | 0.007 | 0.009 | 0.12 | 9.4 |
| controller/avoid/ms | 0.006 | 0.008 | 0.05 | 8.3 |
| script/input/ms | 0.005 | 0.006 | 0.05 | 6.6 |
| anim/apply/ms | 0.004 | 0.006 | 0.11 | 6.0 |
| script/ui/ms | 0.004 | 0.006 | 0.05 | 5.7 |
| physics/insert/ms | 0.004 | 0.000 | 0.66 | 5.4 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
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
| anim/a2/count | count | 0 | 2 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 18 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 9 | 8 |
| crowd/c0/joints/count | count | 272 | 306 | 272 |
| crowd/c0/meshes/count | count | 48 | 54 | 48 |
| crowd/c1/count | count | 123 | 131 | 122 |
| crowd/c1/joints/count | count | 4880 | 5008 | 4848 |
| crowd/c1/meshes/count | count | 1088 | 1127 | 1080 |
| crowd/c2/count | count | 283 | 284 | 284 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 5011 | 4212 |
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
| frame/work/ms | ms | 9.57 | 12.02 | 10.13 |
| gpu/timed/ms | ms | 9.56 | 11.35 | 10.13 |
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
| phases/shadow/batch_sets | count | 31 | 36 | 31 |
| phases/shadow/bins | count | 2029 | 2323 | 2028 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5346 | 5644 | 5345 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 31 | 36 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 129 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 180 | 180 | 180 |
| pipelines/waiting | count | 0 | 30 | 0 |
| preprocess/bind_groups | count | 0 | 120 | 0 |
| preprocess/bind_groups_created | count | 0 | 120 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 201 | 201 | 201 |
| process/cpu_usage | % | 29.37 | 44.71 | 27.27 |
| process/mem_usage | GiB | 4.33 | 4.43 | 4.40 |
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
| system/cpu_usage | % | 34.34 | 61.70 | 45.65 |
| system/mem_usage | % | 72.94 | 73.49 | 73.39 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2480 | 2480 | 2480 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 89 | 94 | 89 |
| views/all/bins | count | 5199 | 5493 | 5198 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.9 k | 14.6 k |
| views/all/pipelines | count | 85 | 90 | 85 |
| views/all/pipelines_distinct | count | 75 | 77 | 75 |
| views/all/work_items | count | 14.8 k | 15.1 k | 14.8 k |
| views/cascade/batch_sets | count | 31 | 36 | 31 |
| views/cascade/bins | count | 2029 | 2323 | 2028 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5346 | 5644 | 5345 |
| views/cascade/pipelines | count | 31 | 36 | 31 |
| views/cascade/work_items | count | 5346 | 5644 | 5345 |
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
| anim/bones_written/count | count | 0.07 | 66 | 99 |
| anim/events/count | count | 0.10 | 4 | 132 |
| assets/Image/added | count | 0.02 | 10 | 24 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.03 | 13 | 35 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.02 | 10 | 24 |
| assets/StandardMaterial/modified | count | 3.61 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.96 | 393 | 1319 |
| physics/inserts | count | 0.10 | 62 | 142 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.66 | 1 | 901 |
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

## Hitches (> 27.3 ms; slowest 25, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.76 | 2761.1 | main/busy/ms 2733, surface/advance_cpu_ms 68.37, surface/cell_upload_ms 0.85, stream/drive/ms 0.32, weather/map/ms 0.27, bigspace/total/ms 0.09, weather/eval/ms 0.08, stream/tick/ms 0.08, bigspace/high_precision/ms 0.05, script/update/ms 0.03, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.34 | 576.1 | main/busy/ms 2771, render/thread/ms 533, render/busy/ms 409, render/acquire/ms 121, surface/advance_cpu_ms 68.37, render/graph/ms 6.39, render/present/ms 2.46, surface/cell_upload_ms 0.85, stream/drive/ms 0.84, terrain/update/ms 0.68, terrain/select/ms 0.52, stream/tick/ms 0.43 |
| warm-up | 3.57 | 231.5 | pipelines/wait_ms 3360, render/thread/ms 545, render/busy/ms 420, pipelines/compile_ms 249, pipelines/front_ms 233, render/acquire/ms 121, main/busy/ms 55.45, terrain/update/ms 20.58, pipelines/driver_ms 16.11, render/graph/ms 10.32, render/present/ms 3.43, stream/drive/ms 1.15 |
| warm-up | 3.81 | 81.2 | render/thread/ms 60.47, main/busy/ms 59.36, render/busy/ms 57.71, render/graph/ms 12.34, stream/drive/ms 1.63, render/present/ms 1.60, stream/tick/ms 1.40, render/acquire/ms 1.16, terrain/update/ms 0.64, terrain/select/ms 0.51, weather/map/ms 0.23, bigspace/total/ms 0.19 |
| warm-up | 4.01 | 67.8 | main/busy/ms 43.57, render/thread/ms 40.02, render/busy/ms 38.35, stream/drive/ms 30.32, render/graph/ms 4.60, render/present/ms 1.38, stream/spawn/ms 1.16, terrain/update/ms 0.84, terrain/select/ms 0.67, stream/tick/ms 0.65, render/acquire/ms 0.29, bigspace/total/ms 0.22 |
| warm-up | 4.11 | 91.4 | pipelines/wait_ms 4429, pipelines/compile_ms 272, pipelines/front_ms 248, render/thread/ms 55.55, render/busy/ms 53.79, main/busy/ms 37.42, pipelines/driver_ms 24.28, stream/drive/ms 17.08, render/graph/ms 4.18, render/present/ms 1.51, stream/tick/ms 1.49, stream/spawn/ms 1.45 |
| warm-up | 4.17 | 60.4 | pipelines/wait_ms 7731, pipelines/compile_ms 324, pipelines/front_ms 294, render/thread/ms 90.07, render/busy/ms 69.72, main/busy/ms 58.69, stream/drive/ms 36.60, pipelines/driver_ms 30.61, render/present/ms 20.09, render/graph/ms 3.70, stream/spawn/ms 1.97, terrain/update/ms 0.92 |
| warm-up | 4.23 | 61.1 | pipelines/wait_ms 5037, render/thread/ms 88.03, pipelines/compile_ms 85.96, pipelines/front_ms 76.43, render/busy/ms 67.32, main/busy/ms 63.21, stream/drive/ms 38.12, render/present/ms 20.35, pipelines/driver_ms 9.53, render/graph/ms 5.21, stream/spawn/ms 3.27, terrain/update/ms 1.15 |
| warm-up | 4.28 | 46.7 | pipelines/wait_ms 1734, render/thread/ms 84.22, render/busy/ms 56.81, main/busy/ms 44.60, pipelines/compile_ms 34.12, pipelines/front_ms 30.92, render/present/ms 27.06, stream/drive/ms 23.17, render/graph/ms 4.22, stream/spawn/ms 3.52, pipelines/driver_ms 3.20, terrain/update/ms 0.90 |
| warm-up | 4.33 | 46.0 | pipelines/wait_ms 1105, pipelines/compile_ms 136, pipelines/front_ms 128, render/thread/ms 70.78, render/busy/ms 43.75, main/busy/ms 37.68, render/present/ms 26.75, stream/drive/ms 21.75, pipelines/driver_ms 8.37, render/graph/ms 5.85, stream/spawn/ms 2.15, terrain/update/ms 0.77 |
| warm-up | 4.37 | 36.4 | pipelines/wait_ms 4291, pipelines/compile_ms 63.52, pipelines/front_ms 58.86, main/busy/ms 41.67, stream/drive/ms 26.47, render/thread/ms 18.40, render/busy/ms 17.32, pipelines/driver_ms 4.66, render/graph/ms 3.93, render/present/ms 0.90, stream/tick/ms 0.82, stream/activate/ms 0.73 |
| warm-up | 4.47 | 51.4 | pipelines/wait_ms 1411, main/busy/ms 58.76, stream/drive/ms 47.10, render/thread/ms 20.00, render/busy/ms 18.78, pipelines/compile_ms 16.38, pipelines/front_ms 15.54, render/graph/ms 3.58, render/present/ms 1.04, stream/spawn/ms 1.01, bigspace/total/ms 0.85, stream/tick/ms 0.85 |
| warm-up | 4.53 | 49.6 | main/busy/ms 55.39, stream/drive/ms 44.35, render/thread/ms 20.51, render/busy/ms 19.05, render/graph/ms 5.09, render/present/ms 1.29, stream/tick/ms 0.87, terrain/update/ms 0.66, terrain/select/ms 0.53, bigspace/total/ms 0.25, render/acquire/ms 0.17, stream/spawn/ms 0.12 |
| warm-up | 4.61 | 35.2 | pipelines/wait_ms 949, pipelines/compile_ms 69.06, pipelines/front_ms 66.93, main/busy/ms 27.85, render/thread/ms 24.51, render/busy/ms 23.32, stream/drive/ms 9.05, terrain/update/ms 6.17, render/graph/ms 3.23, pipelines/driver_ms 2.12, stream/spawn/ms 1.84, render/present/ms 0.98 |
| warm-up | 4.83 | 30.1 | render/thread/ms 37.88, render/busy/ms 36.38, main/busy/ms 26.62, stream/drive/ms 10.60, render/graph/ms 7.16, stream/spawn/ms 2.02, stream/tick/ms 1.32, render/present/ms 1.27, bigspace/total/ms 0.69, terrain/update/ms 0.55, terrain/select/ms 0.42, bigspace/high_precision/ms 0.34 |
| warm-up | 4.95 | 30.3 | pipelines/wait_ms 1359, render/thread/ms 42.79, render/busy/ms 41.55, main/busy/ms 19.15, render/graph/ms 4.37, stream/drive/ms 3.67, stream/spawn/ms 2.09, pipelines/compile_ms 1.33, pipelines/front_ms 1.07, render/present/ms 1.02, stream/tick/ms 0.90, terrain/update/ms 0.61 |
| warm-up | 5.03 | 31.8 | render/thread/ms 45.31, render/busy/ms 43.23, main/busy/ms 22.80, render/graph/ms 6.10, stream/drive/ms 4.47, stream/spawn/ms 2.21, render/present/ms 1.75, stream/tick/ms 1.08, bigspace/total/ms 0.64, terrain/update/ms 0.54, terrain/select/ms 0.42, bigspace/high_precision/ms 0.36 |
| warm-up | 5.09 | 29.1 | render/thread/ms 40.50, render/busy/ms 39.34, main/busy/ms 20.95, stream/drive/ms 4.24, render/graph/ms 4.08, stream/spawn/ms 2.06, render/present/ms 0.99, stream/tick/ms 0.82, terrain/update/ms 0.56, bigspace/total/ms 0.55, terrain/select/ms 0.45, bigspace/high_precision/ms 0.25 |
| warm-up | 5.17 | 34.2 | pipelines/wait_ms 11.0 k, pipelines/compile_ms 196, pipelines/front_ms 184, main/busy/ms 37.36, render/thread/ms 36.67, render/busy/ms 34.28, pipelines/driver_ms 12.20, render/graph/ms 8.16, stream/drive/ms 7.76, stream/spawn/ms 3.83, render/present/ms 2.02, stream/tick/ms 1.31 |
| warm-up | 5.21 | 42.0 | pipelines/wait_ms 7547, pipelines/compile_ms 59.00, pipelines/front_ms 54.49, main/busy/ms 50.91, render/thread/ms 48.11, render/busy/ms 46.23, render/graph/ms 6.04, stream/drive/ms 5.00, pipelines/driver_ms 4.51, stream/spawn/ms 2.83, render/present/ms 1.58, stream/tick/ms 1.36 |
| warm-up | 5.25 | 43.1 | pipelines/wait_ms 3164, main/busy/ms 66.70, render/thread/ms 52.44, render/busy/ms 50.81, stream/drive/ms 14.84, pipelines/compile_ms 10.98, pipelines/front_ms 10.09, render/graph/ms 5.05, stream/spawn/ms 2.19, render/present/ms 1.35, physics/insert/ms 1.26, stream/tick/ms 0.90 |
| warm-up | 5.29 | 34.5 | main/busy/ms 47.94, render/thread/ms 42.29, render/busy/ms 40.77, stream/drive/ms 14.54, render/graph/ms 5.28, stream/spawn/ms 2.13, render/present/ms 1.27, physics/insert/ms 1.12, stream/tick/ms 0.79, bigspace/total/ms 0.71, terrain/update/ms 0.66, terrain/select/ms 0.49 |
| warm-up | 5.60 | 44.7 | pipelines/wait_ms 1943, pipelines/compile_ms 59.27, pipelines/front_ms 57.55, render/thread/ms 48.87, render/busy/ms 47.57, main/busy/ms 38.16, render/graph/ms 6.83, stream/drive/ms 2.59, pipelines/driver_ms 1.72, stream/spawn/ms 0.99, render/present/ms 0.97, stream/tick/ms 0.88 |
| warm-up | 5.82 | 47.7 | render/thread/ms 44.50, render/busy/ms 42.77, main/busy/ms 40.16, stream/drive/ms 12.83, render/graph/ms 6.19, stream/spawn/ms 2.12, render/present/ms 1.44, stream/tick/ms 1.19, bigspace/total/ms 1.07, terrain/update/ms 0.76, terrain/select/ms 0.63, bigspace/high_precision/ms 0.41 |
| warm-up | 6.73 | 42.2 | main/busy/ms 41.42, render/thread/ms 38.49, render/busy/ms 37.06, render/graph/ms 7.23, stream/drive/ms 1.28, render/present/ms 1.22, bigspace/total/ms 0.79, physics/step/ms 0.62, stream/tick/ms 0.56, anim/evaluate_cpu/ms 0.50, terrain/update/ms 0.49, physics/insert/ms 0.43 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_pbr\balmora@2560x1440_off=normalmaps_orm_parallax-1 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 8%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 97%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.0 W · 78 °C (max 81) · VRAM 5700 MiB · throttled 100% of samples
