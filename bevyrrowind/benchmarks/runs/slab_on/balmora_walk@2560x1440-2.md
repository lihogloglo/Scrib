# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4848 frames over 65.7 s · **median 13.27 ms (75 fps)** · mean 13.54 · p95 16.54 · p99 18.73 (1% low 53 fps) · worst 35.33 ms · 33 spikes over 1.5× median · 18.3 stutters/min (work 16.5/min)
- warm-up: 374 frames over 8.8 s, worst 2485.5 ms
- hitches (> 33.2 ms): **2** measured, 20 in warm-up
- timed GPU passes: 7.97 ms of a 13.54 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.27 ms, limit 11 ms
- **FAIL** frame_ms: 20 stutters (> 1.5x moving median of 60) in 1.09 min = 18.28/min, limit 1/min
- **FAIL** worst frame 35.33 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.201 | 2.211 | 2.677 | 1.00 | 0.183 |
| main_opaque_pass_3d | 1.748 | 1.757 | 2.419 | 1.00 | 0.061 |
| early prepass | 0.546 | 0.549 | 0.984 | 1.00 | 0.106 |
| bloom | 0.340 | 0.341 | 0.496 | 1.00 | 0.068 |
| atmosphere_luts | 0.330 | 0.332 | 0.510 | 1.00 | 0.003 |
| water_velocity | 0.302 | 0.304 | 0.511 | 1.00 | 0.002 |
| uniform_allocation | 0.263 | 0.264 | 0.267 | 1.00 | 0.049 |
| water_sspr | 0.222 | 0.223 | 0.226 | 1.00 | 0.002 |
| auxiliary_views | 0.209 | 0.210 | 0.728 | 1.00 | 0.179 |
| ssao | 0.204 | 0.205 | 0.209 | 1.00 | 0.008 |
| bin_unpacking | 0.199 | 0.200 | 0.212 | 1.00 | 0.031 |
| atmos_froxel_inject | 0.157 | 0.158 | 0.153 | 1.00 | 0.002 |
| clouds_trace | 0.140 | 0.141 | 0.231 | 1.00 | 0.001 |
| water_fft | 0.139 | 0.140 | 0.138 | 1.00 | 0.003 |
| upscaling | 0.129 | 0.129 | 0.157 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.112 | 0.111 | 1.00 | 0.002 |
| tonemapping | 0.093 | 0.093 | 0.093 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.091 | 0.092 | 0.170 | 1.00 | 0.008 |
| clustering | 0.068 | 0.068 | 0.069 | 1.00 | 0.020 |
| render_sky | 0.054 | 0.054 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.054 | 0.443 | 0.755 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| surface_trail | 0.049 | 0.058 | 0.057 | 0.84 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.043 | 0.048 | 1.00 | 0.005 |
| water_snapshot | 0.038 | 0.039 | 0.046 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.030 | 0.247 | 0.559 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.030 | 1.00 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.017 | 0.017 | 0.033 | 1.00 | 0.110 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.001 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 34.585 | 0.000 | 15151.10 | 167670.2 |
| render/thread/ms | 12.330 | 15.177 | 34.05 | 59776.7 |
| render/busy/ms | 11.742 | 14.558 | 33.22 | 56927.6 |
| main/busy/ms | 9.348 | 12.200 | 28.69 | 45316.7 |
| render/graph/ms | 7.313 | 9.074 | 17.36 | 35454.7 |
| pipelines/compile_ms | 0.661 | 0.000 | 155.14 | 3205.4 |
| pipelines/front_ms | 0.638 | 0.000 | 149.77 | 3091.0 |
| render/present/ms | 0.493 | 0.594 | 12.15 | 2391.0 |
| bigspace/total/ms | 0.468 | 0.734 | 4.76 | 2269.8 |
| terrain/update/ms | 0.294 | 0.376 | 1.86 | 1427.7 |
| controller/feed/ms | 0.274 | 0.375 | 0.85 | 1326.4 |
| terrain/select/ms | 0.261 | 0.334 | 0.72 | 1265.7 |
| stream/drive/ms | 0.256 | 0.381 | 20.78 | 1243.4 |
| stream/tick/ms | 0.194 | 0.280 | 0.81 | 941.2 |
| far/select/ms | 0.194 | 0.272 | 0.57 | 939.7 |
| bigspace/low_precision/ms | 0.187 | 0.306 | 4.23 | 906.6 |
| anim/evaluate_cpu/ms | 0.175 | 0.221 | 0.50 | 846.5 |
| controller/move/ms | 0.132 | 0.245 | 0.68 | 638.1 |
| anim/evaluate/ms | 0.127 | 0.261 | 3.44 | 613.6 |
| bigspace/high_precision/ms | 0.115 | 0.211 | 3.93 | 558.0 |
| render/acquire/ms | 0.094 | 0.127 | 0.47 | 458.1 |
| physics/step/ms | 0.060 | 0.093 | 0.80 | 289.4 |
| physics/insert/ms | 0.056 | 0.447 | 0.95 | 272.7 |
| weather/map/ms | 0.024 | 0.033 | 0.22 | 116.9 |
| pipelines/driver_ms | 0.024 | 0.000 | 5.37 | 114.4 |
| anim/apply/ms | 0.012 | 0.024 | 0.06 | 59.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.08 | 52.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.22 | 51.6 |
| script/update/ms | 0.007 | 0.010 | 0.10 | 35.5 |
| stream/despawn/ms | 0.007 | 0.000 | 1.16 | 32.2 |
| script/input/ms | 0.005 | 0.007 | 0.17 | 25.4 |
| script/ui/ms | 0.005 | 0.007 | 0.04 | 24.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 24.8 |
| controller/write/ms | 0.003 | 0.004 | 0.03 | 16.9 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.72 | 15.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 1.5 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 3 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 19 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 15 |
| crowd/c0/joints/count | count | 1249 | 2262 | 707 |
| crowd/c0/meshes/count | count | 343 | 646 | 189 |
| crowd/c1/count | count | 146 | 156 | 146 |
| crowd/c1/joints/count | count | 5507 | 5991 | 5489 |
| crowd/c1/meshes/count | count | 1157 | 1295 | 1152 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4272 | 17.7 k | 6179 |
| crowd/joints_uploaded/count | count | 4253 | 6643 | 6179 |
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
| frame/work/ms | ms | 11.54 | 33.22 | 10.49 |
| gpu/timed/ms | ms | 8.69 | 12.71 | 8.42 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 804 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 804 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d/bins | count | 1291 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3384 | 5174 | 3178 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1291 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3384 | 5174 | 3178 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 60 | 323 | 50 |
| phases/shadow/bins | count | 3735 | 5610 | 2266 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9432 | 13.1 k | 5717 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 145 | 329 | 50 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2347 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1061 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 35 | 0 |
| preprocess/bind_groups | count | 12 | 839 | 6 |
| preprocess/bind_groups_created | count | 12 | 839 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 259 | 362 | 257 |
| process/cpu_usage | % | 23.56 | 41.35 | 18.75 |
| process/mem_usage | GiB | 5.16 | 5.33 | 5.33 |
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
| started_load_count |  | 4769 | 4862 | 4862 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.04 | 59.90 | 27.45 |
| system/mem_usage | % | 59.02 | 64.20 | 64.19 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2590 | 2649 | 2649 |
| views/all/batch_sets | count | 128 | 387 | 120 |
| views/all/bins | count | 6308 | 8524 | 5746 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.0 k | 25.7 k | 13.3 k |
| views/all/pipelines | count | 209 | 395 | 116 |
| views/all/pipelines_distinct | count | 98 | 110 | 91 |
| views/all/work_items | count | 17.1 k | 24.1 k | 13.6 k |
| views/cascade/batch_sets | count | 56 | 73 | 50 |
| views/cascade/bins | count | 2611 | 3183 | 2266 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8119 | 10.3 k | 5717 |
| views/cascade/pipelines | count | 56 | 73 | 50 |
| views/cascade/work_items | count | 8119 | 10.3 k | 5717 |
| views/lamp_face/batch_sets | count | 0 | 262 | 0 |
| views/lamp_face/bins | count | 1074 | 2528 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1200 | 2859 | 0 |
| views/lamp_face/pipelines | count | 87 | 262 | 0 |
| views/lamp_face/work_items | count | 0 | 2731 | 0 |
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 2880 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8372 | 13.1 k | 7590 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8538 | 13.3 k | 7898 |
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
| anim/bones_written/count | count | 65.25 | 343 | 316.4 k |
| anim/events/count | count | 0.11 | 3 | 550 |
| assets/Image/added | count | 0.14 | 43 | 665 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.89 | 152 | 4332 |
| assets/Mesh/removed | count | 1.01 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 542 |
| assets/StandardMaterial/modified | count | 4.47 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.81 | 1604 | 8775 |
| physics/inserts | count | 0.66 | 127 | 3208 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.48 | 129 | 2335 |
| physics/steps | count | 0.81 | 2 | 3940 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.01 | 6 | 26 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 22 | 378 |
| stream/upload/bytes | bytes | 6983 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1531 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 33.2 ms; slowest 22, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2485.5 | main/busy/ms 2460, surface/advance_cpu_ms 72.60, surface/cell_upload_ms 0.95, stream/drive/ms 0.27, weather/map/ms 0.12, bigspace/total/ms 0.08, stream/tick/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.05, script/update/ms 0.03, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.21 | 720.1 | main/busy/ms 2493, render/thread/ms 685, render/busy/ms 523, render/acquire/ms 160, surface/advance_cpu_ms 72.60, render/graph/ms 5.42, render/present/ms 2.05, stream/drive/ms 1.12, surface/cell_upload_ms 0.95, terrain/update/ms 0.63, stream/tick/ms 0.58, terrain/select/ms 0.43 |
| warm-up | 3.43 | 224.2 | pipelines/wait_ms 1413, render/thread/ms 697, render/busy/ms 534, pipelines/compile_ms 237, pipelines/front_ms 223, render/acquire/ms 160, main/busy/ms 51.47, terrain/update/ms 17.57, pipelines/driver_ms 13.67, render/graph/ms 9.11, render/present/ms 3.26, stream/drive/ms 1.64 |
| warm-up | 3.67 | 110.1 | main/busy/ms 73.59, render/thread/ms 61.41, render/busy/ms 59.33, render/graph/ms 4.81, stream/drive/ms 2.62, stream/tick/ms 2.06, render/present/ms 1.61, terrain/update/ms 0.79, terrain/select/ms 0.65, render/acquire/ms 0.46, bigspace/total/ms 0.43, bigspace/high_precision/ms 0.22 |
| warm-up | 3.73 | 36.1 | pipelines/compile_ms 313, pipelines/front_ms 294, pipelines/wait_ms 244, render/thread/ms 53.16, render/busy/ms 51.55, main/busy/ms 22.47, pipelines/driver_ms 19.06, render/graph/ms 3.30, stream/drive/ms 3.29, render/present/ms 1.34, stream/spawn/ms 1.30, stream/tick/ms 1.26 |
| warm-up | 3.87 | 74.4 | render/thread/ms 54.86, render/busy/ms 53.45, main/busy/ms 44.34, stream/drive/ms 32.84, render/graph/ms 3.02, render/present/ms 1.23, stream/tick/ms 0.98, terrain/update/ms 0.73, terrain/select/ms 0.59, stream/spawn/ms 0.35, bigspace/total/ms 0.22, render/acquire/ms 0.17 |
| warm-up | 4.02 | 104.9 | pipelines/wait_ms 4192, pipelines/compile_ms 130, pipelines/front_ms 116, render/thread/ms 78.46, render/busy/ms 68.87, main/busy/ms 54.26, stream/drive/ms 27.50, pipelines/driver_ms 14.91, render/present/ms 9.26, render/graph/ms 7.21, stream/spawn/ms 2.39, stream/tick/ms 1.85 |
| warm-up | 4.06 | 37.3 | pipelines/wait_ms 3832, pipelines/compile_ms 128, pipelines/front_ms 110, render/thread/ms 94.02, render/busy/ms 85.06, main/busy/ms 52.58, stream/drive/ms 30.27, pipelines/driver_ms 18.02, render/present/ms 8.67, render/graph/ms 7.72, terrain/update/ms 1.81, stream/spawn/ms 1.61 |
| warm-up | 4.15 | 95.8 | pipelines/wait_ms 2404, pipelines/compile_ms 153, pipelines/front_ms 139, render/thread/ms 95.00, render/busy/ms 93.50, main/busy/ms 45.52, stream/drive/ms 29.30, pipelines/driver_ms 14.66, render/graph/ms 7.95, terrain/update/ms 1.77, stream/tick/ms 1.76, terrain/select/ms 1.55 |
| warm-up | 4.38 | 59.2 | pipelines/wait_ms 2457, pipelines/compile_ms 92.77, pipelines/front_ms 89.27, main/busy/ms 60.25, stream/drive/ms 46.75, render/thread/ms 36.27, render/busy/ms 35.19, render/graph/ms 3.60, pipelines/driver_ms 3.50, stream/spawn/ms 1.23, stream/tick/ms 0.93, render/present/ms 0.90 |
| warm-up | 4.46 | 72.6 | pipelines/wait_ms 4292, pipelines/compile_ms 147, pipelines/front_ms 136, main/busy/ms 66.88, stream/drive/ms 50.11, render/thread/ms 33.56, render/busy/ms 32.18, pipelines/driver_ms 11.31, render/graph/ms 4.82, stream/spawn/ms 2.88, render/present/ms 1.20, stream/tick/ms 1.02 |
| warm-up | 4.52 | 42.0 | pipelines/wait_ms 3106, pipelines/compile_ms 78.25, pipelines/front_ms 72.79, main/busy/ms 50.17, stream/drive/ms 32.88, render/thread/ms 30.85, render/busy/ms 29.44, render/graph/ms 5.64, pipelines/driver_ms 5.45, stream/spawn/ms 1.74, render/present/ms 1.23, stream/activate/ms 1.04 |
| warm-up | 4.57 | 34.6 | pipelines/wait_ms 4088, pipelines/compile_ms 51.49, pipelines/front_ms 48.71, main/busy/ms 38.12, stream/drive/ms 24.52, render/thread/ms 21.61, render/busy/ms 20.25, render/graph/ms 4.75, pipelines/driver_ms 2.79, stream/tick/ms 1.36, render/present/ms 1.14, stream/activate/ms 1.05 |
| warm-up | 5.36 | 33.3 | render/thread/ms 40.85, render/busy/ms 39.09, main/busy/ms 24.66, render/graph/ms 8.45, stream/drive/ms 4.63, stream/spawn/ms 2.09, render/present/ms 1.56, stream/tick/ms 1.39, terrain/update/ms 0.74, bigspace/total/ms 0.67, terrain/select/ms 0.63, bigspace/high_precision/ms 0.30 |
| warm-up | 5.42 | 33.5 | pipelines/wait_ms 5109, pipelines/compile_ms 227, pipelines/front_ms 216, render/thread/ms 48.57, render/busy/ms 47.32, main/busy/ms 18.03, pipelines/driver_ms 10.77, render/graph/ms 5.80, stream/drive/ms 2.60, render/present/ms 1.03, stream/spawn/ms 0.94, stream/tick/ms 0.70 |
| warm-up | 5.56 | 35.1 | render/thread/ms 40.60, render/busy/ms 38.88, main/busy/ms 27.97, stream/drive/ms 9.32, render/graph/ms 7.55, stream/spawn/ms 2.19, render/present/ms 1.40, stream/tick/ms 0.67, terrain/update/ms 0.67, bigspace/total/ms 0.59, terrain/select/ms 0.56, render/acquire/ms 0.31 |
| warm-up | 5.67 | 41.1 | pipelines/wait_ms 2157, main/busy/ms 49.72, pipelines/compile_ms 45.62, pipelines/front_ms 44.51, render/thread/ms 39.88, render/busy/ms 37.79, render/graph/ms 10.99, stream/drive/ms 6.45, stream/spawn/ms 2.82, render/present/ms 1.73, bigspace/total/ms 1.53, physics/insert/ms 1.43 |
| warm-up | 7.56 | 33.8 | main/busy/ms 46.46, render/thread/ms 39.94, render/busy/ms 38.57, render/graph/ms 14.50, stream/drive/ms 1.58, physics/insert/ms 1.31, bigspace/total/ms 1.12, render/present/ms 1.11, stream/tick/ms 0.82, anim/evaluate_cpu/ms 0.67, terrain/update/ms 0.62, far/select/ms 0.56 |
| warm-up | 7.60 | 34.5 | main/busy/ms 54.84, render/thread/ms 47.91, render/busy/ms 46.50, render/graph/ms 12.41, stream/drive/ms 1.75, render/present/ms 1.10, physics/insert/ms 1.04, bigspace/total/ms 1.03, stream/tick/ms 0.90, anim/evaluate_cpu/ms 0.84, terrain/update/ms 0.61, terrain/select/ms 0.53 |
| warm-up | 7.64 | 38.6 | render/thread/ms 48.27, render/busy/ms 46.94, main/busy/ms 41.40, render/graph/ms 11.49, stream/drive/ms 1.22, physics/insert/ms 1.10, render/present/ms 1.05, bigspace/total/ms 0.88, terrain/update/ms 0.55, stream/tick/ms 0.54, anim/evaluate_cpu/ms 0.54, terrain/select/ms 0.48 |
| measure | 2.73 | 35.3 | render/thread/ms 47.17, render/busy/ms 45.77, render/graph/ms 24.64, main/busy/ms 18.83, render/present/ms 1.19, physics/insert/ms 1.09, bigspace/total/ms 0.73, terrain/update/ms 0.57, terrain/select/ms 0.51, controller/feed/ms 0.40, stream/drive/ms 0.40, stream/tick/ms 0.35 |
| measure | 59.05 | 33.5 | render/thread/ms 44.81, render/busy/ms 43.53, main/busy/ms 24.61, render/graph/ms 12.97, bigspace/total/ms 1.18, render/present/ms 0.98, controller/feed/ms 0.94, terrain/update/ms 0.90, terrain/select/ms 0.80, stream/drive/ms 0.78, anim/evaluate_cpu/ms 0.70, stream/tick/ms 0.67 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-on\tex-slab-on.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\slab_on\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **75%** (min 55%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.0 W · 75 °C (max 81) · VRAM 5283 MiB · throttled 100% of samples
