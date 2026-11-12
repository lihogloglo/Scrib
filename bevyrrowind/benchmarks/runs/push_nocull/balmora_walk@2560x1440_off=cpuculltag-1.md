# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cpuculltag · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6284 frames over 65.6 s · **median 10.32 ms (97 fps)** · mean 10.45 · p95 12.54 · p99 13.94 (1% low 72 fps) · worst 23.68 ms · 18 spikes over 1.5× median · 11.0 stutters/min (work 13.7/min)
- warm-up: 475 frames over 5.9 s, worst 581.2 ms
- hitches (> 25.8 ms): **0** measured, 7 in warm-up
- timed GPU passes: 7.69 ms of a 10.45 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.32 ms, limit 11 ms
- **FAIL** frame_ms: 12 stutters (> 1.5x moving median of 60) in 1.09 min = 10.97/min, limit 1/min
- pass worst frame 23.68 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.080 | 2.119 | 0.173 |
| main_opaque_pass_3d | 1.368 | 1.731 | 0.055 |
| atmosphere_luts | 0.833 | 0.968 | 0.003 |
| early prepass | 0.518 | 0.977 | 0.077 |
| lightprobe_irradiance_map | 0.444 | 0.669 | 0.001 |
| bloom | 0.320 | 0.327 | 0.061 |
| lightprobe_radiance_map | 0.224 | 0.234 | 0.004 |
| ssao | 0.205 | 0.207 | 0.007 |
| uniform_allocation | 0.187 | 0.353 | 0.035 |
| auxiliary_views | 0.179 | 0.607 | 0.166 |
| bin_unpacking | 0.129 | 0.139 | 0.025 |
| clouds_trace | 0.127 | 0.216 | 0.001 |
| water_fft | 0.125 | 0.126 | 0.003 |
| upscaling | 0.120 | 0.130 | 0.004 |
| atmos_froxel_inject | 0.092 | 0.097 | 0.002 |
| tonemapping | 0.086 | 0.090 | 0.004 |
| water_velocity | 0.081 | 0.085 | 0.002 |
| clustering | 0.068 | 0.069 | 0.019 |
| clouds_composite | 0.063 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.058 | 0.100 | 0.105 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.053 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.048 | 0.006 |
| clouds_resolve | 0.034 | 0.035 | 0.001 |
| clouds_shadow | 0.031 | 0.034 | 0.003 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.023 | 0.025 | 0.001 |
| water_mips | 0.022 | 0.023 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.024 | 0.007 |
| lightprobe_copy | 0.015 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.011 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.008 | 0.008 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.002 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.275 | 11.284 | 22.52 | 58281.1 |
| render/busy/ms | 8.679 | 10.748 | 21.94 | 54537.2 |
| main/busy/ms | 8.201 | 9.924 | 15.94 | 51533.9 |
| render/graph/ms | 4.763 | 6.108 | 12.95 | 29930.2 |
| pipelines/wait_ms | 2.217 | 0.000 | 4484.94 | 13933.0 |
| bigspace/total/ms | 0.525 | 1.213 | 5.20 | 3300.3 |
| render/present/ms | 0.504 | 1.200 | 2.23 | 3164.0 |
| terrain/update/ms | 0.294 | 0.384 | 2.03 | 1846.5 |
| controller/feed/ms | 0.273 | 0.339 | 0.87 | 1712.8 |
| terrain/select/ms | 0.261 | 0.342 | 0.89 | 1639.1 |
| stream/drive/ms | 0.239 | 0.327 | 6.88 | 1501.9 |
| bigspace/low_precision/ms | 0.209 | 0.590 | 1.90 | 1314.5 |
| far/select/ms | 0.199 | 0.276 | 0.63 | 1253.0 |
| stream/tick/ms | 0.194 | 0.262 | 0.67 | 1216.5 |
| anim/evaluate_cpu/ms | 0.169 | 0.211 | 0.50 | 1063.9 |
| bigspace/high_precision/ms | 0.146 | 0.535 | 1.82 | 914.3 |
| controller/move/ms | 0.136 | 0.264 | 0.75 | 854.6 |
| anim/evaluate/ms | 0.131 | 0.237 | 1.46 | 822.0 |
| pipelines/compile_ms | 0.130 | 0.000 | 164.85 | 819.4 |
| pipelines/front_ms | 0.126 | 0.000 | 159.48 | 792.9 |
| render/acquire/ms | 0.092 | 0.122 | 1.00 | 579.9 |
| physics/step/ms | 0.046 | 0.085 | 0.59 | 288.3 |
| physics/insert/ms | 0.040 | 0.404 | 0.98 | 253.5 |
| weather/map/ms | 0.027 | 0.035 | 0.21 | 169.2 |
| anim/apply/ms | 0.013 | 0.022 | 0.30 | 80.0 |
| weather/eval/ms | 0.011 | 0.015 | 0.18 | 68.4 |
| controller/avoid/ms | 0.011 | 0.014 | 0.15 | 67.7 |
| script/update/ms | 0.008 | 0.010 | 0.05 | 47.7 |
| script/input/ms | 0.005 | 0.007 | 0.13 | 33.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.07 | 28.5 |
| pipelines/driver_ms | 0.004 | 0.000 | 5.36 | 26.5 |
| stream/spawn/ms | 0.004 | 0.000 | 1.10 | 25.9 |
| script/ui/ms | 0.004 | 0.005 | 0.11 | 25.0 |
| controller/write/ms | 0.003 | 0.004 | 0.25 | 21.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 14.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.3 |
| shapes/activate/ms | 0.001 | 0.002 | 0.02 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.9 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 5 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 28 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
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
| frame/work/ms | ms | 8.67 | 21.94 | 8.16 |
| gpu/timed/ms | ms | 7.76 | 9.30 | 8.16 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 812 | 1420 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1420 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1283 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3402 | 5129 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1283 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3402 | 5129 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 41 | 230 | 28 |
| phases/shadow/bins | count | 3784 | 5460 | 2238 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9568 | 12.9 k | 5693 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 112 | 247 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2347 |
| physics/items | count | 200 | 476 | 19 |
| physics/pending | count | 0 | 370 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 172 | 174 | 174 |
| pipelines/waiting | count | 0 | 6 | 0 |
| preprocess/bind_groups | count | 10 | 603 | 9 |
| preprocess/bind_groups_created | count | 10 | 603 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 174 | 262 | 155 |
| process/cpu_usage | % | 25.84 | 38.60 | 14.90 |
| process/mem_usage | GiB | 2.65 | 2.83 | 2.83 |
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
| started_load_count |  | 3484 | 3544 | 3544 |
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
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.24 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.70 | 50.80 | 30.06 |
| system/mem_usage | % | 52.12 | 52.70 | 52.70 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 86 | 270 | 72 |
| views/all/bins | count | 6500 | 8320 | 5718 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.2 k | 25.4 k | 13.3 k |
| views/all/pipelines | count | 154 | 285 | 68 |
| views/all/pipelines_distinct | count | 62 | 69 | 55 |
| views/all/work_items | count | 17.2 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 28 |
| views/cascade/bins | count | 2606 | 3161 | 2238 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8176 | 10.2 k | 5693 |
| views/cascade/pipelines | count | 36 | 50 | 28 |
| views/cascade/work_items | count | 8176 | 10.2 k | 5693 |
| views/lamp_face/batch_sets | count | 3 | 191 | 0 |
| views/lamp_face/bins | count | 1088 | 2451 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1207 | 2824 | 0 |
| views/lamp_face/pipelines | count | 74 | 200 | 0 |
| views/lamp_face/work_items | count | 48 | 2593 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2874 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8423 | 13.0 k | 7600 |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 8582 | 13.2 k | 7908 |
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
| anim/bones_written/count | count | 57.45 | 260 | 361.0 k |
| anim/events/count | count | 0.24 | 4 | 1492 |
| assets/Image/added | count | 0.05 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.65 | 160 | 4086 |
| assets/Mesh/removed | count | 0.78 | 1242 | 4892 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.45 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1494 | 4830 |
| physics/inserts | count | 0.43 | 106 | 2673 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.37 | 118 | 2335 |
| physics/steps | count | 0.63 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 3 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 18 | 378 |
| stream/upload/bytes | bytes | 5387 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1181 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.8 ms; slowest 7, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 581.2 | main/busy/ms 559, surface/advance_cpu_ms 66.01, surface/cell_upload_ms 0.70, stream/drive/ms 0.29, weather/map/ms 0.11, bigspace/total/ms 0.10, stream/tick/ms 0.08, bigspace/high_precision/ms 0.07, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.90 | 317.7 | main/busy/ms 595, render/thread/ms 282, render/busy/ms 211, render/acquire/ms 68.33, surface/advance_cpu_ms 66.01, render/graph/ms 4.68, render/present/ms 2.53, stream/drive/ms 0.92, surface/cell_upload_ms 0.70, terrain/update/ms 0.46, stream/tick/ms 0.45, bigspace/total/ms 0.34 |
| warm-up | 1.14 | 237.2 | pipelines/wait_ms 4376, render/thread/ms 294, pipelines/compile_ms 253, pipelines/front_ms 229, render/busy/ms 222, render/acquire/ms 68.59, main/busy/ms 50.81, pipelines/driver_ms 23.70, terrain/update/ms 20.20, render/graph/ms 8.72, render/present/ms 3.58, stream/drive/ms 1.36 |
| warm-up | 1.38 | 104.9 | pipelines/wait_ms 372, pipelines/compile_ms 263, pipelines/front_ms 242, main/busy/ms 66.87, render/thread/ms 56.86, render/busy/ms 55.42, pipelines/driver_ms 21.55, render/graph/ms 3.96, stream/drive/ms 2.08, stream/tick/ms 1.73, render/present/ms 1.17, terrain/update/ms 0.78 |
| warm-up | 1.55 | 26.8 | render/thread/ms 41.14, render/busy/ms 39.88, main/busy/ms 18.22, render/graph/ms 3.87, stream/drive/ms 3.58, stream/spawn/ms 1.51, render/present/ms 1.08, terrain/update/ms 0.82, stream/tick/ms 0.75, terrain/select/ms 0.69, stream/activate/ms 0.25, bigspace/total/ms 0.20 |
| warm-up | 1.99 | 28.0 | render/thread/ms 36.95, main/busy/ms 35.95, render/busy/ms 35.74, stream/drive/ms 8.69, render/graph/ms 5.00, physics/insert/ms 1.17, render/present/ms 0.99, stream/tick/ms 0.68, bigspace/total/ms 0.64, terrain/update/ms 0.58, terrain/select/ms 0.49, physics/step/ms 0.47 |
| warm-up | 3.92 | 34.4 | main/busy/ms 36.77, render/thread/ms 26.98, render/busy/ms 25.71, render/graph/ms 6.20, stream/drive/ms 1.27, render/present/ms 0.95, physics/insert/ms 0.91, anim/evaluate_cpu/ms 0.84, bigspace/total/ms 0.80, terrain/update/ms 0.67, stream/tick/ms 0.62, terrain/select/ms 0.50 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\push_nocull\balmora_walk@2560x1440_off=cpuculltag-1 --bench-baseline none --off cpuculltag
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 8%, CPU 0%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **85%** (min 68%) · 2580 MHz (min 2580) · memory 8001 MHz · 80.8 W · 73 °C (max 78) · VRAM 4151 MiB · throttled 100% of samples
