# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cpuculltag · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1517 frames over 15.0 s · **median 9.82 ms (102 fps)** · mean 9.89 · p95 10.92 · p99 11.59 (1% low 86 fps) · worst 20.77 ms · 4 spikes over 1.5× median · 20.0 stutters/min (work 12.0/min)
- warm-up: 295 frames over 4.0 s, worst 582.6 ms
- hitches (> 25.0 ms): **0** measured, 9 in warm-up
- timed GPU passes: 8.90 ms of a 9.89 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.82 ms, limit 11 ms
- pass worst frame 20.77 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.417 | 2.810 | 0.043 |
| dlss_super_resolution | 2.096 | 2.418 | 0.184 |
| atmosphere_luts | 1.020 | 1.255 | 0.004 |
| early prepass | 0.620 | 0.935 | 0.083 |
| lightprobe_irradiance_map | 0.422 | 0.418 | 0.001 |
| bloom | 0.324 | 0.336 | 0.066 |
| lightprobe_radiance_map | 0.226 | 0.233 | 0.004 |
| ssao | 0.224 | 0.357 | 0.007 |
| main_transparent_pass_3d | 0.194 | 0.203 | 0.070 |
| uniform_allocation | 0.143 | 0.150 | 0.027 |
| water_fft | 0.138 | 0.272 | 0.002 |
| clouds_trace | 0.137 | 0.163 | 0.001 |
| upscaling | 0.121 | 0.126 | 0.004 |
| bin_unpacking | 0.102 | 0.109 | 0.020 |
| atmos_froxel_inject | 0.091 | 0.091 | 0.001 |
| tonemapping | 0.088 | 0.093 | 0.003 |
| water_velocity | 0.076 | 0.076 | 0.001 |
| clustering | 0.065 | 0.077 | 0.022 |
| clouds_composite | 0.064 | 0.066 | 0.002 |
| render_sky | 0.054 | 0.055 | 0.002 |
| early_mesh_preprocessing | 0.039 | 0.040 | 0.005 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| clouds_shadow | 0.032 | 0.036 | 0.003 |
| water_mips | 0.024 | 0.025 | 0.002 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| per_view_shadow_pass | 0.023 | 0.031 | 0.007 |
| atmos_froxel_integrate | 0.023 | 0.025 | 0.001 |
| lightprobe_copy | 0.016 | 0.016 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.012 | 0.015 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| night_sky | 0.003 | 0.004 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| blas_build | 0.001 | 0.002 | 0.005 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 51.209 | 0.000 | 4897.52 | 77684.8 |
| render/thread/ms | 8.992 | 10.004 | 18.30 | 13641.2 |
| main/busy/ms | 6.620 | 7.684 | 14.50 | 10042.8 |
| render/busy/ms | 6.525 | 7.770 | 14.55 | 9899.1 |
| render/graph/ms | 3.570 | 4.692 | 11.50 | 5415.1 |
| render/acquire/ms | 2.020 | 3.120 | 3.86 | 3064.5 |
| pipelines/compile_ms | 1.157 | 0.000 | 139.54 | 1755.9 |
| pipelines/front_ms | 1.121 | 0.000 | 135.60 | 1700.0 |
| render/present/ms | 0.447 | 0.558 | 3.30 | 677.6 |
| bigspace/total/ms | 0.350 | 0.627 | 1.72 | 530.6 |
| terrain/update/ms | 0.237 | 0.307 | 0.72 | 359.0 |
| terrain/select/ms | 0.205 | 0.263 | 0.68 | 311.5 |
| stream/drive/ms | 0.173 | 0.233 | 0.54 | 262.0 |
| far/select/ms | 0.155 | 0.204 | 0.47 | 234.7 |
| stream/tick/ms | 0.148 | 0.203 | 0.51 | 224.8 |
| controller/move/ms | 0.134 | 0.297 | 0.52 | 202.5 |
| bigspace/high_precision/ms | 0.130 | 0.296 | 1.47 | 197.5 |
| controller/feed/ms | 0.121 | 0.184 | 0.53 | 183.3 |
| bigspace/low_precision/ms | 0.077 | 0.197 | 1.05 | 116.2 |
| anim/evaluate/ms | 0.068 | 0.149 | 1.29 | 103.1 |
| anim/evaluate_cpu/ms | 0.062 | 0.086 | 0.37 | 94.7 |
| pipelines/driver_ms | 0.037 | 0.000 | 4.83 | 55.9 |
| physics/step/ms | 0.030 | 0.056 | 0.88 | 46.1 |
| weather/map/ms | 0.025 | 0.033 | 0.10 | 38.5 |
| physics/insert/ms | 0.016 | 0.102 | 0.83 | 24.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.03 | 16.5 |
| script/update/ms | 0.006 | 0.008 | 0.11 | 9.3 |
| controller/avoid/ms | 0.005 | 0.007 | 0.02 | 7.7 |
| script/input/ms | 0.005 | 0.006 | 0.13 | 7.5 |
| anim/apply/ms | 0.004 | 0.006 | 0.10 | 6.5 |
| script/ui/ms | 0.004 | 0.006 | 0.17 | 5.4 |
| controller/write/ms | 0.002 | 0.002 | 0.03 | 2.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.0 |
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
| anim/a2/count | count | 0 | 11 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 58 | 7 |
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
| frame/work/ms | ms | 8.92 | 14.55 | 8.46 |
| gpu/timed/ms | ms | 8.91 | 10.52 | 8.46 |
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
| phases/opaque3d/bins | count | 1309 | 1310 | 1309 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3771 | 3771 | 3771 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 13 | 14 | 13 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/bins | count | 1309 | 1310 | 1309 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3771 | 3771 | 3771 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 13 | 14 | 13 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 23 | 26 | 23 |
| phases/shadow/bins | count | 2036 | 2343 | 2036 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5353 | 5679 | 5353 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 23 | 26 | 23 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1433 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 29 | 0 |
| preprocess/bind_groups | count | 0 | 154 | 0 |
| preprocess/bind_groups_created | count | 0 | 154 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 154 | 154 | 154 |
| process/cpu_usage | % | 25 | 39.42 | 37.02 |
| process/mem_usage | GiB | 2.13 | 2.19 | 2.19 |
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
| stream/latency/s | s | 0.05 | 0.05 | 0.05 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.34 | 52.07 | 34.09 |
| system/mem_usage | % | 50.41 | 50.60 | 50.59 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 67 | 72 | 67 |
| views/all/bins | count | 5218 | 5527 | 5218 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 15.0 k | 14.6 k |
| views/all/pipelines | count | 63 | 68 | 63 |
| views/all/pipelines_distinct | count | 55 | 57 | 55 |
| views/all/work_items | count | 14.8 k | 15.2 k | 14.8 k |
| views/cascade/batch_sets | count | 23 | 26 | 23 |
| views/cascade/bins | count | 2036 | 2343 | 2036 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5353 | 5679 | 5353 |
| views/cascade/pipelines | count | 23 | 26 | 23 |
| views/cascade/work_items | count | 5353 | 5679 | 5353 |
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
| views/main/bins | count | 3182 | 3184 | 3182 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9294 | 9294 | 9294 |
| views/main/pipelines | count | 40 | 42 | 40 |
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
| anim/bones_written/count | count | 0.13 | 165 | 198 |
| anim/events/count | count | 0.08 | 2 | 122 |
| assets/Image/added | count | 0.05 | 36 | 69 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.12 | 103 | 183 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.05 | 40 | 74 |
| assets/StandardMaterial/modified | count | 3.26 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 3.97 | 2222 | 6022 |
| physics/inserts | count | 0.85 | 155 | 1297 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.59 | 1 | 900 |
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

## Hitches (> 25.0 ms; slowest 9, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 582.6 | main/busy/ms 561, surface/advance_cpu_ms 67.50, surface/cell_upload_ms 0.68, stream/drive/ms 0.48, weather/map/ms 0.12, stream/tick/ms 0.12, bigspace/total/ms 0.06, bigspace/high_precision/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.89 | 305.3 | main/busy/ms 595, render/thread/ms 269, render/busy/ms 196, render/acquire/ms 71.31, surface/advance_cpu_ms 67.50, render/graph/ms 5.23, render/present/ms 2.02, stream/drive/ms 1.22, surface/cell_upload_ms 0.68, stream/tick/ms 0.61, terrain/update/ms 0.55, terrain/select/ms 0.37 |
| warm-up | 0.98 | 88.5 | pipelines/wait_ms 1726, render/thread/ms 279, render/busy/ms 205, pipelines/compile_ms 99.73, pipelines/front_ms 89.98, render/acquire/ms 71.56, main/busy/ms 49.29, terrain/update/ms 20.35, pipelines/driver_ms 9.75, render/graph/ms 8.56, render/present/ms 2.94, stream/drive/ms 1.18 |
| warm-up | 1.22 | 92.4 | pipelines/compile_ms 190, pipelines/front_ms 172, pipelines/wait_ms 171, main/busy/ms 72.07, render/thread/ms 52.00, render/busy/ms 50.22, pipelines/driver_ms 17.88, render/graph/ms 4.63, stream/drive/ms 1.88, render/present/ms 1.53, stream/tick/ms 1.44, terrain/update/ms 0.52 |
| warm-up | 2.60 | 31.8 | pipelines/wait_ms 4731, pipelines/compile_ms 81.21, pipelines/front_ms 77.68, main/busy/ms 56.54, render/thread/ms 31.23, render/busy/ms 28.75, render/graph/ms 9.57, pipelines/driver_ms 3.53, terrain/update/ms 2.53, render/present/ms 2.21, stream/drive/ms 1.93, stream/tick/ms 0.90 |
| warm-up | 2.64 | 40.9 | pipelines/wait_ms 14.4 k, pipelines/compile_ms 92.91, pipelines/front_ms 86.89, main/busy/ms 65.93, render/thread/ms 40.76, render/busy/ms 38.28, render/graph/ms 8.67, pipelines/driver_ms 6.02, render/present/ms 2.24, stream/drive/ms 1.73, anim/evaluate_cpu/ms 1.55, bigspace/total/ms 1.24 |
| warm-up | 2.68 | 41.6 | pipelines/wait_ms 9684, main/busy/ms 73.27, render/thread/ms 48.59, render/busy/ms 46.73, pipelines/compile_ms 11.70, render/graph/ms 9.95, pipelines/front_ms 9.21, pipelines/driver_ms 2.49, stream/drive/ms 1.79, render/present/ms 1.60, anim/evaluate_cpu/ms 1.27, bigspace/total/ms 1.21 |
| warm-up | 2.71 | 31.0 | main/busy/ms 54.93, render/thread/ms 42.54, render/busy/ms 40.73, render/graph/ms 8.68, stream/drive/ms 1.83, render/present/ms 1.52, bigspace/total/ms 1.28, stream/tick/ms 0.70, bigspace/low_precision/ms 0.70, stream/spawn/ms 0.60, terrain/update/ms 0.57, terrain/select/ms 0.45 |
| warm-up | 2.90 | 43.9 | pipelines/wait_ms 4194, pipelines/compile_ms 103, pipelines/driver_ms 53.58, render/thread/ms 53.56, render/busy/ms 52.20, pipelines/front_ms 49.27, render/graph/ms 41.02, main/busy/ms 16.65, stream/drive/ms 1.35, render/present/ms 1.11, stream/tick/ms 0.64, bigspace/total/ms 0.47 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\push_nocull\balmora@2560x1440_off=cpuculltag-3 --bench-baseline none --off cpuculltag
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **97%** (min 87%) · 2580 MHz (min 2580) · memory 8001 MHz · 91.9 W · 71 °C (max 73) · VRAM 3899 MiB · throttled 100% of samples
