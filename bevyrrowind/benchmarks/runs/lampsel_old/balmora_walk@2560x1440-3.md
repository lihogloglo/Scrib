# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5263 frames over 65.7 s · **median 12.05 ms (83 fps)** · mean 12.47 · p95 15.69 · p99 18.39 (1% low 54 fps) · worst 38.25 ms · 57 spikes over 1.5× median · 48.4 stutters/min (work 43.9/min)
- warm-up: 338 frames over 7.6 s, worst 2525.9 ms
- hitches (> 30.1 ms): **5** measured, 22 in warm-up
- timed GPU passes: 7.59 ms of a 12.47 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.05 ms, limit 11 ms
- **FAIL** frame_ms: 53 stutters (> 1.5x moving median of 60) in 1.09 min = 48.44/min, limit 1/min
- **FAIL** worst frame 38.25 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.159 | 2.202 | 2.675 | 0.98 | 0.178 |
| main_opaque_pass_3d | 1.648 | 1.681 | 2.406 | 0.98 | 0.059 |
| early prepass | 0.514 | 0.524 | 0.932 | 0.98 | 0.102 |
| bloom | 0.319 | 0.337 | 0.339 | 0.95 | 0.065 |
| atmosphere_luts | 0.310 | 0.335 | 0.628 | 0.93 | 0.003 |
| water_velocity | 0.294 | 0.300 | 0.566 | 0.98 | 0.001 |
| uniform_allocation | 0.242 | 0.261 | 0.264 | 0.93 | 0.048 |
| water_sspr | 0.214 | 0.224 | 0.222 | 0.96 | 0.002 |
| ssao | 0.201 | 0.205 | 0.210 | 0.98 | 0.007 |
| bin_unpacking | 0.199 | 0.203 | 0.213 | 0.98 | 0.030 |
| atmos_froxel_inject | 0.149 | 0.161 | 0.154 | 0.93 | 0.002 |
| clouds_trace | 0.142 | 0.144 | 0.238 | 0.98 | 0.001 |
| auxiliary_views | 0.140 | 0.143 | 0.630 | 0.98 | 0.131 |
| water_fft | 0.137 | 0.140 | 0.138 | 0.98 | 0.003 |
| upscaling | 0.129 | 0.132 | 0.213 | 0.98 | 0.004 |
| clouds_composite | 0.106 | 0.112 | 0.111 | 0.94 | 0.002 |
| tonemapping | 0.093 | 0.094 | 0.109 | 0.98 | 0.003 |
| main_transmissive_pass_3d | 0.084 | 0.085 | 0.171 | 0.98 | 0.008 |
| clustering | 0.063 | 0.067 | 0.069 | 0.95 | 0.020 |
| render_sky | 0.054 | 0.055 | 0.054 | 0.98 | 0.002 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.98 | 0.001 |
| lightprobe_irradiance_map | 0.046 | 0.432 | 0.431 | 0.11 | 0.001 |
| surface_trail | 0.046 | 0.059 | 0.057 | 0.77 | 0.002 |
| early_mesh_preprocessing | 0.041 | 0.042 | 0.049 | 0.98 | 0.005 |
| water_snapshot | 0.036 | 0.037 | 0.043 | 0.98 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.94 | 0.003 |
| atmos_froxel_integrate | 0.026 | 0.028 | 0.031 | 0.93 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.242 | 0.242 | 0.11 | 0.004 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.94 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.023 | 0.98 | 0.006 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.033 | 0.98 | 0.107 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.95 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.95 | 0.002 |
| night_sky | 0.007 | 0.007 | 0.024 | 0.98 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 0.98 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 69.663 | 0.000 | 17603.39 | 366634.5 |
| render/thread/ms | 11.278 | 14.357 | 31.67 | 59358.5 |
| render/busy/ms | 10.674 | 13.702 | 30.99 | 56176.6 |
| main/busy/ms | 8.867 | 11.484 | 29.87 | 46665.2 |
| render/graph/ms | 6.459 | 8.516 | 20.50 | 33993.3 |
| pipelines/compile_ms | 0.809 | 0.000 | 180.37 | 4258.6 |
| pipelines/front_ms | 0.779 | 0.000 | 171.69 | 4097.4 |
| render/present/ms | 0.501 | 0.609 | 11.70 | 2639.1 |
| bigspace/total/ms | 0.494 | 0.894 | 4.15 | 2600.3 |
| terrain/update/ms | 0.297 | 0.373 | 1.81 | 1563.0 |
| controller/feed/ms | 0.265 | 0.354 | 0.74 | 1394.1 |
| terrain/select/ms | 0.264 | 0.333 | 0.62 | 1391.1 |
| stream/drive/ms | 0.257 | 0.355 | 17.33 | 1355.0 |
| bigspace/low_precision/ms | 0.208 | 0.413 | 3.92 | 1094.1 |
| stream/tick/ms | 0.199 | 0.271 | 0.56 | 1046.2 |
| far/select/ms | 0.185 | 0.251 | 0.64 | 975.4 |
| anim/evaluate_cpu/ms | 0.156 | 0.201 | 1.09 | 821.6 |
| controller/move/ms | 0.135 | 0.268 | 0.58 | 708.9 |
| anim/evaluate/ms | 0.131 | 0.270 | 2.84 | 691.0 |
| bigspace/high_precision/ms | 0.125 | 0.259 | 3.61 | 657.5 |
| render/acquire/ms | 0.103 | 0.119 | 15.11 | 542.9 |
| physics/insert/ms | 0.059 | 0.454 | 1.01 | 310.6 |
| physics/step/ms | 0.054 | 0.088 | 1.05 | 286.5 |
| pipelines/driver_ms | 0.031 | 0.000 | 8.67 | 161.2 |
| weather/map/ms | 0.024 | 0.033 | 0.34 | 126.2 |
| anim/apply/ms | 0.011 | 0.021 | 0.21 | 57.9 |
| controller/avoid/ms | 0.010 | 0.014 | 0.23 | 55.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.16 | 53.3 |
| script/update/ms | 0.007 | 0.009 | 0.17 | 35.9 |
| stream/despawn/ms | 0.006 | 0.000 | 1.02 | 29.2 |
| script/ui/ms | 0.005 | 0.007 | 0.21 | 28.4 |
| script/input/ms | 0.005 | 0.007 | 0.18 | 25.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 25.8 |
| controller/write/ms | 0.004 | 0.004 | 0.03 | 18.6 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.65 | 14.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.08 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 55 | 0 |
| anim/a2/count | count | 1 | 13 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 157 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 89 | 14 |
| crowd/c0/joints/count | count | 1301 | 3347 | 652 |
| crowd/c0/meshes/count | count | 353 | 750 | 162 |
| crowd/c1/count | count | 147 | 155 | 146 |
| crowd/c1/joints/count | count | 5507 | 5940 | 5512 |
| crowd/c1/meshes/count | count | 1163 | 1281 | 1171 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4266 | 7825 | 6169 |
| crowd/joints_uploaded/count | count | 4252 | 6643 | 6169 |
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
| frame/work/ms | ms | 10.37 | 30.99 | 10.44 |
| gpu/timed/ms | ms | 8.57 | 27.61 | 8.36 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 814 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 814 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d/bins | count | 1288 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3412 | 5142 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1288 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3412 | 5142 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 57 | 297 | 47 |
| phases/shadow/bins | count | 3743 | 5561 | 2207 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9498 | 12.8 k | 5668 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 143 | 327 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1316 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 67 | 0 |
| preprocess/bind_groups | count | 12 | 787 | 10 |
| preprocess/bind_groups_created | count | 12 | 787 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 256 | 354 | 247 |
| process/cpu_usage | % | 24.32 | 38.66 | 25.48 |
| process/mem_usage | GiB | 5.15 | 5.33 | 5.33 |
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
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.25 | 45.05 | 27.85 |
| system/mem_usage | % | 63.98 | 64.67 | 64.65 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 125 | 361 | 117 |
| views/all/bins | count | 6466 | 8348 | 5687 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 205 | 387 | 113 |
| views/all/pipelines_distinct | count | 96 | 109 | 91 |
| views/all/work_items | count | 17.1 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 55 | 73 | 47 |
| views/cascade/bins | count | 2596 | 3142 | 2207 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8150 | 10.2 k | 5668 |
| views/cascade/pipelines | count | 55 | 73 | 47 |
| views/cascade/work_items | count | 8150 | 10.2 k | 5668 |
| views/lamp_face/batch_sets | count | 0 | 236 | 0 |
| views/lamp_face/bins | count | 1058 | 2612 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1193 | 2974 | 0 |
| views/lamp_face/pipelines | count | 86 | 254 | 0 |
| views/lamp_face/work_items | count | 0 | 2521 | 0 |
| views/main/batch_sets | count | 68 | 74 | 70 |
| views/main/bins | count | 2862 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8438 | 13.0 k | 7600 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8602 | 13.3 k | 7908 |
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
| anim/bones_written/count | count | 57.29 | 2069 | 301.5 k |
| anim/events/count | count | 0.10 | 3 | 526 |
| assets/Image/added | count | 0.18 | 50 | 935 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.90 | 152 | 4740 |
| assets/Mesh/removed | count | 0.93 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.16 | 55 | 834 |
| assets/StandardMaterial/modified | count | 4.12 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 4.61 | 7428 | 24.3 k |
| physics/inserts | count | 0.65 | 128 | 3412 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.44 | 161 | 2335 |
| physics/steps | count | 0.75 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.01 | 6 | 36 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 6432 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1410 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.1 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.53 | 2525.9 | main/busy/ms 2502, surface/advance_cpu_ms 66.91, surface/cell_upload_ms 0.72, stream/drive/ms 0.33, weather/map/ms 0.14, bigspace/total/ms 0.13, stream/tick/ms 0.10, bigspace/high_precision/ms 0.09, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.20 | 675.8 | main/busy/ms 2537, render/thread/ms 639, render/busy/ms 519, render/acquire/ms 118, surface/advance_cpu_ms 66.91, render/graph/ms 5.82, render/present/ms 2.16, stream/drive/ms 1.00, surface/cell_upload_ms 0.72, terrain/update/ms 0.70, stream/tick/ms 0.52, terrain/select/ms 0.50 |
| warm-up | 3.29 | 87.4 | pipelines/wait_ms 868, render/thread/ms 650, render/busy/ms 529, render/acquire/ms 118, pipelines/compile_ms 97.23, pipelines/front_ms 90.22, main/busy/ms 52.45, terrain/update/ms 20.23, render/graph/ms 9.20, pipelines/driver_ms 7.01, render/present/ms 3.02, stream/drive/ms 1.61 |
| warm-up | 3.53 | 92.8 | pipelines/compile_ms 219, pipelines/front_ms 198, main/busy/ms 69.21, render/thread/ms 65.62, render/busy/ms 63.56, pipelines/driver_ms 20.54, render/graph/ms 4.31, stream/drive/ms 2.16, stream/tick/ms 1.73, render/present/ms 1.68, terrain/update/ms 0.66, terrain/select/ms 0.54 |
| warm-up | 3.58 | 33.8 | render/thread/ms 38.05, render/busy/ms 36.78, main/busy/ms 23.61, terrain/update/ms 5.75, render/graph/ms 3.23, stream/drive/ms 1.89, render/present/ms 1.07, stream/tick/ms 0.90, terrain/select/ms 0.70, stream/spawn/ms 0.58, bigspace/total/ms 0.25, render/acquire/ms 0.20 |
| warm-up | 3.68 | 60.3 | pipelines/wait_ms 929, pipelines/compile_ms 198, pipelines/front_ms 182, render/thread/ms 45.40, main/busy/ms 45.21, render/busy/ms 44.08, stream/drive/ms 35.54, pipelines/driver_ms 16.69, terrain/update/ms 6.01, render/graph/ms 3.16, render/present/ms 1.14, stream/tick/ms 0.92 |
| warm-up | 3.72 | 31.6 | pipelines/wait_ms 1186, pipelines/compile_ms 52.98, pipelines/front_ms 47.05, render/thread/ms 25.98, render/busy/ms 24.50, main/busy/ms 22.53, stream/drive/ms 7.90, pipelines/driver_ms 5.93, render/graph/ms 4.21, render/present/ms 1.24, stream/tick/ms 1.13, terrain/update/ms 0.92 |
| warm-up | 3.81 | 73.8 | pipelines/wait_ms 1320, pipelines/compile_ms 92.05, pipelines/front_ms 86.33, main/busy/ms 57.17, render/thread/ms 53.96, render/busy/ms 52.30, stream/drive/ms 28.10, render/graph/ms 5.77, pipelines/driver_ms 5.71, stream/tick/ms 2.50, stream/spawn/ms 2.43, render/present/ms 1.41 |
| warm-up | 3.94 | 103.9 | pipelines/wait_ms 1153, render/thread/ms 75.41, main/busy/ms 53.43, render/busy/ms 48.29, pipelines/compile_ms 39.75, pipelines/front_ms 37.02, stream/drive/ms 28.15, render/present/ms 26.94, terrain/update/ms 10.08, render/graph/ms 3.47, pipelines/driver_ms 2.73, stream/spawn/ms 1.98 |
| warm-up | 3.98 | 37.4 | pipelines/wait_ms 1563, pipelines/compile_ms 134, pipelines/front_ms 126, render/thread/ms 84.52, render/busy/ms 57.36, main/busy/ms 29.77, render/present/ms 26.92, pipelines/driver_ms 8.28, render/graph/ms 4.53, stream/drive/ms 3.81, terrain/update/ms 2.86, stream/tick/ms 0.93 |
| warm-up | 4.02 | 46.1 | pipelines/wait_ms 3163, pipelines/compile_ms 164, pipelines/front_ms 154, render/thread/ms 65.92, render/busy/ms 64.18, main/busy/ms 23.41, render/graph/ms 10.99, pipelines/driver_ms 9.83, stream/drive/ms 3.20, render/present/ms 1.50, terrain/update/ms 1.08, stream/activate/ms 1.05 |
| warm-up | 4.11 | 59.9 | render/thread/ms 69.75, render/busy/ms 65.95, main/busy/ms 45.39, render/graph/ms 15.88, stream/drive/ms 5.56, render/present/ms 3.40, stream/spawn/ms 2.72, stream/tick/ms 1.10, terrain/update/ms 0.67, stream/activate/ms 0.64, terrain/select/ms 0.58, render/acquire/ms 0.40 |
| warm-up | 4.15 | 39.5 | render/thread/ms 76.22, render/busy/ms 71.96, main/busy/ms 46.25, render/graph/ms 13.75, stream/drive/ms 5.52, render/present/ms 3.72, stream/spawn/ms 2.98, stream/tick/ms 1.71, terrain/update/ms 0.90, terrain/select/ms 0.77, bigspace/total/ms 0.70, render/acquire/ms 0.54 |
| warm-up | 4.20 | 55.1 | pipelines/wait_ms 4273, pipelines/compile_ms 209, pipelines/front_ms 203, main/busy/ms 95.35, render/thread/ms 60.63, render/busy/ms 58.76, stream/drive/ms 54.93, render/graph/ms 6.20, pipelines/driver_ms 6.04, stream/tick/ms 1.63, render/present/ms 1.53, stream/spawn/ms 1.38 |
| warm-up | 4.27 | 63.7 | pipelines/wait_ms 6259, pipelines/compile_ms 238, pipelines/front_ms 231, main/busy/ms 83.29, stream/drive/ms 56.55, render/thread/ms 51.82, render/busy/ms 50.63, pipelines/driver_ms 7.28, render/graph/ms 4.44, terrain/update/ms 4.40, stream/spawn/ms 3.37, render/present/ms 1.02 |
| warm-up | 4.35 | 64.4 | pipelines/wait_ms 1358, pipelines/compile_ms 134, pipelines/front_ms 130, main/busy/ms 68.15, stream/drive/ms 45.85, render/thread/ms 38.21, render/busy/ms 36.77, render/graph/ms 5.68, pipelines/driver_ms 3.72, stream/spawn/ms 2.05, terrain/update/ms 1.91, render/present/ms 1.22 |
| warm-up | 4.42 | 40.8 | pipelines/wait_ms 253, main/busy/ms 44.79, render/thread/ms 32.56, stream/drive/ms 31.65, render/busy/ms 31.13, pipelines/compile_ms 16.23, pipelines/front_ms 15.50, render/graph/ms 4.84, render/present/ms 1.25, stream/activate/ms 1.04, stream/tick/ms 0.92, pipelines/driver_ms 0.74 |
| warm-up | 4.46 | 37.5 | main/busy/ms 38.75, stream/drive/ms 23.93, render/thread/ms 21.25, render/busy/ms 20.02, render/graph/ms 4.08, stream/tick/ms 1.17, stream/activate/ms 1.07, render/present/ms 1.03, stream/spawn/ms 0.75, terrain/update/ms 0.66, physics/insert/ms 0.62, terrain/select/ms 0.59 |
| warm-up | 4.77 | 35.7 | pipelines/wait_ms 487, pipelines/compile_ms 147, pipelines/front_ms 141, render/thread/ms 49.17, render/busy/ms 45.71, main/busy/ms 21.37, render/graph/ms 6.32, pipelines/driver_ms 6.29, stream/drive/ms 3.49, render/present/ms 3.18, stream/spawn/ms 1.87, stream/tick/ms 0.83 |
| warm-up | 4.82 | 31.5 | render/thread/ms 33.87, render/busy/ms 32.73, main/busy/ms 21.30, stream/drive/ms 4.42, render/graph/ms 4.03, stream/spawn/ms 2.50, render/present/ms 0.94, stream/tick/ms 0.81, terrain/update/ms 0.71, terrain/select/ms 0.62, bigspace/total/ms 0.56, stream/activate/ms 0.25 |
| warm-up | 5.24 | 31.4 | render/thread/ms 37.46, render/busy/ms 36.36, main/busy/ms 22.32, stream/drive/ms 5.37, render/graph/ms 3.91, stream/tick/ms 2.13, stream/spawn/ms 2.11, render/present/ms 0.90, terrain/update/ms 0.74, terrain/select/ms 0.65, bigspace/total/ms 0.51, bigspace/high_precision/ms 0.23 |
| warm-up | 5.43 | 30.5 | render/thread/ms 37.45, render/busy/ms 36.31, main/busy/ms 28.44, stream/drive/ms 4.70, render/graph/ms 3.41, stream/spawn/ms 2.08, stream/tick/ms 1.50, bigspace/total/ms 1.46, render/present/ms 0.95, bigspace/high_precision/ms 0.72, terrain/update/ms 0.72, terrain/select/ms 0.63 |
| measure | 1.53 | 38.3 | main/busy/ms 51.29, render/thread/ms 31.70, render/busy/ms 29.90, render/graph/ms 9.09, anim/evaluate_cpu/ms 1.66, render/present/ms 1.59, bigspace/total/ms 1.39, bigspace/low_precision/ms 0.85, physics/insert/ms 0.77, anim/evaluate/ms 0.64, terrain/update/ms 0.53, terrain/select/ms 0.47 |
| measure | 1.57 | 32.7 | main/busy/ms 43.29, render/thread/ms 37.19, render/busy/ms 36.02, render/graph/ms 5.48, anim/evaluate_cpu/ms 1.17, physics/insert/ms 0.99, render/present/ms 0.96, bigspace/total/ms 0.96, terrain/update/ms 0.53, stream/drive/ms 0.51, physics/step/ms 0.51, bigspace/low_precision/ms 0.49 |
| measure | 3.18 | 32.5 | pipelines/wait_ms 14.8 k, render/thread/ms 55.36, render/busy/ms 33.31, render/acquire/ms 20.86, main/busy/ms 18.85, pipelines/compile_ms 15.44, render/graph/ms 15.24, pipelines/front_ms 14.58, render/present/ms 1.19, pipelines/driver_ms 0.86, bigspace/total/ms 0.81, terrain/update/ms 0.71 |
| measure | 19.27 | 33.0 | render/thread/ms 44.53, render/busy/ms 43.28, render/graph/ms 28.60, main/busy/ms 16.79, render/present/ms 1.10, controller/feed/ms 0.77, bigspace/total/ms 0.75, terrain/update/ms 0.62, terrain/select/ms 0.56, physics/insert/ms 0.47, stream/drive/ms 0.43, stream/tick/ms 0.37 |
| measure | 59.05 | 32.9 | render/thread/ms 40.44, render/busy/ms 39.20, main/busy/ms 26.59, render/graph/ms 12.66, bigspace/total/ms 1.37, render/present/ms 1.00, terrain/update/ms 0.89, controller/feed/ms 0.86, terrain/select/ms 0.78, stream/drive/ms 0.72, anim/evaluate_cpu/ms 0.65, bigspace/low_precision/ms 0.64 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_old.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lampsel_old\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **79%** (min 32%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.6 W · 76 °C (max 81) · VRAM 5285 MiB · throttled 100% of samples
