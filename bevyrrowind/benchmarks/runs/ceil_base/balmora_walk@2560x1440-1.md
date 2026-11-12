# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5807 frames over 65.7 s · **median 10.96 ms (91 fps)** · mean 11.31 · p95 13.66 · p99 15.68 (1% low 64 fps) · worst 42.24 ms · 40 spikes over 1.5× median · 21.9 stutters/min (work 28.3/min)
- warm-up: 431 frames over 5.8 s, worst 581.6 ms
- hitches (> 27.4 ms): **2** measured, 7 in warm-up
- timed GPU passes: 7.50 ms of a 11.31 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.96 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.93/min, limit 1/min
- **FAIL** worst frame 42.24 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.232 | 2.611 | 0.188 |
| main_opaque_pass_3d | 1.466 | 2.044 | 0.044 |
| early prepass | 0.520 | 0.889 | 0.078 |
| lightprobe_irradiance_map | 0.444 | 0.756 | 0.001 |
| bloom | 0.339 | 0.505 | 0.066 |
| atmosphere_luts | 0.328 | 0.521 | 0.003 |
| lightprobe_radiance_map | 0.228 | 0.239 | 0.004 |
| ssao | 0.217 | 0.220 | 0.007 |
| uniform_allocation | 0.184 | 0.260 | 0.017 |
| auxiliary_views | 0.163 | 0.628 | 0.144 |
| water_fft | 0.132 | 0.128 | 0.003 |
| upscaling | 0.129 | 0.159 | 0.004 |
| bin_unpacking | 0.129 | 0.141 | 0.012 |
| clouds_trace | 0.127 | 0.214 | 0.001 |
| atmos_froxel_inject | 0.098 | 0.108 | 0.001 |
| tonemapping | 0.093 | 0.094 | 0.003 |
| water_velocity | 0.087 | 0.085 | 0.002 |
| clustering | 0.067 | 0.068 | 0.019 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.058 | 0.106 | 0.108 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.044 | 0.049 | 0.005 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| clouds_shadow | 0.032 | 0.034 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.030 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.023 | 0.007 |
| lightprobe_copy | 0.013 | 0.014 | 0.003 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.650 | 11.879 | 39.98 | 56035.7 |
| render/busy/ms | 9.108 | 11.327 | 39.46 | 52891.7 |
| main/busy/ms | 8.437 | 10.195 | 19.02 | 48995.4 |
| render/graph/ms | 4.528 | 5.889 | 35.09 | 26294.0 |
| pipelines/wait_ms | 1.463 | 0.000 | 4311.25 | 8495.2 |
| bigspace/total/ms | 0.457 | 0.852 | 2.15 | 2656.1 |
| render/present/ms | 0.449 | 0.549 | 15.14 | 2605.0 |
| terrain/update/ms | 0.292 | 0.376 | 1.67 | 1697.3 |
| stream/drive/ms | 0.261 | 0.447 | 2.16 | 1516.5 |
| terrain/select/ms | 0.257 | 0.331 | 0.86 | 1491.8 |
| controller/feed/ms | 0.254 | 0.333 | 0.72 | 1475.8 |
| far/select/ms | 0.222 | 0.383 | 0.82 | 1291.0 |
| stream/tick/ms | 0.210 | 0.366 | 0.86 | 1220.9 |
| bigspace/low_precision/ms | 0.177 | 0.384 | 1.79 | 1025.0 |
| anim/evaluate/ms | 0.174 | 0.268 | 1.23 | 1011.2 |
| anim/evaluate_cpu/ms | 0.158 | 0.196 | 0.60 | 919.0 |
| controller/move/ms | 0.137 | 0.262 | 0.66 | 796.9 |
| bigspace/high_precision/ms | 0.110 | 0.218 | 1.50 | 636.1 |
| render/acquire/ms | 0.093 | 0.121 | 0.49 | 539.0 |
| pipelines/compile_ms | 0.089 | 0.000 | 165.54 | 518.2 |
| pipelines/front_ms | 0.086 | 0.000 | 159.87 | 502.2 |
| anim/apply/ms | 0.082 | 0.126 | 1.12 | 478.2 |
| physics/insert/ms | 0.051 | 0.423 | 0.94 | 294.3 |
| physics/step/ms | 0.047 | 0.081 | 0.70 | 274.2 |
| weather/map/ms | 0.027 | 0.034 | 0.37 | 154.9 |
| controller/avoid/ms | 0.011 | 0.017 | 0.20 | 66.4 |
| weather/eval/ms | 0.010 | 0.014 | 0.16 | 60.8 |
| script/update/ms | 0.007 | 0.014 | 0.09 | 42.3 |
| stream/despawn/ms | 0.006 | 0.000 | 1.43 | 34.5 |
| script/input/ms | 0.005 | 0.006 | 0.12 | 28.3 |
| script/ui/ms | 0.005 | 0.006 | 0.16 | 28.3 |
| stream/spawn/ms | 0.005 | 0.000 | 1.13 | 27.2 |
| controller/write/ms | 0.004 | 0.006 | 0.09 | 20.7 |
| pipelines/driver_ms | 0.003 | 0.000 | 5.67 | 15.9 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.71 | 15.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.5 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 5.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.26 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| ecs/entities | count | 59.2 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 8.93 | 39.46 | 8.21 |
| gpu/timed/ms | ms | 7.54 | 9.11 | 7.71 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 810 | 1420 | 618 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 810 | 1420 | 618 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1287 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3398 | 5157 | 3182 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1287 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3398 | 5157 | 3182 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 40 | 229 | 28 |
| phases/shadow/bins | count | 3817 | 5450 | 2214 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9642 | 12.9 k | 5669 |
| phases/shadow/pipelines | count | 111 | 248 | 28 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 452 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 167 | 169 | 169 |
| pipelines/waiting | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 189 | 606 | 164 |
| preprocess/bind_groups_created | count | 189 | 606 | 164 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 24.52 | 37.98 | 27.40 |
| process/mem_usage | GiB | 2.61 | 2.67 | 2.67 |
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
| started_load_count |  | 3477 | 3542 | 3542 |
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
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.93 | 57.10 | 29.44 |
| system/mem_usage | % | 49.44 | 49.78 | 49.58 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 84 | 269 | 72 |
| views/all/bins | count | 6504 | 8237 | 5694 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.4 k | 13.3 k |
| views/all/pipelines | count | 153 | 290 | 68 |
| views/all/pipelines_distinct | count | 62 | 69 | 55 |
| views/all/work_items | count | 17.3 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 28 |
| views/cascade/bins | count | 2613 | 3179 | 2214 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8158 | 10.2 k | 5669 |
| views/cascade/pipelines | count | 36 | 50 | 28 |
| views/cascade/work_items | count | 8158 | 10.2 k | 5669 |
| views/lamp_face/batch_sets | count | 0 | 190 | 0 |
| views/lamp_face/bins | count | 1088 | 2477 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1240 | 2853 | 0 |
| views/lamp_face/pipelines | count | 73 | 206 | 0 |
| views/lamp_face/work_items | count | 0 | 2531 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2882 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8407 | 13.0 k | 7600 |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 8570 | 13.3 k | 7908 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 55.82 | 249 | 324.1 k |
| anim/events/count | count | 0.27 | 4 | 1570 |
| assets/Image/added | count | 0.05 | 28 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.70 | 160 | 4086 |
| assets/Mesh/removed | count | 0.84 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.73 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.82 | 1391 | 4739 |
| physics/inserts | count | 0.47 | 89 | 2753 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 7 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 20 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 5830 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1278 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.4 ms; slowest 9, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 581.6 | main/busy/ms 560, surface/advance_cpu_ms 65.17, surface/cell_upload_ms 0.73, stream/drive/ms 0.36, weather/map/ms 0.11, stream/tick/ms 0.10, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.85 | 273.2 | main/busy/ms 595, render/thread/ms 234, render/busy/ms 155, render/acquire/ms 77.67, surface/advance_cpu_ms 65.17, render/graph/ms 6.50, render/present/ms 2.09, stream/drive/ms 1.41, surface/cell_upload_ms 0.73, stream/tick/ms 0.71, terrain/update/ms 0.57, terrain/select/ms 0.39 |
| warm-up | 1.05 | 195.7 | pipelines/wait_ms 1713, render/thread/ms 254, pipelines/compile_ms 192, pipelines/front_ms 176, render/busy/ms 173, render/acquire/ms 77.94, main/busy/ms 53.96, terrain/update/ms 20.61, pipelines/driver_ms 15.69, render/graph/ms 12.15, render/present/ms 3.23, stream/drive/ms 1.45 |
| warm-up | 1.27 | 89.5 | pipelines/compile_ms 211, pipelines/front_ms 190, main/busy/ms 67.07, render/thread/ms 52.68, render/busy/ms 50.88, pipelines/driver_ms 21.04, pipelines/wait_ms 10.24, render/graph/ms 3.25, stream/drive/ms 2.55, stream/tick/ms 1.99, render/present/ms 1.47, terrain/update/ms 0.73 |
| warm-up | 2.52 | 52.0 | main/busy/ms 51.25, render/thread/ms 49.31, render/busy/ms 48.02, render/graph/ms 8.48, stream/drive/ms 3.32, stream/spawn/ms 1.49, render/present/ms 1.10, bigspace/total/ms 0.89, stream/tick/ms 0.74, physics/insert/ms 0.73, terrain/update/ms 0.68, terrain/select/ms 0.57 |
| warm-up | 2.57 | 41.9 | pipelines/wait_ms 3410, pipelines/compile_ms 167, pipelines/front_ms 155, main/busy/ms 65.10, render/thread/ms 37.31, render/busy/ms 35.77, pipelines/driver_ms 12.10, render/graph/ms 9.53, stream/drive/ms 2.60, render/present/ms 1.32, stream/spawn/ms 1.26, anim/evaluate_cpu/ms 1.23 |
| warm-up | 2.61 | 37.6 | pipelines/wait_ms 3410, pipelines/compile_ms 167, pipelines/front_ms 155, render/thread/ms 46.70, main/busy/ms 45.35, render/busy/ms 45.24, pipelines/driver_ms 12.10, render/graph/ms 9.69, stream/drive/ms 2.86, render/present/ms 1.22, bigspace/total/ms 1.06, stream/spawn/ms 1.06 |
| measure | 0.54 | 42.2 | render/thread/ms 47.70, render/busy/ms 46.66, render/graph/ms 38.54, main/busy/ms 16.95, bigspace/total/ms 1.29, physics/insert/ms 0.99, render/present/ms 0.87, bigspace/high_precision/ms 0.78, terrain/update/ms 0.55, stream/drive/ms 0.54, terrain/select/ms 0.49, stream/tick/ms 0.47 |
| measure | 1.65 | 35.7 | render/thread/ms 42.69, render/busy/ms 41.64, render/graph/ms 33.61, main/busy/ms 17.97, bigspace/total/ms 1.51, physics/insert/ms 1.03, render/present/ms 0.88, bigspace/low_precision/ms 0.72, terrain/update/ms 0.72, terrain/select/ms 0.66, controller/feed/ms 0.56, bigspace/high_precision/ms 0.45 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ceil_base\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (254 samples): GPU load **78%** (min 61%) · 2581 MHz (min 2580) · memory 8001 MHz · 76.1 W · 70 °C (max 76) · VRAM 3795 MiB · throttled 100% of samples
