# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: actors · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 7189 frames over 65.6 s · **median 8.88 ms (113 fps)** · mean 9.13 · p95 11.73 · p99 13.82 (1% low 72 fps) · worst 34.18 ms · 90 spikes over 1.5× median · 60.3 stutters/min (work 64.0/min)
- warm-up: 334 frames over 4.3 s, worst 580.1 ms
- hitches (> 25.0 ms): **4** measured, 7 in warm-up
- timed GPU passes: 7.21 ms of a 9.13 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 8.88 ms, limit 11 ms
- **FAIL** frame_ms: 66 stutters (> 1.5x moving median of 60) in 1.09 min = 60.33/min, limit 1/min
- **FAIL** worst frame 34.18 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.166 | 2.652 | 0.194 |
| main_opaque_pass_3d | 1.425 | 2.111 | 0.045 |
| early prepass | 0.474 | 0.857 | 0.070 |
| lightprobe_irradiance_map | 0.457 | 0.791 | 0.001 |
| atmosphere_luts | 0.361 | 0.706 | 0.004 |
| bloom | 0.332 | 0.338 | 0.073 |
| lightprobe_radiance_map | 0.243 | 0.391 | 0.005 |
| ssao | 0.233 | 0.566 | 0.008 |
| water_fft | 0.142 | 0.134 | 0.003 |
| uniform_allocation | 0.130 | 0.132 | 0.019 |
| clouds_trace | 0.127 | 0.216 | 0.001 |
| upscaling | 0.124 | 0.144 | 0.005 |
| atmos_froxel_inject | 0.096 | 0.102 | 0.002 |
| bin_unpacking | 0.093 | 0.097 | 0.013 |
| tonemapping | 0.090 | 0.091 | 0.004 |
| water_velocity | 0.085 | 0.085 | 0.002 |
| clustering | 0.070 | 0.069 | 0.022 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.059 | 0.102 | 0.112 |
| surface_trail | 0.058 | 0.057 | 0.003 |
| render_sky | 0.053 | 0.053 | 0.002 |
| auxiliary_views | 0.043 | 0.296 | 0.040 |
| early_mesh_preprocessing | 0.040 | 0.046 | 0.006 |
| clouds_resolve | 0.034 | 0.036 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| water_mips | 0.025 | 0.024 | 0.002 |
| atmos_froxel_integrate | 0.024 | 0.029 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| per_view_shadow_pass | 0.023 | 0.024 | 0.007 |
| lightprobe_copy | 0.015 | 0.016 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| lightprobe_downsampling_second_pass | 0.013 | 0.011 | 0.001 |
| atmosphere_environment | 0.013 | 0.012 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.004 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.002 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.116 | 10.559 | 27.54 | 58342.9 |
| render/busy/ms | 7.424 | 9.802 | 25.92 | 53369.8 |
| main/busy/ms | 7.011 | 8.522 | 28.24 | 50403.3 |
| pipelines/wait_ms | 5.698 | 0.000 | 3405.03 | 40960.6 |
| render/graph/ms | 3.653 | 4.940 | 12.93 | 26262.8 |
| render/present/ms | 0.569 | 1.317 | 2.93 | 4093.1 |
| bigspace/total/ms | 0.309 | 0.471 | 2.17 | 2218.4 |
| terrain/update/ms | 0.296 | 0.378 | 1.45 | 2125.7 |
| terrain/select/ms | 0.259 | 0.329 | 0.75 | 1865.4 |
| stream/drive/ms | 0.258 | 0.359 | 3.51 | 1854.8 |
| stream/tick/ms | 0.213 | 0.293 | 0.77 | 1528.8 |
| far/select/ms | 0.209 | 0.319 | 0.80 | 1504.6 |
| render/acquire/ms | 0.122 | 0.249 | 2.02 | 879.9 |
| pipelines/compile_ms | 0.122 | 0.000 | 134.21 | 877.5 |
| pipelines/front_ms | 0.118 | 0.000 | 129.90 | 848.8 |
| bigspace/high_precision/ms | 0.101 | 0.164 | 1.09 | 726.0 |
| bigspace/low_precision/ms | 0.092 | 0.175 | 1.04 | 658.1 |
| physics/insert/ms | 0.032 | 0.350 | 1.01 | 230.3 |
| weather/map/ms | 0.029 | 0.038 | 0.53 | 205.0 |
| physics/step/ms | 0.014 | 0.035 | 0.53 | 104.0 |
| weather/eval/ms | 0.012 | 0.016 | 0.28 | 85.5 |
| script/update/ms | 0.008 | 0.012 | 0.18 | 57.2 |
| script/input/ms | 0.005 | 0.007 | 0.27 | 38.7 |
| script/ui/ms | 0.004 | 0.007 | 0.17 | 31.7 |
| pipelines/driver_ms | 0.004 | 0.000 | 5.32 | 28.8 |
| stream/despawn/ms | 0.004 | 0.000 | 0.70 | 26.4 |
| stream/spawn/ms | 0.004 | 0.000 | 1.12 | 26.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.70 | 14.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.04 | 7.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.18 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.37 | 4.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| ecs/entities | count | 35.4 k | 41.7 k | 33.6 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 7.58 | 28.24 | 8.18 |
| gpu/timed/ms | ms | 7.16 | 9.41 | 8.18 |
| mesh_allocator_allocations |  meshes | 13.0 k | 14.2 k | 13.1 k |
| mesh_allocator_slabs |  slabs | 12 | 12 | 12 |
| mesh_allocator_slabs_size |  bytes | 487.26 M | 487.26 M | 487.26 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 182 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 895 | 1419 | 617 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 182 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 895 | 1419 | 617 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/opaque3d/batch_sets | count | 10 | 10 | 10 |
| phases/opaque3d/bins | count | 877 | 1208 | 1152 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 2882 | 4285 | 2176 |
| phases/opaque3d/pipelines | count | 9 | 9 | 9 |
| phases/opaque3d_prepass/batch_sets | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/bins | count | 877 | 1208 | 1152 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 2882 | 4285 | 2176 |
| phases/opaque3d_prepass/pipelines | count | 9 | 9 | 9 |
| phases/shadow/batch_sets | count | 23 | 198 | 21 |
| phases/shadow/bins | count | 3193 | 4465 | 1745 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8523 | 11.4 k | 4430 |
| phases/shadow/pipelines | count | 107 | 213 | 21 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2711 | 3248 | 2329 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 395 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 142 | 144 | 144 |
| pipelines/waiting | count | 0 | 13 | 0 |
| preprocess/bind_groups | count | 134 | 520 | 130 |
| preprocess/bind_groups_created | count | 134 | 520 | 130 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 26.92 | 41.35 | 30.29 |
| process/mem_usage | GiB | 2.47 | 2.57 | 2.57 |
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
| statics/materials | count | 437 | 530 | 398 |
| statics/materials_bound | count | 437 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
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
| system/cpu_usage | % | 39.61 | 91.64 | 65.91 |
| system/mem_usage | % | 52.59 | 56.55 | 56.55 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 57 | 226 | 55 |
| views/all/bins | count | 5164 | 6502 | 4843 |
| views/all/count | count | 28 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 16.4 k | 22.1 k | 10.0 k |
| views/all/pipelines | count | 137 | 237 | 51 |
| views/all/pipelines_distinct | count | 44 | 47 | 40 |
| views/all/work_items | count | 15.0 k | 20.6 k | 10.3 k |
| views/cascade/batch_sets | count | 23 | 35 | 21 |
| views/cascade/bins | count | 1964 | 2433 | 1745 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7129 | 8954 | 4430 |
| views/cascade/pipelines | count | 23 | 35 | 21 |
| views/cascade/work_items | count | 7129 | 8954 | 4430 |
| views/lamp_face/batch_sets | count | 0 | 173 | 0 |
| views/lamp_face/bins | count | 1228 | 2241 | 0 |
| views/lamp_face/count | count | 24 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1419 | 2609 | 0 |
| views/lamp_face/pipelines | count | 84 | 178 | 0 |
| views/lamp_face/work_items | count | 0 | 2327 | 0 |
| views/main/batch_sets | count | 34 | 34 | 34 |
| views/main/bins | count | 2072 | 3210 | 3098 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7584 | 11.3 k | 5586 |
| views/main/pipelines | count | 30 | 30 | 30 |
| views/main/work_items | count | 7731 | 11.5 k | 5894 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 28.1 k | 33.7 k | 26.9 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| assets/Image/added | count | 0.02 | 18 | 136 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.48 | 160 | 3454 |
| assets/Mesh/removed | count | 0.68 | 1242 | 4892 |
| assets/StandardMaterial/added | count | 0.04 | 25 | 263 |
| assets/StandardMaterial/modified | count | 3.01 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 12 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.12 | 1388 | 867 |
| physics/inserts | count | 0.36 | 130 | 2606 |
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
| stream/spawn/items | count | 0.05 | 24 | 378 |
| stream/upload/bytes | bytes | 4709 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1032 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 11, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 580.1 | main/busy/ms 552, surface/advance_cpu_ms 64.98, surface/cell_upload_ms 0.70, stream/drive/ms 0.30, weather/map/ms 0.11, bigspace/total/ms 0.09, stream/tick/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 1.01 | 428.6 | main/busy/ms 588, render/thread/ms 396, render/busy/ms 318, render/acquire/ms 75.66, surface/advance_cpu_ms 64.98, render/graph/ms 4.73, render/present/ms 2.24, stream/drive/ms 2.17, surface/cell_upload_ms 0.70, stream/tick/ms 0.53, terrain/update/ms 0.37, bigspace/total/ms 0.26 |
| warm-up | 1.06 | 51.7 | render/thread/ms 414, render/busy/ms 335, pipelines/wait_ms 141, render/acquire/ms 75.80, pipelines/compile_ms 53.70, main/busy/ms 53.24, pipelines/front_ms 48.96, terrain/update/ms 20.27, render/graph/ms 9.72, pipelines/driver_ms 4.75, render/present/ms 3.40, stream/drive/ms 2.37 |
| warm-up | 1.14 | 41.9 | render/thread/ms 39.02, render/busy/ms 37.25, main/busy/ms 19.90, terrain/update/ms 11.93, render/graph/ms 3.31, stream/drive/ms 2.09, stream/tick/ms 1.87, render/present/ms 1.34, terrain/select/ms 0.72, render/acquire/ms 0.43, bigspace/total/ms 0.22, bigspace/high_precision/ms 0.13 |
| warm-up | 1.29 | 84.2 | pipelines/wait_ms 389, pipelines/compile_ms 224, pipelines/front_ms 196, render/thread/ms 59.83, main/busy/ms 58.56, render/busy/ms 58.29, pipelines/driver_ms 27.82, render/graph/ms 3.33, stream/drive/ms 1.77, stream/tick/ms 1.40, render/present/ms 1.30, terrain/update/ms 0.63 |
| warm-up | 1.42 | 30.6 | pipelines/wait_ms 629, render/thread/ms 35.28, pipelines/compile_ms 35.01, render/busy/ms 33.95, pipelines/front_ms 32.22, main/busy/ms 19.74, render/graph/ms 3.75, stream/drive/ms 3.33, pipelines/driver_ms 2.79, render/present/ms 1.14, stream/spawn/ms 1.05, stream/tick/ms 0.85 |
| warm-up | 2.41 | 25.6 | render/thread/ms 31.80, render/busy/ms 30.68, main/busy/ms 20.71, render/graph/ms 4.99, stream/drive/ms 3.84, stream/spawn/ms 1.92, render/present/ms 0.94, stream/tick/ms 0.85, terrain/update/ms 0.81, terrain/select/ms 0.67, bigspace/total/ms 0.50, bigspace/high_precision/ms 0.23 |
| measure | 51.92 | 29.3 | render/thread/ms 43.62, render/busy/ms 41.29, main/busy/ms 37.14, render/graph/ms 15.71, bigspace/total/ms 2.69, render/present/ms 2.11, far/select/ms 0.74, stream/drive/ms 0.70, terrain/update/ms 0.66, stream/tick/ms 0.61, terrain/select/ms 0.58, bigspace/high_precision/ms 0.45 |
| measure | 51.97 | 25.2 | render/thread/ms 43.47, render/busy/ms 40.93, main/busy/ms 37.75, render/graph/ms 14.60, render/present/ms 1.94, bigspace/total/ms 1.28, terrain/update/ms 0.94, far/select/ms 0.86, terrain/select/ms 0.78, stream/drive/ms 0.66, render/acquire/ms 0.60, bigspace/high_precision/ms 0.58 |
| measure | 52.02 | 25.3 | main/busy/ms 49.11, render/thread/ms 46.10, render/busy/ms 42.12, render/graph/ms 16.61, render/present/ms 3.38, bigspace/total/ms 1.68, bigspace/high_precision/ms 1.12, stream/drive/ms 0.74, far/select/ms 0.72, stream/tick/ms 0.66, terrain/update/ms 0.63, render/acquire/ms 0.59 |
| measure | 52.05 | 34.2 | main/busy/ms 54.45, render/thread/ms 50.06, render/busy/ms 45.99, render/graph/ms 15.55, render/present/ms 3.23, bigspace/total/ms 2.26, bigspace/high_precision/ms 1.61, stream/drive/ms 1.21, stream/tick/ms 1.01, render/acquire/ms 0.84, far/select/ms 0.73, terrain/update/ms 0.69 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ceil_actors_off\balmora_walk@2560x1440_off=actors-1 --bench-baseline none --off actors
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 9%
- during the run: other processes 13% CPU (system minus this process)
- nvidia-smi over the window (254 samples): GPU load **90%** (min 31%) · 2580 MHz (min 2580) · memory 8001 MHz · 86.5 W · 74 °C (max 78) · VRAM 4347 MiB · throttled 100% of samples
