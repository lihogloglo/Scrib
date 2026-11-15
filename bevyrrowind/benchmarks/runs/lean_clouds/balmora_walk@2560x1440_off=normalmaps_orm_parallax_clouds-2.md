# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: clouds,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5799 frames over 65.6 s · **median 11.01 ms (91 fps)** · mean 11.32 · p95 14.10 · p99 16.27 (1% low 61 fps) · worst 31.63 ms · 51 spikes over 1.5× median · 32.9 stutters/min (work 32.0/min)
- warm-up: 335 frames over 8.0 s, worst 2560.3 ms
- hitches (> 27.5 ms): **3** measured, 22 in warm-up
- timed GPU passes: 7.03 ms of a 11.32 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.01 ms, limit 11 ms
- **FAIL** frame_ms: 36 stutters (> 1.5x moving median of 60) in 1.09 min = 32.91/min, limit 1/min
- pass worst frame 31.63 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.080 | 2.211 | 2.673 | 0.94 | 0.181 |
| main_opaque_pass_3d | 1.471 | 1.564 | 2.393 | 0.94 | 0.054 |
| early prepass | 0.469 | 0.498 | 0.759 | 0.94 | 0.093 |
| bloom | 0.319 | 0.339 | 0.488 | 0.94 | 0.066 |
| atmosphere_luts | 0.310 | 0.329 | 0.458 | 0.94 | 0.003 |
| water_velocity | 0.286 | 0.304 | 0.359 | 0.94 | 0.002 |
| water_sspr | 0.210 | 0.223 | 0.221 | 0.94 | 0.002 |
| uniform_allocation | 0.210 | 0.223 | 0.229 | 0.94 | 0.037 |
| ssao | 0.191 | 0.203 | 0.209 | 0.94 | 0.008 |
| bin_unpacking | 0.168 | 0.178 | 0.183 | 0.94 | 0.027 |
| auxiliary_views | 0.163 | 0.173 | 0.665 | 0.94 | 0.157 |
| water_fft | 0.135 | 0.143 | 0.140 | 0.94 | 0.003 |
| atmos_froxel_inject | 0.134 | 0.142 | 0.138 | 0.94 | 0.002 |
| upscaling | 0.122 | 0.130 | 0.156 | 0.94 | 0.004 |
| clouds_composite | 0.099 | 0.105 | 0.104 | 0.94 | 0.002 |
| tonemapping | 0.086 | 0.091 | 0.092 | 0.94 | 0.003 |
| main_transmissive_pass_3d | 0.085 | 0.091 | 0.168 | 0.94 | 0.009 |
| clustering | 0.065 | 0.069 | 0.070 | 0.94 | 0.020 |
| render_sky | 0.052 | 0.055 | 0.055 | 0.94 | 0.002 |
| lightprobe_irradiance_map | 0.046 | 0.444 | 0.757 | 0.10 | 0.001 |
| clouds_resolve | 0.045 | 0.048 | 0.050 | 0.94 | 0.001 |
| surface_trail | 0.042 | 0.060 | 0.057 | 0.69 | 0.002 |
| early_mesh_preprocessing | 0.041 | 0.044 | 0.048 | 0.94 | 0.005 |
| water_snapshot | 0.033 | 0.036 | 0.044 | 0.94 | 0.002 |
| atmos_froxel_integrate | 0.026 | 0.028 | 0.030 | 0.94 | 0.001 |
| lightprobe_radiance_map | 0.024 | 0.235 | 0.238 | 0.10 | 0.004 |
| clouds_trace | 0.024 | 0.026 | 0.027 | 0.94 | 0.001 |
| water_mips | 0.023 | 0.025 | 0.024 | 0.94 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.021 | 0.023 | 0.94 | 0.006 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.032 | 0.94 | 0.111 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.94 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.94 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.94 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.94 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 28.954 | 0.000 | 19949.72 | 167906.9 |
| render/thread/ms | 10.101 | 12.725 | 26.24 | 58575.0 |
| render/busy/ms | 9.482 | 12.160 | 25.35 | 54984.2 |
| main/busy/ms | 8.649 | 10.731 | 24.67 | 50156.8 |
| render/graph/ms | 5.304 | 6.958 | 15.06 | 30755.7 |
| render/present/ms | 0.517 | 1.283 | 13.85 | 3000.0 |
| bigspace/total/ms | 0.504 | 1.058 | 2.98 | 2921.8 |
| pipelines/compile_ms | 0.489 | 0.000 | 184.27 | 2838.1 |
| pipelines/front_ms | 0.472 | 0.000 | 176.40 | 2736.6 |
| terrain/update/ms | 0.299 | 0.386 | 1.88 | 1731.4 |
| controller/feed/ms | 0.280 | 0.376 | 0.73 | 1623.7 |
| terrain/select/ms | 0.265 | 0.341 | 0.71 | 1539.4 |
| stream/drive/ms | 0.251 | 0.363 | 17.08 | 1457.2 |
| bigspace/low_precision/ms | 0.204 | 0.518 | 2.16 | 1183.2 |
| stream/tick/ms | 0.196 | 0.274 | 1.03 | 1137.4 |
| far/select/ms | 0.195 | 0.270 | 0.66 | 1129.7 |
| anim/evaluate_cpu/ms | 0.162 | 0.207 | 0.47 | 938.2 |
| controller/move/ms | 0.133 | 0.263 | 0.61 | 774.1 |
| anim/evaluate/ms | 0.133 | 0.257 | 2.00 | 769.5 |
| bigspace/high_precision/ms | 0.132 | 0.316 | 2.11 | 767.2 |
| render/acquire/ms | 0.102 | 0.133 | 1.72 | 590.7 |
| physics/insert/ms | 0.054 | 0.433 | 1.01 | 315.8 |
| physics/step/ms | 0.051 | 0.091 | 0.59 | 296.0 |
| weather/map/ms | 0.026 | 0.036 | 0.45 | 150.6 |
| pipelines/driver_ms | 0.018 | 0.000 | 7.87 | 101.5 |
| anim/apply/ms | 0.012 | 0.022 | 0.11 | 66.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.18 | 61.3 |
| controller/avoid/ms | 0.010 | 0.014 | 0.37 | 60.8 |
| script/update/ms | 0.007 | 0.010 | 0.15 | 41.9 |
| script/ui/ms | 0.005 | 0.007 | 0.16 | 29.0 |
| stream/despawn/ms | 0.005 | 0.000 | 1.10 | 29.0 |
| script/input/ms | 0.005 | 0.006 | 0.12 | 28.2 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 26.1 |
| controller/write/ms | 0.004 | 0.005 | 0.06 | 21.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.71 | 15.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.7 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 19 | 0 |
| anim/a2/count | count | 1 | 14 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 34 | 19 |
| controller/movers/count | count | 20 | 21 | 19 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 528 |
| crowd/c0/meshes/count | count | 353 | 646 | 135 |
| crowd/c1/count | count | 146 | 155 | 149 |
| crowd/c1/joints/count | count | 5507 | 5937 | 5668 |
| crowd/c1/meshes/count | count | 1171 | 1281 | 1206 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 13.7 k | 6179 |
| crowd/joints_uploaded/count | count | 4240 | 6643 | 6179 |
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
| frame/work/ms | ms | 9.28 | 25.35 | 8.55 |
| gpu/timed/ms | ms | 8.14 | 10.97 | 7.90 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 602.13 M | 603.31 M | 603.31 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 820 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 820 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1284 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3418 | 5116 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1284 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3418 | 5116 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 49 | 309 | 39 |
| phases/shadow/bins | count | 3755 | 5570 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9361 | 13.0 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 130 | 310 | 39 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1107 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 204 | 208 | 208 |
| pipelines/waiting | count | 0 | 41 | 0 |
| preprocess/bind_groups | count | 11 | 787 | 6 |
| preprocess/bind_groups_created | count | 11 | 787 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 221 | 331 | 215 |
| process/cpu_usage | % | 25 | 43.30 | 27.88 |
| process/mem_usage | GiB | 5.20 | 5.50 | 5.49 |
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
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.24 | 52.59 | 37.98 |
| system/mem_usage | % | 58.99 | 60.01 | 59.96 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 107 | 361 | 99 |
| views/all/bins | count | 6433 | 8348 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.2 k | 25.4 k | 13.3 k |
| views/all/pipelines | count | 184 | 362 | 95 |
| views/all/pipelines_distinct | count | 81 | 92 | 77 |
| views/all/work_items | count | 17.3 k | 24.1 k | 13.6 k |
| views/cascade/batch_sets | count | 47 | 60 | 39 |
| views/cascade/bins | count | 2590 | 3168 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8158 | 10.2 k | 5663 |
| views/cascade/pipelines | count | 47 | 60 | 39 |
| views/cascade/work_items | count | 8158 | 10.2 k | 5663 |
| views/lamp_face/batch_sets | count | 0 | 256 | 0 |
| views/lamp_face/bins | count | 1059 | 2630 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1187 | 2993 | 0 |
| views/lamp_face/pipelines | count | 80 | 257 | 0 |
| views/lamp_face/work_items | count | 0 | 2735 | 0 |
| views/main/batch_sets | count | 58 | 62 | 60 |
| views/main/bins | count | 2866 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8478 | 13.0 k | 7594 |
| views/main/pipelines | count | 54 | 58 | 56 |
| views/main/work_items | count | 8633 | 13.2 k | 7902 |
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
| anim/bones_written/count | count | 52.36 | 600 | 303.7 k |
| anim/events/count | count | 0.10 | 3 | 553 |
| assets/Image/added | count | 0.11 | 43 | 658 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.75 | 152 | 4321 |
| assets/Mesh/removed | count | 0.84 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 534 |
| assets/StandardMaterial/modified | count | 3.76 | 18 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.24 | 1525 | 13.0 k |
| physics/inserts | count | 0.56 | 154 | 3263 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 161 | 2335 |
| physics/steps | count | 0.68 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 3 | 21 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 18 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 5838 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 63 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1280 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.5 ms; slowest 25, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2560.3 | main/busy/ms 2534, surface/advance_cpu_ms 69.32, surface/cell_upload_ms 1.03, stream/drive/ms 0.34, weather/map/ms 0.19, bigspace/total/ms 0.11, stream/tick/ms 0.08, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.03, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.38 | 817.0 | main/busy/ms 2568, render/thread/ms 784, render/busy/ms 668, render/acquire/ms 113, surface/advance_cpu_ms 69.32, render/graph/ms 4.97, render/present/ms 2.26, stream/drive/ms 1.11, surface/cell_upload_ms 1.03, stream/tick/ms 0.53, terrain/update/ms 0.42, weather/map/ms 0.33 |
| warm-up | 3.47 | 93.6 | pipelines/wait_ms 1649, render/thread/ms 795, render/busy/ms 679, render/acquire/ms 113, pipelines/compile_ms 101, pipelines/front_ms 93.39, main/busy/ms 49.75, terrain/update/ms 18.03, render/graph/ms 8.34, pipelines/driver_ms 7.94, render/present/ms 3.08, stream/drive/ms 1.38 |
| warm-up | 3.70 | 95.8 | pipelines/compile_ms 230, pipelines/front_ms 221, main/busy/ms 61.86, render/thread/ms 58.53, render/busy/ms 55.76, pipelines/driver_ms 8.49, render/graph/ms 4.52, render/present/ms 2.59, stream/drive/ms 1.63, stream/tick/ms 1.06, terrain/update/ms 0.67, terrain/select/ms 0.56 |
| warm-up | 3.92 | 102.3 | render/thread/ms 74.89, render/busy/ms 73.26, main/busy/ms 62.30, stream/drive/ms 39.52, render/graph/ms 6.79, stream/tick/ms 1.75, render/present/ms 1.39, terrain/update/ms 1.16, stream/spawn/ms 1.05, terrain/select/ms 0.96, bigspace/total/ms 0.33, render/acquire/ms 0.24 |
| warm-up | 3.99 | 48.3 | pipelines/wait_ms 2664, pipelines/compile_ms 255, pipelines/front_ms 240, render/thread/ms 43.27, render/busy/ms 41.77, main/busy/ms 28.05, pipelines/driver_ms 15.84, stream/drive/ms 8.44, render/graph/ms 3.83, render/present/ms 1.25, terrain/update/ms 0.96, stream/tick/ms 0.85 |
| warm-up | 4.03 | 35.1 | pipelines/wait_ms 4408, pipelines/compile_ms 114, pipelines/front_ms 108, render/thread/ms 56.36, render/busy/ms 54.59, main/busy/ms 42.67, stream/drive/ms 21.41, pipelines/driver_ms 6.11, render/graph/ms 5.07, stream/spawn/ms 1.86, render/present/ms 1.54, stream/tick/ms 0.87 |
| warm-up | 4.07 | 43.5 | pipelines/wait_ms 3108, render/thread/ms 63.26, pipelines/compile_ms 61.96, render/busy/ms 60.91, pipelines/front_ms 59.25, main/busy/ms 38.94, stream/drive/ms 21.99, render/graph/ms 7.53, pipelines/driver_ms 2.72, stream/spawn/ms 2.29, render/present/ms 2.10, terrain/update/ms 0.86 |
| warm-up | 4.10 | 29.8 | pipelines/wait_ms 1094, render/thread/ms 58.11, render/busy/ms 56.29, main/busy/ms 22.91, pipelines/compile_ms 14.17, pipelines/front_ms 13.21, render/graph/ms 6.51, stream/drive/ms 3.45, render/present/ms 1.57, stream/spawn/ms 1.47, pipelines/driver_ms 0.96, terrain/update/ms 0.95 |
| warm-up | 4.16 | 58.0 | render/thread/ms 64.45, render/busy/ms 62.55, main/busy/ms 24.98, render/graph/ms 8.16, stream/drive/ms 4.65, stream/spawn/ms 2.45, render/present/ms 1.59, terrain/update/ms 0.96, terrain/select/ms 0.80, stream/tick/ms 0.78, stream/activate/ms 0.54, bigspace/total/ms 0.44 |
| warm-up | 4.25 | 66.5 | pipelines/wait_ms 6589, pipelines/compile_ms 226, pipelines/front_ms 210, render/thread/ms 51.90, render/busy/ms 49.70, main/busy/ms 45.40, stream/drive/ms 31.77, pipelines/driver_ms 15.63, render/graph/ms 10.59, render/present/ms 1.99, stream/activate/ms 1.09, terrain/update/ms 0.97 |
| warm-up | 4.35 | 43.6 | pipelines/wait_ms 801, main/busy/ms 44.19, stream/drive/ms 33.00, render/thread/ms 19.33, render/busy/ms 18.29, pipelines/compile_ms 17.27, pipelines/front_ms 15.99, render/graph/ms 3.85, pipelines/driver_ms 1.28, stream/spawn/ms 1.17, render/present/ms 0.83, stream/tick/ms 0.82 |
| warm-up | 4.42 | 58.4 | pipelines/wait_ms 598, pipelines/compile_ms 116, pipelines/front_ms 113, main/busy/ms 59.07, stream/drive/ms 45.52, render/thread/ms 23.65, render/busy/ms 22.68, pipelines/driver_ms 3.64, render/graph/ms 2.87, stream/spawn/ms 2.07, terrain/update/ms 1.04, stream/tick/ms 0.86 |
| warm-up | 4.49 | 60.3 | pipelines/wait_ms 5422, main/busy/ms 51.89, pipelines/compile_ms 37.77, pipelines/front_ms 35.52, stream/drive/ms 31.95, render/thread/ms 30.55, render/busy/ms 29.24, render/graph/ms 3.97, stream/spawn/ms 2.47, pipelines/driver_ms 2.24, render/present/ms 1.08, stream/tick/ms 0.69 |
| warm-up | 4.55 | 44.8 | pipelines/wait_ms 965, pipelines/compile_ms 54.41, pipelines/front_ms 52.04, main/busy/ms 48.36, stream/drive/ms 31.31, render/thread/ms 26.52, render/busy/ms 25.15, render/graph/ms 5.15, pipelines/driver_ms 2.37, stream/spawn/ms 1.52, render/present/ms 1.17, stream/activate/ms 1.07 |
| warm-up | 4.67 | 28.2 | pipelines/wait_ms 785, pipelines/compile_ms 153, pipelines/front_ms 149, render/thread/ms 31.21, render/busy/ms 29.46, main/busy/ms 23.48, stream/drive/ms 6.03, pipelines/driver_ms 4.66, render/graph/ms 4.31, stream/spawn/ms 3.15, render/present/ms 1.46, terrain/update/ms 1.12 |
| warm-up | 5.26 | 28.1 | render/thread/ms 43.17, render/busy/ms 41.70, main/busy/ms 22.15, render/graph/ms 5.60, stream/drive/ms 4.45, stream/spawn/ms 1.88, render/present/ms 1.18, stream/tick/ms 0.68, terrain/update/ms 0.67, bigspace/total/ms 0.63, terrain/select/ms 0.57, bigspace/high_precision/ms 0.36 |
| warm-up | 5.32 | 35.3 | pipelines/wait_ms 2903, pipelines/compile_ms 145, pipelines/front_ms 138, render/thread/ms 44.47, render/busy/ms 43.19, main/busy/ms 25.06, pipelines/driver_ms 6.94, render/graph/ms 4.49, stream/drive/ms 4.31, stream/spawn/ms 2.00, stream/tick/ms 1.32, render/present/ms 0.98 |
| warm-up | 5.57 | 35.6 | render/thread/ms 43.47, render/busy/ms 41.82, main/busy/ms 33.74, render/graph/ms 4.99, stream/drive/ms 4.64, stream/spawn/ms 2.13, stream/tick/ms 1.57, render/present/ms 1.24, bigspace/total/ms 1.13, terrain/update/ms 0.92, terrain/select/ms 0.79, bigspace/high_precision/ms 0.50 |
| warm-up | 5.89 | 42.1 | pipelines/wait_ms 1715, main/busy/ms 65.57, render/thread/ms 49.56, render/busy/ms 48.01, pipelines/compile_ms 20.95, pipelines/front_ms 19.58, render/graph/ms 7.29, stream/drive/ms 2.55, physics/insert/ms 1.39, pipelines/driver_ms 1.37, stream/spawn/ms 1.25, render/present/ms 1.19 |
| warm-up | 5.94 | 48.4 | pipelines/wait_ms 4146, main/busy/ms 70.16, pipelines/compile_ms 69.81, pipelines/front_ms 68.18, render/thread/ms 55.61, render/busy/ms 54.16, stream/drive/ms 13.68, render/graph/ms 6.71, pipelines/driver_ms 1.63, physics/insert/ms 1.33, render/present/ms 1.12, bigspace/total/ms 0.87 |
| warm-up | 5.97 | 30.1 | pipelines/wait_ms 4146, pipelines/compile_ms 69.81, pipelines/front_ms 68.18, render/thread/ms 50.56, main/busy/ms 41.47, render/busy/ms 40.75, stream/drive/ms 13.48, render/present/ms 9.59, render/graph/ms 5.43, pipelines/driver_ms 1.63, physics/insert/ms 1.39, stream/tick/ms 0.84 |
| measure | 0.77 | 28.2 | pipelines/wait_ms 8387, pipelines/compile_ms 146, pipelines/front_ms 140, main/busy/ms 33.53, render/thread/ms 26.18, render/busy/ms 24.53, stream/drive/ms 13.54, render/graph/ms 10.57, pipelines/driver_ms 5.56, render/present/ms 1.07, physics/insert/ms 0.75, bigspace/total/ms 0.62 |
| measure | 59.05 | 31.6 | render/thread/ms 38.15, render/busy/ms 36.99, main/busy/ms 25.71, render/graph/ms 9.47, render/present/ms 0.95, bigspace/total/ms 0.87, controller/feed/ms 0.86, stream/drive/ms 0.86, terrain/update/ms 0.79, far/select/ms 0.78, stream/tick/ms 0.74, terrain/select/ms 0.70 |
| measure | 62.97 | 27.7 | render/thread/ms 37.18, render/busy/ms 22.63, main/busy/ms 18.48, render/present/ms 14.34, render/graph/ms 11.43, bigspace/total/ms 1.01, controller/feed/ms 0.90, stream/drive/ms 0.66, stream/tick/ms 0.59, terrain/update/ms 0.55, far/select/ms 0.55, terrain/select/ms 0.49 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_clouds\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds-2 --bench-baseline none --off normalmaps,orm,parallax,clouds
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **85%** (min 61%) · 2580 MHz (min 2580) · memory 8001 MHz · 81.4 W · 77 °C (max 83) · VRAM 6295 MiB · throttled 100% of samples
