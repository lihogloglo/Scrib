# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1394 frames over 15.0 s · **median 10.73 ms (93 fps)** · mean 10.77 · p95 12.16 · p99 13.28 (1% low 75 fps) · worst 23.12 ms · 8 spikes over 1.5× median · 16.0 stutters/min (work 60.0/min)
- warm-up: 397 frames over 9.7 s, worst 3994.3 ms
- hitches (> 26.8 ms): **0** measured, 29 in warm-up
- timed GPU passes: 8.85 ms of a 10.77 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.73 ms, limit 11 ms
- pass worst frame 23.12 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.688 | 2.725 | 3.462 | 0.99 | 0.069 |
| dlss_super_resolution | 2.225 | 2.256 | 2.835 | 0.99 | 0.189 |
| early prepass | 0.643 | 0.651 | 1.295 | 0.99 | 0.113 |
| atmosphere_luts | 0.365 | 0.381 | 0.707 | 0.96 | 0.003 |
| bloom | 0.343 | 0.348 | 0.481 | 0.99 | 0.066 |
| water_velocity | 0.284 | 0.287 | 0.269 | 0.99 | 0.002 |
| main_transmissive_pass_3d | 0.277 | 0.280 | 0.284 | 0.99 | 0.009 |
| water_sspr | 0.239 | 0.242 | 0.259 | 0.99 | 0.002 |
| uniform_allocation | 0.222 | 0.225 | 0.225 | 0.99 | 0.036 |
| ssao | 0.212 | 0.216 | 0.213 | 0.98 | 0.008 |
| bin_unpacking | 0.175 | 0.178 | 0.182 | 0.99 | 0.027 |
| water_fft | 0.148 | 0.150 | 0.141 | 0.99 | 0.003 |
| atmos_froxel_inject | 0.140 | 0.142 | 0.135 | 0.99 | 0.002 |
| upscaling | 0.128 | 0.129 | 0.144 | 0.99 | 0.004 |
| clouds_composite | 0.110 | 0.112 | 0.108 | 0.99 | 0.002 |
| tonemapping | 0.097 | 0.098 | 0.096 | 0.99 | 0.003 |
| clouds_trace | 0.069 | 0.070 | 0.077 | 0.99 | 0.001 |
| clustering | 0.066 | 0.068 | 0.078 | 0.97 | 0.022 |
| lightprobe_irradiance_map | 0.055 | 0.448 | 0.660 | 0.12 | 0.001 |
| render_sky | 0.055 | 0.055 | 0.056 | 0.99 | 0.002 |
| clouds_resolve | 0.050 | 0.051 | 0.053 | 0.99 | 0.001 |
| water_snapshot | 0.046 | 0.047 | 0.056 | 0.99 | 0.002 |
| early_mesh_preprocessing | 0.035 | 0.035 | 0.034 | 0.99 | 0.006 |
| atmos_froxel_integrate | 0.032 | 0.032 | 0.032 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.032 | 0.258 | 0.569 | 0.12 | 0.004 |
| water_mips | 0.028 | 0.028 | 0.024 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.025 | 0.026 | 0.028 | 0.99 | 0.070 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.020 | 0.99 | 0.005 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| night_sky | 0.008 | 0.008 | 0.013 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 0.98 | 0.004 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.003 | 0.01 | 0.005 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 129.297 | 0.000 | 12542.76 | 180239.7 |
| render/thread/ms | 9.823 | 11.201 | 22.10 | 13693.5 |
| render/busy/ms | 8.303 | 9.665 | 17.01 | 11574.0 |
| main/busy/ms | 7.139 | 8.609 | 15.57 | 9952.1 |
| render/graph/ms | 5.010 | 6.214 | 9.44 | 6984.5 |
| pipelines/compile_ms | 1.708 | 0.000 | 141.90 | 2381.2 |
| pipelines/front_ms | 1.635 | 0.000 | 138.29 | 2279.4 |
| render/acquire/ms | 1.027 | 2.375 | 3.63 | 1431.9 |
| render/present/ms | 0.493 | 0.568 | 12.19 | 687.6 |
| bigspace/total/ms | 0.354 | 0.621 | 2.57 | 493.0 |
| terrain/update/ms | 0.230 | 0.294 | 0.60 | 320.3 |
| terrain/select/ms | 0.200 | 0.256 | 0.57 | 279.2 |
| stream/drive/ms | 0.191 | 0.280 | 0.45 | 266.9 |
| far/select/ms | 0.171 | 0.250 | 0.50 | 238.1 |
| stream/tick/ms | 0.163 | 0.243 | 0.41 | 227.6 |
| controller/feed/ms | 0.157 | 0.208 | 0.46 | 219.4 |
| controller/move/ms | 0.152 | 0.282 | 0.69 | 211.5 |
| bigspace/high_precision/ms | 0.122 | 0.266 | 1.77 | 170.1 |
| bigspace/low_precision/ms | 0.091 | 0.199 | 2.36 | 126.3 |
| anim/evaluate/ms | 0.079 | 0.200 | 1.46 | 110.2 |
| pipelines/driver_ms | 0.073 | 0.000 | 5.08 | 101.7 |
| anim/evaluate_cpu/ms | 0.063 | 0.082 | 0.33 | 87.5 |
| physics/step/ms | 0.038 | 0.061 | 3.84 | 53.3 |
| weather/map/ms | 0.016 | 0.021 | 0.12 | 21.9 |
| weather/eval/ms | 0.010 | 0.013 | 0.09 | 13.9 |
| physics/insert/ms | 0.007 | 0.000 | 1.60 | 10.3 |
| script/update/ms | 0.007 | 0.010 | 0.13 | 9.4 |
| controller/avoid/ms | 0.006 | 0.007 | 0.11 | 8.1 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 7.0 |
| script/ui/ms | 0.005 | 0.006 | 0.05 | 6.4 |
| anim/apply/ms | 0.004 | 0.006 | 0.13 | 6.2 |
| controller/write/ms | 0.002 | 0.002 | 0.08 | 2.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.6 |
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
| anim/a2/count | count | 0 | 12 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 46 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 14 | 8 |
| crowd/c0/joints/count | count | 272 | 476 | 272 |
| crowd/c0/meshes/count | count | 48 | 84 | 48 |
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1080 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 118 | 118 | 118 |
| crowd/joints_extracted/count | count | 2958 | 10.8 k | 2958 |
| crowd/joints_uploaded/count | count | 2958 | 2958 | 2958 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.09 M | 189.09 M | 189.09 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.68 | 17.01 | 9.20 |
| gpu/timed/ms | ms | 9.66 | 11.48 | 9.20 |
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
| phases/opaque3d/batch_sets | count | 34 | 35 | 34 |
| phases/opaque3d/bins | count | 1303 | 1304 | 1303 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 34 | 35 | 34 |
| phases/opaque3d_prepass/bins | count | 1303 | 1304 | 1303 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 29 | 29 | 29 |
| phases/shadow/bins | count | 985 | 985 | 985 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 2006 | 2006 | 2006 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 24 | 24 | 24 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 959 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 195 | 195 | 195 |
| pipelines/waiting | count | 0 | 49 | 0 |
| preprocess/bind_groups | count | 0 | 242 | 0 |
| preprocess/bind_groups_created | count | 0 | 242 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 244 | 244 | 244 |
| process/cpu_usage | % | 28.84 | 41.52 | 28.85 |
| process/mem_usage | GiB | 4.36 | 4.44 | 4.44 |
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
| started_load_count |  | 3545 | 3547 | 3547 |
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
| system/cpu_usage | % | 34.32 | 42.88 | 33.55 |
| system/mem_usage | % | 57.39 | 57.52 | 57.50 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2478 | 2478 | 2478 |
| textures/images | count | 1917 | 1917 | 1917 |
| views/all/batch_sets | count | 111 | 113 | 111 |
| views/all/bins | count | 4151 | 4153 | 4151 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.3 k | 11.3 k | 11.3 k |
| views/all/pipelines | count | 90 | 94 | 90 |
| views/all/pipelines_distinct | count | 89 | 93 | 89 |
| views/all/work_items | count | 11.5 k | 11.5 k | 11.5 k |
| views/cascade/batch_sets | count | 29 | 29 | 29 |
| views/cascade/bins | count | 985 | 985 | 985 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2006 | 2006 | 2006 |
| views/cascade/pipelines | count | 24 | 24 | 24 |
| views/cascade/work_items | count | 2006 | 2006 | 2006 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 82 | 84 | 82 |
| views/main/bins | count | 3166 | 3168 | 3166 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9300 | 9300 | 9300 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 9480 | 9480 | 9480 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 222 | 222 | 222 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0.40 | 231 | 561 |
| anim/events/count | count | 0.09 | 3 | 132 |
| assets/Image/added | count | 0.06 | 16 | 77 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.14 | 47 | 189 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.06 | 25 | 79 |
| assets/StandardMaterial/modified | count | 3.56 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 4.26 | 2026 | 5945 |
| physics/inserts | count | 0.56 | 172 | 786 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.65 | 2 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.01 | 5 | 12 |
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
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.8 ms; slowest 29, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 3.99 | 3994.3 | main/busy/ms 3912, stream/drive/ms 1.78, stream/tick/ms 0.89, surface/advance_cpu_ms 0.72, weather/eval/ms 0.41, surface/cell_upload_ms 0.36, bigspace/total/ms 0.29, script/update/ms 0.26, weather/map/ms 0.17, far/select/ms 0.16, script/input/ms 0.12, bigspace/high_precision/ms 0.04 |
| warm-up | 4.73 | 737.9 | main/busy/ms 3973, render/thread/ms 699, render/busy/ms 580, render/acquire/ms 113, render/graph/ms 9.80, render/present/ms 5.29, stream/drive/ms 4.15, stream/tick/ms 2.07, terrain/update/ms 0.94, surface/advance_cpu_ms 0.72, weather/eval/ms 0.42, bigspace/total/ms 0.39 |
| warm-up | 4.80 | 72.2 | pipelines/wait_ms 998, render/thread/ms 708, render/busy/ms 589, render/acquire/ms 114, main/busy/ms 77.99, pipelines/compile_ms 77.46, pipelines/front_ms 69.54, terrain/update/ms 16.03, render/graph/ms 12.32, pipelines/driver_ms 7.92, render/present/ms 6.06, stream/drive/ms 3.13 |
| warm-up | 5.09 | 94.6 | pipelines/compile_ms 197, pipelines/front_ms 181, main/busy/ms 63.09, render/thread/ms 51.67, render/busy/ms 50.09, pipelines/wait_ms 41.49, pipelines/driver_ms 15.89, render/graph/ms 5.36, render/present/ms 1.35, stream/drive/ms 1.34, stream/tick/ms 1.01, terrain/update/ms 0.75 |
| warm-up | 5.23 | 66.5 | pipelines/wait_ms 407, main/busy/ms 44.76, render/thread/ms 37.24, render/busy/ms 36.25, stream/drive/ms 34.64, render/graph/ms 2.47, pipelines/compile_ms 1.71, pipelines/front_ms 1.14, render/present/ms 0.85, stream/tick/ms 0.77, pipelines/driver_ms 0.58, terrain/update/ms 0.56 |
| warm-up | 5.34 | 43.2 | render/thread/ms 44.84, render/busy/ms 43.07, main/busy/ms 37.98, stream/drive/ms 18.77, render/graph/ms 4.29, render/present/ms 1.40, stream/tick/ms 0.89, terrain/update/ms 0.64, terrain/select/ms 0.52, render/acquire/ms 0.36, bigspace/total/ms 0.24, bigspace/high_precision/ms 0.12 |
| warm-up | 5.46 | 97.5 | pipelines/wait_ms 3844, pipelines/compile_ms 239, pipelines/front_ms 212, main/busy/ms 66.52, stream/drive/ms 50.80, render/thread/ms 50.34, render/busy/ms 49.23, pipelines/driver_ms 27.17, render/graph/ms 2.86, render/present/ms 0.93, stream/tick/ms 0.86, terrain/update/ms 0.73 |
| warm-up | 5.52 | 56.7 | pipelines/wait_ms 3844, pipelines/compile_ms 239, pipelines/front_ms 212, render/thread/ms 77.08, render/busy/ms 46.67, main/busy/ms 46.12, render/present/ms 30.20, pipelines/driver_ms 27.17, stream/drive/ms 24.86, render/graph/ms 2.75, stream/spawn/ms 1.83, stream/tick/ms 0.91 |
| warm-up | 5.55 | 34.6 | render/thread/ms 74.92, main/busy/ms 52.00, render/busy/ms 43.82, render/present/ms 30.81, stream/drive/ms 25.12, render/graph/ms 4.99, stream/spawn/ms 2.96, terrain/update/ms 2.26, stream/tick/ms 0.77, terrain/select/ms 0.41, render/acquire/ms 0.29, bigspace/total/ms 0.26 |
| warm-up | 5.61 | 42.3 | pipelines/wait_ms 526, pipelines/compile_ms 145, pipelines/front_ms 138, render/thread/ms 39.74, render/busy/ms 37.47, main/busy/ms 25.59, pipelines/driver_ms 6.83, render/graph/ms 5.43, stream/drive/ms 5.08, stream/spawn/ms 2.49, render/present/ms 1.97, stream/tick/ms 1.33 |
| warm-up | 5.64 | 29.5 | pipelines/wait_ms 526, pipelines/compile_ms 145, pipelines/front_ms 138, main/busy/ms 54.93, render/thread/ms 44.73, render/busy/ms 43.02, stream/drive/ms 32.88, pipelines/driver_ms 6.83, render/graph/ms 3.33, stream/tick/ms 1.71, render/present/ms 1.43, stream/activate/ms 1.04 |
| warm-up | 5.70 | 62.9 | main/busy/ms 52.49, render/thread/ms 39.67, render/busy/ms 38.30, stream/drive/ms 32.00, terrain/update/ms 4.80, render/graph/ms 2.86, render/present/ms 1.18, stream/tick/ms 1.15, stream/activate/ms 0.89, terrain/select/ms 0.65, physics/insert/ms 0.51, bigspace/total/ms 0.24 |
| warm-up | 5.77 | 39.4 | pipelines/wait_ms 2443, main/busy/ms 79.58, stream/drive/ms 60.21, pipelines/compile_ms 49.67, pipelines/front_ms 46.66, render/thread/ms 25.74, render/busy/ms 23.97, render/graph/ms 3.63, pipelines/driver_ms 3.01, render/present/ms 1.56, stream/spawn/ms 0.98, stream/tick/ms 0.69 |
| warm-up | 5.83 | 68.4 | pipelines/wait_ms 1556, main/busy/ms 99.62, stream/drive/ms 82.68, render/thread/ms 29.84, render/busy/ms 28.13, pipelines/compile_ms 27.86, pipelines/front_ms 26.23, render/graph/ms 3.72, pipelines/driver_ms 1.63, render/present/ms 1.52, stream/tick/ms 0.76, terrain/update/ms 0.71 |
| warm-up | 5.88 | 49.2 | pipelines/wait_ms 4578, pipelines/compile_ms 137, pipelines/front_ms 124, main/busy/ms 55.59, stream/drive/ms 40.82, render/thread/ms 26.59, render/busy/ms 24.88, pipelines/driver_ms 13.50, render/graph/ms 4.06, render/present/ms 1.51, stream/spawn/ms 1.08, stream/activate/ms 1.04 |
| warm-up | 5.94 | 28.0 | main/busy/ms 29.28, render/thread/ms 28.96, render/busy/ms 27.56, stream/drive/ms 11.56, render/graph/ms 3.65, stream/spawn/ms 1.79, stream/tick/ms 1.48, render/present/ms 1.16, terrain/update/ms 0.65, terrain/select/ms 0.52, bigspace/total/ms 0.31, render/acquire/ms 0.24 |
| warm-up | 6.00 | 27.7 | main/busy/ms 27.43, render/thread/ms 20.58, render/busy/ms 19.15, stream/drive/ms 14.71, render/graph/ms 4.00, render/present/ms 1.25, stream/spawn/ms 1.22, stream/tick/ms 0.79, stream/activate/ms 0.74, terrain/update/ms 0.70, terrain/select/ms 0.57, physics/insert/ms 0.37 |
| warm-up | 6.05 | 28.3 | pipelines/wait_ms 5140, pipelines/compile_ms 189, pipelines/front_ms 171, render/thread/ms 32.13, render/busy/ms 30.30, main/busy/ms 24.32, pipelines/driver_ms 18.03, render/graph/ms 5.63, stream/drive/ms 4.40, stream/spawn/ms 2.07, render/present/ms 1.56, stream/tick/ms 1.17 |
| warm-up | 6.10 | 31.1 | pipelines/wait_ms 2195, main/busy/ms 32.42, render/thread/ms 31.09, render/busy/ms 29.66, pipelines/compile_ms 16.79, stream/drive/ms 14.72, pipelines/front_ms 14.14, render/graph/ms 4.45, stream/spawn/ms 3.66, pipelines/driver_ms 2.65, render/present/ms 1.24, stream/tick/ms 1.17 |
| warm-up | 6.34 | 33.2 | pipelines/wait_ms 5218, pipelines/compile_ms 47.32, render/thread/ms 45.45, render/busy/ms 43.47, pipelines/front_ms 40.60, main/busy/ms 22.99, pipelines/driver_ms 6.72, render/graph/ms 6.23, stream/drive/ms 4.21, stream/spawn/ms 2.08, render/present/ms 1.75, stream/tick/ms 0.92 |
| warm-up | 6.38 | 32.6 | pipelines/wait_ms 2736, render/thread/ms 49.81, render/busy/ms 48.44, main/busy/ms 25.68, pipelines/compile_ms 17.63, pipelines/front_ms 15.56, render/graph/ms 6.43, stream/drive/ms 4.49, stream/spawn/ms 2.17, pipelines/driver_ms 2.07, render/present/ms 1.14, stream/tick/ms 0.95 |
| warm-up | 6.52 | 37.6 | pipelines/wait_ms 5752, pipelines/compile_ms 190, pipelines/front_ms 179, render/thread/ms 50.84, render/busy/ms 48.83, main/busy/ms 28.05, pipelines/driver_ms 11.21, render/graph/ms 6.98, stream/drive/ms 6.47, stream/spawn/ms 2.60, render/present/ms 1.74, stream/tick/ms 0.93 |
| warm-up | 6.55 | 36.5 | pipelines/wait_ms 8676, pipelines/compile_ms 233, pipelines/front_ms 220, render/thread/ms 46.29, render/busy/ms 44.70, main/busy/ms 23.77, pipelines/driver_ms 12.99, render/graph/ms 5.42, stream/drive/ms 4.03, stream/spawn/ms 2.24, render/present/ms 1.35, stream/tick/ms 1.05 |
| warm-up | 6.59 | 34.3 | pipelines/wait_ms 4350, pipelines/compile_ms 54.03, pipelines/front_ms 51.35, render/thread/ms 42.08, render/busy/ms 40.86, main/busy/ms 34.83, stream/drive/ms 14.86, render/graph/ms 4.15, pipelines/driver_ms 2.68, stream/spawn/ms 2.19, render/present/ms 1.03, stream/tick/ms 0.82 |
| warm-up | 6.79 | 32.1 | main/busy/ms 43.52, render/thread/ms 16.16, render/busy/ms 14.82, render/graph/ms 5.06, physics/insert/ms 4.69, stream/drive/ms 3.58, physics/step/ms 2.58, stream/spawn/ms 2.08, render/present/ms 1.16, stream/tick/ms 0.82, bigspace/total/ms 0.62, terrain/update/ms 0.48 |
| warm-up | 6.98 | 30.4 | pipelines/wait_ms 5805, pipelines/compile_ms 160, pipelines/front_ms 154, render/thread/ms 35.41, render/busy/ms 33.98, main/busy/ms 27.90, pipelines/driver_ms 6.05, render/graph/ms 5.33, stream/drive/ms 3.78, stream/spawn/ms 2.09, render/present/ms 1.23, anim/evaluate_cpu/ms 0.69 |
| warm-up | 7.08 | 45.7 | main/busy/ms 36.44, render/thread/ms 34.97, render/busy/ms 33.34, stream/drive/ms 13.83, render/graph/ms 5.74, stream/spawn/ms 1.91, render/present/ms 1.38, stream/tick/ms 1.28, bigspace/total/ms 0.93, terrain/update/ms 0.62, terrain/select/ms 0.54, bigspace/high_precision/ms 0.48 |
| warm-up | 8.04 | 35.9 | main/busy/ms 32.04, render/thread/ms 31.77, render/busy/ms 30.33, render/graph/ms 6.01, render/present/ms 1.22, bigspace/total/ms 1.11, stream/drive/ms 0.94, terrain/update/ms 0.57, terrain/select/ms 0.47, stream/tick/ms 0.46, bigspace/high_precision/ms 0.38, far/select/ms 0.34 |
| warm-up | 8.52 | 30.6 | main/busy/ms 36.88, render/thread/ms 18.25, render/busy/ms 17.03, render/graph/ms 6.43, render/present/ms 1.00, stream/drive/ms 0.98, bigspace/total/ms 0.70, stream/tick/ms 0.46, terrain/update/ms 0.43, terrain/select/ms 0.37, far/select/ms 0.29, stream/spawn/ms 0.28 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\h1_old\balmora_night@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **97%** (min 79%) · 2581 MHz (min 2580) · memory 8001 MHz · 88.1 W · 70 °C (max 74) · VRAM 5055 MiB · throttled 100% of samples
