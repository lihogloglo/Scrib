# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4848 frames over 65.7 s · **median 13.19 ms (76 fps)** · mean 13.54 · p95 16.45 · p99 21.59 (1% low 46 fps) · worst 56.95 ms · 64 spikes over 1.5× median · 45.7 stutters/min (work 39.3/min)
- warm-up: 635 frames over 16.1 s, worst 4190.9 ms
- hitches (> 33.0 ms): **6** measured, 21 in warm-up
- timed GPU passes: 9.67 ms of a 13.54 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.19 ms, limit 11 ms
- **FAIL** frame_ms: 50 stutters (> 1.5x moving median of 60) in 1.09 min = 45.69/min, limit 1/min
- **FAIL** worst frame 56.95 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.821 | 4.048 | 5.095 | 0.94 | 0.067 |
| ssao | 0.756 | 0.801 | 1.110 | 0.94 | 0.009 |
| early prepass | 0.654 | 0.693 | 1.143 | 0.94 | 0.103 |
| clouds_composite | 0.437 | 0.463 | 0.783 | 0.94 | 0.002 |
| taa | 0.394 | 0.417 | 0.747 | 0.94 | 0.002 |
| clouds_trace | 0.322 | 0.341 | 0.616 | 0.94 | 0.001 |
| bloom | 0.319 | 0.338 | 0.497 | 0.94 | 0.073 |
| atmosphere_luts | 0.311 | 0.330 | 0.503 | 0.94 | 0.004 |
| water_velocity | 0.286 | 0.303 | 0.491 | 0.94 | 0.002 |
| water_sspr | 0.236 | 0.250 | 0.377 | 0.94 | 0.002 |
| uniform_allocation | 0.211 | 0.224 | 0.230 | 0.94 | 0.043 |
| render_sky | 0.202 | 0.214 | 0.213 | 0.94 | 0.002 |
| auxiliary_views | 0.183 | 0.194 | 0.691 | 0.94 | 0.187 |
| clouds_resolve | 0.182 | 0.193 | 0.203 | 0.94 | 0.001 |
| water_snapshot | 0.174 | 0.184 | 0.322 | 0.94 | 0.002 |
| bin_unpacking | 0.166 | 0.175 | 0.185 | 0.94 | 0.031 |
| atmos_froxel_inject | 0.152 | 0.161 | 0.159 | 0.94 | 0.002 |
| water_fft | 0.131 | 0.139 | 0.138 | 0.94 | 0.003 |
| upscaling | 0.127 | 0.135 | 0.163 | 0.94 | 0.005 |
| main_transmissive_pass_3d | 0.111 | 0.118 | 0.360 | 0.94 | 0.009 |
| tonemapping | 0.087 | 0.092 | 0.095 | 0.94 | 0.004 |
| clustering | 0.064 | 0.068 | 0.075 | 0.94 | 0.021 |
| lightprobe_irradiance_map | 0.050 | 0.444 | 0.757 | 0.11 | 0.001 |
| surface_trail | 0.048 | 0.060 | 0.058 | 0.80 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.045 | 0.049 | 0.94 | 0.006 |
| atmos_froxel_integrate | 0.033 | 0.035 | 0.036 | 0.94 | 0.001 |
| clouds_shadow | 0.029 | 0.030 | 0.033 | 0.94 | 0.004 |
| lightprobe_radiance_map | 0.027 | 0.237 | 0.243 | 0.11 | 0.005 |
| per_view_shadow_pass | 0.024 | 0.025 | 0.031 | 0.94 | 0.007 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.94 | 0.003 |
| night_sky | 0.019 | 0.020 | 0.048 | 0.94 | 0.002 |
| main_transparent_pass_3d | 0.016 | 0.017 | 0.033 | 0.94 | 0.123 |
| atmosphere_environment | 0.010 | 0.011 | 0.013 | 0.94 | 0.004 |
| clouds_fog_light | 0.010 | 0.011 | 0.011 | 0.94 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.94 | 0.004 |
| lightprobe_copy | 0.002 | 0.017 | 0.016 | 0.11 | 0.005 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.03 | 0.007 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 12.174 | 14.895 | 37.50 | 59018.4 |
| render/busy/ms | 11.275 | 14.069 | 32.36 | 54661.3 |
| main/busy/ms | 10.014 | 12.366 | 47.72 | 48545.8 |
| render/graph/ms | 6.496 | 8.263 | 19.72 | 31494.1 |
| render/present/ms | 0.610 | 1.597 | 23.36 | 2958.9 |
| bigspace/total/ms | 0.564 | 1.299 | 4.00 | 2736.0 |
| controller/feed/ms | 0.337 | 0.447 | 0.81 | 1635.0 |
| terrain/update/ms | 0.331 | 0.448 | 1.87 | 1602.8 |
| terrain/select/ms | 0.294 | 0.399 | 0.82 | 1425.9 |
| stream/drive/ms | 0.289 | 0.397 | 37.58 | 1399.2 |
| render/acquire/ms | 0.288 | 1.428 | 3.49 | 1398.2 |
| bigspace/low_precision/ms | 0.235 | 0.466 | 3.02 | 1137.0 |
| far/select/ms | 0.217 | 0.310 | 0.65 | 1050.6 |
| stream/tick/ms | 0.202 | 0.290 | 0.66 | 979.0 |
| anim/evaluate_cpu/ms | 0.193 | 0.252 | 0.60 | 937.7 |
| controller/move/ms | 0.168 | 0.377 | 0.75 | 815.0 |
| pipelines/compile_ms | 0.168 | 0.000 | 190.83 | 812.1 |
| pipelines/front_ms | 0.156 | 0.000 | 183.16 | 756.3 |
| bigspace/high_precision/ms | 0.141 | 0.292 | 3.44 | 682.2 |
| anim/evaluate/ms | 0.132 | 0.274 | 2.62 | 639.4 |
| physics/step/ms | 0.065 | 0.105 | 0.77 | 314.0 |
| pipelines/wait_ms | 0.061 | 0.000 | 139.68 | 297.5 |
| physics/insert/ms | 0.048 | 0.424 | 1.12 | 233.2 |
| weather/map/ms | 0.027 | 0.037 | 0.32 | 131.1 |
| anim/apply/ms | 0.014 | 0.025 | 0.24 | 66.0 |
| weather/eval/ms | 0.012 | 0.015 | 0.35 | 56.9 |
| pipelines/driver_ms | 0.012 | 0.000 | 19.84 | 55.8 |
| controller/avoid/ms | 0.011 | 0.015 | 0.22 | 55.6 |
| script/update/ms | 0.008 | 0.011 | 0.05 | 38.1 |
| stream/despawn/ms | 0.007 | 0.000 | 2.13 | 34.6 |
| stream/deactivate/ms | 0.006 | 0.000 | 13.96 | 30.3 |
| script/ui/ms | 0.006 | 0.008 | 0.16 | 28.8 |
| script/input/ms | 0.005 | 0.007 | 0.19 | 26.3 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 26.3 |
| controller/write/ms | 0.004 | 0.005 | 0.06 | 18.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.5 |
| stream/upload/ms | 0.000 | 0.000 | 0.21 | 1.4 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 20 | 29 | 20 |
| controller/movers/count | count | 20 | 23 | 20 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1306 | 2262 | 659 |
| crowd/c0/meshes/count | count | 355 | 646 | 171 |
| crowd/c1/count | count | 146 | 156 | 147 |
| crowd/c1/joints/count | count | 5518 | 6004 | 5537 |
| crowd/c1/meshes/count | count | 1152 | 1308 | 1170 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4256 | 7274 | 6179 |
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
| frame/work/ms | ms | 11.51 | 47.72 | 12.79 |
| gpu/timed/ms | ms | 11.02 | 15.06 | 11.17 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 191 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 835 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 191 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 835 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 1285 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3470 | 5131 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 22 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d_prepass/bins | count | 1285 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3470 | 5131 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 22 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 52 | 285 | 41 |
| phases/shadow/bins | count | 3829 | 5403 | 2242 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9627 | 12.7 k | 5697 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 146 | 303 | 41 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2674 | 3268 | 2349 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 207 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 200 | 204 | 204 |
| pipelines/waiting | count | 0 | 3 | 0 |
| preprocess/bind_groups | count | 12 | 739 | 10 |
| preprocess/bind_groups_created | count | 12 | 739 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 224 | 338 | 215 |
| process/cpu_usage | % | 28.10 | 49.04 | 33.65 |
| process/mem_usage | GiB | 5.19 | 5.62 | 5.62 |
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
| started_load_count |  | 4769 | 4862 | 4862 |
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
| stream/latency/s | s | 0.05 | 0.15 | 0.05 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.46 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 44.20 | 72.76 | 58.27 |
| system/mem_usage | % | 78.21 | 79.69 | 79.66 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 108 | 337 | 101 |
| views/all/bins | count | 6522 | 8102 | 5722 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.5 k | 25.2 k | 13.3 k |
| views/all/pipelines | count | 202 | 353 | 97 |
| views/all/pipelines_distinct | count | 82 | 92 | 77 |
| views/all/work_items | count | 17.4 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 49 | 63 | 41 |
| views/cascade/bins | count | 2621 | 3155 | 2242 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8186 | 10.2 k | 5697 |
| views/cascade/pipelines | count | 49 | 63 | 41 |
| views/cascade/work_items | count | 8186 | 10.2 k | 5697 |
| views/lamp_face/batch_sets | count | 0 | 232 | 0 |
| views/lamp_face/bins | count | 1113 | 2474 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1256 | 2849 | 0 |
| views/lamp_face/pipelines | count | 92 | 241 | 0 |
| views/lamp_face/work_items | count | 0 | 2501 | 0 |
| views/main/batch_sets | count | 58 | 62 | 60 |
| views/main/bins | count | 2862 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8600 | 13.0 k | 7600 |
| views/main/pipelines | count | 54 | 58 | 56 |
| views/main/work_items | count | 8754 | 13.2 k | 7908 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.7 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 58.84 | 248 | 285.3 k |
| anim/events/count | count | 0.12 | 4 | 596 |
| assets/Image/added | count | 0.12 | 19 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.84 | 152 | 4075 |
| assets/Mesh/removed | count | 1.01 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 29 | 466 |
| assets/StandardMaterial/modified | count | 4.49 | 24 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.97 | 1386 | 4716 |
| physics/inserts | count | 0.46 | 94 | 2243 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.48 | 161 | 2335 |
| physics/steps | count | 0.81 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 2 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.08 | 22 | 378 |
| stream/upload/bytes | bytes | 6983 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1531 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 33.0 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 4.19 | 4190.9 | main/busy/ms 4122, surface/advance_cpu_ms 68.52, stream/drive/ms 1.45, surface/cell_upload_ms 0.99, stream/tick/ms 0.79, weather/map/ms 0.40, bigspace/total/ms 0.35, weather/eval/ms 0.34, script/update/ms 0.28, bigspace/high_precision/ms 0.18, script/input/ms 0.11, weather/apply/ms 0.11 |
| warm-up | 5.00 | 811.1 | main/busy/ms 4178, render/thread/ms 773, render/busy/ms 736, surface/advance_cpu_ms 68.52, render/acquire/ms 31.24, render/graph/ms 20.81, render/present/ms 6.12, stream/drive/ms 3.82, stream/tick/ms 1.91, terrain/update/ms 1.00, surface/cell_upload_ms 0.99, weather/map/ms 0.48 |
| warm-up | 5.24 | 236.3 | pipelines/wait_ms 2133, render/thread/ms 819, render/busy/ms 780, pipelines/compile_ms 282, pipelines/front_ms 258, main/busy/ms 77.74, render/graph/ms 57.98, render/acquire/ms 31.33, pipelines/driver_ms 23.55, terrain/update/ms 18.84, render/present/ms 7.55, stream/drive/ms 3.02 |
| warm-up | 6.99 | 1157.3 | pipelines/wait_ms 76.2 k, pipelines/compile_ms 1238, main/busy/ms 1201, stream/drive/ms 1138, stream/activate/ms 1137, pipelines/front_ms 1037, pipelines/driver_ms 201, render/thread/ms 25.85, render/busy/ms 24.23, physics/insert/ms 4.94, render/graph/ms 4.70, terrain/update/ms 3.03 |
| warm-up | 7.07 | 77.9 | pipelines/wait_ms 76.2 k, pipelines/compile_ms 1241, pipelines/front_ms 1037, pipelines/driver_ms 204, main/busy/ms 72.32, render/thread/ms 56.28, render/busy/ms 54.76, terrain/update/ms 6.75, render/graph/ms 6.44, physics/insert/ms 4.63, physics/step/ms 2.21, stream/drive/ms 1.58 |
| warm-up | 7.13 | 52.3 | render/thread/ms 53.25, render/acquire/ms 30.31, main/busy/ms 25.25, render/busy/ms 21.23, render/graph/ms 10.48, terrain/update/ms 7.34, stream/drive/ms 3.39, stream/spawn/ms 1.81, render/present/ms 1.71, stream/tick/ms 1.02, terrain/select/ms 0.83, bigspace/total/ms 0.28 |
| warm-up | 7.18 | 42.2 | render/thread/ms 85.85, render/acquire/ms 61.00, render/busy/ms 23.12, main/busy/ms 18.89, render/graph/ms 9.89, terrain/update/ms 7.03, stream/drive/ms 2.99, render/present/ms 1.73, stream/spawn/ms 1.25, stream/tick/ms 1.14, terrain/select/ms 0.63, stream/upload/ms 0.26 |
| warm-up | 7.39 | 69.3 | main/busy/ms 87.48, stream/drive/ms 71.67, render/thread/ms 59.75, render/busy/ms 57.86, render/graph/ms 10.94, render/present/ms 1.64, stream/tick/ms 1.30, terrain/update/ms 0.72, terrain/select/ms 0.64, render/acquire/ms 0.26, bigspace/total/ms 0.21, stream/spawn/ms 0.12 |
| warm-up | 7.72 | 57.2 | main/busy/ms 60.67, stream/drive/ms 46.19, render/thread/ms 36.56, render/busy/ms 34.48, render/graph/ms 11.83, render/present/ms 1.65, stream/tick/ms 1.31, terrain/update/ms 0.67, terrain/select/ms 0.59, render/acquire/ms 0.44, bigspace/total/ms 0.23, stream/spawn/ms 0.13 |
| warm-up | 7.76 | 40.1 | render/thread/ms 57.72, render/busy/ms 55.78, render/graph/ms 33.61, main/busy/ms 16.11, render/present/ms 1.70, stream/drive/ms 1.62, stream/tick/ms 1.02, terrain/update/ms 0.68, terrain/select/ms 0.60, bigspace/total/ms 0.25, render/acquire/ms 0.24, physics/step/ms 0.11 |
| warm-up | 7.83 | 41.4 | pipelines/wait_ms 139, render/thread/ms 62.37, render/busy/ms 59.94, pipelines/compile_ms 29.04, pipelines/front_ms 27.57, render/graph/ms 26.94, main/busy/ms 20.20, render/present/ms 2.17, stream/drive/ms 2.05, pipelines/driver_ms 1.47, stream/tick/ms 1.03, terrain/update/ms 0.63 |
| warm-up | 8.19 | 34.0 | render/thread/ms 36.89, render/busy/ms 34.76, main/busy/ms 32.68, stream/drive/ms 18.43, render/graph/ms 10.07, stream/tick/ms 1.75, render/present/ms 1.67, terrain/update/ms 0.60, terrain/select/ms 0.53, render/acquire/ms 0.47, bigspace/total/ms 0.21, stream/spawn/ms 0.12 |
| warm-up | 8.57 | 74.6 | pipelines/wait_ms 1588, main/busy/ms 83.84, stream/drive/ms 68.86, render/thread/ms 31.52, pipelines/compile_ms 31.00, render/busy/ms 29.79, pipelines/front_ms 29.26, render/graph/ms 8.32, pipelines/driver_ms 1.73, render/present/ms 1.47, stream/tick/ms 0.96, terrain/update/ms 0.62 |
| warm-up | 8.79 | 83.5 | pipelines/wait_ms 983, pipelines/compile_ms 150, pipelines/front_ms 147, main/busy/ms 84.86, stream/drive/ms 69.89, render/thread/ms 30.50, render/busy/ms 28.97, render/graph/ms 6.88, pipelines/driver_ms 3.84, render/present/ms 1.29, stream/tick/ms 1.26, stream/activate/ms 1.07 |
| warm-up | 9.03 | 38.2 | pipelines/wait_ms 3828, pipelines/compile_ms 203, pipelines/front_ms 193, main/busy/ms 55.52, stream/drive/ms 42.14, render/thread/ms 25.53, render/busy/ms 24.03, pipelines/driver_ms 9.97, render/graph/ms 8.40, stream/tick/ms 1.40, render/present/ms 1.29, terrain/update/ms 0.57 |
| warm-up | 9.09 | 39.4 | pipelines/wait_ms 9333, pipelines/compile_ms 65.99, pipelines/front_ms 63.82, main/busy/ms 44.14, stream/drive/ms 29.08, render/thread/ms 27.91, render/busy/ms 26.24, render/graph/ms 9.94, pipelines/driver_ms 2.17, render/present/ms 1.48, stream/tick/ms 1.31, stream/activate/ms 1.05 |
| warm-up | 9.19 | 50.5 | main/busy/ms 52.93, render/thread/ms 27.50, render/busy/ms 25.90, stream/drive/ms 15.21, render/graph/ms 8.36, render/present/ms 1.38, stream/tick/ms 1.18, physics/insert/ms 0.89, terrain/update/ms 0.60, terrain/select/ms 0.52, physics/step/ms 0.47, bigspace/total/ms 0.45 |
| warm-up | 12.20 | 33.2 | main/busy/ms 39.30, render/thread/ms 31.16, render/busy/ms 29.46, stream/drive/ms 14.65, render/graph/ms 9.82, stream/spawn/ms 1.54, render/present/ms 1.45, bigspace/total/ms 1.06, stream/tick/ms 0.90, physics/insert/ms 0.75, bigspace/high_precision/ms 0.65, terrain/update/ms 0.65 |
| warm-up | 12.30 | 36.2 | main/busy/ms 44.21, render/thread/ms 35.04, render/busy/ms 33.35, stream/drive/ms 22.51, render/graph/ms 11.38, render/present/ms 1.41, stream/tick/ms 1.11, bigspace/total/ms 0.98, physics/insert/ms 0.78, anim/evaluate_cpu/ms 0.74, terrain/update/ms 0.61, terrain/select/ms 0.54 |
| warm-up | 12.53 | 36.4 | main/busy/ms 57.00, render/thread/ms 32.49, render/busy/ms 30.90, render/graph/ms 10.77, stream/drive/ms 1.38, render/present/ms 1.28, stream/tick/ms 1.00, terrain/update/ms 0.81, bigspace/total/ms 0.81, physics/insert/ms 0.81, terrain/select/ms 0.70, anim/evaluate_cpu/ms 0.52 |
| warm-up | 12.96 | 38.5 | main/busy/ms 43.02, render/thread/ms 27.72, render/busy/ms 26.37, stream/drive/ms 13.97, render/graph/ms 8.61, render/present/ms 1.13, terrain/update/ms 0.83, stream/tick/ms 0.82, terrain/select/ms 0.76, bigspace/total/ms 0.73, anim/evaluate_cpu/ms 0.63, controller/feed/ms 0.50 |
| measure | 0.82 | 57.0 | main/busy/ms 63.53, stream/drive/ms 37.92, render/thread/ms 31.31, render/busy/ms 29.99, render/graph/ms 12.90, bigspace/total/ms 1.34, render/present/ms 1.07, controller/feed/ms 0.71, bigspace/low_precision/ms 0.67, terrain/update/ms 0.58, far/select/ms 0.58, terrain/select/ms 0.52 |
| measure | 4.91 | 35.7 | main/busy/ms 42.81, render/thread/ms 27.04, render/busy/ms 25.79, stream/drive/ms 19.98, render/graph/ms 12.00, bigspace/total/ms 1.16, render/present/ms 1.05, controller/feed/ms 0.78, terrain/update/ms 0.68, bigspace/low_precision/ms 0.60, terrain/select/ms 0.59, anim/evaluate_cpu/ms 0.45 |
| measure | 18.21 | 45.6 | main/busy/ms 54.94, stream/drive/ms 32.03, render/thread/ms 27.15, render/busy/ms 26.02, render/graph/ms 13.69, bigspace/total/ms 1.91, bigspace/high_precision/ms 1.07, render/present/ms 0.95, controller/feed/ms 0.74, terrain/update/ms 0.71, terrain/select/ms 0.62, stream/tick/ms 0.54 |
| measure | 19.28 | 34.9 | render/thread/ms 48.52, render/busy/ms 47.18, render/graph/ms 27.47, main/busy/ms 24.23, bigspace/total/ms 1.78, render/present/ms 1.19, bigspace/low_precision/ms 0.99, stream/drive/ms 0.86, controller/feed/ms 0.79, anim/evaluate_cpu/ms 0.73, stream/tick/ms 0.70, terrain/update/ms 0.58 |
| measure | 48.27 | 35.9 | render/thread/ms 48.71, render/busy/ms 24.84, render/present/ms 23.67, main/busy/ms 20.28, render/graph/ms 14.25, bigspace/total/ms 1.13, controller/move/ms 0.92, controller/feed/ms 0.68, terrain/update/ms 0.64, terrain/select/ms 0.58, bigspace/low_precision/ms 0.50, anim/evaluate_cpu/ms 0.47 |
| measure | 63.23 | 39.1 | render/thread/ms 48.36, render/present/ms 23.80, render/busy/ms 22.39, main/busy/ms 19.76, render/graph/ms 12.95, render/acquire/ms 2.17, bigspace/total/ms 0.97, controller/feed/ms 0.78, terrain/update/ms 0.70, terrain/select/ms 0.63, far/select/ms 0.58, stream/drive/ms 0.51 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\claude\3866c27c11752a1d\4ff90442-bcd4-4820-b03c-4dd8a81992a4\scratchpad\bin\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\tc_pbr\balmora_walk@2560x1440_off=normalmaps_orm_parallax-1 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 16% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **92%** (min 70%) · 2574 MHz (min 2565) · memory 8001 MHz · 89.1 W · 78 °C (max 84) · VRAM 5665 MiB · throttled 100% of samples
