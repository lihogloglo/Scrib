# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1355 frames over 15.0 s · **median 10.96 ms (91 fps)** · mean 11.07 · p95 12.57 · p99 13.73 (1% low 73 fps) · worst 17.92 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 0.0/min)
- warm-up: 557 frames over 10.4 s, worst 2687.6 ms
- hitches (> 27.4 ms): **0** measured, 28 in warm-up
- timed GPU passes: 8.88 ms of a 11.07 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.96 ms, limit 11 ms
- pass worst frame 17.92 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.674 | 2.764 | 3.182 | 0.97 | 0.086 |
| dlss_super_resolution | 2.234 | 2.309 | 2.631 | 0.97 | 0.186 |
| early prepass | 0.619 | 0.639 | 0.985 | 0.97 | 0.115 |
| bloom | 0.412 | 0.426 | 0.786 | 0.97 | 0.067 |
| atmosphere_luts | 0.347 | 0.359 | 0.486 | 0.97 | 0.004 |
| main_transmissive_pass_3d | 0.280 | 0.289 | 0.289 | 0.97 | 0.009 |
| water_velocity | 0.271 | 0.280 | 0.274 | 0.97 | 0.002 |
| uniform_allocation | 0.231 | 0.239 | 0.241 | 0.97 | 0.042 |
| water_sspr | 0.228 | 0.236 | 0.261 | 0.97 | 0.003 |
| ssao | 0.198 | 0.204 | 0.211 | 0.97 | 0.008 |
| bin_unpacking | 0.182 | 0.188 | 0.187 | 0.97 | 0.029 |
| tonemapping | 0.152 | 0.157 | 0.171 | 0.97 | 0.004 |
| water_fft | 0.139 | 0.143 | 0.139 | 0.97 | 0.003 |
| upscaling | 0.137 | 0.142 | 0.169 | 0.97 | 0.005 |
| atmos_froxel_inject | 0.131 | 0.136 | 0.134 | 0.97 | 0.002 |
| clouds_composite | 0.106 | 0.110 | 0.109 | 0.97 | 0.002 |
| clouds_trace | 0.069 | 0.071 | 0.095 | 0.97 | 0.001 |
| clustering | 0.066 | 0.068 | 0.079 | 0.97 | 0.022 |
| render_sky | 0.054 | 0.056 | 0.056 | 0.97 | 0.002 |
| clouds_resolve | 0.049 | 0.051 | 0.053 | 0.97 | 0.001 |
| lightprobe_irradiance_map | 0.049 | 0.432 | 0.429 | 0.11 | 0.001 |
| main_transparent_pass_3d | 0.042 | 0.044 | 0.046 | 0.97 | 0.119 |
| early_mesh_preprocessing | 0.042 | 0.043 | 0.043 | 0.97 | 0.007 |
| water_snapshot | 0.037 | 0.038 | 0.052 | 0.97 | 0.002 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.032 | 0.97 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.231 | 0.234 | 0.11 | 0.005 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.016 | 0.016 | 0.020 | 0.97 | 0.006 |
| atmosphere_environment | 0.011 | 0.011 | 0.014 | 0.97 | 0.004 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.97 | 0.002 |
| night_sky | 0.007 | 0.007 | 0.009 | 0.97 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.97 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.97 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.016 | 0.014 | 0.11 | 0.001 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.11 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.97 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | — | — | — | - | — |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 19.510 | 0.000 | 5864.11 | 26436.4 |
| render/thread/ms | 10.017 | 11.480 | 16.73 | 13573.4 |
| render/busy/ms | 8.797 | 10.430 | 12.38 | 11919.3 |
| main/busy/ms | 7.338 | 8.852 | 10.79 | 9943.5 |
| render/graph/ms | 5.302 | 6.672 | 8.72 | 7184.6 |
| render/acquire/ms | 0.679 | 1.870 | 3.02 | 920.7 |
| render/present/ms | 0.541 | 0.694 | 6.42 | 733.5 |
| pipelines/compile_ms | 0.374 | 0.000 | 126.46 | 507.3 |
| pipelines/front_ms | 0.364 | 0.000 | 123.69 | 493.8 |
| bigspace/total/ms | 0.344 | 0.538 | 3.45 | 465.5 |
| terrain/update/ms | 0.228 | 0.290 | 0.68 | 308.3 |
| terrain/select/ms | 0.197 | 0.248 | 0.62 | 266.8 |
| stream/drive/ms | 0.195 | 0.288 | 0.61 | 264.6 |
| controller/feed/ms | 0.180 | 0.236 | 0.47 | 243.4 |
| far/select/ms | 0.177 | 0.259 | 0.56 | 240.4 |
| stream/tick/ms | 0.166 | 0.250 | 0.55 | 225.3 |
| controller/move/ms | 0.145 | 0.226 | 0.45 | 197.0 |
| bigspace/high_precision/ms | 0.107 | 0.208 | 2.18 | 144.6 |
| bigspace/low_precision/ms | 0.089 | 0.180 | 3.18 | 120.5 |
| anim/evaluate/ms | 0.081 | 0.198 | 1.17 | 110.2 |
| anim/evaluate_cpu/ms | 0.064 | 0.079 | 0.22 | 86.7 |
| physics/step/ms | 0.035 | 0.063 | 0.16 | 47.2 |
| weather/map/ms | 0.016 | 0.022 | 0.06 | 22.2 |
| pipelines/driver_ms | 0.010 | 0.000 | 3.84 | 13.4 |
| weather/eval/ms | 0.010 | 0.013 | 0.07 | 13.4 |
| script/update/ms | 0.008 | 0.011 | 0.12 | 10.7 |
| controller/avoid/ms | 0.006 | 0.008 | 0.03 | 8.5 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 7.3 |
| script/ui/ms | 0.005 | 0.006 | 0.02 | 6.4 |
| anim/apply/ms | 0.005 | 0.006 | 0.01 | 6.3 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.6 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1080 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 118 | 118 | 118 |
| crowd/joints_extracted/count | count | 2958 | 2958 | 2958 |
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
| frame/work/ms | ms | 9.82 | 12.38 | 9.54 |
| gpu/timed/ms | ms | 9.75 | 11.26 | 9.54 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 478.47 M | 478.47 M | 478.47 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d/bins | count | 330 | 330 | 330 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3189 | 3189 | 3189 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 330 | 330 | 330 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3189 | 3189 | 3189 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 32 | 32 | 32 |
| phases/opaque3d/bins | count | 3892 | 3892 | 3892 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 9292 | 9292 | 9292 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 27 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 32 | 32 | 32 |
| phases/opaque3d_prepass/bins | count | 3892 | 3892 | 3892 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 9292 | 9292 | 9292 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 27 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 31 | 31 | 31 |
| phases/shadow/bins | count | 6686 | 6686 | 6686 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 20.0 k | 20.0 k | 20.0 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 31 | 31 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 200 | 200 | 200 |
| pipelines/waiting | count | 0 | 5 | 0 |
| preprocess/bind_groups | count | 0 | 0 | 0 |
| preprocess/bind_groups_created | count | 0 | 0 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 244 | 244 | 244 |
| process/cpu_usage | % | 27.31 | 37.84 | 30.29 |
| process/mem_usage | GiB | 4.40 | 4.50 | 4.50 |
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
| system/cpu_usage | % | 33.91 | 46.82 | 43.47 |
| system/mem_usage | % | 63.41 | 63.61 | 63.49 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2478 | 2478 | 2478 |
| textures/images | count | 1917 | 1917 | 1917 |
| views/all/batch_sets | count | 111 | 111 | 111 |
| views/all/bins | count | 15.1 k | 15.1 k | 15.1 k |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 45.0 k | 45.0 k | 45.0 k |
| views/all/pipelines | count | 99 | 99 | 99 |
| views/all/pipelines_distinct | count | 91 | 91 | 91 |
| views/all/work_items | count | 45.4 k | 45.4 k | 45.4 k |
| views/cascade/batch_sets | count | 31 | 31 | 31 |
| views/cascade/bins | count | 6686 | 6686 | 6686 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 20.0 k | 20.0 k | 20.0 k |
| views/cascade/pipelines | count | 31 | 31 | 31 |
| views/cascade/work_items | count | 20.0 k | 20.0 k | 20.0 k |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 80 | 80 | 80 |
| views/main/bins | count | 8444 | 8444 | 8444 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 25.0 k | 25.0 k | 25.0 k |
| views/main/pipelines | count | 68 | 68 | 68 |
| views/main/work_items | count | 25.4 k | 25.4 k | 25.4 k |
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
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 2 | 139 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.65 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 2 | 1 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.66 | 1 | 900 |
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
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.4 ms; slowest 28, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.69 | 2687.6 | main/busy/ms 2663, surface/advance_cpu_ms 0.87, stream/drive/ms 0.29, surface/cell_upload_ms 0.28, stream/tick/ms 0.08, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.04, weather/map/ms 0.04, weather/eval/ms 0.03, far/select/ms 0.03, script/update/ms 0.02, script/input/ms 0.01 |
| warm-up | 3.30 | 614.4 | main/busy/ms 2700, render/thread/ms 580, render/busy/ms 480, render/acquire/ms 97.66, render/graph/ms 4.52, render/present/ms 2.34, stream/drive/ms 0.87, surface/advance_cpu_ms 0.87, terrain/update/ms 0.53, stream/tick/ms 0.47, terrain/select/ms 0.37, surface/cell_upload_ms 0.28 |
| warm-up | 3.50 | 196.6 | pipelines/wait_ms 1092, render/thread/ms 592, render/busy/ms 491, pipelines/compile_ms 209, pipelines/front_ms 196, render/acquire/ms 97.83, main/busy/ms 56.07, terrain/update/ms 21.08, pipelines/driver_ms 13.26, render/graph/ms 8.50, render/present/ms 3.16, stream/drive/ms 1.26 |
| warm-up | 3.72 | 96.8 | main/busy/ms 69.12, render/thread/ms 51.90, render/busy/ms 50.04, terrain/update/ms 6.77, render/graph/ms 3.99, render/present/ms 1.59, stream/drive/ms 1.10, stream/tick/ms 0.85, terrain/select/ms 0.67, bigspace/total/ms 0.37, render/acquire/ms 0.27, bigspace/low_precision/ms 0.18 |
| warm-up | 3.89 | 58.8 | render/thread/ms 45.36, render/busy/ms 43.87, main/busy/ms 39.81, stream/drive/ms 30.27, render/graph/ms 3.13, render/present/ms 1.30, stream/tick/ms 0.64, terrain/update/ms 0.51, terrain/select/ms 0.40, stream/spawn/ms 0.33, render/acquire/ms 0.20, bigspace/total/ms 0.16 |
| warm-up | 3.95 | 33.0 | pipelines/wait_ms 651, pipelines/compile_ms 200, pipelines/front_ms 186, render/thread/ms 42.31, render/busy/ms 40.00, main/busy/ms 24.47, pipelines/driver_ms 14.29, render/graph/ms 5.20, stream/drive/ms 4.48, render/present/ms 1.89, stream/spawn/ms 1.57, stream/tick/ms 1.50 |
| warm-up | 3.99 | 37.0 | pipelines/wait_ms 233, pipelines/compile_ms 198, pipelines/front_ms 184, render/thread/ms 49.09, render/busy/ms 46.87, main/busy/ms 35.13, stream/drive/ms 15.30, pipelines/driver_ms 13.81, render/graph/ms 4.30, render/present/ms 1.84, stream/tick/ms 1.53, terrain/update/ms 1.04 |
| warm-up | 4.04 | 47.7 | render/thread/ms 57.06, render/busy/ms 55.01, main/busy/ms 32.98, stream/drive/ms 15.82, render/graph/ms 2.82, render/present/ms 1.76, stream/tick/ms 1.24, stream/spawn/ms 1.12, terrain/update/ms 1.10, terrain/select/ms 0.68, stream/activate/ms 0.37, bigspace/total/ms 0.31 |
| warm-up | 4.16 | 102.4 | pipelines/wait_ms 3232, pipelines/compile_ms 269, pipelines/front_ms 246, main/busy/ms 69.99, render/thread/ms 60.22, render/busy/ms 57.18, stream/drive/ms 46.83, pipelines/driver_ms 23.30, render/graph/ms 8.24, stream/spawn/ms 4.01, render/present/ms 2.71, stream/tick/ms 1.22 |
| warm-up | 4.20 | 41.7 | pipelines/wait_ms 4981, pipelines/compile_ms 304, pipelines/front_ms 278, render/thread/ms 67.16, render/busy/ms 64.34, main/busy/ms 38.23, pipelines/driver_ms 25.58, stream/drive/ms 11.28, render/graph/ms 6.10, render/present/ms 2.55, stream/spawn/ms 2.11, stream/tick/ms 1.54 |
| warm-up | 4.30 | 94.0 | pipelines/wait_ms 3110, render/thread/ms 88.76, render/busy/ms 87.66, pipelines/compile_ms 53.26, pipelines/front_ms 49.92, main/busy/ms 30.11, stream/drive/ms 9.96, pipelines/driver_ms 3.34, render/graph/ms 2.42, stream/tick/ms 1.24, stream/activate/ms 1.01, render/present/ms 0.93 |
| warm-up | 4.35 | 41.9 | pipelines/wait_ms 2905, pipelines/compile_ms 127, pipelines/front_ms 121, main/busy/ms 41.20, stream/drive/ms 27.95, render/thread/ms 21.32, render/busy/ms 19.83, pipelines/driver_ms 5.56, render/graph/ms 5.01, render/present/ms 1.32, stream/tick/ms 0.91, stream/activate/ms 0.90 |
| warm-up | 4.49 | 94.0 | pipelines/wait_ms 2758, main/busy/ms 89.22, stream/drive/ms 78.53, pipelines/compile_ms 54.23, pipelines/front_ms 51.71, render/thread/ms 30.93, render/busy/ms 29.45, render/graph/ms 3.13, pipelines/driver_ms 2.52, render/present/ms 1.28, stream/tick/ms 0.74, terrain/update/ms 0.51 |
| warm-up | 4.71 | 39.3 | main/busy/ms 39.04, render/thread/ms 33.86, render/busy/ms 32.47, stream/drive/ms 12.23, render/graph/ms 4.20, terrain/update/ms 3.38, stream/spawn/ms 2.94, render/present/ms 1.19, stream/tick/ms 1.08, terrain/select/ms 0.59, bigspace/total/ms 0.41, stream/activate/ms 0.36 |
| warm-up | 4.74 | 31.2 | render/thread/ms 39.07, render/busy/ms 38.04, main/busy/ms 28.62, terrain/update/ms 8.84, stream/drive/ms 3.99, render/graph/ms 2.75, stream/spawn/ms 2.20, render/present/ms 0.85, stream/tick/ms 0.71, bigspace/total/ms 0.55, terrain/select/ms 0.46, stream/activate/ms 0.36 |
| warm-up | 4.88 | 36.5 | pipelines/wait_ms 2367, render/thread/ms 53.64, render/busy/ms 52.42, main/busy/ms 37.64, pipelines/compile_ms 36.00, pipelines/front_ms 34.25, render/graph/ms 5.81, stream/drive/ms 3.51, stream/spawn/ms 2.05, pipelines/driver_ms 1.75, render/present/ms 0.95, terrain/update/ms 0.68 |
| warm-up | 4.94 | 36.6 | pipelines/wait_ms 1878, pipelines/compile_ms 64.72, pipelines/front_ms 60.88, render/thread/ms 48.64, render/busy/ms 46.48, main/busy/ms 23.33, render/graph/ms 11.32, stream/drive/ms 4.95, pipelines/driver_ms 3.85, stream/spawn/ms 2.36, render/present/ms 1.86, stream/tick/ms 0.69 |
| warm-up | 5.00 | 47.5 | render/thread/ms 49.74, render/busy/ms 48.60, main/busy/ms 21.70, render/graph/ms 4.86, stream/drive/ms 4.17, stream/spawn/ms 2.14, render/present/ms 0.93, stream/tick/ms 0.85, bigspace/total/ms 0.59, terrain/update/ms 0.49, terrain/select/ms 0.39, bigspace/high_precision/ms 0.30 |
| warm-up | 5.06 | 32.3 | pipelines/wait_ms 2670, pipelines/compile_ms 138, pipelines/front_ms 134, render/thread/ms 50.51, render/busy/ms 48.62, render/graph/ms 17.25, main/busy/ms 17.19, stream/drive/ms 4.02, pipelines/driver_ms 3.95, stream/spawn/ms 2.22, render/present/ms 1.58, terrain/update/ms 0.61 |
| warm-up | 5.11 | 43.5 | render/thread/ms 50.17, render/busy/ms 48.90, main/busy/ms 22.52, stream/drive/ms 6.78, render/graph/ms 5.93, stream/spawn/ms 2.00, render/present/ms 0.99, stream/tick/ms 0.76, bigspace/total/ms 0.57, terrain/update/ms 0.56, terrain/select/ms 0.45, bigspace/high_precision/ms 0.33 |
| warm-up | 5.18 | 45.8 | pipelines/wait_ms 1528, pipelines/compile_ms 80.12, pipelines/front_ms 75.71, render/thread/ms 60.72, render/busy/ms 59.02, main/busy/ms 26.68, render/graph/ms 8.09, pipelines/driver_ms 4.41, stream/drive/ms 3.88, stream/spawn/ms 2.29, render/present/ms 1.39, bigspace/total/ms 1.28 |
| warm-up | 5.25 | 41.3 | render/thread/ms 49.21, render/busy/ms 44.79, main/busy/ms 33.03, stream/drive/ms 14.17, render/graph/ms 6.53, render/present/ms 4.23, stream/spawn/ms 2.06, stream/tick/ms 1.00, bigspace/total/ms 0.80, terrain/update/ms 0.69, terrain/select/ms 0.57, bigspace/high_precision/ms 0.43 |
| warm-up | 5.51 | 37.4 | pipelines/wait_ms 9425, pipelines/compile_ms 105, pipelines/front_ms 96.55, render/thread/ms 45.26, main/busy/ms 43.34, render/busy/ms 43.16, render/graph/ms 10.77, pipelines/driver_ms 8.85, stream/drive/ms 7.52, render/present/ms 1.79, stream/spawn/ms 1.03, bigspace/total/ms 0.87 |
| warm-up | 5.55 | 32.5 | pipelines/wait_ms 15.0 k, pipelines/compile_ms 126, pipelines/front_ms 116, render/thread/ms 49.36, render/busy/ms 47.90, main/busy/ms 33.76, pipelines/driver_ms 10.42, render/graph/ms 6.16, stream/drive/ms 4.58, stream/spawn/ms 2.11, render/present/ms 1.27, stream/tick/ms 0.88 |
| warm-up | 5.61 | 36.1 | render/thread/ms 47.59, render/busy/ms 45.54, main/busy/ms 38.93, render/graph/ms 9.88, stream/drive/ms 3.21, render/present/ms 1.85, stream/spawn/ms 1.59, stream/tick/ms 0.62, physics/insert/ms 0.59, terrain/update/ms 0.58, bigspace/total/ms 0.56, terrain/select/ms 0.46 |
| warm-up | 5.76 | 30.4 | pipelines/wait_ms 2013, pipelines/compile_ms 217, pipelines/front_ms 200, render/thread/ms 42.49, render/busy/ms 40.77, main/busy/ms 33.46, pipelines/driver_ms 16.89, stream/drive/ms 11.03, render/graph/ms 10.11, stream/spawn/ms 1.75, render/present/ms 1.44, stream/tick/ms 1.26 |
| warm-up | 5.81 | 45.4 | pipelines/wait_ms 2013, pipelines/compile_ms 217, pipelines/front_ms 200, render/thread/ms 53.87, render/busy/ms 52.76, main/busy/ms 34.08, pipelines/driver_ms 16.89, stream/drive/ms 11.92, render/graph/ms 5.09, stream/spawn/ms 2.35, stream/tick/ms 1.47, render/present/ms 0.86 |
| warm-up | 8.92 | 33.8 | pipelines/wait_ms 14.7 k, pipelines/compile_ms 68.75, pipelines/front_ms 65.68, render/thread/ms 43.13, render/busy/ms 41.74, main/busy/ms 38.06, render/graph/ms 11.15, pipelines/driver_ms 3.06, render/present/ms 1.13, bigspace/total/ms 0.80, physics/insert/ms 0.45, far/select/ms 0.45 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\h1_new\balmora_night@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 6%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **97%** (min 90%) · 2580 MHz (min 2580) · memory 8001 MHz · 87.5 W · 73 °C (max 75) · VRAM 5109 MiB · throttled 100% of samples
