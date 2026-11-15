# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4368 frames over 65.6 s · **median 11.12 ms (90 fps)** · mean 15.03 · p95 40.67 · p99 82.22 (1% low 12 fps) · worst 122.14 ms · 447 spikes over 1.5× median · 20.1 stutters/min (work 16.5/min)
- warm-up: 336 frames over 14.6 s, worst 3648.0 ms
- hitches (> 27.8 ms): **390** measured, 161 in warm-up
- timed GPU passes: 10.01 ms of a 15.03 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.12 ms, limit 11 ms
- **FAIL** frame_ms: 22 stutters (> 1.5x moving median of 60) in 1.09 min = 20.11/min, limit 1/min
- **FAIL** worst frame 122.14 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.248 | 2.375 | 4.822 | 0.95 | 0.183 |
| main_opaque_pass_3d | 2.002 | 2.115 | 5.660 | 0.95 | 0.054 |
| early prepass | 0.919 | 0.970 | 4.108 | 0.95 | 0.094 |
| auxiliary_views | 0.670 | 0.708 | 3.805 | 0.95 | 0.133 |
| bin_unpacking | 0.456 | 0.482 | 2.491 | 0.95 | 0.027 |
| water_fft | 0.455 | 0.481 | 4.069 | 0.95 | 0.003 |
| bloom | 0.346 | 0.365 | 0.691 | 0.95 | 0.065 |
| atmosphere_luts | 0.332 | 0.351 | 0.697 | 0.95 | 0.003 |
| water_velocity | 0.289 | 0.306 | 0.701 | 0.95 | 0.001 |
| uniform_allocation | 0.276 | 0.291 | 0.812 | 0.95 | 0.037 |
| water_sspr | 0.254 | 0.268 | 0.674 | 0.95 | 0.002 |
| early_mesh_preprocessing | 0.221 | 0.234 | 1.803 | 0.95 | 0.005 |
| ssao | 0.206 | 0.217 | 0.354 | 0.95 | 0.008 |
| atmos_froxel_inject | 0.164 | 0.173 | 0.294 | 0.95 | 0.001 |
| upscaling | 0.129 | 0.136 | 0.157 | 0.95 | 0.004 |
| clouds_trace | 0.126 | 0.133 | 0.173 | 0.95 | 0.001 |
| clouds_composite | 0.112 | 0.119 | 0.113 | 0.95 | 0.002 |
| water_snapshot | 0.110 | 0.116 | 0.751 | 0.95 | 0.002 |
| main_transmissive_pass_3d | 0.106 | 0.112 | 0.354 | 0.95 | 0.008 |
| clustering | 0.100 | 0.106 | 0.284 | 0.95 | 0.021 |
| tonemapping | 0.095 | 0.100 | 0.102 | 0.95 | 0.004 |
| render_sky | 0.054 | 0.057 | 0.055 | 0.95 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.479 | 0.979 | 0.11 | 0.001 |
| clouds_resolve | 0.047 | 0.050 | 0.052 | 0.95 | 0.001 |
| surface_trail | 0.045 | 0.063 | 0.059 | 0.71 | 0.002 |
| atmos_froxel_integrate | 0.031 | 0.032 | 0.034 | 0.95 | 0.001 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.95 | 0.003 |
| water_mips | 0.028 | 0.029 | 0.081 | 0.95 | 0.002 |
| lightprobe_radiance_map | 0.027 | 0.254 | 0.561 | 0.11 | 0.004 |
| per_view_shadow_pass | 0.023 | 0.024 | 0.033 | 0.95 | 0.006 |
| main_transparent_pass_3d | 0.018 | 0.019 | 0.032 | 0.95 | 0.110 |
| atmosphere_environment | 0.011 | 0.012 | 0.011 | 0.95 | 0.004 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.95 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 0.95 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.008 | 0.95 | 0.002 |
| lightprobe_copy | 0.003 | 0.028 | 0.072 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.015 | 0.033 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.012 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.007 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 13.817 | 39.250 | 120.92 | 60353.7 |
| render/busy/ms | 9.821 | 13.486 | 73.70 | 42896.4 |
| main/busy/ms | 8.606 | 10.969 | 30.76 | 37592.1 |
| render/graph/ms | 5.545 | 7.757 | 21.38 | 24220.4 |
| render/acquire/ms | 3.449 | 25.090 | 104.82 | 15064.0 |
| render/present/ms | 0.548 | 1.330 | 11.56 | 2393.3 |
| bigspace/total/ms | 0.488 | 1.000 | 2.77 | 2132.9 |
| terrain/update/ms | 0.309 | 0.396 | 1.83 | 1349.6 |
| controller/feed/ms | 0.295 | 0.388 | 0.78 | 1290.6 |
| terrain/select/ms | 0.274 | 0.351 | 0.90 | 1197.1 |
| stream/drive/ms | 0.255 | 0.379 | 21.19 | 1115.8 |
| far/select/ms | 0.197 | 0.274 | 0.98 | 861.6 |
| stream/tick/ms | 0.190 | 0.265 | 1.60 | 831.5 |
| bigspace/low_precision/ms | 0.183 | 0.459 | 2.50 | 798.3 |
| anim/evaluate_cpu/ms | 0.157 | 0.205 | 0.54 | 684.5 |
| anim/evaluate/ms | 0.156 | 0.269 | 1.94 | 680.3 |
| controller/move/ms | 0.155 | 0.325 | 0.70 | 676.2 |
| pipelines/compile_ms | 0.154 | 0.000 | 134.80 | 674.4 |
| pipelines/front_ms | 0.149 | 0.000 | 129.81 | 652.3 |
| bigspace/high_precision/ms | 0.142 | 0.393 | 2.45 | 620.8 |
| physics/insert/ms | 0.061 | 0.466 | 0.98 | 265.8 |
| physics/step/ms | 0.058 | 0.133 | 0.76 | 254.3 |
| pipelines/wait_ms | 0.037 | 0.000 | 122.79 | 162.6 |
| weather/map/ms | 0.025 | 0.035 | 0.21 | 107.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.23 | 48.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.27 | 47.0 |
| anim/apply/ms | 0.010 | 0.019 | 0.19 | 42.6 |
| stream/despawn/ms | 0.007 | 0.000 | 1.09 | 31.4 |
| script/update/ms | 0.007 | 0.011 | 0.17 | 31.1 |
| stream/spawn/ms | 0.006 | 0.000 | 1.10 | 24.2 |
| script/ui/ms | 0.005 | 0.007 | 0.08 | 23.9 |
| script/input/ms | 0.005 | 0.007 | 0.05 | 22.3 |
| pipelines/driver_ms | 0.005 | 0.000 | 4.98 | 22.2 |
| controller/write/ms | 0.004 | 0.004 | 0.04 | 15.5 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.70 | 12.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.47 | 5.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.18 | 4.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.09 | 1.4 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 569 | 579 | 578 |
| anim/characters/count | count | 571 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 17 |
| controller/movers/count | count | 19 | 22 | 17 |
| crowd/c0/count | count | 20 | 42 | 12 |
| crowd/c0/joints/count | count | 1060 | 2218 | 538 |
| crowd/c0/meshes/count | count | 346 | 633 | 129 |
| crowd/c1/count | count | 149 | 155 | 149 |
| crowd/c1/joints/count | count | 5640 | 5937 | 5658 |
| crowd/c1/meshes/count | count | 1187 | 1281 | 1212 |
| crowd/c2/count | count | 401 | 417 | 417 |
| crowd/c2/joints/count | count | 6438 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1836 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 194 | 236 | 222 |
| crowd/joints_extracted/count | count | 4372 | 10.0 k | 6135 |
| crowd/joints_uploaded/count | count | 4317 | 6643 | 6135 |
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
| frame/work/ms | ms | 9.35 | 75.07 | 9.53 |
| gpu/timed/ms | ms | 8.40 | 75.07 | 8.87 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.1 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 288 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 744 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 288 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 744 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 23 | 24 | 23 |
| phases/opaque3d/bins | count | 1306 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3236 | 5128 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 22 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 23 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1306 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3236 | 5128 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 22 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 45 | 293 | 39 |
| phases/shadow/bins | count | 2572 | 5464 | 2207 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6943 | 12.4 k | 5658 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 45 | 323 | 39 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2606 | 3268 | 2346 |
| physics/items | count | 91 | 476 | 19 |
| physics/pending | count | 0 | 356 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 204 | 208 | 208 |
| pipelines/waiting | count | 0 | 3 | 0 |
| preprocess/bind_groups | count | 10 | 755 | 6 |
| preprocess/bind_groups_created | count | 10 | 755 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 219 | 327 | 215 |
| process/cpu_usage | % | 23.08 | 39.82 | 35.10 |
| process/mem_usage | GiB | 6.36 | 6.57 | 6.56 |
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
| started_load_count |  | 4797 | 4859 | 4859 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 420 | 528 | 396 |
| statics/materials_bound | count | 420 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.30 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 320 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.48 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1125 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.10 | 67.42 | 35.73 |
| system/mem_usage | % | 67.44 | 80.40 | 67.72 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2832 | 2938 | 2938 |
| textures/images | count | 2600 | 2649 | 2649 |
| views/all/batch_sets | count | 103 | 345 | 99 |
| views/all/bins | count | 5886 | 8160 | 5687 |
| views/all/count | count | 4 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 13.8 k | 25.0 k | 13.3 k |
| views/all/pipelines | count | 99 | 377 | 95 |
| views/all/pipelines_distinct | count | 79 | 93 | 77 |
| views/all/work_items | count | 14.0 k | 23.7 k | 13.6 k |
| views/cascade/batch_sets | count | 45 | 61 | 39 |
| views/cascade/bins | count | 2545 | 3162 | 2207 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 6943 | 9956 | 5658 |
| views/cascade/pipelines | count | 45 | 61 | 39 |
| views/cascade/work_items | count | 6943 | 9956 | 5658 |
| views/lamp_face/batch_sets | count | 0 | 240 | 0 |
| views/lamp_face/bins | count | 0 | 2577 | 0 |
| views/lamp_face/count | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 2900 | 0 |
| views/lamp_face/pipelines | count | 0 | 267 | 0 |
| views/lamp_face/work_items | count | 0 | 2557 | 0 |
| views/main/batch_sets | count | 60 | 62 | 60 |
| views/main/bins | count | 2966 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7828 | 13.0 k | 7594 |
| views/main/pipelines | count | 56 | 58 | 56 |
| views/main/work_items | count | 8139 | 13.2 k | 7902 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.2 k | 37.8 k | 30.7 k |
| world/point_lights | count | 289 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 40.38 | 257 | 176.4 k |
| anim/events/count | count | 0.12 | 4 | 524 |
| assets/Image/added | count | 0.14 | 43 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.93 | 168 | 4075 |
| assets/Mesh/removed | count | 1.12 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 466 |
| assets/StandardMaterial/modified | count | 4.41 | 22 | 19.3 k |
| assets/StandardMaterial/removed | count | 0.09 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 6 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.08 | 1485 | 4716 |
| physics/inserts | count | 0.52 | 87 | 2283 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.49 | 145 | 2124 |
| physics/steps | count | 0.85 | 3 | 3705 |
| physics/steps_dropped | count | 0.05 | 4 | 235 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 8 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.09 | 22 | 378 |
| stream/upload/bytes | bytes | 7750 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 44 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1699 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.8 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 3.65 | 3648.0 | main/busy/ms 3619, surface/advance_cpu_ms 79.82, stream/drive/ms 0.79, surface/cell_upload_ms 0.79, stream/tick/ms 0.38, weather/map/ms 0.13, bigspace/total/ms 0.10, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 4.69 | 1044.3 | main/busy/ms 3666, render/thread/ms 995, render/busy/ms 858, render/acquire/ms 134, surface/advance_cpu_ms 79.82, render/graph/ms 4.77, render/present/ms 2.27, stream/drive/ms 1.96, stream/tick/ms 1.12, surface/cell_upload_ms 0.79, terrain/update/ms 0.63, terrain/select/ms 0.42 |
| warm-up | 4.91 | 214.6 | pipelines/wait_ms 1208, render/thread/ms 1007, render/busy/ms 870, pipelines/compile_ms 226, pipelines/front_ms 212, render/acquire/ms 134, main/busy/ms 64.54, terrain/update/ms 26.22, pipelines/driver_ms 13.81, render/graph/ms 9.28, render/present/ms 3.30, stream/drive/ms 1.76 |
| warm-up | 5.21 | 136.4 | main/busy/ms 97.49, render/thread/ms 87.19, render/busy/ms 84.41, render/graph/ms 6.49, stream/drive/ms 2.65, stream/tick/ms 2.12, render/present/ms 1.94, terrain/update/ms 1.00, render/acquire/ms 0.84, terrain/select/ms 0.81, bigspace/total/ms 0.38, bigspace/high_precision/ms 0.13 |
| warm-up | 5.45 | 109.3 | render/thread/ms 79.86, main/busy/ms 79.29, render/busy/ms 76.95, stream/drive/ms 54.18, render/graph/ms 7.45, render/present/ms 2.61, stream/tick/ms 2.55, stream/spawn/ms 1.23, terrain/update/ms 1.06, terrain/select/ms 0.84, bigspace/total/ms 0.43, render/acquire/ms 0.31 |
| warm-up | 6.00 | 84.5 | main/busy/ms 76.54, stream/drive/ms 49.01, render/thread/ms 46.32, render/busy/ms 44.97, render/graph/ms 3.74, stream/spawn/ms 2.28, stream/tick/ms 1.49, render/present/ms 1.05, terrain/update/ms 0.77, bigspace/total/ms 0.65, terrain/select/ms 0.63, render/acquire/ms 0.29 |
| warm-up | 6.17 | 139.1 | pipelines/wait_ms 5833, pipelines/compile_ms 238, pipelines/front_ms 222, main/busy/ms 102, render/thread/ms 69.01, render/busy/ms 65.90, stream/drive/ms 60.61, pipelines/driver_ms 16.29, render/graph/ms 10.39, stream/spawn/ms 5.74, render/present/ms 2.68, stream/tick/ms 2.34 |
| warm-up | 6.32 | 78.7 | pipelines/wait_ms 12.6 k, pipelines/compile_ms 236, pipelines/front_ms 223, main/busy/ms 86.25, render/thread/ms 75.65, render/busy/ms 73.29, stream/drive/ms 52.17, pipelines/driver_ms 13.16, render/graph/ms 7.33, stream/spawn/ms 2.16, render/present/ms 2.03, terrain/update/ms 1.32 |
| measure | 2.96 | 115.5 | render/thread/ms 136, render/acquire/ms 93.45, render/busy/ms 40.89, main/busy/ms 26.96, render/graph/ms 21.23, stream/drive/ms 1.45, render/present/ms 1.39, bigspace/total/ms 0.93, terrain/update/ms 0.72, stream/spawn/ms 0.70, physics/insert/ms 0.67, terrain/select/ms 0.63 |
| measure | 6.82 | 78.9 | render/thread/ms 138, render/acquire/ms 107, render/busy/ms 30.33, main/busy/ms 22.98, render/graph/ms 18.02, stream/drive/ms 1.74, render/present/ms 1.00, controller/feed/ms 0.95, stream/tick/ms 0.91, bigspace/total/ms 0.84, terrain/update/ms 0.69, stream/spawn/ms 0.69 |
| measure | 7.50 | 122.1 | render/thread/ms 182, render/acquire/ms 152, render/busy/ms 28.31, main/busy/ms 21.97, render/graph/ms 18.22, render/present/ms 1.41, stream/drive/ms 1.40, bigspace/total/ms 1.08, physics/insert/ms 0.79, terrain/update/ms 0.69, controller/feed/ms 0.68, physics/step/ms 0.66 |
| measure | 7.73 | 86.7 | render/thread/ms 158, render/acquire/ms 124, render/busy/ms 33.54, main/busy/ms 20.79, render/graph/ms 20.14, physics/insert/ms 1.41, stream/drive/ms 1.13, render/present/ms 0.92, bigspace/total/ms 0.83, terrain/update/ms 0.74, terrain/select/ms 0.67, controller/feed/ms 0.62 |
| measure | 7.81 | 80.4 | render/thread/ms 165, render/acquire/ms 132, render/busy/ms 31.99, main/busy/ms 23.43, render/graph/ms 18.24, stream/drive/ms 1.54, physics/insert/ms 1.46, stream/despawn/ms 1.07, render/present/ms 1.04, bigspace/total/ms 0.84, controller/feed/ms 0.65, terrain/update/ms 0.62 |
| measure | 7.90 | 87.4 | render/thread/ms 165, render/acquire/ms 134, render/busy/ms 30.36, main/busy/ms 20.66, render/graph/ms 19.69, physics/insert/ms 1.18, render/present/ms 1.15, stream/drive/ms 0.97, bigspace/total/ms 0.80, terrain/update/ms 0.66, controller/feed/ms 0.63, terrain/select/ms 0.59 |
| measure | 7.98 | 82.2 | render/thread/ms 166, render/acquire/ms 136, render/busy/ms 29.62, render/graph/ms 19.62, main/busy/ms 18.76, render/present/ms 1.07, bigspace/total/ms 0.85, physics/insert/ms 0.80, terrain/update/ms 0.65, terrain/select/ms 0.59, stream/drive/ms 0.50, controller/feed/ms 0.46 |
| measure | 8.28 | 80.8 | render/thread/ms 153, render/acquire/ms 127, render/busy/ms 24.50, main/busy/ms 21.36, render/graph/ms 13.96, physics/insert/ms 1.19, render/present/ms 1.14, bigspace/total/ms 1.01, controller/feed/ms 0.69, stream/drive/ms 0.68, terrain/update/ms 0.68, terrain/select/ms 0.61 |
| measure | 9.46 | 81.4 | render/thread/ms 147, render/acquire/ms 118, render/busy/ms 27.87, main/busy/ms 22.09, render/graph/ms 16.16, render/present/ms 1.15, bigspace/total/ms 0.93, physics/insert/ms 0.89, stream/drive/ms 0.82, terrain/update/ms 0.70, terrain/select/ms 0.63, controller/feed/ms 0.52 |
| measure | 9.54 | 78.7 | render/thread/ms 157, render/acquire/ms 128, render/busy/ms 28.29, main/busy/ms 23.40, render/graph/ms 15.75, stream/drive/ms 1.56, render/present/ms 1.20, physics/insert/ms 1.12, bigspace/total/ms 0.82, stream/despawn/ms 0.71, terrain/update/ms 0.64, terrain/select/ms 0.58 |
| measure | 9.62 | 86.7 | render/thread/ms 162, render/acquire/ms 129, render/busy/ms 32.34, main/busy/ms 24.02, render/graph/ms 18.06, stream/drive/ms 1.76, render/present/ms 1.26, stream/despawn/ms 1.21, physics/insert/ms 0.87, bigspace/total/ms 0.83, terrain/update/ms 0.80, controller/feed/ms 0.64 |
| measure | 9.78 | 84.4 | render/thread/ms 157, render/acquire/ms 129, render/busy/ms 27.64, main/busy/ms 24.72, render/graph/ms 14.96, stream/drive/ms 2.47, render/present/ms 1.16, physics/insert/ms 0.90, stream/despawn/ms 0.87, terrain/update/ms 0.80, bigspace/total/ms 0.78, terrain/select/ms 0.73 |
| measure | 9.87 | 83.0 | render/thread/ms 165, render/acquire/ms 133, render/busy/ms 30.55, main/busy/ms 22.94, render/graph/ms 17.56, stream/drive/ms 1.34, render/present/ms 1.21, physics/insert/ms 1.21, bigspace/total/ms 1.07, stream/despawn/ms 0.78, terrain/update/ms 0.76, terrain/select/ms 0.69 |
| measure | 10.03 | 92.0 | render/thread/ms 166, render/acquire/ms 135, render/busy/ms 29.58, main/busy/ms 22.37, render/graph/ms 16.68, physics/insert/ms 1.18, render/present/ms 1.10, stream/drive/ms 0.89, bigspace/total/ms 0.84, terrain/update/ms 0.63, controller/feed/ms 0.61, terrain/select/ms 0.57 |
| measure | 10.12 | 85.8 | render/thread/ms 175, render/acquire/ms 146, render/busy/ms 28.46, main/busy/ms 18.74, render/graph/ms 16.75, render/present/ms 1.19, bigspace/total/ms 0.86, physics/insert/ms 0.79, controller/feed/ms 0.65, terrain/update/ms 0.58, terrain/select/ms 0.52, stream/drive/ms 0.50 |
| measure | 10.20 | 78.5 | render/thread/ms 162, render/acquire/ms 134, render/busy/ms 26.76, main/busy/ms 20.50, render/graph/ms 15.85, render/present/ms 1.26, physics/insert/ms 1.09, stream/drive/ms 0.88, bigspace/total/ms 0.81, terrain/update/ms 0.61, controller/feed/ms 0.59, terrain/select/ms 0.55 |
| measure | 10.36 | 86.8 | render/thread/ms 161, render/acquire/ms 132, main/busy/ms 27.76, render/busy/ms 27.53, render/graph/ms 14.08, stream/drive/ms 1.80, bigspace/total/ms 1.35, physics/insert/ms 1.22, render/present/ms 1.13, controller/feed/ms 0.97, stream/deactivate/ms 0.85, terrain/update/ms 0.82 |
| measure | 10.52 | 78.6 | render/thread/ms 146, render/acquire/ms 111, render/busy/ms 33.51, main/busy/ms 24.44, render/graph/ms 16.24, stream/drive/ms 2.26, terrain/update/ms 1.51, render/present/ms 1.40, stream/despawn/ms 1.27, bigspace/total/ms 0.80, stream/tick/ms 0.74, controller/feed/ms 0.73 |
| measure | 10.60 | 84.6 | render/thread/ms 160, render/acquire/ms 128, render/busy/ms 30.99, main/busy/ms 23.23, render/graph/ms 17.00, stream/drive/ms 1.92, render/present/ms 1.38, terrain/update/ms 1.32, stream/despawn/ms 1.32, bigspace/total/ms 0.81, controller/feed/ms 0.63, terrain/select/ms 0.54 |
| measure | 10.77 | 87.3 | render/thread/ms 163, render/acquire/ms 130, render/busy/ms 32.30, main/busy/ms 23.30, render/graph/ms 19.03, stream/drive/ms 1.65, render/present/ms 1.01, controller/feed/ms 1.01, physics/insert/ms 1.00, terrain/update/ms 0.83, terrain/select/ms 0.75, bigspace/total/ms 0.74 |
| measure | 10.85 | 82.2 | render/thread/ms 167, render/acquire/ms 134, render/busy/ms 32.22, main/busy/ms 24.18, render/graph/ms 17.40, controller/feed/ms 1.17, render/present/ms 1.06, bigspace/total/ms 1.03, physics/insert/ms 0.97, stream/drive/ms 0.70, terrain/update/ms 0.67, terrain/select/ms 0.61 |
| measure | 10.93 | 84.6 | render/thread/ms 164, render/acquire/ms 132, render/busy/ms 31.53, main/busy/ms 22.85, render/graph/ms 17.73, render/present/ms 1.12, bigspace/total/ms 1.06, controller/feed/ms 0.95, terrain/update/ms 0.69, physics/insert/ms 0.68, stream/drive/ms 0.67, terrain/select/ms 0.62 |
| measure | 11.02 | 83.3 | render/thread/ms 165, render/acquire/ms 135, render/busy/ms 29.03, main/busy/ms 18.13, render/graph/ms 17.73, render/present/ms 1.15, controller/feed/ms 0.77, bigspace/total/ms 0.71, terrain/update/ms 0.69, terrain/select/ms 0.62, physics/insert/ms 0.50, stream/drive/ms 0.48 |
| measure | 11.10 | 88.4 | render/thread/ms 169, render/acquire/ms 140, render/busy/ms 27.73, main/busy/ms 20.72, render/graph/ms 15.78, render/present/ms 1.22, bigspace/total/ms 0.84, physics/insert/ms 0.79, controller/feed/ms 0.67, stream/drive/ms 0.66, terrain/update/ms 0.63, terrain/select/ms 0.57 |
| measure | 11.27 | 84.0 | render/thread/ms 159, render/acquire/ms 131, render/busy/ms 26.67, main/busy/ms 22.73, render/graph/ms 15.86, physics/insert/ms 1.22, render/present/ms 1.14, bigspace/total/ms 0.83, terrain/update/ms 0.77, stream/drive/ms 0.73, terrain/select/ms 0.70, controller/feed/ms 0.69 |
| measure | 11.35 | 83.2 | render/thread/ms 165, render/acquire/ms 135, render/busy/ms 28.91, main/busy/ms 21.92, render/graph/ms 17.50, render/present/ms 1.26, physics/insert/ms 1.17, bigspace/total/ms 0.77, stream/drive/ms 0.76, terrain/update/ms 0.65, controller/feed/ms 0.63, terrain/select/ms 0.60 |
| measure | 11.43 | 82.3 | render/thread/ms 163, render/acquire/ms 133, render/busy/ms 28.34, main/busy/ms 21.81, render/graph/ms 16.10, render/present/ms 1.24, physics/insert/ms 0.99, stream/drive/ms 0.79, bigspace/total/ms 0.79, controller/feed/ms 0.69, terrain/update/ms 0.69, terrain/select/ms 0.62 |
| measure | 11.52 | 85.7 | render/thread/ms 166, render/acquire/ms 136, render/busy/ms 28.19, main/busy/ms 20.02, render/graph/ms 15.90, render/present/ms 1.09, stream/drive/ms 0.78, terrain/update/ms 0.74, bigspace/total/ms 0.71, controller/feed/ms 0.69, terrain/select/ms 0.67, stream/tick/ms 0.57 |
| measure | 11.60 | 84.9 | render/thread/ms 168, render/acquire/ms 140, render/busy/ms 27.51, main/busy/ms 18.75, render/graph/ms 16.76, render/present/ms 1.23, bigspace/total/ms 0.68, terrain/update/ms 0.64, terrain/select/ms 0.58, controller/feed/ms 0.56, physics/insert/ms 0.49, stream/drive/ms 0.46 |
| measure | 11.68 | 81.1 | render/thread/ms 163, render/acquire/ms 136, render/busy/ms 25.51, main/busy/ms 20.63, render/graph/ms 15.46, render/present/ms 1.24, bigspace/total/ms 0.71, controller/feed/ms 0.67, terrain/update/ms 0.67, terrain/select/ms 0.61, physics/insert/ms 0.53, stream/drive/ms 0.53 |
| measure | 11.77 | 90.4 | render/thread/ms 169, render/acquire/ms 139, render/busy/ms 28.32, main/busy/ms 20.66, render/graph/ms 16.07, render/present/ms 1.24, bigspace/total/ms 0.71, controller/feed/ms 0.69, terrain/update/ms 0.67, terrain/select/ms 0.61, stream/drive/ms 0.53, stream/tick/ms 0.47 |
| measure | 11.93 | 90.1 | render/thread/ms 159, render/acquire/ms 129, render/busy/ms 28.43, main/busy/ms 19.96, render/graph/ms 16.95, render/present/ms 1.69, bigspace/total/ms 0.78, controller/feed/ms 0.76, terrain/update/ms 0.71, terrain/select/ms 0.65, stream/drive/ms 0.45, physics/insert/ms 0.45 |
| measure | 12.03 | 94.4 | render/thread/ms 182, render/acquire/ms 154, render/busy/ms 26.64, main/busy/ms 21.43, render/graph/ms 14.68, render/present/ms 1.56, bigspace/total/ms 0.87, terrain/update/ms 0.81, terrain/select/ms 0.73, controller/feed/ms 0.70, stream/drive/ms 0.60, stream/tick/ms 0.53 |
| measure | 12.11 | 85.2 | render/thread/ms 177, render/acquire/ms 149, render/busy/ms 26.91, main/busy/ms 24.56, render/graph/ms 14.96, render/present/ms 0.99, bigspace/total/ms 0.99, terrain/update/ms 0.82, terrain/select/ms 0.73, controller/feed/ms 0.62, stream/drive/ms 0.61, stream/tick/ms 0.54 |
| measure | 12.20 | 85.7 | render/thread/ms 168, render/acquire/ms 135, render/busy/ms 32.06, main/busy/ms 25.36, render/graph/ms 18.67, pipelines/compile_ms 10.29, pipelines/front_ms 10.01, render/present/ms 1.16, bigspace/total/ms 0.99, terrain/update/ms 0.91, terrain/select/ms 0.82, controller/feed/ms 0.56 |
| measure | 12.28 | 83.5 | render/thread/ms 167, render/acquire/ms 135, render/busy/ms 29.97, main/busy/ms 22.61, render/graph/ms 17.36, pipelines/compile_ms 10.29, pipelines/front_ms 10.01, terrain/update/ms 2.34, render/present/ms 1.22, bigspace/total/ms 0.92, physics/insert/ms 0.90, terrain/select/ms 0.85 |
| measure | 12.37 | 84.9 | render/thread/ms 166, render/acquire/ms 140, render/busy/ms 24.86, main/busy/ms 22.31, render/graph/ms 14.21, terrain/update/ms 2.22, render/present/ms 1.28, bigspace/total/ms 0.99, physics/insert/ms 0.98, terrain/select/ms 0.74, controller/feed/ms 0.72, anim/evaluate_cpu/ms 0.49 |
| measure | 12.61 | 87.6 | render/thread/ms 158, render/acquire/ms 132, render/busy/ms 25.57, main/busy/ms 20.78, render/graph/ms 14.18, render/present/ms 1.12, bigspace/total/ms 0.83, terrain/update/ms 0.75, controller/feed/ms 0.70, terrain/select/ms 0.68, physics/insert/ms 0.60, stream/drive/ms 0.50 |
| measure | 12.77 | 93.4 | render/thread/ms 163, render/acquire/ms 137, render/busy/ms 25.02, main/busy/ms 19.98, render/graph/ms 14.59, physics/insert/ms 1.12, render/present/ms 1.09, bigspace/total/ms 0.76, terrain/update/ms 0.72, terrain/select/ms 0.65, controller/feed/ms 0.64, stream/drive/ms 0.43 |
| measure | 12.86 | 85.5 | render/thread/ms 177, render/acquire/ms 148, render/busy/ms 27.88, main/busy/ms 22.46, render/graph/ms 16.86, render/present/ms 1.15, bigspace/total/ms 1.04, physics/insert/ms 0.91, controller/feed/ms 0.74, terrain/update/ms 0.64, terrain/select/ms 0.59, stream/drive/ms 0.46 |
| measure | 12.95 | 92.4 | render/thread/ms 175, render/acquire/ms 147, render/busy/ms 27.04, main/busy/ms 21.84, render/graph/ms 16.45, render/present/ms 1.14, bigspace/total/ms 0.92, physics/insert/ms 0.88, terrain/update/ms 0.83, controller/feed/ms 0.82, stream/drive/ms 0.80, terrain/select/ms 0.75 |
| measure | 13.04 | 90.2 | render/thread/ms 180, render/acquire/ms 152, render/busy/ms 27.10, main/busy/ms 21.35, render/graph/ms 16.58, stream/drive/ms 1.34, physics/insert/ms 1.21, render/present/ms 1.11, terrain/update/ms 0.78, controller/feed/ms 0.70, terrain/select/ms 0.70, bigspace/total/ms 0.69 |
| measure | 13.13 | 88.5 | render/thread/ms 176, render/acquire/ms 145, render/busy/ms 30.05, main/busy/ms 22.78, render/graph/ms 18.37, stream/drive/ms 1.68, stream/despawn/ms 1.20, render/present/ms 1.12, physics/insert/ms 0.92, bigspace/total/ms 0.77, controller/feed/ms 0.64, anim/evaluate_cpu/ms 0.63 |
| measure | 13.22 | 86.9 | render/thread/ms 173, render/acquire/ms 138, render/busy/ms 33.63, main/busy/ms 22.88, render/graph/ms 19.31, stream/drive/ms 1.60, render/present/ms 1.20, stream/despawn/ms 1.14, bigspace/total/ms 0.80, terrain/update/ms 0.62, controller/feed/ms 0.57, terrain/select/ms 0.56 |
| measure | 13.30 | 85.7 | render/thread/ms 170, render/acquire/ms 139, render/busy/ms 30.03, main/busy/ms 24.17, render/graph/ms 16.43, render/present/ms 1.20, stream/drive/ms 0.97, bigspace/total/ms 0.90, controller/feed/ms 0.75, terrain/update/ms 0.74, terrain/select/ms 0.52, stream/despawn/ms 0.51 |
| measure | 13.39 | 90.3 | render/thread/ms 174, render/acquire/ms 147, render/busy/ms 25.70, main/busy/ms 24.50, render/graph/ms 14.40, render/present/ms 1.22, bigspace/total/ms 1.02, terrain/update/ms 0.84, controller/feed/ms 0.73, terrain/select/ms 0.61, anim/evaluate_cpu/ms 0.61, stream/drive/ms 0.50 |
| measure | 13.48 | 89.0 | render/thread/ms 177, render/acquire/ms 144, render/busy/ms 31.09, main/busy/ms 21.57, render/graph/ms 15.87, render/present/ms 1.18, bigspace/total/ms 0.85, terrain/update/ms 0.75, terrain/select/ms 0.67, controller/feed/ms 0.55, anim/evaluate_cpu/ms 0.52, physics/insert/ms 0.42 |
| measure | 13.57 | 86.1 | render/thread/ms 173, render/acquire/ms 135, render/busy/ms 36.84, main/busy/ms 19.06, render/graph/ms 17.79, render/present/ms 1.11, bigspace/total/ms 0.77, terrain/update/ms 0.64, terrain/select/ms 0.58, controller/feed/ms 0.53, physics/insert/ms 0.40, stream/drive/ms 0.40 |
| measure | 13.65 | 80.1 | render/thread/ms 164, render/acquire/ms 130, render/busy/ms 32.65, main/busy/ms 20.96, render/graph/ms 17.16, render/present/ms 1.10, bigspace/total/ms 1.03, terrain/update/ms 0.63, stream/drive/ms 0.62, controller/feed/ms 0.60, terrain/select/ms 0.57, stream/tick/ms 0.52 |
| measure | 13.73 | 85.1 | render/thread/ms 163, render/acquire/ms 136, render/busy/ms 25.40, main/busy/ms 23.06, render/graph/ms 14.76, render/present/ms 1.10, bigspace/total/ms 1.04, terrain/update/ms 0.82, terrain/select/ms 0.74, controller/feed/ms 0.71, stream/drive/ms 0.64, stream/tick/ms 0.54 |
| measure | 13.82 | 92.0 | render/thread/ms 175, render/acquire/ms 150, render/busy/ms 24.02, main/busy/ms 21.05, render/graph/ms 14.54, render/present/ms 1.12, terrain/update/ms 0.83, bigspace/total/ms 0.83, terrain/select/ms 0.75, physics/insert/ms 0.65, controller/feed/ms 0.63, stream/drive/ms 0.44 |
| measure | 13.91 | 83.6 | render/thread/ms 173, render/acquire/ms 147, render/busy/ms 24.88, main/busy/ms 19.80, render/graph/ms 15.29, render/present/ms 1.07, bigspace/total/ms 0.75, physics/insert/ms 0.70, terrain/update/ms 0.64, terrain/select/ms 0.58, controller/feed/ms 0.52, stream/drive/ms 0.42 |
| measure | 14.00 | 91.4 | render/thread/ms 172, render/acquire/ms 142, render/busy/ms 29.60, main/busy/ms 20.41, render/graph/ms 18.40, render/present/ms 0.93, bigspace/total/ms 0.72, terrain/update/ms 0.68, controller/feed/ms 0.65, terrain/select/ms 0.62, stream/drive/ms 0.41, physics/insert/ms 0.39 |
| measure | 14.17 | 94.8 | render/thread/ms 165, render/acquire/ms 138, render/busy/ms 26.23, main/busy/ms 20.58, render/graph/ms 15.36, render/present/ms 1.16, bigspace/total/ms 0.85, controller/feed/ms 0.71, terrain/update/ms 0.68, terrain/select/ms 0.62, stream/drive/ms 0.49, physics/insert/ms 0.47 |
| measure | 14.25 | 85.6 | render/thread/ms 178, render/acquire/ms 151, render/busy/ms 25.36, main/busy/ms 19.79, render/graph/ms 15.04, render/present/ms 1.15, bigspace/total/ms 0.75, terrain/update/ms 0.73, controller/feed/ms 0.68, terrain/select/ms 0.66, physics/insert/ms 0.42, stream/drive/ms 0.41 |
| measure | 14.33 | 78.3 | render/thread/ms 162, render/acquire/ms 136, render/busy/ms 24.48, main/busy/ms 19.32, render/graph/ms 14.16, render/present/ms 1.21, bigspace/total/ms 0.71, terrain/update/ms 0.69, controller/feed/ms 0.65, terrain/select/ms 0.63, stream/drive/ms 0.44, physics/insert/ms 0.39 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\h1_old_nomaps\balmora_walk@2560x1440_off=normalmaps_orm_parallax-3 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 8%
- during the run: other processes 9% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **91%** (min 48%) · 2578 MHz (min 2565) · memory 8001 MHz · 73.3 W · 74 °C (max 81) · VRAM 7852 MiB · throttled 100% of samples
