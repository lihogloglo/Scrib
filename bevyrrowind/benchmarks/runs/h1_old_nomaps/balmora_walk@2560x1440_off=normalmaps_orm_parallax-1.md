# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5737 frames over 65.7 s · **median 11.22 ms (89 fps)** · mean 11.44 · p95 13.94 · p99 16.30 (1% low 61 fps) · worst 36.76 ms · 46 spikes over 1.5× median · 29.2 stutters/min (work 27.4/min)
- warm-up: 338 frames over 8.6 s, worst 2666.7 ms
- hitches (> 28.1 ms): **7** measured, 25 in warm-up
- timed GPU passes: 7.14 ms of a 11.44 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.22 ms, limit 11 ms
- **FAIL** frame_ms: 32 stutters (> 1.5x moving median of 60) in 1.09 min = 29.25/min, limit 1/min
- **FAIL** worst frame 36.76 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.062 | 2.186 | 2.546 | 0.94 | 0.180 |
| main_opaque_pass_3d | 1.468 | 1.556 | 2.196 | 0.94 | 0.055 |
| early prepass | 0.464 | 0.492 | 0.754 | 0.94 | 0.093 |
| bloom | 0.318 | 0.337 | 0.488 | 0.94 | 0.066 |
| atmosphere_luts | 0.311 | 0.329 | 0.510 | 0.94 | 0.003 |
| water_velocity | 0.288 | 0.306 | 0.519 | 0.94 | 0.001 |
| uniform_allocation | 0.210 | 0.222 | 0.230 | 0.94 | 0.037 |
| water_sspr | 0.209 | 0.222 | 0.224 | 0.94 | 0.002 |
| ssao | 0.191 | 0.203 | 0.210 | 0.94 | 0.008 |
| bin_unpacking | 0.168 | 0.178 | 0.185 | 0.94 | 0.026 |
| auxiliary_views | 0.150 | 0.159 | 0.615 | 0.94 | 0.145 |
| atmos_froxel_inject | 0.147 | 0.155 | 0.153 | 0.94 | 0.001 |
| clouds_trace | 0.133 | 0.141 | 0.237 | 0.94 | 0.001 |
| water_fft | 0.132 | 0.140 | 0.139 | 0.94 | 0.003 |
| upscaling | 0.119 | 0.126 | 0.134 | 0.94 | 0.004 |
| clouds_composite | 0.105 | 0.111 | 0.111 | 0.94 | 0.002 |
| main_transmissive_pass_3d | 0.086 | 0.091 | 0.172 | 0.94 | 0.008 |
| tonemapping | 0.085 | 0.090 | 0.091 | 0.94 | 0.003 |
| clustering | 0.064 | 0.068 | 0.070 | 0.94 | 0.020 |
| render_sky | 0.051 | 0.055 | 0.054 | 0.94 | 0.002 |
| clouds_resolve | 0.047 | 0.050 | 0.051 | 0.94 | 0.001 |
| lightprobe_irradiance_map | 0.045 | 0.438 | 0.757 | 0.10 | 0.001 |
| early_mesh_preprocessing | 0.041 | 0.044 | 0.049 | 0.94 | 0.005 |
| surface_trail | 0.041 | 0.058 | 0.057 | 0.70 | 0.002 |
| water_snapshot | 0.034 | 0.036 | 0.044 | 0.94 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.94 | 0.003 |
| atmos_froxel_integrate | 0.026 | 0.027 | 0.030 | 0.94 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.239 | 0.240 | 0.10 | 0.004 |
| water_mips | 0.023 | 0.025 | 0.024 | 0.94 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.021 | 0.023 | 0.94 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.032 | 0.94 | 0.110 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.94 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.94 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.94 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.94 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 32.491 | 0.000 | 18003.39 | 186398.7 |
| render/thread/ms | 10.234 | 12.572 | 33.48 | 58713.9 |
| render/busy/ms | 9.599 | 11.928 | 32.55 | 55072.2 |
| main/busy/ms | 8.644 | 10.767 | 29.86 | 49591.6 |
| render/graph/ms | 5.444 | 6.985 | 16.02 | 31231.6 |
| pipelines/compile_ms | 2.495 | 0.000 | 2828.21 | 14314.9 |
| pipelines/driver_ms | 2.001 | 0.000 | 2695.62 | 11479.2 |
| render/present/ms | 0.541 | 1.360 | 13.54 | 3102.7 |
| pipelines/front_ms | 0.494 | 0.000 | 181.14 | 2835.7 |
| bigspace/total/ms | 0.488 | 0.990 | 2.90 | 2800.0 |
| terrain/update/ms | 0.303 | 0.387 | 2.04 | 1735.7 |
| controller/feed/ms | 0.293 | 0.387 | 0.70 | 1679.5 |
| terrain/select/ms | 0.269 | 0.343 | 0.77 | 1542.9 |
| stream/drive/ms | 0.256 | 0.357 | 21.60 | 1470.0 |
| bigspace/low_precision/ms | 0.201 | 0.502 | 2.60 | 1154.1 |
| stream/tick/ms | 0.199 | 0.278 | 0.83 | 1143.6 |
| far/select/ms | 0.193 | 0.267 | 0.66 | 1107.7 |
| anim/evaluate_cpu/ms | 0.173 | 0.217 | 0.56 | 994.3 |
| controller/move/ms | 0.138 | 0.256 | 0.53 | 789.4 |
| anim/evaluate/ms | 0.132 | 0.251 | 2.01 | 755.3 |
| bigspace/high_precision/ms | 0.126 | 0.293 | 2.37 | 722.9 |
| render/acquire/ms | 0.094 | 0.125 | 0.40 | 539.0 |
| physics/step/ms | 0.050 | 0.087 | 0.86 | 286.1 |
| physics/insert/ms | 0.040 | 0.399 | 0.99 | 230.0 |
| weather/map/ms | 0.024 | 0.033 | 0.19 | 138.2 |
| anim/apply/ms | 0.013 | 0.023 | 0.20 | 72.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.32 | 61.7 |
| controller/avoid/ms | 0.010 | 0.014 | 0.16 | 58.8 |
| script/update/ms | 0.007 | 0.009 | 0.06 | 39.2 |
| script/ui/ms | 0.005 | 0.007 | 0.11 | 31.3 |
| stream/despawn/ms | 0.005 | 0.000 | 1.02 | 29.1 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 29.1 |
| stream/spawn/ms | 0.004 | 0.000 | 1.07 | 25.3 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 19.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.61 | 14.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 17 | 0 |
| anim/a2/count | count | 1 | 13 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 52 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 47 | 12 |
| crowd/c0/joints/count | count | 1285 | 2262 | 547 |
| crowd/c0/meshes/count | count | 353 | 646 | 140 |
| crowd/c1/count | count | 146 | 156 | 149 |
| crowd/c1/joints/count | count | 5500 | 5983 | 5649 |
| crowd/c1/meshes/count | count | 1167 | 1297 | 1201 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 222 |
| crowd/joints_extracted/count | count | 4253 | 11.7 k | 6135 |
| crowd/joints_uploaded/count | count | 4230 | 6643 | 6135 |
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
| frame/work/ms | ms | 9.44 | 32.55 | 8.33 |
| gpu/timed/ms | ms | 8.30 | 12.60 | 8.33 |
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
| phases/alpha_mask3d/items | count | 814 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 814 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1290 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3417 | 5163 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1290 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3417 | 5163 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 53 | 297 | 39 |
| phases/shadow/bins | count | 3805 | 5447 | 2199 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9499 | 12.9 k | 5654 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 144 | 308 | 39 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2650 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 1544 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 199 | 203 | 203 |
| pipelines/waiting | count | 0 | 42 | 0 |
| preprocess/bind_groups | count | 10 | 763 | 10 |
| preprocess/bind_groups_created | count | 10 | 763 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 223 | 340 | 211 |
| process/cpu_usage | % | 25.96 | 48.68 | 26.32 |
| process/mem_usage | GiB | 5.22 | 5.47 | 5.47 |
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
| started_load_count |  | 4765 | 4860 | 4860 |
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
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.74 | 65.49 | 35.45 |
| system/mem_usage | % | 65.52 | 66.37 | 66.26 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 111 | 349 | 99 |
| views/all/bins | count | 6512 | 8269 | 5679 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.3 k | 25.4 k | 13.3 k |
| views/all/pipelines | count | 196 | 362 | 95 |
| views/all/pipelines_distinct | count | 80 | 92 | 77 |
| views/all/work_items | count | 17.4 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 48 | 63 | 39 |
| views/cascade/bins | count | 2616 | 3142 | 2199 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8162 | 10.2 k | 5654 |
| views/cascade/pipelines | count | 48 | 63 | 39 |
| views/cascade/work_items | count | 8162 | 10.2 k | 5654 |
| views/lamp_face/batch_sets | count | 0 | 245 | 0 |
| views/lamp_face/bins | count | 1091 | 2452 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1237 | 2823 | 0 |
| views/lamp_face/pipelines | count | 91 | 250 | 0 |
| views/lamp_face/work_items | count | 0 | 2537 | 0 |
| views/main/batch_sets | count | 60 | 62 | 60 |
| views/main/bins | count | 2872 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8448 | 13.1 k | 7600 |
| views/main/pipelines | count | 56 | 58 | 56 |
| views/main/work_items | count | 8613 | 13.3 k | 7908 |
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
| anim/bones_written/count | count | 61.82 | 714 | 354.7 k |
| anim/events/count | count | 0.10 | 4 | 564 |
| assets/Image/added | count | 0.12 | 56 | 666 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.75 | 152 | 4281 |
| assets/Mesh/removed | count | 0.85 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 57 | 545 |
| assets/StandardMaterial/modified | count | 3.80 | 20 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.10 | 2375 | 12.0 k |
| physics/inserts | count | 0.65 | 150 | 3756 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 161 | 2335 |
| physics/steps | count | 0.69 | 2 | 3940 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 2 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 5901 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1294 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.1 ms; slowest 32, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.67 | 2666.7 | main/busy/ms 2642, surface/advance_cpu_ms 69.17, surface/cell_upload_ms 0.73, stream/drive/ms 0.31, weather/map/ms 0.11, stream/tick/ms 0.09, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, far/select/ms 0.02, script/update/ms 0.02, script/input/ms 0.01 |
| warm-up | 3.42 | 748.9 | main/busy/ms 2674, render/thread/ms 717, render/busy/ms 534, render/acquire/ms 181, surface/advance_cpu_ms 69.17, render/graph/ms 6.45, render/present/ms 2.58, stream/drive/ms 1.16, surface/cell_upload_ms 0.73, stream/tick/ms 0.61, terrain/update/ms 0.55, terrain/select/ms 0.38 |
| warm-up | 3.74 | 320.3 | pipelines/wait_ms 2855, render/thread/ms 730, render/busy/ms 546, pipelines/compile_ms 331, pipelines/front_ms 307, render/acquire/ms 181, main/busy/ms 48.71, pipelines/driver_ms 24.53, terrain/update/ms 17.30, render/graph/ms 11.00, render/present/ms 3.49, stream/drive/ms 1.40 |
| warm-up | 3.98 | 81.5 | main/busy/ms 67.87, render/thread/ms 54.98, render/busy/ms 52.99, render/graph/ms 4.61, render/present/ms 1.59, stream/drive/ms 1.38, stream/tick/ms 1.04, terrain/update/ms 0.75, terrain/select/ms 0.60, bigspace/total/ms 0.49, render/acquire/ms 0.39, bigspace/low_precision/ms 0.26 |
| warm-up | 4.12 | 55.4 | main/busy/ms 45.55, render/thread/ms 37.77, render/busy/ms 36.64, stream/drive/ms 31.29, render/graph/ms 3.26, render/present/ms 0.95, terrain/update/ms 0.92, stream/tick/ms 0.82, terrain/select/ms 0.74, stream/spawn/ms 0.53, bigspace/total/ms 0.26, render/acquire/ms 0.19 |
| warm-up | 4.20 | 33.3 | pipelines/wait_ms 1119, pipelines/compile_ms 186, pipelines/front_ms 173, main/busy/ms 44.69, render/thread/ms 37.90, render/busy/ms 35.83, stream/drive/ms 24.01, pipelines/driver_ms 12.25, render/graph/ms 5.36, render/present/ms 1.77, stream/tick/ms 1.15, stream/spawn/ms 1.13 |
| warm-up | 4.26 | 52.6 | pipelines/wait_ms 4381, pipelines/compile_ms 264, pipelines/front_ms 246, render/thread/ms 63.86, render/busy/ms 61.35, main/busy/ms 41.45, stream/drive/ms 17.94, pipelines/driver_ms 17.43, render/graph/ms 9.12, terrain/update/ms 3.43, stream/spawn/ms 2.35, render/present/ms 2.18 |
| warm-up | 4.29 | 38.4 | pipelines/wait_ms 4756, pipelines/compile_ms 96.38, pipelines/front_ms 88.79, render/thread/ms 64.63, render/busy/ms 61.66, main/busy/ms 27.91, render/graph/ms 10.45, pipelines/driver_ms 7.59, stream/drive/ms 5.10, terrain/update/ms 3.52, render/present/ms 2.67, stream/spawn/ms 2.52 |
| warm-up | 4.34 | 48.6 | pipelines/wait_ms 2548, pipelines/compile_ms 58.94, render/thread/ms 55.83, pipelines/front_ms 55.36, render/busy/ms 52.78, main/busy/ms 49.44, stream/drive/ms 30.21, render/graph/ms 8.57, pipelines/driver_ms 3.58, render/present/ms 2.80, stream/spawn/ms 2.32, stream/tick/ms 1.43 |
| warm-up | 4.41 | 63.0 | pipelines/wait_ms 1054, render/thread/ms 71.10, render/busy/ms 68.90, main/busy/ms 49.15, pipelines/compile_ms 40.73, pipelines/front_ms 39.55, stream/drive/ms 29.40, render/graph/ms 8.62, render/present/ms 2.00, stream/spawn/ms 1.53, pipelines/driver_ms 1.18, terrain/update/ms 0.92 |
| warm-up | 4.44 | 32.7 | pipelines/wait_ms 666, render/thread/ms 65.62, render/busy/ms 63.43, main/busy/ms 23.33, render/graph/ms 11.87, pipelines/compile_ms 7.03, pipelines/front_ms 6.52, stream/drive/ms 3.52, render/present/ms 1.96, terrain/update/ms 1.04, stream/activate/ms 0.98, terrain/select/ms 0.86 |
| warm-up | 4.48 | 41.8 | pipelines/wait_ms 6249, pipelines/compile_ms 183, pipelines/front_ms 167, render/thread/ms 61.50, render/busy/ms 58.58, main/busy/ms 32.06, render/graph/ms 20.70, pipelines/driver_ms 16.15, stream/drive/ms 15.81, render/present/ms 2.69, stream/spawn/ms 2.04, stream/activate/ms 1.09 |
| warm-up | 4.51 | 28.3 | pipelines/wait_ms 6068, pipelines/compile_ms 206, pipelines/front_ms 189, main/busy/ms 69.34, render/thread/ms 54.56, stream/drive/ms 52.02, render/busy/ms 51.89, pipelines/driver_ms 17.12, render/graph/ms 14.75, stream/spawn/ms 2.39, render/present/ms 1.94, stream/tick/ms 1.26 |
| warm-up | 4.58 | 74.9 | pipelines/wait_ms 485, main/busy/ms 59.52, render/thread/ms 45.96, render/busy/ms 44.36, stream/drive/ms 39.53, pipelines/compile_ms 30.64, pipelines/front_ms 29.16, render/graph/ms 3.06, pipelines/driver_ms 1.48, stream/spawn/ms 1.37, stream/tick/ms 1.31, terrain/update/ms 1.00 |
| warm-up | 4.66 | 62.4 | main/busy/ms 64.44, stream/drive/ms 47.59, render/thread/ms 34.85, render/busy/ms 33.86, render/graph/ms 3.61, stream/spawn/ms 2.75, stream/tick/ms 0.84, render/present/ms 0.83, terrain/update/ms 0.74, terrain/select/ms 0.62, stream/activate/ms 0.56, physics/insert/ms 0.28 |
| warm-up | 4.73 | 50.0 | pipelines/wait_ms 2562, pipelines/compile_ms 204, pipelines/front_ms 189, main/busy/ms 60.39, stream/drive/ms 36.94, render/thread/ms 35.12, render/busy/ms 33.65, pipelines/driver_ms 14.21, render/graph/ms 4.54, stream/spawn/ms 1.97, render/present/ms 1.08, stream/activate/ms 1.07 |
| warm-up | 4.78 | 28.7 | main/busy/ms 35.17, stream/drive/ms 22.86, render/thread/ms 21.01, render/busy/ms 19.79, terrain/update/ms 5.68, render/graph/ms 4.57, stream/activate/ms 1.25, render/present/ms 1.01, stream/tick/ms 0.70, terrain/select/ms 0.50, physics/insert/ms 0.37, bigspace/total/ms 0.29 |
| warm-up | 5.45 | 50.5 | pipelines/wait_ms 6623, pipelines/compile_ms 99.41, pipelines/front_ms 90.88, main/busy/ms 58.31, render/thread/ms 27.03, render/busy/ms 25.28, stream/drive/ms 15.01, pipelines/driver_ms 8.53, render/graph/ms 7.02, physics/insert/ms 1.66, render/present/ms 1.58, stream/spawn/ms 1.11 |
| warm-up | 5.48 | 35.3 | pipelines/wait_ms 10.0 k, pipelines/compile_ms 125, pipelines/front_ms 114, main/busy/ms 43.84, render/thread/ms 43.15, render/busy/ms 41.31, pipelines/driver_ms 10.32, render/graph/ms 7.24, stream/drive/ms 3.32, physics/insert/ms 1.64, render/present/ms 1.64, stream/spawn/ms 1.59 |
| warm-up | 5.59 | 29.9 | render/thread/ms 32.99, render/busy/ms 32.00, main/busy/ms 20.81, render/graph/ms 4.78, stream/drive/ms 3.99, stream/spawn/ms 2.17, stream/tick/ms 1.08, physics/insert/ms 0.92, terrain/update/ms 0.83, render/present/ms 0.80, terrain/select/ms 0.72, bigspace/total/ms 0.43 |
| warm-up | 5.98 | 30.7 | render/thread/ms 34.42, render/busy/ms 33.09, main/busy/ms 31.53, stream/drive/ms 13.85, render/graph/ms 4.88, stream/spawn/ms 2.06, physics/insert/ms 1.41, render/present/ms 1.14, terrain/update/ms 0.72, stream/tick/ms 0.66, bigspace/total/ms 0.63, terrain/select/ms 0.60 |
| warm-up | 6.09 | 28.5 | pipelines/wait_ms 10.3 k, main/busy/ms 45.15, pipelines/compile_ms 32.19, render/thread/ms 31.68, render/busy/ms 30.39, pipelines/front_ms 30.13, render/graph/ms 7.49, stream/drive/ms 2.80, pipelines/driver_ms 2.06, stream/spawn/ms 1.51, physics/insert/ms 1.40, render/present/ms 1.01 |
| warm-up | 6.12 | 32.5 | pipelines/wait_ms 2284, main/busy/ms 47.51, render/thread/ms 38.27, render/busy/ms 36.82, pipelines/compile_ms 10.98, pipelines/front_ms 10.68, render/graph/ms 7.79, stream/drive/ms 2.88, stream/spawn/ms 1.47, physics/insert/ms 1.39, render/present/ms 1.11, stream/tick/ms 0.80 |
| warm-up | 6.15 | 29.9 | pipelines/wait_ms 3048, pipelines/compile_ms 73.45, pipelines/front_ms 70.64, main/busy/ms 44.21, render/thread/ms 38.54, render/busy/ms 37.26, stream/drive/ms 8.47, render/graph/ms 5.79, pipelines/driver_ms 2.81, stream/spawn/ms 1.56, physics/insert/ms 1.19, render/present/ms 0.99 |
| warm-up | 6.34 | 30.4 | pipelines/wait_ms 1989, render/thread/ms 41.50, render/busy/ms 40.17, main/busy/ms 39.67, pipelines/compile_ms 18.58, pipelines/front_ms 17.48, render/graph/ms 4.98, stream/drive/ms 2.64, physics/insert/ms 1.59, stream/spawn/ms 1.51, pipelines/driver_ms 1.10, bigspace/total/ms 1.06 |
| measure | 0.80 | 29.3 | main/busy/ms 37.68, render/thread/ms 29.65, render/busy/ms 28.24, stream/drive/ms 14.78, render/graph/ms 12.32, render/present/ms 1.09, bigspace/total/ms 0.74, terrain/update/ms 0.69, physics/insert/ms 0.65, terrain/select/ms 0.61, far/select/ms 0.61, stream/tick/ms 0.55 |
| measure | 1.14 | 36.8 | main/busy/ms 45.92, render/thread/ms 43.25, render/busy/ms 41.83, render/graph/ms 11.72, render/present/ms 1.12, physics/insert/ms 1.03, bigspace/total/ms 0.89, physics/step/ms 0.84, stream/drive/ms 0.71, terrain/update/ms 0.67, stream/tick/ms 0.63, terrain/select/ms 0.59 |
| measure | 2.73 | 34.8 | render/thread/ms 45.04, render/busy/ms 43.50, main/busy/ms 25.77, render/graph/ms 21.12, render/present/ms 1.33, bigspace/total/ms 0.94, physics/insert/ms 0.91, terrain/update/ms 0.60, terrain/select/ms 0.54, controller/feed/ms 0.47, stream/drive/ms 0.46, far/select/ms 0.41 |
| measure | 18.18 | 32.0 | main/busy/ms 39.41, render/thread/ms 24.08, render/busy/ms 23.02, stream/drive/ms 21.83, render/graph/ms 13.89, render/present/ms 0.89, bigspace/total/ms 0.73, terrain/update/ms 0.65, terrain/select/ms 0.58, controller/feed/ms 0.56, far/select/ms 0.42, stream/tick/ms 0.37 |
| measure | 19.27 | 28.2 | render/thread/ms 37.53, render/busy/ms 36.32, render/graph/ms 21.32, main/busy/ms 18.61, render/present/ms 1.04, bigspace/total/ms 0.73, controller/feed/ms 0.58, terrain/update/ms 0.58, terrain/select/ms 0.52, physics/insert/ms 0.51, far/select/ms 0.47, stream/drive/ms 0.43 |
| measure | 59.02 | 31.3 | render/thread/ms 40.01, render/busy/ms 38.16, main/busy/ms 29.18, render/graph/ms 19.19, bigspace/total/ms 4.04, stream/drive/ms 1.99, bigspace/high_precision/ms 1.93, render/present/ms 1.59, controller/feed/ms 1.16, stream/tick/ms 0.99, stream/spawn/ms 0.87, anim/evaluate_cpu/ms 0.74 |
| measure | 59.06 | 33.4 | render/thread/ms 41.64, render/busy/ms 39.96, main/busy/ms 24.80, render/graph/ms 13.93, bigspace/total/ms 2.40, render/present/ms 1.43, stream/drive/ms 1.11, stream/tick/ms 1.00, controller/feed/ms 0.96, bigspace/high_precision/ms 0.90, anim/evaluate_cpu/ms 0.75, terrain/update/ms 0.69 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\h1_old_nomaps\balmora_walk@2560x1440_off=normalmaps_orm_parallax-1 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 15%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **85%** (min 53%) · 2580 MHz (min 2580) · memory 8001 MHz · 80.6 W · 75 °C (max 82) · VRAM 5637 MiB · throttled 100% of samples
