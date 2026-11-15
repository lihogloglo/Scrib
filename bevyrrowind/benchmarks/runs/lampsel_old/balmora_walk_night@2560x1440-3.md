# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5218 frames over 65.6 s · **median 11.75 ms (85 fps)** · mean 12.58 · p95 17.02 · p99 18.94 (1% low 53 fps) · worst 35.22 ms · 176 spikes over 1.5× median · 21.0 stutters/min (work 14.6/min)
- warm-up: 464 frames over 9.3 s, worst 2468.2 ms
- hitches (> 29.4 ms): **6** measured, 21 in warm-up
- timed GPU passes: 7.53 ms of a 12.58 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.75 ms, limit 11 ms
- **FAIL** frame_ms: 23 stutters (> 1.5x moving median of 60) in 1.09 min = 21.02/min, limit 1/min
- **FAIL** worst frame 35.22 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.179 | 2.230 | 2.798 | 0.98 | 0.179 |
| main_opaque_pass_3d | 1.415 | 1.448 | 2.284 | 0.98 | 0.062 |
| early prepass | 0.549 | 0.562 | 0.816 | 0.98 | 0.105 |
| auxiliary_views | 0.399 | 0.408 | 1.574 | 0.98 | 0.320 |
| bloom | 0.334 | 0.342 | 0.489 | 0.98 | 0.065 |
| atmosphere_luts | 0.333 | 0.341 | 0.624 | 0.98 | 0.003 |
| water_velocity | 0.280 | 0.287 | 0.371 | 0.98 | 0.001 |
| water_sspr | 0.217 | 0.222 | 0.218 | 0.98 | 0.002 |
| uniform_allocation | 0.210 | 0.215 | 0.223 | 0.98 | 0.036 |
| ssao | 0.201 | 0.206 | 0.209 | 0.98 | 0.007 |
| bin_unpacking | 0.177 | 0.181 | 0.182 | 0.98 | 0.026 |
| water_fft | 0.139 | 0.142 | 0.139 | 0.98 | 0.003 |
| atmos_froxel_inject | 0.135 | 0.138 | 0.132 | 0.98 | 0.001 |
| upscaling | 0.129 | 0.132 | 0.136 | 0.98 | 0.004 |
| clouds_composite | 0.109 | 0.112 | 0.110 | 0.98 | 0.002 |
| tonemapping | 0.091 | 0.093 | 0.093 | 0.98 | 0.003 |
| main_transmissive_pass_3d | 0.085 | 0.087 | 0.151 | 0.98 | 0.008 |
| clustering | 0.065 | 0.067 | 0.069 | 0.98 | 0.020 |
| render_sky | 0.054 | 0.055 | 0.054 | 0.98 | 0.002 |
| clouds_trace | 0.053 | 0.054 | 0.073 | 0.98 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.98 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.438 | 0.729 | 0.11 | 0.001 |
| surface_trail | 0.046 | 0.060 | 0.057 | 0.77 | 0.002 |
| water_snapshot | 0.037 | 0.038 | 0.047 | 0.98 | 0.002 |
| early_mesh_preprocessing | 0.035 | 0.035 | 0.038 | 0.98 | 0.005 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.030 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.231 | 0.236 | 0.11 | 0.004 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.017 | 0.017 | 0.032 | 0.98 | 0.116 |
| night_sky | 0.015 | 0.016 | 0.033 | 0.98 | 0.001 |
| per_view_shadow_pass | 0.014 | 0.015 | 0.015 | 0.98 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.98 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.98 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.98 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 0.98 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 32.669 | 0.000 | 19830.28 | 170464.3 |
| render/thread/ms | 11.383 | 15.655 | 34.23 | 59395.5 |
| render/busy/ms | 10.780 | 15.032 | 33.43 | 56251.3 |
| main/busy/ms | 9.104 | 11.683 | 29.64 | 47502.6 |
| render/graph/ms | 6.620 | 9.851 | 21.71 | 34542.9 |
| bigspace/total/ms | 0.562 | 1.374 | 3.76 | 2933.1 |
| render/present/ms | 0.511 | 0.624 | 14.66 | 2664.7 |
| pipelines/compile_ms | 0.471 | 0.000 | 168.26 | 2458.4 |
| pipelines/front_ms | 0.455 | 0.000 | 162.53 | 2374.2 |
| terrain/update/ms | 0.307 | 0.400 | 1.70 | 1603.7 |
| terrain/select/ms | 0.274 | 0.357 | 0.70 | 1430.1 |
| controller/feed/ms | 0.274 | 0.359 | 0.66 | 1427.6 |
| stream/drive/ms | 0.250 | 0.350 | 21.32 | 1306.6 |
| bigspace/low_precision/ms | 0.231 | 0.618 | 2.94 | 1207.5 |
| stream/tick/ms | 0.192 | 0.264 | 0.61 | 999.5 |
| far/select/ms | 0.190 | 0.259 | 0.56 | 990.0 |
| bigspace/high_precision/ms | 0.165 | 0.590 | 3.37 | 860.0 |
| anim/evaluate_cpu/ms | 0.163 | 0.207 | 0.53 | 849.7 |
| controller/move/ms | 0.156 | 0.333 | 0.62 | 814.2 |
| anim/evaluate/ms | 0.148 | 0.264 | 2.42 | 770.3 |
| render/acquire/ms | 0.092 | 0.121 | 0.36 | 479.5 |
| physics/insert/ms | 0.066 | 0.477 | 1.02 | 342.9 |
| physics/step/ms | 0.053 | 0.086 | 0.60 | 278.7 |
| pipelines/driver_ms | 0.016 | 0.000 | 6.61 | 84.3 |
| weather/map/ms | 0.016 | 0.020 | 0.25 | 82.8 |
| anim/apply/ms | 0.011 | 0.022 | 0.17 | 59.3 |
| controller/avoid/ms | 0.011 | 0.014 | 0.13 | 56.3 |
| weather/eval/ms | 0.009 | 0.012 | 0.08 | 49.3 |
| script/update/ms | 0.007 | 0.009 | 0.21 | 36.4 |
| stream/despawn/ms | 0.006 | 0.000 | 0.99 | 29.7 |
| script/input/ms | 0.005 | 0.007 | 0.07 | 26.0 |
| script/ui/ms | 0.005 | 0.006 | 0.13 | 25.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.06 | 25.1 |
| controller/write/ms | 0.004 | 0.004 | 0.07 | 18.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.65 | 14.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 569 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 17 |
| controller/movers/count | count | 20 | 23 | 17 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1272 | 2262 | 586 |
| crowd/c0/meshes/count | count | 349 | 646 | 147 |
| crowd/c1/count | count | 147 | 155 | 147 |
| crowd/c1/joints/count | count | 5547 | 5950 | 5578 |
| crowd/c1/meshes/count | count | 1174 | 1294 | 1186 |
| crowd/c2/count | count | 399 | 418 | 418 |
| crowd/c2/joints/count | count | 6406 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1822 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 158 | 215 | 199 |
| crowd/joints_extracted/count | count | 3642 | 8843 | 5466 |
| crowd/joints_uploaded/count | count | 3554 | 6129 | 5466 |
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
| frame/work/ms | ms | 10.05 | 33.43 | 9.19 |
| gpu/timed/ms | ms | 8.35 | 11.44 | 8.48 |
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
| phases/alpha_mask3d/items | count | 790 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 790 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 28 | 30 | 29 |
| phases/opaque3d/bins | count | 1293 | 1400 | 1349 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3346 | 5118 | 3181 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 28 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 30 | 29 |
| phases/opaque3d_prepass/bins | count | 1293 | 1400 | 1349 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3346 | 5118 | 3181 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 28 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 35 | 255 | 28 |
| phases/shadow/bins | count | 2162 | 4059 | 1121 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3259 | 7206 | 2391 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 122 | 293 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2346 |
| physics/items | count | 137 | 476 | 19 |
| physics/pending | count | 0 | 940 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 218 | 223 | 223 |
| pipelines/waiting | count | 0 | 32 | 0 |
| preprocess/bind_groups | count | 11 | 706 | 6 |
| preprocess/bind_groups_created | count | 11 | 706 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 220 | 441 | 216 |
| process/cpu_usage | % | 24.41 | 49.04 | 25.96 |
| process/mem_usage | GiB | 5.13 | 5.29 | 5.29 |
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
| started_load_count |  | 4767 | 4857 | 4857 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 431 | 528 | 396 |
| statics/materials_bound | count | 431 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.47 | 50.87 | 29.01 |
| system/mem_usage | % | 58.77 | 59.63 | 59.26 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2821 | 2932 | 2932 |
| textures/images | count | 2589 | 2642 | 2642 |
| views/all/batch_sets | count | 105 | 321 | 100 |
| views/all/bins | count | 4928 | 6946 | 4613 |
| views/all/count | count | 21 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.4 k | 18.9 k | 9987 |
| views/all/pipelines | count | 192 | 353 | 96 |
| views/all/pipelines_distinct | count | 94 | 110 | 91 |
| views/all/work_items | count | 11.0 k | 17.4 k | 10.3 k |
| views/cascade/batch_sets | count | 33 | 40 | 28 |
| views/cascade/bins | count | 1182 | 1515 | 1121 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2742 | 4271 | 2391 |
| views/cascade/pipelines | count | 33 | 40 | 28 |
| views/cascade/work_items | count | 2742 | 4271 | 2391 |
| views/lamp_face/batch_sets | count | 0 | 220 | 0 |
| views/lamp_face/bins | count | 1044 | 2930 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1192 | 3218 | 0 |
| views/lamp_face/pipelines | count | 90 | 257 | 0 |
| views/lamp_face/work_items | count | 0 | 2815 | 0 |
| views/main/batch_sets | count | 70 | 76 | 72 |
| views/main/bins | count | 2888 | 3598 | 3492 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8269 | 13.0 k | 7596 |
| views/main/pipelines | count | 66 | 72 | 68 |
| views/main/work_items | count | 8432 | 13.2 k | 7904 |
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
| anim/bones_written/count | count | 49.00 | 236 | 255.7 k |
| anim/events/count | count | 0.10 | 3 | 523 |
| assets/Image/added | count | 0.12 | 43 | 614 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.79 | 152 | 4133 |
| assets/Mesh/removed | count | 0.93 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 480 |
| assets/StandardMaterial/modified | count | 4.15 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 394 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.97 | 1379 | 5041 |
| physics/inserts | count | 0.61 | 163 | 3195 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.45 | 145 | 2335 |
| physics/steps | count | 0.75 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 17 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 18 | 378 |
| stream/upload/bytes | bytes | 6488 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1422 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.4 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2468.2 | main/busy/ms 2440, surface/advance_cpu_ms 0.71, stream/drive/ms 0.33, surface/cell_upload_ms 0.29, bigspace/total/ms 0.12, stream/tick/ms 0.09, bigspace/high_precision/ms 0.05, weather/map/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.35 | 885.4 | main/busy/ms 2481, render/thread/ms 848, render/busy/ms 696, render/acquire/ms 149, render/graph/ms 6.28, render/present/ms 2.32, stream/drive/ms 1.09, surface/advance_cpu_ms 0.71, stream/tick/ms 0.57, terrain/update/ms 0.46, terrain/select/ms 0.30, surface/cell_upload_ms 0.29 |
| warm-up | 3.42 | 68.9 | render/thread/ms 859, render/busy/ms 706, pipelines/wait_ms 433, render/acquire/ms 150, pipelines/compile_ms 77.39, pipelines/front_ms 70.47, main/busy/ms 57.35, terrain/update/ms 23.62, render/graph/ms 9.79, pipelines/driver_ms 6.92, render/present/ms 3.32, stream/drive/ms 1.52 |
| warm-up | 3.69 | 111.5 | pipelines/compile_ms 222, pipelines/front_ms 198, main/busy/ms 76.23, render/thread/ms 58.06, render/busy/ms 56.60, pipelines/wait_ms 44.15, pipelines/driver_ms 23.42, render/graph/ms 2.97, stream/drive/ms 1.89, stream/tick/ms 1.64, render/present/ms 1.18, terrain/update/ms 0.75 |
| warm-up | 3.82 | 45.1 | main/busy/ms 39.25, render/thread/ms 37.16, render/busy/ms 36.01, stream/drive/ms 27.71, render/graph/ms 2.37, render/present/ms 0.96, stream/tick/ms 0.84, terrain/update/ms 0.70, stream/spawn/ms 0.70, terrain/select/ms 0.58, bigspace/total/ms 0.23, render/acquire/ms 0.19 |
| warm-up | 3.88 | 46.5 | pipelines/wait_ms 670, pipelines/compile_ms 193, pipelines/front_ms 186, render/thread/ms 39.62, main/busy/ms 38.56, render/busy/ms 37.92, stream/drive/ms 26.16, pipelines/driver_ms 6.84, render/graph/ms 4.58, render/present/ms 1.46, stream/spawn/ms 1.11, stream/tick/ms 0.81 |
| warm-up | 3.96 | 70.7 | render/thread/ms 52.61, render/busy/ms 50.19, main/busy/ms 28.73, stream/drive/ms 11.75, render/graph/ms 7.64, render/present/ms 2.07, stream/tick/ms 1.95, terrain/update/ms 0.85, terrain/select/ms 0.67, stream/spawn/ms 0.65, render/acquire/ms 0.34, stream/activate/ms 0.25 |
| warm-up | 3.99 | 34.5 | render/thread/ms 69.65, render/busy/ms 67.31, main/busy/ms 51.95, stream/drive/ms 32.70, render/graph/ms 8.07, render/present/ms 2.06, stream/tick/ms 1.72, stream/activate/ms 0.79, terrain/update/ms 0.73, terrain/select/ms 0.58, bigspace/total/ms 0.31, render/acquire/ms 0.27 |
| warm-up | 4.11 | 113.6 | pipelines/wait_ms 1596, pipelines/compile_ms 199, pipelines/front_ms 188, render/thread/ms 85.29, render/busy/ms 62.92, main/busy/ms 51.88, stream/drive/ms 32.90, render/present/ms 22.16, pipelines/driver_ms 10.98, render/graph/ms 2.99, stream/spawn/ms 1.12, stream/tick/ms 1.03 |
| warm-up | 4.27 | 49.2 | main/busy/ms 73.77, stream/drive/ms 54.30, render/thread/ms 53.49, render/busy/ms 50.67, render/graph/ms 12.27, stream/spawn/ms 2.67, render/present/ms 2.60, stream/tick/ms 1.39, terrain/update/ms 0.95, terrain/select/ms 0.79, bigspace/total/ms 0.53, bigspace/low_precision/ms 0.36 |
| warm-up | 4.35 | 73.0 | pipelines/wait_ms 4003, pipelines/compile_ms 181, pipelines/front_ms 170, main/busy/ms 76.12, render/thread/ms 60.60, render/busy/ms 58.74, stream/drive/ms 53.98, pipelines/driver_ms 10.89, render/graph/ms 6.80, stream/spawn/ms 2.73, render/present/ms 1.61, stream/tick/ms 1.07 |
| warm-up | 4.43 | 65.2 | pipelines/wait_ms 6994, pipelines/compile_ms 86.60, pipelines/front_ms 81.41, main/busy/ms 67.16, stream/drive/ms 46.00, render/thread/ms 32.20, render/busy/ms 30.99, pipelines/driver_ms 5.20, render/graph/ms 4.22, stream/activate/ms 1.11, render/present/ms 0.98, stream/spawn/ms 0.92 |
| warm-up | 4.49 | 44.1 | pipelines/wait_ms 6457, main/busy/ms 45.33, pipelines/compile_ms 43.76, pipelines/front_ms 40.28, stream/drive/ms 31.48, render/thread/ms 24.50, render/busy/ms 23.20, render/graph/ms 5.36, pipelines/driver_ms 3.48, terrain/update/ms 3.02, render/present/ms 1.13, stream/activate/ms 1.12 |
| warm-up | 4.66 | 31.8 | pipelines/wait_ms 7883, render/thread/ms 42.12, render/busy/ms 40.86, main/busy/ms 35.82, pipelines/compile_ms 35.81, pipelines/front_ms 33.02, terrain/update/ms 8.57, render/graph/ms 6.78, stream/drive/ms 3.96, pipelines/driver_ms 2.80, stream/spawn/ms 2.23, bigspace/total/ms 1.16 |
| warm-up | 4.69 | 29.4 | pipelines/wait_ms 2390, main/busy/ms 49.31, render/thread/ms 46.50, render/busy/ms 44.70, render/graph/ms 7.03, pipelines/compile_ms 6.69, stream/drive/ms 6.38, pipelines/front_ms 5.83, terrain/update/ms 2.98, stream/spawn/ms 2.31, render/present/ms 1.50, physics/insert/ms 0.98 |
| warm-up | 4.74 | 45.2 | render/thread/ms 47.36, render/busy/ms 45.57, main/busy/ms 42.17, render/graph/ms 6.63, stream/drive/ms 4.82, render/present/ms 1.56, stream/tick/ms 0.85, stream/spawn/ms 0.82, terrain/update/ms 0.81, terrain/select/ms 0.66, physics/insert/ms 0.60, bigspace/total/ms 0.37 |
| warm-up | 4.80 | 52.8 | pipelines/wait_ms 3469, pipelines/compile_ms 173, pipelines/front_ms 168, main/busy/ms 43.92, render/thread/ms 30.37, render/busy/ms 28.72, stream/drive/ms 8.92, pipelines/driver_ms 5.02, render/graph/ms 4.51, render/present/ms 1.45, stream/spawn/ms 1.01, stream/tick/ms 0.68 |
| warm-up | 5.29 | 32.3 | render/thread/ms 48.34, render/busy/ms 47.08, main/busy/ms 36.34, render/graph/ms 6.57, stream/drive/ms 4.16, stream/spawn/ms 2.08, physics/insert/ms 1.46, render/present/ms 1.04, stream/tick/ms 0.88, terrain/update/ms 0.82, bigspace/total/ms 0.73, terrain/select/ms 0.68 |
| warm-up | 5.39 | 36.0 | render/thread/ms 50.06, render/busy/ms 48.80, main/busy/ms 34.83, render/graph/ms 5.65, stream/drive/ms 5.34, stream/spawn/ms 2.10, physics/insert/ms 1.68, stream/tick/ms 1.55, render/present/ms 1.00, terrain/update/ms 0.85, bigspace/total/ms 0.82, terrain/select/ms 0.74 |
| warm-up | 6.83 | 35.6 | main/busy/ms 54.03, render/thread/ms 33.34, render/busy/ms 31.57, render/graph/ms 7.04, render/present/ms 1.49, physics/insert/ms 1.29, anim/evaluate_cpu/ms 1.27, stream/drive/ms 1.22, bigspace/total/ms 1.02, stream/tick/ms 0.63, terrain/update/ms 0.61, anim/evaluate/ms 0.60 |
| warm-up | 6.87 | 34.0 | main/busy/ms 41.34, render/thread/ms 36.15, render/busy/ms 34.80, render/graph/ms 5.11, stream/drive/ms 1.33, physics/insert/ms 1.29, bigspace/total/ms 1.27, render/present/ms 1.07, anim/evaluate_cpu/ms 0.76, stream/tick/ms 0.67, terrain/update/ms 0.64, terrain/select/ms 0.56 |
| measure | 2.73 | 35.2 | render/thread/ms 45.71, render/busy/ms 44.42, render/graph/ms 22.46, main/busy/ms 19.90, render/present/ms 1.10, physics/insert/ms 1.07, bigspace/total/ms 0.79, terrain/update/ms 0.56, terrain/select/ms 0.50, stream/drive/ms 0.50, stream/tick/ms 0.45, controller/feed/ms 0.41 |
| measure | 18.21 | 31.0 | main/busy/ms 41.33, render/thread/ms 32.45, render/busy/ms 31.41, stream/drive/ms 21.56, render/graph/ms 19.95, render/present/ms 0.87, bigspace/total/ms 0.76, controller/feed/ms 0.72, terrain/update/ms 0.60, terrain/select/ms 0.53, far/select/ms 0.48, stream/tick/ms 0.35 |
| measure | 22.26 | 32.7 | render/thread/ms 44.78, render/busy/ms 29.40, main/busy/ms 24.32, render/graph/ms 18.67, render/present/ms 15.17, bigspace/total/ms 0.80, controller/feed/ms 0.68, terrain/update/ms 0.57, terrain/select/ms 0.52, stream/drive/ms 0.49, stream/tick/ms 0.44, anim/evaluate_cpu/ms 0.41 |
| measure | 23.12 | 30.2 | render/thread/ms 44.97, render/busy/ms 42.62, main/busy/ms 30.58, render/graph/ms 17.70, render/present/ms 2.18, bigspace/total/ms 1.88, stream/drive/ms 0.95, controller/feed/ms 0.87, bigspace/low_precision/ms 0.76, anim/evaluate_cpu/ms 0.61, terrain/update/ms 0.56, terrain/select/ms 0.50 |
| measure | 59.03 | 30.4 | render/thread/ms 38.28, render/busy/ms 36.85, main/busy/ms 31.16, render/graph/ms 17.94, stream/drive/ms 1.68, controller/feed/ms 1.26, render/present/ms 1.26, bigspace/total/ms 1.22, stream/spawn/ms 0.87, far/select/ms 0.78, terrain/update/ms 0.72, stream/tick/ms 0.68 |
| measure | 59.06 | 29.5 | render/thread/ms 40.14, render/busy/ms 38.99, main/busy/ms 23.77, render/graph/ms 10.90, controller/feed/ms 1.02, render/present/ms 0.98, bigspace/total/ms 0.98, stream/drive/ms 0.89, stream/tick/ms 0.77, terrain/update/ms 0.75, far/select/ms 0.67, terrain/select/ms 0.66 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_old.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lampsel_old\balmora_walk_night@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 14%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **75%** (min 52%) · 2580 MHz (min 2580) · memory 8001 MHz · 73.4 W · 75 °C (max 81) · VRAM 5285 MiB · throttled 100% of samples
