# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1331 frames over 15.0 s · **median 11.20 ms (89 fps)** · mean 11.27 · p95 12.63 · p99 13.77 (1% low 73 fps) · worst 19.76 ms · 1 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 405 frames over 9.0 s, worst 2699.2 ms
- hitches (> 28.0 ms): **0** measured, 31 in warm-up
- timed GPU passes: 9.16 ms of a 11.27 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.20 ms, limit 11 ms
- pass worst frame 19.76 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.809 | 2.852 | 3.425 | 0.98 | 0.061 |
| dlss_super_resolution | 2.264 | 2.298 | 2.881 | 0.98 | 0.190 |
| early prepass | 0.653 | 0.663 | 1.290 | 0.98 | 0.104 |
| atmosphere_luts | 0.378 | 0.384 | 0.982 | 0.98 | 0.004 |
| bloom | 0.347 | 0.352 | 0.504 | 0.98 | 0.068 |
| main_transmissive_pass_3d | 0.305 | 0.310 | 0.306 | 0.98 | 0.009 |
| water_velocity | 0.264 | 0.268 | 0.272 | 0.98 | 0.002 |
| water_sspr | 0.256 | 0.260 | 0.618 | 0.98 | 0.002 |
| ssao | 0.214 | 0.217 | 0.219 | 0.98 | 0.008 |
| uniform_allocation | 0.200 | 0.203 | 0.198 | 0.98 | 0.035 |
| atmos_froxel_inject | 0.161 | 0.163 | 0.165 | 0.98 | 0.001 |
| bin_unpacking | 0.157 | 0.159 | 0.160 | 0.98 | 0.025 |
| clouds_trace | 0.148 | 0.150 | 0.177 | 0.98 | 0.001 |
| water_fft | 0.144 | 0.147 | 0.139 | 0.98 | 0.003 |
| upscaling | 0.126 | 0.128 | 0.139 | 0.98 | 0.004 |
| clouds_composite | 0.117 | 0.118 | 0.117 | 0.98 | 0.002 |
| tonemapping | 0.097 | 0.099 | 0.096 | 0.98 | 0.003 |
| clustering | 0.070 | 0.071 | 0.079 | 0.98 | 0.022 |
| lightprobe_irradiance_map | 0.057 | 0.468 | 1.078 | 0.12 | 0.001 |
| render_sky | 0.055 | 0.056 | 0.056 | 0.98 | 0.002 |
| clouds_resolve | 0.051 | 0.052 | 0.054 | 0.98 | 0.001 |
| early_mesh_preprocessing | 0.046 | 0.047 | 0.041 | 0.98 | 0.006 |
| water_snapshot | 0.040 | 0.041 | 0.055 | 0.98 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.98 | 0.003 |
| lightprobe_radiance_map | 0.030 | 0.247 | 0.235 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 0.98 | 0.001 |
| water_mips | 0.026 | 0.026 | 0.024 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.024 | 0.024 | 0.026 | 0.98 | 0.073 |
| per_view_shadow_pass | 0.023 | 0.023 | 0.029 | 0.98 | 0.007 |
| atmosphere_environment | 0.013 | 0.013 | 0.014 | 0.98 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 0.98 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 0.98 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.007 | 0.98 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.98 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.00 | 0.008 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 39.162 | 0.000 | 5558.04 | 52125.0 |
| render/thread/ms | 10.323 | 11.687 | 15.34 | 13739.3 |
| render/busy/ms | 8.372 | 10.143 | 14.70 | 11143.1 |
| main/busy/ms | 7.272 | 8.916 | 11.49 | 9679.4 |
| render/graph/ms | 5.106 | 6.497 | 9.87 | 6795.8 |
| render/acquire/ms | 1.470 | 2.782 | 4.21 | 1956.7 |
| pipelines/compile_ms | 1.150 | 0.000 | 132.77 | 1531.2 |
| pipelines/front_ms | 1.114 | 0.000 | 130.09 | 1483.2 |
| render/present/ms | 0.481 | 0.577 | 3.53 | 639.5 |
| bigspace/total/ms | 0.392 | 0.754 | 3.16 | 521.7 |
| terrain/update/ms | 0.233 | 0.304 | 0.53 | 310.4 |
| terrain/select/ms | 0.203 | 0.263 | 0.50 | 269.9 |
| stream/drive/ms | 0.187 | 0.283 | 0.52 | 249.0 |
| far/select/ms | 0.165 | 0.252 | 0.42 | 219.3 |
| stream/tick/ms | 0.161 | 0.250 | 0.48 | 214.2 |
| controller/feed/ms | 0.153 | 0.216 | 0.43 | 203.5 |
| bigspace/high_precision/ms | 0.141 | 0.297 | 2.94 | 188.1 |
| controller/move/ms | 0.132 | 0.240 | 0.44 | 175.3 |
| anim/evaluate/ms | 0.098 | 0.219 | 1.76 | 130.0 |
| bigspace/low_precision/ms | 0.097 | 0.239 | 1.94 | 129.7 |
| anim/evaluate_cpu/ms | 0.061 | 0.080 | 0.24 | 81.4 |
| pipelines/driver_ms | 0.036 | 0.000 | 4.24 | 48.0 |
| physics/step/ms | 0.036 | 0.064 | 0.35 | 47.3 |
| weather/map/ms | 0.025 | 0.034 | 0.20 | 32.7 |
| weather/eval/ms | 0.011 | 0.015 | 0.03 | 14.4 |
| script/update/ms | 0.007 | 0.010 | 0.03 | 9.1 |
| controller/avoid/ms | 0.006 | 0.007 | 0.13 | 7.7 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 6.3 |
| anim/apply/ms | 0.005 | 0.006 | 0.04 | 6.0 |
| physics/insert/ms | 0.004 | 0.010 | 0.55 | 5.6 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 5.4 |
| controller/write/ms | 0.002 | 0.002 | 0.14 | 2.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.8 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 123 | 123 | 122 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1080 |
| crowd/c2/count | count | 283 | 284 | 284 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 7474 | 4212 |
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
| frame/work/ms | ms | 9.88 | 14.70 | 9.90 |
| gpu/timed/ms | ms | 9.82 | 11.96 | 9.90 |
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
| phases/opaque3d/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 24 | 26 | 24 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 25 | 27 | 25 |
| phases/opaque3d_prepass/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 24 | 26 | 24 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 33 | 33 | 33 |
| phases/shadow/bins | count | 2028 | 2029 | 2028 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5345 | 5346 | 5345 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 33 | 33 | 33 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 112 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 191 | 191 | 191 |
| pipelines/waiting | count | 0 | 22 | 0 |
| preprocess/bind_groups | count | 0 | 219 | 0 |
| preprocess/bind_groups_created | count | 0 | 219 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 217 | 217 | 217 |
| process/cpu_usage | % | 24.76 | 41.83 | 23.08 |
| process/mem_usage | GiB | 4.36 | 4.41 | 4.38 |
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
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.06 | 62.37 | 24.95 |
| system/mem_usage | % | 72.83 | 73.35 | 73.15 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2480 | 2480 | 2480 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 97 | 101 | 97 |
| views/all/bins | count | 5198 | 5201 | 5198 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.6 k | 14.6 k |
| views/all/pipelines | count | 93 | 97 | 93 |
| views/all/pipelines_distinct | count | 83 | 87 | 83 |
| views/all/work_items | count | 14.8 k | 14.8 k | 14.8 k |
| views/cascade/batch_sets | count | 33 | 33 | 33 |
| views/cascade/bins | count | 2028 | 2029 | 2028 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5345 | 5346 | 5345 |
| views/cascade/pipelines | count | 33 | 33 | 33 |
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
| views/main/batch_sets | count | 64 | 68 | 64 |
| views/main/bins | count | 3170 | 3172 | 3170 |
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
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.13 | 2 | 170 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.72 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.01 | 14 | 15 |
| physics/inserts | count | 0.10 | 53 | 129 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.68 | 2 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.01 | 8 | 10 |
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

## Hitches (> 28.0 ms; slowest 31, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.70 | 2699.2 | main/busy/ms 2675, surface/advance_cpu_ms 69.60, surface/cell_upload_ms 1.26, stream/drive/ms 0.29, weather/map/ms 0.15, bigspace/total/ms 0.10, stream/tick/ms 0.06, bigspace/high_precision/ms 0.06, weather/eval/ms 0.02, script/update/ms 0.02, anim/evaluate/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.50 | 802.5 | main/busy/ms 2717, render/thread/ms 761, render/busy/ms 642, render/acquire/ms 117, surface/advance_cpu_ms 69.60, render/graph/ms 5.60, render/present/ms 1.97, surface/cell_upload_ms 1.26, stream/drive/ms 1.15, stream/tick/ms 0.63, terrain/update/ms 0.44, terrain/select/ms 0.28 |
| warm-up | 3.61 | 106.1 | pipelines/wait_ms 2945, render/thread/ms 773, render/busy/ms 653, render/acquire/ms 117, pipelines/compile_ms 115, pipelines/front_ms 105, main/busy/ms 58.19, terrain/update/ms 24.22, render/graph/ms 9.70, pipelines/driver_ms 9.69, render/present/ms 2.98, stream/drive/ms 1.47 |
| warm-up | 3.84 | 91.4 | main/busy/ms 59.43, render/thread/ms 51.69, render/busy/ms 49.86, render/graph/ms 4.65, stream/drive/ms 1.65, render/present/ms 1.51, stream/tick/ms 0.89, terrain/update/ms 0.48, terrain/select/ms 0.37, stream/spawn/ms 0.35, render/acquire/ms 0.33, bigspace/total/ms 0.24 |
| warm-up | 3.88 | 35.2 | pipelines/compile_ms 235, pipelines/front_ms 222, render/thread/ms 35.73, render/busy/ms 34.32, main/busy/ms 18.30, pipelines/driver_ms 12.14, render/graph/ms 3.09, stream/drive/ms 2.54, render/present/ms 1.24, stream/tick/ms 1.04, stream/spawn/ms 0.92, terrain/update/ms 0.60 |
| warm-up | 3.94 | 44.0 | pipelines/wait_ms 500, pipelines/compile_ms 60.31, pipelines/front_ms 55.69, render/thread/ms 41.27, render/busy/ms 39.86, main/busy/ms 27.81, terrain/update/ms 12.55, pipelines/driver_ms 4.63, render/graph/ms 3.58, stream/drive/ms 2.06, render/present/ms 1.11, stream/tick/ms 1.06 |
| warm-up | 4.02 | 74.5 | main/busy/ms 44.62, render/thread/ms 41.27, render/busy/ms 40.10, stream/drive/ms 29.79, render/graph/ms 2.96, terrain/update/ms 1.01, render/present/ms 0.94, stream/tick/ms 0.64, stream/spawn/ms 0.62, terrain/select/ms 0.51, bigspace/total/ms 0.29, render/acquire/ms 0.23 |
| warm-up | 4.10 | 68.1 | pipelines/wait_ms 704, pipelines/compile_ms 184, pipelines/front_ms 165, render/thread/ms 54.12, render/busy/ms 52.58, main/busy/ms 36.72, stream/drive/ms 19.60, pipelines/driver_ms 18.97, render/graph/ms 5.75, stream/spawn/ms 1.67, stream/tick/ms 1.60, render/present/ms 1.26 |
| warm-up | 4.21 | 87.6 | render/thread/ms 76.61, render/busy/ms 52.44, main/busy/ms 31.98, render/present/ms 24.00, stream/drive/ms 14.35, render/graph/ms 3.51, stream/spawn/ms 1.59, stream/tick/ms 1.22, terrain/update/ms 0.90, terrain/select/ms 0.71, bigspace/total/ms 0.25, render/acquire/ms 0.17 |
| warm-up | 4.24 | 35.6 | render/thread/ms 91.96, render/busy/ms 67.97, main/busy/ms 29.02, render/present/ms 23.82, stream/drive/ms 12.80, stream/spawn/ms 2.40, render/graph/ms 2.11, terrain/update/ms 0.74, stream/tick/ms 0.72, terrain/select/ms 0.58, stream/activate/ms 0.32, bigspace/total/ms 0.30 |
| warm-up | 4.34 | 82.1 | pipelines/wait_ms 4447, pipelines/compile_ms 255, pipelines/front_ms 243, main/busy/ms 65.77, stream/drive/ms 45.15, render/thread/ms 40.66, render/busy/ms 39.40, pipelines/driver_ms 12.22, render/graph/ms 3.59, stream/spawn/ms 2.29, stream/tick/ms 1.29, render/present/ms 1.02 |
| warm-up | 4.37 | 30.2 | pipelines/wait_ms 3012, render/thread/ms 47.56, render/busy/ms 46.36, pipelines/compile_ms 31.71, pipelines/front_ms 29.26, main/busy/ms 26.74, stream/drive/ms 8.85, render/graph/ms 3.20, pipelines/driver_ms 2.45, stream/tick/ms 1.26, render/present/ms 0.95, stream/activate/ms 0.94 |
| warm-up | 4.44 | 39.1 | pipelines/wait_ms 1209, pipelines/compile_ms 134, pipelines/front_ms 130, main/busy/ms 43.40, stream/drive/ms 26.19, render/thread/ms 18.65, render/busy/ms 16.97, pipelines/driver_ms 3.44, render/graph/ms 3.21, render/present/ms 1.50, stream/spawn/ms 1.48, stream/tick/ms 0.62 |
| warm-up | 4.56 | 62.7 | pipelines/wait_ms 6943, pipelines/compile_ms 93.85, pipelines/front_ms 89.34, main/busy/ms 59.30, stream/drive/ms 49.96, render/thread/ms 21.52, render/busy/ms 20.19, pipelines/driver_ms 4.51, render/graph/ms 4.29, render/present/ms 1.15, stream/spawn/ms 1.02, stream/tick/ms 0.67 |
| warm-up | 4.63 | 60.3 | main/busy/ms 55.63, stream/drive/ms 40.62, render/thread/ms 20.81, render/busy/ms 19.68, render/graph/ms 2.84, stream/spawn/ms 2.12, render/present/ms 0.95, stream/tick/ms 0.61, terrain/update/ms 0.57, terrain/select/ms 0.44, bigspace/total/ms 0.38, stream/activate/ms 0.30 |
| warm-up | 4.83 | 28.2 | pipelines/wait_ms 3468, pipelines/compile_ms 55.73, pipelines/front_ms 53.41, render/thread/ms 34.64, render/busy/ms 32.32, main/busy/ms 31.77, stream/drive/ms 11.54, render/graph/ms 5.96, stream/spawn/ms 3.70, terrain/update/ms 3.44, pipelines/driver_ms 2.32, render/present/ms 1.95 |
| warm-up | 5.08 | 34.7 | pipelines/wait_ms 1384, render/thread/ms 51.71, render/busy/ms 50.05, main/busy/ms 26.42, pipelines/compile_ms 24.80, pipelines/front_ms 23.08, render/graph/ms 5.99, stream/drive/ms 4.06, stream/spawn/ms 2.05, pipelines/driver_ms 1.72, render/present/ms 1.35, bigspace/total/ms 1.20 |
| warm-up | 5.12 | 33.0 | render/thread/ms 50.70, render/busy/ms 49.52, main/busy/ms 24.27, render/graph/ms 4.70, stream/drive/ms 4.25, stream/spawn/ms 2.20, bigspace/total/ms 1.20, render/present/ms 0.98, bigspace/high_precision/ms 0.93, stream/tick/ms 0.91, terrain/update/ms 0.72, terrain/select/ms 0.58 |
| warm-up | 5.19 | 29.5 | render/thread/ms 42.35, render/busy/ms 40.62, main/busy/ms 25.82, render/graph/ms 6.07, stream/drive/ms 3.97, stream/spawn/ms 2.11, render/present/ms 1.47, bigspace/total/ms 1.02, stream/tick/ms 0.79, terrain/update/ms 0.66, terrain/select/ms 0.53, bigspace/high_precision/ms 0.44 |
| warm-up | 5.22 | 35.6 | pipelines/wait_ms 4144, pipelines/compile_ms 174, pipelines/front_ms 167, render/thread/ms 56.79, render/busy/ms 54.68, main/busy/ms 29.55, render/graph/ms 8.10, pipelines/driver_ms 7.34, stream/drive/ms 6.76, stream/spawn/ms 2.74, render/present/ms 1.81, bigspace/total/ms 1.05 |
| warm-up | 5.26 | 35.3 | pipelines/wait_ms 4144, pipelines/compile_ms 174, pipelines/front_ms 167, render/thread/ms 58.20, render/busy/ms 55.90, main/busy/ms 33.47, render/graph/ms 9.11, pipelines/driver_ms 7.34, stream/drive/ms 6.44, stream/spawn/ms 2.77, render/present/ms 1.92, terrain/update/ms 0.80 |
| warm-up | 5.31 | 47.8 | render/thread/ms 53.33, render/busy/ms 51.42, main/busy/ms 46.39, stream/drive/ms 18.43, render/graph/ms 7.67, stream/spawn/ms 2.25, render/present/ms 1.62, stream/tick/ms 0.80, bigspace/total/ms 0.76, terrain/update/ms 0.75, terrain/select/ms 0.59, bigspace/high_precision/ms 0.34 |
| warm-up | 5.35 | 47.1 | render/thread/ms 62.49, render/busy/ms 60.92, main/busy/ms 44.02, stream/drive/ms 18.46, render/graph/ms 6.14, stream/spawn/ms 2.11, render/present/ms 1.36, bigspace/total/ms 0.93, terrain/update/ms 0.78, stream/tick/ms 0.73, terrain/select/ms 0.60, bigspace/high_precision/ms 0.45 |
| warm-up | 5.48 | 32.1 | main/busy/ms 50.62, render/thread/ms 27.20, render/busy/ms 25.61, stream/drive/ms 17.80, render/graph/ms 6.53, stream/spawn/ms 2.20, render/present/ms 1.38, stream/tick/ms 0.70, bigspace/total/ms 0.65, terrain/update/ms 0.56, terrain/select/ms 0.45, bigspace/high_precision/ms 0.27 |
| warm-up | 5.51 | 33.8 | pipelines/wait_ms 1792, pipelines/compile_ms 80.02, pipelines/front_ms 76.66, main/busy/ms 45.10, render/thread/ms 33.45, render/busy/ms 31.82, stream/drive/ms 17.57, render/graph/ms 4.31, pipelines/driver_ms 3.36, stream/spawn/ms 2.04, render/present/ms 1.32, stream/tick/ms 0.63 |
| warm-up | 5.74 | 32.4 | pipelines/wait_ms 7485, pipelines/compile_ms 44.37, pipelines/front_ms 41.63, render/thread/ms 40.98, render/busy/ms 39.32, main/busy/ms 23.02, render/graph/ms 7.52, stream/drive/ms 3.46, pipelines/driver_ms 2.74, stream/spawn/ms 2.09, render/present/ms 1.40, bigspace/total/ms 0.74 |
| warm-up | 5.81 | 46.3 | render/thread/ms 40.56, render/busy/ms 38.90, main/busy/ms 32.95, stream/drive/ms 12.25, render/graph/ms 6.63, stream/spawn/ms 1.96, render/present/ms 1.39, terrain/update/ms 1.05, bigspace/total/ms 0.88, stream/tick/ms 0.86, terrain/select/ms 0.83, bigspace/high_precision/ms 0.32 |
| warm-up | 6.27 | 37.1 | pipelines/wait_ms 9765, pipelines/compile_ms 52.26, pipelines/front_ms 49.82, main/busy/ms 38.74, render/thread/ms 38.66, render/busy/ms 36.91, render/graph/ms 7.58, pipelines/driver_ms 2.45, stream/drive/ms 1.37, render/present/ms 1.34, bigspace/total/ms 0.88, stream/tick/ms 0.65 |
| warm-up | 6.78 | 38.4 | main/busy/ms 46.73, render/thread/ms 36.18, render/busy/ms 34.48, render/graph/ms 11.20, render/present/ms 1.50, stream/drive/ms 1.31, bigspace/total/ms 1.11, terrain/update/ms 0.64, anim/evaluate_cpu/ms 0.58, terrain/select/ms 0.52, stream/spawn/ms 0.48, stream/tick/ms 0.48 |
| warm-up | 6.82 | 36.2 | pipelines/wait_ms 3104, pipelines/compile_ms 78.93, pipelines/front_ms 74.20, render/thread/ms 45.06, render/busy/ms 43.54, main/busy/ms 39.35, render/graph/ms 7.55, pipelines/driver_ms 4.73, stream/drive/ms 1.40, render/present/ms 1.32, bigspace/total/ms 0.79, terrain/update/ms 0.65 |
| warm-up | 8.98 | 28.2 | main/busy/ms 33.29, render/thread/ms 31.03, render/busy/ms 29.70, render/graph/ms 11.79, terrain/update/ms 6.86, bigspace/total/ms 1.18, render/present/ms 1.16, stream/drive/ms 0.62, controller/feed/ms 0.54, physics/insert/ms 0.53, stream/tick/ms 0.53, bigspace/high_precision/ms 0.50 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_pom\balmora@2560x1440_off=parallax-3 --bench-baseline none --off parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 18%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **98%** (min 96%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.0 W · 76 °C (max 79) · VRAM 5713 MiB · throttled 100% of samples
