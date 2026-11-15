# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: waterssr,shorewaves,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5193 frames over 65.7 s · **median 12.48 ms (80 fps)** · mean 12.64 · p95 15.19 · p99 17.28 (1% low 58 fps) · worst 37.33 ms · 27 spikes over 1.5× median · 21.9 stutters/min (work 16.4/min)
- warm-up: 335 frames over 8.1 s, worst 2490.9 ms
- hitches (> 31.2 ms): **2** measured, 22 in warm-up
- timed GPU passes: 9.25 ms of a 12.64 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.48 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.93/min, limit 1/min
- **FAIL** worst frame 37.33 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.829 | 4.050 | 5.053 | 0.95 | 0.074 |
| ssao | 0.762 | 0.806 | 1.117 | 0.95 | 0.008 |
| early prepass | 0.654 | 0.691 | 1.138 | 0.95 | 0.100 |
| clouds_composite | 0.439 | 0.464 | 0.796 | 0.95 | 0.002 |
| taa | 0.390 | 0.413 | 0.733 | 0.95 | 0.002 |
| clouds_trace | 0.321 | 0.339 | 0.618 | 0.95 | 0.001 |
| bloom | 0.320 | 0.339 | 0.483 | 0.95 | 0.067 |
| atmosphere_luts | 0.315 | 0.334 | 0.603 | 0.95 | 0.004 |
| uniform_allocation | 0.214 | 0.227 | 0.233 | 0.95 | 0.045 |
| render_sky | 0.204 | 0.216 | 0.213 | 0.95 | 0.002 |
| auxiliary_views | 0.192 | 0.203 | 0.788 | 0.95 | 0.189 |
| clouds_resolve | 0.183 | 0.194 | 0.204 | 0.95 | 0.001 |
| water_snapshot | 0.171 | 0.181 | 0.230 | 0.95 | 0.002 |
| bin_unpacking | 0.165 | 0.174 | 0.186 | 0.95 | 0.030 |
| atmos_froxel_inject | 0.151 | 0.160 | 0.160 | 0.95 | 0.002 |
| water_fft | 0.132 | 0.140 | 0.138 | 0.95 | 0.003 |
| upscaling | 0.123 | 0.130 | 0.147 | 0.95 | 0.004 |
| water_velocity | 0.104 | 0.110 | 0.113 | 0.95 | 0.002 |
| main_transmissive_pass_3d | 0.089 | 0.094 | 0.255 | 0.95 | 0.009 |
| tonemapping | 0.088 | 0.093 | 0.093 | 0.95 | 0.004 |
| clustering | 0.065 | 0.068 | 0.075 | 0.95 | 0.020 |
| lightprobe_irradiance_map | 0.048 | 0.437 | 0.577 | 0.11 | 0.001 |
| surface_trail | 0.046 | 0.060 | 0.058 | 0.76 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.045 | 0.049 | 0.95 | 0.007 |
| atmos_froxel_integrate | 0.033 | 0.035 | 0.036 | 0.95 | 0.001 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.95 | 0.004 |
| lightprobe_radiance_map | 0.026 | 0.238 | 0.241 | 0.11 | 0.005 |
| per_view_shadow_pass | 0.023 | 0.024 | 0.031 | 0.95 | 0.008 |
| water_mips | 0.022 | 0.024 | 0.024 | 0.95 | 0.003 |
| night_sky | 0.019 | 0.020 | 0.048 | 0.95 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.031 | 0.95 | 0.117 |
| atmosphere_environment | 0.010 | 0.011 | 0.013 | 0.95 | 0.003 |
| clouds_fog_light | 0.010 | 0.011 | 0.011 | 0.95 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.95 | 0.003 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.014 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 31.056 | 0.000 | 17405.32 | 161274.6 |
| render/thread/ms | 11.402 | 13.774 | 29.91 | 59211.4 |
| render/busy/ms | 10.366 | 12.841 | 29.22 | 53832.0 |
| main/busy/ms | 9.412 | 11.160 | 25.54 | 48878.6 |
| render/graph/ms | 5.791 | 7.410 | 18.75 | 30071.6 |
| render/present/ms | 0.586 | 1.486 | 16.87 | 3041.5 |
| bigspace/total/ms | 0.505 | 0.953 | 2.84 | 2621.2 |
| pipelines/compile_ms | 0.488 | 0.000 | 158.98 | 2531.7 |
| pipelines/front_ms | 0.471 | 0.000 | 152.39 | 2448.0 |
| render/acquire/ms | 0.450 | 1.771 | 3.61 | 2337.8 |
| terrain/update/ms | 0.311 | 0.415 | 2.00 | 1616.1 |
| controller/feed/ms | 0.305 | 0.407 | 0.81 | 1582.7 |
| terrain/select/ms | 0.277 | 0.369 | 0.78 | 1436.7 |
| stream/drive/ms | 0.254 | 0.372 | 17.19 | 1321.2 |
| far/select/ms | 0.201 | 0.294 | 0.61 | 1045.8 |
| bigspace/low_precision/ms | 0.199 | 0.360 | 2.51 | 1032.1 |
| stream/tick/ms | 0.195 | 0.281 | 0.62 | 1015.2 |
| anim/evaluate_cpu/ms | 0.170 | 0.216 | 0.50 | 885.4 |
| anim/evaluate/ms | 0.144 | 0.305 | 2.19 | 746.2 |
| bigspace/high_precision/ms | 0.124 | 0.271 | 2.19 | 643.5 |
| controller/move/ms | 0.123 | 0.232 | 0.61 | 636.9 |
| physics/insert/ms | 0.060 | 0.473 | 1.27 | 309.8 |
| physics/step/ms | 0.057 | 0.093 | 0.90 | 297.1 |
| weather/map/ms | 0.026 | 0.035 | 0.34 | 134.2 |
| pipelines/driver_ms | 0.016 | 0.000 | 6.59 | 83.7 |
| anim/apply/ms | 0.014 | 0.025 | 0.29 | 71.9 |
| weather/eval/ms | 0.011 | 0.015 | 0.32 | 56.6 |
| controller/avoid/ms | 0.010 | 0.014 | 0.22 | 54.4 |
| script/update/ms | 0.008 | 0.009 | 0.18 | 39.6 |
| stream/despawn/ms | 0.006 | 0.000 | 1.31 | 30.1 |
| script/input/ms | 0.005 | 0.007 | 0.16 | 27.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 26.6 |
| script/ui/ms | 0.005 | 0.006 | 0.08 | 24.5 |
| controller/write/ms | 0.003 | 0.005 | 0.20 | 18.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.67 | 15.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.08 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.08 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 21 |
| controller/movers/count | count | 20 | 21 | 21 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1306 | 2262 | 537 |
| crowd/c0/meshes/count | count | 355 | 646 | 146 |
| crowd/c1/count | count | 146 | 157 | 149 |
| crowd/c1/joints/count | count | 5512 | 6037 | 5659 |
| crowd/c1/meshes/count | count | 1167 | 1311 | 1195 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4253 | 10.0 k | 6179 |
| crowd/joints_uploaded/count | count | 4231 | 6643 | 6179 |
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
| frame/work/ms | ms | 10.93 | 29.22 | 10.85 |
| gpu/timed/ms | ms | 10.64 | 12.79 | 10.85 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 559.62 M | 560.80 M | 560.80 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 189 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 855 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 189 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 855 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1293 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3506 | 5142 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1293 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3506 | 5142 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 51 | 295 | 41 |
| phases/shadow/bins | count | 3907 | 5478 | 2242 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9709 | 12.8 k | 5697 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 151 | 297 | 41 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2708 | 3268 | 2350 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 906 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 201 | 205 | 205 |
| pipelines/waiting | count | 0 | 41 | 0 |
| preprocess/bind_groups | count | 11 | 759 | 10 |
| preprocess/bind_groups_created | count | 11 | 759 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 226 | 365 | 215 |
| process/cpu_usage | % | 29.52 | 44.28 | 29.81 |
| process/mem_usage | GiB | 5.27 | 5.72 | 5.72 |
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
| started_load_count |  | 4765 | 4859 | 4859 |
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
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 337 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 35.86 | 49.37 | 39.48 |
| system/mem_usage | % | 76.21 | 77.68 | 77.65 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2794 | 2906 | 2906 |
| textures/images | count | 2589 | 2644 | 2644 |
| views/all/batch_sets | count | 110 | 347 | 101 |
| views/all/bins | count | 6562 | 8225 | 5722 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.6 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 206 | 350 | 97 |
| views/all/pipelines_distinct | count | 83 | 94 | 77 |
| views/all/work_items | count | 17.7 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 49 | 62 | 41 |
| views/cascade/bins | count | 2629 | 3155 | 2242 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8218 | 10.2 k | 5697 |
| views/cascade/pipelines | count | 49 | 62 | 41 |
| views/cascade/work_items | count | 8218 | 10.2 k | 5697 |
| views/lamp_face/batch_sets | count | 0 | 243 | 0 |
| views/lamp_face/bins | count | 1160 | 2554 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1332 | 2912 | 0 |
| views/lamp_face/pipelines | count | 96 | 243 | 0 |
| views/lamp_face/work_items | count | 0 | 2617 | 0 |
| views/main/batch_sets | count | 58 | 64 | 60 |
| views/main/bins | count | 2886 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8742 | 13.0 k | 7600 |
| views/main/pipelines | count | 56 | 60 | 56 |
| views/main/work_items | count | 8894 | 13.2 k | 7908 |
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
| anim/bones_written/count | count | 64.44 | 237 | 334.6 k |
| anim/events/count | count | 0.10 | 3 | 510 |
| assets/Image/added | count | 0.11 | 43 | 594 |
| assets/Image/modified | count | 0.00 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.78 | 152 | 4075 |
| assets/Mesh/removed | count | 0.94 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 466 |
| assets/StandardMaterial/modified | count | 4.19 | 29 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.08 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.07 | 1328 | 5535 |
| physics/inserts | count | 0.61 | 122 | 3172 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.45 | 129 | 2335 |
| physics/steps | count | 0.76 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 20 | 378 |
| stream/upload/bytes | bytes | 6519 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1429 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.2 ms; slowest 24, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2490.9 | main/busy/ms 2468, surface/advance_cpu_ms 68.49, surface/cell_upload_ms 0.70, stream/drive/ms 0.32, bigspace/total/ms 0.16, weather/map/ms 0.15, stream/tick/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.03, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 2.94 | 451.0 | main/busy/ms 2500, render/thread/ms 419, render/busy/ms 398, surface/advance_cpu_ms 68.49, render/acquire/ms 19.07, render/graph/ms 3.88, render/present/ms 2.03, stream/drive/ms 1.02, surface/cell_upload_ms 0.70, stream/tick/ms 0.52, terrain/update/ms 0.44, bigspace/total/ms 0.27 |
| warm-up | 3.19 | 244.0 | pipelines/wait_ms 2238, render/thread/ms 458, render/busy/ms 435, pipelines/compile_ms 229, pipelines/front_ms 214, main/busy/ms 48.26, render/graph/ms 34.37, render/acquire/ms 19.14, terrain/update/ms 18.11, pipelines/driver_ms 14.49, render/present/ms 3.59, stream/drive/ms 1.41 |
| warm-up | 3.39 | 88.2 | main/busy/ms 61.88, render/thread/ms 57.26, render/busy/ms 55.73, render/graph/ms 2.95, stream/drive/ms 1.46, render/present/ms 1.26, stream/tick/ms 1.18, terrain/update/ms 0.74, terrain/select/ms 0.61, bigspace/total/ms 0.40, render/acquire/ms 0.28, bigspace/low_precision/ms 0.19 |
| warm-up | 3.43 | 31.3 | pipelines/compile_ms 200, pipelines/front_ms 186, pipelines/wait_ms 36.96, render/thread/ms 36.23, render/busy/ms 34.63, main/busy/ms 15.69, pipelines/driver_ms 14.64, render/graph/ms 2.39, stream/drive/ms 2.00, render/present/ms 1.30, stream/tick/ms 1.00, terrain/update/ms 0.97 |
| warm-up | 3.63 | 49.9 | pipelines/wait_ms 2164, pipelines/compile_ms 233, pipelines/front_ms 222, main/busy/ms 44.36, render/thread/ms 33.85, render/busy/ms 32.72, stream/drive/ms 30.96, pipelines/driver_ms 10.88, render/graph/ms 1.95, render/present/ms 0.98, stream/tick/ms 0.77, terrain/update/ms 0.71 |
| warm-up | 3.76 | 37.6 | pipelines/wait_ms 2079, pipelines/compile_ms 79.25, pipelines/front_ms 73.23, main/busy/ms 30.64, render/thread/ms 25.05, render/busy/ms 22.95, stream/drive/ms 16.18, pipelines/driver_ms 6.01, render/graph/ms 4.12, render/present/ms 1.94, stream/tick/ms 1.28, stream/spawn/ms 1.11 |
| warm-up | 3.82 | 37.1 | render/thread/ms 35.07, main/busy/ms 34.64, render/busy/ms 33.14, stream/drive/ms 11.35, stream/spawn/ms 2.79, render/graph/ms 2.74, render/present/ms 1.71, terrain/update/ms 1.11, stream/tick/ms 1.09, terrain/select/ms 0.89, bigspace/total/ms 0.53, bigspace/high_precision/ms 0.27 |
| warm-up | 3.86 | 40.0 | pipelines/wait_ms 1826, pipelines/compile_ms 108, pipelines/front_ms 101, render/thread/ms 53.54, render/busy/ms 51.72, main/busy/ms 49.35, stream/drive/ms 26.51, pipelines/driver_ms 6.73, render/graph/ms 3.25, stream/spawn/ms 2.42, stream/tick/ms 1.89, render/present/ms 1.60 |
| warm-up | 3.91 | 49.9 | pipelines/wait_ms 5118, pipelines/compile_ms 139, pipelines/front_ms 130, main/busy/ms 79.43, render/thread/ms 65.45, render/busy/ms 63.98, stream/drive/ms 59.82, pipelines/driver_ms 9.30, render/graph/ms 3.70, stream/tick/ms 2.11, stream/spawn/ms 2.03, render/present/ms 1.29 |
| warm-up | 3.95 | 48.2 | pipelines/wait_ms 3292, main/busy/ms 71.33, render/thread/ms 62.82, render/busy/ms 61.07, stream/drive/ms 49.99, pipelines/compile_ms 31.33, pipelines/front_ms 28.76, render/graph/ms 5.61, pipelines/driver_ms 2.57, stream/spawn/ms 2.27, stream/tick/ms 1.59, render/present/ms 1.49 |
| warm-up | 4.07 | 86.9 | pipelines/wait_ms 6223, pipelines/compile_ms 181, pipelines/front_ms 171, main/busy/ms 100, stream/drive/ms 76.98, render/thread/ms 75.44, render/busy/ms 73.19, render/graph/ms 19.92, pipelines/driver_ms 10.31, stream/spawn/ms 3.79, stream/tick/ms 2.57, render/present/ms 1.93 |
| warm-up | 4.18 | 110.0 | pipelines/wait_ms 5546, render/thread/ms 130, pipelines/compile_ms 98.53, render/busy/ms 94.09, pipelines/front_ms 92.69, main/busy/ms 71.87, stream/drive/ms 49.51, render/present/ms 35.22, render/graph/ms 10.68, pipelines/driver_ms 5.84, stream/spawn/ms 2.73, stream/tick/ms 2.27 |
| warm-up | 4.25 | 54.5 | pipelines/wait_ms 3954, pipelines/compile_ms 211, pipelines/front_ms 200, main/busy/ms 54.36, stream/drive/ms 32.82, render/thread/ms 28.66, render/busy/ms 27.60, pipelines/driver_ms 10.67, render/graph/ms 2.41, stream/spawn/ms 2.11, stream/tick/ms 1.24, stream/activate/ms 1.11 |
| warm-up | 4.56 | 33.6 | pipelines/wait_ms 5294, render/thread/ms 35.82, pipelines/compile_ms 35.29, render/busy/ms 34.54, pipelines/front_ms 33.23, main/busy/ms 30.50, stream/drive/ms 13.50, render/graph/ms 4.06, pipelines/driver_ms 2.07, stream/spawn/ms 1.23, render/present/ms 1.09, stream/tick/ms 1.06 |
| warm-up | 4.70 | 45.4 | main/busy/ms 43.29, render/thread/ms 41.69, render/busy/ms 40.09, render/graph/ms 7.44, terrain/update/ms 5.22, stream/drive/ms 1.78, render/present/ms 1.41, physics/insert/ms 0.83, stream/tick/ms 0.77, bigspace/total/ms 0.68, anim/evaluate_cpu/ms 0.59, terrain/select/ms 0.51 |
| warm-up | 5.36 | 58.3 | pipelines/wait_ms 880, pipelines/compile_ms 158, pipelines/front_ms 152, main/busy/ms 63.12, render/thread/ms 53.08, render/busy/ms 51.42, render/graph/ms 11.29, pipelines/driver_ms 5.68, stream/drive/ms 5.19, stream/spawn/ms 3.14, physics/insert/ms 1.57, render/present/ms 1.44 |
| warm-up | 5.40 | 34.3 | pipelines/wait_ms 880, pipelines/compile_ms 158, pipelines/front_ms 152, main/busy/ms 58.95, render/thread/ms 58.01, render/busy/ms 56.71, render/graph/ms 7.46, pipelines/driver_ms 5.68, stream/drive/ms 4.17, stream/spawn/ms 2.14, physics/insert/ms 1.65, bigspace/total/ms 1.51 |
| warm-up | 5.44 | 43.2 | main/busy/ms 56.17, render/thread/ms 53.20, render/busy/ms 51.81, stream/drive/ms 14.93, render/graph/ms 7.98, stream/spawn/ms 2.25, physics/insert/ms 1.27, render/present/ms 1.12, bigspace/total/ms 1.08, stream/tick/ms 1.04, terrain/update/ms 0.75, terrain/select/ms 0.62 |
| warm-up | 5.80 | 42.7 | main/busy/ms 42.58, render/thread/ms 25.36, render/busy/ms 24.39, stream/drive/ms 6.44, render/graph/ms 5.03, physics/insert/ms 1.36, render/present/ms 0.82, bigspace/total/ms 0.71, anim/evaluate_cpu/ms 0.70, stream/tick/ms 0.68, terrain/update/ms 0.56, terrain/select/ms 0.49 |
| warm-up | 6.12 | 42.7 | main/busy/ms 52.70, render/thread/ms 48.53, render/busy/ms 42.38, render/graph/ms 10.92, render/acquire/ms 4.43, render/present/ms 1.72, bigspace/total/ms 1.50, stream/drive/ms 1.36, terrain/update/ms 1.10, terrain/select/ms 0.99, physics/insert/ms 0.97, stream/tick/ms 0.61 |
| warm-up | 6.15 | 37.0 | render/thread/ms 54.89, render/busy/ms 53.49, main/busy/ms 43.57, render/graph/ms 6.69, stream/drive/ms 1.48, render/present/ms 1.21, terrain/update/ms 1.09, terrain/select/ms 0.98, physics/insert/ms 0.74, bigspace/total/ms 0.74, stream/tick/ms 0.69, far/select/ms 0.61 |
| measure | 19.27 | 32.7 | render/thread/ms 38.18, render/busy/ms 37.03, render/graph/ms 22.65, main/busy/ms 18.86, bigspace/total/ms 1.87, bigspace/high_precision/ms 1.21, render/present/ms 1.01, physics/insert/ms 0.99, controller/feed/ms 0.55, terrain/update/ms 0.55, terrain/select/ms 0.50, stream/drive/ms 0.46 |
| measure | 59.06 | 37.3 | render/thread/ms 44.90, render/busy/ms 43.68, main/busy/ms 26.14, render/graph/ms 10.34, bigspace/total/ms 1.70, controller/feed/ms 1.02, render/present/ms 0.99, stream/drive/ms 0.84, terrain/update/ms 0.77, stream/tick/ms 0.75, bigspace/high_precision/ms 0.75, far/select/ms 0.71 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\claude\3866c27c11752a1d\4ff90442-bcd4-4820-b03c-4dd8a81992a4\scratchpad\bin\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\tc_pbrwater\balmora_walk@2560x1440_off=normalmaps_orm_parallax_waterssr_shorewaves-2 --bench-baseline none --off normalmaps,orm,parallax,waterssr,shorewaves
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **94%** (min 65%) · 2566 MHz (min 2565) · memory 8001 MHz · 92.3 W · 83 °C (max 87) · VRAM 5432 MiB · throttled 100% of samples
