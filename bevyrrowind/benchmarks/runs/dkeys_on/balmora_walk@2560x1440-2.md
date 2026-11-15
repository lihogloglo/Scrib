# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5156 frames over 65.7 s · **median 12.46 ms (80 fps)** · mean 12.73 · p95 15.51 · p99 17.51 (1% low 57 fps) · worst 34.32 ms · 25 spikes over 1.5× median · 20.1 stutters/min (work 21.0/min)
- warm-up: 336 frames over 8.2 s, worst 2608.1 ms
- hitches (> 31.1 ms): **3** measured, 20 in warm-up
- timed GPU passes: 7.46 ms of a 12.73 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.46 ms, limit 11 ms
- **FAIL** frame_ms: 22 stutters (> 1.5x moving median of 60) in 1.09 min = 20.11/min, limit 1/min
- **FAIL** worst frame 34.32 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.079 | 2.089 | 2.260 | 0.99 | 0.181 |
| main_opaque_pass_3d | 1.661 | 1.669 | 2.056 | 1.00 | 0.064 |
| early prepass | 0.521 | 0.523 | 0.750 | 1.00 | 0.097 |
| bloom | 0.320 | 0.322 | 0.328 | 0.99 | 0.063 |
| atmosphere_luts | 0.317 | 0.319 | 0.375 | 1.00 | 0.003 |
| water_velocity | 0.286 | 0.288 | 0.354 | 0.99 | 0.001 |
| uniform_allocation | 0.223 | 0.224 | 0.237 | 1.00 | 0.040 |
| water_sspr | 0.210 | 0.211 | 0.211 | 1.00 | 0.002 |
| ssao | 0.193 | 0.194 | 0.195 | 1.00 | 0.007 |
| bin_unpacking | 0.182 | 0.182 | 0.191 | 1.00 | 0.027 |
| atmos_froxel_inject | 0.149 | 0.150 | 0.151 | 0.99 | 0.002 |
| clouds_trace | 0.140 | 0.141 | 0.233 | 0.99 | 0.001 |
| water_fft | 0.135 | 0.136 | 0.140 | 1.00 | 0.003 |
| upscaling | 0.121 | 0.121 | 0.140 | 0.99 | 0.004 |
| auxiliary_views | 0.114 | 0.115 | 0.553 | 1.00 | 0.107 |
| clouds_composite | 0.108 | 0.109 | 0.111 | 0.99 | 0.002 |
| main_transmissive_pass_3d | 0.090 | 0.091 | 0.170 | 1.00 | 0.008 |
| tonemapping | 0.088 | 0.088 | 0.091 | 0.99 | 0.003 |
| clustering | 0.065 | 0.066 | 0.068 | 1.00 | 0.020 |
| render_sky | 0.053 | 0.054 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.424 | 0.421 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 0.99 | 0.001 |
| surface_trail | 0.045 | 0.057 | 0.057 | 0.80 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.048 | 1.00 | 0.005 |
| water_snapshot | 0.034 | 0.034 | 0.036 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.027 | 0.225 | 0.233 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.025 | 0.025 | 0.028 | 0.99 | 0.001 |
| water_mips | 0.022 | 0.023 | 0.023 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 1.00 | 0.006 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.031 | 0.99 | 0.109 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.99 | 0.001 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 45.190 | 0.000 | 16850.92 | 232997.8 |
| render/thread/ms | 11.493 | 14.042 | 30.97 | 59258.3 |
| render/busy/ms | 10.926 | 13.390 | 30.21 | 56336.3 |
| main/busy/ms | 9.272 | 12.009 | 28.78 | 47805.6 |
| render/graph/ms | 6.683 | 8.391 | 16.04 | 34459.7 |
| pipelines/compile_ms | 0.729 | 0.000 | 173.83 | 3757.4 |
| pipelines/front_ms | 0.704 | 0.000 | 168.03 | 3627.8 |
| bigspace/total/ms | 0.480 | 0.849 | 4.04 | 2475.5 |
| render/present/ms | 0.474 | 0.561 | 5.86 | 2445.6 |
| terrain/update/ms | 0.296 | 0.377 | 1.85 | 1526.0 |
| controller/feed/ms | 0.270 | 0.347 | 0.75 | 1391.4 |
| terrain/select/ms | 0.263 | 0.335 | 0.70 | 1355.4 |
| stream/drive/ms | 0.250 | 0.347 | 20.49 | 1291.1 |
| stream/tick/ms | 0.192 | 0.266 | 0.62 | 991.5 |
| far/select/ms | 0.189 | 0.257 | 0.56 | 975.2 |
| bigspace/low_precision/ms | 0.187 | 0.339 | 3.71 | 965.8 |
| anim/evaluate_cpu/ms | 0.151 | 0.190 | 0.44 | 777.3 |
| anim/evaluate/ms | 0.133 | 0.253 | 3.73 | 685.7 |
| controller/move/ms | 0.129 | 0.216 | 0.53 | 665.1 |
| bigspace/high_precision/ms | 0.124 | 0.246 | 3.68 | 637.5 |
| render/acquire/ms | 0.092 | 0.120 | 0.37 | 476.5 |
| physics/insert/ms | 0.063 | 0.458 | 1.06 | 323.3 |
| physics/step/ms | 0.057 | 0.090 | 0.87 | 291.7 |
| pipelines/driver_ms | 0.025 | 0.000 | 6.72 | 129.6 |
| weather/map/ms | 0.025 | 0.035 | 0.06 | 128.9 |
| anim/apply/ms | 0.011 | 0.019 | 0.17 | 56.6 |
| controller/avoid/ms | 0.010 | 0.014 | 0.09 | 53.7 |
| weather/eval/ms | 0.010 | 0.014 | 0.19 | 52.0 |
| script/update/ms | 0.007 | 0.009 | 0.13 | 35.7 |
| stream/despawn/ms | 0.005 | 0.000 | 0.90 | 28.1 |
| script/input/ms | 0.005 | 0.007 | 0.09 | 26.7 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 25.5 |
| script/ui/ms | 0.005 | 0.006 | 0.03 | 24.4 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 16.6 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.70 | 14.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.5 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 13 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 44 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 10 |
| crowd/c0/joints/count | count | 1301 | 2262 | 434 |
| crowd/c0/meshes/count | count | 353 | 646 | 101 |
| crowd/c1/count | count | 146 | 156 | 151 |
| crowd/c1/joints/count | count | 5501 | 6004 | 5762 |
| crowd/c1/meshes/count | count | 1167 | 1308 | 1240 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4269 | 8145 | 6179 |
| crowd/joints_uploaded/count | count | 4251 | 6643 | 6179 |
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
| frame/work/ms | ms | 10.75 | 30.21 | 10.06 |
| gpu/timed/ms | ms | 8.18 | 11.36 | 8.50 |
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
| phases/alpha_mask3d/items | count | 813 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 813 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 29 | 28 |
| phases/opaque3d/bins | count | 1288 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3409 | 5110 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 28 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 24 | 26 | 24 |
| phases/opaque3d_prepass/bins | count | 1288 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3409 | 5110 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 23 | 25 | 23 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 49 | 262 | 39 |
| phases/shadow/bins | count | 4798 | 5758 | 2220 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.5 k | 13.2 k | 5675 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 242 | 294 | 39 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2347 |
| physics/items | count | 148 | 476 | 19 |
| physics/pending | count | 0 | 1459 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 211 | 216 | 216 |
| pipelines/waiting | count | 0 | 49 | 0 |
| preprocess/bind_groups | count | 11 | 715 | 11 |
| preprocess/bind_groups_created | count | 11 | 715 | 11 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 233 | 361 | 222 |
| process/cpu_usage | % | 24.31 | 44.02 | 23.08 |
| process/mem_usage | GiB | 5.12 | 5.30 | 5.29 |
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
| started_load_count |  | 4764 | 4858 | 4858 |
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
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.13 | 57.87 | 27.06 |
| system/mem_usage | % | 61.03 | 61.66 | 61.56 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 113 | 325 | 105 |
| views/all/bins | count | 7351 | 8604 | 5700 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.5 k | 25.6 k | 13.3 k |
| views/all/pipelines | count | 303 | 354 | 101 |
| views/all/pipelines_distinct | count | 93 | 100 | 83 |
| views/all/work_items | count | 17.3 k | 23.0 k | 13.6 k |
| views/cascade/batch_sets | count | 48 | 60 | 39 |
| views/cascade/bins | count | 2591 | 3107 | 2220 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8128 | 10.2 k | 5675 |
| views/cascade/pipelines | count | 48 | 60 | 39 |
| views/cascade/work_items | count | 8128 | 10.2 k | 5675 |
| views/lamp_face/batch_sets | count | 0 | 213 | 0 |
| views/lamp_face/bins | count | 2147 | 2748 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2392 | 3042 | 0 |
| views/lamp_face/pipelines | count | 194 | 238 | 0 |
| views/lamp_face/work_items | count | 0 | 2818 | 0 |
| views/main/batch_sets | count | 66 | 70 | 66 |
| views/main/bins | count | 2868 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8431 | 13.0 k | 7600 |
| views/main/pipelines | count | 62 | 66 | 62 |
| views/main/work_items | count | 8594 | 13.2 k | 7908 |
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
| anim/bones_written/count | count | 52.08 | 441 | 268.5 k |
| anim/events/count | count | 0.11 | 3 | 550 |
| assets/Image/added | count | 0.13 | 42 | 645 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.82 | 152 | 4241 |
| assets/Mesh/removed | count | 0.95 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 524 |
| assets/StandardMaterial/modified | count | 4.20 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.62 | 1767 | 8355 |
| physics/inserts | count | 0.70 | 172 | 3608 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.45 | 161 | 2335 |
| physics/steps | count | 0.76 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 23 | 378 |
| stream/upload/bytes | bytes | 6566 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1439 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.1 ms; slowest 23, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.61 | 2608.1 | main/busy/ms 2583, surface/advance_cpu_ms 69.37, surface/cell_upload_ms 0.73, stream/drive/ms 0.46, weather/map/ms 0.21, stream/tick/ms 0.11, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.03, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.35 | 744.4 | main/busy/ms 2624, render/thread/ms 709, render/busy/ms 612, render/acquire/ms 94.94, surface/advance_cpu_ms 69.37, render/graph/ms 5.32, render/present/ms 1.93, stream/drive/ms 1.21, surface/cell_upload_ms 0.73, stream/tick/ms 0.49, terrain/update/ms 0.44, weather/map/ms 0.35 |
| warm-up | 3.43 | 77.8 | pipelines/wait_ms 951, render/thread/ms 720, render/busy/ms 622, render/acquire/ms 95.14, pipelines/compile_ms 85.74, pipelines/front_ms 77.47, main/busy/ms 57.55, terrain/update/ms 23.44, render/graph/ms 9.21, pipelines/driver_ms 8.26, render/present/ms 2.94, stream/drive/ms 1.49 |
| warm-up | 3.68 | 90.9 | pipelines/compile_ms 215, pipelines/front_ms 204, main/busy/ms 63.51, render/thread/ms 48.04, render/busy/ms 46.46, pipelines/driver_ms 10.77, render/graph/ms 4.28, render/present/ms 1.37, stream/drive/ms 1.25, terrain/update/ms 1.10, stream/tick/ms 0.98, terrain/select/ms 0.95 |
| warm-up | 3.81 | 51.5 | pipelines/wait_ms 204, pipelines/compile_ms 175, pipelines/front_ms 166, main/busy/ms 37.38, render/thread/ms 35.98, render/busy/ms 34.85, stream/drive/ms 25.71, pipelines/driver_ms 8.49, render/graph/ms 3.12, render/present/ms 0.91, stream/tick/ms 0.88, terrain/update/ms 0.74 |
| warm-up | 3.88 | 53.0 | pipelines/wait_ms 2814, pipelines/compile_ms 89.64, pipelines/front_ms 80.52, render/thread/ms 44.69, main/busy/ms 44.58, render/busy/ms 42.76, stream/drive/ms 30.92, pipelines/driver_ms 9.12, render/graph/ms 4.65, render/present/ms 1.66, stream/spawn/ms 1.35, stream/tick/ms 0.97 |
| warm-up | 3.96 | 67.7 | pipelines/wait_ms 1744, pipelines/compile_ms 65.59, pipelines/front_ms 62.38, render/thread/ms 53.03, render/busy/ms 50.27, main/busy/ms 28.42, stream/drive/ms 9.59, render/graph/ms 6.99, pipelines/driver_ms 3.21, render/present/ms 2.49, stream/tick/ms 1.29, terrain/update/ms 0.95 |
| warm-up | 4.00 | 33.9 | pipelines/wait_ms 889, render/thread/ms 59.97, render/busy/ms 57.76, pipelines/compile_ms 48.65, pipelines/front_ms 47.12, main/busy/ms 43.43, stream/drive/ms 28.12, render/graph/ms 5.80, render/present/ms 1.98, pipelines/driver_ms 1.53, stream/tick/ms 1.21, terrain/update/ms 0.98 |
| warm-up | 4.09 | 96.7 | render/thread/ms 84.48, render/busy/ms 54.13, main/busy/ms 44.50, render/present/ms 30.20, stream/drive/ms 28.78, render/graph/ms 2.34, stream/spawn/ms 1.22, terrain/update/ms 0.85, stream/tick/ms 0.81, terrain/select/ms 0.71, stream/activate/ms 0.47, bigspace/total/ms 0.34 |
| warm-up | 4.30 | 74.9 | pipelines/wait_ms 486, pipelines/compile_ms 125, pipelines/front_ms 121, main/busy/ms 69.86, render/thread/ms 45.71, stream/drive/ms 45.33, render/busy/ms 44.43, render/graph/ms 6.21, pipelines/driver_ms 4.30, stream/spawn/ms 3.06, stream/tick/ms 1.61, terrain/update/ms 1.27 |
| warm-up | 4.34 | 33.0 | pipelines/wait_ms 486, pipelines/compile_ms 125, pipelines/front_ms 121, main/busy/ms 70.22, render/thread/ms 49.34, render/busy/ms 48.05, stream/drive/ms 46.90, pipelines/driver_ms 4.30, render/graph/ms 4.18, stream/spawn/ms 2.31, stream/tick/ms 1.67, terrain/update/ms 1.24 |
| warm-up | 4.41 | 70.0 | pipelines/wait_ms 6813, pipelines/compile_ms 116, pipelines/front_ms 107, main/busy/ms 69.11, stream/drive/ms 46.27, render/thread/ms 46.21, render/busy/ms 45.05, pipelines/driver_ms 9.04, render/graph/ms 3.56, stream/spawn/ms 2.11, terrain/update/ms 1.17, stream/activate/ms 1.12 |
| warm-up | 4.47 | 46.0 | main/busy/ms 45.41, stream/drive/ms 32.10, render/thread/ms 27.04, render/busy/ms 25.93, render/graph/ms 3.46, stream/activate/ms 1.08, stream/tick/ms 1.07, render/present/ms 0.94, stream/spawn/ms 0.92, terrain/update/ms 0.59, physics/insert/ms 0.52, terrain/select/ms 0.49 |
| warm-up | 4.52 | 45.3 | pipelines/wait_ms 14.5 k, pipelines/compile_ms 126, pipelines/front_ms 117, main/busy/ms 43.69, stream/drive/ms 25.83, render/thread/ms 22.09, render/busy/ms 20.62, pipelines/driver_ms 9.20, render/graph/ms 3.62, stream/spawn/ms 1.32, render/present/ms 1.22, stream/activate/ms 1.07 |
| warm-up | 4.60 | 35.1 | pipelines/wait_ms 1480, main/busy/ms 41.68, pipelines/compile_ms 37.63, pipelines/front_ms 36.08, render/thread/ms 30.38, render/busy/ms 29.21, terrain/update/ms 8.60, render/graph/ms 4.38, stream/drive/ms 3.57, pipelines/driver_ms 1.55, stream/spawn/ms 1.33, stream/tick/ms 1.23 |
| warm-up | 4.66 | 37.4 | pipelines/wait_ms 1299, render/thread/ms 48.93, render/busy/ms 47.14, pipelines/compile_ms 46.45, main/busy/ms 45.02, pipelines/front_ms 43.66, render/graph/ms 8.32, stream/drive/ms 4.64, bigspace/total/ms 2.91, pipelines/driver_ms 2.79, stream/spawn/ms 2.76, bigspace/high_precision/ms 2.39 |
| warm-up | 4.71 | 52.0 | pipelines/wait_ms 177, render/thread/ms 62.63, render/busy/ms 61.24, main/busy/ms 51.68, pipelines/compile_ms 23.70, pipelines/front_ms 21.94, render/graph/ms 6.08, stream/drive/ms 3.98, bigspace/total/ms 2.82, bigspace/high_precision/ms 2.36, stream/spawn/ms 2.09, pipelines/driver_ms 1.76 |
| warm-up | 4.75 | 34.1 | render/thread/ms 58.90, render/busy/ms 57.69, main/busy/ms 47.49, stream/drive/ms 7.45, render/graph/ms 4.42, stream/spawn/ms 2.11, render/present/ms 1.01, stream/tick/ms 0.85, terrain/update/ms 0.76, physics/insert/ms 0.68, terrain/select/ms 0.65, bigspace/total/ms 0.33 |
| warm-up | 5.35 | 37.6 | render/thread/ms 46.66, render/busy/ms 45.18, main/busy/ms 28.19, render/graph/ms 5.90, stream/drive/ms 5.21, stream/spawn/ms 2.16, stream/tick/ms 1.51, render/present/ms 1.20, physics/insert/ms 0.81, terrain/update/ms 0.76, bigspace/total/ms 0.65, terrain/select/ms 0.64 |
| warm-up | 6.68 | 35.6 | pipelines/wait_ms 7944, pipelines/compile_ms 55.79, pipelines/front_ms 53.43, main/busy/ms 51.37, render/thread/ms 41.33, render/busy/ms 39.97, render/graph/ms 8.29, pipelines/driver_ms 2.36, physics/insert/ms 1.28, stream/drive/ms 1.24, physics/step/ms 1.10, bigspace/total/ms 1.08 |
| measure | 2.73 | 32.3 | render/thread/ms 42.71, render/busy/ms 41.43, render/graph/ms 21.41, main/busy/ms 20.92, render/present/ms 1.12, physics/insert/ms 1.02, bigspace/total/ms 0.83, terrain/update/ms 0.56, terrain/select/ms 0.50, controller/feed/ms 0.50, stream/drive/ms 0.49, stream/tick/ms 0.44 |
| measure | 18.18 | 31.6 | main/busy/ms 38.15, render/thread/ms 26.82, render/busy/ms 25.79, stream/drive/ms 20.70, render/graph/ms 15.75, bigspace/total/ms 0.85, render/present/ms 0.84, terrain/update/ms 0.56, controller/feed/ms 0.55, terrain/select/ms 0.50, far/select/ms 0.37, stream/tick/ms 0.33 |
| measure | 59.04 | 34.3 | render/thread/ms 42.23, render/busy/ms 41.00, main/busy/ms 26.94, render/graph/ms 11.75, bigspace/total/ms 2.03, terrain/update/ms 1.14, stream/drive/ms 1.05, bigspace/high_precision/ms 1.02, terrain/select/ms 1.00, controller/feed/ms 1.00, stream/tick/ms 0.93, render/present/ms 0.93 |

## Run

- `C:\Users\leogloglo\AppData\Local\Temp\depth-keys-on\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\dkeys_on\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 4%, CPU 6%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **76%** (min 55%) · 2580 MHz (min 2580) · memory 8001 MHz · 75.7 W · 75 °C (max 80) · VRAM 5847 MiB · throttled 100% of samples
