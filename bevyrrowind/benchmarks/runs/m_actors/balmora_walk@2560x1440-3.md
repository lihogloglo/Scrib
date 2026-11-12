# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: actors · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 7650 frames over 65.6 s · **median 8.50 ms (118 fps)** · mean 8.58 · p95 10.21 · p99 11.82 (1% low 85 fps) · worst 40.08 ms · 37 spikes over 1.5× median · 33.8 stutters/min (work 20.1/min)
- warm-up: 334 frames over 4.1 s, worst 579.1 ms
- hitches (> 25.0 ms): **1** measured, 6 in warm-up
- timed GPU passes: 6.21 ms of a 8.58 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 8.50 ms, limit 11 ms
- **FAIL** frame_ms: 37 stutters (> 1.5x moving median of 60) in 1.09 min = 33.82/min, limit 1/min
- **FAIL** worst frame 40.08 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.121 | 2.251 | 2.860 | 0.94 | 0.189 |
| main_opaque_pass_3d | 1.360 | 1.444 | 2.128 | 0.94 | 0.035 |
| early prepass | 0.442 | 0.469 | 0.832 | 0.94 | 0.065 |
| bloom | 0.325 | 0.344 | 0.495 | 0.94 | 0.066 |
| atmosphere_luts | 0.313 | 0.333 | 0.518 | 0.94 | 0.003 |
| ssao | 0.207 | 0.221 | 0.218 | 0.94 | 0.007 |
| water_fft | 0.124 | 0.132 | 0.128 | 0.94 | 0.003 |
| uniform_allocation | 0.122 | 0.129 | 0.130 | 0.94 | 0.024 |
| clouds_trace | 0.121 | 0.128 | 0.219 | 0.94 | 0.001 |
| upscaling | 0.110 | 0.117 | 0.139 | 0.94 | 0.004 |
| atmos_froxel_inject | 0.093 | 0.099 | 0.108 | 0.94 | 0.001 |
| bin_unpacking | 0.089 | 0.094 | 0.098 | 0.94 | 0.017 |
| tonemapping | 0.089 | 0.095 | 0.095 | 0.94 | 0.004 |
| water_velocity | 0.082 | 0.087 | 0.086 | 0.94 | 0.002 |
| clustering | 0.067 | 0.071 | 0.076 | 0.94 | 0.020 |
| clouds_composite | 0.061 | 0.065 | 0.065 | 0.94 | 0.002 |
| main_transparent_pass_3d | 0.057 | 0.061 | 0.105 | 0.94 | 0.104 |
| render_sky | 0.051 | 0.054 | 0.054 | 0.94 | 0.002 |
| lightprobe_irradiance_map | 0.050 | 0.447 | 0.758 | 0.11 | 0.001 |
| early_mesh_preprocessing | 0.039 | 0.041 | 0.046 | 0.94 | 0.005 |
| auxiliary_views | 0.036 | 0.039 | 0.292 | 0.94 | 0.034 |
| clouds_resolve | 0.033 | 0.035 | 0.041 | 0.94 | 0.001 |
| surface_trail | 0.033 | 0.060 | 0.058 | 0.54 | 0.002 |
| clouds_shadow | 0.030 | 0.032 | 0.035 | 0.94 | 0.003 |
| lightprobe_radiance_map | 0.027 | 0.241 | 0.241 | 0.11 | 0.004 |
| atmos_froxel_integrate | 0.024 | 0.026 | 0.032 | 0.94 | 0.001 |
| water_mips | 0.023 | 0.025 | 0.024 | 0.94 | 0.002 |
| water_snapshot | 0.023 | 0.024 | 0.024 | 0.94 | 0.001 |
| per_view_shadow_pass | 0.023 | 0.024 | 0.031 | 0.94 | 0.006 |
| clouds_fog_light | 0.012 | 0.013 | 0.013 | 0.94 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.94 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.94 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.94 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 12.888 | 0.000 | 6656.31 | 98595.6 |
| render/thread/ms | 7.583 | 9.055 | 39.18 | 58007.6 |
| main/busy/ms | 6.617 | 7.743 | 36.06 | 50618.6 |
| render/busy/ms | 6.570 | 7.999 | 38.41 | 50259.7 |
| render/graph/ms | 3.589 | 4.651 | 35.41 | 27456.1 |
| render/acquire/ms | 0.509 | 1.634 | 3.33 | 3892.5 |
| render/present/ms | 0.504 | 1.080 | 14.23 | 3855.4 |
| bigspace/total/ms | 0.324 | 0.590 | 1.78 | 2476.0 |
| terrain/update/ms | 0.290 | 0.364 | 1.90 | 2219.8 |
| terrain/select/ms | 0.258 | 0.323 | 0.80 | 1975.6 |
| stream/drive/ms | 0.237 | 0.341 | 3.20 | 1810.2 |
| stream/tick/ms | 0.196 | 0.281 | 0.65 | 1502.3 |
| far/select/ms | 0.184 | 0.257 | 0.62 | 1406.0 |
| pipelines/compile_ms | 0.172 | 0.000 | 140.01 | 1312.4 |
| pipelines/front_ms | 0.164 | 0.000 | 136.11 | 1254.0 |
| bigspace/high_precision/ms | 0.114 | 0.253 | 1.52 | 869.0 |
| bigspace/low_precision/ms | 0.098 | 0.227 | 1.31 | 750.1 |
| physics/insert/ms | 0.026 | 0.326 | 1.08 | 202.7 |
| weather/map/ms | 0.026 | 0.035 | 0.39 | 202.2 |
| physics/step/ms | 0.013 | 0.028 | 1.16 | 100.8 |
| weather/eval/ms | 0.012 | 0.015 | 0.31 | 88.0 |
| pipelines/driver_ms | 0.008 | 0.000 | 15.82 | 58.4 |
| script/update/ms | 0.007 | 0.009 | 0.28 | 52.8 |
| script/input/ms | 0.005 | 0.006 | 0.11 | 35.7 |
| script/ui/ms | 0.004 | 0.006 | 0.21 | 30.4 |
| stream/despawn/ms | 0.003 | 0.000 | 0.74 | 25.0 |
| stream/spawn/ms | 0.003 | 0.000 | 1.08 | 24.6 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 13.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.05 | 7.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.12 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 4.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
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
| frame/work/ms | ms | 7.44 | 38.41 | 7.61 |
| gpu/timed/ms | ms | 7.29 | 9.39 | 7.61 |
| mesh_allocator_allocations |  meshes | 13.0 k | 14.2 k | 13.1 k |
| mesh_allocator_slabs |  slabs | 12 | 12 | 12 |
| mesh_allocator_slabs_size |  bytes | 474.50 M | 474.50 M | 474.50 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 187 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 865 | 1419 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 187 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 865 | 1419 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 10 | 10 | 10 |
| phases/opaque3d/bins | count | 878 | 1208 | 1152 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 2807 | 4285 | 2175 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 9 | 9 | 9 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/bins | count | 878 | 1208 | 1152 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 2807 | 4285 | 2175 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 9 | 9 | 9 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 23 | 198 | 21 |
| phases/shadow/bins | count | 3090 | 4466 | 1745 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8493 | 11.4 k | 4430 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 97 | 213 | 21 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2711 | 3248 | 2329 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 1629 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 149 | 151 | 151 |
| pipelines/waiting | count | 0 | 30 | 0 |
| preprocess/bind_groups | count | 8 | 515 | 6 |
| preprocess/bind_groups_created | count | 8 | 515 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 125 | 148 | 124 |
| process/cpu_usage | % | 31.26 | 50.48 | 27.88 |
| process/mem_usage | GiB | 2.88 | 3.08 | 3.08 |
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
| started_load_count |  | 2864 | 2903 | 2903 |
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
| stream/latency/s | s | 0.02 | 0.07 | 0.02 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 345 | 437 | 261 |
| stream/oldest_request/s | s | 0 | 0.22 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 38.44 | 53.65 | 41.52 |
| system/mem_usage | % | 54.82 | 55.40 | 55.40 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 56 | 226 | 55 |
| views/all/bins | count | 5108 | 6511 | 4843 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 16.1 k | 22.1 k | 10.0 k |
| views/all/pipelines | count | 127 | 237 | 51 |
| views/all/pipelines_distinct | count | 43 | 47 | 40 |
| views/all/work_items | count | 14.9 k | 20.6 k | 10.3 k |
| views/cascade/batch_sets | count | 23 | 35 | 21 |
| views/cascade/bins | count | 1954 | 2434 | 1745 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7103 | 8953 | 4430 |
| views/cascade/pipelines | count | 23 | 35 | 21 |
| views/cascade/work_items | count | 7103 | 8953 | 4430 |
| views/lamp_face/batch_sets | count | 0 | 173 | 0 |
| views/lamp_face/bins | count | 1026 | 2241 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1197 | 2609 | 0 |
| views/lamp_face/pipelines | count | 71 | 178 | 0 |
| views/lamp_face/work_items | count | 0 | 2327 | 0 |
| views/main/batch_sets | count | 34 | 34 | 34 |
| views/main/bins | count | 2080 | 3210 | 3098 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7330 | 11.3 k | 5584 |
| views/main/pipelines | count | 30 | 30 | 30 |
| views/main/work_items | count | 7477 | 11.5 k | 5892 |
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
| assets/Mesh/added | count | 0.45 | 135 | 3454 |
| assets/Mesh/removed | count | 0.64 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.03 | 24 | 263 |
| assets/StandardMaterial/modified | count | 2.83 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.05 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 12 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.11 | 1479 | 867 |
| physics/inserts | count | 0.50 | 146 | 3830 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.31 | 161 | 2335 |
| physics/steps | count | 0.51 | 3 | 3938 |
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
| stream/spawn/items | count | 0.05 | 25 | 378 |
| stream/upload/bytes | bytes | 4425 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 970 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 7, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 579.1 | main/busy/ms 558, surface/advance_cpu_ms 67.02, surface/cell_upload_ms 0.86, stream/drive/ms 0.40, weather/map/ms 0.14, stream/tick/ms 0.10, bigspace/total/ms 0.10, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.03, far/select/ms 0.02, script/input/ms 0.01 |
| warm-up | 0.91 | 331.5 | main/busy/ms 596, render/thread/ms 293, render/busy/ms 222, render/acquire/ms 68.84, surface/advance_cpu_ms 67.02, render/graph/ms 4.80, render/present/ms 2.17, stream/drive/ms 1.29, surface/cell_upload_ms 0.86, stream/tick/ms 0.62, terrain/update/ms 0.60, terrain/select/ms 0.45 |
| warm-up | 0.98 | 73.5 | pipelines/wait_ms 511, render/thread/ms 304, render/busy/ms 231, pipelines/compile_ms 74.53, render/acquire/ms 68.99, pipelines/front_ms 67.31, main/busy/ms 54.03, terrain/update/ms 22.74, render/graph/ms 8.28, pipelines/driver_ms 7.22, render/present/ms 3.29, stream/drive/ms 1.46 |
| warm-up | 1.22 | 79.7 | main/busy/ms 60.30, render/thread/ms 50.03, render/busy/ms 48.24, render/graph/ms 3.72, stream/drive/ms 2.06, render/present/ms 1.59, stream/tick/ms 1.59, terrain/update/ms 0.81, terrain/select/ms 0.69, bigspace/total/ms 0.38, bigspace/high_precision/ms 0.21, render/acquire/ms 0.20 |
| warm-up | 1.38 | 29.5 | render/thread/ms 30.71, render/busy/ms 29.71, main/busy/ms 15.85, stream/drive/ms 3.10, render/graph/ms 2.38, stream/spawn/ms 1.37, render/present/ms 0.83, stream/tick/ms 0.79, terrain/update/ms 0.71, terrain/select/ms 0.58, bigspace/total/ms 0.21, render/acquire/ms 0.17 |
| warm-up | 2.30 | 25.5 | render/thread/ms 37.60, render/busy/ms 36.49, main/busy/ms 18.82, render/graph/ms 3.37, stream/drive/ms 3.25, stream/spawn/ms 1.63, render/present/ms 0.83, stream/tick/ms 0.81, terrain/update/ms 0.64, terrain/select/ms 0.54, bigspace/total/ms 0.52, render/acquire/ms 0.28 |
| measure | 47.51 | 40.1 | render/thread/ms 46.99, render/busy/ms 45.47, main/busy/ms 42.59, render/graph/ms 39.63, render/present/ms 1.00, terrain/update/ms 0.57, render/acquire/ms 0.52, terrain/select/ms 0.51, bigspace/total/ms 0.45, far/select/ms 0.41, stream/drive/ms 0.37, stream/tick/ms 0.32 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\m_actors\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=actors
- before the run: GPU 1%, CPU 10%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **95%** (min 57%) · 2574 MHz (min 2565) · memory 8001 MHz · 93.2 W · 79 °C (max 82) · VRAM 4385 MiB · throttled 100% of samples
