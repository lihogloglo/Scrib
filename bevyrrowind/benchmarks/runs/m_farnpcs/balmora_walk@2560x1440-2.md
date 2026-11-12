# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: farnpcs · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6708 frames over 65.6 s · **median 9.57 ms (105 fps)** · mean 9.79 · p95 12.10 · p99 13.65 (1% low 73 fps) · worst 25.89 ms · 38 spikes over 1.5× median · 18.3 stutters/min (work 12.8/min)
- warm-up: 358 frames over 4.7 s, worst 588.1 ms
- hitches (> 25.0 ms): **2** measured, 6 in warm-up
- timed GPU passes: 6.24 ms of a 9.79 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.57 ms, limit 11 ms
- **FAIL** frame_ms: 20 stutters (> 1.5x moving median of 60) in 1.09 min = 18.28/min, limit 1/min
- pass worst frame 25.89 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.051 | 2.249 | 2.859 | 0.91 | 0.178 |
| main_opaque_pass_3d | 1.328 | 1.456 | 2.142 | 0.91 | 0.039 |
| early prepass | 0.441 | 0.484 | 0.956 | 0.91 | 0.067 |
| bloom | 0.313 | 0.343 | 0.502 | 0.91 | 0.063 |
| atmosphere_luts | 0.306 | 0.335 | 0.598 | 0.91 | 0.003 |
| ssao | 0.198 | 0.219 | 0.213 | 0.91 | 0.007 |
| auxiliary_views | 0.177 | 0.194 | 0.685 | 0.91 | 0.163 |
| uniform_allocation | 0.152 | 0.166 | 0.176 | 0.91 | 0.029 |
| water_fft | 0.122 | 0.134 | 0.129 | 0.91 | 0.003 |
| upscaling | 0.119 | 0.130 | 0.160 | 0.91 | 0.004 |
| clouds_trace | 0.117 | 0.129 | 0.216 | 0.91 | 0.001 |
| bin_unpacking | 0.112 | 0.122 | 0.138 | 0.91 | 0.021 |
| atmos_froxel_inject | 0.090 | 0.099 | 0.108 | 0.91 | 0.001 |
| tonemapping | 0.084 | 0.092 | 0.093 | 0.91 | 0.003 |
| water_velocity | 0.082 | 0.090 | 0.087 | 0.91 | 0.002 |
| clustering | 0.062 | 0.069 | 0.070 | 0.90 | 0.019 |
| clouds_composite | 0.059 | 0.065 | 0.065 | 0.91 | 0.002 |
| main_transparent_pass_3d | 0.057 | 0.063 | 0.111 | 0.91 | 0.106 |
| render_sky | 0.049 | 0.054 | 0.054 | 0.91 | 0.002 |
| lightprobe_irradiance_map | 0.042 | 0.449 | 0.758 | 0.09 | 0.001 |
| early_mesh_preprocessing | 0.037 | 0.040 | 0.045 | 0.91 | 0.005 |
| surface_trail | 0.036 | 0.061 | 0.058 | 0.59 | 0.002 |
| clouds_resolve | 0.032 | 0.035 | 0.038 | 0.91 | 0.001 |
| clouds_shadow | 0.029 | 0.032 | 0.035 | 0.91 | 0.003 |
| atmos_froxel_integrate | 0.023 | 0.026 | 0.030 | 0.91 | 0.001 |
| water_snapshot | 0.022 | 0.024 | 0.024 | 0.91 | 0.001 |
| lightprobe_radiance_map | 0.022 | 0.236 | 0.238 | 0.09 | 0.004 |
| water_mips | 0.022 | 0.024 | 0.024 | 0.91 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.023 | 0.025 | 0.91 | 0.006 |
| clouds_fog_light | 0.012 | 0.013 | 0.013 | 0.91 | 0.002 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.91 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.91 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.91 | 0.003 |
| lightprobe_downsampling_first_pass | 0.001 | 0.016 | 0.014 | 0.09 | 0.001 |
| lightprobe_copy | 0.001 | 0.015 | 0.015 | 0.09 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 11.712 | 0.000 | 7620.75 | 78564.7 |
| render/thread/ms | 8.559 | 10.730 | 23.83 | 57414.4 |
| main/busy/ms | 8.009 | 9.587 | 14.75 | 53726.8 |
| render/busy/ms | 7.925 | 10.172 | 23.13 | 53161.4 |
| render/graph/ms | 4.268 | 5.755 | 13.58 | 28629.4 |
| bigspace/total/ms | 0.544 | 1.157 | 2.27 | 3647.2 |
| render/present/ms | 0.515 | 1.192 | 12.30 | 3454.6 |
| terrain/update/ms | 0.294 | 0.376 | 1.41 | 1970.2 |
| controller/feed/ms | 0.267 | 0.357 | 0.70 | 1790.6 |
| terrain/select/ms | 0.262 | 0.334 | 0.90 | 1755.0 |
| stream/drive/ms | 0.234 | 0.325 | 2.12 | 1569.3 |
| pipelines/compile_ms | 0.223 | 0.000 | 131.92 | 1497.0 |
| pipelines/front_ms | 0.216 | 0.000 | 127.67 | 1449.0 |
| bigspace/low_precision/ms | 0.214 | 0.683 | 1.57 | 1438.0 |
| stream/tick/ms | 0.191 | 0.258 | 0.67 | 1283.7 |
| far/select/ms | 0.190 | 0.253 | 0.62 | 1271.9 |
| anim/evaluate_cpu/ms | 0.171 | 0.218 | 0.52 | 1147.7 |
| bigspace/high_precision/ms | 0.161 | 0.645 | 1.79 | 1080.0 |
| controller/move/ms | 0.150 | 0.341 | 0.75 | 1007.5 |
| anim/evaluate/ms | 0.147 | 0.387 | 1.33 | 984.9 |
| render/acquire/ms | 0.119 | 0.258 | 2.22 | 798.4 |
| physics/step/ms | 0.044 | 0.085 | 0.89 | 292.4 |
| physics/insert/ms | 0.032 | 0.369 | 0.80 | 217.3 |
| weather/map/ms | 0.027 | 0.036 | 0.38 | 183.0 |
| anim/apply/ms | 0.011 | 0.021 | 0.20 | 76.2 |
| weather/eval/ms | 0.011 | 0.015 | 0.25 | 74.9 |
| controller/avoid/ms | 0.011 | 0.014 | 0.35 | 70.7 |
| pipelines/driver_ms | 0.007 | 0.000 | 5.09 | 48.1 |
| script/update/ms | 0.007 | 0.010 | 0.15 | 47.6 |
| script/input/ms | 0.005 | 0.006 | 0.21 | 32.7 |
| stream/despawn/ms | 0.004 | 0.000 | 1.01 | 28.4 |
| script/ui/ms | 0.004 | 0.005 | 0.06 | 26.5 |
| stream/spawn/ms | 0.004 | 0.000 | 1.09 | 25.0 |
| controller/write/ms | 0.004 | 0.004 | 0.05 | 23.9 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 13.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 7.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 4.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.02 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 1 | 10 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 11 |
| crowd/c0/joints/count | count | 1301 | 2262 | 480 |
| crowd/c0/meshes/count | count | 353 | 646 | 117 |
| crowd/c1/count | count | 147 | 154 | 150 |
| crowd/c1/joints/count | count | 5531 | 5882 | 5716 |
| crowd/c1/meshes/count | count | 1167 | 1254 | 1224 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 531 | 571 | 571 |
| crowd/joint_blocks/count | count | 15 | 22 | 6 |
| crowd/joints_extracted/count | count | 728 | 5452 | 299 |
| crowd/joints_uploaded/count | count | 728 | 1090 | 299 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.2 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 8.17 | 23.13 | 9.00 |
| gpu/timed/ms | ms | 7.53 | 10.10 | 9.00 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 814 | 1420 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 814 | 1420 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 11 | 16 | 10 |
| phases/opaque3d/bins | count | 976 | 1210 | 1152 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 2802 | 4480 | 2176 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 10 | 15 | 9 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 11 | 16 | 10 |
| phases/opaque3d_prepass/bins | count | 976 | 1210 | 1152 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 2802 | 4480 | 2176 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 10 | 15 | 9 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 38 | 238 | 23 |
| phases/shadow/bins | count | 3556 | 5323 | 1817 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8902 | 12.4 k | 4508 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 106 | 255 | 23 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 1022 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 167 | 169 | 169 |
| pipelines/waiting | count | 0 | 26 | 0 |
| preprocess/bind_groups | count | 9 | 623 | 6 |
| preprocess/bind_groups_created | count | 9 | 623 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 157 | 268 | 128 |
| process/cpu_usage | % | 26.01 | 41.26 | 34.29 |
| process/mem_usage | GiB | 2.73 | 2.89 | 2.89 |
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
| started_load_count |  | 3470 | 3525 | 3525 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 435 | 530 | 398 |
| statics/materials_bound | count | 435 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.24 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.74 | 45.96 | 43.07 |
| system/mem_usage | % | 54.67 | 55.10 | 54.80 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 76 | 280 | 57 |
| views/all/bins | count | 5534 | 7424 | 4915 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 16.5 k | 23.5 k | 10.1 k |
| views/all/pipelines | count | 145 | 290 | 53 |
| views/all/pipelines_distinct | count | 59 | 68 | 42 |
| views/all/work_items | count | 15.7 k | 22.0 k | 10.4 k |
| views/cascade/batch_sets | count | 35 | 50 | 23 |
| views/cascade/bins | count | 2289 | 2892 | 1817 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7448 | 9697 | 4508 |
| views/cascade/pipelines | count | 35 | 50 | 23 |
| views/cascade/work_items | count | 7448 | 9697 | 4508 |
| views/lamp_face/batch_sets | count | 0 | 199 | 0 |
| views/lamp_face/bins | count | 1082 | 2585 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1226 | 2921 | 0 |
| views/lamp_face/pipelines | count | 70 | 207 | 0 |
| views/lamp_face/work_items | count | 0 | 2659 | 0 |
| views/main/batch_sets | count | 36 | 48 | 34 |
| views/main/bins | count | 2302 | 3214 | 3098 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7236 | 11.7 k | 5586 |
| views/main/pipelines | count | 32 | 44 | 30 |
| views/main/work_items | count | 7396 | 11.9 k | 5894 |
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
| anim/bones_written/count | count | 52.54 | 274 | 352.5 k |
| anim/events/count | count | 0.24 | 4 | 1598 |
| assets/Image/added | count | 0.05 | 35 | 310 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.61 | 160 | 4098 |
| assets/Mesh/removed | count | 0.73 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 468 |
| assets/StandardMaterial/modified | count | 3.23 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.86 | 1484 | 5784 |
| physics/inserts | count | 0.48 | 150 | 3220 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 145 | 2335 |
| physics/steps | count | 0.59 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 3 | 9 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 21 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 23 | 378 |
| stream/upload/bytes | bytes | 5047 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1106 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 8, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 588.1 | main/busy/ms 566, surface/advance_cpu_ms 70.10, surface/cell_upload_ms 0.70, stream/drive/ms 0.38, weather/map/ms 0.14, stream/tick/ms 0.11, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.03, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 1.06 | 471.8 | main/busy/ms 599, render/thread/ms 432, render/busy/ms 354, render/acquire/ms 76.68, surface/advance_cpu_ms 70.10, render/graph/ms 4.67, render/present/ms 1.94, stream/drive/ms 1.12, surface/cell_upload_ms 0.70, stream/tick/ms 0.57, terrain/update/ms 0.41, terrain/select/ms 0.28 |
| warm-up | 1.29 | 89.0 | main/busy/ms 62.93, render/thread/ms 56.09, render/busy/ms 53.20, render/graph/ms 5.36, render/present/ms 1.89, stream/drive/ms 1.47, stream/tick/ms 1.17, render/acquire/ms 1.00, terrain/update/ms 0.97, terrain/select/ms 0.78, bigspace/total/ms 0.21, bigspace/high_precision/ms 0.10 |
| warm-up | 1.49 | 30.3 | pipelines/wait_ms 597, pipelines/compile_ms 186, pipelines/front_ms 175, render/thread/ms 39.61, render/busy/ms 38.42, main/busy/ms 18.17, pipelines/driver_ms 10.75, stream/drive/ms 4.99, render/graph/ms 3.04, stream/spawn/ms 2.58, render/present/ms 1.00, stream/tick/ms 0.89 |
| warm-up | 2.43 | 36.7 | render/thread/ms 44.32, render/busy/ms 43.18, main/busy/ms 32.76, render/graph/ms 5.15, stream/drive/ms 2.71, render/present/ms 0.94, stream/spawn/ms 0.85, stream/tick/ms 0.76, terrain/update/ms 0.68, terrain/select/ms 0.58, bigspace/total/ms 0.50, bigspace/high_precision/ms 0.21 |
| warm-up | 3.26 | 28.0 | main/busy/ms 34.01, render/thread/ms 15.76, render/busy/ms 14.48, render/graph/ms 4.56, physics/insert/ms 1.92, stream/drive/ms 1.17, render/present/ms 1.06, terrain/update/ms 0.66, bigspace/total/ms 0.64, stream/tick/ms 0.58, terrain/select/ms 0.55, stream/spawn/ms 0.30 |
| measure | 2.72 | 25.9 | render/thread/ms 28.89, render/busy/ms 27.56, main/busy/ms 20.31, render/graph/ms 15.66, bigspace/total/ms 1.85, bigspace/high_precision/ms 1.25, render/present/ms 1.07, physics/insert/ms 0.71, terrain/update/ms 0.64, terrain/select/ms 0.57, controller/feed/ms 0.48, stream/drive/ms 0.40 |
| measure | 19.26 | 25.6 | render/thread/ms 33.70, render/busy/ms 32.33, main/busy/ms 19.15, render/graph/ms 18.50, render/present/ms 1.19, physics/insert/ms 1.03, bigspace/total/ms 0.76, terrain/update/ms 0.66, terrain/select/ms 0.59, controller/feed/ms 0.55, stream/drive/ms 0.43, stream/tick/ms 0.38 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\m_farnpcs\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=farnpcs
- before the run: GPU 0%, CPU 15%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **89%** (min 70%) · 2580 MHz (min 2580) · memory 8001 MHz · 85.1 W · 75 °C (max 79) · VRAM 4139 MiB · throttled 100% of samples
