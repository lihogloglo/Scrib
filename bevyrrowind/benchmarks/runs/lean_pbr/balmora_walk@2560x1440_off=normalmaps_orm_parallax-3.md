# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5586 frames over 65.6 s · **median 11.46 ms (87 fps)** · mean 11.75 · p95 14.50 · p99 17.21 (1% low 58 fps) · worst 33.99 ms · 57 spikes over 1.5× median · 36.6 stutters/min (work 35.6/min)
- warm-up: 339 frames over 8.4 s, worst 2648.4 ms
- hitches (> 28.6 ms): **3** measured, 27 in warm-up
- timed GPU passes: 7.28 ms of a 11.75 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.46 ms, limit 11 ms
- **FAIL** frame_ms: 40 stutters (> 1.5x moving median of 60) in 1.09 min = 36.56/min, limit 1/min
- **FAIL** worst frame 33.99 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.082 | 2.176 | 2.482 | 0.96 | 0.182 |
| main_opaque_pass_3d | 1.494 | 1.561 | 2.198 | 0.96 | 0.069 |
| early prepass | 0.480 | 0.502 | 0.770 | 0.96 | 0.094 |
| bloom | 0.324 | 0.339 | 0.490 | 0.96 | 0.067 |
| atmosphere_luts | 0.314 | 0.328 | 0.467 | 0.96 | 0.004 |
| water_velocity | 0.291 | 0.304 | 0.378 | 0.96 | 0.002 |
| uniform_allocation | 0.212 | 0.221 | 0.229 | 0.96 | 0.044 |
| water_sspr | 0.209 | 0.218 | 0.217 | 0.96 | 0.003 |
| ssao | 0.193 | 0.201 | 0.210 | 0.96 | 0.008 |
| auxiliary_views | 0.184 | 0.193 | 0.682 | 0.96 | 0.188 |
| bin_unpacking | 0.171 | 0.179 | 0.186 | 0.96 | 0.030 |
| atmos_froxel_inject | 0.149 | 0.155 | 0.153 | 0.96 | 0.002 |
| clouds_trace | 0.134 | 0.140 | 0.230 | 0.96 | 0.001 |
| water_fft | 0.134 | 0.140 | 0.139 | 0.96 | 0.003 |
| upscaling | 0.120 | 0.125 | 0.145 | 0.96 | 0.005 |
| clouds_composite | 0.105 | 0.110 | 0.111 | 0.96 | 0.002 |
| main_transmissive_pass_3d | 0.088 | 0.092 | 0.170 | 0.96 | 0.009 |
| tonemapping | 0.088 | 0.092 | 0.093 | 0.96 | 0.004 |
| clustering | 0.064 | 0.067 | 0.070 | 0.96 | 0.021 |
| render_sky | 0.052 | 0.054 | 0.054 | 0.96 | 0.002 |
| clouds_resolve | 0.048 | 0.050 | 0.051 | 0.96 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.439 | 0.756 | 0.11 | 0.001 |
| surface_trail | 0.043 | 0.059 | 0.057 | 0.72 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.044 | 0.049 | 0.96 | 0.007 |
| water_snapshot | 0.034 | 0.035 | 0.042 | 0.96 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.96 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.027 | 0.030 | 0.96 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.233 | 0.239 | 0.11 | 0.005 |
| water_mips | 0.024 | 0.025 | 0.024 | 0.96 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.022 | 0.023 | 0.96 | 0.007 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.031 | 0.96 | 0.113 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.96 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.96 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.96 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.96 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 22.442 | 0.000 | 9519.45 | 125360.7 |
| render/thread/ms | 10.524 | 13.108 | 29.81 | 58786.6 |
| render/busy/ms | 9.905 | 12.503 | 26.66 | 55331.0 |
| main/busy/ms | 8.936 | 10.943 | 27.91 | 49914.5 |
| render/graph/ms | 5.446 | 7.034 | 14.79 | 30419.2 |
| render/present/ms | 0.524 | 1.266 | 16.86 | 2928.9 |
| bigspace/total/ms | 0.490 | 0.892 | 2.41 | 2737.5 |
| pipelines/compile_ms | 0.488 | 0.000 | 176.02 | 2723.8 |
| pipelines/front_ms | 0.471 | 0.000 | 169.19 | 2631.8 |
| terrain/update/ms | 0.302 | 0.393 | 1.60 | 1687.9 |
| controller/feed/ms | 0.298 | 0.397 | 1.03 | 1663.5 |
| terrain/select/ms | 0.269 | 0.349 | 0.69 | 1501.1 |
| stream/drive/ms | 0.261 | 0.371 | 18.84 | 1460.5 |
| far/select/ms | 0.208 | 0.298 | 0.62 | 1163.8 |
| stream/tick/ms | 0.203 | 0.286 | 0.67 | 1136.4 |
| bigspace/low_precision/ms | 0.195 | 0.350 | 2.15 | 1091.6 |
| anim/evaluate_cpu/ms | 0.170 | 0.215 | 0.70 | 949.5 |
| controller/move/ms | 0.141 | 0.259 | 0.72 | 785.2 |
| anim/evaluate/ms | 0.140 | 0.267 | 1.58 | 780.8 |
| bigspace/high_precision/ms | 0.120 | 0.256 | 1.91 | 668.0 |
| render/acquire/ms | 0.094 | 0.123 | 3.31 | 526.7 |
| physics/insert/ms | 0.057 | 0.456 | 1.00 | 316.1 |
| physics/step/ms | 0.053 | 0.094 | 0.64 | 298.0 |
| weather/map/ms | 0.026 | 0.035 | 0.29 | 143.0 |
| pipelines/driver_ms | 0.016 | 0.000 | 6.83 | 91.9 |
| anim/apply/ms | 0.013 | 0.025 | 0.23 | 72.1 |
| controller/avoid/ms | 0.011 | 0.014 | 0.22 | 59.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.08 | 58.7 |
| script/update/ms | 0.008 | 0.010 | 0.22 | 44.2 |
| stream/despawn/ms | 0.005 | 0.000 | 1.07 | 30.5 |
| script/input/ms | 0.005 | 0.007 | 0.11 | 30.2 |
| stream/spawn/ms | 0.005 | 0.000 | 1.09 | 27.2 |
| script/ui/ms | 0.005 | 0.006 | 0.16 | 25.9 |
| controller/write/ms | 0.004 | 0.005 | 0.26 | 20.7 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.77 | 15.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.40 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.07 | 4.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 33 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 108 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 26 | 60 | 12 |
| crowd/c0/joints/count | count | 1291 | 2292 | 535 |
| crowd/c0/meshes/count | count | 355 | 646 | 144 |
| crowd/c1/count | count | 146 | 156 | 149 |
| crowd/c1/joints/count | count | 5531 | 5947 | 5661 |
| crowd/c1/meshes/count | count | 1166 | 1282 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 8375 | 6179 |
| crowd/joints_uploaded/count | count | 4248 | 6643 | 6179 |
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
| frame/work/ms | ms | 9.71 | 27.91 | 10.02 |
| gpu/timed/ms | ms | 8.28 | 15.92 | 8.13 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 803 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 803 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1293 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3388 | 5132 | 3181 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1293 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3388 | 5132 | 3181 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 54 | 284 | 39 |
| phases/shadow/bins | count | 3790 | 5606 | 2220 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9600 | 12.5 k | 5671 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 136 | 318 | 39 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2349 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 1020 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 204 | 208 | 208 |
| pipelines/waiting | count | 0 | 28 | 0 |
| preprocess/bind_groups | count | 11 | 737 | 4 |
| preprocess/bind_groups_created | count | 11 | 737 | 4 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 224 | 349 | 217 |
| process/cpu_usage | % | 25.84 | 44.71 | 28.37 |
| process/mem_usage | GiB | 5.15 | 5.44 | 5.44 |
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
| started_load_count |  | 4766 | 4859 | 4859 |
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
| stream/latency/s | s | 0.04 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.18 | 65.66 | 46.84 |
| system/mem_usage | % | 75.46 | 76.51 | 76.49 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2591 | 2644 | 2644 |
| views/all/batch_sets | count | 111 | 336 | 99 |
| views/all/bins | count | 6473 | 8254 | 5700 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.1 k | 13.3 k |
| views/all/pipelines | count | 190 | 372 | 95 |
| views/all/pipelines_distinct | count | 81 | 93 | 77 |
| views/all/work_items | count | 17.2 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 48 | 56 | 39 |
| views/cascade/bins | count | 2606 | 3100 | 2220 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8134 | 9791 | 5671 |
| views/cascade/pipelines | count | 48 | 56 | 39 |
| views/cascade/work_items | count | 8134 | 9791 | 5671 |
| views/lamp_face/batch_sets | count | 0 | 232 | 0 |
| views/lamp_face/bins | count | 1076 | 2653 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1226 | 3007 | 0 |
| views/lamp_face/pipelines | count | 86 | 263 | 0 |
| views/lamp_face/work_items | count | 0 | 2499 | 0 |
| views/main/batch_sets | count | 58 | 62 | 60 |
| views/main/bins | count | 2882 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8377 | 13.0 k | 7596 |
| views/main/pipelines | count | 54 | 58 | 56 |
| views/main/work_items | count | 8548 | 13.3 k | 7904 |
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
| anim/bones_written/count | count | 62.69 | 1137 | 350.2 k |
| anim/events/count | count | 0.09 | 3 | 507 |
| assets/Image/added | count | 0.11 | 42 | 627 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.75 | 152 | 4188 |
| assets/Mesh/removed | count | 0.87 | 1254 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 503 |
| assets/StandardMaterial/modified | count | 3.90 | 18 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.92 | 4319 | 10.7 k |
| physics/inserts | count | 0.57 | 106 | 3166 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 161 | 2335 |
| physics/steps | count | 0.71 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 15 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6060 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 62 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1329 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.6 ms; slowest 30, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.65 | 2648.4 | main/busy/ms 2623, surface/advance_cpu_ms 68.59, surface/cell_upload_ms 0.79, stream/drive/ms 0.60, script/update/ms 0.16, weather/map/ms 0.11, stream/tick/ms 0.11, bigspace/total/ms 0.10, bigspace/high_precision/ms 0.07, weather/eval/ms 0.03, anim/evaluate/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.49 | 841.5 | main/busy/ms 2660, render/thread/ms 803, render/busy/ms 695, render/acquire/ms 106, surface/advance_cpu_ms 68.59, render/graph/ms 6.10, render/present/ms 2.29, stream/drive/ms 1.61, surface/cell_upload_ms 0.79, stream/tick/ms 0.71, terrain/update/ms 0.44, terrain/select/ms 0.30 |
| warm-up | 3.57 | 79.7 | render/thread/ms 813, render/busy/ms 704, pipelines/wait_ms 631, render/acquire/ms 106, pipelines/compile_ms 76.60, pipelines/front_ms 68.96, main/busy/ms 52.91, terrain/update/ms 20.88, render/graph/ms 9.28, pipelines/driver_ms 7.64, render/present/ms 3.27, stream/drive/ms 1.66 |
| warm-up | 3.82 | 89.2 | pipelines/compile_ms 235, pipelines/front_ms 223, main/busy/ms 69.34, render/thread/ms 69.21, render/busy/ms 67.60, pipelines/driver_ms 12.42, render/graph/ms 3.49, stream/drive/ms 1.38, render/present/ms 1.32, stream/tick/ms 1.09, terrain/update/ms 0.87, terrain/select/ms 0.70 |
| warm-up | 3.98 | 74.1 | main/busy/ms 45.17, render/thread/ms 38.51, render/busy/ms 37.22, stream/drive/ms 31.32, render/graph/ms 2.56, render/present/ms 1.12, terrain/update/ms 0.91, stream/tick/ms 0.78, terrain/select/ms 0.75, stream/spawn/ms 0.34, bigspace/total/ms 0.24, render/acquire/ms 0.18 |
| warm-up | 4.09 | 57.8 | pipelines/wait_ms 861, render/thread/ms 48.24, render/busy/ms 46.60, main/busy/ms 34.31, stream/drive/ms 21.36, pipelines/compile_ms 9.15, pipelines/front_ms 8.39, render/graph/ms 5.48, render/present/ms 1.39, stream/tick/ms 1.10, terrain/update/ms 0.97, terrain/select/ms 0.79 |
| warm-up | 4.16 | 50.1 | render/thread/ms 54.01, render/busy/ms 52.12, main/busy/ms 28.18, stream/drive/ms 7.49, stream/spawn/ms 5.59, render/graph/ms 3.93, render/present/ms 1.58, stream/tick/ms 0.96, terrain/update/ms 0.83, terrain/select/ms 0.65, bigspace/total/ms 0.47, render/acquire/ms 0.32 |
| warm-up | 4.19 | 29.5 | main/busy/ms 58.16, render/thread/ms 51.29, render/busy/ms 49.52, stream/drive/ms 35.46, stream/spawn/ms 5.58, render/graph/ms 3.36, render/present/ms 1.51, terrain/update/ms 1.20, stream/tick/ms 1.09, terrain/select/ms 1.00, bigspace/total/ms 0.40, stream/upload/ms 0.31 |
| warm-up | 4.27 | 80.3 | pipelines/wait_ms 435, pipelines/compile_ms 238, pipelines/front_ms 221, main/busy/ms 55.50, render/thread/ms 52.89, render/busy/ms 51.72, stream/drive/ms 31.53, pipelines/driver_ms 16.66, render/graph/ms 3.28, stream/spawn/ms 1.42, terrain/update/ms 1.25, terrain/select/ms 1.06 |
| warm-up | 4.31 | 38.0 | pipelines/wait_ms 435, pipelines/compile_ms 238, pipelines/front_ms 221, render/thread/ms 63.43, render/busy/ms 61.65, main/busy/ms 25.09, pipelines/driver_ms 16.66, render/graph/ms 6.68, stream/drive/ms 3.82, render/present/ms 1.60, stream/activate/ms 1.08, terrain/update/ms 1.00 |
| warm-up | 4.38 | 43.7 | main/busy/ms 72.09, stream/drive/ms 55.34, render/thread/ms 51.45, render/busy/ms 49.18, render/graph/ms 13.43, stream/spawn/ms 3.10, render/present/ms 2.06, stream/tick/ms 1.74, stream/activate/ms 1.27, terrain/update/ms 1.14, terrain/select/ms 0.90, physics/insert/ms 0.62 |
| warm-up | 4.44 | 60.8 | pipelines/wait_ms 4930, pipelines/compile_ms 177, pipelines/front_ms 168, main/busy/ms 61.48, render/thread/ms 56.46, render/busy/ms 55.16, stream/drive/ms 42.83, pipelines/driver_ms 8.89, render/graph/ms 7.64, stream/spawn/ms 2.04, stream/tick/ms 1.68, render/present/ms 1.09 |
| warm-up | 4.52 | 72.6 | pipelines/wait_ms 7514, pipelines/compile_ms 123, pipelines/front_ms 116, main/busy/ms 66.10, stream/drive/ms 45.03, render/thread/ms 34.36, render/busy/ms 32.91, pipelines/driver_ms 7.59, render/graph/ms 4.63, stream/spawn/ms 2.20, render/present/ms 1.22, terrain/update/ms 0.93 |
| warm-up | 4.59 | 38.8 | pipelines/wait_ms 1884, main/busy/ms 46.83, render/thread/ms 34.57, stream/drive/ms 34.07, render/busy/ms 33.30, pipelines/compile_ms 18.58, pipelines/front_ms 16.92, render/graph/ms 4.90, pipelines/driver_ms 1.66, stream/spawn/ms 1.53, stream/activate/ms 1.05, render/present/ms 0.97 |
| warm-up | 4.62 | 29.2 | main/busy/ms 33.65, stream/drive/ms 23.48, render/thread/ms 14.89, render/busy/ms 13.84, render/graph/ms 3.50, stream/activate/ms 1.37, stream/tick/ms 0.91, render/present/ms 0.86, stream/spawn/ms 0.70, terrain/update/ms 0.58, terrain/select/ms 0.49, physics/insert/ms 0.39 |
| warm-up | 4.74 | 32.4 | pipelines/wait_ms 11.6 k, pipelines/compile_ms 65.13, pipelines/front_ms 58.67, render/thread/ms 39.24, render/busy/ms 37.24, main/busy/ms 20.99, pipelines/driver_ms 6.46, render/graph/ms 5.55, stream/drive/ms 3.95, render/present/ms 1.75, stream/tick/ms 1.48, stream/spawn/ms 1.42 |
| warm-up | 4.77 | 30.0 | pipelines/wait_ms 14.1 k, pipelines/compile_ms 79.29, pipelines/front_ms 70.36, render/thread/ms 46.30, render/busy/ms 44.95, main/busy/ms 31.09, stream/drive/ms 12.62, pipelines/driver_ms 8.93, render/graph/ms 4.61, stream/tick/ms 1.25, render/present/ms 1.05, terrain/update/ms 0.94 |
| warm-up | 4.86 | 43.5 | pipelines/wait_ms 6170, pipelines/compile_ms 36.21, render/thread/ms 35.48, main/busy/ms 35.06, pipelines/front_ms 33.92, render/busy/ms 33.46, render/graph/ms 6.11, stream/drive/ms 2.99, pipelines/driver_ms 2.29, render/present/ms 1.45, stream/tick/ms 1.18, terrain/update/ms 0.81 |
| warm-up | 4.91 | 42.7 | pipelines/wait_ms 1248, pipelines/compile_ms 52.22, pipelines/front_ms 49.21, render/thread/ms 40.03, render/busy/ms 38.26, main/busy/ms 30.33, render/graph/ms 6.92, pipelines/driver_ms 3.01, stream/drive/ms 1.64, render/present/ms 1.58, terrain/update/ms 0.85, stream/tick/ms 0.77 |
| warm-up | 4.95 | 36.8 | pipelines/wait_ms 1248, pipelines/compile_ms 52.22, render/thread/ms 51.95, render/busy/ms 50.26, pipelines/front_ms 49.21, main/busy/ms 42.23, render/graph/ms 5.75, pipelines/driver_ms 3.01, stream/drive/ms 1.81, render/present/ms 1.49, terrain/update/ms 0.88, stream/tick/ms 0.81 |
| warm-up | 4.99 | 43.3 | render/thread/ms 54.28, render/busy/ms 52.68, main/busy/ms 46.91, render/graph/ms 6.19, stream/drive/ms 1.78, render/present/ms 1.38, terrain/update/ms 1.08, terrain/select/ms 0.92, stream/tick/ms 0.83, bigspace/total/ms 0.69, physics/insert/ms 0.38, physics/step/ms 0.28 |
| warm-up | 5.51 | 28.7 | render/thread/ms 33.16, render/busy/ms 32.12, main/busy/ms 21.35, render/graph/ms 4.52, stream/drive/ms 3.75, stream/spawn/ms 2.05, stream/tick/ms 0.93, render/present/ms 0.88, terrain/update/ms 0.84, physics/step/ms 0.81, physics/insert/ms 0.70, terrain/select/ms 0.67 |
| warm-up | 5.64 | 34.5 | main/busy/ms 46.53, render/thread/ms 37.66, render/busy/ms 35.52, render/graph/ms 9.41, stream/drive/ms 9.27, stream/spawn/ms 2.51, terrain/update/ms 2.27, render/present/ms 1.86, physics/insert/ms 1.49, bigspace/total/ms 0.83, stream/tick/ms 0.72, terrain/select/ms 0.52 |
| warm-up | 5.67 | 28.6 | render/thread/ms 41.80, render/busy/ms 40.39, main/busy/ms 39.91, render/graph/ms 5.03, stream/drive/ms 3.79, terrain/update/ms 2.24, stream/spawn/ms 2.15, render/present/ms 1.12, physics/insert/ms 0.99, bigspace/total/ms 0.93, stream/tick/ms 0.84, terrain/select/ms 0.50 |
| warm-up | 5.97 | 43.1 | pipelines/compile_ms 187, pipelines/front_ms 180, pipelines/wait_ms 74.89, main/busy/ms 45.33, render/thread/ms 42.48, render/busy/ms 40.78, stream/drive/ms 20.21, pipelines/driver_ms 7.34, render/graph/ms 5.61, stream/spawn/ms 2.29, stream/tick/ms 1.59, render/present/ms 1.51 |
| warm-up | 6.01 | 41.0 | pipelines/compile_ms 187, pipelines/front_ms 180, pipelines/wait_ms 74.89, render/thread/ms 57.41, render/busy/ms 56.14, main/busy/ms 35.40, stream/drive/ms 8.85, pipelines/driver_ms 7.34, render/graph/ms 5.61, stream/spawn/ms 2.30, stream/tick/ms 1.55, render/present/ms 1.07 |
| warm-up | 6.66 | 32.0 | pipelines/wait_ms 4795, render/thread/ms 41.00, render/busy/ms 39.89, render/graph/ms 30.57, main/busy/ms 18.76, pipelines/compile_ms 13.26, pipelines/front_ms 12.13, stream/drive/ms 1.54, pipelines/driver_ms 1.13, render/present/ms 0.92, bigspace/total/ms 0.71, stream/tick/ms 0.69 |
| measure | 10.72 | 31.7 | render/thread/ms 45.56, render/busy/ms 29.74, main/busy/ms 27.11, render/graph/ms 15.80, render/present/ms 15.43, bigspace/total/ms 2.05, controller/feed/ms 1.44, bigspace/low_precision/ms 1.42, terrain/update/ms 0.63, stream/drive/ms 0.59, terrain/select/ms 0.56, stream/tick/ms 0.53 |
| measure | 46.39 | 29.8 | render/thread/ms 38.29, render/busy/ms 20.76, main/busy/ms 18.30, render/present/ms 17.30, render/graph/ms 11.39, bigspace/total/ms 1.56, bigspace/low_precision/ms 0.99, stream/drive/ms 0.68, far/select/ms 0.64, terrain/update/ms 0.62, controller/feed/ms 0.62, stream/tick/ms 0.61 |
| measure | 59.05 | 34.0 | render/thread/ms 41.76, render/busy/ms 40.54, main/busy/ms 27.60, render/graph/ms 10.08, bigspace/total/ms 1.45, controller/feed/ms 1.23, terrain/update/ms 1.06, render/present/ms 0.96, stream/drive/ms 0.96, terrain/select/ms 0.93, stream/tick/ms 0.85, far/select/ms 0.72 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_pbr\balmora_walk@2560x1440_off=normalmaps_orm_parallax-3 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **83%** (min 56%) · 2578 MHz (min 2565) · memory 8001 MHz · 79.6 W · 76 °C (max 82) · VRAM 6268 MiB · throttled 100% of samples
