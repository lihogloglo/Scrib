# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5025 frames over 65.7 s · **median 12.50 ms (80 fps)** · mean 13.07 · p95 16.49 · p99 18.86 (1% low 53 fps) · worst 38.88 ms · 54 spikes over 1.5× median · 19.2 stutters/min (work 14.6/min)
- warm-up: 1989 frames over 26.1 s, worst 2526.2 ms
- hitches (> 31.3 ms): **3** measured, 22 in warm-up
- timed GPU passes: 7.76 ms of a 13.07 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.50 ms, limit 11 ms
- **FAIL** frame_ms: 21 stutters (> 1.5x moving median of 60) in 1.09 min = 19.19/min, limit 1/min
- **FAIL** worst frame 38.88 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.155 | 2.187 | 2.525 | 0.99 | 0.183 |
| main_opaque_pass_3d | 1.718 | 1.743 | 2.362 | 0.99 | 0.064 |
| early prepass | 0.534 | 0.541 | 0.851 | 0.99 | 0.105 |
| bloom | 0.333 | 0.338 | 0.494 | 0.99 | 0.065 |
| atmosphere_luts | 0.325 | 0.330 | 0.512 | 0.99 | 0.003 |
| water_velocity | 0.285 | 0.289 | 0.534 | 0.99 | 0.002 |
| uniform_allocation | 0.259 | 0.263 | 0.268 | 0.99 | 0.049 |
| water_sspr | 0.226 | 0.229 | 0.229 | 0.99 | 0.002 |
| ssao | 0.202 | 0.205 | 0.211 | 0.99 | 0.008 |
| bin_unpacking | 0.200 | 0.203 | 0.215 | 0.99 | 0.031 |
| atmos_froxel_inject | 0.154 | 0.157 | 0.153 | 0.99 | 0.002 |
| auxiliary_views | 0.152 | 0.155 | 0.610 | 0.99 | 0.144 |
| clouds_trace | 0.140 | 0.142 | 0.234 | 0.99 | 0.001 |
| water_fft | 0.137 | 0.139 | 0.139 | 0.99 | 0.003 |
| upscaling | 0.123 | 0.125 | 0.152 | 0.99 | 0.004 |
| clouds_composite | 0.110 | 0.111 | 0.111 | 0.99 | 0.002 |
| tonemapping | 0.091 | 0.092 | 0.093 | 0.99 | 0.003 |
| main_transmissive_pass_3d | 0.088 | 0.089 | 0.172 | 0.99 | 0.008 |
| clustering | 0.066 | 0.067 | 0.069 | 0.99 | 0.020 |
| render_sky | 0.054 | 0.054 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.049 | 0.436 | 0.723 | 0.11 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.99 | 0.001 |
| surface_trail | 0.047 | 0.058 | 0.057 | 0.81 | 0.002 |
| early_mesh_preprocessing | 0.044 | 0.044 | 0.049 | 0.99 | 0.005 |
| water_snapshot | 0.037 | 0.037 | 0.046 | 0.99 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.027 | 0.240 | 0.241 | 0.11 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.031 | 0.99 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 0.99 | 0.006 |
| main_transparent_pass_3d | 0.016 | 0.017 | 0.032 | 0.99 | 0.108 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.882 | 15.135 | 33.01 | 59706.8 |
| render/busy/ms | 11.289 | 14.498 | 32.22 | 56729.3 |
| main/busy/ms | 9.138 | 12.051 | 29.88 | 45920.7 |
| pipelines/wait_ms | 8.761 | 0.000 | 21989.42 | 44024.3 |
| render/graph/ms | 6.963 | 9.085 | 19.08 | 34988.9 |
| pipelines/compile_ms | 2.183 | 0.000 | 1933.94 | 10970.0 |
| pipelines/driver_ms | 1.948 | 0.000 | 1807.73 | 9790.4 |
| render/present/ms | 0.500 | 0.610 | 11.49 | 2512.6 |
| bigspace/total/ms | 0.479 | 0.819 | 4.31 | 2409.2 |
| terrain/update/ms | 0.295 | 0.376 | 1.61 | 1481.4 |
| controller/feed/ms | 0.285 | 0.380 | 0.82 | 1431.9 |
| terrain/select/ms | 0.262 | 0.332 | 0.73 | 1314.8 |
| stream/drive/ms | 0.261 | 0.369 | 21.38 | 1310.9 |
| pipelines/front_ms | 0.235 | 0.000 | 160.86 | 1179.6 |
| stream/tick/ms | 0.200 | 0.277 | 0.86 | 1006.0 |
| bigspace/low_precision/ms | 0.200 | 0.356 | 3.99 | 1003.6 |
| far/select/ms | 0.193 | 0.266 | 0.60 | 969.8 |
| anim/evaluate_cpu/ms | 0.174 | 0.214 | 0.70 | 875.0 |
| controller/move/ms | 0.130 | 0.233 | 0.49 | 653.3 |
| anim/evaluate/ms | 0.124 | 0.269 | 2.48 | 621.8 |
| bigspace/high_precision/ms | 0.113 | 0.213 | 3.75 | 568.7 |
| render/acquire/ms | 0.093 | 0.123 | 0.37 | 465.0 |
| physics/step/ms | 0.058 | 0.090 | 0.43 | 289.4 |
| physics/insert/ms | 0.039 | 0.395 | 0.83 | 194.1 |
| weather/map/ms | 0.024 | 0.033 | 0.19 | 120.8 |
| anim/apply/ms | 0.012 | 0.023 | 0.09 | 59.6 |
| controller/avoid/ms | 0.011 | 0.014 | 0.08 | 54.5 |
| weather/eval/ms | 0.010 | 0.014 | 0.04 | 52.2 |
| script/update/ms | 0.007 | 0.010 | 0.27 | 37.1 |
| stream/despawn/ms | 0.006 | 0.000 | 0.99 | 30.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 27.5 |
| script/input/ms | 0.005 | 0.006 | 0.15 | 25.4 |
| script/ui/ms | 0.005 | 0.006 | 0.03 | 23.0 |
| controller/write/ms | 0.004 | 0.004 | 0.03 | 17.7 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.65 | 14.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 4.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 28 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1272 | 2262 | 537 |
| crowd/c0/meshes/count | count | 352 | 646 | 146 |
| crowd/c1/count | count | 145 | 156 | 149 |
| crowd/c1/joints/count | count | 5539 | 5983 | 5659 |
| crowd/c1/meshes/count | count | 1167 | 1297 | 1195 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4271 | 9951 | 6179 |
| crowd/joints_uploaded/count | count | 4254 | 6643 | 6179 |
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
| frame/work/ms | ms | 10.83 | 32.22 | 9.94 |
| gpu/timed/ms | ms | 8.55 | 12.43 | 8.67 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 803 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 803 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d/bins | count | 1295 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3390 | 5161 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1295 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3390 | 5161 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 57 | 302 | 47 |
| phases/shadow/bins | count | 3723 | 5470 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9519 | 12.5 k | 5667 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 137 | 316 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 197 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 3 | 0 |
| preprocess/bind_groups | count | 12 | 797 | 10 |
| preprocess/bind_groups_created | count | 12 | 797 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 257 | 375 | 247 |
| process/cpu_usage | % | 24.04 | 42.41 | 20.67 |
| process/mem_usage | GiB | 5.34 | 5.53 | 5.51 |
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
| started_load_count |  | 4772 | 4864 | 4864 |
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
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.59 | 47.54 | 28.20 |
| system/mem_usage | % | 64.07 | 64.54 | 64.46 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 125 | 366 | 117 |
| views/all/bins | count | 6455 | 8303 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 201 | 382 | 113 |
| views/all/pipelines_distinct | count | 97 | 110 | 91 |
| views/all/work_items | count | 17.2 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 55 | 73 | 47 |
| views/cascade/bins | count | 2601 | 3221 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8144 | 10.1 k | 5667 |
| views/cascade/pipelines | count | 55 | 73 | 47 |
| views/cascade/work_items | count | 8144 | 10.1 k | 5667 |
| views/lamp_face/batch_sets | count | 0 | 241 | 0 |
| views/lamp_face/bins | count | 1052 | 2522 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1185 | 2871 | 0 |
| views/lamp_face/pipelines | count | 80 | 254 | 0 |
| views/lamp_face/work_items | count | 0 | 2584 | 0 |
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 2888 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8386 | 13.1 k | 7600 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8552 | 13.3 k | 7908 |
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
| anim/bones_written/count | count | 59.83 | 306 | 300.7 k |
| anim/events/count | count | 0.12 | 3 | 580 |
| assets/Image/added | count | 0.12 | 43 | 594 |
| assets/Image/modified | count | 0.00 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.81 | 152 | 4075 |
| assets/Mesh/removed | count | 0.97 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 466 |
| assets/StandardMaterial/modified | count | 4.31 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.94 | 1386 | 4717 |
| physics/inserts | count | 0.45 | 110 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.46 | 145 | 2335 |
| physics/steps | count | 0.78 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 21 | 378 |
| stream/upload/bytes | bytes | 6737 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1477 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.3 ms; slowest 25, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.53 | 2526.2 | main/busy/ms 2499, surface/advance_cpu_ms 66.09, surface/cell_upload_ms 0.70, stream/drive/ms 0.38, stream/tick/ms 0.12, bigspace/total/ms 0.10, weather/map/ms 0.09, weather/eval/ms 0.05, bigspace/high_precision/ms 0.04, script/update/ms 0.03, script/input/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 3.12 | 590.8 | main/busy/ms 2532, render/thread/ms 557, render/busy/ms 433, render/acquire/ms 122, surface/advance_cpu_ms 66.09, render/graph/ms 5.98, render/present/ms 2.19, stream/drive/ms 1.14, surface/cell_upload_ms 0.70, stream/tick/ms 0.60, terrain/update/ms 0.50, terrain/select/ms 0.29 |
| warm-up | 3.28 | 165.1 | pipelines/wait_ms 603, render/thread/ms 573, render/busy/ms 447, pipelines/compile_ms 338, pipelines/driver_ms 315, render/acquire/ms 123, main/busy/ms 55.84, pipelines/front_ms 22.12, terrain/update/ms 16.28, render/graph/ms 10.07, render/present/ms 3.77, stream/drive/ms 1.89 |
| warm-up | 3.54 | 90.6 | pipelines/compile_ms 246, pipelines/wait_ms 235, pipelines/driver_ms 229, main/busy/ms 67.35, render/thread/ms 55.08, render/busy/ms 53.52, pipelines/front_ms 17.62, render/graph/ms 3.77, stream/drive/ms 1.43, render/present/ms 1.33, stream/tick/ms 1.14, terrain/update/ms 0.96 |
| warm-up | 3.68 | 56.7 | pipelines/wait_ms 1226, pipelines/compile_ms 47.39, main/busy/ms 42.64, pipelines/driver_ms 35.20, render/thread/ms 29.49, stream/drive/ms 28.23, render/busy/ms 28.00, pipelines/front_ms 12.19, render/graph/ms 3.77, stream/tick/ms 1.55, render/present/ms 1.27, terrain/update/ms 0.81 |
| warm-up | 3.71 | 31.4 | pipelines/wait_ms 1812, pipelines/compile_ms 145, pipelines/driver_ms 114, render/thread/ms 50.47, render/busy/ms 48.64, main/busy/ms 42.21, pipelines/front_ms 31.01, stream/drive/ms 27.81, render/graph/ms 4.78, render/present/ms 1.55, stream/tick/ms 1.46, terrain/update/ms 0.74 |
| warm-up | 3.75 | 41.5 | pipelines/wait_ms 586, pipelines/compile_ms 97.63, pipelines/driver_ms 78.80, render/thread/ms 55.70, render/busy/ms 53.33, main/busy/ms 43.62, stream/drive/ms 28.32, pipelines/front_ms 18.83, render/graph/ms 6.38, render/present/ms 2.03, stream/spawn/ms 1.71, stream/tick/ms 0.99 |
| warm-up | 3.78 | 32.1 | pipelines/wait_ms 713, render/thread/ms 54.23, render/busy/ms 52.10, main/busy/ms 34.06, pipelines/compile_ms 33.25, pipelines/driver_ms 18.34, stream/drive/ms 15.62, pipelines/front_ms 14.91, render/graph/ms 5.37, stream/spawn/ms 2.65, render/present/ms 1.86, stream/tick/ms 1.61 |
| warm-up | 3.85 | 67.4 | pipelines/wait_ms 2253, pipelines/compile_ms 105, pipelines/driver_ms 75.63, render/thread/ms 68.53, render/busy/ms 66.68, main/busy/ms 45.03, pipelines/front_ms 29.76, stream/drive/ms 20.82, stream/spawn/ms 6.92, render/graph/ms 5.89, stream/tick/ms 2.16, render/present/ms 1.59 |
| warm-up | 3.91 | 57.6 | pipelines/wait_ms 2358, pipelines/compile_ms 107, pipelines/driver_ms 80.17, render/thread/ms 79.37, render/busy/ms 77.58, main/busy/ms 57.14, stream/drive/ms 33.40, pipelines/front_ms 26.66, render/graph/ms 6.64, stream/spawn/ms 6.22, stream/tick/ms 1.59, render/present/ms 1.49 |
| warm-up | 3.95 | 44.1 | pipelines/wait_ms 1672, pipelines/compile_ms 116, pipelines/driver_ms 86.14, render/thread/ms 64.75, render/busy/ms 63.12, main/busy/ms 48.31, pipelines/front_ms 30.13, stream/drive/ms 27.75, render/graph/ms 4.74, stream/spawn/ms 1.44, render/present/ms 1.34, stream/activate/ms 0.97 |
| warm-up | 4.06 | 36.7 | render/thread/ms 41.45, render/busy/ms 39.31, main/busy/ms 31.15, stream/drive/ms 18.17, render/graph/ms 7.86, render/present/ms 1.85, stream/spawn/ms 1.28, stream/tick/ms 1.01, terrain/update/ms 0.92, terrain/select/ms 0.74, stream/activate/ms 0.68, render/acquire/ms 0.29 |
| warm-up | 4.20 | 115.2 | render/thread/ms 84.52, render/busy/ms 81.96, main/busy/ms 68.00, stream/drive/ms 51.50, render/graph/ms 19.88, render/present/ms 2.26, stream/tick/ms 1.94, terrain/update/ms 0.90, terrain/select/ms 0.72, bigspace/total/ms 0.37, render/acquire/ms 0.30, stream/spawn/ms 0.25 |
| warm-up | 4.28 | 62.7 | main/busy/ms 64.89, stream/drive/ms 46.97, render/thread/ms 29.01, render/busy/ms 27.45, render/graph/ms 5.31, stream/spawn/ms 2.15, stream/tick/ms 1.45, render/present/ms 1.29, terrain/update/ms 1.07, terrain/select/ms 0.84, stream/activate/ms 0.46, bigspace/total/ms 0.40 |
| warm-up | 4.36 | 60.8 | pipelines/compile_ms 648, pipelines/driver_ms 587, pipelines/wait_ms 363, pipelines/front_ms 61.22, main/busy/ms 55.67, stream/drive/ms 37.05, render/thread/ms 34.48, render/busy/ms 32.83, render/graph/ms 4.75, stream/spawn/ms 2.73, render/present/ms 1.43, stream/tick/ms 0.94 |
| warm-up | 4.42 | 44.4 | main/busy/ms 49.78, stream/drive/ms 32.30, render/thread/ms 30.30, render/busy/ms 28.73, render/graph/ms 5.01, stream/spawn/ms 1.71, render/present/ms 1.35, stream/activate/ms 1.03, stream/tick/ms 0.73, terrain/update/ms 0.72, terrain/select/ms 0.58, physics/insert/ms 0.39 |
| warm-up | 5.25 | 32.7 | render/thread/ms 39.27, render/busy/ms 37.54, main/busy/ms 25.83, stream/drive/ms 3.87, render/graph/ms 3.79, stream/spawn/ms 2.15, render/present/ms 1.42, terrain/update/ms 1.07, stream/tick/ms 1.01, terrain/select/ms 0.86, bigspace/total/ms 0.77, render/acquire/ms 0.31 |
| warm-up | 6.59 | 111.1 | render/thread/ms 117, render/busy/ms 115, main/busy/ms 14.09, render/graph/ms 3.18, stream/drive/ms 1.17, render/present/ms 1.09, terrain/update/ms 0.61, stream/tick/ms 0.58, bigspace/total/ms 0.52, terrain/select/ms 0.51, far/select/ms 0.33, stream/spawn/ms 0.30 |
| warm-up | 10.45 | 164.0 | pipelines/wait_ms 14.0 k, render/thread/ms 169, render/acquire/ms 157, pipelines/compile_ms 56.55, pipelines/driver_ms 56.54, main/busy/ms 12.73, render/busy/ms 11.68, render/graph/ms 5.34, render/present/ms 0.87, terrain/update/ms 0.60, stream/drive/ms 0.56, terrain/select/ms 0.50 |
| warm-up | 10.75 | 299.5 | pipelines/wait_ms 70.7 k, pipelines/compile_ms 4926, pipelines/driver_ms 4474, render/thread/ms 461, pipelines/front_ms 452, render/acquire/ms 448, main/busy/ms 12.16, render/busy/ms 11.92, render/graph/ms 5.82, render/present/ms 1.07, terrain/update/ms 0.60, terrain/select/ms 0.50 |
| warm-up | 18.11 | 44.2 | main/busy/ms 38.18, render/thread/ms 31.51, render/busy/ms 29.95, render/graph/ms 10.32, render/present/ms 1.24, bigspace/total/ms 0.80, terrain/update/ms 0.59, stream/drive/ms 0.55, terrain/select/ms 0.50, far/select/ms 0.49, stream/tick/ms 0.48, anim/evaluate_cpu/ms 0.38 |
| warm-up | 21.35 | 38.3 | main/busy/ms 40.32, render/thread/ms 39.00, render/busy/ms 37.39, render/graph/ms 15.31, render/present/ms 1.39, physics/insert/ms 1.18, bigspace/total/ms 1.10, anim/evaluate_cpu/ms 0.83, terrain/update/ms 0.67, terrain/select/ms 0.56, anim/evaluate/ms 0.52, stream/drive/ms 0.50 |
| measure | 2.73 | 34.0 | render/thread/ms 44.45, render/busy/ms 43.05, render/graph/ms 21.53, main/busy/ms 18.42, render/present/ms 1.24, physics/insert/ms 1.07, bigspace/total/ms 0.85, stream/drive/ms 0.68, stream/tick/ms 0.62, controller/feed/ms 0.60, terrain/update/ms 0.56, terrain/select/ms 0.51 |
| measure | 19.27 | 34.4 | render/thread/ms 46.13, render/busy/ms 44.77, render/graph/ms 27.06, main/busy/ms 20.93, render/present/ms 1.18, bigspace/total/ms 0.88, controller/feed/ms 0.65, physics/insert/ms 0.63, terrain/update/ms 0.55, stream/drive/ms 0.52, terrain/select/ms 0.50, stream/tick/ms 0.46 |
| measure | 59.06 | 38.9 | render/thread/ms 46.82, render/busy/ms 45.51, main/busy/ms 24.65, render/graph/ms 12.85, bigspace/total/ms 1.46, render/present/ms 0.94, controller/feed/ms 0.91, stream/drive/ms 0.87, terrain/update/ms 0.84, stream/tick/ms 0.76, terrain/select/ms 0.75, far/select/ms 0.73 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-on\tex-slab-on.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\slab_on\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **76%** (min 52%) · 2578 MHz (min 2565) · memory 8001 MHz · 75.5 W · 77 °C (max 82) · VRAM 5281 MiB · throttled 100% of samples
