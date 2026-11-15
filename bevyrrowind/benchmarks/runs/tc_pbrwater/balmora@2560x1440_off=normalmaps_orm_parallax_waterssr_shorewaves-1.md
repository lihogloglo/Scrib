# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: waterssr,shorewaves,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1016 frames over 15.0 s · **median 14.76 ms (68 fps)** · mean 14.77 · p95 16.12 · p99 17.09 (1% low 59 fps) · worst 21.80 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 289 frames over 7.5 s, worst 2474.5 ms
- hitches (> 36.9 ms): **0** measured, 20 in warm-up
- timed GPU passes: 12.92 ms of a 14.77 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 14.76 ms, limit 11 ms
- pass worst frame 21.80 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 6.683 | 6.683 | 7.091 | 1.00 | 0.055 |
| early prepass | 0.979 | 0.979 | 1.325 | 1.00 | 0.090 |
| ssao | 0.799 | 0.799 | 1.094 | 1.00 | 0.007 |
| main_transmissive_pass_3d | 0.498 | 0.498 | 0.855 | 1.00 | 0.006 |
| clouds_composite | 0.497 | 0.497 | 0.826 | 1.00 | 0.001 |
| clouds_trace | 0.437 | 0.437 | 0.482 | 1.00 | 0.001 |
| taa | 0.413 | 0.413 | 0.764 | 1.00 | 0.002 |
| atmosphere_luts | 0.362 | 0.362 | 0.644 | 1.00 | 0.003 |
| bloom | 0.337 | 0.337 | 0.671 | 1.00 | 0.060 |
| render_sky | 0.212 | 0.212 | 0.212 | 1.00 | 0.001 |
| clouds_resolve | 0.193 | 0.193 | 0.204 | 1.00 | 0.001 |
| water_snapshot | 0.188 | 0.188 | 0.229 | 1.00 | 0.001 |
| uniform_allocation | 0.182 | 0.182 | 0.183 | 1.00 | 0.030 |
| atmos_froxel_inject | 0.157 | 0.157 | 0.155 | 1.00 | 0.002 |
| water_fft | 0.139 | 0.139 | 0.141 | 1.00 | 0.002 |
| bin_unpacking | 0.138 | 0.138 | 0.142 | 1.00 | 0.022 |
| upscaling | 0.129 | 0.129 | 0.140 | 1.00 | 0.004 |
| water_velocity | 0.121 | 0.121 | 0.124 | 1.00 | 0.001 |
| tonemapping | 0.093 | 0.093 | 0.094 | 1.00 | 0.003 |
| clustering | 0.067 | 0.067 | 0.075 | 1.00 | 0.020 |
| lightprobe_irradiance_map | 0.054 | 0.430 | 0.420 | 0.12 | 0.001 |
| early_mesh_preprocessing | 0.043 | 0.043 | 0.042 | 1.00 | 0.005 |
| atmos_froxel_integrate | 0.032 | 0.032 | 0.035 | 1.00 | 0.001 |
| clouds_shadow | 0.030 | 0.030 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.226 | 0.241 | 0.12 | 0.003 |
| per_view_shadow_pass | 0.024 | 0.024 | 0.031 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.023 | 0.023 | 0.025 | 1.00 | 0.069 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| atmosphere_environment | 0.012 | 0.012 | 0.014 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| night_sky | 0.010 | 0.010 | 0.011 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.003 | 0.004 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 149.549 | 0.000 | 9369.48 | 151942.0 |
| render/thread/ms | 13.859 | 15.164 | 20.65 | 14080.6 |
| render/busy/ms | 7.215 | 7.950 | 10.84 | 7330.4 |
| render/acquire/ms | 6.149 | 7.358 | 9.44 | 6246.9 |
| main/busy/ms | 6.083 | 6.851 | 11.92 | 6180.7 |
| render/graph/ms | 3.895 | 4.501 | 6.22 | 3957.2 |
| pipelines/compile_ms | 1.856 | 0.000 | 139.70 | 1885.3 |
| pipelines/front_ms | 1.792 | 0.000 | 135.13 | 1820.9 |
| render/present/ms | 0.495 | 0.585 | 7.16 | 503.3 |
| bigspace/total/ms | 0.280 | 0.364 | 1.07 | 284.2 |
| terrain/update/ms | 0.225 | 0.277 | 0.50 | 228.2 |
| controller/move/ms | 0.204 | 0.405 | 0.82 | 207.4 |
| terrain/select/ms | 0.196 | 0.241 | 0.47 | 199.0 |
| stream/drive/ms | 0.178 | 0.206 | 0.56 | 181.0 |
| stream/tick/ms | 0.151 | 0.177 | 0.54 | 153.9 |
| controller/feed/ms | 0.150 | 0.185 | 0.36 | 152.4 |
| far/select/ms | 0.150 | 0.172 | 0.41 | 152.2 |
| bigspace/high_precision/ms | 0.090 | 0.110 | 0.34 | 91.1 |
| pipelines/driver_ms | 0.063 | 0.000 | 4.57 | 64.5 |
| bigspace/low_precision/ms | 0.063 | 0.079 | 0.82 | 64.3 |
| anim/evaluate_cpu/ms | 0.059 | 0.071 | 0.42 | 59.5 |
| anim/evaluate/ms | 0.056 | 0.068 | 0.41 | 57.2 |
| physics/step/ms | 0.043 | 0.061 | 0.41 | 44.1 |
| weather/map/ms | 0.024 | 0.033 | 0.23 | 24.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.04 | 10.9 |
| script/update/ms | 0.007 | 0.008 | 0.02 | 7.0 |
| controller/avoid/ms | 0.005 | 0.007 | 0.14 | 5.3 |
| script/input/ms | 0.005 | 0.006 | 0.01 | 5.1 |
| anim/apply/ms | 0.004 | 0.005 | 0.02 | 3.8 |
| script/ui/ms | 0.003 | 0.004 | 0.02 | 3.1 |
| physics/insert/ms | 0.002 | 0.013 | 0.23 | 1.9 |
| controller/write/ms | 0.002 | 0.002 | 0.00 | 1.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 0.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.3 |
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
| anim/a2/count | count | 0 | 5 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 22 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 11 | 8 |
| crowd/c0/joints/count | count | 272 | 374 | 272 |
| crowd/c0/meshes/count | count | 48 | 66 | 48 |
| crowd/c1/count | count | 122 | 124 | 123 |
| crowd/c1/joints/count | count | 4848 | 4939 | 4880 |
| crowd/c1/meshes/count | count | 1080 | 1110 | 1088 |
| crowd/c2/count | count | 284 | 284 | 283 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 6036 | 4212 |
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
| frame/work/ms | ms | 13.63 | 16.70 | 13.66 |
| gpu/timed/ms | ms | 13.63 | 16.70 | 13.66 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 458.63 M | 458.63 M | 458.63 M |
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
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 22 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 31 | 33 | 31 |
| phases/shadow/bins | count | 2029 | 2299 | 2029 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5346 | 5616 | 5346 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 31 | 33 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 90 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 184 | 184 | 184 |
| pipelines/waiting | count | 0 | 35 | 0 |
| preprocess/bind_groups | count | 0 | 100 | 0 |
| preprocess/bind_groups_created | count | 0 | 100 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 201 | 201 | 201 |
| process/cpu_usage | % | 20.19 | 32.37 | 18.47 |
| process/mem_usage | GiB | 4.14 | 4.16 | 4.16 |
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
| stream/latency/s | s | 0.05 | 0.05 | 0.05 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 25.91 | 46.71 | 27.78 |
| system/mem_usage | % | 72.64 | 72.82 | 72.82 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2452 | 2452 | 2452 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 89 | 91 | 89 |
| views/all/bins | count | 5199 | 5415 | 5199 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.8 k | 14.6 k |
| views/all/pipelines | count | 85 | 87 | 85 |
| views/all/pipelines_distinct | count | 75 | 79 | 75 |
| views/all/work_items | count | 14.8 k | 15.0 k | 14.8 k |
| views/cascade/batch_sets | count | 31 | 33 | 31 |
| views/cascade/bins | count | 2029 | 2299 | 2029 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5346 | 5616 | 5346 |
| views/cascade/pipelines | count | 31 | 33 | 31 |
| views/cascade/work_items | count | 5346 | 5616 | 5346 |
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
| views/main/bins | count | 3170 | 3170 | 3170 |
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
| anim/bones_written/count | count | 0.16 | 165 | 165 |
| anim/events/count | count | 0.12 | 3 | 117 |
| assets/Image/added | count | 0.02 | 13 | 16 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.06 | 34 | 59 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.02 | 16 | 19 |
| assets/StandardMaterial/modified | count | 4.87 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.52 | 892 | 1549 |
| physics/inserts | count | 0.10 | 36 | 106 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.89 | 2 | 900 |
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

## Hitches (> 36.9 ms; slowest 20, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2474.5 | main/busy/ms 2453, surface/advance_cpu_ms 67.81, surface/cell_upload_ms 0.75, stream/drive/ms 0.30, weather/map/ms 0.19, bigspace/total/ms 0.12, stream/tick/ms 0.09, weather/eval/ms 0.04, bigspace/high_precision/ms 0.03, script/update/ms 0.03, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.02 | 544.3 | main/busy/ms 2486, render/thread/ms 514, render/busy/ms 494, surface/advance_cpu_ms 67.81, render/acquire/ms 18.39, render/graph/ms 4.18, render/present/ms 1.88, stream/drive/ms 0.92, surface/cell_upload_ms 0.75, terrain/update/ms 0.54, stream/tick/ms 0.50, terrain/select/ms 0.34 |
| warm-up | 3.13 | 111.1 | pipelines/wait_ms 991, render/thread/ms 551, render/busy/ms 530, pipelines/compile_ms 90.54, pipelines/front_ms 83.37, main/busy/ms 49.77, render/graph/ms 33.65, terrain/update/ms 18.70, render/acquire/ms 18.55, pipelines/driver_ms 7.16, render/present/ms 3.11, stream/drive/ms 1.17 |
| warm-up | 3.34 | 101.6 | main/busy/ms 62.51, render/thread/ms 57.42, render/busy/ms 55.69, render/graph/ms 3.38, stream/drive/ms 1.64, render/present/ms 1.50, stream/tick/ms 0.85, terrain/update/ms 0.69, terrain/select/ms 0.54, stream/spawn/ms 0.32, bigspace/total/ms 0.28, render/acquire/ms 0.23 |
| warm-up | 3.48 | 69.1 | render/thread/ms 44.01, render/busy/ms 42.67, main/busy/ms 41.00, stream/drive/ms 29.27, render/graph/ms 1.66, render/present/ms 1.19, terrain/update/ms 0.84, terrain/select/ms 0.68, stream/tick/ms 0.64, bigspace/total/ms 0.36, bigspace/high_precision/ms 0.18, stream/spawn/ms 0.17 |
| warm-up | 3.59 | 39.4 | pipelines/wait_ms 1018, pipelines/compile_ms 250, pipelines/front_ms 231, main/busy/ms 63.76, render/thread/ms 49.78, render/busy/ms 48.19, stream/drive/ms 46.27, pipelines/driver_ms 18.72, render/graph/ms 3.00, render/present/ms 1.22, stream/tick/ms 0.95, terrain/update/ms 0.66 |
| warm-up | 3.68 | 86.1 | pipelines/wait_ms 2307, pipelines/compile_ms 103, pipelines/front_ms 99.63, render/thread/ms 71.40, render/busy/ms 70.00, main/busy/ms 65.03, stream/drive/ms 46.49, pipelines/driver_ms 3.74, render/graph/ms 2.14, render/present/ms 1.11, stream/spawn/ms 1.11, terrain/update/ms 0.58 |
| warm-up | 3.74 | 47.0 | pipelines/wait_ms 1052, pipelines/compile_ms 88.47, pipelines/front_ms 82.00, main/busy/ms 40.01, render/thread/ms 36.35, render/busy/ms 34.61, stream/drive/ms 21.72, pipelines/driver_ms 6.47, render/graph/ms 4.83, render/present/ms 1.51, stream/spawn/ms 1.49, terrain/update/ms 0.95 |
| warm-up | 3.80 | 45.0 | render/thread/ms 45.98, render/busy/ms 44.13, main/busy/ms 26.24, stream/drive/ms 5.64, render/graph/ms 4.21, stream/spawn/ms 2.46, render/present/ms 1.51, stream/tick/ms 1.29, stream/activate/ms 0.80, terrain/update/ms 0.74, terrain/select/ms 0.55, physics/insert/ms 0.39 |
| warm-up | 3.84 | 39.3 | main/busy/ms 54.59, render/thread/ms 53.48, render/busy/ms 52.06, stream/drive/ms 32.08, render/graph/ms 3.28, stream/spawn/ms 2.76, stream/tick/ms 1.73, render/present/ms 1.19, stream/activate/ms 0.86, terrain/update/ms 0.79, terrain/select/ms 0.58, physics/insert/ms 0.39 |
| warm-up | 3.97 | 130.5 | pipelines/wait_ms 2154, pipelines/compile_ms 338, pipelines/front_ms 213, pipelines/driver_ms 125, render/thread/ms 108, render/busy/ms 107, main/busy/ms 52.30, stream/drive/ms 30.71, render/graph/ms 2.10, stream/spawn/ms 2.07, stream/tick/ms 1.22, render/present/ms 0.82 |
| warm-up | 4.09 | 64.9 | pipelines/wait_ms 3504, pipelines/compile_ms 148, pipelines/front_ms 143, main/busy/ms 64.64, stream/drive/ms 51.03, render/thread/ms 21.24, render/busy/ms 20.01, pipelines/driver_ms 4.58, render/graph/ms 2.01, render/present/ms 0.98, stream/tick/ms 0.78, stream/spawn/ms 0.61 |
| warm-up | 4.16 | 55.9 | pipelines/wait_ms 893, main/busy/ms 53.03, stream/drive/ms 35.34, render/thread/ms 20.79, render/busy/ms 19.71, pipelines/compile_ms 18.20, pipelines/front_ms 17.45, stream/spawn/ms 2.39, render/graph/ms 2.35, render/present/ms 0.90, terrain/update/ms 0.79, pipelines/driver_ms 0.75 |
| warm-up | 4.71 | 41.5 | pipelines/wait_ms 9138, render/thread/ms 51.66, render/busy/ms 44.66, main/busy/ms 36.62, pipelines/compile_ms 33.51, pipelines/front_ms 29.94, stream/drive/ms 7.40, render/present/ms 6.74, render/graph/ms 4.99, pipelines/driver_ms 3.57, stream/spawn/ms 2.98, physics/insert/ms 0.90 |
| warm-up | 4.82 | 77.3 | pipelines/wait_ms 17.6 k, pipelines/compile_ms 143, pipelines/front_ms 131, render/thread/ms 78.50, render/busy/ms 76.39, main/busy/ms 72.01, pipelines/driver_ms 11.55, render/graph/ms 5.88, stream/drive/ms 5.15, terrain/update/ms 3.23, stream/spawn/ms 2.89, render/present/ms 1.61 |
| warm-up | 4.88 | 61.6 | pipelines/wait_ms 9774, render/thread/ms 99.52, render/busy/ms 97.85, pipelines/compile_ms 94.37, main/busy/ms 90.11, pipelines/front_ms 89.34, stream/drive/ms 15.26, terrain/update/ms 9.85, pipelines/driver_ms 5.03, render/graph/ms 3.96, stream/spawn/ms 2.23, render/present/ms 1.15 |
| warm-up | 4.92 | 38.6 | pipelines/wait_ms 5156, pipelines/compile_ms 93.21, pipelines/front_ms 87.26, render/thread/ms 74.83, render/busy/ms 73.66, main/busy/ms 57.69, stream/drive/ms 15.12, terrain/update/ms 8.75, pipelines/driver_ms 5.95, render/graph/ms 3.38, stream/spawn/ms 2.13, render/present/ms 0.96 |
| warm-up | 5.33 | 52.0 | pipelines/wait_ms 9398, pipelines/compile_ms 169, pipelines/front_ms 156, render/thread/ms 59.87, render/busy/ms 58.15, main/busy/ms 31.03, pipelines/driver_ms 13.20, render/graph/ms 6.75, stream/drive/ms 4.66, stream/spawn/ms 2.59, render/present/ms 1.50, bigspace/total/ms 1.47 |
| warm-up | 5.41 | 45.8 | pipelines/wait_ms 6668, pipelines/compile_ms 49.79, pipelines/front_ms 47.15, render/thread/ms 43.00, main/busy/ms 42.37, render/busy/ms 41.02, render/graph/ms 7.62, stream/drive/ms 2.71, pipelines/driver_ms 2.65, render/present/ms 1.72, stream/spawn/ms 1.49, bigspace/total/ms 0.84 |
| warm-up | 6.19 | 37.1 | pipelines/wait_ms 5739, pipelines/compile_ms 60.11, pipelines/front_ms 57.66, main/busy/ms 42.85, render/thread/ms 38.22, render/busy/ms 36.95, render/graph/ms 6.51, pipelines/driver_ms 2.45, stream/drive/ms 1.04, render/present/ms 1.02, bigspace/total/ms 0.99, physics/insert/ms 0.75 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\claude\3866c27c11752a1d\4ff90442-bcd4-4820-b03c-4dd8a81992a4\scratchpad\bin\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\tc_pbrwater\balmora@2560x1440_off=normalmaps_orm_parallax_waterssr_shorewaves-1 --bench-baseline none --off normalmaps,orm,parallax,waterssr,shorewaves
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 13%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **98%** (min 98%) · 2573 MHz (min 2565) · memory 8001 MHz · 98.2 W · 79 °C (max 81) · VRAM 4891 MiB · throttled 100% of samples
