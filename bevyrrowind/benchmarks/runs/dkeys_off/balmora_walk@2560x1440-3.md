# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4693 frames over 65.6 s · **median 13.59 ms (74 fps)** · mean 13.99 · p95 17.52 · p99 20.23 (1% low 49 fps) · worst 34.68 ms · 44 spikes over 1.5× median · 18.3 stutters/min (work 15.5/min)
- warm-up: 529 frames over 11.2 s, worst 2753.3 ms
- hitches (> 34.0 ms): **1** measured, 17 in warm-up
- timed GPU passes: 7.57 ms of a 13.99 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.59 ms, limit 11 ms
- **FAIL** frame_ms: 20 stutters (> 1.5x moving median of 60) in 1.09 min = 18.28/min, limit 1/min
- **FAIL** worst frame 34.68 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.092 | 2.095 | 2.329 | 1.00 | 0.181 |
| main_opaque_pass_3d | 1.677 | 1.679 | 2.050 | 1.00 | 0.077 |
| early prepass | 0.524 | 0.525 | 0.732 | 1.00 | 0.107 |
| bloom | 0.323 | 0.324 | 0.327 | 1.00 | 0.066 |
| atmosphere_luts | 0.318 | 0.318 | 0.381 | 1.00 | 0.004 |
| water_velocity | 0.281 | 0.282 | 0.354 | 1.00 | 0.002 |
| uniform_allocation | 0.254 | 0.254 | 0.264 | 1.00 | 0.055 |
| water_sspr | 0.211 | 0.211 | 0.211 | 1.00 | 0.003 |
| bin_unpacking | 0.200 | 0.200 | 0.213 | 1.00 | 0.035 |
| ssao | 0.194 | 0.195 | 0.196 | 1.00 | 0.008 |
| atmos_froxel_inject | 0.150 | 0.150 | 0.150 | 1.00 | 0.002 |
| auxiliary_views | 0.142 | 0.142 | 0.584 | 1.00 | 0.146 |
| clouds_trace | 0.140 | 0.140 | 0.228 | 1.00 | 0.001 |
| water_fft | 0.135 | 0.135 | 0.140 | 1.00 | 0.003 |
| upscaling | 0.122 | 0.123 | 0.129 | 1.00 | 0.004 |
| clouds_composite | 0.108 | 0.109 | 0.111 | 1.00 | 0.002 |
| tonemapping | 0.087 | 0.087 | 0.090 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.086 | 0.086 | 0.166 | 1.00 | 0.009 |
| clustering | 0.066 | 0.066 | 0.069 | 1.00 | 0.021 |
| render_sky | 0.054 | 0.054 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.053 | 0.426 | 0.422 | 0.13 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.049 | 0.057 | 0.057 | 0.86 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.048 | 1.00 | 0.007 |
| water_snapshot | 0.032 | 0.032 | 0.035 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.029 | 0.229 | 0.234 | 0.13 | 0.005 |
| atmos_froxel_integrate | 0.026 | 0.026 | 0.029 | 1.00 | 0.001 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.031 | 1.00 | 0.111 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 12.727 | 16.011 | 33.59 | 59728.3 |
| pipelines/wait_ms | 12.471 | 0.000 | 11754.13 | 58527.7 |
| render/busy/ms | 12.143 | 15.339 | 32.94 | 56985.8 |
| main/busy/ms | 9.927 | 12.904 | 28.43 | 46589.4 |
| render/graph/ms | 7.496 | 9.599 | 22.70 | 35178.0 |
| render/present/ms | 0.494 | 0.592 | 5.07 | 2316.2 |
| bigspace/total/ms | 0.479 | 0.709 | 6.44 | 2248.9 |
| pipelines/compile_ms | 0.407 | 0.000 | 161.74 | 1912.1 |
| pipelines/front_ms | 0.395 | 0.000 | 156.39 | 1852.2 |
| terrain/update/ms | 0.296 | 0.382 | 1.80 | 1386.9 |
| controller/feed/ms | 0.293 | 0.382 | 0.75 | 1373.6 |
| terrain/select/ms | 0.262 | 0.338 | 0.76 | 1228.5 |
| stream/drive/ms | 0.254 | 0.362 | 20.40 | 1193.2 |
| far/select/ms | 0.206 | 0.295 | 0.61 | 968.8 |
| stream/tick/ms | 0.192 | 0.269 | 0.66 | 901.3 |
| bigspace/low_precision/ms | 0.189 | 0.295 | 4.15 | 887.0 |
| anim/evaluate_cpu/ms | 0.169 | 0.216 | 0.48 | 792.6 |
| controller/move/ms | 0.144 | 0.292 | 0.57 | 674.1 |
| anim/evaluate/ms | 0.141 | 0.285 | 3.78 | 661.0 |
| bigspace/high_precision/ms | 0.112 | 0.203 | 5.99 | 525.3 |
| render/acquire/ms | 0.091 | 0.119 | 0.27 | 426.4 |
| physics/step/ms | 0.062 | 0.095 | 0.45 | 289.5 |
| physics/insert/ms | 0.046 | 0.432 | 0.85 | 216.9 |
| weather/map/ms | 0.025 | 0.033 | 0.19 | 115.4 |
| pipelines/driver_ms | 0.013 | 0.000 | 5.36 | 59.9 |
| anim/apply/ms | 0.012 | 0.022 | 0.12 | 57.1 |
| controller/avoid/ms | 0.011 | 0.015 | 0.20 | 52.0 |
| weather/eval/ms | 0.010 | 0.014 | 0.07 | 47.2 |
| script/update/ms | 0.008 | 0.010 | 0.15 | 35.5 |
| stream/despawn/ms | 0.007 | 0.000 | 1.29 | 30.6 |
| script/input/ms | 0.006 | 0.007 | 0.07 | 26.6 |
| stream/spawn/ms | 0.006 | 0.000 | 1.13 | 25.9 |
| script/ui/ms | 0.005 | 0.007 | 0.09 | 22.4 |
| controller/write/ms | 0.004 | 0.005 | 0.14 | 17.5 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.73 | 14.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.04 | 1.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 21 | 18 |
| crowd/c0/count | count | 26 | 42 | 13 |
| crowd/c0/joints/count | count | 1258 | 2204 | 583 |
| crowd/c0/meshes/count | count | 348 | 634 | 162 |
| crowd/c1/count | count | 146 | 155 | 148 |
| crowd/c1/joints/count | count | 5539 | 5949 | 5613 |
| crowd/c1/meshes/count | count | 1176 | 1281 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4273 | 9951 | 6179 |
| crowd/joints_uploaded/count | count | 4249 | 6643 | 6179 |
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
| frame/work/ms | ms | 11.83 | 32.94 | 11.12 |
| gpu/timed/ms | ms | 8.21 | 11.50 | 8.43 |
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
| phases/alpha_mask3d/items | count | 803 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 803 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 28 | 30 | 28 |
| phases/opaque3d/bins | count | 1294 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3381 | 5128 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1294 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3381 | 5128 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 59 | 272 | 47 |
| phases/shadow/bins | count | 4805 | 5759 | 2220 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.5 k | 13.2 k | 5671 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 250 | 308 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 231 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 19 | 0 |
| preprocess/bind_groups | count | 12 | 741 | 6 |
| preprocess/bind_groups_created | count | 12 | 741 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 256 | 376 | 251 |
| process/cpu_usage | % | 23.70 | 49.77 | 24.41 |
| process/mem_usage | GiB | 5.12 | 5.28 | 5.28 |
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
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.17 | 59.16 | 28.65 |
| system/mem_usage | % | 71.30 | 71.73 | 71.72 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 127 | 338 | 117 |
| views/all/bins | count | 7359 | 8638 | 5700 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.5 k | 25.7 k | 13.3 k |
| views/all/pipelines | count | 315 | 369 | 113 |
| views/all/pipelines_distinct | count | 102 | 108 | 91 |
| views/all/work_items | count | 17.2 k | 23.0 k | 13.6 k |
| views/cascade/batch_sets | count | 56 | 73 | 47 |
| views/cascade/bins | count | 2600 | 3171 | 2220 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8140 | 10.2 k | 5671 |
| views/cascade/pipelines | count | 56 | 73 | 47 |
| views/cascade/work_items | count | 8140 | 10.2 k | 5671 |
| views/lamp_face/batch_sets | count | 0 | 216 | 0 |
| views/lamp_face/bins | count | 2143 | 2790 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2394 | 3084 | 0 |
| views/lamp_face/pipelines | count | 193 | 246 | 0 |
| views/lamp_face/work_items | count | 0 | 2830 | 0 |
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 2888 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8368 | 13.0 k | 7594 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8535 | 13.2 k | 7902 |
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
| anim/bones_written/count | count | 49.93 | 231 | 234.3 k |
| anim/events/count | count | 0.12 | 3 | 576 |
| assets/Image/added | count | 0.13 | 43 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.87 | 152 | 4075 |
| assets/Mesh/removed | count | 1.04 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 466 |
| assets/StandardMaterial/modified | count | 4.62 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.01 | 1485 | 4718 |
| physics/inserts | count | 0.53 | 111 | 2493 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.50 | 145 | 2335 |
| physics/steps | count | 0.84 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 26 | 378 |
| stream/upload/bytes | bytes | 7213 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1581 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 34.0 ms; slowest 18, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.75 | 2753.3 | main/busy/ms 2726, surface/advance_cpu_ms 83.35, surface/cell_upload_ms 0.75, stream/drive/ms 0.35, weather/map/ms 0.11, stream/tick/ms 0.11, bigspace/total/ms 0.10, bigspace/high_precision/ms 0.07, weather/eval/ms 0.04, script/update/ms 0.03, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.61 | 860.6 | main/busy/ms 2761, render/thread/ms 825, render/busy/ms 654, render/acquire/ms 169, surface/advance_cpu_ms 83.35, render/graph/ms 5.73, render/present/ms 2.04, stream/drive/ms 1.16, surface/cell_upload_ms 0.75, stream/tick/ms 0.58, terrain/update/ms 0.38, bigspace/total/ms 0.28 |
| warm-up | 3.68 | 66.2 | render/thread/ms 835, render/busy/ms 663, pipelines/wait_ms 429, render/acquire/ms 170, pipelines/compile_ms 73.14, pipelines/front_ms 66.86, main/busy/ms 51.61, terrain/update/ms 19.00, render/graph/ms 9.51, pipelines/driver_ms 6.28, render/present/ms 3.00, stream/drive/ms 1.44 |
| warm-up | 3.92 | 84.8 | pipelines/compile_ms 231, pipelines/front_ms 213, main/busy/ms 64.49, render/thread/ms 50.92, render/busy/ms 49.14, pipelines/driver_ms 18.15, render/graph/ms 3.33, stream/drive/ms 2.31, stream/tick/ms 1.91, render/present/ms 1.46, terrain/update/ms 0.77, terrain/select/ms 0.62 |
| warm-up | 4.04 | 47.5 | main/busy/ms 35.32, render/thread/ms 23.68, stream/drive/ms 22.77, render/busy/ms 22.50, render/graph/ms 2.73, render/present/ms 0.97, terrain/update/ms 0.74, stream/tick/ms 0.67, terrain/select/ms 0.61, stream/spawn/ms 0.38, bigspace/total/ms 0.26, render/acquire/ms 0.22 |
| warm-up | 4.12 | 71.5 | pipelines/wait_ms 1041, pipelines/compile_ms 184, pipelines/front_ms 167, render/thread/ms 52.15, render/busy/ms 50.68, main/busy/ms 39.73, stream/drive/ms 25.73, pipelines/driver_ms 16.79, render/graph/ms 4.45, stream/spawn/ms 1.24, render/present/ms 1.19, stream/tick/ms 1.16 |
| warm-up | 4.20 | 67.6 | pipelines/wait_ms 2423, pipelines/compile_ms 113, pipelines/front_ms 106, render/thread/ms 39.65, render/busy/ms 38.44, main/busy/ms 28.05, stream/drive/ms 10.35, pipelines/driver_ms 6.60, render/graph/ms 2.78, stream/spawn/ms 1.80, stream/tick/ms 1.67, render/present/ms 1.00 |
| warm-up | 4.24 | 40.4 | pipelines/wait_ms 2929, pipelines/compile_ms 114, pipelines/front_ms 107, main/busy/ms 58.57, render/thread/ms 57.50, render/busy/ms 56.15, stream/drive/ms 36.09, pipelines/driver_ms 6.76, render/graph/ms 3.65, stream/spawn/ms 1.86, stream/tick/ms 1.83, render/present/ms 1.11 |
| warm-up | 4.30 | 55.8 | pipelines/wait_ms 506, main/busy/ms 59.92, render/thread/ms 50.47, render/busy/ms 49.22, stream/drive/ms 35.47, terrain/update/ms 4.61, render/graph/ms 3.60, stream/spawn/ms 2.09, pipelines/compile_ms 1.12, render/present/ms 0.98, stream/tick/ms 0.96, pipelines/front_ms 0.96 |
| warm-up | 4.46 | 37.6 | pipelines/wait_ms 1960, pipelines/compile_ms 236, pipelines/front_ms 221, main/busy/ms 67.33, stream/drive/ms 48.88, render/thread/ms 44.35, render/busy/ms 42.80, pipelines/driver_ms 14.70, render/graph/ms 8.66, stream/spawn/ms 2.17, render/present/ms 1.32, stream/tick/ms 1.13 |
| warm-up | 4.53 | 69.3 | pipelines/wait_ms 1960, pipelines/compile_ms 236, pipelines/front_ms 221, main/busy/ms 64.97, stream/drive/ms 50.55, render/thread/ms 40.94, render/busy/ms 39.86, pipelines/driver_ms 14.70, stream/spawn/ms 3.98, render/graph/ms 2.82, terrain/update/ms 1.07, render/present/ms 0.91 |
| warm-up | 4.61 | 73.1 | pipelines/wait_ms 8760, pipelines/compile_ms 200, pipelines/front_ms 187, main/busy/ms 65.71, stream/drive/ms 46.42, render/thread/ms 35.89, render/busy/ms 34.47, pipelines/driver_ms 12.92, render/graph/ms 5.05, stream/spawn/ms 2.08, render/present/ms 1.06, stream/tick/ms 0.78 |
| warm-up | 4.69 | 54.8 | pipelines/wait_ms 3464, main/busy/ms 50.61, render/thread/ms 36.86, render/busy/ms 35.03, stream/drive/ms 34.10, pipelines/compile_ms 19.81, pipelines/front_ms 16.84, render/graph/ms 4.29, pipelines/driver_ms 2.97, stream/spawn/ms 2.22, render/present/ms 1.66, stream/activate/ms 1.07 |
| warm-up | 4.89 | 35.4 | render/thread/ms 43.33, render/busy/ms 41.79, main/busy/ms 25.19, render/graph/ms 5.64, stream/drive/ms 5.39, stream/spawn/ms 2.26, stream/tick/ms 1.72, render/present/ms 1.17, terrain/update/ms 0.94, terrain/select/ms 0.77, bigspace/total/ms 0.76, bigspace/high_precision/ms 0.49 |
| warm-up | 4.95 | 34.4 | pipelines/wait_ms 2075, pipelines/compile_ms 156, pipelines/front_ms 147, render/thread/ms 45.58, render/busy/ms 44.37, main/busy/ms 18.13, pipelines/driver_ms 8.74, render/graph/ms 4.09, stream/drive/ms 2.31, render/present/ms 1.01, terrain/update/ms 0.93, terrain/select/ms 0.79 |
| warm-up | 5.05 | 45.8 | pipelines/wait_ms 6272, pipelines/compile_ms 57.84, pipelines/front_ms 53.53, main/busy/ms 43.87, render/thread/ms 34.01, render/busy/ms 32.69, stream/drive/ms 10.52, render/graph/ms 5.33, pipelines/driver_ms 4.31, terrain/update/ms 3.62, stream/spawn/ms 1.14, render/present/ms 1.12 |
| warm-up | 6.05 | 54.8 | pipelines/wait_ms 2258, render/thread/ms 56.31, render/busy/ms 54.53, main/busy/ms 47.02, pipelines/compile_ms 43.11, pipelines/front_ms 41.39, stream/drive/ms 19.60, render/graph/ms 8.55, stream/spawn/ms 2.17, pipelines/driver_ms 1.72, stream/tick/ms 1.72, render/present/ms 1.56 |
| measure | 2.73 | 34.7 | render/thread/ms 46.31, render/busy/ms 45.15, render/graph/ms 30.25, main/busy/ms 21.73, render/present/ms 0.99, bigspace/total/ms 0.78, terrain/update/ms 0.55, controller/feed/ms 0.53, terrain/select/ms 0.48, stream/drive/ms 0.47, stream/tick/ms 0.42, bigspace/low_precision/ms 0.35 |

## Run

- `C:\Users\leogloglo\AppData\Local\Temp\depth-keys-off\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\dkeys_off\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **70%** (min 47%) · 2580 MHz (min 2580) · memory 8001 MHz · 71.1 W · 73 °C (max 77) · VRAM 5685 MiB · throttled 100% of samples
