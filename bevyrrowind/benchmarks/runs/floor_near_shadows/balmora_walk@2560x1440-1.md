# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near,shadows · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 9323 frames over 65.6 s · **median 7.04 ms (142 fps)** · mean 7.04 · p95 8.03 · p99 8.47 (1% low 118 fps) · worst 10.22 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.9/min)
- warm-up: 330 frames over 3.2 s, worst 581.4 ms
- hitches (> 25.0 ms): **0** measured, 3 in warm-up
- timed GPU passes: 7.03 ms of a 7.04 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 7.04 ms, limit 11 ms
- pass frame_ms: 0 stutters (> 1.5x moving median of 60) in 1.09 min = 0.00/min, limit 1/min
- pass worst frame 10.22 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.168 | 2.472 | 0.180 |
| main_opaque_pass_3d | 1.252 | 1.818 | 0.025 |
| atmosphere_luts | 0.863 | 1.226 | 0.003 |
| lightprobe_irradiance_map | 0.427 | 0.427 | 0.001 |
| bloom | 0.328 | 0.336 | 0.059 |
| early prepass | 0.316 | 0.547 | 0.036 |
| lightprobe_radiance_map | 0.233 | 0.241 | 0.004 |
| ssao | 0.205 | 0.215 | 0.007 |
| clouds_trace | 0.151 | 0.247 | 0.001 |
| water_fft | 0.135 | 0.128 | 0.002 |
| upscaling | 0.098 | 0.118 | 0.004 |
| tonemapping | 0.090 | 0.091 | 0.003 |
| water_velocity | 0.086 | 0.086 | 0.001 |
| atmos_froxel_inject | 0.074 | 0.074 | 0.002 |
| main_transparent_pass_3d | 0.074 | 0.236 | 0.014 |
| clouds_composite | 0.065 | 0.063 | 0.002 |
| clustering | 0.064 | 0.072 | 0.020 |
| surface_trail | 0.059 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| clouds_resolve | 0.035 | 0.042 | 0.001 |
| clouds_shadow | 0.031 | 0.034 | 0.003 |
| uniform_allocation | 0.031 | 0.040 | 0.008 |
| atmos_froxel_integrate | 0.026 | 0.032 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| bin_unpacking | 0.022 | 0.024 | 0.007 |
| early_mesh_preprocessing | 0.015 | 0.015 | 0.004 |
| lightprobe_copy | 0.014 | 0.016 | 0.003 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.011 | 0.014 | 0.003 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| night_sky | 0.008 | 0.016 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| blas_build | 0.002 | 0.003 | 0.004 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.002 | 0.001 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.346 | 7.346 | 9.49 | 59159.6 |
| main/busy/ms | 4.623 | 5.281 | 11.79 | 43097.3 |
| render/busy/ms | 4.591 | 5.288 | 9.01 | 42805.1 |
| render/graph/ms | 2.747 | 3.337 | 6.95 | 25611.1 |
| render/acquire/ms | 1.314 | 2.285 | 3.39 | 12254.1 |
| render/present/ms | 0.440 | 0.538 | 2.09 | 4100.4 |
| terrain/update/ms | 0.299 | 0.385 | 1.89 | 2786.1 |
| terrain/select/ms | 0.266 | 0.344 | 0.77 | 2480.9 |
| bigspace/total/ms | 0.238 | 0.508 | 1.44 | 2216.5 |
| stream/drive/ms | 0.210 | 0.310 | 2.67 | 1955.4 |
| stream/tick/ms | 0.176 | 0.255 | 0.70 | 1641.8 |
| far/select/ms | 0.173 | 0.244 | 0.55 | 1617.1 |
| bigspace/high_precision/ms | 0.123 | 0.282 | 1.35 | 1149.3 |
| bigspace/low_precision/ms | 0.053 | 0.167 | 1.30 | 495.0 |
| weather/map/ms | 0.025 | 0.034 | 0.36 | 233.2 |
| physics/insert/ms | 0.021 | 0.167 | 1.03 | 192.9 |
| weather/eval/ms | 0.010 | 0.015 | 0.20 | 97.4 |
| physics/step/ms | 0.010 | 0.026 | 0.56 | 91.1 |
| script/update/ms | 0.007 | 0.008 | 0.19 | 64.4 |
| script/input/ms | 0.005 | 0.006 | 0.16 | 44.9 |
| script/ui/ms | 0.004 | 0.005 | 0.11 | 34.2 |
| anim/evaluate/ms | 0.002 | 0.003 | 0.17 | 19.0 |
| controller/move/ms | 0.002 | 0.003 | 0.15 | 18.7 |
| stream/deactivate/ms | 0.001 | 0.000 | 0.66 | 13.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 8.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.13 | 6.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 5.3 |
| anim/apply/ms | 0.001 | 0.001 | 0.01 | 4.9 |
| stream/spawn/ms | 0.000 | 0.000 | 0.22 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.13 | 2.9 |
| controller/feed/ms | 0.000 | 0.001 | 0.01 | 2.9 |
| stream/despawn/ms | 0.000 | 0.000 | 0.19 | 2.6 |
| controller/avoid/ms | 0.000 | 0.000 | 0.01 | 1.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.02 | 0.6 |
| controller/write/ms | 0.000 | 0.000 | 0.01 | 0.4 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
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
| frame/work/ms | ms | 7.05 | 11.79 | 7.52 |
| gpu/timed/ms | ms | 7.05 | 8.53 | 7.52 |
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
| phases/opaque3d/items | count | 326 | 618 | 603 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d_prepass/bins | count | 146 | 429 | 419 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 326 | 618 | 603 |
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
| physics/items | count | 249 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 100 | 100 | 100 |
| pipelines/waiting | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 2 | 35 | 2 |
| preprocess/bind_groups_created | count | 2 | 35 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 37 | 41 | 37 |
| process/cpu_usage | % | 22.60 | 40.19 | 19.71 |
| process/mem_usage | GiB | 2.13 | 2.16 | 2.13 |
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
| stream/activate/queue | count | 0 | 0 | 0 |
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
| system/cpu_usage | % | 28.12 | 47.32 | 26.47 |
| system/mem_usage | % | 50.85 | 50.96 | 50.81 |
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
| views/all/work_items | count | 1000 | 2172 | 2129 |
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
| views/main/work_items | count | 1000 | 2172 | 2129 |
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
| assets/Mesh/added | count | 0.22 | 195 | 2087 |
| assets/Mesh/removed | count | 0.47 | 1162 | 4426 |
| assets/StandardMaterial/added | count | 0.01 | 18 | 128 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 137 | -32 |
| physics/inserts | count | 0.26 | 111 | 2382 |
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
| stream/upload/bytes | bytes | 3631 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 16 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 796 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 3, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 581.4 | main/busy/ms 560, surface/advance_cpu_ms 64.81, surface/cell_upload_ms 0.71, stream/drive/ms 0.36, weather/map/ms 0.12, bigspace/total/ms 0.09, stream/tick/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.02, script/input/ms 0.02, far/select/ms 0.01 |
| warm-up | 0.96 | 377.5 | main/busy/ms 593, render/thread/ms 344, render/busy/ms 263, render/acquire/ms 78.47, surface/advance_cpu_ms 64.81, render/graph/ms 6.38, render/present/ms 1.87, stream/drive/ms 1.07, surface/cell_upload_ms 0.71, stream/tick/ms 0.50, terrain/update/ms 0.50, terrain/select/ms 0.35 |
| warm-up | 1.17 | 79.0 | main/busy/ms 59.68, render/thread/ms 43.04, render/busy/ms 41.62, render/graph/ms 3.72, stream/drive/ms 2.36, stream/tick/ms 1.54, render/present/ms 1.22, terrain/update/ms 0.90, terrain/select/ms 0.75, bigspace/total/ms 0.22, render/acquire/ms 0.20, bigspace/high_precision/ms 0.11 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\floor_near_shadows\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=near,shadows
- before the run: GPU 1%, CPU 5%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (254 samples): GPU load **98%** (min 96%) · 2580 MHz (min 2580) · memory 8001 MHz · 99.1 W · 80 °C (max 83) · VRAM 3723 MiB · throttled 100% of samples
