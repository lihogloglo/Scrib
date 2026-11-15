# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4861 frames over 65.7 s · **median 13.36 ms (75 fps)** · mean 13.51 · p95 15.99 · p99 18.98 (1% low 53 fps) · worst 36.72 ms · 34 spikes over 1.5× median · 26.5 stutters/min (work 21.9/min)
- warm-up: 443 frames over 9.8 s, worst 2668.9 ms
- hitches (> 33.4 ms): **3** measured, 18 in warm-up
- timed GPU passes: 9.95 ms of a 13.51 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.36 ms, limit 11 ms
- **FAIL** frame_ms: 29 stutters (> 1.5x moving median of 60) in 1.09 min = 26.50/min, limit 1/min
- **FAIL** worst frame 36.72 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.927 | 4.121 | 5.203 | 0.95 | 0.064 |
| ssao | 0.788 | 0.827 | 1.424 | 0.95 | 0.009 |
| early prepass | 0.678 | 0.711 | 1.242 | 0.95 | 0.103 |
| clouds_composite | 0.452 | 0.474 | 0.801 | 0.95 | 0.002 |
| taa | 0.398 | 0.418 | 0.733 | 0.95 | 0.002 |
| bloom | 0.326 | 0.342 | 0.482 | 0.95 | 0.074 |
| atmosphere_luts | 0.321 | 0.337 | 0.536 | 0.95 | 0.004 |
| clouds_trace | 0.319 | 0.334 | 0.620 | 0.95 | 0.001 |
| water_velocity | 0.300 | 0.315 | 0.502 | 0.95 | 0.002 |
| water_sspr | 0.239 | 0.251 | 0.377 | 0.95 | 0.003 |
| uniform_allocation | 0.219 | 0.230 | 0.232 | 0.95 | 0.043 |
| render_sky | 0.210 | 0.221 | 0.216 | 0.95 | 0.002 |
| auxiliary_views | 0.189 | 0.199 | 0.740 | 0.95 | 0.193 |
| clouds_resolve | 0.185 | 0.194 | 0.204 | 0.95 | 0.001 |
| water_snapshot | 0.178 | 0.187 | 0.307 | 0.95 | 0.002 |
| bin_unpacking | 0.166 | 0.174 | 0.184 | 0.95 | 0.031 |
| atmos_froxel_inject | 0.160 | 0.168 | 0.162 | 0.95 | 0.002 |
| water_fft | 0.138 | 0.145 | 0.140 | 0.95 | 0.003 |
| upscaling | 0.128 | 0.134 | 0.163 | 0.95 | 0.005 |
| main_transmissive_pass_3d | 0.120 | 0.126 | 0.373 | 0.95 | 0.009 |
| tonemapping | 0.090 | 0.094 | 0.094 | 0.95 | 0.004 |
| clustering | 0.067 | 0.070 | 0.075 | 0.95 | 0.022 |
| lightprobe_irradiance_map | 0.051 | 0.446 | 0.759 | 0.11 | 0.001 |
| surface_trail | 0.049 | 0.061 | 0.058 | 0.81 | 0.003 |
| early_mesh_preprocessing | 0.044 | 0.046 | 0.049 | 0.95 | 0.007 |
| atmos_froxel_integrate | 0.034 | 0.036 | 0.037 | 0.95 | 0.001 |
| clouds_shadow | 0.029 | 0.030 | 0.033 | 0.95 | 0.004 |
| lightprobe_radiance_map | 0.028 | 0.244 | 0.244 | 0.11 | 0.005 |
| per_view_shadow_pass | 0.024 | 0.025 | 0.034 | 0.95 | 0.008 |
| water_mips | 0.023 | 0.025 | 0.024 | 0.95 | 0.003 |
| night_sky | 0.019 | 0.020 | 0.048 | 0.95 | 0.002 |
| main_transparent_pass_3d | 0.016 | 0.017 | 0.032 | 0.95 | 0.121 |
| atmosphere_environment | 0.011 | 0.012 | 0.014 | 0.95 | 0.004 |
| clouds_fog_light | 0.010 | 0.011 | 0.011 | 0.95 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.95 | 0.004 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.013 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 12.227 | 14.549 | 35.26 | 59437.1 |
| render/busy/ms | 10.943 | 13.546 | 29.87 | 53191.9 |
| main/busy/ms | 9.776 | 11.771 | 26.95 | 47523.3 |
| pipelines/wait_ms | 8.518 | 0.000 | 11987.12 | 41406.8 |
| render/graph/ms | 6.200 | 7.814 | 20.68 | 30140.2 |
| render/acquire/ms | 0.681 | 2.618 | 5.92 | 3308.2 |
| render/present/ms | 0.604 | 1.493 | 21.95 | 2937.0 |
| bigspace/total/ms | 0.540 | 1.138 | 3.59 | 2624.6 |
| controller/feed/ms | 0.333 | 0.447 | 0.86 | 1618.3 |
| terrain/update/ms | 0.321 | 0.422 | 2.31 | 1561.1 |
| terrain/select/ms | 0.285 | 0.373 | 0.67 | 1385.6 |
| stream/drive/ms | 0.259 | 0.384 | 17.97 | 1261.1 |
| pipelines/compile_ms | 0.238 | 0.000 | 178.10 | 1158.0 |
| pipelines/front_ms | 0.230 | 0.000 | 172.50 | 1120.4 |
| bigspace/low_precision/ms | 0.208 | 0.355 | 3.22 | 1011.7 |
| far/select/ms | 0.205 | 0.301 | 0.70 | 994.9 |
| stream/tick/ms | 0.197 | 0.288 | 0.69 | 957.7 |
| anim/evaluate_cpu/ms | 0.171 | 0.226 | 0.50 | 831.6 |
| anim/evaluate/ms | 0.164 | 0.300 | 2.31 | 799.5 |
| controller/move/ms | 0.150 | 0.316 | 0.70 | 731.0 |
| bigspace/high_precision/ms | 0.140 | 0.305 | 2.79 | 681.5 |
| physics/step/ms | 0.063 | 0.100 | 0.51 | 306.9 |
| physics/insert/ms | 0.059 | 0.465 | 1.16 | 285.4 |
| weather/map/ms | 0.027 | 0.036 | 0.31 | 132.2 |
| anim/apply/ms | 0.013 | 0.024 | 0.29 | 64.3 |
| weather/eval/ms | 0.012 | 0.016 | 0.22 | 57.7 |
| controller/avoid/ms | 0.011 | 0.015 | 0.18 | 55.4 |
| script/update/ms | 0.008 | 0.011 | 0.19 | 38.3 |
| pipelines/driver_ms | 0.008 | 0.000 | 5.59 | 37.6 |
| stream/despawn/ms | 0.006 | 0.000 | 0.93 | 31.4 |
| script/ui/ms | 0.006 | 0.008 | 0.06 | 28.0 |
| stream/spawn/ms | 0.006 | 0.000 | 1.16 | 27.8 |
| script/input/ms | 0.005 | 0.007 | 0.08 | 25.2 |
| controller/write/ms | 0.004 | 0.005 | 0.04 | 17.7 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.60 | 15.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 4.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.07 | 3.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 3 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 561 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1303 | 2262 | 601 |
| crowd/c0/meshes/count | count | 355 | 646 | 159 |
| crowd/c1/count | count | 145 | 156 | 148 |
| crowd/c1/joints/count | count | 5503 | 5991 | 5595 |
| crowd/c1/meshes/count | count | 1166 | 1295 | 1182 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4258 | 9951 | 6179 |
| crowd/joints_uploaded/count | count | 4239 | 6643 | 6179 |
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
| frame/work/ms | ms | 11.63 | 29.87 | 11.60 |
| gpu/timed/ms | ms | 11.24 | 15.19 | 11.60 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 559.62 M | 560.80 M | 560.80 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 188 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 854 | 1416 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 188 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 854 | 1416 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1289 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3501 | 5162 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1289 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3501 | 5162 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 54 | 304 | 39 |
| phases/shadow/bins | count | 3914 | 5456 | 2236 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9709 | 12.5 k | 5687 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 147 | 315 | 39 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2720 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 389 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 201 | 205 | 205 |
| pipelines/waiting | count | 0 | 7 | 0 |
| preprocess/bind_groups | count | 12 | 777 | 7 |
| preprocess/bind_groups_created | count | 12 | 777 | 7 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 224 | 353 | 214 |
| process/cpu_usage | % | 29.96 | 45.67 | 32.08 |
| process/mem_usage | GiB | 5.43 | 5.85 | 5.85 |
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
| started_load_count |  | 4769 | 4864 | 4864 |
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
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 337 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 46.63 | 64.16 | 38.14 |
| system/mem_usage | % | 78.78 | 79.93 | 78.48 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 111 | 356 | 99 |
| views/all/bins | count | 6576 | 8466 | 5716 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.7 k | 25.4 k | 13.3 k |
| views/all/pipelines | count | 201 | 371 | 95 |
| views/all/pipelines_distinct | count | 81 | 94 | 77 |
| views/all/work_items | count | 17.6 k | 24.1 k | 13.6 k |
| views/cascade/batch_sets | count | 49 | 63 | 39 |
| views/cascade/bins | count | 2630 | 3157 | 2236 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8233 | 10.0 k | 5687 |
| views/cascade/pipelines | count | 49 | 63 | 39 |
| views/cascade/work_items | count | 8233 | 10.0 k | 5687 |
| views/lamp_face/batch_sets | count | 6 | 252 | 0 |
| views/lamp_face/bins | count | 1154 | 2604 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1328 | 2922 | 0 |
| views/lamp_face/pipelines | count | 97 | 262 | 0 |
| views/lamp_face/work_items | count | 81 | 2768 | 0 |
| views/main/batch_sets | count | 58 | 64 | 60 |
| views/main/bins | count | 2868 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8720 | 13.1 k | 7592 |
| views/main/pipelines | count | 54 | 60 | 56 |
| views/main/work_items | count | 8869 | 13.3 k | 7900 |
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
| anim/bones_written/count | count | 66.81 | 245 | 324.8 k |
| anim/events/count | count | 0.12 | 3 | 566 |
| assets/Image/added | count | 0.12 | 43 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.84 | 152 | 4075 |
| assets/Mesh/removed | count | 1.00 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 35 | 466 |
| assets/StandardMaterial/modified | count | 4.48 | 29 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.97 | 1413 | 4719 |
| physics/inserts | count | 0.53 | 109 | 2600 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.48 | 145 | 2335 |
| physics/steps | count | 0.81 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 20 | 378 |
| stream/upload/bytes | bytes | 6964 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1527 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 33.4 ms; slowest 21, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.67 | 2668.9 | main/busy/ms 2644, surface/advance_cpu_ms 67.08, surface/cell_upload_ms 0.79, stream/drive/ms 0.32, weather/map/ms 0.14, bigspace/total/ms 0.10, stream/tick/ms 0.09, bigspace/high_precision/ms 0.07, script/update/ms 0.03, weather/eval/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.25 | 580.4 | main/busy/ms 2682, render/thread/ms 540, render/busy/ms 515, surface/advance_cpu_ms 67.08, render/acquire/ms 22.00, render/graph/ms 12.78, render/present/ms 2.15, stream/drive/ms 1.23, surface/cell_upload_ms 0.79, stream/tick/ms 0.55, terrain/update/ms 0.50, bigspace/total/ms 0.34 |
| warm-up | 3.40 | 151.9 | render/thread/ms 582, render/busy/ms 555, pipelines/wait_ms 427, pipelines/compile_ms 138, pipelines/front_ms 130, main/busy/ms 56.34, render/graph/ms 45.34, render/acquire/ms 22.18, terrain/update/ms 19.93, pipelines/driver_ms 7.70, render/present/ms 3.96, stream/drive/ms 1.58 |
| warm-up | 3.64 | 104.8 | pipelines/compile_ms 228, pipelines/front_ms 205, main/busy/ms 75.19, render/thread/ms 68.34, render/busy/ms 65.82, pipelines/wait_ms 41.87, pipelines/driver_ms 23.03, render/graph/ms 3.99, render/present/ms 2.18, stream/drive/ms 2.05, stream/tick/ms 1.77, terrain/update/ms 0.99 |
| warm-up | 3.80 | 49.6 | main/busy/ms 52.21, render/thread/ms 51.05, render/busy/ms 49.61, stream/drive/ms 35.78, render/graph/ms 2.74, stream/tick/ms 1.69, render/present/ms 1.25, terrain/update/ms 0.96, terrain/select/ms 0.81, bigspace/total/ms 0.26, stream/spawn/ms 0.23, render/acquire/ms 0.19 |
| warm-up | 3.92 | 47.0 | pipelines/wait_ms 3484, pipelines/compile_ms 310, pipelines/front_ms 293, render/thread/ms 63.15, render/busy/ms 61.07, main/busy/ms 40.41, stream/drive/ms 18.79, pipelines/driver_ms 17.32, render/graph/ms 5.98, render/present/ms 1.81, stream/spawn/ms 1.55, stream/tick/ms 1.50 |
| warm-up | 4.06 | 119.1 | pipelines/wait_ms 2260, pipelines/compile_ms 109, pipelines/front_ms 103, render/thread/ms 102, render/busy/ms 99.86, main/busy/ms 64.23, stream/drive/ms 38.94, pipelines/driver_ms 6.10, render/graph/ms 3.70, stream/spawn/ms 1.98, render/present/ms 1.74, stream/tick/ms 1.46 |
| warm-up | 4.11 | 51.0 | pipelines/wait_ms 5522, pipelines/compile_ms 212, pipelines/front_ms 191, render/thread/ms 125, render/busy/ms 124, main/busy/ms 70.10, stream/drive/ms 38.64, pipelines/driver_ms 20.85, render/graph/ms 4.73, stream/tick/ms 1.35, stream/spawn/ms 1.27, render/present/ms 1.26 |
| warm-up | 4.14 | 33.8 | pipelines/wait_ms 3262, pipelines/compile_ms 103, pipelines/front_ms 87.83, render/thread/ms 67.42, render/busy/ms 65.45, main/busy/ms 38.97, pipelines/driver_ms 14.75, render/graph/ms 9.72, stream/drive/ms 8.64, render/present/ms 1.64, terrain/update/ms 1.50, terrain/select/ms 1.28 |
| warm-up | 4.40 | 137.1 | pipelines/wait_ms 5757, pipelines/compile_ms 261, pipelines/front_ms 247, render/thread/ms 98.01, main/busy/ms 75.41, render/busy/ms 60.55, stream/drive/ms 52.79, render/present/ms 37.26, pipelines/driver_ms 13.28, render/graph/ms 4.22, stream/spawn/ms 2.31, stream/tick/ms 1.49 |
| warm-up | 4.50 | 80.5 | main/busy/ms 81.66, stream/drive/ms 56.82, render/thread/ms 49.23, render/busy/ms 47.77, render/graph/ms 10.07, stream/spawn/ms 2.93, stream/tick/ms 1.12, render/present/ms 1.11, terrain/update/ms 0.95, terrain/select/ms 0.77, stream/activate/ms 0.52, bigspace/total/ms 0.52 |
| warm-up | 4.60 | 77.7 | pipelines/wait_ms 5338, pipelines/compile_ms 247, pipelines/front_ms 237, main/busy/ms 65.01, render/thread/ms 45.62, render/busy/ms 44.32, stream/drive/ms 40.42, pipelines/driver_ms 10.40, render/graph/ms 5.64, stream/spawn/ms 2.15, stream/activate/ms 1.18, render/present/ms 1.13 |
| warm-up | 4.65 | 40.4 | main/busy/ms 41.05, stream/drive/ms 27.99, render/thread/ms 22.59, render/busy/ms 20.88, render/graph/ms 2.77, render/present/ms 1.52, stream/tick/ms 0.99, stream/activate/ms 0.95, stream/spawn/ms 0.83, terrain/update/ms 0.62, terrain/select/ms 0.51, physics/insert/ms 0.41 |
| warm-up | 4.73 | 34.2 | pipelines/wait_ms 3575, pipelines/compile_ms 156, pipelines/front_ms 151, render/thread/ms 48.38, render/busy/ms 46.49, main/busy/ms 24.06, stream/drive/ms 5.54, pipelines/driver_ms 4.80, render/graph/ms 4.26, stream/spawn/ms 2.77, render/present/ms 1.57, terrain/update/ms 0.89 |
| warm-up | 5.46 | 44.3 | pipelines/wait_ms 1946, pipelines/compile_ms 74.16, pipelines/front_ms 71.67, render/thread/ms 48.33, render/busy/ms 46.43, main/busy/ms 46.17, render/graph/ms 8.41, stream/drive/ms 4.75, stream/spawn/ms 2.59, pipelines/driver_ms 2.48, render/present/ms 1.61, stream/tick/ms 1.07 |
| warm-up | 5.51 | 48.1 | pipelines/wait_ms 1946, pipelines/compile_ms 74.16, pipelines/front_ms 71.67, render/thread/ms 62.89, render/busy/ms 61.62, main/busy/ms 46.93, render/graph/ms 5.82, stream/drive/ms 3.81, pipelines/driver_ms 2.48, stream/spawn/ms 2.04, stream/tick/ms 1.07, render/present/ms 1.04 |
| warm-up | 7.62 | 37.5 | main/busy/ms 58.64, render/thread/ms 47.97, render/busy/ms 46.34, render/graph/ms 7.85, stream/drive/ms 1.56, bigspace/total/ms 1.31, anim/evaluate_cpu/ms 1.30, render/present/ms 1.09, physics/insert/ms 0.70, stream/tick/ms 0.66, bigspace/low_precision/ms 0.65, terrain/update/ms 0.60 |
| warm-up | 7.65 | 33.6 | render/thread/ms 44.08, render/busy/ms 42.83, main/busy/ms 41.04, render/graph/ms 7.56, stream/drive/ms 1.21, bigspace/total/ms 1.18, render/present/ms 0.99, anim/evaluate_cpu/ms 0.74, stream/tick/ms 0.56, terrain/update/ms 0.56, physics/insert/ms 0.55, bigspace/low_precision/ms 0.49 |
| measure | 8.26 | 34.7 | render/thread/ms 44.70, render/present/ms 22.56, render/busy/ms 21.93, main/busy/ms 18.89, render/graph/ms 12.74, bigspace/total/ms 1.37, controller/feed/ms 0.78, bigspace/low_precision/ms 0.75, terrain/update/ms 0.60, terrain/select/ms 0.54, stream/drive/ms 0.44, stream/tick/ms 0.39 |
| measure | 58.27 | 36.7 | render/thread/ms 46.78, render/present/ms 22.36, render/busy/ms 20.92, main/busy/ms 16.97, render/graph/ms 11.08, render/acquire/ms 3.50, bigspace/total/ms 0.98, controller/feed/ms 0.65, stream/drive/ms 0.60, terrain/update/ms 0.59, terrain/select/ms 0.53, stream/tick/ms 0.53 |
| measure | 59.06 | 35.8 | render/thread/ms 42.30, render/busy/ms 41.06, main/busy/ms 27.53, render/graph/ms 10.47, bigspace/total/ms 1.28, render/present/ms 1.04, controller/feed/ms 0.99, controller/move/ms 0.90, terrain/update/ms 0.86, far/select/ms 0.86, terrain/select/ms 0.76, stream/drive/ms 0.64 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\claude\3866c27c11752a1d\4ff90442-bcd4-4820-b03c-4dd8a81992a4\scratchpad\bin\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\tc_pbr\balmora_walk@2560x1440_off=normalmaps_orm_parallax-3 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 15%
- during the run: other processes 17% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **94%** (min 64%) · 2570 MHz (min 2565) · memory 8001 MHz · 91.2 W · 83 °C (max 86) · VRAM 5688 MiB · throttled 100% of samples
