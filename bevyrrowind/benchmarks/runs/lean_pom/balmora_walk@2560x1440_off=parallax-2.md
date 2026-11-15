# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4907 frames over 65.7 s · **median 13.04 ms (77 fps)** · mean 13.38 · p95 16.82 · p99 19.45 (1% low 51 fps) · worst 38.36 ms · 46 spikes over 1.5× median · 18.3 stutters/min (work 13.7/min)
- warm-up: 497 frames over 10.2 s, worst 2724.1 ms
- hitches (> 32.6 ms): **3** measured, 14 in warm-up
- timed GPU passes: 7.78 ms of a 13.38 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.04 ms, limit 11 ms
- **FAIL** frame_ms: 20 stutters (> 1.5x moving median of 60) in 1.09 min = 18.28/min, limit 1/min
- **FAIL** worst frame 38.36 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.165 | 2.185 | 2.523 | 0.99 | 0.181 |
| main_opaque_pass_3d | 1.586 | 1.600 | 2.196 | 0.99 | 0.067 |
| early prepass | 0.537 | 0.542 | 0.812 | 0.99 | 0.114 |
| bloom | 0.334 | 0.337 | 0.487 | 0.99 | 0.067 |
| atmosphere_luts | 0.327 | 0.330 | 0.618 | 0.99 | 0.004 |
| water_velocity | 0.294 | 0.297 | 0.514 | 0.99 | 0.002 |
| uniform_allocation | 0.292 | 0.294 | 0.326 | 0.99 | 0.052 |
| bin_unpacking | 0.222 | 0.224 | 0.255 | 0.99 | 0.036 |
| water_sspr | 0.221 | 0.223 | 0.227 | 0.99 | 0.002 |
| auxiliary_views | 0.212 | 0.214 | 0.761 | 0.99 | 0.187 |
| ssao | 0.202 | 0.204 | 0.209 | 0.99 | 0.008 |
| atmos_froxel_inject | 0.155 | 0.157 | 0.152 | 0.99 | 0.002 |
| clouds_trace | 0.140 | 0.141 | 0.230 | 0.99 | 0.001 |
| water_fft | 0.139 | 0.140 | 0.139 | 0.99 | 0.003 |
| upscaling | 0.125 | 0.126 | 0.134 | 0.99 | 0.004 |
| clouds_composite | 0.110 | 0.111 | 0.111 | 0.99 | 0.002 |
| tonemapping | 0.091 | 0.092 | 0.091 | 0.99 | 0.004 |
| main_transmissive_pass_3d | 0.089 | 0.090 | 0.170 | 0.99 | 0.008 |
| clustering | 0.066 | 0.067 | 0.069 | 0.99 | 0.020 |
| render_sky | 0.054 | 0.055 | 0.055 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.441 | 0.756 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.99 | 0.001 |
| surface_trail | 0.048 | 0.059 | 0.057 | 0.82 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.043 | 0.048 | 0.99 | 0.005 |
| water_snapshot | 0.035 | 0.035 | 0.045 | 0.99 | 0.002 |
| clouds_shadow | 0.031 | 0.032 | 0.035 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.241 | 0.553 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.031 | 0.99 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 0.99 | 0.007 |
| main_transparent_pass_3d | 0.017 | 0.017 | 0.032 | 0.99 | 0.109 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 12.176 | 15.480 | 37.20 | 59748.7 |
| render/busy/ms | 11.603 | 14.875 | 36.32 | 56938.3 |
| main/busy/ms | 9.320 | 12.215 | 29.27 | 45732.0 |
| render/graph/ms | 7.062 | 9.166 | 19.69 | 34654.5 |
| pipelines/wait_ms | 4.697 | 0.000 | 11621.55 | 23049.6 |
| render/present/ms | 0.480 | 0.570 | 12.81 | 2357.5 |
| bigspace/total/ms | 0.451 | 0.701 | 4.31 | 2214.8 |
| terrain/update/ms | 0.291 | 0.364 | 1.74 | 1429.8 |
| controller/feed/ms | 0.275 | 0.367 | 0.94 | 1348.6 |
| stream/drive/ms | 0.259 | 0.367 | 20.63 | 1272.5 |
| terrain/select/ms | 0.259 | 0.324 | 0.71 | 1268.6 |
| pipelines/compile_ms | 0.223 | 0.000 | 163.28 | 1096.5 |
| pipelines/front_ms | 0.215 | 0.000 | 157.06 | 1057.2 |
| stream/tick/ms | 0.198 | 0.274 | 0.79 | 972.5 |
| far/select/ms | 0.196 | 0.273 | 0.57 | 959.6 |
| bigspace/low_precision/ms | 0.170 | 0.287 | 3.98 | 833.9 |
| anim/evaluate_cpu/ms | 0.163 | 0.210 | 0.49 | 799.9 |
| anim/evaluate/ms | 0.133 | 0.262 | 2.68 | 650.5 |
| controller/move/ms | 0.113 | 0.193 | 0.56 | 554.1 |
| bigspace/high_precision/ms | 0.111 | 0.219 | 3.31 | 546.4 |
| render/acquire/ms | 0.092 | 0.123 | 0.33 | 452.9 |
| physics/step/ms | 0.058 | 0.092 | 0.55 | 285.1 |
| physics/insert/ms | 0.058 | 0.455 | 1.05 | 285.1 |
| weather/map/ms | 0.025 | 0.034 | 0.22 | 120.8 |
| anim/apply/ms | 0.012 | 0.021 | 0.17 | 57.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.13 | 52.2 |
| controller/avoid/ms | 0.010 | 0.014 | 0.24 | 51.3 |
| pipelines/driver_ms | 0.008 | 0.000 | 6.22 | 39.3 |
| script/update/ms | 0.007 | 0.010 | 0.19 | 34.9 |
| stream/despawn/ms | 0.006 | 0.000 | 1.16 | 29.3 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 25.1 |
| script/input/ms | 0.005 | 0.007 | 0.27 | 23.8 |
| script/ui/ms | 0.005 | 0.006 | 0.11 | 22.9 |
| controller/write/ms | 0.004 | 0.005 | 0.06 | 18.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.65 | 15.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 4.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.5 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 36 | 18 |
| controller/movers/count | count | 20 | 21 | 18 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1272 | 2262 | 595 |
| crowd/c0/meshes/count | count | 349 | 646 | 158 |
| crowd/c1/count | count | 147 | 155 | 148 |
| crowd/c1/joints/count | count | 5539 | 5933 | 5601 |
| crowd/c1/meshes/count | count | 1167 | 1261 | 1183 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4287 | 9951 | 6179 |
| crowd/joints_uploaded/count | count | 4259 | 6643 | 6179 |
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
| frame/work/ms | ms | 11.36 | 36.32 | 11.34 |
| gpu/timed/ms | ms | 8.51 | 12.54 | 9.19 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 198 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 794 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 198 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 794 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 32 | 39 | 29 |
| phases/opaque3d/bins | count | 1295 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3350 | 5126 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 23 | 26 | 23 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 32 | 39 | 29 |
| phases/opaque3d_prepass/bins | count | 1295 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3350 | 5126 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 23 | 26 | 23 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 73 | 312 | 54 |
| phases/shadow/bins | count | 3759 | 5494 | 2250 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9333 | 12.9 k | 5705 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 136 | 320 | 45 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 147 | 476 | 19 |
| physics/pending | count | 0 | 635 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 214 | 218 | 218 |
| pipelines/waiting | count | 0 | 6 | 0 |
| preprocess/bind_groups | count | 12 | 857 | 10 |
| preprocess/bind_groups_created | count | 12 | 857 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 294 | 443 | 265 |
| process/cpu_usage | % | 23.66 | 38.22 | 20.67 |
| process/mem_usage | GiB | 5.15 | 5.33 | 5.33 |
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
| started_load_count |  | 4767 | 4859 | 4859 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 4 | 3 |
| statics/material_pipelines | count | 3 | 4 | 3 |
| statics/materials | count | 432 | 528 | 396 |
| statics/materials_bound | count | 432 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.34 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.61 | 50.51 | 42.93 |
| system/mem_usage | % | 75.26 | 76.03 | 76.03 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 157 | 396 | 126 |
| views/all/bins | count | 6390 | 8365 | 5730 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 17.4 k | 25.5 k | 13.3 k |
| views/all/pipelines | count | 191 | 377 | 103 |
| views/all/pipelines_distinct | count | 87 | 100 | 81 |
| views/all/work_items | count | 16.9 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 63 | 81 | 54 |
| views/cascade/bins | count | 2607 | 3155 | 2250 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8125 | 10.2 k | 5705 |
| views/cascade/pipelines | count | 52 | 67 | 45 |
| views/cascade/work_items | count | 8125 | 10.2 k | 5705 |
| views/lamp_face/batch_sets | count | 0 | 243 | 0 |
| views/lamp_face/bins | count | 1080 | 2548 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1210 | 2910 | 0 |
| views/lamp_face/pipelines | count | 83 | 255 | 0 |
| views/lamp_face/work_items | count | 0 | 2614 | 0 |
| views/main/batch_sets | count | 80 | 94 | 72 |
| views/main/bins | count | 2888 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8296 | 13.0 k | 7600 |
| views/main/pipelines | count | 58 | 64 | 58 |
| views/main/work_items | count | 8455 | 13.3 k | 7908 |
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
| anim/bones_written/count | count | 62.86 | 281 | 308.4 k |
| anim/events/count | count | 0.11 | 3 | 532 |
| assets/Image/added | count | 0.12 | 43 | 603 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.84 | 152 | 4140 |
| assets/Mesh/removed | count | 0.99 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 480 |
| assets/StandardMaterial/modified | count | 4.42 | 16 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.23 | 1485 | 6047 |
| physics/inserts | count | 0.58 | 106 | 2848 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.48 | 118 | 2335 |
| physics/steps | count | 0.80 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 15 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 17 | 378 |
| stream/upload/bytes | bytes | 6899 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1513 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 32.6 ms; slowest 17, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.72 | 2724.1 | main/busy/ms 2699, surface/advance_cpu_ms 69.61, surface/cell_upload_ms 0.82, stream/drive/ms 0.34, weather/map/ms 0.12, bigspace/total/ms 0.12, stream/tick/ms 0.10, bigspace/high_precision/ms 0.07, weather/eval/ms 0.04, script/update/ms 0.03, anim/evaluate/ms 0.02, script/input/ms 0.01 |
| warm-up | 3.44 | 712.5 | main/busy/ms 2735, render/thread/ms 678, render/busy/ms 552, render/acquire/ms 124, surface/advance_cpu_ms 69.61, render/graph/ms 4.87, render/present/ms 2.09, stream/drive/ms 1.09, surface/cell_upload_ms 0.82, stream/tick/ms 0.57, terrain/update/ms 0.46, bigspace/total/ms 0.32 |
| warm-up | 3.65 | 214.7 | pipelines/wait_ms 1112, render/thread/ms 690, render/busy/ms 563, pipelines/compile_ms 222, pipelines/front_ms 206, render/acquire/ms 124, main/busy/ms 52.38, terrain/update/ms 19.44, pipelines/driver_ms 15.76, render/graph/ms 8.63, render/present/ms 3.28, stream/drive/ms 1.52 |
| warm-up | 3.90 | 99.3 | pipelines/compile_ms 249, pipelines/front_ms 238, main/busy/ms 67.54, render/thread/ms 58.97, render/busy/ms 57.15, pipelines/driver_ms 10.99, render/graph/ms 4.90, stream/drive/ms 2.56, stream/tick/ms 2.02, render/present/ms 1.49, terrain/update/ms 0.76, terrain/select/ms 0.63 |
| warm-up | 4.05 | 49.7 | pipelines/wait_ms 238, pipelines/compile_ms 159, pipelines/front_ms 151, render/thread/ms 37.22, render/busy/ms 35.85, main/busy/ms 35.69, stream/drive/ms 24.35, pipelines/driver_ms 8.22, render/graph/ms 2.42, stream/tick/ms 1.06, render/present/ms 1.02, terrain/update/ms 0.74 |
| warm-up | 4.20 | 81.4 | pipelines/wait_ms 2401, render/thread/ms 64.66, render/busy/ms 63.38, main/busy/ms 50.68, pipelines/compile_ms 35.04, pipelines/front_ms 31.64, stream/drive/ms 24.54, pipelines/driver_ms 3.41, render/graph/ms 2.85, stream/tick/ms 2.01, stream/spawn/ms 1.44, render/present/ms 1.05 |
| warm-up | 4.32 | 92.1 | pipelines/wait_ms 2011, pipelines/compile_ms 174, pipelines/front_ms 168, render/thread/ms 80.99, render/busy/ms 55.88, main/busy/ms 48.62, stream/drive/ms 27.05, render/present/ms 24.82, pipelines/driver_ms 5.64, render/graph/ms 3.73, stream/spawn/ms 1.98, stream/tick/ms 1.06 |
| warm-up | 4.50 | 37.2 | pipelines/wait_ms 1624, main/busy/ms 64.41, stream/drive/ms 47.79, render/thread/ms 38.50, render/busy/ms 37.03, render/graph/ms 4.59, pipelines/compile_ms 2.57, stream/tick/ms 1.69, pipelines/front_ms 1.44, render/present/ms 1.25, pipelines/driver_ms 1.13, terrain/update/ms 0.84 |
| warm-up | 4.56 | 65.5 | pipelines/wait_ms 5938, pipelines/compile_ms 170, pipelines/front_ms 159, main/busy/ms 64.42, stream/drive/ms 48.18, render/thread/ms 41.35, render/busy/ms 40.16, pipelines/driver_ms 11.53, render/graph/ms 2.76, stream/spawn/ms 1.40, stream/tick/ms 1.39, render/present/ms 0.97 |
| warm-up | 4.64 | 59.1 | pipelines/wait_ms 8477, pipelines/compile_ms 103, pipelines/front_ms 96.90, main/busy/ms 63.43, stream/drive/ms 46.41, render/thread/ms 29.37, render/busy/ms 28.00, pipelines/driver_ms 6.30, render/graph/ms 3.32, stream/spawn/ms 2.73, render/present/ms 1.10, stream/tick/ms 0.84 |
| warm-up | 4.71 | 46.8 | main/busy/ms 53.31, render/thread/ms 36.58, render/busy/ms 34.90, stream/drive/ms 33.95, render/graph/ms 6.61, stream/spawn/ms 2.05, render/present/ms 1.48, stream/tick/ms 0.83, terrain/update/ms 0.77, terrain/select/ms 0.64, stream/activate/ms 0.51, bigspace/total/ms 0.33 |
| warm-up | 5.16 | 36.3 | render/thread/ms 48.91, render/busy/ms 47.29, main/busy/ms 33.59, render/graph/ms 6.66, stream/drive/ms 1.76, render/present/ms 1.42, stream/tick/ms 0.79, terrain/update/ms 0.70, terrain/select/ms 0.58, bigspace/total/ms 0.52, physics/step/ms 0.51, physics/insert/ms 0.46 |
| warm-up | 5.23 | 44.2 | pipelines/wait_ms 2675, pipelines/compile_ms 154, pipelines/front_ms 144, render/thread/ms 44.02, render/busy/ms 42.38, main/busy/ms 32.68, pipelines/driver_ms 10.38, stream/drive/ms 6.37, render/graph/ms 5.76, render/present/ms 1.39, terrain/update/ms 0.98, terrain/select/ms 0.82 |
| warm-up | 6.58 | 45.0 | main/busy/ms 53.00, render/thread/ms 31.24, render/busy/ms 29.72, render/graph/ms 7.09, stream/drive/ms 1.45, render/present/ms 1.21, bigspace/total/ms 0.83, stream/tick/ms 0.68, terrain/update/ms 0.56, anim/evaluate_cpu/ms 0.51, terrain/select/ms 0.49, far/select/ms 0.40 |
| measure | 2.73 | 38.4 | render/thread/ms 51.23, render/busy/ms 49.82, render/graph/ms 26.85, main/busy/ms 20.86, render/present/ms 1.22, physics/insert/ms 1.17, bigspace/total/ms 0.73, controller/feed/ms 0.56, terrain/update/ms 0.55, terrain/select/ms 0.50, stream/drive/ms 0.47, stream/tick/ms 0.41 |
| measure | 19.29 | 34.5 | render/thread/ms 48.75, render/busy/ms 47.28, render/graph/ms 29.47, main/busy/ms 21.07, render/present/ms 1.26, physics/insert/ms 0.96, bigspace/total/ms 0.80, controller/feed/ms 0.59, stream/drive/ms 0.57, terrain/update/ms 0.56, terrain/select/ms 0.51, stream/tick/ms 0.45 |
| measure | 59.04 | 33.0 | render/thread/ms 42.44, render/busy/ms 41.22, main/busy/ms 23.97, render/graph/ms 11.80, bigspace/total/ms 1.84, stream/drive/ms 1.23, bigspace/low_precision/ms 1.18, stream/tick/ms 1.12, render/present/ms 0.97, controller/feed/ms 0.91, terrain/update/ms 0.88, far/select/ms 0.87 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_pom\balmora_walk@2560x1440_off=parallax-2 --bench-baseline none --off parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **76%** (min 53%) · 2580 MHz (min 2580) · memory 8001 MHz · 73.5 W · 74 °C (max 79) · VRAM 6284 MiB · throttled 100% of samples
