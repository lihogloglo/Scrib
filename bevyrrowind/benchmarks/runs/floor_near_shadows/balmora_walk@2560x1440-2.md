# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near,shadows · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 9410 frames over 65.6 s · **median 6.98 ms (143 fps)** · mean 6.98 · p95 7.96 · p99 8.37 (1% low 120 fps) · worst 15.27 ms · 2 spikes over 1.5× median · 1.8 stutters/min (work 1.8/min)
- warm-up: 335 frames over 3.2 s, worst 582.1 ms
- hitches (> 25.0 ms): **0** measured, 4 in warm-up
- timed GPU passes: 6.97 ms of a 6.98 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 6.98 ms, limit 11 ms
- **FAIL** frame_ms: 2 stutters (> 1.5x moving median of 60) in 1.09 min = 1.83/min, limit 1/min
- pass worst frame 15.27 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.175 | 2.473 | 0.181 |
| main_opaque_pass_3d | 1.246 | 1.815 | 0.022 |
| atmosphere_luts | 0.801 | 1.117 | 0.003 |
| lightprobe_irradiance_map | 0.426 | 0.424 | 0.001 |
| bloom | 0.325 | 0.334 | 0.057 |
| early prepass | 0.318 | 0.554 | 0.034 |
| lightprobe_radiance_map | 0.233 | 0.242 | 0.003 |
| ssao | 0.206 | 0.212 | 0.007 |
| clouds_trace | 0.151 | 0.247 | 0.001 |
| water_fft | 0.135 | 0.129 | 0.002 |
| upscaling | 0.098 | 0.118 | 0.004 |
| tonemapping | 0.090 | 0.090 | 0.003 |
| water_velocity | 0.087 | 0.086 | 0.001 |
| atmos_froxel_inject | 0.075 | 0.076 | 0.001 |
| main_transparent_pass_3d | 0.074 | 0.232 | 0.013 |
| clouds_composite | 0.065 | 0.063 | 0.002 |
| clustering | 0.063 | 0.067 | 0.019 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| clouds_resolve | 0.035 | 0.042 | 0.001 |
| clouds_shadow | 0.031 | 0.034 | 0.003 |
| uniform_allocation | 0.031 | 0.041 | 0.007 |
| atmos_froxel_integrate | 0.026 | 0.031 | 0.001 |
| water_mips | 0.025 | 0.024 | 0.002 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| bin_unpacking | 0.022 | 0.024 | 0.006 |
| early_mesh_preprocessing | 0.015 | 0.015 | 0.003 |
| lightprobe_copy | 0.015 | 0.016 | 0.004 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.011 | 0.013 | 0.003 |
| night_sky | 0.008 | 0.016 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| blas_build | 0.002 | 0.002 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.002 | 0.001 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.289 | 7.264 | 14.54 | 59182.5 |
| main/busy/ms | 4.437 | 5.100 | 10.88 | 41751.7 |
| render/busy/ms | 4.380 | 4.991 | 13.42 | 41219.6 |
| render/graph/ms | 2.597 | 3.091 | 11.39 | 24442.1 |
| render/acquire/ms | 1.475 | 2.436 | 3.89 | 13883.9 |
| render/present/ms | 0.433 | 0.532 | 1.91 | 4078.9 |
| terrain/update/ms | 0.296 | 0.377 | 1.78 | 2784.0 |
| terrain/select/ms | 0.264 | 0.337 | 1.13 | 2483.1 |
| bigspace/total/ms | 0.224 | 0.455 | 1.20 | 2111.5 |
| stream/drive/ms | 0.212 | 0.292 | 2.25 | 1993.2 |
| stream/tick/ms | 0.179 | 0.238 | 0.57 | 1686.9 |
| far/select/ms | 0.161 | 0.212 | 0.62 | 1515.6 |
| bigspace/high_precision/ms | 0.120 | 0.270 | 0.85 | 1128.0 |
| bigspace/low_precision/ms | 0.048 | 0.157 | 1.04 | 452.8 |
| weather/map/ms | 0.024 | 0.033 | 0.20 | 230.2 |
| physics/insert/ms | 0.020 | 0.171 | 0.82 | 185.1 |
| physics/step/ms | 0.011 | 0.026 | 0.53 | 100.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.14 | 94.5 |
| script/update/ms | 0.006 | 0.008 | 0.23 | 60.5 |
| script/input/ms | 0.005 | 0.006 | 0.15 | 47.9 |
| script/ui/ms | 0.003 | 0.005 | 0.30 | 32.3 |
| anim/evaluate/ms | 0.002 | 0.002 | 0.28 | 18.8 |
| controller/move/ms | 0.002 | 0.003 | 0.12 | 18.3 |
| stream/deactivate/ms | 0.001 | 0.000 | 0.57 | 11.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 8.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.56 | 5.7 |
| anim/apply/ms | 0.001 | 0.001 | 0.03 | 5.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.17 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.10 | 2.8 |
| stream/despawn/ms | 0.000 | 0.000 | 0.18 | 2.6 |
| controller/feed/ms | 0.000 | 0.001 | 0.09 | 2.5 |
| controller/avoid/ms | 0.000 | 0.000 | 0.00 | 0.9 |
| controller/write/ms | 0.000 | 0.000 | 0.22 | 0.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.02 | 0.5 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| anim/evaluate_cpu/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 0 | 0 | 0 |
| anim/characters/count | count | 0 | 0 | 0 |
| anim/evaluated/count | count | 0 | 0 | 0 |
| controller/movers/count | count | 0 | 0 | 0 |
| ecs/entities | count | 8061 | 8414 | 7946 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 7.00 | 13.42 | 7.60 |
| gpu/timed/ms | ms | 7.00 | 8.32 | 7.60 |
| mesh_allocator_allocations |  meshes | 9048 | 9822 | 8524 |
| mesh_allocator_slabs |  slabs | 11 | 12 | 11 |
| mesh_allocator_slabs_size |  bytes | 298.98 M | 300.03 M | 298.98 M |
| phases/alpha_mask3d/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d/bins | count | 124 | 368 | 361 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 124 | 368 | 361 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/bins | count | 124 | 368 | 361 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 124 | 368 | 361 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d/bins | count | 146 | 429 | 419 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 326 | 617 | 603 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d_prepass/bins | count | 146 | 429 | 419 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 326 | 617 | 603 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 0 | 0 | 0 |
| phases/shadow/bins | count | 0 | 0 | 0 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 0 | 0 | 0 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 0 | 0 | 0 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2727 | 3248 | 2329 |
| physics/items | count | 246 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 105 | 105 | 105 |
| pipelines/waiting | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 2 | 35 | 2 |
| preprocess/bind_groups_created | count | 2 | 35 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 37 | 41 | 37 |
| process/cpu_usage | % | 21.15 | 38.94 | 17.14 |
| process/mem_usage | GiB | 2.12 | 2.15 | 2.12 |
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
| started_load_count |  | 3012 | 3046 | 3046 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 0 | 0 | 0 |
| statics/material_classes | count | 0 | 0 | 0 |
| statics/material_keys | count | 0 | 0 | 0 |
| statics/material_pipelines | count | 0 | 0 | 0 |
| statics/materials | count | 0 | 0 | 0 |
| statics/materials_bound | count | 0 | 0 | 0 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.02 | 0.04 | 0.02 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 758 | 851 | 683 |
| stream/oldest_request/s | s | 0 | 0.12 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 27.01 | 44.13 | 24.30 |
| system/mem_usage | % | 50.73 | 50.81 | 50.69 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 12 | 12 | 12 |
| views/all/bins | count | 540 | 1586 | 1560 |
| views/all/count | count | 1 | 3 | 1 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 900 | 1960 | 1928 |
| views/all/pipelines | count | 8 | 8 | 8 |
| views/all/pipelines_distinct | count | 8 | 8 | 8 |
| views/all/work_items | count | 1001 | 2171 | 2129 |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 2 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 12 | 12 | 12 |
| views/main/bins | count | 540 | 1586 | 1560 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 900 | 1960 | 1928 |
| views/main/pipelines | count | 8 | 8 | 8 |
| views/main/work_items | count | 1001 | 2171 | 2129 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 3200 | 3212 | 3203 |
| world/point_lights | count | 0 | 0 | 0 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0 | 0 | 0 |
| assets/Image/added | count | 0.01 | 18 | 125 |
| assets/Image/modified | count | 0 | 0 | 0 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.22 | 213 | 2087 |
| assets/Mesh/removed | count | 0.47 | 1162 | 4426 |
| assets/StandardMaterial/added | count | 0.01 | 18 | 128 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 137 | -32 |
| physics/inserts | count | 0.26 | 111 | 2416 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.25 | 145 | 2335 |
| physics/steps | count | 0.42 | 1 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.02 | 21 | 219 |
| stream/deactivate/items | count | 0.02 | 21 | 223 |
| stream/despawn/items | count | 0.02 | 21 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.04 | 27 | 351 |
| stream/upload/bytes | bytes | 3597 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 16 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 789 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 4, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 582.1 | main/busy/ms 559, surface/advance_cpu_ms 65.17, surface/cell_upload_ms 0.89, stream/drive/ms 0.39, stream/tick/ms 0.12, weather/map/ms 0.09, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.03, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.88 | 294.3 | main/busy/ms 593, render/thread/ms 262, render/busy/ms 195, render/acquire/ms 65.63, surface/advance_cpu_ms 65.17, render/graph/ms 6.25, render/present/ms 1.91, stream/drive/ms 1.04, surface/cell_upload_ms 0.89, terrain/update/ms 0.53, stream/tick/ms 0.48, bigspace/total/ms 0.42 |
| warm-up | 0.96 | 82.6 | pipelines/wait_ms 1155, render/thread/ms 271, render/busy/ms 203, pipelines/compile_ms 86.05, pipelines/front_ms 78.08, render/acquire/ms 65.79, main/busy/ms 48.36, terrain/update/ms 19.85, render/graph/ms 9.59, pipelines/driver_ms 7.98, render/present/ms 2.71, stream/drive/ms 1.16 |
| warm-up | 1.21 | 92.9 | main/busy/ms 66.61, render/thread/ms 48.65, render/busy/ms 47.14, render/graph/ms 3.36, stream/drive/ms 2.14, stream/tick/ms 1.54, render/present/ms 1.31, terrain/update/ms 0.89, terrain/select/ms 0.74, bigspace/total/ms 0.30, stream/spawn/ms 0.23, render/acquire/ms 0.20 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\floor_near_shadows\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=near,shadows
- before the run: GPU 0%, CPU 2%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **98%** (min 86%) · 2580 MHz (min 2580) · memory 8001 MHz · 99.4 W · 80 °C (max 83) · VRAM 3704 MiB · throttled 100% of samples
