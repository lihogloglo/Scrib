# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: farnpcs · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6651 frames over 65.6 s · **median 9.66 ms (103 fps)** · mean 9.87 · p95 12.23 · p99 14.00 (1% low 71 fps) · worst 35.26 ms · 38 spikes over 1.5× median · 22.9 stutters/min (work 16.5/min)
- warm-up: 485 frames over 5.6 s, worst 577.7 ms
- hitches (> 25.0 ms): **2** measured, 6 in warm-up
- timed GPU passes: 6.03 ms of a 9.87 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.66 ms, limit 11 ms
- **FAIL** frame_ms: 25 stutters (> 1.5x moving median of 60) in 1.09 min = 22.85/min, limit 1/min
- **FAIL** worst frame 35.26 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.995 | 2.198 | 2.567 | 0.91 | 0.174 |
| main_opaque_pass_3d | 1.280 | 1.410 | 2.009 | 0.91 | 0.047 |
| early prepass | 0.419 | 0.462 | 0.721 | 0.91 | 0.066 |
| bloom | 0.305 | 0.338 | 0.494 | 0.90 | 0.063 |
| atmosphere_luts | 0.295 | 0.325 | 0.520 | 0.91 | 0.004 |
| ssao | 0.194 | 0.214 | 0.211 | 0.91 | 0.007 |
| uniform_allocation | 0.150 | 0.165 | 0.175 | 0.90 | 0.032 |
| auxiliary_views | 0.148 | 0.163 | 0.671 | 0.91 | 0.142 |
| water_fft | 0.119 | 0.131 | 0.129 | 0.91 | 0.003 |
| upscaling | 0.117 | 0.128 | 0.159 | 0.91 | 0.004 |
| clouds_trace | 0.116 | 0.127 | 0.221 | 0.91 | 0.001 |
| bin_unpacking | 0.105 | 0.116 | 0.132 | 0.91 | 0.022 |
| atmos_froxel_inject | 0.088 | 0.097 | 0.108 | 0.91 | 0.002 |
| tonemapping | 0.083 | 0.092 | 0.094 | 0.91 | 0.004 |
| water_velocity | 0.077 | 0.085 | 0.085 | 0.91 | 0.002 |
| clustering | 0.064 | 0.071 | 0.070 | 0.91 | 0.019 |
| main_transparent_pass_3d | 0.059 | 0.065 | 0.113 | 0.91 | 0.104 |
| clouds_composite | 0.058 | 0.064 | 0.063 | 0.91 | 0.002 |
| render_sky | 0.049 | 0.054 | 0.054 | 0.91 | 0.002 |
| lightprobe_irradiance_map | 0.039 | 0.436 | 0.749 | 0.09 | 0.001 |
| early_mesh_preprocessing | 0.036 | 0.040 | 0.045 | 0.91 | 0.006 |
| surface_trail | 0.035 | 0.059 | 0.057 | 0.60 | 0.002 |
| clouds_resolve | 0.032 | 0.035 | 0.039 | 0.91 | 0.001 |
| clouds_shadow | 0.029 | 0.032 | 0.034 | 0.91 | 0.003 |
| atmos_froxel_integrate | 0.023 | 0.025 | 0.030 | 0.91 | 0.001 |
| water_snapshot | 0.022 | 0.025 | 0.024 | 0.90 | 0.001 |
| water_mips | 0.021 | 0.024 | 0.024 | 0.91 | 0.002 |
| lightprobe_radiance_map | 0.020 | 0.224 | 0.232 | 0.09 | 0.004 |
| per_view_shadow_pass | 0.020 | 0.022 | 0.024 | 0.91 | 0.007 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.90 | 0.002 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.90 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.91 | 0.004 |
| night_sky | 0.006 | 0.006 | 0.014 | 0.91 | 0.002 |
| lightprobe_downsampling_first_pass | 0.001 | 0.015 | 0.015 | 0.09 | 0.001 |
| lightprobe_copy | 0.001 | 0.014 | 0.015 | 0.09 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.012 | 0.012 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 11.610 | 0.000 | 10252.94 | 77216.3 |
| render/thread/ms | 8.607 | 10.837 | 33.74 | 57243.7 |
| main/busy/ms | 8.101 | 9.693 | 16.67 | 53878.1 |
| render/busy/ms | 7.995 | 10.276 | 21.55 | 53175.6 |
| render/graph/ms | 4.337 | 5.843 | 12.37 | 28847.3 |
| bigspace/total/ms | 0.569 | 1.224 | 2.45 | 3784.9 |
| render/present/ms | 0.519 | 1.205 | 23.51 | 3451.5 |
| terrain/update/ms | 0.294 | 0.382 | 2.27 | 1957.7 |
| controller/feed/ms | 0.281 | 0.370 | 0.82 | 1871.6 |
| terrain/select/ms | 0.261 | 0.339 | 0.80 | 1737.7 |
| stream/drive/ms | 0.236 | 0.319 | 2.13 | 1570.0 |
| anim/evaluate/ms | 0.216 | 0.540 | 1.89 | 1438.5 |
| bigspace/low_precision/ms | 0.216 | 0.693 | 1.85 | 1433.9 |
| stream/tick/ms | 0.193 | 0.261 | 0.60 | 1281.8 |
| far/select/ms | 0.191 | 0.263 | 0.64 | 1268.2 |
| bigspace/high_precision/ms | 0.181 | 0.727 | 2.06 | 1201.9 |
| anim/evaluate_cpu/ms | 0.149 | 0.190 | 0.69 | 990.5 |
| pipelines/compile_ms | 0.145 | 0.000 | 145.11 | 967.1 |
| controller/move/ms | 0.141 | 0.337 | 0.74 | 940.3 |
| pipelines/front_ms | 0.140 | 0.000 | 140.95 | 932.0 |
| render/acquire/ms | 0.093 | 0.124 | 1.49 | 616.5 |
| physics/step/ms | 0.042 | 0.084 | 0.90 | 280.7 |
| physics/insert/ms | 0.037 | 0.387 | 1.12 | 244.7 |
| weather/map/ms | 0.027 | 0.035 | 0.20 | 180.4 |
| anim/apply/ms | 0.011 | 0.019 | 0.26 | 74.9 |
| weather/eval/ms | 0.011 | 0.015 | 0.30 | 74.7 |
| controller/avoid/ms | 0.010 | 0.014 | 0.17 | 69.7 |
| script/update/ms | 0.007 | 0.010 | 0.29 | 48.7 |
| pipelines/driver_ms | 0.005 | 0.000 | 5.23 | 35.1 |
| script/input/ms | 0.005 | 0.007 | 0.14 | 34.1 |
| stream/despawn/ms | 0.004 | 0.000 | 1.45 | 29.2 |
| stream/spawn/ms | 0.004 | 0.000 | 1.13 | 25.5 |
| script/ui/ms | 0.003 | 0.005 | 0.15 | 22.9 |
| controller/write/ms | 0.003 | 0.004 | 0.29 | 22.9 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.75 | 14.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 4.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.02 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 11 |
| crowd/c0/joints/count | count | 1301 | 2262 | 489 |
| crowd/c0/meshes/count | count | 355 | 646 | 128 |
| crowd/c1/count | count | 146 | 156 | 150 |
| crowd/c1/joints/count | count | 5531 | 5935 | 5707 |
| crowd/c1/meshes/count | count | 1165 | 1259 | 1213 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 532 | 571 | 571 |
| crowd/joint_blocks/count | count | 15 | 22 | 6 |
| crowd/joints_extracted/count | count | 770 | 6208 | 308 |
| crowd/joints_uploaded/count | count | 770 | 1090 | 308 |
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
| frame/work/ms | ms | 8.20 | 21.55 | 7.47 |
| gpu/timed/ms | ms | 7.35 | 9.68 | 7.26 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 823 | 1420 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 823 | 1420 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 11 | 16 | 10 |
| phases/opaque3d/bins | count | 975 | 1210 | 1152 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 2802 | 4435 | 2175 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 11 | 15 | 9 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 11 | 16 | 10 |
| phases/opaque3d_prepass/bins | count | 975 | 1210 | 1152 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 2802 | 4435 | 2175 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 11 | 15 | 9 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 37 | 235 | 25 |
| phases/shadow/bins | count | 3510 | 5253 | 1861 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8910 | 12.3 k | 4552 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 114 | 255 | 25 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 905 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 172 | 174 | 174 |
| pipelines/waiting | count | 0 | 19 | 0 |
| preprocess/bind_groups | count | 9 | 609 | 6 |
| preprocess/bind_groups_created | count | 9 | 609 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 157 | 271 | 132 |
| process/cpu_usage | % | 26.44 | 40.38 | 26.44 |
| process/mem_usage | GiB | 2.68 | 2.83 | 2.83 |
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
| started_load_count |  | 3473 | 3529 | 3529 |
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
| stream/oldest_request/s | s | 0 | 0.23 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.57 | 47.97 | 35.91 |
| system/mem_usage | % | 53.98 | 54.94 | 54.91 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 76 | 273 | 59 |
| views/all/bins | count | 5542 | 7382 | 4959 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 16.6 k | 23.3 k | 10.1 k |
| views/all/pipelines | count | 148 | 292 | 55 |
| views/all/pipelines_distinct | count | 57 | 67 | 43 |
| views/all/work_items | count | 15.6 k | 21.7 k | 10.4 k |
| views/cascade/batch_sets | count | 35 | 50 | 25 |
| views/cascade/bins | count | 2285 | 2863 | 1861 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7448 | 9667 | 4552 |
| views/cascade/pipelines | count | 35 | 50 | 25 |
| views/cascade/work_items | count | 7448 | 9667 | 4552 |
| views/lamp_face/batch_sets | count | 0 | 196 | 0 |
| views/lamp_face/bins | count | 1064 | 2515 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1192 | 2881 | 0 |
| views/lamp_face/pipelines | count | 74 | 210 | 0 |
| views/lamp_face/work_items | count | 0 | 2608 | 0 |
| views/main/batch_sets | count | 38 | 46 | 34 |
| views/main/bins | count | 2274 | 3214 | 3098 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7254 | 11.6 k | 5584 |
| views/main/pipelines | count | 34 | 42 | 30 |
| views/main/work_items | count | 7409 | 11.8 k | 5892 |
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
| anim/bones_written/count | count | 43.69 | 216 | 290.6 k |
| anim/events/count | count | 0.22 | 3 | 1436 |
| assets/Image/added | count | 0.05 | 35 | 316 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.62 | 176 | 4118 |
| assets/Mesh/removed | count | 0.74 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 473 |
| assets/StandardMaterial/modified | count | 3.26 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.99 | 1391 | 6594 |
| physics/inserts | count | 0.47 | 153 | 3117 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 145 | 2335 |
| physics/steps | count | 0.59 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 6 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 21 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 23 | 378 |
| stream/upload/bytes | bytes | 5090 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1116 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 8, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 577.7 | main/busy/ms 556, surface/advance_cpu_ms 67.76, surface/cell_upload_ms 0.72, stream/drive/ms 0.34, weather/map/ms 0.23, bigspace/total/ms 0.07, stream/tick/ms 0.07, bigspace/high_precision/ms 0.05, script/update/ms 0.03, weather/eval/ms 0.03, anim/evaluate/ms 0.01, far/select/ms 0.01 |
| warm-up | 0.86 | 282.2 | main/busy/ms 588, render/thread/ms 248, render/busy/ms 176, render/acquire/ms 69.61, surface/advance_cpu_ms 67.76, render/graph/ms 5.21, render/present/ms 2.35, stream/drive/ms 1.01, terrain/update/ms 0.74, surface/cell_upload_ms 0.72, terrain/select/ms 0.52, stream/tick/ms 0.45 |
| warm-up | 0.98 | 115.2 | pipelines/wait_ms 596, render/thread/ms 259, render/busy/ms 186, pipelines/compile_ms 134, pipelines/front_ms 124, render/acquire/ms 69.82, main/busy/ms 49.24, terrain/update/ms 17.29, pipelines/driver_ms 10.12, render/graph/ms 9.07, render/present/ms 3.34, stream/drive/ms 1.36 |
| warm-up | 1.21 | 87.0 | main/busy/ms 65.62, render/thread/ms 46.88, render/busy/ms 45.20, render/graph/ms 4.62, render/present/ms 1.47, stream/drive/ms 1.41, stream/tick/ms 1.06, terrain/update/ms 0.88, terrain/select/ms 0.72, bigspace/total/ms 0.38, render/acquire/ms 0.22, bigspace/high_precision/ms 0.19 |
| warm-up | 1.38 | 28.3 | pipelines/wait_ms 602, pipelines/compile_ms 172, pipelines/front_ms 161, render/thread/ms 31.84, render/busy/ms 30.58, main/busy/ms 12.62, pipelines/driver_ms 10.82, render/graph/ms 3.65, stream/drive/ms 2.34, render/present/ms 1.07, stream/tick/ms 0.85, terrain/update/ms 0.76 |
| warm-up | 2.78 | 41.4 | pipelines/wait_ms 2841, pipelines/compile_ms 52.31, render/thread/ms 45.03, render/busy/ms 43.61, render/graph/ms 35.47, pipelines/driver_ms 26.64, pipelines/front_ms 25.67, main/busy/ms 15.35, stream/drive/ms 1.37, render/present/ms 1.23, terrain/update/ms 0.84, bigspace/total/ms 0.81 |
| measure | 31.95 | 35.3 | render/thread/ms 44.55, render/present/ms 24.21, render/busy/ms 20.16, main/busy/ms 18.91, render/graph/ms 10.95, bigspace/total/ms 0.99, controller/feed/ms 0.85, terrain/update/ms 0.55, stream/drive/ms 0.49, terrain/select/ms 0.49, far/select/ms 0.47, stream/tick/ms 0.43 |
| measure | 46.97 | 27.9 | render/thread/ms 37.86, render/busy/ms 21.88, main/busy/ms 20.38, render/present/ms 15.72, render/graph/ms 12.24, bigspace/total/ms 1.40, controller/feed/ms 1.01, bigspace/low_precision/ms 0.77, stream/drive/ms 0.75, terrain/update/ms 0.75, terrain/select/ms 0.66, stream/tick/ms 0.66 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\m_farnpcs\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=farnpcs
- before the run: GPU 5%, CPU 9%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **86%** (min 61%) · 2580 MHz (min 2580) · memory 8001 MHz · 83.8 W · 75 °C (max 80) · VRAM 4391 MiB · throttled 100% of samples
