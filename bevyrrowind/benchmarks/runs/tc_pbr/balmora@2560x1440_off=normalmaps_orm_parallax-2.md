# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 968 frames over 15.0 s · **median 15.43 ms (65 fps)** · mean 15.51 · p95 16.67 · p99 17.67 (1% low 57 fps) · worst 26.37 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 403 frames over 8.6 s, worst 2504.2 ms
- hitches (> 38.6 ms): **0** measured, 13 in warm-up
- timed GPU passes: 13.60 ms of a 15.51 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 15.43 ms, limit 11 ms
- pass worst frame 26.37 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 6.695 | 6.702 | 6.863 | 1.00 | 0.059 |
| early prepass | 0.967 | 0.968 | 1.310 | 1.00 | 0.095 |
| ssao | 0.803 | 0.803 | 1.095 | 1.00 | 0.007 |
| main_transmissive_pass_3d | 0.722 | 0.723 | 1.093 | 1.00 | 0.007 |
| clouds_composite | 0.481 | 0.481 | 0.816 | 1.00 | 0.001 |
| clouds_trace | 0.447 | 0.448 | 0.480 | 1.00 | 0.001 |
| taa | 0.410 | 0.410 | 0.763 | 1.00 | 0.002 |
| atmosphere_luts | 0.357 | 0.357 | 0.626 | 1.00 | 0.003 |
| bloom | 0.332 | 0.332 | 0.339 | 1.00 | 0.059 |
| water_sspr | 0.304 | 0.304 | 0.393 | 1.00 | 0.002 |
| water_velocity | 0.266 | 0.266 | 0.280 | 1.00 | 0.001 |
| render_sky | 0.212 | 0.212 | 0.204 | 1.00 | 0.001 |
| water_snapshot | 0.194 | 0.194 | 0.233 | 1.00 | 0.001 |
| clouds_resolve | 0.194 | 0.194 | 0.204 | 1.00 | 0.001 |
| uniform_allocation | 0.183 | 0.184 | 0.185 | 1.00 | 0.030 |
| atmos_froxel_inject | 0.156 | 0.156 | 0.154 | 1.00 | 0.001 |
| bin_unpacking | 0.143 | 0.144 | 0.144 | 1.00 | 0.022 |
| water_fft | 0.139 | 0.139 | 0.139 | 1.00 | 0.002 |
| upscaling | 0.127 | 0.127 | 0.153 | 1.00 | 0.004 |
| tonemapping | 0.096 | 0.096 | 0.097 | 1.00 | 0.003 |
| clustering | 0.066 | 0.066 | 0.075 | 1.00 | 0.021 |
| lightprobe_irradiance_map | 0.055 | 0.438 | 0.760 | 0.12 | 0.001 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.042 | 1.00 | 0.005 |
| atmos_froxel_integrate | 0.033 | 0.033 | 0.033 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.228 | 0.233 | 0.12 | 0.003 |
| main_transparent_pass_3d | 0.023 | 0.023 | 0.026 | 1.00 | 0.070 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.023 | 0.026 | 1.00 | 0.007 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.012 | 1.00 | 0.003 |
| night_sky | 0.010 | 0.010 | 0.013 | 1.00 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 1.00 | 0.003 |
| auxiliary_views | 0.003 | 0.003 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.017 | 0.015 | 0.12 | 0.001 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.12 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.004 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 35.697 | 0.000 | 10390.68 | 34554.8 |
| render/thread/ms | 14.586 | 15.727 | 25.06 | 14119.3 |
| render/busy/ms | 7.212 | 8.085 | 9.88 | 6981.4 |
| render/acquire/ms | 6.882 | 7.959 | 10.80 | 6661.9 |
| main/busy/ms | 6.055 | 6.608 | 8.55 | 5861.3 |
| render/graph/ms | 3.856 | 4.700 | 5.79 | 3732.7 |
| pipelines/compile_ms | 0.706 | 0.000 | 129.08 | 683.2 |
| pipelines/front_ms | 0.683 | 0.000 | 124.49 | 661.6 |
| render/present/ms | 0.492 | 0.575 | 12.63 | 475.9 |
| bigspace/total/ms | 0.287 | 0.370 | 0.56 | 278.2 |
| terrain/update/ms | 0.230 | 0.274 | 0.43 | 222.4 |
| terrain/select/ms | 0.201 | 0.244 | 0.40 | 194.4 |
| stream/drive/ms | 0.173 | 0.200 | 0.39 | 167.8 |
| far/select/ms | 0.152 | 0.181 | 0.33 | 147.2 |
| controller/feed/ms | 0.152 | 0.193 | 0.33 | 146.8 |
| stream/tick/ms | 0.147 | 0.171 | 0.35 | 142.4 |
| controller/move/ms | 0.122 | 0.185 | 0.37 | 118.5 |
| bigspace/high_precision/ms | 0.094 | 0.120 | 0.32 | 90.7 |
| anim/evaluate/ms | 0.076 | 0.092 | 0.25 | 73.8 |
| bigspace/low_precision/ms | 0.066 | 0.083 | 0.32 | 63.5 |
| anim/evaluate_cpu/ms | 0.056 | 0.067 | 0.23 | 54.0 |
| physics/step/ms | 0.052 | 0.068 | 0.53 | 50.8 |
| weather/map/ms | 0.024 | 0.032 | 0.31 | 23.0 |
| pipelines/driver_ms | 0.022 | 0.000 | 4.58 | 21.7 |
| weather/eval/ms | 0.011 | 0.015 | 0.15 | 10.4 |
| script/update/ms | 0.007 | 0.008 | 0.13 | 6.7 |
| physics/insert/ms | 0.007 | 0.000 | 0.54 | 6.6 |
| controller/avoid/ms | 0.006 | 0.008 | 0.03 | 5.7 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 5.0 |
| anim/apply/ms | 0.004 | 0.005 | 0.01 | 3.7 |
| script/ui/ms | 0.003 | 0.004 | 0.01 | 3.1 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 1.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 0.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.6 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 122 | 123 | 122 |
| crowd/c1/joints/count | count | 4848 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1088 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 4212 | 4212 |
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
| frame/work/ms | ms | 14.21 | 17.52 | 14.10 |
| gpu/timed/ms | ms | 14.21 | 17.52 | 14.10 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 486.97 M | 486.97 M | 486.97 M |
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
| phases/opaque3d/batch_sets | count | 22 | 22 | 22 |
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 21 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 22 | 22 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 21 | 21 |
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
| physics/pending | count | 0 | 385 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 184 | 184 | 184 |
| pipelines/waiting | count | 0 | 12 | 0 |
| preprocess/bind_groups | count | 0 | 1 | 0 |
| preprocess/bind_groups_created | count | 0 | 1 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 201 | 201 | 201 |
| process/cpu_usage | % | 17.43 | 35.10 | 23.08 |
| process/mem_usage | GiB | 4.04 | 4.15 | 4.15 |
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
| stream/latency/s | s | 0.03 | 0.03 | 0.03 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 24.23 | 39.39 | 30.95 |
| system/mem_usage | % | 72.59 | 72.85 | 72.85 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2480 | 2480 | 2480 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 89 | 89 | 89 |
| views/all/bins | count | 5198 | 5199 | 5198 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.6 k | 14.6 k |
| views/all/pipelines | count | 85 | 85 | 85 |
| views/all/pipelines_distinct | count | 75 | 75 | 75 |
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
| views/main/batch_sets | count | 58 | 58 | 58 |
| views/main/bins | count | 3170 | 3170 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 54 | 54 | 54 |
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
| anim/events/count | count | 0.14 | 2 | 132 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 5.12 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| physics/inserts | count | 0.48 | 83 | 465 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.93 | 2 | 901 |
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

## Hitches (> 38.6 ms; slowest 13, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2504.2 | main/busy/ms 2482, surface/advance_cpu_ms 68.91, surface/cell_upload_ms 0.72, stream/drive/ms 0.34, weather/map/ms 0.20, bigspace/total/ms 0.09, stream/tick/ms 0.08, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.01, script/ui/ms 0.01 |
| warm-up | 3.06 | 560.1 | main/busy/ms 2513, render/thread/ms 528, render/busy/ms 506, surface/advance_cpu_ms 68.91, render/acquire/ms 20.29, render/graph/ms 12.62, render/present/ms 1.81, stream/drive/ms 0.87, surface/cell_upload_ms 0.72, terrain/update/ms 0.52, stream/tick/ms 0.43, terrain/select/ms 0.32 |
| warm-up | 3.15 | 82.6 | render/thread/ms 564, render/busy/ms 540, pipelines/wait_ms 426, pipelines/compile_ms 72.26, pipelines/front_ms 65.99, main/busy/ms 47.02, render/graph/ms 39.89, render/acquire/ms 20.55, terrain/update/ms 16.54, pipelines/driver_ms 6.27, render/present/ms 3.53, stream/drive/ms 1.06 |
| warm-up | 3.37 | 91.9 | pipelines/wait_ms 609, pipelines/compile_ms 232, pipelines/front_ms 203, render/thread/ms 60.71, main/busy/ms 60.55, render/busy/ms 59.23, pipelines/driver_ms 28.89, render/graph/ms 3.07, render/present/ms 1.23, stream/drive/ms 0.95, stream/tick/ms 0.80, terrain/update/ms 0.75 |
| warm-up | 3.51 | 61.0 | main/busy/ms 41.98, render/thread/ms 41.91, render/busy/ms 40.89, stream/drive/ms 30.98, render/graph/ms 1.70, render/present/ms 0.84, stream/tick/ms 0.83, terrain/update/ms 0.49, stream/spawn/ms 0.48, terrain/select/ms 0.38, stream/upload/ms 0.31, bigspace/total/ms 0.29 |
| warm-up | 3.60 | 83.6 | pipelines/wait_ms 1024, pipelines/compile_ms 223, pipelines/front_ms 206, render/thread/ms 59.69, render/busy/ms 56.47, main/busy/ms 38.32, stream/drive/ms 22.18, pipelines/driver_ms 16.92, render/graph/ms 6.89, render/present/ms 2.97, stream/spawn/ms 1.96, stream/tick/ms 1.09 |
| warm-up | 3.74 | 124.3 | pipelines/wait_ms 8879, pipelines/compile_ms 251, pipelines/front_ms 150, render/thread/ms 105, render/busy/ms 104, pipelines/driver_ms 101, main/busy/ms 38.42, stream/drive/ms 9.95, render/graph/ms 3.32, render/present/ms 1.65, stream/spawn/ms 1.46, stream/tick/ms 0.88 |
| warm-up | 3.85 | 91.8 | pipelines/wait_ms 1219, pipelines/compile_ms 144, pipelines/front_ms 136, main/busy/ms 60.16, render/thread/ms 58.00, render/busy/ms 55.56, stream/drive/ms 38.00, pipelines/driver_ms 8.42, render/graph/ms 6.41, stream/spawn/ms 3.57, terrain/update/ms 2.89, render/present/ms 2.23 |
| warm-up | 3.93 | 67.0 | pipelines/wait_ms 7726, pipelines/compile_ms 79.48, pipelines/front_ms 73.84, render/thread/ms 45.76, main/busy/ms 44.59, render/busy/ms 44.51, stream/drive/ms 20.48, pipelines/driver_ms 5.64, render/graph/ms 3.22, stream/spawn/ms 2.20, render/present/ms 1.06, stream/activate/ms 0.82 |
| warm-up | 4.09 | 63.2 | main/busy/ms 63.20, stream/drive/ms 52.17, render/thread/ms 23.54, render/busy/ms 22.01, render/graph/ms 2.68, render/present/ms 1.34, stream/tick/ms 0.98, stream/spawn/ms 0.96, terrain/update/ms 0.54, terrain/select/ms 0.43, bigspace/total/ms 0.23, render/acquire/ms 0.19 |
| warm-up | 4.15 | 54.2 | pipelines/wait_ms 6316, pipelines/compile_ms 228, pipelines/front_ms 214, main/busy/ms 51.35, stream/drive/ms 35.76, render/thread/ms 20.88, render/busy/ms 19.83, pipelines/driver_ms 14.23, render/graph/ms 2.50, stream/spawn/ms 2.45, render/present/ms 0.88, terrain/update/ms 0.77 |
| warm-up | 5.30 | 41.6 | render/thread/ms 37.39, render/busy/ms 35.89, main/busy/ms 33.85, stream/drive/ms 11.74, render/graph/ms 4.21, stream/spawn/ms 1.52, stream/tick/ms 1.45, render/present/ms 1.31, bigspace/total/ms 0.86, terrain/update/ms 0.80, terrain/select/ms 0.68, bigspace/high_precision/ms 0.31 |
| warm-up | 6.27 | 41.4 | main/busy/ms 45.49, render/thread/ms 35.40, render/busy/ms 34.20, render/graph/ms 5.79, physics/insert/ms 1.61, stream/drive/ms 1.35, bigspace/total/ms 1.18, render/present/ms 0.99, anim/evaluate_cpu/ms 0.63, stream/tick/ms 0.51, terrain/update/ms 0.44, stream/spawn/ms 0.44 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\claude\3866c27c11752a1d\4ff90442-bcd4-4820-b03c-4dd8a81992a4\scratchpad\bin\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\tc_pbr\balmora@2560x1440_off=normalmaps_orm_parallax-2 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 4%, CPU 9%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2580 MHz (min 2580) · memory 8001 MHz · 99.3 W · 81 °C (max 83) · VRAM 5120 MiB · throttled 100% of samples
