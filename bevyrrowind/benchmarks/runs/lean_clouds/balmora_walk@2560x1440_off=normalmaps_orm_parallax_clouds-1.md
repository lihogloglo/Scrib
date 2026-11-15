# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: clouds,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5785 frames over 65.7 s · **median 11.08 ms (90 fps)** · mean 11.35 · p95 13.95 · p99 15.92 (1% low 63 fps) · worst 31.80 ms · 40 spikes over 1.5× median · 29.2 stutters/min (work 23.8/min)
- warm-up: 481 frames over 9.3 s, worst 2500.1 ms
- hitches (> 27.7 ms): **5** measured, 29 in warm-up
- timed GPU passes: 7.10 ms of a 11.35 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.08 ms, limit 11 ms
- **FAIL** frame_ms: 32 stutters (> 1.5x moving median of 60) in 1.09 min = 29.25/min, limit 1/min
- pass worst frame 31.80 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.089 | 2.200 | 2.569 | 0.95 | 0.182 |
| main_opaque_pass_3d | 1.473 | 1.551 | 2.203 | 0.95 | 0.061 |
| early prepass | 0.478 | 0.504 | 0.786 | 0.95 | 0.092 |
| bloom | 0.321 | 0.338 | 0.488 | 0.95 | 0.065 |
| atmosphere_luts | 0.316 | 0.332 | 0.504 | 0.95 | 0.003 |
| water_velocity | 0.287 | 0.302 | 0.462 | 0.95 | 0.002 |
| uniform_allocation | 0.213 | 0.224 | 0.228 | 0.95 | 0.040 |
| water_sspr | 0.211 | 0.222 | 0.221 | 0.95 | 0.002 |
| ssao | 0.192 | 0.202 | 0.210 | 0.95 | 0.008 |
| auxiliary_views | 0.188 | 0.198 | 0.728 | 0.95 | 0.176 |
| bin_unpacking | 0.167 | 0.176 | 0.183 | 0.95 | 0.028 |
| atmos_froxel_inject | 0.134 | 0.141 | 0.138 | 0.95 | 0.002 |
| water_fft | 0.134 | 0.141 | 0.138 | 0.95 | 0.003 |
| upscaling | 0.122 | 0.129 | 0.155 | 0.95 | 0.004 |
| clouds_composite | 0.100 | 0.105 | 0.105 | 0.95 | 0.002 |
| tonemapping | 0.088 | 0.092 | 0.092 | 0.95 | 0.004 |
| main_transmissive_pass_3d | 0.087 | 0.092 | 0.171 | 0.95 | 0.008 |
| clustering | 0.065 | 0.068 | 0.070 | 0.95 | 0.020 |
| render_sky | 0.053 | 0.056 | 0.055 | 0.95 | 0.002 |
| clouds_resolve | 0.046 | 0.048 | 0.050 | 0.95 | 0.001 |
| lightprobe_irradiance_map | 0.046 | 0.442 | 0.757 | 0.10 | 0.001 |
| early_mesh_preprocessing | 0.041 | 0.043 | 0.048 | 0.95 | 0.006 |
| surface_trail | 0.041 | 0.058 | 0.057 | 0.70 | 0.002 |
| water_snapshot | 0.035 | 0.037 | 0.045 | 0.95 | 0.002 |
| atmos_froxel_integrate | 0.026 | 0.028 | 0.031 | 0.95 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.246 | 0.560 | 0.10 | 0.004 |
| clouds_trace | 0.025 | 0.026 | 0.027 | 0.95 | 0.001 |
| water_mips | 0.023 | 0.025 | 0.024 | 0.95 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.022 | 0.023 | 0.95 | 0.006 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.031 | 0.95 | 0.109 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.95 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.95 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.95 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.95 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.157 | 12.605 | 32.66 | 58755.5 |
| render/busy/ms | 9.541 | 11.990 | 29.37 | 55195.3 |
| pipelines/wait_ms | 8.792 | 0.000 | 7827.82 | 50861.4 |
| main/busy/ms | 8.645 | 10.601 | 24.68 | 50013.9 |
| render/graph/ms | 5.314 | 6.873 | 15.22 | 30741.4 |
| render/present/ms | 0.521 | 1.306 | 10.17 | 3012.2 |
| bigspace/total/ms | 0.496 | 1.062 | 2.78 | 2867.2 |
| terrain/update/ms | 0.303 | 0.390 | 1.97 | 1752.9 |
| controller/feed/ms | 0.282 | 0.383 | 0.91 | 1630.3 |
| terrain/select/ms | 0.270 | 0.346 | 0.73 | 1562.4 |
| pipelines/compile_ms | 0.269 | 0.000 | 154.70 | 1558.7 |
| pipelines/front_ms | 0.260 | 0.000 | 148.33 | 1505.5 |
| stream/drive/ms | 0.257 | 0.377 | 16.90 | 1486.7 |
| stream/tick/ms | 0.201 | 0.284 | 1.58 | 1163.8 |
| bigspace/low_precision/ms | 0.196 | 0.510 | 2.42 | 1136.3 |
| far/select/ms | 0.194 | 0.282 | 0.62 | 1125.0 |
| anim/evaluate_cpu/ms | 0.168 | 0.215 | 0.58 | 969.6 |
| controller/move/ms | 0.161 | 0.365 | 0.65 | 929.7 |
| anim/evaluate/ms | 0.133 | 0.264 | 2.12 | 769.0 |
| bigspace/high_precision/ms | 0.132 | 0.334 | 2.09 | 763.1 |
| render/acquire/ms | 0.095 | 0.125 | 1.36 | 548.0 |
| physics/step/ms | 0.049 | 0.088 | 0.64 | 286.2 |
| physics/insert/ms | 0.049 | 0.442 | 0.93 | 285.7 |
| weather/map/ms | 0.026 | 0.035 | 0.20 | 147.9 |
| anim/apply/ms | 0.011 | 0.021 | 0.10 | 65.6 |
| controller/avoid/ms | 0.011 | 0.014 | 0.13 | 62.0 |
| weather/eval/ms | 0.011 | 0.015 | 0.31 | 61.9 |
| pipelines/driver_ms | 0.009 | 0.000 | 6.37 | 53.2 |
| script/update/ms | 0.007 | 0.009 | 0.13 | 40.7 |
| stream/despawn/ms | 0.005 | 0.000 | 1.47 | 29.6 |
| script/input/ms | 0.005 | 0.006 | 0.14 | 28.8 |
| script/ui/ms | 0.005 | 0.006 | 0.04 | 26.3 |
| stream/spawn/ms | 0.005 | 0.000 | 1.06 | 26.0 |
| controller/write/ms | 0.004 | 0.004 | 0.06 | 20.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.72 | 15.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.57 | 5.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.19 | 4.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.08 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 32 | 19 |
| controller/movers/count | count | 20 | 21 | 19 |
| crowd/c0/count | count | 27 | 42 | 14 |
| crowd/c0/joints/count | count | 1306 | 2204 | 650 |
| crowd/c0/meshes/count | count | 363 | 634 | 185 |
| crowd/c1/count | count | 146 | 154 | 147 |
| crowd/c1/joints/count | count | 5507 | 5885 | 5546 |
| crowd/c1/meshes/count | count | 1160 | 1243 | 1156 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 9951 | 6179 |
| crowd/joints_uploaded/count | count | 4244 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.33 | 29.37 | 9.11 |
| gpu/timed/ms | ms | 8.20 | 10.66 | 8.04 |
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
| phases/alpha_mask3d/items | count | 805 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 805 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1286 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3395 | 5116 | 3174 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1286 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3395 | 5116 | 3174 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 49 | 296 | 41 |
| phases/shadow/bins | count | 3813 | 5478 | 2239 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9578 | 12.9 k | 5694 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 135 | 302 | 41 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 930 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 204 | 208 | 208 |
| pipelines/waiting | count | 0 | 16 | 0 |
| preprocess/bind_groups | count | 11 | 761 | 10 |
| preprocess/bind_groups_created | count | 11 | 761 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 223 | 344 | 215 |
| process/cpu_usage | % | 24.82 | 41.83 | 28.85 |
| process/mem_usage | GiB | 5.20 | 5.50 | 5.50 |
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
| started_load_count |  | 4766 | 4859 | 4859 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.61 | 55.68 | 35.80 |
| system/mem_usage | % | 58.99 | 60.01 | 59.93 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 108 | 348 | 101 |
| views/all/bins | count | 6472 | 8277 | 5719 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.4 k | 13.3 k |
| views/all/pipelines | count | 189 | 356 | 97 |
| views/all/pipelines_distinct | count | 82 | 91 | 77 |
| views/all/work_items | count | 17.2 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 48 | 63 | 41 |
| views/cascade/bins | count | 2597 | 3210 | 2239 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8147 | 10.2 k | 5694 |
| views/cascade/pipelines | count | 48 | 63 | 41 |
| views/cascade/work_items | count | 8147 | 10.2 k | 5694 |
| views/lamp_face/batch_sets | count | 0 | 243 | 0 |
| views/lamp_face/bins | count | 1088 | 2523 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1227 | 2876 | 0 |
| views/lamp_face/pipelines | count | 84 | 243 | 0 |
| views/lamp_face/work_items | count | 0 | 2649 | 0 |
| views/main/batch_sets | count | 60 | 62 | 60 |
| views/main/bins | count | 2868 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8398 | 13.0 k | 7584 |
| views/main/pipelines | count | 56 | 58 | 56 |
| views/main/work_items | count | 8566 | 13.2 k | 7892 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 53.09 | 215 | 307.1 k |
| anim/events/count | count | 0.09 | 3 | 537 |
| assets/Image/added | count | 0.11 | 43 | 619 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.72 | 152 | 4148 |
| assets/Mesh/removed | count | 0.84 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 497 |
| assets/StandardMaterial/modified | count | 3.76 | 22 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.94 | 1492 | 5430 |
| physics/inserts | count | 0.53 | 108 | 3071 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 16 | 378 |
| stream/upload/bytes | bytes | 5852 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1283 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.7 ms; slowest 34, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2500.1 | main/busy/ms 2476, surface/advance_cpu_ms 67.49, surface/cell_upload_ms 0.97, stream/drive/ms 0.28, weather/map/ms 0.14, bigspace/total/ms 0.11, stream/tick/ms 0.07, bigspace/high_precision/ms 0.07, weather/eval/ms 0.02, script/update/ms 0.02, anim/evaluate/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.04 | 535.0 | main/busy/ms 2510, render/thread/ms 501, render/busy/ms 380, render/acquire/ms 118, surface/advance_cpu_ms 67.49, render/graph/ms 5.29, render/present/ms 2.40, stream/drive/ms 1.24, surface/cell_upload_ms 0.97, terrain/update/ms 0.74, stream/tick/ms 0.67, terrain/select/ms 0.48 |
| warm-up | 3.26 | 226.7 | pipelines/wait_ms 1667, render/thread/ms 512, render/busy/ms 391, pipelines/compile_ms 236, pipelines/front_ms 220, render/acquire/ms 118, main/busy/ms 50.45, terrain/update/ms 19.00, pipelines/driver_ms 15.98, render/graph/ms 9.19, render/present/ms 3.28, stream/drive/ms 1.56 |
| warm-up | 3.50 | 83.6 | pipelines/compile_ms 237, pipelines/front_ms 229, render/thread/ms 66.99, render/busy/ms 65.06, main/busy/ms 64.10, pipelines/driver_ms 7.83, render/graph/ms 4.09, render/present/ms 1.69, stream/drive/ms 1.60, stream/tick/ms 1.30, terrain/update/ms 0.62, terrain/select/ms 0.52 |
| warm-up | 3.54 | 28.9 | pipelines/wait_ms 229, pipelines/compile_ms 50.01, pipelines/front_ms 45.95, render/thread/ms 32.39, render/busy/ms 31.07, main/busy/ms 21.84, terrain/update/ms 5.43, pipelines/driver_ms 4.06, render/graph/ms 3.27, stream/drive/ms 2.02, render/present/ms 1.02, stream/tick/ms 0.95 |
| warm-up | 3.67 | 71.1 | render/thread/ms 56.26, render/busy/ms 54.45, main/busy/ms 46.62, stream/drive/ms 31.93, terrain/update/ms 5.57, render/graph/ms 4.68, render/present/ms 1.58, stream/tick/ms 1.18, stream/spawn/ms 0.82, terrain/select/ms 0.72, bigspace/total/ms 0.36, render/acquire/ms 0.23 |
| warm-up | 3.74 | 60.1 | pipelines/wait_ms 2052, pipelines/compile_ms 217, pipelines/front_ms 198, render/thread/ms 40.83, render/busy/ms 38.94, main/busy/ms 34.78, pipelines/driver_ms 19.02, stream/drive/ms 17.78, render/graph/ms 5.53, stream/tick/ms 1.92, render/present/ms 1.66, terrain/update/ms 0.83 |
| warm-up | 3.83 | 66.0 | pipelines/wait_ms 3680, pipelines/compile_ms 98.55, pipelines/front_ms 92.51, render/thread/ms 52.44, render/busy/ms 50.91, main/busy/ms 38.39, stream/drive/ms 17.86, pipelines/driver_ms 6.04, render/graph/ms 3.45, render/present/ms 1.23, stream/spawn/ms 1.05, terrain/update/ms 0.99 |
| warm-up | 3.86 | 32.8 | pipelines/wait_ms 4734, pipelines/compile_ms 110, pipelines/front_ms 102, render/thread/ms 63.61, render/busy/ms 62.18, main/busy/ms 23.38, pipelines/driver_ms 7.59, stream/drive/ms 4.61, render/graph/ms 3.33, stream/spawn/ms 2.39, stream/tick/ms 1.05, render/present/ms 1.04 |
| warm-up | 3.92 | 28.8 | main/busy/ms 46.89, render/thread/ms 38.97, render/busy/ms 37.70, stream/drive/ms 27.48, render/graph/ms 4.08, stream/spawn/ms 2.48, render/present/ms 1.02, stream/tick/ms 0.80, terrain/update/ms 0.80, terrain/select/ms 0.66, stream/activate/ms 0.52, bigspace/total/ms 0.29 |
| warm-up | 3.97 | 55.0 | main/busy/ms 46.46, render/thread/ms 44.97, render/busy/ms 43.61, stream/drive/ms 27.43, render/graph/ms 5.42, stream/spawn/ms 1.98, render/present/ms 1.08, stream/activate/ms 1.06, terrain/update/ms 0.81, stream/tick/ms 0.77, terrain/select/ms 0.67, bigspace/total/ms 0.30 |
| warm-up | 4.01 | 34.4 | pipelines/wait_ms 313, pipelines/compile_ms 133, pipelines/front_ms 129, render/thread/ms 50.87, render/busy/ms 49.09, main/busy/ms 28.91, render/graph/ms 7.99, pipelines/driver_ms 3.84, stream/drive/ms 3.47, render/present/ms 1.52, stream/activate/ms 1.06, terrain/update/ms 0.82 |
| warm-up | 4.04 | 36.4 | pipelines/wait_ms 2947, pipelines/compile_ms 176, pipelines/front_ms 170, render/thread/ms 53.21, render/busy/ms 50.78, main/busy/ms 41.01, stream/drive/ms 14.90, render/graph/ms 14.72, pipelines/driver_ms 6.74, render/present/ms 2.23, stream/spawn/ms 1.16, stream/activate/ms 0.95 |
| warm-up | 4.07 | 29.5 | pipelines/wait_ms 2634, main/busy/ms 75.83, stream/drive/ms 56.02, render/thread/ms 49.23, render/busy/ms 46.28, pipelines/compile_ms 43.47, pipelines/front_ms 40.57, render/graph/ms 12.50, pipelines/driver_ms 2.90, render/present/ms 2.27, stream/tick/ms 1.60, stream/spawn/ms 1.28 |
| warm-up | 4.15 | 76.4 | pipelines/wait_ms 1551, pipelines/compile_ms 76.59, pipelines/front_ms 74.26, main/busy/ms 65.25, render/thread/ms 50.78, render/busy/ms 48.64, stream/drive/ms 44.21, terrain/update/ms 4.56, render/graph/ms 4.31, pipelines/driver_ms 2.32, stream/tick/ms 1.51, render/present/ms 1.44 |
| warm-up | 4.24 | 71.3 | main/busy/ms 67.03, stream/drive/ms 47.03, render/thread/ms 39.50, render/busy/ms 38.12, render/graph/ms 4.79, terrain/update/ms 2.73, stream/spawn/ms 2.47, render/present/ms 1.14, stream/tick/ms 0.78, terrain/select/ms 0.64, stream/activate/ms 0.45, bigspace/total/ms 0.37 |
| warm-up | 4.32 | 56.7 | pipelines/wait_ms 5286, pipelines/compile_ms 188, pipelines/front_ms 167, main/busy/ms 61.80, stream/drive/ms 41.69, render/thread/ms 37.16, render/busy/ms 35.45, pipelines/driver_ms 20.31, render/graph/ms 5.28, stream/spawn/ms 2.07, render/present/ms 1.49, stream/tick/ms 1.21 |
| warm-up | 4.36 | 29.5 | main/busy/ms 35.57, stream/drive/ms 23.79, render/thread/ms 17.20, render/busy/ms 16.02, render/graph/ms 4.20, stream/activate/ms 1.31, render/present/ms 0.99, stream/tick/ms 0.78, physics/insert/ms 0.70, stream/spawn/ms 0.65, terrain/update/ms 0.58, bigspace/total/ms 0.55 |
| warm-up | 4.75 | 37.1 | pipelines/wait_ms 4231, main/busy/ms 40.62, render/thread/ms 38.74, render/busy/ms 37.03, pipelines/compile_ms 34.49, pipelines/front_ms 32.90, stream/drive/ms 7.05, render/graph/ms 5.41, pipelines/driver_ms 1.59, render/present/ms 1.18, stream/tick/ms 0.81, terrain/update/ms 0.55 |
| warm-up | 5.05 | 27.9 | pipelines/wait_ms 1032, pipelines/compile_ms 131, pipelines/front_ms 125, render/thread/ms 37.91, render/busy/ms 36.00, main/busy/ms 22.76, render/graph/ms 7.46, pipelines/driver_ms 6.24, stream/drive/ms 4.27, render/present/ms 1.46, stream/spawn/ms 1.41, bigspace/total/ms 0.87 |
| warm-up | 5.10 | 30.4 | render/thread/ms 29.92, render/busy/ms 28.96, main/busy/ms 18.54, render/graph/ms 4.08, stream/drive/ms 3.25, stream/spawn/ms 1.59, stream/tick/ms 0.96, render/present/ms 0.80, terrain/update/ms 0.66, bigspace/total/ms 0.60, terrain/select/ms 0.55, bigspace/high_precision/ms 0.28 |
| warm-up | 5.29 | 35.2 | pipelines/wait_ms 7079, pipelines/compile_ms 45.48, pipelines/front_ms 42.77, render/thread/ms 41.20, render/busy/ms 39.63, main/busy/ms 28.18, render/graph/ms 5.34, stream/drive/ms 4.10, pipelines/driver_ms 2.72, stream/spawn/ms 2.09, bigspace/total/ms 1.59, render/present/ms 1.33 |
| warm-up | 5.43 | 30.0 | render/thread/ms 46.56, render/busy/ms 44.68, main/busy/ms 26.77, render/graph/ms 6.47, stream/drive/ms 4.66, stream/spawn/ms 2.34, render/present/ms 1.59, stream/tick/ms 1.21, terrain/update/ms 0.74, bigspace/total/ms 0.65, terrain/select/ms 0.60, render/acquire/ms 0.29 |
| warm-up | 5.46 | 28.9 | render/thread/ms 41.57, render/busy/ms 40.03, main/busy/ms 39.37, stream/drive/ms 19.13, render/graph/ms 5.38, stream/spawn/ms 2.06, stream/tick/ms 1.44, render/present/ms 1.29, terrain/update/ms 0.77, terrain/select/ms 0.63, bigspace/total/ms 0.52, render/acquire/ms 0.24 |
| warm-up | 5.50 | 42.8 | pipelines/wait_ms 4038, pipelines/compile_ms 159, pipelines/front_ms 155, render/thread/ms 51.79, render/busy/ms 50.39, main/busy/ms 36.84, stream/drive/ms 18.72, render/graph/ms 5.19, pipelines/driver_ms 3.61, stream/spawn/ms 2.11, render/present/ms 1.21, stream/tick/ms 1.11 |
| warm-up | 5.64 | 30.6 | main/busy/ms 35.09, render/thread/ms 21.47, render/busy/ms 20.34, stream/drive/ms 14.88, render/graph/ms 4.63, stream/tick/ms 1.48, render/present/ms 0.95, terrain/update/ms 0.73, bigspace/total/ms 0.61, terrain/select/ms 0.61, bigspace/high_precision/ms 0.32, render/acquire/ms 0.18 |
| warm-up | 7.17 | 35.1 | main/busy/ms 49.69, render/thread/ms 29.24, render/busy/ms 27.53, render/graph/ms 11.40, render/present/ms 1.44, stream/drive/ms 1.26, bigspace/total/ms 0.65, stream/tick/ms 0.56, terrain/update/ms 0.55, terrain/select/ms 0.48, anim/evaluate_cpu/ms 0.46, far/select/ms 0.45 |
| warm-up | 7.21 | 37.7 | render/thread/ms 38.55, main/busy/ms 36.88, render/busy/ms 36.83, render/graph/ms 9.24, render/present/ms 1.45, stream/drive/ms 1.16, bigspace/total/ms 0.68, stream/tick/ms 0.61, terrain/update/ms 0.57, far/select/ms 0.48, terrain/select/ms 0.48, anim/evaluate_cpu/ms 0.44 |
| warm-up | 8.46 | 28.0 | render/thread/ms 36.38, render/busy/ms 35.27, main/busy/ms 27.96, render/graph/ms 9.49, render/present/ms 0.94, bigspace/total/ms 0.64, terrain/update/ms 0.54, terrain/select/ms 0.48, stream/drive/ms 0.46, stream/tick/ms 0.39, far/select/ms 0.32, controller/feed/ms 0.32 |
| measure | 0.39 | 31.8 | pipelines/wait_ms 1211, render/thread/ms 41.44, render/busy/ms 39.96, main/busy/ms 28.27, render/graph/ms 21.86, pipelines/compile_ms 20.66, pipelines/front_ms 19.50, render/present/ms 1.29, pipelines/driver_ms 1.16, bigspace/total/ms 0.94, controller/feed/ms 0.80, anim/evaluate_cpu/ms 0.71 |
| measure | 0.78 | 28.7 | main/busy/ms 33.52, render/thread/ms 29.81, render/busy/ms 28.51, stream/drive/ms 14.37, render/graph/ms 12.73, render/present/ms 1.06, bigspace/total/ms 0.87, physics/insert/ms 0.74, controller/feed/ms 0.62, terrain/update/ms 0.61, terrain/select/ms 0.54, far/select/ms 0.54 |
| measure | 2.72 | 30.9 | render/thread/ms 40.72, render/busy/ms 39.68, main/busy/ms 18.45, render/graph/ms 18.00, physics/insert/ms 0.88, render/present/ms 0.88, bigspace/total/ms 0.87, terrain/update/ms 0.55, terrain/select/ms 0.49, stream/drive/ms 0.41, controller/feed/ms 0.40, stream/tick/ms 0.36 |
| measure | 4.87 | 27.9 | main/busy/ms 29.31, render/thread/ms 25.65, render/busy/ms 24.52, stream/drive/ms 11.43, render/graph/ms 8.06, render/present/ms 0.95, bigspace/total/ms 0.82, terrain/update/ms 0.56, controller/feed/ms 0.54, terrain/select/ms 0.50, far/select/ms 0.43, bigspace/low_precision/ms 0.39 |
| measure | 19.27 | 28.0 | render/thread/ms 37.57, render/busy/ms 36.35, render/graph/ms 20.94, main/busy/ms 18.05, render/present/ms 1.03, physics/insert/ms 0.87, bigspace/total/ms 0.84, controller/feed/ms 0.69, terrain/update/ms 0.59, stream/drive/ms 0.54, terrain/select/ms 0.53, stream/tick/ms 0.47 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_clouds\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds-1 --bench-baseline none --off normalmaps,orm,parallax,clouds
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **84%** (min 57%) · 2575 MHz (min 2565) · memory 8001 MHz · 80.8 W · 76 °C (max 82) · VRAM 6291 MiB · throttled 100% of samples
