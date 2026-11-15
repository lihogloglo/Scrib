# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5026 frames over 65.6 s · **median 12.66 ms (79 fps)** · mean 13.06 · p95 16.27 · p99 18.43 (1% low 54 fps) · worst 34.95 ms · 35 spikes over 1.5× median · 20.1 stutters/min (work 16.5/min)
- warm-up: 499 frames over 10.1 s, worst 2504.6 ms
- hitches (> 31.6 ms): **2** measured, 24 in warm-up
- timed GPU passes: 7.49 ms of a 13.06 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.66 ms, limit 11 ms
- **FAIL** frame_ms: 22 stutters (> 1.5x moving median of 60) in 1.09 min = 20.11/min, limit 1/min
- **FAIL** worst frame 34.95 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.065 | 2.070 | 2.109 | 1.00 | 0.175 |
| main_opaque_pass_3d | 1.650 | 1.654 | 2.048 | 1.00 | 0.070 |
| early prepass | 0.522 | 0.523 | 0.748 | 1.00 | 0.095 |
| bloom | 0.319 | 0.320 | 0.328 | 1.00 | 0.063 |
| atmosphere_luts | 0.311 | 0.312 | 0.371 | 1.00 | 0.003 |
| water_velocity | 0.277 | 0.278 | 0.353 | 1.00 | 0.002 |
| uniform_allocation | 0.226 | 0.227 | 0.241 | 1.00 | 0.043 |
| water_sspr | 0.209 | 0.209 | 0.212 | 1.00 | 0.002 |
| auxiliary_views | 0.194 | 0.194 | 0.694 | 1.00 | 0.181 |
| ssao | 0.192 | 0.192 | 0.195 | 1.00 | 0.007 |
| bin_unpacking | 0.179 | 0.180 | 0.195 | 1.00 | 0.029 |
| atmos_froxel_inject | 0.148 | 0.148 | 0.150 | 1.00 | 0.001 |
| clouds_trace | 0.141 | 0.141 | 0.230 | 1.00 | 0.001 |
| water_fft | 0.132 | 0.133 | 0.139 | 1.00 | 0.003 |
| upscaling | 0.120 | 0.120 | 0.149 | 1.00 | 0.004 |
| clouds_composite | 0.108 | 0.108 | 0.110 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.089 | 0.089 | 0.168 | 1.00 | 0.008 |
| tonemapping | 0.087 | 0.087 | 0.090 | 1.00 | 0.004 |
| clustering | 0.065 | 0.065 | 0.068 | 1.00 | 0.019 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.417 | 0.420 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.047 | 0.057 | 0.057 | 0.82 | 0.002 |
| early_mesh_preprocessing | 0.041 | 0.041 | 0.048 | 1.00 | 0.005 |
| water_snapshot | 0.032 | 0.032 | 0.036 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.224 | 0.233 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.025 | 0.025 | 0.028 | 1.00 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.022 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.031 | 1.00 | 0.110 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.001 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 26.113 | 0.000 | 19262.29 | 131242.4 |
| render/thread/ms | 11.819 | 14.836 | 32.53 | 59401.4 |
| render/busy/ms | 11.241 | 14.136 | 31.70 | 56496.1 |
| main/busy/ms | 9.453 | 12.077 | 28.16 | 47510.1 |
| render/graph/ms | 6.842 | 8.899 | 15.66 | 34386.4 |
| render/present/ms | 0.489 | 0.568 | 4.48 | 2459.6 |
| bigspace/total/ms | 0.468 | 0.739 | 3.99 | 2350.7 |
| pipelines/compile_ms | 0.398 | 0.000 | 158.59 | 1999.4 |
| pipelines/front_ms | 0.384 | 0.000 | 152.49 | 1928.7 |
| terrain/update/ms | 0.297 | 0.375 | 2.35 | 1494.4 |
| controller/feed/ms | 0.273 | 0.350 | 1.04 | 1372.3 |
| terrain/select/ms | 0.265 | 0.334 | 0.56 | 1330.0 |
| stream/drive/ms | 0.244 | 0.333 | 19.91 | 1224.7 |
| far/select/ms | 0.192 | 0.261 | 0.57 | 965.4 |
| stream/tick/ms | 0.186 | 0.253 | 0.53 | 933.7 |
| bigspace/low_precision/ms | 0.185 | 0.294 | 3.20 | 928.4 |
| anim/evaluate_cpu/ms | 0.170 | 0.210 | 0.48 | 855.9 |
| controller/move/ms | 0.149 | 0.249 | 0.55 | 746.5 |
| anim/evaluate/ms | 0.146 | 0.276 | 3.15 | 732.2 |
| bigspace/high_precision/ms | 0.113 | 0.200 | 3.67 | 567.3 |
| render/acquire/ms | 0.089 | 0.113 | 0.32 | 445.7 |
| physics/step/ms | 0.057 | 0.089 | 0.57 | 285.6 |
| physics/insert/ms | 0.040 | 0.403 | 0.86 | 202.5 |
| weather/map/ms | 0.023 | 0.032 | 0.09 | 117.9 |
| pipelines/driver_ms | 0.014 | 0.000 | 6.10 | 70.7 |
| anim/apply/ms | 0.012 | 0.022 | 0.15 | 59.9 |
| controller/avoid/ms | 0.011 | 0.014 | 0.05 | 53.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.14 | 50.0 |
| script/update/ms | 0.007 | 0.008 | 0.04 | 34.1 |
| stream/despawn/ms | 0.006 | 0.000 | 0.98 | 28.5 |
| script/input/ms | 0.005 | 0.007 | 0.14 | 26.2 |
| stream/spawn/ms | 0.005 | 0.000 | 1.05 | 25.4 |
| script/ui/ms | 0.004 | 0.006 | 0.04 | 21.3 |
| controller/write/ms | 0.003 | 0.004 | 0.05 | 16.5 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.64 | 14.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 4.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 3.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 16 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 50 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1280 | 2262 | 594 |
| crowd/c0/meshes/count | count | 353 | 646 | 150 |
| crowd/c1/count | count | 147 | 157 | 148 |
| crowd/c1/joints/count | count | 5570 | 6046 | 5602 |
| crowd/c1/meshes/count | count | 1194 | 1322 | 1191 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 222 |
| crowd/joints_extracted/count | count | 4277 | 10.0 k | 6124 |
| crowd/joints_uploaded/count | count | 4259 | 6643 | 6124 |
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
| frame/work/ms | ms | 10.94 | 31.70 | 9.91 |
| gpu/timed/ms | ms | 8.20 | 11.35 | 8.37 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 802 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 802 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d/bins | count | 1292 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3381 | 5143 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 24 | 27 | 24 |
| phases/opaque3d_prepass/bins | count | 1292 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3381 | 5143 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 23 | 26 | 23 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 50 | 273 | 39 |
| phases/shadow/bins | count | 4826 | 5857 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.5 k | 12.9 k | 5633 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 245 | 305 | 39 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 218 | 476 | 19 |
| physics/pending | count | 0 | 248 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 216 | 221 | 221 |
| pipelines/waiting | count | 0 | 24 | 0 |
| preprocess/bind_groups | count | 12 | 737 | 6 |
| preprocess/bind_groups_created | count | 12 | 737 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 235 | 403 | 227 |
| process/cpu_usage | % | 24.04 | 50 | 20.19 |
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
| started_load_count |  | 4771 | 4863 | 4863 |
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
| system/cpu_usage | % | 29.35 | 46.82 | 27.22 |
| system/mem_usage | % | 61.04 | 61.88 | 61.82 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2591 | 2644 | 2644 |
| views/all/batch_sets | count | 115 | 336 | 105 |
| views/all/bins | count | 7336 | 8755 | 5692 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.5 k | 25.3 k | 13.2 k |
| views/all/pipelines | count | 306 | 365 | 101 |
| views/all/pipelines_distinct | count | 93 | 101 | 83 |
| views/all/work_items | count | 17.3 k | 23.2 k | 13.5 k |
| views/cascade/batch_sets | count | 48 | 63 | 39 |
| views/cascade/bins | count | 2594 | 3169 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8136 | 9999 | 5633 |
| views/cascade/pipelines | count | 48 | 63 | 39 |
| views/cascade/work_items | count | 8136 | 9999 | 5633 |
| views/lamp_face/batch_sets | count | 0 | 224 | 0 |
| views/lamp_face/bins | count | 2159 | 2804 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2403 | 3101 | 0 |
| views/lamp_face/pipelines | count | 196 | 246 | 0 |
| views/lamp_face/work_items | count | 0 | 2930 | 0 |
| views/main/batch_sets | count | 66 | 71 | 66 |
| views/main/bins | count | 2882 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8365 | 13.0 k | 7594 |
| views/main/pipelines | count | 62 | 67 | 62 |
| views/main/work_items | count | 8534 | 13.2 k | 7902 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.6 k | 37.7 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 61.64 | 566 | 309.8 k |
| anim/events/count | count | 0.12 | 3 | 604 |
| assets/Image/added | count | 0.12 | 43 | 604 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.82 | 152 | 4106 |
| assets/Mesh/removed | count | 0.97 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 479 |
| assets/StandardMaterial/modified | count | 4.31 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.61 | 1615 | 8077 |
| physics/inserts | count | 0.49 | 105 | 2440 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.46 | 161 | 2335 |
| physics/steps | count | 0.78 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 16 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 17 | 378 |
| stream/upload/bytes | bytes | 6735 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1477 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.6 ms; slowest 26, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2504.6 | main/busy/ms 2480, surface/advance_cpu_ms 68.14, surface/cell_upload_ms 0.69, stream/drive/ms 0.34, weather/map/ms 0.15, stream/tick/ms 0.09, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.06, script/update/ms 0.03, weather/eval/ms 0.03, script/ui/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.22 | 713.8 | main/busy/ms 2531, render/thread/ms 679, render/busy/ms 517, render/acquire/ms 159, surface/advance_cpu_ms 68.14, render/graph/ms 5.63, render/present/ms 2.20, stream/drive/ms 1.09, surface/cell_upload_ms 0.69, terrain/update/ms 0.60, stream/tick/ms 0.56, terrain/select/ms 0.40 |
| warm-up | 3.44 | 222.9 | pipelines/wait_ms 1327, render/thread/ms 689, render/busy/ms 527, pipelines/compile_ms 241, pipelines/front_ms 223, render/acquire/ms 159, main/busy/ms 67.72, terrain/update/ms 20.96, pipelines/driver_ms 17.99, render/graph/ms 9.39, render/present/ms 3.04, stream/drive/ms 1.24 |
| warm-up | 3.67 | 64.1 | pipelines/compile_ms 231, pipelines/front_ms 219, pipelines/wait_ms 217, render/thread/ms 62.80, render/busy/ms 60.76, main/busy/ms 58.06, pipelines/driver_ms 12.21, render/graph/ms 4.28, render/present/ms 1.81, stream/drive/ms 1.54, stream/tick/ms 1.30, terrain/update/ms 0.62 |
| warm-up | 3.72 | 34.7 | pipelines/wait_ms 461, pipelines/compile_ms 56.23, pipelines/front_ms 52.70, render/thread/ms 38.50, render/busy/ms 36.70, main/busy/ms 21.67, terrain/update/ms 5.31, render/graph/ms 4.33, pipelines/driver_ms 3.53, stream/drive/ms 2.13, render/present/ms 1.56, stream/tick/ms 1.07 |
| warm-up | 3.82 | 61.0 | main/busy/ms 43.48, render/thread/ms 41.59, render/busy/ms 40.45, stream/drive/ms 32.07, terrain/update/ms 7.04, render/graph/ms 2.58, render/present/ms 0.94, stream/tick/ms 0.93, stream/spawn/ms 0.62, terrain/select/ms 0.58, bigspace/total/ms 0.21, render/acquire/ms 0.19 |
| warm-up | 3.94 | 65.4 | pipelines/wait_ms 4061, pipelines/compile_ms 228, pipelines/front_ms 209, main/busy/ms 48.10, render/thread/ms 42.19, render/busy/ms 40.38, stream/drive/ms 21.29, pipelines/driver_ms 18.69, render/graph/ms 5.58, stream/spawn/ms 2.19, render/present/ms 1.52, stream/tick/ms 1.26 |
| warm-up | 3.97 | 35.8 | pipelines/wait_ms 3253, pipelines/compile_ms 222, pipelines/front_ms 204, main/busy/ms 59.61, render/thread/ms 55.93, render/busy/ms 54.00, stream/drive/ms 31.48, pipelines/driver_ms 18.17, render/graph/ms 5.66, stream/spawn/ms 1.71, stream/tick/ms 1.69, render/present/ms 1.62 |
| warm-up | 4.09 | 111.0 | pipelines/wait_ms 4726, pipelines/compile_ms 108, pipelines/front_ms 103, render/thread/ms 85.46, render/busy/ms 83.88, main/busy/ms 52.70, stream/drive/ms 30.97, pipelines/driver_ms 5.25, render/graph/ms 4.65, stream/tick/ms 1.37, render/present/ms 1.24, stream/spawn/ms 1.17 |
| warm-up | 4.12 | 38.4 | pipelines/wait_ms 4726, pipelines/compile_ms 108, pipelines/front_ms 103, render/thread/ms 96.07, render/busy/ms 94.33, main/busy/ms 23.21, render/graph/ms 15.34, pipelines/driver_ms 5.25, stream/drive/ms 4.03, render/present/ms 1.37, terrain/update/ms 1.07, stream/activate/ms 0.99 |
| warm-up | 4.27 | 41.5 | main/busy/ms 67.55, stream/drive/ms 46.73, render/thread/ms 41.01, render/busy/ms 39.20, render/graph/ms 4.89, stream/tick/ms 2.00, render/present/ms 1.38, bigspace/total/ms 1.33, stream/spawn/ms 1.17, bigspace/high_precision/ms 1.09, terrain/update/ms 0.93, terrain/select/ms 0.78 |
| warm-up | 4.34 | 71.0 | pipelines/wait_ms 522, pipelines/compile_ms 144, pipelines/front_ms 134, main/busy/ms 65.71, render/thread/ms 51.77, render/busy/ms 50.59, stream/drive/ms 46.01, pipelines/driver_ms 10.34, render/graph/ms 3.18, stream/tick/ms 1.31, stream/spawn/ms 1.18, render/present/ms 0.89 |
| warm-up | 4.43 | 72.8 | pipelines/wait_ms 1551, main/busy/ms 69.39, stream/drive/ms 52.28, pipelines/compile_ms 47.73, pipelines/front_ms 44.98, render/thread/ms 27.40, render/busy/ms 26.27, render/graph/ms 5.25, pipelines/driver_ms 2.75, stream/spawn/ms 2.15, terrain/update/ms 0.98, render/present/ms 0.90 |
| warm-up | 4.49 | 42.9 | main/busy/ms 45.18, render/thread/ms 37.87, render/busy/ms 36.13, stream/drive/ms 33.02, render/graph/ms 6.77, stream/spawn/ms 1.52, render/present/ms 1.45, terrain/update/ms 0.81, stream/tick/ms 0.80, terrain/select/ms 0.67, render/acquire/ms 0.30, bigspace/total/ms 0.24 |
| warm-up | 4.54 | 32.3 | main/busy/ms 37.37, stream/drive/ms 24.76, render/thread/ms 20.67, render/busy/ms 19.45, render/graph/ms 4.15, stream/spawn/ms 1.37, stream/activate/ms 1.06, render/present/ms 1.03, stream/tick/ms 0.78, terrain/update/ms 0.61, terrain/select/ms 0.51, physics/insert/ms 0.47 |
| warm-up | 4.76 | 31.8 | pipelines/wait_ms 9627, pipelines/compile_ms 173, pipelines/front_ms 167, render/thread/ms 42.10, render/busy/ms 41.04, main/busy/ms 20.92, stream/drive/ms 5.97, pipelines/driver_ms 5.93, render/graph/ms 3.99, render/present/ms 0.90, stream/tick/ms 0.81, terrain/update/ms 0.77 |
| warm-up | 5.35 | 32.7 | render/thread/ms 42.94, render/busy/ms 41.78, render/graph/ms 24.90, main/busy/ms 16.57, stream/drive/ms 2.58, stream/spawn/ms 1.52, render/present/ms 0.92, terrain/update/ms 0.72, stream/tick/ms 0.70, terrain/select/ms 0.60, bigspace/total/ms 0.42, render/acquire/ms 0.24 |
| warm-up | 5.55 | 34.8 | render/thread/ms 43.95, render/busy/ms 42.71, main/busy/ms 31.14, render/graph/ms 9.68, stream/drive/ms 4.36, stream/spawn/ms 2.55, stream/tick/ms 1.04, render/present/ms 1.02, bigspace/total/ms 0.83, terrain/update/ms 0.81, terrain/select/ms 0.68, bigspace/high_precision/ms 0.31 |
| warm-up | 5.63 | 33.5 | pipelines/wait_ms 4945, pipelines/compile_ms 138, pipelines/front_ms 133, render/thread/ms 49.15, render/busy/ms 47.83, main/busy/ms 20.89, render/graph/ms 8.32, pipelines/driver_ms 5.11, stream/drive/ms 3.47, stream/spawn/ms 1.50, render/present/ms 1.09, stream/tick/ms 0.88 |
| warm-up | 5.72 | 32.9 | render/thread/ms 42.35, render/busy/ms 40.85, main/busy/ms 27.14, render/graph/ms 8.75, stream/drive/ms 4.91, stream/spawn/ms 2.24, stream/tick/ms 1.41, render/present/ms 1.25, terrain/update/ms 0.77, bigspace/total/ms 0.72, terrain/select/ms 0.66, bigspace/high_precision/ms 0.36 |
| warm-up | 5.79 | 43.1 | pipelines/wait_ms 1651, pipelines/compile_ms 157, pipelines/front_ms 151, render/thread/ms 55.62, render/busy/ms 54.41, main/busy/ms 34.95, stream/drive/ms 18.28, render/graph/ms 7.99, pipelines/driver_ms 5.84, stream/spawn/ms 2.07, render/present/ms 1.05, stream/tick/ms 0.85 |
| warm-up | 6.25 | 33.6 | main/busy/ms 43.74, render/thread/ms 28.48, render/busy/ms 27.09, render/graph/ms 9.44, physics/insert/ms 2.05, stream/drive/ms 1.68, render/present/ms 1.05, bigspace/total/ms 0.86, stream/tick/ms 0.79, terrain/update/ms 0.62, terrain/select/ms 0.53, stream/spawn/ms 0.52 |
| warm-up | 6.32 | 33.5 | main/busy/ms 43.97, render/thread/ms 42.03, render/busy/ms 40.82, render/graph/ms 13.14, physics/insert/ms 1.80, stream/drive/ms 1.36, bigspace/total/ms 1.20, render/present/ms 0.97, stream/tick/ms 0.72, terrain/update/ms 0.60, anim/evaluate_cpu/ms 0.58, terrain/select/ms 0.52 |
| warm-up | 6.36 | 31.8 | main/busy/ms 48.63, render/thread/ms 47.50, render/busy/ms 46.30, render/graph/ms 10.39, bigspace/total/ms 2.19, physics/insert/ms 1.53, stream/drive/ms 1.36, bigspace/low_precision/ms 1.22, anim/evaluate_cpu/ms 0.95, render/present/ms 0.93, stream/tick/ms 0.78, terrain/update/ms 0.76 |
| measure | 2.72 | 33.6 | render/thread/ms 44.02, render/busy/ms 42.68, render/graph/ms 21.34, main/busy/ms 18.52, render/present/ms 1.18, bigspace/total/ms 0.83, terrain/update/ms 0.56, terrain/select/ms 0.49, stream/drive/ms 0.46, stream/tick/ms 0.41, controller/feed/ms 0.41, bigspace/low_precision/ms 0.39 |
| measure | 59.04 | 35.0 | render/thread/ms 44.23, render/busy/ms 42.94, main/busy/ms 29.04, render/graph/ms 13.37, bigspace/total/ms 4.36, bigspace/high_precision/ms 3.46, controller/feed/ms 1.02, render/present/ms 0.99, terrain/update/ms 0.87, terrain/select/ms 0.77, far/select/ms 0.69, stream/drive/ms 0.62 |

## Run

- `C:\Users\leogloglo\AppData\Local\Temp\depth-keys-on\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\dkeys_on\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **74%** (min 56%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.3 W · 74 °C (max 79) · VRAM 5842 MiB · throttled 100% of samples
