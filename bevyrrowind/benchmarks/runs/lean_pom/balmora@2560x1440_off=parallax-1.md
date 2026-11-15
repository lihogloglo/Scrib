# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1377 frames over 15.0 s · **median 10.81 ms (93 fps)** · mean 10.90 · p95 12.40 · p99 14.06 (1% low 71 fps) · worst 27.62 ms · 7 spikes over 1.5× median · 24.0 stutters/min (work 44.0/min)
- warm-up: 323 frames over 7.6 s, worst 2662.9 ms
- hitches (> 27.0 ms): **1** measured, 28 in warm-up
- timed GPU passes: 8.66 ms of a 10.90 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.81 ms, limit 11 ms
- pass worst frame 27.62 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.617 | 2.691 | 3.334 | 0.97 | 0.064 |
| dlss_super_resolution | 2.167 | 2.229 | 2.765 | 0.97 | 0.189 |
| early prepass | 0.625 | 0.643 | 0.989 | 0.97 | 0.106 |
| atmosphere_luts | 0.355 | 0.365 | 0.574 | 0.97 | 0.004 |
| bloom | 0.325 | 0.342 | 0.493 | 0.95 | 0.068 |
| main_transmissive_pass_3d | 0.285 | 0.293 | 0.302 | 0.97 | 0.009 |
| water_velocity | 0.256 | 0.263 | 0.280 | 0.97 | 0.002 |
| water_sspr | 0.234 | 0.240 | 0.377 | 0.97 | 0.002 |
| ssao | 0.203 | 0.209 | 0.213 | 0.97 | 0.008 |
| uniform_allocation | 0.188 | 0.193 | 0.198 | 0.97 | 0.036 |
| atmos_froxel_inject | 0.152 | 0.156 | 0.154 | 0.97 | 0.002 |
| bin_unpacking | 0.149 | 0.153 | 0.157 | 0.97 | 0.026 |
| clouds_trace | 0.145 | 0.149 | 0.173 | 0.97 | 0.001 |
| water_fft | 0.140 | 0.144 | 0.140 | 0.97 | 0.003 |
| upscaling | 0.117 | 0.121 | 0.134 | 0.97 | 0.004 |
| clouds_composite | 0.115 | 0.118 | 0.117 | 0.97 | 0.002 |
| tonemapping | 0.095 | 0.098 | 0.107 | 0.97 | 0.004 |
| clustering | 0.067 | 0.069 | 0.077 | 0.97 | 0.023 |
| render_sky | 0.054 | 0.056 | 0.056 | 0.97 | 0.002 |
| lightprobe_irradiance_map | 0.054 | 0.450 | 0.760 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.052 | 0.054 | 0.97 | 0.001 |
| early_mesh_preprocessing | 0.043 | 0.044 | 0.042 | 0.97 | 0.006 |
| water_snapshot | 0.039 | 0.041 | 0.056 | 0.97 | 0.002 |
| clouds_shadow | 0.031 | 0.032 | 0.035 | 0.97 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.233 | 0.234 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.031 | 0.97 | 0.001 |
| main_transparent_pass_3d | 0.024 | 0.024 | 0.027 | 0.97 | 0.075 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.94 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.023 | 0.025 | 0.97 | 0.007 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 0.97 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.012 | 0.93 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.97 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.009 | 0.97 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.97 | 0.003 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 146.202 | 0.000 | 13093.27 | 201320.7 |
| render/thread/ms | 9.935 | 11.385 | 26.71 | 13680.4 |
| render/busy/ms | 8.504 | 10.087 | 26.09 | 11710.3 |
| main/busy/ms | 7.545 | 9.029 | 24.46 | 10389.9 |
| render/graph/ms | 5.144 | 6.415 | 23.02 | 7082.9 |
| pipelines/compile_ms | 1.953 | 0.000 | 149.46 | 2689.5 |
| pipelines/front_ms | 1.887 | 0.000 | 144.56 | 2598.8 |
| render/acquire/ms | 0.922 | 2.330 | 3.68 | 1269.0 |
| render/present/ms | 0.509 | 0.624 | 11.87 | 701.1 |
| bigspace/total/ms | 0.367 | 0.738 | 2.94 | 505.6 |
| terrain/update/ms | 0.230 | 0.299 | 0.65 | 316.9 |
| terrain/select/ms | 0.200 | 0.257 | 0.61 | 275.0 |
| stream/drive/ms | 0.199 | 0.296 | 0.60 | 274.4 |
| far/select/ms | 0.172 | 0.257 | 0.61 | 237.5 |
| stream/tick/ms | 0.170 | 0.257 | 0.57 | 234.6 |
| controller/feed/ms | 0.169 | 0.233 | 0.51 | 233.1 |
| controller/move/ms | 0.128 | 0.206 | 0.53 | 176.0 |
| bigspace/high_precision/ms | 0.124 | 0.261 | 2.74 | 170.1 |
| anim/evaluate/ms | 0.094 | 0.190 | 1.35 | 128.9 |
| bigspace/low_precision/ms | 0.088 | 0.217 | 1.95 | 121.0 |
| pipelines/driver_ms | 0.066 | 0.000 | 4.91 | 90.7 |
| anim/evaluate_cpu/ms | 0.057 | 0.075 | 0.31 | 78.0 |
| physics/step/ms | 0.034 | 0.058 | 0.94 | 46.1 |
| weather/map/ms | 0.025 | 0.034 | 0.20 | 34.1 |
| physics/insert/ms | 0.013 | 0.076 | 0.65 | 18.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.10 | 15.6 |
| script/update/ms | 0.007 | 0.011 | 0.02 | 10.2 |
| controller/avoid/ms | 0.006 | 0.007 | 0.03 | 7.7 |
| script/input/ms | 0.005 | 0.006 | 0.17 | 7.2 |
| script/ui/ms | 0.005 | 0.007 | 0.11 | 6.6 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 5.7 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.04 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.0 |
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
| anim/a2/count | count | 0 | 13 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 35 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 20 | 8 |
| crowd/c0/joints/count | count | 272 | 680 | 272 |
| crowd/c0/meshes/count | count | 48 | 120 | 48 |
| crowd/c1/count | count | 122 | 124 | 122 |
| crowd/c1/joints/count | count | 4848 | 4977 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1132 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 6113 | 4212 |
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
| frame/work/ms | ms | 9.60 | 26.09 | 9.42 |
| gpu/timed/ms | ms | 9.55 | 11.29 | 9.42 |
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
| phases/opaque3d/batch_sets | count | 25 | 27 | 25 |
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 24 | 26 | 24 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 25 | 27 | 25 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 24 | 26 | 24 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 33 | 33 | 33 |
| phases/shadow/bins | count | 2029 | 2029 | 2028 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5346 | 5346 | 5345 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 33 | 33 | 33 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 731 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 189 | 189 | 189 |
| pipelines/waiting | count | 0 | 49 | 0 |
| preprocess/bind_groups | count | 0 | 211 | 0 |
| preprocess/bind_groups_created | count | 0 | 211 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 217 | 217 | 217 |
| process/cpu_usage | % | 29.60 | 43.75 | 31.22 |
| process/mem_usage | GiB | 4.26 | 4.40 | 4.38 |
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
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
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
| system/cpu_usage | % | 36.15 | 58.46 | 34.31 |
| system/mem_usage | % | 72.61 | 72.99 | 72.93 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2480 | 2480 | 2480 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 97 | 101 | 97 |
| views/all/bins | count | 5199 | 5199 | 5198 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.6 k | 14.6 k |
| views/all/pipelines | count | 93 | 97 | 93 |
| views/all/pipelines_distinct | count | 83 | 87 | 83 |
| views/all/work_items | count | 14.8 k | 14.8 k | 14.8 k |
| views/cascade/batch_sets | count | 33 | 33 | 33 |
| views/cascade/bins | count | 2029 | 2029 | 2028 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5346 | 5346 | 5345 |
| views/cascade/pipelines | count | 33 | 33 | 33 |
| views/cascade/work_items | count | 5346 | 5346 | 5345 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 64 | 68 | 64 |
| views/main/bins | count | 3170 | 3170 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 60 | 64 | 60 |
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
| anim/bones_written/count | count | 0.79 | 330 | 1089 |
| anim/events/count | count | 0.11 | 3 | 145 |
| assets/Image/added | count | 0.08 | 25 | 110 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.22 | 65 | 306 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.09 | 27 | 123 |
| assets/StandardMaterial/modified | count | 3.59 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 6.58 | 1714 | 9063 |
| physics/inserts | count | 0.55 | 145 | 763 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.65 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 2 | 2 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 2 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.0 ms; slowest 29, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.66 | 2662.9 | main/busy/ms 2639, surface/advance_cpu_ms 66.92, surface/cell_upload_ms 0.78, stream/drive/ms 0.34, weather/map/ms 0.12, bigspace/total/ms 0.12, stream/tick/ms 0.10, bigspace/high_precision/ms 0.08, weather/eval/ms 0.03, script/update/ms 0.03, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.36 | 698.1 | main/busy/ms 2673, render/thread/ms 657, render/busy/ms 536, render/acquire/ms 119, surface/advance_cpu_ms 66.92, render/graph/ms 4.83, render/present/ms 2.03, stream/drive/ms 1.11, surface/cell_upload_ms 0.78, stream/tick/ms 0.58, terrain/update/ms 0.34, bigspace/total/ms 0.24 |
| warm-up | 3.45 | 84.8 | pipelines/wait_ms 853, render/thread/ms 668, render/busy/ms 545, render/acquire/ms 119, pipelines/compile_ms 94.28, pipelines/front_ms 84.70, main/busy/ms 52.94, terrain/update/ms 19.09, pipelines/driver_ms 9.58, render/graph/ms 8.11, render/present/ms 3.05, stream/drive/ms 1.30 |
| warm-up | 3.65 | 81.1 | main/busy/ms 57.39, render/thread/ms 46.68, render/busy/ms 45.03, render/graph/ms 4.33, render/present/ms 1.34, stream/drive/ms 1.13, stream/tick/ms 0.95, terrain/update/ms 0.68, terrain/select/ms 0.54, render/acquire/ms 0.31, bigspace/total/ms 0.23, weather/map/ms 0.13 |
| warm-up | 3.69 | 29.7 | pipelines/compile_ms 249, pipelines/front_ms 236, render/thread/ms 35.58, render/busy/ms 33.91, main/busy/ms 18.08, pipelines/driver_ms 12.09, render/graph/ms 4.59, stream/drive/ms 2.67, render/present/ms 1.42, stream/tick/ms 1.26, stream/spawn/ms 0.90, terrain/update/ms 0.68 |
| warm-up | 3.83 | 49.4 | render/thread/ms 57.13, render/busy/ms 55.75, main/busy/ms 43.06, stream/drive/ms 31.57, render/graph/ms 4.27, render/present/ms 1.14, terrain/update/ms 0.73, stream/tick/ms 0.58, terrain/select/ms 0.57, bigspace/total/ms 0.38, render/acquire/ms 0.24, bigspace/low_precision/ms 0.21 |
| warm-up | 3.88 | 44.2 | render/thread/ms 33.25, render/busy/ms 31.32, main/busy/ms 31.18, stream/drive/ms 16.47, render/graph/ms 4.60, render/present/ms 1.75, stream/spawn/ms 1.59, stream/tick/ms 1.11, terrain/update/ms 0.58, terrain/select/ms 0.47, bigspace/total/ms 0.30, render/acquire/ms 0.18 |
| warm-up | 3.91 | 27.9 | pipelines/wait_ms 2003, pipelines/compile_ms 196, pipelines/front_ms 178, main/busy/ms 70.83, stream/drive/ms 55.06, render/thread/ms 47.09, render/busy/ms 44.67, pipelines/driver_ms 17.74, render/graph/ms 3.60, render/present/ms 2.15, stream/tick/ms 1.88, stream/spawn/ms 1.77 |
| warm-up | 3.99 | 81.1 | pipelines/wait_ms 7451, pipelines/compile_ms 269, pipelines/front_ms 247, main/busy/ms 105, stream/drive/ms 89.57, render/thread/ms 48.53, render/busy/ms 46.50, pipelines/driver_ms 22.10, render/graph/ms 2.30, render/present/ms 1.78, stream/spawn/ms 1.59, stream/tick/ms 1.51 |
| warm-up | 4.03 | 39.6 | pipelines/wait_ms 6475, pipelines/compile_ms 118, pipelines/front_ms 111, main/busy/ms 88.27, stream/drive/ms 74.33, render/thread/ms 43.07, render/busy/ms 41.80, pipelines/driver_ms 6.76, render/graph/ms 3.11, stream/spawn/ms 1.67, render/present/ms 1.03, stream/tick/ms 0.85 |
| warm-up | 4.08 | 52.5 | pipelines/wait_ms 3107, pipelines/compile_ms 89.14, pipelines/front_ms 84.46, main/busy/ms 52.34, stream/drive/ms 40.35, render/thread/ms 31.48, render/busy/ms 30.20, pipelines/driver_ms 4.69, render/graph/ms 3.33, render/present/ms 1.04, stream/tick/ms 0.83, terrain/update/ms 0.49 |
| warm-up | 4.14 | 49.0 | main/busy/ms 37.25, render/thread/ms 30.25, render/busy/ms 28.71, stream/drive/ms 22.67, render/graph/ms 4.73, render/present/ms 1.33, stream/tick/ms 1.09, stream/spawn/ms 1.05, terrain/update/ms 0.53, stream/activate/ms 0.47, terrain/select/ms 0.41, bigspace/total/ms 0.31 |
| warm-up | 4.20 | 46.5 | pipelines/wait_ms 2563, pipelines/compile_ms 150, pipelines/front_ms 131, render/thread/ms 41.68, render/busy/ms 39.88, main/busy/ms 23.81, pipelines/driver_ms 19.16, render/graph/ms 4.43, stream/drive/ms 3.42, render/present/ms 1.55, stream/spawn/ms 1.19, terrain/update/ms 1.04 |
| warm-up | 4.24 | 34.6 | pipelines/wait_ms 5318, pipelines/compile_ms 189, pipelines/front_ms 167, render/thread/ms 52.60, render/busy/ms 50.62, main/busy/ms 26.62, pipelines/driver_ms 22.36, stream/drive/ms 6.53, render/graph/ms 5.47, stream/spawn/ms 3.85, render/present/ms 1.74, stream/tick/ms 0.88 |
| warm-up | 4.33 | 43.5 | main/busy/ms 48.12, render/thread/ms 29.77, render/busy/ms 28.55, stream/drive/ms 28.19, render/graph/ms 2.94, stream/spawn/ms 2.27, render/present/ms 1.06, terrain/update/ms 0.74, stream/tick/ms 0.62, terrain/select/ms 0.54, stream/activate/ms 0.49, bigspace/total/ms 0.39 |
| warm-up | 4.55 | 31.4 | pipelines/wait_ms 3692, pipelines/compile_ms 186, pipelines/front_ms 169, render/thread/ms 37.56, render/busy/ms 35.30, main/busy/ms 33.88, pipelines/driver_ms 16.57, render/graph/ms 7.11, stream/drive/ms 6.83, stream/spawn/ms 3.39, render/present/ms 1.98, stream/activate/ms 1.25 |
| warm-up | 4.58 | 29.0 | pipelines/wait_ms 4103, pipelines/compile_ms 59.26, pipelines/front_ms 55.08, render/thread/ms 39.67, render/busy/ms 37.83, main/busy/ms 32.51, stream/drive/ms 6.65, render/graph/ms 5.98, pipelines/driver_ms 4.18, stream/spawn/ms 3.35, render/present/ms 1.58, stream/activate/ms 1.13 |
| warm-up | 4.61 | 31.9 | pipelines/wait_ms 2177, main/busy/ms 44.51, render/thread/ms 41.46, render/busy/ms 39.67, pipelines/compile_ms 16.43, pipelines/front_ms 15.46, render/graph/ms 5.45, stream/drive/ms 4.50, stream/spawn/ms 2.49, render/present/ms 1.46, physics/insert/ms 1.11, pipelines/driver_ms 0.97 |
| warm-up | 4.65 | 43.9 | main/busy/ms 48.44, render/thread/ms 48.37, render/busy/ms 46.69, render/graph/ms 4.86, stream/drive/ms 3.95, stream/spawn/ms 2.48, render/present/ms 1.38, physics/insert/ms 1.10, terrain/update/ms 0.68, stream/tick/ms 0.67, bigspace/total/ms 0.58, terrain/select/ms 0.52 |
| warm-up | 4.85 | 30.7 | pipelines/wait_ms 1822, pipelines/compile_ms 143, pipelines/front_ms 136, render/thread/ms 49.32, render/busy/ms 47.60, main/busy/ms 19.32, pipelines/driver_ms 7.73, render/graph/ms 6.61, stream/drive/ms 3.74, stream/spawn/ms 2.33, render/present/ms 1.52, stream/tick/ms 0.71 |
| warm-up | 4.88 | 27.1 | pipelines/wait_ms 5816, pipelines/compile_ms 179, pipelines/front_ms 168, render/thread/ms 50.84, render/busy/ms 48.78, main/busy/ms 23.31, pipelines/driver_ms 10.93, render/graph/ms 9.15, stream/drive/ms 4.20, stream/spawn/ms 2.31, render/present/ms 1.85, stream/tick/ms 0.84 |
| warm-up | 4.91 | 37.7 | pipelines/wait_ms 9268, pipelines/compile_ms 70.48, pipelines/front_ms 65.31, render/thread/ms 47.88, render/busy/ms 46.42, main/busy/ms 24.03, render/graph/ms 7.48, pipelines/driver_ms 5.17, stream/drive/ms 4.01, stream/spawn/ms 2.05, render/present/ms 1.25, stream/tick/ms 0.76 |
| warm-up | 5.02 | 31.3 | pipelines/wait_ms 8537, pipelines/compile_ms 100, pipelines/front_ms 81.32, render/thread/ms 50.24, render/busy/ms 48.92, main/busy/ms 36.64, pipelines/driver_ms 18.84, stream/drive/ms 5.64, render/graph/ms 4.83, stream/spawn/ms 2.18, render/present/ms 1.09, bigspace/total/ms 0.79 |
| warm-up | 5.06 | 41.5 | pipelines/wait_ms 2812, render/thread/ms 58.68, render/busy/ms 56.85, main/busy/ms 46.88, pipelines/compile_ms 24.76, pipelines/front_ms 12.62, pipelines/driver_ms 12.14, render/graph/ms 6.55, stream/drive/ms 5.19, stream/spawn/ms 2.20, render/present/ms 1.46, bigspace/total/ms 0.92 |
| warm-up | 5.10 | 48.2 | render/thread/ms 63.99, render/busy/ms 61.98, main/busy/ms 60.83, render/graph/ms 9.08, stream/drive/ms 3.74, stream/spawn/ms 2.54, render/present/ms 1.53, bigspace/total/ms 0.87, terrain/update/ms 0.83, stream/tick/ms 0.68, terrain/select/ms 0.65, render/acquire/ms 0.47 |
| warm-up | 5.16 | 55.2 | render/thread/ms 68.98, render/busy/ms 67.42, main/busy/ms 63.89, stream/drive/ms 13.62, render/graph/ms 7.38, stream/spawn/ms 2.40, render/present/ms 1.25, terrain/update/ms 0.86, bigspace/total/ms 0.71, terrain/select/ms 0.69, stream/tick/ms 0.65, render/acquire/ms 0.32 |
| warm-up | 5.30 | 27.5 | main/busy/ms 32.60, render/thread/ms 22.73, render/busy/ms 21.41, stream/drive/ms 16.10, render/graph/ms 6.53, stream/spawn/ms 2.03, render/present/ms 1.12, stream/tick/ms 0.72, bigspace/total/ms 0.66, terrain/update/ms 0.52, terrain/select/ms 0.44, bigspace/high_precision/ms 0.32 |
| warm-up | 5.58 | 44.3 | render/thread/ms 41.67, render/busy/ms 40.07, main/busy/ms 37.34, stream/drive/ms 12.28, render/graph/ms 6.38, stream/spawn/ms 1.84, render/present/ms 1.36, stream/tick/ms 1.02, bigspace/total/ms 0.91, terrain/update/ms 0.64, terrain/select/ms 0.52, bigspace/high_precision/ms 0.36 |
| measure | 2.03 | 27.6 | render/thread/ms 37.55, render/busy/ms 35.66, main/busy/ms 33.79, render/graph/ms 28.50, render/present/ms 1.73, bigspace/total/ms 0.60, stream/drive/ms 0.54, stream/tick/ms 0.48, terrain/update/ms 0.43, far/select/ms 0.39, terrain/select/ms 0.37, controller/feed/ms 0.32 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_pom\balmora@2560x1440_off=parallax-1 --bench-baseline none --off parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 15%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **97%** (min 80%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.5 W · 77 °C (max 81) · VRAM 5700 MiB · throttled 100% of samples
