# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cpuculltag · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1420 frames over 15.0 s · **median 10.54 ms (95 fps)** · mean 10.57 · p95 11.74 · p99 12.29 (1% low 81 fps) · worst 18.87 ms · 3 spikes over 1.5× median · 16.0 stutters/min (work 12.0/min)
- warm-up: 294 frames over 4.0 s, worst 600.3 ms
- hitches (> 26.4 ms): **0** measured, 5 in warm-up
- timed GPU passes: 9.54 ms of a 10.57 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.54 ms, limit 11 ms
- pass worst frame 18.87 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.647 | 3.347 | 0.056 |
| dlss_super_resolution | 2.380 | 2.953 | 0.195 |
| atmosphere_luts | 0.934 | 1.239 | 0.003 |
| early prepass | 0.605 | 0.941 | 0.086 |
| lightprobe_irradiance_map | 0.447 | 0.748 | 0.001 |
| bloom | 0.371 | 0.692 | 0.069 |
| lightprobe_radiance_map | 0.269 | 0.569 | 0.004 |
| ssao | 0.216 | 0.217 | 0.008 |
| main_transparent_pass_3d | 0.214 | 0.383 | 0.076 |
| uniform_allocation | 0.159 | 0.295 | 0.031 |
| upscaling | 0.140 | 0.277 | 0.005 |
| clouds_trace | 0.138 | 0.164 | 0.001 |
| water_fft | 0.130 | 0.128 | 0.003 |
| tonemapping | 0.116 | 0.303 | 0.004 |
| bin_unpacking | 0.114 | 0.115 | 0.023 |
| atmos_froxel_inject | 0.098 | 0.094 | 0.002 |
| water_velocity | 0.084 | 0.079 | 0.002 |
| clustering | 0.069 | 0.080 | 0.023 |
| clouds_composite | 0.067 | 0.066 | 0.002 |
| render_sky | 0.057 | 0.056 | 0.003 |
| early_mesh_preprocessing | 0.043 | 0.042 | 0.007 |
| clouds_resolve | 0.036 | 0.043 | 0.001 |
| clouds_shadow | 0.032 | 0.036 | 0.004 |
| water_mips | 0.026 | 0.025 | 0.002 |
| water_snapshot | 0.025 | 0.024 | 0.002 |
| atmos_froxel_integrate | 0.025 | 0.032 | 0.001 |
| per_view_shadow_pass | 0.024 | 0.032 | 0.008 |
| lightprobe_downsampling_first_pass | 0.016 | 0.014 | 0.001 |
| lightprobe_copy | 0.016 | 0.015 | 0.004 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.012 | 0.001 |
| atmosphere_environment | 0.010 | 0.014 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.004 | 0.009 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| blas_build | 0.001 | 0.002 | 0.005 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 80.781 | 0.000 | 14644.42 | 114708.5 |
| render/thread/ms | 9.600 | 10.841 | 17.63 | 13631.3 |
| render/busy/ms | 7.311 | 8.670 | 13.36 | 10381.9 |
| main/busy/ms | 7.253 | 8.389 | 13.58 | 10299.7 |
| render/graph/ms | 4.032 | 5.181 | 7.81 | 5725.8 |
| render/acquire/ms | 1.808 | 3.248 | 4.42 | 2567.7 |
| pipelines/compile_ms | 1.435 | 0.000 | 145.43 | 2037.9 |
| pipelines/front_ms | 1.384 | 0.000 | 135.72 | 1965.4 |
| render/present/ms | 0.480 | 0.611 | 5.43 | 681.8 |
| bigspace/total/ms | 0.378 | 0.684 | 1.59 | 536.4 |
| terrain/update/ms | 0.236 | 0.304 | 0.98 | 335.1 |
| terrain/select/ms | 0.205 | 0.263 | 0.94 | 291.0 |
| stream/drive/ms | 0.193 | 0.269 | 0.74 | 273.5 |
| far/select/ms | 0.174 | 0.257 | 0.73 | 246.8 |
| stream/tick/ms | 0.165 | 0.234 | 0.71 | 234.2 |
| controller/feed/ms | 0.155 | 0.213 | 0.42 | 220.5 |
| bigspace/high_precision/ms | 0.135 | 0.322 | 1.36 | 191.7 |
| controller/move/ms | 0.126 | 0.204 | 0.41 | 178.6 |
| bigspace/low_precision/ms | 0.089 | 0.222 | 1.36 | 127.0 |
| anim/evaluate/ms | 0.082 | 0.206 | 1.01 | 116.8 |
| anim/evaluate_cpu/ms | 0.061 | 0.086 | 0.28 | 87.3 |
| pipelines/driver_ms | 0.051 | 0.000 | 9.72 | 72.5 |
| physics/step/ms | 0.038 | 0.067 | 1.01 | 53.4 |
| weather/map/ms | 0.027 | 0.036 | 0.31 | 38.9 |
| physics/insert/ms | 0.020 | 0.128 | 0.77 | 28.5 |
| weather/eval/ms | 0.012 | 0.016 | 0.03 | 17.2 |
| script/update/ms | 0.008 | 0.010 | 0.12 | 11.0 |
| controller/avoid/ms | 0.006 | 0.008 | 0.21 | 8.4 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 7.7 |
| anim/apply/ms | 0.005 | 0.006 | 0.19 | 6.5 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 6.4 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
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
| anim/a2/count | count | 0 | 6 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 31 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| ecs/entities | count | 47.0 k | 47.0 k | 47.0 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 9.59 | 13.58 | 9.63 |
| gpu/timed/ms | ms | 9.58 | 11.16 | 9.63 |
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
| phases/opaque3d/batch_sets | count | 14 | 16 | 14 |
| phases/opaque3d/bins | count | 1309 | 1310 | 1309 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3771 | 3771 | 3771 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 13 | 15 | 13 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 16 | 14 |
| phases/opaque3d_prepass/bins | count | 1309 | 1310 | 1309 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3771 | 3771 | 3771 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 13 | 15 | 13 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 23 | 26 | 23 |
| phases/shadow/bins | count | 2035 | 2343 | 2035 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5352 | 5679 | 5352 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 23 | 26 | 23 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 2181 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 34 | 0 |
| preprocess/bind_groups | count | 0 | 151 | 0 |
| preprocess/bind_groups_created | count | 0 | 151 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 154 | 154 | 154 |
| process/cpu_usage | % | 24.94 | 37.69 | 25.71 |
| process/mem_usage | GiB | 2.05 | 2.19 | 2.19 |
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
| started_load_count |  | 2597 | 2599 | 2599 |
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
| system/cpu_usage | % | 38.71 | 56.91 | 40.88 |
| system/mem_usage | % | 49.89 | 50.31 | 50.31 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 67 | 70 | 67 |
| views/all/bins | count | 5217 | 5525 | 5217 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 15.0 k | 14.6 k |
| views/all/pipelines | count | 63 | 66 | 63 |
| views/all/pipelines_distinct | count | 55 | 59 | 55 |
| views/all/work_items | count | 14.8 k | 15.2 k | 14.8 k |
| views/cascade/batch_sets | count | 23 | 26 | 23 |
| views/cascade/bins | count | 2035 | 2343 | 2035 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5352 | 5679 | 5352 |
| views/cascade/pipelines | count | 23 | 26 | 23 |
| views/cascade/work_items | count | 5352 | 5679 | 5352 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 3182 | 3184 | 3182 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9294 | 9294 | 9294 |
| views/main/pipelines | count | 40 | 44 | 40 |
| views/main/work_items | count | 9472 | 9472 | 9472 |
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
| anim/bones_written/count | count | 0.05 | 66 | 66 |
| anim/events/count | count | 0.10 | 3 | 148 |
| assets/Image/added | count | 0.05 | 23 | 72 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.19 | 92 | 276 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.05 | 24 | 74 |
| assets/StandardMaterial/modified | count | 3.49 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 3.32 | 1500 | 4708 |
| physics/inserts | count | 1.28 | 231 | 1821 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.63 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.01 | 5 | 9 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 5 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.4 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.60 | 600.3 | main/busy/ms 575, surface/advance_cpu_ms 67.77, surface/cell_upload_ms 0.72, stream/drive/ms 0.42, bigspace/total/ms 0.12, stream/tick/ms 0.10, bigspace/high_precision/ms 0.10, weather/map/ms 0.09, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.97 | 366.4 | main/busy/ms 612, render/thread/ms 326, render/busy/ms 246, render/acquire/ms 77.62, surface/advance_cpu_ms 67.77, render/graph/ms 7.11, render/present/ms 2.29, stream/drive/ms 0.93, surface/cell_upload_ms 0.72, terrain/update/ms 0.47, stream/tick/ms 0.44, terrain/select/ms 0.29 |
| warm-up | 1.04 | 77.5 | pipelines/wait_ms 877, render/thread/ms 338, render/busy/ms 256, render/acquire/ms 77.86, pipelines/compile_ms 77.51, pipelines/front_ms 68.15, main/busy/ms 54.30, terrain/update/ms 22.98, render/graph/ms 10.80, pipelines/driver_ms 9.36, render/present/ms 3.48, bigspace/total/ms 1.61 |
| warm-up | 1.30 | 97.7 | pipelines/compile_ms 219, pipelines/front_ms 198, main/busy/ms 65.53, render/thread/ms 62.27, render/busy/ms 60.17, pipelines/driver_ms 20.76, render/graph/ms 5.08, render/present/ms 1.84, stream/drive/ms 1.43, stream/tick/ms 1.13, terrain/update/ms 0.66, terrain/select/ms 0.53 |
| warm-up | 2.96 | 28.5 | pipelines/wait_ms 16.9 k, main/busy/ms 41.82, pipelines/compile_ms 37.21, pipelines/front_ms 32.71, render/thread/ms 19.46, render/busy/ms 17.87, render/graph/ms 5.65, pipelines/driver_ms 4.50, render/present/ms 1.36, stream/drive/ms 1.26, stream/tick/ms 0.57, bigspace/total/ms 0.57 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\push_nocull\balmora@2560x1440_off=cpuculltag-2 --bench-baseline none --off cpuculltag
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 14%
- during the run: other processes 14% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **97%** (min 65%) · 2581 MHz (min 2580) · memory 8001 MHz · 88.5 W · 70 °C (max 73) · VRAM 3947 MiB · throttled 100% of samples
