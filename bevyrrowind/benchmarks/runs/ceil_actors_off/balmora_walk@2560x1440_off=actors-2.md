# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: actors · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 7219 frames over 65.6 s · **median 8.93 ms (112 fps)** · mean 9.09 · p95 11.25 · p99 13.08 (1% low 76 fps) · worst 30.69 ms · 60 spikes over 1.5× median · 51.2 stutters/min (work 49.4/min)
- warm-up: 364 frames over 4.6 s, worst 573.5 ms
- hitches (> 25.0 ms): **2** measured, 4 in warm-up
- timed GPU passes: 7.27 ms of a 9.09 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 8.93 ms, limit 11 ms
- **FAIL** frame_ms: 56 stutters (> 1.5x moving median of 60) in 1.09 min = 51.19/min, limit 1/min
- pass worst frame 30.69 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.180 | 2.725 | 0.191 |
| main_opaque_pass_3d | 1.461 | 2.126 | 0.043 |
| early prepass | 0.477 | 0.744 | 0.069 |
| lightprobe_irradiance_map | 0.450 | 0.778 | 0.001 |
| atmosphere_luts | 0.361 | 0.679 | 0.004 |
| bloom | 0.334 | 0.477 | 0.071 |
| lightprobe_radiance_map | 0.242 | 0.393 | 0.004 |
| ssao | 0.238 | 0.564 | 0.008 |
| water_fft | 0.143 | 0.315 | 0.003 |
| uniform_allocation | 0.130 | 0.132 | 0.019 |
| upscaling | 0.128 | 0.158 | 0.005 |
| clouds_trace | 0.127 | 0.213 | 0.001 |
| atmos_froxel_inject | 0.096 | 0.103 | 0.002 |
| bin_unpacking | 0.091 | 0.098 | 0.012 |
| tonemapping | 0.090 | 0.091 | 0.004 |
| water_velocity | 0.085 | 0.086 | 0.002 |
| clustering | 0.068 | 0.069 | 0.021 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.064 | 0.113 | 0.112 |
| surface_trail | 0.059 | 0.057 | 0.003 |
| render_sky | 0.053 | 0.054 | 0.002 |
| auxiliary_views | 0.046 | 0.299 | 0.045 |
| early_mesh_preprocessing | 0.040 | 0.045 | 0.006 |
| clouds_resolve | 0.035 | 0.037 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| water_mips | 0.025 | 0.024 | 0.002 |
| atmos_froxel_integrate | 0.025 | 0.030 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.024 | 0.007 |
| lightprobe_downsampling_first_pass | 0.015 | 0.014 | 0.001 |
| lightprobe_copy | 0.015 | 0.016 | 0.005 |
| lightprobe_downsampling_second_pass | 0.013 | 0.012 | 0.001 |
| atmosphere_environment | 0.012 | 0.012 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.004 |
| night_sky | 0.006 | 0.012 | 0.002 |
| blas_build | 0.001 | 0.002 | 0.006 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.104 | 10.080 | 29.66 | 58500.4 |
| render/busy/ms | 7.340 | 9.316 | 29.15 | 52990.8 |
| main/busy/ms | 6.922 | 8.478 | 13.80 | 49971.5 |
| pipelines/wait_ms | 5.738 | 0.000 | 7177.65 | 41421.8 |
| render/graph/ms | 3.658 | 4.835 | 25.65 | 26404.0 |
| render/present/ms | 0.542 | 1.262 | 2.26 | 3913.3 |
| bigspace/total/ms | 0.308 | 0.493 | 1.62 | 2221.3 |
| terrain/update/ms | 0.297 | 0.372 | 1.75 | 2140.6 |
| terrain/select/ms | 0.260 | 0.326 | 0.69 | 1878.0 |
| stream/drive/ms | 0.251 | 0.366 | 4.65 | 1809.6 |
| render/acquire/ms | 0.221 | 1.069 | 2.77 | 1596.3 |
| stream/tick/ms | 0.207 | 0.298 | 0.79 | 1490.9 |
| far/select/ms | 0.201 | 0.296 | 0.60 | 1450.0 |
| bigspace/high_precision/ms | 0.104 | 0.180 | 1.46 | 751.4 |
| pipelines/compile_ms | 0.092 | 0.000 | 163.64 | 661.3 |
| bigspace/low_precision/ms | 0.090 | 0.176 | 1.19 | 652.3 |
| pipelines/front_ms | 0.089 | 0.000 | 160.09 | 642.8 |
| physics/insert/ms | 0.033 | 0.368 | 1.04 | 237.9 |
| weather/map/ms | 0.028 | 0.037 | 0.25 | 200.6 |
| weather/eval/ms | 0.012 | 0.016 | 0.17 | 84.5 |
| physics/step/ms | 0.010 | 0.020 | 1.04 | 74.8 |
| script/update/ms | 0.008 | 0.011 | 0.28 | 55.9 |
| script/input/ms | 0.005 | 0.007 | 0.26 | 38.8 |
| script/ui/ms | 0.004 | 0.007 | 0.22 | 31.9 |
| stream/despawn/ms | 0.004 | 0.000 | 0.69 | 26.3 |
| stream/spawn/ms | 0.003 | 0.000 | 1.08 | 24.8 |
| pipelines/driver_ms | 0.003 | 0.000 | 3.55 | 18.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.61 | 14.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.33 | 8.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.40 | 4.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.06 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| ecs/entities | count | 35.1 k | 41.7 k | 33.6 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 7.69 | 29.15 | 7.68 |
| gpu/timed/ms | ms | 7.28 | 9.33 | 7.68 |
| mesh_allocator_allocations |  meshes | 13.1 k | 14.2 k | 13.1 k |
| mesh_allocator_slabs |  slabs | 12 | 12 | 12 |
| mesh_allocator_slabs_size |  bytes | 487.26 M | 487.26 M | 487.26 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 191 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 847 | 1419 | 617 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 191 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 847 | 1419 | 617 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/opaque3d/batch_sets | count | 10 | 10 | 10 |
| phases/opaque3d/bins | count | 879 | 1208 | 1152 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 2790 | 4288 | 2175 |
| phases/opaque3d/pipelines | count | 9 | 9 | 9 |
| phases/opaque3d_prepass/batch_sets | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/bins | count | 879 | 1208 | 1152 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 2790 | 4288 | 2175 |
| phases/opaque3d_prepass/pipelines | count | 9 | 9 | 9 |
| phases/shadow/batch_sets | count | 23 | 198 | 21 |
| phases/shadow/bins | count | 3048 | 4469 | 1745 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8453 | 11.4 k | 4430 |
| phases/shadow/pipelines | count | 90 | 213 | 21 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2689 | 3248 | 2329 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 694 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 142 | 144 | 144 |
| pipelines/waiting | count | 0 | 12 | 0 |
| preprocess/bind_groups | count | 132 | 520 | 130 |
| preprocess/bind_groups_created | count | 132 | 520 | 130 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 27.88 | 47.12 | 25.48 |
| process/mem_usage | GiB | 2.48 | 2.61 | 2.61 |
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
| started_load_count |  | 2863 | 2902 | 2902 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 435 | 530 | 398 |
| statics/materials_bound | count | 435 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.06 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 345 | 437 | 261 |
| stream/oldest_request/s | s | 0 | 0.19 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 41.22 | 70.49 | 57.33 |
| system/mem_usage | % | 53.76 | 54.34 | 54.08 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 56 | 226 | 55 |
| views/all/bins | count | 5080 | 6525 | 4843 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 16.0 k | 22.1 k | 10.0 k |
| views/all/pipelines | count | 120 | 237 | 51 |
| views/all/pipelines_distinct | count | 41 | 47 | 40 |
| views/all/work_items | count | 14.9 k | 20.6 k | 10.3 k |
| views/cascade/batch_sets | count | 23 | 35 | 21 |
| views/cascade/bins | count | 1954 | 2442 | 1745 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7095 | 8952 | 4430 |
| views/cascade/pipelines | count | 23 | 35 | 21 |
| views/cascade/work_items | count | 7095 | 8952 | 4430 |
| views/lamp_face/batch_sets | count | 0 | 173 | 0 |
| views/lamp_face/bins | count | 1021 | 2241 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1186 | 2609 | 0 |
| views/lamp_face/pipelines | count | 64 | 178 | 0 |
| views/lamp_face/work_items | count | 0 | 2327 | 0 |
| views/main/batch_sets | count | 34 | 34 | 34 |
| views/main/bins | count | 2086 | 3210 | 3098 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7270 | 11.3 k | 5584 |
| views/main/pipelines | count | 30 | 30 | 30 |
| views/main/work_items | count | 7422 | 11.5 k | 5892 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 28.0 k | 33.7 k | 26.9 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| assets/Image/added | count | 0.02 | 18 | 136 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.48 | 176 | 3454 |
| assets/Mesh/removed | count | 0.68 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.04 | 24 | 263 |
| assets/StandardMaterial/modified | count | 3.00 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 12 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.12 | 1556 | 867 |
| physics/inserts | count | 0.41 | 151 | 2988 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.32 | 145 | 2335 |
| physics/steps | count | 0.55 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 4 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 21 | 223 |
| stream/despawn/items | count | 0.03 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.05 | 24 | 378 |
| stream/upload/bytes | bytes | 4689 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 55 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1028 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.57 | 573.5 | main/busy/ms 551, surface/advance_cpu_ms 65.32, surface/cell_upload_ms 1.10, stream/drive/ms 0.32, bigspace/total/ms 0.12, stream/tick/ms 0.09, weather/map/ms 0.08, bigspace/high_precision/ms 0.08, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 0.90 | 328.3 | main/busy/ms 588, render/thread/ms 294, render/busy/ms 210, render/acquire/ms 81.97, surface/advance_cpu_ms 65.32, render/graph/ms 4.95, render/present/ms 2.26, surface/cell_upload_ms 1.10, stream/drive/ms 1.06, terrain/update/ms 0.59, stream/tick/ms 0.54, terrain/select/ms 0.41 |
| warm-up | 0.98 | 79.9 | pipelines/wait_ms 985, render/thread/ms 312, render/busy/ms 227, render/acquire/ms 82.10, pipelines/compile_ms 76.70, pipelines/front_ms 70.74, main/busy/ms 53.51, terrain/update/ms 22.32, render/graph/ms 10.02, pipelines/driver_ms 5.96, render/present/ms 3.69, stream/drive/ms 1.27 |
| warm-up | 1.21 | 77.4 | pipelines/wait_ms 539, pipelines/compile_ms 186, pipelines/front_ms 174, main/busy/ms 52.34, render/thread/ms 42.06, render/busy/ms 40.63, pipelines/driver_ms 11.65, render/graph/ms 3.10, stream/drive/ms 1.33, render/present/ms 1.25, stream/tick/ms 1.03, terrain/update/ms 0.61 |
| measure | 2.16 | 25.8 | render/thread/ms 33.12, render/busy/ms 31.67, render/graph/ms 20.45, main/busy/ms 17.65, pipelines/compile_ms 13.71, pipelines/front_ms 13.42, render/present/ms 1.24, bigspace/total/ms 0.82, terrain/update/ms 0.62, terrain/select/ms 0.54, stream/drive/ms 0.50, stream/tick/ms 0.43 |
| measure | 3.90 | 30.7 | render/thread/ms 36.87, render/busy/ms 34.67, render/graph/ms 28.01, main/busy/ms 15.05, render/present/ms 2.00, bigspace/total/ms 0.65, terrain/update/ms 0.54, stream/drive/ms 0.51, terrain/select/ms 0.48, stream/tick/ms 0.44, far/select/ms 0.34, bigspace/low_precision/ms 0.25 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ceil_actors_off\balmora_walk@2560x1440_off=actors-2 --bench-baseline none --off actors
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 20%
- during the run: other processes 13% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **91%** (min 70%) · 2580 MHz (min 2580) · memory 8001 MHz · 87.3 W · 74 °C (max 79) · VRAM 4474 MiB · throttled 100% of samples
