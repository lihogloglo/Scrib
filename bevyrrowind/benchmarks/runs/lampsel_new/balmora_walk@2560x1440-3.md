# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4700 frames over 65.7 s · **median 13.08 ms (76 fps)** · mean 13.97 · p95 18.54 · p99 26.37 (1% low 38 fps) · worst 50.28 ms · 147 spikes over 1.5× median · 62.1 stutters/min (work 66.7/min)
- warm-up: 493 frames over 9.2 s, worst 2544.5 ms
- hitches (> 32.7 ms): **8** measured, 14 in warm-up
- timed GPU passes: 7.71 ms of a 13.97 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.08 ms, limit 11 ms
- **FAIL** frame_ms: 68 stutters (> 1.5x moving median of 60) in 1.09 min = 62.15/min, limit 1/min
- **FAIL** worst frame 50.28 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.132 | 2.137 | 2.757 | 1.00 | 0.185 |
| main_opaque_pass_3d | 1.661 | 1.665 | 2.090 | 1.00 | 0.071 |
| early prepass | 0.551 | 0.552 | 0.761 | 1.00 | 0.107 |
| bloom | 0.328 | 0.329 | 0.328 | 1.00 | 0.068 |
| atmosphere_luts | 0.322 | 0.322 | 0.380 | 1.00 | 0.003 |
| water_velocity | 0.294 | 0.294 | 0.355 | 1.00 | 0.002 |
| uniform_allocation | 0.246 | 0.255 | 0.265 | 0.96 | 0.050 |
| water_sspr | 0.213 | 0.215 | 0.212 | 0.99 | 0.002 |
| bin_unpacking | 0.204 | 0.204 | 0.216 | 1.00 | 0.032 |
| ssao | 0.200 | 0.200 | 0.198 | 1.00 | 0.008 |
| auxiliary_views | 0.191 | 0.191 | 0.741 | 1.00 | 0.185 |
| atmos_froxel_inject | 0.148 | 0.154 | 0.151 | 0.96 | 0.001 |
| clouds_trace | 0.143 | 0.144 | 0.237 | 1.00 | 0.001 |
| water_fft | 0.136 | 0.137 | 0.138 | 1.00 | 0.003 |
| upscaling | 0.123 | 0.124 | 0.131 | 1.00 | 0.005 |
| clouds_composite | 0.110 | 0.110 | 0.111 | 1.00 | 0.002 |
| tonemapping | 0.087 | 0.089 | 0.091 | 0.97 | 0.004 |
| main_transmissive_pass_3d | 0.086 | 0.086 | 0.167 | 1.00 | 0.009 |
| clustering | 0.066 | 0.067 | 0.069 | 0.99 | 0.020 |
| render_sky | 0.053 | 0.054 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.433 | 0.427 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.050 | 0.058 | 0.057 | 0.85 | 0.002 |
| early_mesh_preprocessing | 0.044 | 0.044 | 0.049 | 1.00 | 0.006 |
| water_snapshot | 0.035 | 0.036 | 0.036 | 1.00 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.97 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.235 | 0.234 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.025 | 0.026 | 0.028 | 0.96 | 0.001 |
| water_mips | 0.023 | 0.023 | 0.023 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.022 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.032 | 1.00 | 0.110 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.98 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.98 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.015 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.002 | 0.012 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 63.011 | 0.000 | 18412.03 | 296150.8 |
| render/thread/ms | 12.654 | 16.939 | 32.89 | 59474.9 |
| render/busy/ms | 12.042 | 16.146 | 31.29 | 56596.3 |
| main/busy/ms | 9.850 | 13.342 | 31.56 | 46293.2 |
| render/graph/ms | 7.361 | 10.206 | 19.78 | 34597.6 |
| pipelines/compile_ms | 0.655 | 0.000 | 176.10 | 3079.2 |
| pipelines/front_ms | 0.631 | 0.000 | 169.32 | 2966.7 |
| render/present/ms | 0.505 | 0.613 | 14.28 | 2372.1 |
| bigspace/total/ms | 0.472 | 0.749 | 8.73 | 2220.7 |
| terrain/update/ms | 0.296 | 0.377 | 1.93 | 1392.2 |
| controller/feed/ms | 0.288 | 0.410 | 0.78 | 1352.3 |
| terrain/select/ms | 0.263 | 0.335 | 0.82 | 1236.0 |
| stream/drive/ms | 0.254 | 0.377 | 20.22 | 1195.3 |
| far/select/ms | 0.198 | 0.289 | 0.67 | 928.5 |
| stream/tick/ms | 0.192 | 0.270 | 0.73 | 900.2 |
| bigspace/low_precision/ms | 0.184 | 0.305 | 8.38 | 864.3 |
| anim/evaluate_cpu/ms | 0.168 | 0.222 | 0.73 | 788.0 |
| controller/move/ms | 0.136 | 0.217 | 0.47 | 639.6 |
| anim/evaluate/ms | 0.134 | 0.278 | 2.31 | 631.1 |
| bigspace/high_precision/ms | 0.111 | 0.211 | 7.72 | 519.9 |
| render/acquire/ms | 0.108 | 0.125 | 8.89 | 506.5 |
| physics/step/ms | 0.063 | 0.102 | 0.57 | 297.0 |
| physics/insert/ms | 0.059 | 0.463 | 0.99 | 277.4 |
| weather/map/ms | 0.025 | 0.034 | 0.27 | 117.6 |
| pipelines/driver_ms | 0.024 | 0.000 | 6.78 | 112.6 |
| anim/apply/ms | 0.012 | 0.022 | 0.22 | 56.4 |
| controller/avoid/ms | 0.011 | 0.014 | 0.06 | 49.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.16 | 48.3 |
| script/update/ms | 0.007 | 0.010 | 0.18 | 34.7 |
| stream/despawn/ms | 0.006 | 0.000 | 0.99 | 30.3 |
| stream/spawn/ms | 0.006 | 0.000 | 1.12 | 26.9 |
| script/input/ms | 0.006 | 0.007 | 0.04 | 26.6 |
| script/ui/ms | 0.005 | 0.008 | 0.22 | 25.8 |
| controller/write/ms | 0.004 | 0.005 | 0.09 | 16.8 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.72 | 15.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.44 | 5.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 4.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 26 | 0 |
| anim/a2/count | count | 0 | 11 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 56 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 49 | 12 |
| crowd/c0/joints/count | count | 1247 | 2218 | 528 |
| crowd/c0/meshes/count | count | 349 | 633 | 135 |
| crowd/c1/count | count | 147 | 155 | 149 |
| crowd/c1/joints/count | count | 5531 | 5949 | 5668 |
| crowd/c1/meshes/count | count | 1173 | 1281 | 1206 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6402 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 224 |
| crowd/joints_extracted/count | count | 4295 | 13.0 k | 6213 |
| crowd/joints_uploaded/count | count | 4254 | 6643 | 6213 |
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
| frame/work/ms | ms | 11.31 | 31.56 | 9.42 |
| gpu/timed/ms | ms | 8.34 | 22.41 | 8.33 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 801 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 801 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 28 | 30 | 28 |
| phases/opaque3d/bins | count | 1295 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3378 | 5134 | 3181 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1295 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3378 | 5134 | 3181 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 57 | 280 | 47 |
| phases/shadow/bins | count | 4837 | 5974 | 2236 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.6 k | 13.0 k | 5697 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 254 | 337 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 518 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 54 | 0 |
| preprocess/bind_groups | count | 14 | 757 | 10 |
| preprocess/bind_groups_created | count | 14 | 757 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 258 | 375 | 247 |
| process/cpu_usage | % | 23.66 | 45.31 | 25.47 |
| process/mem_usage | GiB | 5.13 | 5.30 | 5.30 |
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
| started_load_count |  | 4767 | 4858 | 4858 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 432 | 528 | 396 |
| statics/materials_bound | count | 432 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.13 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.43 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.23 | 100 | 30.85 |
| system/mem_usage | % | 64.30 | 64.98 | 64.78 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 125 | 346 | 117 |
| views/all/bins | count | 7349 | 8927 | 5716 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.5 k | 25.5 k | 13.3 k |
| views/all/pipelines | count | 316 | 403 | 113 |
| views/all/pipelines_distinct | count | 102 | 110 | 91 |
| views/all/work_items | count | 17.1 k | 23.0 k | 13.6 k |
| views/cascade/batch_sets | count | 55 | 74 | 47 |
| views/cascade/bins | count | 2604 | 3174 | 2236 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8120 | 10.1 k | 5697 |
| views/cascade/pipelines | count | 55 | 74 | 47 |
| views/cascade/work_items | count | 8120 | 10.1 k | 5697 |
| views/lamp_face/batch_sets | count | 0 | 224 | 0 |
| views/lamp_face/bins | count | 2199 | 2892 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2449 | 3205 | 0 |
| views/lamp_face/pipelines | count | 196 | 266 | 0 |
| views/lamp_face/work_items | count | 0 | 2893 | 0 |
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 2888 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8359 | 13.0 k | 7598 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8532 | 13.2 k | 7906 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.5 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 56.34 | 995 | 264.8 k |
| anim/events/count | count | 0.12 | 3 | 574 |
| assets/Image/added | count | 0.17 | 68 | 809 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.97 | 184 | 4579 |
| assets/Mesh/removed | count | 1.04 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.15 | 75 | 701 |
| assets/StandardMaterial/modified | count | 4.61 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 3.96 | 3113 | 18.6 k |
| physics/inserts | count | 0.59 | 111 | 2779 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.50 | 145 | 2335 |
| physics/steps | count | 0.84 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.01 | 6 | 27 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 17 | 378 |
| stream/upload/bytes | bytes | 7203 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1579 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 32.7 ms; slowest 22, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2544.5 | main/busy/ms 2519, surface/advance_cpu_ms 70.11, surface/cell_upload_ms 1.02, stream/drive/ms 0.28, weather/map/ms 0.11, bigspace/total/ms 0.09, stream/tick/ms 0.07, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.46 | 919.1 | main/busy/ms 2554, render/thread/ms 881, render/busy/ms 731, render/acquire/ms 148, surface/advance_cpu_ms 70.11, render/graph/ms 5.12, render/present/ms 2.05, stream/drive/ms 1.37, surface/cell_upload_ms 1.02, stream/tick/ms 0.77, terrain/update/ms 0.75, terrain/select/ms 0.53 |
| warm-up | 3.54 | 74.1 | render/thread/ms 894, pipelines/wait_ms 789, render/busy/ms 742, render/acquire/ms 149, pipelines/compile_ms 80.28, pipelines/front_ms 72.61, main/busy/ms 49.94, terrain/update/ms 17.43, render/graph/ms 9.27, pipelines/driver_ms 7.67, render/present/ms 3.41, stream/drive/ms 1.67 |
| warm-up | 3.77 | 89.4 | main/busy/ms 67.09, render/thread/ms 43.90, render/busy/ms 42.06, render/graph/ms 3.74, render/present/ms 1.61, stream/drive/ms 1.56, stream/tick/ms 1.31, terrain/update/ms 0.93, terrain/select/ms 0.81, render/acquire/ms 0.23, bigspace/total/ms 0.22, weather/map/ms 0.10 |
| warm-up | 3.93 | 52.9 | main/busy/ms 44.01, render/thread/ms 40.13, render/busy/ms 38.72, stream/drive/ms 27.77, terrain/update/ms 4.64, render/graph/ms 3.49, stream/tick/ms 1.67, render/present/ms 1.14, terrain/select/ms 0.74, stream/spawn/ms 0.30, render/acquire/ms 0.27, bigspace/total/ms 0.19 |
| warm-up | 4.00 | 47.4 | pipelines/wait_ms 430, render/thread/ms 41.06, main/busy/ms 39.75, render/busy/ms 38.41, stream/drive/ms 26.51, render/graph/ms 5.58, render/present/ms 2.39, pipelines/compile_ms 1.92, stream/spawn/ms 1.62, pipelines/front_ms 1.52, stream/tick/ms 0.80, terrain/update/ms 0.68 |
| warm-up | 4.09 | 63.6 | pipelines/wait_ms 716, pipelines/compile_ms 204, pipelines/front_ms 182, render/thread/ms 64.31, render/busy/ms 54.78, main/busy/ms 29.97, pipelines/driver_ms 21.82, stream/drive/ms 10.99, render/present/ms 9.19, render/graph/ms 6.76, stream/tick/ms 1.66, stream/spawn/ms 1.26 |
| warm-up | 4.21 | 91.3 | pipelines/wait_ms 3074, pipelines/compile_ms 187, pipelines/front_ms 174, render/thread/ms 77.73, render/busy/ms 48.34, main/busy/ms 48.22, stream/drive/ms 29.82, render/present/ms 29.23, pipelines/driver_ms 13.06, stream/spawn/ms 3.37, render/graph/ms 3.35, stream/tick/ms 0.96 |
| warm-up | 4.28 | 50.5 | pipelines/wait_ms 3406, pipelines/compile_ms 107, pipelines/front_ms 99.91, main/busy/ms 47.11, render/thread/ms 44.31, render/busy/ms 41.74, render/graph/ms 11.99, pipelines/driver_ms 6.84, stream/drive/ms 4.07, render/present/ms 2.29, stream/tick/ms 1.22, stream/spawn/ms 1.19 |
| warm-up | 4.44 | 73.7 | pipelines/wait_ms 509, pipelines/compile_ms 117, pipelines/front_ms 115, main/busy/ms 63.79, render/thread/ms 46.93, stream/drive/ms 45.88, render/busy/ms 44.47, render/graph/ms 16.48, pipelines/driver_ms 2.93, stream/spawn/ms 2.40, render/present/ms 2.15, stream/tick/ms 0.80 |
| warm-up | 4.60 | 132.0 | pipelines/wait_ms 1254, pipelines/compile_ms 144, pipelines/front_ms 139, render/thread/ms 86.65, main/busy/ms 75.87, render/busy/ms 54.46, stream/drive/ms 49.46, render/present/ms 32.00, pipelines/driver_ms 5.04, render/graph/ms 4.30, stream/spawn/ms 2.25, stream/tick/ms 2.12 |
| warm-up | 4.67 | 39.5 | pipelines/wait_ms 6432, pipelines/compile_ms 134, pipelines/front_ms 125, main/busy/ms 48.38, render/thread/ms 44.71, render/busy/ms 42.74, stream/drive/ms 34.26, render/graph/ms 11.98, pipelines/driver_ms 9.70, render/present/ms 1.67, stream/spawn/ms 1.39, stream/tick/ms 1.10 |
| warm-up | 4.72 | 46.1 | main/busy/ms 46.69, stream/drive/ms 34.27, render/thread/ms 17.67, render/busy/ms 16.69, render/graph/ms 4.22, stream/spawn/ms 1.28, stream/activate/ms 1.06, stream/tick/ms 0.88, render/present/ms 0.82, terrain/update/ms 0.80, terrain/select/ms 0.67, physics/insert/ms 0.40 |
| warm-up | 5.94 | 34.9 | pipelines/wait_ms 7074, pipelines/compile_ms 208, pipelines/front_ms 192, render/thread/ms 51.92, render/busy/ms 50.78, main/busy/ms 37.83, stream/drive/ms 19.99, pipelines/driver_ms 16.11, render/graph/ms 5.89, stream/spawn/ms 2.08, render/present/ms 0.95, bigspace/total/ms 0.74 |
| measure | 0.09 | 50.3 | pipelines/wait_ms 3385, pipelines/compile_ms 139, pipelines/front_ms 135, main/busy/ms 42.55, render/thread/ms 32.16, render/busy/ms 31.04, render/graph/ms 6.00, pipelines/driver_ms 4.88, render/present/ms 0.90, bigspace/total/ms 0.85, physics/insert/ms 0.59, terrain/update/ms 0.58 |
| measure | 12.90 | 32.9 | render/thread/ms 45.41, render/busy/ms 30.32, main/busy/ms 25.11, render/graph/ms 19.62, render/present/ms 14.89, stream/drive/ms 1.88, stream/despawn/ms 1.12, bigspace/total/ms 1.09, controller/feed/ms 0.77, terrain/update/ms 0.56, terrain/select/ms 0.50, far/select/ms 0.49 |
| measure | 22.78 | 33.0 | render/thread/ms 50.78, main/busy/ms 45.59, render/busy/ms 35.79, render/graph/ms 21.38, render/present/ms 14.73, bigspace/total/ms 1.27, controller/feed/ms 0.83, anim/evaluate/ms 0.73, terrain/update/ms 0.71, stream/drive/ms 0.67, terrain/select/ms 0.65, bigspace/low_precision/ms 0.59 |
| measure | 25.78 | 36.0 | render/thread/ms 61.73, render/busy/ms 58.93, main/busy/ms 40.74, render/graph/ms 35.50, render/present/ms 2.40, bigspace/total/ms 1.25, physics/insert/ms 1.14, controller/feed/ms 0.81, stream/drive/ms 0.67, terrain/update/ms 0.67, terrain/select/ms 0.60, stream/tick/ms 0.59 |
| measure | 25.90 | 32.9 | render/thread/ms 50.03, render/busy/ms 48.24, main/busy/ms 35.77, render/graph/ms 29.12, bigspace/total/ms 1.97, render/present/ms 1.53, physics/insert/ms 1.21, controller/feed/ms 1.00, bigspace/low_precision/ms 0.96, stream/drive/ms 0.94, terrain/update/ms 0.81, stream/tick/ms 0.79 |
| measure | 26.16 | 33.8 | render/thread/ms 58.61, render/busy/ms 56.00, main/busy/ms 40.95, render/graph/ms 34.07, render/present/ms 2.29, bigspace/total/ms 1.21, controller/feed/ms 1.01, stream/drive/ms 0.85, terrain/update/ms 0.80, terrain/select/ms 0.71, stream/tick/ms 0.66, far/select/ms 0.64 |
| measure | 26.32 | 33.6 | render/thread/ms 61.05, render/busy/ms 59.29, main/busy/ms 41.64, render/graph/ms 35.12, render/present/ms 1.47, bigspace/total/ms 1.30, terrain/update/ms 0.98, controller/feed/ms 0.97, terrain/select/ms 0.87, stream/drive/ms 0.84, physics/insert/ms 0.77, stream/tick/ms 0.72 |
| measure | 27.50 | 38.2 | render/thread/ms 61.94, render/busy/ms 60.28, main/busy/ms 38.20, render/graph/ms 25.01, render/present/ms 1.40, stream/drive/ms 1.11, bigspace/total/ms 1.11, controller/feed/ms 0.98, anim/evaluate_cpu/ms 0.72, far/select/ms 0.71, terrain/update/ms 0.68, stream/tick/ms 0.64 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_new.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lampsel_new\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 8%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **72%** (min 36%) · 2580 MHz (min 2580) · memory 8001 MHz · 72.7 W · 75 °C (max 79) · VRAM 5412 MiB · throttled 100% of samples
