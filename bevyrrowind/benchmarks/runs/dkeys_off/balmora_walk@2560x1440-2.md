# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4737 frames over 65.7 s · **median 13.51 ms (74 fps)** · mean 13.86 · p95 17.13 · p99 19.10 (1% low 52 fps) · worst 39.68 ms · 33 spikes over 1.5× median · 22.8 stutters/min (work 24.7/min)
- warm-up: 337 frames over 8.5 s, worst 2657.0 ms
- hitches (> 33.8 ms): **1** measured, 16 in warm-up
- timed GPU passes: 7.54 ms of a 13.86 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.51 ms, limit 11 ms
- **FAIL** frame_ms: 25 stutters (> 1.5x moving median of 60) in 1.09 min = 22.85/min, limit 1/min
- **FAIL** worst frame 39.68 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.073 | 2.074 | 2.182 | 1.00 | 0.181 |
| main_opaque_pass_3d | 1.648 | 1.648 | 2.046 | 1.00 | 0.062 |
| early prepass | 0.519 | 0.519 | 0.746 | 1.00 | 0.106 |
| bloom | 0.320 | 0.321 | 0.326 | 1.00 | 0.068 |
| atmosphere_luts | 0.314 | 0.315 | 0.374 | 1.00 | 0.004 |
| water_velocity | 0.284 | 0.284 | 0.353 | 1.00 | 0.002 |
| uniform_allocation | 0.250 | 0.250 | 0.263 | 1.00 | 0.048 |
| water_sspr | 0.209 | 0.209 | 0.211 | 1.00 | 0.002 |
| bin_unpacking | 0.193 | 0.193 | 0.210 | 1.00 | 0.031 |
| ssao | 0.192 | 0.193 | 0.194 | 1.00 | 0.008 |
| auxiliary_views | 0.187 | 0.187 | 0.656 | 1.00 | 0.174 |
| atmos_froxel_inject | 0.148 | 0.148 | 0.150 | 1.00 | 0.002 |
| clouds_trace | 0.142 | 0.142 | 0.233 | 1.00 | 0.001 |
| water_fft | 0.135 | 0.135 | 0.140 | 1.00 | 0.003 |
| upscaling | 0.119 | 0.119 | 0.126 | 1.00 | 0.004 |
| clouds_composite | 0.108 | 0.108 | 0.110 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.087 | 0.087 | 0.165 | 1.00 | 0.008 |
| tonemapping | 0.087 | 0.087 | 0.090 | 1.00 | 0.004 |
| clustering | 0.065 | 0.065 | 0.069 | 1.00 | 0.021 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.419 | 0.421 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.049 | 0.057 | 0.057 | 0.86 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.048 | 1.00 | 0.005 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| water_snapshot | 0.031 | 0.031 | 0.034 | 1.00 | 0.002 |
| lightprobe_radiance_map | 0.028 | 0.227 | 0.235 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.025 | 0.025 | 0.028 | 1.00 | 0.001 |
| water_mips | 0.023 | 0.023 | 0.023 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 1.00 | 0.006 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.032 | 1.00 | 0.112 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 42.085 | 0.000 | 16899.96 | 199356.8 |
| render/thread/ms | 12.534 | 15.557 | 27.67 | 59373.3 |
| render/busy/ms | 11.963 | 14.946 | 26.91 | 56667.3 |
| main/busy/ms | 9.890 | 12.600 | 29.23 | 46851.0 |
| render/graph/ms | 7.379 | 9.409 | 16.41 | 34952.5 |
| pipelines/compile_ms | 0.761 | 0.000 | 156.13 | 3606.8 |
| pipelines/front_ms | 0.735 | 0.000 | 151.07 | 3480.9 |
| render/present/ms | 0.478 | 0.571 | 3.17 | 2265.7 |
| bigspace/total/ms | 0.464 | 0.693 | 5.73 | 2198.7 |
| terrain/update/ms | 0.297 | 0.377 | 1.78 | 1406.4 |
| controller/feed/ms | 0.274 | 0.374 | 0.92 | 1296.7 |
| terrain/select/ms | 0.264 | 0.335 | 0.70 | 1248.5 |
| stream/drive/ms | 0.258 | 0.347 | 21.05 | 1220.4 |
| far/select/ms | 0.199 | 0.277 | 0.73 | 942.0 |
| stream/tick/ms | 0.195 | 0.263 | 0.80 | 926.0 |
| bigspace/low_precision/ms | 0.176 | 0.283 | 4.05 | 834.9 |
| anim/evaluate_cpu/ms | 0.159 | 0.207 | 0.57 | 752.6 |
| anim/evaluate/ms | 0.133 | 0.264 | 2.73 | 630.0 |
| controller/move/ms | 0.127 | 0.213 | 0.38 | 602.7 |
| bigspace/high_precision/ms | 0.108 | 0.207 | 3.38 | 513.3 |
| render/acquire/ms | 0.093 | 0.124 | 0.35 | 440.2 |
| physics/step/ms | 0.062 | 0.094 | 0.72 | 294.8 |
| physics/insert/ms | 0.054 | 0.437 | 1.00 | 257.3 |
| pipelines/driver_ms | 0.027 | 0.000 | 6.38 | 125.9 |
| weather/map/ms | 0.025 | 0.034 | 0.16 | 118.7 |
| anim/apply/ms | 0.011 | 0.020 | 0.19 | 53.0 |
| controller/avoid/ms | 0.011 | 0.014 | 0.16 | 50.7 |
| weather/eval/ms | 0.011 | 0.014 | 0.07 | 50.3 |
| script/update/ms | 0.007 | 0.010 | 0.20 | 33.5 |
| stream/despawn/ms | 0.006 | 0.000 | 1.21 | 29.8 |
| stream/spawn/ms | 0.006 | 0.000 | 1.16 | 26.3 |
| script/ui/ms | 0.005 | 0.007 | 0.16 | 25.3 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 23.9 |
| controller/write/ms | 0.003 | 0.005 | 0.22 | 16.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.68 | 14.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 27 | 0 |
| anim/a2/count | count | 1 | 13 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 91 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 51 | 10 |
| crowd/c0/joints/count | count | 1277 | 2218 | 434 |
| crowd/c0/meshes/count | count | 352 | 633 | 101 |
| crowd/c1/count | count | 146 | 157 | 151 |
| crowd/c1/joints/count | count | 5475 | 6037 | 5762 |
| crowd/c1/meshes/count | count | 1160 | 1311 | 1240 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4277 | 15.3 k | 6179 |
| crowd/joints_uploaded/count | count | 4242 | 6643 | 6179 |
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
| frame/work/ms | ms | 11.72 | 29.23 | 10.70 |
| gpu/timed/ms | ms | 8.23 | 11.43 | 8.66 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 822 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 822 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d/bins | count | 1283 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3425 | 5128 | 3181 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1283 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3425 | 5128 | 3181 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 58 | 272 | 47 |
| phases/shadow/bins | count | 4890 | 5923 | 2220 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.7 k | 13.3 k | 5675 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 256 | 315 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1220 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 47 | 0 |
| preprocess/bind_groups | count | 12 | 741 | 10 |
| preprocess/bind_groups_created | count | 12 | 741 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 258 | 384 | 247 |
| process/cpu_usage | % | 23.56 | 49.55 | 26.68 |
| process/mem_usage | GiB | 5.11 | 5.27 | 5.27 |
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
| stream/oldest_request/s | s | 0 | 0.34 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 35.37 | 65.93 | 35.53 |
| system/mem_usage | % | 70.02 | 71.51 | 71.29 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 125 | 338 | 117 |
| views/all/bins | count | 7472 | 8787 | 5700 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.6 k | 25.8 k | 13.3 k |
| views/all/pipelines | count | 322 | 382 | 113 |
| views/all/pipelines_distinct | count | 102 | 110 | 91 |
| views/all/work_items | count | 17.3 k | 23.4 k | 13.6 k |
| views/cascade/batch_sets | count | 56 | 73 | 47 |
| views/cascade/bins | count | 2603 | 3170 | 2220 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8144 | 10.2 k | 5675 |
| views/cascade/pipelines | count | 56 | 73 | 47 |
| views/cascade/work_items | count | 8144 | 10.2 k | 5675 |
| views/lamp_face/batch_sets | count | 0 | 216 | 0 |
| views/lamp_face/bins | count | 2220 | 2885 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2476 | 3179 | 0 |
| views/lamp_face/pipelines | count | 199 | 247 | 0 |
| views/lamp_face/work_items | count | 0 | 2848 | 0 |
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 2862 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8492 | 13.0 k | 7598 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8651 | 13.2 k | 7906 |
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
| anim/bones_written/count | count | 52.41 | 1051 | 248.2 k |
| anim/events/count | count | 0.12 | 4 | 551 |
| assets/Image/added | count | 0.15 | 43 | 703 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.94 | 152 | 4430 |
| assets/Mesh/removed | count | 1.03 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.12 | 45 | 583 |
| assets/StandardMaterial/modified | count | 4.57 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 3.25 | 3898 | 15.4 k |
| physics/inserts | count | 0.73 | 172 | 3438 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.49 | 145 | 2335 |
| physics/steps | count | 0.83 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.01 | 6 | 26 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 19 | 378 |
| stream/upload/bytes | bytes | 7146 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1567 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 33.8 ms; slowest 17, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.66 | 2657.0 | main/busy/ms 2632, surface/advance_cpu_ms 68.96, surface/cell_upload_ms 0.80, stream/drive/ms 0.28, weather/map/ms 0.12, bigspace/total/ms 0.10, stream/tick/ms 0.07, bigspace/high_precision/ms 0.06, script/update/ms 0.04, weather/eval/ms 0.04, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.50 | 843.8 | main/busy/ms 2664, render/thread/ms 808, render/busy/ms 642, render/acquire/ms 164, surface/advance_cpu_ms 68.96, render/graph/ms 4.78, render/present/ms 2.08, stream/drive/ms 0.95, surface/cell_upload_ms 0.80, terrain/update/ms 0.56, stream/tick/ms 0.48, terrain/select/ms 0.39 |
| warm-up | 3.59 | 93.2 | pipelines/wait_ms 1482, render/thread/ms 828, render/busy/ms 661, render/acquire/ms 164, pipelines/compile_ms 92.75, pipelines/front_ms 84.19, main/busy/ms 51.66, terrain/update/ms 17.65, render/graph/ms 17.20, pipelines/driver_ms 8.56, bigspace/total/ms 4.30, render/present/ms 3.05 |
| warm-up | 3.83 | 106.8 | pipelines/compile_ms 221, pipelines/front_ms 209, pipelines/wait_ms 208, render/thread/ms 74.73, render/busy/ms 73.09, main/busy/ms 57.73, pipelines/driver_ms 12.22, render/graph/ms 3.86, stream/drive/ms 1.38, render/present/ms 1.35, stream/tick/ms 1.12, terrain/update/ms 0.83 |
| warm-up | 3.88 | 38.3 | render/thread/ms 37.83, render/busy/ms 36.39, main/busy/ms 23.09, terrain/update/ms 6.39, render/graph/ms 4.01, stream/drive/ms 2.01, render/present/ms 1.26, stream/tick/ms 0.79, stream/spawn/ms 0.74, terrain/select/ms 0.62, bigspace/total/ms 0.38, bigspace/high_precision/ms 0.19 |
| warm-up | 3.99 | 75.3 | pipelines/wait_ms 209, pipelines/compile_ms 190, pipelines/front_ms 181, main/busy/ms 52.94, render/thread/ms 43.46, render/busy/ms 42.22, stream/drive/ms 41.63, pipelines/driver_ms 9.44, render/graph/ms 2.35, render/present/ms 1.03, stream/tick/ms 0.95, stream/spawn/ms 0.79 |
| warm-up | 4.05 | 53.8 | pipelines/wait_ms 2476, pipelines/compile_ms 67.95, pipelines/front_ms 60.15, main/busy/ms 37.52, render/thread/ms 32.75, render/busy/ms 30.97, stream/drive/ms 25.52, pipelines/driver_ms 7.79, render/graph/ms 4.16, render/present/ms 1.52, stream/spawn/ms 1.27, terrain/update/ms 0.97 |
| warm-up | 4.13 | 69.2 | pipelines/wait_ms 1335, render/thread/ms 54.90, render/busy/ms 51.92, main/busy/ms 31.06, pipelines/compile_ms 29.76, pipelines/front_ms 27.33, stream/drive/ms 12.06, render/graph/ms 7.93, render/present/ms 2.49, pipelines/driver_ms 2.43, stream/spawn/ms 1.88, stream/tick/ms 1.73 |
| warm-up | 4.26 | 97.4 | pipelines/wait_ms 1370, pipelines/compile_ms 156, pipelines/front_ms 148, render/thread/ms 74.99, render/busy/ms 51.80, main/busy/ms 48.20, stream/drive/ms 30.49, render/present/ms 23.01, pipelines/driver_ms 7.71, stream/spawn/ms 3.71, render/graph/ms 3.43, terrain/update/ms 0.89 |
| warm-up | 4.36 | 34.2 | pipelines/wait_ms 1888, render/thread/ms 46.88, render/busy/ms 44.88, pipelines/compile_ms 42.45, pipelines/front_ms 39.25, main/busy/ms 22.54, render/graph/ms 9.65, stream/drive/ms 5.73, pipelines/driver_ms 3.20, stream/spawn/ms 2.88, render/present/ms 1.72, stream/tick/ms 1.11 |
| warm-up | 4.44 | 44.7 | main/busy/ms 68.41, render/thread/ms 55.38, render/busy/ms 53.37, stream/drive/ms 50.85, render/graph/ms 12.19, stream/tick/ms 2.47, stream/spawn/ms 2.08, render/present/ms 1.74, terrain/update/ms 1.42, terrain/select/ms 1.26, bigspace/total/ms 0.40, render/acquire/ms 0.27 |
| warm-up | 4.49 | 51.8 | main/busy/ms 65.71, stream/drive/ms 49.06, render/thread/ms 48.44, render/busy/ms 47.01, render/graph/ms 6.76, stream/spawn/ms 2.07, render/present/ms 1.27, stream/tick/ms 0.84, terrain/update/ms 0.82, terrain/select/ms 0.69, bigspace/total/ms 0.45, stream/activate/ms 0.17 |
| warm-up | 4.58 | 68.1 | pipelines/wait_ms 1678, pipelines/compile_ms 216, pipelines/front_ms 208, main/busy/ms 68.79, stream/drive/ms 46.19, render/thread/ms 37.50, render/busy/ms 34.76, pipelines/driver_ms 8.15, render/graph/ms 5.13, render/present/ms 2.51, stream/spawn/ms 1.90, terrain/update/ms 0.98 |
| warm-up | 4.65 | 46.4 | main/busy/ms 49.54, stream/drive/ms 36.52, render/thread/ms 27.93, render/busy/ms 26.84, render/graph/ms 3.55, stream/activate/ms 1.07, stream/tick/ms 0.94, render/present/ms 0.87, stream/spawn/ms 0.85, terrain/update/ms 0.71, terrain/select/ms 0.59, physics/insert/ms 0.43 |
| warm-up | 5.35 | 34.9 | pipelines/wait_ms 7968, pipelines/compile_ms 164, pipelines/front_ms 159, main/busy/ms 37.03, render/thread/ms 22.00, render/busy/ms 20.75, stream/drive/ms 16.93, render/graph/ms 6.09, pipelines/driver_ms 4.88, render/present/ms 1.07, stream/tick/ms 0.80, stream/spawn/ms 0.72 |
| warm-up | 7.38 | 38.0 | main/busy/ms 51.34, render/thread/ms 41.84, render/busy/ms 40.30, render/graph/ms 13.28, stream/drive/ms 1.83, render/present/ms 1.26, physics/insert/ms 1.25, stream/tick/ms 1.09, bigspace/total/ms 1.02, terrain/update/ms 0.92, anim/evaluate_cpu/ms 0.87, terrain/select/ms 0.67 |
| measure | 59.07 | 39.7 | render/thread/ms 41.83, render/busy/ms 40.34, main/busy/ms 24.86, render/graph/ms 19.45, render/present/ms 1.27, bigspace/total/ms 1.15, controller/feed/ms 0.95, stream/drive/ms 0.89, terrain/update/ms 0.80, stream/tick/ms 0.79, terrain/select/ms 0.71, anim/evaluate_cpu/ms 0.57 |

## Run

- `C:\Users\leogloglo\AppData\Local\Temp\depth-keys-off\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\dkeys_off\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 2%
- during the run: other processes 12% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **70%** (min 34%) · 2580 MHz (min 2580) · memory 8001 MHz · 71.8 W · 74 °C (max 78) · VRAM 5621 MiB · throttled 100% of samples
