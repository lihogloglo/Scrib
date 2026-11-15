# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4928 frames over 65.6 s · **median 12.70 ms (79 fps)** · mean 13.32 · p95 16.63 · p99 19.34 (1% low 52 fps) · worst 35.67 ms · 55 spikes over 1.5× median · 22.9 stutters/min (work 21.9/min)
- warm-up: 482 frames over 11.3 s, worst 3249.5 ms
- hitches (> 31.7 ms): **3** measured, 20 in warm-up
- timed GPU passes: 7.84 ms of a 13.32 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.70 ms, limit 11 ms
- **FAIL** frame_ms: 25 stutters (> 1.5x moving median of 60) in 1.09 min = 22.85/min, limit 1/min
- **FAIL** worst frame 35.67 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.174 | 2.185 | 2.501 | 1.00 | 0.180 |
| main_opaque_pass_3d | 1.723 | 1.731 | 2.365 | 1.00 | 0.070 |
| early prepass | 0.541 | 0.544 | 0.820 | 1.00 | 0.107 |
| bloom | 0.339 | 0.341 | 0.657 | 1.00 | 0.066 |
| atmosphere_luts | 0.330 | 0.332 | 0.626 | 1.00 | 0.004 |
| water_velocity | 0.294 | 0.296 | 0.383 | 1.00 | 0.002 |
| uniform_allocation | 0.263 | 0.264 | 0.269 | 1.00 | 0.052 |
| water_sspr | 0.219 | 0.220 | 0.221 | 1.00 | 0.002 |
| ssao | 0.203 | 0.204 | 0.210 | 1.00 | 0.008 |
| bin_unpacking | 0.198 | 0.199 | 0.211 | 1.00 | 0.033 |
| auxiliary_views | 0.158 | 0.159 | 0.655 | 1.00 | 0.149 |
| atmos_froxel_inject | 0.154 | 0.155 | 0.152 | 1.00 | 0.001 |
| clouds_trace | 0.140 | 0.141 | 0.233 | 1.00 | 0.001 |
| water_fft | 0.139 | 0.140 | 0.138 | 1.00 | 0.003 |
| upscaling | 0.127 | 0.128 | 0.153 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.111 | 1.00 | 0.002 |
| tonemapping | 0.092 | 0.092 | 0.093 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.090 | 0.091 | 0.169 | 1.00 | 0.009 |
| clustering | 0.067 | 0.067 | 0.069 | 1.00 | 0.020 |
| render_sky | 0.054 | 0.054 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.054 | 0.441 | 0.755 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| surface_trail | 0.047 | 0.057 | 0.057 | 0.83 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.043 | 0.048 | 1.00 | 0.006 |
| water_snapshot | 0.037 | 0.037 | 0.045 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.030 | 0.244 | 0.552 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.027 | 0.030 | 1.00 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.023 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.033 | 1.00 | 0.110 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 12.066 | 15.240 | 34.32 | 59462.5 |
| render/busy/ms | 11.473 | 14.591 | 33.31 | 56536.7 |
| main/busy/ms | 9.278 | 12.040 | 28.31 | 45723.7 |
| render/graph/ms | 7.018 | 9.097 | 17.68 | 34582.9 |
| pipelines/wait_ms | 5.622 | 0.000 | 8592.82 | 27704.7 |
| render/present/ms | 0.500 | 0.590 | 12.20 | 2466.2 |
| bigspace/total/ms | 0.472 | 0.766 | 5.28 | 2324.6 |
| pipelines/compile_ms | 0.309 | 0.000 | 159.42 | 1522.9 |
| pipelines/front_ms | 0.297 | 0.000 | 152.30 | 1464.2 |
| terrain/update/ms | 0.294 | 0.375 | 1.81 | 1446.8 |
| controller/feed/ms | 0.272 | 0.361 | 0.94 | 1338.4 |
| terrain/select/ms | 0.261 | 0.331 | 0.75 | 1284.1 |
| stream/drive/ms | 0.260 | 0.368 | 19.57 | 1279.2 |
| far/select/ms | 0.198 | 0.278 | 0.72 | 976.2 |
| stream/tick/ms | 0.197 | 0.276 | 0.60 | 969.1 |
| bigspace/low_precision/ms | 0.192 | 0.339 | 4.92 | 948.6 |
| anim/evaluate_cpu/ms | 0.163 | 0.214 | 0.55 | 802.4 |
| anim/evaluate/ms | 0.144 | 0.294 | 2.53 | 707.3 |
| controller/move/ms | 0.130 | 0.271 | 0.73 | 640.5 |
| bigspace/high_precision/ms | 0.112 | 0.227 | 3.10 | 552.0 |
| render/acquire/ms | 0.093 | 0.123 | 0.42 | 459.6 |
| physics/step/ms | 0.059 | 0.091 | 0.42 | 290.6 |
| physics/insert/ms | 0.041 | 0.409 | 0.95 | 201.9 |
| weather/map/ms | 0.024 | 0.033 | 0.08 | 119.0 |
| anim/apply/ms | 0.012 | 0.022 | 0.14 | 59.2 |
| pipelines/driver_ms | 0.012 | 0.000 | 7.12 | 58.7 |
| weather/eval/ms | 0.010 | 0.014 | 0.05 | 51.4 |
| controller/avoid/ms | 0.010 | 0.013 | 0.20 | 51.0 |
| script/update/ms | 0.007 | 0.009 | 0.07 | 35.7 |
| stream/despawn/ms | 0.006 | 0.000 | 1.82 | 31.9 |
| script/input/ms | 0.005 | 0.007 | 0.15 | 26.7 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 25.2 |
| script/ui/ms | 0.005 | 0.006 | 0.03 | 22.8 |
| controller/write/ms | 0.004 | 0.005 | 0.31 | 18.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 1.36 | 15.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1249 | 2262 | 535 |
| crowd/c0/meshes/count | count | 349 | 646 | 144 |
| crowd/c1/count | count | 146 | 156 | 149 |
| crowd/c1/joints/count | count | 5531 | 5991 | 5661 |
| crowd/c1/meshes/count | count | 1173 | 1295 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 9879 | 6179 |
| crowd/joints_uploaded/count | count | 4246 | 6643 | 6179 |
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
| frame/work/ms | ms | 10.96 | 33.31 | 9.72 |
| gpu/timed/ms | ms | 8.56 | 12.58 | 9.68 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 803 | 1416 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 803 | 1416 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d/bins | count | 1294 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3382 | 5124 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1294 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3382 | 5124 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 58 | 306 | 47 |
| phases/shadow/bins | count | 3775 | 5420 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9556 | 12.4 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 139 | 319 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 220 | 226 | 226 |
| pipelines/waiting | count | 0 | 6 | 0 |
| preprocess/bind_groups | count | 12 | 805 | 6 |
| preprocess/bind_groups_created | count | 12 | 805 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 259 | 384 | 251 |
| process/cpu_usage | % | 23.08 | 39.90 | 21.43 |
| process/mem_usage | GiB | 5.14 | 5.31 | 5.31 |
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
| started_load_count |  | 4766 | 4859 | 4859 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.54 | 66.38 | 29.58 |
| system/mem_usage | % | 64.13 | 64.62 | 64.57 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 126 | 370 | 117 |
| views/all/bins | count | 6484 | 8249 | 5692 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.0 k | 25.1 k | 13.3 k |
| views/all/pipelines | count | 203 | 379 | 113 |
| views/all/pipelines_distinct | count | 96 | 110 | 91 |
| views/all/work_items | count | 17.1 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 56 | 68 | 47 |
| views/cascade/bins | count | 2602 | 3187 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8147 | 10.0 k | 5663 |
| views/cascade/pipelines | count | 56 | 68 | 47 |
| views/cascade/work_items | count | 8147 | 10.0 k | 5663 |
| views/lamp_face/batch_sets | count | 0 | 245 | 0 |
| views/lamp_face/bins | count | 1061 | 2489 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1208 | 2863 | 0 |
| views/lamp_face/pipelines | count | 82 | 257 | 0 |
| views/lamp_face/work_items | count | 0 | 2607 | 0 |
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 2886 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8376 | 13.0 k | 7594 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8545 | 13.2 k | 7902 |
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
| anim/bones_written/count | count | 59.98 | 267 | 295.6 k |
| anim/events/count | count | 0.11 | 3 | 549 |
| assets/Image/added | count | 0.12 | 28 | 595 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.83 | 152 | 4104 |
| assets/Mesh/removed | count | 0.99 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 467 |
| assets/StandardMaterial/modified | count | 4.40 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.11 | 1492 | 5463 |
| physics/inserts | count | 0.47 | 110 | 2309 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.47 | 145 | 2335 |
| physics/steps | count | 0.80 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.08 | 21 | 378 |
| stream/upload/bytes | bytes | 6869 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1506 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.7 ms; slowest 23, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 3.25 | 3249.5 | main/busy/ms 3159, surface/advance_cpu_ms 66.55, surface/cell_upload_ms 0.88, stream/drive/ms 0.29, weather/map/ms 0.14, stream/tick/ms 0.09, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 4.19 | 941.3 | main/busy/ms 3229, render/thread/ms 900, render/busy/ms 762, render/acquire/ms 136, surface/advance_cpu_ms 66.55, render/graph/ms 9.02, stream/drive/ms 3.58, render/present/ms 2.50, terrain/update/ms 1.81, stream/tick/ms 1.42, surface/cell_upload_ms 0.88, terrain/select/ms 0.55 |
| warm-up | 4.25 | 63.1 | render/thread/ms 913, render/busy/ms 772, pipelines/wait_ms 456, render/acquire/ms 136, main/busy/ms 89.10, pipelines/compile_ms 74.45, pipelines/front_ms 67.97, terrain/update/ms 16.98, render/graph/ms 11.87, pipelines/driver_ms 6.48, render/present/ms 3.97, stream/drive/ms 3.96 |
| warm-up | 4.51 | 89.6 | pipelines/compile_ms 230, pipelines/front_ms 217, main/busy/ms 66.06, render/thread/ms 48.60, render/busy/ms 47.03, pipelines/driver_ms 12.65, render/graph/ms 3.85, stream/drive/ms 3.32, stream/tick/ms 1.61, render/present/ms 1.26, stream/spawn/ms 0.99, terrain/update/ms 0.68 |
| warm-up | 4.65 | 50.7 | main/busy/ms 43.45, render/thread/ms 39.35, render/busy/ms 38.09, stream/drive/ms 31.94, terrain/update/ms 6.41, render/graph/ms 2.57, render/present/ms 0.98, stream/tick/ms 0.87, physics/insert/ms 0.60, terrain/select/ms 0.56, stream/spawn/ms 0.41, stream/activate/ms 0.38 |
| warm-up | 4.70 | 36.8 | pipelines/wait_ms 1501, pipelines/compile_ms 202, pipelines/front_ms 179, render/thread/ms 29.77, main/busy/ms 29.17, render/busy/ms 28.13, pipelines/driver_ms 23.24, stream/drive/ms 15.15, render/graph/ms 4.76, render/present/ms 1.41, stream/spawn/ms 1.37, stream/tick/ms 1.22 |
| warm-up | 4.76 | 37.4 | pipelines/wait_ms 867, render/thread/ms 47.01, render/busy/ms 44.81, main/busy/ms 42.07, pipelines/compile_ms 18.86, pipelines/front_ms 17.01, stream/drive/ms 14.76, render/graph/ms 6.93, stream/spawn/ms 2.28, render/present/ms 1.94, pipelines/driver_ms 1.85, stream/tick/ms 1.08 |
| warm-up | 4.83 | 63.9 | render/thread/ms 69.89, render/busy/ms 68.27, main/busy/ms 41.83, stream/drive/ms 14.63, render/graph/ms 5.14, stream/spawn/ms 2.06, render/present/ms 1.38, stream/tick/ms 0.83, terrain/update/ms 0.63, terrain/select/ms 0.52, bigspace/total/ms 0.36, stream/activate/ms 0.33 |
| warm-up | 4.87 | 45.7 | pipelines/wait_ms 2550, pipelines/compile_ms 167, pipelines/front_ms 159, render/thread/ms 71.35, render/busy/ms 69.93, main/busy/ms 45.84, stream/drive/ms 27.77, pipelines/driver_ms 8.12, render/graph/ms 2.56, render/present/ms 1.23, terrain/update/ms 0.89, stream/spawn/ms 0.85 |
| warm-up | 4.93 | 52.3 | pipelines/wait_ms 2550, pipelines/compile_ms 167, pipelines/front_ms 159, render/thread/ms 68.23, render/busy/ms 66.54, main/busy/ms 43.45, stream/drive/ms 27.79, render/graph/ms 26.58, pipelines/driver_ms 8.12, render/present/ms 1.49, stream/tick/ms 0.97, terrain/update/ms 0.90 |
| warm-up | 5.08 | 38.7 | pipelines/wait_ms 2265, main/busy/ms 69.59, stream/drive/ms 51.45, render/thread/ms 39.78, render/busy/ms 38.09, pipelines/compile_ms 25.73, pipelines/front_ms 23.23, render/graph/ms 4.16, pipelines/driver_ms 2.50, stream/tick/ms 1.81, stream/spawn/ms 1.25, render/present/ms 1.19 |
| warm-up | 5.16 | 80.0 | pipelines/wait_ms 758, main/busy/ms 70.45, render/thread/ms 52.99, stream/drive/ms 52.05, render/busy/ms 51.77, pipelines/compile_ms 17.49, pipelines/front_ms 15.89, render/graph/ms 3.17, stream/spawn/ms 2.10, pipelines/driver_ms 1.60, stream/tick/ms 1.29, terrain/update/ms 1.10 |
| warm-up | 5.24 | 69.6 | pipelines/wait_ms 1730, pipelines/compile_ms 198, pipelines/front_ms 182, main/busy/ms 70.49, stream/drive/ms 44.49, render/thread/ms 38.01, render/busy/ms 36.76, pipelines/driver_ms 16.44, render/graph/ms 8.12, stream/spawn/ms 2.34, render/present/ms 0.97, terrain/update/ms 0.88 |
| warm-up | 5.32 | 49.4 | pipelines/wait_ms 7669, pipelines/compile_ms 106, pipelines/front_ms 97.47, main/busy/ms 51.36, render/thread/ms 43.11, render/busy/ms 41.53, stream/drive/ms 33.71, pipelines/driver_ms 8.45, render/graph/ms 4.18, stream/spawn/ms 1.86, render/present/ms 1.34, terrain/update/ms 0.90 |
| warm-up | 5.50 | 40.8 | pipelines/wait_ms 4785, pipelines/compile_ms 91.89, pipelines/front_ms 87.93, main/busy/ms 51.33, render/thread/ms 47.33, render/busy/ms 45.93, render/graph/ms 5.32, stream/drive/ms 4.79, pipelines/driver_ms 3.95, stream/spawn/ms 2.95, physics/step/ms 2.63, render/present/ms 1.03 |
| warm-up | 6.31 | 40.0 | pipelines/wait_ms 7118, render/thread/ms 44.37, render/busy/ms 42.60, pipelines/compile_ms 40.64, pipelines/front_ms 36.40, main/busy/ms 34.33, render/graph/ms 7.70, stream/drive/ms 4.51, pipelines/driver_ms 4.23, stream/spawn/ms 2.22, render/present/ms 1.35, stream/tick/ms 1.30 |
| warm-up | 6.75 | 32.7 | pipelines/wait_ms 6658, pipelines/compile_ms 105, pipelines/front_ms 98.17, render/thread/ms 46.69, render/busy/ms 44.31, main/busy/ms 38.34, render/graph/ms 12.34, pipelines/driver_ms 6.58, stream/drive/ms 3.03, render/present/ms 1.75, stream/tick/ms 1.15, bigspace/total/ms 0.91 |
| warm-up | 6.79 | 35.4 | pipelines/wait_ms 1844, render/thread/ms 63.16, render/busy/ms 60.94, main/busy/ms 54.77, pipelines/compile_ms 20.35, pipelines/front_ms 19.46, stream/drive/ms 13.85, render/graph/ms 11.20, render/present/ms 1.65, physics/insert/ms 1.28, stream/tick/ms 1.09, bigspace/total/ms 1.04 |
| warm-up | 6.84 | 45.9 | render/thread/ms 70.83, render/busy/ms 69.39, main/busy/ms 42.04, stream/drive/ms 14.29, render/graph/ms 8.76, physics/insert/ms 1.53, stream/spawn/ms 1.45, render/present/ms 1.18, bigspace/total/ms 0.83, stream/tick/ms 0.82, terrain/update/ms 0.77, terrain/select/ms 0.64 |
| warm-up | 8.10 | 38.0 | render/thread/ms 47.85, render/busy/ms 46.32, main/busy/ms 44.34, render/graph/ms 14.35, stream/drive/ms 1.69, render/present/ms 1.28, bigspace/total/ms 1.12, physics/insert/ms 0.86, anim/evaluate_cpu/ms 0.75, stream/tick/ms 0.70, terrain/update/ms 0.55, terrain/select/ms 0.47 |
| measure | 2.73 | 35.5 | render/thread/ms 49.27, render/busy/ms 47.67, render/graph/ms 25.13, main/busy/ms 21.47, render/present/ms 1.42, physics/insert/ms 0.95, bigspace/total/ms 0.83, terrain/update/ms 0.64, terrain/select/ms 0.57, controller/feed/ms 0.56, stream/drive/ms 0.52, stream/tick/ms 0.45 |
| measure | 59.05 | 32.5 | render/thread/ms 41.23, render/busy/ms 39.72, main/busy/ms 21.38, render/graph/ms 16.51, render/present/ms 1.28, bigspace/total/ms 1.13, controller/feed/ms 0.99, stream/drive/ms 0.78, terrain/update/ms 0.78, stream/tick/ms 0.68, terrain/select/ms 0.68, anim/evaluate/ms 0.65 |
| measure | 59.09 | 35.7 | render/thread/ms 41.11, render/busy/ms 39.98, main/busy/ms 17.99, render/graph/ms 12.04, render/present/ms 0.97, bigspace/total/ms 0.90, controller/feed/ms 0.78, terrain/update/ms 0.72, terrain/select/ms 0.63, stream/drive/ms 0.42, controller/move/ms 0.40, anim/evaluate_cpu/ms 0.37 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-on\tex-slab-on.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\slab2_on\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **75%** (min 44%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.1 W · 73 °C (max 80) · VRAM 5282 MiB · throttled 100% of samples
