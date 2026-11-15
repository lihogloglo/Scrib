# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5000 frames over 65.6 s · **median 12.28 ms (81 fps)** · mean 13.13 · p95 17.53 · p99 20.29 (1% low 49 fps) · worst 35.65 ms · 144 spikes over 1.5× median · 24.7 stutters/min (work 23.8/min)
- warm-up: 344 frames over 8.8 s, worst 2614.9 ms
- hitches (> 30.7 ms): **3** measured, 18 in warm-up
- timed GPU passes: 7.49 ms of a 13.13 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.28 ms, limit 11 ms
- **FAIL** frame_ms: 27 stutters (> 1.5x moving median of 60) in 1.09 min = 24.68/min, limit 1/min
- **FAIL** worst frame 35.65 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.075 | 2.088 | 2.241 | 0.99 | 0.186 |
| main_opaque_pass_3d | 1.381 | 1.389 | 1.720 | 0.99 | 0.071 |
| early prepass | 0.524 | 0.527 | 0.738 | 0.99 | 0.108 |
| auxiliary_views | 0.346 | 0.348 | 1.202 | 0.99 | 0.306 |
| bloom | 0.321 | 0.323 | 0.329 | 0.99 | 0.068 |
| atmosphere_luts | 0.318 | 0.320 | 0.382 | 0.99 | 0.003 |
| water_velocity | 0.269 | 0.270 | 0.358 | 0.99 | 0.002 |
| tonemapping | 0.261 | 0.263 | 0.275 | 0.99 | 0.004 |
| upscaling | 0.216 | 0.218 | 0.228 | 0.99 | 0.004 |
| water_sspr | 0.210 | 0.212 | 0.211 | 0.99 | 0.002 |
| uniform_allocation | 0.205 | 0.207 | 0.218 | 0.99 | 0.038 |
| ssao | 0.193 | 0.194 | 0.196 | 0.99 | 0.008 |
| bin_unpacking | 0.168 | 0.169 | 0.178 | 0.99 | 0.027 |
| water_fft | 0.132 | 0.132 | 0.139 | 0.99 | 0.003 |
| atmos_froxel_inject | 0.130 | 0.131 | 0.130 | 0.99 | 0.002 |
| clouds_composite | 0.107 | 0.107 | 0.110 | 0.99 | 0.002 |
| main_transmissive_pass_3d | 0.079 | 0.079 | 0.144 | 0.99 | 0.009 |
| clustering | 0.065 | 0.065 | 0.069 | 0.99 | 0.021 |
| clouds_trace | 0.055 | 0.055 | 0.075 | 0.99 | 0.001 |
| render_sky | 0.055 | 0.055 | 0.057 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.422 | 0.422 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.051 | 0.052 | 0.99 | 0.001 |
| surface_trail | 0.046 | 0.057 | 0.057 | 0.81 | 0.002 |
| water_snapshot | 0.041 | 0.042 | 0.048 | 0.99 | 0.002 |
| early_mesh_preprocessing | 0.034 | 0.034 | 0.038 | 0.99 | 0.005 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.227 | 0.234 | 0.12 | 0.005 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.019 | 0.019 | 0.033 | 0.99 | 0.115 |
| night_sky | 0.016 | 0.016 | 0.035 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.014 | 0.015 | 0.015 | 0.99 | 0.005 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 45.847 | 0.000 | 31226.66 | 229236.9 |
| render/thread/ms | 11.878 | 16.095 | 34.51 | 59390.2 |
| render/busy/ms | 11.282 | 15.423 | 33.70 | 56412.4 |
| main/busy/ms | 9.588 | 12.444 | 30.76 | 47938.5 |
| render/graph/ms | 6.823 | 9.757 | 16.26 | 34114.4 |
| pipelines/compile_ms | 0.762 | 0.000 | 174.22 | 3810.4 |
| pipelines/front_ms | 0.734 | 0.000 | 167.22 | 3668.4 |
| render/present/ms | 0.502 | 0.629 | 5.18 | 2508.6 |
| bigspace/total/ms | 0.490 | 0.819 | 4.91 | 2450.0 |
| terrain/update/ms | 0.306 | 0.393 | 1.85 | 1531.6 |
| controller/feed/ms | 0.278 | 0.365 | 0.83 | 1388.2 |
| terrain/select/ms | 0.272 | 0.348 | 1.08 | 1362.0 |
| stream/drive/ms | 0.249 | 0.349 | 22.24 | 1245.7 |
| far/select/ms | 0.199 | 0.282 | 0.57 | 992.9 |
| bigspace/low_precision/ms | 0.193 | 0.298 | 3.44 | 963.4 |
| stream/tick/ms | 0.189 | 0.260 | 0.58 | 942.9 |
| anim/evaluate_cpu/ms | 0.162 | 0.213 | 0.60 | 809.4 |
| anim/evaluate/ms | 0.152 | 0.278 | 3.07 | 757.7 |
| bigspace/high_precision/ms | 0.123 | 0.225 | 4.30 | 616.2 |
| controller/move/ms | 0.118 | 0.214 | 0.43 | 591.1 |
| render/acquire/ms | 0.094 | 0.123 | 4.05 | 469.1 |
| physics/insert/ms | 0.070 | 0.495 | 1.16 | 348.4 |
| physics/step/ms | 0.059 | 0.095 | 0.66 | 295.1 |
| pipelines/driver_ms | 0.028 | 0.000 | 9.41 | 142.0 |
| weather/map/ms | 0.015 | 0.021 | 0.05 | 77.0 |
| anim/apply/ms | 0.011 | 0.022 | 0.19 | 56.0 |
| controller/avoid/ms | 0.011 | 0.014 | 0.09 | 52.9 |
| weather/eval/ms | 0.010 | 0.013 | 0.23 | 48.1 |
| script/update/ms | 0.007 | 0.010 | 0.04 | 36.7 |
| stream/despawn/ms | 0.006 | 0.000 | 0.90 | 28.5 |
| script/input/ms | 0.005 | 0.007 | 0.06 | 27.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 26.0 |
| script/ui/ms | 0.005 | 0.007 | 0.07 | 25.3 |
| controller/write/ms | 0.003 | 0.005 | 0.08 | 17.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.64 | 14.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.13 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 569 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 52 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 26 | 42 | 13 |
| crowd/c0/joints/count | count | 1272 | 2218 | 604 |
| crowd/c0/meshes/count | count | 352 | 634 | 144 |
| crowd/c1/count | count | 148 | 155 | 148 |
| crowd/c1/joints/count | count | 5547 | 5937 | 5592 |
| crowd/c1/meshes/count | count | 1180 | 1281 | 1197 |
| crowd/c2/count | count | 399 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 159 | 215 | 200 |
| crowd/joints_extracted/count | count | 3677 | 8741 | 5513 |
| crowd/joints_uploaded/count | count | 3573 | 6129 | 5513 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.59 | 33.70 | 8.86 |
| gpu/timed/ms | ms | 8.16 | 11.38 | 8.86 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2523 | 2580 | 2580 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 278.4 k | 278.4 k | 278.4 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 791 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 791 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 28 | 30 | 29 |
| phases/opaque3d/bins | count | 1291 | 1399 | 1349 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3337 | 5155 | 3181 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 28 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 30 | 29 |
| phases/opaque3d_prepass/bins | count | 1291 | 1399 | 1349 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3337 | 5155 | 3181 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 28 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 34 | 251 | 28 |
| phases/shadow/bins | count | 3222 | 4272 | 1120 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 4802 | 7268 | 2417 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 224 | 291 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2630 | 3268 | 2346 |
| physics/items | count | 137 | 476 | 19 |
| physics/pending | count | 0 | 1483 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 223 | 228 | 228 |
| pipelines/waiting | count | 0 | 49 | 0 |
| preprocess/bind_groups | count | 10 | 702 | 6 |
| preprocess/bind_groups_created | count | 10 | 702 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 220 | 383 | 216 |
| process/cpu_usage | % | 24.04 | 45.67 | 18.36 |
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
| started_load_count |  | 4766 | 4856 | 4856 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 431 | 528 | 396 |
| statics/materials_bound | count | 431 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.01 | 73.39 | 28.83 |
| system/mem_usage | % | 63.79 | 64.21 | 64.10 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2821 | 2936 | 2936 |
| textures/images | count | 2589 | 2647 | 2647 |
| views/all/batch_sets | count | 104 | 319 | 100 |
| views/all/bins | count | 5970 | 7148 | 4612 |
| views/all/count | count | 45 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 13.1 k | 19.5 k | 10.0 k |
| views/all/pipelines | count | 292 | 357 | 96 |
| views/all/pipelines_distinct | count | 101 | 109 | 91 |
| views/all/work_items | count | 10.9 k | 17.5 k | 10.3 k |
| views/cascade/batch_sets | count | 32 | 40 | 28 |
| views/cascade/bins | count | 1174 | 1517 | 1120 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2731 | 4278 | 2417 |
| views/cascade/pipelines | count | 32 | 40 | 28 |
| views/cascade/work_items | count | 2731 | 4278 | 2417 |
| views/lamp_face/batch_sets | count | 0 | 216 | 0 |
| views/lamp_face/bins | count | 2212 | 2831 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2424 | 3205 | 0 |
| views/lamp_face/pipelines | count | 197 | 253 | 0 |
| views/lamp_face/work_items | count | 0 | 2796 | 0 |
| views/main/batch_sets | count | 70 | 76 | 72 |
| views/main/bins | count | 2882 | 3574 | 3492 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8240 | 13.1 k | 7596 |
| views/main/pipelines | count | 66 | 72 | 68 |
| views/main/work_items | count | 8406 | 13.3 k | 7904 |
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
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 50.03 | 260 | 250.1 k |
| anim/events/count | count | 0.11 | 3 | 548 |
| assets/Image/added | count | 0.13 | 42 | 637 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.84 | 152 | 4183 |
| assets/Mesh/removed | count | 0.98 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 508 |
| assets/StandardMaterial/modified | count | 4.34 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 394 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.70 | 1485 | 8514 |
| physics/inserts | count | 0.67 | 110 | 3352 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.47 | 145 | 2327 |
| physics/steps | count | 0.79 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 15 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 17 | 378 |
| stream/upload/bytes | bytes | 6770 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1484 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.7 ms; slowest 21, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.61 | 2614.9 | main/busy/ms 2590, surface/advance_cpu_ms 0.72, stream/drive/ms 0.34, surface/cell_upload_ms 0.28, bigspace/total/ms 0.12, stream/tick/ms 0.09, bigspace/high_precision/ms 0.04, weather/map/ms 0.04, script/input/ms 0.03, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.01 |
| warm-up | 3.49 | 875.7 | main/busy/ms 2626, render/thread/ms 841, render/busy/ms 712, render/acquire/ms 126, render/graph/ms 5.48, render/present/ms 2.07, stream/drive/ms 1.04, surface/advance_cpu_ms 0.72, terrain/update/ms 0.61, stream/tick/ms 0.52, terrain/select/ms 0.42, bigspace/total/ms 0.37 |
| warm-up | 3.57 | 82.2 | pipelines/wait_ms 1047, render/thread/ms 851, render/busy/ms 721, render/acquire/ms 127, pipelines/compile_ms 91.87, pipelines/front_ms 83.60, main/busy/ms 50.33, terrain/update/ms 20.04, render/graph/ms 8.95, pipelines/driver_ms 8.28, render/present/ms 3.10, stream/drive/ms 1.23 |
| warm-up | 3.81 | 83.8 | main/busy/ms 59.63, render/thread/ms 41.57, render/busy/ms 39.60, render/graph/ms 3.46, render/present/ms 1.72, stream/drive/ms 1.28, stream/tick/ms 1.01, terrain/update/ms 0.87, terrain/select/ms 0.70, bigspace/total/ms 0.27, render/acquire/ms 0.25, bigspace/high_precision/ms 0.13 |
| warm-up | 3.95 | 42.0 | render/thread/ms 42.68, render/busy/ms 41.38, main/busy/ms 37.90, stream/drive/ms 25.50, render/graph/ms 2.80, render/present/ms 1.09, stream/tick/ms 0.92, terrain/update/ms 0.85, terrain/select/ms 0.73, stream/spawn/ms 0.31, bigspace/total/ms 0.24, render/acquire/ms 0.21 |
| warm-up | 4.03 | 67.0 | pipelines/wait_ms 662, pipelines/compile_ms 215, pipelines/front_ms 203, render/thread/ms 46.89, render/busy/ms 45.05, main/busy/ms 41.49, stream/drive/ms 26.53, pipelines/driver_ms 11.34, render/graph/ms 5.58, render/present/ms 1.53, stream/spawn/ms 1.38, stream/tick/ms 0.75 |
| warm-up | 4.12 | 71.9 | render/thread/ms 55.12, render/busy/ms 53.37, main/busy/ms 34.46, stream/drive/ms 13.24, render/graph/ms 2.97, stream/tick/ms 2.24, render/present/ms 1.55, stream/spawn/ms 1.12, terrain/update/ms 1.02, terrain/select/ms 0.87, bigspace/total/ms 0.33, stream/activate/ms 0.32 |
| warm-up | 4.18 | 51.7 | render/thread/ms 76.60, render/busy/ms 57.91, main/busy/ms 45.38, stream/drive/ms 28.70, render/present/ms 18.48, render/graph/ms 2.50, stream/tick/ms 1.38, terrain/update/ms 1.06, stream/activate/ms 0.85, terrain/select/ms 0.85, bigspace/total/ms 0.34, physics/insert/ms 0.31 |
| warm-up | 4.43 | 127.8 | pipelines/wait_ms 2590, pipelines/compile_ms 179, pipelines/front_ms 172, render/thread/ms 76.14, main/busy/ms 71.74, stream/drive/ms 56.88, render/busy/ms 49.43, render/present/ms 26.39, pipelines/driver_ms 6.64, render/graph/ms 5.60, stream/tick/ms 1.78, terrain/update/ms 0.95 |
| warm-up | 4.46 | 31.7 | pipelines/wait_ms 3983, pipelines/compile_ms 297, pipelines/front_ms 277, render/thread/ms 85.33, render/busy/ms 57.92, main/busy/ms 51.45, stream/drive/ms 36.79, render/present/ms 27.11, pipelines/driver_ms 20.22, render/graph/ms 16.46, stream/spawn/ms 1.09, stream/tick/ms 1.07 |
| warm-up | 4.51 | 47.6 | pipelines/wait_ms 8311, pipelines/compile_ms 191, pipelines/front_ms 156, main/busy/ms 54.05, render/thread/ms 52.56, render/busy/ms 49.84, stream/drive/ms 37.30, pipelines/driver_ms 35.69, render/graph/ms 16.46, render/present/ms 2.44, stream/spawn/ms 2.14, terrain/update/ms 0.88 |
| warm-up | 4.58 | 47.8 | pipelines/wait_ms 4442, pipelines/compile_ms 72.82, pipelines/front_ms 69.66, main/busy/ms 53.31, render/thread/ms 44.27, render/busy/ms 41.83, stream/drive/ms 36.23, render/graph/ms 14.40, pipelines/driver_ms 3.16, render/present/ms 2.27, stream/spawn/ms 2.23, stream/tick/ms 0.89 |
| warm-up | 4.64 | 39.7 | pipelines/wait_ms 3752, main/busy/ms 45.49, pipelines/compile_ms 30.22, stream/drive/ms 29.65, pipelines/front_ms 28.32, render/thread/ms 25.04, render/busy/ms 23.72, render/graph/ms 6.02, pipelines/driver_ms 1.90, stream/spawn/ms 1.33, render/present/ms 1.17, stream/activate/ms 1.05 |
| warm-up | 5.16 | 38.8 | pipelines/wait_ms 15.0 k, pipelines/compile_ms 56.34, pipelines/front_ms 49.43, main/busy/ms 41.37, render/thread/ms 25.91, render/busy/ms 24.27, stream/drive/ms 13.94, pipelines/driver_ms 6.92, render/graph/ms 5.80, render/present/ms 1.38, physics/insert/ms 1.12, stream/tick/ms 0.94 |
| warm-up | 5.77 | 32.1 | pipelines/wait_ms 3354, pipelines/compile_ms 243, pipelines/front_ms 229, main/busy/ms 32.72, render/thread/ms 29.02, render/busy/ms 27.89, stream/drive/ms 14.62, pipelines/driver_ms 13.53, render/graph/ms 4.81, stream/spawn/ms 2.09, stream/tick/ms 0.96, render/present/ms 0.92 |
| warm-up | 6.81 | 40.2 | pipelines/wait_ms 5496, render/thread/ms 46.17, render/busy/ms 44.28, main/busy/ms 39.58, pipelines/compile_ms 16.15, pipelines/front_ms 15.09, render/graph/ms 11.63, render/present/ms 1.57, stream/drive/ms 1.36, pipelines/driver_ms 1.06, bigspace/total/ms 0.70, stream/tick/ms 0.66 |
| warm-up | 6.87 | 41.8 | main/busy/ms 62.66, render/thread/ms 38.25, render/busy/ms 36.66, render/graph/ms 14.13, bigspace/total/ms 1.40, render/present/ms 1.34, stream/drive/ms 1.21, anim/evaluate_cpu/ms 1.11, terrain/update/ms 0.60, stream/tick/ms 0.54, terrain/select/ms 0.50, bigspace/low_precision/ms 0.47 |
| warm-up | 6.90 | 32.8 | pipelines/wait_ms 2631, pipelines/compile_ms 145, pipelines/front_ms 141, main/busy/ms 38.34, render/thread/ms 37.96, render/busy/ms 36.60, render/graph/ms 7.43, pipelines/driver_ms 4.37, bigspace/total/ms 1.29, stream/drive/ms 1.26, render/present/ms 1.09, anim/evaluate_cpu/ms 0.94 |
| measure | 2.73 | 35.7 | pipelines/wait_ms 31.2 k, pipelines/compile_ms 159, pipelines/front_ms 149, render/thread/ms 46.44, render/busy/ms 45.08, render/graph/ms 23.30, main/busy/ms 19.35, pipelines/driver_ms 9.41, render/present/ms 1.18, physics/insert/ms 0.89, controller/feed/ms 0.74, bigspace/total/ms 0.68 |
| measure | 18.20 | 32.7 | main/busy/ms 42.60, render/thread/ms 27.88, render/busy/ms 26.77, stream/drive/ms 22.48, render/graph/ms 16.20, render/present/ms 0.93, bigspace/total/ms 0.82, controller/feed/ms 0.59, terrain/update/ms 0.58, terrain/select/ms 0.52, stream/tick/ms 0.48, far/select/ms 0.37 |
| measure | 59.05 | 32.6 | render/thread/ms 42.08, render/busy/ms 40.77, main/busy/ms 27.39, render/graph/ms 12.29, bigspace/total/ms 1.34, terrain/update/ms 1.17, render/present/ms 1.04, controller/feed/ms 1.01, terrain/select/ms 0.98, stream/drive/ms 0.95, stream/tick/ms 0.83, bigspace/low_precision/ms 0.61 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_new.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lampsel_new\balmora_walk_night@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **72%** (min 53%) · 2580 MHz (min 2580) · memory 8001 MHz · 70.4 W · 73 °C (max 80) · VRAM 5576 MiB · throttled 100% of samples
