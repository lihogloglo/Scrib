# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5149 frames over 65.6 s · **median 12.39 ms (81 fps)** · mean 12.75 · p95 15.57 · p99 17.96 (1% low 56 fps) · worst 45.77 ms · 40 spikes over 1.5× median · 26.5 stutters/min (work 21.0/min)
- warm-up: 444 frames over 9.6 s, worst 2515.9 ms
- hitches (> 31.0 ms): **4** measured, 23 in warm-up
- timed GPU passes: 7.69 ms of a 12.75 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.39 ms, limit 11 ms
- **FAIL** frame_ms: 29 stutters (> 1.5x moving median of 60) in 1.09 min = 26.51/min, limit 1/min
- **FAIL** worst frame 45.77 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.132 | 2.202 | 2.675 | 0.97 | 0.181 |
| main_opaque_pass_3d | 1.701 | 1.756 | 2.581 | 0.97 | 0.061 |
| early prepass | 0.532 | 0.549 | 0.796 | 0.97 | 0.106 |
| bloom | 0.331 | 0.341 | 0.488 | 0.97 | 0.065 |
| atmosphere_luts | 0.324 | 0.334 | 0.592 | 0.97 | 0.003 |
| water_velocity | 0.295 | 0.305 | 0.511 | 0.97 | 0.001 |
| uniform_allocation | 0.254 | 0.262 | 0.266 | 0.97 | 0.046 |
| water_sspr | 0.216 | 0.223 | 0.222 | 0.97 | 0.002 |
| ssao | 0.202 | 0.208 | 0.212 | 0.97 | 0.007 |
| bin_unpacking | 0.194 | 0.200 | 0.212 | 0.97 | 0.030 |
| atmos_froxel_inject | 0.151 | 0.156 | 0.152 | 0.97 | 0.001 |
| auxiliary_views | 0.147 | 0.152 | 0.617 | 0.97 | 0.135 |
| water_fft | 0.137 | 0.142 | 0.140 | 0.97 | 0.003 |
| clouds_trace | 0.136 | 0.141 | 0.234 | 0.97 | 0.001 |
| upscaling | 0.127 | 0.131 | 0.158 | 0.97 | 0.004 |
| clouds_composite | 0.109 | 0.113 | 0.111 | 0.97 | 0.002 |
| tonemapping | 0.091 | 0.094 | 0.093 | 0.97 | 0.004 |
| main_transmissive_pass_3d | 0.090 | 0.093 | 0.172 | 0.97 | 0.008 |
| clustering | 0.065 | 0.067 | 0.069 | 0.97 | 0.020 |
| render_sky | 0.054 | 0.055 | 0.054 | 0.97 | 0.002 |
| clouds_resolve | 0.048 | 0.050 | 0.051 | 0.97 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.442 | 0.755 | 0.11 | 0.001 |
| surface_trail | 0.045 | 0.058 | 0.057 | 0.78 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.043 | 0.048 | 0.97 | 0.005 |
| water_snapshot | 0.037 | 0.038 | 0.045 | 0.97 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.97 | 0.003 |
| lightprobe_radiance_map | 0.026 | 0.240 | 0.239 | 0.11 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.026 | 0.030 | 0.97 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.022 | 0.023 | 0.97 | 0.006 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.033 | 0.97 | 0.107 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.97 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.97 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.97 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.97 | 0.001 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 30.906 | 0.000 | 11601.65 | 159136.0 |
| render/thread/ms | 11.573 | 14.232 | 44.78 | 59588.0 |
| render/busy/ms | 10.973 | 13.631 | 44.24 | 56501.0 |
| main/busy/ms | 9.033 | 11.626 | 24.80 | 46510.8 |
| render/graph/ms | 6.701 | 8.400 | 40.24 | 34501.9 |
| pipelines/compile_ms | 2.285 | 0.000 | 1879.32 | 11767.2 |
| pipelines/driver_ms | 1.724 | 0.000 | 1755.02 | 8877.9 |
| pipelines/front_ms | 0.561 | 0.000 | 177.52 | 2889.4 |
| render/present/ms | 0.509 | 0.648 | 13.81 | 2619.4 |
| bigspace/total/ms | 0.479 | 0.838 | 5.71 | 2466.7 |
| terrain/update/ms | 0.293 | 0.371 | 2.01 | 1506.8 |
| controller/feed/ms | 0.267 | 0.353 | 0.77 | 1372.4 |
| terrain/select/ms | 0.260 | 0.329 | 0.55 | 1338.2 |
| stream/drive/ms | 0.252 | 0.369 | 17.04 | 1298.2 |
| stream/tick/ms | 0.193 | 0.271 | 0.81 | 996.1 |
| bigspace/low_precision/ms | 0.191 | 0.343 | 3.70 | 986.0 |
| far/select/ms | 0.190 | 0.260 | 0.55 | 975.9 |
| anim/evaluate_cpu/ms | 0.157 | 0.194 | 0.51 | 807.3 |
| anim/evaluate/ms | 0.145 | 0.272 | 3.52 | 748.3 |
| controller/move/ms | 0.143 | 0.331 | 0.91 | 735.4 |
| bigspace/high_precision/ms | 0.124 | 0.250 | 5.07 | 640.4 |
| render/acquire/ms | 0.091 | 0.119 | 0.34 | 467.7 |
| physics/step/ms | 0.053 | 0.085 | 0.61 | 274.4 |
| physics/insert/ms | 0.050 | 0.433 | 0.85 | 255.8 |
| weather/map/ms | 0.024 | 0.033 | 0.21 | 124.0 |
| anim/apply/ms | 0.011 | 0.021 | 0.31 | 58.6 |
| controller/avoid/ms | 0.011 | 0.014 | 0.26 | 55.2 |
| weather/eval/ms | 0.011 | 0.014 | 0.07 | 54.1 |
| script/update/ms | 0.007 | 0.009 | 0.15 | 34.8 |
| stream/despawn/ms | 0.006 | 0.000 | 1.17 | 30.7 |
| script/ui/ms | 0.005 | 0.007 | 0.19 | 27.0 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 26.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 26.0 |
| controller/write/ms | 0.003 | 0.004 | 0.16 | 17.7 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.61 | 14.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.09 | 5.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 33 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 71 | 19 |
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 26 | 64 | 12 |
| crowd/c0/joints/count | count | 1306 | 2391 | 538 |
| crowd/c0/meshes/count | count | 355 | 633 | 129 |
| crowd/c1/count | count | 147 | 156 | 147 |
| crowd/c1/joints/count | count | 5507 | 5992 | 5594 |
| crowd/c1/meshes/count | count | 1177 | 1308 | 1196 |
| crowd/c2/count | count | 386 | 419 | 419 |
| crowd/c2/joints/count | count | 6398 | 6968 | 6968 |
| crowd/c2/meshes/count | count | 1822 | 1996 | 1996 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4272 | 8837 | 6179 |
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
| frame/work/ms | ms | 10.69 | 44.24 | 10.01 |
| gpu/timed/ms | ms | 8.61 | 11.80 | 9.27 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 808 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 808 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d/bins | count | 1290 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3396 | 5129 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1290 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3396 | 5129 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 56 | 304 | 47 |
| phases/shadow/bins | count | 3781 | 5374 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9572 | 12.7 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 140 | 314 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 867 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 32 | 0 |
| preprocess/bind_groups | count | 12 | 801 | 6 |
| preprocess/bind_groups_created | count | 12 | 801 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 255 | 359 | 251 |
| process/cpu_usage | % | 24.04 | 43.75 | 23.08 |
| process/mem_usage | GiB | 5.28 | 5.46 | 5.46 |
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
| started_load_count |  | 4766 | 4860 | 4860 |
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
| system/cpu_usage | % | 29.63 | 46.08 | 29.59 |
| system/mem_usage | % | 64.25 | 64.99 | 64.93 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2591 | 2644 | 2644 |
| views/all/batch_sets | count | 124 | 368 | 117 |
| views/all/bins | count | 6379 | 8130 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.0 k | 25.1 k | 13.3 k |
| views/all/pipelines | count | 202 | 374 | 113 |
| views/all/pipelines_distinct | count | 97 | 109 | 91 |
| views/all/work_items | count | 17.2 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 55 | 71 | 47 |
| views/cascade/bins | count | 2589 | 3149 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8142 | 10.1 k | 5663 |
| views/cascade/pipelines | count | 55 | 71 | 47 |
| views/cascade/work_items | count | 8142 | 10.1 k | 5663 |
| views/lamp_face/batch_sets | count | 0 | 244 | 0 |
| views/lamp_face/bins | count | 1066 | 2458 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1211 | 2803 | 0 |
| views/lamp_face/pipelines | count | 82 | 244 | 0 |
| views/lamp_face/work_items | count | 0 | 2571 | 0 |
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 2870 | 3560 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8400 | 13.0 k | 7592 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8567 | 13.2 k | 7900 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 53.38 | 1229 | 274.9 k |
| anim/events/count | count | 0.10 | 2 | 539 |
| assets/Image/added | count | 0.13 | 43 | 662 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.82 | 152 | 4233 |
| assets/Mesh/removed | count | 0.95 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 540 |
| assets/StandardMaterial/modified | count | 4.21 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.56 | 3051 | 13.2 k |
| physics/inserts | count | 0.55 | 107 | 2844 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.45 | 145 | 2335 |
| physics/steps | count | 0.77 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 21 | 378 |
| stream/upload/bytes | bytes | 6575 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1441 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.0 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2515.9 | main/busy/ms 2487, surface/advance_cpu_ms 67.38, surface/cell_upload_ms 0.71, stream/drive/ms 0.32, bigspace/total/ms 0.12, stream/tick/ms 0.10, weather/map/ms 0.10, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.03, script/ui/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.40 | 886.8 | main/busy/ms 2523, render/thread/ms 848, render/busy/ms 723, render/acquire/ms 123, surface/advance_cpu_ms 67.38, render/graph/ms 5.61, render/present/ms 2.15, stream/drive/ms 1.03, surface/cell_upload_ms 0.71, stream/tick/ms 0.51, terrain/update/ms 0.40, terrain/select/ms 0.27 |
| warm-up | 3.48 | 79.9 | render/thread/ms 860, pipelines/wait_ms 750, render/busy/ms 735, render/acquire/ms 123, pipelines/compile_ms 74.59, pipelines/front_ms 66.85, main/busy/ms 53.19, terrain/update/ms 19.30, render/graph/ms 9.57, pipelines/driver_ms 7.75, render/present/ms 3.00, stream/drive/ms 1.29 |
| warm-up | 3.73 | 107.8 | pipelines/compile_ms 226, pipelines/front_ms 210, render/thread/ms 71.87, render/busy/ms 69.91, main/busy/ms 67.65, pipelines/driver_ms 16.19, render/graph/ms 4.91, stream/drive/ms 2.00, stream/tick/ms 1.69, render/present/ms 1.66, terrain/update/ms 0.67, terrain/select/ms 0.55 |
| warm-up | 3.88 | 61.9 | main/busy/ms 43.47, render/thread/ms 42.58, render/busy/ms 41.35, stream/drive/ms 31.09, render/graph/ms 2.50, render/present/ms 1.01, stream/tick/ms 0.77, stream/spawn/ms 0.75, terrain/update/ms 0.75, terrain/select/ms 0.62, bigspace/total/ms 0.26, render/acquire/ms 0.22 |
| warm-up | 3.92 | 31.1 | pipelines/wait_ms 1526, pipelines/compile_ms 169, pipelines/front_ms 157, render/thread/ms 28.44, render/busy/ms 27.15, main/busy/ms 19.79, pipelines/driver_ms 12.90, stream/drive/ms 7.15, render/graph/ms 3.13, render/present/ms 1.07, stream/tick/ms 0.78, terrain/update/ms 0.76 |
| warm-up | 4.00 | 55.1 | pipelines/wait_ms 4493, pipelines/compile_ms 88.03, pipelines/front_ms 81.66, render/thread/ms 45.71, render/busy/ms 43.93, main/busy/ms 43.31, stream/drive/ms 17.94, pipelines/driver_ms 6.37, terrain/update/ms 4.92, render/graph/ms 4.87, stream/spawn/ms 2.29, render/present/ms 1.53 |
| warm-up | 4.07 | 44.9 | main/busy/ms 41.05, render/thread/ms 39.66, render/busy/ms 38.15, stream/drive/ms 27.16, render/graph/ms 5.62, render/present/ms 1.33, stream/tick/ms 0.99, terrain/update/ms 0.91, stream/spawn/ms 0.90, terrain/select/ms 0.72, stream/activate/ms 0.54, bigspace/total/ms 0.26 |
| warm-up | 4.16 | 37.5 | pipelines/wait_ms 3328, pipelines/compile_ms 175, pipelines/front_ms 156, render/thread/ms 52.78, render/busy/ms 49.42, main/busy/ms 18.80, render/graph/ms 18.77, pipelines/driver_ms 18.32, render/present/ms 3.12, stream/drive/ms 2.25, stream/tick/ms 1.09, terrain/update/ms 1.08 |
| warm-up | 4.20 | 46.7 | pipelines/wait_ms 3760, pipelines/compile_ms 209, pipelines/front_ms 188, render/thread/ms 68.09, render/busy/ms 64.76, main/busy/ms 25.85, pipelines/driver_ms 20.42, render/graph/ms 19.00, stream/drive/ms 5.72, stream/spawn/ms 3.55, render/present/ms 2.98, stream/tick/ms 1.04 |
| warm-up | 4.27 | 62.7 | pipelines/wait_ms 7112, pipelines/compile_ms 129, pipelines/front_ms 118, render/thread/ms 63.44, render/busy/ms 61.60, main/busy/ms 25.96, pipelines/driver_ms 11.48, render/graph/ms 7.66, stream/drive/ms 6.72, stream/spawn/ms 4.71, render/present/ms 1.54, stream/tick/ms 1.28 |
| warm-up | 4.31 | 43.7 | pipelines/wait_ms 6823, pipelines/compile_ms 91.01, pipelines/front_ms 81.77, main/busy/ms 65.36, render/thread/ms 52.46, render/busy/ms 51.29, stream/drive/ms 49.07, pipelines/driver_ms 9.24, render/graph/ms 3.14, stream/spawn/ms 2.18, stream/tick/ms 1.10, terrain/update/ms 1.07 |
| warm-up | 4.39 | 80.9 | pipelines/wait_ms 2510, render/thread/ms 72.70, main/busy/ms 66.36, stream/drive/ms 48.64, render/busy/ms 45.02, pipelines/compile_ms 35.74, render/present/ms 27.46, pipelines/driver_ms 27.39, pipelines/front_ms 8.35, render/graph/ms 4.14, stream/spawn/ms 2.23, terrain/update/ms 0.79 |
| warm-up | 4.49 | 74.6 | pipelines/wait_ms 14.6 k, pipelines/compile_ms 240, pipelines/front_ms 198, main/busy/ms 72.39, stream/drive/ms 51.25, pipelines/driver_ms 42.13, render/thread/ms 37.59, render/busy/ms 36.49, render/graph/ms 5.27, stream/spawn/ms 2.19, stream/activate/ms 1.06, terrain/update/ms 0.96 |
| warm-up | 4.56 | 47.2 | pipelines/wait_ms 3721, pipelines/compile_ms 74.33, pipelines/front_ms 64.80, main/busy/ms 49.68, render/thread/ms 39.71, render/busy/ms 37.72, stream/drive/ms 33.98, pipelines/driver_ms 9.52, render/graph/ms 8.53, render/present/ms 1.75, stream/tick/ms 1.44, stream/activate/ms 1.20 |
| warm-up | 4.63 | 57.0 | pipelines/wait_ms 3283, pipelines/compile_ms 96.78, pipelines/front_ms 66.82, main/busy/ms 44.15, render/thread/ms 40.90, stream/drive/ms 29.96, pipelines/driver_ms 29.95, render/busy/ms 22.05, render/present/ms 18.65, render/graph/ms 5.55, stream/spawn/ms 1.46, stream/tick/ms 1.16 |
| warm-up | 5.32 | 37.6 | main/busy/ms 40.52, render/thread/ms 32.76, render/busy/ms 31.14, stream/drive/ms 14.98, render/graph/ms 7.60, render/present/ms 1.30, stream/spawn/ms 0.95, stream/tick/ms 0.91, terrain/update/ms 0.73, terrain/select/ms 0.58, bigspace/total/ms 0.54, stream/activate/ms 0.41 |
| warm-up | 5.41 | 38.9 | pipelines/wait_ms 4844, render/thread/ms 51.44, render/busy/ms 42.84, pipelines/compile_ms 33.56, pipelines/front_ms 31.85, main/busy/ms 27.77, render/graph/ms 11.95, render/present/ms 8.12, stream/drive/ms 4.59, pipelines/driver_ms 1.72, stream/spawn/ms 1.50, bigspace/total/ms 1.08 |
| warm-up | 5.49 | 52.3 | pipelines/wait_ms 5260, pipelines/compile_ms 70.87, pipelines/front_ms 68.76, render/thread/ms 49.42, render/busy/ms 47.85, main/busy/ms 35.43, render/graph/ms 9.85, terrain/update/ms 4.01, stream/drive/ms 3.70, pipelines/driver_ms 2.10, stream/spawn/ms 2.04, render/present/ms 1.38 |
| warm-up | 5.95 | 35.0 | render/thread/ms 50.44, render/busy/ms 48.62, main/busy/ms 38.51, stream/drive/ms 17.86, render/graph/ms 8.46, stream/spawn/ms 1.93, render/present/ms 1.60, physics/insert/ms 0.95, stream/tick/ms 0.73, bigspace/total/ms 0.73, terrain/update/ms 0.65, terrain/select/ms 0.55 |
| warm-up | 7.16 | 31.6 | pipelines/wait_ms 3083, render/thread/ms 40.23, main/busy/ms 40.15, render/busy/ms 38.82, pipelines/compile_ms 22.37, pipelines/front_ms 21.27, render/graph/ms 13.85, bigspace/total/ms 2.30, bigspace/high_precision/ms 1.60, stream/drive/ms 1.47, render/present/ms 1.14, pipelines/driver_ms 1.09 |
| warm-up | 7.24 | 37.2 | main/busy/ms 39.91, render/thread/ms 38.16, render/busy/ms 36.72, render/graph/ms 11.99, render/present/ms 1.25, stream/drive/ms 1.02, physics/insert/ms 0.68, bigspace/total/ms 0.59, terrain/update/ms 0.56, terrain/select/ms 0.49, stream/tick/ms 0.48, far/select/ms 0.35 |
| warm-up | 8.31 | 37.4 | main/busy/ms 50.34, render/thread/ms 35.34, render/busy/ms 33.96, render/graph/ms 8.58, stream/drive/ms 1.22, render/present/ms 1.16, physics/insert/ms 0.96, anim/evaluate_cpu/ms 0.92, physics/step/ms 0.80, bigspace/total/ms 0.71, stream/tick/ms 0.61, terrain/update/ms 0.54 |
| measure | 1.02 | 45.8 | render/thread/ms 56.57, render/busy/ms 55.46, render/graph/ms 47.11, main/busy/ms 20.94, render/present/ms 0.94, physics/insert/ms 0.92, bigspace/total/ms 0.81, stream/drive/ms 0.60, terrain/update/ms 0.58, stream/tick/ms 0.53, terrain/select/ms 0.52, controller/feed/ms 0.49 |
| measure | 2.73 | 34.2 | render/thread/ms 45.14, render/busy/ms 43.79, render/graph/ms 22.58, main/busy/ms 20.18, render/present/ms 1.17, bigspace/total/ms 0.79, physics/insert/ms 0.58, terrain/update/ms 0.55, terrain/select/ms 0.49, controller/feed/ms 0.42, stream/drive/ms 0.42, stream/tick/ms 0.37 |
| measure | 19.28 | 31.4 | render/thread/ms 43.10, render/busy/ms 41.86, render/graph/ms 24.99, main/busy/ms 18.97, stream/drive/ms 1.18, render/present/ms 1.07, stream/tick/ms 0.99, bigspace/total/ms 0.74, terrain/update/ms 0.69, terrain/select/ms 0.63, controller/feed/ms 0.55, far/select/ms 0.35 |
| measure | 23.64 | 31.5 | render/thread/ms 42.71, render/busy/ms 41.44, render/graph/ms 31.53, main/busy/ms 20.57, render/present/ms 1.08, physics/insert/ms 0.84, bigspace/total/ms 0.76, terrain/update/ms 0.58, controller/feed/ms 0.54, terrain/select/ms 0.52, stream/drive/ms 0.39, stream/tick/ms 0.34 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_old.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lampsel_old\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 9%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **80%** (min 56%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.0 W · 76 °C (max 81) · VRAM 5289 MiB · throttled 100% of samples
