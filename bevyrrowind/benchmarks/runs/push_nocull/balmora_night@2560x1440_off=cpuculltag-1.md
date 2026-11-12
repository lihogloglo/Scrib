# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cpuculltag · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1658 frames over 15.0 s · **median 8.99 ms (111 fps)** · mean 9.05 · p95 9.98 · p99 10.57 (1% low 95 fps) · worst 13.93 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 361 frames over 4.3 s, worst 516.8 ms
- hitches (> 25.0 ms): **0** measured, 8 in warm-up
- timed GPU passes: 8.45 ms of a 9.05 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 8.99 ms, limit 11 ms
- pass worst frame 13.93 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.119 | 2.391 | 0.044 |
| dlss_super_resolution | 2.069 | 2.271 | 0.187 |
| atmosphere_luts | 1.104 | 1.320 | 0.003 |
| early prepass | 0.607 | 0.884 | 0.082 |
| lightprobe_irradiance_map | 0.413 | 0.415 | 0.001 |
| bloom | 0.323 | 0.337 | 0.067 |
| lightprobe_radiance_map | 0.228 | 0.234 | 0.006 |
| ssao | 0.219 | 0.221 | 0.007 |
| main_transparent_pass_3d | 0.175 | 0.178 | 0.072 |
| water_fft | 0.129 | 0.128 | 0.002 |
| uniform_allocation | 0.126 | 0.132 | 0.025 |
| upscaling | 0.119 | 0.130 | 0.004 |
| bin_unpacking | 0.094 | 0.100 | 0.018 |
| tonemapping | 0.090 | 0.095 | 0.004 |
| atmos_froxel_inject | 0.077 | 0.079 | 0.002 |
| water_velocity | 0.075 | 0.076 | 0.001 |
| clouds_trace | 0.067 | 0.075 | 0.001 |
| clouds_composite | 0.064 | 0.066 | 0.002 |
| clustering | 0.064 | 0.077 | 0.022 |
| render_sky | 0.054 | 0.055 | 0.002 |
| clouds_resolve | 0.034 | 0.037 | 0.001 |
| early_mesh_preprocessing | 0.032 | 0.034 | 0.005 |
| water_mips | 0.024 | 0.024 | 0.002 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.022 | 0.025 | 0.001 |
| per_view_shadow_pass | 0.015 | 0.022 | 0.005 |
| lightprobe_copy | 0.015 | 0.016 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.015 | 0.001 |
| clouds_fog_light | 0.013 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.012 | 0.012 | 0.001 |
| atmosphere_environment | 0.011 | 0.014 | 0.003 |
| night_sky | 0.007 | 0.012 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.003 |
| blas_build | 0.002 | 0.002 | 0.006 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| surface_trail | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 22.862 | 0.000 | 4696.61 | 37905.0 |
| render/thread/ms | 8.196 | 9.160 | 13.03 | 13589.1 |
| main/busy/ms | 6.428 | 7.342 | 11.16 | 10658.2 |
| render/busy/ms | 6.193 | 7.337 | 12.40 | 10268.4 |
| render/graph/ms | 3.387 | 4.404 | 9.64 | 5615.6 |
| render/acquire/ms | 1.568 | 2.622 | 3.40 | 2599.3 |
| pipelines/compile_ms | 0.568 | 0.000 | 141.73 | 941.2 |
| pipelines/front_ms | 0.550 | 0.000 | 138.46 | 912.5 |
| render/present/ms | 0.435 | 0.548 | 1.71 | 721.4 |
| bigspace/total/ms | 0.386 | 0.696 | 1.55 | 640.1 |
| terrain/update/ms | 0.235 | 0.298 | 0.60 | 389.7 |
| terrain/select/ms | 0.204 | 0.256 | 0.56 | 338.5 |
| stream/drive/ms | 0.175 | 0.245 | 0.55 | 290.8 |
| far/select/ms | 0.160 | 0.231 | 0.42 | 264.8 |
| stream/tick/ms | 0.150 | 0.212 | 0.51 | 248.6 |
| bigspace/high_precision/ms | 0.139 | 0.323 | 1.36 | 231.1 |
| controller/feed/ms | 0.129 | 0.187 | 0.33 | 214.6 |
| controller/move/ms | 0.124 | 0.218 | 0.51 | 205.3 |
| bigspace/low_precision/ms | 0.089 | 0.212 | 1.04 | 147.2 |
| anim/evaluate/ms | 0.081 | 0.209 | 1.18 | 133.7 |
| anim/evaluate_cpu/ms | 0.064 | 0.084 | 0.26 | 106.6 |
| physics/step/ms | 0.029 | 0.059 | 0.64 | 48.8 |
| pipelines/driver_ms | 0.017 | 0.000 | 3.83 | 28.7 |
| weather/map/ms | 0.017 | 0.022 | 0.17 | 27.7 |
| weather/eval/ms | 0.010 | 0.013 | 0.03 | 16.6 |
| physics/insert/ms | 0.007 | 0.031 | 0.60 | 12.4 |
| script/update/ms | 0.006 | 0.009 | 0.12 | 10.5 |
| controller/avoid/ms | 0.006 | 0.008 | 0.02 | 9.8 |
| anim/apply/ms | 0.005 | 0.006 | 0.05 | 7.6 |
| script/input/ms | 0.004 | 0.006 | 0.02 | 7.5 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.0 |
| controller/write/ms | 0.002 | 0.002 | 0.35 | 3.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.16 | 1.3 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 14 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 8.40 | 12.40 | 8.54 |
| gpu/timed/ms | ms | 8.40 | 10.29 | 8.54 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.72 M | 433.72 M | 433.72 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d/bins | count | 282 | 282 | 282 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 876 | 876 | 876 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 282 | 282 | 282 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 876 | 876 | 876 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 14 | 15 | 14 |
| phases/opaque3d/bins | count | 1307 | 1307 | 1307 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3780 | 3780 | 3780 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 13 | 14 | 13 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/bins | count | 1307 | 1307 | 1307 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3780 | 3780 | 3780 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 13 | 14 | 13 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 16 | 16 | 16 |
| phases/shadow/bins | count | 988 | 988 | 988 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 2009 | 2009 | 2009 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 16 | 16 | 16 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 621 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 149 | 149 | 149 |
| pipelines/waiting | count | 0 | 19 | 0 |
| preprocess/bind_groups | count | 0 | 98 | 0 |
| preprocess/bind_groups_created | count | 0 | 98 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 139 | 139 | 139 |
| process/cpu_usage | % | 24.40 | 36.84 | 23.08 |
| process/mem_usage | GiB | 2.13 | 2.21 | 2.21 |
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
| started_load_count |  | 2595 | 2597 | 2597 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 388 | 388 | 388 |
| statics/materials_bound | count | 388 | 388 | 388 |
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
| system/cpu_usage | % | 30.36 | 49.70 | 30.36 |
| system/mem_usage | % | 49.80 | 50.05 | 50.04 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 60 | 62 | 60 |
| views/all/bins | count | 4166 | 4166 | 4166 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.3 k | 11.3 k | 11.3 k |
| views/all/pipelines | count | 56 | 58 | 56 |
| views/all/pipelines_distinct | count | 55 | 57 | 55 |
| views/all/work_items | count | 11.5 k | 11.5 k | 11.5 k |
| views/cascade/batch_sets | count | 16 | 16 | 16 |
| views/cascade/bins | count | 988 | 988 | 988 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2009 | 2009 | 2009 |
| views/cascade/pipelines | count | 16 | 16 | 16 |
| views/cascade/work_items | count | 2009 | 2009 | 2009 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 44 | 46 | 44 |
| views/main/bins | count | 3178 | 3178 | 3178 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9312 | 9312 | 9312 |
| views/main/pipelines | count | 40 | 42 | 40 |
| views/main/work_items | count | 9490 | 9490 | 9490 |
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
| anim/events/count | count | 0.10 | 2 | 164 |
| assets/Image/added | count | 0.00 | 2 | 2 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.02 | 25 | 25 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.00 | 2 | 2 |
| assets/StandardMaterial/modified | count | 2.99 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.41 | 254 | 674 |
| physics/inserts | count | 0.37 | 136 | 621 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.54 | 1 | 900 |
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

## Hitches (> 25.0 ms; slowest 8, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.52 | 516.8 | main/busy/ms 493, surface/advance_cpu_ms 0.73, stream/drive/ms 0.32, surface/cell_upload_ms 0.27, bigspace/total/ms 0.09, stream/tick/ms 0.08, bigspace/high_precision/ms 0.06, weather/map/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.78 | 258.4 | main/busy/ms 530, render/thread/ms 222, render/busy/ms 152, render/acquire/ms 68.22, render/graph/ms 4.81, render/present/ms 1.93, stream/drive/ms 0.98, surface/advance_cpu_ms 0.73, terrain/update/ms 0.53, stream/tick/ms 0.50, terrain/select/ms 0.30, surface/cell_upload_ms 0.27 |
| warm-up | 0.94 | 166.1 | pipelines/wait_ms 1963, render/thread/ms 233, pipelines/compile_ms 181, pipelines/front_ms 166, render/busy/ms 161, render/acquire/ms 68.43, main/busy/ms 52.23, terrain/update/ms 22.44, pipelines/driver_ms 15.46, render/graph/ms 7.91, render/present/ms 2.95, stream/drive/ms 1.08 |
| warm-up | 1.18 | 91.6 | main/busy/ms 60.82, render/thread/ms 52.34, render/busy/ms 50.69, terrain/update/ms 4.60, render/graph/ms 4.22, stream/drive/ms 1.55, render/present/ms 1.37, stream/tick/ms 1.07, terrain/select/ms 0.46, bigspace/total/ms 0.37, render/acquire/ms 0.29, stream/spawn/ms 0.22 |
| warm-up | 1.71 | 33.3 | pipelines/wait_ms 5970, pipelines/compile_ms 47.01, pipelines/front_ms 40.04, render/thread/ms 37.13, render/busy/ms 35.59, main/busy/ms 20.71, pipelines/driver_ms 6.97, render/graph/ms 5.12, stream/drive/ms 3.86, terrain/update/ms 2.36, stream/spawn/ms 2.08, render/present/ms 1.32 |
| warm-up | 1.74 | 29.1 | render/thread/ms 31.14, render/busy/ms 29.89, main/busy/ms 23.38, render/graph/ms 5.11, stream/drive/ms 4.06, stream/spawn/ms 2.18, physics/insert/ms 1.03, render/present/ms 1.02, stream/tick/ms 0.69, bigspace/total/ms 0.57, terrain/update/ms 0.56, terrain/select/ms 0.43 |
| warm-up | 2.08 | 40.7 | pipelines/wait_ms 1123, main/busy/ms 38.97, render/thread/ms 31.51, render/busy/ms 30.22, render/graph/ms 5.54, stream/drive/ms 3.12, stream/spawn/ms 1.61, render/present/ms 1.07, bigspace/total/ms 0.75, stream/tick/ms 0.72, terrain/update/ms 0.56, terrain/select/ms 0.44 |
| warm-up | 3.97 | 30.3 | pipelines/wait_ms 2471, render/thread/ms 35.22, main/busy/ms 34.04, render/busy/ms 33.88, render/graph/ms 28.53, bigspace/total/ms 23.10, bigspace/high_precision/ms 22.66, pipelines/compile_ms 17.28, pipelines/front_ms 16.44, render/present/ms 1.16, pipelines/driver_ms 0.84, terrain/update/ms 0.45 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\push_nocull\balmora_night@2560x1440_off=cpuculltag-1 --bench-baseline none --off cpuculltag
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 97%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.3 W · 75 °C (max 78) · VRAM 3881 MiB · throttled 100% of samples
