# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: actors · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 7647 frames over 65.6 s · **median 8.54 ms (117 fps)** · mean 8.58 · p95 10.11 · p99 11.46 (1% low 87 fps) · worst 25.00 ms · 26 spikes over 1.5× median · 29.3 stutters/min (work 11.9/min)
- warm-up: 338 frames over 4.1 s, worst 575.6 ms
- hitches (> 25.0 ms): **0** measured, 6 in warm-up
- timed GPU passes: 6.14 ms of a 8.58 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 8.54 ms, limit 11 ms
- **FAIL** frame_ms: 32 stutters (> 1.5x moving median of 60) in 1.09 min = 29.25/min, limit 1/min
- pass worst frame 25.00 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.083 | 2.233 | 2.732 | 0.93 | 0.187 |
| main_opaque_pass_3d | 1.350 | 1.447 | 2.129 | 0.93 | 0.038 |
| early prepass | 0.445 | 0.477 | 0.933 | 0.93 | 0.065 |
| bloom | 0.321 | 0.344 | 0.517 | 0.93 | 0.065 |
| atmosphere_luts | 0.315 | 0.337 | 0.511 | 0.93 | 0.003 |
| ssao | 0.202 | 0.220 | 0.218 | 0.92 | 0.007 |
| water_fft | 0.124 | 0.133 | 0.128 | 0.93 | 0.003 |
| uniform_allocation | 0.120 | 0.129 | 0.130 | 0.93 | 0.026 |
| clouds_trace | 0.117 | 0.127 | 0.220 | 0.92 | 0.001 |
| upscaling | 0.112 | 0.120 | 0.167 | 0.93 | 0.004 |
| atmos_froxel_inject | 0.091 | 0.099 | 0.108 | 0.92 | 0.002 |
| tonemapping | 0.087 | 0.094 | 0.099 | 0.93 | 0.004 |
| bin_unpacking | 0.087 | 0.093 | 0.097 | 0.93 | 0.019 |
| water_velocity | 0.082 | 0.088 | 0.086 | 0.93 | 0.002 |
| clustering | 0.064 | 0.069 | 0.075 | 0.93 | 0.020 |
| clouds_composite | 0.061 | 0.065 | 0.065 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.058 | 0.062 | 0.109 | 0.93 | 0.104 |
| render_sky | 0.050 | 0.054 | 0.054 | 0.93 | 0.002 |
| lightprobe_irradiance_map | 0.049 | 0.442 | 0.754 | 0.11 | 0.001 |
| early_mesh_preprocessing | 0.039 | 0.041 | 0.046 | 0.93 | 0.005 |
| auxiliary_views | 0.034 | 0.036 | 0.284 | 0.93 | 0.034 |
| surface_trail | 0.032 | 0.060 | 0.057 | 0.54 | 0.002 |
| clouds_resolve | 0.032 | 0.035 | 0.038 | 0.92 | 0.001 |
| clouds_shadow | 0.030 | 0.032 | 0.035 | 0.93 | 0.003 |
| lightprobe_radiance_map | 0.027 | 0.243 | 0.364 | 0.11 | 0.004 |
| atmos_froxel_integrate | 0.023 | 0.025 | 0.032 | 0.92 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.024 | 0.93 | 0.001 |
| water_mips | 0.022 | 0.024 | 0.024 | 0.93 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.024 | 0.031 | 0.93 | 0.006 |
| clouds_fog_light | 0.011 | 0.012 | 0.013 | 0.92 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.93 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.014 | 0.93 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.93 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.007 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 10.848 | 0.000 | 12234.55 | 82956.0 |
| render/thread/ms | 7.575 | 8.950 | 23.94 | 57923.7 |
| main/busy/ms | 6.624 | 7.698 | 10.79 | 50651.6 |
| render/busy/ms | 6.615 | 7.910 | 20.86 | 50587.8 |
| render/graph/ms | 3.600 | 4.577 | 14.16 | 27529.2 |
| render/present/ms | 0.506 | 1.091 | 16.48 | 3870.7 |
| render/acquire/ms | 0.453 | 1.490 | 2.95 | 3465.2 |
| bigspace/total/ms | 0.323 | 0.596 | 1.39 | 2471.9 |
| terrain/update/ms | 0.291 | 0.366 | 1.52 | 2227.4 |
| terrain/select/ms | 0.259 | 0.326 | 0.83 | 1981.6 |
| stream/drive/ms | 0.235 | 0.345 | 3.48 | 1795.6 |
| stream/tick/ms | 0.194 | 0.283 | 0.73 | 1481.7 |
| far/select/ms | 0.188 | 0.269 | 0.57 | 1440.5 |
| pipelines/compile_ms | 0.163 | 0.000 | 173.69 | 1249.4 |
| pipelines/front_ms | 0.157 | 0.000 | 167.00 | 1203.9 |
| bigspace/high_precision/ms | 0.113 | 0.252 | 1.04 | 861.8 |
| bigspace/low_precision/ms | 0.098 | 0.217 | 1.18 | 750.9 |
| physics/insert/ms | 0.031 | 0.354 | 1.02 | 240.3 |
| weather/map/ms | 0.027 | 0.035 | 0.35 | 203.7 |
| physics/step/ms | 0.014 | 0.029 | 1.12 | 103.6 |
| weather/eval/ms | 0.012 | 0.015 | 0.33 | 88.9 |
| script/update/ms | 0.007 | 0.009 | 0.19 | 52.9 |
| pipelines/driver_ms | 0.006 | 0.000 | 6.70 | 45.5 |
| script/input/ms | 0.005 | 0.006 | 0.32 | 37.7 |
| script/ui/ms | 0.004 | 0.005 | 0.14 | 28.5 |
| stream/despawn/ms | 0.003 | 0.000 | 0.71 | 25.6 |
| stream/spawn/ms | 0.003 | 0.000 | 1.07 | 24.6 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 12.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 7.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.12 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 4.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.7 |
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
| frame/work/ms | ms | 7.46 | 20.86 | 7.86 |
| gpu/timed/ms | ms | 7.28 | 10.99 | 7.86 |
| mesh_allocator_allocations |  meshes | 13.0 k | 14.2 k | 13.1 k |
| mesh_allocator_slabs |  slabs | 12 | 12 | 12 |
| mesh_allocator_slabs_size |  bytes | 487.26 M | 487.26 M | 487.26 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 186 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 891 | 1419 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 186 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 891 | 1419 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 10 | 10 | 10 |
| phases/opaque3d/bins | count | 878 | 1208 | 1152 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 2845 | 4287 | 2175 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 9 | 9 | 9 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/bins | count | 878 | 1208 | 1152 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 2845 | 4287 | 2175 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 9 | 9 | 9 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 23 | 198 | 21 |
| phases/shadow/bins | count | 3127 | 4465 | 1745 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8504 | 11.3 k | 4430 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 107 | 213 | 21 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2711 | 3248 | 2329 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 1661 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 148 | 150 | 150 |
| pipelines/waiting | count | 0 | 24 | 0 |
| preprocess/bind_groups | count | 8 | 515 | 2 |
| preprocess/bind_groups_created | count | 8 | 515 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 125 | 155 | 128 |
| process/cpu_usage | % | 31.08 | 45.85 | 29.81 |
| process/mem_usage | GiB | 2.56 | 2.83 | 2.83 |
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
| stream/latency/s | s | 0.03 | 0.06 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 345 | 437 | 261 |
| stream/oldest_request/s | s | 0 | 0.19 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 38.70 | 49.00 | 36.62 |
| system/mem_usage | % | 53.71 | 54.66 | 54.66 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 56 | 226 | 55 |
| views/all/bins | count | 5122 | 6506 | 4843 |
| views/all/count | count | 28 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 16.3 k | 21.8 k | 10.0 k |
| views/all/pipelines | count | 137 | 237 | 51 |
| views/all/pipelines_distinct | count | 43 | 47 | 40 |
| views/all/work_items | count | 14.9 k | 20.5 k | 10.3 k |
| views/cascade/batch_sets | count | 23 | 35 | 21 |
| views/cascade/bins | count | 1955 | 2433 | 1745 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7120 | 8899 | 4430 |
| views/cascade/pipelines | count | 23 | 35 | 21 |
| views/cascade/work_items | count | 7120 | 8899 | 4430 |
| views/lamp_face/batch_sets | count | 0 | 173 | 0 |
| views/lamp_face/bins | count | 1027 | 2241 | 0 |
| views/lamp_face/count | count | 24 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1214 | 2609 | 0 |
| views/lamp_face/pipelines | count | 84 | 178 | 0 |
| views/lamp_face/work_items | count | 0 | 2327 | 0 |
| views/main/batch_sets | count | 34 | 34 | 34 |
| views/main/bins | count | 2076 | 3210 | 3098 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7486 | 11.3 k | 5584 |
| views/main/pipelines | count | 30 | 30 | 30 |
| views/main/work_items | count | 7636 | 11.5 k | 5892 |
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
| assets/Mesh/added | count | 0.45 | 139 | 3454 |
| assets/Mesh/removed | count | 0.64 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.03 | 25 | 263 |
| assets/StandardMaterial/modified | count | 2.83 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.05 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 12 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.11 | 1494 | 867 |
| physics/inserts | count | 0.51 | 157 | 3879 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.31 | 145 | 2335 |
| physics/steps | count | 0.52 | 2 | 3939 |
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
| stream/spawn/items | count | 0.05 | 22 | 378 |
| stream/upload/bytes | bytes | 4427 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 971 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 575.6 | main/busy/ms 550, surface/advance_cpu_ms 67.23, surface/cell_upload_ms 0.71, stream/drive/ms 0.27, weather/map/ms 0.13, bigspace/total/ms 0.09, stream/tick/ms 0.07, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.90 | 324.1 | main/busy/ms 583, render/thread/ms 290, render/busy/ms 215, render/acquire/ms 73.35, surface/advance_cpu_ms 67.23, render/graph/ms 5.22, render/present/ms 2.20, stream/drive/ms 0.92, surface/cell_upload_ms 0.71, terrain/update/ms 0.60, stream/tick/ms 0.48, terrain/select/ms 0.44 |
| warm-up | 0.97 | 71.9 | pipelines/wait_ms 311, render/thread/ms 300, render/busy/ms 224, pipelines/compile_ms 74.71, render/acquire/ms 73.56, pipelines/front_ms 67.55, main/busy/ms 48.59, terrain/update/ms 18.69, render/graph/ms 8.73, pipelines/driver_ms 7.16, render/present/ms 3.25, stream/drive/ms 1.46 |
| warm-up | 1.21 | 88.0 | pipelines/wait_ms 466, pipelines/compile_ms 207, pipelines/front_ms 188, main/busy/ms 59.96, render/thread/ms 46.94, render/busy/ms 45.54, pipelines/driver_ms 18.61, render/graph/ms 4.10, stream/drive/ms 2.08, stream/tick/ms 1.73, render/present/ms 1.20, terrain/update/ms 0.82 |
| warm-up | 1.33 | 27.5 | render/thread/ms 28.95, render/busy/ms 27.72, main/busy/ms 14.32, render/graph/ms 3.67, stream/drive/ms 3.62, stream/spawn/ms 1.13, render/present/ms 1.06, stream/tick/ms 0.70, terrain/update/ms 0.63, terrain/select/ms 0.53, bigspace/total/ms 0.24, render/acquire/ms 0.17 |
| warm-up | 2.28 | 27.5 | pipelines/wait_ms 881, pipelines/compile_ms 140, pipelines/front_ms 130, render/thread/ms 31.43, render/busy/ms 30.21, main/busy/ms 19.20, pipelines/driver_ms 9.44, render/graph/ms 4.05, stream/drive/ms 3.31, stream/spawn/ms 1.65, render/present/ms 1.04, stream/tick/ms 0.76 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\m_actors\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=actors
- before the run: GPU 4%, CPU 5%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **95%** (min 54%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.4 W · 77 °C (max 81) · VRAM 4374 MiB · throttled 100% of samples
