# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4629 frames over 65.7 s · **median 13.67 ms (73 fps)** · mean 14.18 · p95 17.32 · p99 19.36 (1% low 52 fps) · worst 37.25 ms · 29 spikes over 1.5× median · 15.5 stutters/min (work 12.8/min)
- warm-up: 487 frames over 10.0 s, worst 2573.2 ms
- hitches (> 34.2 ms): **2** measured, 21 in warm-up
- timed GPU passes: 8.12 ms of a 14.18 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.67 ms, limit 11 ms
- **FAIL** frame_ms: 17 stutters (> 1.5x moving median of 60) in 1.09 min = 15.54/min, limit 1/min
- **FAIL** worst frame 37.25 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.203 | 2.205 | 2.735 | 1.00 | 0.180 |
| main_opaque_pass_3d | 1.763 | 1.765 | 2.413 | 1.00 | 0.081 |
| early prepass | 0.554 | 0.554 | 1.023 | 1.00 | 0.124 |
| bloom | 0.342 | 0.342 | 0.503 | 1.00 | 0.067 |
| uniform_allocation | 0.333 | 0.333 | 0.635 | 1.00 | 0.058 |
| atmosphere_luts | 0.330 | 0.330 | 0.463 | 1.00 | 0.004 |
| water_velocity | 0.300 | 0.301 | 0.359 | 1.00 | 0.002 |
| bin_unpacking | 0.258 | 0.258 | 0.275 | 1.00 | 0.040 |
| water_sspr | 0.226 | 0.226 | 0.234 | 1.00 | 0.002 |
| ssao | 0.203 | 0.203 | 0.209 | 1.00 | 0.008 |
| auxiliary_views | 0.200 | 0.200 | 0.753 | 1.00 | 0.175 |
| atmos_froxel_inject | 0.160 | 0.161 | 0.153 | 1.00 | 0.002 |
| clouds_trace | 0.141 | 0.141 | 0.231 | 1.00 | 0.001 |
| water_fft | 0.140 | 0.140 | 0.138 | 1.00 | 0.003 |
| upscaling | 0.126 | 0.126 | 0.139 | 1.00 | 0.004 |
| clouds_composite | 0.113 | 0.113 | 0.111 | 1.00 | 0.002 |
| tonemapping | 0.093 | 0.093 | 0.092 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.092 | 0.092 | 0.171 | 1.00 | 0.009 |
| clustering | 0.067 | 0.067 | 0.069 | 1.00 | 0.020 |
| lightprobe_irradiance_map | 0.056 | 0.445 | 0.759 | 0.13 | 0.001 |
| render_sky | 0.055 | 0.055 | 0.054 | 1.00 | 0.002 |
| surface_trail | 0.051 | 0.059 | 0.057 | 0.87 | 0.002 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.044 | 0.044 | 0.048 | 1.00 | 0.006 |
| water_snapshot | 0.037 | 0.037 | 0.045 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.029 | 0.231 | 0.236 | 0.13 | 0.004 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.031 | 1.00 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.023 | 0.023 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.031 | 1.00 | 0.109 |
| atmosphere_environment | 0.012 | 0.012 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.018 | 0.016 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 14.486 | 0.000 | 9993.61 | 67057.5 |
| render/thread/ms | 12.989 | 15.967 | 35.39 | 60127.7 |
| render/busy/ms | 12.405 | 15.337 | 34.62 | 57423.4 |
| main/busy/ms | 9.607 | 12.490 | 29.03 | 44470.7 |
| render/graph/ms | 7.687 | 9.497 | 18.55 | 35581.2 |
| render/present/ms | 0.494 | 0.584 | 8.92 | 2285.7 |
| bigspace/total/ms | 0.454 | 0.709 | 4.54 | 2100.0 |
| pipelines/compile_ms | 0.390 | 0.000 | 161.24 | 1805.2 |
| pipelines/front_ms | 0.374 | 0.000 | 150.76 | 1733.3 |
| terrain/update/ms | 0.290 | 0.367 | 2.00 | 1341.9 |
| controller/feed/ms | 0.272 | 0.369 | 0.65 | 1258.6 |
| stream/drive/ms | 0.258 | 0.388 | 20.93 | 1195.6 |
| terrain/select/ms | 0.257 | 0.323 | 0.67 | 1188.8 |
| stream/tick/ms | 0.195 | 0.272 | 0.71 | 904.6 |
| far/select/ms | 0.195 | 0.275 | 0.81 | 903.0 |
| anim/evaluate/ms | 0.179 | 0.348 | 3.39 | 826.9 |
| bigspace/low_precision/ms | 0.171 | 0.288 | 3.92 | 791.2 |
| anim/evaluate_cpu/ms | 0.153 | 0.197 | 0.51 | 707.7 |
| controller/move/ms | 0.132 | 0.210 | 0.57 | 612.7 |
| bigspace/high_precision/ms | 0.108 | 0.205 | 3.59 | 499.1 |
| render/acquire/ms | 0.090 | 0.119 | 0.44 | 418.6 |
| physics/step/ms | 0.064 | 0.094 | 0.58 | 296.6 |
| physics/insert/ms | 0.051 | 0.427 | 0.84 | 235.8 |
| weather/map/ms | 0.025 | 0.033 | 0.10 | 113.8 |
| pipelines/driver_ms | 0.016 | 0.000 | 10.48 | 71.9 |
| anim/apply/ms | 0.011 | 0.020 | 0.17 | 52.7 |
| controller/avoid/ms | 0.010 | 0.013 | 0.12 | 47.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.04 | 46.4 |
| script/update/ms | 0.007 | 0.009 | 0.24 | 33.2 |
| stream/despawn/ms | 0.007 | 0.000 | 1.17 | 30.7 |
| stream/spawn/ms | 0.006 | 0.000 | 1.09 | 25.8 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 23.3 |
| script/ui/ms | 0.004 | 0.006 | 0.07 | 20.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 1.44 | 15.2 |
| controller/write/ms | 0.003 | 0.004 | 0.17 | 15.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.42 | 5.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.1 |
| weather/apply/ms | 0.000 | 0.000 | 0.01 | 1.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 10 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 35 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 42 | 12 |
| crowd/c0/joints/count | count | 1301 | 2218 | 535 |
| crowd/c0/meshes/count | count | 353 | 633 | 144 |
| crowd/c1/count | count | 146 | 154 | 148 |
| crowd/c1/joints/count | count | 5531 | 5881 | 5629 |
| crowd/c1/meshes/count | count | 1155 | 1241 | 1189 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4266 | 9951 | 6179 |
| crowd/joints_uploaded/count | count | 4250 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.2 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 11.97 | 34.62 | 11.63 |
| gpu/timed/ms | ms | 8.79 | 13.00 | 8.97 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 3081 | 3151 | 3151 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 2 | 2 | 2 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 277.8 k | 282.2 k | 282.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 809 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 809 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 36 | 41 | 36 |
| phases/opaque3d/bins | count | 1293 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3399 | 5118 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 36 | 41 | 36 |
| phases/opaque3d_prepass/bins | count | 1293 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3399 | 5118 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 72 | 309 | 62 |
| phases/shadow/bins | count | 3812 | 5426 | 2228 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9608 | 12.4 k | 5683 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 145 | 316 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 193 | 476 | 19 |
| physics/pending | count | 0 | 214 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 14 | 0 |
| preprocess/bind_groups | count | 13 | 863 | 10 |
| preprocess/bind_groups_created | count | 13 | 863 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 316 | 445 | 309 |
| process/cpu_usage | % | 22.45 | 40.45 | 17.86 |
| process/mem_usage | GiB | 5.17 | 5.34 | 5.34 |
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
| started_load_count |  | 4765 | 4859 | 4859 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.34 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.80 | 48.10 | 27.28 |
| system/mem_usage | % | 58.99 | 59.47 | 59.42 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 162 | 399 | 148 |
| views/all/bins | count | 6448 | 8260 | 5708 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.0 k | 13.3 k |
| views/all/pipelines | count | 209 | 376 | 113 |
| views/all/pipelines_distinct | count | 97 | 109 | 91 |
| views/all/work_items | count | 17.3 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 68 | 85 | 62 |
| views/cascade/bins | count | 2602 | 3188 | 2228 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8149 | 9903 | 5683 |
| views/cascade/pipelines | count | 56 | 72 | 47 |
| views/cascade/work_items | count | 8149 | 9903 | 5683 |
| views/lamp_face/batch_sets | count | 0 | 238 | 0 |
| views/lamp_face/bins | count | 1071 | 2476 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1222 | 2851 | 0 |
| views/lamp_face/pipelines | count | 87 | 245 | 0 |
| views/lamp_face/work_items | count | 0 | 2608 | 0 |
| views/main/batch_sets | count | 88 | 98 | 86 |
| views/main/bins | count | 2888 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8410 | 13.0 k | 7600 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8577 | 13.2 k | 7908 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.6 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 57.48 | 339 | 266.1 k |
| anim/events/count | count | 0.12 | 3 | 554 |
| assets/Image/added | count | 0.13 | 43 | 611 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.89 | 168 | 4126 |
| assets/Mesh/removed | count | 1.05 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 487 |
| assets/StandardMaterial/modified | count | 4.68 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.26 | 1492 | 5819 |
| physics/inserts | count | 0.53 | 87 | 2456 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.50 | 145 | 2335 |
| physics/steps | count | 0.85 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.08 | 17 | 378 |
| stream/upload/bytes | bytes | 7313 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1603 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 34.2 ms; slowest 23, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2573.2 | main/busy/ms 2533, surface/advance_cpu_ms 68.92, stream/drive/ms 1.94, stream/tick/ms 1.08, surface/cell_upload_ms 0.92, bigspace/total/ms 0.38, bigspace/high_precision/ms 0.34, weather/eval/ms 0.25, weather/map/ms 0.21, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.29 | 717.4 | main/busy/ms 2584, render/thread/ms 680, render/busy/ms 553, render/acquire/ms 125, surface/advance_cpu_ms 68.92, render/graph/ms 6.04, stream/drive/ms 2.96, render/present/ms 2.84, stream/tick/ms 1.67, surface/cell_upload_ms 0.92, terrain/update/ms 0.79, bigspace/total/ms 0.57 |
| warm-up | 3.50 | 208.6 | pipelines/wait_ms 1689, render/thread/ms 693, render/busy/ms 564, pipelines/compile_ms 227, pipelines/front_ms 210, render/acquire/ms 125, main/busy/ms 67.46, terrain/update/ms 20.32, pipelines/driver_ms 16.84, render/graph/ms 9.67, render/present/ms 3.60, stream/drive/ms 1.78 |
| warm-up | 3.73 | 82.3 | pipelines/compile_ms 227, pipelines/front_ms 217, render/thread/ms 62.65, main/busy/ms 61.32, render/busy/ms 60.57, pipelines/driver_ms 9.64, render/graph/ms 3.87, stream/drive/ms 1.81, render/present/ms 1.75, stream/tick/ms 1.41, terrain/update/ms 0.69, terrain/select/ms 0.58 |
| warm-up | 3.91 | 49.9 | render/thread/ms 56.92, render/busy/ms 54.57, main/busy/ms 44.65, stream/drive/ms 27.52, render/graph/ms 6.13, render/present/ms 2.01, stream/tick/ms 1.51, stream/spawn/ms 0.91, terrain/update/ms 0.63, terrain/select/ms 0.53, bigspace/total/ms 0.36, render/acquire/ms 0.34 |
| warm-up | 3.95 | 42.5 | pipelines/wait_ms 3189, pipelines/compile_ms 229, pipelines/front_ms 201, render/thread/ms 66.73, render/busy/ms 64.10, main/busy/ms 38.74, pipelines/driver_ms 27.53, stream/drive/ms 19.20, render/graph/ms 7.29, render/present/ms 2.28, stream/tick/ms 1.75, terrain/update/ms 0.92 |
| warm-up | 4.00 | 49.8 | pipelines/wait_ms 4080, pipelines/compile_ms 239, pipelines/front_ms 211, render/thread/ms 58.71, render/busy/ms 56.68, main/busy/ms 35.07, pipelines/driver_ms 28.16, stream/drive/ms 18.03, render/graph/ms 5.50, stream/tick/ms 1.93, render/present/ms 1.78, terrain/update/ms 1.10 |
| warm-up | 4.07 | 50.2 | render/thread/ms 49.54, render/busy/ms 47.44, main/busy/ms 29.17, render/graph/ms 6.22, stream/drive/ms 5.15, stream/spawn/ms 2.13, render/present/ms 1.84, stream/tick/ms 1.54, terrain/update/ms 1.00, terrain/select/ms 0.82, bigspace/total/ms 0.34, render/acquire/ms 0.25 |
| warm-up | 4.15 | 45.9 | pipelines/wait_ms 2589, pipelines/compile_ms 188, pipelines/front_ms 178, render/thread/ms 50.09, render/busy/ms 48.82, main/busy/ms 25.53, pipelines/driver_ms 10.51, stream/drive/ms 4.98, render/graph/ms 2.72, stream/spawn/ms 1.49, stream/tick/ms 1.22, render/present/ms 1.10 |
| warm-up | 4.21 | 51.7 | pipelines/wait_ms 4063, pipelines/compile_ms 68.38, pipelines/front_ms 63.56, main/busy/ms 43.89, render/thread/ms 30.33, render/busy/ms 28.38, stream/drive/ms 26.99, pipelines/driver_ms 4.83, render/graph/ms 4.31, render/present/ms 1.40, stream/spawn/ms 1.36, stream/tick/ms 1.18 |
| warm-up | 4.28 | 40.3 | pipelines/wait_ms 1852, render/thread/ms 50.04, render/busy/ms 47.71, pipelines/compile_ms 46.95, pipelines/front_ms 43.91, main/busy/ms 30.63, stream/drive/ms 14.89, render/graph/ms 11.38, pipelines/driver_ms 3.03, render/present/ms 2.13, stream/spawn/ms 1.76, stream/tick/ms 0.98 |
| warm-up | 4.39 | 87.3 | pipelines/wait_ms 2347, pipelines/compile_ms 84.84, pipelines/front_ms 78.63, render/thread/ms 66.61, render/busy/ms 64.81, main/busy/ms 58.99, stream/drive/ms 38.90, render/graph/ms 10.74, pipelines/driver_ms 6.21, render/present/ms 1.52, stream/spawn/ms 1.42, stream/tick/ms 1.39 |
| warm-up | 4.47 | 67.1 | pipelines/wait_ms 5827, pipelines/compile_ms 142, pipelines/front_ms 133, main/busy/ms 68.22, stream/drive/ms 49.19, render/thread/ms 30.19, render/busy/ms 28.84, pipelines/driver_ms 9.40, render/graph/ms 6.23, stream/spawn/ms 2.35, render/present/ms 1.17, terrain/update/ms 0.94 |
| warm-up | 4.54 | 48.6 | pipelines/wait_ms 5665, pipelines/compile_ms 84.30, pipelines/front_ms 77.99, main/busy/ms 47.67, render/thread/ms 35.20, stream/drive/ms 34.78, render/busy/ms 33.88, pipelines/driver_ms 6.32, render/graph/ms 5.36, stream/spawn/ms 2.10, render/present/ms 1.13, stream/tick/ms 0.97 |
| warm-up | 4.74 | 35.2 | render/thread/ms 42.81, render/busy/ms 41.04, main/busy/ms 27.76, render/graph/ms 5.90, stream/drive/ms 4.90, stream/spawn/ms 2.52, stream/tick/ms 1.45, render/present/ms 1.35, terrain/update/ms 1.08, terrain/select/ms 0.87, bigspace/total/ms 0.56, render/acquire/ms 0.42 |
| warm-up | 5.38 | 43.3 | pipelines/wait_ms 5227, pipelines/compile_ms 146, pipelines/front_ms 141, render/thread/ms 48.94, render/busy/ms 47.83, main/busy/ms 24.92, pipelines/driver_ms 5.10, render/graph/ms 4.71, stream/drive/ms 4.15, stream/spawn/ms 2.04, stream/tick/ms 1.19, terrain/update/ms 1.04 |
| warm-up | 5.48 | 37.2 | render/thread/ms 36.90, render/busy/ms 35.81, main/busy/ms 22.86, render/graph/ms 5.29, stream/drive/ms 3.63, physics/insert/ms 3.21, terrain/update/ms 2.22, stream/spawn/ms 2.01, render/present/ms 0.94, stream/tick/ms 0.79, terrain/select/ms 0.58, bigspace/total/ms 0.47 |
| warm-up | 5.54 | 49.0 | pipelines/wait_ms 5314, pipelines/compile_ms 103, pipelines/front_ms 98.24, render/thread/ms 46.43, render/busy/ms 45.01, main/busy/ms 39.62, render/graph/ms 7.97, pipelines/driver_ms 5.06, stream/drive/ms 3.93, physics/step/ms 1.81, stream/tick/ms 1.48, stream/spawn/ms 1.34 |
| warm-up | 5.59 | 51.9 | pipelines/wait_ms 5314, pipelines/compile_ms 103, pipelines/front_ms 98.24, render/thread/ms 70.50, render/busy/ms 69.14, main/busy/ms 49.13, stream/drive/ms 9.06, render/graph/ms 7.13, pipelines/driver_ms 5.06, stream/spawn/ms 1.46, stream/tick/ms 1.37, render/present/ms 1.05 |
| warm-up | 5.63 | 38.0 | pipelines/wait_ms 2016, pipelines/compile_ms 68.32, render/thread/ms 64.93, pipelines/front_ms 64.32, render/busy/ms 63.62, main/busy/ms 42.12, stream/drive/ms 8.99, render/graph/ms 7.01, pipelines/driver_ms 4.01, stream/spawn/ms 2.18, render/present/ms 1.03, terrain/update/ms 0.72 |
| warm-up | 5.73 | 35.5 | pipelines/wait_ms 1493, pipelines/compile_ms 128, pipelines/front_ms 120, render/thread/ms 41.14, render/busy/ms 39.83, main/busy/ms 25.90, pipelines/driver_ms 7.48, render/graph/ms 5.41, stream/drive/ms 3.63, stream/spawn/ms 2.06, render/present/ms 1.06, stream/tick/ms 0.95 |
| measure | 2.73 | 36.4 | render/thread/ms 49.67, render/busy/ms 48.42, render/graph/ms 26.16, main/busy/ms 20.82, render/present/ms 1.09, physics/insert/ms 0.96, bigspace/total/ms 0.87, stream/drive/ms 0.67, stream/tick/ms 0.62, terrain/update/ms 0.54, terrain/select/ms 0.49, controller/feed/ms 0.46 |
| measure | 59.06 | 37.2 | render/thread/ms 43.88, render/busy/ms 42.76, main/busy/ms 22.04, render/graph/ms 15.21, far/select/ms 0.94, render/present/ms 0.93, bigspace/total/ms 0.89, terrain/update/ms 0.77, terrain/select/ms 0.68, controller/feed/ms 0.67, stream/drive/ms 0.53, stream/tick/ms 0.46 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-off\tex-slab-off.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\slab2_off\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 14%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **74%** (min 45%) · 2580 MHz (min 2580) · memory 8001 MHz · 72.0 W · 74 °C (max 79) · VRAM 5299 MiB · throttled 100% of samples
