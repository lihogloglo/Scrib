# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5282 frames over 65.6 s · **median 11.74 ms (85 fps)** · mean 12.43 · p95 16.33 · p99 18.35 (1% low 55 fps) · worst 53.27 ms · 99 spikes over 1.5× median · 21.9 stutters/min (work 19.2/min)
- warm-up: 568 frames over 10.5 s, worst 2461.2 ms
- hitches (> 29.3 ms): **7** measured, 20 in warm-up
- timed GPU passes: 7.69 ms of a 12.43 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.74 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.94/min, limit 1/min
- **FAIL** worst frame 53.27 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.177 | 2.200 | 2.533 | 0.99 | 0.178 |
| main_opaque_pass_3d | 1.421 | 1.436 | 2.063 | 0.99 | 0.064 |
| early prepass | 0.546 | 0.551 | 0.818 | 0.99 | 0.106 |
| bloom | 0.334 | 0.338 | 0.486 | 0.99 | 0.064 |
| atmosphere_luts | 0.330 | 0.333 | 0.508 | 0.99 | 0.003 |
| tonemapping | 0.280 | 0.283 | 0.331 | 0.99 | 0.003 |
| water_velocity | 0.277 | 0.280 | 0.362 | 0.99 | 0.001 |
| auxiliary_views | 0.271 | 0.274 | 1.204 | 0.99 | 0.223 |
| upscaling | 0.224 | 0.226 | 0.261 | 0.99 | 0.004 |
| water_sspr | 0.219 | 0.221 | 0.220 | 0.99 | 0.002 |
| uniform_allocation | 0.210 | 0.212 | 0.221 | 0.99 | 0.037 |
| ssao | 0.200 | 0.203 | 0.209 | 0.99 | 0.007 |
| bin_unpacking | 0.172 | 0.173 | 0.178 | 0.99 | 0.026 |
| water_fft | 0.135 | 0.137 | 0.138 | 0.99 | 0.003 |
| atmos_froxel_inject | 0.134 | 0.135 | 0.131 | 0.99 | 0.002 |
| clouds_composite | 0.108 | 0.109 | 0.111 | 0.99 | 0.002 |
| main_transmissive_pass_3d | 0.083 | 0.084 | 0.140 | 0.99 | 0.008 |
| clustering | 0.066 | 0.067 | 0.069 | 0.99 | 0.020 |
| render_sky | 0.055 | 0.056 | 0.061 | 0.99 | 0.002 |
| clouds_trace | 0.054 | 0.055 | 0.073 | 0.99 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.439 | 0.752 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.051 | 0.052 | 0.99 | 0.001 |
| water_snapshot | 0.046 | 0.047 | 0.063 | 0.99 | 0.002 |
| surface_trail | 0.045 | 0.058 | 0.057 | 0.78 | 0.002 |
| early_mesh_preprocessing | 0.034 | 0.035 | 0.038 | 0.99 | 0.005 |
| atmos_froxel_integrate | 0.030 | 0.031 | 0.032 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.230 | 0.237 | 0.12 | 0.004 |
| water_mips | 0.023 | 0.023 | 0.024 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.018 | 0.018 | 0.033 | 0.99 | 0.113 |
| night_sky | 0.015 | 0.016 | 0.036 | 0.99 | 0.001 |
| per_view_shadow_pass | 0.015 | 0.015 | 0.016 | 0.99 | 0.005 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.253 | 15.004 | 50.73 | 59438.3 |
| render/busy/ms | 10.659 | 14.331 | 50.19 | 56302.3 |
| main/busy/ms | 9.011 | 11.638 | 29.36 | 47595.9 |
| render/graph/ms | 6.580 | 9.278 | 46.57 | 34755.1 |
| pipelines/wait_ms | 4.491 | 0.000 | 6910.31 | 23723.7 |
| bigspace/total/ms | 0.598 | 2.202 | 4.14 | 3156.8 |
| render/present/ms | 0.503 | 0.597 | 12.55 | 2654.7 |
| terrain/update/ms | 0.305 | 0.391 | 1.79 | 1608.6 |
| terrain/select/ms | 0.271 | 0.350 | 0.60 | 1431.6 |
| controller/feed/ms | 0.270 | 0.361 | 0.67 | 1425.6 |
| bigspace/low_precision/ms | 0.256 | 0.664 | 3.48 | 1354.2 |
| stream/drive/ms | 0.256 | 0.358 | 21.53 | 1354.1 |
| stream/tick/ms | 0.198 | 0.269 | 0.67 | 1048.1 |
| far/select/ms | 0.190 | 0.265 | 0.63 | 1004.7 |
| pipelines/compile_ms | 0.185 | 0.000 | 126.97 | 976.7 |
| pipelines/front_ms | 0.179 | 0.000 | 122.77 | 943.3 |
| bigspace/high_precision/ms | 0.174 | 0.569 | 3.60 | 920.4 |
| anim/evaluate_cpu/ms | 0.164 | 0.204 | 0.41 | 867.0 |
| anim/evaluate/ms | 0.140 | 0.257 | 4.88 | 741.6 |
| controller/move/ms | 0.131 | 0.303 | 0.83 | 691.3 |
| render/acquire/ms | 0.091 | 0.122 | 0.45 | 481.4 |
| physics/step/ms | 0.053 | 0.086 | 0.54 | 277.6 |
| physics/insert/ms | 0.039 | 0.400 | 0.89 | 204.0 |
| weather/map/ms | 0.016 | 0.021 | 0.15 | 82.4 |
| anim/apply/ms | 0.011 | 0.021 | 0.18 | 58.7 |
| controller/avoid/ms | 0.011 | 0.014 | 0.21 | 57.0 |
| weather/eval/ms | 0.010 | 0.013 | 0.04 | 50.5 |
| script/update/ms | 0.007 | 0.009 | 0.13 | 36.7 |
| pipelines/driver_ms | 0.006 | 0.000 | 4.72 | 33.4 |
| stream/despawn/ms | 0.006 | 0.000 | 1.05 | 29.9 |
| script/input/ms | 0.005 | 0.008 | 0.12 | 28.7 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 26.1 |
| script/ui/ms | 0.005 | 0.006 | 0.12 | 25.4 |
| controller/write/ms | 0.003 | 0.004 | 0.03 | 18.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.65 | 14.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.15 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.13 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 2 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 30 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1236 | 2262 | 583 |
| crowd/c0/meshes/count | count | 341 | 646 | 162 |
| crowd/c1/count | count | 146 | 156 | 148 |
| crowd/c1/joints/count | count | 5573 | 5992 | 5613 |
| crowd/c1/meshes/count | count | 1193 | 1308 | 1179 |
| crowd/c2/count | count | 397 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 156 | 215 | 202 |
| crowd/joints_extracted/count | count | 3578 | 7905 | 5619 |
| crowd/joints_uploaded/count | count | 3522 | 6129 | 5619 |
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
| frame/work/ms | ms | 10.11 | 50.19 | 8.93 |
| gpu/timed/ms | ms | 8.48 | 11.87 | 8.75 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2523 | 2580 | 2580 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 278.4 k | 278.4 k | 278.4 k |
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
| phases/alpha_mask3d/items | count | 799 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 799 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 28 | 30 | 29 |
| phases/opaque3d/bins | count | 1291 | 1426 | 1349 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3363 | 5131 | 3181 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 28 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 30 | 29 |
| phases/opaque3d_prepass/bins | count | 1291 | 1426 | 1349 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3363 | 5131 | 3181 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 28 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 34 | 253 | 29 |
| phases/shadow/bins | count | 2222 | 4043 | 1136 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3608 | 7122 | 2441 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 117 | 279 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 218 | 223 | 223 |
| pipelines/waiting | count | 0 | 8 | 0 |
| preprocess/bind_groups | count | 10 | 702 | 6 |
| preprocess/bind_groups_created | count | 10 | 702 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 219 | 396 | 218 |
| process/cpu_usage | % | 25 | 39.72 | 27.88 |
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
| started_load_count |  | 4765 | 4857 | 4857 |
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
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.18 | 52.29 | 28.85 |
| system/mem_usage | % | 63.35 | 64.45 | 59.22 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2820 | 2936 | 2936 |
| textures/images | count | 2587 | 2647 | 2647 |
| views/all/batch_sets | count | 104 | 319 | 101 |
| views/all/bins | count | 4967 | 6927 | 4628 |
| views/all/count | count | 21 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.5 k | 19.0 k | 10.0 k |
| views/all/pipelines | count | 185 | 344 | 97 |
| views/all/pipelines_distinct | count | 96 | 107 | 92 |
| views/all/work_items | count | 11.0 k | 17.2 k | 10.3 k |
| views/cascade/batch_sets | count | 33 | 40 | 29 |
| views/cascade/bins | count | 1179 | 1487 | 1136 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2738 | 4277 | 2441 |
| views/cascade/pipelines | count | 33 | 40 | 29 |
| views/cascade/work_items | count | 2738 | 4277 | 2441 |
| views/lamp_face/batch_sets | count | 0 | 218 | 0 |
| views/lamp_face/bins | count | 1055 | 2909 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1186 | 3198 | 0 |
| views/lamp_face/pipelines | count | 87 | 244 | 0 |
| views/lamp_face/work_items | count | 0 | 2786 | 0 |
| views/main/batch_sets | count | 70 | 76 | 72 |
| views/main/bins | count | 2888 | 3646 | 3492 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8311 | 13.0 k | 7596 |
| views/main/pipelines | count | 66 | 72 | 68 |
| views/main/work_items | count | 8480 | 13.2 k | 7904 |
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
| anim/bones_written/count | count | 47.36 | 214 | 250.2 k |
| anim/events/count | count | 0.10 | 4 | 514 |
| assets/Image/added | count | 0.11 | 43 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.77 | 152 | 4075 |
| assets/Mesh/removed | count | 0.92 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 459 |
| assets/StandardMaterial/modified | count | 4.10 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 394 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.89 | 1386 | 4718 |
| physics/inserts | count | 0.45 | 134 | 2375 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.44 | 145 | 2335 |
| physics/steps | count | 0.75 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 15 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 19 | 378 |
| stream/upload/bytes | bytes | 6409 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1405 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.3 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.46 | 2461.2 | main/busy/ms 2436, surface/advance_cpu_ms 0.72, stream/drive/ms 0.30, surface/cell_upload_ms 0.28, stream/tick/ms 0.10, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.05, weather/map/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.03, anim/evaluate/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.29 | 832.7 | main/busy/ms 2477, render/thread/ms 797, render/busy/ms 653, render/acquire/ms 142, render/graph/ms 5.57, render/present/ms 2.22, stream/drive/ms 1.03, surface/advance_cpu_ms 0.72, terrain/update/ms 0.56, stream/tick/ms 0.54, terrain/select/ms 0.39, bigspace/total/ms 0.28 |
| warm-up | 3.37 | 79.9 | render/thread/ms 809, render/busy/ms 663, pipelines/wait_ms 531, render/acquire/ms 143, pipelines/compile_ms 87.82, pipelines/front_ms 79.91, main/busy/ms 56.61, terrain/update/ms 18.23, render/graph/ms 9.51, pipelines/driver_ms 7.91, render/present/ms 2.97, stream/drive/ms 1.48 |
| warm-up | 3.62 | 100.8 | pipelines/wait_ms 217, render/thread/ms 68.60, main/busy/ms 68.33, render/busy/ms 67.03, pipelines/compile_ms 6.64, pipelines/front_ms 5.87, render/graph/ms 3.79, stream/drive/ms 1.67, stream/tick/ms 1.34, render/present/ms 1.29, terrain/update/ms 0.86, pipelines/driver_ms 0.77 |
| warm-up | 3.81 | 69.3 | render/thread/ms 46.35, render/busy/ms 44.25, main/busy/ms 43.48, stream/drive/ms 32.38, render/graph/ms 2.59, render/present/ms 1.86, stream/tick/ms 0.99, terrain/update/ms 0.67, terrain/select/ms 0.54, render/acquire/ms 0.23, bigspace/total/ms 0.22, stream/spawn/ms 0.09 |
| warm-up | 3.93 | 55.2 | pipelines/wait_ms 817, pipelines/compile_ms 185, pipelines/front_ms 172, render/thread/ms 50.29, render/busy/ms 48.26, main/busy/ms 35.53, stream/drive/ms 20.59, pipelines/driver_ms 13.71, render/graph/ms 6.50, render/present/ms 1.79, stream/spawn/ms 1.17, stream/tick/ms 0.78 |
| warm-up | 4.00 | 50.4 | main/busy/ms 52.38, render/thread/ms 37.57, render/busy/ms 35.89, stream/drive/ms 32.85, render/graph/ms 3.43, stream/spawn/ms 1.75, render/present/ms 1.48, stream/tick/ms 1.41, terrain/update/ms 0.99, terrain/select/ms 0.79, bigspace/total/ms 0.63, render/acquire/ms 0.20 |
| warm-up | 4.06 | 63.3 | render/thread/ms 56.02, render/busy/ms 54.54, main/busy/ms 53.75, stream/drive/ms 32.83, render/graph/ms 3.09, render/present/ms 1.30, stream/spawn/ms 1.06, terrain/update/ms 0.98, stream/tick/ms 0.95, terrain/select/ms 0.77, stream/activate/ms 0.53, bigspace/total/ms 0.38 |
| warm-up | 4.13 | 69.3 | pipelines/wait_ms 4692, pipelines/compile_ms 352, pipelines/front_ms 212, pipelines/driver_ms 140, render/thread/ms 94.49, render/busy/ms 92.85, main/busy/ms 22.31, render/graph/ms 8.73, stream/drive/ms 4.10, render/present/ms 1.47, stream/activate/ms 1.05, terrain/update/ms 1.04 |
| warm-up | 4.16 | 35.6 | pipelines/wait_ms 8430, pipelines/compile_ms 392, pipelines/front_ms 244, pipelines/driver_ms 148, render/thread/ms 92.82, render/busy/ms 90.68, main/busy/ms 18.68, render/graph/ms 12.22, stream/drive/ms 4.03, render/present/ms 1.91, stream/tick/ms 1.52, stream/activate/ms 1.03 |
| warm-up | 4.19 | 30.2 | pipelines/wait_ms 5292, pipelines/compile_ms 79.54, pipelines/front_ms 69.25, render/thread/ms 57.17, render/busy/ms 55.05, main/busy/ms 22.80, render/graph/ms 17.12, pipelines/driver_ms 10.29, stream/drive/ms 5.72, stream/tick/ms 2.06, render/present/ms 1.71, stream/spawn/ms 1.14 |
| warm-up | 4.37 | 122.2 | pipelines/wait_ms 14.9 k, pipelines/compile_ms 135, pipelines/front_ms 125, main/busy/ms 95.18, render/thread/ms 73.59, stream/drive/ms 57.19, render/busy/ms 47.38, render/present/ms 25.87, pipelines/driver_ms 10.23, terrain/update/ms 3.41, render/graph/ms 3.13, stream/spawn/ms 2.33 |
| warm-up | 4.47 | 85.9 | pipelines/wait_ms 3157, pipelines/compile_ms 161, pipelines/front_ms 134, main/busy/ms 77.82, stream/drive/ms 47.39, render/thread/ms 45.19, render/busy/ms 43.66, pipelines/driver_ms 26.92, render/graph/ms 7.69, terrain/update/ms 6.15, stream/spawn/ms 2.52, render/present/ms 1.28 |
| warm-up | 4.55 | 57.2 | pipelines/wait_ms 417, main/busy/ms 51.64, render/thread/ms 33.05, stream/drive/ms 32.94, render/busy/ms 31.92, pipelines/compile_ms 6.69, pipelines/front_ms 6.42, render/graph/ms 6.12, stream/spawn/ms 1.83, stream/activate/ms 1.19, render/present/ms 0.93, stream/tick/ms 0.77 |
| warm-up | 4.59 | 30.3 | main/busy/ms 33.42, stream/drive/ms 22.96, render/thread/ms 16.02, render/busy/ms 14.90, render/graph/ms 4.12, stream/activate/ms 1.17, terrain/update/ms 0.92, render/present/ms 0.92, terrain/select/ms 0.78, stream/tick/ms 0.74, stream/spawn/ms 0.68, bigspace/total/ms 0.36 |
| warm-up | 4.70 | 29.8 | pipelines/wait_ms 3540, pipelines/compile_ms 41.09, pipelines/front_ms 38.69, render/thread/ms 36.26, render/busy/ms 34.85, main/busy/ms 24.95, render/graph/ms 5.40, stream/drive/ms 3.85, pipelines/driver_ms 2.40, stream/spawn/ms 1.88, stream/tick/ms 1.30, render/present/ms 1.05 |
| warm-up | 5.34 | 36.3 | pipelines/wait_ms 4609, pipelines/compile_ms 184, pipelines/front_ms 178, render/thread/ms 48.14, render/busy/ms 46.87, main/busy/ms 25.47, pipelines/driver_ms 5.26, render/graph/ms 4.97, stream/drive/ms 3.94, stream/tick/ms 1.40, stream/spawn/ms 1.19, terrain/update/ms 1.06 |
| warm-up | 6.20 | 30.2 | main/busy/ms 39.23, render/thread/ms 32.39, render/busy/ms 30.67, render/graph/ms 8.03, render/present/ms 1.51, stream/drive/ms 1.21, physics/step/ms 1.12, physics/insert/ms 1.05, terrain/update/ms 0.69, bigspace/total/ms 0.62, stream/tick/ms 0.61, terrain/select/ms 0.58 |
| warm-up | 6.29 | 62.2 | pipelines/wait_ms 4841, pipelines/compile_ms 90.20, pipelines/front_ms 85.90, main/busy/ms 73.50, render/thread/ms 58.31, render/busy/ms 56.88, render/graph/ms 13.79, pipelines/driver_ms 4.31, anim/evaluate_cpu/ms 1.65, anim/evaluate/ms 1.50, stream/drive/ms 1.40, physics/insert/ms 1.33 |
| warm-up | 6.33 | 44.9 | render/thread/ms 68.30, render/busy/ms 66.98, main/busy/ms 56.33, render/graph/ms 13.43, anim/evaluate/ms 1.50, anim/evaluate_cpu/ms 1.33, stream/drive/ms 1.27, physics/insert/ms 1.10, bigspace/total/ms 1.09, render/present/ms 1.06, physics/step/ms 0.74, stream/tick/ms 0.65 |
| measure | 2.18 | 31.3 | render/thread/ms 43.41, render/busy/ms 41.96, render/graph/ms 28.23, main/busy/ms 20.24, pipelines/compile_ms 13.38, pipelines/front_ms 13.14, render/present/ms 1.27, bigspace/total/ms 0.84, terrain/update/ms 0.55, terrain/select/ms 0.49, controller/feed/ms 0.47, stream/drive/ms 0.43 |
| measure | 2.73 | 35.4 | render/thread/ms 47.31, render/busy/ms 45.78, render/graph/ms 24.75, main/busy/ms 19.47, render/present/ms 1.36, bigspace/total/ms 0.93, physics/insert/ms 0.78, terrain/update/ms 0.62, terrain/select/ms 0.56, stream/drive/ms 0.44, stream/tick/ms 0.39, controller/feed/ms 0.38 |
| measure | 5.19 | 53.3 | render/thread/ms 59.35, render/busy/ms 58.29, render/graph/ms 51.36, main/busy/ms 18.88, bigspace/total/ms 2.81, bigspace/high_precision/ms 2.17, render/present/ms 0.90, terrain/update/ms 0.53, stream/drive/ms 0.49, terrain/select/ms 0.48, stream/tick/ms 0.44, controller/feed/ms 0.43 |
| measure | 18.20 | 33.0 | main/busy/ms 39.79, render/thread/ms 29.12, render/busy/ms 27.93, stream/drive/ms 21.77, render/graph/ms 17.07, render/present/ms 1.02, bigspace/total/ms 0.79, controller/feed/ms 0.65, terrain/update/ms 0.57, terrain/select/ms 0.51, far/select/ms 0.41, stream/tick/ms 0.36 |
| measure | 23.08 | 29.5 | render/thread/ms 43.30, render/busy/ms 42.15, main/busy/ms 32.56, render/graph/ms 18.13, bigspace/total/ms 1.17, stream/drive/ms 0.96, render/present/ms 0.94, controller/feed/ms 0.87, terrain/update/ms 0.66, terrain/select/ms 0.59, anim/evaluate_cpu/ms 0.51, stream/activate/ms 0.44 |
| measure | 28.14 | 30.2 | render/thread/ms 42.89, render/busy/ms 29.70, main/busy/ms 27.80, render/graph/ms 19.93, render/present/ms 13.01, bigspace/total/ms 0.90, stream/drive/ms 0.78, controller/feed/ms 0.76, stream/tick/ms 0.72, terrain/update/ms 0.56, terrain/select/ms 0.51, far/select/ms 0.46 |
| measure | 59.05 | 31.0 | render/thread/ms 35.66, render/busy/ms 34.57, main/busy/ms 19.38, render/graph/ms 11.04, bigspace/total/ms 1.13, render/present/ms 0.93, controller/feed/ms 0.70, terrain/update/ms 0.66, terrain/select/ms 0.59, far/select/ms 0.47, stream/drive/ms 0.41, controller/move/ms 0.39 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_old.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lampsel_old\balmora_walk_night@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **77%** (min 56%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.5 W · 76 °C (max 81) · VRAM 5281 MiB · throttled 100% of samples
