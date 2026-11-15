# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4881 frames over 65.6 s · **median 13.05 ms (77 fps)** · mean 13.45 · p95 16.60 · p99 21.29 (1% low 47 fps) · worst 36.90 ms · 59 spikes over 1.5× median · 43.0 stutters/min (work 39.3/min)
- warm-up: 386 frames over 10.7 s, worst 4196.8 ms
- hitches (> 32.6 ms): **7** measured, 15 in warm-up
- timed GPU passes: 7.96 ms of a 13.45 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.05 ms, limit 11 ms
- **FAIL** frame_ms: 47 stutters (> 1.5x moving median of 60) in 1.09 min = 42.96/min, limit 1/min
- **FAIL** worst frame 36.90 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.164 | 2.219 | 2.739 | 0.98 | 0.180 |
| main_opaque_pass_3d | 1.742 | 1.786 | 2.626 | 0.98 | 0.071 |
| early prepass | 0.540 | 0.554 | 0.806 | 0.98 | 0.119 |
| bloom | 0.333 | 0.342 | 0.489 | 0.98 | 0.067 |
| atmosphere_luts | 0.326 | 0.335 | 0.499 | 0.98 | 0.003 |
| uniform_allocation | 0.309 | 0.316 | 0.436 | 0.98 | 0.054 |
| water_velocity | 0.300 | 0.307 | 0.512 | 0.98 | 0.002 |
| bin_unpacking | 0.235 | 0.241 | 0.264 | 0.98 | 0.038 |
| water_sspr | 0.220 | 0.226 | 0.221 | 0.98 | 0.002 |
| auxiliary_views | 0.203 | 0.208 | 0.772 | 0.98 | 0.181 |
| ssao | 0.202 | 0.207 | 0.210 | 0.98 | 0.007 |
| atmos_froxel_inject | 0.158 | 0.162 | 0.153 | 0.98 | 0.002 |
| clouds_trace | 0.138 | 0.141 | 0.226 | 0.98 | 0.001 |
| water_fft | 0.136 | 0.140 | 0.137 | 0.98 | 0.003 |
| upscaling | 0.129 | 0.132 | 0.156 | 0.98 | 0.004 |
| clouds_composite | 0.110 | 0.112 | 0.111 | 0.98 | 0.002 |
| main_transmissive_pass_3d | 0.091 | 0.094 | 0.173 | 0.98 | 0.009 |
| tonemapping | 0.091 | 0.093 | 0.092 | 0.98 | 0.004 |
| clustering | 0.071 | 0.073 | 0.070 | 0.98 | 0.020 |
| render_sky | 0.053 | 0.055 | 0.054 | 0.98 | 0.002 |
| clouds_resolve | 0.049 | 0.050 | 0.052 | 0.98 | 0.001 |
| lightprobe_irradiance_map | 0.048 | 0.443 | 0.742 | 0.11 | 0.001 |
| surface_trail | 0.048 | 0.059 | 0.057 | 0.82 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.044 | 0.049 | 0.98 | 0.005 |
| water_snapshot | 0.036 | 0.037 | 0.043 | 0.98 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.98 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.031 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.228 | 0.236 | 0.11 | 0.005 |
| per_view_shadow_pass | 0.023 | 0.024 | 0.024 | 0.98 | 0.006 |
| water_mips | 0.023 | 0.023 | 0.024 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.031 | 0.98 | 0.110 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.98 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.98 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.98 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.016 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 35.447 | 0.000 | 19807.70 | 173019.2 |
| render/thread/ms | 12.178 | 15.224 | 31.59 | 59442.6 |
| render/busy/ms | 11.591 | 14.636 | 30.56 | 56573.7 |
| main/busy/ms | 9.294 | 11.991 | 32.63 | 45365.5 |
| render/graph/ms | 7.030 | 8.996 | 19.16 | 34315.2 |
| pipelines/compile_ms | 0.608 | 0.000 | 149.94 | 2967.6 |
| pipelines/front_ms | 0.584 | 0.000 | 144.24 | 2851.2 |
| render/present/ms | 0.499 | 0.624 | 14.50 | 2435.1 |
| bigspace/total/ms | 0.457 | 0.727 | 4.44 | 2229.1 |
| terrain/update/ms | 0.292 | 0.367 | 2.13 | 1426.2 |
| controller/feed/ms | 0.263 | 0.355 | 0.77 | 1284.4 |
| terrain/select/ms | 0.259 | 0.326 | 0.68 | 1263.2 |
| stream/drive/ms | 0.256 | 0.355 | 24.58 | 1247.6 |
| far/select/ms | 0.195 | 0.269 | 0.58 | 951.3 |
| stream/tick/ms | 0.191 | 0.264 | 0.80 | 930.4 |
| bigspace/low_precision/ms | 0.176 | 0.316 | 3.23 | 859.6 |
| anim/evaluate/ms | 0.174 | 0.331 | 2.68 | 850.1 |
| anim/evaluate_cpu/ms | 0.148 | 0.197 | 0.82 | 722.8 |
| controller/move/ms | 0.134 | 0.240 | 1.18 | 655.4 |
| bigspace/high_precision/ms | 0.111 | 0.220 | 4.14 | 542.5 |
| render/acquire/ms | 0.089 | 0.116 | 0.63 | 433.9 |
| physics/step/ms | 0.059 | 0.090 | 4.06 | 286.8 |
| physics/insert/ms | 0.056 | 0.438 | 1.24 | 272.0 |
| weather/map/ms | 0.024 | 0.033 | 0.20 | 119.1 |
| pipelines/driver_ms | 0.024 | 0.000 | 6.77 | 116.4 |
| anim/apply/ms | 0.011 | 0.020 | 0.06 | 53.4 |
| weather/eval/ms | 0.010 | 0.014 | 0.27 | 50.3 |
| controller/avoid/ms | 0.010 | 0.014 | 0.05 | 50.0 |
| script/update/ms | 0.007 | 0.009 | 0.06 | 33.1 |
| stream/despawn/ms | 0.007 | 0.000 | 1.91 | 32.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 24.7 |
| script/ui/ms | 0.005 | 0.006 | 0.06 | 22.0 |
| script/input/ms | 0.004 | 0.006 | 0.06 | 21.6 |
| controller/write/ms | 0.004 | 0.004 | 0.03 | 17.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 1.60 | 15.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.44 | 4.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.19 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 27 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 97 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 59 | 11 |
| crowd/c0/joints/count | count | 1291 | 2292 | 482 |
| crowd/c0/meshes/count | count | 353 | 646 | 119 |
| crowd/c1/count | count | 146 | 157 | 150 |
| crowd/c1/joints/count | count | 5531 | 6046 | 5714 |
| crowd/c1/meshes/count | count | 1166 | 1322 | 1222 |
| crowd/c2/count | count | 397 | 417 | 417 |
| crowd/c2/joints/count | count | 6402 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4271 | 7274 | 6179 |
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
| frame/work/ms | ms | 11.30 | 32.63 | 10.37 |
| gpu/timed/ms | ms | 8.83 | 12.90 | 8.76 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 800 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 800 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 33 | 37 | 33 |
| phases/opaque3d/bins | count | 1282 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3367 | 5096 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 28 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 33 | 37 | 33 |
| phases/opaque3d_prepass/bins | count | 1282 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3367 | 5096 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 28 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 77 | 315 | 55 |
| phases/shadow/bins | count | 3753 | 5475 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9384 | 12.4 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 140 | 317 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 181 | 476 | 19 |
| physics/pending | count | 0 | 1261 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 40 | 0 |
| preprocess/bind_groups | count | 12 | 859 | 6 |
| preprocess/bind_groups_created | count | 12 | 859 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 304 | 480 | 287 |
| process/cpu_usage | % | 23.00 | 37.05 | 20.67 |
| process/mem_usage | GiB | 5.21 | 5.38 | 5.38 |
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
| started_load_count |  | 4769 | 4861 | 4861 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
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
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.34 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.61 | 94.92 | 28.72 |
| system/mem_usage | % | 75.72 | 76.92 | 76.34 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2591 | 2644 | 2644 |
| views/all/batch_sets | count | 157 | 397 | 135 |
| views/all/bins | count | 6372 | 8035 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 17.9 k | 24.9 k | 13.3 k |
| views/all/pipelines | count | 203 | 377 | 113 |
| views/all/pipelines_distinct | count | 96 | 108 | 91 |
| views/all/work_items | count | 17.1 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 69 | 87 | 55 |
| views/cascade/bins | count | 2592 | 3167 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8132 | 9804 | 5663 |
| views/cascade/pipelines | count | 56 | 72 | 47 |
| views/cascade/work_items | count | 8132 | 9804 | 5663 |
| views/lamp_face/batch_sets | count | 6 | 243 | 0 |
| views/lamp_face/bins | count | 1082 | 2521 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1211 | 2868 | 0 |
| views/lamp_face/pipelines | count | 82 | 245 | 0 |
| views/lamp_face/work_items | count | 70 | 2563 | 0 |
| views/main/batch_sets | count | 80 | 90 | 80 |
| views/main/bins | count | 2862 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8332 | 13.0 k | 7592 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8502 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 57.25 | 939 | 279.4 k |
| anim/events/count | count | 0.12 | 3 | 579 |
| assets/Image/added | count | 0.14 | 28 | 684 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.88 | 152 | 4304 |
| assets/Mesh/removed | count | 1.00 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.12 | 35 | 569 |
| assets/StandardMaterial/modified | count | 4.44 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 3.04 | 4947 | 14.8 k |
| physics/inserts | count | 0.67 | 172 | 3272 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.48 | 161 | 2335 |
| physics/steps | count | 0.81 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 4 | 20 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 24 | 378 |
| stream/upload/bytes | bytes | 6936 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1521 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 32.6 ms; slowest 22, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 4.20 | 4196.8 | main/busy/ms 4172, surface/advance_cpu_ms 73.76, surface/cell_upload_ms 1.23, stream/drive/ms 0.36, weather/map/ms 0.28, bigspace/total/ms 0.13, stream/tick/ms 0.10, bigspace/high_precision/ms 0.07, weather/eval/ms 0.04, script/update/ms 0.03, anim/evaluate/ms 0.02, script/input/ms 0.01 |
| warm-up | 4.92 | 721.6 | main/busy/ms 4205, render/thread/ms 687, render/busy/ms 551, render/acquire/ms 134, surface/advance_cpu_ms 73.76, render/graph/ms 6.36, render/present/ms 1.91, surface/cell_upload_ms 1.23, stream/drive/ms 1.12, terrain/update/ms 0.57, stream/tick/ms 0.54, terrain/select/ms 0.39 |
| warm-up | 5.00 | 82.1 | pipelines/wait_ms 1040, render/thread/ms 698, render/busy/ms 561, render/acquire/ms 134, pipelines/compile_ms 91.77, pipelines/front_ms 82.74, main/busy/ms 48.56, terrain/update/ms 18.11, render/graph/ms 9.71, pipelines/driver_ms 9.03, render/present/ms 2.70, stream/drive/ms 1.36 |
| warm-up | 5.27 | 86.1 | pipelines/compile_ms 267, pipelines/front_ms 253, pipelines/wait_ms 207, main/busy/ms 60.55, render/thread/ms 45.74, render/busy/ms 44.11, pipelines/driver_ms 13.50, render/graph/ms 4.02, stream/drive/ms 2.16, stream/tick/ms 1.72, render/present/ms 1.43, terrain/update/ms 0.65 |
| warm-up | 5.41 | 56.6 | pipelines/wait_ms 392, render/thread/ms 44.25, render/busy/ms 42.96, main/busy/ms 41.93, stream/drive/ms 30.91, render/graph/ms 2.42, pipelines/compile_ms 1.52, pipelines/front_ms 1.20, render/present/ms 1.06, stream/tick/ms 0.86, terrain/update/ms 0.61, terrain/select/ms 0.51 |
| warm-up | 5.46 | 47.1 | pipelines/wait_ms 646, pipelines/compile_ms 171, pipelines/front_ms 160, main/busy/ms 38.38, stream/drive/ms 27.39, render/thread/ms 26.89, render/busy/ms 25.36, pipelines/driver_ms 10.84, render/graph/ms 4.32, render/present/ms 1.35, stream/tick/ms 1.15, stream/spawn/ms 1.03 |
| warm-up | 5.56 | 72.9 | pipelines/wait_ms 5290, pipelines/compile_ms 147, pipelines/front_ms 127, render/thread/ms 62.59, render/busy/ms 60.32, main/busy/ms 42.67, stream/drive/ms 22.60, pipelines/driver_ms 20.09, render/graph/ms 7.20, stream/tick/ms 2.02, render/present/ms 2.00, stream/spawn/ms 0.91 |
| warm-up | 5.72 | 126.5 | pipelines/wait_ms 12.2 k, pipelines/compile_ms 228, pipelines/front_ms 154, pipelines/driver_ms 74.66, render/thread/ms 74.30, main/busy/ms 73.10, stream/drive/ms 51.67, render/busy/ms 49.41, render/present/ms 24.66, terrain/update/ms 4.59, render/graph/ms 4.58, stream/tick/ms 1.37 |
| warm-up | 5.81 | 74.1 | pipelines/wait_ms 8733, pipelines/compile_ms 118, main/busy/ms 68.89, pipelines/driver_ms 60.45, pipelines/front_ms 57.37, stream/drive/ms 48.85, render/thread/ms 34.77, render/busy/ms 33.25, render/graph/ms 7.07, terrain/update/ms 2.60, render/present/ms 1.32, stream/spawn/ms 1.19 |
| warm-up | 5.86 | 37.2 | main/busy/ms 40.09, render/thread/ms 30.42, render/busy/ms 28.71, stream/drive/ms 27.08, render/graph/ms 8.92, render/present/ms 1.44, bigspace/total/ms 1.07, terrain/update/ms 1.00, bigspace/low_precision/ms 0.88, terrain/select/ms 0.79, stream/tick/ms 0.72, stream/activate/ms 0.63 |
| warm-up | 6.09 | 105.0 | main/busy/ms 62.89, stream/drive/ms 49.54, render/thread/ms 49.10, render/busy/ms 47.59, render/graph/ms 4.70, stream/tick/ms 1.65, render/present/ms 1.28, stream/activate/ms 0.89, terrain/update/ms 0.73, terrain/select/ms 0.61, physics/insert/ms 0.23, render/acquire/ms 0.23 |
| warm-up | 6.15 | 53.0 | pipelines/wait_ms 3682, pipelines/compile_ms 207, pipelines/front_ms 178, render/thread/ms 74.82, render/busy/ms 73.37, main/busy/ms 45.81, stream/drive/ms 30.51, pipelines/driver_ms 29.53, render/graph/ms 3.49, stream/tick/ms 1.21, render/present/ms 1.20, stream/spawn/ms 1.05 |
| warm-up | 6.46 | 36.1 | pipelines/wait_ms 10.3 k, pipelines/compile_ms 240, pipelines/front_ms 217, render/thread/ms 49.51, render/busy/ms 48.17, main/busy/ms 28.22, pipelines/driver_ms 22.95, stream/drive/ms 10.25, render/graph/ms 6.87, stream/tick/ms 1.29, render/present/ms 1.10, terrain/update/ms 0.78 |
| warm-up | 6.76 | 36.9 | pipelines/wait_ms 3386, main/busy/ms 35.85, pipelines/compile_ms 25.29, render/thread/ms 23.99, pipelines/front_ms 23.38, render/busy/ms 22.46, stream/drive/ms 14.10, render/graph/ms 6.38, pipelines/driver_ms 1.91, render/present/ms 1.32, stream/tick/ms 0.76, terrain/update/ms 0.70 |
| warm-up | 8.98 | 38.3 | main/busy/ms 43.44, render/thread/ms 27.16, render/busy/ms 25.66, render/graph/ms 8.49, render/present/ms 1.33, stream/drive/ms 1.13, physics/insert/ms 0.96, bigspace/total/ms 0.71, terrain/update/ms 0.62, stream/tick/ms 0.55, terrain/select/ms 0.55, stream/spawn/ms 0.29 |
| measure | 0.78 | 33.5 | main/busy/ms 36.36, render/thread/ms 29.29, render/busy/ms 27.94, stream/drive/ms 18.55, render/graph/ms 12.38, render/present/ms 1.19, bigspace/total/ms 0.87, physics/insert/ms 0.76, terrain/update/ms 0.70, terrain/select/ms 0.64, far/select/ms 0.45, controller/feed/ms 0.41 |
| measure | 8.64 | 32.8 | render/thread/ms 46.65, render/busy/ms 34.07, main/busy/ms 26.10, render/graph/ms 20.75, render/present/ms 12.37, bigspace/total/ms 1.37, controller/feed/ms 0.75, bigspace/low_precision/ms 0.74, stream/drive/ms 0.71, terrain/update/ms 0.56, terrain/select/ms 0.50, anim/evaluate_cpu/ms 0.49 |
| measure | 16.24 | 33.0 | render/thread/ms 56.33, render/busy/ms 54.61, main/busy/ms 45.93, render/graph/ms 34.07, render/present/ms 1.41, bigspace/total/ms 1.29, physics/insert/ms 1.11, controller/feed/ms 0.90, terrain/update/ms 0.86, terrain/select/ms 0.77, stream/drive/ms 0.77, stream/tick/ms 0.69 |
| measure | 16.27 | 33.4 | render/thread/ms 62.38, render/busy/ms 60.53, main/busy/ms 39.19, render/graph/ms 35.48, render/present/ms 1.49, bigspace/total/ms 1.42, physics/insert/ms 1.09, controller/feed/ms 0.91, terrain/update/ms 0.86, terrain/select/ms 0.76, stream/drive/ms 0.72, stream/tick/ms 0.65 |
| measure | 18.20 | 34.6 | main/busy/ms 42.83, render/thread/ms 29.06, render/busy/ms 27.98, stream/drive/ms 24.80, render/graph/ms 16.78, render/present/ms 0.91, bigspace/total/ms 0.84, controller/feed/ms 0.62, terrain/update/ms 0.60, terrain/select/ms 0.54, far/select/ms 0.42, stream/tick/ms 0.34 |
| measure | 23.65 | 32.9 | render/thread/ms 47.17, render/busy/ms 31.98, main/busy/ms 26.58, render/graph/ms 18.96, render/present/ms 14.97, bigspace/total/ms 0.98, controller/feed/ms 0.57, terrain/update/ms 0.54, terrain/select/ms 0.48, bigspace/low_precision/ms 0.46, stream/drive/ms 0.41, physics/insert/ms 0.41 |
| measure | 59.06 | 36.9 | render/thread/ms 40.95, render/busy/ms 39.40, main/busy/ms 28.68, render/graph/ms 18.40, bigspace/total/ms 1.75, render/present/ms 1.34, controller/feed/ms 0.92, terrain/update/ms 0.90, terrain/select/ms 0.79, stream/drive/ms 0.78, bigspace/low_precision/ms 0.69, stream/tick/ms 0.69 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_base\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **79%** (min 47%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.2 W · 72 °C (max 79) · VRAM 6290 MiB · throttled 100% of samples
