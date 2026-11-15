# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5597 frames over 65.6 s · **median 11.42 ms (88 fps)** · mean 11.73 · p95 14.40 · p99 16.68 (1% low 60 fps) · worst 34.09 ms · 42 spikes over 1.5× median · 28.3 stutters/min (work 24.7/min)
- warm-up: 507 frames over 10.0 s, worst 2658.5 ms
- hitches (> 28.5 ms): **4** measured, 24 in warm-up
- timed GPU passes: 7.28 ms of a 11.73 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.42 ms, limit 11 ms
- **FAIL** frame_ms: 31 stutters (> 1.5x moving median of 60) in 1.09 min = 28.33/min, limit 1/min
- **FAIL** worst frame 34.09 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.083 | 2.199 | 2.618 | 0.95 | 0.184 |
| main_opaque_pass_3d | 1.489 | 1.571 | 2.288 | 0.95 | 0.065 |
| early prepass | 0.470 | 0.497 | 0.755 | 0.95 | 0.095 |
| bloom | 0.320 | 0.337 | 0.460 | 0.95 | 0.067 |
| atmosphere_luts | 0.313 | 0.330 | 0.479 | 0.95 | 0.004 |
| water_velocity | 0.287 | 0.303 | 0.447 | 0.95 | 0.002 |
| uniform_allocation | 0.213 | 0.224 | 0.229 | 0.95 | 0.042 |
| water_sspr | 0.210 | 0.222 | 0.221 | 0.95 | 0.002 |
| auxiliary_views | 0.207 | 0.218 | 0.735 | 0.95 | 0.205 |
| ssao | 0.194 | 0.205 | 0.209 | 0.95 | 0.008 |
| bin_unpacking | 0.173 | 0.182 | 0.184 | 0.95 | 0.029 |
| atmos_froxel_inject | 0.150 | 0.158 | 0.153 | 0.95 | 0.002 |
| clouds_trace | 0.134 | 0.142 | 0.235 | 0.95 | 0.001 |
| water_fft | 0.133 | 0.140 | 0.139 | 0.95 | 0.003 |
| upscaling | 0.121 | 0.128 | 0.160 | 0.95 | 0.004 |
| clouds_composite | 0.105 | 0.111 | 0.111 | 0.95 | 0.002 |
| tonemapping | 0.087 | 0.092 | 0.091 | 0.95 | 0.004 |
| main_transmissive_pass_3d | 0.086 | 0.091 | 0.170 | 0.95 | 0.009 |
| clustering | 0.065 | 0.068 | 0.070 | 0.95 | 0.021 |
| render_sky | 0.051 | 0.054 | 0.054 | 0.95 | 0.002 |
| clouds_resolve | 0.047 | 0.050 | 0.051 | 0.95 | 0.001 |
| lightprobe_irradiance_map | 0.045 | 0.437 | 0.432 | 0.10 | 0.001 |
| surface_trail | 0.043 | 0.060 | 0.058 | 0.72 | 0.002 |
| early_mesh_preprocessing | 0.041 | 0.043 | 0.048 | 0.95 | 0.006 |
| water_snapshot | 0.035 | 0.037 | 0.044 | 0.95 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.95 | 0.003 |
| atmos_froxel_integrate | 0.026 | 0.028 | 0.030 | 0.95 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.241 | 0.393 | 0.10 | 0.005 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.95 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.022 | 0.023 | 0.95 | 0.007 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.031 | 0.95 | 0.112 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.95 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.95 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.95 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.95 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.503 | 13.043 | 29.48 | 58785.8 |
| render/busy/ms | 9.884 | 12.461 | 25.47 | 55320.5 |
| main/busy/ms | 8.904 | 10.841 | 28.97 | 49838.4 |
| pipelines/wait_ms | 6.752 | 0.000 | 10574.22 | 37790.6 |
| render/graph/ms | 5.470 | 7.094 | 18.36 | 30613.9 |
| render/present/ms | 0.523 | 1.310 | 14.29 | 2927.6 |
| bigspace/total/ms | 0.492 | 0.912 | 2.98 | 2751.4 |
| terrain/update/ms | 0.302 | 0.393 | 2.14 | 1689.5 |
| controller/feed/ms | 0.300 | 0.403 | 0.87 | 1677.0 |
| terrain/select/ms | 0.268 | 0.348 | 0.78 | 1499.0 |
| stream/drive/ms | 0.257 | 0.364 | 20.66 | 1436.9 |
| pipelines/compile_ms | 0.227 | 0.000 | 158.08 | 1271.9 |
| pipelines/front_ms | 0.220 | 0.000 | 151.98 | 1231.8 |
| far/select/ms | 0.203 | 0.291 | 0.64 | 1135.5 |
| stream/tick/ms | 0.199 | 0.279 | 0.81 | 1116.1 |
| bigspace/low_precision/ms | 0.198 | 0.365 | 2.23 | 1106.7 |
| anim/evaluate_cpu/ms | 0.169 | 0.218 | 0.51 | 948.4 |
| anim/evaluate/ms | 0.142 | 0.275 | 1.59 | 795.2 |
| controller/move/ms | 0.135 | 0.238 | 0.57 | 753.0 |
| bigspace/high_precision/ms | 0.119 | 0.243 | 2.62 | 668.1 |
| render/acquire/ms | 0.096 | 0.125 | 1.82 | 537.7 |
| physics/step/ms | 0.055 | 0.094 | 0.47 | 307.9 |
| physics/insert/ms | 0.044 | 0.410 | 1.14 | 247.5 |
| weather/map/ms | 0.025 | 0.035 | 0.11 | 141.3 |
| anim/apply/ms | 0.013 | 0.025 | 0.27 | 73.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.30 | 59.3 |
| weather/eval/ms | 0.010 | 0.015 | 0.11 | 58.2 |
| script/update/ms | 0.007 | 0.009 | 0.33 | 40.8 |
| pipelines/driver_ms | 0.007 | 0.000 | 6.10 | 40.1 |
| stream/despawn/ms | 0.005 | 0.000 | 1.13 | 29.6 |
| script/input/ms | 0.005 | 0.007 | 0.19 | 28.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.09 | 26.4 |
| script/ui/ms | 0.004 | 0.006 | 0.05 | 24.3 |
| controller/write/ms | 0.004 | 0.005 | 0.03 | 20.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.71 | 15.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.14 | 4.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 65 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 42 | 13 |
| crowd/c0/joints/count | count | 1272 | 2218 | 583 |
| crowd/c0/meshes/count | count | 353 | 633 | 162 |
| crowd/c1/count | count | 146 | 155 | 148 |
| crowd/c1/joints/count | count | 5507 | 5936 | 5613 |
| crowd/c1/meshes/count | count | 1163 | 1268 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 9951 | 6179 |
| crowd/joints_uploaded/count | count | 4230 | 6643 | 6179 |
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
| frame/work/ms | ms | 9.66 | 28.97 | 9.22 |
| gpu/timed/ms | ms | 8.36 | 10.14 | 9.22 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 602.13 M | 603.31 M | 603.31 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 809 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 809 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1298 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3400 | 5143 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1298 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3400 | 5143 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 54 | 296 | 39 |
| phases/shadow/bins | count | 3816 | 5536 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9612 | 13.0 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 135 | 310 | 39 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 369 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 204 | 208 | 208 |
| pipelines/waiting | count | 0 | 8 | 0 |
| preprocess/bind_groups | count | 11 | 761 | 6 |
| preprocess/bind_groups_created | count | 11 | 761 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 225 | 343 | 215 |
| process/cpu_usage | % | 25.90 | 41.35 | 33.17 |
| process/mem_usage | GiB | 5.15 | 5.46 | 5.45 |
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
| started_load_count |  | 4767 | 4861 | 4861 |
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
| system/cpu_usage | % | 32.42 | 53.73 | 36.46 |
| system/mem_usage | % | 75.53 | 76.52 | 76.46 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 112 | 348 | 99 |
| views/all/bins | count | 6495 | 8404 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.2 k | 25.5 k | 13.3 k |
| views/all/pipelines | count | 191 | 364 | 95 |
| views/all/pipelines_distinct | count | 82 | 93 | 77 |
| views/all/work_items | count | 17.3 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 48 | 63 | 39 |
| views/cascade/bins | count | 2612 | 3172 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8149 | 10.3 k | 5663 |
| views/cascade/pipelines | count | 48 | 63 | 39 |
| views/cascade/work_items | count | 8149 | 10.3 k | 5663 |
| views/lamp_face/batch_sets | count | 6 | 243 | 0 |
| views/lamp_face/bins | count | 1116 | 2547 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1259 | 2883 | 0 |
| views/lamp_face/pipelines | count | 85 | 256 | 0 |
| views/lamp_face/work_items | count | 79 | 2635 | 0 |
| views/main/batch_sets | count | 60 | 62 | 60 |
| views/main/bins | count | 2890 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8409 | 13.0 k | 7594 |
| views/main/pipelines | count | 56 | 58 | 56 |
| views/main/work_items | count | 8575 | 13.3 k | 7902 |
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
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 68.76 | 346 | 384.9 k |
| anim/events/count | count | 0.10 | 4 | 558 |
| assets/Image/added | count | 0.11 | 43 | 636 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.74 | 152 | 4169 |
| assets/Mesh/removed | count | 0.87 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 36 | 514 |
| assets/StandardMaterial/modified | count | 3.89 | 18 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.12 | 1495 | 6284 |
| physics/inserts | count | 0.46 | 92 | 2562 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 145 | 2335 |
| physics/steps | count | 0.70 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 19 | 378 |
| stream/upload/bytes | bytes | 6048 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1326 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.5 ms; slowest 28, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.66 | 2658.5 | main/busy/ms 2634, surface/advance_cpu_ms 70.39, surface/cell_upload_ms 0.80, stream/drive/ms 0.36, weather/map/ms 0.14, bigspace/total/ms 0.09, stream/tick/ms 0.09, bigspace/high_precision/ms 0.06, script/update/ms 0.03, weather/eval/ms 0.03, anim/evaluate/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.33 | 666.7 | main/busy/ms 2667, render/thread/ms 634, render/busy/ms 521, render/acquire/ms 111, surface/advance_cpu_ms 70.39, render/graph/ms 6.46, render/present/ms 2.07, stream/drive/ms 1.04, surface/cell_upload_ms 0.80, terrain/update/ms 0.65, stream/tick/ms 0.51, terrain/select/ms 0.46 |
| warm-up | 3.52 | 197.7 | pipelines/wait_ms 1099, render/thread/ms 646, render/busy/ms 532, pipelines/compile_ms 207, pipelines/front_ms 193, render/acquire/ms 111, main/busy/ms 50.08, terrain/update/ms 17.27, pipelines/driver_ms 14.40, render/graph/ms 9.95, render/present/ms 3.12, stream/drive/ms 1.39 |
| warm-up | 3.76 | 96.1 | main/busy/ms 69.75, render/thread/ms 50.72, render/busy/ms 48.81, render/graph/ms 5.02, render/present/ms 1.58, stream/drive/ms 1.47, stream/tick/ms 1.24, terrain/update/ms 0.91, terrain/select/ms 0.77, bigspace/total/ms 0.47, render/acquire/ms 0.34, bigspace/high_precision/ms 0.15 |
| warm-up | 3.94 | 87.0 | render/thread/ms 59.10, render/busy/ms 57.27, main/busy/ms 51.24, stream/drive/ms 36.30, render/graph/ms 7.55, render/present/ms 1.56, stream/tick/ms 0.99, terrain/update/ms 0.82, terrain/select/ms 0.69, stream/spawn/ms 0.37, bigspace/total/ms 0.32, render/acquire/ms 0.27 |
| warm-up | 4.05 | 98.0 | pipelines/wait_ms 3430, pipelines/compile_ms 220, pipelines/front_ms 203, render/thread/ms 80.22, render/busy/ms 56.12, main/busy/ms 34.60, render/present/ms 23.80, stream/drive/ms 17.68, pipelines/driver_ms 16.58, render/graph/ms 3.02, stream/tick/ms 1.42, stream/spawn/ms 1.17 |
| warm-up | 4.13 | 61.8 | pipelines/wait_ms 1483, pipelines/compile_ms 62.91, render/thread/ms 56.41, render/busy/ms 53.65, pipelines/front_ms 47.33, main/busy/ms 41.23, pipelines/driver_ms 15.58, stream/drive/ms 12.67, render/graph/ms 8.92, render/present/ms 2.24, stream/spawn/ms 1.86, stream/tick/ms 1.68 |
| warm-up | 4.18 | 45.5 | render/thread/ms 73.72, render/busy/ms 72.19, main/busy/ms 31.07, stream/drive/ms 5.59, render/graph/ms 3.19, stream/spawn/ms 2.06, stream/tick/ms 1.73, render/present/ms 1.11, terrain/update/ms 1.04, terrain/select/ms 0.84, bigspace/total/ms 0.49, render/acquire/ms 0.42 |
| warm-up | 4.25 | 44.6 | pipelines/wait_ms 985, pipelines/compile_ms 176, pipelines/front_ms 162, main/busy/ms 51.73, render/thread/ms 40.51, render/busy/ms 39.34, stream/drive/ms 32.13, pipelines/driver_ms 14.06, render/graph/ms 3.64, stream/spawn/ms 1.67, stream/tick/ms 1.11, terrain/update/ms 0.96 |
| warm-up | 4.32 | 29.7 | render/thread/ms 42.48, render/busy/ms 39.71, main/busy/ms 18.95, render/graph/ms 13.75, stream/drive/ms 4.72, render/present/ms 2.55, stream/tick/ms 1.45, stream/spawn/ms 1.06, terrain/update/ms 0.95, terrain/select/ms 0.78, stream/activate/ms 0.70, bigspace/total/ms 0.43 |
| warm-up | 4.39 | 45.6 | pipelines/wait_ms 3669, pipelines/compile_ms 178, pipelines/front_ms 165, main/busy/ms 74.02, stream/drive/ms 55.07, render/thread/ms 53.10, render/busy/ms 50.91, pipelines/driver_ms 13.38, render/graph/ms 9.32, stream/spawn/ms 3.19, render/present/ms 1.91, stream/tick/ms 1.64 |
| warm-up | 4.51 | 113.6 | pipelines/wait_ms 5740, render/thread/ms 95.32, render/busy/ms 93.30, pipelines/compile_ms 76.15, main/busy/ms 72.01, pipelines/front_ms 71.42, stream/drive/ms 54.30, render/graph/ms 7.46, pipelines/driver_ms 4.73, stream/spawn/ms 2.08, render/present/ms 1.81, stream/tick/ms 1.78 |
| warm-up | 4.60 | 80.8 | pipelines/wait_ms 9334, pipelines/compile_ms 190, pipelines/front_ms 182, main/busy/ms 69.16, stream/drive/ms 49.51, render/thread/ms 32.92, render/busy/ms 31.53, pipelines/driver_ms 8.66, render/graph/ms 4.86, stream/spawn/ms 2.04, render/present/ms 1.19, stream/tick/ms 1.15 |
| warm-up | 4.69 | 68.3 | pipelines/wait_ms 5499, main/busy/ms 71.34, pipelines/compile_ms 67.74, pipelines/front_ms 63.11, render/thread/ms 43.89, render/busy/ms 42.58, stream/drive/ms 38.84, render/graph/ms 10.98, terrain/update/ms 7.16, pipelines/driver_ms 4.63, stream/spawn/ms 2.15, stream/tick/ms 1.29 |
| warm-up | 4.74 | 32.3 | main/busy/ms 39.60, stream/drive/ms 25.16, render/thread/ms 24.29, render/busy/ms 22.93, render/graph/ms 5.91, terrain/update/ms 2.09, stream/spawn/ms 1.56, render/present/ms 1.13, stream/activate/ms 1.08, physics/insert/ms 1.01, stream/tick/ms 0.73, terrain/select/ms 0.50 |
| warm-up | 5.50 | 33.2 | pipelines/wait_ms 1767, pipelines/compile_ms 173, pipelines/front_ms 160, render/thread/ms 54.88, render/busy/ms 53.14, main/busy/ms 23.78, pipelines/driver_ms 12.91, render/graph/ms 5.75, stream/drive/ms 4.64, stream/spawn/ms 2.08, render/present/ms 1.37, stream/tick/ms 0.73 |
| warm-up | 5.55 | 36.1 | render/thread/ms 43.53, render/busy/ms 42.37, main/busy/ms 22.22, stream/drive/ms 3.92, render/graph/ms 3.83, stream/spawn/ms 2.04, stream/tick/ms 1.02, bigspace/total/ms 0.97, render/present/ms 0.96, terrain/update/ms 0.79, bigspace/high_precision/ms 0.73, terrain/select/ms 0.67 |
| warm-up | 5.64 | 29.9 | pipelines/wait_ms 1192, pipelines/compile_ms 147, pipelines/front_ms 141, render/thread/ms 41.84, render/busy/ms 40.55, main/busy/ms 20.40, pipelines/driver_ms 5.34, render/graph/ms 3.74, stream/drive/ms 3.71, stream/spawn/ms 1.93, render/present/ms 1.08, stream/tick/ms 0.79 |
| warm-up | 5.68 | 31.1 | render/thread/ms 36.52, render/busy/ms 35.30, main/busy/ms 25.43, stream/drive/ms 8.89, render/graph/ms 4.08, stream/tick/ms 1.04, stream/spawn/ms 1.04, render/present/ms 0.92, terrain/update/ms 0.77, terrain/select/ms 0.67, bigspace/total/ms 0.55, render/acquire/ms 0.30 |
| warm-up | 5.86 | 28.7 | main/busy/ms 33.28, render/thread/ms 24.02, render/busy/ms 22.59, stream/drive/ms 13.22, render/graph/ms 7.10, stream/spawn/ms 2.25, render/present/ms 1.23, bigspace/total/ms 0.75, terrain/update/ms 0.71, stream/tick/ms 0.71, terrain/select/ms 0.60, bigspace/high_precision/ms 0.31 |
| warm-up | 6.07 | 28.8 | pipelines/wait_ms 5592, pipelines/compile_ms 138, pipelines/front_ms 131, render/thread/ms 38.64, render/busy/ms 37.04, main/busy/ms 36.64, pipelines/driver_ms 7.22, render/graph/ms 4.75, stream/drive/ms 4.46, stream/spawn/ms 2.10, render/present/ms 1.38, stream/tick/ms 0.95 |
| warm-up | 6.13 | 36.9 | pipelines/wait_ms 21.4 k, pipelines/compile_ms 72.76, pipelines/front_ms 67.02, render/thread/ms 44.13, render/busy/ms 42.87, main/busy/ms 36.60, render/graph/ms 5.90, pipelines/driver_ms 5.74, terrain/update/ms 2.80, stream/drive/ms 2.72, stream/spawn/ms 1.38, physics/insert/ms 1.13 |
| warm-up | 6.19 | 34.6 | pipelines/wait_ms 6555, main/busy/ms 42.24, render/thread/ms 39.75, render/busy/ms 38.20, pipelines/compile_ms 19.08, pipelines/front_ms 17.89, render/graph/ms 6.85, stream/drive/ms 3.39, stream/spawn/ms 2.07, physics/insert/ms 1.51, render/present/ms 1.31, pipelines/driver_ms 1.19 |
| warm-up | 6.27 | 36.1 | main/busy/ms 49.22, render/thread/ms 43.02, render/busy/ms 41.60, render/graph/ms 7.73, stream/drive/ms 3.53, physics/insert/ms 1.63, stream/spawn/ms 1.48, bigspace/total/ms 1.11, render/present/ms 1.11, anim/evaluate_cpu/ms 1.02, stream/tick/ms 0.77, physics/step/ms 0.67 |
| measure | 18.18 | 31.2 | main/busy/ms 38.97, render/thread/ms 22.64, render/busy/ms 21.60, stream/drive/ms 20.87, render/graph/ms 11.49, render/present/ms 0.88, bigspace/total/ms 0.79, controller/feed/ms 0.63, terrain/update/ms 0.56, terrain/select/ms 0.50, far/select/ms 0.38, stream/tick/ms 0.34 |
| measure | 23.09 | 28.8 | render/thread/ms 43.25, render/busy/ms 42.01, main/busy/ms 29.76, render/graph/ms 13.87, bigspace/total/ms 1.23, render/present/ms 1.09, stream/drive/ms 1.01, controller/feed/ms 0.83, anim/evaluate_cpu/ms 0.62, terrain/update/ms 0.60, terrain/select/ms 0.54, bigspace/low_precision/ms 0.52 |
| measure | 30.77 | 31.1 | render/thread/ms 42.76, render/busy/ms 27.70, main/busy/ms 22.20, render/graph/ms 16.15, render/present/ms 14.87, bigspace/total/ms 1.36, stream/drive/ms 0.80, stream/tick/ms 0.74, controller/feed/ms 0.68, terrain/update/ms 0.66, terrain/select/ms 0.61, far/select/ms 0.52 |
| measure | 59.05 | 34.1 | render/thread/ms 41.44, render/busy/ms 40.29, main/busy/ms 26.84, render/graph/ms 10.04, bigspace/total/ms 1.26, controller/feed/ms 1.22, stream/drive/ms 1.10, terrain/update/ms 1.01, stream/tick/ms 0.98, render/present/ms 0.94, terrain/select/ms 0.90, anim/evaluate_cpu/ms 0.80 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_pbr\balmora_walk@2560x1440_off=normalmaps_orm_parallax-2 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **84%** (min 60%) · 2579 MHz (min 2565) · memory 8001 MHz · 80.9 W · 77 °C (max 82) · VRAM 6251 MiB · throttled 100% of samples
