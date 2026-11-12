# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: actors · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 7205 frames over 65.6 s · **median 8.90 ms (112 fps)** · mean 9.11 · p95 11.54 · p99 13.43 (1% low 74 fps) · worst 23.17 ms · 78 spikes over 1.5× median · 54.8 stutters/min (work 51.2/min)
- warm-up: 452 frames over 5.3 s, worst 587.8 ms
- hitches (> 25.0 ms): **0** measured, 9 in warm-up
- timed GPU passes: 7.41 ms of a 9.11 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 8.90 ms, limit 11 ms
- **FAIL** frame_ms: 60 stutters (> 1.5x moving median of 60) in 1.09 min = 54.84/min, limit 1/min
- pass worst frame 23.17 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.277 | 2.825 | 0.194 |
| main_opaque_pass_3d | 1.493 | 2.146 | 0.041 |
| early prepass | 0.481 | 0.735 | 0.069 |
| lightprobe_irradiance_map | 0.446 | 0.765 | 0.001 |
| bloom | 0.349 | 0.601 | 0.072 |
| atmosphere_luts | 0.343 | 0.636 | 0.003 |
| lightprobe_radiance_map | 0.245 | 0.432 | 0.004 |
| ssao | 0.224 | 0.446 | 0.008 |
| water_fft | 0.138 | 0.132 | 0.003 |
| uniform_allocation | 0.135 | 0.138 | 0.017 |
| upscaling | 0.132 | 0.163 | 0.005 |
| clouds_trace | 0.128 | 0.213 | 0.001 |
| atmos_froxel_inject | 0.101 | 0.108 | 0.002 |
| tonemapping | 0.096 | 0.100 | 0.004 |
| bin_unpacking | 0.094 | 0.099 | 0.011 |
| water_velocity | 0.087 | 0.087 | 0.002 |
| clustering | 0.069 | 0.070 | 0.021 |
| clouds_composite | 0.066 | 0.064 | 0.002 |
| main_transparent_pass_3d | 0.063 | 0.116 | 0.110 |
| surface_trail | 0.059 | 0.057 | 0.003 |
| render_sky | 0.054 | 0.054 | 0.002 |
| auxiliary_views | 0.042 | 0.296 | 0.041 |
| early_mesh_preprocessing | 0.041 | 0.046 | 0.006 |
| clouds_resolve | 0.035 | 0.040 | 0.001 |
| clouds_shadow | 0.033 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.031 | 0.001 |
| water_mips | 0.025 | 0.024 | 0.002 |
| water_snapshot | 0.025 | 0.024 | 0.001 |
| per_view_shadow_pass | 0.023 | 0.025 | 0.007 |
| lightprobe_copy | 0.017 | 0.016 | 0.005 |
| lightprobe_downsampling_first_pass | 0.015 | 0.014 | 0.001 |
| lightprobe_downsampling_second_pass | 0.013 | 0.011 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| atmosphere_environment | 0.011 | 0.012 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.004 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.002 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.106 | 10.268 | 22.30 | 58404.0 |
| render/busy/ms | 7.292 | 9.518 | 21.45 | 52536.7 |
| main/busy/ms | 6.929 | 8.618 | 14.71 | 49922.3 |
| render/graph/ms | 3.651 | 4.929 | 16.87 | 26308.5 |
| render/present/ms | 0.539 | 1.253 | 2.36 | 3886.2 |
| pipelines/wait_ms | 0.532 | 0.000 | 3830.40 | 3830.5 |
| bigspace/total/ms | 0.318 | 0.536 | 2.43 | 2291.3 |
| terrain/update/ms | 0.297 | 0.375 | 1.94 | 2136.3 |
| render/acquire/ms | 0.275 | 1.221 | 3.04 | 1981.2 |
| terrain/select/ms | 0.260 | 0.328 | 1.00 | 1875.2 |
| stream/drive/ms | 0.252 | 0.369 | 2.29 | 1814.0 |
| stream/tick/ms | 0.209 | 0.302 | 1.05 | 1506.0 |
| far/select/ms | 0.200 | 0.295 | 1.24 | 1437.4 |
| bigspace/high_precision/ms | 0.109 | 0.215 | 1.35 | 783.3 |
| bigspace/low_precision/ms | 0.093 | 0.191 | 1.20 | 667.3 |
| physics/insert/ms | 0.034 | 0.368 | 0.92 | 246.0 |
| pipelines/compile_ms | 0.029 | 0.000 | 91.39 | 207.0 |
| pipelines/front_ms | 0.028 | 0.000 | 88.81 | 201.5 |
| weather/map/ms | 0.028 | 0.036 | 0.32 | 201.0 |
| weather/eval/ms | 0.012 | 0.015 | 0.18 | 83.1 |
| physics/step/ms | 0.010 | 0.020 | 0.52 | 69.2 |
| script/update/ms | 0.007 | 0.011 | 0.20 | 54.0 |
| script/input/ms | 0.005 | 0.007 | 0.19 | 39.0 |
| script/ui/ms | 0.005 | 0.007 | 0.17 | 33.1 |
| stream/despawn/ms | 0.004 | 0.000 | 0.82 | 27.2 |
| stream/spawn/ms | 0.004 | 0.000 | 1.07 | 25.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.66 | 14.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.17 | 8.0 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.58 | 5.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 5.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 4.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.04 | 2.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| ecs/entities | count | 35.0 k | 41.7 k | 33.6 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 7.72 | 21.45 | 7.82 |
| gpu/timed/ms | ms | 7.43 | 10.63 | 7.61 |
| mesh_allocator_allocations |  meshes | 13.0 k | 14.2 k | 13.1 k |
| mesh_allocator_slabs |  slabs | 12 | 12 | 12 |
| mesh_allocator_slabs_size |  bytes | 487.26 M | 487.26 M | 487.26 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 190 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 850 | 1419 | 618 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 190 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 850 | 1419 | 618 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/opaque3d/batch_sets | count | 10 | 10 | 10 |
| phases/opaque3d/bins | count | 880 | 1208 | 1152 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 2794 | 4288 | 2178 |
| phases/opaque3d/pipelines | count | 9 | 9 | 9 |
| phases/opaque3d_prepass/batch_sets | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/bins | count | 880 | 1208 | 1152 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 2794 | 4288 | 2178 |
| phases/opaque3d_prepass/pipelines | count | 9 | 9 | 9 |
| phases/shadow/batch_sets | count | 23 | 198 | 21 |
| phases/shadow/bins | count | 3051 | 4473 | 1745 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8444 | 11.3 k | 4434 |
| phases/shadow/pipelines | count | 90 | 213 | 21 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2689 | 3248 | 2329 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 281 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 147 | 149 | 149 |
| pipelines/waiting | count | 0 | 1 | 0 |
| preprocess/bind_groups | count | 132 | 520 | 130 |
| preprocess/bind_groups_created | count | 132 | 520 | 130 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 27.75 | 45.19 | 32.21 |
| process/mem_usage | GiB | 2.52 | 2.75 | 2.75 |
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
| started_load_count |  | 2863 | 2903 | 2903 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 435 | 530 | 398 |
| statics/materials_bound | count | 435 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.06 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 345 | 437 | 261 |
| stream/oldest_request/s | s | 0 | 0.19 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 40.78 | 78.35 | 68.51 |
| system/mem_usage | % | 54.56 | 56.44 | 56.44 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 56 | 226 | 55 |
| views/all/bins | count | 5090 | 6516 | 4843 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 16.0 k | 21.8 k | 10.0 k |
| views/all/pipelines | count | 120 | 237 | 51 |
| views/all/pipelines_distinct | count | 41 | 47 | 40 |
| views/all/work_items | count | 14.9 k | 20.6 k | 10.3 k |
| views/cascade/batch_sets | count | 23 | 35 | 21 |
| views/cascade/bins | count | 1952 | 2442 | 1745 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7086 | 8899 | 4434 |
| views/cascade/pipelines | count | 23 | 35 | 21 |
| views/cascade/work_items | count | 7086 | 8899 | 4434 |
| views/lamp_face/batch_sets | count | 0 | 173 | 0 |
| views/lamp_face/bins | count | 1023 | 2241 | 0 |
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
| views/main/items | count | 7278 | 11.3 k | 5592 |
| views/main/pipelines | count | 30 | 30 | 30 |
| views/main/work_items | count | 7431 | 11.5 k | 5900 |
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
| assets/Mesh/added | count | 0.48 | 160 | 3454 |
| assets/Mesh/removed | count | 0.68 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.04 | 25 | 263 |
| assets/StandardMaterial/modified | count | 3.01 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 12 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.12 | 1388 | 867 |
| physics/inserts | count | 0.35 | 109 | 2523 |
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
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.05 | 19 | 378 |
| stream/upload/bytes | bytes | 4698 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1030 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 9, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 587.8 | main/busy/ms 566, surface/advance_cpu_ms 68.35, surface/cell_upload_ms 0.72, stream/drive/ms 0.27, weather/map/ms 0.12, bigspace/total/ms 0.09, stream/tick/ms 0.07, bigspace/high_precision/ms 0.04, weather/eval/ms 0.02, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 0.98 | 395.5 | main/busy/ms 600, render/thread/ms 358, render/busy/ms 274, render/acquire/ms 81.30, surface/advance_cpu_ms 68.35, render/graph/ms 4.76, render/present/ms 2.66, stream/drive/ms 1.16, surface/cell_upload_ms 0.72, stream/tick/ms 0.67, terrain/update/ms 0.44, bigspace/total/ms 0.31 |
| warm-up | 1.04 | 61.0 | render/thread/ms 380, render/busy/ms 294, pipelines/wait_ms 122, render/acquire/ms 81.59, pipelines/compile_ms 75.32, pipelines/front_ms 67.60, main/busy/ms 60.86, terrain/update/ms 19.59, render/graph/ms 10.70, pipelines/driver_ms 7.72, render/present/ms 4.21, stream/drive/ms 1.76 |
| warm-up | 1.30 | 76.7 | pipelines/compile_ms 220, pipelines/front_ms 209, render/thread/ms 65.00, main/busy/ms 64.34, render/busy/ms 62.61, pipelines/driver_ms 11.34, render/graph/ms 4.61, stream/drive/ms 2.29, stream/tick/ms 1.70, render/acquire/ms 1.23, render/present/ms 1.16, terrain/update/ms 0.59 |
| warm-up | 1.45 | 28.6 | pipelines/wait_ms 641, pipelines/compile_ms 113, pipelines/front_ms 106, render/thread/ms 32.76, render/busy/ms 31.64, main/busy/ms 15.25, pipelines/driver_ms 7.41, render/graph/ms 4.00, stream/drive/ms 3.19, render/present/ms 0.93, terrain/update/ms 0.91, stream/spawn/ms 0.82 |
| warm-up | 2.04 | 25.0 | pipelines/wait_ms 3574, pipelines/compile_ms 39.01, pipelines/front_ms 35.62, render/thread/ms 32.47, render/busy/ms 30.64, main/busy/ms 22.73, render/graph/ms 4.78, stream/drive/ms 4.54, pipelines/driver_ms 3.38, stream/spawn/ms 2.10, bigspace/total/ms 1.77, render/present/ms 1.57 |
| warm-up | 2.10 | 33.7 | pipelines/wait_ms 962, render/thread/ms 52.30, render/busy/ms 49.81, main/busy/ms 28.60, pipelines/compile_ms 6.86, pipelines/front_ms 5.81, render/graph/ms 5.50, stream/drive/ms 5.17, stream/spawn/ms 2.25, render/present/ms 2.15, stream/tick/ms 1.54, pipelines/driver_ms 1.05 |
| warm-up | 2.15 | 35.3 | render/thread/ms 43.59, render/busy/ms 42.07, main/busy/ms 31.10, render/graph/ms 5.65, terrain/update/ms 5.09, stream/drive/ms 4.53, stream/spawn/ms 2.17, bigspace/total/ms 1.41, render/present/ms 1.25, stream/tick/ms 1.05, physics/insert/ms 0.89, bigspace/low_precision/ms 0.78 |
| warm-up | 2.19 | 25.3 | render/thread/ms 38.99, render/busy/ms 37.80, main/busy/ms 23.19, render/graph/ms 4.28, stream/drive/ms 4.08, terrain/update/ms 3.38, stream/spawn/ms 2.21, physics/insert/ms 1.05, stream/tick/ms 0.95, render/present/ms 0.94, physics/step/ms 0.67, terrain/select/ms 0.66 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ceil_actors_off\balmora_walk@2560x1440_off=actors-3 --bench-baseline none --off actors
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 13% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **93%** (min 66%) · 2580 MHz (min 2580) · memory 8001 MHz · 88.8 W · 75 °C (max 79) · VRAM 4157 MiB · throttled 100% of samples
